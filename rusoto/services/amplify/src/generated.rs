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
/// <p> Represents the different branches of a repository for building, deploying, and hosting an Amplify app. </p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct App {
    /// <p> The Amazon Resource Name (ARN) of the Amplify app. </p>
    #[serde(rename = "appArn")]
    pub app_arn: String,
    /// <p> The unique ID of the Amplify app. </p>
    #[serde(rename = "appId")]
    pub app_id: String,
    /// <p> Describes the automated branch creation configuration for the Amplify app. </p>
    #[serde(rename = "autoBranchCreationConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_branch_creation_config: Option<AutoBranchCreationConfig>,
    /// <p> Describes the automated branch creation glob patterns for the Amplify app. </p>
    #[serde(rename = "autoBranchCreationPatterns")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_branch_creation_patterns: Option<Vec<String>>,
    /// <p> The basic authorization credentials for branches for the Amplify app. </p>
    #[serde(rename = "basicAuthCredentials")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub basic_auth_credentials: Option<String>,
    /// <p> Describes the content of the build specification (build spec) for the Amplify app. </p>
    #[serde(rename = "buildSpec")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub build_spec: Option<String>,
    /// <p> Creates a date and time for the Amplify app. </p>
    #[serde(rename = "createTime")]
    pub create_time: f64,
    /// <p>Describes the custom HTTP headers for the Amplify app.</p>
    #[serde(rename = "customHeaders")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_headers: Option<String>,
    /// <p> Describes the custom redirect and rewrite rules for the Amplify app. </p>
    #[serde(rename = "customRules")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_rules: Option<Vec<CustomRule>>,
    /// <p> The default domain for the Amplify app. </p>
    #[serde(rename = "defaultDomain")]
    pub default_domain: String,
    /// <p> The description for the Amplify app. </p>
    #[serde(rename = "description")]
    pub description: String,
    /// <p> Enables automated branch creation for the Amplify app. </p>
    #[serde(rename = "enableAutoBranchCreation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_auto_branch_creation: Option<bool>,
    /// <p> Enables basic authorization for the Amplify app's branches. </p>
    #[serde(rename = "enableBasicAuth")]
    pub enable_basic_auth: bool,
    /// <p> Enables the auto-building of branches for the Amplify app. </p>
    #[serde(rename = "enableBranchAutoBuild")]
    pub enable_branch_auto_build: bool,
    /// <p> Automatically disconnect a branch in the Amplify Console when you delete a branch from your Git repository. </p>
    #[serde(rename = "enableBranchAutoDeletion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_branch_auto_deletion: Option<bool>,
    /// <p> The environment variables for the Amplify app. </p>
    #[serde(rename = "environmentVariables")]
    pub environment_variables: ::std::collections::HashMap<String, String>,
    /// <p> The AWS Identity and Access Management (IAM) service role for the Amazon Resource Name (ARN) of the Amplify app. </p>
    #[serde(rename = "iamServiceRoleArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iam_service_role_arn: Option<String>,
    /// <p> The name for the Amplify app. </p>
    #[serde(rename = "name")]
    pub name: String,
    /// <p> The platform for the Amplify app. </p>
    #[serde(rename = "platform")]
    pub platform: String,
    /// <p> Describes the information about a production branch of the Amplify app. </p>
    #[serde(rename = "productionBranch")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub production_branch: Option<ProductionBranch>,
    /// <p> The repository for the Amplify app. </p>
    #[serde(rename = "repository")]
    pub repository: String,
    /// <p> The tag for the Amplify app. </p>
    #[serde(rename = "tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<::std::collections::HashMap<String, String>>,
    /// <p> Updates the date and time for the Amplify app. </p>
    #[serde(rename = "updateTime")]
    pub update_time: f64,
}

/// <p> Describes an artifact. </p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct Artifact {
    /// <p> The file name for the artifact. </p>
    #[serde(rename = "artifactFileName")]
    pub artifact_file_name: String,
    /// <p> The unique ID for the artifact. </p>
    #[serde(rename = "artifactId")]
    pub artifact_id: String,
}

/// <p> Describes the automated branch creation configuration. </p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct AutoBranchCreationConfig {
    /// <p> The basic authorization credentials for the autocreated branch. </p>
    #[serde(rename = "basicAuthCredentials")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub basic_auth_credentials: Option<String>,
    /// <p> The build specification (build spec) for the autocreated branch. </p>
    #[serde(rename = "buildSpec")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub build_spec: Option<String>,
    /// <p> Enables auto building for the autocreated branch. </p>
    #[serde(rename = "enableAutoBuild")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_auto_build: Option<bool>,
    /// <p> Enables basic authorization for the autocreated branch. </p>
    #[serde(rename = "enableBasicAuth")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_basic_auth: Option<bool>,
    /// <p>Enables performance mode for the branch.</p> <p>Performance mode optimizes for faster hosting performance by keeping content cached at the edge for a longer interval. When performance mode is enabled, hosting configuration or code changes can take up to 10 minutes to roll out. </p>
    #[serde(rename = "enablePerformanceMode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_performance_mode: Option<bool>,
    /// <p> Enables pull request previews for the autocreated branch. </p>
    #[serde(rename = "enablePullRequestPreview")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_pull_request_preview: Option<bool>,
    /// <p> The environment variables for the autocreated branch. </p>
    #[serde(rename = "environmentVariables")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub environment_variables: Option<::std::collections::HashMap<String, String>>,
    /// <p> The framework for the autocreated branch. </p>
    #[serde(rename = "framework")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub framework: Option<String>,
    /// <p> The Amplify environment name for the pull request. </p>
    #[serde(rename = "pullRequestEnvironmentName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pull_request_environment_name: Option<String>,
    /// <p> Describes the current stage for the autocreated branch. </p>
    #[serde(rename = "stage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stage: Option<String>,
}

/// <p> Describes the backend environment for an Amplify app. </p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct BackendEnvironment {
    /// <p> The Amazon Resource Name (ARN) for a backend environment that is part of an Amplify app. </p>
    #[serde(rename = "backendEnvironmentArn")]
    pub backend_environment_arn: String,
    /// <p> The creation date and time for a backend environment that is part of an Amplify app. </p>
    #[serde(rename = "createTime")]
    pub create_time: f64,
    /// <p> The name of deployment artifacts. </p>
    #[serde(rename = "deploymentArtifacts")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deployment_artifacts: Option<String>,
    /// <p> The name for a backend environment that is part of an Amplify app. </p>
    #[serde(rename = "environmentName")]
    pub environment_name: String,
    /// <p> The AWS CloudFormation stack name of a backend environment. </p>
    #[serde(rename = "stackName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stack_name: Option<String>,
    /// <p> The last updated date and time for a backend environment that is part of an Amplify app. </p>
    #[serde(rename = "updateTime")]
    pub update_time: f64,
}

/// <p> The branch for an Amplify app, which maps to a third-party repository branch. </p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct Branch {
    /// <p> The ID of the active job for a branch of an Amplify app. </p>
    #[serde(rename = "activeJobId")]
    pub active_job_id: String,
    /// <p> A list of custom resources that are linked to this branch. </p>
    #[serde(rename = "associatedResources")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub associated_resources: Option<Vec<String>>,
    /// <p> The Amazon Resource Name (ARN) for a backend environment that is part of an Amplify app. </p>
    #[serde(rename = "backendEnvironmentArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backend_environment_arn: Option<String>,
    /// <p> The basic authorization credentials for a branch of an Amplify app. </p>
    #[serde(rename = "basicAuthCredentials")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub basic_auth_credentials: Option<String>,
    /// <p> The Amazon Resource Name (ARN) for a branch that is part of an Amplify app. </p>
    #[serde(rename = "branchArn")]
    pub branch_arn: String,
    /// <p> The name for the branch that is part of an Amplify app. </p>
    #[serde(rename = "branchName")]
    pub branch_name: String,
    /// <p> The build specification (build spec) content for the branch of an Amplify app. </p>
    #[serde(rename = "buildSpec")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub build_spec: Option<String>,
    /// <p> The creation date and time for a branch that is part of an Amplify app. </p>
    #[serde(rename = "createTime")]
    pub create_time: f64,
    /// <p> The custom domains for a branch of an Amplify app. </p>
    #[serde(rename = "customDomains")]
    pub custom_domains: Vec<String>,
    /// <p> The description for the branch that is part of an Amplify app. </p>
    #[serde(rename = "description")]
    pub description: String,
    /// <p> The destination branch if the branch is a pull request branch. </p>
    #[serde(rename = "destinationBranch")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination_branch: Option<String>,
    /// <p> The display name for the branch. This is used as the default domain prefix. </p>
    #[serde(rename = "displayName")]
    pub display_name: String,
    /// <p> Enables auto-building on push for a branch of an Amplify app. </p>
    #[serde(rename = "enableAutoBuild")]
    pub enable_auto_build: bool,
    /// <p> Enables basic authorization for a branch of an Amplify app. </p>
    #[serde(rename = "enableBasicAuth")]
    pub enable_basic_auth: bool,
    /// <p> Enables notifications for a branch that is part of an Amplify app. </p>
    #[serde(rename = "enableNotification")]
    pub enable_notification: bool,
    /// <p>Enables performance mode for the branch.</p> <p>Performance mode optimizes for faster hosting performance by keeping content cached at the edge for a longer interval. When performance mode is enabled, hosting configuration or code changes can take up to 10 minutes to roll out. </p>
    #[serde(rename = "enablePerformanceMode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_performance_mode: Option<bool>,
    /// <p> Enables pull request previews for the branch. </p>
    #[serde(rename = "enablePullRequestPreview")]
    pub enable_pull_request_preview: bool,
    /// <p> The environment variables specific to a branch of an Amplify app. </p>
    #[serde(rename = "environmentVariables")]
    pub environment_variables: ::std::collections::HashMap<String, String>,
    /// <p> The framework for a branch of an Amplify app. </p>
    #[serde(rename = "framework")]
    pub framework: String,
    /// <p> The Amplify environment name for the pull request. </p>
    #[serde(rename = "pullRequestEnvironmentName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pull_request_environment_name: Option<String>,
    /// <p> The source branch if the branch is a pull request branch. </p>
    #[serde(rename = "sourceBranch")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_branch: Option<String>,
    /// <p> The current stage for the branch that is part of an Amplify app. </p>
    #[serde(rename = "stage")]
    pub stage: String,
    /// <p> The tag for the branch of an Amplify app. </p>
    #[serde(rename = "tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<::std::collections::HashMap<String, String>>,
    /// <p> The thumbnail URL for the branch of an Amplify app. </p>
    #[serde(rename = "thumbnailUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumbnail_url: Option<String>,
    /// <p> The total number of jobs that are part of an Amplify app. </p>
    #[serde(rename = "totalNumberOfJobs")]
    pub total_number_of_jobs: String,
    /// <p> The content Time to Live (TTL) for the website in seconds. </p>
    #[serde(rename = "ttl")]
    pub ttl: String,
    /// <p> The last updated date and time for a branch that is part of an Amplify app. </p>
    #[serde(rename = "updateTime")]
    pub update_time: f64,
}

/// <p> The request structure used to create apps in Amplify. </p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateAppRequest {
    /// <p> The personal access token for a third-party source control system for an Amplify app. The personal access token is used to create a webhook and a read-only deploy key. The token is not stored. </p>
    #[serde(rename = "accessToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_token: Option<String>,
    /// <p> The automated branch creation configuration for an Amplify app. </p>
    #[serde(rename = "autoBranchCreationConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_branch_creation_config: Option<AutoBranchCreationConfig>,
    /// <p> The automated branch creation glob patterns for an Amplify app. </p>
    #[serde(rename = "autoBranchCreationPatterns")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_branch_creation_patterns: Option<Vec<String>>,
    /// <p> The credentials for basic authorization for an Amplify app. </p>
    #[serde(rename = "basicAuthCredentials")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub basic_auth_credentials: Option<String>,
    /// <p> The build specification (build spec) for an Amplify app. </p>
    #[serde(rename = "buildSpec")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub build_spec: Option<String>,
    /// <p>The custom HTTP headers for an Amplify app.</p>
    #[serde(rename = "customHeaders")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_headers: Option<String>,
    /// <p> The custom rewrite and redirect rules for an Amplify app. </p>
    #[serde(rename = "customRules")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_rules: Option<Vec<CustomRule>>,
    /// <p> The description for an Amplify app. </p>
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p> Enables automated branch creation for an Amplify app. </p>
    #[serde(rename = "enableAutoBranchCreation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_auto_branch_creation: Option<bool>,
    /// <p> Enables basic authorization for an Amplify app. This will apply to all branches that are part of this app. </p>
    #[serde(rename = "enableBasicAuth")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_basic_auth: Option<bool>,
    /// <p> Enables the auto building of branches for an Amplify app. </p>
    #[serde(rename = "enableBranchAutoBuild")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_branch_auto_build: Option<bool>,
    /// <p> Automatically disconnects a branch in the Amplify Console when you delete a branch from your Git repository. </p>
    #[serde(rename = "enableBranchAutoDeletion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_branch_auto_deletion: Option<bool>,
    /// <p> The environment variables map for an Amplify app. </p>
    #[serde(rename = "environmentVariables")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub environment_variables: Option<::std::collections::HashMap<String, String>>,
    /// <p> The AWS Identity and Access Management (IAM) service role for an Amplify app. </p>
    #[serde(rename = "iamServiceRoleArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iam_service_role_arn: Option<String>,
    /// <p> The name for an Amplify app. </p>
    #[serde(rename = "name")]
    pub name: String,
    /// <p> The OAuth token for a third-party source control system for an Amplify app. The OAuth token is used to create a webhook and a read-only deploy key. The OAuth token is not stored. </p>
    #[serde(rename = "oauthToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub oauth_token: Option<String>,
    /// <p> The platform or framework for an Amplify app. </p>
    #[serde(rename = "platform")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub platform: Option<String>,
    /// <p> The repository for an Amplify app. </p>
    #[serde(rename = "repository")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repository: Option<String>,
    /// <p> The tag for an Amplify app. </p>
    #[serde(rename = "tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<::std::collections::HashMap<String, String>>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateAppResult {
    #[serde(rename = "app")]
    pub app: App,
}

/// <p> The request structure for the backend environment create request. </p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateBackendEnvironmentRequest {
    /// <p> The unique ID for an Amplify app. </p>
    #[serde(rename = "appId")]
    pub app_id: String,
    /// <p> The name of deployment artifacts. </p>
    #[serde(rename = "deploymentArtifacts")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deployment_artifacts: Option<String>,
    /// <p> The name for the backend environment. </p>
    #[serde(rename = "environmentName")]
    pub environment_name: String,
    /// <p> The AWS CloudFormation stack name of a backend environment. </p>
    #[serde(rename = "stackName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stack_name: Option<String>,
}

/// <p> The result structure for the create backend environment request. </p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateBackendEnvironmentResult {
    /// <p> Describes the backend environment for an Amplify app. </p>
    #[serde(rename = "backendEnvironment")]
    pub backend_environment: BackendEnvironment,
}

/// <p> The request structure for the create branch request. </p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateBranchRequest {
    /// <p> The unique ID for an Amplify app. </p>
    #[serde(rename = "appId")]
    pub app_id: String,
    /// <p> The Amazon Resource Name (ARN) for a backend environment that is part of an Amplify app. </p>
    #[serde(rename = "backendEnvironmentArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backend_environment_arn: Option<String>,
    /// <p> The basic authorization credentials for the branch. </p>
    #[serde(rename = "basicAuthCredentials")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub basic_auth_credentials: Option<String>,
    /// <p> The name for the branch. </p>
    #[serde(rename = "branchName")]
    pub branch_name: String,
    /// <p> The build specification (build spec) for the branch. </p>
    #[serde(rename = "buildSpec")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub build_spec: Option<String>,
    /// <p> The description for the branch. </p>
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p> The display name for a branch. This is used as the default domain prefix. </p>
    #[serde(rename = "displayName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    /// <p> Enables auto building for the branch. </p>
    #[serde(rename = "enableAutoBuild")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_auto_build: Option<bool>,
    /// <p> Enables basic authorization for the branch. </p>
    #[serde(rename = "enableBasicAuth")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_basic_auth: Option<bool>,
    /// <p> Enables notifications for the branch. </p>
    #[serde(rename = "enableNotification")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_notification: Option<bool>,
    /// <p>Enables performance mode for the branch.</p> <p>Performance mode optimizes for faster hosting performance by keeping content cached at the edge for a longer interval. When performance mode is enabled, hosting configuration or code changes can take up to 10 minutes to roll out. </p>
    #[serde(rename = "enablePerformanceMode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_performance_mode: Option<bool>,
    /// <p> Enables pull request previews for this branch. </p>
    #[serde(rename = "enablePullRequestPreview")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_pull_request_preview: Option<bool>,
    /// <p> The environment variables for the branch. </p>
    #[serde(rename = "environmentVariables")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub environment_variables: Option<::std::collections::HashMap<String, String>>,
    /// <p> The framework for the branch. </p>
    #[serde(rename = "framework")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub framework: Option<String>,
    /// <p> The Amplify environment name for the pull request. </p>
    #[serde(rename = "pullRequestEnvironmentName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pull_request_environment_name: Option<String>,
    /// <p> Describes the current stage for the branch. </p>
    #[serde(rename = "stage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stage: Option<String>,
    /// <p> The tag for the branch. </p>
    #[serde(rename = "tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<::std::collections::HashMap<String, String>>,
    /// <p> The content Time To Live (TTL) for the website in seconds. </p>
    #[serde(rename = "ttl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ttl: Option<String>,
}

