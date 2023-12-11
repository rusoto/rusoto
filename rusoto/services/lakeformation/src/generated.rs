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

impl LakeFormationClient {
    fn new_signed_request(&self, http_method: &str, request_uri: &str) -> SignedRequest {
        let mut request =
            SignedRequest::new(http_method, "lakeformation", &self.region, request_uri);

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
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct AddLFTagsToResourceRequest {
    /// <p>The identifier for the Data Catalog. By default, the account ID. The Data Catalog is the persistent metadata store. It contains database definitions, table definitions, and other control information to manage your AWS Lake Formation environment. </p>
    #[serde(rename = "CatalogId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub catalog_id: Option<String>,
    /// <p>The tags to attach to the resource.</p>
    #[serde(rename = "LFTags")]
    pub lf_tags: Vec<LFTagPair>,
    /// <p>The resource to which to attach a tag.</p>
    #[serde(rename = "Resource")]
    pub resource: Resource,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct AddLFTagsToResourceResponse {
    /// <p>A list of failures to tag the resource.</p>
    #[serde(rename = "Failures")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failures: Option<Vec<LFTagError>>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct BatchGrantPermissionsRequest {
    /// <p>The identifier for the Data Catalog. By default, the account ID. The Data Catalog is the persistent metadata store. It contains database definitions, table definitions, and other control information to manage your AWS Lake Formation environment. </p>
    #[serde(rename = "CatalogId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub catalog_id: Option<String>,
    /// <p>A list of up to 20 entries for resource permissions to be granted by batch operation to the principal.</p>
    #[serde(rename = "Entries")]
    pub entries: Vec<BatchPermissionsRequestEntry>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct BatchGrantPermissionsResponse {
    /// <p>A list of failures to grant permissions to the resources.</p>
    #[serde(rename = "Failures")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failures: Option<Vec<BatchPermissionsFailureEntry>>,
}

/// <p>A list of failures when performing a batch grant or batch revoke operation.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct BatchPermissionsFailureEntry {
    /// <p>An error message that applies to the failure of the entry.</p>
    #[serde(rename = "Error")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<ErrorDetail>,
    /// <p>An identifier for an entry of the batch request.</p>
    #[serde(rename = "RequestEntry")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_entry: Option<BatchPermissionsRequestEntry>,
}

/// <p>A permission to a resource granted by batch operation to the principal.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct BatchPermissionsRequestEntry {
    /// <p>A unique identifier for the batch permissions request entry.</p>
    #[serde(rename = "Id")]
    pub id: String,
    /// <p>The permissions to be granted.</p>
    #[serde(rename = "Permissions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permissions: Option<Vec<String>>,
    /// <p>Indicates if the option to pass permissions is granted.</p>
    #[serde(rename = "PermissionsWithGrantOption")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permissions_with_grant_option: Option<Vec<String>>,
    /// <p>The principal to be granted a permission.</p>
    #[serde(rename = "Principal")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub principal: Option<DataLakePrincipal>,
    /// <p>The resource to which the principal is to be granted a permission.</p>
    #[serde(rename = "Resource")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource: Option<Resource>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct BatchRevokePermissionsRequest {
    /// <p>The identifier for the Data Catalog. By default, the account ID. The Data Catalog is the persistent metadata store. It contains database definitions, table definitions, and other control information to manage your AWS Lake Formation environment. </p>
    #[serde(rename = "CatalogId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub catalog_id: Option<String>,
    /// <p>A list of up to 20 entries for resource permissions to be revoked by batch operation to the principal.</p>
    #[serde(rename = "Entries")]
    pub entries: Vec<BatchPermissionsRequestEntry>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct BatchRevokePermissionsResponse {
    /// <p>A list of failures to revoke permissions to the resources.</p>
    #[serde(rename = "Failures")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failures: Option<Vec<BatchPermissionsFailureEntry>>,
}

/// <p>A structure for the catalog object.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct CatalogResource {}

/// <p>A structure containing the name of a column resource and the tags attached to it.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ColumnLFTag {
    /// <p>The tags attached to a column resource.</p>
    #[serde(rename = "LFTags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lf_tags: Option<Vec<LFTagPair>>,
    /// <p>The name of a column resource.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

/// <p>A wildcard object, consisting of an optional list of excluded column names or indexes.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct ColumnWildcard {
    /// <p>Excludes column names. Any column with this name will be excluded.</p>
    #[serde(rename = "ExcludedColumnNames")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub excluded_column_names: Option<Vec<String>>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateLFTagRequest {
    /// <p>The identifier for the Data Catalog. By default, the account ID. The Data Catalog is the persistent metadata store. It contains database definitions, table definitions, and other control information to manage your AWS Lake Formation environment. </p>
    #[serde(rename = "CatalogId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub catalog_id: Option<String>,
    /// <p>The key-name for the tag.</p>
    #[serde(rename = "TagKey")]
    pub tag_key: String,
    /// <p>A list of possible values an attribute can take.</p>
    #[serde(rename = "TagValues")]
    pub tag_values: Vec<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateLFTagResponse {}

/// <p>The AWS Lake Formation principal. Supported principals are IAM users or IAM roles.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct DataLakePrincipal {
    /// <p>An identifier for the AWS Lake Formation principal.</p>
    #[serde(rename = "DataLakePrincipalIdentifier")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_lake_principal_identifier: Option<String>,
}

/// <p>A structure representing a list of AWS Lake Formation principals designated as data lake administrators and lists of principal permission entries for default create database and default create table permissions.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct DataLakeSettings {
    /// <p>A structure representing a list of up to three principal permissions entries for default create database permissions.</p>
    #[serde(rename = "CreateDatabaseDefaultPermissions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_database_default_permissions: Option<Vec<PrincipalPermissions>>,
    /// <p>A structure representing a list of up to three principal permissions entries for default create table permissions.</p>
    #[serde(rename = "CreateTableDefaultPermissions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_table_default_permissions: Option<Vec<PrincipalPermissions>>,
    /// <p>A list of AWS Lake Formation principals. Supported principals are IAM users or IAM roles.</p>
    #[serde(rename = "DataLakeAdmins")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_lake_admins: Option<Vec<DataLakePrincipal>>,
    /// <p>A list of the resource-owning account IDs that the caller's account can use to share their user access details (user ARNs). The user ARNs can be logged in the resource owner's AWS CloudTrail log.</p> <p>You may want to specify this property when you are in a high-trust boundary, such as the same team or company. </p>
    #[serde(rename = "TrustedResourceOwners")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trusted_resource_owners: Option<Vec<String>>,
}

/// <p>A structure for a data location object where permissions are granted or revoked. </p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct DataLocationResource {
    /// <p>The identifier for the Data Catalog where the location is registered with AWS Lake Formation. By default, it is the account ID of the caller.</p>
    #[serde(rename = "CatalogId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub catalog_id: Option<String>,
    /// <p>The Amazon Resource Name (ARN) that uniquely identifies the data location resource.</p>
    #[serde(rename = "ResourceArn")]
    pub resource_arn: String,
}

