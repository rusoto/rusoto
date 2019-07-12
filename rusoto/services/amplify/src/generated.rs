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

#[allow(warnings)]
use futures::future;
use futures::Future;
use rusoto_core::credential::ProvideAwsCredentials;
use rusoto_core::region;
use rusoto_core::request::{BufferedHttpResponse, DispatchSignedRequest};
use rusoto_core::{Client, RusotoError, RusotoFuture};

use rusoto_core::param::{Params, ServiceParams};
use rusoto_core::proto;
use rusoto_core::signature::SignedRequest;
use serde_json;
/// <p> Amplify App represents different branches of a repository for building, deploying, and hosting. </p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct App {
    /// <p> ARN for the Amplify App. </p>
    #[serde(rename = "appArn")]
    pub app_arn: String,
    /// <p> Unique Id for the Amplify App. </p>
    #[serde(rename = "appId")]
    pub app_id: String,
    /// <p> Automated branch creation config for the Amplify App. </p>
    #[serde(rename = "autoBranchCreationConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_branch_creation_config: Option<AutoBranchCreationConfig>,
    /// <p> Automated branch creation glob patterns for the Amplify App. </p>
    #[serde(rename = "autoBranchCreationPatterns")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_branch_creation_patterns: Option<Vec<String>>,
    /// <p> Basic Authorization credentials for branches for the Amplify App. </p>
    #[serde(rename = "basicAuthCredentials")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub basic_auth_credentials: Option<String>,
    /// <p> BuildSpec content for Amplify App. </p>
    #[serde(rename = "buildSpec")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub build_spec: Option<String>,
    /// <p> Create date / time for the Amplify App. </p>
    #[serde(rename = "createTime")]
    pub create_time: f64,
    /// <p> Custom redirect / rewrite rules for the Amplify App. </p>
    #[serde(rename = "customRules")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_rules: Option<Vec<CustomRule>>,
    /// <p> Default domain for the Amplify App. </p>
    #[serde(rename = "defaultDomain")]
    pub default_domain: String,
    /// <p> Description for the Amplify App. </p>
    #[serde(rename = "description")]
    pub description: String,
    /// <p> Enables automated branch creation for the Amplify App. </p>
    #[serde(rename = "enableAutoBranchCreation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_auto_branch_creation: Option<bool>,
    /// <p> Enables Basic Authorization for branches for the Amplify App. </p>
    #[serde(rename = "enableBasicAuth")]
    pub enable_basic_auth: bool,
    /// <p> Enables auto-building of branches for the Amplify App. </p>
    #[serde(rename = "enableBranchAutoBuild")]
    pub enable_branch_auto_build: bool,
    /// <p> Environment Variables for the Amplify App. </p>
    #[serde(rename = "environmentVariables")]
    pub environment_variables: ::std::collections::HashMap<String, String>,
    /// <p> IAM service role ARN for the Amplify App. </p>
    #[serde(rename = "iamServiceRoleArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iam_service_role_arn: Option<String>,
    /// <p> Name for the Amplify App. </p>
    #[serde(rename = "name")]
    pub name: String,
    /// <p> Platform for the Amplify App. </p>
    #[serde(rename = "platform")]
    pub platform: String,
    /// <p> Structure with Production Branch information. </p>
    #[serde(rename = "productionBranch")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub production_branch: Option<ProductionBranch>,
    /// <p> Repository for the Amplify App. </p>
    #[serde(rename = "repository")]
    pub repository: String,
    /// <p> Tag for Amplify App. </p>
    #[serde(rename = "tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<::std::collections::HashMap<String, String>>,
    /// <p> Update date / time for the Amplify App. </p>
    #[serde(rename = "updateTime")]
    pub update_time: f64,
}

/// <p> Structure with auto branch creation config. </p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AutoBranchCreationConfig {
    /// <p> Basic Authorization credentials for the auto created branch. </p>
    #[serde(rename = "basicAuthCredentials")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub basic_auth_credentials: Option<String>,
    /// <p> BuildSpec for the auto created branch. </p>
    #[serde(rename = "buildSpec")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub build_spec: Option<String>,
    /// <p> Enables auto building for the auto created branch. </p>
    #[serde(rename = "enableAutoBuild")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_auto_build: Option<bool>,
    /// <p> Enables Basic Auth for the auto created branch. </p>
    #[serde(rename = "enableBasicAuth")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_basic_auth: Option<bool>,
    /// <p> Environment Variables for the auto created branch. </p>
    #[serde(rename = "environmentVariables")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub environment_variables: Option<::std::collections::HashMap<String, String>>,
    /// <p> Framework for the auto created branch. </p>
    #[serde(rename = "framework")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub framework: Option<String>,
    /// <p> Stage for the auto created branch. </p>
    #[serde(rename = "stage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stage: Option<String>,
}

/// <p> Branch for an Amplify App, which maps to a 3rd party repository branch. </p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct Branch {
    /// <p> Id of the active job for a branch, part of an Amplify App. </p>
    #[serde(rename = "activeJobId")]
    pub active_job_id: String,
    /// <p> List of custom resources that are linked to this branch. </p>
    #[serde(rename = "associatedResources")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub associated_resources: Option<Vec<String>>,
    /// <p> Basic Authorization credentials for a branch, part of an Amplify App. </p>
    #[serde(rename = "basicAuthCredentials")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub basic_auth_credentials: Option<String>,
    /// <p> ARN for a branch, part of an Amplify App. </p>
    #[serde(rename = "branchArn")]
    pub branch_arn: String,
    /// <p> Name for a branch, part of an Amplify App. </p>
    #[serde(rename = "branchName")]
    pub branch_name: String,
    /// <p> BuildSpec content for branch for Amplify App. </p>
    #[serde(rename = "buildSpec")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub build_spec: Option<String>,
    /// <p> Creation date and time for a branch, part of an Amplify App. </p>
    #[serde(rename = "createTime")]
    pub create_time: f64,
    /// <p> Custom domains for a branch, part of an Amplify App. </p>
    #[serde(rename = "customDomains")]
    pub custom_domains: Vec<String>,
    /// <p> Description for a branch, part of an Amplify App. </p>
    #[serde(rename = "description")]
    pub description: String,
    /// <p> Display name for a branch, will use as the default domain prefix. </p>
    #[serde(rename = "displayName")]
    pub display_name: String,
    /// <p> Enables auto-building on push for a branch, part of an Amplify App. </p>
    #[serde(rename = "enableAutoBuild")]
    pub enable_auto_build: bool,
    /// <p> Enables Basic Authorization for a branch, part of an Amplify App. </p>
    #[serde(rename = "enableBasicAuth")]
    pub enable_basic_auth: bool,
    /// <p> Enables notifications for a branch, part of an Amplify App. </p>
    #[serde(rename = "enableNotification")]
    pub enable_notification: bool,
    /// <p> Environment Variables specific to a branch, part of an Amplify App. </p>
    #[serde(rename = "environmentVariables")]
    pub environment_variables: ::std::collections::HashMap<String, String>,
    /// <p> Framework for a branch, part of an Amplify App. </p>
    #[serde(rename = "framework")]
    pub framework: String,
    /// <p> Stage for a branch, part of an Amplify App. </p>
    #[serde(rename = "stage")]
    pub stage: String,
    /// <p> Tag for branch for Amplify App. </p>
    #[serde(rename = "tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<::std::collections::HashMap<String, String>>,
    /// <p> Thumbnail URL for the branch. </p>
    #[serde(rename = "thumbnailUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumbnail_url: Option<String>,
    /// <p> Total number of Jobs part of an Amplify App. </p>
    #[serde(rename = "totalNumberOfJobs")]
    pub total_number_of_jobs: String,
    /// <p> The content TTL for the website in seconds. </p>
    #[serde(rename = "ttl")]
    pub ttl: String,
    /// <p> Last updated date and time for a branch, part of an Amplify App. </p>
    #[serde(rename = "updateTime")]
    pub update_time: f64,
}

/// <p> Request structure used to create Apps in Amplify. </p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct CreateAppRequest {
    /// <p> Personal Access token for 3rd party source control system for an Amplify App, used to create webhook and read-only deploy key. Token is not stored. </p>
    #[serde(rename = "accessToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_token: Option<String>,
    /// <p> Automated branch creation config for the Amplify App. </p>
    #[serde(rename = "autoBranchCreationConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_branch_creation_config: Option<AutoBranchCreationConfig>,
    /// <p> Automated branch creation glob patterns for the Amplify App. </p>
    #[serde(rename = "autoBranchCreationPatterns")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_branch_creation_patterns: Option<Vec<String>>,
    /// <p> Credentials for Basic Authorization for an Amplify App. </p>
    #[serde(rename = "basicAuthCredentials")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub basic_auth_credentials: Option<String>,
    /// <p> BuildSpec for an Amplify App </p>
    #[serde(rename = "buildSpec")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub build_spec: Option<String>,
    /// <p> Custom rewrite / redirect rules for an Amplify App. </p>
    #[serde(rename = "customRules")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_rules: Option<Vec<CustomRule>>,
    /// <p> Description for an Amplify App </p>
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p> Enables automated branch creation for the Amplify App. </p>
    #[serde(rename = "enableAutoBranchCreation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_auto_branch_creation: Option<bool>,
    /// <p> Enable Basic Authorization for an Amplify App, this will apply to all branches part of this App. </p>
    #[serde(rename = "enableBasicAuth")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_basic_auth: Option<bool>,
    /// <p> Enable the auto building of branches for an Amplify App. </p>
    #[serde(rename = "enableBranchAutoBuild")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_branch_auto_build: Option<bool>,
    /// <p> Environment variables map for an Amplify App. </p>
    #[serde(rename = "environmentVariables")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub environment_variables: Option<::std::collections::HashMap<String, String>>,
    /// <p> AWS IAM service role for an Amplify App </p>
    #[serde(rename = "iamServiceRoleArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iam_service_role_arn: Option<String>,
    /// <p> Name for the Amplify App </p>
    #[serde(rename = "name")]
    pub name: String,
    /// <p> OAuth token for 3rd party source control system for an Amplify App, used to create webhook and read-only deploy key. OAuth token is not stored. </p>
    #[serde(rename = "oauthToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub oauth_token: Option<String>,
    /// <p> Platform / framework for an Amplify App </p>
    #[serde(rename = "platform")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub platform: Option<String>,
    /// <p> Repository for an Amplify App </p>
    #[serde(rename = "repository")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repository: Option<String>,
    /// <p> Tag for an Amplify App </p>
    #[serde(rename = "tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<::std::collections::HashMap<String, String>>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct CreateAppResult {
    #[serde(rename = "app")]
    pub app: App,
}

/// <p> Request structure for a branch create request. </p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct CreateBranchRequest {
    /// <p> Unique Id for an Amplify App. </p>
    #[serde(rename = "appId")]
    pub app_id: String,
    /// <p> Basic Authorization credentials for the branch. </p>
    #[serde(rename = "basicAuthCredentials")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub basic_auth_credentials: Option<String>,
    /// <p> Name for the branch. </p>
    #[serde(rename = "branchName")]
    pub branch_name: String,
    /// <p> BuildSpec for the branch. </p>
    #[serde(rename = "buildSpec")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub build_spec: Option<String>,
    /// <p> Description for the branch. </p>
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p> Display name for a branch, will use as the default domain prefix. </p>
    #[serde(rename = "displayName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    /// <p> Enables auto building for the branch. </p>
    #[serde(rename = "enableAutoBuild")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_auto_build: Option<bool>,
    /// <p> Enables Basic Auth for the branch. </p>
    #[serde(rename = "enableBasicAuth")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_basic_auth: Option<bool>,
    /// <p> Enables notifications for the branch. </p>
    #[serde(rename = "enableNotification")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_notification: Option<bool>,
    /// <p> Environment Variables for the branch. </p>
    #[serde(rename = "environmentVariables")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub environment_variables: Option<::std::collections::HashMap<String, String>>,
    /// <p> Framework for the branch. </p>
    #[serde(rename = "framework")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub framework: Option<String>,
    /// <p> Stage for the branch. </p>
    #[serde(rename = "stage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stage: Option<String>,
    /// <p> Tag for the branch. </p>
    #[serde(rename = "tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<::std::collections::HashMap<String, String>>,
    /// <p> The content TTL for the website in seconds. </p>
    #[serde(rename = "ttl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ttl: Option<String>,
}

