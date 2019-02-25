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
use rusoto_core::request::{BufferedHttpResponse, DispatchSignedRequest};
use rusoto_core::{Client, RusotoFuture};

use rusoto_core::credential::{CredentialsError, ProvideAwsCredentials};
use rusoto_core::request::HttpDispatchError;

use rusoto_core::param::{Params, ServiceParams};
use rusoto_core::signature::SignedRequest;
use serde_json;
use serde_json::from_slice;
use serde_json::Value as SerdeJsonValue;
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

/// <p> Branch for an Amplify App, which maps to a 3rd party repository branch. </p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct Branch {
    /// <p> Id of the active job for a branch, part of an Amplify App. </p>
    #[serde(rename = "activeJobId")]
    pub active_job_id: String,
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
    /// <p> Display name for a branch, part of an Amplify App. </p>
    #[serde(rename = "displayName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
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
    /// <p> Thumbnail Url for the branch. </p>
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
    pub oauth_token: String,
    /// <p> Platform / framework for an Amplify App </p>
    #[serde(rename = "platform")]
    pub platform: String,
    /// <p> Repository for an Amplify App </p>
    #[serde(rename = "repository")]
    pub repository: String,
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

/// <p> Structure for Domain Association, which associates a custom domain with an Amplify App. </p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct DomainAssociation {
    /// <p> DNS Record for certificate verification. </p>
    #[serde(rename = "certificateVerificationDNSRecord")]
    pub certificate_verification_dns_record: String,
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
    /// <p> Type for the Job. </p>
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
    /// <p> Thumbnail Url for Production Branch. </p>
    #[serde(rename = "thumbnailUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumbnail_url: Option<String>,
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
    /// <p> Unique Id for the Job. </p>
    #[serde(rename = "jobId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_id: Option<String>,
    /// <p> Reason for the Job. </p>
    #[serde(rename = "jobReason")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_reason: Option<String>,
    /// <p> Type for the Job. </p>
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
    /// <p> Url to teh artifact for the execution step. </p>
    #[serde(rename = "artifactsUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub artifacts_url: Option<String>,
    /// <p> End date/ time of the execution step. </p>
    #[serde(rename = "endTime")]
    pub end_time: f64,
    /// <p> Url to the logs for the execution step. </p>
    #[serde(rename = "logUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_url: Option<String>,
    /// <p> List of screenshot Urls for the execution step, if relevant. </p>
    #[serde(rename = "screenshots")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub screenshots: Option<::std::collections::HashMap<String, String>>,
    /// <p> Start date/ time of the execution step. </p>
    #[serde(rename = "startTime")]
    pub start_time: f64,
    /// <p> Status of the execution step. </p>
    #[serde(rename = "status")]
    pub status: String,
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