/// <p> The result structure for create branch request. </p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateBranchResult {
    /// <p> Describes the branch for an Amplify app, which maps to a third-party repository branch. </p>
    #[serde(rename = "branch")]
    pub branch: Branch,
}

/// <p> The request structure for the create a new deployment request. </p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateDeploymentRequest {
    /// <p> The unique ID for an Amplify app. </p>
    #[serde(rename = "appId")]
    pub app_id: String,
    /// <p> The name for the branch, for the job. </p>
    #[serde(rename = "branchName")]
    pub branch_name: String,
    /// <p> An optional file map that contains the file name as the key and the file content md5 hash as the value. If this argument is provided, the service will generate a unique upload URL per file. Otherwise, the service will only generate a single upload URL for the zipped files. </p>
    #[serde(rename = "fileMap")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_map: Option<::std::collections::HashMap<String, String>>,
}

/// <p> The result structure for the create a new deployment request. </p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateDeploymentResult {
    /// <p> When the <code>fileMap</code> argument is provided in the request, <code>fileUploadUrls</code> will contain a map of file names to upload URLs. </p>
    #[serde(rename = "fileUploadUrls")]
    pub file_upload_urls: ::std::collections::HashMap<String, String>,
    /// <p> The job ID for this deployment. will supply to start deployment api. </p>
    #[serde(rename = "jobId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_id: Option<String>,
    /// <p> When the <code>fileMap</code> argument is not provided in the request, this <code>zipUploadUrl</code> is returned. </p>
    #[serde(rename = "zipUploadUrl")]
    pub zip_upload_url: String,
}

/// <p> The request structure for the create domain association request. </p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateDomainAssociationRequest {
    /// <p> The unique ID for an Amplify app. </p>
    #[serde(rename = "appId")]
    pub app_id: String,
    /// <p> Sets the branch patterns for automatic subdomain creation. </p>
    #[serde(rename = "autoSubDomainCreationPatterns")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_sub_domain_creation_patterns: Option<Vec<String>>,
    /// <p> The required AWS Identity and Access Management (IAM) service role for the Amazon Resource Name (ARN) for automatically creating subdomains. </p>
    #[serde(rename = "autoSubDomainIAMRole")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_sub_domain_iam_role: Option<String>,
    /// <p> The domain name for the domain association. </p>
    #[serde(rename = "domainName")]
    pub domain_name: String,
    /// <p> Enables the automated creation of subdomains for branches. </p>
    #[serde(rename = "enableAutoSubDomain")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_auto_sub_domain: Option<bool>,
    /// <p> The setting for the subdomain. </p>
    #[serde(rename = "subDomainSettings")]
    pub sub_domain_settings: Vec<SubDomainSetting>,
}

/// <p> The result structure for the create domain association request. </p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateDomainAssociationResult {
    /// <p> Describes the structure of a domain association, which associates a custom domain with an Amplify app. </p>
    #[serde(rename = "domainAssociation")]
    pub domain_association: DomainAssociation,
}

/// <p> The request structure for the create webhook request. </p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateWebhookRequest {
    /// <p> The unique ID for an Amplify app. </p>
    #[serde(rename = "appId")]
    pub app_id: String,
    /// <p> The name for a branch that is part of an Amplify app. </p>
    #[serde(rename = "branchName")]
    pub branch_name: String,
    /// <p> The description for a webhook. </p>
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

/// <p> The result structure for the create webhook request. </p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateWebhookResult {
    /// <p> Describes a webhook that connects repository events to an Amplify app. </p>
    #[serde(rename = "webhook")]
    pub webhook: Webhook,
}

/// <p> Describes a custom rewrite or redirect rule. </p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct CustomRule {
    /// <p> The condition for a URL rewrite or redirect rule, such as a country code. </p>
    #[serde(rename = "condition")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub condition: Option<String>,
    /// <p> The source pattern for a URL rewrite or redirect rule. </p>
    #[serde(rename = "source")]
    pub source: String,
    /// <p><p> The status code for a URL rewrite or redirect rule. </p> <dl> <dt>200</dt> <dd> <p>Represents a 200 rewrite rule.</p> </dd> <dt>301</dt> <dd> <p>Represents a 301 (moved pemanently) redirect rule. This and all future requests should be directed to the target URL. </p> </dd> <dt>302</dt> <dd> <p>Represents a 302 temporary redirect rule.</p> </dd> <dt>404</dt> <dd> <p>Represents a 404 redirect rule.</p> </dd> <dt>404-200</dt> <dd> <p>Represents a 404 rewrite rule.</p> </dd> </dl></p>
    #[serde(rename = "status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// <p> The target pattern for a URL rewrite or redirect rule. </p>
    #[serde(rename = "target")]
    pub target: String,
}

/// <p> Describes the request structure for the delete app request. </p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteAppRequest {
    /// <p> The unique ID for an Amplify app. </p>
    #[serde(rename = "appId")]
    pub app_id: String,
}

/// <p> The result structure for the delete app request. </p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DeleteAppResult {
    #[serde(rename = "app")]
    pub app: App,
}

/// <p> The request structure for the delete backend environment request. </p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteBackendEnvironmentRequest {
    /// <p> The unique ID of an Amplify app. </p>
    #[serde(rename = "appId")]
    pub app_id: String,
    /// <p> The name of a backend environment of an Amplify app. </p>
    #[serde(rename = "environmentName")]
    pub environment_name: String,
}

/// <p> The result structure of the delete backend environment result. </p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DeleteBackendEnvironmentResult {
    /// <p> Describes the backend environment for an Amplify app. </p>
    #[serde(rename = "backendEnvironment")]
    pub backend_environment: BackendEnvironment,
}

/// <p> The request structure for the delete branch request. </p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteBranchRequest {
    /// <p> The unique ID for an Amplify app. </p>
    #[serde(rename = "appId")]
    pub app_id: String,
    /// <p> The name for the branch. </p>
    #[serde(rename = "branchName")]
    pub branch_name: String,
}

/// <p> The result structure for the delete branch request. </p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DeleteBranchResult {
    /// <p> The branch for an Amplify app, which maps to a third-party repository branch. </p>
    #[serde(rename = "branch")]
    pub branch: Branch,
}

/// <p> The request structure for the delete domain association request. </p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteDomainAssociationRequest {
    /// <p> The unique id for an Amplify app. </p>
    #[serde(rename = "appId")]
    pub app_id: String,
    /// <p> The name of the domain. </p>
    #[serde(rename = "domainName")]
    pub domain_name: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DeleteDomainAssociationResult {
    #[serde(rename = "domainAssociation")]
    pub domain_association: DomainAssociation,
}

/// <p> The request structure for the delete job request. </p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteJobRequest {
    /// <p> The unique ID for an Amplify app. </p>
    #[serde(rename = "appId")]
    pub app_id: String,
    /// <p> The name for the branch, for the job. </p>
    #[serde(rename = "branchName")]
    pub branch_name: String,
    /// <p> The unique ID for the job. </p>
    #[serde(rename = "jobId")]
    pub job_id: String,
}

/// <p> The result structure for the delete job request. </p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DeleteJobResult {
    #[serde(rename = "jobSummary")]
    pub job_summary: JobSummary,
}

/// <p> The request structure for the delete webhook request. </p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteWebhookRequest {
    /// <p> The unique ID for a webhook. </p>
    #[serde(rename = "webhookId")]
    pub webhook_id: String,
}

/// <p> The result structure for the delete webhook request. </p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DeleteWebhookResult {
    /// <p> Describes a webhook that connects repository events to an Amplify app. </p>
    #[serde(rename = "webhook")]
    pub webhook: Webhook,
}

/// <p> Describes a domain association that associates a custom domain with an Amplify app. </p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DomainAssociation {
    /// <p> Sets branch patterns for automatic subdomain creation. </p>
    #[serde(rename = "autoSubDomainCreationPatterns")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_sub_domain_creation_patterns: Option<Vec<String>>,
    /// <p> The required AWS Identity and Access Management (IAM) service role for the Amazon Resource Name (ARN) for automatically creating subdomains. </p>
    #[serde(rename = "autoSubDomainIAMRole")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_sub_domain_iam_role: Option<String>,
    /// <p> The DNS record for certificate verification. </p>
    #[serde(rename = "certificateVerificationDNSRecord")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate_verification_dns_record: Option<String>,
    /// <p> The Amazon Resource Name (ARN) for the domain association. </p>
    #[serde(rename = "domainAssociationArn")]
    pub domain_association_arn: String,
    /// <p> The name of the domain. </p>
    #[serde(rename = "domainName")]
    pub domain_name: String,
    /// <p> The current status of the domain association. </p>
    #[serde(rename = "domainStatus")]
    pub domain_status: String,
    /// <p> Enables the automated creation of subdomains for branches. </p>
    #[serde(rename = "enableAutoSubDomain")]
    pub enable_auto_sub_domain: bool,
    /// <p> The reason for the current status of the domain association. </p>
    #[serde(rename = "statusReason")]
    pub status_reason: String,
    /// <p> The subdomains for the domain association. </p>
    #[serde(rename = "subDomains")]
    pub sub_domains: Vec<SubDomain>,
}

/// <p> The request structure for the generate access logs request. </p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GenerateAccessLogsRequest {
    /// <p> The unique ID for an Amplify app. </p>
    #[serde(rename = "appId")]
    pub app_id: String,
    /// <p> The name of the domain. </p>
    #[serde(rename = "domainName")]
    pub domain_name: String,
    /// <p> The time at which the logs should end. The time range specified is inclusive of the end time. </p>
    #[serde(rename = "endTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<f64>,
    /// <p> The time at which the logs should start. The time range specified is inclusive of the start time. </p>
    #[serde(rename = "startTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<f64>,
}

/// <p> The result structure for the generate access logs request. </p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GenerateAccessLogsResult {
    /// <p> The pre-signed URL for the requested access logs. </p>
    #[serde(rename = "logUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_url: Option<String>,
}

/// <p> The request structure for the get app request. </p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetAppRequest {
    /// <p> The unique ID for an Amplify app. </p>
    #[serde(rename = "appId")]
    pub app_id: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetAppResult {
    #[serde(rename = "app")]
    pub app: App,
}

/// <p> Returns the request structure for the get artifact request. </p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetArtifactUrlRequest {
    /// <p> The unique ID for an artifact. </p>
    #[serde(rename = "artifactId")]
    pub artifact_id: String,
}

/// <p> Returns the result structure for the get artifact request. </p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetArtifactUrlResult {
    /// <p> The unique ID for an artifact. </p>
    #[serde(rename = "artifactId")]
    pub artifact_id: String,
    /// <p> The presigned URL for the artifact. </p>
    #[serde(rename = "artifactUrl")]
    pub artifact_url: String,
}

/// <p> The request structure for the get backend environment request. </p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetBackendEnvironmentRequest {
    /// <p> The unique id for an Amplify app. </p>
    #[serde(rename = "appId")]
    pub app_id: String,
    /// <p> The name for the backend environment. </p>
    #[serde(rename = "environmentName")]
    pub environment_name: String,
}

/// <p> The result structure for the get backend environment result. </p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetBackendEnvironmentResult {
    /// <p> Describes the backend environment for an Amplify app. </p>
    #[serde(rename = "backendEnvironment")]
    pub backend_environment: BackendEnvironment,
}

/// <p> The request structure for the get branch request. </p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetBranchRequest {
    /// <p> The unique ID for an Amplify app. </p>
    #[serde(rename = "appId")]
    pub app_id: String,
    /// <p> The name for the branch. </p>
    #[serde(rename = "branchName")]
    pub branch_name: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetBranchResult {
    #[serde(rename = "branch")]
    pub branch: Branch,
}

/// <p> The request structure for the get domain association request. </p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetDomainAssociationRequest {
    /// <p> The unique id for an Amplify app. </p>
    #[serde(rename = "appId")]
    pub app_id: String,
    /// <p> The name of the domain. </p>
    #[serde(rename = "domainName")]
    pub domain_name: String,
}

/// <p> The result structure for the get domain association request. </p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetDomainAssociationResult {
    /// <p> Describes the structure of a domain association, which associates a custom domain with an Amplify app. </p>
    #[serde(rename = "domainAssociation")]
    pub domain_association: DomainAssociation,
}

/// <p> The request structure for the get job request. </p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetJobRequest {
    /// <p> The unique ID for an Amplify app. </p>
    #[serde(rename = "appId")]
    pub app_id: String,
    /// <p> The branch name for the job. </p>
    #[serde(rename = "branchName")]
    pub branch_name: String,
    /// <p> The unique ID for the job. </p>
    #[serde(rename = "jobId")]
    pub job_id: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetJobResult {
    #[serde(rename = "job")]
    pub job: Job,
}

/// <p> The request structure for the get webhook request. </p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetWebhookRequest {
    /// <p> The unique ID for a webhook. </p>
    #[serde(rename = "webhookId")]
    pub webhook_id: String,
}

/// <p> The result structure for the get webhook request. </p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetWebhookResult {
    /// <p> Describes the structure of a webhook. </p>
    #[serde(rename = "webhook")]
    pub webhook: Webhook,
}

/// <p> Describes an execution job for an Amplify app. </p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct Job {
    /// <p> The execution steps for an execution job, for an Amplify app. </p>
    #[serde(rename = "steps")]
    pub steps: Vec<Step>,
    /// <p> Describes the summary for an execution job for an Amplify app. </p>
    #[serde(rename = "summary")]
    pub summary: JobSummary,
}

/// <p> Describes the summary for an execution job for an Amplify app. </p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct JobSummary {
    /// <p> The commit ID from a third-party repository provider for the job. </p>
    #[serde(rename = "commitId")]
    pub commit_id: String,
    /// <p> The commit message from a third-party repository provider for the job. </p>
    #[serde(rename = "commitMessage")]
    pub commit_message: String,
    /// <p> The commit date and time for the job. </p>
    #[serde(rename = "commitTime")]
    pub commit_time: f64,
    /// <p> The end date and time for the job. </p>
    #[serde(rename = "endTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<f64>,
    /// <p> The Amazon Resource Name (ARN) for the job. </p>
    #[serde(rename = "jobArn")]
    pub job_arn: String,
    /// <p> The unique ID for the job. </p>
    #[serde(rename = "jobId")]
    pub job_id: String,
    /// <p> The type for the job. If the value is <code>RELEASE</code>, the job was manually released from its source by using the <code>StartJob</code> API. If the value is <code>RETRY</code>, the job was manually retried using the <code>StartJob</code> API. If the value is <code>WEB_HOOK</code>, the job was automatically triggered by webhooks. </p>
    #[serde(rename = "jobType")]
    pub job_type: String,
    /// <p> The start date and time for the job. </p>
    #[serde(rename = "startTime")]
    pub start_time: f64,
    /// <p> The current status for the job. </p>
    #[serde(rename = "status")]
    pub status: String,
}

/// <p> The request structure for the list apps request. </p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListAppsRequest {
    /// <p> The maximum number of records to list in a single response. </p>
    #[serde(rename = "maxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p> A pagination token. If non-null, the pagination token is returned in a result. Pass its value in another request to retrieve more entries. </p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

/// <p> The result structure for an Amplify app list request. </p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListAppsResult {
    /// <p> A list of Amplify apps. </p>
    #[serde(rename = "apps")]
    pub apps: Vec<App>,
    /// <p> A pagination token. Set to null to start listing apps from start. If non-null, the pagination token is returned in a result. Pass its value in here to list more projects. </p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

/// <p> Describes the request structure for the list artifacts request. </p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListArtifactsRequest {
    /// <p> The unique ID for an Amplify app. </p>
    #[serde(rename = "appId")]
    pub app_id: String,
    /// <p> The name of a branch that is part of an Amplify app. </p>
    #[serde(rename = "branchName")]
    pub branch_name: String,
    /// <p> The unique ID for a job. </p>
    #[serde(rename = "jobId")]
    pub job_id: String,
    /// <p> The maximum number of records to list in a single response. </p>
    #[serde(rename = "maxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p> A pagination token. Set to null to start listing artifacts from start. If a non-null pagination token is returned in a result, pass its value in here to list more artifacts. </p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

/// <p> The result structure for the list artifacts request. </p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListArtifactsResult {
    /// <p> A list of artifacts. </p>
    #[serde(rename = "artifacts")]
    pub artifacts: Vec<Artifact>,
    /// <p> A pagination token. If a non-null pagination token is returned in a result, pass its value in another request to retrieve more entries. </p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

/// <p> The request structure for the list backend environments request. </p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListBackendEnvironmentsRequest {
    /// <p> The unique ID for an Amplify app. </p>
    #[serde(rename = "appId")]
    pub app_id: String,
    /// <p> The name of the backend environment </p>
    #[serde(rename = "environmentName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub environment_name: Option<String>,
    /// <p> The maximum number of records to list in a single response. </p>
    #[serde(rename = "maxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p> A pagination token. Set to null to start listing backend environments from the start. If a non-null pagination token is returned in a result, pass its value in here to list more backend environments. </p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