/// <p> Result structure for create branch request. </p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct CreateBranchResult {
    /// <p> Branch structure for an Amplify App. </p>
    #[serde(rename = "branch")]
    pub branch: Branch,
}

/// <p> Request structure for create a new deployment. </p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct CreateDeploymentRequest {
    /// <p> Unique Id for an Amplify App. </p>
    #[serde(rename = "appId")]
    pub app_id: String,
    /// <p> Name for the branch, for the Job. </p>
    #[serde(rename = "branchName")]
    pub branch_name: String,
    /// <p> Optional file map that contains file name as the key and file content md5 hash as the value. If this argument is provided, the service will generate different upload url per file. Otherwise, the service will only generate a single upload url for the zipped files. </p>
    #[serde(rename = "fileMap")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_map: Option<::std::collections::HashMap<String, String>>,
}

/// <p> Result structure for create a new deployment. </p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct CreateDeploymentResult {
    /// <p> When the fileMap argument is provided in the request, the fileUploadUrls will contain a map of file names to upload url. </p>
    #[serde(rename = "fileUploadUrls")]
    pub file_upload_urls: ::std::collections::HashMap<String, String>,
    /// <p> The jobId for this deployment, will supply to start deployment api. </p>
    #[serde(rename = "jobId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_id: Option<String>,
    /// <p> When the fileMap argument is NOT provided. This zipUploadUrl will be returned. </p>
    #[serde(rename = "zipUploadUrl")]
    pub zip_upload_url: String,
}

/// <p> Request structure for create Domain Association request. </p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct CreateDomainAssociationRequest {
    /// <p> Unique Id for an Amplify App. </p>
    #[serde(rename = "appId")]
    pub app_id: String,
    /// <p> Domain name for the Domain Association. </p>
    #[serde(rename = "domainName")]
    pub domain_name: String,
    /// <p> Enables automated creation of Subdomains for branches. </p>
    #[serde(rename = "enableAutoSubDomain")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_auto_sub_domain: Option<bool>,
    /// <p> Setting structure for the Subdomain. </p>
    #[serde(rename = "subDomainSettings")]
    pub sub_domain_settings: Vec<SubDomainSetting>,
}

/// <p> Result structure for the create Domain Association request. </p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct CreateDomainAssociationResult {
    /// <p> Domain Association structure. </p>
    #[serde(rename = "domainAssociation")]
    pub domain_association: DomainAssociation,
}

/// <p> Request structure for create webhook request. </p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct CreateWebhookRequest {
    /// <p> Unique Id for an Amplify App. </p>
    #[serde(rename = "appId")]
    pub app_id: String,
    /// <p> Name for a branch, part of an Amplify App. </p>
    #[serde(rename = "branchName")]
    pub branch_name: String,
    /// <p> Description for a webhook. </p>
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

/// <p> Result structure for the create webhook request. </p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct CreateWebhookResult {
    /// <p> Webhook structure. </p>
    #[serde(rename = "webhook")]
    pub webhook: Webhook,
}

/// <p> Custom rewrite / redirect rule. </p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CustomRule {
    /// <p> The condition for a URL rewrite or redirect rule, e.g. country code. </p>
    #[serde(rename = "condition")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub condition: Option<String>,
    /// <p> The source pattern for a URL rewrite or redirect rule. </p>
    #[serde(rename = "source")]
    pub source: String,
    /// <p> The status code for a URL rewrite or redirect rule. </p>
    #[serde(rename = "status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// <p> The target pattern for a URL rewrite or redirect rule. </p>
    #[serde(rename = "target")]
    pub target: String,
}

/// <p> Request structure for an Amplify App delete request. </p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DeleteAppRequest {
    /// <p> Unique Id for an Amplify App. </p>
    #[serde(rename = "appId")]
    pub app_id: String,
}

/// <p> Result structure for an Amplify App delete request. </p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct DeleteAppResult {
    #[serde(rename = "app")]
    pub app: App,
}

/// <p> Request structure for delete branch request. </p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DeleteBranchRequest {
    /// <p> Unique Id for an Amplify App. </p>
    #[serde(rename = "appId")]
    pub app_id: String,
    /// <p> Name for the branch. </p>
    #[serde(rename = "branchName")]
    pub branch_name: String,
}

/// <p> Result structure for delete branch request. </p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct DeleteBranchResult {
    /// <p> Branch structure for an Amplify App. </p>
    #[serde(rename = "branch")]
    pub branch: Branch,
}

/// <p> Request structure for the delete Domain Association request. </p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DeleteDomainAssociationRequest {
    /// <p> Unique Id for an Amplify App. </p>
    #[serde(rename = "appId")]
    pub app_id: String,
    /// <p> Name of the domain. </p>
    #[serde(rename = "domainName")]
    pub domain_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct DeleteDomainAssociationResult {
    #[serde(rename = "domainAssociation")]
    pub domain_association: DomainAssociation,
}

/// <p> Request structure for delete job request. </p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DeleteJobRequest {
    /// <p> Unique Id for an Amplify App. </p>
    #[serde(rename = "appId")]
    pub app_id: String,
    /// <p> Name for the branch, for the Job. </p>
    #[serde(rename = "branchName")]
    pub branch_name: String,
    /// <p> Unique Id for the Job. </p>
    #[serde(rename = "jobId")]
    pub job_id: String,
}

/// <p> Result structure for the delete job request. </p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct DeleteJobResult {
    #[serde(rename = "jobSummary")]
    pub job_summary: JobSummary,
}

/// <p> Request structure for the delete webhook request. </p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DeleteWebhookRequest {
    /// <p> Unique Id for a webhook. </p>
    #[serde(rename = "webhookId")]
    pub webhook_id: String,
}

/// <p> Result structure for the delete webhook request. </p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct DeleteWebhookResult {
    /// <p> Webhook structure. </p>
    #[serde(rename = "webhook")]
    pub webhook: Webhook,
}

/// <p> Structure for Domain Association, which associates a custom domain with an Amplify App. </p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct DomainAssociation {
    /// <p> DNS Record for certificate verification. </p>
    #[serde(rename = "certificateVerificationDNSRecord")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate_verification_dns_record: Option<String>,
    /// <p> ARN for the Domain Association. </p>
    #[serde(rename = "domainAssociationArn")]
    pub domain_association_arn: String,
    /// <p> Name of the domain. </p>
    #[serde(rename = "domainName")]
    pub domain_name: String,
    /// <p> Status fo the Domain Association. </p>
    #[serde(rename = "domainStatus")]
    pub domain_status: String,
    /// <p> Enables automated creation of Subdomains for branches. </p>
    #[serde(rename = "enableAutoSubDomain")]
    pub enable_auto_sub_domain: bool,
    /// <p> Reason for the current status of the Domain Association. </p>
    #[serde(rename = "statusReason")]
    pub status_reason: String,
    /// <p> Subdomains for the Domain Association. </p>
    #[serde(rename = "subDomains")]
    pub sub_domains: Vec<SubDomain>,
}

/// <p> Request structure for get App request. </p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetAppRequest {
    /// <p> Unique Id for an Amplify App. </p>
    #[serde(rename = "appId")]
    pub app_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct GetAppResult {
    #[serde(rename = "app")]
    pub app: App,
}

/// <p> Result structure for get branch request. </p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetBranchRequest {
    /// <p> Unique Id for an Amplify App. </p>
    #[serde(rename = "appId")]
    pub app_id: String,
    /// <p> Name for the branch. </p>
    #[serde(rename = "branchName")]
    pub branch_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct GetBranchResult {
    #[serde(rename = "branch")]
    pub branch: Branch,
}

/// <p> Request structure for the get Domain Association request. </p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetDomainAssociationRequest {
    /// <p> Unique Id for an Amplify App. </p>
    #[serde(rename = "appId")]
    pub app_id: String,
    /// <p> Name of the domain. </p>
    #[serde(rename = "domainName")]
    pub domain_name: String,
}

/// <p> Result structure for the get Domain Association request. </p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct GetDomainAssociationResult {
    /// <p> Domain Association structure. </p>
    #[serde(rename = "domainAssociation")]
    pub domain_association: DomainAssociation,
}

/// <p> Request structure for get job request. </p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetJobRequest {
    /// <p> Unique Id for an Amplify App. </p>
    #[serde(rename = "appId")]
    pub app_id: String,
    /// <p> Name for the branch, for the Job. </p>
    #[serde(rename = "branchName")]
    pub branch_name: String,
    /// <p> Unique Id for the Job. </p>
    #[serde(rename = "jobId")]
    pub job_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct GetJobResult {
    #[serde(rename = "job")]
    pub job: Job,
}

/// <p> Request structure for the get webhook request. </p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetWebhookRequest {
    /// <p> Unique Id for a webhook. </p>
    #[serde(rename = "webhookId")]
    pub webhook_id: String,
}

/// <p> Result structure for the get webhook request. </p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct GetWebhookResult {
    /// <p> Webhook structure. </p>
    #[serde(rename = "webhook")]
    pub webhook: Webhook,
}

/// <p> Structure for an execution job for an Amplify App. </p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct Job {
    /// <p> Execution steps for an execution job, for an Amplify App. </p>
    #[serde(rename = "steps")]
    pub steps: Vec<Step>,
    /// <p> Summary for an execution job for an Amplify App. </p>
    #[serde(rename = "summary")]
    pub summary: JobSummary,
}

/// <p> Structure for the summary of a Job. </p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct JobSummary {
    /// <p> Commit Id from 3rd party repository provider for the Job. </p>
    #[serde(rename = "commitId")]
    pub commit_id: String,
    /// <p> Commit message from 3rd party repository provider for the Job. </p>
    #[serde(rename = "commitMessage")]
    pub commit_message: String,
    /// <p> Commit date / time for the Job. </p>
    #[serde(rename = "commitTime")]
    pub commit_time: f64,
    /// <p> End date / time for the Job. </p>
    #[serde(rename = "endTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<f64>,
    /// <p> Arn for the Job. </p>
    #[serde(rename = "jobArn")]
    pub job_arn: String,
    /// <p> Unique Id for the Job. </p>
    #[serde(rename = "jobId")]
    pub job_id: String,
    /// <p> Type for the Job. \n "RELEASE": Manually released from source by using StartJob API. "RETRY": Manually retried by using StartJob API. "WEB_HOOK": Automatically triggered by WebHooks. </p>
    #[serde(rename = "jobType")]
    pub job_type: String,
    /// <p> Start date / time for the Job. </p>
    #[serde(rename = "startTime")]
    pub start_time: f64,
    /// <p> Status for the Job. </p>
    #[serde(rename = "status")]
    pub status: String,
}

/// <p> Request structure for an Amplify App list request. </p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ListAppsRequest {
    /// <p> Maximum number of records to list in a single response. </p>
    #[serde(rename = "maxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p> Pagination token. If non-null pagination token is returned in a result, then pass its value in another request to fetch more entries. </p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

