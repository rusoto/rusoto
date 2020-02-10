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
pub struct BatchDescribeSimulationJobRequest {
    /// <p>A list of Amazon Resource Names (ARNs) of simulation jobs to describe.</p>
    #[serde(rename = "jobs")]
    pub jobs: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
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

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CancelDeploymentJobRequest {
    /// <p>The deployment job ARN to cancel.</p>
    #[serde(rename = "job")]
    pub job: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CancelDeploymentJobResponse {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CancelSimulationJobRequest {
    /// <p>The simulation job ARN to cancel.</p>
    #[serde(rename = "job")]
    pub job: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CancelSimulationJobResponse {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
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

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
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

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
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

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
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

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateRobotApplicationRequest {
    /// <p>The name of the robot application.</p>
    #[serde(rename = "name")]
    pub name: String,
    /// <p>The robot software suite used by the robot application.</p>
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

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
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
    /// <p>The robot software suite used by the robot application.</p>
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

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
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

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
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
    /// <p>The robot software suite used by the robot application.</p>
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

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
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

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
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

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateSimulationApplicationRequest {
    /// <p>The name of the simulation application.</p>
    #[serde(rename = "name")]
    pub name: String,
    /// <p>The rendering engine for the simulation application.</p>
    #[serde(rename = "renderingEngine")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rendering_engine: Option<RenderingEngine>,
    /// <p>The robot software suite of the simulation application.</p>
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

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
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
    /// <p>Information about the robot software suite.</p>
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

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
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

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
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
    /// <p>Information about the robot software suite.</p>
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

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateSimulationJobRequest {
    /// <p>Unique, case-sensitive identifier that you provide to ensure the idempotency of the request.</p>
    #[serde(rename = "clientRequestToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_request_token: Option<String>,
    /// <p><p>The data sources for the simulation job.</p> <note> <p>There is a limit of 100 files and a combined size of 25GB for all <code>DataSourceConfig</code> objects. </p> </note></p>
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

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
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

/// <p>Information about a data source.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
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
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteFleetRequest {
    /// <p>The Amazon Resource Name (ARN) of the fleet.</p>
    #[serde(rename = "fleet")]
    pub fleet: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DeleteFleetResponse {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
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

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DeleteRobotApplicationResponse {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteRobotRequest {
    /// <p>The Amazon Resource Name (ARN) of the robot.</p>
    #[serde(rename = "robot")]
    pub robot: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DeleteRobotResponse {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
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

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DeleteSimulationApplicationResponse {}

/// <p>Information about a deployment application configuration.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
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
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
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
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
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
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
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

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeregisterRobotRequest {
    /// <p>The Amazon Resource Name (ARN) of the fleet.</p>
    #[serde(rename = "fleet")]
    pub fleet: String,
    /// <p>The Amazon Resource Name (ARN) of the robot.</p>
    #[serde(rename = "robot")]
    pub robot: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
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

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeDeploymentJobRequest {
    /// <p>The Amazon Resource Name (ARN) of the deployment job.</p>
    #[serde(rename = "job")]
    pub job: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
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

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeFleetRequest {
    /// <p>The Amazon Resource Name (ARN) of the fleet.</p>
    #[serde(rename = "fleet")]
    pub fleet: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
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

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
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

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
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
    /// <p>The robot software suite used by the robot application.</p>
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

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeRobotRequest {
    /// <p>The Amazon Resource Name (ARN) of the robot to be described.</p>
    #[serde(rename = "robot")]
    pub robot: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
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

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
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

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
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
    /// <p>Information about the robot software suite.</p>
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

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeSimulationJobRequest {
    /// <p>The Amazon Resource Name (ARN) of the simulation job to be described.</p>
    #[serde(rename = "job")]
    pub job: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
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

/// <p>Information about a filter.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
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

/// <p>Information about a fleet.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
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

/// <p>Information about a launch configuration.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
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
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListDeploymentJobsRequest {
    /// <p>Optional filters to limit results.</p> <p>The filter names <code>status</code> and <code>fleetName</code> are supported. When filtering, you must use the complete value of the filtered item. You can use up to three filters, but they must be for the same named item. For example, if you are looking for items with the status <code>InProgress</code> or the status <code>Pending</code>.</p>
    #[serde(rename = "filters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<Vec<Filter>>,
    /// <p>The maximum number of deployment job results returned by <code>ListDeploymentJobs</code> in paginated output. When this parameter is used, <code>ListDeploymentJobs</code> only returns <code>maxResults</code> results in a single page along with a <code>nextToken</code> response element. The remaining results of the initial request can be seen by sending another <code>ListDeploymentJobs</code> request with the returned <code>nextToken</code> value. This value can be between 1 and 100. If this parameter is not used, then <code>ListDeploymentJobs</code> returns up to 100 results and a <code>nextToken</code> value if applicable. </p>
    #[serde(rename = "maxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p><p>The <code>nextToken</code> value returned from a previous paginated <code>ListDeploymentJobs</code> request where <code>maxResults</code> was used and the results exceeded the value of that parameter. Pagination continues from the end of the previous results that returned the <code>nextToken</code> value. </p> <note> <p>This token should be treated as an opaque identifier that is only used to retrieve the next items in a list and not for other programmatic purposes.</p> </note></p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListDeploymentJobsResponse {
    /// <p>A list of deployment jobs that meet the criteria of the request.</p>
    #[serde(rename = "deploymentJobs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deployment_jobs: Option<Vec<DeploymentJob>>,
    /// <p>The <code>nextToken</code> value to include in a future <code>ListDeploymentJobs</code> request. When the results of a <code>ListDeploymentJobs</code> request exceed <code>maxResults</code>, this value can be used to retrieve the next page of results. This value is <code>null</code> when there are no more results to return. </p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListFleetsRequest {
    /// <p>Optional filters to limit results.</p> <p>The filter name <code>name</code> is supported. When filtering, you must use the complete value of the filtered item. You can use up to three filters.</p>
    #[serde(rename = "filters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<Vec<Filter>>,
    /// <p>The maximum number of deployment job results returned by <code>ListFleets</code> in paginated output. When this parameter is used, <code>ListFleets</code> only returns <code>maxResults</code> results in a single page along with a <code>nextToken</code> response element. The remaining results of the initial request can be seen by sending another <code>ListFleets</code> request with the returned <code>nextToken</code> value. This value can be between 1 and 100. If this parameter is not used, then <code>ListFleets</code> returns up to 100 results and a <code>nextToken</code> value if applicable. </p>
    #[serde(rename = "maxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p><p>The <code>nextToken</code> value returned from a previous paginated <code>ListFleets</code> request where <code>maxResults</code> was used and the results exceeded the value of that parameter. Pagination continues from the end of the previous results that returned the <code>nextToken</code> value. </p> <note> <p>This token should be treated as an opaque identifier that is only used to retrieve the next items in a list and not for other programmatic purposes.</p> </note></p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListFleetsResponse {
    /// <p>A list of fleet details meeting the request criteria.</p>
    #[serde(rename = "fleetDetails")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fleet_details: Option<Vec<Fleet>>,
    /// <p>The <code>nextToken</code> value to include in a future <code>ListDeploymentJobs</code> request. When the results of a <code>ListFleets</code> request exceed <code>maxResults</code>, this value can be used to retrieve the next page of results. This value is <code>null</code> when there are no more results to return. </p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListRobotApplicationsRequest {
    /// <p>Optional filters to limit results.</p> <p>The filter name <code>name</code> is supported. When filtering, you must use the complete value of the filtered item. You can use up to three filters.</p>
    #[serde(rename = "filters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<Vec<Filter>>,
    /// <p>The maximum number of deployment job results returned by <code>ListRobotApplications</code> in paginated output. When this parameter is used, <code>ListRobotApplications</code> only returns <code>maxResults</code> results in a single page along with a <code>nextToken</code> response element. The remaining results of the initial request can be seen by sending another <code>ListRobotApplications</code> request with the returned <code>nextToken</code> value. This value can be between 1 and 100. If this parameter is not used, then <code>ListRobotApplications</code> returns up to 100 results and a <code>nextToken</code> value if applicable. </p>
    #[serde(rename = "maxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p><p>The <code>nextToken</code> value returned from a previous paginated <code>ListRobotApplications</code> request where <code>maxResults</code> was used and the results exceeded the value of that parameter. Pagination continues from the end of the previous results that returned the <code>nextToken</code> value. </p> <note> <p>This token should be treated as an opaque identifier that is only used to retrieve the next items in a list and not for other programmatic purposes.</p> </note></p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The version qualifier of the robot application.</p>
    #[serde(rename = "versionQualifier")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version_qualifier: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListRobotApplicationsResponse {
    /// <p>The <code>nextToken</code> value to include in a future <code>ListRobotApplications</code> request. When the results of a <code>ListRobotApplications</code> request exceed <code>maxResults</code>, this value can be used to retrieve the next page of results. This value is <code>null</code> when there are no more results to return. </p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>A list of robot application summaries that meet the criteria of the request.</p>
    #[serde(rename = "robotApplicationSummaries")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub robot_application_summaries: Option<Vec<RobotApplicationSummary>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListRobotsRequest {
    /// <p>Optional filters to limit results.</p> <p>The filter names <code>status</code> and <code>fleetName</code> are supported. When filtering, you must use the complete value of the filtered item. You can use up to three filters, but they must be for the same named item. For example, if you are looking for items with the status <code>Registered</code> or the status <code>Available</code>.</p>
    #[serde(rename = "filters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<Vec<Filter>>,
    /// <p>The maximum number of deployment job results returned by <code>ListRobots</code> in paginated output. When this parameter is used, <code>ListRobots</code> only returns <code>maxResults</code> results in a single page along with a <code>nextToken</code> response element. The remaining results of the initial request can be seen by sending another <code>ListRobots</code> request with the returned <code>nextToken</code> value. This value can be between 1 and 100. If this parameter is not used, then <code>ListRobots</code> returns up to 100 results and a <code>nextToken</code> value if applicable. </p>
    #[serde(rename = "maxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p><p>The <code>nextToken</code> value returned from a previous paginated <code>ListRobots</code> request where <code>maxResults</code> was used and the results exceeded the value of that parameter. Pagination continues from the end of the previous results that returned the <code>nextToken</code> value. </p> <note> <p>This token should be treated as an opaque identifier that is only used to retrieve the next items in a list and not for other programmatic purposes.</p> </note></p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListRobotsResponse {
    /// <p>The <code>nextToken</code> value to include in a future <code>ListRobots</code> request. When the results of a <code>ListRobot</code> request exceed <code>maxResults</code>, this value can be used to retrieve the next page of results. This value is <code>null</code> when there are no more results to return. </p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>A list of robots that meet the criteria of the request.</p>
    #[serde(rename = "robots")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub robots: Option<Vec<Robot>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListSimulationApplicationsRequest {
    /// <p>Optional list of filters to limit results.</p> <p>The filter name <code>name</code> is supported. When filtering, you must use the complete value of the filtered item. You can use up to three filters.</p>
    #[serde(rename = "filters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<Vec<Filter>>,
    /// <p>The maximum number of deployment job results returned by <code>ListSimulationApplications</code> in paginated output. When this parameter is used, <code>ListSimulationApplications</code> only returns <code>maxResults</code> results in a single page along with a <code>nextToken</code> response element. The remaining results of the initial request can be seen by sending another <code>ListSimulationApplications</code> request with the returned <code>nextToken</code> value. This value can be between 1 and 100. If this parameter is not used, then <code>ListSimulationApplications</code> returns up to 100 results and a <code>nextToken</code> value if applicable. </p>
    #[serde(rename = "maxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p><p>The <code>nextToken</code> value returned from a previous paginated <code>ListSimulationApplications</code> request where <code>maxResults</code> was used and the results exceeded the value of that parameter. Pagination continues from the end of the previous results that returned the <code>nextToken</code> value. </p> <note> <p>This token should be treated as an opaque identifier that is only used to retrieve the next items in a list and not for other programmatic purposes.</p> </note></p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The version qualifier of the simulation application.</p>
    #[serde(rename = "versionQualifier")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version_qualifier: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListSimulationApplicationsResponse {
    /// <p>The <code>nextToken</code> value to include in a future <code>ListSimulationApplications</code> request. When the results of a <code>ListRobot</code> request exceed <code>maxResults</code>, this value can be used to retrieve the next page of results. This value is <code>null</code> when there are no more results to return. </p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>A list of simulation application summaries that meet the criteria of the request.</p>
    #[serde(rename = "simulationApplicationSummaries")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub simulation_application_summaries: Option<Vec<SimulationApplicationSummary>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListSimulationJobsRequest {
    /// <p>Optional filters to limit results.</p> <p>The filter names <code>status</code> and <code>simulationApplicationName</code> and <code>robotApplicationName</code> are supported. When filtering, you must use the complete value of the filtered item. You can use up to three filters, but they must be for the same named item. For example, if you are looking for items with the status <code>Preparing</code> or the status <code>Running</code>.</p>
    #[serde(rename = "filters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<Vec<Filter>>,
    /// <p>The maximum number of deployment job results returned by <code>ListSimulationJobs</code> in paginated output. When this parameter is used, <code>ListSimulationJobs</code> only returns <code>maxResults</code> results in a single page along with a <code>nextToken</code> response element. The remaining results of the initial request can be seen by sending another <code>ListSimulationJobs</code> request with the returned <code>nextToken</code> value. This value can be between 1 and 100. If this parameter is not used, then <code>ListSimulationJobs</code> returns up to 100 results and a <code>nextToken</code> value if applicable. </p>
    #[serde(rename = "maxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p><p>The <code>nextToken</code> value returned from a previous paginated <code>ListSimulationJobs</code> request where <code>maxResults</code> was used and the results exceeded the value of that parameter. Pagination continues from the end of the previous results that returned the <code>nextToken</code> value. </p> <note> <p>This token should be treated as an opaque identifier that is only used to retrieve the next items in a list and not for other programmatic purposes.</p> </note></p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListSimulationJobsResponse {
    /// <p>The <code>nextToken</code> value to include in a future <code>ListSimulationJobs</code> request. When the results of a <code>ListRobot</code> request exceed <code>maxResults</code>, this value can be used to retrieve the next page of results. This value is <code>null</code> when there are no more results to return. </p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>A list of simulation job summaries that meet the criteria of the request.</p>
    #[serde(rename = "simulationJobSummaries")]
    pub simulation_job_summaries: Vec<SimulationJobSummary>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListTagsForResourceRequest {
    /// <p>The AWS RoboMaker Amazon Resource Name (ARN) with tags to be listed.</p>
    #[serde(rename = "resourceArn")]
    pub resource_arn: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListTagsForResourceResponse {
    /// <p>The list of all tags added to the specified resource.</p>
    #[serde(rename = "tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<::std::collections::HashMap<String, String>>,
}

/// <p>The logging configuration.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LoggingConfig {
    /// <p>A boolean indicating whether to record all ROS topics.</p>
    #[serde(rename = "recordAllRosTopics")]
    pub record_all_ros_topics: bool,
}

/// <p>Describes a network interface.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
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
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
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
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PortForwardingConfig {
    /// <p>The port mappings for the configuration.</p>
    #[serde(rename = "portMappings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub port_mappings: Option<Vec<PortMapping>>,
}

/// <p>An object representing a port mapping.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
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
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
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

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct RegisterRobotRequest {
    /// <p>The Amazon Resource Name (ARN) of the fleet.</p>
    #[serde(rename = "fleet")]
    pub fleet: String,
    /// <p>The Amazon Resource Name (ARN) of the robot.</p>
    #[serde(rename = "robot")]
    pub robot: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
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
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
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

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct RestartSimulationJobRequest {
    /// <p>The Amazon Resource Name (ARN) of the simulation job.</p>
    #[serde(rename = "job")]
    pub job: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct RestartSimulationJobResponse {}

/// <p>Information about a robot.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
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
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
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
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
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
    /// <p>Information about a robot software suite.</p>
    #[serde(rename = "robotSoftwareSuite")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub robot_software_suite: Option<RobotSoftwareSuite>,
    /// <p>The version of the robot application.</p>
    #[serde(rename = "version")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

/// <p>Information about a robot deployment.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
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

/// <p>Information about a robot software suite.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RobotSoftwareSuite {
    /// <p>The name of the robot software suite.</p>
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The version of the robot software suite.</p>
    #[serde(rename = "version")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

/// <p>Information about S3 keys.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
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
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
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
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
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
}

/// <p>Summary information for a simulation application.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
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
    /// <p>Information about a robot software suite.</p>
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
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
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

/// <p>Summary information for a simulation job.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
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
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
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
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
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
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
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

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct SyncDeploymentJobRequest {
    /// <p>Unique, case-sensitive identifier that you provide to ensure the idempotency of the request.</p>
    #[serde(rename = "clientRequestToken")]
    pub client_request_token: String,
    /// <p>The target fleet for the synchronization.</p>
    #[serde(rename = "fleet")]
    pub fleet: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
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

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct TagResourceRequest {
    /// <p>The Amazon Resource Name (ARN) of the AWS RoboMaker resource you are tagging.</p>
    #[serde(rename = "resourceArn")]
    pub resource_arn: String,
    /// <p>A map that contains tag keys and tag values that are attached to the resource.</p>
    #[serde(rename = "tags")]
    pub tags: ::std::collections::HashMap<String, String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct TagResourceResponse {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UntagResourceRequest {
    /// <p>The Amazon Resource Name (ARN) of the AWS RoboMaker resource you are removing tags.</p>
    #[serde(rename = "resourceArn")]
    pub resource_arn: String,
    /// <p>A map that contains tag keys and tag values that will be unattached from the resource.</p>
    #[serde(rename = "tagKeys")]
    pub tag_keys: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UntagResourceResponse {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UpdateRobotApplicationRequest {
    /// <p>The application information for the robot application.</p>
    #[serde(rename = "application")]
    pub application: String,
    /// <p>The revision id for the robot application.</p>
    #[serde(rename = "currentRevisionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_revision_id: Option<String>,
    /// <p>The robot software suite used by the robot application.</p>
    #[serde(rename = "robotSoftwareSuite")]
    pub robot_software_suite: RobotSoftwareSuite,
    /// <p>The sources of the robot application.</p>
    #[serde(rename = "sources")]
    pub sources: Vec<SourceConfig>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
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
    /// <p>The robot software suite used by the robot application.</p>
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

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
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
    /// <p>Information about the robot software suite.</p>
    #[serde(rename = "robotSoftwareSuite")]
    pub robot_software_suite: RobotSoftwareSuite,
    /// <p>The simulation software suite used by the simulation application.</p>
    #[serde(rename = "simulationSoftwareSuite")]
    pub simulation_software_suite: SimulationSoftwareSuite,
    /// <p>The sources of the simulation application.</p>
    #[serde(rename = "sources")]
    pub sources: Vec<SourceConfig>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
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
    /// <p>Information about the robot software suite.</p>
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

/// <p>If your simulation job accesses resources in a VPC, you provide this parameter identifying the list of security group IDs and subnet IDs. These must belong to the same VPC. You must provide at least one security group and two subnet IDs.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
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
/// Trait representing the capabilities of the RoboMaker API. RoboMaker clients implement this trait.
pub trait Robomaker {
    /// <p>Describes one or more simulation jobs.</p>
    fn batch_describe_simulation_job(
        &self,
        input: BatchDescribeSimulationJobRequest,
    ) -> Pin<
        Box<
            dyn Future<
                    Output = Result<
                        BatchDescribeSimulationJobResponse,
                        RusotoError<BatchDescribeSimulationJobError>,
                    >,
                > + Send
                + 'static,
        >,
    >;

    /// <p>Cancels the specified deployment job.</p>
    fn cancel_deployment_job(
        &self,
        input: CancelDeploymentJobRequest,
    ) -> Pin<
        Box<
            dyn Future<
                    Output = Result<
                        CancelDeploymentJobResponse,
                        RusotoError<CancelDeploymentJobError>,
                    >,
                > + Send
                + 'static,
        >,
    >;

    /// <p>Cancels the specified simulation job.</p>
    fn cancel_simulation_job(
        &self,
        input: CancelSimulationJobRequest,
    ) -> Pin<
        Box<
            dyn Future<
                    Output = Result<
                        CancelSimulationJobResponse,
                        RusotoError<CancelSimulationJobError>,
                    >,
                > + Send
                + 'static,
        >,
    >;

    /// <p><p>Deploys a specific version of a robot application to robots in a fleet.</p> <p>The robot application must have a numbered <code>applicationVersion</code> for consistency reasons. To create a new version, use <code>CreateRobotApplicationVersion</code> or see <a href="https://docs.aws.amazon.com/robomaker/latest/dg/create-robot-application-version.html">Creating a Robot Application Version</a>. </p> <note> <p>After 90 days, deployment jobs expire and will be deleted. They will no longer be accessible. </p> </note></p>
    fn create_deployment_job(
        &self,
        input: CreateDeploymentJobRequest,
    ) -> Pin<
        Box<
            dyn Future<
                    Output = Result<
                        CreateDeploymentJobResponse,
                        RusotoError<CreateDeploymentJobError>,
                    >,
                > + Send
                + 'static,
        >,
    >;

    /// <p>Creates a fleet, a logical group of robots running the same robot application.</p>
    fn create_fleet(
        &self,
        input: CreateFleetRequest,
    ) -> Pin<
        Box<
            dyn Future<Output = Result<CreateFleetResponse, RusotoError<CreateFleetError>>>
                + Send
                + 'static,
        >,
    >;

    /// <p>Creates a robot.</p>
    fn create_robot(
        &self,
        input: CreateRobotRequest,
    ) -> Pin<
        Box<
            dyn Future<Output = Result<CreateRobotResponse, RusotoError<CreateRobotError>>>
                + Send
                + 'static,
        >,
    >;

    /// <p>Creates a robot application. </p>
    fn create_robot_application(
        &self,
        input: CreateRobotApplicationRequest,
    ) -> Pin<
        Box<
            dyn Future<
                    Output = Result<
                        CreateRobotApplicationResponse,
                        RusotoError<CreateRobotApplicationError>,
                    >,
                > + Send
                + 'static,
        >,
    >;

    /// <p>Creates a version of a robot application.</p>
    fn create_robot_application_version(
        &self,
        input: CreateRobotApplicationVersionRequest,
    ) -> Pin<
        Box<
            dyn Future<
                    Output = Result<
                        CreateRobotApplicationVersionResponse,
                        RusotoError<CreateRobotApplicationVersionError>,
                    >,
                > + Send
                + 'static,
        >,
    >;

    /// <p>Creates a simulation application.</p>
    fn create_simulation_application(
        &self,
        input: CreateSimulationApplicationRequest,
    ) -> Pin<
        Box<
            dyn Future<
                    Output = Result<
                        CreateSimulationApplicationResponse,
                        RusotoError<CreateSimulationApplicationError>,
                    >,
                > + Send
                + 'static,
        >,
    >;

    /// <p>Creates a simulation application with a specific revision id.</p>
    fn create_simulation_application_version(
        &self,
        input: CreateSimulationApplicationVersionRequest,
    ) -> Pin<
        Box<
            dyn Future<
                    Output = Result<
                        CreateSimulationApplicationVersionResponse,
                        RusotoError<CreateSimulationApplicationVersionError>,
                    >,
                > + Send
                + 'static,
        >,
    >;

    /// <p><p>Creates a simulation job.</p> <note> <p>After 90 days, simulation jobs expire and will be deleted. They will no longer be accessible. </p> </note></p>
    fn create_simulation_job(
        &self,
        input: CreateSimulationJobRequest,
    ) -> Pin<
        Box<
            dyn Future<
                    Output = Result<
                        CreateSimulationJobResponse,
                        RusotoError<CreateSimulationJobError>,
                    >,
                > + Send
                + 'static,
        >,
    >;

    /// <p>Deletes a fleet.</p>
    fn delete_fleet(
        &self,
        input: DeleteFleetRequest,
    ) -> Pin<
        Box<
            dyn Future<Output = Result<DeleteFleetResponse, RusotoError<DeleteFleetError>>>
                + Send
                + 'static,
        >,
    >;

    /// <p>Deletes a robot.</p>
    fn delete_robot(
        &self,
        input: DeleteRobotRequest,
    ) -> Pin<
        Box<
            dyn Future<Output = Result<DeleteRobotResponse, RusotoError<DeleteRobotError>>>
                + Send
                + 'static,
        >,
    >;

    /// <p>Deletes a robot application.</p>
    fn delete_robot_application(
        &self,
        input: DeleteRobotApplicationRequest,
    ) -> Pin<
        Box<
            dyn Future<
                    Output = Result<
                        DeleteRobotApplicationResponse,
                        RusotoError<DeleteRobotApplicationError>,
                    >,
                > + Send
                + 'static,
        >,
    >;

    /// <p>Deletes a simulation application.</p>
    fn delete_simulation_application(
        &self,
        input: DeleteSimulationApplicationRequest,
    ) -> Pin<
        Box<
            dyn Future<
                    Output = Result<
                        DeleteSimulationApplicationResponse,
                        RusotoError<DeleteSimulationApplicationError>,
                    >,
                > + Send
                + 'static,
        >,
    >;

    /// <p>Deregisters a robot.</p>
    fn deregister_robot(
        &self,
        input: DeregisterRobotRequest,
    ) -> Pin<
        Box<
            dyn Future<Output = Result<DeregisterRobotResponse, RusotoError<DeregisterRobotError>>>
                + Send
                + 'static,
        >,
    >;

    /// <p>Describes a deployment job.</p>
    fn describe_deployment_job(
        &self,
        input: DescribeDeploymentJobRequest,
    ) -> Pin<
        Box<
            dyn Future<
                    Output = Result<
                        DescribeDeploymentJobResponse,
                        RusotoError<DescribeDeploymentJobError>,
                    >,
                > + Send
                + 'static,
        >,
    >;

    /// <p>Describes a fleet.</p>
    fn describe_fleet(
        &self,
        input: DescribeFleetRequest,
    ) -> Pin<
        Box<
            dyn Future<Output = Result<DescribeFleetResponse, RusotoError<DescribeFleetError>>>
                + Send
                + 'static,
        >,
    >;

    /// <p>Describes a robot.</p>
    fn describe_robot(
        &self,
        input: DescribeRobotRequest,
    ) -> Pin<
        Box<
            dyn Future<Output = Result<DescribeRobotResponse, RusotoError<DescribeRobotError>>>
                + Send
                + 'static,
        >,
    >;

    /// <p>Describes a robot application.</p>
    fn describe_robot_application(
        &self,
        input: DescribeRobotApplicationRequest,
    ) -> Pin<
        Box<
            dyn Future<
                    Output = Result<
                        DescribeRobotApplicationResponse,
                        RusotoError<DescribeRobotApplicationError>,
                    >,
                > + Send
                + 'static,
        >,
    >;

    /// <p>Describes a simulation application.</p>
    fn describe_simulation_application(
        &self,
        input: DescribeSimulationApplicationRequest,
    ) -> Pin<
        Box<
            dyn Future<
                    Output = Result<
                        DescribeSimulationApplicationResponse,
                        RusotoError<DescribeSimulationApplicationError>,
                    >,
                > + Send
                + 'static,
        >,
    >;

    /// <p>Describes a simulation job.</p>
    fn describe_simulation_job(
        &self,
        input: DescribeSimulationJobRequest,
    ) -> Pin<
        Box<
            dyn Future<
                    Output = Result<
                        DescribeSimulationJobResponse,
                        RusotoError<DescribeSimulationJobError>,
                    >,
                > + Send
                + 'static,
        >,
    >;

    /// <p><p>Returns a list of deployment jobs for a fleet. You can optionally provide filters to retrieve specific deployment jobs. </p> <note> <p> </p> </note></p>
    fn list_deployment_jobs(
        &self,
        input: ListDeploymentJobsRequest,
    ) -> Pin<
        Box<
            dyn Future<
                    Output = Result<
                        ListDeploymentJobsResponse,
                        RusotoError<ListDeploymentJobsError>,
                    >,
                > + Send
                + 'static,
        >,
    >;

    /// <p>Returns a list of fleets. You can optionally provide filters to retrieve specific fleets. </p>
    fn list_fleets(
        &self,
        input: ListFleetsRequest,
    ) -> Pin<
        Box<
            dyn Future<Output = Result<ListFleetsResponse, RusotoError<ListFleetsError>>>
                + Send
                + 'static,
        >,
    >;

    /// <p>Returns a list of robot application. You can optionally provide filters to retrieve specific robot applications.</p>
    fn list_robot_applications(
        &self,
        input: ListRobotApplicationsRequest,
    ) -> Pin<
        Box<
            dyn Future<
                    Output = Result<
                        ListRobotApplicationsResponse,
                        RusotoError<ListRobotApplicationsError>,
                    >,
                > + Send
                + 'static,
        >,
    >;

    /// <p>Returns a list of robots. You can optionally provide filters to retrieve specific robots.</p>
    fn list_robots(
        &self,
        input: ListRobotsRequest,
    ) -> Pin<
        Box<
            dyn Future<Output = Result<ListRobotsResponse, RusotoError<ListRobotsError>>>
                + Send
                + 'static,
        >,
    >;

    /// <p>Returns a list of simulation applications. You can optionally provide filters to retrieve specific simulation applications. </p>
    fn list_simulation_applications(
        &self,
        input: ListSimulationApplicationsRequest,
    ) -> Pin<
        Box<
            dyn Future<
                    Output = Result<
                        ListSimulationApplicationsResponse,
                        RusotoError<ListSimulationApplicationsError>,
                    >,
                > + Send
                + 'static,
        >,
    >;

    /// <p>Returns a list of simulation jobs. You can optionally provide filters to retrieve specific simulation jobs. </p>
    fn list_simulation_jobs(
        &self,
        input: ListSimulationJobsRequest,
    ) -> Pin<
        Box<
            dyn Future<
                    Output = Result<
                        ListSimulationJobsResponse,
                        RusotoError<ListSimulationJobsError>,
                    >,
                > + Send
                + 'static,
        >,
    >;

    /// <p>Lists all tags on a AWS RoboMaker resource.</p>
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

    /// <p>Registers a robot with a fleet.</p>
    fn register_robot(
        &self,
        input: RegisterRobotRequest,
    ) -> Pin<
        Box<
            dyn Future<Output = Result<RegisterRobotResponse, RusotoError<RegisterRobotError>>>
                + Send
                + 'static,
        >,
    >;

    /// <p>Restarts a running simulation job.</p>
    fn restart_simulation_job(
        &self,
        input: RestartSimulationJobRequest,
    ) -> Pin<
        Box<
            dyn Future<
                    Output = Result<
                        RestartSimulationJobResponse,
                        RusotoError<RestartSimulationJobError>,
                    >,
                > + Send
                + 'static,
        >,
    >;

    /// <p>Syncrhonizes robots in a fleet to the latest deployment. This is helpful if robots were added after a deployment.</p>
    fn sync_deployment_job(
        &self,
        input: SyncDeploymentJobRequest,
    ) -> Pin<
        Box<
            dyn Future<
                    Output = Result<SyncDeploymentJobResponse, RusotoError<SyncDeploymentJobError>>,
                > + Send
                + 'static,
        >,
    >;

    /// <p>Adds or edits tags for a AWS RoboMaker resource.</p> <p>Each tag consists of a tag key and a tag value. Tag keys and tag values are both required, but tag values can be empty strings. </p> <p>For information about the rules that apply to tag keys and tag values, see <a href="https://docs.aws.amazon.com/awsaccountbilling/latest/aboutv2/allocation-tag-restrictions.html">User-Defined Tag Restrictions</a> in the <i>AWS Billing and Cost Management User Guide</i>. </p>
    fn tag_resource(
        &self,
        input: TagResourceRequest,
    ) -> Pin<
        Box<
            dyn Future<Output = Result<TagResourceResponse, RusotoError<TagResourceError>>>
                + Send
                + 'static,
        >,
    >;

    /// <p>Removes the specified tags from the specified AWS RoboMaker resource.</p> <p>To remove a tag, specify the tag key. To change the tag value of an existing tag key, use <a href="https://docs.aws.amazon.com/robomaker/latest/dg/API_TagResource.html"> <code>TagResource</code> </a>. </p>
    fn untag_resource(
        &self,
        input: UntagResourceRequest,
    ) -> Pin<
        Box<
            dyn Future<Output = Result<UntagResourceResponse, RusotoError<UntagResourceError>>>
                + Send
                + 'static,
        >,
    >;

    /// <p>Updates a robot application.</p>
    fn update_robot_application(
        &self,
        input: UpdateRobotApplicationRequest,
    ) -> Pin<
        Box<
            dyn Future<
                    Output = Result<
                        UpdateRobotApplicationResponse,
                        RusotoError<UpdateRobotApplicationError>,
                    >,
                > + Send
                + 'static,
        >,
    >;

    /// <p>Updates a simulation application.</p>
    fn update_simulation_application(
        &self,
        input: UpdateSimulationApplicationRequest,
    ) -> Pin<
        Box<
            dyn Future<
                    Output = Result<
                        UpdateSimulationApplicationResponse,
                        RusotoError<UpdateSimulationApplicationError>,
                    >,
                > + Send
                + 'static,
        >,
    >;
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

impl Robomaker for RobomakerClient {
    /// <p>Describes one or more simulation jobs.</p>
    fn batch_describe_simulation_job(
        &self,
        input: BatchDescribeSimulationJobRequest,
    ) -> Pin<
        Box<
            dyn Future<
                    Output = Result<
                        BatchDescribeSimulationJobResponse,
                        RusotoError<BatchDescribeSimulationJobError>,
                    >,
                > + Send
                + 'static,
        >,
    > {
        let request_uri = "/batchDescribeSimulationJob";

        let mut request = SignedRequest::new("POST", "robomaker", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let fut = self.client.sign_and_dispatch(request);
        async move {
            let mut response = fut.await.map_err(RusotoError::from)?;
            if response.status.is_success() {
                let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
                let result = proto::json::ResponsePayload::new(&response)
                    .deserialize::<BatchDescribeSimulationJobResponse, _>()?;

                Ok(result)
            } else {
                let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
                Err(BatchDescribeSimulationJobError::from_response(response))
            }
        }
        .boxed()
    }

    /// <p>Cancels the specified deployment job.</p>
    fn cancel_deployment_job(
        &self,
        input: CancelDeploymentJobRequest,
    ) -> Pin<
        Box<
            dyn Future<
                    Output = Result<
                        CancelDeploymentJobResponse,
                        RusotoError<CancelDeploymentJobError>,
                    >,
                > + Send
                + 'static,
        >,
    > {
        let request_uri = "/cancelDeploymentJob";

        let mut request = SignedRequest::new("POST", "robomaker", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let fut = self.client.sign_and_dispatch(request);
        async move {
            let mut response = fut.await.map_err(RusotoError::from)?;
            if response.status.is_success() {
                let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
                let result = proto::json::ResponsePayload::new(&response)
                    .deserialize::<CancelDeploymentJobResponse, _>()?;

                Ok(result)
            } else {
                let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
                Err(CancelDeploymentJobError::from_response(response))
            }
        }
        .boxed()
    }

    /// <p>Cancels the specified simulation job.</p>
    fn cancel_simulation_job(
        &self,
        input: CancelSimulationJobRequest,
    ) -> Pin<
        Box<
            dyn Future<
                    Output = Result<
                        CancelSimulationJobResponse,
                        RusotoError<CancelSimulationJobError>,
                    >,
                > + Send
                + 'static,
        >,
    > {
        let request_uri = "/cancelSimulationJob";

        let mut request = SignedRequest::new("POST", "robomaker", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let fut = self.client.sign_and_dispatch(request);
        async move {
            let mut response = fut.await.map_err(RusotoError::from)?;
            if response.status.is_success() {
                let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
                let result = proto::json::ResponsePayload::new(&response)
                    .deserialize::<CancelSimulationJobResponse, _>()?;

                Ok(result)
            } else {
                let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
                Err(CancelSimulationJobError::from_response(response))
            }
        }
        .boxed()
    }

    /// <p><p>Deploys a specific version of a robot application to robots in a fleet.</p> <p>The robot application must have a numbered <code>applicationVersion</code> for consistency reasons. To create a new version, use <code>CreateRobotApplicationVersion</code> or see <a href="https://docs.aws.amazon.com/robomaker/latest/dg/create-robot-application-version.html">Creating a Robot Application Version</a>. </p> <note> <p>After 90 days, deployment jobs expire and will be deleted. They will no longer be accessible. </p> </note></p>
    fn create_deployment_job(
        &self,
        input: CreateDeploymentJobRequest,
    ) -> Pin<
        Box<
            dyn Future<
                    Output = Result<
                        CreateDeploymentJobResponse,
                        RusotoError<CreateDeploymentJobError>,
                    >,
                > + Send
                + 'static,
        >,
    > {
        let request_uri = "/createDeploymentJob";

        let mut request = SignedRequest::new("POST", "robomaker", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let fut = self.client.sign_and_dispatch(request);
        async move {
            let mut response = fut.await.map_err(RusotoError::from)?;
            if response.status.is_success() {
                let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
                let result = proto::json::ResponsePayload::new(&response)
                    .deserialize::<CreateDeploymentJobResponse, _>()?;

                Ok(result)
            } else {
                let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
                Err(CreateDeploymentJobError::from_response(response))
            }
        }
        .boxed()
    }

    /// <p>Creates a fleet, a logical group of robots running the same robot application.</p>
    fn create_fleet(
        &self,
        input: CreateFleetRequest,
    ) -> Pin<
        Box<
            dyn Future<Output = Result<CreateFleetResponse, RusotoError<CreateFleetError>>>
                + Send
                + 'static,
        >,
    > {
        let request_uri = "/createFleet";

        let mut request = SignedRequest::new("POST", "robomaker", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let fut = self.client.sign_and_dispatch(request);
        async move {
            let mut response = fut.await.map_err(RusotoError::from)?;
            if response.status.is_success() {
                let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
                let result = proto::json::ResponsePayload::new(&response)
                    .deserialize::<CreateFleetResponse, _>()?;

                Ok(result)
            } else {
                let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
                Err(CreateFleetError::from_response(response))
            }
        }
        .boxed()
    }

    /// <p>Creates a robot.</p>
    fn create_robot(
        &self,
        input: CreateRobotRequest,
    ) -> Pin<
        Box<
            dyn Future<Output = Result<CreateRobotResponse, RusotoError<CreateRobotError>>>
                + Send
                + 'static,
        >,
    > {
        let request_uri = "/createRobot";

        let mut request = SignedRequest::new("POST", "robomaker", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let fut = self.client.sign_and_dispatch(request);
        async move {
            let mut response = fut.await.map_err(RusotoError::from)?;
            if response.status.is_success() {
                let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
                let result = proto::json::ResponsePayload::new(&response)
                    .deserialize::<CreateRobotResponse, _>()?;

                Ok(result)
            } else {
                let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
                Err(CreateRobotError::from_response(response))
            }
        }
        .boxed()
    }

    /// <p>Creates a robot application. </p>
    fn create_robot_application(
        &self,
        input: CreateRobotApplicationRequest,
    ) -> Pin<
        Box<
            dyn Future<
                    Output = Result<
                        CreateRobotApplicationResponse,
                        RusotoError<CreateRobotApplicationError>,
                    >,
                > + Send
                + 'static,
        >,
    > {
        let request_uri = "/createRobotApplication";

        let mut request = SignedRequest::new("POST", "robomaker", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let fut = self.client.sign_and_dispatch(request);
        async move {
            let mut response = fut.await.map_err(RusotoError::from)?;
            if response.status.is_success() {
                let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
                let result = proto::json::ResponsePayload::new(&response)
                    .deserialize::<CreateRobotApplicationResponse, _>()?;

                Ok(result)
            } else {
                let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
                Err(CreateRobotApplicationError::from_response(response))
            }
        }
        .boxed()
    }

    /// <p>Creates a version of a robot application.</p>
    fn create_robot_application_version(
        &self,
        input: CreateRobotApplicationVersionRequest,
    ) -> Pin<
        Box<
            dyn Future<
                    Output = Result<
                        CreateRobotApplicationVersionResponse,
                        RusotoError<CreateRobotApplicationVersionError>,
                    >,
                > + Send
                + 'static,
        >,
    > {
        let request_uri = "/createRobotApplicationVersion";

        let mut request = SignedRequest::new("POST", "robomaker", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let fut = self.client.sign_and_dispatch(request);
        async move {
            let mut response = fut.await.map_err(RusotoError::from)?;
            if response.status.is_success() {
                let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
                let result = proto::json::ResponsePayload::new(&response)
                    .deserialize::<CreateRobotApplicationVersionResponse, _>()?;

                Ok(result)
            } else {
                let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
                Err(CreateRobotApplicationVersionError::from_response(response))
            }
        }
        .boxed()
    }

    /// <p>Creates a simulation application.</p>
    fn create_simulation_application(
        &self,
        input: CreateSimulationApplicationRequest,
    ) -> Pin<
        Box<
            dyn Future<
                    Output = Result<
                        CreateSimulationApplicationResponse,
                        RusotoError<CreateSimulationApplicationError>,
                    >,
                > + Send
                + 'static,
        >,
    > {
        let request_uri = "/createSimulationApplication";

        let mut request = SignedRequest::new("POST", "robomaker", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let fut = self.client.sign_and_dispatch(request);
        async move {
            let mut response = fut.await.map_err(RusotoError::from)?;
            if response.status.is_success() {
                let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
                let result = proto::json::ResponsePayload::new(&response)
                    .deserialize::<CreateSimulationApplicationResponse, _>()?;

                Ok(result)
            } else {
                let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
                Err(CreateSimulationApplicationError::from_response(response))
            }
        }
        .boxed()
    }

    /// <p>Creates a simulation application with a specific revision id.</p>
    fn create_simulation_application_version(
        &self,
        input: CreateSimulationApplicationVersionRequest,
    ) -> Pin<
        Box<
            dyn Future<
                    Output = Result<
                        CreateSimulationApplicationVersionResponse,
                        RusotoError<CreateSimulationApplicationVersionError>,
                    >,
                > + Send
                + 'static,
        >,
    > {
        let request_uri = "/createSimulationApplicationVersion";

        let mut request = SignedRequest::new("POST", "robomaker", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let fut = self.client.sign_and_dispatch(request);
        async move {
            let mut response = fut.await.map_err(RusotoError::from)?;
            if response.status.is_success() {
                let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
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
        .boxed()
    }

    /// <p><p>Creates a simulation job.</p> <note> <p>After 90 days, simulation jobs expire and will be deleted. They will no longer be accessible. </p> </note></p>
    fn create_simulation_job(
        &self,
        input: CreateSimulationJobRequest,
    ) -> Pin<
        Box<
            dyn Future<
                    Output = Result<
                        CreateSimulationJobResponse,
                        RusotoError<CreateSimulationJobError>,
                    >,
                > + Send
                + 'static,
        >,
    > {
        let request_uri = "/createSimulationJob";

        let mut request = SignedRequest::new("POST", "robomaker", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let fut = self.client.sign_and_dispatch(request);
        async move {
            let mut response = fut.await.map_err(RusotoError::from)?;
            if response.status.is_success() {
                let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
                let result = proto::json::ResponsePayload::new(&response)
                    .deserialize::<CreateSimulationJobResponse, _>()?;

                Ok(result)
            } else {
                let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
                Err(CreateSimulationJobError::from_response(response))
            }
        }
        .boxed()
    }

    /// <p>Deletes a fleet.</p>
    fn delete_fleet(
        &self,
        input: DeleteFleetRequest,
    ) -> Pin<
        Box<
            dyn Future<Output = Result<DeleteFleetResponse, RusotoError<DeleteFleetError>>>
                + Send
                + 'static,
        >,
    > {
        let request_uri = "/deleteFleet";

        let mut request = SignedRequest::new("POST", "robomaker", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let fut = self.client.sign_and_dispatch(request);
        async move {
            let mut response = fut.await.map_err(RusotoError::from)?;
            if response.status.is_success() {
                let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
                let result = proto::json::ResponsePayload::new(&response)
                    .deserialize::<DeleteFleetResponse, _>()?;

                Ok(result)
            } else {
                let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
                Err(DeleteFleetError::from_response(response))
            }
        }
        .boxed()
    }

    /// <p>Deletes a robot.</p>
    fn delete_robot(
        &self,
        input: DeleteRobotRequest,
    ) -> Pin<
        Box<
            dyn Future<Output = Result<DeleteRobotResponse, RusotoError<DeleteRobotError>>>
                + Send
                + 'static,
        >,
    > {
        let request_uri = "/deleteRobot";

        let mut request = SignedRequest::new("POST", "robomaker", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let fut = self.client.sign_and_dispatch(request);
        async move {
            let mut response = fut.await.map_err(RusotoError::from)?;
            if response.status.is_success() {
                let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
                let result = proto::json::ResponsePayload::new(&response)
                    .deserialize::<DeleteRobotResponse, _>()?;

                Ok(result)
            } else {
                let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
                Err(DeleteRobotError::from_response(response))
            }
        }
        .boxed()
    }

    /// <p>Deletes a robot application.</p>
    fn delete_robot_application(
        &self,
        input: DeleteRobotApplicationRequest,
    ) -> Pin<
        Box<
            dyn Future<
                    Output = Result<
                        DeleteRobotApplicationResponse,
                        RusotoError<DeleteRobotApplicationError>,
                    >,
                > + Send
                + 'static,
        >,
    > {
        let request_uri = "/deleteRobotApplication";

        let mut request = SignedRequest::new("POST", "robomaker", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let fut = self.client.sign_and_dispatch(request);
        async move {
            let mut response = fut.await.map_err(RusotoError::from)?;
            if response.status.is_success() {
                let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
                let result = proto::json::ResponsePayload::new(&response)
                    .deserialize::<DeleteRobotApplicationResponse, _>()?;

                Ok(result)
            } else {
                let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
                Err(DeleteRobotApplicationError::from_response(response))
            }
        }
        .boxed()
    }

    /// <p>Deletes a simulation application.</p>
    fn delete_simulation_application(
        &self,
        input: DeleteSimulationApplicationRequest,
    ) -> Pin<
        Box<
            dyn Future<
                    Output = Result<
                        DeleteSimulationApplicationResponse,
                        RusotoError<DeleteSimulationApplicationError>,
                    >,
                > + Send
                + 'static,
        >,
    > {
        let request_uri = "/deleteSimulationApplication";

        let mut request = SignedRequest::new("POST", "robomaker", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let fut = self.client.sign_and_dispatch(request);
        async move {
            let mut response = fut.await.map_err(RusotoError::from)?;
            if response.status.is_success() {
                let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
                let result = proto::json::ResponsePayload::new(&response)
                    .deserialize::<DeleteSimulationApplicationResponse, _>()?;

                Ok(result)
            } else {
                let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
                Err(DeleteSimulationApplicationError::from_response(response))
            }
        }
        .boxed()
    }

    /// <p>Deregisters a robot.</p>
    fn deregister_robot(
        &self,
        input: DeregisterRobotRequest,
    ) -> Pin<
        Box<
            dyn Future<Output = Result<DeregisterRobotResponse, RusotoError<DeregisterRobotError>>>
                + Send
                + 'static,
        >,
    > {
        let request_uri = "/deregisterRobot";

        let mut request = SignedRequest::new("POST", "robomaker", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let fut = self.client.sign_and_dispatch(request);
        async move {
            let mut response = fut.await.map_err(RusotoError::from)?;
            if response.status.is_success() {
                let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
                let result = proto::json::ResponsePayload::new(&response)
                    .deserialize::<DeregisterRobotResponse, _>()?;

                Ok(result)
            } else {
                let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
                Err(DeregisterRobotError::from_response(response))
            }
        }
        .boxed()
    }

    /// <p>Describes a deployment job.</p>
    fn describe_deployment_job(
        &self,
        input: DescribeDeploymentJobRequest,
    ) -> Pin<
        Box<
            dyn Future<
                    Output = Result<
                        DescribeDeploymentJobResponse,
                        RusotoError<DescribeDeploymentJobError>,
                    >,
                > + Send
                + 'static,
        >,
    > {
        let request_uri = "/describeDeploymentJob";

        let mut request = SignedRequest::new("POST", "robomaker", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let fut = self.client.sign_and_dispatch(request);
        async move {
            let mut response = fut.await.map_err(RusotoError::from)?;
            if response.status.is_success() {
                let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
                let result = proto::json::ResponsePayload::new(&response)
                    .deserialize::<DescribeDeploymentJobResponse, _>()?;

                Ok(result)
            } else {
                let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
                Err(DescribeDeploymentJobError::from_response(response))
            }
        }
        .boxed()
    }

    /// <p>Describes a fleet.</p>
    fn describe_fleet(
        &self,
        input: DescribeFleetRequest,
    ) -> Pin<
        Box<
            dyn Future<Output = Result<DescribeFleetResponse, RusotoError<DescribeFleetError>>>
                + Send
                + 'static,
        >,
    > {
        let request_uri = "/describeFleet";

        let mut request = SignedRequest::new("POST", "robomaker", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let fut = self.client.sign_and_dispatch(request);
        async move {
            let mut response = fut.await.map_err(RusotoError::from)?;
            if response.status.is_success() {
                let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
                let result = proto::json::ResponsePayload::new(&response)
                    .deserialize::<DescribeFleetResponse, _>()?;

                Ok(result)
            } else {
                let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
                Err(DescribeFleetError::from_response(response))
            }
        }
        .boxed()
    }

    /// <p>Describes a robot.</p>
    fn describe_robot(
        &self,
        input: DescribeRobotRequest,
    ) -> Pin<
        Box<
            dyn Future<Output = Result<DescribeRobotResponse, RusotoError<DescribeRobotError>>>
                + Send
                + 'static,
        >,
    > {
        let request_uri = "/describeRobot";

        let mut request = SignedRequest::new("POST", "robomaker", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let fut = self.client.sign_and_dispatch(request);
        async move {
            let mut response = fut.await.map_err(RusotoError::from)?;
            if response.status.is_success() {
                let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
                let result = proto::json::ResponsePayload::new(&response)
                    .deserialize::<DescribeRobotResponse, _>()?;

                Ok(result)
            } else {
                let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
                Err(DescribeRobotError::from_response(response))
            }
        }
        .boxed()
    }

    /// <p>Describes a robot application.</p>
    fn describe_robot_application(
        &self,
        input: DescribeRobotApplicationRequest,
    ) -> Pin<
        Box<
            dyn Future<
                    Output = Result<
                        DescribeRobotApplicationResponse,
                        RusotoError<DescribeRobotApplicationError>,
                    >,
                > + Send
                + 'static,
        >,
    > {
        let request_uri = "/describeRobotApplication";

        let mut request = SignedRequest::new("POST", "robomaker", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let fut = self.client.sign_and_dispatch(request);
        async move {
            let mut response = fut.await.map_err(RusotoError::from)?;
            if response.status.is_success() {
                let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
                let result = proto::json::ResponsePayload::new(&response)
                    .deserialize::<DescribeRobotApplicationResponse, _>()?;

                Ok(result)
            } else {
                let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
                Err(DescribeRobotApplicationError::from_response(response))
            }
        }
        .boxed()
    }

    /// <p>Describes a simulation application.</p>
    fn describe_simulation_application(
        &self,
        input: DescribeSimulationApplicationRequest,
    ) -> Pin<
        Box<
            dyn Future<
                    Output = Result<
                        DescribeSimulationApplicationResponse,
                        RusotoError<DescribeSimulationApplicationError>,
                    >,
                > + Send
                + 'static,
        >,
    > {
        let request_uri = "/describeSimulationApplication";

        let mut request = SignedRequest::new("POST", "robomaker", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let fut = self.client.sign_and_dispatch(request);
        async move {
            let mut response = fut.await.map_err(RusotoError::from)?;
            if response.status.is_success() {
                let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
                let result = proto::json::ResponsePayload::new(&response)
                    .deserialize::<DescribeSimulationApplicationResponse, _>()?;

                Ok(result)
            } else {
                let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
                Err(DescribeSimulationApplicationError::from_response(response))
            }
        }
        .boxed()
    }

    /// <p>Describes a simulation job.</p>
    fn describe_simulation_job(
        &self,
        input: DescribeSimulationJobRequest,
    ) -> Pin<
        Box<
            dyn Future<
                    Output = Result<
                        DescribeSimulationJobResponse,
                        RusotoError<DescribeSimulationJobError>,
                    >,
                > + Send
                + 'static,
        >,
    > {
        let request_uri = "/describeSimulationJob";

        let mut request = SignedRequest::new("POST", "robomaker", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let fut = self.client.sign_and_dispatch(request);
        async move {
            let mut response = fut.await.map_err(RusotoError::from)?;
            if response.status.is_success() {
                let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
                let result = proto::json::ResponsePayload::new(&response)
                    .deserialize::<DescribeSimulationJobResponse, _>()?;

                Ok(result)
            } else {
                let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
                Err(DescribeSimulationJobError::from_response(response))
            }
        }
        .boxed()
    }

    /// <p><p>Returns a list of deployment jobs for a fleet. You can optionally provide filters to retrieve specific deployment jobs. </p> <note> <p> </p> </note></p>
    fn list_deployment_jobs(
        &self,
        input: ListDeploymentJobsRequest,
    ) -> Pin<
        Box<
            dyn Future<
                    Output = Result<
                        ListDeploymentJobsResponse,
                        RusotoError<ListDeploymentJobsError>,
                    >,
                > + Send
                + 'static,
        >,
    > {
        let request_uri = "/listDeploymentJobs";

        let mut request = SignedRequest::new("POST", "robomaker", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let fut = self.client.sign_and_dispatch(request);
        async move {
            let mut response = fut.await.map_err(RusotoError::from)?;
            if response.status.is_success() {
                let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
                let result = proto::json::ResponsePayload::new(&response)
                    .deserialize::<ListDeploymentJobsResponse, _>()?;

                Ok(result)
            } else {
                let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
                Err(ListDeploymentJobsError::from_response(response))
            }
        }
        .boxed()
    }

    /// <p>Returns a list of fleets. You can optionally provide filters to retrieve specific fleets. </p>
    fn list_fleets(
        &self,
        input: ListFleetsRequest,
    ) -> Pin<
        Box<
            dyn Future<Output = Result<ListFleetsResponse, RusotoError<ListFleetsError>>>
                + Send
                + 'static,
        >,
    > {
        let request_uri = "/listFleets";

        let mut request = SignedRequest::new("POST", "robomaker", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let fut = self.client.sign_and_dispatch(request);
        async move {
            let mut response = fut.await.map_err(RusotoError::from)?;
            if response.status.is_success() {
                let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
                let result = proto::json::ResponsePayload::new(&response)
                    .deserialize::<ListFleetsResponse, _>()?;

                Ok(result)
            } else {
                let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
                Err(ListFleetsError::from_response(response))
            }
        }
        .boxed()
    }

    /// <p>Returns a list of robot application. You can optionally provide filters to retrieve specific robot applications.</p>
    fn list_robot_applications(
        &self,
        input: ListRobotApplicationsRequest,
    ) -> Pin<
        Box<
            dyn Future<
                    Output = Result<
                        ListRobotApplicationsResponse,
                        RusotoError<ListRobotApplicationsError>,
                    >,
                > + Send
                + 'static,
        >,
    > {
        let request_uri = "/listRobotApplications";

        let mut request = SignedRequest::new("POST", "robomaker", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let fut = self.client.sign_and_dispatch(request);
        async move {
            let mut response = fut.await.map_err(RusotoError::from)?;
            if response.status.is_success() {
                let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
                let result = proto::json::ResponsePayload::new(&response)
                    .deserialize::<ListRobotApplicationsResponse, _>()?;

                Ok(result)
            } else {
                let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
                Err(ListRobotApplicationsError::from_response(response))
            }
        }
        .boxed()
    }

    /// <p>Returns a list of robots. You can optionally provide filters to retrieve specific robots.</p>
    fn list_robots(
        &self,
        input: ListRobotsRequest,
    ) -> Pin<
        Box<
            dyn Future<Output = Result<ListRobotsResponse, RusotoError<ListRobotsError>>>
                + Send
                + 'static,
        >,
    > {
        let request_uri = "/listRobots";

        let mut request = SignedRequest::new("POST", "robomaker", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let fut = self.client.sign_and_dispatch(request);
        async move {
            let mut response = fut.await.map_err(RusotoError::from)?;
            if response.status.is_success() {
                let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
                let result = proto::json::ResponsePayload::new(&response)
                    .deserialize::<ListRobotsResponse, _>()?;

                Ok(result)
            } else {
                let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
                Err(ListRobotsError::from_response(response))
            }
        }
        .boxed()
    }

    /// <p>Returns a list of simulation applications. You can optionally provide filters to retrieve specific simulation applications. </p>
    fn list_simulation_applications(
        &self,
        input: ListSimulationApplicationsRequest,
    ) -> Pin<
        Box<
            dyn Future<
                    Output = Result<
                        ListSimulationApplicationsResponse,
                        RusotoError<ListSimulationApplicationsError>,
                    >,
                > + Send
                + 'static,
        >,
    > {
        let request_uri = "/listSimulationApplications";

        let mut request = SignedRequest::new("POST", "robomaker", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let fut = self.client.sign_and_dispatch(request);
        async move {
            let mut response = fut.await.map_err(RusotoError::from)?;
            if response.status.is_success() {
                let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
                let result = proto::json::ResponsePayload::new(&response)
                    .deserialize::<ListSimulationApplicationsResponse, _>()?;

                Ok(result)
            } else {
                let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
                Err(ListSimulationApplicationsError::from_response(response))
            }
        }
        .boxed()
    }

    /// <p>Returns a list of simulation jobs. You can optionally provide filters to retrieve specific simulation jobs. </p>
    fn list_simulation_jobs(
        &self,
        input: ListSimulationJobsRequest,
    ) -> Pin<
        Box<
            dyn Future<
                    Output = Result<
                        ListSimulationJobsResponse,
                        RusotoError<ListSimulationJobsError>,
                    >,
                > + Send
                + 'static,
        >,
    > {
        let request_uri = "/listSimulationJobs";

        let mut request = SignedRequest::new("POST", "robomaker", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let fut = self.client.sign_and_dispatch(request);
        async move {
            let mut response = fut.await.map_err(RusotoError::from)?;
            if response.status.is_success() {
                let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
                let result = proto::json::ResponsePayload::new(&response)
                    .deserialize::<ListSimulationJobsResponse, _>()?;

                Ok(result)
            } else {
                let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
                Err(ListSimulationJobsError::from_response(response))
            }
        }
        .boxed()
    }

    /// <p>Lists all tags on a AWS RoboMaker resource.</p>
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

        let mut request = SignedRequest::new("GET", "robomaker", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let fut = self.client.sign_and_dispatch(request);
        async move {
            let mut response = fut.await.map_err(RusotoError::from)?;
            if response.status.is_success() {
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

    /// <p>Registers a robot with a fleet.</p>
    fn register_robot(
        &self,
        input: RegisterRobotRequest,
    ) -> Pin<
        Box<
            dyn Future<Output = Result<RegisterRobotResponse, RusotoError<RegisterRobotError>>>
                + Send
                + 'static,
        >,
    > {
        let request_uri = "/registerRobot";

        let mut request = SignedRequest::new("POST", "robomaker", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let fut = self.client.sign_and_dispatch(request);
        async move {
            let mut response = fut.await.map_err(RusotoError::from)?;
            if response.status.is_success() {
                let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
                let result = proto::json::ResponsePayload::new(&response)
                    .deserialize::<RegisterRobotResponse, _>()?;

                Ok(result)
            } else {
                let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
                Err(RegisterRobotError::from_response(response))
            }
        }
        .boxed()
    }

    /// <p>Restarts a running simulation job.</p>
    fn restart_simulation_job(
        &self,
        input: RestartSimulationJobRequest,
    ) -> Pin<
        Box<
            dyn Future<
                    Output = Result<
                        RestartSimulationJobResponse,
                        RusotoError<RestartSimulationJobError>,
                    >,
                > + Send
                + 'static,
        >,
    > {
        let request_uri = "/restartSimulationJob";

        let mut request = SignedRequest::new("POST", "robomaker", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let fut = self.client.sign_and_dispatch(request);
        async move {
            let mut response = fut.await.map_err(RusotoError::from)?;
            if response.status.is_success() {
                let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
                let result = proto::json::ResponsePayload::new(&response)
                    .deserialize::<RestartSimulationJobResponse, _>()?;

                Ok(result)
            } else {
                let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
                Err(RestartSimulationJobError::from_response(response))
            }
        }
        .boxed()
    }

    /// <p>Syncrhonizes robots in a fleet to the latest deployment. This is helpful if robots were added after a deployment.</p>
    fn sync_deployment_job(
        &self,
        input: SyncDeploymentJobRequest,
    ) -> Pin<
        Box<
            dyn Future<
                    Output = Result<SyncDeploymentJobResponse, RusotoError<SyncDeploymentJobError>>,
                > + Send
                + 'static,
        >,
    > {
        let request_uri = "/syncDeploymentJob";

        let mut request = SignedRequest::new("POST", "robomaker", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let fut = self.client.sign_and_dispatch(request);
        async move {
            let mut response = fut.await.map_err(RusotoError::from)?;
            if response.status.is_success() {
                let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
                let result = proto::json::ResponsePayload::new(&response)
                    .deserialize::<SyncDeploymentJobResponse, _>()?;

                Ok(result)
            } else {
                let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
                Err(SyncDeploymentJobError::from_response(response))
            }
        }
        .boxed()
    }

    /// <p>Adds or edits tags for a AWS RoboMaker resource.</p> <p>Each tag consists of a tag key and a tag value. Tag keys and tag values are both required, but tag values can be empty strings. </p> <p>For information about the rules that apply to tag keys and tag values, see <a href="https://docs.aws.amazon.com/awsaccountbilling/latest/aboutv2/allocation-tag-restrictions.html">User-Defined Tag Restrictions</a> in the <i>AWS Billing and Cost Management User Guide</i>. </p>
    fn tag_resource(
        &self,
        input: TagResourceRequest,
    ) -> Pin<
        Box<
            dyn Future<Output = Result<TagResourceResponse, RusotoError<TagResourceError>>>
                + Send
                + 'static,
        >,
    > {
        let request_uri = format!("/tags/{resource_arn}", resource_arn = input.resource_arn);

        let mut request = SignedRequest::new("POST", "robomaker", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let fut = self.client.sign_and_dispatch(request);
        async move {
            let mut response = fut.await.map_err(RusotoError::from)?;
            if response.status.is_success() {
                let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
                let result = proto::json::ResponsePayload::new(&response)
                    .deserialize::<TagResourceResponse, _>()?;

                Ok(result)
            } else {
                let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
                Err(TagResourceError::from_response(response))
            }
        }
        .boxed()
    }

    /// <p>Removes the specified tags from the specified AWS RoboMaker resource.</p> <p>To remove a tag, specify the tag key. To change the tag value of an existing tag key, use <a href="https://docs.aws.amazon.com/robomaker/latest/dg/API_TagResource.html"> <code>TagResource</code> </a>. </p>
    fn untag_resource(
        &self,
        input: UntagResourceRequest,
    ) -> Pin<
        Box<
            dyn Future<Output = Result<UntagResourceResponse, RusotoError<UntagResourceError>>>
                + Send
                + 'static,
        >,
    > {
        let request_uri = format!("/tags/{resource_arn}", resource_arn = input.resource_arn);

        let mut request = SignedRequest::new("DELETE", "robomaker", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut params = Params::new();
        for item in input.tag_keys.iter() {
            params.put("tagKeys", item);
        }
        request.set_params(params);

        let fut = self.client.sign_and_dispatch(request);
        async move {
            let mut response = fut.await.map_err(RusotoError::from)?;
            if response.status.is_success() {
                let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
                let result = proto::json::ResponsePayload::new(&response)
                    .deserialize::<UntagResourceResponse, _>()?;

                Ok(result)
            } else {
                let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
                Err(UntagResourceError::from_response(response))
            }
        }
        .boxed()
    }

    /// <p>Updates a robot application.</p>
    fn update_robot_application(
        &self,
        input: UpdateRobotApplicationRequest,
    ) -> Pin<
        Box<
            dyn Future<
                    Output = Result<
                        UpdateRobotApplicationResponse,
                        RusotoError<UpdateRobotApplicationError>,
                    >,
                > + Send
                + 'static,
        >,
    > {
        let request_uri = "/updateRobotApplication";

        let mut request = SignedRequest::new("POST", "robomaker", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let fut = self.client.sign_and_dispatch(request);
        async move {
            let mut response = fut.await.map_err(RusotoError::from)?;
            if response.status.is_success() {
                let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
                let result = proto::json::ResponsePayload::new(&response)
                    .deserialize::<UpdateRobotApplicationResponse, _>()?;

                Ok(result)
            } else {
                let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
                Err(UpdateRobotApplicationError::from_response(response))
            }
        }
        .boxed()
    }

    /// <p>Updates a simulation application.</p>
    fn update_simulation_application(
        &self,
        input: UpdateSimulationApplicationRequest,
    ) -> Pin<
        Box<
            dyn Future<
                    Output = Result<
                        UpdateSimulationApplicationResponse,
                        RusotoError<UpdateSimulationApplicationError>,
                    >,
                > + Send
                + 'static,
        >,
    > {
        let request_uri = "/updateSimulationApplication";

        let mut request = SignedRequest::new("POST", "robomaker", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let fut = self.client.sign_and_dispatch(request);
        async move {
            let mut response = fut.await.map_err(RusotoError::from)?;
            if response.status.is_success() {
                let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
                let result = proto::json::ResponsePayload::new(&response)
                    .deserialize::<UpdateSimulationApplicationResponse, _>()?;

                Ok(result)
            } else {
                let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
                Err(UpdateSimulationApplicationError::from_response(response))
            }
        }
        .boxed()
    }
}