/// <p> The result structure for the list backend environments result. </p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListBackendEnvironmentsResult {
    /// <p> The list of backend environments for an Amplify app. </p>
    #[serde(rename = "backendEnvironments")]
    pub backend_environments: Vec<BackendEnvironment>,
    /// <p> A pagination token. If a non-null pagination token is returned in a result, pass its value in another request to retrieve more entries. </p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

/// <p> The request structure for the list branches request. </p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListBranchesRequest {
    /// <p> The unique ID for an Amplify app. </p>
    #[serde(rename = "appId")]
    pub app_id: String,
    /// <p> The maximum number of records to list in a single response. </p>
    #[serde(rename = "maxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p> A pagination token. Set to null to start listing branches from the start. If a non-null pagination token is returned in a result, pass its value in here to list more branches. </p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

/// <p> The result structure for the list branches request. </p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListBranchesResult {
    /// <p> A list of branches for an Amplify app. </p>
    #[serde(rename = "branches")]
    pub branches: Vec<Branch>,
    /// <p> A pagination token. If a non-null pagination token is returned in a result, pass its value in another request to retrieve more entries. </p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

/// <p> The request structure for the list domain associations request. </p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListDomainAssociationsRequest {
    /// <p> The unique ID for an Amplify app. </p>
    #[serde(rename = "appId")]
    pub app_id: String,
    /// <p> The maximum number of records to list in a single response. </p>
    #[serde(rename = "maxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p> A pagination token. Set to null to start listing apps from the start. If non-null, a pagination token is returned in a result. Pass its value in here to list more projects. </p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

/// <p> The result structure for the list domain association request. </p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListDomainAssociationsResult {
    /// <p> A list of domain associations. </p>
    #[serde(rename = "domainAssociations")]
    pub domain_associations: Vec<DomainAssociation>,
    /// <p> A pagination token. If non-null, a pagination token is returned in a result. Pass its value in another request to retrieve more entries. </p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

/// <p> The request structure for the list jobs request. </p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListJobsRequest {
    /// <p> The unique ID for an Amplify app. </p>
    #[serde(rename = "appId")]
    pub app_id: String,
    /// <p> The name for a branch. </p>
    #[serde(rename = "branchName")]
    pub branch_name: String,
    /// <p> The maximum number of records to list in a single response. </p>
    #[serde(rename = "maxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p> A pagination token. Set to null to start listing steps from the start. If a non-null pagination token is returned in a result, pass its value in here to list more steps. </p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

/// <p> The maximum number of records to list in a single response. </p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListJobsResult {
    /// <p> The result structure for the list job result request. </p>
    #[serde(rename = "jobSummaries")]
    pub job_summaries: Vec<JobSummary>,
    /// <p> A pagination token. If non-null the pagination token is returned in a result. Pass its value in another request to retrieve more entries. </p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

/// <p> The request structure to use to list tags for a resource. </p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListTagsForResourceRequest {
    /// <p> The Amazon Resource Name (ARN) to use to list tags. </p>
    #[serde(rename = "resourceArn")]
    pub resource_arn: String,
}

/// <p> The response for the list tags for resource request. </p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListTagsForResourceResponse {
    /// <p> A list of tags for the specified The Amazon Resource Name (ARN). </p>
    #[serde(rename = "tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<::std::collections::HashMap<String, String>>,
}

/// <p> The request structure for the list webhooks request. </p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListWebhooksRequest {
    /// <p> The unique ID for an Amplify app. </p>
    #[serde(rename = "appId")]
    pub app_id: String,
    /// <p> The maximum number of records to list in a single response. </p>
    #[serde(rename = "maxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p> A pagination token. Set to null to start listing webhooks from the start. If non-null,the pagination token is returned in a result. Pass its value in here to list more webhooks. </p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

/// <p> The result structure for the list webhooks request. </p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListWebhooksResult {
    /// <p> A pagination token. If non-null, the pagination token is returned in a result. Pass its value in another request to retrieve more entries. </p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p> A list of webhooks. </p>
    #[serde(rename = "webhooks")]
    pub webhooks: Vec<Webhook>,
}

/// <p> Describes the information about a production branch for an Amplify app. </p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ProductionBranch {
    /// <p> The branch name for the production branch. </p>
    #[serde(rename = "branchName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub branch_name: Option<String>,
    /// <p> The last deploy time of the production branch. </p>
    #[serde(rename = "lastDeployTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_deploy_time: Option<f64>,
    /// <p> The status of the production branch. </p>
    #[serde(rename = "status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// <p> The thumbnail URL for the production branch. </p>
    #[serde(rename = "thumbnailUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumbnail_url: Option<String>,
}

/// <p> The request structure for the start a deployment request. </p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct StartDeploymentRequest {
    /// <p> The unique ID for an Amplify app. </p>
    #[serde(rename = "appId")]
    pub app_id: String,
    /// <p> The name for the branch, for the job. </p>
    #[serde(rename = "branchName")]
    pub branch_name: String,
    /// <p> The job ID for this deployment, generated by the create deployment request. </p>
    #[serde(rename = "jobId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_id: Option<String>,
    /// <p> The source URL for this deployment, used when calling start deployment without create deployment. The source URL can be any HTTP GET URL that is publicly accessible and downloads a single .zip file. </p>
    #[serde(rename = "sourceUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_url: Option<String>,
}

/// <p> The result structure for the start a deployment request. </p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct StartDeploymentResult {
    /// <p> The summary for the job. </p>
    #[serde(rename = "jobSummary")]
    pub job_summary: JobSummary,
}

/// <p> The request structure for the start job request. </p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct StartJobRequest {
    /// <p> The unique ID for an Amplify app. </p>
    #[serde(rename = "appId")]
    pub app_id: String,
    /// <p> The branch name for the job. </p>
    #[serde(rename = "branchName")]
    pub branch_name: String,
    /// <p> The commit ID from a third-party repository provider for the job. </p>
    #[serde(rename = "commitId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub commit_id: Option<String>,
    /// <p> The commit message from a third-party repository provider for the job. </p>
    #[serde(rename = "commitMessage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub commit_message: Option<String>,
    /// <p> The commit date and time for the job. </p>
    #[serde(rename = "commitTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub commit_time: Option<f64>,
    /// <p> The unique ID for an existing job. This is required if the value of <code>jobType</code> is <code>RETRY</code>. </p>
    #[serde(rename = "jobId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_id: Option<String>,
    /// <p> A descriptive reason for starting this job. </p>
    #[serde(rename = "jobReason")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_reason: Option<String>,
    /// <p> Describes the type for the job. The job type <code>RELEASE</code> starts a new job with the latest change from the specified branch. This value is available only for apps that are connected to a repository. The job type <code>RETRY</code> retries an existing job. If the job type value is <code>RETRY</code>, the <code>jobId</code> is also required. </p>
    #[serde(rename = "jobType")]
    pub job_type: String,
}

/// <p> The result structure for the run job request. </p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct StartJobResult {
    /// <p> The summary for the job. </p>
    #[serde(rename = "jobSummary")]
    pub job_summary: JobSummary,
}

/// <p> Describes an execution step, for an execution job, for an Amplify app. </p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct Step {
    /// <p> The URL to the artifact for the execution step. </p>
    #[serde(rename = "artifactsUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub artifacts_url: Option<String>,
    /// <p> The context for the current step. Includes a build image if the step is build. </p>
    #[serde(rename = "context")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub context: Option<String>,
    /// <p> The end date and time of the execution step. </p>
    #[serde(rename = "endTime")]
    pub end_time: f64,
    /// <p> The URL to the logs for the execution step. </p>
    #[serde(rename = "logUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_url: Option<String>,
    /// <p> The list of screenshot URLs for the execution step, if relevant. </p>
    #[serde(rename = "screenshots")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub screenshots: Option<::std::collections::HashMap<String, String>>,
    /// <p> The start date and time of the execution step. </p>
    #[serde(rename = "startTime")]
    pub start_time: f64,
    /// <p> The status of the execution step. </p>
    #[serde(rename = "status")]
    pub status: String,
    /// <p> The reason for the current step status. </p>
    #[serde(rename = "statusReason")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_reason: Option<String>,
    /// <p> The name of the execution step. </p>
    #[serde(rename = "stepName")]
    pub step_name: String,
    /// <p> The URL to the test artifact for the execution step. </p>
    #[serde(rename = "testArtifactsUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub test_artifacts_url: Option<String>,
    /// <p> The URL to the test configuration for the execution step. </p>
    #[serde(rename = "testConfigUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub test_config_url: Option<String>,
}

/// <p> The request structure for the stop job request. </p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct StopJobRequest {
    /// <p> The unique ID for an Amplify app. </p>
    #[serde(rename = "appId")]
    pub app_id: String,
    /// <p> The name for the branch, for the job. </p>
    #[serde(rename = "branchName")]
    pub branch_name: String,
    /// <p> The unique id for the job. </p>
    #[serde(rename = "jobId")]
    pub job_id: String,
}

/// <p> The result structure for the stop job request. </p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct StopJobResult {
    /// <p> The summary for the job. </p>
    #[serde(rename = "jobSummary")]
    pub job_summary: JobSummary,
}

/// <p> The subdomain for the domain association. </p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct SubDomain {
    /// <p> The DNS record for the subdomain. </p>
    #[serde(rename = "dnsRecord")]
    pub dns_record: String,
    /// <p> Describes the settings for the subdomain. </p>
    #[serde(rename = "subDomainSetting")]
    pub sub_domain_setting: SubDomainSetting,
    /// <p> The verified status of the subdomain </p>
    #[serde(rename = "verified")]
    pub verified: bool,
}

/// <p> Describes the settings for the subdomain. </p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct SubDomainSetting {
    /// <p> The branch name setting for the subdomain. </p>
    #[serde(rename = "branchName")]
    pub branch_name: String,
    /// <p> The prefix setting for the subdomain. </p>
    #[serde(rename = "prefix")]
    pub prefix: String,
}

/// <p> The request structure to tag a resource with a tag key and value. </p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct TagResourceRequest {
    /// <p> The Amazon Resource Name (ARN) to use to tag a resource. </p>
    #[serde(rename = "resourceArn")]
    pub resource_arn: String,
    /// <p> The tags used to tag the resource. </p>
    #[serde(rename = "tags")]
    pub tags: ::std::collections::HashMap<String, String>,
}

/// <p> The response for the tag resource request. </p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct TagResourceResponse {}

/// <p> The request structure for the untag resource request. </p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UntagResourceRequest {
    /// <p> The Amazon Resource Name (ARN) to use to untag a resource. </p>
    #[serde(rename = "resourceArn")]
    pub resource_arn: String,
    /// <p> The tag keys to use to untag a resource. </p>
    #[serde(rename = "tagKeys")]
    pub tag_keys: Vec<String>,
}

/// <p> The response for the untag resource request. </p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UntagResourceResponse {}

/// <p> The request structure for the update app request. </p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UpdateAppRequest {
    /// <p> The personal access token for a third-party source control system for an Amplify app. The token is used to create webhook and a read-only deploy key. The token is not stored. </p>
    #[serde(rename = "accessToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_token: Option<String>,
    /// <p> The unique ID for an Amplify app. </p>
    #[serde(rename = "appId")]
    pub app_id: String,
    /// <p> The automated branch creation configuration for an Amplify app. </p>
    #[serde(rename = "autoBranchCreationConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_branch_creation_config: Option<AutoBranchCreationConfig>,
    /// <p> Describes the automated branch creation glob patterns for an Amplify app. </p>
    #[serde(rename = "autoBranchCreationPatterns")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_branch_creation_patterns: Option<Vec<String>>,
    /// <p> The basic authorization credentials for an Amplify app. </p>
    #[serde(rename = "basicAuthCredentials")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub basic_auth_credentials: Option<String>,
    /// <p> The build specification (build spec) for an Amplify app. </p>
    #[serde(rename = "buildSpec")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub build_spec: Option<String>,
    /// <p>The custom HTTP headers for an Amplify app.</p>
    #[serde(rename = "customHeaders")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_headers: Option<String>,
    /// <p> The custom redirect and rewrite rules for an Amplify app. </p>
    #[serde(rename = "customRules")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_rules: Option<Vec<CustomRule>>,
    /// <p> The description for an Amplify app. </p>
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p> Enables automated branch creation for an Amplify app. </p>
    #[serde(rename = "enableAutoBranchCreation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_auto_branch_creation: Option<bool>,
    /// <p> Enables basic authorization for an Amplify app. </p>
    #[serde(rename = "enableBasicAuth")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_basic_auth: Option<bool>,
    /// <p> Enables branch auto-building for an Amplify app. </p>
    #[serde(rename = "enableBranchAutoBuild")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_branch_auto_build: Option<bool>,
    /// <p> Automatically disconnects a branch in the Amplify Console when you delete a branch from your Git repository. </p>
    #[serde(rename = "enableBranchAutoDeletion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_branch_auto_deletion: Option<bool>,
    /// <p> The environment variables for an Amplify app. </p>
    #[serde(rename = "environmentVariables")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub environment_variables: Option<::std::collections::HashMap<String, String>>,
    /// <p> The AWS Identity and Access Management (IAM) service role for an Amplify app. </p>
    #[serde(rename = "iamServiceRoleArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iam_service_role_arn: Option<String>,
    /// <p> The name for an Amplify app. </p>
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p> The OAuth token for a third-party source control system for an Amplify app. The token is used to create a webhook and a read-only deploy key. The OAuth token is not stored. </p>
    #[serde(rename = "oauthToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub oauth_token: Option<String>,
    /// <p> The platform for an Amplify app. </p>
    #[serde(rename = "platform")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub platform: Option<String>,
    /// <p> The name of the repository for an Amplify app </p>
    #[serde(rename = "repository")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repository: Option<String>,
}

/// <p> The result structure for an Amplify app update request. </p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UpdateAppResult {
    /// <p> Represents the updated Amplify app. </p>
    #[serde(rename = "app")]
    pub app: App,
}

/// <p> The request structure for the update branch request. </p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UpdateBranchRequest {
    /// <p> The unique ID for an Amplify app. </p>
    #[serde(rename = "appId")]
    pub app_id: String,
    /// <p> The Amazon Resource Name (ARN) for a backend environment that is part of an Amplify app. </p>
    #[serde(rename = "backendEnvironmentArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backend_environment_arn: Option<String>,
    /// <p> The basic authorization credentials for the branch. </p>
    #[serde(rename = "basicAuthCredentials")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub basic_auth_credentials: Option<String>,
    /// <p> The name for the branch. </p>
    #[serde(rename = "branchName")]
    pub branch_name: String,
    /// <p> The build specification (build spec) for the branch. </p>
    #[serde(rename = "buildSpec")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub build_spec: Option<String>,
    /// <p> The description for the branch. </p>
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p> The display name for a branch. This is used as the default domain prefix. </p>
    #[serde(rename = "displayName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    /// <p> Enables auto building for the branch. </p>
    #[serde(rename = "enableAutoBuild")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_auto_build: Option<bool>,
    /// <p> Enables basic authorization for the branch. </p>
    #[serde(rename = "enableBasicAuth")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_basic_auth: Option<bool>,
    /// <p> Enables notifications for the branch. </p>
    #[serde(rename = "enableNotification")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_notification: Option<bool>,
    /// <p>Enables performance mode for the branch.</p> <p>Performance mode optimizes for faster hosting performance by keeping content cached at the edge for a longer interval. When performance mode is enabled, hosting configuration or code changes can take up to 10 minutes to roll out. </p>
    #[serde(rename = "enablePerformanceMode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_performance_mode: Option<bool>,
    /// <p> Enables pull request previews for this branch. </p>
    #[serde(rename = "enablePullRequestPreview")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_pull_request_preview: Option<bool>,
    /// <p> The environment variables for the branch. </p>
    #[serde(rename = "environmentVariables")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub environment_variables: Option<::std::collections::HashMap<String, String>>,
    /// <p> The framework for the branch. </p>
    #[serde(rename = "framework")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub framework: Option<String>,
    /// <p> The Amplify environment name for the pull request. </p>
    #[serde(rename = "pullRequestEnvironmentName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pull_request_environment_name: Option<String>,
    /// <p> Describes the current stage for the branch. </p>
    #[serde(rename = "stage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stage: Option<String>,
    /// <p> The content Time to Live (TTL) for the website in seconds. </p>
    #[serde(rename = "ttl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ttl: Option<String>,
}

/// <p> The result structure for the update branch request. </p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UpdateBranchResult {
    /// <p> The branch for an Amplify app, which maps to a third-party repository branch. </p>
    #[serde(rename = "branch")]
    pub branch: Branch,
}

/// <p> The request structure for the update domain association request. </p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UpdateDomainAssociationRequest {
    /// <p> The unique ID for an Amplify app. </p>
    #[serde(rename = "appId")]
    pub app_id: String,
    /// <p> Sets the branch patterns for automatic subdomain creation. </p>
    #[serde(rename = "autoSubDomainCreationPatterns")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_sub_domain_creation_patterns: Option<Vec<String>>,
    /// <p> The required AWS Identity and Access Management (IAM) service role for the Amazon Resource Name (ARN) for automatically creating subdomains. </p>
    #[serde(rename = "autoSubDomainIAMRole")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_sub_domain_iam_role: Option<String>,
    /// <p> The name of the domain. </p>
    #[serde(rename = "domainName")]
    pub domain_name: String,
    /// <p> Enables the automated creation of subdomains for branches. </p>
    #[serde(rename = "enableAutoSubDomain")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_auto_sub_domain: Option<bool>,
    /// <p> Describes the settings for the subdomain. </p>
    #[serde(rename = "subDomainSettings")]
    pub sub_domain_settings: Vec<SubDomainSetting>,
}