/// <p> Result structure for an Amplify App list request. </p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct ListAppsResult {
    /// <p> List of Amplify Apps. </p>
    #[serde(rename = "apps")]
    pub apps: Vec<App>,
    /// <p> Pagination token. Set to null to start listing Apps from start. If non-null pagination token is returned in a result, then pass its value in here to list more projects. </p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

/// <p> Request structure for list branches request. </p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ListBranchesRequest {
    /// <p> Unique Id for an Amplify App. </p>
    #[serde(rename = "appId")]
    pub app_id: String,
    /// <p> Maximum number of records to list in a single response. </p>
    #[serde(rename = "maxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p> Pagination token. Set to null to start listing branches from start. If a non-null pagination token is returned in a result, then pass its value in here to list more branches. </p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

/// <p> Result structure for list branches request. </p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct ListBranchesResult {
    /// <p> List of branches for an Amplify App. </p>
    #[serde(rename = "branches")]
    pub branches: Vec<Branch>,
    /// <p> Pagination token. If non-null pagination token is returned in a result, then pass its value in another request to fetch more entries. </p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

/// <p> Request structure for the list Domain Associations request. </p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ListDomainAssociationsRequest {
    /// <p> Unique Id for an Amplify App. </p>
    #[serde(rename = "appId")]
    pub app_id: String,
    /// <p> Maximum number of records to list in a single response. </p>
    #[serde(rename = "maxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p> Pagination token. Set to null to start listing Apps from start. If non-null pagination token is returned in a result, then pass its value in here to list more projects. </p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

/// <p> Result structure for the list Domain Association request. </p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct ListDomainAssociationsResult {
    /// <p> List of Domain Associations. </p>
    #[serde(rename = "domainAssociations")]
    pub domain_associations: Vec<DomainAssociation>,
    /// <p> Pagination token. If non-null pagination token is returned in a result, then pass its value in another request to fetch more entries. </p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

/// <p> Request structure for list job request. </p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ListJobsRequest {
    /// <p> Unique Id for an Amplify App. </p>
    #[serde(rename = "appId")]
    pub app_id: String,
    /// <p> Name for a branch. </p>
    #[serde(rename = "branchName")]
    pub branch_name: String,
    /// <p> Maximum number of records to list in a single response. </p>
    #[serde(rename = "maxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p> Pagination token. Set to null to start listing steps from start. If a non-null pagination token is returned in a result, then pass its value in here to list more steps. </p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

/// <p> Maximum number of records to list in a single response. </p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct ListJobsResult {
    /// <p> Result structure for list job result request. </p>
    #[serde(rename = "jobSummaries")]
    pub job_summaries: Vec<JobSummary>,
    /// <p> Pagination token. If non-null pagination token is returned in a result, then pass its value in another request to fetch more entries. </p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

/// <p> Request structure used to list tags for resource. </p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ListTagsForResourceRequest {
    /// <p> Resource arn used to list tags. </p>
    #[serde(rename = "resourceArn")]
    pub resource_arn: String,
}

/// <p> Response for list tags. </p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct ListTagsForResourceResponse {
    /// <p> Tags result for response. </p>
    #[serde(rename = "tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<::std::collections::HashMap<String, String>>,
}

/// <p> Request structure for the list webhooks request. </p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ListWebhooksRequest {
    /// <p> Unique Id for an Amplify App. </p>
    #[serde(rename = "appId")]
    pub app_id: String,
    /// <p> Maximum number of records to list in a single response. </p>
    #[serde(rename = "maxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p> Pagination token. Set to null to start listing webhooks from start. If non-null pagination token is returned in a result, then pass its value in here to list more webhooks. </p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

/// <p> Result structure for the list webhooks request. </p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct ListWebhooksResult {
    /// <p> Pagination token. If non-null pagination token is returned in a result, then pass its value in another request to fetch more entries. </p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p> List of webhooks. </p>
    #[serde(rename = "webhooks")]
    pub webhooks: Vec<Webhook>,
}

/// <p> Structure with Production Branch information. </p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct ProductionBranch {
    /// <p> Branch Name for Production Branch. </p>
    #[serde(rename = "branchName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub branch_name: Option<String>,
    /// <p> Last Deploy Time of Production Branch. </p>
    #[serde(rename = "lastDeployTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_deploy_time: Option<f64>,
    /// <p> Status of Production Branch. </p>
    #[serde(rename = "status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// <p> Thumbnail URL for Production Branch. </p>
    #[serde(rename = "thumbnailUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumbnail_url: Option<String>,
}

/// <p> Request structure for start a deployment. </p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct StartDeploymentRequest {
    /// <p> Unique Id for an Amplify App. </p>
    #[serde(rename = "appId")]
    pub app_id: String,
    /// <p> Name for the branch, for the Job. </p>
    #[serde(rename = "branchName")]
    pub branch_name: String,
    /// <p> The job id for this deployment, generated by create deployment request. </p>
    #[serde(rename = "jobId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_id: Option<String>,
    /// <p> The sourceUrl for this deployment, used when calling start deployment without create deployment. SourceUrl can be any HTTP GET url that is public accessible and downloads a single zip. </p>
    #[serde(rename = "sourceUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_url: Option<String>,
}

/// <p> Result structure for start a deployment. </p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct StartDeploymentResult {
    /// <p> Summary for the Job. </p>
    #[serde(rename = "jobSummary")]
    pub job_summary: JobSummary,
}

/// <p> Request structure for Start job request. </p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct StartJobRequest {
    /// <p> Unique Id for an Amplify App. </p>
    #[serde(rename = "appId")]
    pub app_id: String,
    /// <p> Name for the branch, for the Job. </p>
    #[serde(rename = "branchName")]
    pub branch_name: String,
    /// <p> Commit Id from 3rd party repository provider for the Job. </p>
    #[serde(rename = "commitId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub commit_id: Option<String>,
    /// <p> Commit message from 3rd party repository provider for the Job. </p>
    #[serde(rename = "commitMessage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub commit_message: Option<String>,
    /// <p> Commit date / time for the Job. </p>
    #[serde(rename = "commitTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub commit_time: Option<f64>,
    /// <p> Unique Id for an existing job. Required for "RETRY" JobType. </p>
    #[serde(rename = "jobId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_id: Option<String>,
    /// <p> Descriptive reason for starting this job. </p>
    #[serde(rename = "jobReason")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_reason: Option<String>,
    /// <p> Type for the Job. Available JobTypes are: \n "RELEASE": Start a new job with the latest change from the specified branch. Only available for apps that have connected to a repository. "RETRY": Retry an existing job. JobId is required for this type of job. </p>
    #[serde(rename = "jobType")]
    pub job_type: String,
}

/// <p> Result structure for run job request. </p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct StartJobResult {
    /// <p> Summary for the Job. </p>
    #[serde(rename = "jobSummary")]
    pub job_summary: JobSummary,
}

/// <p> Structure for an execution step for an execution job, for an Amplify App. </p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct Step {
    /// <p> URL to the artifact for the execution step. </p>
    #[serde(rename = "artifactsUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub artifacts_url: Option<String>,
    /// <p> The context for current step, will include build image if step is build. </p>
    #[serde(rename = "context")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub context: Option<String>,
    /// <p> End date/ time of the execution step. </p>
    #[serde(rename = "endTime")]
    pub end_time: f64,
    /// <p> URL to the logs for the execution step. </p>
    #[serde(rename = "logUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_url: Option<String>,
    /// <p> List of screenshot URLs for the execution step, if relevant. </p>
    #[serde(rename = "screenshots")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub screenshots: Option<::std::collections::HashMap<String, String>>,
    /// <p> Start date/ time of the execution step. </p>
    #[serde(rename = "startTime")]
    pub start_time: f64,
    /// <p> Status of the execution step. </p>
    #[serde(rename = "status")]
    pub status: String,
    /// <p> The reason for current step status. </p>
    #[serde(rename = "statusReason")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_reason: Option<String>,
    /// <p> Name of the execution step. </p>
    #[serde(rename = "stepName")]
    pub step_name: String,
}

/// <p> Request structure for stop job request. </p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct StopJobRequest {
    /// <p> Unique Id for an Amplify App. </p>
    #[serde(rename = "appId")]
    pub app_id: String,
    /// <p> Name for the branch, for the Job. </p>
    #[serde(rename = "branchName")]
    pub branch_name: String,
    /// <p> Unique Id for the Job. </p>
    #[serde(rename = "jobId")]
    pub job_id: String,
}

/// <p> Result structure for the stop job request. </p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct StopJobResult {
    /// <p> Summary for the Job. </p>
    #[serde(rename = "jobSummary")]
    pub job_summary: JobSummary,
}

/// <p> Subdomain for the Domain Association. </p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct SubDomain {
    /// <p> DNS record for the Subdomain. </p>
    #[serde(rename = "dnsRecord")]
    pub dns_record: String,
    /// <p> Setting structure for the Subdomain. </p>
    #[serde(rename = "subDomainSetting")]
    pub sub_domain_setting: SubDomainSetting,
    /// <p> Verified status of the Subdomain </p>
    #[serde(rename = "verified")]
    pub verified: bool,
}

/// <p> Setting for the Subdomain. </p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SubDomainSetting {
    /// <p> Branch name setting for the Subdomain. </p>
    #[serde(rename = "branchName")]
    pub branch_name: String,
    /// <p> Prefix setting for the Subdomain. </p>
    #[serde(rename = "prefix")]
    pub prefix: String,
}

/// <p> Request structure used to tag resource. </p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct TagResourceRequest {
    /// <p> Resource arn used to tag resource. </p>
    #[serde(rename = "resourceArn")]
    pub resource_arn: String,
    /// <p> Tags used to tag resource. </p>
    #[serde(rename = "tags")]
    pub tags: ::std::collections::HashMap<String, String>,
}

/// <p> Response for tag resource. </p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct TagResourceResponse {}

/// <p> Request structure used to untag resource. </p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct UntagResourceRequest {
    /// <p> Resource arn used to untag resource. </p>
    #[serde(rename = "resourceArn")]
    pub resource_arn: String,
    /// <p> Tag keys used to untag resource. </p>
    #[serde(rename = "tagKeys")]
    pub tag_keys: Vec<String>,
}

/// <p> Response for untag resource. </p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct UntagResourceResponse {}

/// <p> Request structure for update App request. </p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct UpdateAppRequest {
    /// <p> Unique Id for an Amplify App. </p>
    #[serde(rename = "appId")]
    pub app_id: String,
    /// <p> Automated branch creation config for the Amplify App. </p>
    #[serde(rename = "autoBranchCreationConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_branch_creation_config: Option<AutoBranchCreationConfig>,
    /// <p> Automated branch creation glob patterns for the Amplify App. </p>
    #[serde(rename = "autoBranchCreationPatterns")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_branch_creation_patterns: Option<Vec<String>>,
    /// <p> Basic Authorization credentials for an Amplify App. </p>
    #[serde(rename = "basicAuthCredentials")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub basic_auth_credentials: Option<String>,
    /// <p> BuildSpec for an Amplify App. </p>
    #[serde(rename = "buildSpec")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub build_spec: Option<String>,
    /// <p> Custom redirect / rewrite rules for an Amplify App. </p>
    #[serde(rename = "customRules")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_rules: Option<Vec<CustomRule>>,
    /// <p> Description for an Amplify App. </p>
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p> Enables automated branch creation for the Amplify App. </p>
    #[serde(rename = "enableAutoBranchCreation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_auto_branch_creation: Option<bool>,
    /// <p> Enables Basic Authorization for an Amplify App. </p>
    #[serde(rename = "enableBasicAuth")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_basic_auth: Option<bool>,
    /// <p> Enables branch auto-building for an Amplify App. </p>
    #[serde(rename = "enableBranchAutoBuild")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_branch_auto_build: Option<bool>,
    /// <p> Environment Variables for an Amplify App. </p>
    #[serde(rename = "environmentVariables")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub environment_variables: Option<::std::collections::HashMap<String, String>>,
    /// <p> IAM service role for an Amplify App. </p>
    #[serde(rename = "iamServiceRoleArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iam_service_role_arn: Option<String>,
    /// <p> Name for an Amplify App. </p>
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p> Platform for an Amplify App. </p>
    #[serde(rename = "platform")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub platform: Option<String>,
}

