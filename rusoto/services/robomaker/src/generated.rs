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
pub struct BatchDeleteWorldsRequest {
    /// <p>A list of Amazon Resource Names (arns) that correspond to worlds to delete.</p>
    #[serde(rename = "worlds")]
    pub worlds: Vec<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct BatchDeleteWorldsResponse {
    /// <p>A list of unprocessed worlds associated with the call. These worlds were not deleted.</p>
    #[serde(rename = "unprocessedWorlds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unprocessed_worlds: Option<Vec<String>>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct BatchDescribeSimulationJobRequest {
    /// <p>A list of Amazon Resource Names (ARNs) of simulation jobs to describe.</p>
    #[serde(rename = "jobs")]
    pub jobs: Vec<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct BatchDescribeSimulationJobResponse {
    /// <p>A list of simulation jobs.</p>
    #[serde(rename = "jobs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub jobs: Option<Vec<SimulationJob>>,
    /// <p>A list of unprocessed simulation job Amazon Resource Names (ARNs).</p>
    #[serde(rename = "unprocessedJobs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unprocessed_jobs: Option<Vec<String>>,
}

/// <p>Information about the batch policy.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct BatchPolicy {
    /// <p>The number of active simulation jobs create as part of the batch that can be in an active state at the same time. </p> <p>Active states include: <code>Pending</code>,<code>Preparing</code>, <code>Running</code>, <code>Restarting</code>, <code>RunningFailed</code> and <code>Terminating</code>. All other states are terminal states. </p>
    #[serde(rename = "maxConcurrency")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_concurrency: Option<i64>,
    /// <p>The amount of time, in seconds, to wait for the batch to complete. </p> <p>If a batch times out, and there are pending requests that were failing due to an internal failure (like <code>InternalServiceError</code>), they will be moved to the failed list and the batch status will be <code>Failed</code>. If the pending requests were failing for any other reason, the failed pending requests will be moved to the failed list and the batch status will be <code>TimedOut</code>. </p>
    #[serde(rename = "timeoutInSeconds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeout_in_seconds: Option<i64>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CancelDeploymentJobRequest {
    /// <p>The deployment job ARN to cancel.</p>
    #[serde(rename = "job")]
    pub job: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CancelDeploymentJobResponse {}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CancelSimulationJobBatchRequest {
    /// <p>The id of the batch to cancel.</p>
    #[serde(rename = "batch")]
    pub batch: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CancelSimulationJobBatchResponse {}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CancelSimulationJobRequest {
    /// <p>The simulation job ARN to cancel.</p>
    #[serde(rename = "job")]
    pub job: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CancelSimulationJobResponse {}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CancelWorldExportJobRequest {
    /// <p>The Amazon Resource Name (arn) of the world export job to cancel.</p>
    #[serde(rename = "job")]
    pub job: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CancelWorldExportJobResponse {}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CancelWorldGenerationJobRequest {
    /// <p>The Amazon Resource Name (arn) of the world generator job to cancel.</p>
    #[serde(rename = "job")]
    pub job: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CancelWorldGenerationJobResponse {}

/// <p>Compute information for the simulation job.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct Compute {
    /// <p>The simulation unit limit. Your simulation is allocated CPU and memory proportional to the supplied simulation unit limit. A simulation unit is 1 vcpu and 2GB of memory. You are only billed for the SU utilization you consume up to the maximim value provided. The default is 15. </p>
    #[serde(rename = "simulationUnitLimit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub simulation_unit_limit: Option<i64>,
}

/// <p>Compute information for the simulation job</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ComputeResponse {
    /// <p>The simulation unit limit. Your simulation is allocated CPU and memory proportional to the supplied simulation unit limit. A simulation unit is 1 vcpu and 2GB of memory. You are only billed for the SU utilization you consume up to the maximim value provided. The default is 15. </p>
    #[serde(rename = "simulationUnitLimit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub simulation_unit_limit: Option<i64>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateDeploymentJobRequest {
    /// <p>Unique, case-sensitive identifier that you provide to ensure the idempotency of the request.</p>
    #[serde(rename = "clientRequestToken")]
    pub client_request_token: String,
    /// <p>The deployment application configuration.</p>
    #[serde(rename = "deploymentApplicationConfigs")]
    pub deployment_application_configs: Vec<DeploymentApplicationConfig>,
    /// <p>The requested deployment configuration.</p>
    #[serde(rename = "deploymentConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deployment_config: Option<DeploymentConfig>,
    /// <p>The Amazon Resource Name (ARN) of the fleet to deploy.</p>
    #[serde(rename = "fleet")]
    pub fleet: String,
    /// <p>A map that contains tag keys and tag values that are attached to the deployment job.</p>
    #[serde(rename = "tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<::std::collections::HashMap<String, String>>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateDeploymentJobResponse {
    /// <p>The Amazon Resource Name (ARN) of the deployment job.</p>
    #[serde(rename = "arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p>The time, in milliseconds since the epoch, when the fleet was created.</p>
    #[serde(rename = "createdAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<f64>,
    /// <p>The deployment application configuration.</p>
    #[serde(rename = "deploymentApplicationConfigs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deployment_application_configs: Option<Vec<DeploymentApplicationConfig>>,
    /// <p>The deployment configuration.</p>
    #[serde(rename = "deploymentConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deployment_config: Option<DeploymentConfig>,
    /// <p><p>The failure code of the simulation job if it failed:</p> <dl> <dt>BadPermissionError</dt> <dd> <p>AWS Greengrass requires a service-level role permission to access other services. The role must include the <a href="https://console.aws.amazon.com/iam/home?#/policies/arn:aws:iam::aws:policy/service-role/AWSGreengrassResourceAccessRolePolicy$jsonEditor"> <code>AWSGreengrassResourceAccessRolePolicy</code> managed policy</a>. </p> </dd> <dt>ExtractingBundleFailure</dt> <dd> <p>The robot application could not be extracted from the bundle.</p> </dd> <dt>FailureThresholdBreached</dt> <dd> <p>The percentage of robots that could not be updated exceeded the percentage set for the deployment.</p> </dd> <dt>GreengrassDeploymentFailed</dt> <dd> <p>The robot application could not be deployed to the robot.</p> </dd> <dt>GreengrassGroupVersionDoesNotExist</dt> <dd> <p>The AWS Greengrass group or version associated with a robot is missing.</p> </dd> <dt>InternalServerError</dt> <dd> <p>An internal error has occurred. Retry your request, but if the problem persists, contact us with details.</p> </dd> <dt>MissingRobotApplicationArchitecture</dt> <dd> <p>The robot application does not have a source that matches the architecture of the robot.</p> </dd> <dt>MissingRobotDeploymentResource</dt> <dd> <p>One or more of the resources specified for the robot application are missing. For example, does the robot application have the correct launch package and launch file?</p> </dd> <dt>PostLaunchFileFailure</dt> <dd> <p>The post-launch script failed.</p> </dd> <dt>PreLaunchFileFailure</dt> <dd> <p>The pre-launch script failed.</p> </dd> <dt>ResourceNotFound</dt> <dd> <p>One or more deployment resources are missing. For example, do robot application source bundles still exist? </p> </dd> <dt>RobotDeploymentNoResponse</dt> <dd> <p>There is no response from the robot. It might not be powered on or connected to the internet.</p> </dd> </dl></p>
    #[serde(rename = "failureCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_code: Option<String>,
    /// <p>The failure reason of the deployment job if it failed.</p>
    #[serde(rename = "failureReason")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_reason: Option<String>,
    /// <p>The target fleet for the deployment job.</p>
    #[serde(rename = "fleet")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fleet: Option<String>,
    /// <p>The status of the deployment job.</p>
    #[serde(rename = "status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// <p>The list of all tags added to the deployment job.</p>
    #[serde(rename = "tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<::std::collections::HashMap<String, String>>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateFleetRequest {
    /// <p>The name of the fleet.</p>
    #[serde(rename = "name")]
    pub name: String,
    /// <p>A map that contains tag keys and tag values that are attached to the fleet.</p>
    #[serde(rename = "tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<::std::collections::HashMap<String, String>>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateFleetResponse {
    /// <p>The Amazon Resource Name (ARN) of the fleet.</p>
    #[serde(rename = "arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p>The time, in milliseconds since the epoch, when the fleet was created.</p>
    #[serde(rename = "createdAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<f64>,
    /// <p>The name of the fleet.</p>
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The list of all tags added to the fleet.</p>
    #[serde(rename = "tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<::std::collections::HashMap<String, String>>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateRobotApplicationRequest {
    /// <p>The name of the robot application.</p>
    #[serde(rename = "name")]
    pub name: String,
    /// <p>The robot software suite (ROS distribuition) used by the robot application.</p>
    #[serde(rename = "robotSoftwareSuite")]
    pub robot_software_suite: RobotSoftwareSuite,
    /// <p>The sources of the robot application.</p>
    #[serde(rename = "sources")]
    pub sources: Vec<SourceConfig>,
    /// <p>A map that contains tag keys and tag values that are attached to the robot application.</p>
    #[serde(rename = "tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<::std::collections::HashMap<String, String>>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateRobotApplicationResponse {
    /// <p>The Amazon Resource Name (ARN) of the robot application.</p>
    #[serde(rename = "arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p>The time, in milliseconds since the epoch, when the robot application was last updated.</p>
    #[serde(rename = "lastUpdatedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_at: Option<f64>,
    /// <p>The name of the robot application.</p>
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The revision id of the robot application.</p>
    #[serde(rename = "revisionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub revision_id: Option<String>,
    /// <p>The robot software suite (ROS distribution) used by the robot application.</p>
    #[serde(rename = "robotSoftwareSuite")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub robot_software_suite: Option<RobotSoftwareSuite>,
    /// <p>The sources of the robot application.</p>
    #[serde(rename = "sources")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sources: Option<Vec<Source>>,
    /// <p>The list of all tags added to the robot application.</p>
    #[serde(rename = "tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<::std::collections::HashMap<String, String>>,
    /// <p>The version of the robot application.</p>
    #[serde(rename = "version")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateRobotApplicationVersionRequest {
    /// <p>The application information for the robot application.</p>
    #[serde(rename = "application")]
    pub application: String,
    /// <p>The current revision id for the robot application. If you provide a value and it matches the latest revision ID, a new version will be created.</p>
    #[serde(rename = "currentRevisionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_revision_id: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateRobotApplicationVersionResponse {
    /// <p>The Amazon Resource Name (ARN) of the robot application.</p>
    #[serde(rename = "arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p>The time, in milliseconds since the epoch, when the robot application was last updated.</p>
    #[serde(rename = "lastUpdatedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_at: Option<f64>,
    /// <p>The name of the robot application.</p>
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The revision id of the robot application.</p>
    #[serde(rename = "revisionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub revision_id: Option<String>,
    /// <p>The robot software suite (ROS distribution) used by the robot application.</p>
    #[serde(rename = "robotSoftwareSuite")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub robot_software_suite: Option<RobotSoftwareSuite>,
    /// <p>The sources of the robot application.</p>
    #[serde(rename = "sources")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sources: Option<Vec<Source>>,
    /// <p>The version of the robot application.</p>
    #[serde(rename = "version")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateRobotRequest {
    /// <p>The target architecture of the robot.</p>
    #[serde(rename = "architecture")]
    pub architecture: String,
    /// <p>The Greengrass group id.</p>
    #[serde(rename = "greengrassGroupId")]
    pub greengrass_group_id: String,
    /// <p>The name for the robot.</p>
    #[serde(rename = "name")]
    pub name: String,
    /// <p>A map that contains tag keys and tag values that are attached to the robot.</p>
    #[serde(rename = "tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<::std::collections::HashMap<String, String>>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateRobotResponse {
    /// <p>The target architecture of the robot.</p>
    #[serde(rename = "architecture")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub architecture: Option<String>,
    /// <p>The Amazon Resource Name (ARN) of the robot.</p>
    #[serde(rename = "arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p>The time, in milliseconds since the epoch, when the robot was created.</p>
    #[serde(rename = "createdAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<f64>,
    /// <p>The Amazon Resource Name (ARN) of the Greengrass group associated with the robot.</p>
    #[serde(rename = "greengrassGroupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub greengrass_group_id: Option<String>,
    /// <p>The name of the robot.</p>
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The list of all tags added to the robot.</p>
    #[serde(rename = "tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<::std::collections::HashMap<String, String>>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateSimulationApplicationRequest {
    /// <p>The name of the simulation application.</p>
    #[serde(rename = "name")]
    pub name: String,
    /// <p>The rendering engine for the simulation application.</p>
    #[serde(rename = "renderingEngine")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rendering_engine: Option<RenderingEngine>,
    /// <p>The robot software suite (ROS distribution) used by the simulation application.</p>
    #[serde(rename = "robotSoftwareSuite")]
    pub robot_software_suite: RobotSoftwareSuite,
    /// <p>The simulation software suite used by the simulation application.</p>
    #[serde(rename = "simulationSoftwareSuite")]
    pub simulation_software_suite: SimulationSoftwareSuite,
    /// <p>The sources of the simulation application.</p>
    #[serde(rename = "sources")]
    pub sources: Vec<SourceConfig>,
    /// <p>A map that contains tag keys and tag values that are attached to the simulation application.</p>
    #[serde(rename = "tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<::std::collections::HashMap<String, String>>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateSimulationApplicationResponse {
    /// <p>The Amazon Resource Name (ARN) of the simulation application.</p>
    #[serde(rename = "arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p>The time, in milliseconds since the epoch, when the simulation application was last updated.</p>
    #[serde(rename = "lastUpdatedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_at: Option<f64>,
    /// <p>The name of the simulation application.</p>
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The rendering engine for the simulation application.</p>
    #[serde(rename = "renderingEngine")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rendering_engine: Option<RenderingEngine>,
    /// <p>The revision id of the simulation application.</p>
    #[serde(rename = "revisionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub revision_id: Option<String>,
    /// <p>Information about the robot software suite (ROS distribution).</p>
    #[serde(rename = "robotSoftwareSuite")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub robot_software_suite: Option<RobotSoftwareSuite>,
    /// <p>The simulation software suite used by the simulation application.</p>
    #[serde(rename = "simulationSoftwareSuite")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub simulation_software_suite: Option<SimulationSoftwareSuite>,
    /// <p>The sources of the simulation application.</p>
    #[serde(rename = "sources")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sources: Option<Vec<Source>>,
    /// <p>The list of all tags added to the simulation application.</p>
    #[serde(rename = "tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<::std::collections::HashMap<String, String>>,
    /// <p>The version of the simulation application.</p>
    #[serde(rename = "version")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateSimulationApplicationVersionRequest {
    /// <p>The application information for the simulation application.</p>
    #[serde(rename = "application")]
    pub application: String,
    /// <p>The current revision id for the simulation application. If you provide a value and it matches the latest revision ID, a new version will be created.</p>
    #[serde(rename = "currentRevisionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_revision_id: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateSimulationApplicationVersionResponse {
    /// <p>The Amazon Resource Name (ARN) of the simulation application.</p>
    #[serde(rename = "arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p>The time, in milliseconds since the epoch, when the simulation application was last updated.</p>
    #[serde(rename = "lastUpdatedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_at: Option<f64>,
    /// <p>The name of the simulation application.</p>
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The rendering engine for the simulation application.</p>
    #[serde(rename = "renderingEngine")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rendering_engine: Option<RenderingEngine>,
    /// <p>The revision ID of the simulation application.</p>
    #[serde(rename = "revisionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub revision_id: Option<String>,
    /// <p>Information about the robot software suite (ROS distribution).</p>
    #[serde(rename = "robotSoftwareSuite")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub robot_software_suite: Option<RobotSoftwareSuite>,
    /// <p>The simulation software suite used by the simulation application.</p>
    #[serde(rename = "simulationSoftwareSuite")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub simulation_software_suite: Option<SimulationSoftwareSuite>,
    /// <p>The sources of the simulation application.</p>
    #[serde(rename = "sources")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sources: Option<Vec<Source>>,
    /// <p>The version of the simulation application.</p>
    #[serde(rename = "version")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateSimulationJobRequest {
    /// <p>Unique, case-sensitive identifier that you provide to ensure the idempotency of the request.</p>
    #[serde(rename = "clientRequestToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_request_token: Option<String>,
    /// <p>Compute information for the simulation job.</p>
    #[serde(rename = "compute")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compute: Option<Compute>,
    /// <p><p>Specify data sources to mount read-only files from S3 into your simulation. These files are available under <code>/opt/robomaker/datasources/data<em>source</em>name</code>. </p> <note> <p>There is a limit of 100 files and a combined size of 25GB for all <code>DataSourceConfig</code> objects. </p> </note></p>
    #[serde(rename = "dataSources")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_sources: Option<Vec<DataSourceConfig>>,
    /// <p><p>The failure behavior the simulation job.</p> <dl> <dt>Continue</dt> <dd> <p>Restart the simulation job in the same host instance.</p> </dd> <dt>Fail</dt> <dd> <p>Stop the simulation job and terminate the instance.</p> </dd> </dl></p>
    #[serde(rename = "failureBehavior")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_behavior: Option<String>,
    /// <p>The IAM role name that allows the simulation instance to call the AWS APIs that are specified in its associated policies on your behalf. This is how credentials are passed in to your simulation job. </p>
    #[serde(rename = "iamRole")]
    pub iam_role: String,
    /// <p>The logging configuration.</p>
    #[serde(rename = "loggingConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logging_config: Option<LoggingConfig>,
    /// <p>The maximum simulation job duration in seconds (up to 14 days or 1,209,600 seconds. When <code>maxJobDurationInSeconds</code> is reached, the simulation job will status will transition to <code>Completed</code>.</p>
    #[serde(rename = "maxJobDurationInSeconds")]
    pub max_job_duration_in_seconds: i64,
    /// <p>Location for output files generated by the simulation job.</p>
    #[serde(rename = "outputLocation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_location: Option<OutputLocation>,
    /// <p>The robot application to use in the simulation job.</p>
    #[serde(rename = "robotApplications")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub robot_applications: Option<Vec<RobotApplicationConfig>>,
    /// <p>The simulation application to use in the simulation job.</p>
    #[serde(rename = "simulationApplications")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub simulation_applications: Option<Vec<SimulationApplicationConfig>>,
    /// <p>A map that contains tag keys and tag values that are attached to the simulation job.</p>
    #[serde(rename = "tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<::std::collections::HashMap<String, String>>,
    /// <p>If your simulation job accesses resources in a VPC, you provide this parameter identifying the list of security group IDs and subnet IDs. These must belong to the same VPC. You must provide at least one security group and one subnet ID. </p>
    #[serde(rename = "vpcConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_config: Option<VPCConfig>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateSimulationJobResponse {
    /// <p>The Amazon Resource Name (ARN) of the simulation job.</p>
    #[serde(rename = "arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p>Unique, case-sensitive identifier that you provide to ensure the idempotency of the request.</p>
    #[serde(rename = "clientRequestToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_request_token: Option<String>,
    /// <p>Compute information for the simulation job.</p>
    #[serde(rename = "compute")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compute: Option<ComputeResponse>,
    /// <p>The data sources for the simulation job.</p>
    #[serde(rename = "dataSources")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_sources: Option<Vec<DataSource>>,
    /// <p>the failure behavior for the simulation job.</p>
    #[serde(rename = "failureBehavior")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_behavior: Option<String>,
    /// <p><p>The failure code of the simulation job if it failed:</p> <dl> <dt>InternalServiceError</dt> <dd> <p>Internal service error.</p> </dd> <dt>RobotApplicationCrash</dt> <dd> <p>Robot application exited abnormally.</p> </dd> <dt>SimulationApplicationCrash</dt> <dd> <p> Simulation application exited abnormally.</p> </dd> <dt>BadPermissionsRobotApplication</dt> <dd> <p>Robot application bundle could not be downloaded.</p> </dd> <dt>BadPermissionsSimulationApplication</dt> <dd> <p>Simulation application bundle could not be downloaded.</p> </dd> <dt>BadPermissionsS3Output</dt> <dd> <p>Unable to publish outputs to customer-provided S3 bucket.</p> </dd> <dt>BadPermissionsCloudwatchLogs</dt> <dd> <p>Unable to publish logs to customer-provided CloudWatch Logs resource.</p> </dd> <dt>SubnetIpLimitExceeded</dt> <dd> <p>Subnet IP limit exceeded.</p> </dd> <dt>ENILimitExceeded</dt> <dd> <p>ENI limit exceeded.</p> </dd> <dt>BadPermissionsUserCredentials</dt> <dd> <p>Unable to use the Role provided.</p> </dd> <dt>InvalidBundleRobotApplication</dt> <dd> <p>Robot bundle cannot be extracted (invalid format, bundling error, or other issue).</p> </dd> <dt>InvalidBundleSimulationApplication</dt> <dd> <p>Simulation bundle cannot be extracted (invalid format, bundling error, or other issue).</p> </dd> <dt>RobotApplicationVersionMismatchedEtag</dt> <dd> <p>Etag for RobotApplication does not match value during version creation.</p> </dd> <dt>SimulationApplicationVersionMismatchedEtag</dt> <dd> <p>Etag for SimulationApplication does not match value during version creation.</p> </dd> </dl></p>
    #[serde(rename = "failureCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_code: Option<String>,
    /// <p>The IAM role that allows the simulation job to call the AWS APIs that are specified in its associated policies on your behalf.</p>
    #[serde(rename = "iamRole")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iam_role: Option<String>,
    /// <p>The time, in milliseconds since the epoch, when the simulation job was last started.</p>
    #[serde(rename = "lastStartedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_started_at: Option<f64>,
    /// <p>The time, in milliseconds since the epoch, when the simulation job was last updated.</p>
    #[serde(rename = "lastUpdatedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_at: Option<f64>,
    /// <p>The logging configuration.</p>
    #[serde(rename = "loggingConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logging_config: Option<LoggingConfig>,
    /// <p>The maximum simulation job duration in seconds. </p>
    #[serde(rename = "maxJobDurationInSeconds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_job_duration_in_seconds: Option<i64>,
    /// <p>Simulation job output files location.</p>
    #[serde(rename = "outputLocation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_location: Option<OutputLocation>,
    /// <p>The robot application used by the simulation job.</p>
    #[serde(rename = "robotApplications")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub robot_applications: Option<Vec<RobotApplicationConfig>>,
    /// <p>The simulation application used by the simulation job.</p>
    #[serde(rename = "simulationApplications")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub simulation_applications: Option<Vec<SimulationApplicationConfig>>,
    /// <p>The simulation job execution duration in milliseconds.</p>
    #[serde(rename = "simulationTimeMillis")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub simulation_time_millis: Option<i64>,
    /// <p>The status of the simulation job.</p>
    #[serde(rename = "status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// <p>The list of all tags added to the simulation job.</p>
    #[serde(rename = "tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<::std::collections::HashMap<String, String>>,
    /// <p>Information about the vpc configuration.</p>
    #[serde(rename = "vpcConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_config: Option<VPCConfigResponse>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateWorldExportJobRequest {
    /// <p>Unique, case-sensitive identifier that you provide to ensure the idempotency of the request.</p>
    #[serde(rename = "clientRequestToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_request_token: Option<String>,
    /// <p>The IAM role that the world export process uses to access the Amazon S3 bucket and put the export.</p>
    #[serde(rename = "iamRole")]
    pub iam_role: String,
    #[serde(rename = "outputLocation")]
    pub output_location: OutputLocation,
    /// <p>A map that contains tag keys and tag values that are attached to the world export job.</p>
    #[serde(rename = "tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<::std::collections::HashMap<String, String>>,
    /// <p>A list of Amazon Resource Names (arns) that correspond to worlds to export.</p>
    #[serde(rename = "worlds")]
    pub worlds: Vec<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateWorldExportJobResponse {
    /// <p>The Amazon Resource Name (ARN) of the world export job.</p>
    #[serde(rename = "arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p>Unique, case-sensitive identifier that you provide to ensure the idempotency of the request.</p>
    #[serde(rename = "clientRequestToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_request_token: Option<String>,
    /// <p>The time, in milliseconds since the epoch, when the world export job was created.</p>
    #[serde(rename = "createdAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<f64>,
    /// <p>The failure code of the world export job if it failed:</p> <dl> <dt>InternalServiceError</dt> <dd> <p>Internal service error.</p> </dd> <dt>LimitExceeded</dt> <dd> <p>The requested resource exceeds the maximum number allowed, or the number of concurrent stream requests exceeds the maximum number allowed. </p> </dd> <dt>ResourceNotFound</dt> <dd> <p>The specified resource could not be found. </p> </dd> <dt>RequestThrottled</dt> <dd> <p>The request was throttled.</p> </dd> <dt>InvalidInput</dt> <dd> <p>An input parameter in the request is not valid.</p> </dd> <dt>AllWorldGenerationFailed</dt> <dd> <p>All of the worlds in the world generation job failed. This can happen if your <code>worldCount</code> is greater than 50 or less than 1. </p> </dd> </dl> <p>For more information about troubleshooting WorldForge, see <a href="https://docs.aws.amazon.com/robomaker/latest/dg/troubleshooting-worldforge.html">Troubleshooting Simulation WorldForge</a>. </p>
    #[serde(rename = "failureCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_code: Option<String>,
    /// <p>The IAM role that the world export process uses to access the Amazon S3 bucket and put the export. </p>
    #[serde(rename = "iamRole")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iam_role: Option<String>,
    #[serde(rename = "outputLocation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_location: Option<OutputLocation>,
    /// <p><p>The status of the world export job.</p> <dl> <dt>Pending</dt> <dd> <p>The world export job request is pending.</p> </dd> <dt>Running</dt> <dd> <p>The world export job is running. </p> </dd> <dt>Completed</dt> <dd> <p>The world export job completed. </p> </dd> <dt>Failed</dt> <dd> <p>The world export job failed. See <code>failureCode</code> for more information. </p> </dd> <dt>Canceled</dt> <dd> <p>The world export job was cancelled.</p> </dd> <dt>Canceling</dt> <dd> <p>The world export job is being cancelled.</p> </dd> </dl></p>
    #[serde(rename = "status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// <p>A map that contains tag keys and tag values that are attached to the world export job.</p>
    #[serde(rename = "tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<::std::collections::HashMap<String, String>>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateWorldGenerationJobRequest {
    /// <p>Unique, case-sensitive identifier that you provide to ensure the idempotency of the request.</p>
    #[serde(rename = "clientRequestToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_request_token: Option<String>,
    /// <p>A map that contains tag keys and tag values that are attached to the world generator job.</p>
    #[serde(rename = "tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<::std::collections::HashMap<String, String>>,
    /// <p>The Amazon Resource Name (arn) of the world template describing the worlds you want to create.</p>
    #[serde(rename = "template")]
    pub template: String,
    /// <p>Information about the world count.</p>
    #[serde(rename = "worldCount")]
    pub world_count: WorldCount,
    /// <p>A map that contains tag keys and tag values that are attached to the generated worlds.</p>
    #[serde(rename = "worldTags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub world_tags: Option<::std::collections::HashMap<String, String>>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateWorldGenerationJobResponse {
    /// <p>The Amazon Resource Name (ARN) of the world generator job.</p>
    #[serde(rename = "arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p>Unique, case-sensitive identifier that you provide to ensure the idempotency of the request.</p>
    #[serde(rename = "clientRequestToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_request_token: Option<String>,
    /// <p>The time, in milliseconds since the epoch, when the world generator job was created.</p>
    #[serde(rename = "createdAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<f64>,
    /// <p><p>The failure code of the world generator job if it failed:</p> <dl> <dt>InternalServiceError</dt> <dd> <p>Internal service error.</p> </dd> <dt>LimitExceeded</dt> <dd> <p>The requested resource exceeds the maximum number allowed, or the number of concurrent stream requests exceeds the maximum number allowed. </p> </dd> <dt>ResourceNotFound</dt> <dd> <p>The specified resource could not be found. </p> </dd> <dt>RequestThrottled</dt> <dd> <p>The request was throttled.</p> </dd> <dt>InvalidInput</dt> <dd> <p>An input parameter in the request is not valid.</p> </dd> </dl></p>
    #[serde(rename = "failureCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_code: Option<String>,
    /// <p><p>The status of the world generator job.</p> <dl> <dt>Pending</dt> <dd> <p>The world generator job request is pending.</p> </dd> <dt>Running</dt> <dd> <p>The world generator job is running. </p> </dd> <dt>Completed</dt> <dd> <p>The world generator job completed. </p> </dd> <dt>Failed</dt> <dd> <p>The world generator job failed. See <code>failureCode</code> for more information. </p> </dd> <dt>PartialFailed</dt> <dd> <p>Some worlds did not generate.</p> </dd> <dt>Canceled</dt> <dd> <p>The world generator job was cancelled.</p> </dd> <dt>Canceling</dt> <dd> <p>The world generator job is being cancelled.</p> </dd> </dl></p>
    #[serde(rename = "status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// <p>A map that contains tag keys and tag values that are attached to the world generator job.</p>
    #[serde(rename = "tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<::std::collections::HashMap<String, String>>,
    /// <p>The Amazon Resource Name (arn) of the world template.</p>
    #[serde(rename = "template")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template: Option<String>,
    /// <p>Information about the world count. </p>
    #[serde(rename = "worldCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub world_count: Option<WorldCount>,
    /// <p>A map that contains tag keys and tag values that are attached to the generated worlds.</p>
    #[serde(rename = "worldTags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub world_tags: Option<::std::collections::HashMap<String, String>>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateWorldTemplateRequest {
    /// <p>Unique, case-sensitive identifier that you provide to ensure the idempotency of the request.</p>
    #[serde(rename = "clientRequestToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_request_token: Option<String>,
    /// <p>The name of the world template.</p>
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>A map that contains tag keys and tag values that are attached to the world template.</p>
    #[serde(rename = "tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<::std::collections::HashMap<String, String>>,
    /// <p>The world template body.</p>
    #[serde(rename = "templateBody")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template_body: Option<String>,
    /// <p>The location of the world template.</p>
    #[serde(rename = "templateLocation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template_location: Option<TemplateLocation>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateWorldTemplateResponse {
    /// <p>The Amazon Resource Name (ARN) of the world template.</p>
    #[serde(rename = "arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p>Unique, case-sensitive identifier that you provide to ensure the idempotency of the request.</p>
    #[serde(rename = "clientRequestToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_request_token: Option<String>,
    /// <p>The time, in milliseconds since the epoch, when the world template was created.</p>
    #[serde(rename = "createdAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<f64>,
    /// <p>The name of the world template.</p>
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>A map that contains tag keys and tag values that are attached to the world template.</p>
    #[serde(rename = "tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<::std::collections::HashMap<String, String>>,
}

/// <p>Information about a data source.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DataSource {
    /// <p>The name of the data source.</p>
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The S3 bucket where the data files are located.</p>
    #[serde(rename = "s3Bucket")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s_3_bucket: Option<String>,
    /// <p>The list of S3 keys identifying the data source files.</p>
    #[serde(rename = "s3Keys")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s_3_keys: Option<Vec<S3KeyOutput>>,
}

/// <p>Information about a data source.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct DataSourceConfig {
    /// <p>The name of the data source.</p>
    #[serde(rename = "name")]
    pub name: String,
    /// <p>The S3 bucket where the data files are located.</p>
    #[serde(rename = "s3Bucket")]
    pub s_3_bucket: String,
    /// <p>The list of S3 keys identifying the data source files.</p>
    #[serde(rename = "s3Keys")]
    pub s_3_keys: Vec<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteFleetRequest {
    /// <p>The Amazon Resource Name (ARN) of the fleet.</p>
    #[serde(rename = "fleet")]
    pub fleet: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DeleteFleetResponse {}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteRobotApplicationRequest {
    /// <p>The Amazon Resource Name (ARN) of the the robot application.</p>
    #[serde(rename = "application")]
    pub application: String,
    /// <p>The version of the robot application to delete.</p>
    #[serde(rename = "applicationVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_version: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DeleteRobotApplicationResponse {}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteRobotRequest {
    /// <p>The Amazon Resource Name (ARN) of the robot.</p>
    #[serde(rename = "robot")]
    pub robot: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DeleteRobotResponse {}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteSimulationApplicationRequest {
    /// <p>The application information for the simulation application to delete.</p>
    #[serde(rename = "application")]
    pub application: String,
    /// <p>The version of the simulation application to delete.</p>
    #[serde(rename = "applicationVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_version: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DeleteSimulationApplicationResponse {}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteWorldTemplateRequest {
    /// <p>The Amazon Resource Name (arn) of the world template you want to delete.</p>
    #[serde(rename = "template")]
    pub template: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DeleteWorldTemplateResponse {}

/// <p>Information about a deployment application configuration.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct DeploymentApplicationConfig {
    /// <p>The Amazon Resource Name (ARN) of the robot application.</p>
    #[serde(rename = "application")]
    pub application: String,
    /// <p>The version of the application.</p>
    #[serde(rename = "applicationVersion")]
    pub application_version: String,
    /// <p>The launch configuration.</p>
    #[serde(rename = "launchConfig")]
    pub launch_config: DeploymentLaunchConfig,
}

/// <p>Information about a deployment configuration.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct DeploymentConfig {
    /// <p>The percentage of robots receiving the deployment at the same time.</p>
    #[serde(rename = "concurrentDeploymentPercentage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub concurrent_deployment_percentage: Option<i64>,
    /// <p>The download condition file.</p>
    #[serde(rename = "downloadConditionFile")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub download_condition_file: Option<S3Object>,
    /// <p>The percentage of deployments that need to fail before stopping deployment.</p>
    #[serde(rename = "failureThresholdPercentage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_threshold_percentage: Option<i64>,
    /// <p>The amount of time, in seconds, to wait for deployment to a single robot to complete. Choose a time between 1 minute and 7 days. The default is 5 hours.</p>
    #[serde(rename = "robotDeploymentTimeoutInSeconds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub robot_deployment_timeout_in_seconds: Option<i64>,
}

/// <p>Information about a deployment job.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DeploymentJob {
    /// <p>The Amazon Resource Name (ARN) of the deployment job.</p>
    #[serde(rename = "arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p>The time, in milliseconds since the epoch, when the deployment job was created.</p>
    #[serde(rename = "createdAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<f64>,
    /// <p>The deployment application configuration.</p>
    #[serde(rename = "deploymentApplicationConfigs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deployment_application_configs: Option<Vec<DeploymentApplicationConfig>>,
    /// <p>The deployment configuration.</p>
    #[serde(rename = "deploymentConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deployment_config: Option<DeploymentConfig>,
    /// <p>The deployment job failure code.</p>
    #[serde(rename = "failureCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_code: Option<String>,
    /// <p>A short description of the reason why the deployment job failed.</p>
    #[serde(rename = "failureReason")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_reason: Option<String>,
    /// <p>The Amazon Resource Name (ARN) of the fleet.</p>
    #[serde(rename = "fleet")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fleet: Option<String>,
    /// <p>The status of the deployment job.</p>
    #[serde(rename = "status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

/// <p>Configuration information for a deployment launch.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct DeploymentLaunchConfig {
    /// <p>An array of key/value pairs specifying environment variables for the robot application</p>
    #[serde(rename = "environmentVariables")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub environment_variables: Option<::std::collections::HashMap<String, String>>,
    /// <p>The launch file name.</p>
    #[serde(rename = "launchFile")]
    pub launch_file: String,
    /// <p>The package name.</p>
    #[serde(rename = "packageName")]
    pub package_name: String,
    /// <p>The deployment post-launch file. This file will be executed after the launch file.</p>
    #[serde(rename = "postLaunchFile")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub post_launch_file: Option<String>,
    /// <p>The deployment pre-launch file. This file will be executed prior to the launch file.</p>
    #[serde(rename = "preLaunchFile")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pre_launch_file: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeregisterRobotRequest {
    /// <p>The Amazon Resource Name (ARN) of the fleet.</p>
    #[serde(rename = "fleet")]
    pub fleet: String,
    /// <p>The Amazon Resource Name (ARN) of the robot.</p>
    #[serde(rename = "robot")]
    pub robot: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DeregisterRobotResponse {
    /// <p>The Amazon Resource Name (ARN) of the fleet.</p>
    #[serde(rename = "fleet")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fleet: Option<String>,
    /// <p>The Amazon Resource Name (ARN) of the robot.</p>
    #[serde(rename = "robot")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub robot: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeDeploymentJobRequest {
    /// <p>The Amazon Resource Name (ARN) of the deployment job.</p>
    #[serde(rename = "job")]
    pub job: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeDeploymentJobResponse {
    /// <p>The Amazon Resource Name (ARN) of the deployment job.</p>
    #[serde(rename = "arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p>The time, in milliseconds since the epoch, when the deployment job was created.</p>
    #[serde(rename = "createdAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<f64>,
    /// <p>The deployment application configuration.</p>
    #[serde(rename = "deploymentApplicationConfigs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deployment_application_configs: Option<Vec<DeploymentApplicationConfig>>,
    /// <p>The deployment configuration.</p>
    #[serde(rename = "deploymentConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deployment_config: Option<DeploymentConfig>,
    /// <p>The deployment job failure code.</p>
    #[serde(rename = "failureCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_code: Option<String>,
    /// <p>A short description of the reason why the deployment job failed.</p>
    #[serde(rename = "failureReason")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_reason: Option<String>,
    /// <p>The Amazon Resource Name (ARN) of the fleet.</p>
    #[serde(rename = "fleet")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fleet: Option<String>,
    /// <p>A list of robot deployment summaries.</p>
    #[serde(rename = "robotDeploymentSummary")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub robot_deployment_summary: Option<Vec<RobotDeployment>>,
    /// <p>The status of the deployment job.</p>
    #[serde(rename = "status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// <p>The list of all tags added to the specified deployment job.</p>
    #[serde(rename = "tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<::std::collections::HashMap<String, String>>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeFleetRequest {
    /// <p>The Amazon Resource Name (ARN) of the fleet.</p>
    #[serde(rename = "fleet")]
    pub fleet: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeFleetResponse {
    /// <p>The Amazon Resource Name (ARN) of the fleet.</p>
    #[serde(rename = "arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p>The time, in milliseconds since the epoch, when the fleet was created.</p>
    #[serde(rename = "createdAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<f64>,
    /// <p>The Amazon Resource Name (ARN) of the last deployment job.</p>
    #[serde(rename = "lastDeploymentJob")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_deployment_job: Option<String>,
    /// <p>The status of the last deployment.</p>
    #[serde(rename = "lastDeploymentStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_deployment_status: Option<String>,
    /// <p>The time of the last deployment.</p>
    #[serde(rename = "lastDeploymentTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_deployment_time: Option<f64>,
    /// <p>The name of the fleet.</p>
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>A list of robots.</p>
    #[serde(rename = "robots")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub robots: Option<Vec<Robot>>,
    /// <p>The list of all tags added to the specified fleet.</p>
    #[serde(rename = "tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<::std::collections::HashMap<String, String>>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeRobotApplicationRequest {
    /// <p>The Amazon Resource Name (ARN) of the robot application.</p>
    #[serde(rename = "application")]
    pub application: String,
    /// <p>The version of the robot application to describe.</p>
    #[serde(rename = "applicationVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_version: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeRobotApplicationResponse {
    /// <p>The Amazon Resource Name (ARN) of the robot application.</p>
    #[serde(rename = "arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p>The time, in milliseconds since the epoch, when the robot application was last updated.</p>
    #[serde(rename = "lastUpdatedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_at: Option<f64>,
    /// <p>The name of the robot application.</p>
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The revision id of the robot application.</p>
    #[serde(rename = "revisionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub revision_id: Option<String>,
    /// <p>The robot software suite (ROS distribution) used by the robot application.</p>
    #[serde(rename = "robotSoftwareSuite")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub robot_software_suite: Option<RobotSoftwareSuite>,
    /// <p>The sources of the robot application.</p>
    #[serde(rename = "sources")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sources: Option<Vec<Source>>,
    /// <p>The list of all tags added to the specified robot application.</p>
    #[serde(rename = "tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<::std::collections::HashMap<String, String>>,
    /// <p>The version of the robot application.</p>
    #[serde(rename = "version")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeRobotRequest {
    /// <p>The Amazon Resource Name (ARN) of the robot to be described.</p>
    #[serde(rename = "robot")]
    pub robot: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeRobotResponse {
    /// <p>The target architecture of the robot application.</p>
    #[serde(rename = "architecture")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub architecture: Option<String>,
    /// <p>The Amazon Resource Name (ARN) of the robot.</p>
    #[serde(rename = "arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p>The time, in milliseconds since the epoch, when the robot was created.</p>
    #[serde(rename = "createdAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<f64>,
    /// <p>The Amazon Resource Name (ARN) of the fleet.</p>
    #[serde(rename = "fleetArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fleet_arn: Option<String>,
    /// <p>The Greengrass group id.</p>
    #[serde(rename = "greengrassGroupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub greengrass_group_id: Option<String>,
    /// <p>The Amazon Resource Name (ARN) of the last deployment job.</p>
    #[serde(rename = "lastDeploymentJob")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_deployment_job: Option<String>,
    /// <p>The time of the last deployment job.</p>
    #[serde(rename = "lastDeploymentTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_deployment_time: Option<f64>,
    /// <p>The name of the robot.</p>
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The status of the fleet.</p>
    #[serde(rename = "status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// <p>The list of all tags added to the specified robot.</p>
    #[serde(rename = "tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<::std::collections::HashMap<String, String>>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeSimulationApplicationRequest {
    /// <p>The application information for the simulation application.</p>
    #[serde(rename = "application")]
    pub application: String,
    /// <p>The version of the simulation application to describe.</p>
    #[serde(rename = "applicationVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_version: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeSimulationApplicationResponse {
    /// <p>The Amazon Resource Name (ARN) of the robot simulation application.</p>
    #[serde(rename = "arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p>The time, in milliseconds since the epoch, when the simulation application was last updated.</p>
    #[serde(rename = "lastUpdatedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_at: Option<f64>,
    /// <p>The name of the simulation application.</p>
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The rendering engine for the simulation application.</p>
    #[serde(rename = "renderingEngine")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rendering_engine: Option<RenderingEngine>,
    /// <p>The revision id of the simulation application.</p>
    #[serde(rename = "revisionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub revision_id: Option<String>,
    /// <p>Information about the robot software suite (ROS distribution).</p>
    #[serde(rename = "robotSoftwareSuite")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub robot_software_suite: Option<RobotSoftwareSuite>,
    /// <p>The simulation software suite used by the simulation application.</p>
    #[serde(rename = "simulationSoftwareSuite")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub simulation_software_suite: Option<SimulationSoftwareSuite>,
    /// <p>The sources of the simulation application.</p>
    #[serde(rename = "sources")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sources: Option<Vec<Source>>,
    /// <p>The list of all tags added to the specified simulation application.</p>
    #[serde(rename = "tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<::std::collections::HashMap<String, String>>,
    /// <p>The version of the simulation application.</p>
    #[serde(rename = "version")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeSimulationJobBatchRequest {
    /// <p>The id of the batch to describe.</p>
    #[serde(rename = "batch")]
    pub batch: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeSimulationJobBatchResponse {
    /// <p>The Amazon Resource Name (ARN) of the batch.</p>
    #[serde(rename = "arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p>The batch policy.</p>
    #[serde(rename = "batchPolicy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub batch_policy: Option<BatchPolicy>,
    /// <p>Unique, case-sensitive identifier that you provide to ensure the idempotency of the request.</p>
    #[serde(rename = "clientRequestToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_request_token: Option<String>,
    /// <p>The time, in milliseconds since the epoch, when the simulation job batch was created.</p>
    #[serde(rename = "createdAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<f64>,
    /// <p>A list of created simulation job summaries.</p>
    #[serde(rename = "createdRequests")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_requests: Option<Vec<SimulationJobSummary>>,
    /// <p>A list of failed create simulation job requests. The request failed to be created into a simulation job. Failed requests do not have a simulation job ID. </p>
    #[serde(rename = "failedRequests")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failed_requests: Option<Vec<FailedCreateSimulationJobRequest>>,
    /// <p>The failure code of the simulation job batch.</p>
    #[serde(rename = "failureCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_code: Option<String>,
    /// <p>The reason the simulation job batch failed.</p>
    #[serde(rename = "failureReason")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_reason: Option<String>,
    /// <p>The time, in milliseconds since the epoch, when the simulation job batch was last updated.</p>
    #[serde(rename = "lastUpdatedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_at: Option<f64>,
    /// <p>A list of pending simulation job requests. These requests have not yet been created into simulation jobs.</p>
    #[serde(rename = "pendingRequests")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pending_requests: Option<Vec<SimulationJobRequest>>,
    /// <p><p>The status of the batch.</p> <dl> <dt>Pending</dt> <dd> <p>The simulation job batch request is pending.</p> </dd> <dt>InProgress</dt> <dd> <p>The simulation job batch is in progress. </p> </dd> <dt>Failed</dt> <dd> <p>The simulation job batch failed. One or more simulation job requests could not be completed due to an internal failure (like <code>InternalServiceError</code>). See <code>failureCode</code> and <code>failureReason</code> for more information.</p> </dd> <dt>Completed</dt> <dd> <p>The simulation batch job completed. A batch is complete when (1) there are no pending simulation job requests in the batch and none of the failed simulation job requests are due to <code>InternalServiceError</code> and (2) when all created simulation jobs have reached a terminal state (for example, <code>Completed</code> or <code>Failed</code>). </p> </dd> <dt>Canceled</dt> <dd> <p>The simulation batch job was cancelled.</p> </dd> <dt>Canceling</dt> <dd> <p>The simulation batch job is being cancelled.</p> </dd> <dt>Completing</dt> <dd> <p>The simulation batch job is completing.</p> </dd> <dt>TimingOut</dt> <dd> <p>The simulation job batch is timing out.</p> <p>If a batch timing out, and there are pending requests that were failing due to an internal failure (like <code>InternalServiceError</code>), the batch status will be <code>Failed</code>. If there are no such failing request, the batch status will be <code>TimedOut</code>. </p> </dd> <dt>TimedOut</dt> <dd> <p>The simulation batch job timed out.</p> </dd> </dl></p>
    #[serde(rename = "status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// <p>A map that contains tag keys and tag values that are attached to the simulation job batch.</p>
    #[serde(rename = "tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<::std::collections::HashMap<String, String>>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeSimulationJobRequest {
    /// <p>The Amazon Resource Name (ARN) of the simulation job to be described.</p>
    #[serde(rename = "job")]
    pub job: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeSimulationJobResponse {
    /// <p>The Amazon Resource Name (ARN) of the simulation job.</p>
    #[serde(rename = "arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p>Unique, case-sensitive identifier that you provide to ensure the idempotency of the request.</p>
    #[serde(rename = "clientRequestToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_request_token: Option<String>,
    /// <p>Compute information for the simulation job.</p>
    #[serde(rename = "compute")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compute: Option<ComputeResponse>,
    /// <p>The data sources for the simulation job.</p>
    #[serde(rename = "dataSources")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_sources: Option<Vec<DataSource>>,
    /// <p>The failure behavior for the simulation job.</p>
    #[serde(rename = "failureBehavior")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_behavior: Option<String>,
    /// <p><p>The failure code of the simulation job if it failed:</p> <dl> <dt>InternalServiceError</dt> <dd> <p>Internal service error.</p> </dd> <dt>RobotApplicationCrash</dt> <dd> <p>Robot application exited abnormally.</p> </dd> <dt>SimulationApplicationCrash</dt> <dd> <p> Simulation application exited abnormally.</p> </dd> <dt>BadPermissionsRobotApplication</dt> <dd> <p>Robot application bundle could not be downloaded.</p> </dd> <dt>BadPermissionsSimulationApplication</dt> <dd> <p>Simulation application bundle could not be downloaded.</p> </dd> <dt>BadPermissionsS3Output</dt> <dd> <p>Unable to publish outputs to customer-provided S3 bucket.</p> </dd> <dt>BadPermissionsCloudwatchLogs</dt> <dd> <p>Unable to publish logs to customer-provided CloudWatch Logs resource.</p> </dd> <dt>SubnetIpLimitExceeded</dt> <dd> <p>Subnet IP limit exceeded.</p> </dd> <dt>ENILimitExceeded</dt> <dd> <p>ENI limit exceeded.</p> </dd> <dt>BadPermissionsUserCredentials</dt> <dd> <p>Unable to use the Role provided.</p> </dd> <dt>InvalidBundleRobotApplication</dt> <dd> <p>Robot bundle cannot be extracted (invalid format, bundling error, or other issue).</p> </dd> <dt>InvalidBundleSimulationApplication</dt> <dd> <p>Simulation bundle cannot be extracted (invalid format, bundling error, or other issue).</p> </dd> <dt>RobotApplicationVersionMismatchedEtag</dt> <dd> <p>Etag for RobotApplication does not match value during version creation.</p> </dd> <dt>SimulationApplicationVersionMismatchedEtag</dt> <dd> <p>Etag for SimulationApplication does not match value during version creation.</p> </dd> </dl></p>
    #[serde(rename = "failureCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_code: Option<String>,
    /// <p>Details about why the simulation job failed. For more information about troubleshooting, see <a href="https://docs.aws.amazon.com/robomaker/latest/dg/troubleshooting.html">Troubleshooting</a>.</p>
    #[serde(rename = "failureReason")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_reason: Option<String>,
    /// <p>The IAM role that allows the simulation instance to call the AWS APIs that are specified in its associated policies on your behalf.</p>
    #[serde(rename = "iamRole")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iam_role: Option<String>,
    /// <p>The time, in milliseconds since the epoch, when the simulation job was last started.</p>
    #[serde(rename = "lastStartedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_started_at: Option<f64>,
    /// <p>The time, in milliseconds since the epoch, when the simulation job was last updated.</p>
    #[serde(rename = "lastUpdatedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_at: Option<f64>,
    /// <p>The logging configuration.</p>
    #[serde(rename = "loggingConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logging_config: Option<LoggingConfig>,
    /// <p>The maximum job duration in seconds. The value must be 8 days (691,200 seconds) or less.</p>
    #[serde(rename = "maxJobDurationInSeconds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_job_duration_in_seconds: Option<i64>,
    /// <p>The name of the simulation job.</p>
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The network interface information for the simulation job.</p>
    #[serde(rename = "networkInterface")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_interface: Option<NetworkInterface>,
    /// <p>Location for output files generated by the simulation job.</p>
    #[serde(rename = "outputLocation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_location: Option<OutputLocation>,
    /// <p>A list of robot applications.</p>
    #[serde(rename = "robotApplications")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub robot_applications: Option<Vec<RobotApplicationConfig>>,
    /// <p>A list of simulation applications.</p>
    #[serde(rename = "simulationApplications")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub simulation_applications: Option<Vec<SimulationApplicationConfig>>,
    /// <p>The simulation job execution duration in milliseconds.</p>
    #[serde(rename = "simulationTimeMillis")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub simulation_time_millis: Option<i64>,
    /// <p>The status of the simulation job.</p>
    #[serde(rename = "status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// <p>The list of all tags added to the specified simulation job.</p>
    #[serde(rename = "tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<::std::collections::HashMap<String, String>>,
    /// <p>The VPC configuration.</p>
    #[serde(rename = "vpcConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_config: Option<VPCConfigResponse>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeWorldExportJobRequest {
    /// <p>The Amazon Resource Name (arn) of the world export job to describe.</p>
    #[serde(rename = "job")]
    pub job: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeWorldExportJobResponse {
    /// <p>The Amazon Resource Name (ARN) of the world export job.</p>
    #[serde(rename = "arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p>Unique, case-sensitive identifier that you provide to ensure the idempotency of the request.</p>
    #[serde(rename = "clientRequestToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_request_token: Option<String>,
    /// <p>The time, in milliseconds since the epoch, when the world export job was created.</p>
    #[serde(rename = "createdAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<f64>,
    /// <p><p>The failure code of the world export job if it failed:</p> <dl> <dt>InternalServiceError</dt> <dd> <p>Internal service error.</p> </dd> <dt>LimitExceeded</dt> <dd> <p>The requested resource exceeds the maximum number allowed, or the number of concurrent stream requests exceeds the maximum number allowed. </p> </dd> <dt>ResourceNotFound</dt> <dd> <p>The specified resource could not be found. </p> </dd> <dt>RequestThrottled</dt> <dd> <p>The request was throttled.</p> </dd> <dt>InvalidInput</dt> <dd> <p>An input parameter in the request is not valid.</p> </dd> </dl></p>
    #[serde(rename = "failureCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_code: Option<String>,
    /// <p>The reason why the world export job failed.</p>
    #[serde(rename = "failureReason")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_reason: Option<String>,
    /// <p>The IAM role that the world export process uses to access the Amazon S3 bucket and put the export.</p>
    #[serde(rename = "iamRole")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iam_role: Option<String>,
    #[serde(rename = "outputLocation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_location: Option<OutputLocation>,
    /// <p><p>The status of the world export job.</p> <dl> <dt>Pending</dt> <dd> <p>The world export job request is pending.</p> </dd> <dt>Running</dt> <dd> <p>The world export job is running. </p> </dd> <dt>Completed</dt> <dd> <p>The world export job completed. </p> </dd> <dt>Failed</dt> <dd> <p>The world export job failed. See <code>failureCode</code> and <code>failureReason</code> for more information. </p> </dd> <dt>Canceled</dt> <dd> <p>The world export job was cancelled.</p> </dd> <dt>Canceling</dt> <dd> <p>The world export job is being cancelled.</p> </dd> </dl></p>
    #[serde(rename = "status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// <p>A map that contains tag keys and tag values that are attached to the world export job.</p>
    #[serde(rename = "tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<::std::collections::HashMap<String, String>>,
    /// <p>A list of Amazon Resource Names (arns) that correspond to worlds to be exported.</p>
    #[serde(rename = "worlds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub worlds: Option<Vec<String>>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeWorldGenerationJobRequest {
    /// <p>The Amazon Resource Name (arn) of the world generation job to describe.</p>
    #[serde(rename = "job")]
    pub job: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeWorldGenerationJobResponse {
    /// <p>The Amazon Resource Name (ARN) of the world generation job.</p>
    #[serde(rename = "arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p>Unique, case-sensitive identifier that you provide to ensure the idempotency of the request.</p>
    #[serde(rename = "clientRequestToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_request_token: Option<String>,
    /// <p>The time, in milliseconds since the epoch, when the world generation job was created.</p>
    #[serde(rename = "createdAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<f64>,
    /// <p><p>The failure code of the world generation job if it failed:</p> <dl> <dt>InternalServiceError</dt> <dd> <p>Internal service error.</p> </dd> <dt>LimitExceeded</dt> <dd> <p>The requested resource exceeds the maximum number allowed, or the number of concurrent stream requests exceeds the maximum number allowed. </p> </dd> <dt>ResourceNotFound</dt> <dd> <p>The specified resource could not be found. </p> </dd> <dt>RequestThrottled</dt> <dd> <p>The request was throttled.</p> </dd> <dt>InvalidInput</dt> <dd> <p>An input parameter in the request is not valid.</p> </dd> </dl></p>
    #[serde(rename = "failureCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_code: Option<String>,
    /// <p>The reason why the world generation job failed.</p>
    #[serde(rename = "failureReason")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_reason: Option<String>,
    /// <p>Summary information about finished worlds.</p>
    #[serde(rename = "finishedWorldsSummary")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub finished_worlds_summary: Option<FinishedWorldsSummary>,
    /// <p><p>The status of the world generation job:</p> <dl> <dt>Pending</dt> <dd> <p>The world generation job request is pending.</p> </dd> <dt>Running</dt> <dd> <p>The world generation job is running. </p> </dd> <dt>Completed</dt> <dd> <p>The world generation job completed. </p> </dd> <dt>Failed</dt> <dd> <p>The world generation job failed. See <code>failureCode</code> for more information. </p> </dd> <dt>PartialFailed</dt> <dd> <p>Some worlds did not generate.</p> </dd> <dt>Canceled</dt> <dd> <p>The world generation job was cancelled.</p> </dd> <dt>Canceling</dt> <dd> <p>The world generation job is being cancelled.</p> </dd> </dl></p>
    #[serde(rename = "status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// <p>A map that contains tag keys and tag values that are attached to the world generation job.</p>
    #[serde(rename = "tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<::std::collections::HashMap<String, String>>,
    /// <p>The Amazon Resource Name (arn) of the world template.</p>
    #[serde(rename = "template")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template: Option<String>,
    /// <p>Information about the world count.</p>
    #[serde(rename = "worldCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub world_count: Option<WorldCount>,
    /// <p>A map that contains tag keys and tag values that are attached to the generated worlds.</p>
    #[serde(rename = "worldTags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub world_tags: Option<::std::collections::HashMap<String, String>>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeWorldRequest {
    /// <p>The Amazon Resource Name (arn) of the world you want to describe.</p>
    #[serde(rename = "world")]
    pub world: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeWorldResponse {
    /// <p>The Amazon Resource Name (arn) of the world.</p>
    #[serde(rename = "arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p>The time, in milliseconds since the epoch, when the world was created.</p>
    #[serde(rename = "createdAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<f64>,
    /// <p>The Amazon Resource Name (arn) of the world generation job that generated the world.</p>
    #[serde(rename = "generationJob")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub generation_job: Option<String>,
    /// <p>A map that contains tag keys and tag values that are attached to the world.</p>
    #[serde(rename = "tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<::std::collections::HashMap<String, String>>,
    /// <p>The world template.</p>
    #[serde(rename = "template")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeWorldTemplateRequest {
    /// <p>The Amazon Resource Name (arn) of the world template you want to describe.</p>
    #[serde(rename = "template")]
    pub template: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeWorldTemplateResponse {
    /// <p>The Amazon Resource Name (ARN) of the world template.</p>
    #[serde(rename = "arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p>Unique, case-sensitive identifier that you provide to ensure the idempotency of the request.</p>
    #[serde(rename = "clientRequestToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_request_token: Option<String>,
    /// <p>The time, in milliseconds since the epoch, when the world template was created.</p>
    #[serde(rename = "createdAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<f64>,
    /// <p>The time, in milliseconds since the epoch, when the world template was last updated.</p>
    #[serde(rename = "lastUpdatedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_at: Option<f64>,
    /// <p>The name of the world template.</p>
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>A map that contains tag keys and tag values that are attached to the world template.</p>
    #[serde(rename = "tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<::std::collections::HashMap<String, String>>,
}

/// <p>Information about a failed create simulation job request.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct FailedCreateSimulationJobRequest {
    /// <p>The time, in milliseconds since the epoch, when the simulation job batch failed.</p>
    #[serde(rename = "failedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failed_at: Option<f64>,
    /// <p>The failure code.</p>
    #[serde(rename = "failureCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_code: Option<String>,
    /// <p>The failure reason of the simulation job request.</p>
    #[serde(rename = "failureReason")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_reason: Option<String>,
    /// <p>The simulation job request.</p>
    #[serde(rename = "request")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request: Option<SimulationJobRequest>,
}

/// <p>Information about worlds that failed.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct FailureSummary {
    /// <p>The worlds that failed.</p>
    #[serde(rename = "failures")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failures: Option<Vec<WorldFailure>>,
    /// <p>The total number of failures.</p>
    #[serde(rename = "totalFailureCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_failure_count: Option<i64>,
}

/// <p>Information about a filter.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct Filter {
    /// <p>The name of the filter.</p>
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>A list of values.</p>
    #[serde(rename = "values")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub values: Option<Vec<String>>,
}

/// <p>Information about worlds that finished.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct FinishedWorldsSummary {
    /// <p>Information about worlds that failed.</p>
    #[serde(rename = "failureSummary")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_summary: Option<FailureSummary>,
    /// <p>The total number of finished worlds.</p>
    #[serde(rename = "finishedCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub finished_count: Option<i64>,
    /// <p>A list of worlds that succeeded.</p>
    #[serde(rename = "succeededWorlds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub succeeded_worlds: Option<Vec<String>>,
}

/// <p>Information about a fleet.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct Fleet {
    /// <p>The Amazon Resource Name (ARN) of the fleet.</p>
    #[serde(rename = "arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p>The time, in milliseconds since the epoch, when the fleet was created.</p>
    #[serde(rename = "createdAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<f64>,
    /// <p>The Amazon Resource Name (ARN) of the last deployment job.</p>
    #[serde(rename = "lastDeploymentJob")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_deployment_job: Option<String>,
    /// <p>The status of the last fleet deployment.</p>
    #[serde(rename = "lastDeploymentStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_deployment_status: Option<String>,
    /// <p>The time of the last deployment.</p>
    #[serde(rename = "lastDeploymentTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_deployment_time: Option<f64>,
    /// <p>The name of the fleet.</p>
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetWorldTemplateBodyRequest {
    /// <p>The Amazon Resource Name (arn) of the world generator job.</p>
    #[serde(rename = "generationJob")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub generation_job: Option<String>,
    /// <p>The Amazon Resource Name (arn) of the world template.</p>
    #[serde(rename = "template")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetWorldTemplateBodyResponse {
    /// <p>The world template body.</p>
    #[serde(rename = "templateBody")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template_body: Option<String>,
}

/// <p>Information about a launch configuration.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct LaunchConfig {
    /// <p>The environment variables for the application launch.</p>
    #[serde(rename = "environmentVariables")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub environment_variables: Option<::std::collections::HashMap<String, String>>,
    /// <p>The launch file name.</p>
    #[serde(rename = "launchFile")]
    pub launch_file: String,
    /// <p>The package name.</p>
    #[serde(rename = "packageName")]
    pub package_name: String,
    /// <p>The port forwarding configuration.</p>
    #[serde(rename = "portForwardingConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub port_forwarding_config: Option<PortForwardingConfig>,
    /// <p>Boolean indicating whether a streaming session will be configured for the application. If <code>True</code>, AWS RoboMaker will configure a connection so you can interact with your application as it is running in the simulation. You must configure and luanch the component. It must have a graphical user interface. </p>
    #[serde(rename = "streamUI")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream_ui: Option<bool>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListDeploymentJobsRequest {
    /// <p>Optional filters to limit results.</p> <p>The filter names <code>status</code> and <code>fleetName</code> are supported. When filtering, you must use the complete value of the filtered item. You can use up to three filters, but they must be for the same named item. For example, if you are looking for items with the status <code>InProgress</code> or the status <code>Pending</code>.</p>
    #[serde(rename = "filters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<Vec<Filter>>,
    /// <p>When this parameter is used, <code>ListDeploymentJobs</code> only returns <code>maxResults</code> results in a single page along with a <code>nextToken</code> response element. The remaining results of the initial request can be seen by sending another <code>ListDeploymentJobs</code> request with the returned <code>nextToken</code> value. This value can be between 1 and 200. If this parameter is not used, then <code>ListDeploymentJobs</code> returns up to 200 results and a <code>nextToken</code> value if applicable. </p>
    #[serde(rename = "maxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>If the previous paginated request did not return all of the remaining results, the response object's <code>nextToken</code> parameter value is set to a token. To retrieve the next set of results, call <code>ListDeploymentJobs</code> again and assign that token to the request object's <code>nextToken</code> parameter. If there are no remaining results, the previous response object's NextToken parameter is set to null. </p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListDeploymentJobsResponse {
    /// <p>A list of deployment jobs that meet the criteria of the request.</p>
    #[serde(rename = "deploymentJobs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deployment_jobs: Option<Vec<DeploymentJob>>,
    /// <p>If the previous paginated request did not return all of the remaining results, the response object's <code>nextToken</code> parameter value is set to a token. To retrieve the next set of results, call <code>ListDeploymentJobs</code> again and assign that token to the request object's <code>nextToken</code> parameter. If there are no remaining results, the previous response object's NextToken parameter is set to null. </p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListFleetsRequest {
    /// <p>Optional filters to limit results.</p> <p>The filter name <code>name</code> is supported. When filtering, you must use the complete value of the filtered item. You can use up to three filters.</p>
    #[serde(rename = "filters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<Vec<Filter>>,
    /// <p>When this parameter is used, <code>ListFleets</code> only returns <code>maxResults</code> results in a single page along with a <code>nextToken</code> response element. The remaining results of the initial request can be seen by sending another <code>ListFleets</code> request with the returned <code>nextToken</code> value. This value can be between 1 and 200. If this parameter is not used, then <code>ListFleets</code> returns up to 200 results and a <code>nextToken</code> value if applicable. </p>
    #[serde(rename = "maxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p><p>If the previous paginated request did not return all of the remaining results, the response object&#39;s <code>nextToken</code> parameter value is set to a token. To retrieve the next set of results, call <code>ListFleets</code> again and assign that token to the request object&#39;s <code>nextToken</code> parameter. If there are no remaining results, the previous response object&#39;s NextToken parameter is set to null. </p> <note> <p>This token should be treated as an opaque identifier that is only used to retrieve the next items in a list and not for other programmatic purposes.</p> </note></p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListFleetsResponse {
    /// <p>A list of fleet details meeting the request criteria.</p>
    #[serde(rename = "fleetDetails")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fleet_details: Option<Vec<Fleet>>,
    /// <p>If the previous paginated request did not return all of the remaining results, the response object's <code>nextToken</code> parameter value is set to a token. To retrieve the next set of results, call <code>ListFleets</code> again and assign that token to the request object's <code>nextToken</code> parameter. If there are no remaining results, the previous response object's NextToken parameter is set to null. </p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListRobotApplicationsRequest {
    /// <p>Optional filters to limit results.</p> <p>The filter name <code>name</code> is supported. When filtering, you must use the complete value of the filtered item. You can use up to three filters.</p>
    #[serde(rename = "filters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<Vec<Filter>>,
    /// <p>When this parameter is used, <code>ListRobotApplications</code> only returns <code>maxResults</code> results in a single page along with a <code>nextToken</code> response element. The remaining results of the initial request can be seen by sending another <code>ListRobotApplications</code> request with the returned <code>nextToken</code> value. This value can be between 1 and 100. If this parameter is not used, then <code>ListRobotApplications</code> returns up to 100 results and a <code>nextToken</code> value if applicable. </p>
    #[serde(rename = "maxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>If the previous paginated request did not return all of the remaining results, the response object's <code>nextToken</code> parameter value is set to a token. To retrieve the next set of results, call <code>ListRobotApplications</code> again and assign that token to the request object's <code>nextToken</code> parameter. If there are no remaining results, the previous response object's NextToken parameter is set to null. </p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The version qualifier of the robot application.</p>
    #[serde(rename = "versionQualifier")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version_qualifier: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListRobotApplicationsResponse {
    /// <p>If the previous paginated request did not return all of the remaining results, the response object's <code>nextToken</code> parameter value is set to a token. To retrieve the next set of results, call <code>ListRobotApplications</code> again and assign that token to the request object's <code>nextToken</code> parameter. If there are no remaining results, the previous response object's NextToken parameter is set to null. </p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>A list of robot application summaries that meet the criteria of the request.</p>
    #[serde(rename = "robotApplicationSummaries")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub robot_application_summaries: Option<Vec<RobotApplicationSummary>>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListRobotsRequest {
    /// <p>Optional filters to limit results.</p> <p>The filter names <code>status</code> and <code>fleetName</code> are supported. When filtering, you must use the complete value of the filtered item. You can use up to three filters, but they must be for the same named item. For example, if you are looking for items with the status <code>Registered</code> or the status <code>Available</code>.</p>
    #[serde(rename = "filters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<Vec<Filter>>,
    /// <p>When this parameter is used, <code>ListRobots</code> only returns <code>maxResults</code> results in a single page along with a <code>nextToken</code> response element. The remaining results of the initial request can be seen by sending another <code>ListRobots</code> request with the returned <code>nextToken</code> value. This value can be between 1 and 200. If this parameter is not used, then <code>ListRobots</code> returns up to 200 results and a <code>nextToken</code> value if applicable. </p>
    #[serde(rename = "maxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>If the previous paginated request did not return all of the remaining results, the response object's <code>nextToken</code> parameter value is set to a token. To retrieve the next set of results, call <code>ListRobots</code> again and assign that token to the request object's <code>nextToken</code> parameter. If there are no remaining results, the previous response object's NextToken parameter is set to null. </p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListRobotsResponse {
    /// <p>If the previous paginated request did not return all of the remaining results, the response object's <code>nextToken</code> parameter value is set to a token. To retrieve the next set of results, call <code>ListRobots</code> again and assign that token to the request object's <code>nextToken</code> parameter. If there are no remaining results, the previous response object's NextToken parameter is set to null. </p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>A list of robots that meet the criteria of the request.</p>
    #[serde(rename = "robots")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub robots: Option<Vec<Robot>>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListSimulationApplicationsRequest {
    /// <p>Optional list of filters to limit results.</p> <p>The filter name <code>name</code> is supported. When filtering, you must use the complete value of the filtered item. You can use up to three filters.</p>
    #[serde(rename = "filters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<Vec<Filter>>,
    /// <p>When this parameter is used, <code>ListSimulationApplications</code> only returns <code>maxResults</code> results in a single page along with a <code>nextToken</code> response element. The remaining results of the initial request can be seen by sending another <code>ListSimulationApplications</code> request with the returned <code>nextToken</code> value. This value can be between 1 and 100. If this parameter is not used, then <code>ListSimulationApplications</code> returns up to 100 results and a <code>nextToken</code> value if applicable. </p>
    #[serde(rename = "maxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>If the previous paginated request did not return all of the remaining results, the response object's <code>nextToken</code> parameter value is set to a token. To retrieve the next set of results, call <code>ListSimulationApplications</code> again and assign that token to the request object's <code>nextToken</code> parameter. If there are no remaining results, the previous response object's NextToken parameter is set to null. </p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The version qualifier of the simulation application.</p>
    #[serde(rename = "versionQualifier")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version_qualifier: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListSimulationApplicationsResponse {
    /// <p>If the previous paginated request did not return all of the remaining results, the response object's <code>nextToken</code> parameter value is set to a token. To retrieve the next set of results, call <code>ListSimulationApplications</code> again and assign that token to the request object's <code>nextToken</code> parameter. If there are no remaining results, the previous response object's NextToken parameter is set to null. </p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>A list of simulation application summaries that meet the criteria of the request.</p>
    #[serde(rename = "simulationApplicationSummaries")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub simulation_application_summaries: Option<Vec<SimulationApplicationSummary>>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListSimulationJobBatchesRequest {
    /// <p>Optional filters to limit results.</p>
    #[serde(rename = "filters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<Vec<Filter>>,
    /// <p>When this parameter is used, <code>ListSimulationJobBatches</code> only returns <code>maxResults</code> results in a single page along with a <code>nextToken</code> response element. The remaining results of the initial request can be seen by sending another <code>ListSimulationJobBatches</code> request with the returned <code>nextToken</code> value. </p>
    #[serde(rename = "maxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>If the previous paginated request did not return all of the remaining results, the response object's <code>nextToken</code> parameter value is set to a token. To retrieve the next set of results, call <code>ListSimulationJobBatches</code> again and assign that token to the request object's <code>nextToken</code> parameter. If there are no remaining results, the previous response object's NextToken parameter is set to null. </p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListSimulationJobBatchesResponse {
    /// <p>If the previous paginated request did not return all of the remaining results, the response object's <code>nextToken</code> parameter value is set to a token. To retrieve the next set of results, call <code>ListSimulationJobBatches</code> again and assign that token to the request object's <code>nextToken</code> parameter. If there are no remaining results, the previous response object's NextToken parameter is set to null. </p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>A list of simulation job batch summaries.</p>
    #[serde(rename = "simulationJobBatchSummaries")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub simulation_job_batch_summaries: Option<Vec<SimulationJobBatchSummary>>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListSimulationJobsRequest {
    /// <p>Optional filters to limit results.</p> <p>The filter names <code>status</code> and <code>simulationApplicationName</code> and <code>robotApplicationName</code> are supported. When filtering, you must use the complete value of the filtered item. You can use up to three filters, but they must be for the same named item. For example, if you are looking for items with the status <code>Preparing</code> or the status <code>Running</code>.</p>
    #[serde(rename = "filters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<Vec<Filter>>,
    /// <p>When this parameter is used, <code>ListSimulationJobs</code> only returns <code>maxResults</code> results in a single page along with a <code>nextToken</code> response element. The remaining results of the initial request can be seen by sending another <code>ListSimulationJobs</code> request with the returned <code>nextToken</code> value. This value can be between 1 and 1000. If this parameter is not used, then <code>ListSimulationJobs</code> returns up to 1000 results and a <code>nextToken</code> value if applicable. </p>
    #[serde(rename = "maxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>If the previous paginated request did not return all of the remaining results, the response object's <code>nextToken</code> parameter value is set to a token. To retrieve the next set of results, call <code>ListSimulationJobs</code> again and assign that token to the request object's <code>nextToken</code> parameter. If there are no remaining results, the previous response object's NextToken parameter is set to null. </p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListSimulationJobsResponse {
    /// <p>If the previous paginated request did not return all of the remaining results, the response object's <code>nextToken</code> parameter value is set to a token. To retrieve the next set of results, call <code>ListSimulationJobs</code> again and assign that token to the request object's <code>nextToken</code> parameter. If there are no remaining results, the previous response object's NextToken parameter is set to null. </p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>A list of simulation job summaries that meet the criteria of the request.</p>
    #[serde(rename = "simulationJobSummaries")]
    pub simulation_job_summaries: Vec<SimulationJobSummary>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListTagsForResourceRequest {
    /// <p>The AWS RoboMaker Amazon Resource Name (ARN) with tags to be listed.</p>
    #[serde(rename = "resourceArn")]
    pub resource_arn: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListTagsForResourceResponse {
    /// <p>The list of all tags added to the specified resource.</p>
    #[serde(rename = "tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<::std::collections::HashMap<String, String>>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListWorldExportJobsRequest {
    /// <p>Optional filters to limit results. You can use <code>generationJobId</code> and <code>templateId</code>.</p>
    #[serde(rename = "filters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<Vec<Filter>>,
    /// <p>When this parameter is used, <code>ListWorldExportJobs</code> only returns <code>maxResults</code> results in a single page along with a <code>nextToken</code> response element. The remaining results of the initial request can be seen by sending another <code>ListWorldExportJobs</code> request with the returned <code>nextToken</code> value. This value can be between 1 and 100. If this parameter is not used, then <code>ListWorldExportJobs</code> returns up to 100 results and a <code>nextToken</code> value if applicable. </p>
    #[serde(rename = "maxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>If the previous paginated request did not return all of the remaining results, the response object's <code>nextToken</code> parameter value is set to a token. To retrieve the next set of results, call <code>ListWorldExportJobs</code> again and assign that token to the request object's <code>nextToken</code> parameter. If there are no remaining results, the previous response object's NextToken parameter is set to null. </p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListWorldExportJobsResponse {
    /// <p>If the previous paginated request did not return all of the remaining results, the response object's <code>nextToken</code> parameter value is set to a token. To retrieve the next set of results, call <code>ListWorldExportJobsRequest</code> again and assign that token to the request object's <code>nextToken</code> parameter. If there are no remaining results, the previous response object's NextToken parameter is set to null. </p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>Summary information for world export jobs.</p>
    #[serde(rename = "worldExportJobSummaries")]
    pub world_export_job_summaries: Vec<WorldExportJobSummary>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListWorldGenerationJobsRequest {
    /// <p>Optional filters to limit results. You can use <code>status</code> and <code>templateId</code>.</p>
    #[serde(rename = "filters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<Vec<Filter>>,
    /// <p>When this parameter is used, <code>ListWorldGeneratorJobs</code> only returns <code>maxResults</code> results in a single page along with a <code>nextToken</code> response element. The remaining results of the initial request can be seen by sending another <code>ListWorldGeneratorJobs</code> request with the returned <code>nextToken</code> value. This value can be between 1 and 100. If this parameter is not used, then <code>ListWorldGeneratorJobs</code> returns up to 100 results and a <code>nextToken</code> value if applicable. </p>
    #[serde(rename = "maxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>If the previous paginated request did not return all of the remaining results, the response object's <code>nextToken</code> parameter value is set to a token. To retrieve the next set of results, call <code>ListWorldGenerationJobsRequest</code> again and assign that token to the request object's <code>nextToken</code> parameter. If there are no remaining results, the previous response object's NextToken parameter is set to null. </p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListWorldGenerationJobsResponse {
    /// <p>If the previous paginated request did not return all of the remaining results, the response object's <code>nextToken</code> parameter value is set to a token. To retrieve the next set of results, call <code>ListWorldGeneratorJobsRequest</code> again and assign that token to the request object's <code>nextToken</code> parameter. If there are no remaining results, the previous response object's NextToken parameter is set to null. </p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>Summary information for world generator jobs.</p>
    #[serde(rename = "worldGenerationJobSummaries")]
    pub world_generation_job_summaries: Vec<WorldGenerationJobSummary>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListWorldTemplatesRequest {
    /// <p>When this parameter is used, <code>ListWorldTemplates</code> only returns <code>maxResults</code> results in a single page along with a <code>nextToken</code> response element. The remaining results of the initial request can be seen by sending another <code>ListWorldTemplates</code> request with the returned <code>nextToken</code> value. This value can be between 1 and 100. If this parameter is not used, then <code>ListWorldTemplates</code> returns up to 100 results and a <code>nextToken</code> value if applicable. </p>
    #[serde(rename = "maxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>If the previous paginated request did not return all of the remaining results, the response object's <code>nextToken</code> parameter value is set to a token. To retrieve the next set of results, call <code>ListWorldTemplates</code> again and assign that token to the request object's <code>nextToken</code> parameter. If there are no remaining results, the previous response object's NextToken parameter is set to null. </p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListWorldTemplatesResponse {
    /// <p>If the previous paginated request did not return all of the remaining results, the response object's <code>nextToken</code> parameter value is set to a token. To retrieve the next set of results, call <code>ListWorldTemplates</code> again and assign that token to the request object's <code>nextToken</code> parameter. If there are no remaining results, the previous response object's NextToken parameter is set to null. </p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>Summary information for templates.</p>
    #[serde(rename = "templateSummaries")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template_summaries: Option<Vec<TemplateSummary>>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListWorldsRequest {
    /// <p>Optional filters to limit results. You can use <code>status</code>.</p>
    #[serde(rename = "filters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<Vec<Filter>>,
    /// <p>When this parameter is used, <code>ListWorlds</code> only returns <code>maxResults</code> results in a single page along with a <code>nextToken</code> response element. The remaining results of the initial request can be seen by sending another <code>ListWorlds</code> request with the returned <code>nextToken</code> value. This value can be between 1 and 100. If this parameter is not used, then <code>ListWorlds</code> returns up to 100 results and a <code>nextToken</code> value if applicable. </p>
    #[serde(rename = "maxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>If the previous paginated request did not return all of the remaining results, the response object's <code>nextToken</code> parameter value is set to a token. To retrieve the next set of results, call <code>ListWorlds</code> again and assign that token to the request object's <code>nextToken</code> parameter. If there are no remaining results, the previous response object's NextToken parameter is set to null. </p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListWorldsResponse {
    /// <p>If the previous paginated request did not return all of the remaining results, the response object's <code>nextToken</code> parameter value is set to a token. To retrieve the next set of results, call <code>ListWorlds</code> again and assign that token to the request object's <code>nextToken</code> parameter. If there are no remaining results, the previous response object's NextToken parameter is set to null. </p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>Summary information for worlds.</p>
    #[serde(rename = "worldSummaries")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub world_summaries: Option<Vec<WorldSummary>>,
}

/// <p>The logging configuration.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct LoggingConfig {
    /// <p>A boolean indicating whether to record all ROS topics.</p>
    #[serde(rename = "recordAllRosTopics")]
    pub record_all_ros_topics: bool,
}

/// <p>Describes a network interface.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct NetworkInterface {
    /// <p>The ID of the network interface.</p>
    #[serde(rename = "networkInterfaceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_interface_id: Option<String>,
    /// <p>The IPv4 address of the network interface within the subnet.</p>
    #[serde(rename = "privateIpAddress")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub private_ip_address: Option<String>,
    /// <p>The IPv4 public address of the network interface.</p>
    #[serde(rename = "publicIpAddress")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub public_ip_address: Option<String>,
}

/// <p>The output location.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct OutputLocation {
    /// <p>The S3 bucket for output.</p>
    #[serde(rename = "s3Bucket")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s_3_bucket: Option<String>,
    /// <p>The S3 folder in the <code>s3Bucket</code> where output files will be placed.</p>
    #[serde(rename = "s3Prefix")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s_3_prefix: Option<String>,
}

/// <p>Configuration information for port forwarding.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct PortForwardingConfig {
    /// <p>The port mappings for the configuration.</p>
    #[serde(rename = "portMappings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub port_mappings: Option<Vec<PortMapping>>,
}

/// <p>An object representing a port mapping.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct PortMapping {
    /// <p>The port number on the application.</p>
    #[serde(rename = "applicationPort")]
    pub application_port: i64,
    /// <p>A Boolean indicating whether to enable this port mapping on public IP.</p>
    #[serde(rename = "enableOnPublicIp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_on_public_ip: Option<bool>,
    /// <p>The port number on the simulation job instance to use as a remote connection point. </p>
    #[serde(rename = "jobPort")]
    pub job_port: i64,
}

/// <p>Information about the progress of a deployment job.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ProgressDetail {
    /// <p><p>The current progress status.</p> <dl> <dt>Validating</dt> <dd> <p>Validating the deployment.</p> </dd> <dt>DownloadingExtracting</dt> <dd> <p>Downloading and extracting the bundle on the robot.</p> </dd> <dt>ExecutingPreLaunch</dt> <dd> <p>Executing pre-launch script(s) if provided.</p> </dd> <dt>Launching</dt> <dd> <p>Launching the robot application.</p> </dd> <dt>ExecutingPostLaunch</dt> <dd> <p>Executing post-launch script(s) if provided.</p> </dd> <dt>Finished</dt> <dd> <p>Deployment is complete.</p> </dd> </dl></p>
    #[serde(rename = "currentProgress")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_progress: Option<String>,
    /// <p>Estimated amount of time in seconds remaining in the step. This currently only applies to the <code>Downloading/Extracting</code> step of the deployment. It is empty for other steps.</p>
    #[serde(rename = "estimatedTimeRemainingSeconds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub estimated_time_remaining_seconds: Option<i64>,
    /// <p>Precentage of the step that is done. This currently only applies to the <code>Downloading/Extracting</code> step of the deployment. It is empty for other steps.</p>
    #[serde(rename = "percentDone")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub percent_done: Option<f32>,
    /// <p>The Amazon Resource Name (ARN) of the deployment job.</p>
    #[serde(rename = "targetResource")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_resource: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct RegisterRobotRequest {
    /// <p>The Amazon Resource Name (ARN) of the fleet.</p>
    #[serde(rename = "fleet")]
    pub fleet: String,
    /// <p>The Amazon Resource Name (ARN) of the robot.</p>
    #[serde(rename = "robot")]
    pub robot: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct RegisterRobotResponse {
    /// <p>The Amazon Resource Name (ARN) of the fleet that the robot will join.</p>
    #[serde(rename = "fleet")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fleet: Option<String>,
    /// <p>Information about the robot registration.</p>
    #[serde(rename = "robot")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub robot: Option<String>,
}

/// <p>Information about a rendering engine.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct RenderingEngine {
    /// <p>The name of the rendering engine.</p>
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The version of the rendering engine.</p>
    #[serde(rename = "version")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct RestartSimulationJobRequest {
    /// <p>The Amazon Resource Name (ARN) of the simulation job.</p>
    #[serde(rename = "job")]
    pub job: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct RestartSimulationJobResponse {}

/// <p>Information about a robot.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct Robot {
    /// <p>The architecture of the robot.</p>
    #[serde(rename = "architecture")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub architecture: Option<String>,
    /// <p>The Amazon Resource Name (ARN) of the robot.</p>
    #[serde(rename = "arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p>The time, in milliseconds since the epoch, when the robot was created.</p>
    #[serde(rename = "createdAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<f64>,
    /// <p>The Amazon Resource Name (ARN) of the fleet.</p>
    #[serde(rename = "fleetArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fleet_arn: Option<String>,
    /// <p>The Greengrass group associated with the robot.</p>
    #[serde(rename = "greenGrassGroupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub green_grass_group_id: Option<String>,
    /// <p>The Amazon Resource Name (ARN) of the last deployment job.</p>
    #[serde(rename = "lastDeploymentJob")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_deployment_job: Option<String>,
    /// <p>The time of the last deployment.</p>
    #[serde(rename = "lastDeploymentTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_deployment_time: Option<f64>,
    /// <p>The name of the robot.</p>
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The status of the robot.</p>
    #[serde(rename = "status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

/// <p>Application configuration information for a robot.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct RobotApplicationConfig {
    /// <p>The application information for the robot application.</p>
    #[serde(rename = "application")]
    pub application: String,
    /// <p>The version of the robot application.</p>
    #[serde(rename = "applicationVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_version: Option<String>,
    /// <p>The launch configuration for the robot application.</p>
    #[serde(rename = "launchConfig")]
    pub launch_config: LaunchConfig,
}

/// <p>Summary information for a robot application.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct RobotApplicationSummary {
    /// <p>The Amazon Resource Name (ARN) of the robot.</p>
    #[serde(rename = "arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p>The time, in milliseconds since the epoch, when the robot application was last updated.</p>
    #[serde(rename = "lastUpdatedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_at: Option<f64>,
    /// <p>The name of the robot application.</p>
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>Information about a robot software suite (ROS distribution).</p>
    #[serde(rename = "robotSoftwareSuite")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub robot_software_suite: Option<RobotSoftwareSuite>,
    /// <p>The version of the robot application.</p>
    #[serde(rename = "version")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

/// <p>Information about a robot deployment.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct RobotDeployment {
    /// <p>The robot deployment Amazon Resource Name (ARN).</p>
    #[serde(rename = "arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p>The time, in milliseconds since the epoch, when the deployment finished.</p>
    #[serde(rename = "deploymentFinishTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deployment_finish_time: Option<f64>,
    /// <p>The time, in milliseconds since the epoch, when the deployment was started.</p>
    #[serde(rename = "deploymentStartTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deployment_start_time: Option<f64>,
    /// <p>The robot deployment failure code.</p>
    #[serde(rename = "failureCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_code: Option<String>,
    /// <p>A short description of the reason why the robot deployment failed.</p>
    #[serde(rename = "failureReason")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_reason: Option<String>,
    /// <p>Information about how the deployment is progressing.</p>
    #[serde(rename = "progressDetail")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub progress_detail: Option<ProgressDetail>,
    /// <p>The status of the robot deployment.</p>
    #[serde(rename = "status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

/// <p>Information about a robot software suite (ROS distribution).</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct RobotSoftwareSuite {
    /// <p>The name of the robot software suite (ROS distribution).</p>
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The version of the robot software suite (ROS distribution).</p>
    #[serde(rename = "version")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

/// <p>Information about S3 keys.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct S3KeyOutput {
    /// <p>The etag for the object.</p>
    #[serde(rename = "etag")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub etag: Option<String>,
    /// <p>The S3 key.</p>
    #[serde(rename = "s3Key")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s_3_key: Option<String>,
}

/// <p>Information about an S3 object.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct S3Object {
    /// <p>The bucket containing the object.</p>
    #[serde(rename = "bucket")]
    pub bucket: String,
    /// <p>The etag of the object.</p>
    #[serde(rename = "etag")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub etag: Option<String>,
    /// <p>The key of the object.</p>
    #[serde(rename = "key")]
    pub key: String,
}

/// <p>Information about a simulation application configuration.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct SimulationApplicationConfig {
    /// <p>The application information for the simulation application.</p>
    #[serde(rename = "application")]
    pub application: String,
    /// <p>The version of the simulation application.</p>
    #[serde(rename = "applicationVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_version: Option<String>,
    /// <p>The launch configuration for the simulation application.</p>
    #[serde(rename = "launchConfig")]
    pub launch_config: LaunchConfig,
    /// <p>A list of world configurations.</p>
    #[serde(rename = "worldConfigs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub world_configs: Option<Vec<WorldConfig>>,
}

/// <p>Summary information for a simulation application.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct SimulationApplicationSummary {
    /// <p>The Amazon Resource Name (ARN) of the simulation application.</p>
    #[serde(rename = "arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p>The time, in milliseconds since the epoch, when the simulation application was last updated.</p>
    #[serde(rename = "lastUpdatedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_at: Option<f64>,
    /// <p>The name of the simulation application.</p>
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>Information about a robot software suite (ROS distribution).</p>
    #[serde(rename = "robotSoftwareSuite")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub robot_software_suite: Option<RobotSoftwareSuite>,
    /// <p>Information about a simulation software suite.</p>
    #[serde(rename = "simulationSoftwareSuite")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub simulation_software_suite: Option<SimulationSoftwareSuite>,
    /// <p>The version of the simulation application.</p>
    #[serde(rename = "version")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

/// <p>Information about a simulation job.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct SimulationJob {
    /// <p>The Amazon Resource Name (ARN) of the simulation job.</p>
    #[serde(rename = "arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p>A unique identifier for this <code>SimulationJob</code> request.</p>
    #[serde(rename = "clientRequestToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_request_token: Option<String>,
    /// <p>Compute information for the simulation job</p>
    #[serde(rename = "compute")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compute: Option<ComputeResponse>,
    /// <p>The data sources for the simulation job.</p>
    #[serde(rename = "dataSources")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_sources: Option<Vec<DataSource>>,
    /// <p><p>The failure behavior the simulation job.</p> <dl> <dt>Continue</dt> <dd> <p>Restart the simulation job in the same host instance.</p> </dd> <dt>Fail</dt> <dd> <p>Stop the simulation job and terminate the instance.</p> </dd> </dl></p>
    #[serde(rename = "failureBehavior")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_behavior: Option<String>,
    /// <p>The failure code of the simulation job if it failed.</p>
    #[serde(rename = "failureCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_code: Option<String>,
    /// <p>The reason why the simulation job failed.</p>
    #[serde(rename = "failureReason")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_reason: Option<String>,
    /// <p>The IAM role that allows the simulation instance to call the AWS APIs that are specified in its associated policies on your behalf. This is how credentials are passed in to your simulation job. </p>
    #[serde(rename = "iamRole")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iam_role: Option<String>,
    /// <p>The time, in milliseconds since the epoch, when the simulation job was last started.</p>
    #[serde(rename = "lastStartedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_started_at: Option<f64>,
    /// <p>The time, in milliseconds since the epoch, when the simulation job was last updated.</p>
    #[serde(rename = "lastUpdatedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_at: Option<f64>,
    /// <p>The logging configuration.</p>
    #[serde(rename = "loggingConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logging_config: Option<LoggingConfig>,
    /// <p>The maximum simulation job duration in seconds. The value must be 8 days (691,200 seconds) or less.</p>
    #[serde(rename = "maxJobDurationInSeconds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_job_duration_in_seconds: Option<i64>,
    /// <p>The name of the simulation job.</p>
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>Information about a network interface.</p>
    #[serde(rename = "networkInterface")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_interface: Option<NetworkInterface>,
    /// <p>Location for output files generated by the simulation job.</p>
    #[serde(rename = "outputLocation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_location: Option<OutputLocation>,
    /// <p>A list of robot applications.</p>
    #[serde(rename = "robotApplications")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub robot_applications: Option<Vec<RobotApplicationConfig>>,
    /// <p>A list of simulation applications.</p>
    #[serde(rename = "simulationApplications")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub simulation_applications: Option<Vec<SimulationApplicationConfig>>,
    /// <p>The simulation job execution duration in milliseconds.</p>
    #[serde(rename = "simulationTimeMillis")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub simulation_time_millis: Option<i64>,
    /// <p>Status of the simulation job.</p>
    #[serde(rename = "status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// <p>A map that contains tag keys and tag values that are attached to the simulation job.</p>
    #[serde(rename = "tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<::std::collections::HashMap<String, String>>,
    /// <p>VPC configuration information.</p>
    #[serde(rename = "vpcConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_config: Option<VPCConfigResponse>,
}

/// <p>Information about a simulation job batch.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct SimulationJobBatchSummary {
    /// <p>The Amazon Resource Name (ARN) of the batch.</p>
    #[serde(rename = "arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p>The time, in milliseconds since the epoch, when the simulation job batch was created.</p>
    #[serde(rename = "createdAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<f64>,
    /// <p>The number of created simulation job requests.</p>
    #[serde(rename = "createdRequestCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_request_count: Option<i64>,
    /// <p>The number of failed simulation job requests.</p>
    #[serde(rename = "failedRequestCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failed_request_count: Option<i64>,
    /// <p>The time, in milliseconds since the epoch, when the simulation job batch was last updated.</p>
    #[serde(rename = "lastUpdatedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_at: Option<f64>,
    /// <p>The number of pending simulation job requests.</p>
    #[serde(rename = "pendingRequestCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pending_request_count: Option<i64>,
    /// <p><p>The status of the simulation job batch.</p> <dl> <dt>Pending</dt> <dd> <p>The simulation job batch request is pending.</p> </dd> <dt>InProgress</dt> <dd> <p>The simulation job batch is in progress. </p> </dd> <dt>Failed</dt> <dd> <p>The simulation job batch failed. One or more simulation job requests could not be completed due to an internal failure (like <code>InternalServiceError</code>). See <code>failureCode</code> and <code>failureReason</code> for more information.</p> </dd> <dt>Completed</dt> <dd> <p>The simulation batch job completed. A batch is complete when (1) there are no pending simulation job requests in the batch and none of the failed simulation job requests are due to <code>InternalServiceError</code> and (2) when all created simulation jobs have reached a terminal state (for example, <code>Completed</code> or <code>Failed</code>). </p> </dd> <dt>Canceled</dt> <dd> <p>The simulation batch job was cancelled.</p> </dd> <dt>Canceling</dt> <dd> <p>The simulation batch job is being cancelled.</p> </dd> <dt>Completing</dt> <dd> <p>The simulation batch job is completing.</p> </dd> <dt>TimingOut</dt> <dd> <p>The simulation job batch is timing out.</p> <p>If a batch timing out, and there are pending requests that were failing due to an internal failure (like <code>InternalServiceError</code>), the batch status will be <code>Failed</code>. If there are no such failing request, the batch status will be <code>TimedOut</code>. </p> </dd> <dt>TimedOut</dt> <dd> <p>The simulation batch job timed out.</p> </dd> </dl></p>
    #[serde(rename = "status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

/// <p>Information about a simulation job request.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct SimulationJobRequest {
    /// <p>Compute information for the simulation job</p>
    #[serde(rename = "compute")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compute: Option<Compute>,
    /// <p><p>Specify data sources to mount read-only files from S3 into your simulation. These files are available under <code>/opt/robomaker/datasources/data<em>source</em>name</code>. </p> <note> <p>There is a limit of 100 files and a combined size of 25GB for all <code>DataSourceConfig</code> objects. </p> </note></p>
    #[serde(rename = "dataSources")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_sources: Option<Vec<DataSourceConfig>>,
    /// <p><p>The failure behavior the simulation job.</p> <dl> <dt>Continue</dt> <dd> <p>Restart the simulation job in the same host instance.</p> </dd> <dt>Fail</dt> <dd> <p>Stop the simulation job and terminate the instance.</p> </dd> </dl></p>
    #[serde(rename = "failureBehavior")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_behavior: Option<String>,
    /// <p>The IAM role name that allows the simulation instance to call the AWS APIs that are specified in its associated policies on your behalf. This is how credentials are passed in to your simulation job. </p>
    #[serde(rename = "iamRole")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iam_role: Option<String>,
    #[serde(rename = "loggingConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logging_config: Option<LoggingConfig>,
    /// <p>The maximum simulation job duration in seconds. The value must be 8 days (691,200 seconds) or less.</p>
    #[serde(rename = "maxJobDurationInSeconds")]
    pub max_job_duration_in_seconds: i64,
    #[serde(rename = "outputLocation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_location: Option<OutputLocation>,
    /// <p>The robot applications to use in the simulation job.</p>
    #[serde(rename = "robotApplications")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub robot_applications: Option<Vec<RobotApplicationConfig>>,
    /// <p>The simulation applications to use in the simulation job.</p>
    #[serde(rename = "simulationApplications")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub simulation_applications: Option<Vec<SimulationApplicationConfig>>,
    /// <p>A map that contains tag keys and tag values that are attached to the simulation job request.</p>
    #[serde(rename = "tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<::std::collections::HashMap<String, String>>,
    /// <p>Boolean indicating whether to use default simulation tool applications.</p>
    #[serde(rename = "useDefaultApplications")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_default_applications: Option<bool>,
    #[serde(rename = "vpcConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_config: Option<VPCConfig>,
}

/// <p>Summary information for a simulation job.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct SimulationJobSummary {
    /// <p>The Amazon Resource Name (ARN) of the simulation job.</p>
    #[serde(rename = "arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p>The names of the data sources.</p>
    #[serde(rename = "dataSourceNames")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_source_names: Option<Vec<String>>,
    /// <p>The time, in milliseconds since the epoch, when the simulation job was last updated.</p>
    #[serde(rename = "lastUpdatedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_at: Option<f64>,
    /// <p>The name of the simulation job.</p>
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>A list of simulation job robot application names.</p>
    #[serde(rename = "robotApplicationNames")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub robot_application_names: Option<Vec<String>>,
    /// <p>A list of simulation job simulation application names.</p>
    #[serde(rename = "simulationApplicationNames")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub simulation_application_names: Option<Vec<String>>,
    /// <p>The status of the simulation job.</p>
    #[serde(rename = "status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

/// <p>Information about a simulation software suite.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct SimulationSoftwareSuite {
    /// <p>The name of the simulation software suite.</p>
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The version of the simulation software suite.</p>
    #[serde(rename = "version")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

/// <p>Information about a source.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct Source {
    /// <p>The taget processor architecture for the application.</p>
    #[serde(rename = "architecture")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub architecture: Option<String>,
    /// <p>A hash of the object specified by <code>s3Bucket</code> and <code>s3Key</code>.</p>
    #[serde(rename = "etag")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub etag: Option<String>,
    /// <p>The s3 bucket name.</p>
    #[serde(rename = "s3Bucket")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s_3_bucket: Option<String>,
    /// <p>The s3 object key.</p>
    #[serde(rename = "s3Key")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s_3_key: Option<String>,
}

/// <p>Information about a source configuration.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct SourceConfig {
    /// <p>The target processor architecture for the application.</p>
    #[serde(rename = "architecture")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub architecture: Option<String>,
    /// <p>The Amazon S3 bucket name.</p>
    #[serde(rename = "s3Bucket")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s_3_bucket: Option<String>,
    /// <p>The s3 object key.</p>
    #[serde(rename = "s3Key")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s_3_key: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct StartSimulationJobBatchRequest {
    /// <p>The batch policy.</p>
    #[serde(rename = "batchPolicy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub batch_policy: Option<BatchPolicy>,
    /// <p>Unique, case-sensitive identifier that you provide to ensure the idempotency of the request.</p>
    #[serde(rename = "clientRequestToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_request_token: Option<String>,
    /// <p>A list of simulation job requests to create in the batch.</p>
    #[serde(rename = "createSimulationJobRequests")]
    pub create_simulation_job_requests: Vec<SimulationJobRequest>,
    /// <p>A map that contains tag keys and tag values that are attached to the deployment job batch.</p>
    #[serde(rename = "tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<::std::collections::HashMap<String, String>>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct StartSimulationJobBatchResponse {
    /// <p>The Amazon Resource Name (arn) of the batch.</p>
    #[serde(rename = "arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p>The batch policy.</p>
    #[serde(rename = "batchPolicy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub batch_policy: Option<BatchPolicy>,
    /// <p>Unique, case-sensitive identifier that you provide to ensure the idempotency of the request.</p>
    #[serde(rename = "clientRequestToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_request_token: Option<String>,
    /// <p>The time, in milliseconds since the epoch, when the simulation job batch was created.</p>
    #[serde(rename = "createdAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<f64>,
    /// <p>A list of created simulation job request summaries.</p>
    #[serde(rename = "createdRequests")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_requests: Option<Vec<SimulationJobSummary>>,
    /// <p>A list of failed simulation job requests. The request failed to be created into a simulation job. Failed requests do not have a simulation job ID. </p>
    #[serde(rename = "failedRequests")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failed_requests: Option<Vec<FailedCreateSimulationJobRequest>>,
    /// <p>The failure code if the simulation job batch failed.</p>
    #[serde(rename = "failureCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_code: Option<String>,
    /// <p>The reason the simulation job batch failed.</p>
    #[serde(rename = "failureReason")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_reason: Option<String>,
    /// <p>A list of pending simulation job requests. These requests have not yet been created into simulation jobs.</p>
    #[serde(rename = "pendingRequests")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pending_requests: Option<Vec<SimulationJobRequest>>,
    /// <p><p>The status of the simulation job batch.</p> <dl> <dt>Pending</dt> <dd> <p>The simulation job batch request is pending.</p> </dd> <dt>InProgress</dt> <dd> <p>The simulation job batch is in progress. </p> </dd> <dt>Failed</dt> <dd> <p>The simulation job batch failed. One or more simulation job requests could not be completed due to an internal failure (like <code>InternalServiceError</code>). See <code>failureCode</code> and <code>failureReason</code> for more information.</p> </dd> <dt>Completed</dt> <dd> <p>The simulation batch job completed. A batch is complete when (1) there are no pending simulation job requests in the batch and none of the failed simulation job requests are due to <code>InternalServiceError</code> and (2) when all created simulation jobs have reached a terminal state (for example, <code>Completed</code> or <code>Failed</code>). </p> </dd> <dt>Canceled</dt> <dd> <p>The simulation batch job was cancelled.</p> </dd> <dt>Canceling</dt> <dd> <p>The simulation batch job is being cancelled.</p> </dd> <dt>Completing</dt> <dd> <p>The simulation batch job is completing.</p> </dd> <dt>TimingOut</dt> <dd> <p>The simulation job batch is timing out.</p> <p>If a batch timing out, and there are pending requests that were failing due to an internal failure (like <code>InternalServiceError</code>), the batch status will be <code>Failed</code>. If there are no such failing request, the batch status will be <code>TimedOut</code>. </p> </dd> <dt>TimedOut</dt> <dd> <p>The simulation batch job timed out.</p> </dd> </dl></p>
    #[serde(rename = "status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// <p>A map that contains tag keys and tag values that are attached to the deployment job batch.</p>
    #[serde(rename = "tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<::std::collections::HashMap<String, String>>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct SyncDeploymentJobRequest {
    /// <p>Unique, case-sensitive identifier that you provide to ensure the idempotency of the request.</p>
    #[serde(rename = "clientRequestToken")]
    pub client_request_token: String,
    /// <p>The target fleet for the synchronization.</p>
    #[serde(rename = "fleet")]
    pub fleet: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct SyncDeploymentJobResponse {
    /// <p>The Amazon Resource Name (ARN) of the synchronization request.</p>
    #[serde(rename = "arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p>The time, in milliseconds since the epoch, when the fleet was created.</p>
    #[serde(rename = "createdAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<f64>,
    /// <p>Information about the deployment application configurations.</p>
    #[serde(rename = "deploymentApplicationConfigs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deployment_application_configs: Option<Vec<DeploymentApplicationConfig>>,
    /// <p>Information about the deployment configuration.</p>
    #[serde(rename = "deploymentConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deployment_config: Option<DeploymentConfig>,
    /// <p><p>The failure code if the job fails:</p> <dl> <dt>InternalServiceError</dt> <dd> <p>Internal service error.</p> </dd> <dt>RobotApplicationCrash</dt> <dd> <p>Robot application exited abnormally.</p> </dd> <dt>SimulationApplicationCrash</dt> <dd> <p> Simulation application exited abnormally.</p> </dd> <dt>BadPermissionsRobotApplication</dt> <dd> <p>Robot application bundle could not be downloaded.</p> </dd> <dt>BadPermissionsSimulationApplication</dt> <dd> <p>Simulation application bundle could not be downloaded.</p> </dd> <dt>BadPermissionsS3Output</dt> <dd> <p>Unable to publish outputs to customer-provided S3 bucket.</p> </dd> <dt>BadPermissionsCloudwatchLogs</dt> <dd> <p>Unable to publish logs to customer-provided CloudWatch Logs resource.</p> </dd> <dt>SubnetIpLimitExceeded</dt> <dd> <p>Subnet IP limit exceeded.</p> </dd> <dt>ENILimitExceeded</dt> <dd> <p>ENI limit exceeded.</p> </dd> <dt>BadPermissionsUserCredentials</dt> <dd> <p>Unable to use the Role provided.</p> </dd> <dt>InvalidBundleRobotApplication</dt> <dd> <p>Robot bundle cannot be extracted (invalid format, bundling error, or other issue).</p> </dd> <dt>InvalidBundleSimulationApplication</dt> <dd> <p>Simulation bundle cannot be extracted (invalid format, bundling error, or other issue).</p> </dd> <dt>RobotApplicationVersionMismatchedEtag</dt> <dd> <p>Etag for RobotApplication does not match value during version creation.</p> </dd> <dt>SimulationApplicationVersionMismatchedEtag</dt> <dd> <p>Etag for SimulationApplication does not match value during version creation.</p> </dd> </dl></p>
    #[serde(rename = "failureCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_code: Option<String>,
    /// <p>The failure reason if the job fails.</p>
    #[serde(rename = "failureReason")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_reason: Option<String>,
    /// <p>The Amazon Resource Name (ARN) of the fleet.</p>
    #[serde(rename = "fleet")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fleet: Option<String>,
    /// <p>The status of the synchronization job.</p>
    #[serde(rename = "status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct TagResourceRequest {
    /// <p>The Amazon Resource Name (ARN) of the AWS RoboMaker resource you are tagging.</p>
    #[serde(rename = "resourceArn")]
    pub resource_arn: String,
    /// <p>A map that contains tag keys and tag values that are attached to the resource.</p>
    #[serde(rename = "tags")]
    pub tags: ::std::collections::HashMap<String, String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct TagResourceResponse {}

/// <p>Information about a template location.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct TemplateLocation {
    /// <p>The Amazon S3 bucket name.</p>
    #[serde(rename = "s3Bucket")]
    pub s_3_bucket: String,
    /// <p>The list of S3 keys identifying the data source files.</p>
    #[serde(rename = "s3Key")]
    pub s_3_key: String,
}

/// <p>Summary information for a template.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct TemplateSummary {
    /// <p>The Amazon Resource Name (ARN) of the template.</p>
    #[serde(rename = "arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p>The time, in milliseconds since the epoch, when the template was created.</p>
    #[serde(rename = "createdAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<f64>,
    /// <p>The time, in milliseconds since the epoch, when the template was last updated.</p>
    #[serde(rename = "lastUpdatedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_at: Option<f64>,
    /// <p>The name of the template.</p>
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UntagResourceRequest {
    /// <p>The Amazon Resource Name (ARN) of the AWS RoboMaker resource you are removing tags.</p>
    #[serde(rename = "resourceArn")]
    pub resource_arn: String,
    /// <p>A map that contains tag keys and tag values that will be unattached from the resource.</p>
    #[serde(rename = "tagKeys")]
    pub tag_keys: Vec<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UntagResourceResponse {}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UpdateRobotApplicationRequest {
    /// <p>The application information for the robot application.</p>
    #[serde(rename = "application")]
    pub application: String,
    /// <p>The revision id for the robot application.</p>
    #[serde(rename = "currentRevisionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_revision_id: Option<String>,
    /// <p>The robot software suite (ROS distribution) used by the robot application.</p>
    #[serde(rename = "robotSoftwareSuite")]
    pub robot_software_suite: RobotSoftwareSuite,
    /// <p>The sources of the robot application.</p>
    #[serde(rename = "sources")]
    pub sources: Vec<SourceConfig>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UpdateRobotApplicationResponse {
    /// <p>The Amazon Resource Name (ARN) of the updated robot application.</p>
    #[serde(rename = "arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p>The time, in milliseconds since the epoch, when the robot application was last updated.</p>
    #[serde(rename = "lastUpdatedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_at: Option<f64>,
    /// <p>The name of the robot application.</p>
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The revision id of the robot application.</p>
    #[serde(rename = "revisionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub revision_id: Option<String>,
    /// <p>The robot software suite (ROS distribution) used by the robot application.</p>
    #[serde(rename = "robotSoftwareSuite")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub robot_software_suite: Option<RobotSoftwareSuite>,
    /// <p>The sources of the robot application.</p>
    #[serde(rename = "sources")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sources: Option<Vec<Source>>,
    /// <p>The version of the robot application.</p>
    #[serde(rename = "version")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UpdateSimulationApplicationRequest {
    /// <p>The application information for the simulation application.</p>
    #[serde(rename = "application")]
    pub application: String,
    /// <p>The revision id for the robot application.</p>
    #[serde(rename = "currentRevisionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_revision_id: Option<String>,
    /// <p>The rendering engine for the simulation application.</p>
    #[serde(rename = "renderingEngine")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rendering_engine: Option<RenderingEngine>,
    /// <p>Information about the robot software suite (ROS distribution).</p>
    #[serde(rename = "robotSoftwareSuite")]
    pub robot_software_suite: RobotSoftwareSuite,
    /// <p>The simulation software suite used by the simulation application.</p>
    #[serde(rename = "simulationSoftwareSuite")]
    pub simulation_software_suite: SimulationSoftwareSuite,
    /// <p>The sources of the simulation application.</p>
    #[serde(rename = "sources")]
    pub sources: Vec<SourceConfig>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UpdateSimulationApplicationResponse {
    /// <p>The Amazon Resource Name (ARN) of the updated simulation application.</p>
    #[serde(rename = "arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p>The time, in milliseconds since the epoch, when the simulation application was last updated.</p>
    #[serde(rename = "lastUpdatedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_at: Option<f64>,
    /// <p>The name of the simulation application.</p>
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The rendering engine for the simulation application.</p>
    #[serde(rename = "renderingEngine")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rendering_engine: Option<RenderingEngine>,
    /// <p>The revision id of the simulation application.</p>
    #[serde(rename = "revisionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub revision_id: Option<String>,
    /// <p>Information about the robot software suite (ROS distribution).</p>
    #[serde(rename = "robotSoftwareSuite")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub robot_software_suite: Option<RobotSoftwareSuite>,
    /// <p>The simulation software suite used by the simulation application.</p>
    #[serde(rename = "simulationSoftwareSuite")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub simulation_software_suite: Option<SimulationSoftwareSuite>,
    /// <p>The sources of the simulation application.</p>
    #[serde(rename = "sources")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sources: Option<Vec<Source>>,
    /// <p>The version of the robot application.</p>
    #[serde(rename = "version")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UpdateWorldTemplateRequest {
    /// <p>The name of the template.</p>
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The Amazon Resource Name (arn) of the world template to update.</p>
    #[serde(rename = "template")]
    pub template: String,
    /// <p>The world template body.</p>
    #[serde(rename = "templateBody")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template_body: Option<String>,
    /// <p>The location of the world template.</p>
    #[serde(rename = "templateLocation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template_location: Option<TemplateLocation>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UpdateWorldTemplateResponse {
    /// <p>The Amazon Resource Name (arn) of the world template.</p>
    #[serde(rename = "arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p>The time, in milliseconds since the epoch, when the world template was created.</p>
    #[serde(rename = "createdAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<f64>,
    /// <p>The time, in milliseconds since the epoch, when the world template was last updated.</p>
    #[serde(rename = "lastUpdatedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_at: Option<f64>,
    /// <p>The name of the world template.</p>
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

/// <p>If your simulation job accesses resources in a VPC, you provide this parameter identifying the list of security group IDs and subnet IDs. These must belong to the same VPC. You must provide at least one security group and two subnet IDs.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct VPCConfig {
    /// <p>A boolean indicating whether to assign a public IP address.</p>
    #[serde(rename = "assignPublicIp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assign_public_ip: Option<bool>,
    /// <p>A list of one or more security groups IDs in your VPC.</p>
    #[serde(rename = "securityGroups")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_groups: Option<Vec<String>>,
    /// <p>A list of one or more subnet IDs in your VPC.</p>
    #[serde(rename = "subnets")]
    pub subnets: Vec<String>,
}

/// <p>VPC configuration associated with your simulation job.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct VPCConfigResponse {
    /// <p>A boolean indicating if a public IP was assigned.</p>
    #[serde(rename = "assignPublicIp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assign_public_ip: Option<bool>,
    /// <p>A list of security group IDs associated with the simulation job.</p>
    #[serde(rename = "securityGroups")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_groups: Option<Vec<String>>,
    /// <p>A list of subnet IDs associated with the simulation job.</p>
    #[serde(rename = "subnets")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subnets: Option<Vec<String>>,
    /// <p>The VPC ID associated with your simulation job.</p>
    #[serde(rename = "vpcId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_id: Option<String>,
}

/// <p>Configuration information for a world.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct WorldConfig {
    /// <p>The world generated by Simulation WorldForge.</p>
    #[serde(rename = "world")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub world: Option<String>,
}

/// <p>The number of worlds that will be created. You can configure the number of unique floorplans and the number of unique interiors for each floor plan. For example, if you want 1 world with 20 unique interiors, you set <code>floorplanCount = 1</code> and <code>interiorCountPerFloorplan = 20</code>. This will result in 20 worlds (<code>floorplanCount</code> * <code>interiorCountPerFloorplan)</code>. </p> <p>If you set <code>floorplanCount = 4</code> and <code>interiorCountPerFloorplan = 5</code>, there will be 20 worlds with 5 unique floor plans. </p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct WorldCount {
    /// <p>The number of unique floorplans.</p>
    #[serde(rename = "floorplanCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub floorplan_count: Option<i64>,
    /// <p>The number of unique interiors per floorplan.</p>
    #[serde(rename = "interiorCountPerFloorplan")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interior_count_per_floorplan: Option<i64>,
}

/// <p>Information about a world export job.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct WorldExportJobSummary {
    /// <p>The Amazon Resource Name (ARN) of the world export job.</p>
    #[serde(rename = "arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p>The time, in milliseconds since the epoch, when the world export job was created.</p>
    #[serde(rename = "createdAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<f64>,
    /// <p><p>The status of the world export job.</p> <dl> <dt>Pending</dt> <dd> <p>The world export job request is pending.</p> </dd> <dt>Running</dt> <dd> <p>The world export job is running. </p> </dd> <dt>Completed</dt> <dd> <p>The world export job completed. </p> </dd> <dt>Failed</dt> <dd> <p>The world export job failed. See <code>failureCode</code> for more information. </p> </dd> <dt>Canceled</dt> <dd> <p>The world export job was cancelled.</p> </dd> <dt>Canceling</dt> <dd> <p>The world export job is being cancelled.</p> </dd> </dl></p>
    #[serde(rename = "status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// <p>A list of worlds.</p>
    #[serde(rename = "worlds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub worlds: Option<Vec<String>>,
}

