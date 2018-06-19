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
use rusoto_core::reactor::{CredentialsProvider, RequestDispatcher};
use rusoto_core::region;
use rusoto_core::request::DispatchSignedRequest;
use rusoto_core::{ClientInner, RusotoFuture};

use rusoto_core::credential::{CredentialsError, ProvideAwsCredentials};
use rusoto_core::request::HttpDispatchError;

use hyper::StatusCode;
use rusoto_core::signature::SignedRequest;
use serde_json;
use serde_json::from_str;
use serde_json::Value as SerdeJsonValue;
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct BatchDeleteBuildsInput {
    /// <p>The IDs of the builds to delete.</p>
    #[serde(rename = "ids")]
    pub ids: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct BatchDeleteBuildsOutput {
    /// <p>The IDs of the builds that were successfully deleted.</p>
    #[serde(rename = "buildsDeleted")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub builds_deleted: Option<Vec<String>>,
    /// <p>Information about any builds that could not be successfully deleted.</p>
    #[serde(rename = "buildsNotDeleted")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub builds_not_deleted: Option<Vec<BuildNotDeleted>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct BatchGetBuildsInput {
    /// <p>The IDs of the builds.</p>
    #[serde(rename = "ids")]
    pub ids: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct BatchGetBuildsOutput {
    /// <p>Information about the requested builds.</p>
    #[serde(rename = "builds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub builds: Option<Vec<Build>>,
    /// <p>The IDs of builds for which information could not be found.</p>
    #[serde(rename = "buildsNotFound")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub builds_not_found: Option<Vec<String>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct BatchGetProjectsInput {
    /// <p>The names of the build projects.</p>
    #[serde(rename = "names")]
    pub names: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct BatchGetProjectsOutput {
    /// <p>Information about the requested build projects.</p>
    #[serde(rename = "projects")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub projects: Option<Vec<Project>>,
    /// <p>The names of build projects for which information could not be found.</p>
    #[serde(rename = "projectsNotFound")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub projects_not_found: Option<Vec<String>>,
}

/// <p>Information about a build.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct Build {
    /// <p>The Amazon Resource Name (ARN) of the build.</p>
    #[serde(rename = "arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p>Information about the output artifacts for the build.</p>
    #[serde(rename = "artifacts")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub artifacts: Option<BuildArtifacts>,
    /// <p>Whether the build has finished. True if completed; otherwise, false.</p>
    #[serde(rename = "buildComplete")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub build_complete: Option<bool>,
    /// <p><p>The current status of the build. Valid values include:</p> <ul> <li> <p> <code>FAILED</code>: The build failed.</p> </li> <li> <p> <code>FAULT</code>: The build faulted.</p> </li> <li> <p> <code>IN<em>PROGRESS</code>: The build is still in progress.</p> </li> <li> <p> <code>STOPPED</code>: The build stopped.</p> </li> <li> <p> <code>SUCCEEDED</code>: The build succeeded.</p> </li> <li> <p> <code>TIMED</em>OUT</code>: The build timed out.</p> </li> </ul></p>
    #[serde(rename = "buildStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub build_status: Option<String>,
    /// <p>Information about the cache for the build.</p>
    #[serde(rename = "cache")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cache: Option<ProjectCache>,
    /// <p>The current build phase.</p>
    #[serde(rename = "currentPhase")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_phase: Option<String>,
    /// <p>When the build process ended, expressed in Unix time format.</p>
    #[serde(rename = "endTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<f64>,
    /// <p>Information about the build environment for this build.</p>
    #[serde(rename = "environment")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub environment: Option<ProjectEnvironment>,
    /// <p>The unique ID for the build.</p>
    #[serde(rename = "id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// <p><p>The entity that started the build. Valid values include:</p> <ul> <li> <p>If AWS CodePipeline started the build, the pipeline&#39;s name (for example, <code>codepipeline/my-demo-pipeline</code>).</p> </li> <li> <p>If an AWS Identity and Access Management (IAM) user started the build, the user&#39;s name (for example <code>MyUserName</code>).</p> </li> <li> <p>If the Jenkins plugin for AWS CodeBuild started the build, the string <code>CodeBuild-Jenkins-Plugin</code>.</p> </li> </ul></p>
    #[serde(rename = "initiator")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub initiator: Option<String>,
    /// <p>Information about the build's logs in Amazon CloudWatch Logs.</p>
    #[serde(rename = "logs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logs: Option<LogsLocation>,
    /// <p>Describes a network interface.</p>
    #[serde(rename = "networkInterface")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_interface: Option<NetworkInterface>,
    /// <p>Information about all previous build phases that are completed and information about any current build phase that is not yet complete.</p>
    #[serde(rename = "phases")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phases: Option<Vec<BuildPhase>>,
    /// <p>The name of the AWS CodeBuild project.</p>
    #[serde(rename = "projectName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub project_name: Option<String>,
    /// <p>Information about the source code to be built.</p>
    #[serde(rename = "source")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<ProjectSource>,
    /// <p>Any version identifier for the version of the source code to be built.</p>
    #[serde(rename = "sourceVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_version: Option<String>,
    /// <p>When the build process started, expressed in Unix time format.</p>
    #[serde(rename = "startTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<f64>,
    /// <p>How long, in minutes, for AWS CodeBuild to wait before timing out this build if it does not get marked as completed.</p>
    #[serde(rename = "timeoutInMinutes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeout_in_minutes: Option<i64>,
    /// <p>If your AWS CodeBuild project accesses resources in an Amazon VPC, you provide this parameter that identifies the VPC ID and the list of security group IDs and subnet IDs. The security groups and subnets must belong to the same VPC. You must provide at least one security group and one subnet ID.</p>
    #[serde(rename = "vpcConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_config: Option<VpcConfig>,
}

/// <p>Information about build output artifacts.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct BuildArtifacts {
    /// <p>Information about the location of the build artifacts.</p>
    #[serde(rename = "location")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    /// <p><p>The MD5 hash of the build artifact.</p> <p>You can use this hash along with a checksum tool to confirm both file integrity and authenticity.</p> <note> <p>This value is available only if the build project&#39;s <code>packaging</code> value is set to <code>ZIP</code>.</p> </note></p>
    #[serde(rename = "md5sum")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub md_5sum: Option<String>,
    /// <p><p>The SHA-256 hash of the build artifact.</p> <p>You can use this hash along with a checksum tool to confirm both file integrity and authenticity.</p> <note> <p>This value is available only if the build project&#39;s <code>packaging</code> value is set to <code>ZIP</code>.</p> </note></p>
    #[serde(rename = "sha256sum")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sha_25_6sum: Option<String>,
}

/// <p>Information about a build that could not be successfully deleted.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct BuildNotDeleted {
    /// <p>The ID of the build that could not be successfully deleted.</p>
    #[serde(rename = "id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// <p>Additional information about the build that could not be successfully deleted.</p>
    #[serde(rename = "statusCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_code: Option<String>,
}

/// <p>Information about a stage for a build.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct BuildPhase {
    /// <p>Additional information about a build phase, especially to help troubleshoot a failed build.</p>
    #[serde(rename = "contexts")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contexts: Option<Vec<PhaseContext>>,
    /// <p>How long, in seconds, between the starting and ending times of the build's phase.</p>
    #[serde(rename = "durationInSeconds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration_in_seconds: Option<i64>,
    /// <p>When the build phase ended, expressed in Unix time format.</p>
    #[serde(rename = "endTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<f64>,
    /// <p><p>The current status of the build phase. Valid values include:</p> <ul> <li> <p> <code>FAILED</code>: The build phase failed.</p> </li> <li> <p> <code>FAULT</code>: The build phase faulted.</p> </li> <li> <p> <code>IN<em>PROGRESS</code>: The build phase is still in progress.</p> </li> <li> <p> <code>STOPPED</code>: The build phase stopped.</p> </li> <li> <p> <code>SUCCEEDED</code>: The build phase succeeded.</p> </li> <li> <p> <code>TIMED</em>OUT</code>: The build phase timed out.</p> </li> </ul></p>
    #[serde(rename = "phaseStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phase_status: Option<String>,
    /// <p><p>The name of the build phase. Valid values include:</p> <ul> <li> <p> <code>BUILD</code>: Core build activities typically occur in this build phase.</p> </li> <li> <p> <code>COMPLETED</code>: The build has been completed.</p> </li> <li> <p> <code>DOWNLOAD<em>SOURCE</code>: Source code is being downloaded in this build phase.</p> </li> <li> <p> <code>FINALIZING</code>: The build process is completing in this build phase.</p> </li> <li> <p> <code>INSTALL</code>: Installation activities typically occur in this build phase.</p> </li> <li> <p> <code>POST</em>BUILD</code>: Post-build activities typically occur in this build phase.</p> </li> <li> <p> <code>PRE<em>BUILD</code>: Pre-build activities typically occur in this build phase.</p> </li> <li> <p> <code>PROVISIONING</code>: The build environment is being set up.</p> </li> <li> <p> <code>SUBMITTED</code>: The build has been submitted.</p> </li> <li> <p> <code>UPLOAD</em>ARTIFACTS</code>: Build output artifacts are being uploaded to the output location.</p> </li> </ul></p>
    #[serde(rename = "phaseType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phase_type: Option<String>,
    /// <p>When the build phase started, expressed in Unix time format.</p>
    #[serde(rename = "startTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<f64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct CreateProjectInput {
    /// <p>Information about the build output artifacts for the build project.</p>
    #[serde(rename = "artifacts")]
    pub artifacts: ProjectArtifacts,
    /// <p>Set this to true to generate a publicly-accessible URL for your project's build badge.</p>
    #[serde(rename = "badgeEnabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub badge_enabled: Option<bool>,
    /// <p>Stores recently used information so that it can be quickly accessed at a later time.</p>
    #[serde(rename = "cache")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cache: Option<ProjectCache>,
    /// <p>A description that makes the build project easy to identify.</p>
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The AWS Key Management Service (AWS KMS) customer master key (CMK) to be used for encrypting the build output artifacts.</p> <p>You can specify either the CMK's Amazon Resource Name (ARN) or, if available, the CMK's alias (using the format <code>alias/<i>alias-name</i> </code>).</p>
    #[serde(rename = "encryptionKey")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encryption_key: Option<String>,
    /// <p>Information about the build environment for the build project.</p>
    #[serde(rename = "environment")]
    pub environment: ProjectEnvironment,
    /// <p>The name of the build project.</p>
    #[serde(rename = "name")]
    pub name: String,
    /// <p>The ARN of the AWS Identity and Access Management (IAM) role that enables AWS CodeBuild to interact with dependent AWS services on behalf of the AWS account.</p>
    #[serde(rename = "serviceRole")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_role: Option<String>,
    /// <p>Information about the build input source code for the build project.</p>
    #[serde(rename = "source")]
    pub source: ProjectSource,
    /// <p>A set of tags for this build project.</p> <p>These tags are available for use by AWS services that support AWS CodeBuild build project tags.</p>
    #[serde(rename = "tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
    /// <p>How long, in minutes, from 5 to 480 (8 hours), for AWS CodeBuild to wait until timing out any build that has not been marked as completed. The default is 60 minutes.</p>
    #[serde(rename = "timeoutInMinutes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeout_in_minutes: Option<i64>,
    /// <p>VpcConfig enables AWS CodeBuild to access resources in an Amazon VPC.</p>
    #[serde(rename = "vpcConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_config: Option<VpcConfig>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct CreateProjectOutput {
    /// <p>Information about the build project that was created.</p>
    #[serde(rename = "project")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub project: Option<Project>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct CreateWebhookInput {
    /// <p>A regular expression used to determine which branches in a repository are built when a webhook is triggered. If the name of a branch matches the regular expression, then it is built. If it doesn't match, then it is not. If branchFilter is empty, then all branches are built.</p>
    #[serde(rename = "branchFilter")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub branch_filter: Option<String>,
    /// <p>The name of the AWS CodeBuild project.</p>
    #[serde(rename = "projectName")]
    pub project_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct CreateWebhookOutput {
    /// <p>Information about a webhook in GitHub that connects repository events to a build project in AWS CodeBuild.</p>
    #[serde(rename = "webhook")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub webhook: Option<Webhook>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DeleteProjectInput {
    /// <p>The name of the build project.</p>
    #[serde(rename = "name")]
    pub name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct DeleteProjectOutput {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DeleteWebhookInput {
    /// <p>The name of the AWS CodeBuild project.</p>
    #[serde(rename = "projectName")]
    pub project_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct DeleteWebhookOutput {}

/// <p>Information about a Docker image that is managed by AWS CodeBuild.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct EnvironmentImage {
    /// <p>The description of the Docker image.</p>
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The name of the Docker image.</p>
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>A list of environment image versions.</p>
    #[serde(rename = "versions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub versions: Option<Vec<String>>,
}

/// <p>A set of Docker images that are related by programming language and are managed by AWS CodeBuild.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct EnvironmentLanguage {
    /// <p>The list of Docker images that are related by the specified programming language.</p>
    #[serde(rename = "images")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub images: Option<Vec<EnvironmentImage>>,
    /// <p>The programming language for the Docker images.</p>
    #[serde(rename = "language")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language: Option<String>,
}

/// <p>A set of Docker images that are related by platform and are managed by AWS CodeBuild.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct EnvironmentPlatform {
    /// <p>The list of programming languages that are available for the specified platform.</p>
    #[serde(rename = "languages")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub languages: Option<Vec<EnvironmentLanguage>>,
    /// <p>The platform's name.</p>
    #[serde(rename = "platform")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub platform: Option<String>,
}