/// <p> Result structure for an Amplify App update request. </p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct UpdateAppResult {
    /// <p> App structure for the updated App. </p>
    #[serde(rename = "app")]
    pub app: App,
}

/// <p> Request structure for update branch request. </p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct UpdateBranchRequest {
    /// <p> Unique Id for an Amplify App. </p>
    #[serde(rename = "appId")]
    pub app_id: String,
    /// <p> Basic Authorization credentials for the branch. </p>
    #[serde(rename = "basicAuthCredentials")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub basic_auth_credentials: Option<String>,
    /// <p> Name for the branch. </p>
    #[serde(rename = "branchName")]
    pub branch_name: String,
    /// <p> BuildSpec for the branch. </p>
    #[serde(rename = "buildSpec")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub build_spec: Option<String>,
    /// <p> Description for the branch. </p>
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p> Display name for a branch, will use as the default domain prefix. </p>
    #[serde(rename = "displayName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    /// <p> Enables auto building for the branch. </p>
    #[serde(rename = "enableAutoBuild")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_auto_build: Option<bool>,
    /// <p> Enables Basic Auth for the branch. </p>
    #[serde(rename = "enableBasicAuth")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_basic_auth: Option<bool>,
    /// <p> Enables notifications for the branch. </p>
    #[serde(rename = "enableNotification")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_notification: Option<bool>,
    /// <p> Environment Variables for the branch. </p>
    #[serde(rename = "environmentVariables")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub environment_variables: Option<::std::collections::HashMap<String, String>>,
    /// <p> Framework for the branch. </p>
    #[serde(rename = "framework")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub framework: Option<String>,
    /// <p> Stage for the branch. </p>
    #[serde(rename = "stage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stage: Option<String>,
    /// <p> The content TTL for the website in seconds. </p>
    #[serde(rename = "ttl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ttl: Option<String>,
}

/// <p> Result structure for update branch request. </p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct UpdateBranchResult {
    /// <p> Branch structure for an Amplify App. </p>
    #[serde(rename = "branch")]
    pub branch: Branch,
}

/// <p> Request structure for update Domain Association request. </p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct UpdateDomainAssociationRequest {
    /// <p> Unique Id for an Amplify App. </p>
    #[serde(rename = "appId")]
    pub app_id: String,
    /// <p> Name of the domain. </p>
    #[serde(rename = "domainName")]
    pub domain_name: String,
    /// <p> Enables automated creation of Subdomains for branches. </p>
    #[serde(rename = "enableAutoSubDomain")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_auto_sub_domain: Option<bool>,
    /// <p> Setting structure for the Subdomain. </p>
    #[serde(rename = "subDomainSettings")]
    pub sub_domain_settings: Vec<SubDomainSetting>,
}

/// <p> Result structure for the update Domain Association request. </p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct UpdateDomainAssociationResult {
    /// <p> Domain Association structure. </p>
    #[serde(rename = "domainAssociation")]
    pub domain_association: DomainAssociation,
}

/// <p> Request structure for update webhook request. </p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct UpdateWebhookRequest {
    /// <p> Name for a branch, part of an Amplify App. </p>
    #[serde(rename = "branchName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub branch_name: Option<String>,
    /// <p> Description for a webhook. </p>
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p> Unique Id for a webhook. </p>
    #[serde(rename = "webhookId")]
    pub webhook_id: String,
}

/// <p> Result structure for the update webhook request. </p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct UpdateWebhookResult {
    /// <p> Webhook structure. </p>
    #[serde(rename = "webhook")]
    pub webhook: Webhook,
}

/// <p> Structure for webhook, which associates a webhook with an Amplify App. </p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct Webhook {
    /// <p> Name for a branch, part of an Amplify App. </p>
    #[serde(rename = "branchName")]
    pub branch_name: String,
    /// <p> Create date / time for a webhook. </p>
    #[serde(rename = "createTime")]
    pub create_time: f64,
    /// <p> Description for a webhook. </p>
    #[serde(rename = "description")]
    pub description: String,
    /// <p> Update date / time for a webhook. </p>
    #[serde(rename = "updateTime")]
    pub update_time: f64,
    /// <p> ARN for the webhook. </p>
    #[serde(rename = "webhookArn")]
    pub webhook_arn: String,
    /// <p> Id of the webhook. </p>
    #[serde(rename = "webhookId")]
    pub webhook_id: String,
    /// <p> Url of the webhook. </p>
    #[serde(rename = "webhookUrl")]
    pub webhook_url: String,
}

/// Errors returned by CreateApp
#[derive(Debug, PartialEq)]
pub enum CreateAppError {
    /// <p> Exception thrown when a request contains unexpected data. </p>
    BadRequest(String),
    /// <p> Exception thrown when an operation fails due to a dependent service throwing an exception. </p>
    DependentServiceFailure(String),
    /// <p> Exception thrown when the service fails to perform an operation due to an internal issue. </p>
    InternalFailure(String),
    /// <p> Exception thrown when a resource could not be created because of service limits. </p>
    LimitExceeded(String),
    /// <p> Exception thrown when an operation fails due to a lack of access. </p>
    Unauthorized(String),
}

impl CreateAppError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateAppError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
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
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for CreateAppError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateAppError {
    fn description(&self) -> &str {
        match *self {
            CreateAppError::BadRequest(ref cause) => cause,
            CreateAppError::DependentServiceFailure(ref cause) => cause,
            CreateAppError::InternalFailure(ref cause) => cause,
            CreateAppError::LimitExceeded(ref cause) => cause,
            CreateAppError::Unauthorized(ref cause) => cause,
        }
    }
}
/// Errors returned by CreateBranch
#[derive(Debug, PartialEq)]
pub enum CreateBranchError {
    /// <p> Exception thrown when a request contains unexpected data. </p>
    BadRequest(String),
    /// <p> Exception thrown when an operation fails due to a dependent service throwing an exception. </p>
    DependentServiceFailure(String),
    /// <p> Exception thrown when the service fails to perform an operation due to an internal issue. </p>
    InternalFailure(String),
    /// <p> Exception thrown when a resource could not be created because of service limits. </p>
    LimitExceeded(String),
    /// <p> Exception thrown when an entity has not been found during an operation. </p>
    NotFound(String),
    /// <p> Exception thrown when an operation fails due to a lack of access. </p>
    Unauthorized(String),
}

impl CreateBranchError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateBranchError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
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
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for CreateBranchError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateBranchError {
    fn description(&self) -> &str {
        match *self {
            CreateBranchError::BadRequest(ref cause) => cause,
            CreateBranchError::DependentServiceFailure(ref cause) => cause,
            CreateBranchError::InternalFailure(ref cause) => cause,
            CreateBranchError::LimitExceeded(ref cause) => cause,
            CreateBranchError::NotFound(ref cause) => cause,
            CreateBranchError::Unauthorized(ref cause) => cause,
        }
    }
}
/// Errors returned by CreateDeployment
#[derive(Debug, PartialEq)]
pub enum CreateDeploymentError {
    /// <p> Exception thrown when a request contains unexpected data. </p>
    BadRequest(String),
    /// <p> Exception thrown when the service fails to perform an operation due to an internal issue. </p>
    InternalFailure(String),
    /// <p> Exception thrown when a resource could not be created because of service limits. </p>
    LimitExceeded(String),
    /// <p> Exception thrown when an operation fails due to a lack of access. </p>
    Unauthorized(String),
}

impl CreateDeploymentError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateDeploymentError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
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
        return RusotoError::Unknown(res);
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
            CreateDeploymentError::InternalFailure(ref cause) => cause,
            CreateDeploymentError::LimitExceeded(ref cause) => cause,
            CreateDeploymentError::Unauthorized(ref cause) => cause,
        }
    }
}
/// Errors returned by CreateDomainAssociation
#[derive(Debug, PartialEq)]
pub enum CreateDomainAssociationError {
    /// <p> Exception thrown when a request contains unexpected data. </p>
    BadRequest(String),
    /// <p> Exception thrown when an operation fails due to a dependent service throwing an exception. </p>
    DependentServiceFailure(String),
    /// <p> Exception thrown when the service fails to perform an operation due to an internal issue. </p>
    InternalFailure(String),
    /// <p> Exception thrown when a resource could not be created because of service limits. </p>
    LimitExceeded(String),
    /// <p> Exception thrown when an entity has not been found during an operation. </p>
    NotFound(String),
    /// <p> Exception thrown when an operation fails due to a lack of access. </p>
    Unauthorized(String),
}

impl CreateDomainAssociationError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateDomainAssociationError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
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
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for CreateDomainAssociationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateDomainAssociationError {
    fn description(&self) -> &str {
        match *self {
            CreateDomainAssociationError::BadRequest(ref cause) => cause,
            CreateDomainAssociationError::DependentServiceFailure(ref cause) => cause,
            CreateDomainAssociationError::InternalFailure(ref cause) => cause,
            CreateDomainAssociationError::LimitExceeded(ref cause) => cause,
            CreateDomainAssociationError::NotFound(ref cause) => cause,
            CreateDomainAssociationError::Unauthorized(ref cause) => cause,
        }
    }
}
/// Errors returned by CreateWebhook
#[derive(Debug, PartialEq)]
pub enum CreateWebhookError {
    /// <p> Exception thrown when a request contains unexpected data. </p>
    BadRequest(String),
    /// <p> Exception thrown when an operation fails due to a dependent service throwing an exception. </p>
    DependentServiceFailure(String),
    /// <p> Exception thrown when the service fails to perform an operation due to an internal issue. </p>
    InternalFailure(String),
    /// <p> Exception thrown when a resource could not be created because of service limits. </p>
    LimitExceeded(String),
    /// <p> Exception thrown when an entity has not been found during an operation. </p>
    NotFound(String),
    /// <p> Exception thrown when an operation fails due to a lack of access. </p>
    Unauthorized(String),
}

impl CreateWebhookError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateWebhookError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
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
        return RusotoError::Unknown(res);
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
            CreateWebhookError::BadRequest(ref cause) => cause,
            CreateWebhookError::DependentServiceFailure(ref cause) => cause,
            CreateWebhookError::InternalFailure(ref cause) => cause,
            CreateWebhookError::LimitExceeded(ref cause) => cause,
            CreateWebhookError::NotFound(ref cause) => cause,
            CreateWebhookError::Unauthorized(ref cause) => cause,
        }
    }
}
/// Errors returned by DeleteApp
#[derive(Debug, PartialEq)]
pub enum DeleteAppError {
    /// <p> Exception thrown when a request contains unexpected data. </p>
    BadRequest(String),
    /// <p> Exception thrown when an operation fails due to a dependent service throwing an exception. </p>
    DependentServiceFailure(String),
    /// <p> Exception thrown when the service fails to perform an operation due to an internal issue. </p>
    InternalFailure(String),
    /// <p> Exception thrown when an entity has not been found during an operation. </p>
    NotFound(String),
    /// <p> Exception thrown when an operation fails due to a lack of access. </p>
    Unauthorized(String),
}

