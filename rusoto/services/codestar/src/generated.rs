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

use rusoto_core::proto;
use rusoto_core::signature::SignedRequest;
#[allow(unused_imports)]
use serde::{Deserialize, Serialize};
use serde_json;
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct AssociateTeamMemberRequest {
    /// <p>A user- or system-generated token that identifies the entity that requested the team member association to the project. This token can be used to repeat the request.</p>
    #[serde(rename = "clientRequestToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_request_token: Option<String>,
    /// <p>The ID of the project to which you will add the IAM user.</p>
    #[serde(rename = "projectId")]
    pub project_id: String,
    /// <p>The AWS CodeStar project role that will apply to this user. This role determines what actions a user can take in an AWS CodeStar project.</p>
    #[serde(rename = "projectRole")]
    pub project_role: String,
    /// <p>Whether the team member is allowed to use an SSH public/private key pair to remotely access project resources, for example Amazon EC2 instances.</p>
    #[serde(rename = "remoteAccessAllowed")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remote_access_allowed: Option<bool>,
    /// <p>The Amazon Resource Name (ARN) for the IAM user you want to add to the AWS CodeStar project.</p>
    #[serde(rename = "userArn")]
    pub user_arn: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct AssociateTeamMemberResult {
    /// <p>The user- or system-generated token from the initial request that can be used to repeat the request.</p>
    #[serde(rename = "clientRequestToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_request_token: Option<String>,
}

/// <p>Location and destination information about the source code files provided with the project request. The source code is uploaded to the new project source repository after project creation.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct Code {
    /// <p>The repository to be created in AWS CodeStar. Valid values are AWS CodeCommit or GitHub. After AWS CodeStar provisions the new repository, the source code files provided with the project request are placed in the repository.</p>
    #[serde(rename = "destination")]
    pub destination: CodeDestination,
    /// <p>The location where the source code files provided with the project request are stored. AWS CodeStar retrieves the files during project creation.</p>
    #[serde(rename = "source")]
    pub source: CodeSource,
}

/// <p>Information about the AWS CodeCommit repository to be created in AWS CodeStar. This is where the source code files provided with the project request will be uploaded after project creation.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CodeCommitCodeDestination {
    /// <p>The name of the AWS CodeCommit repository to be created in AWS CodeStar.</p>
    #[serde(rename = "name")]
    pub name: String,
}

/// <p>The repository to be created in AWS CodeStar. Valid values are AWS CodeCommit or GitHub. After AWS CodeStar provisions the new repository, the source code files provided with the project request are placed in the repository.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CodeDestination {
    /// <p>Information about the AWS CodeCommit repository to be created in AWS CodeStar. This is where the source code files provided with the project request will be uploaded after project creation.</p>
    #[serde(rename = "codeCommit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code_commit: Option<CodeCommitCodeDestination>,
    /// <p>Information about the GitHub repository to be created in AWS CodeStar. This is where the source code files provided with the project request will be uploaded after project creation.</p>
    #[serde(rename = "gitHub")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub git_hub: Option<GitHubCodeDestination>,
}