/// <p> Request structure for update App request. </p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct UpdateAppRequest {
    /// <p> Unique Id for an Amplify App. </p>
    #[serde(rename = "appId")]
    pub app_id: String,
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
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl CreateAppError {
    // see boto RestJSONParser impl for parsing errors
    // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L838-L850
    pub fn from_response(res: BufferedHttpResponse) -> CreateAppError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let error_type = match res.headers.get("x-amzn-errortype") {
                Some(raw_error_type) => raw_error_type
                    .split(':')
                    .next()
                    .unwrap_or_else(|| "Unknown"),
                _ => json
                    .get("code")
                    .or_else(|| json.get("Code"))
                    .and_then(|c| c.as_str())
                    .unwrap_or_else(|| "Unknown"),
            };

            // message can come in either "message" or "Message"
            // see boto BaseJSONParser impl for parsing message
            // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L595-L598
            let error_message = json
                .get("message")
                .or_else(|| json.get("Message"))
                .and_then(|m| m.as_str())
                .unwrap_or("");

            match error_type {
                "BadRequestException" => {
                    return CreateAppError::BadRequest(String::from(error_message));
                }
                "DependentServiceFailureException" => {
                    return CreateAppError::DependentServiceFailure(String::from(error_message));
                }
                "InternalFailureException" => {
                    return CreateAppError::InternalFailure(String::from(error_message));
                }
                "LimitExceededException" => {
                    return CreateAppError::LimitExceeded(String::from(error_message));
                }
                "UnauthorizedException" => {
                    return CreateAppError::Unauthorized(String::from(error_message));
                }
                "ValidationException" => {
                    return CreateAppError::Validation(error_message.to_string());
                }
                _ => {}
            }
        }
        return CreateAppError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for CreateAppError {
    fn from(err: serde_json::error::Error) -> CreateAppError {
        CreateAppError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for CreateAppError {
    fn from(err: CredentialsError) -> CreateAppError {
        CreateAppError::Credentials(err)
    }
}
impl From<HttpDispatchError> for CreateAppError {
    fn from(err: HttpDispatchError) -> CreateAppError {
        CreateAppError::HttpDispatch(err)
    }
}
impl From<io::Error> for CreateAppError {
    fn from(err: io::Error) -> CreateAppError {
        CreateAppError::HttpDispatch(HttpDispatchError::from(err))
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
            CreateAppError::Validation(ref cause) => cause,
            CreateAppError::Credentials(ref err) => err.description(),
            CreateAppError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            CreateAppError::ParseError(ref cause) => cause,
            CreateAppError::Unknown(_) => "unknown error",
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
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl CreateBranchError {
    // see boto RestJSONParser impl for parsing errors
    // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L838-L850
    pub fn from_response(res: BufferedHttpResponse) -> CreateBranchError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let error_type = match res.headers.get("x-amzn-errortype") {
                Some(raw_error_type) => raw_error_type
                    .split(':')
                    .next()
                    .unwrap_or_else(|| "Unknown"),
                _ => json
                    .get("code")
                    .or_else(|| json.get("Code"))
                    .and_then(|c| c.as_str())
                    .unwrap_or_else(|| "Unknown"),
            };

            // message can come in either "message" or "Message"
            // see boto BaseJSONParser impl for parsing message
            // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L595-L598
            let error_message = json
                .get("message")
                .or_else(|| json.get("Message"))
                .and_then(|m| m.as_str())
                .unwrap_or("");

            match error_type {
                "BadRequestException" => {
                    return CreateBranchError::BadRequest(String::from(error_message));
                }
                "DependentServiceFailureException" => {
                    return CreateBranchError::DependentServiceFailure(String::from(error_message));
                }
                "InternalFailureException" => {
                    return CreateBranchError::InternalFailure(String::from(error_message));
                }
                "LimitExceededException" => {
                    return CreateBranchError::LimitExceeded(String::from(error_message));
                }
                "NotFoundException" => {
                    return CreateBranchError::NotFound(String::from(error_message));
                }
                "UnauthorizedException" => {
                    return CreateBranchError::Unauthorized(String::from(error_message));
                }
                "ValidationException" => {
                    return CreateBranchError::Validation(error_message.to_string());
                }
                _ => {}
            }
        }
        return CreateBranchError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for CreateBranchError {
    fn from(err: serde_json::error::Error) -> CreateBranchError {
        CreateBranchError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for CreateBranchError {
    fn from(err: CredentialsError) -> CreateBranchError {
        CreateBranchError::Credentials(err)
    }
}
impl From<HttpDispatchError> for CreateBranchError {
    fn from(err: HttpDispatchError) -> CreateBranchError {
        CreateBranchError::HttpDispatch(err)
    }
}
impl From<io::Error> for CreateBranchError {
    fn from(err: io::Error) -> CreateBranchError {
        CreateBranchError::HttpDispatch(HttpDispatchError::from(err))
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
            CreateBranchError::Validation(ref cause) => cause,
            CreateBranchError::Credentials(ref err) => err.description(),
            CreateBranchError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            CreateBranchError::ParseError(ref cause) => cause,
            CreateBranchError::Unknown(_) => "unknown error",
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
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl CreateDomainAssociationError {
    // see boto RestJSONParser impl for parsing errors
    // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L838-L850
    pub fn from_response(res: BufferedHttpResponse) -> CreateDomainAssociationError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let error_type = match res.headers.get("x-amzn-errortype") {
                Some(raw_error_type) => raw_error_type
                    .split(':')
                    .next()
                    .unwrap_or_else(|| "Unknown"),
                _ => json
                    .get("code")
                    .or_else(|| json.get("Code"))
                    .and_then(|c| c.as_str())
                    .unwrap_or_else(|| "Unknown"),
            };

            // message can come in either "message" or "Message"
            // see boto BaseJSONParser impl for parsing message
            // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L595-L598
            let error_message = json
                .get("message")
                .or_else(|| json.get("Message"))
                .and_then(|m| m.as_str())
                .unwrap_or("");

            match error_type {
                "BadRequestException" => {
                    return CreateDomainAssociationError::BadRequest(String::from(error_message));
                }
                "DependentServiceFailureException" => {
                    return CreateDomainAssociationError::DependentServiceFailure(String::from(
                        error_message,
                    ));
                }
                "InternalFailureException" => {
                    return CreateDomainAssociationError::InternalFailure(String::from(
                        error_message,
                    ));
                }
                "LimitExceededException" => {
                    return CreateDomainAssociationError::LimitExceeded(String::from(error_message));
                }
                "NotFoundException" => {
                    return CreateDomainAssociationError::NotFound(String::from(error_message));
                }
                "UnauthorizedException" => {
                    return CreateDomainAssociationError::Unauthorized(String::from(error_message));
                }
                "ValidationException" => {
                    return CreateDomainAssociationError::Validation(error_message.to_string());
                }
                _ => {}
            }
        }
        return CreateDomainAssociationError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for CreateDomainAssociationError {
    fn from(err: serde_json::error::Error) -> CreateDomainAssociationError {
        CreateDomainAssociationError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for CreateDomainAssociationError {
    fn from(err: CredentialsError) -> CreateDomainAssociationError {
        CreateDomainAssociationError::Credentials(err)
    }
}
impl From<HttpDispatchError> for CreateDomainAssociationError {
    fn from(err: HttpDispatchError) -> CreateDomainAssociationError {
        CreateDomainAssociationError::HttpDispatch(err)
    }
}
impl From<io::Error> for CreateDomainAssociationError {
    fn from(err: io::Error) -> CreateDomainAssociationError {
        CreateDomainAssociationError::HttpDispatch(HttpDispatchError::from(err))
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
            CreateDomainAssociationError::Validation(ref cause) => cause,
            CreateDomainAssociationError::Credentials(ref err) => err.description(),
            CreateDomainAssociationError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            CreateDomainAssociationError::ParseError(ref cause) => cause,
            CreateDomainAssociationError::Unknown(_) => "unknown error",
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
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl DeleteAppError {
    // see boto RestJSONParser impl for parsing errors
    // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L838-L850
    pub fn from_response(res: BufferedHttpResponse) -> DeleteAppError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let error_type = match res.headers.get("x-amzn-errortype") {
                Some(raw_error_type) => raw_error_type
                    .split(':')
                    .next()
                    .unwrap_or_else(|| "Unknown"),
                _ => json
                    .get("code")
                    .or_else(|| json.get("Code"))
                    .and_then(|c| c.as_str())
                    .unwrap_or_else(|| "Unknown"),
            };

            // message can come in either "message" or "Message"
            // see boto BaseJSONParser impl for parsing message
            // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L595-L598
            let error_message = json
                .get("message")
                .or_else(|| json.get("Message"))
                .and_then(|m| m.as_str())
                .unwrap_or("");

            match error_type {
                "BadRequestException" => {
                    return DeleteAppError::BadRequest(String::from(error_message));
                }
                "DependentServiceFailureException" => {
                    return DeleteAppError::DependentServiceFailure(String::from(error_message));
                }
                "InternalFailureException" => {
                    return DeleteAppError::InternalFailure(String::from(error_message));
                }
                "NotFoundException" => return DeleteAppError::NotFound(String::from(error_message)),
                "UnauthorizedException" => {
                    return DeleteAppError::Unauthorized(String::from(error_message));
                }
                "ValidationException" => {
                    return DeleteAppError::Validation(error_message.to_string());
                }
                _ => {}
            }
        }
        return DeleteAppError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for DeleteAppError {
    fn from(err: serde_json::error::Error) -> DeleteAppError {
        DeleteAppError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for DeleteAppError {
    fn from(err: CredentialsError) -> DeleteAppError {
        DeleteAppError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DeleteAppError {
    fn from(err: HttpDispatchError) -> DeleteAppError {
        DeleteAppError::HttpDispatch(err)
    }
}
impl From<io::Error> for DeleteAppError {
    fn from(err: io::Error) -> DeleteAppError {
        DeleteAppError::HttpDispatch(HttpDispatchError::from(err))
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
            DeleteAppError::Validation(ref cause) => cause,
            DeleteAppError::Credentials(ref err) => err.description(),
            DeleteAppError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            DeleteAppError::ParseError(ref cause) => cause,
            DeleteAppError::Unknown(_) => "unknown error",
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
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl DeleteBranchError {
    // see boto RestJSONParser impl for parsing errors
    // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L838-L850
    pub fn from_response(res: BufferedHttpResponse) -> DeleteBranchError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let error_type = match res.headers.get("x-amzn-errortype") {
                Some(raw_error_type) => raw_error_type
                    .split(':')
                    .next()
                    .unwrap_or_else(|| "Unknown"),
                _ => json
                    .get("code")
                    .or_else(|| json.get("Code"))
                    .and_then(|c| c.as_str())
                    .unwrap_or_else(|| "Unknown"),
            };

            // message can come in either "message" or "Message"
            // see boto BaseJSONParser impl for parsing message
            // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L595-L598
            let error_message = json
                .get("message")
                .or_else(|| json.get("Message"))
                .and_then(|m| m.as_str())
                .unwrap_or("");

            match error_type {
                "BadRequestException" => {
                    return DeleteBranchError::BadRequest(String::from(error_message));
                }
                "DependentServiceFailureException" => {
                    return DeleteBranchError::DependentServiceFailure(String::from(error_message));
                }
                "InternalFailureException" => {
                    return DeleteBranchError::InternalFailure(String::from(error_message));
                }
                "NotFoundException" => {
                    return DeleteBranchError::NotFound(String::from(error_message));
                }
                "UnauthorizedException" => {
                    return DeleteBranchError::Unauthorized(String::from(error_message));
                }
                "ValidationException" => {
                    return DeleteBranchError::Validation(error_message.to_string());
                }
                _ => {}
            }
        }
        return DeleteBranchError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for DeleteBranchError {
    fn from(err: serde_json::error::Error) -> DeleteBranchError {
        DeleteBranchError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for DeleteBranchError {
    fn from(err: CredentialsError) -> DeleteBranchError {
        DeleteBranchError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DeleteBranchError {
    fn from(err: HttpDispatchError) -> DeleteBranchError {
        DeleteBranchError::HttpDispatch(err)
    }
}
impl From<io::Error> for DeleteBranchError {
    fn from(err: io::Error) -> DeleteBranchError {
        DeleteBranchError::HttpDispatch(HttpDispatchError::from(err))
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
            DeleteBranchError::Validation(ref cause) => cause,
            DeleteBranchError::Credentials(ref err) => err.description(),
            DeleteBranchError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            DeleteBranchError::ParseError(ref cause) => cause,
            DeleteBranchError::Unknown(_) => "unknown error",
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
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl DeleteDomainAssociationError {
    // see boto RestJSONParser impl for parsing errors
    // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L838-L850
    pub fn from_response(res: BufferedHttpResponse) -> DeleteDomainAssociationError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let error_type = match res.headers.get("x-amzn-errortype") {
                Some(raw_error_type) => raw_error_type
                    .split(':')
                    .next()
                    .unwrap_or_else(|| "Unknown"),
                _ => json
                    .get("code")
                    .or_else(|| json.get("Code"))
                    .and_then(|c| c.as_str())
                    .unwrap_or_else(|| "Unknown"),
            };

            // message can come in either "message" or "Message"
            // see boto BaseJSONParser impl for parsing message
            // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L595-L598
            let error_message = json
                .get("message")
                .or_else(|| json.get("Message"))
                .and_then(|m| m.as_str())
                .unwrap_or("");

            match error_type {
                "BadRequestException" => {
                    return DeleteDomainAssociationError::BadRequest(String::from(error_message));
                }
                "DependentServiceFailureException" => {
                    return DeleteDomainAssociationError::DependentServiceFailure(String::from(
                        error_message,
                    ));
                }
                "InternalFailureException" => {
                    return DeleteDomainAssociationError::InternalFailure(String::from(
                        error_message,
                    ));
                }
                "NotFoundException" => {
                    return DeleteDomainAssociationError::NotFound(String::from(error_message));
                }
                "UnauthorizedException" => {
                    return DeleteDomainAssociationError::Unauthorized(String::from(error_message));
                }
                "ValidationException" => {
                    return DeleteDomainAssociationError::Validation(error_message.to_string());
                }
                _ => {}
            }
        }
        return DeleteDomainAssociationError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for DeleteDomainAssociationError {
    fn from(err: serde_json::error::Error) -> DeleteDomainAssociationError {
        DeleteDomainAssociationError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for DeleteDomainAssociationError {
    fn from(err: CredentialsError) -> DeleteDomainAssociationError {
        DeleteDomainAssociationError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DeleteDomainAssociationError {
    fn from(err: HttpDispatchError) -> DeleteDomainAssociationError {
        DeleteDomainAssociationError::HttpDispatch(err)
    }
}
impl From<io::Error> for DeleteDomainAssociationError {
    fn from(err: io::Error) -> DeleteDomainAssociationError {
        DeleteDomainAssociationError::HttpDispatch(HttpDispatchError::from(err))
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
            DeleteDomainAssociationError::Validation(ref cause) => cause,
            DeleteDomainAssociationError::Credentials(ref err) => err.description(),
            DeleteDomainAssociationError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DeleteDomainAssociationError::ParseError(ref cause) => cause,
            DeleteDomainAssociationError::Unknown(_) => "unknown error",
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
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl DeleteJobError {
    // see boto RestJSONParser impl for parsing errors
    // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L838-L850
    pub fn from_response(res: BufferedHttpResponse) -> DeleteJobError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let error_type = match res.headers.get("x-amzn-errortype") {
                Some(raw_error_type) => raw_error_type
                    .split(':')
                    .next()
                    .unwrap_or_else(|| "Unknown"),
                _ => json
                    .get("code")
                    .or_else(|| json.get("Code"))
                    .and_then(|c| c.as_str())
                    .unwrap_or_else(|| "Unknown"),
            };

            // message can come in either "message" or "Message"
            // see boto BaseJSONParser impl for parsing message
            // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L595-L598
            let error_message = json
                .get("message")
                .or_else(|| json.get("Message"))
                .and_then(|m| m.as_str())
                .unwrap_or("");

            match error_type {
                "BadRequestException" => {
                    return DeleteJobError::BadRequest(String::from(error_message));
                }
                "InternalFailureException" => {
                    return DeleteJobError::InternalFailure(String::from(error_message));
                }
                "LimitExceededException" => {
                    return DeleteJobError::LimitExceeded(String::from(error_message));
                }
                "NotFoundException" => return DeleteJobError::NotFound(String::from(error_message)),
                "UnauthorizedException" => {
                    return DeleteJobError::Unauthorized(String::from(error_message));
                }
                "ValidationException" => {
                    return DeleteJobError::Validation(error_message.to_string());
                }
                _ => {}
            }
        }
        return DeleteJobError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for DeleteJobError {
    fn from(err: serde_json::error::Error) -> DeleteJobError {
        DeleteJobError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for DeleteJobError {
    fn from(err: CredentialsError) -> DeleteJobError {
        DeleteJobError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DeleteJobError {
    fn from(err: HttpDispatchError) -> DeleteJobError {
        DeleteJobError::HttpDispatch(err)
    }
}
impl From<io::Error> for DeleteJobError {
    fn from(err: io::Error) -> DeleteJobError {
        DeleteJobError::HttpDispatch(HttpDispatchError::from(err))
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
            DeleteJobError::Validation(ref cause) => cause,
            DeleteJobError::Credentials(ref err) => err.description(),
            DeleteJobError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            DeleteJobError::ParseError(ref cause) => cause,
            DeleteJobError::Unknown(_) => "unknown error",
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
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl GetAppError {
    // see boto RestJSONParser impl for parsing errors
    // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L838-L850
    pub fn from_response(res: BufferedHttpResponse) -> GetAppError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let error_type = match res.headers.get("x-amzn-errortype") {
                Some(raw_error_type) => raw_error_type
                    .split(':')
                    .next()
                    .unwrap_or_else(|| "Unknown"),
                _ => json
                    .get("code")
                    .or_else(|| json.get("Code"))
                    .and_then(|c| c.as_str())
                    .unwrap_or_else(|| "Unknown"),
            };

            // message can come in either "message" or "Message"
            // see boto BaseJSONParser impl for parsing message
            // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L595-L598
            let error_message = json
                .get("message")
                .or_else(|| json.get("Message"))
                .and_then(|m| m.as_str())
                .unwrap_or("");

            match error_type {
                "BadRequestException" => {
                    return GetAppError::BadRequest(String::from(error_message));
                }
                "InternalFailureException" => {
                    return GetAppError::InternalFailure(String::from(error_message));
                }
                "NotFoundException" => return GetAppError::NotFound(String::from(error_message)),
                "UnauthorizedException" => {
                    return GetAppError::Unauthorized(String::from(error_message));
                }
                "ValidationException" => return GetAppError::Validation(error_message.to_string()),
                _ => {}
            }
        }
        return GetAppError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for GetAppError {
    fn from(err: serde_json::error::Error) -> GetAppError {
        GetAppError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for GetAppError {
    fn from(err: CredentialsError) -> GetAppError {
        GetAppError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GetAppError {
    fn from(err: HttpDispatchError) -> GetAppError {
        GetAppError::HttpDispatch(err)
    }
}
impl From<io::Error> for GetAppError {
    fn from(err: io::Error) -> GetAppError {
        GetAppError::HttpDispatch(HttpDispatchError::from(err))
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
            GetAppError::Validation(ref cause) => cause,
            GetAppError::Credentials(ref err) => err.description(),
            GetAppError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            GetAppError::ParseError(ref cause) => cause,
            GetAppError::Unknown(_) => "unknown error",
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
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl GetBranchError {
    // see boto RestJSONParser impl for parsing errors
    // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L838-L850
    pub fn from_response(res: BufferedHttpResponse) -> GetBranchError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let error_type = match res.headers.get("x-amzn-errortype") {
                Some(raw_error_type) => raw_error_type
                    .split(':')
                    .next()
                    .unwrap_or_else(|| "Unknown"),
                _ => json
                    .get("code")
                    .or_else(|| json.get("Code"))
                    .and_then(|c| c.as_str())
                    .unwrap_or_else(|| "Unknown"),
            };

            // message can come in either "message" or "Message"
            // see boto BaseJSONParser impl for parsing message
            // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L595-L598
            let error_message = json
                .get("message")
                .or_else(|| json.get("Message"))
                .and_then(|m| m.as_str())
                .unwrap_or("");

            match error_type {
                "BadRequestException" => {
                    return GetBranchError::BadRequest(String::from(error_message));
                }
                "InternalFailureException" => {
                    return GetBranchError::InternalFailure(String::from(error_message));
                }
                "NotFoundException" => return GetBranchError::NotFound(String::from(error_message)),
                "UnauthorizedException" => {
                    return GetBranchError::Unauthorized(String::from(error_message));
                }
                "ValidationException" => {
                    return GetBranchError::Validation(error_message.to_string());
                }
                _ => {}
            }
        }
        return GetBranchError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for GetBranchError {
    fn from(err: serde_json::error::Error) -> GetBranchError {
        GetBranchError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for GetBranchError {
    fn from(err: CredentialsError) -> GetBranchError {
        GetBranchError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GetBranchError {
    fn from(err: HttpDispatchError) -> GetBranchError {
        GetBranchError::HttpDispatch(err)
    }
}
impl From<io::Error> for GetBranchError {
    fn from(err: io::Error) -> GetBranchError {
        GetBranchError::HttpDispatch(HttpDispatchError::from(err))
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
            GetBranchError::Validation(ref cause) => cause,
            GetBranchError::Credentials(ref err) => err.description(),
            GetBranchError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            GetBranchError::ParseError(ref cause) => cause,
            GetBranchError::Unknown(_) => "unknown error",
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
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl GetDomainAssociationError {
    // see boto RestJSONParser impl for parsing errors
    // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L838-L850
    pub fn from_response(res: BufferedHttpResponse) -> GetDomainAssociationError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let error_type = match res.headers.get("x-amzn-errortype") {
                Some(raw_error_type) => raw_error_type
                    .split(':')
                    .next()
                    .unwrap_or_else(|| "Unknown"),
                _ => json
                    .get("code")
                    .or_else(|| json.get("Code"))
                    .and_then(|c| c.as_str())
                    .unwrap_or_else(|| "Unknown"),
            };

            // message can come in either "message" or "Message"
            // see boto BaseJSONParser impl for parsing message
            // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L595-L598
            let error_message = json
                .get("message")
                .or_else(|| json.get("Message"))
                .and_then(|m| m.as_str())
                .unwrap_or("");

            match error_type {
                "BadRequestException" => {
                    return GetDomainAssociationError::BadRequest(String::from(error_message));
                }
                "InternalFailureException" => {
                    return GetDomainAssociationError::InternalFailure(String::from(error_message));
                }
                "NotFoundException" => {
                    return GetDomainAssociationError::NotFound(String::from(error_message));
                }
                "UnauthorizedException" => {
                    return GetDomainAssociationError::Unauthorized(String::from(error_message));
                }
                "ValidationException" => {
                    return GetDomainAssociationError::Validation(error_message.to_string());
                }
                _ => {}
            }
        }
        return GetDomainAssociationError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for GetDomainAssociationError {
    fn from(err: serde_json::error::Error) -> GetDomainAssociationError {
        GetDomainAssociationError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for GetDomainAssociationError {
    fn from(err: CredentialsError) -> GetDomainAssociationError {
        GetDomainAssociationError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GetDomainAssociationError {
    fn from(err: HttpDispatchError) -> GetDomainAssociationError {
        GetDomainAssociationError::HttpDispatch(err)
    }
}
impl From<io::Error> for GetDomainAssociationError {
    fn from(err: io::Error) -> GetDomainAssociationError {
        GetDomainAssociationError::HttpDispatch(HttpDispatchError::from(err))
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
            GetDomainAssociationError::Validation(ref cause) => cause,
            GetDomainAssociationError::Credentials(ref err) => err.description(),
            GetDomainAssociationError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            GetDomainAssociationError::ParseError(ref cause) => cause,
            GetDomainAssociationError::Unknown(_) => "unknown error",
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
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl GetJobError {
    // see boto RestJSONParser impl for parsing errors
    // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L838-L850
    pub fn from_response(res: BufferedHttpResponse) -> GetJobError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let error_type = match res.headers.get("x-amzn-errortype") {
                Some(raw_error_type) => raw_error_type
                    .split(':')
                    .next()
                    .unwrap_or_else(|| "Unknown"),
                _ => json
                    .get("code")
                    .or_else(|| json.get("Code"))
                    .and_then(|c| c.as_str())
                    .unwrap_or_else(|| "Unknown"),
            };

            // message can come in either "message" or "Message"
            // see boto BaseJSONParser impl for parsing message
            // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L595-L598
            let error_message = json
                .get("message")
                .or_else(|| json.get("Message"))
                .and_then(|m| m.as_str())
                .unwrap_or("");

            match error_type {
                "BadRequestException" => {
                    return GetJobError::BadRequest(String::from(error_message));
                }
                "InternalFailureException" => {
                    return GetJobError::InternalFailure(String::from(error_message));
                }
                "LimitExceededException" => {
                    return GetJobError::LimitExceeded(String::from(error_message));
                }
                "NotFoundException" => return GetJobError::NotFound(String::from(error_message)),
                "UnauthorizedException" => {
                    return GetJobError::Unauthorized(String::from(error_message));
                }
                "ValidationException" => return GetJobError::Validation(error_message.to_string()),
                _ => {}
            }
        }
        return GetJobError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for GetJobError {
    fn from(err: serde_json::error::Error) -> GetJobError {
        GetJobError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for GetJobError {
    fn from(err: CredentialsError) -> GetJobError {
        GetJobError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GetJobError {
    fn from(err: HttpDispatchError) -> GetJobError {
        GetJobError::HttpDispatch(err)
    }
}
impl From<io::Error> for GetJobError {
    fn from(err: io::Error) -> GetJobError {
        GetJobError::HttpDispatch(HttpDispatchError::from(err))
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
            GetJobError::Validation(ref cause) => cause,
            GetJobError::Credentials(ref err) => err.description(),
            GetJobError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            GetJobError::ParseError(ref cause) => cause,
            GetJobError::Unknown(_) => "unknown error",
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
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl ListAppsError {
    // see boto RestJSONParser impl for parsing errors
    // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L838-L850
    pub fn from_response(res: BufferedHttpResponse) -> ListAppsError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let error_type = match res.headers.get("x-amzn-errortype") {
                Some(raw_error_type) => raw_error_type
                    .split(':')
                    .next()
                    .unwrap_or_else(|| "Unknown"),
                _ => json
                    .get("code")
                    .or_else(|| json.get("Code"))
                    .and_then(|c| c.as_str())
                    .unwrap_or_else(|| "Unknown"),
            };

            // message can come in either "message" or "Message"
            // see boto BaseJSONParser impl for parsing message
            // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L595-L598
            let error_message = json
                .get("message")
                .or_else(|| json.get("Message"))
                .and_then(|m| m.as_str())
                .unwrap_or("");

            match error_type {
                "BadRequestException" => {
                    return ListAppsError::BadRequest(String::from(error_message));
                }
                "InternalFailureException" => {
                    return ListAppsError::InternalFailure(String::from(error_message));
                }
                "UnauthorizedException" => {
                    return ListAppsError::Unauthorized(String::from(error_message));
                }
                "ValidationException" => {
                    return ListAppsError::Validation(error_message.to_string());
                }
                _ => {}
            }
        }
        return ListAppsError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for ListAppsError {
    fn from(err: serde_json::error::Error) -> ListAppsError {
        ListAppsError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for ListAppsError {
    fn from(err: CredentialsError) -> ListAppsError {
        ListAppsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ListAppsError {
    fn from(err: HttpDispatchError) -> ListAppsError {
        ListAppsError::HttpDispatch(err)
    }
}
impl From<io::Error> for ListAppsError {
    fn from(err: io::Error) -> ListAppsError {
        ListAppsError::HttpDispatch(HttpDispatchError::from(err))
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
            ListAppsError::Validation(ref cause) => cause,
            ListAppsError::Credentials(ref err) => err.description(),
            ListAppsError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            ListAppsError::ParseError(ref cause) => cause,
            ListAppsError::Unknown(_) => "unknown error",
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
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl ListBranchesError {
    // see boto RestJSONParser impl for parsing errors
    // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L838-L850
    pub fn from_response(res: BufferedHttpResponse) -> ListBranchesError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let error_type = match res.headers.get("x-amzn-errortype") {
                Some(raw_error_type) => raw_error_type
                    .split(':')
                    .next()
                    .unwrap_or_else(|| "Unknown"),
                _ => json
                    .get("code")
                    .or_else(|| json.get("Code"))
                    .and_then(|c| c.as_str())
                    .unwrap_or_else(|| "Unknown"),
            };

            // message can come in either "message" or "Message"
            // see boto BaseJSONParser impl for parsing message
            // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L595-L598
            let error_message = json
                .get("message")
                .or_else(|| json.get("Message"))
                .and_then(|m| m.as_str())
                .unwrap_or("");

            match error_type {
                "BadRequestException" => {
                    return ListBranchesError::BadRequest(String::from(error_message));
                }
                "InternalFailureException" => {
                    return ListBranchesError::InternalFailure(String::from(error_message));
                }
                "UnauthorizedException" => {
                    return ListBranchesError::Unauthorized(String::from(error_message));
                }
                "ValidationException" => {
                    return ListBranchesError::Validation(error_message.to_string());
                }
                _ => {}
            }
        }
        return ListBranchesError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for ListBranchesError {
    fn from(err: serde_json::error::Error) -> ListBranchesError {
        ListBranchesError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for ListBranchesError {
    fn from(err: CredentialsError) -> ListBranchesError {
        ListBranchesError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ListBranchesError {
    fn from(err: HttpDispatchError) -> ListBranchesError {
        ListBranchesError::HttpDispatch(err)
    }
}
impl From<io::Error> for ListBranchesError {
    fn from(err: io::Error) -> ListBranchesError {
        ListBranchesError::HttpDispatch(HttpDispatchError::from(err))
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
            ListBranchesError::Validation(ref cause) => cause,
            ListBranchesError::Credentials(ref err) => err.description(),
            ListBranchesError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            ListBranchesError::ParseError(ref cause) => cause,
            ListBranchesError::Unknown(_) => "unknown error",
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
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl ListDomainAssociationsError {
    // see boto RestJSONParser impl for parsing errors
    // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L838-L850
    pub fn from_response(res: BufferedHttpResponse) -> ListDomainAssociationsError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let error_type = match res.headers.get("x-amzn-errortype") {
                Some(raw_error_type) => raw_error_type
                    .split(':')
                    .next()
                    .unwrap_or_else(|| "Unknown"),
                _ => json
                    .get("code")
                    .or_else(|| json.get("Code"))
                    .and_then(|c| c.as_str())
                    .unwrap_or_else(|| "Unknown"),
            };

            // message can come in either "message" or "Message"
            // see boto BaseJSONParser impl for parsing message
            // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L595-L598
            let error_message = json
                .get("message")
                .or_else(|| json.get("Message"))
                .and_then(|m| m.as_str())
                .unwrap_or("");

            match error_type {
                "BadRequestException" => {
                    return ListDomainAssociationsError::BadRequest(String::from(error_message));
                }
                "InternalFailureException" => {
                    return ListDomainAssociationsError::InternalFailure(String::from(error_message));
                }
                "UnauthorizedException" => {
                    return ListDomainAssociationsError::Unauthorized(String::from(error_message));
                }
                "ValidationException" => {
                    return ListDomainAssociationsError::Validation(error_message.to_string());
                }
                _ => {}
            }
        }
        return ListDomainAssociationsError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for ListDomainAssociationsError {
    fn from(err: serde_json::error::Error) -> ListDomainAssociationsError {
        ListDomainAssociationsError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for ListDomainAssociationsError {
    fn from(err: CredentialsError) -> ListDomainAssociationsError {
        ListDomainAssociationsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ListDomainAssociationsError {
    fn from(err: HttpDispatchError) -> ListDomainAssociationsError {
        ListDomainAssociationsError::HttpDispatch(err)
    }
}
impl From<io::Error> for ListDomainAssociationsError {
    fn from(err: io::Error) -> ListDomainAssociationsError {
        ListDomainAssociationsError::HttpDispatch(HttpDispatchError::from(err))
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
            ListDomainAssociationsError::Validation(ref cause) => cause,
            ListDomainAssociationsError::Credentials(ref err) => err.description(),
            ListDomainAssociationsError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            ListDomainAssociationsError::ParseError(ref cause) => cause,
            ListDomainAssociationsError::Unknown(_) => "unknown error",
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
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl ListJobsError {
    // see boto RestJSONParser impl for parsing errors
    // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L838-L850
    pub fn from_response(res: BufferedHttpResponse) -> ListJobsError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let error_type = match res.headers.get("x-amzn-errortype") {
                Some(raw_error_type) => raw_error_type
                    .split(':')
                    .next()
                    .unwrap_or_else(|| "Unknown"),
                _ => json
                    .get("code")
                    .or_else(|| json.get("Code"))
                    .and_then(|c| c.as_str())
                    .unwrap_or_else(|| "Unknown"),
            };

            // message can come in either "message" or "Message"
            // see boto BaseJSONParser impl for parsing message
            // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L595-L598
            let error_message = json
                .get("message")
                .or_else(|| json.get("Message"))
                .and_then(|m| m.as_str())
                .unwrap_or("");

            match error_type {
                "BadRequestException" => {
                    return ListJobsError::BadRequest(String::from(error_message));
                }
                "InternalFailureException" => {
                    return ListJobsError::InternalFailure(String::from(error_message));
                }
                "LimitExceededException" => {
                    return ListJobsError::LimitExceeded(String::from(error_message));
                }
                "UnauthorizedException" => {
                    return ListJobsError::Unauthorized(String::from(error_message));
                }
                "ValidationException" => {
                    return ListJobsError::Validation(error_message.to_string());
                }
                _ => {}
            }
        }
        return ListJobsError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for ListJobsError {
    fn from(err: serde_json::error::Error) -> ListJobsError {
        ListJobsError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for ListJobsError {
    fn from(err: CredentialsError) -> ListJobsError {
        ListJobsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ListJobsError {
    fn from(err: HttpDispatchError) -> ListJobsError {
        ListJobsError::HttpDispatch(err)
    }
}
impl From<io::Error> for ListJobsError {
    fn from(err: io::Error) -> ListJobsError {
        ListJobsError::HttpDispatch(HttpDispatchError::from(err))
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
            ListJobsError::Validation(ref cause) => cause,
            ListJobsError::Credentials(ref err) => err.description(),
            ListJobsError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            ListJobsError::ParseError(ref cause) => cause,
            ListJobsError::Unknown(_) => "unknown error",
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
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl StartJobError {
    // see boto RestJSONParser impl for parsing errors
    // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L838-L850
    pub fn from_response(res: BufferedHttpResponse) -> StartJobError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let error_type = match res.headers.get("x-amzn-errortype") {
                Some(raw_error_type) => raw_error_type
                    .split(':')
                    .next()
                    .unwrap_or_else(|| "Unknown"),
                _ => json
                    .get("code")
                    .or_else(|| json.get("Code"))
                    .and_then(|c| c.as_str())
                    .unwrap_or_else(|| "Unknown"),
            };

            // message can come in either "message" or "Message"
            // see boto BaseJSONParser impl for parsing message
            // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L595-L598
            let error_message = json
                .get("message")
                .or_else(|| json.get("Message"))
                .and_then(|m| m.as_str())
                .unwrap_or("");

            match error_type {
                "BadRequestException" => {
                    return StartJobError::BadRequest(String::from(error_message));
                }
                "InternalFailureException" => {
                    return StartJobError::InternalFailure(String::from(error_message));
                }
                "LimitExceededException" => {
                    return StartJobError::LimitExceeded(String::from(error_message));
                }
                "NotFoundException" => return StartJobError::NotFound(String::from(error_message)),
                "UnauthorizedException" => {
                    return StartJobError::Unauthorized(String::from(error_message));
                }
                "ValidationException" => {
                    return StartJobError::Validation(error_message.to_string());
                }
                _ => {}
            }
        }
        return StartJobError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for StartJobError {
    fn from(err: serde_json::error::Error) -> StartJobError {
        StartJobError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for StartJobError {
    fn from(err: CredentialsError) -> StartJobError {
        StartJobError::Credentials(err)
    }
}
impl From<HttpDispatchError> for StartJobError {
    fn from(err: HttpDispatchError) -> StartJobError {
        StartJobError::HttpDispatch(err)
    }
}
impl From<io::Error> for StartJobError {
    fn from(err: io::Error) -> StartJobError {
        StartJobError::HttpDispatch(HttpDispatchError::from(err))
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
            StartJobError::Validation(ref cause) => cause,
            StartJobError::Credentials(ref err) => err.description(),
            StartJobError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            StartJobError::ParseError(ref cause) => cause,
            StartJobError::Unknown(_) => "unknown error",
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
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl StopJobError {
    // see boto RestJSONParser impl for parsing errors
    // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L838-L850
    pub fn from_response(res: BufferedHttpResponse) -> StopJobError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let error_type = match res.headers.get("x-amzn-errortype") {
                Some(raw_error_type) => raw_error_type
                    .split(':')
                    .next()
                    .unwrap_or_else(|| "Unknown"),
                _ => json
                    .get("code")
                    .or_else(|| json.get("Code"))
                    .and_then(|c| c.as_str())
                    .unwrap_or_else(|| "Unknown"),
            };

            // message can come in either "message" or "Message"
            // see boto BaseJSONParser impl for parsing message
            // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L595-L598
            let error_message = json
                .get("message")
                .or_else(|| json.get("Message"))
                .and_then(|m| m.as_str())
                .unwrap_or("");

            match error_type {
                "BadRequestException" => {
                    return StopJobError::BadRequest(String::from(error_message));
                }
                "InternalFailureException" => {
                    return StopJobError::InternalFailure(String::from(error_message));
                }
                "LimitExceededException" => {
                    return StopJobError::LimitExceeded(String::from(error_message));
                }
                "NotFoundException" => return StopJobError::NotFound(String::from(error_message)),
                "UnauthorizedException" => {
                    return StopJobError::Unauthorized(String::from(error_message));
                }
                "ValidationException" => return StopJobError::Validation(error_message.to_string()),
                _ => {}
            }
        }
        return StopJobError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for StopJobError {
    fn from(err: serde_json::error::Error) -> StopJobError {
        StopJobError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for StopJobError {
    fn from(err: CredentialsError) -> StopJobError {
        StopJobError::Credentials(err)
    }
}
impl From<HttpDispatchError> for StopJobError {
    fn from(err: HttpDispatchError) -> StopJobError {
        StopJobError::HttpDispatch(err)
    }
}
impl From<io::Error> for StopJobError {
    fn from(err: io::Error) -> StopJobError {
        StopJobError::HttpDispatch(HttpDispatchError::from(err))
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
            StopJobError::Validation(ref cause) => cause,
            StopJobError::Credentials(ref err) => err.description(),
            StopJobError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            StopJobError::ParseError(ref cause) => cause,
            StopJobError::Unknown(_) => "unknown error",
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
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl UpdateAppError {
    // see boto RestJSONParser impl for parsing errors
    // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L838-L850
    pub fn from_response(res: BufferedHttpResponse) -> UpdateAppError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let error_type = match res.headers.get("x-amzn-errortype") {
                Some(raw_error_type) => raw_error_type
                    .split(':')
                    .next()
                    .unwrap_or_else(|| "Unknown"),
                _ => json
                    .get("code")
                    .or_else(|| json.get("Code"))
                    .and_then(|c| c.as_str())
                    .unwrap_or_else(|| "Unknown"),
            };

            // message can come in either "message" or "Message"
            // see boto BaseJSONParser impl for parsing message
            // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L595-L598
            let error_message = json
                .get("message")
                .or_else(|| json.get("Message"))
                .and_then(|m| m.as_str())
                .unwrap_or("");

            match error_type {
                "BadRequestException" => {
                    return UpdateAppError::BadRequest(String::from(error_message));
                }
                "InternalFailureException" => {
                    return UpdateAppError::InternalFailure(String::from(error_message));
                }
                "NotFoundException" => return UpdateAppError::NotFound(String::from(error_message)),
                "UnauthorizedException" => {
                    return UpdateAppError::Unauthorized(String::from(error_message));
                }
                "ValidationException" => {
                    return UpdateAppError::Validation(error_message.to_string());
                }
                _ => {}
            }
        }
        return UpdateAppError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for UpdateAppError {
    fn from(err: serde_json::error::Error) -> UpdateAppError {
        UpdateAppError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for UpdateAppError {
    fn from(err: CredentialsError) -> UpdateAppError {
        UpdateAppError::Credentials(err)
    }
}
impl From<HttpDispatchError> for UpdateAppError {
    fn from(err: HttpDispatchError) -> UpdateAppError {
        UpdateAppError::HttpDispatch(err)
    }
}
impl From<io::Error> for UpdateAppError {
    fn from(err: io::Error) -> UpdateAppError {
        UpdateAppError::HttpDispatch(HttpDispatchError::from(err))
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
            UpdateAppError::Validation(ref cause) => cause,
            UpdateAppError::Credentials(ref err) => err.description(),
            UpdateAppError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            UpdateAppError::ParseError(ref cause) => cause,
            UpdateAppError::Unknown(_) => "unknown error",
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
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl UpdateBranchError {
    // see boto RestJSONParser impl for parsing errors
    // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L838-L850
    pub fn from_response(res: BufferedHttpResponse) -> UpdateBranchError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let error_type = match res.headers.get("x-amzn-errortype") {
                Some(raw_error_type) => raw_error_type
                    .split(':')
                    .next()
                    .unwrap_or_else(|| "Unknown"),
                _ => json
                    .get("code")
                    .or_else(|| json.get("Code"))
                    .and_then(|c| c.as_str())
                    .unwrap_or_else(|| "Unknown"),
            };

            // message can come in either "message" or "Message"
            // see boto BaseJSONParser impl for parsing message
            // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L595-L598
            let error_message = json
                .get("message")
                .or_else(|| json.get("Message"))
                .and_then(|m| m.as_str())
                .unwrap_or("");

            match error_type {
                "BadRequestException" => {
                    return UpdateBranchError::BadRequest(String::from(error_message));
                }
                "DependentServiceFailureException" => {
                    return UpdateBranchError::DependentServiceFailure(String::from(error_message));
                }
                "InternalFailureException" => {
                    return UpdateBranchError::InternalFailure(String::from(error_message));
                }
                "NotFoundException" => {
                    return UpdateBranchError::NotFound(String::from(error_message));
                }
                "UnauthorizedException" => {
                    return UpdateBranchError::Unauthorized(String::from(error_message));
                }
                "ValidationException" => {
                    return UpdateBranchError::Validation(error_message.to_string());
                }
                _ => {}
            }
        }
        return UpdateBranchError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for UpdateBranchError {
    fn from(err: serde_json::error::Error) -> UpdateBranchError {
        UpdateBranchError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for UpdateBranchError {
    fn from(err: CredentialsError) -> UpdateBranchError {
        UpdateBranchError::Credentials(err)
    }
}
impl From<HttpDispatchError> for UpdateBranchError {
    fn from(err: HttpDispatchError) -> UpdateBranchError {
        UpdateBranchError::HttpDispatch(err)
    }
}
impl From<io::Error> for UpdateBranchError {
    fn from(err: io::Error) -> UpdateBranchError {
        UpdateBranchError::HttpDispatch(HttpDispatchError::from(err))
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
            UpdateBranchError::Validation(ref cause) => cause,
            UpdateBranchError::Credentials(ref err) => err.description(),
            UpdateBranchError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            UpdateBranchError::ParseError(ref cause) => cause,
            UpdateBranchError::Unknown(_) => "unknown error",
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
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl UpdateDomainAssociationError {
    // see boto RestJSONParser impl for parsing errors
    // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L838-L850
    pub fn from_response(res: BufferedHttpResponse) -> UpdateDomainAssociationError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let error_type = match res.headers.get("x-amzn-errortype") {
                Some(raw_error_type) => raw_error_type
                    .split(':')
                    .next()
                    .unwrap_or_else(|| "Unknown"),
                _ => json
                    .get("code")
                    .or_else(|| json.get("Code"))
                    .and_then(|c| c.as_str())
                    .unwrap_or_else(|| "Unknown"),
            };

            // message can come in either "message" or "Message"
            // see boto BaseJSONParser impl for parsing message
            // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L595-L598
            let error_message = json
                .get("message")
                .or_else(|| json.get("Message"))
                .and_then(|m| m.as_str())
                .unwrap_or("");

            match error_type {
                "BadRequestException" => {
                    return UpdateDomainAssociationError::BadRequest(String::from(error_message));
                }
                "DependentServiceFailureException" => {
                    return UpdateDomainAssociationError::DependentServiceFailure(String::from(
                        error_message,
                    ));
                }
                "InternalFailureException" => {
                    return UpdateDomainAssociationError::InternalFailure(String::from(
                        error_message,
                    ));
                }
                "NotFoundException" => {
                    return UpdateDomainAssociationError::NotFound(String::from(error_message));
                }
                "UnauthorizedException" => {
                    return UpdateDomainAssociationError::Unauthorized(String::from(error_message));
                }
                "ValidationException" => {
                    return UpdateDomainAssociationError::Validation(error_message.to_string());
                }
                _ => {}
            }
        }
        return UpdateDomainAssociationError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for UpdateDomainAssociationError {
    fn from(err: serde_json::error::Error) -> UpdateDomainAssociationError {
        UpdateDomainAssociationError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for UpdateDomainAssociationError {
    fn from(err: CredentialsError) -> UpdateDomainAssociationError {
        UpdateDomainAssociationError::Credentials(err)
    }
}
impl From<HttpDispatchError> for UpdateDomainAssociationError {
    fn from(err: HttpDispatchError) -> UpdateDomainAssociationError {
        UpdateDomainAssociationError::HttpDispatch(err)
    }
}
impl From<io::Error> for UpdateDomainAssociationError {
    fn from(err: io::Error) -> UpdateDomainAssociationError {
        UpdateDomainAssociationError::HttpDispatch(HttpDispatchError::from(err))
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
            UpdateDomainAssociationError::Validation(ref cause) => cause,
            UpdateDomainAssociationError::Credentials(ref err) => err.description(),
            UpdateDomainAssociationError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            UpdateDomainAssociationError::ParseError(ref cause) => cause,
            UpdateDomainAssociationError::Unknown(_) => "unknown error",
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

    /// <p> Create a new DomainAssociation on an App </p>
    fn create_domain_association(
        &self,
        input: CreateDomainAssociationRequest,
    ) -> RusotoFuture<CreateDomainAssociationResult, CreateDomainAssociationError>;

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

    /// <p> Starts a new job for a branch, part of an Amplify App. </p>
    fn start_job(&self, input: StartJobRequest) -> RusotoFuture<StartJobResult, StartJobError>;

    /// <p> Stop a job that is in progress, for an Amplify branch, part of Amplify App. </p>
    fn stop_job(&self, input: StopJobRequest) -> RusotoFuture<StopJobResult, StopJobError>;

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
            region: region,
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
            region: region,
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
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" || body.is_empty() {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result = serde_json::from_slice::<CreateAppResult>(&body).unwrap();

                    result
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
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" || body.is_empty() {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result = serde_json::from_slice::<CreateBranchResult>(&body).unwrap();

                    result
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
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" || body.is_empty() {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result =
                        serde_json::from_slice::<CreateDomainAssociationResult>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(CreateDomainAssociationError::from_response(response))
                }))
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
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" || body.is_empty() {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result = serde_json::from_slice::<DeleteAppResult>(&body).unwrap();

                    result
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
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" || body.is_empty() {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result = serde_json::from_slice::<DeleteBranchResult>(&body).unwrap();

                    result
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
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" || body.is_empty() {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result =
                        serde_json::from_slice::<DeleteDomainAssociationResult>(&body).unwrap();

                    result
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
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" || body.is_empty() {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result = serde_json::from_slice::<DeleteJobResult>(&body).unwrap();

                    result
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

    /// <p> Retrieves an existing Amplify App by appId. </p>
    fn get_app(&self, input: GetAppRequest) -> RusotoFuture<GetAppResult, GetAppError> {
        let request_uri = format!("/apps/{app_id}", app_id = input.app_id);

        let mut request = SignedRequest::new("GET", "amplify", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" || body.is_empty() {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result = serde_json::from_slice::<GetAppResult>(&body).unwrap();

                    result
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
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" || body.is_empty() {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result = serde_json::from_slice::<GetBranchResult>(&body).unwrap();

                    result
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
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" || body.is_empty() {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result =
                        serde_json::from_slice::<GetDomainAssociationResult>(&body).unwrap();

                    result
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
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" || body.is_empty() {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result = serde_json::from_slice::<GetJobResult>(&body).unwrap();

                    result
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
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" || body.is_empty() {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result = serde_json::from_slice::<ListAppsResult>(&body).unwrap();

                    result
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
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" || body.is_empty() {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result = serde_json::from_slice::<ListBranchesResult>(&body).unwrap();

                    result
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
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" || body.is_empty() {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result =
                        serde_json::from_slice::<ListDomainAssociationsResult>(&body).unwrap();

                    result
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
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" || body.is_empty() {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result = serde_json::from_slice::<ListJobsResult>(&body).unwrap();

                    result
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
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" || body.is_empty() {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result = serde_json::from_slice::<StartJobResult>(&body).unwrap();

                    result
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
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" || body.is_empty() {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result = serde_json::from_slice::<StopJobResult>(&body).unwrap();

                    result
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

    /// <p> Updates an existing Amplify App. </p>
    fn update_app(&self, input: UpdateAppRequest) -> RusotoFuture<UpdateAppResult, UpdateAppError> {
        let request_uri = format!("/apps/{app_id}", app_id = input.app_id);

        let mut request = SignedRequest::new("POST", "amplify", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" || body.is_empty() {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result = serde_json::from_slice::<UpdateAppResult>(&body).unwrap();

                    result
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
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" || body.is_empty() {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result = serde_json::from_slice::<UpdateBranchResult>(&body).unwrap();

                    result
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
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" || body.is_empty() {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result =
                        serde_json::from_slice::<UpdateDomainAssociationResult>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(UpdateDomainAssociationError::from_response(response))
                }))
            }
        })
    }
}

#[cfg(test)]
mod protocol_tests {}
