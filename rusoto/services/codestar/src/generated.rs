
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

use std::fmt;
use std::error::Error;
use std::io;
use std::io::Read;

use rusoto_core::region;
use rusoto_core::request::{DispatchSignedRequest, HttpDispatchError};
use rusoto_core::credential::{CredentialsError, ProvideAwsCredentials};

use serde_json;
use rusoto_core::signature::SignedRequest;
use serde_json::Value as SerdeJsonValue;
use serde_json::from_str;
#[derive(Default,Debug,Clone,Serialize)]
pub struct AssociateTeamMemberRequest {
    #[doc="<p>A user- or system-generated token that identifies the entity that requested the team member association to the project. This token can be used to repeat the request. </p>"]
    #[serde(rename="clientRequestToken")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub client_request_token: Option<String>,
    #[doc="<p>The ID of the project to which you will add the IAM user.</p>"]
    #[serde(rename="projectId")]
    pub project_id: String,
    #[doc="<p>The AWS CodeStar project role that will apply to this user. This role determines what actions a user can take in an AWS CodeStar project.</p>"]
    #[serde(rename="projectRole")]
    pub project_role: String,
    #[doc="<p>Whether the team member is allowed to use an SSH public/private key pair to remotely access project resources, for example Amazon EC2 instances.</p>"]
    #[serde(rename="remoteAccessAllowed")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub remote_access_allowed: Option<bool>,
    #[doc="<p>The Amazon Resource Name (ARN) for the IAM user you want to add to the DevHub project.</p>"]
    #[serde(rename="userArn")]
    pub user_arn: String,
}

#[derive(Default,Debug,Clone,Deserialize)]
pub struct AssociateTeamMemberResult {
    #[doc="<p>The user- or system-generated token from the initial request that can be used to repeat the request. </p>"]
    #[serde(rename="clientRequestToken")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub client_request_token: Option<String>,
}

#[derive(Default,Debug,Clone,Serialize)]
pub struct CreateProjectRequest {
    #[doc="<p>Reserved for future use.</p>"]
    #[serde(rename="clientRequestToken")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub client_request_token: Option<String>,
    #[doc="<p>Reserved for future use.</p>"]
    #[serde(rename="description")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub description: Option<String>,
    #[doc="<p>Reserved for future use.</p>"]
    #[serde(rename="id")]
    pub id: String,
    #[doc="<p>Reserved for future use.</p>"]
    #[serde(rename="name")]
    pub name: String,
}

#[derive(Default,Debug,Clone,Deserialize)]
pub struct CreateProjectResult {
    #[doc="<p>Reserved for future use.</p>"]
    #[serde(rename="arn")]
    pub arn: String,
    #[doc="<p>Reserved for future use.</p>"]
    #[serde(rename="clientRequestToken")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub client_request_token: Option<String>,
    #[doc="<p>Reserved for future use.</p>"]
    #[serde(rename="id")]
    pub id: String,
    #[doc="<p>Reserved for future use.</p>"]
    #[serde(rename="projectTemplateId")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub project_template_id: Option<String>,
}

#[derive(Default,Debug,Clone,Serialize)]
pub struct CreateUserProfileRequest {
    #[doc="<p>The name that will be displayed as the friendly name for the user in AWS CodeStar. </p>"]
    #[serde(rename="displayName")]
    pub display_name: String,
    #[doc="<p>The email address that will be displayed as part of the user's profile in AWS CodeStar.</p>"]
    #[serde(rename="emailAddress")]
    pub email_address: String,
    #[doc="<p>The SSH public key associated with the user in AWS CodeStar. If a project owner allows the user remote access to project resources, this public key will be used along with the user's private key for SSH access.</p>"]
    #[serde(rename="sshPublicKey")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub ssh_public_key: Option<String>,
    #[doc="<p>The Amazon Resource Name (ARN) of the user in IAM.</p>"]
    #[serde(rename="userArn")]
    pub user_arn: String,
}

#[derive(Default,Debug,Clone,Deserialize)]
pub struct CreateUserProfileResult {
    #[doc="<p>The date the user profile was created, in timestamp format.</p>"]
    #[serde(rename="createdTimestamp")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub created_timestamp: Option<f64>,
    #[doc="<p>The name that is displayed as the friendly name for the user in AWS CodeStar.</p>"]
    #[serde(rename="displayName")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub display_name: Option<String>,
    #[doc="<p>The email address that is displayed as part of the user's profile in AWS CodeStar.</p>"]
    #[serde(rename="emailAddress")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub email_address: Option<String>,
    #[doc="<p>The date the user profile was last modified, in timestamp format.</p>"]
    #[serde(rename="lastModifiedTimestamp")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub last_modified_timestamp: Option<f64>,
    #[doc="<p>The SSH public key associated with the user in AWS CodeStar. This is the public portion of the public/private keypair the user can use to access project resources if a project owner allows the user remote access to those resources.</p>"]
    #[serde(rename="sshPublicKey")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub ssh_public_key: Option<String>,
    #[doc="<p>The Amazon Resource Name (ARN) of the user in IAM.</p>"]
    #[serde(rename="userArn")]
    pub user_arn: String,
}

#[derive(Default,Debug,Clone,Serialize)]
pub struct DeleteProjectRequest {
    #[doc="<p>A user- or system-generated token that identifies the entity that requested project deletion. This token can be used to repeat the request. </p>"]
    #[serde(rename="clientRequestToken")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub client_request_token: Option<String>,
    #[doc="<p>Whether to send a delete request for the primary stack in AWS CloudFormation originally used to generate the project and its resources. This option will delete all AWS resources for the project (except for any buckets in Amazon S3) as well as deleting the project itself. Recommended for most use cases.</p>"]
    #[serde(rename="deleteStack")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub delete_stack: Option<bool>,
    #[doc="<p>The ID of the project to be deleted in AWS CodeStar.</p>"]
    #[serde(rename="id")]
    pub id: String,
}

#[derive(Default,Debug,Clone,Deserialize)]
pub struct DeleteProjectResult {
    #[doc="<p>The Amazon Resource Name (ARN) of the deleted project.</p>"]
    #[serde(rename="projectArn")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub project_arn: Option<String>,
    #[doc="<p>The ID of the primary stack in AWS CloudFormation that will be deleted as part of deleting the project and its resources.</p>"]
    #[serde(rename="stackId")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub stack_id: Option<String>,
}

#[derive(Default,Debug,Clone,Serialize)]
pub struct DeleteUserProfileRequest {
    #[doc="<p>The Amazon Resource Name (ARN) of the user to delete from AWS CodeStar.</p>"]
    #[serde(rename="userArn")]
    pub user_arn: String,
}

#[derive(Default,Debug,Clone,Deserialize)]
pub struct DeleteUserProfileResult {
    #[doc="<p>The Amazon Resource Name (ARN) of the user deleted from AWS CodeStar.</p>"]
    #[serde(rename="userArn")]
    pub user_arn: String,
}