/// <p>A structure for the database object.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct DatabaseResource {
    /// <p>The identifier for the Data Catalog. By default, it is the account ID of the caller.</p>
    #[serde(rename = "CatalogId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub catalog_id: Option<String>,
    /// <p>The name of the database resource. Unique to the Data Catalog.</p>
    #[serde(rename = "Name")]
    pub name: String,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteLFTagRequest {
    /// <p>The identifier for the Data Catalog. By default, the account ID. The Data Catalog is the persistent metadata store. It contains database definitions, table definitions, and other control information to manage your AWS Lake Formation environment. </p>
    #[serde(rename = "CatalogId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub catalog_id: Option<String>,
    /// <p>The key-name for the tag to delete.</p>
    #[serde(rename = "TagKey")]
    pub tag_key: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DeleteLFTagResponse {}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeregisterResourceRequest {
    /// <p>The Amazon Resource Name (ARN) of the resource that you want to deregister.</p>
    #[serde(rename = "ResourceArn")]
    pub resource_arn: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DeregisterResourceResponse {}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeResourceRequest {
    /// <p>The resource ARN.</p>
    #[serde(rename = "ResourceArn")]
    pub resource_arn: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeResourceResponse {
    /// <p>A structure containing information about an AWS Lake Formation resource.</p>
    #[serde(rename = "ResourceInfo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_info: Option<ResourceInfo>,
}

/// <p>A structure containing the additional details to be returned in the <code>AdditionalDetails</code> attribute of <code>PrincipalResourcePermissions</code>.</p> <p>If a catalog resource is shared through AWS Resource Access Manager (AWS RAM), then there will exist a corresponding RAM resource share ARN.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DetailsMap {
    /// <p>A resource share ARN for a catalog resource shared through AWS Resource Access Manager (AWS RAM).</p>
    #[serde(rename = "ResourceShare")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_share: Option<Vec<String>>,
}

/// <p>Contains details about an error.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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

/// <p>This structure describes the filtering of columns in a table based on a filter condition.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct FilterCondition {
    /// <p>The comparison operator used in the filter condition.</p>
    #[serde(rename = "ComparisonOperator")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comparison_operator: Option<String>,
    /// <p>The field to filter in the filter condition.</p>
    #[serde(rename = "Field")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub field: Option<String>,
    /// <p>A string with values used in evaluating the filter condition.</p>
    #[serde(rename = "StringValueList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub string_value_list: Option<Vec<String>>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetDataLakeSettingsRequest {
    /// <p>The identifier for the Data Catalog. By default, the account ID. The Data Catalog is the persistent metadata store. It contains database definitions, table definitions, and other control information to manage your AWS Lake Formation environment. </p>
    #[serde(rename = "CatalogId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub catalog_id: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetDataLakeSettingsResponse {
    /// <p>A structure representing a list of AWS Lake Formation principals designated as data lake administrators.</p>
    #[serde(rename = "DataLakeSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_lake_settings: Option<DataLakeSettings>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetEffectivePermissionsForPathRequest {
    /// <p>The identifier for the Data Catalog. By default, the account ID. The Data Catalog is the persistent metadata store. It contains database definitions, table definitions, and other control information to manage your AWS Lake Formation environment. </p>
    #[serde(rename = "CatalogId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub catalog_id: Option<String>,
    /// <p>The maximum number of results to return.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>A continuation token, if this is not the first call to retrieve this list.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The Amazon Resource Name (ARN) of the resource for which you want to get permissions.</p>
    #[serde(rename = "ResourceArn")]
    pub resource_arn: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetEffectivePermissionsForPathResponse {
    /// <p>A continuation token, if this is not the first call to retrieve this list.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>A list of the permissions for the specified table or database resource located at the path in Amazon S3.</p>
    #[serde(rename = "Permissions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permissions: Option<Vec<PrincipalResourcePermissions>>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetLFTagRequest {
    /// <p>The identifier for the Data Catalog. By default, the account ID. The Data Catalog is the persistent metadata store. It contains database definitions, table definitions, and other control information to manage your AWS Lake Formation environment. </p>
    #[serde(rename = "CatalogId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub catalog_id: Option<String>,
    /// <p>The key-name for the tag.</p>
    #[serde(rename = "TagKey")]
    pub tag_key: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetLFTagResponse {
    /// <p>The identifier for the Data Catalog. By default, the account ID. The Data Catalog is the persistent metadata store. It contains database definitions, table definitions, and other control information to manage your AWS Lake Formation environment. </p>
    #[serde(rename = "CatalogId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub catalog_id: Option<String>,
    /// <p>The key-name for the tag.</p>
    #[serde(rename = "TagKey")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_key: Option<String>,
    /// <p>A list of possible values an attribute can take.</p>
    #[serde(rename = "TagValues")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_values: Option<Vec<String>>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetResourceLFTagsRequest {
    /// <p>The identifier for the Data Catalog. By default, the account ID. The Data Catalog is the persistent metadata store. It contains database definitions, table definitions, and other control information to manage your AWS Lake Formation environment. </p>
    #[serde(rename = "CatalogId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub catalog_id: Option<String>,
    /// <p>The resource for which you want to return tags.</p>
    #[serde(rename = "Resource")]
    pub resource: Resource,
    /// <p>Indicates whether to show the assigned tags.</p>
    #[serde(rename = "ShowAssignedLFTags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub show_assigned_lf_tags: Option<bool>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetResourceLFTagsResponse {
    /// <p>A list of tags applied to a database resource.</p>
    #[serde(rename = "LFTagOnDatabase")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lf_tag_on_database: Option<Vec<LFTagPair>>,
    /// <p>A list of tags applied to a column resource.</p>
    #[serde(rename = "LFTagsOnColumns")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lf_tags_on_columns: Option<Vec<ColumnLFTag>>,
    /// <p>A list of tags applied to a table resource.</p>
    #[serde(rename = "LFTagsOnTable")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lf_tags_on_table: Option<Vec<LFTagPair>>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GrantPermissionsRequest {
    /// <p>The identifier for the Data Catalog. By default, the account ID. The Data Catalog is the persistent metadata store. It contains database definitions, table definitions, and other control information to manage your AWS Lake Formation environment. </p>
    #[serde(rename = "CatalogId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub catalog_id: Option<String>,
    /// <p>The permissions granted to the principal on the resource. AWS Lake Formation defines privileges to grant and revoke access to metadata in the Data Catalog and data organized in underlying data storage such as Amazon S3. AWS Lake Formation requires that each principal be authorized to perform a specific task on AWS Lake Formation resources. </p>
    #[serde(rename = "Permissions")]
    pub permissions: Vec<String>,
    /// <p>Indicates a list of the granted permissions that the principal may pass to other users. These permissions may only be a subset of the permissions granted in the <code>Privileges</code>.</p>
    #[serde(rename = "PermissionsWithGrantOption")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permissions_with_grant_option: Option<Vec<String>>,
    /// <p>The principal to be granted the permissions on the resource. Supported principals are IAM users or IAM roles, and they are defined by their principal type and their ARN.</p> <p>Note that if you define a resource with a particular ARN, then later delete, and recreate a resource with that same ARN, the resource maintains the permissions already granted. </p>
    #[serde(rename = "Principal")]
    pub principal: DataLakePrincipal,
    /// <p>The resource to which permissions are to be granted. Resources in AWS Lake Formation are the Data Catalog, databases, and tables.</p>
    #[serde(rename = "Resource")]
    pub resource: Resource,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GrantPermissionsResponse {}

/// <p>A structure that allows an admin to grant user permissions on certain conditions. For example, granting a role access to all columns not tagged 'PII' of tables tagged 'Prod'.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct LFTag {
    /// <p>The key-name for the tag.</p>
    #[serde(rename = "TagKey")]
    pub tag_key: String,
    /// <p>A list of possible values an attribute can take.</p>
    #[serde(rename = "TagValues")]
    pub tag_values: Vec<String>,
}

/// <p>A structure containing an error related to a <code>TagResource</code> or <code>UnTagResource</code> operation.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct LFTagError {
    /// <p>An error that occurred with the attachment or detachment of the tag.</p>
    #[serde(rename = "Error")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<ErrorDetail>,
    /// <p>The key-name of the tag.</p>
    #[serde(rename = "LFTag")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lf_tag: Option<LFTagPair>,
}

/// <p>A structure containing a tag key and values for a resource.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct LFTagKeyResource {
    /// <p>The identifier for the Data Catalog. By default, the account ID. The Data Catalog is the persistent metadata store. It contains database definitions, table definitions, and other control information to manage your AWS Lake Formation environment. </p>
    #[serde(rename = "CatalogId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub catalog_id: Option<String>,
    /// <p>The key-name for the tag.</p>
    #[serde(rename = "TagKey")]
    pub tag_key: String,
    /// <p>A list of possible values an attribute can take.</p>
    #[serde(rename = "TagValues")]
    pub tag_values: Vec<String>,
}

/// <p>A structure containing a tag key-value pair.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct LFTagPair {
    /// <p>The identifier for the Data Catalog. By default, the account ID. The Data Catalog is the persistent metadata store. It contains database definitions, table definitions, and other control information to manage your AWS Lake Formation environment. </p>
    #[serde(rename = "CatalogId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub catalog_id: Option<String>,
    /// <p>The key-name for the tag.</p>
    #[serde(rename = "TagKey")]
    pub tag_key: String,
    /// <p>A list of possible values an attribute can take.</p>
    #[serde(rename = "TagValues")]
    pub tag_values: Vec<String>,
}

/// <p>A structure containing a list of tag conditions that apply to a resource's tag policy.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct LFTagPolicyResource {
    /// <p>The identifier for the Data Catalog. By default, the account ID. The Data Catalog is the persistent metadata store. It contains database definitions, table definitions, and other control information to manage your AWS Lake Formation environment. </p>
    #[serde(rename = "CatalogId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub catalog_id: Option<String>,
    /// <p>A list of tag conditions that apply to the resource's tag policy.</p>
    #[serde(rename = "Expression")]
    pub expression: Vec<LFTag>,
    /// <p>The resource type for which the tag policy applies.</p>
    #[serde(rename = "ResourceType")]
    pub resource_type: String,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListLFTagsRequest {
    /// <p>The identifier for the Data Catalog. By default, the account ID. The Data Catalog is the persistent metadata store. It contains database definitions, table definitions, and other control information to manage your AWS Lake Formation environment. </p>
    #[serde(rename = "CatalogId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub catalog_id: Option<String>,
    /// <p>The maximum number of results to return.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>A continuation token, if this is not the first call to retrieve this list.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>If resource share type is <code>ALL</code>, returns both in-account tags and shared tags that the requester has permission to view. If resource share type is <code>FOREIGN</code>, returns all share tags that the requester can view. If no resource share type is passed, lists tags in the given catalog ID that the requester has permission to view.</p>
    #[serde(rename = "ResourceShareType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_share_type: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListLFTagsResponse {
    /// <p>A list of tags that the requested has permission to view.</p>
    #[serde(rename = "LFTags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lf_tags: Option<Vec<LFTagPair>>,
    /// <p>A continuation token, present if the current list segment is not the last.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListPermissionsRequest {
    /// <p>The identifier for the Data Catalog. By default, the account ID. The Data Catalog is the persistent metadata store. It contains database definitions, table definitions, and other control information to manage your AWS Lake Formation environment. </p>
    #[serde(rename = "CatalogId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub catalog_id: Option<String>,
    /// <p>The maximum number of results to return.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>A continuation token, if this is not the first call to retrieve this list.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>Specifies a principal to filter the permissions returned.</p>
    #[serde(rename = "Principal")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub principal: Option<DataLakePrincipal>,
    /// <p>A resource where you will get a list of the principal permissions.</p> <p>This operation does not support getting privileges on a table with columns. Instead, call this operation on the table, and the operation returns the table and the table w columns.</p>
    #[serde(rename = "Resource")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource: Option<Resource>,
    /// <p>Specifies a resource type to filter the permissions returned.</p>
    #[serde(rename = "ResourceType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListPermissionsResponse {
    /// <p>A continuation token, if this is not the first call to retrieve this list.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>A list of principals and their permissions on the resource for the specified principal and resource types.</p>
    #[serde(rename = "PrincipalResourcePermissions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub principal_resource_permissions: Option<Vec<PrincipalResourcePermissions>>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListResourcesRequest {
    /// <p>Any applicable row-level and/or column-level filtering conditions for the resources.</p>
    #[serde(rename = "FilterConditionList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter_condition_list: Option<Vec<FilterCondition>>,
    /// <p>The maximum number of resource results.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>A continuation token, if this is not the first call to retrieve these resources.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListResourcesResponse {
    /// <p>A continuation token, if this is not the first call to retrieve these resources.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>A summary of the data lake resources.</p>
    #[serde(rename = "ResourceInfoList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_info_list: Option<Vec<ResourceInfo>>,
}

/// <p>Permissions granted to a principal.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct PrincipalPermissions {
    /// <p>The permissions that are granted to the principal.</p>
    #[serde(rename = "Permissions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permissions: Option<Vec<String>>,
    /// <p>The principal who is granted permissions.</p>
    #[serde(rename = "Principal")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub principal: Option<DataLakePrincipal>,
}

