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
use rusoto_core::param::{Params, ServiceParams};
use rusoto_core::proto;
use rusoto_core::signature::SignedRequest;
#[allow(unused_imports)]
use serde::{Deserialize, Serialize};
use serde_json;
use std::pin::Pin;
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateDiscovererRequest {
    /// <p>A description for the discoverer.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The ARN of the event bus.</p>
    #[serde(rename = "SourceArn")]
    pub source_arn: String,
    /// <p>Tags associated with the resource.</p>
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<::std::collections::HashMap<String, String>>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateDiscovererResponse {
    /// <p>The description of the discoverer.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The ARN of the discoverer.</p>
    #[serde(rename = "DiscovererArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub discoverer_arn: Option<String>,
    /// <p>The ID of the discoverer.</p>
    #[serde(rename = "DiscovererId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub discoverer_id: Option<String>,
    /// <p>The ARN of the event bus.</p>
    #[serde(rename = "SourceArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_arn: Option<String>,
    /// <p>The state of the discoverer.</p>
    #[serde(rename = "State")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    /// <p>Tags associated with the resource.</p>
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<::std::collections::HashMap<String, String>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateRegistryRequest {
    /// <p>A description of the registry to be created.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "RegistryName")]
    pub registry_name: String,
    /// <p>Tags to associate with the registry.</p>
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<::std::collections::HashMap<String, String>>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateRegistryResponse {
    /// <p>The description of the registry.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The ARN of the registry.</p>
    #[serde(rename = "RegistryArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub registry_arn: Option<String>,
    /// <p>The name of the registry.</p>
    #[serde(rename = "RegistryName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub registry_name: Option<String>,
    /// <p>Tags associated with the registry.</p>
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<::std::collections::HashMap<String, String>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateSchemaRequest {
    #[serde(rename = "Content")]
    pub content: String,
    /// <p>A description of the schema.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "RegistryName")]
    pub registry_name: String,
    #[serde(rename = "SchemaName")]
    pub schema_name: String,
    /// <p>Tags associated with the schema.</p>
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<::std::collections::HashMap<String, String>>,
    #[serde(rename = "Type")]
    pub type_: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateSchemaResponse {
    /// <p>The description of the schema.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The date and time that schema was modified.</p>
    #[serde(rename = "LastModified")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified: Option<f64>,
    /// <p>The ARN of the schema.</p>
    #[serde(rename = "SchemaArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schema_arn: Option<String>,
    /// <p>The name of the schema.</p>
    #[serde(rename = "SchemaName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schema_name: Option<String>,
    /// <p>The version number of the schema</p>
    #[serde(rename = "SchemaVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schema_version: Option<String>,
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<::std::collections::HashMap<String, String>>,
    /// <p>The type of the schema.</p>
    #[serde(rename = "Type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    /// <p>The date the schema version was created.</p>
    #[serde(rename = "VersionCreatedDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version_created_date: Option<f64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteDiscovererRequest {
    #[serde(rename = "DiscovererId")]
    pub discoverer_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteRegistryRequest {
    #[serde(rename = "RegistryName")]
    pub registry_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteSchemaRequest {
    #[serde(rename = "RegistryName")]
    pub registry_name: String,
    #[serde(rename = "SchemaName")]
    pub schema_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteSchemaVersionRequest {
    #[serde(rename = "RegistryName")]
    pub registry_name: String,
    #[serde(rename = "SchemaName")]
    pub schema_name: String,
    #[serde(rename = "SchemaVersion")]
    pub schema_version: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeCodeBindingRequest {
    #[serde(rename = "Language")]
    pub language: String,
    #[serde(rename = "RegistryName")]
    pub registry_name: String,
    #[serde(rename = "SchemaName")]
    pub schema_name: String,
    #[serde(rename = "SchemaVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schema_version: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeCodeBindingResponse {
    /// <p>The time and date that the code binding was created.</p>
    #[serde(rename = "CreationDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_date: Option<f64>,
    /// <p>The date and time that code bindings were modified.</p>
    #[serde(rename = "LastModified")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified: Option<f64>,
    /// <p>The version number of the schema.</p>
    #[serde(rename = "SchemaVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schema_version: Option<String>,
    /// <p>The current status of code binding generation.</p>
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeDiscovererRequest {
    #[serde(rename = "DiscovererId")]
    pub discoverer_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeDiscovererResponse {
    /// <p>The description of the discoverer.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The ARN of the discoverer.</p>
    #[serde(rename = "DiscovererArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub discoverer_arn: Option<String>,
    /// <p>The ID of the discoverer.</p>
    #[serde(rename = "DiscovererId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub discoverer_id: Option<String>,
    /// <p>The ARN of the event bus.</p>
    #[serde(rename = "SourceArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_arn: Option<String>,
    /// <p>The state of the discoverer.</p>
    #[serde(rename = "State")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    /// <p>Tags associated with the resource.</p>
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<::std::collections::HashMap<String, String>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeRegistryRequest {
    #[serde(rename = "RegistryName")]
    pub registry_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeRegistryResponse {
    /// <p>The description of the registry.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The ARN of the registry.</p>
    #[serde(rename = "RegistryArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub registry_arn: Option<String>,
    /// <p>The name of the registry.</p>
    #[serde(rename = "RegistryName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub registry_name: Option<String>,
    /// <p>Tags associated with the registry.</p>
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<::std::collections::HashMap<String, String>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeSchemaRequest {
    #[serde(rename = "RegistryName")]
    pub registry_name: String,
    #[serde(rename = "SchemaName")]
    pub schema_name: String,
    #[serde(rename = "SchemaVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schema_version: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeSchemaResponse {
    #[serde(rename = "Content")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
    /// <p>The description of the schema.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The date and time that schema was modified.</p>
    #[serde(rename = "LastModified")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified: Option<f64>,
    /// <p>The ARN of the schema.</p>
    #[serde(rename = "SchemaArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schema_arn: Option<String>,
    /// <p>The name of the schema.</p>
    #[serde(rename = "SchemaName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schema_name: Option<String>,
    /// <p>The version number of the schema</p>
    #[serde(rename = "SchemaVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schema_version: Option<String>,
    /// <p>Tags associated with the resource.</p>
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<::std::collections::HashMap<String, String>>,
    /// <p>The type of the schema.</p>
    #[serde(rename = "Type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    /// <p>The date the schema version was created.</p>
    #[serde(rename = "VersionCreatedDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version_created_date: Option<f64>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DiscovererSummary {
    /// <p>The ARN of the discoverer.</p>
    #[serde(rename = "DiscovererArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub discoverer_arn: Option<String>,
    /// <p>The ID of the discoverer.</p>
    #[serde(rename = "DiscovererId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub discoverer_id: Option<String>,
    /// <p>The ARN of the event bus.</p>
    #[serde(rename = "SourceArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_arn: Option<String>,
    #[serde(rename = "State")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    /// <p>Tags associated with the resource.</p>
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<::std::collections::HashMap<String, String>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetCodeBindingSourceRequest {
    #[serde(rename = "Language")]
    pub language: String,
    #[serde(rename = "RegistryName")]
    pub registry_name: String,
    #[serde(rename = "SchemaName")]
    pub schema_name: String,
    #[serde(rename = "SchemaVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schema_version: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct GetCodeBindingSourceResponse {
    pub body: Option<bytes::Bytes>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetDiscoveredSchemaRequest {
    /// <p>An array of strings that</p>
    #[serde(rename = "Events")]
    pub events: Vec<String>,
    /// <p>The type of event.</p>
    #[serde(rename = "Type")]
    pub type_: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetDiscoveredSchemaResponse {
    #[serde(rename = "Content")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListDiscoverersRequest {
    #[serde(rename = "DiscovererIdPrefix")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub discoverer_id_prefix: Option<String>,
    #[serde(rename = "Limit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "SourceArnPrefix")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_arn_prefix: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListDiscoverersResponse {
    /// <p>An array of DiscovererSummary information.</p>
    #[serde(rename = "Discoverers")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub discoverers: Option<Vec<DiscovererSummary>>,
    /// <p>The token that specifies the next page of results to return. To request the first page, leave NextToken empty. The token will expire in 24 hours, and cannot be shared with other accounts.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListRegistriesRequest {
    #[serde(rename = "Limit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "RegistryNamePrefix")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub registry_name_prefix: Option<String>,
    #[serde(rename = "Scope")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scope: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListRegistriesResponse {
    /// <p>The token that specifies the next page of results to return. To request the first page, leave NextToken empty. The token will expire in 24 hours, and cannot be shared with other accounts.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>An array of registry summaries.</p>
    #[serde(rename = "Registries")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub registries: Option<Vec<RegistrySummary>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListSchemaVersionsRequest {
    #[serde(rename = "Limit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "RegistryName")]
    pub registry_name: String,
    #[serde(rename = "SchemaName")]
    pub schema_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListSchemaVersionsResponse {
    /// <p>The token that specifies the next page of results to return. To request the first page, leave NextToken empty. The token will expire in 24 hours, and cannot be shared with other accounts.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>An array of schema version summaries.</p>
    #[serde(rename = "SchemaVersions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schema_versions: Option<Vec<SchemaVersionSummary>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListSchemasRequest {
    #[serde(rename = "Limit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "RegistryName")]
    pub registry_name: String,
    #[serde(rename = "SchemaNamePrefix")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schema_name_prefix: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListSchemasResponse {
    /// <p>The token that specifies the next page of results to return. To request the first page, leave NextToken empty. The token will expire in 24 hours, and cannot be shared with other accounts.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>An array of schema summaries.</p>
    #[serde(rename = "Schemas")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schemas: Option<Vec<SchemaSummary>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListTagsForResourceRequest {
    #[serde(rename = "ResourceArn")]
    pub resource_arn: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListTagsForResourceResponse {
    #[serde(rename = "Tags")]
    pub tags: ::std::collections::HashMap<String, String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct LockServiceLinkedRoleRequest {
    #[serde(rename = "RoleArn")]
    pub role_arn: String,
    #[serde(rename = "Timeout")]
    pub timeout: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct LockServiceLinkedRoleResponse {
    #[serde(rename = "CanBeDeleted")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_be_deleted: Option<bool>,
    #[serde(rename = "ReasonOfFailure")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason_of_failure: Option<String>,
    #[serde(rename = "RelatedResources")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub related_resources: Option<Vec<DiscovererSummary>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct PutCodeBindingRequest {
    #[serde(rename = "Language")]
    pub language: String,
    #[serde(rename = "RegistryName")]
    pub registry_name: String,
    #[serde(rename = "SchemaName")]
    pub schema_name: String,
    #[serde(rename = "SchemaVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schema_version: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct PutCodeBindingResponse {
    /// <p>The time and date that the code binding was created.</p>
    #[serde(rename = "CreationDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_date: Option<f64>,
    /// <p>The date and time that code bindings were modified.</p>
    #[serde(rename = "LastModified")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified: Option<f64>,
    /// <p>The version number of the schema.</p>
    #[serde(rename = "SchemaVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schema_version: Option<String>,
    /// <p>The current status of code binding generation.</p>
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct RegistrySummary {
    /// <p>The ARN of the registry.</p>
    #[serde(rename = "RegistryArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub registry_arn: Option<String>,
    /// <p>The name of the registry.</p>
    #[serde(rename = "RegistryName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub registry_name: Option<String>,
    /// <p>Tags associated with the registry.</p>
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<::std::collections::HashMap<String, String>>,
}

/// <p>A summary of schema details.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct SchemaSummary {
    /// <p>The date and time that schema was modified.</p>
    #[serde(rename = "LastModified")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified: Option<f64>,
    /// <p>The ARN of the schema.</p>
    #[serde(rename = "SchemaArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schema_arn: Option<String>,
    /// <p>The name of the schema.</p>
    #[serde(rename = "SchemaName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schema_name: Option<String>,
    /// <p>Tags associated with the schema.</p>
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<::std::collections::HashMap<String, String>>,
    /// <p>The number of versions available for the schema.</p>
    #[serde(rename = "VersionCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version_count: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct SchemaVersionSummary {
    /// <p>The ARN of the schema version.</p>
    #[serde(rename = "SchemaArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schema_arn: Option<String>,
    /// <p>The name of the schema.</p>
    #[serde(rename = "SchemaName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schema_name: Option<String>,
    /// <p>The version number of the schema.</p>
    #[serde(rename = "SchemaVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schema_version: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct SearchSchemaSummary {
    /// <p>The name of the registry.</p>
    #[serde(rename = "RegistryName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub registry_name: Option<String>,
    /// <p>The ARN of the schema.</p>
    #[serde(rename = "SchemaArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schema_arn: Option<String>,
    /// <p>The name of the schema.</p>
    #[serde(rename = "SchemaName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schema_name: Option<String>,
    /// <p>An array of schema version summaries.</p>
    #[serde(rename = "SchemaVersions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schema_versions: Option<Vec<SearchSchemaVersionSummary>>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct SearchSchemaVersionSummary {
    #[serde(rename = "CreatedDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_date: Option<f64>,
    /// <p>The version number of the schema</p>
    #[serde(rename = "SchemaVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schema_version: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct SearchSchemasRequest {
    #[serde(rename = "Keywords")]
    pub keywords: String,
    #[serde(rename = "Limit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "RegistryName")]
    pub registry_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct SearchSchemasResponse {
    /// <p>The token that specifies the next page of results to return. To request the first page, leave NextToken empty. The token will expire in 24 hours, and cannot be shared with other accounts.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>An array of SearchSchemaSummary information.</p>
    #[serde(rename = "Schemas")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schemas: Option<Vec<SearchSchemaSummary>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct StartDiscovererRequest {
    #[serde(rename = "DiscovererId")]
    pub discoverer_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct StartDiscovererResponse {
    /// <p>The ID of the discoverer.</p>
    #[serde(rename = "DiscovererId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub discoverer_id: Option<String>,
    /// <p>The state of the discoverer.</p>
    #[serde(rename = "State")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct StopDiscovererRequest {
    #[serde(rename = "DiscovererId")]
    pub discoverer_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct StopDiscovererResponse {
    /// <p>The ID of the discoverer.</p>
    #[serde(rename = "DiscovererId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub discoverer_id: Option<String>,
    /// <p>The state of the discoverer.</p>
    #[serde(rename = "State")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct TagResourceRequest {
    #[serde(rename = "ResourceArn")]
    pub resource_arn: String,
    #[serde(rename = "Tags")]
    pub tags: ::std::collections::HashMap<String, String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UnlockServiceLinkedRoleRequest {
    #[serde(rename = "RoleArn")]
    pub role_arn: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UnlockServiceLinkedRoleResponse {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UntagResourceRequest {
    #[serde(rename = "ResourceArn")]
    pub resource_arn: String,
    #[serde(rename = "TagKeys")]
    pub tag_keys: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UpdateDiscovererRequest {
    /// <p>The description of the discoverer to update.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "DiscovererId")]
    pub discoverer_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UpdateDiscovererResponse {
    /// <p>The description of the discoverer.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The ARN of the discoverer.</p>
    #[serde(rename = "DiscovererArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub discoverer_arn: Option<String>,
    /// <p>The ID of the discoverer.</p>
    #[serde(rename = "DiscovererId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub discoverer_id: Option<String>,
    /// <p>The ARN of the event bus.</p>
    #[serde(rename = "SourceArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_arn: Option<String>,
    /// <p>The state of the discoverer.</p>
    #[serde(rename = "State")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    /// <p>Tags associated with the resource.</p>
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<::std::collections::HashMap<String, String>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UpdateRegistryRequest {
    /// <p>The description of the registry to update.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "RegistryName")]
    pub registry_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UpdateRegistryResponse {
    /// <p>The description of the registry.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The ARN of the registry.</p>
    #[serde(rename = "RegistryArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub registry_arn: Option<String>,
    /// <p>The name of the registry.</p>
    #[serde(rename = "RegistryName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub registry_name: Option<String>,
    /// <p>Tags associated with the registry.</p>
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<::std::collections::HashMap<String, String>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UpdateSchemaRequest {
    /// <p>The ID of the client token.</p>
    #[serde(rename = "ClientTokenId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token_id: Option<String>,
    /// <p>The source of the schema definition.</p>
    #[serde(rename = "Content")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
    /// <p>The description of the schema.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "RegistryName")]
    pub registry_name: String,
    #[serde(rename = "SchemaName")]
    pub schema_name: String,
    /// <p>The schema type for the events schema.</p>
    #[serde(rename = "Type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UpdateSchemaResponse {
    /// <p>The description of the schema.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The date and time that schema was modified.</p>
    #[serde(rename = "LastModified")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified: Option<f64>,
    /// <p>The ARN of the schema.</p>
    #[serde(rename = "SchemaArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schema_arn: Option<String>,
    /// <p>The name of the schema.</p>
    #[serde(rename = "SchemaName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schema_name: Option<String>,
    /// <p>The version number of the schema</p>
    #[serde(rename = "SchemaVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schema_version: Option<String>,
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<::std::collections::HashMap<String, String>>,
    /// <p>The type of the schema.</p>
    #[serde(rename = "Type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    /// <p>The date the schema version was created.</p>
    #[serde(rename = "VersionCreatedDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version_created_date: Option<f64>,
}

/// Errors returned by CreateDiscoverer
#[derive(Debug, PartialEq)]
pub enum CreateDiscovererError {
    BadRequest(String),

    Conflict(String),

    Forbidden(String),

    InternalServerError(String),

    ServiceUnavailable(String),

    Unauthorized(String),
}

impl CreateDiscovererError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateDiscovererError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(CreateDiscovererError::BadRequest(err.msg))
                }
                "ConflictException" => {
                    return RusotoError::Service(CreateDiscovererError::Conflict(err.msg))
                }
                "ForbiddenException" => {
                    return RusotoError::Service(CreateDiscovererError::Forbidden(err.msg))
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(CreateDiscovererError::InternalServerError(
                        err.msg,
                    ))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(CreateDiscovererError::ServiceUnavailable(err.msg))
                }
                "UnauthorizedException" => {
                    return RusotoError::Service(CreateDiscovererError::Unauthorized(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for CreateDiscovererError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreateDiscovererError::BadRequest(ref cause) => write!(f, "{}", cause),
            CreateDiscovererError::Conflict(ref cause) => write!(f, "{}", cause),
            CreateDiscovererError::Forbidden(ref cause) => write!(f, "{}", cause),
            CreateDiscovererError::InternalServerError(ref cause) => write!(f, "{}", cause),
            CreateDiscovererError::ServiceUnavailable(ref cause) => write!(f, "{}", cause),
            CreateDiscovererError::Unauthorized(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CreateDiscovererError {}
/// Errors returned by CreateRegistry
#[derive(Debug, PartialEq)]
pub enum CreateRegistryError {
    BadRequest(String),

    Conflict(String),

    Forbidden(String),

    InternalServerError(String),

    ServiceUnavailable(String),

    Unauthorized(String),
}

impl CreateRegistryError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateRegistryError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(CreateRegistryError::BadRequest(err.msg))
                }
                "ConflictException" => {
                    return RusotoError::Service(CreateRegistryError::Conflict(err.msg))
                }
                "ForbiddenException" => {
                    return RusotoError::Service(CreateRegistryError::Forbidden(err.msg))
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(CreateRegistryError::InternalServerError(err.msg))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(CreateRegistryError::ServiceUnavailable(err.msg))
                }
                "UnauthorizedException" => {
                    return RusotoError::Service(CreateRegistryError::Unauthorized(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for CreateRegistryError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreateRegistryError::BadRequest(ref cause) => write!(f, "{}", cause),
            CreateRegistryError::Conflict(ref cause) => write!(f, "{}", cause),
            CreateRegistryError::Forbidden(ref cause) => write!(f, "{}", cause),
            CreateRegistryError::InternalServerError(ref cause) => write!(f, "{}", cause),
            CreateRegistryError::ServiceUnavailable(ref cause) => write!(f, "{}", cause),
            CreateRegistryError::Unauthorized(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CreateRegistryError {}
/// Errors returned by CreateSchema
#[derive(Debug, PartialEq)]
pub enum CreateSchemaError {
    BadRequest(String),

    Forbidden(String),

    InternalServerError(String),

    ServiceUnavailable(String),
}

impl CreateSchemaError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateSchemaError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(CreateSchemaError::BadRequest(err.msg))
                }
                "ForbiddenException" => {
                    return RusotoError::Service(CreateSchemaError::Forbidden(err.msg))
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(CreateSchemaError::InternalServerError(err.msg))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(CreateSchemaError::ServiceUnavailable(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for CreateSchemaError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreateSchemaError::BadRequest(ref cause) => write!(f, "{}", cause),
            CreateSchemaError::Forbidden(ref cause) => write!(f, "{}", cause),
            CreateSchemaError::InternalServerError(ref cause) => write!(f, "{}", cause),
            CreateSchemaError::ServiceUnavailable(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CreateSchemaError {}
/// Errors returned by DeleteDiscoverer
#[derive(Debug, PartialEq)]
pub enum DeleteDiscovererError {
    BadRequest(String),

    Forbidden(String),

    InternalServerError(String),

    NotFound(String),

    ServiceUnavailable(String),

    Unauthorized(String),
}

impl DeleteDiscovererError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteDiscovererError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(DeleteDiscovererError::BadRequest(err.msg))
                }
                "ForbiddenException" => {
                    return RusotoError::Service(DeleteDiscovererError::Forbidden(err.msg))
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(DeleteDiscovererError::InternalServerError(
                        err.msg,
                    ))
                }
                "NotFoundException" => {
                    return RusotoError::Service(DeleteDiscovererError::NotFound(err.msg))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(DeleteDiscovererError::ServiceUnavailable(err.msg))
                }
                "UnauthorizedException" => {
                    return RusotoError::Service(DeleteDiscovererError::Unauthorized(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DeleteDiscovererError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteDiscovererError::BadRequest(ref cause) => write!(f, "{}", cause),
            DeleteDiscovererError::Forbidden(ref cause) => write!(f, "{}", cause),
            DeleteDiscovererError::InternalServerError(ref cause) => write!(f, "{}", cause),
            DeleteDiscovererError::NotFound(ref cause) => write!(f, "{}", cause),
            DeleteDiscovererError::ServiceUnavailable(ref cause) => write!(f, "{}", cause),
            DeleteDiscovererError::Unauthorized(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeleteDiscovererError {}
/// Errors returned by DeleteRegistry
#[derive(Debug, PartialEq)]
pub enum DeleteRegistryError {
    BadRequest(String),

    Forbidden(String),

    InternalServerError(String),

    NotFound(String),

    ServiceUnavailable(String),

    Unauthorized(String),
}

impl DeleteRegistryError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteRegistryError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(DeleteRegistryError::BadRequest(err.msg))
                }
                "ForbiddenException" => {
                    return RusotoError::Service(DeleteRegistryError::Forbidden(err.msg))
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(DeleteRegistryError::InternalServerError(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(DeleteRegistryError::NotFound(err.msg))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(DeleteRegistryError::ServiceUnavailable(err.msg))
                }
                "UnauthorizedException" => {
                    return RusotoError::Service(DeleteRegistryError::Unauthorized(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DeleteRegistryError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteRegistryError::BadRequest(ref cause) => write!(f, "{}", cause),
            DeleteRegistryError::Forbidden(ref cause) => write!(f, "{}", cause),
            DeleteRegistryError::InternalServerError(ref cause) => write!(f, "{}", cause),
            DeleteRegistryError::NotFound(ref cause) => write!(f, "{}", cause),
            DeleteRegistryError::ServiceUnavailable(ref cause) => write!(f, "{}", cause),
            DeleteRegistryError::Unauthorized(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeleteRegistryError {}
/// Errors returned by DeleteSchema
#[derive(Debug, PartialEq)]
pub enum DeleteSchemaError {
    BadRequest(String),

    Forbidden(String),

    InternalServerError(String),

    NotFound(String),

    ServiceUnavailable(String),

    Unauthorized(String),
}

impl DeleteSchemaError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteSchemaError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(DeleteSchemaError::BadRequest(err.msg))
                }
                "ForbiddenException" => {
                    return RusotoError::Service(DeleteSchemaError::Forbidden(err.msg))
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(DeleteSchemaError::InternalServerError(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(DeleteSchemaError::NotFound(err.msg))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(DeleteSchemaError::ServiceUnavailable(err.msg))
                }
                "UnauthorizedException" => {
                    return RusotoError::Service(DeleteSchemaError::Unauthorized(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DeleteSchemaError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteSchemaError::BadRequest(ref cause) => write!(f, "{}", cause),
            DeleteSchemaError::Forbidden(ref cause) => write!(f, "{}", cause),
            DeleteSchemaError::InternalServerError(ref cause) => write!(f, "{}", cause),
            DeleteSchemaError::NotFound(ref cause) => write!(f, "{}", cause),
            DeleteSchemaError::ServiceUnavailable(ref cause) => write!(f, "{}", cause),
            DeleteSchemaError::Unauthorized(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeleteSchemaError {}
/// Errors returned by DeleteSchemaVersion
#[derive(Debug, PartialEq)]
pub enum DeleteSchemaVersionError {
    BadRequest(String),

    Forbidden(String),

    InternalServerError(String),

    NotFound(String),

    ServiceUnavailable(String),

    Unauthorized(String),
}

impl DeleteSchemaVersionError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteSchemaVersionError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(DeleteSchemaVersionError::BadRequest(err.msg))
                }
                "ForbiddenException" => {
                    return RusotoError::Service(DeleteSchemaVersionError::Forbidden(err.msg))
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(DeleteSchemaVersionError::InternalServerError(
                        err.msg,
                    ))
                }
                "NotFoundException" => {
                    return RusotoError::Service(DeleteSchemaVersionError::NotFound(err.msg))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(DeleteSchemaVersionError::ServiceUnavailable(
                        err.msg,
                    ))
                }
                "UnauthorizedException" => {
                    return RusotoError::Service(DeleteSchemaVersionError::Unauthorized(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DeleteSchemaVersionError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteSchemaVersionError::BadRequest(ref cause) => write!(f, "{}", cause),
            DeleteSchemaVersionError::Forbidden(ref cause) => write!(f, "{}", cause),
            DeleteSchemaVersionError::InternalServerError(ref cause) => write!(f, "{}", cause),
            DeleteSchemaVersionError::NotFound(ref cause) => write!(f, "{}", cause),
            DeleteSchemaVersionError::ServiceUnavailable(ref cause) => write!(f, "{}", cause),
            DeleteSchemaVersionError::Unauthorized(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeleteSchemaVersionError {}
/// Errors returned by DescribeCodeBinding
#[derive(Debug, PartialEq)]
pub enum DescribeCodeBindingError {
    BadRequest(String),

    Forbidden(String),

    InternalServerError(String),

    NotFound(String),

    TooManyRequests(String),

    Unauthorized(String),
}

impl DescribeCodeBindingError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeCodeBindingError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(DescribeCodeBindingError::BadRequest(err.msg))
                }
                "ForbiddenException" => {
                    return RusotoError::Service(DescribeCodeBindingError::Forbidden(err.msg))
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(DescribeCodeBindingError::InternalServerError(
                        err.msg,
                    ))
                }
                "NotFoundException" => {
                    return RusotoError::Service(DescribeCodeBindingError::NotFound(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(DescribeCodeBindingError::TooManyRequests(err.msg))
                }
                "UnauthorizedException" => {
                    return RusotoError::Service(DescribeCodeBindingError::Unauthorized(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DescribeCodeBindingError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeCodeBindingError::BadRequest(ref cause) => write!(f, "{}", cause),
            DescribeCodeBindingError::Forbidden(ref cause) => write!(f, "{}", cause),
            DescribeCodeBindingError::InternalServerError(ref cause) => write!(f, "{}", cause),
            DescribeCodeBindingError::NotFound(ref cause) => write!(f, "{}", cause),
            DescribeCodeBindingError::TooManyRequests(ref cause) => write!(f, "{}", cause),
            DescribeCodeBindingError::Unauthorized(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DescribeCodeBindingError {}
/// Errors returned by DescribeDiscoverer
#[derive(Debug, PartialEq)]
pub enum DescribeDiscovererError {
    BadRequest(String),

    Forbidden(String),

    InternalServerError(String),

    NotFound(String),

    ServiceUnavailable(String),

    Unauthorized(String),
}

impl DescribeDiscovererError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeDiscovererError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(DescribeDiscovererError::BadRequest(err.msg))
                }
                "ForbiddenException" => {
                    return RusotoError::Service(DescribeDiscovererError::Forbidden(err.msg))
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(DescribeDiscovererError::InternalServerError(
                        err.msg,
                    ))
                }
                "NotFoundException" => {
                    return RusotoError::Service(DescribeDiscovererError::NotFound(err.msg))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(DescribeDiscovererError::ServiceUnavailable(
                        err.msg,
                    ))
                }
                "UnauthorizedException" => {
                    return RusotoError::Service(DescribeDiscovererError::Unauthorized(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DescribeDiscovererError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeDiscovererError::BadRequest(ref cause) => write!(f, "{}", cause),
            DescribeDiscovererError::Forbidden(ref cause) => write!(f, "{}", cause),
            DescribeDiscovererError::InternalServerError(ref cause) => write!(f, "{}", cause),
            DescribeDiscovererError::NotFound(ref cause) => write!(f, "{}", cause),
            DescribeDiscovererError::ServiceUnavailable(ref cause) => write!(f, "{}", cause),
            DescribeDiscovererError::Unauthorized(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DescribeDiscovererError {}
/// Errors returned by DescribeRegistry
#[derive(Debug, PartialEq)]
pub enum DescribeRegistryError {
    BadRequest(String),

    Forbidden(String),

    InternalServerError(String),

    NotFound(String),

    ServiceUnavailable(String),

    Unauthorized(String),
}

impl DescribeRegistryError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeRegistryError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(DescribeRegistryError::BadRequest(err.msg))
                }
                "ForbiddenException" => {
                    return RusotoError::Service(DescribeRegistryError::Forbidden(err.msg))
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(DescribeRegistryError::InternalServerError(
                        err.msg,
                    ))
                }
                "NotFoundException" => {
                    return RusotoError::Service(DescribeRegistryError::NotFound(err.msg))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(DescribeRegistryError::ServiceUnavailable(err.msg))
                }
                "UnauthorizedException" => {
                    return RusotoError::Service(DescribeRegistryError::Unauthorized(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DescribeRegistryError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeRegistryError::BadRequest(ref cause) => write!(f, "{}", cause),
            DescribeRegistryError::Forbidden(ref cause) => write!(f, "{}", cause),
            DescribeRegistryError::InternalServerError(ref cause) => write!(f, "{}", cause),
            DescribeRegistryError::NotFound(ref cause) => write!(f, "{}", cause),
            DescribeRegistryError::ServiceUnavailable(ref cause) => write!(f, "{}", cause),
            DescribeRegistryError::Unauthorized(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DescribeRegistryError {}
/// Errors returned by DescribeSchema
#[derive(Debug, PartialEq)]
pub enum DescribeSchemaError {
    BadRequest(String),

    Forbidden(String),

    InternalServerError(String),

    NotFound(String),

    ServiceUnavailable(String),

    Unauthorized(String),
}

impl DescribeSchemaError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeSchemaError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(DescribeSchemaError::BadRequest(err.msg))
                }
                "ForbiddenException" => {
                    return RusotoError::Service(DescribeSchemaError::Forbidden(err.msg))
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(DescribeSchemaError::InternalServerError(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(DescribeSchemaError::NotFound(err.msg))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(DescribeSchemaError::ServiceUnavailable(err.msg))
                }
                "UnauthorizedException" => {
                    return RusotoError::Service(DescribeSchemaError::Unauthorized(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DescribeSchemaError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeSchemaError::BadRequest(ref cause) => write!(f, "{}", cause),
            DescribeSchemaError::Forbidden(ref cause) => write!(f, "{}", cause),
            DescribeSchemaError::InternalServerError(ref cause) => write!(f, "{}", cause),
            DescribeSchemaError::NotFound(ref cause) => write!(f, "{}", cause),
            DescribeSchemaError::ServiceUnavailable(ref cause) => write!(f, "{}", cause),
            DescribeSchemaError::Unauthorized(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DescribeSchemaError {}
/// Errors returned by GetCodeBindingSource
#[derive(Debug, PartialEq)]
pub enum GetCodeBindingSourceError {
    BadRequest(String),

    Forbidden(String),

    InternalServerError(String),

    NotFound(String),

    TooManyRequests(String),

    Unauthorized(String),
}

impl GetCodeBindingSourceError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetCodeBindingSourceError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(GetCodeBindingSourceError::BadRequest(err.msg))
                }
                "ForbiddenException" => {
                    return RusotoError::Service(GetCodeBindingSourceError::Forbidden(err.msg))
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(GetCodeBindingSourceError::InternalServerError(
                        err.msg,
                    ))
                }
                "NotFoundException" => {
                    return RusotoError::Service(GetCodeBindingSourceError::NotFound(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(GetCodeBindingSourceError::TooManyRequests(
                        err.msg,
                    ))
                }
                "UnauthorizedException" => {
                    return RusotoError::Service(GetCodeBindingSourceError::Unauthorized(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for GetCodeBindingSourceError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetCodeBindingSourceError::BadRequest(ref cause) => write!(f, "{}", cause),
            GetCodeBindingSourceError::Forbidden(ref cause) => write!(f, "{}", cause),
            GetCodeBindingSourceError::InternalServerError(ref cause) => write!(f, "{}", cause),
            GetCodeBindingSourceError::NotFound(ref cause) => write!(f, "{}", cause),
            GetCodeBindingSourceError::TooManyRequests(ref cause) => write!(f, "{}", cause),
            GetCodeBindingSourceError::Unauthorized(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetCodeBindingSourceError {}
/// Errors returned by GetDiscoveredSchema
#[derive(Debug, PartialEq)]
pub enum GetDiscoveredSchemaError {
    BadRequest(String),

    Forbidden(String),

    InternalServerError(String),

    ServiceUnavailable(String),

    Unauthorized(String),
}

impl GetDiscoveredSchemaError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetDiscoveredSchemaError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(GetDiscoveredSchemaError::BadRequest(err.msg))
                }
                "ForbiddenException" => {
                    return RusotoError::Service(GetDiscoveredSchemaError::Forbidden(err.msg))
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(GetDiscoveredSchemaError::InternalServerError(
                        err.msg,
                    ))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(GetDiscoveredSchemaError::ServiceUnavailable(
                        err.msg,
                    ))
                }
                "UnauthorizedException" => {
                    return RusotoError::Service(GetDiscoveredSchemaError::Unauthorized(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for GetDiscoveredSchemaError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetDiscoveredSchemaError::BadRequest(ref cause) => write!(f, "{}", cause),
            GetDiscoveredSchemaError::Forbidden(ref cause) => write!(f, "{}", cause),
            GetDiscoveredSchemaError::InternalServerError(ref cause) => write!(f, "{}", cause),
            GetDiscoveredSchemaError::ServiceUnavailable(ref cause) => write!(f, "{}", cause),
            GetDiscoveredSchemaError::Unauthorized(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetDiscoveredSchemaError {}
/// Errors returned by ListDiscoverers
#[derive(Debug, PartialEq)]
pub enum ListDiscoverersError {
    BadRequest(String),

    Forbidden(String),

    InternalServerError(String),

    ServiceUnavailable(String),

    Unauthorized(String),
}

impl ListDiscoverersError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListDiscoverersError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(ListDiscoverersError::BadRequest(err.msg))
                }
                "ForbiddenException" => {
                    return RusotoError::Service(ListDiscoverersError::Forbidden(err.msg))
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(ListDiscoverersError::InternalServerError(err.msg))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(ListDiscoverersError::ServiceUnavailable(err.msg))
                }
                "UnauthorizedException" => {
                    return RusotoError::Service(ListDiscoverersError::Unauthorized(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ListDiscoverersError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListDiscoverersError::BadRequest(ref cause) => write!(f, "{}", cause),
            ListDiscoverersError::Forbidden(ref cause) => write!(f, "{}", cause),
            ListDiscoverersError::InternalServerError(ref cause) => write!(f, "{}", cause),
            ListDiscoverersError::ServiceUnavailable(ref cause) => write!(f, "{}", cause),
            ListDiscoverersError::Unauthorized(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListDiscoverersError {}
/// Errors returned by ListRegistries
#[derive(Debug, PartialEq)]
pub enum ListRegistriesError {
    BadRequest(String),

    Forbidden(String),

    InternalServerError(String),

    ServiceUnavailable(String),

    Unauthorized(String),
}

impl ListRegistriesError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListRegistriesError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(ListRegistriesError::BadRequest(err.msg))
                }
                "ForbiddenException" => {
                    return RusotoError::Service(ListRegistriesError::Forbidden(err.msg))
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(ListRegistriesError::InternalServerError(err.msg))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(ListRegistriesError::ServiceUnavailable(err.msg))
                }
                "UnauthorizedException" => {
                    return RusotoError::Service(ListRegistriesError::Unauthorized(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ListRegistriesError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListRegistriesError::BadRequest(ref cause) => write!(f, "{}", cause),
            ListRegistriesError::Forbidden(ref cause) => write!(f, "{}", cause),
            ListRegistriesError::InternalServerError(ref cause) => write!(f, "{}", cause),
            ListRegistriesError::ServiceUnavailable(ref cause) => write!(f, "{}", cause),
            ListRegistriesError::Unauthorized(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListRegistriesError {}
/// Errors returned by ListSchemaVersions
#[derive(Debug, PartialEq)]
pub enum ListSchemaVersionsError {
    BadRequest(String),

    Forbidden(String),

    InternalServerError(String),

    NotFound(String),

    ServiceUnavailable(String),

    Unauthorized(String),
}

impl ListSchemaVersionsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListSchemaVersionsError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(ListSchemaVersionsError::BadRequest(err.msg))
                }
                "ForbiddenException" => {
                    return RusotoError::Service(ListSchemaVersionsError::Forbidden(err.msg))
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(ListSchemaVersionsError::InternalServerError(
                        err.msg,
                    ))
                }
                "NotFoundException" => {
                    return RusotoError::Service(ListSchemaVersionsError::NotFound(err.msg))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(ListSchemaVersionsError::ServiceUnavailable(
                        err.msg,
                    ))
                }
                "UnauthorizedException" => {
                    return RusotoError::Service(ListSchemaVersionsError::Unauthorized(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ListSchemaVersionsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListSchemaVersionsError::BadRequest(ref cause) => write!(f, "{}", cause),
            ListSchemaVersionsError::Forbidden(ref cause) => write!(f, "{}", cause),
            ListSchemaVersionsError::InternalServerError(ref cause) => write!(f, "{}", cause),
            ListSchemaVersionsError::NotFound(ref cause) => write!(f, "{}", cause),
            ListSchemaVersionsError::ServiceUnavailable(ref cause) => write!(f, "{}", cause),
            ListSchemaVersionsError::Unauthorized(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListSchemaVersionsError {}
/// Errors returned by ListSchemas
#[derive(Debug, PartialEq)]
pub enum ListSchemasError {
    BadRequest(String),

    Forbidden(String),

    InternalServerError(String),

    ServiceUnavailable(String),

    Unauthorized(String),
}

impl ListSchemasError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListSchemasError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(ListSchemasError::BadRequest(err.msg))
                }
                "ForbiddenException" => {
                    return RusotoError::Service(ListSchemasError::Forbidden(err.msg))
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(ListSchemasError::InternalServerError(err.msg))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(ListSchemasError::ServiceUnavailable(err.msg))
                }
                "UnauthorizedException" => {
                    return RusotoError::Service(ListSchemasError::Unauthorized(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ListSchemasError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListSchemasError::BadRequest(ref cause) => write!(f, "{}", cause),
            ListSchemasError::Forbidden(ref cause) => write!(f, "{}", cause),
            ListSchemasError::InternalServerError(ref cause) => write!(f, "{}", cause),
            ListSchemasError::ServiceUnavailable(ref cause) => write!(f, "{}", cause),
            ListSchemasError::Unauthorized(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListSchemasError {}
/// Errors returned by ListTagsForResource
#[derive(Debug, PartialEq)]
pub enum ListTagsForResourceError {
    BadRequest(String),

    Forbidden(String),

    InternalServerError(String),

    NotFound(String),
}

impl ListTagsForResourceError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListTagsForResourceError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(ListTagsForResourceError::BadRequest(err.msg))
                }
                "ForbiddenException" => {
                    return RusotoError::Service(ListTagsForResourceError::Forbidden(err.msg))
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(ListTagsForResourceError::InternalServerError(
                        err.msg,
                    ))
                }
                "NotFoundException" => {
                    return RusotoError::Service(ListTagsForResourceError::NotFound(err.msg))
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
            ListTagsForResourceError::BadRequest(ref cause) => write!(f, "{}", cause),
            ListTagsForResourceError::Forbidden(ref cause) => write!(f, "{}", cause),
            ListTagsForResourceError::InternalServerError(ref cause) => write!(f, "{}", cause),
            ListTagsForResourceError::NotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListTagsForResourceError {}
/// Errors returned by LockServiceLinkedRole
#[derive(Debug, PartialEq)]
pub enum LockServiceLinkedRoleError {
    BadRequest(String),

    Forbidden(String),

    InternalServerError(String),

    ServiceUnavailable(String),

    Unauthorized(String),
}

impl LockServiceLinkedRoleError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<LockServiceLinkedRoleError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(LockServiceLinkedRoleError::BadRequest(err.msg))
                }
                "ForbiddenException" => {
                    return RusotoError::Service(LockServiceLinkedRoleError::Forbidden(err.msg))
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(LockServiceLinkedRoleError::InternalServerError(
                        err.msg,
                    ))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(LockServiceLinkedRoleError::ServiceUnavailable(
                        err.msg,
                    ))
                }
                "UnauthorizedException" => {
                    return RusotoError::Service(LockServiceLinkedRoleError::Unauthorized(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for LockServiceLinkedRoleError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            LockServiceLinkedRoleError::BadRequest(ref cause) => write!(f, "{}", cause),
            LockServiceLinkedRoleError::Forbidden(ref cause) => write!(f, "{}", cause),
            LockServiceLinkedRoleError::InternalServerError(ref cause) => write!(f, "{}", cause),
            LockServiceLinkedRoleError::ServiceUnavailable(ref cause) => write!(f, "{}", cause),
            LockServiceLinkedRoleError::Unauthorized(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for LockServiceLinkedRoleError {}
/// Errors returned by PutCodeBinding
#[derive(Debug, PartialEq)]
pub enum PutCodeBindingError {
    BadRequest(String),

    Forbidden(String),

    Gone(String),

    InternalServerError(String),

    NotFound(String),

    TooManyRequests(String),

    Unauthorized(String),
}

impl PutCodeBindingError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<PutCodeBindingError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(PutCodeBindingError::BadRequest(err.msg))
                }
                "ForbiddenException" => {
                    return RusotoError::Service(PutCodeBindingError::Forbidden(err.msg))
                }
                "GoneException" => return RusotoError::Service(PutCodeBindingError::Gone(err.msg)),
                "InternalServerErrorException" => {
                    return RusotoError::Service(PutCodeBindingError::InternalServerError(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(PutCodeBindingError::NotFound(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(PutCodeBindingError::TooManyRequests(err.msg))
                }
                "UnauthorizedException" => {
                    return RusotoError::Service(PutCodeBindingError::Unauthorized(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for PutCodeBindingError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            PutCodeBindingError::BadRequest(ref cause) => write!(f, "{}", cause),
            PutCodeBindingError::Forbidden(ref cause) => write!(f, "{}", cause),
            PutCodeBindingError::Gone(ref cause) => write!(f, "{}", cause),
            PutCodeBindingError::InternalServerError(ref cause) => write!(f, "{}", cause),
            PutCodeBindingError::NotFound(ref cause) => write!(f, "{}", cause),
            PutCodeBindingError::TooManyRequests(ref cause) => write!(f, "{}", cause),
            PutCodeBindingError::Unauthorized(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for PutCodeBindingError {}
/// Errors returned by SearchSchemas
#[derive(Debug, PartialEq)]
pub enum SearchSchemasError {
    BadRequest(String),

    Forbidden(String),

    InternalServerError(String),

    ServiceUnavailable(String),

    Unauthorized(String),
}

impl SearchSchemasError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<SearchSchemasError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(SearchSchemasError::BadRequest(err.msg))
                }
                "ForbiddenException" => {
                    return RusotoError::Service(SearchSchemasError::Forbidden(err.msg))
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(SearchSchemasError::InternalServerError(err.msg))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(SearchSchemasError::ServiceUnavailable(err.msg))
                }
                "UnauthorizedException" => {
                    return RusotoError::Service(SearchSchemasError::Unauthorized(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for SearchSchemasError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            SearchSchemasError::BadRequest(ref cause) => write!(f, "{}", cause),
            SearchSchemasError::Forbidden(ref cause) => write!(f, "{}", cause),
            SearchSchemasError::InternalServerError(ref cause) => write!(f, "{}", cause),
            SearchSchemasError::ServiceUnavailable(ref cause) => write!(f, "{}", cause),
            SearchSchemasError::Unauthorized(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for SearchSchemasError {}
/// Errors returned by StartDiscoverer
#[derive(Debug, PartialEq)]
pub enum StartDiscovererError {
    BadRequest(String),

    Forbidden(String),

    InternalServerError(String),

    NotFound(String),

    ServiceUnavailable(String),

    Unauthorized(String),
}

impl StartDiscovererError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<StartDiscovererError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(StartDiscovererError::BadRequest(err.msg))
                }
                "ForbiddenException" => {
                    return RusotoError::Service(StartDiscovererError::Forbidden(err.msg))
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(StartDiscovererError::InternalServerError(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(StartDiscovererError::NotFound(err.msg))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(StartDiscovererError::ServiceUnavailable(err.msg))
                }
                "UnauthorizedException" => {
                    return RusotoError::Service(StartDiscovererError::Unauthorized(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for StartDiscovererError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            StartDiscovererError::BadRequest(ref cause) => write!(f, "{}", cause),
            StartDiscovererError::Forbidden(ref cause) => write!(f, "{}", cause),
            StartDiscovererError::InternalServerError(ref cause) => write!(f, "{}", cause),
            StartDiscovererError::NotFound(ref cause) => write!(f, "{}", cause),
            StartDiscovererError::ServiceUnavailable(ref cause) => write!(f, "{}", cause),
            StartDiscovererError::Unauthorized(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for StartDiscovererError {}
/// Errors returned by StopDiscoverer
#[derive(Debug, PartialEq)]
pub enum StopDiscovererError {
    BadRequest(String),

    Forbidden(String),

    InternalServerError(String),

    NotFound(String),

    ServiceUnavailable(String),

    Unauthorized(String),
}

impl StopDiscovererError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<StopDiscovererError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(StopDiscovererError::BadRequest(err.msg))
                }
                "ForbiddenException" => {
                    return RusotoError::Service(StopDiscovererError::Forbidden(err.msg))
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(StopDiscovererError::InternalServerError(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(StopDiscovererError::NotFound(err.msg))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(StopDiscovererError::ServiceUnavailable(err.msg))
                }
                "UnauthorizedException" => {
                    return RusotoError::Service(StopDiscovererError::Unauthorized(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for StopDiscovererError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            StopDiscovererError::BadRequest(ref cause) => write!(f, "{}", cause),
            StopDiscovererError::Forbidden(ref cause) => write!(f, "{}", cause),
            StopDiscovererError::InternalServerError(ref cause) => write!(f, "{}", cause),
            StopDiscovererError::NotFound(ref cause) => write!(f, "{}", cause),
            StopDiscovererError::ServiceUnavailable(ref cause) => write!(f, "{}", cause),
            StopDiscovererError::Unauthorized(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for StopDiscovererError {}
/// Errors returned by TagResource
#[derive(Debug, PartialEq)]
pub enum TagResourceError {
    BadRequest(String),

    Forbidden(String),

    InternalServerError(String),

    NotFound(String),
}

impl TagResourceError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<TagResourceError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(TagResourceError::BadRequest(err.msg))
                }
                "ForbiddenException" => {
                    return RusotoError::Service(TagResourceError::Forbidden(err.msg))
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(TagResourceError::InternalServerError(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(TagResourceError::NotFound(err.msg))
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
            TagResourceError::BadRequest(ref cause) => write!(f, "{}", cause),
            TagResourceError::Forbidden(ref cause) => write!(f, "{}", cause),
            TagResourceError::InternalServerError(ref cause) => write!(f, "{}", cause),
            TagResourceError::NotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for TagResourceError {}
/// Errors returned by UnlockServiceLinkedRole
#[derive(Debug, PartialEq)]
pub enum UnlockServiceLinkedRoleError {
    BadRequest(String),

    Forbidden(String),

    InternalServerError(String),

    ServiceUnavailable(String),

    Unauthorized(String),
}

impl UnlockServiceLinkedRoleError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UnlockServiceLinkedRoleError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(UnlockServiceLinkedRoleError::BadRequest(err.msg))
                }
                "ForbiddenException" => {
                    return RusotoError::Service(UnlockServiceLinkedRoleError::Forbidden(err.msg))
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(UnlockServiceLinkedRoleError::InternalServerError(
                        err.msg,
                    ))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(UnlockServiceLinkedRoleError::ServiceUnavailable(
                        err.msg,
                    ))
                }
                "UnauthorizedException" => {
                    return RusotoError::Service(UnlockServiceLinkedRoleError::Unauthorized(
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
impl fmt::Display for UnlockServiceLinkedRoleError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UnlockServiceLinkedRoleError::BadRequest(ref cause) => write!(f, "{}", cause),
            UnlockServiceLinkedRoleError::Forbidden(ref cause) => write!(f, "{}", cause),
            UnlockServiceLinkedRoleError::InternalServerError(ref cause) => write!(f, "{}", cause),
            UnlockServiceLinkedRoleError::ServiceUnavailable(ref cause) => write!(f, "{}", cause),
            UnlockServiceLinkedRoleError::Unauthorized(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for UnlockServiceLinkedRoleError {}
/// Errors returned by UntagResource
#[derive(Debug, PartialEq)]
pub enum UntagResourceError {
    BadRequest(String),

    Forbidden(String),

    InternalServerError(String),

    NotFound(String),
}

impl UntagResourceError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UntagResourceError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(UntagResourceError::BadRequest(err.msg))
                }
                "ForbiddenException" => {
                    return RusotoError::Service(UntagResourceError::Forbidden(err.msg))
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(UntagResourceError::InternalServerError(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(UntagResourceError::NotFound(err.msg))
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
            UntagResourceError::BadRequest(ref cause) => write!(f, "{}", cause),
            UntagResourceError::Forbidden(ref cause) => write!(f, "{}", cause),
            UntagResourceError::InternalServerError(ref cause) => write!(f, "{}", cause),
            UntagResourceError::NotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for UntagResourceError {}
/// Errors returned by UpdateDiscoverer
#[derive(Debug, PartialEq)]
pub enum UpdateDiscovererError {
    BadRequest(String),

    Forbidden(String),

    InternalServerError(String),

    NotFound(String),

    ServiceUnavailable(String),

    Unauthorized(String),
}

impl UpdateDiscovererError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdateDiscovererError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(UpdateDiscovererError::BadRequest(err.msg))
                }
                "ForbiddenException" => {
                    return RusotoError::Service(UpdateDiscovererError::Forbidden(err.msg))
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(UpdateDiscovererError::InternalServerError(
                        err.msg,
                    ))
                }
                "NotFoundException" => {
                    return RusotoError::Service(UpdateDiscovererError::NotFound(err.msg))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(UpdateDiscovererError::ServiceUnavailable(err.msg))
                }
                "UnauthorizedException" => {
                    return RusotoError::Service(UpdateDiscovererError::Unauthorized(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for UpdateDiscovererError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UpdateDiscovererError::BadRequest(ref cause) => write!(f, "{}", cause),
            UpdateDiscovererError::Forbidden(ref cause) => write!(f, "{}", cause),
            UpdateDiscovererError::InternalServerError(ref cause) => write!(f, "{}", cause),
            UpdateDiscovererError::NotFound(ref cause) => write!(f, "{}", cause),
            UpdateDiscovererError::ServiceUnavailable(ref cause) => write!(f, "{}", cause),
            UpdateDiscovererError::Unauthorized(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for UpdateDiscovererError {}
/// Errors returned by UpdateRegistry
#[derive(Debug, PartialEq)]
pub enum UpdateRegistryError {
    BadRequest(String),

    Forbidden(String),

    InternalServerError(String),

    NotFound(String),

    ServiceUnavailable(String),

    Unauthorized(String),
}

impl UpdateRegistryError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdateRegistryError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(UpdateRegistryError::BadRequest(err.msg))
                }
                "ForbiddenException" => {
                    return RusotoError::Service(UpdateRegistryError::Forbidden(err.msg))
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(UpdateRegistryError::InternalServerError(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(UpdateRegistryError::NotFound(err.msg))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(UpdateRegistryError::ServiceUnavailable(err.msg))
                }
                "UnauthorizedException" => {
                    return RusotoError::Service(UpdateRegistryError::Unauthorized(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for UpdateRegistryError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UpdateRegistryError::BadRequest(ref cause) => write!(f, "{}", cause),
            UpdateRegistryError::Forbidden(ref cause) => write!(f, "{}", cause),
            UpdateRegistryError::InternalServerError(ref cause) => write!(f, "{}", cause),
            UpdateRegistryError::NotFound(ref cause) => write!(f, "{}", cause),
            UpdateRegistryError::ServiceUnavailable(ref cause) => write!(f, "{}", cause),
            UpdateRegistryError::Unauthorized(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for UpdateRegistryError {}
/// Errors returned by UpdateSchema
#[derive(Debug, PartialEq)]
pub enum UpdateSchemaError {
    BadRequest(String),

    Forbidden(String),

    InternalServerError(String),

    NotFound(String),

    ServiceUnavailable(String),
}

impl UpdateSchemaError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdateSchemaError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(UpdateSchemaError::BadRequest(err.msg))
                }
                "ForbiddenException" => {
                    return RusotoError::Service(UpdateSchemaError::Forbidden(err.msg))
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(UpdateSchemaError::InternalServerError(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(UpdateSchemaError::NotFound(err.msg))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(UpdateSchemaError::ServiceUnavailable(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for UpdateSchemaError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UpdateSchemaError::BadRequest(ref cause) => write!(f, "{}", cause),
            UpdateSchemaError::Forbidden(ref cause) => write!(f, "{}", cause),
            UpdateSchemaError::InternalServerError(ref cause) => write!(f, "{}", cause),
            UpdateSchemaError::NotFound(ref cause) => write!(f, "{}", cause),
            UpdateSchemaError::ServiceUnavailable(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for UpdateSchemaError {}
/// Trait representing the capabilities of the Schemas API. Schemas clients implement this trait.
pub trait Schemas {
    /// <p>Creates a discoverer.</p>
    fn create_discoverer(
        &self,
        input: CreateDiscovererRequest,
    ) -> Pin<
        Box<
            dyn Future<
                    Output = Result<CreateDiscovererResponse, RusotoError<CreateDiscovererError>>,
                > + Send
                + 'static,
        >,
    >;

    /// <p>Creates a registry.</p>
    fn create_registry(
        &self,
        input: CreateRegistryRequest,
    ) -> Pin<
        Box<
            dyn Future<Output = Result<CreateRegistryResponse, RusotoError<CreateRegistryError>>>
                + Send
                + 'static,
        >,
    >;

    /// <p>Creates a schema definition.</p>
    fn create_schema(
        &self,
        input: CreateSchemaRequest,
    ) -> Pin<
        Box<
            dyn Future<Output = Result<CreateSchemaResponse, RusotoError<CreateSchemaError>>>
                + Send
                + 'static,
        >,
    >;

    /// <p>Deletes a discoverer.</p>
    fn delete_discoverer(
        &self,
        input: DeleteDiscovererRequest,
    ) -> Pin<
        Box<dyn Future<Output = Result<(), RusotoError<DeleteDiscovererError>>> + Send + 'static>,
    >;

    /// <p>Deletes a Registry.</p>
    fn delete_registry(
        &self,
        input: DeleteRegistryRequest,
    ) -> Pin<Box<dyn Future<Output = Result<(), RusotoError<DeleteRegistryError>>> + Send + 'static>>;

    /// <p>Delete a schema definition.</p>
    fn delete_schema(
        &self,
        input: DeleteSchemaRequest,
    ) -> Pin<Box<dyn Future<Output = Result<(), RusotoError<DeleteSchemaError>>> + Send + 'static>>;

    /// <p>Delete the schema version definition</p>
    fn delete_schema_version(
        &self,
        input: DeleteSchemaVersionRequest,
    ) -> Pin<
        Box<
            dyn Future<Output = Result<(), RusotoError<DeleteSchemaVersionError>>> + Send + 'static,
        >,
    >;

    /// <p>Describe the code binding URI.</p>
    fn describe_code_binding(
        &self,
        input: DescribeCodeBindingRequest,
    ) -> Pin<
        Box<
            dyn Future<
                    Output = Result<
                        DescribeCodeBindingResponse,
                        RusotoError<DescribeCodeBindingError>,
                    >,
                > + Send
                + 'static,
        >,
    >;

    /// <p>Describes the discoverer.</p>
    fn describe_discoverer(
        &self,
        input: DescribeDiscovererRequest,
    ) -> Pin<
        Box<
            dyn Future<
                    Output = Result<
                        DescribeDiscovererResponse,
                        RusotoError<DescribeDiscovererError>,
                    >,
                > + Send
                + 'static,
        >,
    >;

    /// <p>Describes the registry.</p>
    fn describe_registry(
        &self,
        input: DescribeRegistryRequest,
    ) -> Pin<
        Box<
            dyn Future<
                    Output = Result<DescribeRegistryResponse, RusotoError<DescribeRegistryError>>,
                > + Send
                + 'static,
        >,
    >;

    /// <p>Retrieve the schema definition.</p>
    fn describe_schema(
        &self,
        input: DescribeSchemaRequest,
    ) -> Pin<
        Box<
            dyn Future<Output = Result<DescribeSchemaResponse, RusotoError<DescribeSchemaError>>>
                + Send
                + 'static,
        >,
    >;

    /// <p>Get the code binding source URI.</p>
    fn get_code_binding_source(
        &self,
        input: GetCodeBindingSourceRequest,
    ) -> Pin<
        Box<
            dyn Future<
                    Output = Result<
                        GetCodeBindingSourceResponse,
                        RusotoError<GetCodeBindingSourceError>,
                    >,
                > + Send
                + 'static,
        >,
    >;

    /// <p>Get the discovered schema that was generated based on sampled events.</p>
    fn get_discovered_schema(
        &self,
        input: GetDiscoveredSchemaRequest,
    ) -> Pin<
        Box<
            dyn Future<
                    Output = Result<
                        GetDiscoveredSchemaResponse,
                        RusotoError<GetDiscoveredSchemaError>,
                    >,
                > + Send
                + 'static,
        >,
    >;

    /// <p>List the discoverers.</p>
    fn list_discoverers(
        &self,
        input: ListDiscoverersRequest,
    ) -> Pin<
        Box<
            dyn Future<Output = Result<ListDiscoverersResponse, RusotoError<ListDiscoverersError>>>
                + Send
                + 'static,
        >,
    >;

    /// <p>List the registries.</p>
    fn list_registries(
        &self,
        input: ListRegistriesRequest,
    ) -> Pin<
        Box<
            dyn Future<Output = Result<ListRegistriesResponse, RusotoError<ListRegistriesError>>>
                + Send
                + 'static,
        >,
    >;

    /// <p>Provides a list of the schema versions and related information.</p>
    fn list_schema_versions(
        &self,
        input: ListSchemaVersionsRequest,
    ) -> Pin<
        Box<
            dyn Future<
                    Output = Result<
                        ListSchemaVersionsResponse,
                        RusotoError<ListSchemaVersionsError>,
                    >,
                > + Send
                + 'static,
        >,
    >;

    /// <p>List the schemas.</p>
    fn list_schemas(
        &self,
        input: ListSchemasRequest,
    ) -> Pin<
        Box<
            dyn Future<Output = Result<ListSchemasResponse, RusotoError<ListSchemasError>>>
                + Send
                + 'static,
        >,
    >;

    /// <p>Get tags for resource.</p>
    fn list_tags_for_resource(
        &self,
        input: ListTagsForResourceRequest,
    ) -> Pin<
        Box<
            dyn Future<
                    Output = Result<
                        ListTagsForResourceResponse,
                        RusotoError<ListTagsForResourceError>,
                    >,
                > + Send
                + 'static,
        >,
    >;

    fn lock_service_linked_role(
        &self,
        input: LockServiceLinkedRoleRequest,
    ) -> Pin<
        Box<
            dyn Future<
                    Output = Result<
                        LockServiceLinkedRoleResponse,
                        RusotoError<LockServiceLinkedRoleError>,
                    >,
                > + Send
                + 'static,
        >,
    >;

    /// <p>Put code binding URI</p>
    fn put_code_binding(
        &self,
        input: PutCodeBindingRequest,
    ) -> Pin<
        Box<
            dyn Future<Output = Result<PutCodeBindingResponse, RusotoError<PutCodeBindingError>>>
                + Send
                + 'static,
        >,
    >;

    /// <p>Search the schemas</p>
    fn search_schemas(
        &self,
        input: SearchSchemasRequest,
    ) -> Pin<
        Box<
            dyn Future<Output = Result<SearchSchemasResponse, RusotoError<SearchSchemasError>>>
                + Send
                + 'static,
        >,
    >;

    /// <p>Starts the discoverer</p>
    fn start_discoverer(
        &self,
        input: StartDiscovererRequest,
    ) -> Pin<
        Box<
            dyn Future<Output = Result<StartDiscovererResponse, RusotoError<StartDiscovererError>>>
                + Send
                + 'static,
        >,
    >;

    /// <p>Stops the discoverer</p>
    fn stop_discoverer(
        &self,
        input: StopDiscovererRequest,
    ) -> Pin<
        Box<
            dyn Future<Output = Result<StopDiscovererResponse, RusotoError<StopDiscovererError>>>
                + Send
                + 'static,
        >,
    >;

    /// <p>Add tags to a resource.</p>
    fn tag_resource(
        &self,
        input: TagResourceRequest,
    ) -> Pin<Box<dyn Future<Output = Result<(), RusotoError<TagResourceError>>> + Send + 'static>>;

    fn unlock_service_linked_role(
        &self,
        input: UnlockServiceLinkedRoleRequest,
    ) -> Pin<
        Box<
            dyn Future<
                    Output = Result<
                        UnlockServiceLinkedRoleResponse,
                        RusotoError<UnlockServiceLinkedRoleError>,
                    >,
                > + Send
                + 'static,
        >,
    >;

    /// <p>Removes tags from a resource.</p>
    fn untag_resource(
        &self,
        input: UntagResourceRequest,
    ) -> Pin<Box<dyn Future<Output = Result<(), RusotoError<UntagResourceError>>> + Send + 'static>>;

    /// <p>Updates the discoverer</p>
    fn update_discoverer(
        &self,
        input: UpdateDiscovererRequest,
    ) -> Pin<
        Box<
            dyn Future<
                    Output = Result<UpdateDiscovererResponse, RusotoError<UpdateDiscovererError>>,
                > + Send
                + 'static,
        >,
    >;

    /// <p>Updates a registry.</p>
    fn update_registry(
        &self,
        input: UpdateRegistryRequest,
    ) -> Pin<
        Box<
            dyn Future<Output = Result<UpdateRegistryResponse, RusotoError<UpdateRegistryError>>>
                + Send
                + 'static,
        >,
    >;

    /// <p>Updates the schema definition</p>
    fn update_schema(
        &self,
        input: UpdateSchemaRequest,
    ) -> Pin<
        Box<
            dyn Future<Output = Result<UpdateSchemaResponse, RusotoError<UpdateSchemaError>>>
                + Send
                + 'static,
        >,
    >;
}
/// A client for the Schemas API.
#[derive(Clone)]
pub struct SchemasClient {
    client: Client,
    region: region::Region,
}

impl SchemasClient {
    /// Creates a client backed by the default tokio event loop.
    ///
    /// The client will use the default credentials provider and tls client.
    pub fn new(region: region::Region) -> SchemasClient {
        SchemasClient {
            client: Client::shared(),
            region,
        }
    }

    pub fn new_with<P, D>(
        request_dispatcher: D,
        credentials_provider: P,
        region: region::Region,
    ) -> SchemasClient
    where
        P: ProvideAwsCredentials + Send + Sync + 'static,
        D: DispatchSignedRequest + Send + Sync + 'static,
    {
        SchemasClient {
            client: Client::new_with(credentials_provider, request_dispatcher),
            region,
        }
    }

    pub fn new_with_client(client: Client, region: region::Region) -> SchemasClient {
        SchemasClient { client, region }
    }
}

impl Schemas for SchemasClient {
    /// <p>Creates a discoverer.</p>
    fn create_discoverer(
        &self,
        input: CreateDiscovererRequest,
    ) -> Pin<
        Box<
            dyn Future<
                    Output = Result<CreateDiscovererResponse, RusotoError<CreateDiscovererError>>,
                > + Send
                + 'static,
        >,
    > {
        let request_uri = "/v1/discoverers";

        let mut request = SignedRequest::new("POST", "schemas", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let fut = self.client.sign_and_dispatch(request);
        async move {
            let mut response = fut.await.map_err(RusotoError::from)?;
            if response.status.as_u16() == 201 {
                let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
                let result = proto::json::ResponsePayload::new(&response)
                    .deserialize::<CreateDiscovererResponse, _>()?;

                Ok(result)
            } else {
                let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
                Err(CreateDiscovererError::from_response(response))
            }
        }
        .boxed()
    }

    /// <p>Creates a registry.</p>
    fn create_registry(
        &self,
        input: CreateRegistryRequest,
    ) -> Pin<
        Box<
            dyn Future<Output = Result<CreateRegistryResponse, RusotoError<CreateRegistryError>>>
                + Send
                + 'static,
        >,
    > {
        let request_uri = format!(
            "/v1/registries/name/{registry_name}",
            registry_name = input.registry_name
        );

        let mut request = SignedRequest::new("POST", "schemas", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let fut = self.client.sign_and_dispatch(request);
        async move {
            let mut response = fut.await.map_err(RusotoError::from)?;
            if response.status.as_u16() == 201 {
                let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
                let result = proto::json::ResponsePayload::new(&response)
                    .deserialize::<CreateRegistryResponse, _>()?;

                Ok(result)
            } else {
                let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
                Err(CreateRegistryError::from_response(response))
            }
        }
        .boxed()
    }

    /// <p>Creates a schema definition.</p>
    fn create_schema(
        &self,
        input: CreateSchemaRequest,
    ) -> Pin<
        Box<
            dyn Future<Output = Result<CreateSchemaResponse, RusotoError<CreateSchemaError>>>
                + Send
                + 'static,
        >,
    > {
        let request_uri = format!(
            "/v1/registries/name/{registry_name}/schemas/name/{schema_name}",
            registry_name = input.registry_name,
            schema_name = input.schema_name
        );

        let mut request = SignedRequest::new("POST", "schemas", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let fut = self.client.sign_and_dispatch(request);
        async move {
            let mut response = fut.await.map_err(RusotoError::from)?;
            if response.status.as_u16() == 201 {
                let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
                let result = proto::json::ResponsePayload::new(&response)
                    .deserialize::<CreateSchemaResponse, _>()?;

                Ok(result)
            } else {
                let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
                Err(CreateSchemaError::from_response(response))
            }
        }
        .boxed()
    }

    /// <p>Deletes a discoverer.</p>
    fn delete_discoverer(
        &self,
        input: DeleteDiscovererRequest,
    ) -> Pin<
        Box<dyn Future<Output = Result<(), RusotoError<DeleteDiscovererError>>> + Send + 'static>,
    > {
        let request_uri = format!(
            "/v1/discoverers/id/{discoverer_id}",
            discoverer_id = input.discoverer_id
        );

        let mut request = SignedRequest::new("DELETE", "schemas", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let fut = self.client.sign_and_dispatch(request);
        async move {
            let mut response = fut.await.map_err(RusotoError::from)?;
            if response.status.as_u16() == 204 {
                let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
                let result = ::std::mem::drop(response);

                Ok(result)
            } else {
                let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
                Err(DeleteDiscovererError::from_response(response))
            }
        }
        .boxed()
    }

    /// <p>Deletes a Registry.</p>
    fn delete_registry(
        &self,
        input: DeleteRegistryRequest,
    ) -> Pin<Box<dyn Future<Output = Result<(), RusotoError<DeleteRegistryError>>> + Send + 'static>>
    {
        let request_uri = format!(
            "/v1/registries/name/{registry_name}",
            registry_name = input.registry_name
        );

        let mut request = SignedRequest::new("DELETE", "schemas", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let fut = self.client.sign_and_dispatch(request);
        async move {
            let mut response = fut.await.map_err(RusotoError::from)?;
            if response.status.as_u16() == 204 {
                let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
                let result = ::std::mem::drop(response);

                Ok(result)
            } else {
                let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
                Err(DeleteRegistryError::from_response(response))
            }
        }
        .boxed()
    }

    /// <p>Delete a schema definition.</p>
    fn delete_schema(
        &self,
        input: DeleteSchemaRequest,
    ) -> Pin<Box<dyn Future<Output = Result<(), RusotoError<DeleteSchemaError>>> + Send + 'static>>
    {
        let request_uri = format!(
            "/v1/registries/name/{registry_name}/schemas/name/{schema_name}",
            registry_name = input.registry_name,
            schema_name = input.schema_name
        );

        let mut request = SignedRequest::new("DELETE", "schemas", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let fut = self.client.sign_and_dispatch(request);
        async move {
            let mut response = fut.await.map_err(RusotoError::from)?;
            if response.status.as_u16() == 204 {
                let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
                let result = ::std::mem::drop(response);

                Ok(result)
            } else {
                let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
                Err(DeleteSchemaError::from_response(response))
            }
        }
        .boxed()
    }

    /// <p>Delete the schema version definition</p>
    fn delete_schema_version(
        &self,
        input: DeleteSchemaVersionRequest,
    ) -> Pin<
        Box<
            dyn Future<Output = Result<(), RusotoError<DeleteSchemaVersionError>>> + Send + 'static,
        >,
    > {
        let request_uri = format!("/v1/registries/name/{registry_name}/schemas/name/{schema_name}/version/{schema_version}", registry_name = input.registry_name, schema_name = input.schema_name, schema_version = input.schema_version);

        let mut request = SignedRequest::new("DELETE", "schemas", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let fut = self.client.sign_and_dispatch(request);
        async move {
            let mut response = fut.await.map_err(RusotoError::from)?;
            if response.status.as_u16() == 204 {
                let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
                let result = ::std::mem::drop(response);

                Ok(result)
            } else {
                let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
                Err(DeleteSchemaVersionError::from_response(response))
            }
        }
        .boxed()
    }

    /// <p>Describe the code binding URI.</p>
    fn describe_code_binding(
        &self,
        input: DescribeCodeBindingRequest,
    ) -> Pin<
        Box<
            dyn Future<
                    Output = Result<
                        DescribeCodeBindingResponse,
                        RusotoError<DescribeCodeBindingError>,
                    >,
                > + Send
                + 'static,
        >,
    > {
        let request_uri = format!(
            "/v1/registries/name/{registry_name}/schemas/name/{schema_name}/language/{language}",
            language = input.language,
            registry_name = input.registry_name,
            schema_name = input.schema_name
        );

        let mut request = SignedRequest::new("GET", "schemas", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut params = Params::new();
        if let Some(ref x) = input.schema_version {
            params.put("schemaVersion", x);
        }
        request.set_params(params);

        let fut = self.client.sign_and_dispatch(request);
        async move {
            let mut response = fut.await.map_err(RusotoError::from)?;
            if response.status.as_u16() == 200 {
                let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
                let result = proto::json::ResponsePayload::new(&response)
                    .deserialize::<DescribeCodeBindingResponse, _>()?;

                Ok(result)
            } else {
                let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
                Err(DescribeCodeBindingError::from_response(response))
            }
        }
        .boxed()
    }

    /// <p>Describes the discoverer.</p>
    fn describe_discoverer(
        &self,
        input: DescribeDiscovererRequest,
    ) -> Pin<
        Box<
            dyn Future<
                    Output = Result<
                        DescribeDiscovererResponse,
                        RusotoError<DescribeDiscovererError>,
                    >,
                > + Send
                + 'static,
        >,
    > {
        let request_uri = format!(
            "/v1/discoverers/id/{discoverer_id}",
            discoverer_id = input.discoverer_id
        );

        let mut request = SignedRequest::new("GET", "schemas", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let fut = self.client.sign_and_dispatch(request);
        async move {
            let mut response = fut.await.map_err(RusotoError::from)?;
            if response.status.as_u16() == 200 {
                let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
                let result = proto::json::ResponsePayload::new(&response)
                    .deserialize::<DescribeDiscovererResponse, _>()?;

                Ok(result)
            } else {
                let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
                Err(DescribeDiscovererError::from_response(response))
            }
        }
        .boxed()
    }

    /// <p>Describes the registry.</p>
    fn describe_registry(
        &self,
        input: DescribeRegistryRequest,
    ) -> Pin<
        Box<
            dyn Future<
                    Output = Result<DescribeRegistryResponse, RusotoError<DescribeRegistryError>>,
                > + Send
                + 'static,
        >,
    > {
        let request_uri = format!(
            "/v1/registries/name/{registry_name}",
            registry_name = input.registry_name
        );

        let mut request = SignedRequest::new("GET", "schemas", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let fut = self.client.sign_and_dispatch(request);
        async move {
            let mut response = fut.await.map_err(RusotoError::from)?;
            if response.status.as_u16() == 200 {
                let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
                let result = proto::json::ResponsePayload::new(&response)
                    .deserialize::<DescribeRegistryResponse, _>()?;

                Ok(result)
            } else {
                let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
                Err(DescribeRegistryError::from_response(response))
            }
        }
        .boxed()
    }

    /// <p>Retrieve the schema definition.</p>
    fn describe_schema(
        &self,
        input: DescribeSchemaRequest,
    ) -> Pin<
        Box<
            dyn Future<Output = Result<DescribeSchemaResponse, RusotoError<DescribeSchemaError>>>
                + Send
                + 'static,
        >,
    > {
        let request_uri = format!(
            "/v1/registries/name/{registry_name}/schemas/name/{schema_name}",
            registry_name = input.registry_name,
            schema_name = input.schema_name
        );

        let mut request = SignedRequest::new("GET", "schemas", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut params = Params::new();
        if let Some(ref x) = input.schema_version {
            params.put("schemaVersion", x);
        }
        request.set_params(params);

        let fut = self.client.sign_and_dispatch(request);
        async move {
            let mut response = fut.await.map_err(RusotoError::from)?;
            if response.status.as_u16() == 200 {
                let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
                let result = proto::json::ResponsePayload::new(&response)
                    .deserialize::<DescribeSchemaResponse, _>()?;

                Ok(result)
            } else {
                let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
                Err(DescribeSchemaError::from_response(response))
            }
        }
        .boxed()
    }

    /// <p>Get the code binding source URI.</p>
    fn get_code_binding_source(
        &self,
        input: GetCodeBindingSourceRequest,
    ) -> Pin<
        Box<
            dyn Future<
                    Output = Result<
                        GetCodeBindingSourceResponse,
                        RusotoError<GetCodeBindingSourceError>,
                    >,
                > + Send
                + 'static,
        >,
    > {
        let request_uri = format!("/v1/registries/name/{registry_name}/schemas/name/{schema_name}/language/{language}/source", language = input.language, registry_name = input.registry_name, schema_name = input.schema_name);

        let mut request = SignedRequest::new("GET", "schemas", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut params = Params::new();
        if let Some(ref x) = input.schema_version {
            params.put("schemaVersion", x);
        }
        request.set_params(params);

        let fut = self.client.sign_and_dispatch(request);
        async move {
            let mut response = fut.await.map_err(RusotoError::from)?;
            if response.status.as_u16() == 200 {
                let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;

                let mut result = GetCodeBindingSourceResponse::default();
                result.body = Some(response.body);

                Ok(result)
            } else {
                let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
                Err(GetCodeBindingSourceError::from_response(response))
            }
        }
        .boxed()
    }

    /// <p>Get the discovered schema that was generated based on sampled events.</p>
    fn get_discovered_schema(
        &self,
        input: GetDiscoveredSchemaRequest,
    ) -> Pin<
        Box<
            dyn Future<
                    Output = Result<
                        GetDiscoveredSchemaResponse,
                        RusotoError<GetDiscoveredSchemaError>,
                    >,
                > + Send
                + 'static,
        >,
    > {
        let request_uri = "/v1/discover";

        let mut request = SignedRequest::new("POST", "schemas", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let fut = self.client.sign_and_dispatch(request);
        async move {
            let mut response = fut.await.map_err(RusotoError::from)?;
            if response.status.as_u16() == 200 {
                let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
                let result = proto::json::ResponsePayload::new(&response)
                    .deserialize::<GetDiscoveredSchemaResponse, _>()?;

                Ok(result)
            } else {
                let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
                Err(GetDiscoveredSchemaError::from_response(response))
            }
        }
        .boxed()
    }

    /// <p>List the discoverers.</p>
    fn list_discoverers(
        &self,
        input: ListDiscoverersRequest,
    ) -> Pin<
        Box<
            dyn Future<Output = Result<ListDiscoverersResponse, RusotoError<ListDiscoverersError>>>
                + Send
                + 'static,
        >,
    > {
        let request_uri = "/v1/discoverers";

        let mut request = SignedRequest::new("GET", "schemas", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut params = Params::new();
        if let Some(ref x) = input.discoverer_id_prefix {
            params.put("discovererIdPrefix", x);
        }
        if let Some(ref x) = input.limit {
            params.put("limit", x);
        }
        if let Some(ref x) = input.next_token {
            params.put("nextToken", x);
        }
        if let Some(ref x) = input.source_arn_prefix {
            params.put("sourceArnPrefix", x);
        }
        request.set_params(params);

        let fut = self.client.sign_and_dispatch(request);
        async move {
            let mut response = fut.await.map_err(RusotoError::from)?;
            if response.status.as_u16() == 200 {
                let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
                let result = proto::json::ResponsePayload::new(&response)
                    .deserialize::<ListDiscoverersResponse, _>()?;

                Ok(result)
            } else {
                let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
                Err(ListDiscoverersError::from_response(response))
            }
        }
        .boxed()
    }

    /// <p>List the registries.</p>
    fn list_registries(
        &self,
        input: ListRegistriesRequest,
    ) -> Pin<
        Box<
            dyn Future<Output = Result<ListRegistriesResponse, RusotoError<ListRegistriesError>>>
                + Send
                + 'static,
        >,
    > {
        let request_uri = "/v1/registries";

        let mut request = SignedRequest::new("GET", "schemas", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut params = Params::new();
        if let Some(ref x) = input.limit {
            params.put("limit", x);
        }
        if let Some(ref x) = input.next_token {
            params.put("nextToken", x);
        }
        if let Some(ref x) = input.registry_name_prefix {
            params.put("registryNamePrefix", x);
        }
        if let Some(ref x) = input.scope {
            params.put("scope", x);
        }
        request.set_params(params);

        let fut = self.client.sign_and_dispatch(request);
        async move {
            let mut response = fut.await.map_err(RusotoError::from)?;
            if response.status.as_u16() == 200 {
                let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
                let result = proto::json::ResponsePayload::new(&response)
                    .deserialize::<ListRegistriesResponse, _>()?;

                Ok(result)
            } else {
                let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
                Err(ListRegistriesError::from_response(response))
            }
        }
        .boxed()
    }

    /// <p>Provides a list of the schema versions and related information.</p>
    fn list_schema_versions(
        &self,
        input: ListSchemaVersionsRequest,
    ) -> Pin<
        Box<
            dyn Future<
                    Output = Result<
                        ListSchemaVersionsResponse,
                        RusotoError<ListSchemaVersionsError>,
                    >,
                > + Send
                + 'static,
        >,
    > {
        let request_uri = format!(
            "/v1/registries/name/{registry_name}/schemas/name/{schema_name}/versions",
            registry_name = input.registry_name,
            schema_name = input.schema_name
        );

        let mut request = SignedRequest::new("GET", "schemas", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut params = Params::new();
        if let Some(ref x) = input.limit {
            params.put("limit", x);
        }
        if let Some(ref x) = input.next_token {
            params.put("nextToken", x);
        }
        request.set_params(params);

        let fut = self.client.sign_and_dispatch(request);
        async move {
            let mut response = fut.await.map_err(RusotoError::from)?;
            if response.status.as_u16() == 200 {
                let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
                let result = proto::json::ResponsePayload::new(&response)
                    .deserialize::<ListSchemaVersionsResponse, _>()?;

                Ok(result)
            } else {
                let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
                Err(ListSchemaVersionsError::from_response(response))
            }
        }
        .boxed()
    }

    /// <p>List the schemas.</p>
    fn list_schemas(
        &self,
        input: ListSchemasRequest,
    ) -> Pin<
        Box<
            dyn Future<Output = Result<ListSchemasResponse, RusotoError<ListSchemasError>>>
                + Send
                + 'static,
        >,
    > {
        let request_uri = format!(
            "/v1/registries/name/{registry_name}/schemas",
            registry_name = input.registry_name
        );

        let mut request = SignedRequest::new("GET", "schemas", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut params = Params::new();
        if let Some(ref x) = input.limit {
            params.put("limit", x);
        }
        if let Some(ref x) = input.next_token {
            params.put("nextToken", x);
        }
        if let Some(ref x) = input.schema_name_prefix {
            params.put("schemaNamePrefix", x);
        }
        request.set_params(params);

        let fut = self.client.sign_and_dispatch(request);
        async move {
            let mut response = fut.await.map_err(RusotoError::from)?;
            if response.status.as_u16() == 200 {
                let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
                let result = proto::json::ResponsePayload::new(&response)
                    .deserialize::<ListSchemasResponse, _>()?;

                Ok(result)
            } else {
                let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
                Err(ListSchemasError::from_response(response))
            }
        }
        .boxed()
    }

    /// <p>Get tags for resource.</p>
    fn list_tags_for_resource(
        &self,
        input: ListTagsForResourceRequest,
    ) -> Pin<
        Box<
            dyn Future<
                    Output = Result<
                        ListTagsForResourceResponse,
                        RusotoError<ListTagsForResourceError>,
                    >,
                > + Send
                + 'static,
        >,
    > {
        let request_uri = format!("/tags/{resource_arn}", resource_arn = input.resource_arn);

        let mut request = SignedRequest::new("GET", "schemas", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let fut = self.client.sign_and_dispatch(request);
        async move {
            let mut response = fut.await.map_err(RusotoError::from)?;
            if response.status.as_u16() == 200 {
                let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
                let result = proto::json::ResponsePayload::new(&response)
                    .deserialize::<ListTagsForResourceResponse, _>()?;

                Ok(result)
            } else {
                let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
                Err(ListTagsForResourceError::from_response(response))
            }
        }
        .boxed()
    }

    fn lock_service_linked_role(
        &self,
        input: LockServiceLinkedRoleRequest,
    ) -> Pin<
        Box<
            dyn Future<
                    Output = Result<
                        LockServiceLinkedRoleResponse,
                        RusotoError<LockServiceLinkedRoleError>,
                    >,
                > + Send
                + 'static,
        >,
    > {
        let request_uri = "/slr-deletion/lock";

        let mut request = SignedRequest::new("POST", "schemas", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let fut = self.client.sign_and_dispatch(request);
        async move {
            let mut response = fut.await.map_err(RusotoError::from)?;
            if response.status.as_u16() == 200 {
                let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
                let result = proto::json::ResponsePayload::new(&response)
                    .deserialize::<LockServiceLinkedRoleResponse, _>()?;

                Ok(result)
            } else {
                let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
                Err(LockServiceLinkedRoleError::from_response(response))
            }
        }
        .boxed()
    }

    /// <p>Put code binding URI</p>
    fn put_code_binding(
        &self,
        input: PutCodeBindingRequest,
    ) -> Pin<
        Box<
            dyn Future<Output = Result<PutCodeBindingResponse, RusotoError<PutCodeBindingError>>>
                + Send
                + 'static,
        >,
    > {
        let request_uri = format!(
            "/v1/registries/name/{registry_name}/schemas/name/{schema_name}/language/{language}",
            language = input.language,
            registry_name = input.registry_name,
            schema_name = input.schema_name
        );

        let mut request = SignedRequest::new("POST", "schemas", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut params = Params::new();
        if let Some(ref x) = input.schema_version {
            params.put("schemaVersion", x);
        }
        request.set_params(params);

        let fut = self.client.sign_and_dispatch(request);
        async move {
            let mut response = fut.await.map_err(RusotoError::from)?;
            if response.status.as_u16() == 202 {
                let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
                let result = proto::json::ResponsePayload::new(&response)
                    .deserialize::<PutCodeBindingResponse, _>()?;

                Ok(result)
            } else {
                let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
                Err(PutCodeBindingError::from_response(response))
            }
        }
        .boxed()
    }

    /// <p>Search the schemas</p>
    fn search_schemas(
        &self,
        input: SearchSchemasRequest,
    ) -> Pin<
        Box<
            dyn Future<Output = Result<SearchSchemasResponse, RusotoError<SearchSchemasError>>>
                + Send
                + 'static,
        >,
    > {
        let request_uri = format!(
            "/v1/registries/name/{registry_name}/schemas/search",
            registry_name = input.registry_name
        );

        let mut request = SignedRequest::new("GET", "schemas", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut params = Params::new();
        params.put("keywords", &input.keywords);
        if let Some(ref x) = input.limit {
            params.put("limit", x);
        }
        if let Some(ref x) = input.next_token {
            params.put("nextToken", x);
        }
        request.set_params(params);

        let fut = self.client.sign_and_dispatch(request);
        async move {
            let mut response = fut.await.map_err(RusotoError::from)?;
            if response.status.as_u16() == 200 {
                let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
                let result = proto::json::ResponsePayload::new(&response)
                    .deserialize::<SearchSchemasResponse, _>()?;

                Ok(result)
            } else {
                let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
                Err(SearchSchemasError::from_response(response))
            }
        }
        .boxed()
    }

    /// <p>Starts the discoverer</p>
    fn start_discoverer(
        &self,
        input: StartDiscovererRequest,
    ) -> Pin<
        Box<
            dyn Future<Output = Result<StartDiscovererResponse, RusotoError<StartDiscovererError>>>
                + Send
                + 'static,
        >,
    > {
        let request_uri = format!(
            "/v1/discoverers/id/{discoverer_id}/start",
            discoverer_id = input.discoverer_id
        );

        let mut request = SignedRequest::new("POST", "schemas", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let fut = self.client.sign_and_dispatch(request);
        async move {
            let mut response = fut.await.map_err(RusotoError::from)?;
            if response.status.as_u16() == 200 {
                let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
                let result = proto::json::ResponsePayload::new(&response)
                    .deserialize::<StartDiscovererResponse, _>()?;

                Ok(result)
            } else {
                let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
                Err(StartDiscovererError::from_response(response))
            }
        }
        .boxed()
    }

    /// <p>Stops the discoverer</p>
    fn stop_discoverer(
        &self,
        input: StopDiscovererRequest,
    ) -> Pin<
        Box<
            dyn Future<Output = Result<StopDiscovererResponse, RusotoError<StopDiscovererError>>>
                + Send
                + 'static,
        >,
    > {
        let request_uri = format!(
            "/v1/discoverers/id/{discoverer_id}/stop",
            discoverer_id = input.discoverer_id
        );

        let mut request = SignedRequest::new("POST", "schemas", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let fut = self.client.sign_and_dispatch(request);
        async move {
            let mut response = fut.await.map_err(RusotoError::from)?;
            if response.status.as_u16() == 200 {
                let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
                let result = proto::json::ResponsePayload::new(&response)
                    .deserialize::<StopDiscovererResponse, _>()?;

                Ok(result)
            } else {
                let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
                Err(StopDiscovererError::from_response(response))
            }
        }
        .boxed()
    }

    /// <p>Add tags to a resource.</p>
    fn tag_resource(
        &self,
        input: TagResourceRequest,
    ) -> Pin<Box<dyn Future<Output = Result<(), RusotoError<TagResourceError>>> + Send + 'static>>
    {
        let request_uri = format!("/tags/{resource_arn}", resource_arn = input.resource_arn);

        let mut request = SignedRequest::new("POST", "schemas", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let fut = self.client.sign_and_dispatch(request);
        async move {
            let mut response = fut.await.map_err(RusotoError::from)?;
            if response.status.as_u16() == 204 {
                let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
                let result = ::std::mem::drop(response);

                Ok(result)
            } else {
                let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
                Err(TagResourceError::from_response(response))
            }
        }
        .boxed()
    }

    fn unlock_service_linked_role(
        &self,
        input: UnlockServiceLinkedRoleRequest,
    ) -> Pin<
        Box<
            dyn Future<
                    Output = Result<
                        UnlockServiceLinkedRoleResponse,
                        RusotoError<UnlockServiceLinkedRoleError>,
                    >,
                > + Send
                + 'static,
        >,
    > {
        let request_uri = "/slr-deletion/unlock";

        let mut request = SignedRequest::new("POST", "schemas", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let fut = self.client.sign_and_dispatch(request);
        async move {
            let mut response = fut.await.map_err(RusotoError::from)?;
            if response.status.as_u16() == 200 {
                let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
                let result = proto::json::ResponsePayload::new(&response)
                    .deserialize::<UnlockServiceLinkedRoleResponse, _>()?;

                Ok(result)
            } else {
                let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
                Err(UnlockServiceLinkedRoleError::from_response(response))
            }
        }
        .boxed()
    }

    /// <p>Removes tags from a resource.</p>
    fn untag_resource(
        &self,
        input: UntagResourceRequest,
    ) -> Pin<Box<dyn Future<Output = Result<(), RusotoError<UntagResourceError>>> + Send + 'static>>
    {
        let request_uri = format!("/tags/{resource_arn}", resource_arn = input.resource_arn);

        let mut request = SignedRequest::new("DELETE", "schemas", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut params = Params::new();
        for item in input.tag_keys.iter() {
            params.put("tagKeys", item);
        }
        request.set_params(params);

        let fut = self.client.sign_and_dispatch(request);
        async move {
            let mut response = fut.await.map_err(RusotoError::from)?;
            if response.status.as_u16() == 204 {
                let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
                let result = ::std::mem::drop(response);

                Ok(result)
            } else {
                let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
                Err(UntagResourceError::from_response(response))
            }
        }
        .boxed()
    }

    /// <p>Updates the discoverer</p>
    fn update_discoverer(
        &self,
        input: UpdateDiscovererRequest,
    ) -> Pin<
        Box<
            dyn Future<
                    Output = Result<UpdateDiscovererResponse, RusotoError<UpdateDiscovererError>>,
                > + Send
                + 'static,
        >,
    > {
        let request_uri = format!(
            "/v1/discoverers/id/{discoverer_id}",
            discoverer_id = input.discoverer_id
        );

        let mut request = SignedRequest::new("PUT", "schemas", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let fut = self.client.sign_and_dispatch(request);
        async move {
            let mut response = fut.await.map_err(RusotoError::from)?;
            if response.status.as_u16() == 200 {
                let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
                let result = proto::json::ResponsePayload::new(&response)
                    .deserialize::<UpdateDiscovererResponse, _>()?;

                Ok(result)
            } else {
                let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
                Err(UpdateDiscovererError::from_response(response))
            }
        }
        .boxed()
    }

    /// <p>Updates a registry.</p>
    fn update_registry(
        &self,
        input: UpdateRegistryRequest,
    ) -> Pin<
        Box<
            dyn Future<Output = Result<UpdateRegistryResponse, RusotoError<UpdateRegistryError>>>
                + Send
                + 'static,
        >,
    > {
        let request_uri = format!(
            "/v1/registries/name/{registry_name}",
            registry_name = input.registry_name
        );

        let mut request = SignedRequest::new("PUT", "schemas", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let fut = self.client.sign_and_dispatch(request);
        async move {
            let mut response = fut.await.map_err(RusotoError::from)?;
            if response.status.as_u16() == 200 {
                let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
                let result = proto::json::ResponsePayload::new(&response)
                    .deserialize::<UpdateRegistryResponse, _>()?;

                Ok(result)
            } else {
                let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
                Err(UpdateRegistryError::from_response(response))
            }
        }
        .boxed()
    }

    /// <p>Updates the schema definition</p>
    fn update_schema(
        &self,
        input: UpdateSchemaRequest,
    ) -> Pin<
        Box<
            dyn Future<Output = Result<UpdateSchemaResponse, RusotoError<UpdateSchemaError>>>
                + Send
                + 'static,
        >,
    > {
        let request_uri = format!(
            "/v1/registries/name/{registry_name}/schemas/name/{schema_name}",
            registry_name = input.registry_name,
            schema_name = input.schema_name
        );

        let mut request = SignedRequest::new("PUT", "schemas", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let fut = self.client.sign_and_dispatch(request);
        async move {
            let mut response = fut.await.map_err(RusotoError::from)?;
            if response.status.as_u16() == 200 {
                let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
                let result = proto::json::ResponsePayload::new(&response)
                    .deserialize::<UpdateSchemaResponse, _>()?;

                Ok(result)
            } else {
                let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
                Err(UpdateSchemaError::from_response(response))
            }
        }
        .boxed()
    }
}