#[derive(Default,Debug,Clone,Serialize)]
pub struct DescribeProjectRequest {
    #[doc="<p>The ID of the project.</p>"]
    #[serde(rename="id")]
    pub id: String,
}

#[derive(Default,Debug,Clone,Deserialize)]
pub struct DescribeProjectResult {
    #[doc="<p>The Amazon Resource Name (ARN) for the project.</p>"]
    #[serde(rename="arn")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub arn: Option<String>,
    #[doc="<p>A user- or system-generated token that identifies the entity that requested project creation. </p>"]
    #[serde(rename="clientRequestToken")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub client_request_token: Option<String>,
    #[doc="<p>The date and time the project was created, in timestamp format.</p>"]
    #[serde(rename="createdTimeStamp")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub created_time_stamp: Option<f64>,
    #[doc="<p>The description of the project, if any.</p>"]
    #[serde(rename="description")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub description: Option<String>,
    #[doc="<p>The ID of the project.</p>"]
    #[serde(rename="id")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub id: Option<String>,
    #[doc="<p>The display name for the project.</p>"]
    #[serde(rename="name")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub name: Option<String>,
    #[doc="<p>The ID for the AWS CodeStar project template used to create the project.</p>"]
    #[serde(rename="projectTemplateId")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub project_template_id: Option<String>,
    #[doc="<p>The ID of the primary stack in AWS CloudFormation used to generate resources for the project.</p>"]
    #[serde(rename="stackId")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub stack_id: Option<String>,
}

#[derive(Default,Debug,Clone,Serialize)]
pub struct DescribeUserProfileRequest {
    #[doc="<p>The Amazon Resource Name (ARN) of the user.</p>"]
    #[serde(rename="userArn")]
    pub user_arn: String,
}

#[derive(Default,Debug,Clone,Deserialize)]
pub struct DescribeUserProfileResult {
    #[doc="<p>The date and time when the user profile was created in AWS CodeStar, in timestamp format.</p>"]
    #[serde(rename="createdTimestamp")]
    pub created_timestamp: f64,
    #[doc="<p>The display name shown for the user in AWS CodeStar projects. For example, this could be set to both first and last name (\"Mary Major\") or a single name (\"Mary\"). The display name is also used to generate the initial icon associated with the user in AWS CodeStar projects. If spaces are included in the display name, the first character that appears after the space will be used as the second character in the user initial icon. The initial icon displays a maximum of two characters, so a display name with more than one space (for example \"Mary Jane Major\") would generate an initial icon using the first character and the first character after the space (\"MJ\", not \"MM\").</p>"]
    #[serde(rename="displayName")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub display_name: Option<String>,
    #[doc="<p>The email address for the user. Optional.</p>"]
    #[serde(rename="emailAddress")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub email_address: Option<String>,
    #[doc="<p>The date and time when the user profile was last modified, in timestamp format.</p>"]
    #[serde(rename="lastModifiedTimestamp")]
    pub last_modified_timestamp: f64,
    #[doc="<p>The SSH public key associated with the user. This SSH public key is associated with the user profile, and can be used in conjunction with the associated private key for access to project resources, such as Amazon EC2 instances, if a project owner grants remote access to those resources.</p>"]
    #[serde(rename="sshPublicKey")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub ssh_public_key: Option<String>,
    #[doc="<p>The Amazon Resource Name (ARN) of the user.</p>"]
    #[serde(rename="userArn")]
    pub user_arn: String,
}

#[derive(Default,Debug,Clone,Serialize)]
pub struct DisassociateTeamMemberRequest {
    #[doc="<p>The ID of the AWS CodeStar project from which you want to remove a team member.</p>"]
    #[serde(rename="projectId")]
    pub project_id: String,
    #[doc="<p>The Amazon Resource Name (ARN) of the IAM user or group whom you want to remove from the project.</p>"]
    #[serde(rename="userArn")]
    pub user_arn: String,
}

#[derive(Default,Debug,Clone,Deserialize)]
pub struct DisassociateTeamMemberResult;

#[derive(Default,Debug,Clone,Serialize)]
pub struct ListProjectsRequest {
    #[doc="<p>The maximum amount of data that can be contained in a single set of results.</p>"]
    #[serde(rename="maxResults")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub max_results: Option<i64>,
    #[doc="<p>The continuation token to be used to return the next set of results, if the results cannot be returned in one response.</p>"]
    #[serde(rename="nextToken")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default,Debug,Clone,Deserialize)]
pub struct ListProjectsResult {
    #[doc="<p>The continuation token to use when requesting the next set of results, if there are more results to be returned.</p>"]
    #[serde(rename="nextToken")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub next_token: Option<String>,
    #[doc="<p>A list of projects.</p>"]
    #[serde(rename="projects")]
    pub projects: Vec<ProjectSummary>,
}

#[derive(Default,Debug,Clone,Serialize)]
pub struct ListResourcesRequest {
    #[doc="<p>he maximum amount of data that can be contained in a single set of results.</p>"]
    #[serde(rename="maxResults")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub max_results: Option<i64>,
    #[doc="<p>The continuation token for the next set of results, if the results cannot be returned in one response.</p>"]
    #[serde(rename="nextToken")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub next_token: Option<String>,
    #[doc="<p>The ID of the project.</p>"]
    #[serde(rename="projectId")]
    pub project_id: String,
}

#[derive(Default,Debug,Clone,Deserialize)]
pub struct ListResourcesResult {
    #[doc="<p>The continuation token to use when requesting the next set of results, if there are more results to be returned.</p>"]
    #[serde(rename="nextToken")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub next_token: Option<String>,
    #[doc="<p>An array of resources associated with the project. </p>"]
    #[serde(rename="resources")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub resources: Option<Vec<Resource>>,
}

#[derive(Default,Debug,Clone,Serialize)]
pub struct ListTeamMembersRequest {
    #[doc="<p>The maximum number of team members you want returned in a response.</p>"]
    #[serde(rename="maxResults")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub max_results: Option<i64>,
    #[doc="<p>The continuation token for the next set of results, if the results cannot be returned in one response.</p>"]
    #[serde(rename="nextToken")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub next_token: Option<String>,
    #[doc="<p>The ID of the project for which you want to list team members.</p>"]
    #[serde(rename="projectId")]
    pub project_id: String,
}

#[derive(Default,Debug,Clone,Deserialize)]
pub struct ListTeamMembersResult {
    #[doc="<p>The continuation token to use when requesting the next set of results, if there are more results to be returned.</p>"]
    #[serde(rename="nextToken")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub next_token: Option<String>,
    #[doc="<p>A list of team member objects for the project.</p>"]
    #[serde(rename="teamMembers")]
    pub team_members: Vec<TeamMember>,
}

#[derive(Default,Debug,Clone,Serialize)]
pub struct ListUserProfilesRequest {
    #[doc="<p>The maximum number of results to return in a response.</p>"]
    #[serde(rename="maxResults")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub max_results: Option<i64>,
    #[doc="<p>The continuation token for the next set of results, if the results cannot be returned in one response.</p>"]
    #[serde(rename="nextToken")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default,Debug,Clone,Deserialize)]
pub struct ListUserProfilesResult {
    #[doc="<p>The continuation token to use when requesting the next set of results, if there are more results to be returned.</p>"]
    #[serde(rename="nextToken")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub next_token: Option<String>,
    #[doc="<p>All the user profiles configured in AWS CodeStar for an AWS account.</p>"]
    #[serde(rename="userProfiles")]
    pub user_profiles: Vec<UserProfileSummary>,
}

#[doc="<p>Information about the metadata for a project.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct ProjectSummary {
    #[doc="<p>The Amazon Resource Name (ARN) of the project.</p>"]
    #[serde(rename="projectArn")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub project_arn: Option<String>,
    #[doc="<p>The ID of the project.</p>"]
    #[serde(rename="projectId")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub project_id: Option<String>,
}

#[doc="<p>Information about a resource for a project.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct Resource {
    #[doc="<p>The Amazon Resource Name (ARN) of the resource.</p>"]
    #[serde(rename="id")]
    pub id: String,
}