impl DeleteAppError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteAppError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
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
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for DeleteAppError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteAppError {
    fn description(&self) -> &str {
        match *self {
            DeleteAppError::BadRequest(ref cause) => cause,
            DeleteAppError::DependentServiceFailure(ref cause) => cause,
            DeleteAppError::InternalFailure(ref cause) => cause,
            DeleteAppError::NotFound(ref cause) => cause,
            DeleteAppError::Unauthorized(ref cause) => cause,
        }
    }
}
/// Errors returned by DeleteBranch
#[derive(Debug, PartialEq)]
pub enum DeleteBranchError {
    /// <p> Exception thrown when a request contains unexpected data. </p>
    BadRequest(String),
    /// <p> Exception thrown when an operation fails due to a dependent service throwing an exception. </p>
    DependentServiceFailure(String),
    /// <p> Exception thrown when the service fails to perform an operation due to an internal issue. </p>
    InternalFailure(String),
    /// <p> Exception thrown when an entity has not been found during an operation. </p>
    NotFound(String),
    /// <p> Exception thrown when an operation fails due to a lack of access. </p>
    Unauthorized(String),
}

impl DeleteBranchError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteBranchError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
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
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for DeleteBranchError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteBranchError {
    fn description(&self) -> &str {
        match *self {
            DeleteBranchError::BadRequest(ref cause) => cause,
            DeleteBranchError::DependentServiceFailure(ref cause) => cause,
            DeleteBranchError::InternalFailure(ref cause) => cause,
            DeleteBranchError::NotFound(ref cause) => cause,
            DeleteBranchError::Unauthorized(ref cause) => cause,
        }
    }
}
/// Errors returned by DeleteDomainAssociation
#[derive(Debug, PartialEq)]
pub enum DeleteDomainAssociationError {
    /// <p> Exception thrown when a request contains unexpected data. </p>
    BadRequest(String),
    /// <p> Exception thrown when an operation fails due to a dependent service throwing an exception. </p>
    DependentServiceFailure(String),
    /// <p> Exception thrown when the service fails to perform an operation due to an internal issue. </p>
    InternalFailure(String),
    /// <p> Exception thrown when an entity has not been found during an operation. </p>
    NotFound(String),
    /// <p> Exception thrown when an operation fails due to a lack of access. </p>
    Unauthorized(String),
}

impl DeleteDomainAssociationError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteDomainAssociationError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
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
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for DeleteDomainAssociationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteDomainAssociationError {
    fn description(&self) -> &str {
        match *self {
            DeleteDomainAssociationError::BadRequest(ref cause) => cause,
            DeleteDomainAssociationError::DependentServiceFailure(ref cause) => cause,
            DeleteDomainAssociationError::InternalFailure(ref cause) => cause,
            DeleteDomainAssociationError::NotFound(ref cause) => cause,
            DeleteDomainAssociationError::Unauthorized(ref cause) => cause,
        }
    }
}
/// Errors returned by DeleteJob
#[derive(Debug, PartialEq)]
pub enum DeleteJobError {
    /// <p> Exception thrown when a request contains unexpected data. </p>
    BadRequest(String),
    /// <p> Exception thrown when the service fails to perform an operation due to an internal issue. </p>
    InternalFailure(String),
    /// <p> Exception thrown when a resource could not be created because of service limits. </p>
    LimitExceeded(String),
    /// <p> Exception thrown when an entity has not been found during an operation. </p>
    NotFound(String),
    /// <p> Exception thrown when an operation fails due to a lack of access. </p>
    Unauthorized(String),
}

impl DeleteJobError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteJobError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
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
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for DeleteJobError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteJobError {
    fn description(&self) -> &str {
        match *self {
            DeleteJobError::BadRequest(ref cause) => cause,
            DeleteJobError::InternalFailure(ref cause) => cause,
            DeleteJobError::LimitExceeded(ref cause) => cause,
            DeleteJobError::NotFound(ref cause) => cause,
            DeleteJobError::Unauthorized(ref cause) => cause,
        }
    }
}
/// Errors returned by DeleteWebhook
#[derive(Debug, PartialEq)]
pub enum DeleteWebhookError {
    /// <p> Exception thrown when a request contains unexpected data. </p>
    BadRequest(String),
    /// <p> Exception thrown when the service fails to perform an operation due to an internal issue. </p>
    InternalFailure(String),
    /// <p> Exception thrown when a resource could not be created because of service limits. </p>
    LimitExceeded(String),
    /// <p> Exception thrown when an entity has not been found during an operation. </p>
    NotFound(String),
    /// <p> Exception thrown when an operation fails due to a lack of access. </p>
    Unauthorized(String),
}

impl DeleteWebhookError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteWebhookError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
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
        return RusotoError::Unknown(res);
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
            DeleteWebhookError::BadRequest(ref cause) => cause,
            DeleteWebhookError::InternalFailure(ref cause) => cause,
            DeleteWebhookError::LimitExceeded(ref cause) => cause,
            DeleteWebhookError::NotFound(ref cause) => cause,
            DeleteWebhookError::Unauthorized(ref cause) => cause,
        }
    }
}
/// Errors returned by GetApp
#[derive(Debug, PartialEq)]
pub enum GetAppError {
    /// <p> Exception thrown when a request contains unexpected data. </p>
    BadRequest(String),
    /// <p> Exception thrown when the service fails to perform an operation due to an internal issue. </p>
    InternalFailure(String),
    /// <p> Exception thrown when an entity has not been found during an operation. </p>
    NotFound(String),
    /// <p> Exception thrown when an operation fails due to a lack of access. </p>
    Unauthorized(String),
}

impl GetAppError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetAppError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
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
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for GetAppError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetAppError {
    fn description(&self) -> &str {
        match *self {
            GetAppError::BadRequest(ref cause) => cause,
            GetAppError::InternalFailure(ref cause) => cause,
            GetAppError::NotFound(ref cause) => cause,
            GetAppError::Unauthorized(ref cause) => cause,
        }
    }
}
/// Errors returned by GetBranch
#[derive(Debug, PartialEq)]
pub enum GetBranchError {
    /// <p> Exception thrown when a request contains unexpected data. </p>
    BadRequest(String),
    /// <p> Exception thrown when the service fails to perform an operation due to an internal issue. </p>
    InternalFailure(String),
    /// <p> Exception thrown when an entity has not been found during an operation. </p>
    NotFound(String),
    /// <p> Exception thrown when an operation fails due to a lack of access. </p>
    Unauthorized(String),
}

impl GetBranchError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetBranchError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
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
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for GetBranchError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetBranchError {
    fn description(&self) -> &str {
        match *self {
            GetBranchError::BadRequest(ref cause) => cause,
            GetBranchError::InternalFailure(ref cause) => cause,
            GetBranchError::NotFound(ref cause) => cause,
            GetBranchError::Unauthorized(ref cause) => cause,
        }
    }
}
/// Errors returned by GetDomainAssociation
#[derive(Debug, PartialEq)]
pub enum GetDomainAssociationError {
    /// <p> Exception thrown when a request contains unexpected data. </p>
    BadRequest(String),
    /// <p> Exception thrown when the service fails to perform an operation due to an internal issue. </p>
    InternalFailure(String),
    /// <p> Exception thrown when an entity has not been found during an operation. </p>
    NotFound(String),
    /// <p> Exception thrown when an operation fails due to a lack of access. </p>
    Unauthorized(String),
}

impl GetDomainAssociationError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetDomainAssociationError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
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
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for GetDomainAssociationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetDomainAssociationError {
    fn description(&self) -> &str {
        match *self {
            GetDomainAssociationError::BadRequest(ref cause) => cause,
            GetDomainAssociationError::InternalFailure(ref cause) => cause,
            GetDomainAssociationError::NotFound(ref cause) => cause,
            GetDomainAssociationError::Unauthorized(ref cause) => cause,
        }
    }
}
/// Errors returned by GetJob
#[derive(Debug, PartialEq)]
pub enum GetJobError {
    /// <p> Exception thrown when a request contains unexpected data. </p>
    BadRequest(String),
    /// <p> Exception thrown when the service fails to perform an operation due to an internal issue. </p>
    InternalFailure(String),
    /// <p> Exception thrown when a resource could not be created because of service limits. </p>
    LimitExceeded(String),
    /// <p> Exception thrown when an entity has not been found during an operation. </p>
    NotFound(String),
    /// <p> Exception thrown when an operation fails due to a lack of access. </p>
    Unauthorized(String),
}

impl GetJobError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetJobError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
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
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for GetJobError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetJobError {
    fn description(&self) -> &str {
        match *self {
            GetJobError::BadRequest(ref cause) => cause,
            GetJobError::InternalFailure(ref cause) => cause,
            GetJobError::LimitExceeded(ref cause) => cause,
            GetJobError::NotFound(ref cause) => cause,
            GetJobError::Unauthorized(ref cause) => cause,
        }
    }
}
/// Errors returned by GetWebhook
#[derive(Debug, PartialEq)]
pub enum GetWebhookError {
    /// <p> Exception thrown when a request contains unexpected data. </p>
    BadRequest(String),
    /// <p> Exception thrown when the service fails to perform an operation due to an internal issue. </p>
    InternalFailure(String),
    /// <p> Exception thrown when a resource could not be created because of service limits. </p>
    LimitExceeded(String),
    /// <p> Exception thrown when an entity has not been found during an operation. </p>
    NotFound(String),
    /// <p> Exception thrown when an operation fails due to a lack of access. </p>
    Unauthorized(String),
}

impl GetWebhookError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetWebhookError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
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
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for GetWebhookError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetWebhookError {
    fn description(&self) -> &str {
        match *self {
            GetWebhookError::BadRequest(ref cause) => cause,
            GetWebhookError::InternalFailure(ref cause) => cause,
            GetWebhookError::LimitExceeded(ref cause) => cause,
            GetWebhookError::NotFound(ref cause) => cause,
            GetWebhookError::Unauthorized(ref cause) => cause,
        }
    }
}
/// Errors returned by ListApps
#[derive(Debug, PartialEq)]
pub enum ListAppsError {
    /// <p> Exception thrown when a request contains unexpected data. </p>
    BadRequest(String),
    /// <p> Exception thrown when the service fails to perform an operation due to an internal issue. </p>
    InternalFailure(String),
    /// <p> Exception thrown when an operation fails due to a lack of access. </p>
    Unauthorized(String),
}

impl ListAppsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListAppsError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
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
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for ListAppsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListAppsError {
    fn description(&self) -> &str {
        match *self {
            ListAppsError::BadRequest(ref cause) => cause,
            ListAppsError::InternalFailure(ref cause) => cause,
            ListAppsError::Unauthorized(ref cause) => cause,
        }
    }
}
/// Errors returned by ListBranches
#[derive(Debug, PartialEq)]
pub enum ListBranchesError {
    /// <p> Exception thrown when a request contains unexpected data. </p>
    BadRequest(String),
    /// <p> Exception thrown when the service fails to perform an operation due to an internal issue. </p>
    InternalFailure(String),
    /// <p> Exception thrown when an operation fails due to a lack of access. </p>
    Unauthorized(String),
}

