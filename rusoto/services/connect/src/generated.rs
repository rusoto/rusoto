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
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct AssociateApprovedOriginRequest {
    /// <p>The identifier of the Amazon Connect instance.</p>
    #[serde(rename = "InstanceId")]
    pub instance_id: String,
    /// <p>The domain to add to your allow list.</p>
    #[serde(rename = "Origin")]
    pub origin: String,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct AssociateInstanceStorageConfigRequest {
    /// <p>The identifier of the Amazon Connect instance.</p>
    #[serde(rename = "InstanceId")]
    pub instance_id: String,
    /// <p>A valid resource type.</p>
    #[serde(rename = "ResourceType")]
    pub resource_type: String,
    /// <p>A valid storage type.</p>
    #[serde(rename = "StorageConfig")]
    pub storage_config: InstanceStorageConfig,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct AssociateInstanceStorageConfigResponse {
    /// <p>The existing association identifier that uniquely identifies the resource type and storage config for the given instance ID.</p>
    #[serde(rename = "AssociationId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub association_id: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct AssociateLambdaFunctionRequest {
    /// <p>The Amazon Resource Name (ARN) for the Lambda function being associated. Maximum number of characters allowed is 140.</p>
    #[serde(rename = "FunctionArn")]
    pub function_arn: String,
    /// <p>The identifier of the Amazon Connect instance.</p>
    #[serde(rename = "InstanceId")]
    pub instance_id: String,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct AssociateLexBotRequest {
    /// <p>The identifier of the Amazon Connect instance.</p>
    #[serde(rename = "InstanceId")]
    pub instance_id: String,
    /// <p>The Amazon Lex box to associate with the instance.</p>
    #[serde(rename = "LexBot")]
    pub lex_bot: LexBot,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct AssociateRoutingProfileQueuesRequest {
    /// <p>The identifier of the Amazon Connect instance.</p>
    #[serde(rename = "InstanceId")]
    pub instance_id: String,
    /// <p>The queues to associate with this routing profile.</p>
    #[serde(rename = "QueueConfigs")]
    pub queue_configs: Vec<RoutingProfileQueueConfig>,
    /// <p>The identifier of the routing profile.</p>
    #[serde(rename = "RoutingProfileId")]
    pub routing_profile_id: String,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct AssociateSecurityKeyRequest {
    /// <p>The identifier of the Amazon Connect instance.</p>
    #[serde(rename = "InstanceId")]
    pub instance_id: String,
    /// <p>A valid security key in PEM format.</p>
    #[serde(rename = "Key")]
    pub key: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct AssociateSecurityKeyResponse {
    /// <p>The existing association identifier that uniquely identifies the resource type and storage config for the given instance ID.</p>
    #[serde(rename = "AssociationId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub association_id: Option<String>,
}

/// <p>A toggle for an individual feature at the instance level.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct Attribute {
    /// <p>The type of attribute.</p>
    #[serde(rename = "AttributeType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attribute_type: Option<String>,
    /// <p>The value of the attribute.</p>
    #[serde(rename = "Value")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

/// <p>A chat message.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ChatMessage {
    /// <p>The content of the chat message.</p>
    #[serde(rename = "Content")]
    pub content: String,
    /// <p>The type of the content. Supported types are text/plain.</p>
    #[serde(rename = "ContentType")]
    pub content_type: String,
}

/// <p>Contains information about a contact flow.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ContactFlow {
    /// <p>The Amazon Resource Name (ARN) of the contact flow.</p>
    #[serde(rename = "Arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p>The content of the contact flow.</p>
    #[serde(rename = "Content")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
    /// <p>The description of the contact flow.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The identifier of the contact flow.</p>
    #[serde(rename = "Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// <p>The name of the contact flow.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>One or more tags.</p>
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<::std::collections::HashMap<String, String>>,
    /// <p>The type of the contact flow. For descriptions of the available types, see <a href="https://docs.aws.amazon.com/connect/latest/adminguide/create-contact-flow.html#contact-flow-types">Choose a Contact Flow Type</a> in the <i>Amazon Connect Administrator Guide</i>.</p>
    #[serde(rename = "Type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}

/// <p>Contains summary information about a contact flow.</p> <p>You can also create and update contact flows using the <a href="https://docs.aws.amazon.com/connect/latest/adminguide/flow-language.html">Amazon Connect Flow language</a>.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ContactFlowSummary {
    /// <p>The Amazon Resource Name (ARN) of the contact flow.</p>
    #[serde(rename = "Arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p>The type of contact flow.</p>
    #[serde(rename = "ContactFlowType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contact_flow_type: Option<String>,
    /// <p>The identifier of the contact flow.</p>
    #[serde(rename = "Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// <p>The name of the contact flow.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateContactFlowRequest {
    /// <p>The content of the contact flow. </p>
    #[serde(rename = "Content")]
    pub content: String,
    /// <p>The description of the contact flow. </p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The identifier of the Amazon Connect instance.</p>
    #[serde(rename = "InstanceId")]
    pub instance_id: String,
    /// <p>The name of the contact flow.</p>
    #[serde(rename = "Name")]
    pub name: String,
    /// <p>One or more tags.</p>
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<::std::collections::HashMap<String, String>>,
    /// <p>The type of the contact flow. For descriptions of the available types, see <a href="https://docs.aws.amazon.com/connect/latest/adminguide/create-contact-flow.html#contact-flow-types">Choose a Contact Flow Type</a> in the <i>Amazon Connect Administrator Guide</i>.</p>
    #[serde(rename = "Type")]
    pub type_: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateContactFlowResponse {
    /// <p>The Amazon Resource Name (ARN) of the contact flow.</p>
    #[serde(rename = "ContactFlowArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contact_flow_arn: Option<String>,
    /// <p>The identifier of the contact flow.</p>
    #[serde(rename = "ContactFlowId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contact_flow_id: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateInstanceRequest {
    /// <p>The idempotency token.</p>
    #[serde(rename = "ClientToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
    /// <p>The identifier for the directory.</p>
    #[serde(rename = "DirectoryId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub directory_id: Option<String>,
    /// <p>The type of identity management for your Amazon Connect users.</p>
    #[serde(rename = "IdentityManagementType")]
    pub identity_management_type: String,
    /// <p>Whether your contact center handles incoming contacts.</p>
    #[serde(rename = "InboundCallsEnabled")]
    pub inbound_calls_enabled: bool,
    /// <p>The name for your instance.</p>
    #[serde(rename = "InstanceAlias")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_alias: Option<String>,
    /// <p>Whether your contact center allows outbound calls.</p>
    #[serde(rename = "OutboundCallsEnabled")]
    pub outbound_calls_enabled: bool,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateInstanceResponse {
    /// <p>The Amazon Resource Name (ARN) of the instance.</p>
    #[serde(rename = "Arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p>The identifier for the instance.</p>
    #[serde(rename = "Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateIntegrationAssociationRequest {
    /// <p>The identifier of the Amazon Connect instance.</p>
    #[serde(rename = "InstanceId")]
    pub instance_id: String,
    /// <p>The Amazon Resource Name (ARN) of the integration.</p>
    #[serde(rename = "IntegrationArn")]
    pub integration_arn: String,
    /// <p>The type of information to be ingested.</p>
    #[serde(rename = "IntegrationType")]
    pub integration_type: String,
    /// <p>The name of the external application.</p>
    #[serde(rename = "SourceApplicationName")]
    pub source_application_name: String,
    /// <p>The URL for the external application.</p>
    #[serde(rename = "SourceApplicationUrl")]
    pub source_application_url: String,
    /// <p>The type of the data source.</p>
    #[serde(rename = "SourceType")]
    pub source_type: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateIntegrationAssociationResponse {
    /// <p>The Amazon Resource Name (ARN) for the association.</p>
    #[serde(rename = "IntegrationAssociationArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub integration_association_arn: Option<String>,
    /// <p>The identifier for the association.</p>
    #[serde(rename = "IntegrationAssociationId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub integration_association_id: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateQuickConnectRequest {
    /// <p>The description of the quick connect.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The identifier of the Amazon Connect instance.</p>
    #[serde(rename = "InstanceId")]
    pub instance_id: String,
    /// <p>The name of the quick connect.</p>
    #[serde(rename = "Name")]
    pub name: String,
    /// <p>Configuration settings for the quick connect.</p>
    #[serde(rename = "QuickConnectConfig")]
    pub quick_connect_config: QuickConnectConfig,
    /// <p>One or more tags.</p>
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<::std::collections::HashMap<String, String>>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateQuickConnectResponse {
    /// <p>The Amazon Resource Name (ARN) for the quick connect. </p>
    #[serde(rename = "QuickConnectARN")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quick_connect_arn: Option<String>,
    /// <p>The identifier for the quick connect.</p>
    #[serde(rename = "QuickConnectId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quick_connect_id: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateRoutingProfileRequest {
    /// <p>The default outbound queue for the routing profile.</p>
    #[serde(rename = "DefaultOutboundQueueId")]
    pub default_outbound_queue_id: String,
    /// <p>Description of the routing profile. Must not be more than 250 characters.</p>
    #[serde(rename = "Description")]
    pub description: String,
    /// <p>The identifier of the Amazon Connect instance.</p>
    #[serde(rename = "InstanceId")]
    pub instance_id: String,
    /// <p>The channels agents can handle in the Contact Control Panel (CCP) for this routing profile.</p>
    #[serde(rename = "MediaConcurrencies")]
    pub media_concurrencies: Vec<MediaConcurrency>,
    /// <p>The name of the routing profile. Must not be more than 127 characters.</p>
    #[serde(rename = "Name")]
    pub name: String,
    /// <p>The inbound queues associated with the routing profile. If no queue is added, the agent can only make outbound calls.</p>
    #[serde(rename = "QueueConfigs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub queue_configs: Option<Vec<RoutingProfileQueueConfig>>,
    /// <p>One or more tags.</p>
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<::std::collections::HashMap<String, String>>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateRoutingProfileResponse {
    /// <p>The Amazon Resource Name (ARN) of the routing profile.</p>
    #[serde(rename = "RoutingProfileArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub routing_profile_arn: Option<String>,
    /// <p>The identifier of the routing profile.</p>
    #[serde(rename = "RoutingProfileId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub routing_profile_id: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateUseCaseRequest {
    /// <p>The identifier of the Amazon Connect instance.</p>
    #[serde(rename = "InstanceId")]
    pub instance_id: String,
    /// <p>The identifier for the AppIntegration association.</p>
    #[serde(rename = "IntegrationAssociationId")]
    pub integration_association_id: String,
    /// <p>The type of use case to associate to the AppIntegration association. Each AppIntegration association can have only one of each use case type.</p>
    #[serde(rename = "UseCaseType")]
    pub use_case_type: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateUseCaseResponse {
    /// <p>The Amazon Resource Name (ARN) for the use case.</p>
    #[serde(rename = "UseCaseArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_case_arn: Option<String>,
    /// <p>The identifier of the use case.</p>
    #[serde(rename = "UseCaseId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_case_id: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateUserHierarchyGroupRequest {
    /// <p>The identifier of the Amazon Connect instance.</p>
    #[serde(rename = "InstanceId")]
    pub instance_id: String,
    /// <p>The name of the user hierarchy group. Must not be more than 100 characters.</p>
    #[serde(rename = "Name")]
    pub name: String,
    /// <p>The identifier for the parent hierarchy group. The user hierarchy is created at level one if the parent group ID is null.</p>
    #[serde(rename = "ParentGroupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_group_id: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateUserHierarchyGroupResponse {
    /// <p>The Amazon Resource Name (ARN) of the hierarchy group. </p>
    #[serde(rename = "HierarchyGroupArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hierarchy_group_arn: Option<String>,
    /// <p>The identifier of the hierarchy group.</p>
    #[serde(rename = "HierarchyGroupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hierarchy_group_id: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateUserRequest {
    /// <p>The identifier of the user account in the directory used for identity management. If Amazon Connect cannot access the directory, you can specify this identifier to authenticate users. If you include the identifier, we assume that Amazon Connect cannot access the directory. Otherwise, the identity information is used to authenticate users from your directory.</p> <p>This parameter is required if you are using an existing directory for identity management in Amazon Connect when Amazon Connect cannot access your directory to authenticate users. If you are using SAML for identity management and include this parameter, an error is returned.</p>
    #[serde(rename = "DirectoryUserId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub directory_user_id: Option<String>,
    /// <p>The identifier of the hierarchy group for the user.</p>
    #[serde(rename = "HierarchyGroupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hierarchy_group_id: Option<String>,
    /// <p>The information about the identity of the user.</p>
    #[serde(rename = "IdentityInfo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identity_info: Option<UserIdentityInfo>,
    /// <p>The identifier of the Amazon Connect instance.</p>
    #[serde(rename = "InstanceId")]
    pub instance_id: String,
    /// <p>The password for the user account. A password is required if you are using Amazon Connect for identity management. Otherwise, it is an error to include a password.</p>
    #[serde(rename = "Password")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,
    /// <p>The phone settings for the user.</p>
    #[serde(rename = "PhoneConfig")]
    pub phone_config: UserPhoneConfig,
    /// <p>The identifier of the routing profile for the user.</p>
    #[serde(rename = "RoutingProfileId")]
    pub routing_profile_id: String,
    /// <p>The identifier of the security profile for the user.</p>
    #[serde(rename = "SecurityProfileIds")]
    pub security_profile_ids: Vec<String>,
    /// <p>One or more tags.</p>
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<::std::collections::HashMap<String, String>>,
    /// <p>The user name for the account. For instances not using SAML for identity management, the user name can include up to 20 characters. If you are using SAML for identity management, the user name can include up to 64 characters from [a-zA-Z0-9_-.\@]+.</p>
    #[serde(rename = "Username")]
    pub username: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateUserResponse {
    /// <p>The Amazon Resource Name (ARN) of the user account.</p>
    #[serde(rename = "UserArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_arn: Option<String>,
    /// <p>The identifier of the user account.</p>
    #[serde(rename = "UserId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
}

/// <p>Contains credentials to use for federation.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct Credentials {
    /// <p>An access token generated for a federated user to access Amazon Connect.</p>
    #[serde(rename = "AccessToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_token: Option<String>,
    /// <p>A token generated with an expiration time for the session a user is logged in to Amazon Connect.</p>
    #[serde(rename = "AccessTokenExpiration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_token_expiration: Option<f64>,
    /// <p>Renews a token generated for a user to access the Amazon Connect instance.</p>
    #[serde(rename = "RefreshToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refresh_token: Option<String>,
    /// <p>Renews the expiration timer for a generated token.</p>
    #[serde(rename = "RefreshTokenExpiration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refresh_token_expiration: Option<f64>,
}

/// <p>Contains information about a real-time metric. For a description of each metric, see <a href="https://docs.aws.amazon.com/connect/latest/adminguide/real-time-metrics-definitions.html">Real-time Metrics Definitions</a> in the <i>Amazon Connect Administrator Guide</i>.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct CurrentMetric {
    /// <p>The name of the metric.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The unit for the metric.</p>
    #[serde(rename = "Unit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit: Option<String>,
}

/// <p>Contains the data for a real-time metric.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CurrentMetricData {
    /// <p>Information about the metric.</p>
    #[serde(rename = "Metric")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metric: Option<CurrentMetric>,
    /// <p>The value of the metric.</p>
    #[serde(rename = "Value")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<f64>,
}

/// <p>Contains information about a set of real-time metrics.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CurrentMetricResult {
    /// <p>The set of metrics.</p>
    #[serde(rename = "Collections")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub collections: Option<Vec<CurrentMetricData>>,
    /// <p>The dimensions for the metrics.</p>
    #[serde(rename = "Dimensions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dimensions: Option<Dimensions>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteInstanceRequest {
    /// <p>The identifier of the Amazon Connect instance.</p>
    #[serde(rename = "InstanceId")]
    pub instance_id: String,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteIntegrationAssociationRequest {
    /// <p>The identifier of the Amazon Connect instance.</p>
    #[serde(rename = "InstanceId")]
    pub instance_id: String,
    /// <p>The identifier for the AppIntegration association.</p>
    #[serde(rename = "IntegrationAssociationId")]
    pub integration_association_id: String,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteQuickConnectRequest {
    /// <p>The identifier of the Amazon Connect instance.</p>
    #[serde(rename = "InstanceId")]
    pub instance_id: String,
    /// <p>The identifier for the quick connect.</p>
    #[serde(rename = "QuickConnectId")]
    pub quick_connect_id: String,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteUseCaseRequest {
    /// <p>The identifier of the Amazon Connect instance.</p>
    #[serde(rename = "InstanceId")]
    pub instance_id: String,
    /// <p>The identifier for the AppIntegration association.</p>
    #[serde(rename = "IntegrationAssociationId")]
    pub integration_association_id: String,
    /// <p>The identifier for the use case.</p>
    #[serde(rename = "UseCaseId")]
    pub use_case_id: String,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteUserHierarchyGroupRequest {
    /// <p>The identifier of the hierarchy group.</p>
    #[serde(rename = "HierarchyGroupId")]
    pub hierarchy_group_id: String,
    /// <p>The identifier of the Amazon Connect instance.</p>
    #[serde(rename = "InstanceId")]
    pub instance_id: String,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteUserRequest {
    /// <p>The identifier of the Amazon Connect instance.</p>
    #[serde(rename = "InstanceId")]
    pub instance_id: String,
    /// <p>The identifier of the user.</p>
    #[serde(rename = "UserId")]
    pub user_id: String,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeContactFlowRequest {
    /// <p>The identifier of the contact flow.</p>
    #[serde(rename = "ContactFlowId")]
    pub contact_flow_id: String,
    /// <p>The identifier of the Amazon Connect instance.</p>
    #[serde(rename = "InstanceId")]
    pub instance_id: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeContactFlowResponse {
    /// <p>Information about the contact flow.</p>
    #[serde(rename = "ContactFlow")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contact_flow: Option<ContactFlow>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeInstanceAttributeRequest {
    /// <p>The type of attribute.</p>
    #[serde(rename = "AttributeType")]
    pub attribute_type: String,
    /// <p>The identifier of the Amazon Connect instance.</p>
    #[serde(rename = "InstanceId")]
    pub instance_id: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeInstanceAttributeResponse {
    /// <p>The type of attribute.</p>
    #[serde(rename = "Attribute")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attribute: Option<Attribute>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeInstanceRequest {
    /// <p>The identifier of the Amazon Connect instance.</p>
    #[serde(rename = "InstanceId")]
    pub instance_id: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeInstanceResponse {
    /// <p>The name of the instance.</p>
    #[serde(rename = "Instance")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance: Option<Instance>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeInstanceStorageConfigRequest {
    /// <p>The existing association identifier that uniquely identifies the resource type and storage config for the given instance ID.</p>
    #[serde(rename = "AssociationId")]
    pub association_id: String,
    /// <p>The identifier of the Amazon Connect instance.</p>
    #[serde(rename = "InstanceId")]
    pub instance_id: String,
    /// <p>A valid resource type.</p>
    #[serde(rename = "ResourceType")]
    pub resource_type: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeInstanceStorageConfigResponse {
    /// <p>A valid storage type.</p>
    #[serde(rename = "StorageConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage_config: Option<InstanceStorageConfig>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeQuickConnectRequest {
    /// <p>The identifier of the Amazon Connect instance.</p>
    #[serde(rename = "InstanceId")]
    pub instance_id: String,
    /// <p>The identifier for the quick connect.</p>
    #[serde(rename = "QuickConnectId")]
    pub quick_connect_id: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeQuickConnectResponse {
    /// <p>Information about the quick connect.</p>
    #[serde(rename = "QuickConnect")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quick_connect: Option<QuickConnect>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeRoutingProfileRequest {
    /// <p>The identifier of the Amazon Connect instance.</p>
    #[serde(rename = "InstanceId")]
    pub instance_id: String,
    /// <p>The identifier of the routing profile.</p>
    #[serde(rename = "RoutingProfileId")]
    pub routing_profile_id: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeRoutingProfileResponse {
    /// <p>The routing profile.</p>
    #[serde(rename = "RoutingProfile")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub routing_profile: Option<RoutingProfile>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeUserHierarchyGroupRequest {
    /// <p>The identifier of the hierarchy group.</p>
    #[serde(rename = "HierarchyGroupId")]
    pub hierarchy_group_id: String,
    /// <p>The identifier of the Amazon Connect instance.</p>
    #[serde(rename = "InstanceId")]
    pub instance_id: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeUserHierarchyGroupResponse {
    /// <p>Information about the hierarchy group.</p>
    #[serde(rename = "HierarchyGroup")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hierarchy_group: Option<HierarchyGroup>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeUserHierarchyStructureRequest {
    /// <p>The identifier of the Amazon Connect instance.</p>
    #[serde(rename = "InstanceId")]
    pub instance_id: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeUserHierarchyStructureResponse {
    /// <p>Information about the hierarchy structure.</p>
    #[serde(rename = "HierarchyStructure")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hierarchy_structure: Option<HierarchyStructure>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeUserRequest {
    /// <p>The identifier of the Amazon Connect instance.</p>
    #[serde(rename = "InstanceId")]
    pub instance_id: String,
    /// <p>The identifier of the user account.</p>
    #[serde(rename = "UserId")]
    pub user_id: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeUserResponse {
    /// <p>Information about the user account and configuration settings.</p>
    #[serde(rename = "User")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<User>,
}

/// <p>Contains information about the dimensions for a set of metrics.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct Dimensions {
    /// <p>The channel used for grouping and filters.</p>
    #[serde(rename = "Channel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel: Option<String>,
    /// <p>Information about the queue for which metrics are returned.</p>
    #[serde(rename = "Queue")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub queue: Option<QueueReference>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DisassociateApprovedOriginRequest {
    /// <p>The identifier of the Amazon Connect instance.</p>
    #[serde(rename = "InstanceId")]
    pub instance_id: String,
    /// <p>The domain URL of the integrated application.</p>
    #[serde(rename = "Origin")]
    pub origin: String,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DisassociateInstanceStorageConfigRequest {
    /// <p>The existing association identifier that uniquely identifies the resource type and storage config for the given instance ID.</p>
    #[serde(rename = "AssociationId")]
    pub association_id: String,
    /// <p>The identifier of the Amazon Connect instance.</p>
    #[serde(rename = "InstanceId")]
    pub instance_id: String,
    /// <p>A valid resource type.</p>
    #[serde(rename = "ResourceType")]
    pub resource_type: String,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DisassociateLambdaFunctionRequest {
    /// <p>The Amazon Resource Name (ARN) of the Lambda function being disassociated.</p>
    #[serde(rename = "FunctionArn")]
    pub function_arn: String,
    /// <p>The identifier of the Amazon Connect instance..</p>
    #[serde(rename = "InstanceId")]
    pub instance_id: String,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DisassociateLexBotRequest {
    /// <p>The name of the Amazon Lex bot. Maximum character limit of 50.</p>
    #[serde(rename = "BotName")]
    pub bot_name: String,
    /// <p>The identifier of the Amazon Connect instance.</p>
    #[serde(rename = "InstanceId")]
    pub instance_id: String,
    /// <p>The Region in which the Amazon Lex bot has been created.</p>
    #[serde(rename = "LexRegion")]
    pub lex_region: String,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DisassociateRoutingProfileQueuesRequest {
    /// <p>The identifier of the Amazon Connect instance.</p>
    #[serde(rename = "InstanceId")]
    pub instance_id: String,
    /// <p>The queues to disassociate from this routing profile.</p>
    #[serde(rename = "QueueReferences")]
    pub queue_references: Vec<RoutingProfileQueueReference>,
    /// <p>The identifier of the routing profile.</p>
    #[serde(rename = "RoutingProfileId")]
    pub routing_profile_id: String,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DisassociateSecurityKeyRequest {
    /// <p>The existing association identifier that uniquely identifies the resource type and storage config for the given instance ID.</p>
    #[serde(rename = "AssociationId")]
    pub association_id: String,
    /// <p>The identifier of the Amazon Connect instance.</p>
    #[serde(rename = "InstanceId")]
    pub instance_id: String,
}

/// <p>The encryption configuration.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct EncryptionConfig {
    /// <p>The type of encryption.</p>
    #[serde(rename = "EncryptionType")]
    pub encryption_type: String,
    /// <p>The identifier of the encryption key.</p>
    #[serde(rename = "KeyId")]
    pub key_id: String,
}

/// <p>Contains the filter to apply when retrieving metrics.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct Filters {
    /// <p>The channel to use to filter the metrics.</p>
    #[serde(rename = "Channels")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channels: Option<Vec<String>>,
    /// <p>The queues to use to filter the metrics. You can specify up to 100 queues per request.</p>
    #[serde(rename = "Queues")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub queues: Option<Vec<String>>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetContactAttributesRequest {
    /// <p>The identifier of the initial contact.</p>
    #[serde(rename = "InitialContactId")]
    pub initial_contact_id: String,
    /// <p>The identifier of the Amazon Connect instance.</p>
    #[serde(rename = "InstanceId")]
    pub instance_id: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetContactAttributesResponse {
    /// <p>Information about the attributes.</p>
    #[serde(rename = "Attributes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attributes: Option<::std::collections::HashMap<String, String>>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetCurrentMetricDataRequest {
    /// <p><p>The metrics to retrieve. Specify the name and unit for each metric. The following metrics are available. For a description of all the metrics, see <a href="https://docs.aws.amazon.com/connect/latest/adminguide/real-time-metrics-definitions.html">Real-time Metrics Definitions</a> in the <i>Amazon Connect Administrator Guide</i>.</p> <dl> <dt>AGENTS<em>AFTER</em>CONTACT<em>WORK</dt> <dd> <p>Unit: COUNT</p> <p>Name in real-time metrics report: <a href="https://docs.aws.amazon.com/connect/latest/adminguide/real-time-metrics-definitions.html#aftercallwork-real-time">ACW</a> </p> </dd> <dt>AGENTS</em>AVAILABLE</dt> <dd> <p>Unit: COUNT</p> <p>Name in real-time metrics report: <a href="https://docs.aws.amazon.com/connect/latest/adminguide/real-time-metrics-definitions.html#available-real-time">Available</a> </p> </dd> <dt>AGENTS<em>ERROR</dt> <dd> <p>Unit: COUNT</p> <p>Name in real-time metrics report: <a href="https://docs.aws.amazon.com/connect/latest/adminguide/real-time-metrics-definitions.html#error-real-time">Error</a> </p> </dd> <dt>AGENTS</em>NON<em>PRODUCTIVE</dt> <dd> <p>Unit: COUNT</p> <p>Name in real-time metrics report: <a href="https://docs.aws.amazon.com/connect/latest/adminguide/real-time-metrics-definitions.html#non-productive-time-real-time">NPT (Non-Productive Time)</a> </p> </dd> <dt>AGENTS</em>ON<em>CALL</dt> <dd> <p>Unit: COUNT</p> <p>Name in real-time metrics report: <a href="https://docs.aws.amazon.com/connect/latest/adminguide/real-time-metrics-definitions.html#on-call-real-time">On contact</a> </p> </dd> <dt>AGENTS</em>ON<em>CONTACT</dt> <dd> <p>Unit: COUNT</p> <p>Name in real-time metrics report: <a href="https://docs.aws.amazon.com/connect/latest/adminguide/real-time-metrics-definitions.html#on-call-real-time">On contact</a> </p> </dd> <dt>AGENTS</em>ONLINE</dt> <dd> <p>Unit: COUNT</p> <p>Name in real-time metrics report: <a href="https://docs.aws.amazon.com/connect/latest/adminguide/real-time-metrics-definitions.html#online-real-time">Online</a> </p> </dd> <dt>AGENTS<em>STAFFED</dt> <dd> <p>Unit: COUNT</p> <p>Name in real-time metrics report: <a href="https://docs.aws.amazon.com/connect/latest/adminguide/real-time-metrics-definitions.html#staffed-real-time">Staffed</a> </p> </dd> <dt>CONTACTS</em>IN<em>QUEUE</dt> <dd> <p>Unit: COUNT</p> <p>Name in real-time metrics report: <a href="https://docs.aws.amazon.com/connect/latest/adminguide/real-time-metrics-definitions.html#in-queue-real-time">In queue</a> </p> </dd> <dt>CONTACTS</em>SCHEDULED</dt> <dd> <p>Unit: COUNT</p> <p>Name in real-time metrics report: <a href="https://docs.aws.amazon.com/connect/latest/adminguide/real-time-metrics-definitions.html#scheduled-real-time">Scheduled</a> </p> </dd> <dt>OLDEST<em>CONTACT</em>AGE</dt> <dd> <p>Unit: SECONDS</p> <p>When you use groupings, Unit says SECONDS but the Value is returned in MILLISECONDS. For example, if you get a response like this:</p> <p> <code>{ &quot;Metric&quot;: { &quot;Name&quot;: &quot;OLDEST<em>CONTACT</em>AGE&quot;, &quot;Unit&quot;: &quot;SECONDS&quot; }, &quot;Value&quot;: 24113.0 </code>}</p> <p>The actual OLDEST<em>CONTACT</em>AGE is 24 seconds.</p> <p>Name in real-time metrics report: <a href="https://docs.aws.amazon.com/connect/latest/adminguide/real-time-metrics-definitions.html#oldest-real-time">Oldest</a> </p> </dd> <dt>SLOTS<em>ACTIVE</dt> <dd> <p>Unit: COUNT</p> <p>Name in real-time metrics report: <a href="https://docs.aws.amazon.com/connect/latest/adminguide/real-time-metrics-definitions.html#active-real-time">Active</a> </p> </dd> <dt>SLOTS</em>AVAILABLE</dt> <dd> <p>Unit: COUNT</p> <p>Name in real-time metrics report: <a href="https://docs.aws.amazon.com/connect/latest/adminguide/real-time-metrics-definitions.html#availability-real-time">Availability</a> </p> </dd> </dl></p>
    #[serde(rename = "CurrentMetrics")]
    pub current_metrics: Vec<CurrentMetric>,
    /// <p>The queues, up to 100, or channels, to use to filter the metrics returned. Metric data is retrieved only for the resources associated with the queues or channels included in the filter. You can include both queue IDs and queue ARNs in the same request. VOICE, CHAT, and TASK channels are supported.</p>
    #[serde(rename = "Filters")]
    pub filters: Filters,
    /// <p>The grouping applied to the metrics returned. For example, when grouped by <code>QUEUE</code>, the metrics returned apply to each queue rather than aggregated for all queues. If you group by <code>CHANNEL</code>, you should include a Channels filter. VOICE, CHAT, and TASK channels are supported.</p> <p>If no <code>Grouping</code> is included in the request, a summary of metrics is returned.</p>
    #[serde(rename = "Groupings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub groupings: Option<Vec<String>>,
    /// <p>The identifier of the Amazon Connect instance.</p>
    #[serde(rename = "InstanceId")]
    pub instance_id: String,
    /// <p>The maximimum number of results to return per page.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>The token for the next set of results. Use the value returned in the previous response in the next request to retrieve the next set of results.</p> <p>The token expires after 5 minutes from the time it is created. Subsequent requests that use the token must use the same request parameters as the request that generated the token.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetCurrentMetricDataResponse {
    /// <p>The time at which the metrics were retrieved and cached for pagination.</p>
    #[serde(rename = "DataSnapshotTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_snapshot_time: Option<f64>,
    /// <p>Information about the real-time metrics.</p>
    #[serde(rename = "MetricResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metric_results: Option<Vec<CurrentMetricResult>>,
    /// <p>If there are additional results, this is the token for the next set of results.</p> <p>The token expires after 5 minutes from the time it is created. Subsequent requests that use the token must use the same request parameters as the request that generated the token.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetFederationTokenRequest {
    /// <p>The identifier of the Amazon Connect instance.</p>
    #[serde(rename = "InstanceId")]
    pub instance_id: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetFederationTokenResponse {
    /// <p>The credentials to use for federation.</p>
    #[serde(rename = "Credentials")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub credentials: Option<Credentials>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetMetricDataRequest {
    /// <p>The timestamp, in UNIX Epoch time format, at which to end the reporting interval for the retrieval of historical metrics data. The time must be specified using an interval of 5 minutes, such as 11:00, 11:05, 11:10, and must be later than the start time timestamp.</p> <p>The time range between the start and end time must be less than 24 hours.</p>
    #[serde(rename = "EndTime")]
    pub end_time: f64,
    /// <p>The queues, up to 100, or channels, to use to filter the metrics returned. Metric data is retrieved only for the resources associated with the queues or channels included in the filter. You can include both queue IDs and queue ARNs in the same request. VOICE, CHAT, and TASK channels are supported.</p>
    #[serde(rename = "Filters")]
    pub filters: Filters,
    /// <p>The grouping applied to the metrics returned. For example, when results are grouped by queue, the metrics returned are grouped by queue. The values returned apply to the metrics for each queue rather than aggregated for all queues.</p> <p>The only supported grouping is <code>QUEUE</code>.</p> <p>If no grouping is specified, a summary of metrics for all queues is returned.</p>
    #[serde(rename = "Groupings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub groupings: Option<Vec<String>>,
    /// <p><p>The metrics to retrieve. Specify the name, unit, and statistic for each metric. The following historical metrics are available. For a description of each metric, see <a href="https://docs.aws.amazon.com/connect/latest/adminguide/historical-metrics-definitions.html">Historical Metrics Definitions</a> in the <i>Amazon Connect Administrator Guide</i>.</p> <dl> <dt>ABANDON<em>TIME</dt> <dd> <p>Unit: SECONDS</p> <p>Statistic: AVG</p> </dd> <dt>AFTER</em>CONTACT<em>WORK</em>TIME</dt> <dd> <p>Unit: SECONDS</p> <p>Statistic: AVG</p> </dd> <dt>API<em>CONTACTS</em>HANDLED</dt> <dd> <p>Unit: COUNT</p> <p>Statistic: SUM</p> </dd> <dt>CALLBACK<em>CONTACTS</em>HANDLED</dt> <dd> <p>Unit: COUNT</p> <p>Statistic: SUM</p> </dd> <dt>CONTACTS<em>ABANDONED</dt> <dd> <p>Unit: COUNT</p> <p>Statistic: SUM</p> </dd> <dt>CONTACTS</em>AGENT<em>HUNG</em>UP<em>FIRST</dt> <dd> <p>Unit: COUNT</p> <p>Statistic: SUM</p> </dd> <dt>CONTACTS</em>CONSULTED</dt> <dd> <p>Unit: COUNT</p> <p>Statistic: SUM</p> </dd> <dt>CONTACTS<em>HANDLED</dt> <dd> <p>Unit: COUNT</p> <p>Statistic: SUM</p> </dd> <dt>CONTACTS</em>HANDLED<em>INCOMING</dt> <dd> <p>Unit: COUNT</p> <p>Statistic: SUM</p> </dd> <dt>CONTACTS</em>HANDLED<em>OUTBOUND</dt> <dd> <p>Unit: COUNT</p> <p>Statistic: SUM</p> </dd> <dt>CONTACTS</em>HOLD<em>ABANDONS</dt> <dd> <p>Unit: COUNT</p> <p>Statistic: SUM</p> </dd> <dt>CONTACTS</em>MISSED</dt> <dd> <p>Unit: COUNT</p> <p>Statistic: SUM</p> </dd> <dt>CONTACTS<em>QUEUED</dt> <dd> <p>Unit: COUNT</p> <p>Statistic: SUM</p> </dd> <dt>CONTACTS</em>TRANSFERRED<em>IN</dt> <dd> <p>Unit: COUNT</p> <p>Statistic: SUM</p> </dd> <dt>CONTACTS</em>TRANSFERRED<em>IN</em>FROM<em>QUEUE</dt> <dd> <p>Unit: COUNT</p> <p>Statistic: SUM</p> </dd> <dt>CONTACTS</em>TRANSFERRED<em>OUT</dt> <dd> <p>Unit: COUNT</p> <p>Statistic: SUM</p> </dd> <dt>CONTACTS</em>TRANSFERRED<em>OUT</em>FROM<em>QUEUE</dt> <dd> <p>Unit: COUNT</p> <p>Statistic: SUM</p> </dd> <dt>HANDLE</em>TIME</dt> <dd> <p>Unit: SECONDS</p> <p>Statistic: AVG</p> </dd> <dt>HOLD<em>TIME</dt> <dd> <p>Unit: SECONDS</p> <p>Statistic: AVG</p> </dd> <dt>INTERACTION</em>AND<em>HOLD</em>TIME</dt> <dd> <p>Unit: SECONDS</p> <p>Statistic: AVG</p> </dd> <dt>INTERACTION<em>TIME</dt> <dd> <p>Unit: SECONDS</p> <p>Statistic: AVG</p> </dd> <dt>OCCUPANCY</dt> <dd> <p>Unit: PERCENT</p> <p>Statistic: AVG</p> </dd> <dt>QUEUE</em>ANSWER<em>TIME</dt> <dd> <p>Unit: SECONDS</p> <p>Statistic: AVG</p> </dd> <dt>QUEUED</em>TIME</dt> <dd> <p>Unit: SECONDS</p> <p>Statistic: MAX</p> </dd> <dt>SERVICE_LEVEL</dt> <dd> <p>Unit: PERCENT</p> <p>Statistic: AVG</p> <p>Threshold: Only &quot;Less than&quot; comparisons are supported, with the following service level thresholds: 15, 20, 25, 30, 45, 60, 90, 120, 180, 240, 300, 600</p> </dd> </dl></p>
    #[serde(rename = "HistoricalMetrics")]
    pub historical_metrics: Vec<HistoricalMetric>,
    /// <p>The identifier of the Amazon Connect instance.</p>
    #[serde(rename = "InstanceId")]
    pub instance_id: String,
    /// <p>The maximimum number of results to return per page.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>The token for the next set of results. Use the value returned in the previous response in the next request to retrieve the next set of results.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The timestamp, in UNIX Epoch time format, at which to start the reporting interval for the retrieval of historical metrics data. The time must be specified using a multiple of 5 minutes, such as 10:05, 10:10, 10:15.</p> <p>The start time cannot be earlier than 24 hours before the time of the request. Historical metrics are available only for 24 hours.</p>
    #[serde(rename = "StartTime")]
    pub start_time: f64,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetMetricDataResponse {
    /// <p>Information about the historical metrics.</p> <p>If no grouping is specified, a summary of metric data is returned.</p>
    #[serde(rename = "MetricResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metric_results: Option<Vec<HistoricalMetricResult>>,
    /// <p>If there are additional results, this is the token for the next set of results.</p> <p>The token expires after 5 minutes from the time it is created. Subsequent requests that use the token must use the same request parameters as the request that generated the token.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

/// <p>Contains information about a hierarchy group.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct HierarchyGroup {
    /// <p>The Amazon Resource Name (ARN) of the hierarchy group.</p>
    #[serde(rename = "Arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p>Information about the levels in the hierarchy group.</p>
    #[serde(rename = "HierarchyPath")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hierarchy_path: Option<HierarchyPath>,
    /// <p>The identifier of the hierarchy group.</p>
    #[serde(rename = "Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// <p>The identifier of the level in the hierarchy group.</p>
    #[serde(rename = "LevelId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub level_id: Option<String>,
    /// <p>The name of the hierarchy group.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

/// <p>Contains summary information about a hierarchy group.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct HierarchyGroupSummary {
    /// <p>The Amazon Resource Name (ARN) of the hierarchy group.</p>
    #[serde(rename = "Arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p>The identifier of the hierarchy group.</p>
    #[serde(rename = "Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// <p>The name of the hierarchy group.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

/// <p>Contains information about a hierarchy level.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct HierarchyLevel {
    /// <p>The Amazon Resource Name (ARN) of the hierarchy level.</p>
    #[serde(rename = "Arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p>The identifier of the hierarchy level.</p>
    #[serde(rename = "Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// <p>The name of the hierarchy level.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

/// <p>Contains information about the hierarchy level to update.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct HierarchyLevelUpdate {
    /// <p>The name of the user hierarchy level. Must not be more than 50 characters.</p>
    #[serde(rename = "Name")]
    pub name: String,
}

/// <p>Contains information about the levels of a hierarchy group.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct HierarchyPath {
    /// <p>Information about level five.</p>
    #[serde(rename = "LevelFive")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub level_five: Option<HierarchyGroupSummary>,
    /// <p>Information about level four.</p>
    #[serde(rename = "LevelFour")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub level_four: Option<HierarchyGroupSummary>,
    /// <p>Information about level one.</p>
    #[serde(rename = "LevelOne")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub level_one: Option<HierarchyGroupSummary>,
    /// <p>Information about level three.</p>
    #[serde(rename = "LevelThree")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub level_three: Option<HierarchyGroupSummary>,
    /// <p>Information about level two.</p>
    #[serde(rename = "LevelTwo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub level_two: Option<HierarchyGroupSummary>,
}

/// <p>Contains information about a hierarchy structure.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct HierarchyStructure {
    /// <p>Information about level five.</p>
    #[serde(rename = "LevelFive")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub level_five: Option<HierarchyLevel>,
    /// <p>Information about level four.</p>
    #[serde(rename = "LevelFour")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub level_four: Option<HierarchyLevel>,
    /// <p>Information about level one.</p>
    #[serde(rename = "LevelOne")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub level_one: Option<HierarchyLevel>,
    /// <p>Information about level three.</p>
    #[serde(rename = "LevelThree")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub level_three: Option<HierarchyLevel>,
    /// <p>Information about level two.</p>
    #[serde(rename = "LevelTwo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub level_two: Option<HierarchyLevel>,
}

/// <p>Contains information about the level hierarchy to update.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct HierarchyStructureUpdate {
    /// <p>The update for level five.</p>
    #[serde(rename = "LevelFive")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub level_five: Option<HierarchyLevelUpdate>,
    /// <p>The update for level four.</p>
    #[serde(rename = "LevelFour")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub level_four: Option<HierarchyLevelUpdate>,
    /// <p>The update for level one.</p>
    #[serde(rename = "LevelOne")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub level_one: Option<HierarchyLevelUpdate>,
    /// <p>The update for level three.</p>
    #[serde(rename = "LevelThree")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub level_three: Option<HierarchyLevelUpdate>,
    /// <p>The update for level two.</p>
    #[serde(rename = "LevelTwo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub level_two: Option<HierarchyLevelUpdate>,
}

/// <p>Contains information about a historical metric. For a description of each metric, see <a href="https://docs.aws.amazon.com/connect/latest/adminguide/historical-metrics-definitions.html">Historical Metrics Definitions</a> in the <i>Amazon Connect Administrator Guide</i>.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct HistoricalMetric {
    /// <p>The name of the metric.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The statistic for the metric.</p>
    #[serde(rename = "Statistic")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub statistic: Option<String>,
    /// <p>The threshold for the metric, used with service level metrics.</p>
    #[serde(rename = "Threshold")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub threshold: Option<Threshold>,
    /// <p>The unit for the metric.</p>
    #[serde(rename = "Unit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit: Option<String>,
}

/// <p>Contains the data for a historical metric.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct HistoricalMetricData {
    /// <p>Information about the metric.</p>
    #[serde(rename = "Metric")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metric: Option<HistoricalMetric>,
    /// <p>The value of the metric.</p>
    #[serde(rename = "Value")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<f64>,
}

/// <p>Contains information about the historical metrics retrieved.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct HistoricalMetricResult {
    /// <p>The set of metrics.</p>
    #[serde(rename = "Collections")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub collections: Option<Vec<HistoricalMetricData>>,
    /// <p>The dimension for the metrics.</p>
    #[serde(rename = "Dimensions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dimensions: Option<Dimensions>,
}

/// <p>Contains summary information about hours of operation for a contact center.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct HoursOfOperationSummary {
    /// <p>The Amazon Resource Name (ARN) of the hours of operation.</p>
    #[serde(rename = "Arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p>The identifier of the hours of operation.</p>
    #[serde(rename = "Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// <p>The name of the hours of operation.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

/// <p>The Amazon Connect instance.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct Instance {
    /// <p>The Amazon Resource Name (ARN) of the instance.</p>
    #[serde(rename = "Arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p>When the instance was created.</p>
    #[serde(rename = "CreatedTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_time: Option<f64>,
    /// <p>The identifier of the Amazon Connect instance.</p>
    #[serde(rename = "Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// <p>The identity management type.</p>
    #[serde(rename = "IdentityManagementType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identity_management_type: Option<String>,
    /// <p>Whether inbound calls are enabled.</p>
    #[serde(rename = "InboundCallsEnabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inbound_calls_enabled: Option<bool>,
    /// <p>The alias of instance.</p>
    #[serde(rename = "InstanceAlias")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_alias: Option<String>,
    /// <p>The state of the instance.</p>
    #[serde(rename = "InstanceStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_status: Option<String>,
    /// <p>Whether outbound calls are enabled.</p>
    #[serde(rename = "OutboundCallsEnabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub outbound_calls_enabled: Option<bool>,
    /// <p>The service role of the instance.</p>
    #[serde(rename = "ServiceRole")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_role: Option<String>,
    /// <p>Relevant details why the instance was not successfully created. </p>
    #[serde(rename = "StatusReason")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_reason: Option<InstanceStatusReason>,
}

/// <p>Relevant details why the instance was not successfully created.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct InstanceStatusReason {
    /// <p>The message.</p>
    #[serde(rename = "Message")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

/// <p>The storage configuration for the instance.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct InstanceStorageConfig {
    /// <p>The existing association identifier that uniquely identifies the resource type and storage config for the given instance ID.</p>
    #[serde(rename = "AssociationId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub association_id: Option<String>,
    /// <p>The configuration of the Kinesis Firehose delivery stream.</p>
    #[serde(rename = "KinesisFirehoseConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kinesis_firehose_config: Option<KinesisFirehoseConfig>,
    /// <p>The configuration of the Kinesis data stream.</p>
    #[serde(rename = "KinesisStreamConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kinesis_stream_config: Option<KinesisStreamConfig>,
    /// <p>The configuration of the Kinesis video stream.</p>
    #[serde(rename = "KinesisVideoStreamConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kinesis_video_stream_config: Option<KinesisVideoStreamConfig>,
    /// <p>The S3 configuration.</p>
    #[serde(rename = "S3Config")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_config: Option<S3Config>,
    /// <p>A valid storage type.</p>
    #[serde(rename = "StorageType")]
    pub storage_type: String,
}

/// <p>Information about the instance.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct InstanceSummary {
    /// <p>The Amazon Resource Name (ARN) of the instance.</p>
    #[serde(rename = "Arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p>When the instance was created.</p>
    #[serde(rename = "CreatedTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_time: Option<f64>,
    /// <p>The identifier of the instance.</p>
    #[serde(rename = "Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// <p>The identity management type of the instance.</p>
    #[serde(rename = "IdentityManagementType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identity_management_type: Option<String>,
    /// <p>Whether inbound calls are enabled.</p>
    #[serde(rename = "InboundCallsEnabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inbound_calls_enabled: Option<bool>,
    /// <p>The alias of the instance.</p>
    #[serde(rename = "InstanceAlias")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_alias: Option<String>,
    /// <p>The state of the instance.</p>
    #[serde(rename = "InstanceStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_status: Option<String>,
    /// <p>Whether outbound calls are enabled.</p>
    #[serde(rename = "OutboundCallsEnabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub outbound_calls_enabled: Option<bool>,
    /// <p>The service role of the instance.</p>
    #[serde(rename = "ServiceRole")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_role: Option<String>,
}

/// <p>Contains summary information about the associated AppIntegrations.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct IntegrationAssociationSummary {
    /// <p>The identifier of the Amazon Connect instance.</p>
    #[serde(rename = "InstanceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_id: Option<String>,
    /// <p>The Amazon Resource Name (ARN) for the AppIntegration.</p>
    #[serde(rename = "IntegrationArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub integration_arn: Option<String>,
    /// <p>The Amazon Resource Name (ARN) for the AppIntegration association.</p>
    #[serde(rename = "IntegrationAssociationArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub integration_association_arn: Option<String>,
    /// <p>The identifier for the AppIntegration association.</p>
    #[serde(rename = "IntegrationAssociationId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub integration_association_id: Option<String>,
    /// <p>The integration type.</p>
    #[serde(rename = "IntegrationType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub integration_type: Option<String>,
    /// <p>The user-provided, friendly name for the external application.</p>
    #[serde(rename = "SourceApplicationName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_application_name: Option<String>,
    /// <p>The URL for the external application.</p>
    #[serde(rename = "SourceApplicationUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_application_url: Option<String>,
    /// <p>The name of the source.</p>
    #[serde(rename = "SourceType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_type: Option<String>,
}

/// <p>Configuration information of a Kinesis Firehose delivery stream.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct KinesisFirehoseConfig {
    /// <p>The Amazon Resource Name (ARN) of the delivery stream.</p>
    #[serde(rename = "FirehoseArn")]
    pub firehose_arn: String,
}

/// <p>Configuration information of a Kinesis data stream.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct KinesisStreamConfig {
    /// <p>The Amazon Resource Name (ARN) of the data stream.</p>
    #[serde(rename = "StreamArn")]
    pub stream_arn: String,
}

/// <p>Configuration information of a Kinesis video stream.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct KinesisVideoStreamConfig {
    /// <p>The encryption configuration.</p>
    #[serde(rename = "EncryptionConfig")]
    pub encryption_config: EncryptionConfig,
    /// <p>The prefix of the video stream.</p>
    #[serde(rename = "Prefix")]
    pub prefix: String,
    /// <p>The number of hours data is retained in the stream. Kinesis Video Streams retains the data in a data store that is associated with the stream.</p> <p>The default value is 0, indicating that the stream does not persist data.</p>
    #[serde(rename = "RetentionPeriodHours")]
    pub retention_period_hours: i64,
}

/// <p>Configuration information of an Amazon Lex bot.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct LexBot {
    /// <p>The Region the Amazon Lex bot was created in.</p>
    #[serde(rename = "LexRegion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lex_region: Option<String>,
    /// <p>The name of the Amazon Lex bot.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListApprovedOriginsRequest {
    /// <p>The identifier of the Amazon Connect instance.</p>
    #[serde(rename = "InstanceId")]
    pub instance_id: String,
    /// <p>The maximimum number of results to return per page.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>The token for the next set of results. Use the value returned in the previous response in the next request to retrieve the next set of results.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListApprovedOriginsResponse {
    /// <p>If there are additional results, this is the token for the next set of results.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The approved origins.</p>
    #[serde(rename = "Origins")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub origins: Option<Vec<String>>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListContactFlowsRequest {
    /// <p>The type of contact flow.</p>
    #[serde(rename = "ContactFlowTypes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contact_flow_types: Option<Vec<String>>,
    /// <p>The identifier of the Amazon Connect instance.</p>
    #[serde(rename = "InstanceId")]
    pub instance_id: String,
    /// <p>The maximimum number of results to return per page.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>The token for the next set of results. Use the value returned in the previous response in the next request to retrieve the next set of results.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListContactFlowsResponse {
    /// <p>Information about the contact flows.</p>
    #[serde(rename = "ContactFlowSummaryList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contact_flow_summary_list: Option<Vec<ContactFlowSummary>>,
    /// <p>If there are additional results, this is the token for the next set of results.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListHoursOfOperationsRequest {
    /// <p>The identifier of the Amazon Connect instance.</p>
    #[serde(rename = "InstanceId")]
    pub instance_id: String,
    /// <p>The maximimum number of results to return per page.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>The token for the next set of results. Use the value returned in the previous response in the next request to retrieve the next set of results.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListHoursOfOperationsResponse {
    /// <p>Information about the hours of operation.</p>
    #[serde(rename = "HoursOfOperationSummaryList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hours_of_operation_summary_list: Option<Vec<HoursOfOperationSummary>>,
    /// <p>If there are additional results, this is the token for the next set of results.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListInstanceAttributesRequest {
    /// <p>The identifier of the Amazon Connect instance.</p>
    #[serde(rename = "InstanceId")]
    pub instance_id: String,
    /// <p>The maximimum number of results to return per page.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>The token for the next set of results. Use the value returned in the previous response in the next request to retrieve the next set of results.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListInstanceAttributesResponse {
    /// <p>The attribute types.</p>
    #[serde(rename = "Attributes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attributes: Option<Vec<Attribute>>,
    /// <p>If there are additional results, this is the token for the next set of results.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListInstanceStorageConfigsRequest {
    /// <p>The identifier of the Amazon Connect instance.</p>
    #[serde(rename = "InstanceId")]
    pub instance_id: String,
    /// <p>The maximimum number of results to return per page.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>The token for the next set of results. Use the value returned in the previous response in the next request to retrieve the next set of results.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>A valid resource type.</p>
    #[serde(rename = "ResourceType")]
    pub resource_type: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListInstanceStorageConfigsResponse {
    /// <p>If there are additional results, this is the token for the next set of results.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>A valid storage type.</p>
    #[serde(rename = "StorageConfigs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage_configs: Option<Vec<InstanceStorageConfig>>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListInstancesRequest {
    /// <p>The maximimum number of results to return per page.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>The token for the next set of results. Use the value returned in the previous response in the next request to retrieve the next set of results.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListInstancesResponse {
    /// <p>Information about the instances.</p>
    #[serde(rename = "InstanceSummaryList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_summary_list: Option<Vec<InstanceSummary>>,
    /// <p>If there are additional results, this is the token for the next set of results.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListIntegrationAssociationsRequest {
    /// <p>The identifier of the Amazon Connect instance.</p>
    #[serde(rename = "InstanceId")]
    pub instance_id: String,
    /// <p>The maximimum number of results to return per page.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>The token for the next set of results. Use the value returned in the previous response in the next request to retrieve the next set of results.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListIntegrationAssociationsResponse {
    /// <p>The AppIntegration associations.</p>
    #[serde(rename = "IntegrationAssociationSummaryList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub integration_association_summary_list: Option<Vec<IntegrationAssociationSummary>>,
    /// <p>If there are additional results, this is the token for the next set of results.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListLambdaFunctionsRequest {
    /// <p>The identifier of the Amazon Connect instance.</p>
    #[serde(rename = "InstanceId")]
    pub instance_id: String,
    /// <p>The maximimum number of results to return per page.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>The token for the next set of results. Use the value returned in the previous response in the next request to retrieve the next set of results.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListLambdaFunctionsResponse {
    /// <p>The Lambdafunction ARNs associated with the specified instance.</p>
    #[serde(rename = "LambdaFunctions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lambda_functions: Option<Vec<String>>,
    /// <p>If there are additional results, this is the token for the next set of results.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListLexBotsRequest {
    /// <p>The identifier of the Amazon Connect instance.</p>
    #[serde(rename = "InstanceId")]
    pub instance_id: String,
    /// <p>The maximimum number of results to return per page.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>The token for the next set of results. Use the value returned in the previous response in the next request to retrieve the next set of results.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListLexBotsResponse {
    /// <p>The the names and regions of the Amazon Lex bots associated with the specified instance.</p>
    #[serde(rename = "LexBots")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lex_bots: Option<Vec<LexBot>>,
    /// <p>If there are additional results, this is the token for the next set of results.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListPhoneNumbersRequest {
    /// <p>The identifier of the Amazon Connect instance.</p>
    #[serde(rename = "InstanceId")]
    pub instance_id: String,
    /// <p>The maximimum number of results to return per page.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>The token for the next set of results. Use the value returned in the previous response in the next request to retrieve the next set of results.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The ISO country code.</p>
    #[serde(rename = "PhoneNumberCountryCodes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone_number_country_codes: Option<Vec<String>>,
    /// <p>The type of phone number.</p>
    #[serde(rename = "PhoneNumberTypes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone_number_types: Option<Vec<String>>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListPhoneNumbersResponse {
    /// <p>If there are additional results, this is the token for the next set of results.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>Information about the phone numbers.</p>
    #[serde(rename = "PhoneNumberSummaryList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone_number_summary_list: Option<Vec<PhoneNumberSummary>>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListPromptsRequest {
    /// <p>The identifier of the Amazon Connect instance.</p>
    #[serde(rename = "InstanceId")]
    pub instance_id: String,
    /// <p>The maximum number of results to return per page.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>The token for the next set of results. Use the value returned in the previous response in the next request to retrieve the next set of results.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListPromptsResponse {
    /// <p>If there are additional results, this is the token for the next set of results.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>Information about the prompts.</p>
    #[serde(rename = "PromptSummaryList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prompt_summary_list: Option<Vec<PromptSummary>>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListQueuesRequest {
    /// <p>The identifier of the Amazon Connect instance.</p>
    #[serde(rename = "InstanceId")]
    pub instance_id: String,
    /// <p>The maximimum number of results to return per page.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>The token for the next set of results. Use the value returned in the previous response in the next request to retrieve the next set of results.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The type of queue.</p>
    #[serde(rename = "QueueTypes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub queue_types: Option<Vec<String>>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListQueuesResponse {
    /// <p>If there are additional results, this is the token for the next set of results.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>Information about the queues.</p>
    #[serde(rename = "QueueSummaryList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub queue_summary_list: Option<Vec<QueueSummary>>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListQuickConnectsRequest {
    /// <p>The identifier of the Amazon Connect instance.</p>
    #[serde(rename = "InstanceId")]
    pub instance_id: String,
    /// <p>The maximimum number of results to return per page.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>The token for the next set of results. Use the value returned in the previous response in the next request to retrieve the next set of results.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The type of quick connect. In the Amazon Connect console, when you create a quick connect, you are prompted to assign one of the following types: Agent (USER), External (PHONE_NUMBER), or Queue (QUEUE).</p>
    #[serde(rename = "QuickConnectTypes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quick_connect_types: Option<Vec<String>>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListQuickConnectsResponse {
    /// <p>If there are additional results, this is the token for the next set of results.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>Information about the quick connects.</p>
    #[serde(rename = "QuickConnectSummaryList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quick_connect_summary_list: Option<Vec<QuickConnectSummary>>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListRoutingProfileQueuesRequest {
    /// <p>The identifier of the Amazon Connect instance.</p>
    #[serde(rename = "InstanceId")]
    pub instance_id: String,
    /// <p>The maximimum number of results to return per page.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>The token for the next set of results. Use the value returned in the previous response in the next request to retrieve the next set of results.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The identifier of the routing profile.</p>
    #[serde(rename = "RoutingProfileId")]
    pub routing_profile_id: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListRoutingProfileQueuesResponse {
    /// <p>If there are additional results, this is the token for the next set of results.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>Information about the routing profiles.</p>
    #[serde(rename = "RoutingProfileQueueConfigSummaryList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub routing_profile_queue_config_summary_list: Option<Vec<RoutingProfileQueueConfigSummary>>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListRoutingProfilesRequest {
    /// <p>The identifier of the Amazon Connect instance.</p>
    #[serde(rename = "InstanceId")]
    pub instance_id: String,
    /// <p>The maximimum number of results to return per page.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>The token for the next set of results. Use the value returned in the previous response in the next request to retrieve the next set of results.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListRoutingProfilesResponse {
    /// <p>If there are additional results, this is the token for the next set of results.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>Information about the routing profiles.</p>
    #[serde(rename = "RoutingProfileSummaryList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub routing_profile_summary_list: Option<Vec<RoutingProfileSummary>>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListSecurityKeysRequest {
    /// <p>The identifier of the Amazon Connect instance.</p>
    #[serde(rename = "InstanceId")]
    pub instance_id: String,
    /// <p>The maximimum number of results to return per page.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>The token for the next set of results. Use the value returned in the previous response in the next request to retrieve the next set of results.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListSecurityKeysResponse {
    /// <p>If there are additional results, this is the token for the next set of results.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The security keys.</p>
    #[serde(rename = "SecurityKeys")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_keys: Option<Vec<SecurityKey>>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListSecurityProfilesRequest {
    /// <p>The identifier of the Amazon Connect instance.</p>
    #[serde(rename = "InstanceId")]
    pub instance_id: String,
    /// <p>The maximimum number of results to return per page.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>The token for the next set of results. Use the value returned in the previous response in the next request to retrieve the next set of results.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListSecurityProfilesResponse {
    /// <p>If there are additional results, this is the token for the next set of results.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>Information about the security profiles.</p>
    #[serde(rename = "SecurityProfileSummaryList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_profile_summary_list: Option<Vec<SecurityProfileSummary>>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListTagsForResourceRequest {
    /// <p>The Amazon Resource Name (ARN) of the resource.</p>
    #[serde(rename = "resourceArn")]
    pub resource_arn: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListTagsForResourceResponse {
    /// <p>Information about the tags.</p>
    #[serde(rename = "tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<::std::collections::HashMap<String, String>>,
}

/// <p>Provides summary information about the use cases for the specified Amazon Connect AppIntegration association.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListUseCasesRequest {
    /// <p>The identifier of the Amazon Connect instance.</p>
    #[serde(rename = "InstanceId")]
    pub instance_id: String,
    /// <p>The identifier for the integration association.</p>
    #[serde(rename = "IntegrationAssociationId")]
    pub integration_association_id: String,
    /// <p>The maximimum number of results to return per page.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>The token for the next set of results. Use the value returned in the previous response in the next request to retrieve the next set of results.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListUseCasesResponse {
    /// <p>If there are additional results, this is the token for the next set of results.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The use cases.</p>
    #[serde(rename = "UseCaseSummaryList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_case_summary_list: Option<Vec<UseCase>>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListUserHierarchyGroupsRequest {
    /// <p>The identifier of the Amazon Connect instance.</p>
    #[serde(rename = "InstanceId")]
    pub instance_id: String,
    /// <p>The maximimum number of results to return per page.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>The token for the next set of results. Use the value returned in the previous response in the next request to retrieve the next set of results.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListUserHierarchyGroupsResponse {
    /// <p>If there are additional results, this is the token for the next set of results.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>Information about the hierarchy groups.</p>
    #[serde(rename = "UserHierarchyGroupSummaryList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_hierarchy_group_summary_list: Option<Vec<HierarchyGroupSummary>>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListUsersRequest {
    /// <p>The identifier of the Amazon Connect instance.</p>
    #[serde(rename = "InstanceId")]
    pub instance_id: String,
    /// <p>The maximimum number of results to return per page.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>The token for the next set of results. Use the value returned in the previous response in the next request to retrieve the next set of results.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListUsersResponse {
    /// <p>If there are additional results, this is the token for the next set of results.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>Information about the users.</p>
    #[serde(rename = "UserSummaryList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_summary_list: Option<Vec<UserSummary>>,
}

/// <p>Contains information about which channels are supported, and how many contacts an agent can have on a channel simultaneously.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct MediaConcurrency {
    /// <p>The channels that agents can handle in the Contact Control Panel (CCP).</p>
    #[serde(rename = "Channel")]
    pub channel: String,
    /// <p>The number of contacts an agent can have on a channel simultaneously.</p>
    #[serde(rename = "Concurrency")]
    pub concurrency: i64,
}

/// <p>The customer's details.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ParticipantDetails {
    /// <p>Display name of the participant.</p>
    #[serde(rename = "DisplayName")]
    pub display_name: String,
}

/// <p>Contains information about a phone number for a quick connect.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct PhoneNumberQuickConnectConfig {
    /// <p>The phone number in E.164 format.</p>
    #[serde(rename = "PhoneNumber")]
    pub phone_number: String,
}

/// <p>Contains summary information about a phone number for a contact center.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct PhoneNumberSummary {
    /// <p>The Amazon Resource Name (ARN) of the phone number.</p>
    #[serde(rename = "Arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p>The identifier of the phone number.</p>
    #[serde(rename = "Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// <p>The phone number.</p>
    #[serde(rename = "PhoneNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone_number: Option<String>,
    /// <p>The ISO country code.</p>
    #[serde(rename = "PhoneNumberCountryCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone_number_country_code: Option<String>,
    /// <p>The type of phone number.</p>
    #[serde(rename = "PhoneNumberType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone_number_type: Option<String>,
}

/// <p>Information about a problem detail.</p>
#[derive(Clone, Debug, Default, PartialEq)]
pub struct ProblemDetail {
    /// <p>The problem detail's message.</p>
    pub message: Option<String>,
}

/// <p>Contains information about the prompt.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct PromptSummary {
    /// <p>The Amazon Resource Name (ARN) of the prompt.</p>
    #[serde(rename = "Arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p>The identifier of the prompt.</p>
    #[serde(rename = "Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// <p>The name of the prompt.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

/// <p>Contains information about a queue for a quick connect. The contact flow must be of type Transfer to Queue.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct QueueQuickConnectConfig {
    /// <p>The identifier of the contact flow.</p>
    #[serde(rename = "ContactFlowId")]
    pub contact_flow_id: String,
    /// <p>The identifier of the queue.</p>
    #[serde(rename = "QueueId")]
    pub queue_id: String,
}

/// <p>Contains information about a queue resource for which metrics are returned.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct QueueReference {
    /// <p>The Amazon Resource Name (ARN) of the queue.</p>
    #[serde(rename = "Arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p>The identifier of the queue.</p>
    #[serde(rename = "Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}

/// <p>Contains summary information about a queue.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct QueueSummary {
    /// <p>The Amazon Resource Name (ARN) of the queue.</p>
    #[serde(rename = "Arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p>The identifier of the queue.</p>
    #[serde(rename = "Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// <p>The name of the queue.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The type of queue.</p>
    #[serde(rename = "QueueType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub queue_type: Option<String>,
}

/// <p>Contains information about a quick connect.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct QuickConnect {
    /// <p>The description.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The name of the quick connect.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The Amazon Resource Name (ARN) of the quick connect.</p>
    #[serde(rename = "QuickConnectARN")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quick_connect_arn: Option<String>,
    /// <p>Contains information about the quick connect.</p>
    #[serde(rename = "QuickConnectConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quick_connect_config: Option<QuickConnectConfig>,
    /// <p>The identifier for the quick connect.</p>
    #[serde(rename = "QuickConnectId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quick_connect_id: Option<String>,
    /// <p>One or more tags.</p>
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<::std::collections::HashMap<String, String>>,
}

/// <p>Contains configuration settings for a quick connect.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct QuickConnectConfig {
    /// <p>The phone configuration. This is required only if QuickConnectType is PHONE_NUMBER.</p>
    #[serde(rename = "PhoneConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone_config: Option<PhoneNumberQuickConnectConfig>,
    /// <p>The queue configuration. This is required only if QuickConnectType is QUEUE.</p>
    #[serde(rename = "QueueConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub queue_config: Option<QueueQuickConnectConfig>,
    /// <p>The type of quick connect. In the Amazon Connect console, when you create a quick connect, you are prompted to assign one of the following types: Agent (USER), External (PHONE_NUMBER), or Queue (QUEUE). </p>
    #[serde(rename = "QuickConnectType")]
    pub quick_connect_type: String,
    /// <p>The user configuration. This is required only if QuickConnectType is USER.</p>
    #[serde(rename = "UserConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_config: Option<UserQuickConnectConfig>,
}

/// <p>Contains summary information about a quick connect.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct QuickConnectSummary {
    /// <p>The Amazon Resource Name (ARN).</p>
    #[serde(rename = "Arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p>The identifier for the quick connect.</p>
    #[serde(rename = "Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// <p>The name.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The type of quick connect. In the Amazon Connect console, when you create a quick connect, you are prompted to assign one of the following types: Agent (USER), External (PHONE_NUMBER), or Queue (QUEUE).</p>
    #[serde(rename = "QuickConnectType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quick_connect_type: Option<String>,
}

/// <p>A link that an agent selects to complete a given task. You can have up to 4,096 UTF-8 bytes across all references for a contact.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct Reference {
    /// <p>A valid URL.</p>
    #[serde(rename = "Type")]
    pub type_: String,
    /// <p>A formatted URL that will be shown to an agent in the Contact Control Panel (CCP)</p>
    #[serde(rename = "Value")]
    pub value: String,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ResumeContactRecordingRequest {
    /// <p>The identifier of the contact.</p>
    #[serde(rename = "ContactId")]
    pub contact_id: String,
    /// <p>The identifier of the contact. This is the identifier of the contact associated with the first interaction with the contact center.</p>
    #[serde(rename = "InitialContactId")]
    pub initial_contact_id: String,
    /// <p>The identifier of the Amazon Connect instance.</p>
    #[serde(rename = "InstanceId")]
    pub instance_id: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ResumeContactRecordingResponse {}

/// <p>Contains information about a routing profile.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct RoutingProfile {
    /// <p>The identifier of the default outbound queue for this routing profile.</p>
    #[serde(rename = "DefaultOutboundQueueId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_outbound_queue_id: Option<String>,
    /// <p>The description of the routing profile.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The identifier of the Amazon Connect instance.</p>
    #[serde(rename = "InstanceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_id: Option<String>,
    /// <p>The channels agents can handle in the Contact Control Panel (CCP) for this routing profile.</p>
    #[serde(rename = "MediaConcurrencies")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub media_concurrencies: Option<Vec<MediaConcurrency>>,
    /// <p>The name of the routing profile.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The Amazon Resource Name (ARN) of the routing profile.</p>
    #[serde(rename = "RoutingProfileArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub routing_profile_arn: Option<String>,
    /// <p>The identifier of the routing profile.</p>
    #[serde(rename = "RoutingProfileId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub routing_profile_id: Option<String>,
    /// <p>One or more tags.</p>
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<::std::collections::HashMap<String, String>>,
}

/// <p>Contains information about the queue and channel for which priority and delay can be set.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct RoutingProfileQueueConfig {
    /// <p>The delay, in seconds, a contact should be in the queue before they are routed to an available agent. For more information, see <a href="https://docs.aws.amazon.com/connect/latest/adminguide/concepts-routing-profiles-priority.html">Queues: priority and delay</a> in the <i>Amazon Connect Administrator Guide</i>.</p>
    #[serde(rename = "Delay")]
    pub delay: i64,
    /// <p>The order in which contacts are to be handled for the queue. For more information, see <a href="https://docs.aws.amazon.com/connect/latest/adminguide/concepts-routing-profiles-priority.html">Queues: priority and delay</a>.</p>
    #[serde(rename = "Priority")]
    pub priority: i64,
    /// <p>Contains information about a queue resource.</p>
    #[serde(rename = "QueueReference")]
    pub queue_reference: RoutingProfileQueueReference,
}

/// <p>Contains summary information about a routing profile queue.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct RoutingProfileQueueConfigSummary {
    /// <p>The channels this queue supports.</p>
    #[serde(rename = "Channel")]
    pub channel: String,
    /// <p>The delay, in seconds, that a contact should be in the queue before they are routed to an available agent. For more information, see <a href="https://docs.aws.amazon.com/connect/latest/adminguide/concepts-routing-profiles-priority.html">Queues: priority and delay</a> in the <i>Amazon Connect Administrator Guide</i>.</p>
    #[serde(rename = "Delay")]
    pub delay: i64,
    /// <p>The order in which contacts are to be handled for the queue. For more information, see <a href="https://docs.aws.amazon.com/connect/latest/adminguide/concepts-routing-profiles-priority.html">Queues: priority and delay</a>.</p>
    #[serde(rename = "Priority")]
    pub priority: i64,
    /// <p>The Amazon Resource Name (ARN) of the queue.</p>
    #[serde(rename = "QueueArn")]
    pub queue_arn: String,
    /// <p>The identifier of the queue.</p>
    #[serde(rename = "QueueId")]
    pub queue_id: String,
    /// <p>The name of the queue.</p>
    #[serde(rename = "QueueName")]
    pub queue_name: String,
}

/// <p>Contains the channel and queue identifier for a routing profile.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct RoutingProfileQueueReference {
    /// <p>The channels agents can handle in the Contact Control Panel (CCP) for this routing profile.</p>
    #[serde(rename = "Channel")]
    pub channel: String,
    /// <p>The identifier of the queue.</p>
    #[serde(rename = "QueueId")]
    pub queue_id: String,
}

/// <p>Contains summary information about a routing profile.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct RoutingProfileSummary {
    /// <p>The Amazon Resource Name (ARN) of the routing profile.</p>
    #[serde(rename = "Arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p>The identifier of the routing profile.</p>
    #[serde(rename = "Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// <p>The name of the routing profile.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

/// <p>Information about the S3 storage type.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct S3Config {
    /// <p>The S3 bucket name.</p>
    #[serde(rename = "BucketName")]
    pub bucket_name: String,
    /// <p>The S3 bucket prefix.</p>
    #[serde(rename = "BucketPrefix")]
    pub bucket_prefix: String,
    /// <p>The S3 encryption configuration.</p>
    #[serde(rename = "EncryptionConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encryption_config: Option<EncryptionConfig>,
}

/// <p>Configuration information of the security key.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct SecurityKey {
    /// <p>The existing association identifier that uniquely identifies the resource type and storage config for the given instance ID.</p>
    #[serde(rename = "AssociationId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub association_id: Option<String>,
    /// <p>When the security key was created.</p>
    #[serde(rename = "CreationTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time: Option<f64>,
    /// <p>The key of the security key.</p>
    #[serde(rename = "Key")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
}

/// <p>Contains information about a security profile.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct SecurityProfileSummary {
    /// <p>The Amazon Resource Name (ARN) of the security profile.</p>
    #[serde(rename = "Arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p>The identifier of the security profile.</p>
    #[serde(rename = "Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// <p>The name of the security profile.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct StartChatContactRequest {
    /// <p>A custom key-value pair using an attribute map. The attributes are standard Amazon Connect attributes, and can be accessed in contact flows just like any other contact attributes. </p> <p>There can be up to 32,768 UTF-8 bytes across all key-value pairs per contact. Attribute keys can include only alphanumeric, dash, and underscore characters.</p>
    #[serde(rename = "Attributes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attributes: Option<::std::collections::HashMap<String, String>>,
    /// <p>A unique, case-sensitive identifier that you provide to ensure the idempotency of the request.</p>
    #[serde(rename = "ClientToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
    /// <p>The identifier of the contact flow for initiating the chat. To see the ContactFlowId in the Amazon Connect console user interface, on the navigation menu go to <b>Routing</b>, <b>Contact Flows</b>. Choose the contact flow. On the contact flow page, under the name of the contact flow, choose <b>Show additional flow information</b>. The ContactFlowId is the last part of the ARN, shown here in bold: </p> <p>arn:aws:connect:us-west-2:xxxxxxxxxxxx:instance/xxxxxxxx-xxxx-xxxx-xxxx-xxxxxxxxxxxx/contact-flow/<b>846ec553-a005-41c0-8341-xxxxxxxxxxxx</b> </p>
    #[serde(rename = "ContactFlowId")]
    pub contact_flow_id: String,
    /// <p>The initial message to be sent to the newly created chat.</p>
    #[serde(rename = "InitialMessage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub initial_message: Option<ChatMessage>,
    /// <p>The identifier of the Amazon Connect instance.</p>
    #[serde(rename = "InstanceId")]
    pub instance_id: String,
    /// <p>Information identifying the participant.</p>
    #[serde(rename = "ParticipantDetails")]
    pub participant_details: ParticipantDetails,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct StartChatContactResponse {
    /// <p>The identifier of this contact within the Amazon Connect instance. </p>
    #[serde(rename = "ContactId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contact_id: Option<String>,
    /// <p>The identifier for a chat participant. The participantId for a chat participant is the same throughout the chat lifecycle.</p>
    #[serde(rename = "ParticipantId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub participant_id: Option<String>,
    /// <p>The token used by the chat participant to call <a href="https://docs.aws.amazon.com/connect-participant/latest/APIReference/API_CreateParticipantConnection.html">CreateParticipantConnection</a>. The participant token is valid for the lifetime of a chat participant.</p>
    #[serde(rename = "ParticipantToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub participant_token: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct StartContactRecordingRequest {
    /// <p>The identifier of the contact.</p>
    #[serde(rename = "ContactId")]
    pub contact_id: String,
    /// <p>The identifier of the contact. This is the identifier of the contact associated with the first interaction with the contact center.</p>
    #[serde(rename = "InitialContactId")]
    pub initial_contact_id: String,
    /// <p>The identifier of the Amazon Connect instance.</p>
    #[serde(rename = "InstanceId")]
    pub instance_id: String,
    /// <p>Who is being recorded.</p>
    #[serde(rename = "VoiceRecordingConfiguration")]
    pub voice_recording_configuration: VoiceRecordingConfiguration,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct StartContactRecordingResponse {}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct StartOutboundVoiceContactRequest {
    /// <p>A custom key-value pair using an attribute map. The attributes are standard Amazon Connect attributes, and can be accessed in contact flows just like any other contact attributes.</p> <p>There can be up to 32,768 UTF-8 bytes across all key-value pairs per contact. Attribute keys can include only alphanumeric, dash, and underscore characters.</p>
    #[serde(rename = "Attributes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attributes: Option<::std::collections::HashMap<String, String>>,
    /// <p>A unique, case-sensitive identifier that you provide to ensure the idempotency of the request. The token is valid for 7 days after creation. If a contact is already started, the contact ID is returned. If the contact is disconnected, a new contact is started.</p>
    #[serde(rename = "ClientToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
    /// <p>The identifier of the contact flow for the outbound call. To see the ContactFlowId in the Amazon Connect console user interface, on the navigation menu go to <b>Routing</b>, <b>Contact Flows</b>. Choose the contact flow. On the contact flow page, under the name of the contact flow, choose <b>Show additional flow information</b>. The ContactFlowId is the last part of the ARN, shown here in bold: </p> <p>arn:aws:connect:us-west-2:xxxxxxxxxxxx:instance/xxxxxxxx-xxxx-xxxx-xxxx-xxxxxxxxxxxx/contact-flow/<b>846ec553-a005-41c0-8341-xxxxxxxxxxxx</b> </p>
    #[serde(rename = "ContactFlowId")]
    pub contact_flow_id: String,
    /// <p>The phone number of the customer, in E.164 format.</p>
    #[serde(rename = "DestinationPhoneNumber")]
    pub destination_phone_number: String,
    /// <p>The identifier of the Amazon Connect instance.</p>
    #[serde(rename = "InstanceId")]
    pub instance_id: String,
    /// <p>The queue for the call. If you specify a queue, the phone displayed for caller ID is the phone number specified in the queue. If you do not specify a queue, the queue defined in the contact flow is used. If you do not specify a queue, you must specify a source phone number.</p>
    #[serde(rename = "QueueId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub queue_id: Option<String>,
    /// <p>The phone number associated with the Amazon Connect instance, in E.164 format. If you do not specify a source phone number, you must specify a queue.</p>
    #[serde(rename = "SourcePhoneNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_phone_number: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct StartOutboundVoiceContactResponse {
    /// <p>The identifier of this contact within the Amazon Connect instance.</p>
    #[serde(rename = "ContactId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contact_id: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct StartTaskContactRequest {
    /// <p>A custom key-value pair using an attribute map. The attributes are standard Amazon Connect attributes, and can be accessed in contact flows just like any other contact attributes.</p> <p>There can be up to 32,768 UTF-8 bytes across all key-value pairs per contact. Attribute keys can include only alphanumeric, dash, and underscore characters.</p>
    #[serde(rename = "Attributes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attributes: Option<::std::collections::HashMap<String, String>>,
    /// <p>A unique, case-sensitive identifier that you provide to ensure the idempotency of the request.</p>
    #[serde(rename = "ClientToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
    /// <p>The identifier of the contact flow for initiating the tasks. To see the ContactFlowId in the Amazon Connect console user interface, on the navigation menu go to <b>Routing</b>, <b>Contact Flows</b>. Choose the contact flow. On the contact flow page, under the name of the contact flow, choose <b>Show additional flow information</b>. The ContactFlowId is the last part of the ARN, shown here in bold: </p> <p>arn:aws:connect:us-west-2:xxxxxxxxxxxx:instance/xxxxxxxx-xxxx-xxxx-xxxx-xxxxxxxxxxxx/contact-flow/<b>846ec553-a005-41c0-8341-xxxxxxxxxxxx</b> </p>
    #[serde(rename = "ContactFlowId")]
    pub contact_flow_id: String,
    /// <p>A description of the task that is shown to an agent in the Contact Control Panel (CCP).</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The identifier of the Amazon Connect instance.</p>
    #[serde(rename = "InstanceId")]
    pub instance_id: String,
    /// <p>The name of a task that is shown to an agent in the Contact Control Panel (CCP).</p>
    #[serde(rename = "Name")]
    pub name: String,
    /// <p>The identifier of the previous chat, voice, or task contact. </p>
    #[serde(rename = "PreviousContactId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub previous_contact_id: Option<String>,
    /// <p>A formatted URL that is shown to an agent in the Contact Control Panel (CCP).</p>
    #[serde(rename = "References")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub references: Option<::std::collections::HashMap<String, Reference>>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct StartTaskContactResponse {
    /// <p>The identifier of this contact within the Amazon Connect instance.</p>
    #[serde(rename = "ContactId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contact_id: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct StopContactRecordingRequest {
    /// <p>The identifier of the contact.</p>
    #[serde(rename = "ContactId")]
    pub contact_id: String,
    /// <p>The identifier of the contact. This is the identifier of the contact associated with the first interaction with the contact center.</p>
    #[serde(rename = "InitialContactId")]
    pub initial_contact_id: String,
    /// <p>The identifier of the Amazon Connect instance.</p>
    #[serde(rename = "InstanceId")]
    pub instance_id: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct StopContactRecordingResponse {}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct StopContactRequest {
    /// <p>The ID of the contact.</p>
    #[serde(rename = "ContactId")]
    pub contact_id: String,
    /// <p>The identifier of the Amazon Connect instance.</p>
    #[serde(rename = "InstanceId")]
    pub instance_id: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct StopContactResponse {}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct SuspendContactRecordingRequest {
    /// <p>The identifier of the contact.</p>
    #[serde(rename = "ContactId")]
    pub contact_id: String,
    /// <p>The identifier of the contact. This is the identifier of the contact associated with the first interaction with the contact center.</p>
    #[serde(rename = "InitialContactId")]
    pub initial_contact_id: String,
    /// <p>The identifier of the Amazon Connect instance.</p>
    #[serde(rename = "InstanceId")]
    pub instance_id: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct SuspendContactRecordingResponse {}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct TagResourceRequest {
    /// <p>The Amazon Resource Name (ARN) of the resource.</p>
    #[serde(rename = "resourceArn")]
    pub resource_arn: String,
    /// <p>One or more tags. For example, { "tags": {"key1":"value1", "key2":"value2"} }.</p>
    #[serde(rename = "tags")]
    pub tags: ::std::collections::HashMap<String, String>,
}

/// <p>Contains information about the threshold for service level metrics.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct Threshold {
    /// <p>The type of comparison. Only "less than" (LT) comparisons are supported.</p>
    #[serde(rename = "Comparison")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comparison: Option<String>,
    /// <p>The threshold value to compare.</p>
    #[serde(rename = "ThresholdValue")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub threshold_value: Option<f64>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UntagResourceRequest {
    /// <p>The Amazon Resource Name (ARN) of the resource.</p>
    #[serde(rename = "resourceArn")]
    pub resource_arn: String,
    /// <p>The tag keys.</p>
    #[serde(rename = "tagKeys")]
    pub tag_keys: Vec<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UpdateContactAttributesRequest {
    /// <p>The Amazon Connect attributes. These attributes can be accessed in contact flows just like any other contact attributes.</p> <p>You can have up to 32,768 UTF-8 bytes across all attributes for a contact. Attribute keys can include only alphanumeric, dash, and underscore characters.</p>
    #[serde(rename = "Attributes")]
    pub attributes: ::std::collections::HashMap<String, String>,
    /// <p>The identifier of the contact. This is the identifier of the contact associated with the first interaction with the contact center.</p>
    #[serde(rename = "InitialContactId")]
    pub initial_contact_id: String,
    /// <p>The identifier of the Amazon Connect instance.</p>
    #[serde(rename = "InstanceId")]
    pub instance_id: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UpdateContactAttributesResponse {}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UpdateContactFlowContentRequest {
    /// <p>The identifier of the contact flow.</p>
    #[serde(rename = "ContactFlowId")]
    pub contact_flow_id: String,
    /// <p>The JSON string that represents contact flow’s content. For an example, see <a href="https://docs.aws.amazon.com/connect/latest/adminguide/flow-language-example.html">Example contact flow in Amazon Connect Flow language</a> in the <i>Amazon Connect Administrator Guide</i>. </p>
    #[serde(rename = "Content")]
    pub content: String,
    /// <p>The identifier of the Amazon Connect instance.</p>
    #[serde(rename = "InstanceId")]
    pub instance_id: String,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UpdateContactFlowNameRequest {
    /// <p>The identifier of the contact flow.</p>
    #[serde(rename = "ContactFlowId")]
    pub contact_flow_id: String,
    /// <p>The description of the contact flow.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The identifier of the Amazon Connect instance.</p>
    #[serde(rename = "InstanceId")]
    pub instance_id: String,
    /// <p>The name of the contact flow.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UpdateInstanceAttributeRequest {
    /// <p>The type of attribute.</p>
    #[serde(rename = "AttributeType")]
    pub attribute_type: String,
    /// <p>The identifier of the Amazon Connect instance.</p>
    #[serde(rename = "InstanceId")]
    pub instance_id: String,
    /// <p>The value for the attribute. Maximum character limit is 100. </p>
    #[serde(rename = "Value")]
    pub value: String,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UpdateInstanceStorageConfigRequest {
    /// <p>The existing association identifier that uniquely identifies the resource type and storage config for the given instance ID.</p>
    #[serde(rename = "AssociationId")]
    pub association_id: String,
    /// <p>The identifier of the Amazon Connect instance.</p>
    #[serde(rename = "InstanceId")]
    pub instance_id: String,
    /// <p>A valid resource type.</p>
    #[serde(rename = "ResourceType")]
    pub resource_type: String,
    #[serde(rename = "StorageConfig")]
    pub storage_config: InstanceStorageConfig,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UpdateQuickConnectConfigRequest {
    /// <p>The identifier of the Amazon Connect instance.</p>
    #[serde(rename = "InstanceId")]
    pub instance_id: String,
    /// <p>Information about the configuration settings for the quick connect.</p>
    #[serde(rename = "QuickConnectConfig")]
    pub quick_connect_config: QuickConnectConfig,
    /// <p>The identifier for the quick connect.</p>
    #[serde(rename = "QuickConnectId")]
    pub quick_connect_id: String,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UpdateQuickConnectNameRequest {
    /// <p>The description of the quick connect.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The identifier of the Amazon Connect instance.</p>
    #[serde(rename = "InstanceId")]
    pub instance_id: String,
    /// <p>The name of the quick connect.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The identifier for the quick connect.</p>
    #[serde(rename = "QuickConnectId")]
    pub quick_connect_id: String,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UpdateRoutingProfileConcurrencyRequest {
    /// <p>The identifier of the Amazon Connect instance.</p>
    #[serde(rename = "InstanceId")]
    pub instance_id: String,
    /// <p>The channels agents can handle in the Contact Control Panel (CCP).</p>
    #[serde(rename = "MediaConcurrencies")]
    pub media_concurrencies: Vec<MediaConcurrency>,
    /// <p>The identifier of the routing profile.</p>
    #[serde(rename = "RoutingProfileId")]
    pub routing_profile_id: String,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UpdateRoutingProfileDefaultOutboundQueueRequest {
    /// <p>The identifier for the default outbound queue.</p>
    #[serde(rename = "DefaultOutboundQueueId")]
    pub default_outbound_queue_id: String,
    /// <p>The identifier of the Amazon Connect instance.</p>
    #[serde(rename = "InstanceId")]
    pub instance_id: String,
    /// <p>The identifier of the routing profile.</p>
    #[serde(rename = "RoutingProfileId")]
    pub routing_profile_id: String,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UpdateRoutingProfileNameRequest {
    /// <p>The description of the routing profile. Must not be more than 250 characters.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The identifier of the Amazon Connect instance.</p>
    #[serde(rename = "InstanceId")]
    pub instance_id: String,
    /// <p>The name of the routing profile. Must not be more than 127 characters.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The identifier of the routing profile.</p>
    #[serde(rename = "RoutingProfileId")]
    pub routing_profile_id: String,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UpdateRoutingProfileQueuesRequest {
    /// <p>The identifier of the Amazon Connect instance.</p>
    #[serde(rename = "InstanceId")]
    pub instance_id: String,
    /// <p>The queues to be updated for this routing profile. Queues must first be associated to the routing profile. You can do this using AssociateRoutingProfileQueues.</p>
    #[serde(rename = "QueueConfigs")]
    pub queue_configs: Vec<RoutingProfileQueueConfig>,
    /// <p>The identifier of the routing profile.</p>
    #[serde(rename = "RoutingProfileId")]
    pub routing_profile_id: String,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UpdateUserHierarchyGroupNameRequest {
    /// <p>The identifier of the hierarchy group.</p>
    #[serde(rename = "HierarchyGroupId")]
    pub hierarchy_group_id: String,
    /// <p>The identifier of the Amazon Connect instance.</p>
    #[serde(rename = "InstanceId")]
    pub instance_id: String,
    /// <p>The name of the hierarchy group. Must not be more than 100 characters.</p>
    #[serde(rename = "Name")]
    pub name: String,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UpdateUserHierarchyRequest {
    /// <p>The identifier of the hierarchy group.</p>
    #[serde(rename = "HierarchyGroupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hierarchy_group_id: Option<String>,
    /// <p>The identifier of the Amazon Connect instance.</p>
    #[serde(rename = "InstanceId")]
    pub instance_id: String,
    /// <p>The identifier of the user account.</p>
    #[serde(rename = "UserId")]
    pub user_id: String,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UpdateUserHierarchyStructureRequest {
    /// <p>The hierarchy levels to update.</p>
    #[serde(rename = "HierarchyStructure")]
    pub hierarchy_structure: HierarchyStructureUpdate,
    /// <p>The identifier of the Amazon Connect instance.</p>
    #[serde(rename = "InstanceId")]
    pub instance_id: String,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UpdateUserIdentityInfoRequest {
    /// <p>The identity information for the user.</p>
    #[serde(rename = "IdentityInfo")]
    pub identity_info: UserIdentityInfo,
    /// <p>The identifier of the Amazon Connect instance.</p>
    #[serde(rename = "InstanceId")]
    pub instance_id: String,
    /// <p>The identifier of the user account.</p>
    #[serde(rename = "UserId")]
    pub user_id: String,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UpdateUserPhoneConfigRequest {
    /// <p>The identifier of the Amazon Connect instance.</p>
    #[serde(rename = "InstanceId")]
    pub instance_id: String,
    /// <p>Information about phone configuration settings for the user.</p>
    #[serde(rename = "PhoneConfig")]
    pub phone_config: UserPhoneConfig,
    /// <p>The identifier of the user account.</p>
    #[serde(rename = "UserId")]
    pub user_id: String,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UpdateUserRoutingProfileRequest {
    /// <p>The identifier of the Amazon Connect instance.</p>
    #[serde(rename = "InstanceId")]
    pub instance_id: String,
    /// <p>The identifier of the routing profile for the user.</p>
    #[serde(rename = "RoutingProfileId")]
    pub routing_profile_id: String,
    /// <p>The identifier of the user account.</p>
    #[serde(rename = "UserId")]
    pub user_id: String,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UpdateUserSecurityProfilesRequest {
    /// <p>The identifier of the Amazon Connect instance.</p>
    #[serde(rename = "InstanceId")]
    pub instance_id: String,
    /// <p>The identifiers of the security profiles for the user.</p>
    #[serde(rename = "SecurityProfileIds")]
    pub security_profile_ids: Vec<String>,
    /// <p>The identifier of the user account.</p>
    #[serde(rename = "UserId")]
    pub user_id: String,
}

/// <p>Contains the use case.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UseCase {
    /// <p>The Amazon Resource Name (ARN) for the use case.</p>
    #[serde(rename = "UseCaseArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_case_arn: Option<String>,
    /// <p>The identifier for the use case.</p>
    #[serde(rename = "UseCaseId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_case_id: Option<String>,
    /// <p>The type of use case to associate to the AppIntegration association. Each AppIntegration association can have only one of each use case type.</p>
    #[serde(rename = "UseCaseType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_case_type: Option<String>,
}

/// <p>Contains information about a user account for a Amazon Connect instance.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct User {
    /// <p>The Amazon Resource Name (ARN) of the user account.</p>
    #[serde(rename = "Arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p>The identifier of the user account in the directory used for identity management.</p>
    #[serde(rename = "DirectoryUserId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub directory_user_id: Option<String>,
    /// <p>The identifier of the hierarchy group for the user.</p>
    #[serde(rename = "HierarchyGroupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hierarchy_group_id: Option<String>,
    /// <p>The identifier of the user account.</p>
    #[serde(rename = "Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// <p>Information about the user identity.</p>
    #[serde(rename = "IdentityInfo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identity_info: Option<UserIdentityInfo>,
    /// <p>Information about the phone configuration for the user.</p>
    #[serde(rename = "PhoneConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone_config: Option<UserPhoneConfig>,
    /// <p>The identifier of the routing profile for the user.</p>
    #[serde(rename = "RoutingProfileId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub routing_profile_id: Option<String>,
    /// <p>The identifiers of the security profiles for the user.</p>
    #[serde(rename = "SecurityProfileIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_profile_ids: Option<Vec<String>>,
    /// <p>The tags.</p>
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<::std::collections::HashMap<String, String>>,
    /// <p>The user name assigned to the user account.</p>
    #[serde(rename = "Username")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
}

/// <p>Contains information about the identity of a user.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct UserIdentityInfo {
    /// <p>The email address. If you are using SAML for identity management and include this parameter, an error is returned.</p>
    #[serde(rename = "Email")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    /// <p>The first name. This is required if you are using Amazon Connect or SAML for identity management.</p>
    #[serde(rename = "FirstName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_name: Option<String>,
    /// <p>The last name. This is required if you are using Amazon Connect or SAML for identity management.</p>
    #[serde(rename = "LastName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_name: Option<String>,
}

/// <p>Contains information about the phone configuration settings for a user.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct UserPhoneConfig {
    /// <p>The After Call Work (ACW) timeout setting, in seconds.</p>
    #[serde(rename = "AfterContactWorkTimeLimit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub after_contact_work_time_limit: Option<i64>,
    /// <p>The Auto accept setting.</p>
    #[serde(rename = "AutoAccept")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_accept: Option<bool>,
    /// <p>The phone number for the user's desk phone.</p>
    #[serde(rename = "DeskPhoneNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub desk_phone_number: Option<String>,
    /// <p>The phone type.</p>
    #[serde(rename = "PhoneType")]
    pub phone_type: String,
}

/// <p>Contains information about the quick connect configuration settings for a user. The contact flow must be of type Transfer to Agent.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct UserQuickConnectConfig {
    /// <p>The identifier of the contact flow.</p>
    #[serde(rename = "ContactFlowId")]
    pub contact_flow_id: String,
    /// <p>The identifier of the user.</p>
    #[serde(rename = "UserId")]
    pub user_id: String,
}

/// <p>Contains summary information about a user.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UserSummary {
    /// <p>The Amazon Resource Name (ARN) of the user account.</p>
    #[serde(rename = "Arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p>The identifier of the user account.</p>
    #[serde(rename = "Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// <p>The Amazon Connect user name of the user account.</p>
    #[serde(rename = "Username")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
}

/// <p>Contains information about the recording configuration settings.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct VoiceRecordingConfiguration {
    /// <p>Identifies which track is being recorded.</p>
    #[serde(rename = "VoiceRecordingTrack")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub voice_recording_track: Option<String>,
}

/// Errors returned by AssociateApprovedOrigin
#[derive(Debug, PartialEq)]
pub enum AssociateApprovedOriginError {
    /// <p>Request processing failed due to an error or failure with the service.</p>
    InternalService(String),
    /// <p>One or more of the specified parameters are not valid.</p>
    InvalidParameter(String),
    /// <p>The request is not valid.</p>
    InvalidRequest(String),
    /// <p>A resource already has that name.</p>
    ResourceConflict(String),
    /// <p>The specified resource was not found.</p>
    ResourceNotFound(String),
    /// <p>The service quota has been exceeded.</p>
    ServiceQuotaExceeded(String),
    /// <p>The throttling limit has been exceeded.</p>
    Throttling(String),
}

impl AssociateApprovedOriginError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<AssociateApprovedOriginError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InternalServiceException" => {
                    return RusotoError::Service(AssociateApprovedOriginError::InternalService(
                        err.msg,
                    ))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(AssociateApprovedOriginError::InvalidParameter(
                        err.msg,
                    ))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(AssociateApprovedOriginError::InvalidRequest(
                        err.msg,
                    ))
                }
                "ResourceConflictException" => {
                    return RusotoError::Service(AssociateApprovedOriginError::ResourceConflict(
                        err.msg,
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(AssociateApprovedOriginError::ResourceNotFound(
                        err.msg,
                    ))
                }
                "ServiceQuotaExceededException" => {
                    return RusotoError::Service(
                        AssociateApprovedOriginError::ServiceQuotaExceeded(err.msg),
                    )
                }
                "ThrottlingException" => {
                    return RusotoError::Service(AssociateApprovedOriginError::Throttling(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for AssociateApprovedOriginError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            AssociateApprovedOriginError::InternalService(ref cause) => write!(f, "{}", cause),
            AssociateApprovedOriginError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            AssociateApprovedOriginError::InvalidRequest(ref cause) => write!(f, "{}", cause),
            AssociateApprovedOriginError::ResourceConflict(ref cause) => write!(f, "{}", cause),
            AssociateApprovedOriginError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            AssociateApprovedOriginError::ServiceQuotaExceeded(ref cause) => write!(f, "{}", cause),
            AssociateApprovedOriginError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for AssociateApprovedOriginError {}
/// Errors returned by AssociateInstanceStorageConfig
#[derive(Debug, PartialEq)]
pub enum AssociateInstanceStorageConfigError {
    /// <p>Request processing failed due to an error or failure with the service.</p>
    InternalService(String),
    /// <p>One or more of the specified parameters are not valid.</p>
    InvalidParameter(String),
    /// <p>The request is not valid.</p>
    InvalidRequest(String),
    /// <p>A resource already has that name.</p>
    ResourceConflict(String),
    /// <p>The specified resource was not found.</p>
    ResourceNotFound(String),
    /// <p>The throttling limit has been exceeded.</p>
    Throttling(String),
}

impl AssociateInstanceStorageConfigError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<AssociateInstanceStorageConfigError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InternalServiceException" => {
                    return RusotoError::Service(
                        AssociateInstanceStorageConfigError::InternalService(err.msg),
                    )
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(
                        AssociateInstanceStorageConfigError::InvalidParameter(err.msg),
                    )
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(
                        AssociateInstanceStorageConfigError::InvalidRequest(err.msg),
                    )
                }
                "ResourceConflictException" => {
                    return RusotoError::Service(
                        AssociateInstanceStorageConfigError::ResourceConflict(err.msg),
                    )
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(
                        AssociateInstanceStorageConfigError::ResourceNotFound(err.msg),
                    )
                }
                "ThrottlingException" => {
                    return RusotoError::Service(AssociateInstanceStorageConfigError::Throttling(
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
impl fmt::Display for AssociateInstanceStorageConfigError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            AssociateInstanceStorageConfigError::InternalService(ref cause) => {
                write!(f, "{}", cause)
            }
            AssociateInstanceStorageConfigError::InvalidParameter(ref cause) => {
                write!(f, "{}", cause)
            }
            AssociateInstanceStorageConfigError::InvalidRequest(ref cause) => {
                write!(f, "{}", cause)
            }
            AssociateInstanceStorageConfigError::ResourceConflict(ref cause) => {
                write!(f, "{}", cause)
            }
            AssociateInstanceStorageConfigError::ResourceNotFound(ref cause) => {
                write!(f, "{}", cause)
            }
            AssociateInstanceStorageConfigError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for AssociateInstanceStorageConfigError {}
/// Errors returned by AssociateLambdaFunction
#[derive(Debug, PartialEq)]
pub enum AssociateLambdaFunctionError {
    /// <p>Request processing failed due to an error or failure with the service.</p>
    InternalService(String),
    /// <p>One or more of the specified parameters are not valid.</p>
    InvalidParameter(String),
    /// <p>The request is not valid.</p>
    InvalidRequest(String),
    /// <p>A resource already has that name.</p>
    ResourceConflict(String),
    /// <p>The specified resource was not found.</p>
    ResourceNotFound(String),
    /// <p>The service quota has been exceeded.</p>
    ServiceQuotaExceeded(String),
    /// <p>The throttling limit has been exceeded.</p>
    Throttling(String),
}

impl AssociateLambdaFunctionError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<AssociateLambdaFunctionError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InternalServiceException" => {
                    return RusotoError::Service(AssociateLambdaFunctionError::InternalService(
                        err.msg,
                    ))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(AssociateLambdaFunctionError::InvalidParameter(
                        err.msg,
                    ))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(AssociateLambdaFunctionError::InvalidRequest(
                        err.msg,
                    ))
                }
                "ResourceConflictException" => {
                    return RusotoError::Service(AssociateLambdaFunctionError::ResourceConflict(
                        err.msg,
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(AssociateLambdaFunctionError::ResourceNotFound(
                        err.msg,
                    ))
                }
                "ServiceQuotaExceededException" => {
                    return RusotoError::Service(
                        AssociateLambdaFunctionError::ServiceQuotaExceeded(err.msg),
                    )
                }
                "ThrottlingException" => {
                    return RusotoError::Service(AssociateLambdaFunctionError::Throttling(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for AssociateLambdaFunctionError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            AssociateLambdaFunctionError::InternalService(ref cause) => write!(f, "{}", cause),
            AssociateLambdaFunctionError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            AssociateLambdaFunctionError::InvalidRequest(ref cause) => write!(f, "{}", cause),
            AssociateLambdaFunctionError::ResourceConflict(ref cause) => write!(f, "{}", cause),
            AssociateLambdaFunctionError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            AssociateLambdaFunctionError::ServiceQuotaExceeded(ref cause) => write!(f, "{}", cause),
            AssociateLambdaFunctionError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for AssociateLambdaFunctionError {}
/// Errors returned by AssociateLexBot
#[derive(Debug, PartialEq)]
pub enum AssociateLexBotError {
    /// <p>Request processing failed due to an error or failure with the service.</p>
    InternalService(String),
    /// <p>One or more of the specified parameters are not valid.</p>
    InvalidParameter(String),
    /// <p>The request is not valid.</p>
    InvalidRequest(String),
    /// <p>A resource already has that name.</p>
    ResourceConflict(String),
    /// <p>The specified resource was not found.</p>
    ResourceNotFound(String),
    /// <p>The service quota has been exceeded.</p>
    ServiceQuotaExceeded(String),
    /// <p>The throttling limit has been exceeded.</p>
    Throttling(String),
}

impl AssociateLexBotError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<AssociateLexBotError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InternalServiceException" => {
                    return RusotoError::Service(AssociateLexBotError::InternalService(err.msg))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(AssociateLexBotError::InvalidParameter(err.msg))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(AssociateLexBotError::InvalidRequest(err.msg))
                }
                "ResourceConflictException" => {
                    return RusotoError::Service(AssociateLexBotError::ResourceConflict(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(AssociateLexBotError::ResourceNotFound(err.msg))
                }
                "ServiceQuotaExceededException" => {
                    return RusotoError::Service(AssociateLexBotError::ServiceQuotaExceeded(
                        err.msg,
                    ))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(AssociateLexBotError::Throttling(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for AssociateLexBotError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            AssociateLexBotError::InternalService(ref cause) => write!(f, "{}", cause),
            AssociateLexBotError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            AssociateLexBotError::InvalidRequest(ref cause) => write!(f, "{}", cause),
            AssociateLexBotError::ResourceConflict(ref cause) => write!(f, "{}", cause),
            AssociateLexBotError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            AssociateLexBotError::ServiceQuotaExceeded(ref cause) => write!(f, "{}", cause),
            AssociateLexBotError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for AssociateLexBotError {}
/// Errors returned by AssociateRoutingProfileQueues
#[derive(Debug, PartialEq)]
pub enum AssociateRoutingProfileQueuesError {
    /// <p>Request processing failed due to an error or failure with the service.</p>
    InternalService(String),
    /// <p>One or more of the specified parameters are not valid.</p>
    InvalidParameter(String),
    /// <p>The request is not valid.</p>
    InvalidRequest(String),
    /// <p>The specified resource was not found.</p>
    ResourceNotFound(String),
    /// <p>The throttling limit has been exceeded.</p>
    Throttling(String),
}

impl AssociateRoutingProfileQueuesError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<AssociateRoutingProfileQueuesError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InternalServiceException" => {
                    return RusotoError::Service(
                        AssociateRoutingProfileQueuesError::InternalService(err.msg),
                    )
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(
                        AssociateRoutingProfileQueuesError::InvalidParameter(err.msg),
                    )
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(
                        AssociateRoutingProfileQueuesError::InvalidRequest(err.msg),
                    )
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(
                        AssociateRoutingProfileQueuesError::ResourceNotFound(err.msg),
                    )
                }
                "ThrottlingException" => {
                    return RusotoError::Service(AssociateRoutingProfileQueuesError::Throttling(
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
impl fmt::Display for AssociateRoutingProfileQueuesError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            AssociateRoutingProfileQueuesError::InternalService(ref cause) => {
                write!(f, "{}", cause)
            }
            AssociateRoutingProfileQueuesError::InvalidParameter(ref cause) => {
                write!(f, "{}", cause)
            }
            AssociateRoutingProfileQueuesError::InvalidRequest(ref cause) => write!(f, "{}", cause),
            AssociateRoutingProfileQueuesError::ResourceNotFound(ref cause) => {
                write!(f, "{}", cause)
            }
            AssociateRoutingProfileQueuesError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for AssociateRoutingProfileQueuesError {}
/// Errors returned by AssociateSecurityKey
#[derive(Debug, PartialEq)]
pub enum AssociateSecurityKeyError {
    /// <p>Request processing failed due to an error or failure with the service.</p>
    InternalService(String),
    /// <p>One or more of the specified parameters are not valid.</p>
    InvalidParameter(String),
    /// <p>The request is not valid.</p>
    InvalidRequest(String),
    /// <p>A resource already has that name.</p>
    ResourceConflict(String),
    /// <p>The specified resource was not found.</p>
    ResourceNotFound(String),
    /// <p>The service quota has been exceeded.</p>
    ServiceQuotaExceeded(String),
    /// <p>The throttling limit has been exceeded.</p>
    Throttling(String),
}

impl AssociateSecurityKeyError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<AssociateSecurityKeyError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InternalServiceException" => {
                    return RusotoError::Service(AssociateSecurityKeyError::InternalService(
                        err.msg,
                    ))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(AssociateSecurityKeyError::InvalidParameter(
                        err.msg,
                    ))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(AssociateSecurityKeyError::InvalidRequest(err.msg))
                }
                "ResourceConflictException" => {
                    return RusotoError::Service(AssociateSecurityKeyError::ResourceConflict(
                        err.msg,
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(AssociateSecurityKeyError::ResourceNotFound(
                        err.msg,
                    ))
                }
                "ServiceQuotaExceededException" => {
                    return RusotoError::Service(AssociateSecurityKeyError::ServiceQuotaExceeded(
                        err.msg,
                    ))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(AssociateSecurityKeyError::Throttling(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for AssociateSecurityKeyError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            AssociateSecurityKeyError::InternalService(ref cause) => write!(f, "{}", cause),
            AssociateSecurityKeyError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            AssociateSecurityKeyError::InvalidRequest(ref cause) => write!(f, "{}", cause),
            AssociateSecurityKeyError::ResourceConflict(ref cause) => write!(f, "{}", cause),
            AssociateSecurityKeyError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            AssociateSecurityKeyError::ServiceQuotaExceeded(ref cause) => write!(f, "{}", cause),
            AssociateSecurityKeyError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for AssociateSecurityKeyError {}
/// Errors returned by CreateContactFlow
#[derive(Debug, PartialEq)]
pub enum CreateContactFlowError {
    /// <p>A resource with the specified name already exists.</p>
    DuplicateResource(String),
    /// <p>Request processing failed due to an error or failure with the service.</p>
    InternalService(String),
    /// <p>The contact flow is not valid.</p>
    InvalidContactFlow(String),
    /// <p>One or more of the specified parameters are not valid.</p>
    InvalidParameter(String),
    /// <p>The request is not valid.</p>
    InvalidRequest(String),
    /// <p>The allowed limit for the resource has been exceeded.</p>
    LimitExceeded(String),
    /// <p>The specified resource was not found.</p>
    ResourceNotFound(String),
    /// <p>The throttling limit has been exceeded.</p>
    Throttling(String),
}

impl CreateContactFlowError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateContactFlowError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "DuplicateResourceException" => {
                    return RusotoError::Service(CreateContactFlowError::DuplicateResource(err.msg))
                }
                "InternalServiceException" => {
                    return RusotoError::Service(CreateContactFlowError::InternalService(err.msg))
                }
                "InvalidContactFlowException" => {
                    return RusotoError::Service(CreateContactFlowError::InvalidContactFlow(
                        err.msg,
                    ))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(CreateContactFlowError::InvalidParameter(err.msg))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(CreateContactFlowError::InvalidRequest(err.msg))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(CreateContactFlowError::LimitExceeded(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(CreateContactFlowError::ResourceNotFound(err.msg))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(CreateContactFlowError::Throttling(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for CreateContactFlowError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreateContactFlowError::DuplicateResource(ref cause) => write!(f, "{}", cause),
            CreateContactFlowError::InternalService(ref cause) => write!(f, "{}", cause),
            CreateContactFlowError::InvalidContactFlow(ref cause) => write!(f, "{}", cause),
            CreateContactFlowError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            CreateContactFlowError::InvalidRequest(ref cause) => write!(f, "{}", cause),
            CreateContactFlowError::LimitExceeded(ref cause) => write!(f, "{}", cause),
            CreateContactFlowError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            CreateContactFlowError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CreateContactFlowError {}
/// Errors returned by CreateInstance
#[derive(Debug, PartialEq)]
pub enum CreateInstanceError {
    /// <p>Request processing failed due to an error or failure with the service.</p>
    InternalService(String),
    /// <p>The request is not valid.</p>
    InvalidRequest(String),
    /// <p>The specified resource was not found.</p>
    ResourceNotFound(String),
    /// <p>The service quota has been exceeded.</p>
    ServiceQuotaExceeded(String),
    /// <p>The throttling limit has been exceeded.</p>
    Throttling(String),
}

impl CreateInstanceError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateInstanceError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InternalServiceException" => {
                    return RusotoError::Service(CreateInstanceError::InternalService(err.msg))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(CreateInstanceError::InvalidRequest(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(CreateInstanceError::ResourceNotFound(err.msg))
                }
                "ServiceQuotaExceededException" => {
                    return RusotoError::Service(CreateInstanceError::ServiceQuotaExceeded(err.msg))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(CreateInstanceError::Throttling(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for CreateInstanceError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreateInstanceError::InternalService(ref cause) => write!(f, "{}", cause),
            CreateInstanceError::InvalidRequest(ref cause) => write!(f, "{}", cause),
            CreateInstanceError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            CreateInstanceError::ServiceQuotaExceeded(ref cause) => write!(f, "{}", cause),
            CreateInstanceError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CreateInstanceError {}
/// Errors returned by CreateIntegrationAssociation
#[derive(Debug, PartialEq)]
pub enum CreateIntegrationAssociationError {
    /// <p>A resource with the specified name already exists.</p>
    DuplicateResource(String),
    /// <p>Request processing failed due to an error or failure with the service.</p>
    InternalService(String),
    /// <p>The request is not valid.</p>
    InvalidRequest(String),
    /// <p>The specified resource was not found.</p>
    ResourceNotFound(String),
    /// <p>The throttling limit has been exceeded.</p>
    Throttling(String),
}

impl CreateIntegrationAssociationError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<CreateIntegrationAssociationError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "DuplicateResourceException" => {
                    return RusotoError::Service(
                        CreateIntegrationAssociationError::DuplicateResource(err.msg),
                    )
                }
                "InternalServiceException" => {
                    return RusotoError::Service(
                        CreateIntegrationAssociationError::InternalService(err.msg),
                    )
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(CreateIntegrationAssociationError::InvalidRequest(
                        err.msg,
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(
                        CreateIntegrationAssociationError::ResourceNotFound(err.msg),
                    )
                }
                "ThrottlingException" => {
                    return RusotoError::Service(CreateIntegrationAssociationError::Throttling(
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
impl fmt::Display for CreateIntegrationAssociationError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreateIntegrationAssociationError::DuplicateResource(ref cause) => {
                write!(f, "{}", cause)
            }
            CreateIntegrationAssociationError::InternalService(ref cause) => write!(f, "{}", cause),
            CreateIntegrationAssociationError::InvalidRequest(ref cause) => write!(f, "{}", cause),
            CreateIntegrationAssociationError::ResourceNotFound(ref cause) => {
                write!(f, "{}", cause)
            }
            CreateIntegrationAssociationError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CreateIntegrationAssociationError {}
/// Errors returned by CreateQuickConnect
#[derive(Debug, PartialEq)]
pub enum CreateQuickConnectError {
    /// <p>A resource with the specified name already exists.</p>
    DuplicateResource(String),
    /// <p>Request processing failed due to an error or failure with the service.</p>
    InternalService(String),
    /// <p>One or more of the specified parameters are not valid.</p>
    InvalidParameter(String),
    /// <p>The request is not valid.</p>
    InvalidRequest(String),
    /// <p>The allowed limit for the resource has been exceeded.</p>
    LimitExceeded(String),
    /// <p>The specified resource was not found.</p>
    ResourceNotFound(String),
    /// <p>The throttling limit has been exceeded.</p>
    Throttling(String),
}

impl CreateQuickConnectError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateQuickConnectError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "DuplicateResourceException" => {
                    return RusotoError::Service(CreateQuickConnectError::DuplicateResource(
                        err.msg,
                    ))
                }
                "InternalServiceException" => {
                    return RusotoError::Service(CreateQuickConnectError::InternalService(err.msg))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(CreateQuickConnectError::InvalidParameter(err.msg))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(CreateQuickConnectError::InvalidRequest(err.msg))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(CreateQuickConnectError::LimitExceeded(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(CreateQuickConnectError::ResourceNotFound(err.msg))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(CreateQuickConnectError::Throttling(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for CreateQuickConnectError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreateQuickConnectError::DuplicateResource(ref cause) => write!(f, "{}", cause),
            CreateQuickConnectError::InternalService(ref cause) => write!(f, "{}", cause),
            CreateQuickConnectError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            CreateQuickConnectError::InvalidRequest(ref cause) => write!(f, "{}", cause),
            CreateQuickConnectError::LimitExceeded(ref cause) => write!(f, "{}", cause),
            CreateQuickConnectError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            CreateQuickConnectError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CreateQuickConnectError {}
/// Errors returned by CreateRoutingProfile
#[derive(Debug, PartialEq)]
pub enum CreateRoutingProfileError {
    /// <p>A resource with the specified name already exists.</p>
    DuplicateResource(String),
    /// <p>Request processing failed due to an error or failure with the service.</p>
    InternalService(String),
    /// <p>One or more of the specified parameters are not valid.</p>
    InvalidParameter(String),
    /// <p>The request is not valid.</p>
    InvalidRequest(String),
    /// <p>The allowed limit for the resource has been exceeded.</p>
    LimitExceeded(String),
    /// <p>The specified resource was not found.</p>
    ResourceNotFound(String),
    /// <p>The throttling limit has been exceeded.</p>
    Throttling(String),
}

impl CreateRoutingProfileError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateRoutingProfileError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "DuplicateResourceException" => {
                    return RusotoError::Service(CreateRoutingProfileError::DuplicateResource(
                        err.msg,
                    ))
                }
                "InternalServiceException" => {
                    return RusotoError::Service(CreateRoutingProfileError::InternalService(
                        err.msg,
                    ))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(CreateRoutingProfileError::InvalidParameter(
                        err.msg,
                    ))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(CreateRoutingProfileError::InvalidRequest(err.msg))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(CreateRoutingProfileError::LimitExceeded(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(CreateRoutingProfileError::ResourceNotFound(
                        err.msg,
                    ))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(CreateRoutingProfileError::Throttling(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for CreateRoutingProfileError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreateRoutingProfileError::DuplicateResource(ref cause) => write!(f, "{}", cause),
            CreateRoutingProfileError::InternalService(ref cause) => write!(f, "{}", cause),
            CreateRoutingProfileError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            CreateRoutingProfileError::InvalidRequest(ref cause) => write!(f, "{}", cause),
            CreateRoutingProfileError::LimitExceeded(ref cause) => write!(f, "{}", cause),
            CreateRoutingProfileError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            CreateRoutingProfileError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CreateRoutingProfileError {}
/// Errors returned by CreateUseCase
#[derive(Debug, PartialEq)]
pub enum CreateUseCaseError {
    /// <p>A resource with the specified name already exists.</p>
    DuplicateResource(String),
    /// <p>Request processing failed due to an error or failure with the service.</p>
    InternalService(String),
    /// <p>The request is not valid.</p>
    InvalidRequest(String),
    /// <p>The specified resource was not found.</p>
    ResourceNotFound(String),
    /// <p>The throttling limit has been exceeded.</p>
    Throttling(String),
}

impl CreateUseCaseError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateUseCaseError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "DuplicateResourceException" => {
                    return RusotoError::Service(CreateUseCaseError::DuplicateResource(err.msg))
                }
                "InternalServiceException" => {
                    return RusotoError::Service(CreateUseCaseError::InternalService(err.msg))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(CreateUseCaseError::InvalidRequest(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(CreateUseCaseError::ResourceNotFound(err.msg))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(CreateUseCaseError::Throttling(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for CreateUseCaseError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreateUseCaseError::DuplicateResource(ref cause) => write!(f, "{}", cause),
            CreateUseCaseError::InternalService(ref cause) => write!(f, "{}", cause),
            CreateUseCaseError::InvalidRequest(ref cause) => write!(f, "{}", cause),
            CreateUseCaseError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            CreateUseCaseError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CreateUseCaseError {}
/// Errors returned by CreateUser
#[derive(Debug, PartialEq)]
pub enum CreateUserError {
    /// <p>A resource with the specified name already exists.</p>
    DuplicateResource(String),
    /// <p>Request processing failed due to an error or failure with the service.</p>
    InternalService(String),
    /// <p>One or more of the specified parameters are not valid.</p>
    InvalidParameter(String),
    /// <p>The request is not valid.</p>
    InvalidRequest(String),
    /// <p>The allowed limit for the resource has been exceeded.</p>
    LimitExceeded(String),
    /// <p>The specified resource was not found.</p>
    ResourceNotFound(String),
    /// <p>The throttling limit has been exceeded.</p>
    Throttling(String),
}

impl CreateUserError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateUserError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "DuplicateResourceException" => {
                    return RusotoError::Service(CreateUserError::DuplicateResource(err.msg))
                }
                "InternalServiceException" => {
                    return RusotoError::Service(CreateUserError::InternalService(err.msg))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(CreateUserError::InvalidParameter(err.msg))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(CreateUserError::InvalidRequest(err.msg))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(CreateUserError::LimitExceeded(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(CreateUserError::ResourceNotFound(err.msg))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(CreateUserError::Throttling(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for CreateUserError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreateUserError::DuplicateResource(ref cause) => write!(f, "{}", cause),
            CreateUserError::InternalService(ref cause) => write!(f, "{}", cause),
            CreateUserError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            CreateUserError::InvalidRequest(ref cause) => write!(f, "{}", cause),
            CreateUserError::LimitExceeded(ref cause) => write!(f, "{}", cause),
            CreateUserError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            CreateUserError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CreateUserError {}
/// Errors returned by CreateUserHierarchyGroup
#[derive(Debug, PartialEq)]
pub enum CreateUserHierarchyGroupError {
    /// <p>A resource with the specified name already exists.</p>
    DuplicateResource(String),
    /// <p>Request processing failed due to an error or failure with the service.</p>
    InternalService(String),
    /// <p>One or more of the specified parameters are not valid.</p>
    InvalidParameter(String),
    /// <p>The request is not valid.</p>
    InvalidRequest(String),
    /// <p>The allowed limit for the resource has been exceeded.</p>
    LimitExceeded(String),
    /// <p>The specified resource was not found.</p>
    ResourceNotFound(String),
    /// <p>The throttling limit has been exceeded.</p>
    Throttling(String),
}

impl CreateUserHierarchyGroupError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateUserHierarchyGroupError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "DuplicateResourceException" => {
                    return RusotoError::Service(CreateUserHierarchyGroupError::DuplicateResource(
                        err.msg,
                    ))
                }
                "InternalServiceException" => {
                    return RusotoError::Service(CreateUserHierarchyGroupError::InternalService(
                        err.msg,
                    ))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(CreateUserHierarchyGroupError::InvalidParameter(
                        err.msg,
                    ))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(CreateUserHierarchyGroupError::InvalidRequest(
                        err.msg,
                    ))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(CreateUserHierarchyGroupError::LimitExceeded(
                        err.msg,
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(CreateUserHierarchyGroupError::ResourceNotFound(
                        err.msg,
                    ))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(CreateUserHierarchyGroupError::Throttling(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for CreateUserHierarchyGroupError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreateUserHierarchyGroupError::DuplicateResource(ref cause) => write!(f, "{}", cause),
            CreateUserHierarchyGroupError::InternalService(ref cause) => write!(f, "{}", cause),
            CreateUserHierarchyGroupError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            CreateUserHierarchyGroupError::InvalidRequest(ref cause) => write!(f, "{}", cause),
            CreateUserHierarchyGroupError::LimitExceeded(ref cause) => write!(f, "{}", cause),
            CreateUserHierarchyGroupError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            CreateUserHierarchyGroupError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CreateUserHierarchyGroupError {}
/// Errors returned by DeleteInstance
#[derive(Debug, PartialEq)]
pub enum DeleteInstanceError {
    /// <p>Request processing failed due to an error or failure with the service.</p>
    InternalService(String),
    /// <p>The request is not valid.</p>
    InvalidRequest(String),
    /// <p>The specified resource was not found.</p>
    ResourceNotFound(String),
}

impl DeleteInstanceError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteInstanceError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InternalServiceException" => {
                    return RusotoError::Service(DeleteInstanceError::InternalService(err.msg))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(DeleteInstanceError::InvalidRequest(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(DeleteInstanceError::ResourceNotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DeleteInstanceError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteInstanceError::InternalService(ref cause) => write!(f, "{}", cause),
            DeleteInstanceError::InvalidRequest(ref cause) => write!(f, "{}", cause),
            DeleteInstanceError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeleteInstanceError {}
/// Errors returned by DeleteIntegrationAssociation
#[derive(Debug, PartialEq)]
pub enum DeleteIntegrationAssociationError {
    /// <p>Request processing failed due to an error or failure with the service.</p>
    InternalService(String),
    /// <p>The request is not valid.</p>
    InvalidRequest(String),
    /// <p>The specified resource was not found.</p>
    ResourceNotFound(String),
    /// <p>The throttling limit has been exceeded.</p>
    Throttling(String),
}

impl DeleteIntegrationAssociationError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<DeleteIntegrationAssociationError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InternalServiceException" => {
                    return RusotoError::Service(
                        DeleteIntegrationAssociationError::InternalService(err.msg),
                    )
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(DeleteIntegrationAssociationError::InvalidRequest(
                        err.msg,
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(
                        DeleteIntegrationAssociationError::ResourceNotFound(err.msg),
                    )
                }
                "ThrottlingException" => {
                    return RusotoError::Service(DeleteIntegrationAssociationError::Throttling(
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
impl fmt::Display for DeleteIntegrationAssociationError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteIntegrationAssociationError::InternalService(ref cause) => write!(f, "{}", cause),
            DeleteIntegrationAssociationError::InvalidRequest(ref cause) => write!(f, "{}", cause),
            DeleteIntegrationAssociationError::ResourceNotFound(ref cause) => {
                write!(f, "{}", cause)
            }
            DeleteIntegrationAssociationError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeleteIntegrationAssociationError {}
/// Errors returned by DeleteQuickConnect
#[derive(Debug, PartialEq)]
pub enum DeleteQuickConnectError {
    /// <p>Request processing failed due to an error or failure with the service.</p>
    InternalService(String),
    /// <p>One or more of the specified parameters are not valid.</p>
    InvalidParameter(String),
    /// <p>The request is not valid.</p>
    InvalidRequest(String),
    /// <p>The specified resource was not found.</p>
    ResourceNotFound(String),
    /// <p>The throttling limit has been exceeded.</p>
    Throttling(String),
}

impl DeleteQuickConnectError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteQuickConnectError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InternalServiceException" => {
                    return RusotoError::Service(DeleteQuickConnectError::InternalService(err.msg))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(DeleteQuickConnectError::InvalidParameter(err.msg))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(DeleteQuickConnectError::InvalidRequest(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(DeleteQuickConnectError::ResourceNotFound(err.msg))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(DeleteQuickConnectError::Throttling(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DeleteQuickConnectError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteQuickConnectError::InternalService(ref cause) => write!(f, "{}", cause),
            DeleteQuickConnectError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            DeleteQuickConnectError::InvalidRequest(ref cause) => write!(f, "{}", cause),
            DeleteQuickConnectError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            DeleteQuickConnectError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeleteQuickConnectError {}
/// Errors returned by DeleteUseCase
#[derive(Debug, PartialEq)]
pub enum DeleteUseCaseError {
    /// <p>Request processing failed due to an error or failure with the service.</p>
    InternalService(String),
    /// <p>The request is not valid.</p>
    InvalidRequest(String),
    /// <p>The specified resource was not found.</p>
    ResourceNotFound(String),
    /// <p>The throttling limit has been exceeded.</p>
    Throttling(String),
}

impl DeleteUseCaseError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteUseCaseError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InternalServiceException" => {
                    return RusotoError::Service(DeleteUseCaseError::InternalService(err.msg))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(DeleteUseCaseError::InvalidRequest(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(DeleteUseCaseError::ResourceNotFound(err.msg))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(DeleteUseCaseError::Throttling(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DeleteUseCaseError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteUseCaseError::InternalService(ref cause) => write!(f, "{}", cause),
            DeleteUseCaseError::InvalidRequest(ref cause) => write!(f, "{}", cause),
            DeleteUseCaseError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            DeleteUseCaseError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeleteUseCaseError {}
/// Errors returned by DeleteUser
#[derive(Debug, PartialEq)]
pub enum DeleteUserError {
    /// <p>Request processing failed due to an error or failure with the service.</p>
    InternalService(String),
    /// <p>One or more of the specified parameters are not valid.</p>
    InvalidParameter(String),
    /// <p>The request is not valid.</p>
    InvalidRequest(String),
    /// <p>The specified resource was not found.</p>
    ResourceNotFound(String),
    /// <p>The throttling limit has been exceeded.</p>
    Throttling(String),
}

impl DeleteUserError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteUserError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InternalServiceException" => {
                    return RusotoError::Service(DeleteUserError::InternalService(err.msg))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(DeleteUserError::InvalidParameter(err.msg))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(DeleteUserError::InvalidRequest(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(DeleteUserError::ResourceNotFound(err.msg))
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
            DeleteUserError::InternalService(ref cause) => write!(f, "{}", cause),
            DeleteUserError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            DeleteUserError::InvalidRequest(ref cause) => write!(f, "{}", cause),
            DeleteUserError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            DeleteUserError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeleteUserError {}
/// Errors returned by DeleteUserHierarchyGroup
#[derive(Debug, PartialEq)]
pub enum DeleteUserHierarchyGroupError {
    /// <p>Request processing failed due to an error or failure with the service.</p>
    InternalService(String),
    /// <p>One or more of the specified parameters are not valid.</p>
    InvalidParameter(String),
    /// <p>The request is not valid.</p>
    InvalidRequest(String),
    /// <p>That resource is already in use. Please try another.</p>
    ResourceInUse(String),
    /// <p>The specified resource was not found.</p>
    ResourceNotFound(String),
    /// <p>The throttling limit has been exceeded.</p>
    Throttling(String),
}

impl DeleteUserHierarchyGroupError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteUserHierarchyGroupError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InternalServiceException" => {
                    return RusotoError::Service(DeleteUserHierarchyGroupError::InternalService(
                        err.msg,
                    ))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(DeleteUserHierarchyGroupError::InvalidParameter(
                        err.msg,
                    ))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(DeleteUserHierarchyGroupError::InvalidRequest(
                        err.msg,
                    ))
                }
                "ResourceInUseException" => {
                    return RusotoError::Service(DeleteUserHierarchyGroupError::ResourceInUse(
                        err.msg,
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(DeleteUserHierarchyGroupError::ResourceNotFound(
                        err.msg,
                    ))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(DeleteUserHierarchyGroupError::Throttling(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DeleteUserHierarchyGroupError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteUserHierarchyGroupError::InternalService(ref cause) => write!(f, "{}", cause),
            DeleteUserHierarchyGroupError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            DeleteUserHierarchyGroupError::InvalidRequest(ref cause) => write!(f, "{}", cause),
            DeleteUserHierarchyGroupError::ResourceInUse(ref cause) => write!(f, "{}", cause),
            DeleteUserHierarchyGroupError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            DeleteUserHierarchyGroupError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeleteUserHierarchyGroupError {}
/// Errors returned by DescribeContactFlow
#[derive(Debug, PartialEq)]
pub enum DescribeContactFlowError {
    /// <p>The contact flow has not been published.</p>
    ContactFlowNotPublished(String),
    /// <p>Request processing failed due to an error or failure with the service.</p>
    InternalService(String),
    /// <p>One or more of the specified parameters are not valid.</p>
    InvalidParameter(String),
    /// <p>The request is not valid.</p>
    InvalidRequest(String),
    /// <p>The specified resource was not found.</p>
    ResourceNotFound(String),
    /// <p>The throttling limit has been exceeded.</p>
    Throttling(String),
}

impl DescribeContactFlowError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeContactFlowError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "ContactFlowNotPublishedException" => {
                    return RusotoError::Service(DescribeContactFlowError::ContactFlowNotPublished(
                        err.msg,
                    ))
                }
                "InternalServiceException" => {
                    return RusotoError::Service(DescribeContactFlowError::InternalService(err.msg))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(DescribeContactFlowError::InvalidParameter(
                        err.msg,
                    ))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(DescribeContactFlowError::InvalidRequest(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(DescribeContactFlowError::ResourceNotFound(
                        err.msg,
                    ))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(DescribeContactFlowError::Throttling(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DescribeContactFlowError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeContactFlowError::ContactFlowNotPublished(ref cause) => write!(f, "{}", cause),
            DescribeContactFlowError::InternalService(ref cause) => write!(f, "{}", cause),
            DescribeContactFlowError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            DescribeContactFlowError::InvalidRequest(ref cause) => write!(f, "{}", cause),
            DescribeContactFlowError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            DescribeContactFlowError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DescribeContactFlowError {}
/// Errors returned by DescribeInstance
#[derive(Debug, PartialEq)]
pub enum DescribeInstanceError {
    /// <p>Request processing failed due to an error or failure with the service.</p>
    InternalService(String),
    /// <p>The request is not valid.</p>
    InvalidRequest(String),
    /// <p>The specified resource was not found.</p>
    ResourceNotFound(String),
}

impl DescribeInstanceError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeInstanceError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InternalServiceException" => {
                    return RusotoError::Service(DescribeInstanceError::InternalService(err.msg))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(DescribeInstanceError::InvalidRequest(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(DescribeInstanceError::ResourceNotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DescribeInstanceError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeInstanceError::InternalService(ref cause) => write!(f, "{}", cause),
            DescribeInstanceError::InvalidRequest(ref cause) => write!(f, "{}", cause),
            DescribeInstanceError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DescribeInstanceError {}
/// Errors returned by DescribeInstanceAttribute
#[derive(Debug, PartialEq)]
pub enum DescribeInstanceAttributeError {
    /// <p>Request processing failed due to an error or failure with the service.</p>
    InternalService(String),
    /// <p>One or more of the specified parameters are not valid.</p>
    InvalidParameter(String),
    /// <p>The request is not valid.</p>
    InvalidRequest(String),
    /// <p>The specified resource was not found.</p>
    ResourceNotFound(String),
    /// <p>The throttling limit has been exceeded.</p>
    Throttling(String),
}

impl DescribeInstanceAttributeError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeInstanceAttributeError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InternalServiceException" => {
                    return RusotoError::Service(DescribeInstanceAttributeError::InternalService(
                        err.msg,
                    ))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(DescribeInstanceAttributeError::InvalidParameter(
                        err.msg,
                    ))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(DescribeInstanceAttributeError::InvalidRequest(
                        err.msg,
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(DescribeInstanceAttributeError::ResourceNotFound(
                        err.msg,
                    ))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(DescribeInstanceAttributeError::Throttling(
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
impl fmt::Display for DescribeInstanceAttributeError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeInstanceAttributeError::InternalService(ref cause) => write!(f, "{}", cause),
            DescribeInstanceAttributeError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            DescribeInstanceAttributeError::InvalidRequest(ref cause) => write!(f, "{}", cause),
            DescribeInstanceAttributeError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            DescribeInstanceAttributeError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DescribeInstanceAttributeError {}
/// Errors returned by DescribeInstanceStorageConfig
#[derive(Debug, PartialEq)]
pub enum DescribeInstanceStorageConfigError {
    /// <p>Request processing failed due to an error or failure with the service.</p>
    InternalService(String),
    /// <p>One or more of the specified parameters are not valid.</p>
    InvalidParameter(String),
    /// <p>The request is not valid.</p>
    InvalidRequest(String),
    /// <p>The specified resource was not found.</p>
    ResourceNotFound(String),
    /// <p>The throttling limit has been exceeded.</p>
    Throttling(String),
}

impl DescribeInstanceStorageConfigError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<DescribeInstanceStorageConfigError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InternalServiceException" => {
                    return RusotoError::Service(
                        DescribeInstanceStorageConfigError::InternalService(err.msg),
                    )
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(
                        DescribeInstanceStorageConfigError::InvalidParameter(err.msg),
                    )
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(
                        DescribeInstanceStorageConfigError::InvalidRequest(err.msg),
                    )
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(
                        DescribeInstanceStorageConfigError::ResourceNotFound(err.msg),
                    )
                }
                "ThrottlingException" => {
                    return RusotoError::Service(DescribeInstanceStorageConfigError::Throttling(
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
impl fmt::Display for DescribeInstanceStorageConfigError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeInstanceStorageConfigError::InternalService(ref cause) => {
                write!(f, "{}", cause)
            }
            DescribeInstanceStorageConfigError::InvalidParameter(ref cause) => {
                write!(f, "{}", cause)
            }
            DescribeInstanceStorageConfigError::InvalidRequest(ref cause) => write!(f, "{}", cause),
            DescribeInstanceStorageConfigError::ResourceNotFound(ref cause) => {
                write!(f, "{}", cause)
            }
            DescribeInstanceStorageConfigError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DescribeInstanceStorageConfigError {}
/// Errors returned by DescribeQuickConnect
#[derive(Debug, PartialEq)]
pub enum DescribeQuickConnectError {
    /// <p>Request processing failed due to an error or failure with the service.</p>
    InternalService(String),
    /// <p>One or more of the specified parameters are not valid.</p>
    InvalidParameter(String),
    /// <p>The request is not valid.</p>
    InvalidRequest(String),
    /// <p>The specified resource was not found.</p>
    ResourceNotFound(String),
    /// <p>The throttling limit has been exceeded.</p>
    Throttling(String),
}

impl DescribeQuickConnectError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeQuickConnectError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InternalServiceException" => {
                    return RusotoError::Service(DescribeQuickConnectError::InternalService(
                        err.msg,
                    ))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(DescribeQuickConnectError::InvalidParameter(
                        err.msg,
                    ))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(DescribeQuickConnectError::InvalidRequest(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(DescribeQuickConnectError::ResourceNotFound(
                        err.msg,
                    ))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(DescribeQuickConnectError::Throttling(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DescribeQuickConnectError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeQuickConnectError::InternalService(ref cause) => write!(f, "{}", cause),
            DescribeQuickConnectError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            DescribeQuickConnectError::InvalidRequest(ref cause) => write!(f, "{}", cause),
            DescribeQuickConnectError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            DescribeQuickConnectError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DescribeQuickConnectError {}
/// Errors returned by DescribeRoutingProfile
#[derive(Debug, PartialEq)]
pub enum DescribeRoutingProfileError {
    /// <p>Request processing failed due to an error or failure with the service.</p>
    InternalService(String),
    /// <p>One or more of the specified parameters are not valid.</p>
    InvalidParameter(String),
    /// <p>The request is not valid.</p>
    InvalidRequest(String),
    /// <p>The specified resource was not found.</p>
    ResourceNotFound(String),
    /// <p>The throttling limit has been exceeded.</p>
    Throttling(String),
}

impl DescribeRoutingProfileError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeRoutingProfileError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InternalServiceException" => {
                    return RusotoError::Service(DescribeRoutingProfileError::InternalService(
                        err.msg,
                    ))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(DescribeRoutingProfileError::InvalidParameter(
                        err.msg,
                    ))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(DescribeRoutingProfileError::InvalidRequest(
                        err.msg,
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(DescribeRoutingProfileError::ResourceNotFound(
                        err.msg,
                    ))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(DescribeRoutingProfileError::Throttling(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DescribeRoutingProfileError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeRoutingProfileError::InternalService(ref cause) => write!(f, "{}", cause),
            DescribeRoutingProfileError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            DescribeRoutingProfileError::InvalidRequest(ref cause) => write!(f, "{}", cause),
            DescribeRoutingProfileError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            DescribeRoutingProfileError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DescribeRoutingProfileError {}
/// Errors returned by DescribeUser
#[derive(Debug, PartialEq)]
pub enum DescribeUserError {
    /// <p>Request processing failed due to an error or failure with the service.</p>
    InternalService(String),
    /// <p>One or more of the specified parameters are not valid.</p>
    InvalidParameter(String),
    /// <p>The request is not valid.</p>
    InvalidRequest(String),
    /// <p>The specified resource was not found.</p>
    ResourceNotFound(String),
    /// <p>The throttling limit has been exceeded.</p>
    Throttling(String),
}

impl DescribeUserError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeUserError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InternalServiceException" => {
                    return RusotoError::Service(DescribeUserError::InternalService(err.msg))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(DescribeUserError::InvalidParameter(err.msg))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(DescribeUserError::InvalidRequest(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(DescribeUserError::ResourceNotFound(err.msg))
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
            DescribeUserError::InternalService(ref cause) => write!(f, "{}", cause),
            DescribeUserError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            DescribeUserError::InvalidRequest(ref cause) => write!(f, "{}", cause),
            DescribeUserError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            DescribeUserError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DescribeUserError {}
/// Errors returned by DescribeUserHierarchyGroup
#[derive(Debug, PartialEq)]
pub enum DescribeUserHierarchyGroupError {
    /// <p>Request processing failed due to an error or failure with the service.</p>
    InternalService(String),
    /// <p>One or more of the specified parameters are not valid.</p>
    InvalidParameter(String),
    /// <p>The request is not valid.</p>
    InvalidRequest(String),
    /// <p>The specified resource was not found.</p>
    ResourceNotFound(String),
    /// <p>The throttling limit has been exceeded.</p>
    Throttling(String),
}

impl DescribeUserHierarchyGroupError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<DescribeUserHierarchyGroupError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InternalServiceException" => {
                    return RusotoError::Service(DescribeUserHierarchyGroupError::InternalService(
                        err.msg,
                    ))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(DescribeUserHierarchyGroupError::InvalidParameter(
                        err.msg,
                    ))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(DescribeUserHierarchyGroupError::InvalidRequest(
                        err.msg,
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(DescribeUserHierarchyGroupError::ResourceNotFound(
                        err.msg,
                    ))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(DescribeUserHierarchyGroupError::Throttling(
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
impl fmt::Display for DescribeUserHierarchyGroupError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeUserHierarchyGroupError::InternalService(ref cause) => write!(f, "{}", cause),
            DescribeUserHierarchyGroupError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            DescribeUserHierarchyGroupError::InvalidRequest(ref cause) => write!(f, "{}", cause),
            DescribeUserHierarchyGroupError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            DescribeUserHierarchyGroupError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DescribeUserHierarchyGroupError {}
/// Errors returned by DescribeUserHierarchyStructure
#[derive(Debug, PartialEq)]
pub enum DescribeUserHierarchyStructureError {
    /// <p>Request processing failed due to an error or failure with the service.</p>
    InternalService(String),
    /// <p>One or more of the specified parameters are not valid.</p>
    InvalidParameter(String),
    /// <p>The request is not valid.</p>
    InvalidRequest(String),
    /// <p>The specified resource was not found.</p>
    ResourceNotFound(String),
    /// <p>The throttling limit has been exceeded.</p>
    Throttling(String),
}

impl DescribeUserHierarchyStructureError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<DescribeUserHierarchyStructureError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InternalServiceException" => {
                    return RusotoError::Service(
                        DescribeUserHierarchyStructureError::InternalService(err.msg),
                    )
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(
                        DescribeUserHierarchyStructureError::InvalidParameter(err.msg),
                    )
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(
                        DescribeUserHierarchyStructureError::InvalidRequest(err.msg),
                    )
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(
                        DescribeUserHierarchyStructureError::ResourceNotFound(err.msg),
                    )
                }
                "ThrottlingException" => {
                    return RusotoError::Service(DescribeUserHierarchyStructureError::Throttling(
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
impl fmt::Display for DescribeUserHierarchyStructureError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeUserHierarchyStructureError::InternalService(ref cause) => {
                write!(f, "{}", cause)
            }
            DescribeUserHierarchyStructureError::InvalidParameter(ref cause) => {
                write!(f, "{}", cause)
            }
            DescribeUserHierarchyStructureError::InvalidRequest(ref cause) => {
                write!(f, "{}", cause)
            }
            DescribeUserHierarchyStructureError::ResourceNotFound(ref cause) => {
                write!(f, "{}", cause)
            }
            DescribeUserHierarchyStructureError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DescribeUserHierarchyStructureError {}
/// Errors returned by DisassociateApprovedOrigin
#[derive(Debug, PartialEq)]
pub enum DisassociateApprovedOriginError {
    /// <p>Request processing failed due to an error or failure with the service.</p>
    InternalService(String),
    /// <p>One or more of the specified parameters are not valid.</p>
    InvalidParameter(String),
    /// <p>The request is not valid.</p>
    InvalidRequest(String),
    /// <p>The specified resource was not found.</p>
    ResourceNotFound(String),
    /// <p>The throttling limit has been exceeded.</p>
    Throttling(String),
}

impl DisassociateApprovedOriginError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<DisassociateApprovedOriginError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InternalServiceException" => {
                    return RusotoError::Service(DisassociateApprovedOriginError::InternalService(
                        err.msg,
                    ))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(DisassociateApprovedOriginError::InvalidParameter(
                        err.msg,
                    ))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(DisassociateApprovedOriginError::InvalidRequest(
                        err.msg,
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(DisassociateApprovedOriginError::ResourceNotFound(
                        err.msg,
                    ))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(DisassociateApprovedOriginError::Throttling(
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
impl fmt::Display for DisassociateApprovedOriginError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DisassociateApprovedOriginError::InternalService(ref cause) => write!(f, "{}", cause),
            DisassociateApprovedOriginError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            DisassociateApprovedOriginError::InvalidRequest(ref cause) => write!(f, "{}", cause),
            DisassociateApprovedOriginError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            DisassociateApprovedOriginError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DisassociateApprovedOriginError {}
/// Errors returned by DisassociateInstanceStorageConfig
#[derive(Debug, PartialEq)]
pub enum DisassociateInstanceStorageConfigError {
    /// <p>Request processing failed due to an error or failure with the service.</p>
    InternalService(String),
    /// <p>One or more of the specified parameters are not valid.</p>
    InvalidParameter(String),
    /// <p>The request is not valid.</p>
    InvalidRequest(String),
    /// <p>The specified resource was not found.</p>
    ResourceNotFound(String),
    /// <p>The throttling limit has been exceeded.</p>
    Throttling(String),
}

impl DisassociateInstanceStorageConfigError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<DisassociateInstanceStorageConfigError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InternalServiceException" => {
                    return RusotoError::Service(
                        DisassociateInstanceStorageConfigError::InternalService(err.msg),
                    )
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(
                        DisassociateInstanceStorageConfigError::InvalidParameter(err.msg),
                    )
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(
                        DisassociateInstanceStorageConfigError::InvalidRequest(err.msg),
                    )
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(
                        DisassociateInstanceStorageConfigError::ResourceNotFound(err.msg),
                    )
                }
                "ThrottlingException" => {
                    return RusotoError::Service(
                        DisassociateInstanceStorageConfigError::Throttling(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DisassociateInstanceStorageConfigError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DisassociateInstanceStorageConfigError::InternalService(ref cause) => {
                write!(f, "{}", cause)
            }
            DisassociateInstanceStorageConfigError::InvalidParameter(ref cause) => {
                write!(f, "{}", cause)
            }
            DisassociateInstanceStorageConfigError::InvalidRequest(ref cause) => {
                write!(f, "{}", cause)
            }
            DisassociateInstanceStorageConfigError::ResourceNotFound(ref cause) => {
                write!(f, "{}", cause)
            }
            DisassociateInstanceStorageConfigError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DisassociateInstanceStorageConfigError {}
/// Errors returned by DisassociateLambdaFunction
#[derive(Debug, PartialEq)]
pub enum DisassociateLambdaFunctionError {
    /// <p>Request processing failed due to an error or failure with the service.</p>
    InternalService(String),
    /// <p>One or more of the specified parameters are not valid.</p>
    InvalidParameter(String),
    /// <p>The request is not valid.</p>
    InvalidRequest(String),
    /// <p>The specified resource was not found.</p>
    ResourceNotFound(String),
    /// <p>The throttling limit has been exceeded.</p>
    Throttling(String),
}

impl DisassociateLambdaFunctionError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<DisassociateLambdaFunctionError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InternalServiceException" => {
                    return RusotoError::Service(DisassociateLambdaFunctionError::InternalService(
                        err.msg,
                    ))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(DisassociateLambdaFunctionError::InvalidParameter(
                        err.msg,
                    ))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(DisassociateLambdaFunctionError::InvalidRequest(
                        err.msg,
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(DisassociateLambdaFunctionError::ResourceNotFound(
                        err.msg,
                    ))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(DisassociateLambdaFunctionError::Throttling(
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
impl fmt::Display for DisassociateLambdaFunctionError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DisassociateLambdaFunctionError::InternalService(ref cause) => write!(f, "{}", cause),
            DisassociateLambdaFunctionError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            DisassociateLambdaFunctionError::InvalidRequest(ref cause) => write!(f, "{}", cause),
            DisassociateLambdaFunctionError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            DisassociateLambdaFunctionError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DisassociateLambdaFunctionError {}
/// Errors returned by DisassociateLexBot
#[derive(Debug, PartialEq)]
pub enum DisassociateLexBotError {
    /// <p>Request processing failed due to an error or failure with the service.</p>
    InternalService(String),
    /// <p>One or more of the specified parameters are not valid.</p>
    InvalidParameter(String),
    /// <p>The request is not valid.</p>
    InvalidRequest(String),
    /// <p>The specified resource was not found.</p>
    ResourceNotFound(String),
    /// <p>The throttling limit has been exceeded.</p>
    Throttling(String),
}

impl DisassociateLexBotError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DisassociateLexBotError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InternalServiceException" => {
                    return RusotoError::Service(DisassociateLexBotError::InternalService(err.msg))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(DisassociateLexBotError::InvalidParameter(err.msg))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(DisassociateLexBotError::InvalidRequest(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(DisassociateLexBotError::ResourceNotFound(err.msg))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(DisassociateLexBotError::Throttling(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DisassociateLexBotError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DisassociateLexBotError::InternalService(ref cause) => write!(f, "{}", cause),
            DisassociateLexBotError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            DisassociateLexBotError::InvalidRequest(ref cause) => write!(f, "{}", cause),
            DisassociateLexBotError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            DisassociateLexBotError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DisassociateLexBotError {}
/// Errors returned by DisassociateRoutingProfileQueues
#[derive(Debug, PartialEq)]
pub enum DisassociateRoutingProfileQueuesError {
    /// <p>Request processing failed due to an error or failure with the service.</p>
    InternalService(String),
    /// <p>One or more of the specified parameters are not valid.</p>
    InvalidParameter(String),
    /// <p>The request is not valid.</p>
    InvalidRequest(String),
    /// <p>The specified resource was not found.</p>
    ResourceNotFound(String),
    /// <p>The throttling limit has been exceeded.</p>
    Throttling(String),
}

impl DisassociateRoutingProfileQueuesError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<DisassociateRoutingProfileQueuesError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InternalServiceException" => {
                    return RusotoError::Service(
                        DisassociateRoutingProfileQueuesError::InternalService(err.msg),
                    )
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(
                        DisassociateRoutingProfileQueuesError::InvalidParameter(err.msg),
                    )
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(
                        DisassociateRoutingProfileQueuesError::InvalidRequest(err.msg),
                    )
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(
                        DisassociateRoutingProfileQueuesError::ResourceNotFound(err.msg),
                    )
                }
                "ThrottlingException" => {
                    return RusotoError::Service(DisassociateRoutingProfileQueuesError::Throttling(
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
impl fmt::Display for DisassociateRoutingProfileQueuesError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DisassociateRoutingProfileQueuesError::InternalService(ref cause) => {
                write!(f, "{}", cause)
            }
            DisassociateRoutingProfileQueuesError::InvalidParameter(ref cause) => {
                write!(f, "{}", cause)
            }
            DisassociateRoutingProfileQueuesError::InvalidRequest(ref cause) => {
                write!(f, "{}", cause)
            }
            DisassociateRoutingProfileQueuesError::ResourceNotFound(ref cause) => {
                write!(f, "{}", cause)
            }
            DisassociateRoutingProfileQueuesError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DisassociateRoutingProfileQueuesError {}
/// Errors returned by DisassociateSecurityKey
#[derive(Debug, PartialEq)]
pub enum DisassociateSecurityKeyError {
    /// <p>Request processing failed due to an error or failure with the service.</p>
    InternalService(String),
    /// <p>One or more of the specified parameters are not valid.</p>
    InvalidParameter(String),
    /// <p>The request is not valid.</p>
    InvalidRequest(String),
    /// <p>The specified resource was not found.</p>
    ResourceNotFound(String),
    /// <p>The throttling limit has been exceeded.</p>
    Throttling(String),
}

impl DisassociateSecurityKeyError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DisassociateSecurityKeyError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InternalServiceException" => {
                    return RusotoError::Service(DisassociateSecurityKeyError::InternalService(
                        err.msg,
                    ))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(DisassociateSecurityKeyError::InvalidParameter(
                        err.msg,
                    ))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(DisassociateSecurityKeyError::InvalidRequest(
                        err.msg,
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(DisassociateSecurityKeyError::ResourceNotFound(
                        err.msg,
                    ))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(DisassociateSecurityKeyError::Throttling(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DisassociateSecurityKeyError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DisassociateSecurityKeyError::InternalService(ref cause) => write!(f, "{}", cause),
            DisassociateSecurityKeyError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            DisassociateSecurityKeyError::InvalidRequest(ref cause) => write!(f, "{}", cause),
            DisassociateSecurityKeyError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            DisassociateSecurityKeyError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DisassociateSecurityKeyError {}
/// Errors returned by GetContactAttributes
#[derive(Debug, PartialEq)]
pub enum GetContactAttributesError {
    /// <p>Request processing failed due to an error or failure with the service.</p>
    InternalService(String),
    /// <p>The request is not valid.</p>
    InvalidRequest(String),
    /// <p>The specified resource was not found.</p>
    ResourceNotFound(String),
}

impl GetContactAttributesError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetContactAttributesError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InternalServiceException" => {
                    return RusotoError::Service(GetContactAttributesError::InternalService(
                        err.msg,
                    ))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(GetContactAttributesError::InvalidRequest(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(GetContactAttributesError::ResourceNotFound(
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
impl fmt::Display for GetContactAttributesError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetContactAttributesError::InternalService(ref cause) => write!(f, "{}", cause),
            GetContactAttributesError::InvalidRequest(ref cause) => write!(f, "{}", cause),
            GetContactAttributesError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetContactAttributesError {}
/// Errors returned by GetCurrentMetricData
#[derive(Debug, PartialEq)]
pub enum GetCurrentMetricDataError {
    /// <p>Request processing failed due to an error or failure with the service.</p>
    InternalService(String),
    /// <p>One or more of the specified parameters are not valid.</p>
    InvalidParameter(String),
    /// <p>The request is not valid.</p>
    InvalidRequest(String),
    /// <p>The specified resource was not found.</p>
    ResourceNotFound(String),
    /// <p>The throttling limit has been exceeded.</p>
    Throttling(String),
}

impl GetCurrentMetricDataError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetCurrentMetricDataError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InternalServiceException" => {
                    return RusotoError::Service(GetCurrentMetricDataError::InternalService(
                        err.msg,
                    ))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(GetCurrentMetricDataError::InvalidParameter(
                        err.msg,
                    ))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(GetCurrentMetricDataError::InvalidRequest(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(GetCurrentMetricDataError::ResourceNotFound(
                        err.msg,
                    ))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(GetCurrentMetricDataError::Throttling(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for GetCurrentMetricDataError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetCurrentMetricDataError::InternalService(ref cause) => write!(f, "{}", cause),
            GetCurrentMetricDataError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            GetCurrentMetricDataError::InvalidRequest(ref cause) => write!(f, "{}", cause),
            GetCurrentMetricDataError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            GetCurrentMetricDataError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetCurrentMetricDataError {}
/// Errors returned by GetFederationToken
#[derive(Debug, PartialEq)]
pub enum GetFederationTokenError {
    /// <p>A resource with the specified name already exists.</p>
    DuplicateResource(String),
    /// <p>Request processing failed due to an error or failure with the service.</p>
    InternalService(String),
    /// <p>One or more of the specified parameters are not valid.</p>
    InvalidParameter(String),
    /// <p>The request is not valid.</p>
    InvalidRequest(String),
    /// <p>The specified resource was not found.</p>
    ResourceNotFound(String),
    /// <p>No user with the specified credentials was found in the Amazon Connect instance.</p>
    UserNotFound(String),
}

impl GetFederationTokenError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetFederationTokenError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "DuplicateResourceException" => {
                    return RusotoError::Service(GetFederationTokenError::DuplicateResource(
                        err.msg,
                    ))
                }
                "InternalServiceException" => {
                    return RusotoError::Service(GetFederationTokenError::InternalService(err.msg))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(GetFederationTokenError::InvalidParameter(err.msg))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(GetFederationTokenError::InvalidRequest(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(GetFederationTokenError::ResourceNotFound(err.msg))
                }
                "UserNotFoundException" => {
                    return RusotoError::Service(GetFederationTokenError::UserNotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for GetFederationTokenError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetFederationTokenError::DuplicateResource(ref cause) => write!(f, "{}", cause),
            GetFederationTokenError::InternalService(ref cause) => write!(f, "{}", cause),
            GetFederationTokenError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            GetFederationTokenError::InvalidRequest(ref cause) => write!(f, "{}", cause),
            GetFederationTokenError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            GetFederationTokenError::UserNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetFederationTokenError {}
/// Errors returned by GetMetricData
#[derive(Debug, PartialEq)]
pub enum GetMetricDataError {
    /// <p>Request processing failed due to an error or failure with the service.</p>
    InternalService(String),
    /// <p>One or more of the specified parameters are not valid.</p>
    InvalidParameter(String),
    /// <p>The request is not valid.</p>
    InvalidRequest(String),
    /// <p>The specified resource was not found.</p>
    ResourceNotFound(String),
    /// <p>The throttling limit has been exceeded.</p>
    Throttling(String),
}

impl GetMetricDataError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetMetricDataError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InternalServiceException" => {
                    return RusotoError::Service(GetMetricDataError::InternalService(err.msg))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(GetMetricDataError::InvalidParameter(err.msg))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(GetMetricDataError::InvalidRequest(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(GetMetricDataError::ResourceNotFound(err.msg))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(GetMetricDataError::Throttling(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for GetMetricDataError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetMetricDataError::InternalService(ref cause) => write!(f, "{}", cause),
            GetMetricDataError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            GetMetricDataError::InvalidRequest(ref cause) => write!(f, "{}", cause),
            GetMetricDataError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            GetMetricDataError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetMetricDataError {}
/// Errors returned by ListApprovedOrigins
#[derive(Debug, PartialEq)]
pub enum ListApprovedOriginsError {
    /// <p>Request processing failed due to an error or failure with the service.</p>
    InternalService(String),
    /// <p>One or more of the specified parameters are not valid.</p>
    InvalidParameter(String),
    /// <p>The request is not valid.</p>
    InvalidRequest(String),
    /// <p>The specified resource was not found.</p>
    ResourceNotFound(String),
    /// <p>The throttling limit has been exceeded.</p>
    Throttling(String),
}

impl ListApprovedOriginsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListApprovedOriginsError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InternalServiceException" => {
                    return RusotoError::Service(ListApprovedOriginsError::InternalService(err.msg))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(ListApprovedOriginsError::InvalidParameter(
                        err.msg,
                    ))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(ListApprovedOriginsError::InvalidRequest(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(ListApprovedOriginsError::ResourceNotFound(
                        err.msg,
                    ))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(ListApprovedOriginsError::Throttling(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ListApprovedOriginsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListApprovedOriginsError::InternalService(ref cause) => write!(f, "{}", cause),
            ListApprovedOriginsError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            ListApprovedOriginsError::InvalidRequest(ref cause) => write!(f, "{}", cause),
            ListApprovedOriginsError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            ListApprovedOriginsError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListApprovedOriginsError {}
/// Errors returned by ListContactFlows
#[derive(Debug, PartialEq)]
pub enum ListContactFlowsError {
    /// <p>Request processing failed due to an error or failure with the service.</p>
    InternalService(String),
    /// <p>One or more of the specified parameters are not valid.</p>
    InvalidParameter(String),
    /// <p>The request is not valid.</p>
    InvalidRequest(String),
    /// <p>The specified resource was not found.</p>
    ResourceNotFound(String),
    /// <p>The throttling limit has been exceeded.</p>
    Throttling(String),
}

impl ListContactFlowsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListContactFlowsError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InternalServiceException" => {
                    return RusotoError::Service(ListContactFlowsError::InternalService(err.msg))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(ListContactFlowsError::InvalidParameter(err.msg))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(ListContactFlowsError::InvalidRequest(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(ListContactFlowsError::ResourceNotFound(err.msg))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(ListContactFlowsError::Throttling(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ListContactFlowsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListContactFlowsError::InternalService(ref cause) => write!(f, "{}", cause),
            ListContactFlowsError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            ListContactFlowsError::InvalidRequest(ref cause) => write!(f, "{}", cause),
            ListContactFlowsError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            ListContactFlowsError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListContactFlowsError {}
/// Errors returned by ListHoursOfOperations
#[derive(Debug, PartialEq)]
pub enum ListHoursOfOperationsError {
    /// <p>Request processing failed due to an error or failure with the service.</p>
    InternalService(String),
    /// <p>One or more of the specified parameters are not valid.</p>
    InvalidParameter(String),
    /// <p>The request is not valid.</p>
    InvalidRequest(String),
    /// <p>The specified resource was not found.</p>
    ResourceNotFound(String),
    /// <p>The throttling limit has been exceeded.</p>
    Throttling(String),
}

impl ListHoursOfOperationsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListHoursOfOperationsError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InternalServiceException" => {
                    return RusotoError::Service(ListHoursOfOperationsError::InternalService(
                        err.msg,
                    ))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(ListHoursOfOperationsError::InvalidParameter(
                        err.msg,
                    ))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(ListHoursOfOperationsError::InvalidRequest(
                        err.msg,
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(ListHoursOfOperationsError::ResourceNotFound(
                        err.msg,
                    ))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(ListHoursOfOperationsError::Throttling(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ListHoursOfOperationsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListHoursOfOperationsError::InternalService(ref cause) => write!(f, "{}", cause),
            ListHoursOfOperationsError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            ListHoursOfOperationsError::InvalidRequest(ref cause) => write!(f, "{}", cause),
            ListHoursOfOperationsError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            ListHoursOfOperationsError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListHoursOfOperationsError {}
/// Errors returned by ListInstanceAttributes
#[derive(Debug, PartialEq)]
pub enum ListInstanceAttributesError {
    /// <p>Request processing failed due to an error or failure with the service.</p>
    InternalService(String),
    /// <p>One or more of the specified parameters are not valid.</p>
    InvalidParameter(String),
    /// <p>The request is not valid.</p>
    InvalidRequest(String),
    /// <p>The specified resource was not found.</p>
    ResourceNotFound(String),
    /// <p>The throttling limit has been exceeded.</p>
    Throttling(String),
}

impl ListInstanceAttributesError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListInstanceAttributesError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InternalServiceException" => {
                    return RusotoError::Service(ListInstanceAttributesError::InternalService(
                        err.msg,
                    ))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(ListInstanceAttributesError::InvalidParameter(
                        err.msg,
                    ))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(ListInstanceAttributesError::InvalidRequest(
                        err.msg,
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(ListInstanceAttributesError::ResourceNotFound(
                        err.msg,
                    ))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(ListInstanceAttributesError::Throttling(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ListInstanceAttributesError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListInstanceAttributesError::InternalService(ref cause) => write!(f, "{}", cause),
            ListInstanceAttributesError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            ListInstanceAttributesError::InvalidRequest(ref cause) => write!(f, "{}", cause),
            ListInstanceAttributesError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            ListInstanceAttributesError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListInstanceAttributesError {}
/// Errors returned by ListInstanceStorageConfigs
#[derive(Debug, PartialEq)]
pub enum ListInstanceStorageConfigsError {
    /// <p>Request processing failed due to an error or failure with the service.</p>
    InternalService(String),
    /// <p>One or more of the specified parameters are not valid.</p>
    InvalidParameter(String),
    /// <p>The request is not valid.</p>
    InvalidRequest(String),
    /// <p>The specified resource was not found.</p>
    ResourceNotFound(String),
    /// <p>The throttling limit has been exceeded.</p>
    Throttling(String),
}

impl ListInstanceStorageConfigsError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<ListInstanceStorageConfigsError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InternalServiceException" => {
                    return RusotoError::Service(ListInstanceStorageConfigsError::InternalService(
                        err.msg,
                    ))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(ListInstanceStorageConfigsError::InvalidParameter(
                        err.msg,
                    ))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(ListInstanceStorageConfigsError::InvalidRequest(
                        err.msg,
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(ListInstanceStorageConfigsError::ResourceNotFound(
                        err.msg,
                    ))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(ListInstanceStorageConfigsError::Throttling(
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
impl fmt::Display for ListInstanceStorageConfigsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListInstanceStorageConfigsError::InternalService(ref cause) => write!(f, "{}", cause),
            ListInstanceStorageConfigsError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            ListInstanceStorageConfigsError::InvalidRequest(ref cause) => write!(f, "{}", cause),
            ListInstanceStorageConfigsError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            ListInstanceStorageConfigsError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListInstanceStorageConfigsError {}
/// Errors returned by ListInstances
#[derive(Debug, PartialEq)]
pub enum ListInstancesError {
    /// <p>Request processing failed due to an error or failure with the service.</p>
    InternalService(String),
    /// <p>The request is not valid.</p>
    InvalidRequest(String),
}

impl ListInstancesError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListInstancesError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InternalServiceException" => {
                    return RusotoError::Service(ListInstancesError::InternalService(err.msg))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(ListInstancesError::InvalidRequest(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ListInstancesError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListInstancesError::InternalService(ref cause) => write!(f, "{}", cause),
            ListInstancesError::InvalidRequest(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListInstancesError {}
/// Errors returned by ListIntegrationAssociations
#[derive(Debug, PartialEq)]
pub enum ListIntegrationAssociationsError {
    /// <p>Request processing failed due to an error or failure with the service.</p>
    InternalService(String),
    /// <p>The request is not valid.</p>
    InvalidRequest(String),
    /// <p>The specified resource was not found.</p>
    ResourceNotFound(String),
    /// <p>The throttling limit has been exceeded.</p>
    Throttling(String),
}

impl ListIntegrationAssociationsError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<ListIntegrationAssociationsError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InternalServiceException" => {
                    return RusotoError::Service(ListIntegrationAssociationsError::InternalService(
                        err.msg,
                    ))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(ListIntegrationAssociationsError::InvalidRequest(
                        err.msg,
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(
                        ListIntegrationAssociationsError::ResourceNotFound(err.msg),
                    )
                }
                "ThrottlingException" => {
                    return RusotoError::Service(ListIntegrationAssociationsError::Throttling(
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
impl fmt::Display for ListIntegrationAssociationsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListIntegrationAssociationsError::InternalService(ref cause) => write!(f, "{}", cause),
            ListIntegrationAssociationsError::InvalidRequest(ref cause) => write!(f, "{}", cause),
            ListIntegrationAssociationsError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            ListIntegrationAssociationsError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListIntegrationAssociationsError {}
/// Errors returned by ListLambdaFunctions
#[derive(Debug, PartialEq)]
pub enum ListLambdaFunctionsError {
    /// <p>Request processing failed due to an error or failure with the service.</p>
    InternalService(String),
    /// <p>One or more of the specified parameters are not valid.</p>
    InvalidParameter(String),
    /// <p>The request is not valid.</p>
    InvalidRequest(String),
    /// <p>The specified resource was not found.</p>
    ResourceNotFound(String),
    /// <p>The throttling limit has been exceeded.</p>
    Throttling(String),
}

impl ListLambdaFunctionsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListLambdaFunctionsError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InternalServiceException" => {
                    return RusotoError::Service(ListLambdaFunctionsError::InternalService(err.msg))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(ListLambdaFunctionsError::InvalidParameter(
                        err.msg,
                    ))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(ListLambdaFunctionsError::InvalidRequest(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(ListLambdaFunctionsError::ResourceNotFound(
                        err.msg,
                    ))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(ListLambdaFunctionsError::Throttling(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ListLambdaFunctionsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListLambdaFunctionsError::InternalService(ref cause) => write!(f, "{}", cause),
            ListLambdaFunctionsError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            ListLambdaFunctionsError::InvalidRequest(ref cause) => write!(f, "{}", cause),
            ListLambdaFunctionsError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            ListLambdaFunctionsError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListLambdaFunctionsError {}
/// Errors returned by ListLexBots
#[derive(Debug, PartialEq)]
pub enum ListLexBotsError {
    /// <p>Request processing failed due to an error or failure with the service.</p>
    InternalService(String),
    /// <p>One or more of the specified parameters are not valid.</p>
    InvalidParameter(String),
    /// <p>The request is not valid.</p>
    InvalidRequest(String),
    /// <p>The specified resource was not found.</p>
    ResourceNotFound(String),
    /// <p>The throttling limit has been exceeded.</p>
    Throttling(String),
}

impl ListLexBotsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListLexBotsError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InternalServiceException" => {
                    return RusotoError::Service(ListLexBotsError::InternalService(err.msg))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(ListLexBotsError::InvalidParameter(err.msg))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(ListLexBotsError::InvalidRequest(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(ListLexBotsError::ResourceNotFound(err.msg))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(ListLexBotsError::Throttling(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ListLexBotsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListLexBotsError::InternalService(ref cause) => write!(f, "{}", cause),
            ListLexBotsError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            ListLexBotsError::InvalidRequest(ref cause) => write!(f, "{}", cause),
            ListLexBotsError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            ListLexBotsError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListLexBotsError {}
/// Errors returned by ListPhoneNumbers
#[derive(Debug, PartialEq)]
pub enum ListPhoneNumbersError {
    /// <p>Request processing failed due to an error or failure with the service.</p>
    InternalService(String),
    /// <p>One or more of the specified parameters are not valid.</p>
    InvalidParameter(String),
    /// <p>The request is not valid.</p>
    InvalidRequest(String),
    /// <p>The specified resource was not found.</p>
    ResourceNotFound(String),
    /// <p>The throttling limit has been exceeded.</p>
    Throttling(String),
}

impl ListPhoneNumbersError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListPhoneNumbersError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InternalServiceException" => {
                    return RusotoError::Service(ListPhoneNumbersError::InternalService(err.msg))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(ListPhoneNumbersError::InvalidParameter(err.msg))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(ListPhoneNumbersError::InvalidRequest(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(ListPhoneNumbersError::ResourceNotFound(err.msg))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(ListPhoneNumbersError::Throttling(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ListPhoneNumbersError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListPhoneNumbersError::InternalService(ref cause) => write!(f, "{}", cause),
            ListPhoneNumbersError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            ListPhoneNumbersError::InvalidRequest(ref cause) => write!(f, "{}", cause),
            ListPhoneNumbersError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            ListPhoneNumbersError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListPhoneNumbersError {}
/// Errors returned by ListPrompts
#[derive(Debug, PartialEq)]
pub enum ListPromptsError {
    /// <p>Request processing failed due to an error or failure with the service.</p>
    InternalService(String),
    /// <p>One or more of the specified parameters are not valid.</p>
    InvalidParameter(String),
    /// <p>The request is not valid.</p>
    InvalidRequest(String),
    /// <p>The specified resource was not found.</p>
    ResourceNotFound(String),
    /// <p>The throttling limit has been exceeded.</p>
    Throttling(String),
}

impl ListPromptsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListPromptsError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InternalServiceException" => {
                    return RusotoError::Service(ListPromptsError::InternalService(err.msg))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(ListPromptsError::InvalidParameter(err.msg))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(ListPromptsError::InvalidRequest(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(ListPromptsError::ResourceNotFound(err.msg))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(ListPromptsError::Throttling(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ListPromptsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListPromptsError::InternalService(ref cause) => write!(f, "{}", cause),
            ListPromptsError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            ListPromptsError::InvalidRequest(ref cause) => write!(f, "{}", cause),
            ListPromptsError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            ListPromptsError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListPromptsError {}
/// Errors returned by ListQueues
#[derive(Debug, PartialEq)]
pub enum ListQueuesError {
    /// <p>Request processing failed due to an error or failure with the service.</p>
    InternalService(String),
    /// <p>One or more of the specified parameters are not valid.</p>
    InvalidParameter(String),
    /// <p>The request is not valid.</p>
    InvalidRequest(String),
    /// <p>The specified resource was not found.</p>
    ResourceNotFound(String),
    /// <p>The throttling limit has been exceeded.</p>
    Throttling(String),
}

impl ListQueuesError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListQueuesError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InternalServiceException" => {
                    return RusotoError::Service(ListQueuesError::InternalService(err.msg))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(ListQueuesError::InvalidParameter(err.msg))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(ListQueuesError::InvalidRequest(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(ListQueuesError::ResourceNotFound(err.msg))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(ListQueuesError::Throttling(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ListQueuesError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListQueuesError::InternalService(ref cause) => write!(f, "{}", cause),
            ListQueuesError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            ListQueuesError::InvalidRequest(ref cause) => write!(f, "{}", cause),
            ListQueuesError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            ListQueuesError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListQueuesError {}
/// Errors returned by ListQuickConnects
#[derive(Debug, PartialEq)]
pub enum ListQuickConnectsError {
    /// <p>Request processing failed due to an error or failure with the service.</p>
    InternalService(String),
    /// <p>One or more of the specified parameters are not valid.</p>
    InvalidParameter(String),
    /// <p>The request is not valid.</p>
    InvalidRequest(String),
    /// <p>The specified resource was not found.</p>
    ResourceNotFound(String),
    /// <p>The throttling limit has been exceeded.</p>
    Throttling(String),
}

impl ListQuickConnectsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListQuickConnectsError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InternalServiceException" => {
                    return RusotoError::Service(ListQuickConnectsError::InternalService(err.msg))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(ListQuickConnectsError::InvalidParameter(err.msg))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(ListQuickConnectsError::InvalidRequest(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(ListQuickConnectsError::ResourceNotFound(err.msg))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(ListQuickConnectsError::Throttling(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ListQuickConnectsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListQuickConnectsError::InternalService(ref cause) => write!(f, "{}", cause),
            ListQuickConnectsError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            ListQuickConnectsError::InvalidRequest(ref cause) => write!(f, "{}", cause),
            ListQuickConnectsError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            ListQuickConnectsError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListQuickConnectsError {}
/// Errors returned by ListRoutingProfileQueues
#[derive(Debug, PartialEq)]
pub enum ListRoutingProfileQueuesError {
    /// <p>Request processing failed due to an error or failure with the service.</p>
    InternalService(String),
    /// <p>One or more of the specified parameters are not valid.</p>
    InvalidParameter(String),
    /// <p>The request is not valid.</p>
    InvalidRequest(String),
    /// <p>The specified resource was not found.</p>
    ResourceNotFound(String),
    /// <p>The throttling limit has been exceeded.</p>
    Throttling(String),
}

impl ListRoutingProfileQueuesError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListRoutingProfileQueuesError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InternalServiceException" => {
                    return RusotoError::Service(ListRoutingProfileQueuesError::InternalService(
                        err.msg,
                    ))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(ListRoutingProfileQueuesError::InvalidParameter(
                        err.msg,
                    ))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(ListRoutingProfileQueuesError::InvalidRequest(
                        err.msg,
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(ListRoutingProfileQueuesError::ResourceNotFound(
                        err.msg,
                    ))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(ListRoutingProfileQueuesError::Throttling(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ListRoutingProfileQueuesError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListRoutingProfileQueuesError::InternalService(ref cause) => write!(f, "{}", cause),
            ListRoutingProfileQueuesError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            ListRoutingProfileQueuesError::InvalidRequest(ref cause) => write!(f, "{}", cause),
            ListRoutingProfileQueuesError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            ListRoutingProfileQueuesError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListRoutingProfileQueuesError {}
/// Errors returned by ListRoutingProfiles
#[derive(Debug, PartialEq)]
pub enum ListRoutingProfilesError {
    /// <p>Request processing failed due to an error or failure with the service.</p>
    InternalService(String),
    /// <p>One or more of the specified parameters are not valid.</p>
    InvalidParameter(String),
    /// <p>The request is not valid.</p>
    InvalidRequest(String),
    /// <p>The specified resource was not found.</p>
    ResourceNotFound(String),
    /// <p>The throttling limit has been exceeded.</p>
    Throttling(String),
}

impl ListRoutingProfilesError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListRoutingProfilesError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InternalServiceException" => {
                    return RusotoError::Service(ListRoutingProfilesError::InternalService(err.msg))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(ListRoutingProfilesError::InvalidParameter(
                        err.msg,
                    ))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(ListRoutingProfilesError::InvalidRequest(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(ListRoutingProfilesError::ResourceNotFound(
                        err.msg,
                    ))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(ListRoutingProfilesError::Throttling(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ListRoutingProfilesError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListRoutingProfilesError::InternalService(ref cause) => write!(f, "{}", cause),
            ListRoutingProfilesError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            ListRoutingProfilesError::InvalidRequest(ref cause) => write!(f, "{}", cause),
            ListRoutingProfilesError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            ListRoutingProfilesError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListRoutingProfilesError {}
/// Errors returned by ListSecurityKeys
#[derive(Debug, PartialEq)]
pub enum ListSecurityKeysError {
    /// <p>Request processing failed due to an error or failure with the service.</p>
    InternalService(String),
    /// <p>One or more of the specified parameters are not valid.</p>
    InvalidParameter(String),
    /// <p>The request is not valid.</p>
    InvalidRequest(String),
    /// <p>The specified resource was not found.</p>
    ResourceNotFound(String),
    /// <p>The throttling limit has been exceeded.</p>
    Throttling(String),
}

impl ListSecurityKeysError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListSecurityKeysError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InternalServiceException" => {
                    return RusotoError::Service(ListSecurityKeysError::InternalService(err.msg))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(ListSecurityKeysError::InvalidParameter(err.msg))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(ListSecurityKeysError::InvalidRequest(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(ListSecurityKeysError::ResourceNotFound(err.msg))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(ListSecurityKeysError::Throttling(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ListSecurityKeysError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListSecurityKeysError::InternalService(ref cause) => write!(f, "{}", cause),
            ListSecurityKeysError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            ListSecurityKeysError::InvalidRequest(ref cause) => write!(f, "{}", cause),
            ListSecurityKeysError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            ListSecurityKeysError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListSecurityKeysError {}
/// Errors returned by ListSecurityProfiles
#[derive(Debug, PartialEq)]
pub enum ListSecurityProfilesError {
    /// <p>Request processing failed due to an error or failure with the service.</p>
    InternalService(String),
    /// <p>One or more of the specified parameters are not valid.</p>
    InvalidParameter(String),
    /// <p>The request is not valid.</p>
    InvalidRequest(String),
    /// <p>The specified resource was not found.</p>
    ResourceNotFound(String),
    /// <p>The throttling limit has been exceeded.</p>
    Throttling(String),
}

impl ListSecurityProfilesError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListSecurityProfilesError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InternalServiceException" => {
                    return RusotoError::Service(ListSecurityProfilesError::InternalService(
                        err.msg,
                    ))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(ListSecurityProfilesError::InvalidParameter(
                        err.msg,
                    ))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(ListSecurityProfilesError::InvalidRequest(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(ListSecurityProfilesError::ResourceNotFound(
                        err.msg,
                    ))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(ListSecurityProfilesError::Throttling(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ListSecurityProfilesError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListSecurityProfilesError::InternalService(ref cause) => write!(f, "{}", cause),
            ListSecurityProfilesError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            ListSecurityProfilesError::InvalidRequest(ref cause) => write!(f, "{}", cause),
            ListSecurityProfilesError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            ListSecurityProfilesError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListSecurityProfilesError {}
/// Errors returned by ListTagsForResource
#[derive(Debug, PartialEq)]
pub enum ListTagsForResourceError {
    /// <p>Request processing failed due to an error or failure with the service.</p>
    InternalService(String),
    /// <p>One or more of the specified parameters are not valid.</p>
    InvalidParameter(String),
    /// <p>The request is not valid.</p>
    InvalidRequest(String),
    /// <p>The specified resource was not found.</p>
    ResourceNotFound(String),
    /// <p>The throttling limit has been exceeded.</p>
    Throttling(String),
}

impl ListTagsForResourceError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListTagsForResourceError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InternalServiceException" => {
                    return RusotoError::Service(ListTagsForResourceError::InternalService(err.msg))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(ListTagsForResourceError::InvalidParameter(
                        err.msg,
                    ))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(ListTagsForResourceError::InvalidRequest(err.msg))
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
            ListTagsForResourceError::InternalService(ref cause) => write!(f, "{}", cause),
            ListTagsForResourceError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            ListTagsForResourceError::InvalidRequest(ref cause) => write!(f, "{}", cause),
            ListTagsForResourceError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            ListTagsForResourceError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListTagsForResourceError {}
/// Errors returned by ListUseCases
#[derive(Debug, PartialEq)]
pub enum ListUseCasesError {
    /// <p>Request processing failed due to an error or failure with the service.</p>
    InternalService(String),
    /// <p>The request is not valid.</p>
    InvalidRequest(String),
    /// <p>The specified resource was not found.</p>
    ResourceNotFound(String),
    /// <p>The throttling limit has been exceeded.</p>
    Throttling(String),
}

impl ListUseCasesError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListUseCasesError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InternalServiceException" => {
                    return RusotoError::Service(ListUseCasesError::InternalService(err.msg))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(ListUseCasesError::InvalidRequest(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(ListUseCasesError::ResourceNotFound(err.msg))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(ListUseCasesError::Throttling(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ListUseCasesError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListUseCasesError::InternalService(ref cause) => write!(f, "{}", cause),
            ListUseCasesError::InvalidRequest(ref cause) => write!(f, "{}", cause),
            ListUseCasesError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            ListUseCasesError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListUseCasesError {}
/// Errors returned by ListUserHierarchyGroups
#[derive(Debug, PartialEq)]
pub enum ListUserHierarchyGroupsError {
    /// <p>Request processing failed due to an error or failure with the service.</p>
    InternalService(String),
    /// <p>One or more of the specified parameters are not valid.</p>
    InvalidParameter(String),
    /// <p>The request is not valid.</p>
    InvalidRequest(String),
    /// <p>The specified resource was not found.</p>
    ResourceNotFound(String),
    /// <p>The throttling limit has been exceeded.</p>
    Throttling(String),
}

impl ListUserHierarchyGroupsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListUserHierarchyGroupsError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InternalServiceException" => {
                    return RusotoError::Service(ListUserHierarchyGroupsError::InternalService(
                        err.msg,
                    ))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(ListUserHierarchyGroupsError::InvalidParameter(
                        err.msg,
                    ))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(ListUserHierarchyGroupsError::InvalidRequest(
                        err.msg,
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(ListUserHierarchyGroupsError::ResourceNotFound(
                        err.msg,
                    ))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(ListUserHierarchyGroupsError::Throttling(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ListUserHierarchyGroupsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListUserHierarchyGroupsError::InternalService(ref cause) => write!(f, "{}", cause),
            ListUserHierarchyGroupsError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            ListUserHierarchyGroupsError::InvalidRequest(ref cause) => write!(f, "{}", cause),
            ListUserHierarchyGroupsError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            ListUserHierarchyGroupsError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListUserHierarchyGroupsError {}
/// Errors returned by ListUsers
#[derive(Debug, PartialEq)]
pub enum ListUsersError {
    /// <p>Request processing failed due to an error or failure with the service.</p>
    InternalService(String),
    /// <p>One or more of the specified parameters are not valid.</p>
    InvalidParameter(String),
    /// <p>The request is not valid.</p>
    InvalidRequest(String),
    /// <p>The specified resource was not found.</p>
    ResourceNotFound(String),
    /// <p>The throttling limit has been exceeded.</p>
    Throttling(String),
}

impl ListUsersError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListUsersError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InternalServiceException" => {
                    return RusotoError::Service(ListUsersError::InternalService(err.msg))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(ListUsersError::InvalidParameter(err.msg))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(ListUsersError::InvalidRequest(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(ListUsersError::ResourceNotFound(err.msg))
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
            ListUsersError::InternalService(ref cause) => write!(f, "{}", cause),
            ListUsersError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            ListUsersError::InvalidRequest(ref cause) => write!(f, "{}", cause),
            ListUsersError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            ListUsersError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListUsersError {}
/// Errors returned by ResumeContactRecording
#[derive(Debug, PartialEq)]
pub enum ResumeContactRecordingError {
    /// <p>Request processing failed due to an error or failure with the service.</p>
    InternalService(String),
    /// <p>The request is not valid.</p>
    InvalidRequest(String),
    /// <p>The specified resource was not found.</p>
    ResourceNotFound(String),
}

impl ResumeContactRecordingError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ResumeContactRecordingError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InternalServiceException" => {
                    return RusotoError::Service(ResumeContactRecordingError::InternalService(
                        err.msg,
                    ))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(ResumeContactRecordingError::InvalidRequest(
                        err.msg,
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(ResumeContactRecordingError::ResourceNotFound(
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
impl fmt::Display for ResumeContactRecordingError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ResumeContactRecordingError::InternalService(ref cause) => write!(f, "{}", cause),
            ResumeContactRecordingError::InvalidRequest(ref cause) => write!(f, "{}", cause),
            ResumeContactRecordingError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ResumeContactRecordingError {}
/// Errors returned by StartChatContact
#[derive(Debug, PartialEq)]
pub enum StartChatContactError {
    /// <p>Request processing failed due to an error or failure with the service.</p>
    InternalService(String),
    /// <p>One or more of the specified parameters are not valid.</p>
    InvalidParameter(String),
    /// <p>The request is not valid.</p>
    InvalidRequest(String),
    /// <p>The allowed limit for the resource has been exceeded.</p>
    LimitExceeded(String),
    /// <p>The specified resource was not found.</p>
    ResourceNotFound(String),
}

impl StartChatContactError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<StartChatContactError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InternalServiceException" => {
                    return RusotoError::Service(StartChatContactError::InternalService(err.msg))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(StartChatContactError::InvalidParameter(err.msg))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(StartChatContactError::InvalidRequest(err.msg))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(StartChatContactError::LimitExceeded(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(StartChatContactError::ResourceNotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for StartChatContactError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            StartChatContactError::InternalService(ref cause) => write!(f, "{}", cause),
            StartChatContactError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            StartChatContactError::InvalidRequest(ref cause) => write!(f, "{}", cause),
            StartChatContactError::LimitExceeded(ref cause) => write!(f, "{}", cause),
            StartChatContactError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for StartChatContactError {}
/// Errors returned by StartContactRecording
#[derive(Debug, PartialEq)]
pub enum StartContactRecordingError {
    /// <p>Request processing failed due to an error or failure with the service.</p>
    InternalService(String),
    /// <p>One or more of the specified parameters are not valid.</p>
    InvalidParameter(String),
    /// <p>The request is not valid.</p>
    InvalidRequest(String),
    /// <p>The specified resource was not found.</p>
    ResourceNotFound(String),
}

impl StartContactRecordingError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<StartContactRecordingError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InternalServiceException" => {
                    return RusotoError::Service(StartContactRecordingError::InternalService(
                        err.msg,
                    ))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(StartContactRecordingError::InvalidParameter(
                        err.msg,
                    ))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(StartContactRecordingError::InvalidRequest(
                        err.msg,
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(StartContactRecordingError::ResourceNotFound(
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
impl fmt::Display for StartContactRecordingError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            StartContactRecordingError::InternalService(ref cause) => write!(f, "{}", cause),
            StartContactRecordingError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            StartContactRecordingError::InvalidRequest(ref cause) => write!(f, "{}", cause),
            StartContactRecordingError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for StartContactRecordingError {}
/// Errors returned by StartOutboundVoiceContact
#[derive(Debug, PartialEq)]
pub enum StartOutboundVoiceContactError {
    /// <p>Outbound calls to the destination number are not allowed.</p>
    DestinationNotAllowed(String),
    /// <p>Request processing failed due to an error or failure with the service.</p>
    InternalService(String),
    /// <p>One or more of the specified parameters are not valid.</p>
    InvalidParameter(String),
    /// <p>The request is not valid.</p>
    InvalidRequest(String),
    /// <p>The allowed limit for the resource has been exceeded.</p>
    LimitExceeded(String),
    /// <p>The contact is not permitted.</p>
    OutboundContactNotPermitted(String),
    /// <p>The specified resource was not found.</p>
    ResourceNotFound(String),
}

impl StartOutboundVoiceContactError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<StartOutboundVoiceContactError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "DestinationNotAllowedException" => {
                    return RusotoError::Service(
                        StartOutboundVoiceContactError::DestinationNotAllowed(err.msg),
                    )
                }
                "InternalServiceException" => {
                    return RusotoError::Service(StartOutboundVoiceContactError::InternalService(
                        err.msg,
                    ))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(StartOutboundVoiceContactError::InvalidParameter(
                        err.msg,
                    ))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(StartOutboundVoiceContactError::InvalidRequest(
                        err.msg,
                    ))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(StartOutboundVoiceContactError::LimitExceeded(
                        err.msg,
                    ))
                }
                "OutboundContactNotPermittedException" => {
                    return RusotoError::Service(
                        StartOutboundVoiceContactError::OutboundContactNotPermitted(err.msg),
                    )
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(StartOutboundVoiceContactError::ResourceNotFound(
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
impl fmt::Display for StartOutboundVoiceContactError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            StartOutboundVoiceContactError::DestinationNotAllowed(ref cause) => {
                write!(f, "{}", cause)
            }
            StartOutboundVoiceContactError::InternalService(ref cause) => write!(f, "{}", cause),
            StartOutboundVoiceContactError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            StartOutboundVoiceContactError::InvalidRequest(ref cause) => write!(f, "{}", cause),
            StartOutboundVoiceContactError::LimitExceeded(ref cause) => write!(f, "{}", cause),
            StartOutboundVoiceContactError::OutboundContactNotPermitted(ref cause) => {
                write!(f, "{}", cause)
            }
            StartOutboundVoiceContactError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for StartOutboundVoiceContactError {}
/// Errors returned by StartTaskContact
#[derive(Debug, PartialEq)]
pub enum StartTaskContactError {
    /// <p>Request processing failed due to an error or failure with the service.</p>
    InternalService(String),
    /// <p>One or more of the specified parameters are not valid.</p>
    InvalidParameter(String),
    /// <p>The request is not valid.</p>
    InvalidRequest(String),
    /// <p>The specified resource was not found.</p>
    ResourceNotFound(String),
    /// <p>The service quota has been exceeded.</p>
    ServiceQuotaExceeded(String),
    /// <p>The throttling limit has been exceeded.</p>
    Throttling(String),
}

impl StartTaskContactError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<StartTaskContactError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InternalServiceException" => {
                    return RusotoError::Service(StartTaskContactError::InternalService(err.msg))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(StartTaskContactError::InvalidParameter(err.msg))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(StartTaskContactError::InvalidRequest(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(StartTaskContactError::ResourceNotFound(err.msg))
                }
                "ServiceQuotaExceededException" => {
                    return RusotoError::Service(StartTaskContactError::ServiceQuotaExceeded(
                        err.msg,
                    ))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(StartTaskContactError::Throttling(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for StartTaskContactError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            StartTaskContactError::InternalService(ref cause) => write!(f, "{}", cause),
            StartTaskContactError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            StartTaskContactError::InvalidRequest(ref cause) => write!(f, "{}", cause),
            StartTaskContactError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            StartTaskContactError::ServiceQuotaExceeded(ref cause) => write!(f, "{}", cause),
            StartTaskContactError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for StartTaskContactError {}
/// Errors returned by StopContact
#[derive(Debug, PartialEq)]
pub enum StopContactError {
    /// <p>The contact with the specified ID is not active or does not exist.</p>
    ContactNotFound(String),
    /// <p>Request processing failed due to an error or failure with the service.</p>
    InternalService(String),
    /// <p>One or more of the specified parameters are not valid.</p>
    InvalidParameter(String),
    /// <p>The request is not valid.</p>
    InvalidRequest(String),
    /// <p>The specified resource was not found.</p>
    ResourceNotFound(String),
}

impl StopContactError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<StopContactError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "ContactNotFoundException" => {
                    return RusotoError::Service(StopContactError::ContactNotFound(err.msg))
                }
                "InternalServiceException" => {
                    return RusotoError::Service(StopContactError::InternalService(err.msg))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(StopContactError::InvalidParameter(err.msg))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(StopContactError::InvalidRequest(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(StopContactError::ResourceNotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for StopContactError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            StopContactError::ContactNotFound(ref cause) => write!(f, "{}", cause),
            StopContactError::InternalService(ref cause) => write!(f, "{}", cause),
            StopContactError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            StopContactError::InvalidRequest(ref cause) => write!(f, "{}", cause),
            StopContactError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for StopContactError {}
/// Errors returned by StopContactRecording
#[derive(Debug, PartialEq)]
pub enum StopContactRecordingError {
    /// <p>Request processing failed due to an error or failure with the service.</p>
    InternalService(String),
    /// <p>The request is not valid.</p>
    InvalidRequest(String),
    /// <p>The specified resource was not found.</p>
    ResourceNotFound(String),
}

impl StopContactRecordingError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<StopContactRecordingError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InternalServiceException" => {
                    return RusotoError::Service(StopContactRecordingError::InternalService(
                        err.msg,
                    ))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(StopContactRecordingError::InvalidRequest(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(StopContactRecordingError::ResourceNotFound(
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
impl fmt::Display for StopContactRecordingError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            StopContactRecordingError::InternalService(ref cause) => write!(f, "{}", cause),
            StopContactRecordingError::InvalidRequest(ref cause) => write!(f, "{}", cause),
            StopContactRecordingError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for StopContactRecordingError {}
/// Errors returned by SuspendContactRecording
#[derive(Debug, PartialEq)]
pub enum SuspendContactRecordingError {
    /// <p>Request processing failed due to an error or failure with the service.</p>
    InternalService(String),
    /// <p>The request is not valid.</p>
    InvalidRequest(String),
    /// <p>The specified resource was not found.</p>
    ResourceNotFound(String),
}

impl SuspendContactRecordingError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<SuspendContactRecordingError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InternalServiceException" => {
                    return RusotoError::Service(SuspendContactRecordingError::InternalService(
                        err.msg,
                    ))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(SuspendContactRecordingError::InvalidRequest(
                        err.msg,
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(SuspendContactRecordingError::ResourceNotFound(
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
impl fmt::Display for SuspendContactRecordingError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            SuspendContactRecordingError::InternalService(ref cause) => write!(f, "{}", cause),
            SuspendContactRecordingError::InvalidRequest(ref cause) => write!(f, "{}", cause),
            SuspendContactRecordingError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for SuspendContactRecordingError {}
/// Errors returned by TagResource
#[derive(Debug, PartialEq)]
pub enum TagResourceError {
    /// <p>Request processing failed due to an error or failure with the service.</p>
    InternalService(String),
    /// <p>One or more of the specified parameters are not valid.</p>
    InvalidParameter(String),
    /// <p>The request is not valid.</p>
    InvalidRequest(String),
    /// <p>The specified resource was not found.</p>
    ResourceNotFound(String),
    /// <p>The throttling limit has been exceeded.</p>
    Throttling(String),
}

impl TagResourceError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<TagResourceError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InternalServiceException" => {
                    return RusotoError::Service(TagResourceError::InternalService(err.msg))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(TagResourceError::InvalidParameter(err.msg))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(TagResourceError::InvalidRequest(err.msg))
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
            TagResourceError::InternalService(ref cause) => write!(f, "{}", cause),
            TagResourceError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            TagResourceError::InvalidRequest(ref cause) => write!(f, "{}", cause),
            TagResourceError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            TagResourceError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for TagResourceError {}
/// Errors returned by UntagResource
#[derive(Debug, PartialEq)]
pub enum UntagResourceError {
    /// <p>Request processing failed due to an error or failure with the service.</p>
    InternalService(String),
    /// <p>One or more of the specified parameters are not valid.</p>
    InvalidParameter(String),
    /// <p>The request is not valid.</p>
    InvalidRequest(String),
    /// <p>The specified resource was not found.</p>
    ResourceNotFound(String),
    /// <p>The throttling limit has been exceeded.</p>
    Throttling(String),
}

impl UntagResourceError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UntagResourceError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InternalServiceException" => {
                    return RusotoError::Service(UntagResourceError::InternalService(err.msg))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(UntagResourceError::InvalidParameter(err.msg))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(UntagResourceError::InvalidRequest(err.msg))
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
            UntagResourceError::InternalService(ref cause) => write!(f, "{}", cause),
            UntagResourceError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            UntagResourceError::InvalidRequest(ref cause) => write!(f, "{}", cause),
            UntagResourceError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            UntagResourceError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for UntagResourceError {}
/// Errors returned by UpdateContactAttributes
#[derive(Debug, PartialEq)]
pub enum UpdateContactAttributesError {
    /// <p>Request processing failed due to an error or failure with the service.</p>
    InternalService(String),
    /// <p>One or more of the specified parameters are not valid.</p>
    InvalidParameter(String),
    /// <p>The request is not valid.</p>
    InvalidRequest(String),
    /// <p>The specified resource was not found.</p>
    ResourceNotFound(String),
}

impl UpdateContactAttributesError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdateContactAttributesError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InternalServiceException" => {
                    return RusotoError::Service(UpdateContactAttributesError::InternalService(
                        err.msg,
                    ))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(UpdateContactAttributesError::InvalidParameter(
                        err.msg,
                    ))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(UpdateContactAttributesError::InvalidRequest(
                        err.msg,
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(UpdateContactAttributesError::ResourceNotFound(
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
impl fmt::Display for UpdateContactAttributesError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UpdateContactAttributesError::InternalService(ref cause) => write!(f, "{}", cause),
            UpdateContactAttributesError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            UpdateContactAttributesError::InvalidRequest(ref cause) => write!(f, "{}", cause),
            UpdateContactAttributesError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for UpdateContactAttributesError {}
/// Errors returned by UpdateContactFlowContent
#[derive(Debug, PartialEq)]
pub enum UpdateContactFlowContentError {
    /// <p>Request processing failed due to an error or failure with the service.</p>
    InternalService(String),
    /// <p>The contact flow is not valid.</p>
    InvalidContactFlow(String),
    /// <p>One or more of the specified parameters are not valid.</p>
    InvalidParameter(String),
    /// <p>The request is not valid.</p>
    InvalidRequest(String),
    /// <p>The specified resource was not found.</p>
    ResourceNotFound(String),
    /// <p>The throttling limit has been exceeded.</p>
    Throttling(String),
}

impl UpdateContactFlowContentError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdateContactFlowContentError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InternalServiceException" => {
                    return RusotoError::Service(UpdateContactFlowContentError::InternalService(
                        err.msg,
                    ))
                }
                "InvalidContactFlowException" => {
                    return RusotoError::Service(UpdateContactFlowContentError::InvalidContactFlow(
                        err.msg,
                    ))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(UpdateContactFlowContentError::InvalidParameter(
                        err.msg,
                    ))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(UpdateContactFlowContentError::InvalidRequest(
                        err.msg,
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(UpdateContactFlowContentError::ResourceNotFound(
                        err.msg,
                    ))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(UpdateContactFlowContentError::Throttling(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for UpdateContactFlowContentError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UpdateContactFlowContentError::InternalService(ref cause) => write!(f, "{}", cause),
            UpdateContactFlowContentError::InvalidContactFlow(ref cause) => write!(f, "{}", cause),
            UpdateContactFlowContentError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            UpdateContactFlowContentError::InvalidRequest(ref cause) => write!(f, "{}", cause),
            UpdateContactFlowContentError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            UpdateContactFlowContentError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for UpdateContactFlowContentError {}
/// Errors returned by UpdateContactFlowName
#[derive(Debug, PartialEq)]
pub enum UpdateContactFlowNameError {
    /// <p>A resource with the specified name already exists.</p>
    DuplicateResource(String),
    /// <p>Request processing failed due to an error or failure with the service.</p>
    InternalService(String),
    /// <p>One or more of the specified parameters are not valid.</p>
    InvalidParameter(String),
    /// <p>The request is not valid.</p>
    InvalidRequest(String),
    /// <p>The specified resource was not found.</p>
    ResourceNotFound(String),
    /// <p>The throttling limit has been exceeded.</p>
    Throttling(String),
}

impl UpdateContactFlowNameError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdateContactFlowNameError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "DuplicateResourceException" => {
                    return RusotoError::Service(UpdateContactFlowNameError::DuplicateResource(
                        err.msg,
                    ))
                }
                "InternalServiceException" => {
                    return RusotoError::Service(UpdateContactFlowNameError::InternalService(
                        err.msg,
                    ))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(UpdateContactFlowNameError::InvalidParameter(
                        err.msg,
                    ))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(UpdateContactFlowNameError::InvalidRequest(
                        err.msg,
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(UpdateContactFlowNameError::ResourceNotFound(
                        err.msg,
                    ))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(UpdateContactFlowNameError::Throttling(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for UpdateContactFlowNameError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UpdateContactFlowNameError::DuplicateResource(ref cause) => write!(f, "{}", cause),
            UpdateContactFlowNameError::InternalService(ref cause) => write!(f, "{}", cause),
            UpdateContactFlowNameError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            UpdateContactFlowNameError::InvalidRequest(ref cause) => write!(f, "{}", cause),
            UpdateContactFlowNameError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            UpdateContactFlowNameError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for UpdateContactFlowNameError {}
/// Errors returned by UpdateInstanceAttribute
#[derive(Debug, PartialEq)]
pub enum UpdateInstanceAttributeError {
    /// <p>Request processing failed due to an error or failure with the service.</p>
    InternalService(String),
    /// <p>One or more of the specified parameters are not valid.</p>
    InvalidParameter(String),
    /// <p>The request is not valid.</p>
    InvalidRequest(String),
    /// <p>The specified resource was not found.</p>
    ResourceNotFound(String),
    /// <p>The throttling limit has been exceeded.</p>
    Throttling(String),
}

impl UpdateInstanceAttributeError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdateInstanceAttributeError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InternalServiceException" => {
                    return RusotoError::Service(UpdateInstanceAttributeError::InternalService(
                        err.msg,
                    ))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(UpdateInstanceAttributeError::InvalidParameter(
                        err.msg,
                    ))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(UpdateInstanceAttributeError::InvalidRequest(
                        err.msg,
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(UpdateInstanceAttributeError::ResourceNotFound(
                        err.msg,
                    ))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(UpdateInstanceAttributeError::Throttling(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for UpdateInstanceAttributeError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UpdateInstanceAttributeError::InternalService(ref cause) => write!(f, "{}", cause),
            UpdateInstanceAttributeError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            UpdateInstanceAttributeError::InvalidRequest(ref cause) => write!(f, "{}", cause),
            UpdateInstanceAttributeError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            UpdateInstanceAttributeError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for UpdateInstanceAttributeError {}
/// Errors returned by UpdateInstanceStorageConfig
#[derive(Debug, PartialEq)]
pub enum UpdateInstanceStorageConfigError {
    /// <p>Request processing failed due to an error or failure with the service.</p>
    InternalService(String),
    /// <p>One or more of the specified parameters are not valid.</p>
    InvalidParameter(String),
    /// <p>The request is not valid.</p>
    InvalidRequest(String),
    /// <p>The specified resource was not found.</p>
    ResourceNotFound(String),
    /// <p>The throttling limit has been exceeded.</p>
    Throttling(String),
}

impl UpdateInstanceStorageConfigError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<UpdateInstanceStorageConfigError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InternalServiceException" => {
                    return RusotoError::Service(UpdateInstanceStorageConfigError::InternalService(
                        err.msg,
                    ))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(
                        UpdateInstanceStorageConfigError::InvalidParameter(err.msg),
                    )
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(UpdateInstanceStorageConfigError::InvalidRequest(
                        err.msg,
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(
                        UpdateInstanceStorageConfigError::ResourceNotFound(err.msg),
                    )
                }
                "ThrottlingException" => {
                    return RusotoError::Service(UpdateInstanceStorageConfigError::Throttling(
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
impl fmt::Display for UpdateInstanceStorageConfigError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UpdateInstanceStorageConfigError::InternalService(ref cause) => write!(f, "{}", cause),
            UpdateInstanceStorageConfigError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            UpdateInstanceStorageConfigError::InvalidRequest(ref cause) => write!(f, "{}", cause),
            UpdateInstanceStorageConfigError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            UpdateInstanceStorageConfigError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for UpdateInstanceStorageConfigError {}
/// Errors returned by UpdateQuickConnectConfig
#[derive(Debug, PartialEq)]
pub enum UpdateQuickConnectConfigError {
    /// <p>Request processing failed due to an error or failure with the service.</p>
    InternalService(String),
    /// <p>One or more of the specified parameters are not valid.</p>
    InvalidParameter(String),
    /// <p>The request is not valid.</p>
    InvalidRequest(String),
    /// <p>The specified resource was not found.</p>
    ResourceNotFound(String),
    /// <p>The throttling limit has been exceeded.</p>
    Throttling(String),
}

impl UpdateQuickConnectConfigError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdateQuickConnectConfigError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InternalServiceException" => {
                    return RusotoError::Service(UpdateQuickConnectConfigError::InternalService(
                        err.msg,
                    ))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(UpdateQuickConnectConfigError::InvalidParameter(
                        err.msg,
                    ))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(UpdateQuickConnectConfigError::InvalidRequest(
                        err.msg,
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(UpdateQuickConnectConfigError::ResourceNotFound(
                        err.msg,
                    ))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(UpdateQuickConnectConfigError::Throttling(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for UpdateQuickConnectConfigError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UpdateQuickConnectConfigError::InternalService(ref cause) => write!(f, "{}", cause),
            UpdateQuickConnectConfigError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            UpdateQuickConnectConfigError::InvalidRequest(ref cause) => write!(f, "{}", cause),
            UpdateQuickConnectConfigError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            UpdateQuickConnectConfigError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for UpdateQuickConnectConfigError {}
/// Errors returned by UpdateQuickConnectName
#[derive(Debug, PartialEq)]
pub enum UpdateQuickConnectNameError {
    /// <p>Request processing failed due to an error or failure with the service.</p>
    InternalService(String),
    /// <p>One or more of the specified parameters are not valid.</p>
    InvalidParameter(String),
    /// <p>The request is not valid.</p>
    InvalidRequest(String),
    /// <p>The specified resource was not found.</p>
    ResourceNotFound(String),
    /// <p>The throttling limit has been exceeded.</p>
    Throttling(String),
}

impl UpdateQuickConnectNameError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdateQuickConnectNameError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InternalServiceException" => {
                    return RusotoError::Service(UpdateQuickConnectNameError::InternalService(
                        err.msg,
                    ))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(UpdateQuickConnectNameError::InvalidParameter(
                        err.msg,
                    ))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(UpdateQuickConnectNameError::InvalidRequest(
                        err.msg,
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(UpdateQuickConnectNameError::ResourceNotFound(
                        err.msg,
                    ))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(UpdateQuickConnectNameError::Throttling(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for UpdateQuickConnectNameError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UpdateQuickConnectNameError::InternalService(ref cause) => write!(f, "{}", cause),
            UpdateQuickConnectNameError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            UpdateQuickConnectNameError::InvalidRequest(ref cause) => write!(f, "{}", cause),
            UpdateQuickConnectNameError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            UpdateQuickConnectNameError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for UpdateQuickConnectNameError {}
/// Errors returned by UpdateRoutingProfileConcurrency
#[derive(Debug, PartialEq)]
pub enum UpdateRoutingProfileConcurrencyError {
    /// <p>Request processing failed due to an error or failure with the service.</p>
    InternalService(String),
    /// <p>One or more of the specified parameters are not valid.</p>
    InvalidParameter(String),
    /// <p>The request is not valid.</p>
    InvalidRequest(String),
    /// <p>The specified resource was not found.</p>
    ResourceNotFound(String),
    /// <p>The throttling limit has been exceeded.</p>
    Throttling(String),
}

impl UpdateRoutingProfileConcurrencyError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<UpdateRoutingProfileConcurrencyError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InternalServiceException" => {
                    return RusotoError::Service(
                        UpdateRoutingProfileConcurrencyError::InternalService(err.msg),
                    )
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(
                        UpdateRoutingProfileConcurrencyError::InvalidParameter(err.msg),
                    )
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(
                        UpdateRoutingProfileConcurrencyError::InvalidRequest(err.msg),
                    )
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(
                        UpdateRoutingProfileConcurrencyError::ResourceNotFound(err.msg),
                    )
                }
                "ThrottlingException" => {
                    return RusotoError::Service(UpdateRoutingProfileConcurrencyError::Throttling(
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
impl fmt::Display for UpdateRoutingProfileConcurrencyError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UpdateRoutingProfileConcurrencyError::InternalService(ref cause) => {
                write!(f, "{}", cause)
            }
            UpdateRoutingProfileConcurrencyError::InvalidParameter(ref cause) => {
                write!(f, "{}", cause)
            }
            UpdateRoutingProfileConcurrencyError::InvalidRequest(ref cause) => {
                write!(f, "{}", cause)
            }
            UpdateRoutingProfileConcurrencyError::ResourceNotFound(ref cause) => {
                write!(f, "{}", cause)
            }
            UpdateRoutingProfileConcurrencyError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for UpdateRoutingProfileConcurrencyError {}
/// Errors returned by UpdateRoutingProfileDefaultOutboundQueue
#[derive(Debug, PartialEq)]
pub enum UpdateRoutingProfileDefaultOutboundQueueError {
    /// <p>Request processing failed due to an error or failure with the service.</p>
    InternalService(String),
    /// <p>One or more of the specified parameters are not valid.</p>
    InvalidParameter(String),
    /// <p>The request is not valid.</p>
    InvalidRequest(String),
    /// <p>The specified resource was not found.</p>
    ResourceNotFound(String),
    /// <p>The throttling limit has been exceeded.</p>
    Throttling(String),
}

impl UpdateRoutingProfileDefaultOutboundQueueError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<UpdateRoutingProfileDefaultOutboundQueueError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InternalServiceException" => {
                    return RusotoError::Service(
                        UpdateRoutingProfileDefaultOutboundQueueError::InternalService(err.msg),
                    )
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(
                        UpdateRoutingProfileDefaultOutboundQueueError::InvalidParameter(err.msg),
                    )
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(
                        UpdateRoutingProfileDefaultOutboundQueueError::InvalidRequest(err.msg),
                    )
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(
                        UpdateRoutingProfileDefaultOutboundQueueError::ResourceNotFound(err.msg),
                    )
                }
                "ThrottlingException" => {
                    return RusotoError::Service(
                        UpdateRoutingProfileDefaultOutboundQueueError::Throttling(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for UpdateRoutingProfileDefaultOutboundQueueError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UpdateRoutingProfileDefaultOutboundQueueError::InternalService(ref cause) => {
                write!(f, "{}", cause)
            }
            UpdateRoutingProfileDefaultOutboundQueueError::InvalidParameter(ref cause) => {
                write!(f, "{}", cause)
            }
            UpdateRoutingProfileDefaultOutboundQueueError::InvalidRequest(ref cause) => {
                write!(f, "{}", cause)
            }
            UpdateRoutingProfileDefaultOutboundQueueError::ResourceNotFound(ref cause) => {
                write!(f, "{}", cause)
            }
            UpdateRoutingProfileDefaultOutboundQueueError::Throttling(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for UpdateRoutingProfileDefaultOutboundQueueError {}
/// Errors returned by UpdateRoutingProfileName
#[derive(Debug, PartialEq)]
pub enum UpdateRoutingProfileNameError {
    /// <p>A resource with the specified name already exists.</p>
    DuplicateResource(String),
    /// <p>Request processing failed due to an error or failure with the service.</p>
    InternalService(String),
    /// <p>One or more of the specified parameters are not valid.</p>
    InvalidParameter(String),
    /// <p>The request is not valid.</p>
    InvalidRequest(String),
    /// <p>The specified resource was not found.</p>
    ResourceNotFound(String),
    /// <p>The throttling limit has been exceeded.</p>
    Throttling(String),
}

impl UpdateRoutingProfileNameError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdateRoutingProfileNameError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "DuplicateResourceException" => {
                    return RusotoError::Service(UpdateRoutingProfileNameError::DuplicateResource(
                        err.msg,
                    ))
                }
                "InternalServiceException" => {
                    return RusotoError::Service(UpdateRoutingProfileNameError::InternalService(
                        err.msg,
                    ))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(UpdateRoutingProfileNameError::InvalidParameter(
                        err.msg,
                    ))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(UpdateRoutingProfileNameError::InvalidRequest(
                        err.msg,
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(UpdateRoutingProfileNameError::ResourceNotFound(
                        err.msg,
                    ))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(UpdateRoutingProfileNameError::Throttling(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for UpdateRoutingProfileNameError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UpdateRoutingProfileNameError::DuplicateResource(ref cause) => write!(f, "{}", cause),
            UpdateRoutingProfileNameError::InternalService(ref cause) => write!(f, "{}", cause),
            UpdateRoutingProfileNameError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            UpdateRoutingProfileNameError::InvalidRequest(ref cause) => write!(f, "{}", cause),
            UpdateRoutingProfileNameError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            UpdateRoutingProfileNameError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for UpdateRoutingProfileNameError {}
/// Errors returned by UpdateRoutingProfileQueues
#[derive(Debug, PartialEq)]
pub enum UpdateRoutingProfileQueuesError {
    /// <p>Request processing failed due to an error or failure with the service.</p>
    InternalService(String),
    /// <p>One or more of the specified parameters are not valid.</p>
    InvalidParameter(String),
    /// <p>The request is not valid.</p>
    InvalidRequest(String),
    /// <p>The specified resource was not found.</p>
    ResourceNotFound(String),
    /// <p>The throttling limit has been exceeded.</p>
    Throttling(String),
}

impl UpdateRoutingProfileQueuesError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<UpdateRoutingProfileQueuesError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InternalServiceException" => {
                    return RusotoError::Service(UpdateRoutingProfileQueuesError::InternalService(
                        err.msg,
                    ))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(UpdateRoutingProfileQueuesError::InvalidParameter(
                        err.msg,
                    ))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(UpdateRoutingProfileQueuesError::InvalidRequest(
                        err.msg,
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(UpdateRoutingProfileQueuesError::ResourceNotFound(
                        err.msg,
                    ))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(UpdateRoutingProfileQueuesError::Throttling(
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
impl fmt::Display for UpdateRoutingProfileQueuesError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UpdateRoutingProfileQueuesError::InternalService(ref cause) => write!(f, "{}", cause),
            UpdateRoutingProfileQueuesError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            UpdateRoutingProfileQueuesError::InvalidRequest(ref cause) => write!(f, "{}", cause),
            UpdateRoutingProfileQueuesError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            UpdateRoutingProfileQueuesError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for UpdateRoutingProfileQueuesError {}
/// Errors returned by UpdateUserHierarchy
#[derive(Debug, PartialEq)]
pub enum UpdateUserHierarchyError {
    /// <p>Request processing failed due to an error or failure with the service.</p>
    InternalService(String),
    /// <p>One or more of the specified parameters are not valid.</p>
    InvalidParameter(String),
    /// <p>The request is not valid.</p>
    InvalidRequest(String),
    /// <p>The specified resource was not found.</p>
    ResourceNotFound(String),
    /// <p>The throttling limit has been exceeded.</p>
    Throttling(String),
}

impl UpdateUserHierarchyError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdateUserHierarchyError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InternalServiceException" => {
                    return RusotoError::Service(UpdateUserHierarchyError::InternalService(err.msg))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(UpdateUserHierarchyError::InvalidParameter(
                        err.msg,
                    ))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(UpdateUserHierarchyError::InvalidRequest(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(UpdateUserHierarchyError::ResourceNotFound(
                        err.msg,
                    ))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(UpdateUserHierarchyError::Throttling(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for UpdateUserHierarchyError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UpdateUserHierarchyError::InternalService(ref cause) => write!(f, "{}", cause),
            UpdateUserHierarchyError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            UpdateUserHierarchyError::InvalidRequest(ref cause) => write!(f, "{}", cause),
            UpdateUserHierarchyError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            UpdateUserHierarchyError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for UpdateUserHierarchyError {}
/// Errors returned by UpdateUserHierarchyGroupName
#[derive(Debug, PartialEq)]
pub enum UpdateUserHierarchyGroupNameError {
    /// <p>A resource with the specified name already exists.</p>
    DuplicateResource(String),
    /// <p>Request processing failed due to an error or failure with the service.</p>
    InternalService(String),
    /// <p>One or more of the specified parameters are not valid.</p>
    InvalidParameter(String),
    /// <p>The request is not valid.</p>
    InvalidRequest(String),
    /// <p>The specified resource was not found.</p>
    ResourceNotFound(String),
    /// <p>The throttling limit has been exceeded.</p>
    Throttling(String),
}

impl UpdateUserHierarchyGroupNameError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<UpdateUserHierarchyGroupNameError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "DuplicateResourceException" => {
                    return RusotoError::Service(
                        UpdateUserHierarchyGroupNameError::DuplicateResource(err.msg),
                    )
                }
                "InternalServiceException" => {
                    return RusotoError::Service(
                        UpdateUserHierarchyGroupNameError::InternalService(err.msg),
                    )
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(
                        UpdateUserHierarchyGroupNameError::InvalidParameter(err.msg),
                    )
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(UpdateUserHierarchyGroupNameError::InvalidRequest(
                        err.msg,
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(
                        UpdateUserHierarchyGroupNameError::ResourceNotFound(err.msg),
                    )
                }
                "ThrottlingException" => {
                    return RusotoError::Service(UpdateUserHierarchyGroupNameError::Throttling(
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
impl fmt::Display for UpdateUserHierarchyGroupNameError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UpdateUserHierarchyGroupNameError::DuplicateResource(ref cause) => {
                write!(f, "{}", cause)
            }
            UpdateUserHierarchyGroupNameError::InternalService(ref cause) => write!(f, "{}", cause),
            UpdateUserHierarchyGroupNameError::InvalidParameter(ref cause) => {
                write!(f, "{}", cause)
            }
            UpdateUserHierarchyGroupNameError::InvalidRequest(ref cause) => write!(f, "{}", cause),
            UpdateUserHierarchyGroupNameError::ResourceNotFound(ref cause) => {
                write!(f, "{}", cause)
            }
            UpdateUserHierarchyGroupNameError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for UpdateUserHierarchyGroupNameError {}
/// Errors returned by UpdateUserHierarchyStructure
#[derive(Debug, PartialEq)]
pub enum UpdateUserHierarchyStructureError {
    /// <p>Request processing failed due to an error or failure with the service.</p>
    InternalService(String),
    /// <p>One or more of the specified parameters are not valid.</p>
    InvalidParameter(String),
    /// <p>The request is not valid.</p>
    InvalidRequest(String),
    /// <p>That resource is already in use. Please try another.</p>
    ResourceInUse(String),
    /// <p>The specified resource was not found.</p>
    ResourceNotFound(String),
    /// <p>The throttling limit has been exceeded.</p>
    Throttling(String),
}

impl UpdateUserHierarchyStructureError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<UpdateUserHierarchyStructureError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InternalServiceException" => {
                    return RusotoError::Service(
                        UpdateUserHierarchyStructureError::InternalService(err.msg),
                    )
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(
                        UpdateUserHierarchyStructureError::InvalidParameter(err.msg),
                    )
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(UpdateUserHierarchyStructureError::InvalidRequest(
                        err.msg,
                    ))
                }
                "ResourceInUseException" => {
                    return RusotoError::Service(UpdateUserHierarchyStructureError::ResourceInUse(
                        err.msg,
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(
                        UpdateUserHierarchyStructureError::ResourceNotFound(err.msg),
                    )
                }
                "ThrottlingException" => {
                    return RusotoError::Service(UpdateUserHierarchyStructureError::Throttling(
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
impl fmt::Display for UpdateUserHierarchyStructureError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UpdateUserHierarchyStructureError::InternalService(ref cause) => write!(f, "{}", cause),
            UpdateUserHierarchyStructureError::InvalidParameter(ref cause) => {
                write!(f, "{}", cause)
            }
            UpdateUserHierarchyStructureError::InvalidRequest(ref cause) => write!(f, "{}", cause),
            UpdateUserHierarchyStructureError::ResourceInUse(ref cause) => write!(f, "{}", cause),
            UpdateUserHierarchyStructureError::ResourceNotFound(ref cause) => {
                write!(f, "{}", cause)
            }
            UpdateUserHierarchyStructureError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for UpdateUserHierarchyStructureError {}
/// Errors returned by UpdateUserIdentityInfo
#[derive(Debug, PartialEq)]
pub enum UpdateUserIdentityInfoError {
    /// <p>Request processing failed due to an error or failure with the service.</p>
    InternalService(String),
    /// <p>One or more of the specified parameters are not valid.</p>
    InvalidParameter(String),
    /// <p>The request is not valid.</p>
    InvalidRequest(String),
    /// <p>The specified resource was not found.</p>
    ResourceNotFound(String),
    /// <p>The throttling limit has been exceeded.</p>
    Throttling(String),
}

impl UpdateUserIdentityInfoError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdateUserIdentityInfoError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InternalServiceException" => {
                    return RusotoError::Service(UpdateUserIdentityInfoError::InternalService(
                        err.msg,
                    ))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(UpdateUserIdentityInfoError::InvalidParameter(
                        err.msg,
                    ))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(UpdateUserIdentityInfoError::InvalidRequest(
                        err.msg,
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(UpdateUserIdentityInfoError::ResourceNotFound(
                        err.msg,
                    ))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(UpdateUserIdentityInfoError::Throttling(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for UpdateUserIdentityInfoError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UpdateUserIdentityInfoError::InternalService(ref cause) => write!(f, "{}", cause),
            UpdateUserIdentityInfoError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            UpdateUserIdentityInfoError::InvalidRequest(ref cause) => write!(f, "{}", cause),
            UpdateUserIdentityInfoError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            UpdateUserIdentityInfoError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for UpdateUserIdentityInfoError {}
/// Errors returned by UpdateUserPhoneConfig
#[derive(Debug, PartialEq)]
pub enum UpdateUserPhoneConfigError {
    /// <p>Request processing failed due to an error or failure with the service.</p>
    InternalService(String),
    /// <p>One or more of the specified parameters are not valid.</p>
    InvalidParameter(String),
    /// <p>The request is not valid.</p>
    InvalidRequest(String),
    /// <p>The specified resource was not found.</p>
    ResourceNotFound(String),
    /// <p>The throttling limit has been exceeded.</p>
    Throttling(String),
}

impl UpdateUserPhoneConfigError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdateUserPhoneConfigError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InternalServiceException" => {
                    return RusotoError::Service(UpdateUserPhoneConfigError::InternalService(
                        err.msg,
                    ))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(UpdateUserPhoneConfigError::InvalidParameter(
                        err.msg,
                    ))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(UpdateUserPhoneConfigError::InvalidRequest(
                        err.msg,
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(UpdateUserPhoneConfigError::ResourceNotFound(
                        err.msg,
                    ))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(UpdateUserPhoneConfigError::Throttling(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for UpdateUserPhoneConfigError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UpdateUserPhoneConfigError::InternalService(ref cause) => write!(f, "{}", cause),
            UpdateUserPhoneConfigError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            UpdateUserPhoneConfigError::InvalidRequest(ref cause) => write!(f, "{}", cause),
            UpdateUserPhoneConfigError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            UpdateUserPhoneConfigError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for UpdateUserPhoneConfigError {}
/// Errors returned by UpdateUserRoutingProfile
#[derive(Debug, PartialEq)]
pub enum UpdateUserRoutingProfileError {
    /// <p>Request processing failed due to an error or failure with the service.</p>
    InternalService(String),
    /// <p>One or more of the specified parameters are not valid.</p>
    InvalidParameter(String),
    /// <p>The request is not valid.</p>
    InvalidRequest(String),
    /// <p>The specified resource was not found.</p>
    ResourceNotFound(String),
    /// <p>The throttling limit has been exceeded.</p>
    Throttling(String),
}

impl UpdateUserRoutingProfileError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdateUserRoutingProfileError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InternalServiceException" => {
                    return RusotoError::Service(UpdateUserRoutingProfileError::InternalService(
                        err.msg,
                    ))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(UpdateUserRoutingProfileError::InvalidParameter(
                        err.msg,
                    ))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(UpdateUserRoutingProfileError::InvalidRequest(
                        err.msg,
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(UpdateUserRoutingProfileError::ResourceNotFound(
                        err.msg,
                    ))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(UpdateUserRoutingProfileError::Throttling(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for UpdateUserRoutingProfileError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UpdateUserRoutingProfileError::InternalService(ref cause) => write!(f, "{}", cause),
            UpdateUserRoutingProfileError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            UpdateUserRoutingProfileError::InvalidRequest(ref cause) => write!(f, "{}", cause),
            UpdateUserRoutingProfileError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            UpdateUserRoutingProfileError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for UpdateUserRoutingProfileError {}
/// Errors returned by UpdateUserSecurityProfiles
#[derive(Debug, PartialEq)]
pub enum UpdateUserSecurityProfilesError {
    /// <p>Request processing failed due to an error or failure with the service.</p>
    InternalService(String),
    /// <p>One or more of the specified parameters are not valid.</p>
    InvalidParameter(String),
    /// <p>The request is not valid.</p>
    InvalidRequest(String),
    /// <p>The specified resource was not found.</p>
    ResourceNotFound(String),
    /// <p>The throttling limit has been exceeded.</p>
    Throttling(String),
}

impl UpdateUserSecurityProfilesError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<UpdateUserSecurityProfilesError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InternalServiceException" => {
                    return RusotoError::Service(UpdateUserSecurityProfilesError::InternalService(
                        err.msg,
                    ))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(UpdateUserSecurityProfilesError::InvalidParameter(
                        err.msg,
                    ))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(UpdateUserSecurityProfilesError::InvalidRequest(
                        err.msg,
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(UpdateUserSecurityProfilesError::ResourceNotFound(
                        err.msg,
                    ))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(UpdateUserSecurityProfilesError::Throttling(
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
impl fmt::Display for UpdateUserSecurityProfilesError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UpdateUserSecurityProfilesError::InternalService(ref cause) => write!(f, "{}", cause),
            UpdateUserSecurityProfilesError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            UpdateUserSecurityProfilesError::InvalidRequest(ref cause) => write!(f, "{}", cause),
            UpdateUserSecurityProfilesError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            UpdateUserSecurityProfilesError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for UpdateUserSecurityProfilesError {}
/// Trait representing the capabilities of the Amazon Connect API. Amazon Connect clients implement this trait.
#[async_trait]
pub trait Connect {
    /// <p>This API is in preview release for Amazon Connect and is subject to change.</p> <p>Associates an approved origin to an Amazon Connect instance.</p>
    async fn associate_approved_origin(
        &self,
        input: AssociateApprovedOriginRequest,
    ) -> Result<(), RusotoError<AssociateApprovedOriginError>>;

    /// <p>This API is in preview release for Amazon Connect and is subject to change.</p> <p>Associates a storage resource type for the first time. You can only associate one type of storage configuration in a single call. This means, for example, that you can't define an instance with multiple S3 buckets for storing chat transcripts.</p> <p>This API does not create a resource that doesn't exist. It only associates it to the instance. Ensure that the resource being specified in the storage configuration, like an Amazon S3 bucket, exists when being used for association.</p>
    async fn associate_instance_storage_config(
        &self,
        input: AssociateInstanceStorageConfigRequest,
    ) -> Result<
        AssociateInstanceStorageConfigResponse,
        RusotoError<AssociateInstanceStorageConfigError>,
    >;

    /// <p>This API is in preview release for Amazon Connect and is subject to change.</p> <p>Allows the specified Amazon Connect instance to access the specified Lambda function.</p>
    async fn associate_lambda_function(
        &self,
        input: AssociateLambdaFunctionRequest,
    ) -> Result<(), RusotoError<AssociateLambdaFunctionError>>;

    /// <p>This API is in preview release for Amazon Connect and is subject to change.</p> <p>Allows the specified Amazon Connect instance to access the specified Amazon Lex bot.</p>
    async fn associate_lex_bot(
        &self,
        input: AssociateLexBotRequest,
    ) -> Result<(), RusotoError<AssociateLexBotError>>;

    /// <p>Associates a set of queues with a routing profile.</p>
    async fn associate_routing_profile_queues(
        &self,
        input: AssociateRoutingProfileQueuesRequest,
    ) -> Result<(), RusotoError<AssociateRoutingProfileQueuesError>>;

    /// <p>This API is in preview release for Amazon Connect and is subject to change.</p> <p>Associates a security key to the instance.</p>
    async fn associate_security_key(
        &self,
        input: AssociateSecurityKeyRequest,
    ) -> Result<AssociateSecurityKeyResponse, RusotoError<AssociateSecurityKeyError>>;

    /// <p>Creates a contact flow for the specified Amazon Connect instance.</p> <p>You can also create and update contact flows using the <a href="https://docs.aws.amazon.com/connect/latest/adminguide/flow-language.html">Amazon Connect Flow language</a>.</p>
    async fn create_contact_flow(
        &self,
        input: CreateContactFlowRequest,
    ) -> Result<CreateContactFlowResponse, RusotoError<CreateContactFlowError>>;

    /// <p>This API is in preview release for Amazon Connect and is subject to change.</p> <p>Initiates an Amazon Connect instance with all the supported channels enabled. It does not attach any storage (such as Amazon S3, or Kinesis) or allow for any configurations on features such as Contact Lens for Amazon Connect. </p>
    async fn create_instance(
        &self,
        input: CreateInstanceRequest,
    ) -> Result<CreateInstanceResponse, RusotoError<CreateInstanceError>>;

    /// <p>This API is in preview release for Amazon Connect and is subject to change.</p> <p>Create an AppIntegration association with an Amazon Connect instance.</p>
    async fn create_integration_association(
        &self,
        input: CreateIntegrationAssociationRequest,
    ) -> Result<CreateIntegrationAssociationResponse, RusotoError<CreateIntegrationAssociationError>>;

    /// <p>This API is in preview release for Amazon Connect and is subject to change.</p> <p>Creates a quick connect for the specified Amazon Connect instance.</p>
    async fn create_quick_connect(
        &self,
        input: CreateQuickConnectRequest,
    ) -> Result<CreateQuickConnectResponse, RusotoError<CreateQuickConnectError>>;

    /// <p>Creates a new routing profile.</p>
    async fn create_routing_profile(
        &self,
        input: CreateRoutingProfileRequest,
    ) -> Result<CreateRoutingProfileResponse, RusotoError<CreateRoutingProfileError>>;

    /// <p>This API is in preview release for Amazon Connect and is subject to change.</p> <p>Creates a use case for an AppIntegration association.</p>
    async fn create_use_case(
        &self,
        input: CreateUseCaseRequest,
    ) -> Result<CreateUseCaseResponse, RusotoError<CreateUseCaseError>>;

    /// <p>Creates a user account for the specified Amazon Connect instance.</p> <p>For information about how to create user accounts using the Amazon Connect console, see <a href="https://docs.aws.amazon.com/connect/latest/adminguide/user-management.html">Add Users</a> in the <i>Amazon Connect Administrator Guide</i>.</p>
    async fn create_user(
        &self,
        input: CreateUserRequest,
    ) -> Result<CreateUserResponse, RusotoError<CreateUserError>>;

    /// <p>Creates a new user hierarchy group.</p>
    async fn create_user_hierarchy_group(
        &self,
        input: CreateUserHierarchyGroupRequest,
    ) -> Result<CreateUserHierarchyGroupResponse, RusotoError<CreateUserHierarchyGroupError>>;

    /// <p>This API is in preview release for Amazon Connect and is subject to change.</p> <p>Deletes the Amazon Connect instance.</p>
    async fn delete_instance(
        &self,
        input: DeleteInstanceRequest,
    ) -> Result<(), RusotoError<DeleteInstanceError>>;

    /// <p>This API is in preview release for Amazon Connect and is subject to change.</p> <p>Deletes an AppIntegration association from an Amazon Connect instance. The association must not have any use cases associated with it.</p>
    async fn delete_integration_association(
        &self,
        input: DeleteIntegrationAssociationRequest,
    ) -> Result<(), RusotoError<DeleteIntegrationAssociationError>>;

    /// <p>This API is in preview release for Amazon Connect and is subject to change.</p> <p>Deletes a quick connect.</p>
    async fn delete_quick_connect(
        &self,
        input: DeleteQuickConnectRequest,
    ) -> Result<(), RusotoError<DeleteQuickConnectError>>;

    /// <p>This API is in preview release for Amazon Connect and is subject to change.</p> <p>Deletes a use case from an AppIntegration association.</p>
    async fn delete_use_case(
        &self,
        input: DeleteUseCaseRequest,
    ) -> Result<(), RusotoError<DeleteUseCaseError>>;

    /// <p>Deletes a user account from the specified Amazon Connect instance.</p> <p>For information about what happens to a user's data when their account is deleted, see <a href="https://docs.aws.amazon.com/connect/latest/adminguide/delete-users.html">Delete Users from Your Amazon Connect Instance</a> in the <i>Amazon Connect Administrator Guide</i>.</p>
    async fn delete_user(
        &self,
        input: DeleteUserRequest,
    ) -> Result<(), RusotoError<DeleteUserError>>;

    /// <p>Deletes an existing user hierarchy group. It must not be associated with any agents or have any active child groups.</p>
    async fn delete_user_hierarchy_group(
        &self,
        input: DeleteUserHierarchyGroupRequest,
    ) -> Result<(), RusotoError<DeleteUserHierarchyGroupError>>;

    /// <p>Describes the specified contact flow.</p> <p>You can also create and update contact flows using the <a href="https://docs.aws.amazon.com/connect/latest/adminguide/flow-language.html">Amazon Connect Flow language</a>.</p>
    async fn describe_contact_flow(
        &self,
        input: DescribeContactFlowRequest,
    ) -> Result<DescribeContactFlowResponse, RusotoError<DescribeContactFlowError>>;

    /// <p>This API is in preview release for Amazon Connect and is subject to change.</p> <p>Returns the current state of the specified instance identifier. It tracks the instance while it is being created and returns an error status if applicable. </p> <p>If an instance is not created successfully, the instance status reason field returns details relevant to the reason. The instance in a failed state is returned only for 24 hours after the CreateInstance API was invoked.</p>
    async fn describe_instance(
        &self,
        input: DescribeInstanceRequest,
    ) -> Result<DescribeInstanceResponse, RusotoError<DescribeInstanceError>>;

    /// <p>This API is in preview release for Amazon Connect and is subject to change.</p> <p>Describes the specified instance attribute.</p>
    async fn describe_instance_attribute(
        &self,
        input: DescribeInstanceAttributeRequest,
    ) -> Result<DescribeInstanceAttributeResponse, RusotoError<DescribeInstanceAttributeError>>;

    /// <p>This API is in preview release for Amazon Connect and is subject to change.</p> <p>Retrieves the current storage configurations for the specified resource type, association ID, and instance ID.</p>
    async fn describe_instance_storage_config(
        &self,
        input: DescribeInstanceStorageConfigRequest,
    ) -> Result<
        DescribeInstanceStorageConfigResponse,
        RusotoError<DescribeInstanceStorageConfigError>,
    >;

    /// <p>This API is in preview release for Amazon Connect and is subject to change.</p> <p>Describes the quick connect.</p>
    async fn describe_quick_connect(
        &self,
        input: DescribeQuickConnectRequest,
    ) -> Result<DescribeQuickConnectResponse, RusotoError<DescribeQuickConnectError>>;

    /// <p>Describes the specified routing profile.</p>
    async fn describe_routing_profile(
        &self,
        input: DescribeRoutingProfileRequest,
    ) -> Result<DescribeRoutingProfileResponse, RusotoError<DescribeRoutingProfileError>>;

    /// <p>Describes the specified user account. You can find the instance ID in the console (it’s the final part of the ARN). The console does not display the user IDs. Instead, list the users and note the IDs provided in the output.</p>
    async fn describe_user(
        &self,
        input: DescribeUserRequest,
    ) -> Result<DescribeUserResponse, RusotoError<DescribeUserError>>;

    /// <p>Describes the specified hierarchy group.</p>
    async fn describe_user_hierarchy_group(
        &self,
        input: DescribeUserHierarchyGroupRequest,
    ) -> Result<DescribeUserHierarchyGroupResponse, RusotoError<DescribeUserHierarchyGroupError>>;

    /// <p>Describes the hierarchy structure of the specified Amazon Connect instance.</p>
    async fn describe_user_hierarchy_structure(
        &self,
        input: DescribeUserHierarchyStructureRequest,
    ) -> Result<
        DescribeUserHierarchyStructureResponse,
        RusotoError<DescribeUserHierarchyStructureError>,
    >;

    /// <p>This API is in preview release for Amazon Connect and is subject to change.</p> <p>Revokes access to integrated applications from Amazon Connect.</p>
    async fn disassociate_approved_origin(
        &self,
        input: DisassociateApprovedOriginRequest,
    ) -> Result<(), RusotoError<DisassociateApprovedOriginError>>;

    /// <p>This API is in preview release for Amazon Connect and is subject to change.</p> <p>Removes the storage type configurations for the specified resource type and association ID.</p>
    async fn disassociate_instance_storage_config(
        &self,
        input: DisassociateInstanceStorageConfigRequest,
    ) -> Result<(), RusotoError<DisassociateInstanceStorageConfigError>>;

    /// <p>This API is in preview release for Amazon Connect and is subject to change.</p> <p>Remove the Lambda function from the drop-down options available in the relevant contact flow blocks.</p>
    async fn disassociate_lambda_function(
        &self,
        input: DisassociateLambdaFunctionRequest,
    ) -> Result<(), RusotoError<DisassociateLambdaFunctionError>>;

    /// <p>This API is in preview release for Amazon Connect and is subject to change.</p> <p>Revokes authorization from the specified instance to access the specified Amazon Lex bot.</p>
    async fn disassociate_lex_bot(
        &self,
        input: DisassociateLexBotRequest,
    ) -> Result<(), RusotoError<DisassociateLexBotError>>;

    /// <p>Disassociates a set of queues from a routing profile.</p>
    async fn disassociate_routing_profile_queues(
        &self,
        input: DisassociateRoutingProfileQueuesRequest,
    ) -> Result<(), RusotoError<DisassociateRoutingProfileQueuesError>>;

    /// <p>This API is in preview release for Amazon Connect and is subject to change.</p> <p>Deletes the specified security key.</p>
    async fn disassociate_security_key(
        &self,
        input: DisassociateSecurityKeyRequest,
    ) -> Result<(), RusotoError<DisassociateSecurityKeyError>>;

    /// <p>Retrieves the contact attributes for the specified contact.</p>
    async fn get_contact_attributes(
        &self,
        input: GetContactAttributesRequest,
    ) -> Result<GetContactAttributesResponse, RusotoError<GetContactAttributesError>>;

    /// <p>Gets the real-time metric data from the specified Amazon Connect instance.</p> <p>For a description of each metric, see <a href="https://docs.aws.amazon.com/connect/latest/adminguide/real-time-metrics-definitions.html">Real-time Metrics Definitions</a> in the <i>Amazon Connect Administrator Guide</i>.</p>
    async fn get_current_metric_data(
        &self,
        input: GetCurrentMetricDataRequest,
    ) -> Result<GetCurrentMetricDataResponse, RusotoError<GetCurrentMetricDataError>>;

    /// <p>Retrieves a token for federation.</p>
    async fn get_federation_token(
        &self,
        input: GetFederationTokenRequest,
    ) -> Result<GetFederationTokenResponse, RusotoError<GetFederationTokenError>>;

    /// <p>Gets historical metric data from the specified Amazon Connect instance.</p> <p>For a description of each historical metric, see <a href="https://docs.aws.amazon.com/connect/latest/adminguide/historical-metrics-definitions.html">Historical Metrics Definitions</a> in the <i>Amazon Connect Administrator Guide</i>.</p>
    async fn get_metric_data(
        &self,
        input: GetMetricDataRequest,
    ) -> Result<GetMetricDataResponse, RusotoError<GetMetricDataError>>;

    /// <p>This API is in preview release for Amazon Connect and is subject to change.</p> <p>Returns a paginated list of all approved origins associated with the instance.</p>
    async fn list_approved_origins(
        &self,
        input: ListApprovedOriginsRequest,
    ) -> Result<ListApprovedOriginsResponse, RusotoError<ListApprovedOriginsError>>;

    /// <p>Provides information about the contact flows for the specified Amazon Connect instance.</p> <p>You can also create and update contact flows using the <a href="https://docs.aws.amazon.com/connect/latest/adminguide/flow-language.html">Amazon Connect Flow language</a>.</p> <p>For more information about contact flows, see <a href="https://docs.aws.amazon.com/connect/latest/adminguide/concepts-contact-flows.html">Contact Flows</a> in the <i>Amazon Connect Administrator Guide</i>.</p>
    async fn list_contact_flows(
        &self,
        input: ListContactFlowsRequest,
    ) -> Result<ListContactFlowsResponse, RusotoError<ListContactFlowsError>>;

    /// <p>Provides information about the hours of operation for the specified Amazon Connect instance.</p> <p>For more information about hours of operation, see <a href="https://docs.aws.amazon.com/connect/latest/adminguide/set-hours-operation.html">Set the Hours of Operation for a Queue</a> in the <i>Amazon Connect Administrator Guide</i>.</p>
    async fn list_hours_of_operations(
        &self,
        input: ListHoursOfOperationsRequest,
    ) -> Result<ListHoursOfOperationsResponse, RusotoError<ListHoursOfOperationsError>>;

    /// <p>This API is in preview release for Amazon Connect and is subject to change.</p> <p>Returns a paginated list of all attribute types for the given instance.</p>
    async fn list_instance_attributes(
        &self,
        input: ListInstanceAttributesRequest,
    ) -> Result<ListInstanceAttributesResponse, RusotoError<ListInstanceAttributesError>>;

    /// <p>This API is in preview release for Amazon Connect and is subject to change.</p> <p>Returns a paginated list of storage configs for the identified instance and resource type.</p>
    async fn list_instance_storage_configs(
        &self,
        input: ListInstanceStorageConfigsRequest,
    ) -> Result<ListInstanceStorageConfigsResponse, RusotoError<ListInstanceStorageConfigsError>>;

    /// <p>This API is in preview release for Amazon Connect and is subject to change.</p> <p>Return a list of instances which are in active state, creation-in-progress state, and failed state. Instances that aren't successfully created (they are in a failed state) are returned only for 24 hours after the CreateInstance API was invoked.</p>
    async fn list_instances(
        &self,
        input: ListInstancesRequest,
    ) -> Result<ListInstancesResponse, RusotoError<ListInstancesError>>;

    /// <p>This API is in preview release for Amazon Connect and is subject to change.</p> <p>Provides summary information about the AppIntegration associations for the specified Amazon Connect instance.</p>
    async fn list_integration_associations(
        &self,
        input: ListIntegrationAssociationsRequest,
    ) -> Result<ListIntegrationAssociationsResponse, RusotoError<ListIntegrationAssociationsError>>;

    /// <p>This API is in preview release for Amazon Connect and is subject to change.</p> <p>Returns a paginated list of all the Lambda functions that show up in the drop-down options in the relevant contact flow blocks.</p>
    async fn list_lambda_functions(
        &self,
        input: ListLambdaFunctionsRequest,
    ) -> Result<ListLambdaFunctionsResponse, RusotoError<ListLambdaFunctionsError>>;

    /// <p>This API is in preview release for Amazon Connect and is subject to change.</p> <p>Returns a paginated list of all the Amazon Lex bots currently associated with the instance.</p>
    async fn list_lex_bots(
        &self,
        input: ListLexBotsRequest,
    ) -> Result<ListLexBotsResponse, RusotoError<ListLexBotsError>>;

    /// <p>Provides information about the phone numbers for the specified Amazon Connect instance. </p> <p>For more information about phone numbers, see <a href="https://docs.aws.amazon.com/connect/latest/adminguide/contact-center-phone-number.html">Set Up Phone Numbers for Your Contact Center</a> in the <i>Amazon Connect Administrator Guide</i>.</p>
    async fn list_phone_numbers(
        &self,
        input: ListPhoneNumbersRequest,
    ) -> Result<ListPhoneNumbersResponse, RusotoError<ListPhoneNumbersError>>;

    /// <p>Provides information about the prompts for the specified Amazon Connect instance.</p>
    async fn list_prompts(
        &self,
        input: ListPromptsRequest,
    ) -> Result<ListPromptsResponse, RusotoError<ListPromptsError>>;

    /// <p>Provides information about the queues for the specified Amazon Connect instance.</p> <p>For more information about queues, see <a href="https://docs.aws.amazon.com/connect/latest/adminguide/concepts-queues-standard-and-agent.html">Queues: Standard and Agent</a> in the <i>Amazon Connect Administrator Guide</i>.</p>
    async fn list_queues(
        &self,
        input: ListQueuesRequest,
    ) -> Result<ListQueuesResponse, RusotoError<ListQueuesError>>;

    /// <p>This API is in preview release for Amazon Connect and is subject to change.</p> <p>Provides information about the quick connects for the specified Amazon Connect instance. </p>
    async fn list_quick_connects(
        &self,
        input: ListQuickConnectsRequest,
    ) -> Result<ListQuickConnectsResponse, RusotoError<ListQuickConnectsError>>;

    /// <p>List the queues associated with a routing profile.</p>
    async fn list_routing_profile_queues(
        &self,
        input: ListRoutingProfileQueuesRequest,
    ) -> Result<ListRoutingProfileQueuesResponse, RusotoError<ListRoutingProfileQueuesError>>;

    /// <p>Provides summary information about the routing profiles for the specified Amazon Connect instance.</p> <p>For more information about routing profiles, see <a href="https://docs.aws.amazon.com/connect/latest/adminguide/concepts-routing.html">Routing Profiles</a> and <a href="https://docs.aws.amazon.com/connect/latest/adminguide/routing-profiles.html">Create a Routing Profile</a> in the <i>Amazon Connect Administrator Guide</i>.</p>
    async fn list_routing_profiles(
        &self,
        input: ListRoutingProfilesRequest,
    ) -> Result<ListRoutingProfilesResponse, RusotoError<ListRoutingProfilesError>>;

    /// <p>This API is in preview release for Amazon Connect and is subject to change.</p> <p>Returns a paginated list of all security keys associated with the instance.</p>
    async fn list_security_keys(
        &self,
        input: ListSecurityKeysRequest,
    ) -> Result<ListSecurityKeysResponse, RusotoError<ListSecurityKeysError>>;

    /// <p>Provides summary information about the security profiles for the specified Amazon Connect instance.</p> <p>For more information about security profiles, see <a href="https://docs.aws.amazon.com/connect/latest/adminguide/connect-security-profiles.html">Security Profiles</a> in the <i>Amazon Connect Administrator Guide</i>.</p>
    async fn list_security_profiles(
        &self,
        input: ListSecurityProfilesRequest,
    ) -> Result<ListSecurityProfilesResponse, RusotoError<ListSecurityProfilesError>>;

    /// <p>Lists the tags for the specified resource.</p> <p>For sample policies that use tags, see <a href="https://docs.aws.amazon.com/connect/latest/adminguide/security_iam_id-based-policy-examples.html">Amazon Connect Identity-Based Policy Examples</a> in the <i>Amazon Connect Administrator Guide</i>.</p>
    async fn list_tags_for_resource(
        &self,
        input: ListTagsForResourceRequest,
    ) -> Result<ListTagsForResourceResponse, RusotoError<ListTagsForResourceError>>;

    /// <p>This API is in preview release for Amazon Connect and is subject to change.</p> <p>List the use cases. </p>
    async fn list_use_cases(
        &self,
        input: ListUseCasesRequest,
    ) -> Result<ListUseCasesResponse, RusotoError<ListUseCasesError>>;

    /// <p>Provides summary information about the hierarchy groups for the specified Amazon Connect instance.</p> <p>For more information about agent hierarchies, see <a href="https://docs.aws.amazon.com/connect/latest/adminguide/agent-hierarchy.html">Set Up Agent Hierarchies</a> in the <i>Amazon Connect Administrator Guide</i>.</p>
    async fn list_user_hierarchy_groups(
        &self,
        input: ListUserHierarchyGroupsRequest,
    ) -> Result<ListUserHierarchyGroupsResponse, RusotoError<ListUserHierarchyGroupsError>>;

    /// <p>Provides summary information about the users for the specified Amazon Connect instance.</p>
    async fn list_users(
        &self,
        input: ListUsersRequest,
    ) -> Result<ListUsersResponse, RusotoError<ListUsersError>>;

    /// <p>When a contact is being recorded, and the recording has been suspended using SuspendContactRecording, this API resumes recording the call.</p> <p>Only voice recordings are supported at this time.</p>
    async fn resume_contact_recording(
        &self,
        input: ResumeContactRecordingRequest,
    ) -> Result<ResumeContactRecordingResponse, RusotoError<ResumeContactRecordingError>>;

    /// <p>Initiates a contact flow to start a new chat for the customer. Response of this API provides a token required to obtain credentials from the <a href="https://docs.aws.amazon.com/connect-participant/latest/APIReference/API_CreateParticipantConnection.html">CreateParticipantConnection</a> API in the Amazon Connect Participant Service.</p> <p>When a new chat contact is successfully created, clients need to subscribe to the participant’s connection for the created chat within 5 minutes. This is achieved by invoking <a href="https://docs.aws.amazon.com/connect-participant/latest/APIReference/API_CreateParticipantConnection.html">CreateParticipantConnection</a> with WEBSOCKET and CONNECTION_CREDENTIALS. </p> <p>A 429 error occurs in two situations:</p> <ul> <li> <p>API rate limit is exceeded. API TPS throttling returns a <code>TooManyRequests</code> exception from the API Gateway.</p> </li> <li> <p>The <a href="https://docs.aws.amazon.com/connect/latest/adminguide/amazon-connect-service-limits.html">quota for concurrent active chats</a> is exceeded. Active chat throttling returns a <code>LimitExceededException</code>.</p> </li> </ul> <p>For more information about how chat works, see <a href="https://docs.aws.amazon.com/connect/latest/adminguide/chat.html">Chat</a> in the <i>Amazon Connect Administrator Guide</i>.</p>
    async fn start_chat_contact(
        &self,
        input: StartChatContactRequest,
    ) -> Result<StartChatContactResponse, RusotoError<StartChatContactError>>;

    /// <p>This API starts recording the contact when the agent joins the call. StartContactRecording is a one-time action. For example, if you use StopContactRecording to stop recording an ongoing call, you can't use StartContactRecording to restart it. For scenarios where the recording has started and you want to suspend and resume it, such as when collecting sensitive information (for example, a credit card number), use SuspendContactRecording and ResumeContactRecording.</p> <p>You can use this API to override the recording behavior configured in the <a href="https://docs.aws.amazon.com/connect/latest/adminguide/set-recording-behavior.html">Set recording behavior</a> block.</p> <p>Only voice recordings are supported at this time.</p>
    async fn start_contact_recording(
        &self,
        input: StartContactRecordingRequest,
    ) -> Result<StartContactRecordingResponse, RusotoError<StartContactRecordingError>>;

    /// <p><p>This API places an outbound call to a contact, and then initiates the contact flow. It performs the actions in the contact flow that&#39;s specified (in <code>ContactFlowId</code>).</p> <p>Agents are not involved in initiating the outbound API (that is, dialing the contact). If the contact flow places an outbound call to a contact, and then puts the contact in queue, that&#39;s when the call is routed to the agent, like any other inbound case.</p> <p>There is a 60 second dialing timeout for this operation. If the call is not connected after 60 seconds, it fails.</p> <note> <p>UK numbers with a 447 prefix are not allowed by default. Before you can dial these UK mobile numbers, you must submit a service quota increase request. For more information, see <a href="https://docs.aws.amazon.com/connect/latest/adminguide/amazon-connect-service-limits.html">Amazon Connect Service Quotas</a> in the <i>Amazon Connect Administrator Guide</i>. </p> </note></p>
    async fn start_outbound_voice_contact(
        &self,
        input: StartOutboundVoiceContactRequest,
    ) -> Result<StartOutboundVoiceContactResponse, RusotoError<StartOutboundVoiceContactError>>;

    /// <p>Initiates a contact flow to start a new task.</p>
    async fn start_task_contact(
        &self,
        input: StartTaskContactRequest,
    ) -> Result<StartTaskContactResponse, RusotoError<StartTaskContactError>>;

    /// <p>Ends the specified contact.</p>
    async fn stop_contact(
        &self,
        input: StopContactRequest,
    ) -> Result<StopContactResponse, RusotoError<StopContactError>>;

    /// <p>When a contact is being recorded, this API stops recording the call. StopContactRecording is a one-time action. If you use StopContactRecording to stop recording an ongoing call, you can't use StartContactRecording to restart it. For scenarios where the recording has started and you want to suspend it for sensitive information (for example, to collect a credit card number), and then restart it, use SuspendContactRecording and ResumeContactRecording.</p> <p>Only voice recordings are supported at this time.</p>
    async fn stop_contact_recording(
        &self,
        input: StopContactRecordingRequest,
    ) -> Result<StopContactRecordingResponse, RusotoError<StopContactRecordingError>>;

    /// <p>When a contact is being recorded, this API suspends recording the call. For example, you might suspend the call recording while collecting sensitive information, such as a credit card number. Then use ResumeContactRecording to restart recording. </p> <p>The period of time that the recording is suspended is filled with silence in the final recording. </p> <p>Only voice recordings are supported at this time.</p>
    async fn suspend_contact_recording(
        &self,
        input: SuspendContactRecordingRequest,
    ) -> Result<SuspendContactRecordingResponse, RusotoError<SuspendContactRecordingError>>;

    /// <p>Adds the specified tags to the specified resource.</p> <p>The supported resource types are users, routing profiles, quick connects, and contact flows.</p> <p>For sample policies that use tags, see <a href="https://docs.aws.amazon.com/connect/latest/adminguide/security_iam_id-based-policy-examples.html">Amazon Connect Identity-Based Policy Examples</a> in the <i>Amazon Connect Administrator Guide</i>.</p>
    async fn tag_resource(
        &self,
        input: TagResourceRequest,
    ) -> Result<(), RusotoError<TagResourceError>>;

    /// <p>Removes the specified tags from the specified resource.</p>
    async fn untag_resource(
        &self,
        input: UntagResourceRequest,
    ) -> Result<(), RusotoError<UntagResourceError>>;

    /// <p>Creates or updates the contact attributes associated with the specified contact.</p> <p>You can add or update attributes for both ongoing and completed contacts. For example, you can update the customer's name or the reason the customer called while the call is active, or add notes about steps that the agent took during the call that are displayed to the next agent that takes the call. You can also update attributes for a contact using data from your CRM application and save the data with the contact in Amazon Connect. You could also flag calls for additional analysis, such as legal review or identifying abusive callers.</p> <p>Contact attributes are available in Amazon Connect for 24 months, and are then deleted.</p> <p> <b>Important:</b> You cannot use the operation to update attributes for contacts that occurred prior to the release of the API, September 12, 2018. You can update attributes only for contacts that started after the release of the API. If you attempt to update attributes for a contact that occurred prior to the release of the API, a 400 error is returned. This applies also to queued callbacks that were initiated prior to the release of the API but are still active in your instance.</p>
    async fn update_contact_attributes(
        &self,
        input: UpdateContactAttributesRequest,
    ) -> Result<UpdateContactAttributesResponse, RusotoError<UpdateContactAttributesError>>;

    /// <p>Updates the specified contact flow.</p> <p>You can also create and update contact flows using the <a href="https://docs.aws.amazon.com/connect/latest/adminguide/flow-language.html">Amazon Connect Flow language</a>.</p>
    async fn update_contact_flow_content(
        &self,
        input: UpdateContactFlowContentRequest,
    ) -> Result<(), RusotoError<UpdateContactFlowContentError>>;

    /// <p>The name of the contact flow.</p> <p>You can also create and update contact flows using the <a href="https://docs.aws.amazon.com/connect/latest/adminguide/flow-language.html">Amazon Connect Flow language</a>.</p>
    async fn update_contact_flow_name(
        &self,
        input: UpdateContactFlowNameRequest,
    ) -> Result<(), RusotoError<UpdateContactFlowNameError>>;

    /// <p>This API is in preview release for Amazon Connect and is subject to change.</p> <p>Updates the value for the specified attribute type.</p>
    async fn update_instance_attribute(
        &self,
        input: UpdateInstanceAttributeRequest,
    ) -> Result<(), RusotoError<UpdateInstanceAttributeError>>;

    /// <p>This API is in preview release for Amazon Connect and is subject to change.</p> <p>Updates an existing configuration for a resource type. This API is idempotent.</p>
    async fn update_instance_storage_config(
        &self,
        input: UpdateInstanceStorageConfigRequest,
    ) -> Result<(), RusotoError<UpdateInstanceStorageConfigError>>;

    /// <p>This API is in preview release for Amazon Connect and is subject to change.</p> <p>Updates the configuration settings for the specified quick connect.</p>
    async fn update_quick_connect_config(
        &self,
        input: UpdateQuickConnectConfigRequest,
    ) -> Result<(), RusotoError<UpdateQuickConnectConfigError>>;

    /// <p>This API is in preview release for Amazon Connect and is subject to change.</p> <p>Updates the name and description of a quick connect. The request accepts the following data in JSON format. At least Name or Description must be provided.</p>
    async fn update_quick_connect_name(
        &self,
        input: UpdateQuickConnectNameRequest,
    ) -> Result<(), RusotoError<UpdateQuickConnectNameError>>;

    /// <p>Updates the channels that agents can handle in the Contact Control Panel (CCP) for a routing profile.</p>
    async fn update_routing_profile_concurrency(
        &self,
        input: UpdateRoutingProfileConcurrencyRequest,
    ) -> Result<(), RusotoError<UpdateRoutingProfileConcurrencyError>>;

    /// <p>Updates the default outbound queue of a routing profile.</p>
    async fn update_routing_profile_default_outbound_queue(
        &self,
        input: UpdateRoutingProfileDefaultOutboundQueueRequest,
    ) -> Result<(), RusotoError<UpdateRoutingProfileDefaultOutboundQueueError>>;

    /// <p>Updates the name and description of a routing profile. The request accepts the following data in JSON format. At least <code>Name</code> or <code>Description</code> must be provided.</p>
    async fn update_routing_profile_name(
        &self,
        input: UpdateRoutingProfileNameRequest,
    ) -> Result<(), RusotoError<UpdateRoutingProfileNameError>>;

    /// <p>Updates the properties associated with a set of queues for a routing profile.</p>
    async fn update_routing_profile_queues(
        &self,
        input: UpdateRoutingProfileQueuesRequest,
    ) -> Result<(), RusotoError<UpdateRoutingProfileQueuesError>>;

    /// <p>Assigns the specified hierarchy group to the specified user.</p>
    async fn update_user_hierarchy(
        &self,
        input: UpdateUserHierarchyRequest,
    ) -> Result<(), RusotoError<UpdateUserHierarchyError>>;

    /// <p>Updates the name of the user hierarchy group. </p>
    async fn update_user_hierarchy_group_name(
        &self,
        input: UpdateUserHierarchyGroupNameRequest,
    ) -> Result<(), RusotoError<UpdateUserHierarchyGroupNameError>>;

    /// <p>Updates the user hierarchy structure: add, remove, and rename user hierarchy levels.</p>
    async fn update_user_hierarchy_structure(
        &self,
        input: UpdateUserHierarchyStructureRequest,
    ) -> Result<(), RusotoError<UpdateUserHierarchyStructureError>>;

    /// <p><p>Updates the identity information for the specified user.</p> <important> <p>Someone with the ability to invoke <code>UpdateUserIndentityInfo</code> can change the login credentials of other users by changing their email address. This poses a security risk to your organization. They can change the email address of a user to the attacker&#39;s email address, and then reset the password through email. We strongly recommend limiting who has the ability to invoke <code>UpdateUserIndentityInfo</code>. For more information, see <a href="https://docs.aws.amazon.com/connect/latest/adminguide/security-profile-best-practices.html">Best Practices for Security Profiles</a> in the <i>Amazon Connect Administrator Guide</i>.</p> </important></p>
    async fn update_user_identity_info(
        &self,
        input: UpdateUserIdentityInfoRequest,
    ) -> Result<(), RusotoError<UpdateUserIdentityInfoError>>;

    /// <p>Updates the phone configuration settings for the specified user.</p>
    async fn update_user_phone_config(
        &self,
        input: UpdateUserPhoneConfigRequest,
    ) -> Result<(), RusotoError<UpdateUserPhoneConfigError>>;

    /// <p>Assigns the specified routing profile to the specified user.</p>
    async fn update_user_routing_profile(
        &self,
        input: UpdateUserRoutingProfileRequest,
    ) -> Result<(), RusotoError<UpdateUserRoutingProfileError>>;

    /// <p>Assigns the specified security profiles to the specified user.</p>
    async fn update_user_security_profiles(
        &self,
        input: UpdateUserSecurityProfilesRequest,
    ) -> Result<(), RusotoError<UpdateUserSecurityProfilesError>>;
}
/// A client for the Amazon Connect API.
#[derive(Clone)]
pub struct ConnectClient {
    client: Client,
    region: region::Region,
}

impl ConnectClient {
    /// Creates a client backed by the default tokio event loop.
    ///
    /// The client will use the default credentials provider and tls client.
    pub fn new(region: region::Region) -> ConnectClient {
        ConnectClient {
            client: Client::shared(),
            region,
        }
    }

    pub fn new_with<P, D>(
        request_dispatcher: D,
        credentials_provider: P,
        region: region::Region,
    ) -> ConnectClient
    where
        P: ProvideAwsCredentials + Send + Sync + 'static,
        D: DispatchSignedRequest + Send + Sync + 'static,
    {
        ConnectClient {
            client: Client::new_with(credentials_provider, request_dispatcher),
            region,
        }
    }

    pub fn new_with_client(client: Client, region: region::Region) -> ConnectClient {
        ConnectClient { client, region }
    }
}

#[async_trait]
impl Connect for ConnectClient {
    /// <p>This API is in preview release for Amazon Connect and is subject to change.</p> <p>Associates an approved origin to an Amazon Connect instance.</p>
    #[allow(unused_mut)]
    async fn associate_approved_origin(
        &self,
        input: AssociateApprovedOriginRequest,
    ) -> Result<(), RusotoError<AssociateApprovedOriginError>> {
        let request_uri = format!(
            "/instance/{instance_id}/approved-origin",
            instance_id = input.instance_id
        );

        let mut request = SignedRequest::new("PUT", "connect", &self.region, &request_uri);
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
            ::std::mem::drop(response);

            Ok(())
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(AssociateApprovedOriginError::from_response(response))
        }
    }

    /// <p>This API is in preview release for Amazon Connect and is subject to change.</p> <p>Associates a storage resource type for the first time. You can only associate one type of storage configuration in a single call. This means, for example, that you can't define an instance with multiple S3 buckets for storing chat transcripts.</p> <p>This API does not create a resource that doesn't exist. It only associates it to the instance. Ensure that the resource being specified in the storage configuration, like an Amazon S3 bucket, exists when being used for association.</p>
    #[allow(unused_mut)]
    async fn associate_instance_storage_config(
        &self,
        input: AssociateInstanceStorageConfigRequest,
    ) -> Result<
        AssociateInstanceStorageConfigResponse,
        RusotoError<AssociateInstanceStorageConfigError>,
    > {
        let request_uri = format!(
            "/instance/{instance_id}/storage-config",
            instance_id = input.instance_id
        );

        let mut request = SignedRequest::new("PUT", "connect", &self.region, &request_uri);
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
                .deserialize::<AssociateInstanceStorageConfigResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(AssociateInstanceStorageConfigError::from_response(response))
        }
    }

    /// <p>This API is in preview release for Amazon Connect and is subject to change.</p> <p>Allows the specified Amazon Connect instance to access the specified Lambda function.</p>
    #[allow(unused_mut)]
    async fn associate_lambda_function(
        &self,
        input: AssociateLambdaFunctionRequest,
    ) -> Result<(), RusotoError<AssociateLambdaFunctionError>> {
        let request_uri = format!(
            "/instance/{instance_id}/lambda-function",
            instance_id = input.instance_id
        );

        let mut request = SignedRequest::new("PUT", "connect", &self.region, &request_uri);
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
            ::std::mem::drop(response);

            Ok(())
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(AssociateLambdaFunctionError::from_response(response))
        }
    }

    /// <p>This API is in preview release for Amazon Connect and is subject to change.</p> <p>Allows the specified Amazon Connect instance to access the specified Amazon Lex bot.</p>
    #[allow(unused_mut)]
    async fn associate_lex_bot(
        &self,
        input: AssociateLexBotRequest,
    ) -> Result<(), RusotoError<AssociateLexBotError>> {
        let request_uri = format!(
            "/instance/{instance_id}/lex-bot",
            instance_id = input.instance_id
        );

        let mut request = SignedRequest::new("PUT", "connect", &self.region, &request_uri);
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
            ::std::mem::drop(response);

            Ok(())
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(AssociateLexBotError::from_response(response))
        }
    }

    /// <p>Associates a set of queues with a routing profile.</p>
    #[allow(unused_mut)]
    async fn associate_routing_profile_queues(
        &self,
        input: AssociateRoutingProfileQueuesRequest,
    ) -> Result<(), RusotoError<AssociateRoutingProfileQueuesError>> {
        let request_uri = format!(
            "/routing-profiles/{instance_id}/{routing_profile_id}/associate-queues",
            instance_id = input.instance_id,
            routing_profile_id = input.routing_profile_id
        );

        let mut request = SignedRequest::new("POST", "connect", &self.region, &request_uri);
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
            ::std::mem::drop(response);

            Ok(())
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(AssociateRoutingProfileQueuesError::from_response(response))
        }
    }

    /// <p>This API is in preview release for Amazon Connect and is subject to change.</p> <p>Associates a security key to the instance.</p>
    #[allow(unused_mut)]
    async fn associate_security_key(
        &self,
        input: AssociateSecurityKeyRequest,
    ) -> Result<AssociateSecurityKeyResponse, RusotoError<AssociateSecurityKeyError>> {
        let request_uri = format!(
            "/instance/{instance_id}/security-key",
            instance_id = input.instance_id
        );

        let mut request = SignedRequest::new("PUT", "connect", &self.region, &request_uri);
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
                .deserialize::<AssociateSecurityKeyResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(AssociateSecurityKeyError::from_response(response))
        }
    }

    /// <p>Creates a contact flow for the specified Amazon Connect instance.</p> <p>You can also create and update contact flows using the <a href="https://docs.aws.amazon.com/connect/latest/adminguide/flow-language.html">Amazon Connect Flow language</a>.</p>
    #[allow(unused_mut)]
    async fn create_contact_flow(
        &self,
        input: CreateContactFlowRequest,
    ) -> Result<CreateContactFlowResponse, RusotoError<CreateContactFlowError>> {
        let request_uri = format!(
            "/contact-flows/{instance_id}",
            instance_id = input.instance_id
        );

        let mut request = SignedRequest::new("PUT", "connect", &self.region, &request_uri);
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
                .deserialize::<CreateContactFlowResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(CreateContactFlowError::from_response(response))
        }
    }

    /// <p>This API is in preview release for Amazon Connect and is subject to change.</p> <p>Initiates an Amazon Connect instance with all the supported channels enabled. It does not attach any storage (such as Amazon S3, or Kinesis) or allow for any configurations on features such as Contact Lens for Amazon Connect. </p>
    #[allow(unused_mut)]
    async fn create_instance(
        &self,
        input: CreateInstanceRequest,
    ) -> Result<CreateInstanceResponse, RusotoError<CreateInstanceError>> {
        let request_uri = "/instance";

        let mut request = SignedRequest::new("PUT", "connect", &self.region, &request_uri);
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
                .deserialize::<CreateInstanceResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(CreateInstanceError::from_response(response))
        }
    }

    /// <p>This API is in preview release for Amazon Connect and is subject to change.</p> <p>Create an AppIntegration association with an Amazon Connect instance.</p>
    #[allow(unused_mut)]
    async fn create_integration_association(
        &self,
        input: CreateIntegrationAssociationRequest,
    ) -> Result<CreateIntegrationAssociationResponse, RusotoError<CreateIntegrationAssociationError>>
    {
        let request_uri = format!(
            "/instance/{instance_id}/integration-associations",
            instance_id = input.instance_id
        );

        let mut request = SignedRequest::new("PUT", "connect", &self.region, &request_uri);
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
                .deserialize::<CreateIntegrationAssociationResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(CreateIntegrationAssociationError::from_response(response))
        }
    }

    /// <p>This API is in preview release for Amazon Connect and is subject to change.</p> <p>Creates a quick connect for the specified Amazon Connect instance.</p>
    #[allow(unused_mut)]
    async fn create_quick_connect(
        &self,
        input: CreateQuickConnectRequest,
    ) -> Result<CreateQuickConnectResponse, RusotoError<CreateQuickConnectError>> {
        let request_uri = format!(
            "/quick-connects/{instance_id}",
            instance_id = input.instance_id
        );

        let mut request = SignedRequest::new("PUT", "connect", &self.region, &request_uri);
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
                .deserialize::<CreateQuickConnectResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(CreateQuickConnectError::from_response(response))
        }
    }

    /// <p>Creates a new routing profile.</p>
    #[allow(unused_mut)]
    async fn create_routing_profile(
        &self,
        input: CreateRoutingProfileRequest,
    ) -> Result<CreateRoutingProfileResponse, RusotoError<CreateRoutingProfileError>> {
        let request_uri = format!(
            "/routing-profiles/{instance_id}",
            instance_id = input.instance_id
        );

        let mut request = SignedRequest::new("PUT", "connect", &self.region, &request_uri);
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
                .deserialize::<CreateRoutingProfileResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(CreateRoutingProfileError::from_response(response))
        }
    }

    /// <p>This API is in preview release for Amazon Connect and is subject to change.</p> <p>Creates a use case for an AppIntegration association.</p>
    #[allow(unused_mut)]
    async fn create_use_case(
        &self,
        input: CreateUseCaseRequest,
    ) -> Result<CreateUseCaseResponse, RusotoError<CreateUseCaseError>> {
        let request_uri = format!("/instance/{instance_id}/integration-associations/{integration_association_id}/use-cases", instance_id = input.instance_id, integration_association_id = input.integration_association_id);

        let mut request = SignedRequest::new("PUT", "connect", &self.region, &request_uri);
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
                .deserialize::<CreateUseCaseResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(CreateUseCaseError::from_response(response))
        }
    }

    /// <p>Creates a user account for the specified Amazon Connect instance.</p> <p>For information about how to create user accounts using the Amazon Connect console, see <a href="https://docs.aws.amazon.com/connect/latest/adminguide/user-management.html">Add Users</a> in the <i>Amazon Connect Administrator Guide</i>.</p>
    #[allow(unused_mut)]
    async fn create_user(
        &self,
        input: CreateUserRequest,
    ) -> Result<CreateUserResponse, RusotoError<CreateUserError>> {
        let request_uri = format!("/users/{instance_id}", instance_id = input.instance_id);

        let mut request = SignedRequest::new("PUT", "connect", &self.region, &request_uri);
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
                .deserialize::<CreateUserResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(CreateUserError::from_response(response))
        }
    }

    /// <p>Creates a new user hierarchy group.</p>
    #[allow(unused_mut)]
    async fn create_user_hierarchy_group(
        &self,
        input: CreateUserHierarchyGroupRequest,
    ) -> Result<CreateUserHierarchyGroupResponse, RusotoError<CreateUserHierarchyGroupError>> {
        let request_uri = format!(
            "/user-hierarchy-groups/{instance_id}",
            instance_id = input.instance_id
        );

        let mut request = SignedRequest::new("PUT", "connect", &self.region, &request_uri);
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
                .deserialize::<CreateUserHierarchyGroupResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(CreateUserHierarchyGroupError::from_response(response))
        }
    }

    /// <p>This API is in preview release for Amazon Connect and is subject to change.</p> <p>Deletes the Amazon Connect instance.</p>
    #[allow(unused_mut)]
    async fn delete_instance(
        &self,
        input: DeleteInstanceRequest,
    ) -> Result<(), RusotoError<DeleteInstanceError>> {
        let request_uri = format!("/instance/{instance_id}", instance_id = input.instance_id);

        let mut request = SignedRequest::new("DELETE", "connect", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            ::std::mem::drop(response);

            Ok(())
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(DeleteInstanceError::from_response(response))
        }
    }

    /// <p>This API is in preview release for Amazon Connect and is subject to change.</p> <p>Deletes an AppIntegration association from an Amazon Connect instance. The association must not have any use cases associated with it.</p>
    #[allow(unused_mut)]
    async fn delete_integration_association(
        &self,
        input: DeleteIntegrationAssociationRequest,
    ) -> Result<(), RusotoError<DeleteIntegrationAssociationError>> {
        let request_uri = format!(
            "/instance/{instance_id}/integration-associations/{integration_association_id}",
            instance_id = input.instance_id,
            integration_association_id = input.integration_association_id
        );

        let mut request = SignedRequest::new("DELETE", "connect", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            ::std::mem::drop(response);

            Ok(())
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(DeleteIntegrationAssociationError::from_response(response))
        }
    }

    /// <p>This API is in preview release for Amazon Connect and is subject to change.</p> <p>Deletes a quick connect.</p>
    #[allow(unused_mut)]
    async fn delete_quick_connect(
        &self,
        input: DeleteQuickConnectRequest,
    ) -> Result<(), RusotoError<DeleteQuickConnectError>> {
        let request_uri = format!(
            "/quick-connects/{instance_id}/{quick_connect_id}",
            instance_id = input.instance_id,
            quick_connect_id = input.quick_connect_id
        );

        let mut request = SignedRequest::new("DELETE", "connect", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            ::std::mem::drop(response);

            Ok(())
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(DeleteQuickConnectError::from_response(response))
        }
    }

    /// <p>This API is in preview release for Amazon Connect and is subject to change.</p> <p>Deletes a use case from an AppIntegration association.</p>
    #[allow(unused_mut)]
    async fn delete_use_case(
        &self,
        input: DeleteUseCaseRequest,
    ) -> Result<(), RusotoError<DeleteUseCaseError>> {
        let request_uri = format!("/instance/{instance_id}/integration-associations/{integration_association_id}/use-cases/{use_case_id}", instance_id = input.instance_id, integration_association_id = input.integration_association_id, use_case_id = input.use_case_id);

        let mut request = SignedRequest::new("DELETE", "connect", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            ::std::mem::drop(response);

            Ok(())
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(DeleteUseCaseError::from_response(response))
        }
    }

    /// <p>Deletes a user account from the specified Amazon Connect instance.</p> <p>For information about what happens to a user's data when their account is deleted, see <a href="https://docs.aws.amazon.com/connect/latest/adminguide/delete-users.html">Delete Users from Your Amazon Connect Instance</a> in the <i>Amazon Connect Administrator Guide</i>.</p>
    #[allow(unused_mut)]
    async fn delete_user(
        &self,
        input: DeleteUserRequest,
    ) -> Result<(), RusotoError<DeleteUserError>> {
        let request_uri = format!(
            "/users/{instance_id}/{user_id}",
            instance_id = input.instance_id,
            user_id = input.user_id
        );

        let mut request = SignedRequest::new("DELETE", "connect", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            ::std::mem::drop(response);

            Ok(())
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(DeleteUserError::from_response(response))
        }
    }

    /// <p>Deletes an existing user hierarchy group. It must not be associated with any agents or have any active child groups.</p>
    #[allow(unused_mut)]
    async fn delete_user_hierarchy_group(
        &self,
        input: DeleteUserHierarchyGroupRequest,
    ) -> Result<(), RusotoError<DeleteUserHierarchyGroupError>> {
        let request_uri = format!(
            "/user-hierarchy-groups/{instance_id}/{hierarchy_group_id}",
            hierarchy_group_id = input.hierarchy_group_id,
            instance_id = input.instance_id
        );

        let mut request = SignedRequest::new("DELETE", "connect", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            ::std::mem::drop(response);

            Ok(())
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(DeleteUserHierarchyGroupError::from_response(response))
        }
    }

    /// <p>Describes the specified contact flow.</p> <p>You can also create and update contact flows using the <a href="https://docs.aws.amazon.com/connect/latest/adminguide/flow-language.html">Amazon Connect Flow language</a>.</p>
    #[allow(unused_mut)]
    async fn describe_contact_flow(
        &self,
        input: DescribeContactFlowRequest,
    ) -> Result<DescribeContactFlowResponse, RusotoError<DescribeContactFlowError>> {
        let request_uri = format!(
            "/contact-flows/{instance_id}/{contact_flow_id}",
            contact_flow_id = input.contact_flow_id,
            instance_id = input.instance_id
        );

        let mut request = SignedRequest::new("GET", "connect", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<DescribeContactFlowResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(DescribeContactFlowError::from_response(response))
        }
    }

    /// <p>This API is in preview release for Amazon Connect and is subject to change.</p> <p>Returns the current state of the specified instance identifier. It tracks the instance while it is being created and returns an error status if applicable. </p> <p>If an instance is not created successfully, the instance status reason field returns details relevant to the reason. The instance in a failed state is returned only for 24 hours after the CreateInstance API was invoked.</p>
    #[allow(unused_mut)]
    async fn describe_instance(
        &self,
        input: DescribeInstanceRequest,
    ) -> Result<DescribeInstanceResponse, RusotoError<DescribeInstanceError>> {
        let request_uri = format!("/instance/{instance_id}", instance_id = input.instance_id);

        let mut request = SignedRequest::new("GET", "connect", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<DescribeInstanceResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(DescribeInstanceError::from_response(response))
        }
    }

    /// <p>This API is in preview release for Amazon Connect and is subject to change.</p> <p>Describes the specified instance attribute.</p>
    #[allow(unused_mut)]
    async fn describe_instance_attribute(
        &self,
        input: DescribeInstanceAttributeRequest,
    ) -> Result<DescribeInstanceAttributeResponse, RusotoError<DescribeInstanceAttributeError>>
    {
        let request_uri = format!(
            "/instance/{instance_id}/attribute/{attribute_type}",
            attribute_type = input.attribute_type,
            instance_id = input.instance_id
        );

        let mut request = SignedRequest::new("GET", "connect", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<DescribeInstanceAttributeResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(DescribeInstanceAttributeError::from_response(response))
        }
    }

    /// <p>This API is in preview release for Amazon Connect and is subject to change.</p> <p>Retrieves the current storage configurations for the specified resource type, association ID, and instance ID.</p>
    #[allow(unused_mut)]
    async fn describe_instance_storage_config(
        &self,
        input: DescribeInstanceStorageConfigRequest,
    ) -> Result<
        DescribeInstanceStorageConfigResponse,
        RusotoError<DescribeInstanceStorageConfigError>,
    > {
        let request_uri = format!(
            "/instance/{instance_id}/storage-config/{association_id}",
            association_id = input.association_id,
            instance_id = input.instance_id
        );

        let mut request = SignedRequest::new("GET", "connect", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut params = Params::new();
        params.put("resourceType", &input.resource_type);
        request.set_params(params);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<DescribeInstanceStorageConfigResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(DescribeInstanceStorageConfigError::from_response(response))
        }
    }

    /// <p>This API is in preview release for Amazon Connect and is subject to change.</p> <p>Describes the quick connect.</p>
    #[allow(unused_mut)]
    async fn describe_quick_connect(
        &self,
        input: DescribeQuickConnectRequest,
    ) -> Result<DescribeQuickConnectResponse, RusotoError<DescribeQuickConnectError>> {
        let request_uri = format!(
            "/quick-connects/{instance_id}/{quick_connect_id}",
            instance_id = input.instance_id,
            quick_connect_id = input.quick_connect_id
        );

        let mut request = SignedRequest::new("GET", "connect", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<DescribeQuickConnectResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(DescribeQuickConnectError::from_response(response))
        }
    }

    /// <p>Describes the specified routing profile.</p>
    #[allow(unused_mut)]
    async fn describe_routing_profile(
        &self,
        input: DescribeRoutingProfileRequest,
    ) -> Result<DescribeRoutingProfileResponse, RusotoError<DescribeRoutingProfileError>> {
        let request_uri = format!(
            "/routing-profiles/{instance_id}/{routing_profile_id}",
            instance_id = input.instance_id,
            routing_profile_id = input.routing_profile_id
        );

        let mut request = SignedRequest::new("GET", "connect", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<DescribeRoutingProfileResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(DescribeRoutingProfileError::from_response(response))
        }
    }

    /// <p>Describes the specified user account. You can find the instance ID in the console (it’s the final part of the ARN). The console does not display the user IDs. Instead, list the users and note the IDs provided in the output.</p>
    #[allow(unused_mut)]
    async fn describe_user(
        &self,
        input: DescribeUserRequest,
    ) -> Result<DescribeUserResponse, RusotoError<DescribeUserError>> {
        let request_uri = format!(
            "/users/{instance_id}/{user_id}",
            instance_id = input.instance_id,
            user_id = input.user_id
        );

        let mut request = SignedRequest::new("GET", "connect", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<DescribeUserResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(DescribeUserError::from_response(response))
        }
    }

    /// <p>Describes the specified hierarchy group.</p>
    #[allow(unused_mut)]
    async fn describe_user_hierarchy_group(
        &self,
        input: DescribeUserHierarchyGroupRequest,
    ) -> Result<DescribeUserHierarchyGroupResponse, RusotoError<DescribeUserHierarchyGroupError>>
    {
        let request_uri = format!(
            "/user-hierarchy-groups/{instance_id}/{hierarchy_group_id}",
            hierarchy_group_id = input.hierarchy_group_id,
            instance_id = input.instance_id
        );

        let mut request = SignedRequest::new("GET", "connect", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<DescribeUserHierarchyGroupResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(DescribeUserHierarchyGroupError::from_response(response))
        }
    }

    /// <p>Describes the hierarchy structure of the specified Amazon Connect instance.</p>
    #[allow(unused_mut)]
    async fn describe_user_hierarchy_structure(
        &self,
        input: DescribeUserHierarchyStructureRequest,
    ) -> Result<
        DescribeUserHierarchyStructureResponse,
        RusotoError<DescribeUserHierarchyStructureError>,
    > {
        let request_uri = format!(
            "/user-hierarchy-structure/{instance_id}",
            instance_id = input.instance_id
        );

        let mut request = SignedRequest::new("GET", "connect", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<DescribeUserHierarchyStructureResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(DescribeUserHierarchyStructureError::from_response(response))
        }
    }

    /// <p>This API is in preview release for Amazon Connect and is subject to change.</p> <p>Revokes access to integrated applications from Amazon Connect.</p>
    #[allow(unused_mut)]
    async fn disassociate_approved_origin(
        &self,
        input: DisassociateApprovedOriginRequest,
    ) -> Result<(), RusotoError<DisassociateApprovedOriginError>> {
        let request_uri = format!(
            "/instance/{instance_id}/approved-origin",
            instance_id = input.instance_id
        );

        let mut request = SignedRequest::new("DELETE", "connect", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut params = Params::new();
        params.put("origin", &input.origin);
        request.set_params(params);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            ::std::mem::drop(response);

            Ok(())
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(DisassociateApprovedOriginError::from_response(response))
        }
    }

    /// <p>This API is in preview release for Amazon Connect and is subject to change.</p> <p>Removes the storage type configurations for the specified resource type and association ID.</p>
    #[allow(unused_mut)]
    async fn disassociate_instance_storage_config(
        &self,
        input: DisassociateInstanceStorageConfigRequest,
    ) -> Result<(), RusotoError<DisassociateInstanceStorageConfigError>> {
        let request_uri = format!(
            "/instance/{instance_id}/storage-config/{association_id}",
            association_id = input.association_id,
            instance_id = input.instance_id
        );

        let mut request = SignedRequest::new("DELETE", "connect", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut params = Params::new();
        params.put("resourceType", &input.resource_type);
        request.set_params(params);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            ::std::mem::drop(response);

            Ok(())
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(DisassociateInstanceStorageConfigError::from_response(
                response,
            ))
        }
    }

    /// <p>This API is in preview release for Amazon Connect and is subject to change.</p> <p>Remove the Lambda function from the drop-down options available in the relevant contact flow blocks.</p>
    #[allow(unused_mut)]
    async fn disassociate_lambda_function(
        &self,
        input: DisassociateLambdaFunctionRequest,
    ) -> Result<(), RusotoError<DisassociateLambdaFunctionError>> {
        let request_uri = format!(
            "/instance/{instance_id}/lambda-function",
            instance_id = input.instance_id
        );

        let mut request = SignedRequest::new("DELETE", "connect", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut params = Params::new();
        params.put("functionArn", &input.function_arn);
        request.set_params(params);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            ::std::mem::drop(response);

            Ok(())
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(DisassociateLambdaFunctionError::from_response(response))
        }
    }

    /// <p>This API is in preview release for Amazon Connect and is subject to change.</p> <p>Revokes authorization from the specified instance to access the specified Amazon Lex bot.</p>
    #[allow(unused_mut)]
    async fn disassociate_lex_bot(
        &self,
        input: DisassociateLexBotRequest,
    ) -> Result<(), RusotoError<DisassociateLexBotError>> {
        let request_uri = format!(
            "/instance/{instance_id}/lex-bot",
            instance_id = input.instance_id
        );

        let mut request = SignedRequest::new("DELETE", "connect", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut params = Params::new();
        params.put("botName", &input.bot_name);
        params.put("lexRegion", &input.lex_region);
        request.set_params(params);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            ::std::mem::drop(response);

            Ok(())
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(DisassociateLexBotError::from_response(response))
        }
    }

    /// <p>Disassociates a set of queues from a routing profile.</p>
    #[allow(unused_mut)]
    async fn disassociate_routing_profile_queues(
        &self,
        input: DisassociateRoutingProfileQueuesRequest,
    ) -> Result<(), RusotoError<DisassociateRoutingProfileQueuesError>> {
        let request_uri = format!(
            "/routing-profiles/{instance_id}/{routing_profile_id}/disassociate-queues",
            instance_id = input.instance_id,
            routing_profile_id = input.routing_profile_id
        );

        let mut request = SignedRequest::new("POST", "connect", &self.region, &request_uri);
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
            ::std::mem::drop(response);

            Ok(())
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(DisassociateRoutingProfileQueuesError::from_response(
                response,
            ))
        }
    }

    /// <p>This API is in preview release for Amazon Connect and is subject to change.</p> <p>Deletes the specified security key.</p>
    #[allow(unused_mut)]
    async fn disassociate_security_key(
        &self,
        input: DisassociateSecurityKeyRequest,
    ) -> Result<(), RusotoError<DisassociateSecurityKeyError>> {
        let request_uri = format!(
            "/instance/{instance_id}/security-key/{association_id}",
            association_id = input.association_id,
            instance_id = input.instance_id
        );

        let mut request = SignedRequest::new("DELETE", "connect", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            ::std::mem::drop(response);

            Ok(())
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(DisassociateSecurityKeyError::from_response(response))
        }
    }

    /// <p>Retrieves the contact attributes for the specified contact.</p>
    #[allow(unused_mut)]
    async fn get_contact_attributes(
        &self,
        input: GetContactAttributesRequest,
    ) -> Result<GetContactAttributesResponse, RusotoError<GetContactAttributesError>> {
        let request_uri = format!(
            "/contact/attributes/{instance_id}/{initial_contact_id}",
            initial_contact_id = input.initial_contact_id,
            instance_id = input.instance_id
        );

        let mut request = SignedRequest::new("GET", "connect", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<GetContactAttributesResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(GetContactAttributesError::from_response(response))
        }
    }

    /// <p>Gets the real-time metric data from the specified Amazon Connect instance.</p> <p>For a description of each metric, see <a href="https://docs.aws.amazon.com/connect/latest/adminguide/real-time-metrics-definitions.html">Real-time Metrics Definitions</a> in the <i>Amazon Connect Administrator Guide</i>.</p>
    #[allow(unused_mut)]
    async fn get_current_metric_data(
        &self,
        input: GetCurrentMetricDataRequest,
    ) -> Result<GetCurrentMetricDataResponse, RusotoError<GetCurrentMetricDataError>> {
        let request_uri = format!(
            "/metrics/current/{instance_id}",
            instance_id = input.instance_id
        );

        let mut request = SignedRequest::new("POST", "connect", &self.region, &request_uri);
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
                .deserialize::<GetCurrentMetricDataResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(GetCurrentMetricDataError::from_response(response))
        }
    }

    /// <p>Retrieves a token for federation.</p>
    #[allow(unused_mut)]
    async fn get_federation_token(
        &self,
        input: GetFederationTokenRequest,
    ) -> Result<GetFederationTokenResponse, RusotoError<GetFederationTokenError>> {
        let request_uri = format!(
            "/user/federate/{instance_id}",
            instance_id = input.instance_id
        );

        let mut request = SignedRequest::new("GET", "connect", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<GetFederationTokenResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(GetFederationTokenError::from_response(response))
        }
    }

    /// <p>Gets historical metric data from the specified Amazon Connect instance.</p> <p>For a description of each historical metric, see <a href="https://docs.aws.amazon.com/connect/latest/adminguide/historical-metrics-definitions.html">Historical Metrics Definitions</a> in the <i>Amazon Connect Administrator Guide</i>.</p>
    #[allow(unused_mut)]
    async fn get_metric_data(
        &self,
        input: GetMetricDataRequest,
    ) -> Result<GetMetricDataResponse, RusotoError<GetMetricDataError>> {
        let request_uri = format!(
            "/metrics/historical/{instance_id}",
            instance_id = input.instance_id
        );

        let mut request = SignedRequest::new("POST", "connect", &self.region, &request_uri);
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
                .deserialize::<GetMetricDataResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(GetMetricDataError::from_response(response))
        }
    }

    /// <p>This API is in preview release for Amazon Connect and is subject to change.</p> <p>Returns a paginated list of all approved origins associated with the instance.</p>
    #[allow(unused_mut)]
    async fn list_approved_origins(
        &self,
        input: ListApprovedOriginsRequest,
    ) -> Result<ListApprovedOriginsResponse, RusotoError<ListApprovedOriginsError>> {
        let request_uri = format!(
            "/instance/{instance_id}/approved-origins",
            instance_id = input.instance_id
        );

        let mut request = SignedRequest::new("GET", "connect", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut params = Params::new();
        if let Some(ref x) = input.max_results {
            params.put("maxResults", x);
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
        if response.status.is_success() {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<ListApprovedOriginsResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(ListApprovedOriginsError::from_response(response))
        }
    }

    /// <p>Provides information about the contact flows for the specified Amazon Connect instance.</p> <p>You can also create and update contact flows using the <a href="https://docs.aws.amazon.com/connect/latest/adminguide/flow-language.html">Amazon Connect Flow language</a>.</p> <p>For more information about contact flows, see <a href="https://docs.aws.amazon.com/connect/latest/adminguide/concepts-contact-flows.html">Contact Flows</a> in the <i>Amazon Connect Administrator Guide</i>.</p>
    #[allow(unused_mut)]
    async fn list_contact_flows(
        &self,
        input: ListContactFlowsRequest,
    ) -> Result<ListContactFlowsResponse, RusotoError<ListContactFlowsError>> {
        let request_uri = format!(
            "/contact-flows-summary/{instance_id}",
            instance_id = input.instance_id
        );

        let mut request = SignedRequest::new("GET", "connect", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut params = Params::new();
        if let Some(ref x) = input.contact_flow_types {
            for item in x.iter() {
                params.put("contactFlowTypes", item);
            }
        }
        if let Some(ref x) = input.max_results {
            params.put("maxResults", x);
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
        if response.status.is_success() {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<ListContactFlowsResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(ListContactFlowsError::from_response(response))
        }
    }

    /// <p>Provides information about the hours of operation for the specified Amazon Connect instance.</p> <p>For more information about hours of operation, see <a href="https://docs.aws.amazon.com/connect/latest/adminguide/set-hours-operation.html">Set the Hours of Operation for a Queue</a> in the <i>Amazon Connect Administrator Guide</i>.</p>
    #[allow(unused_mut)]
    async fn list_hours_of_operations(
        &self,
        input: ListHoursOfOperationsRequest,
    ) -> Result<ListHoursOfOperationsResponse, RusotoError<ListHoursOfOperationsError>> {
        let request_uri = format!(
            "/hours-of-operations-summary/{instance_id}",
            instance_id = input.instance_id
        );

        let mut request = SignedRequest::new("GET", "connect", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut params = Params::new();
        if let Some(ref x) = input.max_results {
            params.put("maxResults", x);
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
        if response.status.is_success() {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<ListHoursOfOperationsResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(ListHoursOfOperationsError::from_response(response))
        }
    }

    /// <p>This API is in preview release for Amazon Connect and is subject to change.</p> <p>Returns a paginated list of all attribute types for the given instance.</p>
    #[allow(unused_mut)]
    async fn list_instance_attributes(
        &self,
        input: ListInstanceAttributesRequest,
    ) -> Result<ListInstanceAttributesResponse, RusotoError<ListInstanceAttributesError>> {
        let request_uri = format!(
            "/instance/{instance_id}/attributes",
            instance_id = input.instance_id
        );

        let mut request = SignedRequest::new("GET", "connect", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut params = Params::new();
        if let Some(ref x) = input.max_results {
            params.put("maxResults", x);
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
        if response.status.is_success() {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<ListInstanceAttributesResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(ListInstanceAttributesError::from_response(response))
        }
    }

    /// <p>This API is in preview release for Amazon Connect and is subject to change.</p> <p>Returns a paginated list of storage configs for the identified instance and resource type.</p>
    #[allow(unused_mut)]
    async fn list_instance_storage_configs(
        &self,
        input: ListInstanceStorageConfigsRequest,
    ) -> Result<ListInstanceStorageConfigsResponse, RusotoError<ListInstanceStorageConfigsError>>
    {
        let request_uri = format!(
            "/instance/{instance_id}/storage-configs",
            instance_id = input.instance_id
        );

        let mut request = SignedRequest::new("GET", "connect", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut params = Params::new();
        if let Some(ref x) = input.max_results {
            params.put("maxResults", x);
        }
        if let Some(ref x) = input.next_token {
            params.put("nextToken", x);
        }
        params.put("resourceType", &input.resource_type);
        request.set_params(params);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<ListInstanceStorageConfigsResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(ListInstanceStorageConfigsError::from_response(response))
        }
    }

    /// <p>This API is in preview release for Amazon Connect and is subject to change.</p> <p>Return a list of instances which are in active state, creation-in-progress state, and failed state. Instances that aren't successfully created (they are in a failed state) are returned only for 24 hours after the CreateInstance API was invoked.</p>
    #[allow(unused_mut)]
    async fn list_instances(
        &self,
        input: ListInstancesRequest,
    ) -> Result<ListInstancesResponse, RusotoError<ListInstancesError>> {
        let request_uri = "/instance";

        let mut request = SignedRequest::new("GET", "connect", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut params = Params::new();
        if let Some(ref x) = input.max_results {
            params.put("maxResults", x);
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
        if response.status.is_success() {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<ListInstancesResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(ListInstancesError::from_response(response))
        }
    }

    /// <p>This API is in preview release for Amazon Connect and is subject to change.</p> <p>Provides summary information about the AppIntegration associations for the specified Amazon Connect instance.</p>
    #[allow(unused_mut)]
    async fn list_integration_associations(
        &self,
        input: ListIntegrationAssociationsRequest,
    ) -> Result<ListIntegrationAssociationsResponse, RusotoError<ListIntegrationAssociationsError>>
    {
        let request_uri = format!(
            "/instance/{instance_id}/integration-associations",
            instance_id = input.instance_id
        );

        let mut request = SignedRequest::new("GET", "connect", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut params = Params::new();
        if let Some(ref x) = input.max_results {
            params.put("maxResults", x);
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
        if response.status.is_success() {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<ListIntegrationAssociationsResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(ListIntegrationAssociationsError::from_response(response))
        }
    }

    /// <p>This API is in preview release for Amazon Connect and is subject to change.</p> <p>Returns a paginated list of all the Lambda functions that show up in the drop-down options in the relevant contact flow blocks.</p>
    #[allow(unused_mut)]
    async fn list_lambda_functions(
        &self,
        input: ListLambdaFunctionsRequest,
    ) -> Result<ListLambdaFunctionsResponse, RusotoError<ListLambdaFunctionsError>> {
        let request_uri = format!(
            "/instance/{instance_id}/lambda-functions",
            instance_id = input.instance_id
        );

        let mut request = SignedRequest::new("GET", "connect", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut params = Params::new();
        if let Some(ref x) = input.max_results {
            params.put("maxResults", x);
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
        if response.status.is_success() {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<ListLambdaFunctionsResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(ListLambdaFunctionsError::from_response(response))
        }
    }

    /// <p>This API is in preview release for Amazon Connect and is subject to change.</p> <p>Returns a paginated list of all the Amazon Lex bots currently associated with the instance.</p>
    #[allow(unused_mut)]
    async fn list_lex_bots(
        &self,
        input: ListLexBotsRequest,
    ) -> Result<ListLexBotsResponse, RusotoError<ListLexBotsError>> {
        let request_uri = format!(
            "/instance/{instance_id}/lex-bots",
            instance_id = input.instance_id
        );

        let mut request = SignedRequest::new("GET", "connect", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut params = Params::new();
        if let Some(ref x) = input.max_results {
            params.put("maxResults", x);
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
        if response.status.is_success() {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<ListLexBotsResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(ListLexBotsError::from_response(response))
        }
    }

    /// <p>Provides information about the phone numbers for the specified Amazon Connect instance. </p> <p>For more information about phone numbers, see <a href="https://docs.aws.amazon.com/connect/latest/adminguide/contact-center-phone-number.html">Set Up Phone Numbers for Your Contact Center</a> in the <i>Amazon Connect Administrator Guide</i>.</p>
    #[allow(unused_mut)]
    async fn list_phone_numbers(
        &self,
        input: ListPhoneNumbersRequest,
    ) -> Result<ListPhoneNumbersResponse, RusotoError<ListPhoneNumbersError>> {
        let request_uri = format!(
            "/phone-numbers-summary/{instance_id}",
            instance_id = input.instance_id
        );

        let mut request = SignedRequest::new("GET", "connect", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut params = Params::new();
        if let Some(ref x) = input.max_results {
            params.put("maxResults", x);
        }
        if let Some(ref x) = input.next_token {
            params.put("nextToken", x);
        }
        if let Some(ref x) = input.phone_number_country_codes {
            for item in x.iter() {
                params.put("phoneNumberCountryCodes", item);
            }
        }
        if let Some(ref x) = input.phone_number_types {
            for item in x.iter() {
                params.put("phoneNumberTypes", item);
            }
        }
        request.set_params(params);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<ListPhoneNumbersResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(ListPhoneNumbersError::from_response(response))
        }
    }

    /// <p>Provides information about the prompts for the specified Amazon Connect instance.</p>
    #[allow(unused_mut)]
    async fn list_prompts(
        &self,
        input: ListPromptsRequest,
    ) -> Result<ListPromptsResponse, RusotoError<ListPromptsError>> {
        let request_uri = format!(
            "/prompts-summary/{instance_id}",
            instance_id = input.instance_id
        );

        let mut request = SignedRequest::new("GET", "connect", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut params = Params::new();
        if let Some(ref x) = input.max_results {
            params.put("maxResults", x);
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
        if response.status.is_success() {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<ListPromptsResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(ListPromptsError::from_response(response))
        }
    }

    /// <p>Provides information about the queues for the specified Amazon Connect instance.</p> <p>For more information about queues, see <a href="https://docs.aws.amazon.com/connect/latest/adminguide/concepts-queues-standard-and-agent.html">Queues: Standard and Agent</a> in the <i>Amazon Connect Administrator Guide</i>.</p>
    #[allow(unused_mut)]
    async fn list_queues(
        &self,
        input: ListQueuesRequest,
    ) -> Result<ListQueuesResponse, RusotoError<ListQueuesError>> {
        let request_uri = format!(
            "/queues-summary/{instance_id}",
            instance_id = input.instance_id
        );

        let mut request = SignedRequest::new("GET", "connect", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut params = Params::new();
        if let Some(ref x) = input.max_results {
            params.put("maxResults", x);
        }
        if let Some(ref x) = input.next_token {
            params.put("nextToken", x);
        }
        if let Some(ref x) = input.queue_types {
            for item in x.iter() {
                params.put("queueTypes", item);
            }
        }
        request.set_params(params);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<ListQueuesResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(ListQueuesError::from_response(response))
        }
    }

    /// <p>This API is in preview release for Amazon Connect and is subject to change.</p> <p>Provides information about the quick connects for the specified Amazon Connect instance. </p>
    #[allow(unused_mut)]
    async fn list_quick_connects(
        &self,
        input: ListQuickConnectsRequest,
    ) -> Result<ListQuickConnectsResponse, RusotoError<ListQuickConnectsError>> {
        let request_uri = format!(
            "/quick-connects/{instance_id}",
            instance_id = input.instance_id
        );

        let mut request = SignedRequest::new("GET", "connect", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut params = Params::new();
        if let Some(ref x) = input.max_results {
            params.put("maxResults", x);
        }
        if let Some(ref x) = input.next_token {
            params.put("nextToken", x);
        }
        if let Some(ref x) = input.quick_connect_types {
            for item in x.iter() {
                params.put("QuickConnectTypes", item);
            }
        }
        request.set_params(params);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<ListQuickConnectsResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(ListQuickConnectsError::from_response(response))
        }
    }

    /// <p>List the queues associated with a routing profile.</p>
    #[allow(unused_mut)]
    async fn list_routing_profile_queues(
        &self,
        input: ListRoutingProfileQueuesRequest,
    ) -> Result<ListRoutingProfileQueuesResponse, RusotoError<ListRoutingProfileQueuesError>> {
        let request_uri = format!(
            "/routing-profiles/{instance_id}/{routing_profile_id}/queues",
            instance_id = input.instance_id,
            routing_profile_id = input.routing_profile_id
        );

        let mut request = SignedRequest::new("GET", "connect", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut params = Params::new();
        if let Some(ref x) = input.max_results {
            params.put("maxResults", x);
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
        if response.status.is_success() {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<ListRoutingProfileQueuesResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(ListRoutingProfileQueuesError::from_response(response))
        }
    }

    /// <p>Provides summary information about the routing profiles for the specified Amazon Connect instance.</p> <p>For more information about routing profiles, see <a href="https://docs.aws.amazon.com/connect/latest/adminguide/concepts-routing.html">Routing Profiles</a> and <a href="https://docs.aws.amazon.com/connect/latest/adminguide/routing-profiles.html">Create a Routing Profile</a> in the <i>Amazon Connect Administrator Guide</i>.</p>
    #[allow(unused_mut)]
    async fn list_routing_profiles(
        &self,
        input: ListRoutingProfilesRequest,
    ) -> Result<ListRoutingProfilesResponse, RusotoError<ListRoutingProfilesError>> {
        let request_uri = format!(
            "/routing-profiles-summary/{instance_id}",
            instance_id = input.instance_id
        );

        let mut request = SignedRequest::new("GET", "connect", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut params = Params::new();
        if let Some(ref x) = input.max_results {
            params.put("maxResults", x);
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
        if response.status.is_success() {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<ListRoutingProfilesResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(ListRoutingProfilesError::from_response(response))
        }
    }

    /// <p>This API is in preview release for Amazon Connect and is subject to change.</p> <p>Returns a paginated list of all security keys associated with the instance.</p>
    #[allow(unused_mut)]
    async fn list_security_keys(
        &self,
        input: ListSecurityKeysRequest,
    ) -> Result<ListSecurityKeysResponse, RusotoError<ListSecurityKeysError>> {
        let request_uri = format!(
            "/instance/{instance_id}/security-keys",
            instance_id = input.instance_id
        );

        let mut request = SignedRequest::new("GET", "connect", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut params = Params::new();
        if let Some(ref x) = input.max_results {
            params.put("maxResults", x);
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
        if response.status.is_success() {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<ListSecurityKeysResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(ListSecurityKeysError::from_response(response))
        }
    }

    /// <p>Provides summary information about the security profiles for the specified Amazon Connect instance.</p> <p>For more information about security profiles, see <a href="https://docs.aws.amazon.com/connect/latest/adminguide/connect-security-profiles.html">Security Profiles</a> in the <i>Amazon Connect Administrator Guide</i>.</p>
    #[allow(unused_mut)]
    async fn list_security_profiles(
        &self,
        input: ListSecurityProfilesRequest,
    ) -> Result<ListSecurityProfilesResponse, RusotoError<ListSecurityProfilesError>> {
        let request_uri = format!(
            "/security-profiles-summary/{instance_id}",
            instance_id = input.instance_id
        );

        let mut request = SignedRequest::new("GET", "connect", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut params = Params::new();
        if let Some(ref x) = input.max_results {
            params.put("maxResults", x);
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
        if response.status.is_success() {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<ListSecurityProfilesResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(ListSecurityProfilesError::from_response(response))
        }
    }

    /// <p>Lists the tags for the specified resource.</p> <p>For sample policies that use tags, see <a href="https://docs.aws.amazon.com/connect/latest/adminguide/security_iam_id-based-policy-examples.html">Amazon Connect Identity-Based Policy Examples</a> in the <i>Amazon Connect Administrator Guide</i>.</p>
    #[allow(unused_mut)]
    async fn list_tags_for_resource(
        &self,
        input: ListTagsForResourceRequest,
    ) -> Result<ListTagsForResourceResponse, RusotoError<ListTagsForResourceError>> {
        let request_uri = format!("/tags/{resource_arn}", resource_arn = input.resource_arn);

        let mut request = SignedRequest::new("GET", "connect", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<ListTagsForResourceResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(ListTagsForResourceError::from_response(response))
        }
    }

    /// <p>This API is in preview release for Amazon Connect and is subject to change.</p> <p>List the use cases. </p>
    #[allow(unused_mut)]
    async fn list_use_cases(
        &self,
        input: ListUseCasesRequest,
    ) -> Result<ListUseCasesResponse, RusotoError<ListUseCasesError>> {
        let request_uri = format!("/instance/{instance_id}/integration-associations/{integration_association_id}/use-cases", instance_id = input.instance_id, integration_association_id = input.integration_association_id);

        let mut request = SignedRequest::new("GET", "connect", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut params = Params::new();
        if let Some(ref x) = input.max_results {
            params.put("maxResults", x);
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
        if response.status.is_success() {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<ListUseCasesResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(ListUseCasesError::from_response(response))
        }
    }

    /// <p>Provides summary information about the hierarchy groups for the specified Amazon Connect instance.</p> <p>For more information about agent hierarchies, see <a href="https://docs.aws.amazon.com/connect/latest/adminguide/agent-hierarchy.html">Set Up Agent Hierarchies</a> in the <i>Amazon Connect Administrator Guide</i>.</p>
    #[allow(unused_mut)]
    async fn list_user_hierarchy_groups(
        &self,
        input: ListUserHierarchyGroupsRequest,
    ) -> Result<ListUserHierarchyGroupsResponse, RusotoError<ListUserHierarchyGroupsError>> {
        let request_uri = format!(
            "/user-hierarchy-groups-summary/{instance_id}",
            instance_id = input.instance_id
        );

        let mut request = SignedRequest::new("GET", "connect", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut params = Params::new();
        if let Some(ref x) = input.max_results {
            params.put("maxResults", x);
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
        if response.status.is_success() {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<ListUserHierarchyGroupsResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(ListUserHierarchyGroupsError::from_response(response))
        }
    }

    /// <p>Provides summary information about the users for the specified Amazon Connect instance.</p>
    #[allow(unused_mut)]
    async fn list_users(
        &self,
        input: ListUsersRequest,
    ) -> Result<ListUsersResponse, RusotoError<ListUsersError>> {
        let request_uri = format!(
            "/users-summary/{instance_id}",
            instance_id = input.instance_id
        );

        let mut request = SignedRequest::new("GET", "connect", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut params = Params::new();
        if let Some(ref x) = input.max_results {
            params.put("maxResults", x);
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
        if response.status.is_success() {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<ListUsersResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(ListUsersError::from_response(response))
        }
    }

    /// <p>When a contact is being recorded, and the recording has been suspended using SuspendContactRecording, this API resumes recording the call.</p> <p>Only voice recordings are supported at this time.</p>
    #[allow(unused_mut)]
    async fn resume_contact_recording(
        &self,
        input: ResumeContactRecordingRequest,
    ) -> Result<ResumeContactRecordingResponse, RusotoError<ResumeContactRecordingError>> {
        let request_uri = "/contact/resume-recording";

        let mut request = SignedRequest::new("POST", "connect", &self.region, &request_uri);
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
                .deserialize::<ResumeContactRecordingResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(ResumeContactRecordingError::from_response(response))
        }
    }

    /// <p>Initiates a contact flow to start a new chat for the customer. Response of this API provides a token required to obtain credentials from the <a href="https://docs.aws.amazon.com/connect-participant/latest/APIReference/API_CreateParticipantConnection.html">CreateParticipantConnection</a> API in the Amazon Connect Participant Service.</p> <p>When a new chat contact is successfully created, clients need to subscribe to the participant’s connection for the created chat within 5 minutes. This is achieved by invoking <a href="https://docs.aws.amazon.com/connect-participant/latest/APIReference/API_CreateParticipantConnection.html">CreateParticipantConnection</a> with WEBSOCKET and CONNECTION_CREDENTIALS. </p> <p>A 429 error occurs in two situations:</p> <ul> <li> <p>API rate limit is exceeded. API TPS throttling returns a <code>TooManyRequests</code> exception from the API Gateway.</p> </li> <li> <p>The <a href="https://docs.aws.amazon.com/connect/latest/adminguide/amazon-connect-service-limits.html">quota for concurrent active chats</a> is exceeded. Active chat throttling returns a <code>LimitExceededException</code>.</p> </li> </ul> <p>For more information about how chat works, see <a href="https://docs.aws.amazon.com/connect/latest/adminguide/chat.html">Chat</a> in the <i>Amazon Connect Administrator Guide</i>.</p>
    #[allow(unused_mut)]
    async fn start_chat_contact(
        &self,
        input: StartChatContactRequest,
    ) -> Result<StartChatContactResponse, RusotoError<StartChatContactError>> {
        let request_uri = "/contact/chat";

        let mut request = SignedRequest::new("PUT", "connect", &self.region, &request_uri);
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
                .deserialize::<StartChatContactResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(StartChatContactError::from_response(response))
        }
    }

    /// <p>This API starts recording the contact when the agent joins the call. StartContactRecording is a one-time action. For example, if you use StopContactRecording to stop recording an ongoing call, you can't use StartContactRecording to restart it. For scenarios where the recording has started and you want to suspend and resume it, such as when collecting sensitive information (for example, a credit card number), use SuspendContactRecording and ResumeContactRecording.</p> <p>You can use this API to override the recording behavior configured in the <a href="https://docs.aws.amazon.com/connect/latest/adminguide/set-recording-behavior.html">Set recording behavior</a> block.</p> <p>Only voice recordings are supported at this time.</p>
    #[allow(unused_mut)]
    async fn start_contact_recording(
        &self,
        input: StartContactRecordingRequest,
    ) -> Result<StartContactRecordingResponse, RusotoError<StartContactRecordingError>> {
        let request_uri = "/contact/start-recording";

        let mut request = SignedRequest::new("POST", "connect", &self.region, &request_uri);
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
                .deserialize::<StartContactRecordingResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(StartContactRecordingError::from_response(response))
        }
    }

    /// <p><p>This API places an outbound call to a contact, and then initiates the contact flow. It performs the actions in the contact flow that&#39;s specified (in <code>ContactFlowId</code>).</p> <p>Agents are not involved in initiating the outbound API (that is, dialing the contact). If the contact flow places an outbound call to a contact, and then puts the contact in queue, that&#39;s when the call is routed to the agent, like any other inbound case.</p> <p>There is a 60 second dialing timeout for this operation. If the call is not connected after 60 seconds, it fails.</p> <note> <p>UK numbers with a 447 prefix are not allowed by default. Before you can dial these UK mobile numbers, you must submit a service quota increase request. For more information, see <a href="https://docs.aws.amazon.com/connect/latest/adminguide/amazon-connect-service-limits.html">Amazon Connect Service Quotas</a> in the <i>Amazon Connect Administrator Guide</i>. </p> </note></p>
    #[allow(unused_mut)]
    async fn start_outbound_voice_contact(
        &self,
        input: StartOutboundVoiceContactRequest,
    ) -> Result<StartOutboundVoiceContactResponse, RusotoError<StartOutboundVoiceContactError>>
    {
        let request_uri = "/contact/outbound-voice";

        let mut request = SignedRequest::new("PUT", "connect", &self.region, &request_uri);
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
                .deserialize::<StartOutboundVoiceContactResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(StartOutboundVoiceContactError::from_response(response))
        }
    }

    /// <p>Initiates a contact flow to start a new task.</p>
    #[allow(unused_mut)]
    async fn start_task_contact(
        &self,
        input: StartTaskContactRequest,
    ) -> Result<StartTaskContactResponse, RusotoError<StartTaskContactError>> {
        let request_uri = "/contact/task";

        let mut request = SignedRequest::new("PUT", "connect", &self.region, &request_uri);
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
                .deserialize::<StartTaskContactResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(StartTaskContactError::from_response(response))
        }
    }

    /// <p>Ends the specified contact.</p>
    #[allow(unused_mut)]
    async fn stop_contact(
        &self,
        input: StopContactRequest,
    ) -> Result<StopContactResponse, RusotoError<StopContactError>> {
        let request_uri = "/contact/stop";

        let mut request = SignedRequest::new("POST", "connect", &self.region, &request_uri);
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
                .deserialize::<StopContactResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(StopContactError::from_response(response))
        }
    }

    /// <p>When a contact is being recorded, this API stops recording the call. StopContactRecording is a one-time action. If you use StopContactRecording to stop recording an ongoing call, you can't use StartContactRecording to restart it. For scenarios where the recording has started and you want to suspend it for sensitive information (for example, to collect a credit card number), and then restart it, use SuspendContactRecording and ResumeContactRecording.</p> <p>Only voice recordings are supported at this time.</p>
    #[allow(unused_mut)]
    async fn stop_contact_recording(
        &self,
        input: StopContactRecordingRequest,
    ) -> Result<StopContactRecordingResponse, RusotoError<StopContactRecordingError>> {
        let request_uri = "/contact/stop-recording";

        let mut request = SignedRequest::new("POST", "connect", &self.region, &request_uri);
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
                .deserialize::<StopContactRecordingResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(StopContactRecordingError::from_response(response))
        }
    }

    /// <p>When a contact is being recorded, this API suspends recording the call. For example, you might suspend the call recording while collecting sensitive information, such as a credit card number. Then use ResumeContactRecording to restart recording. </p> <p>The period of time that the recording is suspended is filled with silence in the final recording. </p> <p>Only voice recordings are supported at this time.</p>
    #[allow(unused_mut)]
    async fn suspend_contact_recording(
        &self,
        input: SuspendContactRecordingRequest,
    ) -> Result<SuspendContactRecordingResponse, RusotoError<SuspendContactRecordingError>> {
        let request_uri = "/contact/suspend-recording";

        let mut request = SignedRequest::new("POST", "connect", &self.region, &request_uri);
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
                .deserialize::<SuspendContactRecordingResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(SuspendContactRecordingError::from_response(response))
        }
    }

    /// <p>Adds the specified tags to the specified resource.</p> <p>The supported resource types are users, routing profiles, quick connects, and contact flows.</p> <p>For sample policies that use tags, see <a href="https://docs.aws.amazon.com/connect/latest/adminguide/security_iam_id-based-policy-examples.html">Amazon Connect Identity-Based Policy Examples</a> in the <i>Amazon Connect Administrator Guide</i>.</p>
    #[allow(unused_mut)]
    async fn tag_resource(
        &self,
        input: TagResourceRequest,
    ) -> Result<(), RusotoError<TagResourceError>> {
        let request_uri = format!("/tags/{resource_arn}", resource_arn = input.resource_arn);

        let mut request = SignedRequest::new("POST", "connect", &self.region, &request_uri);
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
            ::std::mem::drop(response);

            Ok(())
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(TagResourceError::from_response(response))
        }
    }

    /// <p>Removes the specified tags from the specified resource.</p>
    #[allow(unused_mut)]
    async fn untag_resource(
        &self,
        input: UntagResourceRequest,
    ) -> Result<(), RusotoError<UntagResourceError>> {
        let request_uri = format!("/tags/{resource_arn}", resource_arn = input.resource_arn);

        let mut request = SignedRequest::new("DELETE", "connect", &self.region, &request_uri);
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
        if response.status.is_success() {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            ::std::mem::drop(response);

            Ok(())
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(UntagResourceError::from_response(response))
        }
    }

    /// <p>Creates or updates the contact attributes associated with the specified contact.</p> <p>You can add or update attributes for both ongoing and completed contacts. For example, you can update the customer's name or the reason the customer called while the call is active, or add notes about steps that the agent took during the call that are displayed to the next agent that takes the call. You can also update attributes for a contact using data from your CRM application and save the data with the contact in Amazon Connect. You could also flag calls for additional analysis, such as legal review or identifying abusive callers.</p> <p>Contact attributes are available in Amazon Connect for 24 months, and are then deleted.</p> <p> <b>Important:</b> You cannot use the operation to update attributes for contacts that occurred prior to the release of the API, September 12, 2018. You can update attributes only for contacts that started after the release of the API. If you attempt to update attributes for a contact that occurred prior to the release of the API, a 400 error is returned. This applies also to queued callbacks that were initiated prior to the release of the API but are still active in your instance.</p>
    #[allow(unused_mut)]
    async fn update_contact_attributes(
        &self,
        input: UpdateContactAttributesRequest,
    ) -> Result<UpdateContactAttributesResponse, RusotoError<UpdateContactAttributesError>> {
        let request_uri = "/contact/attributes";

        let mut request = SignedRequest::new("POST", "connect", &self.region, &request_uri);
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
                .deserialize::<UpdateContactAttributesResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(UpdateContactAttributesError::from_response(response))
        }
    }

    /// <p>Updates the specified contact flow.</p> <p>You can also create and update contact flows using the <a href="https://docs.aws.amazon.com/connect/latest/adminguide/flow-language.html">Amazon Connect Flow language</a>.</p>
    #[allow(unused_mut)]
    async fn update_contact_flow_content(
        &self,
        input: UpdateContactFlowContentRequest,
    ) -> Result<(), RusotoError<UpdateContactFlowContentError>> {
        let request_uri = format!(
            "/contact-flows/{instance_id}/{contact_flow_id}/content",
            contact_flow_id = input.contact_flow_id,
            instance_id = input.instance_id
        );

        let mut request = SignedRequest::new("POST", "connect", &self.region, &request_uri);
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
            ::std::mem::drop(response);

            Ok(())
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(UpdateContactFlowContentError::from_response(response))
        }
    }

    /// <p>The name of the contact flow.</p> <p>You can also create and update contact flows using the <a href="https://docs.aws.amazon.com/connect/latest/adminguide/flow-language.html">Amazon Connect Flow language</a>.</p>
    #[allow(unused_mut)]
    async fn update_contact_flow_name(
        &self,
        input: UpdateContactFlowNameRequest,
    ) -> Result<(), RusotoError<UpdateContactFlowNameError>> {
        let request_uri = format!(
            "/contact-flows/{instance_id}/{contact_flow_id}/name",
            contact_flow_id = input.contact_flow_id,
            instance_id = input.instance_id
        );

        let mut request = SignedRequest::new("POST", "connect", &self.region, &request_uri);
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
            ::std::mem::drop(response);

            Ok(())
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(UpdateContactFlowNameError::from_response(response))
        }
    }

    /// <p>This API is in preview release for Amazon Connect and is subject to change.</p> <p>Updates the value for the specified attribute type.</p>
    #[allow(unused_mut)]
    async fn update_instance_attribute(
        &self,
        input: UpdateInstanceAttributeRequest,
    ) -> Result<(), RusotoError<UpdateInstanceAttributeError>> {
        let request_uri = format!(
            "/instance/{instance_id}/attribute/{attribute_type}",
            attribute_type = input.attribute_type,
            instance_id = input.instance_id
        );

        let mut request = SignedRequest::new("POST", "connect", &self.region, &request_uri);
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
            ::std::mem::drop(response);

            Ok(())
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(UpdateInstanceAttributeError::from_response(response))
        }
    }

    /// <p>This API is in preview release for Amazon Connect and is subject to change.</p> <p>Updates an existing configuration for a resource type. This API is idempotent.</p>
    #[allow(unused_mut)]
    async fn update_instance_storage_config(
        &self,
        input: UpdateInstanceStorageConfigRequest,
    ) -> Result<(), RusotoError<UpdateInstanceStorageConfigError>> {
        let request_uri = format!(
            "/instance/{instance_id}/storage-config/{association_id}",
            association_id = input.association_id,
            instance_id = input.instance_id
        );

        let mut request = SignedRequest::new("POST", "connect", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut params = Params::new();
        params.put("resourceType", &input.resource_type);
        request.set_params(params);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            ::std::mem::drop(response);

            Ok(())
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(UpdateInstanceStorageConfigError::from_response(response))
        }
    }

    /// <p>This API is in preview release for Amazon Connect and is subject to change.</p> <p>Updates the configuration settings for the specified quick connect.</p>
    #[allow(unused_mut)]
    async fn update_quick_connect_config(
        &self,
        input: UpdateQuickConnectConfigRequest,
    ) -> Result<(), RusotoError<UpdateQuickConnectConfigError>> {
        let request_uri = format!(
            "/quick-connects/{instance_id}/{quick_connect_id}/config",
            instance_id = input.instance_id,
            quick_connect_id = input.quick_connect_id
        );

        let mut request = SignedRequest::new("POST", "connect", &self.region, &request_uri);
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
            ::std::mem::drop(response);

            Ok(())
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(UpdateQuickConnectConfigError::from_response(response))
        }
    }

    /// <p>This API is in preview release for Amazon Connect and is subject to change.</p> <p>Updates the name and description of a quick connect. The request accepts the following data in JSON format. At least Name or Description must be provided.</p>
    #[allow(unused_mut)]
    async fn update_quick_connect_name(
        &self,
        input: UpdateQuickConnectNameRequest,
    ) -> Result<(), RusotoError<UpdateQuickConnectNameError>> {
        let request_uri = format!(
            "/quick-connects/{instance_id}/{quick_connect_id}/name",
            instance_id = input.instance_id,
            quick_connect_id = input.quick_connect_id
        );

        let mut request = SignedRequest::new("POST", "connect", &self.region, &request_uri);
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
            ::std::mem::drop(response);

            Ok(())
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(UpdateQuickConnectNameError::from_response(response))
        }
    }

    /// <p>Updates the channels that agents can handle in the Contact Control Panel (CCP) for a routing profile.</p>
    #[allow(unused_mut)]
    async fn update_routing_profile_concurrency(
        &self,
        input: UpdateRoutingProfileConcurrencyRequest,
    ) -> Result<(), RusotoError<UpdateRoutingProfileConcurrencyError>> {
        let request_uri = format!(
            "/routing-profiles/{instance_id}/{routing_profile_id}/concurrency",
            instance_id = input.instance_id,
            routing_profile_id = input.routing_profile_id
        );

        let mut request = SignedRequest::new("POST", "connect", &self.region, &request_uri);
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
            ::std::mem::drop(response);

            Ok(())
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(UpdateRoutingProfileConcurrencyError::from_response(
                response,
            ))
        }
    }

    /// <p>Updates the default outbound queue of a routing profile.</p>
    #[allow(unused_mut)]
    async fn update_routing_profile_default_outbound_queue(
        &self,
        input: UpdateRoutingProfileDefaultOutboundQueueRequest,
    ) -> Result<(), RusotoError<UpdateRoutingProfileDefaultOutboundQueueError>> {
        let request_uri = format!(
            "/routing-profiles/{instance_id}/{routing_profile_id}/default-outbound-queue",
            instance_id = input.instance_id,
            routing_profile_id = input.routing_profile_id
        );

        let mut request = SignedRequest::new("POST", "connect", &self.region, &request_uri);
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
            ::std::mem::drop(response);

            Ok(())
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(UpdateRoutingProfileDefaultOutboundQueueError::from_response(response))
        }
    }

    /// <p>Updates the name and description of a routing profile. The request accepts the following data in JSON format. At least <code>Name</code> or <code>Description</code> must be provided.</p>
    #[allow(unused_mut)]
    async fn update_routing_profile_name(
        &self,
        input: UpdateRoutingProfileNameRequest,
    ) -> Result<(), RusotoError<UpdateRoutingProfileNameError>> {
        let request_uri = format!(
            "/routing-profiles/{instance_id}/{routing_profile_id}/name",
            instance_id = input.instance_id,
            routing_profile_id = input.routing_profile_id
        );

        let mut request = SignedRequest::new("POST", "connect", &self.region, &request_uri);
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
            ::std::mem::drop(response);

            Ok(())
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(UpdateRoutingProfileNameError::from_response(response))
        }
    }

    /// <p>Updates the properties associated with a set of queues for a routing profile.</p>
    #[allow(unused_mut)]
    async fn update_routing_profile_queues(
        &self,
        input: UpdateRoutingProfileQueuesRequest,
    ) -> Result<(), RusotoError<UpdateRoutingProfileQueuesError>> {
        let request_uri = format!(
            "/routing-profiles/{instance_id}/{routing_profile_id}/queues",
            instance_id = input.instance_id,
            routing_profile_id = input.routing_profile_id
        );

        let mut request = SignedRequest::new("POST", "connect", &self.region, &request_uri);
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
            ::std::mem::drop(response);

            Ok(())
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(UpdateRoutingProfileQueuesError::from_response(response))
        }
    }

    /// <p>Assigns the specified hierarchy group to the specified user.</p>
    #[allow(unused_mut)]
    async fn update_user_hierarchy(
        &self,
        input: UpdateUserHierarchyRequest,
    ) -> Result<(), RusotoError<UpdateUserHierarchyError>> {
        let request_uri = format!(
            "/users/{instance_id}/{user_id}/hierarchy",
            instance_id = input.instance_id,
            user_id = input.user_id
        );

        let mut request = SignedRequest::new("POST", "connect", &self.region, &request_uri);
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
            ::std::mem::drop(response);

            Ok(())
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(UpdateUserHierarchyError::from_response(response))
        }
    }

    /// <p>Updates the name of the user hierarchy group. </p>
    #[allow(unused_mut)]
    async fn update_user_hierarchy_group_name(
        &self,
        input: UpdateUserHierarchyGroupNameRequest,
    ) -> Result<(), RusotoError<UpdateUserHierarchyGroupNameError>> {
        let request_uri = format!(
            "/user-hierarchy-groups/{instance_id}/{hierarchy_group_id}/name",
            hierarchy_group_id = input.hierarchy_group_id,
            instance_id = input.instance_id
        );

        let mut request = SignedRequest::new("POST", "connect", &self.region, &request_uri);
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
            ::std::mem::drop(response);

            Ok(())
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(UpdateUserHierarchyGroupNameError::from_response(response))
        }
    }

    /// <p>Updates the user hierarchy structure: add, remove, and rename user hierarchy levels.</p>
    #[allow(unused_mut)]
    async fn update_user_hierarchy_structure(
        &self,
        input: UpdateUserHierarchyStructureRequest,
    ) -> Result<(), RusotoError<UpdateUserHierarchyStructureError>> {
        let request_uri = format!(
            "/user-hierarchy-structure/{instance_id}",
            instance_id = input.instance_id
        );

        let mut request = SignedRequest::new("POST", "connect", &self.region, &request_uri);
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
            ::std::mem::drop(response);

            Ok(())
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(UpdateUserHierarchyStructureError::from_response(response))
        }
    }

    /// <p><p>Updates the identity information for the specified user.</p> <important> <p>Someone with the ability to invoke <code>UpdateUserIndentityInfo</code> can change the login credentials of other users by changing their email address. This poses a security risk to your organization. They can change the email address of a user to the attacker&#39;s email address, and then reset the password through email. We strongly recommend limiting who has the ability to invoke <code>UpdateUserIndentityInfo</code>. For more information, see <a href="https://docs.aws.amazon.com/connect/latest/adminguide/security-profile-best-practices.html">Best Practices for Security Profiles</a> in the <i>Amazon Connect Administrator Guide</i>.</p> </important></p>
    #[allow(unused_mut)]
    async fn update_user_identity_info(
        &self,
        input: UpdateUserIdentityInfoRequest,
    ) -> Result<(), RusotoError<UpdateUserIdentityInfoError>> {
        let request_uri = format!(
            "/users/{instance_id}/{user_id}/identity-info",
            instance_id = input.instance_id,
            user_id = input.user_id
        );

        let mut request = SignedRequest::new("POST", "connect", &self.region, &request_uri);
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
            ::std::mem::drop(response);

            Ok(())
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(UpdateUserIdentityInfoError::from_response(response))
        }
    }

    /// <p>Updates the phone configuration settings for the specified user.</p>
    #[allow(unused_mut)]
    async fn update_user_phone_config(
        &self,
        input: UpdateUserPhoneConfigRequest,
    ) -> Result<(), RusotoError<UpdateUserPhoneConfigError>> {
        let request_uri = format!(
            "/users/{instance_id}/{user_id}/phone-config",
            instance_id = input.instance_id,
            user_id = input.user_id
        );

        let mut request = SignedRequest::new("POST", "connect", &self.region, &request_uri);
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
            ::std::mem::drop(response);

            Ok(())
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(UpdateUserPhoneConfigError::from_response(response))
        }
    }

    /// <p>Assigns the specified routing profile to the specified user.</p>
    #[allow(unused_mut)]
    async fn update_user_routing_profile(
        &self,
        input: UpdateUserRoutingProfileRequest,
    ) -> Result<(), RusotoError<UpdateUserRoutingProfileError>> {
        let request_uri = format!(
            "/users/{instance_id}/{user_id}/routing-profile",
            instance_id = input.instance_id,
            user_id = input.user_id
        );

        let mut request = SignedRequest::new("POST", "connect", &self.region, &request_uri);
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
            ::std::mem::drop(response);

            Ok(())
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(UpdateUserRoutingProfileError::from_response(response))
        }
    }

    /// <p>Assigns the specified security profiles to the specified user.</p>
    #[allow(unused_mut)]
    async fn update_user_security_profiles(
        &self,
        input: UpdateUserSecurityProfilesRequest,
    ) -> Result<(), RusotoError<UpdateUserSecurityProfilesError>> {
        let request_uri = format!(
            "/users/{instance_id}/{user_id}/security-profiles",
            instance_id = input.instance_id,
            user_id = input.user_id
        );

        let mut request = SignedRequest::new("POST", "connect", &self.region, &request_uri);
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
            ::std::mem::drop(response);

            Ok(())
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(UpdateUserSecurityProfilesError::from_response(response))
        }
    }
}
