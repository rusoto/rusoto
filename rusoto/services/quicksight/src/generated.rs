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
/// <p>The Amazon QuickSight customizations associated with your AWS account or a QuickSight namespace in a specific AWS Region.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct AccountCustomization {
    /// <p>The default theme for this QuickSight subscription.</p>
    #[serde(rename = "DefaultTheme")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_theme: Option<String>,
}

/// <p>The QuickSight settings associated with your AWS account.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct AccountSettings {
    /// <p>The "account name" you provided for the QuickSight subscription in your AWS account. You create this name when you sign up for QuickSight. It is unique in all of AWS and it appears only in the console when users sign in.</p>
    #[serde(rename = "AccountName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_name: Option<String>,
    /// <p>The default QuickSight namespace for your AWS account. </p>
    #[serde(rename = "DefaultNamespace")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_namespace: Option<String>,
    /// <p>The edition of QuickSight that you're currently subscribed to: Enterprise edition or Standard edition.</p>
    #[serde(rename = "Edition")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub edition: Option<String>,
    /// <p>The main notification email for your QuickSight subscription.</p>
    #[serde(rename = "NotificationEmail")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notification_email: Option<String>,
}

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

/// <p>Metadata structure for an analysis in Amazon QuickSight</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct Analysis {
    /// <p>The ID of the analysis.</p>
    #[serde(rename = "AnalysisId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub analysis_id: Option<String>,
    /// <p>The Amazon Resource Name (ARN) of the analysis.</p>
    #[serde(rename = "Arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p>The time that the analysis was created.</p>
    #[serde(rename = "CreatedTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_time: Option<f64>,
    /// <p>The ARNs of the datasets of the analysis.</p>
    #[serde(rename = "DataSetArns")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_set_arns: Option<Vec<String>>,
    /// <p>Errors associated with the analysis.</p>
    #[serde(rename = "Errors")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub errors: Option<Vec<AnalysisError>>,
    /// <p>The time that the analysis was last updated.</p>
    #[serde(rename = "LastUpdatedTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_time: Option<f64>,
    /// <p>The descriptive name of the analysis.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>A list of the associated sheets with the unique identifier and name of each sheet.</p>
    #[serde(rename = "Sheets")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sheets: Option<Vec<Sheet>>,
    /// <p>Status associated with the analysis.</p>
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// <p>The ARN of the theme of the analysis.</p>
    #[serde(rename = "ThemeArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub theme_arn: Option<String>,
}

/// <p>A metadata error structure for an analysis.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct AnalysisError {
    /// <p>The message associated with the analysis error.</p>
    #[serde(rename = "Message")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    /// <p>The type of the analysis error.</p>
    #[serde(rename = "Type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}

/// <p>A filter that you apply when searching for one or more analyses.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct AnalysisSearchFilter {
    /// <p>The name of the value that you want to use as a filter, for example <code>"Name": "QUICKSIGHT_USER"</code>.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The comparison operator that you want to use as a filter, for example <code>"Operator": "StringEquals"</code>.</p>
    #[serde(rename = "Operator")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operator: Option<String>,
    /// <p>The value of the named item, in this case <code>QUICKSIGHT_USER</code>, that you want to use as a filter, for example <code>"Value"</code>. An example is <code>"arn:aws:quicksight:us-east-1:1:user/default/UserName1"</code>.</p>
    #[serde(rename = "Value")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

/// <p>The source entity of an analysis.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct AnalysisSourceEntity {
    /// <p>The source template for the source entity of the analysis.</p>
    #[serde(rename = "SourceTemplate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_template: Option<AnalysisSourceTemplate>,
}

/// <p>The source template of an analysis.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct AnalysisSourceTemplate {
    /// <p>The Amazon Resource Name (ARN) of the source template of an analysis.</p>
    #[serde(rename = "Arn")]
    pub arn: String,
    /// <p>The dataset references of the source template of an analysis.</p>
    #[serde(rename = "DataSetReferences")]
    pub data_set_references: Vec<DataSetReference>,
}

/// <p>The summary metadata that describes an analysis.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct AnalysisSummary {
    /// <p>The ID of the analysis. This ID displays in the URL.</p>
    #[serde(rename = "AnalysisId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub analysis_id: Option<String>,
    /// <p>The Amazon Resource Name (ARN) for the analysis.</p>
    #[serde(rename = "Arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p>The time that the analysis was created.</p>
    #[serde(rename = "CreatedTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_time: Option<f64>,
    /// <p>The time that the analysis was last updated.</p>
    #[serde(rename = "LastUpdatedTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_time: Option<f64>,
    /// <p>The name of the analysis. This name is displayed in the QuickSight console. </p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The last known status for the analysis.</p>
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
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

/// <p>The display options for tile borders for visuals.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct BorderStyle {
    /// <p>The option to enable display of borders for visuals.</p>
    #[serde(rename = "Show")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub show: Option<bool>,
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

/// <p>Metadata that contains a description for a column.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct ColumnDescription {
    /// <p>The text of a description for a column.</p>
    #[serde(rename = "Text")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
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

/// <p>A rule defined to grant access on one or more restricted columns. Each dataset can have multiple rules. To create a restricted column, you add it to one or more rules. Each rule must contain at least one column and at least one user or group. To be able to see a restricted column, a user or group needs to be added to a rule for that column.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct ColumnLevelPermissionRule {
    /// <p>An array of column names.</p>
    #[serde(rename = "ColumnNames")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub column_names: Option<Vec<String>>,
    /// <p>An array of Amazon Resource Names (ARNs) for QuickSight users or groups.</p>
    #[serde(rename = "Principals")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub principals: Option<Vec<String>>,
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