/// <p>Information about an environment variable for a build project or a build.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EnvironmentVariable {
    /// <p>The name or key of the environment variable.</p>
    #[serde(rename = "name")]
    pub name: String,
    /// <p><p>The type of environment variable. Valid values include:</p> <ul> <li> <p> <code>PARAMETER_STORE</code>: An environment variable stored in Amazon EC2 Systems Manager Parameter Store.</p> </li> <li> <p> <code>PLAINTEXT</code>: An environment variable in plaintext format.</p> </li> </ul></p>
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    /// <p><p>The value of the environment variable.</p> <important> <p>We strongly discourage using environment variables to store sensitive values, especially AWS secret key IDs and secret access keys. Environment variables can be displayed in plain text using tools such as the AWS CodeBuild console and the AWS Command Line Interface (AWS CLI).</p> </important></p>
    #[serde(rename = "value")]
    pub value: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct InvalidateProjectCacheInput {
    /// <p>The name of the AWS CodeBuild build project that the cache will be reset for.</p>
    #[serde(rename = "projectName")]
    pub project_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct InvalidateProjectCacheOutput {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ListBuildsForProjectInput {
    /// <p>During a previous call, if there are more than 100 items in the list, only the first 100 items are returned, along with a unique string called a <i>next token</i>. To get the next batch of items in the list, call this operation again, adding the next token to the call. To get all of the items in the list, keep calling this operation with each subsequent next token that is returned, until no more next tokens are returned.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The name of the AWS CodeBuild project.</p>
    #[serde(rename = "projectName")]
    pub project_name: String,
    /// <p><p>The order to list build IDs. Valid values include:</p> <ul> <li> <p> <code>ASCENDING</code>: List the build IDs in ascending order by build ID.</p> </li> <li> <p> <code>DESCENDING</code>: List the build IDs in descending order by build ID.</p> </li> </ul></p>
    #[serde(rename = "sortOrder")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_order: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct ListBuildsForProjectOutput {
    /// <p>A list of build IDs for the specified build project, with each build ID representing a single build.</p>
    #[serde(rename = "ids")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ids: Option<Vec<String>>,
    /// <p>If there are more than 100 items in the list, only the first 100 items are returned, along with a unique string called a <i>next token</i>. To get the next batch of items in the list, call this operation again, adding the next token to the call.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ListBuildsInput {
    /// <p>During a previous call, if there are more than 100 items in the list, only the first 100 items are returned, along with a unique string called a <i>next token</i>. To get the next batch of items in the list, call this operation again, adding the next token to the call. To get all of the items in the list, keep calling this operation with each subsequent next token that is returned, until no more next tokens are returned.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p><p>The order to list build IDs. Valid values include:</p> <ul> <li> <p> <code>ASCENDING</code>: List the build IDs in ascending order by build ID.</p> </li> <li> <p> <code>DESCENDING</code>: List the build IDs in descending order by build ID.</p> </li> </ul></p>
    #[serde(rename = "sortOrder")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_order: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct ListBuildsOutput {
    /// <p>A list of build IDs, with each build ID representing a single build.</p>
    #[serde(rename = "ids")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ids: Option<Vec<String>>,
    /// <p>If there are more than 100 items in the list, only the first 100 items are returned, along with a unique string called a <i>next token</i>. To get the next batch of items in the list, call this operation again, adding the next token to the call.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ListCuratedEnvironmentImagesInput {}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct ListCuratedEnvironmentImagesOutput {
    /// <p>Information about supported platforms for Docker images that are managed by AWS CodeBuild.</p>
    #[serde(rename = "platforms")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub platforms: Option<Vec<EnvironmentPlatform>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ListProjectsInput {
    /// <p>During a previous call, if there are more than 100 items in the list, only the first 100 items are returned, along with a unique string called a <i>next token</i>. To get the next batch of items in the list, call this operation again, adding the next token to the call. To get all of the items in the list, keep calling this operation with each subsequent next token that is returned, until no more next tokens are returned.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The criterion to be used to list build project names. Valid values include:</p> <ul> <li> <p> <code>CREATED_TIME</code>: List the build project names based on when each build project was created.</p> </li> <li> <p> <code>LAST_MODIFIED_TIME</code>: List the build project names based on when information about each build project was last changed.</p> </li> <li> <p> <code>NAME</code>: List the build project names based on each build project's name.</p> </li> </ul> <p>Use <code>sortOrder</code> to specify in what order to list the build project names based on the preceding criteria.</p>
    #[serde(rename = "sortBy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_by: Option<String>,
    /// <p>The order in which to list build projects. Valid values include:</p> <ul> <li> <p> <code>ASCENDING</code>: List the build project names in ascending order.</p> </li> <li> <p> <code>DESCENDING</code>: List the build project names in descending order.</p> </li> </ul> <p>Use <code>sortBy</code> to specify the criterion to be used to list build project names.</p>
    #[serde(rename = "sortOrder")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_order: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct ListProjectsOutput {
    /// <p>If there are more than 100 items in the list, only the first 100 items are returned, along with a unique string called a <i>next token</i>. To get the next batch of items in the list, call this operation again, adding the next token to the call.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The list of build project names, with each build project name representing a single build project.</p>
    #[serde(rename = "projects")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub projects: Option<Vec<String>>,
}

/// <p>Information about build logs in Amazon CloudWatch Logs.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct LogsLocation {
    /// <p>The URL to an individual build log in Amazon CloudWatch Logs.</p>
    #[serde(rename = "deepLink")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deep_link: Option<String>,
    /// <p>The name of the Amazon CloudWatch Logs group for the build logs.</p>
    #[serde(rename = "groupName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_name: Option<String>,
    /// <p>The name of the Amazon CloudWatch Logs stream for the build logs.</p>
    #[serde(rename = "streamName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream_name: Option<String>,
}

/// <p>Describes a network interface.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct NetworkInterface {
    /// <p>The ID of the network interface.</p>
    #[serde(rename = "networkInterfaceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_interface_id: Option<String>,
    /// <p>The ID of the subnet.</p>
    #[serde(rename = "subnetId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subnet_id: Option<String>,
}

/// <p>Additional information about a build phase that has an error. You can use this information to help troubleshoot a failed build.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct PhaseContext {
    /// <p>An explanation of the build phase's context. This explanation might include a command ID and an exit code.</p>
    #[serde(rename = "message")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    /// <p>The status code for the context of the build phase.</p>
    #[serde(rename = "statusCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_code: Option<String>,
}

/// <p>Information about a build project.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct Project {
    /// <p>The Amazon Resource Name (ARN) of the build project.</p>
    #[serde(rename = "arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p>Information about the build output artifacts for the build project.</p>
    #[serde(rename = "artifacts")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub artifacts: Option<ProjectArtifacts>,
    /// <p>Information about the build badge for the build project.</p>
    #[serde(rename = "badge")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub badge: Option<ProjectBadge>,
    /// <p>Information about the cache for the build project.</p>
    #[serde(rename = "cache")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cache: Option<ProjectCache>,
    /// <p>When the build project was created, expressed in Unix time format.</p>
    #[serde(rename = "created")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created: Option<f64>,
    /// <p>A description that makes the build project easy to identify.</p>
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The AWS Key Management Service (AWS KMS) customer master key (CMK) to be used for encrypting the build output artifacts.</p> <p>This is expressed either as the CMK's Amazon Resource Name (ARN) or, if specified, the CMK's alias (using the format <code>alias/<i>alias-name</i> </code>).</p>
    #[serde(rename = "encryptionKey")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encryption_key: Option<String>,
    /// <p>Information about the build environment for this build project.</p>
    #[serde(rename = "environment")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub environment: Option<ProjectEnvironment>,
    /// <p>When the build project's settings were last modified, expressed in Unix time format.</p>
    #[serde(rename = "lastModified")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified: Option<f64>,
    /// <p>The name of the build project.</p>
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The ARN of the AWS Identity and Access Management (IAM) role that enables AWS CodeBuild to interact with dependent AWS services on behalf of the AWS account.</p>
    #[serde(rename = "serviceRole")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_role: Option<String>,
    /// <p>Information about the build input source code for this build project.</p>
    #[serde(rename = "source")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<ProjectSource>,
    /// <p>The tags for this build project.</p> <p>These tags are available for use by AWS services that support AWS CodeBuild build project tags.</p>
    #[serde(rename = "tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
    /// <p>How long, in minutes, from 5 to 480 (8 hours), for AWS CodeBuild to wait before timing out any related build that did not get marked as completed. The default is 60 minutes.</p>
    #[serde(rename = "timeoutInMinutes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeout_in_minutes: Option<i64>,
    /// <p>Information about the VPC configuration that AWS CodeBuild will access.</p>
    #[serde(rename = "vpcConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_config: Option<VpcConfig>,
    /// <p>Information about a webhook in GitHub that connects repository events to a build project in AWS CodeBuild.</p>
    #[serde(rename = "webhook")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub webhook: Option<Webhook>,
}

