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
use rusoto_core::request::HttpResponse;
use rusoto_core::signature::SignedRequest;
#[allow(unused_imports)]
use serde::{Deserialize, Serialize};

impl KendraClient {
    fn new_signed_request(&self, http_method: &str, request_uri: &str) -> SignedRequest {
        let mut request = SignedRequest::new(http_method, "kendra", &self.region, request_uri);

        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request
    }

    async fn sign_and_dispatch<E>(
        &self,
        request: SignedRequest,
        from_response: fn(BufferedHttpResponse) -> RusotoError<E>,
    ) -> Result<HttpResponse, RusotoError<E>> {
        let mut response = self.client.sign_and_dispatch(request).await?;
        if !response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            return Err(from_response(response));
        }

        Ok(response)
    }
}

use serde_json;
/// <p>Access Control List files for the documents in a data source. For the format of the file, see <a href="https://docs.aws.amazon.com/kendra/latest/dg/s3-acl.html">Access control for S3 data sources</a>.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct AccessControlListConfiguration {
    /// <p>Path to the AWS S3 bucket that contains the ACL files.</p>
    #[serde(rename = "KeyPath")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_path: Option<String>,
}

/// <p>Provides information about the column that should be used for filtering the query response by groups.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct AclConfiguration {
    /// <p>A list of groups, separated by semi-colons, that filters a query response based on user context. The document is only returned to users that are in one of the groups specified in the <code>UserContext</code> field of the <a>Query</a> operation.</p>
    #[serde(rename = "AllowedGroupsColumnName")]
    pub allowed_groups_column_name: String,
}

/// <p>An attribute returned from an index query.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct AdditionalResultAttribute {
    /// <p>The key that identifies the attribute.</p>
    #[serde(rename = "Key")]
    pub key: String,
    /// <p>An object that contains the attribute value.</p>
    #[serde(rename = "Value")]
    pub value: AdditionalResultAttributeValue,
    /// <p>The data type of the <code>Value</code> property.</p>
    #[serde(rename = "ValueType")]
    pub value_type: AdditionalResultAttributeValueType,
}

/// <p>An attribute returned with a document from a search.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct AdditionalResultAttributeValue {
    /// <p>The text associated with the attribute and information about the highlight to apply to the text.</p>
    #[serde(rename = "TextWithHighlightsValue")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text_with_highlights_value: Option<TextWithHighlights>,
}

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct UnknownAdditionalResultAttributeValueType {
    name: String,
}

#[allow(non_camel_case_types)]
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
#[non_exhaustive]
pub enum AdditionalResultAttributeValueType {
    TextWithHighlightsValue,
    #[doc(hidden)]
    UnknownVariant(UnknownAdditionalResultAttributeValueType),
}

impl Default for AdditionalResultAttributeValueType {
    fn default() -> Self {
        "".into()
    }
}

impl fmt::Display for AdditionalResultAttributeValueType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.into())
    }
}

impl rusoto_core::param::ToParam for AdditionalResultAttributeValueType {
    fn to_param(&self) -> String {
        self.to_string()
    }
}

impl Into<String> for AdditionalResultAttributeValueType {
    fn into(self) -> String {
        match self {
            AdditionalResultAttributeValueType::TextWithHighlightsValue => {
                "TEXT_WITH_HIGHLIGHTS_VALUE".to_string()
            }
            AdditionalResultAttributeValueType::UnknownVariant(
                UnknownAdditionalResultAttributeValueType { name: original },
            ) => original,
        }
    }
}

impl<'a> Into<&'a str> for &'a AdditionalResultAttributeValueType {
    fn into(self) -> &'a str {
        match self {
            AdditionalResultAttributeValueType::TextWithHighlightsValue => {
                &"TEXT_WITH_HIGHLIGHTS_VALUE"
            }
            AdditionalResultAttributeValueType::UnknownVariant(
                UnknownAdditionalResultAttributeValueType { name: original },
            ) => original,
        }
    }
}

impl From<&str> for AdditionalResultAttributeValueType {
    fn from(name: &str) -> Self {
        match name {
            "TEXT_WITH_HIGHLIGHTS_VALUE" => {
                AdditionalResultAttributeValueType::TextWithHighlightsValue
            }
            _ => AdditionalResultAttributeValueType::UnknownVariant(
                UnknownAdditionalResultAttributeValueType {
                    name: name.to_owned(),
                },
            ),
        }
    }
}

impl From<String> for AdditionalResultAttributeValueType {
    fn from(name: String) -> Self {
        match &*name {
            "TEXT_WITH_HIGHLIGHTS_VALUE" => {
                AdditionalResultAttributeValueType::TextWithHighlightsValue
            }
            _ => AdditionalResultAttributeValueType::UnknownVariant(
                UnknownAdditionalResultAttributeValueType { name },
            ),
        }
    }
}

impl ::std::str::FromStr for AdditionalResultAttributeValueType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(s.into())
    }
}

#[cfg(any(test, feature = "serialize_structs"))]
impl Serialize for AdditionalResultAttributeValueType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.into())
    }
}

impl<'de> Deserialize<'de> for AdditionalResultAttributeValueType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Ok(String::deserialize(deserializer)?.into())
    }
}

/// <p>Provides filtering the query results based on document attributes.</p> <p>When you use the <code>AndAllFilters</code> or <code>OrAllFilters</code>, filters you can use 2 layers under the first attribute filter. For example, you can use:</p> <p> <code>&lt;AndAllFilters&gt;</code> </p> <ol> <li> <p> <code> &lt;OrAllFilters&gt;</code> </p> </li> <li> <p> <code> &lt;EqualTo&gt;</code> </p> </li> </ol> <p>If you use more than 2 layers, you receive a <code>ValidationException</code> exception with the message "<code>AttributeFilter</code> cannot have a depth of more than 2."</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct AttributeFilter {
    /// <p>Performs a logical <code>AND</code> operation on all supplied filters.</p>
    #[serde(rename = "AndAllFilters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub and_all_filters: Option<Vec<AttributeFilter>>,
    /// <p>Returns true when a document contains all of the specified document attributes. This filter is only applicable to <code>StringListValue</code> metadata.</p>
    #[serde(rename = "ContainsAll")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contains_all: Option<DocumentAttribute>,
    /// <p>Returns true when a document contains any of the specified document attributes. This filter is only applicable to <code>StringListValue</code> metadata.</p>
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

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct BatchDeleteDocumentRequest {
    #[serde(rename = "DataSourceSyncJobMetricTarget")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_source_sync_job_metric_target: Option<DataSourceSyncJobMetricTarget>,
    /// <p>One or more identifiers for documents to delete from the index.</p>
    #[serde(rename = "DocumentIdList")]
    pub document_id_list: Vec<String>,
    /// <p>The identifier of the index that contains the documents to delete.</p>
    #[serde(rename = "IndexId")]
    pub index_id: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct BatchDeleteDocumentResponse {
    /// <p>A list of documents that could not be removed from the index. Each entry contains an error message that indicates why the document couldn't be removed from the index.</p>
    #[serde(rename = "FailedDocuments")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failed_documents: Option<Vec<BatchDeleteDocumentResponseFailedDocument>>,
}

/// <p>Provides information about documents that could not be removed from an index by the <a>BatchDeleteDocument</a> operation.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct BatchDeleteDocumentResponseFailedDocument {
    /// <p>The error code for why the document couldn't be removed from the index.</p>
    #[serde(rename = "ErrorCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_code: Option<ErrorCode>,
    /// <p>An explanation for why the document couldn't be removed from the index.</p>
    #[serde(rename = "ErrorMessage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
    /// <p>The identifier of the document that couldn't be removed from the index.</p>
    #[serde(rename = "Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct BatchPutDocumentRequest {
    /// <p>One or more documents to add to the index. </p> <p>Documents have the following file size limits.</p> <ul> <li> <p>5 MB total size for inline documents</p> </li> <li> <p>50 MB total size for files from an S3 bucket</p> </li> <li> <p>5 MB extracted text for any file</p> </li> </ul> <p>For more information about file size and transaction per second quotas, see <a href="https://docs.aws.amazon.com/kendra/latest/dg/quotas.html">Quotas</a>.</p>
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

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct BatchPutDocumentResponse {
    /// <p>A list of documents that were not added to the index because the document failed a validation check. Each document contains an error message that indicates why the document couldn't be added to the index.</p> <p>If there was an error adding a document to an index the error is reported in your AWS CloudWatch log. For more information, see <a href="https://docs.aws.amazon.com/kendra/latest/dg/cloudwatch-logs.html">Monitoring Amazon Kendra with Amazon CloudWatch Logs</a> </p>
    #[serde(rename = "FailedDocuments")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failed_documents: Option<Vec<BatchPutDocumentResponseFailedDocument>>,
}

/// <p>Provides information about a document that could not be indexed.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct BatchPutDocumentResponseFailedDocument {
    /// <p>The type of error that caused the document to fail to be indexed.</p>
    #[serde(rename = "ErrorCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_code: Option<ErrorCode>,
    /// <p>A description of the reason why the document could not be indexed.</p>
    #[serde(rename = "ErrorMessage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
    /// <p>The unique identifier of the document.</p>
    #[serde(rename = "Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}

/// <p>Specifies capacity units configured for your index. You can add and remove capacity units to tune an index to your requirements.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct CapacityUnitsConfiguration {
    /// <p>The amount of extra query capacity for an index. Each capacity unit provides 0.5 queries per second and 40,000 queries per day.</p>
    #[serde(rename = "QueryCapacityUnits")]
    pub query_capacity_units: i64,
    /// <p>The amount of extra storage capacity for an index. Each capacity unit provides 150 Gb of storage space or 500,000 documents, whichever is reached first.</p>
    #[serde(rename = "StorageCapacityUnits")]
    pub storage_capacity_units: i64,
}

/// <p>Gathers information about when a particular result was clicked by a user. Your application uses the <a>SubmitFeedback</a> operation to provide click information.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ClickFeedback {
    /// <p>The Unix timestamp of the date and time that the result was clicked.</p>
    #[serde(rename = "ClickTime")]
    pub click_time: f64,
    /// <p>The unique identifier of the search result that was clicked.</p>
    #[serde(rename = "ResultId")]
    pub result_id: String,
}

/// <p>Provides information about how Amazon Kendra should use the columns of a database in an index.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
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

/// <p>Specifies the attachment settings for the Confluence data source. Attachment settings are optional, if you don't specify settings attachments, Amazon Kendra won't index them.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct ConfluenceAttachmentConfiguration {
    /// <p>Defines how attachment metadata fields should be mapped to index fields. Before you can map a field, you must first create an index field with a matching type using the console or the <code>UpdateIndex</code> operation.</p> <p>If you specify the <code>AttachentFieldMappings</code> parameter, you must specify at least one field mapping.</p>
    #[serde(rename = "AttachmentFieldMappings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attachment_field_mappings: Option<Vec<ConfluenceAttachmentToIndexFieldMapping>>,
    /// <p>Indicates whether Amazon Kendra indexes attachments to the pages and blogs in the Confluence data source. </p>
    #[serde(rename = "CrawlAttachments")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub crawl_attachments: Option<bool>,
}

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct UnknownConfluenceAttachmentFieldName {
    name: String,
}

#[allow(non_camel_case_types)]
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
#[non_exhaustive]
pub enum ConfluenceAttachmentFieldName {
    Author,
    ContentType,
    CreatedDate,
    DisplayUrl,
    FileSize,
    ItemType,
    ParentId,
    SpaceKey,
    SpaceName,
    Url,
    Version,
    #[doc(hidden)]
    UnknownVariant(UnknownConfluenceAttachmentFieldName),
}

impl Default for ConfluenceAttachmentFieldName {
    fn default() -> Self {
        "".into()
    }
}

impl fmt::Display for ConfluenceAttachmentFieldName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.into())
    }
}

impl rusoto_core::param::ToParam for ConfluenceAttachmentFieldName {
    fn to_param(&self) -> String {
        self.to_string()
    }
}

impl Into<String> for ConfluenceAttachmentFieldName {
    fn into(self) -> String {
        match self {
            ConfluenceAttachmentFieldName::Author => "AUTHOR".to_string(),
            ConfluenceAttachmentFieldName::ContentType => "CONTENT_TYPE".to_string(),
            ConfluenceAttachmentFieldName::CreatedDate => "CREATED_DATE".to_string(),
            ConfluenceAttachmentFieldName::DisplayUrl => "DISPLAY_URL".to_string(),
            ConfluenceAttachmentFieldName::FileSize => "FILE_SIZE".to_string(),
            ConfluenceAttachmentFieldName::ItemType => "ITEM_TYPE".to_string(),
            ConfluenceAttachmentFieldName::ParentId => "PARENT_ID".to_string(),
            ConfluenceAttachmentFieldName::SpaceKey => "SPACE_KEY".to_string(),
            ConfluenceAttachmentFieldName::SpaceName => "SPACE_NAME".to_string(),
            ConfluenceAttachmentFieldName::Url => "URL".to_string(),
            ConfluenceAttachmentFieldName::Version => "VERSION".to_string(),
            ConfluenceAttachmentFieldName::UnknownVariant(
                UnknownConfluenceAttachmentFieldName { name: original },
            ) => original,
        }
    }
}

impl<'a> Into<&'a str> for &'a ConfluenceAttachmentFieldName {
    fn into(self) -> &'a str {
        match self {
            ConfluenceAttachmentFieldName::Author => &"AUTHOR",
            ConfluenceAttachmentFieldName::ContentType => &"CONTENT_TYPE",
            ConfluenceAttachmentFieldName::CreatedDate => &"CREATED_DATE",
            ConfluenceAttachmentFieldName::DisplayUrl => &"DISPLAY_URL",
            ConfluenceAttachmentFieldName::FileSize => &"FILE_SIZE",
            ConfluenceAttachmentFieldName::ItemType => &"ITEM_TYPE",
            ConfluenceAttachmentFieldName::ParentId => &"PARENT_ID",
            ConfluenceAttachmentFieldName::SpaceKey => &"SPACE_KEY",
            ConfluenceAttachmentFieldName::SpaceName => &"SPACE_NAME",
            ConfluenceAttachmentFieldName::Url => &"URL",
            ConfluenceAttachmentFieldName::Version => &"VERSION",
            ConfluenceAttachmentFieldName::UnknownVariant(
                UnknownConfluenceAttachmentFieldName { name: original },
            ) => original,
        }
    }
}

impl From<&str> for ConfluenceAttachmentFieldName {
    fn from(name: &str) -> Self {
        match name {
            "AUTHOR" => ConfluenceAttachmentFieldName::Author,
            "CONTENT_TYPE" => ConfluenceAttachmentFieldName::ContentType,
            "CREATED_DATE" => ConfluenceAttachmentFieldName::CreatedDate,
            "DISPLAY_URL" => ConfluenceAttachmentFieldName::DisplayUrl,
            "FILE_SIZE" => ConfluenceAttachmentFieldName::FileSize,
            "ITEM_TYPE" => ConfluenceAttachmentFieldName::ItemType,
            "PARENT_ID" => ConfluenceAttachmentFieldName::ParentId,
            "SPACE_KEY" => ConfluenceAttachmentFieldName::SpaceKey,
            "SPACE_NAME" => ConfluenceAttachmentFieldName::SpaceName,
            "URL" => ConfluenceAttachmentFieldName::Url,
            "VERSION" => ConfluenceAttachmentFieldName::Version,
            _ => ConfluenceAttachmentFieldName::UnknownVariant(
                UnknownConfluenceAttachmentFieldName {
                    name: name.to_owned(),
                },
            ),
        }
    }
}

impl From<String> for ConfluenceAttachmentFieldName {
    fn from(name: String) -> Self {
        match &*name {
            "AUTHOR" => ConfluenceAttachmentFieldName::Author,
            "CONTENT_TYPE" => ConfluenceAttachmentFieldName::ContentType,
            "CREATED_DATE" => ConfluenceAttachmentFieldName::CreatedDate,
            "DISPLAY_URL" => ConfluenceAttachmentFieldName::DisplayUrl,
            "FILE_SIZE" => ConfluenceAttachmentFieldName::FileSize,
            "ITEM_TYPE" => ConfluenceAttachmentFieldName::ItemType,
            "PARENT_ID" => ConfluenceAttachmentFieldName::ParentId,
            "SPACE_KEY" => ConfluenceAttachmentFieldName::SpaceKey,
            "SPACE_NAME" => ConfluenceAttachmentFieldName::SpaceName,
            "URL" => ConfluenceAttachmentFieldName::Url,
            "VERSION" => ConfluenceAttachmentFieldName::Version,
            _ => ConfluenceAttachmentFieldName::UnknownVariant(
                UnknownConfluenceAttachmentFieldName { name },
            ),
        }
    }
}

impl ::std::str::FromStr for ConfluenceAttachmentFieldName {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(s.into())
    }
}

impl Serialize for ConfluenceAttachmentFieldName {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.into())
    }
}

impl<'de> Deserialize<'de> for ConfluenceAttachmentFieldName {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Ok(String::deserialize(deserializer)?.into())
    }
}

/// <p>Defines the mapping between a field in the Confluence data source to a Amazon Kendra index field.</p> <p>You must first create the index field using the operation. </p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct ConfluenceAttachmentToIndexFieldMapping {
    /// <p>The name of the field in the data source. </p> <p>You must first create the index field using the operation. </p>
    #[serde(rename = "DataSourceFieldName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_source_field_name: Option<ConfluenceAttachmentFieldName>,
    /// <p>The format for date fields in the data source. If the field specified in <code>DataSourceFieldName</code> is a date field you must specify the date format. If the field is not a date field, an exception is thrown.</p>
    #[serde(rename = "DateFieldFormat")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date_field_format: Option<String>,
    /// <p>The name of the index field to map to the Confluence data source field. The index field type must match the Confluence field type.</p>
    #[serde(rename = "IndexFieldName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub index_field_name: Option<String>,
}

/// <p>Specifies the blog settings for the Confluence data source. Blogs are always indexed unless filtered from the index by the <code>ExclusionPatterns</code> or <code>InclusionPatterns</code> fields in the data type.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct ConfluenceBlogConfiguration {
    /// <p>Defines how blog metadata fields should be mapped to index fields. Before you can map a field, you must first create an index field with a matching type using the console or the <code>UpdateIndex</code> operation.</p> <p>If you specify the <code>BlogFieldMappings</code> parameter, you must specify at least one field mapping.</p>
    #[serde(rename = "BlogFieldMappings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blog_field_mappings: Option<Vec<ConfluenceBlogToIndexFieldMapping>>,
}

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct UnknownConfluenceBlogFieldName {
    name: String,
}

#[allow(non_camel_case_types)]
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
#[non_exhaustive]
pub enum ConfluenceBlogFieldName {
    Author,
    DisplayUrl,
    ItemType,
    Labels,
    PublishDate,
    SpaceKey,
    SpaceName,
    Url,
    Version,
    #[doc(hidden)]
    UnknownVariant(UnknownConfluenceBlogFieldName),
}

impl Default for ConfluenceBlogFieldName {
    fn default() -> Self {
        "".into()
    }
}

impl fmt::Display for ConfluenceBlogFieldName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.into())
    }
}

impl rusoto_core::param::ToParam for ConfluenceBlogFieldName {
    fn to_param(&self) -> String {
        self.to_string()
    }
}

impl Into<String> for ConfluenceBlogFieldName {
    fn into(self) -> String {
        match self {
            ConfluenceBlogFieldName::Author => "AUTHOR".to_string(),
            ConfluenceBlogFieldName::DisplayUrl => "DISPLAY_URL".to_string(),
            ConfluenceBlogFieldName::ItemType => "ITEM_TYPE".to_string(),
            ConfluenceBlogFieldName::Labels => "LABELS".to_string(),
            ConfluenceBlogFieldName::PublishDate => "PUBLISH_DATE".to_string(),
            ConfluenceBlogFieldName::SpaceKey => "SPACE_KEY".to_string(),
            ConfluenceBlogFieldName::SpaceName => "SPACE_NAME".to_string(),
            ConfluenceBlogFieldName::Url => "URL".to_string(),
            ConfluenceBlogFieldName::Version => "VERSION".to_string(),
            ConfluenceBlogFieldName::UnknownVariant(UnknownConfluenceBlogFieldName {
                name: original,
            }) => original,
        }
    }
}

impl<'a> Into<&'a str> for &'a ConfluenceBlogFieldName {
    fn into(self) -> &'a str {
        match self {
            ConfluenceBlogFieldName::Author => &"AUTHOR",
            ConfluenceBlogFieldName::DisplayUrl => &"DISPLAY_URL",
            ConfluenceBlogFieldName::ItemType => &"ITEM_TYPE",
            ConfluenceBlogFieldName::Labels => &"LABELS",
            ConfluenceBlogFieldName::PublishDate => &"PUBLISH_DATE",
            ConfluenceBlogFieldName::SpaceKey => &"SPACE_KEY",
            ConfluenceBlogFieldName::SpaceName => &"SPACE_NAME",
            ConfluenceBlogFieldName::Url => &"URL",
            ConfluenceBlogFieldName::Version => &"VERSION",
            ConfluenceBlogFieldName::UnknownVariant(UnknownConfluenceBlogFieldName {
                name: original,
            }) => original,
        }
    }
}

impl From<&str> for ConfluenceBlogFieldName {
    fn from(name: &str) -> Self {
        match name {
            "AUTHOR" => ConfluenceBlogFieldName::Author,
            "DISPLAY_URL" => ConfluenceBlogFieldName::DisplayUrl,
            "ITEM_TYPE" => ConfluenceBlogFieldName::ItemType,
            "LABELS" => ConfluenceBlogFieldName::Labels,
            "PUBLISH_DATE" => ConfluenceBlogFieldName::PublishDate,
            "SPACE_KEY" => ConfluenceBlogFieldName::SpaceKey,
            "SPACE_NAME" => ConfluenceBlogFieldName::SpaceName,
            "URL" => ConfluenceBlogFieldName::Url,
            "VERSION" => ConfluenceBlogFieldName::Version,
            _ => ConfluenceBlogFieldName::UnknownVariant(UnknownConfluenceBlogFieldName {
                name: name.to_owned(),
            }),
        }
    }
}

impl From<String> for ConfluenceBlogFieldName {
    fn from(name: String) -> Self {
        match &*name {
            "AUTHOR" => ConfluenceBlogFieldName::Author,
            "DISPLAY_URL" => ConfluenceBlogFieldName::DisplayUrl,
            "ITEM_TYPE" => ConfluenceBlogFieldName::ItemType,
            "LABELS" => ConfluenceBlogFieldName::Labels,
            "PUBLISH_DATE" => ConfluenceBlogFieldName::PublishDate,
            "SPACE_KEY" => ConfluenceBlogFieldName::SpaceKey,
            "SPACE_NAME" => ConfluenceBlogFieldName::SpaceName,
            "URL" => ConfluenceBlogFieldName::Url,
            "VERSION" => ConfluenceBlogFieldName::Version,
            _ => ConfluenceBlogFieldName::UnknownVariant(UnknownConfluenceBlogFieldName { name }),
        }
    }
}

impl ::std::str::FromStr for ConfluenceBlogFieldName {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(s.into())
    }
}

impl Serialize for ConfluenceBlogFieldName {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.into())
    }
}

impl<'de> Deserialize<'de> for ConfluenceBlogFieldName {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Ok(String::deserialize(deserializer)?.into())
    }
}

/// <p>Defines the mapping between a blog field in the Confluence data source to a Amazon Kendra index field.</p> <p>You must first create the index field using the operation. </p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct ConfluenceBlogToIndexFieldMapping {
    /// <p>The name of the field in the data source. </p>
    #[serde(rename = "DataSourceFieldName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_source_field_name: Option<ConfluenceBlogFieldName>,
    /// <p>The format for date fields in the data source. If the field specified in <code>DataSourceFieldName</code> is a date field you must specify the date format. If the field is not a date field, an exception is thrown.</p>
    #[serde(rename = "DateFieldFormat")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date_field_format: Option<String>,
    /// <p>The name of the index field to map to the Confluence data source field. The index field type must match the Confluence field type.</p>
    #[serde(rename = "IndexFieldName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub index_field_name: Option<String>,
}

/// <p>Provides configuration information for data sources that connect to Confluence.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct ConfluenceConfiguration {
    /// <p>Specifies configuration information for indexing attachments to Confluence blogs and pages.</p>
    #[serde(rename = "AttachmentConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attachment_configuration: Option<ConfluenceAttachmentConfiguration>,
    /// <p> Specifies configuration information for indexing Confluence blogs.</p>
    #[serde(rename = "BlogConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blog_configuration: Option<ConfluenceBlogConfiguration>,
    /// <p>A list of regular expression patterns that apply to a URL on the Confluence server. An exclusion pattern can apply to a blog post, a page, a space, or an attachment. Items that match the pattern are excluded from the index. Items that don't match the pattern are included in the index. If a item matches both an exclusion pattern and an inclusion pattern, the item isn't included in the index.</p>
    #[serde(rename = "ExclusionPatterns")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exclusion_patterns: Option<Vec<String>>,
    /// <p>A list of regular expression patterns that apply to a URL on the Confluence server. An inclusion pattern can apply to a blog post, a page, a space, or an attachment. Items that match the patterns are included in the index. Items that don't match the pattern are excluded from the index. If an item matches both an inclusion pattern and an exclusion pattern, the item isn't included in the index.</p>
    #[serde(rename = "InclusionPatterns")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inclusion_patterns: Option<Vec<String>>,
    /// <p>Specifies configuration information for indexing Confluence pages.</p>
    #[serde(rename = "PageConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_configuration: Option<ConfluencePageConfiguration>,
    /// <p><p>The Amazon Resource Name (ARN) of an AWS Secrets Manager secret that contains the key/value pairs required to connect to your Confluence server. The secret must contain a JSON structure with the following keys:</p> <ul> <li> <p>username - The user name or email address of a user with administrative privileges for the Confluence server.</p> </li> <li> <p>password - The password associated with the user logging in to the Confluence server.</p> </li> </ul></p>
    #[serde(rename = "SecretArn")]
    pub secret_arn: String,
    /// <p>The URL of your Confluence instance. Use the full URL of the server. For example, <code>https://server.example.com:port/</code>. You can also use an IP address, for example, <code>https://192.168.1.113/</code>.</p>
    #[serde(rename = "ServerUrl")]
    pub server_url: String,
    /// <p>Specifies configuration information for indexing Confluence spaces.</p>
    #[serde(rename = "SpaceConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub space_configuration: Option<ConfluenceSpaceConfiguration>,
    /// <p>Specifies the version of the Confluence installation that you are connecting to.</p>
    #[serde(rename = "Version")]
    pub version: ConfluenceVersion,
    /// <p>Specifies the information for connecting to an Amazon VPC.</p>
    #[serde(rename = "VpcConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_configuration: Option<DataSourceVpcConfiguration>,
}

