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

use rusoto_core::signature::SignedRequest;
use serde_json;
use serde_json::from_slice;
use serde_json::Value as SerdeJsonValue;
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
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
pub struct AssociateTeamMemberResult {
    /// <p>The user- or system-generated token from the initial request that can be used to repeat the request.</p>
    #[serde(rename = "clientRequestToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_request_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct CreateProjectRequest {
    /// <p>Reserved for future use.</p>
    #[serde(rename = "clientRequestToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_request_token: Option<String>,
    /// <p>Reserved for future use.</p>
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>Reserved for future use.</p>
    #[serde(rename = "id")]
    pub id: String,
    /// <p>Reserved for future use.</p>
    #[serde(rename = "name")]
    pub name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct CreateProjectResult {
    /// <p>Reserved for future use.</p>
    #[serde(rename = "arn")]
    pub arn: String,
    /// <p>Reserved for future use.</p>
    #[serde(rename = "clientRequestToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_request_token: Option<String>,
    /// <p>Reserved for future use.</p>
    #[serde(rename = "id")]
    pub id: String,
    /// <p>Reserved for future use.</p>
    #[serde(rename = "projectTemplateId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub project_template_id: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
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
pub struct DeleteUserProfileRequest {
    /// <p>The Amazon Resource Name (ARN) of the user to delete from AWS CodeStar.</p>
    #[serde(rename = "userArn")]
    pub user_arn: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct DeleteUserProfileResult {
    /// <p>The Amazon Resource Name (ARN) of the user deleted from AWS CodeStar.</p>
    #[serde(rename = "userArn")]
    pub user_arn: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DescribeProjectRequest {
    /// <p>The ID of the project.</p>
    #[serde(rename = "id")]
    pub id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
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
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DescribeUserProfileRequest {
    /// <p>The Amazon Resource Name (ARN) of the user.</p>
    #[serde(rename = "userArn")]
    pub user_arn: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
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
pub struct DisassociateTeamMemberRequest {
    /// <p>The ID of the AWS CodeStar project from which you want to remove a team member.</p>
    #[serde(rename = "projectId")]
    pub project_id: String,
    /// <p>The Amazon Resource Name (ARN) of the IAM user or group whom you want to remove from the project.</p>
    #[serde(rename = "userArn")]
    pub user_arn: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct DisassociateTeamMemberResult {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
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
pub struct ListUserProfilesResult {
    /// <p>The continuation token to use when requesting the next set of results, if there are more results to be returned.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>All the user profiles configured in AWS CodeStar for an AWS account.</p>
    #[serde(rename = "userProfiles")]
    pub user_profiles: Vec<UserProfileSummary>,
}

/// <p>Information about the metadata for a project.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
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
pub struct Resource {
    /// <p>The Amazon Resource Name (ARN) of the resource.</p>
    #[serde(rename = "id")]
    pub id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct TagProjectRequest {
    /// <p>The ID of the project you want to add a tag to.</p>
    #[serde(rename = "id")]
    pub id: String,
    /// <p>The tags you want to add to the project.</p>
    #[serde(rename = "tags")]
    pub tags: ::std::collections::HashMap<String, String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct TagProjectResult {
    /// <p>The tags for the project.</p>
    #[serde(rename = "tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<::std::collections::HashMap<String, String>>,
}

/// <p>Information about a team member in a project.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
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

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct UntagProjectRequest {
    /// <p>The ID of the project to remove tags from.</p>
    #[serde(rename = "id")]
    pub id: String,
    /// <p>The tags to remove from the project.</p>
    #[serde(rename = "tags")]
    pub tags: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct UntagProjectResult {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
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
pub struct UpdateProjectResult {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
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

impl AssociateTeamMemberError {
    pub fn from_response(res: BufferedHttpResponse) -> AssociateTeamMemberError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "ConcurrentModificationException" => {
                    return AssociateTeamMemberError::ConcurrentModification(String::from(
                        error_message,
                    ))
                }
                "InvalidServiceRoleException" => {
                    return AssociateTeamMemberError::InvalidServiceRole(String::from(error_message))
                }
                "LimitExceededException" => {
                    return AssociateTeamMemberError::LimitExceeded(String::from(error_message))
                }
                "ProjectConfigurationException" => {
                    return AssociateTeamMemberError::ProjectConfiguration(String::from(
                        error_message,
                    ))
                }
                "ProjectNotFoundException" => {
                    return AssociateTeamMemberError::ProjectNotFound(String::from(error_message))
                }
                "TeamMemberAlreadyAssociatedException" => {
                    return AssociateTeamMemberError::TeamMemberAlreadyAssociated(String::from(
                        error_message,
                    ))
                }
                "ValidationException" => {
                    return AssociateTeamMemberError::Validation(error_message.to_string())
                }
                _ => {}
            }
        }
        return AssociateTeamMemberError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for AssociateTeamMemberError {
    fn from(err: serde_json::error::Error) -> AssociateTeamMemberError {
        AssociateTeamMemberError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for AssociateTeamMemberError {
    fn from(err: CredentialsError) -> AssociateTeamMemberError {
        AssociateTeamMemberError::Credentials(err)
    }
}
impl From<HttpDispatchError> for AssociateTeamMemberError {
    fn from(err: HttpDispatchError) -> AssociateTeamMemberError {
        AssociateTeamMemberError::HttpDispatch(err)
    }
}
impl From<io::Error> for AssociateTeamMemberError {
    fn from(err: io::Error) -> AssociateTeamMemberError {
        AssociateTeamMemberError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for AssociateTeamMemberError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for AssociateTeamMemberError {
    fn description(&self) -> &str {
        match *self {
            AssociateTeamMemberError::ConcurrentModification(ref cause) => cause,
            AssociateTeamMemberError::InvalidServiceRole(ref cause) => cause,
            AssociateTeamMemberError::LimitExceeded(ref cause) => cause,
            AssociateTeamMemberError::ProjectConfiguration(ref cause) => cause,
            AssociateTeamMemberError::ProjectNotFound(ref cause) => cause,
            AssociateTeamMemberError::TeamMemberAlreadyAssociated(ref cause) => cause,
            AssociateTeamMemberError::Validation(ref cause) => cause,
            AssociateTeamMemberError::Credentials(ref err) => err.description(),
            AssociateTeamMemberError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            AssociateTeamMemberError::ParseError(ref cause) => cause,
            AssociateTeamMemberError::Unknown(_) => "unknown error",
        }
    }
}
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

impl CreateProjectError {
    pub fn from_response(res: BufferedHttpResponse) -> CreateProjectError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "ConcurrentModificationException" => {
                    return CreateProjectError::ConcurrentModification(String::from(error_message))
                }
                "InvalidServiceRoleException" => {
                    return CreateProjectError::InvalidServiceRole(String::from(error_message))
                }
                "LimitExceededException" => {
                    return CreateProjectError::LimitExceeded(String::from(error_message))
                }
                "ProjectAlreadyExistsException" => {
                    return CreateProjectError::ProjectAlreadyExists(String::from(error_message))
                }
                "ProjectConfigurationException" => {
                    return CreateProjectError::ProjectConfiguration(String::from(error_message))
                }
                "ProjectCreationFailedException" => {
                    return CreateProjectError::ProjectCreationFailed(String::from(error_message))
                }
                "ValidationException" => {
                    return CreateProjectError::Validation(error_message.to_string())
                }
                _ => {}
            }
        }
        return CreateProjectError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for CreateProjectError {
    fn from(err: serde_json::error::Error) -> CreateProjectError {
        CreateProjectError::ParseError(err.description().to_string())
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
            CreateProjectError::ConcurrentModification(ref cause) => cause,
            CreateProjectError::InvalidServiceRole(ref cause) => cause,
            CreateProjectError::LimitExceeded(ref cause) => cause,
            CreateProjectError::ProjectAlreadyExists(ref cause) => cause,
            CreateProjectError::ProjectConfiguration(ref cause) => cause,
            CreateProjectError::ProjectCreationFailed(ref cause) => cause,
            CreateProjectError::Validation(ref cause) => cause,
            CreateProjectError::Credentials(ref err) => err.description(),
            CreateProjectError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            CreateProjectError::ParseError(ref cause) => cause,
            CreateProjectError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by CreateUserProfile
#[derive(Debug, PartialEq)]
pub enum CreateUserProfileError {
    /// <p>A user profile with that name already exists in this region for the AWS account. AWS CodeStar user profile names must be unique within a region for the AWS account. </p>
    UserProfileAlreadyExists(String),
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

impl CreateUserProfileError {
    pub fn from_response(res: BufferedHttpResponse) -> CreateUserProfileError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "UserProfileAlreadyExistsException" => {
                    return CreateUserProfileError::UserProfileAlreadyExists(String::from(
                        error_message,
                    ))
                }
                "ValidationException" => {
                    return CreateUserProfileError::Validation(error_message.to_string())
                }
                _ => {}
            }
        }
        return CreateUserProfileError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for CreateUserProfileError {
    fn from(err: serde_json::error::Error) -> CreateUserProfileError {
        CreateUserProfileError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for CreateUserProfileError {
    fn from(err: CredentialsError) -> CreateUserProfileError {
        CreateUserProfileError::Credentials(err)
    }
}
impl From<HttpDispatchError> for CreateUserProfileError {
    fn from(err: HttpDispatchError) -> CreateUserProfileError {
        CreateUserProfileError::HttpDispatch(err)
    }
}
impl From<io::Error> for CreateUserProfileError {
    fn from(err: io::Error) -> CreateUserProfileError {
        CreateUserProfileError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for CreateUserProfileError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateUserProfileError {
    fn description(&self) -> &str {
        match *self {
            CreateUserProfileError::UserProfileAlreadyExists(ref cause) => cause,
            CreateUserProfileError::Validation(ref cause) => cause,
            CreateUserProfileError::Credentials(ref err) => err.description(),
            CreateUserProfileError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            CreateUserProfileError::ParseError(ref cause) => cause,
            CreateUserProfileError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by DeleteProject
#[derive(Debug, PartialEq)]
pub enum DeleteProjectError {
    /// <p>Another modification is being made. That modification must complete before you can make your change.</p>
    ConcurrentModification(String),
    /// <p>The service role is not valid.</p>
    InvalidServiceRole(String),
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

impl DeleteProjectError {
    pub fn from_response(res: BufferedHttpResponse) -> DeleteProjectError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "ConcurrentModificationException" => {
                    return DeleteProjectError::ConcurrentModification(String::from(error_message))
                }
                "InvalidServiceRoleException" => {
                    return DeleteProjectError::InvalidServiceRole(String::from(error_message))
                }
                "ValidationException" => {
                    return DeleteProjectError::Validation(error_message.to_string())
                }
                _ => {}
            }
        }
        return DeleteProjectError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for DeleteProjectError {
    fn from(err: serde_json::error::Error) -> DeleteProjectError {
        DeleteProjectError::ParseError(err.description().to_string())
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
            DeleteProjectError::ConcurrentModification(ref cause) => cause,
            DeleteProjectError::InvalidServiceRole(ref cause) => cause,
            DeleteProjectError::Validation(ref cause) => cause,
            DeleteProjectError::Credentials(ref err) => err.description(),
            DeleteProjectError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            DeleteProjectError::ParseError(ref cause) => cause,
            DeleteProjectError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by DeleteUserProfile
#[derive(Debug, PartialEq)]
pub enum DeleteUserProfileError {
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

impl DeleteUserProfileError {
    pub fn from_response(res: BufferedHttpResponse) -> DeleteUserProfileError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "ValidationException" => {
                    return DeleteUserProfileError::Validation(error_message.to_string())
                }
                _ => {}
            }
        }
        return DeleteUserProfileError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for DeleteUserProfileError {
    fn from(err: serde_json::error::Error) -> DeleteUserProfileError {
        DeleteUserProfileError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for DeleteUserProfileError {
    fn from(err: CredentialsError) -> DeleteUserProfileError {
        DeleteUserProfileError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DeleteUserProfileError {
    fn from(err: HttpDispatchError) -> DeleteUserProfileError {
        DeleteUserProfileError::HttpDispatch(err)
    }
}
impl From<io::Error> for DeleteUserProfileError {
    fn from(err: io::Error) -> DeleteUserProfileError {
        DeleteUserProfileError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DeleteUserProfileError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteUserProfileError {
    fn description(&self) -> &str {
        match *self {
            DeleteUserProfileError::Validation(ref cause) => cause,
            DeleteUserProfileError::Credentials(ref err) => err.description(),
            DeleteUserProfileError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DeleteUserProfileError::ParseError(ref cause) => cause,
            DeleteUserProfileError::Unknown(_) => "unknown error",
        }
    }
}
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

impl DescribeProjectError {
    pub fn from_response(res: BufferedHttpResponse) -> DescribeProjectError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "ConcurrentModificationException" => {
                    return DescribeProjectError::ConcurrentModification(String::from(error_message))
                }
                "InvalidServiceRoleException" => {
                    return DescribeProjectError::InvalidServiceRole(String::from(error_message))
                }
                "ProjectConfigurationException" => {
                    return DescribeProjectError::ProjectConfiguration(String::from(error_message))
                }
                "ProjectNotFoundException" => {
                    return DescribeProjectError::ProjectNotFound(String::from(error_message))
                }
                "ValidationException" => {
                    return DescribeProjectError::Validation(error_message.to_string())
                }
                _ => {}
            }
        }
        return DescribeProjectError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for DescribeProjectError {
    fn from(err: serde_json::error::Error) -> DescribeProjectError {
        DescribeProjectError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for DescribeProjectError {
    fn from(err: CredentialsError) -> DescribeProjectError {
        DescribeProjectError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DescribeProjectError {
    fn from(err: HttpDispatchError) -> DescribeProjectError {
        DescribeProjectError::HttpDispatch(err)
    }
}
impl From<io::Error> for DescribeProjectError {
    fn from(err: io::Error) -> DescribeProjectError {
        DescribeProjectError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DescribeProjectError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeProjectError {
    fn description(&self) -> &str {
        match *self {
            DescribeProjectError::ConcurrentModification(ref cause) => cause,
            DescribeProjectError::InvalidServiceRole(ref cause) => cause,
            DescribeProjectError::ProjectConfiguration(ref cause) => cause,
            DescribeProjectError::ProjectNotFound(ref cause) => cause,
            DescribeProjectError::Validation(ref cause) => cause,
            DescribeProjectError::Credentials(ref err) => err.description(),
            DescribeProjectError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            DescribeProjectError::ParseError(ref cause) => cause,
            DescribeProjectError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by DescribeUserProfile
#[derive(Debug, PartialEq)]
pub enum DescribeUserProfileError {
    /// <p>The user profile was not found.</p>
    UserProfileNotFound(String),
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

impl DescribeUserProfileError {
    pub fn from_response(res: BufferedHttpResponse) -> DescribeUserProfileError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "UserProfileNotFoundException" => {
                    return DescribeUserProfileError::UserProfileNotFound(String::from(
                        error_message,
                    ))
                }
                "ValidationException" => {
                    return DescribeUserProfileError::Validation(error_message.to_string())
                }
                _ => {}
            }
        }
        return DescribeUserProfileError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for DescribeUserProfileError {
    fn from(err: serde_json::error::Error) -> DescribeUserProfileError {
        DescribeUserProfileError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for DescribeUserProfileError {
    fn from(err: CredentialsError) -> DescribeUserProfileError {
        DescribeUserProfileError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DescribeUserProfileError {
    fn from(err: HttpDispatchError) -> DescribeUserProfileError {
        DescribeUserProfileError::HttpDispatch(err)
    }
}
impl From<io::Error> for DescribeUserProfileError {
    fn from(err: io::Error) -> DescribeUserProfileError {
        DescribeUserProfileError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DescribeUserProfileError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeUserProfileError {
    fn description(&self) -> &str {
        match *self {
            DescribeUserProfileError::UserProfileNotFound(ref cause) => cause,
            DescribeUserProfileError::Validation(ref cause) => cause,
            DescribeUserProfileError::Credentials(ref err) => err.description(),
            DescribeUserProfileError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DescribeUserProfileError::ParseError(ref cause) => cause,
            DescribeUserProfileError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by DisassociateTeamMember
#[derive(Debug, PartialEq)]
pub enum DisassociateTeamMemberError {
    /// <p>Another modification is being made. That modification must complete before you can make your change.</p>
    ConcurrentModification(String),
    /// <p>The service role is not valid.</p>
    InvalidServiceRole(String),
    /// <p>The specified AWS CodeStar project was not found.</p>
    ProjectNotFound(String),
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

impl DisassociateTeamMemberError {
    pub fn from_response(res: BufferedHttpResponse) -> DisassociateTeamMemberError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "ConcurrentModificationException" => {
                    return DisassociateTeamMemberError::ConcurrentModification(String::from(
                        error_message,
                    ))
                }
                "InvalidServiceRoleException" => {
                    return DisassociateTeamMemberError::InvalidServiceRole(String::from(
                        error_message,
                    ))
                }
                "ProjectNotFoundException" => {
                    return DisassociateTeamMemberError::ProjectNotFound(String::from(error_message))
                }
                "ValidationException" => {
                    return DisassociateTeamMemberError::Validation(error_message.to_string())
                }
                _ => {}
            }
        }
        return DisassociateTeamMemberError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for DisassociateTeamMemberError {
    fn from(err: serde_json::error::Error) -> DisassociateTeamMemberError {
        DisassociateTeamMemberError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for DisassociateTeamMemberError {
    fn from(err: CredentialsError) -> DisassociateTeamMemberError {
        DisassociateTeamMemberError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DisassociateTeamMemberError {
    fn from(err: HttpDispatchError) -> DisassociateTeamMemberError {
        DisassociateTeamMemberError::HttpDispatch(err)
    }
}
impl From<io::Error> for DisassociateTeamMemberError {
    fn from(err: io::Error) -> DisassociateTeamMemberError {
        DisassociateTeamMemberError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DisassociateTeamMemberError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DisassociateTeamMemberError {
    fn description(&self) -> &str {
        match *self {
            DisassociateTeamMemberError::ConcurrentModification(ref cause) => cause,
            DisassociateTeamMemberError::InvalidServiceRole(ref cause) => cause,
            DisassociateTeamMemberError::ProjectNotFound(ref cause) => cause,
            DisassociateTeamMemberError::Validation(ref cause) => cause,
            DisassociateTeamMemberError::Credentials(ref err) => err.description(),
            DisassociateTeamMemberError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DisassociateTeamMemberError::ParseError(ref cause) => cause,
            DisassociateTeamMemberError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by ListProjects
#[derive(Debug, PartialEq)]
pub enum ListProjectsError {
    /// <p>The next token is not valid.</p>
    InvalidNextToken(String),
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

impl ListProjectsError {
    pub fn from_response(res: BufferedHttpResponse) -> ListProjectsError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "InvalidNextTokenException" => {
                    return ListProjectsError::InvalidNextToken(String::from(error_message))
                }
                "ValidationException" => {
                    return ListProjectsError::Validation(error_message.to_string())
                }
                _ => {}
            }
        }
        return ListProjectsError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for ListProjectsError {
    fn from(err: serde_json::error::Error) -> ListProjectsError {
        ListProjectsError::ParseError(err.description().to_string())
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
            ListProjectsError::InvalidNextToken(ref cause) => cause,
            ListProjectsError::Validation(ref cause) => cause,
            ListProjectsError::Credentials(ref err) => err.description(),
            ListProjectsError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            ListProjectsError::ParseError(ref cause) => cause,
            ListProjectsError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by ListResources
#[derive(Debug, PartialEq)]
pub enum ListResourcesError {
    /// <p>The next token is not valid.</p>
    InvalidNextToken(String),
    /// <p>The specified AWS CodeStar project was not found.</p>
    ProjectNotFound(String),
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

impl ListResourcesError {
    pub fn from_response(res: BufferedHttpResponse) -> ListResourcesError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "InvalidNextTokenException" => {
                    return ListResourcesError::InvalidNextToken(String::from(error_message))
                }
                "ProjectNotFoundException" => {
                    return ListResourcesError::ProjectNotFound(String::from(error_message))
                }
                "ValidationException" => {
                    return ListResourcesError::Validation(error_message.to_string())
                }
                _ => {}
            }
        }
        return ListResourcesError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for ListResourcesError {
    fn from(err: serde_json::error::Error) -> ListResourcesError {
        ListResourcesError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for ListResourcesError {
    fn from(err: CredentialsError) -> ListResourcesError {
        ListResourcesError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ListResourcesError {
    fn from(err: HttpDispatchError) -> ListResourcesError {
        ListResourcesError::HttpDispatch(err)
    }
}
impl From<io::Error> for ListResourcesError {
    fn from(err: io::Error) -> ListResourcesError {
        ListResourcesError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for ListResourcesError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListResourcesError {
    fn description(&self) -> &str {
        match *self {
            ListResourcesError::InvalidNextToken(ref cause) => cause,
            ListResourcesError::ProjectNotFound(ref cause) => cause,
            ListResourcesError::Validation(ref cause) => cause,
            ListResourcesError::Credentials(ref err) => err.description(),
            ListResourcesError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            ListResourcesError::ParseError(ref cause) => cause,
            ListResourcesError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by ListTagsForProject
#[derive(Debug, PartialEq)]
pub enum ListTagsForProjectError {
    /// <p>The next token is not valid.</p>
    InvalidNextToken(String),
    /// <p>The specified AWS CodeStar project was not found.</p>
    ProjectNotFound(String),
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

impl ListTagsForProjectError {
    pub fn from_response(res: BufferedHttpResponse) -> ListTagsForProjectError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "InvalidNextTokenException" => {
                    return ListTagsForProjectError::InvalidNextToken(String::from(error_message))
                }
                "ProjectNotFoundException" => {
                    return ListTagsForProjectError::ProjectNotFound(String::from(error_message))
                }
                "ValidationException" => {
                    return ListTagsForProjectError::Validation(error_message.to_string())
                }
                _ => {}
            }
        }
        return ListTagsForProjectError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for ListTagsForProjectError {
    fn from(err: serde_json::error::Error) -> ListTagsForProjectError {
        ListTagsForProjectError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for ListTagsForProjectError {
    fn from(err: CredentialsError) -> ListTagsForProjectError {
        ListTagsForProjectError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ListTagsForProjectError {
    fn from(err: HttpDispatchError) -> ListTagsForProjectError {
        ListTagsForProjectError::HttpDispatch(err)
    }
}
impl From<io::Error> for ListTagsForProjectError {
    fn from(err: io::Error) -> ListTagsForProjectError {
        ListTagsForProjectError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for ListTagsForProjectError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListTagsForProjectError {
    fn description(&self) -> &str {
        match *self {
            ListTagsForProjectError::InvalidNextToken(ref cause) => cause,
            ListTagsForProjectError::ProjectNotFound(ref cause) => cause,
            ListTagsForProjectError::Validation(ref cause) => cause,
            ListTagsForProjectError::Credentials(ref err) => err.description(),
            ListTagsForProjectError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            ListTagsForProjectError::ParseError(ref cause) => cause,
            ListTagsForProjectError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by ListTeamMembers
#[derive(Debug, PartialEq)]
pub enum ListTeamMembersError {
    /// <p>The next token is not valid.</p>
    InvalidNextToken(String),
    /// <p>The specified AWS CodeStar project was not found.</p>
    ProjectNotFound(String),
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

impl ListTeamMembersError {
    pub fn from_response(res: BufferedHttpResponse) -> ListTeamMembersError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "InvalidNextTokenException" => {
                    return ListTeamMembersError::InvalidNextToken(String::from(error_message))
                }
                "ProjectNotFoundException" => {
                    return ListTeamMembersError::ProjectNotFound(String::from(error_message))
                }
                "ValidationException" => {
                    return ListTeamMembersError::Validation(error_message.to_string())
                }
                _ => {}
            }
        }
        return ListTeamMembersError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for ListTeamMembersError {
    fn from(err: serde_json::error::Error) -> ListTeamMembersError {
        ListTeamMembersError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for ListTeamMembersError {
    fn from(err: CredentialsError) -> ListTeamMembersError {
        ListTeamMembersError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ListTeamMembersError {
    fn from(err: HttpDispatchError) -> ListTeamMembersError {
        ListTeamMembersError::HttpDispatch(err)
    }
}
impl From<io::Error> for ListTeamMembersError {
    fn from(err: io::Error) -> ListTeamMembersError {
        ListTeamMembersError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for ListTeamMembersError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListTeamMembersError {
    fn description(&self) -> &str {
        match *self {
            ListTeamMembersError::InvalidNextToken(ref cause) => cause,
            ListTeamMembersError::ProjectNotFound(ref cause) => cause,
            ListTeamMembersError::Validation(ref cause) => cause,
            ListTeamMembersError::Credentials(ref err) => err.description(),
            ListTeamMembersError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            ListTeamMembersError::ParseError(ref cause) => cause,
            ListTeamMembersError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by ListUserProfiles
#[derive(Debug, PartialEq)]
pub enum ListUserProfilesError {
    /// <p>The next token is not valid.</p>
    InvalidNextToken(String),
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

impl ListUserProfilesError {
    pub fn from_response(res: BufferedHttpResponse) -> ListUserProfilesError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "InvalidNextTokenException" => {
                    return ListUserProfilesError::InvalidNextToken(String::from(error_message))
                }
                "ValidationException" => {
                    return ListUserProfilesError::Validation(error_message.to_string())
                }
                _ => {}
            }
        }
        return ListUserProfilesError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for ListUserProfilesError {
    fn from(err: serde_json::error::Error) -> ListUserProfilesError {
        ListUserProfilesError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for ListUserProfilesError {
    fn from(err: CredentialsError) -> ListUserProfilesError {
        ListUserProfilesError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ListUserProfilesError {
    fn from(err: HttpDispatchError) -> ListUserProfilesError {
        ListUserProfilesError::HttpDispatch(err)
    }
}
impl From<io::Error> for ListUserProfilesError {
    fn from(err: io::Error) -> ListUserProfilesError {
        ListUserProfilesError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for ListUserProfilesError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListUserProfilesError {
    fn description(&self) -> &str {
        match *self {
            ListUserProfilesError::InvalidNextToken(ref cause) => cause,
            ListUserProfilesError::Validation(ref cause) => cause,
            ListUserProfilesError::Credentials(ref err) => err.description(),
            ListUserProfilesError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            ListUserProfilesError::ParseError(ref cause) => cause,
            ListUserProfilesError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by TagProject
#[derive(Debug, PartialEq)]
pub enum TagProjectError {
    /// <p>Another modification is being made. That modification must complete before you can make your change.</p>
    ConcurrentModification(String),
    /// <p>A resource limit has been exceeded.</p>
    LimitExceeded(String),
    /// <p>The specified AWS CodeStar project was not found.</p>
    ProjectNotFound(String),
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

impl TagProjectError {
    pub fn from_response(res: BufferedHttpResponse) -> TagProjectError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "ConcurrentModificationException" => {
                    return TagProjectError::ConcurrentModification(String::from(error_message))
                }
                "LimitExceededException" => {
                    return TagProjectError::LimitExceeded(String::from(error_message))
                }
                "ProjectNotFoundException" => {
                    return TagProjectError::ProjectNotFound(String::from(error_message))
                }
                "ValidationException" => {
                    return TagProjectError::Validation(error_message.to_string())
                }
                _ => {}
            }
        }
        return TagProjectError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for TagProjectError {
    fn from(err: serde_json::error::Error) -> TagProjectError {
        TagProjectError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for TagProjectError {
    fn from(err: CredentialsError) -> TagProjectError {
        TagProjectError::Credentials(err)
    }
}
impl From<HttpDispatchError> for TagProjectError {
    fn from(err: HttpDispatchError) -> TagProjectError {
        TagProjectError::HttpDispatch(err)
    }
}
impl From<io::Error> for TagProjectError {
    fn from(err: io::Error) -> TagProjectError {
        TagProjectError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for TagProjectError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for TagProjectError {
    fn description(&self) -> &str {
        match *self {
            TagProjectError::ConcurrentModification(ref cause) => cause,
            TagProjectError::LimitExceeded(ref cause) => cause,
            TagProjectError::ProjectNotFound(ref cause) => cause,
            TagProjectError::Validation(ref cause) => cause,
            TagProjectError::Credentials(ref err) => err.description(),
            TagProjectError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            TagProjectError::ParseError(ref cause) => cause,
            TagProjectError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by UntagProject
#[derive(Debug, PartialEq)]
pub enum UntagProjectError {
    /// <p>Another modification is being made. That modification must complete before you can make your change.</p>
    ConcurrentModification(String),
    /// <p>A resource limit has been exceeded.</p>
    LimitExceeded(String),
    /// <p>The specified AWS CodeStar project was not found.</p>
    ProjectNotFound(String),
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

impl UntagProjectError {
    pub fn from_response(res: BufferedHttpResponse) -> UntagProjectError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "ConcurrentModificationException" => {
                    return UntagProjectError::ConcurrentModification(String::from(error_message))
                }
                "LimitExceededException" => {
                    return UntagProjectError::LimitExceeded(String::from(error_message))
                }
                "ProjectNotFoundException" => {
                    return UntagProjectError::ProjectNotFound(String::from(error_message))
                }
                "ValidationException" => {
                    return UntagProjectError::Validation(error_message.to_string())
                }
                _ => {}
            }
        }
        return UntagProjectError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for UntagProjectError {
    fn from(err: serde_json::error::Error) -> UntagProjectError {
        UntagProjectError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for UntagProjectError {
    fn from(err: CredentialsError) -> UntagProjectError {
        UntagProjectError::Credentials(err)
    }
}
impl From<HttpDispatchError> for UntagProjectError {
    fn from(err: HttpDispatchError) -> UntagProjectError {
        UntagProjectError::HttpDispatch(err)
    }
}
impl From<io::Error> for UntagProjectError {
    fn from(err: io::Error) -> UntagProjectError {
        UntagProjectError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for UntagProjectError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for UntagProjectError {
    fn description(&self) -> &str {
        match *self {
            UntagProjectError::ConcurrentModification(ref cause) => cause,
            UntagProjectError::LimitExceeded(ref cause) => cause,
            UntagProjectError::ProjectNotFound(ref cause) => cause,
            UntagProjectError::Validation(ref cause) => cause,
            UntagProjectError::Credentials(ref err) => err.description(),
            UntagProjectError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            UntagProjectError::ParseError(ref cause) => cause,
            UntagProjectError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by UpdateProject
#[derive(Debug, PartialEq)]
pub enum UpdateProjectError {
    /// <p>The specified AWS CodeStar project was not found.</p>
    ProjectNotFound(String),
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

impl UpdateProjectError {
    pub fn from_response(res: BufferedHttpResponse) -> UpdateProjectError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "ProjectNotFoundException" => {
                    return UpdateProjectError::ProjectNotFound(String::from(error_message))
                }
                "ValidationException" => {
                    return UpdateProjectError::Validation(error_message.to_string())
                }
                _ => {}
            }
        }
        return UpdateProjectError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for UpdateProjectError {
    fn from(err: serde_json::error::Error) -> UpdateProjectError {
        UpdateProjectError::ParseError(err.description().to_string())
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
            UpdateProjectError::ProjectNotFound(ref cause) => cause,
            UpdateProjectError::Validation(ref cause) => cause,
            UpdateProjectError::Credentials(ref err) => err.description(),
            UpdateProjectError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            UpdateProjectError::ParseError(ref cause) => cause,
            UpdateProjectError::Unknown(_) => "unknown error",
        }
    }
}
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

impl UpdateTeamMemberError {
    pub fn from_response(res: BufferedHttpResponse) -> UpdateTeamMemberError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "ConcurrentModificationException" => {
                    return UpdateTeamMemberError::ConcurrentModification(String::from(
                        error_message,
                    ))
                }
                "InvalidServiceRoleException" => {
                    return UpdateTeamMemberError::InvalidServiceRole(String::from(error_message))
                }
                "LimitExceededException" => {
                    return UpdateTeamMemberError::LimitExceeded(String::from(error_message))
                }
                "ProjectConfigurationException" => {
                    return UpdateTeamMemberError::ProjectConfiguration(String::from(error_message))
                }
                "ProjectNotFoundException" => {
                    return UpdateTeamMemberError::ProjectNotFound(String::from(error_message))
                }
                "TeamMemberNotFoundException" => {
                    return UpdateTeamMemberError::TeamMemberNotFound(String::from(error_message))
                }
                "ValidationException" => {
                    return UpdateTeamMemberError::Validation(error_message.to_string())
                }
                _ => {}
            }
        }
        return UpdateTeamMemberError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for UpdateTeamMemberError {
    fn from(err: serde_json::error::Error) -> UpdateTeamMemberError {
        UpdateTeamMemberError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for UpdateTeamMemberError {
    fn from(err: CredentialsError) -> UpdateTeamMemberError {
        UpdateTeamMemberError::Credentials(err)
    }
}
impl From<HttpDispatchError> for UpdateTeamMemberError {
    fn from(err: HttpDispatchError) -> UpdateTeamMemberError {
        UpdateTeamMemberError::HttpDispatch(err)
    }
}
impl From<io::Error> for UpdateTeamMemberError {
    fn from(err: io::Error) -> UpdateTeamMemberError {
        UpdateTeamMemberError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for UpdateTeamMemberError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for UpdateTeamMemberError {
    fn description(&self) -> &str {
        match *self {
            UpdateTeamMemberError::ConcurrentModification(ref cause) => cause,
            UpdateTeamMemberError::InvalidServiceRole(ref cause) => cause,
            UpdateTeamMemberError::LimitExceeded(ref cause) => cause,
            UpdateTeamMemberError::ProjectConfiguration(ref cause) => cause,
            UpdateTeamMemberError::ProjectNotFound(ref cause) => cause,
            UpdateTeamMemberError::TeamMemberNotFound(ref cause) => cause,
            UpdateTeamMemberError::Validation(ref cause) => cause,
            UpdateTeamMemberError::Credentials(ref err) => err.description(),
            UpdateTeamMemberError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            UpdateTeamMemberError::ParseError(ref cause) => cause,
            UpdateTeamMemberError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by UpdateUserProfile
#[derive(Debug, PartialEq)]
pub enum UpdateUserProfileError {
    /// <p>The user profile was not found.</p>
    UserProfileNotFound(String),
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

impl UpdateUserProfileError {
    pub fn from_response(res: BufferedHttpResponse) -> UpdateUserProfileError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "UserProfileNotFoundException" => {
                    return UpdateUserProfileError::UserProfileNotFound(String::from(error_message))
                }
                "ValidationException" => {
                    return UpdateUserProfileError::Validation(error_message.to_string())
                }
                _ => {}
            }
        }
        return UpdateUserProfileError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for UpdateUserProfileError {
    fn from(err: serde_json::error::Error) -> UpdateUserProfileError {
        UpdateUserProfileError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for UpdateUserProfileError {
    fn from(err: CredentialsError) -> UpdateUserProfileError {
        UpdateUserProfileError::Credentials(err)
    }
}
impl From<HttpDispatchError> for UpdateUserProfileError {
    fn from(err: HttpDispatchError) -> UpdateUserProfileError {
        UpdateUserProfileError::HttpDispatch(err)
    }
}
impl From<io::Error> for UpdateUserProfileError {
    fn from(err: io::Error) -> UpdateUserProfileError {
        UpdateUserProfileError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for UpdateUserProfileError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for UpdateUserProfileError {
    fn description(&self) -> &str {
        match *self {
            UpdateUserProfileError::UserProfileNotFound(ref cause) => cause,
            UpdateUserProfileError::Validation(ref cause) => cause,
            UpdateUserProfileError::Credentials(ref err) => err.description(),
            UpdateUserProfileError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            UpdateUserProfileError::ParseError(ref cause) => cause,
            UpdateUserProfileError::Unknown(_) => "unknown error",
        }
    }
}
/// Trait representing the capabilities of the CodeStar API. CodeStar clients implement this trait.
pub trait CodeStar {
    /// <p>Adds an IAM user to the team for an AWS CodeStar project.</p>
    fn associate_team_member(
        &self,
        input: AssociateTeamMemberRequest,
    ) -> RusotoFuture<AssociateTeamMemberResult, AssociateTeamMemberError>;

    /// <p>Reserved for future use. To create a project, use the AWS CodeStar console.</p>
    fn create_project(
        &self,
        input: CreateProjectRequest,
    ) -> RusotoFuture<CreateProjectResult, CreateProjectError>;

    /// <p>Creates a profile for a user that includes user preferences, such as the display name and email address assocciated with the user, in AWS CodeStar. The user profile is not project-specific. Information in the user profile is displayed wherever the user's information appears to other users in AWS CodeStar.</p>
    fn create_user_profile(
        &self,
        input: CreateUserProfileRequest,
    ) -> RusotoFuture<CreateUserProfileResult, CreateUserProfileError>;

    /// <p>Deletes a project, including project resources. Does not delete users associated with the project, but does delete the IAM roles that allowed access to the project.</p>
    fn delete_project(
        &self,
        input: DeleteProjectRequest,
    ) -> RusotoFuture<DeleteProjectResult, DeleteProjectError>;

    /// <p>Deletes a user profile in AWS CodeStar, including all personal preference data associated with that profile, such as display name and email address. It does not delete the history of that user, for example the history of commits made by that user.</p>
    fn delete_user_profile(
        &self,
        input: DeleteUserProfileRequest,
    ) -> RusotoFuture<DeleteUserProfileResult, DeleteUserProfileError>;

    /// <p>Describes a project and its resources.</p>
    fn describe_project(
        &self,
        input: DescribeProjectRequest,
    ) -> RusotoFuture<DescribeProjectResult, DescribeProjectError>;

    /// <p>Describes a user in AWS CodeStar and the user attributes across all projects.</p>
    fn describe_user_profile(
        &self,
        input: DescribeUserProfileRequest,
    ) -> RusotoFuture<DescribeUserProfileResult, DescribeUserProfileError>;

    /// <p>Removes a user from a project. Removing a user from a project also removes the IAM policies from that user that allowed access to the project and its resources. Disassociating a team member does not remove that user's profile from AWS CodeStar. It does not remove the user from IAM.</p>
    fn disassociate_team_member(
        &self,
        input: DisassociateTeamMemberRequest,
    ) -> RusotoFuture<DisassociateTeamMemberResult, DisassociateTeamMemberError>;

    /// <p>Lists all projects in AWS CodeStar associated with your AWS account.</p>
    fn list_projects(
        &self,
        input: ListProjectsRequest,
    ) -> RusotoFuture<ListProjectsResult, ListProjectsError>;

    /// <p>Lists resources associated with a project in AWS CodeStar.</p>
    fn list_resources(
        &self,
        input: ListResourcesRequest,
    ) -> RusotoFuture<ListResourcesResult, ListResourcesError>;

    /// <p>Gets the tags for a project.</p>
    fn list_tags_for_project(
        &self,
        input: ListTagsForProjectRequest,
    ) -> RusotoFuture<ListTagsForProjectResult, ListTagsForProjectError>;

    /// <p>Lists all team members associated with a project.</p>
    fn list_team_members(
        &self,
        input: ListTeamMembersRequest,
    ) -> RusotoFuture<ListTeamMembersResult, ListTeamMembersError>;

    /// <p>Lists all the user profiles configured for your AWS account in AWS CodeStar.</p>
    fn list_user_profiles(
        &self,
        input: ListUserProfilesRequest,
    ) -> RusotoFuture<ListUserProfilesResult, ListUserProfilesError>;

    /// <p>Adds tags to a project.</p>
    fn tag_project(
        &self,
        input: TagProjectRequest,
    ) -> RusotoFuture<TagProjectResult, TagProjectError>;

    /// <p>Removes tags from a project.</p>
    fn untag_project(
        &self,
        input: UntagProjectRequest,
    ) -> RusotoFuture<UntagProjectResult, UntagProjectError>;

    /// <p>Updates a project in AWS CodeStar.</p>
    fn update_project(
        &self,
        input: UpdateProjectRequest,
    ) -> RusotoFuture<UpdateProjectResult, UpdateProjectError>;

    /// <p>Updates a team member's attributes in an AWS CodeStar project. For example, you can change a team member's role in the project, or change whether they have remote access to project resources.</p>
    fn update_team_member(
        &self,
        input: UpdateTeamMemberRequest,
    ) -> RusotoFuture<UpdateTeamMemberResult, UpdateTeamMemberError>;

    /// <p>Updates a user's profile in AWS CodeStar. The user profile is not project-specific. Information in the user profile is displayed wherever the user's information appears to other users in AWS CodeStar. </p>
    fn update_user_profile(
        &self,
        input: UpdateUserProfileRequest,
    ) -> RusotoFuture<UpdateUserProfileResult, UpdateUserProfileError>;
}
/// A client for the CodeStar API.
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
            region: region,
        }
    }

    pub fn new_with<P, D>(
        request_dispatcher: D,
        credentials_provider: P,
        region: region::Region,
    ) -> CodeStarClient
    where
        P: ProvideAwsCredentials + Send + Sync + 'static,
        P::Future: Send,
        D: DispatchSignedRequest + Send + Sync + 'static,
        D::Future: Send,
    {
        CodeStarClient {
            client: Client::new_with(credentials_provider, request_dispatcher),
            region: region,
        }
    }
}

impl CodeStar for CodeStarClient {
    /// <p>Adds an IAM user to the team for an AWS CodeStar project.</p>
    fn associate_team_member(
        &self,
        input: AssociateTeamMemberRequest,
    ) -> RusotoFuture<AssociateTeamMemberResult, AssociateTeamMemberError> {
        let mut request = SignedRequest::new("POST", "codestar", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "CodeStar_20170419.AssociateTeamMember");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<AssociateTeamMemberResult>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(AssociateTeamMemberError::from_response(response))
                    }),
                )
            }
        })
    }

    /// <p>Reserved for future use. To create a project, use the AWS CodeStar console.</p>
    fn create_project(
        &self,
        input: CreateProjectRequest,
    ) -> RusotoFuture<CreateProjectResult, CreateProjectError> {
        let mut request = SignedRequest::new("POST", "codestar", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "CodeStar_20170419.CreateProject");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<CreateProjectResult>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(CreateProjectError::from_response(response))),
                )
            }
        })
    }

    /// <p>Creates a profile for a user that includes user preferences, such as the display name and email address assocciated with the user, in AWS CodeStar. The user profile is not project-specific. Information in the user profile is displayed wherever the user's information appears to other users in AWS CodeStar.</p>
    fn create_user_profile(
        &self,
        input: CreateUserProfileRequest,
    ) -> RusotoFuture<CreateUserProfileResult, CreateUserProfileError> {
        let mut request = SignedRequest::new("POST", "codestar", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "CodeStar_20170419.CreateUserProfile");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<CreateUserProfileResult>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(CreateUserProfileError::from_response(response))),
                )
            }
        })
    }

    /// <p>Deletes a project, including project resources. Does not delete users associated with the project, but does delete the IAM roles that allowed access to the project.</p>
    fn delete_project(
        &self,
        input: DeleteProjectRequest,
    ) -> RusotoFuture<DeleteProjectResult, DeleteProjectError> {
        let mut request = SignedRequest::new("POST", "codestar", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "CodeStar_20170419.DeleteProject");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<DeleteProjectResult>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(DeleteProjectError::from_response(response))),
                )
            }
        })
    }

    /// <p>Deletes a user profile in AWS CodeStar, including all personal preference data associated with that profile, such as display name and email address. It does not delete the history of that user, for example the history of commits made by that user.</p>
    fn delete_user_profile(
        &self,
        input: DeleteUserProfileRequest,
    ) -> RusotoFuture<DeleteUserProfileResult, DeleteUserProfileError> {
        let mut request = SignedRequest::new("POST", "codestar", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "CodeStar_20170419.DeleteUserProfile");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<DeleteUserProfileResult>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(DeleteUserProfileError::from_response(response))),
                )
            }
        })
    }

    /// <p>Describes a project and its resources.</p>
    fn describe_project(
        &self,
        input: DescribeProjectRequest,
    ) -> RusotoFuture<DescribeProjectResult, DescribeProjectError> {
        let mut request = SignedRequest::new("POST", "codestar", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "CodeStar_20170419.DescribeProject");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<DescribeProjectResult>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(DescribeProjectError::from_response(response))),
                )
            }
        })
    }

    /// <p>Describes a user in AWS CodeStar and the user attributes across all projects.</p>
    fn describe_user_profile(
        &self,
        input: DescribeUserProfileRequest,
    ) -> RusotoFuture<DescribeUserProfileResult, DescribeUserProfileError> {
        let mut request = SignedRequest::new("POST", "codestar", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "CodeStar_20170419.DescribeUserProfile");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<DescribeUserProfileResult>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(DescribeUserProfileError::from_response(response))
                    }),
                )
            }
        })
    }

    /// <p>Removes a user from a project. Removing a user from a project also removes the IAM policies from that user that allowed access to the project and its resources. Disassociating a team member does not remove that user's profile from AWS CodeStar. It does not remove the user from IAM.</p>
    fn disassociate_team_member(
        &self,
        input: DisassociateTeamMemberRequest,
    ) -> RusotoFuture<DisassociateTeamMemberResult, DisassociateTeamMemberError> {
        let mut request = SignedRequest::new("POST", "codestar", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "CodeStar_20170419.DisassociateTeamMember");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<DisassociateTeamMemberResult>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(DisassociateTeamMemberError::from_response(response))
                    }),
                )
            }
        })
    }

    /// <p>Lists all projects in AWS CodeStar associated with your AWS account.</p>
    fn list_projects(
        &self,
        input: ListProjectsRequest,
    ) -> RusotoFuture<ListProjectsResult, ListProjectsError> {
        let mut request = SignedRequest::new("POST", "codestar", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "CodeStar_20170419.ListProjects");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<ListProjectsResult>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(ListProjectsError::from_response(response))),
                )
            }
        })
    }

    /// <p>Lists resources associated with a project in AWS CodeStar.</p>
    fn list_resources(
        &self,
        input: ListResourcesRequest,
    ) -> RusotoFuture<ListResourcesResult, ListResourcesError> {
        let mut request = SignedRequest::new("POST", "codestar", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "CodeStar_20170419.ListResources");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<ListResourcesResult>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(ListResourcesError::from_response(response))),
                )
            }
        })
    }

    /// <p>Gets the tags for a project.</p>
    fn list_tags_for_project(
        &self,
        input: ListTagsForProjectRequest,
    ) -> RusotoFuture<ListTagsForProjectResult, ListTagsForProjectError> {
        let mut request = SignedRequest::new("POST", "codestar", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "CodeStar_20170419.ListTagsForProject");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<ListTagsForProjectResult>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(ListTagsForProjectError::from_response(response))),
                )
            }
        })
    }

    /// <p>Lists all team members associated with a project.</p>
    fn list_team_members(
        &self,
        input: ListTeamMembersRequest,
    ) -> RusotoFuture<ListTeamMembersResult, ListTeamMembersError> {
        let mut request = SignedRequest::new("POST", "codestar", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "CodeStar_20170419.ListTeamMembers");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<ListTeamMembersResult>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(ListTeamMembersError::from_response(response))),
                )
            }
        })
    }

    /// <p>Lists all the user profiles configured for your AWS account in AWS CodeStar.</p>
    fn list_user_profiles(
        &self,
        input: ListUserProfilesRequest,
    ) -> RusotoFuture<ListUserProfilesResult, ListUserProfilesError> {
        let mut request = SignedRequest::new("POST", "codestar", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "CodeStar_20170419.ListUserProfiles");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<ListUserProfilesResult>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(ListUserProfilesError::from_response(response))),
                )
            }
        })
    }

    /// <p>Adds tags to a project.</p>
    fn tag_project(
        &self,
        input: TagProjectRequest,
    ) -> RusotoFuture<TagProjectResult, TagProjectError> {
        let mut request = SignedRequest::new("POST", "codestar", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "CodeStar_20170419.TagProject");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<TagProjectResult>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(TagProjectError::from_response(response))),
                )
            }
        })
    }

    /// <p>Removes tags from a project.</p>
    fn untag_project(
        &self,
        input: UntagProjectRequest,
    ) -> RusotoFuture<UntagProjectResult, UntagProjectError> {
        let mut request = SignedRequest::new("POST", "codestar", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "CodeStar_20170419.UntagProject");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<UntagProjectResult>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(UntagProjectError::from_response(response))),
                )
            }
        })
    }

    /// <p>Updates a project in AWS CodeStar.</p>
    fn update_project(
        &self,
        input: UpdateProjectRequest,
    ) -> RusotoFuture<UpdateProjectResult, UpdateProjectError> {
        let mut request = SignedRequest::new("POST", "codestar", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "CodeStar_20170419.UpdateProject");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<UpdateProjectResult>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(UpdateProjectError::from_response(response))),
                )
            }
        })
    }

    /// <p>Updates a team member's attributes in an AWS CodeStar project. For example, you can change a team member's role in the project, or change whether they have remote access to project resources.</p>
    fn update_team_member(
        &self,
        input: UpdateTeamMemberRequest,
    ) -> RusotoFuture<UpdateTeamMemberResult, UpdateTeamMemberError> {
        let mut request = SignedRequest::new("POST", "codestar", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "CodeStar_20170419.UpdateTeamMember");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<UpdateTeamMemberResult>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(UpdateTeamMemberError::from_response(response))),
                )
            }
        })
    }

    /// <p>Updates a user's profile in AWS CodeStar. The user profile is not project-specific. Information in the user profile is displayed wherever the user's information appears to other users in AWS CodeStar. </p>
    fn update_user_profile(
        &self,
        input: UpdateUserProfileRequest,
    ) -> RusotoFuture<UpdateUserProfileResult, UpdateUserProfileError> {
        let mut request = SignedRequest::new("POST", "codestar", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "CodeStar_20170419.UpdateUserProfile");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<UpdateUserProfileResult>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(UpdateUserProfileError::from_response(response))),
                )
            }
        })
    }
}

#[cfg(test)]
mod protocol_tests {}
