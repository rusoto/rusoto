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

#[allow(warnings)]
use hyper::Client;
use hyper::status::StatusCode;
use rusoto_core::request::DispatchSignedRequest;
use rusoto_core::region;

use std::fmt;
use std::error::Error;
use std::io;
use std::io::Read;
use rusoto_core::request::HttpDispatchError;
use rusoto_core::credential::{CredentialsError, ProvideAwsCredentials};

use serde_json;
use rusoto_core::param::{Params, ServiceParams};
use rusoto_core::signature::SignedRequest;
use serde_json::from_str;
use serde_json::Value as SerdeJsonValue;
#[derive(Default, Debug, Clone, Serialize)]
pub struct AssociateRoleToGroupRequest {
    /// The unique Id of the AWS Greengrass Group
    #[serde(rename = "GroupId")]
    pub group_id: String,
    /// Role arn you wish to associate with this group.
    #[serde(rename = "RoleArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_arn: Option<String>,
}

#[derive(Default, Debug, Clone, Deserialize)]
pub struct AssociateRoleToGroupResponse {
    /// Time the role arn was associated to your group.
    #[serde(rename = "AssociatedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub associated_at: Option<String>,
}

#[derive(Default, Debug, Clone, Serialize)]
pub struct AssociateServiceRoleToAccountRequest {
    /// Role arn you wish to associate with this account.
    #[serde(rename = "RoleArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_arn: Option<String>,
}

#[derive(Default, Debug, Clone, Deserialize)]
pub struct AssociateServiceRoleToAccountResponse {
    /// Time when the service role was associated to the account.
    #[serde(rename = "AssociatedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub associated_at: Option<String>,
}

/// Connectivity Info
#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct ConnectivityInfo {
    /// Endpoint for the GGC. Can be an IP address or DNS.
    #[serde(rename = "HostAddress")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub host_address: Option<String>,
    /// Element Id for this entry in the list.
    #[serde(rename = "Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// Metadata for this endpoint.
    #[serde(rename = "Metadata")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<String>,
    /// Port of the GGC. Usually 8883.
    #[serde(rename = "PortNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub port_number: Option<i64>,
}

/// Information on the core
#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct Core {
    /// Certificate arn of the core.
    #[serde(rename = "CertificateArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate_arn: Option<String>,
    /// Element Id for this entry in the list.
    #[serde(rename = "Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// If true, the local shadow value automatically syncs with the cloud's shadow state.
    #[serde(rename = "SyncShadow")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sync_shadow: Option<bool>,
    /// Thing arn of the core.
    #[serde(rename = "ThingArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thing_arn: Option<String>,
}

/// Information on core definition version
#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct CoreDefinitionVersion {
    /// Cores in the definition version.
    #[serde(rename = "Cores")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cores: Option<Vec<Core>>,
}

/// Information on the core definition request
#[derive(Default, Debug, Clone, Serialize)]
pub struct CreateCoreDefinitionRequest {
    /// The client token used to request idempotent operations.
    #[serde(rename = "AmznClientToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amzn_client_token: Option<String>,
    /// Information on the initial version
    #[serde(rename = "InitialVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub initial_version: Option<CoreDefinitionVersion>,
    /// name of the core definition
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Default, Debug, Clone, Deserialize)]
pub struct CreateCoreDefinitionResponse {
    /// Arn of the definition.
    #[serde(rename = "Arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// Timestamp of when the definition was created.
    #[serde(rename = "CreationTimestamp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_timestamp: Option<String>,
    /// Id of the definition.
    #[serde(rename = "Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// Last updated timestamp of the definition.
    #[serde(rename = "LastUpdatedTimestamp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_timestamp: Option<String>,
    /// Last version of the definition.
    #[serde(rename = "LatestVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latest_version: Option<String>,
    /// Latest version arn of the definition.
    #[serde(rename = "LatestVersionArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latest_version_arn: Option<String>,
    /// Name of the definition.
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Default, Debug, Clone, Serialize)]
pub struct CreateCoreDefinitionVersionRequest {
    /// The client token used to request idempotent operations.
    #[serde(rename = "AmznClientToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amzn_client_token: Option<String>,
    /// core definition Id
    #[serde(rename = "CoreDefinitionId")]
    pub core_definition_id: String,
    /// Cores in the definition version.
    #[serde(rename = "Cores")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cores: Option<Vec<Core>>,
}

#[derive(Default, Debug, Clone, Deserialize)]
pub struct CreateCoreDefinitionVersionResponse {
    /// Arn of the version.
    #[serde(rename = "Arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// Timestamp of when the version was created.
    #[serde(rename = "CreationTimestamp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_timestamp: Option<String>,
    /// Id of the resource container.
    #[serde(rename = "Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// Unique Id of a version.
    #[serde(rename = "Version")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

#[derive(Default, Debug, Clone, Serialize)]
pub struct CreateDeploymentRequest {
    /// The client token used to request idempotent operations.
    #[serde(rename = "AmznClientToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amzn_client_token: Option<String>,
    /// Id of the deployment if you wish to redeploy a previous deployment.
    #[serde(rename = "DeploymentId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deployment_id: Option<String>,
    /// Type of deployment
    #[serde(rename = "DeploymentType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deployment_type: Option<String>,
    /// The unique Id of the AWS Greengrass Group
    #[serde(rename = "GroupId")]
    pub group_id: String,
    /// Group Version you wish to deploy.
    #[serde(rename = "GroupVersionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_version_id: Option<String>,
}

#[derive(Default, Debug, Clone, Deserialize)]
pub struct CreateDeploymentResponse {
    /// Arn of the deployment.
    #[serde(rename = "DeploymentArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deployment_arn: Option<String>,
    /// Id of the deployment.
    #[serde(rename = "DeploymentId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deployment_id: Option<String>,
}

#[derive(Default, Debug, Clone, Serialize)]
pub struct CreateDeviceDefinitionRequest {
    /// The client token used to request idempotent operations.
    #[serde(rename = "AmznClientToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amzn_client_token: Option<String>,
    /// Information on the initial version
    #[serde(rename = "InitialVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub initial_version: Option<DeviceDefinitionVersion>,
    /// name of the device definition
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Default, Debug, Clone, Deserialize)]
pub struct CreateDeviceDefinitionResponse {
    /// Arn of the definition.
    #[serde(rename = "Arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// Timestamp of when the definition was created.
    #[serde(rename = "CreationTimestamp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_timestamp: Option<String>,
    /// Id of the definition.
    #[serde(rename = "Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// Last updated timestamp of the definition.
    #[serde(rename = "LastUpdatedTimestamp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_timestamp: Option<String>,
    /// Last version of the definition.
    #[serde(rename = "LatestVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latest_version: Option<String>,
    /// Latest version arn of the definition.
    #[serde(rename = "LatestVersionArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latest_version_arn: Option<String>,
    /// Name of the definition.
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Default, Debug, Clone, Serialize)]
pub struct CreateDeviceDefinitionVersionRequest {
    /// The client token used to request idempotent operations.
    #[serde(rename = "AmznClientToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amzn_client_token: Option<String>,
    /// device definition Id
    #[serde(rename = "DeviceDefinitionId")]
    pub device_definition_id: String,
    /// Devices in the definition version.
    #[serde(rename = "Devices")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub devices: Option<Vec<Device>>,
}

#[derive(Default, Debug, Clone, Deserialize)]
pub struct CreateDeviceDefinitionVersionResponse {
    /// Arn of the version.
    #[serde(rename = "Arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// Timestamp of when the version was created.
    #[serde(rename = "CreationTimestamp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_timestamp: Option<String>,
    /// Id of the resource container.
    #[serde(rename = "Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// Unique Id of a version.
    #[serde(rename = "Version")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

#[derive(Default, Debug, Clone, Serialize)]
pub struct CreateFunctionDefinitionRequest {
    /// The client token used to request idempotent operations.
    #[serde(rename = "AmznClientToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amzn_client_token: Option<String>,
    /// Information on the initial version
    #[serde(rename = "InitialVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub initial_version: Option<FunctionDefinitionVersion>,
    /// name of the function definition
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Default, Debug, Clone, Deserialize)]
pub struct CreateFunctionDefinitionResponse {
    /// Arn of the definition.
    #[serde(rename = "Arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// Timestamp of when the definition was created.
    #[serde(rename = "CreationTimestamp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_timestamp: Option<String>,
    /// Id of the definition.
    #[serde(rename = "Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// Last updated timestamp of the definition.
    #[serde(rename = "LastUpdatedTimestamp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_timestamp: Option<String>,
    /// Last version of the definition.
    #[serde(rename = "LatestVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latest_version: Option<String>,
    /// Latest version arn of the definition.
    #[serde(rename = "LatestVersionArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latest_version_arn: Option<String>,
    /// Name of the definition.
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

/// Function definition version
#[derive(Default, Debug, Clone, Serialize)]
pub struct CreateFunctionDefinitionVersionRequest {
    /// The client token used to request idempotent operations.
    #[serde(rename = "AmznClientToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amzn_client_token: Option<String>,
    /// the unique Id of the lambda definition
    #[serde(rename = "FunctionDefinitionId")]
    pub function_definition_id: String,
    /// Lambda functions in this function definition version.
    #[serde(rename = "Functions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub functions: Option<Vec<Function>>,
}

#[derive(Default, Debug, Clone, Deserialize)]
pub struct CreateFunctionDefinitionVersionResponse {
    /// Arn of the version.
    #[serde(rename = "Arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// Timestamp of when the version was created.
    #[serde(rename = "CreationTimestamp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_timestamp: Option<String>,
    /// Id of the resource container.
    #[serde(rename = "Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// Unique Id of a version.
    #[serde(rename = "Version")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

#[derive(Default, Debug, Clone, Serialize)]
pub struct CreateGroupCertificateAuthorityRequest {
    /// The client token used to request idempotent operations.
    #[serde(rename = "AmznClientToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amzn_client_token: Option<String>,
    /// The unique Id of the AWS Greengrass Group
    #[serde(rename = "GroupId")]
    pub group_id: String,
}

#[derive(Default, Debug, Clone, Deserialize)]
pub struct CreateGroupCertificateAuthorityResponse {
    /// Arn of the group certificate authority.
    #[serde(rename = "GroupCertificateAuthorityArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_certificate_authority_arn: Option<String>,
}

#[derive(Default, Debug, Clone, Serialize)]
pub struct CreateGroupRequest {
    /// The client token used to request idempotent operations.
    #[serde(rename = "AmznClientToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amzn_client_token: Option<String>,
    /// Information on the initial version
    #[serde(rename = "InitialVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub initial_version: Option<GroupVersion>,
    /// name of the group
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Default, Debug, Clone, Deserialize)]
pub struct CreateGroupResponse {
    /// Arn of the definition.
    #[serde(rename = "Arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// Timestamp of when the definition was created.
    #[serde(rename = "CreationTimestamp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_timestamp: Option<String>,
    /// Id of the definition.
    #[serde(rename = "Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// Last updated timestamp of the definition.
    #[serde(rename = "LastUpdatedTimestamp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_timestamp: Option<String>,
    /// Last version of the definition.
    #[serde(rename = "LatestVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latest_version: Option<String>,
    /// Latest version arn of the definition.
    #[serde(rename = "LatestVersionArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latest_version_arn: Option<String>,
    /// Name of the definition.
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Default, Debug, Clone, Serialize)]
pub struct CreateGroupVersionRequest {
    /// The client token used to request idempotent operations.
    #[serde(rename = "AmznClientToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amzn_client_token: Option<String>,
    /// Core definition version arn for this group.
    #[serde(rename = "CoreDefinitionVersionArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub core_definition_version_arn: Option<String>,
    /// Device definition version arn for this group.
    #[serde(rename = "DeviceDefinitionVersionArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_definition_version_arn: Option<String>,
    /// Function definition version arn for this group.
    #[serde(rename = "FunctionDefinitionVersionArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub function_definition_version_arn: Option<String>,
    /// The unique Id of the AWS Greengrass Group
    #[serde(rename = "GroupId")]
    pub group_id: String,
    /// Logger definitionv ersion arn for this group.
    #[serde(rename = "LoggerDefinitionVersionArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logger_definition_version_arn: Option<String>,
    /// Subscription definition version arn for this group.
    #[serde(rename = "SubscriptionDefinitionVersionArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscription_definition_version_arn: Option<String>,
}

#[derive(Default, Debug, Clone, Deserialize)]
pub struct CreateGroupVersionResponse {
    /// Arn of the version.
    #[serde(rename = "Arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// Timestamp of when the version was created.
    #[serde(rename = "CreationTimestamp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_timestamp: Option<String>,
    /// Id of the resource container.
    #[serde(rename = "Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// Unique Id of a version.
    #[serde(rename = "Version")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

#[derive(Default, Debug, Clone, Serialize)]
pub struct CreateLoggerDefinitionRequest {
    /// The client token used to request idempotent operations.
    #[serde(rename = "AmznClientToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amzn_client_token: Option<String>,
    /// Information on the initial version
    #[serde(rename = "InitialVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub initial_version: Option<LoggerDefinitionVersion>,
    /// name of the logger definition
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Default, Debug, Clone, Deserialize)]
pub struct CreateLoggerDefinitionResponse {
    /// Arn of the definition.
    #[serde(rename = "Arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// Timestamp of when the definition was created.
    #[serde(rename = "CreationTimestamp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_timestamp: Option<String>,
    /// Id of the definition.
    #[serde(rename = "Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// Last updated timestamp of the definition.
    #[serde(rename = "LastUpdatedTimestamp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_timestamp: Option<String>,
    /// Last version of the definition.
    #[serde(rename = "LatestVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latest_version: Option<String>,
    /// Latest version arn of the definition.
    #[serde(rename = "LatestVersionArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latest_version_arn: Option<String>,
    /// Name of the definition.
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Default, Debug, Clone, Serialize)]
pub struct CreateLoggerDefinitionVersionRequest {
    /// The client token used to request idempotent operations.
    #[serde(rename = "AmznClientToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amzn_client_token: Option<String>,
    /// logger definition Id
    #[serde(rename = "LoggerDefinitionId")]
    pub logger_definition_id: String,
    /// List of loggers.
    #[serde(rename = "Loggers")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub loggers: Option<Vec<Logger>>,
}

#[derive(Default, Debug, Clone, Deserialize)]
pub struct CreateLoggerDefinitionVersionResponse {
    /// Arn of the version.
    #[serde(rename = "Arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// Timestamp of when the version was created.
    #[serde(rename = "CreationTimestamp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_timestamp: Option<String>,
    /// Id of the resource container.
    #[serde(rename = "Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// Unique Id of a version.
    #[serde(rename = "Version")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

#[derive(Default, Debug, Clone, Serialize)]
pub struct CreateSubscriptionDefinitionRequest {
    /// The client token used to request idempotent operations.
    #[serde(rename = "AmznClientToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amzn_client_token: Option<String>,
    /// Information on the initial version
    #[serde(rename = "InitialVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub initial_version: Option<SubscriptionDefinitionVersion>,
    /// name of the subscription definition
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Default, Debug, Clone, Deserialize)]
pub struct CreateSubscriptionDefinitionResponse {
    /// Arn of the definition.
    #[serde(rename = "Arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// Timestamp of when the definition was created.
    #[serde(rename = "CreationTimestamp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_timestamp: Option<String>,
    /// Id of the definition.
    #[serde(rename = "Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// Last updated timestamp of the definition.
    #[serde(rename = "LastUpdatedTimestamp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_timestamp: Option<String>,
    /// Last version of the definition.
    #[serde(rename = "LatestVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latest_version: Option<String>,
    /// Latest version arn of the definition.
    #[serde(rename = "LatestVersionArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latest_version_arn: Option<String>,
    /// Name of the definition.
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Default, Debug, Clone, Serialize)]
pub struct CreateSubscriptionDefinitionVersionRequest {
    /// The client token used to request idempotent operations.
    #[serde(rename = "AmznClientToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amzn_client_token: Option<String>,
    /// subscription definition Id
    #[serde(rename = "SubscriptionDefinitionId")]
    pub subscription_definition_id: String,
    /// Subscriptions in the version.
    #[serde(rename = "Subscriptions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscriptions: Option<Vec<Subscription>>,
}

#[derive(Default, Debug, Clone, Deserialize)]
pub struct CreateSubscriptionDefinitionVersionResponse {
    /// Arn of the version.
    #[serde(rename = "Arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// Timestamp of when the version was created.
    #[serde(rename = "CreationTimestamp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_timestamp: Option<String>,
    /// Id of the resource container.
    #[serde(rename = "Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// Unique Id of a version.
    #[serde(rename = "Version")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

/// Information on the Definition
#[derive(Default, Debug, Clone, Deserialize)]
pub struct DefinitionInformation {
    /// Arn of the definition.
    #[serde(rename = "Arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// Timestamp of when the definition was created.
    #[serde(rename = "CreationTimestamp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_timestamp: Option<String>,
    /// Id of the definition.
    #[serde(rename = "Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// Last updated timestamp of the definition.
    #[serde(rename = "LastUpdatedTimestamp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_timestamp: Option<String>,
    /// Last version of the definition.
    #[serde(rename = "LatestVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latest_version: Option<String>,
    /// Latest version arn of the definition.
    #[serde(rename = "LatestVersionArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latest_version_arn: Option<String>,
    /// Name of the definition.
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Default, Debug, Clone, Serialize)]
pub struct DeleteCoreDefinitionRequest {
    /// core definition Id
    #[serde(rename = "CoreDefinitionId")]
    pub core_definition_id: String,
}

#[derive(Default, Debug, Clone, Deserialize)]
pub struct DeleteCoreDefinitionResponse;

#[derive(Default, Debug, Clone, Serialize)]
pub struct DeleteDeviceDefinitionRequest {
    /// device definition Id
    #[serde(rename = "DeviceDefinitionId")]
    pub device_definition_id: String,
}

#[derive(Default, Debug, Clone, Deserialize)]
pub struct DeleteDeviceDefinitionResponse;

#[derive(Default, Debug, Clone, Serialize)]
pub struct DeleteFunctionDefinitionRequest {
    /// the unique Id of the lambda definition
    #[serde(rename = "FunctionDefinitionId")]
    pub function_definition_id: String,
}

#[derive(Default, Debug, Clone, Deserialize)]
pub struct DeleteFunctionDefinitionResponse;

#[derive(Default, Debug, Clone, Serialize)]
pub struct DeleteGroupRequest {
    /// The unique Id of the AWS Greengrass Group
    #[serde(rename = "GroupId")]
    pub group_id: String,
}

#[derive(Default, Debug, Clone, Deserialize)]
pub struct DeleteGroupResponse;

#[derive(Default, Debug, Clone, Serialize)]
pub struct DeleteLoggerDefinitionRequest {
    /// logger definition Id
    #[serde(rename = "LoggerDefinitionId")]
    pub logger_definition_id: String,
}

#[derive(Default, Debug, Clone, Deserialize)]
pub struct DeleteLoggerDefinitionResponse;

#[derive(Default, Debug, Clone, Serialize)]
pub struct DeleteSubscriptionDefinitionRequest {
    /// subscription definition Id
    #[serde(rename = "SubscriptionDefinitionId")]
    pub subscription_definition_id: String,
}

#[derive(Default, Debug, Clone, Deserialize)]
pub struct DeleteSubscriptionDefinitionResponse;

/// Information on the deployment
#[derive(Default, Debug, Clone, Deserialize)]
pub struct Deployment {
    /// Timestamp when the deployment was created.
    #[serde(rename = "CreatedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    /// Arn of the deployment.
    #[serde(rename = "DeploymentArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deployment_arn: Option<String>,
    /// Id of the deployment.
    #[serde(rename = "DeploymentId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deployment_id: Option<String>,
    /// Arn of the group for this deployment.
    #[serde(rename = "GroupArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_arn: Option<String>,
}

/// Information on a Device
#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct Device {
    /// Certificate arn of the device.
    #[serde(rename = "CertificateArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate_arn: Option<String>,
    /// Element Id for this entry in the list.
    #[serde(rename = "Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// If true, the local shadow value automatically syncs with the cloud's shadow state.
    #[serde(rename = "SyncShadow")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sync_shadow: Option<bool>,
    /// Thing arn of the device.
    #[serde(rename = "ThingArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thing_arn: Option<String>,
}

/// Information on device definition version
#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct DeviceDefinitionVersion {
    /// Devices in the definition version.
    #[serde(rename = "Devices")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub devices: Option<Vec<Device>>,
}

#[derive(Default, Debug, Clone, Serialize)]
pub struct DisassociateRoleFromGroupRequest {
    /// The unique Id of the AWS Greengrass Group
    #[serde(rename = "GroupId")]
    pub group_id: String,
}

#[derive(Default, Debug, Clone, Deserialize)]
pub struct DisassociateRoleFromGroupResponse {
    /// Time when the role was disassociated from the group.
    #[serde(rename = "DisassociatedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disassociated_at: Option<String>,
}

#[derive(Default, Debug, Clone, Serialize)]
pub struct DisassociateServiceRoleFromAccountRequest;

#[derive(Default, Debug, Clone, Deserialize)]
pub struct DisassociateServiceRoleFromAccountResponse {
    /// Time when the service role was disassociated from the account.
    #[serde(rename = "DisassociatedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disassociated_at: Option<String>,
}

/// Empty
#[derive(Default, Debug, Clone)]
pub struct Empty;

/// ErrorDetail
#[derive(Default, Debug, Clone)]
pub struct ErrorDetail {
    /// Detailed Error Code
    pub detailed_error_code: Option<String>,
    /// Detailed Error Message
    pub detailed_error_message: Option<String>,
}

/// Information on function
#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct Function {
    /// Arn of the Lambda function.
    #[serde(rename = "FunctionArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub function_arn: Option<String>,
    /// Configuration of the function
    #[serde(rename = "FunctionConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub function_configuration: Option<FunctionConfiguration>,
    /// Id of the function in this version.
    #[serde(rename = "Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}

/// Configuration of the function
#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct FunctionConfiguration {
    /// Environment of the function configuration
    #[serde(rename = "Environment")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub environment: Option<FunctionConfigurationEnvironment>,
    /// Execution Arguments
    #[serde(rename = "ExecArgs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exec_args: Option<String>,
    /// Executable
    #[serde(rename = "Executable")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub executable: Option<String>,
    /// The memory size, in KB, you configured for the function.
    #[serde(rename = "MemorySize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub memory_size: Option<i64>,
    /// Whether the function is pinned or not. Pinned means the function is long-lived and starts when the core starts.
    #[serde(rename = "Pinned")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pinned: Option<bool>,
    /// The function execution time at which Lambda should terminate the function. This timeout still applies to pinned lambdas for each request.
    #[serde(rename = "Timeout")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeout: Option<i64>,
}

/// Environment of the function configuration
#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct FunctionConfigurationEnvironment {
    /// Environment variables for the lambda function.
    #[serde(rename = "Variables")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub variables: Option<::std::collections::HashMap<String, String>>,
}

/// Information on the function definition version
#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct FunctionDefinitionVersion {
    /// Lambda functions in this function definition version.
    #[serde(rename = "Functions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub functions: Option<Vec<Function>>,
}

/// General Error
#[derive(Default, Debug, Clone)]
pub struct GeneralError {
    /// Error Details
    pub error_details: Option<Vec<ErrorDetail>>,
    /// Message
    pub message: Option<String>,
}

#[derive(Default, Debug, Clone, Serialize)]
pub struct GetAssociatedRoleRequest {
    /// The unique Id of the AWS Greengrass Group
    #[serde(rename = "GroupId")]
    pub group_id: String,
}

#[derive(Default, Debug, Clone, Deserialize)]
pub struct GetAssociatedRoleResponse {
    /// Time when the role was associated for the group.
    #[serde(rename = "AssociatedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub associated_at: Option<String>,
    /// Arn of the role that is associated with the group.
    #[serde(rename = "RoleArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_arn: Option<String>,
}

#[derive(Default, Debug, Clone, Serialize)]
pub struct GetConnectivityInfoRequest {
    /// Thing Name
    #[serde(rename = "ThingName")]
    pub thing_name: String,
}

#[derive(Default, Debug, Clone, Deserialize)]
pub struct GetConnectivityInfoResponse {
    /// Connectivity info array
    #[serde(rename = "ConnectivityInfo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connectivity_info: Option<Vec<ConnectivityInfo>>,
    /// Response Text
    #[serde(rename = "Message")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

#[derive(Default, Debug, Clone, Serialize)]
pub struct GetCoreDefinitionRequest {
    /// core definition Id
    #[serde(rename = "CoreDefinitionId")]
    pub core_definition_id: String,
}

#[derive(Default, Debug, Clone, Deserialize)]
pub struct GetCoreDefinitionResponse {
    /// Arn of the definition.
    #[serde(rename = "Arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// Timestamp of when the definition was created.
    #[serde(rename = "CreationTimestamp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_timestamp: Option<String>,
    /// Id of the definition.
    #[serde(rename = "Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// Last updated timestamp of the definition.
    #[serde(rename = "LastUpdatedTimestamp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_timestamp: Option<String>,
    /// Last version of the definition.
    #[serde(rename = "LatestVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latest_version: Option<String>,
    /// Latest version arn of the definition.
    #[serde(rename = "LatestVersionArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latest_version_arn: Option<String>,
    /// Name of the definition.
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Default, Debug, Clone, Serialize)]
pub struct GetCoreDefinitionVersionRequest {
    /// core definition Id
    #[serde(rename = "CoreDefinitionId")]
    pub core_definition_id: String,
    /// core definition version Id
    #[serde(rename = "CoreDefinitionVersionId")]
    pub core_definition_version_id: String,
}

#[derive(Default, Debug, Clone, Deserialize)]
pub struct GetCoreDefinitionVersionResponse {
    /// Arn of the core definition version.
    #[serde(rename = "Arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// Timestamp of when the core definition version was created.
    #[serde(rename = "CreationTimestamp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_timestamp: Option<String>,
    /// Information on definition
    #[serde(rename = "Definition")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub definition: Option<CoreDefinitionVersion>,
    /// Id of the core definition the version belongs to.
    #[serde(rename = "Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// Version of the core definition version.
    #[serde(rename = "Version")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

#[derive(Default, Debug, Clone, Serialize)]
pub struct GetDeploymentStatusRequest {
    /// the deployment Id
    #[serde(rename = "DeploymentId")]
    pub deployment_id: String,
    /// The unique Id of the AWS Greengrass Group
    #[serde(rename = "GroupId")]
    pub group_id: String,
}

#[derive(Default, Debug, Clone, Deserialize)]
pub struct GetDeploymentStatusResponse {
    /// Status of the deployment.
    #[serde(rename = "DeploymentStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deployment_status: Option<String>,
    /// Error Message
    #[serde(rename = "ErrorMessage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
    /// Last time the deployment status was updated.
    #[serde(rename = "UpdatedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
}

#[derive(Default, Debug, Clone, Serialize)]
pub struct GetDeviceDefinitionRequest {
    /// device definition Id
    #[serde(rename = "DeviceDefinitionId")]
    pub device_definition_id: String,
}

#[derive(Default, Debug, Clone, Deserialize)]
pub struct GetDeviceDefinitionResponse {
    /// Arn of the definition.
    #[serde(rename = "Arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// Timestamp of when the definition was created.
    #[serde(rename = "CreationTimestamp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_timestamp: Option<String>,
    /// Id of the definition.
    #[serde(rename = "Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// Last updated timestamp of the definition.
    #[serde(rename = "LastUpdatedTimestamp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_timestamp: Option<String>,
    /// Last version of the definition.
    #[serde(rename = "LatestVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latest_version: Option<String>,
    /// Latest version arn of the definition.
    #[serde(rename = "LatestVersionArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latest_version_arn: Option<String>,
    /// Name of the definition.
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Default, Debug, Clone, Serialize)]
pub struct GetDeviceDefinitionVersionRequest {
    /// device definition Id
    #[serde(rename = "DeviceDefinitionId")]
    pub device_definition_id: String,
    /// device definition version Id
    #[serde(rename = "DeviceDefinitionVersionId")]
    pub device_definition_version_id: String,
}

#[derive(Default, Debug, Clone, Deserialize)]
pub struct GetDeviceDefinitionVersionResponse {
    /// Arn of the device definition version.
    #[serde(rename = "Arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// Timestamp of when the device definition version was created.
    #[serde(rename = "CreationTimestamp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_timestamp: Option<String>,
    /// Device definition version
    #[serde(rename = "Definition")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub definition: Option<DeviceDefinitionVersion>,
    /// Id of the device definition the version belongs to.
    #[serde(rename = "Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// Version of the device definition version.
    #[serde(rename = "Version")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

#[derive(Default, Debug, Clone, Serialize)]
pub struct GetFunctionDefinitionRequest {
    /// the unique Id of the lambda definition
    #[serde(rename = "FunctionDefinitionId")]
    pub function_definition_id: String,
}

#[derive(Default, Debug, Clone, Deserialize)]
pub struct GetFunctionDefinitionResponse {
    /// Arn of the definition.
    #[serde(rename = "Arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// Timestamp of when the definition was created.
    #[serde(rename = "CreationTimestamp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_timestamp: Option<String>,
    /// Id of the definition.
    #[serde(rename = "Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// Last updated timestamp of the definition.
    #[serde(rename = "LastUpdatedTimestamp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_timestamp: Option<String>,
    /// Last version of the definition.
    #[serde(rename = "LatestVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latest_version: Option<String>,
    /// Latest version arn of the definition.
    #[serde(rename = "LatestVersionArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latest_version_arn: Option<String>,
    /// Name of the definition.
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Default, Debug, Clone, Serialize)]
pub struct GetFunctionDefinitionVersionRequest {
    /// the unique Id of the lambda definition
    #[serde(rename = "FunctionDefinitionId")]
    pub function_definition_id: String,
    /// Function definition version Id
    #[serde(rename = "FunctionDefinitionVersionId")]
    pub function_definition_version_id: String,
}

#[derive(Default, Debug, Clone, Deserialize)]
pub struct GetFunctionDefinitionVersionResponse {
    /// Arn of the function definition version.
    #[serde(rename = "Arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// Timestamp when the funtion definition version was created.
    #[serde(rename = "CreationTimestamp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_timestamp: Option<String>,
    #[serde(rename = "Definition")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub definition: Option<FunctionDefinitionVersion>,
    /// Id of the function definition the version belongs to.
    #[serde(rename = "Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// Version of the function definition version.
    #[serde(rename = "Version")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

#[derive(Default, Debug, Clone, Serialize)]
pub struct GetGroupCertificateAuthorityRequest {
    /// certificate authority Id
    #[serde(rename = "CertificateAuthorityId")]
    pub certificate_authority_id: String,
    /// The unique Id of the AWS Greengrass Group
    #[serde(rename = "GroupId")]
    pub group_id: String,
}

#[derive(Default, Debug, Clone, Deserialize)]
pub struct GetGroupCertificateAuthorityResponse {
    /// Arn of the certificate authority for the group.
    #[serde(rename = "GroupCertificateAuthorityArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_certificate_authority_arn: Option<String>,
    /// Id of the certificate authority for the group.
    #[serde(rename = "GroupCertificateAuthorityId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_certificate_authority_id: Option<String>,
    /// PEM encoded certificate for the group.
    #[serde(rename = "PemEncodedCertificate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pem_encoded_certificate: Option<String>,
}

#[derive(Default, Debug, Clone, Serialize)]
pub struct GetGroupCertificateConfigurationRequest {
    /// The unique Id of the AWS Greengrass Group
    #[serde(rename = "GroupId")]
    pub group_id: String,
}

#[derive(Default, Debug, Clone, Deserialize)]
pub struct GetGroupCertificateConfigurationResponse {
    /// Amount of time when the certificate authority expires in milliseconds.
    #[serde(rename = "CertificateAuthorityExpiryInMilliseconds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate_authority_expiry_in_milliseconds: Option<String>,
    /// Amount of time when the certificate expires in milliseconds.
    #[serde(rename = "CertificateExpiryInMilliseconds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate_expiry_in_milliseconds: Option<String>,
    /// Id of the group the certificate configuration belongs to.
    #[serde(rename = "GroupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_id: Option<String>,
}

#[derive(Default, Debug, Clone, Serialize)]
pub struct GetGroupRequest {
    /// The unique Id of the AWS Greengrass Group
    #[serde(rename = "GroupId")]
    pub group_id: String,
}

#[derive(Default, Debug, Clone, Deserialize)]
pub struct GetGroupResponse {
    /// Arn of the definition.
    #[serde(rename = "Arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// Timestamp of when the definition was created.
    #[serde(rename = "CreationTimestamp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_timestamp: Option<String>,
    /// Id of the definition.
    #[serde(rename = "Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// Last updated timestamp of the definition.
    #[serde(rename = "LastUpdatedTimestamp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_timestamp: Option<String>,
    /// Last version of the definition.
    #[serde(rename = "LatestVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latest_version: Option<String>,
    /// Latest version arn of the definition.
    #[serde(rename = "LatestVersionArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latest_version_arn: Option<String>,
    /// Name of the definition.
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Default, Debug, Clone, Serialize)]
pub struct GetGroupVersionRequest {
    /// The unique Id of the AWS Greengrass Group
    #[serde(rename = "GroupId")]
    pub group_id: String,
    /// Group version Id
    #[serde(rename = "GroupVersionId")]
    pub group_version_id: String,
}