/// <p>Specifies the page settings for the Confluence data source.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct ConfluencePageConfiguration {
    /// <p>Defines how page metadata fields should be mapped to index fields. Before you can map a field, you must first create an index field with a matching type using the console or the <code>UpdateIndex</code> operation.</p> <p>If you specify the <code>PageFieldMappings</code> parameter, you must specify at least one field mapping.</p>
    #[serde(rename = "PageFieldMappings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_field_mappings: Option<Vec<ConfluencePageToIndexFieldMapping>>,
}

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct UnknownConfluencePageFieldName {
    name: String,
}

#[allow(non_camel_case_types)]
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
#[non_exhaustive]
pub enum ConfluencePageFieldName {
    Author,
    ContentStatus,
    CreatedDate,
    DisplayUrl,
    ItemType,
    Labels,
    ModifiedDate,
    ParentId,
    SpaceKey,
    SpaceName,
    Url,
    Version,
    #[doc(hidden)]
    UnknownVariant(UnknownConfluencePageFieldName),
}

impl Default for ConfluencePageFieldName {
    fn default() -> Self {
        "".into()
    }
}

impl fmt::Display for ConfluencePageFieldName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.into())
    }
}

impl rusoto_core::param::ToParam for ConfluencePageFieldName {
    fn to_param(&self) -> String {
        self.to_string()
    }
}

impl Into<String> for ConfluencePageFieldName {
    fn into(self) -> String {
        match self {
            ConfluencePageFieldName::Author => "AUTHOR".to_string(),
            ConfluencePageFieldName::ContentStatus => "CONTENT_STATUS".to_string(),
            ConfluencePageFieldName::CreatedDate => "CREATED_DATE".to_string(),
            ConfluencePageFieldName::DisplayUrl => "DISPLAY_URL".to_string(),
            ConfluencePageFieldName::ItemType => "ITEM_TYPE".to_string(),
            ConfluencePageFieldName::Labels => "LABELS".to_string(),
            ConfluencePageFieldName::ModifiedDate => "MODIFIED_DATE".to_string(),
            ConfluencePageFieldName::ParentId => "PARENT_ID".to_string(),
            ConfluencePageFieldName::SpaceKey => "SPACE_KEY".to_string(),
            ConfluencePageFieldName::SpaceName => "SPACE_NAME".to_string(),
            ConfluencePageFieldName::Url => "URL".to_string(),
            ConfluencePageFieldName::Version => "VERSION".to_string(),
            ConfluencePageFieldName::UnknownVariant(UnknownConfluencePageFieldName {
                name: original,
            }) => original,
        }
    }
}

impl<'a> Into<&'a str> for &'a ConfluencePageFieldName {
    fn into(self) -> &'a str {
        match self {
            ConfluencePageFieldName::Author => &"AUTHOR",
            ConfluencePageFieldName::ContentStatus => &"CONTENT_STATUS",
            ConfluencePageFieldName::CreatedDate => &"CREATED_DATE",
            ConfluencePageFieldName::DisplayUrl => &"DISPLAY_URL",
            ConfluencePageFieldName::ItemType => &"ITEM_TYPE",
            ConfluencePageFieldName::Labels => &"LABELS",
            ConfluencePageFieldName::ModifiedDate => &"MODIFIED_DATE",
            ConfluencePageFieldName::ParentId => &"PARENT_ID",
            ConfluencePageFieldName::SpaceKey => &"SPACE_KEY",
            ConfluencePageFieldName::SpaceName => &"SPACE_NAME",
            ConfluencePageFieldName::Url => &"URL",
            ConfluencePageFieldName::Version => &"VERSION",
            ConfluencePageFieldName::UnknownVariant(UnknownConfluencePageFieldName {
                name: original,
            }) => original,
        }
    }
}

impl From<&str> for ConfluencePageFieldName {
    fn from(name: &str) -> Self {
        match name {
            "AUTHOR" => ConfluencePageFieldName::Author,
            "CONTENT_STATUS" => ConfluencePageFieldName::ContentStatus,
            "CREATED_DATE" => ConfluencePageFieldName::CreatedDate,
            "DISPLAY_URL" => ConfluencePageFieldName::DisplayUrl,
            "ITEM_TYPE" => ConfluencePageFieldName::ItemType,
            "LABELS" => ConfluencePageFieldName::Labels,
            "MODIFIED_DATE" => ConfluencePageFieldName::ModifiedDate,
            "PARENT_ID" => ConfluencePageFieldName::ParentId,
            "SPACE_KEY" => ConfluencePageFieldName::SpaceKey,
            "SPACE_NAME" => ConfluencePageFieldName::SpaceName,
            "URL" => ConfluencePageFieldName::Url,
            "VERSION" => ConfluencePageFieldName::Version,
            _ => ConfluencePageFieldName::UnknownVariant(UnknownConfluencePageFieldName {
                name: name.to_owned(),
            }),
        }
    }
}

impl From<String> for ConfluencePageFieldName {
    fn from(name: String) -> Self {
        match &*name {
            "AUTHOR" => ConfluencePageFieldName::Author,
            "CONTENT_STATUS" => ConfluencePageFieldName::ContentStatus,
            "CREATED_DATE" => ConfluencePageFieldName::CreatedDate,
            "DISPLAY_URL" => ConfluencePageFieldName::DisplayUrl,
            "ITEM_TYPE" => ConfluencePageFieldName::ItemType,
            "LABELS" => ConfluencePageFieldName::Labels,
            "MODIFIED_DATE" => ConfluencePageFieldName::ModifiedDate,
            "PARENT_ID" => ConfluencePageFieldName::ParentId,
            "SPACE_KEY" => ConfluencePageFieldName::SpaceKey,
            "SPACE_NAME" => ConfluencePageFieldName::SpaceName,
            "URL" => ConfluencePageFieldName::Url,
            "VERSION" => ConfluencePageFieldName::Version,
            _ => ConfluencePageFieldName::UnknownVariant(UnknownConfluencePageFieldName { name }),
        }
    }
}

impl ::std::str::FromStr for ConfluencePageFieldName {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(s.into())
    }
}

impl Serialize for ConfluencePageFieldName {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.into())
    }
}

impl<'de> Deserialize<'de> for ConfluencePageFieldName {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Ok(String::deserialize(deserializer)?.into())
    }
}

/// <p>Defines the mapping between a field in the Confluence data source to a Amazon Kendra index field.</p> <p>You must first create the index field using the operation. </p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct ConfluencePageToIndexFieldMapping {
    /// <p>The name of the field in the data source. </p>
    #[serde(rename = "DataSourceFieldName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_source_field_name: Option<ConfluencePageFieldName>,
    /// <p>The format for date fields in the data source. If the field specified in <code>DataSourceFieldName</code> is a date field you must specify the date format. If the field is not a date field, an exception is thrown.</p>
    #[serde(rename = "DateFieldFormat")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date_field_format: Option<String>,
    /// <p>The name of the index field to map to the Confluence data source field. The index field type must match the Confluence field type.</p>
    #[serde(rename = "IndexFieldName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub index_field_name: Option<String>,
}

/// <p>Specifies the configuration for indexing Confluence spaces.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct ConfluenceSpaceConfiguration {
    /// <p>Specifies whether Amazon Kendra should index archived spaces.</p>
    #[serde(rename = "CrawlArchivedSpaces")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub crawl_archived_spaces: Option<bool>,
    /// <p>Specifies whether Amazon Kendra should index personal spaces. Users can add restrictions to items in personal spaces. If personal spaces are indexed, queries without user context information may return restricted items from a personal space in their results. For more information, see <a href="https://docs.aws.amazon.com/kendra/latest/dg/user-context-filter.html">Filtering on user context</a>.</p>
    #[serde(rename = "CrawlPersonalSpaces")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub crawl_personal_spaces: Option<bool>,
    /// <p>A list of space keys of Confluence spaces. If you include a key, the blogs, documents, and attachments in the space are not indexed. If a space is in both the <code>ExcludeSpaces</code> and the <code>IncludeSpaces</code> list, the space is excluded.</p>
    #[serde(rename = "ExcludeSpaces")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exclude_spaces: Option<Vec<String>>,
    /// <p>A list of space keys for Confluence spaces. If you include a key, the blogs, documents, and attachments in the space are indexed. Spaces that aren't in the list aren't indexed. A space in the list must exist. Otherwise, Amazon Kendra logs an error when the data source is synchronized. If a space is in both the <code>IncludeSpaces</code> and the <code>ExcludeSpaces</code> list, the space is excluded.</p>
    #[serde(rename = "IncludeSpaces")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_spaces: Option<Vec<String>>,
    /// <p>Defines how space metadata fields should be mapped to index fields. Before you can map a field, you must first create an index field with a matching type using the console or the <code>UpdateIndex</code> operation.</p> <p>If you specify the <code>SpaceFieldMappings</code> parameter, you must specify at least one field mapping.</p>
    #[serde(rename = "SpaceFieldMappings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub space_field_mappings: Option<Vec<ConfluenceSpaceToIndexFieldMapping>>,
}

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct UnknownConfluenceSpaceFieldName {
    name: String,
}

#[allow(non_camel_case_types)]
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
#[non_exhaustive]
pub enum ConfluenceSpaceFieldName {
    DisplayUrl,
    ItemType,
    SpaceKey,
    Url,
    #[doc(hidden)]
    UnknownVariant(UnknownConfluenceSpaceFieldName),
}

impl Default for ConfluenceSpaceFieldName {
    fn default() -> Self {
        "".into()
    }
}

impl fmt::Display for ConfluenceSpaceFieldName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.into())
    }
}

impl rusoto_core::param::ToParam for ConfluenceSpaceFieldName {
    fn to_param(&self) -> String {
        self.to_string()
    }
}

impl Into<String> for ConfluenceSpaceFieldName {
    fn into(self) -> String {
        match self {
            ConfluenceSpaceFieldName::DisplayUrl => "DISPLAY_URL".to_string(),
            ConfluenceSpaceFieldName::ItemType => "ITEM_TYPE".to_string(),
            ConfluenceSpaceFieldName::SpaceKey => "SPACE_KEY".to_string(),
            ConfluenceSpaceFieldName::Url => "URL".to_string(),
            ConfluenceSpaceFieldName::UnknownVariant(UnknownConfluenceSpaceFieldName {
                name: original,
            }) => original,
        }
    }
}

impl<'a> Into<&'a str> for &'a ConfluenceSpaceFieldName {
    fn into(self) -> &'a str {
        match self {
            ConfluenceSpaceFieldName::DisplayUrl => &"DISPLAY_URL",
            ConfluenceSpaceFieldName::ItemType => &"ITEM_TYPE",
            ConfluenceSpaceFieldName::SpaceKey => &"SPACE_KEY",
            ConfluenceSpaceFieldName::Url => &"URL",
            ConfluenceSpaceFieldName::UnknownVariant(UnknownConfluenceSpaceFieldName {
                name: original,
            }) => original,
        }
    }
}

impl From<&str> for ConfluenceSpaceFieldName {
    fn from(name: &str) -> Self {
        match name {
            "DISPLAY_URL" => ConfluenceSpaceFieldName::DisplayUrl,
            "ITEM_TYPE" => ConfluenceSpaceFieldName::ItemType,
            "SPACE_KEY" => ConfluenceSpaceFieldName::SpaceKey,
            "URL" => ConfluenceSpaceFieldName::Url,
            _ => ConfluenceSpaceFieldName::UnknownVariant(UnknownConfluenceSpaceFieldName {
                name: name.to_owned(),
            }),
        }
    }
}

impl From<String> for ConfluenceSpaceFieldName {
    fn from(name: String) -> Self {
        match &*name {
            "DISPLAY_URL" => ConfluenceSpaceFieldName::DisplayUrl,
            "ITEM_TYPE" => ConfluenceSpaceFieldName::ItemType,
            "SPACE_KEY" => ConfluenceSpaceFieldName::SpaceKey,
            "URL" => ConfluenceSpaceFieldName::Url,
            _ => ConfluenceSpaceFieldName::UnknownVariant(UnknownConfluenceSpaceFieldName { name }),
        }
    }
}

impl ::std::str::FromStr for ConfluenceSpaceFieldName {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(s.into())
    }
}

impl Serialize for ConfluenceSpaceFieldName {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.into())
    }
}

impl<'de> Deserialize<'de> for ConfluenceSpaceFieldName {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Ok(String::deserialize(deserializer)?.into())
    }
}

/// <p>Defines the mapping between a field in the Confluence data source to a Amazon Kendra index field.</p> <p>You must first create the index field using the operation. </p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct ConfluenceSpaceToIndexFieldMapping {
    /// <p>The name of the field in the data source. </p>
    #[serde(rename = "DataSourceFieldName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_source_field_name: Option<ConfluenceSpaceFieldName>,
    /// <p>The format for date fields in the data source. If the field specified in <code>DataSourceFieldName</code> is a date field you must specify the date format. If the field is not a date field, an exception is thrown.</p>
    #[serde(rename = "DateFieldFormat")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date_field_format: Option<String>,
    /// <p>The name of the index field to map to the Confluence data source field. The index field type must match the Confluence field type.</p>
    #[serde(rename = "IndexFieldName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub index_field_name: Option<String>,
}

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct UnknownConfluenceVersion {
    name: String,
}

#[allow(non_camel_case_types)]
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
#[non_exhaustive]
pub enum ConfluenceVersion {
    Cloud,
    Server,
    #[doc(hidden)]
    UnknownVariant(UnknownConfluenceVersion),
}

impl Default for ConfluenceVersion {
    fn default() -> Self {
        "".into()
    }
}

impl fmt::Display for ConfluenceVersion {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.into())
    }
}

impl rusoto_core::param::ToParam for ConfluenceVersion {
    fn to_param(&self) -> String {
        self.to_string()
    }
}

impl Into<String> for ConfluenceVersion {
    fn into(self) -> String {
        match self {
            ConfluenceVersion::Cloud => "CLOUD".to_string(),
            ConfluenceVersion::Server => "SERVER".to_string(),
            ConfluenceVersion::UnknownVariant(UnknownConfluenceVersion { name: original }) => {
                original
            }
        }
    }
}

impl<'a> Into<&'a str> for &'a ConfluenceVersion {
    fn into(self) -> &'a str {
        match self {
            ConfluenceVersion::Cloud => &"CLOUD",
            ConfluenceVersion::Server => &"SERVER",
            ConfluenceVersion::UnknownVariant(UnknownConfluenceVersion { name: original }) => {
                original
            }
        }
    }
}

impl From<&str> for ConfluenceVersion {
    fn from(name: &str) -> Self {
        match name {
            "CLOUD" => ConfluenceVersion::Cloud,
            "SERVER" => ConfluenceVersion::Server,
            _ => ConfluenceVersion::UnknownVariant(UnknownConfluenceVersion {
                name: name.to_owned(),
            }),
        }
    }
}

impl From<String> for ConfluenceVersion {
    fn from(name: String) -> Self {
        match &*name {
            "CLOUD" => ConfluenceVersion::Cloud,
            "SERVER" => ConfluenceVersion::Server,
            _ => ConfluenceVersion::UnknownVariant(UnknownConfluenceVersion { name }),
        }
    }
}

impl ::std::str::FromStr for ConfluenceVersion {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(s.into())
    }
}

impl Serialize for ConfluenceVersion {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.into())
    }
}

impl<'de> Deserialize<'de> for ConfluenceVersion {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Ok(String::deserialize(deserializer)?.into())
    }
}

/// <p>Provides the information necessary to connect to a database.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
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

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct UnknownContentType {
    name: String,
}

#[allow(non_camel_case_types)]
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
#[non_exhaustive]
pub enum ContentType {
    Html,
    MsWord,
    Pdf,
    PlainText,
    Ppt,
    #[doc(hidden)]
    UnknownVariant(UnknownContentType),
}

impl Default for ContentType {
    fn default() -> Self {
        "".into()
    }
}

impl fmt::Display for ContentType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.into())
    }
}

impl rusoto_core::param::ToParam for ContentType {
    fn to_param(&self) -> String {
        self.to_string()
    }
}

impl Into<String> for ContentType {
    fn into(self) -> String {
        match self {
            ContentType::Html => "HTML".to_string(),
            ContentType::MsWord => "MS_WORD".to_string(),
            ContentType::Pdf => "PDF".to_string(),
            ContentType::PlainText => "PLAIN_TEXT".to_string(),
            ContentType::Ppt => "PPT".to_string(),
            ContentType::UnknownVariant(UnknownContentType { name: original }) => original,
        }
    }
}

impl<'a> Into<&'a str> for &'a ContentType {
    fn into(self) -> &'a str {
        match self {
            ContentType::Html => &"HTML",
            ContentType::MsWord => &"MS_WORD",
            ContentType::Pdf => &"PDF",
            ContentType::PlainText => &"PLAIN_TEXT",
            ContentType::Ppt => &"PPT",
            ContentType::UnknownVariant(UnknownContentType { name: original }) => original,
        }
    }
}

impl From<&str> for ContentType {
    fn from(name: &str) -> Self {
        match name {
            "HTML" => ContentType::Html,
            "MS_WORD" => ContentType::MsWord,
            "PDF" => ContentType::Pdf,
            "PLAIN_TEXT" => ContentType::PlainText,
            "PPT" => ContentType::Ppt,
            _ => ContentType::UnknownVariant(UnknownContentType {
                name: name.to_owned(),
            }),
        }
    }
}

impl From<String> for ContentType {
    fn from(name: String) -> Self {
        match &*name {
            "HTML" => ContentType::Html,
            "MS_WORD" => ContentType::MsWord,
            "PDF" => ContentType::Pdf,
            "PLAIN_TEXT" => ContentType::PlainText,
            "PPT" => ContentType::Ppt,
            _ => ContentType::UnknownVariant(UnknownContentType { name }),
        }
    }
}

impl ::std::str::FromStr for ContentType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(s.into())
    }
}

impl Serialize for ContentType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.into())
    }
}

#[cfg(feature = "deserialize_structs")]
impl<'de> Deserialize<'de> for ContentType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Ok(String::deserialize(deserializer)?.into())
    }
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateDataSourceRequest {
    /// <p>A token that you provide to identify the request to create a data source. Multiple calls to the <code>CreateDataSource</code> operation with the same client token will create only one data source.</p>
    #[serde(rename = "ClientToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
    /// <p>The connector configuration information that is required to access the repository.</p> <p>You can't specify the <code>Configuration</code> parameter when the <code>Type</code> parameter is set to <code>CUSTOM</code>. If you do, you receive a <code>ValidationException</code> exception.</p> <p>The <code>Configuration</code> parameter is required for all other data sources.</p>
    #[serde(rename = "Configuration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configuration: Option<DataSourceConfiguration>,
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
    /// <p>The Amazon Resource Name (ARN) of a role with permission to access the data source. For more information, see <a href="https://docs.aws.amazon.com/kendra/latest/dg/iam-roles.html">IAM Roles for Amazon Kendra</a>.</p> <p>You can't specify the <code>RoleArn</code> parameter when the <code>Type</code> parameter is set to <code>CUSTOM</code>. If you do, you receive a <code>ValidationException</code> exception.</p> <p>The <code>RoleArn</code> parameter is required for all other data sources.</p>
    #[serde(rename = "RoleArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_arn: Option<String>,
    /// <p>Sets the frequency that Amazon Kendra will check the documents in your repository and update the index. If you don't set a schedule Amazon Kendra will not periodically update the index. You can call the <code>StartDataSourceSyncJob</code> operation to update the index.</p> <p>You can't specify the <code>Schedule</code> parameter when the <code>Type</code> parameter is set to <code>CUSTOM</code>. If you do, you receive a <code>ValidationException</code> exception.</p>
    #[serde(rename = "Schedule")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schedule: Option<String>,
    /// <p>A list of key-value pairs that identify the data source. You can use the tags to identify and organize your resources and to control access to resources.</p>
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
    /// <p>The type of repository that contains the data source.</p>
    #[serde(rename = "Type")]
    pub type_: DataSourceType,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateDataSourceResponse {
    /// <p>A unique identifier for the data source.</p>
    #[serde(rename = "Id")]
    pub id: String,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateFaqRequest {
    /// <p>A token that you provide to identify the request to create a FAQ. Multiple calls to the <code>CreateFaqRequest</code> operation with the same client token will create only one FAQ. </p>
    #[serde(rename = "ClientToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
    /// <p>A description of the FAQ.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The format of the input file. You can choose between a basic CSV format, a CSV format that includes customs attributes in a header, and a JSON format that includes custom attributes.</p> <p>The format must match the format of the file stored in the S3 bucket identified in the <code>S3Path</code> parameter.</p> <p>For more information, see <a href="https://docs.aws.amazon.com/kendra/latest/dg/in-creating-faq.html">Adding questions and answers</a>.</p>
    #[serde(rename = "FileFormat")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_format: Option<FaqFileFormat>,
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
    /// <p>A list of key-value pairs that identify the FAQ. You can use the tags to identify and organize your resources and to control access to resources.</p>
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateFaqResponse {
    /// <p>The unique identifier of the FAQ.</p>
    #[serde(rename = "Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateIndexRequest {
    /// <p>A token that you provide to identify the request to create an index. Multiple calls to the <code>CreateIndex</code> operation with the same client token will create only one index.</p>
    #[serde(rename = "ClientToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
    /// <p>A description for the index.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The Amazon Kendra edition to use for the index. Choose <code>DEVELOPER_EDITION</code> for indexes intended for development, testing, or proof of concept. Use <code>ENTERPRISE_EDITION</code> for your production databases. Once you set the edition for an index, it can't be changed. </p> <p>The <code>Edition</code> parameter is optional. If you don't supply a value, the default is <code>ENTERPRISE_EDITION</code>.</p>
    #[serde(rename = "Edition")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub edition: Option<IndexEdition>,
    /// <p>The name for the new index.</p>
    #[serde(rename = "Name")]
    pub name: String,
    /// <p>An AWS Identity and Access Management (IAM) role that gives Amazon Kendra permissions to access your Amazon CloudWatch logs and metrics. This is also the role used when you use the <code>BatchPutDocument</code> operation to index documents from an Amazon S3 bucket.</p>
    #[serde(rename = "RoleArn")]
    pub role_arn: String,
    /// <p>The identifier of the AWS KMS customer managed key (CMK) to use to encrypt data indexed by Amazon Kendra. Amazon Kendra doesn't support asymmetric CMKs.</p>
    #[serde(rename = "ServerSideEncryptionConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server_side_encryption_configuration: Option<ServerSideEncryptionConfiguration>,
    /// <p>A list of key-value pairs that identify the index. You can use the tags to identify and organize your resources and to control access to resources.</p>
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
    /// <p><p>The user context policy.</p> <dl> <dt>ATTRIBUTE<em>FILTER</dt> <dd> <p>All indexed content is searchable and displayable for all users. If there is an access control list, it is ignored. You can filter on user and group attributes. </p> </dd> <dt>USER</em>TOKEN</dt> <dd> <p>Enables SSO and token-based user access control. All documents with no access control and all documents accessible to the user will be searchable and displayable. </p> </dd> </dl></p>
    #[serde(rename = "UserContextPolicy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_context_policy: Option<UserContextPolicy>,
    /// <p>The user token configuration.</p>
    #[serde(rename = "UserTokenConfigurations")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_token_configurations: Option<Vec<UserTokenConfiguration>>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateIndexResponse {
    /// <p>The unique identifier of the index. Use this identifier when you query an index, set up a data source, or index a document.</p>
    #[serde(rename = "Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateThesaurusRequest {
    /// <p>A token that you provide to identify the request to create a thesaurus. Multiple calls to the <code>CreateThesaurus</code> operation with the same client token will create only one index. </p>
    #[serde(rename = "ClientToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
    /// <p>The description for the new thesaurus.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The unique identifier of the index for the new thesaurus. </p>
    #[serde(rename = "IndexId")]
    pub index_id: String,
    /// <p>The name for the new thesaurus.</p>
    #[serde(rename = "Name")]
    pub name: String,
    /// <p>An AWS Identity and Access Management (IAM) role that gives Amazon Kendra permissions to access thesaurus file specified in <code>SourceS3Path</code>. </p>
    #[serde(rename = "RoleArn")]
    pub role_arn: String,
    /// <p>The thesaurus file Amazon S3 source path. </p>
    #[serde(rename = "SourceS3Path")]
    pub source_s3_path: S3Path,
    /// <p>A list of key-value pairs that identify the thesaurus. You can use the tags to identify and organize your resources and to control access to resources. </p>
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateThesaurusResponse {
    /// <p>The unique identifier of the thesaurus. </p>
    #[serde(rename = "Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}

/// <p>Configuration information for a Amazon Kendra data source.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct DataSourceConfiguration {
    /// <p>Provides configuration information for connecting to a Confluence data source.</p>
    #[serde(rename = "ConfluenceConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub confluence_configuration: Option<ConfluenceConfiguration>,
    /// <p>Provides information necessary to create a data source connector for a database.</p>
    #[serde(rename = "DatabaseConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub database_configuration: Option<DatabaseConfiguration>,
    /// <p>Provides configuration for data sources that connect to Google Drive. </p>
    #[serde(rename = "GoogleDriveConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub google_drive_configuration: Option<GoogleDriveConfiguration>,
    /// <p>Provides configuration for data sources that connect to Microsoft OneDrive.</p>
    #[serde(rename = "OneDriveConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub one_drive_configuration: Option<OneDriveConfiguration>,
    /// <p>Provides information to create a data source connector for a document repository in an Amazon S3 bucket.</p>
    #[serde(rename = "S3Configuration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_configuration: Option<S3DataSourceConfiguration>,
    /// <p>Provides configuration information for data sources that connect to a Salesforce site.</p>
    #[serde(rename = "SalesforceConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub salesforce_configuration: Option<SalesforceConfiguration>,
    /// <p>Provides configuration for data sources that connect to ServiceNow instances.</p>
    #[serde(rename = "ServiceNowConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_now_configuration: Option<ServiceNowConfiguration>,
    /// <p>Provides information necessary to create a data source connector for a Microsoft SharePoint site.</p>
    #[serde(rename = "SharePointConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub share_point_configuration: Option<SharePointConfiguration>,
}

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct UnknownDataSourceStatus {
    name: String,
}

#[allow(non_camel_case_types)]
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
#[non_exhaustive]
pub enum DataSourceStatus {
    Active,
    Creating,
    Deleting,
    Failed,
    Updating,
    #[doc(hidden)]
    UnknownVariant(UnknownDataSourceStatus),
}

impl Default for DataSourceStatus {
    fn default() -> Self {
        "".into()
    }
}

impl fmt::Display for DataSourceStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.into())
    }
}

impl rusoto_core::param::ToParam for DataSourceStatus {
    fn to_param(&self) -> String {
        self.to_string()
    }
}

impl Into<String> for DataSourceStatus {
    fn into(self) -> String {
        match self {
            DataSourceStatus::Active => "ACTIVE".to_string(),
            DataSourceStatus::Creating => "CREATING".to_string(),
            DataSourceStatus::Deleting => "DELETING".to_string(),
            DataSourceStatus::Failed => "FAILED".to_string(),
            DataSourceStatus::Updating => "UPDATING".to_string(),
            DataSourceStatus::UnknownVariant(UnknownDataSourceStatus { name: original }) => {
                original
            }
        }
    }
}

impl<'a> Into<&'a str> for &'a DataSourceStatus {
    fn into(self) -> &'a str {
        match self {
            DataSourceStatus::Active => &"ACTIVE",
            DataSourceStatus::Creating => &"CREATING",
            DataSourceStatus::Deleting => &"DELETING",
            DataSourceStatus::Failed => &"FAILED",
            DataSourceStatus::Updating => &"UPDATING",
            DataSourceStatus::UnknownVariant(UnknownDataSourceStatus { name: original }) => {
                original
            }
        }
    }
}