impl ListBranchesError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListBranchesError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
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
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for ListBranchesError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListBranchesError {
    fn description(&self) -> &str {
        match *self {
            ListBranchesError::BadRequest(ref cause) => cause,
            ListBranchesError::InternalFailure(ref cause) => cause,
            ListBranchesError::Unauthorized(ref cause) => cause,
        }
    }
}
/// Errors returned by ListDomainAssociations
#[derive(Debug, PartialEq)]
pub enum ListDomainAssociationsError {
    /// <p> Exception thrown when a request contains unexpected data. </p>
    BadRequest(String),
    /// <p> Exception thrown when the service fails to perform an operation due to an internal issue. </p>
    InternalFailure(String),
    /// <p> Exception thrown when an operation fails due to a lack of access. </p>
    Unauthorized(String),
}

impl ListDomainAssociationsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListDomainAssociationsError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
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
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for ListDomainAssociationsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListDomainAssociationsError {
    fn description(&self) -> &str {
        match *self {
            ListDomainAssociationsError::BadRequest(ref cause) => cause,
            ListDomainAssociationsError::InternalFailure(ref cause) => cause,
            ListDomainAssociationsError::Unauthorized(ref cause) => cause,
        }
    }
}
/// Errors returned by ListJobs
#[derive(Debug, PartialEq)]
pub enum ListJobsError {
    /// <p> Exception thrown when a request contains unexpected data. </p>
    BadRequest(String),
    /// <p> Exception thrown when the service fails to perform an operation due to an internal issue. </p>
    InternalFailure(String),
    /// <p> Exception thrown when a resource could not be created because of service limits. </p>
    LimitExceeded(String),
    /// <p> Exception thrown when an operation fails due to a lack of access. </p>
    Unauthorized(String),
}

impl ListJobsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListJobsError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
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
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for ListJobsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListJobsError {
    fn description(&self) -> &str {
        match *self {
            ListJobsError::BadRequest(ref cause) => cause,
            ListJobsError::InternalFailure(ref cause) => cause,
            ListJobsError::LimitExceeded(ref cause) => cause,
            ListJobsError::Unauthorized(ref cause) => cause,
        }
    }
}
/// Errors returned by ListTagsForResource
#[derive(Debug, PartialEq)]
pub enum ListTagsForResourceError {
    /// <p> Exception thrown when a request contains unexpected data. </p>
    BadRequest(String),
    /// <p> Exception thrown when the service fails to perform an operation due to an internal issue. </p>
    InternalFailure(String),
    /// <p> Exception thrown when an operation fails due to non-existent resource. </p>
    ResourceNotFound(String),
}

impl ListTagsForResourceError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListTagsForResourceError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
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
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for ListTagsForResourceError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListTagsForResourceError {
    fn description(&self) -> &str {
        match *self {
            ListTagsForResourceError::BadRequest(ref cause) => cause,
            ListTagsForResourceError::InternalFailure(ref cause) => cause,
            ListTagsForResourceError::ResourceNotFound(ref cause) => cause,
        }
    }
}
/// Errors returned by ListWebhooks
#[derive(Debug, PartialEq)]
pub enum ListWebhooksError {
    /// <p> Exception thrown when a request contains unexpected data. </p>
    BadRequest(String),
    /// <p> Exception thrown when the service fails to perform an operation due to an internal issue. </p>
    InternalFailure(String),
    /// <p> Exception thrown when a resource could not be created because of service limits. </p>
    LimitExceeded(String),
    /// <p> Exception thrown when an operation fails due to a lack of access. </p>
    Unauthorized(String),
}

impl ListWebhooksError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListWebhooksError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
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
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for ListWebhooksError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListWebhooksError {
    fn description(&self) -> &str {
        match *self {
            ListWebhooksError::BadRequest(ref cause) => cause,
            ListWebhooksError::InternalFailure(ref cause) => cause,
            ListWebhooksError::LimitExceeded(ref cause) => cause,
            ListWebhooksError::Unauthorized(ref cause) => cause,
        }
    }
}
/// Errors returned by StartDeployment
#[derive(Debug, PartialEq)]
pub enum StartDeploymentError {
    /// <p> Exception thrown when a request contains unexpected data. </p>
    BadRequest(String),
    /// <p> Exception thrown when the service fails to perform an operation due to an internal issue. </p>
    InternalFailure(String),
    /// <p> Exception thrown when a resource could not be created because of service limits. </p>
    LimitExceeded(String),
    /// <p> Exception thrown when an entity has not been found during an operation. </p>
    NotFound(String),
    /// <p> Exception thrown when an operation fails due to a lack of access. </p>
    Unauthorized(String),
}

impl StartDeploymentError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<StartDeploymentError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
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
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for StartDeploymentError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for StartDeploymentError {
    fn description(&self) -> &str {
        match *self {
            StartDeploymentError::BadRequest(ref cause) => cause,
            StartDeploymentError::InternalFailure(ref cause) => cause,
            StartDeploymentError::LimitExceeded(ref cause) => cause,
            StartDeploymentError::NotFound(ref cause) => cause,
            StartDeploymentError::Unauthorized(ref cause) => cause,
        }
    }
}
/// Errors returned by StartJob
#[derive(Debug, PartialEq)]
pub enum StartJobError {
    /// <p> Exception thrown when a request contains unexpected data. </p>
    BadRequest(String),
    /// <p> Exception thrown when the service fails to perform an operation due to an internal issue. </p>
    InternalFailure(String),
    /// <p> Exception thrown when a resource could not be created because of service limits. </p>
    LimitExceeded(String),
    /// <p> Exception thrown when an entity has not been found during an operation. </p>
    NotFound(String),
    /// <p> Exception thrown when an operation fails due to a lack of access. </p>
    Unauthorized(String),
}

impl StartJobError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<StartJobError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
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
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for StartJobError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for StartJobError {
    fn description(&self) -> &str {
        match *self {
            StartJobError::BadRequest(ref cause) => cause,
            StartJobError::InternalFailure(ref cause) => cause,
            StartJobError::LimitExceeded(ref cause) => cause,
            StartJobError::NotFound(ref cause) => cause,
            StartJobError::Unauthorized(ref cause) => cause,
        }
    }
}
/// Errors returned by StopJob
#[derive(Debug, PartialEq)]
pub enum StopJobError {
    /// <p> Exception thrown when a request contains unexpected data. </p>
    BadRequest(String),
    /// <p> Exception thrown when the service fails to perform an operation due to an internal issue. </p>
    InternalFailure(String),
    /// <p> Exception thrown when a resource could not be created because of service limits. </p>
    LimitExceeded(String),
    /// <p> Exception thrown when an entity has not been found during an operation. </p>
    NotFound(String),
    /// <p> Exception thrown when an operation fails due to a lack of access. </p>
    Unauthorized(String),
}

impl StopJobError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<StopJobError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
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
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for StopJobError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for StopJobError {
    fn description(&self) -> &str {
        match *self {
            StopJobError::BadRequest(ref cause) => cause,
            StopJobError::InternalFailure(ref cause) => cause,
            StopJobError::LimitExceeded(ref cause) => cause,
            StopJobError::NotFound(ref cause) => cause,
            StopJobError::Unauthorized(ref cause) => cause,
        }
    }
}
/// Errors returned by TagResource
#[derive(Debug, PartialEq)]
pub enum TagResourceError {
    /// <p> Exception thrown when a request contains unexpected data. </p>
    BadRequest(String),
    /// <p> Exception thrown when the service fails to perform an operation due to an internal issue. </p>
    InternalFailure(String),
    /// <p> Exception thrown when an operation fails due to non-existent resource. </p>
    ResourceNotFound(String),
}

impl TagResourceError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<TagResourceError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
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
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for TagResourceError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for TagResourceError {
    fn description(&self) -> &str {
        match *self {
            TagResourceError::BadRequest(ref cause) => cause,
            TagResourceError::InternalFailure(ref cause) => cause,
            TagResourceError::ResourceNotFound(ref cause) => cause,
        }
    }
}
/// Errors returned by UntagResource
#[derive(Debug, PartialEq)]
pub enum UntagResourceError {
    /// <p> Exception thrown when a request contains unexpected data. </p>
    BadRequest(String),
    /// <p> Exception thrown when the service fails to perform an operation due to an internal issue. </p>
    InternalFailure(String),
    /// <p> Exception thrown when an operation fails due to non-existent resource. </p>
    ResourceNotFound(String),
}

impl UntagResourceError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UntagResourceError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
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
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for UntagResourceError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for UntagResourceError {
    fn description(&self) -> &str {
        match *self {
            UntagResourceError::BadRequest(ref cause) => cause,
            UntagResourceError::InternalFailure(ref cause) => cause,
            UntagResourceError::ResourceNotFound(ref cause) => cause,
        }
    }
}
/// Errors returned by UpdateApp
#[derive(Debug, PartialEq)]
pub enum UpdateAppError {
    /// <p> Exception thrown when a request contains unexpected data. </p>
    BadRequest(String),
    /// <p> Exception thrown when the service fails to perform an operation due to an internal issue. </p>
    InternalFailure(String),
    /// <p> Exception thrown when an entity has not been found during an operation. </p>
    NotFound(String),
    /// <p> Exception thrown when an operation fails due to a lack of access. </p>
    Unauthorized(String),
}

impl UpdateAppError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdateAppError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
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
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for UpdateAppError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for UpdateAppError {
    fn description(&self) -> &str {
        match *self {
            UpdateAppError::BadRequest(ref cause) => cause,
            UpdateAppError::InternalFailure(ref cause) => cause,
            UpdateAppError::NotFound(ref cause) => cause,
            UpdateAppError::Unauthorized(ref cause) => cause,
        }
    }
}
/// Errors returned by UpdateBranch
#[derive(Debug, PartialEq)]
pub enum UpdateBranchError {
    /// <p> Exception thrown when a request contains unexpected data. </p>
    BadRequest(String),
    /// <p> Exception thrown when an operation fails due to a dependent service throwing an exception. </p>
    DependentServiceFailure(String),
    /// <p> Exception thrown when the service fails to perform an operation due to an internal issue. </p>
    InternalFailure(String),
    /// <p> Exception thrown when an entity has not been found during an operation. </p>
    NotFound(String),
    /// <p> Exception thrown when an operation fails due to a lack of access. </p>
    Unauthorized(String),
}

impl UpdateBranchError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdateBranchError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
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
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for UpdateBranchError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for UpdateBranchError {
    fn description(&self) -> &str {
        match *self {
            UpdateBranchError::BadRequest(ref cause) => cause,
            UpdateBranchError::DependentServiceFailure(ref cause) => cause,
            UpdateBranchError::InternalFailure(ref cause) => cause,
            UpdateBranchError::NotFound(ref cause) => cause,
            UpdateBranchError::Unauthorized(ref cause) => cause,
        }
    }
}
/// Errors returned by UpdateDomainAssociation
#[derive(Debug, PartialEq)]
pub enum UpdateDomainAssociationError {
    /// <p> Exception thrown when a request contains unexpected data. </p>
    BadRequest(String),
    /// <p> Exception thrown when an operation fails due to a dependent service throwing an exception. </p>
    DependentServiceFailure(String),
    /// <p> Exception thrown when the service fails to perform an operation due to an internal issue. </p>
    InternalFailure(String),
    /// <p> Exception thrown when an entity has not been found during an operation. </p>
    NotFound(String),
    /// <p> Exception thrown when an operation fails due to a lack of access. </p>
    Unauthorized(String),
}