/// <p> The result structure for the update domain association request. </p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UpdateDomainAssociationResult {
    /// <p> Describes a domain association, which associates a custom domain with an Amplify app. </p>
    #[serde(rename = "domainAssociation")]
    pub domain_association: DomainAssociation,
}

/// <p> The request structure for the update webhook request. </p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UpdateWebhookRequest {
    /// <p> The name for a branch that is part of an Amplify app. </p>
    #[serde(rename = "branchName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub branch_name: Option<String>,
    /// <p> The description for a webhook. </p>
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p> The unique ID for a webhook. </p>
    #[serde(rename = "webhookId")]
    pub webhook_id: String,
}

/// <p> The result structure for the update webhook request. </p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UpdateWebhookResult {
    /// <p> Describes a webhook that connects repository events to an Amplify app. </p>
    #[serde(rename = "webhook")]
    pub webhook: Webhook,
}

/// <p> Describes a webhook that connects repository events to an Amplify app. </p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct Webhook {
    /// <p> The name for a branch that is part of an Amplify app. </p>
    #[serde(rename = "branchName")]
    pub branch_name: String,
    /// <p> The create date and time for a webhook. </p>
    #[serde(rename = "createTime")]
    pub create_time: f64,
    /// <p> The description for a webhook. </p>
    #[serde(rename = "description")]
    pub description: String,
    /// <p> Updates the date and time for a webhook. </p>
    #[serde(rename = "updateTime")]
    pub update_time: f64,
    /// <p> The Amazon Resource Name (ARN) for the webhook. </p>
    #[serde(rename = "webhookArn")]
    pub webhook_arn: String,
    /// <p> The ID of the webhook. </p>
    #[serde(rename = "webhookId")]
    pub webhook_id: String,
    /// <p> The URL of the webhook. </p>
    #[serde(rename = "webhookUrl")]
    pub webhook_url: String,
}

/// Errors returned by CreateApp
#[derive(Debug, PartialEq)]
pub enum CreateAppError {
    /// <p> A request contains unexpected data. </p>
    BadRequest(String),
    /// <p> An operation failed because a dependent service threw an exception. </p>
    DependentServiceFailure(String),
    /// <p> The service failed to perform an operation due to an internal issue. </p>
    InternalFailure(String),
    /// <p> A resource could not be created because service quotas were exceeded. </p>
    LimitExceeded(String),
    /// <p> An operation failed due to a lack of access. </p>
    Unauthorized(String),
}