impl From<&str> for DataSourceStatus {
    fn from(name: &str) -> Self {
        match name {
            "ACTIVE" => DataSourceStatus::Active,
            "CREATING" => DataSourceStatus::Creating,
            "DELETING" => DataSourceStatus::Deleting,
            "FAILED" => DataSourceStatus::Failed,
            "UPDATING" => DataSourceStatus::Updating,
            _ => DataSourceStatus::UnknownVariant(UnknownDataSourceStatus {
                name: name.to_owned(),
            }),
        }
    }
}

impl From<String> for DataSourceStatus {
    fn from(name: String) -> Self {
        match &*name {
            "ACTIVE" => DataSourceStatus::Active,
            "CREATING" => DataSourceStatus::Creating,
            "DELETING" => DataSourceStatus::Deleting,
            "FAILED" => DataSourceStatus::Failed,
            "UPDATING" => DataSourceStatus::Updating,
            _ => DataSourceStatus::UnknownVariant(UnknownDataSourceStatus { name }),
        }
    }
}

impl ::std::str::FromStr for DataSourceStatus {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(s.into())
    }
}

#[cfg(any(test, feature = "serialize_structs"))]
impl Serialize for DataSourceStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.into())
    }
}

impl<'de> Deserialize<'de> for DataSourceStatus {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Ok(String::deserialize(deserializer)?.into())
    }
}

/// <p>Summary information for a Amazon Kendra data source. Returned in a call to .</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
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
    pub status: Option<DataSourceStatus>,
    /// <p>The type of the data source.</p>
    #[serde(rename = "Type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<DataSourceType>,
    /// <p>The UNIX datetime that the data source was lasted updated. </p>
    #[serde(rename = "UpdatedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<f64>,
}

/// <p>Provides information about a synchronization job.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
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
    pub error_code: Option<ErrorCode>,
    /// <p>If the <code>Status</code> field is set to <code>ERROR</code>, the <code>ErrorMessage</code> field contains a description of the error that caused the synchronization to fail.</p>
    #[serde(rename = "ErrorMessage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
    /// <p>A unique identifier for the synchronization job.</p>
    #[serde(rename = "ExecutionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub execution_id: Option<String>,
    /// <p>Maps a batch delete document request to a specific data source sync job. This is optional and should only be supplied when documents are deleted by a data source connector.</p>
    #[serde(rename = "Metrics")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metrics: Option<DataSourceSyncJobMetrics>,
    /// <p>The UNIX datetime that the synchronization job was started.</p>
    #[serde(rename = "StartTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<f64>,
    /// <p>The execution status of the synchronization job. When the <code>Status</code> field is set to <code>SUCCEEDED</code>, the synchronization job is done. If the status code is set to <code>FAILED</code>, the <code>ErrorCode</code> and <code>ErrorMessage</code> fields give you the reason for the failure.</p>
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<DataSourceSyncJobStatus>,
}

/// <p>Maps a particular data source sync job to a particular data source.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DataSourceSyncJobMetricTarget {
    /// <p>The ID of the data source that is running the sync job.</p>
    #[serde(rename = "DataSourceId")]
    pub data_source_id: String,
    /// <p>The ID of the sync job that is running on the data source.</p>
    #[serde(rename = "DataSourceSyncJobId")]
    pub data_source_sync_job_id: String,
}

/// <p>Maps a batch delete document request to a specific data source sync job. This is optional and should only be supplied when documents are deleted by a data source connector.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DataSourceSyncJobMetrics {
    /// <p>The number of documents added from the data source up to now in the data source sync.</p>
    #[serde(rename = "DocumentsAdded")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub documents_added: Option<String>,
    /// <p>The number of documents deleted from the data source up to now in the data source sync run.</p>
    #[serde(rename = "DocumentsDeleted")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub documents_deleted: Option<String>,
    /// <p>The number of documents that failed to sync from the data source up to now in the data source sync run.</p>
    #[serde(rename = "DocumentsFailed")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub documents_failed: Option<String>,
    /// <p>The number of documents modified in the data source up to now in the data source sync run.</p>
    #[serde(rename = "DocumentsModified")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub documents_modified: Option<String>,
    /// <p>The current number of documents crawled by the current sync job in the data source.</p>
    #[serde(rename = "DocumentsScanned")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub documents_scanned: Option<String>,
}

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct UnknownDataSourceSyncJobStatus {
    name: String,
}

#[allow(non_camel_case_types)]
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
#[non_exhaustive]
pub enum DataSourceSyncJobStatus {
    Aborted,
    Failed,
    Incomplete,
    Stopping,
    Succeeded,
    Syncing,
    SyncingIndexing,
    #[doc(hidden)]
    UnknownVariant(UnknownDataSourceSyncJobStatus),
}

impl Default for DataSourceSyncJobStatus {
    fn default() -> Self {
        "".into()
    }
}

impl fmt::Display for DataSourceSyncJobStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.into())
    }
}

impl rusoto_core::param::ToParam for DataSourceSyncJobStatus {
    fn to_param(&self) -> String {
        self.to_string()
    }
}

impl Into<String> for DataSourceSyncJobStatus {
    fn into(self) -> String {
        match self {
            DataSourceSyncJobStatus::Aborted => "ABORTED".to_string(),
            DataSourceSyncJobStatus::Failed => "FAILED".to_string(),
            DataSourceSyncJobStatus::Incomplete => "INCOMPLETE".to_string(),
            DataSourceSyncJobStatus::Stopping => "STOPPING".to_string(),
            DataSourceSyncJobStatus::Succeeded => "SUCCEEDED".to_string(),
            DataSourceSyncJobStatus::Syncing => "SYNCING".to_string(),
            DataSourceSyncJobStatus::SyncingIndexing => "SYNCING_INDEXING".to_string(),
            DataSourceSyncJobStatus::UnknownVariant(UnknownDataSourceSyncJobStatus {
                name: original,
            }) => original,
        }
    }
}

impl<'a> Into<&'a str> for &'a DataSourceSyncJobStatus {
    fn into(self) -> &'a str {
        match self {
            DataSourceSyncJobStatus::Aborted => &"ABORTED",
            DataSourceSyncJobStatus::Failed => &"FAILED",
            DataSourceSyncJobStatus::Incomplete => &"INCOMPLETE",
            DataSourceSyncJobStatus::Stopping => &"STOPPING",
            DataSourceSyncJobStatus::Succeeded => &"SUCCEEDED",
            DataSourceSyncJobStatus::Syncing => &"SYNCING",
            DataSourceSyncJobStatus::SyncingIndexing => &"SYNCING_INDEXING",
            DataSourceSyncJobStatus::UnknownVariant(UnknownDataSourceSyncJobStatus {
                name: original,
            }) => original,
        }
    }
}

impl From<&str> for DataSourceSyncJobStatus {
    fn from(name: &str) -> Self {
        match name {
            "ABORTED" => DataSourceSyncJobStatus::Aborted,
            "FAILED" => DataSourceSyncJobStatus::Failed,
            "INCOMPLETE" => DataSourceSyncJobStatus::Incomplete,
            "STOPPING" => DataSourceSyncJobStatus::Stopping,
            "SUCCEEDED" => DataSourceSyncJobStatus::Succeeded,
            "SYNCING" => DataSourceSyncJobStatus::Syncing,
            "SYNCING_INDEXING" => DataSourceSyncJobStatus::SyncingIndexing,
            _ => DataSourceSyncJobStatus::UnknownVariant(UnknownDataSourceSyncJobStatus {
                name: name.to_owned(),
            }),
        }
    }
}

impl From<String> for DataSourceSyncJobStatus {
    fn from(name: String) -> Self {
        match &*name {
            "ABORTED" => DataSourceSyncJobStatus::Aborted,
            "FAILED" => DataSourceSyncJobStatus::Failed,
            "INCOMPLETE" => DataSourceSyncJobStatus::Incomplete,
            "STOPPING" => DataSourceSyncJobStatus::Stopping,
            "SUCCEEDED" => DataSourceSyncJobStatus::Succeeded,
            "SYNCING" => DataSourceSyncJobStatus::Syncing,
            "SYNCING_INDEXING" => DataSourceSyncJobStatus::SyncingIndexing,
            _ => DataSourceSyncJobStatus::UnknownVariant(UnknownDataSourceSyncJobStatus { name }),
        }
    }
}

impl ::std::str::FromStr for DataSourceSyncJobStatus {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(s.into())
    }
}

impl Serialize for DataSourceSyncJobStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.into())
    }
}

impl<'de> Deserialize<'de> for DataSourceSyncJobStatus {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Ok(String::deserialize(deserializer)?.into())
    }
}

/// <p>Maps a column or attribute in the data source to an index field. You must first create the fields in the index using the <a>UpdateIndex</a> operation.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
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

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct UnknownDataSourceType {
    name: String,
}

#[allow(non_camel_case_types)]
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
#[non_exhaustive]
pub enum DataSourceType {
    Confluence,
    Custom,
    Database,
    Googledrive,
    Onedrive,
    S3,
    Salesforce,
    Servicenow,
    Sharepoint,
    #[doc(hidden)]
    UnknownVariant(UnknownDataSourceType),
}

impl Default for DataSourceType {
    fn default() -> Self {
        "".into()
    }
}

impl fmt::Display for DataSourceType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.into())
    }
}

impl rusoto_core::param::ToParam for DataSourceType {
    fn to_param(&self) -> String {
        self.to_string()
    }
}

impl Into<String> for DataSourceType {
    fn into(self) -> String {
        match self {
            DataSourceType::Confluence => "CONFLUENCE".to_string(),
            DataSourceType::Custom => "CUSTOM".to_string(),
            DataSourceType::Database => "DATABASE".to_string(),
            DataSourceType::Googledrive => "GOOGLEDRIVE".to_string(),
            DataSourceType::Onedrive => "ONEDRIVE".to_string(),
            DataSourceType::S3 => "S3".to_string(),
            DataSourceType::Salesforce => "SALESFORCE".to_string(),
            DataSourceType::Servicenow => "SERVICENOW".to_string(),
            DataSourceType::Sharepoint => "SHAREPOINT".to_string(),
            DataSourceType::UnknownVariant(UnknownDataSourceType { name: original }) => original,
        }
    }
}

impl<'a> Into<&'a str> for &'a DataSourceType {
    fn into(self) -> &'a str {
        match self {
            DataSourceType::Confluence => &"CONFLUENCE",
            DataSourceType::Custom => &"CUSTOM",
            DataSourceType::Database => &"DATABASE",
            DataSourceType::Googledrive => &"GOOGLEDRIVE",
            DataSourceType::Onedrive => &"ONEDRIVE",
            DataSourceType::S3 => &"S3",
            DataSourceType::Salesforce => &"SALESFORCE",
            DataSourceType::Servicenow => &"SERVICENOW",
            DataSourceType::Sharepoint => &"SHAREPOINT",
            DataSourceType::UnknownVariant(UnknownDataSourceType { name: original }) => original,
        }
    }
}

impl From<&str> for DataSourceType {
    fn from(name: &str) -> Self {
        match name {
            "CONFLUENCE" => DataSourceType::Confluence,
            "CUSTOM" => DataSourceType::Custom,
            "DATABASE" => DataSourceType::Database,
            "GOOGLEDRIVE" => DataSourceType::Googledrive,
            "ONEDRIVE" => DataSourceType::Onedrive,
            "S3" => DataSourceType::S3,
            "SALESFORCE" => DataSourceType::Salesforce,
            "SERVICENOW" => DataSourceType::Servicenow,
            "SHAREPOINT" => DataSourceType::Sharepoint,
            _ => DataSourceType::UnknownVariant(UnknownDataSourceType {
                name: name.to_owned(),
            }),
        }
    }
}

impl From<String> for DataSourceType {
    fn from(name: String) -> Self {
        match &*name {
            "CONFLUENCE" => DataSourceType::Confluence,
            "CUSTOM" => DataSourceType::Custom,
            "DATABASE" => DataSourceType::Database,
            "GOOGLEDRIVE" => DataSourceType::Googledrive,
            "ONEDRIVE" => DataSourceType::Onedrive,
            "S3" => DataSourceType::S3,
            "SALESFORCE" => DataSourceType::Salesforce,
            "SERVICENOW" => DataSourceType::Servicenow,
            "SHAREPOINT" => DataSourceType::Sharepoint,
            _ => DataSourceType::UnknownVariant(UnknownDataSourceType { name }),
        }
    }
}

impl ::std::str::FromStr for DataSourceType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(s.into())
    }
}

impl Serialize for DataSourceType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.into())
    }
}

impl<'de> Deserialize<'de> for DataSourceType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Ok(String::deserialize(deserializer)?.into())
    }
}

/// <p>Provides information for connecting to an Amazon VPC.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct DataSourceVpcConfiguration {
    /// <p>A list of identifiers of security groups within your Amazon VPC. The security groups should enable Amazon Kendra to connect to the data source.</p>
    #[serde(rename = "SecurityGroupIds")]
    pub security_group_ids: Vec<String>,
    /// <p>A list of identifiers for subnets within your Amazon VPC. The subnets should be able to connect to each other in the VPC, and they should have outgoing access to the Internet through a NAT device.</p>
    #[serde(rename = "SubnetIds")]
    pub subnet_ids: Vec<String>,
}

/// <p>Provides the information necessary to connect a database to an index. </p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
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
    pub database_engine_type: DatabaseEngineType,
    /// <p>Provides information about how Amazon Kendra uses quote marks around SQL identifiers when querying a database data source.</p>
    #[serde(rename = "SqlConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sql_configuration: Option<SqlConfiguration>,
    #[serde(rename = "VpcConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_configuration: Option<DataSourceVpcConfiguration>,
}

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct UnknownDatabaseEngineType {
    name: String,
}

#[allow(non_camel_case_types)]
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
#[non_exhaustive]
pub enum DatabaseEngineType {
    RdsAuroraMysql,
    RdsAuroraPostgresql,
    RdsMysql,
    RdsPostgresql,
    #[doc(hidden)]
    UnknownVariant(UnknownDatabaseEngineType),
}

impl Default for DatabaseEngineType {
    fn default() -> Self {
        "".into()
    }
}

impl fmt::Display for DatabaseEngineType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.into())
    }
}

impl rusoto_core::param::ToParam for DatabaseEngineType {
    fn to_param(&self) -> String {
        self.to_string()
    }
}

impl Into<String> for DatabaseEngineType {
    fn into(self) -> String {
        match self {
            DatabaseEngineType::RdsAuroraMysql => "RDS_AURORA_MYSQL".to_string(),
            DatabaseEngineType::RdsAuroraPostgresql => "RDS_AURORA_POSTGRESQL".to_string(),
            DatabaseEngineType::RdsMysql => "RDS_MYSQL".to_string(),
            DatabaseEngineType::RdsPostgresql => "RDS_POSTGRESQL".to_string(),
            DatabaseEngineType::UnknownVariant(UnknownDatabaseEngineType { name: original }) => {
                original
            }
        }
    }
}

impl<'a> Into<&'a str> for &'a DatabaseEngineType {
    fn into(self) -> &'a str {
        match self {
            DatabaseEngineType::RdsAuroraMysql => &"RDS_AURORA_MYSQL",
            DatabaseEngineType::RdsAuroraPostgresql => &"RDS_AURORA_POSTGRESQL",
            DatabaseEngineType::RdsMysql => &"RDS_MYSQL",
            DatabaseEngineType::RdsPostgresql => &"RDS_POSTGRESQL",
            DatabaseEngineType::UnknownVariant(UnknownDatabaseEngineType { name: original }) => {
                original
            }
        }
    }
}

impl From<&str> for DatabaseEngineType {
    fn from(name: &str) -> Self {
        match name {
            "RDS_AURORA_MYSQL" => DatabaseEngineType::RdsAuroraMysql,
            "RDS_AURORA_POSTGRESQL" => DatabaseEngineType::RdsAuroraPostgresql,
            "RDS_MYSQL" => DatabaseEngineType::RdsMysql,
            "RDS_POSTGRESQL" => DatabaseEngineType::RdsPostgresql,
            _ => DatabaseEngineType::UnknownVariant(UnknownDatabaseEngineType {
                name: name.to_owned(),
            }),
        }
    }
}

impl From<String> for DatabaseEngineType {
    fn from(name: String) -> Self {
        match &*name {
            "RDS_AURORA_MYSQL" => DatabaseEngineType::RdsAuroraMysql,
            "RDS_AURORA_POSTGRESQL" => DatabaseEngineType::RdsAuroraPostgresql,
            "RDS_MYSQL" => DatabaseEngineType::RdsMysql,
            "RDS_POSTGRESQL" => DatabaseEngineType::RdsPostgresql,
            _ => DatabaseEngineType::UnknownVariant(UnknownDatabaseEngineType { name }),
        }
    }
}

impl ::std::str::FromStr for DatabaseEngineType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(s.into())
    }
}

impl Serialize for DatabaseEngineType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.into())
    }
}

impl<'de> Deserialize<'de> for DatabaseEngineType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Ok(String::deserialize(deserializer)?.into())
    }
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteDataSourceRequest {
    /// <p>The unique identifier of the data source to delete.</p>
    #[serde(rename = "Id")]
    pub id: String,
    /// <p>The unique identifier of the index associated with the data source.</p>
    #[serde(rename = "IndexId")]
    pub index_id: String,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteFaqRequest {
    /// <p>The identifier of the FAQ to remove.</p>
    #[serde(rename = "Id")]
    pub id: String,
    /// <p>The index to remove the FAQ from.</p>
    #[serde(rename = "IndexId")]
    pub index_id: String,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteIndexRequest {
    /// <p>The identifier of the index to delete.</p>
    #[serde(rename = "Id")]
    pub id: String,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteThesaurusRequest {
    /// <p>The identifier of the thesaurus to delete.</p>
    #[serde(rename = "Id")]
    pub id: String,
    /// <p>The identifier of the index associated with the thesaurus to delete.</p>
    #[serde(rename = "IndexId")]
    pub index_id: String,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeDataSourceRequest {
    /// <p>The unique identifier of the data source to describe.</p>
    #[serde(rename = "Id")]
    pub id: String,
    /// <p>The identifier of the index that contains the data source.</p>
    #[serde(rename = "IndexId")]
    pub index_id: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
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
    pub status: Option<DataSourceStatus>,
    /// <p>The type of the data source.</p>
    #[serde(rename = "Type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<DataSourceType>,
    /// <p>The Unix timestamp of when the data source was last updated.</p>
    #[serde(rename = "UpdatedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<f64>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeFaqRequest {
    /// <p>The unique identifier of the FAQ.</p>
    #[serde(rename = "Id")]
    pub id: String,
    /// <p>The identifier of the index that contains the FAQ.</p>
    #[serde(rename = "IndexId")]
    pub index_id: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
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
    /// <p>The file format used by the input files for the FAQ.</p>
    #[serde(rename = "FileFormat")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_format: Option<FaqFileFormat>,
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
    pub status: Option<FaqStatus>,
    /// <p>The date and time that the FAQ was last updated.</p>
    #[serde(rename = "UpdatedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<f64>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeIndexRequest {
    /// <p>The name of the index to describe.</p>
    #[serde(rename = "Id")]
    pub id: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeIndexResponse {
    /// <p>For enterprise edtion indexes, you can choose to use additional capacity to meet the needs of your application. This contains the capacity units used for the index. A 0 for the query capacity or the storage capacity indicates that the index is using the default capacity for the index.</p>
    #[serde(rename = "CapacityUnits")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capacity_units: Option<CapacityUnitsConfiguration>,
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
    /// <p>The Amazon Kendra edition used for the index. You decide the edition when you create the index.</p>
    #[serde(rename = "Edition")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub edition: Option<IndexEdition>,
    /// <p>When th e<code>Status</code> field value is <code>FAILED</code>, the <code>ErrorMessage</code> field contains a message that explains why.</p>
    #[serde(rename = "ErrorMessage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
    /// <p>The name of the index.</p>
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
    pub status: Option<IndexStatus>,
    /// <p>The Unix datetime that the index was last updated.</p>
    #[serde(rename = "UpdatedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<f64>,
    /// <p>The user context policy for the Amazon Kendra index.</p>
    #[serde(rename = "UserContextPolicy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_context_policy: Option<UserContextPolicy>,
    /// <p>The user token configuration for the Amazon Kendra index.</p>
    #[serde(rename = "UserTokenConfigurations")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_token_configurations: Option<Vec<UserTokenConfiguration>>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeThesaurusRequest {
    /// <p>The identifier of the thesaurus to describe.</p>
    #[serde(rename = "Id")]
    pub id: String,
    /// <p>The identifier of the index associated with the thesaurus to describe.</p>
    #[serde(rename = "IndexId")]
    pub index_id: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeThesaurusResponse {
    /// <p>The Unix datetime that the thesaurus was created.</p>
    #[serde(rename = "CreatedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<f64>,
    /// <p>The thesaurus description.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>When the <code>Status</code> field value is <code>FAILED</code>, the <code>ErrorMessage</code> field provides more information. </p>
    #[serde(rename = "ErrorMessage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
    /// <p>The size of the thesaurus file in bytes.</p>
    #[serde(rename = "FileSizeBytes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_size_bytes: Option<i64>,
    /// <p>The identifier of the thesaurus.</p>
    #[serde(rename = "Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// <p>The identifier of the index associated with the thesaurus to describe.</p>
    #[serde(rename = "IndexId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub index_id: Option<String>,
    /// <p>The thesaurus name.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>An AWS Identity and Access Management (IAM) role that gives Amazon Kendra permissions to access thesaurus file specified in <code>SourceS3Path</code>. </p>
    #[serde(rename = "RoleArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_arn: Option<String>,
    #[serde(rename = "SourceS3Path")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_s3_path: Option<S3Path>,
    /// <p>The current status of the thesaurus. When the value is <code>ACTIVE</code>, queries are able to use the thesaurus. If the <code>Status</code> field value is <code>FAILED</code>, the <code>ErrorMessage</code> field provides more information. </p> <p>If the status is <code>ACTIVE_BUT_UPDATE_FAILED</code>, it means that Amazon Kendra could not ingest the new thesaurus file. The old thesaurus file is still active. </p>
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<ThesaurusStatus>,
    /// <p>The number of synonym rules in the thesaurus file.</p>
    #[serde(rename = "SynonymRuleCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub synonym_rule_count: Option<i64>,
    /// <p>The number of unique terms in the thesaurus file. For example, the synonyms <code>a,b,c</code> and <code>a=&gt;d</code>, the term count would be 4. </p>
    #[serde(rename = "TermCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub term_count: Option<i64>,
    /// <p>The Unix datetime that the thesaurus was last updated.</p>
    #[serde(rename = "UpdatedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<f64>,
}

/// <p>A document in an index.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
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
    /// <p>The contents of the document. </p> <p>Documents passed to the <code>Blob</code> parameter must be base64 encoded. Your code might not need to encode the document file bytes if you're using an AWS SDK to call Amazon Kendra operations. If you are calling the Amazon Kendra endpoint directly using REST, you must base64 encode the contents before sending.</p>
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
    pub content_type: Option<ContentType>,
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
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct DocumentAttribute {
    /// <p>The identifier for the attribute.</p>
    #[serde(rename = "Key")]
    pub key: String,
    /// <p>The value of the attribute.</p>
    #[serde(rename = "Value")]
    pub value: DocumentAttributeValue,
}

/// <p>The value of a custom document attribute. You can only provide one value for a custom attribute.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct DocumentAttributeValue {
    /// <p>A date expressed as an ISO 8601 string.</p>
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
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
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

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct UnknownDocumentAttributeValueType {
    name: String,
}

#[allow(non_camel_case_types)]
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
#[non_exhaustive]
pub enum DocumentAttributeValueType {
    DateValue,
    LongValue,
    StringListValue,
    StringValue,
    #[doc(hidden)]
    UnknownVariant(UnknownDocumentAttributeValueType),
}

impl Default for DocumentAttributeValueType {
    fn default() -> Self {
        "".into()
    }
}

impl fmt::Display for DocumentAttributeValueType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.into())
    }
}

impl rusoto_core::param::ToParam for DocumentAttributeValueType {
    fn to_param(&self) -> String {
        self.to_string()
    }
}

impl Into<String> for DocumentAttributeValueType {
    fn into(self) -> String {
        match self {
            DocumentAttributeValueType::DateValue => "DATE_VALUE".to_string(),
            DocumentAttributeValueType::LongValue => "LONG_VALUE".to_string(),
            DocumentAttributeValueType::StringListValue => "STRING_LIST_VALUE".to_string(),
            DocumentAttributeValueType::StringValue => "STRING_VALUE".to_string(),
            DocumentAttributeValueType::UnknownVariant(UnknownDocumentAttributeValueType {
                name: original,
            }) => original,
        }
    }
}

impl<'a> Into<&'a str> for &'a DocumentAttributeValueType {
    fn into(self) -> &'a str {
        match self {
            DocumentAttributeValueType::DateValue => &"DATE_VALUE",
            DocumentAttributeValueType::LongValue => &"LONG_VALUE",
            DocumentAttributeValueType::StringListValue => &"STRING_LIST_VALUE",
            DocumentAttributeValueType::StringValue => &"STRING_VALUE",
            DocumentAttributeValueType::UnknownVariant(UnknownDocumentAttributeValueType {
                name: original,
            }) => original,
        }
    }
}

impl From<&str> for DocumentAttributeValueType {
    fn from(name: &str) -> Self {
        match name {
            "DATE_VALUE" => DocumentAttributeValueType::DateValue,
            "LONG_VALUE" => DocumentAttributeValueType::LongValue,
            "STRING_LIST_VALUE" => DocumentAttributeValueType::StringListValue,
            "STRING_VALUE" => DocumentAttributeValueType::StringValue,
            _ => DocumentAttributeValueType::UnknownVariant(UnknownDocumentAttributeValueType {
                name: name.to_owned(),
            }),
        }
    }
}

impl From<String> for DocumentAttributeValueType {
    fn from(name: String) -> Self {
        match &*name {
            "DATE_VALUE" => DocumentAttributeValueType::DateValue,
            "LONG_VALUE" => DocumentAttributeValueType::LongValue,
            "STRING_LIST_VALUE" => DocumentAttributeValueType::StringListValue,
            "STRING_VALUE" => DocumentAttributeValueType::StringValue,
            _ => DocumentAttributeValueType::UnknownVariant(UnknownDocumentAttributeValueType {
                name,
            }),
        }
    }
}

impl ::std::str::FromStr for DocumentAttributeValueType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(s.into())
    }
}