#[doc="<p>Information about a team member in a project.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct TeamMember {
    #[doc="<p>The role assigned to the user in the project. Project roles have different levels of access. For more information, see <a href=\"http://docs.aws.amazon.com/codestar/latest/userguide/working-with-teams.html\">Working with Teams</a> in the AWS CodeStar User Guide. </p>"]
    #[serde(rename="projectRole")]
    pub project_role: String,
    #[doc="<p>Whether the user is allowed to remotely access project resources using an SSH public/private key pair.</p>"]
    #[serde(rename="remoteAccessAllowed")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub remote_access_allowed: Option<bool>,
    #[doc="<p>The Amazon Resource Name (ARN) of the user in IAM.</p>"]
    #[serde(rename="userArn")]
    pub user_arn: String,
}

#[derive(Default,Debug,Clone,Serialize)]
pub struct UpdateProjectRequest {
    #[doc="<p>The description of the project, if any.</p>"]
    #[serde(rename="description")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub description: Option<String>,
    #[doc="<p>The ID of the project you want to update.</p>"]
    #[serde(rename="id")]
    pub id: String,
    #[doc="<p>The name of the project you want to update.</p>"]
    #[serde(rename="name")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub name: Option<String>,
}

#[derive(Default,Debug,Clone,Deserialize)]
pub struct UpdateProjectResult;

#[derive(Default,Debug,Clone,Serialize)]
pub struct UpdateTeamMemberRequest {
    #[doc="<p>The ID of the project.</p>"]
    #[serde(rename="projectId")]
    pub project_id: String,
    #[doc="<p>The role assigned to the user in the project. Project roles have different levels of access. For more information, see <a href=\"http://docs.aws.amazon.com/codestar/latest/userguide/working-with-teams.html\">Working with Teams</a> in the AWS CodeStar User Guide.</p>"]
    #[serde(rename="projectRole")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub project_role: Option<String>,
    #[doc="<p>Whether a team member is allowed to remotely access project resources using the SSH public key associated with the user's profile. Even if this is set to True, the user must associate a public key with their profile before the user can access resources.</p>"]
    #[serde(rename="remoteAccessAllowed")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub remote_access_allowed: Option<bool>,
    #[doc="<p>The Amazon Resource Name (ARN) of the user for whom you want to change team membership attributes.</p>"]
    #[serde(rename="userArn")]
    pub user_arn: String,
}

#[derive(Default,Debug,Clone,Deserialize)]
pub struct UpdateTeamMemberResult {
    #[doc="<p>The project role granted to the user.</p>"]
    #[serde(rename="projectRole")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub project_role: Option<String>,
    #[doc="<p>Whether a team member is allowed to remotely access project resources using the SSH public key associated with the user's profile.</p>"]
    #[serde(rename="remoteAccessAllowed")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub remote_access_allowed: Option<bool>,
    #[doc="<p>The Amazon Resource Name (ARN) of the user whose team membership attributes were updated.</p>"]
    #[serde(rename="userArn")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub user_arn: Option<String>,
}

#[derive(Default,Debug,Clone,Serialize)]
pub struct UpdateUserProfileRequest {
    #[doc="<p>The name that is displayed as the friendly name for the user in AWS CodeStar.</p>"]
    #[serde(rename="displayName")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub display_name: Option<String>,
    #[doc="<p>The email address that is displayed as part of the user's profile in AWS CodeStar.</p>"]
    #[serde(rename="emailAddress")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub email_address: Option<String>,
    #[doc="<p>The SSH public key associated with the user in AWS CodeStar. If a project owner allows the user remote access to project resources, this public key will be used along with the user's private key for SSH access.</p>"]
    #[serde(rename="sshPublicKey")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub ssh_public_key: Option<String>,
    #[doc="<p>The name that will be displayed as the friendly name for the user in AWS CodeStar.</p>"]
    #[serde(rename="userArn")]
    pub user_arn: String,
}

#[derive(Default,Debug,Clone,Deserialize)]
pub struct UpdateUserProfileResult {
    #[doc="<p>The date the user profile was created, in timestamp format.</p>"]
    #[serde(rename="createdTimestamp")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub created_timestamp: Option<f64>,
    #[doc="<p>The name that is displayed as the friendly name for the user in AWS CodeStar.</p>"]
    #[serde(rename="displayName")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub display_name: Option<String>,
    #[doc="<p>The email address that is displayed as part of the user's profile in AWS CodeStar.</p>"]
    #[serde(rename="emailAddress")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub email_address: Option<String>,
    #[doc="<p>The date the user profile was last modified, in timestamp format.</p>"]
    #[serde(rename="lastModifiedTimestamp")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub last_modified_timestamp: Option<f64>,
    #[doc="<p>The SSH public key associated with the user in AWS CodeStar. This is the public portion of the public/private keypair the user can use to access project resources if a project owner allows the user remote access to those resources.</p>"]
    #[serde(rename="sshPublicKey")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub ssh_public_key: Option<String>,
    #[doc="<p>The Amazon Resource Name (ARN) of the user in IAM.</p>"]
    #[serde(rename="userArn")]
    pub user_arn: String,
}

#[doc="<p>Information about a user's profile in AWS CodeStar.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct UserProfileSummary {
    #[doc="<p>The display name of a user in AWS CodeStar. For example, this could be set to both first and last name (\"Mary Major\") or a single name (\"Mary\"). The display name is also used to generate the initial icon associated with the user in AWS CodeStar projects. If spaces are included in the display name, the first character that appears after the space will be used as the second character in the user initial icon. The initial icon displays a maximum of two characters, so a display name with more than one space (for example \"Mary Jane Major\") would generate an initial icon using the first character and the first character after the space (\"MJ\", not \"MM\").</p>"]
    #[serde(rename="displayName")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub display_name: Option<String>,
    #[doc="<p>The email address associated with the user.</p>"]
    #[serde(rename="emailAddress")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub email_address: Option<String>,
    #[doc="<p>The SSH public key associated with the user in AWS CodeStar. If a project owner allows the user remote access to project resources, this public key will be used along with the user's private key for SSH access.</p>"]
    #[serde(rename="sshPublicKey")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub ssh_public_key: Option<String>,
    #[doc="<p>The Amazon Resource Name (ARN) of the user in IAM.</p>"]
    #[serde(rename="userArn")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub user_arn: Option<String>,
}