/// <p>Information about the build output artifacts for the build project.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ProjectArtifacts {
    /// <p><p>Information about the build output artifact location, as follows:</p> <ul> <li> <p>If <code>type</code> is set to <code>CODEPIPELINE</code>, then AWS CodePipeline will ignore this value if specified. This is because AWS CodePipeline manages its build output locations instead of AWS CodeBuild.</p> </li> <li> <p>If <code>type</code> is set to <code>NO_ARTIFACTS</code>, then this value will be ignored if specified, because no build output will be produced.</p> </li> <li> <p>If <code>type</code> is set to <code>S3</code>, this is the name of the output bucket.</p> </li> </ul></p>
    #[serde(rename = "location")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    /// <p>Along with <code>path</code> and <code>namespaceType</code>, the pattern that AWS CodeBuild will use to name and store the output artifact, as follows:</p> <ul> <li> <p>If <code>type</code> is set to <code>CODEPIPELINE</code>, then AWS CodePipeline will ignore this value if specified. This is because AWS CodePipeline manages its build output names instead of AWS CodeBuild.</p> </li> <li> <p>If <code>type</code> is set to <code>NO_ARTIFACTS</code>, then this value will be ignored if specified, because no build output will be produced.</p> </li> <li> <p>If <code>type</code> is set to <code>S3</code>, this is the name of the output artifact object.</p> </li> </ul> <p>For example, if <code>path</code> is set to <code>MyArtifacts</code>, <code>namespaceType</code> is set to <code>BUILD_ID</code>, and <code>name</code> is set to <code>MyArtifact.zip</code>, then the output artifact would be stored in <code>MyArtifacts/<i>build-ID</i>/MyArtifact.zip</code>.</p>
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>Along with <code>path</code> and <code>name</code>, the pattern that AWS CodeBuild will use to determine the name and location to store the output artifact, as follows:</p> <ul> <li> <p>If <code>type</code> is set to <code>CODEPIPELINE</code>, then AWS CodePipeline will ignore this value if specified. This is because AWS CodePipeline manages its build output names instead of AWS CodeBuild.</p> </li> <li> <p>If <code>type</code> is set to <code>NO_ARTIFACTS</code>, then this value will be ignored if specified, because no build output will be produced.</p> </li> <li> <p>If <code>type</code> is set to <code>S3</code>, then valid values include:</p> <ul> <li> <p> <code>BUILD_ID</code>: Include the build ID in the location of the build output artifact.</p> </li> <li> <p> <code>NONE</code>: Do not include the build ID. This is the default if <code>namespaceType</code> is not specified.</p> </li> </ul> </li> </ul> <p>For example, if <code>path</code> is set to <code>MyArtifacts</code>, <code>namespaceType</code> is set to <code>BUILD_ID</code>, and <code>name</code> is set to <code>MyArtifact.zip</code>, then the output artifact would be stored in <code>MyArtifacts/<i>build-ID</i>/MyArtifact.zip</code>.</p>
    #[serde(rename = "namespaceType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub namespace_type: Option<String>,
    /// <p><p>The type of build output artifact to create, as follows:</p> <ul> <li> <p>If <code>type</code> is set to <code>CODEPIPELINE</code>, then AWS CodePipeline will ignore this value if specified. This is because AWS CodePipeline manages its build output artifacts instead of AWS CodeBuild.</p> </li> <li> <p>If <code>type</code> is set to <code>NO_ARTIFACTS</code>, then this value will be ignored if specified, because no build output will be produced.</p> </li> <li> <p>If <code>type</code> is set to <code>S3</code>, valid values include:</p> <ul> <li> <p> <code>NONE</code>: AWS CodeBuild will create in the output bucket a folder containing the build output. This is the default if <code>packaging</code> is not specified.</p> </li> <li> <p> <code>ZIP</code>: AWS CodeBuild will create in the output bucket a ZIP file containing the build output.</p> </li> </ul> </li> </ul></p>
    #[serde(rename = "packaging")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub packaging: Option<String>,
    /// <p>Along with <code>namespaceType</code> and <code>name</code>, the pattern that AWS CodeBuild will use to name and store the output artifact, as follows:</p> <ul> <li> <p>If <code>type</code> is set to <code>CODEPIPELINE</code>, then AWS CodePipeline will ignore this value if specified. This is because AWS CodePipeline manages its build output names instead of AWS CodeBuild.</p> </li> <li> <p>If <code>type</code> is set to <code>NO_ARTIFACTS</code>, then this value will be ignored if specified, because no build output will be produced.</p> </li> <li> <p>If <code>type</code> is set to <code>S3</code>, this is the path to the output artifact. If <code>path</code> is not specified, then <code>path</code> will not be used.</p> </li> </ul> <p>For example, if <code>path</code> is set to <code>MyArtifacts</code>, <code>namespaceType</code> is set to <code>NONE</code>, and <code>name</code> is set to <code>MyArtifact.zip</code>, then the output artifact would be stored in the output bucket at <code>MyArtifacts/MyArtifact.zip</code>.</p>
    #[serde(rename = "path")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    /// <p><p>The type of build output artifact. Valid values include:</p> <ul> <li> <p> <code>CODEPIPELINE</code>: The build project will have build output generated through AWS CodePipeline.</p> </li> <li> <p> <code>NO_ARTIFACTS</code>: The build project will not produce any build output.</p> </li> <li> <p> <code>S3</code>: The build project will store build output in Amazon Simple Storage Service (Amazon S3).</p> </li> </ul></p>
    #[serde(rename = "type")]
    pub type_: String,
}

/// <p>Information about the build badge for the build project.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct ProjectBadge {
    /// <p>Set this to true to generate a publicly-accessible URL for your project's build badge.</p>
    #[serde(rename = "badgeEnabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub badge_enabled: Option<bool>,
    /// <p>The publicly-accessible URL through which you can access the build badge for your project. </p>
    #[serde(rename = "badgeRequestUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub badge_request_url: Option<String>,
}

/// <p>Information about the cache for the build project.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ProjectCache {
    /// <p><p>Information about the cache location, as follows: </p> <ul> <li> <p> <code>NO_CACHE</code>: This value will be ignored.</p> </li> <li> <p> <code>S3</code>: This is the S3 bucket name/prefix.</p> </li> </ul></p>
    #[serde(rename = "location")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    /// <p><p>The type of cache used by the build project. Valid values include:</p> <ul> <li> <p> <code>NO_CACHE</code>: The build project will not use any cache.</p> </li> <li> <p> <code>S3</code>: The build project will read and write from/to S3.</p> </li> </ul></p>
    #[serde(rename = "type")]
    pub type_: String,
}

/// <p>Information about the build environment of the build project.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ProjectEnvironment {
    /// <p>The certificate to use with this build project.</p>
    #[serde(rename = "certificate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate: Option<String>,
    /// <p><p>Information about the compute resources the build project will use. Available values include:</p> <ul> <li> <p> <code>BUILD<em>GENERAL1</em>SMALL</code>: Use up to 3 GB memory and 2 vCPUs for builds.</p> </li> <li> <p> <code>BUILD<em>GENERAL1</em>MEDIUM</code>: Use up to 7 GB memory and 4 vCPUs for builds.</p> </li> <li> <p> <code>BUILD<em>GENERAL1</em>LARGE</code>: Use up to 15 GB memory and 8 vCPUs for builds.</p> </li> </ul></p>
    #[serde(rename = "computeType")]
    pub compute_type: String,
    /// <p>A set of environment variables to make available to builds for this build project.</p>
    #[serde(rename = "environmentVariables")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub environment_variables: Option<Vec<EnvironmentVariable>>,
    /// <p>The ID of the Docker image to use for this build project.</p>
    #[serde(rename = "image")]
    pub image: String,
    /// <p>Enables running the Docker daemon inside a Docker container. Set to true only if the build project is be used to build Docker images, and the specified build environment image is not provided by AWS CodeBuild with Docker support. Otherwise, all associated builds that attempt to interact with the Docker daemon will fail. Note that you must also start the Docker daemon so that builds can interact with it. One way to do this is to initialize the Docker daemon during the install phase of your build spec by running the following build commands. (Do not run the following build commands if the specified build environment image is provided by AWS CodeBuild with Docker support.)</p> <p> <code>- nohup /usr/local/bin/dockerd --host=unix:///var/run/docker.sock --host=tcp://0.0.0.0:2375 --storage-driver=overlay&amp; - timeout -t 15 sh -c "until docker info; do echo .; sleep 1; done"</code> </p>
    #[serde(rename = "privilegedMode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub privileged_mode: Option<bool>,
    /// <p>The type of build environment to use for related builds.</p>
    #[serde(rename = "type")]
    pub type_: String,
}

