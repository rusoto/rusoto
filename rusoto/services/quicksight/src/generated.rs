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

use rusoto_core::param::{Params, ServiceParams};
use rusoto_core::proto;
use rusoto_core::signature::SignedRequest;
#[allow(unused_imports)]
use serde::{Deserialize, Serialize};
use serde_json;
/// <p>The active AWS Identity and Access Management (IAM) policy assignment.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ActiveIAMPolicyAssignment {
    /// <p>A name for the IAM policy assignment.</p>
    #[serde(rename = "AssignmentName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assignment_name: Option<String>,
    /// <p>The Amazon Resource Name (ARN) of the resource.</p>
    #[serde(rename = "PolicyArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_arn: Option<String>,
}

/// <p>Ad hoc (one-time) filtering option.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct AdHocFilteringOption {
    /// <p>Availability status.</p>
    #[serde(rename = "AvailabilityStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub availability_status: Option<String>,
}

/// <p>Amazon Elasticsearch Service parameters.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct AmazonElasticsearchParameters {
    /// <p>The Amazon Elasticsearch Service domain.</p>
    #[serde(rename = "Domain")]
    pub domain: String,
}

/// <p>Amazon Athena parameters.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct AthenaParameters {
    /// <p>The workgroup that Amazon Athena uses.</p>
    #[serde(rename = "WorkGroup")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub work_group: Option<String>,
}

/// <p>Amazon Aurora parameters.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct AuroraParameters {
    /// <p>Database.</p>
    #[serde(rename = "Database")]
    pub database: String,
    /// <p>Host.</p>
    #[serde(rename = "Host")]
    pub host: String,
    /// <p>Port.</p>
    #[serde(rename = "Port")]
    pub port: i64,
}

/// <p>Amazon Aurora with PostgreSQL compatibility parameters.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct AuroraPostgreSqlParameters {
    /// <p>Database.</p>
    #[serde(rename = "Database")]
    pub database: String,
    /// <p>Host.</p>
    #[serde(rename = "Host")]
    pub host: String,
    /// <p>Port.</p>
    #[serde(rename = "Port")]
    pub port: i64,
}

/// <p>AWS IoT Analytics parameters.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct AwsIotAnalyticsParameters {
    /// <p>Dataset name.</p>
    #[serde(rename = "DataSetName")]
    pub data_set_name: String,
}

/// <p>A calculated column for a dataset.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct CalculatedColumn {
    /// <p>A unique ID to identify a calculated column. During a dataset update, if the column ID of a calculated column matches that of an existing calculated column, Amazon QuickSight preserves the existing calculated column.</p>
    #[serde(rename = "ColumnId")]
    pub column_id: String,
    /// <p>Column name.</p>
    #[serde(rename = "ColumnName")]
    pub column_name: String,
    /// <p>An expression that defines the calculated column.</p>
    #[serde(rename = "Expression")]
    pub expression: String,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CancelIngestionRequest {
    /// <p>The AWS account ID.</p>
    #[serde(rename = "AwsAccountId")]
    pub aws_account_id: String,
    /// <p>The ID of the dataset used in the ingestion.</p>
    #[serde(rename = "DataSetId")]
    pub data_set_id: String,
    /// <p>An ID for the ingestion.</p>
    #[serde(rename = "IngestionId")]
    pub ingestion_id: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CancelIngestionResponse {
    /// <p>The Amazon Resource Name (ARN) for the data ingestion.</p>
    #[serde(rename = "Arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p>An ID for the ingestion.</p>
    #[serde(rename = "IngestionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ingestion_id: Option<String>,
    /// <p>The AWS request ID for this operation.</p>
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// <p>The HTTP status of the request.</p>
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<i64>,
}

/// <p>A transform operation that casts a column to a different type.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct CastColumnTypeOperation {
    /// <p>Column name.</p>
    #[serde(rename = "ColumnName")]
    pub column_name: String,
    /// <p>When casting a column from string to datetime type, you can supply a string in a format supported by Amazon QuickSight to denote the source data format.</p>
    #[serde(rename = "Format")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<String>,
    /// <p>New column data type.</p>
    #[serde(rename = "NewColumnType")]
    pub new_column_type: String,
}

/// <p>Groupings of columns that work together in certain Amazon QuickSight features. This is a variant type structure. For this structure to be valid, only one of the attributes can be non-null.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct ColumnGroup {
    /// <p>Geospatial column group that denotes a hierarchy.</p>
    #[serde(rename = "GeoSpatialColumnGroup")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub geo_spatial_column_group: Option<GeoSpatialColumnGroup>,
}

/// <p>A structure describing the name, data type, and geographic role of the columns.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ColumnGroupColumnSchema {
    /// <p>The name of the column group's column schema.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

/// <p>The column group schema.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ColumnGroupSchema {
    /// <p>A structure containing the list of schemas for column group columns.</p>
    #[serde(rename = "ColumnGroupColumnSchemaList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub column_group_column_schema_list: Option<Vec<ColumnGroupColumnSchema>>,
    /// <p>The name of the column group schema.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

/// <p>The column schema.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ColumnSchema {
    /// <p>The data type of the column schema.</p>
    #[serde(rename = "DataType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_type: Option<String>,
    /// <p>The geographic role of the column schema.</p>
    #[serde(rename = "GeographicRole")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub geographic_role: Option<String>,
    /// <p>The name of the column schema.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

/// <p>A tag for a column in a <code>TagColumnOperation</code> structure. This is a variant type structure. For this structure to be valid, only one of the attributes can be non-null.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct ColumnTag {
    /// <p>A geospatial role for a column.</p>
    #[serde(rename = "ColumnGeographicRole")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub column_geographic_role: Option<String>,
}

/// <p>A transform operation that creates calculated columns. Columns created in one such operation form a lexical closure.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct CreateColumnsOperation {
    /// <p>Calculated columns to create.</p>
    #[serde(rename = "Columns")]
    pub columns: Vec<CalculatedColumn>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateDashboardRequest {
    /// <p>The ID of the AWS account where you want to create the dashboard.</p>
    #[serde(rename = "AwsAccountId")]
    pub aws_account_id: String,
    /// <p>The ID for the dashboard, also added to the IAM policy.</p>
    #[serde(rename = "DashboardId")]
    pub dashboard_id: String,
    /// <p><p>Options for publishing the dashboard when you create it:</p> <ul> <li> <p> <code>AvailabilityStatus</code> for <code>AdHocFilteringOption</code> - This status can be either <code>ENABLED</code> or <code>DISABLED</code>. When this is set to <code>DISABLED</code>, QuickSight disables the left filter pane on the published dashboard, which can be used for ad hoc (one-time) filtering. This option is <code>ENABLED</code> by default. </p> </li> <li> <p> <code>AvailabilityStatus</code> for <code>ExportToCSVOption</code> - This status can be either <code>ENABLED</code> or <code>DISABLED</code>. The visual option to export data to .csv format isn&#39;t enabled when this is set to <code>DISABLED</code>. This option is <code>ENABLED</code> by default. </p> </li> <li> <p> <code>VisibilityState</code> for <code>SheetControlsOption</code> - This visibility state can be either <code>COLLAPSED</code> or <code>EXPANDED</code>. The sheet controls pane is collapsed by default when set to true. This option is <code>COLLAPSED</code> by default. </p> </li> </ul></p>
    #[serde(rename = "DashboardPublishOptions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dashboard_publish_options: Option<DashboardPublishOptions>,
    /// <p>The display name of the dashboard.</p>
    #[serde(rename = "Name")]
    pub name: String,
    /// <p>A structure that contains the parameters of the dashboard. These are parameter overrides for a dashboard. A dashboard can have any type of parameters, and some parameters might accept multiple values. You can use the dashboard permissions structure described following to override two string parameters that accept multiple values. </p>
    #[serde(rename = "Parameters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameters: Option<Parameters>,
    /// <p>A structure that contains the permissions of the dashboard. You can use this structure for granting permissions with principal and action information.</p>
    #[serde(rename = "Permissions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permissions: Option<Vec<ResourcePermission>>,
    /// <p>The source entity from which the dashboard is created. The source entity accepts the Amazon Resource Name (ARN) of the source template or analysis and also references the replacement datasets for the placeholders set when creating the template. The replacement datasets need to follow the same schema as the datasets for which placeholders were created when creating the template. </p> <p>If you are creating a dashboard from a source entity in a different AWS account, use the ARN of the source template.</p>
    #[serde(rename = "SourceEntity")]
    pub source_entity: DashboardSourceEntity,
    /// <p>Contains a map of the key-value pairs for the resource tag or tags assigned to the dashboard.</p>
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
    /// <p>A description for the first version of the dashboard being created.</p>
    #[serde(rename = "VersionDescription")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version_description: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateDashboardResponse {
    /// <p>The Amazon Resource Name (ARN) of the dashboard.</p>
    #[serde(rename = "Arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p>The status of the dashboard creation request.</p>
    #[serde(rename = "CreationStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_status: Option<String>,
    /// <p>The ID for the dashboard.</p>
    #[serde(rename = "DashboardId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dashboard_id: Option<String>,
    /// <p>The AWS request ID for this operation.</p>
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// <p>The HTTP status of the request.</p>
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<i64>,
    /// <p>The ARN of the dashboard, including the version number of the first version that is created.</p>
    #[serde(rename = "VersionArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version_arn: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateDataSetRequest {
    /// <p>The AWS account ID.</p>
    #[serde(rename = "AwsAccountId")]
    pub aws_account_id: String,
    /// <p>Groupings of columns that work together in certain QuickSight features. Currently, only geospatial hierarchy is supported.</p>
    #[serde(rename = "ColumnGroups")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub column_groups: Option<Vec<ColumnGroup>>,
    /// <p>An ID for the dataset that you want to create. This ID is unique per AWS Region for each AWS account.</p>
    #[serde(rename = "DataSetId")]
    pub data_set_id: String,
    /// <p>Indicates whether you want to import the data into SPICE.</p>
    #[serde(rename = "ImportMode")]
    pub import_mode: String,
    /// <p>Configures the combination and transformation of the data from the physical tables.</p>
    #[serde(rename = "LogicalTableMap")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logical_table_map: Option<::std::collections::HashMap<String, LogicalTable>>,
    /// <p>The display name for the dataset.</p>
    #[serde(rename = "Name")]
    pub name: String,
    /// <p>A list of resource permissions on the dataset.</p>
    #[serde(rename = "Permissions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permissions: Option<Vec<ResourcePermission>>,
    /// <p>Declares the physical tables that are available in the underlying data sources.</p>
    #[serde(rename = "PhysicalTableMap")]
    pub physical_table_map: ::std::collections::HashMap<String, PhysicalTable>,
    /// <p>The row-level security configuration for the data that you want to create.</p>
    #[serde(rename = "RowLevelPermissionDataSet")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub row_level_permission_data_set: Option<RowLevelPermissionDataSet>,
    /// <p>Contains a map of the key-value pairs for the resource tag or tags assigned to the dataset.</p>
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateDataSetResponse {
    /// <p>The Amazon Resource Name (ARN) of the dataset.</p>
    #[serde(rename = "Arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p>The ID for the dataset that you want to create. This ID is unique per AWS Region for each AWS account.</p>
    #[serde(rename = "DataSetId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_set_id: Option<String>,
    /// <p>The ARN for the ingestion, which is triggered as a result of dataset creation if the import mode is SPICE.</p>
    #[serde(rename = "IngestionArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ingestion_arn: Option<String>,
    /// <p>The ID of the ingestion, which is triggered as a result of dataset creation if the import mode is SPICE.</p>
    #[serde(rename = "IngestionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ingestion_id: Option<String>,
    /// <p>The AWS request ID for this operation.</p>
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// <p>The HTTP status of the request.</p>
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<i64>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateDataSourceRequest {
    /// <p>The AWS account ID.</p>
    #[serde(rename = "AwsAccountId")]
    pub aws_account_id: String,
    /// <p>The credentials QuickSight that uses to connect to your underlying source. Currently, only credentials based on user name and password are supported.</p>
    #[serde(rename = "Credentials")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub credentials: Option<DataSourceCredentials>,
    /// <p>An ID for the data source. This ID is unique per AWS Region for each AWS account. </p>
    #[serde(rename = "DataSourceId")]
    pub data_source_id: String,
    /// <p>The parameters that QuickSight uses to connect to your underlying source.</p>
    #[serde(rename = "DataSourceParameters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_source_parameters: Option<DataSourceParameters>,
    /// <p>A display name for the data source.</p>
    #[serde(rename = "Name")]
    pub name: String,
    /// <p>A list of resource permissions on the data source.</p>
    #[serde(rename = "Permissions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permissions: Option<Vec<ResourcePermission>>,
    /// <p>Secure Socket Layer (SSL) properties that apply when QuickSight connects to your underlying source.</p>
    #[serde(rename = "SslProperties")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ssl_properties: Option<SslProperties>,
    /// <p>Contains a map of the key-value pairs for the resource tag or tags assigned to the data source.</p>
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
    /// <p>The type of the data source. Currently, the supported types for this operation are: <code>ATHENA, AURORA, AURORA_POSTGRESQL, MARIADB, MYSQL, POSTGRESQL, PRESTO, REDSHIFT, S3, SNOWFLAKE, SPARK, SQLSERVER, TERADATA</code>. Use <code>ListDataSources</code> to return a list of all data sources.</p>
    #[serde(rename = "Type")]
    pub type_: String,
    /// <p>Use this parameter only when you want QuickSight to use a VPC connection when connecting to your underlying source.</p>
    #[serde(rename = "VpcConnectionProperties")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_connection_properties: Option<VpcConnectionProperties>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateDataSourceResponse {
    /// <p>The Amazon Resource Name (ARN) of the data source.</p>
    #[serde(rename = "Arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p>The status of creating the data source.</p>
    #[serde(rename = "CreationStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_status: Option<String>,
    /// <p>The ID of the data source. This ID is unique per AWS Region for each AWS account.</p>
    #[serde(rename = "DataSourceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_source_id: Option<String>,
    /// <p>The AWS request ID for this operation.</p>
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// <p>The HTTP status of the request.</p>
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<i64>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateGroupMembershipRequest {
    /// <p>The ID for the AWS account that the group is in. Currently, you use the ID for the AWS account that contains your Amazon QuickSight account.</p>
    #[serde(rename = "AwsAccountId")]
    pub aws_account_id: String,
    /// <p>The name of the group that you want to add the user to.</p>
    #[serde(rename = "GroupName")]
    pub group_name: String,
    /// <p>The name of the user that you want to add to the group membership.</p>
    #[serde(rename = "MemberName")]
    pub member_name: String,
    /// <p>The namespace. Currently, you should set this to <code>default</code>.</p>
    #[serde(rename = "Namespace")]
    pub namespace: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateGroupMembershipResponse {
    /// <p>The group member.</p>
    #[serde(rename = "GroupMember")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_member: Option<GroupMember>,
    /// <p>The AWS request ID for this operation.</p>
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// <p>The HTTP status of the request.</p>
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<i64>,
}

/// <p>The request object for this operation. </p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateGroupRequest {
    /// <p>The ID for the AWS account that the group is in. Currently, you use the ID for the AWS account that contains your Amazon QuickSight account.</p>
    #[serde(rename = "AwsAccountId")]
    pub aws_account_id: String,
    /// <p>A description for the group that you want to create.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>A name for the group that you want to create.</p>
    #[serde(rename = "GroupName")]
    pub group_name: String,
    /// <p>The namespace. Currently, you should set this to <code>default</code>.</p>
    #[serde(rename = "Namespace")]
    pub namespace: String,
}

/// <p>The response object for this operation.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateGroupResponse {
    /// <p>The name of the group.</p>
    #[serde(rename = "Group")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group: Option<Group>,
    /// <p>The AWS request ID for this operation.</p>
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// <p>The HTTP status of the request.</p>
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<i64>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateIAMPolicyAssignmentRequest {
    /// <p>The name of the assignment. It must be unique within an AWS account.</p>
    #[serde(rename = "AssignmentName")]
    pub assignment_name: String,
    /// <p><p>The status of the assignment. Possible values are as follows:</p> <ul> <li> <p> <code>ENABLED</code> - Anything specified in this assignment is used when creating the data source.</p> </li> <li> <p> <code>DISABLED</code> - This assignment isn&#39;t used when creating the data source.</p> </li> <li> <p> <code>DRAFT</code> - This assignment is an unfinished draft and isn&#39;t used when creating the data source.</p> </li> </ul></p>
    #[serde(rename = "AssignmentStatus")]
    pub assignment_status: String,
    /// <p>The ID of the AWS account where you want to assign an IAM policy to QuickSight users or groups.</p>
    #[serde(rename = "AwsAccountId")]
    pub aws_account_id: String,
    /// <p>The QuickSight users, groups, or both that you want to assign the policy to.</p>
    #[serde(rename = "Identities")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identities: Option<::std::collections::HashMap<String, Vec<String>>>,
    /// <p>The namespace that contains the assignment.</p>
    #[serde(rename = "Namespace")]
    pub namespace: String,
    /// <p>The ARN for the IAM policy to apply to the QuickSight users and groups specified in this assignment.</p>
    #[serde(rename = "PolicyArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_arn: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateIAMPolicyAssignmentResponse {
    /// <p>The ID for the assignment.</p>
    #[serde(rename = "AssignmentId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assignment_id: Option<String>,
    /// <p>The name of the assignment. This name must be unique within the AWS account.</p>
    #[serde(rename = "AssignmentName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assignment_name: Option<String>,
    /// <p><p>The status of the assignment. Possible values are as follows:</p> <ul> <li> <p> <code>ENABLED</code> - Anything specified in this assignment is used when creating the data source.</p> </li> <li> <p> <code>DISABLED</code> - This assignment isn&#39;t used when creating the data source.</p> </li> <li> <p> <code>DRAFT</code> - This assignment is an unfinished draft and isn&#39;t used when creating the data source.</p> </li> </ul></p>
    #[serde(rename = "AssignmentStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assignment_status: Option<String>,
    /// <p>The QuickSight users, groups, or both that the IAM policy is assigned to.</p>
    #[serde(rename = "Identities")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identities: Option<::std::collections::HashMap<String, Vec<String>>>,
    /// <p>The ARN for the IAM policy that is applied to the QuickSight users and groups specified in this assignment.</p>
    #[serde(rename = "PolicyArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_arn: Option<String>,
    /// <p>The AWS request ID for this operation.</p>
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// <p>The HTTP status of the request.</p>
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<i64>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateIngestionRequest {
    /// <p>The AWS account ID.</p>
    #[serde(rename = "AwsAccountId")]
    pub aws_account_id: String,
    /// <p>The ID of the dataset used in the ingestion.</p>
    #[serde(rename = "DataSetId")]
    pub data_set_id: String,
    /// <p>An ID for the ingestion.</p>
    #[serde(rename = "IngestionId")]
    pub ingestion_id: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateIngestionResponse {
    /// <p>The Amazon Resource Name (ARN) for the data ingestion.</p>
    #[serde(rename = "Arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p>An ID for the ingestion.</p>
    #[serde(rename = "IngestionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ingestion_id: Option<String>,
    /// <p>The ingestion status.</p>
    #[serde(rename = "IngestionStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ingestion_status: Option<String>,
    /// <p>The AWS request ID for this operation.</p>
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// <p>The HTTP status of the request.</p>
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<i64>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateTemplateAliasRequest {
    /// <p>The name that you want to give to the template alias that you're creating. Don't start the alias name with the <code>$</code> character. Alias names that start with <code>$</code> are reserved by QuickSight. </p>
    #[serde(rename = "AliasName")]
    pub alias_name: String,
    /// <p>The ID of the AWS account that contains the template that you creating an alias for.</p>
    #[serde(rename = "AwsAccountId")]
    pub aws_account_id: String,
    /// <p>An ID for the template.</p>
    #[serde(rename = "TemplateId")]
    pub template_id: String,
    /// <p>The version number of the template.</p>
    #[serde(rename = "TemplateVersionNumber")]
    pub template_version_number: i64,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateTemplateAliasResponse {
    /// <p>The AWS request ID for this operation.</p>
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// <p>The HTTP status of the request.</p>
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<i64>,
    /// <p>Information about the template alias.</p>
    #[serde(rename = "TemplateAlias")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template_alias: Option<TemplateAlias>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateTemplateRequest {
    /// <p>The ID for the AWS account that the group is in. Currently, you use the ID for the AWS account that contains your Amazon QuickSight account.</p>
    #[serde(rename = "AwsAccountId")]
    pub aws_account_id: String,
    /// <p>A display name for the template.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>A list of resource permissions to be set on the template. </p>
    #[serde(rename = "Permissions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permissions: Option<Vec<ResourcePermission>>,
    /// <p>The Amazon Resource Name (ARN) of the source entity from which this template is being created. Currently, you can create a template from an analysis or another template. If the ARN is for an analysis, include its dataset references. </p>
    #[serde(rename = "SourceEntity")]
    pub source_entity: TemplateSourceEntity,
    /// <p>Contains a map of the key-value pairs for the resource tag or tags assigned to the resource.</p>
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
    /// <p>An ID for the template that you want to create. This template is unique per AWS Region in each AWS account.</p>
    #[serde(rename = "TemplateId")]
    pub template_id: String,
    /// <p>A description of the current template version being created. This API operation creates the first version of the template. Every time <code>UpdateTemplate</code> is called, a new version is created. Each version of the template maintains a description of the version in the <code>VersionDescription</code> field.</p>
    #[serde(rename = "VersionDescription")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version_description: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateTemplateResponse {
    /// <p>The ARN for the template.</p>
    #[serde(rename = "Arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p>The template creation status.</p>
    #[serde(rename = "CreationStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_status: Option<String>,
    /// <p>The AWS request ID for this operation.</p>
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// <p>The HTTP status of the request.</p>
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<i64>,
    /// <p>The ID of the template.</p>
    #[serde(rename = "TemplateId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template_id: Option<String>,
    /// <p>The ARN for the template, including the version information of the first version.</p>
    #[serde(rename = "VersionArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version_arn: Option<String>,
}

/// <p>The combination of user name and password that are used as credentials.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CredentialPair {
    /// <p>Password.</p>
    #[serde(rename = "Password")]
    pub password: String,
    /// <p>User name.</p>
    #[serde(rename = "Username")]
    pub username: String,
}

/// <p>A physical table type built from the results of the custom SQL query.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct CustomSql {
    /// <p>The column schema from the SQL query result set.</p>
    #[serde(rename = "Columns")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub columns: Option<Vec<InputColumn>>,
    /// <p>The Amazon Resource Name (ARN) of the data source.</p>
    #[serde(rename = "DataSourceArn")]
    pub data_source_arn: String,
    /// <p>A display name for the SQL query result.</p>
    #[serde(rename = "Name")]
    pub name: String,
    /// <p>The SQL query.</p>
    #[serde(rename = "SqlQuery")]
    pub sql_query: String,
}

/// <p>Dashboard.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct Dashboard {
    /// <p>The Amazon Resource Name (ARN) of the resource.</p>
    #[serde(rename = "Arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p>The time that this dataset was created.</p>
    #[serde(rename = "CreatedTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_time: Option<f64>,
    /// <p>Dashboard ID.</p>
    #[serde(rename = "DashboardId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dashboard_id: Option<String>,
    /// <p>The last time that this dataset was published.</p>
    #[serde(rename = "LastPublishedTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_published_time: Option<f64>,
    /// <p>The last time that this dataset was updated.</p>
    #[serde(rename = "LastUpdatedTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_time: Option<f64>,
    /// <p>A display name for the dataset.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>Version.</p>
    #[serde(rename = "Version")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<DashboardVersion>,
}

/// <p>Dashboard error.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DashboardError {
    /// <p>Message.</p>
    #[serde(rename = "Message")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    /// <p>Type.</p>
    #[serde(rename = "Type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}

/// <p>Dashboard publish options.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DashboardPublishOptions {
    /// <p>Ad hoc (one-time) filtering option.</p>
    #[serde(rename = "AdHocFilteringOption")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ad_hoc_filtering_option: Option<AdHocFilteringOption>,
    /// <p>Export to .csv option.</p>
    #[serde(rename = "ExportToCSVOption")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub export_to_csv_option: Option<ExportToCSVOption>,
    /// <p>Sheet controls option.</p>
    #[serde(rename = "SheetControlsOption")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sheet_controls_option: Option<SheetControlsOption>,
}

/// <p>A filter that you apply when searching for dashboards. </p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DashboardSearchFilter {
    /// <p>The name of the value that you want to use as a filter. For example, <code>"Name": "QUICKSIGHT_USER"</code>. </p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The comparison operator that you want to use as a filter. For example, <code>"Operator": "StringEquals"</code>.</p>
    #[serde(rename = "Operator")]
    pub operator: String,
    /// <p>The value of the named item, in this case <code>QUICKSIGHT_USER</code>, that you want to use as a filter. For example, <code>"Value": "arn:aws:quicksight:us-east-1:1:user/default/UserName1"</code>. </p>
    #[serde(rename = "Value")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

/// <p>Dashboard source entity.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DashboardSourceEntity {
    /// <p>Source template.</p>
    #[serde(rename = "SourceTemplate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_template: Option<DashboardSourceTemplate>,
}

/// <p>Dashboard source template.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DashboardSourceTemplate {
    /// <p>The Amazon Resource Name (ARN) of the resource.</p>
    #[serde(rename = "Arn")]
    pub arn: String,
    /// <p>Dataset references.</p>
    #[serde(rename = "DataSetReferences")]
    pub data_set_references: Vec<DataSetReference>,
}

/// <p>Dashboard summary.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DashboardSummary {
    /// <p>The Amazon Resource Name (ARN) of the resource.</p>
    #[serde(rename = "Arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p>The time that this dashboard was created.</p>
    #[serde(rename = "CreatedTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_time: Option<f64>,
    /// <p>Dashboard ID.</p>
    #[serde(rename = "DashboardId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dashboard_id: Option<String>,
    /// <p>The last time that this dashboard was published.</p>
    #[serde(rename = "LastPublishedTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_published_time: Option<f64>,
    /// <p>The last time that this dashboard was updated.</p>
    #[serde(rename = "LastUpdatedTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_time: Option<f64>,
    /// <p>A display name for the dashboard.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>Published version number.</p>
    #[serde(rename = "PublishedVersionNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub published_version_number: Option<i64>,
}

/// <p>Dashboard version.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DashboardVersion {
    /// <p>The Amazon Resource Name (ARN) of the resource.</p>
    #[serde(rename = "Arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p>The time that this dashboard version was created.</p>
    #[serde(rename = "CreatedTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_time: Option<f64>,
    /// <p>Description.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>Errors.</p>
    #[serde(rename = "Errors")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub errors: Option<Vec<DashboardError>>,
    /// <p>Source entity ARN.</p>
    #[serde(rename = "SourceEntityArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_entity_arn: Option<String>,
    /// <p>The HTTP status of the request.</p>
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// <p>Version number.</p>
    #[serde(rename = "VersionNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version_number: Option<i64>,
}

/// <p>Dashboard version summary.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DashboardVersionSummary {
    /// <p>The Amazon Resource Name (ARN) of the resource.</p>
    #[serde(rename = "Arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p>The time that this dashboard version was created.</p>
    #[serde(rename = "CreatedTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_time: Option<f64>,
    /// <p>Description.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>Source entity ARN.</p>
    #[serde(rename = "SourceEntityArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_entity_arn: Option<String>,
    /// <p>The HTTP status of the request.</p>
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// <p>Version number.</p>
    #[serde(rename = "VersionNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version_number: Option<i64>,
}

/// <p>Dataset.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DataSet {
    /// <p>The Amazon Resource Name (ARN) of the resource.</p>
    #[serde(rename = "Arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p>Groupings of columns that work together in certain Amazon QuickSight features. Currently, only geospatial hierarchy is supported.</p>
    #[serde(rename = "ColumnGroups")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub column_groups: Option<Vec<ColumnGroup>>,
    /// <p>The amount of SPICE capacity used by this dataset. This is 0 if the dataset isn't imported into SPICE.</p>
    #[serde(rename = "ConsumedSpiceCapacityInBytes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub consumed_spice_capacity_in_bytes: Option<i64>,
    /// <p>The time that this dataset was created.</p>
    #[serde(rename = "CreatedTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_time: Option<f64>,
    /// <p>The ID of the dataset.</p>
    #[serde(rename = "DataSetId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_set_id: Option<String>,
    /// <p>Indicates whether you want to import the data into SPICE.</p>
    #[serde(rename = "ImportMode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub import_mode: Option<String>,
    /// <p>The last time that this dataset was updated.</p>
    #[serde(rename = "LastUpdatedTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_time: Option<f64>,
    /// <p>Configures the combination and transformation of the data from the physical tables.</p>
    #[serde(rename = "LogicalTableMap")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logical_table_map: Option<::std::collections::HashMap<String, LogicalTable>>,
    /// <p>A display name for the dataset.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The list of columns after all transforms. These columns are available in templates, analyses, and dashboards.</p>
    #[serde(rename = "OutputColumns")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_columns: Option<Vec<OutputColumn>>,
    /// <p>Declares the physical tables that are available in the underlying data sources.</p>
    #[serde(rename = "PhysicalTableMap")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub physical_table_map: Option<::std::collections::HashMap<String, PhysicalTable>>,
    /// <p>The row-level security configuration for the dataset.</p>
    #[serde(rename = "RowLevelPermissionDataSet")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub row_level_permission_data_set: Option<RowLevelPermissionDataSet>,
}

/// <p>Dataset configuration.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DataSetConfiguration {
    /// <p>A structure containing the list of column group schemas.</p>
    #[serde(rename = "ColumnGroupSchemaList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub column_group_schema_list: Option<Vec<ColumnGroupSchema>>,
    /// <p>Dataset schema.</p>
    #[serde(rename = "DataSetSchema")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_set_schema: Option<DataSetSchema>,
    /// <p>Placeholder.</p>
    #[serde(rename = "Placeholder")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub placeholder: Option<String>,
}

/// <p>Dataset reference.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DataSetReference {
    /// <p>Dataset Amazon Resource Name (ARN).</p>
    #[serde(rename = "DataSetArn")]
    pub data_set_arn: String,
    /// <p>Dataset placeholder.</p>
    #[serde(rename = "DataSetPlaceholder")]
    pub data_set_placeholder: String,
}

/// <p>Dataset schema.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DataSetSchema {
    /// <p>A structure containing the list of column schemas.</p>
    #[serde(rename = "ColumnSchemaList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub column_schema_list: Option<Vec<ColumnSchema>>,
}

/// <p>Dataset summary.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DataSetSummary {
    /// <p>The Amazon Resource Name (ARN) of the dataset.</p>
    #[serde(rename = "Arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p>The time that this dataset was created.</p>
    #[serde(rename = "CreatedTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_time: Option<f64>,
    /// <p>The ID of the dataset.</p>
    #[serde(rename = "DataSetId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_set_id: Option<String>,
    /// <p>Indicates whether you want to import the data into SPICE.</p>
    #[serde(rename = "ImportMode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub import_mode: Option<String>,
    /// <p>The last time that this dataset was updated.</p>
    #[serde(rename = "LastUpdatedTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_time: Option<f64>,
    /// <p>A display name for the dataset.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The row-level security configuration for the dataset.</p>
    #[serde(rename = "RowLevelPermissionDataSet")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub row_level_permission_data_set: Option<RowLevelPermissionDataSet>,
}

/// <p>The structure of a data source.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DataSource {
    /// <p>The Amazon Resource Name (ARN) of the data source.</p>
    #[serde(rename = "Arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p>The time that this data source was created.</p>
    #[serde(rename = "CreatedTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_time: Option<f64>,
    /// <p>The ID of the data source. This ID is unique per AWS Region for each AWS account.</p>
    #[serde(rename = "DataSourceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_source_id: Option<String>,
    /// <p>The parameters that Amazon QuickSight uses to connect to your underlying source. This is a variant type structure. For this structure to be valid, only one of the attributes can be non-null.</p>
    #[serde(rename = "DataSourceParameters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_source_parameters: Option<DataSourceParameters>,
    /// <p>Error information from the last update or the creation of the data source.</p>
    #[serde(rename = "ErrorInfo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_info: Option<DataSourceErrorInfo>,
    /// <p>The last time that this data source was updated.</p>
    #[serde(rename = "LastUpdatedTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_time: Option<f64>,
    /// <p>A display name for the data source.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>Secure Socket Layer (SSL) properties that apply when QuickSight connects to your underlying source.</p>
    #[serde(rename = "SslProperties")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ssl_properties: Option<SslProperties>,
    /// <p>The HTTP status of the request.</p>
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// <p>The type of the data source. This type indicates which database engine the data source connects to.</p>
    #[serde(rename = "Type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    /// <p>The VPC connection information. You need to use this parameter only when you want QuickSight to use a VPC connection when connecting to your underlying source.</p>
    #[serde(rename = "VpcConnectionProperties")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_connection_properties: Option<VpcConnectionProperties>,
}

/// <p>Data source credentials.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DataSourceCredentials {
    /// <p>Credential pair.</p>
    #[serde(rename = "CredentialPair")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub credential_pair: Option<CredentialPair>,
}

/// <p>Error information for the data source creation or update.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DataSourceErrorInfo {
    /// <p>Error message.</p>
    #[serde(rename = "Message")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    /// <p>Error type.</p>
    #[serde(rename = "Type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}

/// <p>The parameters that Amazon QuickSight uses to connect to your underlying data source. This is a variant type structure. For this structure to be valid, only one of the attributes can be non-null.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct DataSourceParameters {
    /// <p>Amazon Elasticsearch Service parameters.</p>
    #[serde(rename = "AmazonElasticsearchParameters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amazon_elasticsearch_parameters: Option<AmazonElasticsearchParameters>,
    /// <p>Amazon Athena parameters.</p>
    #[serde(rename = "AthenaParameters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub athena_parameters: Option<AthenaParameters>,
    /// <p>Amazon Aurora MySQL parameters.</p>
    #[serde(rename = "AuroraParameters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aurora_parameters: Option<AuroraParameters>,
    /// <p>Aurora PostgreSQL parameters.</p>
    #[serde(rename = "AuroraPostgreSqlParameters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aurora_postgre_sql_parameters: Option<AuroraPostgreSqlParameters>,
    /// <p>AWS IoT Analytics parameters.</p>
    #[serde(rename = "AwsIotAnalyticsParameters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aws_iot_analytics_parameters: Option<AwsIotAnalyticsParameters>,
    /// <p>Jira parameters.</p>
    #[serde(rename = "JiraParameters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub jira_parameters: Option<JiraParameters>,
    /// <p>MariaDB parameters.</p>
    #[serde(rename = "MariaDbParameters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maria_db_parameters: Option<MariaDbParameters>,
    /// <p>MySQL parameters.</p>
    #[serde(rename = "MySqlParameters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub my_sql_parameters: Option<MySqlParameters>,
    /// <p>PostgreSQL parameters.</p>
    #[serde(rename = "PostgreSqlParameters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub postgre_sql_parameters: Option<PostgreSqlParameters>,
    /// <p>Presto parameters.</p>
    #[serde(rename = "PrestoParameters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub presto_parameters: Option<PrestoParameters>,
    /// <p>Amazon RDS parameters.</p>
    #[serde(rename = "RdsParameters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rds_parameters: Option<RdsParameters>,
    /// <p>Amazon Redshift parameters.</p>
    #[serde(rename = "RedshiftParameters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub redshift_parameters: Option<RedshiftParameters>,
    /// <p>S3 parameters.</p>
    #[serde(rename = "S3Parameters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_parameters: Option<S3Parameters>,
    /// <p>ServiceNow parameters.</p>
    #[serde(rename = "ServiceNowParameters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_now_parameters: Option<ServiceNowParameters>,
    /// <p>Snowflake parameters.</p>
    #[serde(rename = "SnowflakeParameters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub snowflake_parameters: Option<SnowflakeParameters>,
    /// <p>Spark parameters.</p>
    #[serde(rename = "SparkParameters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spark_parameters: Option<SparkParameters>,
    /// <p>SQL Server parameters.</p>
    #[serde(rename = "SqlServerParameters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sql_server_parameters: Option<SqlServerParameters>,
    /// <p>Teradata parameters.</p>
    #[serde(rename = "TeradataParameters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub teradata_parameters: Option<TeradataParameters>,
    /// <p>Twitter parameters.</p>
    #[serde(rename = "TwitterParameters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub twitter_parameters: Option<TwitterParameters>,
}

/// <p>Date time parameter.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DateTimeParameter {
    /// <p>A display name for the dataset.</p>
    #[serde(rename = "Name")]
    pub name: String,
    /// <p>Values.</p>
    #[serde(rename = "Values")]
    pub values: Vec<f64>,
}

/// <p>Decimal parameter.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DecimalParameter {
    /// <p>A display name for the dataset.</p>
    #[serde(rename = "Name")]
    pub name: String,
    /// <p>Values.</p>
    #[serde(rename = "Values")]
    pub values: Vec<f64>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteDashboardRequest {
    /// <p>The ID of the AWS account that contains the dashboard that you're deleting.</p>
    #[serde(rename = "AwsAccountId")]
    pub aws_account_id: String,
    /// <p>The ID for the dashboard.</p>
    #[serde(rename = "DashboardId")]
    pub dashboard_id: String,
    /// <p>The version number of the dashboard. If the version number property is provided, only the specified version of the dashboard is deleted.</p>
    #[serde(rename = "VersionNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version_number: Option<i64>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DeleteDashboardResponse {
    /// <p>The Secure Socket Layer (SSL) properties that apply for the resource.</p>
    #[serde(rename = "Arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p>The ID of the dashboard.</p>
    #[serde(rename = "DashboardId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dashboard_id: Option<String>,
    /// <p>The AWS request ID for this operation.</p>
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// <p>The HTTP status of the request.</p>
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<i64>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteDataSetRequest {
    /// <p>The AWS account ID.</p>
    #[serde(rename = "AwsAccountId")]
    pub aws_account_id: String,
    /// <p>The ID for the dataset that you want to create. This ID is unique per AWS Region for each AWS account.</p>
    #[serde(rename = "DataSetId")]
    pub data_set_id: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DeleteDataSetResponse {
    /// <p>The Amazon Resource Name (ARN) of the dataset.</p>
    #[serde(rename = "Arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p>The ID for the dataset that you want to create. This ID is unique per AWS Region for each AWS account.</p>
    #[serde(rename = "DataSetId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_set_id: Option<String>,
    /// <p>The AWS request ID for this operation.</p>
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// <p>The HTTP status of the request.</p>
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<i64>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteDataSourceRequest {
    /// <p>The AWS account ID.</p>
    #[serde(rename = "AwsAccountId")]
    pub aws_account_id: String,
    /// <p>The ID of the data source. This ID is unique per AWS Region for each AWS account.</p>
    #[serde(rename = "DataSourceId")]
    pub data_source_id: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DeleteDataSourceResponse {
    /// <p>The Amazon Resource Name (ARN) of the data source that you deleted.</p>
    #[serde(rename = "Arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p>The ID of the data source. This ID is unique per AWS Region for each AWS account.</p>
    #[serde(rename = "DataSourceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_source_id: Option<String>,
    /// <p>The AWS request ID for this operation.</p>
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// <p>The HTTP status of the request.</p>
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<i64>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteGroupMembershipRequest {
    /// <p>The ID for the AWS account that the group is in. Currently, you use the ID for the AWS account that contains your Amazon QuickSight account.</p>
    #[serde(rename = "AwsAccountId")]
    pub aws_account_id: String,
    /// <p>The name of the group that you want to delete the user from.</p>
    #[serde(rename = "GroupName")]
    pub group_name: String,
    /// <p>The name of the user that you want to delete from the group membership.</p>
    #[serde(rename = "MemberName")]
    pub member_name: String,
    /// <p>The namespace. Currently, you should set this to <code>default</code>.</p>
    #[serde(rename = "Namespace")]
    pub namespace: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DeleteGroupMembershipResponse {
    /// <p>The AWS request ID for this operation.</p>
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// <p>The HTTP status of the request.</p>
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<i64>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteGroupRequest {
    /// <p>The ID for the AWS account that the group is in. Currently, you use the ID for the AWS account that contains your Amazon QuickSight account.</p>
    #[serde(rename = "AwsAccountId")]
    pub aws_account_id: String,
    /// <p>The name of the group that you want to delete.</p>
    #[serde(rename = "GroupName")]
    pub group_name: String,
    /// <p>The namespace. Currently, you should set this to <code>default</code>.</p>
    #[serde(rename = "Namespace")]
    pub namespace: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DeleteGroupResponse {
    /// <p>The AWS request ID for this operation.</p>
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// <p>The HTTP status of the request.</p>
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<i64>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteIAMPolicyAssignmentRequest {
    /// <p>The name of the assignment. </p>
    #[serde(rename = "AssignmentName")]
    pub assignment_name: String,
    /// <p>The AWS account ID where you want to delete the IAM policy assignment.</p>
    #[serde(rename = "AwsAccountId")]
    pub aws_account_id: String,
    /// <p>The namespace that contains the assignment.</p>
    #[serde(rename = "Namespace")]
    pub namespace: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DeleteIAMPolicyAssignmentResponse {
    /// <p>The name of the assignment. </p>
    #[serde(rename = "AssignmentName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assignment_name: Option<String>,
    /// <p>The AWS request ID for this operation.</p>
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// <p>The HTTP status of the request.</p>
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<i64>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteTemplateAliasRequest {
    /// <p>The name for the template alias. If you name a specific alias, you delete the version that the alias points to. You can specify the latest version of the template by providing the keyword <code>$LATEST</code> in the <code>AliasName</code> parameter. </p>
    #[serde(rename = "AliasName")]
    pub alias_name: String,
    /// <p>The ID of the AWS account that contains the item to delete.</p>
    #[serde(rename = "AwsAccountId")]
    pub aws_account_id: String,
    /// <p>The ID for the template that the specified alias is for.</p>
    #[serde(rename = "TemplateId")]
    pub template_id: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DeleteTemplateAliasResponse {
    /// <p>The name for the template alias.</p>
    #[serde(rename = "AliasName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alias_name: Option<String>,
    /// <p>The Amazon Resource Name (ARN) of the resource.</p>
    #[serde(rename = "Arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p>The AWS request ID for this operation.</p>
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// <p>The HTTP status of the request.</p>
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<i64>,
    /// <p>An ID for the template associated with the deletion.</p>
    #[serde(rename = "TemplateId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template_id: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteTemplateRequest {
    /// <p>The ID of the AWS account that contains the template that you're deleting.</p>
    #[serde(rename = "AwsAccountId")]
    pub aws_account_id: String,
    /// <p>An ID for the template you want to delete.</p>
    #[serde(rename = "TemplateId")]
    pub template_id: String,
    /// <p>Specifies the version of the template that you want to delete. If you don't provide a version number, <code>DeleteTemplate</code> deletes all versions of the template. </p>
    #[serde(rename = "VersionNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version_number: Option<i64>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DeleteTemplateResponse {
    /// <p>The Amazon Resource Name (ARN) of the resource.</p>
    #[serde(rename = "Arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p>The AWS request ID for this operation.</p>
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// <p>The HTTP status of the request.</p>
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<i64>,
    /// <p>An ID for the template.</p>
    #[serde(rename = "TemplateId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template_id: Option<String>,
}

/// <p><p/></p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteUserByPrincipalIdRequest {
    /// <p>The ID for the AWS account that the user is in. Currently, you use the ID for the AWS account that contains your Amazon QuickSight account.</p>
    #[serde(rename = "AwsAccountId")]
    pub aws_account_id: String,
    /// <p>The namespace. Currently, you should set this to <code>default</code>.</p>
    #[serde(rename = "Namespace")]
    pub namespace: String,
    /// <p>The principal ID of the user.</p>
    #[serde(rename = "PrincipalId")]
    pub principal_id: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DeleteUserByPrincipalIdResponse {
    /// <p>The AWS request ID for this operation.</p>
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// <p>The HTTP status of the request.</p>
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<i64>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteUserRequest {
    /// <p>The ID for the AWS account that the user is in. Currently, you use the ID for the AWS account that contains your Amazon QuickSight account.</p>
    #[serde(rename = "AwsAccountId")]
    pub aws_account_id: String,
    /// <p>The namespace. Currently, you should set this to <code>default</code>.</p>
    #[serde(rename = "Namespace")]
    pub namespace: String,
    /// <p>The name of the user that you want to delete.</p>
    #[serde(rename = "UserName")]
    pub user_name: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DeleteUserResponse {
    /// <p>The AWS request ID for this operation.</p>
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// <p>The HTTP status of the request.</p>
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<i64>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeDashboardPermissionsRequest {
    /// <p>The ID of the AWS account that contains the dashboard that you're describing permissions for.</p>
    #[serde(rename = "AwsAccountId")]
    pub aws_account_id: String,
    /// <p>The ID for the dashboard, also added to the IAM policy.</p>
    #[serde(rename = "DashboardId")]
    pub dashboard_id: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeDashboardPermissionsResponse {
    /// <p>The Amazon Resource Name (ARN) of the dashboard.</p>
    #[serde(rename = "DashboardArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dashboard_arn: Option<String>,
    /// <p>The ID for the dashboard.</p>
    #[serde(rename = "DashboardId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dashboard_id: Option<String>,
    /// <p>A structure that contains the permissions for the dashboard.</p>
    #[serde(rename = "Permissions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permissions: Option<Vec<ResourcePermission>>,
    /// <p>The AWS request ID for this operation.</p>
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// <p>The HTTP status of the request.</p>
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<i64>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeDashboardRequest {
    /// <p>The alias name.</p>
    #[serde(rename = "AliasName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alias_name: Option<String>,
    /// <p>The ID of the AWS account that contains the dashboard that you're describing.</p>
    #[serde(rename = "AwsAccountId")]
    pub aws_account_id: String,
    /// <p>The ID for the dashboard.</p>
    #[serde(rename = "DashboardId")]
    pub dashboard_id: String,
    /// <p>The version number for the dashboard. If a version number isn't passed, the latest published dashboard version is described. </p>
    #[serde(rename = "VersionNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version_number: Option<i64>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeDashboardResponse {
    /// <p>Information about the dashboard.</p>
    #[serde(rename = "Dashboard")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dashboard: Option<Dashboard>,
    /// <p>The AWS request ID for this operation.</p>
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// <p>The HTTP status of this request.</p>
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<i64>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeDataSetPermissionsRequest {
    /// <p>The AWS account ID.</p>
    #[serde(rename = "AwsAccountId")]
    pub aws_account_id: String,
    /// <p>The ID for the dataset that you want to create. This ID is unique per AWS Region for each AWS account.</p>
    #[serde(rename = "DataSetId")]
    pub data_set_id: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeDataSetPermissionsResponse {
    /// <p>The Amazon Resource Name (ARN) of the dataset.</p>
    #[serde(rename = "DataSetArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_set_arn: Option<String>,
    /// <p>The ID for the dataset that you want to create. This ID is unique per AWS Region for each AWS account.</p>
    #[serde(rename = "DataSetId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_set_id: Option<String>,
    /// <p>A list of resource permissions on the dataset.</p>
    #[serde(rename = "Permissions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permissions: Option<Vec<ResourcePermission>>,
    /// <p>The AWS request ID for this operation.</p>
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// <p>The HTTP status of the request.</p>
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<i64>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeDataSetRequest {
    /// <p>The AWS account ID.</p>
    #[serde(rename = "AwsAccountId")]
    pub aws_account_id: String,
    /// <p>The ID for the dataset that you want to create. This ID is unique per AWS Region for each AWS account.</p>
    #[serde(rename = "DataSetId")]
    pub data_set_id: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeDataSetResponse {
    /// <p>Information on the dataset.</p>
    #[serde(rename = "DataSet")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_set: Option<DataSet>,
    /// <p>The AWS request ID for this operation.</p>
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// <p>The HTTP status of the request.</p>
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<i64>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeDataSourcePermissionsRequest {
    /// <p>The AWS account ID.</p>
    #[serde(rename = "AwsAccountId")]
    pub aws_account_id: String,
    /// <p>The ID of the data source. This ID is unique per AWS Region for each AWS account.</p>
    #[serde(rename = "DataSourceId")]
    pub data_source_id: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeDataSourcePermissionsResponse {
    /// <p>The Amazon Resource Name (ARN) of the data source.</p>
    #[serde(rename = "DataSourceArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_source_arn: Option<String>,
    /// <p>The ID of the data source. This ID is unique per AWS Region for each AWS account.</p>
    #[serde(rename = "DataSourceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_source_id: Option<String>,
    /// <p>A list of resource permissions on the data source.</p>
    #[serde(rename = "Permissions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permissions: Option<Vec<ResourcePermission>>,
    /// <p>The AWS request ID for this operation.</p>
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// <p>The HTTP status of the request.</p>
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<i64>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeDataSourceRequest {
    /// <p>The AWS account ID.</p>
    #[serde(rename = "AwsAccountId")]
    pub aws_account_id: String,
    /// <p>The ID of the data source. This ID is unique per AWS Region for each AWS account.</p>
    #[serde(rename = "DataSourceId")]
    pub data_source_id: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeDataSourceResponse {
    /// <p>The information on the data source.</p>
    #[serde(rename = "DataSource")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_source: Option<DataSource>,
    /// <p>The AWS request ID for this operation.</p>
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// <p>The HTTP status of the request.</p>
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<i64>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeGroupRequest {
    /// <p>The ID for the AWS account that the group is in. Currently, you use the ID for the AWS account that contains your Amazon QuickSight account.</p>
    #[serde(rename = "AwsAccountId")]
    pub aws_account_id: String,
    /// <p>The name of the group that you want to describe.</p>
    #[serde(rename = "GroupName")]
    pub group_name: String,
    /// <p>The namespace. Currently, you should set this to <code>default</code>.</p>
    #[serde(rename = "Namespace")]
    pub namespace: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeGroupResponse {
    /// <p>The name of the group.</p>
    #[serde(rename = "Group")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group: Option<Group>,
    /// <p>The AWS request ID for this operation.</p>
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// <p>The HTTP status of the request.</p>
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<i64>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeIAMPolicyAssignmentRequest {
    /// <p>The name of the assignment. </p>
    #[serde(rename = "AssignmentName")]
    pub assignment_name: String,
    /// <p>The ID of the AWS account that contains the assignment that you want to describe.</p>
    #[serde(rename = "AwsAccountId")]
    pub aws_account_id: String,
    /// <p>The namespace that contains the assignment.</p>
    #[serde(rename = "Namespace")]
    pub namespace: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeIAMPolicyAssignmentResponse {
    /// <p>Information describing the IAM policy assignment.</p>
    #[serde(rename = "IAMPolicyAssignment")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iam_policy_assignment: Option<IAMPolicyAssignment>,
    /// <p>The AWS request ID for this operation.</p>
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// <p>The HTTP status of the request.</p>
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<i64>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeIngestionRequest {
    /// <p>The AWS account ID.</p>
    #[serde(rename = "AwsAccountId")]
    pub aws_account_id: String,
    /// <p>The ID of the dataset used in the ingestion.</p>
    #[serde(rename = "DataSetId")]
    pub data_set_id: String,
    /// <p>An ID for the ingestion.</p>
    #[serde(rename = "IngestionId")]
    pub ingestion_id: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeIngestionResponse {
    /// <p>Information about the ingestion.</p>
    #[serde(rename = "Ingestion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ingestion: Option<Ingestion>,
    /// <p>The AWS request ID for this operation.</p>
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// <p>The HTTP status of the request.</p>
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<i64>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeTemplateAliasRequest {
    /// <p>The name of the template alias that you want to describe. If you name a specific alias, you describe the version that the alias points to. You can specify the latest version of the template by providing the keyword <code>$LATEST</code> in the <code>AliasName</code> parameter. The keyword <code>$PUBLISHED</code> doesn't apply to templates.</p>
    #[serde(rename = "AliasName")]
    pub alias_name: String,
    /// <p>The ID of the AWS account that contains the template alias that you're describing.</p>
    #[serde(rename = "AwsAccountId")]
    pub aws_account_id: String,
    /// <p>The ID for the template.</p>
    #[serde(rename = "TemplateId")]
    pub template_id: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeTemplateAliasResponse {
    /// <p>The AWS request ID for this operation.</p>
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// <p>The HTTP status of the request.</p>
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<i64>,
    /// <p>Information about the template alias.</p>
    #[serde(rename = "TemplateAlias")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template_alias: Option<TemplateAlias>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeTemplatePermissionsRequest {
    /// <p>The ID of the AWS account that contains the template that you're describing.</p>
    #[serde(rename = "AwsAccountId")]
    pub aws_account_id: String,
    /// <p>The ID for the template.</p>
    #[serde(rename = "TemplateId")]
    pub template_id: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeTemplatePermissionsResponse {
    /// <p>A list of resource permissions to be set on the template. </p>
    #[serde(rename = "Permissions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permissions: Option<Vec<ResourcePermission>>,
    /// <p>The AWS request ID for this operation.</p>
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// <p>The HTTP status of the request.</p>
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<i64>,
    /// <p>The Amazon Resource Name (ARN) of the template.</p>
    #[serde(rename = "TemplateArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template_arn: Option<String>,
    /// <p>The ID for the template.</p>
    #[serde(rename = "TemplateId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template_id: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeTemplateRequest {
    /// <p>The alias of the template that you want to describe. If you name a specific alias, you describe the version that the alias points to. You can specify the latest version of the template by providing the keyword <code>$LATEST</code> in the <code>AliasName</code> parameter. The keyword <code>$PUBLISHED</code> doesn't apply to templates.</p>
    #[serde(rename = "AliasName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alias_name: Option<String>,
    /// <p>The ID of the AWS account that contains the template that you're describing.</p>
    #[serde(rename = "AwsAccountId")]
    pub aws_account_id: String,
    /// <p>The ID for the template.</p>
    #[serde(rename = "TemplateId")]
    pub template_id: String,
    /// <p>(Optional) The number for the version to describe. If a <code>VersionNumber</code> parameter value isn't provided, the latest version of the template is described.</p>
    #[serde(rename = "VersionNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version_number: Option<i64>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeTemplateResponse {
    /// <p>The HTTP status of the request.</p>
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<i64>,
    /// <p>The template structure for the object you want to describe.</p>
    #[serde(rename = "Template")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template: Option<Template>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeUserRequest {
    /// <p>The ID for the AWS account that the user is in. Currently, you use the ID for the AWS account that contains your Amazon QuickSight account.</p>
    #[serde(rename = "AwsAccountId")]
    pub aws_account_id: String,
    /// <p>The namespace. Currently, you should set this to <code>default</code>.</p>
    #[serde(rename = "Namespace")]
    pub namespace: String,
    /// <p>The name of the user that you want to describe.</p>
    #[serde(rename = "UserName")]
    pub user_name: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeUserResponse {
    /// <p>The AWS request ID for this operation.</p>
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// <p>The HTTP status of the request.</p>
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<i64>,
    /// <p>The user name.</p>
    #[serde(rename = "User")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<User>,
}

/// <p>Error information for the SPICE ingestion of a dataset.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ErrorInfo {
    /// <p>Error message.</p>
    #[serde(rename = "Message")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    /// <p>Error type.</p>
    #[serde(rename = "Type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}

/// <p>Export to .csv option.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ExportToCSVOption {
    /// <p>Availability status.</p>
    #[serde(rename = "AvailabilityStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub availability_status: Option<String>,
}

/// <p>A transform operation that filters rows based on a condition.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct FilterOperation {
    /// <p>An expression that must evaluate to a Boolean value. Rows for which the expression evaluates to true are kept in the dataset.</p>
    #[serde(rename = "ConditionExpression")]
    pub condition_expression: String,
}

/// <p>Geospatial column group that denotes a hierarchy.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct GeoSpatialColumnGroup {
    /// <p>Columns in this hierarchy.</p>
    #[serde(rename = "Columns")]
    pub columns: Vec<String>,
    /// <p>Country code.</p>
    #[serde(rename = "CountryCode")]
    pub country_code: String,
    /// <p>A display name for the hierarchy.</p>
    #[serde(rename = "Name")]
    pub name: String,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetDashboardEmbedUrlRequest {
    /// <p>The ID for the AWS account that contains the dashboard that you're embedding.</p>
    #[serde(rename = "AwsAccountId")]
    pub aws_account_id: String,
    /// <p>The ID for the dashboard, also added to the IAM policy.</p>
    #[serde(rename = "DashboardId")]
    pub dashboard_id: String,
    /// <p>The authentication method that the user uses to sign in.</p>
    #[serde(rename = "IdentityType")]
    pub identity_type: String,
    /// <p>Remove the reset button on the embedded dashboard. The default is FALSE, which enables the reset button.</p>
    #[serde(rename = "ResetDisabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reset_disabled: Option<bool>,
    /// <p>How many minutes the session is valid. The session lifetime must be 15-600 minutes.</p>
    #[serde(rename = "SessionLifetimeInMinutes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub session_lifetime_in_minutes: Option<i64>,
    /// <p>Remove the undo/redo button on the embedded dashboard. The default is FALSE, which enables the undo/redo button.</p>
    #[serde(rename = "UndoRedoDisabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub undo_redo_disabled: Option<bool>,
    /// <p><p>The Amazon QuickSight user&#39;s Amazon Resource Name (ARN), for use with <code>QUICKSIGHT</code> identity type. You can use this for any Amazon QuickSight users in your account (readers, authors, or admins) authenticated as one of the following:</p> <ul> <li> <p>Active Directory (AD) users or group members</p> </li> <li> <p>Invited nonfederated users</p> </li> <li> <p>IAM users and IAM role-based sessions authenticated through Federated Single Sign-On using SAML, OpenID Connect, or IAM federation.</p> </li> </ul></p>
    #[serde(rename = "UserArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_arn: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetDashboardEmbedUrlResponse {
    /// <p>An URL that you can put into your server-side webpage to embed your dashboard. This URL is valid for 5 minutes, and the resulting session is valid for 10 hours. The API provides the URL with an <code>auth_code</code> value that enables a single sign-on session. </p>
    #[serde(rename = "EmbedUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub embed_url: Option<String>,
    /// <p>The AWS request ID for this operation.</p>
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// <p>The HTTP status of the request.</p>
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<i64>,
}

/// <p>A <i>group</i> in Amazon QuickSight consists of a set of users. You can use groups to make it easier to manage access and security. Currently, an Amazon QuickSight subscription can't contain more than 500 Amazon QuickSight groups.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct Group {
    /// <p>The Amazon Resource Name (ARN) for the group.</p>
    #[serde(rename = "Arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p>The group description.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The name of the group.</p>
    #[serde(rename = "GroupName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_name: Option<String>,
    /// <p>The principal ID of the group.</p>
    #[serde(rename = "PrincipalId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub principal_id: Option<String>,
}

/// <p>A member of an Amazon QuickSight group. Currently, group members must be users. Groups can't be members of another group. .</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GroupMember {
    /// <p>The Amazon Resource Name (ARN) for the group member (user).</p>
    #[serde(rename = "Arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p>The name of the group member (user).</p>
    #[serde(rename = "MemberName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member_name: Option<String>,
}

/// <p>An IAM policy assignment.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct IAMPolicyAssignment {
    /// <p>Assignment ID.</p>
    #[serde(rename = "AssignmentId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assignment_id: Option<String>,
    /// <p>Assignment name.</p>
    #[serde(rename = "AssignmentName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assignment_name: Option<String>,
    /// <p>Assignment status.</p>
    #[serde(rename = "AssignmentStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assignment_status: Option<String>,
    /// <p>The AWS account ID.</p>
    #[serde(rename = "AwsAccountId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aws_account_id: Option<String>,
    /// <p>Identities.</p>
    #[serde(rename = "Identities")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identities: Option<::std::collections::HashMap<String, Vec<String>>>,
    /// <p>The Amazon Resource Name (ARN) for the IAM policy.</p>
    #[serde(rename = "PolicyArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_arn: Option<String>,
}

/// <p>IAM policy assignment summary.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct IAMPolicyAssignmentSummary {
    /// <p>Assignment name.</p>
    #[serde(rename = "AssignmentName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assignment_name: Option<String>,
    /// <p>Assignment status.</p>
    #[serde(rename = "AssignmentStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assignment_status: Option<String>,
}

/// <p>Information about the SPICE ingestion for a dataset.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct Ingestion {
    /// <p>The Amazon Resource Name (ARN) of the resource.</p>
    #[serde(rename = "Arn")]
    pub arn: String,
    /// <p>The time that this ingestion started.</p>
    #[serde(rename = "CreatedTime")]
    pub created_time: f64,
    /// <p>Error information for this ingestion.</p>
    #[serde(rename = "ErrorInfo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_info: Option<ErrorInfo>,
    /// <p>Ingestion ID.</p>
    #[serde(rename = "IngestionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ingestion_id: Option<String>,
    /// <p>The size of the data ingested, in bytes.</p>
    #[serde(rename = "IngestionSizeInBytes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ingestion_size_in_bytes: Option<i64>,
    /// <p>Ingestion status.</p>
    #[serde(rename = "IngestionStatus")]
    pub ingestion_status: String,
    /// <p>The time that this ingestion took, measured in seconds.</p>
    #[serde(rename = "IngestionTimeInSeconds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ingestion_time_in_seconds: Option<i64>,
    #[serde(rename = "QueueInfo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub queue_info: Option<QueueInfo>,
    /// <p>Event source for this ingestion.</p>
    #[serde(rename = "RequestSource")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_source: Option<String>,
    /// <p>Type of this ingestion.</p>
    #[serde(rename = "RequestType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_type: Option<String>,
    #[serde(rename = "RowInfo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub row_info: Option<RowInfo>,
}

/// <p>Metadata for a column that is used as the input of a transform operation.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct InputColumn {
    /// <p>The name of this column in the underlying data source.</p>
    #[serde(rename = "Name")]
    pub name: String,
    /// <p>The data type of the column.</p>
    #[serde(rename = "Type")]
    pub type_: String,
}

/// <p>Integer parameter.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct IntegerParameter {
    /// <p>A display name for the dataset.</p>
    #[serde(rename = "Name")]
    pub name: String,
    /// <p>Values.</p>
    #[serde(rename = "Values")]
    pub values: Vec<i64>,
}

/// <p>Jira parameters.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct JiraParameters {
    /// <p>The base URL of the Jira site.</p>
    #[serde(rename = "SiteBaseUrl")]
    pub site_base_url: String,
}

/// <p>Join instruction.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct JoinInstruction {
    /// <p>Left operand.</p>
    #[serde(rename = "LeftOperand")]
    pub left_operand: String,
    /// <p>On Clause.</p>
    #[serde(rename = "OnClause")]
    pub on_clause: String,
    /// <p>Right operand.</p>
    #[serde(rename = "RightOperand")]
    pub right_operand: String,
    /// <p>Type.</p>
    #[serde(rename = "Type")]
    pub type_: String,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListDashboardVersionsRequest {
    /// <p>The ID of the AWS account that contains the dashboard that you're listing versions for.</p>
    #[serde(rename = "AwsAccountId")]
    pub aws_account_id: String,
    /// <p>The ID for the dashboard.</p>
    #[serde(rename = "DashboardId")]
    pub dashboard_id: String,
    /// <p>The maximum number of results to be returned per request.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>The token for the next set of results, or null if there are no more results.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListDashboardVersionsResponse {
    /// <p>A structure that contains information about each version of the dashboard.</p>
    #[serde(rename = "DashboardVersionSummaryList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dashboard_version_summary_list: Option<Vec<DashboardVersionSummary>>,
    /// <p>The token for the next set of results, or null if there are no more results.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The AWS request ID for this operation.</p>
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// <p>The HTTP status of the request.</p>
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<i64>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListDashboardsRequest {
    /// <p>The ID of the AWS account that contains the dashboards that you're listing.</p>
    #[serde(rename = "AwsAccountId")]
    pub aws_account_id: String,
    /// <p>The maximum number of results to be returned per request.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>The token for the next set of results, or null if there are no more results.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListDashboardsResponse {
    /// <p>A structure that contains all of the dashboards shared with the user. This structure provides basic information about the dashboards.</p>
    #[serde(rename = "DashboardSummaryList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dashboard_summary_list: Option<Vec<DashboardSummary>>,
    /// <p>The token for the next set of results, or null if there are no more results.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The AWS request ID for this operation.</p>
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// <p>The HTTP status of the request.</p>
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<i64>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListDataSetsRequest {
    /// <p>The AWS account ID.</p>
    #[serde(rename = "AwsAccountId")]
    pub aws_account_id: String,
    /// <p>The maximum number of results to be returned per request.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>The token for the next set of results, or null if there are no more results.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListDataSetsResponse {
    /// <p>The list of dataset summaries.</p>
    #[serde(rename = "DataSetSummaries")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_set_summaries: Option<Vec<DataSetSummary>>,
    /// <p>The token for the next set of results, or null if there are no more results.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The AWS request ID for this operation.</p>
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// <p>The HTTP status of the request.</p>
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<i64>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListDataSourcesRequest {
    /// <p>The AWS account ID.</p>
    #[serde(rename = "AwsAccountId")]
    pub aws_account_id: String,
    /// <p>The maximum number of results to be returned per request.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>The token for the next set of results, or null if there are no more results.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListDataSourcesResponse {
    /// <p>A list of data sources.</p>
    #[serde(rename = "DataSources")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_sources: Option<Vec<DataSource>>,
    /// <p>The token for the next set of results, or null if there are no more results.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The AWS request ID for this operation.</p>
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// <p>The HTTP status of the request.</p>
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<i64>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListGroupMembershipsRequest {
    /// <p>The ID for the AWS account that the group is in. Currently, you use the ID for the AWS account that contains your Amazon QuickSight account.</p>
    #[serde(rename = "AwsAccountId")]
    pub aws_account_id: String,
    /// <p>The name of the group that you want to see a membership list of.</p>
    #[serde(rename = "GroupName")]
    pub group_name: String,
    /// <p>The maximum number of results to return from this request.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>The namespace. Currently, you should set this to <code>default</code>.</p>
    #[serde(rename = "Namespace")]
    pub namespace: String,
    /// <p>A pagination token that can be used in a subsequent request.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListGroupMembershipsResponse {
    /// <p>The list of the members of the group.</p>
    #[serde(rename = "GroupMemberList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_member_list: Option<Vec<GroupMember>>,
    /// <p>A pagination token that can be used in a subsequent request.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The AWS request ID for this operation.</p>
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// <p>The HTTP status of the request.</p>
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<i64>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListGroupsRequest {
    /// <p>The ID for the AWS account that the group is in. Currently, you use the ID for the AWS account that contains your Amazon QuickSight account.</p>
    #[serde(rename = "AwsAccountId")]
    pub aws_account_id: String,
    /// <p>The maximum number of results to return.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>The namespace. Currently, you should set this to <code>default</code>.</p>
    #[serde(rename = "Namespace")]
    pub namespace: String,
    /// <p>A pagination token that can be used in a subsequent request.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListGroupsResponse {
    /// <p>The list of the groups.</p>
    #[serde(rename = "GroupList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_list: Option<Vec<Group>>,
    /// <p>A pagination token that can be used in a subsequent request.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The AWS request ID for this operation.</p>
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// <p>The HTTP status of the request.</p>
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<i64>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListIAMPolicyAssignmentsForUserRequest {
    /// <p>The ID of the AWS account that contains the assignments.</p>
    #[serde(rename = "AwsAccountId")]
    pub aws_account_id: String,
    /// <p>The maximum number of results to be returned per request.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>The namespace of the assignment.</p>
    #[serde(rename = "Namespace")]
    pub namespace: String,
    /// <p>The token for the next set of results, or null if there are no more results.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The name of the user.</p>
    #[serde(rename = "UserName")]
    pub user_name: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListIAMPolicyAssignmentsForUserResponse {
    /// <p>The active assignments for this user.</p>
    #[serde(rename = "ActiveAssignments")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active_assignments: Option<Vec<ActiveIAMPolicyAssignment>>,
    /// <p>The token for the next set of results, or null if there are no more results.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The AWS request ID for this operation.</p>
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// <p>The HTTP status of the request.</p>
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<i64>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListIAMPolicyAssignmentsRequest {
    /// <p>The status of the assignments.</p>
    #[serde(rename = "AssignmentStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assignment_status: Option<String>,
    /// <p>The ID of the AWS account that contains these IAM policy assignments.</p>
    #[serde(rename = "AwsAccountId")]
    pub aws_account_id: String,
    /// <p>The maximum number of results to be returned per request.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>The namespace for the assignments.</p>
    #[serde(rename = "Namespace")]
    pub namespace: String,
    /// <p>The token for the next set of results, or null if there are no more results.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListIAMPolicyAssignmentsResponse {
    /// <p>Information describing the IAM policy assignments.</p>
    #[serde(rename = "IAMPolicyAssignments")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iam_policy_assignments: Option<Vec<IAMPolicyAssignmentSummary>>,
    /// <p>The token for the next set of results, or null if there are no more results.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The AWS request ID for this operation.</p>
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// <p>The HTTP status of the request.</p>
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<i64>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListIngestionsRequest {
    /// <p>The AWS account ID.</p>
    #[serde(rename = "AwsAccountId")]
    pub aws_account_id: String,
    /// <p>The ID of the dataset used in the ingestion.</p>
    #[serde(rename = "DataSetId")]
    pub data_set_id: String,
    /// <p>The maximum number of results to be returned per request.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>The token for the next set of results, or null if there are no more results.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListIngestionsResponse {
    /// <p>A list of the ingestions.</p>
    #[serde(rename = "Ingestions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ingestions: Option<Vec<Ingestion>>,
    /// <p>The token for the next set of results, or null if there are no more results.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The AWS request ID for this operation.</p>
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// <p>The HTTP status of the request.</p>
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<i64>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListTagsForResourceRequest {
    /// <p>The Amazon Resource Name (ARN) of the resource that you want a list of tags for.</p>
    #[serde(rename = "ResourceArn")]
    pub resource_arn: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListTagsForResourceResponse {
    /// <p>The AWS request ID for this operation.</p>
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// <p>The HTTP status of the request.</p>
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<i64>,
    /// <p>Contains a map of the key-value pairs for the resource tag or tags assigned to the resource.</p>
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListTemplateAliasesRequest {
    /// <p>The ID of the AWS account that contains the template aliases that you're listing.</p>
    #[serde(rename = "AwsAccountId")]
    pub aws_account_id: String,
    /// <p>The maximum number of results to be returned per request.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>The token for the next set of results, or null if there are no more results.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The ID for the template.</p>
    #[serde(rename = "TemplateId")]
    pub template_id: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListTemplateAliasesResponse {
    /// <p>The token for the next set of results, or null if there are no more results.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The AWS request ID for this operation.</p>
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// <p>The HTTP status of the request.</p>
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<i64>,
    /// <p>A structure containing the list of the template's aliases.</p>
    #[serde(rename = "TemplateAliasList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template_alias_list: Option<Vec<TemplateAlias>>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListTemplateVersionsRequest {
    /// <p>The ID of the AWS account that contains the templates that you're listing.</p>
    #[serde(rename = "AwsAccountId")]
    pub aws_account_id: String,
    /// <p>The maximum number of results to be returned per request.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>The token for the next set of results, or null if there are no more results.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The ID for the template.</p>
    #[serde(rename = "TemplateId")]
    pub template_id: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListTemplateVersionsResponse {
    /// <p>The token for the next set of results, or null if there are no more results.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The AWS request ID for this operation.</p>
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// <p>The HTTP status of the request.</p>
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<i64>,
    /// <p>A structure containing a list of all the versions of the specified template.</p>
    #[serde(rename = "TemplateVersionSummaryList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template_version_summary_list: Option<Vec<TemplateVersionSummary>>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListTemplatesRequest {
    /// <p>The ID of the AWS account that contains the templates that you're listing.</p>
    #[serde(rename = "AwsAccountId")]
    pub aws_account_id: String,
    /// <p>The maximum number of results to be returned per request.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>The token for the next set of results, or null if there are no more results.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListTemplatesResponse {
    /// <p>The token for the next set of results, or null if there are no more results.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The AWS request ID for this operation.</p>
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// <p>The HTTP status of the request.</p>
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<i64>,
    /// <p>A structure containing information about the templates in the list.</p>
    #[serde(rename = "TemplateSummaryList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template_summary_list: Option<Vec<TemplateSummary>>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListUserGroupsRequest {
    /// <p>The AWS account ID that the user is in. Currently, you use the ID for the AWS account that contains your Amazon QuickSight account.</p>
    #[serde(rename = "AwsAccountId")]
    pub aws_account_id: String,
    /// <p>The maximum number of results to return from this request.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>The namespace. Currently, you should set this to <code>default</code>.</p>
    #[serde(rename = "Namespace")]
    pub namespace: String,
    /// <p>A pagination token that can be used in a subsequent request.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The Amazon QuickSight user name that you want to list group memberships for.</p>
    #[serde(rename = "UserName")]
    pub user_name: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListUserGroupsResponse {
    /// <p>The list of groups the user is a member of.</p>
    #[serde(rename = "GroupList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_list: Option<Vec<Group>>,
    /// <p>A pagination token that can be used in a subsequent request.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The AWS request ID for this operation.</p>
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// <p>The HTTP status of the request.</p>
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<i64>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListUsersRequest {
    /// <p>The ID for the AWS account that the user is in. Currently, you use the ID for the AWS account that contains your Amazon QuickSight account.</p>
    #[serde(rename = "AwsAccountId")]
    pub aws_account_id: String,
    /// <p>The maximum number of results to return from this request.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>The namespace. Currently, you should set this to <code>default</code>.</p>
    #[serde(rename = "Namespace")]
    pub namespace: String,
    /// <p>A pagination token that can be used in a subsequent request.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListUsersResponse {
    /// <p>A pagination token that can be used in a subsequent request.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The AWS request ID for this operation.</p>
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// <p>The HTTP status of the request.</p>
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<i64>,
    /// <p>The list of users.</p>
    #[serde(rename = "UserList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_list: Option<Vec<User>>,
}

/// <p>A <i>logical table</i> is a unit that joins and that data transformations operate on. A logical table has a source, which can be either a physical table or result of a join. When a logical table points to a physical table, the logical table acts as a mutable copy of that physical table through transform operations.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct LogicalTable {
    /// <p>A display name for the logical table.</p>
    #[serde(rename = "Alias")]
    pub alias: String,
    /// <p>Transform operations that act on this logical table.</p>
    #[serde(rename = "DataTransforms")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_transforms: Option<Vec<TransformOperation>>,
    /// <p>Source of this logical table.</p>
    #[serde(rename = "Source")]
    pub source: LogicalTableSource,
}

/// <p>Information about the source of a logical table. This is a variant type structure. For this structure to be valid, only one of the attributes can be non-null.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct LogicalTableSource {
    /// <p>Specifies the result of a join of two logical tables.</p>
    #[serde(rename = "JoinInstruction")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub join_instruction: Option<JoinInstruction>,
    /// <p>Physical table ID.</p>
    #[serde(rename = "PhysicalTableId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub physical_table_id: Option<String>,
}

/// <p>Amazon S3 manifest file location.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct ManifestFileLocation {
    /// <p>Amazon S3 bucket.</p>
    #[serde(rename = "Bucket")]
    pub bucket: String,
    /// <p>Amazon S3 key that identifies an object.</p>
    #[serde(rename = "Key")]
    pub key: String,
}

/// <p>MariaDB parameters.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct MariaDbParameters {
    /// <p>Database.</p>
    #[serde(rename = "Database")]
    pub database: String,
    /// <p>Host.</p>
    #[serde(rename = "Host")]
    pub host: String,
    /// <p>Port.</p>
    #[serde(rename = "Port")]
    pub port: i64,
}

/// <p>MySQL parameters.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct MySqlParameters {
    /// <p>Database.</p>
    #[serde(rename = "Database")]
    pub database: String,
    /// <p>Host.</p>
    #[serde(rename = "Host")]
    pub host: String,
    /// <p>Port.</p>
    #[serde(rename = "Port")]
    pub port: i64,
}

/// <p>Output column.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct OutputColumn {
    /// <p>A display name for the dataset.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>Type.</p>
    #[serde(rename = "Type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}

/// <p>Parameters.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct Parameters {
    /// <p>DateTime parameters.</p>
    #[serde(rename = "DateTimeParameters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date_time_parameters: Option<Vec<DateTimeParameter>>,
    /// <p>Decimal parameters.</p>
    #[serde(rename = "DecimalParameters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub decimal_parameters: Option<Vec<DecimalParameter>>,
    /// <p>Integer parameters.</p>
    #[serde(rename = "IntegerParameters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub integer_parameters: Option<Vec<IntegerParameter>>,
    /// <p>String parameters.</p>
    #[serde(rename = "StringParameters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub string_parameters: Option<Vec<StringParameter>>,
}

/// <p>A view of a data source that contains information about the shape of the data in the underlying source. This is a variant type structure. For this structure to be valid, only one of the attributes can be non-null.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct PhysicalTable {
    /// <p>A physical table type built from the results of the custom SQL query.</p>
    #[serde(rename = "CustomSql")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_sql: Option<CustomSql>,
    /// <p>A physical table type for relational data sources.</p>
    #[serde(rename = "RelationalTable")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub relational_table: Option<RelationalTable>,
    /// <p>A physical table type for as S3 data source.</p>
    #[serde(rename = "S3Source")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_source: Option<S3Source>,
}

/// <p>PostgreSQL parameters.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct PostgreSqlParameters {
    /// <p>Database.</p>
    #[serde(rename = "Database")]
    pub database: String,
    /// <p>Host.</p>
    #[serde(rename = "Host")]
    pub host: String,
    /// <p>Port.</p>
    #[serde(rename = "Port")]
    pub port: i64,
}

/// <p>Presto parameters.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct PrestoParameters {
    /// <p>Catalog.</p>
    #[serde(rename = "Catalog")]
    pub catalog: String,
    /// <p>Host.</p>
    #[serde(rename = "Host")]
    pub host: String,
    /// <p>Port.</p>
    #[serde(rename = "Port")]
    pub port: i64,
}

/// <p>A transform operation that projects columns. Operations that come after a projection can only refer to projected columns.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct ProjectOperation {
    /// <p>Projected columns.</p>
    #[serde(rename = "ProjectedColumns")]
    pub projected_columns: Vec<String>,
}

/// <p>Information about a queued dataset SPICE ingestion.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct QueueInfo {
    /// <p>The ID of the ongoing ingestion. The queued ingestion is waiting for the ongoing ingestion to complete.</p>
    #[serde(rename = "QueuedIngestion")]
    pub queued_ingestion: String,
    /// <p>The ID of the queued ingestion.</p>
    #[serde(rename = "WaitingOnIngestion")]
    pub waiting_on_ingestion: String,
}

/// <p>Amazon RDS parameters.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct RdsParameters {
    /// <p>Database.</p>
    #[serde(rename = "Database")]
    pub database: String,
    /// <p>Instance ID.</p>
    #[serde(rename = "InstanceId")]
    pub instance_id: String,
}

/// <p>Amazon Redshift parameters. The <code>ClusterId</code> field can be blank if <code>Host</code> and <code>Port</code> are both set. The <code>Host</code> and <code>Port</code> fields can be blank if the <code>ClusterId</code> field is set.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct RedshiftParameters {
    /// <p>Cluster ID. This field can be blank if the <code>Host</code> and <code>Port</code> are provided.</p>
    #[serde(rename = "ClusterId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_id: Option<String>,
    /// <p>Database.</p>
    #[serde(rename = "Database")]
    pub database: String,
    /// <p>Host. This field can be blank if <code>ClusterId</code> is provided.</p>
    #[serde(rename = "Host")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub host: Option<String>,
    /// <p>Port. This field can be blank if the <code>ClusterId</code> is provided.</p>
    #[serde(rename = "Port")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub port: Option<i64>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct RegisterUserRequest {
    /// <p>The ID for the AWS account that the user is in. Currently, you use the ID for the AWS account that contains your Amazon QuickSight account.</p>
    #[serde(rename = "AwsAccountId")]
    pub aws_account_id: String,
    /// <p>The email address of the user that you want to register.</p>
    #[serde(rename = "Email")]
    pub email: String,
    /// <p>The ARN of the IAM user or role that you are registering with Amazon QuickSight. </p>
    #[serde(rename = "IamArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iam_arn: Option<String>,
    /// <p><p>Amazon QuickSight supports several ways of managing the identity of users. This parameter accepts two values:</p> <ul> <li> <p> <code>IAM</code>: A user whose identity maps to an existing IAM user or role. </p> </li> <li> <p> <code>QUICKSIGHT</code>: A user whose identity is owned and managed internally by Amazon QuickSight. </p> </li> </ul></p>
    #[serde(rename = "IdentityType")]
    pub identity_type: String,
    /// <p>The namespace. Currently, you should set this to <code>default</code>.</p>
    #[serde(rename = "Namespace")]
    pub namespace: String,
    /// <p>You need to use this parameter only when you register one or more users using an assumed IAM role. You don't need to provide the session name for other scenarios, for example when you are registering an IAM user or an Amazon QuickSight user. You can register multiple users using the same IAM role if each user has a different session name. For more information on assuming IAM roles, see <a href="https://docs.aws.example.com/cli/latest/reference/sts/assume-role.html"> <code>assume-role</code> </a> in the <i>AWS CLI Reference.</i> </p>
    #[serde(rename = "SessionName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub session_name: Option<String>,
    /// <p>The Amazon QuickSight user name that you want to create for the user you are registering.</p>
    #[serde(rename = "UserName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_name: Option<String>,
    /// <p><p>The Amazon QuickSight role for the user. The user role can be one of the following:</p> <ul> <li> <p> <code>READER</code>: A user who has read-only access to dashboards.</p> </li> <li> <p> <code>AUTHOR</code>: A user who can create data sources, datasets, analyses, and dashboards.</p> </li> <li> <p> <code>ADMIN</code>: A user who is an author, who can also manage Amazon QuickSight settings.</p> </li> <li> <p> <code>RESTRICTED<em>READER</code>: This role isn&#39;t currently available for use.</p> </li> <li> <p> <code>RESTRICTED</em>AUTHOR</code>: This role isn&#39;t currently available for use.</p> </li> </ul></p>
    #[serde(rename = "UserRole")]
    pub user_role: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct RegisterUserResponse {
    /// <p>The AWS request ID for this operation.</p>
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// <p>The HTTP status of the request.</p>
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<i64>,
    /// <p>The user name.</p>
    #[serde(rename = "User")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<User>,
    /// <p>The URL the user visits to complete registration and provide a password. This is returned only for users with an identity type of <code>QUICKSIGHT</code>.</p>
    #[serde(rename = "UserInvitationUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_invitation_url: Option<String>,
}

/// <p>A physical table type for relational data sources.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct RelationalTable {
    /// <p>The Amazon Resource Name (ARN) for the data source.</p>
    #[serde(rename = "DataSourceArn")]
    pub data_source_arn: String,
    /// <p>The column schema of the table.</p>
    #[serde(rename = "InputColumns")]
    pub input_columns: Vec<InputColumn>,
    /// <p>The name of the relational table.</p>
    #[serde(rename = "Name")]
    pub name: String,
    /// <p>The schema name. This name applies to certain relational database engines.</p>
    #[serde(rename = "Schema")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schema: Option<String>,
}

/// <p>A transform operation that renames a column.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct RenameColumnOperation {
    /// <p>The name of the column to be renamed.</p>
    #[serde(rename = "ColumnName")]
    pub column_name: String,
    /// <p>The new name for the column.</p>
    #[serde(rename = "NewColumnName")]
    pub new_column_name: String,
}

/// <p>Permission for the resource.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct ResourcePermission {
    /// <p>The action to grant or revoke permissions on, for example <code>"quicksight:DescribeDashboard"</code>.</p>
    #[serde(rename = "Actions")]
    pub actions: Vec<String>,
    /// <p>The Amazon Resource Name (ARN) of an Amazon QuickSight user or group, or an IAM ARN. If you are using cross-account resource sharing, this is the IAM ARN of an account root. Otherwise, it is the ARN of a QuickSight user or group. .</p>
    #[serde(rename = "Principal")]
    pub principal: String,
}

/// <p>Information about rows for a data set SPICE ingestion.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct RowInfo {
    /// <p>The number of rows that were not ingested.</p>
    #[serde(rename = "RowsDropped")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rows_dropped: Option<i64>,
    /// <p>The number of rows that were ingested.</p>
    #[serde(rename = "RowsIngested")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rows_ingested: Option<i64>,
}

/// <p>The row-level security configuration for the dataset.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct RowLevelPermissionDataSet {
    /// <p>The Amazon Resource Name (ARN) of the permission dataset.</p>
    #[serde(rename = "Arn")]
    pub arn: String,
    /// <p>Permission policy.</p>
    #[serde(rename = "PermissionPolicy")]
    pub permission_policy: String,
}

/// <p>S3 parameters.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct S3Parameters {
    /// <p>Location of the Amazon S3 manifest file. This is NULL if the manifest file was uploaded in the console.</p>
    #[serde(rename = "ManifestFileLocation")]
    pub manifest_file_location: ManifestFileLocation,
}

/// <p>A physical table type for as S3 data source.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct S3Source {
    /// <p>The amazon Resource Name (ARN) for the data source.</p>
    #[serde(rename = "DataSourceArn")]
    pub data_source_arn: String,
    /// <p>A physical table type for as S3 data source.</p>
    #[serde(rename = "InputColumns")]
    pub input_columns: Vec<InputColumn>,
    /// <p>Information about the format for the S3 source file or files.</p>
    #[serde(rename = "UploadSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub upload_settings: Option<UploadSettings>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct SearchDashboardsRequest {
    /// <p>The ID of the AWS account that contains the user whose dashboards you're searching for. </p>
    #[serde(rename = "AwsAccountId")]
    pub aws_account_id: String,
    /// <p>The filters to apply to the search. Currently, you can search only by user name. For example, <code>"Filters": [ { "Name": "QUICKSIGHT_USER", "Operator": "StringEquals", "Value": "arn:aws:quicksight:us-east-1:1:user/default/UserName1" } ]</code> </p>
    #[serde(rename = "Filters")]
    pub filters: Vec<DashboardSearchFilter>,
    /// <p>The maximum number of results to be returned per request.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>The token for the next set of results, or null if there are no more results.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct SearchDashboardsResponse {
    /// <p>The list of dashboards owned by the user specified in <code>Filters</code> in your request.</p>
    #[serde(rename = "DashboardSummaryList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dashboard_summary_list: Option<Vec<DashboardSummary>>,
    /// <p>The token for the next set of results, or null if there are no more results.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The AWS request ID for this operation.</p>
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// <p>The HTTP status of the request.</p>
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<i64>,
}

/// <p>ServiceNow parameters.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct ServiceNowParameters {
    /// <p>URL of the base site.</p>
    #[serde(rename = "SiteBaseUrl")]
    pub site_base_url: String,
}

/// <p>Sheet controls option.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct SheetControlsOption {
    /// <p>Visibility state.</p>
    #[serde(rename = "VisibilityState")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub visibility_state: Option<String>,
}

/// <p>Snowflake parameters.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct SnowflakeParameters {
    /// <p>Database.</p>
    #[serde(rename = "Database")]
    pub database: String,
    /// <p>Host.</p>
    #[serde(rename = "Host")]
    pub host: String,
    /// <p>Warehouse.</p>
    #[serde(rename = "Warehouse")]
    pub warehouse: String,
}

/// <p>Spark parameters.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct SparkParameters {
    /// <p>Host.</p>
    #[serde(rename = "Host")]
    pub host: String,
    /// <p>Port.</p>
    #[serde(rename = "Port")]
    pub port: i64,
}

/// <p>SQL Server parameters.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct SqlServerParameters {
    /// <p>Database.</p>
    #[serde(rename = "Database")]
    pub database: String,
    /// <p>Host.</p>
    #[serde(rename = "Host")]
    pub host: String,
    /// <p>Port.</p>
    #[serde(rename = "Port")]
    pub port: i64,
}

/// <p>Secure Socket Layer (SSL) properties that apply when QuickSight connects to your underlying data source.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct SslProperties {
    /// <p>A Boolean option to control whether SSL should be disabled.</p>
    #[serde(rename = "DisableSsl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_ssl: Option<bool>,
}

/// <p>String parameter.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct StringParameter {
    /// <p>A display name for the dataset.</p>
    #[serde(rename = "Name")]
    pub name: String,
    /// <p>Values.</p>
    #[serde(rename = "Values")]
    pub values: Vec<String>,
}

/// <p>The key or keys of the key-value pairs for the resource tag or tags assigned to the resource.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct Tag {
    /// <p>Tag key.</p>
    #[serde(rename = "Key")]
    pub key: String,
    /// <p>Tag value.</p>
    #[serde(rename = "Value")]
    pub value: String,
}

/// <p>A transform operation that tags a column with additional information.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct TagColumnOperation {
    /// <p>The column that this operation acts on.</p>
    #[serde(rename = "ColumnName")]
    pub column_name: String,
    /// <p><p>The dataset column tag, currently only used for geospatial type tagging. .</p> <note> <p>This is not tags for the AWS tagging feature. .</p> </note></p>
    #[serde(rename = "Tags")]
    pub tags: Vec<ColumnTag>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct TagResourceRequest {
    /// <p>The Amazon Resource Name (ARN) of the resource that you want to tag.</p>
    #[serde(rename = "ResourceArn")]
    pub resource_arn: String,
    /// <p>Contains a map of the key-value pairs for the resource tag or tags assigned to the resource.</p>
    #[serde(rename = "Tags")]
    pub tags: Vec<Tag>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct TagResourceResponse {
    /// <p>The AWS request ID for this operation.</p>
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// <p>The HTTP status of the request.</p>
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<i64>,
}

/// <p>A template object. A <i>template</i> is an entity in QuickSight that encapsulates the metadata required to create an analysis and that you can use to create a dashboard. A template adds a layer of abstraction by using placeholders to replace the dataset associated with the analysis. You can use templates to create dashboards by replacing dataset placeholders with datasets that follow the same schema that was used to create the source analysis and template.</p> <p>You can share templates across AWS accounts by allowing users in other AWS accounts to create a template or a dashboard from an existing template.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct Template {
    /// <p>The Amazon Resource Name (ARN) of the template.</p>
    #[serde(rename = "Arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p>Time when this was created.</p>
    #[serde(rename = "CreatedTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_time: Option<f64>,
    /// <p>Time when this was last updated.</p>
    #[serde(rename = "LastUpdatedTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_time: Option<f64>,
    /// <p>The display name of the template.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The ID for the template. This is unique per AWS Region for each AWS account.</p>
    #[serde(rename = "TemplateId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template_id: Option<String>,
    /// <p>A structure describing the versions of the template.</p>
    #[serde(rename = "Version")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<TemplateVersion>,
}

/// <p>The template alias.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct TemplateAlias {
    /// <p>The display name of the template alias.</p>
    #[serde(rename = "AliasName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alias_name: Option<String>,
    /// <p>The Amazon Resource Name (ARN) of the template alias.</p>
    #[serde(rename = "Arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p>The version number of the template alias.</p>
    #[serde(rename = "TemplateVersionNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template_version_number: Option<i64>,
}

/// <p>List of errors that occurred when the template version creation failed.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct TemplateError {
    /// <p>Description of the error type.</p>
    #[serde(rename = "Message")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    /// <p>Type of error.</p>
    #[serde(rename = "Type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}

/// <p>The source analysis of the template.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct TemplateSourceAnalysis {
    /// <p>The Amazon Resource Name (ARN) of the resource.</p>
    #[serde(rename = "Arn")]
    pub arn: String,
    /// <p>A structure containing information about the dataset references used as placeholders in the template.</p>
    #[serde(rename = "DataSetReferences")]
    pub data_set_references: Vec<DataSetReference>,
}

/// <p>The source entity of the template.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct TemplateSourceEntity {
    /// <p>The source analysis, if it is based on an analysis.</p>
    #[serde(rename = "SourceAnalysis")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_analysis: Option<TemplateSourceAnalysis>,
    /// <p>The source template, if it is based on an template.</p>
    #[serde(rename = "SourceTemplate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_template: Option<TemplateSourceTemplate>,
}

/// <p>The source template of the template.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct TemplateSourceTemplate {
    /// <p>The Amazon Resource Name (ARN) of the resource.</p>
    #[serde(rename = "Arn")]
    pub arn: String,
}

/// <p>The template summary.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct TemplateSummary {
    /// <p>A summary of a template.</p>
    #[serde(rename = "Arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p>The last time that this template was created.</p>
    #[serde(rename = "CreatedTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_time: Option<f64>,
    /// <p>The last time that this template was updated.</p>
    #[serde(rename = "LastUpdatedTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_time: Option<f64>,
    /// <p>A structure containing a list of version numbers for the template summary.</p>
    #[serde(rename = "LatestVersionNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latest_version_number: Option<i64>,
    /// <p>A display name for the template.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The ID of the template. This ID is unique per AWS Region for each AWS account.</p>
    #[serde(rename = "TemplateId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template_id: Option<String>,
}

/// <p>A version of a template.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct TemplateVersion {
    /// <p>The time that this template version was created.</p>
    #[serde(rename = "CreatedTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_time: Option<f64>,
    /// <p>Schema of the dataset identified by the placeholder. The idea is that any dashboard created from the template should be bound to new datasets matching the same schema described through this API. .</p>
    #[serde(rename = "DataSetConfigurations")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_set_configurations: Option<Vec<DataSetConfiguration>>,
    /// <p>The description of the template.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>Errors associated with the template.</p>
    #[serde(rename = "Errors")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub errors: Option<Vec<TemplateError>>,
    /// <p>The Amazon Resource Name (ARN) of the analysis or template which was used to create this template.</p>
    #[serde(rename = "SourceEntityArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_entity_arn: Option<String>,
    /// <p>The HTTP status of the request.</p>
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// <p>The version number of the template.</p>
    #[serde(rename = "VersionNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version_number: Option<i64>,
}

/// <p>The template version.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct TemplateVersionSummary {
    /// <p>The ARN of the template version.</p>
    #[serde(rename = "Arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p>The time that this template version was created.</p>
    #[serde(rename = "CreatedTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_time: Option<f64>,
    /// <p>The description of the template version.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The status of the template version.</p>
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// <p>The version number of the template version.</p>
    #[serde(rename = "VersionNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version_number: Option<i64>,
}

/// <p>Teradata parameters.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct TeradataParameters {
    /// <p>Database.</p>
    #[serde(rename = "Database")]
    pub database: String,
    /// <p>Host.</p>
    #[serde(rename = "Host")]
    pub host: String,
    /// <p>Port.</p>
    #[serde(rename = "Port")]
    pub port: i64,
}

/// <p>A data transformation on a logical table. This is a variant type structure. For this structure to be valid, only one of the attributes can be non-null.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct TransformOperation {
    /// <p>A transform operation that casts a column to a different type.</p>
    #[serde(rename = "CastColumnTypeOperation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cast_column_type_operation: Option<CastColumnTypeOperation>,
    /// <p>An operation that creates calculated columns. Columns created in one such operation form a lexical closure.</p>
    #[serde(rename = "CreateColumnsOperation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_columns_operation: Option<CreateColumnsOperation>,
    /// <p>An operation that filters rows based on some condition.</p>
    #[serde(rename = "FilterOperation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter_operation: Option<FilterOperation>,
    /// <p>An operation that projects columns. Operations that come after a projection can only refer to projected columns.</p>
    #[serde(rename = "ProjectOperation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub project_operation: Option<ProjectOperation>,
    /// <p>An operation that renames a column.</p>
    #[serde(rename = "RenameColumnOperation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rename_column_operation: Option<RenameColumnOperation>,
    /// <p>An operation that tags a column with additional information.</p>
    #[serde(rename = "TagColumnOperation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_column_operation: Option<TagColumnOperation>,
}

/// <p>Twitter parameters.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct TwitterParameters {
    /// <p>Maximum number of rows to query Twitter.</p>
    #[serde(rename = "MaxRows")]
    pub max_rows: i64,
    /// <p>Twitter query string.</p>
    #[serde(rename = "Query")]
    pub query: String,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UntagResourceRequest {
    /// <p>The Amazon Resource Name (ARN) of the resource that you want to untag.</p>
    #[serde(rename = "ResourceArn")]
    pub resource_arn: String,
    /// <p>The keys of the key-value pairs for the resource tag or tags assigned to the resource.</p>
    #[serde(rename = "TagKeys")]
    pub tag_keys: Vec<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UntagResourceResponse {
    /// <p>The AWS request ID for this operation.</p>
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// <p>The HTTP status of the request.</p>
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<i64>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UpdateDashboardPermissionsRequest {
    /// <p>The ID of the AWS account that contains the dashboard whose permissions you're updating.</p>
    #[serde(rename = "AwsAccountId")]
    pub aws_account_id: String,
    /// <p>The ID for the dashboard.</p>
    #[serde(rename = "DashboardId")]
    pub dashboard_id: String,
    /// <p>The permissions that you want to grant on this resource.</p>
    #[serde(rename = "GrantPermissions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub grant_permissions: Option<Vec<ResourcePermission>>,
    /// <p>The permissions that you want to revoke from this resource.</p>
    #[serde(rename = "RevokePermissions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub revoke_permissions: Option<Vec<ResourcePermission>>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UpdateDashboardPermissionsResponse {
    /// <p>The Amazon Resource Name (ARN) of the dashboard.</p>
    #[serde(rename = "DashboardArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dashboard_arn: Option<String>,
    /// <p>The ID for the dashboard.</p>
    #[serde(rename = "DashboardId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dashboard_id: Option<String>,
    /// <p>Information about the permissions on the dashboard.</p>
    #[serde(rename = "Permissions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permissions: Option<Vec<ResourcePermission>>,
    /// <p>The AWS request ID for this operation.</p>
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// <p>The HTTP status of the request.</p>
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<i64>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UpdateDashboardPublishedVersionRequest {
    /// <p>The ID of the AWS account that contains the dashboard that you're updating.</p>
    #[serde(rename = "AwsAccountId")]
    pub aws_account_id: String,
    /// <p>The ID for the dashboard.</p>
    #[serde(rename = "DashboardId")]
    pub dashboard_id: String,
    /// <p>The version number of the dashboard.</p>
    #[serde(rename = "VersionNumber")]
    pub version_number: i64,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UpdateDashboardPublishedVersionResponse {
    /// <p>The Amazon Resource Name (ARN) of the dashboard.</p>
    #[serde(rename = "DashboardArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dashboard_arn: Option<String>,
    /// <p>The ID for the dashboard.</p>
    #[serde(rename = "DashboardId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dashboard_id: Option<String>,
    /// <p>The AWS request ID for this operation.</p>
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// <p>The HTTP status of the request.</p>
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<i64>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UpdateDashboardRequest {
    /// <p>The ID of the AWS account that contains the dashboard that you're updating.</p>
    #[serde(rename = "AwsAccountId")]
    pub aws_account_id: String,
    /// <p>The ID for the dashboard.</p>
    #[serde(rename = "DashboardId")]
    pub dashboard_id: String,
    /// <p><p>Options for publishing the dashboard when you create it:</p> <ul> <li> <p> <code>AvailabilityStatus</code> for <code>AdHocFilteringOption</code> - This status can be either <code>ENABLED</code> or <code>DISABLED</code>. When this is set to <code>DISABLED</code>, QuickSight disables the left filter pane on the published dashboard, which can be used for ad hoc (one-time) filtering. This option is <code>ENABLED</code> by default. </p> </li> <li> <p> <code>AvailabilityStatus</code> for <code>ExportToCSVOption</code> - This status can be either <code>ENABLED</code> or <code>DISABLED</code>. The visual option to export data to .csv format isn&#39;t enabled when this is set to <code>DISABLED</code>. This option is <code>ENABLED</code> by default. </p> </li> <li> <p> <code>VisibilityState</code> for <code>SheetControlsOption</code> - This visibility state can be either <code>COLLAPSED</code> or <code>EXPANDED</code>. The sheet controls pane is collapsed by default when set to true. This option is <code>COLLAPSED</code> by default. </p> </li> </ul></p>
    #[serde(rename = "DashboardPublishOptions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dashboard_publish_options: Option<DashboardPublishOptions>,
    /// <p>The display name of the dashboard.</p>
    #[serde(rename = "Name")]
    pub name: String,
    /// <p>A structure that contains the parameters of the dashboard.</p>
    #[serde(rename = "Parameters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameters: Option<Parameters>,
    /// <p>The template or analysis from which the dashboard is created. The <code>SouceTemplate</code> entity accepts the Amazon Resource Name (ARN) of the template and also references to replacement datasets for the placeholders set when creating the template. The replacement datasets need to follow the same schema as the datasets for which placeholders were created when creating the template.</p>
    #[serde(rename = "SourceEntity")]
    pub source_entity: DashboardSourceEntity,
    /// <p>A description for the first version of the dashboard being created.</p>
    #[serde(rename = "VersionDescription")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version_description: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UpdateDashboardResponse {
    /// <p>The Amazon Resource Name (ARN) of the resource.</p>
    #[serde(rename = "Arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p>The creation status of the request.</p>
    #[serde(rename = "CreationStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_status: Option<String>,
    /// <p>The ID for the dashboard.</p>
    #[serde(rename = "DashboardId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dashboard_id: Option<String>,
    /// <p>The AWS request ID for this operation.</p>
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// <p>The HTTP status of the request.</p>
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<i64>,
    /// <p>The ARN of the dashboard, including the version number.</p>
    #[serde(rename = "VersionArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version_arn: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UpdateDataSetPermissionsRequest {
    /// <p>The AWS account ID.</p>
    #[serde(rename = "AwsAccountId")]
    pub aws_account_id: String,
    /// <p>The ID for the dataset whose permissions you want to update. This ID is unique per AWS Region for each AWS account.</p>
    #[serde(rename = "DataSetId")]
    pub data_set_id: String,
    /// <p>The resource permissions that you want to grant to the dataset.</p>
    #[serde(rename = "GrantPermissions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub grant_permissions: Option<Vec<ResourcePermission>>,
    /// <p>The resource permissions that you want to revoke from the dataset.</p>
    #[serde(rename = "RevokePermissions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub revoke_permissions: Option<Vec<ResourcePermission>>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UpdateDataSetPermissionsResponse {
    /// <p>The Amazon Resource Name (ARN) of the dataset.</p>
    #[serde(rename = "DataSetArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_set_arn: Option<String>,
    /// <p>The ID for the dataset whose permissions you want to update. This ID is unique per AWS Region for each AWS account.</p>
    #[serde(rename = "DataSetId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_set_id: Option<String>,
    /// <p>The AWS request ID for this operation.</p>
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// <p>The HTTP status of the request.</p>
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<i64>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UpdateDataSetRequest {
    /// <p>The AWS account ID.</p>
    #[serde(rename = "AwsAccountId")]
    pub aws_account_id: String,
    /// <p>Groupings of columns that work together in certain QuickSight features. Currently, only geospatial hierarchy is supported.</p>
    #[serde(rename = "ColumnGroups")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub column_groups: Option<Vec<ColumnGroup>>,
    /// <p>The ID for the dataset that you want to update. This ID is unique per AWS Region for each AWS account.</p>
    #[serde(rename = "DataSetId")]
    pub data_set_id: String,
    /// <p>Indicates whether you want to import the data into SPICE.</p>
    #[serde(rename = "ImportMode")]
    pub import_mode: String,
    /// <p>Configures the combination and transformation of the data from the physical tables.</p>
    #[serde(rename = "LogicalTableMap")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logical_table_map: Option<::std::collections::HashMap<String, LogicalTable>>,
    /// <p>The display name for the dataset.</p>
    #[serde(rename = "Name")]
    pub name: String,
    /// <p>Declares the physical tables that are available in the underlying data sources.</p>
    #[serde(rename = "PhysicalTableMap")]
    pub physical_table_map: ::std::collections::HashMap<String, PhysicalTable>,
    /// <p>The row-level security configuration for the data you want to create.</p>
    #[serde(rename = "RowLevelPermissionDataSet")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub row_level_permission_data_set: Option<RowLevelPermissionDataSet>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UpdateDataSetResponse {
    /// <p>The Amazon Resource Name (ARN) of the dataset.</p>
    #[serde(rename = "Arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p>The ID for the dataset that you want to create. This ID is unique per AWS Region for each AWS account.</p>
    #[serde(rename = "DataSetId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_set_id: Option<String>,
    /// <p>The ARN for the ingestion, which is triggered as a result of dataset creation if the import mode is SPICE.</p>
    #[serde(rename = "IngestionArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ingestion_arn: Option<String>,
    /// <p>The ID of the ingestion, which is triggered as a result of dataset creation if the import mode is SPICE.</p>
    #[serde(rename = "IngestionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ingestion_id: Option<String>,
    /// <p>The AWS request ID for this operation.</p>
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// <p>The HTTP status of the request.</p>
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<i64>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UpdateDataSourcePermissionsRequest {
    /// <p>The AWS account ID.</p>
    #[serde(rename = "AwsAccountId")]
    pub aws_account_id: String,
    /// <p>The ID of the data source. This ID is unique per AWS Region for each AWS account. </p>
    #[serde(rename = "DataSourceId")]
    pub data_source_id: String,
    /// <p>A list of resource permissions that you want to grant on the data source.</p>
    #[serde(rename = "GrantPermissions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub grant_permissions: Option<Vec<ResourcePermission>>,
    /// <p>A list of resource permissions that you want to revoke on the data source.</p>
    #[serde(rename = "RevokePermissions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub revoke_permissions: Option<Vec<ResourcePermission>>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UpdateDataSourcePermissionsResponse {
    /// <p>The Amazon Resource Name (ARN) of the data source.</p>
    #[serde(rename = "DataSourceArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_source_arn: Option<String>,
    /// <p>The ID of the data source. This ID is unique per AWS Region for each AWS account.</p>
    #[serde(rename = "DataSourceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_source_id: Option<String>,
    /// <p>The AWS request ID for this operation.</p>
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// <p>The HTTP status of the request.</p>
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<i64>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UpdateDataSourceRequest {
    /// <p>The AWS account ID.</p>
    #[serde(rename = "AwsAccountId")]
    pub aws_account_id: String,
    /// <p>The credentials that QuickSight that uses to connect to your underlying source. Currently, only credentials based on user name and password are supported.</p>
    #[serde(rename = "Credentials")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub credentials: Option<DataSourceCredentials>,
    /// <p>The ID of the data source. This ID is unique per AWS Region for each AWS account. </p>
    #[serde(rename = "DataSourceId")]
    pub data_source_id: String,
    /// <p>The parameters that QuickSight uses to connect to your underlying source.</p>
    #[serde(rename = "DataSourceParameters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_source_parameters: Option<DataSourceParameters>,
    /// <p>A display name for the data source.</p>
    #[serde(rename = "Name")]
    pub name: String,
    /// <p>Secure Socket Layer (SSL) properties that apply when QuickSight connects to your underlying source.</p>
    #[serde(rename = "SslProperties")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ssl_properties: Option<SslProperties>,
    /// <p>Use this parameter only when you want QuickSight to use a VPC connection when connecting to your underlying source.</p>
    #[serde(rename = "VpcConnectionProperties")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_connection_properties: Option<VpcConnectionProperties>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UpdateDataSourceResponse {
    /// <p>The Amazon Resource Name (ARN) of the data source.</p>
    #[serde(rename = "Arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p>The ID of the data source. This ID is unique per AWS Region for each AWS account.</p>
    #[serde(rename = "DataSourceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_source_id: Option<String>,
    /// <p>The AWS request ID for this operation.</p>
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// <p>The HTTP status of the request.</p>
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<i64>,
    /// <p>The update status of the data source's last update.</p>
    #[serde(rename = "UpdateStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_status: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UpdateGroupRequest {
    /// <p>The ID for the AWS account that the group is in. Currently, you use the ID for the AWS account that contains your Amazon QuickSight account.</p>
    #[serde(rename = "AwsAccountId")]
    pub aws_account_id: String,
    /// <p>The description for the group that you want to update.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The name of the group that you want to update.</p>
    #[serde(rename = "GroupName")]
    pub group_name: String,
    /// <p>The namespace. Currently, you should set this to <code>default</code>.</p>
    #[serde(rename = "Namespace")]
    pub namespace: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UpdateGroupResponse {
    /// <p>The name of the group.</p>
    #[serde(rename = "Group")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group: Option<Group>,
    /// <p>The AWS request ID for this operation.</p>
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// <p>The HTTP status of the request.</p>
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<i64>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UpdateIAMPolicyAssignmentRequest {
    /// <p>The name of the assignment. This name must be unique within an AWS account.</p>
    #[serde(rename = "AssignmentName")]
    pub assignment_name: String,
    /// <p><p>The status of the assignment. Possible values are as follows:</p> <ul> <li> <p> <code>ENABLED</code> - Anything specified in this assignment is used when creating the data source.</p> </li> <li> <p> <code>DISABLED</code> - This assignment isn&#39;t used when creating the data source.</p> </li> <li> <p> <code>DRAFT</code> - This assignment is an unfinished draft and isn&#39;t used when creating the data source.</p> </li> </ul></p>
    #[serde(rename = "AssignmentStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assignment_status: Option<String>,
    /// <p>The ID of the AWS account that contains the IAM policy assignment.</p>
    #[serde(rename = "AwsAccountId")]
    pub aws_account_id: String,
    /// <p>The QuickSight users, groups, or both that you want to assign the policy to.</p>
    #[serde(rename = "Identities")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identities: Option<::std::collections::HashMap<String, Vec<String>>>,
    /// <p>The namespace of the assignment.</p>
    #[serde(rename = "Namespace")]
    pub namespace: String,
    /// <p>The ARN for the IAM policy to apply to the QuickSight users and groups specified in this assignment.</p>
    #[serde(rename = "PolicyArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_arn: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UpdateIAMPolicyAssignmentResponse {
    /// <p>The ID of the assignment.</p>
    #[serde(rename = "AssignmentId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assignment_id: Option<String>,
    /// <p>The name of the assignment. </p>
    #[serde(rename = "AssignmentName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assignment_name: Option<String>,
    /// <p><p>The status of the assignment. Possible values are as follows:</p> <ul> <li> <p> <code>ENABLED</code> - Anything specified in this assignment is used when creating the data source.</p> </li> <li> <p> <code>DISABLED</code> - This assignment isn&#39;t used when creating the data source.</p> </li> <li> <p> <code>DRAFT</code> - This assignment is an unfinished draft and isn&#39;t used when creating the data source.</p> </li> </ul></p>
    #[serde(rename = "AssignmentStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assignment_status: Option<String>,
    /// <p>The QuickSight users, groups, or both that the IAM policy is assigned to.</p>
    #[serde(rename = "Identities")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identities: Option<::std::collections::HashMap<String, Vec<String>>>,
    /// <p>The ARN for the IAM policy applied to the QuickSight users and groups specified in this assignment.</p>
    #[serde(rename = "PolicyArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_arn: Option<String>,
    /// <p>The AWS request ID for this operation.</p>
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// <p>The HTTP status of the request.</p>
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<i64>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UpdateTemplateAliasRequest {
    /// <p>The alias of the template that you want to update. If you name a specific alias, you update the version that the alias points to. You can specify the latest version of the template by providing the keyword <code>$LATEST</code> in the <code>AliasName</code> parameter. The keyword <code>$PUBLISHED</code> doesn't apply to templates.</p>
    #[serde(rename = "AliasName")]
    pub alias_name: String,
    /// <p>The ID of the AWS account that contains the template alias that you're updating.</p>
    #[serde(rename = "AwsAccountId")]
    pub aws_account_id: String,
    /// <p>The ID for the template.</p>
    #[serde(rename = "TemplateId")]
    pub template_id: String,
    /// <p>The version number of the template.</p>
    #[serde(rename = "TemplateVersionNumber")]
    pub template_version_number: i64,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UpdateTemplateAliasResponse {
    /// <p>The AWS request ID for this operation.</p>
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// <p>The HTTP status of the request.</p>
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<i64>,
    /// <p>The template alias.</p>
    #[serde(rename = "TemplateAlias")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template_alias: Option<TemplateAlias>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UpdateTemplatePermissionsRequest {
    /// <p>The ID of the AWS account that contains the template.</p>
    #[serde(rename = "AwsAccountId")]
    pub aws_account_id: String,
    /// <p>A list of resource permissions to be granted on the template. </p>
    #[serde(rename = "GrantPermissions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub grant_permissions: Option<Vec<ResourcePermission>>,
    /// <p>A list of resource permissions to be revoked from the template. </p>
    #[serde(rename = "RevokePermissions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub revoke_permissions: Option<Vec<ResourcePermission>>,
    /// <p>The ID for the template.</p>
    #[serde(rename = "TemplateId")]
    pub template_id: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UpdateTemplatePermissionsResponse {
    /// <p>A list of resource permissions to be set on the template.</p>
    #[serde(rename = "Permissions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permissions: Option<Vec<ResourcePermission>>,
    /// <p>The AWS request ID for this operation.</p>
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// <p>The HTTP status of the request.</p>
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<i64>,
    /// <p>The Amazon Resource Name (ARN) of the template.</p>
    #[serde(rename = "TemplateArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template_arn: Option<String>,
    /// <p>The ID for the template.</p>
    #[serde(rename = "TemplateId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template_id: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UpdateTemplateRequest {
    /// <p>The ID of the AWS account that contains the template that you're updating.</p>
    #[serde(rename = "AwsAccountId")]
    pub aws_account_id: String,
    /// <p>The name for the template.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The source QuickSight entity from which this template is being updated. You can currently update templates from an Analysis or another template.</p>
    #[serde(rename = "SourceEntity")]
    pub source_entity: TemplateSourceEntity,
    /// <p>The ID for the template.</p>
    #[serde(rename = "TemplateId")]
    pub template_id: String,
    /// <p>A description of the current template version that is being updated. Every time you call <code>UpdateTemplate</code>, you create a new version of the template. Each version of the template maintains a description of the version in the <code>VersionDescription</code> field.</p>
    #[serde(rename = "VersionDescription")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version_description: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UpdateTemplateResponse {
    /// <p>The Amazon Resource Name (ARN) for the template.</p>
    #[serde(rename = "Arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p>The creation status of the template.</p>
    #[serde(rename = "CreationStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_status: Option<String>,
    /// <p>The AWS request ID for this operation.</p>
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// <p>The HTTP status of the request.</p>
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<i64>,
    /// <p>The ID for the template.</p>
    #[serde(rename = "TemplateId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template_id: Option<String>,
    /// <p>The ARN for the template, including the version information of the first version.</p>
    #[serde(rename = "VersionArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version_arn: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UpdateUserRequest {
    /// <p>The ID for the AWS account that the user is in. Currently, you use the ID for the AWS account that contains your Amazon QuickSight account.</p>
    #[serde(rename = "AwsAccountId")]
    pub aws_account_id: String,
    /// <p>The email address of the user that you want to update.</p>
    #[serde(rename = "Email")]
    pub email: String,
    /// <p>The namespace. Currently, you should set this to <code>default</code>.</p>
    #[serde(rename = "Namespace")]
    pub namespace: String,
    /// <p><p>The Amazon QuickSight role of the user. The user role can be one of the following:</p> <ul> <li> <p> <code>READER</code>: A user who has read-only access to dashboards.</p> </li> <li> <p> <code>AUTHOR</code>: A user who can create data sources, datasets, analyses, and dashboards.</p> </li> <li> <p> <code>ADMIN</code>: A user who is an author, who can also manage Amazon QuickSight settings.</p> </li> </ul></p>
    #[serde(rename = "Role")]
    pub role: String,
    /// <p>The Amazon QuickSight user name that you want to update.</p>
    #[serde(rename = "UserName")]
    pub user_name: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UpdateUserResponse {
    /// <p>The AWS request ID for this operation.</p>
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// <p>The HTTP status of the request.</p>
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<i64>,
    /// <p>The Amazon QuickSight user.</p>
    #[serde(rename = "User")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<User>,
}

/// <p>Information about the format for a source file or files.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct UploadSettings {
    /// <p>Whether the file has a header row, or the files each have a header row.</p>
    #[serde(rename = "ContainsHeader")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contains_header: Option<bool>,
    /// <p>The delimiter between values in the file.</p>
    #[serde(rename = "Delimiter")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delimiter: Option<String>,
    /// <p>File format.</p>
    #[serde(rename = "Format")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<String>,
    /// <p>A row number to start reading data from.</p>
    #[serde(rename = "StartFromRow")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_from_row: Option<i64>,
    /// <p>Text qualifier.</p>
    #[serde(rename = "TextQualifier")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text_qualifier: Option<String>,
}

/// <p>A registered user of Amazon QuickSight. Currently, an Amazon QuickSight subscription can't contain more than 20 million users.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct User {
    /// <p>The active status of user. When you create an Amazon QuickSight user that’s not an IAM user or an Active Directory user, that user is inactive until they sign in and provide a password.</p>
    #[serde(rename = "Active")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active: Option<bool>,
    /// <p>The Amazon Resource Name (ARN) for the user.</p>
    #[serde(rename = "Arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p>The user's email address.</p>
    #[serde(rename = "Email")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    /// <p>The type of identity authentication used by the user.</p>
    #[serde(rename = "IdentityType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identity_type: Option<String>,
    /// <p>The principal ID of the user.</p>
    #[serde(rename = "PrincipalId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub principal_id: Option<String>,
    /// <p><p>The Amazon QuickSight role for the user. The user role can be one of the following:.</p> <ul> <li> <p> <code>READER</code>: A user who has read-only access to dashboards.</p> </li> <li> <p> <code>AUTHOR</code>: A user who can create data sources, datasets, analyses, and dashboards.</p> </li> <li> <p> <code>ADMIN</code>: A user who is an author, who can also manage Amazon QuickSight settings.</p> </li> <li> <p> <code>RESTRICTED<em>READER</code>: This role isn&#39;t currently available for use.</p> </li> <li> <p> <code>RESTRICTED</em>AUTHOR</code>: This role isn&#39;t currently available for use.</p> </li> </ul></p>
    #[serde(rename = "Role")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role: Option<String>,
    /// <p>The user's user name.</p>
    #[serde(rename = "UserName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_name: Option<String>,
}

/// <p>VPC connection properties.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct VpcConnectionProperties {
    /// <p>The Amazon Resource Name (ARN) for the VPC connection.</p>
    #[serde(rename = "VpcConnectionArn")]
    pub vpc_connection_arn: String,
}

/// Errors returned by CancelIngestion
#[derive(Debug, PartialEq)]
pub enum CancelIngestionError {
    /// <p>You don't have access to this item. The provided credentials couldn't be validated. You might not be authorized to carry out the request. Make sure that your account is authorized to use the Amazon QuickSight service, that your policies have the correct permissions, and that you are using the correct access keys.</p>
    AccessDenied(String),
    /// <p>An internal failure occurred.</p>
    InternalFailure(String),
    /// <p>One or more parameters has a value that isn't valid.</p>
    InvalidParameterValue(String),
    /// <p>The resource specified already exists. </p>
    ResourceExists(String),
    /// <p>One or more resources can't be found.</p>
    ResourceNotFound(String),
    /// <p>Access is throttled.</p>
    Throttling(String),
}

impl CancelIngestionError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CancelIngestionError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(CancelIngestionError::AccessDenied(err.msg))
                }
                "InternalFailureException" => {
                    return RusotoError::Service(CancelIngestionError::InternalFailure(err.msg))
                }
                "InvalidParameterValueException" => {
                    return RusotoError::Service(CancelIngestionError::InvalidParameterValue(
                        err.msg,
                    ))
                }
                "ResourceExistsException" => {
                    return RusotoError::Service(CancelIngestionError::ResourceExists(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(CancelIngestionError::ResourceNotFound(err.msg))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(CancelIngestionError::Throttling(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for CancelIngestionError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CancelIngestionError::AccessDenied(ref cause) => write!(f, "{}", cause),
            CancelIngestionError::InternalFailure(ref cause) => write!(f, "{}", cause),
            CancelIngestionError::InvalidParameterValue(ref cause) => write!(f, "{}", cause),
            CancelIngestionError::ResourceExists(ref cause) => write!(f, "{}", cause),
            CancelIngestionError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            CancelIngestionError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CancelIngestionError {}
/// Errors returned by CreateDashboard
#[derive(Debug, PartialEq)]
pub enum CreateDashboardError {
    /// <p>Updating or deleting a resource can cause an inconsistent state.</p>
    Conflict(String),
    /// <p>An internal failure occurred.</p>
    InternalFailure(String),
    /// <p>One or more parameters has a value that isn't valid.</p>
    InvalidParameterValue(String),
    /// <p>The resource specified already exists. </p>
    ResourceExists(String),
    /// <p>One or more resources can't be found.</p>
    ResourceNotFound(String),
    /// <p>Access is throttled.</p>
    Throttling(String),
    /// <p>This error indicates that you are calling an operation on an Amazon QuickSight subscription where the edition doesn't include support for that operation. Amazon QuickSight currently has Standard Edition and Enterprise Edition. Not every operation and capability is available in every edition.</p>
    UnsupportedUserEdition(String),
}

impl CreateDashboardError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateDashboardError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "ConflictException" => {
                    return RusotoError::Service(CreateDashboardError::Conflict(err.msg))
                }
                "InternalFailureException" => {
                    return RusotoError::Service(CreateDashboardError::InternalFailure(err.msg))
                }
                "InvalidParameterValueException" => {
                    return RusotoError::Service(CreateDashboardError::InvalidParameterValue(
                        err.msg,
                    ))
                }
                "ResourceExistsException" => {
                    return RusotoError::Service(CreateDashboardError::ResourceExists(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(CreateDashboardError::ResourceNotFound(err.msg))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(CreateDashboardError::Throttling(err.msg))
                }
                "UnsupportedUserEditionException" => {
                    return RusotoError::Service(CreateDashboardError::UnsupportedUserEdition(
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
impl fmt::Display for CreateDashboardError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreateDashboardError::Conflict(ref cause) => write!(f, "{}", cause),
            CreateDashboardError::InternalFailure(ref cause) => write!(f, "{}", cause),
            CreateDashboardError::InvalidParameterValue(ref cause) => write!(f, "{}", cause),
            CreateDashboardError::ResourceExists(ref cause) => write!(f, "{}", cause),
            CreateDashboardError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            CreateDashboardError::Throttling(ref cause) => write!(f, "{}", cause),
            CreateDashboardError::UnsupportedUserEdition(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CreateDashboardError {}
/// Errors returned by CreateDataSet
#[derive(Debug, PartialEq)]
pub enum CreateDataSetError {
    /// <p>You don't have access to this item. The provided credentials couldn't be validated. You might not be authorized to carry out the request. Make sure that your account is authorized to use the Amazon QuickSight service, that your policies have the correct permissions, and that you are using the correct access keys.</p>
    AccessDenied(String),
    /// <p>Updating or deleting a resource can cause an inconsistent state.</p>
    Conflict(String),
    /// <p>An internal failure occurred.</p>
    InternalFailure(String),
    /// <p>One or more parameters has a value that isn't valid.</p>
    InvalidParameterValue(String),
    /// <p>A limit is exceeded.</p>
    LimitExceeded(String),
    /// <p>The resource specified already exists. </p>
    ResourceExists(String),
    /// <p>One or more resources can't be found.</p>
    ResourceNotFound(String),
    /// <p>Access is throttled.</p>
    Throttling(String),
    /// <p>This error indicates that you are calling an operation on an Amazon QuickSight subscription where the edition doesn't include support for that operation. Amazon QuickSight currently has Standard Edition and Enterprise Edition. Not every operation and capability is available in every edition.</p>
    UnsupportedUserEdition(String),
}

impl CreateDataSetError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateDataSetError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(CreateDataSetError::AccessDenied(err.msg))
                }
                "ConflictException" => {
                    return RusotoError::Service(CreateDataSetError::Conflict(err.msg))
                }
                "InternalFailureException" => {
                    return RusotoError::Service(CreateDataSetError::InternalFailure(err.msg))
                }
                "InvalidParameterValueException" => {
                    return RusotoError::Service(CreateDataSetError::InvalidParameterValue(err.msg))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(CreateDataSetError::LimitExceeded(err.msg))
                }
                "ResourceExistsException" => {
                    return RusotoError::Service(CreateDataSetError::ResourceExists(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(CreateDataSetError::ResourceNotFound(err.msg))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(CreateDataSetError::Throttling(err.msg))
                }
                "UnsupportedUserEditionException" => {
                    return RusotoError::Service(CreateDataSetError::UnsupportedUserEdition(
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
impl fmt::Display for CreateDataSetError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreateDataSetError::AccessDenied(ref cause) => write!(f, "{}", cause),
            CreateDataSetError::Conflict(ref cause) => write!(f, "{}", cause),
            CreateDataSetError::InternalFailure(ref cause) => write!(f, "{}", cause),
            CreateDataSetError::InvalidParameterValue(ref cause) => write!(f, "{}", cause),
            CreateDataSetError::LimitExceeded(ref cause) => write!(f, "{}", cause),
            CreateDataSetError::ResourceExists(ref cause) => write!(f, "{}", cause),
            CreateDataSetError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            CreateDataSetError::Throttling(ref cause) => write!(f, "{}", cause),
            CreateDataSetError::UnsupportedUserEdition(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CreateDataSetError {}
/// Errors returned by CreateDataSource
#[derive(Debug, PartialEq)]
pub enum CreateDataSourceError {
    /// <p>You don't have access to this item. The provided credentials couldn't be validated. You might not be authorized to carry out the request. Make sure that your account is authorized to use the Amazon QuickSight service, that your policies have the correct permissions, and that you are using the correct access keys.</p>
    AccessDenied(String),
    /// <p>Updating or deleting a resource can cause an inconsistent state.</p>
    Conflict(String),
    /// <p>An internal failure occurred.</p>
    InternalFailure(String),
    /// <p>One or more parameters has a value that isn't valid.</p>
    InvalidParameterValue(String),
    /// <p>A limit is exceeded.</p>
    LimitExceeded(String),
    /// <p>The resource specified already exists. </p>
    ResourceExists(String),
    /// <p>One or more resources can't be found.</p>
    ResourceNotFound(String),
    /// <p>Access is throttled.</p>
    Throttling(String),
}

impl CreateDataSourceError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateDataSourceError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(CreateDataSourceError::AccessDenied(err.msg))
                }
                "ConflictException" => {
                    return RusotoError::Service(CreateDataSourceError::Conflict(err.msg))
                }
                "InternalFailureException" => {
                    return RusotoError::Service(CreateDataSourceError::InternalFailure(err.msg))
                }
                "InvalidParameterValueException" => {
                    return RusotoError::Service(CreateDataSourceError::InvalidParameterValue(
                        err.msg,
                    ))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(CreateDataSourceError::LimitExceeded(err.msg))
                }
                "ResourceExistsException" => {
                    return RusotoError::Service(CreateDataSourceError::ResourceExists(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(CreateDataSourceError::ResourceNotFound(err.msg))
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
            CreateDataSourceError::InternalFailure(ref cause) => write!(f, "{}", cause),
            CreateDataSourceError::InvalidParameterValue(ref cause) => write!(f, "{}", cause),
            CreateDataSourceError::LimitExceeded(ref cause) => write!(f, "{}", cause),
            CreateDataSourceError::ResourceExists(ref cause) => write!(f, "{}", cause),
            CreateDataSourceError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            CreateDataSourceError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CreateDataSourceError {}
/// Errors returned by CreateGroup
#[derive(Debug, PartialEq)]
pub enum CreateGroupError {
    /// <p>You don't have access to this item. The provided credentials couldn't be validated. You might not be authorized to carry out the request. Make sure that your account is authorized to use the Amazon QuickSight service, that your policies have the correct permissions, and that you are using the correct access keys.</p>
    AccessDenied(String),
    /// <p>An internal failure occurred.</p>
    InternalFailure(String),
    /// <p>One or more parameters has a value that isn't valid.</p>
    InvalidParameterValue(String),
    /// <p>A limit is exceeded.</p>
    LimitExceeded(String),
    /// <p>One or more preconditions aren't met.</p>
    PreconditionNotMet(String),
    /// <p>The resource specified already exists. </p>
    ResourceExists(String),
    /// <p>One or more resources can't be found.</p>
    ResourceNotFound(String),
    /// <p>This resource is currently unavailable.</p>
    ResourceUnavailable(String),
    /// <p>Access is throttled.</p>
    Throttling(String),
}

impl CreateGroupError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateGroupError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(CreateGroupError::AccessDenied(err.msg))
                }
                "InternalFailureException" => {
                    return RusotoError::Service(CreateGroupError::InternalFailure(err.msg))
                }
                "InvalidParameterValueException" => {
                    return RusotoError::Service(CreateGroupError::InvalidParameterValue(err.msg))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(CreateGroupError::LimitExceeded(err.msg))
                }
                "PreconditionNotMetException" => {
                    return RusotoError::Service(CreateGroupError::PreconditionNotMet(err.msg))
                }
                "ResourceExistsException" => {
                    return RusotoError::Service(CreateGroupError::ResourceExists(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(CreateGroupError::ResourceNotFound(err.msg))
                }
                "ResourceUnavailableException" => {
                    return RusotoError::Service(CreateGroupError::ResourceUnavailable(err.msg))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(CreateGroupError::Throttling(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for CreateGroupError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreateGroupError::AccessDenied(ref cause) => write!(f, "{}", cause),
            CreateGroupError::InternalFailure(ref cause) => write!(f, "{}", cause),
            CreateGroupError::InvalidParameterValue(ref cause) => write!(f, "{}", cause),
            CreateGroupError::LimitExceeded(ref cause) => write!(f, "{}", cause),
            CreateGroupError::PreconditionNotMet(ref cause) => write!(f, "{}", cause),
            CreateGroupError::ResourceExists(ref cause) => write!(f, "{}", cause),
            CreateGroupError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            CreateGroupError::ResourceUnavailable(ref cause) => write!(f, "{}", cause),
            CreateGroupError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CreateGroupError {}
/// Errors returned by CreateGroupMembership
#[derive(Debug, PartialEq)]
pub enum CreateGroupMembershipError {
    /// <p>You don't have access to this item. The provided credentials couldn't be validated. You might not be authorized to carry out the request. Make sure that your account is authorized to use the Amazon QuickSight service, that your policies have the correct permissions, and that you are using the correct access keys.</p>
    AccessDenied(String),
    /// <p>An internal failure occurred.</p>
    InternalFailure(String),
    /// <p>One or more parameters has a value that isn't valid.</p>
    InvalidParameterValue(String),
    /// <p>One or more preconditions aren't met.</p>
    PreconditionNotMet(String),
    /// <p>One or more resources can't be found.</p>
    ResourceNotFound(String),
    /// <p>This resource is currently unavailable.</p>
    ResourceUnavailable(String),
    /// <p>Access is throttled.</p>
    Throttling(String),
}

impl CreateGroupMembershipError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateGroupMembershipError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(CreateGroupMembershipError::AccessDenied(err.msg))
                }
                "InternalFailureException" => {
                    return RusotoError::Service(CreateGroupMembershipError::InternalFailure(
                        err.msg,
                    ))
                }
                "InvalidParameterValueException" => {
                    return RusotoError::Service(CreateGroupMembershipError::InvalidParameterValue(
                        err.msg,
                    ))
                }
                "PreconditionNotMetException" => {
                    return RusotoError::Service(CreateGroupMembershipError::PreconditionNotMet(
                        err.msg,
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(CreateGroupMembershipError::ResourceNotFound(
                        err.msg,
                    ))
                }
                "ResourceUnavailableException" => {
                    return RusotoError::Service(CreateGroupMembershipError::ResourceUnavailable(
                        err.msg,
                    ))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(CreateGroupMembershipError::Throttling(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for CreateGroupMembershipError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreateGroupMembershipError::AccessDenied(ref cause) => write!(f, "{}", cause),
            CreateGroupMembershipError::InternalFailure(ref cause) => write!(f, "{}", cause),
            CreateGroupMembershipError::InvalidParameterValue(ref cause) => write!(f, "{}", cause),
            CreateGroupMembershipError::PreconditionNotMet(ref cause) => write!(f, "{}", cause),
            CreateGroupMembershipError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            CreateGroupMembershipError::ResourceUnavailable(ref cause) => write!(f, "{}", cause),
            CreateGroupMembershipError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CreateGroupMembershipError {}
/// Errors returned by CreateIAMPolicyAssignment
#[derive(Debug, PartialEq)]
pub enum CreateIAMPolicyAssignmentError {
    /// <p>You don't have access to this item. The provided credentials couldn't be validated. You might not be authorized to carry out the request. Make sure that your account is authorized to use the Amazon QuickSight service, that your policies have the correct permissions, and that you are using the correct access keys.</p>
    AccessDenied(String),
    /// <p>A resource is already in a state that indicates an action is happening that must complete before a new update can be applied.</p>
    ConcurrentUpdating(String),
    /// <p>An internal failure occurred.</p>
    InternalFailure(String),
    /// <p>One or more parameters has a value that isn't valid.</p>
    InvalidParameterValue(String),
    /// <p>The resource specified already exists. </p>
    ResourceExists(String),
    /// <p>One or more resources can't be found.</p>
    ResourceNotFound(String),
    /// <p>Access is throttled.</p>
    Throttling(String),
}

impl CreateIAMPolicyAssignmentError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateIAMPolicyAssignmentError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(CreateIAMPolicyAssignmentError::AccessDenied(
                        err.msg,
                    ))
                }
                "ConcurrentUpdatingException" => {
                    return RusotoError::Service(
                        CreateIAMPolicyAssignmentError::ConcurrentUpdating(err.msg),
                    )
                }
                "InternalFailureException" => {
                    return RusotoError::Service(CreateIAMPolicyAssignmentError::InternalFailure(
                        err.msg,
                    ))
                }
                "InvalidParameterValueException" => {
                    return RusotoError::Service(
                        CreateIAMPolicyAssignmentError::InvalidParameterValue(err.msg),
                    )
                }
                "ResourceExistsException" => {
                    return RusotoError::Service(CreateIAMPolicyAssignmentError::ResourceExists(
                        err.msg,
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(CreateIAMPolicyAssignmentError::ResourceNotFound(
                        err.msg,
                    ))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(CreateIAMPolicyAssignmentError::Throttling(
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
impl fmt::Display for CreateIAMPolicyAssignmentError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreateIAMPolicyAssignmentError::AccessDenied(ref cause) => write!(f, "{}", cause),
            CreateIAMPolicyAssignmentError::ConcurrentUpdating(ref cause) => write!(f, "{}", cause),
            CreateIAMPolicyAssignmentError::InternalFailure(ref cause) => write!(f, "{}", cause),
            CreateIAMPolicyAssignmentError::InvalidParameterValue(ref cause) => {
                write!(f, "{}", cause)
            }
            CreateIAMPolicyAssignmentError::ResourceExists(ref cause) => write!(f, "{}", cause),
            CreateIAMPolicyAssignmentError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            CreateIAMPolicyAssignmentError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CreateIAMPolicyAssignmentError {}
/// Errors returned by CreateIngestion
#[derive(Debug, PartialEq)]
pub enum CreateIngestionError {
    /// <p>You don't have access to this item. The provided credentials couldn't be validated. You might not be authorized to carry out the request. Make sure that your account is authorized to use the Amazon QuickSight service, that your policies have the correct permissions, and that you are using the correct access keys.</p>
    AccessDenied(String),
    /// <p>An internal failure occurred.</p>
    InternalFailure(String),
    /// <p>One or more parameters has a value that isn't valid.</p>
    InvalidParameterValue(String),
    /// <p>A limit is exceeded.</p>
    LimitExceeded(String),
    /// <p>The resource specified already exists. </p>
    ResourceExists(String),
    /// <p>One or more resources can't be found.</p>
    ResourceNotFound(String),
    /// <p>Access is throttled.</p>
    Throttling(String),
}

impl CreateIngestionError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateIngestionError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(CreateIngestionError::AccessDenied(err.msg))
                }
                "InternalFailureException" => {
                    return RusotoError::Service(CreateIngestionError::InternalFailure(err.msg))
                }
                "InvalidParameterValueException" => {
                    return RusotoError::Service(CreateIngestionError::InvalidParameterValue(
                        err.msg,
                    ))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(CreateIngestionError::LimitExceeded(err.msg))
                }
                "ResourceExistsException" => {
                    return RusotoError::Service(CreateIngestionError::ResourceExists(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(CreateIngestionError::ResourceNotFound(err.msg))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(CreateIngestionError::Throttling(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for CreateIngestionError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreateIngestionError::AccessDenied(ref cause) => write!(f, "{}", cause),
            CreateIngestionError::InternalFailure(ref cause) => write!(f, "{}", cause),
            CreateIngestionError::InvalidParameterValue(ref cause) => write!(f, "{}", cause),
            CreateIngestionError::LimitExceeded(ref cause) => write!(f, "{}", cause),
            CreateIngestionError::ResourceExists(ref cause) => write!(f, "{}", cause),
            CreateIngestionError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            CreateIngestionError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CreateIngestionError {}
/// Errors returned by CreateTemplate
#[derive(Debug, PartialEq)]
pub enum CreateTemplateError {
    /// <p>You don't have access to this item. The provided credentials couldn't be validated. You might not be authorized to carry out the request. Make sure that your account is authorized to use the Amazon QuickSight service, that your policies have the correct permissions, and that you are using the correct access keys.</p>
    AccessDenied(String),
    /// <p>An internal failure occurred.</p>
    InternalFailure(String),
    /// <p>One or more parameters has a value that isn't valid.</p>
    InvalidParameterValue(String),
    /// <p>A limit is exceeded.</p>
    LimitExceeded(String),
    /// <p>The resource specified already exists. </p>
    ResourceExists(String),
    /// <p>One or more resources can't be found.</p>
    ResourceNotFound(String),
    /// <p>Access is throttled.</p>
    Throttling(String),
    /// <p>This error indicates that you are calling an operation on an Amazon QuickSight subscription where the edition doesn't include support for that operation. Amazon QuickSight currently has Standard Edition and Enterprise Edition. Not every operation and capability is available in every edition.</p>
    UnsupportedUserEdition(String),
}

impl CreateTemplateError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateTemplateError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(CreateTemplateError::AccessDenied(err.msg))
                }
                "InternalFailureException" => {
                    return RusotoError::Service(CreateTemplateError::InternalFailure(err.msg))
                }
                "InvalidParameterValueException" => {
                    return RusotoError::Service(CreateTemplateError::InvalidParameterValue(
                        err.msg,
                    ))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(CreateTemplateError::LimitExceeded(err.msg))
                }
                "ResourceExistsException" => {
                    return RusotoError::Service(CreateTemplateError::ResourceExists(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(CreateTemplateError::ResourceNotFound(err.msg))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(CreateTemplateError::Throttling(err.msg))
                }
                "UnsupportedUserEditionException" => {
                    return RusotoError::Service(CreateTemplateError::UnsupportedUserEdition(
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
impl fmt::Display for CreateTemplateError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreateTemplateError::AccessDenied(ref cause) => write!(f, "{}", cause),
            CreateTemplateError::InternalFailure(ref cause) => write!(f, "{}", cause),
            CreateTemplateError::InvalidParameterValue(ref cause) => write!(f, "{}", cause),
            CreateTemplateError::LimitExceeded(ref cause) => write!(f, "{}", cause),
            CreateTemplateError::ResourceExists(ref cause) => write!(f, "{}", cause),
            CreateTemplateError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            CreateTemplateError::Throttling(ref cause) => write!(f, "{}", cause),
            CreateTemplateError::UnsupportedUserEdition(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CreateTemplateError {}
/// Errors returned by CreateTemplateAlias
#[derive(Debug, PartialEq)]
pub enum CreateTemplateAliasError {
    /// <p>Updating or deleting a resource can cause an inconsistent state.</p>
    Conflict(String),
    /// <p>An internal failure occurred.</p>
    InternalFailure(String),
    /// <p>A limit is exceeded.</p>
    LimitExceeded(String),
    /// <p>The resource specified already exists. </p>
    ResourceExists(String),
    /// <p>One or more resources can't be found.</p>
    ResourceNotFound(String),
    /// <p>Access is throttled.</p>
    Throttling(String),
    /// <p>This error indicates that you are calling an operation on an Amazon QuickSight subscription where the edition doesn't include support for that operation. Amazon QuickSight currently has Standard Edition and Enterprise Edition. Not every operation and capability is available in every edition.</p>
    UnsupportedUserEdition(String),
}

impl CreateTemplateAliasError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateTemplateAliasError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "ConflictException" => {
                    return RusotoError::Service(CreateTemplateAliasError::Conflict(err.msg))
                }
                "InternalFailureException" => {
                    return RusotoError::Service(CreateTemplateAliasError::InternalFailure(err.msg))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(CreateTemplateAliasError::LimitExceeded(err.msg))
                }
                "ResourceExistsException" => {
                    return RusotoError::Service(CreateTemplateAliasError::ResourceExists(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(CreateTemplateAliasError::ResourceNotFound(
                        err.msg,
                    ))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(CreateTemplateAliasError::Throttling(err.msg))
                }
                "UnsupportedUserEditionException" => {
                    return RusotoError::Service(CreateTemplateAliasError::UnsupportedUserEdition(
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
impl fmt::Display for CreateTemplateAliasError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreateTemplateAliasError::Conflict(ref cause) => write!(f, "{}", cause),
            CreateTemplateAliasError::InternalFailure(ref cause) => write!(f, "{}", cause),
            CreateTemplateAliasError::LimitExceeded(ref cause) => write!(f, "{}", cause),
            CreateTemplateAliasError::ResourceExists(ref cause) => write!(f, "{}", cause),
            CreateTemplateAliasError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            CreateTemplateAliasError::Throttling(ref cause) => write!(f, "{}", cause),
            CreateTemplateAliasError::UnsupportedUserEdition(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CreateTemplateAliasError {}
/// Errors returned by DeleteDashboard
#[derive(Debug, PartialEq)]
pub enum DeleteDashboardError {
    /// <p>Updating or deleting a resource can cause an inconsistent state.</p>
    Conflict(String),
    /// <p>An internal failure occurred.</p>
    InternalFailure(String),
    /// <p>One or more parameters has a value that isn't valid.</p>
    InvalidParameterValue(String),
    /// <p>One or more resources can't be found.</p>
    ResourceNotFound(String),
    /// <p>Access is throttled.</p>
    Throttling(String),
    /// <p>This error indicates that you are calling an operation on an Amazon QuickSight subscription where the edition doesn't include support for that operation. Amazon QuickSight currently has Standard Edition and Enterprise Edition. Not every operation and capability is available in every edition.</p>
    UnsupportedUserEdition(String),
}

impl DeleteDashboardError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteDashboardError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "ConflictException" => {
                    return RusotoError::Service(DeleteDashboardError::Conflict(err.msg))
                }
                "InternalFailureException" => {
                    return RusotoError::Service(DeleteDashboardError::InternalFailure(err.msg))
                }
                "InvalidParameterValueException" => {
                    return RusotoError::Service(DeleteDashboardError::InvalidParameterValue(
                        err.msg,
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(DeleteDashboardError::ResourceNotFound(err.msg))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(DeleteDashboardError::Throttling(err.msg))
                }
                "UnsupportedUserEditionException" => {
                    return RusotoError::Service(DeleteDashboardError::UnsupportedUserEdition(
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
impl fmt::Display for DeleteDashboardError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteDashboardError::Conflict(ref cause) => write!(f, "{}", cause),
            DeleteDashboardError::InternalFailure(ref cause) => write!(f, "{}", cause),
            DeleteDashboardError::InvalidParameterValue(ref cause) => write!(f, "{}", cause),
            DeleteDashboardError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            DeleteDashboardError::Throttling(ref cause) => write!(f, "{}", cause),
            DeleteDashboardError::UnsupportedUserEdition(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeleteDashboardError {}
/// Errors returned by DeleteDataSet
#[derive(Debug, PartialEq)]
pub enum DeleteDataSetError {
    /// <p>You don't have access to this item. The provided credentials couldn't be validated. You might not be authorized to carry out the request. Make sure that your account is authorized to use the Amazon QuickSight service, that your policies have the correct permissions, and that you are using the correct access keys.</p>
    AccessDenied(String),
    /// <p>An internal failure occurred.</p>
    InternalFailure(String),
    /// <p>One or more parameters has a value that isn't valid.</p>
    InvalidParameterValue(String),
    /// <p>One or more resources can't be found.</p>
    ResourceNotFound(String),
    /// <p>Access is throttled.</p>
    Throttling(String),
}

impl DeleteDataSetError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteDataSetError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(DeleteDataSetError::AccessDenied(err.msg))
                }
                "InternalFailureException" => {
                    return RusotoError::Service(DeleteDataSetError::InternalFailure(err.msg))
                }
                "InvalidParameterValueException" => {
                    return RusotoError::Service(DeleteDataSetError::InvalidParameterValue(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(DeleteDataSetError::ResourceNotFound(err.msg))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(DeleteDataSetError::Throttling(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DeleteDataSetError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteDataSetError::AccessDenied(ref cause) => write!(f, "{}", cause),
            DeleteDataSetError::InternalFailure(ref cause) => write!(f, "{}", cause),
            DeleteDataSetError::InvalidParameterValue(ref cause) => write!(f, "{}", cause),
            DeleteDataSetError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            DeleteDataSetError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeleteDataSetError {}
/// Errors returned by DeleteDataSource
#[derive(Debug, PartialEq)]
pub enum DeleteDataSourceError {
    /// <p>You don't have access to this item. The provided credentials couldn't be validated. You might not be authorized to carry out the request. Make sure that your account is authorized to use the Amazon QuickSight service, that your policies have the correct permissions, and that you are using the correct access keys.</p>
    AccessDenied(String),
    /// <p>An internal failure occurred.</p>
    InternalFailure(String),
    /// <p>One or more parameters has a value that isn't valid.</p>
    InvalidParameterValue(String),
    /// <p>One or more resources can't be found.</p>
    ResourceNotFound(String),
    /// <p>Access is throttled.</p>
    Throttling(String),
}

impl DeleteDataSourceError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteDataSourceError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(DeleteDataSourceError::AccessDenied(err.msg))
                }
                "InternalFailureException" => {
                    return RusotoError::Service(DeleteDataSourceError::InternalFailure(err.msg))
                }
                "InvalidParameterValueException" => {
                    return RusotoError::Service(DeleteDataSourceError::InvalidParameterValue(
                        err.msg,
                    ))
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
            DeleteDataSourceError::InternalFailure(ref cause) => write!(f, "{}", cause),
            DeleteDataSourceError::InvalidParameterValue(ref cause) => write!(f, "{}", cause),
            DeleteDataSourceError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            DeleteDataSourceError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeleteDataSourceError {}
/// Errors returned by DeleteGroup
#[derive(Debug, PartialEq)]
pub enum DeleteGroupError {
    /// <p>You don't have access to this item. The provided credentials couldn't be validated. You might not be authorized to carry out the request. Make sure that your account is authorized to use the Amazon QuickSight service, that your policies have the correct permissions, and that you are using the correct access keys.</p>
    AccessDenied(String),
    /// <p>An internal failure occurred.</p>
    InternalFailure(String),
    /// <p>One or more parameters has a value that isn't valid.</p>
    InvalidParameterValue(String),
    /// <p>One or more preconditions aren't met.</p>
    PreconditionNotMet(String),
    /// <p>One or more resources can't be found.</p>
    ResourceNotFound(String),
    /// <p>This resource is currently unavailable.</p>
    ResourceUnavailable(String),
    /// <p>Access is throttled.</p>
    Throttling(String),
}

impl DeleteGroupError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteGroupError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(DeleteGroupError::AccessDenied(err.msg))
                }
                "InternalFailureException" => {
                    return RusotoError::Service(DeleteGroupError::InternalFailure(err.msg))
                }
                "InvalidParameterValueException" => {
                    return RusotoError::Service(DeleteGroupError::InvalidParameterValue(err.msg))
                }
                "PreconditionNotMetException" => {
                    return RusotoError::Service(DeleteGroupError::PreconditionNotMet(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(DeleteGroupError::ResourceNotFound(err.msg))
                }
                "ResourceUnavailableException" => {
                    return RusotoError::Service(DeleteGroupError::ResourceUnavailable(err.msg))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(DeleteGroupError::Throttling(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DeleteGroupError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteGroupError::AccessDenied(ref cause) => write!(f, "{}", cause),
            DeleteGroupError::InternalFailure(ref cause) => write!(f, "{}", cause),
            DeleteGroupError::InvalidParameterValue(ref cause) => write!(f, "{}", cause),
            DeleteGroupError::PreconditionNotMet(ref cause) => write!(f, "{}", cause),
            DeleteGroupError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            DeleteGroupError::ResourceUnavailable(ref cause) => write!(f, "{}", cause),
            DeleteGroupError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeleteGroupError {}
/// Errors returned by DeleteGroupMembership
#[derive(Debug, PartialEq)]
pub enum DeleteGroupMembershipError {
    /// <p>You don't have access to this item. The provided credentials couldn't be validated. You might not be authorized to carry out the request. Make sure that your account is authorized to use the Amazon QuickSight service, that your policies have the correct permissions, and that you are using the correct access keys.</p>
    AccessDenied(String),
    /// <p>An internal failure occurred.</p>
    InternalFailure(String),
    /// <p>One or more parameters has a value that isn't valid.</p>
    InvalidParameterValue(String),
    /// <p>One or more preconditions aren't met.</p>
    PreconditionNotMet(String),
    /// <p>One or more resources can't be found.</p>
    ResourceNotFound(String),
    /// <p>This resource is currently unavailable.</p>
    ResourceUnavailable(String),
    /// <p>Access is throttled.</p>
    Throttling(String),
}

impl DeleteGroupMembershipError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteGroupMembershipError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(DeleteGroupMembershipError::AccessDenied(err.msg))
                }
                "InternalFailureException" => {
                    return RusotoError::Service(DeleteGroupMembershipError::InternalFailure(
                        err.msg,
                    ))
                }
                "InvalidParameterValueException" => {
                    return RusotoError::Service(DeleteGroupMembershipError::InvalidParameterValue(
                        err.msg,
                    ))
                }
                "PreconditionNotMetException" => {
                    return RusotoError::Service(DeleteGroupMembershipError::PreconditionNotMet(
                        err.msg,
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(DeleteGroupMembershipError::ResourceNotFound(
                        err.msg,
                    ))
                }
                "ResourceUnavailableException" => {
                    return RusotoError::Service(DeleteGroupMembershipError::ResourceUnavailable(
                        err.msg,
                    ))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(DeleteGroupMembershipError::Throttling(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DeleteGroupMembershipError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteGroupMembershipError::AccessDenied(ref cause) => write!(f, "{}", cause),
            DeleteGroupMembershipError::InternalFailure(ref cause) => write!(f, "{}", cause),
            DeleteGroupMembershipError::InvalidParameterValue(ref cause) => write!(f, "{}", cause),
            DeleteGroupMembershipError::PreconditionNotMet(ref cause) => write!(f, "{}", cause),
            DeleteGroupMembershipError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            DeleteGroupMembershipError::ResourceUnavailable(ref cause) => write!(f, "{}", cause),
            DeleteGroupMembershipError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeleteGroupMembershipError {}
/// Errors returned by DeleteIAMPolicyAssignment
#[derive(Debug, PartialEq)]
pub enum DeleteIAMPolicyAssignmentError {
    /// <p>You don't have access to this item. The provided credentials couldn't be validated. You might not be authorized to carry out the request. Make sure that your account is authorized to use the Amazon QuickSight service, that your policies have the correct permissions, and that you are using the correct access keys.</p>
    AccessDenied(String),
    /// <p>A resource is already in a state that indicates an action is happening that must complete before a new update can be applied.</p>
    ConcurrentUpdating(String),
    /// <p>An internal failure occurred.</p>
    InternalFailure(String),
    /// <p>One or more parameters has a value that isn't valid.</p>
    InvalidParameterValue(String),
    /// <p>The resource specified already exists. </p>
    ResourceExists(String),
    /// <p>One or more resources can't be found.</p>
    ResourceNotFound(String),
    /// <p>Access is throttled.</p>
    Throttling(String),
}

impl DeleteIAMPolicyAssignmentError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteIAMPolicyAssignmentError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(DeleteIAMPolicyAssignmentError::AccessDenied(
                        err.msg,
                    ))
                }
                "ConcurrentUpdatingException" => {
                    return RusotoError::Service(
                        DeleteIAMPolicyAssignmentError::ConcurrentUpdating(err.msg),
                    )
                }
                "InternalFailureException" => {
                    return RusotoError::Service(DeleteIAMPolicyAssignmentError::InternalFailure(
                        err.msg,
                    ))
                }
                "InvalidParameterValueException" => {
                    return RusotoError::Service(
                        DeleteIAMPolicyAssignmentError::InvalidParameterValue(err.msg),
                    )
                }
                "ResourceExistsException" => {
                    return RusotoError::Service(DeleteIAMPolicyAssignmentError::ResourceExists(
                        err.msg,
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(DeleteIAMPolicyAssignmentError::ResourceNotFound(
                        err.msg,
                    ))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(DeleteIAMPolicyAssignmentError::Throttling(
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
impl fmt::Display for DeleteIAMPolicyAssignmentError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteIAMPolicyAssignmentError::AccessDenied(ref cause) => write!(f, "{}", cause),
            DeleteIAMPolicyAssignmentError::ConcurrentUpdating(ref cause) => write!(f, "{}", cause),
            DeleteIAMPolicyAssignmentError::InternalFailure(ref cause) => write!(f, "{}", cause),
            DeleteIAMPolicyAssignmentError::InvalidParameterValue(ref cause) => {
                write!(f, "{}", cause)
            }
            DeleteIAMPolicyAssignmentError::ResourceExists(ref cause) => write!(f, "{}", cause),
            DeleteIAMPolicyAssignmentError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            DeleteIAMPolicyAssignmentError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeleteIAMPolicyAssignmentError {}
/// Errors returned by DeleteTemplate
#[derive(Debug, PartialEq)]
pub enum DeleteTemplateError {
    /// <p>Updating or deleting a resource can cause an inconsistent state.</p>
    Conflict(String),
    /// <p>An internal failure occurred.</p>
    InternalFailure(String),
    /// <p>One or more parameters has a value that isn't valid.</p>
    InvalidParameterValue(String),
    /// <p>A limit is exceeded.</p>
    LimitExceeded(String),
    /// <p>One or more resources can't be found.</p>
    ResourceNotFound(String),
    /// <p>Access is throttled.</p>
    Throttling(String),
    /// <p>This error indicates that you are calling an operation on an Amazon QuickSight subscription where the edition doesn't include support for that operation. Amazon QuickSight currently has Standard Edition and Enterprise Edition. Not every operation and capability is available in every edition.</p>
    UnsupportedUserEdition(String),
}

impl DeleteTemplateError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteTemplateError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "ConflictException" => {
                    return RusotoError::Service(DeleteTemplateError::Conflict(err.msg))
                }
                "InternalFailureException" => {
                    return RusotoError::Service(DeleteTemplateError::InternalFailure(err.msg))
                }
                "InvalidParameterValueException" => {
                    return RusotoError::Service(DeleteTemplateError::InvalidParameterValue(
                        err.msg,
                    ))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(DeleteTemplateError::LimitExceeded(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(DeleteTemplateError::ResourceNotFound(err.msg))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(DeleteTemplateError::Throttling(err.msg))
                }
                "UnsupportedUserEditionException" => {
                    return RusotoError::Service(DeleteTemplateError::UnsupportedUserEdition(
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
impl fmt::Display for DeleteTemplateError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteTemplateError::Conflict(ref cause) => write!(f, "{}", cause),
            DeleteTemplateError::InternalFailure(ref cause) => write!(f, "{}", cause),
            DeleteTemplateError::InvalidParameterValue(ref cause) => write!(f, "{}", cause),
            DeleteTemplateError::LimitExceeded(ref cause) => write!(f, "{}", cause),
            DeleteTemplateError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            DeleteTemplateError::Throttling(ref cause) => write!(f, "{}", cause),
            DeleteTemplateError::UnsupportedUserEdition(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeleteTemplateError {}
/// Errors returned by DeleteTemplateAlias
#[derive(Debug, PartialEq)]
pub enum DeleteTemplateAliasError {
    /// <p>An internal failure occurred.</p>
    InternalFailure(String),
    /// <p>One or more resources can't be found.</p>
    ResourceNotFound(String),
    /// <p>Access is throttled.</p>
    Throttling(String),
    /// <p>This error indicates that you are calling an operation on an Amazon QuickSight subscription where the edition doesn't include support for that operation. Amazon QuickSight currently has Standard Edition and Enterprise Edition. Not every operation and capability is available in every edition.</p>
    UnsupportedUserEdition(String),
}

impl DeleteTemplateAliasError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteTemplateAliasError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InternalFailureException" => {
                    return RusotoError::Service(DeleteTemplateAliasError::InternalFailure(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(DeleteTemplateAliasError::ResourceNotFound(
                        err.msg,
                    ))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(DeleteTemplateAliasError::Throttling(err.msg))
                }
                "UnsupportedUserEditionException" => {
                    return RusotoError::Service(DeleteTemplateAliasError::UnsupportedUserEdition(
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
impl fmt::Display for DeleteTemplateAliasError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteTemplateAliasError::InternalFailure(ref cause) => write!(f, "{}", cause),
            DeleteTemplateAliasError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            DeleteTemplateAliasError::Throttling(ref cause) => write!(f, "{}", cause),
            DeleteTemplateAliasError::UnsupportedUserEdition(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeleteTemplateAliasError {}
/// Errors returned by DeleteUser
#[derive(Debug, PartialEq)]
pub enum DeleteUserError {
    /// <p>You don't have access to this item. The provided credentials couldn't be validated. You might not be authorized to carry out the request. Make sure that your account is authorized to use the Amazon QuickSight service, that your policies have the correct permissions, and that you are using the correct access keys.</p>
    AccessDenied(String),
    /// <p>An internal failure occurred.</p>
    InternalFailure(String),
    /// <p>One or more parameters has a value that isn't valid.</p>
    InvalidParameterValue(String),
    /// <p>One or more resources can't be found.</p>
    ResourceNotFound(String),
    /// <p>This resource is currently unavailable.</p>
    ResourceUnavailable(String),
    /// <p>Access is throttled.</p>
    Throttling(String),
}

impl DeleteUserError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteUserError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(DeleteUserError::AccessDenied(err.msg))
                }
                "InternalFailureException" => {
                    return RusotoError::Service(DeleteUserError::InternalFailure(err.msg))
                }
                "InvalidParameterValueException" => {
                    return RusotoError::Service(DeleteUserError::InvalidParameterValue(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(DeleteUserError::ResourceNotFound(err.msg))
                }
                "ResourceUnavailableException" => {
                    return RusotoError::Service(DeleteUserError::ResourceUnavailable(err.msg))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(DeleteUserError::Throttling(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DeleteUserError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteUserError::AccessDenied(ref cause) => write!(f, "{}", cause),
            DeleteUserError::InternalFailure(ref cause) => write!(f, "{}", cause),
            DeleteUserError::InvalidParameterValue(ref cause) => write!(f, "{}", cause),
            DeleteUserError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            DeleteUserError::ResourceUnavailable(ref cause) => write!(f, "{}", cause),
            DeleteUserError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeleteUserError {}
/// Errors returned by DeleteUserByPrincipalId
#[derive(Debug, PartialEq)]
pub enum DeleteUserByPrincipalIdError {
    /// <p>You don't have access to this item. The provided credentials couldn't be validated. You might not be authorized to carry out the request. Make sure that your account is authorized to use the Amazon QuickSight service, that your policies have the correct permissions, and that you are using the correct access keys.</p>
    AccessDenied(String),
    /// <p>An internal failure occurred.</p>
    InternalFailure(String),
    /// <p>One or more parameters has a value that isn't valid.</p>
    InvalidParameterValue(String),
    /// <p>One or more resources can't be found.</p>
    ResourceNotFound(String),
    /// <p>This resource is currently unavailable.</p>
    ResourceUnavailable(String),
    /// <p>Access is throttled.</p>
    Throttling(String),
}

impl DeleteUserByPrincipalIdError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteUserByPrincipalIdError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(DeleteUserByPrincipalIdError::AccessDenied(
                        err.msg,
                    ))
                }
                "InternalFailureException" => {
                    return RusotoError::Service(DeleteUserByPrincipalIdError::InternalFailure(
                        err.msg,
                    ))
                }
                "InvalidParameterValueException" => {
                    return RusotoError::Service(
                        DeleteUserByPrincipalIdError::InvalidParameterValue(err.msg),
                    )
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(DeleteUserByPrincipalIdError::ResourceNotFound(
                        err.msg,
                    ))
                }
                "ResourceUnavailableException" => {
                    return RusotoError::Service(DeleteUserByPrincipalIdError::ResourceUnavailable(
                        err.msg,
                    ))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(DeleteUserByPrincipalIdError::Throttling(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DeleteUserByPrincipalIdError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteUserByPrincipalIdError::AccessDenied(ref cause) => write!(f, "{}", cause),
            DeleteUserByPrincipalIdError::InternalFailure(ref cause) => write!(f, "{}", cause),
            DeleteUserByPrincipalIdError::InvalidParameterValue(ref cause) => {
                write!(f, "{}", cause)
            }
            DeleteUserByPrincipalIdError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            DeleteUserByPrincipalIdError::ResourceUnavailable(ref cause) => write!(f, "{}", cause),
            DeleteUserByPrincipalIdError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeleteUserByPrincipalIdError {}
/// Errors returned by DescribeDashboard
#[derive(Debug, PartialEq)]
pub enum DescribeDashboardError {
    /// <p>You don't have access to this item. The provided credentials couldn't be validated. You might not be authorized to carry out the request. Make sure that your account is authorized to use the Amazon QuickSight service, that your policies have the correct permissions, and that you are using the correct access keys.</p>
    AccessDenied(String),
    /// <p>An internal failure occurred.</p>
    InternalFailure(String),
    /// <p>One or more parameters has a value that isn't valid.</p>
    InvalidParameterValue(String),
    /// <p>One or more resources can't be found.</p>
    ResourceNotFound(String),
    /// <p>Access is throttled.</p>
    Throttling(String),
    /// <p>This error indicates that you are calling an operation on an Amazon QuickSight subscription where the edition doesn't include support for that operation. Amazon QuickSight currently has Standard Edition and Enterprise Edition. Not every operation and capability is available in every edition.</p>
    UnsupportedUserEdition(String),
}

impl DescribeDashboardError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeDashboardError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(DescribeDashboardError::AccessDenied(err.msg))
                }
                "InternalFailureException" => {
                    return RusotoError::Service(DescribeDashboardError::InternalFailure(err.msg))
                }
                "InvalidParameterValueException" => {
                    return RusotoError::Service(DescribeDashboardError::InvalidParameterValue(
                        err.msg,
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(DescribeDashboardError::ResourceNotFound(err.msg))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(DescribeDashboardError::Throttling(err.msg))
                }
                "UnsupportedUserEditionException" => {
                    return RusotoError::Service(DescribeDashboardError::UnsupportedUserEdition(
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
impl fmt::Display for DescribeDashboardError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeDashboardError::AccessDenied(ref cause) => write!(f, "{}", cause),
            DescribeDashboardError::InternalFailure(ref cause) => write!(f, "{}", cause),
            DescribeDashboardError::InvalidParameterValue(ref cause) => write!(f, "{}", cause),
            DescribeDashboardError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            DescribeDashboardError::Throttling(ref cause) => write!(f, "{}", cause),
            DescribeDashboardError::UnsupportedUserEdition(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DescribeDashboardError {}
/// Errors returned by DescribeDashboardPermissions
#[derive(Debug, PartialEq)]
pub enum DescribeDashboardPermissionsError {
    /// <p>An internal failure occurred.</p>
    InternalFailure(String),
    /// <p>One or more parameters has a value that isn't valid.</p>
    InvalidParameterValue(String),
    /// <p>One or more resources can't be found.</p>
    ResourceNotFound(String),
    /// <p>Access is throttled.</p>
    Throttling(String),
    /// <p>This error indicates that you are calling an operation on an Amazon QuickSight subscription where the edition doesn't include support for that operation. Amazon QuickSight currently has Standard Edition and Enterprise Edition. Not every operation and capability is available in every edition.</p>
    UnsupportedUserEdition(String),
}

impl DescribeDashboardPermissionsError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<DescribeDashboardPermissionsError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InternalFailureException" => {
                    return RusotoError::Service(
                        DescribeDashboardPermissionsError::InternalFailure(err.msg),
                    )
                }
                "InvalidParameterValueException" => {
                    return RusotoError::Service(
                        DescribeDashboardPermissionsError::InvalidParameterValue(err.msg),
                    )
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(
                        DescribeDashboardPermissionsError::ResourceNotFound(err.msg),
                    )
                }
                "ThrottlingException" => {
                    return RusotoError::Service(DescribeDashboardPermissionsError::Throttling(
                        err.msg,
                    ))
                }
                "UnsupportedUserEditionException" => {
                    return RusotoError::Service(
                        DescribeDashboardPermissionsError::UnsupportedUserEdition(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DescribeDashboardPermissionsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeDashboardPermissionsError::InternalFailure(ref cause) => write!(f, "{}", cause),
            DescribeDashboardPermissionsError::InvalidParameterValue(ref cause) => {
                write!(f, "{}", cause)
            }
            DescribeDashboardPermissionsError::ResourceNotFound(ref cause) => {
                write!(f, "{}", cause)
            }
            DescribeDashboardPermissionsError::Throttling(ref cause) => write!(f, "{}", cause),
            DescribeDashboardPermissionsError::UnsupportedUserEdition(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for DescribeDashboardPermissionsError {}
/// Errors returned by DescribeDataSet
#[derive(Debug, PartialEq)]
pub enum DescribeDataSetError {
    /// <p>You don't have access to this item. The provided credentials couldn't be validated. You might not be authorized to carry out the request. Make sure that your account is authorized to use the Amazon QuickSight service, that your policies have the correct permissions, and that you are using the correct access keys.</p>
    AccessDenied(String),
    /// <p>An internal failure occurred.</p>
    InternalFailure(String),
    /// <p>One or more parameters has a value that isn't valid.</p>
    InvalidParameterValue(String),
    /// <p>One or more resources can't be found.</p>
    ResourceNotFound(String),
    /// <p>Access is throttled.</p>
    Throttling(String),
}

impl DescribeDataSetError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeDataSetError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(DescribeDataSetError::AccessDenied(err.msg))
                }
                "InternalFailureException" => {
                    return RusotoError::Service(DescribeDataSetError::InternalFailure(err.msg))
                }
                "InvalidParameterValueException" => {
                    return RusotoError::Service(DescribeDataSetError::InvalidParameterValue(
                        err.msg,
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(DescribeDataSetError::ResourceNotFound(err.msg))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(DescribeDataSetError::Throttling(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DescribeDataSetError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeDataSetError::AccessDenied(ref cause) => write!(f, "{}", cause),
            DescribeDataSetError::InternalFailure(ref cause) => write!(f, "{}", cause),
            DescribeDataSetError::InvalidParameterValue(ref cause) => write!(f, "{}", cause),
            DescribeDataSetError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            DescribeDataSetError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DescribeDataSetError {}
/// Errors returned by DescribeDataSetPermissions
#[derive(Debug, PartialEq)]
pub enum DescribeDataSetPermissionsError {
    /// <p>You don't have access to this item. The provided credentials couldn't be validated. You might not be authorized to carry out the request. Make sure that your account is authorized to use the Amazon QuickSight service, that your policies have the correct permissions, and that you are using the correct access keys.</p>
    AccessDenied(String),
    /// <p>An internal failure occurred.</p>
    InternalFailure(String),
    /// <p>One or more parameters has a value that isn't valid.</p>
    InvalidParameterValue(String),
    /// <p>One or more resources can't be found.</p>
    ResourceNotFound(String),
    /// <p>Access is throttled.</p>
    Throttling(String),
}

impl DescribeDataSetPermissionsError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<DescribeDataSetPermissionsError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(DescribeDataSetPermissionsError::AccessDenied(
                        err.msg,
                    ))
                }
                "InternalFailureException" => {
                    return RusotoError::Service(DescribeDataSetPermissionsError::InternalFailure(
                        err.msg,
                    ))
                }
                "InvalidParameterValueException" => {
                    return RusotoError::Service(
                        DescribeDataSetPermissionsError::InvalidParameterValue(err.msg),
                    )
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(DescribeDataSetPermissionsError::ResourceNotFound(
                        err.msg,
                    ))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(DescribeDataSetPermissionsError::Throttling(
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
impl fmt::Display for DescribeDataSetPermissionsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeDataSetPermissionsError::AccessDenied(ref cause) => write!(f, "{}", cause),
            DescribeDataSetPermissionsError::InternalFailure(ref cause) => write!(f, "{}", cause),
            DescribeDataSetPermissionsError::InvalidParameterValue(ref cause) => {
                write!(f, "{}", cause)
            }
            DescribeDataSetPermissionsError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            DescribeDataSetPermissionsError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DescribeDataSetPermissionsError {}
/// Errors returned by DescribeDataSource
#[derive(Debug, PartialEq)]
pub enum DescribeDataSourceError {
    /// <p>You don't have access to this item. The provided credentials couldn't be validated. You might not be authorized to carry out the request. Make sure that your account is authorized to use the Amazon QuickSight service, that your policies have the correct permissions, and that you are using the correct access keys.</p>
    AccessDenied(String),
    /// <p>An internal failure occurred.</p>
    InternalFailure(String),
    /// <p>One or more parameters has a value that isn't valid.</p>
    InvalidParameterValue(String),
    /// <p>One or more resources can't be found.</p>
    ResourceNotFound(String),
    /// <p>Access is throttled.</p>
    Throttling(String),
}

impl DescribeDataSourceError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeDataSourceError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(DescribeDataSourceError::AccessDenied(err.msg))
                }
                "InternalFailureException" => {
                    return RusotoError::Service(DescribeDataSourceError::InternalFailure(err.msg))
                }
                "InvalidParameterValueException" => {
                    return RusotoError::Service(DescribeDataSourceError::InvalidParameterValue(
                        err.msg,
                    ))
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
            DescribeDataSourceError::InternalFailure(ref cause) => write!(f, "{}", cause),
            DescribeDataSourceError::InvalidParameterValue(ref cause) => write!(f, "{}", cause),
            DescribeDataSourceError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            DescribeDataSourceError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DescribeDataSourceError {}
/// Errors returned by DescribeDataSourcePermissions
#[derive(Debug, PartialEq)]
pub enum DescribeDataSourcePermissionsError {
    /// <p>You don't have access to this item. The provided credentials couldn't be validated. You might not be authorized to carry out the request. Make sure that your account is authorized to use the Amazon QuickSight service, that your policies have the correct permissions, and that you are using the correct access keys.</p>
    AccessDenied(String),
    /// <p>An internal failure occurred.</p>
    InternalFailure(String),
    /// <p>One or more parameters has a value that isn't valid.</p>
    InvalidParameterValue(String),
    /// <p>One or more resources can't be found.</p>
    ResourceNotFound(String),
    /// <p>Access is throttled.</p>
    Throttling(String),
}

impl DescribeDataSourcePermissionsError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<DescribeDataSourcePermissionsError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(DescribeDataSourcePermissionsError::AccessDenied(
                        err.msg,
                    ))
                }
                "InternalFailureException" => {
                    return RusotoError::Service(
                        DescribeDataSourcePermissionsError::InternalFailure(err.msg),
                    )
                }
                "InvalidParameterValueException" => {
                    return RusotoError::Service(
                        DescribeDataSourcePermissionsError::InvalidParameterValue(err.msg),
                    )
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(
                        DescribeDataSourcePermissionsError::ResourceNotFound(err.msg),
                    )
                }
                "ThrottlingException" => {
                    return RusotoError::Service(DescribeDataSourcePermissionsError::Throttling(
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
impl fmt::Display for DescribeDataSourcePermissionsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeDataSourcePermissionsError::AccessDenied(ref cause) => write!(f, "{}", cause),
            DescribeDataSourcePermissionsError::InternalFailure(ref cause) => {
                write!(f, "{}", cause)
            }
            DescribeDataSourcePermissionsError::InvalidParameterValue(ref cause) => {
                write!(f, "{}", cause)
            }
            DescribeDataSourcePermissionsError::ResourceNotFound(ref cause) => {
                write!(f, "{}", cause)
            }
            DescribeDataSourcePermissionsError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DescribeDataSourcePermissionsError {}
/// Errors returned by DescribeGroup
#[derive(Debug, PartialEq)]
pub enum DescribeGroupError {
    /// <p>You don't have access to this item. The provided credentials couldn't be validated. You might not be authorized to carry out the request. Make sure that your account is authorized to use the Amazon QuickSight service, that your policies have the correct permissions, and that you are using the correct access keys.</p>
    AccessDenied(String),
    /// <p>An internal failure occurred.</p>
    InternalFailure(String),
    /// <p>One or more parameters has a value that isn't valid.</p>
    InvalidParameterValue(String),
    /// <p>One or more preconditions aren't met.</p>
    PreconditionNotMet(String),
    /// <p>One or more resources can't be found.</p>
    ResourceNotFound(String),
    /// <p>This resource is currently unavailable.</p>
    ResourceUnavailable(String),
    /// <p>Access is throttled.</p>
    Throttling(String),
}

impl DescribeGroupError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeGroupError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(DescribeGroupError::AccessDenied(err.msg))
                }
                "InternalFailureException" => {
                    return RusotoError::Service(DescribeGroupError::InternalFailure(err.msg))
                }
                "InvalidParameterValueException" => {
                    return RusotoError::Service(DescribeGroupError::InvalidParameterValue(err.msg))
                }
                "PreconditionNotMetException" => {
                    return RusotoError::Service(DescribeGroupError::PreconditionNotMet(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(DescribeGroupError::ResourceNotFound(err.msg))
                }
                "ResourceUnavailableException" => {
                    return RusotoError::Service(DescribeGroupError::ResourceUnavailable(err.msg))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(DescribeGroupError::Throttling(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DescribeGroupError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeGroupError::AccessDenied(ref cause) => write!(f, "{}", cause),
            DescribeGroupError::InternalFailure(ref cause) => write!(f, "{}", cause),
            DescribeGroupError::InvalidParameterValue(ref cause) => write!(f, "{}", cause),
            DescribeGroupError::PreconditionNotMet(ref cause) => write!(f, "{}", cause),
            DescribeGroupError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            DescribeGroupError::ResourceUnavailable(ref cause) => write!(f, "{}", cause),
            DescribeGroupError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DescribeGroupError {}
/// Errors returned by DescribeIAMPolicyAssignment
#[derive(Debug, PartialEq)]
pub enum DescribeIAMPolicyAssignmentError {
    /// <p>You don't have access to this item. The provided credentials couldn't be validated. You might not be authorized to carry out the request. Make sure that your account is authorized to use the Amazon QuickSight service, that your policies have the correct permissions, and that you are using the correct access keys.</p>
    AccessDenied(String),
    /// <p>An internal failure occurred.</p>
    InternalFailure(String),
    /// <p>The <code>NextToken</code> value isn't valid.</p>
    InvalidNextToken(String),
    /// <p>One or more parameters has a value that isn't valid.</p>
    InvalidParameterValue(String),
    /// <p>One or more resources can't be found.</p>
    ResourceNotFound(String),
    /// <p>Access is throttled.</p>
    Throttling(String),
}

impl DescribeIAMPolicyAssignmentError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<DescribeIAMPolicyAssignmentError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(DescribeIAMPolicyAssignmentError::AccessDenied(
                        err.msg,
                    ))
                }
                "InternalFailureException" => {
                    return RusotoError::Service(DescribeIAMPolicyAssignmentError::InternalFailure(
                        err.msg,
                    ))
                }
                "InvalidNextTokenException" => {
                    return RusotoError::Service(
                        DescribeIAMPolicyAssignmentError::InvalidNextToken(err.msg),
                    )
                }
                "InvalidParameterValueException" => {
                    return RusotoError::Service(
                        DescribeIAMPolicyAssignmentError::InvalidParameterValue(err.msg),
                    )
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(
                        DescribeIAMPolicyAssignmentError::ResourceNotFound(err.msg),
                    )
                }
                "ThrottlingException" => {
                    return RusotoError::Service(DescribeIAMPolicyAssignmentError::Throttling(
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
impl fmt::Display for DescribeIAMPolicyAssignmentError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeIAMPolicyAssignmentError::AccessDenied(ref cause) => write!(f, "{}", cause),
            DescribeIAMPolicyAssignmentError::InternalFailure(ref cause) => write!(f, "{}", cause),
            DescribeIAMPolicyAssignmentError::InvalidNextToken(ref cause) => write!(f, "{}", cause),
            DescribeIAMPolicyAssignmentError::InvalidParameterValue(ref cause) => {
                write!(f, "{}", cause)
            }
            DescribeIAMPolicyAssignmentError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            DescribeIAMPolicyAssignmentError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DescribeIAMPolicyAssignmentError {}
/// Errors returned by DescribeIngestion
#[derive(Debug, PartialEq)]
pub enum DescribeIngestionError {
    /// <p>You don't have access to this item. The provided credentials couldn't be validated. You might not be authorized to carry out the request. Make sure that your account is authorized to use the Amazon QuickSight service, that your policies have the correct permissions, and that you are using the correct access keys.</p>
    AccessDenied(String),
    /// <p>An internal failure occurred.</p>
    InternalFailure(String),
    /// <p>One or more parameters has a value that isn't valid.</p>
    InvalidParameterValue(String),
    /// <p>The resource specified already exists. </p>
    ResourceExists(String),
    /// <p>One or more resources can't be found.</p>
    ResourceNotFound(String),
    /// <p>Access is throttled.</p>
    Throttling(String),
}

impl DescribeIngestionError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeIngestionError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(DescribeIngestionError::AccessDenied(err.msg))
                }
                "InternalFailureException" => {
                    return RusotoError::Service(DescribeIngestionError::InternalFailure(err.msg))
                }
                "InvalidParameterValueException" => {
                    return RusotoError::Service(DescribeIngestionError::InvalidParameterValue(
                        err.msg,
                    ))
                }
                "ResourceExistsException" => {
                    return RusotoError::Service(DescribeIngestionError::ResourceExists(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(DescribeIngestionError::ResourceNotFound(err.msg))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(DescribeIngestionError::Throttling(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DescribeIngestionError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeIngestionError::AccessDenied(ref cause) => write!(f, "{}", cause),
            DescribeIngestionError::InternalFailure(ref cause) => write!(f, "{}", cause),
            DescribeIngestionError::InvalidParameterValue(ref cause) => write!(f, "{}", cause),
            DescribeIngestionError::ResourceExists(ref cause) => write!(f, "{}", cause),
            DescribeIngestionError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            DescribeIngestionError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DescribeIngestionError {}
/// Errors returned by DescribeTemplate
#[derive(Debug, PartialEq)]
pub enum DescribeTemplateError {
    /// <p>You don't have access to this item. The provided credentials couldn't be validated. You might not be authorized to carry out the request. Make sure that your account is authorized to use the Amazon QuickSight service, that your policies have the correct permissions, and that you are using the correct access keys.</p>
    AccessDenied(String),
    /// <p>Updating or deleting a resource can cause an inconsistent state.</p>
    Conflict(String),
    /// <p>An internal failure occurred.</p>
    InternalFailure(String),
    /// <p>One or more parameters has a value that isn't valid.</p>
    InvalidParameterValue(String),
    /// <p>The resource specified already exists. </p>
    ResourceExists(String),
    /// <p>One or more resources can't be found.</p>
    ResourceNotFound(String),
    /// <p>Access is throttled.</p>
    Throttling(String),
    /// <p>This error indicates that you are calling an operation on an Amazon QuickSight subscription where the edition doesn't include support for that operation. Amazon QuickSight currently has Standard Edition and Enterprise Edition. Not every operation and capability is available in every edition.</p>
    UnsupportedUserEdition(String),
}

impl DescribeTemplateError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeTemplateError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(DescribeTemplateError::AccessDenied(err.msg))
                }
                "ConflictException" => {
                    return RusotoError::Service(DescribeTemplateError::Conflict(err.msg))
                }
                "InternalFailureException" => {
                    return RusotoError::Service(DescribeTemplateError::InternalFailure(err.msg))
                }
                "InvalidParameterValueException" => {
                    return RusotoError::Service(DescribeTemplateError::InvalidParameterValue(
                        err.msg,
                    ))
                }
                "ResourceExistsException" => {
                    return RusotoError::Service(DescribeTemplateError::ResourceExists(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(DescribeTemplateError::ResourceNotFound(err.msg))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(DescribeTemplateError::Throttling(err.msg))
                }
                "UnsupportedUserEditionException" => {
                    return RusotoError::Service(DescribeTemplateError::UnsupportedUserEdition(
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
impl fmt::Display for DescribeTemplateError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeTemplateError::AccessDenied(ref cause) => write!(f, "{}", cause),
            DescribeTemplateError::Conflict(ref cause) => write!(f, "{}", cause),
            DescribeTemplateError::InternalFailure(ref cause) => write!(f, "{}", cause),
            DescribeTemplateError::InvalidParameterValue(ref cause) => write!(f, "{}", cause),
            DescribeTemplateError::ResourceExists(ref cause) => write!(f, "{}", cause),
            DescribeTemplateError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            DescribeTemplateError::Throttling(ref cause) => write!(f, "{}", cause),
            DescribeTemplateError::UnsupportedUserEdition(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DescribeTemplateError {}
/// Errors returned by DescribeTemplateAlias
#[derive(Debug, PartialEq)]
pub enum DescribeTemplateAliasError {
    /// <p>An internal failure occurred.</p>
    InternalFailure(String),
    /// <p>One or more resources can't be found.</p>
    ResourceNotFound(String),
    /// <p>Access is throttled.</p>
    Throttling(String),
    /// <p>This error indicates that you are calling an operation on an Amazon QuickSight subscription where the edition doesn't include support for that operation. Amazon QuickSight currently has Standard Edition and Enterprise Edition. Not every operation and capability is available in every edition.</p>
    UnsupportedUserEdition(String),
}

impl DescribeTemplateAliasError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeTemplateAliasError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InternalFailureException" => {
                    return RusotoError::Service(DescribeTemplateAliasError::InternalFailure(
                        err.msg,
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(DescribeTemplateAliasError::ResourceNotFound(
                        err.msg,
                    ))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(DescribeTemplateAliasError::Throttling(err.msg))
                }
                "UnsupportedUserEditionException" => {
                    return RusotoError::Service(
                        DescribeTemplateAliasError::UnsupportedUserEdition(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DescribeTemplateAliasError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeTemplateAliasError::InternalFailure(ref cause) => write!(f, "{}", cause),
            DescribeTemplateAliasError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            DescribeTemplateAliasError::Throttling(ref cause) => write!(f, "{}", cause),
            DescribeTemplateAliasError::UnsupportedUserEdition(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DescribeTemplateAliasError {}
/// Errors returned by DescribeTemplatePermissions
#[derive(Debug, PartialEq)]
pub enum DescribeTemplatePermissionsError {
    /// <p>Updating or deleting a resource can cause an inconsistent state.</p>
    Conflict(String),
    /// <p>An internal failure occurred.</p>
    InternalFailure(String),
    /// <p>One or more parameters has a value that isn't valid.</p>
    InvalidParameterValue(String),
    /// <p>One or more resources can't be found.</p>
    ResourceNotFound(String),
    /// <p>Access is throttled.</p>
    Throttling(String),
    /// <p>This error indicates that you are calling an operation on an Amazon QuickSight subscription where the edition doesn't include support for that operation. Amazon QuickSight currently has Standard Edition and Enterprise Edition. Not every operation and capability is available in every edition.</p>
    UnsupportedUserEdition(String),
}

impl DescribeTemplatePermissionsError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<DescribeTemplatePermissionsError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "ConflictException" => {
                    return RusotoError::Service(DescribeTemplatePermissionsError::Conflict(
                        err.msg,
                    ))
                }
                "InternalFailureException" => {
                    return RusotoError::Service(DescribeTemplatePermissionsError::InternalFailure(
                        err.msg,
                    ))
                }
                "InvalidParameterValueException" => {
                    return RusotoError::Service(
                        DescribeTemplatePermissionsError::InvalidParameterValue(err.msg),
                    )
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(
                        DescribeTemplatePermissionsError::ResourceNotFound(err.msg),
                    )
                }
                "ThrottlingException" => {
                    return RusotoError::Service(DescribeTemplatePermissionsError::Throttling(
                        err.msg,
                    ))
                }
                "UnsupportedUserEditionException" => {
                    return RusotoError::Service(
                        DescribeTemplatePermissionsError::UnsupportedUserEdition(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DescribeTemplatePermissionsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeTemplatePermissionsError::Conflict(ref cause) => write!(f, "{}", cause),
            DescribeTemplatePermissionsError::InternalFailure(ref cause) => write!(f, "{}", cause),
            DescribeTemplatePermissionsError::InvalidParameterValue(ref cause) => {
                write!(f, "{}", cause)
            }
            DescribeTemplatePermissionsError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            DescribeTemplatePermissionsError::Throttling(ref cause) => write!(f, "{}", cause),
            DescribeTemplatePermissionsError::UnsupportedUserEdition(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for DescribeTemplatePermissionsError {}
/// Errors returned by DescribeUser
#[derive(Debug, PartialEq)]
pub enum DescribeUserError {
    /// <p>You don't have access to this item. The provided credentials couldn't be validated. You might not be authorized to carry out the request. Make sure that your account is authorized to use the Amazon QuickSight service, that your policies have the correct permissions, and that you are using the correct access keys.</p>
    AccessDenied(String),
    /// <p>An internal failure occurred.</p>
    InternalFailure(String),
    /// <p>One or more parameters has a value that isn't valid.</p>
    InvalidParameterValue(String),
    /// <p>One or more resources can't be found.</p>
    ResourceNotFound(String),
    /// <p>This resource is currently unavailable.</p>
    ResourceUnavailable(String),
    /// <p>Access is throttled.</p>
    Throttling(String),
}

impl DescribeUserError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeUserError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(DescribeUserError::AccessDenied(err.msg))
                }
                "InternalFailureException" => {
                    return RusotoError::Service(DescribeUserError::InternalFailure(err.msg))
                }
                "InvalidParameterValueException" => {
                    return RusotoError::Service(DescribeUserError::InvalidParameterValue(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(DescribeUserError::ResourceNotFound(err.msg))
                }
                "ResourceUnavailableException" => {
                    return RusotoError::Service(DescribeUserError::ResourceUnavailable(err.msg))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(DescribeUserError::Throttling(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DescribeUserError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeUserError::AccessDenied(ref cause) => write!(f, "{}", cause),
            DescribeUserError::InternalFailure(ref cause) => write!(f, "{}", cause),
            DescribeUserError::InvalidParameterValue(ref cause) => write!(f, "{}", cause),
            DescribeUserError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            DescribeUserError::ResourceUnavailable(ref cause) => write!(f, "{}", cause),
            DescribeUserError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DescribeUserError {}
/// Errors returned by GetDashboardEmbedUrl
#[derive(Debug, PartialEq)]
pub enum GetDashboardEmbedUrlError {
    /// <p>You don't have access to this item. The provided credentials couldn't be validated. You might not be authorized to carry out the request. Make sure that your account is authorized to use the Amazon QuickSight service, that your policies have the correct permissions, and that you are using the correct access keys.</p>
    AccessDenied(String),
    /// <p>The domain specified isn't on the allow list. All domains for embedded dashboards must be added to the approved list by an Amazon QuickSight admin.</p>
    DomainNotWhitelisted(String),
    /// <p>The identity type specified isn't supported. Supported identity types include <code>IAM</code> and <code>QUICKSIGHT</code>.</p>
    IdentityTypeNotSupported(String),
    /// <p>An internal failure occurred.</p>
    InternalFailure(String),
    /// <p>One or more parameters has a value that isn't valid.</p>
    InvalidParameterValue(String),
    /// <p>The user with the provided name isn't found. This error can happen in any operation that requires finding a user based on a provided user name, such as <code>DeleteUser</code>, <code>DescribeUser</code>, and so on.</p>
    QuickSightUserNotFound(String),
    /// <p>The resource specified already exists. </p>
    ResourceExists(String),
    /// <p>One or more resources can't be found.</p>
    ResourceNotFound(String),
    /// <p>The number of minutes specified for the lifetime of a session isn't valid. The session lifetime must be 15-600 minutes.</p>
    SessionLifetimeInMinutesInvalid(String),
    /// <p>Access is throttled.</p>
    Throttling(String),
    /// <p>This error indicates that you are calling an operation on an Amazon QuickSight subscription where the edition doesn't include support for that operation. Amazon QuickSight currently has Standard Edition and Enterprise Edition. Not every operation and capability is available in every edition.</p>
    UnsupportedUserEdition(String),
}

impl GetDashboardEmbedUrlError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetDashboardEmbedUrlError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(GetDashboardEmbedUrlError::AccessDenied(err.msg))
                }
                "DomainNotWhitelistedException" => {
                    return RusotoError::Service(GetDashboardEmbedUrlError::DomainNotWhitelisted(
                        err.msg,
                    ))
                }
                "IdentityTypeNotSupportedException" => {
                    return RusotoError::Service(
                        GetDashboardEmbedUrlError::IdentityTypeNotSupported(err.msg),
                    )
                }
                "InternalFailureException" => {
                    return RusotoError::Service(GetDashboardEmbedUrlError::InternalFailure(
                        err.msg,
                    ))
                }
                "InvalidParameterValueException" => {
                    return RusotoError::Service(GetDashboardEmbedUrlError::InvalidParameterValue(
                        err.msg,
                    ))
                }
                "QuickSightUserNotFoundException" => {
                    return RusotoError::Service(GetDashboardEmbedUrlError::QuickSightUserNotFound(
                        err.msg,
                    ))
                }
                "ResourceExistsException" => {
                    return RusotoError::Service(GetDashboardEmbedUrlError::ResourceExists(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(GetDashboardEmbedUrlError::ResourceNotFound(
                        err.msg,
                    ))
                }
                "SessionLifetimeInMinutesInvalidException" => {
                    return RusotoError::Service(
                        GetDashboardEmbedUrlError::SessionLifetimeInMinutesInvalid(err.msg),
                    )
                }
                "ThrottlingException" => {
                    return RusotoError::Service(GetDashboardEmbedUrlError::Throttling(err.msg))
                }
                "UnsupportedUserEditionException" => {
                    return RusotoError::Service(GetDashboardEmbedUrlError::UnsupportedUserEdition(
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
impl fmt::Display for GetDashboardEmbedUrlError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetDashboardEmbedUrlError::AccessDenied(ref cause) => write!(f, "{}", cause),
            GetDashboardEmbedUrlError::DomainNotWhitelisted(ref cause) => write!(f, "{}", cause),
            GetDashboardEmbedUrlError::IdentityTypeNotSupported(ref cause) => {
                write!(f, "{}", cause)
            }
            GetDashboardEmbedUrlError::InternalFailure(ref cause) => write!(f, "{}", cause),
            GetDashboardEmbedUrlError::InvalidParameterValue(ref cause) => write!(f, "{}", cause),
            GetDashboardEmbedUrlError::QuickSightUserNotFound(ref cause) => write!(f, "{}", cause),
            GetDashboardEmbedUrlError::ResourceExists(ref cause) => write!(f, "{}", cause),
            GetDashboardEmbedUrlError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            GetDashboardEmbedUrlError::SessionLifetimeInMinutesInvalid(ref cause) => {
                write!(f, "{}", cause)
            }
            GetDashboardEmbedUrlError::Throttling(ref cause) => write!(f, "{}", cause),
            GetDashboardEmbedUrlError::UnsupportedUserEdition(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetDashboardEmbedUrlError {}
/// Errors returned by ListDashboardVersions
#[derive(Debug, PartialEq)]
pub enum ListDashboardVersionsError {
    /// <p>An internal failure occurred.</p>
    InternalFailure(String),
    /// <p>The <code>NextToken</code> value isn't valid.</p>
    InvalidNextToken(String),
    /// <p>One or more parameters has a value that isn't valid.</p>
    InvalidParameterValue(String),
    /// <p>One or more resources can't be found.</p>
    ResourceNotFound(String),
    /// <p>Access is throttled.</p>
    Throttling(String),
    /// <p>This error indicates that you are calling an operation on an Amazon QuickSight subscription where the edition doesn't include support for that operation. Amazon QuickSight currently has Standard Edition and Enterprise Edition. Not every operation and capability is available in every edition.</p>
    UnsupportedUserEdition(String),
}

impl ListDashboardVersionsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListDashboardVersionsError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InternalFailureException" => {
                    return RusotoError::Service(ListDashboardVersionsError::InternalFailure(
                        err.msg,
                    ))
                }
                "InvalidNextTokenException" => {
                    return RusotoError::Service(ListDashboardVersionsError::InvalidNextToken(
                        err.msg,
                    ))
                }
                "InvalidParameterValueException" => {
                    return RusotoError::Service(ListDashboardVersionsError::InvalidParameterValue(
                        err.msg,
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(ListDashboardVersionsError::ResourceNotFound(
                        err.msg,
                    ))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(ListDashboardVersionsError::Throttling(err.msg))
                }
                "UnsupportedUserEditionException" => {
                    return RusotoError::Service(
                        ListDashboardVersionsError::UnsupportedUserEdition(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ListDashboardVersionsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListDashboardVersionsError::InternalFailure(ref cause) => write!(f, "{}", cause),
            ListDashboardVersionsError::InvalidNextToken(ref cause) => write!(f, "{}", cause),
            ListDashboardVersionsError::InvalidParameterValue(ref cause) => write!(f, "{}", cause),
            ListDashboardVersionsError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            ListDashboardVersionsError::Throttling(ref cause) => write!(f, "{}", cause),
            ListDashboardVersionsError::UnsupportedUserEdition(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListDashboardVersionsError {}
/// Errors returned by ListDashboards
#[derive(Debug, PartialEq)]
pub enum ListDashboardsError {
    /// <p>An internal failure occurred.</p>
    InternalFailure(String),
    /// <p>The <code>NextToken</code> value isn't valid.</p>
    InvalidNextToken(String),
    /// <p>Access is throttled.</p>
    Throttling(String),
    /// <p>This error indicates that you are calling an operation on an Amazon QuickSight subscription where the edition doesn't include support for that operation. Amazon QuickSight currently has Standard Edition and Enterprise Edition. Not every operation and capability is available in every edition.</p>
    UnsupportedUserEdition(String),
}

impl ListDashboardsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListDashboardsError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InternalFailureException" => {
                    return RusotoError::Service(ListDashboardsError::InternalFailure(err.msg))
                }
                "InvalidNextTokenException" => {
                    return RusotoError::Service(ListDashboardsError::InvalidNextToken(err.msg))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(ListDashboardsError::Throttling(err.msg))
                }
                "UnsupportedUserEditionException" => {
                    return RusotoError::Service(ListDashboardsError::UnsupportedUserEdition(
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
impl fmt::Display for ListDashboardsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListDashboardsError::InternalFailure(ref cause) => write!(f, "{}", cause),
            ListDashboardsError::InvalidNextToken(ref cause) => write!(f, "{}", cause),
            ListDashboardsError::Throttling(ref cause) => write!(f, "{}", cause),
            ListDashboardsError::UnsupportedUserEdition(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListDashboardsError {}
/// Errors returned by ListDataSets
#[derive(Debug, PartialEq)]
pub enum ListDataSetsError {
    /// <p>You don't have access to this item. The provided credentials couldn't be validated. You might not be authorized to carry out the request. Make sure that your account is authorized to use the Amazon QuickSight service, that your policies have the correct permissions, and that you are using the correct access keys.</p>
    AccessDenied(String),
    /// <p>An internal failure occurred.</p>
    InternalFailure(String),
    /// <p>The <code>NextToken</code> value isn't valid.</p>
    InvalidNextToken(String),
    /// <p>One or more parameters has a value that isn't valid.</p>
    InvalidParameterValue(String),
    /// <p>Access is throttled.</p>
    Throttling(String),
}

impl ListDataSetsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListDataSetsError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(ListDataSetsError::AccessDenied(err.msg))
                }
                "InternalFailureException" => {
                    return RusotoError::Service(ListDataSetsError::InternalFailure(err.msg))
                }
                "InvalidNextTokenException" => {
                    return RusotoError::Service(ListDataSetsError::InvalidNextToken(err.msg))
                }
                "InvalidParameterValueException" => {
                    return RusotoError::Service(ListDataSetsError::InvalidParameterValue(err.msg))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(ListDataSetsError::Throttling(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ListDataSetsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListDataSetsError::AccessDenied(ref cause) => write!(f, "{}", cause),
            ListDataSetsError::InternalFailure(ref cause) => write!(f, "{}", cause),
            ListDataSetsError::InvalidNextToken(ref cause) => write!(f, "{}", cause),
            ListDataSetsError::InvalidParameterValue(ref cause) => write!(f, "{}", cause),
            ListDataSetsError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListDataSetsError {}
/// Errors returned by ListDataSources
#[derive(Debug, PartialEq)]
pub enum ListDataSourcesError {
    /// <p>You don't have access to this item. The provided credentials couldn't be validated. You might not be authorized to carry out the request. Make sure that your account is authorized to use the Amazon QuickSight service, that your policies have the correct permissions, and that you are using the correct access keys.</p>
    AccessDenied(String),
    /// <p>An internal failure occurred.</p>
    InternalFailure(String),
    /// <p>The <code>NextToken</code> value isn't valid.</p>
    InvalidNextToken(String),
    /// <p>One or more parameters has a value that isn't valid.</p>
    InvalidParameterValue(String),
    /// <p>Access is throttled.</p>
    Throttling(String),
}

impl ListDataSourcesError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListDataSourcesError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(ListDataSourcesError::AccessDenied(err.msg))
                }
                "InternalFailureException" => {
                    return RusotoError::Service(ListDataSourcesError::InternalFailure(err.msg))
                }
                "InvalidNextTokenException" => {
                    return RusotoError::Service(ListDataSourcesError::InvalidNextToken(err.msg))
                }
                "InvalidParameterValueException" => {
                    return RusotoError::Service(ListDataSourcesError::InvalidParameterValue(
                        err.msg,
                    ))
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
            ListDataSourcesError::InternalFailure(ref cause) => write!(f, "{}", cause),
            ListDataSourcesError::InvalidNextToken(ref cause) => write!(f, "{}", cause),
            ListDataSourcesError::InvalidParameterValue(ref cause) => write!(f, "{}", cause),
            ListDataSourcesError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListDataSourcesError {}
/// Errors returned by ListGroupMemberships
#[derive(Debug, PartialEq)]
pub enum ListGroupMembershipsError {
    /// <p>You don't have access to this item. The provided credentials couldn't be validated. You might not be authorized to carry out the request. Make sure that your account is authorized to use the Amazon QuickSight service, that your policies have the correct permissions, and that you are using the correct access keys.</p>
    AccessDenied(String),
    /// <p>An internal failure occurred.</p>
    InternalFailure(String),
    /// <p>The <code>NextToken</code> value isn't valid.</p>
    InvalidNextToken(String),
    /// <p>One or more parameters has a value that isn't valid.</p>
    InvalidParameterValue(String),
    /// <p>One or more preconditions aren't met.</p>
    PreconditionNotMet(String),
    /// <p>One or more resources can't be found.</p>
    ResourceNotFound(String),
    /// <p>This resource is currently unavailable.</p>
    ResourceUnavailable(String),
    /// <p>Access is throttled.</p>
    Throttling(String),
}

impl ListGroupMembershipsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListGroupMembershipsError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(ListGroupMembershipsError::AccessDenied(err.msg))
                }
                "InternalFailureException" => {
                    return RusotoError::Service(ListGroupMembershipsError::InternalFailure(
                        err.msg,
                    ))
                }
                "InvalidNextTokenException" => {
                    return RusotoError::Service(ListGroupMembershipsError::InvalidNextToken(
                        err.msg,
                    ))
                }
                "InvalidParameterValueException" => {
                    return RusotoError::Service(ListGroupMembershipsError::InvalidParameterValue(
                        err.msg,
                    ))
                }
                "PreconditionNotMetException" => {
                    return RusotoError::Service(ListGroupMembershipsError::PreconditionNotMet(
                        err.msg,
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(ListGroupMembershipsError::ResourceNotFound(
                        err.msg,
                    ))
                }
                "ResourceUnavailableException" => {
                    return RusotoError::Service(ListGroupMembershipsError::ResourceUnavailable(
                        err.msg,
                    ))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(ListGroupMembershipsError::Throttling(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ListGroupMembershipsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListGroupMembershipsError::AccessDenied(ref cause) => write!(f, "{}", cause),
            ListGroupMembershipsError::InternalFailure(ref cause) => write!(f, "{}", cause),
            ListGroupMembershipsError::InvalidNextToken(ref cause) => write!(f, "{}", cause),
            ListGroupMembershipsError::InvalidParameterValue(ref cause) => write!(f, "{}", cause),
            ListGroupMembershipsError::PreconditionNotMet(ref cause) => write!(f, "{}", cause),
            ListGroupMembershipsError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            ListGroupMembershipsError::ResourceUnavailable(ref cause) => write!(f, "{}", cause),
            ListGroupMembershipsError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListGroupMembershipsError {}
/// Errors returned by ListGroups
#[derive(Debug, PartialEq)]
pub enum ListGroupsError {
    /// <p>You don't have access to this item. The provided credentials couldn't be validated. You might not be authorized to carry out the request. Make sure that your account is authorized to use the Amazon QuickSight service, that your policies have the correct permissions, and that you are using the correct access keys.</p>
    AccessDenied(String),
    /// <p>An internal failure occurred.</p>
    InternalFailure(String),
    /// <p>The <code>NextToken</code> value isn't valid.</p>
    InvalidNextToken(String),
    /// <p>One or more parameters has a value that isn't valid.</p>
    InvalidParameterValue(String),
    /// <p>One or more preconditions aren't met.</p>
    PreconditionNotMet(String),
    /// <p>One or more resources can't be found.</p>
    ResourceNotFound(String),
    /// <p>This resource is currently unavailable.</p>
    ResourceUnavailable(String),
    /// <p>Access is throttled.</p>
    Throttling(String),
}

impl ListGroupsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListGroupsError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(ListGroupsError::AccessDenied(err.msg))
                }
                "InternalFailureException" => {
                    return RusotoError::Service(ListGroupsError::InternalFailure(err.msg))
                }
                "InvalidNextTokenException" => {
                    return RusotoError::Service(ListGroupsError::InvalidNextToken(err.msg))
                }
                "InvalidParameterValueException" => {
                    return RusotoError::Service(ListGroupsError::InvalidParameterValue(err.msg))
                }
                "PreconditionNotMetException" => {
                    return RusotoError::Service(ListGroupsError::PreconditionNotMet(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(ListGroupsError::ResourceNotFound(err.msg))
                }
                "ResourceUnavailableException" => {
                    return RusotoError::Service(ListGroupsError::ResourceUnavailable(err.msg))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(ListGroupsError::Throttling(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ListGroupsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListGroupsError::AccessDenied(ref cause) => write!(f, "{}", cause),
            ListGroupsError::InternalFailure(ref cause) => write!(f, "{}", cause),
            ListGroupsError::InvalidNextToken(ref cause) => write!(f, "{}", cause),
            ListGroupsError::InvalidParameterValue(ref cause) => write!(f, "{}", cause),
            ListGroupsError::PreconditionNotMet(ref cause) => write!(f, "{}", cause),
            ListGroupsError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            ListGroupsError::ResourceUnavailable(ref cause) => write!(f, "{}", cause),
            ListGroupsError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListGroupsError {}
/// Errors returned by ListIAMPolicyAssignments
#[derive(Debug, PartialEq)]
pub enum ListIAMPolicyAssignmentsError {
    /// <p>You don't have access to this item. The provided credentials couldn't be validated. You might not be authorized to carry out the request. Make sure that your account is authorized to use the Amazon QuickSight service, that your policies have the correct permissions, and that you are using the correct access keys.</p>
    AccessDenied(String),
    /// <p>An internal failure occurred.</p>
    InternalFailure(String),
    /// <p>The <code>NextToken</code> value isn't valid.</p>
    InvalidNextToken(String),
    /// <p>One or more parameters has a value that isn't valid.</p>
    InvalidParameterValue(String),
    /// <p>One or more resources can't be found.</p>
    ResourceNotFound(String),
    /// <p>Access is throttled.</p>
    Throttling(String),
}

impl ListIAMPolicyAssignmentsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListIAMPolicyAssignmentsError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(ListIAMPolicyAssignmentsError::AccessDenied(
                        err.msg,
                    ))
                }
                "InternalFailureException" => {
                    return RusotoError::Service(ListIAMPolicyAssignmentsError::InternalFailure(
                        err.msg,
                    ))
                }
                "InvalidNextTokenException" => {
                    return RusotoError::Service(ListIAMPolicyAssignmentsError::InvalidNextToken(
                        err.msg,
                    ))
                }
                "InvalidParameterValueException" => {
                    return RusotoError::Service(
                        ListIAMPolicyAssignmentsError::InvalidParameterValue(err.msg),
                    )
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(ListIAMPolicyAssignmentsError::ResourceNotFound(
                        err.msg,
                    ))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(ListIAMPolicyAssignmentsError::Throttling(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ListIAMPolicyAssignmentsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListIAMPolicyAssignmentsError::AccessDenied(ref cause) => write!(f, "{}", cause),
            ListIAMPolicyAssignmentsError::InternalFailure(ref cause) => write!(f, "{}", cause),
            ListIAMPolicyAssignmentsError::InvalidNextToken(ref cause) => write!(f, "{}", cause),
            ListIAMPolicyAssignmentsError::InvalidParameterValue(ref cause) => {
                write!(f, "{}", cause)
            }
            ListIAMPolicyAssignmentsError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            ListIAMPolicyAssignmentsError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListIAMPolicyAssignmentsError {}
/// Errors returned by ListIAMPolicyAssignmentsForUser
#[derive(Debug, PartialEq)]
pub enum ListIAMPolicyAssignmentsForUserError {
    /// <p>You don't have access to this item. The provided credentials couldn't be validated. You might not be authorized to carry out the request. Make sure that your account is authorized to use the Amazon QuickSight service, that your policies have the correct permissions, and that you are using the correct access keys.</p>
    AccessDenied(String),
    /// <p>A resource is already in a state that indicates an action is happening that must complete before a new update can be applied.</p>
    ConcurrentUpdating(String),
    /// <p>An internal failure occurred.</p>
    InternalFailure(String),
    /// <p>One or more parameters has a value that isn't valid.</p>
    InvalidParameterValue(String),
    /// <p>The resource specified already exists. </p>
    ResourceExists(String),
    /// <p>One or more resources can't be found.</p>
    ResourceNotFound(String),
    /// <p>Access is throttled.</p>
    Throttling(String),
}

impl ListIAMPolicyAssignmentsForUserError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<ListIAMPolicyAssignmentsForUserError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(
                        ListIAMPolicyAssignmentsForUserError::AccessDenied(err.msg),
                    )
                }
                "ConcurrentUpdatingException" => {
                    return RusotoError::Service(
                        ListIAMPolicyAssignmentsForUserError::ConcurrentUpdating(err.msg),
                    )
                }
                "InternalFailureException" => {
                    return RusotoError::Service(
                        ListIAMPolicyAssignmentsForUserError::InternalFailure(err.msg),
                    )
                }
                "InvalidParameterValueException" => {
                    return RusotoError::Service(
                        ListIAMPolicyAssignmentsForUserError::InvalidParameterValue(err.msg),
                    )
                }
                "ResourceExistsException" => {
                    return RusotoError::Service(
                        ListIAMPolicyAssignmentsForUserError::ResourceExists(err.msg),
                    )
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(
                        ListIAMPolicyAssignmentsForUserError::ResourceNotFound(err.msg),
                    )
                }
                "ThrottlingException" => {
                    return RusotoError::Service(ListIAMPolicyAssignmentsForUserError::Throttling(
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
impl fmt::Display for ListIAMPolicyAssignmentsForUserError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListIAMPolicyAssignmentsForUserError::AccessDenied(ref cause) => write!(f, "{}", cause),
            ListIAMPolicyAssignmentsForUserError::ConcurrentUpdating(ref cause) => {
                write!(f, "{}", cause)
            }
            ListIAMPolicyAssignmentsForUserError::InternalFailure(ref cause) => {
                write!(f, "{}", cause)
            }
            ListIAMPolicyAssignmentsForUserError::InvalidParameterValue(ref cause) => {
                write!(f, "{}", cause)
            }
            ListIAMPolicyAssignmentsForUserError::ResourceExists(ref cause) => {
                write!(f, "{}", cause)
            }
            ListIAMPolicyAssignmentsForUserError::ResourceNotFound(ref cause) => {
                write!(f, "{}", cause)
            }
            ListIAMPolicyAssignmentsForUserError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListIAMPolicyAssignmentsForUserError {}
/// Errors returned by ListIngestions
#[derive(Debug, PartialEq)]
pub enum ListIngestionsError {
    /// <p>You don't have access to this item. The provided credentials couldn't be validated. You might not be authorized to carry out the request. Make sure that your account is authorized to use the Amazon QuickSight service, that your policies have the correct permissions, and that you are using the correct access keys.</p>
    AccessDenied(String),
    /// <p>An internal failure occurred.</p>
    InternalFailure(String),
    /// <p>The <code>NextToken</code> value isn't valid.</p>
    InvalidNextToken(String),
    /// <p>One or more parameters has a value that isn't valid.</p>
    InvalidParameterValue(String),
    /// <p>The resource specified already exists. </p>
    ResourceExists(String),
    /// <p>One or more resources can't be found.</p>
    ResourceNotFound(String),
    /// <p>Access is throttled.</p>
    Throttling(String),
}

impl ListIngestionsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListIngestionsError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(ListIngestionsError::AccessDenied(err.msg))
                }
                "InternalFailureException" => {
                    return RusotoError::Service(ListIngestionsError::InternalFailure(err.msg))
                }
                "InvalidNextTokenException" => {
                    return RusotoError::Service(ListIngestionsError::InvalidNextToken(err.msg))
                }
                "InvalidParameterValueException" => {
                    return RusotoError::Service(ListIngestionsError::InvalidParameterValue(
                        err.msg,
                    ))
                }
                "ResourceExistsException" => {
                    return RusotoError::Service(ListIngestionsError::ResourceExists(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(ListIngestionsError::ResourceNotFound(err.msg))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(ListIngestionsError::Throttling(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ListIngestionsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListIngestionsError::AccessDenied(ref cause) => write!(f, "{}", cause),
            ListIngestionsError::InternalFailure(ref cause) => write!(f, "{}", cause),
            ListIngestionsError::InvalidNextToken(ref cause) => write!(f, "{}", cause),
            ListIngestionsError::InvalidParameterValue(ref cause) => write!(f, "{}", cause),
            ListIngestionsError::ResourceExists(ref cause) => write!(f, "{}", cause),
            ListIngestionsError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            ListIngestionsError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListIngestionsError {}
/// Errors returned by ListTagsForResource
#[derive(Debug, PartialEq)]
pub enum ListTagsForResourceError {
    /// <p>You don't have access to this item. The provided credentials couldn't be validated. You might not be authorized to carry out the request. Make sure that your account is authorized to use the Amazon QuickSight service, that your policies have the correct permissions, and that you are using the correct access keys.</p>
    AccessDenied(String),
    /// <p>An internal failure occurred.</p>
    InternalFailure(String),
    /// <p>One or more parameters has a value that isn't valid.</p>
    InvalidParameterValue(String),
    /// <p>One or more resources can't be found.</p>
    ResourceNotFound(String),
    /// <p>Access is throttled.</p>
    Throttling(String),
}

impl ListTagsForResourceError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListTagsForResourceError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(ListTagsForResourceError::AccessDenied(err.msg))
                }
                "InternalFailureException" => {
                    return RusotoError::Service(ListTagsForResourceError::InternalFailure(err.msg))
                }
                "InvalidParameterValueException" => {
                    return RusotoError::Service(ListTagsForResourceError::InvalidParameterValue(
                        err.msg,
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(ListTagsForResourceError::ResourceNotFound(
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
            ListTagsForResourceError::InternalFailure(ref cause) => write!(f, "{}", cause),
            ListTagsForResourceError::InvalidParameterValue(ref cause) => write!(f, "{}", cause),
            ListTagsForResourceError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            ListTagsForResourceError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListTagsForResourceError {}
/// Errors returned by ListTemplateAliases
#[derive(Debug, PartialEq)]
pub enum ListTemplateAliasesError {
    /// <p>An internal failure occurred.</p>
    InternalFailure(String),
    /// <p>One or more resources can't be found.</p>
    ResourceNotFound(String),
    /// <p>Access is throttled.</p>
    Throttling(String),
    /// <p>This error indicates that you are calling an operation on an Amazon QuickSight subscription where the edition doesn't include support for that operation. Amazon QuickSight currently has Standard Edition and Enterprise Edition. Not every operation and capability is available in every edition.</p>
    UnsupportedUserEdition(String),
}

impl ListTemplateAliasesError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListTemplateAliasesError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InternalFailureException" => {
                    return RusotoError::Service(ListTemplateAliasesError::InternalFailure(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(ListTemplateAliasesError::ResourceNotFound(
                        err.msg,
                    ))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(ListTemplateAliasesError::Throttling(err.msg))
                }
                "UnsupportedUserEditionException" => {
                    return RusotoError::Service(ListTemplateAliasesError::UnsupportedUserEdition(
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
impl fmt::Display for ListTemplateAliasesError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListTemplateAliasesError::InternalFailure(ref cause) => write!(f, "{}", cause),
            ListTemplateAliasesError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            ListTemplateAliasesError::Throttling(ref cause) => write!(f, "{}", cause),
            ListTemplateAliasesError::UnsupportedUserEdition(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListTemplateAliasesError {}
/// Errors returned by ListTemplateVersions
#[derive(Debug, PartialEq)]
pub enum ListTemplateVersionsError {
    /// <p>An internal failure occurred.</p>
    InternalFailure(String),
    /// <p>The <code>NextToken</code> value isn't valid.</p>
    InvalidNextToken(String),
    /// <p>One or more parameters has a value that isn't valid.</p>
    InvalidParameterValue(String),
    /// <p>One or more resources can't be found.</p>
    ResourceNotFound(String),
    /// <p>Access is throttled.</p>
    Throttling(String),
    /// <p>This error indicates that you are calling an operation on an Amazon QuickSight subscription where the edition doesn't include support for that operation. Amazon QuickSight currently has Standard Edition and Enterprise Edition. Not every operation and capability is available in every edition.</p>
    UnsupportedUserEdition(String),
}

impl ListTemplateVersionsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListTemplateVersionsError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InternalFailureException" => {
                    return RusotoError::Service(ListTemplateVersionsError::InternalFailure(
                        err.msg,
                    ))
                }
                "InvalidNextTokenException" => {
                    return RusotoError::Service(ListTemplateVersionsError::InvalidNextToken(
                        err.msg,
                    ))
                }
                "InvalidParameterValueException" => {
                    return RusotoError::Service(ListTemplateVersionsError::InvalidParameterValue(
                        err.msg,
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(ListTemplateVersionsError::ResourceNotFound(
                        err.msg,
                    ))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(ListTemplateVersionsError::Throttling(err.msg))
                }
                "UnsupportedUserEditionException" => {
                    return RusotoError::Service(ListTemplateVersionsError::UnsupportedUserEdition(
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
impl fmt::Display for ListTemplateVersionsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListTemplateVersionsError::InternalFailure(ref cause) => write!(f, "{}", cause),
            ListTemplateVersionsError::InvalidNextToken(ref cause) => write!(f, "{}", cause),
            ListTemplateVersionsError::InvalidParameterValue(ref cause) => write!(f, "{}", cause),
            ListTemplateVersionsError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            ListTemplateVersionsError::Throttling(ref cause) => write!(f, "{}", cause),
            ListTemplateVersionsError::UnsupportedUserEdition(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListTemplateVersionsError {}
/// Errors returned by ListTemplates
#[derive(Debug, PartialEq)]
pub enum ListTemplatesError {
    /// <p>An internal failure occurred.</p>
    InternalFailure(String),
    /// <p>The <code>NextToken</code> value isn't valid.</p>
    InvalidNextToken(String),
    /// <p>One or more parameters has a value that isn't valid.</p>
    InvalidParameterValue(String),
    /// <p>One or more resources can't be found.</p>
    ResourceNotFound(String),
    /// <p>Access is throttled.</p>
    Throttling(String),
    /// <p>This error indicates that you are calling an operation on an Amazon QuickSight subscription where the edition doesn't include support for that operation. Amazon QuickSight currently has Standard Edition and Enterprise Edition. Not every operation and capability is available in every edition.</p>
    UnsupportedUserEdition(String),
}

impl ListTemplatesError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListTemplatesError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InternalFailureException" => {
                    return RusotoError::Service(ListTemplatesError::InternalFailure(err.msg))
                }
                "InvalidNextTokenException" => {
                    return RusotoError::Service(ListTemplatesError::InvalidNextToken(err.msg))
                }
                "InvalidParameterValueException" => {
                    return RusotoError::Service(ListTemplatesError::InvalidParameterValue(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(ListTemplatesError::ResourceNotFound(err.msg))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(ListTemplatesError::Throttling(err.msg))
                }
                "UnsupportedUserEditionException" => {
                    return RusotoError::Service(ListTemplatesError::UnsupportedUserEdition(
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
impl fmt::Display for ListTemplatesError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListTemplatesError::InternalFailure(ref cause) => write!(f, "{}", cause),
            ListTemplatesError::InvalidNextToken(ref cause) => write!(f, "{}", cause),
            ListTemplatesError::InvalidParameterValue(ref cause) => write!(f, "{}", cause),
            ListTemplatesError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            ListTemplatesError::Throttling(ref cause) => write!(f, "{}", cause),
            ListTemplatesError::UnsupportedUserEdition(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListTemplatesError {}
/// Errors returned by ListUserGroups
#[derive(Debug, PartialEq)]
pub enum ListUserGroupsError {
    /// <p>You don't have access to this item. The provided credentials couldn't be validated. You might not be authorized to carry out the request. Make sure that your account is authorized to use the Amazon QuickSight service, that your policies have the correct permissions, and that you are using the correct access keys.</p>
    AccessDenied(String),
    /// <p>An internal failure occurred.</p>
    InternalFailure(String),
    /// <p>One or more parameters has a value that isn't valid.</p>
    InvalidParameterValue(String),
    /// <p>One or more resources can't be found.</p>
    ResourceNotFound(String),
    /// <p>This resource is currently unavailable.</p>
    ResourceUnavailable(String),
    /// <p>Access is throttled.</p>
    Throttling(String),
}

impl ListUserGroupsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListUserGroupsError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(ListUserGroupsError::AccessDenied(err.msg))
                }
                "InternalFailureException" => {
                    return RusotoError::Service(ListUserGroupsError::InternalFailure(err.msg))
                }
                "InvalidParameterValueException" => {
                    return RusotoError::Service(ListUserGroupsError::InvalidParameterValue(
                        err.msg,
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(ListUserGroupsError::ResourceNotFound(err.msg))
                }
                "ResourceUnavailableException" => {
                    return RusotoError::Service(ListUserGroupsError::ResourceUnavailable(err.msg))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(ListUserGroupsError::Throttling(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ListUserGroupsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListUserGroupsError::AccessDenied(ref cause) => write!(f, "{}", cause),
            ListUserGroupsError::InternalFailure(ref cause) => write!(f, "{}", cause),
            ListUserGroupsError::InvalidParameterValue(ref cause) => write!(f, "{}", cause),
            ListUserGroupsError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            ListUserGroupsError::ResourceUnavailable(ref cause) => write!(f, "{}", cause),
            ListUserGroupsError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListUserGroupsError {}
/// Errors returned by ListUsers
#[derive(Debug, PartialEq)]
pub enum ListUsersError {
    /// <p>You don't have access to this item. The provided credentials couldn't be validated. You might not be authorized to carry out the request. Make sure that your account is authorized to use the Amazon QuickSight service, that your policies have the correct permissions, and that you are using the correct access keys.</p>
    AccessDenied(String),
    /// <p>An internal failure occurred.</p>
    InternalFailure(String),
    /// <p>The <code>NextToken</code> value isn't valid.</p>
    InvalidNextToken(String),
    /// <p>One or more parameters has a value that isn't valid.</p>
    InvalidParameterValue(String),
    /// <p>One or more resources can't be found.</p>
    ResourceNotFound(String),
    /// <p>This resource is currently unavailable.</p>
    ResourceUnavailable(String),
    /// <p>Access is throttled.</p>
    Throttling(String),
}

impl ListUsersError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListUsersError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(ListUsersError::AccessDenied(err.msg))
                }
                "InternalFailureException" => {
                    return RusotoError::Service(ListUsersError::InternalFailure(err.msg))
                }
                "InvalidNextTokenException" => {
                    return RusotoError::Service(ListUsersError::InvalidNextToken(err.msg))
                }
                "InvalidParameterValueException" => {
                    return RusotoError::Service(ListUsersError::InvalidParameterValue(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(ListUsersError::ResourceNotFound(err.msg))
                }
                "ResourceUnavailableException" => {
                    return RusotoError::Service(ListUsersError::ResourceUnavailable(err.msg))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(ListUsersError::Throttling(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ListUsersError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListUsersError::AccessDenied(ref cause) => write!(f, "{}", cause),
            ListUsersError::InternalFailure(ref cause) => write!(f, "{}", cause),
            ListUsersError::InvalidNextToken(ref cause) => write!(f, "{}", cause),
            ListUsersError::InvalidParameterValue(ref cause) => write!(f, "{}", cause),
            ListUsersError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            ListUsersError::ResourceUnavailable(ref cause) => write!(f, "{}", cause),
            ListUsersError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListUsersError {}
/// Errors returned by RegisterUser
#[derive(Debug, PartialEq)]
pub enum RegisterUserError {
    /// <p>You don't have access to this item. The provided credentials couldn't be validated. You might not be authorized to carry out the request. Make sure that your account is authorized to use the Amazon QuickSight service, that your policies have the correct permissions, and that you are using the correct access keys.</p>
    AccessDenied(String),
    /// <p>An internal failure occurred.</p>
    InternalFailure(String),
    /// <p>One or more parameters has a value that isn't valid.</p>
    InvalidParameterValue(String),
    /// <p>A limit is exceeded.</p>
    LimitExceeded(String),
    /// <p>One or more preconditions aren't met.</p>
    PreconditionNotMet(String),
    /// <p>The resource specified already exists. </p>
    ResourceExists(String),
    /// <p>One or more resources can't be found.</p>
    ResourceNotFound(String),
    /// <p>This resource is currently unavailable.</p>
    ResourceUnavailable(String),
    /// <p>Access is throttled.</p>
    Throttling(String),
}

impl RegisterUserError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<RegisterUserError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(RegisterUserError::AccessDenied(err.msg))
                }
                "InternalFailureException" => {
                    return RusotoError::Service(RegisterUserError::InternalFailure(err.msg))
                }
                "InvalidParameterValueException" => {
                    return RusotoError::Service(RegisterUserError::InvalidParameterValue(err.msg))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(RegisterUserError::LimitExceeded(err.msg))
                }
                "PreconditionNotMetException" => {
                    return RusotoError::Service(RegisterUserError::PreconditionNotMet(err.msg))
                }
                "ResourceExistsException" => {
                    return RusotoError::Service(RegisterUserError::ResourceExists(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(RegisterUserError::ResourceNotFound(err.msg))
                }
                "ResourceUnavailableException" => {
                    return RusotoError::Service(RegisterUserError::ResourceUnavailable(err.msg))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(RegisterUserError::Throttling(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for RegisterUserError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            RegisterUserError::AccessDenied(ref cause) => write!(f, "{}", cause),
            RegisterUserError::InternalFailure(ref cause) => write!(f, "{}", cause),
            RegisterUserError::InvalidParameterValue(ref cause) => write!(f, "{}", cause),
            RegisterUserError::LimitExceeded(ref cause) => write!(f, "{}", cause),
            RegisterUserError::PreconditionNotMet(ref cause) => write!(f, "{}", cause),
            RegisterUserError::ResourceExists(ref cause) => write!(f, "{}", cause),
            RegisterUserError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            RegisterUserError::ResourceUnavailable(ref cause) => write!(f, "{}", cause),
            RegisterUserError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for RegisterUserError {}
/// Errors returned by SearchDashboards
#[derive(Debug, PartialEq)]
pub enum SearchDashboardsError {
    /// <p>An internal failure occurred.</p>
    InternalFailure(String),
    /// <p>The <code>NextToken</code> value isn't valid.</p>
    InvalidNextToken(String),
    /// <p>One or more parameters has a value that isn't valid.</p>
    InvalidParameterValue(String),
    /// <p>One or more resources can't be found.</p>
    ResourceNotFound(String),
    /// <p>Access is throttled.</p>
    Throttling(String),
    /// <p>This error indicates that you are calling an operation on an Amazon QuickSight subscription where the edition doesn't include support for that operation. Amazon QuickSight currently has Standard Edition and Enterprise Edition. Not every operation and capability is available in every edition.</p>
    UnsupportedUserEdition(String),
}

impl SearchDashboardsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<SearchDashboardsError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InternalFailureException" => {
                    return RusotoError::Service(SearchDashboardsError::InternalFailure(err.msg))
                }
                "InvalidNextTokenException" => {
                    return RusotoError::Service(SearchDashboardsError::InvalidNextToken(err.msg))
                }
                "InvalidParameterValueException" => {
                    return RusotoError::Service(SearchDashboardsError::InvalidParameterValue(
                        err.msg,
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(SearchDashboardsError::ResourceNotFound(err.msg))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(SearchDashboardsError::Throttling(err.msg))
                }
                "UnsupportedUserEditionException" => {
                    return RusotoError::Service(SearchDashboardsError::UnsupportedUserEdition(
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
impl fmt::Display for SearchDashboardsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            SearchDashboardsError::InternalFailure(ref cause) => write!(f, "{}", cause),
            SearchDashboardsError::InvalidNextToken(ref cause) => write!(f, "{}", cause),
            SearchDashboardsError::InvalidParameterValue(ref cause) => write!(f, "{}", cause),
            SearchDashboardsError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            SearchDashboardsError::Throttling(ref cause) => write!(f, "{}", cause),
            SearchDashboardsError::UnsupportedUserEdition(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for SearchDashboardsError {}
/// Errors returned by TagResource
#[derive(Debug, PartialEq)]
pub enum TagResourceError {
    /// <p>You don't have access to this item. The provided credentials couldn't be validated. You might not be authorized to carry out the request. Make sure that your account is authorized to use the Amazon QuickSight service, that your policies have the correct permissions, and that you are using the correct access keys.</p>
    AccessDenied(String),
    /// <p>An internal failure occurred.</p>
    InternalFailure(String),
    /// <p>One or more parameters has a value that isn't valid.</p>
    InvalidParameterValue(String),
    /// <p>A limit is exceeded.</p>
    LimitExceeded(String),
    /// <p>One or more resources can't be found.</p>
    ResourceNotFound(String),
    /// <p>Access is throttled.</p>
    Throttling(String),
}

impl TagResourceError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<TagResourceError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(TagResourceError::AccessDenied(err.msg))
                }
                "InternalFailureException" => {
                    return RusotoError::Service(TagResourceError::InternalFailure(err.msg))
                }
                "InvalidParameterValueException" => {
                    return RusotoError::Service(TagResourceError::InvalidParameterValue(err.msg))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(TagResourceError::LimitExceeded(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(TagResourceError::ResourceNotFound(err.msg))
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
            TagResourceError::InternalFailure(ref cause) => write!(f, "{}", cause),
            TagResourceError::InvalidParameterValue(ref cause) => write!(f, "{}", cause),
            TagResourceError::LimitExceeded(ref cause) => write!(f, "{}", cause),
            TagResourceError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            TagResourceError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for TagResourceError {}
/// Errors returned by UntagResource
#[derive(Debug, PartialEq)]
pub enum UntagResourceError {
    /// <p>You don't have access to this item. The provided credentials couldn't be validated. You might not be authorized to carry out the request. Make sure that your account is authorized to use the Amazon QuickSight service, that your policies have the correct permissions, and that you are using the correct access keys.</p>
    AccessDenied(String),
    /// <p>An internal failure occurred.</p>
    InternalFailure(String),
    /// <p>One or more parameters has a value that isn't valid.</p>
    InvalidParameterValue(String),
    /// <p>One or more resources can't be found.</p>
    ResourceNotFound(String),
    /// <p>Access is throttled.</p>
    Throttling(String),
}

impl UntagResourceError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UntagResourceError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(UntagResourceError::AccessDenied(err.msg))
                }
                "InternalFailureException" => {
                    return RusotoError::Service(UntagResourceError::InternalFailure(err.msg))
                }
                "InvalidParameterValueException" => {
                    return RusotoError::Service(UntagResourceError::InvalidParameterValue(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(UntagResourceError::ResourceNotFound(err.msg))
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
            UntagResourceError::InternalFailure(ref cause) => write!(f, "{}", cause),
            UntagResourceError::InvalidParameterValue(ref cause) => write!(f, "{}", cause),
            UntagResourceError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            UntagResourceError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for UntagResourceError {}
/// Errors returned by UpdateDashboard
#[derive(Debug, PartialEq)]
pub enum UpdateDashboardError {
    /// <p>Updating or deleting a resource can cause an inconsistent state.</p>
    Conflict(String),
    /// <p>An internal failure occurred.</p>
    InternalFailure(String),
    /// <p>One or more parameters has a value that isn't valid.</p>
    InvalidParameterValue(String),
    /// <p>A limit is exceeded.</p>
    LimitExceeded(String),
    /// <p>One or more resources can't be found.</p>
    ResourceNotFound(String),
    /// <p>Access is throttled.</p>
    Throttling(String),
    /// <p>This error indicates that you are calling an operation on an Amazon QuickSight subscription where the edition doesn't include support for that operation. Amazon QuickSight currently has Standard Edition and Enterprise Edition. Not every operation and capability is available in every edition.</p>
    UnsupportedUserEdition(String),
}

impl UpdateDashboardError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdateDashboardError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "ConflictException" => {
                    return RusotoError::Service(UpdateDashboardError::Conflict(err.msg))
                }
                "InternalFailureException" => {
                    return RusotoError::Service(UpdateDashboardError::InternalFailure(err.msg))
                }
                "InvalidParameterValueException" => {
                    return RusotoError::Service(UpdateDashboardError::InvalidParameterValue(
                        err.msg,
                    ))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(UpdateDashboardError::LimitExceeded(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(UpdateDashboardError::ResourceNotFound(err.msg))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(UpdateDashboardError::Throttling(err.msg))
                }
                "UnsupportedUserEditionException" => {
                    return RusotoError::Service(UpdateDashboardError::UnsupportedUserEdition(
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
impl fmt::Display for UpdateDashboardError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UpdateDashboardError::Conflict(ref cause) => write!(f, "{}", cause),
            UpdateDashboardError::InternalFailure(ref cause) => write!(f, "{}", cause),
            UpdateDashboardError::InvalidParameterValue(ref cause) => write!(f, "{}", cause),
            UpdateDashboardError::LimitExceeded(ref cause) => write!(f, "{}", cause),
            UpdateDashboardError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            UpdateDashboardError::Throttling(ref cause) => write!(f, "{}", cause),
            UpdateDashboardError::UnsupportedUserEdition(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for UpdateDashboardError {}
/// Errors returned by UpdateDashboardPermissions
#[derive(Debug, PartialEq)]
pub enum UpdateDashboardPermissionsError {
    /// <p>Updating or deleting a resource can cause an inconsistent state.</p>
    Conflict(String),
    /// <p>An internal failure occurred.</p>
    InternalFailure(String),
    /// <p>One or more parameters has a value that isn't valid.</p>
    InvalidParameterValue(String),
    /// <p>One or more resources can't be found.</p>
    ResourceNotFound(String),
    /// <p>Access is throttled.</p>
    Throttling(String),
    /// <p>This error indicates that you are calling an operation on an Amazon QuickSight subscription where the edition doesn't include support for that operation. Amazon QuickSight currently has Standard Edition and Enterprise Edition. Not every operation and capability is available in every edition.</p>
    UnsupportedUserEdition(String),
}

impl UpdateDashboardPermissionsError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<UpdateDashboardPermissionsError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "ConflictException" => {
                    return RusotoError::Service(UpdateDashboardPermissionsError::Conflict(err.msg))
                }
                "InternalFailureException" => {
                    return RusotoError::Service(UpdateDashboardPermissionsError::InternalFailure(
                        err.msg,
                    ))
                }
                "InvalidParameterValueException" => {
                    return RusotoError::Service(
                        UpdateDashboardPermissionsError::InvalidParameterValue(err.msg),
                    )
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(UpdateDashboardPermissionsError::ResourceNotFound(
                        err.msg,
                    ))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(UpdateDashboardPermissionsError::Throttling(
                        err.msg,
                    ))
                }
                "UnsupportedUserEditionException" => {
                    return RusotoError::Service(
                        UpdateDashboardPermissionsError::UnsupportedUserEdition(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for UpdateDashboardPermissionsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UpdateDashboardPermissionsError::Conflict(ref cause) => write!(f, "{}", cause),
            UpdateDashboardPermissionsError::InternalFailure(ref cause) => write!(f, "{}", cause),
            UpdateDashboardPermissionsError::InvalidParameterValue(ref cause) => {
                write!(f, "{}", cause)
            }
            UpdateDashboardPermissionsError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            UpdateDashboardPermissionsError::Throttling(ref cause) => write!(f, "{}", cause),
            UpdateDashboardPermissionsError::UnsupportedUserEdition(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for UpdateDashboardPermissionsError {}
/// Errors returned by UpdateDashboardPublishedVersion
#[derive(Debug, PartialEq)]
pub enum UpdateDashboardPublishedVersionError {
    /// <p>Updating or deleting a resource can cause an inconsistent state.</p>
    Conflict(String),
    /// <p>An internal failure occurred.</p>
    InternalFailure(String),
    /// <p>One or more parameters has a value that isn't valid.</p>
    InvalidParameterValue(String),
    /// <p>One or more resources can't be found.</p>
    ResourceNotFound(String),
    /// <p>Access is throttled.</p>
    Throttling(String),
    /// <p>This error indicates that you are calling an operation on an Amazon QuickSight subscription where the edition doesn't include support for that operation. Amazon QuickSight currently has Standard Edition and Enterprise Edition. Not every operation and capability is available in every edition.</p>
    UnsupportedUserEdition(String),
}

impl UpdateDashboardPublishedVersionError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<UpdateDashboardPublishedVersionError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "ConflictException" => {
                    return RusotoError::Service(UpdateDashboardPublishedVersionError::Conflict(
                        err.msg,
                    ))
                }
                "InternalFailureException" => {
                    return RusotoError::Service(
                        UpdateDashboardPublishedVersionError::InternalFailure(err.msg),
                    )
                }
                "InvalidParameterValueException" => {
                    return RusotoError::Service(
                        UpdateDashboardPublishedVersionError::InvalidParameterValue(err.msg),
                    )
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(
                        UpdateDashboardPublishedVersionError::ResourceNotFound(err.msg),
                    )
                }
                "ThrottlingException" => {
                    return RusotoError::Service(UpdateDashboardPublishedVersionError::Throttling(
                        err.msg,
                    ))
                }
                "UnsupportedUserEditionException" => {
                    return RusotoError::Service(
                        UpdateDashboardPublishedVersionError::UnsupportedUserEdition(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for UpdateDashboardPublishedVersionError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UpdateDashboardPublishedVersionError::Conflict(ref cause) => write!(f, "{}", cause),
            UpdateDashboardPublishedVersionError::InternalFailure(ref cause) => {
                write!(f, "{}", cause)
            }
            UpdateDashboardPublishedVersionError::InvalidParameterValue(ref cause) => {
                write!(f, "{}", cause)
            }
            UpdateDashboardPublishedVersionError::ResourceNotFound(ref cause) => {
                write!(f, "{}", cause)
            }
            UpdateDashboardPublishedVersionError::Throttling(ref cause) => write!(f, "{}", cause),
            UpdateDashboardPublishedVersionError::UnsupportedUserEdition(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for UpdateDashboardPublishedVersionError {}
/// Errors returned by UpdateDataSet
#[derive(Debug, PartialEq)]
pub enum UpdateDataSetError {
    /// <p>You don't have access to this item. The provided credentials couldn't be validated. You might not be authorized to carry out the request. Make sure that your account is authorized to use the Amazon QuickSight service, that your policies have the correct permissions, and that you are using the correct access keys.</p>
    AccessDenied(String),
    /// <p>Updating or deleting a resource can cause an inconsistent state.</p>
    Conflict(String),
    /// <p>An internal failure occurred.</p>
    InternalFailure(String),
    /// <p>One or more parameters has a value that isn't valid.</p>
    InvalidParameterValue(String),
    /// <p>A limit is exceeded.</p>
    LimitExceeded(String),
    /// <p>One or more resources can't be found.</p>
    ResourceNotFound(String),
    /// <p>Access is throttled.</p>
    Throttling(String),
    /// <p>This error indicates that you are calling an operation on an Amazon QuickSight subscription where the edition doesn't include support for that operation. Amazon QuickSight currently has Standard Edition and Enterprise Edition. Not every operation and capability is available in every edition.</p>
    UnsupportedUserEdition(String),
}

impl UpdateDataSetError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdateDataSetError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(UpdateDataSetError::AccessDenied(err.msg))
                }
                "ConflictException" => {
                    return RusotoError::Service(UpdateDataSetError::Conflict(err.msg))
                }
                "InternalFailureException" => {
                    return RusotoError::Service(UpdateDataSetError::InternalFailure(err.msg))
                }
                "InvalidParameterValueException" => {
                    return RusotoError::Service(UpdateDataSetError::InvalidParameterValue(err.msg))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(UpdateDataSetError::LimitExceeded(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(UpdateDataSetError::ResourceNotFound(err.msg))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(UpdateDataSetError::Throttling(err.msg))
                }
                "UnsupportedUserEditionException" => {
                    return RusotoError::Service(UpdateDataSetError::UnsupportedUserEdition(
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
impl fmt::Display for UpdateDataSetError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UpdateDataSetError::AccessDenied(ref cause) => write!(f, "{}", cause),
            UpdateDataSetError::Conflict(ref cause) => write!(f, "{}", cause),
            UpdateDataSetError::InternalFailure(ref cause) => write!(f, "{}", cause),
            UpdateDataSetError::InvalidParameterValue(ref cause) => write!(f, "{}", cause),
            UpdateDataSetError::LimitExceeded(ref cause) => write!(f, "{}", cause),
            UpdateDataSetError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            UpdateDataSetError::Throttling(ref cause) => write!(f, "{}", cause),
            UpdateDataSetError::UnsupportedUserEdition(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for UpdateDataSetError {}
/// Errors returned by UpdateDataSetPermissions
#[derive(Debug, PartialEq)]
pub enum UpdateDataSetPermissionsError {
    /// <p>You don't have access to this item. The provided credentials couldn't be validated. You might not be authorized to carry out the request. Make sure that your account is authorized to use the Amazon QuickSight service, that your policies have the correct permissions, and that you are using the correct access keys.</p>
    AccessDenied(String),
    /// <p>Updating or deleting a resource can cause an inconsistent state.</p>
    Conflict(String),
    /// <p>An internal failure occurred.</p>
    InternalFailure(String),
    /// <p>One or more parameters has a value that isn't valid.</p>
    InvalidParameterValue(String),
    /// <p>One or more resources can't be found.</p>
    ResourceNotFound(String),
    /// <p>Access is throttled.</p>
    Throttling(String),
}

impl UpdateDataSetPermissionsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdateDataSetPermissionsError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(UpdateDataSetPermissionsError::AccessDenied(
                        err.msg,
                    ))
                }
                "ConflictException" => {
                    return RusotoError::Service(UpdateDataSetPermissionsError::Conflict(err.msg))
                }
                "InternalFailureException" => {
                    return RusotoError::Service(UpdateDataSetPermissionsError::InternalFailure(
                        err.msg,
                    ))
                }
                "InvalidParameterValueException" => {
                    return RusotoError::Service(
                        UpdateDataSetPermissionsError::InvalidParameterValue(err.msg),
                    )
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(UpdateDataSetPermissionsError::ResourceNotFound(
                        err.msg,
                    ))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(UpdateDataSetPermissionsError::Throttling(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for UpdateDataSetPermissionsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UpdateDataSetPermissionsError::AccessDenied(ref cause) => write!(f, "{}", cause),
            UpdateDataSetPermissionsError::Conflict(ref cause) => write!(f, "{}", cause),
            UpdateDataSetPermissionsError::InternalFailure(ref cause) => write!(f, "{}", cause),
            UpdateDataSetPermissionsError::InvalidParameterValue(ref cause) => {
                write!(f, "{}", cause)
            }
            UpdateDataSetPermissionsError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            UpdateDataSetPermissionsError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for UpdateDataSetPermissionsError {}
/// Errors returned by UpdateDataSource
#[derive(Debug, PartialEq)]
pub enum UpdateDataSourceError {
    /// <p>You don't have access to this item. The provided credentials couldn't be validated. You might not be authorized to carry out the request. Make sure that your account is authorized to use the Amazon QuickSight service, that your policies have the correct permissions, and that you are using the correct access keys.</p>
    AccessDenied(String),
    /// <p>Updating or deleting a resource can cause an inconsistent state.</p>
    Conflict(String),
    /// <p>An internal failure occurred.</p>
    InternalFailure(String),
    /// <p>One or more parameters has a value that isn't valid.</p>
    InvalidParameterValue(String),
    /// <p>One or more resources can't be found.</p>
    ResourceNotFound(String),
    /// <p>Access is throttled.</p>
    Throttling(String),
}

impl UpdateDataSourceError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdateDataSourceError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(UpdateDataSourceError::AccessDenied(err.msg))
                }
                "ConflictException" => {
                    return RusotoError::Service(UpdateDataSourceError::Conflict(err.msg))
                }
                "InternalFailureException" => {
                    return RusotoError::Service(UpdateDataSourceError::InternalFailure(err.msg))
                }
                "InvalidParameterValueException" => {
                    return RusotoError::Service(UpdateDataSourceError::InvalidParameterValue(
                        err.msg,
                    ))
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
            UpdateDataSourceError::InternalFailure(ref cause) => write!(f, "{}", cause),
            UpdateDataSourceError::InvalidParameterValue(ref cause) => write!(f, "{}", cause),
            UpdateDataSourceError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            UpdateDataSourceError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for UpdateDataSourceError {}
/// Errors returned by UpdateDataSourcePermissions
#[derive(Debug, PartialEq)]
pub enum UpdateDataSourcePermissionsError {
    /// <p>You don't have access to this item. The provided credentials couldn't be validated. You might not be authorized to carry out the request. Make sure that your account is authorized to use the Amazon QuickSight service, that your policies have the correct permissions, and that you are using the correct access keys.</p>
    AccessDenied(String),
    /// <p>Updating or deleting a resource can cause an inconsistent state.</p>
    Conflict(String),
    /// <p>An internal failure occurred.</p>
    InternalFailure(String),
    /// <p>One or more parameters has a value that isn't valid.</p>
    InvalidParameterValue(String),
    /// <p>One or more resources can't be found.</p>
    ResourceNotFound(String),
    /// <p>Access is throttled.</p>
    Throttling(String),
}

impl UpdateDataSourcePermissionsError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<UpdateDataSourcePermissionsError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(UpdateDataSourcePermissionsError::AccessDenied(
                        err.msg,
                    ))
                }
                "ConflictException" => {
                    return RusotoError::Service(UpdateDataSourcePermissionsError::Conflict(
                        err.msg,
                    ))
                }
                "InternalFailureException" => {
                    return RusotoError::Service(UpdateDataSourcePermissionsError::InternalFailure(
                        err.msg,
                    ))
                }
                "InvalidParameterValueException" => {
                    return RusotoError::Service(
                        UpdateDataSourcePermissionsError::InvalidParameterValue(err.msg),
                    )
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(
                        UpdateDataSourcePermissionsError::ResourceNotFound(err.msg),
                    )
                }
                "ThrottlingException" => {
                    return RusotoError::Service(UpdateDataSourcePermissionsError::Throttling(
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
impl fmt::Display for UpdateDataSourcePermissionsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UpdateDataSourcePermissionsError::AccessDenied(ref cause) => write!(f, "{}", cause),
            UpdateDataSourcePermissionsError::Conflict(ref cause) => write!(f, "{}", cause),
            UpdateDataSourcePermissionsError::InternalFailure(ref cause) => write!(f, "{}", cause),
            UpdateDataSourcePermissionsError::InvalidParameterValue(ref cause) => {
                write!(f, "{}", cause)
            }
            UpdateDataSourcePermissionsError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            UpdateDataSourcePermissionsError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for UpdateDataSourcePermissionsError {}
/// Errors returned by UpdateGroup
#[derive(Debug, PartialEq)]
pub enum UpdateGroupError {
    /// <p>You don't have access to this item. The provided credentials couldn't be validated. You might not be authorized to carry out the request. Make sure that your account is authorized to use the Amazon QuickSight service, that your policies have the correct permissions, and that you are using the correct access keys.</p>
    AccessDenied(String),
    /// <p>An internal failure occurred.</p>
    InternalFailure(String),
    /// <p>One or more parameters has a value that isn't valid.</p>
    InvalidParameterValue(String),
    /// <p>One or more preconditions aren't met.</p>
    PreconditionNotMet(String),
    /// <p>One or more resources can't be found.</p>
    ResourceNotFound(String),
    /// <p>This resource is currently unavailable.</p>
    ResourceUnavailable(String),
    /// <p>Access is throttled.</p>
    Throttling(String),
}

impl UpdateGroupError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdateGroupError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(UpdateGroupError::AccessDenied(err.msg))
                }
                "InternalFailureException" => {
                    return RusotoError::Service(UpdateGroupError::InternalFailure(err.msg))
                }
                "InvalidParameterValueException" => {
                    return RusotoError::Service(UpdateGroupError::InvalidParameterValue(err.msg))
                }
                "PreconditionNotMetException" => {
                    return RusotoError::Service(UpdateGroupError::PreconditionNotMet(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(UpdateGroupError::ResourceNotFound(err.msg))
                }
                "ResourceUnavailableException" => {
                    return RusotoError::Service(UpdateGroupError::ResourceUnavailable(err.msg))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(UpdateGroupError::Throttling(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for UpdateGroupError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UpdateGroupError::AccessDenied(ref cause) => write!(f, "{}", cause),
            UpdateGroupError::InternalFailure(ref cause) => write!(f, "{}", cause),
            UpdateGroupError::InvalidParameterValue(ref cause) => write!(f, "{}", cause),
            UpdateGroupError::PreconditionNotMet(ref cause) => write!(f, "{}", cause),
            UpdateGroupError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            UpdateGroupError::ResourceUnavailable(ref cause) => write!(f, "{}", cause),
            UpdateGroupError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for UpdateGroupError {}
/// Errors returned by UpdateIAMPolicyAssignment
#[derive(Debug, PartialEq)]
pub enum UpdateIAMPolicyAssignmentError {
    /// <p>You don't have access to this item. The provided credentials couldn't be validated. You might not be authorized to carry out the request. Make sure that your account is authorized to use the Amazon QuickSight service, that your policies have the correct permissions, and that you are using the correct access keys.</p>
    AccessDenied(String),
    /// <p>A resource is already in a state that indicates an action is happening that must complete before a new update can be applied.</p>
    ConcurrentUpdating(String),
    /// <p>An internal failure occurred.</p>
    InternalFailure(String),
    /// <p>One or more parameters has a value that isn't valid.</p>
    InvalidParameterValue(String),
    /// <p>The resource specified already exists. </p>
    ResourceExists(String),
    /// <p>One or more resources can't be found.</p>
    ResourceNotFound(String),
    /// <p>Access is throttled.</p>
    Throttling(String),
}

impl UpdateIAMPolicyAssignmentError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdateIAMPolicyAssignmentError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(UpdateIAMPolicyAssignmentError::AccessDenied(
                        err.msg,
                    ))
                }
                "ConcurrentUpdatingException" => {
                    return RusotoError::Service(
                        UpdateIAMPolicyAssignmentError::ConcurrentUpdating(err.msg),
                    )
                }
                "InternalFailureException" => {
                    return RusotoError::Service(UpdateIAMPolicyAssignmentError::InternalFailure(
                        err.msg,
                    ))
                }
                "InvalidParameterValueException" => {
                    return RusotoError::Service(
                        UpdateIAMPolicyAssignmentError::InvalidParameterValue(err.msg),
                    )
                }
                "ResourceExistsException" => {
                    return RusotoError::Service(UpdateIAMPolicyAssignmentError::ResourceExists(
                        err.msg,
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(UpdateIAMPolicyAssignmentError::ResourceNotFound(
                        err.msg,
                    ))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(UpdateIAMPolicyAssignmentError::Throttling(
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
impl fmt::Display for UpdateIAMPolicyAssignmentError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UpdateIAMPolicyAssignmentError::AccessDenied(ref cause) => write!(f, "{}", cause),
            UpdateIAMPolicyAssignmentError::ConcurrentUpdating(ref cause) => write!(f, "{}", cause),
            UpdateIAMPolicyAssignmentError::InternalFailure(ref cause) => write!(f, "{}", cause),
            UpdateIAMPolicyAssignmentError::InvalidParameterValue(ref cause) => {
                write!(f, "{}", cause)
            }
            UpdateIAMPolicyAssignmentError::ResourceExists(ref cause) => write!(f, "{}", cause),
            UpdateIAMPolicyAssignmentError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            UpdateIAMPolicyAssignmentError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for UpdateIAMPolicyAssignmentError {}
/// Errors returned by UpdateTemplate
#[derive(Debug, PartialEq)]
pub enum UpdateTemplateError {
    /// <p>Updating or deleting a resource can cause an inconsistent state.</p>
    Conflict(String),
    /// <p>An internal failure occurred.</p>
    InternalFailure(String),
    /// <p>One or more parameters has a value that isn't valid.</p>
    InvalidParameterValue(String),
    /// <p>A limit is exceeded.</p>
    LimitExceeded(String),
    /// <p>The resource specified already exists. </p>
    ResourceExists(String),
    /// <p>One or more resources can't be found.</p>
    ResourceNotFound(String),
    /// <p>Access is throttled.</p>
    Throttling(String),
    /// <p>This error indicates that you are calling an operation on an Amazon QuickSight subscription where the edition doesn't include support for that operation. Amazon QuickSight currently has Standard Edition and Enterprise Edition. Not every operation and capability is available in every edition.</p>
    UnsupportedUserEdition(String),
}

impl UpdateTemplateError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdateTemplateError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "ConflictException" => {
                    return RusotoError::Service(UpdateTemplateError::Conflict(err.msg))
                }
                "InternalFailureException" => {
                    return RusotoError::Service(UpdateTemplateError::InternalFailure(err.msg))
                }
                "InvalidParameterValueException" => {
                    return RusotoError::Service(UpdateTemplateError::InvalidParameterValue(
                        err.msg,
                    ))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(UpdateTemplateError::LimitExceeded(err.msg))
                }
                "ResourceExistsException" => {
                    return RusotoError::Service(UpdateTemplateError::ResourceExists(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(UpdateTemplateError::ResourceNotFound(err.msg))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(UpdateTemplateError::Throttling(err.msg))
                }
                "UnsupportedUserEditionException" => {
                    return RusotoError::Service(UpdateTemplateError::UnsupportedUserEdition(
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
impl fmt::Display for UpdateTemplateError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UpdateTemplateError::Conflict(ref cause) => write!(f, "{}", cause),
            UpdateTemplateError::InternalFailure(ref cause) => write!(f, "{}", cause),
            UpdateTemplateError::InvalidParameterValue(ref cause) => write!(f, "{}", cause),
            UpdateTemplateError::LimitExceeded(ref cause) => write!(f, "{}", cause),
            UpdateTemplateError::ResourceExists(ref cause) => write!(f, "{}", cause),
            UpdateTemplateError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            UpdateTemplateError::Throttling(ref cause) => write!(f, "{}", cause),
            UpdateTemplateError::UnsupportedUserEdition(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for UpdateTemplateError {}
/// Errors returned by UpdateTemplateAlias
#[derive(Debug, PartialEq)]
pub enum UpdateTemplateAliasError {
    /// <p>Updating or deleting a resource can cause an inconsistent state.</p>
    Conflict(String),
    /// <p>An internal failure occurred.</p>
    InternalFailure(String),
    /// <p>One or more resources can't be found.</p>
    ResourceNotFound(String),
    /// <p>Access is throttled.</p>
    Throttling(String),
    /// <p>This error indicates that you are calling an operation on an Amazon QuickSight subscription where the edition doesn't include support for that operation. Amazon QuickSight currently has Standard Edition and Enterprise Edition. Not every operation and capability is available in every edition.</p>
    UnsupportedUserEdition(String),
}

impl UpdateTemplateAliasError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdateTemplateAliasError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "ConflictException" => {
                    return RusotoError::Service(UpdateTemplateAliasError::Conflict(err.msg))
                }
                "InternalFailureException" => {
                    return RusotoError::Service(UpdateTemplateAliasError::InternalFailure(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(UpdateTemplateAliasError::ResourceNotFound(
                        err.msg,
                    ))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(UpdateTemplateAliasError::Throttling(err.msg))
                }
                "UnsupportedUserEditionException" => {
                    return RusotoError::Service(UpdateTemplateAliasError::UnsupportedUserEdition(
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
impl fmt::Display for UpdateTemplateAliasError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UpdateTemplateAliasError::Conflict(ref cause) => write!(f, "{}", cause),
            UpdateTemplateAliasError::InternalFailure(ref cause) => write!(f, "{}", cause),
            UpdateTemplateAliasError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            UpdateTemplateAliasError::Throttling(ref cause) => write!(f, "{}", cause),
            UpdateTemplateAliasError::UnsupportedUserEdition(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for UpdateTemplateAliasError {}
/// Errors returned by UpdateTemplatePermissions
#[derive(Debug, PartialEq)]
pub enum UpdateTemplatePermissionsError {
    /// <p>Updating or deleting a resource can cause an inconsistent state.</p>
    Conflict(String),
    /// <p>An internal failure occurred.</p>
    InternalFailure(String),
    /// <p>One or more parameters has a value that isn't valid.</p>
    InvalidParameterValue(String),
    /// <p>One or more resources can't be found.</p>
    ResourceNotFound(String),
    /// <p>Access is throttled.</p>
    Throttling(String),
    /// <p>This error indicates that you are calling an operation on an Amazon QuickSight subscription where the edition doesn't include support for that operation. Amazon QuickSight currently has Standard Edition and Enterprise Edition. Not every operation and capability is available in every edition.</p>
    UnsupportedUserEdition(String),
}

impl UpdateTemplatePermissionsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdateTemplatePermissionsError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "ConflictException" => {
                    return RusotoError::Service(UpdateTemplatePermissionsError::Conflict(err.msg))
                }
                "InternalFailureException" => {
                    return RusotoError::Service(UpdateTemplatePermissionsError::InternalFailure(
                        err.msg,
                    ))
                }
                "InvalidParameterValueException" => {
                    return RusotoError::Service(
                        UpdateTemplatePermissionsError::InvalidParameterValue(err.msg),
                    )
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(UpdateTemplatePermissionsError::ResourceNotFound(
                        err.msg,
                    ))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(UpdateTemplatePermissionsError::Throttling(
                        err.msg,
                    ))
                }
                "UnsupportedUserEditionException" => {
                    return RusotoError::Service(
                        UpdateTemplatePermissionsError::UnsupportedUserEdition(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for UpdateTemplatePermissionsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UpdateTemplatePermissionsError::Conflict(ref cause) => write!(f, "{}", cause),
            UpdateTemplatePermissionsError::InternalFailure(ref cause) => write!(f, "{}", cause),
            UpdateTemplatePermissionsError::InvalidParameterValue(ref cause) => {
                write!(f, "{}", cause)
            }
            UpdateTemplatePermissionsError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            UpdateTemplatePermissionsError::Throttling(ref cause) => write!(f, "{}", cause),
            UpdateTemplatePermissionsError::UnsupportedUserEdition(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for UpdateTemplatePermissionsError {}
/// Errors returned by UpdateUser
#[derive(Debug, PartialEq)]
pub enum UpdateUserError {
    /// <p>You don't have access to this item. The provided credentials couldn't be validated. You might not be authorized to carry out the request. Make sure that your account is authorized to use the Amazon QuickSight service, that your policies have the correct permissions, and that you are using the correct access keys.</p>
    AccessDenied(String),
    /// <p>An internal failure occurred.</p>
    InternalFailure(String),
    /// <p>One or more parameters has a value that isn't valid.</p>
    InvalidParameterValue(String),
    /// <p>One or more resources can't be found.</p>
    ResourceNotFound(String),
    /// <p>This resource is currently unavailable.</p>
    ResourceUnavailable(String),
    /// <p>Access is throttled.</p>
    Throttling(String),
}

impl UpdateUserError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdateUserError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(UpdateUserError::AccessDenied(err.msg))
                }
                "InternalFailureException" => {
                    return RusotoError::Service(UpdateUserError::InternalFailure(err.msg))
                }
                "InvalidParameterValueException" => {
                    return RusotoError::Service(UpdateUserError::InvalidParameterValue(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(UpdateUserError::ResourceNotFound(err.msg))
                }
                "ResourceUnavailableException" => {
                    return RusotoError::Service(UpdateUserError::ResourceUnavailable(err.msg))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(UpdateUserError::Throttling(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for UpdateUserError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UpdateUserError::AccessDenied(ref cause) => write!(f, "{}", cause),
            UpdateUserError::InternalFailure(ref cause) => write!(f, "{}", cause),
            UpdateUserError::InvalidParameterValue(ref cause) => write!(f, "{}", cause),
            UpdateUserError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            UpdateUserError::ResourceUnavailable(ref cause) => write!(f, "{}", cause),
            UpdateUserError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for UpdateUserError {}
/// Trait representing the capabilities of the Amazon QuickSight API. Amazon QuickSight clients implement this trait.
#[async_trait]
pub trait Quicksight {
    /// <p>Cancels an ongoing ingestion of data into SPICE.</p>
    async fn cancel_ingestion(
        &self,
        input: CancelIngestionRequest,
    ) -> Result<CancelIngestionResponse, RusotoError<CancelIngestionError>>;

    /// <p>Creates a dashboard from a template. To first create a template, see the CreateTemplate API operation.</p> <p>A dashboard is an entity in QuickSight that identifies QuickSight reports, created from analyses. You can share QuickSight dashboards. With the right permissions, you can create scheduled email reports from them. The <code>CreateDashboard</code>, <code>DescribeDashboard</code>, and <code>ListDashboardsByUser</code> API operations act on the dashboard entity. If you have the correct permissions, you can create a dashboard from a template that exists in a different AWS account.</p>
    async fn create_dashboard(
        &self,
        input: CreateDashboardRequest,
    ) -> Result<CreateDashboardResponse, RusotoError<CreateDashboardError>>;

    /// <p>Creates a dataset.</p>
    async fn create_data_set(
        &self,
        input: CreateDataSetRequest,
    ) -> Result<CreateDataSetResponse, RusotoError<CreateDataSetError>>;

    /// <p>Creates a data source.</p>
    async fn create_data_source(
        &self,
        input: CreateDataSourceRequest,
    ) -> Result<CreateDataSourceResponse, RusotoError<CreateDataSourceError>>;

    /// <p>Creates an Amazon QuickSight group.</p> <p>The permissions resource is <code>arn:aws:quicksight:us-east-1:<i>&lt;relevant-aws-account-id&gt;</i>:group/default/<i>&lt;group-name&gt;</i> </code>.</p> <p>The response is a group object.</p>
    async fn create_group(
        &self,
        input: CreateGroupRequest,
    ) -> Result<CreateGroupResponse, RusotoError<CreateGroupError>>;

    /// <p>Adds an Amazon QuickSight user to an Amazon QuickSight group. </p>
    async fn create_group_membership(
        &self,
        input: CreateGroupMembershipRequest,
    ) -> Result<CreateGroupMembershipResponse, RusotoError<CreateGroupMembershipError>>;

    /// <p>Creates an assignment with one specified IAM policy, identified by its Amazon Resource Name (ARN). This policy will be assigned to specified groups or users of Amazon QuickSight. The users and groups need to be in the same namespace. </p>
    async fn create_iam_policy_assignment(
        &self,
        input: CreateIAMPolicyAssignmentRequest,
    ) -> Result<CreateIAMPolicyAssignmentResponse, RusotoError<CreateIAMPolicyAssignmentError>>;

    /// <p>Creates and starts a new SPICE ingestion on a dataset</p> <p>Any ingestions operating on tagged datasets inherit the same tags automatically for use in access control. For an example, see <a href="https://aws.example.com/premiumsupport/knowledge-center/iam-ec2-resource-tags/">How do I create an IAM policy to control access to Amazon EC2 resources using tags?</a> in the AWS Knowledge Center. Tags are visible on the tagged dataset, but not on the ingestion resource.</p>
    async fn create_ingestion(
        &self,
        input: CreateIngestionRequest,
    ) -> Result<CreateIngestionResponse, RusotoError<CreateIngestionError>>;

    /// <p>Creates a template from an existing QuickSight analysis or template. You can use the resulting template to create a dashboard.</p> <p>A <i>template</i> is an entity in QuickSight that encapsulates the metadata required to create an analysis and that you can use to create s dashboard. A template adds a layer of abstraction by using placeholders to replace the dataset associated with the analysis. You can use templates to create dashboards by replacing dataset placeholders with datasets that follow the same schema that was used to create the source analysis and template.</p>
    async fn create_template(
        &self,
        input: CreateTemplateRequest,
    ) -> Result<CreateTemplateResponse, RusotoError<CreateTemplateError>>;

    /// <p>Creates a template alias for a template.</p>
    async fn create_template_alias(
        &self,
        input: CreateTemplateAliasRequest,
    ) -> Result<CreateTemplateAliasResponse, RusotoError<CreateTemplateAliasError>>;

    /// <p>Deletes a dashboard.</p>
    async fn delete_dashboard(
        &self,
        input: DeleteDashboardRequest,
    ) -> Result<DeleteDashboardResponse, RusotoError<DeleteDashboardError>>;

    /// <p>Deletes a dataset.</p>
    async fn delete_data_set(
        &self,
        input: DeleteDataSetRequest,
    ) -> Result<DeleteDataSetResponse, RusotoError<DeleteDataSetError>>;

    /// <p>Deletes the data source permanently. This action breaks all the datasets that reference the deleted data source.</p>
    async fn delete_data_source(
        &self,
        input: DeleteDataSourceRequest,
    ) -> Result<DeleteDataSourceResponse, RusotoError<DeleteDataSourceError>>;

    /// <p>Removes a user group from Amazon QuickSight. </p>
    async fn delete_group(
        &self,
        input: DeleteGroupRequest,
    ) -> Result<DeleteGroupResponse, RusotoError<DeleteGroupError>>;

    /// <p>Removes a user from a group so that the user is no longer a member of the group.</p>
    async fn delete_group_membership(
        &self,
        input: DeleteGroupMembershipRequest,
    ) -> Result<DeleteGroupMembershipResponse, RusotoError<DeleteGroupMembershipError>>;

    /// <p>Deletes an existing IAM policy assignment.</p>
    async fn delete_iam_policy_assignment(
        &self,
        input: DeleteIAMPolicyAssignmentRequest,
    ) -> Result<DeleteIAMPolicyAssignmentResponse, RusotoError<DeleteIAMPolicyAssignmentError>>;

    /// <p>Deletes a template.</p>
    async fn delete_template(
        &self,
        input: DeleteTemplateRequest,
    ) -> Result<DeleteTemplateResponse, RusotoError<DeleteTemplateError>>;

    /// <p>Deletes the item that the specified template alias points to. If you provide a specific alias, you delete the version of the template that the alias points to.</p>
    async fn delete_template_alias(
        &self,
        input: DeleteTemplateAliasRequest,
    ) -> Result<DeleteTemplateAliasResponse, RusotoError<DeleteTemplateAliasError>>;

    /// <p>Deletes the Amazon QuickSight user that is associated with the identity of the AWS Identity and Access Management (IAM) user or role that's making the call. The IAM user isn't deleted as a result of this call. </p>
    async fn delete_user(
        &self,
        input: DeleteUserRequest,
    ) -> Result<DeleteUserResponse, RusotoError<DeleteUserError>>;

    /// <p>Deletes a user identified by its principal ID. </p>
    async fn delete_user_by_principal_id(
        &self,
        input: DeleteUserByPrincipalIdRequest,
    ) -> Result<DeleteUserByPrincipalIdResponse, RusotoError<DeleteUserByPrincipalIdError>>;

    /// <p>Provides a summary for a dashboard.</p>
    async fn describe_dashboard(
        &self,
        input: DescribeDashboardRequest,
    ) -> Result<DescribeDashboardResponse, RusotoError<DescribeDashboardError>>;

    /// <p>Describes read and write permissions for a dashboard.</p>
    async fn describe_dashboard_permissions(
        &self,
        input: DescribeDashboardPermissionsRequest,
    ) -> Result<DescribeDashboardPermissionsResponse, RusotoError<DescribeDashboardPermissionsError>>;

    /// <p>Describes a dataset. </p>
    async fn describe_data_set(
        &self,
        input: DescribeDataSetRequest,
    ) -> Result<DescribeDataSetResponse, RusotoError<DescribeDataSetError>>;

    /// <p>Describes the permissions on a dataset.</p> <p>The permissions resource is <code>arn:aws:quicksight:region:aws-account-id:dataset/data-set-id</code>.</p>
    async fn describe_data_set_permissions(
        &self,
        input: DescribeDataSetPermissionsRequest,
    ) -> Result<DescribeDataSetPermissionsResponse, RusotoError<DescribeDataSetPermissionsError>>;

    /// <p>Describes a data source.</p>
    async fn describe_data_source(
        &self,
        input: DescribeDataSourceRequest,
    ) -> Result<DescribeDataSourceResponse, RusotoError<DescribeDataSourceError>>;

    /// <p>Describes the resource permissions for a data source.</p>
    async fn describe_data_source_permissions(
        &self,
        input: DescribeDataSourcePermissionsRequest,
    ) -> Result<
        DescribeDataSourcePermissionsResponse,
        RusotoError<DescribeDataSourcePermissionsError>,
    >;

    /// <p>Returns an Amazon QuickSight group's description and Amazon Resource Name (ARN). </p>
    async fn describe_group(
        &self,
        input: DescribeGroupRequest,
    ) -> Result<DescribeGroupResponse, RusotoError<DescribeGroupError>>;

    /// <p>Describes an existing IAM policy assignment, as specified by the assignment name.</p>
    async fn describe_iam_policy_assignment(
        &self,
        input: DescribeIAMPolicyAssignmentRequest,
    ) -> Result<DescribeIAMPolicyAssignmentResponse, RusotoError<DescribeIAMPolicyAssignmentError>>;

    /// <p>Describes a SPICE ingestion.</p>
    async fn describe_ingestion(
        &self,
        input: DescribeIngestionRequest,
    ) -> Result<DescribeIngestionResponse, RusotoError<DescribeIngestionError>>;

    /// <p>Describes a template's metadata.</p>
    async fn describe_template(
        &self,
        input: DescribeTemplateRequest,
    ) -> Result<DescribeTemplateResponse, RusotoError<DescribeTemplateError>>;

    /// <p>Describes the template alias for a template.</p>
    async fn describe_template_alias(
        &self,
        input: DescribeTemplateAliasRequest,
    ) -> Result<DescribeTemplateAliasResponse, RusotoError<DescribeTemplateAliasError>>;

    /// <p>Describes read and write permissions on a template.</p>
    async fn describe_template_permissions(
        &self,
        input: DescribeTemplatePermissionsRequest,
    ) -> Result<DescribeTemplatePermissionsResponse, RusotoError<DescribeTemplatePermissionsError>>;

    /// <p>Returns information about a user, given the user name. </p>
    async fn describe_user(
        &self,
        input: DescribeUserRequest,
    ) -> Result<DescribeUserResponse, RusotoError<DescribeUserError>>;

    /// <p>Generates a server-side embeddable URL and authorization code. For this process to work properly, first configure the dashboards and user permissions. For more information, see <a href="https://docs.aws.amazon.com/quicksight/latest/user/embedding-dashboards.html">Embedding Amazon QuickSight Dashboards</a> in the <i>Amazon QuickSight User Guide</i> or <a href="https://docs.aws.amazon.com/quicksight/latest/APIReference/qs-dev-embedded-dashboards.html">Embedding Amazon QuickSight Dashboards</a> in the <i>Amazon QuickSight API Reference</i>.</p> <p>Currently, you can use <code>GetDashboardEmbedURL</code> only from the server, not from the user’s browser.</p>
    async fn get_dashboard_embed_url(
        &self,
        input: GetDashboardEmbedUrlRequest,
    ) -> Result<GetDashboardEmbedUrlResponse, RusotoError<GetDashboardEmbedUrlError>>;

    /// <p>Lists all the versions of the dashboards in the QuickSight subscription.</p>
    async fn list_dashboard_versions(
        &self,
        input: ListDashboardVersionsRequest,
    ) -> Result<ListDashboardVersionsResponse, RusotoError<ListDashboardVersionsError>>;

    /// <p>Lists dashboards in an AWS account.</p>
    async fn list_dashboards(
        &self,
        input: ListDashboardsRequest,
    ) -> Result<ListDashboardsResponse, RusotoError<ListDashboardsError>>;

    /// <p>Lists all of the datasets belonging to the current AWS account in an AWS Region.</p> <p>The permissions resource is <code>arn:aws:quicksight:region:aws-account-id:dataset/*</code>.</p>
    async fn list_data_sets(
        &self,
        input: ListDataSetsRequest,
    ) -> Result<ListDataSetsResponse, RusotoError<ListDataSetsError>>;

    /// <p>Lists data sources in current AWS Region that belong to this AWS account.</p>
    async fn list_data_sources(
        &self,
        input: ListDataSourcesRequest,
    ) -> Result<ListDataSourcesResponse, RusotoError<ListDataSourcesError>>;

    /// <p>Lists member users in a group.</p>
    async fn list_group_memberships(
        &self,
        input: ListGroupMembershipsRequest,
    ) -> Result<ListGroupMembershipsResponse, RusotoError<ListGroupMembershipsError>>;

    /// <p>Lists all user groups in Amazon QuickSight. </p>
    async fn list_groups(
        &self,
        input: ListGroupsRequest,
    ) -> Result<ListGroupsResponse, RusotoError<ListGroupsError>>;

    /// <p>Lists IAM policy assignments in the current Amazon QuickSight account.</p>
    async fn list_iam_policy_assignments(
        &self,
        input: ListIAMPolicyAssignmentsRequest,
    ) -> Result<ListIAMPolicyAssignmentsResponse, RusotoError<ListIAMPolicyAssignmentsError>>;

    /// <p>Lists all the IAM policy assignments, including the Amazon Resource Names (ARNs) for the IAM policies assigned to the specified user and group or groups that the user belongs to.</p>
    async fn list_iam_policy_assignments_for_user(
        &self,
        input: ListIAMPolicyAssignmentsForUserRequest,
    ) -> Result<
        ListIAMPolicyAssignmentsForUserResponse,
        RusotoError<ListIAMPolicyAssignmentsForUserError>,
    >;

    /// <p>Lists the history of SPICE ingestions for a dataset.</p>
    async fn list_ingestions(
        &self,
        input: ListIngestionsRequest,
    ) -> Result<ListIngestionsResponse, RusotoError<ListIngestionsError>>;

    /// <p>Lists the tags assigned to a resource.</p>
    async fn list_tags_for_resource(
        &self,
        input: ListTagsForResourceRequest,
    ) -> Result<ListTagsForResourceResponse, RusotoError<ListTagsForResourceError>>;

    /// <p>Lists all the aliases of a template.</p>
    async fn list_template_aliases(
        &self,
        input: ListTemplateAliasesRequest,
    ) -> Result<ListTemplateAliasesResponse, RusotoError<ListTemplateAliasesError>>;

    /// <p>Lists all the versions of the templates in the current Amazon QuickSight account.</p>
    async fn list_template_versions(
        &self,
        input: ListTemplateVersionsRequest,
    ) -> Result<ListTemplateVersionsResponse, RusotoError<ListTemplateVersionsError>>;

    /// <p>Lists all the templates in the current Amazon QuickSight account.</p>
    async fn list_templates(
        &self,
        input: ListTemplatesRequest,
    ) -> Result<ListTemplatesResponse, RusotoError<ListTemplatesError>>;

    /// <p>Lists the Amazon QuickSight groups that an Amazon QuickSight user is a member of.</p>
    async fn list_user_groups(
        &self,
        input: ListUserGroupsRequest,
    ) -> Result<ListUserGroupsResponse, RusotoError<ListUserGroupsError>>;

    /// <p>Returns a list of all of the Amazon QuickSight users belonging to this account. </p>
    async fn list_users(
        &self,
        input: ListUsersRequest,
    ) -> Result<ListUsersResponse, RusotoError<ListUsersError>>;

    /// <p>Creates an Amazon QuickSight user, whose identity is associated with the AWS Identity and Access Management (IAM) identity or role specified in the request. </p>
    async fn register_user(
        &self,
        input: RegisterUserRequest,
    ) -> Result<RegisterUserResponse, RusotoError<RegisterUserError>>;

    /// <p>Searchs for dashboards that belong to a user. </p>
    async fn search_dashboards(
        &self,
        input: SearchDashboardsRequest,
    ) -> Result<SearchDashboardsResponse, RusotoError<SearchDashboardsError>>;

    /// <p><p>Assigns one or more tags (key-value pairs) to the specified QuickSight resource. </p> <p>Tags can help you organize and categorize your resources. You can also use them to scope user permissions, by granting a user permission to access or change only resources with certain tag values. You can use the <code>TagResource</code> operation with a resource that already has tags. If you specify a new tag key for the resource, this tag is appended to the list of tags associated with the resource. If you specify a tag key that is already associated with the resource, the new tag value that you specify replaces the previous value for that tag.</p> <p>You can associate as many as 50 tags with a resource. QuickSight supports tagging on data set, data source, dashboard, and template. </p> <p>Tagging for QuickSight works in a similar way to tagging for other AWS services, except for the following:</p> <ul> <li> <p>You can&#39;t use tags to track AWS costs for QuickSight. This restriction is because QuickSight costs are based on users and SPICE capacity, which aren&#39;t taggable resources.</p> </li> <li> <p>QuickSight doesn&#39;t currently support the Tag Editor for AWS Resource Groups.</p> </li> </ul></p>
    async fn tag_resource(
        &self,
        input: TagResourceRequest,
    ) -> Result<TagResourceResponse, RusotoError<TagResourceError>>;

    /// <p>Removes a tag or tags from a resource.</p>
    async fn untag_resource(
        &self,
        input: UntagResourceRequest,
    ) -> Result<UntagResourceResponse, RusotoError<UntagResourceError>>;

    /// <p>Updates a dashboard in an AWS account.</p>
    async fn update_dashboard(
        &self,
        input: UpdateDashboardRequest,
    ) -> Result<UpdateDashboardResponse, RusotoError<UpdateDashboardError>>;

    /// <p>Updates read and write permissions on a dashboard.</p>
    async fn update_dashboard_permissions(
        &self,
        input: UpdateDashboardPermissionsRequest,
    ) -> Result<UpdateDashboardPermissionsResponse, RusotoError<UpdateDashboardPermissionsError>>;

    /// <p>Updates the published version of a dashboard.</p>
    async fn update_dashboard_published_version(
        &self,
        input: UpdateDashboardPublishedVersionRequest,
    ) -> Result<
        UpdateDashboardPublishedVersionResponse,
        RusotoError<UpdateDashboardPublishedVersionError>,
    >;

    /// <p>Updates a dataset.</p>
    async fn update_data_set(
        &self,
        input: UpdateDataSetRequest,
    ) -> Result<UpdateDataSetResponse, RusotoError<UpdateDataSetError>>;

    /// <p>Updates the permissions on a dataset.</p> <p>The permissions resource is <code>arn:aws:quicksight:region:aws-account-id:dataset/data-set-id</code>.</p>
    async fn update_data_set_permissions(
        &self,
        input: UpdateDataSetPermissionsRequest,
    ) -> Result<UpdateDataSetPermissionsResponse, RusotoError<UpdateDataSetPermissionsError>>;

    /// <p>Updates a data source.</p>
    async fn update_data_source(
        &self,
        input: UpdateDataSourceRequest,
    ) -> Result<UpdateDataSourceResponse, RusotoError<UpdateDataSourceError>>;

    /// <p>Updates the permissions to a data source.</p>
    async fn update_data_source_permissions(
        &self,
        input: UpdateDataSourcePermissionsRequest,
    ) -> Result<UpdateDataSourcePermissionsResponse, RusotoError<UpdateDataSourcePermissionsError>>;

    /// <p>Changes a group description. </p>
    async fn update_group(
        &self,
        input: UpdateGroupRequest,
    ) -> Result<UpdateGroupResponse, RusotoError<UpdateGroupError>>;

    /// <p>Updates an existing IAM policy assignment. This operation updates only the optional parameter or parameters that are specified in the request.</p>
    async fn update_iam_policy_assignment(
        &self,
        input: UpdateIAMPolicyAssignmentRequest,
    ) -> Result<UpdateIAMPolicyAssignmentResponse, RusotoError<UpdateIAMPolicyAssignmentError>>;

    /// <p>Updates a template from an existing Amazon QuickSight analysis or another template.</p>
    async fn update_template(
        &self,
        input: UpdateTemplateRequest,
    ) -> Result<UpdateTemplateResponse, RusotoError<UpdateTemplateError>>;

    /// <p>Updates the template alias of a template.</p>
    async fn update_template_alias(
        &self,
        input: UpdateTemplateAliasRequest,
    ) -> Result<UpdateTemplateAliasResponse, RusotoError<UpdateTemplateAliasError>>;

    /// <p>Updates the resource permissions for a template.</p>
    async fn update_template_permissions(
        &self,
        input: UpdateTemplatePermissionsRequest,
    ) -> Result<UpdateTemplatePermissionsResponse, RusotoError<UpdateTemplatePermissionsError>>;

    /// <p>Updates an Amazon QuickSight user.</p>
    async fn update_user(
        &self,
        input: UpdateUserRequest,
    ) -> Result<UpdateUserResponse, RusotoError<UpdateUserError>>;
}
/// A client for the Amazon QuickSight API.
#[derive(Clone)]
pub struct QuicksightClient {
    client: Client,
    region: region::Region,
}

impl QuicksightClient {
    /// Creates a client backed by the default tokio event loop.
    ///
    /// The client will use the default credentials provider and tls client.
    pub fn new(region: region::Region) -> QuicksightClient {
        QuicksightClient {
            client: Client::shared(),
            region,
        }
    }

    pub fn new_with<P, D>(
        request_dispatcher: D,
        credentials_provider: P,
        region: region::Region,
    ) -> QuicksightClient
    where
        P: ProvideAwsCredentials + Send + Sync + 'static,
        D: DispatchSignedRequest + Send + Sync + 'static,
    {
        QuicksightClient {
            client: Client::new_with(credentials_provider, request_dispatcher),
            region,
        }
    }

    pub fn new_with_client(client: Client, region: region::Region) -> QuicksightClient {
        QuicksightClient { client, region }
    }
}

#[async_trait]
impl Quicksight for QuicksightClient {
    /// <p>Cancels an ongoing ingestion of data into SPICE.</p>
    async fn cancel_ingestion(
        &self,
        input: CancelIngestionRequest,
    ) -> Result<CancelIngestionResponse, RusotoError<CancelIngestionError>> {
        let request_uri = format!(
            "/accounts/{aws_account_id}/data-sets/{data_set_id}/ingestions/{ingestion_id}",
            aws_account_id = input.aws_account_id,
            data_set_id = input.data_set_id,
            ingestion_id = input.ingestion_id
        );

        let mut request = SignedRequest::new("DELETE", "quicksight", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let mut result = proto::json::ResponsePayload::new(&response)
                .deserialize::<CancelIngestionResponse, _>()?;

            result.status = Some(response.status.as_u16() as i64);
            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(CancelIngestionError::from_response(response))
        }
    }

    /// <p>Creates a dashboard from a template. To first create a template, see the CreateTemplate API operation.</p> <p>A dashboard is an entity in QuickSight that identifies QuickSight reports, created from analyses. You can share QuickSight dashboards. With the right permissions, you can create scheduled email reports from them. The <code>CreateDashboard</code>, <code>DescribeDashboard</code>, and <code>ListDashboardsByUser</code> API operations act on the dashboard entity. If you have the correct permissions, you can create a dashboard from a template that exists in a different AWS account.</p>
    async fn create_dashboard(
        &self,
        input: CreateDashboardRequest,
    ) -> Result<CreateDashboardResponse, RusotoError<CreateDashboardError>> {
        let request_uri = format!(
            "/accounts/{aws_account_id}/dashboards/{dashboard_id}",
            aws_account_id = input.aws_account_id,
            dashboard_id = input.dashboard_id
        );

        let mut request = SignedRequest::new("POST", "quicksight", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let mut result = proto::json::ResponsePayload::new(&response)
                .deserialize::<CreateDashboardResponse, _>()?;

            result.status = Some(response.status.as_u16() as i64);
            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(CreateDashboardError::from_response(response))
        }
    }

    /// <p>Creates a dataset.</p>
    async fn create_data_set(
        &self,
        input: CreateDataSetRequest,
    ) -> Result<CreateDataSetResponse, RusotoError<CreateDataSetError>> {
        let request_uri = format!(
            "/accounts/{aws_account_id}/data-sets",
            aws_account_id = input.aws_account_id
        );

        let mut request = SignedRequest::new("POST", "quicksight", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let mut result = proto::json::ResponsePayload::new(&response)
                .deserialize::<CreateDataSetResponse, _>()?;

            result.status = Some(response.status.as_u16() as i64);
            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(CreateDataSetError::from_response(response))
        }
    }

    /// <p>Creates a data source.</p>
    async fn create_data_source(
        &self,
        input: CreateDataSourceRequest,
    ) -> Result<CreateDataSourceResponse, RusotoError<CreateDataSourceError>> {
        let request_uri = format!(
            "/accounts/{aws_account_id}/data-sources",
            aws_account_id = input.aws_account_id
        );

        let mut request = SignedRequest::new("POST", "quicksight", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let mut result = proto::json::ResponsePayload::new(&response)
                .deserialize::<CreateDataSourceResponse, _>()?;

            result.status = Some(response.status.as_u16() as i64);
            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(CreateDataSourceError::from_response(response))
        }
    }

    /// <p>Creates an Amazon QuickSight group.</p> <p>The permissions resource is <code>arn:aws:quicksight:us-east-1:<i>&lt;relevant-aws-account-id&gt;</i>:group/default/<i>&lt;group-name&gt;</i> </code>.</p> <p>The response is a group object.</p>
    async fn create_group(
        &self,
        input: CreateGroupRequest,
    ) -> Result<CreateGroupResponse, RusotoError<CreateGroupError>> {
        let request_uri = format!(
            "/accounts/{aws_account_id}/namespaces/{namespace}/groups",
            aws_account_id = input.aws_account_id,
            namespace = input.namespace
        );

        let mut request = SignedRequest::new("POST", "quicksight", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let mut result = proto::json::ResponsePayload::new(&response)
                .deserialize::<CreateGroupResponse, _>()?;

            result.status = Some(response.status.as_u16() as i64);
            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(CreateGroupError::from_response(response))
        }
    }

    /// <p>Adds an Amazon QuickSight user to an Amazon QuickSight group. </p>
    async fn create_group_membership(
        &self,
        input: CreateGroupMembershipRequest,
    ) -> Result<CreateGroupMembershipResponse, RusotoError<CreateGroupMembershipError>> {
        let request_uri = format!("/accounts/{aws_account_id}/namespaces/{namespace}/groups/{group_name}/members/{member_name}", aws_account_id = input.aws_account_id, group_name = input.group_name, member_name = input.member_name, namespace = input.namespace);

        let mut request = SignedRequest::new("PUT", "quicksight", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let mut result = proto::json::ResponsePayload::new(&response)
                .deserialize::<CreateGroupMembershipResponse, _>()?;

            result.status = Some(response.status.as_u16() as i64);
            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(CreateGroupMembershipError::from_response(response))
        }
    }

    /// <p>Creates an assignment with one specified IAM policy, identified by its Amazon Resource Name (ARN). This policy will be assigned to specified groups or users of Amazon QuickSight. The users and groups need to be in the same namespace. </p>
    async fn create_iam_policy_assignment(
        &self,
        input: CreateIAMPolicyAssignmentRequest,
    ) -> Result<CreateIAMPolicyAssignmentResponse, RusotoError<CreateIAMPolicyAssignmentError>>
    {
        let request_uri = format!(
            "/accounts/{aws_account_id}/namespaces/{namespace}/iam-policy-assignments/",
            aws_account_id = input.aws_account_id,
            namespace = input.namespace
        );

        let mut request = SignedRequest::new("POST", "quicksight", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let mut result = proto::json::ResponsePayload::new(&response)
                .deserialize::<CreateIAMPolicyAssignmentResponse, _>()?;

            result.status = Some(response.status.as_u16() as i64);
            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(CreateIAMPolicyAssignmentError::from_response(response))
        }
    }

    /// <p>Creates and starts a new SPICE ingestion on a dataset</p> <p>Any ingestions operating on tagged datasets inherit the same tags automatically for use in access control. For an example, see <a href="https://aws.example.com/premiumsupport/knowledge-center/iam-ec2-resource-tags/">How do I create an IAM policy to control access to Amazon EC2 resources using tags?</a> in the AWS Knowledge Center. Tags are visible on the tagged dataset, but not on the ingestion resource.</p>
    async fn create_ingestion(
        &self,
        input: CreateIngestionRequest,
    ) -> Result<CreateIngestionResponse, RusotoError<CreateIngestionError>> {
        let request_uri = format!(
            "/accounts/{aws_account_id}/data-sets/{data_set_id}/ingestions/{ingestion_id}",
            aws_account_id = input.aws_account_id,
            data_set_id = input.data_set_id,
            ingestion_id = input.ingestion_id
        );

        let mut request = SignedRequest::new("PUT", "quicksight", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let mut result = proto::json::ResponsePayload::new(&response)
                .deserialize::<CreateIngestionResponse, _>()?;

            result.status = Some(response.status.as_u16() as i64);
            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(CreateIngestionError::from_response(response))
        }
    }

    /// <p>Creates a template from an existing QuickSight analysis or template. You can use the resulting template to create a dashboard.</p> <p>A <i>template</i> is an entity in QuickSight that encapsulates the metadata required to create an analysis and that you can use to create s dashboard. A template adds a layer of abstraction by using placeholders to replace the dataset associated with the analysis. You can use templates to create dashboards by replacing dataset placeholders with datasets that follow the same schema that was used to create the source analysis and template.</p>
    async fn create_template(
        &self,
        input: CreateTemplateRequest,
    ) -> Result<CreateTemplateResponse, RusotoError<CreateTemplateError>> {
        let request_uri = format!(
            "/accounts/{aws_account_id}/templates/{template_id}",
            aws_account_id = input.aws_account_id,
            template_id = input.template_id
        );

        let mut request = SignedRequest::new("POST", "quicksight", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let mut result = proto::json::ResponsePayload::new(&response)
                .deserialize::<CreateTemplateResponse, _>()?;

            result.status = Some(response.status.as_u16() as i64);
            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(CreateTemplateError::from_response(response))
        }
    }

    /// <p>Creates a template alias for a template.</p>
    async fn create_template_alias(
        &self,
        input: CreateTemplateAliasRequest,
    ) -> Result<CreateTemplateAliasResponse, RusotoError<CreateTemplateAliasError>> {
        let request_uri = format!(
            "/accounts/{aws_account_id}/templates/{template_id}/aliases/{alias_name}",
            alias_name = input.alias_name,
            aws_account_id = input.aws_account_id,
            template_id = input.template_id
        );

        let mut request = SignedRequest::new("POST", "quicksight", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let mut result = proto::json::ResponsePayload::new(&response)
                .deserialize::<CreateTemplateAliasResponse, _>()?;

            result.status = Some(response.status.as_u16() as i64);
            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(CreateTemplateAliasError::from_response(response))
        }
    }

    /// <p>Deletes a dashboard.</p>
    async fn delete_dashboard(
        &self,
        input: DeleteDashboardRequest,
    ) -> Result<DeleteDashboardResponse, RusotoError<DeleteDashboardError>> {
        let request_uri = format!(
            "/accounts/{aws_account_id}/dashboards/{dashboard_id}",
            aws_account_id = input.aws_account_id,
            dashboard_id = input.dashboard_id
        );

        let mut request = SignedRequest::new("DELETE", "quicksight", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut params = Params::new();
        if let Some(ref x) = input.version_number {
            params.put("version-number", x);
        }
        request.set_params(params);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let mut result = proto::json::ResponsePayload::new(&response)
                .deserialize::<DeleteDashboardResponse, _>()?;

            result.status = Some(response.status.as_u16() as i64);
            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(DeleteDashboardError::from_response(response))
        }
    }

    /// <p>Deletes a dataset.</p>
    async fn delete_data_set(
        &self,
        input: DeleteDataSetRequest,
    ) -> Result<DeleteDataSetResponse, RusotoError<DeleteDataSetError>> {
        let request_uri = format!(
            "/accounts/{aws_account_id}/data-sets/{data_set_id}",
            aws_account_id = input.aws_account_id,
            data_set_id = input.data_set_id
        );

        let mut request = SignedRequest::new("DELETE", "quicksight", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let mut result = proto::json::ResponsePayload::new(&response)
                .deserialize::<DeleteDataSetResponse, _>()?;

            result.status = Some(response.status.as_u16() as i64);
            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(DeleteDataSetError::from_response(response))
        }
    }

    /// <p>Deletes the data source permanently. This action breaks all the datasets that reference the deleted data source.</p>
    async fn delete_data_source(
        &self,
        input: DeleteDataSourceRequest,
    ) -> Result<DeleteDataSourceResponse, RusotoError<DeleteDataSourceError>> {
        let request_uri = format!(
            "/accounts/{aws_account_id}/data-sources/{data_source_id}",
            aws_account_id = input.aws_account_id,
            data_source_id = input.data_source_id
        );

        let mut request = SignedRequest::new("DELETE", "quicksight", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let mut result = proto::json::ResponsePayload::new(&response)
                .deserialize::<DeleteDataSourceResponse, _>()?;

            result.status = Some(response.status.as_u16() as i64);
            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(DeleteDataSourceError::from_response(response))
        }
    }

    /// <p>Removes a user group from Amazon QuickSight. </p>
    async fn delete_group(
        &self,
        input: DeleteGroupRequest,
    ) -> Result<DeleteGroupResponse, RusotoError<DeleteGroupError>> {
        let request_uri = format!(
            "/accounts/{aws_account_id}/namespaces/{namespace}/groups/{group_name}",
            aws_account_id = input.aws_account_id,
            group_name = input.group_name,
            namespace = input.namespace
        );

        let mut request = SignedRequest::new("DELETE", "quicksight", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let mut result = proto::json::ResponsePayload::new(&response)
                .deserialize::<DeleteGroupResponse, _>()?;

            result.status = Some(response.status.as_u16() as i64);
            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(DeleteGroupError::from_response(response))
        }
    }

    /// <p>Removes a user from a group so that the user is no longer a member of the group.</p>
    async fn delete_group_membership(
        &self,
        input: DeleteGroupMembershipRequest,
    ) -> Result<DeleteGroupMembershipResponse, RusotoError<DeleteGroupMembershipError>> {
        let request_uri = format!("/accounts/{aws_account_id}/namespaces/{namespace}/groups/{group_name}/members/{member_name}", aws_account_id = input.aws_account_id, group_name = input.group_name, member_name = input.member_name, namespace = input.namespace);

        let mut request = SignedRequest::new("DELETE", "quicksight", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let mut result = proto::json::ResponsePayload::new(&response)
                .deserialize::<DeleteGroupMembershipResponse, _>()?;

            result.status = Some(response.status.as_u16() as i64);
            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(DeleteGroupMembershipError::from_response(response))
        }
    }

    /// <p>Deletes an existing IAM policy assignment.</p>
    async fn delete_iam_policy_assignment(
        &self,
        input: DeleteIAMPolicyAssignmentRequest,
    ) -> Result<DeleteIAMPolicyAssignmentResponse, RusotoError<DeleteIAMPolicyAssignmentError>>
    {
        let request_uri = format!("/accounts/{aws_account_id}/namespace/{namespace}/iam-policy-assignments/{assignment_name}", assignment_name = input.assignment_name, aws_account_id = input.aws_account_id, namespace = input.namespace);

        let mut request = SignedRequest::new("DELETE", "quicksight", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let mut result = proto::json::ResponsePayload::new(&response)
                .deserialize::<DeleteIAMPolicyAssignmentResponse, _>()?;

            result.status = Some(response.status.as_u16() as i64);
            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(DeleteIAMPolicyAssignmentError::from_response(response))
        }
    }

    /// <p>Deletes a template.</p>
    async fn delete_template(
        &self,
        input: DeleteTemplateRequest,
    ) -> Result<DeleteTemplateResponse, RusotoError<DeleteTemplateError>> {
        let request_uri = format!(
            "/accounts/{aws_account_id}/templates/{template_id}",
            aws_account_id = input.aws_account_id,
            template_id = input.template_id
        );

        let mut request = SignedRequest::new("DELETE", "quicksight", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut params = Params::new();
        if let Some(ref x) = input.version_number {
            params.put("version-number", x);
        }
        request.set_params(params);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let mut result = proto::json::ResponsePayload::new(&response)
                .deserialize::<DeleteTemplateResponse, _>()?;

            result.status = Some(response.status.as_u16() as i64);
            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(DeleteTemplateError::from_response(response))
        }
    }

    /// <p>Deletes the item that the specified template alias points to. If you provide a specific alias, you delete the version of the template that the alias points to.</p>
    async fn delete_template_alias(
        &self,
        input: DeleteTemplateAliasRequest,
    ) -> Result<DeleteTemplateAliasResponse, RusotoError<DeleteTemplateAliasError>> {
        let request_uri = format!(
            "/accounts/{aws_account_id}/templates/{template_id}/aliases/{alias_name}",
            alias_name = input.alias_name,
            aws_account_id = input.aws_account_id,
            template_id = input.template_id
        );

        let mut request = SignedRequest::new("DELETE", "quicksight", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let mut result = proto::json::ResponsePayload::new(&response)
                .deserialize::<DeleteTemplateAliasResponse, _>()?;

            result.status = Some(response.status.as_u16() as i64);
            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(DeleteTemplateAliasError::from_response(response))
        }
    }

    /// <p>Deletes the Amazon QuickSight user that is associated with the identity of the AWS Identity and Access Management (IAM) user or role that's making the call. The IAM user isn't deleted as a result of this call. </p>
    async fn delete_user(
        &self,
        input: DeleteUserRequest,
    ) -> Result<DeleteUserResponse, RusotoError<DeleteUserError>> {
        let request_uri = format!(
            "/accounts/{aws_account_id}/namespaces/{namespace}/users/{user_name}",
            aws_account_id = input.aws_account_id,
            namespace = input.namespace,
            user_name = input.user_name
        );

        let mut request = SignedRequest::new("DELETE", "quicksight", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let mut result = proto::json::ResponsePayload::new(&response)
                .deserialize::<DeleteUserResponse, _>()?;

            result.status = Some(response.status.as_u16() as i64);
            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(DeleteUserError::from_response(response))
        }
    }

    /// <p>Deletes a user identified by its principal ID. </p>
    async fn delete_user_by_principal_id(
        &self,
        input: DeleteUserByPrincipalIdRequest,
    ) -> Result<DeleteUserByPrincipalIdResponse, RusotoError<DeleteUserByPrincipalIdError>> {
        let request_uri = format!(
            "/accounts/{aws_account_id}/namespaces/{namespace}/user-principals/{principal_id}",
            aws_account_id = input.aws_account_id,
            namespace = input.namespace,
            principal_id = input.principal_id
        );

        let mut request = SignedRequest::new("DELETE", "quicksight", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let mut result = proto::json::ResponsePayload::new(&response)
                .deserialize::<DeleteUserByPrincipalIdResponse, _>()?;

            result.status = Some(response.status.as_u16() as i64);
            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(DeleteUserByPrincipalIdError::from_response(response))
        }
    }

    /// <p>Provides a summary for a dashboard.</p>
    async fn describe_dashboard(
        &self,
        input: DescribeDashboardRequest,
    ) -> Result<DescribeDashboardResponse, RusotoError<DescribeDashboardError>> {
        let request_uri = format!(
            "/accounts/{aws_account_id}/dashboards/{dashboard_id}",
            aws_account_id = input.aws_account_id,
            dashboard_id = input.dashboard_id
        );

        let mut request = SignedRequest::new("GET", "quicksight", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut params = Params::new();
        if let Some(ref x) = input.alias_name {
            params.put("alias-name", x);
        }
        if let Some(ref x) = input.version_number {
            params.put("version-number", x);
        }
        request.set_params(params);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let mut result = proto::json::ResponsePayload::new(&response)
                .deserialize::<DescribeDashboardResponse, _>()?;

            result.status = Some(response.status.as_u16() as i64);
            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(DescribeDashboardError::from_response(response))
        }
    }

    /// <p>Describes read and write permissions for a dashboard.</p>
    async fn describe_dashboard_permissions(
        &self,
        input: DescribeDashboardPermissionsRequest,
    ) -> Result<DescribeDashboardPermissionsResponse, RusotoError<DescribeDashboardPermissionsError>>
    {
        let request_uri = format!(
            "/accounts/{aws_account_id}/dashboards/{dashboard_id}/permissions",
            aws_account_id = input.aws_account_id,
            dashboard_id = input.dashboard_id
        );

        let mut request = SignedRequest::new("GET", "quicksight", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let mut result = proto::json::ResponsePayload::new(&response)
                .deserialize::<DescribeDashboardPermissionsResponse, _>()?;

            result.status = Some(response.status.as_u16() as i64);
            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(DescribeDashboardPermissionsError::from_response(response))
        }
    }

    /// <p>Describes a dataset. </p>
    async fn describe_data_set(
        &self,
        input: DescribeDataSetRequest,
    ) -> Result<DescribeDataSetResponse, RusotoError<DescribeDataSetError>> {
        let request_uri = format!(
            "/accounts/{aws_account_id}/data-sets/{data_set_id}",
            aws_account_id = input.aws_account_id,
            data_set_id = input.data_set_id
        );

        let mut request = SignedRequest::new("GET", "quicksight", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let mut result = proto::json::ResponsePayload::new(&response)
                .deserialize::<DescribeDataSetResponse, _>()?;

            result.status = Some(response.status.as_u16() as i64);
            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(DescribeDataSetError::from_response(response))
        }
    }

    /// <p>Describes the permissions on a dataset.</p> <p>The permissions resource is <code>arn:aws:quicksight:region:aws-account-id:dataset/data-set-id</code>.</p>
    async fn describe_data_set_permissions(
        &self,
        input: DescribeDataSetPermissionsRequest,
    ) -> Result<DescribeDataSetPermissionsResponse, RusotoError<DescribeDataSetPermissionsError>>
    {
        let request_uri = format!(
            "/accounts/{aws_account_id}/data-sets/{data_set_id}/permissions",
            aws_account_id = input.aws_account_id,
            data_set_id = input.data_set_id
        );

        let mut request = SignedRequest::new("GET", "quicksight", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let mut result = proto::json::ResponsePayload::new(&response)
                .deserialize::<DescribeDataSetPermissionsResponse, _>()?;

            result.status = Some(response.status.as_u16() as i64);
            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(DescribeDataSetPermissionsError::from_response(response))
        }
    }

    /// <p>Describes a data source.</p>
    async fn describe_data_source(
        &self,
        input: DescribeDataSourceRequest,
    ) -> Result<DescribeDataSourceResponse, RusotoError<DescribeDataSourceError>> {
        let request_uri = format!(
            "/accounts/{aws_account_id}/data-sources/{data_source_id}",
            aws_account_id = input.aws_account_id,
            data_source_id = input.data_source_id
        );

        let mut request = SignedRequest::new("GET", "quicksight", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let mut result = proto::json::ResponsePayload::new(&response)
                .deserialize::<DescribeDataSourceResponse, _>()?;

            result.status = Some(response.status.as_u16() as i64);
            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(DescribeDataSourceError::from_response(response))
        }
    }

    /// <p>Describes the resource permissions for a data source.</p>
    async fn describe_data_source_permissions(
        &self,
        input: DescribeDataSourcePermissionsRequest,
    ) -> Result<
        DescribeDataSourcePermissionsResponse,
        RusotoError<DescribeDataSourcePermissionsError>,
    > {
        let request_uri = format!(
            "/accounts/{aws_account_id}/data-sources/{data_source_id}/permissions",
            aws_account_id = input.aws_account_id,
            data_source_id = input.data_source_id
        );

        let mut request = SignedRequest::new("GET", "quicksight", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let mut result = proto::json::ResponsePayload::new(&response)
                .deserialize::<DescribeDataSourcePermissionsResponse, _>()?;

            result.status = Some(response.status.as_u16() as i64);
            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(DescribeDataSourcePermissionsError::from_response(response))
        }
    }

    /// <p>Returns an Amazon QuickSight group's description and Amazon Resource Name (ARN). </p>
    async fn describe_group(
        &self,
        input: DescribeGroupRequest,
    ) -> Result<DescribeGroupResponse, RusotoError<DescribeGroupError>> {
        let request_uri = format!(
            "/accounts/{aws_account_id}/namespaces/{namespace}/groups/{group_name}",
            aws_account_id = input.aws_account_id,
            group_name = input.group_name,
            namespace = input.namespace
        );

        let mut request = SignedRequest::new("GET", "quicksight", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let mut result = proto::json::ResponsePayload::new(&response)
                .deserialize::<DescribeGroupResponse, _>()?;

            result.status = Some(response.status.as_u16() as i64);
            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(DescribeGroupError::from_response(response))
        }
    }

    /// <p>Describes an existing IAM policy assignment, as specified by the assignment name.</p>
    async fn describe_iam_policy_assignment(
        &self,
        input: DescribeIAMPolicyAssignmentRequest,
    ) -> Result<DescribeIAMPolicyAssignmentResponse, RusotoError<DescribeIAMPolicyAssignmentError>>
    {
        let request_uri = format!("/accounts/{aws_account_id}/namespaces/{namespace}/iam-policy-assignments/{assignment_name}", assignment_name = input.assignment_name, aws_account_id = input.aws_account_id, namespace = input.namespace);

        let mut request = SignedRequest::new("GET", "quicksight", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let mut result = proto::json::ResponsePayload::new(&response)
                .deserialize::<DescribeIAMPolicyAssignmentResponse, _>()?;

            result.status = Some(response.status.as_u16() as i64);
            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(DescribeIAMPolicyAssignmentError::from_response(response))
        }
    }

    /// <p>Describes a SPICE ingestion.</p>
    async fn describe_ingestion(
        &self,
        input: DescribeIngestionRequest,
    ) -> Result<DescribeIngestionResponse, RusotoError<DescribeIngestionError>> {
        let request_uri = format!(
            "/accounts/{aws_account_id}/data-sets/{data_set_id}/ingestions/{ingestion_id}",
            aws_account_id = input.aws_account_id,
            data_set_id = input.data_set_id,
            ingestion_id = input.ingestion_id
        );

        let mut request = SignedRequest::new("GET", "quicksight", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let mut result = proto::json::ResponsePayload::new(&response)
                .deserialize::<DescribeIngestionResponse, _>()?;

            result.status = Some(response.status.as_u16() as i64);
            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(DescribeIngestionError::from_response(response))
        }
    }

    /// <p>Describes a template's metadata.</p>
    async fn describe_template(
        &self,
        input: DescribeTemplateRequest,
    ) -> Result<DescribeTemplateResponse, RusotoError<DescribeTemplateError>> {
        let request_uri = format!(
            "/accounts/{aws_account_id}/templates/{template_id}",
            aws_account_id = input.aws_account_id,
            template_id = input.template_id
        );

        let mut request = SignedRequest::new("GET", "quicksight", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut params = Params::new();
        if let Some(ref x) = input.alias_name {
            params.put("alias-name", x);
        }
        if let Some(ref x) = input.version_number {
            params.put("version-number", x);
        }
        request.set_params(params);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let mut result = proto::json::ResponsePayload::new(&response)
                .deserialize::<DescribeTemplateResponse, _>()?;

            result.status = Some(response.status.as_u16() as i64);
            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(DescribeTemplateError::from_response(response))
        }
    }

    /// <p>Describes the template alias for a template.</p>
    async fn describe_template_alias(
        &self,
        input: DescribeTemplateAliasRequest,
    ) -> Result<DescribeTemplateAliasResponse, RusotoError<DescribeTemplateAliasError>> {
        let request_uri = format!(
            "/accounts/{aws_account_id}/templates/{template_id}/aliases/{alias_name}",
            alias_name = input.alias_name,
            aws_account_id = input.aws_account_id,
            template_id = input.template_id
        );

        let mut request = SignedRequest::new("GET", "quicksight", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let mut result = proto::json::ResponsePayload::new(&response)
                .deserialize::<DescribeTemplateAliasResponse, _>()?;

            result.status = Some(response.status.as_u16() as i64);
            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(DescribeTemplateAliasError::from_response(response))
        }
    }

    /// <p>Describes read and write permissions on a template.</p>
    async fn describe_template_permissions(
        &self,
        input: DescribeTemplatePermissionsRequest,
    ) -> Result<DescribeTemplatePermissionsResponse, RusotoError<DescribeTemplatePermissionsError>>
    {
        let request_uri = format!(
            "/accounts/{aws_account_id}/templates/{template_id}/permissions",
            aws_account_id = input.aws_account_id,
            template_id = input.template_id
        );

        let mut request = SignedRequest::new("GET", "quicksight", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let mut result = proto::json::ResponsePayload::new(&response)
                .deserialize::<DescribeTemplatePermissionsResponse, _>()?;

            result.status = Some(response.status.as_u16() as i64);
            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(DescribeTemplatePermissionsError::from_response(response))
        }
    }

    /// <p>Returns information about a user, given the user name. </p>
    async fn describe_user(
        &self,
        input: DescribeUserRequest,
    ) -> Result<DescribeUserResponse, RusotoError<DescribeUserError>> {
        let request_uri = format!(
            "/accounts/{aws_account_id}/namespaces/{namespace}/users/{user_name}",
            aws_account_id = input.aws_account_id,
            namespace = input.namespace,
            user_name = input.user_name
        );

        let mut request = SignedRequest::new("GET", "quicksight", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let mut result = proto::json::ResponsePayload::new(&response)
                .deserialize::<DescribeUserResponse, _>()?;

            result.status = Some(response.status.as_u16() as i64);
            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(DescribeUserError::from_response(response))
        }
    }

    /// <p>Generates a server-side embeddable URL and authorization code. For this process to work properly, first configure the dashboards and user permissions. For more information, see <a href="https://docs.aws.amazon.com/quicksight/latest/user/embedding-dashboards.html">Embedding Amazon QuickSight Dashboards</a> in the <i>Amazon QuickSight User Guide</i> or <a href="https://docs.aws.amazon.com/quicksight/latest/APIReference/qs-dev-embedded-dashboards.html">Embedding Amazon QuickSight Dashboards</a> in the <i>Amazon QuickSight API Reference</i>.</p> <p>Currently, you can use <code>GetDashboardEmbedURL</code> only from the server, not from the user’s browser.</p>
    async fn get_dashboard_embed_url(
        &self,
        input: GetDashboardEmbedUrlRequest,
    ) -> Result<GetDashboardEmbedUrlResponse, RusotoError<GetDashboardEmbedUrlError>> {
        let request_uri = format!(
            "/accounts/{aws_account_id}/dashboards/{dashboard_id}/embed-url",
            aws_account_id = input.aws_account_id,
            dashboard_id = input.dashboard_id
        );

        let mut request = SignedRequest::new("GET", "quicksight", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut params = Params::new();
        params.put("creds-type", &input.identity_type);
        if let Some(ref x) = input.reset_disabled {
            params.put("reset-disabled", x);
        }
        if let Some(ref x) = input.session_lifetime_in_minutes {
            params.put("session-lifetime", x);
        }
        if let Some(ref x) = input.undo_redo_disabled {
            params.put("undo-redo-disabled", x);
        }
        if let Some(ref x) = input.user_arn {
            params.put("user-arn", x);
        }
        request.set_params(params);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let mut result = proto::json::ResponsePayload::new(&response)
                .deserialize::<GetDashboardEmbedUrlResponse, _>()?;

            result.status = Some(response.status.as_u16() as i64);
            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(GetDashboardEmbedUrlError::from_response(response))
        }
    }

    /// <p>Lists all the versions of the dashboards in the QuickSight subscription.</p>
    async fn list_dashboard_versions(
        &self,
        input: ListDashboardVersionsRequest,
    ) -> Result<ListDashboardVersionsResponse, RusotoError<ListDashboardVersionsError>> {
        let request_uri = format!(
            "/accounts/{aws_account_id}/dashboards/{dashboard_id}/versions",
            aws_account_id = input.aws_account_id,
            dashboard_id = input.dashboard_id
        );

        let mut request = SignedRequest::new("GET", "quicksight", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut params = Params::new();
        if let Some(ref x) = input.max_results {
            params.put("max-results", x);
        }
        if let Some(ref x) = input.next_token {
            params.put("next-token", x);
        }
        request.set_params(params);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let mut result = proto::json::ResponsePayload::new(&response)
                .deserialize::<ListDashboardVersionsResponse, _>()?;

            result.status = Some(response.status.as_u16() as i64);
            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(ListDashboardVersionsError::from_response(response))
        }
    }

    /// <p>Lists dashboards in an AWS account.</p>
    async fn list_dashboards(
        &self,
        input: ListDashboardsRequest,
    ) -> Result<ListDashboardsResponse, RusotoError<ListDashboardsError>> {
        let request_uri = format!(
            "/accounts/{aws_account_id}/dashboards",
            aws_account_id = input.aws_account_id
        );

        let mut request = SignedRequest::new("GET", "quicksight", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut params = Params::new();
        if let Some(ref x) = input.max_results {
            params.put("max-results", x);
        }
        if let Some(ref x) = input.next_token {
            params.put("next-token", x);
        }
        request.set_params(params);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let mut result = proto::json::ResponsePayload::new(&response)
                .deserialize::<ListDashboardsResponse, _>()?;

            result.status = Some(response.status.as_u16() as i64);
            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(ListDashboardsError::from_response(response))
        }
    }

    /// <p>Lists all of the datasets belonging to the current AWS account in an AWS Region.</p> <p>The permissions resource is <code>arn:aws:quicksight:region:aws-account-id:dataset/*</code>.</p>
    async fn list_data_sets(
        &self,
        input: ListDataSetsRequest,
    ) -> Result<ListDataSetsResponse, RusotoError<ListDataSetsError>> {
        let request_uri = format!(
            "/accounts/{aws_account_id}/data-sets",
            aws_account_id = input.aws_account_id
        );

        let mut request = SignedRequest::new("GET", "quicksight", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut params = Params::new();
        if let Some(ref x) = input.max_results {
            params.put("max-results", x);
        }
        if let Some(ref x) = input.next_token {
            params.put("next-token", x);
        }
        request.set_params(params);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let mut result = proto::json::ResponsePayload::new(&response)
                .deserialize::<ListDataSetsResponse, _>()?;

            result.status = Some(response.status.as_u16() as i64);
            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(ListDataSetsError::from_response(response))
        }
    }

    /// <p>Lists data sources in current AWS Region that belong to this AWS account.</p>
    async fn list_data_sources(
        &self,
        input: ListDataSourcesRequest,
    ) -> Result<ListDataSourcesResponse, RusotoError<ListDataSourcesError>> {
        let request_uri = format!(
            "/accounts/{aws_account_id}/data-sources",
            aws_account_id = input.aws_account_id
        );

        let mut request = SignedRequest::new("GET", "quicksight", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut params = Params::new();
        if let Some(ref x) = input.max_results {
            params.put("max-results", x);
        }
        if let Some(ref x) = input.next_token {
            params.put("next-token", x);
        }
        request.set_params(params);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let mut result = proto::json::ResponsePayload::new(&response)
                .deserialize::<ListDataSourcesResponse, _>()?;

            result.status = Some(response.status.as_u16() as i64);
            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(ListDataSourcesError::from_response(response))
        }
    }

    /// <p>Lists member users in a group.</p>
    async fn list_group_memberships(
        &self,
        input: ListGroupMembershipsRequest,
    ) -> Result<ListGroupMembershipsResponse, RusotoError<ListGroupMembershipsError>> {
        let request_uri = format!(
            "/accounts/{aws_account_id}/namespaces/{namespace}/groups/{group_name}/members",
            aws_account_id = input.aws_account_id,
            group_name = input.group_name,
            namespace = input.namespace
        );

        let mut request = SignedRequest::new("GET", "quicksight", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut params = Params::new();
        if let Some(ref x) = input.max_results {
            params.put("max-results", x);
        }
        if let Some(ref x) = input.next_token {
            params.put("next-token", x);
        }
        request.set_params(params);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let mut result = proto::json::ResponsePayload::new(&response)
                .deserialize::<ListGroupMembershipsResponse, _>()?;

            result.status = Some(response.status.as_u16() as i64);
            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(ListGroupMembershipsError::from_response(response))
        }
    }

    /// <p>Lists all user groups in Amazon QuickSight. </p>
    async fn list_groups(
        &self,
        input: ListGroupsRequest,
    ) -> Result<ListGroupsResponse, RusotoError<ListGroupsError>> {
        let request_uri = format!(
            "/accounts/{aws_account_id}/namespaces/{namespace}/groups",
            aws_account_id = input.aws_account_id,
            namespace = input.namespace
        );

        let mut request = SignedRequest::new("GET", "quicksight", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut params = Params::new();
        if let Some(ref x) = input.max_results {
            params.put("max-results", x);
        }
        if let Some(ref x) = input.next_token {
            params.put("next-token", x);
        }
        request.set_params(params);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let mut result = proto::json::ResponsePayload::new(&response)
                .deserialize::<ListGroupsResponse, _>()?;

            result.status = Some(response.status.as_u16() as i64);
            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(ListGroupsError::from_response(response))
        }
    }

    /// <p>Lists IAM policy assignments in the current Amazon QuickSight account.</p>
    async fn list_iam_policy_assignments(
        &self,
        input: ListIAMPolicyAssignmentsRequest,
    ) -> Result<ListIAMPolicyAssignmentsResponse, RusotoError<ListIAMPolicyAssignmentsError>> {
        let request_uri = format!(
            "/accounts/{aws_account_id}/namespaces/{namespace}/iam-policy-assignments",
            aws_account_id = input.aws_account_id,
            namespace = input.namespace
        );

        let mut request = SignedRequest::new("GET", "quicksight", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut params = Params::new();
        if let Some(ref x) = input.max_results {
            params.put("max-results", x);
        }
        if let Some(ref x) = input.next_token {
            params.put("next-token", x);
        }
        request.set_params(params);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let mut result = proto::json::ResponsePayload::new(&response)
                .deserialize::<ListIAMPolicyAssignmentsResponse, _>()?;

            result.status = Some(response.status.as_u16() as i64);
            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(ListIAMPolicyAssignmentsError::from_response(response))
        }
    }

    /// <p>Lists all the IAM policy assignments, including the Amazon Resource Names (ARNs) for the IAM policies assigned to the specified user and group or groups that the user belongs to.</p>
    async fn list_iam_policy_assignments_for_user(
        &self,
        input: ListIAMPolicyAssignmentsForUserRequest,
    ) -> Result<
        ListIAMPolicyAssignmentsForUserResponse,
        RusotoError<ListIAMPolicyAssignmentsForUserError>,
    > {
        let request_uri = format!("/accounts/{aws_account_id}/namespaces/{namespace}/users/{user_name}/iam-policy-assignments", aws_account_id = input.aws_account_id, namespace = input.namespace, user_name = input.user_name);

        let mut request = SignedRequest::new("GET", "quicksight", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut params = Params::new();
        if let Some(ref x) = input.max_results {
            params.put("max-results", x);
        }
        if let Some(ref x) = input.next_token {
            params.put("next-token", x);
        }
        request.set_params(params);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let mut result = proto::json::ResponsePayload::new(&response)
                .deserialize::<ListIAMPolicyAssignmentsForUserResponse, _>()?;

            result.status = Some(response.status.as_u16() as i64);
            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(ListIAMPolicyAssignmentsForUserError::from_response(
                response,
            ))
        }
    }

    /// <p>Lists the history of SPICE ingestions for a dataset.</p>
    async fn list_ingestions(
        &self,
        input: ListIngestionsRequest,
    ) -> Result<ListIngestionsResponse, RusotoError<ListIngestionsError>> {
        let request_uri = format!(
            "/accounts/{aws_account_id}/data-sets/{data_set_id}/ingestions",
            aws_account_id = input.aws_account_id,
            data_set_id = input.data_set_id
        );

        let mut request = SignedRequest::new("GET", "quicksight", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut params = Params::new();
        if let Some(ref x) = input.max_results {
            params.put("max-results", x);
        }
        if let Some(ref x) = input.next_token {
            params.put("next-token", x);
        }
        request.set_params(params);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let mut result = proto::json::ResponsePayload::new(&response)
                .deserialize::<ListIngestionsResponse, _>()?;

            result.status = Some(response.status.as_u16() as i64);
            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(ListIngestionsError::from_response(response))
        }
    }

    /// <p>Lists the tags assigned to a resource.</p>
    async fn list_tags_for_resource(
        &self,
        input: ListTagsForResourceRequest,
    ) -> Result<ListTagsForResourceResponse, RusotoError<ListTagsForResourceError>> {
        let request_uri = format!(
            "/resources/{resource_arn}/tags",
            resource_arn = input.resource_arn
        );

        let mut request = SignedRequest::new("GET", "quicksight", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let mut result = proto::json::ResponsePayload::new(&response)
                .deserialize::<ListTagsForResourceResponse, _>()?;

            result.status = Some(response.status.as_u16() as i64);
            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(ListTagsForResourceError::from_response(response))
        }
    }

    /// <p>Lists all the aliases of a template.</p>
    async fn list_template_aliases(
        &self,
        input: ListTemplateAliasesRequest,
    ) -> Result<ListTemplateAliasesResponse, RusotoError<ListTemplateAliasesError>> {
        let request_uri = format!(
            "/accounts/{aws_account_id}/templates/{template_id}/aliases",
            aws_account_id = input.aws_account_id,
            template_id = input.template_id
        );

        let mut request = SignedRequest::new("GET", "quicksight", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut params = Params::new();
        if let Some(ref x) = input.max_results {
            params.put("max-result", x);
        }
        if let Some(ref x) = input.next_token {
            params.put("next-token", x);
        }
        request.set_params(params);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let mut result = proto::json::ResponsePayload::new(&response)
                .deserialize::<ListTemplateAliasesResponse, _>()?;

            result.status = Some(response.status.as_u16() as i64);
            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(ListTemplateAliasesError::from_response(response))
        }
    }

    /// <p>Lists all the versions of the templates in the current Amazon QuickSight account.</p>
    async fn list_template_versions(
        &self,
        input: ListTemplateVersionsRequest,
    ) -> Result<ListTemplateVersionsResponse, RusotoError<ListTemplateVersionsError>> {
        let request_uri = format!(
            "/accounts/{aws_account_id}/templates/{template_id}/versions",
            aws_account_id = input.aws_account_id,
            template_id = input.template_id
        );

        let mut request = SignedRequest::new("GET", "quicksight", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut params = Params::new();
        if let Some(ref x) = input.max_results {
            params.put("max-results", x);
        }
        if let Some(ref x) = input.next_token {
            params.put("next-token", x);
        }
        request.set_params(params);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let mut result = proto::json::ResponsePayload::new(&response)
                .deserialize::<ListTemplateVersionsResponse, _>()?;

            result.status = Some(response.status.as_u16() as i64);
            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(ListTemplateVersionsError::from_response(response))
        }
    }

    /// <p>Lists all the templates in the current Amazon QuickSight account.</p>
    async fn list_templates(
        &self,
        input: ListTemplatesRequest,
    ) -> Result<ListTemplatesResponse, RusotoError<ListTemplatesError>> {
        let request_uri = format!(
            "/accounts/{aws_account_id}/templates",
            aws_account_id = input.aws_account_id
        );

        let mut request = SignedRequest::new("GET", "quicksight", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut params = Params::new();
        if let Some(ref x) = input.max_results {
            params.put("max-result", x);
        }
        if let Some(ref x) = input.next_token {
            params.put("next-token", x);
        }
        request.set_params(params);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let mut result = proto::json::ResponsePayload::new(&response)
                .deserialize::<ListTemplatesResponse, _>()?;

            result.status = Some(response.status.as_u16() as i64);
            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(ListTemplatesError::from_response(response))
        }
    }

    /// <p>Lists the Amazon QuickSight groups that an Amazon QuickSight user is a member of.</p>
    async fn list_user_groups(
        &self,
        input: ListUserGroupsRequest,
    ) -> Result<ListUserGroupsResponse, RusotoError<ListUserGroupsError>> {
        let request_uri = format!(
            "/accounts/{aws_account_id}/namespaces/{namespace}/users/{user_name}/groups",
            aws_account_id = input.aws_account_id,
            namespace = input.namespace,
            user_name = input.user_name
        );

        let mut request = SignedRequest::new("GET", "quicksight", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut params = Params::new();
        if let Some(ref x) = input.max_results {
            params.put("max-results", x);
        }
        if let Some(ref x) = input.next_token {
            params.put("next-token", x);
        }
        request.set_params(params);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let mut result = proto::json::ResponsePayload::new(&response)
                .deserialize::<ListUserGroupsResponse, _>()?;

            result.status = Some(response.status.as_u16() as i64);
            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(ListUserGroupsError::from_response(response))
        }
    }

    /// <p>Returns a list of all of the Amazon QuickSight users belonging to this account. </p>
    async fn list_users(
        &self,
        input: ListUsersRequest,
    ) -> Result<ListUsersResponse, RusotoError<ListUsersError>> {
        let request_uri = format!(
            "/accounts/{aws_account_id}/namespaces/{namespace}/users",
            aws_account_id = input.aws_account_id,
            namespace = input.namespace
        );

        let mut request = SignedRequest::new("GET", "quicksight", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut params = Params::new();
        if let Some(ref x) = input.max_results {
            params.put("max-results", x);
        }
        if let Some(ref x) = input.next_token {
            params.put("next-token", x);
        }
        request.set_params(params);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let mut result = proto::json::ResponsePayload::new(&response)
                .deserialize::<ListUsersResponse, _>()?;

            result.status = Some(response.status.as_u16() as i64);
            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(ListUsersError::from_response(response))
        }
    }

    /// <p>Creates an Amazon QuickSight user, whose identity is associated with the AWS Identity and Access Management (IAM) identity or role specified in the request. </p>
    async fn register_user(
        &self,
        input: RegisterUserRequest,
    ) -> Result<RegisterUserResponse, RusotoError<RegisterUserError>> {
        let request_uri = format!(
            "/accounts/{aws_account_id}/namespaces/{namespace}/users",
            aws_account_id = input.aws_account_id,
            namespace = input.namespace
        );

        let mut request = SignedRequest::new("POST", "quicksight", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let mut result = proto::json::ResponsePayload::new(&response)
                .deserialize::<RegisterUserResponse, _>()?;

            result.status = Some(response.status.as_u16() as i64);
            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(RegisterUserError::from_response(response))
        }
    }

    /// <p>Searchs for dashboards that belong to a user. </p>
    async fn search_dashboards(
        &self,
        input: SearchDashboardsRequest,
    ) -> Result<SearchDashboardsResponse, RusotoError<SearchDashboardsError>> {
        let request_uri = format!(
            "/accounts/{aws_account_id}/search/dashboards",
            aws_account_id = input.aws_account_id
        );

        let mut request = SignedRequest::new("POST", "quicksight", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let mut result = proto::json::ResponsePayload::new(&response)
                .deserialize::<SearchDashboardsResponse, _>()?;

            result.status = Some(response.status.as_u16() as i64);
            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(SearchDashboardsError::from_response(response))
        }
    }

    /// <p><p>Assigns one or more tags (key-value pairs) to the specified QuickSight resource. </p> <p>Tags can help you organize and categorize your resources. You can also use them to scope user permissions, by granting a user permission to access or change only resources with certain tag values. You can use the <code>TagResource</code> operation with a resource that already has tags. If you specify a new tag key for the resource, this tag is appended to the list of tags associated with the resource. If you specify a tag key that is already associated with the resource, the new tag value that you specify replaces the previous value for that tag.</p> <p>You can associate as many as 50 tags with a resource. QuickSight supports tagging on data set, data source, dashboard, and template. </p> <p>Tagging for QuickSight works in a similar way to tagging for other AWS services, except for the following:</p> <ul> <li> <p>You can&#39;t use tags to track AWS costs for QuickSight. This restriction is because QuickSight costs are based on users and SPICE capacity, which aren&#39;t taggable resources.</p> </li> <li> <p>QuickSight doesn&#39;t currently support the Tag Editor for AWS Resource Groups.</p> </li> </ul></p>
    async fn tag_resource(
        &self,
        input: TagResourceRequest,
    ) -> Result<TagResourceResponse, RusotoError<TagResourceError>> {
        let request_uri = format!(
            "/resources/{resource_arn}/tags",
            resource_arn = input.resource_arn
        );

        let mut request = SignedRequest::new("POST", "quicksight", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let mut result = proto::json::ResponsePayload::new(&response)
                .deserialize::<TagResourceResponse, _>()?;

            result.status = Some(response.status.as_u16() as i64);
            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(TagResourceError::from_response(response))
        }
    }

    /// <p>Removes a tag or tags from a resource.</p>
    async fn untag_resource(
        &self,
        input: UntagResourceRequest,
    ) -> Result<UntagResourceResponse, RusotoError<UntagResourceError>> {
        let request_uri = format!(
            "/resources/{resource_arn}/tags",
            resource_arn = input.resource_arn
        );

        let mut request = SignedRequest::new("DELETE", "quicksight", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut params = Params::new();
        for item in input.tag_keys.iter() {
            params.put("keys", item);
        }
        request.set_params(params);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let mut result = proto::json::ResponsePayload::new(&response)
                .deserialize::<UntagResourceResponse, _>()?;

            result.status = Some(response.status.as_u16() as i64);
            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(UntagResourceError::from_response(response))
        }
    }

    /// <p>Updates a dashboard in an AWS account.</p>
    async fn update_dashboard(
        &self,
        input: UpdateDashboardRequest,
    ) -> Result<UpdateDashboardResponse, RusotoError<UpdateDashboardError>> {
        let request_uri = format!(
            "/accounts/{aws_account_id}/dashboards/{dashboard_id}",
            aws_account_id = input.aws_account_id,
            dashboard_id = input.dashboard_id
        );

        let mut request = SignedRequest::new("PUT", "quicksight", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<UpdateDashboardResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(UpdateDashboardError::from_response(response))
        }
    }

    /// <p>Updates read and write permissions on a dashboard.</p>
    async fn update_dashboard_permissions(
        &self,
        input: UpdateDashboardPermissionsRequest,
    ) -> Result<UpdateDashboardPermissionsResponse, RusotoError<UpdateDashboardPermissionsError>>
    {
        let request_uri = format!(
            "/accounts/{aws_account_id}/dashboards/{dashboard_id}/permissions",
            aws_account_id = input.aws_account_id,
            dashboard_id = input.dashboard_id
        );

        let mut request = SignedRequest::new("PUT", "quicksight", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let mut result = proto::json::ResponsePayload::new(&response)
                .deserialize::<UpdateDashboardPermissionsResponse, _>()?;

            result.status = Some(response.status.as_u16() as i64);
            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(UpdateDashboardPermissionsError::from_response(response))
        }
    }

    /// <p>Updates the published version of a dashboard.</p>
    async fn update_dashboard_published_version(
        &self,
        input: UpdateDashboardPublishedVersionRequest,
    ) -> Result<
        UpdateDashboardPublishedVersionResponse,
        RusotoError<UpdateDashboardPublishedVersionError>,
    > {
        let request_uri = format!(
            "/accounts/{aws_account_id}/dashboards/{dashboard_id}/versions/{version_number}",
            aws_account_id = input.aws_account_id,
            dashboard_id = input.dashboard_id,
            version_number = input.version_number
        );

        let mut request = SignedRequest::new("PUT", "quicksight", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let mut result = proto::json::ResponsePayload::new(&response)
                .deserialize::<UpdateDashboardPublishedVersionResponse, _>()?;

            result.status = Some(response.status.as_u16() as i64);
            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(UpdateDashboardPublishedVersionError::from_response(
                response,
            ))
        }
    }

    /// <p>Updates a dataset.</p>
    async fn update_data_set(
        &self,
        input: UpdateDataSetRequest,
    ) -> Result<UpdateDataSetResponse, RusotoError<UpdateDataSetError>> {
        let request_uri = format!(
            "/accounts/{aws_account_id}/data-sets/{data_set_id}",
            aws_account_id = input.aws_account_id,
            data_set_id = input.data_set_id
        );

        let mut request = SignedRequest::new("PUT", "quicksight", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let mut result = proto::json::ResponsePayload::new(&response)
                .deserialize::<UpdateDataSetResponse, _>()?;

            result.status = Some(response.status.as_u16() as i64);
            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(UpdateDataSetError::from_response(response))
        }
    }

    /// <p>Updates the permissions on a dataset.</p> <p>The permissions resource is <code>arn:aws:quicksight:region:aws-account-id:dataset/data-set-id</code>.</p>
    async fn update_data_set_permissions(
        &self,
        input: UpdateDataSetPermissionsRequest,
    ) -> Result<UpdateDataSetPermissionsResponse, RusotoError<UpdateDataSetPermissionsError>> {
        let request_uri = format!(
            "/accounts/{aws_account_id}/data-sets/{data_set_id}/permissions",
            aws_account_id = input.aws_account_id,
            data_set_id = input.data_set_id
        );

        let mut request = SignedRequest::new("POST", "quicksight", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let mut result = proto::json::ResponsePayload::new(&response)
                .deserialize::<UpdateDataSetPermissionsResponse, _>()?;

            result.status = Some(response.status.as_u16() as i64);
            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(UpdateDataSetPermissionsError::from_response(response))
        }
    }

    /// <p>Updates a data source.</p>
    async fn update_data_source(
        &self,
        input: UpdateDataSourceRequest,
    ) -> Result<UpdateDataSourceResponse, RusotoError<UpdateDataSourceError>> {
        let request_uri = format!(
            "/accounts/{aws_account_id}/data-sources/{data_source_id}",
            aws_account_id = input.aws_account_id,
            data_source_id = input.data_source_id
        );

        let mut request = SignedRequest::new("PUT", "quicksight", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let mut result = proto::json::ResponsePayload::new(&response)
                .deserialize::<UpdateDataSourceResponse, _>()?;

            result.status = Some(response.status.as_u16() as i64);
            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(UpdateDataSourceError::from_response(response))
        }
    }

    /// <p>Updates the permissions to a data source.</p>
    async fn update_data_source_permissions(
        &self,
        input: UpdateDataSourcePermissionsRequest,
    ) -> Result<UpdateDataSourcePermissionsResponse, RusotoError<UpdateDataSourcePermissionsError>>
    {
        let request_uri = format!(
            "/accounts/{aws_account_id}/data-sources/{data_source_id}/permissions",
            aws_account_id = input.aws_account_id,
            data_source_id = input.data_source_id
        );

        let mut request = SignedRequest::new("POST", "quicksight", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let mut result = proto::json::ResponsePayload::new(&response)
                .deserialize::<UpdateDataSourcePermissionsResponse, _>()?;

            result.status = Some(response.status.as_u16() as i64);
            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(UpdateDataSourcePermissionsError::from_response(response))
        }
    }

    /// <p>Changes a group description. </p>
    async fn update_group(
        &self,
        input: UpdateGroupRequest,
    ) -> Result<UpdateGroupResponse, RusotoError<UpdateGroupError>> {
        let request_uri = format!(
            "/accounts/{aws_account_id}/namespaces/{namespace}/groups/{group_name}",
            aws_account_id = input.aws_account_id,
            group_name = input.group_name,
            namespace = input.namespace
        );

        let mut request = SignedRequest::new("PUT", "quicksight", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let mut result = proto::json::ResponsePayload::new(&response)
                .deserialize::<UpdateGroupResponse, _>()?;

            result.status = Some(response.status.as_u16() as i64);
            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(UpdateGroupError::from_response(response))
        }
    }

    /// <p>Updates an existing IAM policy assignment. This operation updates only the optional parameter or parameters that are specified in the request.</p>
    async fn update_iam_policy_assignment(
        &self,
        input: UpdateIAMPolicyAssignmentRequest,
    ) -> Result<UpdateIAMPolicyAssignmentResponse, RusotoError<UpdateIAMPolicyAssignmentError>>
    {
        let request_uri = format!("/accounts/{aws_account_id}/namespaces/{namespace}/iam-policy-assignments/{assignment_name}", assignment_name = input.assignment_name, aws_account_id = input.aws_account_id, namespace = input.namespace);

        let mut request = SignedRequest::new("PUT", "quicksight", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let mut result = proto::json::ResponsePayload::new(&response)
                .deserialize::<UpdateIAMPolicyAssignmentResponse, _>()?;

            result.status = Some(response.status.as_u16() as i64);
            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(UpdateIAMPolicyAssignmentError::from_response(response))
        }
    }

    /// <p>Updates a template from an existing Amazon QuickSight analysis or another template.</p>
    async fn update_template(
        &self,
        input: UpdateTemplateRequest,
    ) -> Result<UpdateTemplateResponse, RusotoError<UpdateTemplateError>> {
        let request_uri = format!(
            "/accounts/{aws_account_id}/templates/{template_id}",
            aws_account_id = input.aws_account_id,
            template_id = input.template_id
        );

        let mut request = SignedRequest::new("PUT", "quicksight", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let mut result = proto::json::ResponsePayload::new(&response)
                .deserialize::<UpdateTemplateResponse, _>()?;

            result.status = Some(response.status.as_u16() as i64);
            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(UpdateTemplateError::from_response(response))
        }
    }

    /// <p>Updates the template alias of a template.</p>
    async fn update_template_alias(
        &self,
        input: UpdateTemplateAliasRequest,
    ) -> Result<UpdateTemplateAliasResponse, RusotoError<UpdateTemplateAliasError>> {
        let request_uri = format!(
            "/accounts/{aws_account_id}/templates/{template_id}/aliases/{alias_name}",
            alias_name = input.alias_name,
            aws_account_id = input.aws_account_id,
            template_id = input.template_id
        );

        let mut request = SignedRequest::new("PUT", "quicksight", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let mut result = proto::json::ResponsePayload::new(&response)
                .deserialize::<UpdateTemplateAliasResponse, _>()?;

            result.status = Some(response.status.as_u16() as i64);
            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(UpdateTemplateAliasError::from_response(response))
        }
    }

    /// <p>Updates the resource permissions for a template.</p>
    async fn update_template_permissions(
        &self,
        input: UpdateTemplatePermissionsRequest,
    ) -> Result<UpdateTemplatePermissionsResponse, RusotoError<UpdateTemplatePermissionsError>>
    {
        let request_uri = format!(
            "/accounts/{aws_account_id}/templates/{template_id}/permissions",
            aws_account_id = input.aws_account_id,
            template_id = input.template_id
        );

        let mut request = SignedRequest::new("PUT", "quicksight", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let mut result = proto::json::ResponsePayload::new(&response)
                .deserialize::<UpdateTemplatePermissionsResponse, _>()?;

            result.status = Some(response.status.as_u16() as i64);
            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(UpdateTemplatePermissionsError::from_response(response))
        }
    }

    /// <p>Updates an Amazon QuickSight user.</p>
    async fn update_user(
        &self,
        input: UpdateUserRequest,
    ) -> Result<UpdateUserResponse, RusotoError<UpdateUserError>> {
        let request_uri = format!(
            "/accounts/{aws_account_id}/namespaces/{namespace}/users/{user_name}",
            aws_account_id = input.aws_account_id,
            namespace = input.namespace,
            user_name = input.user_name
        );

        let mut request = SignedRequest::new("PUT", "quicksight", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let mut result = proto::json::ResponsePayload::new(&response)
                .deserialize::<UpdateUserResponse, _>()?;

            result.status = Some(response.status.as_u16() as i64);
            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(UpdateUserError::from_response(response))
        }
    }
}