/// <p>A tag for a column in a <a>TagColumnOperation</a> structure. This is a variant type structure. For this structure to be valid, only one of the attributes can be non-null.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct ColumnTag {
    /// <p>A description for a column.</p>
    #[serde(rename = "ColumnDescription")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub column_description: Option<ColumnDescription>,
    /// <p>A geospatial role for a column.</p>
    #[serde(rename = "ColumnGeographicRole")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub column_geographic_role: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateAccountCustomizationRequest {
    /// <p>The QuickSight customizations you're adding in the current AWS Region. You can add these to an AWS account and a QuickSight namespace. </p> <p>For example, you can add a default theme by setting <code>AccountCustomization</code> to the midnight theme: <code>"AccountCustomization": { "DefaultTheme": "arn:aws:quicksight::aws:theme/MIDNIGHT" }</code>. Or, you can add a custom theme by specifying <code>"AccountCustomization": { "DefaultTheme": "arn:aws:quicksight:us-west-2:111122223333:theme/bdb844d0-0fe9-4d9d-b520-0fe602d93639" }</code>. </p>
    #[serde(rename = "AccountCustomization")]
    pub account_customization: AccountCustomization,
    /// <p>The ID for the AWS account that you want to customize QuickSight for.</p>
    #[serde(rename = "AwsAccountId")]
    pub aws_account_id: String,
    /// <p>The QuickSight namespace that you want to add customizations to.</p>
    #[serde(rename = "Namespace")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
    /// <p>A list of the tags that you want to attach to this resource.</p>
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateAccountCustomizationResponse {
    /// <p>The QuickSight customizations you're adding in the current AWS Region. </p>
    #[serde(rename = "AccountCustomization")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_customization: Option<AccountCustomization>,
    /// <p>The Amazon Resource Name (ARN) for the customization that you created for this AWS account.</p>
    #[serde(rename = "Arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p>The ID for the AWS account that you want to customize QuickSight for.</p>
    #[serde(rename = "AwsAccountId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aws_account_id: Option<String>,
    /// <p>The namespace associated with the customization you're creating. </p>
    #[serde(rename = "Namespace")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
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
pub struct CreateAnalysisRequest {
    /// <p>The ID for the analysis that you're creating. This ID displays in the URL of the analysis.</p>
    #[serde(rename = "AnalysisId")]
    pub analysis_id: String,
    /// <p>The ID of the AWS account where you are creating an analysis.</p>
    #[serde(rename = "AwsAccountId")]
    pub aws_account_id: String,
    /// <p>A descriptive name for the analysis that you're creating. This name displays for the analysis in the QuickSight console. </p>
    #[serde(rename = "Name")]
    pub name: String,
    /// <p>The parameter names and override values that you want to use. An analysis can have any parameter type, and some parameters might accept multiple values. </p>
    #[serde(rename = "Parameters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameters: Option<Parameters>,
    /// <p>A structure that describes the principals and the resource-level permissions on an analysis. You can use the <code>Permissions</code> structure to grant permissions by providing a list of AWS Identity and Access Management (IAM) action information for each principal listed by Amazon Resource Name (ARN). </p> <p>To specify no permissions, omit <code>Permissions</code>.</p>
    #[serde(rename = "Permissions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permissions: Option<Vec<ResourcePermission>>,
    /// <p>A source entity to use for the analysis that you're creating. This metadata structure contains details that describe a source template and one or more datasets.</p>
    #[serde(rename = "SourceEntity")]
    pub source_entity: AnalysisSourceEntity,
    /// <p>Contains a map of the key-value pairs for the resource tag or tags assigned to the analysis.</p>
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
    /// <p>The ARN for the theme to apply to the analysis that you're creating. To see the theme in the QuickSight console, make sure that you have access to it.</p>
    #[serde(rename = "ThemeArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub theme_arn: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateAnalysisResponse {
    /// <p>The ID of the analysis.</p>
    #[serde(rename = "AnalysisId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub analysis_id: Option<String>,
    /// <p>The ARN for the analysis.</p>
    #[serde(rename = "Arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p>The status of the creation of the analysis. </p>
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
    /// <p><p>Options for publishing the dashboard when you create it:</p> <ul> <li> <p> <code>AvailabilityStatus</code> for <code>AdHocFilteringOption</code> - This status can be either <code>ENABLED</code> or <code>DISABLED</code>. When this is set to <code>DISABLED</code>, QuickSight disables the left filter pane on the published dashboard, which can be used for ad hoc (one-time) filtering. This option is <code>ENABLED</code> by default. </p> </li> <li> <p> <code>AvailabilityStatus</code> for <code>ExportToCSVOption</code> - This status can be either <code>ENABLED</code> or <code>DISABLED</code>. The visual option to export data to .CSV format isn&#39;t enabled when this is set to <code>DISABLED</code>. This option is <code>ENABLED</code> by default. </p> </li> <li> <p> <code>VisibilityState</code> for <code>SheetControlsOption</code> - This visibility state can be either <code>COLLAPSED</code> or <code>EXPANDED</code>. This option is <code>COLLAPSED</code> by default. </p> </li> </ul></p>
    #[serde(rename = "DashboardPublishOptions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dashboard_publish_options: Option<DashboardPublishOptions>,
    /// <p>The display name of the dashboard.</p>
    #[serde(rename = "Name")]
    pub name: String,
    /// <p>The parameters for the creation of the dashboard, which you want to use to override the default settings. A dashboard can have any type of parameters, and some parameters might accept multiple values. </p>
    #[serde(rename = "Parameters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameters: Option<Parameters>,
    /// <p>A structure that contains the permissions of the dashboard. You can use this structure for granting permissions by providing a list of IAM action information for each principal ARN. </p> <p>To specify no permissions, omit the permissions list.</p>
    #[serde(rename = "Permissions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permissions: Option<Vec<ResourcePermission>>,
    /// <p>The entity that you are using as a source when you create the dashboard. In <code>SourceEntity</code>, you specify the type of object you're using as source. You can only create a dashboard from a template, so you use a <code>SourceTemplate</code> entity. If you need to create a dashboard from an analysis, first convert the analysis to a template by using the <a>CreateTemplate</a> API operation. For <code>SourceTemplate</code>, specify the Amazon Resource Name (ARN) of the source template. The <code>SourceTemplate</code>ARN can contain any AWS Account and any QuickSight-supported AWS Region. </p> <p>Use the <code>DataSetReferences</code> entity within <code>SourceTemplate</code> to list the replacement datasets for the placeholders listed in the original. The schema in each dataset must match its placeholder. </p>
    #[serde(rename = "SourceEntity")]
    pub source_entity: DashboardSourceEntity,
    /// <p>Contains a map of the key-value pairs for the resource tag or tags assigned to the dashboard.</p>
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
    /// <p>The Amazon Resource Name (ARN) of the theme that is being used for this dashboard. If you add a value for this field, it overrides the value that is used in the source entity. The theme ARN must exist in the same AWS account where you create the dashboard.</p>
    #[serde(rename = "ThemeArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub theme_arn: Option<String>,
    /// <p>A description for the first version of the dashboard being created.</p>
    #[serde(rename = "VersionDescription")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version_description: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateDashboardResponse {
    /// <p>The ARN of the dashboard.</p>
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
    /// <p>A set of one or more definitions of a <code> <a>ColumnLevelPermissionRule</a> </code>.</p>
    #[serde(rename = "ColumnLevelPermissionRules")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub column_level_permission_rules: Option<Vec<ColumnLevelPermissionRule>>,
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
    /// <p>The name of the assignment, also called a rule. It must be unique within an AWS account.</p>
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
pub struct CreateNamespaceRequest {
    /// <p>The ID for the AWS account that you want to create the QuickSight namespace in.</p>
    #[serde(rename = "AwsAccountId")]
    pub aws_account_id: String,
    /// <p>Specifies the type of your user identity directory. Currently, this supports users with an identity type of <code>QUICKSIGHT</code>.</p>
    #[serde(rename = "IdentityStore")]
    pub identity_store: String,
    /// <p>The name that you want to use to describe the new namespace.</p>
    #[serde(rename = "Namespace")]
    pub namespace: String,
    /// <p>The tags that you want to associate with the namespace that you're creating.</p>
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateNamespaceResponse {
    /// <p>The ARN of the QuickSight namespace you created. </p>
    #[serde(rename = "Arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p>The AWS Region that you want to use for the free SPICE capacity for the new namespace. This is set to the region that you run CreateNamespace in. </p>
    #[serde(rename = "CapacityRegion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capacity_region: Option<String>,
    /// <p>The status of the creation of the namespace. This is an asynchronous process. A status of <code>CREATED</code> means that your namespace is ready to use. If an error occurs, it indicates if the process is <code>retryable</code> or <code>non-retryable</code>. In the case of a non-retryable error, refer to the error message for follow-up tasks.</p>
    #[serde(rename = "CreationStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_status: Option<String>,
    /// <p>Specifies the type of your user identity directory. Currently, this supports users with an identity type of <code>QUICKSIGHT</code>.</p>
    #[serde(rename = "IdentityStore")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identity_store: Option<String>,
    /// <p>The name of the new namespace that you created.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
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
    /// <p>The entity that you are using as a source when you create the template. In <code>SourceEntity</code>, you specify the type of object you're using as source: <code>SourceTemplate</code> for a template or <code>SourceAnalysis</code> for an analysis. Both of these require an Amazon Resource Name (ARN). For <code>SourceTemplate</code>, specify the ARN of the source template. For <code>SourceAnalysis</code>, specify the ARN of the source analysis. The <code>SourceTemplate</code> ARN can contain any AWS Account and any QuickSight-supported AWS Region. </p> <p>Use the <code>DataSetReferences</code> entity within <code>SourceTemplate</code> or <code>SourceAnalysis</code> to list the replacement datasets for the placeholders listed in the original. The schema in each dataset must match its placeholder. </p>
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

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateThemeAliasRequest {
    /// <p>The name that you want to give to the theme alias that you are creating. The alias name can't begin with a <code>$</code>. Alias names that start with <code>$</code> are reserved by Amazon QuickSight. </p>
    #[serde(rename = "AliasName")]
    pub alias_name: String,
    /// <p>The ID of the AWS account that contains the theme for the new theme alias.</p>
    #[serde(rename = "AwsAccountId")]
    pub aws_account_id: String,
    /// <p>An ID for the theme alias.</p>
    #[serde(rename = "ThemeId")]
    pub theme_id: String,
    /// <p>The version number of the theme.</p>
    #[serde(rename = "ThemeVersionNumber")]
    pub theme_version_number: i64,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateThemeAliasResponse {
    /// <p>The AWS request ID for this operation.</p>
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// <p>The HTTP status of the request.</p>
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<i64>,
    /// <p>Information about the theme alias.</p>
    #[serde(rename = "ThemeAlias")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub theme_alias: Option<ThemeAlias>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateThemeRequest {
    /// <p>The ID of the AWS account where you want to store the new theme. </p>
    #[serde(rename = "AwsAccountId")]
    pub aws_account_id: String,
    /// <p>The ID of the theme that a custom theme will inherit from. All themes inherit from one of the starting themes defined by Amazon QuickSight. For a list of the starting themes, use <code>ListThemes</code> or choose <b>Themes</b> from within a QuickSight analysis. </p>
    #[serde(rename = "BaseThemeId")]
    pub base_theme_id: String,
    /// <p>The theme configuration, which contains the theme display properties.</p>
    #[serde(rename = "Configuration")]
    pub configuration: ThemeConfiguration,
    /// <p>A display name for the theme.</p>
    #[serde(rename = "Name")]
    pub name: String,
    /// <p>A valid grouping of resource permissions to apply to the new theme. </p>
    #[serde(rename = "Permissions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permissions: Option<Vec<ResourcePermission>>,
    /// <p>A map of the key-value pairs for the resource tag or tags that you want to add to the resource.</p>
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
    /// <p>An ID for the theme that you want to create. The theme ID is unique per AWS Region in each AWS account.</p>
    #[serde(rename = "ThemeId")]
    pub theme_id: String,
    /// <p>A description of the first version of the theme that you're creating. Every time <code>UpdateTheme</code> is called, a new version is created. Each version of the theme has a description of the version in the <code>VersionDescription</code> field.</p>
    #[serde(rename = "VersionDescription")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version_description: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateThemeResponse {
    /// <p>The Amazon Resource Name (ARN) for the theme.</p>
    #[serde(rename = "Arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p>The theme creation status.</p>
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
    /// <p>The ID of the theme.</p>
    #[serde(rename = "ThemeId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub theme_id: Option<String>,
    /// <p>The Amazon Resource Name (ARN) for the new theme.</p>
    #[serde(rename = "VersionArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version_arn: Option<String>,
}

/// <p>The combination of user name and password that are used as credentials.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CredentialPair {
    /// <p>A set of alternate data source parameters that you want to share for these credentials. The credentials are applied in tandem with the data source parameters when you copy a data source by using a create or update request. The API operation compares the <code>DataSourceParameters</code> structure that's in the request with the structures in the <code>AlternateDataSourceParameters</code> allow list. If the structures are an exact match, the request is allowed to use the new data source with the existing credentials. If the <code>AlternateDataSourceParameters</code> list is null, the <code>DataSourceParameters</code> originally used with these <code>Credentials</code> is automatically allowed.</p>
    #[serde(rename = "AlternateDataSourceParameters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alternate_data_source_parameters: Option<Vec<DataSourceParameters>>,
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
    /// <p>A display name for the dashboard.</p>
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
    /// <p>The name of the value that you want to use as a filter, for example, <code>"Name": "QUICKSIGHT_USER"</code>. </p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The comparison operator that you want to use as a filter, for example, <code>"Operator": "StringEquals"</code>.</p>
    #[serde(rename = "Operator")]
    pub operator: String,
    /// <p>The value of the named item, in this case <code>QUICKSIGHT_USER</code>, that you want to use as a filter, for example, <code>"Value": "arn:aws:quicksight:us-east-1:1:user/default/UserName1"</code>. </p>
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
    /// <p>The Amazon Resource Numbers (ARNs) for the datasets that are associated with this version of the dashboard.</p>
    #[serde(rename = "DataSetArns")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_set_arns: Option<Vec<String>>,
    /// <p>Description.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>Errors associated with this dashboard version.</p>
    #[serde(rename = "Errors")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub errors: Option<Vec<DashboardError>>,
    /// <p>A list of the associated sheets with the unique identifier and name of each sheet.</p>
    #[serde(rename = "Sheets")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sheets: Option<Vec<Sheet>>,
    /// <p>Source entity ARN.</p>
    #[serde(rename = "SourceEntityArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_entity_arn: Option<String>,
    /// <p>The HTTP status of the request.</p>
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// <p>The ARN of the theme associated with a version of the dashboard.</p>
    #[serde(rename = "ThemeArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub theme_arn: Option<String>,
    /// <p>Version number for this version of the dashboard.</p>
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

/// <p>The theme colors that are used for data colors in charts. The colors description is a hexadecimal color code that consists of six alphanumerical characters, prefixed with <code>#</code>, for example #37BFF5. </p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct DataColorPalette {
    /// <p>The hexadecimal codes for the colors.</p>
    #[serde(rename = "Colors")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub colors: Option<Vec<String>>,
    /// <p>The hexadecimal code of a color that applies to charts where a lack of data is highlighted.</p>
    #[serde(rename = "EmptyFillColor")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub empty_fill_color: Option<String>,
    /// <p>The minimum and maximum hexadecimal codes that describe a color gradient. </p>
    #[serde(rename = "MinMaxGradient")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_max_gradient: Option<Vec<String>>,
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
    /// <p>A set of one or more definitions of a <code> <a>ColumnLevelPermissionRule</a> </code>.</p>
    #[serde(rename = "ColumnLevelPermissionRules")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub column_level_permission_rules: Option<Vec<ColumnLevelPermissionRule>>,
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
    /// <p>A value that indicates whether you want to import the data into SPICE.</p>
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
    /// <p>A value that indicates if the dataset has column level permission configured.</p>
    #[serde(rename = "ColumnLevelPermissionRulesApplied")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub column_level_permission_rules_applied: Option<bool>,
    /// <p>The time that this dataset was created.</p>
    #[serde(rename = "CreatedTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_time: Option<f64>,
    /// <p>The ID of the dataset.</p>
    #[serde(rename = "DataSetId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_set_id: Option<String>,
    /// <p>A value that indicates whether you want to import the data into SPICE.</p>
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
    /// <p>A set of alternate data source parameters that you want to share for the credentials stored with this data source. The credentials are applied in tandem with the data source parameters when you copy a data source by using a create or update request. The API operation compares the <code>DataSourceParameters</code> structure that's in the request with the structures in the <code>AlternateDataSourceParameters</code> allow list. If the structures are an exact match, the request is allowed to use the credentials from this existing data source. If the <code>AlternateDataSourceParameters</code> list is null, the <code>Credentials</code> originally used with this <code>DataSourceParameters</code> are automatically allowed.</p>
    #[serde(rename = "AlternateDataSourceParameters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alternate_data_source_parameters: Option<Vec<DataSourceParameters>>,
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

/// <p>Data source credentials. This is a variant type structure. For this structure to be valid, only one of the attributes can be non-null.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DataSourceCredentials {
    /// <p>The Amazon Resource Name (ARN) of a data source that has the credential pair that you want to use. When <code>CopySourceArn</code> is not null, the credential pair from the data source in the ARN is used as the credentials for the <code>DataSourceCredentials</code> structure.</p>
    #[serde(rename = "CopySourceArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub copy_source_arn: Option<String>,
    /// <p>Credential pair. For more information, see <a>CredentialPair</a>.</p>
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
    /// <p>Oracle parameters.</p>
    #[serde(rename = "OracleParameters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub oracle_parameters: Option<OracleParameters>,
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

/// <p>A date-time parameter.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DateTimeParameter {
    /// <p>A display name for the date-time parameter.</p>
    #[serde(rename = "Name")]
    pub name: String,
    /// <p>The values for the date-time parameter.</p>
    #[serde(rename = "Values")]
    pub values: Vec<f64>,
}

/// <p>A decimal parameter.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DecimalParameter {
    /// <p>A display name for the decimal parameter.</p>
    #[serde(rename = "Name")]
    pub name: String,
    /// <p>The values for the decimal parameter.</p>
    #[serde(rename = "Values")]
    pub values: Vec<f64>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteAccountCustomizationRequest {
    /// <p>The ID for the AWS account that you want to delete QuickSight customizations from in this AWS Region.</p>
    #[serde(rename = "AwsAccountId")]
    pub aws_account_id: String,
    /// <p>The QuickSight namespace that you're deleting the customizations from.</p>
    #[serde(rename = "Namespace")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DeleteAccountCustomizationResponse {
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
pub struct DeleteAnalysisRequest {
    /// <p>The ID of the analysis that you're deleting.</p>
    #[serde(rename = "AnalysisId")]
    pub analysis_id: String,
    /// <p>The ID of the AWS account where you want to delete an analysis.</p>
    #[serde(rename = "AwsAccountId")]
    pub aws_account_id: String,
    /// <p>This option defaults to the value <code>NoForceDeleteWithoutRecovery</code>. To immediately delete the analysis, add the <code>ForceDeleteWithoutRecovery</code> option. You can't restore an analysis after it's deleted. </p>
    #[serde(rename = "ForceDeleteWithoutRecovery")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub force_delete_without_recovery: Option<bool>,
    /// <p>A value that specifies the number of days that QuickSight waits before it deletes the analysis. You can't use this parameter with the <code>ForceDeleteWithoutRecovery</code> option in the same API call. The default value is 30.</p>
    #[serde(rename = "RecoveryWindowInDays")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recovery_window_in_days: Option<i64>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DeleteAnalysisResponse {
    /// <p>The ID of the deleted analysis.</p>
    #[serde(rename = "AnalysisId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub analysis_id: Option<String>,
    /// <p>The Amazon Resource Name (ARN) of the deleted analysis.</p>
    #[serde(rename = "Arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p>The date and time that the analysis is scheduled to be deleted.</p>
    #[serde(rename = "DeletionTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deletion_time: Option<f64>,
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
pub struct DeleteNamespaceRequest {
    /// <p>The ID for the AWS account that you want to delete the QuickSight namespace from.</p>
    #[serde(rename = "AwsAccountId")]
    pub aws_account_id: String,
    /// <p>The namespace that you want to delete.</p>
    #[serde(rename = "Namespace")]
    pub namespace: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DeleteNamespaceResponse {
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
    /// <p>The name for the template alias. To delete a specific alias, you delete the version that the alias points to. You can specify the alias name, or specify the latest version of the template by providing the keyword <code>$LATEST</code> in the <code>AliasName</code> parameter. </p>
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
    /// <p>The Amazon Resource Name (ARN) of the template you want to delete.</p>
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

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteThemeAliasRequest {
    /// <p>The unique name for the theme alias to delete.</p>
    #[serde(rename = "AliasName")]
    pub alias_name: String,
    /// <p>The ID of the AWS account that contains the theme alias to delete.</p>
    #[serde(rename = "AwsAccountId")]
    pub aws_account_id: String,
    /// <p>The ID for the theme that the specified alias is for.</p>
    #[serde(rename = "ThemeId")]
    pub theme_id: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DeleteThemeAliasResponse {
    /// <p>The name for the theme alias.</p>
    #[serde(rename = "AliasName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alias_name: Option<String>,
    /// <p>The Amazon Resource Name (ARN) of the theme resource using the deleted alias.</p>
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
    /// <p>An ID for the theme associated with the deletion.</p>
    #[serde(rename = "ThemeId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub theme_id: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteThemeRequest {
    /// <p>The ID of the AWS account that contains the theme that you're deleting.</p>
    #[serde(rename = "AwsAccountId")]
    pub aws_account_id: String,
    /// <p>An ID for the theme that you want to delete.</p>
    #[serde(rename = "ThemeId")]
    pub theme_id: String,
    /// <p>The version of the theme that you want to delete. </p> <p> <b>Note:</b> If you don't provide a version number, you're using this call to <code>DeleteTheme</code> to delete all versions of the theme.</p>
    #[serde(rename = "VersionNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version_number: Option<i64>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DeleteThemeResponse {
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
    /// <p>An ID for the theme.</p>
    #[serde(rename = "ThemeId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub theme_id: Option<String>,
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
pub struct DescribeAccountCustomizationRequest {
    /// <p>The ID for the AWS account that you want to describe QuickSight customizations for.</p>
    #[serde(rename = "AwsAccountId")]
    pub aws_account_id: String,
    /// <p>The QuickSight namespace that you want to describe QuickSight customizations for.</p>
    #[serde(rename = "Namespace")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
    /// <p>The <code>Resolved</code> flag works with the other parameters to determine which view of QuickSight customizations is returned. You can add this flag to your command to use the same view that QuickSight uses to identify which customizations to apply to the console. Omit this flag, or set it to <code>no-resolved</code>, to reveal customizations that are configured at different levels. </p>
    #[serde(rename = "Resolved")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resolved: Option<bool>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeAccountCustomizationResponse {
    /// <p>The QuickSight customizations that exist in the current AWS Region. </p>
    #[serde(rename = "AccountCustomization")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_customization: Option<AccountCustomization>,
    /// <p>The Amazon Resource Name (ARN) of the customization that's associated with this AWS account.</p>
    #[serde(rename = "Arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p>The ID for the AWS account that you're describing.</p>
    #[serde(rename = "AwsAccountId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aws_account_id: Option<String>,
    /// <p>The QuickSight namespace that you're describing. </p>
    #[serde(rename = "Namespace")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
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
pub struct DescribeAccountSettingsRequest {
    /// <p>The ID for the AWS account that contains the settings that you want to list.</p>
    #[serde(rename = "AwsAccountId")]
    pub aws_account_id: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeAccountSettingsResponse {
    /// <p>The QuickSight settings for this AWS account. This information includes the edition of Amazon QuickSight that you subscribed to (Standard or Enterprise) and the notification email for the QuickSight subscription. In the QuickSight console, the QuickSight subscription is sometimes referred to as a QuickSight "account" even though it's technically not an account by itself. Instead, it's a subscription to the QuickSight service for your AWS account. The edition that you subscribe to applies to QuickSight in every AWS Region where you use it.</p>
    #[serde(rename = "AccountSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_settings: Option<AccountSettings>,
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
pub struct DescribeAnalysisPermissionsRequest {
    /// <p>The ID of the analysis whose permissions you're describing. The ID is part of the analysis URL.</p>
    #[serde(rename = "AnalysisId")]
    pub analysis_id: String,
    /// <p>The ID of the AWS account that contains the analysis whose permissions you're describing. You must be using the AWS account that the analysis is in.</p>
    #[serde(rename = "AwsAccountId")]
    pub aws_account_id: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeAnalysisPermissionsResponse {
    /// <p>The Amazon Resource Name (ARN) of the analysis whose permissions you're describing.</p>
    #[serde(rename = "AnalysisArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub analysis_arn: Option<String>,
    /// <p>The ID of the analysis whose permissions you're describing.</p>
    #[serde(rename = "AnalysisId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub analysis_id: Option<String>,
    /// <p>A structure that describes the principals and the resource-level permissions on an analysis.</p>
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
pub struct DescribeAnalysisRequest {
    /// <p>The ID of the analysis that you're describing. The ID is part of the URL of the analysis.</p>
    #[serde(rename = "AnalysisId")]
    pub analysis_id: String,
    /// <p>The ID of the AWS account that contains the analysis. You must be using the AWS account that the analysis is in.</p>
    #[serde(rename = "AwsAccountId")]
    pub aws_account_id: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeAnalysisResponse {
    /// <p>A metadata structure that contains summary information for the analysis that you're describing.</p>
    #[serde(rename = "Analysis")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub analysis: Option<Analysis>,
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
    /// <p>The name of the assignment, also called a rule.</p>
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
pub struct DescribeNamespaceRequest {
    /// <p>The ID for the AWS account that contains the QuickSight namespace that you want to describe.</p>
    #[serde(rename = "AwsAccountId")]
    pub aws_account_id: String,
    /// <p>The namespace that you want to describe.</p>
    #[serde(rename = "Namespace")]
    pub namespace: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeNamespaceResponse {
    /// <p>The information about the namespace that you're describing. The response includes the namespace ARN, name, AWS Region, creation status, and identity store. <code>DescribeNamespace</code> also works for namespaces that are in the process of being created. For incomplete namespaces, this API operation lists the namespace error types and messages associated with the creation process.</p>
    #[serde(rename = "Namespace")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub namespace: Option<NamespaceInfoV2>,
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
    /// <p>The AWS request ID for this operation.</p>
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
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
pub struct DescribeThemeAliasRequest {
    /// <p>The name of the theme alias that you want to describe.</p>
    #[serde(rename = "AliasName")]
    pub alias_name: String,
    /// <p>The ID of the AWS account that contains the theme alias that you're describing.</p>
    #[serde(rename = "AwsAccountId")]
    pub aws_account_id: String,
    /// <p>The ID for the theme.</p>
    #[serde(rename = "ThemeId")]
    pub theme_id: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeThemeAliasResponse {
    /// <p>The AWS request ID for this operation.</p>
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// <p>The HTTP status of the request.</p>
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<i64>,
    /// <p>Information about the theme alias.</p>
    #[serde(rename = "ThemeAlias")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub theme_alias: Option<ThemeAlias>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeThemePermissionsRequest {
    /// <p>The ID of the AWS account that contains the theme that you're describing.</p>
    #[serde(rename = "AwsAccountId")]
    pub aws_account_id: String,
    /// <p>The ID for the theme that you want to describe permissions for.</p>
    #[serde(rename = "ThemeId")]
    pub theme_id: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeThemePermissionsResponse {
    /// <p>A list of resource permissions set on the theme. </p>
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
    /// <p>The Amazon Resource Name (ARN) of the theme.</p>
    #[serde(rename = "ThemeArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub theme_arn: Option<String>,
    /// <p>The ID for the theme.</p>
    #[serde(rename = "ThemeId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub theme_id: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeThemeRequest {
    /// <p>The alias of the theme that you want to describe. If you name a specific alias, you describe the version that the alias points to. You can specify the latest version of the theme by providing the keyword <code>$LATEST</code> in the <code>AliasName</code> parameter. The keyword <code>$PUBLISHED</code> doesn't apply to themes.</p>
    #[serde(rename = "AliasName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alias_name: Option<String>,
    /// <p>The ID of the AWS account that contains the theme that you're describing.</p>
    #[serde(rename = "AwsAccountId")]
    pub aws_account_id: String,
    /// <p>The ID for the theme.</p>
    #[serde(rename = "ThemeId")]
    pub theme_id: String,
    /// <p>The version number for the version to describe. If a <code>VersionNumber</code> parameter value isn't provided, the latest version of the theme is described.</p>
    #[serde(rename = "VersionNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version_number: Option<i64>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeThemeResponse {
    /// <p>The AWS request ID for this operation.</p>
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// <p>The HTTP status of the request.</p>
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<i64>,
    /// <p>The information about the theme that you are describing.</p>
    #[serde(rename = "Theme")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub theme: Option<Theme>,
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
    /// <p>A list of one or more dashboard IDs that you want to add to a session that includes anonymous users. The <code>IdentityType</code> parameter must be set to <code>ANONYMOUS</code> for this to work, because other identity types authenticate as QuickSight or IAM users. For example, if you set "<code>--dashboard-id dash_id1 --dashboard-id dash_id2 dash_id3 identity-type ANONYMOUS</code>", the session can access all three dashboards. </p>
    #[serde(rename = "AdditionalDashboardIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_dashboard_ids: Option<Vec<String>>,
    /// <p>The ID for the AWS account that contains the dashboard that you're embedding.</p>
    #[serde(rename = "AwsAccountId")]
    pub aws_account_id: String,
    /// <p>The ID for the dashboard, also added to the AWS Identity and Access Management (IAM) policy.</p>
    #[serde(rename = "DashboardId")]
    pub dashboard_id: String,
    /// <p>The authentication method that the user uses to sign in.</p>
    #[serde(rename = "IdentityType")]
    pub identity_type: String,
    /// <p>The QuickSight namespace that contains the dashboard IDs in this request. If you're not using a custom namespace, set this to "<code>default</code>".</p>
    #[serde(rename = "Namespace")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
    /// <p>Remove the reset button on the embedded dashboard. The default is FALSE, which enables the reset button.</p>
    #[serde(rename = "ResetDisabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reset_disabled: Option<bool>,
    /// <p>How many minutes the session is valid. The session lifetime must be 15-600 minutes.</p>
    #[serde(rename = "SessionLifetimeInMinutes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub session_lifetime_in_minutes: Option<i64>,
    /// <p>Adds persistence of state for the user session in an embedded dashboard. Persistence applies to the sheet and the parameter settings. These are control settings that the dashboard subscriber (QuickSight reader) chooses while viewing the dashboard. If this is set to <code>TRUE</code>, the settings are the same when the subscriber reopens the same dashboard URL. The state is stored in QuickSight, not in a browser cookie. If this is set to FALSE, the state of the user session is not persisted. The default is <code>FALSE</code>.</p>
    #[serde(rename = "StatePersistenceEnabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state_persistence_enabled: Option<bool>,
    /// <p>Remove the undo/redo button on the embedded dashboard. The default is FALSE, which enables the undo/redo button.</p>
    #[serde(rename = "UndoRedoDisabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub undo_redo_disabled: Option<bool>,
    /// <p>The Amazon QuickSight user's Amazon Resource Name (ARN), for use with <code>QUICKSIGHT</code> identity type. You can use this for any Amazon QuickSight users in your account (readers, authors, or admins) authenticated as one of the following:</p> <ul> <li> <p>Active Directory (AD) users or group members</p> </li> <li> <p>Invited nonfederated users</p> </li> <li> <p>IAM users and IAM role-based sessions authenticated through Federated Single Sign-On using SAML, OpenID Connect, or IAM federation.</p> </li> </ul> <p>Omit this parameter for users in the third group – IAM users and IAM role-based sessions.</p>
    #[serde(rename = "UserArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_arn: Option<String>,
}

/// <p>Output returned from the <code>GetDashboardEmbedUrl</code> operation.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetDashboardEmbedUrlResponse {
    /// <p>A single-use URL that you can put into your server-side webpage to embed your dashboard. This URL is valid for 5 minutes. The API operation provides the URL with an <code>auth_code</code> value that enables one (and only one) sign-on to a user session that is valid for 10 hours. </p>
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

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetSessionEmbedUrlRequest {
    /// <p>The ID for the AWS account associated with your QuickSight subscription.</p>
    #[serde(rename = "AwsAccountId")]
    pub aws_account_id: String,
    /// <p><p>The URL you use to access the embedded session. The entry point URL is constrained to the following paths:</p> <ul> <li> <p> <code>/start</code> </p> </li> <li> <p> <code>/start/analyses</code> </p> </li> <li> <p> <code>/start/dashboards</code> </p> </li> <li> <p> <code>/start/favorites</code> </p> </li> <li> <p> <code>/dashboards/<i>DashboardId</i> </code> - where <code>DashboardId</code> is the actual ID key from the QuickSight console URL of the dashboard</p> </li> <li> <p> <code>/analyses/<i>AnalysisId</i> </code> - where <code>AnalysisId</code> is the actual ID key from the QuickSight console URL of the analysis</p> </li> </ul></p>
    #[serde(rename = "EntryPoint")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entry_point: Option<String>,
    /// <p>How many minutes the session is valid. The session lifetime must be 15-600 minutes.</p>
    #[serde(rename = "SessionLifetimeInMinutes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub session_lifetime_in_minutes: Option<i64>,
    /// <p>The Amazon QuickSight user's Amazon Resource Name (ARN), for use with <code>QUICKSIGHT</code> identity type. You can use this for any type of Amazon QuickSight users in your account (readers, authors, or admins). They need to be authenticated as one of the following:</p> <ol> <li> <p>Active Directory (AD) users or group members</p> </li> <li> <p>Invited nonfederated users</p> </li> <li> <p>AWS Identity and Access Management (IAM) users and IAM role-based sessions authenticated through Federated Single Sign-On using SAML, OpenID Connect, or IAM federation</p> </li> </ol> <p>Omit this parameter for users in the third group, IAM users and IAM role-based sessions.</p>
    #[serde(rename = "UserArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_arn: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetSessionEmbedUrlResponse {
    /// <p>A single-use URL that you can put into your server-side web page to embed your QuickSight session. This URL is valid for 5 minutes. The API operation provides the URL with an <code>auth_code</code> value that enables one (and only one) sign-on to a user session that is valid for 10 hours. </p>
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

/// <p>A <i>group</i> in Amazon QuickSight consists of a set of users. You can use groups to make it easier to manage access and security. </p>
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

/// <p>The display options for gutter spacing between tiles on a sheet.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct GutterStyle {
    /// <p>This Boolean value controls whether to display a gutter space between sheet tiles. </p>
    #[serde(rename = "Show")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub show: Option<bool>,
}

/// <p>An AWS Identity and Access Management (IAM) policy assignment.</p>
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

/// <p>An integer parameter.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct IntegerParameter {
    /// <p>The name of the integer parameter.</p>
    #[serde(rename = "Name")]
    pub name: String,
    /// <p>The values for the integer parameter.</p>
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

/// <p>The instructions associated with a join. </p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct JoinInstruction {
    /// <p>Join key properties of the left operand.</p>
    #[serde(rename = "LeftJoinKeyProperties")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub left_join_key_properties: Option<JoinKeyProperties>,
    /// <p>The operand on the left side of a join.</p>
    #[serde(rename = "LeftOperand")]
    pub left_operand: String,
    /// <p>The join instructions provided in the <code>ON</code> clause of a join.</p>
    #[serde(rename = "OnClause")]
    pub on_clause: String,
    /// <p>Join key properties of the right operand.</p>
    #[serde(rename = "RightJoinKeyProperties")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub right_join_key_properties: Option<JoinKeyProperties>,
    /// <p>The operand on the right side of a join.</p>
    #[serde(rename = "RightOperand")]
    pub right_operand: String,
    /// <p>The type of join that it is.</p>
    #[serde(rename = "Type")]
    pub type_: String,
}

/// <p>Properties associated with the columns participating in a join.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct JoinKeyProperties {
    /// <p>A value that indicates that a row in a table is uniquely identified by the columns in a join key. This is used by QuickSight to optimize query performance.</p>
    #[serde(rename = "UniqueKey")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unique_key: Option<bool>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListAnalysesRequest {
    /// <p>The ID of the AWS account that contains the analyses.</p>
    #[serde(rename = "AwsAccountId")]
    pub aws_account_id: String,
    /// <p>The maximum number of results to return.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>A pagination token that can be used in a subsequent request.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListAnalysesResponse {
    /// <p>Metadata describing each of the analyses that are listed.</p>
    #[serde(rename = "AnalysisSummaryList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub analysis_summary_list: Option<Vec<AnalysisSummary>>,
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
    /// <p>A structure that contains all of the dashboards in your AWS account. This structure provides basic information about the dashboards.</p>
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
pub struct ListNamespacesRequest {
    /// <p>The ID for the AWS account that contains the QuickSight namespaces that you want to list.</p>
    #[serde(rename = "AwsAccountId")]
    pub aws_account_id: String,
    /// <p>The maximum number of results to return.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>A pagination token that can be used in a subsequent request.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListNamespacesResponse {
    /// <p>The information about the namespaces in this AWS account. The response includes the namespace ARN, name, AWS Region, notification email address, creation status, and identity store.</p>
    #[serde(rename = "Namespaces")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub namespaces: Option<Vec<NamespaceInfoV2>>,
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
pub struct ListThemeAliasesRequest {
    /// <p>The ID of the AWS account that contains the theme aliases that you're listing.</p>
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
    /// <p>The ID for the theme.</p>
    #[serde(rename = "ThemeId")]
    pub theme_id: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListThemeAliasesResponse {
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
    /// <p>A structure containing the list of the theme's aliases.</p>
    #[serde(rename = "ThemeAliasList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub theme_alias_list: Option<Vec<ThemeAlias>>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListThemeVersionsRequest {
    /// <p>The ID of the AWS account that contains the themes that you're listing.</p>
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
    /// <p>The ID for the theme.</p>
    #[serde(rename = "ThemeId")]
    pub theme_id: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListThemeVersionsResponse {
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
    /// <p>A structure containing a list of all the versions of the specified theme.</p>
    #[serde(rename = "ThemeVersionSummaryList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub theme_version_summary_list: Option<Vec<ThemeVersionSummary>>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListThemesRequest {
    /// <p>The ID of the AWS account that contains the themes that you're listing.</p>
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
    /// <p><p>The type of themes that you want to list. Valid options include the following:</p> <ul> <li> <p> <code>ALL (default)</code>- Display all existing themes.</p> </li> <li> <p> <code>CUSTOM</code> - Display only the themes created by people using Amazon QuickSight.</p> </li> <li> <p> <code>QUICKSIGHT</code> - Display only the starting themes defined by QuickSight.</p> </li> </ul></p>
    #[serde(rename = "Type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListThemesResponse {
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
    /// <p>Information about the themes in the list.</p>
    #[serde(rename = "ThemeSummaryList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub theme_summary_list: Option<Vec<ThemeSummary>>,
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

/// <p>The display options for margins around the outside edge of sheets.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct MarginStyle {
    /// <p>This Boolean value controls whether to display sheet margins.</p>
    #[serde(rename = "Show")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub show: Option<bool>,
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

/// <p>Errors that occur during namespace creation.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct NamespaceError {
    /// <p>The message for the error.</p>
    #[serde(rename = "Message")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    /// <p>The error type.</p>
    #[serde(rename = "Type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}

/// <p>The error type.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct NamespaceInfoV2 {
    /// <p>The namespace ARN.</p>
    #[serde(rename = "Arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p>The namespace AWS Region.</p>
    #[serde(rename = "CapacityRegion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capacity_region: Option<String>,
    /// <p>The creation status of a namespace that is not yet completely created.</p>
    #[serde(rename = "CreationStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_status: Option<String>,
    /// <p>The identity store used for the namespace.</p>
    #[serde(rename = "IdentityStore")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identity_store: Option<String>,
    /// <p>The name of the error.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>An error that occurred when the namespace was created.</p>
    #[serde(rename = "NamespaceError")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub namespace_error: Option<NamespaceError>,
}

/// <p>Oracle parameters.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct OracleParameters {
    /// <p>Database.</p>
    #[serde(rename = "Database")]
    pub database: String,
    /// <p>An Oracle host.</p>
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
    /// <p>A description for a column.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>A display name for the dataset.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>Type.</p>
    #[serde(rename = "Type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}

/// <p>A list of QuickSight parameters and the list's override values.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct Parameters {
    /// <p>Date-time parameters.</p>
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
    /// <p>(Enterprise edition only) The name of the custom permissions profile that you want to assign to this user. Customized permissions allows you to control a user's access by restricting access the following operations:</p> <ul> <li> <p>Create and update data sources</p> </li> <li> <p>Create and update datasets</p> </li> <li> <p>Create and update email reports</p> </li> <li> <p>Subscribe to email reports</p> </li> </ul> <p>To add custom permissions to an existing user, use <code> <a>UpdateUser</a> </code> instead.</p> <p>A set of custom permissions includes any combination of these restrictions. Currently, you need to create the profile names for custom permission sets by using the QuickSight console. Then, you use the <code>RegisterUser</code> API operation to assign the named set of permissions to a QuickSight user. </p> <p>QuickSight custom permissions are applied through IAM policies. Therefore, they override the permissions typically granted by assigning QuickSight users to one of the default security cohorts in QuickSight (admin, author, reader).</p> <p>This feature is available only to QuickSight Enterprise edition subscriptions that use SAML 2.0-Based Federation for Single Sign-On (SSO).</p>
    #[serde(rename = "CustomPermissionsName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_permissions_name: Option<String>,
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
    /// <p>You need to use this parameter only when you register one or more users using an assumed IAM role. You don't need to provide the session name for other scenarios, for example when you are registering an IAM user or an Amazon QuickSight user. You can register multiple users using the same IAM role if each user has a different session name. For more information on assuming IAM roles, see <a href="https://docs.aws.amazon.com/cli/latest/reference/sts/assume-role.html"> <code>assume-role</code> </a> in the <i>AWS CLI Reference.</i> </p>
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
    /// <p>The user's user name.</p>
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
    /// <p>The catalog associated with a table.</p>
    #[serde(rename = "Catalog")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub catalog: Option<String>,
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
    /// <p>The IAM action to grant or revoke permissions on.</p>
    #[serde(rename = "Actions")]
    pub actions: Vec<String>,
    /// <p><p>The Amazon Resource Name (ARN) of the principal. This can be one of the following:</p> <ul> <li> <p>The ARN of an Amazon QuickSight user or group associated with a data source or dataset. (This is common.)</p> </li> <li> <p>The ARN of an Amazon QuickSight user, group, or namespace associated with an analysis, dashboard, template, or theme. (This is common.)</p> </li> <li> <p>The ARN of an AWS account root: This is an IAM ARN rather than a QuickSight ARN. Use this option only to share resources (templates) across AWS accounts. (This is less common.) </p> </li> </ul></p>
    #[serde(rename = "Principal")]
    pub principal: String,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct RestoreAnalysisRequest {
    /// <p>The ID of the analysis that you're restoring.</p>
    #[serde(rename = "AnalysisId")]
    pub analysis_id: String,
    /// <p>The ID of the AWS account that contains the analysis.</p>
    #[serde(rename = "AwsAccountId")]
    pub aws_account_id: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct RestoreAnalysisResponse {
    /// <p>The ID of the analysis that you're restoring. </p>
    #[serde(rename = "AnalysisId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub analysis_id: Option<String>,
    /// <p>The Amazon Resource Name (ARN) of the analysis that you're restoring.</p>
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
    /// <p>The namespace associated with the row-level permissions dataset.</p>
    #[serde(rename = "Namespace")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
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
pub struct SearchAnalysesRequest {
    /// <p>The ID of the AWS account that contains the analyses that you're searching for.</p>
    #[serde(rename = "AwsAccountId")]
    pub aws_account_id: String,
    /// <p>The structure for the search filters that you want to apply to your search. </p>
    #[serde(rename = "Filters")]
    pub filters: Vec<AnalysisSearchFilter>,
    /// <p>The maximum number of results to return.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>A pagination token that can be used in a subsequent request.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct SearchAnalysesResponse {
    /// <p>Metadata describing the analyses that you searched for.</p>
    #[serde(rename = "AnalysisSummaryList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub analysis_summary_list: Option<Vec<AnalysisSummary>>,
    /// <p>A pagination token that can be used in a subsequent request. </p>
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
pub struct SearchDashboardsRequest {
    /// <p>The ID of the AWS account that contains the user whose dashboards you're searching for. </p>
    #[serde(rename = "AwsAccountId")]
    pub aws_account_id: String,
    /// <p>The filters to apply to the search. Currently, you can search only by user name, for example, <code>"Filters": [ { "Name": "QUICKSIGHT_USER", "Operator": "StringEquals", "Value": "arn:aws:quicksight:us-east-1:1:user/default/UserName1" } ]</code> </p>
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

/// <p>A <i>sheet</i>, which is an object that contains a set of visuals that are viewed together on one page in the Amazon QuickSight console. Every analysis and dashboard contains at least one sheet. Each sheet contains at least one visualization widget, for example a chart, pivot table, or narrative insight. Sheets can be associated with other components, such as controls, filters, and so on.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct Sheet {
    /// <p>The name of a sheet. This name is displayed on the sheet's tab in the QuickSight console.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The unique identifier associated with a sheet.</p>
    #[serde(rename = "SheetId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sheet_id: Option<String>,
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

/// <p>The theme display options for sheets. </p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct SheetStyle {
    /// <p>The display options for tiles.</p>
    #[serde(rename = "Tile")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tile: Option<TileStyle>,
    /// <p>The layout options for tiles.</p>
    #[serde(rename = "TileLayout")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tile_layout: Option<TileLayoutStyle>,
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

/// <p>A string parameter.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct StringParameter {
    /// <p>A display name for a string parameter.</p>
    #[serde(rename = "Name")]
    pub name: String,
    /// <p>The values of a string parameter.</p>
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

/// <p>A template object. A <i>template</i> is an entity in QuickSight that encapsulates the metadata required to create an analysis and that you can use to create a dashboard. A template adds a layer of abstraction by using placeholders to replace the dataset associated with an analysis. You can use templates to create dashboards by replacing dataset placeholders with datasets that follow the same schema that was used to create the source analysis and template.</p> <p>You can share templates across AWS accounts by allowing users in other AWS accounts to create a template or a dashboard from an existing template.</p>
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
    /// <p>Schema of the dataset identified by the placeholder. Any dashboard created from this template should be bound to new datasets matching the same schema described through this API operation.</p>
    #[serde(rename = "DataSetConfigurations")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_set_configurations: Option<Vec<DataSetConfiguration>>,
    /// <p>The description of the template.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>Errors associated with this template version.</p>
    #[serde(rename = "Errors")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub errors: Option<Vec<TemplateError>>,
    /// <p>A list of the associated sheets with the unique identifier and name of each sheet.</p>
    #[serde(rename = "Sheets")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sheets: Option<Vec<Sheet>>,
    /// <p>The Amazon Resource Name (ARN) of an analysis or template that was used to create this template.</p>
    #[serde(rename = "SourceEntityArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_entity_arn: Option<String>,
    /// <p>The HTTP status of the request.</p>
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// <p>The ARN of the theme associated with this version of the template.</p>
    #[serde(rename = "ThemeArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub theme_arn: Option<String>,
    /// <p>The version number of the template version.</p>
    #[serde(rename = "VersionNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version_number: Option<i64>,
}

/// <p>The template version.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct TemplateVersionSummary {
    /// <p>The Amazon Resource Name (ARN) of the template version.</p>
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

/// <p>Summary information about a theme.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct Theme {
    /// <p>The Amazon Resource Name (ARN) of the theme.</p>
    #[serde(rename = "Arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p>The date and time that the theme was created.</p>
    #[serde(rename = "CreatedTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_time: Option<f64>,
    /// <p>The date and time that the theme was last updated.</p>
    #[serde(rename = "LastUpdatedTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_time: Option<f64>,
    /// <p>The name that the user gives to the theme.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The identifier that the user gives to the theme.</p>
    #[serde(rename = "ThemeId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub theme_id: Option<String>,
    /// <p>The type of theme, based on how it was created. Valid values include: <code>QUICKSIGHT</code> and <code>CUSTOM</code>.</p>
    #[serde(rename = "Type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[serde(rename = "Version")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<ThemeVersion>,
}

/// <p>An alias for a theme.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ThemeAlias {
    /// <p>The display name of the theme alias.</p>
    #[serde(rename = "AliasName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alias_name: Option<String>,
    /// <p>The Amazon Resource Name (ARN) of the theme alias.</p>
    #[serde(rename = "Arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p>The version number of the theme alias.</p>
    #[serde(rename = "ThemeVersionNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub theme_version_number: Option<i64>,
}

/// <p>The theme configuration. This configuration contains all of the display properties for a theme.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct ThemeConfiguration {
    /// <p>Color properties that apply to chart data colors.</p>
    #[serde(rename = "DataColorPalette")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_color_palette: Option<DataColorPalette>,
    /// <p>Display options related to sheets.</p>
    #[serde(rename = "Sheet")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sheet: Option<SheetStyle>,
    /// <p>Color properties that apply to the UI and to charts, excluding the colors that apply to data. </p>
    #[serde(rename = "UIColorPalette")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ui_color_palette: Option<UIColorPalette>,
}

/// <p>Theme error.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ThemeError {
    /// <p>The error message.</p>
    #[serde(rename = "Message")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    /// <p>The type of error.</p>
    #[serde(rename = "Type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}

/// <p>The theme summary.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ThemeSummary {
    /// <p>The Amazon Resource Name (ARN) of the resource.</p>
    #[serde(rename = "Arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p>The date and time that this theme was created.</p>
    #[serde(rename = "CreatedTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_time: Option<f64>,
    /// <p>The last date and time that this theme was updated.</p>
    #[serde(rename = "LastUpdatedTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_time: Option<f64>,
    /// <p>The latest version number for the theme. </p>
    #[serde(rename = "LatestVersionNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latest_version_number: Option<i64>,
    /// <p>the display name for the theme.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The ID of the theme. This ID is unique per AWS Region for each AWS account.</p>
    #[serde(rename = "ThemeId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub theme_id: Option<String>,
}

/// <p>A version of a theme.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ThemeVersion {
    /// <p>The Amazon Resource Name (ARN) of the resource.</p>
    #[serde(rename = "Arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p>The Amazon QuickSight-defined ID of the theme that a custom theme inherits from. All themes initially inherit from a default QuickSight theme.</p>
    #[serde(rename = "BaseThemeId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub base_theme_id: Option<String>,
    /// <p>The theme configuration, which contains all the theme display properties.</p>
    #[serde(rename = "Configuration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configuration: Option<ThemeConfiguration>,
    /// <p>The date and time that this theme version was created.</p>
    #[serde(rename = "CreatedTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_time: Option<f64>,
    /// <p>The description of the theme.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>Errors associated with the theme.</p>
    #[serde(rename = "Errors")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub errors: Option<Vec<ThemeError>>,
    /// <p>The status of the theme version.</p>
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// <p>The version number of the theme.</p>
    #[serde(rename = "VersionNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version_number: Option<i64>,
}

/// <p>The theme version.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ThemeVersionSummary {
    /// <p>The Amazon Resource Name (ARN) of the theme version.</p>
    #[serde(rename = "Arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p>The date and time that this theme version was created.</p>
    #[serde(rename = "CreatedTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_time: Option<f64>,
    /// <p>The description of the theme version.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The status of the theme version.</p>
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// <p>The version number of the theme version.</p>
    #[serde(rename = "VersionNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version_number: Option<i64>,
}

/// <p>The display options for the layout of tiles on a sheet.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct TileLayoutStyle {
    /// <p>The gutter settings that apply between tiles. </p>
    #[serde(rename = "Gutter")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gutter: Option<GutterStyle>,
    /// <p>The margin settings that apply around the outside edge of sheets.</p>
    #[serde(rename = "Margin")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub margin: Option<MarginStyle>,
}

/// <p>Display options related to tiles on a sheet.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct TileStyle {
    /// <p>The border around a tile.</p>
    #[serde(rename = "Border")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub border: Option<BorderStyle>,
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

/// <p>The theme colors that apply to UI and to charts, excluding data colors. The colors description is a hexadecimal color code that consists of six alphanumerical characters, prefixed with <code>#</code>, for example #37BFF5. For more information, see <a href="https://docs.aws.amazon.com/quicksight/latest/user/themes-in-quicksight.html">Using Themes in Amazon QuickSight</a> in the <i>Amazon QuickSight User Guide.</i> </p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct UIColorPalette {
    /// <p>This color is that applies to selected states and buttons.</p>
    #[serde(rename = "Accent")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accent: Option<String>,
    /// <p>The foreground color that applies to any text or other elements that appear over the accent color.</p>
    #[serde(rename = "AccentForeground")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accent_foreground: Option<String>,
    /// <p>The color that applies to error messages.</p>
    #[serde(rename = "Danger")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub danger: Option<String>,
    /// <p>The foreground color that applies to any text or other elements that appear over the error color.</p>
    #[serde(rename = "DangerForeground")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub danger_foreground: Option<String>,
    /// <p>The color that applies to the names of fields that are identified as dimensions.</p>
    #[serde(rename = "Dimension")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dimension: Option<String>,
    /// <p>The foreground color that applies to any text or other elements that appear over the dimension color.</p>
    #[serde(rename = "DimensionForeground")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dimension_foreground: Option<String>,
    /// <p>The color that applies to the names of fields that are identified as measures.</p>
    #[serde(rename = "Measure")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub measure: Option<String>,
    /// <p>The foreground color that applies to any text or other elements that appear over the measure color.</p>
    #[serde(rename = "MeasureForeground")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub measure_foreground: Option<String>,
    /// <p>The background color that applies to visuals and other high emphasis UI.</p>
    #[serde(rename = "PrimaryBackground")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub primary_background: Option<String>,
    /// <p>The color of text and other foreground elements that appear over the primary background regions, such as grid lines, borders, table banding, icons, and so on.</p>
    #[serde(rename = "PrimaryForeground")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub primary_foreground: Option<String>,
    /// <p>The background color that applies to the sheet background and sheet controls.</p>
    #[serde(rename = "SecondaryBackground")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secondary_background: Option<String>,
    /// <p>The foreground color that applies to any sheet title, sheet control text, or UI that appears over the secondary background.</p>
    #[serde(rename = "SecondaryForeground")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secondary_foreground: Option<String>,
    /// <p>The color that applies to success messages, for example the check mark for a successful download.</p>
    #[serde(rename = "Success")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub success: Option<String>,
    /// <p>The foreground color that applies to any text or other elements that appear over the success color.</p>
    #[serde(rename = "SuccessForeground")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub success_foreground: Option<String>,
    /// <p>This color that applies to warning and informational messages.</p>
    #[serde(rename = "Warning")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub warning: Option<String>,
    /// <p>The foreground color that applies to any text or other elements that appear over the warning color.</p>
    #[serde(rename = "WarningForeground")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub warning_foreground: Option<String>,
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
pub struct UpdateAccountCustomizationRequest {
    /// <p>The QuickSight customizations you're updating in the current AWS Region. </p>
    #[serde(rename = "AccountCustomization")]
    pub account_customization: AccountCustomization,
    /// <p>The ID for the AWS account that you want to update QuickSight customizations for.</p>
    #[serde(rename = "AwsAccountId")]
    pub aws_account_id: String,
    /// <p>The namespace that you want to update QuickSight customizations for.</p>
    #[serde(rename = "Namespace")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UpdateAccountCustomizationResponse {
    /// <p>The QuickSight customizations you're updating in the current AWS Region. </p>
    #[serde(rename = "AccountCustomization")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_customization: Option<AccountCustomization>,
    /// <p>The Amazon Resource Name (ARN) for the updated customization for this AWS account.</p>
    #[serde(rename = "Arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p>The ID for the AWS account that you want to update QuickSight customizations for.</p>
    #[serde(rename = "AwsAccountId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aws_account_id: Option<String>,
    /// <p>The namespace associated with the customization that you're updating.</p>
    #[serde(rename = "Namespace")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
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
pub struct UpdateAccountSettingsRequest {
    /// <p>The ID for the AWS account that contains the QuickSight settings that you want to list.</p>
    #[serde(rename = "AwsAccountId")]
    pub aws_account_id: String,
    /// <p>The default namespace for this AWS account. Currently, the default is <code>default</code>. AWS Identity and Access Management (IAM) users that register for the first time with QuickSight provide an email that becomes associated with the default namespace.</p>
    #[serde(rename = "DefaultNamespace")]
    pub default_namespace: String,
    /// <p>The email address that you want QuickSight to send notifications to regarding your AWS account or QuickSight subscription.</p>
    #[serde(rename = "NotificationEmail")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notification_email: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UpdateAccountSettingsResponse {
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
pub struct UpdateAnalysisPermissionsRequest {
    /// <p>The ID of the analysis whose permissions you're updating. The ID is part of the analysis URL.</p>
    #[serde(rename = "AnalysisId")]
    pub analysis_id: String,
    /// <p>The ID of the AWS account that contains the analysis whose permissions you're updating. You must be using the AWS account that the analysis is in.</p>
    #[serde(rename = "AwsAccountId")]
    pub aws_account_id: String,
    /// <p>A structure that describes the permissions to add and the principal to add them to.</p>
    #[serde(rename = "GrantPermissions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub grant_permissions: Option<Vec<ResourcePermission>>,
    /// <p>A structure that describes the permissions to remove and the principal to remove them from.</p>
    #[serde(rename = "RevokePermissions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub revoke_permissions: Option<Vec<ResourcePermission>>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UpdateAnalysisPermissionsResponse {
    /// <p>The Amazon Resource Name (ARN) of the analysis that you updated.</p>
    #[serde(rename = "AnalysisArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub analysis_arn: Option<String>,
    /// <p>The ID of the analysis that you updated permissions for.</p>
    #[serde(rename = "AnalysisId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub analysis_id: Option<String>,
    /// <p>A structure that describes the principals and the resource-level permissions on an analysis.</p>
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
pub struct UpdateAnalysisRequest {
    /// <p>The ID for the analysis that you're updating. This ID displays in the URL of the analysis.</p>
    #[serde(rename = "AnalysisId")]
    pub analysis_id: String,
    /// <p>The ID of the AWS account that contains the analysis that you're updating.</p>
    #[serde(rename = "AwsAccountId")]
    pub aws_account_id: String,
    /// <p>A descriptive name for the analysis that you're updating. This name displays for the analysis in the QuickSight console.</p>
    #[serde(rename = "Name")]
    pub name: String,
    /// <p>The parameter names and override values that you want to use. An analysis can have any parameter type, and some parameters might accept multiple values. </p>
    #[serde(rename = "Parameters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameters: Option<Parameters>,
    /// <p>A source entity to use for the analysis that you're updating. This metadata structure contains details that describe a source template and one or more datasets.</p>
    #[serde(rename = "SourceEntity")]
    pub source_entity: AnalysisSourceEntity,
    /// <p>The Amazon Resource Name (ARN) for the theme to apply to the analysis that you're creating. To see the theme in the QuickSight console, make sure that you have access to it.</p>
    #[serde(rename = "ThemeArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub theme_arn: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UpdateAnalysisResponse {
    /// <p>The ID of the analysis.</p>
    #[serde(rename = "AnalysisId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub analysis_id: Option<String>,
    /// <p>The ARN of the analysis that you're updating.</p>
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
    /// <p>The update status of the last update that was made to the analysis.</p>
    #[serde(rename = "UpdateStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_status: Option<String>,
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
    /// <p><p>Options for publishing the dashboard when you create it:</p> <ul> <li> <p> <code>AvailabilityStatus</code> for <code>AdHocFilteringOption</code> - This status can be either <code>ENABLED</code> or <code>DISABLED</code>. When this is set to <code>DISABLED</code>, QuickSight disables the left filter pane on the published dashboard, which can be used for ad hoc (one-time) filtering. This option is <code>ENABLED</code> by default. </p> </li> <li> <p> <code>AvailabilityStatus</code> for <code>ExportToCSVOption</code> - This status can be either <code>ENABLED</code> or <code>DISABLED</code>. The visual option to export data to .CSV format isn&#39;t enabled when this is set to <code>DISABLED</code>. This option is <code>ENABLED</code> by default. </p> </li> <li> <p> <code>VisibilityState</code> for <code>SheetControlsOption</code> - This visibility state can be either <code>COLLAPSED</code> or <code>EXPANDED</code>. This option is <code>COLLAPSED</code> by default. </p> </li> </ul></p>
    #[serde(rename = "DashboardPublishOptions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dashboard_publish_options: Option<DashboardPublishOptions>,
    /// <p>The display name of the dashboard.</p>
    #[serde(rename = "Name")]
    pub name: String,
    /// <p>A structure that contains the parameters of the dashboard. These are parameter overrides for a dashboard. A dashboard can have any type of parameters, and some parameters might accept multiple values.</p>
    #[serde(rename = "Parameters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameters: Option<Parameters>,
    /// <p>The entity that you are using as a source when you update the dashboard. In <code>SourceEntity</code>, you specify the type of object you're using as source. You can only update a dashboard from a template, so you use a <code>SourceTemplate</code> entity. If you need to update a dashboard from an analysis, first convert the analysis to a template by using the <a>CreateTemplate</a> API operation. For <code>SourceTemplate</code>, specify the Amazon Resource Name (ARN) of the source template. The <code>SourceTemplate</code> ARN can contain any AWS Account and any QuickSight-supported AWS Region. </p> <p>Use the <code>DataSetReferences</code> entity within <code>SourceTemplate</code> to list the replacement datasets for the placeholders listed in the original. The schema in each dataset must match its placeholder. </p>
    #[serde(rename = "SourceEntity")]
    pub source_entity: DashboardSourceEntity,
    /// <p>The Amazon Resource Name (ARN) of the theme that is being used for this dashboard. If you add a value for this field, it overrides the value that was originally associated with the entity. The theme ARN must exist in the same AWS account where you create the dashboard.</p>
    #[serde(rename = "ThemeArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub theme_arn: Option<String>,
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
    /// <p>A set of one or more definitions of a <code> <a>ColumnLevelPermissionRule</a> </code>.</p>
    #[serde(rename = "ColumnLevelPermissionRules")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub column_level_permission_rules: Option<Vec<ColumnLevelPermissionRule>>,
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
    /// <p>The name of the assignment, also called a rule. This name must be unique within an AWS account.</p>
    #[serde(rename = "AssignmentName")]
    pub assignment_name: String,
    /// <p><p>The status of the assignment. Possible values are as follows:</p> <ul> <li> <p> <code>ENABLED</code> - Anything specified in this assignment is used when creating the data source.</p> </li> <li> <p> <code>DISABLED</code> - This assignment isn&#39;t used when creating the data source.</p> </li> <li> <p> <code>DRAFT</code> - This assignment is an unfinished draft and isn&#39;t used when creating the data source.</p> </li> </ul></p>
    #[serde(rename = "AssignmentStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assignment_status: Option<String>,
    /// <p>The ID of the AWS account that contains the IAM policy assignment. </p>
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
    /// <p>The name of the assignment or rule.</p>
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
    /// <p>The entity that you are using as a source when you update the template. In <code>SourceEntity</code>, you specify the type of object you're using as source: <code>SourceTemplate</code> for a template or <code>SourceAnalysis</code> for an analysis. Both of these require an Amazon Resource Name (ARN). For <code>SourceTemplate</code>, specify the ARN of the source template. For <code>SourceAnalysis</code>, specify the ARN of the source analysis. The <code>SourceTemplate</code> ARN can contain any AWS Account and any QuickSight-supported AWS Region. </p> <p>Use the <code>DataSetReferences</code> entity within <code>SourceTemplate</code> or <code>SourceAnalysis</code> to list the replacement datasets for the placeholders listed in the original. The schema in each dataset must match its placeholder. </p>
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
pub struct UpdateThemeAliasRequest {
    /// <p>The name of the theme alias that you want to update.</p>
    #[serde(rename = "AliasName")]
    pub alias_name: String,
    /// <p>The ID of the AWS account that contains the theme alias that you're updating.</p>
    #[serde(rename = "AwsAccountId")]
    pub aws_account_id: String,
    /// <p>The ID for the theme.</p>
    #[serde(rename = "ThemeId")]
    pub theme_id: String,
    /// <p>The version number of the theme that the alias should reference.</p>
    #[serde(rename = "ThemeVersionNumber")]
    pub theme_version_number: i64,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UpdateThemeAliasResponse {
    /// <p>The AWS request ID for this operation.</p>
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// <p>The HTTP status of the request.</p>
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<i64>,
    /// <p>Information about the theme alias.</p>
    #[serde(rename = "ThemeAlias")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub theme_alias: Option<ThemeAlias>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UpdateThemePermissionsRequest {
    /// <p>The ID of the AWS account that contains the theme.</p>
    #[serde(rename = "AwsAccountId")]
    pub aws_account_id: String,
    /// <p>A list of resource permissions to be granted for the theme.</p>
    #[serde(rename = "GrantPermissions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub grant_permissions: Option<Vec<ResourcePermission>>,
    /// <p>A list of resource permissions to be revoked from the theme.</p>
    #[serde(rename = "RevokePermissions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub revoke_permissions: Option<Vec<ResourcePermission>>,
    /// <p>The ID for the theme.</p>
    #[serde(rename = "ThemeId")]
    pub theme_id: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UpdateThemePermissionsResponse {
    /// <p>The resulting list of resource permissions for the theme.</p>
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
    /// <p>The Amazon Resource Name (ARN) of the theme.</p>
    #[serde(rename = "ThemeArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub theme_arn: Option<String>,
    /// <p>The ID for the theme.</p>
    #[serde(rename = "ThemeId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub theme_id: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UpdateThemeRequest {
    /// <p>The ID of the AWS account that contains the theme that you're updating.</p>
    #[serde(rename = "AwsAccountId")]
    pub aws_account_id: String,
    /// <p>The theme ID, defined by Amazon QuickSight, that a custom theme inherits from. All themes initially inherit from a default QuickSight theme.</p>
    #[serde(rename = "BaseThemeId")]
    pub base_theme_id: String,
    /// <p>The theme configuration, which contains the theme display properties.</p>
    #[serde(rename = "Configuration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configuration: Option<ThemeConfiguration>,
    /// <p>The name for the theme.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The ID for the theme.</p>
    #[serde(rename = "ThemeId")]
    pub theme_id: String,
    /// <p>A description of the theme version that you're updating Every time that you call <code>UpdateTheme</code>, you create a new version of the theme. Each version of the theme maintains a description of the version in <code>VersionDescription</code>.</p>
    #[serde(rename = "VersionDescription")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version_description: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UpdateThemeResponse {
    /// <p>The Amazon Resource Name (ARN) for the theme.</p>
    #[serde(rename = "Arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p>The creation status of the theme.</p>
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
    /// <p>The ID for the theme.</p>
    #[serde(rename = "ThemeId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub theme_id: Option<String>,
    /// <p>The Amazon Resource Name (ARN) for the new version of the theme.</p>
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
    /// <p>(Enterprise edition only) The name of the custom permissions profile that you want to assign to this user. Customized permissions allows you to control a user's access by restricting access the following operations:</p> <ul> <li> <p>Create and update data sources</p> </li> <li> <p>Create and update datasets</p> </li> <li> <p>Create and update email reports</p> </li> <li> <p>Subscribe to email reports</p> </li> </ul> <p>A set of custom permissions includes any combination of these restrictions. Currently, you need to create the profile names for custom permission sets by using the QuickSight console. Then, you use the <code>RegisterUser</code> API operation to assign the named set of permissions to a QuickSight user. </p> <p>QuickSight custom permissions are applied through IAM policies. Therefore, they override the permissions typically granted by assigning QuickSight users to one of the default security cohorts in QuickSight (admin, author, reader).</p> <p>This feature is available only to QuickSight Enterprise edition subscriptions that use SAML 2.0-Based Federation for Single Sign-On (SSO).</p>
    #[serde(rename = "CustomPermissionsName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_permissions_name: Option<String>,
    /// <p>The email address of the user that you want to update.</p>
    #[serde(rename = "Email")]
    pub email: String,
    /// <p>The namespace. Currently, you should set this to <code>default</code>.</p>
    #[serde(rename = "Namespace")]
    pub namespace: String,
    /// <p>The Amazon QuickSight role of the user. The role can be one of the following default security cohorts:</p> <ul> <li> <p> <code>READER</code>: A user who has read-only access to dashboards.</p> </li> <li> <p> <code>AUTHOR</code>: A user who can create data sources, datasets, analyses, and dashboards.</p> </li> <li> <p> <code>ADMIN</code>: A user who is an author, who can also manage Amazon QuickSight settings.</p> </li> </ul> <p>The name of the QuickSight role is invisible to the user except for the console screens dealing with permissions.</p>
    #[serde(rename = "Role")]
    pub role: String,
    /// <p>A flag that you use to indicate that you want to remove all custom permissions from this user. Using this parameter resets the user to the state it was in before a custom permissions profile was applied. This parameter defaults to NULL and it doesn't accept any other value.</p>
    #[serde(rename = "UnapplyCustomPermissions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unapply_custom_permissions: Option<bool>,
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

/// <p>A registered user of Amazon QuickSight. </p>
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
    /// <p>The custom permissions profile associated with this user.</p>
    #[serde(rename = "CustomPermissionsName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_permissions_name: Option<String>,
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

    fn refine(err: RusotoError<std::convert::Infallible>) -> RusotoError<CancelIngestionError> {
        match err {
            RusotoError::Service(err) => match err {},
            RusotoError::HttpDispatch(err) => RusotoError::HttpDispatch(err),
            RusotoError::Credentials(err) => RusotoError::Credentials(err),
            RusotoError::Validation(err) => RusotoError::Validation(err),
            RusotoError::ParseError(err) => RusotoError::ParseError(err),
            RusotoError::Unknown(res) => Self::from_response(res),
            RusotoError::Blocking => RusotoError::Blocking,
        }
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
/// Errors returned by CreateAccountCustomization
#[derive(Debug, PartialEq)]
pub enum CreateAccountCustomizationError {
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
    /// <p>This resource is currently unavailable.</p>
    ResourceUnavailable(String),
    /// <p>Access is throttled.</p>
    Throttling(String),
}

impl CreateAccountCustomizationError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<CreateAccountCustomizationError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(CreateAccountCustomizationError::AccessDenied(
                        err.msg,
                    ))
                }
                "InternalFailureException" => {
                    return RusotoError::Service(CreateAccountCustomizationError::InternalFailure(
                        err.msg,
                    ))
                }
                "InvalidParameterValueException" => {
                    return RusotoError::Service(
                        CreateAccountCustomizationError::InvalidParameterValue(err.msg),
                    )
                }
                "ResourceExistsException" => {
                    return RusotoError::Service(CreateAccountCustomizationError::ResourceExists(
                        err.msg,
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(CreateAccountCustomizationError::ResourceNotFound(
                        err.msg,
                    ))
                }
                "ResourceUnavailableException" => {
                    return RusotoError::Service(
                        CreateAccountCustomizationError::ResourceUnavailable(err.msg),
                    )
                }
                "ThrottlingException" => {
                    return RusotoError::Service(CreateAccountCustomizationError::Throttling(
                        err.msg,
                    ))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }

    fn refine(
        err: RusotoError<std::convert::Infallible>,
    ) -> RusotoError<CreateAccountCustomizationError> {
        match err {
            RusotoError::Service(err) => match err {},
            RusotoError::HttpDispatch(err) => RusotoError::HttpDispatch(err),
            RusotoError::Credentials(err) => RusotoError::Credentials(err),
            RusotoError::Validation(err) => RusotoError::Validation(err),
            RusotoError::ParseError(err) => RusotoError::ParseError(err),
            RusotoError::Unknown(res) => Self::from_response(res),
            RusotoError::Blocking => RusotoError::Blocking,
        }
    }
}
impl fmt::Display for CreateAccountCustomizationError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreateAccountCustomizationError::AccessDenied(ref cause) => write!(f, "{}", cause),
            CreateAccountCustomizationError::InternalFailure(ref cause) => write!(f, "{}", cause),
            CreateAccountCustomizationError::InvalidParameterValue(ref cause) => {
                write!(f, "{}", cause)
            }
            CreateAccountCustomizationError::ResourceExists(ref cause) => write!(f, "{}", cause),
            CreateAccountCustomizationError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            CreateAccountCustomizationError::ResourceUnavailable(ref cause) => {
                write!(f, "{}", cause)
            }
            CreateAccountCustomizationError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CreateAccountCustomizationError {}
/// Errors returned by CreateAnalysis
#[derive(Debug, PartialEq)]
pub enum CreateAnalysisError {
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

impl CreateAnalysisError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateAnalysisError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "ConflictException" => {
                    return RusotoError::Service(CreateAnalysisError::Conflict(err.msg))
                }
                "InternalFailureException" => {
                    return RusotoError::Service(CreateAnalysisError::InternalFailure(err.msg))
                }
                "InvalidParameterValueException" => {
                    return RusotoError::Service(CreateAnalysisError::InvalidParameterValue(
                        err.msg,
                    ))
                }
                "ResourceExistsException" => {
                    return RusotoError::Service(CreateAnalysisError::ResourceExists(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(CreateAnalysisError::ResourceNotFound(err.msg))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(CreateAnalysisError::Throttling(err.msg))
                }
                "UnsupportedUserEditionException" => {
                    return RusotoError::Service(CreateAnalysisError::UnsupportedUserEdition(
                        err.msg,
                    ))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }

    fn refine(err: RusotoError<std::convert::Infallible>) -> RusotoError<CreateAnalysisError> {
        match err {
            RusotoError::Service(err) => match err {},
            RusotoError::HttpDispatch(err) => RusotoError::HttpDispatch(err),
            RusotoError::Credentials(err) => RusotoError::Credentials(err),
            RusotoError::Validation(err) => RusotoError::Validation(err),
            RusotoError::ParseError(err) => RusotoError::ParseError(err),
            RusotoError::Unknown(res) => Self::from_response(res),
            RusotoError::Blocking => RusotoError::Blocking,
        }
    }
}
impl fmt::Display for CreateAnalysisError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreateAnalysisError::Conflict(ref cause) => write!(f, "{}", cause),
            CreateAnalysisError::InternalFailure(ref cause) => write!(f, "{}", cause),
            CreateAnalysisError::InvalidParameterValue(ref cause) => write!(f, "{}", cause),
            CreateAnalysisError::ResourceExists(ref cause) => write!(f, "{}", cause),
            CreateAnalysisError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            CreateAnalysisError::Throttling(ref cause) => write!(f, "{}", cause),
            CreateAnalysisError::UnsupportedUserEdition(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CreateAnalysisError {}
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

    fn refine(err: RusotoError<std::convert::Infallible>) -> RusotoError<CreateDashboardError> {
        match err {
            RusotoError::Service(err) => match err {},
            RusotoError::HttpDispatch(err) => RusotoError::HttpDispatch(err),
            RusotoError::Credentials(err) => RusotoError::Credentials(err),
            RusotoError::Validation(err) => RusotoError::Validation(err),
            RusotoError::ParseError(err) => RusotoError::ParseError(err),
            RusotoError::Unknown(res) => Self::from_response(res),
            RusotoError::Blocking => RusotoError::Blocking,
        }
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

    fn refine(err: RusotoError<std::convert::Infallible>) -> RusotoError<CreateDataSetError> {
        match err {
            RusotoError::Service(err) => match err {},
            RusotoError::HttpDispatch(err) => RusotoError::HttpDispatch(err),
            RusotoError::Credentials(err) => RusotoError::Credentials(err),
            RusotoError::Validation(err) => RusotoError::Validation(err),
            RusotoError::ParseError(err) => RusotoError::ParseError(err),
            RusotoError::Unknown(res) => Self::from_response(res),
            RusotoError::Blocking => RusotoError::Blocking,
        }
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

    fn refine(err: RusotoError<std::convert::Infallible>) -> RusotoError<CreateDataSourceError> {
        match err {
            RusotoError::Service(err) => match err {},
            RusotoError::HttpDispatch(err) => RusotoError::HttpDispatch(err),
            RusotoError::Credentials(err) => RusotoError::Credentials(err),
            RusotoError::Validation(err) => RusotoError::Validation(err),
            RusotoError::ParseError(err) => RusotoError::ParseError(err),
            RusotoError::Unknown(res) => Self::from_response(res),
            RusotoError::Blocking => RusotoError::Blocking,
        }
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

    fn refine(err: RusotoError<std::convert::Infallible>) -> RusotoError<CreateGroupError> {
        match err {
            RusotoError::Service(err) => match err {},
            RusotoError::HttpDispatch(err) => RusotoError::HttpDispatch(err),
            RusotoError::Credentials(err) => RusotoError::Credentials(err),
            RusotoError::Validation(err) => RusotoError::Validation(err),
            RusotoError::ParseError(err) => RusotoError::ParseError(err),
            RusotoError::Unknown(res) => Self::from_response(res),
            RusotoError::Blocking => RusotoError::Blocking,
        }
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

    fn refine(
        err: RusotoError<std::convert::Infallible>,
    ) -> RusotoError<CreateGroupMembershipError> {
        match err {
            RusotoError::Service(err) => match err {},
            RusotoError::HttpDispatch(err) => RusotoError::HttpDispatch(err),
            RusotoError::Credentials(err) => RusotoError::Credentials(err),
            RusotoError::Validation(err) => RusotoError::Validation(err),
            RusotoError::ParseError(err) => RusotoError::ParseError(err),
            RusotoError::Unknown(res) => Self::from_response(res),
            RusotoError::Blocking => RusotoError::Blocking,
        }
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
    /// <p>A resource is already in a state that indicates an operation is happening that must complete before a new update can be applied.</p>
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

    fn refine(
        err: RusotoError<std::convert::Infallible>,
    ) -> RusotoError<CreateIAMPolicyAssignmentError> {
        match err {
            RusotoError::Service(err) => match err {},
            RusotoError::HttpDispatch(err) => RusotoError::HttpDispatch(err),
            RusotoError::Credentials(err) => RusotoError::Credentials(err),
            RusotoError::Validation(err) => RusotoError::Validation(err),
            RusotoError::ParseError(err) => RusotoError::ParseError(err),
            RusotoError::Unknown(res) => Self::from_response(res),
            RusotoError::Blocking => RusotoError::Blocking,
        }
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

    fn refine(err: RusotoError<std::convert::Infallible>) -> RusotoError<CreateIngestionError> {
        match err {
            RusotoError::Service(err) => match err {},
            RusotoError::HttpDispatch(err) => RusotoError::HttpDispatch(err),
            RusotoError::Credentials(err) => RusotoError::Credentials(err),
            RusotoError::Validation(err) => RusotoError::Validation(err),
            RusotoError::ParseError(err) => RusotoError::ParseError(err),
            RusotoError::Unknown(res) => Self::from_response(res),
            RusotoError::Blocking => RusotoError::Blocking,
        }
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
/// Errors returned by CreateNamespace
#[derive(Debug, PartialEq)]
pub enum CreateNamespaceError {
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

impl CreateNamespaceError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateNamespaceError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(CreateNamespaceError::AccessDenied(err.msg))
                }
                "ConflictException" => {
                    return RusotoError::Service(CreateNamespaceError::Conflict(err.msg))
                }
                "InternalFailureException" => {
                    return RusotoError::Service(CreateNamespaceError::InternalFailure(err.msg))
                }
                "InvalidParameterValueException" => {
                    return RusotoError::Service(CreateNamespaceError::InvalidParameterValue(
                        err.msg,
                    ))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(CreateNamespaceError::LimitExceeded(err.msg))
                }
                "PreconditionNotMetException" => {
                    return RusotoError::Service(CreateNamespaceError::PreconditionNotMet(err.msg))
                }
                "ResourceExistsException" => {
                    return RusotoError::Service(CreateNamespaceError::ResourceExists(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(CreateNamespaceError::ResourceNotFound(err.msg))
                }
                "ResourceUnavailableException" => {
                    return RusotoError::Service(CreateNamespaceError::ResourceUnavailable(err.msg))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(CreateNamespaceError::Throttling(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }

    fn refine(err: RusotoError<std::convert::Infallible>) -> RusotoError<CreateNamespaceError> {
        match err {
            RusotoError::Service(err) => match err {},
            RusotoError::HttpDispatch(err) => RusotoError::HttpDispatch(err),
            RusotoError::Credentials(err) => RusotoError::Credentials(err),
            RusotoError::Validation(err) => RusotoError::Validation(err),
            RusotoError::ParseError(err) => RusotoError::ParseError(err),
            RusotoError::Unknown(res) => Self::from_response(res),
            RusotoError::Blocking => RusotoError::Blocking,
        }
    }
}
impl fmt::Display for CreateNamespaceError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreateNamespaceError::AccessDenied(ref cause) => write!(f, "{}", cause),
            CreateNamespaceError::Conflict(ref cause) => write!(f, "{}", cause),
            CreateNamespaceError::InternalFailure(ref cause) => write!(f, "{}", cause),
            CreateNamespaceError::InvalidParameterValue(ref cause) => write!(f, "{}", cause),
            CreateNamespaceError::LimitExceeded(ref cause) => write!(f, "{}", cause),
            CreateNamespaceError::PreconditionNotMet(ref cause) => write!(f, "{}", cause),
            CreateNamespaceError::ResourceExists(ref cause) => write!(f, "{}", cause),
            CreateNamespaceError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            CreateNamespaceError::ResourceUnavailable(ref cause) => write!(f, "{}", cause),
            CreateNamespaceError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CreateNamespaceError {}
/// Errors returned by CreateTemplate
#[derive(Debug, PartialEq)]
pub enum CreateTemplateError {
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

impl CreateTemplateError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateTemplateError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(CreateTemplateError::AccessDenied(err.msg))
                }
                "ConflictException" => {
                    return RusotoError::Service(CreateTemplateError::Conflict(err.msg))
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

    fn refine(err: RusotoError<std::convert::Infallible>) -> RusotoError<CreateTemplateError> {
        match err {
            RusotoError::Service(err) => match err {},
            RusotoError::HttpDispatch(err) => RusotoError::HttpDispatch(err),
            RusotoError::Credentials(err) => RusotoError::Credentials(err),
            RusotoError::Validation(err) => RusotoError::Validation(err),
            RusotoError::ParseError(err) => RusotoError::ParseError(err),
            RusotoError::Unknown(res) => Self::from_response(res),
            RusotoError::Blocking => RusotoError::Blocking,
        }
    }
}
impl fmt::Display for CreateTemplateError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreateTemplateError::AccessDenied(ref cause) => write!(f, "{}", cause),
            CreateTemplateError::Conflict(ref cause) => write!(f, "{}", cause),
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

    fn refine(err: RusotoError<std::convert::Infallible>) -> RusotoError<CreateTemplateAliasError> {
        match err {
            RusotoError::Service(err) => match err {},
            RusotoError::HttpDispatch(err) => RusotoError::HttpDispatch(err),
            RusotoError::Credentials(err) => RusotoError::Credentials(err),
            RusotoError::Validation(err) => RusotoError::Validation(err),
            RusotoError::ParseError(err) => RusotoError::ParseError(err),
            RusotoError::Unknown(res) => Self::from_response(res),
            RusotoError::Blocking => RusotoError::Blocking,
        }
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
/// Errors returned by CreateTheme
#[derive(Debug, PartialEq)]
pub enum CreateThemeError {
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

impl CreateThemeError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateThemeError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(CreateThemeError::AccessDenied(err.msg))
                }
                "InternalFailureException" => {
                    return RusotoError::Service(CreateThemeError::InternalFailure(err.msg))
                }
                "InvalidParameterValueException" => {
                    return RusotoError::Service(CreateThemeError::InvalidParameterValue(err.msg))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(CreateThemeError::LimitExceeded(err.msg))
                }
                "ResourceExistsException" => {
                    return RusotoError::Service(CreateThemeError::ResourceExists(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(CreateThemeError::ResourceNotFound(err.msg))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(CreateThemeError::Throttling(err.msg))
                }
                "UnsupportedUserEditionException" => {
                    return RusotoError::Service(CreateThemeError::UnsupportedUserEdition(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }

    fn refine(err: RusotoError<std::convert::Infallible>) -> RusotoError<CreateThemeError> {
        match err {
            RusotoError::Service(err) => match err {},
            RusotoError::HttpDispatch(err) => RusotoError::HttpDispatch(err),
            RusotoError::Credentials(err) => RusotoError::Credentials(err),
            RusotoError::Validation(err) => RusotoError::Validation(err),
            RusotoError::ParseError(err) => RusotoError::ParseError(err),
            RusotoError::Unknown(res) => Self::from_response(res),
            RusotoError::Blocking => RusotoError::Blocking,
        }
    }
}
impl fmt::Display for CreateThemeError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreateThemeError::AccessDenied(ref cause) => write!(f, "{}", cause),
            CreateThemeError::InternalFailure(ref cause) => write!(f, "{}", cause),
            CreateThemeError::InvalidParameterValue(ref cause) => write!(f, "{}", cause),
            CreateThemeError::LimitExceeded(ref cause) => write!(f, "{}", cause),
            CreateThemeError::ResourceExists(ref cause) => write!(f, "{}", cause),
            CreateThemeError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            CreateThemeError::Throttling(ref cause) => write!(f, "{}", cause),
            CreateThemeError::UnsupportedUserEdition(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CreateThemeError {}
/// Errors returned by CreateThemeAlias
#[derive(Debug, PartialEq)]
pub enum CreateThemeAliasError {
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

impl CreateThemeAliasError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateThemeAliasError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "ConflictException" => {
                    return RusotoError::Service(CreateThemeAliasError::Conflict(err.msg))
                }
                "InternalFailureException" => {
                    return RusotoError::Service(CreateThemeAliasError::InternalFailure(err.msg))
                }
                "InvalidParameterValueException" => {
                    return RusotoError::Service(CreateThemeAliasError::InvalidParameterValue(
                        err.msg,
                    ))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(CreateThemeAliasError::LimitExceeded(err.msg))
                }
                "ResourceExistsException" => {
                    return RusotoError::Service(CreateThemeAliasError::ResourceExists(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(CreateThemeAliasError::ResourceNotFound(err.msg))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(CreateThemeAliasError::Throttling(err.msg))
                }
                "UnsupportedUserEditionException" => {
                    return RusotoError::Service(CreateThemeAliasError::UnsupportedUserEdition(
                        err.msg,
                    ))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }

    fn refine(err: RusotoError<std::convert::Infallible>) -> RusotoError<CreateThemeAliasError> {
        match err {
            RusotoError::Service(err) => match err {},
            RusotoError::HttpDispatch(err) => RusotoError::HttpDispatch(err),
            RusotoError::Credentials(err) => RusotoError::Credentials(err),
            RusotoError::Validation(err) => RusotoError::Validation(err),
            RusotoError::ParseError(err) => RusotoError::ParseError(err),
            RusotoError::Unknown(res) => Self::from_response(res),
            RusotoError::Blocking => RusotoError::Blocking,
        }
    }
}
impl fmt::Display for CreateThemeAliasError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreateThemeAliasError::Conflict(ref cause) => write!(f, "{}", cause),
            CreateThemeAliasError::InternalFailure(ref cause) => write!(f, "{}", cause),
            CreateThemeAliasError::InvalidParameterValue(ref cause) => write!(f, "{}", cause),
            CreateThemeAliasError::LimitExceeded(ref cause) => write!(f, "{}", cause),
            CreateThemeAliasError::ResourceExists(ref cause) => write!(f, "{}", cause),
            CreateThemeAliasError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            CreateThemeAliasError::Throttling(ref cause) => write!(f, "{}", cause),
            CreateThemeAliasError::UnsupportedUserEdition(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CreateThemeAliasError {}
/// Errors returned by DeleteAccountCustomization
#[derive(Debug, PartialEq)]
pub enum DeleteAccountCustomizationError {
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

impl DeleteAccountCustomizationError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<DeleteAccountCustomizationError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(DeleteAccountCustomizationError::AccessDenied(
                        err.msg,
                    ))
                }
                "InternalFailureException" => {
                    return RusotoError::Service(DeleteAccountCustomizationError::InternalFailure(
                        err.msg,
                    ))
                }
                "InvalidParameterValueException" => {
                    return RusotoError::Service(
                        DeleteAccountCustomizationError::InvalidParameterValue(err.msg),
                    )
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(DeleteAccountCustomizationError::ResourceNotFound(
                        err.msg,
                    ))
                }
                "ResourceUnavailableException" => {
                    return RusotoError::Service(
                        DeleteAccountCustomizationError::ResourceUnavailable(err.msg),
                    )
                }
                "ThrottlingException" => {
                    return RusotoError::Service(DeleteAccountCustomizationError::Throttling(
                        err.msg,
                    ))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }

    fn refine(
        err: RusotoError<std::convert::Infallible>,
    ) -> RusotoError<DeleteAccountCustomizationError> {
        match err {
            RusotoError::Service(err) => match err {},
            RusotoError::HttpDispatch(err) => RusotoError::HttpDispatch(err),
            RusotoError::Credentials(err) => RusotoError::Credentials(err),
            RusotoError::Validation(err) => RusotoError::Validation(err),
            RusotoError::ParseError(err) => RusotoError::ParseError(err),
            RusotoError::Unknown(res) => Self::from_response(res),
            RusotoError::Blocking => RusotoError::Blocking,
        }
    }
}
impl fmt::Display for DeleteAccountCustomizationError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteAccountCustomizationError::AccessDenied(ref cause) => write!(f, "{}", cause),
            DeleteAccountCustomizationError::InternalFailure(ref cause) => write!(f, "{}", cause),
            DeleteAccountCustomizationError::InvalidParameterValue(ref cause) => {
                write!(f, "{}", cause)
            }
            DeleteAccountCustomizationError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            DeleteAccountCustomizationError::ResourceUnavailable(ref cause) => {
                write!(f, "{}", cause)
            }
            DeleteAccountCustomizationError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeleteAccountCustomizationError {}
/// Errors returned by DeleteAnalysis
#[derive(Debug, PartialEq)]
pub enum DeleteAnalysisError {
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

impl DeleteAnalysisError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteAnalysisError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "ConflictException" => {
                    return RusotoError::Service(DeleteAnalysisError::Conflict(err.msg))
                }
                "InternalFailureException" => {
                    return RusotoError::Service(DeleteAnalysisError::InternalFailure(err.msg))
                }
                "InvalidParameterValueException" => {
                    return RusotoError::Service(DeleteAnalysisError::InvalidParameterValue(
                        err.msg,
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(DeleteAnalysisError::ResourceNotFound(err.msg))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(DeleteAnalysisError::Throttling(err.msg))
                }
                "UnsupportedUserEditionException" => {
                    return RusotoError::Service(DeleteAnalysisError::UnsupportedUserEdition(
                        err.msg,
                    ))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }

    fn refine(err: RusotoError<std::convert::Infallible>) -> RusotoError<DeleteAnalysisError> {
        match err {
            RusotoError::Service(err) => match err {},
            RusotoError::HttpDispatch(err) => RusotoError::HttpDispatch(err),
            RusotoError::Credentials(err) => RusotoError::Credentials(err),
            RusotoError::Validation(err) => RusotoError::Validation(err),
            RusotoError::ParseError(err) => RusotoError::ParseError(err),
            RusotoError::Unknown(res) => Self::from_response(res),
            RusotoError::Blocking => RusotoError::Blocking,
        }
    }
}
impl fmt::Display for DeleteAnalysisError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteAnalysisError::Conflict(ref cause) => write!(f, "{}", cause),
            DeleteAnalysisError::InternalFailure(ref cause) => write!(f, "{}", cause),
            DeleteAnalysisError::InvalidParameterValue(ref cause) => write!(f, "{}", cause),
            DeleteAnalysisError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            DeleteAnalysisError::Throttling(ref cause) => write!(f, "{}", cause),
            DeleteAnalysisError::UnsupportedUserEdition(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeleteAnalysisError {}
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

    fn refine(err: RusotoError<std::convert::Infallible>) -> RusotoError<DeleteDashboardError> {
        match err {
            RusotoError::Service(err) => match err {},
            RusotoError::HttpDispatch(err) => RusotoError::HttpDispatch(err),
            RusotoError::Credentials(err) => RusotoError::Credentials(err),
            RusotoError::Validation(err) => RusotoError::Validation(err),
            RusotoError::ParseError(err) => RusotoError::ParseError(err),
            RusotoError::Unknown(res) => Self::from_response(res),
            RusotoError::Blocking => RusotoError::Blocking,
        }
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

    fn refine(err: RusotoError<std::convert::Infallible>) -> RusotoError<DeleteDataSetError> {
        match err {
            RusotoError::Service(err) => match err {},
            RusotoError::HttpDispatch(err) => RusotoError::HttpDispatch(err),
            RusotoError::Credentials(err) => RusotoError::Credentials(err),
            RusotoError::Validation(err) => RusotoError::Validation(err),
            RusotoError::ParseError(err) => RusotoError::ParseError(err),
            RusotoError::Unknown(res) => Self::from_response(res),
            RusotoError::Blocking => RusotoError::Blocking,
        }
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

    fn refine(err: RusotoError<std::convert::Infallible>) -> RusotoError<DeleteDataSourceError> {
        match err {
            RusotoError::Service(err) => match err {},
            RusotoError::HttpDispatch(err) => RusotoError::HttpDispatch(err),
            RusotoError::Credentials(err) => RusotoError::Credentials(err),
            RusotoError::Validation(err) => RusotoError::Validation(err),
            RusotoError::ParseError(err) => RusotoError::ParseError(err),
            RusotoError::Unknown(res) => Self::from_response(res),
            RusotoError::Blocking => RusotoError::Blocking,
        }
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

    fn refine(err: RusotoError<std::convert::Infallible>) -> RusotoError<DeleteGroupError> {
        match err {
            RusotoError::Service(err) => match err {},
            RusotoError::HttpDispatch(err) => RusotoError::HttpDispatch(err),
            RusotoError::Credentials(err) => RusotoError::Credentials(err),
            RusotoError::Validation(err) => RusotoError::Validation(err),
            RusotoError::ParseError(err) => RusotoError::ParseError(err),
            RusotoError::Unknown(res) => Self::from_response(res),
            RusotoError::Blocking => RusotoError::Blocking,
        }
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

    fn refine(
        err: RusotoError<std::convert::Infallible>,
    ) -> RusotoError<DeleteGroupMembershipError> {
        match err {
            RusotoError::Service(err) => match err {},
            RusotoError::HttpDispatch(err) => RusotoError::HttpDispatch(err),
            RusotoError::Credentials(err) => RusotoError::Credentials(err),
            RusotoError::Validation(err) => RusotoError::Validation(err),
            RusotoError::ParseError(err) => RusotoError::ParseError(err),
            RusotoError::Unknown(res) => Self::from_response(res),
            RusotoError::Blocking => RusotoError::Blocking,
        }
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
    /// <p>A resource is already in a state that indicates an operation is happening that must complete before a new update can be applied.</p>
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

    fn refine(
        err: RusotoError<std::convert::Infallible>,
    ) -> RusotoError<DeleteIAMPolicyAssignmentError> {
        match err {
            RusotoError::Service(err) => match err {},
            RusotoError::HttpDispatch(err) => RusotoError::HttpDispatch(err),
            RusotoError::Credentials(err) => RusotoError::Credentials(err),
            RusotoError::Validation(err) => RusotoError::Validation(err),
            RusotoError::ParseError(err) => RusotoError::ParseError(err),
            RusotoError::Unknown(res) => Self::from_response(res),
            RusotoError::Blocking => RusotoError::Blocking,
        }
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
/// Errors returned by DeleteNamespace
#[derive(Debug, PartialEq)]
pub enum DeleteNamespaceError {
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

impl DeleteNamespaceError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteNamespaceError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(DeleteNamespaceError::AccessDenied(err.msg))
                }
                "InternalFailureException" => {
                    return RusotoError::Service(DeleteNamespaceError::InternalFailure(err.msg))
                }
                "InvalidParameterValueException" => {
                    return RusotoError::Service(DeleteNamespaceError::InvalidParameterValue(
                        err.msg,
                    ))
                }
                "PreconditionNotMetException" => {
                    return RusotoError::Service(DeleteNamespaceError::PreconditionNotMet(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(DeleteNamespaceError::ResourceNotFound(err.msg))
                }
                "ResourceUnavailableException" => {
                    return RusotoError::Service(DeleteNamespaceError::ResourceUnavailable(err.msg))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(DeleteNamespaceError::Throttling(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }

    fn refine(err: RusotoError<std::convert::Infallible>) -> RusotoError<DeleteNamespaceError> {
        match err {
            RusotoError::Service(err) => match err {},
            RusotoError::HttpDispatch(err) => RusotoError::HttpDispatch(err),
            RusotoError::Credentials(err) => RusotoError::Credentials(err),
            RusotoError::Validation(err) => RusotoError::Validation(err),
            RusotoError::ParseError(err) => RusotoError::ParseError(err),
            RusotoError::Unknown(res) => Self::from_response(res),
            RusotoError::Blocking => RusotoError::Blocking,
        }
    }
}
impl fmt::Display for DeleteNamespaceError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteNamespaceError::AccessDenied(ref cause) => write!(f, "{}", cause),
            DeleteNamespaceError::InternalFailure(ref cause) => write!(f, "{}", cause),
            DeleteNamespaceError::InvalidParameterValue(ref cause) => write!(f, "{}", cause),
            DeleteNamespaceError::PreconditionNotMet(ref cause) => write!(f, "{}", cause),
            DeleteNamespaceError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            DeleteNamespaceError::ResourceUnavailable(ref cause) => write!(f, "{}", cause),
            DeleteNamespaceError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeleteNamespaceError {}
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

    fn refine(err: RusotoError<std::convert::Infallible>) -> RusotoError<DeleteTemplateError> {
        match err {
            RusotoError::Service(err) => match err {},
            RusotoError::HttpDispatch(err) => RusotoError::HttpDispatch(err),
            RusotoError::Credentials(err) => RusotoError::Credentials(err),
            RusotoError::Validation(err) => RusotoError::Validation(err),
            RusotoError::ParseError(err) => RusotoError::ParseError(err),
            RusotoError::Unknown(res) => Self::from_response(res),
            RusotoError::Blocking => RusotoError::Blocking,
        }
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

impl DeleteTemplateAliasError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteTemplateAliasError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "ConflictException" => {
                    return RusotoError::Service(DeleteTemplateAliasError::Conflict(err.msg))
                }
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

    fn refine(err: RusotoError<std::convert::Infallible>) -> RusotoError<DeleteTemplateAliasError> {
        match err {
            RusotoError::Service(err) => match err {},
            RusotoError::HttpDispatch(err) => RusotoError::HttpDispatch(err),
            RusotoError::Credentials(err) => RusotoError::Credentials(err),
            RusotoError::Validation(err) => RusotoError::Validation(err),
            RusotoError::ParseError(err) => RusotoError::ParseError(err),
            RusotoError::Unknown(res) => Self::from_response(res),
            RusotoError::Blocking => RusotoError::Blocking,
        }
    }
}
impl fmt::Display for DeleteTemplateAliasError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteTemplateAliasError::Conflict(ref cause) => write!(f, "{}", cause),
            DeleteTemplateAliasError::InternalFailure(ref cause) => write!(f, "{}", cause),
            DeleteTemplateAliasError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            DeleteTemplateAliasError::Throttling(ref cause) => write!(f, "{}", cause),
            DeleteTemplateAliasError::UnsupportedUserEdition(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeleteTemplateAliasError {}
/// Errors returned by DeleteTheme
#[derive(Debug, PartialEq)]
pub enum DeleteThemeError {
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
    /// <p>This error indicates that you are calling an operation on an Amazon QuickSight subscription where the edition doesn't include support for that operation. Amazon QuickSight currently has Standard Edition and Enterprise Edition. Not every operation and capability is available in every edition.</p>
    UnsupportedUserEdition(String),
}

impl DeleteThemeError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteThemeError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(DeleteThemeError::AccessDenied(err.msg))
                }
                "ConflictException" => {
                    return RusotoError::Service(DeleteThemeError::Conflict(err.msg))
                }
                "InternalFailureException" => {
                    return RusotoError::Service(DeleteThemeError::InternalFailure(err.msg))
                }
                "InvalidParameterValueException" => {
                    return RusotoError::Service(DeleteThemeError::InvalidParameterValue(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(DeleteThemeError::ResourceNotFound(err.msg))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(DeleteThemeError::Throttling(err.msg))
                }
                "UnsupportedUserEditionException" => {
                    return RusotoError::Service(DeleteThemeError::UnsupportedUserEdition(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }

    fn refine(err: RusotoError<std::convert::Infallible>) -> RusotoError<DeleteThemeError> {
        match err {
            RusotoError::Service(err) => match err {},
            RusotoError::HttpDispatch(err) => RusotoError::HttpDispatch(err),
            RusotoError::Credentials(err) => RusotoError::Credentials(err),
            RusotoError::Validation(err) => RusotoError::Validation(err),
            RusotoError::ParseError(err) => RusotoError::ParseError(err),
            RusotoError::Unknown(res) => Self::from_response(res),
            RusotoError::Blocking => RusotoError::Blocking,
        }
    }
}
impl fmt::Display for DeleteThemeError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteThemeError::AccessDenied(ref cause) => write!(f, "{}", cause),
            DeleteThemeError::Conflict(ref cause) => write!(f, "{}", cause),
            DeleteThemeError::InternalFailure(ref cause) => write!(f, "{}", cause),
            DeleteThemeError::InvalidParameterValue(ref cause) => write!(f, "{}", cause),
            DeleteThemeError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            DeleteThemeError::Throttling(ref cause) => write!(f, "{}", cause),
            DeleteThemeError::UnsupportedUserEdition(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeleteThemeError {}
/// Errors returned by DeleteThemeAlias
#[derive(Debug, PartialEq)]
pub enum DeleteThemeAliasError {
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

impl DeleteThemeAliasError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteThemeAliasError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "ConflictException" => {
                    return RusotoError::Service(DeleteThemeAliasError::Conflict(err.msg))
                }
                "InternalFailureException" => {
                    return RusotoError::Service(DeleteThemeAliasError::InternalFailure(err.msg))
                }
                "InvalidParameterValueException" => {
                    return RusotoError::Service(DeleteThemeAliasError::InvalidParameterValue(
                        err.msg,
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(DeleteThemeAliasError::ResourceNotFound(err.msg))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(DeleteThemeAliasError::Throttling(err.msg))
                }
                "UnsupportedUserEditionException" => {
                    return RusotoError::Service(DeleteThemeAliasError::UnsupportedUserEdition(
                        err.msg,
                    ))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }

    fn refine(err: RusotoError<std::convert::Infallible>) -> RusotoError<DeleteThemeAliasError> {
        match err {
            RusotoError::Service(err) => match err {},
            RusotoError::HttpDispatch(err) => RusotoError::HttpDispatch(err),
            RusotoError::Credentials(err) => RusotoError::Credentials(err),
            RusotoError::Validation(err) => RusotoError::Validation(err),
            RusotoError::ParseError(err) => RusotoError::ParseError(err),
            RusotoError::Unknown(res) => Self::from_response(res),
            RusotoError::Blocking => RusotoError::Blocking,
        }
    }
}
impl fmt::Display for DeleteThemeAliasError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteThemeAliasError::Conflict(ref cause) => write!(f, "{}", cause),
            DeleteThemeAliasError::InternalFailure(ref cause) => write!(f, "{}", cause),
            DeleteThemeAliasError::InvalidParameterValue(ref cause) => write!(f, "{}", cause),
            DeleteThemeAliasError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            DeleteThemeAliasError::Throttling(ref cause) => write!(f, "{}", cause),
            DeleteThemeAliasError::UnsupportedUserEdition(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeleteThemeAliasError {}
/// Errors returned by DeleteUser
#[derive(Debug, PartialEq)]
pub enum DeleteUserError {
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
                "PreconditionNotMetException" => {
                    return RusotoError::Service(DeleteUserError::PreconditionNotMet(err.msg))
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

    fn refine(err: RusotoError<std::convert::Infallible>) -> RusotoError<DeleteUserError> {
        match err {
            RusotoError::Service(err) => match err {},
            RusotoError::HttpDispatch(err) => RusotoError::HttpDispatch(err),
            RusotoError::Credentials(err) => RusotoError::Credentials(err),
            RusotoError::Validation(err) => RusotoError::Validation(err),
            RusotoError::ParseError(err) => RusotoError::ParseError(err),
            RusotoError::Unknown(res) => Self::from_response(res),
            RusotoError::Blocking => RusotoError::Blocking,
        }
    }
}
impl fmt::Display for DeleteUserError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteUserError::AccessDenied(ref cause) => write!(f, "{}", cause),
            DeleteUserError::InternalFailure(ref cause) => write!(f, "{}", cause),
            DeleteUserError::InvalidParameterValue(ref cause) => write!(f, "{}", cause),
            DeleteUserError::PreconditionNotMet(ref cause) => write!(f, "{}", cause),
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
    /// <p>One or more preconditions aren't met.</p>
    PreconditionNotMet(String),
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
                "PreconditionNotMetException" => {
                    return RusotoError::Service(DeleteUserByPrincipalIdError::PreconditionNotMet(
                        err.msg,
                    ))
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

    fn refine(
        err: RusotoError<std::convert::Infallible>,
    ) -> RusotoError<DeleteUserByPrincipalIdError> {
        match err {
            RusotoError::Service(err) => match err {},
            RusotoError::HttpDispatch(err) => RusotoError::HttpDispatch(err),
            RusotoError::Credentials(err) => RusotoError::Credentials(err),
            RusotoError::Validation(err) => RusotoError::Validation(err),
            RusotoError::ParseError(err) => RusotoError::ParseError(err),
            RusotoError::Unknown(res) => Self::from_response(res),
            RusotoError::Blocking => RusotoError::Blocking,
        }
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
            DeleteUserByPrincipalIdError::PreconditionNotMet(ref cause) => write!(f, "{}", cause),
            DeleteUserByPrincipalIdError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            DeleteUserByPrincipalIdError::ResourceUnavailable(ref cause) => write!(f, "{}", cause),
            DeleteUserByPrincipalIdError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeleteUserByPrincipalIdError {}
/// Errors returned by DescribeAccountCustomization
#[derive(Debug, PartialEq)]
pub enum DescribeAccountCustomizationError {
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

impl DescribeAccountCustomizationError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<DescribeAccountCustomizationError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(DescribeAccountCustomizationError::AccessDenied(
                        err.msg,
                    ))
                }
                "InternalFailureException" => {
                    return RusotoError::Service(
                        DescribeAccountCustomizationError::InternalFailure(err.msg),
                    )
                }
                "InvalidParameterValueException" => {
                    return RusotoError::Service(
                        DescribeAccountCustomizationError::InvalidParameterValue(err.msg),
                    )
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(
                        DescribeAccountCustomizationError::ResourceNotFound(err.msg),
                    )
                }
                "ResourceUnavailableException" => {
                    return RusotoError::Service(
                        DescribeAccountCustomizationError::ResourceUnavailable(err.msg),
                    )
                }
                "ThrottlingException" => {
                    return RusotoError::Service(DescribeAccountCustomizationError::Throttling(
                        err.msg,
                    ))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }

    fn refine(
        err: RusotoError<std::convert::Infallible>,
    ) -> RusotoError<DescribeAccountCustomizationError> {
        match err {
            RusotoError::Service(err) => match err {},
            RusotoError::HttpDispatch(err) => RusotoError::HttpDispatch(err),
            RusotoError::Credentials(err) => RusotoError::Credentials(err),
            RusotoError::Validation(err) => RusotoError::Validation(err),
            RusotoError::ParseError(err) => RusotoError::ParseError(err),
            RusotoError::Unknown(res) => Self::from_response(res),
            RusotoError::Blocking => RusotoError::Blocking,
        }
    }
}
impl fmt::Display for DescribeAccountCustomizationError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeAccountCustomizationError::AccessDenied(ref cause) => write!(f, "{}", cause),
            DescribeAccountCustomizationError::InternalFailure(ref cause) => write!(f, "{}", cause),
            DescribeAccountCustomizationError::InvalidParameterValue(ref cause) => {
                write!(f, "{}", cause)
            }
            DescribeAccountCustomizationError::ResourceNotFound(ref cause) => {
                write!(f, "{}", cause)
            }
            DescribeAccountCustomizationError::ResourceUnavailable(ref cause) => {
                write!(f, "{}", cause)
            }
            DescribeAccountCustomizationError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DescribeAccountCustomizationError {}
/// Errors returned by DescribeAccountSettings
#[derive(Debug, PartialEq)]
pub enum DescribeAccountSettingsError {
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

impl DescribeAccountSettingsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeAccountSettingsError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(DescribeAccountSettingsError::AccessDenied(
                        err.msg,
                    ))
                }
                "InternalFailureException" => {
                    return RusotoError::Service(DescribeAccountSettingsError::InternalFailure(
                        err.msg,
                    ))
                }
                "InvalidParameterValueException" => {
                    return RusotoError::Service(
                        DescribeAccountSettingsError::InvalidParameterValue(err.msg),
                    )
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(DescribeAccountSettingsError::ResourceNotFound(
                        err.msg,
                    ))
                }
                "ResourceUnavailableException" => {
                    return RusotoError::Service(DescribeAccountSettingsError::ResourceUnavailable(
                        err.msg,
                    ))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(DescribeAccountSettingsError::Throttling(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }

    fn refine(
        err: RusotoError<std::convert::Infallible>,
    ) -> RusotoError<DescribeAccountSettingsError> {
        match err {
            RusotoError::Service(err) => match err {},
            RusotoError::HttpDispatch(err) => RusotoError::HttpDispatch(err),
            RusotoError::Credentials(err) => RusotoError::Credentials(err),
            RusotoError::Validation(err) => RusotoError::Validation(err),
            RusotoError::ParseError(err) => RusotoError::ParseError(err),
            RusotoError::Unknown(res) => Self::from_response(res),
            RusotoError::Blocking => RusotoError::Blocking,
        }
    }
}
impl fmt::Display for DescribeAccountSettingsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeAccountSettingsError::AccessDenied(ref cause) => write!(f, "{}", cause),
            DescribeAccountSettingsError::InternalFailure(ref cause) => write!(f, "{}", cause),
            DescribeAccountSettingsError::InvalidParameterValue(ref cause) => {
                write!(f, "{}", cause)
            }
            DescribeAccountSettingsError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            DescribeAccountSettingsError::ResourceUnavailable(ref cause) => write!(f, "{}", cause),
            DescribeAccountSettingsError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DescribeAccountSettingsError {}
/// Errors returned by DescribeAnalysis
#[derive(Debug, PartialEq)]
pub enum DescribeAnalysisError {
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

impl DescribeAnalysisError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeAnalysisError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(DescribeAnalysisError::AccessDenied(err.msg))
                }
                "InternalFailureException" => {
                    return RusotoError::Service(DescribeAnalysisError::InternalFailure(err.msg))
                }
                "InvalidParameterValueException" => {
                    return RusotoError::Service(DescribeAnalysisError::InvalidParameterValue(
                        err.msg,
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(DescribeAnalysisError::ResourceNotFound(err.msg))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(DescribeAnalysisError::Throttling(err.msg))
                }
                "UnsupportedUserEditionException" => {
                    return RusotoError::Service(DescribeAnalysisError::UnsupportedUserEdition(
                        err.msg,
                    ))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }

    fn refine(err: RusotoError<std::convert::Infallible>) -> RusotoError<DescribeAnalysisError> {
        match err {
            RusotoError::Service(err) => match err {},
            RusotoError::HttpDispatch(err) => RusotoError::HttpDispatch(err),
            RusotoError::Credentials(err) => RusotoError::Credentials(err),
            RusotoError::Validation(err) => RusotoError::Validation(err),
            RusotoError::ParseError(err) => RusotoError::ParseError(err),
            RusotoError::Unknown(res) => Self::from_response(res),
            RusotoError::Blocking => RusotoError::Blocking,
        }
    }
}
impl fmt::Display for DescribeAnalysisError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeAnalysisError::AccessDenied(ref cause) => write!(f, "{}", cause),
            DescribeAnalysisError::InternalFailure(ref cause) => write!(f, "{}", cause),
            DescribeAnalysisError::InvalidParameterValue(ref cause) => write!(f, "{}", cause),
            DescribeAnalysisError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            DescribeAnalysisError::Throttling(ref cause) => write!(f, "{}", cause),
            DescribeAnalysisError::UnsupportedUserEdition(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DescribeAnalysisError {}
/// Errors returned by DescribeAnalysisPermissions
#[derive(Debug, PartialEq)]
pub enum DescribeAnalysisPermissionsError {
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

impl DescribeAnalysisPermissionsError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<DescribeAnalysisPermissionsError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InternalFailureException" => {
                    return RusotoError::Service(DescribeAnalysisPermissionsError::InternalFailure(
                        err.msg,
                    ))
                }
                "InvalidParameterValueException" => {
                    return RusotoError::Service(
                        DescribeAnalysisPermissionsError::InvalidParameterValue(err.msg),
                    )
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(
                        DescribeAnalysisPermissionsError::ResourceNotFound(err.msg),
                    )
                }
                "ThrottlingException" => {
                    return RusotoError::Service(DescribeAnalysisPermissionsError::Throttling(
                        err.msg,
                    ))
                }
                "UnsupportedUserEditionException" => {
                    return RusotoError::Service(
                        DescribeAnalysisPermissionsError::UnsupportedUserEdition(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }

    fn refine(
        err: RusotoError<std::convert::Infallible>,
    ) -> RusotoError<DescribeAnalysisPermissionsError> {
        match err {
            RusotoError::Service(err) => match err {},
            RusotoError::HttpDispatch(err) => RusotoError::HttpDispatch(err),
            RusotoError::Credentials(err) => RusotoError::Credentials(err),
            RusotoError::Validation(err) => RusotoError::Validation(err),
            RusotoError::ParseError(err) => RusotoError::ParseError(err),
            RusotoError::Unknown(res) => Self::from_response(res),
            RusotoError::Blocking => RusotoError::Blocking,
        }
    }
}
impl fmt::Display for DescribeAnalysisPermissionsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeAnalysisPermissionsError::InternalFailure(ref cause) => write!(f, "{}", cause),
            DescribeAnalysisPermissionsError::InvalidParameterValue(ref cause) => {
                write!(f, "{}", cause)
            }
            DescribeAnalysisPermissionsError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            DescribeAnalysisPermissionsError::Throttling(ref cause) => write!(f, "{}", cause),
            DescribeAnalysisPermissionsError::UnsupportedUserEdition(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for DescribeAnalysisPermissionsError {}
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

    fn refine(err: RusotoError<std::convert::Infallible>) -> RusotoError<DescribeDashboardError> {
        match err {
            RusotoError::Service(err) => match err {},
            RusotoError::HttpDispatch(err) => RusotoError::HttpDispatch(err),
            RusotoError::Credentials(err) => RusotoError::Credentials(err),
            RusotoError::Validation(err) => RusotoError::Validation(err),
            RusotoError::ParseError(err) => RusotoError::ParseError(err),
            RusotoError::Unknown(res) => Self::from_response(res),
            RusotoError::Blocking => RusotoError::Blocking,
        }
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

    fn refine(
        err: RusotoError<std::convert::Infallible>,
    ) -> RusotoError<DescribeDashboardPermissionsError> {
        match err {
            RusotoError::Service(err) => match err {},
            RusotoError::HttpDispatch(err) => RusotoError::HttpDispatch(err),
            RusotoError::Credentials(err) => RusotoError::Credentials(err),
            RusotoError::Validation(err) => RusotoError::Validation(err),
            RusotoError::ParseError(err) => RusotoError::ParseError(err),
            RusotoError::Unknown(res) => Self::from_response(res),
            RusotoError::Blocking => RusotoError::Blocking,
        }
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

    fn refine(err: RusotoError<std::convert::Infallible>) -> RusotoError<DescribeDataSetError> {
        match err {
            RusotoError::Service(err) => match err {},
            RusotoError::HttpDispatch(err) => RusotoError::HttpDispatch(err),
            RusotoError::Credentials(err) => RusotoError::Credentials(err),
            RusotoError::Validation(err) => RusotoError::Validation(err),
            RusotoError::ParseError(err) => RusotoError::ParseError(err),
            RusotoError::Unknown(res) => Self::from_response(res),
            RusotoError::Blocking => RusotoError::Blocking,
        }
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

    fn refine(
        err: RusotoError<std::convert::Infallible>,
    ) -> RusotoError<DescribeDataSetPermissionsError> {
        match err {
            RusotoError::Service(err) => match err {},
            RusotoError::HttpDispatch(err) => RusotoError::HttpDispatch(err),
            RusotoError::Credentials(err) => RusotoError::Credentials(err),
            RusotoError::Validation(err) => RusotoError::Validation(err),
            RusotoError::ParseError(err) => RusotoError::ParseError(err),
            RusotoError::Unknown(res) => Self::from_response(res),
            RusotoError::Blocking => RusotoError::Blocking,
        }
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

    fn refine(err: RusotoError<std::convert::Infallible>) -> RusotoError<DescribeDataSourceError> {
        match err {
            RusotoError::Service(err) => match err {},
            RusotoError::HttpDispatch(err) => RusotoError::HttpDispatch(err),
            RusotoError::Credentials(err) => RusotoError::Credentials(err),
            RusotoError::Validation(err) => RusotoError::Validation(err),
            RusotoError::ParseError(err) => RusotoError::ParseError(err),
            RusotoError::Unknown(res) => Self::from_response(res),
            RusotoError::Blocking => RusotoError::Blocking,
        }
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

    fn refine(
        err: RusotoError<std::convert::Infallible>,
    ) -> RusotoError<DescribeDataSourcePermissionsError> {
        match err {
            RusotoError::Service(err) => match err {},
            RusotoError::HttpDispatch(err) => RusotoError::HttpDispatch(err),
            RusotoError::Credentials(err) => RusotoError::Credentials(err),
            RusotoError::Validation(err) => RusotoError::Validation(err),
            RusotoError::ParseError(err) => RusotoError::ParseError(err),
            RusotoError::Unknown(res) => Self::from_response(res),
            RusotoError::Blocking => RusotoError::Blocking,
        }
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

    fn refine(err: RusotoError<std::convert::Infallible>) -> RusotoError<DescribeGroupError> {
        match err {
            RusotoError::Service(err) => match err {},
            RusotoError::HttpDispatch(err) => RusotoError::HttpDispatch(err),
            RusotoError::Credentials(err) => RusotoError::Credentials(err),
            RusotoError::Validation(err) => RusotoError::Validation(err),
            RusotoError::ParseError(err) => RusotoError::ParseError(err),
            RusotoError::Unknown(res) => Self::from_response(res),
            RusotoError::Blocking => RusotoError::Blocking,
        }
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

    fn refine(
        err: RusotoError<std::convert::Infallible>,
    ) -> RusotoError<DescribeIAMPolicyAssignmentError> {
        match err {
            RusotoError::Service(err) => match err {},
            RusotoError::HttpDispatch(err) => RusotoError::HttpDispatch(err),
            RusotoError::Credentials(err) => RusotoError::Credentials(err),
            RusotoError::Validation(err) => RusotoError::Validation(err),
            RusotoError::ParseError(err) => RusotoError::ParseError(err),
            RusotoError::Unknown(res) => Self::from_response(res),
            RusotoError::Blocking => RusotoError::Blocking,
        }
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

    fn refine(err: RusotoError<std::convert::Infallible>) -> RusotoError<DescribeIngestionError> {
        match err {
            RusotoError::Service(err) => match err {},
            RusotoError::HttpDispatch(err) => RusotoError::HttpDispatch(err),
            RusotoError::Credentials(err) => RusotoError::Credentials(err),
            RusotoError::Validation(err) => RusotoError::Validation(err),
            RusotoError::ParseError(err) => RusotoError::ParseError(err),
            RusotoError::Unknown(res) => Self::from_response(res),
            RusotoError::Blocking => RusotoError::Blocking,
        }
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
/// Errors returned by DescribeNamespace
#[derive(Debug, PartialEq)]
pub enum DescribeNamespaceError {
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

impl DescribeNamespaceError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeNamespaceError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(DescribeNamespaceError::AccessDenied(err.msg))
                }
                "InternalFailureException" => {
                    return RusotoError::Service(DescribeNamespaceError::InternalFailure(err.msg))
                }
                "InvalidParameterValueException" => {
                    return RusotoError::Service(DescribeNamespaceError::InvalidParameterValue(
                        err.msg,
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(DescribeNamespaceError::ResourceNotFound(err.msg))
                }
                "ResourceUnavailableException" => {
                    return RusotoError::Service(DescribeNamespaceError::ResourceUnavailable(
                        err.msg,
                    ))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(DescribeNamespaceError::Throttling(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }

    fn refine(err: RusotoError<std::convert::Infallible>) -> RusotoError<DescribeNamespaceError> {
        match err {
            RusotoError::Service(err) => match err {},
            RusotoError::HttpDispatch(err) => RusotoError::HttpDispatch(err),
            RusotoError::Credentials(err) => RusotoError::Credentials(err),
            RusotoError::Validation(err) => RusotoError::Validation(err),
            RusotoError::ParseError(err) => RusotoError::ParseError(err),
            RusotoError::Unknown(res) => Self::from_response(res),
            RusotoError::Blocking => RusotoError::Blocking,
        }
    }
}
impl fmt::Display for DescribeNamespaceError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeNamespaceError::AccessDenied(ref cause) => write!(f, "{}", cause),
            DescribeNamespaceError::InternalFailure(ref cause) => write!(f, "{}", cause),
            DescribeNamespaceError::InvalidParameterValue(ref cause) => write!(f, "{}", cause),
            DescribeNamespaceError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            DescribeNamespaceError::ResourceUnavailable(ref cause) => write!(f, "{}", cause),
            DescribeNamespaceError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DescribeNamespaceError {}
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

    fn refine(err: RusotoError<std::convert::Infallible>) -> RusotoError<DescribeTemplateError> {
        match err {
            RusotoError::Service(err) => match err {},
            RusotoError::HttpDispatch(err) => RusotoError::HttpDispatch(err),
            RusotoError::Credentials(err) => RusotoError::Credentials(err),
            RusotoError::Validation(err) => RusotoError::Validation(err),
            RusotoError::ParseError(err) => RusotoError::ParseError(err),
            RusotoError::Unknown(res) => Self::from_response(res),
            RusotoError::Blocking => RusotoError::Blocking,
        }
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

    fn refine(
        err: RusotoError<std::convert::Infallible>,
    ) -> RusotoError<DescribeTemplateAliasError> {
        match err {
            RusotoError::Service(err) => match err {},
            RusotoError::HttpDispatch(err) => RusotoError::HttpDispatch(err),
            RusotoError::Credentials(err) => RusotoError::Credentials(err),
            RusotoError::Validation(err) => RusotoError::Validation(err),
            RusotoError::ParseError(err) => RusotoError::ParseError(err),
            RusotoError::Unknown(res) => Self::from_response(res),
            RusotoError::Blocking => RusotoError::Blocking,
        }
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

    fn refine(
        err: RusotoError<std::convert::Infallible>,
    ) -> RusotoError<DescribeTemplatePermissionsError> {
        match err {
            RusotoError::Service(err) => match err {},
            RusotoError::HttpDispatch(err) => RusotoError::HttpDispatch(err),
            RusotoError::Credentials(err) => RusotoError::Credentials(err),
            RusotoError::Validation(err) => RusotoError::Validation(err),
            RusotoError::ParseError(err) => RusotoError::ParseError(err),
            RusotoError::Unknown(res) => Self::from_response(res),
            RusotoError::Blocking => RusotoError::Blocking,
        }
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
/// Errors returned by DescribeTheme
#[derive(Debug, PartialEq)]
pub enum DescribeThemeError {
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
    /// <p>This error indicates that you are calling an operation on an Amazon QuickSight subscription where the edition doesn't include support for that operation. Amazon QuickSight currently has Standard Edition and Enterprise Edition. Not every operation and capability is available in every edition.</p>
    UnsupportedUserEdition(String),
}

impl DescribeThemeError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeThemeError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(DescribeThemeError::AccessDenied(err.msg))
                }
                "InternalFailureException" => {
                    return RusotoError::Service(DescribeThemeError::InternalFailure(err.msg))
                }
                "InvalidParameterValueException" => {
                    return RusotoError::Service(DescribeThemeError::InvalidParameterValue(err.msg))
                }
                "ResourceExistsException" => {
                    return RusotoError::Service(DescribeThemeError::ResourceExists(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(DescribeThemeError::ResourceNotFound(err.msg))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(DescribeThemeError::Throttling(err.msg))
                }
                "UnsupportedUserEditionException" => {
                    return RusotoError::Service(DescribeThemeError::UnsupportedUserEdition(
                        err.msg,
                    ))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }

    fn refine(err: RusotoError<std::convert::Infallible>) -> RusotoError<DescribeThemeError> {
        match err {
            RusotoError::Service(err) => match err {},
            RusotoError::HttpDispatch(err) => RusotoError::HttpDispatch(err),
            RusotoError::Credentials(err) => RusotoError::Credentials(err),
            RusotoError::Validation(err) => RusotoError::Validation(err),
            RusotoError::ParseError(err) => RusotoError::ParseError(err),
            RusotoError::Unknown(res) => Self::from_response(res),
            RusotoError::Blocking => RusotoError::Blocking,
        }
    }
}
impl fmt::Display for DescribeThemeError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeThemeError::AccessDenied(ref cause) => write!(f, "{}", cause),
            DescribeThemeError::InternalFailure(ref cause) => write!(f, "{}", cause),
            DescribeThemeError::InvalidParameterValue(ref cause) => write!(f, "{}", cause),
            DescribeThemeError::ResourceExists(ref cause) => write!(f, "{}", cause),
            DescribeThemeError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            DescribeThemeError::Throttling(ref cause) => write!(f, "{}", cause),
            DescribeThemeError::UnsupportedUserEdition(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DescribeThemeError {}
/// Errors returned by DescribeThemeAlias
#[derive(Debug, PartialEq)]
pub enum DescribeThemeAliasError {
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

impl DescribeThemeAliasError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeThemeAliasError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "ConflictException" => {
                    return RusotoError::Service(DescribeThemeAliasError::Conflict(err.msg))
                }
                "InternalFailureException" => {
                    return RusotoError::Service(DescribeThemeAliasError::InternalFailure(err.msg))
                }
                "InvalidParameterValueException" => {
                    return RusotoError::Service(DescribeThemeAliasError::InvalidParameterValue(
                        err.msg,
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(DescribeThemeAliasError::ResourceNotFound(err.msg))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(DescribeThemeAliasError::Throttling(err.msg))
                }
                "UnsupportedUserEditionException" => {
                    return RusotoError::Service(DescribeThemeAliasError::UnsupportedUserEdition(
                        err.msg,
                    ))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }

    fn refine(err: RusotoError<std::convert::Infallible>) -> RusotoError<DescribeThemeAliasError> {
        match err {
            RusotoError::Service(err) => match err {},
            RusotoError::HttpDispatch(err) => RusotoError::HttpDispatch(err),
            RusotoError::Credentials(err) => RusotoError::Credentials(err),
            RusotoError::Validation(err) => RusotoError::Validation(err),
            RusotoError::ParseError(err) => RusotoError::ParseError(err),
            RusotoError::Unknown(res) => Self::from_response(res),
            RusotoError::Blocking => RusotoError::Blocking,
        }
    }
}
impl fmt::Display for DescribeThemeAliasError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeThemeAliasError::Conflict(ref cause) => write!(f, "{}", cause),
            DescribeThemeAliasError::InternalFailure(ref cause) => write!(f, "{}", cause),
            DescribeThemeAliasError::InvalidParameterValue(ref cause) => write!(f, "{}", cause),
            DescribeThemeAliasError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            DescribeThemeAliasError::Throttling(ref cause) => write!(f, "{}", cause),
            DescribeThemeAliasError::UnsupportedUserEdition(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DescribeThemeAliasError {}
/// Errors returned by DescribeThemePermissions
#[derive(Debug, PartialEq)]
pub enum DescribeThemePermissionsError {
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

impl DescribeThemePermissionsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeThemePermissionsError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(DescribeThemePermissionsError::AccessDenied(
                        err.msg,
                    ))
                }
                "InternalFailureException" => {
                    return RusotoError::Service(DescribeThemePermissionsError::InternalFailure(
                        err.msg,
                    ))
                }
                "InvalidParameterValueException" => {
                    return RusotoError::Service(
                        DescribeThemePermissionsError::InvalidParameterValue(err.msg),
                    )
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(DescribeThemePermissionsError::ResourceNotFound(
                        err.msg,
                    ))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(DescribeThemePermissionsError::Throttling(err.msg))
                }
                "UnsupportedUserEditionException" => {
                    return RusotoError::Service(
                        DescribeThemePermissionsError::UnsupportedUserEdition(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }

    fn refine(
        err: RusotoError<std::convert::Infallible>,
    ) -> RusotoError<DescribeThemePermissionsError> {
        match err {
            RusotoError::Service(err) => match err {},
            RusotoError::HttpDispatch(err) => RusotoError::HttpDispatch(err),
            RusotoError::Credentials(err) => RusotoError::Credentials(err),
            RusotoError::Validation(err) => RusotoError::Validation(err),
            RusotoError::ParseError(err) => RusotoError::ParseError(err),
            RusotoError::Unknown(res) => Self::from_response(res),
            RusotoError::Blocking => RusotoError::Blocking,
        }
    }
}
impl fmt::Display for DescribeThemePermissionsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeThemePermissionsError::AccessDenied(ref cause) => write!(f, "{}", cause),
            DescribeThemePermissionsError::InternalFailure(ref cause) => write!(f, "{}", cause),
            DescribeThemePermissionsError::InvalidParameterValue(ref cause) => {
                write!(f, "{}", cause)
            }
            DescribeThemePermissionsError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            DescribeThemePermissionsError::Throttling(ref cause) => write!(f, "{}", cause),
            DescribeThemePermissionsError::UnsupportedUserEdition(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for DescribeThemePermissionsError {}
/// Errors returned by DescribeUser
#[derive(Debug, PartialEq)]
pub enum DescribeUserError {
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
                "PreconditionNotMetException" => {
                    return RusotoError::Service(DescribeUserError::PreconditionNotMet(err.msg))
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

    fn refine(err: RusotoError<std::convert::Infallible>) -> RusotoError<DescribeUserError> {
        match err {
            RusotoError::Service(err) => match err {},
            RusotoError::HttpDispatch(err) => RusotoError::HttpDispatch(err),
            RusotoError::Credentials(err) => RusotoError::Credentials(err),
            RusotoError::Validation(err) => RusotoError::Validation(err),
            RusotoError::ParseError(err) => RusotoError::ParseError(err),
            RusotoError::Unknown(res) => Self::from_response(res),
            RusotoError::Blocking => RusotoError::Blocking,
        }
    }
}
impl fmt::Display for DescribeUserError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeUserError::AccessDenied(ref cause) => write!(f, "{}", cause),
            DescribeUserError::InternalFailure(ref cause) => write!(f, "{}", cause),
            DescribeUserError::InvalidParameterValue(ref cause) => write!(f, "{}", cause),
            DescribeUserError::PreconditionNotMet(ref cause) => write!(f, "{}", cause),
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
    /// <p>This error indicates that you are calling an embedding operation in Amazon QuickSight without the required pricing plan on your AWS account. Before you can use embedding for anonymous users, a QuickSight administrator needs to add capacity pricing to QuickSight. You can do this on the <b>Manage QuickSight</b> page. </p> <p>After capacity pricing is added, you can use the <a>GetDashboardEmbedUrl</a> API operation with the <code>--identity-type ANONYMOUS</code> option.</p>
    UnsupportedPricingPlan(String),
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
                "UnsupportedPricingPlanException" => {
                    return RusotoError::Service(GetDashboardEmbedUrlError::UnsupportedPricingPlan(
                        err.msg,
                    ))
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

    fn refine(
        err: RusotoError<std::convert::Infallible>,
    ) -> RusotoError<GetDashboardEmbedUrlError> {
        match err {
            RusotoError::Service(err) => match err {},
            RusotoError::HttpDispatch(err) => RusotoError::HttpDispatch(err),
            RusotoError::Credentials(err) => RusotoError::Credentials(err),
            RusotoError::Validation(err) => RusotoError::Validation(err),
            RusotoError::ParseError(err) => RusotoError::ParseError(err),
            RusotoError::Unknown(res) => Self::from_response(res),
            RusotoError::Blocking => RusotoError::Blocking,
        }
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
            GetDashboardEmbedUrlError::UnsupportedPricingPlan(ref cause) => write!(f, "{}", cause),
            GetDashboardEmbedUrlError::UnsupportedUserEdition(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetDashboardEmbedUrlError {}
/// Errors returned by GetSessionEmbedUrl
#[derive(Debug, PartialEq)]
pub enum GetSessionEmbedUrlError {
    /// <p>You don't have access to this item. The provided credentials couldn't be validated. You might not be authorized to carry out the request. Make sure that your account is authorized to use the Amazon QuickSight service, that your policies have the correct permissions, and that you are using the correct access keys.</p>
    AccessDenied(String),
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

impl GetSessionEmbedUrlError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetSessionEmbedUrlError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(GetSessionEmbedUrlError::AccessDenied(err.msg))
                }
                "InternalFailureException" => {
                    return RusotoError::Service(GetSessionEmbedUrlError::InternalFailure(err.msg))
                }
                "InvalidParameterValueException" => {
                    return RusotoError::Service(GetSessionEmbedUrlError::InvalidParameterValue(
                        err.msg,
                    ))
                }
                "QuickSightUserNotFoundException" => {
                    return RusotoError::Service(GetSessionEmbedUrlError::QuickSightUserNotFound(
                        err.msg,
                    ))
                }
                "ResourceExistsException" => {
                    return RusotoError::Service(GetSessionEmbedUrlError::ResourceExists(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(GetSessionEmbedUrlError::ResourceNotFound(err.msg))
                }
                "SessionLifetimeInMinutesInvalidException" => {
                    return RusotoError::Service(
                        GetSessionEmbedUrlError::SessionLifetimeInMinutesInvalid(err.msg),
                    )
                }
                "ThrottlingException" => {
                    return RusotoError::Service(GetSessionEmbedUrlError::Throttling(err.msg))
                }
                "UnsupportedUserEditionException" => {
                    return RusotoError::Service(GetSessionEmbedUrlError::UnsupportedUserEdition(
                        err.msg,
                    ))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }

    fn refine(err: RusotoError<std::convert::Infallible>) -> RusotoError<GetSessionEmbedUrlError> {
        match err {
            RusotoError::Service(err) => match err {},
            RusotoError::HttpDispatch(err) => RusotoError::HttpDispatch(err),
            RusotoError::Credentials(err) => RusotoError::Credentials(err),
            RusotoError::Validation(err) => RusotoError::Validation(err),
            RusotoError::ParseError(err) => RusotoError::ParseError(err),
            RusotoError::Unknown(res) => Self::from_response(res),
            RusotoError::Blocking => RusotoError::Blocking,
        }
    }
}
impl fmt::Display for GetSessionEmbedUrlError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetSessionEmbedUrlError::AccessDenied(ref cause) => write!(f, "{}", cause),
            GetSessionEmbedUrlError::InternalFailure(ref cause) => write!(f, "{}", cause),
            GetSessionEmbedUrlError::InvalidParameterValue(ref cause) => write!(f, "{}", cause),
            GetSessionEmbedUrlError::QuickSightUserNotFound(ref cause) => write!(f, "{}", cause),
            GetSessionEmbedUrlError::ResourceExists(ref cause) => write!(f, "{}", cause),
            GetSessionEmbedUrlError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            GetSessionEmbedUrlError::SessionLifetimeInMinutesInvalid(ref cause) => {
                write!(f, "{}", cause)
            }
            GetSessionEmbedUrlError::Throttling(ref cause) => write!(f, "{}", cause),
            GetSessionEmbedUrlError::UnsupportedUserEdition(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetSessionEmbedUrlError {}
/// Errors returned by ListAnalyses
#[derive(Debug, PartialEq)]
pub enum ListAnalysesError {
    /// <p>An internal failure occurred.</p>
    InternalFailure(String),
    /// <p>The <code>NextToken</code> value isn't valid.</p>
    InvalidNextToken(String),
    /// <p>Access is throttled.</p>
    Throttling(String),
    /// <p>This error indicates that you are calling an operation on an Amazon QuickSight subscription where the edition doesn't include support for that operation. Amazon QuickSight currently has Standard Edition and Enterprise Edition. Not every operation and capability is available in every edition.</p>
    UnsupportedUserEdition(String),
}

impl ListAnalysesError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListAnalysesError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InternalFailureException" => {
                    return RusotoError::Service(ListAnalysesError::InternalFailure(err.msg))
                }
                "InvalidNextTokenException" => {
                    return RusotoError::Service(ListAnalysesError::InvalidNextToken(err.msg))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(ListAnalysesError::Throttling(err.msg))
                }
                "UnsupportedUserEditionException" => {
                    return RusotoError::Service(ListAnalysesError::UnsupportedUserEdition(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }

    fn refine(err: RusotoError<std::convert::Infallible>) -> RusotoError<ListAnalysesError> {
        match err {
            RusotoError::Service(err) => match err {},
            RusotoError::HttpDispatch(err) => RusotoError::HttpDispatch(err),
            RusotoError::Credentials(err) => RusotoError::Credentials(err),
            RusotoError::Validation(err) => RusotoError::Validation(err),
            RusotoError::ParseError(err) => RusotoError::ParseError(err),
            RusotoError::Unknown(res) => Self::from_response(res),
            RusotoError::Blocking => RusotoError::Blocking,
        }
    }
}
impl fmt::Display for ListAnalysesError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListAnalysesError::InternalFailure(ref cause) => write!(f, "{}", cause),
            ListAnalysesError::InvalidNextToken(ref cause) => write!(f, "{}", cause),
            ListAnalysesError::Throttling(ref cause) => write!(f, "{}", cause),
            ListAnalysesError::UnsupportedUserEdition(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListAnalysesError {}
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

    fn refine(
        err: RusotoError<std::convert::Infallible>,
    ) -> RusotoError<ListDashboardVersionsError> {
        match err {
            RusotoError::Service(err) => match err {},
            RusotoError::HttpDispatch(err) => RusotoError::HttpDispatch(err),
            RusotoError::Credentials(err) => RusotoError::Credentials(err),
            RusotoError::Validation(err) => RusotoError::Validation(err),
            RusotoError::ParseError(err) => RusotoError::ParseError(err),
            RusotoError::Unknown(res) => Self::from_response(res),
            RusotoError::Blocking => RusotoError::Blocking,
        }
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

    fn refine(err: RusotoError<std::convert::Infallible>) -> RusotoError<ListDashboardsError> {
        match err {
            RusotoError::Service(err) => match err {},
            RusotoError::HttpDispatch(err) => RusotoError::HttpDispatch(err),
            RusotoError::Credentials(err) => RusotoError::Credentials(err),
            RusotoError::Validation(err) => RusotoError::Validation(err),
            RusotoError::ParseError(err) => RusotoError::ParseError(err),
            RusotoError::Unknown(res) => Self::from_response(res),
            RusotoError::Blocking => RusotoError::Blocking,
        }
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

    fn refine(err: RusotoError<std::convert::Infallible>) -> RusotoError<ListDataSetsError> {
        match err {
            RusotoError::Service(err) => match err {},
            RusotoError::HttpDispatch(err) => RusotoError::HttpDispatch(err),
            RusotoError::Credentials(err) => RusotoError::Credentials(err),
            RusotoError::Validation(err) => RusotoError::Validation(err),
            RusotoError::ParseError(err) => RusotoError::ParseError(err),
            RusotoError::Unknown(res) => Self::from_response(res),
            RusotoError::Blocking => RusotoError::Blocking,
        }
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

    fn refine(err: RusotoError<std::convert::Infallible>) -> RusotoError<ListDataSourcesError> {
        match err {
            RusotoError::Service(err) => match err {},
            RusotoError::HttpDispatch(err) => RusotoError::HttpDispatch(err),
            RusotoError::Credentials(err) => RusotoError::Credentials(err),
            RusotoError::Validation(err) => RusotoError::Validation(err),
            RusotoError::ParseError(err) => RusotoError::ParseError(err),
            RusotoError::Unknown(res) => Self::from_response(res),
            RusotoError::Blocking => RusotoError::Blocking,
        }
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

    fn refine(
        err: RusotoError<std::convert::Infallible>,
    ) -> RusotoError<ListGroupMembershipsError> {
        match err {
            RusotoError::Service(err) => match err {},
            RusotoError::HttpDispatch(err) => RusotoError::HttpDispatch(err),
            RusotoError::Credentials(err) => RusotoError::Credentials(err),
            RusotoError::Validation(err) => RusotoError::Validation(err),
            RusotoError::ParseError(err) => RusotoError::ParseError(err),
            RusotoError::Unknown(res) => Self::from_response(res),
            RusotoError::Blocking => RusotoError::Blocking,
        }
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

    fn refine(err: RusotoError<std::convert::Infallible>) -> RusotoError<ListGroupsError> {
        match err {
            RusotoError::Service(err) => match err {},
            RusotoError::HttpDispatch(err) => RusotoError::HttpDispatch(err),
            RusotoError::Credentials(err) => RusotoError::Credentials(err),
            RusotoError::Validation(err) => RusotoError::Validation(err),
            RusotoError::ParseError(err) => RusotoError::ParseError(err),
            RusotoError::Unknown(res) => Self::from_response(res),
            RusotoError::Blocking => RusotoError::Blocking,
        }
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

    fn refine(
        err: RusotoError<std::convert::Infallible>,
    ) -> RusotoError<ListIAMPolicyAssignmentsError> {
        match err {
            RusotoError::Service(err) => match err {},
            RusotoError::HttpDispatch(err) => RusotoError::HttpDispatch(err),
            RusotoError::Credentials(err) => RusotoError::Credentials(err),
            RusotoError::Validation(err) => RusotoError::Validation(err),
            RusotoError::ParseError(err) => RusotoError::ParseError(err),
            RusotoError::Unknown(res) => Self::from_response(res),
            RusotoError::Blocking => RusotoError::Blocking,
        }
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
    /// <p>A resource is already in a state that indicates an operation is happening that must complete before a new update can be applied.</p>
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

    fn refine(
        err: RusotoError<std::convert::Infallible>,
    ) -> RusotoError<ListIAMPolicyAssignmentsForUserError> {
        match err {
            RusotoError::Service(err) => match err {},
            RusotoError::HttpDispatch(err) => RusotoError::HttpDispatch(err),
            RusotoError::Credentials(err) => RusotoError::Credentials(err),
            RusotoError::Validation(err) => RusotoError::Validation(err),
            RusotoError::ParseError(err) => RusotoError::ParseError(err),
            RusotoError::Unknown(res) => Self::from_response(res),
            RusotoError::Blocking => RusotoError::Blocking,
        }
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

    fn refine(err: RusotoError<std::convert::Infallible>) -> RusotoError<ListIngestionsError> {
        match err {
            RusotoError::Service(err) => match err {},
            RusotoError::HttpDispatch(err) => RusotoError::HttpDispatch(err),
            RusotoError::Credentials(err) => RusotoError::Credentials(err),
            RusotoError::Validation(err) => RusotoError::Validation(err),
            RusotoError::ParseError(err) => RusotoError::ParseError(err),
            RusotoError::Unknown(res) => Self::from_response(res),
            RusotoError::Blocking => RusotoError::Blocking,
        }
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
/// Errors returned by ListNamespaces
#[derive(Debug, PartialEq)]
pub enum ListNamespacesError {
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

impl ListNamespacesError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListNamespacesError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(ListNamespacesError::AccessDenied(err.msg))
                }
                "InternalFailureException" => {
                    return RusotoError::Service(ListNamespacesError::InternalFailure(err.msg))
                }
                "InvalidNextTokenException" => {
                    return RusotoError::Service(ListNamespacesError::InvalidNextToken(err.msg))
                }
                "InvalidParameterValueException" => {
                    return RusotoError::Service(ListNamespacesError::InvalidParameterValue(
                        err.msg,
                    ))
                }
                "PreconditionNotMetException" => {
                    return RusotoError::Service(ListNamespacesError::PreconditionNotMet(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(ListNamespacesError::ResourceNotFound(err.msg))
                }
                "ResourceUnavailableException" => {
                    return RusotoError::Service(ListNamespacesError::ResourceUnavailable(err.msg))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(ListNamespacesError::Throttling(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }

    fn refine(err: RusotoError<std::convert::Infallible>) -> RusotoError<ListNamespacesError> {
        match err {
            RusotoError::Service(err) => match err {},
            RusotoError::HttpDispatch(err) => RusotoError::HttpDispatch(err),
            RusotoError::Credentials(err) => RusotoError::Credentials(err),
            RusotoError::Validation(err) => RusotoError::Validation(err),
            RusotoError::ParseError(err) => RusotoError::ParseError(err),
            RusotoError::Unknown(res) => Self::from_response(res),
            RusotoError::Blocking => RusotoError::Blocking,
        }
    }
}
impl fmt::Display for ListNamespacesError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListNamespacesError::AccessDenied(ref cause) => write!(f, "{}", cause),
            ListNamespacesError::InternalFailure(ref cause) => write!(f, "{}", cause),
            ListNamespacesError::InvalidNextToken(ref cause) => write!(f, "{}", cause),
            ListNamespacesError::InvalidParameterValue(ref cause) => write!(f, "{}", cause),
            ListNamespacesError::PreconditionNotMet(ref cause) => write!(f, "{}", cause),
            ListNamespacesError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            ListNamespacesError::ResourceUnavailable(ref cause) => write!(f, "{}", cause),
            ListNamespacesError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListNamespacesError {}
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

    fn refine(err: RusotoError<std::convert::Infallible>) -> RusotoError<ListTagsForResourceError> {
        match err {
            RusotoError::Service(err) => match err {},
            RusotoError::HttpDispatch(err) => RusotoError::HttpDispatch(err),
            RusotoError::Credentials(err) => RusotoError::Credentials(err),
            RusotoError::Validation(err) => RusotoError::Validation(err),
            RusotoError::ParseError(err) => RusotoError::ParseError(err),
            RusotoError::Unknown(res) => Self::from_response(res),
            RusotoError::Blocking => RusotoError::Blocking,
        }
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
    /// <p>The <code>NextToken</code> value isn't valid.</p>
    InvalidNextToken(String),
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
                "InvalidNextTokenException" => {
                    return RusotoError::Service(ListTemplateAliasesError::InvalidNextToken(
                        err.msg,
                    ))
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

    fn refine(err: RusotoError<std::convert::Infallible>) -> RusotoError<ListTemplateAliasesError> {
        match err {
            RusotoError::Service(err) => match err {},
            RusotoError::HttpDispatch(err) => RusotoError::HttpDispatch(err),
            RusotoError::Credentials(err) => RusotoError::Credentials(err),
            RusotoError::Validation(err) => RusotoError::Validation(err),
            RusotoError::ParseError(err) => RusotoError::ParseError(err),
            RusotoError::Unknown(res) => Self::from_response(res),
            RusotoError::Blocking => RusotoError::Blocking,
        }
    }
}
impl fmt::Display for ListTemplateAliasesError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListTemplateAliasesError::InternalFailure(ref cause) => write!(f, "{}", cause),
            ListTemplateAliasesError::InvalidNextToken(ref cause) => write!(f, "{}", cause),
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

    fn refine(
        err: RusotoError<std::convert::Infallible>,
    ) -> RusotoError<ListTemplateVersionsError> {
        match err {
            RusotoError::Service(err) => match err {},
            RusotoError::HttpDispatch(err) => RusotoError::HttpDispatch(err),
            RusotoError::Credentials(err) => RusotoError::Credentials(err),
            RusotoError::Validation(err) => RusotoError::Validation(err),
            RusotoError::ParseError(err) => RusotoError::ParseError(err),
            RusotoError::Unknown(res) => Self::from_response(res),
            RusotoError::Blocking => RusotoError::Blocking,
        }
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

    fn refine(err: RusotoError<std::convert::Infallible>) -> RusotoError<ListTemplatesError> {
        match err {
            RusotoError::Service(err) => match err {},
            RusotoError::HttpDispatch(err) => RusotoError::HttpDispatch(err),
            RusotoError::Credentials(err) => RusotoError::Credentials(err),
            RusotoError::Validation(err) => RusotoError::Validation(err),
            RusotoError::ParseError(err) => RusotoError::ParseError(err),
            RusotoError::Unknown(res) => Self::from_response(res),
            RusotoError::Blocking => RusotoError::Blocking,
        }
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
/// Errors returned by ListThemeAliases
#[derive(Debug, PartialEq)]
pub enum ListThemeAliasesError {
    /// <p>Updating or deleting a resource can cause an inconsistent state.</p>
    Conflict(String),
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

impl ListThemeAliasesError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListThemeAliasesError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "ConflictException" => {
                    return RusotoError::Service(ListThemeAliasesError::Conflict(err.msg))
                }
                "InternalFailureException" => {
                    return RusotoError::Service(ListThemeAliasesError::InternalFailure(err.msg))
                }
                "InvalidNextTokenException" => {
                    return RusotoError::Service(ListThemeAliasesError::InvalidNextToken(err.msg))
                }
                "InvalidParameterValueException" => {
                    return RusotoError::Service(ListThemeAliasesError::InvalidParameterValue(
                        err.msg,
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(ListThemeAliasesError::ResourceNotFound(err.msg))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(ListThemeAliasesError::Throttling(err.msg))
                }
                "UnsupportedUserEditionException" => {
                    return RusotoError::Service(ListThemeAliasesError::UnsupportedUserEdition(
                        err.msg,
                    ))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }

    fn refine(err: RusotoError<std::convert::Infallible>) -> RusotoError<ListThemeAliasesError> {
        match err {
            RusotoError::Service(err) => match err {},
            RusotoError::HttpDispatch(err) => RusotoError::HttpDispatch(err),
            RusotoError::Credentials(err) => RusotoError::Credentials(err),
            RusotoError::Validation(err) => RusotoError::Validation(err),
            RusotoError::ParseError(err) => RusotoError::ParseError(err),
            RusotoError::Unknown(res) => Self::from_response(res),
            RusotoError::Blocking => RusotoError::Blocking,
        }
    }
}
impl fmt::Display for ListThemeAliasesError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListThemeAliasesError::Conflict(ref cause) => write!(f, "{}", cause),
            ListThemeAliasesError::InternalFailure(ref cause) => write!(f, "{}", cause),
            ListThemeAliasesError::InvalidNextToken(ref cause) => write!(f, "{}", cause),
            ListThemeAliasesError::InvalidParameterValue(ref cause) => write!(f, "{}", cause),
            ListThemeAliasesError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            ListThemeAliasesError::Throttling(ref cause) => write!(f, "{}", cause),
            ListThemeAliasesError::UnsupportedUserEdition(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListThemeAliasesError {}
/// Errors returned by ListThemeVersions
#[derive(Debug, PartialEq)]
pub enum ListThemeVersionsError {
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
    /// <p>This error indicates that you are calling an operation on an Amazon QuickSight subscription where the edition doesn't include support for that operation. Amazon QuickSight currently has Standard Edition and Enterprise Edition. Not every operation and capability is available in every edition.</p>
    UnsupportedUserEdition(String),
}

impl ListThemeVersionsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListThemeVersionsError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(ListThemeVersionsError::AccessDenied(err.msg))
                }
                "InternalFailureException" => {
                    return RusotoError::Service(ListThemeVersionsError::InternalFailure(err.msg))
                }
                "InvalidNextTokenException" => {
                    return RusotoError::Service(ListThemeVersionsError::InvalidNextToken(err.msg))
                }
                "InvalidParameterValueException" => {
                    return RusotoError::Service(ListThemeVersionsError::InvalidParameterValue(
                        err.msg,
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(ListThemeVersionsError::ResourceNotFound(err.msg))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(ListThemeVersionsError::Throttling(err.msg))
                }
                "UnsupportedUserEditionException" => {
                    return RusotoError::Service(ListThemeVersionsError::UnsupportedUserEdition(
                        err.msg,
                    ))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }

    fn refine(err: RusotoError<std::convert::Infallible>) -> RusotoError<ListThemeVersionsError> {
        match err {
            RusotoError::Service(err) => match err {},
            RusotoError::HttpDispatch(err) => RusotoError::HttpDispatch(err),
            RusotoError::Credentials(err) => RusotoError::Credentials(err),
            RusotoError::Validation(err) => RusotoError::Validation(err),
            RusotoError::ParseError(err) => RusotoError::ParseError(err),
            RusotoError::Unknown(res) => Self::from_response(res),
            RusotoError::Blocking => RusotoError::Blocking,
        }
    }
}
impl fmt::Display for ListThemeVersionsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListThemeVersionsError::AccessDenied(ref cause) => write!(f, "{}", cause),
            ListThemeVersionsError::InternalFailure(ref cause) => write!(f, "{}", cause),
            ListThemeVersionsError::InvalidNextToken(ref cause) => write!(f, "{}", cause),
            ListThemeVersionsError::InvalidParameterValue(ref cause) => write!(f, "{}", cause),
            ListThemeVersionsError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            ListThemeVersionsError::Throttling(ref cause) => write!(f, "{}", cause),
            ListThemeVersionsError::UnsupportedUserEdition(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListThemeVersionsError {}
/// Errors returned by ListThemes
#[derive(Debug, PartialEq)]
pub enum ListThemesError {
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
    /// <p>This error indicates that you are calling an operation on an Amazon QuickSight subscription where the edition doesn't include support for that operation. Amazon QuickSight currently has Standard Edition and Enterprise Edition. Not every operation and capability is available in every edition.</p>
    UnsupportedUserEdition(String),
}

impl ListThemesError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListThemesError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(ListThemesError::AccessDenied(err.msg))
                }
                "InternalFailureException" => {
                    return RusotoError::Service(ListThemesError::InternalFailure(err.msg))
                }
                "InvalidNextTokenException" => {
                    return RusotoError::Service(ListThemesError::InvalidNextToken(err.msg))
                }
                "InvalidParameterValueException" => {
                    return RusotoError::Service(ListThemesError::InvalidParameterValue(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(ListThemesError::ResourceNotFound(err.msg))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(ListThemesError::Throttling(err.msg))
                }
                "UnsupportedUserEditionException" => {
                    return RusotoError::Service(ListThemesError::UnsupportedUserEdition(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }

    fn refine(err: RusotoError<std::convert::Infallible>) -> RusotoError<ListThemesError> {
        match err {
            RusotoError::Service(err) => match err {},
            RusotoError::HttpDispatch(err) => RusotoError::HttpDispatch(err),
            RusotoError::Credentials(err) => RusotoError::Credentials(err),
            RusotoError::Validation(err) => RusotoError::Validation(err),
            RusotoError::ParseError(err) => RusotoError::ParseError(err),
            RusotoError::Unknown(res) => Self::from_response(res),
            RusotoError::Blocking => RusotoError::Blocking,
        }
    }
}
impl fmt::Display for ListThemesError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListThemesError::AccessDenied(ref cause) => write!(f, "{}", cause),
            ListThemesError::InternalFailure(ref cause) => write!(f, "{}", cause),
            ListThemesError::InvalidNextToken(ref cause) => write!(f, "{}", cause),
            ListThemesError::InvalidParameterValue(ref cause) => write!(f, "{}", cause),
            ListThemesError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            ListThemesError::Throttling(ref cause) => write!(f, "{}", cause),
            ListThemesError::UnsupportedUserEdition(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListThemesError {}
/// Errors returned by ListUserGroups
#[derive(Debug, PartialEq)]
pub enum ListUserGroupsError {
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
                "PreconditionNotMetException" => {
                    return RusotoError::Service(ListUserGroupsError::PreconditionNotMet(err.msg))
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

    fn refine(err: RusotoError<std::convert::Infallible>) -> RusotoError<ListUserGroupsError> {
        match err {
            RusotoError::Service(err) => match err {},
            RusotoError::HttpDispatch(err) => RusotoError::HttpDispatch(err),
            RusotoError::Credentials(err) => RusotoError::Credentials(err),
            RusotoError::Validation(err) => RusotoError::Validation(err),
            RusotoError::ParseError(err) => RusotoError::ParseError(err),
            RusotoError::Unknown(res) => Self::from_response(res),
            RusotoError::Blocking => RusotoError::Blocking,
        }
    }
}
impl fmt::Display for ListUserGroupsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListUserGroupsError::AccessDenied(ref cause) => write!(f, "{}", cause),
            ListUserGroupsError::InternalFailure(ref cause) => write!(f, "{}", cause),
            ListUserGroupsError::InvalidParameterValue(ref cause) => write!(f, "{}", cause),
            ListUserGroupsError::PreconditionNotMet(ref cause) => write!(f, "{}", cause),
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
    /// <p>One or more preconditions aren't met.</p>
    PreconditionNotMet(String),
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
                "PreconditionNotMetException" => {
                    return RusotoError::Service(ListUsersError::PreconditionNotMet(err.msg))
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

    fn refine(err: RusotoError<std::convert::Infallible>) -> RusotoError<ListUsersError> {
        match err {
            RusotoError::Service(err) => match err {},
            RusotoError::HttpDispatch(err) => RusotoError::HttpDispatch(err),
            RusotoError::Credentials(err) => RusotoError::Credentials(err),
            RusotoError::Validation(err) => RusotoError::Validation(err),
            RusotoError::ParseError(err) => RusotoError::ParseError(err),
            RusotoError::Unknown(res) => Self::from_response(res),
            RusotoError::Blocking => RusotoError::Blocking,
        }
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
            ListUsersError::PreconditionNotMet(ref cause) => write!(f, "{}", cause),
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

    fn refine(err: RusotoError<std::convert::Infallible>) -> RusotoError<RegisterUserError> {
        match err {
            RusotoError::Service(err) => match err {},
            RusotoError::HttpDispatch(err) => RusotoError::HttpDispatch(err),
            RusotoError::Credentials(err) => RusotoError::Credentials(err),
            RusotoError::Validation(err) => RusotoError::Validation(err),
            RusotoError::ParseError(err) => RusotoError::ParseError(err),
            RusotoError::Unknown(res) => Self::from_response(res),
            RusotoError::Blocking => RusotoError::Blocking,
        }
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
/// Errors returned by RestoreAnalysis
#[derive(Debug, PartialEq)]
pub enum RestoreAnalysisError {
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

impl RestoreAnalysisError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<RestoreAnalysisError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "ConflictException" => {
                    return RusotoError::Service(RestoreAnalysisError::Conflict(err.msg))
                }
                "InternalFailureException" => {
                    return RusotoError::Service(RestoreAnalysisError::InternalFailure(err.msg))
                }
                "InvalidParameterValueException" => {
                    return RusotoError::Service(RestoreAnalysisError::InvalidParameterValue(
                        err.msg,
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(RestoreAnalysisError::ResourceNotFound(err.msg))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(RestoreAnalysisError::Throttling(err.msg))
                }
                "UnsupportedUserEditionException" => {
                    return RusotoError::Service(RestoreAnalysisError::UnsupportedUserEdition(
                        err.msg,
                    ))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }

    fn refine(err: RusotoError<std::convert::Infallible>) -> RusotoError<RestoreAnalysisError> {
        match err {
            RusotoError::Service(err) => match err {},
            RusotoError::HttpDispatch(err) => RusotoError::HttpDispatch(err),
            RusotoError::Credentials(err) => RusotoError::Credentials(err),
            RusotoError::Validation(err) => RusotoError::Validation(err),
            RusotoError::ParseError(err) => RusotoError::ParseError(err),
            RusotoError::Unknown(res) => Self::from_response(res),
            RusotoError::Blocking => RusotoError::Blocking,
        }
    }
}
impl fmt::Display for RestoreAnalysisError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            RestoreAnalysisError::Conflict(ref cause) => write!(f, "{}", cause),
            RestoreAnalysisError::InternalFailure(ref cause) => write!(f, "{}", cause),
            RestoreAnalysisError::InvalidParameterValue(ref cause) => write!(f, "{}", cause),
            RestoreAnalysisError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            RestoreAnalysisError::Throttling(ref cause) => write!(f, "{}", cause),
            RestoreAnalysisError::UnsupportedUserEdition(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for RestoreAnalysisError {}
/// Errors returned by SearchAnalyses
#[derive(Debug, PartialEq)]
pub enum SearchAnalysesError {
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

impl SearchAnalysesError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<SearchAnalysesError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InternalFailureException" => {
                    return RusotoError::Service(SearchAnalysesError::InternalFailure(err.msg))
                }
                "InvalidNextTokenException" => {
                    return RusotoError::Service(SearchAnalysesError::InvalidNextToken(err.msg))
                }
                "InvalidParameterValueException" => {
                    return RusotoError::Service(SearchAnalysesError::InvalidParameterValue(
                        err.msg,
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(SearchAnalysesError::ResourceNotFound(err.msg))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(SearchAnalysesError::Throttling(err.msg))
                }
                "UnsupportedUserEditionException" => {
                    return RusotoError::Service(SearchAnalysesError::UnsupportedUserEdition(
                        err.msg,
                    ))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }

    fn refine(err: RusotoError<std::convert::Infallible>) -> RusotoError<SearchAnalysesError> {
        match err {
            RusotoError::Service(err) => match err {},
            RusotoError::HttpDispatch(err) => RusotoError::HttpDispatch(err),
            RusotoError::Credentials(err) => RusotoError::Credentials(err),
            RusotoError::Validation(err) => RusotoError::Validation(err),
            RusotoError::ParseError(err) => RusotoError::ParseError(err),
            RusotoError::Unknown(res) => Self::from_response(res),
            RusotoError::Blocking => RusotoError::Blocking,
        }
    }
}
impl fmt::Display for SearchAnalysesError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            SearchAnalysesError::InternalFailure(ref cause) => write!(f, "{}", cause),
            SearchAnalysesError::InvalidNextToken(ref cause) => write!(f, "{}", cause),
            SearchAnalysesError::InvalidParameterValue(ref cause) => write!(f, "{}", cause),
            SearchAnalysesError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            SearchAnalysesError::Throttling(ref cause) => write!(f, "{}", cause),
            SearchAnalysesError::UnsupportedUserEdition(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for SearchAnalysesError {}
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

    fn refine(err: RusotoError<std::convert::Infallible>) -> RusotoError<SearchDashboardsError> {
        match err {
            RusotoError::Service(err) => match err {},
            RusotoError::HttpDispatch(err) => RusotoError::HttpDispatch(err),
            RusotoError::Credentials(err) => RusotoError::Credentials(err),
            RusotoError::Validation(err) => RusotoError::Validation(err),
            RusotoError::ParseError(err) => RusotoError::ParseError(err),
            RusotoError::Unknown(res) => Self::from_response(res),
            RusotoError::Blocking => RusotoError::Blocking,
        }
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

    fn refine(err: RusotoError<std::convert::Infallible>) -> RusotoError<TagResourceError> {
        match err {
            RusotoError::Service(err) => match err {},
            RusotoError::HttpDispatch(err) => RusotoError::HttpDispatch(err),
            RusotoError::Credentials(err) => RusotoError::Credentials(err),
            RusotoError::Validation(err) => RusotoError::Validation(err),
            RusotoError::ParseError(err) => RusotoError::ParseError(err),
            RusotoError::Unknown(res) => Self::from_response(res),
            RusotoError::Blocking => RusotoError::Blocking,
        }
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

    fn refine(err: RusotoError<std::convert::Infallible>) -> RusotoError<UntagResourceError> {
        match err {
            RusotoError::Service(err) => match err {},
            RusotoError::HttpDispatch(err) => RusotoError::HttpDispatch(err),
            RusotoError::Credentials(err) => RusotoError::Credentials(err),
            RusotoError::Validation(err) => RusotoError::Validation(err),
            RusotoError::ParseError(err) => RusotoError::ParseError(err),
            RusotoError::Unknown(res) => Self::from_response(res),
            RusotoError::Blocking => RusotoError::Blocking,
        }
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
/// Errors returned by UpdateAccountCustomization
#[derive(Debug, PartialEq)]
pub enum UpdateAccountCustomizationError {
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

impl UpdateAccountCustomizationError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<UpdateAccountCustomizationError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(UpdateAccountCustomizationError::AccessDenied(
                        err.msg,
                    ))
                }
                "InternalFailureException" => {
                    return RusotoError::Service(UpdateAccountCustomizationError::InternalFailure(
                        err.msg,
                    ))
                }
                "InvalidParameterValueException" => {
                    return RusotoError::Service(
                        UpdateAccountCustomizationError::InvalidParameterValue(err.msg),
                    )
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(UpdateAccountCustomizationError::ResourceNotFound(
                        err.msg,
                    ))
                }
                "ResourceUnavailableException" => {
                    return RusotoError::Service(
                        UpdateAccountCustomizationError::ResourceUnavailable(err.msg),
                    )
                }
                "ThrottlingException" => {
                    return RusotoError::Service(UpdateAccountCustomizationError::Throttling(
                        err.msg,
                    ))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }

    fn refine(
        err: RusotoError<std::convert::Infallible>,
    ) -> RusotoError<UpdateAccountCustomizationError> {
        match err {
            RusotoError::Service(err) => match err {},
            RusotoError::HttpDispatch(err) => RusotoError::HttpDispatch(err),
            RusotoError::Credentials(err) => RusotoError::Credentials(err),
            RusotoError::Validation(err) => RusotoError::Validation(err),
            RusotoError::ParseError(err) => RusotoError::ParseError(err),
            RusotoError::Unknown(res) => Self::from_response(res),
            RusotoError::Blocking => RusotoError::Blocking,
        }
    }
}
impl fmt::Display for UpdateAccountCustomizationError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UpdateAccountCustomizationError::AccessDenied(ref cause) => write!(f, "{}", cause),
            UpdateAccountCustomizationError::InternalFailure(ref cause) => write!(f, "{}", cause),
            UpdateAccountCustomizationError::InvalidParameterValue(ref cause) => {
                write!(f, "{}", cause)
            }
            UpdateAccountCustomizationError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            UpdateAccountCustomizationError::ResourceUnavailable(ref cause) => {
                write!(f, "{}", cause)
            }
            UpdateAccountCustomizationError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for UpdateAccountCustomizationError {}
/// Errors returned by UpdateAccountSettings
#[derive(Debug, PartialEq)]
pub enum UpdateAccountSettingsError {
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

impl UpdateAccountSettingsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdateAccountSettingsError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(UpdateAccountSettingsError::AccessDenied(err.msg))
                }
                "InternalFailureException" => {
                    return RusotoError::Service(UpdateAccountSettingsError::InternalFailure(
                        err.msg,
                    ))
                }
                "InvalidParameterValueException" => {
                    return RusotoError::Service(UpdateAccountSettingsError::InvalidParameterValue(
                        err.msg,
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(UpdateAccountSettingsError::ResourceNotFound(
                        err.msg,
                    ))
                }
                "ResourceUnavailableException" => {
                    return RusotoError::Service(UpdateAccountSettingsError::ResourceUnavailable(
                        err.msg,
                    ))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(UpdateAccountSettingsError::Throttling(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }

    fn refine(
        err: RusotoError<std::convert::Infallible>,
    ) -> RusotoError<UpdateAccountSettingsError> {
        match err {
            RusotoError::Service(err) => match err {},
            RusotoError::HttpDispatch(err) => RusotoError::HttpDispatch(err),
            RusotoError::Credentials(err) => RusotoError::Credentials(err),
            RusotoError::Validation(err) => RusotoError::Validation(err),
            RusotoError::ParseError(err) => RusotoError::ParseError(err),
            RusotoError::Unknown(res) => Self::from_response(res),
            RusotoError::Blocking => RusotoError::Blocking,
        }
    }
}
impl fmt::Display for UpdateAccountSettingsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UpdateAccountSettingsError::AccessDenied(ref cause) => write!(f, "{}", cause),
            UpdateAccountSettingsError::InternalFailure(ref cause) => write!(f, "{}", cause),
            UpdateAccountSettingsError::InvalidParameterValue(ref cause) => write!(f, "{}", cause),
            UpdateAccountSettingsError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            UpdateAccountSettingsError::ResourceUnavailable(ref cause) => write!(f, "{}", cause),
            UpdateAccountSettingsError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for UpdateAccountSettingsError {}
/// Errors returned by UpdateAnalysis
#[derive(Debug, PartialEq)]
pub enum UpdateAnalysisError {
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

impl UpdateAnalysisError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdateAnalysisError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "ConflictException" => {
                    return RusotoError::Service(UpdateAnalysisError::Conflict(err.msg))
                }
                "InternalFailureException" => {
                    return RusotoError::Service(UpdateAnalysisError::InternalFailure(err.msg))
                }
                "InvalidParameterValueException" => {
                    return RusotoError::Service(UpdateAnalysisError::InvalidParameterValue(
                        err.msg,
                    ))
                }
                "ResourceExistsException" => {
                    return RusotoError::Service(UpdateAnalysisError::ResourceExists(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(UpdateAnalysisError::ResourceNotFound(err.msg))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(UpdateAnalysisError::Throttling(err.msg))
                }
                "UnsupportedUserEditionException" => {
                    return RusotoError::Service(UpdateAnalysisError::UnsupportedUserEdition(
                        err.msg,
                    ))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }

    fn refine(err: RusotoError<std::convert::Infallible>) -> RusotoError<UpdateAnalysisError> {
        match err {
            RusotoError::Service(err) => match err {},
            RusotoError::HttpDispatch(err) => RusotoError::HttpDispatch(err),
            RusotoError::Credentials(err) => RusotoError::Credentials(err),
            RusotoError::Validation(err) => RusotoError::Validation(err),
            RusotoError::ParseError(err) => RusotoError::ParseError(err),
            RusotoError::Unknown(res) => Self::from_response(res),
            RusotoError::Blocking => RusotoError::Blocking,
        }
    }
}
impl fmt::Display for UpdateAnalysisError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UpdateAnalysisError::Conflict(ref cause) => write!(f, "{}", cause),
            UpdateAnalysisError::InternalFailure(ref cause) => write!(f, "{}", cause),
            UpdateAnalysisError::InvalidParameterValue(ref cause) => write!(f, "{}", cause),
            UpdateAnalysisError::ResourceExists(ref cause) => write!(f, "{}", cause),
            UpdateAnalysisError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            UpdateAnalysisError::Throttling(ref cause) => write!(f, "{}", cause),
            UpdateAnalysisError::UnsupportedUserEdition(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for UpdateAnalysisError {}
/// Errors returned by UpdateAnalysisPermissions
#[derive(Debug, PartialEq)]
pub enum UpdateAnalysisPermissionsError {
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

impl UpdateAnalysisPermissionsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdateAnalysisPermissionsError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "ConflictException" => {
                    return RusotoError::Service(UpdateAnalysisPermissionsError::Conflict(err.msg))
                }
                "InternalFailureException" => {
                    return RusotoError::Service(UpdateAnalysisPermissionsError::InternalFailure(
                        err.msg,
                    ))
                }
                "InvalidParameterValueException" => {
                    return RusotoError::Service(
                        UpdateAnalysisPermissionsError::InvalidParameterValue(err.msg),
                    )
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(UpdateAnalysisPermissionsError::ResourceNotFound(
                        err.msg,
                    ))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(UpdateAnalysisPermissionsError::Throttling(
                        err.msg,
                    ))
                }
                "UnsupportedUserEditionException" => {
                    return RusotoError::Service(
                        UpdateAnalysisPermissionsError::UnsupportedUserEdition(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }

    fn refine(
        err: RusotoError<std::convert::Infallible>,
    ) -> RusotoError<UpdateAnalysisPermissionsError> {
        match err {
            RusotoError::Service(err) => match err {},
            RusotoError::HttpDispatch(err) => RusotoError::HttpDispatch(err),
            RusotoError::Credentials(err) => RusotoError::Credentials(err),
            RusotoError::Validation(err) => RusotoError::Validation(err),
            RusotoError::ParseError(err) => RusotoError::ParseError(err),
            RusotoError::Unknown(res) => Self::from_response(res),
            RusotoError::Blocking => RusotoError::Blocking,
        }
    }
}
impl fmt::Display for UpdateAnalysisPermissionsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UpdateAnalysisPermissionsError::Conflict(ref cause) => write!(f, "{}", cause),
            UpdateAnalysisPermissionsError::InternalFailure(ref cause) => write!(f, "{}", cause),
            UpdateAnalysisPermissionsError::InvalidParameterValue(ref cause) => {
                write!(f, "{}", cause)
            }
            UpdateAnalysisPermissionsError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            UpdateAnalysisPermissionsError::Throttling(ref cause) => write!(f, "{}", cause),
            UpdateAnalysisPermissionsError::UnsupportedUserEdition(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for UpdateAnalysisPermissionsError {}
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

    fn refine(err: RusotoError<std::convert::Infallible>) -> RusotoError<UpdateDashboardError> {
        match err {
            RusotoError::Service(err) => match err {},
            RusotoError::HttpDispatch(err) => RusotoError::HttpDispatch(err),
            RusotoError::Credentials(err) => RusotoError::Credentials(err),
            RusotoError::Validation(err) => RusotoError::Validation(err),
            RusotoError::ParseError(err) => RusotoError::ParseError(err),
            RusotoError::Unknown(res) => Self::from_response(res),
            RusotoError::Blocking => RusotoError::Blocking,
        }
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

    fn refine(
        err: RusotoError<std::convert::Infallible>,
    ) -> RusotoError<UpdateDashboardPermissionsError> {
        match err {
            RusotoError::Service(err) => match err {},
            RusotoError::HttpDispatch(err) => RusotoError::HttpDispatch(err),
            RusotoError::Credentials(err) => RusotoError::Credentials(err),
            RusotoError::Validation(err) => RusotoError::Validation(err),
            RusotoError::ParseError(err) => RusotoError::ParseError(err),
            RusotoError::Unknown(res) => Self::from_response(res),
            RusotoError::Blocking => RusotoError::Blocking,
        }
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

    fn refine(
        err: RusotoError<std::convert::Infallible>,
    ) -> RusotoError<UpdateDashboardPublishedVersionError> {
        match err {
            RusotoError::Service(err) => match err {},
            RusotoError::HttpDispatch(err) => RusotoError::HttpDispatch(err),
            RusotoError::Credentials(err) => RusotoError::Credentials(err),
            RusotoError::Validation(err) => RusotoError::Validation(err),
            RusotoError::ParseError(err) => RusotoError::ParseError(err),
            RusotoError::Unknown(res) => Self::from_response(res),
            RusotoError::Blocking => RusotoError::Blocking,
        }
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

    fn refine(err: RusotoError<std::convert::Infallible>) -> RusotoError<UpdateDataSetError> {
        match err {
            RusotoError::Service(err) => match err {},
            RusotoError::HttpDispatch(err) => RusotoError::HttpDispatch(err),
            RusotoError::Credentials(err) => RusotoError::Credentials(err),
            RusotoError::Validation(err) => RusotoError::Validation(err),
            RusotoError::ParseError(err) => RusotoError::ParseError(err),
            RusotoError::Unknown(res) => Self::from_response(res),
            RusotoError::Blocking => RusotoError::Blocking,
        }
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

    fn refine(
        err: RusotoError<std::convert::Infallible>,
    ) -> RusotoError<UpdateDataSetPermissionsError> {
        match err {
            RusotoError::Service(err) => match err {},
            RusotoError::HttpDispatch(err) => RusotoError::HttpDispatch(err),
            RusotoError::Credentials(err) => RusotoError::Credentials(err),
            RusotoError::Validation(err) => RusotoError::Validation(err),
            RusotoError::ParseError(err) => RusotoError::ParseError(err),
            RusotoError::Unknown(res) => Self::from_response(res),
            RusotoError::Blocking => RusotoError::Blocking,
        }
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

    fn refine(err: RusotoError<std::convert::Infallible>) -> RusotoError<UpdateDataSourceError> {
        match err {
            RusotoError::Service(err) => match err {},
            RusotoError::HttpDispatch(err) => RusotoError::HttpDispatch(err),
            RusotoError::Credentials(err) => RusotoError::Credentials(err),
            RusotoError::Validation(err) => RusotoError::Validation(err),
            RusotoError::ParseError(err) => RusotoError::ParseError(err),
            RusotoError::Unknown(res) => Self::from_response(res),
            RusotoError::Blocking => RusotoError::Blocking,
        }
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

    fn refine(
        err: RusotoError<std::convert::Infallible>,
    ) -> RusotoError<UpdateDataSourcePermissionsError> {
        match err {
            RusotoError::Service(err) => match err {},
            RusotoError::HttpDispatch(err) => RusotoError::HttpDispatch(err),
            RusotoError::Credentials(err) => RusotoError::Credentials(err),
            RusotoError::Validation(err) => RusotoError::Validation(err),
            RusotoError::ParseError(err) => RusotoError::ParseError(err),
            RusotoError::Unknown(res) => Self::from_response(res),
            RusotoError::Blocking => RusotoError::Blocking,
        }
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

    fn refine(err: RusotoError<std::convert::Infallible>) -> RusotoError<UpdateGroupError> {
        match err {
            RusotoError::Service(err) => match err {},
            RusotoError::HttpDispatch(err) => RusotoError::HttpDispatch(err),
            RusotoError::Credentials(err) => RusotoError::Credentials(err),
            RusotoError::Validation(err) => RusotoError::Validation(err),
            RusotoError::ParseError(err) => RusotoError::ParseError(err),
            RusotoError::Unknown(res) => Self::from_response(res),
            RusotoError::Blocking => RusotoError::Blocking,
        }
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
    /// <p>A resource is already in a state that indicates an operation is happening that must complete before a new update can be applied.</p>
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

    fn refine(
        err: RusotoError<std::convert::Infallible>,
    ) -> RusotoError<UpdateIAMPolicyAssignmentError> {
        match err {
            RusotoError::Service(err) => match err {},
            RusotoError::HttpDispatch(err) => RusotoError::HttpDispatch(err),
            RusotoError::Credentials(err) => RusotoError::Credentials(err),
            RusotoError::Validation(err) => RusotoError::Validation(err),
            RusotoError::ParseError(err) => RusotoError::ParseError(err),
            RusotoError::Unknown(res) => Self::from_response(res),
            RusotoError::Blocking => RusotoError::Blocking,
        }
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

    fn refine(err: RusotoError<std::convert::Infallible>) -> RusotoError<UpdateTemplateError> {
        match err {
            RusotoError::Service(err) => match err {},
            RusotoError::HttpDispatch(err) => RusotoError::HttpDispatch(err),
            RusotoError::Credentials(err) => RusotoError::Credentials(err),
            RusotoError::Validation(err) => RusotoError::Validation(err),
            RusotoError::ParseError(err) => RusotoError::ParseError(err),
            RusotoError::Unknown(res) => Self::from_response(res),
            RusotoError::Blocking => RusotoError::Blocking,
        }
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

    fn refine(err: RusotoError<std::convert::Infallible>) -> RusotoError<UpdateTemplateAliasError> {
        match err {
            RusotoError::Service(err) => match err {},
            RusotoError::HttpDispatch(err) => RusotoError::HttpDispatch(err),
            RusotoError::Credentials(err) => RusotoError::Credentials(err),
            RusotoError::Validation(err) => RusotoError::Validation(err),
            RusotoError::ParseError(err) => RusotoError::ParseError(err),
            RusotoError::Unknown(res) => Self::from_response(res),
            RusotoError::Blocking => RusotoError::Blocking,
        }
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

    fn refine(
        err: RusotoError<std::convert::Infallible>,
    ) -> RusotoError<UpdateTemplatePermissionsError> {
        match err {
            RusotoError::Service(err) => match err {},
            RusotoError::HttpDispatch(err) => RusotoError::HttpDispatch(err),
            RusotoError::Credentials(err) => RusotoError::Credentials(err),
            RusotoError::Validation(err) => RusotoError::Validation(err),
            RusotoError::ParseError(err) => RusotoError::ParseError(err),
            RusotoError::Unknown(res) => Self::from_response(res),
            RusotoError::Blocking => RusotoError::Blocking,
        }
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
/// Errors returned by UpdateTheme
#[derive(Debug, PartialEq)]
pub enum UpdateThemeError {
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

impl UpdateThemeError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdateThemeError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(UpdateThemeError::AccessDenied(err.msg))
                }
                "InternalFailureException" => {
                    return RusotoError::Service(UpdateThemeError::InternalFailure(err.msg))
                }
                "InvalidParameterValueException" => {
                    return RusotoError::Service(UpdateThemeError::InvalidParameterValue(err.msg))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(UpdateThemeError::LimitExceeded(err.msg))
                }
                "ResourceExistsException" => {
                    return RusotoError::Service(UpdateThemeError::ResourceExists(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(UpdateThemeError::ResourceNotFound(err.msg))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(UpdateThemeError::Throttling(err.msg))
                }
                "UnsupportedUserEditionException" => {
                    return RusotoError::Service(UpdateThemeError::UnsupportedUserEdition(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }

    fn refine(err: RusotoError<std::convert::Infallible>) -> RusotoError<UpdateThemeError> {
        match err {
            RusotoError::Service(err) => match err {},
            RusotoError::HttpDispatch(err) => RusotoError::HttpDispatch(err),
            RusotoError::Credentials(err) => RusotoError::Credentials(err),
            RusotoError::Validation(err) => RusotoError::Validation(err),
            RusotoError::ParseError(err) => RusotoError::ParseError(err),
            RusotoError::Unknown(res) => Self::from_response(res),
            RusotoError::Blocking => RusotoError::Blocking,
        }
    }
}
impl fmt::Display for UpdateThemeError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UpdateThemeError::AccessDenied(ref cause) => write!(f, "{}", cause),
            UpdateThemeError::InternalFailure(ref cause) => write!(f, "{}", cause),
            UpdateThemeError::InvalidParameterValue(ref cause) => write!(f, "{}", cause),
            UpdateThemeError::LimitExceeded(ref cause) => write!(f, "{}", cause),
            UpdateThemeError::ResourceExists(ref cause) => write!(f, "{}", cause),
            UpdateThemeError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            UpdateThemeError::Throttling(ref cause) => write!(f, "{}", cause),
            UpdateThemeError::UnsupportedUserEdition(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for UpdateThemeError {}
/// Errors returned by UpdateThemeAlias
#[derive(Debug, PartialEq)]
pub enum UpdateThemeAliasError {
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

impl UpdateThemeAliasError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdateThemeAliasError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "ConflictException" => {
                    return RusotoError::Service(UpdateThemeAliasError::Conflict(err.msg))
                }
                "InternalFailureException" => {
                    return RusotoError::Service(UpdateThemeAliasError::InternalFailure(err.msg))
                }
                "InvalidParameterValueException" => {
                    return RusotoError::Service(UpdateThemeAliasError::InvalidParameterValue(
                        err.msg,
                    ))
                }
                "ResourceExistsException" => {
                    return RusotoError::Service(UpdateThemeAliasError::ResourceExists(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(UpdateThemeAliasError::ResourceNotFound(err.msg))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(UpdateThemeAliasError::Throttling(err.msg))
                }
                "UnsupportedUserEditionException" => {
                    return RusotoError::Service(UpdateThemeAliasError::UnsupportedUserEdition(
                        err.msg,
                    ))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }

    fn refine(err: RusotoError<std::convert::Infallible>) -> RusotoError<UpdateThemeAliasError> {
        match err {
            RusotoError::Service(err) => match err {},
            RusotoError::HttpDispatch(err) => RusotoError::HttpDispatch(err),
            RusotoError::Credentials(err) => RusotoError::Credentials(err),
            RusotoError::Validation(err) => RusotoError::Validation(err),
            RusotoError::ParseError(err) => RusotoError::ParseError(err),
            RusotoError::Unknown(res) => Self::from_response(res),
            RusotoError::Blocking => RusotoError::Blocking,
        }
    }
}
impl fmt::Display for UpdateThemeAliasError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UpdateThemeAliasError::Conflict(ref cause) => write!(f, "{}", cause),
            UpdateThemeAliasError::InternalFailure(ref cause) => write!(f, "{}", cause),
            UpdateThemeAliasError::InvalidParameterValue(ref cause) => write!(f, "{}", cause),
            UpdateThemeAliasError::ResourceExists(ref cause) => write!(f, "{}", cause),
            UpdateThemeAliasError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            UpdateThemeAliasError::Throttling(ref cause) => write!(f, "{}", cause),
            UpdateThemeAliasError::UnsupportedUserEdition(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for UpdateThemeAliasError {}
/// Errors returned by UpdateThemePermissions
#[derive(Debug, PartialEq)]
pub enum UpdateThemePermissionsError {
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

impl UpdateThemePermissionsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdateThemePermissionsError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(UpdateThemePermissionsError::AccessDenied(err.msg))
                }
                "InternalFailureException" => {
                    return RusotoError::Service(UpdateThemePermissionsError::InternalFailure(
                        err.msg,
                    ))
                }
                "InvalidParameterValueException" => {
                    return RusotoError::Service(
                        UpdateThemePermissionsError::InvalidParameterValue(err.msg),
                    )
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(UpdateThemePermissionsError::ResourceNotFound(
                        err.msg,
                    ))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(UpdateThemePermissionsError::Throttling(err.msg))
                }
                "UnsupportedUserEditionException" => {
                    return RusotoError::Service(
                        UpdateThemePermissionsError::UnsupportedUserEdition(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }

    fn refine(
        err: RusotoError<std::convert::Infallible>,
    ) -> RusotoError<UpdateThemePermissionsError> {
        match err {
            RusotoError::Service(err) => match err {},
            RusotoError::HttpDispatch(err) => RusotoError::HttpDispatch(err),
            RusotoError::Credentials(err) => RusotoError::Credentials(err),
            RusotoError::Validation(err) => RusotoError::Validation(err),
            RusotoError::ParseError(err) => RusotoError::ParseError(err),
            RusotoError::Unknown(res) => Self::from_response(res),
            RusotoError::Blocking => RusotoError::Blocking,
        }
    }
}
impl fmt::Display for UpdateThemePermissionsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UpdateThemePermissionsError::AccessDenied(ref cause) => write!(f, "{}", cause),
            UpdateThemePermissionsError::InternalFailure(ref cause) => write!(f, "{}", cause),
            UpdateThemePermissionsError::InvalidParameterValue(ref cause) => write!(f, "{}", cause),
            UpdateThemePermissionsError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            UpdateThemePermissionsError::Throttling(ref cause) => write!(f, "{}", cause),
            UpdateThemePermissionsError::UnsupportedUserEdition(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for UpdateThemePermissionsError {}
/// Errors returned by UpdateUser
#[derive(Debug, PartialEq)]
pub enum UpdateUserError {
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
                "PreconditionNotMetException" => {
                    return RusotoError::Service(UpdateUserError::PreconditionNotMet(err.msg))
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

    fn refine(err: RusotoError<std::convert::Infallible>) -> RusotoError<UpdateUserError> {
        match err {
            RusotoError::Service(err) => match err {},
            RusotoError::HttpDispatch(err) => RusotoError::HttpDispatch(err),
            RusotoError::Credentials(err) => RusotoError::Credentials(err),
            RusotoError::Validation(err) => RusotoError::Validation(err),
            RusotoError::ParseError(err) => RusotoError::ParseError(err),
            RusotoError::Unknown(res) => Self::from_response(res),
            RusotoError::Blocking => RusotoError::Blocking,
        }
    }
}
impl fmt::Display for UpdateUserError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UpdateUserError::AccessDenied(ref cause) => write!(f, "{}", cause),
            UpdateUserError::InternalFailure(ref cause) => write!(f, "{}", cause),
            UpdateUserError::InvalidParameterValue(ref cause) => write!(f, "{}", cause),
            UpdateUserError::PreconditionNotMet(ref cause) => write!(f, "{}", cause),
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

    /// <p>Creates Amazon QuickSight customizations the current AWS Region. Currently, you can add a custom default theme by using the <code>CreateAccountCustomization</code> or <code>UpdateAccountCustomization</code> API operation. To further customize QuickSight by removing QuickSight sample assets and videos for all new users, see <a href="https://docs.aws.amazon.com/quicksight/latest/user/customizing-quicksight.html">Customizing QuickSight</a> in the <i>Amazon QuickSight User Guide.</i> </p> <p>You can create customizations for your AWS account or, if you specify a namespace, for a QuickSight namespace instead. Customizations that apply to a namespace always override customizations that apply to an AWS account. To find out which customizations apply, use the <code>DescribeAccountCustomization</code> API operation.</p> <p>Before you use the <code>CreateAccountCustomization</code> API operation to add a theme as the namespace default, make sure that you first share the theme with the namespace. If you don't share it with the namespace, the theme isn't visible to your users even if you make it the default theme. To check if the theme is shared, view the current permissions by using the <code> <a>DescribeThemePermissions</a> </code> API operation. To share the theme, grant permissions by using the <code> <a>UpdateThemePermissions</a> </code> API operation. </p>
    async fn create_account_customization(
        &self,
        input: CreateAccountCustomizationRequest,
    ) -> Result<CreateAccountCustomizationResponse, RusotoError<CreateAccountCustomizationError>>;

    /// <p>Creates an analysis in Amazon QuickSight.</p>
    async fn create_analysis(
        &self,
        input: CreateAnalysisRequest,
    ) -> Result<CreateAnalysisResponse, RusotoError<CreateAnalysisError>>;

    /// <p>Creates a dashboard from a template. To first create a template, see the <code> <a>CreateTemplate</a> </code> API operation.</p> <p>A dashboard is an entity in QuickSight that identifies QuickSight reports, created from analyses. You can share QuickSight dashboards. With the right permissions, you can create scheduled email reports from them. If you have the correct permissions, you can create a dashboard from a template that exists in a different AWS account.</p>
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

    /// <p>Creates an assignment with one specified IAM policy, identified by its Amazon Resource Name (ARN). This policy assignment is attached to the specified groups or users of Amazon QuickSight. Assignment names are unique per AWS account. To avoid overwriting rules in other namespaces, use assignment names that are unique.</p>
    async fn create_iam_policy_assignment(
        &self,
        input: CreateIAMPolicyAssignmentRequest,
    ) -> Result<CreateIAMPolicyAssignmentResponse, RusotoError<CreateIAMPolicyAssignmentError>>;

    /// <p>Creates and starts a new SPICE ingestion on a dataset</p> <p>Any ingestions operating on tagged datasets inherit the same tags automatically for use in access control. For an example, see <a href="http://aws.amazon.com/premiumsupport/knowledge-center/iam-ec2-resource-tags/">How do I create an IAM policy to control access to Amazon EC2 resources using tags?</a> in the AWS Knowledge Center. Tags are visible on the tagged dataset, but not on the ingestion resource.</p>
    async fn create_ingestion(
        &self,
        input: CreateIngestionRequest,
    ) -> Result<CreateIngestionResponse, RusotoError<CreateIngestionError>>;

    /// <p>(Enterprise edition only) Creates a new namespace for you to use with Amazon QuickSight.</p> <p>A namespace allows you to isolate the QuickSight users and groups that are registered for that namespace. Users that access the namespace can share assets only with other users or groups in the same namespace. They can't see users and groups in other namespaces. You can create a namespace after your AWS account is subscribed to QuickSight. The namespace must be unique within the AWS account. By default, there is a limit of 100 namespaces per AWS account. To increase your limit, create a ticket with AWS Support. </p>
    async fn create_namespace(
        &self,
        input: CreateNamespaceRequest,
    ) -> Result<CreateNamespaceResponse, RusotoError<CreateNamespaceError>>;

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

    /// <p>Creates a theme.</p> <p>A <i>theme</i> is set of configuration options for color and layout. Themes apply to analyses and dashboards. For more information, see <a href="https://docs.aws.amazon.com/quicksight/latest/user/themes-in-quicksight.html">Using Themes in Amazon QuickSight</a> in the <i>Amazon QuickSight User Guide</i>.</p>
    async fn create_theme(
        &self,
        input: CreateThemeRequest,
    ) -> Result<CreateThemeResponse, RusotoError<CreateThemeError>>;

    /// <p>Creates a theme alias for a theme.</p>
    async fn create_theme_alias(
        &self,
        input: CreateThemeAliasRequest,
    ) -> Result<CreateThemeAliasResponse, RusotoError<CreateThemeAliasError>>;

    /// <p>Deletes all Amazon QuickSight customizations in this AWS Region for the specified AWS account and QuickSight namespace.</p>
    async fn delete_account_customization(
        &self,
        input: DeleteAccountCustomizationRequest,
    ) -> Result<DeleteAccountCustomizationResponse, RusotoError<DeleteAccountCustomizationError>>;

    /// <p>Deletes an analysis from Amazon QuickSight. You can optionally include a recovery window during which you can restore the analysis. If you don't specify a recovery window value, the operation defaults to 30 days. QuickSight attaches a <code>DeletionTime</code> stamp to the response that specifies the end of the recovery window. At the end of the recovery window, QuickSight deletes the analysis permanently.</p> <p>At any time before recovery window ends, you can use the <code>RestoreAnalysis</code> API operation to remove the <code>DeletionTime</code> stamp and cancel the deletion of the analysis. The analysis remains visible in the API until it's deleted, so you can describe it but you can't make a template from it.</p> <p>An analysis that's scheduled for deletion isn't accessible in the QuickSight console. To access it in the console, restore it. Deleting an analysis doesn't delete the dashboards that you publish from it.</p>
    async fn delete_analysis(
        &self,
        input: DeleteAnalysisRequest,
    ) -> Result<DeleteAnalysisResponse, RusotoError<DeleteAnalysisError>>;

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

    /// <p>Deletes the data source permanently. This operation breaks all the datasets that reference the deleted data source.</p>
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

    /// <p>Deletes a namespace and the users and groups that are associated with the namespace. This is an asynchronous process. Assets including dashboards, analyses, datasets and data sources are not deleted. To delete these assets, you use the API operations for the relevant asset. </p>
    async fn delete_namespace(
        &self,
        input: DeleteNamespaceRequest,
    ) -> Result<DeleteNamespaceResponse, RusotoError<DeleteNamespaceError>>;

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

    /// <p>Deletes a theme.</p>
    async fn delete_theme(
        &self,
        input: DeleteThemeRequest,
    ) -> Result<DeleteThemeResponse, RusotoError<DeleteThemeError>>;

    /// <p>Deletes the version of the theme that the specified theme alias points to. If you provide a specific alias, you delete the version of the theme that the alias points to.</p>
    async fn delete_theme_alias(
        &self,
        input: DeleteThemeAliasRequest,
    ) -> Result<DeleteThemeAliasResponse, RusotoError<DeleteThemeAliasError>>;

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

    /// <p><p>Describes the customizations associated with the provided AWS account and Amazon QuickSight namespace in an AWS Region. The QuickSight console evaluates which customizations to apply by running this API operation with the <code>Resolved</code> flag included. </p> <p>To determine what customizations display when you run this command, it can help to visualize the relationship of the entities involved. </p> <ul> <li> <p> <code>AWS Account</code> - The AWS account exists at the top of the hierarchy. It has the potential to use all of the AWS Regions and AWS Services. When you subscribe to QuickSight, you choose one AWS Region to use as your home Region. That&#39;s where your free SPICE capacity is located. You can use QuickSight in any supported AWS Region. </p> </li> <li> <p> <code>AWS Region</code> - In each AWS Region where you sign in to QuickSight at least once, QuickSight acts as a separate instance of the same service. If you have a user directory, it resides in us-east-1, which is the US East (N. Virginia). Generally speaking, these users have access to QuickSight in any AWS Region, unless they are constrained to a namespace. </p> <p>To run the command in a different AWS Region, you change your Region settings. If you&#39;re using the AWS CLI, you can use one of the following options:</p> <ul> <li> <p>Use <a href="https://docs.aws.amazon.com/cli/latest/userguide/cli-configure-options.html">command line options</a>. </p> </li> <li> <p>Use <a href="https://docs.aws.amazon.com/cli/latest/userguide/cli-configure-profiles.html">named profiles</a>. </p> </li> <li> <p>Run <code>aws configure</code> to change your default AWS Region. Use Enter to key the same settings for your keys. For more information, see <a href="https://docs.aws.amazon.com/cli/latest/userguide/cli-chap-configure.html">Configuring the AWS CLI</a>.</p> </li> </ul> </li> <li> <p> <code>Namespace</code> - A QuickSight namespace is a partition that contains users and assets (data sources, datasets, dashboards, and so on). To access assets that are in a specific namespace, users and groups must also be part of the same namespace. People who share a namespace are completely isolated from users and assets in other namespaces, even if they are in the same AWS account and AWS Region.</p> </li> <li> <p> <code>Applied customizations</code> - Within an AWS Region, a set of QuickSight customizations can apply to an AWS account or to a namespace. Settings that you apply to a namespace override settings that you apply to an AWS account. All settings are isolated to a single AWS Region. To apply them in other AWS Regions, run the <code>CreateAccountCustomization</code> command in each AWS Region where you want to apply the same customizations. </p> </li> </ul></p>
    async fn describe_account_customization(
        &self,
        input: DescribeAccountCustomizationRequest,
    ) -> Result<DescribeAccountCustomizationResponse, RusotoError<DescribeAccountCustomizationError>>;

    /// <p>Describes the settings that were used when your QuickSight subscription was first created in this AWS account.</p>
    async fn describe_account_settings(
        &self,
        input: DescribeAccountSettingsRequest,
    ) -> Result<DescribeAccountSettingsResponse, RusotoError<DescribeAccountSettingsError>>;

    /// <p>Provides a summary of the metadata for an analysis.</p>
    async fn describe_analysis(
        &self,
        input: DescribeAnalysisRequest,
    ) -> Result<DescribeAnalysisResponse, RusotoError<DescribeAnalysisError>>;

    /// <p>Provides the read and write permissions for an analysis.</p>
    async fn describe_analysis_permissions(
        &self,
        input: DescribeAnalysisPermissionsRequest,
    ) -> Result<DescribeAnalysisPermissionsResponse, RusotoError<DescribeAnalysisPermissionsError>>;

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

    /// <p>Describes the current namespace.</p>
    async fn describe_namespace(
        &self,
        input: DescribeNamespaceRequest,
    ) -> Result<DescribeNamespaceResponse, RusotoError<DescribeNamespaceError>>;

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

    /// <p>Describes a theme.</p>
    async fn describe_theme(
        &self,
        input: DescribeThemeRequest,
    ) -> Result<DescribeThemeResponse, RusotoError<DescribeThemeError>>;

    /// <p>Describes the alias for a theme.</p>
    async fn describe_theme_alias(
        &self,
        input: DescribeThemeAliasRequest,
    ) -> Result<DescribeThemeAliasResponse, RusotoError<DescribeThemeAliasError>>;

    /// <p>Describes the read and write permissions for a theme.</p>
    async fn describe_theme_permissions(
        &self,
        input: DescribeThemePermissionsRequest,
    ) -> Result<DescribeThemePermissionsResponse, RusotoError<DescribeThemePermissionsError>>;

    /// <p>Returns information about a user, given the user name. </p>
    async fn describe_user(
        &self,
        input: DescribeUserRequest,
    ) -> Result<DescribeUserResponse, RusotoError<DescribeUserError>>;

    /// <p>Generates a session URL and authorization code that you can use to embed an Amazon QuickSight read-only dashboard in your web server code. Before you use this command, make sure that you have configured the dashboards and permissions. </p> <p>Currently, you can use <code>GetDashboardEmbedURL</code> only from the server, not from the user's browser. The following rules apply to the combination of URL and authorization code:</p> <ul> <li> <p>They must be used together.</p> </li> <li> <p>They can be used one time only.</p> </li> <li> <p>They are valid for 5 minutes after you run this command.</p> </li> <li> <p>The resulting user session is valid for 10 hours.</p> </li> </ul> <p>For more information, see <a href="https://docs.aws.amazon.com/quicksight/latest/user/embedded-analytics.html">Embedded Analytics</a> in the <i>Amazon QuickSight User Guide</i>.</p>
    async fn get_dashboard_embed_url(
        &self,
        input: GetDashboardEmbedUrlRequest,
    ) -> Result<GetDashboardEmbedUrlResponse, RusotoError<GetDashboardEmbedUrlError>>;

    /// <p><p>Generates a session URL and authorization code that you can use to embed the Amazon QuickSight console in your web server code. Use <code>GetSessionEmbedUrl</code> where you want to provide an authoring portal that allows users to create data sources, datasets, analyses, and dashboards. The users who access an embedded QuickSight console need belong to the author or admin security cohort. If you want to restrict permissions to some of these features, add a custom permissions profile to the user with the <code> <a>UpdateUser</a> </code> API operation. Use <code> <a>RegisterUser</a> </code> API operation to add a new user with a custom permission profile attached. For more information, see the following sections in the <i>Amazon QuickSight User Guide</i>:</p> <ul> <li> <p> <a href="https://docs.aws.amazon.com/quicksight/latest/user/embedding-the-quicksight-console.html">Embedding the Amazon QuickSight Console</a> </p> </li> <li> <p> <a href="https://docs.aws.amazon.com/quicksight/latest/user/customizing-permissions-to-the-quicksight-console.html">Customizing Access to the Amazon QuickSight Console</a> </p> </li> </ul></p>
    async fn get_session_embed_url(
        &self,
        input: GetSessionEmbedUrlRequest,
    ) -> Result<GetSessionEmbedUrlResponse, RusotoError<GetSessionEmbedUrlError>>;

    /// <p>Lists Amazon QuickSight analyses that exist in the specified AWS account.</p>
    async fn list_analyses(
        &self,
        input: ListAnalysesRequest,
    ) -> Result<ListAnalysesResponse, RusotoError<ListAnalysesError>>;

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

    /// <p>Lists the namespaces for the specified AWS account.</p>
    async fn list_namespaces(
        &self,
        input: ListNamespacesRequest,
    ) -> Result<ListNamespacesResponse, RusotoError<ListNamespacesError>>;

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

    /// <p>Lists all the aliases of a theme.</p>
    async fn list_theme_aliases(
        &self,
        input: ListThemeAliasesRequest,
    ) -> Result<ListThemeAliasesResponse, RusotoError<ListThemeAliasesError>>;

    /// <p>Lists all the versions of the themes in the current AWS account.</p>
    async fn list_theme_versions(
        &self,
        input: ListThemeVersionsRequest,
    ) -> Result<ListThemeVersionsResponse, RusotoError<ListThemeVersionsError>>;

    /// <p>Lists all the themes in the current AWS account.</p>
    async fn list_themes(
        &self,
        input: ListThemesRequest,
    ) -> Result<ListThemesResponse, RusotoError<ListThemesError>>;

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

    /// <p>Restores an analysis.</p>
    async fn restore_analysis(
        &self,
        input: RestoreAnalysisRequest,
    ) -> Result<RestoreAnalysisResponse, RusotoError<RestoreAnalysisError>>;

    /// <p>Searches for analyses that belong to the user specified in the filter.</p>
    async fn search_analyses(
        &self,
        input: SearchAnalysesRequest,
    ) -> Result<SearchAnalysesResponse, RusotoError<SearchAnalysesError>>;

    /// <p>Searches for dashboards that belong to a user. </p>
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

    /// <p>Updates Amazon QuickSight customizations the current AWS Region. Currently, the only customization you can use is a theme.</p> <p>You can use customizations for your AWS account or, if you specify a namespace, for a QuickSight namespace instead. Customizations that apply to a namespace override customizations that apply to an AWS account. To find out which customizations apply, use the <code>DescribeAccountCustomization</code> API operation. </p>
    async fn update_account_customization(
        &self,
        input: UpdateAccountCustomizationRequest,
    ) -> Result<UpdateAccountCustomizationResponse, RusotoError<UpdateAccountCustomizationError>>;

    /// <p>Updates the Amazon QuickSight settings in your AWS account.</p>
    async fn update_account_settings(
        &self,
        input: UpdateAccountSettingsRequest,
    ) -> Result<UpdateAccountSettingsResponse, RusotoError<UpdateAccountSettingsError>>;

    /// <p>Updates an analysis in Amazon QuickSight</p>
    async fn update_analysis(
        &self,
        input: UpdateAnalysisRequest,
    ) -> Result<UpdateAnalysisResponse, RusotoError<UpdateAnalysisError>>;

    /// <p>Updates the read and write permissions for an analysis.</p>
    async fn update_analysis_permissions(
        &self,
        input: UpdateAnalysisPermissionsRequest,
    ) -> Result<UpdateAnalysisPermissionsResponse, RusotoError<UpdateAnalysisPermissionsError>>;

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

    /// <p>Updates an existing IAM policy assignment. This operation updates only the optional parameter or parameters that are specified in the request. This overwrites all of the users included in <code>Identities</code>. </p>
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

    /// <p>Updates a theme.</p>
    async fn update_theme(
        &self,
        input: UpdateThemeRequest,
    ) -> Result<UpdateThemeResponse, RusotoError<UpdateThemeError>>;

    /// <p>Updates an alias of a theme.</p>
    async fn update_theme_alias(
        &self,
        input: UpdateThemeAliasRequest,
    ) -> Result<UpdateThemeAliasResponse, RusotoError<UpdateThemeAliasError>>;

    /// <p><p>Updates the resource permissions for a theme. Permissions apply to the action to grant or revoke permissions on, for example <code>&quot;quicksight:DescribeTheme&quot;</code>.</p> <p>Theme permissions apply in groupings. Valid groupings include the following for the three levels of permissions, which are user, owner, or no permissions: </p> <ul> <li> <p>User</p> <ul> <li> <p> <code>&quot;quicksight:DescribeTheme&quot;</code> </p> </li> <li> <p> <code>&quot;quicksight:DescribeThemeAlias&quot;</code> </p> </li> <li> <p> <code>&quot;quicksight:ListThemeAliases&quot;</code> </p> </li> <li> <p> <code>&quot;quicksight:ListThemeVersions&quot;</code> </p> </li> </ul> </li> <li> <p>Owner</p> <ul> <li> <p> <code>&quot;quicksight:DescribeTheme&quot;</code> </p> </li> <li> <p> <code>&quot;quicksight:DescribeThemeAlias&quot;</code> </p> </li> <li> <p> <code>&quot;quicksight:ListThemeAliases&quot;</code> </p> </li> <li> <p> <code>&quot;quicksight:ListThemeVersions&quot;</code> </p> </li> <li> <p> <code>&quot;quicksight:DeleteTheme&quot;</code> </p> </li> <li> <p> <code>&quot;quicksight:UpdateTheme&quot;</code> </p> </li> <li> <p> <code>&quot;quicksight:CreateThemeAlias&quot;</code> </p> </li> <li> <p> <code>&quot;quicksight:DeleteThemeAlias&quot;</code> </p> </li> <li> <p> <code>&quot;quicksight:UpdateThemeAlias&quot;</code> </p> </li> <li> <p> <code>&quot;quicksight:UpdateThemePermissions&quot;</code> </p> </li> <li> <p> <code>&quot;quicksight:DescribeThemePermissions&quot;</code> </p> </li> </ul> </li> <li> <p>To specify no permissions, omit the permissions list.</p> </li> </ul></p>
    async fn update_theme_permissions(
        &self,
        input: UpdateThemePermissionsRequest,
    ) -> Result<UpdateThemePermissionsResponse, RusotoError<UpdateThemePermissionsError>>;

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
    #[allow(unused_mut)]
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
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let mut result = proto::json::ResponsePayload::new(&response)
                .deserialize::<CancelIngestionResponse, _>()?;

            result.status = Some(response.status.as_u16() as i64);
            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(CancelIngestionError::from_response(response))
        }
    }

    /// <p>Creates Amazon QuickSight customizations the current AWS Region. Currently, you can add a custom default theme by using the <code>CreateAccountCustomization</code> or <code>UpdateAccountCustomization</code> API operation. To further customize QuickSight by removing QuickSight sample assets and videos for all new users, see <a href="https://docs.aws.amazon.com/quicksight/latest/user/customizing-quicksight.html">Customizing QuickSight</a> in the <i>Amazon QuickSight User Guide.</i> </p> <p>You can create customizations for your AWS account or, if you specify a namespace, for a QuickSight namespace instead. Customizations that apply to a namespace always override customizations that apply to an AWS account. To find out which customizations apply, use the <code>DescribeAccountCustomization</code> API operation.</p> <p>Before you use the <code>CreateAccountCustomization</code> API operation to add a theme as the namespace default, make sure that you first share the theme with the namespace. If you don't share it with the namespace, the theme isn't visible to your users even if you make it the default theme. To check if the theme is shared, view the current permissions by using the <code> <a>DescribeThemePermissions</a> </code> API operation. To share the theme, grant permissions by using the <code> <a>UpdateThemePermissions</a> </code> API operation. </p>
    #[allow(unused_mut)]
    async fn create_account_customization(
        &self,
        input: CreateAccountCustomizationRequest,
    ) -> Result<CreateAccountCustomizationResponse, RusotoError<CreateAccountCustomizationError>>
    {
        let request_uri = format!(
            "/accounts/{aws_account_id}/customizations",
            aws_account_id = input.aws_account_id
        );

        let mut request = SignedRequest::new("POST", "quicksight", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut params = Params::new();
        if let Some(ref x) = input.namespace {
            params.put("namespace", x);
        }
        request.set_params(params);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let mut result = proto::json::ResponsePayload::new(&response)
                .deserialize::<CreateAccountCustomizationResponse, _>()?;

            result.status = Some(response.status.as_u16() as i64);
            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(CreateAccountCustomizationError::from_response(response))
        }
    }

    /// <p>Creates an analysis in Amazon QuickSight.</p>
    #[allow(unused_mut)]
    async fn create_analysis(
        &self,
        input: CreateAnalysisRequest,
    ) -> Result<CreateAnalysisResponse, RusotoError<CreateAnalysisError>> {
        let request_uri = format!(
            "/accounts/{aws_account_id}/analyses/{analysis_id}",
            analysis_id = input.analysis_id,
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
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let mut result = proto::json::ResponsePayload::new(&response)
                .deserialize::<CreateAnalysisResponse, _>()?;

            result.status = Some(response.status.as_u16() as i64);
            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(CreateAnalysisError::from_response(response))
        }
    }

    /// <p>Creates a dashboard from a template. To first create a template, see the <code> <a>CreateTemplate</a> </code> API operation.</p> <p>A dashboard is an entity in QuickSight that identifies QuickSight reports, created from analyses. You can share QuickSight dashboards. With the right permissions, you can create scheduled email reports from them. If you have the correct permissions, you can create a dashboard from a template that exists in a different AWS account.</p>
    #[allow(unused_mut)]
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
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
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
    #[allow(unused_mut)]
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
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
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
    #[allow(unused_mut)]
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
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
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
    #[allow(unused_mut)]
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
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
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
    #[allow(unused_mut)]
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
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let mut result = proto::json::ResponsePayload::new(&response)
                .deserialize::<CreateGroupMembershipResponse, _>()?;

            result.status = Some(response.status.as_u16() as i64);
            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(CreateGroupMembershipError::from_response(response))
        }
    }

    /// <p>Creates an assignment with one specified IAM policy, identified by its Amazon Resource Name (ARN). This policy assignment is attached to the specified groups or users of Amazon QuickSight. Assignment names are unique per AWS account. To avoid overwriting rules in other namespaces, use assignment names that are unique.</p>
    #[allow(unused_mut)]
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
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let mut result = proto::json::ResponsePayload::new(&response)
                .deserialize::<CreateIAMPolicyAssignmentResponse, _>()?;

            result.status = Some(response.status.as_u16() as i64);
            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(CreateIAMPolicyAssignmentError::from_response(response))
        }
    }

    /// <p>Creates and starts a new SPICE ingestion on a dataset</p> <p>Any ingestions operating on tagged datasets inherit the same tags automatically for use in access control. For an example, see <a href="http://aws.amazon.com/premiumsupport/knowledge-center/iam-ec2-resource-tags/">How do I create an IAM policy to control access to Amazon EC2 resources using tags?</a> in the AWS Knowledge Center. Tags are visible on the tagged dataset, but not on the ingestion resource.</p>
    #[allow(unused_mut)]
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
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let mut result = proto::json::ResponsePayload::new(&response)
                .deserialize::<CreateIngestionResponse, _>()?;

            result.status = Some(response.status.as_u16() as i64);
            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(CreateIngestionError::from_response(response))
        }
    }

    /// <p>(Enterprise edition only) Creates a new namespace for you to use with Amazon QuickSight.</p> <p>A namespace allows you to isolate the QuickSight users and groups that are registered for that namespace. Users that access the namespace can share assets only with other users or groups in the same namespace. They can't see users and groups in other namespaces. You can create a namespace after your AWS account is subscribed to QuickSight. The namespace must be unique within the AWS account. By default, there is a limit of 100 namespaces per AWS account. To increase your limit, create a ticket with AWS Support. </p>
    #[allow(unused_mut)]
    async fn create_namespace(
        &self,
        input: CreateNamespaceRequest,
    ) -> Result<CreateNamespaceResponse, RusotoError<CreateNamespaceError>> {
        let request_uri = format!(
            "/accounts/{aws_account_id}",
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
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let mut result = proto::json::ResponsePayload::new(&response)
                .deserialize::<CreateNamespaceResponse, _>()?;

            result.status = Some(response.status.as_u16() as i64);
            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(CreateNamespaceError::from_response(response))
        }
    }

    /// <p>Creates a template from an existing QuickSight analysis or template. You can use the resulting template to create a dashboard.</p> <p>A <i>template</i> is an entity in QuickSight that encapsulates the metadata required to create an analysis and that you can use to create s dashboard. A template adds a layer of abstraction by using placeholders to replace the dataset associated with the analysis. You can use templates to create dashboards by replacing dataset placeholders with datasets that follow the same schema that was used to create the source analysis and template.</p>
    #[allow(unused_mut)]
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
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
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
    #[allow(unused_mut)]
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
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let mut result = proto::json::ResponsePayload::new(&response)
                .deserialize::<CreateTemplateAliasResponse, _>()?;

            result.status = Some(response.status.as_u16() as i64);
            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(CreateTemplateAliasError::from_response(response))
        }
    }

    /// <p>Creates a theme.</p> <p>A <i>theme</i> is set of configuration options for color and layout. Themes apply to analyses and dashboards. For more information, see <a href="https://docs.aws.amazon.com/quicksight/latest/user/themes-in-quicksight.html">Using Themes in Amazon QuickSight</a> in the <i>Amazon QuickSight User Guide</i>.</p>
    #[allow(unused_mut)]
    async fn create_theme(
        &self,
        input: CreateThemeRequest,
    ) -> Result<CreateThemeResponse, RusotoError<CreateThemeError>> {
        let request_uri = format!(
            "/accounts/{aws_account_id}/themes/{theme_id}",
            aws_account_id = input.aws_account_id,
            theme_id = input.theme_id
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
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let mut result = proto::json::ResponsePayload::new(&response)
                .deserialize::<CreateThemeResponse, _>()?;

            result.status = Some(response.status.as_u16() as i64);
            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(CreateThemeError::from_response(response))
        }
    }

    /// <p>Creates a theme alias for a theme.</p>
    #[allow(unused_mut)]
    async fn create_theme_alias(
        &self,
        input: CreateThemeAliasRequest,
    ) -> Result<CreateThemeAliasResponse, RusotoError<CreateThemeAliasError>> {
        let request_uri = format!(
            "/accounts/{aws_account_id}/themes/{theme_id}/aliases/{alias_name}",
            alias_name = input.alias_name,
            aws_account_id = input.aws_account_id,
            theme_id = input.theme_id
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
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let mut result = proto::json::ResponsePayload::new(&response)
                .deserialize::<CreateThemeAliasResponse, _>()?;

            result.status = Some(response.status.as_u16() as i64);
            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(CreateThemeAliasError::from_response(response))
        }
    }

    /// <p>Deletes all Amazon QuickSight customizations in this AWS Region for the specified AWS account and QuickSight namespace.</p>
    #[allow(unused_mut)]
    async fn delete_account_customization(
        &self,
        input: DeleteAccountCustomizationRequest,
    ) -> Result<DeleteAccountCustomizationResponse, RusotoError<DeleteAccountCustomizationError>>
    {
        let request_uri = format!(
            "/accounts/{aws_account_id}/customizations",
            aws_account_id = input.aws_account_id
        );

        let mut request = SignedRequest::new("DELETE", "quicksight", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut params = Params::new();
        if let Some(ref x) = input.namespace {
            params.put("namespace", x);
        }
        request.set_params(params);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let mut result = proto::json::ResponsePayload::new(&response)
                .deserialize::<DeleteAccountCustomizationResponse, _>()?;

            result.status = Some(response.status.as_u16() as i64);
            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(DeleteAccountCustomizationError::from_response(response))
        }
    }

    /// <p>Deletes an analysis from Amazon QuickSight. You can optionally include a recovery window during which you can restore the analysis. If you don't specify a recovery window value, the operation defaults to 30 days. QuickSight attaches a <code>DeletionTime</code> stamp to the response that specifies the end of the recovery window. At the end of the recovery window, QuickSight deletes the analysis permanently.</p> <p>At any time before recovery window ends, you can use the <code>RestoreAnalysis</code> API operation to remove the <code>DeletionTime</code> stamp and cancel the deletion of the analysis. The analysis remains visible in the API until it's deleted, so you can describe it but you can't make a template from it.</p> <p>An analysis that's scheduled for deletion isn't accessible in the QuickSight console. To access it in the console, restore it. Deleting an analysis doesn't delete the dashboards that you publish from it.</p>
    #[allow(unused_mut)]
    async fn delete_analysis(
        &self,
        input: DeleteAnalysisRequest,
    ) -> Result<DeleteAnalysisResponse, RusotoError<DeleteAnalysisError>> {
        let request_uri = format!(
            "/accounts/{aws_account_id}/analyses/{analysis_id}",
            analysis_id = input.analysis_id,
            aws_account_id = input.aws_account_id
        );

        let mut request = SignedRequest::new("DELETE", "quicksight", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut params = Params::new();
        if let Some(ref x) = input.force_delete_without_recovery {
            params.put("force-delete-without-recovery", x);
        }
        if let Some(ref x) = input.recovery_window_in_days {
            params.put("recovery-window-in-days", x);
        }
        request.set_params(params);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let mut result = proto::json::ResponsePayload::new(&response)
                .deserialize::<DeleteAnalysisResponse, _>()?;

            result.status = Some(response.status.as_u16() as i64);
            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(DeleteAnalysisError::from_response(response))
        }
    }

    /// <p>Deletes a dashboard.</p>
    #[allow(unused_mut)]
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
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
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
    #[allow(unused_mut)]
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
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let mut result = proto::json::ResponsePayload::new(&response)
                .deserialize::<DeleteDataSetResponse, _>()?;

            result.status = Some(response.status.as_u16() as i64);
            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(DeleteDataSetError::from_response(response))
        }
    }

    /// <p>Deletes the data source permanently. This operation breaks all the datasets that reference the deleted data source.</p>
    #[allow(unused_mut)]
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
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
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
    #[allow(unused_mut)]
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
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
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
    #[allow(unused_mut)]
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
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
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
    #[allow(unused_mut)]
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
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let mut result = proto::json::ResponsePayload::new(&response)
                .deserialize::<DeleteIAMPolicyAssignmentResponse, _>()?;

            result.status = Some(response.status.as_u16() as i64);
            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(DeleteIAMPolicyAssignmentError::from_response(response))
        }
    }

    /// <p>Deletes a namespace and the users and groups that are associated with the namespace. This is an asynchronous process. Assets including dashboards, analyses, datasets and data sources are not deleted. To delete these assets, you use the API operations for the relevant asset. </p>
    #[allow(unused_mut)]
    async fn delete_namespace(
        &self,
        input: DeleteNamespaceRequest,
    ) -> Result<DeleteNamespaceResponse, RusotoError<DeleteNamespaceError>> {
        let request_uri = format!(
            "/accounts/{aws_account_id}/namespaces/{namespace}",
            aws_account_id = input.aws_account_id,
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
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let mut result = proto::json::ResponsePayload::new(&response)
                .deserialize::<DeleteNamespaceResponse, _>()?;

            result.status = Some(response.status.as_u16() as i64);
            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(DeleteNamespaceError::from_response(response))
        }
    }

    /// <p>Deletes a template.</p>
    #[allow(unused_mut)]
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
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
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
    #[allow(unused_mut)]
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
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let mut result = proto::json::ResponsePayload::new(&response)
                .deserialize::<DeleteTemplateAliasResponse, _>()?;

            result.status = Some(response.status.as_u16() as i64);
            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(DeleteTemplateAliasError::from_response(response))
        }
    }

    /// <p>Deletes a theme.</p>
    #[allow(unused_mut)]
    async fn delete_theme(
        &self,
        input: DeleteThemeRequest,
    ) -> Result<DeleteThemeResponse, RusotoError<DeleteThemeError>> {
        let request_uri = format!(
            "/accounts/{aws_account_id}/themes/{theme_id}",
            aws_account_id = input.aws_account_id,
            theme_id = input.theme_id
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
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let mut result = proto::json::ResponsePayload::new(&response)
                .deserialize::<DeleteThemeResponse, _>()?;

            result.status = Some(response.status.as_u16() as i64);
            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(DeleteThemeError::from_response(response))
        }
    }

    /// <p>Deletes the version of the theme that the specified theme alias points to. If you provide a specific alias, you delete the version of the theme that the alias points to.</p>
    #[allow(unused_mut)]
    async fn delete_theme_alias(
        &self,
        input: DeleteThemeAliasRequest,
    ) -> Result<DeleteThemeAliasResponse, RusotoError<DeleteThemeAliasError>> {
        let request_uri = format!(
            "/accounts/{aws_account_id}/themes/{theme_id}/aliases/{alias_name}",
            alias_name = input.alias_name,
            aws_account_id = input.aws_account_id,
            theme_id = input.theme_id
        );

        let mut request = SignedRequest::new("DELETE", "quicksight", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let mut result = proto::json::ResponsePayload::new(&response)
                .deserialize::<DeleteThemeAliasResponse, _>()?;

            result.status = Some(response.status.as_u16() as i64);
            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(DeleteThemeAliasError::from_response(response))
        }
    }

    /// <p>Deletes the Amazon QuickSight user that is associated with the identity of the AWS Identity and Access Management (IAM) user or role that's making the call. The IAM user isn't deleted as a result of this call. </p>
    #[allow(unused_mut)]
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
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
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
    #[allow(unused_mut)]
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
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let mut result = proto::json::ResponsePayload::new(&response)
                .deserialize::<DeleteUserByPrincipalIdResponse, _>()?;

            result.status = Some(response.status.as_u16() as i64);
            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(DeleteUserByPrincipalIdError::from_response(response))
        }
    }

    /// <p><p>Describes the customizations associated with the provided AWS account and Amazon QuickSight namespace in an AWS Region. The QuickSight console evaluates which customizations to apply by running this API operation with the <code>Resolved</code> flag included. </p> <p>To determine what customizations display when you run this command, it can help to visualize the relationship of the entities involved. </p> <ul> <li> <p> <code>AWS Account</code> - The AWS account exists at the top of the hierarchy. It has the potential to use all of the AWS Regions and AWS Services. When you subscribe to QuickSight, you choose one AWS Region to use as your home Region. That&#39;s where your free SPICE capacity is located. You can use QuickSight in any supported AWS Region. </p> </li> <li> <p> <code>AWS Region</code> - In each AWS Region where you sign in to QuickSight at least once, QuickSight acts as a separate instance of the same service. If you have a user directory, it resides in us-east-1, which is the US East (N. Virginia). Generally speaking, these users have access to QuickSight in any AWS Region, unless they are constrained to a namespace. </p> <p>To run the command in a different AWS Region, you change your Region settings. If you&#39;re using the AWS CLI, you can use one of the following options:</p> <ul> <li> <p>Use <a href="https://docs.aws.amazon.com/cli/latest/userguide/cli-configure-options.html">command line options</a>. </p> </li> <li> <p>Use <a href="https://docs.aws.amazon.com/cli/latest/userguide/cli-configure-profiles.html">named profiles</a>. </p> </li> <li> <p>Run <code>aws configure</code> to change your default AWS Region. Use Enter to key the same settings for your keys. For more information, see <a href="https://docs.aws.amazon.com/cli/latest/userguide/cli-chap-configure.html">Configuring the AWS CLI</a>.</p> </li> </ul> </li> <li> <p> <code>Namespace</code> - A QuickSight namespace is a partition that contains users and assets (data sources, datasets, dashboards, and so on). To access assets that are in a specific namespace, users and groups must also be part of the same namespace. People who share a namespace are completely isolated from users and assets in other namespaces, even if they are in the same AWS account and AWS Region.</p> </li> <li> <p> <code>Applied customizations</code> - Within an AWS Region, a set of QuickSight customizations can apply to an AWS account or to a namespace. Settings that you apply to a namespace override settings that you apply to an AWS account. All settings are isolated to a single AWS Region. To apply them in other AWS Regions, run the <code>CreateAccountCustomization</code> command in each AWS Region where you want to apply the same customizations. </p> </li> </ul></p>
    #[allow(unused_mut)]
    async fn describe_account_customization(
        &self,
        input: DescribeAccountCustomizationRequest,
    ) -> Result<DescribeAccountCustomizationResponse, RusotoError<DescribeAccountCustomizationError>>
    {
        let request_uri = format!(
            "/accounts/{aws_account_id}/customizations",
            aws_account_id = input.aws_account_id
        );

        let mut request = SignedRequest::new("GET", "quicksight", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut params = Params::new();
        if let Some(ref x) = input.namespace {
            params.put("namespace", x);
        }
        if let Some(ref x) = input.resolved {
            params.put("resolved", x);
        }
        request.set_params(params);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let mut result = proto::json::ResponsePayload::new(&response)
                .deserialize::<DescribeAccountCustomizationResponse, _>()?;

            result.status = Some(response.status.as_u16() as i64);
            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(DescribeAccountCustomizationError::from_response(response))
        }
    }

    /// <p>Describes the settings that were used when your QuickSight subscription was first created in this AWS account.</p>
    #[allow(unused_mut)]
    async fn describe_account_settings(
        &self,
        input: DescribeAccountSettingsRequest,
    ) -> Result<DescribeAccountSettingsResponse, RusotoError<DescribeAccountSettingsError>> {
        let request_uri = format!(
            "/accounts/{aws_account_id}/settings",
            aws_account_id = input.aws_account_id
        );

        let mut request = SignedRequest::new("GET", "quicksight", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let mut result = proto::json::ResponsePayload::new(&response)
                .deserialize::<DescribeAccountSettingsResponse, _>()?;

            result.status = Some(response.status.as_u16() as i64);
            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(DescribeAccountSettingsError::from_response(response))
        }
    }

    /// <p>Provides a summary of the metadata for an analysis.</p>
    #[allow(unused_mut)]
    async fn describe_analysis(
        &self,
        input: DescribeAnalysisRequest,
    ) -> Result<DescribeAnalysisResponse, RusotoError<DescribeAnalysisError>> {
        let request_uri = format!(
            "/accounts/{aws_account_id}/analyses/{analysis_id}",
            analysis_id = input.analysis_id,
            aws_account_id = input.aws_account_id
        );

        let mut request = SignedRequest::new("GET", "quicksight", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let mut result = proto::json::ResponsePayload::new(&response)
                .deserialize::<DescribeAnalysisResponse, _>()?;

            result.status = Some(response.status.as_u16() as i64);
            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(DescribeAnalysisError::from_response(response))
        }
    }

    /// <p>Provides the read and write permissions for an analysis.</p>
    #[allow(unused_mut)]
    async fn describe_analysis_permissions(
        &self,
        input: DescribeAnalysisPermissionsRequest,
    ) -> Result<DescribeAnalysisPermissionsResponse, RusotoError<DescribeAnalysisPermissionsError>>
    {
        let request_uri = format!(
            "/accounts/{aws_account_id}/analyses/{analysis_id}/permissions",
            analysis_id = input.analysis_id,
            aws_account_id = input.aws_account_id
        );

        let mut request = SignedRequest::new("GET", "quicksight", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let mut result = proto::json::ResponsePayload::new(&response)
                .deserialize::<DescribeAnalysisPermissionsResponse, _>()?;

            result.status = Some(response.status.as_u16() as i64);
            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(DescribeAnalysisPermissionsError::from_response(response))
        }
    }

    /// <p>Provides a summary for a dashboard.</p>
    #[allow(unused_mut)]
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
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
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
    #[allow(unused_mut)]
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
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
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
    #[allow(unused_mut)]
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
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
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
    #[allow(unused_mut)]
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
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
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
    #[allow(unused_mut)]
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
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
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
    #[allow(unused_mut)]
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
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
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
    #[allow(unused_mut)]
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
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
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
    #[allow(unused_mut)]
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
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
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
    #[allow(unused_mut)]
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
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let mut result = proto::json::ResponsePayload::new(&response)
                .deserialize::<DescribeIngestionResponse, _>()?;

            result.status = Some(response.status.as_u16() as i64);
            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(DescribeIngestionError::from_response(response))
        }
    }

    /// <p>Describes the current namespace.</p>
    #[allow(unused_mut)]
    async fn describe_namespace(
        &self,
        input: DescribeNamespaceRequest,
    ) -> Result<DescribeNamespaceResponse, RusotoError<DescribeNamespaceError>> {
        let request_uri = format!(
            "/accounts/{aws_account_id}/namespaces/{namespace}",
            aws_account_id = input.aws_account_id,
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
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let mut result = proto::json::ResponsePayload::new(&response)
                .deserialize::<DescribeNamespaceResponse, _>()?;

            result.status = Some(response.status.as_u16() as i64);
            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(DescribeNamespaceError::from_response(response))
        }
    }

    /// <p>Describes a template's metadata.</p>
    #[allow(unused_mut)]
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
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
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
    #[allow(unused_mut)]
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
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
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
    #[allow(unused_mut)]
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
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let mut result = proto::json::ResponsePayload::new(&response)
                .deserialize::<DescribeTemplatePermissionsResponse, _>()?;

            result.status = Some(response.status.as_u16() as i64);
            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(DescribeTemplatePermissionsError::from_response(response))
        }
    }

    /// <p>Describes a theme.</p>
    #[allow(unused_mut)]
    async fn describe_theme(
        &self,
        input: DescribeThemeRequest,
    ) -> Result<DescribeThemeResponse, RusotoError<DescribeThemeError>> {
        let request_uri = format!(
            "/accounts/{aws_account_id}/themes/{theme_id}",
            aws_account_id = input.aws_account_id,
            theme_id = input.theme_id
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
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let mut result = proto::json::ResponsePayload::new(&response)
                .deserialize::<DescribeThemeResponse, _>()?;

            result.status = Some(response.status.as_u16() as i64);
            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(DescribeThemeError::from_response(response))
        }
    }

    /// <p>Describes the alias for a theme.</p>
    #[allow(unused_mut)]
    async fn describe_theme_alias(
        &self,
        input: DescribeThemeAliasRequest,
    ) -> Result<DescribeThemeAliasResponse, RusotoError<DescribeThemeAliasError>> {
        let request_uri = format!(
            "/accounts/{aws_account_id}/themes/{theme_id}/aliases/{alias_name}",
            alias_name = input.alias_name,
            aws_account_id = input.aws_account_id,
            theme_id = input.theme_id
        );

        let mut request = SignedRequest::new("GET", "quicksight", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let mut result = proto::json::ResponsePayload::new(&response)
                .deserialize::<DescribeThemeAliasResponse, _>()?;

            result.status = Some(response.status.as_u16() as i64);
            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(DescribeThemeAliasError::from_response(response))
        }
    }

    /// <p>Describes the read and write permissions for a theme.</p>
    #[allow(unused_mut)]
    async fn describe_theme_permissions(
        &self,
        input: DescribeThemePermissionsRequest,
    ) -> Result<DescribeThemePermissionsResponse, RusotoError<DescribeThemePermissionsError>> {
        let request_uri = format!(
            "/accounts/{aws_account_id}/themes/{theme_id}/permissions",
            aws_account_id = input.aws_account_id,
            theme_id = input.theme_id
        );

        let mut request = SignedRequest::new("GET", "quicksight", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let mut result = proto::json::ResponsePayload::new(&response)
                .deserialize::<DescribeThemePermissionsResponse, _>()?;

            result.status = Some(response.status.as_u16() as i64);
            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(DescribeThemePermissionsError::from_response(response))
        }
    }

    /// <p>Returns information about a user, given the user name. </p>
    #[allow(unused_mut)]
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
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let mut result = proto::json::ResponsePayload::new(&response)
                .deserialize::<DescribeUserResponse, _>()?;

            result.status = Some(response.status.as_u16() as i64);
            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(DescribeUserError::from_response(response))
        }
    }

    /// <p>Generates a session URL and authorization code that you can use to embed an Amazon QuickSight read-only dashboard in your web server code. Before you use this command, make sure that you have configured the dashboards and permissions. </p> <p>Currently, you can use <code>GetDashboardEmbedURL</code> only from the server, not from the user's browser. The following rules apply to the combination of URL and authorization code:</p> <ul> <li> <p>They must be used together.</p> </li> <li> <p>They can be used one time only.</p> </li> <li> <p>They are valid for 5 minutes after you run this command.</p> </li> <li> <p>The resulting user session is valid for 10 hours.</p> </li> </ul> <p>For more information, see <a href="https://docs.aws.amazon.com/quicksight/latest/user/embedded-analytics.html">Embedded Analytics</a> in the <i>Amazon QuickSight User Guide</i>.</p>
    #[allow(unused_mut)]
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
        if let Some(ref x) = input.additional_dashboard_ids {
            for item in x.iter() {
                params.put("additional-dashboard-ids", item);
            }
        }
        params.put("creds-type", &input.identity_type);
        if let Some(ref x) = input.namespace {
            params.put("namespace", x);
        }
        if let Some(ref x) = input.reset_disabled {
            params.put("reset-disabled", x);
        }
        if let Some(ref x) = input.session_lifetime_in_minutes {
            params.put("session-lifetime", x);
        }
        if let Some(ref x) = input.state_persistence_enabled {
            params.put("state-persistence-enabled", x);
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
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let mut result = proto::json::ResponsePayload::new(&response)
                .deserialize::<GetDashboardEmbedUrlResponse, _>()?;

            result.status = Some(response.status.as_u16() as i64);
            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(GetDashboardEmbedUrlError::from_response(response))
        }
    }

    /// <p><p>Generates a session URL and authorization code that you can use to embed the Amazon QuickSight console in your web server code. Use <code>GetSessionEmbedUrl</code> where you want to provide an authoring portal that allows users to create data sources, datasets, analyses, and dashboards. The users who access an embedded QuickSight console need belong to the author or admin security cohort. If you want to restrict permissions to some of these features, add a custom permissions profile to the user with the <code> <a>UpdateUser</a> </code> API operation. Use <code> <a>RegisterUser</a> </code> API operation to add a new user with a custom permission profile attached. For more information, see the following sections in the <i>Amazon QuickSight User Guide</i>:</p> <ul> <li> <p> <a href="https://docs.aws.amazon.com/quicksight/latest/user/embedding-the-quicksight-console.html">Embedding the Amazon QuickSight Console</a> </p> </li> <li> <p> <a href="https://docs.aws.amazon.com/quicksight/latest/user/customizing-permissions-to-the-quicksight-console.html">Customizing Access to the Amazon QuickSight Console</a> </p> </li> </ul></p>
    #[allow(unused_mut)]
    async fn get_session_embed_url(
        &self,
        input: GetSessionEmbedUrlRequest,
    ) -> Result<GetSessionEmbedUrlResponse, RusotoError<GetSessionEmbedUrlError>> {
        let request_uri = format!(
            "/accounts/{aws_account_id}/session-embed-url",
            aws_account_id = input.aws_account_id
        );

        let mut request = SignedRequest::new("GET", "quicksight", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut params = Params::new();
        if let Some(ref x) = input.entry_point {
            params.put("entry-point", x);
        }
        if let Some(ref x) = input.session_lifetime_in_minutes {
            params.put("session-lifetime", x);
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
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let mut result = proto::json::ResponsePayload::new(&response)
                .deserialize::<GetSessionEmbedUrlResponse, _>()?;

            result.status = Some(response.status.as_u16() as i64);
            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(GetSessionEmbedUrlError::from_response(response))
        }
    }

    /// <p>Lists Amazon QuickSight analyses that exist in the specified AWS account.</p>
    #[allow(unused_mut)]
    async fn list_analyses(
        &self,
        input: ListAnalysesRequest,
    ) -> Result<ListAnalysesResponse, RusotoError<ListAnalysesError>> {
        let request_uri = format!(
            "/accounts/{aws_account_id}/analyses",
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
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let mut result = proto::json::ResponsePayload::new(&response)
                .deserialize::<ListAnalysesResponse, _>()?;

            result.status = Some(response.status.as_u16() as i64);
            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(ListAnalysesError::from_response(response))
        }
    }

    /// <p>Lists all the versions of the dashboards in the QuickSight subscription.</p>
    #[allow(unused_mut)]
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
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
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
    #[allow(unused_mut)]
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
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
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
    #[allow(unused_mut)]
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
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
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
    #[allow(unused_mut)]
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
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
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
    #[allow(unused_mut)]
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
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
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
    #[allow(unused_mut)]
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
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
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
    #[allow(unused_mut)]
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
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
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
    #[allow(unused_mut)]
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
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
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
    #[allow(unused_mut)]
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
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let mut result = proto::json::ResponsePayload::new(&response)
                .deserialize::<ListIngestionsResponse, _>()?;

            result.status = Some(response.status.as_u16() as i64);
            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(ListIngestionsError::from_response(response))
        }
    }

    /// <p>Lists the namespaces for the specified AWS account.</p>
    #[allow(unused_mut)]
    async fn list_namespaces(
        &self,
        input: ListNamespacesRequest,
    ) -> Result<ListNamespacesResponse, RusotoError<ListNamespacesError>> {
        let request_uri = format!(
            "/accounts/{aws_account_id}/namespaces",
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
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let mut result = proto::json::ResponsePayload::new(&response)
                .deserialize::<ListNamespacesResponse, _>()?;

            result.status = Some(response.status.as_u16() as i64);
            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(ListNamespacesError::from_response(response))
        }
    }

    /// <p>Lists the tags assigned to a resource.</p>
    #[allow(unused_mut)]
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
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
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
    #[allow(unused_mut)]
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
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
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
    #[allow(unused_mut)]
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
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
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
    #[allow(unused_mut)]
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
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let mut result = proto::json::ResponsePayload::new(&response)
                .deserialize::<ListTemplatesResponse, _>()?;

            result.status = Some(response.status.as_u16() as i64);
            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(ListTemplatesError::from_response(response))
        }
    }

    /// <p>Lists all the aliases of a theme.</p>
    #[allow(unused_mut)]
    async fn list_theme_aliases(
        &self,
        input: ListThemeAliasesRequest,
    ) -> Result<ListThemeAliasesResponse, RusotoError<ListThemeAliasesError>> {
        let request_uri = format!(
            "/accounts/{aws_account_id}/themes/{theme_id}/aliases",
            aws_account_id = input.aws_account_id,
            theme_id = input.theme_id
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
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let mut result = proto::json::ResponsePayload::new(&response)
                .deserialize::<ListThemeAliasesResponse, _>()?;

            result.status = Some(response.status.as_u16() as i64);
            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(ListThemeAliasesError::from_response(response))
        }
    }

    /// <p>Lists all the versions of the themes in the current AWS account.</p>
    #[allow(unused_mut)]
    async fn list_theme_versions(
        &self,
        input: ListThemeVersionsRequest,
    ) -> Result<ListThemeVersionsResponse, RusotoError<ListThemeVersionsError>> {
        let request_uri = format!(
            "/accounts/{aws_account_id}/themes/{theme_id}/versions",
            aws_account_id = input.aws_account_id,
            theme_id = input.theme_id
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
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let mut result = proto::json::ResponsePayload::new(&response)
                .deserialize::<ListThemeVersionsResponse, _>()?;

            result.status = Some(response.status.as_u16() as i64);
            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(ListThemeVersionsError::from_response(response))
        }
    }

    /// <p>Lists all the themes in the current AWS account.</p>
    #[allow(unused_mut)]
    async fn list_themes(
        &self,
        input: ListThemesRequest,
    ) -> Result<ListThemesResponse, RusotoError<ListThemesError>> {
        let request_uri = format!(
            "/accounts/{aws_account_id}/themes",
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
        if let Some(ref x) = input.type_ {
            params.put("type", x);
        }
        request.set_params(params);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let mut result = proto::json::ResponsePayload::new(&response)
                .deserialize::<ListThemesResponse, _>()?;

            result.status = Some(response.status.as_u16() as i64);
            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(ListThemesError::from_response(response))
        }
    }

    /// <p>Lists the Amazon QuickSight groups that an Amazon QuickSight user is a member of.</p>
    #[allow(unused_mut)]
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
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
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
    #[allow(unused_mut)]
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
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
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
    #[allow(unused_mut)]
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
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let mut result = proto::json::ResponsePayload::new(&response)
                .deserialize::<RegisterUserResponse, _>()?;

            result.status = Some(response.status.as_u16() as i64);
            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(RegisterUserError::from_response(response))
        }
    }

    /// <p>Restores an analysis.</p>
    #[allow(unused_mut)]
    async fn restore_analysis(
        &self,
        input: RestoreAnalysisRequest,
    ) -> Result<RestoreAnalysisResponse, RusotoError<RestoreAnalysisError>> {
        let request_uri = format!(
            "/accounts/{aws_account_id}/restore/analyses/{analysis_id}",
            analysis_id = input.analysis_id,
            aws_account_id = input.aws_account_id
        );

        let mut request = SignedRequest::new("POST", "quicksight", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let mut result = proto::json::ResponsePayload::new(&response)
                .deserialize::<RestoreAnalysisResponse, _>()?;

            result.status = Some(response.status.as_u16() as i64);
            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(RestoreAnalysisError::from_response(response))
        }
    }

    /// <p>Searches for analyses that belong to the user specified in the filter.</p>
    #[allow(unused_mut)]
    async fn search_analyses(
        &self,
        input: SearchAnalysesRequest,
    ) -> Result<SearchAnalysesResponse, RusotoError<SearchAnalysesError>> {
        let request_uri = format!(
            "/accounts/{aws_account_id}/search/analyses",
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
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let mut result = proto::json::ResponsePayload::new(&response)
                .deserialize::<SearchAnalysesResponse, _>()?;

            result.status = Some(response.status.as_u16() as i64);
            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(SearchAnalysesError::from_response(response))
        }
    }

    /// <p>Searches for dashboards that belong to a user. </p>
    #[allow(unused_mut)]
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
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
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
    #[allow(unused_mut)]
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
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
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
    #[allow(unused_mut)]
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
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let mut result = proto::json::ResponsePayload::new(&response)
                .deserialize::<UntagResourceResponse, _>()?;

            result.status = Some(response.status.as_u16() as i64);
            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(UntagResourceError::from_response(response))
        }
    }

    /// <p>Updates Amazon QuickSight customizations the current AWS Region. Currently, the only customization you can use is a theme.</p> <p>You can use customizations for your AWS account or, if you specify a namespace, for a QuickSight namespace instead. Customizations that apply to a namespace override customizations that apply to an AWS account. To find out which customizations apply, use the <code>DescribeAccountCustomization</code> API operation. </p>
    #[allow(unused_mut)]
    async fn update_account_customization(
        &self,
        input: UpdateAccountCustomizationRequest,
    ) -> Result<UpdateAccountCustomizationResponse, RusotoError<UpdateAccountCustomizationError>>
    {
        let request_uri = format!(
            "/accounts/{aws_account_id}/customizations",
            aws_account_id = input.aws_account_id
        );

        let mut request = SignedRequest::new("PUT", "quicksight", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut params = Params::new();
        if let Some(ref x) = input.namespace {
            params.put("namespace", x);
        }
        request.set_params(params);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let mut result = proto::json::ResponsePayload::new(&response)
                .deserialize::<UpdateAccountCustomizationResponse, _>()?;

            result.status = Some(response.status.as_u16() as i64);
            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(UpdateAccountCustomizationError::from_response(response))
        }
    }

    /// <p>Updates the Amazon QuickSight settings in your AWS account.</p>
    #[allow(unused_mut)]
    async fn update_account_settings(
        &self,
        input: UpdateAccountSettingsRequest,
    ) -> Result<UpdateAccountSettingsResponse, RusotoError<UpdateAccountSettingsError>> {
        let request_uri = format!(
            "/accounts/{aws_account_id}/settings",
            aws_account_id = input.aws_account_id
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
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let mut result = proto::json::ResponsePayload::new(&response)
                .deserialize::<UpdateAccountSettingsResponse, _>()?;

            result.status = Some(response.status.as_u16() as i64);
            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(UpdateAccountSettingsError::from_response(response))
        }
    }

    /// <p>Updates an analysis in Amazon QuickSight</p>
    #[allow(unused_mut)]
    async fn update_analysis(
        &self,
        input: UpdateAnalysisRequest,
    ) -> Result<UpdateAnalysisResponse, RusotoError<UpdateAnalysisError>> {
        let request_uri = format!(
            "/accounts/{aws_account_id}/analyses/{analysis_id}",
            analysis_id = input.analysis_id,
            aws_account_id = input.aws_account_id
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
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let mut result = proto::json::ResponsePayload::new(&response)
                .deserialize::<UpdateAnalysisResponse, _>()?;

            result.status = Some(response.status.as_u16() as i64);
            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(UpdateAnalysisError::from_response(response))
        }
    }

    /// <p>Updates the read and write permissions for an analysis.</p>
    #[allow(unused_mut)]
    async fn update_analysis_permissions(
        &self,
        input: UpdateAnalysisPermissionsRequest,
    ) -> Result<UpdateAnalysisPermissionsResponse, RusotoError<UpdateAnalysisPermissionsError>>
    {
        let request_uri = format!(
            "/accounts/{aws_account_id}/analyses/{analysis_id}/permissions",
            analysis_id = input.analysis_id,
            aws_account_id = input.aws_account_id
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
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let mut result = proto::json::ResponsePayload::new(&response)
                .deserialize::<UpdateAnalysisPermissionsResponse, _>()?;

            result.status = Some(response.status.as_u16() as i64);
            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(UpdateAnalysisPermissionsError::from_response(response))
        }
    }

    /// <p>Updates a dashboard in an AWS account.</p>
    #[allow(unused_mut)]
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
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<UpdateDashboardResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(UpdateDashboardError::from_response(response))
        }
    }

    /// <p>Updates read and write permissions on a dashboard.</p>
    #[allow(unused_mut)]
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
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
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
    #[allow(unused_mut)]
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
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
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
    #[allow(unused_mut)]
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
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
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
    #[allow(unused_mut)]
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
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
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
    #[allow(unused_mut)]
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
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
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
    #[allow(unused_mut)]
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
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
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
    #[allow(unused_mut)]
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
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let mut result = proto::json::ResponsePayload::new(&response)
                .deserialize::<UpdateGroupResponse, _>()?;

            result.status = Some(response.status.as_u16() as i64);
            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(UpdateGroupError::from_response(response))
        }
    }

    /// <p>Updates an existing IAM policy assignment. This operation updates only the optional parameter or parameters that are specified in the request. This overwrites all of the users included in <code>Identities</code>. </p>
    #[allow(unused_mut)]
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
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
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
    #[allow(unused_mut)]
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
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
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
    #[allow(unused_mut)]
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
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
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
    #[allow(unused_mut)]
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
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let mut result = proto::json::ResponsePayload::new(&response)
                .deserialize::<UpdateTemplatePermissionsResponse, _>()?;

            result.status = Some(response.status.as_u16() as i64);
            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(UpdateTemplatePermissionsError::from_response(response))
        }
    }

    /// <p>Updates a theme.</p>
    #[allow(unused_mut)]
    async fn update_theme(
        &self,
        input: UpdateThemeRequest,
    ) -> Result<UpdateThemeResponse, RusotoError<UpdateThemeError>> {
        let request_uri = format!(
            "/accounts/{aws_account_id}/themes/{theme_id}",
            aws_account_id = input.aws_account_id,
            theme_id = input.theme_id
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
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let mut result = proto::json::ResponsePayload::new(&response)
                .deserialize::<UpdateThemeResponse, _>()?;

            result.status = Some(response.status.as_u16() as i64);
            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(UpdateThemeError::from_response(response))
        }
    }

    /// <p>Updates an alias of a theme.</p>
    #[allow(unused_mut)]
    async fn update_theme_alias(
        &self,
        input: UpdateThemeAliasRequest,
    ) -> Result<UpdateThemeAliasResponse, RusotoError<UpdateThemeAliasError>> {
        let request_uri = format!(
            "/accounts/{aws_account_id}/themes/{theme_id}/aliases/{alias_name}",
            alias_name = input.alias_name,
            aws_account_id = input.aws_account_id,
            theme_id = input.theme_id
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
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let mut result = proto::json::ResponsePayload::new(&response)
                .deserialize::<UpdateThemeAliasResponse, _>()?;

            result.status = Some(response.status.as_u16() as i64);
            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(UpdateThemeAliasError::from_response(response))
        }
    }

    /// <p><p>Updates the resource permissions for a theme. Permissions apply to the action to grant or revoke permissions on, for example <code>&quot;quicksight:DescribeTheme&quot;</code>.</p> <p>Theme permissions apply in groupings. Valid groupings include the following for the three levels of permissions, which are user, owner, or no permissions: </p> <ul> <li> <p>User</p> <ul> <li> <p> <code>&quot;quicksight:DescribeTheme&quot;</code> </p> </li> <li> <p> <code>&quot;quicksight:DescribeThemeAlias&quot;</code> </p> </li> <li> <p> <code>&quot;quicksight:ListThemeAliases&quot;</code> </p> </li> <li> <p> <code>&quot;quicksight:ListThemeVersions&quot;</code> </p> </li> </ul> </li> <li> <p>Owner</p> <ul> <li> <p> <code>&quot;quicksight:DescribeTheme&quot;</code> </p> </li> <li> <p> <code>&quot;quicksight:DescribeThemeAlias&quot;</code> </p> </li> <li> <p> <code>&quot;quicksight:ListThemeAliases&quot;</code> </p> </li> <li> <p> <code>&quot;quicksight:ListThemeVersions&quot;</code> </p> </li> <li> <p> <code>&quot;quicksight:DeleteTheme&quot;</code> </p> </li> <li> <p> <code>&quot;quicksight:UpdateTheme&quot;</code> </p> </li> <li> <p> <code>&quot;quicksight:CreateThemeAlias&quot;</code> </p> </li> <li> <p> <code>&quot;quicksight:DeleteThemeAlias&quot;</code> </p> </li> <li> <p> <code>&quot;quicksight:UpdateThemeAlias&quot;</code> </p> </li> <li> <p> <code>&quot;quicksight:UpdateThemePermissions&quot;</code> </p> </li> <li> <p> <code>&quot;quicksight:DescribeThemePermissions&quot;</code> </p> </li> </ul> </li> <li> <p>To specify no permissions, omit the permissions list.</p> </li> </ul></p>
    #[allow(unused_mut)]
    async fn update_theme_permissions(
        &self,
        input: UpdateThemePermissionsRequest,
    ) -> Result<UpdateThemePermissionsResponse, RusotoError<UpdateThemePermissionsError>> {
        let request_uri = format!(
            "/accounts/{aws_account_id}/themes/{theme_id}/permissions",
            aws_account_id = input.aws_account_id,
            theme_id = input.theme_id
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
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let mut result = proto::json::ResponsePayload::new(&response)
                .deserialize::<UpdateThemePermissionsResponse, _>()?;

            result.status = Some(response.status.as_u16() as i64);
            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(UpdateThemePermissionsError::from_response(response))
        }
    }

    /// <p>Updates an Amazon QuickSight user.</p>
    #[allow(unused_mut)]
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
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
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