impl CreateAppError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateAppError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            #[allow(clippy::single_match)]
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(CreateAppError::BadRequest(err.msg))
                }
                "DependentServiceFailureException" => {
                    return RusotoError::Service(CreateAppError::DependentServiceFailure(err.msg))
                }
                "InternalFailureException" => {
                    return RusotoError::Service(CreateAppError::InternalFailure(err.msg))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(CreateAppError::LimitExceeded(err.msg))
                }
                "UnauthorizedException" => {
                    return RusotoError::Service(CreateAppError::Unauthorized(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for CreateAppError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreateAppError::BadRequest(ref cause) => write!(f, "{}", cause),
            CreateAppError::DependentServiceFailure(ref cause) => write!(f, "{}", cause),
            CreateAppError::InternalFailure(ref cause) => write!(f, "{}", cause),
            CreateAppError::LimitExceeded(ref cause) => write!(f, "{}", cause),
            CreateAppError::Unauthorized(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CreateAppError {}
/// Errors returned by CreateBackendEnvironment
#[derive(Debug, PartialEq)]
pub enum CreateBackendEnvironmentError {
    /// <p> A request contains unexpected data. </p>
    BadRequest(String),
    /// <p> The service failed to perform an operation due to an internal issue. </p>
    InternalFailure(String),
    /// <p> A resource could not be created because service quotas were exceeded. </p>
    LimitExceeded(String),
    /// <p> An entity was not found during an operation. </p>
    NotFound(String),
    /// <p> An operation failed due to a lack of access. </p>
    Unauthorized(String),
}

impl CreateBackendEnvironmentError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateBackendEnvironmentError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            #[allow(clippy::single_match)]
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(CreateBackendEnvironmentError::BadRequest(err.msg))
                }
                "InternalFailureException" => {
                    return RusotoError::Service(CreateBackendEnvironmentError::InternalFailure(
                        err.msg,
                    ))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(CreateBackendEnvironmentError::LimitExceeded(
                        err.msg,
                    ))
                }
                "NotFoundException" => {
                    return RusotoError::Service(CreateBackendEnvironmentError::NotFound(err.msg))
                }
                "UnauthorizedException" => {
                    return RusotoError::Service(CreateBackendEnvironmentError::Unauthorized(
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
impl fmt::Display for CreateBackendEnvironmentError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreateBackendEnvironmentError::BadRequest(ref cause) => write!(f, "{}", cause),
            CreateBackendEnvironmentError::InternalFailure(ref cause) => write!(f, "{}", cause),
            CreateBackendEnvironmentError::LimitExceeded(ref cause) => write!(f, "{}", cause),
            CreateBackendEnvironmentError::NotFound(ref cause) => write!(f, "{}", cause),
            CreateBackendEnvironmentError::Unauthorized(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CreateBackendEnvironmentError {}
/// Errors returned by CreateBranch
#[derive(Debug, PartialEq)]
pub enum CreateBranchError {
    /// <p> A request contains unexpected data. </p>
    BadRequest(String),
    /// <p> An operation failed because a dependent service threw an exception. </p>
    DependentServiceFailure(String),
    /// <p> The service failed to perform an operation due to an internal issue. </p>
    InternalFailure(String),
    /// <p> A resource could not be created because service quotas were exceeded. </p>
    LimitExceeded(String),
    /// <p> An entity was not found during an operation. </p>
    NotFound(String),
    /// <p> An operation failed due to a lack of access. </p>
    Unauthorized(String),
}

impl CreateBranchError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateBranchError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            #[allow(clippy::single_match)]
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(CreateBranchError::BadRequest(err.msg))
                }
                "DependentServiceFailureException" => {
                    return RusotoError::Service(CreateBranchError::DependentServiceFailure(
                        err.msg,
                    ))
                }
                "InternalFailureException" => {
                    return RusotoError::Service(CreateBranchError::InternalFailure(err.msg))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(CreateBranchError::LimitExceeded(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(CreateBranchError::NotFound(err.msg))
                }
                "UnauthorizedException" => {
                    return RusotoError::Service(CreateBranchError::Unauthorized(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for CreateBranchError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreateBranchError::BadRequest(ref cause) => write!(f, "{}", cause),
            CreateBranchError::DependentServiceFailure(ref cause) => write!(f, "{}", cause),
            CreateBranchError::InternalFailure(ref cause) => write!(f, "{}", cause),
            CreateBranchError::LimitExceeded(ref cause) => write!(f, "{}", cause),
            CreateBranchError::NotFound(ref cause) => write!(f, "{}", cause),
            CreateBranchError::Unauthorized(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CreateBranchError {}
/// Errors returned by CreateDeployment
#[derive(Debug, PartialEq)]
pub enum CreateDeploymentError {
    /// <p> A request contains unexpected data. </p>
    BadRequest(String),
    /// <p> The service failed to perform an operation due to an internal issue. </p>
    InternalFailure(String),
    /// <p> A resource could not be created because service quotas were exceeded. </p>
    LimitExceeded(String),
    /// <p> An operation failed due to a lack of access. </p>
    Unauthorized(String),
}

impl CreateDeploymentError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateDeploymentError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            #[allow(clippy::single_match)]
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(CreateDeploymentError::BadRequest(err.msg))
                }
                "InternalFailureException" => {
                    return RusotoError::Service(CreateDeploymentError::InternalFailure(err.msg))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(CreateDeploymentError::LimitExceeded(err.msg))
                }
                "UnauthorizedException" => {
                    return RusotoError::Service(CreateDeploymentError::Unauthorized(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for CreateDeploymentError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreateDeploymentError::BadRequest(ref cause) => write!(f, "{}", cause),
            CreateDeploymentError::InternalFailure(ref cause) => write!(f, "{}", cause),
            CreateDeploymentError::LimitExceeded(ref cause) => write!(f, "{}", cause),
            CreateDeploymentError::Unauthorized(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CreateDeploymentError {}
/// Errors returned by CreateDomainAssociation
#[derive(Debug, PartialEq)]
pub enum CreateDomainAssociationError {
    /// <p> A request contains unexpected data. </p>
    BadRequest(String),
    /// <p> An operation failed because a dependent service threw an exception. </p>
    DependentServiceFailure(String),
    /// <p> The service failed to perform an operation due to an internal issue. </p>
    InternalFailure(String),
    /// <p> A resource could not be created because service quotas were exceeded. </p>
    LimitExceeded(String),
    /// <p> An entity was not found during an operation. </p>
    NotFound(String),
    /// <p> An operation failed due to a lack of access. </p>
    Unauthorized(String),
}

impl CreateDomainAssociationError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateDomainAssociationError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            #[allow(clippy::single_match)]
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(CreateDomainAssociationError::BadRequest(err.msg))
                }
                "DependentServiceFailureException" => {
                    return RusotoError::Service(
                        CreateDomainAssociationError::DependentServiceFailure(err.msg),
                    )
                }
                "InternalFailureException" => {
                    return RusotoError::Service(CreateDomainAssociationError::InternalFailure(
                        err.msg,
                    ))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(CreateDomainAssociationError::LimitExceeded(
                        err.msg,
                    ))
                }
                "NotFoundException" => {
                    return RusotoError::Service(CreateDomainAssociationError::NotFound(err.msg))
                }
                "UnauthorizedException" => {
                    return RusotoError::Service(CreateDomainAssociationError::Unauthorized(
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
impl fmt::Display for CreateDomainAssociationError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreateDomainAssociationError::BadRequest(ref cause) => write!(f, "{}", cause),
            CreateDomainAssociationError::DependentServiceFailure(ref cause) => {
                write!(f, "{}", cause)
            }
            CreateDomainAssociationError::InternalFailure(ref cause) => write!(f, "{}", cause),
            CreateDomainAssociationError::LimitExceeded(ref cause) => write!(f, "{}", cause),
            CreateDomainAssociationError::NotFound(ref cause) => write!(f, "{}", cause),
            CreateDomainAssociationError::Unauthorized(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CreateDomainAssociationError {}
/// Errors returned by CreateWebhook
#[derive(Debug, PartialEq)]
pub enum CreateWebhookError {
    /// <p> A request contains unexpected data. </p>
    BadRequest(String),
    /// <p> An operation failed because a dependent service threw an exception. </p>
    DependentServiceFailure(String),
    /// <p> The service failed to perform an operation due to an internal issue. </p>
    InternalFailure(String),
    /// <p> A resource could not be created because service quotas were exceeded. </p>
    LimitExceeded(String),
    /// <p> An entity was not found during an operation. </p>
    NotFound(String),
    /// <p> An operation failed due to a lack of access. </p>
    Unauthorized(String),
}

impl CreateWebhookError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateWebhookError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            #[allow(clippy::single_match)]
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(CreateWebhookError::BadRequest(err.msg))
                }
                "DependentServiceFailureException" => {
                    return RusotoError::Service(CreateWebhookError::DependentServiceFailure(
                        err.msg,
                    ))
                }
                "InternalFailureException" => {
                    return RusotoError::Service(CreateWebhookError::InternalFailure(err.msg))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(CreateWebhookError::LimitExceeded(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(CreateWebhookError::NotFound(err.msg))
                }
                "UnauthorizedException" => {
                    return RusotoError::Service(CreateWebhookError::Unauthorized(err.msg))
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
            CreateWebhookError::BadRequest(ref cause) => write!(f, "{}", cause),
            CreateWebhookError::DependentServiceFailure(ref cause) => write!(f, "{}", cause),
            CreateWebhookError::InternalFailure(ref cause) => write!(f, "{}", cause),
            CreateWebhookError::LimitExceeded(ref cause) => write!(f, "{}", cause),
            CreateWebhookError::NotFound(ref cause) => write!(f, "{}", cause),
            CreateWebhookError::Unauthorized(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CreateWebhookError {}
/// Errors returned by DeleteApp
#[derive(Debug, PartialEq)]
pub enum DeleteAppError {
    /// <p> A request contains unexpected data. </p>
    BadRequest(String),
    /// <p> An operation failed because a dependent service threw an exception. </p>
    DependentServiceFailure(String),
    /// <p> The service failed to perform an operation due to an internal issue. </p>
    InternalFailure(String),
    /// <p> An entity was not found during an operation. </p>
    NotFound(String),
    /// <p> An operation failed due to a lack of access. </p>
    Unauthorized(String),
}

impl DeleteAppError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteAppError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            #[allow(clippy::single_match)]
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(DeleteAppError::BadRequest(err.msg))
                }
                "DependentServiceFailureException" => {
                    return RusotoError::Service(DeleteAppError::DependentServiceFailure(err.msg))
                }
                "InternalFailureException" => {
                    return RusotoError::Service(DeleteAppError::InternalFailure(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(DeleteAppError::NotFound(err.msg))
                }
                "UnauthorizedException" => {
                    return RusotoError::Service(DeleteAppError::Unauthorized(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DeleteAppError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteAppError::BadRequest(ref cause) => write!(f, "{}", cause),
            DeleteAppError::DependentServiceFailure(ref cause) => write!(f, "{}", cause),
            DeleteAppError::InternalFailure(ref cause) => write!(f, "{}", cause),
            DeleteAppError::NotFound(ref cause) => write!(f, "{}", cause),
            DeleteAppError::Unauthorized(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeleteAppError {}
/// Errors returned by DeleteBackendEnvironment
#[derive(Debug, PartialEq)]
pub enum DeleteBackendEnvironmentError {
    /// <p> A request contains unexpected data. </p>
    BadRequest(String),
    /// <p> An operation failed because a dependent service threw an exception. </p>
    DependentServiceFailure(String),
    /// <p> The service failed to perform an operation due to an internal issue. </p>
    InternalFailure(String),
    /// <p> An entity was not found during an operation. </p>
    NotFound(String),
    /// <p> An operation failed due to a lack of access. </p>
    Unauthorized(String),
}

impl DeleteBackendEnvironmentError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteBackendEnvironmentError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            #[allow(clippy::single_match)]
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(DeleteBackendEnvironmentError::BadRequest(err.msg))
                }
                "DependentServiceFailureException" => {
                    return RusotoError::Service(
                        DeleteBackendEnvironmentError::DependentServiceFailure(err.msg),
                    )
                }
                "InternalFailureException" => {
                    return RusotoError::Service(DeleteBackendEnvironmentError::InternalFailure(
                        err.msg,
                    ))
                }
                "NotFoundException" => {
                    return RusotoError::Service(DeleteBackendEnvironmentError::NotFound(err.msg))
                }
                "UnauthorizedException" => {
                    return RusotoError::Service(DeleteBackendEnvironmentError::Unauthorized(
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
impl fmt::Display for DeleteBackendEnvironmentError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteBackendEnvironmentError::BadRequest(ref cause) => write!(f, "{}", cause),
            DeleteBackendEnvironmentError::DependentServiceFailure(ref cause) => {
                write!(f, "{}", cause)
            }
            DeleteBackendEnvironmentError::InternalFailure(ref cause) => write!(f, "{}", cause),
            DeleteBackendEnvironmentError::NotFound(ref cause) => write!(f, "{}", cause),
            DeleteBackendEnvironmentError::Unauthorized(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeleteBackendEnvironmentError {}
/// Errors returned by DeleteBranch
#[derive(Debug, PartialEq)]
pub enum DeleteBranchError {
    /// <p> A request contains unexpected data. </p>
    BadRequest(String),
    /// <p> An operation failed because a dependent service threw an exception. </p>
    DependentServiceFailure(String),
    /// <p> The service failed to perform an operation due to an internal issue. </p>
    InternalFailure(String),
    /// <p> An entity was not found during an operation. </p>
    NotFound(String),
    /// <p> An operation failed due to a lack of access. </p>
    Unauthorized(String),
}

impl DeleteBranchError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteBranchError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            #[allow(clippy::single_match)]
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(DeleteBranchError::BadRequest(err.msg))
                }
                "DependentServiceFailureException" => {
                    return RusotoError::Service(DeleteBranchError::DependentServiceFailure(
                        err.msg,
                    ))
                }
                "InternalFailureException" => {
                    return RusotoError::Service(DeleteBranchError::InternalFailure(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(DeleteBranchError::NotFound(err.msg))
                }
                "UnauthorizedException" => {
                    return RusotoError::Service(DeleteBranchError::Unauthorized(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DeleteBranchError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteBranchError::BadRequest(ref cause) => write!(f, "{}", cause),
            DeleteBranchError::DependentServiceFailure(ref cause) => write!(f, "{}", cause),
            DeleteBranchError::InternalFailure(ref cause) => write!(f, "{}", cause),
            DeleteBranchError::NotFound(ref cause) => write!(f, "{}", cause),
            DeleteBranchError::Unauthorized(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeleteBranchError {}
/// Errors returned by DeleteDomainAssociation
#[derive(Debug, PartialEq)]
pub enum DeleteDomainAssociationError {
    /// <p> A request contains unexpected data. </p>
    BadRequest(String),
    /// <p> An operation failed because a dependent service threw an exception. </p>
    DependentServiceFailure(String),
    /// <p> The service failed to perform an operation due to an internal issue. </p>
    InternalFailure(String),
    /// <p> An entity was not found during an operation. </p>
    NotFound(String),
    /// <p> An operation failed due to a lack of access. </p>
    Unauthorized(String),
}

impl DeleteDomainAssociationError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteDomainAssociationError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            #[allow(clippy::single_match)]
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(DeleteDomainAssociationError::BadRequest(err.msg))
                }
                "DependentServiceFailureException" => {
                    return RusotoError::Service(
                        DeleteDomainAssociationError::DependentServiceFailure(err.msg),
                    )
                }
                "InternalFailureException" => {
                    return RusotoError::Service(DeleteDomainAssociationError::InternalFailure(
                        err.msg,
                    ))
                }
                "NotFoundException" => {
                    return RusotoError::Service(DeleteDomainAssociationError::NotFound(err.msg))
                }
                "UnauthorizedException" => {
                    return RusotoError::Service(DeleteDomainAssociationError::Unauthorized(
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
impl fmt::Display for DeleteDomainAssociationError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteDomainAssociationError::BadRequest(ref cause) => write!(f, "{}", cause),
            DeleteDomainAssociationError::DependentServiceFailure(ref cause) => {
                write!(f, "{}", cause)
            }
            DeleteDomainAssociationError::InternalFailure(ref cause) => write!(f, "{}", cause),
            DeleteDomainAssociationError::NotFound(ref cause) => write!(f, "{}", cause),
            DeleteDomainAssociationError::Unauthorized(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeleteDomainAssociationError {}
/// Errors returned by DeleteJob
#[derive(Debug, PartialEq)]
pub enum DeleteJobError {
    /// <p> A request contains unexpected data. </p>
    BadRequest(String),
    /// <p> The service failed to perform an operation due to an internal issue. </p>
    InternalFailure(String),
    /// <p> A resource could not be created because service quotas were exceeded. </p>
    LimitExceeded(String),
    /// <p> An entity was not found during an operation. </p>
    NotFound(String),
    /// <p> An operation failed due to a lack of access. </p>
    Unauthorized(String),
}

impl DeleteJobError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteJobError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            #[allow(clippy::single_match)]
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(DeleteJobError::BadRequest(err.msg))
                }
                "InternalFailureException" => {
                    return RusotoError::Service(DeleteJobError::InternalFailure(err.msg))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(DeleteJobError::LimitExceeded(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(DeleteJobError::NotFound(err.msg))
                }
                "UnauthorizedException" => {
                    return RusotoError::Service(DeleteJobError::Unauthorized(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DeleteJobError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteJobError::BadRequest(ref cause) => write!(f, "{}", cause),
            DeleteJobError::InternalFailure(ref cause) => write!(f, "{}", cause),
            DeleteJobError::LimitExceeded(ref cause) => write!(f, "{}", cause),
            DeleteJobError::NotFound(ref cause) => write!(f, "{}", cause),
            DeleteJobError::Unauthorized(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeleteJobError {}
/// Errors returned by DeleteWebhook
#[derive(Debug, PartialEq)]
pub enum DeleteWebhookError {
    /// <p> A request contains unexpected data. </p>
    BadRequest(String),
    /// <p> The service failed to perform an operation due to an internal issue. </p>
    InternalFailure(String),
    /// <p> A resource could not be created because service quotas were exceeded. </p>
    LimitExceeded(String),
    /// <p> An entity was not found during an operation. </p>
    NotFound(String),
    /// <p> An operation failed due to a lack of access. </p>
    Unauthorized(String),
}

impl DeleteWebhookError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteWebhookError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            #[allow(clippy::single_match)]
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(DeleteWebhookError::BadRequest(err.msg))
                }
                "InternalFailureException" => {
                    return RusotoError::Service(DeleteWebhookError::InternalFailure(err.msg))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(DeleteWebhookError::LimitExceeded(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(DeleteWebhookError::NotFound(err.msg))
                }
                "UnauthorizedException" => {
                    return RusotoError::Service(DeleteWebhookError::Unauthorized(err.msg))
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
            DeleteWebhookError::BadRequest(ref cause) => write!(f, "{}", cause),
            DeleteWebhookError::InternalFailure(ref cause) => write!(f, "{}", cause),
            DeleteWebhookError::LimitExceeded(ref cause) => write!(f, "{}", cause),
            DeleteWebhookError::NotFound(ref cause) => write!(f, "{}", cause),
            DeleteWebhookError::Unauthorized(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeleteWebhookError {}
/// Errors returned by GenerateAccessLogs
#[derive(Debug, PartialEq)]
pub enum GenerateAccessLogsError {
    /// <p> A request contains unexpected data. </p>
    BadRequest(String),
    /// <p> The service failed to perform an operation due to an internal issue. </p>
    InternalFailure(String),
    /// <p> An entity was not found during an operation. </p>
    NotFound(String),
    /// <p> An operation failed due to a lack of access. </p>
    Unauthorized(String),
}

impl GenerateAccessLogsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GenerateAccessLogsError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            #[allow(clippy::single_match)]
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(GenerateAccessLogsError::BadRequest(err.msg))
                }
                "InternalFailureException" => {
                    return RusotoError::Service(GenerateAccessLogsError::InternalFailure(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(GenerateAccessLogsError::NotFound(err.msg))
                }
                "UnauthorizedException" => {
                    return RusotoError::Service(GenerateAccessLogsError::Unauthorized(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for GenerateAccessLogsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GenerateAccessLogsError::BadRequest(ref cause) => write!(f, "{}", cause),
            GenerateAccessLogsError::InternalFailure(ref cause) => write!(f, "{}", cause),
            GenerateAccessLogsError::NotFound(ref cause) => write!(f, "{}", cause),
            GenerateAccessLogsError::Unauthorized(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GenerateAccessLogsError {}
/// Errors returned by GetApp
#[derive(Debug, PartialEq)]
pub enum GetAppError {
    /// <p> A request contains unexpected data. </p>
    BadRequest(String),
    /// <p> The service failed to perform an operation due to an internal issue. </p>
    InternalFailure(String),
    /// <p> An entity was not found during an operation. </p>
    NotFound(String),
    /// <p> An operation failed due to a lack of access. </p>
    Unauthorized(String),
}

impl GetAppError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetAppError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            #[allow(clippy::single_match)]
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(GetAppError::BadRequest(err.msg))
                }
                "InternalFailureException" => {
                    return RusotoError::Service(GetAppError::InternalFailure(err.msg))
                }
                "NotFoundException" => return RusotoError::Service(GetAppError::NotFound(err.msg)),
                "UnauthorizedException" => {
                    return RusotoError::Service(GetAppError::Unauthorized(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for GetAppError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetAppError::BadRequest(ref cause) => write!(f, "{}", cause),
            GetAppError::InternalFailure(ref cause) => write!(f, "{}", cause),
            GetAppError::NotFound(ref cause) => write!(f, "{}", cause),
            GetAppError::Unauthorized(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetAppError {}
/// Errors returned by GetArtifactUrl
#[derive(Debug, PartialEq)]
pub enum GetArtifactUrlError {
    /// <p> A request contains unexpected data. </p>
    BadRequest(String),
    /// <p> The service failed to perform an operation due to an internal issue. </p>
    InternalFailure(String),
    /// <p> A resource could not be created because service quotas were exceeded. </p>
    LimitExceeded(String),
    /// <p> An entity was not found during an operation. </p>
    NotFound(String),
    /// <p> An operation failed due to a lack of access. </p>
    Unauthorized(String),
}

impl GetArtifactUrlError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetArtifactUrlError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            #[allow(clippy::single_match)]
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(GetArtifactUrlError::BadRequest(err.msg))
                }
                "InternalFailureException" => {
                    return RusotoError::Service(GetArtifactUrlError::InternalFailure(err.msg))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(GetArtifactUrlError::LimitExceeded(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(GetArtifactUrlError::NotFound(err.msg))
                }
                "UnauthorizedException" => {
                    return RusotoError::Service(GetArtifactUrlError::Unauthorized(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for GetArtifactUrlError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetArtifactUrlError::BadRequest(ref cause) => write!(f, "{}", cause),
            GetArtifactUrlError::InternalFailure(ref cause) => write!(f, "{}", cause),
            GetArtifactUrlError::LimitExceeded(ref cause) => write!(f, "{}", cause),
            GetArtifactUrlError::NotFound(ref cause) => write!(f, "{}", cause),
            GetArtifactUrlError::Unauthorized(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetArtifactUrlError {}
/// Errors returned by GetBackendEnvironment
#[derive(Debug, PartialEq)]
pub enum GetBackendEnvironmentError {
    /// <p> A request contains unexpected data. </p>
    BadRequest(String),
    /// <p> The service failed to perform an operation due to an internal issue. </p>
    InternalFailure(String),
    /// <p> An entity was not found during an operation. </p>
    NotFound(String),
    /// <p> An operation failed due to a lack of access. </p>
    Unauthorized(String),
}

impl GetBackendEnvironmentError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetBackendEnvironmentError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            #[allow(clippy::single_match)]
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(GetBackendEnvironmentError::BadRequest(err.msg))
                }
                "InternalFailureException" => {
                    return RusotoError::Service(GetBackendEnvironmentError::InternalFailure(
                        err.msg,
                    ))
                }
                "NotFoundException" => {
                    return RusotoError::Service(GetBackendEnvironmentError::NotFound(err.msg))
                }
                "UnauthorizedException" => {
                    return RusotoError::Service(GetBackendEnvironmentError::Unauthorized(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for GetBackendEnvironmentError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetBackendEnvironmentError::BadRequest(ref cause) => write!(f, "{}", cause),
            GetBackendEnvironmentError::InternalFailure(ref cause) => write!(f, "{}", cause),
            GetBackendEnvironmentError::NotFound(ref cause) => write!(f, "{}", cause),
            GetBackendEnvironmentError::Unauthorized(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetBackendEnvironmentError {}
/// Errors returned by GetBranch
#[derive(Debug, PartialEq)]
pub enum GetBranchError {
    /// <p> A request contains unexpected data. </p>
    BadRequest(String),
    /// <p> The service failed to perform an operation due to an internal issue. </p>
    InternalFailure(String),
    /// <p> An entity was not found during an operation. </p>
    NotFound(String),
    /// <p> An operation failed due to a lack of access. </p>
    Unauthorized(String),
}

impl GetBranchError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetBranchError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            #[allow(clippy::single_match)]
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(GetBranchError::BadRequest(err.msg))
                }
                "InternalFailureException" => {
                    return RusotoError::Service(GetBranchError::InternalFailure(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(GetBranchError::NotFound(err.msg))
                }
                "UnauthorizedException" => {
                    return RusotoError::Service(GetBranchError::Unauthorized(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for GetBranchError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetBranchError::BadRequest(ref cause) => write!(f, "{}", cause),
            GetBranchError::InternalFailure(ref cause) => write!(f, "{}", cause),
            GetBranchError::NotFound(ref cause) => write!(f, "{}", cause),
            GetBranchError::Unauthorized(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetBranchError {}
/// Errors returned by GetDomainAssociation
#[derive(Debug, PartialEq)]
pub enum GetDomainAssociationError {
    /// <p> A request contains unexpected data. </p>
    BadRequest(String),
    /// <p> The service failed to perform an operation due to an internal issue. </p>
    InternalFailure(String),
    /// <p> An entity was not found during an operation. </p>
    NotFound(String),
    /// <p> An operation failed due to a lack of access. </p>
    Unauthorized(String),
}

impl GetDomainAssociationError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetDomainAssociationError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            #[allow(clippy::single_match)]
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(GetDomainAssociationError::BadRequest(err.msg))
                }
                "InternalFailureException" => {
                    return RusotoError::Service(GetDomainAssociationError::InternalFailure(
                        err.msg,
                    ))
                }
                "NotFoundException" => {
                    return RusotoError::Service(GetDomainAssociationError::NotFound(err.msg))
                }
                "UnauthorizedException" => {
                    return RusotoError::Service(GetDomainAssociationError::Unauthorized(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for GetDomainAssociationError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetDomainAssociationError::BadRequest(ref cause) => write!(f, "{}", cause),
            GetDomainAssociationError::InternalFailure(ref cause) => write!(f, "{}", cause),
            GetDomainAssociationError::NotFound(ref cause) => write!(f, "{}", cause),
            GetDomainAssociationError::Unauthorized(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetDomainAssociationError {}
/// Errors returned by GetJob
#[derive(Debug, PartialEq)]
pub enum GetJobError {
    /// <p> A request contains unexpected data. </p>
    BadRequest(String),
    /// <p> The service failed to perform an operation due to an internal issue. </p>
    InternalFailure(String),
    /// <p> A resource could not be created because service quotas were exceeded. </p>
    LimitExceeded(String),
    /// <p> An entity was not found during an operation. </p>
    NotFound(String),
    /// <p> An operation failed due to a lack of access. </p>
    Unauthorized(String),
}

impl GetJobError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetJobError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            #[allow(clippy::single_match)]
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(GetJobError::BadRequest(err.msg))
                }
                "InternalFailureException" => {
                    return RusotoError::Service(GetJobError::InternalFailure(err.msg))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(GetJobError::LimitExceeded(err.msg))
                }
                "NotFoundException" => return RusotoError::Service(GetJobError::NotFound(err.msg)),
                "UnauthorizedException" => {
                    return RusotoError::Service(GetJobError::Unauthorized(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for GetJobError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetJobError::BadRequest(ref cause) => write!(f, "{}", cause),
            GetJobError::InternalFailure(ref cause) => write!(f, "{}", cause),
            GetJobError::LimitExceeded(ref cause) => write!(f, "{}", cause),
            GetJobError::NotFound(ref cause) => write!(f, "{}", cause),
            GetJobError::Unauthorized(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetJobError {}
/// Errors returned by GetWebhook
#[derive(Debug, PartialEq)]
pub enum GetWebhookError {
    /// <p> A request contains unexpected data. </p>
    BadRequest(String),
    /// <p> The service failed to perform an operation due to an internal issue. </p>
    InternalFailure(String),
    /// <p> A resource could not be created because service quotas were exceeded. </p>
    LimitExceeded(String),
    /// <p> An entity was not found during an operation. </p>
    NotFound(String),
    /// <p> An operation failed due to a lack of access. </p>
    Unauthorized(String),
}

impl GetWebhookError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetWebhookError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            #[allow(clippy::single_match)]
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(GetWebhookError::BadRequest(err.msg))
                }
                "InternalFailureException" => {
                    return RusotoError::Service(GetWebhookError::InternalFailure(err.msg))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(GetWebhookError::LimitExceeded(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(GetWebhookError::NotFound(err.msg))
                }
                "UnauthorizedException" => {
                    return RusotoError::Service(GetWebhookError::Unauthorized(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for GetWebhookError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetWebhookError::BadRequest(ref cause) => write!(f, "{}", cause),
            GetWebhookError::InternalFailure(ref cause) => write!(f, "{}", cause),
            GetWebhookError::LimitExceeded(ref cause) => write!(f, "{}", cause),
            GetWebhookError::NotFound(ref cause) => write!(f, "{}", cause),
            GetWebhookError::Unauthorized(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetWebhookError {}
/// Errors returned by ListApps
#[derive(Debug, PartialEq)]
pub enum ListAppsError {
    /// <p> A request contains unexpected data. </p>
    BadRequest(String),
    /// <p> The service failed to perform an operation due to an internal issue. </p>
    InternalFailure(String),
    /// <p> An operation failed due to a lack of access. </p>
    Unauthorized(String),
}

impl ListAppsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListAppsError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            #[allow(clippy::single_match)]
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(ListAppsError::BadRequest(err.msg))
                }
                "InternalFailureException" => {
                    return RusotoError::Service(ListAppsError::InternalFailure(err.msg))
                }
                "UnauthorizedException" => {
                    return RusotoError::Service(ListAppsError::Unauthorized(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ListAppsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListAppsError::BadRequest(ref cause) => write!(f, "{}", cause),
            ListAppsError::InternalFailure(ref cause) => write!(f, "{}", cause),
            ListAppsError::Unauthorized(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListAppsError {}
/// Errors returned by ListArtifacts
#[derive(Debug, PartialEq)]
pub enum ListArtifactsError {
    /// <p> A request contains unexpected data. </p>
    BadRequest(String),
    /// <p> The service failed to perform an operation due to an internal issue. </p>
    InternalFailure(String),
    /// <p> A resource could not be created because service quotas were exceeded. </p>
    LimitExceeded(String),
    /// <p> An operation failed due to a lack of access. </p>
    Unauthorized(String),
}

impl ListArtifactsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListArtifactsError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            #[allow(clippy::single_match)]
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(ListArtifactsError::BadRequest(err.msg))
                }
                "InternalFailureException" => {
                    return RusotoError::Service(ListArtifactsError::InternalFailure(err.msg))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(ListArtifactsError::LimitExceeded(err.msg))
                }
                "UnauthorizedException" => {
                    return RusotoError::Service(ListArtifactsError::Unauthorized(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ListArtifactsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListArtifactsError::BadRequest(ref cause) => write!(f, "{}", cause),
            ListArtifactsError::InternalFailure(ref cause) => write!(f, "{}", cause),
            ListArtifactsError::LimitExceeded(ref cause) => write!(f, "{}", cause),
            ListArtifactsError::Unauthorized(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListArtifactsError {}
/// Errors returned by ListBackendEnvironments
#[derive(Debug, PartialEq)]
pub enum ListBackendEnvironmentsError {
    /// <p> A request contains unexpected data. </p>
    BadRequest(String),
    /// <p> The service failed to perform an operation due to an internal issue. </p>
    InternalFailure(String),
    /// <p> An operation failed due to a lack of access. </p>
    Unauthorized(String),
}

impl ListBackendEnvironmentsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListBackendEnvironmentsError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            #[allow(clippy::single_match)]
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(ListBackendEnvironmentsError::BadRequest(err.msg))
                }
                "InternalFailureException" => {
                    return RusotoError::Service(ListBackendEnvironmentsError::InternalFailure(
                        err.msg,
                    ))
                }
                "UnauthorizedException" => {
                    return RusotoError::Service(ListBackendEnvironmentsError::Unauthorized(
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
impl fmt::Display for ListBackendEnvironmentsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListBackendEnvironmentsError::BadRequest(ref cause) => write!(f, "{}", cause),
            ListBackendEnvironmentsError::InternalFailure(ref cause) => write!(f, "{}", cause),
            ListBackendEnvironmentsError::Unauthorized(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListBackendEnvironmentsError {}
/// Errors returned by ListBranches
#[derive(Debug, PartialEq)]
pub enum ListBranchesError {
    /// <p> A request contains unexpected data. </p>
    BadRequest(String),
    /// <p> The service failed to perform an operation due to an internal issue. </p>
    InternalFailure(String),
    /// <p> An operation failed due to a lack of access. </p>
    Unauthorized(String),
}

impl ListBranchesError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListBranchesError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            #[allow(clippy::single_match)]
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(ListBranchesError::BadRequest(err.msg))
                }
                "InternalFailureException" => {
                    return RusotoError::Service(ListBranchesError::InternalFailure(err.msg))
                }
                "UnauthorizedException" => {
                    return RusotoError::Service(ListBranchesError::Unauthorized(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ListBranchesError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListBranchesError::BadRequest(ref cause) => write!(f, "{}", cause),
            ListBranchesError::InternalFailure(ref cause) => write!(f, "{}", cause),
            ListBranchesError::Unauthorized(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListBranchesError {}
/// Errors returned by ListDomainAssociations
#[derive(Debug, PartialEq)]
pub enum ListDomainAssociationsError {
    /// <p> A request contains unexpected data. </p>
    BadRequest(String),
    /// <p> The service failed to perform an operation due to an internal issue. </p>
    InternalFailure(String),
    /// <p> An operation failed due to a lack of access. </p>
    Unauthorized(String),
}

impl ListDomainAssociationsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListDomainAssociationsError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            #[allow(clippy::single_match)]
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(ListDomainAssociationsError::BadRequest(err.msg))
                }
                "InternalFailureException" => {
                    return RusotoError::Service(ListDomainAssociationsError::InternalFailure(
                        err.msg,
                    ))
                }
                "UnauthorizedException" => {
                    return RusotoError::Service(ListDomainAssociationsError::Unauthorized(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ListDomainAssociationsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListDomainAssociationsError::BadRequest(ref cause) => write!(f, "{}", cause),
            ListDomainAssociationsError::InternalFailure(ref cause) => write!(f, "{}", cause),
            ListDomainAssociationsError::Unauthorized(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListDomainAssociationsError {}
/// Errors returned by ListJobs
#[derive(Debug, PartialEq)]
pub enum ListJobsError {
    /// <p> A request contains unexpected data. </p>
    BadRequest(String),
    /// <p> The service failed to perform an operation due to an internal issue. </p>
    InternalFailure(String),
    /// <p> A resource could not be created because service quotas were exceeded. </p>
    LimitExceeded(String),
    /// <p> An operation failed due to a lack of access. </p>
    Unauthorized(String),
}

impl ListJobsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListJobsError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            #[allow(clippy::single_match)]
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(ListJobsError::BadRequest(err.msg))
                }
                "InternalFailureException" => {
                    return RusotoError::Service(ListJobsError::InternalFailure(err.msg))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(ListJobsError::LimitExceeded(err.msg))
                }
                "UnauthorizedException" => {
                    return RusotoError::Service(ListJobsError::Unauthorized(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ListJobsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListJobsError::BadRequest(ref cause) => write!(f, "{}", cause),
            ListJobsError::InternalFailure(ref cause) => write!(f, "{}", cause),
            ListJobsError::LimitExceeded(ref cause) => write!(f, "{}", cause),
            ListJobsError::Unauthorized(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListJobsError {}
/// Errors returned by ListTagsForResource
#[derive(Debug, PartialEq)]
pub enum ListTagsForResourceError {
    /// <p> A request contains unexpected data. </p>
    BadRequest(String),
    /// <p> The service failed to perform an operation due to an internal issue. </p>
    InternalFailure(String),
    /// <p> An operation failed due to a non-existent resource. </p>
    ResourceNotFound(String),
}

impl ListTagsForResourceError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListTagsForResourceError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            #[allow(clippy::single_match)]
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(ListTagsForResourceError::BadRequest(err.msg))
                }
                "InternalFailureException" => {
                    return RusotoError::Service(ListTagsForResourceError::InternalFailure(err.msg))
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
            ListTagsForResourceError::InternalFailure(ref cause) => write!(f, "{}", cause),
            ListTagsForResourceError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListTagsForResourceError {}
/// Errors returned by ListWebhooks
#[derive(Debug, PartialEq)]
pub enum ListWebhooksError {
    /// <p> A request contains unexpected data. </p>
    BadRequest(String),
    /// <p> The service failed to perform an operation due to an internal issue. </p>
    InternalFailure(String),
    /// <p> A resource could not be created because service quotas were exceeded. </p>
    LimitExceeded(String),
    /// <p> An operation failed due to a lack of access. </p>
    Unauthorized(String),
}

impl ListWebhooksError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListWebhooksError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            #[allow(clippy::single_match)]
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(ListWebhooksError::BadRequest(err.msg))
                }
                "InternalFailureException" => {
                    return RusotoError::Service(ListWebhooksError::InternalFailure(err.msg))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(ListWebhooksError::LimitExceeded(err.msg))
                }
                "UnauthorizedException" => {
                    return RusotoError::Service(ListWebhooksError::Unauthorized(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ListWebhooksError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListWebhooksError::BadRequest(ref cause) => write!(f, "{}", cause),
            ListWebhooksError::InternalFailure(ref cause) => write!(f, "{}", cause),
            ListWebhooksError::LimitExceeded(ref cause) => write!(f, "{}", cause),
            ListWebhooksError::Unauthorized(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListWebhooksError {}
/// Errors returned by StartDeployment
#[derive(Debug, PartialEq)]
pub enum StartDeploymentError {
    /// <p> A request contains unexpected data. </p>
    BadRequest(String),
    /// <p> The service failed to perform an operation due to an internal issue. </p>
    InternalFailure(String),
    /// <p> A resource could not be created because service quotas were exceeded. </p>
    LimitExceeded(String),
    /// <p> An entity was not found during an operation. </p>
    NotFound(String),
    /// <p> An operation failed due to a lack of access. </p>
    Unauthorized(String),
}

impl StartDeploymentError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<StartDeploymentError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            #[allow(clippy::single_match)]
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(StartDeploymentError::BadRequest(err.msg))
                }
                "InternalFailureException" => {
                    return RusotoError::Service(StartDeploymentError::InternalFailure(err.msg))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(StartDeploymentError::LimitExceeded(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(StartDeploymentError::NotFound(err.msg))
                }
                "UnauthorizedException" => {
                    return RusotoError::Service(StartDeploymentError::Unauthorized(err.msg))
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
            StartDeploymentError::InternalFailure(ref cause) => write!(f, "{}", cause),
            StartDeploymentError::LimitExceeded(ref cause) => write!(f, "{}", cause),
            StartDeploymentError::NotFound(ref cause) => write!(f, "{}", cause),
            StartDeploymentError::Unauthorized(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for StartDeploymentError {}
/// Errors returned by StartJob
#[derive(Debug, PartialEq)]
pub enum StartJobError {
    /// <p> A request contains unexpected data. </p>
    BadRequest(String),
    /// <p> The service failed to perform an operation due to an internal issue. </p>
    InternalFailure(String),
    /// <p> A resource could not be created because service quotas were exceeded. </p>
    LimitExceeded(String),
    /// <p> An entity was not found during an operation. </p>
    NotFound(String),
    /// <p> An operation failed due to a lack of access. </p>
    Unauthorized(String),
}

impl StartJobError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<StartJobError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            #[allow(clippy::single_match)]
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(StartJobError::BadRequest(err.msg))
                }
                "InternalFailureException" => {
                    return RusotoError::Service(StartJobError::InternalFailure(err.msg))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(StartJobError::LimitExceeded(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(StartJobError::NotFound(err.msg))
                }
                "UnauthorizedException" => {
                    return RusotoError::Service(StartJobError::Unauthorized(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for StartJobError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            StartJobError::BadRequest(ref cause) => write!(f, "{}", cause),
            StartJobError::InternalFailure(ref cause) => write!(f, "{}", cause),
            StartJobError::LimitExceeded(ref cause) => write!(f, "{}", cause),
            StartJobError::NotFound(ref cause) => write!(f, "{}", cause),
            StartJobError::Unauthorized(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for StartJobError {}
/// Errors returned by StopJob
#[derive(Debug, PartialEq)]
pub enum StopJobError {
    /// <p> A request contains unexpected data. </p>
    BadRequest(String),
    /// <p> The service failed to perform an operation due to an internal issue. </p>
    InternalFailure(String),
    /// <p> A resource could not be created because service quotas were exceeded. </p>
    LimitExceeded(String),
    /// <p> An entity was not found during an operation. </p>
    NotFound(String),
    /// <p> An operation failed due to a lack of access. </p>
    Unauthorized(String),
}

impl StopJobError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<StopJobError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            #[allow(clippy::single_match)]
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(StopJobError::BadRequest(err.msg))
                }
                "InternalFailureException" => {
                    return RusotoError::Service(StopJobError::InternalFailure(err.msg))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(StopJobError::LimitExceeded(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(StopJobError::NotFound(err.msg))
                }
                "UnauthorizedException" => {
                    return RusotoError::Service(StopJobError::Unauthorized(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for StopJobError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            StopJobError::BadRequest(ref cause) => write!(f, "{}", cause),
            StopJobError::InternalFailure(ref cause) => write!(f, "{}", cause),
            StopJobError::LimitExceeded(ref cause) => write!(f, "{}", cause),
            StopJobError::NotFound(ref cause) => write!(f, "{}", cause),
            StopJobError::Unauthorized(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for StopJobError {}
/// Errors returned by TagResource
#[derive(Debug, PartialEq)]
pub enum TagResourceError {
    /// <p> A request contains unexpected data. </p>
    BadRequest(String),
    /// <p> The service failed to perform an operation due to an internal issue. </p>
    InternalFailure(String),
    /// <p> An operation failed due to a non-existent resource. </p>
    ResourceNotFound(String),
}

impl TagResourceError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<TagResourceError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            #[allow(clippy::single_match)]
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(TagResourceError::BadRequest(err.msg))
                }
                "InternalFailureException" => {
                    return RusotoError::Service(TagResourceError::InternalFailure(err.msg))
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
            TagResourceError::InternalFailure(ref cause) => write!(f, "{}", cause),
            TagResourceError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for TagResourceError {}
/// Errors returned by UntagResource
#[derive(Debug, PartialEq)]
pub enum UntagResourceError {
    /// <p> A request contains unexpected data. </p>
    BadRequest(String),
    /// <p> The service failed to perform an operation due to an internal issue. </p>
    InternalFailure(String),
    /// <p> An operation failed due to a non-existent resource. </p>
    ResourceNotFound(String),
}

impl UntagResourceError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UntagResourceError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            #[allow(clippy::single_match)]
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(UntagResourceError::BadRequest(err.msg))
                }
                "InternalFailureException" => {
                    return RusotoError::Service(UntagResourceError::InternalFailure(err.msg))
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
            UntagResourceError::InternalFailure(ref cause) => write!(f, "{}", cause),
            UntagResourceError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for UntagResourceError {}
/// Errors returned by UpdateApp
#[derive(Debug, PartialEq)]
pub enum UpdateAppError {
    /// <p> A request contains unexpected data. </p>
    BadRequest(String),
    /// <p> The service failed to perform an operation due to an internal issue. </p>
    InternalFailure(String),
    /// <p> An entity was not found during an operation. </p>
    NotFound(String),
    /// <p> An operation failed due to a lack of access. </p>
    Unauthorized(String),
}

impl UpdateAppError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdateAppError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            #[allow(clippy::single_match)]
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(UpdateAppError::BadRequest(err.msg))
                }
                "InternalFailureException" => {
                    return RusotoError::Service(UpdateAppError::InternalFailure(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(UpdateAppError::NotFound(err.msg))
                }
                "UnauthorizedException" => {
                    return RusotoError::Service(UpdateAppError::Unauthorized(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for UpdateAppError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UpdateAppError::BadRequest(ref cause) => write!(f, "{}", cause),
            UpdateAppError::InternalFailure(ref cause) => write!(f, "{}", cause),
            UpdateAppError::NotFound(ref cause) => write!(f, "{}", cause),
            UpdateAppError::Unauthorized(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for UpdateAppError {}
/// Errors returned by UpdateBranch
#[derive(Debug, PartialEq)]
pub enum UpdateBranchError {
    /// <p> A request contains unexpected data. </p>
    BadRequest(String),
    /// <p> An operation failed because a dependent service threw an exception. </p>
    DependentServiceFailure(String),
    /// <p> The service failed to perform an operation due to an internal issue. </p>
    InternalFailure(String),
    /// <p> An entity was not found during an operation. </p>
    NotFound(String),
    /// <p> An operation failed due to a lack of access. </p>
    Unauthorized(String),
}

impl UpdateBranchError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdateBranchError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            #[allow(clippy::single_match)]
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(UpdateBranchError::BadRequest(err.msg))
                }
                "DependentServiceFailureException" => {
                    return RusotoError::Service(UpdateBranchError::DependentServiceFailure(
                        err.msg,
                    ))
                }
                "InternalFailureException" => {
                    return RusotoError::Service(UpdateBranchError::InternalFailure(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(UpdateBranchError::NotFound(err.msg))
                }
                "UnauthorizedException" => {
                    return RusotoError::Service(UpdateBranchError::Unauthorized(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for UpdateBranchError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UpdateBranchError::BadRequest(ref cause) => write!(f, "{}", cause),
            UpdateBranchError::DependentServiceFailure(ref cause) => write!(f, "{}", cause),
            UpdateBranchError::InternalFailure(ref cause) => write!(f, "{}", cause),
            UpdateBranchError::NotFound(ref cause) => write!(f, "{}", cause),
            UpdateBranchError::Unauthorized(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for UpdateBranchError {}
/// Errors returned by UpdateDomainAssociation
#[derive(Debug, PartialEq)]
pub enum UpdateDomainAssociationError {
    /// <p> A request contains unexpected data. </p>
    BadRequest(String),
    /// <p> An operation failed because a dependent service threw an exception. </p>
    DependentServiceFailure(String),
    /// <p> The service failed to perform an operation due to an internal issue. </p>
    InternalFailure(String),
    /// <p> An entity was not found during an operation. </p>
    NotFound(String),
    /// <p> An operation failed due to a lack of access. </p>
    Unauthorized(String),
}

impl UpdateDomainAssociationError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdateDomainAssociationError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            #[allow(clippy::single_match)]
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(UpdateDomainAssociationError::BadRequest(err.msg))
                }
                "DependentServiceFailureException" => {
                    return RusotoError::Service(
                        UpdateDomainAssociationError::DependentServiceFailure(err.msg),
                    )
                }
                "InternalFailureException" => {
                    return RusotoError::Service(UpdateDomainAssociationError::InternalFailure(
                        err.msg,
                    ))
                }
                "NotFoundException" => {
                    return RusotoError::Service(UpdateDomainAssociationError::NotFound(err.msg))
                }
                "UnauthorizedException" => {
                    return RusotoError::Service(UpdateDomainAssociationError::Unauthorized(
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
impl fmt::Display for UpdateDomainAssociationError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UpdateDomainAssociationError::BadRequest(ref cause) => write!(f, "{}", cause),
            UpdateDomainAssociationError::DependentServiceFailure(ref cause) => {
                write!(f, "{}", cause)
            }
            UpdateDomainAssociationError::InternalFailure(ref cause) => write!(f, "{}", cause),
            UpdateDomainAssociationError::NotFound(ref cause) => write!(f, "{}", cause),
            UpdateDomainAssociationError::Unauthorized(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for UpdateDomainAssociationError {}
/// Errors returned by UpdateWebhook
#[derive(Debug, PartialEq)]
pub enum UpdateWebhookError {
    /// <p> A request contains unexpected data. </p>
    BadRequest(String),
    /// <p> An operation failed because a dependent service threw an exception. </p>
    DependentServiceFailure(String),
    /// <p> The service failed to perform an operation due to an internal issue. </p>
    InternalFailure(String),
    /// <p> An entity was not found during an operation. </p>
    NotFound(String),
    /// <p> An operation failed due to a lack of access. </p>
    Unauthorized(String),
}

impl UpdateWebhookError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdateWebhookError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            #[allow(clippy::single_match)]
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(UpdateWebhookError::BadRequest(err.msg))
                }
                "DependentServiceFailureException" => {
                    return RusotoError::Service(UpdateWebhookError::DependentServiceFailure(
                        err.msg,
                    ))
                }
                "InternalFailureException" => {
                    return RusotoError::Service(UpdateWebhookError::InternalFailure(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(UpdateWebhookError::NotFound(err.msg))
                }
                "UnauthorizedException" => {
                    return RusotoError::Service(UpdateWebhookError::Unauthorized(err.msg))
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
            UpdateWebhookError::BadRequest(ref cause) => write!(f, "{}", cause),
            UpdateWebhookError::DependentServiceFailure(ref cause) => write!(f, "{}", cause),
            UpdateWebhookError::InternalFailure(ref cause) => write!(f, "{}", cause),
            UpdateWebhookError::NotFound(ref cause) => write!(f, "{}", cause),
            UpdateWebhookError::Unauthorized(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for UpdateWebhookError {}
/// Trait representing the capabilities of the Amplify API. Amplify clients implement this trait.
#[async_trait]
pub trait Amplify {
    /// <p> Creates a new Amplify app. </p>
    async fn create_app(
        &self,
        input: CreateAppRequest,
    ) -> Result<CreateAppResult, RusotoError<CreateAppError>>;

    /// <p> Creates a new backend environment for an Amplify app. </p>
    async fn create_backend_environment(
        &self,
        input: CreateBackendEnvironmentRequest,
    ) -> Result<CreateBackendEnvironmentResult, RusotoError<CreateBackendEnvironmentError>>;

    /// <p> Creates a new branch for an Amplify app. </p>
    async fn create_branch(
        &self,
        input: CreateBranchRequest,
    ) -> Result<CreateBranchResult, RusotoError<CreateBranchError>>;

    /// <p> Creates a deployment for a manually deployed Amplify app. Manually deployed apps are not connected to a repository. </p>
    async fn create_deployment(
        &self,
        input: CreateDeploymentRequest,
    ) -> Result<CreateDeploymentResult, RusotoError<CreateDeploymentError>>;

    /// <p> Creates a new domain association for an Amplify app. This action associates a custom domain with the Amplify app </p>
    async fn create_domain_association(
        &self,
        input: CreateDomainAssociationRequest,
    ) -> Result<CreateDomainAssociationResult, RusotoError<CreateDomainAssociationError>>;

    /// <p> Creates a new webhook on an Amplify app. </p>
    async fn create_webhook(
        &self,
        input: CreateWebhookRequest,
    ) -> Result<CreateWebhookResult, RusotoError<CreateWebhookError>>;

    /// <p> Deletes an existing Amplify app specified by an app ID. </p>
    async fn delete_app(
        &self,
        input: DeleteAppRequest,
    ) -> Result<DeleteAppResult, RusotoError<DeleteAppError>>;

    /// <p> Deletes a backend environment for an Amplify app. </p>
    async fn delete_backend_environment(
        &self,
        input: DeleteBackendEnvironmentRequest,
    ) -> Result<DeleteBackendEnvironmentResult, RusotoError<DeleteBackendEnvironmentError>>;

    /// <p> Deletes a branch for an Amplify app. </p>
    async fn delete_branch(
        &self,
        input: DeleteBranchRequest,
    ) -> Result<DeleteBranchResult, RusotoError<DeleteBranchError>>;

    /// <p> Deletes a domain association for an Amplify app. </p>
    async fn delete_domain_association(
        &self,
        input: DeleteDomainAssociationRequest,
    ) -> Result<DeleteDomainAssociationResult, RusotoError<DeleteDomainAssociationError>>;

    /// <p> Deletes a job for a branch of an Amplify app. </p>
    async fn delete_job(
        &self,
        input: DeleteJobRequest,
    ) -> Result<DeleteJobResult, RusotoError<DeleteJobError>>;

    /// <p> Deletes a webhook. </p>
    async fn delete_webhook(
        &self,
        input: DeleteWebhookRequest,
    ) -> Result<DeleteWebhookResult, RusotoError<DeleteWebhookError>>;

    /// <p> Returns the website access logs for a specific time range using a presigned URL. </p>
    async fn generate_access_logs(
        &self,
        input: GenerateAccessLogsRequest,
    ) -> Result<GenerateAccessLogsResult, RusotoError<GenerateAccessLogsError>>;

    /// <p> Returns an existing Amplify app by appID. </p>
    async fn get_app(&self, input: GetAppRequest)
        -> Result<GetAppResult, RusotoError<GetAppError>>;

    /// <p> Returns the artifact info that corresponds to an artifact id. </p>
    async fn get_artifact_url(
        &self,
        input: GetArtifactUrlRequest,
    ) -> Result<GetArtifactUrlResult, RusotoError<GetArtifactUrlError>>;

    /// <p> Returns a backend environment for an Amplify app. </p>
    async fn get_backend_environment(
        &self,
        input: GetBackendEnvironmentRequest,
    ) -> Result<GetBackendEnvironmentResult, RusotoError<GetBackendEnvironmentError>>;

    /// <p> Returns a branch for an Amplify app. </p>
    async fn get_branch(
        &self,
        input: GetBranchRequest,
    ) -> Result<GetBranchResult, RusotoError<GetBranchError>>;

    /// <p> Returns the domain information for an Amplify app. </p>
    async fn get_domain_association(
        &self,
        input: GetDomainAssociationRequest,
    ) -> Result<GetDomainAssociationResult, RusotoError<GetDomainAssociationError>>;

    /// <p> Returns a job for a branch of an Amplify app. </p>
    async fn get_job(&self, input: GetJobRequest)
        -> Result<GetJobResult, RusotoError<GetJobError>>;

    /// <p> Returns the webhook information that corresponds to a specified webhook ID. </p>
    async fn get_webhook(
        &self,
        input: GetWebhookRequest,
    ) -> Result<GetWebhookResult, RusotoError<GetWebhookError>>;

    /// <p> Returns a list of the existing Amplify apps. </p>
    async fn list_apps(
        &self,
        input: ListAppsRequest,
    ) -> Result<ListAppsResult, RusotoError<ListAppsError>>;

    /// <p> Returns a list of artifacts for a specified app, branch, and job. </p>
    async fn list_artifacts(
        &self,
        input: ListArtifactsRequest,
    ) -> Result<ListArtifactsResult, RusotoError<ListArtifactsError>>;

    /// <p> Lists the backend environments for an Amplify app. </p>
    async fn list_backend_environments(
        &self,
        input: ListBackendEnvironmentsRequest,
    ) -> Result<ListBackendEnvironmentsResult, RusotoError<ListBackendEnvironmentsError>>;

    /// <p> Lists the branches of an Amplify app. </p>
    async fn list_branches(
        &self,
        input: ListBranchesRequest,
    ) -> Result<ListBranchesResult, RusotoError<ListBranchesError>>;

    /// <p> Returns the domain associations for an Amplify app. </p>
    async fn list_domain_associations(
        &self,
        input: ListDomainAssociationsRequest,
    ) -> Result<ListDomainAssociationsResult, RusotoError<ListDomainAssociationsError>>;

    /// <p> Lists the jobs for a branch of an Amplify app. </p>
    async fn list_jobs(
        &self,
        input: ListJobsRequest,
    ) -> Result<ListJobsResult, RusotoError<ListJobsError>>;

    /// <p> Returns a list of tags for a specified Amazon Resource Name (ARN). </p>
    async fn list_tags_for_resource(
        &self,
        input: ListTagsForResourceRequest,
    ) -> Result<ListTagsForResourceResponse, RusotoError<ListTagsForResourceError>>;

    /// <p> Returns a list of webhooks for an Amplify app. </p>
    async fn list_webhooks(
        &self,
        input: ListWebhooksRequest,
    ) -> Result<ListWebhooksResult, RusotoError<ListWebhooksError>>;

    /// <p> Starts a deployment for a manually deployed app. Manually deployed apps are not connected to a repository. </p>
    async fn start_deployment(
        &self,
        input: StartDeploymentRequest,
    ) -> Result<StartDeploymentResult, RusotoError<StartDeploymentError>>;

    /// <p> Starts a new job for a branch of an Amplify app. </p>
    async fn start_job(
        &self,
        input: StartJobRequest,
    ) -> Result<StartJobResult, RusotoError<StartJobError>>;

    /// <p> Stops a job that is in progress for a branch of an Amplify app. </p>
    async fn stop_job(
        &self,
        input: StopJobRequest,
    ) -> Result<StopJobResult, RusotoError<StopJobError>>;

    /// <p> Tags the resource with a tag key and value. </p>
    async fn tag_resource(
        &self,
        input: TagResourceRequest,
    ) -> Result<TagResourceResponse, RusotoError<TagResourceError>>;

    /// <p> Untags a resource with a specified Amazon Resource Name (ARN). </p>
    async fn untag_resource(
        &self,
        input: UntagResourceRequest,
    ) -> Result<UntagResourceResponse, RusotoError<UntagResourceError>>;

    /// <p> Updates an existing Amplify app. </p>
    async fn update_app(
        &self,
        input: UpdateAppRequest,
    ) -> Result<UpdateAppResult, RusotoError<UpdateAppError>>;

    /// <p> Updates a branch for an Amplify app. </p>
    async fn update_branch(
        &self,
        input: UpdateBranchRequest,
    ) -> Result<UpdateBranchResult, RusotoError<UpdateBranchError>>;

    /// <p> Creates a new domain association for an Amplify app.</p>
    async fn update_domain_association(
        &self,
        input: UpdateDomainAssociationRequest,
    ) -> Result<UpdateDomainAssociationResult, RusotoError<UpdateDomainAssociationError>>;

    /// <p> Updates a webhook. </p>
    async fn update_webhook(
        &self,
        input: UpdateWebhookRequest,
    ) -> Result<UpdateWebhookResult, RusotoError<UpdateWebhookError>>;
}
/// A client for the Amplify API.
#[derive(Clone)]
pub struct AmplifyClient {
    client: Client,
    region: region::Region,
}

impl AmplifyClient {
    /// Creates a client backed by the default tokio event loop.
    ///
    /// The client will use the default credentials provider and tls client.
    pub fn new(region: region::Region) -> AmplifyClient {
        AmplifyClient {
            client: Client::shared(),
            region,
        }
    }

    pub fn new_with<P, D>(
        request_dispatcher: D,
        credentials_provider: P,
        region: region::Region,
    ) -> AmplifyClient
    where
        P: ProvideAwsCredentials + Send + Sync + 'static,
        D: DispatchSignedRequest + Send + Sync + 'static,
    {
        AmplifyClient {
            client: Client::new_with(credentials_provider, request_dispatcher),
            region,
        }
    }

    pub fn new_with_client(client: Client, region: region::Region) -> AmplifyClient {
        AmplifyClient { client, region }
    }
}

#[async_trait]
impl Amplify for AmplifyClient {
    /// <p> Creates a new Amplify app. </p>
    #[allow(unused_mut)]
    async fn create_app(
        &self,
        input: CreateAppRequest,
    ) -> Result<CreateAppResult, RusotoError<CreateAppError>> {
        #![allow(clippy::needless_update)]
        let request_uri = "/apps";

        let mut request = SignedRequest::new("POST", "amplify", &self.region, &request_uri);
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
            let result =
                proto::json::ResponsePayload::new(&response).deserialize::<CreateAppResult, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(CreateAppError::from_response(response))
        }
    }

    /// <p> Creates a new backend environment for an Amplify app. </p>
    #[allow(unused_mut)]
    async fn create_backend_environment(
        &self,
        input: CreateBackendEnvironmentRequest,
    ) -> Result<CreateBackendEnvironmentResult, RusotoError<CreateBackendEnvironmentError>> {
        #![allow(clippy::needless_update)]
        let request_uri = format!("/apps/{app_id}/backendenvironments", app_id = input.app_id);

        let mut request = SignedRequest::new("POST", "amplify", &self.region, &request_uri);
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
                .deserialize::<CreateBackendEnvironmentResult, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(CreateBackendEnvironmentError::from_response(response))
        }
    }

    /// <p> Creates a new branch for an Amplify app. </p>
    #[allow(unused_mut)]
    async fn create_branch(
        &self,
        input: CreateBranchRequest,
    ) -> Result<CreateBranchResult, RusotoError<CreateBranchError>> {
        #![allow(clippy::needless_update)]
        let request_uri = format!("/apps/{app_id}/branches", app_id = input.app_id);

        let mut request = SignedRequest::new("POST", "amplify", &self.region, &request_uri);
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
                .deserialize::<CreateBranchResult, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(CreateBranchError::from_response(response))
        }
    }

    /// <p> Creates a deployment for a manually deployed Amplify app. Manually deployed apps are not connected to a repository. </p>
    #[allow(unused_mut)]
    async fn create_deployment(
        &self,
        input: CreateDeploymentRequest,
    ) -> Result<CreateDeploymentResult, RusotoError<CreateDeploymentError>> {
        #![allow(clippy::needless_update)]
        let request_uri = format!(
            "/apps/{app_id}/branches/{branch_name}/deployments",
            app_id = input.app_id,
            branch_name = input.branch_name
        );

        let mut request = SignedRequest::new("POST", "amplify", &self.region, &request_uri);
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
                .deserialize::<CreateDeploymentResult, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(CreateDeploymentError::from_response(response))
        }
    }

    /// <p> Creates a new domain association for an Amplify app. This action associates a custom domain with the Amplify app </p>
    #[allow(unused_mut)]
    async fn create_domain_association(
        &self,
        input: CreateDomainAssociationRequest,
    ) -> Result<CreateDomainAssociationResult, RusotoError<CreateDomainAssociationError>> {
        #![allow(clippy::needless_update)]
        let request_uri = format!("/apps/{app_id}/domains", app_id = input.app_id);

        let mut request = SignedRequest::new("POST", "amplify", &self.region, &request_uri);
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
                .deserialize::<CreateDomainAssociationResult, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(CreateDomainAssociationError::from_response(response))
        }
    }

    /// <p> Creates a new webhook on an Amplify app. </p>
    #[allow(unused_mut)]
    async fn create_webhook(
        &self,
        input: CreateWebhookRequest,
    ) -> Result<CreateWebhookResult, RusotoError<CreateWebhookError>> {
        #![allow(clippy::needless_update)]
        let request_uri = format!("/apps/{app_id}/webhooks", app_id = input.app_id);

        let mut request = SignedRequest::new("POST", "amplify", &self.region, &request_uri);
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
                .deserialize::<CreateWebhookResult, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(CreateWebhookError::from_response(response))
        }
    }

    /// <p> Deletes an existing Amplify app specified by an app ID. </p>
    #[allow(unused_mut)]
    async fn delete_app(
        &self,
        input: DeleteAppRequest,
    ) -> Result<DeleteAppResult, RusotoError<DeleteAppError>> {
        #![allow(clippy::needless_update)]
        let request_uri = format!("/apps/{app_id}", app_id = input.app_id);

        let mut request = SignedRequest::new("DELETE", "amplify", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result =
                proto::json::ResponsePayload::new(&response).deserialize::<DeleteAppResult, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(DeleteAppError::from_response(response))
        }
    }

    /// <p> Deletes a backend environment for an Amplify app. </p>
    #[allow(unused_mut)]
    async fn delete_backend_environment(
        &self,
        input: DeleteBackendEnvironmentRequest,
    ) -> Result<DeleteBackendEnvironmentResult, RusotoError<DeleteBackendEnvironmentError>> {
        #![allow(clippy::needless_update)]
        let request_uri = format!(
            "/apps/{app_id}/backendenvironments/{environment_name}",
            app_id = input.app_id,
            environment_name = input.environment_name
        );

        let mut request = SignedRequest::new("DELETE", "amplify", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<DeleteBackendEnvironmentResult, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(DeleteBackendEnvironmentError::from_response(response))
        }
    }

    /// <p> Deletes a branch for an Amplify app. </p>
    #[allow(unused_mut)]
    async fn delete_branch(
        &self,
        input: DeleteBranchRequest,
    ) -> Result<DeleteBranchResult, RusotoError<DeleteBranchError>> {
        #![allow(clippy::needless_update)]
        let request_uri = format!(
            "/apps/{app_id}/branches/{branch_name}",
            app_id = input.app_id,
            branch_name = input.branch_name
        );

        let mut request = SignedRequest::new("DELETE", "amplify", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<DeleteBranchResult, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(DeleteBranchError::from_response(response))
        }
    }

    /// <p> Deletes a domain association for an Amplify app. </p>
    #[allow(unused_mut)]
    async fn delete_domain_association(
        &self,
        input: DeleteDomainAssociationRequest,
    ) -> Result<DeleteDomainAssociationResult, RusotoError<DeleteDomainAssociationError>> {
        #![allow(clippy::needless_update)]
        let request_uri = format!(
            "/apps/{app_id}/domains/{domain_name}",
            app_id = input.app_id,
            domain_name = input.domain_name
        );

        let mut request = SignedRequest::new("DELETE", "amplify", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<DeleteDomainAssociationResult, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(DeleteDomainAssociationError::from_response(response))
        }
    }

    /// <p> Deletes a job for a branch of an Amplify app. </p>
    #[allow(unused_mut)]
    async fn delete_job(
        &self,
        input: DeleteJobRequest,
    ) -> Result<DeleteJobResult, RusotoError<DeleteJobError>> {
        #![allow(clippy::needless_update)]
        let request_uri = format!(
            "/apps/{app_id}/branches/{branch_name}/jobs/{job_id}",
            app_id = input.app_id,
            branch_name = input.branch_name,
            job_id = input.job_id
        );

        let mut request = SignedRequest::new("DELETE", "amplify", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result =
                proto::json::ResponsePayload::new(&response).deserialize::<DeleteJobResult, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(DeleteJobError::from_response(response))
        }
    }

    /// <p> Deletes a webhook. </p>
    #[allow(unused_mut)]
    async fn delete_webhook(
        &self,
        input: DeleteWebhookRequest,
    ) -> Result<DeleteWebhookResult, RusotoError<DeleteWebhookError>> {
        #![allow(clippy::needless_update)]
        let request_uri = format!("/webhooks/{webhook_id}", webhook_id = input.webhook_id);

        let mut request = SignedRequest::new("DELETE", "amplify", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<DeleteWebhookResult, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(DeleteWebhookError::from_response(response))
        }
    }

    /// <p> Returns the website access logs for a specific time range using a presigned URL. </p>
    #[allow(unused_mut)]
    async fn generate_access_logs(
        &self,
        input: GenerateAccessLogsRequest,
    ) -> Result<GenerateAccessLogsResult, RusotoError<GenerateAccessLogsError>> {
        #![allow(clippy::needless_update)]
        let request_uri = format!("/apps/{app_id}/accesslogs", app_id = input.app_id);

        let mut request = SignedRequest::new("POST", "amplify", &self.region, &request_uri);
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
                .deserialize::<GenerateAccessLogsResult, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(GenerateAccessLogsError::from_response(response))
        }
    }

    /// <p> Returns an existing Amplify app by appID. </p>
    #[allow(unused_mut)]
    async fn get_app(
        &self,
        input: GetAppRequest,
    ) -> Result<GetAppResult, RusotoError<GetAppError>> {
        #![allow(clippy::needless_update)]
        let request_uri = format!("/apps/{app_id}", app_id = input.app_id);

        let mut request = SignedRequest::new("GET", "amplify", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result =
                proto::json::ResponsePayload::new(&response).deserialize::<GetAppResult, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(GetAppError::from_response(response))
        }
    }

    /// <p> Returns the artifact info that corresponds to an artifact id. </p>
    #[allow(unused_mut)]
    async fn get_artifact_url(
        &self,
        input: GetArtifactUrlRequest,
    ) -> Result<GetArtifactUrlResult, RusotoError<GetArtifactUrlError>> {
        #![allow(clippy::needless_update)]
        let request_uri = format!("/artifacts/{artifact_id}", artifact_id = input.artifact_id);

        let mut request = SignedRequest::new("GET", "amplify", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<GetArtifactUrlResult, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(GetArtifactUrlError::from_response(response))
        }
    }

    /// <p> Returns a backend environment for an Amplify app. </p>
    #[allow(unused_mut)]
    async fn get_backend_environment(
        &self,
        input: GetBackendEnvironmentRequest,
    ) -> Result<GetBackendEnvironmentResult, RusotoError<GetBackendEnvironmentError>> {
        #![allow(clippy::needless_update)]
        let request_uri = format!(
            "/apps/{app_id}/backendenvironments/{environment_name}",
            app_id = input.app_id,
            environment_name = input.environment_name
        );

        let mut request = SignedRequest::new("GET", "amplify", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<GetBackendEnvironmentResult, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(GetBackendEnvironmentError::from_response(response))
        }
    }

    /// <p> Returns a branch for an Amplify app. </p>
    #[allow(unused_mut)]
    async fn get_branch(
        &self,
        input: GetBranchRequest,
    ) -> Result<GetBranchResult, RusotoError<GetBranchError>> {
        #![allow(clippy::needless_update)]
        let request_uri = format!(
            "/apps/{app_id}/branches/{branch_name}",
            app_id = input.app_id,
            branch_name = input.branch_name
        );

        let mut request = SignedRequest::new("GET", "amplify", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result =
                proto::json::ResponsePayload::new(&response).deserialize::<GetBranchResult, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(GetBranchError::from_response(response))
        }
    }

    /// <p> Returns the domain information for an Amplify app. </p>
    #[allow(unused_mut)]
    async fn get_domain_association(
        &self,
        input: GetDomainAssociationRequest,
    ) -> Result<GetDomainAssociationResult, RusotoError<GetDomainAssociationError>> {
        #![allow(clippy::needless_update)]
        let request_uri = format!(
            "/apps/{app_id}/domains/{domain_name}",
            app_id = input.app_id,
            domain_name = input.domain_name
        );

        let mut request = SignedRequest::new("GET", "amplify", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<GetDomainAssociationResult, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(GetDomainAssociationError::from_response(response))
        }
    }

    /// <p> Returns a job for a branch of an Amplify app. </p>
    #[allow(unused_mut)]
    async fn get_job(
        &self,
        input: GetJobRequest,
    ) -> Result<GetJobResult, RusotoError<GetJobError>> {
        #![allow(clippy::needless_update)]
        let request_uri = format!(
            "/apps/{app_id}/branches/{branch_name}/jobs/{job_id}",
            app_id = input.app_id,
            branch_name = input.branch_name,
            job_id = input.job_id
        );

        let mut request = SignedRequest::new("GET", "amplify", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result =
                proto::json::ResponsePayload::new(&response).deserialize::<GetJobResult, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(GetJobError::from_response(response))
        }
    }

    /// <p> Returns the webhook information that corresponds to a specified webhook ID. </p>
    #[allow(unused_mut)]
    async fn get_webhook(
        &self,
        input: GetWebhookRequest,
    ) -> Result<GetWebhookResult, RusotoError<GetWebhookError>> {
        #![allow(clippy::needless_update)]
        let request_uri = format!("/webhooks/{webhook_id}", webhook_id = input.webhook_id);

        let mut request = SignedRequest::new("GET", "amplify", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<GetWebhookResult, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(GetWebhookError::from_response(response))
        }
    }

    /// <p> Returns a list of the existing Amplify apps. </p>
    #[allow(unused_mut)]
    async fn list_apps(
        &self,
        input: ListAppsRequest,
    ) -> Result<ListAppsResult, RusotoError<ListAppsError>> {
        #![allow(clippy::needless_update)]
        let request_uri = "/apps";

        let mut request = SignedRequest::new("GET", "amplify", &self.region, &request_uri);
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
            let result =
                proto::json::ResponsePayload::new(&response).deserialize::<ListAppsResult, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(ListAppsError::from_response(response))
        }
    }

    /// <p> Returns a list of artifacts for a specified app, branch, and job. </p>
    #[allow(unused_mut)]
    async fn list_artifacts(
        &self,
        input: ListArtifactsRequest,
    ) -> Result<ListArtifactsResult, RusotoError<ListArtifactsError>> {
        #![allow(clippy::needless_update)]
        let request_uri = format!(
            "/apps/{app_id}/branches/{branch_name}/jobs/{job_id}/artifacts",
            app_id = input.app_id,
            branch_name = input.branch_name,
            job_id = input.job_id
        );

        let mut request = SignedRequest::new("GET", "amplify", &self.region, &request_uri);
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
                .deserialize::<ListArtifactsResult, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(ListArtifactsError::from_response(response))
        }
    }

    /// <p> Lists the backend environments for an Amplify app. </p>
    #[allow(unused_mut)]
    async fn list_backend_environments(
        &self,
        input: ListBackendEnvironmentsRequest,
    ) -> Result<ListBackendEnvironmentsResult, RusotoError<ListBackendEnvironmentsError>> {
        #![allow(clippy::needless_update)]
        let request_uri = format!("/apps/{app_id}/backendenvironments", app_id = input.app_id);

        let mut request = SignedRequest::new("GET", "amplify", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut params = Params::new();
        if let Some(ref x) = input.environment_name {
            params.put("environmentName", x);
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
                .deserialize::<ListBackendEnvironmentsResult, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(ListBackendEnvironmentsError::from_response(response))
        }
    }

    /// <p> Lists the branches of an Amplify app. </p>
    #[allow(unused_mut)]
    async fn list_branches(
        &self,
        input: ListBranchesRequest,
    ) -> Result<ListBranchesResult, RusotoError<ListBranchesError>> {
        #![allow(clippy::needless_update)]
        let request_uri = format!("/apps/{app_id}/branches", app_id = input.app_id);

        let mut request = SignedRequest::new("GET", "amplify", &self.region, &request_uri);
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
                .deserialize::<ListBranchesResult, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(ListBranchesError::from_response(response))
        }
    }

    /// <p> Returns the domain associations for an Amplify app. </p>
    #[allow(unused_mut)]
    async fn list_domain_associations(
        &self,
        input: ListDomainAssociationsRequest,
    ) -> Result<ListDomainAssociationsResult, RusotoError<ListDomainAssociationsError>> {
        #![allow(clippy::needless_update)]
        let request_uri = format!("/apps/{app_id}/domains", app_id = input.app_id);

        let mut request = SignedRequest::new("GET", "amplify", &self.region, &request_uri);
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
                .deserialize::<ListDomainAssociationsResult, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(ListDomainAssociationsError::from_response(response))
        }
    }

    /// <p> Lists the jobs for a branch of an Amplify app. </p>
    #[allow(unused_mut)]
    async fn list_jobs(
        &self,
        input: ListJobsRequest,
    ) -> Result<ListJobsResult, RusotoError<ListJobsError>> {
        #![allow(clippy::needless_update)]
        let request_uri = format!(
            "/apps/{app_id}/branches/{branch_name}/jobs",
            app_id = input.app_id,
            branch_name = input.branch_name
        );

        let mut request = SignedRequest::new("GET", "amplify", &self.region, &request_uri);
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
            let result =
                proto::json::ResponsePayload::new(&response).deserialize::<ListJobsResult, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(ListJobsError::from_response(response))
        }
    }

    /// <p> Returns a list of tags for a specified Amazon Resource Name (ARN). </p>
    #[allow(unused_mut)]
    async fn list_tags_for_resource(
        &self,
        input: ListTagsForResourceRequest,
    ) -> Result<ListTagsForResourceResponse, RusotoError<ListTagsForResourceError>> {
        #![allow(clippy::needless_update)]
        let request_uri = format!("/tags/{resource_arn}", resource_arn = input.resource_arn);

        let mut request = SignedRequest::new("GET", "amplify", &self.region, &request_uri);
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

    /// <p> Returns a list of webhooks for an Amplify app. </p>
    #[allow(unused_mut)]
    async fn list_webhooks(
        &self,
        input: ListWebhooksRequest,
    ) -> Result<ListWebhooksResult, RusotoError<ListWebhooksError>> {
        #![allow(clippy::needless_update)]
        let request_uri = format!("/apps/{app_id}/webhooks", app_id = input.app_id);

        let mut request = SignedRequest::new("GET", "amplify", &self.region, &request_uri);
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
                .deserialize::<ListWebhooksResult, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(ListWebhooksError::from_response(response))
        }
    }

    /// <p> Starts a deployment for a manually deployed app. Manually deployed apps are not connected to a repository. </p>
    #[allow(unused_mut)]
    async fn start_deployment(
        &self,
        input: StartDeploymentRequest,
    ) -> Result<StartDeploymentResult, RusotoError<StartDeploymentError>> {
        #![allow(clippy::needless_update)]
        let request_uri = format!(
            "/apps/{app_id}/branches/{branch_name}/deployments/start",
            app_id = input.app_id,
            branch_name = input.branch_name
        );

        let mut request = SignedRequest::new("POST", "amplify", &self.region, &request_uri);
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
                .deserialize::<StartDeploymentResult, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(StartDeploymentError::from_response(response))
        }
    }

    /// <p> Starts a new job for a branch of an Amplify app. </p>
    #[allow(unused_mut)]
    async fn start_job(
        &self,
        input: StartJobRequest,
    ) -> Result<StartJobResult, RusotoError<StartJobError>> {
        #![allow(clippy::needless_update)]
        let request_uri = format!(
            "/apps/{app_id}/branches/{branch_name}/jobs",
            app_id = input.app_id,
            branch_name = input.branch_name
        );

        let mut request = SignedRequest::new("POST", "amplify", &self.region, &request_uri);
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
            let result =
                proto::json::ResponsePayload::new(&response).deserialize::<StartJobResult, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(StartJobError::from_response(response))
        }
    }

    /// <p> Stops a job that is in progress for a branch of an Amplify app. </p>
    #[allow(unused_mut)]
    async fn stop_job(
        &self,
        input: StopJobRequest,
    ) -> Result<StopJobResult, RusotoError<StopJobError>> {
        #![allow(clippy::needless_update)]
        let request_uri = format!(
            "/apps/{app_id}/branches/{branch_name}/jobs/{job_id}/stop",
            app_id = input.app_id,
            branch_name = input.branch_name,
            job_id = input.job_id
        );

        let mut request = SignedRequest::new("DELETE", "amplify", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result =
                proto::json::ResponsePayload::new(&response).deserialize::<StopJobResult, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(StopJobError::from_response(response))
        }
    }

    /// <p> Tags the resource with a tag key and value. </p>
    #[allow(unused_mut)]
    async fn tag_resource(
        &self,
        input: TagResourceRequest,
    ) -> Result<TagResourceResponse, RusotoError<TagResourceError>> {
        #![allow(clippy::needless_update)]
        let request_uri = format!("/tags/{resource_arn}", resource_arn = input.resource_arn);

        let mut request = SignedRequest::new("POST", "amplify", &self.region, &request_uri);
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

    /// <p> Untags a resource with a specified Amazon Resource Name (ARN). </p>
    #[allow(unused_mut)]
    async fn untag_resource(
        &self,
        input: UntagResourceRequest,
    ) -> Result<UntagResourceResponse, RusotoError<UntagResourceError>> {
        #![allow(clippy::needless_update)]
        let request_uri = format!("/tags/{resource_arn}", resource_arn = input.resource_arn);

        let mut request = SignedRequest::new("DELETE", "amplify", &self.region, &request_uri);
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

    /// <p> Updates an existing Amplify app. </p>
    #[allow(unused_mut)]
    async fn update_app(
        &self,
        input: UpdateAppRequest,
    ) -> Result<UpdateAppResult, RusotoError<UpdateAppError>> {
        #![allow(clippy::needless_update)]
        let request_uri = format!("/apps/{app_id}", app_id = input.app_id);

        let mut request = SignedRequest::new("POST", "amplify", &self.region, &request_uri);
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
            let result =
                proto::json::ResponsePayload::new(&response).deserialize::<UpdateAppResult, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(UpdateAppError::from_response(response))
        }
    }

    /// <p> Updates a branch for an Amplify app. </p>
    #[allow(unused_mut)]
    async fn update_branch(
        &self,
        input: UpdateBranchRequest,
    ) -> Result<UpdateBranchResult, RusotoError<UpdateBranchError>> {
        #![allow(clippy::needless_update)]
        let request_uri = format!(
            "/apps/{app_id}/branches/{branch_name}",
            app_id = input.app_id,
            branch_name = input.branch_name
        );

        let mut request = SignedRequest::new("POST", "amplify", &self.region, &request_uri);
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
                .deserialize::<UpdateBranchResult, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(UpdateBranchError::from_response(response))
        }
    }

    /// <p> Creates a new domain association for an Amplify app.</p>
    #[allow(unused_mut)]
    async fn update_domain_association(
        &self,
        input: UpdateDomainAssociationRequest,
    ) -> Result<UpdateDomainAssociationResult, RusotoError<UpdateDomainAssociationError>> {
        #![allow(clippy::needless_update)]
        let request_uri = format!(
            "/apps/{app_id}/domains/{domain_name}",
            app_id = input.app_id,
            domain_name = input.domain_name
        );

        let mut request = SignedRequest::new("POST", "amplify", &self.region, &request_uri);
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
                .deserialize::<UpdateDomainAssociationResult, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(UpdateDomainAssociationError::from_response(response))
        }
    }

    /// <p> Updates a webhook. </p>
    #[allow(unused_mut)]
    async fn update_webhook(
        &self,
        input: UpdateWebhookRequest,
    ) -> Result<UpdateWebhookResult, RusotoError<UpdateWebhookError>> {
        #![allow(clippy::needless_update)]
        let request_uri = format!("/webhooks/{webhook_id}", webhook_id = input.webhook_id);

        let mut request = SignedRequest::new("POST", "amplify", &self.region, &request_uri);
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
                .deserialize::<UpdateWebhookResult, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(UpdateWebhookError::from_response(response))
        }
    }
}