/// <p>The location where the source code files provided with the project request are stored. AWS CodeStar retrieves the files during project creation.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CodeSource {
    /// <p>Information about the Amazon S3 location where the source code files provided with the project request are stored. </p>
    #[serde(rename = "s3")]
    pub s_3: S3Location,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateProjectRequest {
    /// <p>A user- or system-generated token that identifies the entity that requested project creation. This token can be used to repeat the request.</p>
    #[serde(rename = "clientRequestToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_request_token: Option<String>,
    /// <p>The description of the project, if any.</p>
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The ID of the project to be created in AWS CodeStar.</p>
    #[serde(rename = "id")]
    pub id: String,
    /// <p>The display name for the project to be created in AWS CodeStar.</p>
    #[serde(rename = "name")]
    pub name: String,
    /// <p>A list of the Code objects submitted with the project request. If this parameter is specified, the request must also include the toolchain parameter.</p>
    #[serde(rename = "sourceCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_code: Option<Vec<Code>>,
    /// <p>The tags created for the project.</p>
    #[serde(rename = "tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<::std::collections::HashMap<String, String>>,
    /// <p>The name of the toolchain template file submitted with the project request. If this parameter is specified, the request must also include the sourceCode parameter.</p>
    #[serde(rename = "toolchain")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub toolchain: Option<Toolchain>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateProjectResult {
    /// <p>The Amazon Resource Name (ARN) of the created project.</p>
    #[serde(rename = "arn")]
    pub arn: String,
    /// <p>A user- or system-generated token that identifies the entity that requested project creation.</p>
    #[serde(rename = "clientRequestToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_request_token: Option<String>,
    /// <p>The ID of the project.</p>
    #[serde(rename = "id")]
    pub id: String,
    /// <p>Reserved for future use.</p>
    #[serde(rename = "projectTemplateId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub project_template_id: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateUserProfileRequest {
    /// <p>The name that will be displayed as the friendly name for the user in AWS CodeStar. </p>
    #[serde(rename = "displayName")]
    pub display_name: String,
    /// <p>The email address that will be displayed as part of the user's profile in AWS CodeStar.</p>
    #[serde(rename = "emailAddress")]
    pub email_address: String,
    /// <p>The SSH public key associated with the user in AWS CodeStar. If a project owner allows the user remote access to project resources, this public key will be used along with the user's private key for SSH access.</p>
    #[serde(rename = "sshPublicKey")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ssh_public_key: Option<String>,
    /// <p>The Amazon Resource Name (ARN) of the user in IAM.</p>
    #[serde(rename = "userArn")]
    pub user_arn: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateUserProfileResult {
    /// <p>The date the user profile was created, in timestamp format.</p>
    #[serde(rename = "createdTimestamp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_timestamp: Option<f64>,
    /// <p>The name that is displayed as the friendly name for the user in AWS CodeStar.</p>
    #[serde(rename = "displayName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    /// <p>The email address that is displayed as part of the user's profile in AWS CodeStar.</p>
    #[serde(rename = "emailAddress")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email_address: Option<String>,
    /// <p>The date the user profile was last modified, in timestamp format.</p>
    #[serde(rename = "lastModifiedTimestamp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_timestamp: Option<f64>,
    /// <p>The SSH public key associated with the user in AWS CodeStar. This is the public portion of the public/private keypair the user can use to access project resources if a project owner allows the user remote access to those resources.</p>
    #[serde(rename = "sshPublicKey")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ssh_public_key: Option<String>,
    /// <p>The Amazon Resource Name (ARN) of the user in IAM.</p>
    #[serde(rename = "userArn")]
    pub user_arn: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteProjectRequest {
    /// <p>A user- or system-generated token that identifies the entity that requested project deletion. This token can be used to repeat the request. </p>
    #[serde(rename = "clientRequestToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_request_token: Option<String>,
    /// <p>Whether to send a delete request for the primary stack in AWS CloudFormation originally used to generate the project and its resources. This option will delete all AWS resources for the project (except for any buckets in Amazon S3) as well as deleting the project itself. Recommended for most use cases.</p>
    #[serde(rename = "deleteStack")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delete_stack: Option<bool>,
    /// <p>The ID of the project to be deleted in AWS CodeStar.</p>
    #[serde(rename = "id")]
    pub id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DeleteProjectResult {
    /// <p>The Amazon Resource Name (ARN) of the deleted project.</p>
    #[serde(rename = "projectArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub project_arn: Option<String>,
    /// <p>The ID of the primary stack in AWS CloudFormation that will be deleted as part of deleting the project and its resources.</p>
    #[serde(rename = "stackId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stack_id: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteUserProfileRequest {
    /// <p>The Amazon Resource Name (ARN) of the user to delete from AWS CodeStar.</p>
    #[serde(rename = "userArn")]
    pub user_arn: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DeleteUserProfileResult {
    /// <p>The Amazon Resource Name (ARN) of the user deleted from AWS CodeStar.</p>
    #[serde(rename = "userArn")]
    pub user_arn: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeProjectRequest {
    /// <p>The ID of the project.</p>
    #[serde(rename = "id")]
    pub id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeProjectResult {
    /// <p>The Amazon Resource Name (ARN) for the project.</p>
    #[serde(rename = "arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p>A user- or system-generated token that identifies the entity that requested project creation. </p>
    #[serde(rename = "clientRequestToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_request_token: Option<String>,
    /// <p>The date and time the project was created, in timestamp format.</p>
    #[serde(rename = "createdTimeStamp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_time_stamp: Option<f64>,
    /// <p>The description of the project, if any.</p>
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The ID of the project.</p>
    #[serde(rename = "id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// <p>The display name for the project.</p>
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The ID for the AWS CodeStar project template used to create the project.</p>
    #[serde(rename = "projectTemplateId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub project_template_id: Option<String>,
    /// <p>The ID of the primary stack in AWS CloudFormation used to generate resources for the project.</p>
    #[serde(rename = "stackId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stack_id: Option<String>,
    /// <p>The project creation or deletion status.</p>
    #[serde(rename = "status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<ProjectStatus>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeUserProfileRequest {
    /// <p>The Amazon Resource Name (ARN) of the user.</p>
    #[serde(rename = "userArn")]
    pub user_arn: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeUserProfileResult {
    /// <p>The date and time when the user profile was created in AWS CodeStar, in timestamp format.</p>
    #[serde(rename = "createdTimestamp")]
    pub created_timestamp: f64,
    /// <p>The display name shown for the user in AWS CodeStar projects. For example, this could be set to both first and last name ("Mary Major") or a single name ("Mary"). The display name is also used to generate the initial icon associated with the user in AWS CodeStar projects. If spaces are included in the display name, the first character that appears after the space will be used as the second character in the user initial icon. The initial icon displays a maximum of two characters, so a display name with more than one space (for example "Mary Jane Major") would generate an initial icon using the first character and the first character after the space ("MJ", not "MM").</p>
    #[serde(rename = "displayName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    /// <p>The email address for the user. Optional.</p>
    #[serde(rename = "emailAddress")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email_address: Option<String>,
    /// <p>The date and time when the user profile was last modified, in timestamp format.</p>
    #[serde(rename = "lastModifiedTimestamp")]
    pub last_modified_timestamp: f64,
    /// <p>The SSH public key associated with the user. This SSH public key is associated with the user profile, and can be used in conjunction with the associated private key for access to project resources, such as Amazon EC2 instances, if a project owner grants remote access to those resources.</p>
    #[serde(rename = "sshPublicKey")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ssh_public_key: Option<String>,
    /// <p>The Amazon Resource Name (ARN) of the user.</p>
    #[serde(rename = "userArn")]
    pub user_arn: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DisassociateTeamMemberRequest {
    /// <p>The ID of the AWS CodeStar project from which you want to remove a team member.</p>
    #[serde(rename = "projectId")]
    pub project_id: String,
    /// <p>The Amazon Resource Name (ARN) of the IAM user or group whom you want to remove from the project.</p>
    #[serde(rename = "userArn")]
    pub user_arn: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DisassociateTeamMemberResult {}

/// <p>Information about the GitHub repository to be created in AWS CodeStar. This is where the source code files provided with the project request will be uploaded after project creation.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GitHubCodeDestination {
    /// <p>Description for the GitHub repository to be created in AWS CodeStar. This description displays in GitHub after the repository is created.</p>
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>Whether to enable issues for the GitHub repository.</p>
    #[serde(rename = "issuesEnabled")]
    pub issues_enabled: bool,
    /// <p>Name of the GitHub repository to be created in AWS CodeStar.</p>
    #[serde(rename = "name")]
    pub name: String,
    /// <p>The GitHub username for the owner of the GitHub repository to be created in AWS CodeStar. If this repository should be owned by a GitHub organization, provide its name.</p>
    #[serde(rename = "owner")]
    pub owner: String,
    /// <p>Whether the GitHub repository is to be a private repository.</p>
    #[serde(rename = "privateRepository")]
    pub private_repository: bool,
    /// <p>The GitHub user's personal access token for the GitHub repository.</p>
    #[serde(rename = "token")]
    pub token: String,
    /// <p>The type of GitHub repository to be created in AWS CodeStar. Valid values are User or Organization.</p>
    #[serde(rename = "type")]
    pub type_: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListProjectsRequest {
    /// <p>The maximum amount of data that can be contained in a single set of results.</p>
    #[serde(rename = "maxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>The continuation token to be used to return the next set of results, if the results cannot be returned in one response.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListProjectsResult {
    /// <p>The continuation token to use when requesting the next set of results, if there are more results to be returned.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>A list of projects.</p>
    #[serde(rename = "projects")]
    pub projects: Vec<ProjectSummary>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListResourcesRequest {
    /// <p>The maximum amount of data that can be contained in a single set of results.</p>
    #[serde(rename = "maxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>The continuation token for the next set of results, if the results cannot be returned in one response.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The ID of the project.</p>
    #[serde(rename = "projectId")]
    pub project_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListResourcesResult {
    /// <p>The continuation token to use when requesting the next set of results, if there are more results to be returned.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>An array of resources associated with the project. </p>
    #[serde(rename = "resources")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resources: Option<Vec<Resource>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListTagsForProjectRequest {
    /// <p>The ID of the project to get tags for.</p>
    #[serde(rename = "id")]
    pub id: String,
    /// <p>Reserved for future use.</p>
    #[serde(rename = "maxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>Reserved for future use.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListTagsForProjectResult {
    /// <p>Reserved for future use.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The tags for the project.</p>
    #[serde(rename = "tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<::std::collections::HashMap<String, String>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListTeamMembersRequest {
    /// <p>The maximum number of team members you want returned in a response.</p>
    #[serde(rename = "maxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>The continuation token for the next set of results, if the results cannot be returned in one response.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The ID of the project for which you want to list team members.</p>
    #[serde(rename = "projectId")]
    pub project_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListTeamMembersResult {
    /// <p>The continuation token to use when requesting the next set of results, if there are more results to be returned.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>A list of team member objects for the project.</p>
    #[serde(rename = "teamMembers")]
    pub team_members: Vec<TeamMember>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListUserProfilesRequest {
    /// <p>The maximum number of results to return in a response.</p>
    #[serde(rename = "maxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>The continuation token for the next set of results, if the results cannot be returned in one response.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListUserProfilesResult {
    /// <p>The continuation token to use when requesting the next set of results, if there are more results to be returned.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>All the user profiles configured in AWS CodeStar for an AWS account.</p>
    #[serde(rename = "userProfiles")]
    pub user_profiles: Vec<UserProfileSummary>,
}

/// <p>An indication of whether a project creation or deletion is failed or successful.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ProjectStatus {
    /// <p>In the case of a project creation or deletion failure, a reason for the failure.</p>
    #[serde(rename = "reason")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    /// <p>The phase of completion for a project creation or deletion.</p>
    #[serde(rename = "state")]
    pub state: String,
}

/// <p>Information about the metadata for a project.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ProjectSummary {
    /// <p>The Amazon Resource Name (ARN) of the project.</p>
    #[serde(rename = "projectArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub project_arn: Option<String>,
    /// <p>The ID of the project.</p>
    #[serde(rename = "projectId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub project_id: Option<String>,
}

/// <p>Information about a resource for a project.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct Resource {
    /// <p>The Amazon Resource Name (ARN) of the resource.</p>
    #[serde(rename = "id")]
    pub id: String,
}

/// <p>The Amazon S3 location where the source code files provided with the project request are stored.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct S3Location {
    /// <p>The Amazon S3 object key where the source code files provided with the project request are stored.</p>
    #[serde(rename = "bucketKey")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bucket_key: Option<String>,
    /// <p>The Amazon S3 bucket name where the source code files provided with the project request are stored.</p>
    #[serde(rename = "bucketName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bucket_name: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct TagProjectRequest {
    /// <p>The ID of the project you want to add a tag to.</p>
    #[serde(rename = "id")]
    pub id: String,
    /// <p>The tags you want to add to the project.</p>
    #[serde(rename = "tags")]
    pub tags: ::std::collections::HashMap<String, String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct TagProjectResult {
    /// <p>The tags for the project.</p>
    #[serde(rename = "tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<::std::collections::HashMap<String, String>>,
}

/// <p>Information about a team member in a project.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct TeamMember {
    /// <p>The role assigned to the user in the project. Project roles have different levels of access. For more information, see <a href="http://docs.aws.amazon.com/codestar/latest/userguide/working-with-teams.html">Working with Teams</a> in the <i>AWS CodeStar User Guide</i>. </p>
    #[serde(rename = "projectRole")]
    pub project_role: String,
    /// <p>Whether the user is allowed to remotely access project resources using an SSH public/private key pair.</p>
    #[serde(rename = "remoteAccessAllowed")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remote_access_allowed: Option<bool>,
    /// <p>The Amazon Resource Name (ARN) of the user in IAM.</p>
    #[serde(rename = "userArn")]
    pub user_arn: String,
}

/// <p>The toolchain template file provided with the project request. AWS CodeStar uses the template to provision the toolchain stack in AWS CloudFormation.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct Toolchain {
    /// <p>The service role ARN for AWS CodeStar to use for the toolchain template during stack provisioning.</p>
    #[serde(rename = "roleArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_arn: Option<String>,
    /// <p>The Amazon S3 location where the toolchain template file provided with the project request is stored. AWS CodeStar retrieves the file during project creation.</p>
    #[serde(rename = "source")]
    pub source: ToolchainSource,
    /// <p>The list of parameter overrides to be passed into the toolchain template during stack provisioning, if any.</p>
    #[serde(rename = "stackParameters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stack_parameters: Option<::std::collections::HashMap<String, String>>,
}

/// <p>The Amazon S3 location where the toolchain template file provided with the project request is stored. AWS CodeStar retrieves the file during project creation.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ToolchainSource {
    /// <p>The Amazon S3 bucket where the toolchain template file provided with the project request is stored.</p>
    #[serde(rename = "s3")]
    pub s_3: S3Location,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UntagProjectRequest {
    /// <p>The ID of the project to remove tags from.</p>
    #[serde(rename = "id")]
    pub id: String,
    /// <p>The tags to remove from the project.</p>
    #[serde(rename = "tags")]
    pub tags: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UntagProjectResult {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UpdateProjectRequest {
    /// <p>The description of the project, if any.</p>
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The ID of the project you want to update.</p>
    #[serde(rename = "id")]
    pub id: String,
    /// <p>The name of the project you want to update.</p>
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UpdateProjectResult {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UpdateTeamMemberRequest {
    /// <p>The ID of the project.</p>
    #[serde(rename = "projectId")]
    pub project_id: String,
    /// <p>The role assigned to the user in the project. Project roles have different levels of access. For more information, see <a href="http://docs.aws.amazon.com/codestar/latest/userguide/working-with-teams.html">Working with Teams</a> in the <i>AWS CodeStar User Guide</i>.</p>
    #[serde(rename = "projectRole")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub project_role: Option<String>,
    /// <p>Whether a team member is allowed to remotely access project resources using the SSH public key associated with the user's profile. Even if this is set to True, the user must associate a public key with their profile before the user can access resources.</p>
    #[serde(rename = "remoteAccessAllowed")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remote_access_allowed: Option<bool>,
    /// <p>The Amazon Resource Name (ARN) of the user for whom you want to change team membership attributes.</p>
    #[serde(rename = "userArn")]
    pub user_arn: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UpdateTeamMemberResult {
    /// <p>The project role granted to the user.</p>
    #[serde(rename = "projectRole")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub project_role: Option<String>,
    /// <p>Whether a team member is allowed to remotely access project resources using the SSH public key associated with the user's profile.</p>
    #[serde(rename = "remoteAccessAllowed")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remote_access_allowed: Option<bool>,
    /// <p>The Amazon Resource Name (ARN) of the user whose team membership attributes were updated.</p>
    #[serde(rename = "userArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_arn: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UpdateUserProfileRequest {
    /// <p>The name that is displayed as the friendly name for the user in AWS CodeStar.</p>
    #[serde(rename = "displayName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    /// <p>The email address that is displayed as part of the user's profile in AWS CodeStar.</p>
    #[serde(rename = "emailAddress")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email_address: Option<String>,
    /// <p>The SSH public key associated with the user in AWS CodeStar. If a project owner allows the user remote access to project resources, this public key will be used along with the user's private key for SSH access.</p>
    #[serde(rename = "sshPublicKey")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ssh_public_key: Option<String>,
    /// <p>The name that will be displayed as the friendly name for the user in AWS CodeStar.</p>
    #[serde(rename = "userArn")]
    pub user_arn: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UpdateUserProfileResult {
    /// <p>The date the user profile was created, in timestamp format.</p>
    #[serde(rename = "createdTimestamp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_timestamp: Option<f64>,
    /// <p>The name that is displayed as the friendly name for the user in AWS CodeStar.</p>
    #[serde(rename = "displayName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    /// <p>The email address that is displayed as part of the user's profile in AWS CodeStar.</p>
    #[serde(rename = "emailAddress")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email_address: Option<String>,
    /// <p>The date the user profile was last modified, in timestamp format.</p>
    #[serde(rename = "lastModifiedTimestamp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_timestamp: Option<f64>,
    /// <p>The SSH public key associated with the user in AWS CodeStar. This is the public portion of the public/private keypair the user can use to access project resources if a project owner allows the user remote access to those resources.</p>
    #[serde(rename = "sshPublicKey")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ssh_public_key: Option<String>,
    /// <p>The Amazon Resource Name (ARN) of the user in IAM.</p>
    #[serde(rename = "userArn")]
    pub user_arn: String,
}

/// <p>Information about a user's profile in AWS CodeStar.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UserProfileSummary {
    /// <p>The display name of a user in AWS CodeStar. For example, this could be set to both first and last name ("Mary Major") or a single name ("Mary"). The display name is also used to generate the initial icon associated with the user in AWS CodeStar projects. If spaces are included in the display name, the first character that appears after the space will be used as the second character in the user initial icon. The initial icon displays a maximum of two characters, so a display name with more than one space (for example "Mary Jane Major") would generate an initial icon using the first character and the first character after the space ("MJ", not "MM").</p>
    #[serde(rename = "displayName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    /// <p>The email address associated with the user.</p>
    #[serde(rename = "emailAddress")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email_address: Option<String>,
    /// <p>The SSH public key associated with the user in AWS CodeStar. If a project owner allows the user remote access to project resources, this public key will be used along with the user's private key for SSH access.</p>
    #[serde(rename = "sshPublicKey")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ssh_public_key: Option<String>,
    /// <p>The Amazon Resource Name (ARN) of the user in IAM.</p>
    #[serde(rename = "userArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_arn: Option<String>,
}

/// Errors returned by AssociateTeamMember
#[derive(Debug, PartialEq)]
pub enum AssociateTeamMemberError {
    /// <p>Another modification is being made. That modification must complete before you can make your change.</p>
    ConcurrentModification(String),
    /// <p>The service role is not valid.</p>
    InvalidServiceRole(String),
    /// <p>A resource limit has been exceeded.</p>
    LimitExceeded(String),
    /// <p>Project configuration information is required but not specified.</p>
    ProjectConfiguration(String),
    /// <p>The specified AWS CodeStar project was not found.</p>
    ProjectNotFound(String),
    /// <p>The team member is already associated with a role in this project.</p>
    TeamMemberAlreadyAssociated(String),
}

impl AssociateTeamMemberError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<AssociateTeamMemberError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ConcurrentModificationException" => {
                    return RusotoError::Service(AssociateTeamMemberError::ConcurrentModification(
                        err.msg,
                    ))
                }
                "InvalidServiceRoleException" => {
                    return RusotoError::Service(AssociateTeamMemberError::InvalidServiceRole(
                        err.msg,
                    ))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(AssociateTeamMemberError::LimitExceeded(err.msg))
                }
                "ProjectConfigurationException" => {
                    return RusotoError::Service(AssociateTeamMemberError::ProjectConfiguration(
                        err.msg,
                    ))
                }
                "ProjectNotFoundException" => {
                    return RusotoError::Service(AssociateTeamMemberError::ProjectNotFound(err.msg))
                }
                "TeamMemberAlreadyAssociatedException" => {
                    return RusotoError::Service(
                        AssociateTeamMemberError::TeamMemberAlreadyAssociated(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for AssociateTeamMemberError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            AssociateTeamMemberError::ConcurrentModification(ref cause) => write!(f, "{}", cause),
            AssociateTeamMemberError::InvalidServiceRole(ref cause) => write!(f, "{}", cause),
            AssociateTeamMemberError::LimitExceeded(ref cause) => write!(f, "{}", cause),
            AssociateTeamMemberError::ProjectConfiguration(ref cause) => write!(f, "{}", cause),
            AssociateTeamMemberError::ProjectNotFound(ref cause) => write!(f, "{}", cause),
            AssociateTeamMemberError::TeamMemberAlreadyAssociated(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for AssociateTeamMemberError {}
/// Errors returned by CreateProject
#[derive(Debug, PartialEq)]
pub enum CreateProjectError {
    /// <p>Another modification is being made. That modification must complete before you can make your change.</p>
    ConcurrentModification(String),
    /// <p>The service role is not valid.</p>
    InvalidServiceRole(String),
    /// <p>A resource limit has been exceeded.</p>
    LimitExceeded(String),
    /// <p>An AWS CodeStar project with the same ID already exists in this region for the AWS account. AWS CodeStar project IDs must be unique within a region for the AWS account.</p>
    ProjectAlreadyExists(String),
    /// <p>Project configuration information is required but not specified.</p>
    ProjectConfiguration(String),
    /// <p>The project creation request was valid, but a nonspecific exception or error occurred during project creation. The project could not be created in AWS CodeStar.</p>
    ProjectCreationFailed(String),
}

impl CreateProjectError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateProjectError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ConcurrentModificationException" => {
                    return RusotoError::Service(CreateProjectError::ConcurrentModification(
                        err.msg,
                    ))
                }
                "InvalidServiceRoleException" => {
                    return RusotoError::Service(CreateProjectError::InvalidServiceRole(err.msg))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(CreateProjectError::LimitExceeded(err.msg))
                }
                "ProjectAlreadyExistsException" => {
                    return RusotoError::Service(CreateProjectError::ProjectAlreadyExists(err.msg))
                }
                "ProjectConfigurationException" => {
                    return RusotoError::Service(CreateProjectError::ProjectConfiguration(err.msg))
                }
                "ProjectCreationFailedException" => {
                    return RusotoError::Service(CreateProjectError::ProjectCreationFailed(err.msg))
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
            CreateProjectError::ConcurrentModification(ref cause) => write!(f, "{}", cause),
            CreateProjectError::InvalidServiceRole(ref cause) => write!(f, "{}", cause),
            CreateProjectError::LimitExceeded(ref cause) => write!(f, "{}", cause),
            CreateProjectError::ProjectAlreadyExists(ref cause) => write!(f, "{}", cause),
            CreateProjectError::ProjectConfiguration(ref cause) => write!(f, "{}", cause),
            CreateProjectError::ProjectCreationFailed(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CreateProjectError {}
/// Errors returned by CreateUserProfile
#[derive(Debug, PartialEq)]
pub enum CreateUserProfileError {
    /// <p>A user profile with that name already exists in this region for the AWS account. AWS CodeStar user profile names must be unique within a region for the AWS account. </p>
    UserProfileAlreadyExists(String),
}

impl CreateUserProfileError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateUserProfileError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "UserProfileAlreadyExistsException" => {
                    return RusotoError::Service(CreateUserProfileError::UserProfileAlreadyExists(
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
impl fmt::Display for CreateUserProfileError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreateUserProfileError::UserProfileAlreadyExists(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CreateUserProfileError {}
/// Errors returned by DeleteProject
#[derive(Debug, PartialEq)]
pub enum DeleteProjectError {
    /// <p>Another modification is being made. That modification must complete before you can make your change.</p>
    ConcurrentModification(String),
    /// <p>The service role is not valid.</p>
    InvalidServiceRole(String),
}

impl DeleteProjectError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteProjectError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ConcurrentModificationException" => {
                    return RusotoError::Service(DeleteProjectError::ConcurrentModification(
                        err.msg,
                    ))
                }
                "InvalidServiceRoleException" => {
                    return RusotoError::Service(DeleteProjectError::InvalidServiceRole(err.msg))
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
            DeleteProjectError::ConcurrentModification(ref cause) => write!(f, "{}", cause),
            DeleteProjectError::InvalidServiceRole(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeleteProjectError {}
/// Errors returned by DeleteUserProfile
#[derive(Debug, PartialEq)]
pub enum DeleteUserProfileError {}

impl DeleteUserProfileError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteUserProfileError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DeleteUserProfileError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {}
    }
}
impl Error for DeleteUserProfileError {}
/// Errors returned by DescribeProject
#[derive(Debug, PartialEq)]
pub enum DescribeProjectError {
    /// <p>Another modification is being made. That modification must complete before you can make your change.</p>
    ConcurrentModification(String),
    /// <p>The service role is not valid.</p>
    InvalidServiceRole(String),
    /// <p>Project configuration information is required but not specified.</p>
    ProjectConfiguration(String),
    /// <p>The specified AWS CodeStar project was not found.</p>
    ProjectNotFound(String),
}

impl DescribeProjectError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeProjectError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ConcurrentModificationException" => {
                    return RusotoError::Service(DescribeProjectError::ConcurrentModification(
                        err.msg,
                    ))
                }
                "InvalidServiceRoleException" => {
                    return RusotoError::Service(DescribeProjectError::InvalidServiceRole(err.msg))
                }
                "ProjectConfigurationException" => {
                    return RusotoError::Service(DescribeProjectError::ProjectConfiguration(
                        err.msg,
                    ))
                }
                "ProjectNotFoundException" => {
                    return RusotoError::Service(DescribeProjectError::ProjectNotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DescribeProjectError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeProjectError::ConcurrentModification(ref cause) => write!(f, "{}", cause),
            DescribeProjectError::InvalidServiceRole(ref cause) => write!(f, "{}", cause),
            DescribeProjectError::ProjectConfiguration(ref cause) => write!(f, "{}", cause),
            DescribeProjectError::ProjectNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DescribeProjectError {}
/// Errors returned by DescribeUserProfile
#[derive(Debug, PartialEq)]
pub enum DescribeUserProfileError {
    /// <p>The user profile was not found.</p>
    UserProfileNotFound(String),
}

impl DescribeUserProfileError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeUserProfileError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "UserProfileNotFoundException" => {
                    return RusotoError::Service(DescribeUserProfileError::UserProfileNotFound(
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
impl fmt::Display for DescribeUserProfileError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeUserProfileError::UserProfileNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DescribeUserProfileError {}
/// Errors returned by DisassociateTeamMember
#[derive(Debug, PartialEq)]
pub enum DisassociateTeamMemberError {
    /// <p>Another modification is being made. That modification must complete before you can make your change.</p>
    ConcurrentModification(String),
    /// <p>The service role is not valid.</p>
    InvalidServiceRole(String),
    /// <p>The specified AWS CodeStar project was not found.</p>
    ProjectNotFound(String),
}

impl DisassociateTeamMemberError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DisassociateTeamMemberError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ConcurrentModificationException" => {
                    return RusotoError::Service(
                        DisassociateTeamMemberError::ConcurrentModification(err.msg),
                    )
                }
                "InvalidServiceRoleException" => {
                    return RusotoError::Service(DisassociateTeamMemberError::InvalidServiceRole(
                        err.msg,
                    ))
                }
                "ProjectNotFoundException" => {
                    return RusotoError::Service(DisassociateTeamMemberError::ProjectNotFound(
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
impl fmt::Display for DisassociateTeamMemberError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DisassociateTeamMemberError::ConcurrentModification(ref cause) => {
                write!(f, "{}", cause)
            }
            DisassociateTeamMemberError::InvalidServiceRole(ref cause) => write!(f, "{}", cause),
            DisassociateTeamMemberError::ProjectNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DisassociateTeamMemberError {}
/// Errors returned by ListProjects
#[derive(Debug, PartialEq)]
pub enum ListProjectsError {
    /// <p>The next token is not valid.</p>
    InvalidNextToken(String),
}

impl ListProjectsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListProjectsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidNextTokenException" => {
                    return RusotoError::Service(ListProjectsError::InvalidNextToken(err.msg))
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
            ListProjectsError::InvalidNextToken(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListProjectsError {}
/// Errors returned by ListResources
#[derive(Debug, PartialEq)]
pub enum ListResourcesError {
    /// <p>The next token is not valid.</p>
    InvalidNextToken(String),
    /// <p>The specified AWS CodeStar project was not found.</p>
    ProjectNotFound(String),
}

impl ListResourcesError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListResourcesError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidNextTokenException" => {
                    return RusotoError::Service(ListResourcesError::InvalidNextToken(err.msg))
                }
                "ProjectNotFoundException" => {
                    return RusotoError::Service(ListResourcesError::ProjectNotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ListResourcesError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListResourcesError::InvalidNextToken(ref cause) => write!(f, "{}", cause),
            ListResourcesError::ProjectNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListResourcesError {}
/// Errors returned by ListTagsForProject
#[derive(Debug, PartialEq)]
pub enum ListTagsForProjectError {
    /// <p>The next token is not valid.</p>
    InvalidNextToken(String),
    /// <p>The specified AWS CodeStar project was not found.</p>
    ProjectNotFound(String),
}

impl ListTagsForProjectError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListTagsForProjectError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidNextTokenException" => {
                    return RusotoError::Service(ListTagsForProjectError::InvalidNextToken(err.msg))
                }
                "ProjectNotFoundException" => {
                    return RusotoError::Service(ListTagsForProjectError::ProjectNotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ListTagsForProjectError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListTagsForProjectError::InvalidNextToken(ref cause) => write!(f, "{}", cause),
            ListTagsForProjectError::ProjectNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListTagsForProjectError {}
/// Errors returned by ListTeamMembers
#[derive(Debug, PartialEq)]
pub enum ListTeamMembersError {
    /// <p>The next token is not valid.</p>
    InvalidNextToken(String),
    /// <p>The specified AWS CodeStar project was not found.</p>
    ProjectNotFound(String),
}

impl ListTeamMembersError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListTeamMembersError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidNextTokenException" => {
                    return RusotoError::Service(ListTeamMembersError::InvalidNextToken(err.msg))
                }
                "ProjectNotFoundException" => {
                    return RusotoError::Service(ListTeamMembersError::ProjectNotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ListTeamMembersError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListTeamMembersError::InvalidNextToken(ref cause) => write!(f, "{}", cause),
            ListTeamMembersError::ProjectNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListTeamMembersError {}
/// Errors returned by ListUserProfiles
#[derive(Debug, PartialEq)]
pub enum ListUserProfilesError {
    /// <p>The next token is not valid.</p>
    InvalidNextToken(String),
}

impl ListUserProfilesError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListUserProfilesError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidNextTokenException" => {
                    return RusotoError::Service(ListUserProfilesError::InvalidNextToken(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ListUserProfilesError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListUserProfilesError::InvalidNextToken(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListUserProfilesError {}
/// Errors returned by TagProject
#[derive(Debug, PartialEq)]
pub enum TagProjectError {
    /// <p>Another modification is being made. That modification must complete before you can make your change.</p>
    ConcurrentModification(String),
    /// <p>A resource limit has been exceeded.</p>
    LimitExceeded(String),
    /// <p>The specified AWS CodeStar project was not found.</p>
    ProjectNotFound(String),
}

impl TagProjectError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<TagProjectError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ConcurrentModificationException" => {
                    return RusotoError::Service(TagProjectError::ConcurrentModification(err.msg))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(TagProjectError::LimitExceeded(err.msg))
                }
                "ProjectNotFoundException" => {
                    return RusotoError::Service(TagProjectError::ProjectNotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for TagProjectError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            TagProjectError::ConcurrentModification(ref cause) => write!(f, "{}", cause),
            TagProjectError::LimitExceeded(ref cause) => write!(f, "{}", cause),
            TagProjectError::ProjectNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for TagProjectError {}
/// Errors returned by UntagProject
#[derive(Debug, PartialEq)]
pub enum UntagProjectError {
    /// <p>Another modification is being made. That modification must complete before you can make your change.</p>
    ConcurrentModification(String),
    /// <p>A resource limit has been exceeded.</p>
    LimitExceeded(String),
    /// <p>The specified AWS CodeStar project was not found.</p>
    ProjectNotFound(String),
}

impl UntagProjectError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UntagProjectError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ConcurrentModificationException" => {
                    return RusotoError::Service(UntagProjectError::ConcurrentModification(err.msg))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(UntagProjectError::LimitExceeded(err.msg))
                }
                "ProjectNotFoundException" => {
                    return RusotoError::Service(UntagProjectError::ProjectNotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for UntagProjectError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UntagProjectError::ConcurrentModification(ref cause) => write!(f, "{}", cause),
            UntagProjectError::LimitExceeded(ref cause) => write!(f, "{}", cause),
            UntagProjectError::ProjectNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for UntagProjectError {}
/// Errors returned by UpdateProject
#[derive(Debug, PartialEq)]
pub enum UpdateProjectError {
    /// <p>The specified AWS CodeStar project was not found.</p>
    ProjectNotFound(String),
}

impl UpdateProjectError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdateProjectError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ProjectNotFoundException" => {
                    return RusotoError::Service(UpdateProjectError::ProjectNotFound(err.msg))
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
            UpdateProjectError::ProjectNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for UpdateProjectError {}
/// Errors returned by UpdateTeamMember
#[derive(Debug, PartialEq)]
pub enum UpdateTeamMemberError {
    /// <p>Another modification is being made. That modification must complete before you can make your change.</p>
    ConcurrentModification(String),
    /// <p>The service role is not valid.</p>
    InvalidServiceRole(String),
    /// <p>A resource limit has been exceeded.</p>
    LimitExceeded(String),
    /// <p>Project configuration information is required but not specified.</p>
    ProjectConfiguration(String),
    /// <p>The specified AWS CodeStar project was not found.</p>
    ProjectNotFound(String),
    /// <p>The specified team member was not found.</p>
    TeamMemberNotFound(String),
}

impl UpdateTeamMemberError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdateTeamMemberError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ConcurrentModificationException" => {
                    return RusotoError::Service(UpdateTeamMemberError::ConcurrentModification(
                        err.msg,
                    ))
                }
                "InvalidServiceRoleException" => {
                    return RusotoError::Service(UpdateTeamMemberError::InvalidServiceRole(err.msg))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(UpdateTeamMemberError::LimitExceeded(err.msg))
                }
                "ProjectConfigurationException" => {
                    return RusotoError::Service(UpdateTeamMemberError::ProjectConfiguration(
                        err.msg,
                    ))
                }
                "ProjectNotFoundException" => {
                    return RusotoError::Service(UpdateTeamMemberError::ProjectNotFound(err.msg))
                }
                "TeamMemberNotFoundException" => {
                    return RusotoError::Service(UpdateTeamMemberError::TeamMemberNotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for UpdateTeamMemberError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UpdateTeamMemberError::ConcurrentModification(ref cause) => write!(f, "{}", cause),
            UpdateTeamMemberError::InvalidServiceRole(ref cause) => write!(f, "{}", cause),
            UpdateTeamMemberError::LimitExceeded(ref cause) => write!(f, "{}", cause),
            UpdateTeamMemberError::ProjectConfiguration(ref cause) => write!(f, "{}", cause),
            UpdateTeamMemberError::ProjectNotFound(ref cause) => write!(f, "{}", cause),
            UpdateTeamMemberError::TeamMemberNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for UpdateTeamMemberError {}
/// Errors returned by UpdateUserProfile
#[derive(Debug, PartialEq)]
pub enum UpdateUserProfileError {
    /// <p>The user profile was not found.</p>
    UserProfileNotFound(String),
}

impl UpdateUserProfileError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdateUserProfileError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "UserProfileNotFoundException" => {
                    return RusotoError::Service(UpdateUserProfileError::UserProfileNotFound(
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
impl fmt::Display for UpdateUserProfileError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UpdateUserProfileError::UserProfileNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for UpdateUserProfileError {}
/// Trait representing the capabilities of the CodeStar API. CodeStar clients implement this trait.
#[async_trait]
pub trait CodeStar {
    /// <p>Adds an IAM user to the team for an AWS CodeStar project.</p>
    async fn associate_team_member(
        &self,
        input: AssociateTeamMemberRequest,
    ) -> Result<AssociateTeamMemberResult, RusotoError<AssociateTeamMemberError>>;

    /// <p>Creates a project, including project resources. This action creates a project based on a submitted project request. A set of source code files and a toolchain template file can be included with the project request. If these are not provided, an empty project is created.</p>
    async fn create_project(
        &self,
        input: CreateProjectRequest,
    ) -> Result<CreateProjectResult, RusotoError<CreateProjectError>>;

    /// <p>Creates a profile for a user that includes user preferences, such as the display name and email address assocciated with the user, in AWS CodeStar. The user profile is not project-specific. Information in the user profile is displayed wherever the user's information appears to other users in AWS CodeStar.</p>
    async fn create_user_profile(
        &self,
        input: CreateUserProfileRequest,
    ) -> Result<CreateUserProfileResult, RusotoError<CreateUserProfileError>>;

    /// <p>Deletes a project, including project resources. Does not delete users associated with the project, but does delete the IAM roles that allowed access to the project.</p>
    async fn delete_project(
        &self,
        input: DeleteProjectRequest,
    ) -> Result<DeleteProjectResult, RusotoError<DeleteProjectError>>;

    /// <p>Deletes a user profile in AWS CodeStar, including all personal preference data associated with that profile, such as display name and email address. It does not delete the history of that user, for example the history of commits made by that user.</p>
    async fn delete_user_profile(
        &self,
        input: DeleteUserProfileRequest,
    ) -> Result<DeleteUserProfileResult, RusotoError<DeleteUserProfileError>>;

    /// <p>Describes a project and its resources.</p>
    async fn describe_project(
        &self,
        input: DescribeProjectRequest,
    ) -> Result<DescribeProjectResult, RusotoError<DescribeProjectError>>;

    /// <p>Describes a user in AWS CodeStar and the user attributes across all projects.</p>
    async fn describe_user_profile(
        &self,
        input: DescribeUserProfileRequest,
    ) -> Result<DescribeUserProfileResult, RusotoError<DescribeUserProfileError>>;

    /// <p>Removes a user from a project. Removing a user from a project also removes the IAM policies from that user that allowed access to the project and its resources. Disassociating a team member does not remove that user's profile from AWS CodeStar. It does not remove the user from IAM.</p>
    async fn disassociate_team_member(
        &self,
        input: DisassociateTeamMemberRequest,
    ) -> Result<DisassociateTeamMemberResult, RusotoError<DisassociateTeamMemberError>>;

    /// <p>Lists all projects in AWS CodeStar associated with your AWS account.</p>
    async fn list_projects(
        &self,
        input: ListProjectsRequest,
    ) -> Result<ListProjectsResult, RusotoError<ListProjectsError>>;

    /// <p>Lists resources associated with a project in AWS CodeStar.</p>
    async fn list_resources(
        &self,
        input: ListResourcesRequest,
    ) -> Result<ListResourcesResult, RusotoError<ListResourcesError>>;

    /// <p>Gets the tags for a project.</p>
    async fn list_tags_for_project(
        &self,
        input: ListTagsForProjectRequest,
    ) -> Result<ListTagsForProjectResult, RusotoError<ListTagsForProjectError>>;

    /// <p>Lists all team members associated with a project.</p>
    async fn list_team_members(
        &self,
        input: ListTeamMembersRequest,
    ) -> Result<ListTeamMembersResult, RusotoError<ListTeamMembersError>>;

    /// <p>Lists all the user profiles configured for your AWS account in AWS CodeStar.</p>
    async fn list_user_profiles(
        &self,
        input: ListUserProfilesRequest,
    ) -> Result<ListUserProfilesResult, RusotoError<ListUserProfilesError>>;

    /// <p>Adds tags to a project.</p>
    async fn tag_project(
        &self,
        input: TagProjectRequest,
    ) -> Result<TagProjectResult, RusotoError<TagProjectError>>;

    /// <p>Removes tags from a project.</p>
    async fn untag_project(
        &self,
        input: UntagProjectRequest,
    ) -> Result<UntagProjectResult, RusotoError<UntagProjectError>>;

    /// <p>Updates a project in AWS CodeStar.</p>
    async fn update_project(
        &self,
        input: UpdateProjectRequest,
    ) -> Result<UpdateProjectResult, RusotoError<UpdateProjectError>>;

    /// <p>Updates a team member's attributes in an AWS CodeStar project. For example, you can change a team member's role in the project, or change whether they have remote access to project resources.</p>
    async fn update_team_member(
        &self,
        input: UpdateTeamMemberRequest,
    ) -> Result<UpdateTeamMemberResult, RusotoError<UpdateTeamMemberError>>;

    /// <p>Updates a user's profile in AWS CodeStar. The user profile is not project-specific. Information in the user profile is displayed wherever the user's information appears to other users in AWS CodeStar. </p>
    async fn update_user_profile(
        &self,
        input: UpdateUserProfileRequest,
    ) -> Result<UpdateUserProfileResult, RusotoError<UpdateUserProfileError>>;
}
/// A client for the CodeStar API.
#[derive(Clone)]
pub struct CodeStarClient {
    client: Client,
    region: region::Region,
}

impl CodeStarClient {
    /// Creates a client backed by the default tokio event loop.
    ///
    /// The client will use the default credentials provider and tls client.
    pub fn new(region: region::Region) -> CodeStarClient {
        CodeStarClient {
            client: Client::shared(),
            region,
        }
    }

    pub fn new_with<P, D>(
        request_dispatcher: D,
        credentials_provider: P,
        region: region::Region,
    ) -> CodeStarClient
    where
        P: ProvideAwsCredentials + Send + Sync + 'static,
        D: DispatchSignedRequest + Send + Sync + 'static,
    {
        CodeStarClient {
            client: Client::new_with(credentials_provider, request_dispatcher),
            region,
        }
    }

    pub fn new_with_client(client: Client, region: region::Region) -> CodeStarClient {
        CodeStarClient { client, region }
    }
}

#[async_trait]
impl CodeStar for CodeStarClient {
    /// <p>Adds an IAM user to the team for an AWS CodeStar project.</p>
    async fn associate_team_member(
        &self,
        input: AssociateTeamMemberRequest,
    ) -> Result<AssociateTeamMemberResult, RusotoError<AssociateTeamMemberError>> {
        let mut request = SignedRequest::new("POST", "codestar", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "CodeStar_20170419.AssociateTeamMember");
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
                .deserialize::<AssociateTeamMemberResult, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(AssociateTeamMemberError::from_response(response))
        }
    }

    /// <p>Creates a project, including project resources. This action creates a project based on a submitted project request. A set of source code files and a toolchain template file can be included with the project request. If these are not provided, an empty project is created.</p>
    async fn create_project(
        &self,
        input: CreateProjectRequest,
    ) -> Result<CreateProjectResult, RusotoError<CreateProjectError>> {
        let mut request = SignedRequest::new("POST", "codestar", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "CodeStar_20170419.CreateProject");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response).deserialize::<CreateProjectResult, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(CreateProjectError::from_response(response))
        }
    }

    /// <p>Creates a profile for a user that includes user preferences, such as the display name and email address assocciated with the user, in AWS CodeStar. The user profile is not project-specific. Information in the user profile is displayed wherever the user's information appears to other users in AWS CodeStar.</p>
    async fn create_user_profile(
        &self,
        input: CreateUserProfileRequest,
    ) -> Result<CreateUserProfileResult, RusotoError<CreateUserProfileError>> {
        let mut request = SignedRequest::new("POST", "codestar", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "CodeStar_20170419.CreateUserProfile");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response).deserialize::<CreateUserProfileResult, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(CreateUserProfileError::from_response(response))
        }
    }

    /// <p>Deletes a project, including project resources. Does not delete users associated with the project, but does delete the IAM roles that allowed access to the project.</p>
    async fn delete_project(
        &self,
        input: DeleteProjectRequest,
    ) -> Result<DeleteProjectResult, RusotoError<DeleteProjectError>> {
        let mut request = SignedRequest::new("POST", "codestar", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "CodeStar_20170419.DeleteProject");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response).deserialize::<DeleteProjectResult, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(DeleteProjectError::from_response(response))
        }
    }

    /// <p>Deletes a user profile in AWS CodeStar, including all personal preference data associated with that profile, such as display name and email address. It does not delete the history of that user, for example the history of commits made by that user.</p>
    async fn delete_user_profile(
        &self,
        input: DeleteUserProfileRequest,
    ) -> Result<DeleteUserProfileResult, RusotoError<DeleteUserProfileError>> {
        let mut request = SignedRequest::new("POST", "codestar", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "CodeStar_20170419.DeleteUserProfile");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response).deserialize::<DeleteUserProfileResult, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(DeleteUserProfileError::from_response(response))
        }
    }

    /// <p>Describes a project and its resources.</p>
    async fn describe_project(
        &self,
        input: DescribeProjectRequest,
    ) -> Result<DescribeProjectResult, RusotoError<DescribeProjectError>> {
        let mut request = SignedRequest::new("POST", "codestar", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "CodeStar_20170419.DescribeProject");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response).deserialize::<DescribeProjectResult, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(DescribeProjectError::from_response(response))
        }
    }

    /// <p>Describes a user in AWS CodeStar and the user attributes across all projects.</p>
    async fn describe_user_profile(
        &self,
        input: DescribeUserProfileRequest,
    ) -> Result<DescribeUserProfileResult, RusotoError<DescribeUserProfileError>> {
        let mut request = SignedRequest::new("POST", "codestar", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "CodeStar_20170419.DescribeUserProfile");
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
                .deserialize::<DescribeUserProfileResult, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(DescribeUserProfileError::from_response(response))
        }
    }

    /// <p>Removes a user from a project. Removing a user from a project also removes the IAM policies from that user that allowed access to the project and its resources. Disassociating a team member does not remove that user's profile from AWS CodeStar. It does not remove the user from IAM.</p>
    async fn disassociate_team_member(
        &self,
        input: DisassociateTeamMemberRequest,
    ) -> Result<DisassociateTeamMemberResult, RusotoError<DisassociateTeamMemberError>> {
        let mut request = SignedRequest::new("POST", "codestar", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "CodeStar_20170419.DisassociateTeamMember");
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
                .deserialize::<DisassociateTeamMemberResult, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(DisassociateTeamMemberError::from_response(response))
        }
    }

    /// <p>Lists all projects in AWS CodeStar associated with your AWS account.</p>
    async fn list_projects(
        &self,
        input: ListProjectsRequest,
    ) -> Result<ListProjectsResult, RusotoError<ListProjectsError>> {
        let mut request = SignedRequest::new("POST", "codestar", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "CodeStar_20170419.ListProjects");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response).deserialize::<ListProjectsResult, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(ListProjectsError::from_response(response))
        }
    }

    /// <p>Lists resources associated with a project in AWS CodeStar.</p>
    async fn list_resources(
        &self,
        input: ListResourcesRequest,
    ) -> Result<ListResourcesResult, RusotoError<ListResourcesError>> {
        let mut request = SignedRequest::new("POST", "codestar", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "CodeStar_20170419.ListResources");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response).deserialize::<ListResourcesResult, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(ListResourcesError::from_response(response))
        }
    }

    /// <p>Gets the tags for a project.</p>
    async fn list_tags_for_project(
        &self,
        input: ListTagsForProjectRequest,
    ) -> Result<ListTagsForProjectResult, RusotoError<ListTagsForProjectError>> {
        let mut request = SignedRequest::new("POST", "codestar", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "CodeStar_20170419.ListTagsForProject");
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
                .deserialize::<ListTagsForProjectResult, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(ListTagsForProjectError::from_response(response))
        }
    }

    /// <p>Lists all team members associated with a project.</p>
    async fn list_team_members(
        &self,
        input: ListTeamMembersRequest,
    ) -> Result<ListTeamMembersResult, RusotoError<ListTeamMembersError>> {
        let mut request = SignedRequest::new("POST", "codestar", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "CodeStar_20170419.ListTeamMembers");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response).deserialize::<ListTeamMembersResult, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(ListTeamMembersError::from_response(response))
        }
    }

    /// <p>Lists all the user profiles configured for your AWS account in AWS CodeStar.</p>
    async fn list_user_profiles(
        &self,
        input: ListUserProfilesRequest,
    ) -> Result<ListUserProfilesResult, RusotoError<ListUserProfilesError>> {
        let mut request = SignedRequest::new("POST", "codestar", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "CodeStar_20170419.ListUserProfiles");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response).deserialize::<ListUserProfilesResult, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(ListUserProfilesError::from_response(response))
        }
    }

    /// <p>Adds tags to a project.</p>
    async fn tag_project(
        &self,
        input: TagProjectRequest,
    ) -> Result<TagProjectResult, RusotoError<TagProjectError>> {
        let mut request = SignedRequest::new("POST", "codestar", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "CodeStar_20170419.TagProject");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response).deserialize::<TagProjectResult, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(TagProjectError::from_response(response))
        }
    }

    /// <p>Removes tags from a project.</p>
    async fn untag_project(
        &self,
        input: UntagProjectRequest,
    ) -> Result<UntagProjectResult, RusotoError<UntagProjectError>> {
        let mut request = SignedRequest::new("POST", "codestar", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "CodeStar_20170419.UntagProject");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response).deserialize::<UntagProjectResult, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(UntagProjectError::from_response(response))
        }
    }

    /// <p>Updates a project in AWS CodeStar.</p>
    async fn update_project(
        &self,
        input: UpdateProjectRequest,
    ) -> Result<UpdateProjectResult, RusotoError<UpdateProjectError>> {
        let mut request = SignedRequest::new("POST", "codestar", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "CodeStar_20170419.UpdateProject");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response).deserialize::<UpdateProjectResult, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(UpdateProjectError::from_response(response))
        }
    }

    /// <p>Updates a team member's attributes in an AWS CodeStar project. For example, you can change a team member's role in the project, or change whether they have remote access to project resources.</p>
    async fn update_team_member(
        &self,
        input: UpdateTeamMemberRequest,
    ) -> Result<UpdateTeamMemberResult, RusotoError<UpdateTeamMemberError>> {
        let mut request = SignedRequest::new("POST", "codestar", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "CodeStar_20170419.UpdateTeamMember");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response).deserialize::<UpdateTeamMemberResult, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(UpdateTeamMemberError::from_response(response))
        }
    }

    /// <p>Updates a user's profile in AWS CodeStar. The user profile is not project-specific. Information in the user profile is displayed wherever the user's information appears to other users in AWS CodeStar. </p>
    async fn update_user_profile(
        &self,
        input: UpdateUserProfileRequest,
    ) -> Result<UpdateUserProfileResult, RusotoError<UpdateUserProfileError>> {
        let mut request = SignedRequest::new("POST", "codestar", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "CodeStar_20170419.UpdateUserProfile");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response).deserialize::<UpdateUserProfileResult, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(UpdateUserProfileError::from_response(response))
        }
    }
}