/// <p>The permissions granted or revoked on a resource.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct PrincipalResourcePermissions {
    /// <p>This attribute can be used to return any additional details of <code>PrincipalResourcePermissions</code>. Currently returns only as a RAM resource share ARN.</p>
    #[serde(rename = "AdditionalDetails")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_details: Option<DetailsMap>,
    /// <p>The permissions to be granted or revoked on the resource.</p>
    #[serde(rename = "Permissions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permissions: Option<Vec<String>>,
    /// <p>Indicates whether to grant the ability to grant permissions (as a subset of permissions granted).</p>
    #[serde(rename = "PermissionsWithGrantOption")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permissions_with_grant_option: Option<Vec<String>>,
    /// <p>The Data Lake principal to be granted or revoked permissions.</p>
    #[serde(rename = "Principal")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub principal: Option<DataLakePrincipal>,
    /// <p>The resource where permissions are to be granted or revoked.</p>
    #[serde(rename = "Resource")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource: Option<Resource>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct PutDataLakeSettingsRequest {
    /// <p>The identifier for the Data Catalog. By default, the account ID. The Data Catalog is the persistent metadata store. It contains database definitions, table definitions, and other control information to manage your AWS Lake Formation environment. </p>
    #[serde(rename = "CatalogId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub catalog_id: Option<String>,
    /// <p>A structure representing a list of AWS Lake Formation principals designated as data lake administrators.</p>
    #[serde(rename = "DataLakeSettings")]
    pub data_lake_settings: DataLakeSettings,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct PutDataLakeSettingsResponse {}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct RegisterResourceRequest {
    /// <p>The Amazon Resource Name (ARN) of the resource that you want to register.</p>
    #[serde(rename = "ResourceArn")]
    pub resource_arn: String,
    /// <p>The identifier for the role that registers the resource.</p>
    #[serde(rename = "RoleArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_arn: Option<String>,
    /// <p>Designates an AWS Identity and Access Management (IAM) service-linked role by registering this role with the Data Catalog. A service-linked role is a unique type of IAM role that is linked directly to Lake Formation.</p> <p>For more information, see <a href="https://docs-aws.amazon.com/lake-formation/latest/dg/service-linked-roles.html">Using Service-Linked Roles for Lake Formation</a>.</p>
    #[serde(rename = "UseServiceLinkedRole")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_service_linked_role: Option<bool>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct RegisterResourceResponse {}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct RemoveLFTagsFromResourceRequest {
    /// <p>The identifier for the Data Catalog. By default, the account ID. The Data Catalog is the persistent metadata store. It contains database definitions, table definitions, and other control information to manage your AWS Lake Formation environment. </p>
    #[serde(rename = "CatalogId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub catalog_id: Option<String>,
    /// <p>The tags to be removed from the resource.</p>
    #[serde(rename = "LFTags")]
    pub lf_tags: Vec<LFTagPair>,
    /// <p>The resource where you want to remove a tag.</p>
    #[serde(rename = "Resource")]
    pub resource: Resource,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct RemoveLFTagsFromResourceResponse {
    /// <p>A list of failures to untag a resource.</p>
    #[serde(rename = "Failures")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failures: Option<Vec<LFTagError>>,
}

/// <p>A structure for the resource.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct Resource {
    /// <p>The identifier for the Data Catalog. By default, the account ID. The Data Catalog is the persistent metadata store. It contains database definitions, table definitions, and other control information to manage your AWS Lake Formation environment. </p>
    #[serde(rename = "Catalog")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub catalog: Option<CatalogResource>,
    /// <p>The location of an Amazon S3 path where permissions are granted or revoked. </p>
    #[serde(rename = "DataLocation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_location: Option<DataLocationResource>,
    /// <p>The database for the resource. Unique to the Data Catalog. A database is a set of associated table definitions organized into a logical group. You can Grant and Revoke database permissions to a principal. </p>
    #[serde(rename = "Database")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub database: Option<DatabaseResource>,
    /// <p>The tag key and values attached to a resource.</p>
    #[serde(rename = "LFTag")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lf_tag: Option<LFTagKeyResource>,
    /// <p>A list of tag conditions that define a resource's tag policy.</p>
    #[serde(rename = "LFTagPolicy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lf_tag_policy: Option<LFTagPolicyResource>,
    /// <p>The table for the resource. A table is a metadata definition that represents your data. You can Grant and Revoke table privileges to a principal. </p>
    #[serde(rename = "Table")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub table: Option<TableResource>,
    /// <p>The table with columns for the resource. A principal with permissions to this resource can select metadata from the columns of a table in the Data Catalog and the underlying data in Amazon S3.</p>
    #[serde(rename = "TableWithColumns")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub table_with_columns: Option<TableWithColumnsResource>,
}

/// <p>A structure containing information about an AWS Lake Formation resource.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ResourceInfo {
    /// <p>The date and time the resource was last modified.</p>
    #[serde(rename = "LastModified")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified: Option<f64>,
    /// <p>The Amazon Resource Name (ARN) of the resource.</p>
    #[serde(rename = "ResourceArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_arn: Option<String>,
    /// <p>The IAM role that registered a resource.</p>
    #[serde(rename = "RoleArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_arn: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct RevokePermissionsRequest {
    /// <p>The identifier for the Data Catalog. By default, the account ID. The Data Catalog is the persistent metadata store. It contains database definitions, table definitions, and other control information to manage your AWS Lake Formation environment. </p>
    #[serde(rename = "CatalogId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub catalog_id: Option<String>,
    /// <p>The permissions revoked to the principal on the resource. For information about permissions, see <a href="https://docs-aws.amazon.com/lake-formation/latest/dg/security-data-access.html">Security and Access Control to Metadata and Data</a>.</p>
    #[serde(rename = "Permissions")]
    pub permissions: Vec<String>,
    /// <p>Indicates a list of permissions for which to revoke the grant option allowing the principal to pass permissions to other principals.</p>
    #[serde(rename = "PermissionsWithGrantOption")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permissions_with_grant_option: Option<Vec<String>>,
    /// <p>The principal to be revoked permissions on the resource.</p>
    #[serde(rename = "Principal")]
    pub principal: DataLakePrincipal,
    /// <p>The resource to which permissions are to be revoked.</p>
    #[serde(rename = "Resource")]
    pub resource: Resource,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct RevokePermissionsResponse {}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct SearchDatabasesByLFTagsRequest {
    /// <p>The identifier for the Data Catalog. By default, the account ID. The Data Catalog is the persistent metadata store. It contains database definitions, table definitions, and other control information to manage your AWS Lake Formation environment. </p>
    #[serde(rename = "CatalogId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub catalog_id: Option<String>,
    /// <p>A list of conditions (<code>LFTag</code> structures) to search for in database resources.</p>
    #[serde(rename = "Expression")]
    pub expression: Vec<LFTag>,
    /// <p>The maximum number of results to return.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>A continuation token, if this is not the first call to retrieve this list.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct SearchDatabasesByLFTagsResponse {
    /// <p>A list of databases that meet the tag conditions.</p>
    #[serde(rename = "DatabaseList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub database_list: Option<Vec<TaggedDatabase>>,
    /// <p>A continuation token, present if the current list segment is not the last.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct SearchTablesByLFTagsRequest {
    /// <p>The identifier for the Data Catalog. By default, the account ID. The Data Catalog is the persistent metadata store. It contains database definitions, table definitions, and other control information to manage your AWS Lake Formation environment. </p>
    #[serde(rename = "CatalogId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub catalog_id: Option<String>,
    /// <p>A list of conditions (<code>LFTag</code> structures) to search for in table resources.</p>
    #[serde(rename = "Expression")]
    pub expression: Vec<LFTag>,
    /// <p>The maximum number of results to return.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>A continuation token, if this is not the first call to retrieve this list.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct SearchTablesByLFTagsResponse {
    /// <p>A continuation token, present if the current list segment is not the last.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>A list of tables that meet the tag conditions.</p>
    #[serde(rename = "TableList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub table_list: Option<Vec<TaggedTable>>,
}

/// <p>A structure for the table object. A table is a metadata definition that represents your data. You can Grant and Revoke table privileges to a principal. </p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct TableResource {
    /// <p>The identifier for the Data Catalog. By default, it is the account ID of the caller.</p>
    #[serde(rename = "CatalogId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub catalog_id: Option<String>,
    /// <p>The name of the database for the table. Unique to a Data Catalog. A database is a set of associated table definitions organized into a logical group. You can Grant and Revoke database privileges to a principal. </p>
    #[serde(rename = "DatabaseName")]
    pub database_name: String,
    /// <p>The name of the table.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>A wildcard object representing every table under a database.</p> <p>At least one of <code>TableResource$Name</code> or <code>TableResource$TableWildcard</code> is required.</p>
    #[serde(rename = "TableWildcard")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub table_wildcard: Option<TableWildcard>,
}

/// <p>A wildcard object representing every table under a database.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct TableWildcard {}

/// <p>A structure for a table with columns object. This object is only used when granting a SELECT permission.</p> <p>This object must take a value for at least one of <code>ColumnsNames</code>, <code>ColumnsIndexes</code>, or <code>ColumnsWildcard</code>.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct TableWithColumnsResource {
    /// <p>The identifier for the Data Catalog. By default, it is the account ID of the caller.</p>
    #[serde(rename = "CatalogId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub catalog_id: Option<String>,
    /// <p>The list of column names for the table. At least one of <code>ColumnNames</code> or <code>ColumnWildcard</code> is required.</p>
    #[serde(rename = "ColumnNames")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub column_names: Option<Vec<String>>,
    /// <p>A wildcard specified by a <code>ColumnWildcard</code> object. At least one of <code>ColumnNames</code> or <code>ColumnWildcard</code> is required.</p>
    #[serde(rename = "ColumnWildcard")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub column_wildcard: Option<ColumnWildcard>,
    /// <p>The name of the database for the table with columns resource. Unique to the Data Catalog. A database is a set of associated table definitions organized into a logical group. You can Grant and Revoke database privileges to a principal. </p>
    #[serde(rename = "DatabaseName")]
    pub database_name: String,
    /// <p>The name of the table resource. A table is a metadata definition that represents your data. You can Grant and Revoke table privileges to a principal. </p>
    #[serde(rename = "Name")]
    pub name: String,
}

/// <p>A structure describing a database resource with tags.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct TaggedDatabase {
    /// <p>A database that has tags attached to it.</p>
    #[serde(rename = "Database")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub database: Option<DatabaseResource>,
    /// <p>A list of tags attached to the database.</p>
    #[serde(rename = "LFTags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lf_tags: Option<Vec<LFTagPair>>,
}

/// <p>A structure describing a table resource with tags.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct TaggedTable {
    /// <p>A list of tags attached to the database where the table resides.</p>
    #[serde(rename = "LFTagOnDatabase")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lf_tag_on_database: Option<Vec<LFTagPair>>,
    /// <p>A list of tags attached to columns in the table.</p>
    #[serde(rename = "LFTagsOnColumns")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lf_tags_on_columns: Option<Vec<ColumnLFTag>>,
    /// <p>A list of tags attached to the table.</p>
    #[serde(rename = "LFTagsOnTable")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lf_tags_on_table: Option<Vec<LFTagPair>>,
    /// <p>A table that has tags attached to it.</p>
    #[serde(rename = "Table")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub table: Option<TableResource>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UpdateLFTagRequest {
    /// <p>The identifier for the Data Catalog. By default, the account ID. The Data Catalog is the persistent metadata store. It contains database definitions, table definitions, and other control information to manage your AWS Lake Formation environment. </p>
    #[serde(rename = "CatalogId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub catalog_id: Option<String>,
    /// <p>The key-name for the tag for which to add or delete values.</p>
    #[serde(rename = "TagKey")]
    pub tag_key: String,
    /// <p>A list of tag values to add from the tag.</p>
    #[serde(rename = "TagValuesToAdd")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_values_to_add: Option<Vec<String>>,
    /// <p>A list of tag values to delete from the tag.</p>
    #[serde(rename = "TagValuesToDelete")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_values_to_delete: Option<Vec<String>>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UpdateLFTagResponse {}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UpdateResourceRequest {
    /// <p>The resource ARN.</p>
    #[serde(rename = "ResourceArn")]
    pub resource_arn: String,
    /// <p>The new role to use for the given resource registered in AWS Lake Formation.</p>
    #[serde(rename = "RoleArn")]
    pub role_arn: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UpdateResourceResponse {}