impl Serialize for DocumentAttributeValueType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.into())
    }
}

impl<'de> Deserialize<'de> for DocumentAttributeValueType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Ok(String::deserialize(deserializer)?.into())
    }
}

/// <p>Specifies the properties of a custom index field.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
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
    pub type_: DocumentAttributeValueType,
}

/// <p>Document metadata files that contain information such as the document access control information, source URI, document author, and custom attributes. Each metadata file contains metadata about a single document.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct DocumentsMetadataConfiguration {
    /// <p>A prefix used to filter metadata configuration files in the AWS S3 bucket. The S3 bucket might contain multiple metadata files. Use <code>S3Prefix</code> to include only the desired metadata files.</p>
    #[serde(rename = "S3Prefix")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_prefix: Option<String>,
}

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct UnknownErrorCode {
    name: String,
}

#[allow(non_camel_case_types)]
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
#[non_exhaustive]
pub enum ErrorCode {
    InternalError,
    InvalidRequest,
    #[doc(hidden)]
    UnknownVariant(UnknownErrorCode),
}

impl Default for ErrorCode {
    fn default() -> Self {
        "".into()
    }
}

impl fmt::Display for ErrorCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.into())
    }
}

impl rusoto_core::param::ToParam for ErrorCode {
    fn to_param(&self) -> String {
        self.to_string()
    }
}

impl Into<String> for ErrorCode {
    fn into(self) -> String {
        match self {
            ErrorCode::InternalError => "InternalError".to_string(),
            ErrorCode::InvalidRequest => "InvalidRequest".to_string(),
            ErrorCode::UnknownVariant(UnknownErrorCode { name: original }) => original,
        }
    }
}

impl<'a> Into<&'a str> for &'a ErrorCode {
    fn into(self) -> &'a str {
        match self {
            ErrorCode::InternalError => &"InternalError",
            ErrorCode::InvalidRequest => &"InvalidRequest",
            ErrorCode::UnknownVariant(UnknownErrorCode { name: original }) => original,
        }
    }
}

impl From<&str> for ErrorCode {
    fn from(name: &str) -> Self {
        match name {
            "InternalError" => ErrorCode::InternalError,
            "InvalidRequest" => ErrorCode::InvalidRequest,
            _ => ErrorCode::UnknownVariant(UnknownErrorCode {
                name: name.to_owned(),
            }),
        }
    }
}

impl From<String> for ErrorCode {
    fn from(name: String) -> Self {
        match &*name {
            "InternalError" => ErrorCode::InternalError,
            "InvalidRequest" => ErrorCode::InvalidRequest,
            _ => ErrorCode::UnknownVariant(UnknownErrorCode { name }),
        }
    }
}

impl ::std::str::FromStr for ErrorCode {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(s.into())
    }
}

#[cfg(any(test, feature = "serialize_structs"))]
impl Serialize for ErrorCode {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.into())
    }
}

impl<'de> Deserialize<'de> for ErrorCode {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Ok(String::deserialize(deserializer)?.into())
    }
}

/// <p>Information about a document attribute</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct Facet {
    /// <p>The unique key for the document attribute.</p>
    #[serde(rename = "DocumentAttributeKey")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document_attribute_key: Option<String>,
}

/// <p>The facet values for the documents in the response.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
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
    /// <p>The data type of the facet value. This is the same as the type defined for the index field when it was created.</p>
    #[serde(rename = "DocumentAttributeValueType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document_attribute_value_type: Option<DocumentAttributeValueType>,
}

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct UnknownFaqFileFormat {
    name: String,
}

#[allow(non_camel_case_types)]
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
#[non_exhaustive]
pub enum FaqFileFormat {
    Csv,
    CsvWithHeader,
    Json,
    #[doc(hidden)]
    UnknownVariant(UnknownFaqFileFormat),
}

impl Default for FaqFileFormat {
    fn default() -> Self {
        "".into()
    }
}

impl fmt::Display for FaqFileFormat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.into())
    }
}

impl rusoto_core::param::ToParam for FaqFileFormat {
    fn to_param(&self) -> String {
        self.to_string()
    }
}

impl Into<String> for FaqFileFormat {
    fn into(self) -> String {
        match self {
            FaqFileFormat::Csv => "CSV".to_string(),
            FaqFileFormat::CsvWithHeader => "CSV_WITH_HEADER".to_string(),
            FaqFileFormat::Json => "JSON".to_string(),
            FaqFileFormat::UnknownVariant(UnknownFaqFileFormat { name: original }) => original,
        }
    }
}

impl<'a> Into<&'a str> for &'a FaqFileFormat {
    fn into(self) -> &'a str {
        match self {
            FaqFileFormat::Csv => &"CSV",
            FaqFileFormat::CsvWithHeader => &"CSV_WITH_HEADER",
            FaqFileFormat::Json => &"JSON",
            FaqFileFormat::UnknownVariant(UnknownFaqFileFormat { name: original }) => original,
        }
    }
}

impl From<&str> for FaqFileFormat {
    fn from(name: &str) -> Self {
        match name {
            "CSV" => FaqFileFormat::Csv,
            "CSV_WITH_HEADER" => FaqFileFormat::CsvWithHeader,
            "JSON" => FaqFileFormat::Json,
            _ => FaqFileFormat::UnknownVariant(UnknownFaqFileFormat {
                name: name.to_owned(),
            }),
        }
    }
}

impl From<String> for FaqFileFormat {
    fn from(name: String) -> Self {
        match &*name {
            "CSV" => FaqFileFormat::Csv,
            "CSV_WITH_HEADER" => FaqFileFormat::CsvWithHeader,
            "JSON" => FaqFileFormat::Json,
            _ => FaqFileFormat::UnknownVariant(UnknownFaqFileFormat { name }),
        }
    }
}

impl ::std::str::FromStr for FaqFileFormat {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(s.into())
    }
}

impl Serialize for FaqFileFormat {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.into())
    }
}

impl<'de> Deserialize<'de> for FaqFileFormat {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Ok(String::deserialize(deserializer)?.into())
    }
}

/// <p>Provides statistical information about the FAQ questions and answers contained in an index.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct FaqStatistics {
    /// <p>The total number of FAQ questions and answers contained in the index.</p>
    #[serde(rename = "IndexedQuestionAnswersCount")]
    pub indexed_question_answers_count: i64,
}

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct UnknownFaqStatus {
    name: String,
}

#[allow(non_camel_case_types)]
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
#[non_exhaustive]
pub enum FaqStatus {
    Active,
    Creating,
    Deleting,
    Failed,
    Updating,
    #[doc(hidden)]
    UnknownVariant(UnknownFaqStatus),
}

impl Default for FaqStatus {
    fn default() -> Self {
        "".into()
    }
}

impl fmt::Display for FaqStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.into())
    }
}

impl rusoto_core::param::ToParam for FaqStatus {
    fn to_param(&self) -> String {
        self.to_string()
    }
}

impl Into<String> for FaqStatus {
    fn into(self) -> String {
        match self {
            FaqStatus::Active => "ACTIVE".to_string(),
            FaqStatus::Creating => "CREATING".to_string(),
            FaqStatus::Deleting => "DELETING".to_string(),
            FaqStatus::Failed => "FAILED".to_string(),
            FaqStatus::Updating => "UPDATING".to_string(),
            FaqStatus::UnknownVariant(UnknownFaqStatus { name: original }) => original,
        }
    }
}

impl<'a> Into<&'a str> for &'a FaqStatus {
    fn into(self) -> &'a str {
        match self {
            FaqStatus::Active => &"ACTIVE",
            FaqStatus::Creating => &"CREATING",
            FaqStatus::Deleting => &"DELETING",
            FaqStatus::Failed => &"FAILED",
            FaqStatus::Updating => &"UPDATING",
            FaqStatus::UnknownVariant(UnknownFaqStatus { name: original }) => original,
        }
    }
}

impl From<&str> for FaqStatus {
    fn from(name: &str) -> Self {
        match name {
            "ACTIVE" => FaqStatus::Active,
            "CREATING" => FaqStatus::Creating,
            "DELETING" => FaqStatus::Deleting,
            "FAILED" => FaqStatus::Failed,
            "UPDATING" => FaqStatus::Updating,
            _ => FaqStatus::UnknownVariant(UnknownFaqStatus {
                name: name.to_owned(),
            }),
        }
    }
}

impl From<String> for FaqStatus {
    fn from(name: String) -> Self {
        match &*name {
            "ACTIVE" => FaqStatus::Active,
            "CREATING" => FaqStatus::Creating,
            "DELETING" => FaqStatus::Deleting,
            "FAILED" => FaqStatus::Failed,
            "UPDATING" => FaqStatus::Updating,
            _ => FaqStatus::UnknownVariant(UnknownFaqStatus { name }),
        }
    }
}

impl ::std::str::FromStr for FaqStatus {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(s.into())
    }
}

#[cfg(any(test, feature = "serialize_structs"))]
impl Serialize for FaqStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.into())
    }
}

impl<'de> Deserialize<'de> for FaqStatus {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Ok(String::deserialize(deserializer)?.into())
    }
}

/// <p>Provides information about a frequently asked questions and answer contained in an index.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct FaqSummary {
    /// <p>The UNIX datetime that the FAQ was added to the index.</p>
    #[serde(rename = "CreatedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<f64>,
    /// <p>The file type used to create the FAQ. </p>
    #[serde(rename = "FileFormat")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_format: Option<FaqFileFormat>,
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
    pub status: Option<FaqStatus>,
    /// <p>The UNIX datetime that the FAQ was last updated.</p>
    #[serde(rename = "UpdatedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<f64>,
}

/// <p>Provides configuration information for data sources that connect to Google Drive.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct GoogleDriveConfiguration {
    /// <p>A list of MIME types to exclude from the index. All documents matching the specified MIME type are excluded. </p> <p>For a list of MIME types, see <a href="https://docs.aws.amazon.com/kendra/latest/dg/data-source-google-drive.html">Using a Google Workspace Drive data source</a>.</p>
    #[serde(rename = "ExcludeMimeTypes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exclude_mime_types: Option<Vec<String>>,
    /// <p>A list of identifiers or shared drives to exclude from the index. All files and folders stored on the shared drive are excluded.</p>
    #[serde(rename = "ExcludeSharedDrives")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exclude_shared_drives: Option<Vec<String>>,
    /// <p>A list of email addresses of the users. Documents owned by these users are excluded from the index. Documents shared with excluded users are indexed unless they are excluded in another way.</p>
    #[serde(rename = "ExcludeUserAccounts")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exclude_user_accounts: Option<Vec<String>>,
    /// <p>A list of regular expression patterns that apply to the path on Google Drive. Items that match the pattern are excluded from the index from both shared drives and users' My Drives. Items that don't match the pattern are included in the index. If an item matches both an exclusion pattern and an inclusion pattern, it is excluded from the index.</p>
    #[serde(rename = "ExclusionPatterns")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exclusion_patterns: Option<Vec<String>>,
    /// <p>Defines mapping between a field in the Google Drive and a Amazon Kendra index field.</p> <p>If you are using the console, you can define index fields when creating the mapping. If you are using the API, you must first create the field using the <a>UpdateIndex</a> operation.</p>
    #[serde(rename = "FieldMappings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub field_mappings: Option<Vec<DataSourceToIndexFieldMapping>>,
    /// <p>A list of regular expression patterns that apply to path on Google Drive. Items that match the pattern are included in the index from both shared drives and users' My Drives. Items that don't match the pattern are excluded from the index. If an item matches both an inclusion pattern and an exclusion pattern, it is excluded from the index.</p>
    #[serde(rename = "InclusionPatterns")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inclusion_patterns: Option<Vec<String>>,
    /// <p>The Amazon Resource Name (ARN) of a AWS Secrets Manager secret that contains the credentials required to connect to Google Drive. For more information, see <a href="https://docs.aws.amazon.com/kendra/latest/dg/data-source-google-drive.html">Using a Google Workspace Drive data source</a>.</p>
    #[serde(rename = "SecretArn")]
    pub secret_arn: String,
}

/// <p>Provides information that you can use to highlight a search result so that your users can quickly identify terms in the response.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
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
    /// <p>The highlight type. </p>
    #[serde(rename = "Type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<HighlightType>,
}

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct UnknownHighlightType {
    name: String,
}

#[allow(non_camel_case_types)]
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
#[non_exhaustive]
pub enum HighlightType {
    Standard,
    ThesaurusSynonym,
    #[doc(hidden)]
    UnknownVariant(UnknownHighlightType),
}

impl Default for HighlightType {
    fn default() -> Self {
        "".into()
    }
}

impl fmt::Display for HighlightType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.into())
    }
}

impl rusoto_core::param::ToParam for HighlightType {
    fn to_param(&self) -> String {
        self.to_string()
    }
}

impl Into<String> for HighlightType {
    fn into(self) -> String {
        match self {
            HighlightType::Standard => "STANDARD".to_string(),
            HighlightType::ThesaurusSynonym => "THESAURUS_SYNONYM".to_string(),
            HighlightType::UnknownVariant(UnknownHighlightType { name: original }) => original,
        }
    }
}

impl<'a> Into<&'a str> for &'a HighlightType {
    fn into(self) -> &'a str {
        match self {
            HighlightType::Standard => &"STANDARD",
            HighlightType::ThesaurusSynonym => &"THESAURUS_SYNONYM",
            HighlightType::UnknownVariant(UnknownHighlightType { name: original }) => original,
        }
    }
}

impl From<&str> for HighlightType {
    fn from(name: &str) -> Self {
        match name {
            "STANDARD" => HighlightType::Standard,
            "THESAURUS_SYNONYM" => HighlightType::ThesaurusSynonym,
            _ => HighlightType::UnknownVariant(UnknownHighlightType {
                name: name.to_owned(),
            }),
        }
    }
}

impl From<String> for HighlightType {
    fn from(name: String) -> Self {
        match &*name {
            "STANDARD" => HighlightType::Standard,
            "THESAURUS_SYNONYM" => HighlightType::ThesaurusSynonym,
            _ => HighlightType::UnknownVariant(UnknownHighlightType { name }),
        }
    }
}

impl ::std::str::FromStr for HighlightType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(s.into())
    }
}

#[cfg(any(test, feature = "serialize_structs"))]
impl Serialize for HighlightType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.into())
    }
}

impl<'de> Deserialize<'de> for HighlightType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Ok(String::deserialize(deserializer)?.into())
    }
}

/// <p>A summary of information about an index.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct IndexConfigurationSummary {
    /// <p>The Unix timestamp when the index was created.</p>
    #[serde(rename = "CreatedAt")]
    pub created_at: f64,
    /// <p>Indicates whether the index is a enterprise edition index or a developer edition index. </p>
    #[serde(rename = "Edition")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub edition: Option<IndexEdition>,
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
    pub status: IndexStatus,
    /// <p>The Unix timestamp when the index was last updated by the <code>UpdateIndex</code> operation.</p>
    #[serde(rename = "UpdatedAt")]
    pub updated_at: f64,
}

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct UnknownIndexEdition {
    name: String,
}

#[allow(non_camel_case_types)]
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
#[non_exhaustive]
pub enum IndexEdition {
    DeveloperEdition,
    EnterpriseEdition,
    #[doc(hidden)]
    UnknownVariant(UnknownIndexEdition),
}

impl Default for IndexEdition {
    fn default() -> Self {
        "".into()
    }
}

impl fmt::Display for IndexEdition {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.into())
    }
}

impl rusoto_core::param::ToParam for IndexEdition {
    fn to_param(&self) -> String {
        self.to_string()
    }
}

impl Into<String> for IndexEdition {
    fn into(self) -> String {
        match self {
            IndexEdition::DeveloperEdition => "DEVELOPER_EDITION".to_string(),
            IndexEdition::EnterpriseEdition => "ENTERPRISE_EDITION".to_string(),
            IndexEdition::UnknownVariant(UnknownIndexEdition { name: original }) => original,
        }
    }
}

impl<'a> Into<&'a str> for &'a IndexEdition {
    fn into(self) -> &'a str {
        match self {
            IndexEdition::DeveloperEdition => &"DEVELOPER_EDITION",
            IndexEdition::EnterpriseEdition => &"ENTERPRISE_EDITION",
            IndexEdition::UnknownVariant(UnknownIndexEdition { name: original }) => original,
        }
    }
}

impl From<&str> for IndexEdition {
    fn from(name: &str) -> Self {
        match name {
            "DEVELOPER_EDITION" => IndexEdition::DeveloperEdition,
            "ENTERPRISE_EDITION" => IndexEdition::EnterpriseEdition,
            _ => IndexEdition::UnknownVariant(UnknownIndexEdition {
                name: name.to_owned(),
            }),
        }
    }
}

impl From<String> for IndexEdition {
    fn from(name: String) -> Self {
        match &*name {
            "DEVELOPER_EDITION" => IndexEdition::DeveloperEdition,
            "ENTERPRISE_EDITION" => IndexEdition::EnterpriseEdition,
            _ => IndexEdition::UnknownVariant(UnknownIndexEdition { name }),
        }
    }
}

impl ::std::str::FromStr for IndexEdition {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(s.into())
    }
}

impl Serialize for IndexEdition {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.into())
    }
}

impl<'de> Deserialize<'de> for IndexEdition {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Ok(String::deserialize(deserializer)?.into())
    }
}

/// <p>Provides information about the number of documents and the number of questions and answers in an index.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct IndexStatistics {
    /// <p>The number of question and answer topics in the index.</p>
    #[serde(rename = "FaqStatistics")]
    pub faq_statistics: FaqStatistics,
    /// <p>The number of text documents indexed.</p>
    #[serde(rename = "TextDocumentStatistics")]
    pub text_document_statistics: TextDocumentStatistics,
}

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct UnknownIndexStatus {
    name: String,
}

#[allow(non_camel_case_types)]
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
#[non_exhaustive]
pub enum IndexStatus {
    Active,
    Creating,
    Deleting,
    Failed,
    SystemUpdating,
    Updating,
    #[doc(hidden)]
    UnknownVariant(UnknownIndexStatus),
}

impl Default for IndexStatus {
    fn default() -> Self {
        "".into()
    }
}

impl fmt::Display for IndexStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.into())
    }
}

impl rusoto_core::param::ToParam for IndexStatus {
    fn to_param(&self) -> String {
        self.to_string()
    }
}

impl Into<String> for IndexStatus {
    fn into(self) -> String {
        match self {
            IndexStatus::Active => "ACTIVE".to_string(),
            IndexStatus::Creating => "CREATING".to_string(),
            IndexStatus::Deleting => "DELETING".to_string(),
            IndexStatus::Failed => "FAILED".to_string(),
            IndexStatus::SystemUpdating => "SYSTEM_UPDATING".to_string(),
            IndexStatus::Updating => "UPDATING".to_string(),
            IndexStatus::UnknownVariant(UnknownIndexStatus { name: original }) => original,
        }
    }
}

impl<'a> Into<&'a str> for &'a IndexStatus {
    fn into(self) -> &'a str {
        match self {
            IndexStatus::Active => &"ACTIVE",
            IndexStatus::Creating => &"CREATING",
            IndexStatus::Deleting => &"DELETING",
            IndexStatus::Failed => &"FAILED",
            IndexStatus::SystemUpdating => &"SYSTEM_UPDATING",
            IndexStatus::Updating => &"UPDATING",
            IndexStatus::UnknownVariant(UnknownIndexStatus { name: original }) => original,
        }
    }
}

impl From<&str> for IndexStatus {
    fn from(name: &str) -> Self {
        match name {
            "ACTIVE" => IndexStatus::Active,
            "CREATING" => IndexStatus::Creating,
            "DELETING" => IndexStatus::Deleting,
            "FAILED" => IndexStatus::Failed,
            "SYSTEM_UPDATING" => IndexStatus::SystemUpdating,
            "UPDATING" => IndexStatus::Updating,
            _ => IndexStatus::UnknownVariant(UnknownIndexStatus {
                name: name.to_owned(),
            }),
        }
    }
}

impl From<String> for IndexStatus {
    fn from(name: String) -> Self {
        match &*name {
            "ACTIVE" => IndexStatus::Active,
            "CREATING" => IndexStatus::Creating,
            "DELETING" => IndexStatus::Deleting,
            "FAILED" => IndexStatus::Failed,
            "SYSTEM_UPDATING" => IndexStatus::SystemUpdating,
            "UPDATING" => IndexStatus::Updating,
            _ => IndexStatus::UnknownVariant(UnknownIndexStatus { name }),
        }
    }
}

impl ::std::str::FromStr for IndexStatus {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(s.into())
    }
}

#[cfg(any(test, feature = "serialize_structs"))]
impl Serialize for IndexStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.into())
    }
}

impl<'de> Deserialize<'de> for IndexStatus {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Ok(String::deserialize(deserializer)?.into())
    }
}

/// <p>Configuration information for the JSON token type.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct JsonTokenTypeConfiguration {
    /// <p>The group attribute field.</p>
    #[serde(rename = "GroupAttributeField")]
    pub group_attribute_field: String,
    /// <p>The user name attribute field.</p>
    #[serde(rename = "UserNameAttributeField")]
    pub user_name_attribute_field: String,
}

/// <p>Configuration information for the JWT token type.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct JwtTokenTypeConfiguration {
    /// <p>The regular expression that identifies the claim.</p>
    #[serde(rename = "ClaimRegex")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub claim_regex: Option<String>,
    /// <p>The group attribute field.</p>
    #[serde(rename = "GroupAttributeField")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_attribute_field: Option<String>,
    /// <p>The issuer of the token.</p>
    #[serde(rename = "Issuer")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub issuer: Option<String>,
    /// <p>The location of the key.</p>
    #[serde(rename = "KeyLocation")]
    pub key_location: KeyLocation,
    /// <p>The Amazon Resource Name (arn) of the secret.</p>
    #[serde(rename = "SecretManagerArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secret_manager_arn: Option<String>,
    /// <p>The signing key URL.</p>
    #[serde(rename = "URL")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    /// <p>The user name attribute field.</p>
    #[serde(rename = "UserNameAttributeField")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_name_attribute_field: Option<String>,
}

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct UnknownKeyLocation {
    name: String,
}

#[allow(non_camel_case_types)]
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
#[non_exhaustive]
pub enum KeyLocation {
    SecretManager,
    Url,
    #[doc(hidden)]
    UnknownVariant(UnknownKeyLocation),
}

impl Default for KeyLocation {
    fn default() -> Self {
        "".into()
    }
}

impl fmt::Display for KeyLocation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.into())
    }
}

impl rusoto_core::param::ToParam for KeyLocation {
    fn to_param(&self) -> String {
        self.to_string()
    }
}

impl Into<String> for KeyLocation {
    fn into(self) -> String {
        match self {
            KeyLocation::SecretManager => "SECRET_MANAGER".to_string(),
            KeyLocation::Url => "URL".to_string(),
            KeyLocation::UnknownVariant(UnknownKeyLocation { name: original }) => original,
        }
    }
}

impl<'a> Into<&'a str> for &'a KeyLocation {
    fn into(self) -> &'a str {
        match self {
            KeyLocation::SecretManager => &"SECRET_MANAGER",
            KeyLocation::Url => &"URL",
            KeyLocation::UnknownVariant(UnknownKeyLocation { name: original }) => original,
        }
    }
}

impl From<&str> for KeyLocation {
    fn from(name: &str) -> Self {
        match name {
            "SECRET_MANAGER" => KeyLocation::SecretManager,
            "URL" => KeyLocation::Url,
            _ => KeyLocation::UnknownVariant(UnknownKeyLocation {
                name: name.to_owned(),
            }),
        }
    }
}

impl From<String> for KeyLocation {
    fn from(name: String) -> Self {
        match &*name {
            "SECRET_MANAGER" => KeyLocation::SecretManager,
            "URL" => KeyLocation::Url,
            _ => KeyLocation::UnknownVariant(UnknownKeyLocation { name }),
        }
    }
}

impl ::std::str::FromStr for KeyLocation {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(s.into())
    }
}

impl Serialize for KeyLocation {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.into())
    }
}

impl<'de> Deserialize<'de> for KeyLocation {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Ok(String::deserialize(deserializer)?.into())
    }
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
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
    pub status_filter: Option<DataSourceSyncJobStatus>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
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

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
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

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
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

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
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

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
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

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
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

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
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

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListTagsForResourceRequest {
    /// <p>The Amazon Resource Name (ARN) of the index, FAQ, or data source to get a list of tags for.</p>
    #[serde(rename = "ResourceARN")]
    pub resource_arn: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListTagsForResourceResponse {
    /// <p>A list of tags associated with the index, FAQ, or data source.</p>
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListThesauriRequest {
    /// <p>The identifier of the index associated with the thesaurus to list.</p>
    #[serde(rename = "IndexId")]
    pub index_id: String,
    /// <p>The maximum number of thesauri to return.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>If the previous response was incomplete (because there is more data to retrieve), Amazon Kendra returns a pagination token in the response. You can use this pagination token to retrieve the next set of thesauri (<code>ThesaurusSummaryItems</code>). </p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListThesauriResponse {
    /// <p>If the response is truncated, Amazon Kendra returns this token that you can use in the subsequent request to retrieve the next set of thesauri. </p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>An array of summary information for one or more thesauruses.</p>
    #[serde(rename = "ThesaurusSummaryItems")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thesaurus_summary_items: Option<Vec<ThesaurusSummary>>,
}

/// <p>Provides configuration information for data sources that connect to OneDrive.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct OneDriveConfiguration {
    /// <p>A Boolean value that specifies whether local groups are disabled (<code>True</code>) or enabled (<code>False</code>). </p>
    #[serde(rename = "DisableLocalGroups")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_local_groups: Option<bool>,
    /// <p>List of regular expressions applied to documents. Items that match the exclusion pattern are not indexed. If you provide both an inclusion pattern and an exclusion pattern, any item that matches the exclusion pattern isn't indexed. </p> <p>The exclusion pattern is applied to the file name.</p>
    #[serde(rename = "ExclusionPatterns")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exclusion_patterns: Option<Vec<String>>,
    /// <p>A list of <code>DataSourceToIndexFieldMapping</code> objects that map Microsoft OneDrive fields to custom fields in the Amazon Kendra index. You must first create the index fields before you map OneDrive fields.</p>
    #[serde(rename = "FieldMappings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub field_mappings: Option<Vec<DataSourceToIndexFieldMapping>>,
    /// <p>A list of regular expression patterns. Documents that match the pattern are included in the index. Documents that don't match the pattern are excluded from the index. If a document matches both an inclusion pattern and an exclusion pattern, the document is not included in the index. </p> <p>The exclusion pattern is applied to the file name.</p>
    #[serde(rename = "InclusionPatterns")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inclusion_patterns: Option<Vec<String>>,
    /// <p>A list of user accounts whose documents should be indexed.</p>
    #[serde(rename = "OneDriveUsers")]
    pub one_drive_users: OneDriveUsers,
    /// <p>The Amazon Resource Name (ARN) of an AWS Secrets Manager secret that contains the user name and password to connect to OneDrive. The user namd should be the application ID for the OneDrive application, and the password is the application key for the OneDrive application.</p>
    #[serde(rename = "SecretArn")]
    pub secret_arn: String,
    /// <p>The Azure Active Directory domain of the organization. </p>
    #[serde(rename = "TenantDomain")]
    pub tenant_domain: String,
}

/// <p>User accounts whose documents should be indexed.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct OneDriveUsers {
    /// <p>A list of users whose documents should be indexed. Specify the user names in email format, for example, <code>username@tenantdomain</code>. If you need to index the documents of more than 100 users, use the <code>OneDriveUserS3Path</code> field to specify the location of a file containing a list of users.</p>
    #[serde(rename = "OneDriveUserList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub one_drive_user_list: Option<Vec<String>>,
    /// <p>The S3 bucket location of a file containing a list of users whose documents should be indexed.</p>
    #[serde(rename = "OneDriveUserS3Path")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub one_drive_user_s3_path: Option<S3Path>,
}

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct UnknownOrder {
    name: String,
}

