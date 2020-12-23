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

/// <p>A wildcard object, consisting of an optional list of excluded column names or indexes.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct ColumnWildcard {
    /// <p>Excludes column names. Any column with this name will be excluded.</p>
    #[serde(rename = "ExcludedColumnNames")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub excluded_column_names: Option<Vec<String>>,
}

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

/// <p>A structure containing the additional details to be returned in the <code>AdditionalDetails</code> attribute of <code>PrincipalResourcePermissions</code>.</p> <p>If a catalog resource is shared through AWS Resource Access Manager (AWS RAM), then there will exist a corresponding RAM share resource ARN.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DetailsMap {
    /// <p>A share resource ARN for a catalog resource shared through AWS Resource Access Manager (AWS RAM).</p>
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
    /// <p>This attribute can be used to return any additional details of <code>PrincipalResourcePermissions</code>. Currently returns only as a RAM share resource ARN.</p>
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

    /// <p>Grants permissions to the principal to access metadata in the Data Catalog and data organized in underlying data storage such as Amazon S3.</p> <p>For information about permissions, see <a href="https://docs-aws.amazon.com/lake-formation/latest/dg/security-data-access.html">Security and Access Control to Metadata and Data</a>.</p>
    async fn grant_permissions(
        &self,
        input: GrantPermissionsRequest,
    ) -> Result<GrantPermissionsResponse, RusotoError<GrantPermissionsError>>;

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

    /// <p>Revokes permissions to the principal to access metadata in the Data Catalog and data organized in underlying data storage such as Amazon S3.</p>
    async fn revoke_permissions(
        &self,
        input: RevokePermissionsRequest,
    ) -> Result<RevokePermissionsResponse, RusotoError<RevokePermissionsError>>;

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
