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
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct Application {
    /// <p>The description of the application.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The application ID.</p>
    #[serde(rename = "Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// <p>The application name.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct Applications {
    /// <p>The elements from this collection.</p>
    #[serde(rename = "Items")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<Application>>,
    /// <p>The token for the next set of items to return. Use this token to get the next set of results.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct Configuration {
    /// <p>The configuration version.</p>
    pub configuration_version: Option<String>,
    /// <p>The content of the configuration or the configuration data.</p>
    pub content: Option<bytes::Bytes>,
    /// <p>A standard MIME type describing the format of the configuration content. For more information, see <a href="http://www.w3.org/Protocols/rfc2616/rfc2616-sec14.html#sec14.17">Content-Type</a>.</p>
    pub content_type: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ConfigurationProfile {
    /// <p>The application ID.</p>
    #[serde(rename = "ApplicationId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_id: Option<String>,
    /// <p>The configuration profile description.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The configuration profile ID.</p>
    #[serde(rename = "Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// <p>The URI location of the configuration.</p>
    #[serde(rename = "LocationUri")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location_uri: Option<String>,
    /// <p>The name of the configuration profile.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The ARN of an IAM role with permission to access the configuration at the specified LocationUri.</p>
    #[serde(rename = "RetrievalRoleArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retrieval_role_arn: Option<String>,
    /// <p>A list of methods for validating the configuration.</p>
    #[serde(rename = "Validators")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub validators: Option<Vec<Validator>>,
}

/// <p>A summary of a configuration profile.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ConfigurationProfileSummary {
    /// <p>The application ID.</p>
    #[serde(rename = "ApplicationId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_id: Option<String>,
    /// <p>The ID of the configuration profile.</p>
    #[serde(rename = "Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// <p>The URI location of the configuration.</p>
    #[serde(rename = "LocationUri")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location_uri: Option<String>,
    /// <p>The name of the configuration profile.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The types of validators in the configuration profile.</p>
    #[serde(rename = "ValidatorTypes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub validator_types: Option<Vec<String>>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ConfigurationProfiles {
    /// <p>The elements from this collection.</p>
    #[serde(rename = "Items")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<ConfigurationProfileSummary>>,
    /// <p>The token for the next set of items to return. Use this token to get the next set of results.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateApplicationRequest {
    /// <p>A description of the application.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>A name for the application.</p>
    #[serde(rename = "Name")]
    pub name: String,
    /// <p>Metadata to assign to the application. Tags help organize and categorize your AppConfig resources. Each tag consists of a key and an optional value, both of which you define.</p>
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<::std::collections::HashMap<String, String>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateConfigurationProfileRequest {
    /// <p>The application ID.</p>
    #[serde(rename = "ApplicationId")]
    pub application_id: String,
    /// <p>A description of the configuration profile.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>A URI to locate the configuration. You can specify either a Systems Manager (SSM) document or an SSM Parameter Store parameter. For an SSM document, specify either the document name in the format <code>ssm-document://&lt;Document name&gt;</code> or the Amazon Resource Name (ARN). For a parameter, specify either the parameter name in the format <code>ssm-parameter://&lt;Parameter name&gt;</code> or the ARN.</p>
    #[serde(rename = "LocationUri")]
    pub location_uri: String,
    /// <p>A name for the configuration profile.</p>
    #[serde(rename = "Name")]
    pub name: String,
    /// <p>The ARN of an IAM role with permission to access the configuration at the specified LocationUri.</p>
    #[serde(rename = "RetrievalRoleArn")]
    pub retrieval_role_arn: String,
    /// <p>Metadata to assign to the configuration profile. Tags help organize and categorize your AppConfig resources. Each tag consists of a key and an optional value, both of which you define.</p>
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<::std::collections::HashMap<String, String>>,
    /// <p>A list of methods for validating the configuration.</p>
    #[serde(rename = "Validators")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub validators: Option<Vec<Validator>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateDeploymentStrategyRequest {
    /// <p>Total amount of time for a deployment to last.</p>
    #[serde(rename = "DeploymentDurationInMinutes")]
    pub deployment_duration_in_minutes: i64,
    /// <p>A description of the deployment strategy.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The amount of time AppConfig monitors for alarms before considering the deployment to be complete and no longer eligible for automatic roll back.</p>
    #[serde(rename = "FinalBakeTimeInMinutes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub final_bake_time_in_minutes: Option<i64>,
    /// <p>The percentage of targets to receive a deployed configuration during each interval.</p>
    #[serde(rename = "GrowthFactor")]
    pub growth_factor: f32,
    /// <p>The algorithm used to define how percentage grows over time.</p>
    #[serde(rename = "GrowthType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub growth_type: Option<String>,
    /// <p>A name for the deployment strategy.</p>
    #[serde(rename = "Name")]
    pub name: String,
    /// <p>Save the deployment strategy to a Systems Manager (SSM) document.</p>
    #[serde(rename = "ReplicateTo")]
    pub replicate_to: String,
    /// <p>Metadata to assign to the deployment strategy. Tags help organize and categorize your AppConfig resources. Each tag consists of a key and an optional value, both of which you define.</p>
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<::std::collections::HashMap<String, String>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateEnvironmentRequest {
    /// <p>The application ID.</p>
    #[serde(rename = "ApplicationId")]
    pub application_id: String,
    /// <p>A description of the environment.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>Amazon CloudWatch alarms to monitor during the deployment process.</p>
    #[serde(rename = "Monitors")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub monitors: Option<Vec<Monitor>>,
    /// <p>A name for the environment.</p>
    #[serde(rename = "Name")]
    pub name: String,
    /// <p>Metadata to assign to the environment. Tags help organize and categorize your AppConfig resources. Each tag consists of a key and an optional value, both of which you define.</p>
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<::std::collections::HashMap<String, String>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteApplicationRequest {
    /// <p>The ID of the application to delete.</p>
    #[serde(rename = "ApplicationId")]
    pub application_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteConfigurationProfileRequest {
    /// <p>The application ID that includes the configuration profile you want to delete.</p>
    #[serde(rename = "ApplicationId")]
    pub application_id: String,
    /// <p>The ID of the configuration profile you want to delete.</p>
    #[serde(rename = "ConfigurationProfileId")]
    pub configuration_profile_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteDeploymentStrategyRequest {
    /// <p>The ID of the deployment strategy you want to delete.</p>
    #[serde(rename = "DeploymentStrategyId")]
    pub deployment_strategy_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteEnvironmentRequest {
    /// <p>The application ID that includes the environment you want to delete.</p>
    #[serde(rename = "ApplicationId")]
    pub application_id: String,
    /// <p>The ID of the environment you want to delete.</p>
    #[serde(rename = "EnvironmentId")]
    pub environment_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct Deployment {
    /// <p>The ID of the application that was deployed.</p>
    #[serde(rename = "ApplicationId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_id: Option<String>,
    /// <p>The time the deployment completed. </p>
    #[serde(rename = "CompletedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub completed_at: Option<f64>,
    /// <p>Information about the source location of the configuration.</p>
    #[serde(rename = "ConfigurationLocationUri")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configuration_location_uri: Option<String>,
    /// <p>The name of the configuration.</p>
    #[serde(rename = "ConfigurationName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configuration_name: Option<String>,
    /// <p>The ID of the configuration profile that was deployed.</p>
    #[serde(rename = "ConfigurationProfileId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configuration_profile_id: Option<String>,
    /// <p>The configuration version that was deployed.</p>
    #[serde(rename = "ConfigurationVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configuration_version: Option<String>,
    /// <p>Total amount of time the deployment lasted.</p>
    #[serde(rename = "DeploymentDurationInMinutes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deployment_duration_in_minutes: Option<i64>,
    /// <p>The sequence number of the deployment.</p>
    #[serde(rename = "DeploymentNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deployment_number: Option<i64>,
    /// <p>The ID of the deployment strategy that was deployed.</p>
    #[serde(rename = "DeploymentStrategyId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deployment_strategy_id: Option<String>,
    /// <p>The description of the deployment.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The ID of the environment that was deployed.</p>
    #[serde(rename = "EnvironmentId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub environment_id: Option<String>,
    /// <p>The amount of time AppConfig monitored for alarms before considering the deployment to be complete and no longer eligible for automatic roll back.</p>
    #[serde(rename = "FinalBakeTimeInMinutes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub final_bake_time_in_minutes: Option<i64>,
    /// <p>The percentage of targets to receive a deployed configuration during each interval.</p>
    #[serde(rename = "GrowthFactor")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub growth_factor: Option<f32>,
    /// <p>The algorithm used to define how percentage grew over time.</p>
    #[serde(rename = "GrowthType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub growth_type: Option<String>,
    /// <p>The percentage of targets for which the deployment is available.</p>
    #[serde(rename = "PercentageComplete")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub percentage_complete: Option<f32>,
    /// <p>The time the deployment started.</p>
    #[serde(rename = "StartedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub started_at: Option<f64>,
    /// <p>The state of the deployment.</p>
    #[serde(rename = "State")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DeploymentStrategies {
    /// <p>The elements from this collection.</p>
    #[serde(rename = "Items")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<DeploymentStrategy>>,
    /// <p>The token for the next set of items to return. Use this token to get the next set of results.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DeploymentStrategy {
    /// <p>Total amount of time the deployment lasted.</p>
    #[serde(rename = "DeploymentDurationInMinutes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deployment_duration_in_minutes: Option<i64>,
    /// <p>The description of the deployment strategy.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The amount of time AppConfig monitored for alarms before considering the deployment to be complete and no longer eligible for automatic roll back.</p>
    #[serde(rename = "FinalBakeTimeInMinutes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub final_bake_time_in_minutes: Option<i64>,
    /// <p>The percentage of targets that received a deployed configuration during each interval.</p>
    #[serde(rename = "GrowthFactor")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub growth_factor: Option<f32>,
    /// <p>The algorithm used to define how percentage grew over time.</p>
    #[serde(rename = "GrowthType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub growth_type: Option<String>,
    /// <p>The deployment strategy ID.</p>
    #[serde(rename = "Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// <p>The name of the deployment strategy.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>Save the deployment strategy to a Systems Manager (SSM) document.</p>
    #[serde(rename = "ReplicateTo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replicate_to: Option<String>,
}

/// <p>Information about the deployment.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DeploymentSummary {
    /// <p>Time the deployment completed.</p>
    #[serde(rename = "CompletedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub completed_at: Option<f64>,
    /// <p>The name of the configuration.</p>
    #[serde(rename = "ConfigurationName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configuration_name: Option<String>,
    /// <p>The version of the configuration.</p>
    #[serde(rename = "ConfigurationVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configuration_version: Option<String>,
    /// <p>Total amount of time the deployment lasted.</p>
    #[serde(rename = "DeploymentDurationInMinutes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deployment_duration_in_minutes: Option<i64>,
    /// <p>The sequence number of the deployment.</p>
    #[serde(rename = "DeploymentNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deployment_number: Option<i64>,
    /// <p>The amount of time AppConfig monitors for alarms before considering the deployment to be complete and no longer eligible for automatic roll back.</p>
    #[serde(rename = "FinalBakeTimeInMinutes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub final_bake_time_in_minutes: Option<i64>,
    /// <p>The percentage of targets to receive a deployed configuration during each interval.</p>
    #[serde(rename = "GrowthFactor")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub growth_factor: Option<f32>,
    /// <p>The algorithm used to define how percentage grows over time.</p>
    #[serde(rename = "GrowthType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub growth_type: Option<String>,
    /// <p>The percentage of targets for which the deployment is available.</p>
    #[serde(rename = "PercentageComplete")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub percentage_complete: Option<f32>,
    /// <p>Time the deployment started.</p>
    #[serde(rename = "StartedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub started_at: Option<f64>,
    /// <p>The state of the deployment.</p>
    #[serde(rename = "State")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct Deployments {
    /// <p>The elements from this collection.</p>
    #[serde(rename = "Items")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<DeploymentSummary>>,
    /// <p>The token for the next set of items to return. Use this token to get the next set of results.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct Environment {
    /// <p>The application ID.</p>
    #[serde(rename = "ApplicationId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_id: Option<String>,
    /// <p>The description of the environment.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The environment ID.</p>
    #[serde(rename = "Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// <p>Amazon CloudWatch alarms monitored during the deployment.</p>
    #[serde(rename = "Monitors")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub monitors: Option<Vec<Monitor>>,
    /// <p>The name of the environment.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The state of the environment. An environment can be in one of the following states: <code>READY_FOR_DEPLOYMENT</code>, <code>DEPLOYING</code>, <code>ROLLING_BACK</code>, or <code>ROLLED_BACK</code> </p>
    #[serde(rename = "State")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct Environments {
    /// <p>The elements from this collection.</p>
    #[serde(rename = "Items")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<Environment>>,
    /// <p>The token for the next set of items to return. Use this token to get the next set of results.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetApplicationRequest {
    /// <p>The ID of the application you want to get.</p>
    #[serde(rename = "ApplicationId")]
    pub application_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetConfigurationProfileRequest {
    /// <p>The ID of the application that includes the configuration profile you want to get.</p>
    #[serde(rename = "ApplicationId")]
    pub application_id: String,
    /// <p>The ID of the configuration profile you want to get.</p>
    #[serde(rename = "ConfigurationProfileId")]
    pub configuration_profile_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetConfigurationRequest {
    /// <p>The application to get.</p>
    #[serde(rename = "Application")]
    pub application: String,
    /// <p>The configuration version returned in the most recent GetConfiguration response.</p>
    #[serde(rename = "ClientConfigurationVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_configuration_version: Option<String>,
    /// <p>A unique ID to identify the client for the configuration. This ID enables AppConfig to deploy the configuration in intervals, as defined in the deployment strategy.</p>
    #[serde(rename = "ClientId")]
    pub client_id: String,
    /// <p>The configuration to get.</p>
    #[serde(rename = "Configuration")]
    pub configuration: String,
    /// <p>The environment to get.</p>
    #[serde(rename = "Environment")]
    pub environment: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetDeploymentRequest {
    /// <p>The ID of the application that includes the deployment you want to get. </p>
    #[serde(rename = "ApplicationId")]
    pub application_id: String,
    /// <p>The sequence number of the deployment.</p>
    #[serde(rename = "DeploymentNumber")]
    pub deployment_number: i64,
    /// <p>The ID of the environment that includes the deployment you want to get. </p>
    #[serde(rename = "EnvironmentId")]
    pub environment_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetDeploymentStrategyRequest {
    /// <p>The ID of the deployment strategy to get.</p>
    #[serde(rename = "DeploymentStrategyId")]
    pub deployment_strategy_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetEnvironmentRequest {
    /// <p>The ID of the application that includes the environment you want to get.</p>
    #[serde(rename = "ApplicationId")]
    pub application_id: String,
    /// <p>The ID of the environment you wnat to get.</p>
    #[serde(rename = "EnvironmentId")]
    pub environment_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListApplicationsRequest {
    /// <p>The maximum number of items to return for this call. The call also returns a token that you can specify in a subsequent call to get the next set of results.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>A token to start the list. Use this token to get the next set of results.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListConfigurationProfilesRequest {
    /// <p>The application ID.</p>
    #[serde(rename = "ApplicationId")]
    pub application_id: String,
    /// <p>The maximum number of items to return for this call. The call also returns a token that you can specify in a subsequent call to get the next set of results.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>A token to start the list. Use this token to get the next set of results.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListDeploymentStrategiesRequest {
    /// <p>The maximum number of items to return for this call. The call also returns a token that you can specify in a subsequent call to get the next set of results.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>A token to start the list. Use this token to get the next set of results.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListDeploymentsRequest {
    /// <p>The application ID.</p>
    #[serde(rename = "ApplicationId")]
    pub application_id: String,
    /// <p>The environment ID.</p>
    #[serde(rename = "EnvironmentId")]
    pub environment_id: String,
    /// <p>The maximum number of items to return for this call. The call also returns a token that you can specify in a subsequent call to get the next set of results.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>A token to start the list. Use this token to get the next set of results.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListEnvironmentsRequest {
    /// <p>The application ID.</p>
    #[serde(rename = "ApplicationId")]
    pub application_id: String,
    /// <p>The maximum number of items to return for this call. The call also returns a token that you can specify in a subsequent call to get the next set of results.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>A token to start the list. Use this token to get the next set of results.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListTagsForResourceRequest {
    /// <p>The resource ARN.</p>
    #[serde(rename = "ResourceArn")]
    pub resource_arn: String,
}

/// <p>Amazon CloudWatch alarms to monitor during the deployment process.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Monitor {
    /// <p>ARN of the Amazon CloudWatch alarm.</p>
    #[serde(rename = "AlarmArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alarm_arn: Option<String>,
    /// <p>ARN of an IAM role for AppConfig to monitor <code>AlarmArn</code>.</p>
    #[serde(rename = "AlarmRoleArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alarm_role_arn: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ResourceTags {
    /// <p>Metadata to assign to AppConfig resources. Tags help organize and categorize your AppConfig resources. Each tag consists of a key and an optional value, both of which you define.</p>
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<::std::collections::HashMap<String, String>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct StartDeploymentRequest {
    /// <p>The application ID.</p>
    #[serde(rename = "ApplicationId")]
    pub application_id: String,
    /// <p>The configuration profile ID.</p>
    #[serde(rename = "ConfigurationProfileId")]
    pub configuration_profile_id: String,
    /// <p>The configuration version to deploy.</p>
    #[serde(rename = "ConfigurationVersion")]
    pub configuration_version: String,
    /// <p>The deployment strategy ID.</p>
    #[serde(rename = "DeploymentStrategyId")]
    pub deployment_strategy_id: String,
    /// <p>A description of the deployment.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The environment ID.</p>
    #[serde(rename = "EnvironmentId")]
    pub environment_id: String,
    /// <p>Metadata to assign to the deployment. Tags help organize and categorize your AppConfig resources. Each tag consists of a key and an optional value, both of which you define.</p>
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<::std::collections::HashMap<String, String>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct StopDeploymentRequest {
    /// <p>The application ID.</p>
    #[serde(rename = "ApplicationId")]
    pub application_id: String,
    /// <p>The sequence number of the deployment.</p>
    #[serde(rename = "DeploymentNumber")]
    pub deployment_number: i64,
    /// <p>The environment ID.</p>
    #[serde(rename = "EnvironmentId")]
    pub environment_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct TagResourceRequest {
    /// <p>The ARN of the resource for which to retrieve tags.</p>
    #[serde(rename = "ResourceArn")]
    pub resource_arn: String,
    /// <p>The key-value string map. The valid character set is [a-zA-Z+-=._:/]. The tag key can be up to 128 characters and must not start with <code>aws:</code>. The tag value can be up to 256 characters.</p>
    #[serde(rename = "Tags")]
    pub tags: ::std::collections::HashMap<String, String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UntagResourceRequest {
    /// <p>The ARN of the resource for which to remove tags.</p>
    #[serde(rename = "ResourceArn")]
    pub resource_arn: String,
    /// <p>The tag keys to delete.</p>
    #[serde(rename = "TagKeys")]
    pub tag_keys: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UpdateApplicationRequest {
    /// <p>The application ID.</p>
    #[serde(rename = "ApplicationId")]
    pub application_id: String,
    /// <p>A description of the application.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The name of the application.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UpdateConfigurationProfileRequest {
    /// <p>The application ID.</p>
    #[serde(rename = "ApplicationId")]
    pub application_id: String,
    /// <p>The ID of the configuration profile.</p>
    #[serde(rename = "ConfigurationProfileId")]
    pub configuration_profile_id: String,
    /// <p>A description of the configuration profile.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The name of the configuration profile.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The ARN of an IAM role with permission to access the configuration at the specified LocationUri.</p>
    #[serde(rename = "RetrievalRoleArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retrieval_role_arn: Option<String>,
    /// <p>A list of methods for validating the configuration.</p>
    #[serde(rename = "Validators")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub validators: Option<Vec<Validator>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UpdateDeploymentStrategyRequest {
    /// <p>Total amount of time for a deployment to last.</p>
    #[serde(rename = "DeploymentDurationInMinutes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deployment_duration_in_minutes: Option<i64>,
    /// <p>The deployment strategy ID.</p>
    #[serde(rename = "DeploymentStrategyId")]
    pub deployment_strategy_id: String,
    /// <p>A description of the deployment strategy.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The amount of time AppConfig monitors for alarms before considering the deployment to be complete and no longer eligible for automatic roll back.</p>
    #[serde(rename = "FinalBakeTimeInMinutes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub final_bake_time_in_minutes: Option<i64>,
    /// <p>The percentage of targets to receive a deployed configuration during each interval.</p>
    #[serde(rename = "GrowthFactor")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub growth_factor: Option<f32>,
    /// <p>The algorithm used to define how percentage grows over time.</p>
    #[serde(rename = "GrowthType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub growth_type: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UpdateEnvironmentRequest {
    /// <p>The application ID.</p>
    #[serde(rename = "ApplicationId")]
    pub application_id: String,
    /// <p>A description of the environment.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The environment ID.</p>
    #[serde(rename = "EnvironmentId")]
    pub environment_id: String,
    /// <p>Amazon CloudWatch alarms to monitor during the deployment process.</p>
    #[serde(rename = "Monitors")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub monitors: Option<Vec<Monitor>>,
    /// <p>The name of the environment.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ValidateConfigurationRequest {
    /// <p>The application ID.</p>
    #[serde(rename = "ApplicationId")]
    pub application_id: String,
    /// <p>The configuration profile ID.</p>
    #[serde(rename = "ConfigurationProfileId")]
    pub configuration_profile_id: String,
    /// <p>The version of the configuration to validate.</p>
    #[serde(rename = "ConfigurationVersion")]
    pub configuration_version: String,
}

/// <p>A validator provides a syntactic or semantic check to ensure the configuration you want to deploy functions as intended. To validate your application configuration data, you provide a schema or a Lambda function that runs against the configuration. The configuration deployment or update can only proceed when the configuration data is valid.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Validator {
    /// <p>Either the JSON Schema content or an AWS Lambda function name.</p>
    #[serde(rename = "Content")]
    pub content: String,
    /// <p>AppConfig supports validators of type <code>JSON_SCHEMA</code> and <code>LAMBDA</code> </p>
    #[serde(rename = "Type")]
    pub type_: String,
}

/// Errors returned by CreateApplication
#[derive(Debug, PartialEq)]
pub enum CreateApplicationError {
    /// <p>The input fails to satisfy the constraints specified by an AWS service.</p>
    BadRequest(String),
    /// <p>There was an internal failure in the AppConfig service.</p>
    InternalServer(String),
}

impl CreateApplicationError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateApplicationError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(CreateApplicationError::BadRequest(err.msg))
                }
                "InternalServerException" => {
                    return RusotoError::Service(CreateApplicationError::InternalServer(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for CreateApplicationError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreateApplicationError::BadRequest(ref cause) => write!(f, "{}", cause),
            CreateApplicationError::InternalServer(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CreateApplicationError {}
/// Errors returned by CreateConfigurationProfile
#[derive(Debug, PartialEq)]
pub enum CreateConfigurationProfileError {
    /// <p>The input fails to satisfy the constraints specified by an AWS service.</p>
    BadRequest(String),
    /// <p>There was an internal failure in the AppConfig service.</p>
    InternalServer(String),
    /// <p>The requested resource could not be found.</p>
    ResourceNotFound(String),
}

impl CreateConfigurationProfileError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<CreateConfigurationProfileError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(CreateConfigurationProfileError::BadRequest(
                        err.msg,
                    ))
                }
                "InternalServerException" => {
                    return RusotoError::Service(CreateConfigurationProfileError::InternalServer(
                        err.msg,
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(CreateConfigurationProfileError::ResourceNotFound(
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
impl fmt::Display for CreateConfigurationProfileError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreateConfigurationProfileError::BadRequest(ref cause) => write!(f, "{}", cause),
            CreateConfigurationProfileError::InternalServer(ref cause) => write!(f, "{}", cause),
            CreateConfigurationProfileError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CreateConfigurationProfileError {}
/// Errors returned by CreateDeploymentStrategy
#[derive(Debug, PartialEq)]
pub enum CreateDeploymentStrategyError {
    /// <p>The input fails to satisfy the constraints specified by an AWS service.</p>
    BadRequest(String),
    /// <p>There was an internal failure in the AppConfig service.</p>
    InternalServer(String),
}

impl CreateDeploymentStrategyError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateDeploymentStrategyError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(CreateDeploymentStrategyError::BadRequest(err.msg))
                }
                "InternalServerException" => {
                    return RusotoError::Service(CreateDeploymentStrategyError::InternalServer(
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
impl fmt::Display for CreateDeploymentStrategyError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreateDeploymentStrategyError::BadRequest(ref cause) => write!(f, "{}", cause),
            CreateDeploymentStrategyError::InternalServer(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CreateDeploymentStrategyError {}
/// Errors returned by CreateEnvironment
#[derive(Debug, PartialEq)]
pub enum CreateEnvironmentError {
    /// <p>The input fails to satisfy the constraints specified by an AWS service.</p>
    BadRequest(String),
    /// <p>There was an internal failure in the AppConfig service.</p>
    InternalServer(String),
    /// <p>The requested resource could not be found.</p>
    ResourceNotFound(String),
}

impl CreateEnvironmentError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateEnvironmentError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(CreateEnvironmentError::BadRequest(err.msg))
                }
                "InternalServerException" => {
                    return RusotoError::Service(CreateEnvironmentError::InternalServer(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(CreateEnvironmentError::ResourceNotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for CreateEnvironmentError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreateEnvironmentError::BadRequest(ref cause) => write!(f, "{}", cause),
            CreateEnvironmentError::InternalServer(ref cause) => write!(f, "{}", cause),
            CreateEnvironmentError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CreateEnvironmentError {}
/// Errors returned by DeleteApplication
#[derive(Debug, PartialEq)]
pub enum DeleteApplicationError {
    /// <p>The input fails to satisfy the constraints specified by an AWS service.</p>
    BadRequest(String),
    /// <p>There was an internal failure in the AppConfig service.</p>
    InternalServer(String),
    /// <p>The requested resource could not be found.</p>
    ResourceNotFound(String),
}

impl DeleteApplicationError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteApplicationError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(DeleteApplicationError::BadRequest(err.msg))
                }
                "InternalServerException" => {
                    return RusotoError::Service(DeleteApplicationError::InternalServer(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(DeleteApplicationError::ResourceNotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DeleteApplicationError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteApplicationError::BadRequest(ref cause) => write!(f, "{}", cause),
            DeleteApplicationError::InternalServer(ref cause) => write!(f, "{}", cause),
            DeleteApplicationError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeleteApplicationError {}
/// Errors returned by DeleteConfigurationProfile
#[derive(Debug, PartialEq)]
pub enum DeleteConfigurationProfileError {
    /// <p>The input fails to satisfy the constraints specified by an AWS service.</p>
    BadRequest(String),
    /// <p>The request could not be processed because of conflict in the current state of the resource.</p>
    Conflict(String),
    /// <p>There was an internal failure in the AppConfig service.</p>
    InternalServer(String),
    /// <p>The requested resource could not be found.</p>
    ResourceNotFound(String),
}

impl DeleteConfigurationProfileError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<DeleteConfigurationProfileError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(DeleteConfigurationProfileError::BadRequest(
                        err.msg,
                    ))
                }
                "ConflictException" => {
                    return RusotoError::Service(DeleteConfigurationProfileError::Conflict(err.msg))
                }
                "InternalServerException" => {
                    return RusotoError::Service(DeleteConfigurationProfileError::InternalServer(
                        err.msg,
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(DeleteConfigurationProfileError::ResourceNotFound(
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
impl fmt::Display for DeleteConfigurationProfileError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteConfigurationProfileError::BadRequest(ref cause) => write!(f, "{}", cause),
            DeleteConfigurationProfileError::Conflict(ref cause) => write!(f, "{}", cause),
            DeleteConfigurationProfileError::InternalServer(ref cause) => write!(f, "{}", cause),
            DeleteConfigurationProfileError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeleteConfigurationProfileError {}
/// Errors returned by DeleteDeploymentStrategy
#[derive(Debug, PartialEq)]
pub enum DeleteDeploymentStrategyError {
    /// <p>The input fails to satisfy the constraints specified by an AWS service.</p>
    BadRequest(String),
    /// <p>There was an internal failure in the AppConfig service.</p>
    InternalServer(String),
    /// <p>The requested resource could not be found.</p>
    ResourceNotFound(String),
}

impl DeleteDeploymentStrategyError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteDeploymentStrategyError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(DeleteDeploymentStrategyError::BadRequest(err.msg))
                }
                "InternalServerException" => {
                    return RusotoError::Service(DeleteDeploymentStrategyError::InternalServer(
                        err.msg,
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(DeleteDeploymentStrategyError::ResourceNotFound(
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
impl fmt::Display for DeleteDeploymentStrategyError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteDeploymentStrategyError::BadRequest(ref cause) => write!(f, "{}", cause),
            DeleteDeploymentStrategyError::InternalServer(ref cause) => write!(f, "{}", cause),
            DeleteDeploymentStrategyError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeleteDeploymentStrategyError {}
/// Errors returned by DeleteEnvironment
#[derive(Debug, PartialEq)]
pub enum DeleteEnvironmentError {
    /// <p>The input fails to satisfy the constraints specified by an AWS service.</p>
    BadRequest(String),
    /// <p>The request could not be processed because of conflict in the current state of the resource.</p>
    Conflict(String),
    /// <p>There was an internal failure in the AppConfig service.</p>
    InternalServer(String),
    /// <p>The requested resource could not be found.</p>
    ResourceNotFound(String),
}

impl DeleteEnvironmentError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteEnvironmentError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(DeleteEnvironmentError::BadRequest(err.msg))
                }
                "ConflictException" => {
                    return RusotoError::Service(DeleteEnvironmentError::Conflict(err.msg))
                }
                "InternalServerException" => {
                    return RusotoError::Service(DeleteEnvironmentError::InternalServer(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(DeleteEnvironmentError::ResourceNotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DeleteEnvironmentError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteEnvironmentError::BadRequest(ref cause) => write!(f, "{}", cause),
            DeleteEnvironmentError::Conflict(ref cause) => write!(f, "{}", cause),
            DeleteEnvironmentError::InternalServer(ref cause) => write!(f, "{}", cause),
            DeleteEnvironmentError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeleteEnvironmentError {}
/// Errors returned by GetApplication
#[derive(Debug, PartialEq)]
pub enum GetApplicationError {
    /// <p>The input fails to satisfy the constraints specified by an AWS service.</p>
    BadRequest(String),
    /// <p>There was an internal failure in the AppConfig service.</p>
    InternalServer(String),
    /// <p>The requested resource could not be found.</p>
    ResourceNotFound(String),
}

impl GetApplicationError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetApplicationError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(GetApplicationError::BadRequest(err.msg))
                }
                "InternalServerException" => {
                    return RusotoError::Service(GetApplicationError::InternalServer(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(GetApplicationError::ResourceNotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for GetApplicationError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetApplicationError::BadRequest(ref cause) => write!(f, "{}", cause),
            GetApplicationError::InternalServer(ref cause) => write!(f, "{}", cause),
            GetApplicationError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetApplicationError {}
/// Errors returned by GetConfiguration
#[derive(Debug, PartialEq)]
pub enum GetConfigurationError {
    /// <p>The input fails to satisfy the constraints specified by an AWS service.</p>
    BadRequest(String),
    /// <p>There was an internal failure in the AppConfig service.</p>
    InternalServer(String),
    /// <p>The requested resource could not be found.</p>
    ResourceNotFound(String),
}

impl GetConfigurationError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetConfigurationError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(GetConfigurationError::BadRequest(err.msg))
                }
                "InternalServerException" => {
                    return RusotoError::Service(GetConfigurationError::InternalServer(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(GetConfigurationError::ResourceNotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for GetConfigurationError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetConfigurationError::BadRequest(ref cause) => write!(f, "{}", cause),
            GetConfigurationError::InternalServer(ref cause) => write!(f, "{}", cause),
            GetConfigurationError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetConfigurationError {}
/// Errors returned by GetConfigurationProfile
#[derive(Debug, PartialEq)]
pub enum GetConfigurationProfileError {
    /// <p>The input fails to satisfy the constraints specified by an AWS service.</p>
    BadRequest(String),
    /// <p>There was an internal failure in the AppConfig service.</p>
    InternalServer(String),
    /// <p>The requested resource could not be found.</p>
    ResourceNotFound(String),
}

impl GetConfigurationProfileError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetConfigurationProfileError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(GetConfigurationProfileError::BadRequest(err.msg))
                }
                "InternalServerException" => {
                    return RusotoError::Service(GetConfigurationProfileError::InternalServer(
                        err.msg,
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(GetConfigurationProfileError::ResourceNotFound(
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
impl fmt::Display for GetConfigurationProfileError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetConfigurationProfileError::BadRequest(ref cause) => write!(f, "{}", cause),
            GetConfigurationProfileError::InternalServer(ref cause) => write!(f, "{}", cause),
            GetConfigurationProfileError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetConfigurationProfileError {}
/// Errors returned by GetDeployment
#[derive(Debug, PartialEq)]
pub enum GetDeploymentError {
    /// <p>The input fails to satisfy the constraints specified by an AWS service.</p>
    BadRequest(String),
    /// <p>There was an internal failure in the AppConfig service.</p>
    InternalServer(String),
    /// <p>The requested resource could not be found.</p>
    ResourceNotFound(String),
}

impl GetDeploymentError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetDeploymentError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(GetDeploymentError::BadRequest(err.msg))
                }
                "InternalServerException" => {
                    return RusotoError::Service(GetDeploymentError::InternalServer(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(GetDeploymentError::ResourceNotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for GetDeploymentError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetDeploymentError::BadRequest(ref cause) => write!(f, "{}", cause),
            GetDeploymentError::InternalServer(ref cause) => write!(f, "{}", cause),
            GetDeploymentError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetDeploymentError {}
/// Errors returned by GetDeploymentStrategy
#[derive(Debug, PartialEq)]
pub enum GetDeploymentStrategyError {
    /// <p>The input fails to satisfy the constraints specified by an AWS service.</p>
    BadRequest(String),
    /// <p>There was an internal failure in the AppConfig service.</p>
    InternalServer(String),
    /// <p>The requested resource could not be found.</p>
    ResourceNotFound(String),
}

impl GetDeploymentStrategyError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetDeploymentStrategyError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(GetDeploymentStrategyError::BadRequest(err.msg))
                }
                "InternalServerException" => {
                    return RusotoError::Service(GetDeploymentStrategyError::InternalServer(
                        err.msg,
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(GetDeploymentStrategyError::ResourceNotFound(
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
impl fmt::Display for GetDeploymentStrategyError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetDeploymentStrategyError::BadRequest(ref cause) => write!(f, "{}", cause),
            GetDeploymentStrategyError::InternalServer(ref cause) => write!(f, "{}", cause),
            GetDeploymentStrategyError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetDeploymentStrategyError {}
/// Errors returned by GetEnvironment
#[derive(Debug, PartialEq)]
pub enum GetEnvironmentError {
    /// <p>The input fails to satisfy the constraints specified by an AWS service.</p>
    BadRequest(String),
    /// <p>There was an internal failure in the AppConfig service.</p>
    InternalServer(String),
    /// <p>The requested resource could not be found.</p>
    ResourceNotFound(String),
}

impl GetEnvironmentError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetEnvironmentError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(GetEnvironmentError::BadRequest(err.msg))
                }
                "InternalServerException" => {
                    return RusotoError::Service(GetEnvironmentError::InternalServer(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(GetEnvironmentError::ResourceNotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for GetEnvironmentError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetEnvironmentError::BadRequest(ref cause) => write!(f, "{}", cause),
            GetEnvironmentError::InternalServer(ref cause) => write!(f, "{}", cause),
            GetEnvironmentError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetEnvironmentError {}
/// Errors returned by ListApplications
#[derive(Debug, PartialEq)]
pub enum ListApplicationsError {
    /// <p>The input fails to satisfy the constraints specified by an AWS service.</p>
    BadRequest(String),
    /// <p>There was an internal failure in the AppConfig service.</p>
    InternalServer(String),
}

impl ListApplicationsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListApplicationsError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(ListApplicationsError::BadRequest(err.msg))
                }
                "InternalServerException" => {
                    return RusotoError::Service(ListApplicationsError::InternalServer(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ListApplicationsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListApplicationsError::BadRequest(ref cause) => write!(f, "{}", cause),
            ListApplicationsError::InternalServer(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListApplicationsError {}
/// Errors returned by ListConfigurationProfiles
#[derive(Debug, PartialEq)]
pub enum ListConfigurationProfilesError {
    /// <p>The input fails to satisfy the constraints specified by an AWS service.</p>
    BadRequest(String),
    /// <p>There was an internal failure in the AppConfig service.</p>
    InternalServer(String),
    /// <p>The requested resource could not be found.</p>
    ResourceNotFound(String),
}

impl ListConfigurationProfilesError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListConfigurationProfilesError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(ListConfigurationProfilesError::BadRequest(
                        err.msg,
                    ))
                }
                "InternalServerException" => {
                    return RusotoError::Service(ListConfigurationProfilesError::InternalServer(
                        err.msg,
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(ListConfigurationProfilesError::ResourceNotFound(
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
impl fmt::Display for ListConfigurationProfilesError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListConfigurationProfilesError::BadRequest(ref cause) => write!(f, "{}", cause),
            ListConfigurationProfilesError::InternalServer(ref cause) => write!(f, "{}", cause),
            ListConfigurationProfilesError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListConfigurationProfilesError {}
/// Errors returned by ListDeploymentStrategies
#[derive(Debug, PartialEq)]
pub enum ListDeploymentStrategiesError {
    /// <p>The input fails to satisfy the constraints specified by an AWS service.</p>
    BadRequest(String),
    /// <p>There was an internal failure in the AppConfig service.</p>
    InternalServer(String),
}

impl ListDeploymentStrategiesError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListDeploymentStrategiesError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(ListDeploymentStrategiesError::BadRequest(err.msg))
                }
                "InternalServerException" => {
                    return RusotoError::Service(ListDeploymentStrategiesError::InternalServer(
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
impl fmt::Display for ListDeploymentStrategiesError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListDeploymentStrategiesError::BadRequest(ref cause) => write!(f, "{}", cause),
            ListDeploymentStrategiesError::InternalServer(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListDeploymentStrategiesError {}
/// Errors returned by ListDeployments
#[derive(Debug, PartialEq)]
pub enum ListDeploymentsError {
    /// <p>The input fails to satisfy the constraints specified by an AWS service.</p>
    BadRequest(String),
    /// <p>There was an internal failure in the AppConfig service.</p>
    InternalServer(String),
    /// <p>The requested resource could not be found.</p>
    ResourceNotFound(String),
}

impl ListDeploymentsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListDeploymentsError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(ListDeploymentsError::BadRequest(err.msg))
                }
                "InternalServerException" => {
                    return RusotoError::Service(ListDeploymentsError::InternalServer(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(ListDeploymentsError::ResourceNotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ListDeploymentsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListDeploymentsError::BadRequest(ref cause) => write!(f, "{}", cause),
            ListDeploymentsError::InternalServer(ref cause) => write!(f, "{}", cause),
            ListDeploymentsError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListDeploymentsError {}
/// Errors returned by ListEnvironments
#[derive(Debug, PartialEq)]
pub enum ListEnvironmentsError {
    /// <p>The input fails to satisfy the constraints specified by an AWS service.</p>
    BadRequest(String),
    /// <p>There was an internal failure in the AppConfig service.</p>
    InternalServer(String),
    /// <p>The requested resource could not be found.</p>
    ResourceNotFound(String),
}

impl ListEnvironmentsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListEnvironmentsError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(ListEnvironmentsError::BadRequest(err.msg))
                }
                "InternalServerException" => {
                    return RusotoError::Service(ListEnvironmentsError::InternalServer(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(ListEnvironmentsError::ResourceNotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ListEnvironmentsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListEnvironmentsError::BadRequest(ref cause) => write!(f, "{}", cause),
            ListEnvironmentsError::InternalServer(ref cause) => write!(f, "{}", cause),
            ListEnvironmentsError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListEnvironmentsError {}
/// Errors returned by ListTagsForResource
#[derive(Debug, PartialEq)]
pub enum ListTagsForResourceError {
    /// <p>The input fails to satisfy the constraints specified by an AWS service.</p>
    BadRequest(String),
    /// <p>There was an internal failure in the AppConfig service.</p>
    InternalServer(String),
    /// <p>The requested resource could not be found.</p>
    ResourceNotFound(String),
}

impl ListTagsForResourceError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListTagsForResourceError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(ListTagsForResourceError::BadRequest(err.msg))
                }
                "InternalServerException" => {
                    return RusotoError::Service(ListTagsForResourceError::InternalServer(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(ListTagsForResourceError::ResourceNotFound(
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
impl fmt::Display for ListTagsForResourceError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListTagsForResourceError::BadRequest(ref cause) => write!(f, "{}", cause),
            ListTagsForResourceError::InternalServer(ref cause) => write!(f, "{}", cause),
            ListTagsForResourceError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListTagsForResourceError {}
/// Errors returned by StartDeployment
#[derive(Debug, PartialEq)]
pub enum StartDeploymentError {
    /// <p>The input fails to satisfy the constraints specified by an AWS service.</p>
    BadRequest(String),
    /// <p>The request could not be processed because of conflict in the current state of the resource.</p>
    Conflict(String),
    /// <p>There was an internal failure in the AppConfig service.</p>
    InternalServer(String),
    /// <p>The requested resource could not be found.</p>
    ResourceNotFound(String),
}

impl StartDeploymentError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<StartDeploymentError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(StartDeploymentError::BadRequest(err.msg))
                }
                "ConflictException" => {
                    return RusotoError::Service(StartDeploymentError::Conflict(err.msg))
                }
                "InternalServerException" => {
                    return RusotoError::Service(StartDeploymentError::InternalServer(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(StartDeploymentError::ResourceNotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for StartDeploymentError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            StartDeploymentError::BadRequest(ref cause) => write!(f, "{}", cause),
            StartDeploymentError::Conflict(ref cause) => write!(f, "{}", cause),
            StartDeploymentError::InternalServer(ref cause) => write!(f, "{}", cause),
            StartDeploymentError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for StartDeploymentError {}
/// Errors returned by StopDeployment
#[derive(Debug, PartialEq)]
pub enum StopDeploymentError {
    /// <p>The input fails to satisfy the constraints specified by an AWS service.</p>
    BadRequest(String),
    /// <p>There was an internal failure in the AppConfig service.</p>
    InternalServer(String),
    /// <p>The requested resource could not be found.</p>
    ResourceNotFound(String),
}

impl StopDeploymentError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<StopDeploymentError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(StopDeploymentError::BadRequest(err.msg))
                }
                "InternalServerException" => {
                    return RusotoError::Service(StopDeploymentError::InternalServer(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(StopDeploymentError::ResourceNotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for StopDeploymentError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            StopDeploymentError::BadRequest(ref cause) => write!(f, "{}", cause),
            StopDeploymentError::InternalServer(ref cause) => write!(f, "{}", cause),
            StopDeploymentError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for StopDeploymentError {}
/// Errors returned by TagResource
#[derive(Debug, PartialEq)]
pub enum TagResourceError {
    /// <p>The input fails to satisfy the constraints specified by an AWS service.</p>
    BadRequest(String),
    /// <p>There was an internal failure in the AppConfig service.</p>
    InternalServer(String),
    /// <p>The requested resource could not be found.</p>
    ResourceNotFound(String),
}

impl TagResourceError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<TagResourceError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(TagResourceError::BadRequest(err.msg))
                }
                "InternalServerException" => {
                    return RusotoError::Service(TagResourceError::InternalServer(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(TagResourceError::ResourceNotFound(err.msg))
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
            TagResourceError::InternalServer(ref cause) => write!(f, "{}", cause),
            TagResourceError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for TagResourceError {}
/// Errors returned by UntagResource
#[derive(Debug, PartialEq)]
pub enum UntagResourceError {
    /// <p>The input fails to satisfy the constraints specified by an AWS service.</p>
    BadRequest(String),
    /// <p>There was an internal failure in the AppConfig service.</p>
    InternalServer(String),
    /// <p>The requested resource could not be found.</p>
    ResourceNotFound(String),
}

impl UntagResourceError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UntagResourceError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(UntagResourceError::BadRequest(err.msg))
                }
                "InternalServerException" => {
                    return RusotoError::Service(UntagResourceError::InternalServer(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(UntagResourceError::ResourceNotFound(err.msg))
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
            UntagResourceError::InternalServer(ref cause) => write!(f, "{}", cause),
            UntagResourceError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for UntagResourceError {}
/// Errors returned by UpdateApplication
#[derive(Debug, PartialEq)]
pub enum UpdateApplicationError {
    /// <p>The input fails to satisfy the constraints specified by an AWS service.</p>
    BadRequest(String),
    /// <p>There was an internal failure in the AppConfig service.</p>
    InternalServer(String),
    /// <p>The requested resource could not be found.</p>
    ResourceNotFound(String),
}

impl UpdateApplicationError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdateApplicationError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(UpdateApplicationError::BadRequest(err.msg))
                }
                "InternalServerException" => {
                    return RusotoError::Service(UpdateApplicationError::InternalServer(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(UpdateApplicationError::ResourceNotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for UpdateApplicationError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UpdateApplicationError::BadRequest(ref cause) => write!(f, "{}", cause),
            UpdateApplicationError::InternalServer(ref cause) => write!(f, "{}", cause),
            UpdateApplicationError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for UpdateApplicationError {}
/// Errors returned by UpdateConfigurationProfile
#[derive(Debug, PartialEq)]
pub enum UpdateConfigurationProfileError {
    /// <p>The input fails to satisfy the constraints specified by an AWS service.</p>
    BadRequest(String),
    /// <p>There was an internal failure in the AppConfig service.</p>
    InternalServer(String),
    /// <p>The requested resource could not be found.</p>
    ResourceNotFound(String),
}

impl UpdateConfigurationProfileError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<UpdateConfigurationProfileError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(UpdateConfigurationProfileError::BadRequest(
                        err.msg,
                    ))
                }
                "InternalServerException" => {
                    return RusotoError::Service(UpdateConfigurationProfileError::InternalServer(
                        err.msg,
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(UpdateConfigurationProfileError::ResourceNotFound(
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
impl fmt::Display for UpdateConfigurationProfileError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UpdateConfigurationProfileError::BadRequest(ref cause) => write!(f, "{}", cause),
            UpdateConfigurationProfileError::InternalServer(ref cause) => write!(f, "{}", cause),
            UpdateConfigurationProfileError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for UpdateConfigurationProfileError {}
/// Errors returned by UpdateDeploymentStrategy
#[derive(Debug, PartialEq)]
pub enum UpdateDeploymentStrategyError {
    /// <p>The input fails to satisfy the constraints specified by an AWS service.</p>
    BadRequest(String),
    /// <p>There was an internal failure in the AppConfig service.</p>
    InternalServer(String),
    /// <p>The requested resource could not be found.</p>
    ResourceNotFound(String),
}

impl UpdateDeploymentStrategyError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdateDeploymentStrategyError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(UpdateDeploymentStrategyError::BadRequest(err.msg))
                }
                "InternalServerException" => {
                    return RusotoError::Service(UpdateDeploymentStrategyError::InternalServer(
                        err.msg,
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(UpdateDeploymentStrategyError::ResourceNotFound(
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
impl fmt::Display for UpdateDeploymentStrategyError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UpdateDeploymentStrategyError::BadRequest(ref cause) => write!(f, "{}", cause),
            UpdateDeploymentStrategyError::InternalServer(ref cause) => write!(f, "{}", cause),
            UpdateDeploymentStrategyError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for UpdateDeploymentStrategyError {}
/// Errors returned by UpdateEnvironment
#[derive(Debug, PartialEq)]
pub enum UpdateEnvironmentError {
    /// <p>The input fails to satisfy the constraints specified by an AWS service.</p>
    BadRequest(String),
    /// <p>There was an internal failure in the AppConfig service.</p>
    InternalServer(String),
    /// <p>The requested resource could not be found.</p>
    ResourceNotFound(String),
}

impl UpdateEnvironmentError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdateEnvironmentError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(UpdateEnvironmentError::BadRequest(err.msg))
                }
                "InternalServerException" => {
                    return RusotoError::Service(UpdateEnvironmentError::InternalServer(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(UpdateEnvironmentError::ResourceNotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for UpdateEnvironmentError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UpdateEnvironmentError::BadRequest(ref cause) => write!(f, "{}", cause),
            UpdateEnvironmentError::InternalServer(ref cause) => write!(f, "{}", cause),
            UpdateEnvironmentError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for UpdateEnvironmentError {}
/// Errors returned by ValidateConfiguration
#[derive(Debug, PartialEq)]
pub enum ValidateConfigurationError {
    /// <p>The input fails to satisfy the constraints specified by an AWS service.</p>
    BadRequest(String),
    /// <p>There was an internal failure in the AppConfig service.</p>
    InternalServer(String),
    /// <p>The requested resource could not be found.</p>
    ResourceNotFound(String),
}

impl ValidateConfigurationError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ValidateConfigurationError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(ValidateConfigurationError::BadRequest(err.msg))
                }
                "InternalServerException" => {
                    return RusotoError::Service(ValidateConfigurationError::InternalServer(
                        err.msg,
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(ValidateConfigurationError::ResourceNotFound(
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
impl fmt::Display for ValidateConfigurationError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ValidateConfigurationError::BadRequest(ref cause) => write!(f, "{}", cause),
            ValidateConfigurationError::InternalServer(ref cause) => write!(f, "{}", cause),
            ValidateConfigurationError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ValidateConfigurationError {}
/// Trait representing the capabilities of the AppConfig API. AppConfig clients implement this trait.
#[async_trait]
pub trait AppConfig {
    /// <p>An application in AppConfig is a logical unit of code that provides capabilities for your customers. For example, an application can be a microservice that runs on Amazon EC2 instances, a mobile application installed by your users, a serverless application using Amazon API Gateway and AWS Lambda, or any system you run on behalf of others.</p>
    async fn create_application(
        &self,
        input: CreateApplicationRequest,
    ) -> Result<Application, RusotoError<CreateApplicationError>>;

    /// <p><p>Information that enables AppConfig to access the configuration source. Valid configuration sources include Systems Manager (SSM) documents and SSM Parameter Store parameters. A configuration profile includes the following information.</p> <ul> <li> <p>The Uri location of the configuration data.</p> </li> <li> <p>The AWS Identity and Access Management (IAM) role that provides access to the configuration data.</p> </li> <li> <p>A validator for the configuration data. Available validators include either a JSON Schema or an AWS Lambda function.</p> </li> </ul></p>
    async fn create_configuration_profile(
        &self,
        input: CreateConfigurationProfileRequest,
    ) -> Result<ConfigurationProfile, RusotoError<CreateConfigurationProfileError>>;

    /// <p>A deployment strategy defines important criteria for rolling out your configuration to the designated targets. A deployment strategy includes: the overall duration required, a percentage of targets to receive the deployment during each interval, an algorithm that defines how percentage grows, and bake time.</p>
    async fn create_deployment_strategy(
        &self,
        input: CreateDeploymentStrategyRequest,
    ) -> Result<DeploymentStrategy, RusotoError<CreateDeploymentStrategyError>>;

    /// <p>For each application, you define one or more environments. An environment is a logical deployment group of AppConfig targets, such as applications in a <code>Beta</code> or <code>Production</code> environment. You can also define environments for application subcomponents such as the <code>Web</code>, <code>Mobile</code> and <code>Back-end</code> components for your application. You can configure Amazon CloudWatch alarms for each environment. The system monitors alarms during a configuration deployment. If an alarm is triggered, the system rolls back the configuration.</p>
    async fn create_environment(
        &self,
        input: CreateEnvironmentRequest,
    ) -> Result<Environment, RusotoError<CreateEnvironmentError>>;

    /// <p>Delete an application. Deleting an application does not delete a configuration from a host.</p>
    async fn delete_application(
        &self,
        input: DeleteApplicationRequest,
    ) -> Result<(), RusotoError<DeleteApplicationError>>;

    /// <p>Delete a configuration profile. Deleting a configuration profile does not delete a configuration from a host.</p>
    async fn delete_configuration_profile(
        &self,
        input: DeleteConfigurationProfileRequest,
    ) -> Result<(), RusotoError<DeleteConfigurationProfileError>>;

    /// <p>Delete a deployment strategy. Deleting a deployment strategy does not delete a configuration from a host.</p>
    async fn delete_deployment_strategy(
        &self,
        input: DeleteDeploymentStrategyRequest,
    ) -> Result<(), RusotoError<DeleteDeploymentStrategyError>>;

    /// <p>Delete an environment. Deleting an environment does not delete a configuration from a host.</p>
    async fn delete_environment(
        &self,
        input: DeleteEnvironmentRequest,
    ) -> Result<(), RusotoError<DeleteEnvironmentError>>;

    /// <p>Retrieve information about an application.</p>
    async fn get_application(
        &self,
        input: GetApplicationRequest,
    ) -> Result<Application, RusotoError<GetApplicationError>>;

    /// <p>Retrieve information about a configuration.</p>
    async fn get_configuration(
        &self,
        input: GetConfigurationRequest,
    ) -> Result<Configuration, RusotoError<GetConfigurationError>>;

    /// <p>Retrieve information about a configuration profile.</p>
    async fn get_configuration_profile(
        &self,
        input: GetConfigurationProfileRequest,
    ) -> Result<ConfigurationProfile, RusotoError<GetConfigurationProfileError>>;

    /// <p>Retrieve information about a configuration deployment.</p>
    async fn get_deployment(
        &self,
        input: GetDeploymentRequest,
    ) -> Result<Deployment, RusotoError<GetDeploymentError>>;

    /// <p>Retrieve information about a deployment strategy. A deployment strategy defines important criteria for rolling out your configuration to the designated targets. A deployment strategy includes: the overall duration required, a percentage of targets to receive the deployment during each interval, an algorithm that defines how percentage grows, and bake time.</p>
    async fn get_deployment_strategy(
        &self,
        input: GetDeploymentStrategyRequest,
    ) -> Result<DeploymentStrategy, RusotoError<GetDeploymentStrategyError>>;

    /// <p>Retrieve information about an environment. An environment is a logical deployment group of AppConfig applications, such as applications in a <code>Production</code> environment or in an <code>EU_Region</code> environment. Each configuration deployment targets an environment. You can enable one or more Amazon CloudWatch alarms for an environment. If an alarm is triggered during a deployment, AppConfig roles back the configuration.</p>
    async fn get_environment(
        &self,
        input: GetEnvironmentRequest,
    ) -> Result<Environment, RusotoError<GetEnvironmentError>>;

    /// <p>List all applications in your AWS account.</p>
    async fn list_applications(
        &self,
        input: ListApplicationsRequest,
    ) -> Result<Applications, RusotoError<ListApplicationsError>>;

    /// <p>Lists the configuration profiles for an application.</p>
    async fn list_configuration_profiles(
        &self,
        input: ListConfigurationProfilesRequest,
    ) -> Result<ConfigurationProfiles, RusotoError<ListConfigurationProfilesError>>;

    /// <p>List deployment strategies.</p>
    async fn list_deployment_strategies(
        &self,
        input: ListDeploymentStrategiesRequest,
    ) -> Result<DeploymentStrategies, RusotoError<ListDeploymentStrategiesError>>;

    /// <p>Lists the deployments for an environment.</p>
    async fn list_deployments(
        &self,
        input: ListDeploymentsRequest,
    ) -> Result<Deployments, RusotoError<ListDeploymentsError>>;

    /// <p>List the environments for an application.</p>
    async fn list_environments(
        &self,
        input: ListEnvironmentsRequest,
    ) -> Result<Environments, RusotoError<ListEnvironmentsError>>;

    /// <p>Retrieves the list of key-value tags assigned to the resource.</p>
    async fn list_tags_for_resource(
        &self,
        input: ListTagsForResourceRequest,
    ) -> Result<ResourceTags, RusotoError<ListTagsForResourceError>>;

    /// <p>Starts a deployment.</p>
    async fn start_deployment(
        &self,
        input: StartDeploymentRequest,
    ) -> Result<Deployment, RusotoError<StartDeploymentError>>;

    /// <p>Stops a deployment. This API action works only on deployments that have a status of <code>DEPLOYING</code>. This action moves the deployment to a status of <code>ROLLED_BACK</code>.</p>
    async fn stop_deployment(
        &self,
        input: StopDeploymentRequest,
    ) -> Result<Deployment, RusotoError<StopDeploymentError>>;

    /// <p>Metadata to assign to an AppConfig resource. Tags help organize and categorize your AppConfig resources. Each tag consists of a key and an optional value, both of which you define. You can specify a maximum of 50 tags for a resource.</p>
    async fn tag_resource(
        &self,
        input: TagResourceRequest,
    ) -> Result<(), RusotoError<TagResourceError>>;

    /// <p>Deletes a tag key and value from an AppConfig resource.</p>
    async fn untag_resource(
        &self,
        input: UntagResourceRequest,
    ) -> Result<(), RusotoError<UntagResourceError>>;

    /// <p>Updates an application.</p>
    async fn update_application(
        &self,
        input: UpdateApplicationRequest,
    ) -> Result<Application, RusotoError<UpdateApplicationError>>;

    /// <p>Updates a configuration profile.</p>
    async fn update_configuration_profile(
        &self,
        input: UpdateConfigurationProfileRequest,
    ) -> Result<ConfigurationProfile, RusotoError<UpdateConfigurationProfileError>>;

    /// <p>Updates a deployment strategy.</p>
    async fn update_deployment_strategy(
        &self,
        input: UpdateDeploymentStrategyRequest,
    ) -> Result<DeploymentStrategy, RusotoError<UpdateDeploymentStrategyError>>;

    /// <p>Updates an environment.</p>
    async fn update_environment(
        &self,
        input: UpdateEnvironmentRequest,
    ) -> Result<Environment, RusotoError<UpdateEnvironmentError>>;

    /// <p>Uses the validators in a configuration profile to validate a configuration.</p>
    async fn validate_configuration(
        &self,
        input: ValidateConfigurationRequest,
    ) -> Result<(), RusotoError<ValidateConfigurationError>>;
}
/// A client for the AppConfig API.
#[derive(Clone)]
pub struct AppConfigClient {
    client: Client,
    region: region::Region,
}

impl AppConfigClient {
    /// Creates a client backed by the default tokio event loop.
    ///
    /// The client will use the default credentials provider and tls client.
    pub fn new(region: region::Region) -> AppConfigClient {
        AppConfigClient {
            client: Client::shared(),
            region,
        }
    }

    pub fn new_with<P, D>(
        request_dispatcher: D,
        credentials_provider: P,
        region: region::Region,
    ) -> AppConfigClient
    where
        P: ProvideAwsCredentials + Send + Sync + 'static,
        D: DispatchSignedRequest + Send + Sync + 'static,
    {
        AppConfigClient {
            client: Client::new_with(credentials_provider, request_dispatcher),
            region,
        }
    }

    pub fn new_with_client(client: Client, region: region::Region) -> AppConfigClient {
        AppConfigClient { client, region }
    }
}

#[async_trait]
impl AppConfig for AppConfigClient {
    /// <p>An application in AppConfig is a logical unit of code that provides capabilities for your customers. For example, an application can be a microservice that runs on Amazon EC2 instances, a mobile application installed by your users, a serverless application using Amazon API Gateway and AWS Lambda, or any system you run on behalf of others.</p>
    async fn create_application(
        &self,
        input: CreateApplicationRequest,
    ) -> Result<Application, RusotoError<CreateApplicationError>> {
        let request_uri = "/applications";

        let mut request = SignedRequest::new("POST", "appconfig", &self.region, &request_uri);
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
            let result =
                proto::json::ResponsePayload::new(&response).deserialize::<Application, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(CreateApplicationError::from_response(response))
        }
    }

    /// <p><p>Information that enables AppConfig to access the configuration source. Valid configuration sources include Systems Manager (SSM) documents and SSM Parameter Store parameters. A configuration profile includes the following information.</p> <ul> <li> <p>The Uri location of the configuration data.</p> </li> <li> <p>The AWS Identity and Access Management (IAM) role that provides access to the configuration data.</p> </li> <li> <p>A validator for the configuration data. Available validators include either a JSON Schema or an AWS Lambda function.</p> </li> </ul></p>
    async fn create_configuration_profile(
        &self,
        input: CreateConfigurationProfileRequest,
    ) -> Result<ConfigurationProfile, RusotoError<CreateConfigurationProfileError>> {
        let request_uri = format!(
            "/applications/{application_id}/configurationprofiles",
            application_id = input.application_id
        );

        let mut request = SignedRequest::new("POST", "appconfig", &self.region, &request_uri);
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
                .deserialize::<ConfigurationProfile, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(CreateConfigurationProfileError::from_response(response))
        }
    }

    /// <p>A deployment strategy defines important criteria for rolling out your configuration to the designated targets. A deployment strategy includes: the overall duration required, a percentage of targets to receive the deployment during each interval, an algorithm that defines how percentage grows, and bake time.</p>
    async fn create_deployment_strategy(
        &self,
        input: CreateDeploymentStrategyRequest,
    ) -> Result<DeploymentStrategy, RusotoError<CreateDeploymentStrategyError>> {
        let request_uri = "/deploymentstrategies";

        let mut request = SignedRequest::new("POST", "appconfig", &self.region, &request_uri);
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
                .deserialize::<DeploymentStrategy, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(CreateDeploymentStrategyError::from_response(response))
        }
    }

    /// <p>For each application, you define one or more environments. An environment is a logical deployment group of AppConfig targets, such as applications in a <code>Beta</code> or <code>Production</code> environment. You can also define environments for application subcomponents such as the <code>Web</code>, <code>Mobile</code> and <code>Back-end</code> components for your application. You can configure Amazon CloudWatch alarms for each environment. The system monitors alarms during a configuration deployment. If an alarm is triggered, the system rolls back the configuration.</p>
    async fn create_environment(
        &self,
        input: CreateEnvironmentRequest,
    ) -> Result<Environment, RusotoError<CreateEnvironmentError>> {
        let request_uri = format!(
            "/applications/{application_id}/environments",
            application_id = input.application_id
        );

        let mut request = SignedRequest::new("POST", "appconfig", &self.region, &request_uri);
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
            let result =
                proto::json::ResponsePayload::new(&response).deserialize::<Environment, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(CreateEnvironmentError::from_response(response))
        }
    }

    /// <p>Delete an application. Deleting an application does not delete a configuration from a host.</p>
    async fn delete_application(
        &self,
        input: DeleteApplicationRequest,
    ) -> Result<(), RusotoError<DeleteApplicationError>> {
        let request_uri = format!(
            "/applications/{application_id}",
            application_id = input.application_id
        );

        let mut request = SignedRequest::new("DELETE", "appconfig", &self.region, &request_uri);
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
            Err(DeleteApplicationError::from_response(response))
        }
    }

    /// <p>Delete a configuration profile. Deleting a configuration profile does not delete a configuration from a host.</p>
    async fn delete_configuration_profile(
        &self,
        input: DeleteConfigurationProfileRequest,
    ) -> Result<(), RusotoError<DeleteConfigurationProfileError>> {
        let request_uri = format!(
            "/applications/{application_id}/configurationprofiles/{configuration_profile_id}",
            application_id = input.application_id,
            configuration_profile_id = input.configuration_profile_id
        );

        let mut request = SignedRequest::new("DELETE", "appconfig", &self.region, &request_uri);
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
            Err(DeleteConfigurationProfileError::from_response(response))
        }
    }

    /// <p>Delete a deployment strategy. Deleting a deployment strategy does not delete a configuration from a host.</p>
    async fn delete_deployment_strategy(
        &self,
        input: DeleteDeploymentStrategyRequest,
    ) -> Result<(), RusotoError<DeleteDeploymentStrategyError>> {
        let request_uri = format!(
            "/deployementstrategies/{deployment_strategy_id}",
            deployment_strategy_id = input.deployment_strategy_id
        );

        let mut request = SignedRequest::new("DELETE", "appconfig", &self.region, &request_uri);
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
            Err(DeleteDeploymentStrategyError::from_response(response))
        }
    }

    /// <p>Delete an environment. Deleting an environment does not delete a configuration from a host.</p>
    async fn delete_environment(
        &self,
        input: DeleteEnvironmentRequest,
    ) -> Result<(), RusotoError<DeleteEnvironmentError>> {
        let request_uri = format!(
            "/applications/{application_id}/environments/{environment_id}",
            application_id = input.application_id,
            environment_id = input.environment_id
        );

        let mut request = SignedRequest::new("DELETE", "appconfig", &self.region, &request_uri);
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
            Err(DeleteEnvironmentError::from_response(response))
        }
    }

    /// <p>Retrieve information about an application.</p>
    async fn get_application(
        &self,
        input: GetApplicationRequest,
    ) -> Result<Application, RusotoError<GetApplicationError>> {
        let request_uri = format!(
            "/applications/{application_id}",
            application_id = input.application_id
        );

        let mut request = SignedRequest::new("GET", "appconfig", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result =
                proto::json::ResponsePayload::new(&response).deserialize::<Application, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(GetApplicationError::from_response(response))
        }
    }

    /// <p>Retrieve information about a configuration.</p>
    async fn get_configuration(
        &self,
        input: GetConfigurationRequest,
    ) -> Result<Configuration, RusotoError<GetConfigurationError>> {
        let request_uri = format!(
            "/applications/{application}/environments/{environment}/configurations/{configuration}",
            application = input.application,
            configuration = input.configuration,
            environment = input.environment
        );

        let mut request = SignedRequest::new("GET", "appconfig", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut params = Params::new();
        if let Some(ref x) = input.client_configuration_version {
            params.put("client_configuration_version", x);
        }
        params.put("client_id", &input.client_id);
        request.set_params(params);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;

            let mut result = Configuration::default();
            result.content = Some(response.body);

            if let Some(configuration_version) = response.headers.get("Configuration-Version") {
                let value = configuration_version.to_owned();
                result.configuration_version = Some(value)
            };
            if let Some(content_type) = response.headers.get("Content-Type") {
                let value = content_type.to_owned();
                result.content_type = Some(value)
            };

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(GetConfigurationError::from_response(response))
        }
    }

    /// <p>Retrieve information about a configuration profile.</p>
    async fn get_configuration_profile(
        &self,
        input: GetConfigurationProfileRequest,
    ) -> Result<ConfigurationProfile, RusotoError<GetConfigurationProfileError>> {
        let request_uri = format!(
            "/applications/{application_id}/configurationprofiles/{configuration_profile_id}",
            application_id = input.application_id,
            configuration_profile_id = input.configuration_profile_id
        );

        let mut request = SignedRequest::new("GET", "appconfig", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<ConfigurationProfile, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(GetConfigurationProfileError::from_response(response))
        }
    }

    /// <p>Retrieve information about a configuration deployment.</p>
    async fn get_deployment(
        &self,
        input: GetDeploymentRequest,
    ) -> Result<Deployment, RusotoError<GetDeploymentError>> {
        let request_uri = format!("/applications/{application_id}/environments/{environment_id}/deployments/{deployment_number}", application_id = input.application_id, deployment_number = input.deployment_number, environment_id = input.environment_id);

        let mut request = SignedRequest::new("GET", "appconfig", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result =
                proto::json::ResponsePayload::new(&response).deserialize::<Deployment, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(GetDeploymentError::from_response(response))
        }
    }

    /// <p>Retrieve information about a deployment strategy. A deployment strategy defines important criteria for rolling out your configuration to the designated targets. A deployment strategy includes: the overall duration required, a percentage of targets to receive the deployment during each interval, an algorithm that defines how percentage grows, and bake time.</p>
    async fn get_deployment_strategy(
        &self,
        input: GetDeploymentStrategyRequest,
    ) -> Result<DeploymentStrategy, RusotoError<GetDeploymentStrategyError>> {
        let request_uri = format!(
            "/deploymentstrategies/{deployment_strategy_id}",
            deployment_strategy_id = input.deployment_strategy_id
        );

        let mut request = SignedRequest::new("GET", "appconfig", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<DeploymentStrategy, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(GetDeploymentStrategyError::from_response(response))
        }
    }

    /// <p>Retrieve information about an environment. An environment is a logical deployment group of AppConfig applications, such as applications in a <code>Production</code> environment or in an <code>EU_Region</code> environment. Each configuration deployment targets an environment. You can enable one or more Amazon CloudWatch alarms for an environment. If an alarm is triggered during a deployment, AppConfig roles back the configuration.</p>
    async fn get_environment(
        &self,
        input: GetEnvironmentRequest,
    ) -> Result<Environment, RusotoError<GetEnvironmentError>> {
        let request_uri = format!(
            "/applications/{application_id}/environments/{environment_id}",
            application_id = input.application_id,
            environment_id = input.environment_id
        );

        let mut request = SignedRequest::new("GET", "appconfig", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result =
                proto::json::ResponsePayload::new(&response).deserialize::<Environment, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(GetEnvironmentError::from_response(response))
        }
    }

    /// <p>List all applications in your AWS account.</p>
    async fn list_applications(
        &self,
        input: ListApplicationsRequest,
    ) -> Result<Applications, RusotoError<ListApplicationsError>> {
        let request_uri = "/applications";

        let mut request = SignedRequest::new("GET", "appconfig", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut params = Params::new();
        if let Some(ref x) = input.max_results {
            params.put("max_results", x);
        }
        if let Some(ref x) = input.next_token {
            params.put("next_token", x);
        }
        request.set_params(params);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result =
                proto::json::ResponsePayload::new(&response).deserialize::<Applications, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(ListApplicationsError::from_response(response))
        }
    }

    /// <p>Lists the configuration profiles for an application.</p>
    async fn list_configuration_profiles(
        &self,
        input: ListConfigurationProfilesRequest,
    ) -> Result<ConfigurationProfiles, RusotoError<ListConfigurationProfilesError>> {
        let request_uri = format!(
            "/applications/{application_id}/configurationprofiles",
            application_id = input.application_id
        );

        let mut request = SignedRequest::new("GET", "appconfig", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut params = Params::new();
        if let Some(ref x) = input.max_results {
            params.put("max_results", x);
        }
        if let Some(ref x) = input.next_token {
            params.put("next_token", x);
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
                .deserialize::<ConfigurationProfiles, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(ListConfigurationProfilesError::from_response(response))
        }
    }

    /// <p>List deployment strategies.</p>
    async fn list_deployment_strategies(
        &self,
        input: ListDeploymentStrategiesRequest,
    ) -> Result<DeploymentStrategies, RusotoError<ListDeploymentStrategiesError>> {
        let request_uri = "/deploymentstrategies";

        let mut request = SignedRequest::new("GET", "appconfig", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut params = Params::new();
        if let Some(ref x) = input.max_results {
            params.put("max_results", x);
        }
        if let Some(ref x) = input.next_token {
            params.put("next_token", x);
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
                .deserialize::<DeploymentStrategies, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(ListDeploymentStrategiesError::from_response(response))
        }
    }

    /// <p>Lists the deployments for an environment.</p>
    async fn list_deployments(
        &self,
        input: ListDeploymentsRequest,
    ) -> Result<Deployments, RusotoError<ListDeploymentsError>> {
        let request_uri = format!(
            "/applications/{application_id}/environments/{environment_id}/deployments",
            application_id = input.application_id,
            environment_id = input.environment_id
        );

        let mut request = SignedRequest::new("GET", "appconfig", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut params = Params::new();
        if let Some(ref x) = input.max_results {
            params.put("max_results", x);
        }
        if let Some(ref x) = input.next_token {
            params.put("next_token", x);
        }
        request.set_params(params);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result =
                proto::json::ResponsePayload::new(&response).deserialize::<Deployments, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(ListDeploymentsError::from_response(response))
        }
    }

    /// <p>List the environments for an application.</p>
    async fn list_environments(
        &self,
        input: ListEnvironmentsRequest,
    ) -> Result<Environments, RusotoError<ListEnvironmentsError>> {
        let request_uri = format!(
            "/applications/{application_id}/environments",
            application_id = input.application_id
        );

        let mut request = SignedRequest::new("GET", "appconfig", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut params = Params::new();
        if let Some(ref x) = input.max_results {
            params.put("max_results", x);
        }
        if let Some(ref x) = input.next_token {
            params.put("next_token", x);
        }
        request.set_params(params);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result =
                proto::json::ResponsePayload::new(&response).deserialize::<Environments, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(ListEnvironmentsError::from_response(response))
        }
    }

    /// <p>Retrieves the list of key-value tags assigned to the resource.</p>
    async fn list_tags_for_resource(
        &self,
        input: ListTagsForResourceRequest,
    ) -> Result<ResourceTags, RusotoError<ListTagsForResourceError>> {
        let request_uri = format!("/tags/{resource_arn}", resource_arn = input.resource_arn);

        let mut request = SignedRequest::new("GET", "appconfig", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result =
                proto::json::ResponsePayload::new(&response).deserialize::<ResourceTags, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(ListTagsForResourceError::from_response(response))
        }
    }

    /// <p>Starts a deployment.</p>
    async fn start_deployment(
        &self,
        input: StartDeploymentRequest,
    ) -> Result<Deployment, RusotoError<StartDeploymentError>> {
        let request_uri = format!(
            "/applications/{application_id}/environments/{environment_id}/deployments",
            application_id = input.application_id,
            environment_id = input.environment_id
        );

        let mut request = SignedRequest::new("POST", "appconfig", &self.region, &request_uri);
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
            let result =
                proto::json::ResponsePayload::new(&response).deserialize::<Deployment, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(StartDeploymentError::from_response(response))
        }
    }

    /// <p>Stops a deployment. This API action works only on deployments that have a status of <code>DEPLOYING</code>. This action moves the deployment to a status of <code>ROLLED_BACK</code>.</p>
    async fn stop_deployment(
        &self,
        input: StopDeploymentRequest,
    ) -> Result<Deployment, RusotoError<StopDeploymentError>> {
        let request_uri = format!("/applications/{application_id}/environments/{environment_id}/deployments/{deployment_number}", application_id = input.application_id, deployment_number = input.deployment_number, environment_id = input.environment_id);

        let mut request = SignedRequest::new("DELETE", "appconfig", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 202 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result =
                proto::json::ResponsePayload::new(&response).deserialize::<Deployment, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(StopDeploymentError::from_response(response))
        }
    }

    /// <p>Metadata to assign to an AppConfig resource. Tags help organize and categorize your AppConfig resources. Each tag consists of a key and an optional value, both of which you define. You can specify a maximum of 50 tags for a resource.</p>
    async fn tag_resource(
        &self,
        input: TagResourceRequest,
    ) -> Result<(), RusotoError<TagResourceError>> {
        let request_uri = format!("/tags/{resource_arn}", resource_arn = input.resource_arn);

        let mut request = SignedRequest::new("POST", "appconfig", &self.region, &request_uri);
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

    /// <p>Deletes a tag key and value from an AppConfig resource.</p>
    async fn untag_resource(
        &self,
        input: UntagResourceRequest,
    ) -> Result<(), RusotoError<UntagResourceError>> {
        let request_uri = format!("/tags/{resource_arn}", resource_arn = input.resource_arn);

        let mut request = SignedRequest::new("DELETE", "appconfig", &self.region, &request_uri);
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

    /// <p>Updates an application.</p>
    async fn update_application(
        &self,
        input: UpdateApplicationRequest,
    ) -> Result<Application, RusotoError<UpdateApplicationError>> {
        let request_uri = format!(
            "/applications/{application_id}",
            application_id = input.application_id
        );

        let mut request = SignedRequest::new("PATCH", "appconfig", &self.region, &request_uri);
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
            let result =
                proto::json::ResponsePayload::new(&response).deserialize::<Application, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(UpdateApplicationError::from_response(response))
        }
    }

    /// <p>Updates a configuration profile.</p>
    async fn update_configuration_profile(
        &self,
        input: UpdateConfigurationProfileRequest,
    ) -> Result<ConfigurationProfile, RusotoError<UpdateConfigurationProfileError>> {
        let request_uri = format!(
            "/applications/{application_id}/configurationprofiles/{configuration_profile_id}",
            application_id = input.application_id,
            configuration_profile_id = input.configuration_profile_id
        );

        let mut request = SignedRequest::new("PATCH", "appconfig", &self.region, &request_uri);
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
                .deserialize::<ConfigurationProfile, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(UpdateConfigurationProfileError::from_response(response))
        }
    }

    /// <p>Updates a deployment strategy.</p>
    async fn update_deployment_strategy(
        &self,
        input: UpdateDeploymentStrategyRequest,
    ) -> Result<DeploymentStrategy, RusotoError<UpdateDeploymentStrategyError>> {
        let request_uri = format!(
            "/deploymentstrategies/{deployment_strategy_id}",
            deployment_strategy_id = input.deployment_strategy_id
        );

        let mut request = SignedRequest::new("PATCH", "appconfig", &self.region, &request_uri);
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
                .deserialize::<DeploymentStrategy, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(UpdateDeploymentStrategyError::from_response(response))
        }
    }

    /// <p>Updates an environment.</p>
    async fn update_environment(
        &self,
        input: UpdateEnvironmentRequest,
    ) -> Result<Environment, RusotoError<UpdateEnvironmentError>> {
        let request_uri = format!(
            "/applications/{application_id}/environments/{environment_id}",
            application_id = input.application_id,
            environment_id = input.environment_id
        );

        let mut request = SignedRequest::new("PATCH", "appconfig", &self.region, &request_uri);
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
            let result =
                proto::json::ResponsePayload::new(&response).deserialize::<Environment, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(UpdateEnvironmentError::from_response(response))
        }
    }

    /// <p>Uses the validators in a configuration profile to validate a configuration.</p>
    async fn validate_configuration(
        &self,
        input: ValidateConfigurationRequest,
    ) -> Result<(), RusotoError<ValidateConfigurationError>> {
        let request_uri = format!("/applications/{application_id}/configurationprofiles/{configuration_profile_id}/validators", application_id = input.application_id, configuration_profile_id = input.configuration_profile_id);

        let mut request = SignedRequest::new("POST", "appconfig", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut params = Params::new();
        params.put("configuration_version", &input.configuration_version);
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
            Err(ValidateConfigurationError::from_response(response))
        }
    }
}