#[allow(non_camel_case_types)]
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
#[non_exhaustive]
pub enum Order {
    Ascending,
    Descending,
    #[doc(hidden)]
    UnknownVariant(UnknownOrder),
}

impl Default for Order {
    fn default() -> Self {
        "".into()
    }
}

impl fmt::Display for Order {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.into())
    }
}

impl rusoto_core::param::ToParam for Order {
    fn to_param(&self) -> String {
        self.to_string()
    }
}

impl Into<String> for Order {
    fn into(self) -> String {
        match self {
            Order::Ascending => "ASCENDING".to_string(),
            Order::Descending => "DESCENDING".to_string(),
            Order::UnknownVariant(UnknownOrder { name: original }) => original,
        }
    }
}

impl<'a> Into<&'a str> for &'a Order {
    fn into(self) -> &'a str {
        match self {
            Order::Ascending => &"ASCENDING",
            Order::Descending => &"DESCENDING",
            Order::UnknownVariant(UnknownOrder { name: original }) => original,
        }
    }
}

impl From<&str> for Order {
    fn from(name: &str) -> Self {
        match name {
            "ASCENDING" => Order::Ascending,
            "DESCENDING" => Order::Descending,
            _ => Order::UnknownVariant(UnknownOrder {
                name: name.to_owned(),
            }),
        }
    }
}

impl From<String> for Order {
    fn from(name: String) -> Self {
        match &*name {
            "ASCENDING" => Order::Ascending,
            "DESCENDING" => Order::Descending,
            _ => Order::UnknownVariant(UnknownOrder { name }),
        }
    }
}

impl ::std::str::FromStr for Order {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(s.into())
    }
}

impl Serialize for Order {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.into())
    }
}

impl<'de> Deserialize<'de> for Order {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Ok(String::deserialize(deserializer)?.into())
    }
}

/// <p>Provides user and group information for document access filtering.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct Principal {
    /// <p>Whether to allow or deny access to the principal.</p>
    #[serde(rename = "Access")]
    pub access: ReadAccessType,
    /// <p>The name of the user or group.</p>
    #[serde(rename = "Name")]
    pub name: String,
    /// <p>The type of principal.</p>
    #[serde(rename = "Type")]
    pub type_: PrincipalType,
}

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct UnknownPrincipalType {
    name: String,
}

#[allow(non_camel_case_types)]
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
#[non_exhaustive]
pub enum PrincipalType {
    Group,
    User,
    #[doc(hidden)]
    UnknownVariant(UnknownPrincipalType),
}

impl Default for PrincipalType {
    fn default() -> Self {
        "".into()
    }
}

impl fmt::Display for PrincipalType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.into())
    }
}

impl rusoto_core::param::ToParam for PrincipalType {
    fn to_param(&self) -> String {
        self.to_string()
    }
}

impl Into<String> for PrincipalType {
    fn into(self) -> String {
        match self {
            PrincipalType::Group => "GROUP".to_string(),
            PrincipalType::User => "USER".to_string(),
            PrincipalType::UnknownVariant(UnknownPrincipalType { name: original }) => original,
        }
    }
}

impl<'a> Into<&'a str> for &'a PrincipalType {
    fn into(self) -> &'a str {
        match self {
            PrincipalType::Group => &"GROUP",
            PrincipalType::User => &"USER",
            PrincipalType::UnknownVariant(UnknownPrincipalType { name: original }) => original,
        }
    }
}

impl From<&str> for PrincipalType {
    fn from(name: &str) -> Self {
        match name {
            "GROUP" => PrincipalType::Group,
            "USER" => PrincipalType::User,
            _ => PrincipalType::UnknownVariant(UnknownPrincipalType {
                name: name.to_owned(),
            }),
        }
    }
}

impl From<String> for PrincipalType {
    fn from(name: String) -> Self {
        match &*name {
            "GROUP" => PrincipalType::Group,
            "USER" => PrincipalType::User,
            _ => PrincipalType::UnknownVariant(UnknownPrincipalType { name }),
        }
    }
}

impl ::std::str::FromStr for PrincipalType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(s.into())
    }
}

impl Serialize for PrincipalType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.into())
    }
}

#[cfg(feature = "deserialize_structs")]
impl<'de> Deserialize<'de> for PrincipalType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Ok(String::deserialize(deserializer)?.into())
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct UnknownQueryIdentifiersEnclosingOption {
    name: String,
}

#[allow(non_camel_case_types)]
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
#[non_exhaustive]
pub enum QueryIdentifiersEnclosingOption {
    DoubleQuotes,
    None,
    #[doc(hidden)]
    UnknownVariant(UnknownQueryIdentifiersEnclosingOption),
}

impl Default for QueryIdentifiersEnclosingOption {
    fn default() -> Self {
        "".into()
    }
}

impl fmt::Display for QueryIdentifiersEnclosingOption {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.into())
    }
}

impl rusoto_core::param::ToParam for QueryIdentifiersEnclosingOption {
    fn to_param(&self) -> String {
        self.to_string()
    }
}

impl Into<String> for QueryIdentifiersEnclosingOption {
    fn into(self) -> String {
        match self {
            QueryIdentifiersEnclosingOption::DoubleQuotes => "DOUBLE_QUOTES".to_string(),
            QueryIdentifiersEnclosingOption::None => "NONE".to_string(),
            QueryIdentifiersEnclosingOption::UnknownVariant(
                UnknownQueryIdentifiersEnclosingOption { name: original },
            ) => original,
        }
    }
}

impl<'a> Into<&'a str> for &'a QueryIdentifiersEnclosingOption {
    fn into(self) -> &'a str {
        match self {
            QueryIdentifiersEnclosingOption::DoubleQuotes => &"DOUBLE_QUOTES",
            QueryIdentifiersEnclosingOption::None => &"NONE",
            QueryIdentifiersEnclosingOption::UnknownVariant(
                UnknownQueryIdentifiersEnclosingOption { name: original },
            ) => original,
        }
    }
}

impl From<&str> for QueryIdentifiersEnclosingOption {
    fn from(name: &str) -> Self {
        match name {
            "DOUBLE_QUOTES" => QueryIdentifiersEnclosingOption::DoubleQuotes,
            "NONE" => QueryIdentifiersEnclosingOption::None,
            _ => QueryIdentifiersEnclosingOption::UnknownVariant(
                UnknownQueryIdentifiersEnclosingOption {
                    name: name.to_owned(),
                },
            ),
        }
    }
}

impl From<String> for QueryIdentifiersEnclosingOption {
    fn from(name: String) -> Self {
        match &*name {
            "DOUBLE_QUOTES" => QueryIdentifiersEnclosingOption::DoubleQuotes,
            "NONE" => QueryIdentifiersEnclosingOption::None,
            _ => QueryIdentifiersEnclosingOption::UnknownVariant(
                UnknownQueryIdentifiersEnclosingOption { name },
            ),
        }
    }
}

impl ::std::str::FromStr for QueryIdentifiersEnclosingOption {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(s.into())
    }
}

impl Serialize for QueryIdentifiersEnclosingOption {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.into())
    }
}

impl<'de> Deserialize<'de> for QueryIdentifiersEnclosingOption {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Ok(String::deserialize(deserializer)?.into())
    }
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
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
    /// <p>Sets the number of results that are returned in each page of results. The default page size is 10. The maximum number of results returned is 100. If you ask for more than 100 results, only 100 are returned.</p>
    #[serde(rename = "PageSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i64>,
    /// <p>Sets the type of query. Only results for the specified query type are returned.</p>
    #[serde(rename = "QueryResultTypeFilter")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query_result_type_filter: Option<QueryResultType>,
    /// <p>The text to search for.</p>
    #[serde(rename = "QueryText")]
    pub query_text: String,
    /// <p>An array of document attributes to include in the response. No other document attributes are included in the response. By default all document attributes are included in the response. </p>
    #[serde(rename = "RequestedDocumentAttributes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested_document_attributes: Option<Vec<String>>,
    /// <p>Provides information that determines how the results of the query are sorted. You can set the field that Amazon Kendra should sort the results on, and specify whether the results should be sorted in ascending or descending order. In the case of ties in sorting the results, the results are sorted by relevance.</p> <p>If you don't provide sorting configuration, the results are sorted by the relevance that Amazon Kendra determines for the result.</p>
    #[serde(rename = "SortingConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sorting_configuration: Option<SortingConfiguration>,
    /// <p>The user context token.</p>
    #[serde(rename = "UserContext")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_context: Option<UserContext>,
    /// <p>Provides an identifier for a specific user. The <code>VisitorId</code> should be a unique identifier, such as a GUID. Don't use personally identifiable information, such as the user's email address, as the <code>VisitorId</code>.</p>
    #[serde(rename = "VisitorId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub visitor_id: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
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
    /// <p>The total number of items found by the search; however, you can only retrieve up to 100 items. For example, if the search found 192 items, you can only retrieve the first 100 of the items.</p>
    #[serde(rename = "TotalNumberOfResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_number_of_results: Option<i64>,
}

/// <p>A single query result.</p> <p>A query result contains information about a document returned by the query. This includes the original location of the document, a list of attributes assigned to the document, and relevant text from the document that satisfies the query.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct QueryResultItem {
    /// <p>One or more additional attributes associated with the query result.</p>
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
    /// <p>A token that identifies a particular result from a particular query. Use this token to provide click-through feedback for the result. For more information, see <a href="https://docs.aws.amazon.com/kendra/latest/dg/submitting-feedback.html"> Submitting feedback </a>.</p>
    #[serde(rename = "FeedbackToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub feedback_token: Option<String>,
    /// <p>The unique identifier for the query result.</p>
    #[serde(rename = "Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// <p>Indicates the confidence that Amazon Kendra has that a result matches the query that you provided. Each result is placed into a bin that indicates the confidence, <code>VERY_HIGH</code>, <code>HIGH</code>, <code>MEDIUM</code> and <code>LOW</code>. You can use the score to determine if a response meets the confidence needed for your application.</p> <p>The field is only set to <code>LOW</code> when the <code>Type</code> field is set to <code>DOCUMENT</code> and Amazon Kendra is not confident that the result matches the query.</p>
    #[serde(rename = "ScoreAttributes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub score_attributes: Option<ScoreAttributes>,
    /// <p>The type of document. </p>
    #[serde(rename = "Type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<QueryResultType>,
}

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct UnknownQueryResultType {
    name: String,
}

#[allow(non_camel_case_types)]
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
#[non_exhaustive]
pub enum QueryResultType {
    Answer,
    Document,
    QuestionAnswer,
    #[doc(hidden)]
    UnknownVariant(UnknownQueryResultType),
}

impl Default for QueryResultType {
    fn default() -> Self {
        "".into()
    }
}

impl fmt::Display for QueryResultType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.into())
    }
}

impl rusoto_core::param::ToParam for QueryResultType {
    fn to_param(&self) -> String {
        self.to_string()
    }
}

impl Into<String> for QueryResultType {
    fn into(self) -> String {
        match self {
            QueryResultType::Answer => "ANSWER".to_string(),
            QueryResultType::Document => "DOCUMENT".to_string(),
            QueryResultType::QuestionAnswer => "QUESTION_ANSWER".to_string(),
            QueryResultType::UnknownVariant(UnknownQueryResultType { name: original }) => original,
        }
    }
}

impl<'a> Into<&'a str> for &'a QueryResultType {
    fn into(self) -> &'a str {
        match self {
            QueryResultType::Answer => &"ANSWER",
            QueryResultType::Document => &"DOCUMENT",
            QueryResultType::QuestionAnswer => &"QUESTION_ANSWER",
            QueryResultType::UnknownVariant(UnknownQueryResultType { name: original }) => original,
        }
    }
}

impl From<&str> for QueryResultType {
    fn from(name: &str) -> Self {
        match name {
            "ANSWER" => QueryResultType::Answer,
            "DOCUMENT" => QueryResultType::Document,
            "QUESTION_ANSWER" => QueryResultType::QuestionAnswer,
            _ => QueryResultType::UnknownVariant(UnknownQueryResultType {
                name: name.to_owned(),
            }),
        }
    }
}

impl From<String> for QueryResultType {
    fn from(name: String) -> Self {
        match &*name {
            "ANSWER" => QueryResultType::Answer,
            "DOCUMENT" => QueryResultType::Document,
            "QUESTION_ANSWER" => QueryResultType::QuestionAnswer,
            _ => QueryResultType::UnknownVariant(UnknownQueryResultType { name }),
        }
    }
}

impl ::std::str::FromStr for QueryResultType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(s.into())
    }
}

impl Serialize for QueryResultType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.into())
    }
}

impl<'de> Deserialize<'de> for QueryResultType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Ok(String::deserialize(deserializer)?.into())
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct UnknownReadAccessType {
    name: String,
}

#[allow(non_camel_case_types)]
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
#[non_exhaustive]
pub enum ReadAccessType {
    Allow,
    Deny,
    #[doc(hidden)]
    UnknownVariant(UnknownReadAccessType),
}

impl Default for ReadAccessType {
    fn default() -> Self {
        "".into()
    }
}

impl fmt::Display for ReadAccessType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.into())
    }
}

impl rusoto_core::param::ToParam for ReadAccessType {
    fn to_param(&self) -> String {
        self.to_string()
    }
}

impl Into<String> for ReadAccessType {
    fn into(self) -> String {
        match self {
            ReadAccessType::Allow => "ALLOW".to_string(),
            ReadAccessType::Deny => "DENY".to_string(),
            ReadAccessType::UnknownVariant(UnknownReadAccessType { name: original }) => original,
        }
    }
}

impl<'a> Into<&'a str> for &'a ReadAccessType {
    fn into(self) -> &'a str {
        match self {
            ReadAccessType::Allow => &"ALLOW",
            ReadAccessType::Deny => &"DENY",
            ReadAccessType::UnknownVariant(UnknownReadAccessType { name: original }) => original,
        }
    }
}

impl From<&str> for ReadAccessType {
    fn from(name: &str) -> Self {
        match name {
            "ALLOW" => ReadAccessType::Allow,
            "DENY" => ReadAccessType::Deny,
            _ => ReadAccessType::UnknownVariant(UnknownReadAccessType {
                name: name.to_owned(),
            }),
        }
    }
}

impl From<String> for ReadAccessType {
    fn from(name: String) -> Self {
        match &*name {
            "ALLOW" => ReadAccessType::Allow,
            "DENY" => ReadAccessType::Deny,
            _ => ReadAccessType::UnknownVariant(UnknownReadAccessType { name }),
        }
    }
}

impl ::std::str::FromStr for ReadAccessType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(s.into())
    }
}

impl Serialize for ReadAccessType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.into())
    }
}

#[cfg(feature = "deserialize_structs")]
impl<'de> Deserialize<'de> for ReadAccessType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Ok(String::deserialize(deserializer)?.into())
    }
}

/// <p>Provides information for manually tuning the relevance of a field in a search. When a query includes terms that match the field, the results are given a boost in the response based on these tuning parameters.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
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
    pub rank_order: Option<Order>,
    /// <p>A list of values that should be given a different boost when they appear in the result list. For example, if you are boosting a field called "department," query terms that match the department field are boosted in the result. However, you can add entries from the department field to boost documents with those values higher. </p> <p>For example, you can add entries to the map with names of departments. If you add "HR",5 and "Legal",3 those departments are given special attention when they appear in the metadata of a document. When those terms appear they are given the specified importance instead of the regular importance for the boost.</p>
    #[serde(rename = "ValueImportanceMap")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value_importance_map: Option<::std::collections::HashMap<String, i64>>,
}

/// <p>Provides feedback on how relevant a document is to a search. Your application uses the <a>SubmitFeedback</a> operation to provide relevance information.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct RelevanceFeedback {
    /// <p>Whether to document was relevant or not relevant to the search.</p>
    #[serde(rename = "RelevanceValue")]
    pub relevance_value: RelevanceType,
    /// <p>The unique identifier of the search result that the user provided relevance feedback for.</p>
    #[serde(rename = "ResultId")]
    pub result_id: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct UnknownRelevanceType {
    name: String,
}

#[allow(non_camel_case_types)]
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
#[non_exhaustive]
pub enum RelevanceType {
    NotRelevant,
    Relevant,
    #[doc(hidden)]
    UnknownVariant(UnknownRelevanceType),
}

impl Default for RelevanceType {
    fn default() -> Self {
        "".into()
    }
}

impl fmt::Display for RelevanceType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.into())
    }
}

impl rusoto_core::param::ToParam for RelevanceType {
    fn to_param(&self) -> String {
        self.to_string()
    }
}

impl Into<String> for RelevanceType {
    fn into(self) -> String {
        match self {
            RelevanceType::NotRelevant => "NOT_RELEVANT".to_string(),
            RelevanceType::Relevant => "RELEVANT".to_string(),
            RelevanceType::UnknownVariant(UnknownRelevanceType { name: original }) => original,
        }
    }
}

impl<'a> Into<&'a str> for &'a RelevanceType {
    fn into(self) -> &'a str {
        match self {
            RelevanceType::NotRelevant => &"NOT_RELEVANT",
            RelevanceType::Relevant => &"RELEVANT",
            RelevanceType::UnknownVariant(UnknownRelevanceType { name: original }) => original,
        }
    }
}

impl From<&str> for RelevanceType {
    fn from(name: &str) -> Self {
        match name {
            "NOT_RELEVANT" => RelevanceType::NotRelevant,
            "RELEVANT" => RelevanceType::Relevant,
            _ => RelevanceType::UnknownVariant(UnknownRelevanceType {
                name: name.to_owned(),
            }),
        }
    }
}

impl From<String> for RelevanceType {
    fn from(name: String) -> Self {
        match &*name {
            "NOT_RELEVANT" => RelevanceType::NotRelevant,
            "RELEVANT" => RelevanceType::Relevant,
            _ => RelevanceType::UnknownVariant(UnknownRelevanceType { name }),
        }
    }
}

impl ::std::str::FromStr for RelevanceType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(s.into())
    }
}

impl Serialize for RelevanceType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.into())
    }
}

#[cfg(feature = "deserialize_structs")]
impl<'de> Deserialize<'de> for RelevanceType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Ok(String::deserialize(deserializer)?.into())
    }
}

/// <p>Provides configuration information for a data source to index documents in an Amazon S3 bucket.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct S3DataSourceConfiguration {
    /// <p>Provides the path to the S3 bucket that contains the user context filtering files for the data source. For the format of the file, see <a href="https://docs.aws.amazon.com/kendra/latest/dg/s3-acl.html">Access control for S3 data sources</a>.</p>
    #[serde(rename = "AccessControlListConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_control_list_configuration: Option<AccessControlListConfiguration>,
    /// <p>The name of the bucket that contains the documents.</p>
    #[serde(rename = "BucketName")]
    pub bucket_name: String,
    #[serde(rename = "DocumentsMetadataConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub documents_metadata_configuration: Option<DocumentsMetadataConfiguration>,
    /// <p>A list of glob patterns for documents that should not be indexed. If a document that matches an inclusion prefix or inclusion pattern also matches an exclusion pattern, the document is not indexed.</p> <p>For more information about glob patterns, see <a href="https://en.wikipedia.org/wiki/Glob_(programming)">glob (programming)</a> in <i>Wikipedia</i>.</p>
    #[serde(rename = "ExclusionPatterns")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exclusion_patterns: Option<Vec<String>>,
    /// <p>A list of glob patterns for documents that should be indexed. If a document that matches an inclusion pattern also matches an exclusion pattern, the document is not indexed.</p> <p>For more information about glob patterns, see <a href="https://en.wikipedia.org/wiki/Glob_(programming)">glob (programming)</a> in <i>Wikipedia</i>.</p>
    #[serde(rename = "InclusionPatterns")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inclusion_patterns: Option<Vec<String>>,
    /// <p>A list of S3 prefixes for the documents that should be included in the index.</p>
    #[serde(rename = "InclusionPrefixes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inclusion_prefixes: Option<Vec<String>>,
}

/// <p>Information required to find a specific file in an Amazon S3 bucket.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct S3Path {
    /// <p>The name of the S3 bucket that contains the file.</p>
    #[serde(rename = "Bucket")]
    pub bucket: String,
    /// <p>The name of the file.</p>
    #[serde(rename = "Key")]
    pub key: String,
}

/// <p>Defines configuration for syncing a Salesforce chatter feed. The contents of the object comes from the Salesforce FeedItem table.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct SalesforceChatterFeedConfiguration {
    /// <p>The name of the column in the Salesforce FeedItem table that contains the content to index. Typically this is the <code>Body</code> column.</p>
    #[serde(rename = "DocumentDataFieldName")]
    pub document_data_field_name: String,
    /// <p>The name of the column in the Salesforce FeedItem table that contains the title of the document. This is typically the <code>Title</code> collumn.</p>
    #[serde(rename = "DocumentTitleFieldName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document_title_field_name: Option<String>,
    /// <p>Maps fields from a Salesforce chatter feed into Amazon Kendra index fields.</p>
    #[serde(rename = "FieldMappings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub field_mappings: Option<Vec<DataSourceToIndexFieldMapping>>,
    /// <p>Filters the documents in the feed based on status of the user. When you specify <code>ACTIVE_USERS</code> only documents from users who have an active account are indexed. When you specify <code>STANDARD_USER</code> only documents for Salesforce standard users are documented. You can specify both.</p>
    #[serde(rename = "IncludeFilterTypes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_filter_types: Option<Vec<SalesforceChatterFeedIncludeFilterType>>,
}

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct UnknownSalesforceChatterFeedIncludeFilterType {
    name: String,
}

#[allow(non_camel_case_types)]
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
#[non_exhaustive]
pub enum SalesforceChatterFeedIncludeFilterType {
    ActiveUser,
    StandardUser,
    #[doc(hidden)]
    UnknownVariant(UnknownSalesforceChatterFeedIncludeFilterType),
}

impl Default for SalesforceChatterFeedIncludeFilterType {
    fn default() -> Self {
        "".into()
    }
}

impl fmt::Display for SalesforceChatterFeedIncludeFilterType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.into())
    }
}

impl rusoto_core::param::ToParam for SalesforceChatterFeedIncludeFilterType {
    fn to_param(&self) -> String {
        self.to_string()
    }
}

impl Into<String> for SalesforceChatterFeedIncludeFilterType {
    fn into(self) -> String {
        match self {
            SalesforceChatterFeedIncludeFilterType::ActiveUser => "ACTIVE_USER".to_string(),
            SalesforceChatterFeedIncludeFilterType::StandardUser => "STANDARD_USER".to_string(),
            SalesforceChatterFeedIncludeFilterType::UnknownVariant(
                UnknownSalesforceChatterFeedIncludeFilterType { name: original },
            ) => original,
        }
    }
}

impl<'a> Into<&'a str> for &'a SalesforceChatterFeedIncludeFilterType {
    fn into(self) -> &'a str {
        match self {
            SalesforceChatterFeedIncludeFilterType::ActiveUser => &"ACTIVE_USER",
            SalesforceChatterFeedIncludeFilterType::StandardUser => &"STANDARD_USER",
            SalesforceChatterFeedIncludeFilterType::UnknownVariant(
                UnknownSalesforceChatterFeedIncludeFilterType { name: original },
            ) => original,
        }
    }
}

impl From<&str> for SalesforceChatterFeedIncludeFilterType {
    fn from(name: &str) -> Self {
        match name {
            "ACTIVE_USER" => SalesforceChatterFeedIncludeFilterType::ActiveUser,
            "STANDARD_USER" => SalesforceChatterFeedIncludeFilterType::StandardUser,
            _ => SalesforceChatterFeedIncludeFilterType::UnknownVariant(
                UnknownSalesforceChatterFeedIncludeFilterType {
                    name: name.to_owned(),
                },
            ),
        }
    }
}

impl From<String> for SalesforceChatterFeedIncludeFilterType {
    fn from(name: String) -> Self {
        match &*name {
            "ACTIVE_USER" => SalesforceChatterFeedIncludeFilterType::ActiveUser,
            "STANDARD_USER" => SalesforceChatterFeedIncludeFilterType::StandardUser,
            _ => SalesforceChatterFeedIncludeFilterType::UnknownVariant(
                UnknownSalesforceChatterFeedIncludeFilterType { name },
            ),
        }
    }
}

impl ::std::str::FromStr for SalesforceChatterFeedIncludeFilterType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(s.into())
    }
}

impl Serialize for SalesforceChatterFeedIncludeFilterType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.into())
    }
}

impl<'de> Deserialize<'de> for SalesforceChatterFeedIncludeFilterType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Ok(String::deserialize(deserializer)?.into())
    }
}