/// <p>Information about a failed world.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct WorldFailure {
    /// <p><p>The failure code of the world export job if it failed:</p> <dl> <dt>InternalServiceError</dt> <dd> <p>Internal service error.</p> </dd> <dt>LimitExceeded</dt> <dd> <p>The requested resource exceeds the maximum number allowed, or the number of concurrent stream requests exceeds the maximum number allowed. </p> </dd> <dt>ResourceNotFound</dt> <dd> <p>The specified resource could not be found. </p> </dd> <dt>RequestThrottled</dt> <dd> <p>The request was throttled.</p> </dd> <dt>InvalidInput</dt> <dd> <p>An input parameter in the request is not valid.</p> </dd> </dl></p>
    #[serde(rename = "failureCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_code: Option<String>,
    /// <p>The number of failed worlds.</p>
    #[serde(rename = "failureCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_count: Option<i64>,
    /// <p>The sample reason why the world failed. World errors are aggregated. A sample is used as the <code>sampleFailureReason</code>. </p>
    #[serde(rename = "sampleFailureReason")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sample_failure_reason: Option<String>,
}

/// <p>Information about a world generator job.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct WorldGenerationJobSummary {
    /// <p>The Amazon Resource Name (ARN) of the world generator job.</p>
    #[serde(rename = "arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p>The time, in milliseconds since the epoch, when the world generator job was created.</p>
    #[serde(rename = "createdAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<f64>,
    /// <p>The number of worlds that failed.</p>
    #[serde(rename = "failedWorldCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failed_world_count: Option<i64>,
    /// <p><p>The status of the world generator job:</p> <dl> <dt>Pending</dt> <dd> <p>The world generator job request is pending.</p> </dd> <dt>Running</dt> <dd> <p>The world generator job is running. </p> </dd> <dt>Completed</dt> <dd> <p>The world generator job completed. </p> </dd> <dt>Failed</dt> <dd> <p>The world generator job failed. See <code>failureCode</code> for more information. </p> </dd> <dt>PartialFailed</dt> <dd> <p>Some worlds did not generate.</p> </dd> <dt>Canceled</dt> <dd> <p>The world generator job was cancelled.</p> </dd> <dt>Canceling</dt> <dd> <p>The world generator job is being cancelled.</p> </dd> </dl></p>
    #[serde(rename = "status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// <p>The number of worlds that were generated.</p>
    #[serde(rename = "succeededWorldCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub succeeded_world_count: Option<i64>,
    /// <p>The Amazon Resource Name (arn) of the world template.</p>
    #[serde(rename = "template")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template: Option<String>,
    /// <p>Information about the world count.</p>
    #[serde(rename = "worldCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub world_count: Option<WorldCount>,
}