/// Errors returned by AssociateTeamMember
#[derive(Debug, PartialEq)]
pub enum AssociateTeamMemberError {
    ///<p>Another modification is being made. That modification must complete before you can make your change.</p>
    ConcurrentModification(String),
    ///<p>The service role is not valid.</p>
    InvalidServiceRole(String),
    ///<p>A resource limit has been exceeded.</p>
    LimitExceeded(String),
    ///<p>Project configuration information is required but not specified.</p>
    ProjectConfiguration(String),
    ///<p>The specified AWS CodeStar project was not found.</p>
    ProjectNotFound(String),
    ///<p>The team member is already associated with a role in this project.</p>
    TeamMemberAlreadyAssociated(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl AssociateTeamMemberError {
    pub fn from_body(body: &str) -> AssociateTeamMemberError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "ConcurrentModificationException" => AssociateTeamMemberError::ConcurrentModification(String::from(error_message)),
                    "InvalidServiceRoleException" => {
                        AssociateTeamMemberError::InvalidServiceRole(String::from(error_message))
                    }
                    "LimitExceededException" => {
                        AssociateTeamMemberError::LimitExceeded(String::from(error_message))
                    }
                    "ProjectConfigurationException" => {
                        AssociateTeamMemberError::ProjectConfiguration(String::from(error_message))
                    }
                    "ProjectNotFoundException" => {
                        AssociateTeamMemberError::ProjectNotFound(String::from(error_message))
                    }
                    "TeamMemberAlreadyAssociatedException" => AssociateTeamMemberError::TeamMemberAlreadyAssociated(String::from(error_message)),
                    "ValidationException" => {
                        AssociateTeamMemberError::Validation(error_message.to_string())
                    }
                    _ => AssociateTeamMemberError::Unknown(String::from(body)),
                }
            }
            Err(_) => AssociateTeamMemberError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for AssociateTeamMemberError {
    fn from(err: serde_json::error::Error) -> AssociateTeamMemberError {
        AssociateTeamMemberError::Unknown(err.description().to_string())
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
            AssociateTeamMemberError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by CreateProject
#[derive(Debug, PartialEq)]
pub enum CreateProjectError {
    ///<p>Another modification is being made. That modification must complete before you can make your change.</p>
    ConcurrentModification(String),
    ///<p>The service role is not valid.</p>
    InvalidServiceRole(String),
    ///<p>A resource limit has been exceeded.</p>
    LimitExceeded(String),
    ///<p>An AWS CodeStar project with the same ID already exists in this region for the AWS account. AWS CodeStar project IDs must be unique within a region for the AWS account.</p>
    ProjectAlreadyExists(String),
    ///<p>Project configuration information is required but not specified.</p>
    ProjectConfiguration(String),
    ///<p>The project creation request was valid, but a nonspecific exception or error occurred during project creation. The project could not be created in AWS CodeStar.</p>
    ProjectCreationFailed(String),
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
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "ConcurrentModificationException" => {
                        CreateProjectError::ConcurrentModification(String::from(error_message))
                    }
                    "InvalidServiceRoleException" => {
                        CreateProjectError::InvalidServiceRole(String::from(error_message))
                    }
                    "LimitExceededException" => {
                        CreateProjectError::LimitExceeded(String::from(error_message))
                    }
                    "ProjectAlreadyExistsException" => {
                        CreateProjectError::ProjectAlreadyExists(String::from(error_message))
                    }
                    "ProjectConfigurationException" => {
                        CreateProjectError::ProjectConfiguration(String::from(error_message))
                    }
                    "ProjectCreationFailedException" => {
                        CreateProjectError::ProjectCreationFailed(String::from(error_message))
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
            CreateProjectError::ConcurrentModification(ref cause) => cause,
            CreateProjectError::InvalidServiceRole(ref cause) => cause,
            CreateProjectError::LimitExceeded(ref cause) => cause,
            CreateProjectError::ProjectAlreadyExists(ref cause) => cause,
            CreateProjectError::ProjectConfiguration(ref cause) => cause,
            CreateProjectError::ProjectCreationFailed(ref cause) => cause,
            CreateProjectError::Validation(ref cause) => cause,
            CreateProjectError::Credentials(ref err) => err.description(),
            CreateProjectError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            CreateProjectError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by CreateUserProfile
#[derive(Debug, PartialEq)]
pub enum CreateUserProfileError {
    ///<p>A user profile with that name already exists in this region for the AWS account. AWS CodeStar user profile names must be unique within a region for the AWS account. </p>
    UserProfileAlreadyExists(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl CreateUserProfileError {
    pub fn from_body(body: &str) -> CreateUserProfileError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "UserProfileAlreadyExistsException" => CreateUserProfileError::UserProfileAlreadyExists(String::from(error_message)),
                    "ValidationException" => {
                        CreateUserProfileError::Validation(error_message.to_string())
                    }
                    _ => CreateUserProfileError::Unknown(String::from(body)),
                }
            }
            Err(_) => CreateUserProfileError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for CreateUserProfileError {
    fn from(err: serde_json::error::Error) -> CreateUserProfileError {
        CreateUserProfileError::Unknown(err.description().to_string())
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
            CreateUserProfileError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DeleteProject
#[derive(Debug, PartialEq)]
pub enum DeleteProjectError {
    ///<p>Another modification is being made. That modification must complete before you can make your change.</p>
    ConcurrentModification(String),
    ///<p>The service role is not valid.</p>
    InvalidServiceRole(String),
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
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "ConcurrentModificationException" => {
                        DeleteProjectError::ConcurrentModification(String::from(error_message))
                    }
                    "InvalidServiceRoleException" => {
                        DeleteProjectError::InvalidServiceRole(String::from(error_message))
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
            DeleteProjectError::ConcurrentModification(ref cause) => cause,
            DeleteProjectError::InvalidServiceRole(ref cause) => cause,
            DeleteProjectError::Validation(ref cause) => cause,
            DeleteProjectError::Credentials(ref err) => err.description(),
            DeleteProjectError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            DeleteProjectError::Unknown(ref cause) => cause,
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
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl DeleteUserProfileError {
    pub fn from_body(body: &str) -> DeleteUserProfileError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "ValidationException" => {
                        DeleteUserProfileError::Validation(error_message.to_string())
                    }
                    _ => DeleteUserProfileError::Unknown(String::from(body)),
                }
            }
            Err(_) => DeleteUserProfileError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DeleteUserProfileError {
    fn from(err: serde_json::error::Error) -> DeleteUserProfileError {
        DeleteUserProfileError::Unknown(err.description().to_string())
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
            DeleteUserProfileError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DescribeProject
#[derive(Debug, PartialEq)]
pub enum DescribeProjectError {
    ///<p>Another modification is being made. That modification must complete before you can make your change.</p>
    ConcurrentModification(String),
    ///<p>The service role is not valid.</p>
    InvalidServiceRole(String),
    ///<p>Project configuration information is required but not specified.</p>
    ProjectConfiguration(String),
    ///<p>The specified AWS CodeStar project was not found.</p>
    ProjectNotFound(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl DescribeProjectError {
    pub fn from_body(body: &str) -> DescribeProjectError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "ConcurrentModificationException" => {
                        DescribeProjectError::ConcurrentModification(String::from(error_message))
                    }
                    "InvalidServiceRoleException" => {
                        DescribeProjectError::InvalidServiceRole(String::from(error_message))
                    }
                    "ProjectConfigurationException" => {
                        DescribeProjectError::ProjectConfiguration(String::from(error_message))
                    }
                    "ProjectNotFoundException" => {
                        DescribeProjectError::ProjectNotFound(String::from(error_message))
                    }
                    "ValidationException" => {
                        DescribeProjectError::Validation(error_message.to_string())
                    }
                    _ => DescribeProjectError::Unknown(String::from(body)),
                }
            }
            Err(_) => DescribeProjectError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DescribeProjectError {
    fn from(err: serde_json::error::Error) -> DescribeProjectError {
        DescribeProjectError::Unknown(err.description().to_string())
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
            DescribeProjectError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DescribeUserProfile
#[derive(Debug, PartialEq)]
pub enum DescribeUserProfileError {
    ///<p>The user profile was not found.</p>
    UserProfileNotFound(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl DescribeUserProfileError {
    pub fn from_body(body: &str) -> DescribeUserProfileError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "UserProfileNotFoundException" => {
                        DescribeUserProfileError::UserProfileNotFound(String::from(error_message))
                    }
                    "ValidationException" => {
                        DescribeUserProfileError::Validation(error_message.to_string())
                    }
                    _ => DescribeUserProfileError::Unknown(String::from(body)),
                }
            }
            Err(_) => DescribeUserProfileError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DescribeUserProfileError {
    fn from(err: serde_json::error::Error) -> DescribeUserProfileError {
        DescribeUserProfileError::Unknown(err.description().to_string())
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
            DescribeUserProfileError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DisassociateTeamMember
#[derive(Debug, PartialEq)]
pub enum DisassociateTeamMemberError {
    ///<p>Another modification is being made. That modification must complete before you can make your change.</p>
    ConcurrentModification(String),
    ///<p>The service role is not valid.</p>
    InvalidServiceRole(String),
    ///<p>The specified AWS CodeStar project was not found.</p>
    ProjectNotFound(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl DisassociateTeamMemberError {
    pub fn from_body(body: &str) -> DisassociateTeamMemberError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "ConcurrentModificationException" => DisassociateTeamMemberError::ConcurrentModification(String::from(error_message)),
                    "InvalidServiceRoleException" => {
                        DisassociateTeamMemberError::InvalidServiceRole(String::from(error_message))
                    }
                    "ProjectNotFoundException" => {
                        DisassociateTeamMemberError::ProjectNotFound(String::from(error_message))
                    }
                    "ValidationException" => {
                        DisassociateTeamMemberError::Validation(error_message.to_string())
                    }
                    _ => DisassociateTeamMemberError::Unknown(String::from(body)),
                }
            }
            Err(_) => DisassociateTeamMemberError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DisassociateTeamMemberError {
    fn from(err: serde_json::error::Error) -> DisassociateTeamMemberError {
        DisassociateTeamMemberError::Unknown(err.description().to_string())
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
            DisassociateTeamMemberError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by ListProjects
#[derive(Debug, PartialEq)]
pub enum ListProjectsError {
    ///<p>The next token is not valid.</p>
    InvalidNextToken(String),
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
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InvalidNextTokenException" => {
                        ListProjectsError::InvalidNextToken(String::from(error_message))
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
            ListProjectsError::InvalidNextToken(ref cause) => cause,
            ListProjectsError::Validation(ref cause) => cause,
            ListProjectsError::Credentials(ref err) => err.description(),
            ListProjectsError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            ListProjectsError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by ListResources
#[derive(Debug, PartialEq)]
pub enum ListResourcesError {
    ///<p>The next token is not valid.</p>
    InvalidNextToken(String),
    ///<p>The specified AWS CodeStar project was not found.</p>
    ProjectNotFound(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl ListResourcesError {
    pub fn from_body(body: &str) -> ListResourcesError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InvalidNextTokenException" => {
                        ListResourcesError::InvalidNextToken(String::from(error_message))
                    }
                    "ProjectNotFoundException" => {
                        ListResourcesError::ProjectNotFound(String::from(error_message))
                    }
                    "ValidationException" => {
                        ListResourcesError::Validation(error_message.to_string())
                    }
                    _ => ListResourcesError::Unknown(String::from(body)),
                }
            }
            Err(_) => ListResourcesError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for ListResourcesError {
    fn from(err: serde_json::error::Error) -> ListResourcesError {
        ListResourcesError::Unknown(err.description().to_string())
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
            ListResourcesError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by ListTeamMembers
#[derive(Debug, PartialEq)]
pub enum ListTeamMembersError {
    ///<p>The next token is not valid.</p>
    InvalidNextToken(String),
    ///<p>The specified AWS CodeStar project was not found.</p>
    ProjectNotFound(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl ListTeamMembersError {
    pub fn from_body(body: &str) -> ListTeamMembersError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InvalidNextTokenException" => {
                        ListTeamMembersError::InvalidNextToken(String::from(error_message))
                    }
                    "ProjectNotFoundException" => {
                        ListTeamMembersError::ProjectNotFound(String::from(error_message))
                    }
                    "ValidationException" => {
                        ListTeamMembersError::Validation(error_message.to_string())
                    }
                    _ => ListTeamMembersError::Unknown(String::from(body)),
                }
            }
            Err(_) => ListTeamMembersError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for ListTeamMembersError {
    fn from(err: serde_json::error::Error) -> ListTeamMembersError {
        ListTeamMembersError::Unknown(err.description().to_string())
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
            ListTeamMembersError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by ListUserProfiles
#[derive(Debug, PartialEq)]
pub enum ListUserProfilesError {
    ///<p>The next token is not valid.</p>
    InvalidNextToken(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl ListUserProfilesError {
    pub fn from_body(body: &str) -> ListUserProfilesError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InvalidNextTokenException" => {
                        ListUserProfilesError::InvalidNextToken(String::from(error_message))
                    }
                    "ValidationException" => {
                        ListUserProfilesError::Validation(error_message.to_string())
                    }
                    _ => ListUserProfilesError::Unknown(String::from(body)),
                }
            }
            Err(_) => ListUserProfilesError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for ListUserProfilesError {
    fn from(err: serde_json::error::Error) -> ListUserProfilesError {
        ListUserProfilesError::Unknown(err.description().to_string())
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
            ListUserProfilesError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by UpdateProject
#[derive(Debug, PartialEq)]
pub enum UpdateProjectError {
    ///<p>The specified AWS CodeStar project was not found.</p>
    ProjectNotFound(String),
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
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "ProjectNotFoundException" => {
                        UpdateProjectError::ProjectNotFound(String::from(error_message))
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
            UpdateProjectError::ProjectNotFound(ref cause) => cause,
            UpdateProjectError::Validation(ref cause) => cause,
            UpdateProjectError::Credentials(ref err) => err.description(),
            UpdateProjectError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            UpdateProjectError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by UpdateTeamMember
#[derive(Debug, PartialEq)]
pub enum UpdateTeamMemberError {
    ///<p>Another modification is being made. That modification must complete before you can make your change.</p>
    ConcurrentModification(String),
    ///<p>The service role is not valid.</p>
    InvalidServiceRole(String),
    ///<p>A resource limit has been exceeded.</p>
    LimitExceeded(String),
    ///<p>Project configuration information is required but not specified.</p>
    ProjectConfiguration(String),
    ///<p>The specified AWS CodeStar project was not found.</p>
    ProjectNotFound(String),
    ///<p>The specified team member was not found.</p>
    TeamMemberNotFound(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl UpdateTeamMemberError {
    pub fn from_body(body: &str) -> UpdateTeamMemberError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "ConcurrentModificationException" => {
                        UpdateTeamMemberError::ConcurrentModification(String::from(error_message))
                    }
                    "InvalidServiceRoleException" => {
                        UpdateTeamMemberError::InvalidServiceRole(String::from(error_message))
                    }
                    "LimitExceededException" => {
                        UpdateTeamMemberError::LimitExceeded(String::from(error_message))
                    }
                    "ProjectConfigurationException" => {
                        UpdateTeamMemberError::ProjectConfiguration(String::from(error_message))
                    }
                    "ProjectNotFoundException" => {
                        UpdateTeamMemberError::ProjectNotFound(String::from(error_message))
                    }
                    "TeamMemberNotFoundException" => {
                        UpdateTeamMemberError::TeamMemberNotFound(String::from(error_message))
                    }
                    "ValidationException" => {
                        UpdateTeamMemberError::Validation(error_message.to_string())
                    }
                    _ => UpdateTeamMemberError::Unknown(String::from(body)),
                }
            }
            Err(_) => UpdateTeamMemberError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for UpdateTeamMemberError {
    fn from(err: serde_json::error::Error) -> UpdateTeamMemberError {
        UpdateTeamMemberError::Unknown(err.description().to_string())
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
            UpdateTeamMemberError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by UpdateUserProfile
#[derive(Debug, PartialEq)]
pub enum UpdateUserProfileError {
    ///<p>The user profile was not found.</p>
    UserProfileNotFound(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl UpdateUserProfileError {
    pub fn from_body(body: &str) -> UpdateUserProfileError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "UserProfileNotFoundException" => {
                        UpdateUserProfileError::UserProfileNotFound(String::from(error_message))
                    }
                    "ValidationException" => {
                        UpdateUserProfileError::Validation(error_message.to_string())
                    }
                    _ => UpdateUserProfileError::Unknown(String::from(body)),
                }
            }
            Err(_) => UpdateUserProfileError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for UpdateUserProfileError {
    fn from(err: serde_json::error::Error) -> UpdateUserProfileError {
        UpdateUserProfileError::Unknown(err.description().to_string())
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
            UpdateUserProfileError::Unknown(ref cause) => cause,
        }
    }
}
/// Trait representing the capabilities of the CodeStar API. CodeStar clients implement this trait.
pub trait CodeStar {
    #[doc="<p>Adds an IAM user to the team for an AWS CodeStar project.</p>"]
    fn associate_team_member(&self,
                             input: &AssociateTeamMemberRequest)
                             -> Result<AssociateTeamMemberResult, AssociateTeamMemberError>;


    #[doc="<p>Reserved for future use. To create a project, use the AWS CodeStar console.</p>"]
    fn create_project(&self,
                      input: &CreateProjectRequest)
                      -> Result<CreateProjectResult, CreateProjectError>;


    #[doc="<p>Creates a profile for a user that includes user preferences, such as the display name and email address assocciated with the user, in AWS CodeStar. The user profile is not project-specific. Information in the user profile is displayed wherever the user's information appears to other users in AWS CodeStar.</p>"]
    fn create_user_profile(&self,
                           input: &CreateUserProfileRequest)
                           -> Result<CreateUserProfileResult, CreateUserProfileError>;


    #[doc="<p>Deletes a project, including project resources. Does not delete users associated with the project, but does delete the IAM roles that allowed access to the project.</p>"]
    fn delete_project(&self,
                      input: &DeleteProjectRequest)
                      -> Result<DeleteProjectResult, DeleteProjectError>;


    #[doc="<p>Deletes a user profile in AWS CodeStar, including all personal preference data associated with that profile, such as display name and email address. It does not delete the history of that user, for example the history of commits made by that user.</p>"]
    fn delete_user_profile(&self,
                           input: &DeleteUserProfileRequest)
                           -> Result<DeleteUserProfileResult, DeleteUserProfileError>;


    #[doc="<p>Describes a project and its resources.</p>"]
    fn describe_project(&self,
                        input: &DescribeProjectRequest)
                        -> Result<DescribeProjectResult, DescribeProjectError>;


    #[doc="<p>Describes a user in AWS CodeStar and the user attributes across all projects.</p>"]
    fn describe_user_profile(&self,
                             input: &DescribeUserProfileRequest)
                             -> Result<DescribeUserProfileResult, DescribeUserProfileError>;


    #[doc="<p>Removes a user from a project. Removing a user from a project also removes the IAM policies from that user that allowed access to the project and its resources. Disassociating a team member does not remove that user's profile from AWS CodeStar. It does not remove the user from IAM.</p>"]
    fn disassociate_team_member
        (&self,
         input: &DisassociateTeamMemberRequest)
         -> Result<DisassociateTeamMemberResult, DisassociateTeamMemberError>;


    #[doc="<p>Lists all projects in AWS CodeStar associated with your AWS account.</p>"]
    fn list_projects(&self,
                     input: &ListProjectsRequest)
                     -> Result<ListProjectsResult, ListProjectsError>;


    #[doc="<p>Lists resources associated with a project in AWS CodeStar.</p>"]
    fn list_resources(&self,
                      input: &ListResourcesRequest)
                      -> Result<ListResourcesResult, ListResourcesError>;


    #[doc="<p>Lists all team members associated with a project.</p>"]
    fn list_team_members(&self,
                         input: &ListTeamMembersRequest)
                         -> Result<ListTeamMembersResult, ListTeamMembersError>;


    #[doc="<p>Lists all the user profiles configured for your AWS account in AWS CodeStar.</p>"]
    fn list_user_profiles(&self,
                          input: &ListUserProfilesRequest)
                          -> Result<ListUserProfilesResult, ListUserProfilesError>;


    #[doc="<p>Updates a project in AWS CodeStar.</p>"]
    fn update_project(&self,
                      input: &UpdateProjectRequest)
                      -> Result<UpdateProjectResult, UpdateProjectError>;


    #[doc="<p>Updates a team member's attributes in an AWS CodeStar project. For example, you can change a team member's role in the project, or change whether they have remote access to project resources.</p>"]
    fn update_team_member(&self,
                          input: &UpdateTeamMemberRequest)
                          -> Result<UpdateTeamMemberResult, UpdateTeamMemberError>;


    #[doc="<p>Updates a user's profile in AWS CodeStar. The user profile is not project-specific. Information in the user profile is displayed wherever the user's information appears to other users in AWS CodeStar. </p>"]
    fn update_user_profile(&self,
                           input: &UpdateUserProfileRequest)
                           -> Result<UpdateUserProfileResult, UpdateUserProfileError>;
}
/// A client for the CodeStar API.
pub struct CodeStarClient<P, D>
    where P: ProvideAwsCredentials,
          D: DispatchSignedRequest
{
    credentials_provider: P,
    region: region::Region,
    dispatcher: D,
}

impl<P, D> CodeStarClient<P, D>
    where P: ProvideAwsCredentials,
          D: DispatchSignedRequest
{
    pub fn new(request_dispatcher: D, credentials_provider: P, region: region::Region) -> Self {
        CodeStarClient {
            credentials_provider: credentials_provider,
            region: region,
            dispatcher: request_dispatcher,
        }
    }
}

impl<P, D> CodeStar for CodeStarClient<P, D>
    where P: ProvideAwsCredentials,
          D: DispatchSignedRequest
{
    #[doc="<p>Adds an IAM user to the team for an AWS CodeStar project.</p>"]
    fn associate_team_member(&self,
                             input: &AssociateTeamMemberRequest)
                             -> Result<AssociateTeamMemberResult, AssociateTeamMemberError> {
        let mut request = SignedRequest::new("POST", "codestar", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "CodeStar_20170419.AssociateTeamMember");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign_with_plus(&try!(self.credentials_provider.credentials()), true);

        let mut response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            ::hyper::status::StatusCode::Ok => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Ok(serde_json::from_str::<AssociateTeamMemberResult>(String::from_utf8_lossy(&body).as_ref()).unwrap())
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(AssociateTeamMemberError::from_body(String::from_utf8_lossy(&body).as_ref()))
            }
        }
    }


    #[doc="<p>Reserved for future use. To create a project, use the AWS CodeStar console.</p>"]
    fn create_project(&self,
                      input: &CreateProjectRequest)
                      -> Result<CreateProjectResult, CreateProjectError> {
        let mut request = SignedRequest::new("POST", "codestar", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "CodeStar_20170419.CreateProject");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign_with_plus(&try!(self.credentials_provider.credentials()), true);

        let mut response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            ::hyper::status::StatusCode::Ok => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Ok(serde_json::from_str::<CreateProjectResult>(String::from_utf8_lossy(&body)
                                                                   .as_ref())
                           .unwrap())
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(CreateProjectError::from_body(String::from_utf8_lossy(&body).as_ref()))
            }
        }
    }


    #[doc="<p>Creates a profile for a user that includes user preferences, such as the display name and email address assocciated with the user, in AWS CodeStar. The user profile is not project-specific. Information in the user profile is displayed wherever the user's information appears to other users in AWS CodeStar.</p>"]
    fn create_user_profile(&self,
                           input: &CreateUserProfileRequest)
                           -> Result<CreateUserProfileResult, CreateUserProfileError> {
        let mut request = SignedRequest::new("POST", "codestar", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "CodeStar_20170419.CreateUserProfile");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign_with_plus(&try!(self.credentials_provider.credentials()), true);

        let mut response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            ::hyper::status::StatusCode::Ok => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Ok(serde_json::from_str::<CreateUserProfileResult>(String::from_utf8_lossy(&body)
                                                                       .as_ref())
                           .unwrap())
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(CreateUserProfileError::from_body(String::from_utf8_lossy(&body).as_ref()))
            }
        }
    }


    #[doc="<p>Deletes a project, including project resources. Does not delete users associated with the project, but does delete the IAM roles that allowed access to the project.</p>"]
    fn delete_project(&self,
                      input: &DeleteProjectRequest)
                      -> Result<DeleteProjectResult, DeleteProjectError> {
        let mut request = SignedRequest::new("POST", "codestar", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "CodeStar_20170419.DeleteProject");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign_with_plus(&try!(self.credentials_provider.credentials()), true);

        let mut response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            ::hyper::status::StatusCode::Ok => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Ok(serde_json::from_str::<DeleteProjectResult>(String::from_utf8_lossy(&body)
                                                                   .as_ref())
                           .unwrap())
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(DeleteProjectError::from_body(String::from_utf8_lossy(&body).as_ref()))
            }
        }
    }


    #[doc="<p>Deletes a user profile in AWS CodeStar, including all personal preference data associated with that profile, such as display name and email address. It does not delete the history of that user, for example the history of commits made by that user.</p>"]
    fn delete_user_profile(&self,
                           input: &DeleteUserProfileRequest)
                           -> Result<DeleteUserProfileResult, DeleteUserProfileError> {
        let mut request = SignedRequest::new("POST", "codestar", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "CodeStar_20170419.DeleteUserProfile");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign_with_plus(&try!(self.credentials_provider.credentials()), true);

        let mut response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            ::hyper::status::StatusCode::Ok => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Ok(serde_json::from_str::<DeleteUserProfileResult>(String::from_utf8_lossy(&body)
                                                                       .as_ref())
                           .unwrap())
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(DeleteUserProfileError::from_body(String::from_utf8_lossy(&body).as_ref()))
            }
        }
    }


    #[doc="<p>Describes a project and its resources.</p>"]
    fn describe_project(&self,
                        input: &DescribeProjectRequest)
                        -> Result<DescribeProjectResult, DescribeProjectError> {
        let mut request = SignedRequest::new("POST", "codestar", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "CodeStar_20170419.DescribeProject");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign_with_plus(&try!(self.credentials_provider.credentials()), true);

        let mut response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            ::hyper::status::StatusCode::Ok => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Ok(serde_json::from_str::<DescribeProjectResult>(String::from_utf8_lossy(&body)
                                                                     .as_ref())
                           .unwrap())
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(DescribeProjectError::from_body(String::from_utf8_lossy(&body).as_ref()))
            }
        }
    }


    #[doc="<p>Describes a user in AWS CodeStar and the user attributes across all projects.</p>"]
    fn describe_user_profile(&self,
                             input: &DescribeUserProfileRequest)
                             -> Result<DescribeUserProfileResult, DescribeUserProfileError> {
        let mut request = SignedRequest::new("POST", "codestar", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "CodeStar_20170419.DescribeUserProfile");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign_with_plus(&try!(self.credentials_provider.credentials()), true);

        let mut response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            ::hyper::status::StatusCode::Ok => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Ok(serde_json::from_str::<DescribeUserProfileResult>(String::from_utf8_lossy(&body).as_ref()).unwrap())
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(DescribeUserProfileError::from_body(String::from_utf8_lossy(&body).as_ref()))
            }
        }
    }


    #[doc="<p>Removes a user from a project. Removing a user from a project also removes the IAM policies from that user that allowed access to the project and its resources. Disassociating a team member does not remove that user's profile from AWS CodeStar. It does not remove the user from IAM.</p>"]
    fn disassociate_team_member
        (&self,
         input: &DisassociateTeamMemberRequest)
         -> Result<DisassociateTeamMemberResult, DisassociateTeamMemberError> {
        let mut request = SignedRequest::new("POST", "codestar", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "CodeStar_20170419.DisassociateTeamMember");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign_with_plus(&try!(self.credentials_provider.credentials()), true);

        let mut response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            ::hyper::status::StatusCode::Ok => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Ok(serde_json::from_str::<DisassociateTeamMemberResult>(String::from_utf8_lossy(&body).as_ref()).unwrap())
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(DisassociateTeamMemberError::from_body(String::from_utf8_lossy(&body).as_ref()))
            }
        }
    }


    #[doc="<p>Lists all projects in AWS CodeStar associated with your AWS account.</p>"]
    fn list_projects(&self,
                     input: &ListProjectsRequest)
                     -> Result<ListProjectsResult, ListProjectsError> {
        let mut request = SignedRequest::new("POST", "codestar", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "CodeStar_20170419.ListProjects");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign_with_plus(&try!(self.credentials_provider.credentials()), true);

        let mut response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            ::hyper::status::StatusCode::Ok => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Ok(serde_json::from_str::<ListProjectsResult>(String::from_utf8_lossy(&body)
                                                                  .as_ref())
                           .unwrap())
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(ListProjectsError::from_body(String::from_utf8_lossy(&body).as_ref()))
            }
        }
    }


    #[doc="<p>Lists resources associated with a project in AWS CodeStar.</p>"]
    fn list_resources(&self,
                      input: &ListResourcesRequest)
                      -> Result<ListResourcesResult, ListResourcesError> {
        let mut request = SignedRequest::new("POST", "codestar", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "CodeStar_20170419.ListResources");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign_with_plus(&try!(self.credentials_provider.credentials()), true);

        let mut response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            ::hyper::status::StatusCode::Ok => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Ok(serde_json::from_str::<ListResourcesResult>(String::from_utf8_lossy(&body)
                                                                   .as_ref())
                           .unwrap())
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(ListResourcesError::from_body(String::from_utf8_lossy(&body).as_ref()))
            }
        }
    }


    #[doc="<p>Lists all team members associated with a project.</p>"]
    fn list_team_members(&self,
                         input: &ListTeamMembersRequest)
                         -> Result<ListTeamMembersResult, ListTeamMembersError> {
        let mut request = SignedRequest::new("POST", "codestar", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "CodeStar_20170419.ListTeamMembers");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign_with_plus(&try!(self.credentials_provider.credentials()), true);

        let mut response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            ::hyper::status::StatusCode::Ok => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Ok(serde_json::from_str::<ListTeamMembersResult>(String::from_utf8_lossy(&body)
                                                                     .as_ref())
                           .unwrap())
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(ListTeamMembersError::from_body(String::from_utf8_lossy(&body).as_ref()))
            }
        }
    }


    #[doc="<p>Lists all the user profiles configured for your AWS account in AWS CodeStar.</p>"]
    fn list_user_profiles(&self,
                          input: &ListUserProfilesRequest)
                          -> Result<ListUserProfilesResult, ListUserProfilesError> {
        let mut request = SignedRequest::new("POST", "codestar", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "CodeStar_20170419.ListUserProfiles");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign_with_plus(&try!(self.credentials_provider.credentials()), true);

        let mut response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            ::hyper::status::StatusCode::Ok => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Ok(serde_json::from_str::<ListUserProfilesResult>(String::from_utf8_lossy(&body)
                                                                      .as_ref())
                           .unwrap())
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(ListUserProfilesError::from_body(String::from_utf8_lossy(&body).as_ref()))
            }
        }
    }


    #[doc="<p>Updates a project in AWS CodeStar.</p>"]
    fn update_project(&self,
                      input: &UpdateProjectRequest)
                      -> Result<UpdateProjectResult, UpdateProjectError> {
        let mut request = SignedRequest::new("POST", "codestar", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "CodeStar_20170419.UpdateProject");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign_with_plus(&try!(self.credentials_provider.credentials()), true);

        let mut response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            ::hyper::status::StatusCode::Ok => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Ok(serde_json::from_str::<UpdateProjectResult>(String::from_utf8_lossy(&body)
                                                                   .as_ref())
                           .unwrap())
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(UpdateProjectError::from_body(String::from_utf8_lossy(&body).as_ref()))
            }
        }
    }


    #[doc="<p>Updates a team member's attributes in an AWS CodeStar project. For example, you can change a team member's role in the project, or change whether they have remote access to project resources.</p>"]
    fn update_team_member(&self,
                          input: &UpdateTeamMemberRequest)
                          -> Result<UpdateTeamMemberResult, UpdateTeamMemberError> {
        let mut request = SignedRequest::new("POST", "codestar", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "CodeStar_20170419.UpdateTeamMember");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign_with_plus(&try!(self.credentials_provider.credentials()), true);

        let mut response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            ::hyper::status::StatusCode::Ok => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Ok(serde_json::from_str::<UpdateTeamMemberResult>(String::from_utf8_lossy(&body)
                                                                      .as_ref())
                           .unwrap())
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(UpdateTeamMemberError::from_body(String::from_utf8_lossy(&body).as_ref()))
            }
        }
    }


    #[doc="<p>Updates a user's profile in AWS CodeStar. The user profile is not project-specific. Information in the user profile is displayed wherever the user's information appears to other users in AWS CodeStar. </p>"]
    fn update_user_profile(&self,
                           input: &UpdateUserProfileRequest)
                           -> Result<UpdateUserProfileResult, UpdateUserProfileError> {
        let mut request = SignedRequest::new("POST", "codestar", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "CodeStar_20170419.UpdateUserProfile");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign_with_plus(&try!(self.credentials_provider.credentials()), true);

        let mut response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            ::hyper::status::StatusCode::Ok => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Ok(serde_json::from_str::<UpdateUserProfileResult>(String::from_utf8_lossy(&body)
                                                                       .as_ref())
                           .unwrap())
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(UpdateUserProfileError::from_body(String::from_utf8_lossy(&body).as_ref()))
            }
        }
    }
}

#[cfg(test)]
mod protocol_tests {}