/// <p>Information about the build input source code for the build project.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ProjectSource {
    /// <p>Information about the authorization settings for AWS CodeBuild to access the source code to be built.</p> <p>This information is for the AWS CodeBuild console's use only. Your code should not get or set this information directly (unless the build project's source <code>type</code> value is <code>BITBUCKET</code> or <code>GITHUB</code>).</p>
    #[serde(rename = "auth")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auth: Option<SourceAuth>,
    /// <p>The build spec declaration to use for the builds in this build project.</p> <p>If this value is not specified, a build spec must be included along with the source code to be built.</p>
    #[serde(rename = "buildspec")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub buildspec: Option<String>,
    /// <p>Information about the git clone depth for the build project.</p>
    #[serde(rename = "gitCloneDepth")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub git_clone_depth: Option<i64>,
    /// <p>Enable this flag to ignore SSL warnings while connecting to the project source code.</p>
    #[serde(rename = "insecureSsl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub insecure_ssl: Option<bool>,
    /// <p><p>Information about the location of the source code to be built. Valid values include:</p> <ul> <li> <p>For source code settings that are specified in the source action of a pipeline in AWS CodePipeline, <code>location</code> should not be specified. If it is specified, AWS CodePipeline will ignore it. This is because AWS CodePipeline uses the settings in a pipeline&#39;s source action instead of this value.</p> </li> <li> <p>For source code in an AWS CodeCommit repository, the HTTPS clone URL to the repository that contains the source code and the build spec (for example, <code>https://git-codecommit.<i>region-ID</i>.amazonaws.com/v1/repos/<i>repo-name</i> </code>).</p> </li> <li> <p>For source code in an Amazon Simple Storage Service (Amazon S3) input bucket, the path to the ZIP file that contains the source code (for example, <code> <i>bucket-name</i>/<i>path</i>/<i>to</i>/<i>object-name</i>.zip</code>)</p> </li> <li> <p>For source code in a GitHub repository, the HTTPS clone URL to the repository that contains the source and the build spec. Also, you must connect your AWS account to your GitHub account. To do this, use the AWS CodeBuild console to begin creating a build project. When you use the console to connect (or reconnect) with GitHub, on the GitHub <b>Authorize application</b> page that displays, for <b>Organization access</b>, choose <b>Request access</b> next to each repository you want to allow AWS CodeBuild to have access to. Then choose <b>Authorize application</b>. (After you have connected to your GitHub account, you do not need to finish creating the build project, and you may then leave the AWS CodeBuild console.) To instruct AWS CodeBuild to then use this connection, in the <code>source</code> object, set the <code>auth</code> object&#39;s <code>type</code> value to <code>OAUTH</code>.</p> </li> <li> <p>For source code in a Bitbucket repository, the HTTPS clone URL to the repository that contains the source and the build spec. Also, you must connect your AWS account to your Bitbucket account. To do this, use the AWS CodeBuild console to begin creating a build project. When you use the console to connect (or reconnect) with Bitbucket, on the Bitbucket <b>Confirm access to your account</b> page that displays, choose <b>Grant access</b>. (After you have connected to your Bitbucket account, you do not need to finish creating the build project, and you may then leave the AWS CodeBuild console.) To instruct AWS CodeBuild to then use this connection, in the <code>source</code> object, set the <code>auth</code> object&#39;s <code>type</code> value to <code>OAUTH</code>.</p> </li> </ul></p>
    #[serde(rename = "location")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    /// <p><p>The type of repository that contains the source code to be built. Valid values include:</p> <ul> <li> <p> <code>BITBUCKET</code>: The source code is in a Bitbucket repository.</p> </li> <li> <p> <code>CODECOMMIT</code>: The source code is in an AWS CodeCommit repository.</p> </li> <li> <p> <code>CODEPIPELINE</code>: The source code settings are specified in the source action of a pipeline in AWS CodePipeline.</p> </li> <li> <p> <code>GITHUB</code>: The source code is in a GitHub repository.</p> </li> <li> <p> <code>S3</code>: The source code is in an Amazon Simple Storage Service (Amazon S3) input bucket.</p> </li> </ul></p>
    #[serde(rename = "type")]
    pub type_: String,
}

/// <p>Information about the authorization settings for AWS CodeBuild to access the source code to be built.</p> <p>This information is for the AWS CodeBuild console's use only. Your code should not get or set this information directly (unless the build project's source <code>type</code> value is <code>BITBUCKET</code> or <code>GITHUB</code>).</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SourceAuth {
    /// <p>The resource value that applies to the specified authorization type.</p>
    #[serde(rename = "resource")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource: Option<String>,
    /// <p>The authorization type to use. The only valid value is <code>OAUTH</code>, which represents the OAuth authorization type.</p>
    #[serde(rename = "type")]
    pub type_: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct StartBuildInput {
    /// <p>Build output artifact settings that override, for this build only, the latest ones already defined in the build project.</p>
    #[serde(rename = "artifactsOverride")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub artifacts_override: Option<ProjectArtifacts>,
    /// <p>A build spec declaration that overrides, for this build only, the latest one already defined in the build project.</p>
    #[serde(rename = "buildspecOverride")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub buildspec_override: Option<String>,
    /// <p>A set of environment variables that overrides, for this build only, the latest ones already defined in the build project.</p>
    #[serde(rename = "environmentVariablesOverride")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub environment_variables_override: Option<Vec<EnvironmentVariable>>,
    /// <p>The user-defined depth of history, with a minimum value of 0, that overrides, for this build only, any previous depth of history defined in the build project.</p>
    #[serde(rename = "gitCloneDepthOverride")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub git_clone_depth_override: Option<i64>,
    /// <p>The name of the AWS CodeBuild build project to start running a build.</p>
    #[serde(rename = "projectName")]
    pub project_name: String,
    /// <p><p>A version of the build input to be built, for this build only. If not specified, the latest version will be used. If specified, must be one of:</p> <ul> <li> <p>For AWS CodeCommit: the commit ID to use.</p> </li> <li> <p>For GitHub: the commit ID, pull request ID, branch name, or tag name that corresponds to the version of the source code you want to build. If a pull request ID is specified, it must use the format <code>pr/pull-request-ID</code> (for example <code>pr/25</code>). If a branch name is specified, the branch&#39;s HEAD commit ID will be used. If not specified, the default branch&#39;s HEAD commit ID will be used.</p> </li> <li> <p>For Bitbucket: the commit ID, branch name, or tag name that corresponds to the version of the source code you want to build. If a branch name is specified, the branch&#39;s HEAD commit ID will be used. If not specified, the default branch&#39;s HEAD commit ID will be used.</p> </li> <li> <p>For Amazon Simple Storage Service (Amazon S3): the version ID of the object representing the build input ZIP file to use.</p> </li> </ul></p>
    #[serde(rename = "sourceVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_version: Option<String>,
    /// <p>The number of build timeout minutes, from 5 to 480 (8 hours), that overrides, for this build only, the latest setting already defined in the build project.</p>
    #[serde(rename = "timeoutInMinutesOverride")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeout_in_minutes_override: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct StartBuildOutput {
    /// <p>Information about the build to be run.</p>
    #[serde(rename = "build")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub build: Option<Build>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct StopBuildInput {
    /// <p>The ID of the build.</p>
    #[serde(rename = "id")]
    pub id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct StopBuildOutput {
    /// <p>Information about the build.</p>
    #[serde(rename = "build")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub build: Option<Build>,
}

/// <p>A tag, consisting of a key and a value.</p> <p>This tag is available for use by AWS services that support tags in AWS CodeBuild.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Tag {
    /// <p>The tag's key.</p>
    #[serde(rename = "key")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    /// <p>The tag's value.</p>
    #[serde(rename = "value")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct UpdateProjectInput {
    /// <p>Information to be changed about the build output artifacts for the build project.</p>
    #[serde(rename = "artifacts")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub artifacts: Option<ProjectArtifacts>,
    /// <p>Set this to true to generate a publicly-accessible URL for your project's build badge.</p>
    #[serde(rename = "badgeEnabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub badge_enabled: Option<bool>,
    /// <p>Stores recently used information so that it can be quickly accessed at a later time.</p>
    #[serde(rename = "cache")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cache: Option<ProjectCache>,
    /// <p>A new or replacement description of the build project.</p>
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The replacement AWS Key Management Service (AWS KMS) customer master key (CMK) to be used for encrypting the build output artifacts.</p> <p>You can specify either the CMK's Amazon Resource Name (ARN) or, if available, the CMK's alias (using the format <code>alias/<i>alias-name</i> </code>).</p>
    #[serde(rename = "encryptionKey")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encryption_key: Option<String>,
    /// <p>Information to be changed about the build environment for the build project.</p>
    #[serde(rename = "environment")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub environment: Option<ProjectEnvironment>,
    /// <p><p>The name of the build project.</p> <note> <p>You cannot change a build project&#39;s name.</p> </note></p>
    #[serde(rename = "name")]
    pub name: String,
    /// <p>The replacement ARN of the AWS Identity and Access Management (IAM) role that enables AWS CodeBuild to interact with dependent AWS services on behalf of the AWS account.</p>
    #[serde(rename = "serviceRole")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_role: Option<String>,
    /// <p>Information to be changed about the build input source code for the build project.</p>
    #[serde(rename = "source")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<ProjectSource>,
    /// <p>The replacement set of tags for this build project.</p> <p>These tags are available for use by AWS services that support AWS CodeBuild build project tags.</p>
    #[serde(rename = "tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
    /// <p>The replacement value in minutes, from 5 to 480 (8 hours), for AWS CodeBuild to wait before timing out any related build that did not get marked as completed.</p>
    #[serde(rename = "timeoutInMinutes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeout_in_minutes: Option<i64>,
    /// <p>VpcConfig enables AWS CodeBuild to access resources in an Amazon VPC.</p>
    #[serde(rename = "vpcConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_config: Option<VpcConfig>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct UpdateProjectOutput {
    /// <p>Information about the build project that was changed.</p>
    #[serde(rename = "project")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub project: Option<Project>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct UpdateWebhookInput {
    /// <p>A regular expression used to determine which branches in a repository are built when a webhook is triggered. If the name of a branch matches the regular expression, then it is built. If it doesn't match, then it is not. If branchFilter is empty, then all branches are built.</p>
    #[serde(rename = "branchFilter")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub branch_filter: Option<String>,
    /// <p>The name of the AWS CodeBuild project.</p>
    #[serde(rename = "projectName")]
    pub project_name: String,
    /// <p> A boolean value that specifies whether the associated repository's secret token should be updated. </p>
    #[serde(rename = "rotateSecret")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rotate_secret: Option<bool>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct UpdateWebhookOutput {
    /// <p> Information about a repository's webhook that is associated with a project in AWS CodeBuild. </p>
    #[serde(rename = "webhook")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub webhook: Option<Webhook>,
}

/// <p>Information about the VPC configuration that AWS CodeBuild will access.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct VpcConfig {
    /// <p>A list of one or more security groups IDs in your Amazon VPC.</p>
    #[serde(rename = "securityGroupIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_group_ids: Option<Vec<String>>,
    /// <p>A list of one or more subnet IDs in your Amazon VPC.</p>
    #[serde(rename = "subnets")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subnets: Option<Vec<String>>,
    /// <p>The ID of the Amazon VPC.</p>
    #[serde(rename = "vpcId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_id: Option<String>,
}