impl UpdateDomainAssociationError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdateDomainAssociationError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
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
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for UpdateDomainAssociationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for UpdateDomainAssociationError {
    fn description(&self) -> &str {
        match *self {
            UpdateDomainAssociationError::BadRequest(ref cause) => cause,
            UpdateDomainAssociationError::DependentServiceFailure(ref cause) => cause,
            UpdateDomainAssociationError::InternalFailure(ref cause) => cause,
            UpdateDomainAssociationError::NotFound(ref cause) => cause,
            UpdateDomainAssociationError::Unauthorized(ref cause) => cause,
        }
    }
}
/// Errors returned by UpdateWebhook
#[derive(Debug, PartialEq)]
pub enum UpdateWebhookError {
    /// <p> Exception thrown when a request contains unexpected data. </p>
    BadRequest(String),
    /// <p> Exception thrown when an operation fails due to a dependent service throwing an exception. </p>
    DependentServiceFailure(String),
    /// <p> Exception thrown when the service fails to perform an operation due to an internal issue. </p>
    InternalFailure(String),
    /// <p> Exception thrown when an entity has not been found during an operation. </p>
    NotFound(String),
    /// <p> Exception thrown when an operation fails due to a lack of access. </p>
    Unauthorized(String),
}

impl UpdateWebhookError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdateWebhookError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
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
        return RusotoError::Unknown(res);
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
            UpdateWebhookError::BadRequest(ref cause) => cause,
            UpdateWebhookError::DependentServiceFailure(ref cause) => cause,
            UpdateWebhookError::InternalFailure(ref cause) => cause,
            UpdateWebhookError::NotFound(ref cause) => cause,
            UpdateWebhookError::Unauthorized(ref cause) => cause,
        }
    }
}
/// Trait representing the capabilities of the Amplify API. Amplify clients implement this trait.
pub trait Amplify {
    /// <p> Creates a new Amplify App. </p>
    fn create_app(&self, input: CreateAppRequest) -> RusotoFuture<CreateAppResult, CreateAppError>;

    /// <p> Creates a new Branch for an Amplify App. </p>
    fn create_branch(
        &self,
        input: CreateBranchRequest,
    ) -> RusotoFuture<CreateBranchResult, CreateBranchError>;

    /// <p> Create a deployment for manual deploy apps. (Apps are not connected to repository) </p>
    fn create_deployment(
        &self,
        input: CreateDeploymentRequest,
    ) -> RusotoFuture<CreateDeploymentResult, CreateDeploymentError>;

    /// <p> Create a new DomainAssociation on an App </p>
    fn create_domain_association(
        &self,
        input: CreateDomainAssociationRequest,
    ) -> RusotoFuture<CreateDomainAssociationResult, CreateDomainAssociationError>;

    /// <p> Create a new webhook on an App. </p>
    fn create_webhook(
        &self,
        input: CreateWebhookRequest,
    ) -> RusotoFuture<CreateWebhookResult, CreateWebhookError>;

    /// <p> Delete an existing Amplify App by appId. </p>
    fn delete_app(&self, input: DeleteAppRequest) -> RusotoFuture<DeleteAppResult, DeleteAppError>;

    /// <p> Deletes a branch for an Amplify App. </p>
    fn delete_branch(
        &self,
        input: DeleteBranchRequest,
    ) -> RusotoFuture<DeleteBranchResult, DeleteBranchError>;

    /// <p> Deletes a DomainAssociation. </p>
    fn delete_domain_association(
        &self,
        input: DeleteDomainAssociationRequest,
    ) -> RusotoFuture<DeleteDomainAssociationResult, DeleteDomainAssociationError>;

    /// <p> Delete a job, for an Amplify branch, part of Amplify App. </p>
    fn delete_job(&self, input: DeleteJobRequest) -> RusotoFuture<DeleteJobResult, DeleteJobError>;

    /// <p> Deletes a webhook. </p>
    fn delete_webhook(
        &self,
        input: DeleteWebhookRequest,
    ) -> RusotoFuture<DeleteWebhookResult, DeleteWebhookError>;

    /// <p> Retrieves an existing Amplify App by appId. </p>
    fn get_app(&self, input: GetAppRequest) -> RusotoFuture<GetAppResult, GetAppError>;

    /// <p> Retrieves a branch for an Amplify App. </p>
    fn get_branch(&self, input: GetBranchRequest) -> RusotoFuture<GetBranchResult, GetBranchError>;

    /// <p> Retrieves domain info that corresponds to an appId and domainName. </p>
    fn get_domain_association(
        &self,
        input: GetDomainAssociationRequest,
    ) -> RusotoFuture<GetDomainAssociationResult, GetDomainAssociationError>;

    /// <p> Get a job for a branch, part of an Amplify App. </p>
    fn get_job(&self, input: GetJobRequest) -> RusotoFuture<GetJobResult, GetJobError>;

    /// <p> Retrieves webhook info that corresponds to a webhookId. </p>
    fn get_webhook(
        &self,
        input: GetWebhookRequest,
    ) -> RusotoFuture<GetWebhookResult, GetWebhookError>;

    /// <p> Lists existing Amplify Apps. </p>
    fn list_apps(&self, input: ListAppsRequest) -> RusotoFuture<ListAppsResult, ListAppsError>;

    /// <p> Lists branches for an Amplify App. </p>
    fn list_branches(
        &self,
        input: ListBranchesRequest,
    ) -> RusotoFuture<ListBranchesResult, ListBranchesError>;

    /// <p> List domains with an app </p>
    fn list_domain_associations(
        &self,
        input: ListDomainAssociationsRequest,
    ) -> RusotoFuture<ListDomainAssociationsResult, ListDomainAssociationsError>;

    /// <p> List Jobs for a branch, part of an Amplify App. </p>
    fn list_jobs(&self, input: ListJobsRequest) -> RusotoFuture<ListJobsResult, ListJobsError>;

    /// <p> List tags for resource. </p>
    fn list_tags_for_resource(
        &self,
        input: ListTagsForResourceRequest,
    ) -> RusotoFuture<ListTagsForResourceResponse, ListTagsForResourceError>;

    /// <p> List webhooks with an app. </p>
    fn list_webhooks(
        &self,
        input: ListWebhooksRequest,
    ) -> RusotoFuture<ListWebhooksResult, ListWebhooksError>;

    /// <p> Start a deployment for manual deploy apps. (Apps are not connected to repository) </p>
    fn start_deployment(
        &self,
        input: StartDeploymentRequest,
    ) -> RusotoFuture<StartDeploymentResult, StartDeploymentError>;

    /// <p> Starts a new job for a branch, part of an Amplify App. </p>
    fn start_job(&self, input: StartJobRequest) -> RusotoFuture<StartJobResult, StartJobError>;

    /// <p> Stop a job that is in progress, for an Amplify branch, part of Amplify App. </p>
    fn stop_job(&self, input: StopJobRequest) -> RusotoFuture<StopJobResult, StopJobError>;

    /// <p> Tag resource with tag key and value. </p>
    fn tag_resource(
        &self,
        input: TagResourceRequest,
    ) -> RusotoFuture<TagResourceResponse, TagResourceError>;

    /// <p> Untag resource with resourceArn. </p>
    fn untag_resource(
        &self,
        input: UntagResourceRequest,
    ) -> RusotoFuture<UntagResourceResponse, UntagResourceError>;

    /// <p> Updates an existing Amplify App. </p>
    fn update_app(&self, input: UpdateAppRequest) -> RusotoFuture<UpdateAppResult, UpdateAppError>;

    /// <p> Updates a branch for an Amplify App. </p>
    fn update_branch(
        &self,
        input: UpdateBranchRequest,
    ) -> RusotoFuture<UpdateBranchResult, UpdateBranchError>;

    /// <p> Create a new DomainAssociation on an App </p>
    fn update_domain_association(
        &self,
        input: UpdateDomainAssociationRequest,
    ) -> RusotoFuture<UpdateDomainAssociationResult, UpdateDomainAssociationError>;

    /// <p> Update a webhook. </p>
    fn update_webhook(
        &self,
        input: UpdateWebhookRequest,
    ) -> RusotoFuture<UpdateWebhookResult, UpdateWebhookError>;
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
        P::Future: Send,
        D: DispatchSignedRequest + Send + Sync + 'static,
        D::Future: Send,
    {
        AmplifyClient {
            client: Client::new_with(credentials_provider, request_dispatcher),
            region,
        }
    }
}

