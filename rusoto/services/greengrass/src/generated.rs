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
use std::io;

#[allow(warnings)]
use futures::future;
use futures::Future;
use rusoto_core::region;
use rusoto_core::request::DispatchSignedRequest;
use rusoto_core::{Client, RusotoFuture};

use rusoto_core::credential::{CredentialsError, ProvideAwsCredentials};
use rusoto_core::request::HttpDispatchError;

use rusoto_core::param::{Params, ServiceParams};
use rusoto_core::signature::SignedRequest;
use serde_json;
use serde_json::from_str;
use serde_json::Value as SerdeJsonValue;
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct AssociateRoleToGroupRequest {
    /// <p>The ID of the AWS Greengrass group.</p>
    #[serde(rename = "GroupId")]
    pub group_id: String,
    /// <p>The ARN of the role you wish to associate with this group.</p>
    #[serde(rename = "RoleArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_arn: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct AssociateRoleToGroupResponse {
    /// <p>The time, in milliseconds since the epoch, when the role ARN was associated with the group.</p>
    #[serde(rename = "AssociatedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub associated_at: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct AssociateServiceRoleToAccountRequest {
    /// <p>The ARN of the service role you wish to associate with your account.</p>
    #[serde(rename = "RoleArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_arn: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct AssociateServiceRoleToAccountResponse {
    /// <p>The time when the service role was associated with the account.</p>
    #[serde(rename = "AssociatedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub associated_at: Option<String>,
}

/// <p>Information about a Greengrass core&#39;s connectivity.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ConnectivityInfo {
    /// <p>The endpoint for the Greengrass core. Can be an IP address or DNS.</p>
    #[serde(rename = "HostAddress")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub host_address: Option<String>,
    /// <p>The ID of the connectivity information.</p>
    #[serde(rename = "Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// <p>Metadata for this endpoint.</p>
    #[serde(rename = "Metadata")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<String>,
    /// <p>The port of the Greengrass core. Usually 8883.</p>
    #[serde(rename = "PortNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub port_number: Option<i64>,
}

/// <p>Information about a core.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Core {
    /// <p>The ARN of the certificate associated with the core.</p>
    #[serde(rename = "CertificateArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate_arn: Option<String>,
    /// <p>The ID of the core.</p>
    #[serde(rename = "Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// <p>If true, the core&#39;s local shadow is automatically synced with the cloud.</p>
    #[serde(rename = "SyncShadow")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sync_shadow: Option<bool>,
    /// <p>The ARN of the thing which is the core.</p>
    #[serde(rename = "ThingArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thing_arn: Option<String>,
}

/// <p>Information about a core definition version.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CoreDefinitionVersion {
    /// <p>A list of cores in the core definition version.</p>
    #[serde(rename = "Cores")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cores: Option<Vec<Core>>,
}

/// <p>Information needed to create a core definition.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct CreateCoreDefinitionRequest {
    /// <p>A client token used to correlate requests and responses.</p>
    #[serde(rename = "AmznClientToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amzn_client_token: Option<String>,
    /// <p>Information about the initial version of the core definition.</p>
    #[serde(rename = "InitialVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub initial_version: Option<CoreDefinitionVersion>,
    /// <p>The name of the core definition.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct CreateCoreDefinitionResponse {
    /// <p>The ARN of the definition.</p>
    #[serde(rename = "Arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p>The time, in milliseconds since the epoch, when the definition was created.</p>
    #[serde(rename = "CreationTimestamp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_timestamp: Option<String>,
    /// <p>The ID of the definition.</p>
    #[serde(rename = "Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// <p>The time, in milliseconds since the epoch, when the definition was last updated.</p>
    #[serde(rename = "LastUpdatedTimestamp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_timestamp: Option<String>,
    /// <p>The latest version of the definition.</p>
    #[serde(rename = "LatestVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latest_version: Option<String>,
    /// <p>The ARN of the latest version of the definition.</p>
    #[serde(rename = "LatestVersionArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latest_version_arn: Option<String>,
    /// <p>The name of the definition.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct CreateCoreDefinitionVersionRequest {
    /// <p>A client token used to correlate requests and responses.</p>
    #[serde(rename = "AmznClientToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amzn_client_token: Option<String>,
    /// <p>The ID of the core definition.</p>
    #[serde(rename = "CoreDefinitionId")]
    pub core_definition_id: String,
    /// <p>A list of cores in the core definition version.</p>
    #[serde(rename = "Cores")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cores: Option<Vec<Core>>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct CreateCoreDefinitionVersionResponse {
    /// <p>The ARN of the version.</p>
    #[serde(rename = "Arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p>The time, in milliseconds since the epoch, when the version was created.</p>
    #[serde(rename = "CreationTimestamp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_timestamp: Option<String>,
    /// <p>The ID of the version.</p>
    #[serde(rename = "Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// <p>The unique ID of the version.</p>
    #[serde(rename = "Version")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct CreateDeploymentRequest {
    /// <p>A client token used to correlate requests and responses.</p>
    #[serde(rename = "AmznClientToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amzn_client_token: Option<String>,
    /// <p>The ID of the deployment if you wish to redeploy a previous deployment.</p>
    #[serde(rename = "DeploymentId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deployment_id: Option<String>,
    /// <p>The type of deployment. When used in &#39;&#39;CreateDeployment&#39;&#39;, only &#39;&#39;NewDeployment&#39;&#39; and &#39;&#39;Redeployment&#39;&#39; are valid.</p>
    #[serde(rename = "DeploymentType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deployment_type: Option<String>,
    /// <p>The ID of the AWS Greengrass group.</p>
    #[serde(rename = "GroupId")]
    pub group_id: String,
    /// <p>The ID of the group version to be deployed.</p>
    #[serde(rename = "GroupVersionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_version_id: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct CreateDeploymentResponse {
    /// <p>The ARN of the deployment.</p>
    #[serde(rename = "DeploymentArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deployment_arn: Option<String>,
    /// <p>The ID of the deployment.</p>
    #[serde(rename = "DeploymentId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deployment_id: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct CreateDeviceDefinitionRequest {
    /// <p>A client token used to correlate requests and responses.</p>
    #[serde(rename = "AmznClientToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amzn_client_token: Option<String>,
    /// <p>Information about the initial version of the device definition.</p>
    #[serde(rename = "InitialVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub initial_version: Option<DeviceDefinitionVersion>,
    /// <p>The name of the device definition.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct CreateDeviceDefinitionResponse {
    /// <p>The ARN of the definition.</p>
    #[serde(rename = "Arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p>The time, in milliseconds since the epoch, when the definition was created.</p>
    #[serde(rename = "CreationTimestamp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_timestamp: Option<String>,
    /// <p>The ID of the definition.</p>
    #[serde(rename = "Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// <p>The time, in milliseconds since the epoch, when the definition was last updated.</p>
    #[serde(rename = "LastUpdatedTimestamp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_timestamp: Option<String>,
    /// <p>The latest version of the definition.</p>
    #[serde(rename = "LatestVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latest_version: Option<String>,
    /// <p>The ARN of the latest version of the definition.</p>
    #[serde(rename = "LatestVersionArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latest_version_arn: Option<String>,
    /// <p>The name of the definition.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct CreateDeviceDefinitionVersionRequest {
    /// <p>A client token used to correlate requests and responses.</p>
    #[serde(rename = "AmznClientToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amzn_client_token: Option<String>,
    /// <p>The ID of the device definition.</p>
    #[serde(rename = "DeviceDefinitionId")]
    pub device_definition_id: String,
    /// <p>A list of devices in the definition version.</p>
    #[serde(rename = "Devices")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub devices: Option<Vec<Device>>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct CreateDeviceDefinitionVersionResponse {
    /// <p>The ARN of the version.</p>
    #[serde(rename = "Arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p>The time, in milliseconds since the epoch, when the version was created.</p>
    #[serde(rename = "CreationTimestamp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_timestamp: Option<String>,
    /// <p>The ID of the version.</p>
    #[serde(rename = "Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// <p>The unique ID of the version.</p>
    #[serde(rename = "Version")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct CreateFunctionDefinitionRequest {
    /// <p>A client token used to correlate requests and responses.</p>
    #[serde(rename = "AmznClientToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amzn_client_token: Option<String>,
    /// <p>Information about the initial version of the function definition.</p>
    #[serde(rename = "InitialVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub initial_version: Option<FunctionDefinitionVersion>,
    /// <p>The name of the function definition.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct CreateFunctionDefinitionResponse {
    /// <p>The ARN of the definition.</p>
    #[serde(rename = "Arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p>The time, in milliseconds since the epoch, when the definition was created.</p>
    #[serde(rename = "CreationTimestamp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_timestamp: Option<String>,
    /// <p>The ID of the definition.</p>
    #[serde(rename = "Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// <p>The time, in milliseconds since the epoch, when the definition was last updated.</p>
    #[serde(rename = "LastUpdatedTimestamp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_timestamp: Option<String>,
    /// <p>The latest version of the definition.</p>
    #[serde(rename = "LatestVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latest_version: Option<String>,
    /// <p>The ARN of the latest version of the definition.</p>
    #[serde(rename = "LatestVersionArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latest_version_arn: Option<String>,
    /// <p>The name of the definition.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

