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
#[allow(warnings)]
use rusoto_core::request::{BufferedHttpResponse, DispatchSignedRequest};
use rusoto_core::{Client, RusotoError};

use rusoto_core::proto;
use rusoto_core::signature::SignedRequest;
#[allow(unused_imports)]
use serde::{Deserialize, Serialize};
use serde_json;
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct BatchDeleteBuildsInput {
    /// <p>The IDs of the builds to delete.</p>
    #[serde(rename = "ids")]
    pub ids: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct BatchGetBuildsInput {
    /// <p>The IDs of the builds.</p>
    #[serde(rename = "ids")]
    pub ids: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct BatchGetProjectsInput {
    /// <p>The names or ARNs of the build projects. To get information about a project shared with your AWS account, its ARN must be specified. You cannot specify a shared project using its name.</p>
    #[serde(rename = "names")]
    pub names: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct BatchGetReportGroupsInput {
    /// <p> An array of report group ARNs that identify the report groups to return. </p>
    #[serde(rename = "reportGroupArns")]
    pub report_group_arns: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct BatchGetReportGroupsOutput {
    /// <p> The array of report groups returned by <code>BatchGetReportGroups</code>. </p>
    #[serde(rename = "reportGroups")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub report_groups: Option<Vec<ReportGroup>>,
    /// <p> An array of ARNs passed to <code>BatchGetReportGroups</code> that are not associated with a <code>ReportGroup</code>. </p>
    #[serde(rename = "reportGroupsNotFound")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub report_groups_not_found: Option<Vec<String>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct BatchGetReportsInput {
    /// <p> An array of ARNs that identify the <code>Report</code> objects to return. </p>
    #[serde(rename = "reportArns")]
    pub report_arns: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct BatchGetReportsOutput {
    /// <p> The array of <code>Report</code> objects returned by <code>BatchGetReports</code>. </p>
    #[serde(rename = "reports")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reports: Option<Vec<Report>>,
    /// <p> An array of ARNs passed to <code>BatchGetReportGroups</code> that are not associated with a <code>Report</code>. </p>
    #[serde(rename = "reportsNotFound")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reports_not_found: Option<Vec<String>>,
}

/// <p>Information about a build.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct Build {
    /// <p>The Amazon Resource Name (ARN) of the build.</p>
    #[serde(rename = "arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p>Information about the output artifacts for the build.</p>
    #[serde(rename = "artifacts")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub artifacts: Option<BuildArtifacts>,
    /// <p>Whether the build is complete. True if complete; otherwise, false.</p>
    #[serde(rename = "buildComplete")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub build_complete: Option<bool>,
    /// <p>The number of the build. For each project, the <code>buildNumber</code> of its first build is <code>1</code>. The <code>buildNumber</code> of each subsequent build is incremented by <code>1</code>. If a build is deleted, the <code>buildNumber</code> of other builds does not change.</p>
    #[serde(rename = "buildNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub build_number: Option<i64>,
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
    /// <p>The AWS Key Management Service (AWS KMS) customer master key (CMK) to be used for encrypting the build output artifacts.</p> <note> <p> You can use a cross-account KMS key to encrypt the build output artifacts if your service role has permission to that key. </p> </note> <p>You can specify either the Amazon Resource Name (ARN) of the CMK or, if available, the CMK's alias (using the format <code>alias/<i>alias-name</i> </code>).</p>
    #[serde(rename = "encryptionKey")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encryption_key: Option<String>,
    /// <p>When the build process ended, expressed in Unix time format.</p>
    #[serde(rename = "endTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<f64>,
    /// <p>Information about the build environment for this build.</p>
    #[serde(rename = "environment")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub environment: Option<ProjectEnvironment>,
    /// <p> A list of exported environment variables for this build. </p>
    #[serde(rename = "exportedEnvironmentVariables")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exported_environment_variables: Option<Vec<ExportedEnvironmentVariable>>,
    /// <p>The unique ID for the build.</p>
    #[serde(rename = "id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// <p><p>The entity that started the build. Valid values include:</p> <ul> <li> <p>If AWS CodePipeline started the build, the pipeline&#39;s name (for example, <code>codepipeline/my-demo-pipeline</code>).</p> </li> <li> <p>If an AWS Identity and Access Management (IAM) user started the build, the user&#39;s name (for example, <code>MyUserName</code>).</p> </li> <li> <p>If the Jenkins plugin for AWS CodeBuild started the build, the string <code>CodeBuild-Jenkins-Plugin</code>.</p> </li> </ul></p>
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
    /// <p>Information about all previous build phases that are complete and information about any current build phase that is not yet complete.</p>
    #[serde(rename = "phases")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phases: Option<Vec<BuildPhase>>,
    /// <p>The name of the AWS CodeBuild project.</p>
    #[serde(rename = "projectName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub project_name: Option<String>,
    /// <p> The number of minutes a build is allowed to be queued before it times out. </p>
    #[serde(rename = "queuedTimeoutInMinutes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub queued_timeout_in_minutes: Option<i64>,
    /// <p> An array of the ARNs associated with this build's reports. </p>
    #[serde(rename = "reportArns")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub report_arns: Option<Vec<String>>,
    /// <p><p> An identifier for the version of this build&#39;s source code. </p> <ul> <li> <p> For AWS CodeCommit, GitHub, GitHub Enterprise, and BitBucket, the commit ID. </p> </li> <li> <p> For AWS CodePipeline, the source revision provided by AWS CodePipeline. </p> </li> <li> <p> For Amazon Simple Storage Service (Amazon S3), this does not apply. </p> </li> </ul></p>
    #[serde(rename = "resolvedSourceVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resolved_source_version: Option<String>,
    /// <p> An array of <code>ProjectArtifacts</code> objects. </p>
    #[serde(rename = "secondaryArtifacts")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secondary_artifacts: Option<Vec<BuildArtifacts>>,
    /// <p><p> An array of <code>ProjectSourceVersion</code> objects. Each <code>ProjectSourceVersion</code> must be one of: </p> <ul> <li> <p>For AWS CodeCommit: the commit ID, branch, or Git tag to use.</p> </li> <li> <p>For GitHub: the commit ID, pull request ID, branch name, or tag name that corresponds to the version of the source code you want to build. If a pull request ID is specified, it must use the format <code>pr/pull-request-ID</code> (for example, <code>pr/25</code>). If a branch name is specified, the branch&#39;s HEAD commit ID is used. If not specified, the default branch&#39;s HEAD commit ID is used.</p> </li> <li> <p>For Bitbucket: the commit ID, branch name, or tag name that corresponds to the version of the source code you want to build. If a branch name is specified, the branch&#39;s HEAD commit ID is used. If not specified, the default branch&#39;s HEAD commit ID is used.</p> </li> <li> <p>For Amazon Simple Storage Service (Amazon S3): the version ID of the object that represents the build input ZIP file to use.</p> </li> </ul></p>
    #[serde(rename = "secondarySourceVersions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secondary_source_versions: Option<Vec<ProjectSourceVersion>>,
    /// <p> An array of <code>ProjectSource</code> objects. </p>
    #[serde(rename = "secondarySources")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secondary_sources: Option<Vec<ProjectSource>>,
    /// <p>The name of a service role used for this build.</p>
    #[serde(rename = "serviceRole")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_role: Option<String>,
    /// <p>Information about the source code to be built.</p>
    #[serde(rename = "source")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<ProjectSource>,
    /// <p>Any version identifier for the version of the source code to be built. If <code>sourceVersion</code> is specified at the project level, then this <code>sourceVersion</code> (at the build level) takes precedence. </p> <p> For more information, see <a href="https://docs.aws.amazon.com/codebuild/latest/userguide/sample-source-version.html">Source Version Sample with CodeBuild</a> in the <i>AWS CodeBuild User Guide</i>. </p>
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct BuildArtifacts {
    /// <p> An identifier for this artifact definition. </p>
    #[serde(rename = "artifactIdentifier")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub artifact_identifier: Option<String>,
    /// <p> Information that tells you if encryption for build artifacts is disabled. </p>
    #[serde(rename = "encryptionDisabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encryption_disabled: Option<bool>,
    /// <p>Information about the location of the build artifacts.</p>
    #[serde(rename = "location")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    /// <p><p>The MD5 hash of the build artifact.</p> <p>You can use this hash along with a checksum tool to confirm file integrity and authenticity.</p> <note> <p>This value is available only if the build project&#39;s <code>packaging</code> value is set to <code>ZIP</code>.</p> </note></p>
    #[serde(rename = "md5sum")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub md_5sum: Option<String>,
    /// <p> If this flag is set, a name specified in the build spec file overrides the artifact name. The name specified in a build spec file is calculated at build time and uses the Shell Command Language. For example, you can append a date and time to your artifact name so that it is always unique. </p>
    #[serde(rename = "overrideArtifactName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub override_artifact_name: Option<bool>,
    /// <p><p>The SHA-256 hash of the build artifact.</p> <p>You can use this hash along with a checksum tool to confirm file integrity and authenticity.</p> <note> <p>This value is available only if the build project&#39;s <code>packaging</code> value is set to <code>ZIP</code>.</p> </note></p>
    #[serde(rename = "sha256sum")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sha_25_6sum: Option<String>,
}

/// <p>Information about a build that could not be successfully deleted.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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
    /// <p><p>The current status of the build phase. Valid values include:</p> <ul> <li> <p> <code>FAILED</code>: The build phase failed.</p> </li> <li> <p> <code>FAULT</code>: The build phase faulted.</p> </li> <li> <p> <code>IN<em>PROGRESS</code>: The build phase is still in progress.</p> </li> <li> <p> <code>QUEUED</code>: The build has been submitted and is queued behind other submitted builds.</p> </li> <li> <p> <code>STOPPED</code>: The build phase stopped.</p> </li> <li> <p> <code>SUCCEEDED</code>: The build phase succeeded.</p> </li> <li> <p> <code>TIMED</em>OUT</code>: The build phase timed out.</p> </li> </ul></p>
    #[serde(rename = "phaseStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phase_status: Option<String>,
    /// <p><p>The name of the build phase. Valid values include:</p> <ul> <li> <p> <code>BUILD</code>: Core build activities typically occur in this build phase.</p> </li> <li> <p> <code>COMPLETED</code>: The build has been completed.</p> </li> <li> <p> <code>DOWNLOAD<em>SOURCE</code>: Source code is being downloaded in this build phase.</p> </li> <li> <p> <code>FINALIZING</code>: The build process is completing in this build phase.</p> </li> <li> <p> <code>INSTALL</code>: Installation activities typically occur in this build phase.</p> </li> <li> <p> <code>POST</em>BUILD</code>: Post-build activities typically occur in this build phase.</p> </li> <li> <p> <code>PRE<em>BUILD</code>: Pre-build activities typically occur in this build phase.</p> </li> <li> <p> <code>PROVISIONING</code>: The build environment is being set up.</p> </li> <li> <p> <code>QUEUED</code>: The build has been submitted and is queued behind other submitted builds.</p> </li> <li> <p> <code>SUBMITTED</code>: The build has been submitted.</p> </li> <li> <p> <code>UPLOAD</em>ARTIFACTS</code>: Build output artifacts are being uploaded to the output location.</p> </li> </ul></p>
    #[serde(rename = "phaseType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phase_type: Option<String>,
    /// <p>When the build phase started, expressed in Unix time format.</p>
    #[serde(rename = "startTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<f64>,
}

/// <p> Information about Amazon CloudWatch Logs for a build project. </p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CloudWatchLogsConfig {
    /// <p> The group name of the logs in Amazon CloudWatch Logs. For more information, see <a href="https://docs.aws.amazon.com/AmazonCloudWatch/latest/logs/Working-with-log-groups-and-streams.html">Working with Log Groups and Log Streams</a>. </p>
    #[serde(rename = "groupName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_name: Option<String>,
    /// <p><p>The current status of the logs in Amazon CloudWatch Logs for a build project. Valid values are:</p> <ul> <li> <p> <code>ENABLED</code>: Amazon CloudWatch Logs are enabled for this build project.</p> </li> <li> <p> <code>DISABLED</code>: Amazon CloudWatch Logs are not enabled for this build project.</p> </li> </ul></p>
    #[serde(rename = "status")]
    pub status: String,
    /// <p> The prefix of the stream name of the Amazon CloudWatch Logs. For more information, see <a href="https://docs.aws.amazon.com/AmazonCloudWatch/latest/logs/Working-with-log-groups-and-streams.html">Working with Log Groups and Log Streams</a>. </p>
    #[serde(rename = "streamName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream_name: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateProjectInput {
    /// <p>Information about the build output artifacts for the build project.</p>
    #[serde(rename = "artifacts")]
    pub artifacts: ProjectArtifacts,
    /// <p>Set this to true to generate a publicly accessible URL for your project's build badge.</p>
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
    /// <p>The AWS Key Management Service (AWS KMS) customer master key (CMK) to be used for encrypting the build output artifacts.</p> <note> <p> You can use a cross-account KMS key to encrypt the build output artifacts if your service role has permission to that key. </p> </note> <p>You can specify either the Amazon Resource Name (ARN) of the CMK or, if available, the CMK's alias (using the format <code>alias/<i>alias-name</i> </code>).</p>
    #[serde(rename = "encryptionKey")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encryption_key: Option<String>,
    /// <p>Information about the build environment for the build project.</p>
    #[serde(rename = "environment")]
    pub environment: ProjectEnvironment,
    /// <p> Information about logs for the build project. These can be logs in Amazon CloudWatch Logs, logs uploaded to a specified S3 bucket, or both. </p>
    #[serde(rename = "logsConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logs_config: Option<LogsConfig>,
    /// <p>The name of the build project.</p>
    #[serde(rename = "name")]
    pub name: String,
    /// <p> The number of minutes a build is allowed to be queued before it times out. </p>
    #[serde(rename = "queuedTimeoutInMinutes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub queued_timeout_in_minutes: Option<i64>,
    /// <p> An array of <code>ProjectArtifacts</code> objects. </p>
    #[serde(rename = "secondaryArtifacts")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secondary_artifacts: Option<Vec<ProjectArtifacts>>,
    /// <p> An array of <code>ProjectSourceVersion</code> objects. If <code>secondarySourceVersions</code> is specified at the build level, then they take precedence over these <code>secondarySourceVersions</code> (at the project level). </p>
    #[serde(rename = "secondarySourceVersions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secondary_source_versions: Option<Vec<ProjectSourceVersion>>,
    /// <p> An array of <code>ProjectSource</code> objects. </p>
    #[serde(rename = "secondarySources")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secondary_sources: Option<Vec<ProjectSource>>,
    /// <p>The ARN of the AWS Identity and Access Management (IAM) role that enables AWS CodeBuild to interact with dependent AWS services on behalf of the AWS account.</p>
    #[serde(rename = "serviceRole")]
    pub service_role: String,
    /// <p>Information about the build input source code for the build project.</p>
    #[serde(rename = "source")]
    pub source: ProjectSource,
    /// <p> A version of the build input to be built for this project. If not specified, the latest version is used. If specified, it must be one of: </p> <ul> <li> <p>For AWS CodeCommit: the commit ID, branch, or Git tag to use.</p> </li> <li> <p>For GitHub: the commit ID, pull request ID, branch name, or tag name that corresponds to the version of the source code you want to build. If a pull request ID is specified, it must use the format <code>pr/pull-request-ID</code> (for example <code>pr/25</code>). If a branch name is specified, the branch's HEAD commit ID is used. If not specified, the default branch's HEAD commit ID is used.</p> </li> <li> <p>For Bitbucket: the commit ID, branch name, or tag name that corresponds to the version of the source code you want to build. If a branch name is specified, the branch's HEAD commit ID is used. If not specified, the default branch's HEAD commit ID is used.</p> </li> <li> <p>For Amazon Simple Storage Service (Amazon S3): the version ID of the object that represents the build input ZIP file to use.</p> </li> </ul> <p> If <code>sourceVersion</code> is specified at the build level, then that version takes precedence over this <code>sourceVersion</code> (at the project level). </p> <p> For more information, see <a href="https://docs.aws.amazon.com/codebuild/latest/userguide/sample-source-version.html">Source Version Sample with CodeBuild</a> in the <i>AWS CodeBuild User Guide</i>. </p>
    #[serde(rename = "sourceVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_version: Option<String>,
    /// <p>A set of tags for this build project.</p> <p>These tags are available for use by AWS services that support AWS CodeBuild build project tags.</p>
    #[serde(rename = "tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
    /// <p>How long, in minutes, from 5 to 480 (8 hours), for AWS CodeBuild to wait before it times out any build that has not been marked as completed. The default is 60 minutes.</p>
    #[serde(rename = "timeoutInMinutes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeout_in_minutes: Option<i64>,
    /// <p>VpcConfig enables AWS CodeBuild to access resources in an Amazon VPC.</p>
    #[serde(rename = "vpcConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_config: Option<VpcConfig>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateProjectOutput {
    /// <p>Information about the build project that was created.</p>
    #[serde(rename = "project")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub project: Option<Project>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateReportGroupInput {
    /// <p> A <code>ReportExportConfig</code> object that contains information about where the report group test results are exported. </p>
    #[serde(rename = "exportConfig")]
    pub export_config: ReportExportConfig,
    /// <p> The name of the report group. </p>
    #[serde(rename = "name")]
    pub name: String,
    /// <p> The type of report group. </p>
    #[serde(rename = "type")]
    pub type_: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateReportGroupOutput {
    /// <p> Information about the report group that was created. </p>
    #[serde(rename = "reportGroup")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub report_group: Option<ReportGroup>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateWebhookInput {
    /// <p><p>A regular expression used to determine which repository branches are built when a webhook is triggered. If the name of a branch matches the regular expression, then it is built. If <code>branchFilter</code> is empty, then all branches are built.</p> <note> <p> It is recommended that you use <code>filterGroups</code> instead of <code>branchFilter</code>. </p> </note></p>
    #[serde(rename = "branchFilter")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub branch_filter: Option<String>,
    /// <p> An array of arrays of <code>WebhookFilter</code> objects used to determine which webhooks are triggered. At least one <code>WebhookFilter</code> in the array must specify <code>EVENT</code> as its <code>type</code>. </p> <p> For a build to be triggered, at least one filter group in the <code>filterGroups</code> array must pass. For a filter group to pass, each of its filters must pass. </p>
    #[serde(rename = "filterGroups")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter_groups: Option<Vec<Vec<WebhookFilter>>>,
    /// <p>The name of the AWS CodeBuild project.</p>
    #[serde(rename = "projectName")]
    pub project_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateWebhookOutput {
    /// <p>Information about a webhook that connects repository events to a build project in AWS CodeBuild.</p>
    #[serde(rename = "webhook")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub webhook: Option<Webhook>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteProjectInput {
    /// <p>The name of the build project.</p>
    #[serde(rename = "name")]
    pub name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DeleteProjectOutput {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteReportGroupInput {
    /// <p> The ARN of the report group to delete. </p>
    #[serde(rename = "arn")]
    pub arn: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DeleteReportGroupOutput {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteReportInput {
    /// <p> The ARN of the report to delete. </p>
    #[serde(rename = "arn")]
    pub arn: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DeleteReportOutput {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteResourcePolicyInput {
    /// <p> The ARN of the resource that is associated with the resource policy. </p>
    #[serde(rename = "resourceArn")]
    pub resource_arn: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DeleteResourcePolicyOutput {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteSourceCredentialsInput {
    /// <p> The Amazon Resource Name (ARN) of the token.</p>
    #[serde(rename = "arn")]
    pub arn: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DeleteSourceCredentialsOutput {
    /// <p> The Amazon Resource Name (ARN) of the token. </p>
    #[serde(rename = "arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteWebhookInput {
    /// <p>The name of the AWS CodeBuild project.</p>
    #[serde(rename = "projectName")]
    pub project_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DeleteWebhookOutput {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeTestCasesInput {
    /// <p> A <code>TestCaseFilter</code> object used to filter the returned reports. </p>
    #[serde(rename = "filter")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter: Option<TestCaseFilter>,
    /// <p> The maximum number of paginated test cases returned per response. Use <code>nextToken</code> to iterate pages in the list of returned <code>TestCase</code> objects. The default value is 100. </p>
    #[serde(rename = "maxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p> During a previous call, the maximum number of items that can be returned is the value specified in <code>maxResults</code>. If there more items in the list, then a unique string called a <i>nextToken</i> is returned. To get the next batch of items in the list, call this operation again, adding the next token to the call. To get all of the items in the list, keep calling this operation with each subsequent next token that is returned, until no more next tokens are returned. </p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p> The ARN of the report for which test cases are returned. </p>
    #[serde(rename = "reportArn")]
    pub report_arn: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeTestCasesOutput {
    /// <p> During a previous call, the maximum number of items that can be returned is the value specified in <code>maxResults</code>. If there more items in the list, then a unique string called a <i>nextToken</i> is returned. To get the next batch of items in the list, call this operation again, adding the next token to the call. To get all of the items in the list, keep calling this operation with each subsequent next token that is returned, until no more next tokens are returned. </p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p> The returned list of test cases. </p>
    #[serde(rename = "testCases")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub test_cases: Option<Vec<TestCase>>,
}

/// <p>Information about a Docker image that is managed by AWS CodeBuild.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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
    /// <p><p>The type of environment variable. Valid values include:</p> <ul> <li> <p> <code>PARAMETER<em>STORE</code>: An environment variable stored in Amazon EC2 Systems Manager Parameter Store.</p> </li> <li> <p> <code>PLAINTEXT</code>: An environment variable in plain text format. This is the default value.</p> </li> <li> <p> <code>SECRETS</em>MANAGER</code>: An environment variable stored in AWS Secrets Manager.</p> </li> </ul></p>
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    /// <p><p>The value of the environment variable.</p> <important> <p>We strongly discourage the use of environment variables to store sensitive values, especially AWS secret key IDs and secret access keys. Environment variables can be displayed in plain text using the AWS CodeBuild console and the AWS Command Line Interface (AWS CLI).</p> </important></p>
    #[serde(rename = "value")]
    pub value: String,
}

/// <p> Information about an exported environment variable. </p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ExportedEnvironmentVariable {
    /// <p> The name of this exported environment variable. </p>
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p><p> The value assigned to this exported environment variable. </p> <note> <p> During a build, the value of a variable is available starting with the <code>install</code> phase. It can be updated between the start of the <code>install</code> phase and the end of the <code>post<em>build</code> phase. After the <code>post</em>build</code> phase ends, the value of exported variables cannot change.</p> </note></p>
    #[serde(rename = "value")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetResourcePolicyInput {
    /// <p> The ARN of the resource that is associated with the resource policy. </p>
    #[serde(rename = "resourceArn")]
    pub resource_arn: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetResourcePolicyOutput {
    /// <p> The resource policy for the resource identified by the input ARN parameter. </p>
    #[serde(rename = "policy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy: Option<String>,
}

/// <p> Information about the Git submodules configuration for an AWS CodeBuild build project. </p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GitSubmodulesConfig {
    /// <p> Set to true to fetch Git submodules for your AWS CodeBuild build project. </p>
    #[serde(rename = "fetchSubmodules")]
    pub fetch_submodules: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ImportSourceCredentialsInput {
    /// <p> The type of authentication used to connect to a GitHub, GitHub Enterprise, or Bitbucket repository. An OAUTH connection is not supported by the API and must be created using the AWS CodeBuild console. </p>
    #[serde(rename = "authType")]
    pub auth_type: String,
    /// <p> The source provider used for this project. </p>
    #[serde(rename = "serverType")]
    pub server_type: String,
    /// <p> Set to <code>false</code> to prevent overwriting the repository source credentials. Set to <code>true</code> to overwrite the repository source credentials. The default value is <code>true</code>. </p>
    #[serde(rename = "shouldOverwrite")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub should_overwrite: Option<bool>,
    /// <p> For GitHub or GitHub Enterprise, this is the personal access token. For Bitbucket, this is the app password. </p>
    #[serde(rename = "token")]
    pub token: String,
    /// <p> The Bitbucket username when the <code>authType</code> is BASIC_AUTH. This parameter is not valid for other types of source providers or connections. </p>
    #[serde(rename = "username")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ImportSourceCredentialsOutput {
    /// <p> The Amazon Resource Name (ARN) of the token. </p>
    #[serde(rename = "arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct InvalidateProjectCacheInput {
    /// <p>The name of the AWS CodeBuild build project that the cache is reset for.</p>
    #[serde(rename = "projectName")]
    pub project_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct InvalidateProjectCacheOutput {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListBuildsForProjectInput {
    /// <p>During a previous call, if there are more than 100 items in the list, only the first 100 items are returned, along with a unique string called a <i>nextToken</i>. To get the next batch of items in the list, call this operation again, adding the next token to the call. To get all of the items in the list, keep calling this operation with each subsequent next token that is returned, until no more next tokens are returned.</p>
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListBuildsForProjectOutput {
    /// <p>A list of build IDs for the specified build project, with each build ID representing a single build.</p>
    #[serde(rename = "ids")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ids: Option<Vec<String>>,
    /// <p>If there are more than 100 items in the list, only the first 100 items are returned, along with a unique string called a <i>nextToken</i>. To get the next batch of items in the list, call this operation again, adding the next token to the call.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListBuildsInput {
    /// <p>During a previous call, if there are more than 100 items in the list, only the first 100 items are returned, along with a unique string called a <i>nextToken</i>. To get the next batch of items in the list, call this operation again, adding the next token to the call. To get all of the items in the list, keep calling this operation with each subsequent next token that is returned, until no more next tokens are returned.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p><p>The order to list build IDs. Valid values include:</p> <ul> <li> <p> <code>ASCENDING</code>: List the build IDs in ascending order by build ID.</p> </li> <li> <p> <code>DESCENDING</code>: List the build IDs in descending order by build ID.</p> </li> </ul></p>
    #[serde(rename = "sortOrder")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_order: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListBuildsOutput {
    /// <p>A list of build IDs, with each build ID representing a single build.</p>
    #[serde(rename = "ids")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ids: Option<Vec<String>>,
    /// <p>If there are more than 100 items in the list, only the first 100 items are returned, along with a unique string called a <i>nextToken</i>. To get the next batch of items in the list, call this operation again, adding the next token to the call.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListCuratedEnvironmentImagesInput {}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListCuratedEnvironmentImagesOutput {
    /// <p>Information about supported platforms for Docker images that are managed by AWS CodeBuild.</p>
    #[serde(rename = "platforms")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub platforms: Option<Vec<EnvironmentPlatform>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListProjectsInput {
    /// <p>During a previous call, if there are more than 100 items in the list, only the first 100 items are returned, along with a unique string called a <i>nextToken</i>. To get the next batch of items in the list, call this operation again, adding the next token to the call. To get all of the items in the list, keep calling this operation with each subsequent next token that is returned, until no more next tokens are returned.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The criterion to be used to list build project names. Valid values include:</p> <ul> <li> <p> <code>CREATED_TIME</code>: List based on when each build project was created.</p> </li> <li> <p> <code>LAST_MODIFIED_TIME</code>: List based on when information about each build project was last changed.</p> </li> <li> <p> <code>NAME</code>: List based on each build project's name.</p> </li> </ul> <p>Use <code>sortOrder</code> to specify in what order to list the build project names based on the preceding criteria.</p>
    #[serde(rename = "sortBy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_by: Option<String>,
    /// <p>The order in which to list build projects. Valid values include:</p> <ul> <li> <p> <code>ASCENDING</code>: List in ascending order.</p> </li> <li> <p> <code>DESCENDING</code>: List in descending order.</p> </li> </ul> <p>Use <code>sortBy</code> to specify the criterion to be used to list build project names.</p>
    #[serde(rename = "sortOrder")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_order: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListProjectsOutput {
    /// <p>If there are more than 100 items in the list, only the first 100 items are returned, along with a unique string called a <i>nextToken</i>. To get the next batch of items in the list, call this operation again, adding the next token to the call.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The list of build project names, with each build project name representing a single build project.</p>
    #[serde(rename = "projects")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub projects: Option<Vec<String>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListReportGroupsInput {
    /// <p> The maximum number of paginated report groups returned per response. Use <code>nextToken</code> to iterate pages in the list of returned <code>ReportGroup</code> objects. The default value is 100. </p>
    #[serde(rename = "maxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p> During a previous call, the maximum number of items that can be returned is the value specified in <code>maxResults</code>. If there more items in the list, then a unique string called a <i>nextToken</i> is returned. To get the next batch of items in the list, call this operation again, adding the next token to the call. To get all of the items in the list, keep calling this operation with each subsequent next token that is returned, until no more next tokens are returned. </p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p><p> The criterion to be used to list build report groups. Valid values include: </p> <ul> <li> <p> <code>CREATED<em>TIME</code>: List based on when each report group was created.</p> </li> <li> <p> <code>LAST</em>MODIFIED_TIME</code>: List based on when each report group was last changed.</p> </li> <li> <p> <code>NAME</code>: List based on each report group&#39;s name.</p> </li> </ul></p>
    #[serde(rename = "sortBy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_by: Option<String>,
    /// <p> Used to specify the order to sort the list of returned report groups. Valid values are <code>ASCENDING</code> and <code>DESCENDING</code>. </p>
    #[serde(rename = "sortOrder")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_order: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListReportGroupsOutput {
    /// <p> During a previous call, the maximum number of items that can be returned is the value specified in <code>maxResults</code>. If there more items in the list, then a unique string called a <i>nextToken</i> is returned. To get the next batch of items in the list, call this operation again, adding the next token to the call. To get all of the items in the list, keep calling this operation with each subsequent next token that is returned, until no more next tokens are returned. </p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p> The list of ARNs for the report groups in the current AWS account. </p>
    #[serde(rename = "reportGroups")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub report_groups: Option<Vec<String>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListReportsForReportGroupInput {
    /// <p> A <code>ReportFilter</code> object used to filter the returned reports. </p>
    #[serde(rename = "filter")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter: Option<ReportFilter>,
    /// <p> The maximum number of paginated reports in this report group returned per response. Use <code>nextToken</code> to iterate pages in the list of returned <code>Report</code> objects. The default value is 100. </p>
    #[serde(rename = "maxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p> During a previous call, the maximum number of items that can be returned is the value specified in <code>maxResults</code>. If there more items in the list, then a unique string called a <i>nextToken</i> is returned. To get the next batch of items in the list, call this operation again, adding the next token to the call. To get all of the items in the list, keep calling this operation with each subsequent next token that is returned, until no more next tokens are returned. </p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p> The ARN of the report group for which you want to return report ARNs. </p>
    #[serde(rename = "reportGroupArn")]
    pub report_group_arn: String,
    /// <p> Use to specify whether the results are returned in ascending or descending order. </p>
    #[serde(rename = "sortOrder")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_order: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListReportsForReportGroupOutput {
    /// <p> During a previous call, the maximum number of items that can be returned is the value specified in <code>maxResults</code>. If there more items in the list, then a unique string called a <i>nextToken</i> is returned. To get the next batch of items in the list, call this operation again, adding the next token to the call. To get all of the items in the list, keep calling this operation with each subsequent next token that is returned, until no more next tokens are returned. </p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p> The list of returned report group ARNs. </p>
    #[serde(rename = "reports")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reports: Option<Vec<String>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListReportsInput {
    /// <p> A <code>ReportFilter</code> object used to filter the returned reports. </p>
    #[serde(rename = "filter")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter: Option<ReportFilter>,
    /// <p> The maximum number of paginated reports returned per response. Use <code>nextToken</code> to iterate pages in the list of returned <code>Report</code> objects. The default value is 100. </p>
    #[serde(rename = "maxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p> During a previous call, the maximum number of items that can be returned is the value specified in <code>maxResults</code>. If there more items in the list, then a unique string called a <i>nextToken</i> is returned. To get the next batch of items in the list, call this operation again, adding the next token to the call. To get all of the items in the list, keep calling this operation with each subsequent next token that is returned, until no more next tokens are returned. </p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p><p> Specifies the sort order for the list of returned reports. Valid values are: </p> <ul> <li> <p> <code>ASCENDING</code>: return reports in chronological order based on their creation date. </p> </li> <li> <p> <code>DESCENDING</code>: return reports in the reverse chronological order based on their creation date. </p> </li> </ul></p>
    #[serde(rename = "sortOrder")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_order: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListReportsOutput {
    /// <p> During a previous call, the maximum number of items that can be returned is the value specified in <code>maxResults</code>. If there more items in the list, then a unique string called a <i>nextToken</i> is returned. To get the next batch of items in the list, call this operation again, adding the next token to the call. To get all of the items in the list, keep calling this operation with each subsequent next token that is returned, until no more next tokens are returned. </p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p> The list of returned ARNs for the reports in the current AWS account. </p>
    #[serde(rename = "reports")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reports: Option<Vec<String>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListSharedProjectsInput {
    /// <p> The maximum number of paginated shared build projects returned per response. Use <code>nextToken</code> to iterate pages in the list of returned <code>Project</code> objects. The default value is 100. </p>
    #[serde(rename = "maxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p> During a previous call, the maximum number of items that can be returned is the value specified in <code>maxResults</code>. If there more items in the list, then a unique string called a <i>nextToken</i> is returned. To get the next batch of items in the list, call this operation again, adding the next token to the call. To get all of the items in the list, keep calling this operation with each subsequent next token that is returned, until no more next tokens are returned. </p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p><p> The criterion to be used to list build projects shared with the current AWS account or user. Valid values include: </p> <ul> <li> <p> <code>ARN</code>: List based on the ARN. </p> </li> <li> <p> <code>MODIFIED_TIME</code>: List based on when information about the shared project was last changed. </p> </li> </ul></p>
    #[serde(rename = "sortBy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_by: Option<String>,
    /// <p><p>The order in which to list shared build projects. Valid values include:</p> <ul> <li> <p> <code>ASCENDING</code>: List in ascending order.</p> </li> <li> <p> <code>DESCENDING</code>: List in descending order.</p> </li> </ul></p>
    #[serde(rename = "sortOrder")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_order: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListSharedProjectsOutput {
    /// <p> During a previous call, the maximum number of items that can be returned is the value specified in <code>maxResults</code>. If there more items in the list, then a unique string called a <i>nextToken</i> is returned. To get the next batch of items in the list, call this operation again, adding the next token to the call. To get all of the items in the list, keep calling this operation with each subsequent next token that is returned, until no more next tokens are returned. </p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p> The list of ARNs for the build projects shared with the current AWS account or user. </p>
    #[serde(rename = "projects")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub projects: Option<Vec<String>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListSharedReportGroupsInput {
    /// <p> The maximum number of paginated shared report groups per response. Use <code>nextToken</code> to iterate pages in the list of returned <code>ReportGroup</code> objects. The default value is 100. </p>
    #[serde(rename = "maxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p> During a previous call, the maximum number of items that can be returned is the value specified in <code>maxResults</code>. If there more items in the list, then a unique string called a <i>nextToken</i> is returned. To get the next batch of items in the list, call this operation again, adding the next token to the call. To get all of the items in the list, keep calling this operation with each subsequent next token that is returned, until no more next tokens are returned. </p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p><p> The criterion to be used to list report groups shared with the current AWS account or user. Valid values include: </p> <ul> <li> <p> <code>ARN</code>: List based on the ARN. </p> </li> <li> <p> <code>MODIFIED_TIME</code>: List based on when information about the shared report group was last changed. </p> </li> </ul></p>
    #[serde(rename = "sortBy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_by: Option<String>,
    /// <p><p>The order in which to list shared report groups. Valid values include:</p> <ul> <li> <p> <code>ASCENDING</code>: List in ascending order.</p> </li> <li> <p> <code>DESCENDING</code>: List in descending order.</p> </li> </ul></p>
    #[serde(rename = "sortOrder")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_order: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListSharedReportGroupsOutput {
    /// <p> During a previous call, the maximum number of items that can be returned is the value specified in <code>maxResults</code>. If there more items in the list, then a unique string called a <i>nextToken</i> is returned. To get the next batch of items in the list, call this operation again, adding the next token to the call. To get all of the items in the list, keep calling this operation with each subsequent next token that is returned, until no more next tokens are returned. </p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p> The list of ARNs for the report groups shared with the current AWS account or user. </p>
    #[serde(rename = "reportGroups")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub report_groups: Option<Vec<String>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListSourceCredentialsInput {}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListSourceCredentialsOutput {
    /// <p> A list of <code>SourceCredentialsInfo</code> objects. Each <code>SourceCredentialsInfo</code> object includes the authentication type, token ARN, and type of source provider for one set of credentials. </p>
    #[serde(rename = "sourceCredentialsInfos")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_credentials_infos: Option<Vec<SourceCredentialsInfo>>,
}

/// <p> Information about logs for a build project. These can be logs in Amazon CloudWatch Logs, built in a specified S3 bucket, or both. </p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LogsConfig {
    /// <p> Information about Amazon CloudWatch Logs for a build project. Amazon CloudWatch Logs are enabled by default. </p>
    #[serde(rename = "cloudWatchLogs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_watch_logs: Option<CloudWatchLogsConfig>,
    /// <p> Information about logs built to an S3 bucket for a build project. S3 logs are not enabled by default. </p>
    #[serde(rename = "s3Logs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s_3_logs: Option<S3LogsConfig>,
}

/// <p>Information about build logs in Amazon CloudWatch Logs.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct LogsLocation {
    /// <p> Information about Amazon CloudWatch Logs for a build project. </p>
    #[serde(rename = "cloudWatchLogs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_watch_logs: Option<CloudWatchLogsConfig>,
    /// <p> The ARN of Amazon CloudWatch Logs for a build project. Its format is <code>arn:${Partition}:logs:${Region}:${Account}:log-group:${LogGroupName}:log-stream:${LogStreamName}</code>. For more information, see <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/list_amazoncloudwatchlogs.html#amazoncloudwatchlogs-resources-for-iam-policies">Resources Defined by Amazon CloudWatch Logs</a>. </p>
    #[serde(rename = "cloudWatchLogsArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_watch_logs_arn: Option<String>,
    /// <p>The URL to an individual build log in Amazon CloudWatch Logs.</p>
    #[serde(rename = "deepLink")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deep_link: Option<String>,
    /// <p>The name of the Amazon CloudWatch Logs group for the build logs.</p>
    #[serde(rename = "groupName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_name: Option<String>,
    /// <p> The URL to a build log in an S3 bucket. </p>
    #[serde(rename = "s3DeepLink")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s_3_deep_link: Option<String>,
    /// <p> Information about S3 logs for a build project. </p>
    #[serde(rename = "s3Logs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s_3_logs: Option<S3LogsConfig>,
    /// <p> The ARN of S3 logs for a build project. Its format is <code>arn:${Partition}:s3:::${BucketName}/${ObjectName}</code>. For more information, see <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/list_amazons3.html#amazons3-resources-for-iam-policies">Resources Defined by Amazon S3</a>. </p>
    #[serde(rename = "s3LogsArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s_3_logs_arn: Option<String>,
    /// <p>The name of the Amazon CloudWatch Logs stream for the build logs.</p>
    #[serde(rename = "streamName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream_name: Option<String>,
}

/// <p>Describes a network interface.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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

/// <p>Additional information about a build phase that has an error. You can use this information for troubleshooting.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct PhaseContext {
    /// <p>An explanation of the build phase's context. This might include a command ID and an exit code.</p>
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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
    /// <p>The AWS Key Management Service (AWS KMS) customer master key (CMK) to be used for encrypting the build output artifacts.</p> <note> <p> You can use a cross-account KMS key to encrypt the build output artifacts if your service role has permission to that key. </p> </note> <p>You can specify either the Amazon Resource Name (ARN) of the CMK or, if available, the CMK's alias (using the format <code>alias/<i>alias-name</i> </code>).</p>
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
    /// <p> Information about logs for the build project. A project can create logs in Amazon CloudWatch Logs, an S3 bucket, or both. </p>
    #[serde(rename = "logsConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logs_config: Option<LogsConfig>,
    /// <p>The name of the build project.</p>
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p> The number of minutes a build is allowed to be queued before it times out. </p>
    #[serde(rename = "queuedTimeoutInMinutes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub queued_timeout_in_minutes: Option<i64>,
    /// <p> An array of <code>ProjectArtifacts</code> objects. </p>
    #[serde(rename = "secondaryArtifacts")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secondary_artifacts: Option<Vec<ProjectArtifacts>>,
    /// <p> An array of <code>ProjectSourceVersion</code> objects. If <code>secondarySourceVersions</code> is specified at the build level, then they take over these <code>secondarySourceVersions</code> (at the project level). </p>
    #[serde(rename = "secondarySourceVersions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secondary_source_versions: Option<Vec<ProjectSourceVersion>>,
    /// <p> An array of <code>ProjectSource</code> objects. </p>
    #[serde(rename = "secondarySources")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secondary_sources: Option<Vec<ProjectSource>>,
    /// <p>The ARN of the AWS Identity and Access Management (IAM) role that enables AWS CodeBuild to interact with dependent AWS services on behalf of the AWS account.</p>
    #[serde(rename = "serviceRole")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_role: Option<String>,
    /// <p>Information about the build input source code for this build project.</p>
    #[serde(rename = "source")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<ProjectSource>,
    /// <p>A version of the build input to be built for this project. If not specified, the latest version is used. If specified, it must be one of:</p> <ul> <li> <p>For AWS CodeCommit: the commit ID, branch, or Git tag to use.</p> </li> <li> <p>For GitHub: the commit ID, pull request ID, branch name, or tag name that corresponds to the version of the source code you want to build. If a pull request ID is specified, it must use the format <code>pr/pull-request-ID</code> (for example <code>pr/25</code>). If a branch name is specified, the branch's HEAD commit ID is used. If not specified, the default branch's HEAD commit ID is used.</p> </li> <li> <p>For Bitbucket: the commit ID, branch name, or tag name that corresponds to the version of the source code you want to build. If a branch name is specified, the branch's HEAD commit ID is used. If not specified, the default branch's HEAD commit ID is used.</p> </li> <li> <p>For Amazon Simple Storage Service (Amazon S3): the version ID of the object that represents the build input ZIP file to use.</p> </li> </ul> <p> If <code>sourceVersion</code> is specified at the build level, then that version takes precedence over this <code>sourceVersion</code> (at the project level). </p> <p> For more information, see <a href="https://docs.aws.amazon.com/codebuild/latest/userguide/sample-source-version.html">Source Version Sample with CodeBuild</a> in the <i>AWS CodeBuild User Guide</i>. </p>
    #[serde(rename = "sourceVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_version: Option<String>,
    /// <p>The tags for this build project.</p> <p>These tags are available for use by AWS services that support AWS CodeBuild build project tags.</p>
    #[serde(rename = "tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
    /// <p>How long, in minutes, from 5 to 480 (8 hours), for AWS CodeBuild to wait before timing out any related build that did not get marked as completed. The default is 60 minutes.</p>
    #[serde(rename = "timeoutInMinutes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeout_in_minutes: Option<i64>,
    /// <p>Information about the VPC configuration that AWS CodeBuild accesses.</p>
    #[serde(rename = "vpcConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_config: Option<VpcConfig>,
    /// <p>Information about a webhook that connects repository events to a build project in AWS CodeBuild.</p>
    #[serde(rename = "webhook")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub webhook: Option<Webhook>,
}

/// <p>Information about the build output artifacts for the build project.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ProjectArtifacts {
    /// <p> An identifier for this artifact definition. </p>
    #[serde(rename = "artifactIdentifier")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub artifact_identifier: Option<String>,
    /// <p> Set to true if you do not want your output artifacts encrypted. This option is valid only if your artifacts type is Amazon Simple Storage Service (Amazon S3). If this is set with another artifacts type, an invalidInputException is thrown. </p>
    #[serde(rename = "encryptionDisabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encryption_disabled: Option<bool>,
    /// <p><p>Information about the build output artifact location:</p> <ul> <li> <p>If <code>type</code> is set to <code>CODEPIPELINE</code>, AWS CodePipeline ignores this value if specified. This is because AWS CodePipeline manages its build output locations instead of AWS CodeBuild.</p> </li> <li> <p>If <code>type</code> is set to <code>NO_ARTIFACTS</code>, this value is ignored if specified, because no build output is produced.</p> </li> <li> <p>If <code>type</code> is set to <code>S3</code>, this is the name of the output bucket.</p> </li> </ul></p>
    #[serde(rename = "location")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    /// <p><p>Along with <code>path</code> and <code>namespaceType</code>, the pattern that AWS CodeBuild uses to name and store the output artifact:</p> <ul> <li> <p>If <code>type</code> is set to <code>CODEPIPELINE</code>, AWS CodePipeline ignores this value if specified. This is because AWS CodePipeline manages its build output names instead of AWS CodeBuild.</p> </li> <li> <p>If <code>type</code> is set to <code>NO<em>ARTIFACTS</code>, this value is ignored if specified, because no build output is produced.</p> </li> <li> <p>If <code>type</code> is set to <code>S3</code>, this is the name of the output artifact object. If you set the name to be a forward slash (&quot;/&quot;), the artifact is stored in the root of the output bucket.</p> </li> </ul> <p>For example:</p> <ul> <li> <p> If <code>path</code> is set to <code>MyArtifacts</code>, <code>namespaceType</code> is set to <code>BUILD</em>ID</code>, and <code>name</code> is set to <code>MyArtifact.zip</code>, then the output artifact is stored in <code>MyArtifacts/<i>build-ID</i>/MyArtifact.zip</code>. </p> </li> <li> <p> If <code>path</code> is empty, <code>namespaceType</code> is set to <code>NONE</code>, and <code>name</code> is set to &quot;<code>/</code>&quot;, the output artifact is stored in the root of the output bucket. </p> </li> <li> <p> If <code>path</code> is set to <code>MyArtifacts</code>, <code>namespaceType</code> is set to <code>BUILD_ID</code>, and <code>name</code> is set to &quot;<code>/</code>&quot;, the output artifact is stored in <code>MyArtifacts/<i>build-ID</i> </code>. </p> </li> </ul></p>
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>Along with <code>path</code> and <code>name</code>, the pattern that AWS CodeBuild uses to determine the name and location to store the output artifact:</p> <ul> <li> <p>If <code>type</code> is set to <code>CODEPIPELINE</code>, AWS CodePipeline ignores this value if specified. This is because AWS CodePipeline manages its build output names instead of AWS CodeBuild.</p> </li> <li> <p>If <code>type</code> is set to <code>NO_ARTIFACTS</code>, this value is ignored if specified, because no build output is produced.</p> </li> <li> <p>If <code>type</code> is set to <code>S3</code>, valid values include:</p> <ul> <li> <p> <code>BUILD_ID</code>: Include the build ID in the location of the build output artifact.</p> </li> <li> <p> <code>NONE</code>: Do not include the build ID. This is the default if <code>namespaceType</code> is not specified.</p> </li> </ul> </li> </ul> <p>For example, if <code>path</code> is set to <code>MyArtifacts</code>, <code>namespaceType</code> is set to <code>BUILD_ID</code>, and <code>name</code> is set to <code>MyArtifact.zip</code>, the output artifact is stored in <code>MyArtifacts/<i>build-ID</i>/MyArtifact.zip</code>.</p>
    #[serde(rename = "namespaceType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub namespace_type: Option<String>,
    /// <p> If this flag is set, a name specified in the build spec file overrides the artifact name. The name specified in a build spec file is calculated at build time and uses the Shell Command Language. For example, you can append a date and time to your artifact name so that it is always unique. </p>
    #[serde(rename = "overrideArtifactName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub override_artifact_name: Option<bool>,
    /// <p><p>The type of build output artifact to create:</p> <ul> <li> <p>If <code>type</code> is set to <code>CODEPIPELINE</code>, AWS CodePipeline ignores this value if specified. This is because AWS CodePipeline manages its build output artifacts instead of AWS CodeBuild.</p> </li> <li> <p>If <code>type</code> is set to <code>NO_ARTIFACTS</code>, this value is ignored if specified, because no build output is produced.</p> </li> <li> <p>If <code>type</code> is set to <code>S3</code>, valid values include:</p> <ul> <li> <p> <code>NONE</code>: AWS CodeBuild creates in the output bucket a folder that contains the build output. This is the default if <code>packaging</code> is not specified.</p> </li> <li> <p> <code>ZIP</code>: AWS CodeBuild creates in the output bucket a ZIP file that contains the build output.</p> </li> </ul> </li> </ul></p>
    #[serde(rename = "packaging")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub packaging: Option<String>,
    /// <p>Along with <code>namespaceType</code> and <code>name</code>, the pattern that AWS CodeBuild uses to name and store the output artifact:</p> <ul> <li> <p>If <code>type</code> is set to <code>CODEPIPELINE</code>, AWS CodePipeline ignores this value if specified. This is because AWS CodePipeline manages its build output names instead of AWS CodeBuild.</p> </li> <li> <p>If <code>type</code> is set to <code>NO_ARTIFACTS</code>, this value is ignored if specified, because no build output is produced.</p> </li> <li> <p>If <code>type</code> is set to <code>S3</code>, this is the path to the output artifact. If <code>path</code> is not specified, <code>path</code> is not used.</p> </li> </ul> <p>For example, if <code>path</code> is set to <code>MyArtifacts</code>, <code>namespaceType</code> is set to <code>NONE</code>, and <code>name</code> is set to <code>MyArtifact.zip</code>, the output artifact is stored in the output bucket at <code>MyArtifacts/MyArtifact.zip</code>.</p>
    #[serde(rename = "path")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    /// <p><p>The type of build output artifact. Valid values include:</p> <ul> <li> <p> <code>CODEPIPELINE</code>: The build project has build output generated through AWS CodePipeline. </p> <note> <p>The <code>CODEPIPELINE</code> type is not supported for <code>secondaryArtifacts</code>.</p> </note> </li> <li> <p> <code>NO_ARTIFACTS</code>: The build project does not produce any build output.</p> </li> <li> <p> <code>S3</code>: The build project stores build output in Amazon Simple Storage Service (Amazon S3).</p> </li> </ul></p>
    #[serde(rename = "type")]
    pub type_: String,
}

/// <p>Information about the build badge for the build project.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ProjectBadge {
    /// <p>Set this to true to generate a publicly accessible URL for your project's build badge.</p>
    #[serde(rename = "badgeEnabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub badge_enabled: Option<bool>,
    /// <p>The publicly-accessible URL through which you can access the build badge for your project. </p> <p>The publicly accessible URL through which you can access the build badge for your project. </p>
    #[serde(rename = "badgeRequestUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub badge_request_url: Option<String>,
}

/// <p>Information about the cache for the build project.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ProjectCache {
    /// <p><p>Information about the cache location: </p> <ul> <li> <p> <code>NO_CACHE</code> or <code>LOCAL</code>: This value is ignored.</p> </li> <li> <p> <code>S3</code>: This is the S3 bucket name/prefix.</p> </li> </ul></p>
    #[serde(rename = "location")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    /// <p><p> If you use a <code>LOCAL</code> cache, the local cache mode. You can use one or more local cache modes at the same time. </p> <ul> <li> <p> <code>LOCAL<em>SOURCE</em>CACHE</code> mode caches Git metadata for primary and secondary sources. After the cache is created, subsequent builds pull only the change between commits. This mode is a good choice for projects with a clean working directory and a source that is a large Git repository. If you choose this option and your project does not use a Git repository (GitHub, GitHub Enterprise, or Bitbucket), the option is ignored. </p> </li> <li> <p> <code>LOCAL<em>DOCKER</em>LAYER<em>CACHE</code> mode caches existing Docker layers. This mode is a good choice for projects that build or pull large Docker images. It can prevent the performance issues caused by pulling large Docker images down from the network. </p> <note> <ul> <li> <p> You can use a Docker layer cache in the Linux environment only. </p> </li> <li> <p> The <code>privileged</code> flag must be set so that your project has the required Docker permissions. </p> </li> <li> <p> You should consider the security implications before you use a Docker layer cache. </p> </li> </ul> </note> </li> </ul> <ul> <li> <p> <code>LOCAL</em>CUSTOM_CACHE</code> mode caches directories you specify in the buildspec file. This mode is a good choice if your build scenario is not suited to one of the other three local cache modes. If you use a custom cache: </p> <ul> <li> <p> Only directories can be specified for caching. You cannot specify individual files. </p> </li> <li> <p> Symlinks are used to reference cached directories. </p> </li> <li> <p> Cached directories are linked to your build before it downloads its project sources. Cached items are overridden if a source item has the same name. Directories are specified using cache paths in the buildspec file. </p> </li> </ul> </li> </ul></p>
    #[serde(rename = "modes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modes: Option<Vec<String>>,
    /// <p><p>The type of cache used by the build project. Valid values include:</p> <ul> <li> <p> <code>NO_CACHE</code>: The build project does not use any cache.</p> </li> <li> <p> <code>S3</code>: The build project reads and writes from and to S3.</p> </li> <li> <p> <code>LOCAL</code>: The build project stores a cache locally on a build host that is only available to that build host.</p> </li> </ul></p>
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
    /// <p>Information about the compute resources the build project uses. Available values include:</p> <ul> <li> <p> <code>BUILD_GENERAL1_SMALL</code>: Use up to 3 GB memory and 2 vCPUs for builds.</p> </li> <li> <p> <code>BUILD_GENERAL1_MEDIUM</code>: Use up to 7 GB memory and 4 vCPUs for builds.</p> </li> <li> <p> <code>BUILD_GENERAL1_LARGE</code>: Use up to 16 GB memory and 8 vCPUs for builds, depending on your environment type.</p> </li> <li> <p> <code>BUILD_GENERAL1_2XLARGE</code>: Use up to 145 GB memory, 72 vCPUs, and 824 GB of SSD storage for builds. This compute type supports Docker images up to 100 GB uncompressed.</p> </li> </ul> <p> If you use <code>BUILD_GENERAL1_LARGE</code>: </p> <ul> <li> <p> For environment type <code>LINUX_CONTAINER</code>, you can use up to 15 GB memory and 8 vCPUs for builds. </p> </li> <li> <p> For environment type <code>LINUX_GPU_CONTAINER</code>, you can use up to 255 GB memory, 32 vCPUs, and 4 NVIDIA Tesla V100 GPUs for builds.</p> </li> <li> <p> For environment type <code>ARM_CONTAINER</code>, you can use up to 16 GB memory and 8 vCPUs on ARM-based processors for builds.</p> </li> </ul> <p> For more information, see <a href="https://docs.aws.amazon.com/codebuild/latest/userguide/build-env-ref-compute-types.html">Build Environment Compute Types</a> in the <i>AWS CodeBuild User Guide.</i> </p>
    #[serde(rename = "computeType")]
    pub compute_type: String,
    /// <p>A set of environment variables to make available to builds for this build project.</p>
    #[serde(rename = "environmentVariables")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub environment_variables: Option<Vec<EnvironmentVariable>>,
    /// <p><p>The image tag or image digest that identifies the Docker image to use for this build project. Use the following formats:</p> <ul> <li> <p>For an image tag: <code>registry/repository:tag</code>. For example, to specify an image with the tag &quot;latest,&quot; use <code>registry/repository:latest</code>.</p> </li> <li> <p>For an image digest: <code>registry/repository@digest</code>. For example, to specify an image with the digest &quot;sha256:cbbf2f9a99b47fc460d422812b6a5adff7dfee951d8fa2e4a98caa0382cfbdbf,&quot; use <code>registry/repository@sha256:cbbf2f9a99b47fc460d422812b6a5adff7dfee951d8fa2e4a98caa0382cfbdbf</code>.</p> </li> </ul></p>
    #[serde(rename = "image")]
    pub image: String,
    /// <p> The type of credentials AWS CodeBuild uses to pull images in your build. There are two valid values: </p> <ul> <li> <p> <code>CODEBUILD</code> specifies that AWS CodeBuild uses its own credentials. This requires that you modify your ECR repository policy to trust AWS CodeBuild's service principal. </p> </li> <li> <p> <code>SERVICE_ROLE</code> specifies that AWS CodeBuild uses your build project's service role. </p> </li> </ul> <p> When you use a cross-account or private registry image, you must use SERVICE_ROLE credentials. When you use an AWS CodeBuild curated image, you must use CODEBUILD credentials. </p>
    #[serde(rename = "imagePullCredentialsType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_pull_credentials_type: Option<String>,
    /// <p>Enables running the Docker daemon inside a Docker container. Set to true only if the build project is used to build Docker images. Otherwise, a build that attempts to interact with the Docker daemon fails. The default setting is <code>false</code>.</p> <p>You can initialize the Docker daemon during the install phase of your build by adding one of the following sets of commands to the install phase of your buildspec file:</p> <p>If the operating system's base image is Ubuntu Linux:</p> <p> <code>- nohup /usr/local/bin/dockerd --host=unix:///var/run/docker.sock --host=tcp://0.0.0.0:2375 --storage-driver=overlay&amp;</code> </p> <p> <code>- timeout 15 sh -c "until docker info; do echo .; sleep 1; done"</code> </p> <p>If the operating system's base image is Alpine Linux and the previous command does not work, add the <code>-t</code> argument to <code>timeout</code>:</p> <p> <code>- nohup /usr/local/bin/dockerd --host=unix:///var/run/docker.sock --host=tcp://0.0.0.0:2375 --storage-driver=overlay&amp;</code> </p> <p> <code>- timeout -t 15 sh -c "until docker info; do echo .; sleep 1; done"</code> </p>
    #[serde(rename = "privilegedMode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub privileged_mode: Option<bool>,
    /// <p> The credentials for access to a private registry.</p>
    #[serde(rename = "registryCredential")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub registry_credential: Option<RegistryCredential>,
    /// <p><p>The type of build environment to use for related builds.</p> <ul> <li> <p>The environment type <code>ARM<em>CONTAINER</code> is available only in regions US East (N. Virginia), US East (Ohio), US West (Oregon), EU (Ireland), Asia Pacific (Mumbai), Asia Pacific (Tokyo), Asia Pacific (Sydney), and EU (Frankfurt).</p> </li> <li> <p>The environment type <code>LINUX</em>CONTAINER</code> with compute type <code>build.general1.2xlarge</code> is available only in regions US East (N. Virginia), US East (N. Virginia), US West (Oregon), Canada (Central), EU (Ireland), EU (London), EU (Frankfurt), Asia Pacific (Tokyo), Asia Pacific (Seoul), Asia Pacific (Singapore), Asia Pacific (Sydney), China (Beijing), and China (Ningxia).</p> </li> <li> <p>The environment type <code>LINUX<em>GPU</em>CONTAINER</code> is available only in regions US East (N. Virginia), US East (N. Virginia), US West (Oregon), Canada (Central), EU (Ireland), EU (London), EU (Frankfurt), Asia Pacific (Tokyo), Asia Pacific (Seoul), Asia Pacific (Singapore), Asia Pacific (Sydney) , China (Beijing), and China (Ningxia).</p> </li> </ul></p>
    #[serde(rename = "type")]
    pub type_: String,
}

/// <p>Information about the build input source code for the build project.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ProjectSource {
    /// <p>Information about the authorization settings for AWS CodeBuild to access the source code to be built.</p> <p>This information is for the AWS CodeBuild console's use only. Your code should not get or set this information directly.</p>
    #[serde(rename = "auth")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auth: Option<SourceAuth>,
    /// <p>The build spec declaration to use for the builds in this build project.</p> <p>If this value is not specified, a build spec must be included along with the source code to be built.</p>
    #[serde(rename = "buildspec")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub buildspec: Option<String>,
    /// <p>Information about the Git clone depth for the build project.</p>
    #[serde(rename = "gitCloneDepth")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub git_clone_depth: Option<i64>,
    /// <p> Information about the Git submodules configuration for the build project. </p>
    #[serde(rename = "gitSubmodulesConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub git_submodules_config: Option<GitSubmodulesConfig>,
    /// <p>Enable this flag to ignore SSL warnings while connecting to the project source code.</p>
    #[serde(rename = "insecureSsl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub insecure_ssl: Option<bool>,
    /// <p><p>Information about the location of the source code to be built. Valid values include:</p> <ul> <li> <p>For source code settings that are specified in the source action of a pipeline in AWS CodePipeline, <code>location</code> should not be specified. If it is specified, AWS CodePipeline ignores it. This is because AWS CodePipeline uses the settings in a pipeline&#39;s source action instead of this value.</p> </li> <li> <p>For source code in an AWS CodeCommit repository, the HTTPS clone URL to the repository that contains the source code and the build spec (for example, <code>https://git-codecommit.<i>region-ID</i>.amazonaws.com/v1/repos/<i>repo-name</i> </code>).</p> </li> <li> <p>For source code in an Amazon Simple Storage Service (Amazon S3) input bucket, one of the following. </p> <ul> <li> <p> The path to the ZIP file that contains the source code (for example, <code> <i>bucket-name</i>/<i>path</i>/<i>to</i>/<i>object-name</i>.zip</code>). </p> </li> <li> <p> The path to the folder that contains the source code (for example, <code> <i>bucket-name</i>/<i>path</i>/<i>to</i>/<i>source-code</i>/<i>folder</i>/</code>). </p> </li> </ul> </li> <li> <p>For source code in a GitHub repository, the HTTPS clone URL to the repository that contains the source and the build spec. You must connect your AWS account to your GitHub account. Use the AWS CodeBuild console to start creating a build project. When you use the console to connect (or reconnect) with GitHub, on the GitHub <b>Authorize application</b> page, for <b>Organization access</b>, choose <b>Request access</b> next to each repository you want to allow AWS CodeBuild to have access to, and then choose <b>Authorize application</b>. (After you have connected to your GitHub account, you do not need to finish creating the build project. You can leave the AWS CodeBuild console.) To instruct AWS CodeBuild to use this connection, in the <code>source</code> object, set the <code>auth</code> object&#39;s <code>type</code> value to <code>OAUTH</code>.</p> </li> <li> <p>For source code in a Bitbucket repository, the HTTPS clone URL to the repository that contains the source and the build spec. You must connect your AWS account to your Bitbucket account. Use the AWS CodeBuild console to start creating a build project. When you use the console to connect (or reconnect) with Bitbucket, on the Bitbucket <b>Confirm access to your account</b> page, choose <b>Grant access</b>. (After you have connected to your Bitbucket account, you do not need to finish creating the build project. You can leave the AWS CodeBuild console.) To instruct AWS CodeBuild to use this connection, in the <code>source</code> object, set the <code>auth</code> object&#39;s <code>type</code> value to <code>OAUTH</code>.</p> </li> </ul></p>
    #[serde(rename = "location")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    /// <p><p> Set to true to report the status of a build&#39;s start and finish to your source provider. This option is valid only when your source provider is GitHub, GitHub Enterprise, or Bitbucket. If this is set and you use a different source provider, an invalidInputException is thrown. </p> <note> <p> The status of a build triggered by a webhook is always reported to your source provider. </p> </note></p>
    #[serde(rename = "reportBuildStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub report_build_status: Option<bool>,
    /// <p> An identifier for this project source. </p>
    #[serde(rename = "sourceIdentifier")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_identifier: Option<String>,
    /// <p><p>The type of repository that contains the source code to be built. Valid values include:</p> <ul> <li> <p> <code>BITBUCKET</code>: The source code is in a Bitbucket repository.</p> </li> <li> <p> <code>CODECOMMIT</code>: The source code is in an AWS CodeCommit repository.</p> </li> <li> <p> <code>CODEPIPELINE</code>: The source code settings are specified in the source action of a pipeline in AWS CodePipeline.</p> </li> <li> <p> <code>GITHUB</code>: The source code is in a GitHub repository.</p> </li> <li> <p> <code>GITHUB<em>ENTERPRISE</code>: The source code is in a GitHub Enterprise repository.</p> </li> <li> <p> <code>NO</em>SOURCE</code>: The project does not have input source code.</p> </li> <li> <p> <code>S3</code>: The source code is in an Amazon Simple Storage Service (Amazon S3) input bucket.</p> </li> </ul></p>
    #[serde(rename = "type")]
    pub type_: String,
}

/// <p> A source identifier and its corresponding version. </p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ProjectSourceVersion {
    /// <p>An identifier for a source in the build project.</p>
    #[serde(rename = "sourceIdentifier")]
    pub source_identifier: String,
    /// <p>The source version for the corresponding source identifier. If specified, must be one of:</p> <ul> <li> <p>For AWS CodeCommit: the commit ID, branch, or Git tag to use.</p> </li> <li> <p>For GitHub: the commit ID, pull request ID, branch name, or tag name that corresponds to the version of the source code you want to build. If a pull request ID is specified, it must use the format <code>pr/pull-request-ID</code> (for example, <code>pr/25</code>). If a branch name is specified, the branch's HEAD commit ID is used. If not specified, the default branch's HEAD commit ID is used.</p> </li> <li> <p>For Bitbucket: the commit ID, branch name, or tag name that corresponds to the version of the source code you want to build. If a branch name is specified, the branch's HEAD commit ID is used. If not specified, the default branch's HEAD commit ID is used.</p> </li> <li> <p>For Amazon Simple Storage Service (Amazon S3): the version ID of the object that represents the build input ZIP file to use.</p> </li> </ul> <p> For more information, see <a href="https://docs.aws.amazon.com/codebuild/latest/userguide/sample-source-version.html">Source Version Sample with CodeBuild</a> in the <i>AWS CodeBuild User Guide</i>. </p>
    #[serde(rename = "sourceVersion")]
    pub source_version: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct PutResourcePolicyInput {
    /// <p> A JSON-formatted resource policy. For more information, see <a href="https://docs.aws.amazon.com/codebuild/latest/userguide/project-sharing.html#project-sharing-share">Sharing a Project</a> and <a href="https://docs.aws.amazon.com/codebuild/latest/userguide/report-groups-sharing.html#report-groups-sharing-share">Sharing a Report Group</a> in the <i>AWS CodeBuild User Guide</i>. </p>
    #[serde(rename = "policy")]
    pub policy: String,
    /// <p> The ARN of the <code>Project</code> or <code>ReportGroup</code> resource you want to associate with a resource policy. </p>
    #[serde(rename = "resourceArn")]
    pub resource_arn: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct PutResourcePolicyOutput {
    /// <p> The ARN of the <code>Project</code> or <code>ReportGroup</code> resource that is associated with a resource policy. </p>
    #[serde(rename = "resourceArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_arn: Option<String>,
}

/// <p> Information about credentials that provide access to a private Docker registry. When this is set: </p> <ul> <li> <p> <code>imagePullCredentialsType</code> must be set to <code>SERVICE_ROLE</code>. </p> </li> <li> <p> images cannot be curated or an Amazon ECR image.</p> </li> </ul> <p> For more information, see <a href="https://docs.aws.amazon.com/codebuild/latest/userguide/sample-private-registry.html">Private Registry with AWS Secrets Manager Sample for AWS CodeBuild</a>. </p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RegistryCredential {
    /// <p><p> The Amazon Resource Name (ARN) or name of credentials created using AWS Secrets Manager. </p> <note> <p> The <code>credential</code> can use the name of the credentials only if they exist in your current region. </p> </note></p>
    #[serde(rename = "credential")]
    pub credential: String,
    /// <p> The service that created the credentials to access a private Docker registry. The valid value, SECRETS_MANAGER, is for AWS Secrets Manager. </p>
    #[serde(rename = "credentialProvider")]
    pub credential_provider: String,
}

/// <p> Information about the results from running a series of test cases during the run of a build project. The test cases are specified in the buildspec for the build project using one or more paths to the test case files. You can specify any type of tests you want, such as unit tests, integration tests, and functional tests. </p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct Report {
    /// <p> The ARN of the report run. </p>
    #[serde(rename = "arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p> The date and time this report run occurred. </p>
    #[serde(rename = "created")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created: Option<f64>,
    /// <p> The ARN of the build run that generated this report. </p>
    #[serde(rename = "executionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub execution_id: Option<String>,
    /// <p> The date and time a report expires. A report expires 30 days after it is created. An expired report is not available to view in CodeBuild. </p>
    #[serde(rename = "expired")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expired: Option<f64>,
    /// <p> Information about where the raw data used to generate this report was exported. </p>
    #[serde(rename = "exportConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub export_config: Option<ReportExportConfig>,
    /// <p> The name of the report that was run. </p>
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p> The ARN of the report group associated with this report. </p>
    #[serde(rename = "reportGroupArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub report_group_arn: Option<String>,
    /// <p> The status of this report. </p>
    #[serde(rename = "status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// <p> A <code>TestReportSummary</code> object that contains information about this test report. </p>
    #[serde(rename = "testSummary")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub test_summary: Option<TestReportSummary>,
    /// <p> A boolean that specifies if this report run is truncated. The list of test cases is truncated after the maximum number of test cases is reached. </p>
    #[serde(rename = "truncated")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub truncated: Option<bool>,
    /// <p> The type of the report that was run. </p>
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}

/// <p> Information about the location where the run of a report is exported. </p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ReportExportConfig {
    /// <p><p> The export configuration type. Valid values are: </p> <ul> <li> <p> <code>S3</code>: The report results are exported to an S3 bucket. </p> </li> <li> <p> <code>NO_EXPORT</code>: The report results are not exported. </p> </li> </ul></p>
    #[serde(rename = "exportConfigType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub export_config_type: Option<String>,
    /// <p> A <code>S3ReportExportConfig</code> object that contains information about the S3 bucket where the run of a report is exported. </p>
    #[serde(rename = "s3Destination")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s_3_destination: Option<S3ReportExportConfig>,
}

/// <p> A filter used to return reports with the status specified by the input <code>status</code> parameter. </p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ReportFilter {
    /// <p> The status used to filter reports. You can filter using one status only. </p>
    #[serde(rename = "status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

/// <p> A series of reports. Each report contains information about the results from running a series of test cases. You specify the test cases for a report group in the buildspec for a build project using one or more paths to the test case files. </p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ReportGroup {
    /// <p> The ARN of a <code>ReportGroup</code>. </p>
    #[serde(rename = "arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p> The date and time this <code>ReportGroup</code> was created. </p>
    #[serde(rename = "created")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created: Option<f64>,
    /// <p> Information about the destination where the raw data of this <code>ReportGroup</code> is exported. </p>
    #[serde(rename = "exportConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub export_config: Option<ReportExportConfig>,
    /// <p> The date and time this <code>ReportGroup</code> was last modified. </p>
    #[serde(rename = "lastModified")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified: Option<f64>,
    /// <p> The name of a <code>ReportGroup</code>. </p>
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p> The type of the <code>ReportGroup</code>. The one valid value is <code>TEST</code>. </p>
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}

/// <p> Information about S3 logs for a build project. </p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct S3LogsConfig {
    /// <p> Set to true if you do not want your S3 build log output encrypted. By default S3 build logs are encrypted. </p>
    #[serde(rename = "encryptionDisabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encryption_disabled: Option<bool>,
    /// <p> The ARN of an S3 bucket and the path prefix for S3 logs. If your Amazon S3 bucket name is <code>my-bucket</code>, and your path prefix is <code>build-log</code>, then acceptable formats are <code>my-bucket/build-log</code> or <code>arn:aws:s3:::my-bucket/build-log</code>. </p>
    #[serde(rename = "location")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    /// <p><p>The current status of the S3 build logs. Valid values are:</p> <ul> <li> <p> <code>ENABLED</code>: S3 build logs are enabled for this build project.</p> </li> <li> <p> <code>DISABLED</code>: S3 build logs are not enabled for this build project.</p> </li> </ul></p>
    #[serde(rename = "status")]
    pub status: String,
}

/// <p> Information about the S3 bucket where the raw data of a report are exported. </p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct S3ReportExportConfig {
    /// <p> The name of the S3 bucket where the raw data of a report are exported. </p>
    #[serde(rename = "bucket")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bucket: Option<String>,
    /// <p> A boolean value that specifies if the results of a report are encrypted. </p>
    #[serde(rename = "encryptionDisabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encryption_disabled: Option<bool>,
    /// <p> The encryption key for the report's encrypted raw data. </p>
    #[serde(rename = "encryptionKey")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encryption_key: Option<String>,
    /// <p><p> The type of build output artifact to create. Valid values include: </p> <ul> <li> <p> <code>NONE</code>: AWS CodeBuild creates the raw data in the output bucket. This is the default if packaging is not specified. </p> </li> <li> <p> <code>ZIP</code>: AWS CodeBuild creates a ZIP file with the raw data in the output bucket. </p> </li> </ul></p>
    #[serde(rename = "packaging")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub packaging: Option<String>,
    /// <p> The path to the exported report's raw data results. </p>
    #[serde(rename = "path")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
}

/// <p>Information about the authorization settings for AWS CodeBuild to access the source code to be built.</p> <p>This information is for the AWS CodeBuild console's use only. Your code should not get or set this information directly.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SourceAuth {
    /// <p>The resource value that applies to the specified authorization type.</p>
    #[serde(rename = "resource")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource: Option<String>,
    /// <p><note> <p> This data type is deprecated and is no longer accurate or used. </p> </note> <p>The authorization type to use. The only valid value is <code>OAUTH</code>, which represents the OAuth authorization type.</p></p>
    #[serde(rename = "type")]
    pub type_: String,
}

/// <p> Information about the credentials for a GitHub, GitHub Enterprise, or Bitbucket repository. </p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct SourceCredentialsInfo {
    /// <p> The Amazon Resource Name (ARN) of the token. </p>
    #[serde(rename = "arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p> The type of authentication used by the credentials. Valid options are OAUTH, BASIC_AUTH, or PERSONAL_ACCESS_TOKEN. </p>
    #[serde(rename = "authType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auth_type: Option<String>,
    /// <p> The type of source provider. The valid options are GITHUB, GITHUB_ENTERPRISE, or BITBUCKET. </p>
    #[serde(rename = "serverType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server_type: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct StartBuildInput {
    /// <p>Build output artifact settings that override, for this build only, the latest ones already defined in the build project.</p>
    #[serde(rename = "artifactsOverride")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub artifacts_override: Option<ProjectArtifacts>,
    /// <p>A build spec declaration that overrides, for this build only, the latest one already defined in the build project.</p>
    #[serde(rename = "buildspecOverride")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub buildspec_override: Option<String>,
    /// <p>A ProjectCache object specified for this build that overrides the one defined in the build project.</p>
    #[serde(rename = "cacheOverride")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cache_override: Option<ProjectCache>,
    /// <p>The name of a certificate for this build that overrides the one specified in the build project.</p>
    #[serde(rename = "certificateOverride")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate_override: Option<String>,
    /// <p>The name of a compute type for this build that overrides the one specified in the build project.</p>
    #[serde(rename = "computeTypeOverride")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compute_type_override: Option<String>,
    /// <p>The AWS Key Management Service (AWS KMS) customer master key (CMK) that overrides the one specified in the build project. The CMK key encrypts the build output artifacts.</p> <note> <p> You can use a cross-account KMS key to encrypt the build output artifacts if your service role has permission to that key. </p> </note> <p>You can specify either the Amazon Resource Name (ARN) of the CMK or, if available, the CMK's alias (using the format <code>alias/<i>alias-name</i> </code>).</p>
    #[serde(rename = "encryptionKeyOverride")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encryption_key_override: Option<String>,
    /// <p>A container type for this build that overrides the one specified in the build project.</p>
    #[serde(rename = "environmentTypeOverride")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub environment_type_override: Option<String>,
    /// <p>A set of environment variables that overrides, for this build only, the latest ones already defined in the build project.</p>
    #[serde(rename = "environmentVariablesOverride")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub environment_variables_override: Option<Vec<EnvironmentVariable>>,
    /// <p>The user-defined depth of history, with a minimum value of 0, that overrides, for this build only, any previous depth of history defined in the build project.</p>
    #[serde(rename = "gitCloneDepthOverride")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub git_clone_depth_override: Option<i64>,
    /// <p> Information about the Git submodules configuration for this build of an AWS CodeBuild build project. </p>
    #[serde(rename = "gitSubmodulesConfigOverride")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub git_submodules_config_override: Option<GitSubmodulesConfig>,
    /// <p>A unique, case sensitive identifier you provide to ensure the idempotency of the StartBuild request. The token is included in the StartBuild request and is valid for 12 hours. If you repeat the StartBuild request with the same token, but change a parameter, AWS CodeBuild returns a parameter mismatch error. </p>
    #[serde(rename = "idempotencyToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub idempotency_token: Option<String>,
    /// <p>The name of an image for this build that overrides the one specified in the build project.</p>
    #[serde(rename = "imageOverride")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_override: Option<String>,
    /// <p> The type of credentials AWS CodeBuild uses to pull images in your build. There are two valid values: </p> <ul> <li> <p> <code>CODEBUILD</code> specifies that AWS CodeBuild uses its own credentials. This requires that you modify your ECR repository policy to trust AWS CodeBuild's service principal.</p> </li> <li> <p> <code>SERVICE_ROLE</code> specifies that AWS CodeBuild uses your build project's service role. </p> </li> </ul> <p> When using a cross-account or private registry image, you must use SERVICE_ROLE credentials. When using an AWS CodeBuild curated image, you must use CODEBUILD credentials. </p>
    #[serde(rename = "imagePullCredentialsTypeOverride")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_pull_credentials_type_override: Option<String>,
    /// <p>Enable this flag to override the insecure SSL setting that is specified in the build project. The insecure SSL setting determines whether to ignore SSL warnings while connecting to the project source code. This override applies only if the build's source is GitHub Enterprise.</p>
    #[serde(rename = "insecureSslOverride")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub insecure_ssl_override: Option<bool>,
    /// <p> Log settings for this build that override the log settings defined in the build project. </p>
    #[serde(rename = "logsConfigOverride")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logs_config_override: Option<LogsConfig>,
    /// <p>Enable this flag to override privileged mode in the build project.</p>
    #[serde(rename = "privilegedModeOverride")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub privileged_mode_override: Option<bool>,
    /// <p>The name of the AWS CodeBuild build project to start running a build.</p>
    #[serde(rename = "projectName")]
    pub project_name: String,
    /// <p> The number of minutes a build is allowed to be queued before it times out. </p>
    #[serde(rename = "queuedTimeoutInMinutesOverride")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub queued_timeout_in_minutes_override: Option<i64>,
    /// <p> The credentials for access to a private registry. </p>
    #[serde(rename = "registryCredentialOverride")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub registry_credential_override: Option<RegistryCredential>,
    /// <p><p> Set to true to report to your source provider the status of a build&#39;s start and completion. If you use this option with a source provider other than GitHub, GitHub Enterprise, or Bitbucket, an invalidInputException is thrown. </p> <note> <p> The status of a build triggered by a webhook is always reported to your source provider. </p> </note></p>
    #[serde(rename = "reportBuildStatusOverride")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub report_build_status_override: Option<bool>,
    /// <p> An array of <code>ProjectArtifacts</code> objects. </p>
    #[serde(rename = "secondaryArtifactsOverride")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secondary_artifacts_override: Option<Vec<ProjectArtifacts>>,
    /// <p> An array of <code>ProjectSource</code> objects. </p>
    #[serde(rename = "secondarySourcesOverride")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secondary_sources_override: Option<Vec<ProjectSource>>,
    /// <p> An array of <code>ProjectSourceVersion</code> objects that specify one or more versions of the project's secondary sources to be used for this build only. </p>
    #[serde(rename = "secondarySourcesVersionOverride")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secondary_sources_version_override: Option<Vec<ProjectSourceVersion>>,
    /// <p>The name of a service role for this build that overrides the one specified in the build project.</p>
    #[serde(rename = "serviceRoleOverride")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_role_override: Option<String>,
    /// <p>An authorization type for this build that overrides the one defined in the build project. This override applies only if the build project's source is BitBucket or GitHub.</p>
    #[serde(rename = "sourceAuthOverride")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_auth_override: Option<SourceAuth>,
    /// <p>A location that overrides, for this build, the source location for the one defined in the build project.</p>
    #[serde(rename = "sourceLocationOverride")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_location_override: Option<String>,
    /// <p>A source input type, for this build, that overrides the source input defined in the build project.</p>
    #[serde(rename = "sourceTypeOverride")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_type_override: Option<String>,
    /// <p>A version of the build input to be built, for this build only. If not specified, the latest version is used. If specified, must be one of:</p> <ul> <li> <p>For AWS CodeCommit: the commit ID, branch, or Git tag to use.</p> </li> <li> <p>For GitHub: the commit ID, pull request ID, branch name, or tag name that corresponds to the version of the source code you want to build. If a pull request ID is specified, it must use the format <code>pr/pull-request-ID</code> (for example <code>pr/25</code>). If a branch name is specified, the branch's HEAD commit ID is used. If not specified, the default branch's HEAD commit ID is used.</p> </li> <li> <p>For Bitbucket: the commit ID, branch name, or tag name that corresponds to the version of the source code you want to build. If a branch name is specified, the branch's HEAD commit ID is used. If not specified, the default branch's HEAD commit ID is used.</p> </li> <li> <p>For Amazon Simple Storage Service (Amazon S3): the version ID of the object that represents the build input ZIP file to use.</p> </li> </ul> <p> If <code>sourceVersion</code> is specified at the project level, then this <code>sourceVersion</code> (at the build level) takes precedence. </p> <p> For more information, see <a href="https://docs.aws.amazon.com/codebuild/latest/userguide/sample-source-version.html">Source Version Sample with CodeBuild</a> in the <i>AWS CodeBuild User Guide</i>. </p>
    #[serde(rename = "sourceVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_version: Option<String>,
    /// <p>The number of build timeout minutes, from 5 to 480 (8 hours), that overrides, for this build only, the latest setting already defined in the build project.</p>
    #[serde(rename = "timeoutInMinutesOverride")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeout_in_minutes_override: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct StartBuildOutput {
    /// <p>Information about the build to be run.</p>
    #[serde(rename = "build")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub build: Option<Build>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct StopBuildInput {
    /// <p>The ID of the build.</p>
    #[serde(rename = "id")]
    pub id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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

/// <p> Information about a test case created using a framework such as NUnit or Cucumber. A test case might be a unit test or a configuration test. </p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct TestCase {
    /// <p> The number of nanoseconds it took to run this test case. </p>
    #[serde(rename = "durationInNanoSeconds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration_in_nano_seconds: Option<i64>,
    /// <p> The date and time a test case expires. A test case expires 30 days after it is created. An expired test case is not available to view in CodeBuild. </p>
    #[serde(rename = "expired")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expired: Option<f64>,
    /// <p> A message associated with a test case. For example, an error message or stack trace. </p>
    #[serde(rename = "message")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    /// <p> The name of the test case. </p>
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p> A string that is applied to a series of related test cases. CodeBuild generates the prefix. The prefix depends on the framework used to generate the tests. </p>
    #[serde(rename = "prefix")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prefix: Option<String>,
    /// <p> The ARN of the report to which the test case belongs. </p>
    #[serde(rename = "reportArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub report_arn: Option<String>,
    /// <p> The status returned by the test case after it was run. Valid statuses are <code>SUCCEEDED</code>, <code>FAILED</code>, <code>ERROR</code>, <code>SKIPPED</code>, and <code>UNKNOWN</code>. </p>
    #[serde(rename = "status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// <p> The path to the raw data file that contains the test result. </p>
    #[serde(rename = "testRawDataPath")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub test_raw_data_path: Option<String>,
}

/// <p> A filter used to return specific types of test cases. </p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct TestCaseFilter {
    /// <p> The status used to filter test cases. Valid statuses are <code>SUCCEEDED</code>, <code>FAILED</code>, <code>ERROR</code>, <code>SKIPPED</code>, and <code>UNKNOWN</code>. A <code>TestCaseFilter</code> can have one status. </p>
    #[serde(rename = "status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

/// <p> Information about a test report. </p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct TestReportSummary {
    /// <p> The number of nanoseconds it took to run all of the test cases in this report. </p>
    #[serde(rename = "durationInNanoSeconds")]
    pub duration_in_nano_seconds: i64,
    /// <p> A map that contains the number of each type of status returned by the test results in this <code>TestReportSummary</code>. </p>
    #[serde(rename = "statusCounts")]
    pub status_counts: ::std::collections::HashMap<String, i64>,
    /// <p> The number of test cases in this <code>TestReportSummary</code>. The total includes truncated test cases. </p>
    #[serde(rename = "total")]
    pub total: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UpdateProjectInput {
    /// <p>Information to be changed about the build output artifacts for the build project.</p>
    #[serde(rename = "artifacts")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub artifacts: Option<ProjectArtifacts>,
    /// <p>Set this to true to generate a publicly accessible URL for your project's build badge.</p>
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
    /// <p>The AWS Key Management Service (AWS KMS) customer master key (CMK) to be used for encrypting the build output artifacts.</p> <note> <p> You can use a cross-account KMS key to encrypt the build output artifacts if your service role has permission to that key. </p> </note> <p>You can specify either the Amazon Resource Name (ARN) of the CMK or, if available, the CMK's alias (using the format <code>alias/<i>alias-name</i> </code>).</p>
    #[serde(rename = "encryptionKey")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encryption_key: Option<String>,
    /// <p>Information to be changed about the build environment for the build project.</p>
    #[serde(rename = "environment")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub environment: Option<ProjectEnvironment>,
    /// <p> Information about logs for the build project. A project can create logs in Amazon CloudWatch Logs, logs in an S3 bucket, or both. </p>
    #[serde(rename = "logsConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logs_config: Option<LogsConfig>,
    /// <p><p>The name of the build project.</p> <note> <p>You cannot change a build project&#39;s name.</p> </note></p>
    #[serde(rename = "name")]
    pub name: String,
    /// <p> The number of minutes a build is allowed to be queued before it times out. </p>
    #[serde(rename = "queuedTimeoutInMinutes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub queued_timeout_in_minutes: Option<i64>,
    /// <p> An array of <code>ProjectSource</code> objects. </p>
    #[serde(rename = "secondaryArtifacts")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secondary_artifacts: Option<Vec<ProjectArtifacts>>,
    /// <p> An array of <code>ProjectSourceVersion</code> objects. If <code>secondarySourceVersions</code> is specified at the build level, then they take over these <code>secondarySourceVersions</code> (at the project level). </p>
    #[serde(rename = "secondarySourceVersions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secondary_source_versions: Option<Vec<ProjectSourceVersion>>,
    /// <p> An array of <code>ProjectSource</code> objects. </p>
    #[serde(rename = "secondarySources")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secondary_sources: Option<Vec<ProjectSource>>,
    /// <p>The replacement ARN of the AWS Identity and Access Management (IAM) role that enables AWS CodeBuild to interact with dependent AWS services on behalf of the AWS account.</p>
    #[serde(rename = "serviceRole")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_role: Option<String>,
    /// <p>Information to be changed about the build input source code for the build project.</p>
    #[serde(rename = "source")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<ProjectSource>,
    /// <p> A version of the build input to be built for this project. If not specified, the latest version is used. If specified, it must be one of: </p> <ul> <li> <p>For AWS CodeCommit: the commit ID, branch, or Git tag to use.</p> </li> <li> <p>For GitHub: the commit ID, pull request ID, branch name, or tag name that corresponds to the version of the source code you want to build. If a pull request ID is specified, it must use the format <code>pr/pull-request-ID</code> (for example <code>pr/25</code>). If a branch name is specified, the branch's HEAD commit ID is used. If not specified, the default branch's HEAD commit ID is used.</p> </li> <li> <p>For Bitbucket: the commit ID, branch name, or tag name that corresponds to the version of the source code you want to build. If a branch name is specified, the branch's HEAD commit ID is used. If not specified, the default branch's HEAD commit ID is used.</p> </li> <li> <p>For Amazon Simple Storage Service (Amazon S3): the version ID of the object that represents the build input ZIP file to use.</p> </li> </ul> <p> If <code>sourceVersion</code> is specified at the build level, then that version takes precedence over this <code>sourceVersion</code> (at the project level). </p> <p> For more information, see <a href="https://docs.aws.amazon.com/codebuild/latest/userguide/sample-source-version.html">Source Version Sample with CodeBuild</a> in the <i>AWS CodeBuild User Guide</i>. </p>
    #[serde(rename = "sourceVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_version: Option<String>,
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UpdateProjectOutput {
    /// <p>Information about the build project that was changed.</p>
    #[serde(rename = "project")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub project: Option<Project>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UpdateReportGroupInput {
    /// <p> The ARN of the report group to update. </p>
    #[serde(rename = "arn")]
    pub arn: String,
    /// <p><p> Used to specify an updated export type. Valid values are: </p> <ul> <li> <p> <code>S3</code>: The report results are exported to an S3 bucket. </p> </li> <li> <p> <code>NO_EXPORT</code>: The report results are not exported. </p> </li> </ul></p>
    #[serde(rename = "exportConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub export_config: Option<ReportExportConfig>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UpdateReportGroupOutput {
    /// <p> Information about the updated report group. </p>
    #[serde(rename = "reportGroup")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub report_group: Option<ReportGroup>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UpdateWebhookInput {
    /// <p><p>A regular expression used to determine which repository branches are built when a webhook is triggered. If the name of a branch matches the regular expression, then it is built. If <code>branchFilter</code> is empty, then all branches are built.</p> <note> <p> It is recommended that you use <code>filterGroups</code> instead of <code>branchFilter</code>. </p> </note></p>
    #[serde(rename = "branchFilter")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub branch_filter: Option<String>,
    /// <p> An array of arrays of <code>WebhookFilter</code> objects used to determine if a webhook event can trigger a build. A filter group must contain at least one <code>EVENT</code> <code>WebhookFilter</code>. </p>
    #[serde(rename = "filterGroups")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter_groups: Option<Vec<Vec<WebhookFilter>>>,
    /// <p>The name of the AWS CodeBuild project.</p>
    #[serde(rename = "projectName")]
    pub project_name: String,
    /// <p> A boolean value that specifies whether the associated GitHub repository's secret token should be updated. If you use Bitbucket for your repository, <code>rotateSecret</code> is ignored. </p>
    #[serde(rename = "rotateSecret")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rotate_secret: Option<bool>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UpdateWebhookOutput {
    /// <p> Information about a repository's webhook that is associated with a project in AWS CodeBuild. </p>
    #[serde(rename = "webhook")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub webhook: Option<Webhook>,
}

/// <p>Information about the VPC configuration that AWS CodeBuild accesses.</p>
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

/// <p>Information about a webhook that connects repository events to a build project in AWS CodeBuild.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct Webhook {
    /// <p><p>A regular expression used to determine which repository branches are built when a webhook is triggered. If the name of a branch matches the regular expression, then it is built. If <code>branchFilter</code> is empty, then all branches are built.</p> <note> <p> It is recommended that you use <code>filterGroups</code> instead of <code>branchFilter</code>. </p> </note></p>
    #[serde(rename = "branchFilter")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub branch_filter: Option<String>,
    /// <p> An array of arrays of <code>WebhookFilter</code> objects used to determine which webhooks are triggered. At least one <code>WebhookFilter</code> in the array must specify <code>EVENT</code> as its <code>type</code>. </p> <p> For a build to be triggered, at least one filter group in the <code>filterGroups</code> array must pass. For a filter group to pass, each of its filters must pass. </p>
    #[serde(rename = "filterGroups")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter_groups: Option<Vec<Vec<WebhookFilter>>>,
    /// <p> A timestamp that indicates the last time a repository's secret token was modified. </p>
    #[serde(rename = "lastModifiedSecret")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_secret: Option<f64>,
    /// <p> The AWS CodeBuild endpoint where webhook events are sent.</p>
    #[serde(rename = "payloadUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payload_url: Option<String>,
    /// <p><p> The secret token of the associated repository. </p> <note> <p> A Bitbucket webhook does not support <code>secret</code>. </p> </note></p>
    #[serde(rename = "secret")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secret: Option<String>,
    /// <p>The URL to the webhook.</p>
    #[serde(rename = "url")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

/// <p> A filter used to determine which webhooks trigger a build. </p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WebhookFilter {
    /// <p> Used to indicate that the <code>pattern</code> determines which webhook events do not trigger a build. If true, then a webhook event that does not match the <code>pattern</code> triggers a build. If false, then a webhook event that matches the <code>pattern</code> triggers a build. </p>
    #[serde(rename = "excludeMatchedPattern")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exclude_matched_pattern: Option<bool>,
    /// <p> For a <code>WebHookFilter</code> that uses <code>EVENT</code> type, a comma-separated string that specifies one or more events. For example, the webhook filter <code>PUSH, PULL_REQUEST_CREATED, PULL_REQUEST_UPDATED</code> allows all push, pull request created, and pull request updated events to trigger a build. </p> <p> For a <code>WebHookFilter</code> that uses any of the other filter types, a regular expression pattern. For example, a <code>WebHookFilter</code> that uses <code>HEAD_REF</code> for its <code>type</code> and the pattern <code>^refs/heads/</code> triggers a build when the head reference is a branch with a reference name <code>refs/heads/branch-name</code>. </p>
    #[serde(rename = "pattern")]
    pub pattern: String,
    /// <p><p> The type of webhook filter. There are five webhook filter types: <code>EVENT</code>, <code>ACTOR<em>ACCOUNT</em>ID</code>, <code>HEAD<em>REF</code>, <code>BASE</em>REF</code>, and <code>FILE<em>PATH</code>. </p> <dl> <dt> EVENT </dt> <dd> <p> A webhook event triggers a build when the provided <code>pattern</code> matches one of four event types: <code>PUSH</code>, <code>PULL</em>REQUEST<em>CREATED</code>, <code>PULL</em>REQUEST<em>UPDATED</code>, and <code>PULL</em>REQUEST<em>REOPENED</code>. The <code>EVENT</code> patterns are specified as a comma-separated string. For example, <code>PUSH, PULL</em>REQUEST<em>CREATED, PULL</em>REQUEST<em>UPDATED</code> filters all push, pull request created, and pull request updated events. </p> <note> <p> The <code>PULL</em>REQUEST<em>REOPENED</code> works with GitHub and GitHub Enterprise only. </p> </note> </dd> <dt> ACTOR</em>ACCOUNT<em>ID </dt> <dd> <p> A webhook event triggers a build when a GitHub, GitHub Enterprise, or Bitbucket account ID matches the regular expression <code>pattern</code>. </p> </dd> <dt> HEAD</em>REF </dt> <dd> <p> A webhook event triggers a build when the head reference matches the regular expression <code>pattern</code>. For example, <code>refs/heads/branch-name</code> and <code>refs/tags/tag-name</code>. </p> <p> Works with GitHub and GitHub Enterprise push, GitHub and GitHub Enterprise pull request, Bitbucket push, and Bitbucket pull request events. </p> </dd> <dt> BASE<em>REF </dt> <dd> <p> A webhook event triggers a build when the base reference matches the regular expression <code>pattern</code>. For example, <code>refs/heads/branch-name</code>. </p> <note> <p> Works with pull request events only. </p> </note> </dd> <dt> FILE</em>PATH </dt> <dd> <p> A webhook triggers a build when the path of a changed file matches the regular expression <code>pattern</code>. </p> <note> <p> Works with GitHub and GitHub Enterprise push events only. </p> </note> </dd> </dl></p>
    #[serde(rename = "type")]
    pub type_: String,
}

/// Errors returned by BatchDeleteBuilds
#[derive(Debug, PartialEq)]
pub enum BatchDeleteBuildsError {
    /// <p>The input value that was provided is not valid.</p>
    InvalidInput(String),
}

impl BatchDeleteBuildsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<BatchDeleteBuildsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidInputException" => {
                    return RusotoError::Service(BatchDeleteBuildsError::InvalidInput(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for BatchDeleteBuildsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            BatchDeleteBuildsError::InvalidInput(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for BatchDeleteBuildsError {}
/// Errors returned by BatchGetBuilds
#[derive(Debug, PartialEq)]
pub enum BatchGetBuildsError {
    /// <p>The input value that was provided is not valid.</p>
    InvalidInput(String),
}

impl BatchGetBuildsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<BatchGetBuildsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidInputException" => {
                    return RusotoError::Service(BatchGetBuildsError::InvalidInput(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for BatchGetBuildsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            BatchGetBuildsError::InvalidInput(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for BatchGetBuildsError {}
/// Errors returned by BatchGetProjects
#[derive(Debug, PartialEq)]
pub enum BatchGetProjectsError {
    /// <p>The input value that was provided is not valid.</p>
    InvalidInput(String),
}

impl BatchGetProjectsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<BatchGetProjectsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidInputException" => {
                    return RusotoError::Service(BatchGetProjectsError::InvalidInput(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for BatchGetProjectsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            BatchGetProjectsError::InvalidInput(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for BatchGetProjectsError {}
/// Errors returned by BatchGetReportGroups
#[derive(Debug, PartialEq)]
pub enum BatchGetReportGroupsError {
    /// <p>The input value that was provided is not valid.</p>
    InvalidInput(String),
}

impl BatchGetReportGroupsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<BatchGetReportGroupsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidInputException" => {
                    return RusotoError::Service(BatchGetReportGroupsError::InvalidInput(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for BatchGetReportGroupsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            BatchGetReportGroupsError::InvalidInput(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for BatchGetReportGroupsError {}
/// Errors returned by BatchGetReports
#[derive(Debug, PartialEq)]
pub enum BatchGetReportsError {
    /// <p>The input value that was provided is not valid.</p>
    InvalidInput(String),
}

impl BatchGetReportsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<BatchGetReportsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidInputException" => {
                    return RusotoError::Service(BatchGetReportsError::InvalidInput(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for BatchGetReportsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            BatchGetReportsError::InvalidInput(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for BatchGetReportsError {}
/// Errors returned by CreateProject
#[derive(Debug, PartialEq)]
pub enum CreateProjectError {
    /// <p>An AWS service limit was exceeded for the calling AWS account.</p>
    AccountLimitExceeded(String),
    /// <p>The input value that was provided is not valid.</p>
    InvalidInput(String),
    /// <p>The specified AWS resource cannot be created, because an AWS resource with the same settings already exists.</p>
    ResourceAlreadyExists(String),
}

impl CreateProjectError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateProjectError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccountLimitExceededException" => {
                    return RusotoError::Service(CreateProjectError::AccountLimitExceeded(err.msg))
                }
                "InvalidInputException" => {
                    return RusotoError::Service(CreateProjectError::InvalidInput(err.msg))
                }
                "ResourceAlreadyExistsException" => {
                    return RusotoError::Service(CreateProjectError::ResourceAlreadyExists(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for CreateProjectError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreateProjectError::AccountLimitExceeded(ref cause) => write!(f, "{}", cause),
            CreateProjectError::InvalidInput(ref cause) => write!(f, "{}", cause),
            CreateProjectError::ResourceAlreadyExists(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CreateProjectError {}
/// Errors returned by CreateReportGroup
#[derive(Debug, PartialEq)]
pub enum CreateReportGroupError {
    /// <p>An AWS service limit was exceeded for the calling AWS account.</p>
    AccountLimitExceeded(String),
    /// <p>The input value that was provided is not valid.</p>
    InvalidInput(String),
    /// <p>The specified AWS resource cannot be created, because an AWS resource with the same settings already exists.</p>
    ResourceAlreadyExists(String),
}

impl CreateReportGroupError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateReportGroupError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccountLimitExceededException" => {
                    return RusotoError::Service(CreateReportGroupError::AccountLimitExceeded(
                        err.msg,
                    ))
                }
                "InvalidInputException" => {
                    return RusotoError::Service(CreateReportGroupError::InvalidInput(err.msg))
                }
                "ResourceAlreadyExistsException" => {
                    return RusotoError::Service(CreateReportGroupError::ResourceAlreadyExists(
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
impl fmt::Display for CreateReportGroupError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreateReportGroupError::AccountLimitExceeded(ref cause) => write!(f, "{}", cause),
            CreateReportGroupError::InvalidInput(ref cause) => write!(f, "{}", cause),
            CreateReportGroupError::ResourceAlreadyExists(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CreateReportGroupError {}
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
}

impl CreateWebhookError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateWebhookError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidInputException" => {
                    return RusotoError::Service(CreateWebhookError::InvalidInput(err.msg))
                }
                "OAuthProviderException" => {
                    return RusotoError::Service(CreateWebhookError::OAuthProvider(err.msg))
                }
                "ResourceAlreadyExistsException" => {
                    return RusotoError::Service(CreateWebhookError::ResourceAlreadyExists(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(CreateWebhookError::ResourceNotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for CreateWebhookError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreateWebhookError::InvalidInput(ref cause) => write!(f, "{}", cause),
            CreateWebhookError::OAuthProvider(ref cause) => write!(f, "{}", cause),
            CreateWebhookError::ResourceAlreadyExists(ref cause) => write!(f, "{}", cause),
            CreateWebhookError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CreateWebhookError {}
/// Errors returned by DeleteProject
#[derive(Debug, PartialEq)]
pub enum DeleteProjectError {
    /// <p>The input value that was provided is not valid.</p>
    InvalidInput(String),
}

impl DeleteProjectError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteProjectError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidInputException" => {
                    return RusotoError::Service(DeleteProjectError::InvalidInput(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DeleteProjectError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteProjectError::InvalidInput(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeleteProjectError {}
/// Errors returned by DeleteReport
#[derive(Debug, PartialEq)]
pub enum DeleteReportError {
    /// <p>The input value that was provided is not valid.</p>
    InvalidInput(String),
}

impl DeleteReportError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteReportError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidInputException" => {
                    return RusotoError::Service(DeleteReportError::InvalidInput(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DeleteReportError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteReportError::InvalidInput(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeleteReportError {}
/// Errors returned by DeleteReportGroup
#[derive(Debug, PartialEq)]
pub enum DeleteReportGroupError {
    /// <p>The input value that was provided is not valid.</p>
    InvalidInput(String),
}

impl DeleteReportGroupError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteReportGroupError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidInputException" => {
                    return RusotoError::Service(DeleteReportGroupError::InvalidInput(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DeleteReportGroupError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteReportGroupError::InvalidInput(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeleteReportGroupError {}
/// Errors returned by DeleteResourcePolicy
#[derive(Debug, PartialEq)]
pub enum DeleteResourcePolicyError {
    /// <p>The input value that was provided is not valid.</p>
    InvalidInput(String),
}

impl DeleteResourcePolicyError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteResourcePolicyError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidInputException" => {
                    return RusotoError::Service(DeleteResourcePolicyError::InvalidInput(err.msg))
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
            DeleteResourcePolicyError::InvalidInput(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeleteResourcePolicyError {}
/// Errors returned by DeleteSourceCredentials
#[derive(Debug, PartialEq)]
pub enum DeleteSourceCredentialsError {
    /// <p>The input value that was provided is not valid.</p>
    InvalidInput(String),
    /// <p>The specified AWS resource cannot be found.</p>
    ResourceNotFound(String),
}

impl DeleteSourceCredentialsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteSourceCredentialsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidInputException" => {
                    return RusotoError::Service(DeleteSourceCredentialsError::InvalidInput(
                        err.msg,
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(DeleteSourceCredentialsError::ResourceNotFound(
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
impl fmt::Display for DeleteSourceCredentialsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteSourceCredentialsError::InvalidInput(ref cause) => write!(f, "{}", cause),
            DeleteSourceCredentialsError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeleteSourceCredentialsError {}
/// Errors returned by DeleteWebhook
#[derive(Debug, PartialEq)]
pub enum DeleteWebhookError {
    /// <p>The input value that was provided is not valid.</p>
    InvalidInput(String),
    /// <p>There was a problem with the underlying OAuth provider.</p>
    OAuthProvider(String),
    /// <p>The specified AWS resource cannot be found.</p>
    ResourceNotFound(String),
}

impl DeleteWebhookError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteWebhookError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidInputException" => {
                    return RusotoError::Service(DeleteWebhookError::InvalidInput(err.msg))
                }
                "OAuthProviderException" => {
                    return RusotoError::Service(DeleteWebhookError::OAuthProvider(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(DeleteWebhookError::ResourceNotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DeleteWebhookError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteWebhookError::InvalidInput(ref cause) => write!(f, "{}", cause),
            DeleteWebhookError::OAuthProvider(ref cause) => write!(f, "{}", cause),
            DeleteWebhookError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeleteWebhookError {}
/// Errors returned by DescribeTestCases
#[derive(Debug, PartialEq)]
pub enum DescribeTestCasesError {
    /// <p>The input value that was provided is not valid.</p>
    InvalidInput(String),
    /// <p>The specified AWS resource cannot be found.</p>
    ResourceNotFound(String),
}

impl DescribeTestCasesError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeTestCasesError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidInputException" => {
                    return RusotoError::Service(DescribeTestCasesError::InvalidInput(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(DescribeTestCasesError::ResourceNotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DescribeTestCasesError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeTestCasesError::InvalidInput(ref cause) => write!(f, "{}", cause),
            DescribeTestCasesError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DescribeTestCasesError {}
/// Errors returned by GetResourcePolicy
#[derive(Debug, PartialEq)]
pub enum GetResourcePolicyError {
    /// <p>The input value that was provided is not valid.</p>
    InvalidInput(String),
    /// <p>The specified AWS resource cannot be found.</p>
    ResourceNotFound(String),
}

impl GetResourcePolicyError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetResourcePolicyError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidInputException" => {
                    return RusotoError::Service(GetResourcePolicyError::InvalidInput(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(GetResourcePolicyError::ResourceNotFound(err.msg))
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
            GetResourcePolicyError::InvalidInput(ref cause) => write!(f, "{}", cause),
            GetResourcePolicyError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetResourcePolicyError {}
/// Errors returned by ImportSourceCredentials
#[derive(Debug, PartialEq)]
pub enum ImportSourceCredentialsError {
    /// <p>An AWS service limit was exceeded for the calling AWS account.</p>
    AccountLimitExceeded(String),
    /// <p>The input value that was provided is not valid.</p>
    InvalidInput(String),
    /// <p>The specified AWS resource cannot be created, because an AWS resource with the same settings already exists.</p>
    ResourceAlreadyExists(String),
}

impl ImportSourceCredentialsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ImportSourceCredentialsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccountLimitExceededException" => {
                    return RusotoError::Service(
                        ImportSourceCredentialsError::AccountLimitExceeded(err.msg),
                    )
                }
                "InvalidInputException" => {
                    return RusotoError::Service(ImportSourceCredentialsError::InvalidInput(
                        err.msg,
                    ))
                }
                "ResourceAlreadyExistsException" => {
                    return RusotoError::Service(
                        ImportSourceCredentialsError::ResourceAlreadyExists(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ImportSourceCredentialsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ImportSourceCredentialsError::AccountLimitExceeded(ref cause) => write!(f, "{}", cause),
            ImportSourceCredentialsError::InvalidInput(ref cause) => write!(f, "{}", cause),
            ImportSourceCredentialsError::ResourceAlreadyExists(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for ImportSourceCredentialsError {}
/// Errors returned by InvalidateProjectCache
#[derive(Debug, PartialEq)]
pub enum InvalidateProjectCacheError {
    /// <p>The input value that was provided is not valid.</p>
    InvalidInput(String),
    /// <p>The specified AWS resource cannot be found.</p>
    ResourceNotFound(String),
}

impl InvalidateProjectCacheError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<InvalidateProjectCacheError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidInputException" => {
                    return RusotoError::Service(InvalidateProjectCacheError::InvalidInput(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(InvalidateProjectCacheError::ResourceNotFound(
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
impl fmt::Display for InvalidateProjectCacheError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            InvalidateProjectCacheError::InvalidInput(ref cause) => write!(f, "{}", cause),
            InvalidateProjectCacheError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for InvalidateProjectCacheError {}
/// Errors returned by ListBuilds
#[derive(Debug, PartialEq)]
pub enum ListBuildsError {
    /// <p>The input value that was provided is not valid.</p>
    InvalidInput(String),
}

impl ListBuildsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListBuildsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidInputException" => {
                    return RusotoError::Service(ListBuildsError::InvalidInput(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ListBuildsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListBuildsError::InvalidInput(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListBuildsError {}
/// Errors returned by ListBuildsForProject
#[derive(Debug, PartialEq)]
pub enum ListBuildsForProjectError {
    /// <p>The input value that was provided is not valid.</p>
    InvalidInput(String),
    /// <p>The specified AWS resource cannot be found.</p>
    ResourceNotFound(String),
}

impl ListBuildsForProjectError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListBuildsForProjectError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidInputException" => {
                    return RusotoError::Service(ListBuildsForProjectError::InvalidInput(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(ListBuildsForProjectError::ResourceNotFound(
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
impl fmt::Display for ListBuildsForProjectError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListBuildsForProjectError::InvalidInput(ref cause) => write!(f, "{}", cause),
            ListBuildsForProjectError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListBuildsForProjectError {}
/// Errors returned by ListCuratedEnvironmentImages
#[derive(Debug, PartialEq)]
pub enum ListCuratedEnvironmentImagesError {}

impl ListCuratedEnvironmentImagesError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<ListCuratedEnvironmentImagesError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ListCuratedEnvironmentImagesError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {}
    }
}
impl Error for ListCuratedEnvironmentImagesError {}
/// Errors returned by ListProjects
#[derive(Debug, PartialEq)]
pub enum ListProjectsError {
    /// <p>The input value that was provided is not valid.</p>
    InvalidInput(String),
}

impl ListProjectsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListProjectsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidInputException" => {
                    return RusotoError::Service(ListProjectsError::InvalidInput(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ListProjectsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListProjectsError::InvalidInput(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListProjectsError {}
/// Errors returned by ListReportGroups
#[derive(Debug, PartialEq)]
pub enum ListReportGroupsError {
    /// <p>The input value that was provided is not valid.</p>
    InvalidInput(String),
}

impl ListReportGroupsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListReportGroupsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidInputException" => {
                    return RusotoError::Service(ListReportGroupsError::InvalidInput(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ListReportGroupsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListReportGroupsError::InvalidInput(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListReportGroupsError {}
/// Errors returned by ListReports
#[derive(Debug, PartialEq)]
pub enum ListReportsError {
    /// <p>The input value that was provided is not valid.</p>
    InvalidInput(String),
}

impl ListReportsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListReportsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidInputException" => {
                    return RusotoError::Service(ListReportsError::InvalidInput(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ListReportsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListReportsError::InvalidInput(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListReportsError {}
/// Errors returned by ListReportsForReportGroup
#[derive(Debug, PartialEq)]
pub enum ListReportsForReportGroupError {
    /// <p>The input value that was provided is not valid.</p>
    InvalidInput(String),
    /// <p>The specified AWS resource cannot be found.</p>
    ResourceNotFound(String),
}

impl ListReportsForReportGroupError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListReportsForReportGroupError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidInputException" => {
                    return RusotoError::Service(ListReportsForReportGroupError::InvalidInput(
                        err.msg,
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(ListReportsForReportGroupError::ResourceNotFound(
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
impl fmt::Display for ListReportsForReportGroupError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListReportsForReportGroupError::InvalidInput(ref cause) => write!(f, "{}", cause),
            ListReportsForReportGroupError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListReportsForReportGroupError {}
/// Errors returned by ListSharedProjects
#[derive(Debug, PartialEq)]
pub enum ListSharedProjectsError {
    /// <p>The input value that was provided is not valid.</p>
    InvalidInput(String),
}

impl ListSharedProjectsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListSharedProjectsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidInputException" => {
                    return RusotoError::Service(ListSharedProjectsError::InvalidInput(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ListSharedProjectsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListSharedProjectsError::InvalidInput(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListSharedProjectsError {}
/// Errors returned by ListSharedReportGroups
#[derive(Debug, PartialEq)]
pub enum ListSharedReportGroupsError {
    /// <p>The input value that was provided is not valid.</p>
    InvalidInput(String),
}

impl ListSharedReportGroupsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListSharedReportGroupsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidInputException" => {
                    return RusotoError::Service(ListSharedReportGroupsError::InvalidInput(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ListSharedReportGroupsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListSharedReportGroupsError::InvalidInput(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListSharedReportGroupsError {}
/// Errors returned by ListSourceCredentials
#[derive(Debug, PartialEq)]
pub enum ListSourceCredentialsError {}

impl ListSourceCredentialsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListSourceCredentialsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ListSourceCredentialsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {}
    }
}
impl Error for ListSourceCredentialsError {}
/// Errors returned by PutResourcePolicy
#[derive(Debug, PartialEq)]
pub enum PutResourcePolicyError {
    /// <p>The input value that was provided is not valid.</p>
    InvalidInput(String),
    /// <p>The specified AWS resource cannot be found.</p>
    ResourceNotFound(String),
}

impl PutResourcePolicyError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<PutResourcePolicyError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidInputException" => {
                    return RusotoError::Service(PutResourcePolicyError::InvalidInput(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(PutResourcePolicyError::ResourceNotFound(err.msg))
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
            PutResourcePolicyError::InvalidInput(ref cause) => write!(f, "{}", cause),
            PutResourcePolicyError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for PutResourcePolicyError {}
/// Errors returned by StartBuild
#[derive(Debug, PartialEq)]
pub enum StartBuildError {
    /// <p>An AWS service limit was exceeded for the calling AWS account.</p>
    AccountLimitExceeded(String),
    /// <p>The input value that was provided is not valid.</p>
    InvalidInput(String),
    /// <p>The specified AWS resource cannot be found.</p>
    ResourceNotFound(String),
}

impl StartBuildError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<StartBuildError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccountLimitExceededException" => {
                    return RusotoError::Service(StartBuildError::AccountLimitExceeded(err.msg))
                }
                "InvalidInputException" => {
                    return RusotoError::Service(StartBuildError::InvalidInput(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(StartBuildError::ResourceNotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for StartBuildError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            StartBuildError::AccountLimitExceeded(ref cause) => write!(f, "{}", cause),
            StartBuildError::InvalidInput(ref cause) => write!(f, "{}", cause),
            StartBuildError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for StartBuildError {}
/// Errors returned by StopBuild
#[derive(Debug, PartialEq)]
pub enum StopBuildError {
    /// <p>The input value that was provided is not valid.</p>
    InvalidInput(String),
    /// <p>The specified AWS resource cannot be found.</p>
    ResourceNotFound(String),
}

impl StopBuildError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<StopBuildError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidInputException" => {
                    return RusotoError::Service(StopBuildError::InvalidInput(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(StopBuildError::ResourceNotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for StopBuildError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            StopBuildError::InvalidInput(ref cause) => write!(f, "{}", cause),
            StopBuildError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for StopBuildError {}
/// Errors returned by UpdateProject
#[derive(Debug, PartialEq)]
pub enum UpdateProjectError {
    /// <p>The input value that was provided is not valid.</p>
    InvalidInput(String),
    /// <p>The specified AWS resource cannot be found.</p>
    ResourceNotFound(String),
}

impl UpdateProjectError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdateProjectError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidInputException" => {
                    return RusotoError::Service(UpdateProjectError::InvalidInput(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(UpdateProjectError::ResourceNotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for UpdateProjectError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UpdateProjectError::InvalidInput(ref cause) => write!(f, "{}", cause),
            UpdateProjectError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for UpdateProjectError {}
/// Errors returned by UpdateReportGroup
#[derive(Debug, PartialEq)]
pub enum UpdateReportGroupError {
    /// <p>The input value that was provided is not valid.</p>
    InvalidInput(String),
    /// <p>The specified AWS resource cannot be found.</p>
    ResourceNotFound(String),
}

impl UpdateReportGroupError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdateReportGroupError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidInputException" => {
                    return RusotoError::Service(UpdateReportGroupError::InvalidInput(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(UpdateReportGroupError::ResourceNotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for UpdateReportGroupError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UpdateReportGroupError::InvalidInput(ref cause) => write!(f, "{}", cause),
            UpdateReportGroupError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for UpdateReportGroupError {}
/// Errors returned by UpdateWebhook
#[derive(Debug, PartialEq)]
pub enum UpdateWebhookError {
    /// <p>The input value that was provided is not valid.</p>
    InvalidInput(String),
    /// <p>There was a problem with the underlying OAuth provider.</p>
    OAuthProvider(String),
    /// <p>The specified AWS resource cannot be found.</p>
    ResourceNotFound(String),
}

impl UpdateWebhookError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdateWebhookError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidInputException" => {
                    return RusotoError::Service(UpdateWebhookError::InvalidInput(err.msg))
                }
                "OAuthProviderException" => {
                    return RusotoError::Service(UpdateWebhookError::OAuthProvider(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(UpdateWebhookError::ResourceNotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for UpdateWebhookError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UpdateWebhookError::InvalidInput(ref cause) => write!(f, "{}", cause),
            UpdateWebhookError::OAuthProvider(ref cause) => write!(f, "{}", cause),
            UpdateWebhookError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for UpdateWebhookError {}
/// Trait representing the capabilities of the AWS CodeBuild API. AWS CodeBuild clients implement this trait.
#[async_trait]
pub trait CodeBuild {
    /// <p>Deletes one or more builds.</p>
    async fn batch_delete_builds(
        &self,
        input: BatchDeleteBuildsInput,
    ) -> Result<BatchDeleteBuildsOutput, RusotoError<BatchDeleteBuildsError>>;

    /// <p>Gets information about one or more builds.</p>
    async fn batch_get_builds(
        &self,
        input: BatchGetBuildsInput,
    ) -> Result<BatchGetBuildsOutput, RusotoError<BatchGetBuildsError>>;

    /// <p>Gets information about one or more build projects.</p>
    async fn batch_get_projects(
        &self,
        input: BatchGetProjectsInput,
    ) -> Result<BatchGetProjectsOutput, RusotoError<BatchGetProjectsError>>;

    /// <p> Returns an array of report groups. </p>
    async fn batch_get_report_groups(
        &self,
        input: BatchGetReportGroupsInput,
    ) -> Result<BatchGetReportGroupsOutput, RusotoError<BatchGetReportGroupsError>>;

    /// <p> Returns an array of reports. </p>
    async fn batch_get_reports(
        &self,
        input: BatchGetReportsInput,
    ) -> Result<BatchGetReportsOutput, RusotoError<BatchGetReportsError>>;

    /// <p>Creates a build project.</p>
    async fn create_project(
        &self,
        input: CreateProjectInput,
    ) -> Result<CreateProjectOutput, RusotoError<CreateProjectError>>;

    /// <p> Creates a report group. A report group contains a collection of reports. </p>
    async fn create_report_group(
        &self,
        input: CreateReportGroupInput,
    ) -> Result<CreateReportGroupOutput, RusotoError<CreateReportGroupError>>;

    /// <p><p>For an existing AWS CodeBuild build project that has its source code stored in a GitHub or Bitbucket repository, enables AWS CodeBuild to start rebuilding the source code every time a code change is pushed to the repository.</p> <important> <p>If you enable webhooks for an AWS CodeBuild project, and the project is used as a build step in AWS CodePipeline, then two identical builds are created for each commit. One build is triggered through webhooks, and one through AWS CodePipeline. Because billing is on a per-build basis, you are billed for both builds. Therefore, if you are using AWS CodePipeline, we recommend that you disable webhooks in AWS CodeBuild. In the AWS CodeBuild console, clear the Webhook box. For more information, see step 5 in <a href="https://docs.aws.amazon.com/codebuild/latest/userguide/change-project.html#change-project-console">Change a Build Project&#39;s Settings</a>.</p> </important></p>
    async fn create_webhook(
        &self,
        input: CreateWebhookInput,
    ) -> Result<CreateWebhookOutput, RusotoError<CreateWebhookError>>;

    /// <p> Deletes a build project. When you delete a project, its builds are not deleted. </p>
    async fn delete_project(
        &self,
        input: DeleteProjectInput,
    ) -> Result<DeleteProjectOutput, RusotoError<DeleteProjectError>>;

    /// <p> Deletes a report. </p>
    async fn delete_report(
        &self,
        input: DeleteReportInput,
    ) -> Result<DeleteReportOutput, RusotoError<DeleteReportError>>;

    /// <p> <code>DeleteReportGroup</code>: Deletes a report group. Before you delete a report group, you must delete its reports. Use <a href="https://docs.aws.amazon.com/codebuild/latest/APIReference/API_ListReportsForReportGroup.html">ListReportsForReportGroup</a> to get the reports in a report group. Use <a href="https://docs.aws.amazon.com/codebuild/latest/APIReference/API_DeleteReport.html">DeleteReport</a> to delete the reports. If you call <code>DeleteReportGroup</code> for a report group that contains one or more reports, an exception is thrown. </p>
    async fn delete_report_group(
        &self,
        input: DeleteReportGroupInput,
    ) -> Result<DeleteReportGroupOutput, RusotoError<DeleteReportGroupError>>;

    /// <p> Deletes a resource policy that is identified by its resource ARN. </p>
    async fn delete_resource_policy(
        &self,
        input: DeleteResourcePolicyInput,
    ) -> Result<DeleteResourcePolicyOutput, RusotoError<DeleteResourcePolicyError>>;

    /// <p> Deletes a set of GitHub, GitHub Enterprise, or Bitbucket source credentials. </p>
    async fn delete_source_credentials(
        &self,
        input: DeleteSourceCredentialsInput,
    ) -> Result<DeleteSourceCredentialsOutput, RusotoError<DeleteSourceCredentialsError>>;

    /// <p>For an existing AWS CodeBuild build project that has its source code stored in a GitHub or Bitbucket repository, stops AWS CodeBuild from rebuilding the source code every time a code change is pushed to the repository.</p>
    async fn delete_webhook(
        &self,
        input: DeleteWebhookInput,
    ) -> Result<DeleteWebhookOutput, RusotoError<DeleteWebhookError>>;

    /// <p> Returns a list of details about test cases for a report. </p>
    async fn describe_test_cases(
        &self,
        input: DescribeTestCasesInput,
    ) -> Result<DescribeTestCasesOutput, RusotoError<DescribeTestCasesError>>;

    /// <p> Gets a resource policy that is identified by its resource ARN. </p>
    async fn get_resource_policy(
        &self,
        input: GetResourcePolicyInput,
    ) -> Result<GetResourcePolicyOutput, RusotoError<GetResourcePolicyError>>;

    /// <p> Imports the source repository credentials for an AWS CodeBuild project that has its source code stored in a GitHub, GitHub Enterprise, or Bitbucket repository. </p>
    async fn import_source_credentials(
        &self,
        input: ImportSourceCredentialsInput,
    ) -> Result<ImportSourceCredentialsOutput, RusotoError<ImportSourceCredentialsError>>;

    /// <p>Resets the cache for a project.</p>
    async fn invalidate_project_cache(
        &self,
        input: InvalidateProjectCacheInput,
    ) -> Result<InvalidateProjectCacheOutput, RusotoError<InvalidateProjectCacheError>>;

    /// <p>Gets a list of build IDs, with each build ID representing a single build.</p>
    async fn list_builds(
        &self,
        input: ListBuildsInput,
    ) -> Result<ListBuildsOutput, RusotoError<ListBuildsError>>;

    /// <p>Gets a list of build IDs for the specified build project, with each build ID representing a single build.</p>
    async fn list_builds_for_project(
        &self,
        input: ListBuildsForProjectInput,
    ) -> Result<ListBuildsForProjectOutput, RusotoError<ListBuildsForProjectError>>;

    /// <p>Gets information about Docker images that are managed by AWS CodeBuild.</p>
    async fn list_curated_environment_images(
        &self,
    ) -> Result<ListCuratedEnvironmentImagesOutput, RusotoError<ListCuratedEnvironmentImagesError>>;

    /// <p>Gets a list of build project names, with each build project name representing a single build project.</p>
    async fn list_projects(
        &self,
        input: ListProjectsInput,
    ) -> Result<ListProjectsOutput, RusotoError<ListProjectsError>>;

    /// <p> Gets a list ARNs for the report groups in the current AWS account. </p>
    async fn list_report_groups(
        &self,
        input: ListReportGroupsInput,
    ) -> Result<ListReportGroupsOutput, RusotoError<ListReportGroupsError>>;

    /// <p> Returns a list of ARNs for the reports in the current AWS account. </p>
    async fn list_reports(
        &self,
        input: ListReportsInput,
    ) -> Result<ListReportsOutput, RusotoError<ListReportsError>>;

    /// <p> Returns a list of ARNs for the reports that belong to a <code>ReportGroup</code>. </p>
    async fn list_reports_for_report_group(
        &self,
        input: ListReportsForReportGroupInput,
    ) -> Result<ListReportsForReportGroupOutput, RusotoError<ListReportsForReportGroupError>>;

    /// <p> Gets a list of projects that are shared with other AWS accounts or users. </p>
    async fn list_shared_projects(
        &self,
        input: ListSharedProjectsInput,
    ) -> Result<ListSharedProjectsOutput, RusotoError<ListSharedProjectsError>>;

    /// <p> Gets a list of report groups that are shared with other AWS accounts or users. </p>
    async fn list_shared_report_groups(
        &self,
        input: ListSharedReportGroupsInput,
    ) -> Result<ListSharedReportGroupsOutput, RusotoError<ListSharedReportGroupsError>>;

    /// <p> Returns a list of <code>SourceCredentialsInfo</code> objects. </p>
    async fn list_source_credentials(
        &self,
    ) -> Result<ListSourceCredentialsOutput, RusotoError<ListSourceCredentialsError>>;

    /// <p> Stores a resource policy for the ARN of a <code>Project</code> or <code>ReportGroup</code> object. </p>
    async fn put_resource_policy(
        &self,
        input: PutResourcePolicyInput,
    ) -> Result<PutResourcePolicyOutput, RusotoError<PutResourcePolicyError>>;

    /// <p>Starts running a build.</p>
    async fn start_build(
        &self,
        input: StartBuildInput,
    ) -> Result<StartBuildOutput, RusotoError<StartBuildError>>;

    /// <p>Attempts to stop running a build.</p>
    async fn stop_build(
        &self,
        input: StopBuildInput,
    ) -> Result<StopBuildOutput, RusotoError<StopBuildError>>;

    /// <p>Changes the settings of a build project.</p>
    async fn update_project(
        &self,
        input: UpdateProjectInput,
    ) -> Result<UpdateProjectOutput, RusotoError<UpdateProjectError>>;

    /// <p> Updates a report group. </p>
    async fn update_report_group(
        &self,
        input: UpdateReportGroupInput,
    ) -> Result<UpdateReportGroupOutput, RusotoError<UpdateReportGroupError>>;

    /// <p><p> Updates the webhook associated with an AWS CodeBuild build project. </p> <note> <p> If you use Bitbucket for your repository, <code>rotateSecret</code> is ignored. </p> </note></p>
    async fn update_webhook(
        &self,
        input: UpdateWebhookInput,
    ) -> Result<UpdateWebhookOutput, RusotoError<UpdateWebhookError>>;
}
/// A client for the AWS CodeBuild API.
#[derive(Clone)]
pub struct CodeBuildClient {
    client: Client,
    region: region::Region,
}

impl CodeBuildClient {
    /// Creates a client backed by the default tokio event loop.
    ///
    /// The client will use the default credentials provider and tls client.
    pub fn new(region: region::Region) -> CodeBuildClient {
        CodeBuildClient {
            client: Client::shared(),
            region,
        }
    }

    pub fn new_with<P, D>(
        request_dispatcher: D,
        credentials_provider: P,
        region: region::Region,
    ) -> CodeBuildClient
    where
        P: ProvideAwsCredentials + Send + Sync + 'static,
        D: DispatchSignedRequest + Send + Sync + 'static,
    {
        CodeBuildClient {
            client: Client::new_with(credentials_provider, request_dispatcher),
            region,
        }
    }

    pub fn new_with_client(client: Client, region: region::Region) -> CodeBuildClient {
        CodeBuildClient { client, region }
    }
}

#[async_trait]
impl CodeBuild for CodeBuildClient {
    /// <p>Deletes one or more builds.</p>
    async fn batch_delete_builds(
        &self,
        input: BatchDeleteBuildsInput,
    ) -> Result<BatchDeleteBuildsOutput, RusotoError<BatchDeleteBuildsError>> {
        let mut request = SignedRequest::new("POST", "codebuild", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "CodeBuild_20161006.BatchDeleteBuilds");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response).deserialize::<BatchDeleteBuildsOutput, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(BatchDeleteBuildsError::from_response(response))
        }
    }

    /// <p>Gets information about one or more builds.</p>
    async fn batch_get_builds(
        &self,
        input: BatchGetBuildsInput,
    ) -> Result<BatchGetBuildsOutput, RusotoError<BatchGetBuildsError>> {
        let mut request = SignedRequest::new("POST", "codebuild", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "CodeBuild_20161006.BatchGetBuilds");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response).deserialize::<BatchGetBuildsOutput, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(BatchGetBuildsError::from_response(response))
        }
    }

    /// <p>Gets information about one or more build projects.</p>
    async fn batch_get_projects(
        &self,
        input: BatchGetProjectsInput,
    ) -> Result<BatchGetProjectsOutput, RusotoError<BatchGetProjectsError>> {
        let mut request = SignedRequest::new("POST", "codebuild", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "CodeBuild_20161006.BatchGetProjects");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response).deserialize::<BatchGetProjectsOutput, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(BatchGetProjectsError::from_response(response))
        }
    }

    /// <p> Returns an array of report groups. </p>
    async fn batch_get_report_groups(
        &self,
        input: BatchGetReportGroupsInput,
    ) -> Result<BatchGetReportGroupsOutput, RusotoError<BatchGetReportGroupsError>> {
        let mut request = SignedRequest::new("POST", "codebuild", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "CodeBuild_20161006.BatchGetReportGroups");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response)
                .deserialize::<BatchGetReportGroupsOutput, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(BatchGetReportGroupsError::from_response(response))
        }
    }

    /// <p> Returns an array of reports. </p>
    async fn batch_get_reports(
        &self,
        input: BatchGetReportsInput,
    ) -> Result<BatchGetReportsOutput, RusotoError<BatchGetReportsError>> {
        let mut request = SignedRequest::new("POST", "codebuild", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "CodeBuild_20161006.BatchGetReports");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response).deserialize::<BatchGetReportsOutput, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(BatchGetReportsError::from_response(response))
        }
    }

    /// <p>Creates a build project.</p>
    async fn create_project(
        &self,
        input: CreateProjectInput,
    ) -> Result<CreateProjectOutput, RusotoError<CreateProjectError>> {
        let mut request = SignedRequest::new("POST", "codebuild", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "CodeBuild_20161006.CreateProject");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response).deserialize::<CreateProjectOutput, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(CreateProjectError::from_response(response))
        }
    }

    /// <p> Creates a report group. A report group contains a collection of reports. </p>
    async fn create_report_group(
        &self,
        input: CreateReportGroupInput,
    ) -> Result<CreateReportGroupOutput, RusotoError<CreateReportGroupError>> {
        let mut request = SignedRequest::new("POST", "codebuild", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "CodeBuild_20161006.CreateReportGroup");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response).deserialize::<CreateReportGroupOutput, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(CreateReportGroupError::from_response(response))
        }
    }

    /// <p><p>For an existing AWS CodeBuild build project that has its source code stored in a GitHub or Bitbucket repository, enables AWS CodeBuild to start rebuilding the source code every time a code change is pushed to the repository.</p> <important> <p>If you enable webhooks for an AWS CodeBuild project, and the project is used as a build step in AWS CodePipeline, then two identical builds are created for each commit. One build is triggered through webhooks, and one through AWS CodePipeline. Because billing is on a per-build basis, you are billed for both builds. Therefore, if you are using AWS CodePipeline, we recommend that you disable webhooks in AWS CodeBuild. In the AWS CodeBuild console, clear the Webhook box. For more information, see step 5 in <a href="https://docs.aws.amazon.com/codebuild/latest/userguide/change-project.html#change-project-console">Change a Build Project&#39;s Settings</a>.</p> </important></p>
    async fn create_webhook(
        &self,
        input: CreateWebhookInput,
    ) -> Result<CreateWebhookOutput, RusotoError<CreateWebhookError>> {
        let mut request = SignedRequest::new("POST", "codebuild", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "CodeBuild_20161006.CreateWebhook");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response).deserialize::<CreateWebhookOutput, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(CreateWebhookError::from_response(response))
        }
    }

    /// <p> Deletes a build project. When you delete a project, its builds are not deleted. </p>
    async fn delete_project(
        &self,
        input: DeleteProjectInput,
    ) -> Result<DeleteProjectOutput, RusotoError<DeleteProjectError>> {
        let mut request = SignedRequest::new("POST", "codebuild", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "CodeBuild_20161006.DeleteProject");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response).deserialize::<DeleteProjectOutput, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(DeleteProjectError::from_response(response))
        }
    }

    /// <p> Deletes a report. </p>
    async fn delete_report(
        &self,
        input: DeleteReportInput,
    ) -> Result<DeleteReportOutput, RusotoError<DeleteReportError>> {
        let mut request = SignedRequest::new("POST", "codebuild", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "CodeBuild_20161006.DeleteReport");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response).deserialize::<DeleteReportOutput, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(DeleteReportError::from_response(response))
        }
    }

    /// <p> <code>DeleteReportGroup</code>: Deletes a report group. Before you delete a report group, you must delete its reports. Use <a href="https://docs.aws.amazon.com/codebuild/latest/APIReference/API_ListReportsForReportGroup.html">ListReportsForReportGroup</a> to get the reports in a report group. Use <a href="https://docs.aws.amazon.com/codebuild/latest/APIReference/API_DeleteReport.html">DeleteReport</a> to delete the reports. If you call <code>DeleteReportGroup</code> for a report group that contains one or more reports, an exception is thrown. </p>
    async fn delete_report_group(
        &self,
        input: DeleteReportGroupInput,
    ) -> Result<DeleteReportGroupOutput, RusotoError<DeleteReportGroupError>> {
        let mut request = SignedRequest::new("POST", "codebuild", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "CodeBuild_20161006.DeleteReportGroup");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response).deserialize::<DeleteReportGroupOutput, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(DeleteReportGroupError::from_response(response))
        }
    }

    /// <p> Deletes a resource policy that is identified by its resource ARN. </p>
    async fn delete_resource_policy(
        &self,
        input: DeleteResourcePolicyInput,
    ) -> Result<DeleteResourcePolicyOutput, RusotoError<DeleteResourcePolicyError>> {
        let mut request = SignedRequest::new("POST", "codebuild", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "CodeBuild_20161006.DeleteResourcePolicy");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response)
                .deserialize::<DeleteResourcePolicyOutput, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(DeleteResourcePolicyError::from_response(response))
        }
    }

    /// <p> Deletes a set of GitHub, GitHub Enterprise, or Bitbucket source credentials. </p>
    async fn delete_source_credentials(
        &self,
        input: DeleteSourceCredentialsInput,
    ) -> Result<DeleteSourceCredentialsOutput, RusotoError<DeleteSourceCredentialsError>> {
        let mut request = SignedRequest::new("POST", "codebuild", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "CodeBuild_20161006.DeleteSourceCredentials");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response)
                .deserialize::<DeleteSourceCredentialsOutput, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(DeleteSourceCredentialsError::from_response(response))
        }
    }

    /// <p>For an existing AWS CodeBuild build project that has its source code stored in a GitHub or Bitbucket repository, stops AWS CodeBuild from rebuilding the source code every time a code change is pushed to the repository.</p>
    async fn delete_webhook(
        &self,
        input: DeleteWebhookInput,
    ) -> Result<DeleteWebhookOutput, RusotoError<DeleteWebhookError>> {
        let mut request = SignedRequest::new("POST", "codebuild", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "CodeBuild_20161006.DeleteWebhook");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response).deserialize::<DeleteWebhookOutput, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(DeleteWebhookError::from_response(response))
        }
    }

    /// <p> Returns a list of details about test cases for a report. </p>
    async fn describe_test_cases(
        &self,
        input: DescribeTestCasesInput,
    ) -> Result<DescribeTestCasesOutput, RusotoError<DescribeTestCasesError>> {
        let mut request = SignedRequest::new("POST", "codebuild", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "CodeBuild_20161006.DescribeTestCases");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response).deserialize::<DescribeTestCasesOutput, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(DescribeTestCasesError::from_response(response))
        }
    }

    /// <p> Gets a resource policy that is identified by its resource ARN. </p>
    async fn get_resource_policy(
        &self,
        input: GetResourcePolicyInput,
    ) -> Result<GetResourcePolicyOutput, RusotoError<GetResourcePolicyError>> {
        let mut request = SignedRequest::new("POST", "codebuild", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "CodeBuild_20161006.GetResourcePolicy");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response).deserialize::<GetResourcePolicyOutput, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(GetResourcePolicyError::from_response(response))
        }
    }

    /// <p> Imports the source repository credentials for an AWS CodeBuild project that has its source code stored in a GitHub, GitHub Enterprise, or Bitbucket repository. </p>
    async fn import_source_credentials(
        &self,
        input: ImportSourceCredentialsInput,
    ) -> Result<ImportSourceCredentialsOutput, RusotoError<ImportSourceCredentialsError>> {
        let mut request = SignedRequest::new("POST", "codebuild", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "CodeBuild_20161006.ImportSourceCredentials");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response)
                .deserialize::<ImportSourceCredentialsOutput, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(ImportSourceCredentialsError::from_response(response))
        }
    }

    /// <p>Resets the cache for a project.</p>
    async fn invalidate_project_cache(
        &self,
        input: InvalidateProjectCacheInput,
    ) -> Result<InvalidateProjectCacheOutput, RusotoError<InvalidateProjectCacheError>> {
        let mut request = SignedRequest::new("POST", "codebuild", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "CodeBuild_20161006.InvalidateProjectCache");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response)
                .deserialize::<InvalidateProjectCacheOutput, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(InvalidateProjectCacheError::from_response(response))
        }
    }

    /// <p>Gets a list of build IDs, with each build ID representing a single build.</p>
    async fn list_builds(
        &self,
        input: ListBuildsInput,
    ) -> Result<ListBuildsOutput, RusotoError<ListBuildsError>> {
        let mut request = SignedRequest::new("POST", "codebuild", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "CodeBuild_20161006.ListBuilds");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response).deserialize::<ListBuildsOutput, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(ListBuildsError::from_response(response))
        }
    }

    /// <p>Gets a list of build IDs for the specified build project, with each build ID representing a single build.</p>
    async fn list_builds_for_project(
        &self,
        input: ListBuildsForProjectInput,
    ) -> Result<ListBuildsForProjectOutput, RusotoError<ListBuildsForProjectError>> {
        let mut request = SignedRequest::new("POST", "codebuild", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "CodeBuild_20161006.ListBuildsForProject");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response)
                .deserialize::<ListBuildsForProjectOutput, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(ListBuildsForProjectError::from_response(response))
        }
    }

    /// <p>Gets information about Docker images that are managed by AWS CodeBuild.</p>
    async fn list_curated_environment_images(
        &self,
    ) -> Result<ListCuratedEnvironmentImagesOutput, RusotoError<ListCuratedEnvironmentImagesError>>
    {
        let mut request = SignedRequest::new("POST", "codebuild", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "CodeBuild_20161006.ListCuratedEnvironmentImages",
        );
        request.set_payload(Some(bytes::Bytes::from_static(b"{}")));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response)
                .deserialize::<ListCuratedEnvironmentImagesOutput, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(ListCuratedEnvironmentImagesError::from_response(response))
        }
    }

    /// <p>Gets a list of build project names, with each build project name representing a single build project.</p>
    async fn list_projects(
        &self,
        input: ListProjectsInput,
    ) -> Result<ListProjectsOutput, RusotoError<ListProjectsError>> {
        let mut request = SignedRequest::new("POST", "codebuild", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "CodeBuild_20161006.ListProjects");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response).deserialize::<ListProjectsOutput, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(ListProjectsError::from_response(response))
        }
    }

    /// <p> Gets a list ARNs for the report groups in the current AWS account. </p>
    async fn list_report_groups(
        &self,
        input: ListReportGroupsInput,
    ) -> Result<ListReportGroupsOutput, RusotoError<ListReportGroupsError>> {
        let mut request = SignedRequest::new("POST", "codebuild", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "CodeBuild_20161006.ListReportGroups");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response).deserialize::<ListReportGroupsOutput, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(ListReportGroupsError::from_response(response))
        }
    }

    /// <p> Returns a list of ARNs for the reports in the current AWS account. </p>
    async fn list_reports(
        &self,
        input: ListReportsInput,
    ) -> Result<ListReportsOutput, RusotoError<ListReportsError>> {
        let mut request = SignedRequest::new("POST", "codebuild", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "CodeBuild_20161006.ListReports");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response).deserialize::<ListReportsOutput, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(ListReportsError::from_response(response))
        }
    }

    /// <p> Returns a list of ARNs for the reports that belong to a <code>ReportGroup</code>. </p>
    async fn list_reports_for_report_group(
        &self,
        input: ListReportsForReportGroupInput,
    ) -> Result<ListReportsForReportGroupOutput, RusotoError<ListReportsForReportGroupError>> {
        let mut request = SignedRequest::new("POST", "codebuild", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "CodeBuild_20161006.ListReportsForReportGroup",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response)
                .deserialize::<ListReportsForReportGroupOutput, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(ListReportsForReportGroupError::from_response(response))
        }
    }

    /// <p> Gets a list of projects that are shared with other AWS accounts or users. </p>
    async fn list_shared_projects(
        &self,
        input: ListSharedProjectsInput,
    ) -> Result<ListSharedProjectsOutput, RusotoError<ListSharedProjectsError>> {
        let mut request = SignedRequest::new("POST", "codebuild", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "CodeBuild_20161006.ListSharedProjects");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response)
                .deserialize::<ListSharedProjectsOutput, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(ListSharedProjectsError::from_response(response))
        }
    }

    /// <p> Gets a list of report groups that are shared with other AWS accounts or users. </p>
    async fn list_shared_report_groups(
        &self,
        input: ListSharedReportGroupsInput,
    ) -> Result<ListSharedReportGroupsOutput, RusotoError<ListSharedReportGroupsError>> {
        let mut request = SignedRequest::new("POST", "codebuild", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "CodeBuild_20161006.ListSharedReportGroups");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response)
                .deserialize::<ListSharedReportGroupsOutput, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(ListSharedReportGroupsError::from_response(response))
        }
    }

    /// <p> Returns a list of <code>SourceCredentialsInfo</code> objects. </p>
    async fn list_source_credentials(
        &self,
    ) -> Result<ListSourceCredentialsOutput, RusotoError<ListSourceCredentialsError>> {
        let mut request = SignedRequest::new("POST", "codebuild", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "CodeBuild_20161006.ListSourceCredentials");
        request.set_payload(Some(bytes::Bytes::from_static(b"{}")));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response)
                .deserialize::<ListSourceCredentialsOutput, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(ListSourceCredentialsError::from_response(response))
        }
    }

    /// <p> Stores a resource policy for the ARN of a <code>Project</code> or <code>ReportGroup</code> object. </p>
    async fn put_resource_policy(
        &self,
        input: PutResourcePolicyInput,
    ) -> Result<PutResourcePolicyOutput, RusotoError<PutResourcePolicyError>> {
        let mut request = SignedRequest::new("POST", "codebuild", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "CodeBuild_20161006.PutResourcePolicy");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response).deserialize::<PutResourcePolicyOutput, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(PutResourcePolicyError::from_response(response))
        }
    }

    /// <p>Starts running a build.</p>
    async fn start_build(
        &self,
        input: StartBuildInput,
    ) -> Result<StartBuildOutput, RusotoError<StartBuildError>> {
        let mut request = SignedRequest::new("POST", "codebuild", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "CodeBuild_20161006.StartBuild");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response).deserialize::<StartBuildOutput, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(StartBuildError::from_response(response))
        }
    }

    /// <p>Attempts to stop running a build.</p>
    async fn stop_build(
        &self,
        input: StopBuildInput,
    ) -> Result<StopBuildOutput, RusotoError<StopBuildError>> {
        let mut request = SignedRequest::new("POST", "codebuild", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "CodeBuild_20161006.StopBuild");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response).deserialize::<StopBuildOutput, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(StopBuildError::from_response(response))
        }
    }

    /// <p>Changes the settings of a build project.</p>
    async fn update_project(
        &self,
        input: UpdateProjectInput,
    ) -> Result<UpdateProjectOutput, RusotoError<UpdateProjectError>> {
        let mut request = SignedRequest::new("POST", "codebuild", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "CodeBuild_20161006.UpdateProject");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response).deserialize::<UpdateProjectOutput, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(UpdateProjectError::from_response(response))
        }
    }

    /// <p> Updates a report group. </p>
    async fn update_report_group(
        &self,
        input: UpdateReportGroupInput,
    ) -> Result<UpdateReportGroupOutput, RusotoError<UpdateReportGroupError>> {
        let mut request = SignedRequest::new("POST", "codebuild", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "CodeBuild_20161006.UpdateReportGroup");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response).deserialize::<UpdateReportGroupOutput, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(UpdateReportGroupError::from_response(response))
        }
    }

    /// <p><p> Updates the webhook associated with an AWS CodeBuild build project. </p> <note> <p> If you use Bitbucket for your repository, <code>rotateSecret</code> is ignored. </p> </note></p>
    async fn update_webhook(
        &self,
        input: UpdateWebhookInput,
    ) -> Result<UpdateWebhookOutput, RusotoError<UpdateWebhookError>> {
        let mut request = SignedRequest::new("POST", "codebuild", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "CodeBuild_20161006.UpdateWebhook");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response).deserialize::<UpdateWebhookOutput, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(UpdateWebhookError::from_response(response))
        }
    }
}