#[derive(Default, Debug, Clone, Deserialize)]
pub struct GetGroupVersionResponse {
    /// Arn of the group version.
    #[serde(rename = "Arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// Timestamp when the group version was created.
    #[serde(rename = "CreationTimestamp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_timestamp: Option<String>,
    /// Information on the definition
    #[serde(rename = "Definition")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub definition: Option<GroupVersion>,
    /// Id of the group version.
    #[serde(rename = "Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// Unique Id for a version of the Group.
    #[serde(rename = "Version")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

#[derive(Default, Debug, Clone, Serialize)]
pub struct GetLoggerDefinitionRequest {
    /// logger definition Id
    #[serde(rename = "LoggerDefinitionId")]
    pub logger_definition_id: String,
}

#[derive(Default, Debug, Clone, Deserialize)]
pub struct GetLoggerDefinitionResponse {
    /// Arn of the definition.
    #[serde(rename = "Arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// Timestamp of when the definition was created.
    #[serde(rename = "CreationTimestamp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_timestamp: Option<String>,
    /// Id of the definition.
    #[serde(rename = "Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// Last updated timestamp of the definition.
    #[serde(rename = "LastUpdatedTimestamp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_timestamp: Option<String>,
    /// Last version of the definition.
    #[serde(rename = "LatestVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latest_version: Option<String>,
    /// Latest version arn of the definition.
    #[serde(rename = "LatestVersionArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latest_version_arn: Option<String>,
    /// Name of the definition.
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Default, Debug, Clone, Serialize)]
pub struct GetLoggerDefinitionVersionRequest {
    /// logger definition Id
    #[serde(rename = "LoggerDefinitionId")]
    pub logger_definition_id: String,
    /// logger definition version Id
    #[serde(rename = "LoggerDefinitionVersionId")]
    pub logger_definition_version_id: String,
}

#[derive(Default, Debug, Clone, Deserialize)]
pub struct GetLoggerDefinitionVersionResponse {
    /// Arn of the logger definition version.
    #[serde(rename = "Arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// Timestamp of when the logger definition version was created.
    #[serde(rename = "CreationTimestamp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_timestamp: Option<String>,
    /// Information on definition
    #[serde(rename = "Definition")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub definition: Option<LoggerDefinitionVersion>,
    /// Id of the logger definition the version belongs to.
    #[serde(rename = "Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// Version of the logger definition version.
    #[serde(rename = "Version")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

#[derive(Default, Debug, Clone, Serialize)]
pub struct GetServiceRoleForAccountRequest;

#[derive(Default, Debug, Clone, Deserialize)]
pub struct GetServiceRoleForAccountResponse {
    /// Time when the service role was associated to the account.
    #[serde(rename = "AssociatedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub associated_at: Option<String>,
    /// Role arn which is associated to the account.
    #[serde(rename = "RoleArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_arn: Option<String>,
}

#[derive(Default, Debug, Clone, Serialize)]
pub struct GetSubscriptionDefinitionRequest {
    /// subscription definition Id
    #[serde(rename = "SubscriptionDefinitionId")]
    pub subscription_definition_id: String,
}

#[derive(Default, Debug, Clone, Deserialize)]
pub struct GetSubscriptionDefinitionResponse {
    /// Arn of the definition.
    #[serde(rename = "Arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// Timestamp of when the definition was created.
    #[serde(rename = "CreationTimestamp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_timestamp: Option<String>,
    /// Id of the definition.
    #[serde(rename = "Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// Last updated timestamp of the definition.
    #[serde(rename = "LastUpdatedTimestamp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_timestamp: Option<String>,
    /// Last version of the definition.
    #[serde(rename = "LatestVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latest_version: Option<String>,
    /// Latest version arn of the definition.
    #[serde(rename = "LatestVersionArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latest_version_arn: Option<String>,
    /// Name of the definition.
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Default, Debug, Clone, Serialize)]
pub struct GetSubscriptionDefinitionVersionRequest {
    /// subscription definition Id
    #[serde(rename = "SubscriptionDefinitionId")]
    pub subscription_definition_id: String,
    /// subscription definition version Id
    #[serde(rename = "SubscriptionDefinitionVersionId")]
    pub subscription_definition_version_id: String,
}

#[derive(Default, Debug, Clone, Deserialize)]
pub struct GetSubscriptionDefinitionVersionResponse {
    /// Arn of the subscription definition version.
    #[serde(rename = "Arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// Timestamp of when the subscription definition version was created.
    #[serde(rename = "CreationTimestamp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_timestamp: Option<String>,
    /// Information on the definition
    #[serde(rename = "Definition")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub definition: Option<SubscriptionDefinitionVersion>,
    /// Id of the subscription definition the version belongs to.
    #[serde(rename = "Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// Version of the subscription definition version.
    #[serde(rename = "Version")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

/// Information on group certificate authority properties
#[derive(Default, Debug, Clone, Deserialize)]
pub struct GroupCertificateAuthorityProperties {
    /// Arn of the certificate authority for the group.
    #[serde(rename = "GroupCertificateAuthorityArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_certificate_authority_arn: Option<String>,
    /// Id of the certificate authority for the group.
    #[serde(rename = "GroupCertificateAuthorityId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_certificate_authority_id: Option<String>,
}

/// Information on the group certificate configuration
#[derive(Default, Debug, Clone)]
pub struct GroupCertificateConfiguration {
    /// Amount of time when the certificate authority expires in milliseconds.
    pub certificate_authority_expiry_in_milliseconds: Option<String>,
    /// Amount of time when the certificate expires in milliseconds.
    pub certificate_expiry_in_milliseconds: Option<String>,
    /// Id of the group the certificate configuration belongs to.
    pub group_id: Option<String>,
}

/// Information of a group
#[derive(Default, Debug, Clone, Deserialize)]
pub struct GroupInformation {
    /// Arn of a group.
    #[serde(rename = "Arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// Timestamp of when the group was created.
    #[serde(rename = "CreationTimestamp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_timestamp: Option<String>,
    /// Id of a group.
    #[serde(rename = "Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// Last updated timestamp of the group.
    #[serde(rename = "LastUpdatedTimestamp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_timestamp: Option<String>,
    /// Last version of the group.
    #[serde(rename = "LatestVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latest_version: Option<String>,
    /// Latest version arn of the group.
    #[serde(rename = "LatestVersionArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latest_version_arn: Option<String>,
    /// Name of a group.
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

/// Information on group version
#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct GroupVersion {
    /// Core definition version arn for this group.
    #[serde(rename = "CoreDefinitionVersionArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub core_definition_version_arn: Option<String>,
    /// Device definition version arn for this group.
    #[serde(rename = "DeviceDefinitionVersionArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_definition_version_arn: Option<String>,
    /// Function definition version arn for this group.
    #[serde(rename = "FunctionDefinitionVersionArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub function_definition_version_arn: Option<String>,
    /// Logger definitionv ersion arn for this group.
    #[serde(rename = "LoggerDefinitionVersionArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logger_definition_version_arn: Option<String>,
    /// Subscription definition version arn for this group.
    #[serde(rename = "SubscriptionDefinitionVersionArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscription_definition_version_arn: Option<String>,
}

#[derive(Default, Debug, Clone, Serialize)]
pub struct ListCoreDefinitionVersionsRequest {
    /// core definition Id
    #[serde(rename = "CoreDefinitionId")]
    pub core_definition_id: String,
    /// Specifies the maximum number of list results to be returned in this page
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<String>,
    /// Specifies the pagination token used when iterating through a paginated request
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, Deserialize)]
pub struct ListCoreDefinitionVersionsResponse {
    /// The token for the next set of results, or ''null'' if there are no additional results.
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// Versions
    #[serde(rename = "Versions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub versions: Option<Vec<VersionInformation>>,
}

#[derive(Default, Debug, Clone, Serialize)]
pub struct ListCoreDefinitionsRequest {
    /// Specifies the maximum number of list results to be returned in this page
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<String>,
    /// Specifies the pagination token used when iterating through a paginated request
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, Deserialize)]
pub struct ListCoreDefinitionsResponse {
    /// Definitions
    #[serde(rename = "Definitions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub definitions: Option<Vec<DefinitionInformation>>,
    /// The token for the next set of results, or ''null'' if there are no additional results.
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

/// List of definition response
#[derive(Default, Debug, Clone)]
pub struct ListDefinitionsResponse {
    /// Definitions
    pub definitions: Option<Vec<DefinitionInformation>>,
    /// The token for the next set of results, or ''null'' if there are no additional results.
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, Serialize)]
pub struct ListDeploymentsRequest {
    /// The unique Id of the AWS Greengrass Group
    #[serde(rename = "GroupId")]
    pub group_id: String,
    /// Specifies the maximum number of list results to be returned in this page
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<String>,
    /// Specifies the pagination token used when iterating through a paginated request
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, Deserialize)]
pub struct ListDeploymentsResponse {
    /// Information on deployments
    #[serde(rename = "Deployments")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deployments: Option<Vec<Deployment>>,
    /// The token for the next set of results, or ''null'' if there are no additional results.
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, Serialize)]
pub struct ListDeviceDefinitionVersionsRequest {
    /// device definition Id
    #[serde(rename = "DeviceDefinitionId")]
    pub device_definition_id: String,
    /// Specifies the maximum number of list results to be returned in this page
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<String>,
    /// Specifies the pagination token used when iterating through a paginated request
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, Deserialize)]
pub struct ListDeviceDefinitionVersionsResponse {
    /// The token for the next set of results, or ''null'' if there are no additional results.
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// Versions
    #[serde(rename = "Versions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub versions: Option<Vec<VersionInformation>>,
}

#[derive(Default, Debug, Clone, Serialize)]
pub struct ListDeviceDefinitionsRequest {
    /// Specifies the maximum number of list results to be returned in this page
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<String>,
    /// Specifies the pagination token used when iterating through a paginated request
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, Deserialize)]
pub struct ListDeviceDefinitionsResponse {
    /// Definitions
    #[serde(rename = "Definitions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub definitions: Option<Vec<DefinitionInformation>>,
    /// The token for the next set of results, or ''null'' if there are no additional results.
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, Serialize)]
pub struct ListFunctionDefinitionVersionsRequest {
    /// the unique Id of the lambda definition
    #[serde(rename = "FunctionDefinitionId")]
    pub function_definition_id: String,
    /// Specifies the maximum number of list results to be returned in this page
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<String>,
    /// Specifies the pagination token used when iterating through a paginated request
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, Deserialize)]
pub struct ListFunctionDefinitionVersionsResponse {
    /// The token for the next set of results, or ''null'' if there are no additional results.
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// Versions
    #[serde(rename = "Versions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub versions: Option<Vec<VersionInformation>>,
}

#[derive(Default, Debug, Clone, Serialize)]
pub struct ListFunctionDefinitionsRequest {
    /// Specifies the maximum number of list results to be returned in this page
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<String>,
    /// Specifies the pagination token used when iterating through a paginated request
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, Deserialize)]
pub struct ListFunctionDefinitionsResponse {
    /// Definitions
    #[serde(rename = "Definitions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub definitions: Option<Vec<DefinitionInformation>>,
    /// The token for the next set of results, or ''null'' if there are no additional results.
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, Serialize)]
pub struct ListGroupCertificateAuthoritiesRequest {
    /// The unique Id of the AWS Greengrass Group
    #[serde(rename = "GroupId")]
    pub group_id: String,
}

#[derive(Default, Debug, Clone, Deserialize)]
pub struct ListGroupCertificateAuthoritiesResponse {
    /// List of certificate authorities associated with the group.
    #[serde(rename = "GroupCertificateAuthorities")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_certificate_authorities: Option<Vec<GroupCertificateAuthorityProperties>>,
}

#[derive(Default, Debug, Clone, Serialize)]
pub struct ListGroupVersionsRequest {
    /// The unique Id of the AWS Greengrass Group
    #[serde(rename = "GroupId")]
    pub group_id: String,
    /// Specifies the maximum number of list results to be returned in this page
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<String>,
    /// Specifies the pagination token used when iterating through a paginated request
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, Deserialize)]
pub struct ListGroupVersionsResponse {
    /// The token for the next set of results, or ''null'' if there are no additional results.
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// Versions
    #[serde(rename = "Versions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub versions: Option<Vec<VersionInformation>>,
}

#[derive(Default, Debug, Clone, Serialize)]
pub struct ListGroupsRequest {
    /// Specifies the maximum number of list results to be returned in this page
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<String>,
    /// Specifies the pagination token used when iterating through a paginated request
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, Deserialize)]
pub struct ListGroupsResponse {
    /// Groups
    #[serde(rename = "Groups")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub groups: Option<Vec<GroupInformation>>,
    /// The token for the next set of results, or ''null'' if there are no additional results.
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, Serialize)]
pub struct ListLoggerDefinitionVersionsRequest {
    /// logger definition Id
    #[serde(rename = "LoggerDefinitionId")]
    pub logger_definition_id: String,
    /// Specifies the maximum number of list results to be returned in this page
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<String>,
    /// Specifies the pagination token used when iterating through a paginated request
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, Deserialize)]
pub struct ListLoggerDefinitionVersionsResponse {
    /// The token for the next set of results, or ''null'' if there are no additional results.
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// Versions
    #[serde(rename = "Versions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub versions: Option<Vec<VersionInformation>>,
}

#[derive(Default, Debug, Clone, Serialize)]
pub struct ListLoggerDefinitionsRequest {
    /// Specifies the maximum number of list results to be returned in this page
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<String>,
    /// Specifies the pagination token used when iterating through a paginated request
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, Deserialize)]
pub struct ListLoggerDefinitionsResponse {
    /// Definitions
    #[serde(rename = "Definitions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub definitions: Option<Vec<DefinitionInformation>>,
    /// The token for the next set of results, or ''null'' if there are no additional results.
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, Serialize)]
pub struct ListSubscriptionDefinitionVersionsRequest {
    /// Specifies the maximum number of list results to be returned in this page
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<String>,
    /// Specifies the pagination token used when iterating through a paginated request
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// subscription definition Id
    #[serde(rename = "SubscriptionDefinitionId")]
    pub subscription_definition_id: String,
}

#[derive(Default, Debug, Clone, Deserialize)]
pub struct ListSubscriptionDefinitionVersionsResponse {
    /// The token for the next set of results, or ''null'' if there are no additional results.
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// Versions
    #[serde(rename = "Versions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub versions: Option<Vec<VersionInformation>>,
}

#[derive(Default, Debug, Clone, Serialize)]
pub struct ListSubscriptionDefinitionsRequest {
    /// Specifies the maximum number of list results to be returned in this page
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<String>,
    /// Specifies the pagination token used when iterating through a paginated request
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, Deserialize)]
pub struct ListSubscriptionDefinitionsResponse {
    /// Definitions
    #[serde(rename = "Definitions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub definitions: Option<Vec<DefinitionInformation>>,
    /// The token for the next set of results, or ''null'' if there are no additional results.
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

/// List of versions response
#[derive(Default, Debug, Clone)]
pub struct ListVersionsResponse {
    /// The token for the next set of results, or ''null'' if there are no additional results.
    pub next_token: Option<String>,
    /// Versions
    pub versions: Option<Vec<VersionInformation>>,
}

/// Information on the Logger
#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct Logger {
    /// The component that will be subject to logs
    #[serde(rename = "Component")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub component: Option<String>,
    /// Element Id for this entry in the list.
    #[serde(rename = "Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// The level of the logs
    #[serde(rename = "Level")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub level: Option<String>,
    /// Amount of hardware space, in KB, to use if file system is used for logging purposes.
    #[serde(rename = "Space")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub space: Option<i64>,
    /// The type which will be use for log output
    #[serde(rename = "Type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}

/// Information on logger definition version
#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct LoggerDefinitionVersion {
    /// List of loggers.
    #[serde(rename = "Loggers")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub loggers: Option<Vec<Logger>>,
}

/// Information on subscription
#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct Subscription {
    /// Element Id for this entry in the list.
    #[serde(rename = "Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// Source of the subscription. Can be a thing arn, lambda arn or word 'cloud'
    #[serde(rename = "Source")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
    /// Subject of the message.
    #[serde(rename = "Subject")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subject: Option<String>,
    /// Where the message is sent to. Can be a thing arn, lambda arn or word 'cloud'.
    #[serde(rename = "Target")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target: Option<String>,
}

/// Information on subscription definition version
#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct SubscriptionDefinitionVersion {
    /// Subscriptions in the version.
    #[serde(rename = "Subscriptions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscriptions: Option<Vec<Subscription>>,
}

/// connectivity info request
#[derive(Default, Debug, Clone, Serialize)]
pub struct UpdateConnectivityInfoRequest {
    /// Connectivity info array
    #[serde(rename = "ConnectivityInfo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connectivity_info: Option<Vec<ConnectivityInfo>>,
    /// Thing Name
    #[serde(rename = "ThingName")]
    pub thing_name: String,
}

#[derive(Default, Debug, Clone, Deserialize)]
pub struct UpdateConnectivityInfoResponse {
    /// Response Text
    #[serde(rename = "Message")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    /// New Version
    #[serde(rename = "Version")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

#[derive(Default, Debug, Clone, Serialize)]
pub struct UpdateCoreDefinitionRequest {
    /// core definition Id
    #[serde(rename = "CoreDefinitionId")]
    pub core_definition_id: String,
    /// name of the definition
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Default, Debug, Clone, Deserialize)]
pub struct UpdateCoreDefinitionResponse;

#[derive(Default, Debug, Clone, Serialize)]
pub struct UpdateDeviceDefinitionRequest {
    /// device definition Id
    #[serde(rename = "DeviceDefinitionId")]
    pub device_definition_id: String,
    /// name of the definition
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Default, Debug, Clone, Deserialize)]
pub struct UpdateDeviceDefinitionResponse;

#[derive(Default, Debug, Clone, Serialize)]
pub struct UpdateFunctionDefinitionRequest {
    /// the unique Id of the lambda definition
    #[serde(rename = "FunctionDefinitionId")]
    pub function_definition_id: String,
    /// name of the definition
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Default, Debug, Clone, Deserialize)]
pub struct UpdateFunctionDefinitionResponse;

#[derive(Default, Debug, Clone, Serialize)]
pub struct UpdateGroupCertificateConfigurationRequest {
    /// Amount of time when the certificate expires in milliseconds.
    #[serde(rename = "CertificateExpiryInMilliseconds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate_expiry_in_milliseconds: Option<String>,
    /// The unique Id of the AWS Greengrass Group
    #[serde(rename = "GroupId")]
    pub group_id: String,
}

#[derive(Default, Debug, Clone, Deserialize)]
pub struct UpdateGroupCertificateConfigurationResponse {
    /// Amount of time when the certificate authority expires in milliseconds.
    #[serde(rename = "CertificateAuthorityExpiryInMilliseconds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate_authority_expiry_in_milliseconds: Option<String>,
    /// Amount of time when the certificate expires in milliseconds.
    #[serde(rename = "CertificateExpiryInMilliseconds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate_expiry_in_milliseconds: Option<String>,
    /// Id of the group the certificate configuration belongs to.
    #[serde(rename = "GroupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_id: Option<String>,
}

#[derive(Default, Debug, Clone, Serialize)]
pub struct UpdateGroupRequest {
    /// The unique Id of the AWS Greengrass Group
    #[serde(rename = "GroupId")]
    pub group_id: String,
    /// name of the definition
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Default, Debug, Clone, Deserialize)]
pub struct UpdateGroupResponse;

#[derive(Default, Debug, Clone, Serialize)]
pub struct UpdateLoggerDefinitionRequest {
    /// logger definition Id
    #[serde(rename = "LoggerDefinitionId")]
    pub logger_definition_id: String,
    /// name of the definition
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Default, Debug, Clone, Deserialize)]
pub struct UpdateLoggerDefinitionResponse;

#[derive(Default, Debug, Clone, Serialize)]
pub struct UpdateSubscriptionDefinitionRequest {
    /// name of the definition
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// subscription definition Id
    #[serde(rename = "SubscriptionDefinitionId")]
    pub subscription_definition_id: String,
}

#[derive(Default, Debug, Clone, Deserialize)]
pub struct UpdateSubscriptionDefinitionResponse;