/// <p>Provides configuration information for connecting to a Salesforce data source.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct SalesforceConfiguration {
    /// <p>Specifies configuration information for Salesforce chatter feeds.</p>
    #[serde(rename = "ChatterFeedConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chatter_feed_configuration: Option<SalesforceChatterFeedConfiguration>,
    /// <p>Indicates whether Amazon Kendra should index attachments to Salesforce objects.</p>
    #[serde(rename = "CrawlAttachments")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub crawl_attachments: Option<bool>,
    /// <p>A list of regular expression patterns. Documents that match the patterns are excluded from the index. Documents that don't match the patterns are included in the index. If a document matches both an exclusion pattern and an inclusion pattern, the document is not included in the index.</p> <p>The regex is applied to the name of the attached file.</p>
    #[serde(rename = "ExcludeAttachmentFilePatterns")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exclude_attachment_file_patterns: Option<Vec<String>>,
    /// <p>A list of regular expression patterns. Documents that match the patterns are included in the index. Documents that don't match the patterns are excluded from the index. If a document matches both an inclusion pattern and an exclusion pattern, the document is not included in the index.</p> <p>The regex is applied to the name of the attached file.</p>
    #[serde(rename = "IncludeAttachmentFilePatterns")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_attachment_file_patterns: Option<Vec<String>>,
    /// <p>Specifies configuration information for the knowlege article types that Amazon Kendra indexes. Amazon Kendra indexes standard knowledge articles and the standard fields of knowledge articles, or the custom fields of custom knowledge articles, but not both.</p>
    #[serde(rename = "KnowledgeArticleConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub knowledge_article_configuration: Option<SalesforceKnowledgeArticleConfiguration>,
    /// <p><p>The Amazon Resource Name (ARN) of an AWS Secrets Manager secret that contains the key/value pairs required to connect to your Salesforce instance. The secret must contain a JSON structure with the following keys:</p> <ul> <li> <p>authenticationUrl - The OAUTH endpoint that Amazon Kendra connects to get an OAUTH token. </p> </li> <li> <p>consumerKey - The application public key generated when you created your Salesforce application.</p> </li> <li> <p>consumerSecret - The application private key generated when you created your Salesforce application.</p> </li> <li> <p>password - The password associated with the user logging in to the Salesforce instance.</p> </li> <li> <p>securityToken - The token associated with the user account logging in to the Salesforce instance.</p> </li> <li> <p>username - The user name of the user logging in to the Salesforce instance.</p> </li> </ul></p>
    #[serde(rename = "SecretArn")]
    pub secret_arn: String,
    /// <p>The instance URL for the Salesforce site that you want to index.</p>
    #[serde(rename = "ServerUrl")]
    pub server_url: String,
    /// <p>Provides configuration information for processing attachments to Salesforce standard objects. </p>
    #[serde(rename = "StandardObjectAttachmentConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub standard_object_attachment_configuration:
        Option<SalesforceStandardObjectAttachmentConfiguration>,
    /// <p>Specifies the Salesforce standard objects that Amazon Kendra indexes.</p>
    #[serde(rename = "StandardObjectConfigurations")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub standard_object_configurations: Option<Vec<SalesforceStandardObjectConfiguration>>,
}

/// <p>Provides configuration information for indexing Salesforce custom articles.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct SalesforceCustomKnowledgeArticleTypeConfiguration {
    /// <p>The name of the field in the custom knowledge article that contains the document data to index.</p>
    #[serde(rename = "DocumentDataFieldName")]
    pub document_data_field_name: String,
    /// <p>The name of the field in the custom knowledge article that contains the document title.</p>
    #[serde(rename = "DocumentTitleFieldName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document_title_field_name: Option<String>,
    /// <p>One or more objects that map fields in the custom knowledge article to fields in the Amazon Kendra index.</p>
    #[serde(rename = "FieldMappings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub field_mappings: Option<Vec<DataSourceToIndexFieldMapping>>,
    /// <p>The name of the configuration.</p>
    #[serde(rename = "Name")]
    pub name: String,
}

/// <p>Specifies configuration information for the knowlege article types that Amazon Kendra indexes. Amazon Kendra indexes standard knowledge articles and the standard fields of knowledge articles, or the custom fields of custom knowledge articles, but not both </p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct SalesforceKnowledgeArticleConfiguration {
    /// <p>Provides configuration information for custom Salesforce knowledge articles.</p>
    #[serde(rename = "CustomKnowledgeArticleTypeConfigurations")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_knowledge_article_type_configurations:
        Option<Vec<SalesforceCustomKnowledgeArticleTypeConfiguration>>,
    /// <p>Specifies the document states that should be included when Amazon Kendra indexes knowledge articles. You must specify at least one state.</p>
    #[serde(rename = "IncludedStates")]
    pub included_states: Vec<SalesforceKnowledgeArticleState>,
    /// <p>Provides configuration information for standard Salesforce knowledge articles.</p>
    #[serde(rename = "StandardKnowledgeArticleTypeConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub standard_knowledge_article_type_configuration:
        Option<SalesforceStandardKnowledgeArticleTypeConfiguration>,
}

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct UnknownSalesforceKnowledgeArticleState {
    name: String,
}

#[allow(non_camel_case_types)]
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
#[non_exhaustive]
pub enum SalesforceKnowledgeArticleState {
    Archived,
    Draft,
    Published,
    #[doc(hidden)]
    UnknownVariant(UnknownSalesforceKnowledgeArticleState),
}

impl Default for SalesforceKnowledgeArticleState {
    fn default() -> Self {
        "".into()
    }
}

impl fmt::Display for SalesforceKnowledgeArticleState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.into())
    }
}

impl rusoto_core::param::ToParam for SalesforceKnowledgeArticleState {
    fn to_param(&self) -> String {
        self.to_string()
    }
}

impl Into<String> for SalesforceKnowledgeArticleState {
    fn into(self) -> String {
        match self {
            SalesforceKnowledgeArticleState::Archived => "ARCHIVED".to_string(),
            SalesforceKnowledgeArticleState::Draft => "DRAFT".to_string(),
            SalesforceKnowledgeArticleState::Published => "PUBLISHED".to_string(),
            SalesforceKnowledgeArticleState::UnknownVariant(
                UnknownSalesforceKnowledgeArticleState { name: original },
            ) => original,
        }
    }
}

impl<'a> Into<&'a str> for &'a SalesforceKnowledgeArticleState {
    fn into(self) -> &'a str {
        match self {
            SalesforceKnowledgeArticleState::Archived => &"ARCHIVED",
            SalesforceKnowledgeArticleState::Draft => &"DRAFT",
            SalesforceKnowledgeArticleState::Published => &"PUBLISHED",
            SalesforceKnowledgeArticleState::UnknownVariant(
                UnknownSalesforceKnowledgeArticleState { name: original },
            ) => original,
        }
    }
}

impl From<&str> for SalesforceKnowledgeArticleState {
    fn from(name: &str) -> Self {
        match name {
            "ARCHIVED" => SalesforceKnowledgeArticleState::Archived,
            "DRAFT" => SalesforceKnowledgeArticleState::Draft,
            "PUBLISHED" => SalesforceKnowledgeArticleState::Published,
            _ => SalesforceKnowledgeArticleState::UnknownVariant(
                UnknownSalesforceKnowledgeArticleState {
                    name: name.to_owned(),
                },
            ),
        }
    }
}

impl From<String> for SalesforceKnowledgeArticleState {
    fn from(name: String) -> Self {
        match &*name {
            "ARCHIVED" => SalesforceKnowledgeArticleState::Archived,
            "DRAFT" => SalesforceKnowledgeArticleState::Draft,
            "PUBLISHED" => SalesforceKnowledgeArticleState::Published,
            _ => SalesforceKnowledgeArticleState::UnknownVariant(
                UnknownSalesforceKnowledgeArticleState { name },
            ),
        }
    }
}

impl ::std::str::FromStr for SalesforceKnowledgeArticleState {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(s.into())
    }
}

impl Serialize for SalesforceKnowledgeArticleState {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.into())
    }
}

impl<'de> Deserialize<'de> for SalesforceKnowledgeArticleState {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Ok(String::deserialize(deserializer)?.into())
    }
}

/// <p>Provides configuration information for standard Salesforce knowledge articles.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct SalesforceStandardKnowledgeArticleTypeConfiguration {
    /// <p>The name of the field that contains the document data to index.</p>
    #[serde(rename = "DocumentDataFieldName")]
    pub document_data_field_name: String,
    /// <p>The name of the field that contains the document title.</p>
    #[serde(rename = "DocumentTitleFieldName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document_title_field_name: Option<String>,
    /// <p>One or more objects that map fields in the knowledge article to Amazon Kendra index fields. The index field must exist before you can map a Salesforce field to it.</p>
    #[serde(rename = "FieldMappings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub field_mappings: Option<Vec<DataSourceToIndexFieldMapping>>,
}

/// <p>Provides configuration information for processing attachments to Salesforce standard objects. </p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct SalesforceStandardObjectAttachmentConfiguration {
    /// <p>The name of the field used for the document title.</p>
    #[serde(rename = "DocumentTitleFieldName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document_title_field_name: Option<String>,
    /// <p>One or more objects that map fields in attachments to Amazon Kendra index fields.</p>
    #[serde(rename = "FieldMappings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub field_mappings: Option<Vec<DataSourceToIndexFieldMapping>>,
}

/// <p>Specifies confguration information for indexing a single standard object.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct SalesforceStandardObjectConfiguration {
    /// <p>The name of the field in the standard object table that contains the document contents.</p>
    #[serde(rename = "DocumentDataFieldName")]
    pub document_data_field_name: String,
    /// <p>The name of the field in the standard object table that contains the document titleB.</p>
    #[serde(rename = "DocumentTitleFieldName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document_title_field_name: Option<String>,
    /// <p>One or more objects that map fields in the standard object to Amazon Kendra index fields. The index field must exist before you can map a Salesforce field to it.</p>
    #[serde(rename = "FieldMappings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub field_mappings: Option<Vec<DataSourceToIndexFieldMapping>>,
    /// <p>The name of the standard object.</p>
    #[serde(rename = "Name")]
    pub name: SalesforceStandardObjectName,
}

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct UnknownSalesforceStandardObjectName {
    name: String,
}

#[allow(non_camel_case_types)]
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
#[non_exhaustive]
pub enum SalesforceStandardObjectName {
    Account,
    Campaign,
    Case,
    Contact,
    Contract,
    Document,
    Group,
    Idea,
    Lead,
    Opportunity,
    Partner,
    Pricebook,
    Product,
    Profile,
    Solution,
    Task,
    User,
    #[doc(hidden)]
    UnknownVariant(UnknownSalesforceStandardObjectName),
}

impl Default for SalesforceStandardObjectName {
    fn default() -> Self {
        "".into()
    }
}

impl fmt::Display for SalesforceStandardObjectName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.into())
    }
}

impl rusoto_core::param::ToParam for SalesforceStandardObjectName {
    fn to_param(&self) -> String {
        self.to_string()
    }
}

impl Into<String> for SalesforceStandardObjectName {
    fn into(self) -> String {
        match self {
            SalesforceStandardObjectName::Account => "ACCOUNT".to_string(),
            SalesforceStandardObjectName::Campaign => "CAMPAIGN".to_string(),
            SalesforceStandardObjectName::Case => "CASE".to_string(),
            SalesforceStandardObjectName::Contact => "CONTACT".to_string(),
            SalesforceStandardObjectName::Contract => "CONTRACT".to_string(),
            SalesforceStandardObjectName::Document => "DOCUMENT".to_string(),
            SalesforceStandardObjectName::Group => "GROUP".to_string(),
            SalesforceStandardObjectName::Idea => "IDEA".to_string(),
            SalesforceStandardObjectName::Lead => "LEAD".to_string(),
            SalesforceStandardObjectName::Opportunity => "OPPORTUNITY".to_string(),
            SalesforceStandardObjectName::Partner => "PARTNER".to_string(),
            SalesforceStandardObjectName::Pricebook => "PRICEBOOK".to_string(),
            SalesforceStandardObjectName::Product => "PRODUCT".to_string(),
            SalesforceStandardObjectName::Profile => "PROFILE".to_string(),
            SalesforceStandardObjectName::Solution => "SOLUTION".to_string(),
            SalesforceStandardObjectName::Task => "TASK".to_string(),
            SalesforceStandardObjectName::User => "USER".to_string(),
            SalesforceStandardObjectName::UnknownVariant(UnknownSalesforceStandardObjectName {
                name: original,
            }) => original,
        }
    }
}

impl<'a> Into<&'a str> for &'a SalesforceStandardObjectName {
    fn into(self) -> &'a str {
        match self {
            SalesforceStandardObjectName::Account => &"ACCOUNT",
            SalesforceStandardObjectName::Campaign => &"CAMPAIGN",
            SalesforceStandardObjectName::Case => &"CASE",
            SalesforceStandardObjectName::Contact => &"CONTACT",
            SalesforceStandardObjectName::Contract => &"CONTRACT",
            SalesforceStandardObjectName::Document => &"DOCUMENT",
            SalesforceStandardObjectName::Group => &"GROUP",
            SalesforceStandardObjectName::Idea => &"IDEA",
            SalesforceStandardObjectName::Lead => &"LEAD",
            SalesforceStandardObjectName::Opportunity => &"OPPORTUNITY",
            SalesforceStandardObjectName::Partner => &"PARTNER",
            SalesforceStandardObjectName::Pricebook => &"PRICEBOOK",
            SalesforceStandardObjectName::Product => &"PRODUCT",
            SalesforceStandardObjectName::Profile => &"PROFILE",
            SalesforceStandardObjectName::Solution => &"SOLUTION",
            SalesforceStandardObjectName::Task => &"TASK",
            SalesforceStandardObjectName::User => &"USER",
            SalesforceStandardObjectName::UnknownVariant(UnknownSalesforceStandardObjectName {
                name: original,
            }) => original,
        }
    }
}

impl From<&str> for SalesforceStandardObjectName {
    fn from(name: &str) -> Self {
        match name {
            "ACCOUNT" => SalesforceStandardObjectName::Account,
            "CAMPAIGN" => SalesforceStandardObjectName::Campaign,
            "CASE" => SalesforceStandardObjectName::Case,
            "CONTACT" => SalesforceStandardObjectName::Contact,
            "CONTRACT" => SalesforceStandardObjectName::Contract,
            "DOCUMENT" => SalesforceStandardObjectName::Document,
            "GROUP" => SalesforceStandardObjectName::Group,
            "IDEA" => SalesforceStandardObjectName::Idea,
            "LEAD" => SalesforceStandardObjectName::Lead,
            "OPPORTUNITY" => SalesforceStandardObjectName::Opportunity,
            "PARTNER" => SalesforceStandardObjectName::Partner,
            "PRICEBOOK" => SalesforceStandardObjectName::Pricebook,
            "PRODUCT" => SalesforceStandardObjectName::Product,
            "PROFILE" => SalesforceStandardObjectName::Profile,
            "SOLUTION" => SalesforceStandardObjectName::Solution,
            "TASK" => SalesforceStandardObjectName::Task,
            "USER" => SalesforceStandardObjectName::User,
            _ => {
                SalesforceStandardObjectName::UnknownVariant(UnknownSalesforceStandardObjectName {
                    name: name.to_owned(),
                })
            }
        }
    }
}

impl From<String> for SalesforceStandardObjectName {
    fn from(name: String) -> Self {
        match &*name {
            "ACCOUNT" => SalesforceStandardObjectName::Account,
            "CAMPAIGN" => SalesforceStandardObjectName::Campaign,
            "CASE" => SalesforceStandardObjectName::Case,
            "CONTACT" => SalesforceStandardObjectName::Contact,
            "CONTRACT" => SalesforceStandardObjectName::Contract,
            "DOCUMENT" => SalesforceStandardObjectName::Document,
            "GROUP" => SalesforceStandardObjectName::Group,
            "IDEA" => SalesforceStandardObjectName::Idea,
            "LEAD" => SalesforceStandardObjectName::Lead,
            "OPPORTUNITY" => SalesforceStandardObjectName::Opportunity,
            "PARTNER" => SalesforceStandardObjectName::Partner,
            "PRICEBOOK" => SalesforceStandardObjectName::Pricebook,
            "PRODUCT" => SalesforceStandardObjectName::Product,
            "PROFILE" => SalesforceStandardObjectName::Profile,
            "SOLUTION" => SalesforceStandardObjectName::Solution,
            "TASK" => SalesforceStandardObjectName::Task,
            "USER" => SalesforceStandardObjectName::User,
            _ => {
                SalesforceStandardObjectName::UnknownVariant(UnknownSalesforceStandardObjectName {
                    name,
                })
            }
        }
    }
}

impl ::std::str::FromStr for SalesforceStandardObjectName {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(s.into())
    }
}

impl Serialize for SalesforceStandardObjectName {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.into())
    }
}

impl<'de> Deserialize<'de> for SalesforceStandardObjectName {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Ok(String::deserialize(deserializer)?.into())
    }
}

/// <p>Provides a relative ranking that indicates how confident Amazon Kendra is that the response matches the query.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ScoreAttributes {
    /// <p>A relative ranking for how well the response matches the query.</p>
    #[serde(rename = "ScoreConfidence")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub score_confidence: Option<ScoreConfidence>,
}

/// <p>Enumeration for query score confidence.</p>

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct UnknownScoreConfidence {
    name: String,
}

#[allow(non_camel_case_types)]
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
#[non_exhaustive]
pub enum ScoreConfidence {
    High,
    Low,
    Medium,
    VeryHigh,
    #[doc(hidden)]
    UnknownVariant(UnknownScoreConfidence),
}

impl Default for ScoreConfidence {
    fn default() -> Self {
        "".into()
    }
}

impl fmt::Display for ScoreConfidence {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.into())
    }
}

impl rusoto_core::param::ToParam for ScoreConfidence {
    fn to_param(&self) -> String {
        self.to_string()
    }
}

impl Into<String> for ScoreConfidence {
    fn into(self) -> String {
        match self {
            ScoreConfidence::High => "HIGH".to_string(),
            ScoreConfidence::Low => "LOW".to_string(),
            ScoreConfidence::Medium => "MEDIUM".to_string(),
            ScoreConfidence::VeryHigh => "VERY_HIGH".to_string(),
            ScoreConfidence::UnknownVariant(UnknownScoreConfidence { name: original }) => original,
        }
    }
}

impl<'a> Into<&'a str> for &'a ScoreConfidence {
    fn into(self) -> &'a str {
        match self {
            ScoreConfidence::High => &"HIGH",
            ScoreConfidence::Low => &"LOW",
            ScoreConfidence::Medium => &"MEDIUM",
            ScoreConfidence::VeryHigh => &"VERY_HIGH",
            ScoreConfidence::UnknownVariant(UnknownScoreConfidence { name: original }) => original,
        }
    }
}

impl From<&str> for ScoreConfidence {
    fn from(name: &str) -> Self {
        match name {
            "HIGH" => ScoreConfidence::High,
            "LOW" => ScoreConfidence::Low,
            "MEDIUM" => ScoreConfidence::Medium,
            "VERY_HIGH" => ScoreConfidence::VeryHigh,
            _ => ScoreConfidence::UnknownVariant(UnknownScoreConfidence {
                name: name.to_owned(),
            }),
        }
    }
}

impl From<String> for ScoreConfidence {
    fn from(name: String) -> Self {
        match &*name {
            "HIGH" => ScoreConfidence::High,
            "LOW" => ScoreConfidence::Low,
            "MEDIUM" => ScoreConfidence::Medium,
            "VERY_HIGH" => ScoreConfidence::VeryHigh,
            _ => ScoreConfidence::UnknownVariant(UnknownScoreConfidence { name }),
        }
    }
}

impl ::std::str::FromStr for ScoreConfidence {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(s.into())
    }
}

#[cfg(any(test, feature = "serialize_structs"))]
impl Serialize for ScoreConfidence {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.into())
    }
}

impl<'de> Deserialize<'de> for ScoreConfidence {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Ok(String::deserialize(deserializer)?.into())
    }
}

/// <p>Provides information about how a custom index field is used during a search.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
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
    /// <p>Determines whether the field can be used to sort the results of a query. If you specify sorting on a field that does not have <code>Sortable</code> set to <code>true</code>, Amazon Kendra returns an exception. The default is <code>false</code>.</p>
    #[serde(rename = "Sortable")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sortable: Option<bool>,
}

/// <p>Provides the identifier of the AWS KMS customer master key (CMK) used to encrypt data indexed by Amazon Kendra. Amazon Kendra doesn't support asymmetric CMKs.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct ServerSideEncryptionConfiguration {
    /// <p>The identifier of the AWS KMS customer master key (CMK). Amazon Kendra doesn't support asymmetric CMKs.</p>
    #[serde(rename = "KmsKeyId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_key_id: Option<String>,
}

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct UnknownServiceNowBuildVersionType {
    name: String,
}

#[allow(non_camel_case_types)]
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
#[non_exhaustive]
pub enum ServiceNowBuildVersionType {
    London,
    Others,
    #[doc(hidden)]
    UnknownVariant(UnknownServiceNowBuildVersionType),
}

impl Default for ServiceNowBuildVersionType {
    fn default() -> Self {
        "".into()
    }
}

impl fmt::Display for ServiceNowBuildVersionType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.into())
    }
}

impl rusoto_core::param::ToParam for ServiceNowBuildVersionType {
    fn to_param(&self) -> String {
        self.to_string()
    }
}

impl Into<String> for ServiceNowBuildVersionType {
    fn into(self) -> String {
        match self {
            ServiceNowBuildVersionType::London => "LONDON".to_string(),
            ServiceNowBuildVersionType::Others => "OTHERS".to_string(),
            ServiceNowBuildVersionType::UnknownVariant(UnknownServiceNowBuildVersionType {
                name: original,
            }) => original,
        }
    }
}

impl<'a> Into<&'a str> for &'a ServiceNowBuildVersionType {
    fn into(self) -> &'a str {
        match self {
            ServiceNowBuildVersionType::London => &"LONDON",
            ServiceNowBuildVersionType::Others => &"OTHERS",
            ServiceNowBuildVersionType::UnknownVariant(UnknownServiceNowBuildVersionType {
                name: original,
            }) => original,
        }
    }
}

impl From<&str> for ServiceNowBuildVersionType {
    fn from(name: &str) -> Self {
        match name {
            "LONDON" => ServiceNowBuildVersionType::London,
            "OTHERS" => ServiceNowBuildVersionType::Others,
            _ => ServiceNowBuildVersionType::UnknownVariant(UnknownServiceNowBuildVersionType {
                name: name.to_owned(),
            }),
        }
    }
}

impl From<String> for ServiceNowBuildVersionType {
    fn from(name: String) -> Self {
        match &*name {
            "LONDON" => ServiceNowBuildVersionType::London,
            "OTHERS" => ServiceNowBuildVersionType::Others,
            _ => ServiceNowBuildVersionType::UnknownVariant(UnknownServiceNowBuildVersionType {
                name,
            }),
        }
    }
}

impl ::std::str::FromStr for ServiceNowBuildVersionType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(s.into())
    }
}

impl Serialize for ServiceNowBuildVersionType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.into())
    }
}

impl<'de> Deserialize<'de> for ServiceNowBuildVersionType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Ok(String::deserialize(deserializer)?.into())
    }
}

/// <p>Provides configuration information required to connect to a ServiceNow data source.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct ServiceNowConfiguration {
    /// <p>The ServiceNow instance that the data source connects to. The host endpoint should look like the following: <code>{instance}.service-now.com.</code> </p>
    #[serde(rename = "HostUrl")]
    pub host_url: String,
    /// <p>Provides configuration information for crawling knowledge articles in the ServiceNow site.</p>
    #[serde(rename = "KnowledgeArticleConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub knowledge_article_configuration: Option<ServiceNowKnowledgeArticleConfiguration>,
    /// <p>The Amazon Resource Name (ARN) of the AWS Secret Manager secret that contains the user name and password required to connect to the ServiceNow instance.</p>
    #[serde(rename = "SecretArn")]
    pub secret_arn: String,
    /// <p>Provides configuration information for crawling service catalogs in the ServiceNow site.</p>
    #[serde(rename = "ServiceCatalogConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_catalog_configuration: Option<ServiceNowServiceCatalogConfiguration>,
    /// <p>The identifier of the release that the ServiceNow host is running. If the host is not running the <code>LONDON</code> release, use <code>OTHERS</code>.</p>
    #[serde(rename = "ServiceNowBuildVersion")]
    pub service_now_build_version: ServiceNowBuildVersionType,
}

/// <p>Provides configuration information for crawling knowledge articles in the ServiceNow site.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct ServiceNowKnowledgeArticleConfiguration {
    /// <p>Indicates whether Amazon Kendra should index attachments to knowledge articles.</p>
    #[serde(rename = "CrawlAttachments")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub crawl_attachments: Option<bool>,
    /// <p>The name of the ServiceNow field that is mapped to the index document contents field in the Amazon Kendra index.</p>
    #[serde(rename = "DocumentDataFieldName")]
    pub document_data_field_name: String,
    /// <p>The name of the ServiceNow field that is mapped to the index document title field.</p>
    #[serde(rename = "DocumentTitleFieldName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document_title_field_name: Option<String>,
    /// <p>List of regular expressions applied to knowledge articles. Items that don't match the inclusion pattern are not indexed. The regex is applied to the field specified in the <code>PatternTargetField</code> </p>
    #[serde(rename = "ExcludeAttachmentFilePatterns")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exclude_attachment_file_patterns: Option<Vec<String>>,
    /// <p>Mapping between ServiceNow fields and Amazon Kendra index fields. You must create the index field before you map the field.</p>
    #[serde(rename = "FieldMappings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub field_mappings: Option<Vec<DataSourceToIndexFieldMapping>>,
    /// <p>List of regular expressions applied to knowledge articles. Items that don't match the inclusion pattern are not indexed. The regex is applied to the field specified in the <code>PatternTargetField</code>.</p>
    #[serde(rename = "IncludeAttachmentFilePatterns")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_attachment_file_patterns: Option<Vec<String>>,
}

/// <p>Provides configuration information for crawling service catalog items in the ServiceNow site</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct ServiceNowServiceCatalogConfiguration {
    /// <p>Indicates whether Amazon Kendra should crawl attachments to the service catalog items. </p>
    #[serde(rename = "CrawlAttachments")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub crawl_attachments: Option<bool>,
    /// <p>The name of the ServiceNow field that is mapped to the index document contents field in the Amazon Kendra index.</p>
    #[serde(rename = "DocumentDataFieldName")]
    pub document_data_field_name: String,
    /// <p>The name of the ServiceNow field that is mapped to the index document title field.</p>
    #[serde(rename = "DocumentTitleFieldName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document_title_field_name: Option<String>,
    /// <p>Determines the types of file attachments that are excluded from the index.</p>
    #[serde(rename = "ExcludeAttachmentFilePatterns")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exclude_attachment_file_patterns: Option<Vec<String>>,
    /// <p>Mapping between ServiceNow fields and Amazon Kendra index fields. You must create the index field before you map the field.</p>
    #[serde(rename = "FieldMappings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub field_mappings: Option<Vec<DataSourceToIndexFieldMapping>>,
    /// <p>Determines the types of file attachments that are included in the index. </p>
    #[serde(rename = "IncludeAttachmentFilePatterns")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_attachment_file_patterns: Option<Vec<String>>,
}