/// <p>Information about a world.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct WorldSummary {
    /// <p>The Amazon Resource Name (ARN) of the world.</p>
    #[serde(rename = "arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p>The time, in milliseconds since the epoch, when the world was created.</p>
    #[serde(rename = "createdAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<f64>,
    /// <p>The Amazon Resource Name (arn) of the world generation job.</p>
    #[serde(rename = "generationJob")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub generation_job: Option<String>,
    /// <p>The Amazon Resource Name (arn) of the world template.</p>
    #[serde(rename = "template")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template: Option<String>,
}

/// Errors returned by BatchDeleteWorlds
#[derive(Debug, PartialEq)]
pub enum BatchDeleteWorldsError {
    /// <p>AWS RoboMaker experienced a service issue. Try your call again.</p>
    InternalServer(String),
    /// <p>A parameter specified in a request is not valid, is unsupported, or cannot be used. The returned message provides an explanation of the error value.</p>
    InvalidParameter(String),
    /// <p>AWS RoboMaker is temporarily unable to process the request. Try your call again.</p>
    Throttling(String),
}

impl BatchDeleteWorldsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<BatchDeleteWorldsError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            #[allow(clippy::single_match)]
            match err.typ.as_str() {
                "InternalServerException" => {
                    return RusotoError::Service(BatchDeleteWorldsError::InternalServer(err.msg))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(BatchDeleteWorldsError::InvalidParameter(err.msg))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(BatchDeleteWorldsError::Throttling(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for BatchDeleteWorldsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            BatchDeleteWorldsError::InternalServer(ref cause) => write!(f, "{}", cause),
            BatchDeleteWorldsError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            BatchDeleteWorldsError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for BatchDeleteWorldsError {}
/// Errors returned by BatchDescribeSimulationJob
#[derive(Debug, PartialEq)]
pub enum BatchDescribeSimulationJobError {
    /// <p>AWS RoboMaker experienced a service issue. Try your call again.</p>
    InternalServer(String),
    /// <p>A parameter specified in a request is not valid, is unsupported, or cannot be used. The returned message provides an explanation of the error value.</p>
    InvalidParameter(String),
    /// <p>The specified resource does not exist.</p>
    ResourceNotFound(String),
    /// <p>AWS RoboMaker is temporarily unable to process the request. Try your call again.</p>
    Throttling(String),
}

impl BatchDescribeSimulationJobError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<BatchDescribeSimulationJobError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            #[allow(clippy::single_match)]
            match err.typ.as_str() {
                "InternalServerException" => {
                    return RusotoError::Service(BatchDescribeSimulationJobError::InternalServer(
                        err.msg,
                    ))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(BatchDescribeSimulationJobError::InvalidParameter(
                        err.msg,
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(BatchDescribeSimulationJobError::ResourceNotFound(
                        err.msg,
                    ))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(BatchDescribeSimulationJobError::Throttling(
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
impl fmt::Display for BatchDescribeSimulationJobError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            BatchDescribeSimulationJobError::InternalServer(ref cause) => write!(f, "{}", cause),
            BatchDescribeSimulationJobError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            BatchDescribeSimulationJobError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            BatchDescribeSimulationJobError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for BatchDescribeSimulationJobError {}
/// Errors returned by CancelDeploymentJob
#[derive(Debug, PartialEq)]
pub enum CancelDeploymentJobError {
    /// <p>AWS RoboMaker experienced a service issue. Try your call again.</p>
    InternalServer(String),
    /// <p>A parameter specified in a request is not valid, is unsupported, or cannot be used. The returned message provides an explanation of the error value.</p>
    InvalidParameter(String),
    /// <p>The specified resource does not exist.</p>
    ResourceNotFound(String),
    /// <p>AWS RoboMaker is temporarily unable to process the request. Try your call again.</p>
    Throttling(String),
}

impl CancelDeploymentJobError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CancelDeploymentJobError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            #[allow(clippy::single_match)]
            match err.typ.as_str() {
                "InternalServerException" => {
                    return RusotoError::Service(CancelDeploymentJobError::InternalServer(err.msg))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(CancelDeploymentJobError::InvalidParameter(
                        err.msg,
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(CancelDeploymentJobError::ResourceNotFound(
                        err.msg,
                    ))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(CancelDeploymentJobError::Throttling(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for CancelDeploymentJobError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CancelDeploymentJobError::InternalServer(ref cause) => write!(f, "{}", cause),
            CancelDeploymentJobError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            CancelDeploymentJobError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            CancelDeploymentJobError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CancelDeploymentJobError {}
/// Errors returned by CancelSimulationJob
#[derive(Debug, PartialEq)]
pub enum CancelSimulationJobError {
    /// <p>AWS RoboMaker experienced a service issue. Try your call again.</p>
    InternalServer(String),
    /// <p>A parameter specified in a request is not valid, is unsupported, or cannot be used. The returned message provides an explanation of the error value.</p>
    InvalidParameter(String),
    /// <p>The specified resource does not exist.</p>
    ResourceNotFound(String),
    /// <p>AWS RoboMaker is temporarily unable to process the request. Try your call again.</p>
    Throttling(String),
}

impl CancelSimulationJobError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CancelSimulationJobError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            #[allow(clippy::single_match)]
            match err.typ.as_str() {
                "InternalServerException" => {
                    return RusotoError::Service(CancelSimulationJobError::InternalServer(err.msg))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(CancelSimulationJobError::InvalidParameter(
                        err.msg,
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(CancelSimulationJobError::ResourceNotFound(
                        err.msg,
                    ))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(CancelSimulationJobError::Throttling(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for CancelSimulationJobError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CancelSimulationJobError::InternalServer(ref cause) => write!(f, "{}", cause),
            CancelSimulationJobError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            CancelSimulationJobError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            CancelSimulationJobError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CancelSimulationJobError {}
/// Errors returned by CancelSimulationJobBatch
#[derive(Debug, PartialEq)]
pub enum CancelSimulationJobBatchError {
    /// <p>AWS RoboMaker experienced a service issue. Try your call again.</p>
    InternalServer(String),
    /// <p>A parameter specified in a request is not valid, is unsupported, or cannot be used. The returned message provides an explanation of the error value.</p>
    InvalidParameter(String),
    /// <p>The specified resource does not exist.</p>
    ResourceNotFound(String),
    /// <p>AWS RoboMaker is temporarily unable to process the request. Try your call again.</p>
    Throttling(String),
}

impl CancelSimulationJobBatchError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CancelSimulationJobBatchError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            #[allow(clippy::single_match)]
            match err.typ.as_str() {
                "InternalServerException" => {
                    return RusotoError::Service(CancelSimulationJobBatchError::InternalServer(
                        err.msg,
                    ))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(CancelSimulationJobBatchError::InvalidParameter(
                        err.msg,
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(CancelSimulationJobBatchError::ResourceNotFound(
                        err.msg,
                    ))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(CancelSimulationJobBatchError::Throttling(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for CancelSimulationJobBatchError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CancelSimulationJobBatchError::InternalServer(ref cause) => write!(f, "{}", cause),
            CancelSimulationJobBatchError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            CancelSimulationJobBatchError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            CancelSimulationJobBatchError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CancelSimulationJobBatchError {}
/// Errors returned by CancelWorldExportJob
#[derive(Debug, PartialEq)]
pub enum CancelWorldExportJobError {
    /// <p>AWS RoboMaker experienced a service issue. Try your call again.</p>
    InternalServer(String),
    /// <p>A parameter specified in a request is not valid, is unsupported, or cannot be used. The returned message provides an explanation of the error value.</p>
    InvalidParameter(String),
    /// <p>The specified resource does not exist.</p>
    ResourceNotFound(String),
    /// <p>AWS RoboMaker is temporarily unable to process the request. Try your call again.</p>
    Throttling(String),
}

impl CancelWorldExportJobError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CancelWorldExportJobError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            #[allow(clippy::single_match)]
            match err.typ.as_str() {
                "InternalServerException" => {
                    return RusotoError::Service(CancelWorldExportJobError::InternalServer(err.msg))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(CancelWorldExportJobError::InvalidParameter(
                        err.msg,
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(CancelWorldExportJobError::ResourceNotFound(
                        err.msg,
                    ))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(CancelWorldExportJobError::Throttling(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for CancelWorldExportJobError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CancelWorldExportJobError::InternalServer(ref cause) => write!(f, "{}", cause),
            CancelWorldExportJobError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            CancelWorldExportJobError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            CancelWorldExportJobError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CancelWorldExportJobError {}
/// Errors returned by CancelWorldGenerationJob
#[derive(Debug, PartialEq)]
pub enum CancelWorldGenerationJobError {
    /// <p>AWS RoboMaker experienced a service issue. Try your call again.</p>
    InternalServer(String),
    /// <p>A parameter specified in a request is not valid, is unsupported, or cannot be used. The returned message provides an explanation of the error value.</p>
    InvalidParameter(String),
    /// <p>The specified resource does not exist.</p>
    ResourceNotFound(String),
    /// <p>AWS RoboMaker is temporarily unable to process the request. Try your call again.</p>
    Throttling(String),
}

impl CancelWorldGenerationJobError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CancelWorldGenerationJobError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            #[allow(clippy::single_match)]
            match err.typ.as_str() {
                "InternalServerException" => {
                    return RusotoError::Service(CancelWorldGenerationJobError::InternalServer(
                        err.msg,
                    ))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(CancelWorldGenerationJobError::InvalidParameter(
                        err.msg,
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(CancelWorldGenerationJobError::ResourceNotFound(
                        err.msg,
                    ))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(CancelWorldGenerationJobError::Throttling(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for CancelWorldGenerationJobError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CancelWorldGenerationJobError::InternalServer(ref cause) => write!(f, "{}", cause),
            CancelWorldGenerationJobError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            CancelWorldGenerationJobError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            CancelWorldGenerationJobError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CancelWorldGenerationJobError {}
/// Errors returned by CreateDeploymentJob
#[derive(Debug, PartialEq)]
pub enum CreateDeploymentJobError {
    /// <p>The failure percentage threshold percentage was met.</p>
    ConcurrentDeployment(String),
    /// <p>The request uses the same client token as a previous, but non-identical request. Do not reuse a client token with different requests, unless the requests are identical. </p>
    IdempotentParameterMismatch(String),
    /// <p>AWS RoboMaker experienced a service issue. Try your call again.</p>
    InternalServer(String),
    /// <p>A parameter specified in a request is not valid, is unsupported, or cannot be used. The returned message provides an explanation of the error value.</p>
    InvalidParameter(String),
    /// <p>The requested resource exceeds the maximum number allowed, or the number of concurrent stream requests exceeds the maximum number allowed. </p>
    LimitExceeded(String),
    /// <p>The specified resource does not exist.</p>
    ResourceNotFound(String),
    /// <p>AWS RoboMaker is temporarily unable to process the request. Try your call again.</p>
    Throttling(String),
}

impl CreateDeploymentJobError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateDeploymentJobError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            #[allow(clippy::single_match)]
            match err.typ.as_str() {
                "ConcurrentDeploymentException" => {
                    return RusotoError::Service(CreateDeploymentJobError::ConcurrentDeployment(
                        err.msg,
                    ))
                }
                "IdempotentParameterMismatchException" => {
                    return RusotoError::Service(
                        CreateDeploymentJobError::IdempotentParameterMismatch(err.msg),
                    )
                }
                "InternalServerException" => {
                    return RusotoError::Service(CreateDeploymentJobError::InternalServer(err.msg))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(CreateDeploymentJobError::InvalidParameter(
                        err.msg,
                    ))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(CreateDeploymentJobError::LimitExceeded(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(CreateDeploymentJobError::ResourceNotFound(
                        err.msg,
                    ))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(CreateDeploymentJobError::Throttling(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for CreateDeploymentJobError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreateDeploymentJobError::ConcurrentDeployment(ref cause) => write!(f, "{}", cause),
            CreateDeploymentJobError::IdempotentParameterMismatch(ref cause) => {
                write!(f, "{}", cause)
            }
            CreateDeploymentJobError::InternalServer(ref cause) => write!(f, "{}", cause),
            CreateDeploymentJobError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            CreateDeploymentJobError::LimitExceeded(ref cause) => write!(f, "{}", cause),
            CreateDeploymentJobError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            CreateDeploymentJobError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CreateDeploymentJobError {}
/// Errors returned by CreateFleet
#[derive(Debug, PartialEq)]
pub enum CreateFleetError {
    /// <p>AWS RoboMaker experienced a service issue. Try your call again.</p>
    InternalServer(String),
    /// <p>A parameter specified in a request is not valid, is unsupported, or cannot be used. The returned message provides an explanation of the error value.</p>
    InvalidParameter(String),
    /// <p>The requested resource exceeds the maximum number allowed, or the number of concurrent stream requests exceeds the maximum number allowed. </p>
    LimitExceeded(String),
    /// <p>AWS RoboMaker is temporarily unable to process the request. Try your call again.</p>
    Throttling(String),
}

impl CreateFleetError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateFleetError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            #[allow(clippy::single_match)]
            match err.typ.as_str() {
                "InternalServerException" => {
                    return RusotoError::Service(CreateFleetError::InternalServer(err.msg))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(CreateFleetError::InvalidParameter(err.msg))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(CreateFleetError::LimitExceeded(err.msg))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(CreateFleetError::Throttling(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for CreateFleetError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreateFleetError::InternalServer(ref cause) => write!(f, "{}", cause),
            CreateFleetError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            CreateFleetError::LimitExceeded(ref cause) => write!(f, "{}", cause),
            CreateFleetError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CreateFleetError {}
/// Errors returned by CreateRobot
#[derive(Debug, PartialEq)]
pub enum CreateRobotError {
    /// <p>AWS RoboMaker experienced a service issue. Try your call again.</p>
    InternalServer(String),
    /// <p>A parameter specified in a request is not valid, is unsupported, or cannot be used. The returned message provides an explanation of the error value.</p>
    InvalidParameter(String),
    /// <p>The requested resource exceeds the maximum number allowed, or the number of concurrent stream requests exceeds the maximum number allowed. </p>
    LimitExceeded(String),
    /// <p>The specified resource already exists.</p>
    ResourceAlreadyExists(String),
    /// <p>AWS RoboMaker is temporarily unable to process the request. Try your call again.</p>
    Throttling(String),
}

impl CreateRobotError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateRobotError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            #[allow(clippy::single_match)]
            match err.typ.as_str() {
                "InternalServerException" => {
                    return RusotoError::Service(CreateRobotError::InternalServer(err.msg))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(CreateRobotError::InvalidParameter(err.msg))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(CreateRobotError::LimitExceeded(err.msg))
                }
                "ResourceAlreadyExistsException" => {
                    return RusotoError::Service(CreateRobotError::ResourceAlreadyExists(err.msg))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(CreateRobotError::Throttling(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for CreateRobotError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreateRobotError::InternalServer(ref cause) => write!(f, "{}", cause),
            CreateRobotError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            CreateRobotError::LimitExceeded(ref cause) => write!(f, "{}", cause),
            CreateRobotError::ResourceAlreadyExists(ref cause) => write!(f, "{}", cause),
            CreateRobotError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CreateRobotError {}
/// Errors returned by CreateRobotApplication
#[derive(Debug, PartialEq)]
pub enum CreateRobotApplicationError {
    /// <p>The request uses the same client token as a previous, but non-identical request. Do not reuse a client token with different requests, unless the requests are identical. </p>
    IdempotentParameterMismatch(String),
    /// <p>AWS RoboMaker experienced a service issue. Try your call again.</p>
    InternalServer(String),
    /// <p>A parameter specified in a request is not valid, is unsupported, or cannot be used. The returned message provides an explanation of the error value.</p>
    InvalidParameter(String),
    /// <p>The requested resource exceeds the maximum number allowed, or the number of concurrent stream requests exceeds the maximum number allowed. </p>
    LimitExceeded(String),
    /// <p>The specified resource already exists.</p>
    ResourceAlreadyExists(String),
    /// <p>AWS RoboMaker is temporarily unable to process the request. Try your call again.</p>
    Throttling(String),
}

impl CreateRobotApplicationError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateRobotApplicationError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            #[allow(clippy::single_match)]
            match err.typ.as_str() {
                "IdempotentParameterMismatchException" => {
                    return RusotoError::Service(
                        CreateRobotApplicationError::IdempotentParameterMismatch(err.msg),
                    )
                }
                "InternalServerException" => {
                    return RusotoError::Service(CreateRobotApplicationError::InternalServer(
                        err.msg,
                    ))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(CreateRobotApplicationError::InvalidParameter(
                        err.msg,
                    ))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(CreateRobotApplicationError::LimitExceeded(
                        err.msg,
                    ))
                }
                "ResourceAlreadyExistsException" => {
                    return RusotoError::Service(
                        CreateRobotApplicationError::ResourceAlreadyExists(err.msg),
                    )
                }
                "ThrottlingException" => {
                    return RusotoError::Service(CreateRobotApplicationError::Throttling(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for CreateRobotApplicationError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreateRobotApplicationError::IdempotentParameterMismatch(ref cause) => {
                write!(f, "{}", cause)
            }
            CreateRobotApplicationError::InternalServer(ref cause) => write!(f, "{}", cause),
            CreateRobotApplicationError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            CreateRobotApplicationError::LimitExceeded(ref cause) => write!(f, "{}", cause),
            CreateRobotApplicationError::ResourceAlreadyExists(ref cause) => write!(f, "{}", cause),
            CreateRobotApplicationError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CreateRobotApplicationError {}
/// Errors returned by CreateRobotApplicationVersion
#[derive(Debug, PartialEq)]
pub enum CreateRobotApplicationVersionError {
    /// <p>The request uses the same client token as a previous, but non-identical request. Do not reuse a client token with different requests, unless the requests are identical. </p>
    IdempotentParameterMismatch(String),
    /// <p>AWS RoboMaker experienced a service issue. Try your call again.</p>
    InternalServer(String),
    /// <p>A parameter specified in a request is not valid, is unsupported, or cannot be used. The returned message provides an explanation of the error value.</p>
    InvalidParameter(String),
    /// <p>The requested resource exceeds the maximum number allowed, or the number of concurrent stream requests exceeds the maximum number allowed. </p>
    LimitExceeded(String),
    /// <p>AWS RoboMaker is temporarily unable to process the request. Try your call again.</p>
    Throttling(String),
}

impl CreateRobotApplicationVersionError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<CreateRobotApplicationVersionError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            #[allow(clippy::single_match)]
            match err.typ.as_str() {
                "IdempotentParameterMismatchException" => {
                    return RusotoError::Service(
                        CreateRobotApplicationVersionError::IdempotentParameterMismatch(err.msg),
                    )
                }
                "InternalServerException" => {
                    return RusotoError::Service(
                        CreateRobotApplicationVersionError::InternalServer(err.msg),
                    )
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(
                        CreateRobotApplicationVersionError::InvalidParameter(err.msg),
                    )
                }
                "LimitExceededException" => {
                    return RusotoError::Service(CreateRobotApplicationVersionError::LimitExceeded(
                        err.msg,
                    ))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(CreateRobotApplicationVersionError::Throttling(
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
impl fmt::Display for CreateRobotApplicationVersionError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreateRobotApplicationVersionError::IdempotentParameterMismatch(ref cause) => {
                write!(f, "{}", cause)
            }
            CreateRobotApplicationVersionError::InternalServer(ref cause) => write!(f, "{}", cause),
            CreateRobotApplicationVersionError::InvalidParameter(ref cause) => {
                write!(f, "{}", cause)
            }
            CreateRobotApplicationVersionError::LimitExceeded(ref cause) => write!(f, "{}", cause),
            CreateRobotApplicationVersionError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CreateRobotApplicationVersionError {}
/// Errors returned by CreateSimulationApplication
#[derive(Debug, PartialEq)]
pub enum CreateSimulationApplicationError {
    /// <p>The request uses the same client token as a previous, but non-identical request. Do not reuse a client token with different requests, unless the requests are identical. </p>
    IdempotentParameterMismatch(String),
    /// <p>AWS RoboMaker experienced a service issue. Try your call again.</p>
    InternalServer(String),
    /// <p>A parameter specified in a request is not valid, is unsupported, or cannot be used. The returned message provides an explanation of the error value.</p>
    InvalidParameter(String),
    /// <p>The requested resource exceeds the maximum number allowed, or the number of concurrent stream requests exceeds the maximum number allowed. </p>
    LimitExceeded(String),
    /// <p>The specified resource already exists.</p>
    ResourceAlreadyExists(String),
    /// <p>AWS RoboMaker is temporarily unable to process the request. Try your call again.</p>
    Throttling(String),
}

impl CreateSimulationApplicationError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<CreateSimulationApplicationError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            #[allow(clippy::single_match)]
            match err.typ.as_str() {
                "IdempotentParameterMismatchException" => {
                    return RusotoError::Service(
                        CreateSimulationApplicationError::IdempotentParameterMismatch(err.msg),
                    )
                }
                "InternalServerException" => {
                    return RusotoError::Service(CreateSimulationApplicationError::InternalServer(
                        err.msg,
                    ))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(
                        CreateSimulationApplicationError::InvalidParameter(err.msg),
                    )
                }
                "LimitExceededException" => {
                    return RusotoError::Service(CreateSimulationApplicationError::LimitExceeded(
                        err.msg,
                    ))
                }
                "ResourceAlreadyExistsException" => {
                    return RusotoError::Service(
                        CreateSimulationApplicationError::ResourceAlreadyExists(err.msg),
                    )
                }
                "ThrottlingException" => {
                    return RusotoError::Service(CreateSimulationApplicationError::Throttling(
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
impl fmt::Display for CreateSimulationApplicationError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreateSimulationApplicationError::IdempotentParameterMismatch(ref cause) => {
                write!(f, "{}", cause)
            }
            CreateSimulationApplicationError::InternalServer(ref cause) => write!(f, "{}", cause),
            CreateSimulationApplicationError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            CreateSimulationApplicationError::LimitExceeded(ref cause) => write!(f, "{}", cause),
            CreateSimulationApplicationError::ResourceAlreadyExists(ref cause) => {
                write!(f, "{}", cause)
            }
            CreateSimulationApplicationError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CreateSimulationApplicationError {}
/// Errors returned by CreateSimulationApplicationVersion
#[derive(Debug, PartialEq)]
pub enum CreateSimulationApplicationVersionError {
    /// <p>The request uses the same client token as a previous, but non-identical request. Do not reuse a client token with different requests, unless the requests are identical. </p>
    IdempotentParameterMismatch(String),
    /// <p>AWS RoboMaker experienced a service issue. Try your call again.</p>
    InternalServer(String),
    /// <p>A parameter specified in a request is not valid, is unsupported, or cannot be used. The returned message provides an explanation of the error value.</p>
    InvalidParameter(String),
    /// <p>The requested resource exceeds the maximum number allowed, or the number of concurrent stream requests exceeds the maximum number allowed. </p>
    LimitExceeded(String),
    /// <p>AWS RoboMaker is temporarily unable to process the request. Try your call again.</p>
    Throttling(String),
}

impl CreateSimulationApplicationVersionError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<CreateSimulationApplicationVersionError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            #[allow(clippy::single_match)]
            match err.typ.as_str() {
                "IdempotentParameterMismatchException" => {
                    return RusotoError::Service(
                        CreateSimulationApplicationVersionError::IdempotentParameterMismatch(
                            err.msg,
                        ),
                    )
                }
                "InternalServerException" => {
                    return RusotoError::Service(
                        CreateSimulationApplicationVersionError::InternalServer(err.msg),
                    )
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(
                        CreateSimulationApplicationVersionError::InvalidParameter(err.msg),
                    )
                }
                "LimitExceededException" => {
                    return RusotoError::Service(
                        CreateSimulationApplicationVersionError::LimitExceeded(err.msg),
                    )
                }
                "ThrottlingException" => {
                    return RusotoError::Service(
                        CreateSimulationApplicationVersionError::Throttling(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for CreateSimulationApplicationVersionError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreateSimulationApplicationVersionError::IdempotentParameterMismatch(ref cause) => {
                write!(f, "{}", cause)
            }
            CreateSimulationApplicationVersionError::InternalServer(ref cause) => {
                write!(f, "{}", cause)
            }
            CreateSimulationApplicationVersionError::InvalidParameter(ref cause) => {
                write!(f, "{}", cause)
            }
            CreateSimulationApplicationVersionError::LimitExceeded(ref cause) => {
                write!(f, "{}", cause)
            }
            CreateSimulationApplicationVersionError::Throttling(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for CreateSimulationApplicationVersionError {}
/// Errors returned by CreateSimulationJob
#[derive(Debug, PartialEq)]
pub enum CreateSimulationJobError {
    /// <p>The request uses the same client token as a previous, but non-identical request. Do not reuse a client token with different requests, unless the requests are identical. </p>
    IdempotentParameterMismatch(String),
    /// <p>AWS RoboMaker experienced a service issue. Try your call again.</p>
    InternalServer(String),
    /// <p>A parameter specified in a request is not valid, is unsupported, or cannot be used. The returned message provides an explanation of the error value.</p>
    InvalidParameter(String),
    /// <p>The requested resource exceeds the maximum number allowed, or the number of concurrent stream requests exceeds the maximum number allowed. </p>
    LimitExceeded(String),
    /// <p>The specified resource does not exist.</p>
    ResourceNotFound(String),
    /// <p>The request has failed due to a temporary failure of the server.</p>
    ServiceUnavailable(String),
    /// <p>AWS RoboMaker is temporarily unable to process the request. Try your call again.</p>
    Throttling(String),
}

impl CreateSimulationJobError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateSimulationJobError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            #[allow(clippy::single_match)]
            match err.typ.as_str() {
                "IdempotentParameterMismatchException" => {
                    return RusotoError::Service(
                        CreateSimulationJobError::IdempotentParameterMismatch(err.msg),
                    )
                }
                "InternalServerException" => {
                    return RusotoError::Service(CreateSimulationJobError::InternalServer(err.msg))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(CreateSimulationJobError::InvalidParameter(
                        err.msg,
                    ))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(CreateSimulationJobError::LimitExceeded(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(CreateSimulationJobError::ResourceNotFound(
                        err.msg,
                    ))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(CreateSimulationJobError::ServiceUnavailable(
                        err.msg,
                    ))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(CreateSimulationJobError::Throttling(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for CreateSimulationJobError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreateSimulationJobError::IdempotentParameterMismatch(ref cause) => {
                write!(f, "{}", cause)
            }
            CreateSimulationJobError::InternalServer(ref cause) => write!(f, "{}", cause),
            CreateSimulationJobError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            CreateSimulationJobError::LimitExceeded(ref cause) => write!(f, "{}", cause),
            CreateSimulationJobError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            CreateSimulationJobError::ServiceUnavailable(ref cause) => write!(f, "{}", cause),
            CreateSimulationJobError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CreateSimulationJobError {}
/// Errors returned by CreateWorldExportJob
#[derive(Debug, PartialEq)]
pub enum CreateWorldExportJobError {
    /// <p>The request uses the same client token as a previous, but non-identical request. Do not reuse a client token with different requests, unless the requests are identical. </p>
    IdempotentParameterMismatch(String),
    /// <p>AWS RoboMaker experienced a service issue. Try your call again.</p>
    InternalServer(String),
    /// <p>A parameter specified in a request is not valid, is unsupported, or cannot be used. The returned message provides an explanation of the error value.</p>
    InvalidParameter(String),
    /// <p>The specified resource does not exist.</p>
    ResourceNotFound(String),
    /// <p>The request has failed due to a temporary failure of the server.</p>
    ServiceUnavailable(String),
    /// <p>AWS RoboMaker is temporarily unable to process the request. Try your call again.</p>
    Throttling(String),
}

impl CreateWorldExportJobError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateWorldExportJobError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            #[allow(clippy::single_match)]
            match err.typ.as_str() {
                "IdempotentParameterMismatchException" => {
                    return RusotoError::Service(
                        CreateWorldExportJobError::IdempotentParameterMismatch(err.msg),
                    )
                }
                "InternalServerException" => {
                    return RusotoError::Service(CreateWorldExportJobError::InternalServer(err.msg))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(CreateWorldExportJobError::InvalidParameter(
                        err.msg,
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(CreateWorldExportJobError::ResourceNotFound(
                        err.msg,
                    ))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(CreateWorldExportJobError::ServiceUnavailable(
                        err.msg,
                    ))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(CreateWorldExportJobError::Throttling(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for CreateWorldExportJobError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreateWorldExportJobError::IdempotentParameterMismatch(ref cause) => {
                write!(f, "{}", cause)
            }
            CreateWorldExportJobError::InternalServer(ref cause) => write!(f, "{}", cause),
            CreateWorldExportJobError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            CreateWorldExportJobError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            CreateWorldExportJobError::ServiceUnavailable(ref cause) => write!(f, "{}", cause),
            CreateWorldExportJobError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CreateWorldExportJobError {}
/// Errors returned by CreateWorldGenerationJob
#[derive(Debug, PartialEq)]
pub enum CreateWorldGenerationJobError {
    /// <p>The request uses the same client token as a previous, but non-identical request. Do not reuse a client token with different requests, unless the requests are identical. </p>
    IdempotentParameterMismatch(String),
    /// <p>AWS RoboMaker experienced a service issue. Try your call again.</p>
    InternalServer(String),
    /// <p>A parameter specified in a request is not valid, is unsupported, or cannot be used. The returned message provides an explanation of the error value.</p>
    InvalidParameter(String),
    /// <p>The requested resource exceeds the maximum number allowed, or the number of concurrent stream requests exceeds the maximum number allowed. </p>
    LimitExceeded(String),
    /// <p>The specified resource does not exist.</p>
    ResourceNotFound(String),
    /// <p>The request has failed due to a temporary failure of the server.</p>
    ServiceUnavailable(String),
    /// <p>AWS RoboMaker is temporarily unable to process the request. Try your call again.</p>
    Throttling(String),
}

impl CreateWorldGenerationJobError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateWorldGenerationJobError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            #[allow(clippy::single_match)]
            match err.typ.as_str() {
                "IdempotentParameterMismatchException" => {
                    return RusotoError::Service(
                        CreateWorldGenerationJobError::IdempotentParameterMismatch(err.msg),
                    )
                }
                "InternalServerException" => {
                    return RusotoError::Service(CreateWorldGenerationJobError::InternalServer(
                        err.msg,
                    ))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(CreateWorldGenerationJobError::InvalidParameter(
                        err.msg,
                    ))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(CreateWorldGenerationJobError::LimitExceeded(
                        err.msg,
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(CreateWorldGenerationJobError::ResourceNotFound(
                        err.msg,
                    ))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(CreateWorldGenerationJobError::ServiceUnavailable(
                        err.msg,
                    ))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(CreateWorldGenerationJobError::Throttling(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for CreateWorldGenerationJobError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreateWorldGenerationJobError::IdempotentParameterMismatch(ref cause) => {
                write!(f, "{}", cause)
            }
            CreateWorldGenerationJobError::InternalServer(ref cause) => write!(f, "{}", cause),
            CreateWorldGenerationJobError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            CreateWorldGenerationJobError::LimitExceeded(ref cause) => write!(f, "{}", cause),
            CreateWorldGenerationJobError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            CreateWorldGenerationJobError::ServiceUnavailable(ref cause) => write!(f, "{}", cause),
            CreateWorldGenerationJobError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CreateWorldGenerationJobError {}
/// Errors returned by CreateWorldTemplate
#[derive(Debug, PartialEq)]
pub enum CreateWorldTemplateError {
    /// <p>AWS RoboMaker experienced a service issue. Try your call again.</p>
    InternalServer(String),
    /// <p>A parameter specified in a request is not valid, is unsupported, or cannot be used. The returned message provides an explanation of the error value.</p>
    InvalidParameter(String),
    /// <p>The requested resource exceeds the maximum number allowed, or the number of concurrent stream requests exceeds the maximum number allowed. </p>
    LimitExceeded(String),
    /// <p>The specified resource already exists.</p>
    ResourceAlreadyExists(String),
    /// <p>The specified resource does not exist.</p>
    ResourceNotFound(String),
    /// <p>AWS RoboMaker is temporarily unable to process the request. Try your call again.</p>
    Throttling(String),
}

impl CreateWorldTemplateError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateWorldTemplateError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            #[allow(clippy::single_match)]
            match err.typ.as_str() {
                "InternalServerException" => {
                    return RusotoError::Service(CreateWorldTemplateError::InternalServer(err.msg))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(CreateWorldTemplateError::InvalidParameter(
                        err.msg,
                    ))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(CreateWorldTemplateError::LimitExceeded(err.msg))
                }
                "ResourceAlreadyExistsException" => {
                    return RusotoError::Service(CreateWorldTemplateError::ResourceAlreadyExists(
                        err.msg,
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(CreateWorldTemplateError::ResourceNotFound(
                        err.msg,
                    ))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(CreateWorldTemplateError::Throttling(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for CreateWorldTemplateError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreateWorldTemplateError::InternalServer(ref cause) => write!(f, "{}", cause),
            CreateWorldTemplateError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            CreateWorldTemplateError::LimitExceeded(ref cause) => write!(f, "{}", cause),
            CreateWorldTemplateError::ResourceAlreadyExists(ref cause) => write!(f, "{}", cause),
            CreateWorldTemplateError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            CreateWorldTemplateError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CreateWorldTemplateError {}
/// Errors returned by DeleteFleet
#[derive(Debug, PartialEq)]
pub enum DeleteFleetError {
    /// <p>AWS RoboMaker experienced a service issue. Try your call again.</p>
    InternalServer(String),
    /// <p>A parameter specified in a request is not valid, is unsupported, or cannot be used. The returned message provides an explanation of the error value.</p>
    InvalidParameter(String),
    /// <p>AWS RoboMaker is temporarily unable to process the request. Try your call again.</p>
    Throttling(String),
}

impl DeleteFleetError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteFleetError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            #[allow(clippy::single_match)]
            match err.typ.as_str() {
                "InternalServerException" => {
                    return RusotoError::Service(DeleteFleetError::InternalServer(err.msg))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(DeleteFleetError::InvalidParameter(err.msg))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(DeleteFleetError::Throttling(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DeleteFleetError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteFleetError::InternalServer(ref cause) => write!(f, "{}", cause),
            DeleteFleetError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            DeleteFleetError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeleteFleetError {}
/// Errors returned by DeleteRobot
#[derive(Debug, PartialEq)]
pub enum DeleteRobotError {
    /// <p>AWS RoboMaker experienced a service issue. Try your call again.</p>
    InternalServer(String),
    /// <p>A parameter specified in a request is not valid, is unsupported, or cannot be used. The returned message provides an explanation of the error value.</p>
    InvalidParameter(String),
    /// <p>AWS RoboMaker is temporarily unable to process the request. Try your call again.</p>
    Throttling(String),
}

impl DeleteRobotError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteRobotError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            #[allow(clippy::single_match)]
            match err.typ.as_str() {
                "InternalServerException" => {
                    return RusotoError::Service(DeleteRobotError::InternalServer(err.msg))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(DeleteRobotError::InvalidParameter(err.msg))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(DeleteRobotError::Throttling(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DeleteRobotError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteRobotError::InternalServer(ref cause) => write!(f, "{}", cause),
            DeleteRobotError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            DeleteRobotError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeleteRobotError {}
/// Errors returned by DeleteRobotApplication
#[derive(Debug, PartialEq)]
pub enum DeleteRobotApplicationError {
    /// <p>AWS RoboMaker experienced a service issue. Try your call again.</p>
    InternalServer(String),
    /// <p>A parameter specified in a request is not valid, is unsupported, or cannot be used. The returned message provides an explanation of the error value.</p>
    InvalidParameter(String),
    /// <p>AWS RoboMaker is temporarily unable to process the request. Try your call again.</p>
    Throttling(String),
}

impl DeleteRobotApplicationError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteRobotApplicationError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            #[allow(clippy::single_match)]
            match err.typ.as_str() {
                "InternalServerException" => {
                    return RusotoError::Service(DeleteRobotApplicationError::InternalServer(
                        err.msg,
                    ))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(DeleteRobotApplicationError::InvalidParameter(
                        err.msg,
                    ))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(DeleteRobotApplicationError::Throttling(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DeleteRobotApplicationError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteRobotApplicationError::InternalServer(ref cause) => write!(f, "{}", cause),
            DeleteRobotApplicationError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            DeleteRobotApplicationError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeleteRobotApplicationError {}
/// Errors returned by DeleteSimulationApplication
#[derive(Debug, PartialEq)]
pub enum DeleteSimulationApplicationError {
    /// <p>AWS RoboMaker experienced a service issue. Try your call again.</p>
    InternalServer(String),
    /// <p>A parameter specified in a request is not valid, is unsupported, or cannot be used. The returned message provides an explanation of the error value.</p>
    InvalidParameter(String),
    /// <p>AWS RoboMaker is temporarily unable to process the request. Try your call again.</p>
    Throttling(String),
}

impl DeleteSimulationApplicationError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<DeleteSimulationApplicationError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            #[allow(clippy::single_match)]
            match err.typ.as_str() {
                "InternalServerException" => {
                    return RusotoError::Service(DeleteSimulationApplicationError::InternalServer(
                        err.msg,
                    ))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(
                        DeleteSimulationApplicationError::InvalidParameter(err.msg),
                    )
                }
                "ThrottlingException" => {
                    return RusotoError::Service(DeleteSimulationApplicationError::Throttling(
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
impl fmt::Display for DeleteSimulationApplicationError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteSimulationApplicationError::InternalServer(ref cause) => write!(f, "{}", cause),
            DeleteSimulationApplicationError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            DeleteSimulationApplicationError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeleteSimulationApplicationError {}
/// Errors returned by DeleteWorldTemplate
#[derive(Debug, PartialEq)]
pub enum DeleteWorldTemplateError {
    /// <p>AWS RoboMaker experienced a service issue. Try your call again.</p>
    InternalServer(String),
    /// <p>A parameter specified in a request is not valid, is unsupported, or cannot be used. The returned message provides an explanation of the error value.</p>
    InvalidParameter(String),
    /// <p>The specified resource does not exist.</p>
    ResourceNotFound(String),
    /// <p>AWS RoboMaker is temporarily unable to process the request. Try your call again.</p>
    Throttling(String),
}

impl DeleteWorldTemplateError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteWorldTemplateError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            #[allow(clippy::single_match)]
            match err.typ.as_str() {
                "InternalServerException" => {
                    return RusotoError::Service(DeleteWorldTemplateError::InternalServer(err.msg))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(DeleteWorldTemplateError::InvalidParameter(
                        err.msg,
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(DeleteWorldTemplateError::ResourceNotFound(
                        err.msg,
                    ))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(DeleteWorldTemplateError::Throttling(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DeleteWorldTemplateError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteWorldTemplateError::InternalServer(ref cause) => write!(f, "{}", cause),
            DeleteWorldTemplateError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            DeleteWorldTemplateError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            DeleteWorldTemplateError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeleteWorldTemplateError {}
/// Errors returned by DeregisterRobot
#[derive(Debug, PartialEq)]
pub enum DeregisterRobotError {
    /// <p>AWS RoboMaker experienced a service issue. Try your call again.</p>
    InternalServer(String),
    /// <p>A parameter specified in a request is not valid, is unsupported, or cannot be used. The returned message provides an explanation of the error value.</p>
    InvalidParameter(String),
    /// <p>The specified resource does not exist.</p>
    ResourceNotFound(String),
    /// <p>AWS RoboMaker is temporarily unable to process the request. Try your call again.</p>
    Throttling(String),
}

impl DeregisterRobotError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeregisterRobotError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            #[allow(clippy::single_match)]
            match err.typ.as_str() {
                "InternalServerException" => {
                    return RusotoError::Service(DeregisterRobotError::InternalServer(err.msg))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(DeregisterRobotError::InvalidParameter(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(DeregisterRobotError::ResourceNotFound(err.msg))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(DeregisterRobotError::Throttling(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DeregisterRobotError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeregisterRobotError::InternalServer(ref cause) => write!(f, "{}", cause),
            DeregisterRobotError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            DeregisterRobotError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            DeregisterRobotError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeregisterRobotError {}
/// Errors returned by DescribeDeploymentJob
#[derive(Debug, PartialEq)]
pub enum DescribeDeploymentJobError {
    /// <p>AWS RoboMaker experienced a service issue. Try your call again.</p>
    InternalServer(String),
    /// <p>A parameter specified in a request is not valid, is unsupported, or cannot be used. The returned message provides an explanation of the error value.</p>
    InvalidParameter(String),
    /// <p>The specified resource does not exist.</p>
    ResourceNotFound(String),
    /// <p>AWS RoboMaker is temporarily unable to process the request. Try your call again.</p>
    Throttling(String),
}

impl DescribeDeploymentJobError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeDeploymentJobError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            #[allow(clippy::single_match)]
            match err.typ.as_str() {
                "InternalServerException" => {
                    return RusotoError::Service(DescribeDeploymentJobError::InternalServer(
                        err.msg,
                    ))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(DescribeDeploymentJobError::InvalidParameter(
                        err.msg,
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(DescribeDeploymentJobError::ResourceNotFound(
                        err.msg,
                    ))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(DescribeDeploymentJobError::Throttling(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DescribeDeploymentJobError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeDeploymentJobError::InternalServer(ref cause) => write!(f, "{}", cause),
            DescribeDeploymentJobError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            DescribeDeploymentJobError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            DescribeDeploymentJobError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DescribeDeploymentJobError {}
/// Errors returned by DescribeFleet
#[derive(Debug, PartialEq)]
pub enum DescribeFleetError {
    /// <p>AWS RoboMaker experienced a service issue. Try your call again.</p>
    InternalServer(String),
    /// <p>A parameter specified in a request is not valid, is unsupported, or cannot be used. The returned message provides an explanation of the error value.</p>
    InvalidParameter(String),
    /// <p>The specified resource does not exist.</p>
    ResourceNotFound(String),
    /// <p>AWS RoboMaker is temporarily unable to process the request. Try your call again.</p>
    Throttling(String),
}

impl DescribeFleetError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeFleetError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            #[allow(clippy::single_match)]
            match err.typ.as_str() {
                "InternalServerException" => {
                    return RusotoError::Service(DescribeFleetError::InternalServer(err.msg))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(DescribeFleetError::InvalidParameter(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(DescribeFleetError::ResourceNotFound(err.msg))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(DescribeFleetError::Throttling(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DescribeFleetError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeFleetError::InternalServer(ref cause) => write!(f, "{}", cause),
            DescribeFleetError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            DescribeFleetError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            DescribeFleetError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DescribeFleetError {}
/// Errors returned by DescribeRobot
#[derive(Debug, PartialEq)]
pub enum DescribeRobotError {
    /// <p>AWS RoboMaker experienced a service issue. Try your call again.</p>
    InternalServer(String),
    /// <p>A parameter specified in a request is not valid, is unsupported, or cannot be used. The returned message provides an explanation of the error value.</p>
    InvalidParameter(String),
    /// <p>The specified resource does not exist.</p>
    ResourceNotFound(String),
    /// <p>AWS RoboMaker is temporarily unable to process the request. Try your call again.</p>
    Throttling(String),
}

impl DescribeRobotError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeRobotError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            #[allow(clippy::single_match)]
            match err.typ.as_str() {
                "InternalServerException" => {
                    return RusotoError::Service(DescribeRobotError::InternalServer(err.msg))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(DescribeRobotError::InvalidParameter(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(DescribeRobotError::ResourceNotFound(err.msg))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(DescribeRobotError::Throttling(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DescribeRobotError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeRobotError::InternalServer(ref cause) => write!(f, "{}", cause),
            DescribeRobotError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            DescribeRobotError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            DescribeRobotError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DescribeRobotError {}
/// Errors returned by DescribeRobotApplication
#[derive(Debug, PartialEq)]
pub enum DescribeRobotApplicationError {
    /// <p>AWS RoboMaker experienced a service issue. Try your call again.</p>
    InternalServer(String),
    /// <p>A parameter specified in a request is not valid, is unsupported, or cannot be used. The returned message provides an explanation of the error value.</p>
    InvalidParameter(String),
    /// <p>The specified resource does not exist.</p>
    ResourceNotFound(String),
    /// <p>AWS RoboMaker is temporarily unable to process the request. Try your call again.</p>
    Throttling(String),
}

impl DescribeRobotApplicationError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeRobotApplicationError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            #[allow(clippy::single_match)]
            match err.typ.as_str() {
                "InternalServerException" => {
                    return RusotoError::Service(DescribeRobotApplicationError::InternalServer(
                        err.msg,
                    ))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(DescribeRobotApplicationError::InvalidParameter(
                        err.msg,
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(DescribeRobotApplicationError::ResourceNotFound(
                        err.msg,
                    ))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(DescribeRobotApplicationError::Throttling(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DescribeRobotApplicationError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeRobotApplicationError::InternalServer(ref cause) => write!(f, "{}", cause),
            DescribeRobotApplicationError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            DescribeRobotApplicationError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            DescribeRobotApplicationError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DescribeRobotApplicationError {}
/// Errors returned by DescribeSimulationApplication
#[derive(Debug, PartialEq)]
pub enum DescribeSimulationApplicationError {
    /// <p>AWS RoboMaker experienced a service issue. Try your call again.</p>
    InternalServer(String),
    /// <p>A parameter specified in a request is not valid, is unsupported, or cannot be used. The returned message provides an explanation of the error value.</p>
    InvalidParameter(String),
    /// <p>The specified resource does not exist.</p>
    ResourceNotFound(String),
    /// <p>AWS RoboMaker is temporarily unable to process the request. Try your call again.</p>
    Throttling(String),
}

impl DescribeSimulationApplicationError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<DescribeSimulationApplicationError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            #[allow(clippy::single_match)]
            match err.typ.as_str() {
                "InternalServerException" => {
                    return RusotoError::Service(
                        DescribeSimulationApplicationError::InternalServer(err.msg),
                    )
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(
                        DescribeSimulationApplicationError::InvalidParameter(err.msg),
                    )
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(
                        DescribeSimulationApplicationError::ResourceNotFound(err.msg),
                    )
                }
                "ThrottlingException" => {
                    return RusotoError::Service(DescribeSimulationApplicationError::Throttling(
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
impl fmt::Display for DescribeSimulationApplicationError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeSimulationApplicationError::InternalServer(ref cause) => write!(f, "{}", cause),
            DescribeSimulationApplicationError::InvalidParameter(ref cause) => {
                write!(f, "{}", cause)
            }
            DescribeSimulationApplicationError::ResourceNotFound(ref cause) => {
                write!(f, "{}", cause)
            }
            DescribeSimulationApplicationError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DescribeSimulationApplicationError {}
/// Errors returned by DescribeSimulationJob
#[derive(Debug, PartialEq)]
pub enum DescribeSimulationJobError {
    /// <p>AWS RoboMaker experienced a service issue. Try your call again.</p>
    InternalServer(String),
    /// <p>A parameter specified in a request is not valid, is unsupported, or cannot be used. The returned message provides an explanation of the error value.</p>
    InvalidParameter(String),
    /// <p>The specified resource does not exist.</p>
    ResourceNotFound(String),
    /// <p>AWS RoboMaker is temporarily unable to process the request. Try your call again.</p>
    Throttling(String),
}

impl DescribeSimulationJobError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeSimulationJobError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            #[allow(clippy::single_match)]
            match err.typ.as_str() {
                "InternalServerException" => {
                    return RusotoError::Service(DescribeSimulationJobError::InternalServer(
                        err.msg,
                    ))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(DescribeSimulationJobError::InvalidParameter(
                        err.msg,
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(DescribeSimulationJobError::ResourceNotFound(
                        err.msg,
                    ))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(DescribeSimulationJobError::Throttling(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DescribeSimulationJobError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeSimulationJobError::InternalServer(ref cause) => write!(f, "{}", cause),
            DescribeSimulationJobError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            DescribeSimulationJobError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            DescribeSimulationJobError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DescribeSimulationJobError {}
/// Errors returned by DescribeSimulationJobBatch
#[derive(Debug, PartialEq)]
pub enum DescribeSimulationJobBatchError {
    /// <p>AWS RoboMaker experienced a service issue. Try your call again.</p>
    InternalServer(String),
    /// <p>A parameter specified in a request is not valid, is unsupported, or cannot be used. The returned message provides an explanation of the error value.</p>
    InvalidParameter(String),
    /// <p>The specified resource does not exist.</p>
    ResourceNotFound(String),
}

impl DescribeSimulationJobBatchError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<DescribeSimulationJobBatchError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            #[allow(clippy::single_match)]
            match err.typ.as_str() {
                "InternalServerException" => {
                    return RusotoError::Service(DescribeSimulationJobBatchError::InternalServer(
                        err.msg,
                    ))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(DescribeSimulationJobBatchError::InvalidParameter(
                        err.msg,
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(DescribeSimulationJobBatchError::ResourceNotFound(
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
impl fmt::Display for DescribeSimulationJobBatchError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeSimulationJobBatchError::InternalServer(ref cause) => write!(f, "{}", cause),
            DescribeSimulationJobBatchError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            DescribeSimulationJobBatchError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DescribeSimulationJobBatchError {}
/// Errors returned by DescribeWorld
#[derive(Debug, PartialEq)]
pub enum DescribeWorldError {
    /// <p>AWS RoboMaker experienced a service issue. Try your call again.</p>
    InternalServer(String),
    /// <p>A parameter specified in a request is not valid, is unsupported, or cannot be used. The returned message provides an explanation of the error value.</p>
    InvalidParameter(String),
    /// <p>The specified resource does not exist.</p>
    ResourceNotFound(String),
    /// <p>AWS RoboMaker is temporarily unable to process the request. Try your call again.</p>
    Throttling(String),
}

impl DescribeWorldError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeWorldError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            #[allow(clippy::single_match)]
            match err.typ.as_str() {
                "InternalServerException" => {
                    return RusotoError::Service(DescribeWorldError::InternalServer(err.msg))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(DescribeWorldError::InvalidParameter(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(DescribeWorldError::ResourceNotFound(err.msg))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(DescribeWorldError::Throttling(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DescribeWorldError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeWorldError::InternalServer(ref cause) => write!(f, "{}", cause),
            DescribeWorldError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            DescribeWorldError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            DescribeWorldError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DescribeWorldError {}
/// Errors returned by DescribeWorldExportJob
#[derive(Debug, PartialEq)]
pub enum DescribeWorldExportJobError {
    /// <p>AWS RoboMaker experienced a service issue. Try your call again.</p>
    InternalServer(String),
    /// <p>A parameter specified in a request is not valid, is unsupported, or cannot be used. The returned message provides an explanation of the error value.</p>
    InvalidParameter(String),
    /// <p>The specified resource does not exist.</p>
    ResourceNotFound(String),
    /// <p>AWS RoboMaker is temporarily unable to process the request. Try your call again.</p>
    Throttling(String),
}

impl DescribeWorldExportJobError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeWorldExportJobError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            #[allow(clippy::single_match)]
            match err.typ.as_str() {
                "InternalServerException" => {
                    return RusotoError::Service(DescribeWorldExportJobError::InternalServer(
                        err.msg,
                    ))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(DescribeWorldExportJobError::InvalidParameter(
                        err.msg,
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(DescribeWorldExportJobError::ResourceNotFound(
                        err.msg,
                    ))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(DescribeWorldExportJobError::Throttling(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DescribeWorldExportJobError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeWorldExportJobError::InternalServer(ref cause) => write!(f, "{}", cause),
            DescribeWorldExportJobError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            DescribeWorldExportJobError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            DescribeWorldExportJobError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DescribeWorldExportJobError {}
/// Errors returned by DescribeWorldGenerationJob
#[derive(Debug, PartialEq)]
pub enum DescribeWorldGenerationJobError {
    /// <p>AWS RoboMaker experienced a service issue. Try your call again.</p>
    InternalServer(String),
    /// <p>A parameter specified in a request is not valid, is unsupported, or cannot be used. The returned message provides an explanation of the error value.</p>
    InvalidParameter(String),
    /// <p>The specified resource does not exist.</p>
    ResourceNotFound(String),
    /// <p>AWS RoboMaker is temporarily unable to process the request. Try your call again.</p>
    Throttling(String),
}

impl DescribeWorldGenerationJobError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<DescribeWorldGenerationJobError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            #[allow(clippy::single_match)]
            match err.typ.as_str() {
                "InternalServerException" => {
                    return RusotoError::Service(DescribeWorldGenerationJobError::InternalServer(
                        err.msg,
                    ))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(DescribeWorldGenerationJobError::InvalidParameter(
                        err.msg,
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(DescribeWorldGenerationJobError::ResourceNotFound(
                        err.msg,
                    ))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(DescribeWorldGenerationJobError::Throttling(
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
impl fmt::Display for DescribeWorldGenerationJobError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeWorldGenerationJobError::InternalServer(ref cause) => write!(f, "{}", cause),
            DescribeWorldGenerationJobError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            DescribeWorldGenerationJobError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            DescribeWorldGenerationJobError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DescribeWorldGenerationJobError {}
/// Errors returned by DescribeWorldTemplate
#[derive(Debug, PartialEq)]
pub enum DescribeWorldTemplateError {
    /// <p>AWS RoboMaker experienced a service issue. Try your call again.</p>
    InternalServer(String),
    /// <p>A parameter specified in a request is not valid, is unsupported, or cannot be used. The returned message provides an explanation of the error value.</p>
    InvalidParameter(String),
    /// <p>The specified resource does not exist.</p>
    ResourceNotFound(String),
    /// <p>AWS RoboMaker is temporarily unable to process the request. Try your call again.</p>
    Throttling(String),
}

impl DescribeWorldTemplateError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeWorldTemplateError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            #[allow(clippy::single_match)]
            match err.typ.as_str() {
                "InternalServerException" => {
                    return RusotoError::Service(DescribeWorldTemplateError::InternalServer(
                        err.msg,
                    ))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(DescribeWorldTemplateError::InvalidParameter(
                        err.msg,
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(DescribeWorldTemplateError::ResourceNotFound(
                        err.msg,
                    ))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(DescribeWorldTemplateError::Throttling(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DescribeWorldTemplateError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeWorldTemplateError::InternalServer(ref cause) => write!(f, "{}", cause),
            DescribeWorldTemplateError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            DescribeWorldTemplateError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            DescribeWorldTemplateError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DescribeWorldTemplateError {}
/// Errors returned by GetWorldTemplateBody
#[derive(Debug, PartialEq)]
pub enum GetWorldTemplateBodyError {
    /// <p>AWS RoboMaker experienced a service issue. Try your call again.</p>
    InternalServer(String),
    /// <p>A parameter specified in a request is not valid, is unsupported, or cannot be used. The returned message provides an explanation of the error value.</p>
    InvalidParameter(String),
    /// <p>The specified resource does not exist.</p>
    ResourceNotFound(String),
    /// <p>AWS RoboMaker is temporarily unable to process the request. Try your call again.</p>
    Throttling(String),
}

impl GetWorldTemplateBodyError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetWorldTemplateBodyError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            #[allow(clippy::single_match)]
            match err.typ.as_str() {
                "InternalServerException" => {
                    return RusotoError::Service(GetWorldTemplateBodyError::InternalServer(err.msg))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(GetWorldTemplateBodyError::InvalidParameter(
                        err.msg,
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(GetWorldTemplateBodyError::ResourceNotFound(
                        err.msg,
                    ))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(GetWorldTemplateBodyError::Throttling(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for GetWorldTemplateBodyError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetWorldTemplateBodyError::InternalServer(ref cause) => write!(f, "{}", cause),
            GetWorldTemplateBodyError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            GetWorldTemplateBodyError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            GetWorldTemplateBodyError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetWorldTemplateBodyError {}
/// Errors returned by ListDeploymentJobs
#[derive(Debug, PartialEq)]
pub enum ListDeploymentJobsError {
    /// <p>AWS RoboMaker experienced a service issue. Try your call again.</p>
    InternalServer(String),
    /// <p>A parameter specified in a request is not valid, is unsupported, or cannot be used. The returned message provides an explanation of the error value.</p>
    InvalidParameter(String),
    /// <p>The specified resource does not exist.</p>
    ResourceNotFound(String),
    /// <p>AWS RoboMaker is temporarily unable to process the request. Try your call again.</p>
    Throttling(String),
}

impl ListDeploymentJobsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListDeploymentJobsError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            #[allow(clippy::single_match)]
            match err.typ.as_str() {
                "InternalServerException" => {
                    return RusotoError::Service(ListDeploymentJobsError::InternalServer(err.msg))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(ListDeploymentJobsError::InvalidParameter(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(ListDeploymentJobsError::ResourceNotFound(err.msg))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(ListDeploymentJobsError::Throttling(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ListDeploymentJobsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListDeploymentJobsError::InternalServer(ref cause) => write!(f, "{}", cause),
            ListDeploymentJobsError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            ListDeploymentJobsError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            ListDeploymentJobsError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListDeploymentJobsError {}
/// Errors returned by ListFleets
#[derive(Debug, PartialEq)]
pub enum ListFleetsError {
    /// <p>AWS RoboMaker experienced a service issue. Try your call again.</p>
    InternalServer(String),
    /// <p>A parameter specified in a request is not valid, is unsupported, or cannot be used. The returned message provides an explanation of the error value.</p>
    InvalidParameter(String),
    /// <p>The specified resource does not exist.</p>
    ResourceNotFound(String),
    /// <p>AWS RoboMaker is temporarily unable to process the request. Try your call again.</p>
    Throttling(String),
}

impl ListFleetsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListFleetsError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            #[allow(clippy::single_match)]
            match err.typ.as_str() {
                "InternalServerException" => {
                    return RusotoError::Service(ListFleetsError::InternalServer(err.msg))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(ListFleetsError::InvalidParameter(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(ListFleetsError::ResourceNotFound(err.msg))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(ListFleetsError::Throttling(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ListFleetsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListFleetsError::InternalServer(ref cause) => write!(f, "{}", cause),
            ListFleetsError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            ListFleetsError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            ListFleetsError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListFleetsError {}
/// Errors returned by ListRobotApplications
#[derive(Debug, PartialEq)]
pub enum ListRobotApplicationsError {
    /// <p>AWS RoboMaker experienced a service issue. Try your call again.</p>
    InternalServer(String),
    /// <p>A parameter specified in a request is not valid, is unsupported, or cannot be used. The returned message provides an explanation of the error value.</p>
    InvalidParameter(String),
    /// <p>AWS RoboMaker is temporarily unable to process the request. Try your call again.</p>
    Throttling(String),
}

impl ListRobotApplicationsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListRobotApplicationsError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            #[allow(clippy::single_match)]
            match err.typ.as_str() {
                "InternalServerException" => {
                    return RusotoError::Service(ListRobotApplicationsError::InternalServer(
                        err.msg,
                    ))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(ListRobotApplicationsError::InvalidParameter(
                        err.msg,
                    ))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(ListRobotApplicationsError::Throttling(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ListRobotApplicationsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListRobotApplicationsError::InternalServer(ref cause) => write!(f, "{}", cause),
            ListRobotApplicationsError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            ListRobotApplicationsError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListRobotApplicationsError {}
/// Errors returned by ListRobots
#[derive(Debug, PartialEq)]
pub enum ListRobotsError {
    /// <p>AWS RoboMaker experienced a service issue. Try your call again.</p>
    InternalServer(String),
    /// <p>A parameter specified in a request is not valid, is unsupported, or cannot be used. The returned message provides an explanation of the error value.</p>
    InvalidParameter(String),
    /// <p>The specified resource does not exist.</p>
    ResourceNotFound(String),
    /// <p>AWS RoboMaker is temporarily unable to process the request. Try your call again.</p>
    Throttling(String),
}

impl ListRobotsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListRobotsError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            #[allow(clippy::single_match)]
            match err.typ.as_str() {
                "InternalServerException" => {
                    return RusotoError::Service(ListRobotsError::InternalServer(err.msg))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(ListRobotsError::InvalidParameter(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(ListRobotsError::ResourceNotFound(err.msg))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(ListRobotsError::Throttling(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ListRobotsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListRobotsError::InternalServer(ref cause) => write!(f, "{}", cause),
            ListRobotsError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            ListRobotsError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            ListRobotsError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListRobotsError {}
/// Errors returned by ListSimulationApplications
#[derive(Debug, PartialEq)]
pub enum ListSimulationApplicationsError {
    /// <p>AWS RoboMaker experienced a service issue. Try your call again.</p>
    InternalServer(String),
    /// <p>A parameter specified in a request is not valid, is unsupported, or cannot be used. The returned message provides an explanation of the error value.</p>
    InvalidParameter(String),
    /// <p>AWS RoboMaker is temporarily unable to process the request. Try your call again.</p>
    Throttling(String),
}

impl ListSimulationApplicationsError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<ListSimulationApplicationsError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            #[allow(clippy::single_match)]
            match err.typ.as_str() {
                "InternalServerException" => {
                    return RusotoError::Service(ListSimulationApplicationsError::InternalServer(
                        err.msg,
                    ))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(ListSimulationApplicationsError::InvalidParameter(
                        err.msg,
                    ))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(ListSimulationApplicationsError::Throttling(
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
impl fmt::Display for ListSimulationApplicationsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListSimulationApplicationsError::InternalServer(ref cause) => write!(f, "{}", cause),
            ListSimulationApplicationsError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            ListSimulationApplicationsError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListSimulationApplicationsError {}
/// Errors returned by ListSimulationJobBatches
#[derive(Debug, PartialEq)]
pub enum ListSimulationJobBatchesError {
    /// <p>AWS RoboMaker experienced a service issue. Try your call again.</p>
    InternalServer(String),
    /// <p>A parameter specified in a request is not valid, is unsupported, or cannot be used. The returned message provides an explanation of the error value.</p>
    InvalidParameter(String),
}

impl ListSimulationJobBatchesError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListSimulationJobBatchesError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            #[allow(clippy::single_match)]
            match err.typ.as_str() {
                "InternalServerException" => {
                    return RusotoError::Service(ListSimulationJobBatchesError::InternalServer(
                        err.msg,
                    ))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(ListSimulationJobBatchesError::InvalidParameter(
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
impl fmt::Display for ListSimulationJobBatchesError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListSimulationJobBatchesError::InternalServer(ref cause) => write!(f, "{}", cause),
            ListSimulationJobBatchesError::InvalidParameter(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListSimulationJobBatchesError {}
/// Errors returned by ListSimulationJobs
#[derive(Debug, PartialEq)]
pub enum ListSimulationJobsError {
    /// <p>AWS RoboMaker experienced a service issue. Try your call again.</p>
    InternalServer(String),
    /// <p>A parameter specified in a request is not valid, is unsupported, or cannot be used. The returned message provides an explanation of the error value.</p>
    InvalidParameter(String),
    /// <p>AWS RoboMaker is temporarily unable to process the request. Try your call again.</p>
    Throttling(String),
}

impl ListSimulationJobsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListSimulationJobsError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            #[allow(clippy::single_match)]
            match err.typ.as_str() {
                "InternalServerException" => {
                    return RusotoError::Service(ListSimulationJobsError::InternalServer(err.msg))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(ListSimulationJobsError::InvalidParameter(err.msg))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(ListSimulationJobsError::Throttling(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ListSimulationJobsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListSimulationJobsError::InternalServer(ref cause) => write!(f, "{}", cause),
            ListSimulationJobsError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            ListSimulationJobsError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListSimulationJobsError {}
/// Errors returned by ListTagsForResource
#[derive(Debug, PartialEq)]
pub enum ListTagsForResourceError {
    /// <p>AWS RoboMaker experienced a service issue. Try your call again.</p>
    InternalServer(String),
    /// <p>A parameter specified in a request is not valid, is unsupported, or cannot be used. The returned message provides an explanation of the error value.</p>
    InvalidParameter(String),
    /// <p>The specified resource does not exist.</p>
    ResourceNotFound(String),
    /// <p>AWS RoboMaker is temporarily unable to process the request. Try your call again.</p>
    Throttling(String),
}

impl ListTagsForResourceError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListTagsForResourceError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            #[allow(clippy::single_match)]
            match err.typ.as_str() {
                "InternalServerException" => {
                    return RusotoError::Service(ListTagsForResourceError::InternalServer(err.msg))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(ListTagsForResourceError::InvalidParameter(
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
            ListTagsForResourceError::InternalServer(ref cause) => write!(f, "{}", cause),
            ListTagsForResourceError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            ListTagsForResourceError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            ListTagsForResourceError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListTagsForResourceError {}
/// Errors returned by ListWorldExportJobs
#[derive(Debug, PartialEq)]
pub enum ListWorldExportJobsError {
    /// <p>AWS RoboMaker experienced a service issue. Try your call again.</p>
    InternalServer(String),
    /// <p>A parameter specified in a request is not valid, is unsupported, or cannot be used. The returned message provides an explanation of the error value.</p>
    InvalidParameter(String),
    /// <p>AWS RoboMaker is temporarily unable to process the request. Try your call again.</p>
    Throttling(String),
}

impl ListWorldExportJobsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListWorldExportJobsError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            #[allow(clippy::single_match)]
            match err.typ.as_str() {
                "InternalServerException" => {
                    return RusotoError::Service(ListWorldExportJobsError::InternalServer(err.msg))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(ListWorldExportJobsError::InvalidParameter(
                        err.msg,
                    ))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(ListWorldExportJobsError::Throttling(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ListWorldExportJobsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListWorldExportJobsError::InternalServer(ref cause) => write!(f, "{}", cause),
            ListWorldExportJobsError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            ListWorldExportJobsError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListWorldExportJobsError {}
/// Errors returned by ListWorldGenerationJobs
#[derive(Debug, PartialEq)]
pub enum ListWorldGenerationJobsError {
    /// <p>AWS RoboMaker experienced a service issue. Try your call again.</p>
    InternalServer(String),
    /// <p>A parameter specified in a request is not valid, is unsupported, or cannot be used. The returned message provides an explanation of the error value.</p>
    InvalidParameter(String),
    /// <p>AWS RoboMaker is temporarily unable to process the request. Try your call again.</p>
    Throttling(String),
}

impl ListWorldGenerationJobsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListWorldGenerationJobsError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            #[allow(clippy::single_match)]
            match err.typ.as_str() {
                "InternalServerException" => {
                    return RusotoError::Service(ListWorldGenerationJobsError::InternalServer(
                        err.msg,
                    ))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(ListWorldGenerationJobsError::InvalidParameter(
                        err.msg,
                    ))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(ListWorldGenerationJobsError::Throttling(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ListWorldGenerationJobsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListWorldGenerationJobsError::InternalServer(ref cause) => write!(f, "{}", cause),
            ListWorldGenerationJobsError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            ListWorldGenerationJobsError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListWorldGenerationJobsError {}
/// Errors returned by ListWorldTemplates
#[derive(Debug, PartialEq)]
pub enum ListWorldTemplatesError {
    /// <p>AWS RoboMaker experienced a service issue. Try your call again.</p>
    InternalServer(String),
    /// <p>A parameter specified in a request is not valid, is unsupported, or cannot be used. The returned message provides an explanation of the error value.</p>
    InvalidParameter(String),
    /// <p>AWS RoboMaker is temporarily unable to process the request. Try your call again.</p>
    Throttling(String),
}

impl ListWorldTemplatesError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListWorldTemplatesError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            #[allow(clippy::single_match)]
            match err.typ.as_str() {
                "InternalServerException" => {
                    return RusotoError::Service(ListWorldTemplatesError::InternalServer(err.msg))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(ListWorldTemplatesError::InvalidParameter(err.msg))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(ListWorldTemplatesError::Throttling(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ListWorldTemplatesError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListWorldTemplatesError::InternalServer(ref cause) => write!(f, "{}", cause),
            ListWorldTemplatesError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            ListWorldTemplatesError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListWorldTemplatesError {}
/// Errors returned by ListWorlds
#[derive(Debug, PartialEq)]
pub enum ListWorldsError {
    /// <p>AWS RoboMaker experienced a service issue. Try your call again.</p>
    InternalServer(String),
    /// <p>A parameter specified in a request is not valid, is unsupported, or cannot be used. The returned message provides an explanation of the error value.</p>
    InvalidParameter(String),
    /// <p>AWS RoboMaker is temporarily unable to process the request. Try your call again.</p>
    Throttling(String),
}

impl ListWorldsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListWorldsError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            #[allow(clippy::single_match)]
            match err.typ.as_str() {
                "InternalServerException" => {
                    return RusotoError::Service(ListWorldsError::InternalServer(err.msg))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(ListWorldsError::InvalidParameter(err.msg))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(ListWorldsError::Throttling(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ListWorldsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListWorldsError::InternalServer(ref cause) => write!(f, "{}", cause),
            ListWorldsError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            ListWorldsError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListWorldsError {}
/// Errors returned by RegisterRobot
#[derive(Debug, PartialEq)]
pub enum RegisterRobotError {
    /// <p>AWS RoboMaker experienced a service issue. Try your call again.</p>
    InternalServer(String),
    /// <p>A parameter specified in a request is not valid, is unsupported, or cannot be used. The returned message provides an explanation of the error value.</p>
    InvalidParameter(String),
    /// <p>The requested resource exceeds the maximum number allowed, or the number of concurrent stream requests exceeds the maximum number allowed. </p>
    LimitExceeded(String),
    /// <p>The specified resource does not exist.</p>
    ResourceNotFound(String),
    /// <p>AWS RoboMaker is temporarily unable to process the request. Try your call again.</p>
    Throttling(String),
}

impl RegisterRobotError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<RegisterRobotError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            #[allow(clippy::single_match)]
            match err.typ.as_str() {
                "InternalServerException" => {
                    return RusotoError::Service(RegisterRobotError::InternalServer(err.msg))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(RegisterRobotError::InvalidParameter(err.msg))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(RegisterRobotError::LimitExceeded(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(RegisterRobotError::ResourceNotFound(err.msg))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(RegisterRobotError::Throttling(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for RegisterRobotError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            RegisterRobotError::InternalServer(ref cause) => write!(f, "{}", cause),
            RegisterRobotError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            RegisterRobotError::LimitExceeded(ref cause) => write!(f, "{}", cause),
            RegisterRobotError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            RegisterRobotError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for RegisterRobotError {}
/// Errors returned by RestartSimulationJob
#[derive(Debug, PartialEq)]
pub enum RestartSimulationJobError {
    /// <p>AWS RoboMaker experienced a service issue. Try your call again.</p>
    InternalServer(String),
    /// <p>A parameter specified in a request is not valid, is unsupported, or cannot be used. The returned message provides an explanation of the error value.</p>
    InvalidParameter(String),
    /// <p>The requested resource exceeds the maximum number allowed, or the number of concurrent stream requests exceeds the maximum number allowed. </p>
    LimitExceeded(String),
    /// <p>The specified resource does not exist.</p>
    ResourceNotFound(String),
    /// <p>AWS RoboMaker is temporarily unable to process the request. Try your call again.</p>
    Throttling(String),
}

impl RestartSimulationJobError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<RestartSimulationJobError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            #[allow(clippy::single_match)]
            match err.typ.as_str() {
                "InternalServerException" => {
                    return RusotoError::Service(RestartSimulationJobError::InternalServer(err.msg))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(RestartSimulationJobError::InvalidParameter(
                        err.msg,
                    ))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(RestartSimulationJobError::LimitExceeded(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(RestartSimulationJobError::ResourceNotFound(
                        err.msg,
                    ))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(RestartSimulationJobError::Throttling(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for RestartSimulationJobError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            RestartSimulationJobError::InternalServer(ref cause) => write!(f, "{}", cause),
            RestartSimulationJobError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            RestartSimulationJobError::LimitExceeded(ref cause) => write!(f, "{}", cause),
            RestartSimulationJobError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            RestartSimulationJobError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for RestartSimulationJobError {}
/// Errors returned by StartSimulationJobBatch
#[derive(Debug, PartialEq)]
pub enum StartSimulationJobBatchError {
    /// <p>The request uses the same client token as a previous, but non-identical request. Do not reuse a client token with different requests, unless the requests are identical. </p>
    IdempotentParameterMismatch(String),
    /// <p>AWS RoboMaker experienced a service issue. Try your call again.</p>
    InternalServer(String),
    /// <p>A parameter specified in a request is not valid, is unsupported, or cannot be used. The returned message provides an explanation of the error value.</p>
    InvalidParameter(String),
    /// <p>The requested resource exceeds the maximum number allowed, or the number of concurrent stream requests exceeds the maximum number allowed. </p>
    LimitExceeded(String),
    /// <p>AWS RoboMaker is temporarily unable to process the request. Try your call again.</p>
    Throttling(String),
}

impl StartSimulationJobBatchError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<StartSimulationJobBatchError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            #[allow(clippy::single_match)]
            match err.typ.as_str() {
                "IdempotentParameterMismatchException" => {
                    return RusotoError::Service(
                        StartSimulationJobBatchError::IdempotentParameterMismatch(err.msg),
                    )
                }
                "InternalServerException" => {
                    return RusotoError::Service(StartSimulationJobBatchError::InternalServer(
                        err.msg,
                    ))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(StartSimulationJobBatchError::InvalidParameter(
                        err.msg,
                    ))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(StartSimulationJobBatchError::LimitExceeded(
                        err.msg,
                    ))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(StartSimulationJobBatchError::Throttling(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for StartSimulationJobBatchError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            StartSimulationJobBatchError::IdempotentParameterMismatch(ref cause) => {
                write!(f, "{}", cause)
            }
            StartSimulationJobBatchError::InternalServer(ref cause) => write!(f, "{}", cause),
            StartSimulationJobBatchError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            StartSimulationJobBatchError::LimitExceeded(ref cause) => write!(f, "{}", cause),
            StartSimulationJobBatchError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for StartSimulationJobBatchError {}
/// Errors returned by SyncDeploymentJob
#[derive(Debug, PartialEq)]
pub enum SyncDeploymentJobError {
    /// <p>The failure percentage threshold percentage was met.</p>
    ConcurrentDeployment(String),
    /// <p>The request uses the same client token as a previous, but non-identical request. Do not reuse a client token with different requests, unless the requests are identical. </p>
    IdempotentParameterMismatch(String),
    /// <p>AWS RoboMaker experienced a service issue. Try your call again.</p>
    InternalServer(String),
    /// <p>A parameter specified in a request is not valid, is unsupported, or cannot be used. The returned message provides an explanation of the error value.</p>
    InvalidParameter(String),
    /// <p>The requested resource exceeds the maximum number allowed, or the number of concurrent stream requests exceeds the maximum number allowed. </p>
    LimitExceeded(String),
    /// <p>The specified resource does not exist.</p>
    ResourceNotFound(String),
    /// <p>AWS RoboMaker is temporarily unable to process the request. Try your call again.</p>
    Throttling(String),
}

impl SyncDeploymentJobError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<SyncDeploymentJobError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            #[allow(clippy::single_match)]
            match err.typ.as_str() {
                "ConcurrentDeploymentException" => {
                    return RusotoError::Service(SyncDeploymentJobError::ConcurrentDeployment(
                        err.msg,
                    ))
                }
                "IdempotentParameterMismatchException" => {
                    return RusotoError::Service(
                        SyncDeploymentJobError::IdempotentParameterMismatch(err.msg),
                    )
                }
                "InternalServerException" => {
                    return RusotoError::Service(SyncDeploymentJobError::InternalServer(err.msg))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(SyncDeploymentJobError::InvalidParameter(err.msg))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(SyncDeploymentJobError::LimitExceeded(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(SyncDeploymentJobError::ResourceNotFound(err.msg))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(SyncDeploymentJobError::Throttling(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for SyncDeploymentJobError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            SyncDeploymentJobError::ConcurrentDeployment(ref cause) => write!(f, "{}", cause),
            SyncDeploymentJobError::IdempotentParameterMismatch(ref cause) => {
                write!(f, "{}", cause)
            }
            SyncDeploymentJobError::InternalServer(ref cause) => write!(f, "{}", cause),
            SyncDeploymentJobError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            SyncDeploymentJobError::LimitExceeded(ref cause) => write!(f, "{}", cause),
            SyncDeploymentJobError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            SyncDeploymentJobError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for SyncDeploymentJobError {}
/// Errors returned by TagResource
#[derive(Debug, PartialEq)]
pub enum TagResourceError {
    /// <p>AWS RoboMaker experienced a service issue. Try your call again.</p>
    InternalServer(String),
    /// <p>A parameter specified in a request is not valid, is unsupported, or cannot be used. The returned message provides an explanation of the error value.</p>
    InvalidParameter(String),
    /// <p>The specified resource does not exist.</p>
    ResourceNotFound(String),
    /// <p>AWS RoboMaker is temporarily unable to process the request. Try your call again.</p>
    Throttling(String),
}

impl TagResourceError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<TagResourceError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            #[allow(clippy::single_match)]
            match err.typ.as_str() {
                "InternalServerException" => {
                    return RusotoError::Service(TagResourceError::InternalServer(err.msg))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(TagResourceError::InvalidParameter(err.msg))
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
            TagResourceError::InternalServer(ref cause) => write!(f, "{}", cause),
            TagResourceError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            TagResourceError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            TagResourceError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for TagResourceError {}
/// Errors returned by UntagResource
#[derive(Debug, PartialEq)]
pub enum UntagResourceError {
    /// <p>AWS RoboMaker experienced a service issue. Try your call again.</p>
    InternalServer(String),
    /// <p>A parameter specified in a request is not valid, is unsupported, or cannot be used. The returned message provides an explanation of the error value.</p>
    InvalidParameter(String),
    /// <p>The specified resource does not exist.</p>
    ResourceNotFound(String),
    /// <p>AWS RoboMaker is temporarily unable to process the request. Try your call again.</p>
    Throttling(String),
}

impl UntagResourceError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UntagResourceError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            #[allow(clippy::single_match)]
            match err.typ.as_str() {
                "InternalServerException" => {
                    return RusotoError::Service(UntagResourceError::InternalServer(err.msg))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(UntagResourceError::InvalidParameter(err.msg))
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
            UntagResourceError::InternalServer(ref cause) => write!(f, "{}", cause),
            UntagResourceError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            UntagResourceError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            UntagResourceError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for UntagResourceError {}
/// Errors returned by UpdateRobotApplication
#[derive(Debug, PartialEq)]
pub enum UpdateRobotApplicationError {
    /// <p>AWS RoboMaker experienced a service issue. Try your call again.</p>
    InternalServer(String),
    /// <p>A parameter specified in a request is not valid, is unsupported, or cannot be used. The returned message provides an explanation of the error value.</p>
    InvalidParameter(String),
    /// <p>The requested resource exceeds the maximum number allowed, or the number of concurrent stream requests exceeds the maximum number allowed. </p>
    LimitExceeded(String),
    /// <p>The specified resource does not exist.</p>
    ResourceNotFound(String),
    /// <p>AWS RoboMaker is temporarily unable to process the request. Try your call again.</p>
    Throttling(String),
}

impl UpdateRobotApplicationError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdateRobotApplicationError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            #[allow(clippy::single_match)]
            match err.typ.as_str() {
                "InternalServerException" => {
                    return RusotoError::Service(UpdateRobotApplicationError::InternalServer(
                        err.msg,
                    ))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(UpdateRobotApplicationError::InvalidParameter(
                        err.msg,
                    ))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(UpdateRobotApplicationError::LimitExceeded(
                        err.msg,
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(UpdateRobotApplicationError::ResourceNotFound(
                        err.msg,
                    ))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(UpdateRobotApplicationError::Throttling(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for UpdateRobotApplicationError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UpdateRobotApplicationError::InternalServer(ref cause) => write!(f, "{}", cause),
            UpdateRobotApplicationError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            UpdateRobotApplicationError::LimitExceeded(ref cause) => write!(f, "{}", cause),
            UpdateRobotApplicationError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            UpdateRobotApplicationError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for UpdateRobotApplicationError {}
/// Errors returned by UpdateSimulationApplication
#[derive(Debug, PartialEq)]
pub enum UpdateSimulationApplicationError {
    /// <p>AWS RoboMaker experienced a service issue. Try your call again.</p>
    InternalServer(String),
    /// <p>A parameter specified in a request is not valid, is unsupported, or cannot be used. The returned message provides an explanation of the error value.</p>
    InvalidParameter(String),
    /// <p>The requested resource exceeds the maximum number allowed, or the number of concurrent stream requests exceeds the maximum number allowed. </p>
    LimitExceeded(String),
    /// <p>The specified resource does not exist.</p>
    ResourceNotFound(String),
    /// <p>AWS RoboMaker is temporarily unable to process the request. Try your call again.</p>
    Throttling(String),
}

impl UpdateSimulationApplicationError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<UpdateSimulationApplicationError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            #[allow(clippy::single_match)]
            match err.typ.as_str() {
                "InternalServerException" => {
                    return RusotoError::Service(UpdateSimulationApplicationError::InternalServer(
                        err.msg,
                    ))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(
                        UpdateSimulationApplicationError::InvalidParameter(err.msg),
                    )
                }
                "LimitExceededException" => {
                    return RusotoError::Service(UpdateSimulationApplicationError::LimitExceeded(
                        err.msg,
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(
                        UpdateSimulationApplicationError::ResourceNotFound(err.msg),
                    )
                }
                "ThrottlingException" => {
                    return RusotoError::Service(UpdateSimulationApplicationError::Throttling(
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
impl fmt::Display for UpdateSimulationApplicationError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UpdateSimulationApplicationError::InternalServer(ref cause) => write!(f, "{}", cause),
            UpdateSimulationApplicationError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            UpdateSimulationApplicationError::LimitExceeded(ref cause) => write!(f, "{}", cause),
            UpdateSimulationApplicationError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            UpdateSimulationApplicationError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for UpdateSimulationApplicationError {}
/// Errors returned by UpdateWorldTemplate
#[derive(Debug, PartialEq)]
pub enum UpdateWorldTemplateError {
    /// <p>AWS RoboMaker experienced a service issue. Try your call again.</p>
    InternalServer(String),
    /// <p>A parameter specified in a request is not valid, is unsupported, or cannot be used. The returned message provides an explanation of the error value.</p>
    InvalidParameter(String),
    /// <p>The specified resource does not exist.</p>
    ResourceNotFound(String),
    /// <p>AWS RoboMaker is temporarily unable to process the request. Try your call again.</p>
    Throttling(String),
}

impl UpdateWorldTemplateError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdateWorldTemplateError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            #[allow(clippy::single_match)]
            match err.typ.as_str() {
                "InternalServerException" => {
                    return RusotoError::Service(UpdateWorldTemplateError::InternalServer(err.msg))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(UpdateWorldTemplateError::InvalidParameter(
                        err.msg,
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(UpdateWorldTemplateError::ResourceNotFound(
                        err.msg,
                    ))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(UpdateWorldTemplateError::Throttling(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for UpdateWorldTemplateError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UpdateWorldTemplateError::InternalServer(ref cause) => write!(f, "{}", cause),
            UpdateWorldTemplateError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            UpdateWorldTemplateError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            UpdateWorldTemplateError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for UpdateWorldTemplateError {}
/// Trait representing the capabilities of the RoboMaker API. RoboMaker clients implement this trait.
#[async_trait]
pub trait Robomaker {
    /// <p>Deletes one or more worlds in a batch operation.</p>
    async fn batch_delete_worlds(
        &self,
        input: BatchDeleteWorldsRequest,
    ) -> Result<BatchDeleteWorldsResponse, RusotoError<BatchDeleteWorldsError>>;

    /// <p>Describes one or more simulation jobs.</p>
    async fn batch_describe_simulation_job(
        &self,
        input: BatchDescribeSimulationJobRequest,
    ) -> Result<BatchDescribeSimulationJobResponse, RusotoError<BatchDescribeSimulationJobError>>;

    /// <p>Cancels the specified deployment job.</p>
    async fn cancel_deployment_job(
        &self,
        input: CancelDeploymentJobRequest,
    ) -> Result<CancelDeploymentJobResponse, RusotoError<CancelDeploymentJobError>>;

    /// <p>Cancels the specified simulation job.</p>
    async fn cancel_simulation_job(
        &self,
        input: CancelSimulationJobRequest,
    ) -> Result<CancelSimulationJobResponse, RusotoError<CancelSimulationJobError>>;

    /// <p>Cancels a simulation job batch. When you cancel a simulation job batch, you are also cancelling all of the active simulation jobs created as part of the batch. </p>
    async fn cancel_simulation_job_batch(
        &self,
        input: CancelSimulationJobBatchRequest,
    ) -> Result<CancelSimulationJobBatchResponse, RusotoError<CancelSimulationJobBatchError>>;

    /// <p>Cancels the specified export job.</p>
    async fn cancel_world_export_job(
        &self,
        input: CancelWorldExportJobRequest,
    ) -> Result<CancelWorldExportJobResponse, RusotoError<CancelWorldExportJobError>>;

    /// <p>Cancels the specified world generator job.</p>
    async fn cancel_world_generation_job(
        &self,
        input: CancelWorldGenerationJobRequest,
    ) -> Result<CancelWorldGenerationJobResponse, RusotoError<CancelWorldGenerationJobError>>;

    /// <p><p>Deploys a specific version of a robot application to robots in a fleet.</p> <p>The robot application must have a numbered <code>applicationVersion</code> for consistency reasons. To create a new version, use <code>CreateRobotApplicationVersion</code> or see <a href="https://docs.aws.amazon.com/robomaker/latest/dg/create-robot-application-version.html">Creating a Robot Application Version</a>. </p> <note> <p>After 90 days, deployment jobs expire and will be deleted. They will no longer be accessible. </p> </note></p>
    async fn create_deployment_job(
        &self,
        input: CreateDeploymentJobRequest,
    ) -> Result<CreateDeploymentJobResponse, RusotoError<CreateDeploymentJobError>>;

    /// <p>Creates a fleet, a logical group of robots running the same robot application.</p>
    async fn create_fleet(
        &self,
        input: CreateFleetRequest,
    ) -> Result<CreateFleetResponse, RusotoError<CreateFleetError>>;

    /// <p>Creates a robot.</p>
    async fn create_robot(
        &self,
        input: CreateRobotRequest,
    ) -> Result<CreateRobotResponse, RusotoError<CreateRobotError>>;

    /// <p>Creates a robot application. </p>
    async fn create_robot_application(
        &self,
        input: CreateRobotApplicationRequest,
    ) -> Result<CreateRobotApplicationResponse, RusotoError<CreateRobotApplicationError>>;

    /// <p>Creates a version of a robot application.</p>
    async fn create_robot_application_version(
        &self,
        input: CreateRobotApplicationVersionRequest,
    ) -> Result<
        CreateRobotApplicationVersionResponse,
        RusotoError<CreateRobotApplicationVersionError>,
    >;

    /// <p>Creates a simulation application.</p>
    async fn create_simulation_application(
        &self,
        input: CreateSimulationApplicationRequest,
    ) -> Result<CreateSimulationApplicationResponse, RusotoError<CreateSimulationApplicationError>>;

    /// <p>Creates a simulation application with a specific revision id.</p>
    async fn create_simulation_application_version(
        &self,
        input: CreateSimulationApplicationVersionRequest,
    ) -> Result<
        CreateSimulationApplicationVersionResponse,
        RusotoError<CreateSimulationApplicationVersionError>,
    >;

    /// <p><p>Creates a simulation job.</p> <note> <p>After 90 days, simulation jobs expire and will be deleted. They will no longer be accessible. </p> </note></p>
    async fn create_simulation_job(
        &self,
        input: CreateSimulationJobRequest,
    ) -> Result<CreateSimulationJobResponse, RusotoError<CreateSimulationJobError>>;

    /// <p>Creates a world export job.</p>
    async fn create_world_export_job(
        &self,
        input: CreateWorldExportJobRequest,
    ) -> Result<CreateWorldExportJobResponse, RusotoError<CreateWorldExportJobError>>;

    /// <p>Creates worlds using the specified template.</p>
    async fn create_world_generation_job(
        &self,
        input: CreateWorldGenerationJobRequest,
    ) -> Result<CreateWorldGenerationJobResponse, RusotoError<CreateWorldGenerationJobError>>;

    /// <p>Creates a world template.</p>
    async fn create_world_template(
        &self,
        input: CreateWorldTemplateRequest,
    ) -> Result<CreateWorldTemplateResponse, RusotoError<CreateWorldTemplateError>>;

    /// <p>Deletes a fleet.</p>
    async fn delete_fleet(
        &self,
        input: DeleteFleetRequest,
    ) -> Result<DeleteFleetResponse, RusotoError<DeleteFleetError>>;

    /// <p>Deletes a robot.</p>
    async fn delete_robot(
        &self,
        input: DeleteRobotRequest,
    ) -> Result<DeleteRobotResponse, RusotoError<DeleteRobotError>>;

    /// <p>Deletes a robot application.</p>
    async fn delete_robot_application(
        &self,
        input: DeleteRobotApplicationRequest,
    ) -> Result<DeleteRobotApplicationResponse, RusotoError<DeleteRobotApplicationError>>;

    /// <p>Deletes a simulation application.</p>
    async fn delete_simulation_application(
        &self,
        input: DeleteSimulationApplicationRequest,
    ) -> Result<DeleteSimulationApplicationResponse, RusotoError<DeleteSimulationApplicationError>>;

    /// <p>Deletes a world template.</p>
    async fn delete_world_template(
        &self,
        input: DeleteWorldTemplateRequest,
    ) -> Result<DeleteWorldTemplateResponse, RusotoError<DeleteWorldTemplateError>>;

    /// <p>Deregisters a robot.</p>
    async fn deregister_robot(
        &self,
        input: DeregisterRobotRequest,
    ) -> Result<DeregisterRobotResponse, RusotoError<DeregisterRobotError>>;

    /// <p>Describes a deployment job.</p>
    async fn describe_deployment_job(
        &self,
        input: DescribeDeploymentJobRequest,
    ) -> Result<DescribeDeploymentJobResponse, RusotoError<DescribeDeploymentJobError>>;

    /// <p>Describes a fleet.</p>
    async fn describe_fleet(
        &self,
        input: DescribeFleetRequest,
    ) -> Result<DescribeFleetResponse, RusotoError<DescribeFleetError>>;

    /// <p>Describes a robot.</p>
    async fn describe_robot(
        &self,
        input: DescribeRobotRequest,
    ) -> Result<DescribeRobotResponse, RusotoError<DescribeRobotError>>;

    /// <p>Describes a robot application.</p>
    async fn describe_robot_application(
        &self,
        input: DescribeRobotApplicationRequest,
    ) -> Result<DescribeRobotApplicationResponse, RusotoError<DescribeRobotApplicationError>>;

    /// <p>Describes a simulation application.</p>
    async fn describe_simulation_application(
        &self,
        input: DescribeSimulationApplicationRequest,
    ) -> Result<
        DescribeSimulationApplicationResponse,
        RusotoError<DescribeSimulationApplicationError>,
    >;

    /// <p>Describes a simulation job.</p>
    async fn describe_simulation_job(
        &self,
        input: DescribeSimulationJobRequest,
    ) -> Result<DescribeSimulationJobResponse, RusotoError<DescribeSimulationJobError>>;

    /// <p>Describes a simulation job batch.</p>
    async fn describe_simulation_job_batch(
        &self,
        input: DescribeSimulationJobBatchRequest,
    ) -> Result<DescribeSimulationJobBatchResponse, RusotoError<DescribeSimulationJobBatchError>>;

    /// <p>Describes a world.</p>
    async fn describe_world(
        &self,
        input: DescribeWorldRequest,
    ) -> Result<DescribeWorldResponse, RusotoError<DescribeWorldError>>;

    /// <p>Describes a world export job.</p>
    async fn describe_world_export_job(
        &self,
        input: DescribeWorldExportJobRequest,
    ) -> Result<DescribeWorldExportJobResponse, RusotoError<DescribeWorldExportJobError>>;

    /// <p>Describes a world generation job.</p>
    async fn describe_world_generation_job(
        &self,
        input: DescribeWorldGenerationJobRequest,
    ) -> Result<DescribeWorldGenerationJobResponse, RusotoError<DescribeWorldGenerationJobError>>;

    /// <p>Describes a world template.</p>
    async fn describe_world_template(
        &self,
        input: DescribeWorldTemplateRequest,
    ) -> Result<DescribeWorldTemplateResponse, RusotoError<DescribeWorldTemplateError>>;

    /// <p>Gets the world template body.</p>
    async fn get_world_template_body(
        &self,
        input: GetWorldTemplateBodyRequest,
    ) -> Result<GetWorldTemplateBodyResponse, RusotoError<GetWorldTemplateBodyError>>;

    /// <p>Returns a list of deployment jobs for a fleet. You can optionally provide filters to retrieve specific deployment jobs. </p>
    async fn list_deployment_jobs(
        &self,
        input: ListDeploymentJobsRequest,
    ) -> Result<ListDeploymentJobsResponse, RusotoError<ListDeploymentJobsError>>;

    /// <p>Returns a list of fleets. You can optionally provide filters to retrieve specific fleets. </p>
    async fn list_fleets(
        &self,
        input: ListFleetsRequest,
    ) -> Result<ListFleetsResponse, RusotoError<ListFleetsError>>;

    /// <p>Returns a list of robot application. You can optionally provide filters to retrieve specific robot applications.</p>
    async fn list_robot_applications(
        &self,
        input: ListRobotApplicationsRequest,
    ) -> Result<ListRobotApplicationsResponse, RusotoError<ListRobotApplicationsError>>;

    /// <p>Returns a list of robots. You can optionally provide filters to retrieve specific robots.</p>
    async fn list_robots(
        &self,
        input: ListRobotsRequest,
    ) -> Result<ListRobotsResponse, RusotoError<ListRobotsError>>;

    /// <p>Returns a list of simulation applications. You can optionally provide filters to retrieve specific simulation applications. </p>
    async fn list_simulation_applications(
        &self,
        input: ListSimulationApplicationsRequest,
    ) -> Result<ListSimulationApplicationsResponse, RusotoError<ListSimulationApplicationsError>>;

    /// <p>Returns a list simulation job batches. You can optionally provide filters to retrieve specific simulation batch jobs. </p>
    async fn list_simulation_job_batches(
        &self,
        input: ListSimulationJobBatchesRequest,
    ) -> Result<ListSimulationJobBatchesResponse, RusotoError<ListSimulationJobBatchesError>>;

    /// <p>Returns a list of simulation jobs. You can optionally provide filters to retrieve specific simulation jobs. </p>
    async fn list_simulation_jobs(
        &self,
        input: ListSimulationJobsRequest,
    ) -> Result<ListSimulationJobsResponse, RusotoError<ListSimulationJobsError>>;

    /// <p>Lists all tags on a AWS RoboMaker resource.</p>
    async fn list_tags_for_resource(
        &self,
        input: ListTagsForResourceRequest,
    ) -> Result<ListTagsForResourceResponse, RusotoError<ListTagsForResourceError>>;

    /// <p>Lists world export jobs.</p>
    async fn list_world_export_jobs(
        &self,
        input: ListWorldExportJobsRequest,
    ) -> Result<ListWorldExportJobsResponse, RusotoError<ListWorldExportJobsError>>;

    /// <p>Lists world generator jobs.</p>
    async fn list_world_generation_jobs(
        &self,
        input: ListWorldGenerationJobsRequest,
    ) -> Result<ListWorldGenerationJobsResponse, RusotoError<ListWorldGenerationJobsError>>;

    /// <p>Lists world templates.</p>
    async fn list_world_templates(
        &self,
        input: ListWorldTemplatesRequest,
    ) -> Result<ListWorldTemplatesResponse, RusotoError<ListWorldTemplatesError>>;

    /// <p>Lists worlds.</p>
    async fn list_worlds(
        &self,
        input: ListWorldsRequest,
    ) -> Result<ListWorldsResponse, RusotoError<ListWorldsError>>;

    /// <p>Registers a robot with a fleet.</p>
    async fn register_robot(
        &self,
        input: RegisterRobotRequest,
    ) -> Result<RegisterRobotResponse, RusotoError<RegisterRobotError>>;

    /// <p>Restarts a running simulation job.</p>
    async fn restart_simulation_job(
        &self,
        input: RestartSimulationJobRequest,
    ) -> Result<RestartSimulationJobResponse, RusotoError<RestartSimulationJobError>>;

    /// <p>Starts a new simulation job batch. The batch is defined using one or more <code>SimulationJobRequest</code> objects. </p>
    async fn start_simulation_job_batch(
        &self,
        input: StartSimulationJobBatchRequest,
    ) -> Result<StartSimulationJobBatchResponse, RusotoError<StartSimulationJobBatchError>>;

    /// <p>Syncrhonizes robots in a fleet to the latest deployment. This is helpful if robots were added after a deployment.</p>
    async fn sync_deployment_job(
        &self,
        input: SyncDeploymentJobRequest,
    ) -> Result<SyncDeploymentJobResponse, RusotoError<SyncDeploymentJobError>>;

    /// <p>Adds or edits tags for a AWS RoboMaker resource.</p> <p>Each tag consists of a tag key and a tag value. Tag keys and tag values are both required, but tag values can be empty strings. </p> <p>For information about the rules that apply to tag keys and tag values, see <a href="https://docs.aws.amazon.com/awsaccountbilling/latest/aboutv2/allocation-tag-restrictions.html">User-Defined Tag Restrictions</a> in the <i>AWS Billing and Cost Management User Guide</i>. </p>
    async fn tag_resource(
        &self,
        input: TagResourceRequest,
    ) -> Result<TagResourceResponse, RusotoError<TagResourceError>>;

    /// <p>Removes the specified tags from the specified AWS RoboMaker resource.</p> <p>To remove a tag, specify the tag key. To change the tag value of an existing tag key, use <a href="https://docs.aws.amazon.com/robomaker/latest/dg/API_TagResource.html"> <code>TagResource</code> </a>. </p>
    async fn untag_resource(
        &self,
        input: UntagResourceRequest,
    ) -> Result<UntagResourceResponse, RusotoError<UntagResourceError>>;

    /// <p>Updates a robot application.</p>
    async fn update_robot_application(
        &self,
        input: UpdateRobotApplicationRequest,
    ) -> Result<UpdateRobotApplicationResponse, RusotoError<UpdateRobotApplicationError>>;

    /// <p>Updates a simulation application.</p>
    async fn update_simulation_application(
        &self,
        input: UpdateSimulationApplicationRequest,
    ) -> Result<UpdateSimulationApplicationResponse, RusotoError<UpdateSimulationApplicationError>>;

    /// <p>Updates a world template.</p>
    async fn update_world_template(
        &self,
        input: UpdateWorldTemplateRequest,
    ) -> Result<UpdateWorldTemplateResponse, RusotoError<UpdateWorldTemplateError>>;
}
/// A client for the RoboMaker API.
#[derive(Clone)]
pub struct RobomakerClient {
    client: Client,
    region: region::Region,
}

impl RobomakerClient {
    /// Creates a client backed by the default tokio event loop.
    ///
    /// The client will use the default credentials provider and tls client.
    pub fn new(region: region::Region) -> RobomakerClient {
        RobomakerClient {
            client: Client::shared(),
            region,
        }
    }

    pub fn new_with<P, D>(
        request_dispatcher: D,
        credentials_provider: P,
        region: region::Region,
    ) -> RobomakerClient
    where
        P: ProvideAwsCredentials + Send + Sync + 'static,
        D: DispatchSignedRequest + Send + Sync + 'static,
    {
        RobomakerClient {
            client: Client::new_with(credentials_provider, request_dispatcher),
            region,
        }
    }

    pub fn new_with_client(client: Client, region: region::Region) -> RobomakerClient {
        RobomakerClient { client, region }
    }
}

#[async_trait]
impl Robomaker for RobomakerClient {
    /// <p>Deletes one or more worlds in a batch operation.</p>
    #[allow(unused_mut)]
    async fn batch_delete_worlds(
        &self,
        input: BatchDeleteWorldsRequest,
    ) -> Result<BatchDeleteWorldsResponse, RusotoError<BatchDeleteWorldsError>> {
        #![allow(clippy::needless_update)]
        let request_uri = "/batchDeleteWorlds";

        let mut request = SignedRequest::new("POST", "robomaker", &self.region, &request_uri);
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
                .deserialize::<BatchDeleteWorldsResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(BatchDeleteWorldsError::from_response(response))
        }
    }

    /// <p>Describes one or more simulation jobs.</p>
    #[allow(unused_mut)]
    async fn batch_describe_simulation_job(
        &self,
        input: BatchDescribeSimulationJobRequest,
    ) -> Result<BatchDescribeSimulationJobResponse, RusotoError<BatchDescribeSimulationJobError>>
    {
        #![allow(clippy::needless_update)]
        let request_uri = "/batchDescribeSimulationJob";

        let mut request = SignedRequest::new("POST", "robomaker", &self.region, &request_uri);
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
                .deserialize::<BatchDescribeSimulationJobResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(BatchDescribeSimulationJobError::from_response(response))
        }
    }

    /// <p>Cancels the specified deployment job.</p>
    #[allow(unused_mut)]
    async fn cancel_deployment_job(
        &self,
        input: CancelDeploymentJobRequest,
    ) -> Result<CancelDeploymentJobResponse, RusotoError<CancelDeploymentJobError>> {
        #![allow(clippy::needless_update)]
        let request_uri = "/cancelDeploymentJob";

        let mut request = SignedRequest::new("POST", "robomaker", &self.region, &request_uri);
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
                .deserialize::<CancelDeploymentJobResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(CancelDeploymentJobError::from_response(response))
        }
    }

    /// <p>Cancels the specified simulation job.</p>
    #[allow(unused_mut)]
    async fn cancel_simulation_job(
        &self,
        input: CancelSimulationJobRequest,
    ) -> Result<CancelSimulationJobResponse, RusotoError<CancelSimulationJobError>> {
        #![allow(clippy::needless_update)]
        let request_uri = "/cancelSimulationJob";

        let mut request = SignedRequest::new("POST", "robomaker", &self.region, &request_uri);
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
                .deserialize::<CancelSimulationJobResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(CancelSimulationJobError::from_response(response))
        }
    }

    /// <p>Cancels a simulation job batch. When you cancel a simulation job batch, you are also cancelling all of the active simulation jobs created as part of the batch. </p>
    #[allow(unused_mut)]
    async fn cancel_simulation_job_batch(
        &self,
        input: CancelSimulationJobBatchRequest,
    ) -> Result<CancelSimulationJobBatchResponse, RusotoError<CancelSimulationJobBatchError>> {
        #![allow(clippy::needless_update)]
        let request_uri = "/cancelSimulationJobBatch";

        let mut request = SignedRequest::new("POST", "robomaker", &self.region, &request_uri);
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
                .deserialize::<CancelSimulationJobBatchResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(CancelSimulationJobBatchError::from_response(response))
        }
    }

    /// <p>Cancels the specified export job.</p>
    #[allow(unused_mut)]
    async fn cancel_world_export_job(
        &self,
        input: CancelWorldExportJobRequest,
    ) -> Result<CancelWorldExportJobResponse, RusotoError<CancelWorldExportJobError>> {
        #![allow(clippy::needless_update)]
        let request_uri = "/cancelWorldExportJob";

        let mut request = SignedRequest::new("POST", "robomaker", &self.region, &request_uri);
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
                .deserialize::<CancelWorldExportJobResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(CancelWorldExportJobError::from_response(response))
        }
    }

    /// <p>Cancels the specified world generator job.</p>
    #[allow(unused_mut)]
    async fn cancel_world_generation_job(
        &self,
        input: CancelWorldGenerationJobRequest,
    ) -> Result<CancelWorldGenerationJobResponse, RusotoError<CancelWorldGenerationJobError>> {
        #![allow(clippy::needless_update)]
        let request_uri = "/cancelWorldGenerationJob";

        let mut request = SignedRequest::new("POST", "robomaker", &self.region, &request_uri);
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
                .deserialize::<CancelWorldGenerationJobResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(CancelWorldGenerationJobError::from_response(response))
        }
    }

    /// <p><p>Deploys a specific version of a robot application to robots in a fleet.</p> <p>The robot application must have a numbered <code>applicationVersion</code> for consistency reasons. To create a new version, use <code>CreateRobotApplicationVersion</code> or see <a href="https://docs.aws.amazon.com/robomaker/latest/dg/create-robot-application-version.html">Creating a Robot Application Version</a>. </p> <note> <p>After 90 days, deployment jobs expire and will be deleted. They will no longer be accessible. </p> </note></p>
    #[allow(unused_mut)]
    async fn create_deployment_job(
        &self,
        input: CreateDeploymentJobRequest,
    ) -> Result<CreateDeploymentJobResponse, RusotoError<CreateDeploymentJobError>> {
        #![allow(clippy::needless_update)]
        let request_uri = "/createDeploymentJob";

        let mut request = SignedRequest::new("POST", "robomaker", &self.region, &request_uri);
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
                .deserialize::<CreateDeploymentJobResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(CreateDeploymentJobError::from_response(response))
        }
    }

    /// <p>Creates a fleet, a logical group of robots running the same robot application.</p>
    #[allow(unused_mut)]
    async fn create_fleet(
        &self,
        input: CreateFleetRequest,
    ) -> Result<CreateFleetResponse, RusotoError<CreateFleetError>> {
        #![allow(clippy::needless_update)]
        let request_uri = "/createFleet";

        let mut request = SignedRequest::new("POST", "robomaker", &self.region, &request_uri);
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
                .deserialize::<CreateFleetResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(CreateFleetError::from_response(response))
        }
    }

    /// <p>Creates a robot.</p>
    #[allow(unused_mut)]
    async fn create_robot(
        &self,
        input: CreateRobotRequest,
    ) -> Result<CreateRobotResponse, RusotoError<CreateRobotError>> {
        #![allow(clippy::needless_update)]
        let request_uri = "/createRobot";

        let mut request = SignedRequest::new("POST", "robomaker", &self.region, &request_uri);
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
                .deserialize::<CreateRobotResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(CreateRobotError::from_response(response))
        }
    }

    /// <p>Creates a robot application. </p>
    #[allow(unused_mut)]
    async fn create_robot_application(
        &self,
        input: CreateRobotApplicationRequest,
    ) -> Result<CreateRobotApplicationResponse, RusotoError<CreateRobotApplicationError>> {
        #![allow(clippy::needless_update)]
        let request_uri = "/createRobotApplication";

        let mut request = SignedRequest::new("POST", "robomaker", &self.region, &request_uri);
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
                .deserialize::<CreateRobotApplicationResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(CreateRobotApplicationError::from_response(response))
        }
    }

    /// <p>Creates a version of a robot application.</p>
    #[allow(unused_mut)]
    async fn create_robot_application_version(
        &self,
        input: CreateRobotApplicationVersionRequest,
    ) -> Result<
        CreateRobotApplicationVersionResponse,
        RusotoError<CreateRobotApplicationVersionError>,
    > {
        #![allow(clippy::needless_update)]
        let request_uri = "/createRobotApplicationVersion";

        let mut request = SignedRequest::new("POST", "robomaker", &self.region, &request_uri);
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
                .deserialize::<CreateRobotApplicationVersionResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(CreateRobotApplicationVersionError::from_response(response))
        }
    }

    /// <p>Creates a simulation application.</p>
    #[allow(unused_mut)]
    async fn create_simulation_application(
        &self,
        input: CreateSimulationApplicationRequest,
    ) -> Result<CreateSimulationApplicationResponse, RusotoError<CreateSimulationApplicationError>>
    {
        #![allow(clippy::needless_update)]
        let request_uri = "/createSimulationApplication";

        let mut request = SignedRequest::new("POST", "robomaker", &self.region, &request_uri);
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
                .deserialize::<CreateSimulationApplicationResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(CreateSimulationApplicationError::from_response(response))
        }
    }

    /// <p>Creates a simulation application with a specific revision id.</p>
    #[allow(unused_mut)]
    async fn create_simulation_application_version(
        &self,
        input: CreateSimulationApplicationVersionRequest,
    ) -> Result<
        CreateSimulationApplicationVersionResponse,
        RusotoError<CreateSimulationApplicationVersionError>,
    > {
        #![allow(clippy::needless_update)]
        let request_uri = "/createSimulationApplicationVersion";

        let mut request = SignedRequest::new("POST", "robomaker", &self.region, &request_uri);
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
                .deserialize::<CreateSimulationApplicationVersionResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(CreateSimulationApplicationVersionError::from_response(
                response,
            ))
        }
    }

    /// <p><p>Creates a simulation job.</p> <note> <p>After 90 days, simulation jobs expire and will be deleted. They will no longer be accessible. </p> </note></p>
    #[allow(unused_mut)]
    async fn create_simulation_job(
        &self,
        input: CreateSimulationJobRequest,
    ) -> Result<CreateSimulationJobResponse, RusotoError<CreateSimulationJobError>> {
        #![allow(clippy::needless_update)]
        let request_uri = "/createSimulationJob";

        let mut request = SignedRequest::new("POST", "robomaker", &self.region, &request_uri);
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
                .deserialize::<CreateSimulationJobResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(CreateSimulationJobError::from_response(response))
        }
    }

    /// <p>Creates a world export job.</p>
    #[allow(unused_mut)]
    async fn create_world_export_job(
        &self,
        input: CreateWorldExportJobRequest,
    ) -> Result<CreateWorldExportJobResponse, RusotoError<CreateWorldExportJobError>> {
        #![allow(clippy::needless_update)]
        let request_uri = "/createWorldExportJob";

        let mut request = SignedRequest::new("POST", "robomaker", &self.region, &request_uri);
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
                .deserialize::<CreateWorldExportJobResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(CreateWorldExportJobError::from_response(response))
        }
    }

    /// <p>Creates worlds using the specified template.</p>
    #[allow(unused_mut)]
    async fn create_world_generation_job(
        &self,
        input: CreateWorldGenerationJobRequest,
    ) -> Result<CreateWorldGenerationJobResponse, RusotoError<CreateWorldGenerationJobError>> {
        #![allow(clippy::needless_update)]
        let request_uri = "/createWorldGenerationJob";

        let mut request = SignedRequest::new("POST", "robomaker", &self.region, &request_uri);
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
                .deserialize::<CreateWorldGenerationJobResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(CreateWorldGenerationJobError::from_response(response))
        }
    }

    /// <p>Creates a world template.</p>
    #[allow(unused_mut)]
    async fn create_world_template(
        &self,
        input: CreateWorldTemplateRequest,
    ) -> Result<CreateWorldTemplateResponse, RusotoError<CreateWorldTemplateError>> {
        #![allow(clippy::needless_update)]
        let request_uri = "/createWorldTemplate";

        let mut request = SignedRequest::new("POST", "robomaker", &self.region, &request_uri);
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
                .deserialize::<CreateWorldTemplateResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(CreateWorldTemplateError::from_response(response))
        }
    }

    /// <p>Deletes a fleet.</p>
    #[allow(unused_mut)]
    async fn delete_fleet(
        &self,
        input: DeleteFleetRequest,
    ) -> Result<DeleteFleetResponse, RusotoError<DeleteFleetError>> {
        #![allow(clippy::needless_update)]
        let request_uri = "/deleteFleet";

        let mut request = SignedRequest::new("POST", "robomaker", &self.region, &request_uri);
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
                .deserialize::<DeleteFleetResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(DeleteFleetError::from_response(response))
        }
    }

    /// <p>Deletes a robot.</p>
    #[allow(unused_mut)]
    async fn delete_robot(
        &self,
        input: DeleteRobotRequest,
    ) -> Result<DeleteRobotResponse, RusotoError<DeleteRobotError>> {
        #![allow(clippy::needless_update)]
        let request_uri = "/deleteRobot";

        let mut request = SignedRequest::new("POST", "robomaker", &self.region, &request_uri);
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
                .deserialize::<DeleteRobotResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(DeleteRobotError::from_response(response))
        }
    }

    /// <p>Deletes a robot application.</p>
    #[allow(unused_mut)]
    async fn delete_robot_application(
        &self,
        input: DeleteRobotApplicationRequest,
    ) -> Result<DeleteRobotApplicationResponse, RusotoError<DeleteRobotApplicationError>> {
        #![allow(clippy::needless_update)]
        let request_uri = "/deleteRobotApplication";

        let mut request = SignedRequest::new("POST", "robomaker", &self.region, &request_uri);
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
                .deserialize::<DeleteRobotApplicationResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(DeleteRobotApplicationError::from_response(response))
        }
    }

    /// <p>Deletes a simulation application.</p>
    #[allow(unused_mut)]
    async fn delete_simulation_application(
        &self,
        input: DeleteSimulationApplicationRequest,
    ) -> Result<DeleteSimulationApplicationResponse, RusotoError<DeleteSimulationApplicationError>>
    {
        #![allow(clippy::needless_update)]
        let request_uri = "/deleteSimulationApplication";

        let mut request = SignedRequest::new("POST", "robomaker", &self.region, &request_uri);
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
                .deserialize::<DeleteSimulationApplicationResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(DeleteSimulationApplicationError::from_response(response))
        }
    }

    /// <p>Deletes a world template.</p>
    #[allow(unused_mut)]
    async fn delete_world_template(
        &self,
        input: DeleteWorldTemplateRequest,
    ) -> Result<DeleteWorldTemplateResponse, RusotoError<DeleteWorldTemplateError>> {
        #![allow(clippy::needless_update)]
        let request_uri = "/deleteWorldTemplate";

        let mut request = SignedRequest::new("POST", "robomaker", &self.region, &request_uri);
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
                .deserialize::<DeleteWorldTemplateResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(DeleteWorldTemplateError::from_response(response))
        }
    }

    /// <p>Deregisters a robot.</p>
    #[allow(unused_mut)]
    async fn deregister_robot(
        &self,
        input: DeregisterRobotRequest,
    ) -> Result<DeregisterRobotResponse, RusotoError<DeregisterRobotError>> {
        #![allow(clippy::needless_update)]
        let request_uri = "/deregisterRobot";

        let mut request = SignedRequest::new("POST", "robomaker", &self.region, &request_uri);
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
                .deserialize::<DeregisterRobotResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(DeregisterRobotError::from_response(response))
        }
    }

    /// <p>Describes a deployment job.</p>
    #[allow(unused_mut)]
    async fn describe_deployment_job(
        &self,
        input: DescribeDeploymentJobRequest,
    ) -> Result<DescribeDeploymentJobResponse, RusotoError<DescribeDeploymentJobError>> {
        #![allow(clippy::needless_update)]
        let request_uri = "/describeDeploymentJob";

        let mut request = SignedRequest::new("POST", "robomaker", &self.region, &request_uri);
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
                .deserialize::<DescribeDeploymentJobResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(DescribeDeploymentJobError::from_response(response))
        }
    }

    /// <p>Describes a fleet.</p>
    #[allow(unused_mut)]
    async fn describe_fleet(
        &self,
        input: DescribeFleetRequest,
    ) -> Result<DescribeFleetResponse, RusotoError<DescribeFleetError>> {
        #![allow(clippy::needless_update)]
        let request_uri = "/describeFleet";

        let mut request = SignedRequest::new("POST", "robomaker", &self.region, &request_uri);
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
                .deserialize::<DescribeFleetResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(DescribeFleetError::from_response(response))
        }
    }

    /// <p>Describes a robot.</p>
    #[allow(unused_mut)]
    async fn describe_robot(
        &self,
        input: DescribeRobotRequest,
    ) -> Result<DescribeRobotResponse, RusotoError<DescribeRobotError>> {
        #![allow(clippy::needless_update)]
        let request_uri = "/describeRobot";

        let mut request = SignedRequest::new("POST", "robomaker", &self.region, &request_uri);
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
                .deserialize::<DescribeRobotResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(DescribeRobotError::from_response(response))
        }
    }

    /// <p>Describes a robot application.</p>
    #[allow(unused_mut)]
    async fn describe_robot_application(
        &self,
        input: DescribeRobotApplicationRequest,
    ) -> Result<DescribeRobotApplicationResponse, RusotoError<DescribeRobotApplicationError>> {
        #![allow(clippy::needless_update)]
        let request_uri = "/describeRobotApplication";

        let mut request = SignedRequest::new("POST", "robomaker", &self.region, &request_uri);
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
                .deserialize::<DescribeRobotApplicationResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(DescribeRobotApplicationError::from_response(response))
        }
    }

    /// <p>Describes a simulation application.</p>
    #[allow(unused_mut)]
    async fn describe_simulation_application(
        &self,
        input: DescribeSimulationApplicationRequest,
    ) -> Result<
        DescribeSimulationApplicationResponse,
        RusotoError<DescribeSimulationApplicationError>,
    > {
        #![allow(clippy::needless_update)]
        let request_uri = "/describeSimulationApplication";

        let mut request = SignedRequest::new("POST", "robomaker", &self.region, &request_uri);
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
                .deserialize::<DescribeSimulationApplicationResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(DescribeSimulationApplicationError::from_response(response))
        }
    }

    /// <p>Describes a simulation job.</p>
    #[allow(unused_mut)]
    async fn describe_simulation_job(
        &self,
        input: DescribeSimulationJobRequest,
    ) -> Result<DescribeSimulationJobResponse, RusotoError<DescribeSimulationJobError>> {
        #![allow(clippy::needless_update)]
        let request_uri = "/describeSimulationJob";

        let mut request = SignedRequest::new("POST", "robomaker", &self.region, &request_uri);
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
                .deserialize::<DescribeSimulationJobResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(DescribeSimulationJobError::from_response(response))
        }
    }

    /// <p>Describes a simulation job batch.</p>
    #[allow(unused_mut)]
    async fn describe_simulation_job_batch(
        &self,
        input: DescribeSimulationJobBatchRequest,
    ) -> Result<DescribeSimulationJobBatchResponse, RusotoError<DescribeSimulationJobBatchError>>
    {
        #![allow(clippy::needless_update)]
        let request_uri = "/describeSimulationJobBatch";

        let mut request = SignedRequest::new("POST", "robomaker", &self.region, &request_uri);
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
                .deserialize::<DescribeSimulationJobBatchResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(DescribeSimulationJobBatchError::from_response(response))
        }
    }

    /// <p>Describes a world.</p>
    #[allow(unused_mut)]
    async fn describe_world(
        &self,
        input: DescribeWorldRequest,
    ) -> Result<DescribeWorldResponse, RusotoError<DescribeWorldError>> {
        #![allow(clippy::needless_update)]
        let request_uri = "/describeWorld";

        let mut request = SignedRequest::new("POST", "robomaker", &self.region, &request_uri);
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
                .deserialize::<DescribeWorldResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(DescribeWorldError::from_response(response))
        }
    }

    /// <p>Describes a world export job.</p>
    #[allow(unused_mut)]
    async fn describe_world_export_job(
        &self,
        input: DescribeWorldExportJobRequest,
    ) -> Result<DescribeWorldExportJobResponse, RusotoError<DescribeWorldExportJobError>> {
        #![allow(clippy::needless_update)]
        let request_uri = "/describeWorldExportJob";

        let mut request = SignedRequest::new("POST", "robomaker", &self.region, &request_uri);
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
                .deserialize::<DescribeWorldExportJobResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(DescribeWorldExportJobError::from_response(response))
        }
    }

    /// <p>Describes a world generation job.</p>
    #[allow(unused_mut)]
    async fn describe_world_generation_job(
        &self,
        input: DescribeWorldGenerationJobRequest,
    ) -> Result<DescribeWorldGenerationJobResponse, RusotoError<DescribeWorldGenerationJobError>>
    {
        #![allow(clippy::needless_update)]
        let request_uri = "/describeWorldGenerationJob";

        let mut request = SignedRequest::new("POST", "robomaker", &self.region, &request_uri);
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
                .deserialize::<DescribeWorldGenerationJobResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(DescribeWorldGenerationJobError::from_response(response))
        }
    }

    /// <p>Describes a world template.</p>
    #[allow(unused_mut)]
    async fn describe_world_template(
        &self,
        input: DescribeWorldTemplateRequest,
    ) -> Result<DescribeWorldTemplateResponse, RusotoError<DescribeWorldTemplateError>> {
        #![allow(clippy::needless_update)]
        let request_uri = "/describeWorldTemplate";

        let mut request = SignedRequest::new("POST", "robomaker", &self.region, &request_uri);
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
                .deserialize::<DescribeWorldTemplateResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(DescribeWorldTemplateError::from_response(response))
        }
    }

    /// <p>Gets the world template body.</p>
    #[allow(unused_mut)]
    async fn get_world_template_body(
        &self,
        input: GetWorldTemplateBodyRequest,
    ) -> Result<GetWorldTemplateBodyResponse, RusotoError<GetWorldTemplateBodyError>> {
        #![allow(clippy::needless_update)]
        let request_uri = "/getWorldTemplateBody";

        let mut request = SignedRequest::new("POST", "robomaker", &self.region, &request_uri);
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
                .deserialize::<GetWorldTemplateBodyResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(GetWorldTemplateBodyError::from_response(response))
        }
    }

    /// <p>Returns a list of deployment jobs for a fleet. You can optionally provide filters to retrieve specific deployment jobs. </p>
    #[allow(unused_mut)]
    async fn list_deployment_jobs(
        &self,
        input: ListDeploymentJobsRequest,
    ) -> Result<ListDeploymentJobsResponse, RusotoError<ListDeploymentJobsError>> {
        #![allow(clippy::needless_update)]
        let request_uri = "/listDeploymentJobs";

        let mut request = SignedRequest::new("POST", "robomaker", &self.region, &request_uri);
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
                .deserialize::<ListDeploymentJobsResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(ListDeploymentJobsError::from_response(response))
        }
    }

    /// <p>Returns a list of fleets. You can optionally provide filters to retrieve specific fleets. </p>
    #[allow(unused_mut)]
    async fn list_fleets(
        &self,
        input: ListFleetsRequest,
    ) -> Result<ListFleetsResponse, RusotoError<ListFleetsError>> {
        #![allow(clippy::needless_update)]
        let request_uri = "/listFleets";

        let mut request = SignedRequest::new("POST", "robomaker", &self.region, &request_uri);
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
                .deserialize::<ListFleetsResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(ListFleetsError::from_response(response))
        }
    }

    /// <p>Returns a list of robot application. You can optionally provide filters to retrieve specific robot applications.</p>
    #[allow(unused_mut)]
    async fn list_robot_applications(
        &self,
        input: ListRobotApplicationsRequest,
    ) -> Result<ListRobotApplicationsResponse, RusotoError<ListRobotApplicationsError>> {
        #![allow(clippy::needless_update)]
        let request_uri = "/listRobotApplications";

        let mut request = SignedRequest::new("POST", "robomaker", &self.region, &request_uri);
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
                .deserialize::<ListRobotApplicationsResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(ListRobotApplicationsError::from_response(response))
        }
    }

    /// <p>Returns a list of robots. You can optionally provide filters to retrieve specific robots.</p>
    #[allow(unused_mut)]
    async fn list_robots(
        &self,
        input: ListRobotsRequest,
    ) -> Result<ListRobotsResponse, RusotoError<ListRobotsError>> {
        #![allow(clippy::needless_update)]
        let request_uri = "/listRobots";

        let mut request = SignedRequest::new("POST", "robomaker", &self.region, &request_uri);
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
                .deserialize::<ListRobotsResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(ListRobotsError::from_response(response))
        }
    }

    /// <p>Returns a list of simulation applications. You can optionally provide filters to retrieve specific simulation applications. </p>
    #[allow(unused_mut)]
    async fn list_simulation_applications(
        &self,
        input: ListSimulationApplicationsRequest,
    ) -> Result<ListSimulationApplicationsResponse, RusotoError<ListSimulationApplicationsError>>
    {
        #![allow(clippy::needless_update)]
        let request_uri = "/listSimulationApplications";

        let mut request = SignedRequest::new("POST", "robomaker", &self.region, &request_uri);
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
                .deserialize::<ListSimulationApplicationsResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(ListSimulationApplicationsError::from_response(response))
        }
    }

    /// <p>Returns a list simulation job batches. You can optionally provide filters to retrieve specific simulation batch jobs. </p>
    #[allow(unused_mut)]
    async fn list_simulation_job_batches(
        &self,
        input: ListSimulationJobBatchesRequest,
    ) -> Result<ListSimulationJobBatchesResponse, RusotoError<ListSimulationJobBatchesError>> {
        #![allow(clippy::needless_update)]
        let request_uri = "/listSimulationJobBatches";

        let mut request = SignedRequest::new("POST", "robomaker", &self.region, &request_uri);
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
                .deserialize::<ListSimulationJobBatchesResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(ListSimulationJobBatchesError::from_response(response))
        }
    }

    /// <p>Returns a list of simulation jobs. You can optionally provide filters to retrieve specific simulation jobs. </p>
    #[allow(unused_mut)]
    async fn list_simulation_jobs(
        &self,
        input: ListSimulationJobsRequest,
    ) -> Result<ListSimulationJobsResponse, RusotoError<ListSimulationJobsError>> {
        #![allow(clippy::needless_update)]
        let request_uri = "/listSimulationJobs";

        let mut request = SignedRequest::new("POST", "robomaker", &self.region, &request_uri);
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
                .deserialize::<ListSimulationJobsResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(ListSimulationJobsError::from_response(response))
        }
    }

    /// <p>Lists all tags on a AWS RoboMaker resource.</p>
    #[allow(unused_mut)]
    async fn list_tags_for_resource(
        &self,
        input: ListTagsForResourceRequest,
    ) -> Result<ListTagsForResourceResponse, RusotoError<ListTagsForResourceError>> {
        #![allow(clippy::needless_update)]
        let request_uri = format!("/tags/{resource_arn}", resource_arn = input.resource_arn);

        let mut request = SignedRequest::new("GET", "robomaker", &self.region, &request_uri);
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

    /// <p>Lists world export jobs.</p>
    #[allow(unused_mut)]
    async fn list_world_export_jobs(
        &self,
        input: ListWorldExportJobsRequest,
    ) -> Result<ListWorldExportJobsResponse, RusotoError<ListWorldExportJobsError>> {
        #![allow(clippy::needless_update)]
        let request_uri = "/listWorldExportJobs";

        let mut request = SignedRequest::new("POST", "robomaker", &self.region, &request_uri);
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
                .deserialize::<ListWorldExportJobsResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(ListWorldExportJobsError::from_response(response))
        }
    }

    /// <p>Lists world generator jobs.</p>
    #[allow(unused_mut)]
    async fn list_world_generation_jobs(
        &self,
        input: ListWorldGenerationJobsRequest,
    ) -> Result<ListWorldGenerationJobsResponse, RusotoError<ListWorldGenerationJobsError>> {
        #![allow(clippy::needless_update)]
        let request_uri = "/listWorldGenerationJobs";

        let mut request = SignedRequest::new("POST", "robomaker", &self.region, &request_uri);
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
                .deserialize::<ListWorldGenerationJobsResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(ListWorldGenerationJobsError::from_response(response))
        }
    }

    /// <p>Lists world templates.</p>
    #[allow(unused_mut)]
    async fn list_world_templates(
        &self,
        input: ListWorldTemplatesRequest,
    ) -> Result<ListWorldTemplatesResponse, RusotoError<ListWorldTemplatesError>> {
        #![allow(clippy::needless_update)]
        let request_uri = "/listWorldTemplates";

        let mut request = SignedRequest::new("POST", "robomaker", &self.region, &request_uri);
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
                .deserialize::<ListWorldTemplatesResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(ListWorldTemplatesError::from_response(response))
        }
    }

    /// <p>Lists worlds.</p>
    #[allow(unused_mut)]
    async fn list_worlds(
        &self,
        input: ListWorldsRequest,
    ) -> Result<ListWorldsResponse, RusotoError<ListWorldsError>> {
        #![allow(clippy::needless_update)]
        let request_uri = "/listWorlds";

        let mut request = SignedRequest::new("POST", "robomaker", &self.region, &request_uri);
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
                .deserialize::<ListWorldsResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(ListWorldsError::from_response(response))
        }
    }

    /// <p>Registers a robot with a fleet.</p>
    #[allow(unused_mut)]
    async fn register_robot(
        &self,
        input: RegisterRobotRequest,
    ) -> Result<RegisterRobotResponse, RusotoError<RegisterRobotError>> {
        #![allow(clippy::needless_update)]
        let request_uri = "/registerRobot";

        let mut request = SignedRequest::new("POST", "robomaker", &self.region, &request_uri);
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
                .deserialize::<RegisterRobotResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(RegisterRobotError::from_response(response))
        }
    }

    /// <p>Restarts a running simulation job.</p>
    #[allow(unused_mut)]
    async fn restart_simulation_job(
        &self,
        input: RestartSimulationJobRequest,
    ) -> Result<RestartSimulationJobResponse, RusotoError<RestartSimulationJobError>> {
        #![allow(clippy::needless_update)]
        let request_uri = "/restartSimulationJob";

        let mut request = SignedRequest::new("POST", "robomaker", &self.region, &request_uri);
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
                .deserialize::<RestartSimulationJobResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(RestartSimulationJobError::from_response(response))
        }
    }

    /// <p>Starts a new simulation job batch. The batch is defined using one or more <code>SimulationJobRequest</code> objects. </p>
    #[allow(unused_mut)]
    async fn start_simulation_job_batch(
        &self,
        input: StartSimulationJobBatchRequest,
    ) -> Result<StartSimulationJobBatchResponse, RusotoError<StartSimulationJobBatchError>> {
        #![allow(clippy::needless_update)]
        let request_uri = "/startSimulationJobBatch";

        let mut request = SignedRequest::new("POST", "robomaker", &self.region, &request_uri);
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
                .deserialize::<StartSimulationJobBatchResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(StartSimulationJobBatchError::from_response(response))
        }
    }

    /// <p>Syncrhonizes robots in a fleet to the latest deployment. This is helpful if robots were added after a deployment.</p>
    #[allow(unused_mut)]
    async fn sync_deployment_job(
        &self,
        input: SyncDeploymentJobRequest,
    ) -> Result<SyncDeploymentJobResponse, RusotoError<SyncDeploymentJobError>> {
        #![allow(clippy::needless_update)]
        let request_uri = "/syncDeploymentJob";

        let mut request = SignedRequest::new("POST", "robomaker", &self.region, &request_uri);
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
                .deserialize::<SyncDeploymentJobResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(SyncDeploymentJobError::from_response(response))
        }
    }

    /// <p>Adds or edits tags for a AWS RoboMaker resource.</p> <p>Each tag consists of a tag key and a tag value. Tag keys and tag values are both required, but tag values can be empty strings. </p> <p>For information about the rules that apply to tag keys and tag values, see <a href="https://docs.aws.amazon.com/awsaccountbilling/latest/aboutv2/allocation-tag-restrictions.html">User-Defined Tag Restrictions</a> in the <i>AWS Billing and Cost Management User Guide</i>. </p>
    #[allow(unused_mut)]
    async fn tag_resource(
        &self,
        input: TagResourceRequest,
    ) -> Result<TagResourceResponse, RusotoError<TagResourceError>> {
        #![allow(clippy::needless_update)]
        let request_uri = format!("/tags/{resource_arn}", resource_arn = input.resource_arn);

        let mut request = SignedRequest::new("POST", "robomaker", &self.region, &request_uri);
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
                .deserialize::<TagResourceResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(TagResourceError::from_response(response))
        }
    }

    /// <p>Removes the specified tags from the specified AWS RoboMaker resource.</p> <p>To remove a tag, specify the tag key. To change the tag value of an existing tag key, use <a href="https://docs.aws.amazon.com/robomaker/latest/dg/API_TagResource.html"> <code>TagResource</code> </a>. </p>
    #[allow(unused_mut)]
    async fn untag_resource(
        &self,
        input: UntagResourceRequest,
    ) -> Result<UntagResourceResponse, RusotoError<UntagResourceError>> {
        #![allow(clippy::needless_update)]
        let request_uri = format!("/tags/{resource_arn}", resource_arn = input.resource_arn);

        let mut request = SignedRequest::new("DELETE", "robomaker", &self.region, &request_uri);
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
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<UntagResourceResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(UntagResourceError::from_response(response))
        }
    }

    /// <p>Updates a robot application.</p>
    #[allow(unused_mut)]
    async fn update_robot_application(
        &self,
        input: UpdateRobotApplicationRequest,
    ) -> Result<UpdateRobotApplicationResponse, RusotoError<UpdateRobotApplicationError>> {
        #![allow(clippy::needless_update)]
        let request_uri = "/updateRobotApplication";

        let mut request = SignedRequest::new("POST", "robomaker", &self.region, &request_uri);
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
                .deserialize::<UpdateRobotApplicationResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(UpdateRobotApplicationError::from_response(response))
        }
    }

    /// <p>Updates a simulation application.</p>
    #[allow(unused_mut)]
    async fn update_simulation_application(
        &self,
        input: UpdateSimulationApplicationRequest,
    ) -> Result<UpdateSimulationApplicationResponse, RusotoError<UpdateSimulationApplicationError>>
    {
        #![allow(clippy::needless_update)]
        let request_uri = "/updateSimulationApplication";

        let mut request = SignedRequest::new("POST", "robomaker", &self.region, &request_uri);
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
                .deserialize::<UpdateSimulationApplicationResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(UpdateSimulationApplicationError::from_response(response))
        }
    }

    /// <p>Updates a world template.</p>
    #[allow(unused_mut)]
    async fn update_world_template(
        &self,
        input: UpdateWorldTemplateRequest,
    ) -> Result<UpdateWorldTemplateResponse, RusotoError<UpdateWorldTemplateError>> {
        #![allow(clippy::needless_update)]
        let request_uri = "/updateWorldTemplate";

        let mut request = SignedRequest::new("POST", "robomaker", &self.region, &request_uri);
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
                .deserialize::<UpdateWorldTemplateResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(UpdateWorldTemplateError::from_response(response))
        }
    }
}