/// Information on the version
#[derive(Default, Debug, Clone, Deserialize)]
pub struct VersionInformation {
    /// Arn of the version.
    #[serde(rename = "Arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// Timestamp of when the version was created.
    #[serde(rename = "CreationTimestamp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_timestamp: Option<String>,
    /// Id of the resource container.
    #[serde(rename = "Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// Unique Id of a version.
    #[serde(rename = "Version")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

/// Errors returned by AssociateRoleToGroup
#[derive(Debug, PartialEq)]
pub enum AssociateRoleToGroupError {
    ///General Error
    InternalServerError(String),
    ///General Error
    BadRequest(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl AssociateRoleToGroupError {
    pub fn from_body(body: &str) -> AssociateRoleToGroupError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InternalServerErrorException" => {
                        AssociateRoleToGroupError::InternalServerError(String::from(error_message))
                    }
                    "BadRequestException" => {
                        AssociateRoleToGroupError::BadRequest(String::from(error_message))
                    }
                    "ValidationException" => {
                        AssociateRoleToGroupError::Validation(error_message.to_string())
                    }
                    _ => AssociateRoleToGroupError::Unknown(String::from(body)),
                }
            }
            Err(_) => AssociateRoleToGroupError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for AssociateRoleToGroupError {
    fn from(err: serde_json::error::Error) -> AssociateRoleToGroupError {
        AssociateRoleToGroupError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for AssociateRoleToGroupError {
    fn from(err: CredentialsError) -> AssociateRoleToGroupError {
        AssociateRoleToGroupError::Credentials(err)
    }
}
impl From<HttpDispatchError> for AssociateRoleToGroupError {
    fn from(err: HttpDispatchError) -> AssociateRoleToGroupError {
        AssociateRoleToGroupError::HttpDispatch(err)
    }
}
impl From<io::Error> for AssociateRoleToGroupError {
    fn from(err: io::Error) -> AssociateRoleToGroupError {
        AssociateRoleToGroupError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for AssociateRoleToGroupError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for AssociateRoleToGroupError {
    fn description(&self) -> &str {
        match *self {
            AssociateRoleToGroupError::InternalServerError(ref cause) => cause,
            AssociateRoleToGroupError::BadRequest(ref cause) => cause,
            AssociateRoleToGroupError::Validation(ref cause) => cause,
            AssociateRoleToGroupError::Credentials(ref err) => err.description(),
            AssociateRoleToGroupError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            AssociateRoleToGroupError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by AssociateServiceRoleToAccount
#[derive(Debug, PartialEq)]
pub enum AssociateServiceRoleToAccountError {
    ///General Error
    InternalServerError(String),
    ///General Error
    BadRequest(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl AssociateServiceRoleToAccountError {
    pub fn from_body(body: &str) -> AssociateServiceRoleToAccountError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InternalServerErrorException" => {
                        AssociateServiceRoleToAccountError::InternalServerError(String::from(
                            error_message,
                        ))
                    }
                    "BadRequestException" => {
                        AssociateServiceRoleToAccountError::BadRequest(String::from(error_message))
                    }
                    "ValidationException" => {
                        AssociateServiceRoleToAccountError::Validation(error_message.to_string())
                    }
                    _ => AssociateServiceRoleToAccountError::Unknown(String::from(body)),
                }
            }
            Err(_) => AssociateServiceRoleToAccountError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for AssociateServiceRoleToAccountError {
    fn from(err: serde_json::error::Error) -> AssociateServiceRoleToAccountError {
        AssociateServiceRoleToAccountError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for AssociateServiceRoleToAccountError {
    fn from(err: CredentialsError) -> AssociateServiceRoleToAccountError {
        AssociateServiceRoleToAccountError::Credentials(err)
    }
}
impl From<HttpDispatchError> for AssociateServiceRoleToAccountError {
    fn from(err: HttpDispatchError) -> AssociateServiceRoleToAccountError {
        AssociateServiceRoleToAccountError::HttpDispatch(err)
    }
}
impl From<io::Error> for AssociateServiceRoleToAccountError {
    fn from(err: io::Error) -> AssociateServiceRoleToAccountError {
        AssociateServiceRoleToAccountError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for AssociateServiceRoleToAccountError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for AssociateServiceRoleToAccountError {
    fn description(&self) -> &str {
        match *self {
            AssociateServiceRoleToAccountError::InternalServerError(ref cause) => cause,
            AssociateServiceRoleToAccountError::BadRequest(ref cause) => cause,
            AssociateServiceRoleToAccountError::Validation(ref cause) => cause,
            AssociateServiceRoleToAccountError::Credentials(ref err) => err.description(),
            AssociateServiceRoleToAccountError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            AssociateServiceRoleToAccountError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by CreateCoreDefinition
#[derive(Debug, PartialEq)]
pub enum CreateCoreDefinitionError {
    ///General Error
    BadRequest(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl CreateCoreDefinitionError {
    pub fn from_body(body: &str) -> CreateCoreDefinitionError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "BadRequestException" => {
                        CreateCoreDefinitionError::BadRequest(String::from(error_message))
                    }
                    "ValidationException" => {
                        CreateCoreDefinitionError::Validation(error_message.to_string())
                    }
                    _ => CreateCoreDefinitionError::Unknown(String::from(body)),
                }
            }
            Err(_) => CreateCoreDefinitionError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for CreateCoreDefinitionError {
    fn from(err: serde_json::error::Error) -> CreateCoreDefinitionError {
        CreateCoreDefinitionError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for CreateCoreDefinitionError {
    fn from(err: CredentialsError) -> CreateCoreDefinitionError {
        CreateCoreDefinitionError::Credentials(err)
    }
}
impl From<HttpDispatchError> for CreateCoreDefinitionError {
    fn from(err: HttpDispatchError) -> CreateCoreDefinitionError {
        CreateCoreDefinitionError::HttpDispatch(err)
    }
}
impl From<io::Error> for CreateCoreDefinitionError {
    fn from(err: io::Error) -> CreateCoreDefinitionError {
        CreateCoreDefinitionError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for CreateCoreDefinitionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateCoreDefinitionError {
    fn description(&self) -> &str {
        match *self {
            CreateCoreDefinitionError::BadRequest(ref cause) => cause,
            CreateCoreDefinitionError::Validation(ref cause) => cause,
            CreateCoreDefinitionError::Credentials(ref err) => err.description(),
            CreateCoreDefinitionError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            CreateCoreDefinitionError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by CreateCoreDefinitionVersion
#[derive(Debug, PartialEq)]
pub enum CreateCoreDefinitionVersionError {
    ///General Error
    BadRequest(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl CreateCoreDefinitionVersionError {
    pub fn from_body(body: &str) -> CreateCoreDefinitionVersionError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "BadRequestException" => {
                        CreateCoreDefinitionVersionError::BadRequest(String::from(error_message))
                    }
                    "ValidationException" => {
                        CreateCoreDefinitionVersionError::Validation(error_message.to_string())
                    }
                    _ => CreateCoreDefinitionVersionError::Unknown(String::from(body)),
                }
            }
            Err(_) => CreateCoreDefinitionVersionError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for CreateCoreDefinitionVersionError {
    fn from(err: serde_json::error::Error) -> CreateCoreDefinitionVersionError {
        CreateCoreDefinitionVersionError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for CreateCoreDefinitionVersionError {
    fn from(err: CredentialsError) -> CreateCoreDefinitionVersionError {
        CreateCoreDefinitionVersionError::Credentials(err)
    }
}
impl From<HttpDispatchError> for CreateCoreDefinitionVersionError {
    fn from(err: HttpDispatchError) -> CreateCoreDefinitionVersionError {
        CreateCoreDefinitionVersionError::HttpDispatch(err)
    }
}
impl From<io::Error> for CreateCoreDefinitionVersionError {
    fn from(err: io::Error) -> CreateCoreDefinitionVersionError {
        CreateCoreDefinitionVersionError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for CreateCoreDefinitionVersionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateCoreDefinitionVersionError {
    fn description(&self) -> &str {
        match *self {
            CreateCoreDefinitionVersionError::BadRequest(ref cause) => cause,
            CreateCoreDefinitionVersionError::Validation(ref cause) => cause,
            CreateCoreDefinitionVersionError::Credentials(ref err) => err.description(),
            CreateCoreDefinitionVersionError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            CreateCoreDefinitionVersionError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by CreateDeployment
#[derive(Debug, PartialEq)]
pub enum CreateDeploymentError {
    ///General Error
    BadRequest(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl CreateDeploymentError {
    pub fn from_body(body: &str) -> CreateDeploymentError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "BadRequestException" => {
                        CreateDeploymentError::BadRequest(String::from(error_message))
                    }
                    "ValidationException" => {
                        CreateDeploymentError::Validation(error_message.to_string())
                    }
                    _ => CreateDeploymentError::Unknown(String::from(body)),
                }
            }
            Err(_) => CreateDeploymentError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for CreateDeploymentError {
    fn from(err: serde_json::error::Error) -> CreateDeploymentError {
        CreateDeploymentError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for CreateDeploymentError {
    fn from(err: CredentialsError) -> CreateDeploymentError {
        CreateDeploymentError::Credentials(err)
    }
}
impl From<HttpDispatchError> for CreateDeploymentError {
    fn from(err: HttpDispatchError) -> CreateDeploymentError {
        CreateDeploymentError::HttpDispatch(err)
    }
}
impl From<io::Error> for CreateDeploymentError {
    fn from(err: io::Error) -> CreateDeploymentError {
        CreateDeploymentError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for CreateDeploymentError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateDeploymentError {
    fn description(&self) -> &str {
        match *self {
            CreateDeploymentError::BadRequest(ref cause) => cause,
            CreateDeploymentError::Validation(ref cause) => cause,
            CreateDeploymentError::Credentials(ref err) => err.description(),
            CreateDeploymentError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            CreateDeploymentError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by CreateDeviceDefinition
#[derive(Debug, PartialEq)]
pub enum CreateDeviceDefinitionError {
    ///General Error
    BadRequest(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl CreateDeviceDefinitionError {
    pub fn from_body(body: &str) -> CreateDeviceDefinitionError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "BadRequestException" => {
                        CreateDeviceDefinitionError::BadRequest(String::from(error_message))
                    }
                    "ValidationException" => {
                        CreateDeviceDefinitionError::Validation(error_message.to_string())
                    }
                    _ => CreateDeviceDefinitionError::Unknown(String::from(body)),
                }
            }
            Err(_) => CreateDeviceDefinitionError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for CreateDeviceDefinitionError {
    fn from(err: serde_json::error::Error) -> CreateDeviceDefinitionError {
        CreateDeviceDefinitionError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for CreateDeviceDefinitionError {
    fn from(err: CredentialsError) -> CreateDeviceDefinitionError {
        CreateDeviceDefinitionError::Credentials(err)
    }
}
impl From<HttpDispatchError> for CreateDeviceDefinitionError {
    fn from(err: HttpDispatchError) -> CreateDeviceDefinitionError {
        CreateDeviceDefinitionError::HttpDispatch(err)
    }
}
impl From<io::Error> for CreateDeviceDefinitionError {
    fn from(err: io::Error) -> CreateDeviceDefinitionError {
        CreateDeviceDefinitionError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for CreateDeviceDefinitionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateDeviceDefinitionError {
    fn description(&self) -> &str {
        match *self {
            CreateDeviceDefinitionError::BadRequest(ref cause) => cause,
            CreateDeviceDefinitionError::Validation(ref cause) => cause,
            CreateDeviceDefinitionError::Credentials(ref err) => err.description(),
            CreateDeviceDefinitionError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            CreateDeviceDefinitionError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by CreateDeviceDefinitionVersion
#[derive(Debug, PartialEq)]
pub enum CreateDeviceDefinitionVersionError {
    ///General Error
    BadRequest(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl CreateDeviceDefinitionVersionError {
    pub fn from_body(body: &str) -> CreateDeviceDefinitionVersionError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "BadRequestException" => {
                        CreateDeviceDefinitionVersionError::BadRequest(String::from(error_message))
                    }
                    "ValidationException" => {
                        CreateDeviceDefinitionVersionError::Validation(error_message.to_string())
                    }
                    _ => CreateDeviceDefinitionVersionError::Unknown(String::from(body)),
                }
            }
            Err(_) => CreateDeviceDefinitionVersionError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for CreateDeviceDefinitionVersionError {
    fn from(err: serde_json::error::Error) -> CreateDeviceDefinitionVersionError {
        CreateDeviceDefinitionVersionError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for CreateDeviceDefinitionVersionError {
    fn from(err: CredentialsError) -> CreateDeviceDefinitionVersionError {
        CreateDeviceDefinitionVersionError::Credentials(err)
    }
}
impl From<HttpDispatchError> for CreateDeviceDefinitionVersionError {
    fn from(err: HttpDispatchError) -> CreateDeviceDefinitionVersionError {
        CreateDeviceDefinitionVersionError::HttpDispatch(err)
    }
}
impl From<io::Error> for CreateDeviceDefinitionVersionError {
    fn from(err: io::Error) -> CreateDeviceDefinitionVersionError {
        CreateDeviceDefinitionVersionError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for CreateDeviceDefinitionVersionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateDeviceDefinitionVersionError {
    fn description(&self) -> &str {
        match *self {
            CreateDeviceDefinitionVersionError::BadRequest(ref cause) => cause,
            CreateDeviceDefinitionVersionError::Validation(ref cause) => cause,
            CreateDeviceDefinitionVersionError::Credentials(ref err) => err.description(),
            CreateDeviceDefinitionVersionError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            CreateDeviceDefinitionVersionError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by CreateFunctionDefinition
#[derive(Debug, PartialEq)]
pub enum CreateFunctionDefinitionError {
    ///General Error
    BadRequest(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl CreateFunctionDefinitionError {
    pub fn from_body(body: &str) -> CreateFunctionDefinitionError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "BadRequestException" => {
                        CreateFunctionDefinitionError::BadRequest(String::from(error_message))
                    }
                    "ValidationException" => {
                        CreateFunctionDefinitionError::Validation(error_message.to_string())
                    }
                    _ => CreateFunctionDefinitionError::Unknown(String::from(body)),
                }
            }
            Err(_) => CreateFunctionDefinitionError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for CreateFunctionDefinitionError {
    fn from(err: serde_json::error::Error) -> CreateFunctionDefinitionError {
        CreateFunctionDefinitionError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for CreateFunctionDefinitionError {
    fn from(err: CredentialsError) -> CreateFunctionDefinitionError {
        CreateFunctionDefinitionError::Credentials(err)
    }
}
impl From<HttpDispatchError> for CreateFunctionDefinitionError {
    fn from(err: HttpDispatchError) -> CreateFunctionDefinitionError {
        CreateFunctionDefinitionError::HttpDispatch(err)
    }
}
impl From<io::Error> for CreateFunctionDefinitionError {
    fn from(err: io::Error) -> CreateFunctionDefinitionError {
        CreateFunctionDefinitionError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for CreateFunctionDefinitionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateFunctionDefinitionError {
    fn description(&self) -> &str {
        match *self {
            CreateFunctionDefinitionError::BadRequest(ref cause) => cause,
            CreateFunctionDefinitionError::Validation(ref cause) => cause,
            CreateFunctionDefinitionError::Credentials(ref err) => err.description(),
            CreateFunctionDefinitionError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            CreateFunctionDefinitionError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by CreateFunctionDefinitionVersion
#[derive(Debug, PartialEq)]
pub enum CreateFunctionDefinitionVersionError {
    ///General Error
    BadRequest(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl CreateFunctionDefinitionVersionError {
    pub fn from_body(body: &str) -> CreateFunctionDefinitionVersionError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "BadRequestException" => CreateFunctionDefinitionVersionError::BadRequest(
                        String::from(error_message),
                    ),
                    "ValidationException" => {
                        CreateFunctionDefinitionVersionError::Validation(error_message.to_string())
                    }
                    _ => CreateFunctionDefinitionVersionError::Unknown(String::from(body)),
                }
            }
            Err(_) => CreateFunctionDefinitionVersionError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for CreateFunctionDefinitionVersionError {
    fn from(err: serde_json::error::Error) -> CreateFunctionDefinitionVersionError {
        CreateFunctionDefinitionVersionError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for CreateFunctionDefinitionVersionError {
    fn from(err: CredentialsError) -> CreateFunctionDefinitionVersionError {
        CreateFunctionDefinitionVersionError::Credentials(err)
    }
}
impl From<HttpDispatchError> for CreateFunctionDefinitionVersionError {
    fn from(err: HttpDispatchError) -> CreateFunctionDefinitionVersionError {
        CreateFunctionDefinitionVersionError::HttpDispatch(err)
    }
}
impl From<io::Error> for CreateFunctionDefinitionVersionError {
    fn from(err: io::Error) -> CreateFunctionDefinitionVersionError {
        CreateFunctionDefinitionVersionError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for CreateFunctionDefinitionVersionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateFunctionDefinitionVersionError {
    fn description(&self) -> &str {
        match *self {
            CreateFunctionDefinitionVersionError::BadRequest(ref cause) => cause,
            CreateFunctionDefinitionVersionError::Validation(ref cause) => cause,
            CreateFunctionDefinitionVersionError::Credentials(ref err) => err.description(),
            CreateFunctionDefinitionVersionError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            CreateFunctionDefinitionVersionError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by CreateGroup
#[derive(Debug, PartialEq)]
pub enum CreateGroupError {
    ///General Error
    BadRequest(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl CreateGroupError {
    pub fn from_body(body: &str) -> CreateGroupError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "BadRequestException" => {
                        CreateGroupError::BadRequest(String::from(error_message))
                    }
                    "ValidationException" => {
                        CreateGroupError::Validation(error_message.to_string())
                    }
                    _ => CreateGroupError::Unknown(String::from(body)),
                }
            }
            Err(_) => CreateGroupError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for CreateGroupError {
    fn from(err: serde_json::error::Error) -> CreateGroupError {
        CreateGroupError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for CreateGroupError {
    fn from(err: CredentialsError) -> CreateGroupError {
        CreateGroupError::Credentials(err)
    }
}
impl From<HttpDispatchError> for CreateGroupError {
    fn from(err: HttpDispatchError) -> CreateGroupError {
        CreateGroupError::HttpDispatch(err)
    }
}
impl From<io::Error> for CreateGroupError {
    fn from(err: io::Error) -> CreateGroupError {
        CreateGroupError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for CreateGroupError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateGroupError {
    fn description(&self) -> &str {
        match *self {
            CreateGroupError::BadRequest(ref cause) => cause,
            CreateGroupError::Validation(ref cause) => cause,
            CreateGroupError::Credentials(ref err) => err.description(),
            CreateGroupError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            CreateGroupError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by CreateGroupCertificateAuthority
#[derive(Debug, PartialEq)]
pub enum CreateGroupCertificateAuthorityError {
    ///General Error
    InternalServerError(String),
    ///General Error
    BadRequest(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl CreateGroupCertificateAuthorityError {
    pub fn from_body(body: &str) -> CreateGroupCertificateAuthorityError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InternalServerErrorException" => {
                        CreateGroupCertificateAuthorityError::InternalServerError(String::from(
                            error_message,
                        ))
                    }
                    "BadRequestException" => CreateGroupCertificateAuthorityError::BadRequest(
                        String::from(error_message),
                    ),
                    "ValidationException" => {
                        CreateGroupCertificateAuthorityError::Validation(error_message.to_string())
                    }
                    _ => CreateGroupCertificateAuthorityError::Unknown(String::from(body)),
                }
            }
            Err(_) => CreateGroupCertificateAuthorityError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for CreateGroupCertificateAuthorityError {
    fn from(err: serde_json::error::Error) -> CreateGroupCertificateAuthorityError {
        CreateGroupCertificateAuthorityError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for CreateGroupCertificateAuthorityError {
    fn from(err: CredentialsError) -> CreateGroupCertificateAuthorityError {
        CreateGroupCertificateAuthorityError::Credentials(err)
    }
}
impl From<HttpDispatchError> for CreateGroupCertificateAuthorityError {
    fn from(err: HttpDispatchError) -> CreateGroupCertificateAuthorityError {
        CreateGroupCertificateAuthorityError::HttpDispatch(err)
    }
}
impl From<io::Error> for CreateGroupCertificateAuthorityError {
    fn from(err: io::Error) -> CreateGroupCertificateAuthorityError {
        CreateGroupCertificateAuthorityError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for CreateGroupCertificateAuthorityError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateGroupCertificateAuthorityError {
    fn description(&self) -> &str {
        match *self {
            CreateGroupCertificateAuthorityError::InternalServerError(ref cause) => cause,
            CreateGroupCertificateAuthorityError::BadRequest(ref cause) => cause,
            CreateGroupCertificateAuthorityError::Validation(ref cause) => cause,
            CreateGroupCertificateAuthorityError::Credentials(ref err) => err.description(),
            CreateGroupCertificateAuthorityError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            CreateGroupCertificateAuthorityError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by CreateGroupVersion
#[derive(Debug, PartialEq)]
pub enum CreateGroupVersionError {
    ///General Error
    BadRequest(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl CreateGroupVersionError {
    pub fn from_body(body: &str) -> CreateGroupVersionError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "BadRequestException" => {
                        CreateGroupVersionError::BadRequest(String::from(error_message))
                    }
                    "ValidationException" => {
                        CreateGroupVersionError::Validation(error_message.to_string())
                    }
                    _ => CreateGroupVersionError::Unknown(String::from(body)),
                }
            }
            Err(_) => CreateGroupVersionError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for CreateGroupVersionError {
    fn from(err: serde_json::error::Error) -> CreateGroupVersionError {
        CreateGroupVersionError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for CreateGroupVersionError {
    fn from(err: CredentialsError) -> CreateGroupVersionError {
        CreateGroupVersionError::Credentials(err)
    }
}
impl From<HttpDispatchError> for CreateGroupVersionError {
    fn from(err: HttpDispatchError) -> CreateGroupVersionError {
        CreateGroupVersionError::HttpDispatch(err)
    }
}
impl From<io::Error> for CreateGroupVersionError {
    fn from(err: io::Error) -> CreateGroupVersionError {
        CreateGroupVersionError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for CreateGroupVersionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateGroupVersionError {
    fn description(&self) -> &str {
        match *self {
            CreateGroupVersionError::BadRequest(ref cause) => cause,
            CreateGroupVersionError::Validation(ref cause) => cause,
            CreateGroupVersionError::Credentials(ref err) => err.description(),
            CreateGroupVersionError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            CreateGroupVersionError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by CreateLoggerDefinition
#[derive(Debug, PartialEq)]
pub enum CreateLoggerDefinitionError {
    ///General Error
    BadRequest(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl CreateLoggerDefinitionError {
    pub fn from_body(body: &str) -> CreateLoggerDefinitionError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "BadRequestException" => {
                        CreateLoggerDefinitionError::BadRequest(String::from(error_message))
                    }
                    "ValidationException" => {
                        CreateLoggerDefinitionError::Validation(error_message.to_string())
                    }
                    _ => CreateLoggerDefinitionError::Unknown(String::from(body)),
                }
            }
            Err(_) => CreateLoggerDefinitionError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for CreateLoggerDefinitionError {
    fn from(err: serde_json::error::Error) -> CreateLoggerDefinitionError {
        CreateLoggerDefinitionError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for CreateLoggerDefinitionError {
    fn from(err: CredentialsError) -> CreateLoggerDefinitionError {
        CreateLoggerDefinitionError::Credentials(err)
    }
}
impl From<HttpDispatchError> for CreateLoggerDefinitionError {
    fn from(err: HttpDispatchError) -> CreateLoggerDefinitionError {
        CreateLoggerDefinitionError::HttpDispatch(err)
    }
}
impl From<io::Error> for CreateLoggerDefinitionError {
    fn from(err: io::Error) -> CreateLoggerDefinitionError {
        CreateLoggerDefinitionError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for CreateLoggerDefinitionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateLoggerDefinitionError {
    fn description(&self) -> &str {
        match *self {
            CreateLoggerDefinitionError::BadRequest(ref cause) => cause,
            CreateLoggerDefinitionError::Validation(ref cause) => cause,
            CreateLoggerDefinitionError::Credentials(ref err) => err.description(),
            CreateLoggerDefinitionError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            CreateLoggerDefinitionError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by CreateLoggerDefinitionVersion
#[derive(Debug, PartialEq)]
pub enum CreateLoggerDefinitionVersionError {
    ///General Error
    BadRequest(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl CreateLoggerDefinitionVersionError {
    pub fn from_body(body: &str) -> CreateLoggerDefinitionVersionError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "BadRequestException" => {
                        CreateLoggerDefinitionVersionError::BadRequest(String::from(error_message))
                    }
                    "ValidationException" => {
                        CreateLoggerDefinitionVersionError::Validation(error_message.to_string())
                    }
                    _ => CreateLoggerDefinitionVersionError::Unknown(String::from(body)),
                }
            }
            Err(_) => CreateLoggerDefinitionVersionError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for CreateLoggerDefinitionVersionError {
    fn from(err: serde_json::error::Error) -> CreateLoggerDefinitionVersionError {
        CreateLoggerDefinitionVersionError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for CreateLoggerDefinitionVersionError {
    fn from(err: CredentialsError) -> CreateLoggerDefinitionVersionError {
        CreateLoggerDefinitionVersionError::Credentials(err)
    }
}
impl From<HttpDispatchError> for CreateLoggerDefinitionVersionError {
    fn from(err: HttpDispatchError) -> CreateLoggerDefinitionVersionError {
        CreateLoggerDefinitionVersionError::HttpDispatch(err)
    }
}
impl From<io::Error> for CreateLoggerDefinitionVersionError {
    fn from(err: io::Error) -> CreateLoggerDefinitionVersionError {
        CreateLoggerDefinitionVersionError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for CreateLoggerDefinitionVersionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateLoggerDefinitionVersionError {
    fn description(&self) -> &str {
        match *self {
            CreateLoggerDefinitionVersionError::BadRequest(ref cause) => cause,
            CreateLoggerDefinitionVersionError::Validation(ref cause) => cause,
            CreateLoggerDefinitionVersionError::Credentials(ref err) => err.description(),
            CreateLoggerDefinitionVersionError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            CreateLoggerDefinitionVersionError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by CreateSubscriptionDefinition
#[derive(Debug, PartialEq)]
pub enum CreateSubscriptionDefinitionError {
    ///General Error
    BadRequest(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl CreateSubscriptionDefinitionError {
    pub fn from_body(body: &str) -> CreateSubscriptionDefinitionError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "BadRequestException" => {
                        CreateSubscriptionDefinitionError::BadRequest(String::from(error_message))
                    }
                    "ValidationException" => {
                        CreateSubscriptionDefinitionError::Validation(error_message.to_string())
                    }
                    _ => CreateSubscriptionDefinitionError::Unknown(String::from(body)),
                }
            }
            Err(_) => CreateSubscriptionDefinitionError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for CreateSubscriptionDefinitionError {
    fn from(err: serde_json::error::Error) -> CreateSubscriptionDefinitionError {
        CreateSubscriptionDefinitionError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for CreateSubscriptionDefinitionError {
    fn from(err: CredentialsError) -> CreateSubscriptionDefinitionError {
        CreateSubscriptionDefinitionError::Credentials(err)
    }
}
impl From<HttpDispatchError> for CreateSubscriptionDefinitionError {
    fn from(err: HttpDispatchError) -> CreateSubscriptionDefinitionError {
        CreateSubscriptionDefinitionError::HttpDispatch(err)
    }
}
impl From<io::Error> for CreateSubscriptionDefinitionError {
    fn from(err: io::Error) -> CreateSubscriptionDefinitionError {
        CreateSubscriptionDefinitionError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for CreateSubscriptionDefinitionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateSubscriptionDefinitionError {
    fn description(&self) -> &str {
        match *self {
            CreateSubscriptionDefinitionError::BadRequest(ref cause) => cause,
            CreateSubscriptionDefinitionError::Validation(ref cause) => cause,
            CreateSubscriptionDefinitionError::Credentials(ref err) => err.description(),
            CreateSubscriptionDefinitionError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            CreateSubscriptionDefinitionError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by CreateSubscriptionDefinitionVersion
#[derive(Debug, PartialEq)]
pub enum CreateSubscriptionDefinitionVersionError {
    ///General Error
    BadRequest(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl CreateSubscriptionDefinitionVersionError {
    pub fn from_body(body: &str) -> CreateSubscriptionDefinitionVersionError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "BadRequestException" => CreateSubscriptionDefinitionVersionError::BadRequest(
                        String::from(error_message),
                    ),
                    "ValidationException" => CreateSubscriptionDefinitionVersionError::Validation(
                        error_message.to_string(),
                    ),
                    _ => CreateSubscriptionDefinitionVersionError::Unknown(String::from(body)),
                }
            }
            Err(_) => CreateSubscriptionDefinitionVersionError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for CreateSubscriptionDefinitionVersionError {
    fn from(err: serde_json::error::Error) -> CreateSubscriptionDefinitionVersionError {
        CreateSubscriptionDefinitionVersionError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for CreateSubscriptionDefinitionVersionError {
    fn from(err: CredentialsError) -> CreateSubscriptionDefinitionVersionError {
        CreateSubscriptionDefinitionVersionError::Credentials(err)
    }
}
impl From<HttpDispatchError> for CreateSubscriptionDefinitionVersionError {
    fn from(err: HttpDispatchError) -> CreateSubscriptionDefinitionVersionError {
        CreateSubscriptionDefinitionVersionError::HttpDispatch(err)
    }
}
impl From<io::Error> for CreateSubscriptionDefinitionVersionError {
    fn from(err: io::Error) -> CreateSubscriptionDefinitionVersionError {
        CreateSubscriptionDefinitionVersionError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for CreateSubscriptionDefinitionVersionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateSubscriptionDefinitionVersionError {
    fn description(&self) -> &str {
        match *self {
            CreateSubscriptionDefinitionVersionError::BadRequest(ref cause) => cause,
            CreateSubscriptionDefinitionVersionError::Validation(ref cause) => cause,
            CreateSubscriptionDefinitionVersionError::Credentials(ref err) => err.description(),
            CreateSubscriptionDefinitionVersionError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            CreateSubscriptionDefinitionVersionError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DeleteCoreDefinition
#[derive(Debug, PartialEq)]
pub enum DeleteCoreDefinitionError {
    ///General Error
    BadRequest(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl DeleteCoreDefinitionError {
    pub fn from_body(body: &str) -> DeleteCoreDefinitionError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "BadRequestException" => {
                        DeleteCoreDefinitionError::BadRequest(String::from(error_message))
                    }
                    "ValidationException" => {
                        DeleteCoreDefinitionError::Validation(error_message.to_string())
                    }
                    _ => DeleteCoreDefinitionError::Unknown(String::from(body)),
                }
            }
            Err(_) => DeleteCoreDefinitionError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DeleteCoreDefinitionError {
    fn from(err: serde_json::error::Error) -> DeleteCoreDefinitionError {
        DeleteCoreDefinitionError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DeleteCoreDefinitionError {
    fn from(err: CredentialsError) -> DeleteCoreDefinitionError {
        DeleteCoreDefinitionError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DeleteCoreDefinitionError {
    fn from(err: HttpDispatchError) -> DeleteCoreDefinitionError {
        DeleteCoreDefinitionError::HttpDispatch(err)
    }
}
impl From<io::Error> for DeleteCoreDefinitionError {
    fn from(err: io::Error) -> DeleteCoreDefinitionError {
        DeleteCoreDefinitionError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DeleteCoreDefinitionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteCoreDefinitionError {
    fn description(&self) -> &str {
        match *self {
            DeleteCoreDefinitionError::BadRequest(ref cause) => cause,
            DeleteCoreDefinitionError::Validation(ref cause) => cause,
            DeleteCoreDefinitionError::Credentials(ref err) => err.description(),
            DeleteCoreDefinitionError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DeleteCoreDefinitionError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DeleteDeviceDefinition
#[derive(Debug, PartialEq)]
pub enum DeleteDeviceDefinitionError {
    ///General Error
    BadRequest(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl DeleteDeviceDefinitionError {
    pub fn from_body(body: &str) -> DeleteDeviceDefinitionError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "BadRequestException" => {
                        DeleteDeviceDefinitionError::BadRequest(String::from(error_message))
                    }
                    "ValidationException" => {
                        DeleteDeviceDefinitionError::Validation(error_message.to_string())
                    }
                    _ => DeleteDeviceDefinitionError::Unknown(String::from(body)),
                }
            }
            Err(_) => DeleteDeviceDefinitionError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DeleteDeviceDefinitionError {
    fn from(err: serde_json::error::Error) -> DeleteDeviceDefinitionError {
        DeleteDeviceDefinitionError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DeleteDeviceDefinitionError {
    fn from(err: CredentialsError) -> DeleteDeviceDefinitionError {
        DeleteDeviceDefinitionError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DeleteDeviceDefinitionError {
    fn from(err: HttpDispatchError) -> DeleteDeviceDefinitionError {
        DeleteDeviceDefinitionError::HttpDispatch(err)
    }
}
impl From<io::Error> for DeleteDeviceDefinitionError {
    fn from(err: io::Error) -> DeleteDeviceDefinitionError {
        DeleteDeviceDefinitionError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DeleteDeviceDefinitionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteDeviceDefinitionError {
    fn description(&self) -> &str {
        match *self {
            DeleteDeviceDefinitionError::BadRequest(ref cause) => cause,
            DeleteDeviceDefinitionError::Validation(ref cause) => cause,
            DeleteDeviceDefinitionError::Credentials(ref err) => err.description(),
            DeleteDeviceDefinitionError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DeleteDeviceDefinitionError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DeleteFunctionDefinition
#[derive(Debug, PartialEq)]
pub enum DeleteFunctionDefinitionError {
    ///General Error
    BadRequest(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl DeleteFunctionDefinitionError {
    pub fn from_body(body: &str) -> DeleteFunctionDefinitionError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "BadRequestException" => {
                        DeleteFunctionDefinitionError::BadRequest(String::from(error_message))
                    }
                    "ValidationException" => {
                        DeleteFunctionDefinitionError::Validation(error_message.to_string())
                    }
                    _ => DeleteFunctionDefinitionError::Unknown(String::from(body)),
                }
            }
            Err(_) => DeleteFunctionDefinitionError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DeleteFunctionDefinitionError {
    fn from(err: serde_json::error::Error) -> DeleteFunctionDefinitionError {
        DeleteFunctionDefinitionError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DeleteFunctionDefinitionError {
    fn from(err: CredentialsError) -> DeleteFunctionDefinitionError {
        DeleteFunctionDefinitionError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DeleteFunctionDefinitionError {
    fn from(err: HttpDispatchError) -> DeleteFunctionDefinitionError {
        DeleteFunctionDefinitionError::HttpDispatch(err)
    }
}
impl From<io::Error> for DeleteFunctionDefinitionError {
    fn from(err: io::Error) -> DeleteFunctionDefinitionError {
        DeleteFunctionDefinitionError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DeleteFunctionDefinitionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteFunctionDefinitionError {
    fn description(&self) -> &str {
        match *self {
            DeleteFunctionDefinitionError::BadRequest(ref cause) => cause,
            DeleteFunctionDefinitionError::Validation(ref cause) => cause,
            DeleteFunctionDefinitionError::Credentials(ref err) => err.description(),
            DeleteFunctionDefinitionError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DeleteFunctionDefinitionError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DeleteGroup
#[derive(Debug, PartialEq)]
pub enum DeleteGroupError {
    ///General Error
    BadRequest(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl DeleteGroupError {
    pub fn from_body(body: &str) -> DeleteGroupError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "BadRequestException" => {
                        DeleteGroupError::BadRequest(String::from(error_message))
                    }
                    "ValidationException" => {
                        DeleteGroupError::Validation(error_message.to_string())
                    }
                    _ => DeleteGroupError::Unknown(String::from(body)),
                }
            }
            Err(_) => DeleteGroupError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DeleteGroupError {
    fn from(err: serde_json::error::Error) -> DeleteGroupError {
        DeleteGroupError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DeleteGroupError {
    fn from(err: CredentialsError) -> DeleteGroupError {
        DeleteGroupError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DeleteGroupError {
    fn from(err: HttpDispatchError) -> DeleteGroupError {
        DeleteGroupError::HttpDispatch(err)
    }
}
impl From<io::Error> for DeleteGroupError {
    fn from(err: io::Error) -> DeleteGroupError {
        DeleteGroupError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DeleteGroupError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteGroupError {
    fn description(&self) -> &str {
        match *self {
            DeleteGroupError::BadRequest(ref cause) => cause,
            DeleteGroupError::Validation(ref cause) => cause,
            DeleteGroupError::Credentials(ref err) => err.description(),
            DeleteGroupError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            DeleteGroupError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DeleteLoggerDefinition
#[derive(Debug, PartialEq)]
pub enum DeleteLoggerDefinitionError {
    ///General Error
    BadRequest(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl DeleteLoggerDefinitionError {
    pub fn from_body(body: &str) -> DeleteLoggerDefinitionError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "BadRequestException" => {
                        DeleteLoggerDefinitionError::BadRequest(String::from(error_message))
                    }
                    "ValidationException" => {
                        DeleteLoggerDefinitionError::Validation(error_message.to_string())
                    }
                    _ => DeleteLoggerDefinitionError::Unknown(String::from(body)),
                }
            }
            Err(_) => DeleteLoggerDefinitionError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DeleteLoggerDefinitionError {
    fn from(err: serde_json::error::Error) -> DeleteLoggerDefinitionError {
        DeleteLoggerDefinitionError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DeleteLoggerDefinitionError {
    fn from(err: CredentialsError) -> DeleteLoggerDefinitionError {
        DeleteLoggerDefinitionError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DeleteLoggerDefinitionError {
    fn from(err: HttpDispatchError) -> DeleteLoggerDefinitionError {
        DeleteLoggerDefinitionError::HttpDispatch(err)
    }
}
impl From<io::Error> for DeleteLoggerDefinitionError {
    fn from(err: io::Error) -> DeleteLoggerDefinitionError {
        DeleteLoggerDefinitionError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DeleteLoggerDefinitionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteLoggerDefinitionError {
    fn description(&self) -> &str {
        match *self {
            DeleteLoggerDefinitionError::BadRequest(ref cause) => cause,
            DeleteLoggerDefinitionError::Validation(ref cause) => cause,
            DeleteLoggerDefinitionError::Credentials(ref err) => err.description(),
            DeleteLoggerDefinitionError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DeleteLoggerDefinitionError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DeleteSubscriptionDefinition
#[derive(Debug, PartialEq)]
pub enum DeleteSubscriptionDefinitionError {
    ///General Error
    BadRequest(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl DeleteSubscriptionDefinitionError {
    pub fn from_body(body: &str) -> DeleteSubscriptionDefinitionError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "BadRequestException" => {
                        DeleteSubscriptionDefinitionError::BadRequest(String::from(error_message))
                    }
                    "ValidationException" => {
                        DeleteSubscriptionDefinitionError::Validation(error_message.to_string())
                    }
                    _ => DeleteSubscriptionDefinitionError::Unknown(String::from(body)),
                }
            }
            Err(_) => DeleteSubscriptionDefinitionError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DeleteSubscriptionDefinitionError {
    fn from(err: serde_json::error::Error) -> DeleteSubscriptionDefinitionError {
        DeleteSubscriptionDefinitionError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DeleteSubscriptionDefinitionError {
    fn from(err: CredentialsError) -> DeleteSubscriptionDefinitionError {
        DeleteSubscriptionDefinitionError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DeleteSubscriptionDefinitionError {
    fn from(err: HttpDispatchError) -> DeleteSubscriptionDefinitionError {
        DeleteSubscriptionDefinitionError::HttpDispatch(err)
    }
}
impl From<io::Error> for DeleteSubscriptionDefinitionError {
    fn from(err: io::Error) -> DeleteSubscriptionDefinitionError {
        DeleteSubscriptionDefinitionError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DeleteSubscriptionDefinitionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteSubscriptionDefinitionError {
    fn description(&self) -> &str {
        match *self {
            DeleteSubscriptionDefinitionError::BadRequest(ref cause) => cause,
            DeleteSubscriptionDefinitionError::Validation(ref cause) => cause,
            DeleteSubscriptionDefinitionError::Credentials(ref err) => err.description(),
            DeleteSubscriptionDefinitionError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DeleteSubscriptionDefinitionError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DisassociateRoleFromGroup
#[derive(Debug, PartialEq)]
pub enum DisassociateRoleFromGroupError {
    ///General Error
    InternalServerError(String),
    ///General Error
    BadRequest(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl DisassociateRoleFromGroupError {
    pub fn from_body(body: &str) -> DisassociateRoleFromGroupError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InternalServerErrorException" => {
                        DisassociateRoleFromGroupError::InternalServerError(String::from(
                            error_message,
                        ))
                    }
                    "BadRequestException" => {
                        DisassociateRoleFromGroupError::BadRequest(String::from(error_message))
                    }
                    "ValidationException" => {
                        DisassociateRoleFromGroupError::Validation(error_message.to_string())
                    }
                    _ => DisassociateRoleFromGroupError::Unknown(String::from(body)),
                }
            }
            Err(_) => DisassociateRoleFromGroupError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DisassociateRoleFromGroupError {
    fn from(err: serde_json::error::Error) -> DisassociateRoleFromGroupError {
        DisassociateRoleFromGroupError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DisassociateRoleFromGroupError {
    fn from(err: CredentialsError) -> DisassociateRoleFromGroupError {
        DisassociateRoleFromGroupError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DisassociateRoleFromGroupError {
    fn from(err: HttpDispatchError) -> DisassociateRoleFromGroupError {
        DisassociateRoleFromGroupError::HttpDispatch(err)
    }
}
impl From<io::Error> for DisassociateRoleFromGroupError {
    fn from(err: io::Error) -> DisassociateRoleFromGroupError {
        DisassociateRoleFromGroupError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DisassociateRoleFromGroupError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DisassociateRoleFromGroupError {
    fn description(&self) -> &str {
        match *self {
            DisassociateRoleFromGroupError::InternalServerError(ref cause) => cause,
            DisassociateRoleFromGroupError::BadRequest(ref cause) => cause,
            DisassociateRoleFromGroupError::Validation(ref cause) => cause,
            DisassociateRoleFromGroupError::Credentials(ref err) => err.description(),
            DisassociateRoleFromGroupError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DisassociateRoleFromGroupError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DisassociateServiceRoleFromAccount
#[derive(Debug, PartialEq)]
pub enum DisassociateServiceRoleFromAccountError {
    ///General Error
    InternalServerError(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl DisassociateServiceRoleFromAccountError {
    pub fn from_body(body: &str) -> DisassociateServiceRoleFromAccountError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InternalServerErrorException" => {
                        DisassociateServiceRoleFromAccountError::InternalServerError(String::from(
                            error_message,
                        ))
                    }
                    "ValidationException" => DisassociateServiceRoleFromAccountError::Validation(
                        error_message.to_string(),
                    ),
                    _ => DisassociateServiceRoleFromAccountError::Unknown(String::from(body)),
                }
            }
            Err(_) => DisassociateServiceRoleFromAccountError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DisassociateServiceRoleFromAccountError {
    fn from(err: serde_json::error::Error) -> DisassociateServiceRoleFromAccountError {
        DisassociateServiceRoleFromAccountError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DisassociateServiceRoleFromAccountError {
    fn from(err: CredentialsError) -> DisassociateServiceRoleFromAccountError {
        DisassociateServiceRoleFromAccountError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DisassociateServiceRoleFromAccountError {
    fn from(err: HttpDispatchError) -> DisassociateServiceRoleFromAccountError {
        DisassociateServiceRoleFromAccountError::HttpDispatch(err)
    }
}
impl From<io::Error> for DisassociateServiceRoleFromAccountError {
    fn from(err: io::Error) -> DisassociateServiceRoleFromAccountError {
        DisassociateServiceRoleFromAccountError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DisassociateServiceRoleFromAccountError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DisassociateServiceRoleFromAccountError {
    fn description(&self) -> &str {
        match *self {
            DisassociateServiceRoleFromAccountError::InternalServerError(ref cause) => cause,
            DisassociateServiceRoleFromAccountError::Validation(ref cause) => cause,
            DisassociateServiceRoleFromAccountError::Credentials(ref err) => err.description(),
            DisassociateServiceRoleFromAccountError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DisassociateServiceRoleFromAccountError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by GetAssociatedRole
#[derive(Debug, PartialEq)]
pub enum GetAssociatedRoleError {
    ///General Error
    InternalServerError(String),
    ///General Error
    BadRequest(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl GetAssociatedRoleError {
    pub fn from_body(body: &str) -> GetAssociatedRoleError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InternalServerErrorException" => {
                        GetAssociatedRoleError::InternalServerError(String::from(error_message))
                    }
                    "BadRequestException" => {
                        GetAssociatedRoleError::BadRequest(String::from(error_message))
                    }
                    "ValidationException" => {
                        GetAssociatedRoleError::Validation(error_message.to_string())
                    }
                    _ => GetAssociatedRoleError::Unknown(String::from(body)),
                }
            }
            Err(_) => GetAssociatedRoleError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for GetAssociatedRoleError {
    fn from(err: serde_json::error::Error) -> GetAssociatedRoleError {
        GetAssociatedRoleError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for GetAssociatedRoleError {
    fn from(err: CredentialsError) -> GetAssociatedRoleError {
        GetAssociatedRoleError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GetAssociatedRoleError {
    fn from(err: HttpDispatchError) -> GetAssociatedRoleError {
        GetAssociatedRoleError::HttpDispatch(err)
    }
}
impl From<io::Error> for GetAssociatedRoleError {
    fn from(err: io::Error) -> GetAssociatedRoleError {
        GetAssociatedRoleError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for GetAssociatedRoleError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetAssociatedRoleError {
    fn description(&self) -> &str {
        match *self {
            GetAssociatedRoleError::InternalServerError(ref cause) => cause,
            GetAssociatedRoleError::BadRequest(ref cause) => cause,
            GetAssociatedRoleError::Validation(ref cause) => cause,
            GetAssociatedRoleError::Credentials(ref err) => err.description(),
            GetAssociatedRoleError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            GetAssociatedRoleError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by GetConnectivityInfo
#[derive(Debug, PartialEq)]
pub enum GetConnectivityInfoError {
    ///General Error
    BadRequest(String),
    ///General Error
    InternalServerError(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl GetConnectivityInfoError {
    pub fn from_body(body: &str) -> GetConnectivityInfoError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "BadRequestException" => {
                        GetConnectivityInfoError::BadRequest(String::from(error_message))
                    }
                    "InternalServerErrorException" => {
                        GetConnectivityInfoError::InternalServerError(String::from(error_message))
                    }
                    "ValidationException" => {
                        GetConnectivityInfoError::Validation(error_message.to_string())
                    }
                    _ => GetConnectivityInfoError::Unknown(String::from(body)),
                }
            }
            Err(_) => GetConnectivityInfoError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for GetConnectivityInfoError {
    fn from(err: serde_json::error::Error) -> GetConnectivityInfoError {
        GetConnectivityInfoError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for GetConnectivityInfoError {
    fn from(err: CredentialsError) -> GetConnectivityInfoError {
        GetConnectivityInfoError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GetConnectivityInfoError {
    fn from(err: HttpDispatchError) -> GetConnectivityInfoError {
        GetConnectivityInfoError::HttpDispatch(err)
    }
}
impl From<io::Error> for GetConnectivityInfoError {
    fn from(err: io::Error) -> GetConnectivityInfoError {
        GetConnectivityInfoError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for GetConnectivityInfoError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetConnectivityInfoError {
    fn description(&self) -> &str {
        match *self {
            GetConnectivityInfoError::BadRequest(ref cause) => cause,
            GetConnectivityInfoError::InternalServerError(ref cause) => cause,
            GetConnectivityInfoError::Validation(ref cause) => cause,
            GetConnectivityInfoError::Credentials(ref err) => err.description(),
            GetConnectivityInfoError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            GetConnectivityInfoError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by GetCoreDefinition
#[derive(Debug, PartialEq)]
pub enum GetCoreDefinitionError {
    ///General Error
    BadRequest(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl GetCoreDefinitionError {
    pub fn from_body(body: &str) -> GetCoreDefinitionError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "BadRequestException" => {
                        GetCoreDefinitionError::BadRequest(String::from(error_message))
                    }
                    "ValidationException" => {
                        GetCoreDefinitionError::Validation(error_message.to_string())
                    }
                    _ => GetCoreDefinitionError::Unknown(String::from(body)),
                }
            }
            Err(_) => GetCoreDefinitionError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for GetCoreDefinitionError {
    fn from(err: serde_json::error::Error) -> GetCoreDefinitionError {
        GetCoreDefinitionError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for GetCoreDefinitionError {
    fn from(err: CredentialsError) -> GetCoreDefinitionError {
        GetCoreDefinitionError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GetCoreDefinitionError {
    fn from(err: HttpDispatchError) -> GetCoreDefinitionError {
        GetCoreDefinitionError::HttpDispatch(err)
    }
}
impl From<io::Error> for GetCoreDefinitionError {
    fn from(err: io::Error) -> GetCoreDefinitionError {
        GetCoreDefinitionError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for GetCoreDefinitionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetCoreDefinitionError {
    fn description(&self) -> &str {
        match *self {
            GetCoreDefinitionError::BadRequest(ref cause) => cause,
            GetCoreDefinitionError::Validation(ref cause) => cause,
            GetCoreDefinitionError::Credentials(ref err) => err.description(),
            GetCoreDefinitionError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            GetCoreDefinitionError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by GetCoreDefinitionVersion
#[derive(Debug, PartialEq)]
pub enum GetCoreDefinitionVersionError {
    ///General Error
    BadRequest(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl GetCoreDefinitionVersionError {
    pub fn from_body(body: &str) -> GetCoreDefinitionVersionError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "BadRequestException" => {
                        GetCoreDefinitionVersionError::BadRequest(String::from(error_message))
                    }
                    "ValidationException" => {
                        GetCoreDefinitionVersionError::Validation(error_message.to_string())
                    }
                    _ => GetCoreDefinitionVersionError::Unknown(String::from(body)),
                }
            }
            Err(_) => GetCoreDefinitionVersionError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for GetCoreDefinitionVersionError {
    fn from(err: serde_json::error::Error) -> GetCoreDefinitionVersionError {
        GetCoreDefinitionVersionError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for GetCoreDefinitionVersionError {
    fn from(err: CredentialsError) -> GetCoreDefinitionVersionError {
        GetCoreDefinitionVersionError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GetCoreDefinitionVersionError {
    fn from(err: HttpDispatchError) -> GetCoreDefinitionVersionError {
        GetCoreDefinitionVersionError::HttpDispatch(err)
    }
}
impl From<io::Error> for GetCoreDefinitionVersionError {
    fn from(err: io::Error) -> GetCoreDefinitionVersionError {
        GetCoreDefinitionVersionError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for GetCoreDefinitionVersionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetCoreDefinitionVersionError {
    fn description(&self) -> &str {
        match *self {
            GetCoreDefinitionVersionError::BadRequest(ref cause) => cause,
            GetCoreDefinitionVersionError::Validation(ref cause) => cause,
            GetCoreDefinitionVersionError::Credentials(ref err) => err.description(),
            GetCoreDefinitionVersionError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            GetCoreDefinitionVersionError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by GetDeploymentStatus
#[derive(Debug, PartialEq)]
pub enum GetDeploymentStatusError {
    ///General Error
    BadRequest(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl GetDeploymentStatusError {
    pub fn from_body(body: &str) -> GetDeploymentStatusError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "BadRequestException" => {
                        GetDeploymentStatusError::BadRequest(String::from(error_message))
                    }
                    "ValidationException" => {
                        GetDeploymentStatusError::Validation(error_message.to_string())
                    }
                    _ => GetDeploymentStatusError::Unknown(String::from(body)),
                }
            }
            Err(_) => GetDeploymentStatusError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for GetDeploymentStatusError {
    fn from(err: serde_json::error::Error) -> GetDeploymentStatusError {
        GetDeploymentStatusError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for GetDeploymentStatusError {
    fn from(err: CredentialsError) -> GetDeploymentStatusError {
        GetDeploymentStatusError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GetDeploymentStatusError {
    fn from(err: HttpDispatchError) -> GetDeploymentStatusError {
        GetDeploymentStatusError::HttpDispatch(err)
    }
}
impl From<io::Error> for GetDeploymentStatusError {
    fn from(err: io::Error) -> GetDeploymentStatusError {
        GetDeploymentStatusError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for GetDeploymentStatusError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetDeploymentStatusError {
    fn description(&self) -> &str {
        match *self {
            GetDeploymentStatusError::BadRequest(ref cause) => cause,
            GetDeploymentStatusError::Validation(ref cause) => cause,
            GetDeploymentStatusError::Credentials(ref err) => err.description(),
            GetDeploymentStatusError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            GetDeploymentStatusError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by GetDeviceDefinition
#[derive(Debug, PartialEq)]
pub enum GetDeviceDefinitionError {
    ///General Error
    BadRequest(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl GetDeviceDefinitionError {
    pub fn from_body(body: &str) -> GetDeviceDefinitionError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "BadRequestException" => {
                        GetDeviceDefinitionError::BadRequest(String::from(error_message))
                    }
                    "ValidationException" => {
                        GetDeviceDefinitionError::Validation(error_message.to_string())
                    }
                    _ => GetDeviceDefinitionError::Unknown(String::from(body)),
                }
            }
            Err(_) => GetDeviceDefinitionError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for GetDeviceDefinitionError {
    fn from(err: serde_json::error::Error) -> GetDeviceDefinitionError {
        GetDeviceDefinitionError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for GetDeviceDefinitionError {
    fn from(err: CredentialsError) -> GetDeviceDefinitionError {
        GetDeviceDefinitionError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GetDeviceDefinitionError {
    fn from(err: HttpDispatchError) -> GetDeviceDefinitionError {
        GetDeviceDefinitionError::HttpDispatch(err)
    }
}
impl From<io::Error> for GetDeviceDefinitionError {
    fn from(err: io::Error) -> GetDeviceDefinitionError {
        GetDeviceDefinitionError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for GetDeviceDefinitionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetDeviceDefinitionError {
    fn description(&self) -> &str {
        match *self {
            GetDeviceDefinitionError::BadRequest(ref cause) => cause,
            GetDeviceDefinitionError::Validation(ref cause) => cause,
            GetDeviceDefinitionError::Credentials(ref err) => err.description(),
            GetDeviceDefinitionError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            GetDeviceDefinitionError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by GetDeviceDefinitionVersion
#[derive(Debug, PartialEq)]
pub enum GetDeviceDefinitionVersionError {
    ///General Error
    BadRequest(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl GetDeviceDefinitionVersionError {
    pub fn from_body(body: &str) -> GetDeviceDefinitionVersionError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "BadRequestException" => {
                        GetDeviceDefinitionVersionError::BadRequest(String::from(error_message))
                    }
                    "ValidationException" => {
                        GetDeviceDefinitionVersionError::Validation(error_message.to_string())
                    }
                    _ => GetDeviceDefinitionVersionError::Unknown(String::from(body)),
                }
            }
            Err(_) => GetDeviceDefinitionVersionError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for GetDeviceDefinitionVersionError {
    fn from(err: serde_json::error::Error) -> GetDeviceDefinitionVersionError {
        GetDeviceDefinitionVersionError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for GetDeviceDefinitionVersionError {
    fn from(err: CredentialsError) -> GetDeviceDefinitionVersionError {
        GetDeviceDefinitionVersionError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GetDeviceDefinitionVersionError {
    fn from(err: HttpDispatchError) -> GetDeviceDefinitionVersionError {
        GetDeviceDefinitionVersionError::HttpDispatch(err)
    }
}
impl From<io::Error> for GetDeviceDefinitionVersionError {
    fn from(err: io::Error) -> GetDeviceDefinitionVersionError {
        GetDeviceDefinitionVersionError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for GetDeviceDefinitionVersionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetDeviceDefinitionVersionError {
    fn description(&self) -> &str {
        match *self {
            GetDeviceDefinitionVersionError::BadRequest(ref cause) => cause,
            GetDeviceDefinitionVersionError::Validation(ref cause) => cause,
            GetDeviceDefinitionVersionError::Credentials(ref err) => err.description(),
            GetDeviceDefinitionVersionError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            GetDeviceDefinitionVersionError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by GetFunctionDefinition
#[derive(Debug, PartialEq)]
pub enum GetFunctionDefinitionError {
    ///General Error
    BadRequest(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl GetFunctionDefinitionError {
    pub fn from_body(body: &str) -> GetFunctionDefinitionError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "BadRequestException" => {
                        GetFunctionDefinitionError::BadRequest(String::from(error_message))
                    }
                    "ValidationException" => {
                        GetFunctionDefinitionError::Validation(error_message.to_string())
                    }
                    _ => GetFunctionDefinitionError::Unknown(String::from(body)),
                }
            }
            Err(_) => GetFunctionDefinitionError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for GetFunctionDefinitionError {
    fn from(err: serde_json::error::Error) -> GetFunctionDefinitionError {
        GetFunctionDefinitionError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for GetFunctionDefinitionError {
    fn from(err: CredentialsError) -> GetFunctionDefinitionError {
        GetFunctionDefinitionError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GetFunctionDefinitionError {
    fn from(err: HttpDispatchError) -> GetFunctionDefinitionError {
        GetFunctionDefinitionError::HttpDispatch(err)
    }
}
impl From<io::Error> for GetFunctionDefinitionError {
    fn from(err: io::Error) -> GetFunctionDefinitionError {
        GetFunctionDefinitionError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for GetFunctionDefinitionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetFunctionDefinitionError {
    fn description(&self) -> &str {
        match *self {
            GetFunctionDefinitionError::BadRequest(ref cause) => cause,
            GetFunctionDefinitionError::Validation(ref cause) => cause,
            GetFunctionDefinitionError::Credentials(ref err) => err.description(),
            GetFunctionDefinitionError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            GetFunctionDefinitionError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by GetFunctionDefinitionVersion
#[derive(Debug, PartialEq)]
pub enum GetFunctionDefinitionVersionError {
    ///General Error
    BadRequest(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl GetFunctionDefinitionVersionError {
    pub fn from_body(body: &str) -> GetFunctionDefinitionVersionError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "BadRequestException" => {
                        GetFunctionDefinitionVersionError::BadRequest(String::from(error_message))
                    }
                    "ValidationException" => {
                        GetFunctionDefinitionVersionError::Validation(error_message.to_string())
                    }
                    _ => GetFunctionDefinitionVersionError::Unknown(String::from(body)),
                }
            }
            Err(_) => GetFunctionDefinitionVersionError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for GetFunctionDefinitionVersionError {
    fn from(err: serde_json::error::Error) -> GetFunctionDefinitionVersionError {
        GetFunctionDefinitionVersionError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for GetFunctionDefinitionVersionError {
    fn from(err: CredentialsError) -> GetFunctionDefinitionVersionError {
        GetFunctionDefinitionVersionError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GetFunctionDefinitionVersionError {
    fn from(err: HttpDispatchError) -> GetFunctionDefinitionVersionError {
        GetFunctionDefinitionVersionError::HttpDispatch(err)
    }
}
impl From<io::Error> for GetFunctionDefinitionVersionError {
    fn from(err: io::Error) -> GetFunctionDefinitionVersionError {
        GetFunctionDefinitionVersionError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for GetFunctionDefinitionVersionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetFunctionDefinitionVersionError {
    fn description(&self) -> &str {
        match *self {
            GetFunctionDefinitionVersionError::BadRequest(ref cause) => cause,
            GetFunctionDefinitionVersionError::Validation(ref cause) => cause,
            GetFunctionDefinitionVersionError::Credentials(ref err) => err.description(),
            GetFunctionDefinitionVersionError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            GetFunctionDefinitionVersionError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by GetGroup
#[derive(Debug, PartialEq)]
pub enum GetGroupError {
    ///General Error
    BadRequest(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl GetGroupError {
    pub fn from_body(body: &str) -> GetGroupError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "BadRequestException" => GetGroupError::BadRequest(String::from(error_message)),
                    "ValidationException" => GetGroupError::Validation(error_message.to_string()),
                    _ => GetGroupError::Unknown(String::from(body)),
                }
            }
            Err(_) => GetGroupError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for GetGroupError {
    fn from(err: serde_json::error::Error) -> GetGroupError {
        GetGroupError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for GetGroupError {
    fn from(err: CredentialsError) -> GetGroupError {
        GetGroupError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GetGroupError {
    fn from(err: HttpDispatchError) -> GetGroupError {
        GetGroupError::HttpDispatch(err)
    }
}
impl From<io::Error> for GetGroupError {
    fn from(err: io::Error) -> GetGroupError {
        GetGroupError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for GetGroupError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetGroupError {
    fn description(&self) -> &str {
        match *self {
            GetGroupError::BadRequest(ref cause) => cause,
            GetGroupError::Validation(ref cause) => cause,
            GetGroupError::Credentials(ref err) => err.description(),
            GetGroupError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            GetGroupError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by GetGroupCertificateAuthority
#[derive(Debug, PartialEq)]
pub enum GetGroupCertificateAuthorityError {
    ///General Error
    BadRequest(String),
    ///General Error
    InternalServerError(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl GetGroupCertificateAuthorityError {
    pub fn from_body(body: &str) -> GetGroupCertificateAuthorityError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "BadRequestException" => {
                        GetGroupCertificateAuthorityError::BadRequest(String::from(error_message))
                    }
                    "InternalServerErrorException" => {
                        GetGroupCertificateAuthorityError::InternalServerError(String::from(
                            error_message,
                        ))
                    }
                    "ValidationException" => {
                        GetGroupCertificateAuthorityError::Validation(error_message.to_string())
                    }
                    _ => GetGroupCertificateAuthorityError::Unknown(String::from(body)),
                }
            }
            Err(_) => GetGroupCertificateAuthorityError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for GetGroupCertificateAuthorityError {
    fn from(err: serde_json::error::Error) -> GetGroupCertificateAuthorityError {
        GetGroupCertificateAuthorityError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for GetGroupCertificateAuthorityError {
    fn from(err: CredentialsError) -> GetGroupCertificateAuthorityError {
        GetGroupCertificateAuthorityError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GetGroupCertificateAuthorityError {
    fn from(err: HttpDispatchError) -> GetGroupCertificateAuthorityError {
        GetGroupCertificateAuthorityError::HttpDispatch(err)
    }
}
impl From<io::Error> for GetGroupCertificateAuthorityError {
    fn from(err: io::Error) -> GetGroupCertificateAuthorityError {
        GetGroupCertificateAuthorityError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for GetGroupCertificateAuthorityError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetGroupCertificateAuthorityError {
    fn description(&self) -> &str {
        match *self {
            GetGroupCertificateAuthorityError::BadRequest(ref cause) => cause,
            GetGroupCertificateAuthorityError::InternalServerError(ref cause) => cause,
            GetGroupCertificateAuthorityError::Validation(ref cause) => cause,
            GetGroupCertificateAuthorityError::Credentials(ref err) => err.description(),
            GetGroupCertificateAuthorityError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            GetGroupCertificateAuthorityError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by GetGroupCertificateConfiguration
#[derive(Debug, PartialEq)]
pub enum GetGroupCertificateConfigurationError {
    ///General Error
    BadRequest(String),
    ///General Error
    InternalServerError(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl GetGroupCertificateConfigurationError {
    pub fn from_body(body: &str) -> GetGroupCertificateConfigurationError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "BadRequestException" => GetGroupCertificateConfigurationError::BadRequest(
                        String::from(error_message),
                    ),
                    "InternalServerErrorException" => {
                        GetGroupCertificateConfigurationError::InternalServerError(String::from(
                            error_message,
                        ))
                    }
                    "ValidationException" => {
                        GetGroupCertificateConfigurationError::Validation(error_message.to_string())
                    }
                    _ => GetGroupCertificateConfigurationError::Unknown(String::from(body)),
                }
            }
            Err(_) => GetGroupCertificateConfigurationError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for GetGroupCertificateConfigurationError {
    fn from(err: serde_json::error::Error) -> GetGroupCertificateConfigurationError {
        GetGroupCertificateConfigurationError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for GetGroupCertificateConfigurationError {
    fn from(err: CredentialsError) -> GetGroupCertificateConfigurationError {
        GetGroupCertificateConfigurationError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GetGroupCertificateConfigurationError {
    fn from(err: HttpDispatchError) -> GetGroupCertificateConfigurationError {
        GetGroupCertificateConfigurationError::HttpDispatch(err)
    }
}
impl From<io::Error> for GetGroupCertificateConfigurationError {
    fn from(err: io::Error) -> GetGroupCertificateConfigurationError {
        GetGroupCertificateConfigurationError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for GetGroupCertificateConfigurationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetGroupCertificateConfigurationError {
    fn description(&self) -> &str {
        match *self {
            GetGroupCertificateConfigurationError::BadRequest(ref cause) => cause,
            GetGroupCertificateConfigurationError::InternalServerError(ref cause) => cause,
            GetGroupCertificateConfigurationError::Validation(ref cause) => cause,
            GetGroupCertificateConfigurationError::Credentials(ref err) => err.description(),
            GetGroupCertificateConfigurationError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            GetGroupCertificateConfigurationError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by GetGroupVersion
#[derive(Debug, PartialEq)]
pub enum GetGroupVersionError {
    ///General Error
    BadRequest(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl GetGroupVersionError {
    pub fn from_body(body: &str) -> GetGroupVersionError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "BadRequestException" => {
                        GetGroupVersionError::BadRequest(String::from(error_message))
                    }
                    "ValidationException" => {
                        GetGroupVersionError::Validation(error_message.to_string())
                    }
                    _ => GetGroupVersionError::Unknown(String::from(body)),
                }
            }
            Err(_) => GetGroupVersionError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for GetGroupVersionError {
    fn from(err: serde_json::error::Error) -> GetGroupVersionError {
        GetGroupVersionError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for GetGroupVersionError {
    fn from(err: CredentialsError) -> GetGroupVersionError {
        GetGroupVersionError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GetGroupVersionError {
    fn from(err: HttpDispatchError) -> GetGroupVersionError {
        GetGroupVersionError::HttpDispatch(err)
    }
}
impl From<io::Error> for GetGroupVersionError {
    fn from(err: io::Error) -> GetGroupVersionError {
        GetGroupVersionError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for GetGroupVersionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetGroupVersionError {
    fn description(&self) -> &str {
        match *self {
            GetGroupVersionError::BadRequest(ref cause) => cause,
            GetGroupVersionError::Validation(ref cause) => cause,
            GetGroupVersionError::Credentials(ref err) => err.description(),
            GetGroupVersionError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            GetGroupVersionError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by GetLoggerDefinition
#[derive(Debug, PartialEq)]
pub enum GetLoggerDefinitionError {
    ///General Error
    BadRequest(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl GetLoggerDefinitionError {
    pub fn from_body(body: &str) -> GetLoggerDefinitionError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "BadRequestException" => {
                        GetLoggerDefinitionError::BadRequest(String::from(error_message))
                    }
                    "ValidationException" => {
                        GetLoggerDefinitionError::Validation(error_message.to_string())
                    }
                    _ => GetLoggerDefinitionError::Unknown(String::from(body)),
                }
            }
            Err(_) => GetLoggerDefinitionError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for GetLoggerDefinitionError {
    fn from(err: serde_json::error::Error) -> GetLoggerDefinitionError {
        GetLoggerDefinitionError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for GetLoggerDefinitionError {
    fn from(err: CredentialsError) -> GetLoggerDefinitionError {
        GetLoggerDefinitionError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GetLoggerDefinitionError {
    fn from(err: HttpDispatchError) -> GetLoggerDefinitionError {
        GetLoggerDefinitionError::HttpDispatch(err)
    }
}
impl From<io::Error> for GetLoggerDefinitionError {
    fn from(err: io::Error) -> GetLoggerDefinitionError {
        GetLoggerDefinitionError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for GetLoggerDefinitionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetLoggerDefinitionError {
    fn description(&self) -> &str {
        match *self {
            GetLoggerDefinitionError::BadRequest(ref cause) => cause,
            GetLoggerDefinitionError::Validation(ref cause) => cause,
            GetLoggerDefinitionError::Credentials(ref err) => err.description(),
            GetLoggerDefinitionError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            GetLoggerDefinitionError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by GetLoggerDefinitionVersion
#[derive(Debug, PartialEq)]
pub enum GetLoggerDefinitionVersionError {
    ///General Error
    BadRequest(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl GetLoggerDefinitionVersionError {
    pub fn from_body(body: &str) -> GetLoggerDefinitionVersionError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "BadRequestException" => {
                        GetLoggerDefinitionVersionError::BadRequest(String::from(error_message))
                    }
                    "ValidationException" => {
                        GetLoggerDefinitionVersionError::Validation(error_message.to_string())
                    }
                    _ => GetLoggerDefinitionVersionError::Unknown(String::from(body)),
                }
            }
            Err(_) => GetLoggerDefinitionVersionError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for GetLoggerDefinitionVersionError {
    fn from(err: serde_json::error::Error) -> GetLoggerDefinitionVersionError {
        GetLoggerDefinitionVersionError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for GetLoggerDefinitionVersionError {
    fn from(err: CredentialsError) -> GetLoggerDefinitionVersionError {
        GetLoggerDefinitionVersionError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GetLoggerDefinitionVersionError {
    fn from(err: HttpDispatchError) -> GetLoggerDefinitionVersionError {
        GetLoggerDefinitionVersionError::HttpDispatch(err)
    }
}
impl From<io::Error> for GetLoggerDefinitionVersionError {
    fn from(err: io::Error) -> GetLoggerDefinitionVersionError {
        GetLoggerDefinitionVersionError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for GetLoggerDefinitionVersionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetLoggerDefinitionVersionError {
    fn description(&self) -> &str {
        match *self {
            GetLoggerDefinitionVersionError::BadRequest(ref cause) => cause,
            GetLoggerDefinitionVersionError::Validation(ref cause) => cause,
            GetLoggerDefinitionVersionError::Credentials(ref err) => err.description(),
            GetLoggerDefinitionVersionError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            GetLoggerDefinitionVersionError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by GetServiceRoleForAccount
#[derive(Debug, PartialEq)]
pub enum GetServiceRoleForAccountError {
    ///General Error
    InternalServerError(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl GetServiceRoleForAccountError {
    pub fn from_body(body: &str) -> GetServiceRoleForAccountError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InternalServerErrorException" => {
                        GetServiceRoleForAccountError::InternalServerError(String::from(
                            error_message,
                        ))
                    }
                    "ValidationException" => {
                        GetServiceRoleForAccountError::Validation(error_message.to_string())
                    }
                    _ => GetServiceRoleForAccountError::Unknown(String::from(body)),
                }
            }
            Err(_) => GetServiceRoleForAccountError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for GetServiceRoleForAccountError {
    fn from(err: serde_json::error::Error) -> GetServiceRoleForAccountError {
        GetServiceRoleForAccountError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for GetServiceRoleForAccountError {
    fn from(err: CredentialsError) -> GetServiceRoleForAccountError {
        GetServiceRoleForAccountError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GetServiceRoleForAccountError {
    fn from(err: HttpDispatchError) -> GetServiceRoleForAccountError {
        GetServiceRoleForAccountError::HttpDispatch(err)
    }
}
impl From<io::Error> for GetServiceRoleForAccountError {
    fn from(err: io::Error) -> GetServiceRoleForAccountError {
        GetServiceRoleForAccountError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for GetServiceRoleForAccountError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetServiceRoleForAccountError {
    fn description(&self) -> &str {
        match *self {
            GetServiceRoleForAccountError::InternalServerError(ref cause) => cause,
            GetServiceRoleForAccountError::Validation(ref cause) => cause,
            GetServiceRoleForAccountError::Credentials(ref err) => err.description(),
            GetServiceRoleForAccountError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            GetServiceRoleForAccountError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by GetSubscriptionDefinition
#[derive(Debug, PartialEq)]
pub enum GetSubscriptionDefinitionError {
    ///General Error
    BadRequest(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl GetSubscriptionDefinitionError {
    pub fn from_body(body: &str) -> GetSubscriptionDefinitionError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "BadRequestException" => {
                        GetSubscriptionDefinitionError::BadRequest(String::from(error_message))
                    }
                    "ValidationException" => {
                        GetSubscriptionDefinitionError::Validation(error_message.to_string())
                    }
                    _ => GetSubscriptionDefinitionError::Unknown(String::from(body)),
                }
            }
            Err(_) => GetSubscriptionDefinitionError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for GetSubscriptionDefinitionError {
    fn from(err: serde_json::error::Error) -> GetSubscriptionDefinitionError {
        GetSubscriptionDefinitionError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for GetSubscriptionDefinitionError {
    fn from(err: CredentialsError) -> GetSubscriptionDefinitionError {
        GetSubscriptionDefinitionError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GetSubscriptionDefinitionError {
    fn from(err: HttpDispatchError) -> GetSubscriptionDefinitionError {
        GetSubscriptionDefinitionError::HttpDispatch(err)
    }
}
impl From<io::Error> for GetSubscriptionDefinitionError {
    fn from(err: io::Error) -> GetSubscriptionDefinitionError {
        GetSubscriptionDefinitionError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for GetSubscriptionDefinitionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetSubscriptionDefinitionError {
    fn description(&self) -> &str {
        match *self {
            GetSubscriptionDefinitionError::BadRequest(ref cause) => cause,
            GetSubscriptionDefinitionError::Validation(ref cause) => cause,
            GetSubscriptionDefinitionError::Credentials(ref err) => err.description(),
            GetSubscriptionDefinitionError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            GetSubscriptionDefinitionError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by GetSubscriptionDefinitionVersion
#[derive(Debug, PartialEq)]
pub enum GetSubscriptionDefinitionVersionError {
    ///General Error
    BadRequest(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl GetSubscriptionDefinitionVersionError {
    pub fn from_body(body: &str) -> GetSubscriptionDefinitionVersionError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "BadRequestException" => GetSubscriptionDefinitionVersionError::BadRequest(
                        String::from(error_message),
                    ),
                    "ValidationException" => {
                        GetSubscriptionDefinitionVersionError::Validation(error_message.to_string())
                    }
                    _ => GetSubscriptionDefinitionVersionError::Unknown(String::from(body)),
                }
            }
            Err(_) => GetSubscriptionDefinitionVersionError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for GetSubscriptionDefinitionVersionError {
    fn from(err: serde_json::error::Error) -> GetSubscriptionDefinitionVersionError {
        GetSubscriptionDefinitionVersionError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for GetSubscriptionDefinitionVersionError {
    fn from(err: CredentialsError) -> GetSubscriptionDefinitionVersionError {
        GetSubscriptionDefinitionVersionError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GetSubscriptionDefinitionVersionError {
    fn from(err: HttpDispatchError) -> GetSubscriptionDefinitionVersionError {
        GetSubscriptionDefinitionVersionError::HttpDispatch(err)
    }
}
impl From<io::Error> for GetSubscriptionDefinitionVersionError {
    fn from(err: io::Error) -> GetSubscriptionDefinitionVersionError {
        GetSubscriptionDefinitionVersionError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for GetSubscriptionDefinitionVersionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetSubscriptionDefinitionVersionError {
    fn description(&self) -> &str {
        match *self {
            GetSubscriptionDefinitionVersionError::BadRequest(ref cause) => cause,
            GetSubscriptionDefinitionVersionError::Validation(ref cause) => cause,
            GetSubscriptionDefinitionVersionError::Credentials(ref err) => err.description(),
            GetSubscriptionDefinitionVersionError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            GetSubscriptionDefinitionVersionError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by ListCoreDefinitionVersions
#[derive(Debug, PartialEq)]
pub enum ListCoreDefinitionVersionsError {
    ///General Error
    BadRequest(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl ListCoreDefinitionVersionsError {
    pub fn from_body(body: &str) -> ListCoreDefinitionVersionsError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "BadRequestException" => {
                        ListCoreDefinitionVersionsError::BadRequest(String::from(error_message))
                    }
                    "ValidationException" => {
                        ListCoreDefinitionVersionsError::Validation(error_message.to_string())
                    }
                    _ => ListCoreDefinitionVersionsError::Unknown(String::from(body)),
                }
            }
            Err(_) => ListCoreDefinitionVersionsError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for ListCoreDefinitionVersionsError {
    fn from(err: serde_json::error::Error) -> ListCoreDefinitionVersionsError {
        ListCoreDefinitionVersionsError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for ListCoreDefinitionVersionsError {
    fn from(err: CredentialsError) -> ListCoreDefinitionVersionsError {
        ListCoreDefinitionVersionsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ListCoreDefinitionVersionsError {
    fn from(err: HttpDispatchError) -> ListCoreDefinitionVersionsError {
        ListCoreDefinitionVersionsError::HttpDispatch(err)
    }
}
impl From<io::Error> for ListCoreDefinitionVersionsError {
    fn from(err: io::Error) -> ListCoreDefinitionVersionsError {
        ListCoreDefinitionVersionsError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for ListCoreDefinitionVersionsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListCoreDefinitionVersionsError {
    fn description(&self) -> &str {
        match *self {
            ListCoreDefinitionVersionsError::BadRequest(ref cause) => cause,
            ListCoreDefinitionVersionsError::Validation(ref cause) => cause,
            ListCoreDefinitionVersionsError::Credentials(ref err) => err.description(),
            ListCoreDefinitionVersionsError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            ListCoreDefinitionVersionsError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by ListCoreDefinitions
#[derive(Debug, PartialEq)]
pub enum ListCoreDefinitionsError {
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl ListCoreDefinitionsError {
    pub fn from_body(body: &str) -> ListCoreDefinitionsError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "ValidationException" => {
                        ListCoreDefinitionsError::Validation(error_message.to_string())
                    }
                    _ => ListCoreDefinitionsError::Unknown(String::from(body)),
                }
            }
            Err(_) => ListCoreDefinitionsError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for ListCoreDefinitionsError {
    fn from(err: serde_json::error::Error) -> ListCoreDefinitionsError {
        ListCoreDefinitionsError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for ListCoreDefinitionsError {
    fn from(err: CredentialsError) -> ListCoreDefinitionsError {
        ListCoreDefinitionsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ListCoreDefinitionsError {
    fn from(err: HttpDispatchError) -> ListCoreDefinitionsError {
        ListCoreDefinitionsError::HttpDispatch(err)
    }
}
impl From<io::Error> for ListCoreDefinitionsError {
    fn from(err: io::Error) -> ListCoreDefinitionsError {
        ListCoreDefinitionsError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for ListCoreDefinitionsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListCoreDefinitionsError {
    fn description(&self) -> &str {
        match *self {
            ListCoreDefinitionsError::Validation(ref cause) => cause,
            ListCoreDefinitionsError::Credentials(ref err) => err.description(),
            ListCoreDefinitionsError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            ListCoreDefinitionsError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by ListDeployments
#[derive(Debug, PartialEq)]
pub enum ListDeploymentsError {
    ///General Error
    BadRequest(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl ListDeploymentsError {
    pub fn from_body(body: &str) -> ListDeploymentsError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "BadRequestException" => {
                        ListDeploymentsError::BadRequest(String::from(error_message))
                    }
                    "ValidationException" => {
                        ListDeploymentsError::Validation(error_message.to_string())
                    }
                    _ => ListDeploymentsError::Unknown(String::from(body)),
                }
            }
            Err(_) => ListDeploymentsError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for ListDeploymentsError {
    fn from(err: serde_json::error::Error) -> ListDeploymentsError {
        ListDeploymentsError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for ListDeploymentsError {
    fn from(err: CredentialsError) -> ListDeploymentsError {
        ListDeploymentsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ListDeploymentsError {
    fn from(err: HttpDispatchError) -> ListDeploymentsError {
        ListDeploymentsError::HttpDispatch(err)
    }
}
impl From<io::Error> for ListDeploymentsError {
    fn from(err: io::Error) -> ListDeploymentsError {
        ListDeploymentsError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for ListDeploymentsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListDeploymentsError {
    fn description(&self) -> &str {
        match *self {
            ListDeploymentsError::BadRequest(ref cause) => cause,
            ListDeploymentsError::Validation(ref cause) => cause,
            ListDeploymentsError::Credentials(ref err) => err.description(),
            ListDeploymentsError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            ListDeploymentsError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by ListDeviceDefinitionVersions
#[derive(Debug, PartialEq)]
pub enum ListDeviceDefinitionVersionsError {
    ///General Error
    BadRequest(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl ListDeviceDefinitionVersionsError {
    pub fn from_body(body: &str) -> ListDeviceDefinitionVersionsError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "BadRequestException" => {
                        ListDeviceDefinitionVersionsError::BadRequest(String::from(error_message))
                    }
                    "ValidationException" => {
                        ListDeviceDefinitionVersionsError::Validation(error_message.to_string())
                    }
                    _ => ListDeviceDefinitionVersionsError::Unknown(String::from(body)),
                }
            }
            Err(_) => ListDeviceDefinitionVersionsError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for ListDeviceDefinitionVersionsError {
    fn from(err: serde_json::error::Error) -> ListDeviceDefinitionVersionsError {
        ListDeviceDefinitionVersionsError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for ListDeviceDefinitionVersionsError {
    fn from(err: CredentialsError) -> ListDeviceDefinitionVersionsError {
        ListDeviceDefinitionVersionsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ListDeviceDefinitionVersionsError {
    fn from(err: HttpDispatchError) -> ListDeviceDefinitionVersionsError {
        ListDeviceDefinitionVersionsError::HttpDispatch(err)
    }
}
impl From<io::Error> for ListDeviceDefinitionVersionsError {
    fn from(err: io::Error) -> ListDeviceDefinitionVersionsError {
        ListDeviceDefinitionVersionsError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for ListDeviceDefinitionVersionsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListDeviceDefinitionVersionsError {
    fn description(&self) -> &str {
        match *self {
            ListDeviceDefinitionVersionsError::BadRequest(ref cause) => cause,
            ListDeviceDefinitionVersionsError::Validation(ref cause) => cause,
            ListDeviceDefinitionVersionsError::Credentials(ref err) => err.description(),
            ListDeviceDefinitionVersionsError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            ListDeviceDefinitionVersionsError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by ListDeviceDefinitions
#[derive(Debug, PartialEq)]
pub enum ListDeviceDefinitionsError {
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl ListDeviceDefinitionsError {
    pub fn from_body(body: &str) -> ListDeviceDefinitionsError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "ValidationException" => {
                        ListDeviceDefinitionsError::Validation(error_message.to_string())
                    }
                    _ => ListDeviceDefinitionsError::Unknown(String::from(body)),
                }
            }
            Err(_) => ListDeviceDefinitionsError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for ListDeviceDefinitionsError {
    fn from(err: serde_json::error::Error) -> ListDeviceDefinitionsError {
        ListDeviceDefinitionsError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for ListDeviceDefinitionsError {
    fn from(err: CredentialsError) -> ListDeviceDefinitionsError {
        ListDeviceDefinitionsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ListDeviceDefinitionsError {
    fn from(err: HttpDispatchError) -> ListDeviceDefinitionsError {
        ListDeviceDefinitionsError::HttpDispatch(err)
    }
}
impl From<io::Error> for ListDeviceDefinitionsError {
    fn from(err: io::Error) -> ListDeviceDefinitionsError {
        ListDeviceDefinitionsError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for ListDeviceDefinitionsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListDeviceDefinitionsError {
    fn description(&self) -> &str {
        match *self {
            ListDeviceDefinitionsError::Validation(ref cause) => cause,
            ListDeviceDefinitionsError::Credentials(ref err) => err.description(),
            ListDeviceDefinitionsError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            ListDeviceDefinitionsError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by ListFunctionDefinitionVersions
#[derive(Debug, PartialEq)]
pub enum ListFunctionDefinitionVersionsError {
    ///General Error
    BadRequest(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl ListFunctionDefinitionVersionsError {
    pub fn from_body(body: &str) -> ListFunctionDefinitionVersionsError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "BadRequestException" => {
                        ListFunctionDefinitionVersionsError::BadRequest(String::from(error_message))
                    }
                    "ValidationException" => {
                        ListFunctionDefinitionVersionsError::Validation(error_message.to_string())
                    }
                    _ => ListFunctionDefinitionVersionsError::Unknown(String::from(body)),
                }
            }
            Err(_) => ListFunctionDefinitionVersionsError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for ListFunctionDefinitionVersionsError {
    fn from(err: serde_json::error::Error) -> ListFunctionDefinitionVersionsError {
        ListFunctionDefinitionVersionsError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for ListFunctionDefinitionVersionsError {
    fn from(err: CredentialsError) -> ListFunctionDefinitionVersionsError {
        ListFunctionDefinitionVersionsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ListFunctionDefinitionVersionsError {
    fn from(err: HttpDispatchError) -> ListFunctionDefinitionVersionsError {
        ListFunctionDefinitionVersionsError::HttpDispatch(err)
    }
}
impl From<io::Error> for ListFunctionDefinitionVersionsError {
    fn from(err: io::Error) -> ListFunctionDefinitionVersionsError {
        ListFunctionDefinitionVersionsError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for ListFunctionDefinitionVersionsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListFunctionDefinitionVersionsError {
    fn description(&self) -> &str {
        match *self {
            ListFunctionDefinitionVersionsError::BadRequest(ref cause) => cause,
            ListFunctionDefinitionVersionsError::Validation(ref cause) => cause,
            ListFunctionDefinitionVersionsError::Credentials(ref err) => err.description(),
            ListFunctionDefinitionVersionsError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            ListFunctionDefinitionVersionsError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by ListFunctionDefinitions
#[derive(Debug, PartialEq)]
pub enum ListFunctionDefinitionsError {
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl ListFunctionDefinitionsError {
    pub fn from_body(body: &str) -> ListFunctionDefinitionsError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "ValidationException" => {
                        ListFunctionDefinitionsError::Validation(error_message.to_string())
                    }
                    _ => ListFunctionDefinitionsError::Unknown(String::from(body)),
                }
            }
            Err(_) => ListFunctionDefinitionsError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for ListFunctionDefinitionsError {
    fn from(err: serde_json::error::Error) -> ListFunctionDefinitionsError {
        ListFunctionDefinitionsError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for ListFunctionDefinitionsError {
    fn from(err: CredentialsError) -> ListFunctionDefinitionsError {
        ListFunctionDefinitionsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ListFunctionDefinitionsError {
    fn from(err: HttpDispatchError) -> ListFunctionDefinitionsError {
        ListFunctionDefinitionsError::HttpDispatch(err)
    }
}
impl From<io::Error> for ListFunctionDefinitionsError {
    fn from(err: io::Error) -> ListFunctionDefinitionsError {
        ListFunctionDefinitionsError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for ListFunctionDefinitionsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListFunctionDefinitionsError {
    fn description(&self) -> &str {
        match *self {
            ListFunctionDefinitionsError::Validation(ref cause) => cause,
            ListFunctionDefinitionsError::Credentials(ref err) => err.description(),
            ListFunctionDefinitionsError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            ListFunctionDefinitionsError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by ListGroupCertificateAuthorities
#[derive(Debug, PartialEq)]
pub enum ListGroupCertificateAuthoritiesError {
    ///General Error
    BadRequest(String),
    ///General Error
    InternalServerError(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl ListGroupCertificateAuthoritiesError {
    pub fn from_body(body: &str) -> ListGroupCertificateAuthoritiesError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "BadRequestException" => ListGroupCertificateAuthoritiesError::BadRequest(
                        String::from(error_message),
                    ),
                    "InternalServerErrorException" => {
                        ListGroupCertificateAuthoritiesError::InternalServerError(String::from(
                            error_message,
                        ))
                    }
                    "ValidationException" => {
                        ListGroupCertificateAuthoritiesError::Validation(error_message.to_string())
                    }
                    _ => ListGroupCertificateAuthoritiesError::Unknown(String::from(body)),
                }
            }
            Err(_) => ListGroupCertificateAuthoritiesError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for ListGroupCertificateAuthoritiesError {
    fn from(err: serde_json::error::Error) -> ListGroupCertificateAuthoritiesError {
        ListGroupCertificateAuthoritiesError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for ListGroupCertificateAuthoritiesError {
    fn from(err: CredentialsError) -> ListGroupCertificateAuthoritiesError {
        ListGroupCertificateAuthoritiesError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ListGroupCertificateAuthoritiesError {
    fn from(err: HttpDispatchError) -> ListGroupCertificateAuthoritiesError {
        ListGroupCertificateAuthoritiesError::HttpDispatch(err)
    }
}
impl From<io::Error> for ListGroupCertificateAuthoritiesError {
    fn from(err: io::Error) -> ListGroupCertificateAuthoritiesError {
        ListGroupCertificateAuthoritiesError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for ListGroupCertificateAuthoritiesError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListGroupCertificateAuthoritiesError {
    fn description(&self) -> &str {
        match *self {
            ListGroupCertificateAuthoritiesError::BadRequest(ref cause) => cause,
            ListGroupCertificateAuthoritiesError::InternalServerError(ref cause) => cause,
            ListGroupCertificateAuthoritiesError::Validation(ref cause) => cause,
            ListGroupCertificateAuthoritiesError::Credentials(ref err) => err.description(),
            ListGroupCertificateAuthoritiesError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            ListGroupCertificateAuthoritiesError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by ListGroupVersions
#[derive(Debug, PartialEq)]
pub enum ListGroupVersionsError {
    ///General Error
    BadRequest(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl ListGroupVersionsError {
    pub fn from_body(body: &str) -> ListGroupVersionsError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "BadRequestException" => {
                        ListGroupVersionsError::BadRequest(String::from(error_message))
                    }
                    "ValidationException" => {
                        ListGroupVersionsError::Validation(error_message.to_string())
                    }
                    _ => ListGroupVersionsError::Unknown(String::from(body)),
                }
            }
            Err(_) => ListGroupVersionsError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for ListGroupVersionsError {
    fn from(err: serde_json::error::Error) -> ListGroupVersionsError {
        ListGroupVersionsError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for ListGroupVersionsError {
    fn from(err: CredentialsError) -> ListGroupVersionsError {
        ListGroupVersionsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ListGroupVersionsError {
    fn from(err: HttpDispatchError) -> ListGroupVersionsError {
        ListGroupVersionsError::HttpDispatch(err)
    }
}
impl From<io::Error> for ListGroupVersionsError {
    fn from(err: io::Error) -> ListGroupVersionsError {
        ListGroupVersionsError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for ListGroupVersionsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListGroupVersionsError {
    fn description(&self) -> &str {
        match *self {
            ListGroupVersionsError::BadRequest(ref cause) => cause,
            ListGroupVersionsError::Validation(ref cause) => cause,
            ListGroupVersionsError::Credentials(ref err) => err.description(),
            ListGroupVersionsError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            ListGroupVersionsError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by ListGroups
#[derive(Debug, PartialEq)]
pub enum ListGroupsError {
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl ListGroupsError {
    pub fn from_body(body: &str) -> ListGroupsError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "ValidationException" => ListGroupsError::Validation(error_message.to_string()),
                    _ => ListGroupsError::Unknown(String::from(body)),
                }
            }
            Err(_) => ListGroupsError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for ListGroupsError {
    fn from(err: serde_json::error::Error) -> ListGroupsError {
        ListGroupsError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for ListGroupsError {
    fn from(err: CredentialsError) -> ListGroupsError {
        ListGroupsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ListGroupsError {
    fn from(err: HttpDispatchError) -> ListGroupsError {
        ListGroupsError::HttpDispatch(err)
    }
}
impl From<io::Error> for ListGroupsError {
    fn from(err: io::Error) -> ListGroupsError {
        ListGroupsError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for ListGroupsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListGroupsError {
    fn description(&self) -> &str {
        match *self {
            ListGroupsError::Validation(ref cause) => cause,
            ListGroupsError::Credentials(ref err) => err.description(),
            ListGroupsError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            ListGroupsError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by ListLoggerDefinitionVersions
#[derive(Debug, PartialEq)]
pub enum ListLoggerDefinitionVersionsError {
    ///General Error
    BadRequest(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl ListLoggerDefinitionVersionsError {
    pub fn from_body(body: &str) -> ListLoggerDefinitionVersionsError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "BadRequestException" => {
                        ListLoggerDefinitionVersionsError::BadRequest(String::from(error_message))
                    }
                    "ValidationException" => {
                        ListLoggerDefinitionVersionsError::Validation(error_message.to_string())
                    }
                    _ => ListLoggerDefinitionVersionsError::Unknown(String::from(body)),
                }
            }
            Err(_) => ListLoggerDefinitionVersionsError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for ListLoggerDefinitionVersionsError {
    fn from(err: serde_json::error::Error) -> ListLoggerDefinitionVersionsError {
        ListLoggerDefinitionVersionsError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for ListLoggerDefinitionVersionsError {
    fn from(err: CredentialsError) -> ListLoggerDefinitionVersionsError {
        ListLoggerDefinitionVersionsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ListLoggerDefinitionVersionsError {
    fn from(err: HttpDispatchError) -> ListLoggerDefinitionVersionsError {
        ListLoggerDefinitionVersionsError::HttpDispatch(err)
    }
}
impl From<io::Error> for ListLoggerDefinitionVersionsError {
    fn from(err: io::Error) -> ListLoggerDefinitionVersionsError {
        ListLoggerDefinitionVersionsError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for ListLoggerDefinitionVersionsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListLoggerDefinitionVersionsError {
    fn description(&self) -> &str {
        match *self {
            ListLoggerDefinitionVersionsError::BadRequest(ref cause) => cause,
            ListLoggerDefinitionVersionsError::Validation(ref cause) => cause,
            ListLoggerDefinitionVersionsError::Credentials(ref err) => err.description(),
            ListLoggerDefinitionVersionsError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            ListLoggerDefinitionVersionsError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by ListLoggerDefinitions
#[derive(Debug, PartialEq)]
pub enum ListLoggerDefinitionsError {
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl ListLoggerDefinitionsError {
    pub fn from_body(body: &str) -> ListLoggerDefinitionsError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "ValidationException" => {
                        ListLoggerDefinitionsError::Validation(error_message.to_string())
                    }
                    _ => ListLoggerDefinitionsError::Unknown(String::from(body)),
                }
            }
            Err(_) => ListLoggerDefinitionsError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for ListLoggerDefinitionsError {
    fn from(err: serde_json::error::Error) -> ListLoggerDefinitionsError {
        ListLoggerDefinitionsError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for ListLoggerDefinitionsError {
    fn from(err: CredentialsError) -> ListLoggerDefinitionsError {
        ListLoggerDefinitionsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ListLoggerDefinitionsError {
    fn from(err: HttpDispatchError) -> ListLoggerDefinitionsError {
        ListLoggerDefinitionsError::HttpDispatch(err)
    }
}
impl From<io::Error> for ListLoggerDefinitionsError {
    fn from(err: io::Error) -> ListLoggerDefinitionsError {
        ListLoggerDefinitionsError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for ListLoggerDefinitionsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListLoggerDefinitionsError {
    fn description(&self) -> &str {
        match *self {
            ListLoggerDefinitionsError::Validation(ref cause) => cause,
            ListLoggerDefinitionsError::Credentials(ref err) => err.description(),
            ListLoggerDefinitionsError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            ListLoggerDefinitionsError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by ListSubscriptionDefinitionVersions
#[derive(Debug, PartialEq)]
pub enum ListSubscriptionDefinitionVersionsError {
    ///General Error
    BadRequest(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl ListSubscriptionDefinitionVersionsError {
    pub fn from_body(body: &str) -> ListSubscriptionDefinitionVersionsError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "BadRequestException" => ListSubscriptionDefinitionVersionsError::BadRequest(
                        String::from(error_message),
                    ),
                    "ValidationException" => ListSubscriptionDefinitionVersionsError::Validation(
                        error_message.to_string(),
                    ),
                    _ => ListSubscriptionDefinitionVersionsError::Unknown(String::from(body)),
                }
            }
            Err(_) => ListSubscriptionDefinitionVersionsError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for ListSubscriptionDefinitionVersionsError {
    fn from(err: serde_json::error::Error) -> ListSubscriptionDefinitionVersionsError {
        ListSubscriptionDefinitionVersionsError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for ListSubscriptionDefinitionVersionsError {
    fn from(err: CredentialsError) -> ListSubscriptionDefinitionVersionsError {
        ListSubscriptionDefinitionVersionsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ListSubscriptionDefinitionVersionsError {
    fn from(err: HttpDispatchError) -> ListSubscriptionDefinitionVersionsError {
        ListSubscriptionDefinitionVersionsError::HttpDispatch(err)
    }
}
impl From<io::Error> for ListSubscriptionDefinitionVersionsError {
    fn from(err: io::Error) -> ListSubscriptionDefinitionVersionsError {
        ListSubscriptionDefinitionVersionsError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for ListSubscriptionDefinitionVersionsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListSubscriptionDefinitionVersionsError {
    fn description(&self) -> &str {
        match *self {
            ListSubscriptionDefinitionVersionsError::BadRequest(ref cause) => cause,
            ListSubscriptionDefinitionVersionsError::Validation(ref cause) => cause,
            ListSubscriptionDefinitionVersionsError::Credentials(ref err) => err.description(),
            ListSubscriptionDefinitionVersionsError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            ListSubscriptionDefinitionVersionsError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by ListSubscriptionDefinitions
#[derive(Debug, PartialEq)]
pub enum ListSubscriptionDefinitionsError {
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl ListSubscriptionDefinitionsError {
    pub fn from_body(body: &str) -> ListSubscriptionDefinitionsError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "ValidationException" => {
                        ListSubscriptionDefinitionsError::Validation(error_message.to_string())
                    }
                    _ => ListSubscriptionDefinitionsError::Unknown(String::from(body)),
                }
            }
            Err(_) => ListSubscriptionDefinitionsError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for ListSubscriptionDefinitionsError {
    fn from(err: serde_json::error::Error) -> ListSubscriptionDefinitionsError {
        ListSubscriptionDefinitionsError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for ListSubscriptionDefinitionsError {
    fn from(err: CredentialsError) -> ListSubscriptionDefinitionsError {
        ListSubscriptionDefinitionsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ListSubscriptionDefinitionsError {
    fn from(err: HttpDispatchError) -> ListSubscriptionDefinitionsError {
        ListSubscriptionDefinitionsError::HttpDispatch(err)
    }
}
impl From<io::Error> for ListSubscriptionDefinitionsError {
    fn from(err: io::Error) -> ListSubscriptionDefinitionsError {
        ListSubscriptionDefinitionsError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for ListSubscriptionDefinitionsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListSubscriptionDefinitionsError {
    fn description(&self) -> &str {
        match *self {
            ListSubscriptionDefinitionsError::Validation(ref cause) => cause,
            ListSubscriptionDefinitionsError::Credentials(ref err) => err.description(),
            ListSubscriptionDefinitionsError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            ListSubscriptionDefinitionsError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by UpdateConnectivityInfo
#[derive(Debug, PartialEq)]
pub enum UpdateConnectivityInfoError {
    ///General Error
    BadRequest(String),
    ///General Error
    InternalServerError(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl UpdateConnectivityInfoError {
    pub fn from_body(body: &str) -> UpdateConnectivityInfoError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "BadRequestException" => {
                        UpdateConnectivityInfoError::BadRequest(String::from(error_message))
                    }
                    "InternalServerErrorException" => {
                        UpdateConnectivityInfoError::InternalServerError(String::from(
                            error_message,
                        ))
                    }
                    "ValidationException" => {
                        UpdateConnectivityInfoError::Validation(error_message.to_string())
                    }
                    _ => UpdateConnectivityInfoError::Unknown(String::from(body)),
                }
            }
            Err(_) => UpdateConnectivityInfoError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for UpdateConnectivityInfoError {
    fn from(err: serde_json::error::Error) -> UpdateConnectivityInfoError {
        UpdateConnectivityInfoError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for UpdateConnectivityInfoError {
    fn from(err: CredentialsError) -> UpdateConnectivityInfoError {
        UpdateConnectivityInfoError::Credentials(err)
    }
}
impl From<HttpDispatchError> for UpdateConnectivityInfoError {
    fn from(err: HttpDispatchError) -> UpdateConnectivityInfoError {
        UpdateConnectivityInfoError::HttpDispatch(err)
    }
}
impl From<io::Error> for UpdateConnectivityInfoError {
    fn from(err: io::Error) -> UpdateConnectivityInfoError {
        UpdateConnectivityInfoError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for UpdateConnectivityInfoError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for UpdateConnectivityInfoError {
    fn description(&self) -> &str {
        match *self {
            UpdateConnectivityInfoError::BadRequest(ref cause) => cause,
            UpdateConnectivityInfoError::InternalServerError(ref cause) => cause,
            UpdateConnectivityInfoError::Validation(ref cause) => cause,
            UpdateConnectivityInfoError::Credentials(ref err) => err.description(),
            UpdateConnectivityInfoError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            UpdateConnectivityInfoError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by UpdateCoreDefinition
#[derive(Debug, PartialEq)]
pub enum UpdateCoreDefinitionError {
    ///General Error
    BadRequest(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl UpdateCoreDefinitionError {
    pub fn from_body(body: &str) -> UpdateCoreDefinitionError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "BadRequestException" => {
                        UpdateCoreDefinitionError::BadRequest(String::from(error_message))
                    }
                    "ValidationException" => {
                        UpdateCoreDefinitionError::Validation(error_message.to_string())
                    }
                    _ => UpdateCoreDefinitionError::Unknown(String::from(body)),
                }
            }
            Err(_) => UpdateCoreDefinitionError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for UpdateCoreDefinitionError {
    fn from(err: serde_json::error::Error) -> UpdateCoreDefinitionError {
        UpdateCoreDefinitionError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for UpdateCoreDefinitionError {
    fn from(err: CredentialsError) -> UpdateCoreDefinitionError {
        UpdateCoreDefinitionError::Credentials(err)
    }
}
impl From<HttpDispatchError> for UpdateCoreDefinitionError {
    fn from(err: HttpDispatchError) -> UpdateCoreDefinitionError {
        UpdateCoreDefinitionError::HttpDispatch(err)
    }
}
impl From<io::Error> for UpdateCoreDefinitionError {
    fn from(err: io::Error) -> UpdateCoreDefinitionError {
        UpdateCoreDefinitionError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for UpdateCoreDefinitionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for UpdateCoreDefinitionError {
    fn description(&self) -> &str {
        match *self {
            UpdateCoreDefinitionError::BadRequest(ref cause) => cause,
            UpdateCoreDefinitionError::Validation(ref cause) => cause,
            UpdateCoreDefinitionError::Credentials(ref err) => err.description(),
            UpdateCoreDefinitionError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            UpdateCoreDefinitionError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by UpdateDeviceDefinition
#[derive(Debug, PartialEq)]
pub enum UpdateDeviceDefinitionError {
    ///General Error
    BadRequest(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl UpdateDeviceDefinitionError {
    pub fn from_body(body: &str) -> UpdateDeviceDefinitionError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "BadRequestException" => {
                        UpdateDeviceDefinitionError::BadRequest(String::from(error_message))
                    }
                    "ValidationException" => {
                        UpdateDeviceDefinitionError::Validation(error_message.to_string())
                    }
                    _ => UpdateDeviceDefinitionError::Unknown(String::from(body)),
                }
            }
            Err(_) => UpdateDeviceDefinitionError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for UpdateDeviceDefinitionError {
    fn from(err: serde_json::error::Error) -> UpdateDeviceDefinitionError {
        UpdateDeviceDefinitionError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for UpdateDeviceDefinitionError {
    fn from(err: CredentialsError) -> UpdateDeviceDefinitionError {
        UpdateDeviceDefinitionError::Credentials(err)
    }
}
impl From<HttpDispatchError> for UpdateDeviceDefinitionError {
    fn from(err: HttpDispatchError) -> UpdateDeviceDefinitionError {
        UpdateDeviceDefinitionError::HttpDispatch(err)
    }
}
impl From<io::Error> for UpdateDeviceDefinitionError {
    fn from(err: io::Error) -> UpdateDeviceDefinitionError {
        UpdateDeviceDefinitionError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for UpdateDeviceDefinitionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for UpdateDeviceDefinitionError {
    fn description(&self) -> &str {
        match *self {
            UpdateDeviceDefinitionError::BadRequest(ref cause) => cause,
            UpdateDeviceDefinitionError::Validation(ref cause) => cause,
            UpdateDeviceDefinitionError::Credentials(ref err) => err.description(),
            UpdateDeviceDefinitionError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            UpdateDeviceDefinitionError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by UpdateFunctionDefinition
#[derive(Debug, PartialEq)]
pub enum UpdateFunctionDefinitionError {
    ///General Error
    BadRequest(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl UpdateFunctionDefinitionError {
    pub fn from_body(body: &str) -> UpdateFunctionDefinitionError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "BadRequestException" => {
                        UpdateFunctionDefinitionError::BadRequest(String::from(error_message))
                    }
                    "ValidationException" => {
                        UpdateFunctionDefinitionError::Validation(error_message.to_string())
                    }
                    _ => UpdateFunctionDefinitionError::Unknown(String::from(body)),
                }
            }
            Err(_) => UpdateFunctionDefinitionError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for UpdateFunctionDefinitionError {
    fn from(err: serde_json::error::Error) -> UpdateFunctionDefinitionError {
        UpdateFunctionDefinitionError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for UpdateFunctionDefinitionError {
    fn from(err: CredentialsError) -> UpdateFunctionDefinitionError {
        UpdateFunctionDefinitionError::Credentials(err)
    }
}
impl From<HttpDispatchError> for UpdateFunctionDefinitionError {
    fn from(err: HttpDispatchError) -> UpdateFunctionDefinitionError {
        UpdateFunctionDefinitionError::HttpDispatch(err)
    }
}
impl From<io::Error> for UpdateFunctionDefinitionError {
    fn from(err: io::Error) -> UpdateFunctionDefinitionError {
        UpdateFunctionDefinitionError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for UpdateFunctionDefinitionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for UpdateFunctionDefinitionError {
    fn description(&self) -> &str {
        match *self {
            UpdateFunctionDefinitionError::BadRequest(ref cause) => cause,
            UpdateFunctionDefinitionError::Validation(ref cause) => cause,
            UpdateFunctionDefinitionError::Credentials(ref err) => err.description(),
            UpdateFunctionDefinitionError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            UpdateFunctionDefinitionError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by UpdateGroup
#[derive(Debug, PartialEq)]
pub enum UpdateGroupError {
    ///General Error
    BadRequest(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl UpdateGroupError {
    pub fn from_body(body: &str) -> UpdateGroupError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "BadRequestException" => {
                        UpdateGroupError::BadRequest(String::from(error_message))
                    }
                    "ValidationException" => {
                        UpdateGroupError::Validation(error_message.to_string())
                    }
                    _ => UpdateGroupError::Unknown(String::from(body)),
                }
            }
            Err(_) => UpdateGroupError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for UpdateGroupError {
    fn from(err: serde_json::error::Error) -> UpdateGroupError {
        UpdateGroupError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for UpdateGroupError {
    fn from(err: CredentialsError) -> UpdateGroupError {
        UpdateGroupError::Credentials(err)
    }
}
impl From<HttpDispatchError> for UpdateGroupError {
    fn from(err: HttpDispatchError) -> UpdateGroupError {
        UpdateGroupError::HttpDispatch(err)
    }
}
impl From<io::Error> for UpdateGroupError {
    fn from(err: io::Error) -> UpdateGroupError {
        UpdateGroupError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for UpdateGroupError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for UpdateGroupError {
    fn description(&self) -> &str {
        match *self {
            UpdateGroupError::BadRequest(ref cause) => cause,
            UpdateGroupError::Validation(ref cause) => cause,
            UpdateGroupError::Credentials(ref err) => err.description(),
            UpdateGroupError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            UpdateGroupError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by UpdateGroupCertificateConfiguration
#[derive(Debug, PartialEq)]
pub enum UpdateGroupCertificateConfigurationError {
    ///General Error
    BadRequest(String),
    ///General Error
    InternalServerError(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl UpdateGroupCertificateConfigurationError {
    pub fn from_body(body: &str) -> UpdateGroupCertificateConfigurationError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "BadRequestException" => UpdateGroupCertificateConfigurationError::BadRequest(
                        String::from(error_message),
                    ),
                    "InternalServerErrorException" => {
                        UpdateGroupCertificateConfigurationError::InternalServerError(
                            String::from(error_message),
                        )
                    }
                    "ValidationException" => UpdateGroupCertificateConfigurationError::Validation(
                        error_message.to_string(),
                    ),
                    _ => UpdateGroupCertificateConfigurationError::Unknown(String::from(body)),
                }
            }
            Err(_) => UpdateGroupCertificateConfigurationError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for UpdateGroupCertificateConfigurationError {
    fn from(err: serde_json::error::Error) -> UpdateGroupCertificateConfigurationError {
        UpdateGroupCertificateConfigurationError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for UpdateGroupCertificateConfigurationError {
    fn from(err: CredentialsError) -> UpdateGroupCertificateConfigurationError {
        UpdateGroupCertificateConfigurationError::Credentials(err)
    }
}
impl From<HttpDispatchError> for UpdateGroupCertificateConfigurationError {
    fn from(err: HttpDispatchError) -> UpdateGroupCertificateConfigurationError {
        UpdateGroupCertificateConfigurationError::HttpDispatch(err)
    }
}
impl From<io::Error> for UpdateGroupCertificateConfigurationError {
    fn from(err: io::Error) -> UpdateGroupCertificateConfigurationError {
        UpdateGroupCertificateConfigurationError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for UpdateGroupCertificateConfigurationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for UpdateGroupCertificateConfigurationError {
    fn description(&self) -> &str {
        match *self {
            UpdateGroupCertificateConfigurationError::BadRequest(ref cause) => cause,
            UpdateGroupCertificateConfigurationError::InternalServerError(ref cause) => cause,
            UpdateGroupCertificateConfigurationError::Validation(ref cause) => cause,
            UpdateGroupCertificateConfigurationError::Credentials(ref err) => err.description(),
            UpdateGroupCertificateConfigurationError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            UpdateGroupCertificateConfigurationError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by UpdateLoggerDefinition
#[derive(Debug, PartialEq)]
pub enum UpdateLoggerDefinitionError {
    ///General Error
    BadRequest(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl UpdateLoggerDefinitionError {
    pub fn from_body(body: &str) -> UpdateLoggerDefinitionError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "BadRequestException" => {
                        UpdateLoggerDefinitionError::BadRequest(String::from(error_message))
                    }
                    "ValidationException" => {
                        UpdateLoggerDefinitionError::Validation(error_message.to_string())
                    }
                    _ => UpdateLoggerDefinitionError::Unknown(String::from(body)),
                }
            }
            Err(_) => UpdateLoggerDefinitionError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for UpdateLoggerDefinitionError {
    fn from(err: serde_json::error::Error) -> UpdateLoggerDefinitionError {
        UpdateLoggerDefinitionError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for UpdateLoggerDefinitionError {
    fn from(err: CredentialsError) -> UpdateLoggerDefinitionError {
        UpdateLoggerDefinitionError::Credentials(err)
    }
}
impl From<HttpDispatchError> for UpdateLoggerDefinitionError {
    fn from(err: HttpDispatchError) -> UpdateLoggerDefinitionError {
        UpdateLoggerDefinitionError::HttpDispatch(err)
    }
}
impl From<io::Error> for UpdateLoggerDefinitionError {
    fn from(err: io::Error) -> UpdateLoggerDefinitionError {
        UpdateLoggerDefinitionError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for UpdateLoggerDefinitionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for UpdateLoggerDefinitionError {
    fn description(&self) -> &str {
        match *self {
            UpdateLoggerDefinitionError::BadRequest(ref cause) => cause,
            UpdateLoggerDefinitionError::Validation(ref cause) => cause,
            UpdateLoggerDefinitionError::Credentials(ref err) => err.description(),
            UpdateLoggerDefinitionError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            UpdateLoggerDefinitionError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by UpdateSubscriptionDefinition
#[derive(Debug, PartialEq)]
pub enum UpdateSubscriptionDefinitionError {
    ///General Error
    BadRequest(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl UpdateSubscriptionDefinitionError {
    pub fn from_body(body: &str) -> UpdateSubscriptionDefinitionError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "BadRequestException" => {
                        UpdateSubscriptionDefinitionError::BadRequest(String::from(error_message))
                    }
                    "ValidationException" => {
                        UpdateSubscriptionDefinitionError::Validation(error_message.to_string())
                    }
                    _ => UpdateSubscriptionDefinitionError::Unknown(String::from(body)),
                }
            }
            Err(_) => UpdateSubscriptionDefinitionError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for UpdateSubscriptionDefinitionError {
    fn from(err: serde_json::error::Error) -> UpdateSubscriptionDefinitionError {
        UpdateSubscriptionDefinitionError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for UpdateSubscriptionDefinitionError {
    fn from(err: CredentialsError) -> UpdateSubscriptionDefinitionError {
        UpdateSubscriptionDefinitionError::Credentials(err)
    }
}
impl From<HttpDispatchError> for UpdateSubscriptionDefinitionError {
    fn from(err: HttpDispatchError) -> UpdateSubscriptionDefinitionError {
        UpdateSubscriptionDefinitionError::HttpDispatch(err)
    }
}
impl From<io::Error> for UpdateSubscriptionDefinitionError {
    fn from(err: io::Error) -> UpdateSubscriptionDefinitionError {
        UpdateSubscriptionDefinitionError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for UpdateSubscriptionDefinitionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for UpdateSubscriptionDefinitionError {
    fn description(&self) -> &str {
        match *self {
            UpdateSubscriptionDefinitionError::BadRequest(ref cause) => cause,
            UpdateSubscriptionDefinitionError::Validation(ref cause) => cause,
            UpdateSubscriptionDefinitionError::Credentials(ref err) => err.description(),
            UpdateSubscriptionDefinitionError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            UpdateSubscriptionDefinitionError::Unknown(ref cause) => cause,
        }
    }
}
/// Trait representing the capabilities of the AWS Greengrass API. AWS Greengrass clients implement this trait.
pub trait GreenGrass {
    #[doc="Associates a role with a group. The role will be used by the AWS Greengrass core in order to access AWS cloud services. The role's permissions will allow Greengrass core Lambda functions to perform actions against the cloud."]
    fn associate_role_to_group(
        &self,
        input: &AssociateRoleToGroupRequest,
    ) -> Result<AssociateRoleToGroupResponse, AssociateRoleToGroupError>;

    #[doc="Associates a role which is used by AWS Greengrass. AWS Greengrass uses the role to access your Lambda functions and AWS IoT resources. This is necessary for deployments to succeed. It needs to have minimum permissions in policy ``AWSGreengrassResourceAccessRolePolicy``"]
    fn associate_service_role_to_account(
        &self,
        input: &AssociateServiceRoleToAccountRequest,
    ) -> Result<AssociateServiceRoleToAccountResponse, AssociateServiceRoleToAccountError>;

    #[doc="Creates a core definition. You may optionally provide the initial version of the core definition or use ''CreateCoreDefinitionVersion'' at a later time. AWS Greengrass Groups must each contain exactly 1 AWS Greengrass Core."]
    fn create_core_definition(
        &self,
        input: &CreateCoreDefinitionRequest,
    ) -> Result<CreateCoreDefinitionResponse, CreateCoreDefinitionError>;

    #[doc="Creates a version of a core definition that has already been defined. AWS Greengrass Groups must each contain exactly 1 AWS Greengrass Core."]
    fn create_core_definition_version(
        &self,
        input: &CreateCoreDefinitionVersionRequest,
    ) -> Result<CreateCoreDefinitionVersionResponse, CreateCoreDefinitionVersionError>;

    #[doc = "Creates a deployment."]
    fn create_deployment(
        &self,
        input: &CreateDeploymentRequest,
    ) -> Result<CreateDeploymentResponse, CreateDeploymentError>;

    #[doc="Creates a device definition. You may optinally provide the initial version of the device definition or use ``CreateDeviceDefinitionVersion`` at a later time."]
    fn create_device_definition(
        &self,
        input: &CreateDeviceDefinitionRequest,
    ) -> Result<CreateDeviceDefinitionResponse, CreateDeviceDefinitionError>;

    #[doc = "Creates a version of a device definition that has already been defined."]
    fn create_device_definition_version(
        &self,
        input: &CreateDeviceDefinitionVersionRequest,
    ) -> Result<CreateDeviceDefinitionVersionResponse, CreateDeviceDefinitionVersionError>;

    #[doc="Creates a Lambda function definition which contains a list of Lambda functions and their configurations to be used in a group. You can create an initial version of the definition by providing a list of Lambda functions and their configurations now, or use ``CreateFunctionDefinitionVersion`` later."]
    fn create_function_definition(
        &self,
        input: &CreateFunctionDefinitionRequest,
    ) -> Result<CreateFunctionDefinitionResponse, CreateFunctionDefinitionError>;

    #[doc = "Create a version of a Lambda function definition that has already been defined."]
    fn create_function_definition_version(
        &self,
        input: &CreateFunctionDefinitionVersionRequest,
    ) -> Result<CreateFunctionDefinitionVersionResponse, CreateFunctionDefinitionVersionError>;

    #[doc="Creates a group. You may optionally provide the initial version of the group or use ''CreateGroupVersion'' at a later time."]
    fn create_group(
        &self,
        input: &CreateGroupRequest,
    ) -> Result<CreateGroupResponse, CreateGroupError>;

    #[doc = "Creates a CA for the group. If a CA already exists, it will rotate the existing CA."]
    fn create_group_certificate_authority(
        &self,
        input: &CreateGroupCertificateAuthorityRequest,
    ) -> Result<CreateGroupCertificateAuthorityResponse, CreateGroupCertificateAuthorityError>;

    #[doc = "Creates a version of a group which has already been defined."]
    fn create_group_version(
        &self,
        input: &CreateGroupVersionRequest,
    ) -> Result<CreateGroupVersionResponse, CreateGroupVersionError>;

    #[doc="Creates a logger definition. You may optionally provide the initial version of the logger definition or use ``CreateLoggerDefinitionVersion`` at a later time."]
    fn create_logger_definition(
        &self,
        input: &CreateLoggerDefinitionRequest,
    ) -> Result<CreateLoggerDefinitionResponse, CreateLoggerDefinitionError>;

    #[doc = "Creates a version of a logger definition that has already been defined."]
    fn create_logger_definition_version(
        &self,
        input: &CreateLoggerDefinitionVersionRequest,
    ) -> Result<CreateLoggerDefinitionVersionResponse, CreateLoggerDefinitionVersionError>;

    #[doc="Creates a subscription definition. You may optionally provide the initial version of the subscription definition or use ``CreateSubscriptionDefinitionVersion`` at a later time."]
    fn create_subscription_definition(
        &self,
        input: &CreateSubscriptionDefinitionRequest,
    ) -> Result<CreateSubscriptionDefinitionResponse, CreateSubscriptionDefinitionError>;

    #[doc = "Creates a version of a subscription definition which has already been defined."]
    fn create_subscription_definition_version(
        &self,
        input: &CreateSubscriptionDefinitionVersionRequest,
    ) -> Result<CreateSubscriptionDefinitionVersionResponse, CreateSubscriptionDefinitionVersionError>;

    #[doc="Deletes a core definition. The core definition must not have been used in a deployment."]
    fn delete_core_definition(
        &self,
        input: &DeleteCoreDefinitionRequest,
    ) -> Result<DeleteCoreDefinitionResponse, DeleteCoreDefinitionError>;

    #[doc="Deletes a device definition. The device definition must not have been used in a deployment."]
    fn delete_device_definition(
        &self,
        input: &DeleteDeviceDefinitionRequest,
    ) -> Result<DeleteDeviceDefinitionResponse, DeleteDeviceDefinitionError>;

    #[doc="Deletes a Lambda function definition. The Lambda function definition must not have been used in a deployment."]
    fn delete_function_definition(
        &self,
        input: &DeleteFunctionDefinitionRequest,
    ) -> Result<DeleteFunctionDefinitionResponse, DeleteFunctionDefinitionError>;

    #[doc = "Deletes a group. The group must not have been used in deployment."]
    fn delete_group(
        &self,
        input: &DeleteGroupRequest,
    ) -> Result<DeleteGroupResponse, DeleteGroupError>;

    #[doc="Deletes a logger definition. The logger definition must not have been used in a deployment."]
    fn delete_logger_definition(
        &self,
        input: &DeleteLoggerDefinitionRequest,
    ) -> Result<DeleteLoggerDefinitionResponse, DeleteLoggerDefinitionError>;

    #[doc="Deletes a subscription definition. The subscription definition must not have been used in a deployment."]
    fn delete_subscription_definition(
        &self,
        input: &DeleteSubscriptionDefinitionRequest,
    ) -> Result<DeleteSubscriptionDefinitionResponse, DeleteSubscriptionDefinitionError>;

    #[doc = "Disassociates the role from a group."]
    fn disassociate_role_from_group(
        &self,
        input: &DisassociateRoleFromGroupRequest,
    ) -> Result<DisassociateRoleFromGroupResponse, DisassociateRoleFromGroupError>;

    #[doc="Disassociates the service role from the account. Without a service role, deployments will not work."]
    fn disassociate_service_role_from_account(
        &self,
    ) -> Result<DisassociateServiceRoleFromAccountResponse, DisassociateServiceRoleFromAccountError>;

    #[doc = "Retrieves the role associated with a particular group."]
    fn get_associated_role(
        &self,
        input: &GetAssociatedRoleRequest,
    ) -> Result<GetAssociatedRoleResponse, GetAssociatedRoleError>;

    #[doc = "Retrieves the connectivity information for a core."]
    fn get_connectivity_info(
        &self,
        input: &GetConnectivityInfoRequest,
    ) -> Result<GetConnectivityInfoResponse, GetConnectivityInfoError>;

    #[doc = "Retrieves information about a core definition version."]
    fn get_core_definition(
        &self,
        input: &GetCoreDefinitionRequest,
    ) -> Result<GetCoreDefinitionResponse, GetCoreDefinitionError>;

    #[doc = "Retrieves information about a core definition version."]
    fn get_core_definition_version(
        &self,
        input: &GetCoreDefinitionVersionRequest,
    ) -> Result<GetCoreDefinitionVersionResponse, GetCoreDefinitionVersionError>;

    #[doc = "Returns the status of a deployment."]
    fn get_deployment_status(
        &self,
        input: &GetDeploymentStatusRequest,
    ) -> Result<GetDeploymentStatusResponse, GetDeploymentStatusError>;

    #[doc = "Retrieves information about a device definition."]
    fn get_device_definition(
        &self,
        input: &GetDeviceDefinitionRequest,
    ) -> Result<GetDeviceDefinitionResponse, GetDeviceDefinitionError>;

    #[doc = "Retrieves information about a device definition version."]
    fn get_device_definition_version(
        &self,
        input: &GetDeviceDefinitionVersionRequest,
    ) -> Result<GetDeviceDefinitionVersionResponse, GetDeviceDefinitionVersionError>;

    #[doc="Retrieves information about a Lambda function definition, such as its creation time and latest version."]
    fn get_function_definition(
        &self,
        input: &GetFunctionDefinitionRequest,
    ) -> Result<GetFunctionDefinitionResponse, GetFunctionDefinitionError>;

    #[doc="Retrieves information about a Lambda function definition version, such as which Lambda functions are included in the version and their configurations."]
    fn get_function_definition_version(
        &self,
        input: &GetFunctionDefinitionVersionRequest,
    ) -> Result<GetFunctionDefinitionVersionResponse, GetFunctionDefinitionVersionError>;

    #[doc = "Retrieves information about a group."]
    fn get_group(&self, input: &GetGroupRequest) -> Result<GetGroupResponse, GetGroupError>;

    #[doc = "Retreives the CA associated with a group. Returns the public key of the CA."]
    fn get_group_certificate_authority(
        &self,
        input: &GetGroupCertificateAuthorityRequest,
    ) -> Result<GetGroupCertificateAuthorityResponse, GetGroupCertificateAuthorityError>;

    #[doc = "Retrieves the current configuration for the CA used by the group."]
    fn get_group_certificate_configuration(
        &self,
        input: &GetGroupCertificateConfigurationRequest,
    ) -> Result<GetGroupCertificateConfigurationResponse, GetGroupCertificateConfigurationError>;

    #[doc = "Retrieves information about a group version."]
    fn get_group_version(
        &self,
        input: &GetGroupVersionRequest,
    ) -> Result<GetGroupVersionResponse, GetGroupVersionError>;

    #[doc = "Retrieves information about a logger definition."]
    fn get_logger_definition(
        &self,
        input: &GetLoggerDefinitionRequest,
    ) -> Result<GetLoggerDefinitionResponse, GetLoggerDefinitionError>;

    #[doc = "Retrieves information about a logger definition version."]
    fn get_logger_definition_version(
        &self,
        input: &GetLoggerDefinitionVersionRequest,
    ) -> Result<GetLoggerDefinitionVersionResponse, GetLoggerDefinitionVersionError>;

    #[doc = "Retrieves the service role that is attached to the account."]
    fn get_service_role_for_account(
        &self,
    ) -> Result<GetServiceRoleForAccountResponse, GetServiceRoleForAccountError>;

    #[doc = "Retrieves information about a subscription definition."]
    fn get_subscription_definition(
        &self,
        input: &GetSubscriptionDefinitionRequest,
    ) -> Result<GetSubscriptionDefinitionResponse, GetSubscriptionDefinitionError>;

    #[doc = "Retrieves information about a subscription definition version."]
    fn get_subscription_definition_version(
        &self,
        input: &GetSubscriptionDefinitionVersionRequest,
    ) -> Result<GetSubscriptionDefinitionVersionResponse, GetSubscriptionDefinitionVersionError>;

    #[doc = "Lists versions of a core definition."]
    fn list_core_definition_versions(
        &self,
        input: &ListCoreDefinitionVersionsRequest,
    ) -> Result<ListCoreDefinitionVersionsResponse, ListCoreDefinitionVersionsError>;

    #[doc = "Retrieves a list of core definitions."]
    fn list_core_definitions(
        &self,
        input: &ListCoreDefinitionsRequest,
    ) -> Result<ListCoreDefinitionsResponse, ListCoreDefinitionsError>;

    #[doc = "Returns a history of deployments for the group."]
    fn list_deployments(
        &self,
        input: &ListDeploymentsRequest,
    ) -> Result<ListDeploymentsResponse, ListDeploymentsError>;

    #[doc = "Lists the versions of a device definition."]
    fn list_device_definition_versions(
        &self,
        input: &ListDeviceDefinitionVersionsRequest,
    ) -> Result<ListDeviceDefinitionVersionsResponse, ListDeviceDefinitionVersionsError>;

    #[doc = "Retrieves a list of device definitions."]
    fn list_device_definitions(
        &self,
        input: &ListDeviceDefinitionsRequest,
    ) -> Result<ListDeviceDefinitionsResponse, ListDeviceDefinitionsError>;

    #[doc = "Lists the versions of a Lambda function definition."]
    fn list_function_definition_versions(
        &self,
        input: &ListFunctionDefinitionVersionsRequest,
    ) -> Result<ListFunctionDefinitionVersionsResponse, ListFunctionDefinitionVersionsError>;

    #[doc = "Retrieves a list of Lambda function definitions."]
    fn list_function_definitions(
        &self,
        input: &ListFunctionDefinitionsRequest,
    ) -> Result<ListFunctionDefinitionsResponse, ListFunctionDefinitionsError>;

    #[doc = "Retrieves the current CAs for a group."]
    fn list_group_certificate_authorities(
        &self,
        input: &ListGroupCertificateAuthoritiesRequest,
    ) -> Result<ListGroupCertificateAuthoritiesResponse, ListGroupCertificateAuthoritiesError>;

    #[doc = "List the versions of a group."]
    fn list_group_versions(
        &self,
        input: &ListGroupVersionsRequest,
    ) -> Result<ListGroupVersionsResponse, ListGroupVersionsError>;

    #[doc = "Retrieves a list of groups."]
    fn list_groups(&self, input: &ListGroupsRequest)
        -> Result<ListGroupsResponse, ListGroupsError>;

    #[doc = "Lists the versions of a logger definition."]
    fn list_logger_definition_versions(
        &self,
        input: &ListLoggerDefinitionVersionsRequest,
    ) -> Result<ListLoggerDefinitionVersionsResponse, ListLoggerDefinitionVersionsError>;

    #[doc = "Retrieves a list of logger definitions."]
    fn list_logger_definitions(
        &self,
        input: &ListLoggerDefinitionsRequest,
    ) -> Result<ListLoggerDefinitionsResponse, ListLoggerDefinitionsError>;

    #[doc = "Lists the versions of a subscription definition."]
    fn list_subscription_definition_versions(
        &self,
        input: &ListSubscriptionDefinitionVersionsRequest,
    ) -> Result<ListSubscriptionDefinitionVersionsResponse, ListSubscriptionDefinitionVersionsError>;

    #[doc = "Retrieves a list of subscription definitions."]
    fn list_subscription_definitions(
        &self,
        input: &ListSubscriptionDefinitionsRequest,
    ) -> Result<ListSubscriptionDefinitionsResponse, ListSubscriptionDefinitionsError>;

    #[doc="Updates the connectivity information for the core. Any devices that belong to the group which has this core will receive this information in order to find the location of the core and connect to it."]
    fn update_connectivity_info(
        &self,
        input: &UpdateConnectivityInfoRequest,
    ) -> Result<UpdateConnectivityInfoResponse, UpdateConnectivityInfoError>;

    #[doc = "Updates a core definition."]
    fn update_core_definition(
        &self,
        input: &UpdateCoreDefinitionRequest,
    ) -> Result<UpdateCoreDefinitionResponse, UpdateCoreDefinitionError>;

    #[doc = "Updates a device definition."]
    fn update_device_definition(
        &self,
        input: &UpdateDeviceDefinitionRequest,
    ) -> Result<UpdateDeviceDefinitionResponse, UpdateDeviceDefinitionError>;

    #[doc = "Updates a Lambda function definition."]
    fn update_function_definition(
        &self,
        input: &UpdateFunctionDefinitionRequest,
    ) -> Result<UpdateFunctionDefinitionResponse, UpdateFunctionDefinitionError>;

    #[doc = "Updates a group."]
    fn update_group(
        &self,
        input: &UpdateGroupRequest,
    ) -> Result<UpdateGroupResponse, UpdateGroupError>;

    #[doc = "Updates the Cert expiry time for a group."]
    fn update_group_certificate_configuration(
        &self,
        input: &UpdateGroupCertificateConfigurationRequest,
    ) -> Result<UpdateGroupCertificateConfigurationResponse, UpdateGroupCertificateConfigurationError>;

    #[doc = "Updates a logger definition."]
    fn update_logger_definition(
        &self,
        input: &UpdateLoggerDefinitionRequest,
    ) -> Result<UpdateLoggerDefinitionResponse, UpdateLoggerDefinitionError>;

    #[doc = "Updates a subscription definition."]
    fn update_subscription_definition(
        &self,
        input: &UpdateSubscriptionDefinitionRequest,
    ) -> Result<UpdateSubscriptionDefinitionResponse, UpdateSubscriptionDefinitionError>;
}
/// A client for the AWS Greengrass API.
pub struct GreenGrassClient<P, D>
where
    P: ProvideAwsCredentials,
    D: DispatchSignedRequest,
{
    credentials_provider: P,
    region: region::Region,
    dispatcher: D,
}

impl<P, D> GreenGrassClient<P, D>
where
    P: ProvideAwsCredentials,
    D: DispatchSignedRequest,
{
    pub fn new(request_dispatcher: D, credentials_provider: P, region: region::Region) -> Self {
        GreenGrassClient {
            credentials_provider: credentials_provider,
            region: region,
            dispatcher: request_dispatcher,
        }
    }
}

impl<P, D> GreenGrass for GreenGrassClient<P, D>
where
    P: ProvideAwsCredentials,
    D: DispatchSignedRequest,
{
    #[doc="Associates a role with a group. The role will be used by the AWS Greengrass core in order to access AWS cloud services. The role's permissions will allow Greengrass core Lambda functions to perform actions against the cloud."]
    fn associate_role_to_group(
        &self,
        input: &AssociateRoleToGroupRequest,
    ) -> Result<AssociateRoleToGroupResponse, AssociateRoleToGroupError> {
        let request_uri = format!(
            "/greengrass/groups/{group_id}/role",
            group_id = input.group_id
        );

        let mut request = SignedRequest::new("PUT", "greengrass", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(input).unwrap());
        request.set_payload(encoded);

        request.sign_with_plus(&self.credentials_provider.credentials()?, true);
        let mut response = self.dispatcher.dispatch(&request)?;

        match response.status {
            StatusCode::Ok => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));

                if body == b"{}" {
                    body = b"null".to_vec();
                }

                debug!("Response body: {:?}", body);
                debug!("Response status: {}", response.status);
                let result = serde_json::from_slice::<AssociateRoleToGroupResponse>(&body).unwrap();

                Ok(result)
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(AssociateRoleToGroupError::from_body(
                    String::from_utf8_lossy(&body).as_ref(),
                ))
            }
        }
    }

    #[doc="Associates a role which is used by AWS Greengrass. AWS Greengrass uses the role to access your Lambda functions and AWS IoT resources. This is necessary for deployments to succeed. It needs to have minimum permissions in policy ``AWSGreengrassResourceAccessRolePolicy``"]
    fn associate_service_role_to_account(
        &self,
        input: &AssociateServiceRoleToAccountRequest,
    ) -> Result<AssociateServiceRoleToAccountResponse, AssociateServiceRoleToAccountError> {
        let request_uri = "/greengrass/servicerole";

        let mut request = SignedRequest::new("PUT", "greengrass", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(input).unwrap());
        request.set_payload(encoded);

        request.sign_with_plus(&self.credentials_provider.credentials()?, true);
        let mut response = self.dispatcher.dispatch(&request)?;

        match response.status {
            StatusCode::Ok => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));

                if body == b"{}" {
                    body = b"null".to_vec();
                }

                debug!("Response body: {:?}", body);
                debug!("Response status: {}", response.status);
                let result =
                    serde_json::from_slice::<AssociateServiceRoleToAccountResponse>(&body).unwrap();

                Ok(result)
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(AssociateServiceRoleToAccountError::from_body(
                    String::from_utf8_lossy(&body).as_ref(),
                ))
            }
        }
    }

    #[doc="Creates a core definition. You may optionally provide the initial version of the core definition or use ''CreateCoreDefinitionVersion'' at a later time. AWS Greengrass Groups must each contain exactly 1 AWS Greengrass Core."]
    fn create_core_definition(
        &self,
        input: &CreateCoreDefinitionRequest,
    ) -> Result<CreateCoreDefinitionResponse, CreateCoreDefinitionError> {
        let request_uri = "/greengrass/definition/cores";

        let mut request = SignedRequest::new("POST", "greengrass", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(input).unwrap());
        request.set_payload(encoded);

        if let Some(ref amzn_client_token) = input.amzn_client_token {
            request.add_header("X-Amzn-Client-Token", &amzn_client_token.to_string());
        }

        request.sign_with_plus(&self.credentials_provider.credentials()?, true);
        let mut response = self.dispatcher.dispatch(&request)?;

        match response.status {
            StatusCode::Ok => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));

                if body == b"{}" {
                    body = b"null".to_vec();
                }

                debug!("Response body: {:?}", body);
                debug!("Response status: {}", response.status);
                let result = serde_json::from_slice::<CreateCoreDefinitionResponse>(&body).unwrap();

                Ok(result)
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(CreateCoreDefinitionError::from_body(
                    String::from_utf8_lossy(&body).as_ref(),
                ))
            }
        }
    }

    #[doc="Creates a version of a core definition that has already been defined. AWS Greengrass Groups must each contain exactly 1 AWS Greengrass Core."]
    fn create_core_definition_version(
        &self,
        input: &CreateCoreDefinitionVersionRequest,
    ) -> Result<CreateCoreDefinitionVersionResponse, CreateCoreDefinitionVersionError> {
        let request_uri = format!(
            "/greengrass/definition/cores/{core_definition_id}/versions",
            core_definition_id = input.core_definition_id
        );

        let mut request = SignedRequest::new("POST", "greengrass", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(input).unwrap());
        request.set_payload(encoded);

        if let Some(ref amzn_client_token) = input.amzn_client_token {
            request.add_header("X-Amzn-Client-Token", &amzn_client_token.to_string());
        }

        request.sign_with_plus(&self.credentials_provider.credentials()?, true);
        let mut response = self.dispatcher.dispatch(&request)?;

        match response.status {
            StatusCode::Ok => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));

                if body == b"{}" {
                    body = b"null".to_vec();
                }

                debug!("Response body: {:?}", body);
                debug!("Response status: {}", response.status);
                let result =
                    serde_json::from_slice::<CreateCoreDefinitionVersionResponse>(&body).unwrap();

                Ok(result)
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(CreateCoreDefinitionVersionError::from_body(
                    String::from_utf8_lossy(&body).as_ref(),
                ))
            }
        }
    }

    #[doc = "Creates a deployment."]
    fn create_deployment(
        &self,
        input: &CreateDeploymentRequest,
    ) -> Result<CreateDeploymentResponse, CreateDeploymentError> {
        let request_uri = format!(
            "/greengrass/groups/{group_id}/deployments",
            group_id = input.group_id
        );

        let mut request = SignedRequest::new("POST", "greengrass", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(input).unwrap());
        request.set_payload(encoded);

        if let Some(ref amzn_client_token) = input.amzn_client_token {
            request.add_header("X-Amzn-Client-Token", &amzn_client_token.to_string());
        }

        request.sign_with_plus(&self.credentials_provider.credentials()?, true);
        let mut response = self.dispatcher.dispatch(&request)?;

        match response.status {
            StatusCode::Ok => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));

                if body == b"{}" {
                    body = b"null".to_vec();
                }

                debug!("Response body: {:?}", body);
                debug!("Response status: {}", response.status);
                let result = serde_json::from_slice::<CreateDeploymentResponse>(&body).unwrap();

                Ok(result)
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(CreateDeploymentError::from_body(
                    String::from_utf8_lossy(&body).as_ref(),
                ))
            }
        }
    }

    #[doc="Creates a device definition. You may optinally provide the initial version of the device definition or use ``CreateDeviceDefinitionVersion`` at a later time."]
    fn create_device_definition(
        &self,
        input: &CreateDeviceDefinitionRequest,
    ) -> Result<CreateDeviceDefinitionResponse, CreateDeviceDefinitionError> {
        let request_uri = "/greengrass/definition/devices";

        let mut request = SignedRequest::new("POST", "greengrass", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(input).unwrap());
        request.set_payload(encoded);

        if let Some(ref amzn_client_token) = input.amzn_client_token {
            request.add_header("X-Amzn-Client-Token", &amzn_client_token.to_string());
        }

        request.sign_with_plus(&self.credentials_provider.credentials()?, true);
        let mut response = self.dispatcher.dispatch(&request)?;

        match response.status {
            StatusCode::Ok => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));

                if body == b"{}" {
                    body = b"null".to_vec();
                }

                debug!("Response body: {:?}", body);
                debug!("Response status: {}", response.status);
                let result =
                    serde_json::from_slice::<CreateDeviceDefinitionResponse>(&body).unwrap();

                Ok(result)
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(CreateDeviceDefinitionError::from_body(
                    String::from_utf8_lossy(&body).as_ref(),
                ))
            }
        }
    }

    #[doc = "Creates a version of a device definition that has already been defined."]
    fn create_device_definition_version(
        &self,
        input: &CreateDeviceDefinitionVersionRequest,
    ) -> Result<CreateDeviceDefinitionVersionResponse, CreateDeviceDefinitionVersionError> {
        let request_uri = format!(
            "/greengrass/definition/devices/{device_definition_id}/versions",
            device_definition_id = input.device_definition_id
        );

        let mut request = SignedRequest::new("POST", "greengrass", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(input).unwrap());
        request.set_payload(encoded);

        if let Some(ref amzn_client_token) = input.amzn_client_token {
            request.add_header("X-Amzn-Client-Token", &amzn_client_token.to_string());
        }

        request.sign_with_plus(&self.credentials_provider.credentials()?, true);
        let mut response = self.dispatcher.dispatch(&request)?;

        match response.status {
            StatusCode::Ok => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));

                if body == b"{}" {
                    body = b"null".to_vec();
                }

                debug!("Response body: {:?}", body);
                debug!("Response status: {}", response.status);
                let result =
                    serde_json::from_slice::<CreateDeviceDefinitionVersionResponse>(&body).unwrap();

                Ok(result)
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(CreateDeviceDefinitionVersionError::from_body(
                    String::from_utf8_lossy(&body).as_ref(),
                ))
            }
        }
    }

    #[doc="Creates a Lambda function definition which contains a list of Lambda functions and their configurations to be used in a group. You can create an initial version of the definition by providing a list of Lambda functions and their configurations now, or use ``CreateFunctionDefinitionVersion`` later."]
    fn create_function_definition(
        &self,
        input: &CreateFunctionDefinitionRequest,
    ) -> Result<CreateFunctionDefinitionResponse, CreateFunctionDefinitionError> {
        let request_uri = "/greengrass/definition/functions";

        let mut request = SignedRequest::new("POST", "greengrass", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(input).unwrap());
        request.set_payload(encoded);

        if let Some(ref amzn_client_token) = input.amzn_client_token {
            request.add_header("X-Amzn-Client-Token", &amzn_client_token.to_string());
        }

        request.sign_with_plus(&self.credentials_provider.credentials()?, true);
        let mut response = self.dispatcher.dispatch(&request)?;

        match response.status {
            StatusCode::Ok => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));

                if body == b"{}" {
                    body = b"null".to_vec();
                }

                debug!("Response body: {:?}", body);
                debug!("Response status: {}", response.status);
                let result =
                    serde_json::from_slice::<CreateFunctionDefinitionResponse>(&body).unwrap();

                Ok(result)
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(CreateFunctionDefinitionError::from_body(
                    String::from_utf8_lossy(&body).as_ref(),
                ))
            }
        }
    }

    #[doc = "Create a version of a Lambda function definition that has already been defined."]
    fn create_function_definition_version(
        &self,
        input: &CreateFunctionDefinitionVersionRequest,
    ) -> Result<CreateFunctionDefinitionVersionResponse, CreateFunctionDefinitionVersionError> {
        let request_uri = format!(
            "/greengrass/definition/functions/{function_definition_id}/versions",
            function_definition_id = input.function_definition_id
        );

        let mut request = SignedRequest::new("POST", "greengrass", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(input).unwrap());
        request.set_payload(encoded);

        if let Some(ref amzn_client_token) = input.amzn_client_token {
            request.add_header("X-Amzn-Client-Token", &amzn_client_token.to_string());
        }

        request.sign_with_plus(&self.credentials_provider.credentials()?, true);
        let mut response = self.dispatcher.dispatch(&request)?;

        match response.status {
            StatusCode::Ok => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));

                if body == b"{}" {
                    body = b"null".to_vec();
                }

                debug!("Response body: {:?}", body);
                debug!("Response status: {}", response.status);
                let result = serde_json::from_slice::<CreateFunctionDefinitionVersionResponse>(
                    &body,
                ).unwrap();

                Ok(result)
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(CreateFunctionDefinitionVersionError::from_body(
                    String::from_utf8_lossy(&body).as_ref(),
                ))
            }
        }
    }

    #[doc="Creates a group. You may optionally provide the initial version of the group or use ''CreateGroupVersion'' at a later time."]
    fn create_group(
        &self,
        input: &CreateGroupRequest,
    ) -> Result<CreateGroupResponse, CreateGroupError> {
        let request_uri = "/greengrass/groups";

        let mut request = SignedRequest::new("POST", "greengrass", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(input).unwrap());
        request.set_payload(encoded);

        if let Some(ref amzn_client_token) = input.amzn_client_token {
            request.add_header("X-Amzn-Client-Token", &amzn_client_token.to_string());
        }

        request.sign_with_plus(&self.credentials_provider.credentials()?, true);
        let mut response = self.dispatcher.dispatch(&request)?;

        match response.status {
            StatusCode::Ok => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));

                if body == b"{}" {
                    body = b"null".to_vec();
                }

                debug!("Response body: {:?}", body);
                debug!("Response status: {}", response.status);
                let result = serde_json::from_slice::<CreateGroupResponse>(&body).unwrap();

                Ok(result)
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(CreateGroupError::from_body(
                    String::from_utf8_lossy(&body).as_ref(),
                ))
            }
        }
    }

    #[doc = "Creates a CA for the group. If a CA already exists, it will rotate the existing CA."]
    fn create_group_certificate_authority(
        &self,
        input: &CreateGroupCertificateAuthorityRequest,
    ) -> Result<CreateGroupCertificateAuthorityResponse, CreateGroupCertificateAuthorityError> {
        let request_uri = format!(
            "/greengrass/groups/{group_id}/certificateauthorities",
            group_id = input.group_id
        );

        let mut request = SignedRequest::new("POST", "greengrass", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        if let Some(ref amzn_client_token) = input.amzn_client_token {
            request.add_header("X-Amzn-Client-Token", &amzn_client_token.to_string());
        }

        request.sign_with_plus(&self.credentials_provider.credentials()?, true);
        let mut response = self.dispatcher.dispatch(&request)?;

        match response.status {
            StatusCode::Ok => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));

                if body == b"{}" {
                    body = b"null".to_vec();
                }

                debug!("Response body: {:?}", body);
                debug!("Response status: {}", response.status);
                let result = serde_json::from_slice::<CreateGroupCertificateAuthorityResponse>(
                    &body,
                ).unwrap();

                Ok(result)
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(CreateGroupCertificateAuthorityError::from_body(
                    String::from_utf8_lossy(&body).as_ref(),
                ))
            }
        }
    }

    #[doc = "Creates a version of a group which has already been defined."]
    fn create_group_version(
        &self,
        input: &CreateGroupVersionRequest,
    ) -> Result<CreateGroupVersionResponse, CreateGroupVersionError> {
        let request_uri = format!(
            "/greengrass/groups/{group_id}/versions",
            group_id = input.group_id
        );

        let mut request = SignedRequest::new("POST", "greengrass", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(input).unwrap());
        request.set_payload(encoded);

        if let Some(ref amzn_client_token) = input.amzn_client_token {
            request.add_header("X-Amzn-Client-Token", &amzn_client_token.to_string());
        }

        request.sign_with_plus(&self.credentials_provider.credentials()?, true);
        let mut response = self.dispatcher.dispatch(&request)?;

        match response.status {
            StatusCode::Ok => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));

                if body == b"{}" {
                    body = b"null".to_vec();
                }

                debug!("Response body: {:?}", body);
                debug!("Response status: {}", response.status);
                let result = serde_json::from_slice::<CreateGroupVersionResponse>(&body).unwrap();

                Ok(result)
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(CreateGroupVersionError::from_body(
                    String::from_utf8_lossy(&body).as_ref(),
                ))
            }
        }
    }

    #[doc="Creates a logger definition. You may optionally provide the initial version of the logger definition or use ``CreateLoggerDefinitionVersion`` at a later time."]
    fn create_logger_definition(
        &self,
        input: &CreateLoggerDefinitionRequest,
    ) -> Result<CreateLoggerDefinitionResponse, CreateLoggerDefinitionError> {
        let request_uri = "/greengrass/definition/loggers";

        let mut request = SignedRequest::new("POST", "greengrass", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(input).unwrap());
        request.set_payload(encoded);

        if let Some(ref amzn_client_token) = input.amzn_client_token {
            request.add_header("X-Amzn-Client-Token", &amzn_client_token.to_string());
        }

        request.sign_with_plus(&self.credentials_provider.credentials()?, true);
        let mut response = self.dispatcher.dispatch(&request)?;

        match response.status {
            StatusCode::Ok => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));

                if body == b"{}" {
                    body = b"null".to_vec();
                }

                debug!("Response body: {:?}", body);
                debug!("Response status: {}", response.status);
                let result =
                    serde_json::from_slice::<CreateLoggerDefinitionResponse>(&body).unwrap();

                Ok(result)
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(CreateLoggerDefinitionError::from_body(
                    String::from_utf8_lossy(&body).as_ref(),
                ))
            }
        }
    }

    #[doc = "Creates a version of a logger definition that has already been defined."]
    fn create_logger_definition_version(
        &self,
        input: &CreateLoggerDefinitionVersionRequest,
    ) -> Result<CreateLoggerDefinitionVersionResponse, CreateLoggerDefinitionVersionError> {
        let request_uri = format!(
            "/greengrass/definition/loggers/{logger_definition_id}/versions",
            logger_definition_id = input.logger_definition_id
        );

        let mut request = SignedRequest::new("POST", "greengrass", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(input).unwrap());
        request.set_payload(encoded);

        if let Some(ref amzn_client_token) = input.amzn_client_token {
            request.add_header("X-Amzn-Client-Token", &amzn_client_token.to_string());
        }

        request.sign_with_plus(&self.credentials_provider.credentials()?, true);
        let mut response = self.dispatcher.dispatch(&request)?;

        match response.status {
            StatusCode::Ok => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));

                if body == b"{}" {
                    body = b"null".to_vec();
                }

                debug!("Response body: {:?}", body);
                debug!("Response status: {}", response.status);
                let result =
                    serde_json::from_slice::<CreateLoggerDefinitionVersionResponse>(&body).unwrap();

                Ok(result)
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(CreateLoggerDefinitionVersionError::from_body(
                    String::from_utf8_lossy(&body).as_ref(),
                ))
            }
        }
    }

    #[doc="Creates a subscription definition. You may optionally provide the initial version of the subscription definition or use ``CreateSubscriptionDefinitionVersion`` at a later time."]
    fn create_subscription_definition(
        &self,
        input: &CreateSubscriptionDefinitionRequest,
    ) -> Result<CreateSubscriptionDefinitionResponse, CreateSubscriptionDefinitionError> {
        let request_uri = "/greengrass/definition/subscriptions";

        let mut request = SignedRequest::new("POST", "greengrass", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(input).unwrap());
        request.set_payload(encoded);

        if let Some(ref amzn_client_token) = input.amzn_client_token {
            request.add_header("X-Amzn-Client-Token", &amzn_client_token.to_string());
        }

        request.sign_with_plus(&self.credentials_provider.credentials()?, true);
        let mut response = self.dispatcher.dispatch(&request)?;

        match response.status {
            StatusCode::Ok => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));

                if body == b"{}" {
                    body = b"null".to_vec();
                }

                debug!("Response body: {:?}", body);
                debug!("Response status: {}", response.status);
                let result =
                    serde_json::from_slice::<CreateSubscriptionDefinitionResponse>(&body).unwrap();

                Ok(result)
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(CreateSubscriptionDefinitionError::from_body(
                    String::from_utf8_lossy(&body).as_ref(),
                ))
            }
        }
    }

    #[doc = "Creates a version of a subscription definition which has already been defined."]
    fn create_subscription_definition_version(
        &self,
        input: &CreateSubscriptionDefinitionVersionRequest,
    ) -> Result<CreateSubscriptionDefinitionVersionResponse, CreateSubscriptionDefinitionVersionError>
    {
        let request_uri = format!(
            "/greengrass/definition/subscriptions/{subscription_definition_id}/versions",
            subscription_definition_id = input.subscription_definition_id
        );

        let mut request = SignedRequest::new("POST", "greengrass", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(input).unwrap());
        request.set_payload(encoded);

        if let Some(ref amzn_client_token) = input.amzn_client_token {
            request.add_header("X-Amzn-Client-Token", &amzn_client_token.to_string());
        }

        request.sign_with_plus(&self.credentials_provider.credentials()?, true);
        let mut response = self.dispatcher.dispatch(&request)?;

        match response.status {
            StatusCode::Ok => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));

                if body == b"{}" {
                    body = b"null".to_vec();
                }

                debug!("Response body: {:?}", body);
                debug!("Response status: {}", response.status);
                let result =
                    serde_json::from_slice::<CreateSubscriptionDefinitionVersionResponse>(&body)
                        .unwrap();

                Ok(result)
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(CreateSubscriptionDefinitionVersionError::from_body(
                    String::from_utf8_lossy(&body).as_ref(),
                ))
            }
        }
    }

    #[doc="Deletes a core definition. The core definition must not have been used in a deployment."]
    fn delete_core_definition(
        &self,
        input: &DeleteCoreDefinitionRequest,
    ) -> Result<DeleteCoreDefinitionResponse, DeleteCoreDefinitionError> {
        let request_uri = format!(
            "/greengrass/definition/cores/{core_definition_id}",
            core_definition_id = input.core_definition_id
        );

        let mut request = SignedRequest::new("DELETE", "greengrass", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.sign_with_plus(&self.credentials_provider.credentials()?, true);
        let mut response = self.dispatcher.dispatch(&request)?;

        match response.status {
            StatusCode::Ok => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));

                if body == b"{}" {
                    body = b"null".to_vec();
                }

                debug!("Response body: {:?}", body);
                debug!("Response status: {}", response.status);
                let result = serde_json::from_slice::<DeleteCoreDefinitionResponse>(&body).unwrap();

                Ok(result)
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(DeleteCoreDefinitionError::from_body(
                    String::from_utf8_lossy(&body).as_ref(),
                ))
            }
        }
    }

    #[doc="Deletes a device definition. The device definition must not have been used in a deployment."]
    fn delete_device_definition(
        &self,
        input: &DeleteDeviceDefinitionRequest,
    ) -> Result<DeleteDeviceDefinitionResponse, DeleteDeviceDefinitionError> {
        let request_uri = format!(
            "/greengrass/definition/devices/{device_definition_id}",
            device_definition_id = input.device_definition_id
        );

        let mut request = SignedRequest::new("DELETE", "greengrass", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.sign_with_plus(&self.credentials_provider.credentials()?, true);
        let mut response = self.dispatcher.dispatch(&request)?;

        match response.status {
            StatusCode::Ok => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));

                if body == b"{}" {
                    body = b"null".to_vec();
                }

                debug!("Response body: {:?}", body);
                debug!("Response status: {}", response.status);
                let result =
                    serde_json::from_slice::<DeleteDeviceDefinitionResponse>(&body).unwrap();

                Ok(result)
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(DeleteDeviceDefinitionError::from_body(
                    String::from_utf8_lossy(&body).as_ref(),
                ))
            }
        }
    }

    #[doc="Deletes a Lambda function definition. The Lambda function definition must not have been used in a deployment."]
    fn delete_function_definition(
        &self,
        input: &DeleteFunctionDefinitionRequest,
    ) -> Result<DeleteFunctionDefinitionResponse, DeleteFunctionDefinitionError> {
        let request_uri = format!(
            "/greengrass/definition/functions/{function_definition_id}",
            function_definition_id = input.function_definition_id
        );

        let mut request = SignedRequest::new("DELETE", "greengrass", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.sign_with_plus(&self.credentials_provider.credentials()?, true);
        let mut response = self.dispatcher.dispatch(&request)?;

        match response.status {
            StatusCode::Ok => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));

                if body == b"{}" {
                    body = b"null".to_vec();
                }

                debug!("Response body: {:?}", body);
                debug!("Response status: {}", response.status);
                let result =
                    serde_json::from_slice::<DeleteFunctionDefinitionResponse>(&body).unwrap();

                Ok(result)
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(DeleteFunctionDefinitionError::from_body(
                    String::from_utf8_lossy(&body).as_ref(),
                ))
            }
        }
    }

    #[doc = "Deletes a group. The group must not have been used in deployment."]
    fn delete_group(
        &self,
        input: &DeleteGroupRequest,
    ) -> Result<DeleteGroupResponse, DeleteGroupError> {
        let request_uri = format!("/greengrass/groups/{group_id}", group_id = input.group_id);

        let mut request = SignedRequest::new("DELETE", "greengrass", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.sign_with_plus(&self.credentials_provider.credentials()?, true);
        let mut response = self.dispatcher.dispatch(&request)?;

        match response.status {
            StatusCode::Ok => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));

                if body == b"{}" {
                    body = b"null".to_vec();
                }

                debug!("Response body: {:?}", body);
                debug!("Response status: {}", response.status);
                let result = serde_json::from_slice::<DeleteGroupResponse>(&body).unwrap();

                Ok(result)
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(DeleteGroupError::from_body(
                    String::from_utf8_lossy(&body).as_ref(),
                ))
            }
        }
    }

    #[doc="Deletes a logger definition. The logger definition must not have been used in a deployment."]
    fn delete_logger_definition(
        &self,
        input: &DeleteLoggerDefinitionRequest,
    ) -> Result<DeleteLoggerDefinitionResponse, DeleteLoggerDefinitionError> {
        let request_uri = format!(
            "/greengrass/definition/loggers/{logger_definition_id}",
            logger_definition_id = input.logger_definition_id
        );

        let mut request = SignedRequest::new("DELETE", "greengrass", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.sign_with_plus(&self.credentials_provider.credentials()?, true);
        let mut response = self.dispatcher.dispatch(&request)?;

        match response.status {
            StatusCode::Ok => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));

                if body == b"{}" {
                    body = b"null".to_vec();
                }

                debug!("Response body: {:?}", body);
                debug!("Response status: {}", response.status);
                let result =
                    serde_json::from_slice::<DeleteLoggerDefinitionResponse>(&body).unwrap();

                Ok(result)
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(DeleteLoggerDefinitionError::from_body(
                    String::from_utf8_lossy(&body).as_ref(),
                ))
            }
        }
    }

    #[doc="Deletes a subscription definition. The subscription definition must not have been used in a deployment."]
    fn delete_subscription_definition(
        &self,
        input: &DeleteSubscriptionDefinitionRequest,
    ) -> Result<DeleteSubscriptionDefinitionResponse, DeleteSubscriptionDefinitionError> {
        let request_uri = format!(
            "/greengrass/definition/subscriptions/{subscription_definition_id}",
            subscription_definition_id = input.subscription_definition_id
        );

        let mut request = SignedRequest::new("DELETE", "greengrass", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.sign_with_plus(&self.credentials_provider.credentials()?, true);
        let mut response = self.dispatcher.dispatch(&request)?;

        match response.status {
            StatusCode::Ok => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));

                if body == b"{}" {
                    body = b"null".to_vec();
                }

                debug!("Response body: {:?}", body);
                debug!("Response status: {}", response.status);
                let result =
                    serde_json::from_slice::<DeleteSubscriptionDefinitionResponse>(&body).unwrap();

                Ok(result)
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(DeleteSubscriptionDefinitionError::from_body(
                    String::from_utf8_lossy(&body).as_ref(),
                ))
            }
        }
    }

    #[doc = "Disassociates the role from a group."]
    fn disassociate_role_from_group(
        &self,
        input: &DisassociateRoleFromGroupRequest,
    ) -> Result<DisassociateRoleFromGroupResponse, DisassociateRoleFromGroupError> {
        let request_uri = format!(
            "/greengrass/groups/{group_id}/role",
            group_id = input.group_id
        );

        let mut request = SignedRequest::new("DELETE", "greengrass", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.sign_with_plus(&self.credentials_provider.credentials()?, true);
        let mut response = self.dispatcher.dispatch(&request)?;

        match response.status {
            StatusCode::Ok => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));

                if body == b"{}" {
                    body = b"null".to_vec();
                }

                debug!("Response body: {:?}", body);
                debug!("Response status: {}", response.status);
                let result =
                    serde_json::from_slice::<DisassociateRoleFromGroupResponse>(&body).unwrap();

                Ok(result)
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(DisassociateRoleFromGroupError::from_body(
                    String::from_utf8_lossy(&body).as_ref(),
                ))
            }
        }
    }

    #[doc="Disassociates the service role from the account. Without a service role, deployments will not work."]
    fn disassociate_service_role_from_account(
        &self,
    ) -> Result<DisassociateServiceRoleFromAccountResponse, DisassociateServiceRoleFromAccountError>
    {
        let request_uri = "/greengrass/servicerole";

        let mut request = SignedRequest::new("DELETE", "greengrass", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.sign_with_plus(&self.credentials_provider.credentials()?, true);
        let mut response = self.dispatcher.dispatch(&request)?;

        match response.status {
            StatusCode::Ok => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));

                if body == b"{}" {
                    body = b"null".to_vec();
                }

                debug!("Response body: {:?}", body);
                debug!("Response status: {}", response.status);
                let result = serde_json::from_slice::<DisassociateServiceRoleFromAccountResponse>(
                    &body,
                ).unwrap();

                Ok(result)
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(DisassociateServiceRoleFromAccountError::from_body(
                    String::from_utf8_lossy(&body).as_ref(),
                ))
            }
        }
    }

    #[doc = "Retrieves the role associated with a particular group."]
    fn get_associated_role(
        &self,
        input: &GetAssociatedRoleRequest,
    ) -> Result<GetAssociatedRoleResponse, GetAssociatedRoleError> {
        let request_uri = format!(
            "/greengrass/groups/{group_id}/role",
            group_id = input.group_id
        );

        let mut request = SignedRequest::new("GET", "greengrass", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.sign_with_plus(&self.credentials_provider.credentials()?, true);
        let mut response = self.dispatcher.dispatch(&request)?;

        match response.status {
            StatusCode::Ok => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));

                if body == b"{}" {
                    body = b"null".to_vec();
                }

                debug!("Response body: {:?}", body);
                debug!("Response status: {}", response.status);
                let result = serde_json::from_slice::<GetAssociatedRoleResponse>(&body).unwrap();

                Ok(result)
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(GetAssociatedRoleError::from_body(
                    String::from_utf8_lossy(&body).as_ref(),
                ))
            }
        }
    }

    #[doc = "Retrieves the connectivity information for a core."]
    fn get_connectivity_info(
        &self,
        input: &GetConnectivityInfoRequest,
    ) -> Result<GetConnectivityInfoResponse, GetConnectivityInfoError> {
        let request_uri = format!(
            "/greengrass/things/{thing_name}/connectivityInfo",
            thing_name = input.thing_name
        );

        let mut request = SignedRequest::new("GET", "greengrass", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.sign_with_plus(&self.credentials_provider.credentials()?, true);
        let mut response = self.dispatcher.dispatch(&request)?;

        match response.status {
            StatusCode::Ok => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));

                if body == b"{}" {
                    body = b"null".to_vec();
                }

                debug!("Response body: {:?}", body);
                debug!("Response status: {}", response.status);
                let result = serde_json::from_slice::<GetConnectivityInfoResponse>(&body).unwrap();

                Ok(result)
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(GetConnectivityInfoError::from_body(
                    String::from_utf8_lossy(&body).as_ref(),
                ))
            }
        }
    }

    #[doc = "Retrieves information about a core definition version."]
    fn get_core_definition(
        &self,
        input: &GetCoreDefinitionRequest,
    ) -> Result<GetCoreDefinitionResponse, GetCoreDefinitionError> {
        let request_uri = format!(
            "/greengrass/definition/cores/{core_definition_id}",
            core_definition_id = input.core_definition_id
        );

        let mut request = SignedRequest::new("GET", "greengrass", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.sign_with_plus(&self.credentials_provider.credentials()?, true);
        let mut response = self.dispatcher.dispatch(&request)?;

        match response.status {
            StatusCode::Ok => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));

                if body == b"{}" {
                    body = b"null".to_vec();
                }

                debug!("Response body: {:?}", body);
                debug!("Response status: {}", response.status);
                let result = serde_json::from_slice::<GetCoreDefinitionResponse>(&body).unwrap();

                Ok(result)
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(GetCoreDefinitionError::from_body(
                    String::from_utf8_lossy(&body).as_ref(),
                ))
            }
        }
    }

    #[doc = "Retrieves information about a core definition version."]
    fn get_core_definition_version(
        &self,
        input: &GetCoreDefinitionVersionRequest,
    ) -> Result<GetCoreDefinitionVersionResponse, GetCoreDefinitionVersionError> {
        let request_uri = format!("/greengrass/definition/cores/{core_definition_id}/versions/{core_definition_version_id}", core_definition_id = input.core_definition_id, core_definition_version_id = input.core_definition_version_id);

        let mut request = SignedRequest::new("GET", "greengrass", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.sign_with_plus(&self.credentials_provider.credentials()?, true);
        let mut response = self.dispatcher.dispatch(&request)?;

        match response.status {
            StatusCode::Ok => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));

                if body == b"{}" {
                    body = b"null".to_vec();
                }

                debug!("Response body: {:?}", body);
                debug!("Response status: {}", response.status);
                let result =
                    serde_json::from_slice::<GetCoreDefinitionVersionResponse>(&body).unwrap();

                Ok(result)
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(GetCoreDefinitionVersionError::from_body(
                    String::from_utf8_lossy(&body).as_ref(),
                ))
            }
        }
    }

    #[doc = "Returns the status of a deployment."]
    fn get_deployment_status(
        &self,
        input: &GetDeploymentStatusRequest,
    ) -> Result<GetDeploymentStatusResponse, GetDeploymentStatusError> {
        let request_uri = format!(
            "/greengrass/groups/{group_id}/deployments/{deployment_id}/status",
            deployment_id = input.deployment_id,
            group_id = input.group_id
        );

        let mut request = SignedRequest::new("GET", "greengrass", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.sign_with_plus(&self.credentials_provider.credentials()?, true);
        let mut response = self.dispatcher.dispatch(&request)?;

        match response.status {
            StatusCode::Ok => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));

                if body == b"{}" {
                    body = b"null".to_vec();
                }

                debug!("Response body: {:?}", body);
                debug!("Response status: {}", response.status);
                let result = serde_json::from_slice::<GetDeploymentStatusResponse>(&body).unwrap();

                Ok(result)
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(GetDeploymentStatusError::from_body(
                    String::from_utf8_lossy(&body).as_ref(),
                ))
            }
        }
    }

    #[doc = "Retrieves information about a device definition."]
    fn get_device_definition(
        &self,
        input: &GetDeviceDefinitionRequest,
    ) -> Result<GetDeviceDefinitionResponse, GetDeviceDefinitionError> {
        let request_uri = format!(
            "/greengrass/definition/devices/{device_definition_id}",
            device_definition_id = input.device_definition_id
        );

        let mut request = SignedRequest::new("GET", "greengrass", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.sign_with_plus(&self.credentials_provider.credentials()?, true);
        let mut response = self.dispatcher.dispatch(&request)?;

        match response.status {
            StatusCode::Ok => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));

                if body == b"{}" {
                    body = b"null".to_vec();
                }

                debug!("Response body: {:?}", body);
                debug!("Response status: {}", response.status);
                let result = serde_json::from_slice::<GetDeviceDefinitionResponse>(&body).unwrap();

                Ok(result)
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(GetDeviceDefinitionError::from_body(
                    String::from_utf8_lossy(&body).as_ref(),
                ))
            }
        }
    }

    #[doc = "Retrieves information about a device definition version."]
    fn get_device_definition_version(
        &self,
        input: &GetDeviceDefinitionVersionRequest,
    ) -> Result<GetDeviceDefinitionVersionResponse, GetDeviceDefinitionVersionError> {
        let request_uri = format!("/greengrass/definition/devices/{device_definition_id}/versions/{device_definition_version_id}", device_definition_id = input.device_definition_id, device_definition_version_id = input.device_definition_version_id);

        let mut request = SignedRequest::new("GET", "greengrass", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.sign_with_plus(&self.credentials_provider.credentials()?, true);
        let mut response = self.dispatcher.dispatch(&request)?;

        match response.status {
            StatusCode::Ok => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));

                if body == b"{}" {
                    body = b"null".to_vec();
                }

                debug!("Response body: {:?}", body);
                debug!("Response status: {}", response.status);
                let result =
                    serde_json::from_slice::<GetDeviceDefinitionVersionResponse>(&body).unwrap();

                Ok(result)
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(GetDeviceDefinitionVersionError::from_body(
                    String::from_utf8_lossy(&body).as_ref(),
                ))
            }
        }
    }

    #[doc="Retrieves information about a Lambda function definition, such as its creation time and latest version."]
    fn get_function_definition(
        &self,
        input: &GetFunctionDefinitionRequest,
    ) -> Result<GetFunctionDefinitionResponse, GetFunctionDefinitionError> {
        let request_uri = format!(
            "/greengrass/definition/functions/{function_definition_id}",
            function_definition_id = input.function_definition_id
        );

        let mut request = SignedRequest::new("GET", "greengrass", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.sign_with_plus(&self.credentials_provider.credentials()?, true);
        let mut response = self.dispatcher.dispatch(&request)?;

        match response.status {
            StatusCode::Ok => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));

                if body == b"{}" {
                    body = b"null".to_vec();
                }

                debug!("Response body: {:?}", body);
                debug!("Response status: {}", response.status);
                let result =
                    serde_json::from_slice::<GetFunctionDefinitionResponse>(&body).unwrap();

                Ok(result)
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(GetFunctionDefinitionError::from_body(
                    String::from_utf8_lossy(&body).as_ref(),
                ))
            }
        }
    }

    #[doc="Retrieves information about a Lambda function definition version, such as which Lambda functions are included in the version and their configurations."]
    fn get_function_definition_version(
        &self,
        input: &GetFunctionDefinitionVersionRequest,
    ) -> Result<GetFunctionDefinitionVersionResponse, GetFunctionDefinitionVersionError> {
        let request_uri = format!("/greengrass/definition/functions/{function_definition_id}/versions/{function_definition_version_id}", function_definition_id = input.function_definition_id, function_definition_version_id = input.function_definition_version_id);

        let mut request = SignedRequest::new("GET", "greengrass", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.sign_with_plus(&self.credentials_provider.credentials()?, true);
        let mut response = self.dispatcher.dispatch(&request)?;

        match response.status {
            StatusCode::Ok => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));

                if body == b"{}" {
                    body = b"null".to_vec();
                }

                debug!("Response body: {:?}", body);
                debug!("Response status: {}", response.status);
                let result =
                    serde_json::from_slice::<GetFunctionDefinitionVersionResponse>(&body).unwrap();

                Ok(result)
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(GetFunctionDefinitionVersionError::from_body(
                    String::from_utf8_lossy(&body).as_ref(),
                ))
            }
        }
    }

    #[doc = "Retrieves information about a group."]
    fn get_group(&self, input: &GetGroupRequest) -> Result<GetGroupResponse, GetGroupError> {
        let request_uri = format!("/greengrass/groups/{group_id}", group_id = input.group_id);

        let mut request = SignedRequest::new("GET", "greengrass", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.sign_with_plus(&self.credentials_provider.credentials()?, true);
        let mut response = self.dispatcher.dispatch(&request)?;

        match response.status {
            StatusCode::Ok => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));

                if body == b"{}" {
                    body = b"null".to_vec();
                }

                debug!("Response body: {:?}", body);
                debug!("Response status: {}", response.status);
                let result = serde_json::from_slice::<GetGroupResponse>(&body).unwrap();

                Ok(result)
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(GetGroupError::from_body(
                    String::from_utf8_lossy(&body).as_ref(),
                ))
            }
        }
    }

    #[doc = "Retreives the CA associated with a group. Returns the public key of the CA."]
    fn get_group_certificate_authority(
        &self,
        input: &GetGroupCertificateAuthorityRequest,
    ) -> Result<GetGroupCertificateAuthorityResponse, GetGroupCertificateAuthorityError> {
        let request_uri = format!(
            "/greengrass/groups/{group_id}/certificateauthorities/{certificate_authority_id}",
            certificate_authority_id = input.certificate_authority_id,
            group_id = input.group_id
        );

        let mut request = SignedRequest::new("GET", "greengrass", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.sign_with_plus(&self.credentials_provider.credentials()?, true);
        let mut response = self.dispatcher.dispatch(&request)?;

        match response.status {
            StatusCode::Ok => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));

                if body == b"{}" {
                    body = b"null".to_vec();
                }

                debug!("Response body: {:?}", body);
                debug!("Response status: {}", response.status);
                let result =
                    serde_json::from_slice::<GetGroupCertificateAuthorityResponse>(&body).unwrap();

                Ok(result)
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(GetGroupCertificateAuthorityError::from_body(
                    String::from_utf8_lossy(&body).as_ref(),
                ))
            }
        }
    }

    #[doc = "Retrieves the current configuration for the CA used by the group."]
    fn get_group_certificate_configuration(
        &self,
        input: &GetGroupCertificateConfigurationRequest,
    ) -> Result<GetGroupCertificateConfigurationResponse, GetGroupCertificateConfigurationError>
    {
        let request_uri = format!(
            "/greengrass/groups/{group_id}/certificateauthorities/configuration/expiry",
            group_id = input.group_id
        );

        let mut request = SignedRequest::new("GET", "greengrass", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.sign_with_plus(&self.credentials_provider.credentials()?, true);
        let mut response = self.dispatcher.dispatch(&request)?;

        match response.status {
            StatusCode::Ok => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));

                if body == b"{}" {
                    body = b"null".to_vec();
                }

                debug!("Response body: {:?}", body);
                debug!("Response status: {}", response.status);
                let result = serde_json::from_slice::<GetGroupCertificateConfigurationResponse>(
                    &body,
                ).unwrap();

                Ok(result)
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(GetGroupCertificateConfigurationError::from_body(
                    String::from_utf8_lossy(&body).as_ref(),
                ))
            }
        }
    }

    #[doc = "Retrieves information about a group version."]
    fn get_group_version(
        &self,
        input: &GetGroupVersionRequest,
    ) -> Result<GetGroupVersionResponse, GetGroupVersionError> {
        let request_uri = format!(
            "/greengrass/groups/{group_id}/versions/{group_version_id}",
            group_id = input.group_id,
            group_version_id = input.group_version_id
        );

        let mut request = SignedRequest::new("GET", "greengrass", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.sign_with_plus(&self.credentials_provider.credentials()?, true);
        let mut response = self.dispatcher.dispatch(&request)?;

        match response.status {
            StatusCode::Ok => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));

                if body == b"{}" {
                    body = b"null".to_vec();
                }

                debug!("Response body: {:?}", body);
                debug!("Response status: {}", response.status);
                let result = serde_json::from_slice::<GetGroupVersionResponse>(&body).unwrap();

                Ok(result)
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(GetGroupVersionError::from_body(
                    String::from_utf8_lossy(&body).as_ref(),
                ))
            }
        }
    }

    #[doc = "Retrieves information about a logger definition."]
    fn get_logger_definition(
        &self,
        input: &GetLoggerDefinitionRequest,
    ) -> Result<GetLoggerDefinitionResponse, GetLoggerDefinitionError> {
        let request_uri = format!(
            "/greengrass/definition/loggers/{logger_definition_id}",
            logger_definition_id = input.logger_definition_id
        );

        let mut request = SignedRequest::new("GET", "greengrass", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.sign_with_plus(&self.credentials_provider.credentials()?, true);
        let mut response = self.dispatcher.dispatch(&request)?;

        match response.status {
            StatusCode::Ok => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));

                if body == b"{}" {
                    body = b"null".to_vec();
                }

                debug!("Response body: {:?}", body);
                debug!("Response status: {}", response.status);
                let result = serde_json::from_slice::<GetLoggerDefinitionResponse>(&body).unwrap();

                Ok(result)
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(GetLoggerDefinitionError::from_body(
                    String::from_utf8_lossy(&body).as_ref(),
                ))
            }
        }
    }

    #[doc = "Retrieves information about a logger definition version."]
    fn get_logger_definition_version(
        &self,
        input: &GetLoggerDefinitionVersionRequest,
    ) -> Result<GetLoggerDefinitionVersionResponse, GetLoggerDefinitionVersionError> {
        let request_uri = format!("/greengrass/definition/loggers/{logger_definition_id}/versions/{logger_definition_version_id}", logger_definition_id = input.logger_definition_id, logger_definition_version_id = input.logger_definition_version_id);

        let mut request = SignedRequest::new("GET", "greengrass", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.sign_with_plus(&self.credentials_provider.credentials()?, true);
        let mut response = self.dispatcher.dispatch(&request)?;

        match response.status {
            StatusCode::Ok => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));

                if body == b"{}" {
                    body = b"null".to_vec();
                }

                debug!("Response body: {:?}", body);
                debug!("Response status: {}", response.status);
                let result =
                    serde_json::from_slice::<GetLoggerDefinitionVersionResponse>(&body).unwrap();

                Ok(result)
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(GetLoggerDefinitionVersionError::from_body(
                    String::from_utf8_lossy(&body).as_ref(),
                ))
            }
        }
    }

    #[doc = "Retrieves the service role that is attached to the account."]
    fn get_service_role_for_account(
        &self,
    ) -> Result<GetServiceRoleForAccountResponse, GetServiceRoleForAccountError> {
        let request_uri = "/greengrass/servicerole";

        let mut request = SignedRequest::new("GET", "greengrass", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.sign_with_plus(&self.credentials_provider.credentials()?, true);
        let mut response = self.dispatcher.dispatch(&request)?;

        match response.status {
            StatusCode::Ok => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));

                if body == b"{}" {
                    body = b"null".to_vec();
                }

                debug!("Response body: {:?}", body);
                debug!("Response status: {}", response.status);
                let result =
                    serde_json::from_slice::<GetServiceRoleForAccountResponse>(&body).unwrap();

                Ok(result)
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(GetServiceRoleForAccountError::from_body(
                    String::from_utf8_lossy(&body).as_ref(),
                ))
            }
        }
    }

    #[doc = "Retrieves information about a subscription definition."]
    fn get_subscription_definition(
        &self,
        input: &GetSubscriptionDefinitionRequest,
    ) -> Result<GetSubscriptionDefinitionResponse, GetSubscriptionDefinitionError> {
        let request_uri = format!(
            "/greengrass/definition/subscriptions/{subscription_definition_id}",
            subscription_definition_id = input.subscription_definition_id
        );

        let mut request = SignedRequest::new("GET", "greengrass", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.sign_with_plus(&self.credentials_provider.credentials()?, true);
        let mut response = self.dispatcher.dispatch(&request)?;

        match response.status {
            StatusCode::Ok => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));

                if body == b"{}" {
                    body = b"null".to_vec();
                }

                debug!("Response body: {:?}", body);
                debug!("Response status: {}", response.status);
                let result =
                    serde_json::from_slice::<GetSubscriptionDefinitionResponse>(&body).unwrap();

                Ok(result)
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(GetSubscriptionDefinitionError::from_body(
                    String::from_utf8_lossy(&body).as_ref(),
                ))
            }
        }
    }

    #[doc = "Retrieves information about a subscription definition version."]
    fn get_subscription_definition_version(
        &self,
        input: &GetSubscriptionDefinitionVersionRequest,
    ) -> Result<GetSubscriptionDefinitionVersionResponse, GetSubscriptionDefinitionVersionError>
    {
        let request_uri = format!("/greengrass/definition/subscriptions/{subscription_definition_id}/versions/{subscription_definition_version_id}", subscription_definition_id = input.subscription_definition_id, subscription_definition_version_id = input.subscription_definition_version_id);

        let mut request = SignedRequest::new("GET", "greengrass", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.sign_with_plus(&self.credentials_provider.credentials()?, true);
        let mut response = self.dispatcher.dispatch(&request)?;

        match response.status {
            StatusCode::Ok => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));

                if body == b"{}" {
                    body = b"null".to_vec();
                }

                debug!("Response body: {:?}", body);
                debug!("Response status: {}", response.status);
                let result = serde_json::from_slice::<GetSubscriptionDefinitionVersionResponse>(
                    &body,
                ).unwrap();

                Ok(result)
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(GetSubscriptionDefinitionVersionError::from_body(
                    String::from_utf8_lossy(&body).as_ref(),
                ))
            }
        }
    }

    #[doc = "Lists versions of a core definition."]
    fn list_core_definition_versions(
        &self,
        input: &ListCoreDefinitionVersionsRequest,
    ) -> Result<ListCoreDefinitionVersionsResponse, ListCoreDefinitionVersionsError> {
        let request_uri = format!(
            "/greengrass/definition/cores/{core_definition_id}/versions",
            core_definition_id = input.core_definition_id
        );

        let mut request = SignedRequest::new("GET", "greengrass", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut params = Params::new();
        if let Some(ref x) = input.max_results {
            params.put("MaxResults", x);
        }
        if let Some(ref x) = input.next_token {
            params.put("NextToken", x);
        }
        request.set_params(params);

        request.sign_with_plus(&self.credentials_provider.credentials()?, true);
        let mut response = self.dispatcher.dispatch(&request)?;

        match response.status {
            StatusCode::Ok => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));

                if body == b"{}" {
                    body = b"null".to_vec();
                }

                debug!("Response body: {:?}", body);
                debug!("Response status: {}", response.status);
                let result =
                    serde_json::from_slice::<ListCoreDefinitionVersionsResponse>(&body).unwrap();

                Ok(result)
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(ListCoreDefinitionVersionsError::from_body(
                    String::from_utf8_lossy(&body).as_ref(),
                ))
            }
        }
    }

    #[doc = "Retrieves a list of core definitions."]
    fn list_core_definitions(
        &self,
        input: &ListCoreDefinitionsRequest,
    ) -> Result<ListCoreDefinitionsResponse, ListCoreDefinitionsError> {
        let request_uri = "/greengrass/definition/cores";

        let mut request = SignedRequest::new("GET", "greengrass", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut params = Params::new();
        if let Some(ref x) = input.max_results {
            params.put("MaxResults", x);
        }
        if let Some(ref x) = input.next_token {
            params.put("NextToken", x);
        }
        request.set_params(params);

        request.sign_with_plus(&self.credentials_provider.credentials()?, true);
        let mut response = self.dispatcher.dispatch(&request)?;

        match response.status {
            StatusCode::Ok => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));

                if body == b"{}" {
                    body = b"null".to_vec();
                }

                debug!("Response body: {:?}", body);
                debug!("Response status: {}", response.status);
                let result = serde_json::from_slice::<ListCoreDefinitionsResponse>(&body).unwrap();

                Ok(result)
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(ListCoreDefinitionsError::from_body(
                    String::from_utf8_lossy(&body).as_ref(),
                ))
            }
        }
    }

    #[doc = "Returns a history of deployments for the group."]
    fn list_deployments(
        &self,
        input: &ListDeploymentsRequest,
    ) -> Result<ListDeploymentsResponse, ListDeploymentsError> {
        let request_uri = format!(
            "/greengrass/groups/{group_id}/deployments",
            group_id = input.group_id
        );

        let mut request = SignedRequest::new("GET", "greengrass", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut params = Params::new();
        if let Some(ref x) = input.max_results {
            params.put("MaxResults", x);
        }
        if let Some(ref x) = input.next_token {
            params.put("NextToken", x);
        }
        request.set_params(params);

        request.sign_with_plus(&self.credentials_provider.credentials()?, true);
        let mut response = self.dispatcher.dispatch(&request)?;

        match response.status {
            StatusCode::Ok => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));

                if body == b"{}" {
                    body = b"null".to_vec();
                }

                debug!("Response body: {:?}", body);
                debug!("Response status: {}", response.status);
                let result = serde_json::from_slice::<ListDeploymentsResponse>(&body).unwrap();

                Ok(result)
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(ListDeploymentsError::from_body(
                    String::from_utf8_lossy(&body).as_ref(),
                ))
            }
        }
    }

    #[doc = "Lists the versions of a device definition."]
    fn list_device_definition_versions(
        &self,
        input: &ListDeviceDefinitionVersionsRequest,
    ) -> Result<ListDeviceDefinitionVersionsResponse, ListDeviceDefinitionVersionsError> {
        let request_uri = format!(
            "/greengrass/definition/devices/{device_definition_id}/versions",
            device_definition_id = input.device_definition_id
        );

        let mut request = SignedRequest::new("GET", "greengrass", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut params = Params::new();
        if let Some(ref x) = input.max_results {
            params.put("MaxResults", x);
        }
        if let Some(ref x) = input.next_token {
            params.put("NextToken", x);
        }
        request.set_params(params);

        request.sign_with_plus(&self.credentials_provider.credentials()?, true);
        let mut response = self.dispatcher.dispatch(&request)?;

        match response.status {
            StatusCode::Ok => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));

                if body == b"{}" {
                    body = b"null".to_vec();
                }

                debug!("Response body: {:?}", body);
                debug!("Response status: {}", response.status);
                let result =
                    serde_json::from_slice::<ListDeviceDefinitionVersionsResponse>(&body).unwrap();

                Ok(result)
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(ListDeviceDefinitionVersionsError::from_body(
                    String::from_utf8_lossy(&body).as_ref(),
                ))
            }
        }
    }

    #[doc = "Retrieves a list of device definitions."]
    fn list_device_definitions(
        &self,
        input: &ListDeviceDefinitionsRequest,
    ) -> Result<ListDeviceDefinitionsResponse, ListDeviceDefinitionsError> {
        let request_uri = "/greengrass/definition/devices";

        let mut request = SignedRequest::new("GET", "greengrass", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut params = Params::new();
        if let Some(ref x) = input.max_results {
            params.put("MaxResults", x);
        }
        if let Some(ref x) = input.next_token {
            params.put("NextToken", x);
        }
        request.set_params(params);

        request.sign_with_plus(&self.credentials_provider.credentials()?, true);
        let mut response = self.dispatcher.dispatch(&request)?;

        match response.status {
            StatusCode::Ok => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));

                if body == b"{}" {
                    body = b"null".to_vec();
                }

                debug!("Response body: {:?}", body);
                debug!("Response status: {}", response.status);
                let result =
                    serde_json::from_slice::<ListDeviceDefinitionsResponse>(&body).unwrap();

                Ok(result)
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(ListDeviceDefinitionsError::from_body(
                    String::from_utf8_lossy(&body).as_ref(),
                ))
            }
        }
    }

    #[doc = "Lists the versions of a Lambda function definition."]
    fn list_function_definition_versions(
        &self,
        input: &ListFunctionDefinitionVersionsRequest,
    ) -> Result<ListFunctionDefinitionVersionsResponse, ListFunctionDefinitionVersionsError> {
        let request_uri = format!(
            "/greengrass/definition/functions/{function_definition_id}/versions",
            function_definition_id = input.function_definition_id
        );

        let mut request = SignedRequest::new("GET", "greengrass", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut params = Params::new();
        if let Some(ref x) = input.max_results {
            params.put("MaxResults", x);
        }
        if let Some(ref x) = input.next_token {
            params.put("NextToken", x);
        }
        request.set_params(params);

        request.sign_with_plus(&self.credentials_provider.credentials()?, true);
        let mut response = self.dispatcher.dispatch(&request)?;

        match response.status {
            StatusCode::Ok => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));

                if body == b"{}" {
                    body = b"null".to_vec();
                }

                debug!("Response body: {:?}", body);
                debug!("Response status: {}", response.status);
                let result = serde_json::from_slice::<ListFunctionDefinitionVersionsResponse>(
                    &body,
                ).unwrap();

                Ok(result)
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(ListFunctionDefinitionVersionsError::from_body(
                    String::from_utf8_lossy(&body).as_ref(),
                ))
            }
        }
    }

    #[doc = "Retrieves a list of Lambda function definitions."]
    fn list_function_definitions(
        &self,
        input: &ListFunctionDefinitionsRequest,
    ) -> Result<ListFunctionDefinitionsResponse, ListFunctionDefinitionsError> {
        let request_uri = "/greengrass/definition/functions";

        let mut request = SignedRequest::new("GET", "greengrass", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut params = Params::new();
        if let Some(ref x) = input.max_results {
            params.put("MaxResults", x);
        }
        if let Some(ref x) = input.next_token {
            params.put("NextToken", x);
        }
        request.set_params(params);

        request.sign_with_plus(&self.credentials_provider.credentials()?, true);
        let mut response = self.dispatcher.dispatch(&request)?;

        match response.status {
            StatusCode::Ok => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));

                if body == b"{}" {
                    body = b"null".to_vec();
                }

                debug!("Response body: {:?}", body);
                debug!("Response status: {}", response.status);
                let result =
                    serde_json::from_slice::<ListFunctionDefinitionsResponse>(&body).unwrap();

                Ok(result)
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(ListFunctionDefinitionsError::from_body(
                    String::from_utf8_lossy(&body).as_ref(),
                ))
            }
        }
    }

    #[doc = "Retrieves the current CAs for a group."]
    fn list_group_certificate_authorities(
        &self,
        input: &ListGroupCertificateAuthoritiesRequest,
    ) -> Result<ListGroupCertificateAuthoritiesResponse, ListGroupCertificateAuthoritiesError> {
        let request_uri = format!(
            "/greengrass/groups/{group_id}/certificateauthorities",
            group_id = input.group_id
        );

        let mut request = SignedRequest::new("GET", "greengrass", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.sign_with_plus(&self.credentials_provider.credentials()?, true);
        let mut response = self.dispatcher.dispatch(&request)?;

        match response.status {
            StatusCode::Ok => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));

                if body == b"{}" {
                    body = b"null".to_vec();
                }

                debug!("Response body: {:?}", body);
                debug!("Response status: {}", response.status);
                let result = serde_json::from_slice::<ListGroupCertificateAuthoritiesResponse>(
                    &body,
                ).unwrap();

                Ok(result)
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(ListGroupCertificateAuthoritiesError::from_body(
                    String::from_utf8_lossy(&body).as_ref(),
                ))
            }
        }
    }

    #[doc = "List the versions of a group."]
    fn list_group_versions(
        &self,
        input: &ListGroupVersionsRequest,
    ) -> Result<ListGroupVersionsResponse, ListGroupVersionsError> {
        let request_uri = format!(
            "/greengrass/groups/{group_id}/versions",
            group_id = input.group_id
        );

        let mut request = SignedRequest::new("GET", "greengrass", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut params = Params::new();
        if let Some(ref x) = input.max_results {
            params.put("MaxResults", x);
        }
        if let Some(ref x) = input.next_token {
            params.put("NextToken", x);
        }
        request.set_params(params);

        request.sign_with_plus(&self.credentials_provider.credentials()?, true);
        let mut response = self.dispatcher.dispatch(&request)?;

        match response.status {
            StatusCode::Ok => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));

                if body == b"{}" {
                    body = b"null".to_vec();
                }

                debug!("Response body: {:?}", body);
                debug!("Response status: {}", response.status);
                let result = serde_json::from_slice::<ListGroupVersionsResponse>(&body).unwrap();

                Ok(result)
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(ListGroupVersionsError::from_body(
                    String::from_utf8_lossy(&body).as_ref(),
                ))
            }
        }
    }

    #[doc = "Retrieves a list of groups."]
    fn list_groups(
        &self,
        input: &ListGroupsRequest,
    ) -> Result<ListGroupsResponse, ListGroupsError> {
        let request_uri = "/greengrass/groups";

        let mut request = SignedRequest::new("GET", "greengrass", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut params = Params::new();
        if let Some(ref x) = input.max_results {
            params.put("MaxResults", x);
        }
        if let Some(ref x) = input.next_token {
            params.put("NextToken", x);
        }
        request.set_params(params);

        request.sign_with_plus(&self.credentials_provider.credentials()?, true);
        let mut response = self.dispatcher.dispatch(&request)?;

        match response.status {
            StatusCode::Ok => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));

                if body == b"{}" {
                    body = b"null".to_vec();
                }

                debug!("Response body: {:?}", body);
                debug!("Response status: {}", response.status);
                let result = serde_json::from_slice::<ListGroupsResponse>(&body).unwrap();

                Ok(result)
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(ListGroupsError::from_body(
                    String::from_utf8_lossy(&body).as_ref(),
                ))
            }
        }
    }

    #[doc = "Lists the versions of a logger definition."]
    fn list_logger_definition_versions(
        &self,
        input: &ListLoggerDefinitionVersionsRequest,
    ) -> Result<ListLoggerDefinitionVersionsResponse, ListLoggerDefinitionVersionsError> {
        let request_uri = format!(
            "/greengrass/definition/loggers/{logger_definition_id}/versions",
            logger_definition_id = input.logger_definition_id
        );

        let mut request = SignedRequest::new("GET", "greengrass", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut params = Params::new();
        if let Some(ref x) = input.max_results {
            params.put("MaxResults", x);
        }
        if let Some(ref x) = input.next_token {
            params.put("NextToken", x);
        }
        request.set_params(params);

        request.sign_with_plus(&self.credentials_provider.credentials()?, true);
        let mut response = self.dispatcher.dispatch(&request)?;

        match response.status {
            StatusCode::Ok => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));

                if body == b"{}" {
                    body = b"null".to_vec();
                }

                debug!("Response body: {:?}", body);
                debug!("Response status: {}", response.status);
                let result =
                    serde_json::from_slice::<ListLoggerDefinitionVersionsResponse>(&body).unwrap();

                Ok(result)
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(ListLoggerDefinitionVersionsError::from_body(
                    String::from_utf8_lossy(&body).as_ref(),
                ))
            }
        }
    }

    #[doc = "Retrieves a list of logger definitions."]
    fn list_logger_definitions(
        &self,
        input: &ListLoggerDefinitionsRequest,
    ) -> Result<ListLoggerDefinitionsResponse, ListLoggerDefinitionsError> {
        let request_uri = "/greengrass/definition/loggers";

        let mut request = SignedRequest::new("GET", "greengrass", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut params = Params::new();
        if let Some(ref x) = input.max_results {
            params.put("MaxResults", x);
        }
        if let Some(ref x) = input.next_token {
            params.put("NextToken", x);
        }
        request.set_params(params);

        request.sign_with_plus(&self.credentials_provider.credentials()?, true);
        let mut response = self.dispatcher.dispatch(&request)?;

        match response.status {
            StatusCode::Ok => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));

                if body == b"{}" {
                    body = b"null".to_vec();
                }

                debug!("Response body: {:?}", body);
                debug!("Response status: {}", response.status);
                let result =
                    serde_json::from_slice::<ListLoggerDefinitionsResponse>(&body).unwrap();

                Ok(result)
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(ListLoggerDefinitionsError::from_body(
                    String::from_utf8_lossy(&body).as_ref(),
                ))
            }
        }
    }

    #[doc = "Lists the versions of a subscription definition."]
    fn list_subscription_definition_versions(
        &self,
        input: &ListSubscriptionDefinitionVersionsRequest,
    ) -> Result<ListSubscriptionDefinitionVersionsResponse, ListSubscriptionDefinitionVersionsError>
    {
        let request_uri = format!(
            "/greengrass/definition/subscriptions/{subscription_definition_id}/versions",
            subscription_definition_id = input.subscription_definition_id
        );

        let mut request = SignedRequest::new("GET", "greengrass", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut params = Params::new();
        if let Some(ref x) = input.max_results {
            params.put("MaxResults", x);
        }
        if let Some(ref x) = input.next_token {
            params.put("NextToken", x);
        }
        request.set_params(params);

        request.sign_with_plus(&self.credentials_provider.credentials()?, true);
        let mut response = self.dispatcher.dispatch(&request)?;

        match response.status {
            StatusCode::Ok => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));

                if body == b"{}" {
                    body = b"null".to_vec();
                }

                debug!("Response body: {:?}", body);
                debug!("Response status: {}", response.status);
                let result = serde_json::from_slice::<ListSubscriptionDefinitionVersionsResponse>(
                    &body,
                ).unwrap();

                Ok(result)
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(ListSubscriptionDefinitionVersionsError::from_body(
                    String::from_utf8_lossy(&body).as_ref(),
                ))
            }
        }
    }

    #[doc = "Retrieves a list of subscription definitions."]
    fn list_subscription_definitions(
        &self,
        input: &ListSubscriptionDefinitionsRequest,
    ) -> Result<ListSubscriptionDefinitionsResponse, ListSubscriptionDefinitionsError> {
        let request_uri = "/greengrass/definition/subscriptions";

        let mut request = SignedRequest::new("GET", "greengrass", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut params = Params::new();
        if let Some(ref x) = input.max_results {
            params.put("MaxResults", x);
        }
        if let Some(ref x) = input.next_token {
            params.put("NextToken", x);
        }
        request.set_params(params);

        request.sign_with_plus(&self.credentials_provider.credentials()?, true);
        let mut response = self.dispatcher.dispatch(&request)?;

        match response.status {
            StatusCode::Ok => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));

                if body == b"{}" {
                    body = b"null".to_vec();
                }

                debug!("Response body: {:?}", body);
                debug!("Response status: {}", response.status);
                let result =
                    serde_json::from_slice::<ListSubscriptionDefinitionsResponse>(&body).unwrap();

                Ok(result)
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(ListSubscriptionDefinitionsError::from_body(
                    String::from_utf8_lossy(&body).as_ref(),
                ))
            }
        }
    }

    #[doc="Updates the connectivity information for the core. Any devices that belong to the group which has this core will receive this information in order to find the location of the core and connect to it."]
    fn update_connectivity_info(
        &self,
        input: &UpdateConnectivityInfoRequest,
    ) -> Result<UpdateConnectivityInfoResponse, UpdateConnectivityInfoError> {
        let request_uri = format!(
            "/greengrass/things/{thing_name}/connectivityInfo",
            thing_name = input.thing_name
        );

        let mut request = SignedRequest::new("PUT", "greengrass", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(input).unwrap());
        request.set_payload(encoded);

        request.sign_with_plus(&self.credentials_provider.credentials()?, true);
        let mut response = self.dispatcher.dispatch(&request)?;

        match response.status {
            StatusCode::Ok => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));

                if body == b"{}" {
                    body = b"null".to_vec();
                }

                debug!("Response body: {:?}", body);
                debug!("Response status: {}", response.status);
                let result =
                    serde_json::from_slice::<UpdateConnectivityInfoResponse>(&body).unwrap();

                Ok(result)
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(UpdateConnectivityInfoError::from_body(
                    String::from_utf8_lossy(&body).as_ref(),
                ))
            }
        }
    }

    #[doc = "Updates a core definition."]
    fn update_core_definition(
        &self,
        input: &UpdateCoreDefinitionRequest,
    ) -> Result<UpdateCoreDefinitionResponse, UpdateCoreDefinitionError> {
        let request_uri = format!(
            "/greengrass/definition/cores/{core_definition_id}",
            core_definition_id = input.core_definition_id
        );

        let mut request = SignedRequest::new("PUT", "greengrass", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(input).unwrap());
        request.set_payload(encoded);

        request.sign_with_plus(&self.credentials_provider.credentials()?, true);
        let mut response = self.dispatcher.dispatch(&request)?;

        match response.status {
            StatusCode::Ok => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));

                if body == b"{}" {
                    body = b"null".to_vec();
                }

                debug!("Response body: {:?}", body);
                debug!("Response status: {}", response.status);
                let result = serde_json::from_slice::<UpdateCoreDefinitionResponse>(&body).unwrap();

                Ok(result)
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(UpdateCoreDefinitionError::from_body(
                    String::from_utf8_lossy(&body).as_ref(),
                ))
            }
        }
    }

    #[doc = "Updates a device definition."]
    fn update_device_definition(
        &self,
        input: &UpdateDeviceDefinitionRequest,
    ) -> Result<UpdateDeviceDefinitionResponse, UpdateDeviceDefinitionError> {
        let request_uri = format!(
            "/greengrass/definition/devices/{device_definition_id}",
            device_definition_id = input.device_definition_id
        );

        let mut request = SignedRequest::new("PUT", "greengrass", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(input).unwrap());
        request.set_payload(encoded);

        request.sign_with_plus(&self.credentials_provider.credentials()?, true);
        let mut response = self.dispatcher.dispatch(&request)?;

        match response.status {
            StatusCode::Ok => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));

                if body == b"{}" {
                    body = b"null".to_vec();
                }

                debug!("Response body: {:?}", body);
                debug!("Response status: {}", response.status);
                let result =
                    serde_json::from_slice::<UpdateDeviceDefinitionResponse>(&body).unwrap();

                Ok(result)
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(UpdateDeviceDefinitionError::from_body(
                    String::from_utf8_lossy(&body).as_ref(),
                ))
            }
        }
    }

    #[doc = "Updates a Lambda function definition."]
    fn update_function_definition(
        &self,
        input: &UpdateFunctionDefinitionRequest,
    ) -> Result<UpdateFunctionDefinitionResponse, UpdateFunctionDefinitionError> {
        let request_uri = format!(
            "/greengrass/definition/functions/{function_definition_id}",
            function_definition_id = input.function_definition_id
        );

        let mut request = SignedRequest::new("PUT", "greengrass", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(input).unwrap());
        request.set_payload(encoded);

        request.sign_with_plus(&self.credentials_provider.credentials()?, true);
        let mut response = self.dispatcher.dispatch(&request)?;

        match response.status {
            StatusCode::Ok => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));

                if body == b"{}" {
                    body = b"null".to_vec();
                }

                debug!("Response body: {:?}", body);
                debug!("Response status: {}", response.status);
                let result =
                    serde_json::from_slice::<UpdateFunctionDefinitionResponse>(&body).unwrap();

                Ok(result)
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(UpdateFunctionDefinitionError::from_body(
                    String::from_utf8_lossy(&body).as_ref(),
                ))
            }
        }
    }

    #[doc = "Updates a group."]
    fn update_group(
        &self,
        input: &UpdateGroupRequest,
    ) -> Result<UpdateGroupResponse, UpdateGroupError> {
        let request_uri = format!("/greengrass/groups/{group_id}", group_id = input.group_id);

        let mut request = SignedRequest::new("PUT", "greengrass", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(input).unwrap());
        request.set_payload(encoded);

        request.sign_with_plus(&self.credentials_provider.credentials()?, true);
        let mut response = self.dispatcher.dispatch(&request)?;

        match response.status {
            StatusCode::Ok => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));

                if body == b"{}" {
                    body = b"null".to_vec();
                }

                debug!("Response body: {:?}", body);
                debug!("Response status: {}", response.status);
                let result = serde_json::from_slice::<UpdateGroupResponse>(&body).unwrap();

                Ok(result)
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(UpdateGroupError::from_body(
                    String::from_utf8_lossy(&body).as_ref(),
                ))
            }
        }
    }

    #[doc = "Updates the Cert expiry time for a group."]
    fn update_group_certificate_configuration(
        &self,
        input: &UpdateGroupCertificateConfigurationRequest,
    ) -> Result<UpdateGroupCertificateConfigurationResponse, UpdateGroupCertificateConfigurationError>
    {
        let request_uri = format!(
            "/greengrass/groups/{group_id}/certificateauthorities/configuration/expiry",
            group_id = input.group_id
        );

        let mut request = SignedRequest::new("PUT", "greengrass", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(input).unwrap());
        request.set_payload(encoded);

        request.sign_with_plus(&self.credentials_provider.credentials()?, true);
        let mut response = self.dispatcher.dispatch(&request)?;

        match response.status {
            StatusCode::Ok => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));

                if body == b"{}" {
                    body = b"null".to_vec();
                }

                debug!("Response body: {:?}", body);
                debug!("Response status: {}", response.status);
                let result =
                    serde_json::from_slice::<UpdateGroupCertificateConfigurationResponse>(&body)
                        .unwrap();

                Ok(result)
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(UpdateGroupCertificateConfigurationError::from_body(
                    String::from_utf8_lossy(&body).as_ref(),
                ))
            }
        }
    }

    #[doc = "Updates a logger definition."]
    fn update_logger_definition(
        &self,
        input: &UpdateLoggerDefinitionRequest,
    ) -> Result<UpdateLoggerDefinitionResponse, UpdateLoggerDefinitionError> {
        let request_uri = format!(
            "/greengrass/definition/loggers/{logger_definition_id}",
            logger_definition_id = input.logger_definition_id
        );

        let mut request = SignedRequest::new("PUT", "greengrass", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(input).unwrap());
        request.set_payload(encoded);

        request.sign_with_plus(&self.credentials_provider.credentials()?, true);
        let mut response = self.dispatcher.dispatch(&request)?;

        match response.status {
            StatusCode::Ok => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));

                if body == b"{}" {
                    body = b"null".to_vec();
                }

                debug!("Response body: {:?}", body);
                debug!("Response status: {}", response.status);
                let result =
                    serde_json::from_slice::<UpdateLoggerDefinitionResponse>(&body).unwrap();

                Ok(result)
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(UpdateLoggerDefinitionError::from_body(
                    String::from_utf8_lossy(&body).as_ref(),
                ))
            }
        }
    }

    #[doc = "Updates a subscription definition."]
    fn update_subscription_definition(
        &self,
        input: &UpdateSubscriptionDefinitionRequest,
    ) -> Result<UpdateSubscriptionDefinitionResponse, UpdateSubscriptionDefinitionError> {
        let request_uri = format!(
            "/greengrass/definition/subscriptions/{subscription_definition_id}",
            subscription_definition_id = input.subscription_definition_id
        );

        let mut request = SignedRequest::new("PUT", "greengrass", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(input).unwrap());
        request.set_payload(encoded);

        request.sign_with_plus(&self.credentials_provider.credentials()?, true);
        let mut response = self.dispatcher.dispatch(&request)?;

        match response.status {
            StatusCode::Ok => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));

                if body == b"{}" {
                    body = b"null".to_vec();
                }

                debug!("Response body: {:?}", body);
                debug!("Response status: {}", response.status);
                let result =
                    serde_json::from_slice::<UpdateSubscriptionDefinitionResponse>(&body).unwrap();

                Ok(result)
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(UpdateSubscriptionDefinitionError::from_body(
                    String::from_utf8_lossy(&body).as_ref(),
                ))
            }
        }
    }
}

#[cfg(test)]
mod protocol_tests {}