/// <p>Provides configuration information for connecting to a Microsoft SharePoint data source.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct SharePointConfiguration {
    /// <p> <code>TRUE</code> to include attachments to documents stored in your Microsoft SharePoint site in the index; otherwise, <code>FALSE</code>.</p>
    #[serde(rename = "CrawlAttachments")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub crawl_attachments: Option<bool>,
    /// <p>A Boolean value that specifies whether local groups are disabled (<code>True</code>) or enabled (<code>False</code>). </p>
    #[serde(rename = "DisableLocalGroups")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_local_groups: Option<bool>,
    /// <p>The Microsoft SharePoint attribute field that contains the title of the document.</p>
    #[serde(rename = "DocumentTitleFieldName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document_title_field_name: Option<String>,
    /// <p>A list of regular expression patterns. Documents that match the patterns are excluded from the index. Documents that don't match the patterns are included in the index. If a document matches both an exclusion pattern and an inclusion pattern, the document is not included in the index.</p> <p>The regex is applied to the display URL of the SharePoint document.</p>
    #[serde(rename = "ExclusionPatterns")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exclusion_patterns: Option<Vec<String>>,
    /// <p>A list of <code>DataSourceToIndexFieldMapping</code> objects that map Microsoft SharePoint attributes to custom fields in the Amazon Kendra index. You must first create the index fields using the operation before you map SharePoint attributes. For more information, see <a href="https://docs.aws.amazon.com/kendra/latest/dg/field-mapping.html">Mapping Data Source Fields</a>.</p>
    #[serde(rename = "FieldMappings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub field_mappings: Option<Vec<DataSourceToIndexFieldMapping>>,
    /// <p>A list of regular expression patterns. Documents that match the patterns are included in the index. Documents that don't match the patterns are excluded from the index. If a document matches both an inclusion pattern and an exclusion pattern, the document is not included in the index.</p> <p>The regex is applied to the display URL of the SharePoint document.</p>
    #[serde(rename = "InclusionPatterns")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inclusion_patterns: Option<Vec<String>>,
    /// <p>The Amazon Resource Name (ARN) of credentials stored in AWS Secrets Manager. The credentials should be a user/password pair. For more information, see <a href="https://docs.aws.amazon.com/kendra/latest/dg/data-source-sharepoint.html">Using a Microsoft SharePoint Data Source</a>. For more information about AWS Secrets Manager, see <a href="https://docs.aws.amazon.com/secretsmanager/latest/userguide/intro.html"> What Is AWS Secrets Manager </a> in the <i>AWS Secrets Manager</i> user guide.</p>
    #[serde(rename = "SecretArn")]
    pub secret_arn: String,
    /// <p>The version of Microsoft SharePoint that you are using as a data source.</p>
    #[serde(rename = "SharePointVersion")]
    pub share_point_version: SharePointVersion,
    /// <p>The URLs of the Microsoft SharePoint site that contains the documents that should be indexed.</p>
    #[serde(rename = "Urls")]
    pub urls: Vec<String>,
    /// <p>Set to <code>TRUE</code> to use the Microsoft SharePoint change log to determine the documents that need to be updated in the index. Depending on the size of the SharePoint change log, it may take longer for Amazon Kendra to use the change log than it takes it to determine the changed documents using the Amazon Kendra document crawler.</p>
    #[serde(rename = "UseChangeLog")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_change_log: Option<bool>,
    #[serde(rename = "VpcConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_configuration: Option<DataSourceVpcConfiguration>,
}

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct UnknownSharePointVersion {
    name: String,
}

#[allow(non_camel_case_types)]
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
#[non_exhaustive]
pub enum SharePointVersion {
    SharepointOnline,
    #[doc(hidden)]
    UnknownVariant(UnknownSharePointVersion),
}

impl Default for SharePointVersion {
    fn default() -> Self {
        "".into()
    }
}

impl fmt::Display for SharePointVersion {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.into())
    }
}

impl rusoto_core::param::ToParam for SharePointVersion {
    fn to_param(&self) -> String {
        self.to_string()
    }
}

impl Into<String> for SharePointVersion {
    fn into(self) -> String {
        match self {
            SharePointVersion::SharepointOnline => "SHAREPOINT_ONLINE".to_string(),
            SharePointVersion::UnknownVariant(UnknownSharePointVersion { name: original }) => {
                original
            }
        }
    }
}

impl<'a> Into<&'a str> for &'a SharePointVersion {
    fn into(self) -> &'a str {
        match self {
            SharePointVersion::SharepointOnline => &"SHAREPOINT_ONLINE",
            SharePointVersion::UnknownVariant(UnknownSharePointVersion { name: original }) => {
                original
            }
        }
    }
}

impl From<&str> for SharePointVersion {
    fn from(name: &str) -> Self {
        match name {
            "SHAREPOINT_ONLINE" => SharePointVersion::SharepointOnline,
            _ => SharePointVersion::UnknownVariant(UnknownSharePointVersion {
                name: name.to_owned(),
            }),
        }
    }
}

impl From<String> for SharePointVersion {
    fn from(name: String) -> Self {
        match &*name {
            "SHAREPOINT_ONLINE" => SharePointVersion::SharepointOnline,
            _ => SharePointVersion::UnknownVariant(UnknownSharePointVersion { name }),
        }
    }
}

impl ::std::str::FromStr for SharePointVersion {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(s.into())
    }
}

impl Serialize for SharePointVersion {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.into())
    }
}

impl<'de> Deserialize<'de> for SharePointVersion {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Ok(String::deserialize(deserializer)?.into())
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct UnknownSortOrder {
    name: String,
}

#[allow(non_camel_case_types)]
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
#[non_exhaustive]
pub enum SortOrder {
    Asc,
    Desc,
    #[doc(hidden)]
    UnknownVariant(UnknownSortOrder),
}

impl Default for SortOrder {
    fn default() -> Self {
        "".into()
    }
}

impl fmt::Display for SortOrder {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.into())
    }
}

impl rusoto_core::param::ToParam for SortOrder {
    fn to_param(&self) -> String {
        self.to_string()
    }
}

impl Into<String> for SortOrder {
    fn into(self) -> String {
        match self {
            SortOrder::Asc => "ASC".to_string(),
            SortOrder::Desc => "DESC".to_string(),
            SortOrder::UnknownVariant(UnknownSortOrder { name: original }) => original,
        }
    }
}

impl<'a> Into<&'a str> for &'a SortOrder {
    fn into(self) -> &'a str {
        match self {
            SortOrder::Asc => &"ASC",
            SortOrder::Desc => &"DESC",
            SortOrder::UnknownVariant(UnknownSortOrder { name: original }) => original,
        }
    }
}

impl From<&str> for SortOrder {
    fn from(name: &str) -> Self {
        match name {
            "ASC" => SortOrder::Asc,
            "DESC" => SortOrder::Desc,
            _ => SortOrder::UnknownVariant(UnknownSortOrder {
                name: name.to_owned(),
            }),
        }
    }
}

impl From<String> for SortOrder {
    fn from(name: String) -> Self {
        match &*name {
            "ASC" => SortOrder::Asc,
            "DESC" => SortOrder::Desc,
            _ => SortOrder::UnknownVariant(UnknownSortOrder { name }),
        }
    }
}

impl ::std::str::FromStr for SortOrder {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(s.into())
    }
}

impl Serialize for SortOrder {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.into())
    }
}

#[cfg(feature = "deserialize_structs")]
impl<'de> Deserialize<'de> for SortOrder {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Ok(String::deserialize(deserializer)?.into())
    }
}

/// <p><p>Specifies the document attribute to use to sort the response to a Amazon Kendra query. You can specify a single attribute for sorting. The attribute must have the <code>Sortable</code> flag set to <code>true</code>, otherwise Amazon Kendra returns an exception.</p> <p>You can sort attributes of the following types.</p> <ul> <li> <p>Date value</p> </li> <li> <p>Long value</p> </li> <li> <p>String value</p> </li> </ul> <p>You can&#39;t sort attributes of the following type.</p> <ul> <li> <p>String list value</p> </li> </ul></p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct SortingConfiguration {
    /// <p><p>The name of the document attribute used to sort the response. You can use any field that has the <code>Sortable</code> flag set to true.</p> <p>You can also sort by any of the following built-in attributes:</p> <ul> <li> <p><em>category</p> </li> <li> <p></em>created<em>at</p> </li> <li> <p></em>last<em>updated</em>at</p> </li> <li> <p><em>version</p> </li> <li> <p></em>view_count</p> </li> </ul></p>
    #[serde(rename = "DocumentAttributeKey")]
    pub document_attribute_key: String,
    /// <p>The order that the results should be returned in. In case of ties, the relevance assigned to the result by Amazon Kendra is used as the tie-breaker.</p>
    #[serde(rename = "SortOrder")]
    pub sort_order: SortOrder,
}

/// <p>Provides information that configures Amazon Kendra to use a SQL database.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct SqlConfiguration {
    /// <p>Determines whether Amazon Kendra encloses SQL identifiers for tables and column names in double quotes (") when making a database query.</p> <p>By default, Amazon Kendra passes SQL identifiers the way that they are entered into the data source configuration. It does not change the case of identifiers or enclose them in quotes.</p> <p>PostgreSQL internally converts uppercase characters to lower case characters in identifiers unless they are quoted. Choosing this option encloses identifiers in quotes so that PostgreSQL does not convert the character's case.</p> <p>For MySQL databases, you must enable the <code>ansi_quotes</code> option when you set this field to <code>DOUBLE_QUOTES</code>.</p>
    #[serde(rename = "QueryIdentifiersEnclosingOption")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query_identifiers_enclosing_option: Option<QueryIdentifiersEnclosingOption>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct StartDataSourceSyncJobRequest {
    /// <p>The identifier of the data source to synchronize.</p>
    #[serde(rename = "Id")]
    pub id: String,
    /// <p>The identifier of the index that contains the data source.</p>
    #[serde(rename = "IndexId")]
    pub index_id: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct StartDataSourceSyncJobResponse {
    /// <p>Identifies a particular synchronization job.</p>
    #[serde(rename = "ExecutionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub execution_id: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct StopDataSourceSyncJobRequest {
    /// <p>The identifier of the data source for which to stop the synchronization jobs.</p>
    #[serde(rename = "Id")]
    pub id: String,
    /// <p>The identifier of the index that contains the data source.</p>
    #[serde(rename = "IndexId")]
    pub index_id: String,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
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

/// <p>A list of key/value pairs that identify an index, FAQ, or data source. Tag keys and values can consist of Unicode letters, digits, white space, and any of the following symbols: _ . : / = + - @.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct Tag {
    /// <p>The key for the tag. Keys are not case sensitive and must be unique for the index, FAQ, or data source.</p>
    #[serde(rename = "Key")]
    pub key: String,
    /// <p>The value associated with the tag. The value may be an empty string but it can't be null.</p>
    #[serde(rename = "Value")]
    pub value: String,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct TagResourceRequest {
    /// <p>The Amazon Resource Name (ARN) of the index, FAQ, or data source to tag.</p>
    #[serde(rename = "ResourceARN")]
    pub resource_arn: String,
    /// <p>A list of tag keys to add to the index, FAQ, or data source. If a tag already exists, the existing value is replaced with the new value.</p>
    #[serde(rename = "Tags")]
    pub tags: Vec<Tag>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct TagResourceResponse {}

/// <p>Provides information about text documents indexed in an index.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct TextDocumentStatistics {
    /// <p>The total size, in bytes, of the indexed documents.</p>
    #[serde(rename = "IndexedTextBytes")]
    pub indexed_text_bytes: i64,
    /// <p>The number of text documents indexed.</p>
    #[serde(rename = "IndexedTextDocumentsCount")]
    pub indexed_text_documents_count: i64,
}

/// <p>Provides text and information about where to highlight the text.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
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

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct UnknownThesaurusStatus {
    name: String,
}

#[allow(non_camel_case_types)]
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
#[non_exhaustive]
pub enum ThesaurusStatus {
    Active,
    ActiveButUpdateFailed,
    Creating,
    Deleting,
    Failed,
    Updating,
    #[doc(hidden)]
    UnknownVariant(UnknownThesaurusStatus),
}

impl Default for ThesaurusStatus {
    fn default() -> Self {
        "".into()
    }
}

impl fmt::Display for ThesaurusStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.into())
    }
}

impl rusoto_core::param::ToParam for ThesaurusStatus {
    fn to_param(&self) -> String {
        self.to_string()
    }
}

impl Into<String> for ThesaurusStatus {
    fn into(self) -> String {
        match self {
            ThesaurusStatus::Active => "ACTIVE".to_string(),
            ThesaurusStatus::ActiveButUpdateFailed => "ACTIVE_BUT_UPDATE_FAILED".to_string(),
            ThesaurusStatus::Creating => "CREATING".to_string(),
            ThesaurusStatus::Deleting => "DELETING".to_string(),
            ThesaurusStatus::Failed => "FAILED".to_string(),
            ThesaurusStatus::Updating => "UPDATING".to_string(),
            ThesaurusStatus::UnknownVariant(UnknownThesaurusStatus { name: original }) => original,
        }
    }
}

impl<'a> Into<&'a str> for &'a ThesaurusStatus {
    fn into(self) -> &'a str {
        match self {
            ThesaurusStatus::Active => &"ACTIVE",
            ThesaurusStatus::ActiveButUpdateFailed => &"ACTIVE_BUT_UPDATE_FAILED",
            ThesaurusStatus::Creating => &"CREATING",
            ThesaurusStatus::Deleting => &"DELETING",
            ThesaurusStatus::Failed => &"FAILED",
            ThesaurusStatus::Updating => &"UPDATING",
            ThesaurusStatus::UnknownVariant(UnknownThesaurusStatus { name: original }) => original,
        }
    }
}

impl From<&str> for ThesaurusStatus {
    fn from(name: &str) -> Self {
        match name {
            "ACTIVE" => ThesaurusStatus::Active,
            "ACTIVE_BUT_UPDATE_FAILED" => ThesaurusStatus::ActiveButUpdateFailed,
            "CREATING" => ThesaurusStatus::Creating,
            "DELETING" => ThesaurusStatus::Deleting,
            "FAILED" => ThesaurusStatus::Failed,
            "UPDATING" => ThesaurusStatus::Updating,
            _ => ThesaurusStatus::UnknownVariant(UnknownThesaurusStatus {
                name: name.to_owned(),
            }),
        }
    }
}

impl From<String> for ThesaurusStatus {
    fn from(name: String) -> Self {
        match &*name {
            "ACTIVE" => ThesaurusStatus::Active,
            "ACTIVE_BUT_UPDATE_FAILED" => ThesaurusStatus::ActiveButUpdateFailed,
            "CREATING" => ThesaurusStatus::Creating,
            "DELETING" => ThesaurusStatus::Deleting,
            "FAILED" => ThesaurusStatus::Failed,
            "UPDATING" => ThesaurusStatus::Updating,
            _ => ThesaurusStatus::UnknownVariant(UnknownThesaurusStatus { name }),
        }
    }
}

impl ::std::str::FromStr for ThesaurusStatus {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(s.into())
    }
}

#[cfg(any(test, feature = "serialize_structs"))]
impl Serialize for ThesaurusStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.into())
    }
}

impl<'de> Deserialize<'de> for ThesaurusStatus {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Ok(String::deserialize(deserializer)?.into())
    }
}

/// <p>An array of summary information for one or more thesauruses.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ThesaurusSummary {
    /// <p>The Unix datetime that the thesaurus was created.</p>
    #[serde(rename = "CreatedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<f64>,
    /// <p>The identifier of the thesaurus.</p>
    #[serde(rename = "Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// <p>The name of the thesaurus.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The status of the thesaurus.</p>
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<ThesaurusStatus>,
    /// <p>The Unix datetime that the thesaurus was last updated.</p>
    #[serde(rename = "UpdatedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<f64>,
}

/// <p>Provides a range of time.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
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

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UntagResourceRequest {
    /// <p>The Amazon Resource Name (ARN) of the index, FAQ, or data source to remove the tag from.</p>
    #[serde(rename = "ResourceARN")]
    pub resource_arn: String,
    /// <p>A list of tag keys to remove from the index, FAQ, or data source. If a tag key does not exist on the resource, it is ignored.</p>
    #[serde(rename = "TagKeys")]
    pub tag_keys: Vec<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UntagResourceResponse {}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
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

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UpdateIndexRequest {
    /// <p>Sets the number of addtional storage and query capacity units that should be used by the index. You can change the capacity of the index up to 5 times per day.</p> <p>If you are using extra storage units, you can't reduce the storage capacity below that required to meet the storage needs for your index.</p>
    #[serde(rename = "CapacityUnits")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capacity_units: Option<CapacityUnitsConfiguration>,
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
    /// <p>The user user token context policy.</p>
    #[serde(rename = "UserContextPolicy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_context_policy: Option<UserContextPolicy>,
    /// <p>The user token configuration.</p>
    #[serde(rename = "UserTokenConfigurations")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_token_configurations: Option<Vec<UserTokenConfiguration>>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UpdateThesaurusRequest {
    /// <p>The updated description of the thesaurus.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The identifier of the thesaurus to update.</p>
    #[serde(rename = "Id")]
    pub id: String,
    /// <p>The identifier of the index associated with the thesaurus to update.</p>
    #[serde(rename = "IndexId")]
    pub index_id: String,
    /// <p>The updated name of the thesaurus.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The updated role ARN of the thesaurus.</p>
    #[serde(rename = "RoleArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_arn: Option<String>,
    #[serde(rename = "SourceS3Path")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_s3_path: Option<S3Path>,
}

/// <p>Provides information about the user context for a Amazon Kendra index.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UserContext {
    /// <p>The user context token. It must be a JWT or a JSON token.</p>
    #[serde(rename = "Token")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token: Option<String>,
}

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct UnknownUserContextPolicy {
    name: String,
}

#[allow(non_camel_case_types)]
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
#[non_exhaustive]
pub enum UserContextPolicy {
    AttributeFilter,
    UserToken,
    #[doc(hidden)]
    UnknownVariant(UnknownUserContextPolicy),
}

impl Default for UserContextPolicy {
    fn default() -> Self {
        "".into()
    }
}

impl fmt::Display for UserContextPolicy {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.into())
    }
}

impl rusoto_core::param::ToParam for UserContextPolicy {
    fn to_param(&self) -> String {
        self.to_string()
    }
}

impl Into<String> for UserContextPolicy {
    fn into(self) -> String {
        match self {
            UserContextPolicy::AttributeFilter => "ATTRIBUTE_FILTER".to_string(),
            UserContextPolicy::UserToken => "USER_TOKEN".to_string(),
            UserContextPolicy::UnknownVariant(UnknownUserContextPolicy { name: original }) => {
                original
            }
        }
    }
}

impl<'a> Into<&'a str> for &'a UserContextPolicy {
    fn into(self) -> &'a str {
        match self {
            UserContextPolicy::AttributeFilter => &"ATTRIBUTE_FILTER",
            UserContextPolicy::UserToken => &"USER_TOKEN",
            UserContextPolicy::UnknownVariant(UnknownUserContextPolicy { name: original }) => {
                original
            }
        }
    }
}

impl From<&str> for UserContextPolicy {
    fn from(name: &str) -> Self {
        match name {
            "ATTRIBUTE_FILTER" => UserContextPolicy::AttributeFilter,
            "USER_TOKEN" => UserContextPolicy::UserToken,
            _ => UserContextPolicy::UnknownVariant(UnknownUserContextPolicy {
                name: name.to_owned(),
            }),
        }
    }
}

impl From<String> for UserContextPolicy {
    fn from(name: String) -> Self {
        match &*name {
            "ATTRIBUTE_FILTER" => UserContextPolicy::AttributeFilter,
            "USER_TOKEN" => UserContextPolicy::UserToken,
            _ => UserContextPolicy::UnknownVariant(UnknownUserContextPolicy { name }),
        }
    }
}

impl ::std::str::FromStr for UserContextPolicy {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(s.into())
    }
}

impl Serialize for UserContextPolicy {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.into())
    }
}

impl<'de> Deserialize<'de> for UserContextPolicy {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Ok(String::deserialize(deserializer)?.into())
    }
}