impl Amplify for AmplifyClient {
    /// <p> Creates a new Amplify App. </p>
    fn create_app(&self, input: CreateAppRequest) -> RusotoFuture<CreateAppResult, CreateAppError> {
        let request_uri = "/apps";

        let mut request = SignedRequest::new("POST", "amplify", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<CreateAppResult, _>()?;

                    Ok(result)
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(CreateAppError::from_response(response))),
                )
            }
        })
    }

    /// <p> Creates a new Branch for an Amplify App. </p>
    fn create_branch(
        &self,
        input: CreateBranchRequest,
    ) -> RusotoFuture<CreateBranchResult, CreateBranchError> {
        let request_uri = format!("/apps/{app_id}/branches", app_id = input.app_id);

        let mut request = SignedRequest::new("POST", "amplify", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<CreateBranchResult, _>()?;

                    Ok(result)
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(CreateBranchError::from_response(response))),
                )
            }
        })
    }

    /// <p> Create a deployment for manual deploy apps. (Apps are not connected to repository) </p>
    fn create_deployment(
        &self,
        input: CreateDeploymentRequest,
    ) -> RusotoFuture<CreateDeploymentResult, CreateDeploymentError> {
        let request_uri = format!(
            "/apps/{app_id}/branches/{branch_name}/deployments",
            app_id = input.app_id,
            branch_name = input.branch_name
        );

        let mut request = SignedRequest::new("POST", "amplify", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<CreateDeploymentResult, _>()?;

                    Ok(result)
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(CreateDeploymentError::from_response(response))),
                )
            }
        })
    }

    /// <p> Create a new DomainAssociation on an App </p>
    fn create_domain_association(
        &self,
        input: CreateDomainAssociationRequest,
    ) -> RusotoFuture<CreateDomainAssociationResult, CreateDomainAssociationError> {
        let request_uri = format!("/apps/{app_id}/domains", app_id = input.app_id);

        let mut request = SignedRequest::new("POST", "amplify", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<CreateDomainAssociationResult, _>()?;

                    Ok(result)
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(CreateDomainAssociationError::from_response(response))
                }))
            }
        })
    }

    /// <p> Create a new webhook on an App. </p>
    fn create_webhook(
        &self,
        input: CreateWebhookRequest,
    ) -> RusotoFuture<CreateWebhookResult, CreateWebhookError> {
        let request_uri = format!("/apps/{app_id}/webhooks", app_id = input.app_id);

        let mut request = SignedRequest::new("POST", "amplify", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<CreateWebhookResult, _>()?;

                    Ok(result)
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(CreateWebhookError::from_response(response))),
                )
            }
        })
    }

    /// <p> Delete an existing Amplify App by appId. </p>
    fn delete_app(&self, input: DeleteAppRequest) -> RusotoFuture<DeleteAppResult, DeleteAppError> {
        let request_uri = format!("/apps/{app_id}", app_id = input.app_id);

        let mut request = SignedRequest::new("DELETE", "amplify", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<DeleteAppResult, _>()?;

                    Ok(result)
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(DeleteAppError::from_response(response))),
                )
            }
        })
    }

    /// <p> Deletes a branch for an Amplify App. </p>
    fn delete_branch(
        &self,
        input: DeleteBranchRequest,
    ) -> RusotoFuture<DeleteBranchResult, DeleteBranchError> {
        let request_uri = format!(
            "/apps/{app_id}/branches/{branch_name}",
            app_id = input.app_id,
            branch_name = input.branch_name
        );

        let mut request = SignedRequest::new("DELETE", "amplify", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<DeleteBranchResult, _>()?;

                    Ok(result)
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(DeleteBranchError::from_response(response))),
                )
            }
        })
    }

    /// <p> Deletes a DomainAssociation. </p>
    fn delete_domain_association(
        &self,
        input: DeleteDomainAssociationRequest,
    ) -> RusotoFuture<DeleteDomainAssociationResult, DeleteDomainAssociationError> {
        let request_uri = format!(
            "/apps/{app_id}/domains/{domain_name}",
            app_id = input.app_id,
            domain_name = input.domain_name
        );

        let mut request = SignedRequest::new("DELETE", "amplify", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<DeleteDomainAssociationResult, _>()?;

                    Ok(result)
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(DeleteDomainAssociationError::from_response(response))
                }))
            }
        })
    }

    /// <p> Delete a job, for an Amplify branch, part of Amplify App. </p>
    fn delete_job(&self, input: DeleteJobRequest) -> RusotoFuture<DeleteJobResult, DeleteJobError> {
        let request_uri = format!(
            "/apps/{app_id}/branches/{branch_name}/jobs/{job_id}",
            app_id = input.app_id,
            branch_name = input.branch_name,
            job_id = input.job_id
        );

        let mut request = SignedRequest::new("DELETE", "amplify", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<DeleteJobResult, _>()?;

                    Ok(result)
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(DeleteJobError::from_response(response))),
                )
            }
        })
    }

    /// <p> Deletes a webhook. </p>
    fn delete_webhook(
        &self,
        input: DeleteWebhookRequest,
    ) -> RusotoFuture<DeleteWebhookResult, DeleteWebhookError> {
        let request_uri = format!("/webhooks/{webhook_id}", webhook_id = input.webhook_id);

        let mut request = SignedRequest::new("DELETE", "amplify", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<DeleteWebhookResult, _>()?;

                    Ok(result)
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(DeleteWebhookError::from_response(response))),
                )
            }
        })
    }

    /// <p> Retrieves an existing Amplify App by appId. </p>
    fn get_app(&self, input: GetAppRequest) -> RusotoFuture<GetAppResult, GetAppError> {
        let request_uri = format!("/apps/{app_id}", app_id = input.app_id);

        let mut request = SignedRequest::new("GET", "amplify", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<GetAppResult, _>()?;

                    Ok(result)
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(GetAppError::from_response(response))),
                )
            }
        })
    }

    /// <p> Retrieves a branch for an Amplify App. </p>
    fn get_branch(&self, input: GetBranchRequest) -> RusotoFuture<GetBranchResult, GetBranchError> {
        let request_uri = format!(
            "/apps/{app_id}/branches/{branch_name}",
            app_id = input.app_id,
            branch_name = input.branch_name
        );

        let mut request = SignedRequest::new("GET", "amplify", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<GetBranchResult, _>()?;

                    Ok(result)
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(GetBranchError::from_response(response))),
                )
            }
        })
    }

    /// <p> Retrieves domain info that corresponds to an appId and domainName. </p>
    fn get_domain_association(
        &self,
        input: GetDomainAssociationRequest,
    ) -> RusotoFuture<GetDomainAssociationResult, GetDomainAssociationError> {
        let request_uri = format!(
            "/apps/{app_id}/domains/{domain_name}",
            app_id = input.app_id,
            domain_name = input.domain_name
        );

        let mut request = SignedRequest::new("GET", "amplify", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<GetDomainAssociationResult, _>()?;

                    Ok(result)
                }))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(GetDomainAssociationError::from_response(response))
                    }),
                )
            }
        })
    }

    /// <p> Get a job for a branch, part of an Amplify App. </p>
    fn get_job(&self, input: GetJobRequest) -> RusotoFuture<GetJobResult, GetJobError> {
        let request_uri = format!(
            "/apps/{app_id}/branches/{branch_name}/jobs/{job_id}",
            app_id = input.app_id,
            branch_name = input.branch_name,
            job_id = input.job_id
        );

        let mut request = SignedRequest::new("GET", "amplify", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<GetJobResult, _>()?;

                    Ok(result)
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(GetJobError::from_response(response))),
                )
            }
        })
    }

    /// <p> Retrieves webhook info that corresponds to a webhookId. </p>
    fn get_webhook(
        &self,
        input: GetWebhookRequest,
    ) -> RusotoFuture<GetWebhookResult, GetWebhookError> {
        let request_uri = format!("/webhooks/{webhook_id}", webhook_id = input.webhook_id);

        let mut request = SignedRequest::new("GET", "amplify", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<GetWebhookResult, _>()?;

                    Ok(result)
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(GetWebhookError::from_response(response))),
                )
            }
        })
    }

    /// <p> Lists existing Amplify Apps. </p>
    fn list_apps(&self, input: ListAppsRequest) -> RusotoFuture<ListAppsResult, ListAppsError> {
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

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<ListAppsResult, _>()?;

                    Ok(result)
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(ListAppsError::from_response(response))),
                )
            }
        })
    }

    /// <p> Lists branches for an Amplify App. </p>
    fn list_branches(
        &self,
        input: ListBranchesRequest,
    ) -> RusotoFuture<ListBranchesResult, ListBranchesError> {
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

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<ListBranchesResult, _>()?;

                    Ok(result)
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(ListBranchesError::from_response(response))),
                )
            }
        })
    }

    /// <p> List domains with an app </p>
    fn list_domain_associations(
        &self,
        input: ListDomainAssociationsRequest,
    ) -> RusotoFuture<ListDomainAssociationsResult, ListDomainAssociationsError> {
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

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<ListDomainAssociationsResult, _>()?;

                    Ok(result)
                }))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(ListDomainAssociationsError::from_response(response))
                    }),
                )
            }
        })
    }

    /// <p> List Jobs for a branch, part of an Amplify App. </p>
    fn list_jobs(&self, input: ListJobsRequest) -> RusotoFuture<ListJobsResult, ListJobsError> {
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

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<ListJobsResult, _>()?;

                    Ok(result)
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(ListJobsError::from_response(response))),
                )
            }
        })
    }

    /// <p> List tags for resource. </p>
    fn list_tags_for_resource(
        &self,
        input: ListTagsForResourceRequest,
    ) -> RusotoFuture<ListTagsForResourceResponse, ListTagsForResourceError> {
        let request_uri = format!("/tags/{resource_arn}", resource_arn = input.resource_arn);

        let mut request = SignedRequest::new("GET", "amplify", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<ListTagsForResourceResponse, _>()?;

                    Ok(result)
                }))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(ListTagsForResourceError::from_response(response))
                    }),
                )
            }
        })
    }

    /// <p> List webhooks with an app. </p>
    fn list_webhooks(
        &self,
        input: ListWebhooksRequest,
    ) -> RusotoFuture<ListWebhooksResult, ListWebhooksError> {
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

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<ListWebhooksResult, _>()?;

                    Ok(result)
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(ListWebhooksError::from_response(response))),
                )
            }
        })
    }

    /// <p> Start a deployment for manual deploy apps. (Apps are not connected to repository) </p>
    fn start_deployment(
        &self,
        input: StartDeploymentRequest,
    ) -> RusotoFuture<StartDeploymentResult, StartDeploymentError> {
        let request_uri = format!(
            "/apps/{app_id}/branches/{branch_name}/deployments/start",
            app_id = input.app_id,
            branch_name = input.branch_name
        );

        let mut request = SignedRequest::new("POST", "amplify", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<StartDeploymentResult, _>()?;

                    Ok(result)
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(StartDeploymentError::from_response(response))),
                )
            }
        })
    }

    /// <p> Starts a new job for a branch, part of an Amplify App. </p>
    fn start_job(&self, input: StartJobRequest) -> RusotoFuture<StartJobResult, StartJobError> {
        let request_uri = format!(
            "/apps/{app_id}/branches/{branch_name}/jobs",
            app_id = input.app_id,
            branch_name = input.branch_name
        );

        let mut request = SignedRequest::new("POST", "amplify", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<StartJobResult, _>()?;

                    Ok(result)
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(StartJobError::from_response(response))),
                )
            }
        })
    }

    /// <p> Stop a job that is in progress, for an Amplify branch, part of Amplify App. </p>
    fn stop_job(&self, input: StopJobRequest) -> RusotoFuture<StopJobResult, StopJobError> {
        let request_uri = format!(
            "/apps/{app_id}/branches/{branch_name}/jobs/{job_id}/stop",
            app_id = input.app_id,
            branch_name = input.branch_name,
            job_id = input.job_id
        );

        let mut request = SignedRequest::new("DELETE", "amplify", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<StopJobResult, _>()?;

                    Ok(result)
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(StopJobError::from_response(response))),
                )
            }
        })
    }

    /// <p> Tag resource with tag key and value. </p>
    fn tag_resource(
        &self,
        input: TagResourceRequest,
    ) -> RusotoFuture<TagResourceResponse, TagResourceError> {
        let request_uri = format!("/tags/{resource_arn}", resource_arn = input.resource_arn);

        let mut request = SignedRequest::new("POST", "amplify", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<TagResourceResponse, _>()?;

                    Ok(result)
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(TagResourceError::from_response(response))),
                )
            }
        })
    }

    /// <p> Untag resource with resourceArn. </p>
    fn untag_resource(
        &self,
        input: UntagResourceRequest,
    ) -> RusotoFuture<UntagResourceResponse, UntagResourceError> {
        let request_uri = format!("/tags/{resource_arn}", resource_arn = input.resource_arn);

        let mut request = SignedRequest::new("DELETE", "amplify", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut params = Params::new();
        for item in input.tag_keys.iter() {
            params.put("tagKeys", item);
        }
        request.set_params(params);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<UntagResourceResponse, _>()?;

                    Ok(result)
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(UntagResourceError::from_response(response))),
                )
            }
        })
    }

    /// <p> Updates an existing Amplify App. </p>
    fn update_app(&self, input: UpdateAppRequest) -> RusotoFuture<UpdateAppResult, UpdateAppError> {
        let request_uri = format!("/apps/{app_id}", app_id = input.app_id);

        let mut request = SignedRequest::new("POST", "amplify", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<UpdateAppResult, _>()?;

                    Ok(result)
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(UpdateAppError::from_response(response))),
                )
            }
        })
    }

    /// <p> Updates a branch for an Amplify App. </p>
    fn update_branch(
        &self,
        input: UpdateBranchRequest,
    ) -> RusotoFuture<UpdateBranchResult, UpdateBranchError> {
        let request_uri = format!(
            "/apps/{app_id}/branches/{branch_name}",
            app_id = input.app_id,
            branch_name = input.branch_name
        );

        let mut request = SignedRequest::new("POST", "amplify", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<UpdateBranchResult, _>()?;

                    Ok(result)
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(UpdateBranchError::from_response(response))),
                )
            }
        })
    }

    /// <p> Create a new DomainAssociation on an App </p>
    fn update_domain_association(
        &self,
        input: UpdateDomainAssociationRequest,
    ) -> RusotoFuture<UpdateDomainAssociationResult, UpdateDomainAssociationError> {
        let request_uri = format!(
            "/apps/{app_id}/domains/{domain_name}",
            app_id = input.app_id,
            domain_name = input.domain_name
        );

        let mut request = SignedRequest::new("POST", "amplify", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<UpdateDomainAssociationResult, _>()?;

                    Ok(result)
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(UpdateDomainAssociationError::from_response(response))
                }))
            }
        })
    }

    /// <p> Update a webhook. </p>
    fn update_webhook(
        &self,
        input: UpdateWebhookRequest,
    ) -> RusotoFuture<UpdateWebhookResult, UpdateWebhookError> {
        let request_uri = format!("/webhooks/{webhook_id}", webhook_id = input.webhook_id);

        let mut request = SignedRequest::new("POST", "amplify", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<UpdateWebhookResult, _>()?;

                    Ok(result)
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(UpdateWebhookError::from_response(response))),
                )
            }
        })
    }
}