/// <p>Information about a webhook in GitHub that connects repository events to a build project in AWS CodeBuild.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct Webhook {
    /// <p>A regular expression used to determine which branches in a repository are built when a webhook is triggered. If the name of a branch matches the regular expression, then it is built. If it doesn't match, then it is not. If branchFilter is empty, then all branches are built.</p>
    #[serde(rename = "branchFilter")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub branch_filter: Option<String>,
    /// <p> A timestamp indicating the last time a repository's secret token was modified. </p>
    #[serde(rename = "lastModifiedSecret")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_secret: Option<f64>,
    /// <p> The CodeBuild endpoint where webhook events are sent.</p>
    #[serde(rename = "payloadUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payload_url: Option<String>,
    /// <p> The secret token of the associated repository. </p>
    #[serde(rename = "secret")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secret: Option<String>,
    /// <p>The URL to the webhook.</p>
    #[serde(rename = "url")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

/// Errors returned by BatchDeleteBuilds
#[derive(Debug, PartialEq)]
pub enum BatchDeleteBuildsError {
    /// <p>The input value that was provided is not valid.</p>
    InvalidInput(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl BatchDeleteBuildsError {
    pub fn from_body(body: &str) -> BatchDeleteBuildsError {
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
                    "InvalidInputException" => {
                        BatchDeleteBuildsError::InvalidInput(String::from(error_message))
                    }
                    "ValidationException" => {
                        BatchDeleteBuildsError::Validation(error_message.to_string())
                    }
                    _ => BatchDeleteBuildsError::Unknown(String::from(body)),
                }
            }
            Err(_) => BatchDeleteBuildsError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for BatchDeleteBuildsError {
    fn from(err: serde_json::error::Error) -> BatchDeleteBuildsError {
        BatchDeleteBuildsError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for BatchDeleteBuildsError {
    fn from(err: CredentialsError) -> BatchDeleteBuildsError {
        BatchDeleteBuildsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for BatchDeleteBuildsError {
    fn from(err: HttpDispatchError) -> BatchDeleteBuildsError {
        BatchDeleteBuildsError::HttpDispatch(err)
    }
}
impl From<io::Error> for BatchDeleteBuildsError {
    fn from(err: io::Error) -> BatchDeleteBuildsError {
        BatchDeleteBuildsError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for BatchDeleteBuildsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for BatchDeleteBuildsError {
    fn description(&self) -> &str {
        match *self {
            BatchDeleteBuildsError::InvalidInput(ref cause) => cause,
            BatchDeleteBuildsError::Validation(ref cause) => cause,
            BatchDeleteBuildsError::Credentials(ref err) => err.description(),
            BatchDeleteBuildsError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            BatchDeleteBuildsError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by BatchGetBuilds
#[derive(Debug, PartialEq)]
pub enum BatchGetBuildsError {
    /// <p>The input value that was provided is not valid.</p>
    InvalidInput(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl BatchGetBuildsError {
    pub fn from_body(body: &str) -> BatchGetBuildsError {
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
                    "InvalidInputException" => {
                        BatchGetBuildsError::InvalidInput(String::from(error_message))
                    }
                    "ValidationException" => {
                        BatchGetBuildsError::Validation(error_message.to_string())
                    }
                    _ => BatchGetBuildsError::Unknown(String::from(body)),
                }
            }
            Err(_) => BatchGetBuildsError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for BatchGetBuildsError {
    fn from(err: serde_json::error::Error) -> BatchGetBuildsError {
        BatchGetBuildsError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for BatchGetBuildsError {
    fn from(err: CredentialsError) -> BatchGetBuildsError {
        BatchGetBuildsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for BatchGetBuildsError {
    fn from(err: HttpDispatchError) -> BatchGetBuildsError {
        BatchGetBuildsError::HttpDispatch(err)
    }
}
impl From<io::Error> for BatchGetBuildsError {
    fn from(err: io::Error) -> BatchGetBuildsError {
        BatchGetBuildsError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for BatchGetBuildsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for BatchGetBuildsError {
    fn description(&self) -> &str {
        match *self {
            BatchGetBuildsError::InvalidInput(ref cause) => cause,
            BatchGetBuildsError::Validation(ref cause) => cause,
            BatchGetBuildsError::Credentials(ref err) => err.description(),
            BatchGetBuildsError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            BatchGetBuildsError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by BatchGetProjects
#[derive(Debug, PartialEq)]
pub enum BatchGetProjectsError {
    /// <p>The input value that was provided is not valid.</p>
    InvalidInput(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl BatchGetProjectsError {
    pub fn from_body(body: &str) -> BatchGetProjectsError {
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
                    "InvalidInputException" => {
                        BatchGetProjectsError::InvalidInput(String::from(error_message))
                    }
                    "ValidationException" => {
                        BatchGetProjectsError::Validation(error_message.to_string())
                    }
                    _ => BatchGetProjectsError::Unknown(String::from(body)),
                }
            }
            Err(_) => BatchGetProjectsError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for BatchGetProjectsError {
    fn from(err: serde_json::error::Error) -> BatchGetProjectsError {
        BatchGetProjectsError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for BatchGetProjectsError {
    fn from(err: CredentialsError) -> BatchGetProjectsError {
        BatchGetProjectsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for BatchGetProjectsError {
    fn from(err: HttpDispatchError) -> BatchGetProjectsError {
        BatchGetProjectsError::HttpDispatch(err)
    }
}
impl From<io::Error> for BatchGetProjectsError {
    fn from(err: io::Error) -> BatchGetProjectsError {
        BatchGetProjectsError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for BatchGetProjectsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for BatchGetProjectsError {
    fn description(&self) -> &str {
        match *self {
            BatchGetProjectsError::InvalidInput(ref cause) => cause,
            BatchGetProjectsError::Validation(ref cause) => cause,
            BatchGetProjectsError::Credentials(ref err) => err.description(),
            BatchGetProjectsError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            BatchGetProjectsError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by CreateProject
#[derive(Debug, PartialEq)]
pub enum CreateProjectError {
    /// <p>An AWS service limit was exceeded for the calling AWS account.</p>
    AccountLimitExceeded(String),
    /// <p>The input value that was provided is not valid.</p>
    InvalidInput(String),
    /// <p>The specified AWS resource cannot be created, because an AWS resource with the same settings already exists.</p>
    ResourceAlreadyExists(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl CreateProjectError {
    pub fn from_body(body: &str) -> CreateProjectError {
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
                    "AccountLimitExceededException" => {
                        CreateProjectError::AccountLimitExceeded(String::from(error_message))
                    }
                    "InvalidInputException" => {
                        CreateProjectError::InvalidInput(String::from(error_message))
                    }
                    "ResourceAlreadyExistsException" => {
                        CreateProjectError::ResourceAlreadyExists(String::from(error_message))
                    }
                    "ValidationException" => {
                        CreateProjectError::Validation(error_message.to_string())
                    }
                    _ => CreateProjectError::Unknown(String::from(body)),
                }
            }
            Err(_) => CreateProjectError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for CreateProjectError {
    fn from(err: serde_json::error::Error) -> CreateProjectError {
        CreateProjectError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for CreateProjectError {
    fn from(err: CredentialsError) -> CreateProjectError {
        CreateProjectError::Credentials(err)
    }
}
impl From<HttpDispatchError> for CreateProjectError {
    fn from(err: HttpDispatchError) -> CreateProjectError {
        CreateProjectError::HttpDispatch(err)
    }
}
impl From<io::Error> for CreateProjectError {
    fn from(err: io::Error) -> CreateProjectError {
        CreateProjectError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for CreateProjectError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateProjectError {
    fn description(&self) -> &str {
        match *self {
            CreateProjectError::AccountLimitExceeded(ref cause) => cause,
            CreateProjectError::InvalidInput(ref cause) => cause,
            CreateProjectError::ResourceAlreadyExists(ref cause) => cause,
            CreateProjectError::Validation(ref cause) => cause,
            CreateProjectError::Credentials(ref err) => err.description(),
            CreateProjectError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            CreateProjectError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by CreateWebhook
#[derive(Debug, PartialEq)]
pub enum CreateWebhookError {
    /// <p>The input value that was provided is not valid.</p>
    InvalidInput(String),
    /// <p>There was a problem with the underlying OAuth provider.</p>
    OAuthProvider(String),
    /// <p>The specified AWS resource cannot be created, because an AWS resource with the same settings already exists.</p>
    ResourceAlreadyExists(String),
    /// <p>The specified AWS resource cannot be found.</p>
    ResourceNotFound(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl CreateWebhookError {
    pub fn from_body(body: &str) -> CreateWebhookError {
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
                    "InvalidInputException" => {
                        CreateWebhookError::InvalidInput(String::from(error_message))
                    }
                    "OAuthProviderException" => {
                        CreateWebhookError::OAuthProvider(String::from(error_message))
                    }
                    "ResourceAlreadyExistsException" => {
                        CreateWebhookError::ResourceAlreadyExists(String::from(error_message))
                    }
                    "ResourceNotFoundException" => {
                        CreateWebhookError::ResourceNotFound(String::from(error_message))
                    }
                    "ValidationException" => {
                        CreateWebhookError::Validation(error_message.to_string())
                    }
                    _ => CreateWebhookError::Unknown(String::from(body)),
                }
            }
            Err(_) => CreateWebhookError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for CreateWebhookError {
    fn from(err: serde_json::error::Error) -> CreateWebhookError {
        CreateWebhookError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for CreateWebhookError {
    fn from(err: CredentialsError) -> CreateWebhookError {
        CreateWebhookError::Credentials(err)
    }
}
impl From<HttpDispatchError> for CreateWebhookError {
    fn from(err: HttpDispatchError) -> CreateWebhookError {
        CreateWebhookError::HttpDispatch(err)
    }
}
impl From<io::Error> for CreateWebhookError {
    fn from(err: io::Error) -> CreateWebhookError {
        CreateWebhookError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for CreateWebhookError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateWebhookError {
    fn description(&self) -> &str {
        match *self {
            CreateWebhookError::InvalidInput(ref cause) => cause,
            CreateWebhookError::OAuthProvider(ref cause) => cause,
            CreateWebhookError::ResourceAlreadyExists(ref cause) => cause,
            CreateWebhookError::ResourceNotFound(ref cause) => cause,
            CreateWebhookError::Validation(ref cause) => cause,
            CreateWebhookError::Credentials(ref err) => err.description(),
            CreateWebhookError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            CreateWebhookError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DeleteProject
#[derive(Debug, PartialEq)]
pub enum DeleteProjectError {
    /// <p>The input value that was provided is not valid.</p>
    InvalidInput(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl DeleteProjectError {
    pub fn from_body(body: &str) -> DeleteProjectError {
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
                    "InvalidInputException" => {
                        DeleteProjectError::InvalidInput(String::from(error_message))
                    }
                    "ValidationException" => {
                        DeleteProjectError::Validation(error_message.to_string())
                    }
                    _ => DeleteProjectError::Unknown(String::from(body)),
                }
            }
            Err(_) => DeleteProjectError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DeleteProjectError {
    fn from(err: serde_json::error::Error) -> DeleteProjectError {
        DeleteProjectError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DeleteProjectError {
    fn from(err: CredentialsError) -> DeleteProjectError {
        DeleteProjectError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DeleteProjectError {
    fn from(err: HttpDispatchError) -> DeleteProjectError {
        DeleteProjectError::HttpDispatch(err)
    }
}
impl From<io::Error> for DeleteProjectError {
    fn from(err: io::Error) -> DeleteProjectError {
        DeleteProjectError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DeleteProjectError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteProjectError {
    fn description(&self) -> &str {
        match *self {
            DeleteProjectError::InvalidInput(ref cause) => cause,
            DeleteProjectError::Validation(ref cause) => cause,
            DeleteProjectError::Credentials(ref err) => err.description(),
            DeleteProjectError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            DeleteProjectError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DeleteWebhook
#[derive(Debug, PartialEq)]
pub enum DeleteWebhookError {
    /// <p>The input value that was provided is not valid.</p>
    InvalidInput(String),
    /// <p>There was a problem with the underlying OAuth provider.</p>
    OAuthProvider(String),
    /// <p>The specified AWS resource cannot be found.</p>
    ResourceNotFound(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl DeleteWebhookError {
    pub fn from_body(body: &str) -> DeleteWebhookError {
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
                    "InvalidInputException" => {
                        DeleteWebhookError::InvalidInput(String::from(error_message))
                    }
                    "OAuthProviderException" => {
                        DeleteWebhookError::OAuthProvider(String::from(error_message))
                    }
                    "ResourceNotFoundException" => {
                        DeleteWebhookError::ResourceNotFound(String::from(error_message))
                    }
                    "ValidationException" => {
                        DeleteWebhookError::Validation(error_message.to_string())
                    }
                    _ => DeleteWebhookError::Unknown(String::from(body)),
                }
            }
            Err(_) => DeleteWebhookError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DeleteWebhookError {
    fn from(err: serde_json::error::Error) -> DeleteWebhookError {
        DeleteWebhookError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DeleteWebhookError {
    fn from(err: CredentialsError) -> DeleteWebhookError {
        DeleteWebhookError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DeleteWebhookError {
    fn from(err: HttpDispatchError) -> DeleteWebhookError {
        DeleteWebhookError::HttpDispatch(err)
    }
}
impl From<io::Error> for DeleteWebhookError {
    fn from(err: io::Error) -> DeleteWebhookError {
        DeleteWebhookError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DeleteWebhookError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteWebhookError {
    fn description(&self) -> &str {
        match *self {
            DeleteWebhookError::InvalidInput(ref cause) => cause,
            DeleteWebhookError::OAuthProvider(ref cause) => cause,
            DeleteWebhookError::ResourceNotFound(ref cause) => cause,
            DeleteWebhookError::Validation(ref cause) => cause,
            DeleteWebhookError::Credentials(ref err) => err.description(),
            DeleteWebhookError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            DeleteWebhookError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by InvalidateProjectCache
#[derive(Debug, PartialEq)]
pub enum InvalidateProjectCacheError {
    /// <p>The input value that was provided is not valid.</p>
    InvalidInput(String),
    /// <p>The specified AWS resource cannot be found.</p>
    ResourceNotFound(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl InvalidateProjectCacheError {
    pub fn from_body(body: &str) -> InvalidateProjectCacheError {
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
                    "InvalidInputException" => {
                        InvalidateProjectCacheError::InvalidInput(String::from(error_message))
                    }
                    "ResourceNotFoundException" => {
                        InvalidateProjectCacheError::ResourceNotFound(String::from(error_message))
                    }
                    "ValidationException" => {
                        InvalidateProjectCacheError::Validation(error_message.to_string())
                    }
                    _ => InvalidateProjectCacheError::Unknown(String::from(body)),
                }
            }
            Err(_) => InvalidateProjectCacheError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for InvalidateProjectCacheError {
    fn from(err: serde_json::error::Error) -> InvalidateProjectCacheError {
        InvalidateProjectCacheError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for InvalidateProjectCacheError {
    fn from(err: CredentialsError) -> InvalidateProjectCacheError {
        InvalidateProjectCacheError::Credentials(err)
    }
}
impl From<HttpDispatchError> for InvalidateProjectCacheError {
    fn from(err: HttpDispatchError) -> InvalidateProjectCacheError {
        InvalidateProjectCacheError::HttpDispatch(err)
    }
}
impl From<io::Error> for InvalidateProjectCacheError {
    fn from(err: io::Error) -> InvalidateProjectCacheError {
        InvalidateProjectCacheError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for InvalidateProjectCacheError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for InvalidateProjectCacheError {
    fn description(&self) -> &str {
        match *self {
            InvalidateProjectCacheError::InvalidInput(ref cause) => cause,
            InvalidateProjectCacheError::ResourceNotFound(ref cause) => cause,
            InvalidateProjectCacheError::Validation(ref cause) => cause,
            InvalidateProjectCacheError::Credentials(ref err) => err.description(),
            InvalidateProjectCacheError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            InvalidateProjectCacheError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by ListBuilds
#[derive(Debug, PartialEq)]
pub enum ListBuildsError {
    /// <p>The input value that was provided is not valid.</p>
    InvalidInput(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl ListBuildsError {
    pub fn from_body(body: &str) -> ListBuildsError {
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
                    "InvalidInputException" => {
                        ListBuildsError::InvalidInput(String::from(error_message))
                    }
                    "ValidationException" => ListBuildsError::Validation(error_message.to_string()),
                    _ => ListBuildsError::Unknown(String::from(body)),
                }
            }
            Err(_) => ListBuildsError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for ListBuildsError {
    fn from(err: serde_json::error::Error) -> ListBuildsError {
        ListBuildsError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for ListBuildsError {
    fn from(err: CredentialsError) -> ListBuildsError {
        ListBuildsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ListBuildsError {
    fn from(err: HttpDispatchError) -> ListBuildsError {
        ListBuildsError::HttpDispatch(err)
    }
}
impl From<io::Error> for ListBuildsError {
    fn from(err: io::Error) -> ListBuildsError {
        ListBuildsError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for ListBuildsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListBuildsError {
    fn description(&self) -> &str {
        match *self {
            ListBuildsError::InvalidInput(ref cause) => cause,
            ListBuildsError::Validation(ref cause) => cause,
            ListBuildsError::Credentials(ref err) => err.description(),
            ListBuildsError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            ListBuildsError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by ListBuildsForProject
#[derive(Debug, PartialEq)]
pub enum ListBuildsForProjectError {
    /// <p>The input value that was provided is not valid.</p>
    InvalidInput(String),
    /// <p>The specified AWS resource cannot be found.</p>
    ResourceNotFound(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl ListBuildsForProjectError {
    pub fn from_body(body: &str) -> ListBuildsForProjectError {
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
                    "InvalidInputException" => {
                        ListBuildsForProjectError::InvalidInput(String::from(error_message))
                    }
                    "ResourceNotFoundException" => {
                        ListBuildsForProjectError::ResourceNotFound(String::from(error_message))
                    }
                    "ValidationException" => {
                        ListBuildsForProjectError::Validation(error_message.to_string())
                    }
                    _ => ListBuildsForProjectError::Unknown(String::from(body)),
                }
            }
            Err(_) => ListBuildsForProjectError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for ListBuildsForProjectError {
    fn from(err: serde_json::error::Error) -> ListBuildsForProjectError {
        ListBuildsForProjectError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for ListBuildsForProjectError {
    fn from(err: CredentialsError) -> ListBuildsForProjectError {
        ListBuildsForProjectError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ListBuildsForProjectError {
    fn from(err: HttpDispatchError) -> ListBuildsForProjectError {
        ListBuildsForProjectError::HttpDispatch(err)
    }
}
impl From<io::Error> for ListBuildsForProjectError {
    fn from(err: io::Error) -> ListBuildsForProjectError {
        ListBuildsForProjectError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for ListBuildsForProjectError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListBuildsForProjectError {
    fn description(&self) -> &str {
        match *self {
            ListBuildsForProjectError::InvalidInput(ref cause) => cause,
            ListBuildsForProjectError::ResourceNotFound(ref cause) => cause,
            ListBuildsForProjectError::Validation(ref cause) => cause,
            ListBuildsForProjectError::Credentials(ref err) => err.description(),
            ListBuildsForProjectError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            ListBuildsForProjectError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by ListCuratedEnvironmentImages
#[derive(Debug, PartialEq)]
pub enum ListCuratedEnvironmentImagesError {
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl ListCuratedEnvironmentImagesError {
    pub fn from_body(body: &str) -> ListCuratedEnvironmentImagesError {
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
                        ListCuratedEnvironmentImagesError::Validation(error_message.to_string())
                    }
                    _ => ListCuratedEnvironmentImagesError::Unknown(String::from(body)),
                }
            }
            Err(_) => ListCuratedEnvironmentImagesError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for ListCuratedEnvironmentImagesError {
    fn from(err: serde_json::error::Error) -> ListCuratedEnvironmentImagesError {
        ListCuratedEnvironmentImagesError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for ListCuratedEnvironmentImagesError {
    fn from(err: CredentialsError) -> ListCuratedEnvironmentImagesError {
        ListCuratedEnvironmentImagesError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ListCuratedEnvironmentImagesError {
    fn from(err: HttpDispatchError) -> ListCuratedEnvironmentImagesError {
        ListCuratedEnvironmentImagesError::HttpDispatch(err)
    }
}
impl From<io::Error> for ListCuratedEnvironmentImagesError {
    fn from(err: io::Error) -> ListCuratedEnvironmentImagesError {
        ListCuratedEnvironmentImagesError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for ListCuratedEnvironmentImagesError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListCuratedEnvironmentImagesError {
    fn description(&self) -> &str {
        match *self {
            ListCuratedEnvironmentImagesError::Validation(ref cause) => cause,
            ListCuratedEnvironmentImagesError::Credentials(ref err) => err.description(),
            ListCuratedEnvironmentImagesError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            ListCuratedEnvironmentImagesError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by ListProjects
#[derive(Debug, PartialEq)]
pub enum ListProjectsError {
    /// <p>The input value that was provided is not valid.</p>
    InvalidInput(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl ListProjectsError {
    pub fn from_body(body: &str) -> ListProjectsError {
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
                    "InvalidInputException" => {
                        ListProjectsError::InvalidInput(String::from(error_message))
                    }
                    "ValidationException" => {
                        ListProjectsError::Validation(error_message.to_string())
                    }
                    _ => ListProjectsError::Unknown(String::from(body)),
                }
            }
            Err(_) => ListProjectsError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for ListProjectsError {
    fn from(err: serde_json::error::Error) -> ListProjectsError {
        ListProjectsError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for ListProjectsError {
    fn from(err: CredentialsError) -> ListProjectsError {
        ListProjectsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ListProjectsError {
    fn from(err: HttpDispatchError) -> ListProjectsError {
        ListProjectsError::HttpDispatch(err)
    }
}
impl From<io::Error> for ListProjectsError {
    fn from(err: io::Error) -> ListProjectsError {
        ListProjectsError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for ListProjectsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListProjectsError {
    fn description(&self) -> &str {
        match *self {
            ListProjectsError::InvalidInput(ref cause) => cause,
            ListProjectsError::Validation(ref cause) => cause,
            ListProjectsError::Credentials(ref err) => err.description(),
            ListProjectsError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            ListProjectsError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by StartBuild
#[derive(Debug, PartialEq)]
pub enum StartBuildError {
    /// <p>An AWS service limit was exceeded for the calling AWS account.</p>
    AccountLimitExceeded(String),
    /// <p>The input value that was provided is not valid.</p>
    InvalidInput(String),
    /// <p>The specified AWS resource cannot be found.</p>
    ResourceNotFound(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl StartBuildError {
    pub fn from_body(body: &str) -> StartBuildError {
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
                    "AccountLimitExceededException" => {
                        StartBuildError::AccountLimitExceeded(String::from(error_message))
                    }
                    "InvalidInputException" => {
                        StartBuildError::InvalidInput(String::from(error_message))
                    }
                    "ResourceNotFoundException" => {
                        StartBuildError::ResourceNotFound(String::from(error_message))
                    }
                    "ValidationException" => StartBuildError::Validation(error_message.to_string()),
                    _ => StartBuildError::Unknown(String::from(body)),
                }
            }
            Err(_) => StartBuildError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for StartBuildError {
    fn from(err: serde_json::error::Error) -> StartBuildError {
        StartBuildError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for StartBuildError {
    fn from(err: CredentialsError) -> StartBuildError {
        StartBuildError::Credentials(err)
    }
}
impl From<HttpDispatchError> for StartBuildError {
    fn from(err: HttpDispatchError) -> StartBuildError {
        StartBuildError::HttpDispatch(err)
    }
}
impl From<io::Error> for StartBuildError {
    fn from(err: io::Error) -> StartBuildError {
        StartBuildError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for StartBuildError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for StartBuildError {
    fn description(&self) -> &str {
        match *self {
            StartBuildError::AccountLimitExceeded(ref cause) => cause,
            StartBuildError::InvalidInput(ref cause) => cause,
            StartBuildError::ResourceNotFound(ref cause) => cause,
            StartBuildError::Validation(ref cause) => cause,
            StartBuildError::Credentials(ref err) => err.description(),
            StartBuildError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            StartBuildError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by StopBuild
#[derive(Debug, PartialEq)]
pub enum StopBuildError {
    /// <p>The input value that was provided is not valid.</p>
    InvalidInput(String),
    /// <p>The specified AWS resource cannot be found.</p>
    ResourceNotFound(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl StopBuildError {
    pub fn from_body(body: &str) -> StopBuildError {
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
                    "InvalidInputException" => {
                        StopBuildError::InvalidInput(String::from(error_message))
                    }
                    "ResourceNotFoundException" => {
                        StopBuildError::ResourceNotFound(String::from(error_message))
                    }
                    "ValidationException" => StopBuildError::Validation(error_message.to_string()),
                    _ => StopBuildError::Unknown(String::from(body)),
                }
            }
            Err(_) => StopBuildError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for StopBuildError {
    fn from(err: serde_json::error::Error) -> StopBuildError {
        StopBuildError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for StopBuildError {
    fn from(err: CredentialsError) -> StopBuildError {
        StopBuildError::Credentials(err)
    }
}
impl From<HttpDispatchError> for StopBuildError {
    fn from(err: HttpDispatchError) -> StopBuildError {
        StopBuildError::HttpDispatch(err)
    }
}
impl From<io::Error> for StopBuildError {
    fn from(err: io::Error) -> StopBuildError {
        StopBuildError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for StopBuildError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for StopBuildError {
    fn description(&self) -> &str {
        match *self {
            StopBuildError::InvalidInput(ref cause) => cause,
            StopBuildError::ResourceNotFound(ref cause) => cause,
            StopBuildError::Validation(ref cause) => cause,
            StopBuildError::Credentials(ref err) => err.description(),
            StopBuildError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            StopBuildError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by UpdateProject
#[derive(Debug, PartialEq)]
pub enum UpdateProjectError {
    /// <p>The input value that was provided is not valid.</p>
    InvalidInput(String),
    /// <p>The specified AWS resource cannot be found.</p>
    ResourceNotFound(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl UpdateProjectError {
    pub fn from_body(body: &str) -> UpdateProjectError {
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
                    "InvalidInputException" => {
                        UpdateProjectError::InvalidInput(String::from(error_message))
                    }
                    "ResourceNotFoundException" => {
                        UpdateProjectError::ResourceNotFound(String::from(error_message))
                    }
                    "ValidationException" => {
                        UpdateProjectError::Validation(error_message.to_string())
                    }
                    _ => UpdateProjectError::Unknown(String::from(body)),
                }
            }
            Err(_) => UpdateProjectError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for UpdateProjectError {
    fn from(err: serde_json::error::Error) -> UpdateProjectError {
        UpdateProjectError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for UpdateProjectError {
    fn from(err: CredentialsError) -> UpdateProjectError {
        UpdateProjectError::Credentials(err)
    }
}
impl From<HttpDispatchError> for UpdateProjectError {
    fn from(err: HttpDispatchError) -> UpdateProjectError {
        UpdateProjectError::HttpDispatch(err)
    }
}
impl From<io::Error> for UpdateProjectError {
    fn from(err: io::Error) -> UpdateProjectError {
        UpdateProjectError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for UpdateProjectError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for UpdateProjectError {
    fn description(&self) -> &str {
        match *self {
            UpdateProjectError::InvalidInput(ref cause) => cause,
            UpdateProjectError::ResourceNotFound(ref cause) => cause,
            UpdateProjectError::Validation(ref cause) => cause,
            UpdateProjectError::Credentials(ref err) => err.description(),
            UpdateProjectError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            UpdateProjectError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by UpdateWebhook
#[derive(Debug, PartialEq)]
pub enum UpdateWebhookError {
    /// <p>The input value that was provided is not valid.</p>
    InvalidInput(String),
    /// <p>There was a problem with the underlying OAuth provider.</p>
    OAuthProvider(String),
    /// <p>The specified AWS resource cannot be found.</p>
    ResourceNotFound(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl UpdateWebhookError {
    pub fn from_body(body: &str) -> UpdateWebhookError {
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
                    "InvalidInputException" => {
                        UpdateWebhookError::InvalidInput(String::from(error_message))
                    }
                    "OAuthProviderException" => {
                        UpdateWebhookError::OAuthProvider(String::from(error_message))
                    }
                    "ResourceNotFoundException" => {
                        UpdateWebhookError::ResourceNotFound(String::from(error_message))
                    }
                    "ValidationException" => {
                        UpdateWebhookError::Validation(error_message.to_string())
                    }
                    _ => UpdateWebhookError::Unknown(String::from(body)),
                }
            }
            Err(_) => UpdateWebhookError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for UpdateWebhookError {
    fn from(err: serde_json::error::Error) -> UpdateWebhookError {
        UpdateWebhookError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for UpdateWebhookError {
    fn from(err: CredentialsError) -> UpdateWebhookError {
        UpdateWebhookError::Credentials(err)
    }
}
impl From<HttpDispatchError> for UpdateWebhookError {
    fn from(err: HttpDispatchError) -> UpdateWebhookError {
        UpdateWebhookError::HttpDispatch(err)
    }
}
impl From<io::Error> for UpdateWebhookError {
    fn from(err: io::Error) -> UpdateWebhookError {
        UpdateWebhookError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for UpdateWebhookError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for UpdateWebhookError {
    fn description(&self) -> &str {
        match *self {
            UpdateWebhookError::InvalidInput(ref cause) => cause,
            UpdateWebhookError::OAuthProvider(ref cause) => cause,
            UpdateWebhookError::ResourceNotFound(ref cause) => cause,
            UpdateWebhookError::Validation(ref cause) => cause,
            UpdateWebhookError::Credentials(ref err) => err.description(),
            UpdateWebhookError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            UpdateWebhookError::Unknown(ref cause) => cause,
        }
    }
}
/// Trait representing the capabilities of the AWS CodeBuild API. AWS CodeBuild clients implement this trait.
pub trait CodeBuild {
    /// <p>Deletes one or more builds.</p>
    fn batch_delete_builds(
        &self,
        input: BatchDeleteBuildsInput,
    ) -> RusotoFuture<BatchDeleteBuildsOutput, BatchDeleteBuildsError>;

    /// <p>Gets information about builds.</p>
    fn batch_get_builds(
        &self,
        input: BatchGetBuildsInput,
    ) -> RusotoFuture<BatchGetBuildsOutput, BatchGetBuildsError>;

    /// <p>Gets information about build projects.</p>
    fn batch_get_projects(
        &self,
        input: BatchGetProjectsInput,
    ) -> RusotoFuture<BatchGetProjectsOutput, BatchGetProjectsError>;

    /// <p>Creates a build project.</p>
    fn create_project(
        &self,
        input: CreateProjectInput,
    ) -> RusotoFuture<CreateProjectOutput, CreateProjectError>;

    /// <p><p>For an existing AWS CodeBuild build project that has its source code stored in a GitHub repository, enables AWS CodeBuild to begin automatically rebuilding the source code every time a code change is pushed to the repository.</p> <important> <p>If you enable webhooks for an AWS CodeBuild project, and the project is used as a build step in AWS CodePipeline, then two identical builds will be created for each commit. One build is triggered through webhooks, and one through AWS CodePipeline. Because billing is on a per-build basis, you will be billed for both builds. Therefore, if you are using AWS CodePipeline, we recommend that you disable webhooks in CodeBuild. In the AWS CodeBuild console, clear the Webhook box. For more information, see step 9 in <a href="http://docs.aws.amazon.com/codebuild/latest/userguide/change-project.html#change-project-console">Change a Build Project&#39;s Settings</a>.</p> </important></p>
    fn create_webhook(
        &self,
        input: CreateWebhookInput,
    ) -> RusotoFuture<CreateWebhookOutput, CreateWebhookError>;

    /// <p>Deletes a build project.</p>
    fn delete_project(
        &self,
        input: DeleteProjectInput,
    ) -> RusotoFuture<DeleteProjectOutput, DeleteProjectError>;

    /// <p>For an existing AWS CodeBuild build project that has its source code stored in a GitHub repository, stops AWS CodeBuild from automatically rebuilding the source code every time a code change is pushed to the repository.</p>
    fn delete_webhook(
        &self,
        input: DeleteWebhookInput,
    ) -> RusotoFuture<DeleteWebhookOutput, DeleteWebhookError>;

    /// <p>Resets the cache for a project.</p>
    fn invalidate_project_cache(
        &self,
        input: InvalidateProjectCacheInput,
    ) -> RusotoFuture<InvalidateProjectCacheOutput, InvalidateProjectCacheError>;

    /// <p>Gets a list of build IDs, with each build ID representing a single build.</p>
    fn list_builds(
        &self,
        input: ListBuildsInput,
    ) -> RusotoFuture<ListBuildsOutput, ListBuildsError>;

    /// <p>Gets a list of build IDs for the specified build project, with each build ID representing a single build.</p>
    fn list_builds_for_project(
        &self,
        input: ListBuildsForProjectInput,
    ) -> RusotoFuture<ListBuildsForProjectOutput, ListBuildsForProjectError>;

    /// <p>Gets information about Docker images that are managed by AWS CodeBuild.</p>
    fn list_curated_environment_images(
        &self,
    ) -> RusotoFuture<ListCuratedEnvironmentImagesOutput, ListCuratedEnvironmentImagesError>;

    /// <p>Gets a list of build project names, with each build project name representing a single build project.</p>
    fn list_projects(
        &self,
        input: ListProjectsInput,
    ) -> RusotoFuture<ListProjectsOutput, ListProjectsError>;

    /// <p>Starts running a build.</p>
    fn start_build(
        &self,
        input: StartBuildInput,
    ) -> RusotoFuture<StartBuildOutput, StartBuildError>;

    /// <p>Attempts to stop running a build.</p>
    fn stop_build(&self, input: StopBuildInput) -> RusotoFuture<StopBuildOutput, StopBuildError>;

    /// <p>Changes the settings of a build project.</p>
    fn update_project(
        &self,
        input: UpdateProjectInput,
    ) -> RusotoFuture<UpdateProjectOutput, UpdateProjectError>;

    /// <p> Updates the webhook associated with an AWS CodeBuild build project. </p>
    fn update_webhook(
        &self,
        input: UpdateWebhookInput,
    ) -> RusotoFuture<UpdateWebhookOutput, UpdateWebhookError>;
}
/// A client for the AWS CodeBuild API.
pub struct CodeBuildClient<P = CredentialsProvider, D = RequestDispatcher>
where
    P: ProvideAwsCredentials,
    D: DispatchSignedRequest,
{
    inner: ClientInner<P, D>,
    region: region::Region,
}

impl CodeBuildClient {
    /// Creates a simple client backed by an implicit event loop.
    ///
    /// The client will use the default credentials provider and tls client.
    ///
    /// See the `rusoto_core::reactor` module for more details.
    pub fn simple(region: region::Region) -> CodeBuildClient {
        CodeBuildClient::new(
            RequestDispatcher::default(),
            CredentialsProvider::default(),
            region,
        )
    }
}

impl<P, D> CodeBuildClient<P, D>
where
    P: ProvideAwsCredentials,
    D: DispatchSignedRequest,
{
    pub fn new(request_dispatcher: D, credentials_provider: P, region: region::Region) -> Self {
        CodeBuildClient {
            inner: ClientInner::new(credentials_provider, request_dispatcher),
            region: region,
        }
    }
}

impl<P, D> CodeBuild for CodeBuildClient<P, D>
where
    P: ProvideAwsCredentials + 'static,
    D: DispatchSignedRequest + 'static,
{
    /// <p>Deletes one or more builds.</p>
    fn batch_delete_builds(
        &self,
        input: BatchDeleteBuildsInput,
    ) -> RusotoFuture<BatchDeleteBuildsOutput, BatchDeleteBuildsError> {
        let mut request = SignedRequest::new("POST", "codebuild", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "CodeBuild_20161006.BatchDeleteBuilds");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<BatchDeleteBuildsOutput>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(BatchDeleteBuildsError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Gets information about builds.</p>
    fn batch_get_builds(
        &self,
        input: BatchGetBuildsInput,
    ) -> RusotoFuture<BatchGetBuildsOutput, BatchGetBuildsError> {
        let mut request = SignedRequest::new("POST", "codebuild", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "CodeBuild_20161006.BatchGetBuilds");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<BatchGetBuildsOutput>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(BatchGetBuildsError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Gets information about build projects.</p>
    fn batch_get_projects(
        &self,
        input: BatchGetProjectsInput,
    ) -> RusotoFuture<BatchGetProjectsOutput, BatchGetProjectsError> {
        let mut request = SignedRequest::new("POST", "codebuild", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "CodeBuild_20161006.BatchGetProjects");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<BatchGetProjectsOutput>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(BatchGetProjectsError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Creates a build project.</p>
    fn create_project(
        &self,
        input: CreateProjectInput,
    ) -> RusotoFuture<CreateProjectOutput, CreateProjectError> {
        let mut request = SignedRequest::new("POST", "codebuild", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "CodeBuild_20161006.CreateProject");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<CreateProjectOutput>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(CreateProjectError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p><p>For an existing AWS CodeBuild build project that has its source code stored in a GitHub repository, enables AWS CodeBuild to begin automatically rebuilding the source code every time a code change is pushed to the repository.</p> <important> <p>If you enable webhooks for an AWS CodeBuild project, and the project is used as a build step in AWS CodePipeline, then two identical builds will be created for each commit. One build is triggered through webhooks, and one through AWS CodePipeline. Because billing is on a per-build basis, you will be billed for both builds. Therefore, if you are using AWS CodePipeline, we recommend that you disable webhooks in CodeBuild. In the AWS CodeBuild console, clear the Webhook box. For more information, see step 9 in <a href="http://docs.aws.amazon.com/codebuild/latest/userguide/change-project.html#change-project-console">Change a Build Project&#39;s Settings</a>.</p> </important></p>
    fn create_webhook(
        &self,
        input: CreateWebhookInput,
    ) -> RusotoFuture<CreateWebhookOutput, CreateWebhookError> {
        let mut request = SignedRequest::new("POST", "codebuild", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "CodeBuild_20161006.CreateWebhook");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<CreateWebhookOutput>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(CreateWebhookError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Deletes a build project.</p>
    fn delete_project(
        &self,
        input: DeleteProjectInput,
    ) -> RusotoFuture<DeleteProjectOutput, DeleteProjectError> {
        let mut request = SignedRequest::new("POST", "codebuild", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "CodeBuild_20161006.DeleteProject");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<DeleteProjectOutput>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(DeleteProjectError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>For an existing AWS CodeBuild build project that has its source code stored in a GitHub repository, stops AWS CodeBuild from automatically rebuilding the source code every time a code change is pushed to the repository.</p>
    fn delete_webhook(
        &self,
        input: DeleteWebhookInput,
    ) -> RusotoFuture<DeleteWebhookOutput, DeleteWebhookError> {
        let mut request = SignedRequest::new("POST", "codebuild", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "CodeBuild_20161006.DeleteWebhook");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<DeleteWebhookOutput>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(DeleteWebhookError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Resets the cache for a project.</p>
    fn invalidate_project_cache(
        &self,
        input: InvalidateProjectCacheInput,
    ) -> RusotoFuture<InvalidateProjectCacheOutput, InvalidateProjectCacheError> {
        let mut request = SignedRequest::new("POST", "codebuild", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "CodeBuild_20161006.InvalidateProjectCache");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<InvalidateProjectCacheOutput>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(InvalidateProjectCacheError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Gets a list of build IDs, with each build ID representing a single build.</p>
    fn list_builds(
        &self,
        input: ListBuildsInput,
    ) -> RusotoFuture<ListBuildsOutput, ListBuildsError> {
        let mut request = SignedRequest::new("POST", "codebuild", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "CodeBuild_20161006.ListBuilds");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<ListBuildsOutput>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(ListBuildsError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Gets a list of build IDs for the specified build project, with each build ID representing a single build.</p>
    fn list_builds_for_project(
        &self,
        input: ListBuildsForProjectInput,
    ) -> RusotoFuture<ListBuildsForProjectOutput, ListBuildsForProjectError> {
        let mut request = SignedRequest::new("POST", "codebuild", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "CodeBuild_20161006.ListBuildsForProject");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<ListBuildsForProjectOutput>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(ListBuildsForProjectError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Gets information about Docker images that are managed by AWS CodeBuild.</p>
    fn list_curated_environment_images(
        &self,
    ) -> RusotoFuture<ListCuratedEnvironmentImagesOutput, ListCuratedEnvironmentImagesError> {
        let mut request = SignedRequest::new("POST", "codebuild", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "CodeBuild_20161006.ListCuratedEnvironmentImages",
        );
        request.set_payload(Some(b"{}".to_vec()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<ListCuratedEnvironmentImagesOutput>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(ListCuratedEnvironmentImagesError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Gets a list of build project names, with each build project name representing a single build project.</p>
    fn list_projects(
        &self,
        input: ListProjectsInput,
    ) -> RusotoFuture<ListProjectsOutput, ListProjectsError> {
        let mut request = SignedRequest::new("POST", "codebuild", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "CodeBuild_20161006.ListProjects");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<ListProjectsOutput>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(ListProjectsError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Starts running a build.</p>
    fn start_build(
        &self,
        input: StartBuildInput,
    ) -> RusotoFuture<StartBuildOutput, StartBuildError> {
        let mut request = SignedRequest::new("POST", "codebuild", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "CodeBuild_20161006.StartBuild");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<StartBuildOutput>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(StartBuildError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Attempts to stop running a build.</p>
    fn stop_build(&self, input: StopBuildInput) -> RusotoFuture<StopBuildOutput, StopBuildError> {
        let mut request = SignedRequest::new("POST", "codebuild", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "CodeBuild_20161006.StopBuild");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<StopBuildOutput>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(StopBuildError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Changes the settings of a build project.</p>
    fn update_project(
        &self,
        input: UpdateProjectInput,
    ) -> RusotoFuture<UpdateProjectOutput, UpdateProjectError> {
        let mut request = SignedRequest::new("POST", "codebuild", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "CodeBuild_20161006.UpdateProject");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<UpdateProjectOutput>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(UpdateProjectError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p> Updates the webhook associated with an AWS CodeBuild build project. </p>
    fn update_webhook(
        &self,
        input: UpdateWebhookInput,
    ) -> RusotoFuture<UpdateWebhookOutput, UpdateWebhookError> {
        let mut request = SignedRequest::new("POST", "codebuild", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "CodeBuild_20161006.UpdateWebhook");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<UpdateWebhookOutput>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(UpdateWebhookError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }
}

#[cfg(test)]
mod protocol_tests {}