/// <p>Provides configuration information for a token configuration.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct UserTokenConfiguration {
    /// <p>Information about the JSON token type configuration.</p>
    #[serde(rename = "JsonTokenTypeConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub json_token_type_configuration: Option<JsonTokenTypeConfiguration>,
    /// <p>Information about the JWT token type configuration.</p>
    #[serde(rename = "JwtTokenTypeConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub jwt_token_type_configuration: Option<JwtTokenTypeConfiguration>,
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
    Conflict(String),
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
                "ConflictException" => {
                    return RusotoError::Service(CreateIndexError::Conflict(err.msg))
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
            CreateIndexError::Conflict(ref cause) => write!(f, "{}", cause),
            CreateIndexError::InternalServer(ref cause) => write!(f, "{}", cause),
            CreateIndexError::ResourceAlreadyExist(ref cause) => write!(f, "{}", cause),
            CreateIndexError::ServiceQuotaExceeded(ref cause) => write!(f, "{}", cause),
            CreateIndexError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CreateIndexError {}
/// Errors returned by CreateThesaurus
#[derive(Debug, PartialEq)]
pub enum CreateThesaurusError {
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

impl CreateThesaurusError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateThesaurusError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(CreateThesaurusError::AccessDenied(err.msg))
                }
                "ConflictException" => {
                    return RusotoError::Service(CreateThesaurusError::Conflict(err.msg))
                }
                "InternalServerException" => {
                    return RusotoError::Service(CreateThesaurusError::InternalServer(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(CreateThesaurusError::ResourceNotFound(err.msg))
                }
                "ServiceQuotaExceededException" => {
                    return RusotoError::Service(CreateThesaurusError::ServiceQuotaExceeded(
                        err.msg,
                    ))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(CreateThesaurusError::Throttling(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for CreateThesaurusError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreateThesaurusError::AccessDenied(ref cause) => write!(f, "{}", cause),
            CreateThesaurusError::Conflict(ref cause) => write!(f, "{}", cause),
            CreateThesaurusError::InternalServer(ref cause) => write!(f, "{}", cause),
            CreateThesaurusError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            CreateThesaurusError::ServiceQuotaExceeded(ref cause) => write!(f, "{}", cause),
            CreateThesaurusError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CreateThesaurusError {}
/// Errors returned by DeleteDataSource
#[derive(Debug, PartialEq)]
pub enum DeleteDataSourceError {
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

impl DeleteDataSourceError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteDataSourceError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(DeleteDataSourceError::AccessDenied(err.msg))
                }
                "ConflictException" => {
                    return RusotoError::Service(DeleteDataSourceError::Conflict(err.msg))
                }
                "InternalServerException" => {
                    return RusotoError::Service(DeleteDataSourceError::InternalServer(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(DeleteDataSourceError::ResourceNotFound(err.msg))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(DeleteDataSourceError::Throttling(err.msg))
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
            DeleteDataSourceError::AccessDenied(ref cause) => write!(f, "{}", cause),
            DeleteDataSourceError::Conflict(ref cause) => write!(f, "{}", cause),
            DeleteDataSourceError::InternalServer(ref cause) => write!(f, "{}", cause),
            DeleteDataSourceError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            DeleteDataSourceError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeleteDataSourceError {}
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
/// Errors returned by DeleteThesaurus
#[derive(Debug, PartialEq)]
pub enum DeleteThesaurusError {
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

impl DeleteThesaurusError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteThesaurusError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(DeleteThesaurusError::AccessDenied(err.msg))
                }
                "ConflictException" => {
                    return RusotoError::Service(DeleteThesaurusError::Conflict(err.msg))
                }
                "InternalServerException" => {
                    return RusotoError::Service(DeleteThesaurusError::InternalServer(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(DeleteThesaurusError::ResourceNotFound(err.msg))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(DeleteThesaurusError::Throttling(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DeleteThesaurusError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteThesaurusError::AccessDenied(ref cause) => write!(f, "{}", cause),
            DeleteThesaurusError::Conflict(ref cause) => write!(f, "{}", cause),
            DeleteThesaurusError::InternalServer(ref cause) => write!(f, "{}", cause),
            DeleteThesaurusError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            DeleteThesaurusError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeleteThesaurusError {}
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
/// Errors returned by DescribeThesaurus
#[derive(Debug, PartialEq)]
pub enum DescribeThesaurusError {
    /// <p><p/></p>
    AccessDenied(String),
    /// <p><p/></p>
    InternalServer(String),
    /// <p><p/></p>
    ResourceNotFound(String),
    /// <p><p/></p>
    Throttling(String),
}

impl DescribeThesaurusError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeThesaurusError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(DescribeThesaurusError::AccessDenied(err.msg))
                }
                "InternalServerException" => {
                    return RusotoError::Service(DescribeThesaurusError::InternalServer(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(DescribeThesaurusError::ResourceNotFound(err.msg))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(DescribeThesaurusError::Throttling(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DescribeThesaurusError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeThesaurusError::AccessDenied(ref cause) => write!(f, "{}", cause),
            DescribeThesaurusError::InternalServer(ref cause) => write!(f, "{}", cause),
            DescribeThesaurusError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            DescribeThesaurusError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DescribeThesaurusError {}
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
/// Errors returned by ListTagsForResource
#[derive(Debug, PartialEq)]
pub enum ListTagsForResourceError {
    /// <p><p/></p>
    AccessDenied(String),
    /// <p><p/></p>
    InternalServer(String),
    /// <p><p/></p>
    ResourceUnavailable(String),
    /// <p><p/></p>
    Throttling(String),
}

impl ListTagsForResourceError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListTagsForResourceError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(ListTagsForResourceError::AccessDenied(err.msg))
                }
                "InternalServerException" => {
                    return RusotoError::Service(ListTagsForResourceError::InternalServer(err.msg))
                }
                "ResourceUnavailableException" => {
                    return RusotoError::Service(ListTagsForResourceError::ResourceUnavailable(
                        err.msg,
                    ))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(ListTagsForResourceError::Throttling(err.msg))
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
            ListTagsForResourceError::AccessDenied(ref cause) => write!(f, "{}", cause),
            ListTagsForResourceError::InternalServer(ref cause) => write!(f, "{}", cause),
            ListTagsForResourceError::ResourceUnavailable(ref cause) => write!(f, "{}", cause),
            ListTagsForResourceError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListTagsForResourceError {}
/// Errors returned by ListThesauri
#[derive(Debug, PartialEq)]
pub enum ListThesauriError {
    /// <p><p/></p>
    AccessDenied(String),
    /// <p><p/></p>
    InternalServer(String),
    /// <p><p/></p>
    ResourceNotFound(String),
    /// <p><p/></p>
    Throttling(String),
}

impl ListThesauriError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListThesauriError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(ListThesauriError::AccessDenied(err.msg))
                }
                "InternalServerException" => {
                    return RusotoError::Service(ListThesauriError::InternalServer(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(ListThesauriError::ResourceNotFound(err.msg))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(ListThesauriError::Throttling(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ListThesauriError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListThesauriError::AccessDenied(ref cause) => write!(f, "{}", cause),
            ListThesauriError::InternalServer(ref cause) => write!(f, "{}", cause),
            ListThesauriError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            ListThesauriError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListThesauriError {}
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
    ServiceQuotaExceeded(String),
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
                "ServiceQuotaExceededException" => {
                    return RusotoError::Service(QueryError::ServiceQuotaExceeded(err.msg))
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
            QueryError::ServiceQuotaExceeded(ref cause) => write!(f, "{}", cause),
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
/// Errors returned by TagResource
#[derive(Debug, PartialEq)]
pub enum TagResourceError {
    /// <p><p/></p>
    AccessDenied(String),
    /// <p><p/></p>
    InternalServer(String),
    /// <p><p/></p>
    ResourceUnavailable(String),
    /// <p><p/></p>
    Throttling(String),
}

impl TagResourceError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<TagResourceError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(TagResourceError::AccessDenied(err.msg))
                }
                "InternalServerException" => {
                    return RusotoError::Service(TagResourceError::InternalServer(err.msg))
                }
                "ResourceUnavailableException" => {
                    return RusotoError::Service(TagResourceError::ResourceUnavailable(err.msg))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(TagResourceError::Throttling(err.msg))
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
            TagResourceError::AccessDenied(ref cause) => write!(f, "{}", cause),
            TagResourceError::InternalServer(ref cause) => write!(f, "{}", cause),
            TagResourceError::ResourceUnavailable(ref cause) => write!(f, "{}", cause),
            TagResourceError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for TagResourceError {}
/// Errors returned by UntagResource
#[derive(Debug, PartialEq)]
pub enum UntagResourceError {
    /// <p><p/></p>
    AccessDenied(String),
    /// <p><p/></p>
    InternalServer(String),
    /// <p><p/></p>
    ResourceUnavailable(String),
    /// <p><p/></p>
    Throttling(String),
}

impl UntagResourceError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UntagResourceError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(UntagResourceError::AccessDenied(err.msg))
                }
                "InternalServerException" => {
                    return RusotoError::Service(UntagResourceError::InternalServer(err.msg))
                }
                "ResourceUnavailableException" => {
                    return RusotoError::Service(UntagResourceError::ResourceUnavailable(err.msg))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(UntagResourceError::Throttling(err.msg))
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
            UntagResourceError::AccessDenied(ref cause) => write!(f, "{}", cause),
            UntagResourceError::InternalServer(ref cause) => write!(f, "{}", cause),
            UntagResourceError::ResourceUnavailable(ref cause) => write!(f, "{}", cause),
            UntagResourceError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for UntagResourceError {}
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
    ServiceQuotaExceeded(String),
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
                "ServiceQuotaExceededException" => {
                    return RusotoError::Service(UpdateIndexError::ServiceQuotaExceeded(err.msg))
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
            UpdateIndexError::ServiceQuotaExceeded(ref cause) => write!(f, "{}", cause),
            UpdateIndexError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for UpdateIndexError {}
/// Errors returned by UpdateThesaurus
#[derive(Debug, PartialEq)]
pub enum UpdateThesaurusError {
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

impl UpdateThesaurusError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdateThesaurusError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(UpdateThesaurusError::AccessDenied(err.msg))
                }
                "ConflictException" => {
                    return RusotoError::Service(UpdateThesaurusError::Conflict(err.msg))
                }
                "InternalServerException" => {
                    return RusotoError::Service(UpdateThesaurusError::InternalServer(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(UpdateThesaurusError::ResourceNotFound(err.msg))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(UpdateThesaurusError::Throttling(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for UpdateThesaurusError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UpdateThesaurusError::AccessDenied(ref cause) => write!(f, "{}", cause),
            UpdateThesaurusError::Conflict(ref cause) => write!(f, "{}", cause),
            UpdateThesaurusError::InternalServer(ref cause) => write!(f, "{}", cause),
            UpdateThesaurusError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            UpdateThesaurusError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for UpdateThesaurusError {}
/// Trait representing the capabilities of the kendra API. kendra clients implement this trait.
#[async_trait]
pub trait Kendra {
    /// <p>Removes one or more documents from an index. The documents must have been added with the <a>BatchPutDocument</a> operation.</p> <p>The documents are deleted asynchronously. You can see the progress of the deletion by using AWS CloudWatch. Any error messages releated to the processing of the batch are sent to you CloudWatch log.</p>
    async fn batch_delete_document(
        &self,
        input: BatchDeleteDocumentRequest,
    ) -> Result<BatchDeleteDocumentResponse, RusotoError<BatchDeleteDocumentError>>;

    /// <p>Adds one or more documents to an index.</p> <p>The <code>BatchPutDocument</code> operation enables you to ingest inline documents or a set of documents stored in an Amazon S3 bucket. Use this operation to ingest your text and unstructured text into an index, add custom attributes to the documents, and to attach an access control list to the documents added to the index.</p> <p>The documents are indexed asynchronously. You can see the progress of the batch using AWS CloudWatch. Any error messages related to processing the batch are sent to your AWS CloudWatch log.</p>
    async fn batch_put_document(
        &self,
        input: BatchPutDocumentRequest,
    ) -> Result<BatchPutDocumentResponse, RusotoError<BatchPutDocumentError>>;

    /// <p>Creates a data source that you use to with an Amazon Kendra index. </p> <p>You specify a name, data source connector type and description for your data source. You also specify configuration information such as document metadata (author, source URI, and so on) and user context information.</p> <p> <code>CreateDataSource</code> is a synchronous operation. The operation returns 200 if the data source was successfully created. Otherwise, an exception is raised.</p>
    async fn create_data_source(
        &self,
        input: CreateDataSourceRequest,
    ) -> Result<CreateDataSourceResponse, RusotoError<CreateDataSourceError>>;

    /// <p>Creates an new set of frequently asked question (FAQ) questions and answers.</p>
    async fn create_faq(
        &self,
        input: CreateFaqRequest,
    ) -> Result<CreateFaqResponse, RusotoError<CreateFaqError>>;

    /// <p>Creates a new Amazon Kendra index. Index creation is an asynchronous operation. To determine if index creation has completed, check the <code>Status</code> field returned from a call to . The <code>Status</code> field is set to <code>ACTIVE</code> when the index is ready to use.</p> <p>Once the index is active you can index your documents using the operation or using one of the supported data sources. </p>
    async fn create_index(
        &self,
        input: CreateIndexRequest,
    ) -> Result<CreateIndexResponse, RusotoError<CreateIndexError>>;

    /// <p>Creates a thesaurus for an index. The thesaurus contains a list of synonyms in Solr format.</p>
    async fn create_thesaurus(
        &self,
        input: CreateThesaurusRequest,
    ) -> Result<CreateThesaurusResponse, RusotoError<CreateThesaurusError>>;

    /// <p>Deletes an Amazon Kendra data source. An exception is not thrown if the data source is already being deleted. While the data source is being deleted, the <code>Status</code> field returned by a call to the operation is set to <code>DELETING</code>. For more information, see <a href="https://docs.aws.amazon.com/kendra/latest/dg/delete-data-source.html">Deleting Data Sources</a>.</p>
    async fn delete_data_source(
        &self,
        input: DeleteDataSourceRequest,
    ) -> Result<(), RusotoError<DeleteDataSourceError>>;

    /// <p>Removes an FAQ from an index.</p>
    async fn delete_faq(&self, input: DeleteFaqRequest) -> Result<(), RusotoError<DeleteFaqError>>;

    /// <p>Deletes an existing Amazon Kendra index. An exception is not thrown if the index is already being deleted. While the index is being deleted, the <code>Status</code> field returned by a call to the <a>DescribeIndex</a> operation is set to <code>DELETING</code>.</p>
    async fn delete_index(
        &self,
        input: DeleteIndexRequest,
    ) -> Result<(), RusotoError<DeleteIndexError>>;

    /// <p>Deletes an existing Amazon Kendra thesaurus. </p>
    async fn delete_thesaurus(
        &self,
        input: DeleteThesaurusRequest,
    ) -> Result<(), RusotoError<DeleteThesaurusError>>;

    /// <p>Gets information about a Amazon Kendra data source.</p>
    async fn describe_data_source(
        &self,
        input: DescribeDataSourceRequest,
    ) -> Result<DescribeDataSourceResponse, RusotoError<DescribeDataSourceError>>;

    /// <p>Gets information about an FAQ list.</p>
    async fn describe_faq(
        &self,
        input: DescribeFaqRequest,
    ) -> Result<DescribeFaqResponse, RusotoError<DescribeFaqError>>;

    /// <p>Describes an existing Amazon Kendra index</p>
    async fn describe_index(
        &self,
        input: DescribeIndexRequest,
    ) -> Result<DescribeIndexResponse, RusotoError<DescribeIndexError>>;

    /// <p>Describes an existing Amazon Kendra thesaurus.</p>
    async fn describe_thesaurus(
        &self,
        input: DescribeThesaurusRequest,
    ) -> Result<DescribeThesaurusResponse, RusotoError<DescribeThesaurusError>>;

    /// <p>Gets statistics about synchronizing Amazon Kendra with a data source.</p>
    async fn list_data_source_sync_jobs(
        &self,
        input: ListDataSourceSyncJobsRequest,
    ) -> Result<ListDataSourceSyncJobsResponse, RusotoError<ListDataSourceSyncJobsError>>;

    /// <p>Lists the data sources that you have created.</p>
    async fn list_data_sources(
        &self,
        input: ListDataSourcesRequest,
    ) -> Result<ListDataSourcesResponse, RusotoError<ListDataSourcesError>>;

    /// <p>Gets a list of FAQ lists associated with an index.</p>
    async fn list_faqs(
        &self,
        input: ListFaqsRequest,
    ) -> Result<ListFaqsResponse, RusotoError<ListFaqsError>>;

    /// <p>Lists the Amazon Kendra indexes that you have created.</p>
    async fn list_indices(
        &self,
        input: ListIndicesRequest,
    ) -> Result<ListIndicesResponse, RusotoError<ListIndicesError>>;

    /// <p>Gets a list of tags associated with a specified resource. Indexes, FAQs, and data sources can have tags associated with them.</p>
    async fn list_tags_for_resource(
        &self,
        input: ListTagsForResourceRequest,
    ) -> Result<ListTagsForResourceResponse, RusotoError<ListTagsForResourceError>>;

    /// <p>Lists the Amazon Kendra thesauri associated with an index.</p>
    async fn list_thesauri(
        &self,
        input: ListThesauriRequest,
    ) -> Result<ListThesauriResponse, RusotoError<ListThesauriError>>;

    /// <p>Searches an active index. Use this API to search your documents using query. The <code>Query</code> operation enables to do faceted search and to filter results based on document attributes.</p> <p>It also enables you to provide user context that Amazon Kendra uses to enforce document access control in the search results. </p> <p>Amazon Kendra searches your index for text content and question and answer (FAQ) content. By default the response contains three types of results.</p> <ul> <li> <p>Relevant passages</p> </li> <li> <p>Matching FAQs</p> </li> <li> <p>Relevant documents</p> </li> </ul> <p>You can specify that the query return only one type of result using the <code>QueryResultTypeConfig</code> parameter.</p> <p>Each query returns the 100 most relevant results. </p>
    async fn query(&self, input: QueryRequest) -> Result<QueryResult, RusotoError<QueryError>>;

    /// <p>Starts a synchronization job for a data source. If a synchronization job is already in progress, Amazon Kendra returns a <code>ResourceInUseException</code> exception.</p>
    async fn start_data_source_sync_job(
        &self,
        input: StartDataSourceSyncJobRequest,
    ) -> Result<StartDataSourceSyncJobResponse, RusotoError<StartDataSourceSyncJobError>>;

    /// <p>Stops a running synchronization job. You can't stop a scheduled synchronization job.</p>
    async fn stop_data_source_sync_job(
        &self,
        input: StopDataSourceSyncJobRequest,
    ) -> Result<(), RusotoError<StopDataSourceSyncJobError>>;

    /// <p>Enables you to provide feedback to Amazon Kendra to improve the performance of the service. </p>
    async fn submit_feedback(
        &self,
        input: SubmitFeedbackRequest,
    ) -> Result<(), RusotoError<SubmitFeedbackError>>;

    /// <p>Adds the specified tag to the specified index, FAQ, or data source resource. If the tag already exists, the existing value is replaced with the new value.</p>
    async fn tag_resource(
        &self,
        input: TagResourceRequest,
    ) -> Result<TagResourceResponse, RusotoError<TagResourceError>>;

    /// <p>Removes a tag from an index, FAQ, or a data source.</p>
    async fn untag_resource(
        &self,
        input: UntagResourceRequest,
    ) -> Result<UntagResourceResponse, RusotoError<UntagResourceError>>;

    /// <p>Updates an existing Amazon Kendra data source.</p>
    async fn update_data_source(
        &self,
        input: UpdateDataSourceRequest,
    ) -> Result<(), RusotoError<UpdateDataSourceError>>;

    /// <p>Updates an existing Amazon Kendra index.</p>
    async fn update_index(
        &self,
        input: UpdateIndexRequest,
    ) -> Result<(), RusotoError<UpdateIndexError>>;

    /// <p>Updates a thesaurus file associated with an index.</p>
    async fn update_thesaurus(
        &self,
        input: UpdateThesaurusRequest,
    ) -> Result<(), RusotoError<UpdateThesaurusError>>;
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

#[async_trait]
impl Kendra for KendraClient {
    /// <p>Removes one or more documents from an index. The documents must have been added with the <a>BatchPutDocument</a> operation.</p> <p>The documents are deleted asynchronously. You can see the progress of the deletion by using AWS CloudWatch. Any error messages releated to the processing of the batch are sent to you CloudWatch log.</p>
    async fn batch_delete_document(
        &self,
        input: BatchDeleteDocumentRequest,
    ) -> Result<BatchDeleteDocumentResponse, RusotoError<BatchDeleteDocumentError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "AWSKendraFrontendService.BatchDeleteDocument",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, BatchDeleteDocumentError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<BatchDeleteDocumentResponse, _>()
    }

    /// <p>Adds one or more documents to an index.</p> <p>The <code>BatchPutDocument</code> operation enables you to ingest inline documents or a set of documents stored in an Amazon S3 bucket. Use this operation to ingest your text and unstructured text into an index, add custom attributes to the documents, and to attach an access control list to the documents added to the index.</p> <p>The documents are indexed asynchronously. You can see the progress of the batch using AWS CloudWatch. Any error messages related to processing the batch are sent to your AWS CloudWatch log.</p>
    async fn batch_put_document(
        &self,
        input: BatchPutDocumentRequest,
    ) -> Result<BatchPutDocumentResponse, RusotoError<BatchPutDocumentError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "AWSKendraFrontendService.BatchPutDocument");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, BatchPutDocumentError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<BatchPutDocumentResponse, _>()
    }

    /// <p>Creates a data source that you use to with an Amazon Kendra index. </p> <p>You specify a name, data source connector type and description for your data source. You also specify configuration information such as document metadata (author, source URI, and so on) and user context information.</p> <p> <code>CreateDataSource</code> is a synchronous operation. The operation returns 200 if the data source was successfully created. Otherwise, an exception is raised.</p>
    async fn create_data_source(
        &self,
        input: CreateDataSourceRequest,
    ) -> Result<CreateDataSourceResponse, RusotoError<CreateDataSourceError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "AWSKendraFrontendService.CreateDataSource");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, CreateDataSourceError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<CreateDataSourceResponse, _>()
    }

    /// <p>Creates an new set of frequently asked question (FAQ) questions and answers.</p>
    async fn create_faq(
        &self,
        input: CreateFaqRequest,
    ) -> Result<CreateFaqResponse, RusotoError<CreateFaqError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "AWSKendraFrontendService.CreateFaq");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, CreateFaqError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<CreateFaqResponse, _>()
    }

    /// <p>Creates a new Amazon Kendra index. Index creation is an asynchronous operation. To determine if index creation has completed, check the <code>Status</code> field returned from a call to . The <code>Status</code> field is set to <code>ACTIVE</code> when the index is ready to use.</p> <p>Once the index is active you can index your documents using the operation or using one of the supported data sources. </p>
    async fn create_index(
        &self,
        input: CreateIndexRequest,
    ) -> Result<CreateIndexResponse, RusotoError<CreateIndexError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "AWSKendraFrontendService.CreateIndex");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, CreateIndexError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<CreateIndexResponse, _>()
    }

    /// <p>Creates a thesaurus for an index. The thesaurus contains a list of synonyms in Solr format.</p>
    async fn create_thesaurus(
        &self,
        input: CreateThesaurusRequest,
    ) -> Result<CreateThesaurusResponse, RusotoError<CreateThesaurusError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "AWSKendraFrontendService.CreateThesaurus");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, CreateThesaurusError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<CreateThesaurusResponse, _>()
    }

    /// <p>Deletes an Amazon Kendra data source. An exception is not thrown if the data source is already being deleted. While the data source is being deleted, the <code>Status</code> field returned by a call to the operation is set to <code>DELETING</code>. For more information, see <a href="https://docs.aws.amazon.com/kendra/latest/dg/delete-data-source.html">Deleting Data Sources</a>.</p>
    async fn delete_data_source(
        &self,
        input: DeleteDataSourceRequest,
    ) -> Result<(), RusotoError<DeleteDataSourceError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "AWSKendraFrontendService.DeleteDataSource");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, DeleteDataSourceError::from_response)
            .await?;
        std::mem::drop(response);
        Ok(())
    }

    /// <p>Removes an FAQ from an index.</p>
    async fn delete_faq(&self, input: DeleteFaqRequest) -> Result<(), RusotoError<DeleteFaqError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "AWSKendraFrontendService.DeleteFaq");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, DeleteFaqError::from_response)
            .await?;
        std::mem::drop(response);
        Ok(())
    }

    /// <p>Deletes an existing Amazon Kendra index. An exception is not thrown if the index is already being deleted. While the index is being deleted, the <code>Status</code> field returned by a call to the <a>DescribeIndex</a> operation is set to <code>DELETING</code>.</p>
    async fn delete_index(
        &self,
        input: DeleteIndexRequest,
    ) -> Result<(), RusotoError<DeleteIndexError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "AWSKendraFrontendService.DeleteIndex");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, DeleteIndexError::from_response)
            .await?;
        std::mem::drop(response);
        Ok(())
    }

    /// <p>Deletes an existing Amazon Kendra thesaurus. </p>
    async fn delete_thesaurus(
        &self,
        input: DeleteThesaurusRequest,
    ) -> Result<(), RusotoError<DeleteThesaurusError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "AWSKendraFrontendService.DeleteThesaurus");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, DeleteThesaurusError::from_response)
            .await?;
        std::mem::drop(response);
        Ok(())
    }

    /// <p>Gets information about a Amazon Kendra data source.</p>
    async fn describe_data_source(
        &self,
        input: DescribeDataSourceRequest,
    ) -> Result<DescribeDataSourceResponse, RusotoError<DescribeDataSourceError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "AWSKendraFrontendService.DescribeDataSource",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, DescribeDataSourceError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<DescribeDataSourceResponse, _>()
    }

    /// <p>Gets information about an FAQ list.</p>
    async fn describe_faq(
        &self,
        input: DescribeFaqRequest,
    ) -> Result<DescribeFaqResponse, RusotoError<DescribeFaqError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "AWSKendraFrontendService.DescribeFaq");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, DescribeFaqError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<DescribeFaqResponse, _>()
    }

    /// <p>Describes an existing Amazon Kendra index</p>
    async fn describe_index(
        &self,
        input: DescribeIndexRequest,
    ) -> Result<DescribeIndexResponse, RusotoError<DescribeIndexError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "AWSKendraFrontendService.DescribeIndex");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, DescribeIndexError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<DescribeIndexResponse, _>()
    }

    /// <p>Describes an existing Amazon Kendra thesaurus.</p>
    async fn describe_thesaurus(
        &self,
        input: DescribeThesaurusRequest,
    ) -> Result<DescribeThesaurusResponse, RusotoError<DescribeThesaurusError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "AWSKendraFrontendService.DescribeThesaurus");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, DescribeThesaurusError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<DescribeThesaurusResponse, _>()
    }

    /// <p>Gets statistics about synchronizing Amazon Kendra with a data source.</p>
    async fn list_data_source_sync_jobs(
        &self,
        input: ListDataSourceSyncJobsRequest,
    ) -> Result<ListDataSourceSyncJobsResponse, RusotoError<ListDataSourceSyncJobsError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "AWSKendraFrontendService.ListDataSourceSyncJobs",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, ListDataSourceSyncJobsError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response)
            .deserialize::<ListDataSourceSyncJobsResponse, _>()
    }

    /// <p>Lists the data sources that you have created.</p>
    async fn list_data_sources(
        &self,
        input: ListDataSourcesRequest,
    ) -> Result<ListDataSourcesResponse, RusotoError<ListDataSourcesError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "AWSKendraFrontendService.ListDataSources");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, ListDataSourcesError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<ListDataSourcesResponse, _>()
    }

    /// <p>Gets a list of FAQ lists associated with an index.</p>
    async fn list_faqs(
        &self,
        input: ListFaqsRequest,
    ) -> Result<ListFaqsResponse, RusotoError<ListFaqsError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "AWSKendraFrontendService.ListFaqs");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, ListFaqsError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<ListFaqsResponse, _>()
    }

    /// <p>Lists the Amazon Kendra indexes that you have created.</p>
    async fn list_indices(
        &self,
        input: ListIndicesRequest,
    ) -> Result<ListIndicesResponse, RusotoError<ListIndicesError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "AWSKendraFrontendService.ListIndices");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, ListIndicesError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<ListIndicesResponse, _>()
    }

    /// <p>Gets a list of tags associated with a specified resource. Indexes, FAQs, and data sources can have tags associated with them.</p>
    async fn list_tags_for_resource(
        &self,
        input: ListTagsForResourceRequest,
    ) -> Result<ListTagsForResourceResponse, RusotoError<ListTagsForResourceError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "AWSKendraFrontendService.ListTagsForResource",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, ListTagsForResourceError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<ListTagsForResourceResponse, _>()
    }

    /// <p>Lists the Amazon Kendra thesauri associated with an index.</p>
    async fn list_thesauri(
        &self,
        input: ListThesauriRequest,
    ) -> Result<ListThesauriResponse, RusotoError<ListThesauriError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "AWSKendraFrontendService.ListThesauri");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, ListThesauriError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<ListThesauriResponse, _>()
    }

    /// <p>Searches an active index. Use this API to search your documents using query. The <code>Query</code> operation enables to do faceted search and to filter results based on document attributes.</p> <p>It also enables you to provide user context that Amazon Kendra uses to enforce document access control in the search results. </p> <p>Amazon Kendra searches your index for text content and question and answer (FAQ) content. By default the response contains three types of results.</p> <ul> <li> <p>Relevant passages</p> </li> <li> <p>Matching FAQs</p> </li> <li> <p>Relevant documents</p> </li> </ul> <p>You can specify that the query return only one type of result using the <code>QueryResultTypeConfig</code> parameter.</p> <p>Each query returns the 100 most relevant results. </p>
    async fn query(&self, input: QueryRequest) -> Result<QueryResult, RusotoError<QueryError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "AWSKendraFrontendService.Query");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, QueryError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<QueryResult, _>()
    }

    /// <p>Starts a synchronization job for a data source. If a synchronization job is already in progress, Amazon Kendra returns a <code>ResourceInUseException</code> exception.</p>
    async fn start_data_source_sync_job(
        &self,
        input: StartDataSourceSyncJobRequest,
    ) -> Result<StartDataSourceSyncJobResponse, RusotoError<StartDataSourceSyncJobError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "AWSKendraFrontendService.StartDataSourceSyncJob",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, StartDataSourceSyncJobError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response)
            .deserialize::<StartDataSourceSyncJobResponse, _>()
    }

    /// <p>Stops a running synchronization job. You can't stop a scheduled synchronization job.</p>
    async fn stop_data_source_sync_job(
        &self,
        input: StopDataSourceSyncJobRequest,
    ) -> Result<(), RusotoError<StopDataSourceSyncJobError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "AWSKendraFrontendService.StopDataSourceSyncJob",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, StopDataSourceSyncJobError::from_response)
            .await?;
        std::mem::drop(response);
        Ok(())
    }

    /// <p>Enables you to provide feedback to Amazon Kendra to improve the performance of the service. </p>
    async fn submit_feedback(
        &self,
        input: SubmitFeedbackRequest,
    ) -> Result<(), RusotoError<SubmitFeedbackError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "AWSKendraFrontendService.SubmitFeedback");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, SubmitFeedbackError::from_response)
            .await?;
        std::mem::drop(response);
        Ok(())
    }

    /// <p>Adds the specified tag to the specified index, FAQ, or data source resource. If the tag already exists, the existing value is replaced with the new value.</p>
    async fn tag_resource(
        &self,
        input: TagResourceRequest,
    ) -> Result<TagResourceResponse, RusotoError<TagResourceError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "AWSKendraFrontendService.TagResource");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, TagResourceError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<TagResourceResponse, _>()
    }

    /// <p>Removes a tag from an index, FAQ, or a data source.</p>
    async fn untag_resource(
        &self,
        input: UntagResourceRequest,
    ) -> Result<UntagResourceResponse, RusotoError<UntagResourceError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "AWSKendraFrontendService.UntagResource");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, UntagResourceError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<UntagResourceResponse, _>()
    }

    /// <p>Updates an existing Amazon Kendra data source.</p>
    async fn update_data_source(
        &self,
        input: UpdateDataSourceRequest,
    ) -> Result<(), RusotoError<UpdateDataSourceError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "AWSKendraFrontendService.UpdateDataSource");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, UpdateDataSourceError::from_response)
            .await?;
        std::mem::drop(response);
        Ok(())
    }

    /// <p>Updates an existing Amazon Kendra index.</p>
    async fn update_index(
        &self,
        input: UpdateIndexRequest,
    ) -> Result<(), RusotoError<UpdateIndexError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "AWSKendraFrontendService.UpdateIndex");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, UpdateIndexError::from_response)
            .await?;
        std::mem::drop(response);
        Ok(())
    }

    /// <p>Updates a thesaurus file associated with an index.</p>
    async fn update_thesaurus(
        &self,
        input: UpdateThesaurusRequest,
    ) -> Result<(), RusotoError<UpdateThesaurusError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "AWSKendraFrontendService.UpdateThesaurus");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, UpdateThesaurusError::from_response)
            .await?;
        std::mem::drop(response);
        Ok(())
    }
}