/// Errors returned by AddLFTagsToResource
#[derive(Debug, PartialEq)]
pub enum AddLFTagsToResourceError {
    /// <p>Access to a resource was denied.</p>
    AccessDenied(String),
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

impl AddLFTagsToResourceError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<AddLFTagsToResourceError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(AddLFTagsToResourceError::AccessDenied(err.msg))
                }
                "ConcurrentModificationException" => {
                    return RusotoError::Service(AddLFTagsToResourceError::ConcurrentModification(
                        err.msg,
                    ))
                }
                "EntityNotFoundException" => {
                    return RusotoError::Service(AddLFTagsToResourceError::EntityNotFound(err.msg))
                }
                "InternalServiceException" => {
                    return RusotoError::Service(AddLFTagsToResourceError::InternalService(err.msg))
                }
                "InvalidInputException" => {
                    return RusotoError::Service(AddLFTagsToResourceError::InvalidInput(err.msg))
                }
                "OperationTimeoutException" => {
                    return RusotoError::Service(AddLFTagsToResourceError::OperationTimeout(
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
impl fmt::Display for AddLFTagsToResourceError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            AddLFTagsToResourceError::AccessDenied(ref cause) => write!(f, "{}", cause),
            AddLFTagsToResourceError::ConcurrentModification(ref cause) => write!(f, "{}", cause),
            AddLFTagsToResourceError::EntityNotFound(ref cause) => write!(f, "{}", cause),
            AddLFTagsToResourceError::InternalService(ref cause) => write!(f, "{}", cause),
            AddLFTagsToResourceError::InvalidInput(ref cause) => write!(f, "{}", cause),
            AddLFTagsToResourceError::OperationTimeout(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for AddLFTagsToResourceError {}
/// Errors returned by BatchGrantPermissions
#[derive(Debug, PartialEq)]
pub enum BatchGrantPermissionsError {
    /// <p>The input provided was not valid.</p>
    InvalidInput(String),
    /// <p>The operation timed out.</p>
    OperationTimeout(String),
}

impl BatchGrantPermissionsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<BatchGrantPermissionsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidInputException" => {
                    return RusotoError::Service(BatchGrantPermissionsError::InvalidInput(err.msg))
                }
                "OperationTimeoutException" => {
                    return RusotoError::Service(BatchGrantPermissionsError::OperationTimeout(
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
impl fmt::Display for BatchGrantPermissionsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            BatchGrantPermissionsError::InvalidInput(ref cause) => write!(f, "{}", cause),
            BatchGrantPermissionsError::OperationTimeout(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for BatchGrantPermissionsError {}
/// Errors returned by BatchRevokePermissions
#[derive(Debug, PartialEq)]
pub enum BatchRevokePermissionsError {
    /// <p>The input provided was not valid.</p>
    InvalidInput(String),
    /// <p>The operation timed out.</p>
    OperationTimeout(String),
}

impl BatchRevokePermissionsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<BatchRevokePermissionsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidInputException" => {
                    return RusotoError::Service(BatchRevokePermissionsError::InvalidInput(err.msg))
                }
                "OperationTimeoutException" => {
                    return RusotoError::Service(BatchRevokePermissionsError::OperationTimeout(
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
impl fmt::Display for BatchRevokePermissionsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            BatchRevokePermissionsError::InvalidInput(ref cause) => write!(f, "{}", cause),
            BatchRevokePermissionsError::OperationTimeout(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for BatchRevokePermissionsError {}
/// Errors returned by CreateLFTag
#[derive(Debug, PartialEq)]
pub enum CreateLFTagError {
    /// <p>Access to a resource was denied.</p>
    AccessDenied(String),
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

impl CreateLFTagError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateLFTagError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(CreateLFTagError::AccessDenied(err.msg))
                }
                "EntityNotFoundException" => {
                    return RusotoError::Service(CreateLFTagError::EntityNotFound(err.msg))
                }
                "InternalServiceException" => {
                    return RusotoError::Service(CreateLFTagError::InternalService(err.msg))
                }
                "InvalidInputException" => {
                    return RusotoError::Service(CreateLFTagError::InvalidInput(err.msg))
                }
                "OperationTimeoutException" => {
                    return RusotoError::Service(CreateLFTagError::OperationTimeout(err.msg))
                }
                "ResourceNumberLimitExceededException" => {
                    return RusotoError::Service(CreateLFTagError::ResourceNumberLimitExceeded(
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
impl fmt::Display for CreateLFTagError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreateLFTagError::AccessDenied(ref cause) => write!(f, "{}", cause),
            CreateLFTagError::EntityNotFound(ref cause) => write!(f, "{}", cause),
            CreateLFTagError::InternalService(ref cause) => write!(f, "{}", cause),
            CreateLFTagError::InvalidInput(ref cause) => write!(f, "{}", cause),
            CreateLFTagError::OperationTimeout(ref cause) => write!(f, "{}", cause),
            CreateLFTagError::ResourceNumberLimitExceeded(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CreateLFTagError {}
/// Errors returned by DeleteLFTag
#[derive(Debug, PartialEq)]
pub enum DeleteLFTagError {
    /// <p>Access to a resource was denied.</p>
    AccessDenied(String),
    /// <p>A specified entity does not exist</p>
    EntityNotFound(String),
    /// <p>An internal service error occurred.</p>
    InternalService(String),
    /// <p>The input provided was not valid.</p>
    InvalidInput(String),
    /// <p>The operation timed out.</p>
    OperationTimeout(String),
}

impl DeleteLFTagError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteLFTagError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(DeleteLFTagError::AccessDenied(err.msg))
                }
                "EntityNotFoundException" => {
                    return RusotoError::Service(DeleteLFTagError::EntityNotFound(err.msg))
                }
                "InternalServiceException" => {
                    return RusotoError::Service(DeleteLFTagError::InternalService(err.msg))
                }
                "InvalidInputException" => {
                    return RusotoError::Service(DeleteLFTagError::InvalidInput(err.msg))
                }
                "OperationTimeoutException" => {
                    return RusotoError::Service(DeleteLFTagError::OperationTimeout(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DeleteLFTagError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteLFTagError::AccessDenied(ref cause) => write!(f, "{}", cause),
            DeleteLFTagError::EntityNotFound(ref cause) => write!(f, "{}", cause),
            DeleteLFTagError::InternalService(ref cause) => write!(f, "{}", cause),
            DeleteLFTagError::InvalidInput(ref cause) => write!(f, "{}", cause),
            DeleteLFTagError::OperationTimeout(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeleteLFTagError {}
/// Errors returned by DeregisterResource
#[derive(Debug, PartialEq)]
pub enum DeregisterResourceError {
    /// <p>A specified entity does not exist</p>
    EntityNotFound(String),
    /// <p>An internal service error occurred.</p>
    InternalService(String),
    /// <p>The input provided was not valid.</p>
    InvalidInput(String),
    /// <p>The operation timed out.</p>
    OperationTimeout(String),
}

impl DeregisterResourceError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeregisterResourceError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "EntityNotFoundException" => {
                    return RusotoError::Service(DeregisterResourceError::EntityNotFound(err.msg))
                }
                "InternalServiceException" => {
                    return RusotoError::Service(DeregisterResourceError::InternalService(err.msg))
                }
                "InvalidInputException" => {
                    return RusotoError::Service(DeregisterResourceError::InvalidInput(err.msg))
                }
                "OperationTimeoutException" => {
                    return RusotoError::Service(DeregisterResourceError::OperationTimeout(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DeregisterResourceError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeregisterResourceError::EntityNotFound(ref cause) => write!(f, "{}", cause),
            DeregisterResourceError::InternalService(ref cause) => write!(f, "{}", cause),
            DeregisterResourceError::InvalidInput(ref cause) => write!(f, "{}", cause),
            DeregisterResourceError::OperationTimeout(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeregisterResourceError {}
/// Errors returned by DescribeResource
#[derive(Debug, PartialEq)]
pub enum DescribeResourceError {
    /// <p>A specified entity does not exist</p>
    EntityNotFound(String),
    /// <p>An internal service error occurred.</p>
    InternalService(String),
    /// <p>The input provided was not valid.</p>
    InvalidInput(String),
    /// <p>The operation timed out.</p>
    OperationTimeout(String),
}

impl DescribeResourceError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeResourceError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "EntityNotFoundException" => {
                    return RusotoError::Service(DescribeResourceError::EntityNotFound(err.msg))
                }
                "InternalServiceException" => {
                    return RusotoError::Service(DescribeResourceError::InternalService(err.msg))
                }
                "InvalidInputException" => {
                    return RusotoError::Service(DescribeResourceError::InvalidInput(err.msg))
                }
                "OperationTimeoutException" => {
                    return RusotoError::Service(DescribeResourceError::OperationTimeout(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DescribeResourceError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeResourceError::EntityNotFound(ref cause) => write!(f, "{}", cause),
            DescribeResourceError::InternalService(ref cause) => write!(f, "{}", cause),
            DescribeResourceError::InvalidInput(ref cause) => write!(f, "{}", cause),
            DescribeResourceError::OperationTimeout(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DescribeResourceError {}
/// Errors returned by GetDataLakeSettings
#[derive(Debug, PartialEq)]
pub enum GetDataLakeSettingsError {
    /// <p>A specified entity does not exist</p>
    EntityNotFound(String),
    /// <p>An internal service error occurred.</p>
    InternalService(String),
    /// <p>The input provided was not valid.</p>
    InvalidInput(String),
}

impl GetDataLakeSettingsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetDataLakeSettingsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "EntityNotFoundException" => {
                    return RusotoError::Service(GetDataLakeSettingsError::EntityNotFound(err.msg))
                }
                "InternalServiceException" => {
                    return RusotoError::Service(GetDataLakeSettingsError::InternalService(err.msg))
                }
                "InvalidInputException" => {
                    return RusotoError::Service(GetDataLakeSettingsError::InvalidInput(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for GetDataLakeSettingsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetDataLakeSettingsError::EntityNotFound(ref cause) => write!(f, "{}", cause),
            GetDataLakeSettingsError::InternalService(ref cause) => write!(f, "{}", cause),
            GetDataLakeSettingsError::InvalidInput(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetDataLakeSettingsError {}
/// Errors returned by GetEffectivePermissionsForPath
#[derive(Debug, PartialEq)]
pub enum GetEffectivePermissionsForPathError {
    /// <p>A specified entity does not exist</p>
    EntityNotFound(String),
    /// <p>An internal service error occurred.</p>
    InternalService(String),
    /// <p>The input provided was not valid.</p>
    InvalidInput(String),
    /// <p>The operation timed out.</p>
    OperationTimeout(String),
}

impl GetEffectivePermissionsForPathError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<GetEffectivePermissionsForPathError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "EntityNotFoundException" => {
                    return RusotoError::Service(
                        GetEffectivePermissionsForPathError::EntityNotFound(err.msg),
                    )
                }
                "InternalServiceException" => {
                    return RusotoError::Service(
                        GetEffectivePermissionsForPathError::InternalService(err.msg),
                    )
                }
                "InvalidInputException" => {
                    return RusotoError::Service(GetEffectivePermissionsForPathError::InvalidInput(
                        err.msg,
                    ))
                }
                "OperationTimeoutException" => {
                    return RusotoError::Service(
                        GetEffectivePermissionsForPathError::OperationTimeout(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for GetEffectivePermissionsForPathError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetEffectivePermissionsForPathError::EntityNotFound(ref cause) => {
                write!(f, "{}", cause)
            }
            GetEffectivePermissionsForPathError::InternalService(ref cause) => {
                write!(f, "{}", cause)
            }
            GetEffectivePermissionsForPathError::InvalidInput(ref cause) => write!(f, "{}", cause),
            GetEffectivePermissionsForPathError::OperationTimeout(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for GetEffectivePermissionsForPathError {}
/// Errors returned by GetLFTag
#[derive(Debug, PartialEq)]
pub enum GetLFTagError {
    /// <p>Access to a resource was denied.</p>
    AccessDenied(String),
    /// <p>A specified entity does not exist</p>
    EntityNotFound(String),
    /// <p>An internal service error occurred.</p>
    InternalService(String),
    /// <p>The input provided was not valid.</p>
    InvalidInput(String),
    /// <p>The operation timed out.</p>
    OperationTimeout(String),
}

impl GetLFTagError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetLFTagError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(GetLFTagError::AccessDenied(err.msg))
                }
                "EntityNotFoundException" => {
                    return RusotoError::Service(GetLFTagError::EntityNotFound(err.msg))
                }
                "InternalServiceException" => {
                    return RusotoError::Service(GetLFTagError::InternalService(err.msg))
                }
                "InvalidInputException" => {
                    return RusotoError::Service(GetLFTagError::InvalidInput(err.msg))
                }
                "OperationTimeoutException" => {
                    return RusotoError::Service(GetLFTagError::OperationTimeout(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for GetLFTagError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetLFTagError::AccessDenied(ref cause) => write!(f, "{}", cause),
            GetLFTagError::EntityNotFound(ref cause) => write!(f, "{}", cause),
            GetLFTagError::InternalService(ref cause) => write!(f, "{}", cause),
            GetLFTagError::InvalidInput(ref cause) => write!(f, "{}", cause),
            GetLFTagError::OperationTimeout(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetLFTagError {}
/// Errors returned by GetResourceLFTags
#[derive(Debug, PartialEq)]
pub enum GetResourceLFTagsError {
    /// <p>Access to a resource was denied.</p>
    AccessDenied(String),
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

impl GetResourceLFTagsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetResourceLFTagsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(GetResourceLFTagsError::AccessDenied(err.msg))
                }
                "EntityNotFoundException" => {
                    return RusotoError::Service(GetResourceLFTagsError::EntityNotFound(err.msg))
                }
                "GlueEncryptionException" => {
                    return RusotoError::Service(GetResourceLFTagsError::GlueEncryption(err.msg))
                }
                "InternalServiceException" => {
                    return RusotoError::Service(GetResourceLFTagsError::InternalService(err.msg))
                }
                "InvalidInputException" => {
                    return RusotoError::Service(GetResourceLFTagsError::InvalidInput(err.msg))
                }
                "OperationTimeoutException" => {
                    return RusotoError::Service(GetResourceLFTagsError::OperationTimeout(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for GetResourceLFTagsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetResourceLFTagsError::AccessDenied(ref cause) => write!(f, "{}", cause),
            GetResourceLFTagsError::EntityNotFound(ref cause) => write!(f, "{}", cause),
            GetResourceLFTagsError::GlueEncryption(ref cause) => write!(f, "{}", cause),
            GetResourceLFTagsError::InternalService(ref cause) => write!(f, "{}", cause),
            GetResourceLFTagsError::InvalidInput(ref cause) => write!(f, "{}", cause),
            GetResourceLFTagsError::OperationTimeout(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetResourceLFTagsError {}
/// Errors returned by GrantPermissions
#[derive(Debug, PartialEq)]
pub enum GrantPermissionsError {
    /// <p>Two processes are trying to modify a resource simultaneously.</p>
    ConcurrentModification(String),
    /// <p>A specified entity does not exist</p>
    EntityNotFound(String),
    /// <p>The input provided was not valid.</p>
    InvalidInput(String),
}

impl GrantPermissionsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GrantPermissionsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ConcurrentModificationException" => {
                    return RusotoError::Service(GrantPermissionsError::ConcurrentModification(
                        err.msg,
                    ))
                }
                "EntityNotFoundException" => {
                    return RusotoError::Service(GrantPermissionsError::EntityNotFound(err.msg))
                }
                "InvalidInputException" => {
                    return RusotoError::Service(GrantPermissionsError::InvalidInput(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for GrantPermissionsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GrantPermissionsError::ConcurrentModification(ref cause) => write!(f, "{}", cause),
            GrantPermissionsError::EntityNotFound(ref cause) => write!(f, "{}", cause),
            GrantPermissionsError::InvalidInput(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GrantPermissionsError {}
/// Errors returned by ListLFTags
#[derive(Debug, PartialEq)]
pub enum ListLFTagsError {
    /// <p>A specified entity does not exist</p>
    EntityNotFound(String),
    /// <p>An internal service error occurred.</p>
    InternalService(String),
    /// <p>The input provided was not valid.</p>
    InvalidInput(String),
    /// <p>The operation timed out.</p>
    OperationTimeout(String),
}

impl ListLFTagsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListLFTagsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "EntityNotFoundException" => {
                    return RusotoError::Service(ListLFTagsError::EntityNotFound(err.msg))
                }
                "InternalServiceException" => {
                    return RusotoError::Service(ListLFTagsError::InternalService(err.msg))
                }
                "InvalidInputException" => {
                    return RusotoError::Service(ListLFTagsError::InvalidInput(err.msg))
                }
                "OperationTimeoutException" => {
                    return RusotoError::Service(ListLFTagsError::OperationTimeout(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ListLFTagsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListLFTagsError::EntityNotFound(ref cause) => write!(f, "{}", cause),
            ListLFTagsError::InternalService(ref cause) => write!(f, "{}", cause),
            ListLFTagsError::InvalidInput(ref cause) => write!(f, "{}", cause),
            ListLFTagsError::OperationTimeout(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListLFTagsError {}
/// Errors returned by ListPermissions
#[derive(Debug, PartialEq)]
pub enum ListPermissionsError {
    /// <p>An internal service error occurred.</p>
    InternalService(String),
    /// <p>The input provided was not valid.</p>
    InvalidInput(String),
    /// <p>The operation timed out.</p>
    OperationTimeout(String),
}

impl ListPermissionsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListPermissionsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServiceException" => {
                    return RusotoError::Service(ListPermissionsError::InternalService(err.msg))
                }
                "InvalidInputException" => {
                    return RusotoError::Service(ListPermissionsError::InvalidInput(err.msg))
                }
                "OperationTimeoutException" => {
                    return RusotoError::Service(ListPermissionsError::OperationTimeout(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ListPermissionsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListPermissionsError::InternalService(ref cause) => write!(f, "{}", cause),
            ListPermissionsError::InvalidInput(ref cause) => write!(f, "{}", cause),
            ListPermissionsError::OperationTimeout(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListPermissionsError {}
/// Errors returned by ListResources
#[derive(Debug, PartialEq)]
pub enum ListResourcesError {
    /// <p>An internal service error occurred.</p>
    InternalService(String),
    /// <p>The input provided was not valid.</p>
    InvalidInput(String),
    /// <p>The operation timed out.</p>
    OperationTimeout(String),
}

impl ListResourcesError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListResourcesError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServiceException" => {
                    return RusotoError::Service(ListResourcesError::InternalService(err.msg))
                }
                "InvalidInputException" => {
                    return RusotoError::Service(ListResourcesError::InvalidInput(err.msg))
                }
                "OperationTimeoutException" => {
                    return RusotoError::Service(ListResourcesError::OperationTimeout(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ListResourcesError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListResourcesError::InternalService(ref cause) => write!(f, "{}", cause),
            ListResourcesError::InvalidInput(ref cause) => write!(f, "{}", cause),
            ListResourcesError::OperationTimeout(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListResourcesError {}
/// Errors returned by PutDataLakeSettings
#[derive(Debug, PartialEq)]
pub enum PutDataLakeSettingsError {
    /// <p>An internal service error occurred.</p>
    InternalService(String),
    /// <p>The input provided was not valid.</p>
    InvalidInput(String),
}

impl PutDataLakeSettingsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<PutDataLakeSettingsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServiceException" => {
                    return RusotoError::Service(PutDataLakeSettingsError::InternalService(err.msg))
                }
                "InvalidInputException" => {
                    return RusotoError::Service(PutDataLakeSettingsError::InvalidInput(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for PutDataLakeSettingsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            PutDataLakeSettingsError::InternalService(ref cause) => write!(f, "{}", cause),
            PutDataLakeSettingsError::InvalidInput(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for PutDataLakeSettingsError {}
/// Errors returned by RegisterResource
#[derive(Debug, PartialEq)]
pub enum RegisterResourceError {
    /// <p>A resource to be created or added already exists.</p>
    AlreadyExists(String),
    /// <p>An internal service error occurred.</p>
    InternalService(String),
    /// <p>The input provided was not valid.</p>
    InvalidInput(String),
    /// <p>The operation timed out.</p>
    OperationTimeout(String),
}

impl RegisterResourceError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<RegisterResourceError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AlreadyExistsException" => {
                    return RusotoError::Service(RegisterResourceError::AlreadyExists(err.msg))
                }
                "InternalServiceException" => {
                    return RusotoError::Service(RegisterResourceError::InternalService(err.msg))
                }
                "InvalidInputException" => {
                    return RusotoError::Service(RegisterResourceError::InvalidInput(err.msg))
                }
                "OperationTimeoutException" => {
                    return RusotoError::Service(RegisterResourceError::OperationTimeout(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for RegisterResourceError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            RegisterResourceError::AlreadyExists(ref cause) => write!(f, "{}", cause),
            RegisterResourceError::InternalService(ref cause) => write!(f, "{}", cause),
            RegisterResourceError::InvalidInput(ref cause) => write!(f, "{}", cause),
            RegisterResourceError::OperationTimeout(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for RegisterResourceError {}
/// Errors returned by RemoveLFTagsFromResource
#[derive(Debug, PartialEq)]
pub enum RemoveLFTagsFromResourceError {
    /// <p>Access to a resource was denied.</p>
    AccessDenied(String),
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
}

impl RemoveLFTagsFromResourceError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<RemoveLFTagsFromResourceError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(RemoveLFTagsFromResourceError::AccessDenied(
                        err.msg,
                    ))
                }
                "ConcurrentModificationException" => {
                    return RusotoError::Service(
                        RemoveLFTagsFromResourceError::ConcurrentModification(err.msg),
                    )
                }
                "EntityNotFoundException" => {
                    return RusotoError::Service(RemoveLFTagsFromResourceError::EntityNotFound(
                        err.msg,
                    ))
                }
                "GlueEncryptionException" => {
                    return RusotoError::Service(RemoveLFTagsFromResourceError::GlueEncryption(
                        err.msg,
                    ))
                }
                "InternalServiceException" => {
                    return RusotoError::Service(RemoveLFTagsFromResourceError::InternalService(
                        err.msg,
                    ))
                }
                "InvalidInputException" => {
                    return RusotoError::Service(RemoveLFTagsFromResourceError::InvalidInput(
                        err.msg,
                    ))
                }
                "OperationTimeoutException" => {
                    return RusotoError::Service(RemoveLFTagsFromResourceError::OperationTimeout(
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
impl fmt::Display for RemoveLFTagsFromResourceError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            RemoveLFTagsFromResourceError::AccessDenied(ref cause) => write!(f, "{}", cause),
            RemoveLFTagsFromResourceError::ConcurrentModification(ref cause) => {
                write!(f, "{}", cause)
            }
            RemoveLFTagsFromResourceError::EntityNotFound(ref cause) => write!(f, "{}", cause),
            RemoveLFTagsFromResourceError::GlueEncryption(ref cause) => write!(f, "{}", cause),
            RemoveLFTagsFromResourceError::InternalService(ref cause) => write!(f, "{}", cause),
            RemoveLFTagsFromResourceError::InvalidInput(ref cause) => write!(f, "{}", cause),
            RemoveLFTagsFromResourceError::OperationTimeout(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for RemoveLFTagsFromResourceError {}
/// Errors returned by RevokePermissions
#[derive(Debug, PartialEq)]
pub enum RevokePermissionsError {
    /// <p>Two processes are trying to modify a resource simultaneously.</p>
    ConcurrentModification(String),
    /// <p>A specified entity does not exist</p>
    EntityNotFound(String),
    /// <p>The input provided was not valid.</p>
    InvalidInput(String),
}

impl RevokePermissionsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<RevokePermissionsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ConcurrentModificationException" => {
                    return RusotoError::Service(RevokePermissionsError::ConcurrentModification(
                        err.msg,
                    ))
                }
                "EntityNotFoundException" => {
                    return RusotoError::Service(RevokePermissionsError::EntityNotFound(err.msg))
                }
                "InvalidInputException" => {
                    return RusotoError::Service(RevokePermissionsError::InvalidInput(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for RevokePermissionsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            RevokePermissionsError::ConcurrentModification(ref cause) => write!(f, "{}", cause),
            RevokePermissionsError::EntityNotFound(ref cause) => write!(f, "{}", cause),
            RevokePermissionsError::InvalidInput(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for RevokePermissionsError {}
/// Errors returned by SearchDatabasesByLFTags
#[derive(Debug, PartialEq)]
pub enum SearchDatabasesByLFTagsError {
    /// <p>Access to a resource was denied.</p>
    AccessDenied(String),
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

impl SearchDatabasesByLFTagsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<SearchDatabasesByLFTagsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(SearchDatabasesByLFTagsError::AccessDenied(
                        err.msg,
                    ))
                }
                "EntityNotFoundException" => {
                    return RusotoError::Service(SearchDatabasesByLFTagsError::EntityNotFound(
                        err.msg,
                    ))
                }
                "GlueEncryptionException" => {
                    return RusotoError::Service(SearchDatabasesByLFTagsError::GlueEncryption(
                        err.msg,
                    ))
                }
                "InternalServiceException" => {
                    return RusotoError::Service(SearchDatabasesByLFTagsError::InternalService(
                        err.msg,
                    ))
                }
                "InvalidInputException" => {
                    return RusotoError::Service(SearchDatabasesByLFTagsError::InvalidInput(
                        err.msg,
                    ))
                }
                "OperationTimeoutException" => {
                    return RusotoError::Service(SearchDatabasesByLFTagsError::OperationTimeout(
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
impl fmt::Display for SearchDatabasesByLFTagsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            SearchDatabasesByLFTagsError::AccessDenied(ref cause) => write!(f, "{}", cause),
            SearchDatabasesByLFTagsError::EntityNotFound(ref cause) => write!(f, "{}", cause),
            SearchDatabasesByLFTagsError::GlueEncryption(ref cause) => write!(f, "{}", cause),
            SearchDatabasesByLFTagsError::InternalService(ref cause) => write!(f, "{}", cause),
            SearchDatabasesByLFTagsError::InvalidInput(ref cause) => write!(f, "{}", cause),
            SearchDatabasesByLFTagsError::OperationTimeout(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for SearchDatabasesByLFTagsError {}
/// Errors returned by SearchTablesByLFTags
#[derive(Debug, PartialEq)]
pub enum SearchTablesByLFTagsError {
    /// <p>Access to a resource was denied.</p>
    AccessDenied(String),
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

impl SearchTablesByLFTagsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<SearchTablesByLFTagsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(SearchTablesByLFTagsError::AccessDenied(err.msg))
                }
                "EntityNotFoundException" => {
                    return RusotoError::Service(SearchTablesByLFTagsError::EntityNotFound(err.msg))
                }
                "GlueEncryptionException" => {
                    return RusotoError::Service(SearchTablesByLFTagsError::GlueEncryption(err.msg))
                }
                "InternalServiceException" => {
                    return RusotoError::Service(SearchTablesByLFTagsError::InternalService(
                        err.msg,
                    ))
                }
                "InvalidInputException" => {
                    return RusotoError::Service(SearchTablesByLFTagsError::InvalidInput(err.msg))
                }
                "OperationTimeoutException" => {
                    return RusotoError::Service(SearchTablesByLFTagsError::OperationTimeout(
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
impl fmt::Display for SearchTablesByLFTagsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            SearchTablesByLFTagsError::AccessDenied(ref cause) => write!(f, "{}", cause),
            SearchTablesByLFTagsError::EntityNotFound(ref cause) => write!(f, "{}", cause),
            SearchTablesByLFTagsError::GlueEncryption(ref cause) => write!(f, "{}", cause),
            SearchTablesByLFTagsError::InternalService(ref cause) => write!(f, "{}", cause),
            SearchTablesByLFTagsError::InvalidInput(ref cause) => write!(f, "{}", cause),
            SearchTablesByLFTagsError::OperationTimeout(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for SearchTablesByLFTagsError {}
/// Errors returned by UpdateLFTag
#[derive(Debug, PartialEq)]
pub enum UpdateLFTagError {
    /// <p>Access to a resource was denied.</p>
    AccessDenied(String),
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

impl UpdateLFTagError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdateLFTagError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(UpdateLFTagError::AccessDenied(err.msg))
                }
                "ConcurrentModificationException" => {
                    return RusotoError::Service(UpdateLFTagError::ConcurrentModification(err.msg))
                }
                "EntityNotFoundException" => {
                    return RusotoError::Service(UpdateLFTagError::EntityNotFound(err.msg))
                }
                "InternalServiceException" => {
                    return RusotoError::Service(UpdateLFTagError::InternalService(err.msg))
                }
                "InvalidInputException" => {
                    return RusotoError::Service(UpdateLFTagError::InvalidInput(err.msg))
                }
                "OperationTimeoutException" => {
                    return RusotoError::Service(UpdateLFTagError::OperationTimeout(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for UpdateLFTagError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UpdateLFTagError::AccessDenied(ref cause) => write!(f, "{}", cause),
            UpdateLFTagError::ConcurrentModification(ref cause) => write!(f, "{}", cause),
            UpdateLFTagError::EntityNotFound(ref cause) => write!(f, "{}", cause),
            UpdateLFTagError::InternalService(ref cause) => write!(f, "{}", cause),
            UpdateLFTagError::InvalidInput(ref cause) => write!(f, "{}", cause),
            UpdateLFTagError::OperationTimeout(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for UpdateLFTagError {}
/// Errors returned by UpdateResource
#[derive(Debug, PartialEq)]
pub enum UpdateResourceError {
    /// <p>A specified entity does not exist</p>
    EntityNotFound(String),
    /// <p>An internal service error occurred.</p>
    InternalService(String),
    /// <p>The input provided was not valid.</p>
    InvalidInput(String),
    /// <p>The operation timed out.</p>
    OperationTimeout(String),
}

impl UpdateResourceError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdateResourceError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "EntityNotFoundException" => {
                    return RusotoError::Service(UpdateResourceError::EntityNotFound(err.msg))
                }
                "InternalServiceException" => {
                    return RusotoError::Service(UpdateResourceError::InternalService(err.msg))
                }
                "InvalidInputException" => {
                    return RusotoError::Service(UpdateResourceError::InvalidInput(err.msg))
                }
                "OperationTimeoutException" => {
                    return RusotoError::Service(UpdateResourceError::OperationTimeout(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for UpdateResourceError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UpdateResourceError::EntityNotFound(ref cause) => write!(f, "{}", cause),
            UpdateResourceError::InternalService(ref cause) => write!(f, "{}", cause),
            UpdateResourceError::InvalidInput(ref cause) => write!(f, "{}", cause),
            UpdateResourceError::OperationTimeout(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for UpdateResourceError {}
/// Trait representing the capabilities of the AWS Lake Formation API. AWS Lake Formation clients implement this trait.
#[async_trait]
pub trait LakeFormation {
    /// <p>Attaches one or more tags to an existing resource.</p>
    async fn add_lf_tags_to_resource(
        &self,
        input: AddLFTagsToResourceRequest,
    ) -> Result<AddLFTagsToResourceResponse, RusotoError<AddLFTagsToResourceError>>;

    /// <p>Batch operation to grant permissions to the principal.</p>
    async fn batch_grant_permissions(
        &self,
        input: BatchGrantPermissionsRequest,
    ) -> Result<BatchGrantPermissionsResponse, RusotoError<BatchGrantPermissionsError>>;

    /// <p>Batch operation to revoke permissions from the principal.</p>
    async fn batch_revoke_permissions(
        &self,
        input: BatchRevokePermissionsRequest,
    ) -> Result<BatchRevokePermissionsResponse, RusotoError<BatchRevokePermissionsError>>;

    /// <p>Creates a tag with the specified name and values.</p>
    async fn create_lf_tag(
        &self,
        input: CreateLFTagRequest,
    ) -> Result<CreateLFTagResponse, RusotoError<CreateLFTagError>>;

    /// <p>Deletes the specified tag key name. If the attribute key does not exist or the tag does not exist, then the operation will not do anything. If the attribute key exists, then the operation checks if any resources are tagged with this attribute key, if yes, the API throws a 400 Exception with the message "Delete not allowed" as the tag key is still attached with resources. You can consider untagging resources with this tag key.</p>
    async fn delete_lf_tag(
        &self,
        input: DeleteLFTagRequest,
    ) -> Result<DeleteLFTagResponse, RusotoError<DeleteLFTagError>>;

    /// <p>Deregisters the resource as managed by the Data Catalog.</p> <p>When you deregister a path, Lake Formation removes the path from the inline policy attached to your service-linked role.</p>
    async fn deregister_resource(
        &self,
        input: DeregisterResourceRequest,
    ) -> Result<DeregisterResourceResponse, RusotoError<DeregisterResourceError>>;

    /// <p>Retrieves the current data access role for the given resource registered in AWS Lake Formation.</p>
    async fn describe_resource(
        &self,
        input: DescribeResourceRequest,
    ) -> Result<DescribeResourceResponse, RusotoError<DescribeResourceError>>;

    /// <p>Retrieves the list of the data lake administrators of a Lake Formation-managed data lake. </p>
    async fn get_data_lake_settings(
        &self,
        input: GetDataLakeSettingsRequest,
    ) -> Result<GetDataLakeSettingsResponse, RusotoError<GetDataLakeSettingsError>>;

    /// <p>Returns the Lake Formation permissions for a specified table or database resource located at a path in Amazon S3. <code>GetEffectivePermissionsForPath</code> will not return databases and tables if the catalog is encrypted.</p>
    async fn get_effective_permissions_for_path(
        &self,
        input: GetEffectivePermissionsForPathRequest,
    ) -> Result<
        GetEffectivePermissionsForPathResponse,
        RusotoError<GetEffectivePermissionsForPathError>,
    >;

    /// <p>Returns a tag definition.</p>
    async fn get_lf_tag(
        &self,
        input: GetLFTagRequest,
    ) -> Result<GetLFTagResponse, RusotoError<GetLFTagError>>;

    /// <p>Returns the tags applied to a resource.</p>
    async fn get_resource_lf_tags(
        &self,
        input: GetResourceLFTagsRequest,
    ) -> Result<GetResourceLFTagsResponse, RusotoError<GetResourceLFTagsError>>;

    /// <p>Grants permissions to the principal to access metadata in the Data Catalog and data organized in underlying data storage such as Amazon S3.</p> <p>For information about permissions, see <a href="https://docs-aws.amazon.com/lake-formation/latest/dg/security-data-access.html">Security and Access Control to Metadata and Data</a>.</p>
    async fn grant_permissions(
        &self,
        input: GrantPermissionsRequest,
    ) -> Result<GrantPermissionsResponse, RusotoError<GrantPermissionsError>>;

    /// <p>Lists tags that the requester has permission to view. </p>
    async fn list_lf_tags(
        &self,
        input: ListLFTagsRequest,
    ) -> Result<ListLFTagsResponse, RusotoError<ListLFTagsError>>;

    /// <p>Returns a list of the principal permissions on the resource, filtered by the permissions of the caller. For example, if you are granted an ALTER permission, you are able to see only the principal permissions for ALTER.</p> <p>This operation returns only those permissions that have been explicitly granted.</p> <p>For information about permissions, see <a href="https://docs-aws.amazon.com/lake-formation/latest/dg/security-data-access.html">Security and Access Control to Metadata and Data</a>.</p>
    async fn list_permissions(
        &self,
        input: ListPermissionsRequest,
    ) -> Result<ListPermissionsResponse, RusotoError<ListPermissionsError>>;

    /// <p>Lists the resources registered to be managed by the Data Catalog.</p>
    async fn list_resources(
        &self,
        input: ListResourcesRequest,
    ) -> Result<ListResourcesResponse, RusotoError<ListResourcesError>>;

    /// <p>Sets the list of data lake administrators who have admin privileges on all resources managed by Lake Formation. For more information on admin privileges, see <a href="https://docs.aws.amazon.com/lake-formation/latest/dg/lake-formation-permissions.html">Granting Lake Formation Permissions</a>.</p> <p>This API replaces the current list of data lake admins with the new list being passed. To add an admin, fetch the current list and add the new admin to that list and pass that list in this API.</p>
    async fn put_data_lake_settings(
        &self,
        input: PutDataLakeSettingsRequest,
    ) -> Result<PutDataLakeSettingsResponse, RusotoError<PutDataLakeSettingsError>>;

    /// <p>Registers the resource as managed by the Data Catalog.</p> <p>To add or update data, Lake Formation needs read/write access to the chosen Amazon S3 path. Choose a role that you know has permission to do this, or choose the AWSServiceRoleForLakeFormationDataAccess service-linked role. When you register the first Amazon S3 path, the service-linked role and a new inline policy are created on your behalf. Lake Formation adds the first path to the inline policy and attaches it to the service-linked role. When you register subsequent paths, Lake Formation adds the path to the existing policy.</p> <p>The following request registers a new location and gives AWS Lake Formation permission to use the service-linked role to access that location.</p> <p> <code>ResourceArn = arn:aws:s3:::my-bucket UseServiceLinkedRole = true</code> </p> <p>If <code>UseServiceLinkedRole</code> is not set to true, you must provide or set the <code>RoleArn</code>:</p> <p> <code>arn:aws:iam::12345:role/my-data-access-role</code> </p>
    async fn register_resource(
        &self,
        input: RegisterResourceRequest,
    ) -> Result<RegisterResourceResponse, RusotoError<RegisterResourceError>>;

    /// <p>Removes a tag from the resource. Only database, table, or tableWithColumns resource are allowed. To tag columns, use the column inclusion list in <code>tableWithColumns</code> to specify column input.</p>
    async fn remove_lf_tags_from_resource(
        &self,
        input: RemoveLFTagsFromResourceRequest,
    ) -> Result<RemoveLFTagsFromResourceResponse, RusotoError<RemoveLFTagsFromResourceError>>;

    /// <p>Revokes permissions to the principal to access metadata in the Data Catalog and data organized in underlying data storage such as Amazon S3.</p>
    async fn revoke_permissions(
        &self,
        input: RevokePermissionsRequest,
    ) -> Result<RevokePermissionsResponse, RusotoError<RevokePermissionsError>>;

    /// <p>This operation allows a search on <code>DATABASE</code> resources by <code>TagCondition</code>. This operation is used by admins who want to grant user permissions on certain <code>TagConditions</code>. Before making a grant, the admin can use <code>SearchDatabasesByTags</code> to find all resources where the given <code>TagConditions</code> are valid to verify whether the returned resources can be shared.</p>
    async fn search_databases_by_lf_tags(
        &self,
        input: SearchDatabasesByLFTagsRequest,
    ) -> Result<SearchDatabasesByLFTagsResponse, RusotoError<SearchDatabasesByLFTagsError>>;

    /// <p>This operation allows a search on <code>TABLE</code> resources by <code>LFTag</code>s. This will be used by admins who want to grant user permissions on certain LFTags. Before making a grant, the admin can use <code>SearchTablesByLFTags</code> to find all resources where the given <code>LFTag</code>s are valid to verify whether the returned resources can be shared.</p>
    async fn search_tables_by_lf_tags(
        &self,
        input: SearchTablesByLFTagsRequest,
    ) -> Result<SearchTablesByLFTagsResponse, RusotoError<SearchTablesByLFTagsError>>;

    /// <p>Updates the list of possible values for the specified tag key. If the tag does not exist, the operation throws an EntityNotFoundException. The values in the delete key values will be deleted from list of possible values. If any value in the delete key values is attached to a resource, then API errors out with a 400 Exception - "Update not allowed". Untag the attribute before deleting the tag key's value. </p>
    async fn update_lf_tag(
        &self,
        input: UpdateLFTagRequest,
    ) -> Result<UpdateLFTagResponse, RusotoError<UpdateLFTagError>>;

    /// <p>Updates the data access role used for vending access to the given (registered) resource in AWS Lake Formation. </p>
    async fn update_resource(
        &self,
        input: UpdateResourceRequest,
    ) -> Result<UpdateResourceResponse, RusotoError<UpdateResourceError>>;
}
/// A client for the AWS Lake Formation API.
#[derive(Clone)]
pub struct LakeFormationClient {
    client: Client,
    region: region::Region,
}

impl LakeFormationClient {
    /// Creates a client backed by the default tokio event loop.
    ///
    /// The client will use the default credentials provider and tls client.
    pub fn new(region: region::Region) -> LakeFormationClient {
        LakeFormationClient {
            client: Client::shared(),
            region,
        }
    }

    pub fn new_with<P, D>(
        request_dispatcher: D,
        credentials_provider: P,
        region: region::Region,
    ) -> LakeFormationClient
    where
        P: ProvideAwsCredentials + Send + Sync + 'static,
        D: DispatchSignedRequest + Send + Sync + 'static,
    {
        LakeFormationClient {
            client: Client::new_with(credentials_provider, request_dispatcher),
            region,
        }
    }

    pub fn new_with_client(client: Client, region: region::Region) -> LakeFormationClient {
        LakeFormationClient { client, region }
    }
}

#[async_trait]
impl LakeFormation for LakeFormationClient {
    /// <p>Attaches one or more tags to an existing resource.</p>
    async fn add_lf_tags_to_resource(
        &self,
        input: AddLFTagsToResourceRequest,
    ) -> Result<AddLFTagsToResourceResponse, RusotoError<AddLFTagsToResourceError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "AWSLakeFormation.AddLFTagsToResource");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, AddLFTagsToResourceError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<AddLFTagsToResourceResponse, _>()
    }

    /// <p>Batch operation to grant permissions to the principal.</p>
    async fn batch_grant_permissions(
        &self,
        input: BatchGrantPermissionsRequest,
    ) -> Result<BatchGrantPermissionsResponse, RusotoError<BatchGrantPermissionsError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "AWSLakeFormation.BatchGrantPermissions");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, BatchGrantPermissionsError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response)
            .deserialize::<BatchGrantPermissionsResponse, _>()
    }

    /// <p>Batch operation to revoke permissions from the principal.</p>
    async fn batch_revoke_permissions(
        &self,
        input: BatchRevokePermissionsRequest,
    ) -> Result<BatchRevokePermissionsResponse, RusotoError<BatchRevokePermissionsError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "AWSLakeFormation.BatchRevokePermissions");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, BatchRevokePermissionsError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response)
            .deserialize::<BatchRevokePermissionsResponse, _>()
    }

    /// <p>Creates a tag with the specified name and values.</p>
    async fn create_lf_tag(
        &self,
        input: CreateLFTagRequest,
    ) -> Result<CreateLFTagResponse, RusotoError<CreateLFTagError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "AWSLakeFormation.CreateLFTag");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, CreateLFTagError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<CreateLFTagResponse, _>()
    }

    /// <p>Deletes the specified tag key name. If the attribute key does not exist or the tag does not exist, then the operation will not do anything. If the attribute key exists, then the operation checks if any resources are tagged with this attribute key, if yes, the API throws a 400 Exception with the message "Delete not allowed" as the tag key is still attached with resources. You can consider untagging resources with this tag key.</p>
    async fn delete_lf_tag(
        &self,
        input: DeleteLFTagRequest,
    ) -> Result<DeleteLFTagResponse, RusotoError<DeleteLFTagError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "AWSLakeFormation.DeleteLFTag");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, DeleteLFTagError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<DeleteLFTagResponse, _>()
    }

    /// <p>Deregisters the resource as managed by the Data Catalog.</p> <p>When you deregister a path, Lake Formation removes the path from the inline policy attached to your service-linked role.</p>
    async fn deregister_resource(
        &self,
        input: DeregisterResourceRequest,
    ) -> Result<DeregisterResourceResponse, RusotoError<DeregisterResourceError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "AWSLakeFormation.DeregisterResource");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, DeregisterResourceError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<DeregisterResourceResponse, _>()
    }

    /// <p>Retrieves the current data access role for the given resource registered in AWS Lake Formation.</p>
    async fn describe_resource(
        &self,
        input: DescribeResourceRequest,
    ) -> Result<DescribeResourceResponse, RusotoError<DescribeResourceError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "AWSLakeFormation.DescribeResource");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, DescribeResourceError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<DescribeResourceResponse, _>()
    }

    /// <p>Retrieves the list of the data lake administrators of a Lake Formation-managed data lake. </p>
    async fn get_data_lake_settings(
        &self,
        input: GetDataLakeSettingsRequest,
    ) -> Result<GetDataLakeSettingsResponse, RusotoError<GetDataLakeSettingsError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "AWSLakeFormation.GetDataLakeSettings");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, GetDataLakeSettingsError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<GetDataLakeSettingsResponse, _>()
    }

    /// <p>Returns the Lake Formation permissions for a specified table or database resource located at a path in Amazon S3. <code>GetEffectivePermissionsForPath</code> will not return databases and tables if the catalog is encrypted.</p>
    async fn get_effective_permissions_for_path(
        &self,
        input: GetEffectivePermissionsForPathRequest,
    ) -> Result<
        GetEffectivePermissionsForPathResponse,
        RusotoError<GetEffectivePermissionsForPathError>,
    > {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "AWSLakeFormation.GetEffectivePermissionsForPath",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, GetEffectivePermissionsForPathError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response)
            .deserialize::<GetEffectivePermissionsForPathResponse, _>()
    }

    /// <p>Returns a tag definition.</p>
    async fn get_lf_tag(
        &self,
        input: GetLFTagRequest,
    ) -> Result<GetLFTagResponse, RusotoError<GetLFTagError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "AWSLakeFormation.GetLFTag");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, GetLFTagError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<GetLFTagResponse, _>()
    }

    /// <p>Returns the tags applied to a resource.</p>
    async fn get_resource_lf_tags(
        &self,
        input: GetResourceLFTagsRequest,
    ) -> Result<GetResourceLFTagsResponse, RusotoError<GetResourceLFTagsError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "AWSLakeFormation.GetResourceLFTags");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, GetResourceLFTagsError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<GetResourceLFTagsResponse, _>()
    }

    /// <p>Grants permissions to the principal to access metadata in the Data Catalog and data organized in underlying data storage such as Amazon S3.</p> <p>For information about permissions, see <a href="https://docs-aws.amazon.com/lake-formation/latest/dg/security-data-access.html">Security and Access Control to Metadata and Data</a>.</p>
    async fn grant_permissions(
        &self,
        input: GrantPermissionsRequest,
    ) -> Result<GrantPermissionsResponse, RusotoError<GrantPermissionsError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "AWSLakeFormation.GrantPermissions");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, GrantPermissionsError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<GrantPermissionsResponse, _>()
    }

    /// <p>Lists tags that the requester has permission to view. </p>
    async fn list_lf_tags(
        &self,
        input: ListLFTagsRequest,
    ) -> Result<ListLFTagsResponse, RusotoError<ListLFTagsError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "AWSLakeFormation.ListLFTags");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, ListLFTagsError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<ListLFTagsResponse, _>()
    }

    /// <p>Returns a list of the principal permissions on the resource, filtered by the permissions of the caller. For example, if you are granted an ALTER permission, you are able to see only the principal permissions for ALTER.</p> <p>This operation returns only those permissions that have been explicitly granted.</p> <p>For information about permissions, see <a href="https://docs-aws.amazon.com/lake-formation/latest/dg/security-data-access.html">Security and Access Control to Metadata and Data</a>.</p>
    async fn list_permissions(
        &self,
        input: ListPermissionsRequest,
    ) -> Result<ListPermissionsResponse, RusotoError<ListPermissionsError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "AWSLakeFormation.ListPermissions");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, ListPermissionsError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<ListPermissionsResponse, _>()
    }

    /// <p>Lists the resources registered to be managed by the Data Catalog.</p>
    async fn list_resources(
        &self,
        input: ListResourcesRequest,
    ) -> Result<ListResourcesResponse, RusotoError<ListResourcesError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "AWSLakeFormation.ListResources");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, ListResourcesError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<ListResourcesResponse, _>()
    }

    /// <p>Sets the list of data lake administrators who have admin privileges on all resources managed by Lake Formation. For more information on admin privileges, see <a href="https://docs.aws.amazon.com/lake-formation/latest/dg/lake-formation-permissions.html">Granting Lake Formation Permissions</a>.</p> <p>This API replaces the current list of data lake admins with the new list being passed. To add an admin, fetch the current list and add the new admin to that list and pass that list in this API.</p>
    async fn put_data_lake_settings(
        &self,
        input: PutDataLakeSettingsRequest,
    ) -> Result<PutDataLakeSettingsResponse, RusotoError<PutDataLakeSettingsError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "AWSLakeFormation.PutDataLakeSettings");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, PutDataLakeSettingsError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<PutDataLakeSettingsResponse, _>()
    }

    /// <p>Registers the resource as managed by the Data Catalog.</p> <p>To add or update data, Lake Formation needs read/write access to the chosen Amazon S3 path. Choose a role that you know has permission to do this, or choose the AWSServiceRoleForLakeFormationDataAccess service-linked role. When you register the first Amazon S3 path, the service-linked role and a new inline policy are created on your behalf. Lake Formation adds the first path to the inline policy and attaches it to the service-linked role. When you register subsequent paths, Lake Formation adds the path to the existing policy.</p> <p>The following request registers a new location and gives AWS Lake Formation permission to use the service-linked role to access that location.</p> <p> <code>ResourceArn = arn:aws:s3:::my-bucket UseServiceLinkedRole = true</code> </p> <p>If <code>UseServiceLinkedRole</code> is not set to true, you must provide or set the <code>RoleArn</code>:</p> <p> <code>arn:aws:iam::12345:role/my-data-access-role</code> </p>
    async fn register_resource(
        &self,
        input: RegisterResourceRequest,
    ) -> Result<RegisterResourceResponse, RusotoError<RegisterResourceError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "AWSLakeFormation.RegisterResource");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, RegisterResourceError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<RegisterResourceResponse, _>()
    }

    /// <p>Removes a tag from the resource. Only database, table, or tableWithColumns resource are allowed. To tag columns, use the column inclusion list in <code>tableWithColumns</code> to specify column input.</p>
    async fn remove_lf_tags_from_resource(
        &self,
        input: RemoveLFTagsFromResourceRequest,
    ) -> Result<RemoveLFTagsFromResourceResponse, RusotoError<RemoveLFTagsFromResourceError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "AWSLakeFormation.RemoveLFTagsFromResource");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, RemoveLFTagsFromResourceError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response)
            .deserialize::<RemoveLFTagsFromResourceResponse, _>()
    }

    /// <p>Revokes permissions to the principal to access metadata in the Data Catalog and data organized in underlying data storage such as Amazon S3.</p>
    async fn revoke_permissions(
        &self,
        input: RevokePermissionsRequest,
    ) -> Result<RevokePermissionsResponse, RusotoError<RevokePermissionsError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "AWSLakeFormation.RevokePermissions");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, RevokePermissionsError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<RevokePermissionsResponse, _>()
    }

    /// <p>This operation allows a search on <code>DATABASE</code> resources by <code>TagCondition</code>. This operation is used by admins who want to grant user permissions on certain <code>TagConditions</code>. Before making a grant, the admin can use <code>SearchDatabasesByTags</code> to find all resources where the given <code>TagConditions</code> are valid to verify whether the returned resources can be shared.</p>
    async fn search_databases_by_lf_tags(
        &self,
        input: SearchDatabasesByLFTagsRequest,
    ) -> Result<SearchDatabasesByLFTagsResponse, RusotoError<SearchDatabasesByLFTagsError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "AWSLakeFormation.SearchDatabasesByLFTags");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, SearchDatabasesByLFTagsError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response)
            .deserialize::<SearchDatabasesByLFTagsResponse, _>()
    }

    /// <p>This operation allows a search on <code>TABLE</code> resources by <code>LFTag</code>s. This will be used by admins who want to grant user permissions on certain LFTags. Before making a grant, the admin can use <code>SearchTablesByLFTags</code> to find all resources where the given <code>LFTag</code>s are valid to verify whether the returned resources can be shared.</p>
    async fn search_tables_by_lf_tags(
        &self,
        input: SearchTablesByLFTagsRequest,
    ) -> Result<SearchTablesByLFTagsResponse, RusotoError<SearchTablesByLFTagsError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "AWSLakeFormation.SearchTablesByLFTags");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, SearchTablesByLFTagsError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response)
            .deserialize::<SearchTablesByLFTagsResponse, _>()
    }

    /// <p>Updates the list of possible values for the specified tag key. If the tag does not exist, the operation throws an EntityNotFoundException. The values in the delete key values will be deleted from list of possible values. If any value in the delete key values is attached to a resource, then API errors out with a 400 Exception - "Update not allowed". Untag the attribute before deleting the tag key's value. </p>
    async fn update_lf_tag(
        &self,
        input: UpdateLFTagRequest,
    ) -> Result<UpdateLFTagResponse, RusotoError<UpdateLFTagError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "AWSLakeFormation.UpdateLFTag");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, UpdateLFTagError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<UpdateLFTagResponse, _>()
    }

    /// <p>Updates the data access role used for vending access to the given (registered) resource in AWS Lake Formation. </p>
    async fn update_resource(
        &self,
        input: UpdateResourceRequest,
    ) -> Result<UpdateResourceResponse, RusotoError<UpdateResourceError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "AWSLakeFormation.UpdateResource");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, UpdateResourceError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<UpdateResourceResponse, _>()
    }
}