/// <p>Information needed to create a function definition version.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct CreateFunctionDefinitionVersionRequest {
    /// <p>A client token used to correlate requests and responses.</p>
    #[serde(rename = "AmznClientToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amzn_client_token: Option<String>,
    /// <p>The ID of the Lambda function definition.</p>
    #[serde(rename = "FunctionDefinitionId")]
    pub function_definition_id: String,
    /// <p>A list of Lambda functions in this function definition version.</p>
    #[serde(rename = "Functions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub functions: Option<Vec<Function>>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct CreateFunctionDefinitionVersionResponse {
    /// <p>The ARN of the version.</p>
    #[serde(rename = "Arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p>The time, in milliseconds since the epoch, when the version was created.</p>
    #[serde(rename = "CreationTimestamp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_timestamp: Option<String>,
    /// <p>The ID of the version.</p>
    #[serde(rename = "Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// <p>The unique ID of the version.</p>
    #[serde(rename = "Version")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct CreateGroupCertificateAuthorityRequest {
    /// <p>A client token used to correlate requests and responses.</p>
    #[serde(rename = "AmznClientToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amzn_client_token: Option<String>,
    /// <p>The ID of the AWS Greengrass group.</p>
    #[serde(rename = "GroupId")]
    pub group_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct CreateGroupCertificateAuthorityResponse {
    /// <p>The ARN of the group certificate authority.</p>
    #[serde(rename = "GroupCertificateAuthorityArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_certificate_authority_arn: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct CreateGroupRequest {
    /// <p>A client token used to correlate requests and responses.</p>
    #[serde(rename = "AmznClientToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amzn_client_token: Option<String>,
    /// <p>Information about the initial version of the group.</p>
    #[serde(rename = "InitialVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub initial_version: Option<GroupVersion>,
    /// <p>The name of the group.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct CreateGroupResponse {
    /// <p>The ARN of the definition.</p>
    #[serde(rename = "Arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p>The time, in milliseconds since the epoch, when the definition was created.</p>
    #[serde(rename = "CreationTimestamp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_timestamp: Option<String>,
    /// <p>The ID of the definition.</p>
    #[serde(rename = "Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// <p>The time, in milliseconds since the epoch, when the definition was last updated.</p>
    #[serde(rename = "LastUpdatedTimestamp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_timestamp: Option<String>,
    /// <p>The latest version of the definition.</p>
    #[serde(rename = "LatestVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latest_version: Option<String>,
    /// <p>The ARN of the latest version of the definition.</p>
    #[serde(rename = "LatestVersionArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latest_version_arn: Option<String>,
    /// <p>The name of the definition.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct CreateGroupVersionRequest {
    /// <p>A client token used to correlate requests and responses.</p>
    #[serde(rename = "AmznClientToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amzn_client_token: Option<String>,
    /// <p>The ARN of the core definition version for this group.</p>
    #[serde(rename = "CoreDefinitionVersionArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub core_definition_version_arn: Option<String>,
    /// <p>The ARN of the device definition version for this group.</p>
    #[serde(rename = "DeviceDefinitionVersionArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_definition_version_arn: Option<String>,
    /// <p>The ARN of the function definition version for this group.</p>
    #[serde(rename = "FunctionDefinitionVersionArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub function_definition_version_arn: Option<String>,
    /// <p>The ID of the AWS Greengrass group.</p>
    #[serde(rename = "GroupId")]
    pub group_id: String,
    /// <p>The ARN of the logger definition version for this group.</p>
    #[serde(rename = "LoggerDefinitionVersionArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logger_definition_version_arn: Option<String>,
    /// <p>The resource definition version ARN for this group.</p>
    #[serde(rename = "ResourceDefinitionVersionArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_definition_version_arn: Option<String>,
    /// <p>The ARN of the subscription definition version for this group.</p>
    #[serde(rename = "SubscriptionDefinitionVersionArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscription_definition_version_arn: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct CreateGroupVersionResponse {
    /// <p>The ARN of the version.</p>
    #[serde(rename = "Arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p>The time, in milliseconds since the epoch, when the version was created.</p>
    #[serde(rename = "CreationTimestamp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_timestamp: Option<String>,
    /// <p>The ID of the version.</p>
    #[serde(rename = "Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// <p>The unique ID of the version.</p>
    #[serde(rename = "Version")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct CreateLoggerDefinitionRequest {
    /// <p>A client token used to correlate requests and responses.</p>
    #[serde(rename = "AmznClientToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amzn_client_token: Option<String>,
    /// <p>Information about the initial version of the logger definition.</p>
    #[serde(rename = "InitialVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub initial_version: Option<LoggerDefinitionVersion>,
    /// <p>The name of the logger definition.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct CreateLoggerDefinitionResponse {
    /// <p>The ARN of the definition.</p>
    #[serde(rename = "Arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p>The time, in milliseconds since the epoch, when the definition was created.</p>
    #[serde(rename = "CreationTimestamp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_timestamp: Option<String>,
    /// <p>The ID of the definition.</p>
    #[serde(rename = "Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// <p>The time, in milliseconds since the epoch, when the definition was last updated.</p>
    #[serde(rename = "LastUpdatedTimestamp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_timestamp: Option<String>,
    /// <p>The latest version of the definition.</p>
    #[serde(rename = "LatestVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latest_version: Option<String>,
    /// <p>The ARN of the latest version of the definition.</p>
    #[serde(rename = "LatestVersionArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latest_version_arn: Option<String>,
    /// <p>The name of the definition.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct CreateLoggerDefinitionVersionRequest {
    /// <p>A client token used to correlate requests and responses.</p>
    #[serde(rename = "AmznClientToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amzn_client_token: Option<String>,
    /// <p>The ID of the logger definition.</p>
    #[serde(rename = "LoggerDefinitionId")]
    pub logger_definition_id: String,
    /// <p>A list of loggers.</p>
    #[serde(rename = "Loggers")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub loggers: Option<Vec<Logger>>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct CreateLoggerDefinitionVersionResponse {
    /// <p>The ARN of the version.</p>
    #[serde(rename = "Arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p>The time, in milliseconds since the epoch, when the version was created.</p>
    #[serde(rename = "CreationTimestamp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_timestamp: Option<String>,
    /// <p>The ID of the version.</p>
    #[serde(rename = "Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// <p>The unique ID of the version.</p>
    #[serde(rename = "Version")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct CreateResourceDefinitionRequest {
    /// <p>A client token used to correlate requests and responses.</p>
    #[serde(rename = "AmznClientToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amzn_client_token: Option<String>,
    /// <p>Information about the initial version of the resource definition.</p>
    #[serde(rename = "InitialVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub initial_version: Option<ResourceDefinitionVersion>,
    /// <p>The name of the resource definition.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct CreateResourceDefinitionResponse {
    /// <p>The ARN of the definition.</p>
    #[serde(rename = "Arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p>The time, in milliseconds since the epoch, when the definition was created.</p>
    #[serde(rename = "CreationTimestamp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_timestamp: Option<String>,
    /// <p>The ID of the definition.</p>
    #[serde(rename = "Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// <p>The time, in milliseconds since the epoch, when the definition was last updated.</p>
    #[serde(rename = "LastUpdatedTimestamp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_timestamp: Option<String>,
    /// <p>The latest version of the definition.</p>
    #[serde(rename = "LatestVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latest_version: Option<String>,
    /// <p>The ARN of the latest version of the definition.</p>
    #[serde(rename = "LatestVersionArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latest_version_arn: Option<String>,
    /// <p>The name of the definition.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct CreateResourceDefinitionVersionRequest {
    /// <p>A client token used to correlate requests and responses.</p>
    #[serde(rename = "AmznClientToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amzn_client_token: Option<String>,
    /// <p>The ID of the resource definition.</p>
    #[serde(rename = "ResourceDefinitionId")]
    pub resource_definition_id: String,
    /// <p>A list of resources.</p>
    #[serde(rename = "Resources")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resources: Option<Vec<Resource>>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct CreateResourceDefinitionVersionResponse {
    /// <p>The ARN of the version.</p>
    #[serde(rename = "Arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p>The time, in milliseconds since the epoch, when the version was created.</p>
    #[serde(rename = "CreationTimestamp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_timestamp: Option<String>,
    /// <p>The ID of the version.</p>
    #[serde(rename = "Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// <p>The unique ID of the version.</p>
    #[serde(rename = "Version")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct CreateSoftwareUpdateJobRequest {
    /// <p>A client token used to correlate requests and responses.</p>
    #[serde(rename = "AmznClientToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amzn_client_token: Option<String>,
    #[serde(rename = "S3UrlSignerRole")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_url_signer_role: Option<String>,
    #[serde(rename = "SoftwareToUpdate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub software_to_update: Option<String>,
    #[serde(rename = "UpdateAgentLogLevel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_agent_log_level: Option<String>,
    #[serde(rename = "UpdateTargets")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_targets: Option<Vec<String>>,
    #[serde(rename = "UpdateTargetsArchitecture")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_targets_architecture: Option<String>,
    #[serde(rename = "UpdateTargetsOperatingSystem")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_targets_operating_system: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct CreateSoftwareUpdateJobResponse {
    /// <p>The IoT Job ARN corresponding to this update.</p>
    #[serde(rename = "IotJobArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iot_job_arn: Option<String>,
    /// <p>The IoT Job Id corresponding to this update.</p>
    #[serde(rename = "IotJobId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iot_job_id: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct CreateSubscriptionDefinitionRequest {
    /// <p>A client token used to correlate requests and responses.</p>
    #[serde(rename = "AmznClientToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amzn_client_token: Option<String>,
    /// <p>Information about the initial version of the subscription definition.</p>
    #[serde(rename = "InitialVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub initial_version: Option<SubscriptionDefinitionVersion>,
    /// <p>The name of the subscription definition.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct CreateSubscriptionDefinitionResponse {
    /// <p>The ARN of the definition.</p>
    #[serde(rename = "Arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p>The time, in milliseconds since the epoch, when the definition was created.</p>
    #[serde(rename = "CreationTimestamp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_timestamp: Option<String>,
    /// <p>The ID of the definition.</p>
    #[serde(rename = "Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// <p>The time, in milliseconds since the epoch, when the definition was last updated.</p>
    #[serde(rename = "LastUpdatedTimestamp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_timestamp: Option<String>,
    /// <p>The latest version of the definition.</p>
    #[serde(rename = "LatestVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latest_version: Option<String>,
    /// <p>The ARN of the latest version of the definition.</p>
    #[serde(rename = "LatestVersionArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latest_version_arn: Option<String>,
    /// <p>The name of the definition.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct CreateSubscriptionDefinitionVersionRequest {
    /// <p>A client token used to correlate requests and responses.</p>
    #[serde(rename = "AmznClientToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amzn_client_token: Option<String>,
    /// <p>The ID of the subscription definition.</p>
    #[serde(rename = "SubscriptionDefinitionId")]
    pub subscription_definition_id: String,
    /// <p>A list of subscriptions.</p>
    #[serde(rename = "Subscriptions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscriptions: Option<Vec<Subscription>>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct CreateSubscriptionDefinitionVersionResponse {
    /// <p>The ARN of the version.</p>
    #[serde(rename = "Arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p>The time, in milliseconds since the epoch, when the version was created.</p>
    #[serde(rename = "CreationTimestamp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_timestamp: Option<String>,
    /// <p>The ID of the version.</p>
    #[serde(rename = "Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// <p>The unique ID of the version.</p>
    #[serde(rename = "Version")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

/// <p>Information about a definition.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct DefinitionInformation {
    /// <p>The ARN of the definition.</p>
    #[serde(rename = "Arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p>The time, in milliseconds since the epoch, when the definition was created.</p>
    #[serde(rename = "CreationTimestamp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_timestamp: Option<String>,
    /// <p>The ID of the definition.</p>
    #[serde(rename = "Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// <p>The time, in milliseconds since the epoch, when the definition was last updated.</p>
    #[serde(rename = "LastUpdatedTimestamp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_timestamp: Option<String>,
    /// <p>The latest version of the definition.</p>
    #[serde(rename = "LatestVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latest_version: Option<String>,
    /// <p>The ARN of the latest version of the definition.</p>
    #[serde(rename = "LatestVersionArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latest_version_arn: Option<String>,
    /// <p>The name of the definition.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DeleteCoreDefinitionRequest {
    /// <p>The ID of the core definition.</p>
    #[serde(rename = "CoreDefinitionId")]
    pub core_definition_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct DeleteCoreDefinitionResponse {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DeleteDeviceDefinitionRequest {
    /// <p>The ID of the device definition.</p>
    #[serde(rename = "DeviceDefinitionId")]
    pub device_definition_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct DeleteDeviceDefinitionResponse {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DeleteFunctionDefinitionRequest {
    /// <p>The ID of the Lambda function definition.</p>
    #[serde(rename = "FunctionDefinitionId")]
    pub function_definition_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct DeleteFunctionDefinitionResponse {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DeleteGroupRequest {
    /// <p>The ID of the AWS Greengrass group.</p>
    #[serde(rename = "GroupId")]
    pub group_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct DeleteGroupResponse {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DeleteLoggerDefinitionRequest {
    /// <p>The ID of the logger definition.</p>
    #[serde(rename = "LoggerDefinitionId")]
    pub logger_definition_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct DeleteLoggerDefinitionResponse {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DeleteResourceDefinitionRequest {
    /// <p>The ID of the resource definition.</p>
    #[serde(rename = "ResourceDefinitionId")]
    pub resource_definition_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct DeleteResourceDefinitionResponse {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DeleteSubscriptionDefinitionRequest {
    /// <p>The ID of the subscription definition.</p>
    #[serde(rename = "SubscriptionDefinitionId")]
    pub subscription_definition_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct DeleteSubscriptionDefinitionResponse {}

/// <p>Information about a deployment.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct Deployment {
    /// <p>The time, in milliseconds since the epoch, when the deployment was created.</p>
    #[serde(rename = "CreatedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    /// <p>The ARN of the deployment.</p>
    #[serde(rename = "DeploymentArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deployment_arn: Option<String>,
    /// <p>The ID of the deployment.</p>
    #[serde(rename = "DeploymentId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deployment_id: Option<String>,
    /// <p>The type of the deployment.</p>
    #[serde(rename = "DeploymentType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deployment_type: Option<String>,
    /// <p>The ARN of the group for this deployment.</p>
    #[serde(rename = "GroupArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_arn: Option<String>,
}

/// <p>Information about a device.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Device {
    /// <p>The ARN of the certificate associated with the device.</p>
    #[serde(rename = "CertificateArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate_arn: Option<String>,
    /// <p>The ID of the device.</p>
    #[serde(rename = "Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// <p>If true, the device&#39;s local shadow will be automatically synced with the cloud.</p>
    #[serde(rename = "SyncShadow")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sync_shadow: Option<bool>,
    /// <p>The thing ARN of the device.</p>
    #[serde(rename = "ThingArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thing_arn: Option<String>,
}

/// <p>Information about a device definition version.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DeviceDefinitionVersion {
    /// <p>A list of devices in the definition version.</p>
    #[serde(rename = "Devices")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub devices: Option<Vec<Device>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DisassociateRoleFromGroupRequest {
    /// <p>The ID of the AWS Greengrass group.</p>
    #[serde(rename = "GroupId")]
    pub group_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct DisassociateRoleFromGroupResponse {
    /// <p>The time, in milliseconds since the epoch, when the role was disassociated from the group.</p>
    #[serde(rename = "DisassociatedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disassociated_at: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DisassociateServiceRoleFromAccountRequest {}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct DisassociateServiceRoleFromAccountResponse {
    /// <p>The time when the service role was disassociated from the account.</p>
    #[serde(rename = "DisassociatedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disassociated_at: Option<String>,
}

/// <p>Empty</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct Empty {}

/// <p>Details about the error.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct ErrorDetail {
    /// <p>A detailed error code.</p>
    #[serde(rename = "DetailedErrorCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detailed_error_code: Option<String>,
    /// <p>A detailed error message.</p>
    #[serde(rename = "DetailedErrorMessage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detailed_error_message: Option<String>,
}

/// <p>Information about a Lambda function.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Function {
    /// <p>The ARN of the Lambda function.</p>
    #[serde(rename = "FunctionArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub function_arn: Option<String>,
    /// <p>The configuration of the Lambda function.</p>
    #[serde(rename = "FunctionConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub function_configuration: Option<FunctionConfiguration>,
    /// <p>The ID of the Lambda function.</p>
    #[serde(rename = "Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}

/// <p>The configuration of the Lambda function.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FunctionConfiguration {
    /// <p>The expected encoding type of the input payload for the function. The default is &#39;&#39;json&#39;&#39;.</p>
    #[serde(rename = "EncodingType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encoding_type: Option<String>,
    /// <p>The environment configuration of the function.</p>
    #[serde(rename = "Environment")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub environment: Option<FunctionConfigurationEnvironment>,
    /// <p>The execution arguments.</p>
    #[serde(rename = "ExecArgs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exec_args: Option<String>,
    /// <p>The name of the function executable.</p>
    #[serde(rename = "Executable")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub executable: Option<String>,
    /// <p>The memory size, in KB, which the function requires.</p>
    #[serde(rename = "MemorySize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub memory_size: Option<i64>,
    /// <p>True if the function is pinned. Pinned means the function is long-lived and starts when the core starts.</p>
    #[serde(rename = "Pinned")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pinned: Option<bool>,
    /// <p>The allowed function execution time, after which Lambda should terminate the function. This timeout still applies to pinned lambdas for each request.</p>
    #[serde(rename = "Timeout")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeout: Option<i64>,
}

/// <p>The environment configuration of the function.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FunctionConfigurationEnvironment {
    /// <p>If true, the Lambda function is allowed to access the host&#39;s /sys folder. Use this when the Lambda function needs to read device information from /sys.</p>
    #[serde(rename = "AccessSysfs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_sysfs: Option<bool>,
    /// <p>A list of the resources, with their permissions, to which the Lambda function will be granted access. A Lambda function can have at most 10 resources.</p>
    #[serde(rename = "ResourceAccessPolicies")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_access_policies: Option<Vec<ResourceAccessPolicy>>,
    /// <p>Environment variables for the Lambda function&#39;s configuration.</p>
    #[serde(rename = "Variables")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub variables: Option<::std::collections::HashMap<String, String>>,
}

/// <p>Information about a function definition version.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FunctionDefinitionVersion {
    /// <p>A list of Lambda functions in this function definition version.</p>
    #[serde(rename = "Functions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub functions: Option<Vec<Function>>,
}

/// <p>General error information.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct GeneralError {
    /// <p>Details about the error.</p>
    pub error_details: Option<Vec<ErrorDetail>>,
    /// <p>A message containing information about the error.</p>
    pub message: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetAssociatedRoleRequest {
    /// <p>The ID of the AWS Greengrass group.</p>
    #[serde(rename = "GroupId")]
    pub group_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct GetAssociatedRoleResponse {
    /// <p>The time when the role was associated with the group.</p>
    #[serde(rename = "AssociatedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub associated_at: Option<String>,
    /// <p>The ARN of the role that is associated with the group.</p>
    #[serde(rename = "RoleArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_arn: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetConnectivityInfoRequest {
    /// <p>The thing name.</p>
    #[serde(rename = "ThingName")]
    pub thing_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct GetConnectivityInfoResponse {
    /// <p>Connectivity info list.</p>
    #[serde(rename = "ConnectivityInfo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connectivity_info: Option<Vec<ConnectivityInfo>>,
    /// <p>A message about the connectivity info request.</p>
    #[serde(rename = "Message")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetCoreDefinitionRequest {
    /// <p>The ID of the core definition.</p>
    #[serde(rename = "CoreDefinitionId")]
    pub core_definition_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct GetCoreDefinitionResponse {
    /// <p>The ARN of the definition.</p>
    #[serde(rename = "Arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p>The time, in milliseconds since the epoch, when the definition was created.</p>
    #[serde(rename = "CreationTimestamp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_timestamp: Option<String>,
    /// <p>The ID of the definition.</p>
    #[serde(rename = "Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// <p>The time, in milliseconds since the epoch, when the definition was last updated.</p>
    #[serde(rename = "LastUpdatedTimestamp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_timestamp: Option<String>,
    /// <p>The latest version of the definition.</p>
    #[serde(rename = "LatestVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latest_version: Option<String>,
    /// <p>The ARN of the latest version of the definition.</p>
    #[serde(rename = "LatestVersionArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latest_version_arn: Option<String>,
    /// <p>The name of the definition.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetCoreDefinitionVersionRequest {
    /// <p>The ID of the core definition.</p>
    #[serde(rename = "CoreDefinitionId")]
    pub core_definition_id: String,
    /// <p>The ID of the core definition version.</p>
    #[serde(rename = "CoreDefinitionVersionId")]
    pub core_definition_version_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct GetCoreDefinitionVersionResponse {
    /// <p>The ARN of the core definition version.</p>
    #[serde(rename = "Arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p>The time, in milliseconds since the epoch, when the core definition version was created.</p>
    #[serde(rename = "CreationTimestamp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_timestamp: Option<String>,
    /// <p>Information about the core definition version.</p>
    #[serde(rename = "Definition")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub definition: Option<CoreDefinitionVersion>,
    /// <p>The ID of the core definition version.</p>
    #[serde(rename = "Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// <p>The version of the core definition version.</p>
    #[serde(rename = "Version")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetDeploymentStatusRequest {
    /// <p>The ID of the deployment.</p>
    #[serde(rename = "DeploymentId")]
    pub deployment_id: String,
    /// <p>The ID of the AWS Greengrass group.</p>
    #[serde(rename = "GroupId")]
    pub group_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct GetDeploymentStatusResponse {
    /// <p>The status of the deployment.</p>
    #[serde(rename = "DeploymentStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deployment_status: Option<String>,
    /// <p>The type of the deployment.</p>
    #[serde(rename = "DeploymentType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deployment_type: Option<String>,
    /// <p>Error details</p>
    #[serde(rename = "ErrorDetails")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_details: Option<Vec<ErrorDetail>>,
    /// <p>Error message</p>
    #[serde(rename = "ErrorMessage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
    /// <p>The time, in milliseconds since the epoch, when the deployment status was updated.</p>
    #[serde(rename = "UpdatedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetDeviceDefinitionRequest {
    /// <p>The ID of the device definition.</p>
    #[serde(rename = "DeviceDefinitionId")]
    pub device_definition_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct GetDeviceDefinitionResponse {
    /// <p>The ARN of the definition.</p>
    #[serde(rename = "Arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p>The time, in milliseconds since the epoch, when the definition was created.</p>
    #[serde(rename = "CreationTimestamp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_timestamp: Option<String>,
    /// <p>The ID of the definition.</p>
    #[serde(rename = "Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// <p>The time, in milliseconds since the epoch, when the definition was last updated.</p>
    #[serde(rename = "LastUpdatedTimestamp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_timestamp: Option<String>,
    /// <p>The latest version of the definition.</p>
    #[serde(rename = "LatestVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latest_version: Option<String>,
    /// <p>The ARN of the latest version of the definition.</p>
    #[serde(rename = "LatestVersionArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latest_version_arn: Option<String>,
    /// <p>The name of the definition.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetDeviceDefinitionVersionRequest {
    /// <p>The ID of the device definition.</p>
    #[serde(rename = "DeviceDefinitionId")]
    pub device_definition_id: String,
    /// <p>The ID of the device definition version.</p>
    #[serde(rename = "DeviceDefinitionVersionId")]
    pub device_definition_version_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct GetDeviceDefinitionVersionResponse {
    /// <p>The ARN of the device definition version.</p>
    #[serde(rename = "Arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p>The time, in milliseconds since the epoch, when the device definition version was created.</p>
    #[serde(rename = "CreationTimestamp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_timestamp: Option<String>,
    /// <p>Information about the device definition version.</p>
    #[serde(rename = "Definition")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub definition: Option<DeviceDefinitionVersion>,
    /// <p>The ID of the device definition version.</p>
    #[serde(rename = "Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// <p>The version of the device definition version.</p>
    #[serde(rename = "Version")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetFunctionDefinitionRequest {
    /// <p>The ID of the Lambda function definition.</p>
    #[serde(rename = "FunctionDefinitionId")]
    pub function_definition_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct GetFunctionDefinitionResponse {
    /// <p>The ARN of the definition.</p>
    #[serde(rename = "Arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p>The time, in milliseconds since the epoch, when the definition was created.</p>
    #[serde(rename = "CreationTimestamp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_timestamp: Option<String>,
    /// <p>The ID of the definition.</p>
    #[serde(rename = "Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// <p>The time, in milliseconds since the epoch, when the definition was last updated.</p>
    #[serde(rename = "LastUpdatedTimestamp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_timestamp: Option<String>,
    /// <p>The latest version of the definition.</p>
    #[serde(rename = "LatestVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latest_version: Option<String>,
    /// <p>The ARN of the latest version of the definition.</p>
    #[serde(rename = "LatestVersionArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latest_version_arn: Option<String>,
    /// <p>The name of the definition.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetFunctionDefinitionVersionRequest {
    /// <p>The ID of the Lambda function definition.</p>
    #[serde(rename = "FunctionDefinitionId")]
    pub function_definition_id: String,
    /// <p>The ID of the function definition version.</p>
    #[serde(rename = "FunctionDefinitionVersionId")]
    pub function_definition_version_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct GetFunctionDefinitionVersionResponse {
    /// <p>The ARN of the function definition version.</p>
    #[serde(rename = "Arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p>The time, in milliseconds since the epoch, when the function definition version was created.</p>
    #[serde(rename = "CreationTimestamp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_timestamp: Option<String>,
    /// <p>Information on the definition.</p>
    #[serde(rename = "Definition")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub definition: Option<FunctionDefinitionVersion>,
    /// <p>The ID of the function definition version.</p>
    #[serde(rename = "Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// <p>The version of the function definition version.</p>
    #[serde(rename = "Version")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetGroupCertificateAuthorityRequest {
    /// <p>The ID of the certificate authority.</p>
    #[serde(rename = "CertificateAuthorityId")]
    pub certificate_authority_id: String,
    /// <p>The ID of the AWS Greengrass group.</p>
    #[serde(rename = "GroupId")]
    pub group_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct GetGroupCertificateAuthorityResponse {
    /// <p>The ARN of the certificate authority for the group.</p>
    #[serde(rename = "GroupCertificateAuthorityArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_certificate_authority_arn: Option<String>,
    /// <p>The ID of the certificate authority for the group.</p>
    #[serde(rename = "GroupCertificateAuthorityId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_certificate_authority_id: Option<String>,
    /// <p>The PEM encoded certificate for the group.</p>
    #[serde(rename = "PemEncodedCertificate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pem_encoded_certificate: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetGroupCertificateConfigurationRequest {
    /// <p>The ID of the AWS Greengrass group.</p>
    #[serde(rename = "GroupId")]
    pub group_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct GetGroupCertificateConfigurationResponse {
    /// <p>The amount of time remaining before the certificate authority expires, in milliseconds.</p>
    #[serde(rename = "CertificateAuthorityExpiryInMilliseconds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate_authority_expiry_in_milliseconds: Option<String>,
    /// <p>The amount of time remaining before the certificate expires, in milliseconds.</p>
    #[serde(rename = "CertificateExpiryInMilliseconds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate_expiry_in_milliseconds: Option<String>,
    /// <p>The ID of the group certificate configuration.</p>
    #[serde(rename = "GroupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_id: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetGroupRequest {
    /// <p>The ID of the AWS Greengrass group.</p>
    #[serde(rename = "GroupId")]
    pub group_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct GetGroupResponse {
    /// <p>The ARN of the definition.</p>
    #[serde(rename = "Arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p>The time, in milliseconds since the epoch, when the definition was created.</p>
    #[serde(rename = "CreationTimestamp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_timestamp: Option<String>,
    /// <p>The ID of the definition.</p>
    #[serde(rename = "Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// <p>The time, in milliseconds since the epoch, when the definition was last updated.</p>
    #[serde(rename = "LastUpdatedTimestamp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_timestamp: Option<String>,
    /// <p>The latest version of the definition.</p>
    #[serde(rename = "LatestVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latest_version: Option<String>,
    /// <p>The ARN of the latest version of the definition.</p>
    #[serde(rename = "LatestVersionArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latest_version_arn: Option<String>,
    /// <p>The name of the definition.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetGroupVersionRequest {
    /// <p>The ID of the AWS Greengrass group.</p>
    #[serde(rename = "GroupId")]
    pub group_id: String,
    /// <p>The ID of the group version.</p>
    #[serde(rename = "GroupVersionId")]
    pub group_version_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct GetGroupVersionResponse {
    /// <p>The ARN of the group version.</p>
    #[serde(rename = "Arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p>The time, in milliseconds since the epoch, when the group version was created.</p>
    #[serde(rename = "CreationTimestamp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_timestamp: Option<String>,
    /// <p>Information about the group version definition.</p>
    #[serde(rename = "Definition")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub definition: Option<GroupVersion>,
    /// <p>The ID of the group version.</p>
    #[serde(rename = "Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// <p>The unique ID for the version of the group.</p>
    #[serde(rename = "Version")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetLoggerDefinitionRequest {
    /// <p>The ID of the logger definition.</p>
    #[serde(rename = "LoggerDefinitionId")]
    pub logger_definition_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct GetLoggerDefinitionResponse {
    /// <p>The ARN of the definition.</p>
    #[serde(rename = "Arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p>The time, in milliseconds since the epoch, when the definition was created.</p>
    #[serde(rename = "CreationTimestamp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_timestamp: Option<String>,
    /// <p>The ID of the definition.</p>
    #[serde(rename = "Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// <p>The time, in milliseconds since the epoch, when the definition was last updated.</p>
    #[serde(rename = "LastUpdatedTimestamp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_timestamp: Option<String>,
    /// <p>The latest version of the definition.</p>
    #[serde(rename = "LatestVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latest_version: Option<String>,
    /// <p>The ARN of the latest version of the definition.</p>
    #[serde(rename = "LatestVersionArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latest_version_arn: Option<String>,
    /// <p>The name of the definition.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetLoggerDefinitionVersionRequest {
    /// <p>The ID of the logger definition.</p>
    #[serde(rename = "LoggerDefinitionId")]
    pub logger_definition_id: String,
    /// <p>The ID of the logger definition version.</p>
    #[serde(rename = "LoggerDefinitionVersionId")]
    pub logger_definition_version_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct GetLoggerDefinitionVersionResponse {
    /// <p>The ARN of the logger definition version.</p>
    #[serde(rename = "Arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p>The time, in milliseconds since the epoch, when the logger definition version was created.</p>
    #[serde(rename = "CreationTimestamp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_timestamp: Option<String>,
    /// <p>Information about the logger definition version.</p>
    #[serde(rename = "Definition")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub definition: Option<LoggerDefinitionVersion>,
    /// <p>The ID of the logger definition version.</p>
    #[serde(rename = "Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// <p>The version of the logger definition version.</p>
    #[serde(rename = "Version")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetResourceDefinitionRequest {
    /// <p>The ID of the resource definition.</p>
    #[serde(rename = "ResourceDefinitionId")]
    pub resource_definition_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct GetResourceDefinitionResponse {
    /// <p>The ARN of the definition.</p>
    #[serde(rename = "Arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p>The time, in milliseconds since the epoch, when the definition was created.</p>
    #[serde(rename = "CreationTimestamp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_timestamp: Option<String>,
    /// <p>The ID of the definition.</p>
    #[serde(rename = "Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// <p>The time, in milliseconds since the epoch, when the definition was last updated.</p>
    #[serde(rename = "LastUpdatedTimestamp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_timestamp: Option<String>,
    /// <p>The latest version of the definition.</p>
    #[serde(rename = "LatestVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latest_version: Option<String>,
    /// <p>The ARN of the latest version of the definition.</p>
    #[serde(rename = "LatestVersionArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latest_version_arn: Option<String>,
    /// <p>The name of the definition.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetResourceDefinitionVersionRequest {
    /// <p>The ID of the resource definition.</p>
    #[serde(rename = "ResourceDefinitionId")]
    pub resource_definition_id: String,
    /// <p>The ID of the resource definition version.</p>
    #[serde(rename = "ResourceDefinitionVersionId")]
    pub resource_definition_version_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct GetResourceDefinitionVersionResponse {
    /// <p>Arn of the resource definition version.</p>
    #[serde(rename = "Arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p>The time, in milliseconds since the epoch, when the resource definition version was created.</p>
    #[serde(rename = "CreationTimestamp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_timestamp: Option<String>,
    /// <p>Information about the definition.</p>
    #[serde(rename = "Definition")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub definition: Option<ResourceDefinitionVersion>,
    /// <p>The ID of the resource definition version.</p>
    #[serde(rename = "Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// <p>The version of the resource definition version.</p>
    #[serde(rename = "Version")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetServiceRoleForAccountRequest {}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct GetServiceRoleForAccountResponse {
    /// <p>The time when the service role was associated with the account.</p>
    #[serde(rename = "AssociatedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub associated_at: Option<String>,
    /// <p>The ARN of the role which is associated with the account.</p>
    #[serde(rename = "RoleArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_arn: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetSubscriptionDefinitionRequest {
    /// <p>The ID of the subscription definition.</p>
    #[serde(rename = "SubscriptionDefinitionId")]
    pub subscription_definition_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct GetSubscriptionDefinitionResponse {
    /// <p>The ARN of the definition.</p>
    #[serde(rename = "Arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p>The time, in milliseconds since the epoch, when the definition was created.</p>
    #[serde(rename = "CreationTimestamp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_timestamp: Option<String>,
    /// <p>The ID of the definition.</p>
    #[serde(rename = "Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// <p>The time, in milliseconds since the epoch, when the definition was last updated.</p>
    #[serde(rename = "LastUpdatedTimestamp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_timestamp: Option<String>,
    /// <p>The latest version of the definition.</p>
    #[serde(rename = "LatestVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latest_version: Option<String>,
    /// <p>The ARN of the latest version of the definition.</p>
    #[serde(rename = "LatestVersionArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latest_version_arn: Option<String>,
    /// <p>The name of the definition.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetSubscriptionDefinitionVersionRequest {
    /// <p>The ID of the subscription definition.</p>
    #[serde(rename = "SubscriptionDefinitionId")]
    pub subscription_definition_id: String,
    /// <p>The ID of the subscription definition version.</p>
    #[serde(rename = "SubscriptionDefinitionVersionId")]
    pub subscription_definition_version_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct GetSubscriptionDefinitionVersionResponse {
    /// <p>The ARN of the subscription definition version.</p>
    #[serde(rename = "Arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p>The time, in milliseconds since the epoch, when the subscription definition version was created.</p>
    #[serde(rename = "CreationTimestamp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_timestamp: Option<String>,
    /// <p>Information about the subscription definition version.</p>
    #[serde(rename = "Definition")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub definition: Option<SubscriptionDefinitionVersion>,
    /// <p>The ID of the subscription definition version.</p>
    #[serde(rename = "Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// <p>The version of the subscription definition version.</p>
    #[serde(rename = "Version")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

/// <p>Information about a certificate authority for a group.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct GroupCertificateAuthorityProperties {
    /// <p>The ARN of the certificate authority for the group.</p>
    #[serde(rename = "GroupCertificateAuthorityArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_certificate_authority_arn: Option<String>,
    /// <p>The ID of the certificate authority for the group.</p>
    #[serde(rename = "GroupCertificateAuthorityId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_certificate_authority_id: Option<String>,
}

/// <p>Information about a group certificate configuration.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct GroupCertificateConfiguration {
    /// <p>The amount of time remaining before the certificate authority expires, in milliseconds.</p>
    pub certificate_authority_expiry_in_milliseconds: Option<String>,
    /// <p>The amount of time remaining before the certificate expires, in milliseconds.</p>
    pub certificate_expiry_in_milliseconds: Option<String>,
    /// <p>The ID of the group certificate configuration.</p>
    pub group_id: Option<String>,
}

/// <p>Information about a group.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct GroupInformation {
    /// <p>The ARN of the group.</p>
    #[serde(rename = "Arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p>The time, in milliseconds since the epoch, when the group was created.</p>
    #[serde(rename = "CreationTimestamp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_timestamp: Option<String>,
    /// <p>The ID of the group.</p>
    #[serde(rename = "Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// <p>The time, in milliseconds since the epoch, when the group was last updated.</p>
    #[serde(rename = "LastUpdatedTimestamp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_timestamp: Option<String>,
    /// <p>The latest version of the group.</p>
    #[serde(rename = "LatestVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latest_version: Option<String>,
    /// <p>The ARN of the latest version of the group.</p>
    #[serde(rename = "LatestVersionArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latest_version_arn: Option<String>,
    /// <p>The name of the group.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

/// <p>Group owner related settings for local resources.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GroupOwnerSetting {
    /// <p>If true, GreenGrass automatically adds the specified Linux OS group owner of the resource to the Lambda process privileges. Thus the Lambda process will have the file access permissions of the added Linux group.</p>
    #[serde(rename = "AutoAddGroupOwner")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_add_group_owner: Option<bool>,
    /// <p>The name of the Linux OS group whose privileges will be added to the Lambda process. This field is optional.</p>
    #[serde(rename = "GroupOwner")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_owner: Option<String>,
}

/// <p>Information about a group version.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GroupVersion {
    /// <p>The ARN of the core definition version for this group.</p>
    #[serde(rename = "CoreDefinitionVersionArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub core_definition_version_arn: Option<String>,
    /// <p>The ARN of the device definition version for this group.</p>
    #[serde(rename = "DeviceDefinitionVersionArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_definition_version_arn: Option<String>,
    /// <p>The ARN of the function definition version for this group.</p>
    #[serde(rename = "FunctionDefinitionVersionArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub function_definition_version_arn: Option<String>,
    /// <p>The ARN of the logger definition version for this group.</p>
    #[serde(rename = "LoggerDefinitionVersionArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logger_definition_version_arn: Option<String>,
    /// <p>The resource definition version ARN for this group.</p>
    #[serde(rename = "ResourceDefinitionVersionArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_definition_version_arn: Option<String>,
    /// <p>The ARN of the subscription definition version for this group.</p>
    #[serde(rename = "SubscriptionDefinitionVersionArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscription_definition_version_arn: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ListCoreDefinitionVersionsRequest {
    /// <p>The ID of the core definition.</p>
    #[serde(rename = "CoreDefinitionId")]
    pub core_definition_id: String,
    /// <p>The maximum number of results to be returned per request.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<String>,
    /// <p>The token for the next set of results, or &#39;&#39;null&#39;&#39; if there are no additional results.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct ListCoreDefinitionVersionsResponse {
    /// <p>The token for the next set of results, or &#39;&#39;null&#39;&#39; if there are no additional results.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>Information about a version.</p>
    #[serde(rename = "Versions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub versions: Option<Vec<VersionInformation>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ListCoreDefinitionsRequest {
    /// <p>The maximum number of results to be returned per request.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<String>,
    /// <p>The token for the next set of results, or &#39;&#39;null&#39;&#39; if there are no additional results.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct ListCoreDefinitionsResponse {
    /// <p>Information about a definition.</p>
    #[serde(rename = "Definitions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub definitions: Option<Vec<DefinitionInformation>>,
    /// <p>The token for the next set of results, or &#39;&#39;null&#39;&#39; if there are no additional results.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

/// <p>A list of definitions.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct ListDefinitionsResponse {
    /// <p>Information about a definition.</p>
    pub definitions: Option<Vec<DefinitionInformation>>,
    /// <p>The token for the next set of results, or &#39;&#39;null&#39;&#39; if there are no additional results.</p>
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ListDeploymentsRequest {
    /// <p>The ID of the AWS Greengrass group.</p>
    #[serde(rename = "GroupId")]
    pub group_id: String,
    /// <p>The maximum number of results to be returned per request.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<String>,
    /// <p>The token for the next set of results, or &#39;&#39;null&#39;&#39; if there are no additional results.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct ListDeploymentsResponse {
    /// <p>A list of deployments for the requested groups.</p>
    #[serde(rename = "Deployments")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deployments: Option<Vec<Deployment>>,
    /// <p>The token for the next set of results, or &#39;&#39;null&#39;&#39; if there are no additional results.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ListDeviceDefinitionVersionsRequest {
    /// <p>The ID of the device definition.</p>
    #[serde(rename = "DeviceDefinitionId")]
    pub device_definition_id: String,
    /// <p>The maximum number of results to be returned per request.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<String>,
    /// <p>The token for the next set of results, or &#39;&#39;null&#39;&#39; if there are no additional results.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct ListDeviceDefinitionVersionsResponse {
    /// <p>The token for the next set of results, or &#39;&#39;null&#39;&#39; if there are no additional results.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>Information about a version.</p>
    #[serde(rename = "Versions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub versions: Option<Vec<VersionInformation>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ListDeviceDefinitionsRequest {
    /// <p>The maximum number of results to be returned per request.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<String>,
    /// <p>The token for the next set of results, or &#39;&#39;null&#39;&#39; if there are no additional results.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct ListDeviceDefinitionsResponse {
    /// <p>Information about a definition.</p>
    #[serde(rename = "Definitions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub definitions: Option<Vec<DefinitionInformation>>,
    /// <p>The token for the next set of results, or &#39;&#39;null&#39;&#39; if there are no additional results.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ListFunctionDefinitionVersionsRequest {
    /// <p>The ID of the Lambda function definition.</p>
    #[serde(rename = "FunctionDefinitionId")]
    pub function_definition_id: String,
    /// <p>The maximum number of results to be returned per request.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<String>,
    /// <p>The token for the next set of results, or &#39;&#39;null&#39;&#39; if there are no additional results.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct ListFunctionDefinitionVersionsResponse {
    /// <p>The token for the next set of results, or &#39;&#39;null&#39;&#39; if there are no additional results.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>Information about a version.</p>
    #[serde(rename = "Versions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub versions: Option<Vec<VersionInformation>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ListFunctionDefinitionsRequest {
    /// <p>The maximum number of results to be returned per request.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<String>,
    /// <p>The token for the next set of results, or &#39;&#39;null&#39;&#39; if there are no additional results.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct ListFunctionDefinitionsResponse {
    /// <p>Information about a definition.</p>
    #[serde(rename = "Definitions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub definitions: Option<Vec<DefinitionInformation>>,
    /// <p>The token for the next set of results, or &#39;&#39;null&#39;&#39; if there are no additional results.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ListGroupCertificateAuthoritiesRequest {
    /// <p>The ID of the AWS Greengrass group.</p>
    #[serde(rename = "GroupId")]
    pub group_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct ListGroupCertificateAuthoritiesResponse {
    /// <p>A list of certificate authorities associated with the group.</p>
    #[serde(rename = "GroupCertificateAuthorities")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_certificate_authorities: Option<Vec<GroupCertificateAuthorityProperties>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ListGroupVersionsRequest {
    /// <p>The ID of the AWS Greengrass group.</p>
    #[serde(rename = "GroupId")]
    pub group_id: String,
    /// <p>The maximum number of results to be returned per request.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<String>,
    /// <p>The token for the next set of results, or &#39;&#39;null&#39;&#39; if there are no additional results.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct ListGroupVersionsResponse {
    /// <p>The token for the next set of results, or &#39;&#39;null&#39;&#39; if there are no additional results.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>Information about a version.</p>
    #[serde(rename = "Versions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub versions: Option<Vec<VersionInformation>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ListGroupsRequest {
    /// <p>The maximum number of results to be returned per request.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<String>,
    /// <p>The token for the next set of results, or &#39;&#39;null&#39;&#39; if there are no additional results.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct ListGroupsResponse {
    /// <p>Information about a group.</p>
    #[serde(rename = "Groups")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub groups: Option<Vec<GroupInformation>>,
    /// <p>The token for the next set of results, or &#39;&#39;null&#39;&#39; if there are no additional results.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ListLoggerDefinitionVersionsRequest {
    /// <p>The ID of the logger definition.</p>
    #[serde(rename = "LoggerDefinitionId")]
    pub logger_definition_id: String,
    /// <p>The maximum number of results to be returned per request.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<String>,
    /// <p>The token for the next set of results, or &#39;&#39;null&#39;&#39; if there are no additional results.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct ListLoggerDefinitionVersionsResponse {
    /// <p>The token for the next set of results, or &#39;&#39;null&#39;&#39; if there are no additional results.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>Information about a version.</p>
    #[serde(rename = "Versions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub versions: Option<Vec<VersionInformation>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ListLoggerDefinitionsRequest {
    /// <p>The maximum number of results to be returned per request.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<String>,
    /// <p>The token for the next set of results, or &#39;&#39;null&#39;&#39; if there are no additional results.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct ListLoggerDefinitionsResponse {
    /// <p>Information about a definition.</p>
    #[serde(rename = "Definitions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub definitions: Option<Vec<DefinitionInformation>>,
    /// <p>The token for the next set of results, or &#39;&#39;null&#39;&#39; if there are no additional results.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ListResourceDefinitionVersionsRequest {
    /// <p>The maximum number of results to be returned per request.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<String>,
    /// <p>The token for the next set of results, or &#39;&#39;null&#39;&#39; if there are no additional results.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The ID of the resource definition.</p>
    #[serde(rename = "ResourceDefinitionId")]
    pub resource_definition_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct ListResourceDefinitionVersionsResponse {
    /// <p>The token for the next set of results, or &#39;&#39;null&#39;&#39; if there are no additional results.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>Information about a version.</p>
    #[serde(rename = "Versions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub versions: Option<Vec<VersionInformation>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ListResourceDefinitionsRequest {
    /// <p>The maximum number of results to be returned per request.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<String>,
    /// <p>The token for the next set of results, or &#39;&#39;null&#39;&#39; if there are no additional results.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct ListResourceDefinitionsResponse {
    /// <p>Information about a definition.</p>
    #[serde(rename = "Definitions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub definitions: Option<Vec<DefinitionInformation>>,
    /// <p>The token for the next set of results, or &#39;&#39;null&#39;&#39; if there are no additional results.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ListSubscriptionDefinitionVersionsRequest {
    /// <p>The maximum number of results to be returned per request.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<String>,
    /// <p>The token for the next set of results, or &#39;&#39;null&#39;&#39; if there are no additional results.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The ID of the subscription definition.</p>
    #[serde(rename = "SubscriptionDefinitionId")]
    pub subscription_definition_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct ListSubscriptionDefinitionVersionsResponse {
    /// <p>The token for the next set of results, or &#39;&#39;null&#39;&#39; if there are no additional results.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>Information about a version.</p>
    #[serde(rename = "Versions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub versions: Option<Vec<VersionInformation>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ListSubscriptionDefinitionsRequest {
    /// <p>The maximum number of results to be returned per request.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<String>,
    /// <p>The token for the next set of results, or &#39;&#39;null&#39;&#39; if there are no additional results.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct ListSubscriptionDefinitionsResponse {
    /// <p>Information about a definition.</p>
    #[serde(rename = "Definitions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub definitions: Option<Vec<DefinitionInformation>>,
    /// <p>The token for the next set of results, or &#39;&#39;null&#39;&#39; if there are no additional results.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

/// <p>A list of versions.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct ListVersionsResponse {
    /// <p>The token for the next set of results, or &#39;&#39;null&#39;&#39; if there are no additional results.</p>
    pub next_token: Option<String>,
    /// <p>Information about a version.</p>
    pub versions: Option<Vec<VersionInformation>>,
}

/// <p>Attributes that define a local device resource.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LocalDeviceResourceData {
    /// <p>Group/owner related settings for local resources.</p>
    #[serde(rename = "GroupOwnerSetting")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_owner_setting: Option<GroupOwnerSetting>,
    /// <p>The local absolute path of the device resource. The source path for a device resource can refer only to a character device or block device under &#39;&#39;/dev&#39;&#39;.</p>
    #[serde(rename = "SourcePath")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_path: Option<String>,
}

/// <p>Attributes that define a local volume resource.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LocalVolumeResourceData {
    /// <p>The absolute local path of the resource inside the lambda environment.</p>
    #[serde(rename = "DestinationPath")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination_path: Option<String>,
    /// <p>Allows you to configure additional group privileges for the Lambda process. This field is optional.</p>
    #[serde(rename = "GroupOwnerSetting")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_owner_setting: Option<GroupOwnerSetting>,
    /// <p>The local absolute path of the volume resource on the host. The source path for a volume resource type cannot start with &#39;&#39;/sys&#39;&#39;.</p>
    #[serde(rename = "SourcePath")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_path: Option<String>,
}

/// <p>Information about a logger</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Logger {
    /// <p>The component that will be subject to logging.</p>
    #[serde(rename = "Component")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub component: Option<String>,
    /// <p>The id of the logger.</p>
    #[serde(rename = "Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// <p>The level of the logs.</p>
    #[serde(rename = "Level")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub level: Option<String>,
    /// <p>The amount of file space, in KB, to use if the local file system is used for logging purposes.</p>
    #[serde(rename = "Space")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub space: Option<i64>,
    /// <p>The type of log output which will be used.</p>
    #[serde(rename = "Type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}

/// <p>Information about a logger definition version.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LoggerDefinitionVersion {
    /// <p>A list of loggers.</p>
    #[serde(rename = "Loggers")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub loggers: Option<Vec<Logger>>,
}

/// <p>Information needed to reset deployments.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ResetDeploymentsRequest {
    /// <p>A client token used to correlate requests and responses.</p>
    #[serde(rename = "AmznClientToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amzn_client_token: Option<String>,
    /// <p>If true, performs a best-effort only core reset.</p>
    #[serde(rename = "Force")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub force: Option<bool>,
    /// <p>The ID of the AWS Greengrass group.</p>
    #[serde(rename = "GroupId")]
    pub group_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct ResetDeploymentsResponse {
    /// <p>The ARN of the deployment.</p>
    #[serde(rename = "DeploymentArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deployment_arn: Option<String>,
    /// <p>The ID of the deployment.</p>
    #[serde(rename = "DeploymentId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deployment_id: Option<String>,
}

/// <p>Information about a resource.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Resource {
    /// <p>The resource ID, used to refer to a resource in the Lambda function configuration. Max length is 128 characters with pattern &#39;&#39;[a-zA-Z0-9:_-]+&#39;&#39;. This must be unique within a Greengrass group.</p>
    #[serde(rename = "Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// <p>The descriptive resource name, which is displayed on the Greengrass console. Max length 128 characters with pattern &#39;&#39;[a-zA-Z0-9:_-]+&#39;&#39;. This must be unique within a Greengrass group.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>A container of data for all resource types.</p>
    #[serde(rename = "ResourceDataContainer")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_data_container: Option<ResourceDataContainer>,
}

/// <p>A policy used by the function to access a resource.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ResourceAccessPolicy {
    /// <p>The permissions that the Lambda function has to the resource. Can be one of &#39;&#39;rw&#39;&#39; (read/write) or &#39;&#39;ro&#39;&#39; (read-only).</p>
    #[serde(rename = "Permission")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permission: Option<String>,
    /// <p>The ID of the resource. (This ID is assigned to the resource when you create the resource definiton.)</p>
    #[serde(rename = "ResourceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_id: Option<String>,
}

/// <p>A container for resource data. The container takes only one of the following supported resource data types: &#39;&#39;LocalDeviceResourceData&#39;&#39;, &#39;&#39;LocalVolumeResourceData&#39;&#39;, &#39;&#39;SageMakerMachineLearningModelResourceData&#39;&#39;, &#39;&#39;S3MachineLearningModelResourceData&#39;&#39;.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ResourceDataContainer {
    /// <p>Attributes that define the local device resource.</p>
    #[serde(rename = "LocalDeviceResourceData")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub local_device_resource_data: Option<LocalDeviceResourceData>,
    /// <p>Attributes that define the local volume resource.</p>
    #[serde(rename = "LocalVolumeResourceData")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub local_volume_resource_data: Option<LocalVolumeResourceData>,
    /// <p>Attributes that define an S3 machine learning resource.</p>
    #[serde(rename = "S3MachineLearningModelResourceData")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_machine_learning_model_resource_data: Option<S3MachineLearningModelResourceData>,
    /// <p>Attributes that define an SageMaker machine learning resource.</p>
    #[serde(rename = "SageMakerMachineLearningModelResourceData")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sage_maker_machine_learning_model_resource_data:
        Option<SageMakerMachineLearningModelResourceData>,
}

/// <p>Information about a resource definition version.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ResourceDefinitionVersion {
    /// <p>A list of resources.</p>
    #[serde(rename = "Resources")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resources: Option<Vec<Resource>>,
}

/// <p>Attributes that define an S3 machine learning resource.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct S3MachineLearningModelResourceData {
    /// <p>The absolute local path of the resource inside the Lambda environment.</p>
    #[serde(rename = "DestinationPath")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination_path: Option<String>,
    /// <p>The URI of the source model in an S3 bucket. The model package must be in tar.gz or .zip format.</p>
    #[serde(rename = "S3Uri")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_uri: Option<String>,
}

/// <p>Attributes that define an SageMaker machine learning resource.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SageMakerMachineLearningModelResourceData {
    /// <p>The absolute local path of the resource inside the Lambda environment.</p>
    #[serde(rename = "DestinationPath")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination_path: Option<String>,
    /// <p>The ARN of the SageMaker training job that represents the source model.</p>
    #[serde(rename = "SageMakerJobArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sage_maker_job_arn: Option<String>,
}

/// <p>Information about a subscription.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Subscription {
    /// <p>The id of the subscription.</p>
    #[serde(rename = "Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// <p>The source of the subscription. Can be a thing ARN, a Lambda function ARN, &#39;cloud&#39; (which represents the IoT cloud), or &#39;GGShadowService&#39;.</p>
    #[serde(rename = "Source")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
    /// <p>The subject of the message.</p>
    #[serde(rename = "Subject")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subject: Option<String>,
    /// <p>Where the message is sent to. Can be a thing ARN, a Lambda function ARN, &#39;cloud&#39; (which represents the IoT cloud), or &#39;GGShadowService&#39;.</p>
    #[serde(rename = "Target")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target: Option<String>,
}

/// <p>Information about a subscription definition version.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SubscriptionDefinitionVersion {
    /// <p>A list of subscriptions.</p>
    #[serde(rename = "Subscriptions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscriptions: Option<Vec<Subscription>>,
}

/// <p>Connectivity information.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct UpdateConnectivityInfoRequest {
    /// <p>A list of connectivity info.</p>
    #[serde(rename = "ConnectivityInfo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connectivity_info: Option<Vec<ConnectivityInfo>>,
    /// <p>The thing name.</p>
    #[serde(rename = "ThingName")]
    pub thing_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct UpdateConnectivityInfoResponse {
    /// <p>A message about the connectivity info update request.</p>
    #[serde(rename = "Message")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    /// <p>The new version of the connectivity info.</p>
    #[serde(rename = "Version")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct UpdateCoreDefinitionRequest {
    /// <p>The ID of the core definition.</p>
    #[serde(rename = "CoreDefinitionId")]
    pub core_definition_id: String,
    /// <p>The name of the definition.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct UpdateCoreDefinitionResponse {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct UpdateDeviceDefinitionRequest {
    /// <p>The ID of the device definition.</p>
    #[serde(rename = "DeviceDefinitionId")]
    pub device_definition_id: String,
    /// <p>The name of the definition.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct UpdateDeviceDefinitionResponse {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct UpdateFunctionDefinitionRequest {
    /// <p>The ID of the Lambda function definition.</p>
    #[serde(rename = "FunctionDefinitionId")]
    pub function_definition_id: String,
    /// <p>The name of the definition.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct UpdateFunctionDefinitionResponse {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct UpdateGroupCertificateConfigurationRequest {
    /// <p>The amount of time remaining before the certificate expires, in milliseconds.</p>
    #[serde(rename = "CertificateExpiryInMilliseconds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate_expiry_in_milliseconds: Option<String>,
    /// <p>The ID of the AWS Greengrass group.</p>
    #[serde(rename = "GroupId")]
    pub group_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct UpdateGroupCertificateConfigurationResponse {
    /// <p>The amount of time remaining before the certificate authority expires, in milliseconds.</p>
    #[serde(rename = "CertificateAuthorityExpiryInMilliseconds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate_authority_expiry_in_milliseconds: Option<String>,
    /// <p>The amount of time remaining before the certificate expires, in milliseconds.</p>
    #[serde(rename = "CertificateExpiryInMilliseconds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate_expiry_in_milliseconds: Option<String>,
    /// <p>The ID of the group certificate configuration.</p>
    #[serde(rename = "GroupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_id: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct UpdateGroupRequest {
    /// <p>The ID of the AWS Greengrass group.</p>
    #[serde(rename = "GroupId")]
    pub group_id: String,
    /// <p>The name of the definition.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct UpdateGroupResponse {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct UpdateLoggerDefinitionRequest {
    /// <p>The ID of the logger definition.</p>
    #[serde(rename = "LoggerDefinitionId")]
    pub logger_definition_id: String,
    /// <p>The name of the definition.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct UpdateLoggerDefinitionResponse {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct UpdateResourceDefinitionRequest {
    /// <p>The name of the definition.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The ID of the resource definition.</p>
    #[serde(rename = "ResourceDefinitionId")]
    pub resource_definition_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct UpdateResourceDefinitionResponse {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct UpdateSubscriptionDefinitionRequest {
    /// <p>The name of the definition.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The ID of the subscription definition.</p>
    #[serde(rename = "SubscriptionDefinitionId")]
    pub subscription_definition_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct UpdateSubscriptionDefinitionResponse {}

/// <p>Information about a version.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct VersionInformation {
    /// <p>The ARN of the version.</p>
    #[serde(rename = "Arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p>The time, in milliseconds since the epoch, when the version was created.</p>
    #[serde(rename = "CreationTimestamp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_timestamp: Option<String>,
    /// <p>The ID of the version.</p>
    #[serde(rename = "Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// <p>The unique ID of the version.</p>
    #[serde(rename = "Version")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

/// Errors returned by AssociateRoleToGroup
#[derive(Debug, PartialEq)]
pub enum AssociateRoleToGroupError {
    /// <p>General error information.</p>
    BadRequest(String),
    /// <p>General error information.</p>
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

impl AssociateRoleToGroupError {
    pub fn from_body(body: &str) -> AssociateRoleToGroupError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json
                    .get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "BadRequestException" => {
                        AssociateRoleToGroupError::BadRequest(String::from(error_message))
                    }
                    "InternalServerErrorException" => {
                        AssociateRoleToGroupError::InternalServerError(String::from(error_message))
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
            AssociateRoleToGroupError::BadRequest(ref cause) => cause,
            AssociateRoleToGroupError::InternalServerError(ref cause) => cause,
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
    /// <p>General error information.</p>
    BadRequest(String),
    /// <p>General error information.</p>
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

impl AssociateServiceRoleToAccountError {
    pub fn from_body(body: &str) -> AssociateServiceRoleToAccountError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json
                    .get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "BadRequestException" => {
                        AssociateServiceRoleToAccountError::BadRequest(String::from(error_message))
                    }
                    "InternalServerErrorException" => {
                        AssociateServiceRoleToAccountError::InternalServerError(String::from(
                            error_message,
                        ))
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
            AssociateServiceRoleToAccountError::BadRequest(ref cause) => cause,
            AssociateServiceRoleToAccountError::InternalServerError(ref cause) => cause,
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
    /// <p>General error information.</p>
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
                let raw_error_type = json
                    .get("__type")
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
    /// <p>General error information.</p>
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
                let raw_error_type = json
                    .get("__type")
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
    /// <p>General error information.</p>
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
                let raw_error_type = json
                    .get("__type")
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
    /// <p>General error information.</p>
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
                let raw_error_type = json
                    .get("__type")
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
    /// <p>General error information.</p>
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
                let raw_error_type = json
                    .get("__type")
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
    /// <p>General error information.</p>
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
                let raw_error_type = json
                    .get("__type")
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
    /// <p>General error information.</p>
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
                let raw_error_type = json
                    .get("__type")
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
    /// <p>General error information.</p>
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
                let raw_error_type = json
                    .get("__type")
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
    /// <p>General error information.</p>
    BadRequest(String),
    /// <p>General error information.</p>
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

impl CreateGroupCertificateAuthorityError {
    pub fn from_body(body: &str) -> CreateGroupCertificateAuthorityError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json
                    .get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "BadRequestException" => CreateGroupCertificateAuthorityError::BadRequest(
                        String::from(error_message),
                    ),
                    "InternalServerErrorException" => {
                        CreateGroupCertificateAuthorityError::InternalServerError(String::from(
                            error_message,
                        ))
                    }
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
            CreateGroupCertificateAuthorityError::BadRequest(ref cause) => cause,
            CreateGroupCertificateAuthorityError::InternalServerError(ref cause) => cause,
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
    /// <p>General error information.</p>
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
                let raw_error_type = json
                    .get("__type")
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
    /// <p>General error information.</p>
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
                let raw_error_type = json
                    .get("__type")
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
    /// <p>General error information.</p>
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
                let raw_error_type = json
                    .get("__type")
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
/// Errors returned by CreateResourceDefinition
#[derive(Debug, PartialEq)]
pub enum CreateResourceDefinitionError {
    /// <p>General error information.</p>
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

impl CreateResourceDefinitionError {
    pub fn from_body(body: &str) -> CreateResourceDefinitionError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json
                    .get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "BadRequestException" => {
                        CreateResourceDefinitionError::BadRequest(String::from(error_message))
                    }
                    "ValidationException" => {
                        CreateResourceDefinitionError::Validation(error_message.to_string())
                    }
                    _ => CreateResourceDefinitionError::Unknown(String::from(body)),
                }
            }
            Err(_) => CreateResourceDefinitionError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for CreateResourceDefinitionError {
    fn from(err: serde_json::error::Error) -> CreateResourceDefinitionError {
        CreateResourceDefinitionError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for CreateResourceDefinitionError {
    fn from(err: CredentialsError) -> CreateResourceDefinitionError {
        CreateResourceDefinitionError::Credentials(err)
    }
}
impl From<HttpDispatchError> for CreateResourceDefinitionError {
    fn from(err: HttpDispatchError) -> CreateResourceDefinitionError {
        CreateResourceDefinitionError::HttpDispatch(err)
    }
}
impl From<io::Error> for CreateResourceDefinitionError {
    fn from(err: io::Error) -> CreateResourceDefinitionError {
        CreateResourceDefinitionError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for CreateResourceDefinitionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateResourceDefinitionError {
    fn description(&self) -> &str {
        match *self {
            CreateResourceDefinitionError::BadRequest(ref cause) => cause,
            CreateResourceDefinitionError::Validation(ref cause) => cause,
            CreateResourceDefinitionError::Credentials(ref err) => err.description(),
            CreateResourceDefinitionError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            CreateResourceDefinitionError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by CreateResourceDefinitionVersion
#[derive(Debug, PartialEq)]
pub enum CreateResourceDefinitionVersionError {
    /// <p>General error information.</p>
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

impl CreateResourceDefinitionVersionError {
    pub fn from_body(body: &str) -> CreateResourceDefinitionVersionError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json
                    .get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "BadRequestException" => CreateResourceDefinitionVersionError::BadRequest(
                        String::from(error_message),
                    ),
                    "ValidationException" => {
                        CreateResourceDefinitionVersionError::Validation(error_message.to_string())
                    }
                    _ => CreateResourceDefinitionVersionError::Unknown(String::from(body)),
                }
            }
            Err(_) => CreateResourceDefinitionVersionError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for CreateResourceDefinitionVersionError {
    fn from(err: serde_json::error::Error) -> CreateResourceDefinitionVersionError {
        CreateResourceDefinitionVersionError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for CreateResourceDefinitionVersionError {
    fn from(err: CredentialsError) -> CreateResourceDefinitionVersionError {
        CreateResourceDefinitionVersionError::Credentials(err)
    }
}
impl From<HttpDispatchError> for CreateResourceDefinitionVersionError {
    fn from(err: HttpDispatchError) -> CreateResourceDefinitionVersionError {
        CreateResourceDefinitionVersionError::HttpDispatch(err)
    }
}
impl From<io::Error> for CreateResourceDefinitionVersionError {
    fn from(err: io::Error) -> CreateResourceDefinitionVersionError {
        CreateResourceDefinitionVersionError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for CreateResourceDefinitionVersionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateResourceDefinitionVersionError {
    fn description(&self) -> &str {
        match *self {
            CreateResourceDefinitionVersionError::BadRequest(ref cause) => cause,
            CreateResourceDefinitionVersionError::Validation(ref cause) => cause,
            CreateResourceDefinitionVersionError::Credentials(ref err) => err.description(),
            CreateResourceDefinitionVersionError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            CreateResourceDefinitionVersionError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by CreateSoftwareUpdateJob
#[derive(Debug, PartialEq)]
pub enum CreateSoftwareUpdateJobError {
    /// <p>General error information.</p>
    BadRequest(String),
    /// <p>General error information.</p>
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

impl CreateSoftwareUpdateJobError {
    pub fn from_body(body: &str) -> CreateSoftwareUpdateJobError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json
                    .get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "BadRequestException" => {
                        CreateSoftwareUpdateJobError::BadRequest(String::from(error_message))
                    }
                    "InternalServerErrorException" => {
                        CreateSoftwareUpdateJobError::InternalServerError(String::from(
                            error_message,
                        ))
                    }
                    "ValidationException" => {
                        CreateSoftwareUpdateJobError::Validation(error_message.to_string())
                    }
                    _ => CreateSoftwareUpdateJobError::Unknown(String::from(body)),
                }
            }
            Err(_) => CreateSoftwareUpdateJobError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for CreateSoftwareUpdateJobError {
    fn from(err: serde_json::error::Error) -> CreateSoftwareUpdateJobError {
        CreateSoftwareUpdateJobError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for CreateSoftwareUpdateJobError {
    fn from(err: CredentialsError) -> CreateSoftwareUpdateJobError {
        CreateSoftwareUpdateJobError::Credentials(err)
    }
}
impl From<HttpDispatchError> for CreateSoftwareUpdateJobError {
    fn from(err: HttpDispatchError) -> CreateSoftwareUpdateJobError {
        CreateSoftwareUpdateJobError::HttpDispatch(err)
    }
}
impl From<io::Error> for CreateSoftwareUpdateJobError {
    fn from(err: io::Error) -> CreateSoftwareUpdateJobError {
        CreateSoftwareUpdateJobError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for CreateSoftwareUpdateJobError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateSoftwareUpdateJobError {
    fn description(&self) -> &str {
        match *self {
            CreateSoftwareUpdateJobError::BadRequest(ref cause) => cause,
            CreateSoftwareUpdateJobError::InternalServerError(ref cause) => cause,
            CreateSoftwareUpdateJobError::Validation(ref cause) => cause,
            CreateSoftwareUpdateJobError::Credentials(ref err) => err.description(),
            CreateSoftwareUpdateJobError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            CreateSoftwareUpdateJobError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by CreateSubscriptionDefinition
#[derive(Debug, PartialEq)]
pub enum CreateSubscriptionDefinitionError {
    /// <p>General error information.</p>
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
                let raw_error_type = json
                    .get("__type")
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
    /// <p>General error information.</p>
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
                let raw_error_type = json
                    .get("__type")
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
    /// <p>General error information.</p>
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
                let raw_error_type = json
                    .get("__type")
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
    /// <p>General error information.</p>
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
                let raw_error_type = json
                    .get("__type")
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
    /// <p>General error information.</p>
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
                let raw_error_type = json
                    .get("__type")
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
    /// <p>General error information.</p>
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
                let raw_error_type = json
                    .get("__type")
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
    /// <p>General error information.</p>
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
                let raw_error_type = json
                    .get("__type")
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
/// Errors returned by DeleteResourceDefinition
#[derive(Debug, PartialEq)]
pub enum DeleteResourceDefinitionError {
    /// <p>General error information.</p>
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

impl DeleteResourceDefinitionError {
    pub fn from_body(body: &str) -> DeleteResourceDefinitionError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json
                    .get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "BadRequestException" => {
                        DeleteResourceDefinitionError::BadRequest(String::from(error_message))
                    }
                    "ValidationException" => {
                        DeleteResourceDefinitionError::Validation(error_message.to_string())
                    }
                    _ => DeleteResourceDefinitionError::Unknown(String::from(body)),
                }
            }
            Err(_) => DeleteResourceDefinitionError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DeleteResourceDefinitionError {
    fn from(err: serde_json::error::Error) -> DeleteResourceDefinitionError {
        DeleteResourceDefinitionError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DeleteResourceDefinitionError {
    fn from(err: CredentialsError) -> DeleteResourceDefinitionError {
        DeleteResourceDefinitionError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DeleteResourceDefinitionError {
    fn from(err: HttpDispatchError) -> DeleteResourceDefinitionError {
        DeleteResourceDefinitionError::HttpDispatch(err)
    }
}
impl From<io::Error> for DeleteResourceDefinitionError {
    fn from(err: io::Error) -> DeleteResourceDefinitionError {
        DeleteResourceDefinitionError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DeleteResourceDefinitionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteResourceDefinitionError {
    fn description(&self) -> &str {
        match *self {
            DeleteResourceDefinitionError::BadRequest(ref cause) => cause,
            DeleteResourceDefinitionError::Validation(ref cause) => cause,
            DeleteResourceDefinitionError::Credentials(ref err) => err.description(),
            DeleteResourceDefinitionError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DeleteResourceDefinitionError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DeleteSubscriptionDefinition
#[derive(Debug, PartialEq)]
pub enum DeleteSubscriptionDefinitionError {
    /// <p>General error information.</p>
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
                let raw_error_type = json
                    .get("__type")
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
    /// <p>General error information.</p>
    BadRequest(String),
    /// <p>General error information.</p>
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

impl DisassociateRoleFromGroupError {
    pub fn from_body(body: &str) -> DisassociateRoleFromGroupError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json
                    .get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "BadRequestException" => {
                        DisassociateRoleFromGroupError::BadRequest(String::from(error_message))
                    }
                    "InternalServerErrorException" => {
                        DisassociateRoleFromGroupError::InternalServerError(String::from(
                            error_message,
                        ))
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
            DisassociateRoleFromGroupError::BadRequest(ref cause) => cause,
            DisassociateRoleFromGroupError::InternalServerError(ref cause) => cause,
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
    /// <p>General error information.</p>
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
                let raw_error_type = json
                    .get("__type")
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
    /// <p>General error information.</p>
    BadRequest(String),
    /// <p>General error information.</p>
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

impl GetAssociatedRoleError {
    pub fn from_body(body: &str) -> GetAssociatedRoleError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json
                    .get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "BadRequestException" => {
                        GetAssociatedRoleError::BadRequest(String::from(error_message))
                    }
                    "InternalServerErrorException" => {
                        GetAssociatedRoleError::InternalServerError(String::from(error_message))
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
            GetAssociatedRoleError::BadRequest(ref cause) => cause,
            GetAssociatedRoleError::InternalServerError(ref cause) => cause,
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
    /// <p>General error information.</p>
    BadRequest(String),
    /// <p>General error information.</p>
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
                let raw_error_type = json
                    .get("__type")
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
    /// <p>General error information.</p>
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
                let raw_error_type = json
                    .get("__type")
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
    /// <p>General error information.</p>
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
                let raw_error_type = json
                    .get("__type")
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
    /// <p>General error information.</p>
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
                let raw_error_type = json
                    .get("__type")
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
    /// <p>General error information.</p>
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
                let raw_error_type = json
                    .get("__type")
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
    /// <p>General error information.</p>
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
                let raw_error_type = json
                    .get("__type")
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
    /// <p>General error information.</p>
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
                let raw_error_type = json
                    .get("__type")
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
    /// <p>General error information.</p>
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
                let raw_error_type = json
                    .get("__type")
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
    /// <p>General error information.</p>
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
                let raw_error_type = json
                    .get("__type")
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
    /// <p>General error information.</p>
    BadRequest(String),
    /// <p>General error information.</p>
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
                let raw_error_type = json
                    .get("__type")
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
    /// <p>General error information.</p>
    BadRequest(String),
    /// <p>General error information.</p>
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
                let raw_error_type = json
                    .get("__type")
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
    /// <p>General error information.</p>
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
                let raw_error_type = json
                    .get("__type")
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
    /// <p>General error information.</p>
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
                let raw_error_type = json
                    .get("__type")
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
    /// <p>General error information.</p>
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
                let raw_error_type = json
                    .get("__type")
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
/// Errors returned by GetResourceDefinition
#[derive(Debug, PartialEq)]
pub enum GetResourceDefinitionError {
    /// <p>General error information.</p>
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

impl GetResourceDefinitionError {
    pub fn from_body(body: &str) -> GetResourceDefinitionError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json
                    .get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "BadRequestException" => {
                        GetResourceDefinitionError::BadRequest(String::from(error_message))
                    }
                    "ValidationException" => {
                        GetResourceDefinitionError::Validation(error_message.to_string())
                    }
                    _ => GetResourceDefinitionError::Unknown(String::from(body)),
                }
            }
            Err(_) => GetResourceDefinitionError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for GetResourceDefinitionError {
    fn from(err: serde_json::error::Error) -> GetResourceDefinitionError {
        GetResourceDefinitionError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for GetResourceDefinitionError {
    fn from(err: CredentialsError) -> GetResourceDefinitionError {
        GetResourceDefinitionError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GetResourceDefinitionError {
    fn from(err: HttpDispatchError) -> GetResourceDefinitionError {
        GetResourceDefinitionError::HttpDispatch(err)
    }
}
impl From<io::Error> for GetResourceDefinitionError {
    fn from(err: io::Error) -> GetResourceDefinitionError {
        GetResourceDefinitionError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for GetResourceDefinitionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetResourceDefinitionError {
    fn description(&self) -> &str {
        match *self {
            GetResourceDefinitionError::BadRequest(ref cause) => cause,
            GetResourceDefinitionError::Validation(ref cause) => cause,
            GetResourceDefinitionError::Credentials(ref err) => err.description(),
            GetResourceDefinitionError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            GetResourceDefinitionError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by GetResourceDefinitionVersion
#[derive(Debug, PartialEq)]
pub enum GetResourceDefinitionVersionError {
    /// <p>General error information.</p>
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

impl GetResourceDefinitionVersionError {
    pub fn from_body(body: &str) -> GetResourceDefinitionVersionError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json
                    .get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "BadRequestException" => {
                        GetResourceDefinitionVersionError::BadRequest(String::from(error_message))
                    }
                    "ValidationException" => {
                        GetResourceDefinitionVersionError::Validation(error_message.to_string())
                    }
                    _ => GetResourceDefinitionVersionError::Unknown(String::from(body)),
                }
            }
            Err(_) => GetResourceDefinitionVersionError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for GetResourceDefinitionVersionError {
    fn from(err: serde_json::error::Error) -> GetResourceDefinitionVersionError {
        GetResourceDefinitionVersionError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for GetResourceDefinitionVersionError {
    fn from(err: CredentialsError) -> GetResourceDefinitionVersionError {
        GetResourceDefinitionVersionError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GetResourceDefinitionVersionError {
    fn from(err: HttpDispatchError) -> GetResourceDefinitionVersionError {
        GetResourceDefinitionVersionError::HttpDispatch(err)
    }
}
impl From<io::Error> for GetResourceDefinitionVersionError {
    fn from(err: io::Error) -> GetResourceDefinitionVersionError {
        GetResourceDefinitionVersionError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for GetResourceDefinitionVersionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetResourceDefinitionVersionError {
    fn description(&self) -> &str {
        match *self {
            GetResourceDefinitionVersionError::BadRequest(ref cause) => cause,
            GetResourceDefinitionVersionError::Validation(ref cause) => cause,
            GetResourceDefinitionVersionError::Credentials(ref err) => err.description(),
            GetResourceDefinitionVersionError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            GetResourceDefinitionVersionError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by GetServiceRoleForAccount
#[derive(Debug, PartialEq)]
pub enum GetServiceRoleForAccountError {
    /// <p>General error information.</p>
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
                let raw_error_type = json
                    .get("__type")
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
    /// <p>General error information.</p>
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
                let raw_error_type = json
                    .get("__type")
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
    /// <p>General error information.</p>
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
                let raw_error_type = json
                    .get("__type")
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
    /// <p>General error information.</p>
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
                let raw_error_type = json
                    .get("__type")
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
                let raw_error_type = json
                    .get("__type")
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
    /// <p>General error information.</p>
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
                let raw_error_type = json
                    .get("__type")
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
    /// <p>General error information.</p>
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
                let raw_error_type = json
                    .get("__type")
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
                let raw_error_type = json
                    .get("__type")
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
    /// <p>General error information.</p>
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
                let raw_error_type = json
                    .get("__type")
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
                let raw_error_type = json
                    .get("__type")
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
    /// <p>General error information.</p>
    BadRequest(String),
    /// <p>General error information.</p>
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
                let raw_error_type = json
                    .get("__type")
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
    /// <p>General error information.</p>
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
                let raw_error_type = json
                    .get("__type")
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
                let raw_error_type = json
                    .get("__type")
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
    /// <p>General error information.</p>
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
                let raw_error_type = json
                    .get("__type")
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
                let raw_error_type = json
                    .get("__type")
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
/// Errors returned by ListResourceDefinitionVersions
#[derive(Debug, PartialEq)]
pub enum ListResourceDefinitionVersionsError {
    /// <p>General error information.</p>
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

impl ListResourceDefinitionVersionsError {
    pub fn from_body(body: &str) -> ListResourceDefinitionVersionsError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json
                    .get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "BadRequestException" => {
                        ListResourceDefinitionVersionsError::BadRequest(String::from(error_message))
                    }
                    "ValidationException" => {
                        ListResourceDefinitionVersionsError::Validation(error_message.to_string())
                    }
                    _ => ListResourceDefinitionVersionsError::Unknown(String::from(body)),
                }
            }
            Err(_) => ListResourceDefinitionVersionsError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for ListResourceDefinitionVersionsError {
    fn from(err: serde_json::error::Error) -> ListResourceDefinitionVersionsError {
        ListResourceDefinitionVersionsError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for ListResourceDefinitionVersionsError {
    fn from(err: CredentialsError) -> ListResourceDefinitionVersionsError {
        ListResourceDefinitionVersionsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ListResourceDefinitionVersionsError {
    fn from(err: HttpDispatchError) -> ListResourceDefinitionVersionsError {
        ListResourceDefinitionVersionsError::HttpDispatch(err)
    }
}
impl From<io::Error> for ListResourceDefinitionVersionsError {
    fn from(err: io::Error) -> ListResourceDefinitionVersionsError {
        ListResourceDefinitionVersionsError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for ListResourceDefinitionVersionsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListResourceDefinitionVersionsError {
    fn description(&self) -> &str {
        match *self {
            ListResourceDefinitionVersionsError::BadRequest(ref cause) => cause,
            ListResourceDefinitionVersionsError::Validation(ref cause) => cause,
            ListResourceDefinitionVersionsError::Credentials(ref err) => err.description(),
            ListResourceDefinitionVersionsError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            ListResourceDefinitionVersionsError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by ListResourceDefinitions
#[derive(Debug, PartialEq)]
pub enum ListResourceDefinitionsError {
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl ListResourceDefinitionsError {
    pub fn from_body(body: &str) -> ListResourceDefinitionsError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json
                    .get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "ValidationException" => {
                        ListResourceDefinitionsError::Validation(error_message.to_string())
                    }
                    _ => ListResourceDefinitionsError::Unknown(String::from(body)),
                }
            }
            Err(_) => ListResourceDefinitionsError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for ListResourceDefinitionsError {
    fn from(err: serde_json::error::Error) -> ListResourceDefinitionsError {
        ListResourceDefinitionsError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for ListResourceDefinitionsError {
    fn from(err: CredentialsError) -> ListResourceDefinitionsError {
        ListResourceDefinitionsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ListResourceDefinitionsError {
    fn from(err: HttpDispatchError) -> ListResourceDefinitionsError {
        ListResourceDefinitionsError::HttpDispatch(err)
    }
}
impl From<io::Error> for ListResourceDefinitionsError {
    fn from(err: io::Error) -> ListResourceDefinitionsError {
        ListResourceDefinitionsError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for ListResourceDefinitionsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListResourceDefinitionsError {
    fn description(&self) -> &str {
        match *self {
            ListResourceDefinitionsError::Validation(ref cause) => cause,
            ListResourceDefinitionsError::Credentials(ref err) => err.description(),
            ListResourceDefinitionsError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            ListResourceDefinitionsError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by ListSubscriptionDefinitionVersions
#[derive(Debug, PartialEq)]
pub enum ListSubscriptionDefinitionVersionsError {
    /// <p>General error information.</p>
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
                let raw_error_type = json
                    .get("__type")
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
                let raw_error_type = json
                    .get("__type")
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
/// Errors returned by ResetDeployments
#[derive(Debug, PartialEq)]
pub enum ResetDeploymentsError {
    /// <p>General error information.</p>
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

impl ResetDeploymentsError {
    pub fn from_body(body: &str) -> ResetDeploymentsError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json
                    .get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "BadRequestException" => {
                        ResetDeploymentsError::BadRequest(String::from(error_message))
                    }
                    "ValidationException" => {
                        ResetDeploymentsError::Validation(error_message.to_string())
                    }
                    _ => ResetDeploymentsError::Unknown(String::from(body)),
                }
            }
            Err(_) => ResetDeploymentsError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for ResetDeploymentsError {
    fn from(err: serde_json::error::Error) -> ResetDeploymentsError {
        ResetDeploymentsError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for ResetDeploymentsError {
    fn from(err: CredentialsError) -> ResetDeploymentsError {
        ResetDeploymentsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ResetDeploymentsError {
    fn from(err: HttpDispatchError) -> ResetDeploymentsError {
        ResetDeploymentsError::HttpDispatch(err)
    }
}
impl From<io::Error> for ResetDeploymentsError {
    fn from(err: io::Error) -> ResetDeploymentsError {
        ResetDeploymentsError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for ResetDeploymentsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ResetDeploymentsError {
    fn description(&self) -> &str {
        match *self {
            ResetDeploymentsError::BadRequest(ref cause) => cause,
            ResetDeploymentsError::Validation(ref cause) => cause,
            ResetDeploymentsError::Credentials(ref err) => err.description(),
            ResetDeploymentsError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            ResetDeploymentsError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by UpdateConnectivityInfo
#[derive(Debug, PartialEq)]
pub enum UpdateConnectivityInfoError {
    /// <p>General error information.</p>
    BadRequest(String),
    /// <p>General error information.</p>
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
                let raw_error_type = json
                    .get("__type")
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
    /// <p>General error information.</p>
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
                let raw_error_type = json
                    .get("__type")
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
    /// <p>General error information.</p>
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
                let raw_error_type = json
                    .get("__type")
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
    /// <p>General error information.</p>
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
                let raw_error_type = json
                    .get("__type")
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
    /// <p>General error information.</p>
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
                let raw_error_type = json
                    .get("__type")
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
    /// <p>General error information.</p>
    BadRequest(String),
    /// <p>General error information.</p>
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
                let raw_error_type = json
                    .get("__type")
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
                        UpdateGroupCertificateConfigurationError::InternalServerError(String::from(
                            error_message,
                        ))
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
    /// <p>General error information.</p>
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
                let raw_error_type = json
                    .get("__type")
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
/// Errors returned by UpdateResourceDefinition
#[derive(Debug, PartialEq)]
pub enum UpdateResourceDefinitionError {
    /// <p>General error information.</p>
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

impl UpdateResourceDefinitionError {
    pub fn from_body(body: &str) -> UpdateResourceDefinitionError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json
                    .get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "BadRequestException" => {
                        UpdateResourceDefinitionError::BadRequest(String::from(error_message))
                    }
                    "ValidationException" => {
                        UpdateResourceDefinitionError::Validation(error_message.to_string())
                    }
                    _ => UpdateResourceDefinitionError::Unknown(String::from(body)),
                }
            }
            Err(_) => UpdateResourceDefinitionError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for UpdateResourceDefinitionError {
    fn from(err: serde_json::error::Error) -> UpdateResourceDefinitionError {
        UpdateResourceDefinitionError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for UpdateResourceDefinitionError {
    fn from(err: CredentialsError) -> UpdateResourceDefinitionError {
        UpdateResourceDefinitionError::Credentials(err)
    }
}
impl From<HttpDispatchError> for UpdateResourceDefinitionError {
    fn from(err: HttpDispatchError) -> UpdateResourceDefinitionError {
        UpdateResourceDefinitionError::HttpDispatch(err)
    }
}
impl From<io::Error> for UpdateResourceDefinitionError {
    fn from(err: io::Error) -> UpdateResourceDefinitionError {
        UpdateResourceDefinitionError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for UpdateResourceDefinitionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for UpdateResourceDefinitionError {
    fn description(&self) -> &str {
        match *self {
            UpdateResourceDefinitionError::BadRequest(ref cause) => cause,
            UpdateResourceDefinitionError::Validation(ref cause) => cause,
            UpdateResourceDefinitionError::Credentials(ref err) => err.description(),
            UpdateResourceDefinitionError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            UpdateResourceDefinitionError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by UpdateSubscriptionDefinition
#[derive(Debug, PartialEq)]
pub enum UpdateSubscriptionDefinitionError {
    /// <p>General error information.</p>
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
                let raw_error_type = json
                    .get("__type")
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
    /// <p>Associates a role with a group. Your AWS Greengrass core will use the role to access AWS cloud services. The role&#39;s permissions should allow Greengrass core Lambda functions to perform actions against the cloud.</p>
    fn associate_role_to_group(
        &self,
        input: AssociateRoleToGroupRequest,
    ) -> RusotoFuture<AssociateRoleToGroupResponse, AssociateRoleToGroupError>;

    /// <p>Associates a role with your account. AWS Greengrass will use the role to access your Lambda functions and AWS IoT resources. This is necessary for deployments to succeed. The role must have at least minimum permissions in the policy &#39;&#39;AWSGreengrassResourceAccessRolePolicy&#39;&#39;.</p>
    fn associate_service_role_to_account(
        &self,
        input: AssociateServiceRoleToAccountRequest,
    ) -> RusotoFuture<AssociateServiceRoleToAccountResponse, AssociateServiceRoleToAccountError>;

    /// <p>Creates a core definition. You may provide the initial version of the core definition now or use &#39;&#39;CreateCoreDefinitionVersion&#39;&#39; at a later time. AWS Greengrass groups must each contain exactly one AWS Greengrass core.</p>
    fn create_core_definition(
        &self,
        input: CreateCoreDefinitionRequest,
    ) -> RusotoFuture<CreateCoreDefinitionResponse, CreateCoreDefinitionError>;

    /// <p>Creates a version of a core definition that has already been defined. AWS Greengrass groups must each contain exactly one AWS Greengrass core.</p>
    fn create_core_definition_version(
        &self,
        input: CreateCoreDefinitionVersionRequest,
    ) -> RusotoFuture<CreateCoreDefinitionVersionResponse, CreateCoreDefinitionVersionError>;

    /// <p>Creates a deployment.</p>
    fn create_deployment(
        &self,
        input: CreateDeploymentRequest,
    ) -> RusotoFuture<CreateDeploymentResponse, CreateDeploymentError>;

    /// <p>Creates a device definition. You may provide the initial version of the device definition now or use &#39;&#39;CreateDeviceDefinitionVersion&#39;&#39; at a later time.</p>
    fn create_device_definition(
        &self,
        input: CreateDeviceDefinitionRequest,
    ) -> RusotoFuture<CreateDeviceDefinitionResponse, CreateDeviceDefinitionError>;

    /// <p>Creates a version of a device definition that has already been defined.</p>
    fn create_device_definition_version(
        &self,
        input: CreateDeviceDefinitionVersionRequest,
    ) -> RusotoFuture<CreateDeviceDefinitionVersionResponse, CreateDeviceDefinitionVersionError>;

    /// <p>Creates a Lambda function definition which contains a list of Lambda functions and their configurations to be used in a group. You can create an initial version of the definition by providing a list of Lambda functions and their configurations now, or use &#39;&#39;CreateFunctionDefinitionVersion&#39;&#39; later.</p>
    fn create_function_definition(
        &self,
        input: CreateFunctionDefinitionRequest,
    ) -> RusotoFuture<CreateFunctionDefinitionResponse, CreateFunctionDefinitionError>;

    /// <p>Creates a version of a Lambda function definition that has already been defined.</p>
    fn create_function_definition_version(
        &self,
        input: CreateFunctionDefinitionVersionRequest,
    ) -> RusotoFuture<CreateFunctionDefinitionVersionResponse, CreateFunctionDefinitionVersionError>;

    /// <p>Creates a group. You may provide the initial version of the group or use &#39;&#39;CreateGroupVersion&#39;&#39; at a later time.</p>
    fn create_group(
        &self,
        input: CreateGroupRequest,
    ) -> RusotoFuture<CreateGroupResponse, CreateGroupError>;

    /// <p>Creates a CA for the group. If a CA already exists, it will rotate the existing CA.</p>
    fn create_group_certificate_authority(
        &self,
        input: CreateGroupCertificateAuthorityRequest,
    ) -> RusotoFuture<CreateGroupCertificateAuthorityResponse, CreateGroupCertificateAuthorityError>;

    /// <p>Creates a version of a group which has already been defined.</p>
    fn create_group_version(
        &self,
        input: CreateGroupVersionRequest,
    ) -> RusotoFuture<CreateGroupVersionResponse, CreateGroupVersionError>;

    /// <p>Creates a logger definition. You may provide the initial version of the logger definition now or use &#39;&#39;CreateLoggerDefinitionVersion&#39;&#39; at a later time.</p>
    fn create_logger_definition(
        &self,
        input: CreateLoggerDefinitionRequest,
    ) -> RusotoFuture<CreateLoggerDefinitionResponse, CreateLoggerDefinitionError>;

    /// <p>Creates a version of a logger definition that has already been defined.</p>
    fn create_logger_definition_version(
        &self,
        input: CreateLoggerDefinitionVersionRequest,
    ) -> RusotoFuture<CreateLoggerDefinitionVersionResponse, CreateLoggerDefinitionVersionError>;

    /// <p>Creates a resource definition which contains a list of resources to be used in a group. You can create an initial version of the definition by providing a list of resources now, or use &#39;&#39;CreateResourceDefinitionVersion&#39;&#39; later.</p>
    fn create_resource_definition(
        &self,
        input: CreateResourceDefinitionRequest,
    ) -> RusotoFuture<CreateResourceDefinitionResponse, CreateResourceDefinitionError>;

    /// <p>Creates a version of a resource definition that has already been defined.</p>
    fn create_resource_definition_version(
        &self,
        input: CreateResourceDefinitionVersionRequest,
    ) -> RusotoFuture<CreateResourceDefinitionVersionResponse, CreateResourceDefinitionVersionError>;

    /// <p>Creates a software update for a core or group of cores (specified as an IoT thing group.) Use this to update the OTA Agent as well as the Greengrass core software. It makes use of the IoT Jobs feature which provides additional commands to manage a Greengrass core software update job.</p>
    fn create_software_update_job(
        &self,
        input: CreateSoftwareUpdateJobRequest,
    ) -> RusotoFuture<CreateSoftwareUpdateJobResponse, CreateSoftwareUpdateJobError>;

    /// <p>Creates a subscription definition. You may provide the initial version of the subscription definition now or use &#39;&#39;CreateSubscriptionDefinitionVersion&#39;&#39; at a later time.</p>
    fn create_subscription_definition(
        &self,
        input: CreateSubscriptionDefinitionRequest,
    ) -> RusotoFuture<CreateSubscriptionDefinitionResponse, CreateSubscriptionDefinitionError>;

    /// <p>Creates a version of a subscription definition which has already been defined.</p>
    fn create_subscription_definition_version(
        &self,
        input: CreateSubscriptionDefinitionVersionRequest,
    ) -> RusotoFuture<
        CreateSubscriptionDefinitionVersionResponse,
        CreateSubscriptionDefinitionVersionError,
    >;

    /// <p>Deletes a core definition.</p>
    fn delete_core_definition(
        &self,
        input: DeleteCoreDefinitionRequest,
    ) -> RusotoFuture<DeleteCoreDefinitionResponse, DeleteCoreDefinitionError>;

    /// <p>Deletes a device definition.</p>
    fn delete_device_definition(
        &self,
        input: DeleteDeviceDefinitionRequest,
    ) -> RusotoFuture<DeleteDeviceDefinitionResponse, DeleteDeviceDefinitionError>;

    /// <p>Deletes a Lambda function definition.</p>
    fn delete_function_definition(
        &self,
        input: DeleteFunctionDefinitionRequest,
    ) -> RusotoFuture<DeleteFunctionDefinitionResponse, DeleteFunctionDefinitionError>;

    /// <p>Deletes a group.</p>
    fn delete_group(
        &self,
        input: DeleteGroupRequest,
    ) -> RusotoFuture<DeleteGroupResponse, DeleteGroupError>;

    /// <p>Deletes a logger definition.</p>
    fn delete_logger_definition(
        &self,
        input: DeleteLoggerDefinitionRequest,
    ) -> RusotoFuture<DeleteLoggerDefinitionResponse, DeleteLoggerDefinitionError>;

    /// <p>Deletes a resource definition.</p>
    fn delete_resource_definition(
        &self,
        input: DeleteResourceDefinitionRequest,
    ) -> RusotoFuture<DeleteResourceDefinitionResponse, DeleteResourceDefinitionError>;

    /// <p>Deletes a subscription definition.</p>
    fn delete_subscription_definition(
        &self,
        input: DeleteSubscriptionDefinitionRequest,
    ) -> RusotoFuture<DeleteSubscriptionDefinitionResponse, DeleteSubscriptionDefinitionError>;

    /// <p>Disassociates the role from a group.</p>
    fn disassociate_role_from_group(
        &self,
        input: DisassociateRoleFromGroupRequest,
    ) -> RusotoFuture<DisassociateRoleFromGroupResponse, DisassociateRoleFromGroupError>;

    /// <p>Disassociates the service role from your account. Without a service role, deployments will not work.</p>
    fn disassociate_service_role_from_account(
        &self,
    ) -> RusotoFuture<
        DisassociateServiceRoleFromAccountResponse,
        DisassociateServiceRoleFromAccountError,
    >;

    /// <p>Retrieves the role associated with a particular group.</p>
    fn get_associated_role(
        &self,
        input: GetAssociatedRoleRequest,
    ) -> RusotoFuture<GetAssociatedRoleResponse, GetAssociatedRoleError>;

    /// <p>Retrieves the connectivity information for a core.</p>
    fn get_connectivity_info(
        &self,
        input: GetConnectivityInfoRequest,
    ) -> RusotoFuture<GetConnectivityInfoResponse, GetConnectivityInfoError>;

    /// <p>Retrieves information about a core definition version.</p>
    fn get_core_definition(
        &self,
        input: GetCoreDefinitionRequest,
    ) -> RusotoFuture<GetCoreDefinitionResponse, GetCoreDefinitionError>;

    /// <p>Retrieves information about a core definition version.</p>
    fn get_core_definition_version(
        &self,
        input: GetCoreDefinitionVersionRequest,
    ) -> RusotoFuture<GetCoreDefinitionVersionResponse, GetCoreDefinitionVersionError>;

    /// <p>Returns the status of a deployment.</p>
    fn get_deployment_status(
        &self,
        input: GetDeploymentStatusRequest,
    ) -> RusotoFuture<GetDeploymentStatusResponse, GetDeploymentStatusError>;

    /// <p>Retrieves information about a device definition.</p>
    fn get_device_definition(
        &self,
        input: GetDeviceDefinitionRequest,
    ) -> RusotoFuture<GetDeviceDefinitionResponse, GetDeviceDefinitionError>;

    /// <p>Retrieves information about a device definition version.</p>
    fn get_device_definition_version(
        &self,
        input: GetDeviceDefinitionVersionRequest,
    ) -> RusotoFuture<GetDeviceDefinitionVersionResponse, GetDeviceDefinitionVersionError>;

    /// <p>Retrieves information about a Lambda function definition, including its creation time and latest version.</p>
    fn get_function_definition(
        &self,
        input: GetFunctionDefinitionRequest,
    ) -> RusotoFuture<GetFunctionDefinitionResponse, GetFunctionDefinitionError>;

    /// <p>Retrieves information about a Lambda function definition version, including which Lambda functions are included in the version and their configurations.</p>
    fn get_function_definition_version(
        &self,
        input: GetFunctionDefinitionVersionRequest,
    ) -> RusotoFuture<GetFunctionDefinitionVersionResponse, GetFunctionDefinitionVersionError>;

    /// <p>Retrieves information about a group.</p>
    fn get_group(&self, input: GetGroupRequest) -> RusotoFuture<GetGroupResponse, GetGroupError>;

    /// <p>Retreives the CA associated with a group. Returns the public key of the CA.</p>
    fn get_group_certificate_authority(
        &self,
        input: GetGroupCertificateAuthorityRequest,
    ) -> RusotoFuture<GetGroupCertificateAuthorityResponse, GetGroupCertificateAuthorityError>;

    /// <p>Retrieves the current configuration for the CA used by the group.</p>
    fn get_group_certificate_configuration(
        &self,
        input: GetGroupCertificateConfigurationRequest,
    ) -> RusotoFuture<GetGroupCertificateConfigurationResponse, GetGroupCertificateConfigurationError>;

    /// <p>Retrieves information about a group version.</p>
    fn get_group_version(
        &self,
        input: GetGroupVersionRequest,
    ) -> RusotoFuture<GetGroupVersionResponse, GetGroupVersionError>;

    /// <p>Retrieves information about a logger definition.</p>
    fn get_logger_definition(
        &self,
        input: GetLoggerDefinitionRequest,
    ) -> RusotoFuture<GetLoggerDefinitionResponse, GetLoggerDefinitionError>;

    /// <p>Retrieves information about a logger definition version.</p>
    fn get_logger_definition_version(
        &self,
        input: GetLoggerDefinitionVersionRequest,
    ) -> RusotoFuture<GetLoggerDefinitionVersionResponse, GetLoggerDefinitionVersionError>;

    /// <p>Retrieves information about a resource definition, including its creation time and latest version.</p>
    fn get_resource_definition(
        &self,
        input: GetResourceDefinitionRequest,
    ) -> RusotoFuture<GetResourceDefinitionResponse, GetResourceDefinitionError>;

    /// <p>Retrieves information about a resource definition version, including which resources are included in the version.</p>
    fn get_resource_definition_version(
        &self,
        input: GetResourceDefinitionVersionRequest,
    ) -> RusotoFuture<GetResourceDefinitionVersionResponse, GetResourceDefinitionVersionError>;

    /// <p>Retrieves the service role that is attached to your account.</p>
    fn get_service_role_for_account(
        &self,
    ) -> RusotoFuture<GetServiceRoleForAccountResponse, GetServiceRoleForAccountError>;

    /// <p>Retrieves information about a subscription definition.</p>
    fn get_subscription_definition(
        &self,
        input: GetSubscriptionDefinitionRequest,
    ) -> RusotoFuture<GetSubscriptionDefinitionResponse, GetSubscriptionDefinitionError>;

    /// <p>Retrieves information about a subscription definition version.</p>
    fn get_subscription_definition_version(
        &self,
        input: GetSubscriptionDefinitionVersionRequest,
    ) -> RusotoFuture<GetSubscriptionDefinitionVersionResponse, GetSubscriptionDefinitionVersionError>;

    /// <p>Lists the versions of a core definition.</p>
    fn list_core_definition_versions(
        &self,
        input: ListCoreDefinitionVersionsRequest,
    ) -> RusotoFuture<ListCoreDefinitionVersionsResponse, ListCoreDefinitionVersionsError>;

    /// <p>Retrieves a list of core definitions.</p>
    fn list_core_definitions(
        &self,
        input: ListCoreDefinitionsRequest,
    ) -> RusotoFuture<ListCoreDefinitionsResponse, ListCoreDefinitionsError>;

    /// <p>Returns a history of deployments for the group.</p>
    fn list_deployments(
        &self,
        input: ListDeploymentsRequest,
    ) -> RusotoFuture<ListDeploymentsResponse, ListDeploymentsError>;

    /// <p>Lists the versions of a device definition.</p>
    fn list_device_definition_versions(
        &self,
        input: ListDeviceDefinitionVersionsRequest,
    ) -> RusotoFuture<ListDeviceDefinitionVersionsResponse, ListDeviceDefinitionVersionsError>;

    /// <p>Retrieves a list of device definitions.</p>
    fn list_device_definitions(
        &self,
        input: ListDeviceDefinitionsRequest,
    ) -> RusotoFuture<ListDeviceDefinitionsResponse, ListDeviceDefinitionsError>;

    /// <p>Lists the versions of a Lambda function definition.</p>
    fn list_function_definition_versions(
        &self,
        input: ListFunctionDefinitionVersionsRequest,
    ) -> RusotoFuture<ListFunctionDefinitionVersionsResponse, ListFunctionDefinitionVersionsError>;

    /// <p>Retrieves a list of Lambda function definitions.</p>
    fn list_function_definitions(
        &self,
        input: ListFunctionDefinitionsRequest,
    ) -> RusotoFuture<ListFunctionDefinitionsResponse, ListFunctionDefinitionsError>;

    /// <p>Retrieves the current CAs for a group.</p>
    fn list_group_certificate_authorities(
        &self,
        input: ListGroupCertificateAuthoritiesRequest,
    ) -> RusotoFuture<ListGroupCertificateAuthoritiesResponse, ListGroupCertificateAuthoritiesError>;

    /// <p>Lists the versions of a group.</p>
    fn list_group_versions(
        &self,
        input: ListGroupVersionsRequest,
    ) -> RusotoFuture<ListGroupVersionsResponse, ListGroupVersionsError>;

    /// <p>Retrieves a list of groups.</p>
    fn list_groups(
        &self,
        input: ListGroupsRequest,
    ) -> RusotoFuture<ListGroupsResponse, ListGroupsError>;

    /// <p>Lists the versions of a logger definition.</p>
    fn list_logger_definition_versions(
        &self,
        input: ListLoggerDefinitionVersionsRequest,
    ) -> RusotoFuture<ListLoggerDefinitionVersionsResponse, ListLoggerDefinitionVersionsError>;

    /// <p>Retrieves a list of logger definitions.</p>
    fn list_logger_definitions(
        &self,
        input: ListLoggerDefinitionsRequest,
    ) -> RusotoFuture<ListLoggerDefinitionsResponse, ListLoggerDefinitionsError>;

    /// <p>Lists the versions of a resource definition.</p>
    fn list_resource_definition_versions(
        &self,
        input: ListResourceDefinitionVersionsRequest,
    ) -> RusotoFuture<ListResourceDefinitionVersionsResponse, ListResourceDefinitionVersionsError>;

    /// <p>Retrieves a list of resource definitions.</p>
    fn list_resource_definitions(
        &self,
        input: ListResourceDefinitionsRequest,
    ) -> RusotoFuture<ListResourceDefinitionsResponse, ListResourceDefinitionsError>;

    /// <p>Lists the versions of a subscription definition.</p>
    fn list_subscription_definition_versions(
        &self,
        input: ListSubscriptionDefinitionVersionsRequest,
    ) -> RusotoFuture<
        ListSubscriptionDefinitionVersionsResponse,
        ListSubscriptionDefinitionVersionsError,
    >;

    /// <p>Retrieves a list of subscription definitions.</p>
    fn list_subscription_definitions(
        &self,
        input: ListSubscriptionDefinitionsRequest,
    ) -> RusotoFuture<ListSubscriptionDefinitionsResponse, ListSubscriptionDefinitionsError>;

    /// <p>Resets a group&#39;s deployments.</p>
    fn reset_deployments(
        &self,
        input: ResetDeploymentsRequest,
    ) -> RusotoFuture<ResetDeploymentsResponse, ResetDeploymentsError>;

    /// <p>Updates the connectivity information for the core. Any devices that belong to the group which has this core will receive this information in order to find the location of the core and connect to it.</p>
    fn update_connectivity_info(
        &self,
        input: UpdateConnectivityInfoRequest,
    ) -> RusotoFuture<UpdateConnectivityInfoResponse, UpdateConnectivityInfoError>;

    /// <p>Updates a core definition.</p>
    fn update_core_definition(
        &self,
        input: UpdateCoreDefinitionRequest,
    ) -> RusotoFuture<UpdateCoreDefinitionResponse, UpdateCoreDefinitionError>;

    /// <p>Updates a device definition.</p>
    fn update_device_definition(
        &self,
        input: UpdateDeviceDefinitionRequest,
    ) -> RusotoFuture<UpdateDeviceDefinitionResponse, UpdateDeviceDefinitionError>;

    /// <p>Updates a Lambda function definition.</p>
    fn update_function_definition(
        &self,
        input: UpdateFunctionDefinitionRequest,
    ) -> RusotoFuture<UpdateFunctionDefinitionResponse, UpdateFunctionDefinitionError>;

    /// <p>Updates a group.</p>
    fn update_group(
        &self,
        input: UpdateGroupRequest,
    ) -> RusotoFuture<UpdateGroupResponse, UpdateGroupError>;

    /// <p>Updates the Certificate expiry time for a group.</p>
    fn update_group_certificate_configuration(
        &self,
        input: UpdateGroupCertificateConfigurationRequest,
    ) -> RusotoFuture<
        UpdateGroupCertificateConfigurationResponse,
        UpdateGroupCertificateConfigurationError,
    >;

    /// <p>Updates a logger definition.</p>
    fn update_logger_definition(
        &self,
        input: UpdateLoggerDefinitionRequest,
    ) -> RusotoFuture<UpdateLoggerDefinitionResponse, UpdateLoggerDefinitionError>;

    /// <p>Updates a resource definition.</p>
    fn update_resource_definition(
        &self,
        input: UpdateResourceDefinitionRequest,
    ) -> RusotoFuture<UpdateResourceDefinitionResponse, UpdateResourceDefinitionError>;

    /// <p>Updates a subscription definition.</p>
    fn update_subscription_definition(
        &self,
        input: UpdateSubscriptionDefinitionRequest,
    ) -> RusotoFuture<UpdateSubscriptionDefinitionResponse, UpdateSubscriptionDefinitionError>;
}
/// A client for the AWS Greengrass API.
pub struct GreenGrassClient {
    client: Client,
    region: region::Region,
}

impl GreenGrassClient {
    /// Creates a client backed by the default tokio event loop.
    ///
    /// The client will use the default credentials provider and tls client.
    pub fn new(region: region::Region) -> GreenGrassClient {
        GreenGrassClient {
            client: Client::shared(),
            region: region,
        }
    }

    pub fn new_with<P, D>(
        request_dispatcher: D,
        credentials_provider: P,
        region: region::Region,
    ) -> GreenGrassClient
    where
        P: ProvideAwsCredentials + Send + Sync + 'static,
        P::Future: Send,
        D: DispatchSignedRequest + Send + Sync + 'static,
        D::Future: Send,
    {
        GreenGrassClient {
            client: Client::new_with(credentials_provider, request_dispatcher),
            region: region,
        }
    }
}

impl GreenGrass for GreenGrassClient {
    /// <p>Associates a role with a group. Your AWS Greengrass core will use the role to access AWS cloud services. The role&#39;s permissions should allow Greengrass core Lambda functions to perform actions against the cloud.</p>
    fn associate_role_to_group(
        &self,
        input: AssociateRoleToGroupRequest,
    ) -> RusotoFuture<AssociateRoleToGroupResponse, AssociateRoleToGroupError> {
        let request_uri = format!(
            "/greengrass/groups/{group_id}/role",
            group_id = input.group_id
        );

        let mut request = SignedRequest::new("PUT", "greengrass", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result =
                        serde_json::from_slice::<AssociateRoleToGroupResponse>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(AssociateRoleToGroupError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        })
    }

    /// <p>Associates a role with your account. AWS Greengrass will use the role to access your Lambda functions and AWS IoT resources. This is necessary for deployments to succeed. The role must have at least minimum permissions in the policy &#39;&#39;AWSGreengrassResourceAccessRolePolicy&#39;&#39;.</p>
    fn associate_service_role_to_account(
        &self,
        input: AssociateServiceRoleToAccountRequest,
    ) -> RusotoFuture<AssociateServiceRoleToAccountResponse, AssociateServiceRoleToAccountError>
    {
        let request_uri = "/greengrass/servicerole";

        let mut request = SignedRequest::new("PUT", "greengrass", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result =
                        serde_json::from_slice::<AssociateServiceRoleToAccountResponse>(&body)
                            .unwrap();

                    result
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(AssociateServiceRoleToAccountError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        })
    }

    /// <p>Creates a core definition. You may provide the initial version of the core definition now or use &#39;&#39;CreateCoreDefinitionVersion&#39;&#39; at a later time. AWS Greengrass groups must each contain exactly one AWS Greengrass core.</p>
    fn create_core_definition(
        &self,
        input: CreateCoreDefinitionRequest,
    ) -> RusotoFuture<CreateCoreDefinitionResponse, CreateCoreDefinitionError> {
        let request_uri = "/greengrass/definition/cores";

        let mut request = SignedRequest::new("POST", "greengrass", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        if let Some(ref amzn_client_token) = input.amzn_client_token {
            request.add_header("X-Amzn-Client-Token", &amzn_client_token.to_string());
        }

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result =
                        serde_json::from_slice::<CreateCoreDefinitionResponse>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(CreateCoreDefinitionError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        })
    }

    /// <p>Creates a version of a core definition that has already been defined. AWS Greengrass groups must each contain exactly one AWS Greengrass core.</p>
    fn create_core_definition_version(
        &self,
        input: CreateCoreDefinitionVersionRequest,
    ) -> RusotoFuture<CreateCoreDefinitionVersionResponse, CreateCoreDefinitionVersionError> {
        let request_uri = format!(
            "/greengrass/definition/cores/{core_definition_id}/versions",
            core_definition_id = input.core_definition_id
        );

        let mut request = SignedRequest::new("POST", "greengrass", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        if let Some(ref amzn_client_token) = input.amzn_client_token {
            request.add_header("X-Amzn-Client-Token", &amzn_client_token.to_string());
        }

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result =
                        serde_json::from_slice::<CreateCoreDefinitionVersionResponse>(&body)
                            .unwrap();

                    result
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(CreateCoreDefinitionVersionError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        })
    }

    /// <p>Creates a deployment.</p>
    fn create_deployment(
        &self,
        input: CreateDeploymentRequest,
    ) -> RusotoFuture<CreateDeploymentResponse, CreateDeploymentError> {
        let request_uri = format!(
            "/greengrass/groups/{group_id}/deployments",
            group_id = input.group_id
        );

        let mut request = SignedRequest::new("POST", "greengrass", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        if let Some(ref amzn_client_token) = input.amzn_client_token {
            request.add_header("X-Amzn-Client-Token", &amzn_client_token.to_string());
        }

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result = serde_json::from_slice::<CreateDeploymentResponse>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(CreateDeploymentError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        })
    }

    /// <p>Creates a device definition. You may provide the initial version of the device definition now or use &#39;&#39;CreateDeviceDefinitionVersion&#39;&#39; at a later time.</p>
    fn create_device_definition(
        &self,
        input: CreateDeviceDefinitionRequest,
    ) -> RusotoFuture<CreateDeviceDefinitionResponse, CreateDeviceDefinitionError> {
        let request_uri = "/greengrass/definition/devices";

        let mut request = SignedRequest::new("POST", "greengrass", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        if let Some(ref amzn_client_token) = input.amzn_client_token {
            request.add_header("X-Amzn-Client-Token", &amzn_client_token.to_string());
        }

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result =
                        serde_json::from_slice::<CreateDeviceDefinitionResponse>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(CreateDeviceDefinitionError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        })
    }

    /// <p>Creates a version of a device definition that has already been defined.</p>
    fn create_device_definition_version(
        &self,
        input: CreateDeviceDefinitionVersionRequest,
    ) -> RusotoFuture<CreateDeviceDefinitionVersionResponse, CreateDeviceDefinitionVersionError>
    {
        let request_uri = format!(
            "/greengrass/definition/devices/{device_definition_id}/versions",
            device_definition_id = input.device_definition_id
        );

        let mut request = SignedRequest::new("POST", "greengrass", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        if let Some(ref amzn_client_token) = input.amzn_client_token {
            request.add_header("X-Amzn-Client-Token", &amzn_client_token.to_string());
        }

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result =
                        serde_json::from_slice::<CreateDeviceDefinitionVersionResponse>(&body)
                            .unwrap();

                    result
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(CreateDeviceDefinitionVersionError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        })
    }

    /// <p>Creates a Lambda function definition which contains a list of Lambda functions and their configurations to be used in a group. You can create an initial version of the definition by providing a list of Lambda functions and their configurations now, or use &#39;&#39;CreateFunctionDefinitionVersion&#39;&#39; later.</p>
    fn create_function_definition(
        &self,
        input: CreateFunctionDefinitionRequest,
    ) -> RusotoFuture<CreateFunctionDefinitionResponse, CreateFunctionDefinitionError> {
        let request_uri = "/greengrass/definition/functions";

        let mut request = SignedRequest::new("POST", "greengrass", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        if let Some(ref amzn_client_token) = input.amzn_client_token {
            request.add_header("X-Amzn-Client-Token", &amzn_client_token.to_string());
        }

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result =
                        serde_json::from_slice::<CreateFunctionDefinitionResponse>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(CreateFunctionDefinitionError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        })
    }

    /// <p>Creates a version of a Lambda function definition that has already been defined.</p>
    fn create_function_definition_version(
        &self,
        input: CreateFunctionDefinitionVersionRequest,
    ) -> RusotoFuture<CreateFunctionDefinitionVersionResponse, CreateFunctionDefinitionVersionError>
    {
        let request_uri = format!(
            "/greengrass/definition/functions/{function_definition_id}/versions",
            function_definition_id = input.function_definition_id
        );

        let mut request = SignedRequest::new("POST", "greengrass", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        if let Some(ref amzn_client_token) = input.amzn_client_token {
            request.add_header("X-Amzn-Client-Token", &amzn_client_token.to_string());
        }

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result =
                        serde_json::from_slice::<CreateFunctionDefinitionVersionResponse>(&body)
                            .unwrap();

                    result
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(CreateFunctionDefinitionVersionError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        })
    }

    /// <p>Creates a group. You may provide the initial version of the group or use &#39;&#39;CreateGroupVersion&#39;&#39; at a later time.</p>
    fn create_group(
        &self,
        input: CreateGroupRequest,
    ) -> RusotoFuture<CreateGroupResponse, CreateGroupError> {
        let request_uri = "/greengrass/groups";

        let mut request = SignedRequest::new("POST", "greengrass", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        if let Some(ref amzn_client_token) = input.amzn_client_token {
            request.add_header("X-Amzn-Client-Token", &amzn_client_token.to_string());
        }

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result = serde_json::from_slice::<CreateGroupResponse>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(CreateGroupError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        })
    }

    /// <p>Creates a CA for the group. If a CA already exists, it will rotate the existing CA.</p>
    fn create_group_certificate_authority(
        &self,
        input: CreateGroupCertificateAuthorityRequest,
    ) -> RusotoFuture<CreateGroupCertificateAuthorityResponse, CreateGroupCertificateAuthorityError>
    {
        let request_uri = format!(
            "/greengrass/groups/{group_id}/certificateauthorities",
            group_id = input.group_id
        );

        let mut request = SignedRequest::new("POST", "greengrass", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        if let Some(ref amzn_client_token) = input.amzn_client_token {
            request.add_header("X-Amzn-Client-Token", &amzn_client_token.to_string());
        }

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result =
                        serde_json::from_slice::<CreateGroupCertificateAuthorityResponse>(&body)
                            .unwrap();

                    result
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(CreateGroupCertificateAuthorityError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        })
    }

    /// <p>Creates a version of a group which has already been defined.</p>
    fn create_group_version(
        &self,
        input: CreateGroupVersionRequest,
    ) -> RusotoFuture<CreateGroupVersionResponse, CreateGroupVersionError> {
        let request_uri = format!(
            "/greengrass/groups/{group_id}/versions",
            group_id = input.group_id
        );

        let mut request = SignedRequest::new("POST", "greengrass", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        if let Some(ref amzn_client_token) = input.amzn_client_token {
            request.add_header("X-Amzn-Client-Token", &amzn_client_token.to_string());
        }

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result =
                        serde_json::from_slice::<CreateGroupVersionResponse>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(CreateGroupVersionError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        })
    }

    /// <p>Creates a logger definition. You may provide the initial version of the logger definition now or use &#39;&#39;CreateLoggerDefinitionVersion&#39;&#39; at a later time.</p>
    fn create_logger_definition(
        &self,
        input: CreateLoggerDefinitionRequest,
    ) -> RusotoFuture<CreateLoggerDefinitionResponse, CreateLoggerDefinitionError> {
        let request_uri = "/greengrass/definition/loggers";

        let mut request = SignedRequest::new("POST", "greengrass", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        if let Some(ref amzn_client_token) = input.amzn_client_token {
            request.add_header("X-Amzn-Client-Token", &amzn_client_token.to_string());
        }

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result =
                        serde_json::from_slice::<CreateLoggerDefinitionResponse>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(CreateLoggerDefinitionError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        })
    }

    /// <p>Creates a version of a logger definition that has already been defined.</p>
    fn create_logger_definition_version(
        &self,
        input: CreateLoggerDefinitionVersionRequest,
    ) -> RusotoFuture<CreateLoggerDefinitionVersionResponse, CreateLoggerDefinitionVersionError>
    {
        let request_uri = format!(
            "/greengrass/definition/loggers/{logger_definition_id}/versions",
            logger_definition_id = input.logger_definition_id
        );

        let mut request = SignedRequest::new("POST", "greengrass", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        if let Some(ref amzn_client_token) = input.amzn_client_token {
            request.add_header("X-Amzn-Client-Token", &amzn_client_token.to_string());
        }

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result =
                        serde_json::from_slice::<CreateLoggerDefinitionVersionResponse>(&body)
                            .unwrap();

                    result
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(CreateLoggerDefinitionVersionError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        })
    }

    /// <p>Creates a resource definition which contains a list of resources to be used in a group. You can create an initial version of the definition by providing a list of resources now, or use &#39;&#39;CreateResourceDefinitionVersion&#39;&#39; later.</p>
    fn create_resource_definition(
        &self,
        input: CreateResourceDefinitionRequest,
    ) -> RusotoFuture<CreateResourceDefinitionResponse, CreateResourceDefinitionError> {
        let request_uri = "/greengrass/definition/resources";

        let mut request = SignedRequest::new("POST", "greengrass", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        if let Some(ref amzn_client_token) = input.amzn_client_token {
            request.add_header("X-Amzn-Client-Token", &amzn_client_token.to_string());
        }

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result =
                        serde_json::from_slice::<CreateResourceDefinitionResponse>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(CreateResourceDefinitionError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        })
    }

    /// <p>Creates a version of a resource definition that has already been defined.</p>
    fn create_resource_definition_version(
        &self,
        input: CreateResourceDefinitionVersionRequest,
    ) -> RusotoFuture<CreateResourceDefinitionVersionResponse, CreateResourceDefinitionVersionError>
    {
        let request_uri = format!(
            "/greengrass/definition/resources/{resource_definition_id}/versions",
            resource_definition_id = input.resource_definition_id
        );

        let mut request = SignedRequest::new("POST", "greengrass", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        if let Some(ref amzn_client_token) = input.amzn_client_token {
            request.add_header("X-Amzn-Client-Token", &amzn_client_token.to_string());
        }

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result =
                        serde_json::from_slice::<CreateResourceDefinitionVersionResponse>(&body)
                            .unwrap();

                    result
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(CreateResourceDefinitionVersionError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        })
    }

    /// <p>Creates a software update for a core or group of cores (specified as an IoT thing group.) Use this to update the OTA Agent as well as the Greengrass core software. It makes use of the IoT Jobs feature which provides additional commands to manage a Greengrass core software update job.</p>
    fn create_software_update_job(
        &self,
        input: CreateSoftwareUpdateJobRequest,
    ) -> RusotoFuture<CreateSoftwareUpdateJobResponse, CreateSoftwareUpdateJobError> {
        let request_uri = "/greengrass/updates";

        let mut request = SignedRequest::new("POST", "greengrass", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        if let Some(ref amzn_client_token) = input.amzn_client_token {
            request.add_header("X-Amzn-Client-Token", &amzn_client_token.to_string());
        }

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result =
                        serde_json::from_slice::<CreateSoftwareUpdateJobResponse>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(CreateSoftwareUpdateJobError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        })
    }

    /// <p>Creates a subscription definition. You may provide the initial version of the subscription definition now or use &#39;&#39;CreateSubscriptionDefinitionVersion&#39;&#39; at a later time.</p>
    fn create_subscription_definition(
        &self,
        input: CreateSubscriptionDefinitionRequest,
    ) -> RusotoFuture<CreateSubscriptionDefinitionResponse, CreateSubscriptionDefinitionError> {
        let request_uri = "/greengrass/definition/subscriptions";

        let mut request = SignedRequest::new("POST", "greengrass", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        if let Some(ref amzn_client_token) = input.amzn_client_token {
            request.add_header("X-Amzn-Client-Token", &amzn_client_token.to_string());
        }

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result =
                        serde_json::from_slice::<CreateSubscriptionDefinitionResponse>(&body)
                            .unwrap();

                    result
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(CreateSubscriptionDefinitionError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        })
    }

    /// <p>Creates a version of a subscription definition which has already been defined.</p>
    fn create_subscription_definition_version(
        &self,
        input: CreateSubscriptionDefinitionVersionRequest,
    ) -> RusotoFuture<
        CreateSubscriptionDefinitionVersionResponse,
        CreateSubscriptionDefinitionVersionError,
    > {
        let request_uri = format!(
            "/greengrass/definition/subscriptions/{subscription_definition_id}/versions",
            subscription_definition_id = input.subscription_definition_id
        );

        let mut request = SignedRequest::new("POST", "greengrass", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        if let Some(ref amzn_client_token) = input.amzn_client_token {
            request.add_header("X-Amzn-Client-Token", &amzn_client_token.to_string());
        }

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result = serde_json::from_slice::<
                        CreateSubscriptionDefinitionVersionResponse,
                    >(&body).unwrap();

                    result
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(CreateSubscriptionDefinitionVersionError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        })
    }

    /// <p>Deletes a core definition.</p>
    fn delete_core_definition(
        &self,
        input: DeleteCoreDefinitionRequest,
    ) -> RusotoFuture<DeleteCoreDefinitionResponse, DeleteCoreDefinitionError> {
        let request_uri = format!(
            "/greengrass/definition/cores/{core_definition_id}",
            core_definition_id = input.core_definition_id
        );

        let mut request = SignedRequest::new("DELETE", "greengrass", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result =
                        serde_json::from_slice::<DeleteCoreDefinitionResponse>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(DeleteCoreDefinitionError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        })
    }

    /// <p>Deletes a device definition.</p>
    fn delete_device_definition(
        &self,
        input: DeleteDeviceDefinitionRequest,
    ) -> RusotoFuture<DeleteDeviceDefinitionResponse, DeleteDeviceDefinitionError> {
        let request_uri = format!(
            "/greengrass/definition/devices/{device_definition_id}",
            device_definition_id = input.device_definition_id
        );

        let mut request = SignedRequest::new("DELETE", "greengrass", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result =
                        serde_json::from_slice::<DeleteDeviceDefinitionResponse>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(DeleteDeviceDefinitionError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        })
    }

    /// <p>Deletes a Lambda function definition.</p>
    fn delete_function_definition(
        &self,
        input: DeleteFunctionDefinitionRequest,
    ) -> RusotoFuture<DeleteFunctionDefinitionResponse, DeleteFunctionDefinitionError> {
        let request_uri = format!(
            "/greengrass/definition/functions/{function_definition_id}",
            function_definition_id = input.function_definition_id
        );

        let mut request = SignedRequest::new("DELETE", "greengrass", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result =
                        serde_json::from_slice::<DeleteFunctionDefinitionResponse>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(DeleteFunctionDefinitionError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        })
    }

    /// <p>Deletes a group.</p>
    fn delete_group(
        &self,
        input: DeleteGroupRequest,
    ) -> RusotoFuture<DeleteGroupResponse, DeleteGroupError> {
        let request_uri = format!("/greengrass/groups/{group_id}", group_id = input.group_id);

        let mut request = SignedRequest::new("DELETE", "greengrass", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result = serde_json::from_slice::<DeleteGroupResponse>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(DeleteGroupError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        })
    }

    /// <p>Deletes a logger definition.</p>
    fn delete_logger_definition(
        &self,
        input: DeleteLoggerDefinitionRequest,
    ) -> RusotoFuture<DeleteLoggerDefinitionResponse, DeleteLoggerDefinitionError> {
        let request_uri = format!(
            "/greengrass/definition/loggers/{logger_definition_id}",
            logger_definition_id = input.logger_definition_id
        );

        let mut request = SignedRequest::new("DELETE", "greengrass", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result =
                        serde_json::from_slice::<DeleteLoggerDefinitionResponse>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(DeleteLoggerDefinitionError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        })
    }

    /// <p>Deletes a resource definition.</p>
    fn delete_resource_definition(
        &self,
        input: DeleteResourceDefinitionRequest,
    ) -> RusotoFuture<DeleteResourceDefinitionResponse, DeleteResourceDefinitionError> {
        let request_uri = format!(
            "/greengrass/definition/resources/{resource_definition_id}",
            resource_definition_id = input.resource_definition_id
        );

        let mut request = SignedRequest::new("DELETE", "greengrass", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result =
                        serde_json::from_slice::<DeleteResourceDefinitionResponse>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(DeleteResourceDefinitionError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        })
    }

    /// <p>Deletes a subscription definition.</p>
    fn delete_subscription_definition(
        &self,
        input: DeleteSubscriptionDefinitionRequest,
    ) -> RusotoFuture<DeleteSubscriptionDefinitionResponse, DeleteSubscriptionDefinitionError> {
        let request_uri = format!(
            "/greengrass/definition/subscriptions/{subscription_definition_id}",
            subscription_definition_id = input.subscription_definition_id
        );

        let mut request = SignedRequest::new("DELETE", "greengrass", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result =
                        serde_json::from_slice::<DeleteSubscriptionDefinitionResponse>(&body)
                            .unwrap();

                    result
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(DeleteSubscriptionDefinitionError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        })
    }

    /// <p>Disassociates the role from a group.</p>
    fn disassociate_role_from_group(
        &self,
        input: DisassociateRoleFromGroupRequest,
    ) -> RusotoFuture<DisassociateRoleFromGroupResponse, DisassociateRoleFromGroupError> {
        let request_uri = format!(
            "/greengrass/groups/{group_id}/role",
            group_id = input.group_id
        );

        let mut request = SignedRequest::new("DELETE", "greengrass", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result =
                        serde_json::from_slice::<DisassociateRoleFromGroupResponse>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(DisassociateRoleFromGroupError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        })
    }

    /// <p>Disassociates the service role from your account. Without a service role, deployments will not work.</p>
    fn disassociate_service_role_from_account(
        &self,
    ) -> RusotoFuture<
        DisassociateServiceRoleFromAccountResponse,
        DisassociateServiceRoleFromAccountError,
    > {
        let request_uri = "/greengrass/servicerole";

        let mut request = SignedRequest::new("DELETE", "greengrass", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result =
                        serde_json::from_slice::<DisassociateServiceRoleFromAccountResponse>(&body)
                            .unwrap();

                    result
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(DisassociateServiceRoleFromAccountError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        })
    }

    /// <p>Retrieves the role associated with a particular group.</p>
    fn get_associated_role(
        &self,
        input: GetAssociatedRoleRequest,
    ) -> RusotoFuture<GetAssociatedRoleResponse, GetAssociatedRoleError> {
        let request_uri = format!(
            "/greengrass/groups/{group_id}/role",
            group_id = input.group_id
        );

        let mut request = SignedRequest::new("GET", "greengrass", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result =
                        serde_json::from_slice::<GetAssociatedRoleResponse>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(GetAssociatedRoleError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        })
    }

    /// <p>Retrieves the connectivity information for a core.</p>
    fn get_connectivity_info(
        &self,
        input: GetConnectivityInfoRequest,
    ) -> RusotoFuture<GetConnectivityInfoResponse, GetConnectivityInfoError> {
        let request_uri = format!(
            "/greengrass/things/{thing_name}/connectivityInfo",
            thing_name = input.thing_name
        );

        let mut request = SignedRequest::new("GET", "greengrass", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result =
                        serde_json::from_slice::<GetConnectivityInfoResponse>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(GetConnectivityInfoError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        })
    }

    /// <p>Retrieves information about a core definition version.</p>
    fn get_core_definition(
        &self,
        input: GetCoreDefinitionRequest,
    ) -> RusotoFuture<GetCoreDefinitionResponse, GetCoreDefinitionError> {
        let request_uri = format!(
            "/greengrass/definition/cores/{core_definition_id}",
            core_definition_id = input.core_definition_id
        );

        let mut request = SignedRequest::new("GET", "greengrass", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result =
                        serde_json::from_slice::<GetCoreDefinitionResponse>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(GetCoreDefinitionError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        })
    }

    /// <p>Retrieves information about a core definition version.</p>
    fn get_core_definition_version(
        &self,
        input: GetCoreDefinitionVersionRequest,
    ) -> RusotoFuture<GetCoreDefinitionVersionResponse, GetCoreDefinitionVersionError> {
        let request_uri = format!("/greengrass/definition/cores/{core_definition_id}/versions/{core_definition_version_id}", core_definition_id = input.core_definition_id, core_definition_version_id = input.core_definition_version_id);

        let mut request = SignedRequest::new("GET", "greengrass", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result =
                        serde_json::from_slice::<GetCoreDefinitionVersionResponse>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(GetCoreDefinitionVersionError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        })
    }

    /// <p>Returns the status of a deployment.</p>
    fn get_deployment_status(
        &self,
        input: GetDeploymentStatusRequest,
    ) -> RusotoFuture<GetDeploymentStatusResponse, GetDeploymentStatusError> {
        let request_uri = format!(
            "/greengrass/groups/{group_id}/deployments/{deployment_id}/status",
            deployment_id = input.deployment_id,
            group_id = input.group_id
        );

        let mut request = SignedRequest::new("GET", "greengrass", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result =
                        serde_json::from_slice::<GetDeploymentStatusResponse>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(GetDeploymentStatusError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        })
    }

    /// <p>Retrieves information about a device definition.</p>
    fn get_device_definition(
        &self,
        input: GetDeviceDefinitionRequest,
    ) -> RusotoFuture<GetDeviceDefinitionResponse, GetDeviceDefinitionError> {
        let request_uri = format!(
            "/greengrass/definition/devices/{device_definition_id}",
            device_definition_id = input.device_definition_id
        );

        let mut request = SignedRequest::new("GET", "greengrass", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result =
                        serde_json::from_slice::<GetDeviceDefinitionResponse>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(GetDeviceDefinitionError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        })
    }

    /// <p>Retrieves information about a device definition version.</p>
    fn get_device_definition_version(
        &self,
        input: GetDeviceDefinitionVersionRequest,
    ) -> RusotoFuture<GetDeviceDefinitionVersionResponse, GetDeviceDefinitionVersionError> {
        let request_uri = format!("/greengrass/definition/devices/{device_definition_id}/versions/{device_definition_version_id}", device_definition_id = input.device_definition_id, device_definition_version_id = input.device_definition_version_id);

        let mut request = SignedRequest::new("GET", "greengrass", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result =
                        serde_json::from_slice::<GetDeviceDefinitionVersionResponse>(&body)
                            .unwrap();

                    result
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(GetDeviceDefinitionVersionError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        })
    }

    /// <p>Retrieves information about a Lambda function definition, including its creation time and latest version.</p>
    fn get_function_definition(
        &self,
        input: GetFunctionDefinitionRequest,
    ) -> RusotoFuture<GetFunctionDefinitionResponse, GetFunctionDefinitionError> {
        let request_uri = format!(
            "/greengrass/definition/functions/{function_definition_id}",
            function_definition_id = input.function_definition_id
        );

        let mut request = SignedRequest::new("GET", "greengrass", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result =
                        serde_json::from_slice::<GetFunctionDefinitionResponse>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(GetFunctionDefinitionError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        })
    }

    /// <p>Retrieves information about a Lambda function definition version, including which Lambda functions are included in the version and their configurations.</p>
    fn get_function_definition_version(
        &self,
        input: GetFunctionDefinitionVersionRequest,
    ) -> RusotoFuture<GetFunctionDefinitionVersionResponse, GetFunctionDefinitionVersionError> {
        let request_uri = format!("/greengrass/definition/functions/{function_definition_id}/versions/{function_definition_version_id}", function_definition_id = input.function_definition_id, function_definition_version_id = input.function_definition_version_id);

        let mut request = SignedRequest::new("GET", "greengrass", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result =
                        serde_json::from_slice::<GetFunctionDefinitionVersionResponse>(&body)
                            .unwrap();

                    result
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(GetFunctionDefinitionVersionError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        })
    }

    /// <p>Retrieves information about a group.</p>
    fn get_group(&self, input: GetGroupRequest) -> RusotoFuture<GetGroupResponse, GetGroupError> {
        let request_uri = format!("/greengrass/groups/{group_id}", group_id = input.group_id);

        let mut request = SignedRequest::new("GET", "greengrass", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result = serde_json::from_slice::<GetGroupResponse>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(GetGroupError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        })
    }

    /// <p>Retreives the CA associated with a group. Returns the public key of the CA.</p>
    fn get_group_certificate_authority(
        &self,
        input: GetGroupCertificateAuthorityRequest,
    ) -> RusotoFuture<GetGroupCertificateAuthorityResponse, GetGroupCertificateAuthorityError> {
        let request_uri = format!(
            "/greengrass/groups/{group_id}/certificateauthorities/{certificate_authority_id}",
            certificate_authority_id = input.certificate_authority_id,
            group_id = input.group_id
        );

        let mut request = SignedRequest::new("GET", "greengrass", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result =
                        serde_json::from_slice::<GetGroupCertificateAuthorityResponse>(&body)
                            .unwrap();

                    result
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(GetGroupCertificateAuthorityError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        })
    }

    /// <p>Retrieves the current configuration for the CA used by the group.</p>
    fn get_group_certificate_configuration(
        &self,
        input: GetGroupCertificateConfigurationRequest,
    ) -> RusotoFuture<GetGroupCertificateConfigurationResponse, GetGroupCertificateConfigurationError>
    {
        let request_uri = format!(
            "/greengrass/groups/{group_id}/certificateauthorities/configuration/expiry",
            group_id = input.group_id
        );

        let mut request = SignedRequest::new("GET", "greengrass", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result =
                        serde_json::from_slice::<GetGroupCertificateConfigurationResponse>(&body)
                            .unwrap();

                    result
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(GetGroupCertificateConfigurationError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        })
    }

    /// <p>Retrieves information about a group version.</p>
    fn get_group_version(
        &self,
        input: GetGroupVersionRequest,
    ) -> RusotoFuture<GetGroupVersionResponse, GetGroupVersionError> {
        let request_uri = format!(
            "/greengrass/groups/{group_id}/versions/{group_version_id}",
            group_id = input.group_id,
            group_version_id = input.group_version_id
        );

        let mut request = SignedRequest::new("GET", "greengrass", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result = serde_json::from_slice::<GetGroupVersionResponse>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(GetGroupVersionError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        })
    }

    /// <p>Retrieves information about a logger definition.</p>
    fn get_logger_definition(
        &self,
        input: GetLoggerDefinitionRequest,
    ) -> RusotoFuture<GetLoggerDefinitionResponse, GetLoggerDefinitionError> {
        let request_uri = format!(
            "/greengrass/definition/loggers/{logger_definition_id}",
            logger_definition_id = input.logger_definition_id
        );

        let mut request = SignedRequest::new("GET", "greengrass", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result =
                        serde_json::from_slice::<GetLoggerDefinitionResponse>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(GetLoggerDefinitionError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        })
    }

    /// <p>Retrieves information about a logger definition version.</p>
    fn get_logger_definition_version(
        &self,
        input: GetLoggerDefinitionVersionRequest,
    ) -> RusotoFuture<GetLoggerDefinitionVersionResponse, GetLoggerDefinitionVersionError> {
        let request_uri = format!("/greengrass/definition/loggers/{logger_definition_id}/versions/{logger_definition_version_id}", logger_definition_id = input.logger_definition_id, logger_definition_version_id = input.logger_definition_version_id);

        let mut request = SignedRequest::new("GET", "greengrass", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result =
                        serde_json::from_slice::<GetLoggerDefinitionVersionResponse>(&body)
                            .unwrap();

                    result
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(GetLoggerDefinitionVersionError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        })
    }

    /// <p>Retrieves information about a resource definition, including its creation time and latest version.</p>
    fn get_resource_definition(
        &self,
        input: GetResourceDefinitionRequest,
    ) -> RusotoFuture<GetResourceDefinitionResponse, GetResourceDefinitionError> {
        let request_uri = format!(
            "/greengrass/definition/resources/{resource_definition_id}",
            resource_definition_id = input.resource_definition_id
        );

        let mut request = SignedRequest::new("GET", "greengrass", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result =
                        serde_json::from_slice::<GetResourceDefinitionResponse>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(GetResourceDefinitionError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        })
    }

    /// <p>Retrieves information about a resource definition version, including which resources are included in the version.</p>
    fn get_resource_definition_version(
        &self,
        input: GetResourceDefinitionVersionRequest,
    ) -> RusotoFuture<GetResourceDefinitionVersionResponse, GetResourceDefinitionVersionError> {
        let request_uri = format!("/greengrass/definition/resources/{resource_definition_id}/versions/{resource_definition_version_id}", resource_definition_id = input.resource_definition_id, resource_definition_version_id = input.resource_definition_version_id);

        let mut request = SignedRequest::new("GET", "greengrass", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result =
                        serde_json::from_slice::<GetResourceDefinitionVersionResponse>(&body)
                            .unwrap();

                    result
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(GetResourceDefinitionVersionError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        })
    }

    /// <p>Retrieves the service role that is attached to your account.</p>
    fn get_service_role_for_account(
        &self,
    ) -> RusotoFuture<GetServiceRoleForAccountResponse, GetServiceRoleForAccountError> {
        let request_uri = "/greengrass/servicerole";

        let mut request = SignedRequest::new("GET", "greengrass", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result =
                        serde_json::from_slice::<GetServiceRoleForAccountResponse>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(GetServiceRoleForAccountError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        })
    }

    /// <p>Retrieves information about a subscription definition.</p>
    fn get_subscription_definition(
        &self,
        input: GetSubscriptionDefinitionRequest,
    ) -> RusotoFuture<GetSubscriptionDefinitionResponse, GetSubscriptionDefinitionError> {
        let request_uri = format!(
            "/greengrass/definition/subscriptions/{subscription_definition_id}",
            subscription_definition_id = input.subscription_definition_id
        );

        let mut request = SignedRequest::new("GET", "greengrass", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result =
                        serde_json::from_slice::<GetSubscriptionDefinitionResponse>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(GetSubscriptionDefinitionError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        })
    }

    /// <p>Retrieves information about a subscription definition version.</p>
    fn get_subscription_definition_version(
        &self,
        input: GetSubscriptionDefinitionVersionRequest,
    ) -> RusotoFuture<GetSubscriptionDefinitionVersionResponse, GetSubscriptionDefinitionVersionError>
    {
        let request_uri = format!("/greengrass/definition/subscriptions/{subscription_definition_id}/versions/{subscription_definition_version_id}", subscription_definition_id = input.subscription_definition_id, subscription_definition_version_id = input.subscription_definition_version_id);

        let mut request = SignedRequest::new("GET", "greengrass", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result =
                        serde_json::from_slice::<GetSubscriptionDefinitionVersionResponse>(&body)
                            .unwrap();

                    result
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(GetSubscriptionDefinitionVersionError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        })
    }

    /// <p>Lists the versions of a core definition.</p>
    fn list_core_definition_versions(
        &self,
        input: ListCoreDefinitionVersionsRequest,
    ) -> RusotoFuture<ListCoreDefinitionVersionsResponse, ListCoreDefinitionVersionsError> {
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

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result =
                        serde_json::from_slice::<ListCoreDefinitionVersionsResponse>(&body)
                            .unwrap();

                    result
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(ListCoreDefinitionVersionsError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        })
    }

    /// <p>Retrieves a list of core definitions.</p>
    fn list_core_definitions(
        &self,
        input: ListCoreDefinitionsRequest,
    ) -> RusotoFuture<ListCoreDefinitionsResponse, ListCoreDefinitionsError> {
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

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result =
                        serde_json::from_slice::<ListCoreDefinitionsResponse>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(ListCoreDefinitionsError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        })
    }

    /// <p>Returns a history of deployments for the group.</p>
    fn list_deployments(
        &self,
        input: ListDeploymentsRequest,
    ) -> RusotoFuture<ListDeploymentsResponse, ListDeploymentsError> {
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

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result = serde_json::from_slice::<ListDeploymentsResponse>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(ListDeploymentsError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        })
    }

    /// <p>Lists the versions of a device definition.</p>
    fn list_device_definition_versions(
        &self,
        input: ListDeviceDefinitionVersionsRequest,
    ) -> RusotoFuture<ListDeviceDefinitionVersionsResponse, ListDeviceDefinitionVersionsError> {
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

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result =
                        serde_json::from_slice::<ListDeviceDefinitionVersionsResponse>(&body)
                            .unwrap();

                    result
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(ListDeviceDefinitionVersionsError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        })
    }

    /// <p>Retrieves a list of device definitions.</p>
    fn list_device_definitions(
        &self,
        input: ListDeviceDefinitionsRequest,
    ) -> RusotoFuture<ListDeviceDefinitionsResponse, ListDeviceDefinitionsError> {
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

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result =
                        serde_json::from_slice::<ListDeviceDefinitionsResponse>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(ListDeviceDefinitionsError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        })
    }

    /// <p>Lists the versions of a Lambda function definition.</p>
    fn list_function_definition_versions(
        &self,
        input: ListFunctionDefinitionVersionsRequest,
    ) -> RusotoFuture<ListFunctionDefinitionVersionsResponse, ListFunctionDefinitionVersionsError>
    {
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

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result =
                        serde_json::from_slice::<ListFunctionDefinitionVersionsResponse>(&body)
                            .unwrap();

                    result
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(ListFunctionDefinitionVersionsError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        })
    }

    /// <p>Retrieves a list of Lambda function definitions.</p>
    fn list_function_definitions(
        &self,
        input: ListFunctionDefinitionsRequest,
    ) -> RusotoFuture<ListFunctionDefinitionsResponse, ListFunctionDefinitionsError> {
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

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result =
                        serde_json::from_slice::<ListFunctionDefinitionsResponse>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(ListFunctionDefinitionsError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        })
    }

    /// <p>Retrieves the current CAs for a group.</p>
    fn list_group_certificate_authorities(
        &self,
        input: ListGroupCertificateAuthoritiesRequest,
    ) -> RusotoFuture<ListGroupCertificateAuthoritiesResponse, ListGroupCertificateAuthoritiesError>
    {
        let request_uri = format!(
            "/greengrass/groups/{group_id}/certificateauthorities",
            group_id = input.group_id
        );

        let mut request = SignedRequest::new("GET", "greengrass", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result =
                        serde_json::from_slice::<ListGroupCertificateAuthoritiesResponse>(&body)
                            .unwrap();

                    result
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(ListGroupCertificateAuthoritiesError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        })
    }

    /// <p>Lists the versions of a group.</p>
    fn list_group_versions(
        &self,
        input: ListGroupVersionsRequest,
    ) -> RusotoFuture<ListGroupVersionsResponse, ListGroupVersionsError> {
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

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result =
                        serde_json::from_slice::<ListGroupVersionsResponse>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(ListGroupVersionsError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        })
    }

    /// <p>Retrieves a list of groups.</p>
    fn list_groups(
        &self,
        input: ListGroupsRequest,
    ) -> RusotoFuture<ListGroupsResponse, ListGroupsError> {
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

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result = serde_json::from_slice::<ListGroupsResponse>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(ListGroupsError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        })
    }

    /// <p>Lists the versions of a logger definition.</p>
    fn list_logger_definition_versions(
        &self,
        input: ListLoggerDefinitionVersionsRequest,
    ) -> RusotoFuture<ListLoggerDefinitionVersionsResponse, ListLoggerDefinitionVersionsError> {
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

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result =
                        serde_json::from_slice::<ListLoggerDefinitionVersionsResponse>(&body)
                            .unwrap();

                    result
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(ListLoggerDefinitionVersionsError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        })
    }

    /// <p>Retrieves a list of logger definitions.</p>
    fn list_logger_definitions(
        &self,
        input: ListLoggerDefinitionsRequest,
    ) -> RusotoFuture<ListLoggerDefinitionsResponse, ListLoggerDefinitionsError> {
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

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result =
                        serde_json::from_slice::<ListLoggerDefinitionsResponse>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(ListLoggerDefinitionsError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        })
    }

    /// <p>Lists the versions of a resource definition.</p>
    fn list_resource_definition_versions(
        &self,
        input: ListResourceDefinitionVersionsRequest,
    ) -> RusotoFuture<ListResourceDefinitionVersionsResponse, ListResourceDefinitionVersionsError>
    {
        let request_uri = format!(
            "/greengrass/definition/resources/{resource_definition_id}/versions",
            resource_definition_id = input.resource_definition_id
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

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result =
                        serde_json::from_slice::<ListResourceDefinitionVersionsResponse>(&body)
                            .unwrap();

                    result
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(ListResourceDefinitionVersionsError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        })
    }

    /// <p>Retrieves a list of resource definitions.</p>
    fn list_resource_definitions(
        &self,
        input: ListResourceDefinitionsRequest,
    ) -> RusotoFuture<ListResourceDefinitionsResponse, ListResourceDefinitionsError> {
        let request_uri = "/greengrass/definition/resources";

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

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result =
                        serde_json::from_slice::<ListResourceDefinitionsResponse>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(ListResourceDefinitionsError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        })
    }

    /// <p>Lists the versions of a subscription definition.</p>
    fn list_subscription_definition_versions(
        &self,
        input: ListSubscriptionDefinitionVersionsRequest,
    ) -> RusotoFuture<
        ListSubscriptionDefinitionVersionsResponse,
        ListSubscriptionDefinitionVersionsError,
    > {
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

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result =
                        serde_json::from_slice::<ListSubscriptionDefinitionVersionsResponse>(&body)
                            .unwrap();

                    result
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(ListSubscriptionDefinitionVersionsError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        })
    }

    /// <p>Retrieves a list of subscription definitions.</p>
    fn list_subscription_definitions(
        &self,
        input: ListSubscriptionDefinitionsRequest,
    ) -> RusotoFuture<ListSubscriptionDefinitionsResponse, ListSubscriptionDefinitionsError> {
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

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result =
                        serde_json::from_slice::<ListSubscriptionDefinitionsResponse>(&body)
                            .unwrap();

                    result
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(ListSubscriptionDefinitionsError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        })
    }

    /// <p>Resets a group&#39;s deployments.</p>
    fn reset_deployments(
        &self,
        input: ResetDeploymentsRequest,
    ) -> RusotoFuture<ResetDeploymentsResponse, ResetDeploymentsError> {
        let request_uri = format!(
            "/greengrass/groups/{group_id}/deployments/$reset",
            group_id = input.group_id
        );

        let mut request = SignedRequest::new("POST", "greengrass", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        if let Some(ref amzn_client_token) = input.amzn_client_token {
            request.add_header("X-Amzn-Client-Token", &amzn_client_token.to_string());
        }

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result = serde_json::from_slice::<ResetDeploymentsResponse>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(ResetDeploymentsError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        })
    }

    /// <p>Updates the connectivity information for the core. Any devices that belong to the group which has this core will receive this information in order to find the location of the core and connect to it.</p>
    fn update_connectivity_info(
        &self,
        input: UpdateConnectivityInfoRequest,
    ) -> RusotoFuture<UpdateConnectivityInfoResponse, UpdateConnectivityInfoError> {
        let request_uri = format!(
            "/greengrass/things/{thing_name}/connectivityInfo",
            thing_name = input.thing_name
        );

        let mut request = SignedRequest::new("PUT", "greengrass", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result =
                        serde_json::from_slice::<UpdateConnectivityInfoResponse>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(UpdateConnectivityInfoError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        })
    }

    /// <p>Updates a core definition.</p>
    fn update_core_definition(
        &self,
        input: UpdateCoreDefinitionRequest,
    ) -> RusotoFuture<UpdateCoreDefinitionResponse, UpdateCoreDefinitionError> {
        let request_uri = format!(
            "/greengrass/definition/cores/{core_definition_id}",
            core_definition_id = input.core_definition_id
        );

        let mut request = SignedRequest::new("PUT", "greengrass", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result =
                        serde_json::from_slice::<UpdateCoreDefinitionResponse>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(UpdateCoreDefinitionError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        })
    }

    /// <p>Updates a device definition.</p>
    fn update_device_definition(
        &self,
        input: UpdateDeviceDefinitionRequest,
    ) -> RusotoFuture<UpdateDeviceDefinitionResponse, UpdateDeviceDefinitionError> {
        let request_uri = format!(
            "/greengrass/definition/devices/{device_definition_id}",
            device_definition_id = input.device_definition_id
        );

        let mut request = SignedRequest::new("PUT", "greengrass", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result =
                        serde_json::from_slice::<UpdateDeviceDefinitionResponse>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(UpdateDeviceDefinitionError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        })
    }

    /// <p>Updates a Lambda function definition.</p>
    fn update_function_definition(
        &self,
        input: UpdateFunctionDefinitionRequest,
    ) -> RusotoFuture<UpdateFunctionDefinitionResponse, UpdateFunctionDefinitionError> {
        let request_uri = format!(
            "/greengrass/definition/functions/{function_definition_id}",
            function_definition_id = input.function_definition_id
        );

        let mut request = SignedRequest::new("PUT", "greengrass", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result =
                        serde_json::from_slice::<UpdateFunctionDefinitionResponse>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(UpdateFunctionDefinitionError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        })
    }

    /// <p>Updates a group.</p>
    fn update_group(
        &self,
        input: UpdateGroupRequest,
    ) -> RusotoFuture<UpdateGroupResponse, UpdateGroupError> {
        let request_uri = format!("/greengrass/groups/{group_id}", group_id = input.group_id);

        let mut request = SignedRequest::new("PUT", "greengrass", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result = serde_json::from_slice::<UpdateGroupResponse>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(UpdateGroupError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        })
    }

    /// <p>Updates the Certificate expiry time for a group.</p>
    fn update_group_certificate_configuration(
        &self,
        input: UpdateGroupCertificateConfigurationRequest,
    ) -> RusotoFuture<
        UpdateGroupCertificateConfigurationResponse,
        UpdateGroupCertificateConfigurationError,
    > {
        let request_uri = format!(
            "/greengrass/groups/{group_id}/certificateauthorities/configuration/expiry",
            group_id = input.group_id
        );

        let mut request = SignedRequest::new("PUT", "greengrass", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result = serde_json::from_slice::<
                        UpdateGroupCertificateConfigurationResponse,
                    >(&body).unwrap();

                    result
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(UpdateGroupCertificateConfigurationError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        })
    }

    /// <p>Updates a logger definition.</p>
    fn update_logger_definition(
        &self,
        input: UpdateLoggerDefinitionRequest,
    ) -> RusotoFuture<UpdateLoggerDefinitionResponse, UpdateLoggerDefinitionError> {
        let request_uri = format!(
            "/greengrass/definition/loggers/{logger_definition_id}",
            logger_definition_id = input.logger_definition_id
        );

        let mut request = SignedRequest::new("PUT", "greengrass", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result =
                        serde_json::from_slice::<UpdateLoggerDefinitionResponse>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(UpdateLoggerDefinitionError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        })
    }

    /// <p>Updates a resource definition.</p>
    fn update_resource_definition(
        &self,
        input: UpdateResourceDefinitionRequest,
    ) -> RusotoFuture<UpdateResourceDefinitionResponse, UpdateResourceDefinitionError> {
        let request_uri = format!(
            "/greengrass/definition/resources/{resource_definition_id}",
            resource_definition_id = input.resource_definition_id
        );

        let mut request = SignedRequest::new("PUT", "greengrass", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result =
                        serde_json::from_slice::<UpdateResourceDefinitionResponse>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(UpdateResourceDefinitionError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        })
    }

    /// <p>Updates a subscription definition.</p>
    fn update_subscription_definition(
        &self,
        input: UpdateSubscriptionDefinitionRequest,
    ) -> RusotoFuture<UpdateSubscriptionDefinitionResponse, UpdateSubscriptionDefinitionError> {
        let request_uri = format!(
            "/greengrass/definition/subscriptions/{subscription_definition_id}",
            subscription_definition_id = input.subscription_definition_id
        );

        let mut request = SignedRequest::new("PUT", "greengrass", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result =
                        serde_json::from_slice::<UpdateSubscriptionDefinitionResponse>(&body)
                            .unwrap();

                    result
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(UpdateSubscriptionDefinitionError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        })
    }
}

#[cfg(test)]
mod protocol_tests {}
