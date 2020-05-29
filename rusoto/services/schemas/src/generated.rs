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
    /// <p>The name of the registry.</p>
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
    /// <p>The source of the schema definition.</p>
    #[serde(rename = "Content")]
    pub content: String,
    /// <p>A description of the schema.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The name of the registry.</p>
    #[serde(rename = "RegistryName")]
    pub registry_name: String,
    /// <p>The name of the schema.</p>
    #[serde(rename = "SchemaName")]
    pub schema_name: String,
    /// <p>Tags associated with the schema.</p>
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<::std::collections::HashMap<String, String>>,
    /// <p>The type of schema.</p>
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
    /// <p>The ID of the discoverer.</p>
    #[serde(rename = "DiscovererId")]
    pub discoverer_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteRegistryRequest {
    /// <p>The name of the registry.</p>
    #[serde(rename = "RegistryName")]
    pub registry_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteResourcePolicyRequest {
    /// <p>The name of the registry.</p>
    #[serde(rename = "RegistryName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub registry_name: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteSchemaRequest {
    /// <p>The name of the registry.</p>
    #[serde(rename = "RegistryName")]
    pub registry_name: String,
    /// <p>The name of the schema.</p>
    #[serde(rename = "SchemaName")]
    pub schema_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteSchemaVersionRequest {
    /// <p>The name of the registry.</p>
    #[serde(rename = "RegistryName")]
    pub registry_name: String,
    /// <p>The name of the schema.</p>
    #[serde(rename = "SchemaName")]
    pub schema_name: String,
    /// <p>The version number of the schema</p>
    #[serde(rename = "SchemaVersion")]
    pub schema_version: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeCodeBindingRequest {
    /// <p>The language of the code binding.</p>
    #[serde(rename = "Language")]
    pub language: String,
    /// <p>The name of the registry.</p>
    #[serde(rename = "RegistryName")]
    pub registry_name: String,
    /// <p>The name of the schema.</p>
    #[serde(rename = "SchemaName")]
    pub schema_name: String,
    /// <p>Specifying this limits the results to only this schema version.</p>
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
    /// <p>The ID of the discoverer.</p>
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
    /// <p>The name of the registry.</p>
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
    /// <p>The name of the registry.</p>
    #[serde(rename = "RegistryName")]
    pub registry_name: String,
    /// <p>The name of the schema.</p>
    #[serde(rename = "SchemaName")]
    pub schema_name: String,
    /// <p>Specifying this limits the results to only this schema version.</p>
    #[serde(rename = "SchemaVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schema_version: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeSchemaResponse {
    /// <p>The source of the schema definition.</p>
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
pub struct GetCodeBindingSourceRequest {
    /// <p>The language of the code binding.</p>
    #[serde(rename = "Language")]
    pub language: String,
    /// <p>The name of the registry.</p>
    #[serde(rename = "RegistryName")]
    pub registry_name: String,
    /// <p>The name of the schema.</p>
    #[serde(rename = "SchemaName")]
    pub schema_name: String,
    /// <p>Specifying this limits the results to only this schema version.</p>
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
    /// <p>An array of strings where each string is a JSON event. These are the events that were used to generate the schema. The array includes a single type of event and has a maximum size of 10 events.</p>
    #[serde(rename = "Events")]
    pub events: Vec<String>,
    /// <p>The type of event.</p>
    #[serde(rename = "Type")]
    pub type_: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetDiscoveredSchemaResponse {
    /// <p>The source of the schema definition.</p>
    #[serde(rename = "Content")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetResourcePolicyRequest {
    /// <p>The name of the registry.</p>
    #[serde(rename = "RegistryName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub registry_name: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetResourcePolicyResponse {
    /// <p>The resource-based policy.</p>
    #[serde(rename = "Policy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy: Option<String>,
    /// <p>The revision ID.</p>
    #[serde(rename = "RevisionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub revision_id: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListDiscoverersRequest {
    /// <p>Specifying this limits the results to only those discoverer IDs that start with the specified prefix.</p>
    #[serde(rename = "DiscovererIdPrefix")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub discoverer_id_prefix: Option<String>,
    #[serde(rename = "Limit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// <p>The token that specifies the next page of results to return. To request the first page, leave NextToken empty. The token will expire in 24 hours, and cannot be shared with other accounts.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>Specifying this limits the results to only those ARNs that start with the specified prefix.</p>
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
    /// <p>The token that specifies the next page of results to return. To request the first page, leave NextToken empty. The token will expire in 24 hours, and cannot be shared with other accounts.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>Specifying this limits the results to only those registry names that start with the specified prefix.</p>
    #[serde(rename = "RegistryNamePrefix")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub registry_name_prefix: Option<String>,
    /// <p>Can be set to Local or AWS to limit responses to your custom registries, or the ones provided by AWS.</p>
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
    /// <p>The token that specifies the next page of results to return. To request the first page, leave NextToken empty. The token will expire in 24 hours, and cannot be shared with other accounts.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The name of the registry.</p>
    #[serde(rename = "RegistryName")]
    pub registry_name: String,
    /// <p>The name of the schema.</p>
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
    /// <p>The token that specifies the next page of results to return. To request the first page, leave NextToken empty. The token will expire in 24 hours, and cannot be shared with other accounts.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The name of the registry.</p>
    #[serde(rename = "RegistryName")]
    pub registry_name: String,
    /// <p>Specifying this limits the results to only those schema names that start with the specified prefix.</p>
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
    /// <p>The ARN of the resource.</p>
    #[serde(rename = "ResourceArn")]
    pub resource_arn: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListTagsForResourceResponse {
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<::std::collections::HashMap<String, String>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct PutCodeBindingRequest {
    /// <p>The language of the code binding.</p>
    #[serde(rename = "Language")]
    pub language: String,
    /// <p>The name of the registry.</p>
    #[serde(rename = "RegistryName")]
    pub registry_name: String,
    /// <p>The name of the schema.</p>
    #[serde(rename = "SchemaName")]
    pub schema_name: String,
    /// <p>Specifying this limits the results to only this schema version.</p>
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

/// <p>The name of the policy.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct PutResourcePolicyRequest {
    /// <p>The resource-based policy.</p>
    #[serde(rename = "Policy")]
    pub policy: String,
    /// <p>The name of the registry.</p>
    #[serde(rename = "RegistryName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub registry_name: Option<String>,
    /// <p>The revision ID of the policy.</p>
    #[serde(rename = "RevisionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub revision_id: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct PutResourcePolicyResponse {
    /// <p>The resource-based policy.</p>
    #[serde(rename = "Policy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy: Option<String>,
    /// <p>The revision ID of the policy.</p>
    #[serde(rename = "RevisionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub revision_id: Option<String>,
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
    /// <p>The date the schema version was created.</p>
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
    /// <p>Specifying this limits the results to only schemas that include the provided keywords.</p>
    #[serde(rename = "Keywords")]
    pub keywords: String,
    #[serde(rename = "Limit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// <p>The token that specifies the next page of results to return. To request the first page, leave NextToken empty. The token will expire in 24 hours, and cannot be shared with other accounts.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The name of the registry.</p>
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
    /// <p>The ID of the discoverer.</p>
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
    /// <p>The ID of the discoverer.</p>
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

/// <p></p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct TagResourceRequest {
    /// <p>The ARN of the resource.</p>
    #[serde(rename = "ResourceArn")]
    pub resource_arn: String,
    /// <p>Tags associated with the resource.</p>
    #[serde(rename = "Tags")]
    pub tags: ::std::collections::HashMap<String, String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UntagResourceRequest {
    /// <p>The ARN of the resource.</p>
    #[serde(rename = "ResourceArn")]
    pub resource_arn: String,
    /// <p>Keys of key-value pairs.</p>
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
    /// <p>The ID of the discoverer.</p>
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

/// <p>Updates the registry.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UpdateRegistryRequest {
    /// <p>The description of the registry to update.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The name of the registry.</p>
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
    /// <p>The name of the registry.</p>
    #[serde(rename = "RegistryName")]
    pub registry_name: String,
    /// <p>The name of the schema.</p>
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
/// Errors returned by DeleteResourcePolicy
#[derive(Debug, PartialEq)]
pub enum DeleteResourcePolicyError {
    BadRequest(String),

    Forbidden(String),

    InternalServerError(String),

    NotFound(String),

    ServiceUnavailable(String),

    Unauthorized(String),
}

impl DeleteResourcePolicyError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteResourcePolicyError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(DeleteResourcePolicyError::BadRequest(err.msg))
                }
                "ForbiddenException" => {
                    return RusotoError::Service(DeleteResourcePolicyError::Forbidden(err.msg))
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(DeleteResourcePolicyError::InternalServerError(
                        err.msg,
                    ))
                }
                "NotFoundException" => {
                    return RusotoError::Service(DeleteResourcePolicyError::NotFound(err.msg))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(DeleteResourcePolicyError::ServiceUnavailable(
                        err.msg,
                    ))
                }
                "UnauthorizedException" => {
                    return RusotoError::Service(DeleteResourcePolicyError::Unauthorized(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DeleteResourcePolicyError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteResourcePolicyError::BadRequest(ref cause) => write!(f, "{}", cause),
            DeleteResourcePolicyError::Forbidden(ref cause) => write!(f, "{}", cause),
            DeleteResourcePolicyError::InternalServerError(ref cause) => write!(f, "{}", cause),
            DeleteResourcePolicyError::NotFound(ref cause) => write!(f, "{}", cause),
            DeleteResourcePolicyError::ServiceUnavailable(ref cause) => write!(f, "{}", cause),
            DeleteResourcePolicyError::Unauthorized(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeleteResourcePolicyError {}
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
/// Errors returned by GetResourcePolicy
#[derive(Debug, PartialEq)]
pub enum GetResourcePolicyError {
    BadRequest(String),

    Forbidden(String),

    InternalServerError(String),

    NotFound(String),

    ServiceUnavailable(String),

    Unauthorized(String),
}

impl GetResourcePolicyError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetResourcePolicyError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(GetResourcePolicyError::BadRequest(err.msg))
                }
                "ForbiddenException" => {
                    return RusotoError::Service(GetResourcePolicyError::Forbidden(err.msg))
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(GetResourcePolicyError::InternalServerError(
                        err.msg,
                    ))
                }
                "NotFoundException" => {
                    return RusotoError::Service(GetResourcePolicyError::NotFound(err.msg))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(GetResourcePolicyError::ServiceUnavailable(
                        err.msg,
                    ))
                }
                "UnauthorizedException" => {
                    return RusotoError::Service(GetResourcePolicyError::Unauthorized(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for GetResourcePolicyError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetResourcePolicyError::BadRequest(ref cause) => write!(f, "{}", cause),
            GetResourcePolicyError::Forbidden(ref cause) => write!(f, "{}", cause),
            GetResourcePolicyError::InternalServerError(ref cause) => write!(f, "{}", cause),
            GetResourcePolicyError::NotFound(ref cause) => write!(f, "{}", cause),
            GetResourcePolicyError::ServiceUnavailable(ref cause) => write!(f, "{}", cause),
            GetResourcePolicyError::Unauthorized(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetResourcePolicyError {}
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
/// Errors returned by PutResourcePolicy
#[derive(Debug, PartialEq)]
pub enum PutResourcePolicyError {
    BadRequest(String),

    Forbidden(String),

    InternalServerError(String),

    NotFound(String),

    PreconditionFailed(String),

    ServiceUnavailable(String),

    Unauthorized(String),
}

impl PutResourcePolicyError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<PutResourcePolicyError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(PutResourcePolicyError::BadRequest(err.msg))
                }
                "ForbiddenException" => {
                    return RusotoError::Service(PutResourcePolicyError::Forbidden(err.msg))
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(PutResourcePolicyError::InternalServerError(
                        err.msg,
                    ))
                }
                "NotFoundException" => {
                    return RusotoError::Service(PutResourcePolicyError::NotFound(err.msg))
                }
                "PreconditionFailedException" => {
                    return RusotoError::Service(PutResourcePolicyError::PreconditionFailed(
                        err.msg,
                    ))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(PutResourcePolicyError::ServiceUnavailable(
                        err.msg,
                    ))
                }
                "UnauthorizedException" => {
                    return RusotoError::Service(PutResourcePolicyError::Unauthorized(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for PutResourcePolicyError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            PutResourcePolicyError::BadRequest(ref cause) => write!(f, "{}", cause),
            PutResourcePolicyError::Forbidden(ref cause) => write!(f, "{}", cause),
            PutResourcePolicyError::InternalServerError(ref cause) => write!(f, "{}", cause),
            PutResourcePolicyError::NotFound(ref cause) => write!(f, "{}", cause),
            PutResourcePolicyError::PreconditionFailed(ref cause) => write!(f, "{}", cause),
            PutResourcePolicyError::ServiceUnavailable(ref cause) => write!(f, "{}", cause),
            PutResourcePolicyError::Unauthorized(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for PutResourcePolicyError {}
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
#[async_trait]
pub trait Schemas {
    /// <p>Creates a discoverer.</p>
    async fn create_discoverer(
        &self,
        input: CreateDiscovererRequest,
    ) -> Result<CreateDiscovererResponse, RusotoError<CreateDiscovererError>>;

    /// <p>Creates a registry.</p>
    async fn create_registry(
        &self,
        input: CreateRegistryRequest,
    ) -> Result<CreateRegistryResponse, RusotoError<CreateRegistryError>>;

    /// <p><p>Creates a schema definition.</p> <note><p>Inactive schemas will be deleted after two years.</p></note></p>
    async fn create_schema(
        &self,
        input: CreateSchemaRequest,
    ) -> Result<CreateSchemaResponse, RusotoError<CreateSchemaError>>;

    /// <p>Deletes a discoverer.</p>
    async fn delete_discoverer(
        &self,
        input: DeleteDiscovererRequest,
    ) -> Result<(), RusotoError<DeleteDiscovererError>>;

    /// <p>Deletes a Registry.</p>
    async fn delete_registry(
        &self,
        input: DeleteRegistryRequest,
    ) -> Result<(), RusotoError<DeleteRegistryError>>;

    /// <p>Delete the resource-based policy attached to the specified registry.</p>
    async fn delete_resource_policy(
        &self,
        input: DeleteResourcePolicyRequest,
    ) -> Result<(), RusotoError<DeleteResourcePolicyError>>;

    /// <p>Delete a schema definition.</p>
    async fn delete_schema(
        &self,
        input: DeleteSchemaRequest,
    ) -> Result<(), RusotoError<DeleteSchemaError>>;

    /// <p>Delete the schema version definition</p>
    async fn delete_schema_version(
        &self,
        input: DeleteSchemaVersionRequest,
    ) -> Result<(), RusotoError<DeleteSchemaVersionError>>;

    /// <p>Describe the code binding URI.</p>
    async fn describe_code_binding(
        &self,
        input: DescribeCodeBindingRequest,
    ) -> Result<DescribeCodeBindingResponse, RusotoError<DescribeCodeBindingError>>;

    /// <p>Describes the discoverer.</p>
    async fn describe_discoverer(
        &self,
        input: DescribeDiscovererRequest,
    ) -> Result<DescribeDiscovererResponse, RusotoError<DescribeDiscovererError>>;

    /// <p>Describes the registry.</p>
    async fn describe_registry(
        &self,
        input: DescribeRegistryRequest,
    ) -> Result<DescribeRegistryResponse, RusotoError<DescribeRegistryError>>;

    /// <p>Retrieve the schema definition.</p>
    async fn describe_schema(
        &self,
        input: DescribeSchemaRequest,
    ) -> Result<DescribeSchemaResponse, RusotoError<DescribeSchemaError>>;

    /// <p>Get the code binding source URI.</p>
    async fn get_code_binding_source(
        &self,
        input: GetCodeBindingSourceRequest,
    ) -> Result<GetCodeBindingSourceResponse, RusotoError<GetCodeBindingSourceError>>;

    /// <p>Get the discovered schema that was generated based on sampled events.</p>
    async fn get_discovered_schema(
        &self,
        input: GetDiscoveredSchemaRequest,
    ) -> Result<GetDiscoveredSchemaResponse, RusotoError<GetDiscoveredSchemaError>>;

    /// <p>Retrieves the resource-based policy attached to a given registry.</p>
    async fn get_resource_policy(
        &self,
        input: GetResourcePolicyRequest,
    ) -> Result<GetResourcePolicyResponse, RusotoError<GetResourcePolicyError>>;

    /// <p>List the discoverers.</p>
    async fn list_discoverers(
        &self,
        input: ListDiscoverersRequest,
    ) -> Result<ListDiscoverersResponse, RusotoError<ListDiscoverersError>>;

    /// <p>List the registries.</p>
    async fn list_registries(
        &self,
        input: ListRegistriesRequest,
    ) -> Result<ListRegistriesResponse, RusotoError<ListRegistriesError>>;

    /// <p>Provides a list of the schema versions and related information.</p>
    async fn list_schema_versions(
        &self,
        input: ListSchemaVersionsRequest,
    ) -> Result<ListSchemaVersionsResponse, RusotoError<ListSchemaVersionsError>>;

    /// <p>List the schemas.</p>
    async fn list_schemas(
        &self,
        input: ListSchemasRequest,
    ) -> Result<ListSchemasResponse, RusotoError<ListSchemasError>>;

    /// <p>Get tags for resource.</p>
    async fn list_tags_for_resource(
        &self,
        input: ListTagsForResourceRequest,
    ) -> Result<ListTagsForResourceResponse, RusotoError<ListTagsForResourceError>>;

    /// <p>Put code binding URI</p>
    async fn put_code_binding(
        &self,
        input: PutCodeBindingRequest,
    ) -> Result<PutCodeBindingResponse, RusotoError<PutCodeBindingError>>;

    /// <p>The name of the policy.</p>
    async fn put_resource_policy(
        &self,
        input: PutResourcePolicyRequest,
    ) -> Result<PutResourcePolicyResponse, RusotoError<PutResourcePolicyError>>;

    /// <p>Search the schemas</p>
    async fn search_schemas(
        &self,
        input: SearchSchemasRequest,
    ) -> Result<SearchSchemasResponse, RusotoError<SearchSchemasError>>;

    /// <p>Starts the discoverer</p>
    async fn start_discoverer(
        &self,
        input: StartDiscovererRequest,
    ) -> Result<StartDiscovererResponse, RusotoError<StartDiscovererError>>;

    /// <p>Stops the discoverer</p>
    async fn stop_discoverer(
        &self,
        input: StopDiscovererRequest,
    ) -> Result<StopDiscovererResponse, RusotoError<StopDiscovererError>>;

    /// <p>Add tags to a resource.</p>
    async fn tag_resource(
        &self,
        input: TagResourceRequest,
    ) -> Result<(), RusotoError<TagResourceError>>;

    /// <p>Removes tags from a resource.</p>
    async fn untag_resource(
        &self,
        input: UntagResourceRequest,
    ) -> Result<(), RusotoError<UntagResourceError>>;

    /// <p>Updates the discoverer</p>
    async fn update_discoverer(
        &self,
        input: UpdateDiscovererRequest,
    ) -> Result<UpdateDiscovererResponse, RusotoError<UpdateDiscovererError>>;

    /// <p>Updates a registry.</p>
    async fn update_registry(
        &self,
        input: UpdateRegistryRequest,
    ) -> Result<UpdateRegistryResponse, RusotoError<UpdateRegistryError>>;

    /// <p><p>Updates the schema definition</p> <note><p>Inactive schemas will be deleted after two years.</p></note></p>
    async fn update_schema(
        &self,
        input: UpdateSchemaRequest,
    ) -> Result<UpdateSchemaResponse, RusotoError<UpdateSchemaError>>;
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

#[async_trait]
impl Schemas for SchemasClient {
    /// <p>Creates a discoverer.</p>
    async fn create_discoverer(
        &self,
        input: CreateDiscovererRequest,
    ) -> Result<CreateDiscovererResponse, RusotoError<CreateDiscovererError>> {
        let request_uri = "/v1/discoverers";

        let mut request = SignedRequest::new("POST", "schemas", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
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

    /// <p>Creates a registry.</p>
    async fn create_registry(
        &self,
        input: CreateRegistryRequest,
    ) -> Result<CreateRegistryResponse, RusotoError<CreateRegistryError>> {
        let request_uri = format!(
            "/v1/registries/name/{registry_name}",
            registry_name = input.registry_name
        );

        let mut request = SignedRequest::new("POST", "schemas", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
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

    /// <p><p>Creates a schema definition.</p> <note><p>Inactive schemas will be deleted after two years.</p></note></p>
    async fn create_schema(
        &self,
        input: CreateSchemaRequest,
    ) -> Result<CreateSchemaResponse, RusotoError<CreateSchemaError>> {
        let request_uri = format!(
            "/v1/registries/name/{registry_name}/schemas/name/{schema_name}",
            registry_name = input.registry_name,
            schema_name = input.schema_name
        );

        let mut request = SignedRequest::new("POST", "schemas", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
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

    /// <p>Deletes a discoverer.</p>
    async fn delete_discoverer(
        &self,
        input: DeleteDiscovererRequest,
    ) -> Result<(), RusotoError<DeleteDiscovererError>> {
        let request_uri = format!(
            "/v1/discoverers/id/{discoverer_id}",
            discoverer_id = input.discoverer_id
        );

        let mut request = SignedRequest::new("DELETE", "schemas", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 204 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = ::std::mem::drop(response);

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(DeleteDiscovererError::from_response(response))
        }
    }

    /// <p>Deletes a Registry.</p>
    async fn delete_registry(
        &self,
        input: DeleteRegistryRequest,
    ) -> Result<(), RusotoError<DeleteRegistryError>> {
        let request_uri = format!(
            "/v1/registries/name/{registry_name}",
            registry_name = input.registry_name
        );

        let mut request = SignedRequest::new("DELETE", "schemas", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 204 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = ::std::mem::drop(response);

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(DeleteRegistryError::from_response(response))
        }
    }

    /// <p>Delete the resource-based policy attached to the specified registry.</p>
    async fn delete_resource_policy(
        &self,
        input: DeleteResourcePolicyRequest,
    ) -> Result<(), RusotoError<DeleteResourcePolicyError>> {
        let request_uri = "/v1/policy";

        let mut request = SignedRequest::new("DELETE", "schemas", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut params = Params::new();
        if let Some(ref x) = input.registry_name {
            params.put("registryName", x);
        }
        request.set_params(params);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 204 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = ::std::mem::drop(response);

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(DeleteResourcePolicyError::from_response(response))
        }
    }

    /// <p>Delete a schema definition.</p>
    async fn delete_schema(
        &self,
        input: DeleteSchemaRequest,
    ) -> Result<(), RusotoError<DeleteSchemaError>> {
        let request_uri = format!(
            "/v1/registries/name/{registry_name}/schemas/name/{schema_name}",
            registry_name = input.registry_name,
            schema_name = input.schema_name
        );

        let mut request = SignedRequest::new("DELETE", "schemas", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 204 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = ::std::mem::drop(response);

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(DeleteSchemaError::from_response(response))
        }
    }

    /// <p>Delete the schema version definition</p>
    async fn delete_schema_version(
        &self,
        input: DeleteSchemaVersionRequest,
    ) -> Result<(), RusotoError<DeleteSchemaVersionError>> {
        let request_uri = format!("/v1/registries/name/{registry_name}/schemas/name/{schema_name}/version/{schema_version}", registry_name = input.registry_name, schema_name = input.schema_name, schema_version = input.schema_version);

        let mut request = SignedRequest::new("DELETE", "schemas", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 204 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = ::std::mem::drop(response);

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(DeleteSchemaVersionError::from_response(response))
        }
    }

    /// <p>Describe the code binding URI.</p>
    async fn describe_code_binding(
        &self,
        input: DescribeCodeBindingRequest,
    ) -> Result<DescribeCodeBindingResponse, RusotoError<DescribeCodeBindingError>> {
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

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
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

    /// <p>Describes the discoverer.</p>
    async fn describe_discoverer(
        &self,
        input: DescribeDiscovererRequest,
    ) -> Result<DescribeDiscovererResponse, RusotoError<DescribeDiscovererError>> {
        let request_uri = format!(
            "/v1/discoverers/id/{discoverer_id}",
            discoverer_id = input.discoverer_id
        );

        let mut request = SignedRequest::new("GET", "schemas", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
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

    /// <p>Describes the registry.</p>
    async fn describe_registry(
        &self,
        input: DescribeRegistryRequest,
    ) -> Result<DescribeRegistryResponse, RusotoError<DescribeRegistryError>> {
        let request_uri = format!(
            "/v1/registries/name/{registry_name}",
            registry_name = input.registry_name
        );

        let mut request = SignedRequest::new("GET", "schemas", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
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

    /// <p>Retrieve the schema definition.</p>
    async fn describe_schema(
        &self,
        input: DescribeSchemaRequest,
    ) -> Result<DescribeSchemaResponse, RusotoError<DescribeSchemaError>> {
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

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
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

    /// <p>Get the code binding source URI.</p>
    async fn get_code_binding_source(
        &self,
        input: GetCodeBindingSourceRequest,
    ) -> Result<GetCodeBindingSourceResponse, RusotoError<GetCodeBindingSourceError>> {
        let request_uri = format!("/v1/registries/name/{registry_name}/schemas/name/{schema_name}/language/{language}/source", language = input.language, registry_name = input.registry_name, schema_name = input.schema_name);

        let mut request = SignedRequest::new("GET", "schemas", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut params = Params::new();
        if let Some(ref x) = input.schema_version {
            params.put("schemaVersion", x);
        }
        request.set_params(params);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
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

    /// <p>Get the discovered schema that was generated based on sampled events.</p>
    async fn get_discovered_schema(
        &self,
        input: GetDiscoveredSchemaRequest,
    ) -> Result<GetDiscoveredSchemaResponse, RusotoError<GetDiscoveredSchemaError>> {
        let request_uri = "/v1/discover";

        let mut request = SignedRequest::new("POST", "schemas", &self.region, &request_uri);
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
                .deserialize::<GetDiscoveredSchemaResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(GetDiscoveredSchemaError::from_response(response))
        }
    }

    /// <p>Retrieves the resource-based policy attached to a given registry.</p>
    async fn get_resource_policy(
        &self,
        input: GetResourcePolicyRequest,
    ) -> Result<GetResourcePolicyResponse, RusotoError<GetResourcePolicyError>> {
        let request_uri = "/v1/policy";

        let mut request = SignedRequest::new("GET", "schemas", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut params = Params::new();
        if let Some(ref x) = input.registry_name {
            params.put("registryName", x);
        }
        request.set_params(params);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<GetResourcePolicyResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(GetResourcePolicyError::from_response(response))
        }
    }

    /// <p>List the discoverers.</p>
    async fn list_discoverers(
        &self,
        input: ListDiscoverersRequest,
    ) -> Result<ListDiscoverersResponse, RusotoError<ListDiscoverersError>> {
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

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
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

    /// <p>List the registries.</p>
    async fn list_registries(
        &self,
        input: ListRegistriesRequest,
    ) -> Result<ListRegistriesResponse, RusotoError<ListRegistriesError>> {
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

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
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

    /// <p>Provides a list of the schema versions and related information.</p>
    async fn list_schema_versions(
        &self,
        input: ListSchemaVersionsRequest,
    ) -> Result<ListSchemaVersionsResponse, RusotoError<ListSchemaVersionsError>> {
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

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
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

    /// <p>List the schemas.</p>
    async fn list_schemas(
        &self,
        input: ListSchemasRequest,
    ) -> Result<ListSchemasResponse, RusotoError<ListSchemasError>> {
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

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
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

    /// <p>Get tags for resource.</p>
    async fn list_tags_for_resource(
        &self,
        input: ListTagsForResourceRequest,
    ) -> Result<ListTagsForResourceResponse, RusotoError<ListTagsForResourceError>> {
        let request_uri = format!("/tags/{resource_arn}", resource_arn = input.resource_arn);

        let mut request = SignedRequest::new("GET", "schemas", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
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

    /// <p>Put code binding URI</p>
    async fn put_code_binding(
        &self,
        input: PutCodeBindingRequest,
    ) -> Result<PutCodeBindingResponse, RusotoError<PutCodeBindingError>> {
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

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
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

    /// <p>The name of the policy.</p>
    async fn put_resource_policy(
        &self,
        input: PutResourcePolicyRequest,
    ) -> Result<PutResourcePolicyResponse, RusotoError<PutResourcePolicyError>> {
        let request_uri = "/v1/policy";

        let mut request = SignedRequest::new("PUT", "schemas", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut params = Params::new();
        if let Some(ref x) = input.registry_name {
            params.put("registryName", x);
        }
        request.set_params(params);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<PutResourcePolicyResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(PutResourcePolicyError::from_response(response))
        }
    }

    /// <p>Search the schemas</p>
    async fn search_schemas(
        &self,
        input: SearchSchemasRequest,
    ) -> Result<SearchSchemasResponse, RusotoError<SearchSchemasError>> {
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

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
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

    /// <p>Starts the discoverer</p>
    async fn start_discoverer(
        &self,
        input: StartDiscovererRequest,
    ) -> Result<StartDiscovererResponse, RusotoError<StartDiscovererError>> {
        let request_uri = format!(
            "/v1/discoverers/id/{discoverer_id}/start",
            discoverer_id = input.discoverer_id
        );

        let mut request = SignedRequest::new("POST", "schemas", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
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

    /// <p>Stops the discoverer</p>
    async fn stop_discoverer(
        &self,
        input: StopDiscovererRequest,
    ) -> Result<StopDiscovererResponse, RusotoError<StopDiscovererError>> {
        let request_uri = format!(
            "/v1/discoverers/id/{discoverer_id}/stop",
            discoverer_id = input.discoverer_id
        );

        let mut request = SignedRequest::new("POST", "schemas", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
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

    /// <p>Add tags to a resource.</p>
    async fn tag_resource(
        &self,
        input: TagResourceRequest,
    ) -> Result<(), RusotoError<TagResourceError>> {
        let request_uri = format!("/tags/{resource_arn}", resource_arn = input.resource_arn);

        let mut request = SignedRequest::new("POST", "schemas", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 204 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = ::std::mem::drop(response);

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(TagResourceError::from_response(response))
        }
    }

    /// <p>Removes tags from a resource.</p>
    async fn untag_resource(
        &self,
        input: UntagResourceRequest,
    ) -> Result<(), RusotoError<UntagResourceError>> {
        let request_uri = format!("/tags/{resource_arn}", resource_arn = input.resource_arn);

        let mut request = SignedRequest::new("DELETE", "schemas", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut params = Params::new();
        for item in input.tag_keys.iter() {
            params.put("tagKeys", item);
        }
        request.set_params(params);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 204 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = ::std::mem::drop(response);

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(UntagResourceError::from_response(response))
        }
    }

    /// <p>Updates the discoverer</p>
    async fn update_discoverer(
        &self,
        input: UpdateDiscovererRequest,
    ) -> Result<UpdateDiscovererResponse, RusotoError<UpdateDiscovererError>> {
        let request_uri = format!(
            "/v1/discoverers/id/{discoverer_id}",
            discoverer_id = input.discoverer_id
        );

        let mut request = SignedRequest::new("PUT", "schemas", &self.region, &request_uri);
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
                .deserialize::<UpdateDiscovererResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(UpdateDiscovererError::from_response(response))
        }
    }

    /// <p>Updates a registry.</p>
    async fn update_registry(
        &self,
        input: UpdateRegistryRequest,
    ) -> Result<UpdateRegistryResponse, RusotoError<UpdateRegistryError>> {
        let request_uri = format!(
            "/v1/registries/name/{registry_name}",
            registry_name = input.registry_name
        );

        let mut request = SignedRequest::new("PUT", "schemas", &self.region, &request_uri);
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
                .deserialize::<UpdateRegistryResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(UpdateRegistryError::from_response(response))
        }
    }

    /// <p><p>Updates the schema definition</p> <note><p>Inactive schemas will be deleted after two years.</p></note></p>
    async fn update_schema(
        &self,
        input: UpdateSchemaRequest,
    ) -> Result<UpdateSchemaResponse, RusotoError<UpdateSchemaError>> {
        let request_uri = format!(
            "/v1/registries/name/{registry_name}/schemas/name/{schema_name}",
            registry_name = input.registry_name,
            schema_name = input.schema_name
        );

        let mut request = SignedRequest::new("PUT", "schemas", &self.region, &request_uri);
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
                .deserialize::<UpdateSchemaResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(UpdateSchemaError::from_response(response))
        }
    }
}
