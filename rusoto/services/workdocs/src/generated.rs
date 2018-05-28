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

use rusoto_core::param::{Params, ServiceParams};
use rusoto_core::signature::SignedRequest;
use serde_json;
use serde_json::from_str;
use serde_json::Value as SerdeJsonValue;
#[derive(Default, Debug, Clone, Serialize)]
pub struct AbortDocumentVersionUploadRequest {
    /// <p>Amazon WorkDocs authentication token. Do not set this field when using administrative API actions, as in accessing the API using AWS credentials.</p>
    #[serde(rename = "AuthenticationToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authentication_token: Option<String>,
    /// <p>The ID of the document.</p>
    #[serde(rename = "DocumentId")]
    pub document_id: String,
    /// <p>The ID of the version.</p>
    #[serde(rename = "VersionId")]
    pub version_id: String,
}

#[derive(Default, Debug, Clone, Serialize)]
pub struct ActivateUserRequest {
    /// <p>Amazon WorkDocs authentication token. Do not set this field when using administrative API actions, as in accessing the API using AWS credentials.</p>
    #[serde(rename = "AuthenticationToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authentication_token: Option<String>,
    /// <p>The ID of the user.</p>
    #[serde(rename = "UserId")]
    pub user_id: String,
}

#[derive(Default, Debug, Clone, Deserialize)]
pub struct ActivateUserResponse {
    /// <p>The user information.</p>
    #[serde(rename = "User")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<User>,
}

/// <p>Describes the activity information.</p>
#[derive(Default, Debug, Clone, Deserialize)]
pub struct Activity {
    /// <p>Metadata of the commenting activity. This is an optional field and is filled for commenting activities.</p>
    #[serde(rename = "CommentMetadata")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment_metadata: Option<CommentMetadata>,
    /// <p>The user who performed the action.</p>
    #[serde(rename = "Initiator")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub initiator: Option<UserMetadata>,
    /// <p>The ID of the organization.</p>
    #[serde(rename = "OrganizationId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub organization_id: Option<String>,
    /// <p>The original parent of the resource. This is an optional field and is filled for move activities.</p>
    #[serde(rename = "OriginalParent")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub original_parent: Option<ResourceMetadata>,
    /// <p>The list of users or groups impacted by this action. This is an optional field and is filled for the following sharing activities: DOCUMENT_SHARED, DOCUMENT_SHARED, DOCUMENT_UNSHARED, FOLDER_SHARED, FOLDER_UNSHARED.</p>
    #[serde(rename = "Participants")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub participants: Option<Participants>,
    /// <p>The metadata of the resource involved in the user action.</p>
    #[serde(rename = "ResourceMetadata")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_metadata: Option<ResourceMetadata>,
    /// <p>The timestamp when the action was performed.</p>
    #[serde(rename = "TimeStamp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_stamp: Option<f64>,
    /// <p>The activity type.</p>
    #[serde(rename = "Type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}

#[derive(Default, Debug, Clone, Serialize)]
pub struct AddResourcePermissionsRequest {
    /// <p>Amazon WorkDocs authentication token. Do not set this field when using administrative API actions, as in accessing the API using AWS credentials.</p>
    #[serde(rename = "AuthenticationToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authentication_token: Option<String>,
    /// <p>The notification options.</p>
    #[serde(rename = "NotificationOptions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notification_options: Option<NotificationOptions>,
    /// <p>The users, groups, or organization being granted permission.</p>
    #[serde(rename = "Principals")]
    pub principals: Vec<SharePrincipal>,
    /// <p>The ID of the resource.</p>
    #[serde(rename = "ResourceId")]
    pub resource_id: String,
}

#[derive(Default, Debug, Clone, Deserialize)]
pub struct AddResourcePermissionsResponse {
    /// <p>The share results.</p>
    #[serde(rename = "ShareResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub share_results: Option<Vec<ShareResult>>,
}

/// <p>Describes a comment.</p>
#[derive(Default, Debug, Clone, Deserialize)]
pub struct Comment {
    /// <p>The ID of the comment.</p>
    #[serde(rename = "CommentId")]
    pub comment_id: String,
    /// <p>The details of the user who made the comment.</p>
    #[serde(rename = "Contributor")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contributor: Option<User>,
    /// <p>The time that the comment was created.</p>
    #[serde(rename = "CreatedTimestamp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_timestamp: Option<f64>,
    /// <p>The ID of the parent comment.</p>
    #[serde(rename = "ParentId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_id: Option<String>,
    /// <p>If the comment is a reply to another user's comment, this field contains the user ID of the user being replied to.</p>
    #[serde(rename = "RecipientId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recipient_id: Option<String>,
    /// <p>The status of the comment.</p>
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// <p>The text of the comment.</p>
    #[serde(rename = "Text")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    /// <p>The ID of the root comment in the thread.</p>
    #[serde(rename = "ThreadId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thread_id: Option<String>,
    /// <p>The visibility of the comment. Options are either PRIVATE, where the comment is visible only to the comment author and document owner and co-owners, or PUBLIC, where the comment is visible to document owners, co-owners, and contributors.</p>
    #[serde(rename = "Visibility")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub visibility: Option<String>,
}

/// <p>Describes the metadata of a comment.</p>
#[derive(Default, Debug, Clone, Deserialize)]
pub struct CommentMetadata {
    /// <p>The ID of the comment.</p>
    #[serde(rename = "CommentId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment_id: Option<String>,
    /// <p>The status of the comment.</p>
    #[serde(rename = "CommentStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment_status: Option<String>,
    /// <p>The user who made the comment.</p>
    #[serde(rename = "Contributor")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contributor: Option<User>,
    /// <p>The timestamp that the comment was created.</p>
    #[serde(rename = "CreatedTimestamp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_timestamp: Option<f64>,
    /// <p>The ID of the user being replied to.</p>
    #[serde(rename = "RecipientId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recipient_id: Option<String>,
}

#[derive(Default, Debug, Clone, Serialize)]
pub struct CreateCommentRequest {
    /// <p>Amazon WorkDocs authentication token. Do not set this field when using administrative API actions, as in accessing the API using AWS credentials.</p>
    #[serde(rename = "AuthenticationToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authentication_token: Option<String>,
    /// <p>The ID of the document.</p>
    #[serde(rename = "DocumentId")]
    pub document_id: String,
    /// <p>Set this parameter to TRUE to send an email out to the document collaborators after the comment is created.</p>
    #[serde(rename = "NotifyCollaborators")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notify_collaborators: Option<bool>,
    /// <p>The ID of the parent comment.</p>
    #[serde(rename = "ParentId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_id: Option<String>,
    /// <p>The text of the comment.</p>
    #[serde(rename = "Text")]
    pub text: String,
    /// <p>The ID of the root comment in the thread.</p>
    #[serde(rename = "ThreadId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thread_id: Option<String>,
    /// <p>The ID of the document version.</p>
    #[serde(rename = "VersionId")]
    pub version_id: String,
    /// <p>The visibility of the comment. Options are either PRIVATE, where the comment is visible only to the comment author and document owner and co-owners, or PUBLIC, where the comment is visible to document owners, co-owners, and contributors.</p>
    #[serde(rename = "Visibility")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub visibility: Option<String>,
}

#[derive(Default, Debug, Clone, Deserialize)]
pub struct CreateCommentResponse {
    /// <p>The comment that has been created.</p>
    #[serde(rename = "Comment")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment: Option<Comment>,
}

#[derive(Default, Debug, Clone, Serialize)]
pub struct CreateCustomMetadataRequest {
    /// <p>Amazon WorkDocs authentication token. Do not set this field when using administrative API actions, as in accessing the API using AWS credentials.</p>
    #[serde(rename = "AuthenticationToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authentication_token: Option<String>,
    /// <p>Custom metadata in the form of name-value pairs.</p>
    #[serde(rename = "CustomMetadata")]
    pub custom_metadata: ::std::collections::HashMap<String, String>,
    /// <p>The ID of the resource.</p>
    #[serde(rename = "ResourceId")]
    pub resource_id: String,
    /// <p>The ID of the version, if the custom metadata is being added to a document version.</p>
    #[serde(rename = "VersionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version_id: Option<String>,
}

#[derive(Default, Debug, Clone, Deserialize)]
pub struct CreateCustomMetadataResponse {}

#[derive(Default, Debug, Clone, Serialize)]
pub struct CreateFolderRequest {
    /// <p>Amazon WorkDocs authentication token. Do not set this field when using administrative API actions, as in accessing the API using AWS credentials.</p>
    #[serde(rename = "AuthenticationToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authentication_token: Option<String>,
    /// <p>The name of the new folder.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The ID of the parent folder.</p>
    #[serde(rename = "ParentFolderId")]
    pub parent_folder_id: String,
}

#[derive(Default, Debug, Clone, Deserialize)]
pub struct CreateFolderResponse {
    /// <p>The metadata of the folder.</p>
    #[serde(rename = "Metadata")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<FolderMetadata>,
}

#[derive(Default, Debug, Clone, Serialize)]
pub struct CreateLabelsRequest {
    /// <p>Amazon WorkDocs authentication token. Do not set this field when using administrative API actions, as in accessing the API using AWS credentials.</p>
    #[serde(rename = "AuthenticationToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authentication_token: Option<String>,
    /// <p>List of labels to add to the resource.</p>
    #[serde(rename = "Labels")]
    pub labels: Vec<String>,
    /// <p>The ID of the resource.</p>
    #[serde(rename = "ResourceId")]
    pub resource_id: String,
}

#[derive(Default, Debug, Clone, Deserialize)]
pub struct CreateLabelsResponse {}

#[derive(Default, Debug, Clone, Serialize)]
pub struct CreateNotificationSubscriptionRequest {
    /// <p>The endpoint to receive the notifications. If the protocol is HTTPS, the endpoint is a URL that begins with "https://".</p>
    #[serde(rename = "Endpoint")]
    pub endpoint: String,
    /// <p>The ID of the organization.</p>
    #[serde(rename = "OrganizationId")]
    pub organization_id: String,
    /// <p>The protocol to use. The supported value is https, which delivers JSON-encoded messages using HTTPS POST.</p>
    #[serde(rename = "Protocol")]
    pub protocol: String,
    /// <p>The notification type.</p>
    #[serde(rename = "SubscriptionType")]
    pub subscription_type: String,
}

#[derive(Default, Debug, Clone, Deserialize)]
pub struct CreateNotificationSubscriptionResponse {
    /// <p>The subscription.</p>
    #[serde(rename = "Subscription")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscription: Option<Subscription>,
}

#[derive(Default, Debug, Clone, Serialize)]
pub struct CreateUserRequest {
    /// <p>Amazon WorkDocs authentication token. Do not set this field when using administrative API actions, as in accessing the API using AWS credentials.</p>
    #[serde(rename = "AuthenticationToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authentication_token: Option<String>,
    /// <p>The email address of the user.</p>
    #[serde(rename = "EmailAddress")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email_address: Option<String>,
    /// <p>The given name of the user.</p>
    #[serde(rename = "GivenName")]
    pub given_name: String,
    /// <p>The ID of the organization.</p>
    #[serde(rename = "OrganizationId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub organization_id: Option<String>,
    /// <p>The password of the user.</p>
    #[serde(rename = "Password")]
    pub password: String,
    /// <p>The amount of storage for the user.</p>
    #[serde(rename = "StorageRule")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage_rule: Option<StorageRuleType>,
    /// <p>The surname of the user.</p>
    #[serde(rename = "Surname")]
    pub surname: String,
    /// <p>The time zone ID of the user.</p>
    #[serde(rename = "TimeZoneId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_zone_id: Option<String>,
    /// <p>The login name of the user.</p>
    #[serde(rename = "Username")]
    pub username: String,
}

#[derive(Default, Debug, Clone, Deserialize)]
pub struct CreateUserResponse {
    /// <p>The user information.</p>
    #[serde(rename = "User")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<User>,
}

#[derive(Default, Debug, Clone, Serialize)]
pub struct DeactivateUserRequest {
    /// <p>Amazon WorkDocs authentication token. Do not set this field when using administrative API actions, as in accessing the API using AWS credentials.</p>
    #[serde(rename = "AuthenticationToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authentication_token: Option<String>,
    /// <p>The ID of the user.</p>
    #[serde(rename = "UserId")]
    pub user_id: String,
}

#[derive(Default, Debug, Clone, Serialize)]
pub struct DeleteCommentRequest {
    /// <p>Amazon WorkDocs authentication token. Do not set this field when using administrative API actions, as in accessing the API using AWS credentials.</p>
    #[serde(rename = "AuthenticationToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authentication_token: Option<String>,
    /// <p>The ID of the comment.</p>
    #[serde(rename = "CommentId")]
    pub comment_id: String,
    /// <p>The ID of the document.</p>
    #[serde(rename = "DocumentId")]
    pub document_id: String,
    /// <p>The ID of the document version.</p>
    #[serde(rename = "VersionId")]
    pub version_id: String,
}

#[derive(Default, Debug, Clone, Serialize)]
pub struct DeleteCustomMetadataRequest {
    /// <p>Amazon WorkDocs authentication token. Do not set this field when using administrative API actions, as in accessing the API using AWS credentials.</p>
    #[serde(rename = "AuthenticationToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authentication_token: Option<String>,
    /// <p>Flag to indicate removal of all custom metadata properties from the specified resource.</p>
    #[serde(rename = "DeleteAll")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delete_all: Option<bool>,
    /// <p>List of properties to remove.</p>
    #[serde(rename = "Keys")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub keys: Option<Vec<String>>,
    /// <p>The ID of the resource, either a document or folder.</p>
    #[serde(rename = "ResourceId")]
    pub resource_id: String,
    /// <p>The ID of the version, if the custom metadata is being deleted from a document version.</p>
    #[serde(rename = "VersionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version_id: Option<String>,
}

#[derive(Default, Debug, Clone, Deserialize)]
pub struct DeleteCustomMetadataResponse {}

#[derive(Default, Debug, Clone, Serialize)]
pub struct DeleteDocumentRequest {
    /// <p>Amazon WorkDocs authentication token. Do not set this field when using administrative API actions, as in accessing the API using AWS credentials.</p>
    #[serde(rename = "AuthenticationToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authentication_token: Option<String>,
    /// <p>The ID of the document.</p>
    #[serde(rename = "DocumentId")]
    pub document_id: String,
}

#[derive(Default, Debug, Clone, Serialize)]
pub struct DeleteFolderContentsRequest {
    /// <p>Amazon WorkDocs authentication token. Do not set this field when using administrative API actions, as in accessing the API using AWS credentials.</p>
    #[serde(rename = "AuthenticationToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authentication_token: Option<String>,
    /// <p>The ID of the folder.</p>
    #[serde(rename = "FolderId")]
    pub folder_id: String,
}

#[derive(Default, Debug, Clone, Serialize)]
pub struct DeleteFolderRequest {
    /// <p>Amazon WorkDocs authentication token. Do not set this field when using administrative API actions, as in accessing the API using AWS credentials.</p>
    #[serde(rename = "AuthenticationToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authentication_token: Option<String>,
    /// <p>The ID of the folder.</p>
    #[serde(rename = "FolderId")]
    pub folder_id: String,
}

#[derive(Default, Debug, Clone, Serialize)]
pub struct DeleteLabelsRequest {
    /// <p>Amazon WorkDocs authentication token. Do not set this field when using administrative API actions, as in accessing the API using AWS credentials.</p>
    #[serde(rename = "AuthenticationToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authentication_token: Option<String>,
    /// <p>Flag to request removal of all labels from the specified resource.</p>
    #[serde(rename = "DeleteAll")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delete_all: Option<bool>,
    /// <p>List of labels to delete from the resource.</p>
    #[serde(rename = "Labels")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub labels: Option<Vec<String>>,
    /// <p>The ID of the resource.</p>
    #[serde(rename = "ResourceId")]
    pub resource_id: String,
}

#[derive(Default, Debug, Clone, Deserialize)]
pub struct DeleteLabelsResponse {}

#[derive(Default, Debug, Clone, Serialize)]
pub struct DeleteNotificationSubscriptionRequest {
    /// <p>The ID of the organization.</p>
    #[serde(rename = "OrganizationId")]
    pub organization_id: String,
    /// <p>The ID of the subscription.</p>
    #[serde(rename = "SubscriptionId")]
    pub subscription_id: String,
}

#[derive(Default, Debug, Clone, Serialize)]
pub struct DeleteUserRequest {
    /// <p>Amazon WorkDocs authentication token. Do not set this field when using administrative API actions, as in accessing the API using AWS credentials.</p>
    #[serde(rename = "AuthenticationToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authentication_token: Option<String>,
    /// <p>The ID of the user.</p>
    #[serde(rename = "UserId")]
    pub user_id: String,
}

#[derive(Default, Debug, Clone, Serialize)]
pub struct DescribeActivitiesRequest {
    /// <p>Amazon WorkDocs authentication token. Do not set this field when using administrative API actions, as in accessing the API using AWS credentials.</p>
    #[serde(rename = "AuthenticationToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authentication_token: Option<String>,
    /// <p>The timestamp that determines the end time of the activities. The response includes the activities performed before the specified timestamp.</p>
    #[serde(rename = "EndTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<f64>,
    /// <p>The maximum number of items to return.</p>
    #[serde(rename = "Limit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// <p>The marker for the next set of results.</p>
    #[serde(rename = "Marker")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
    /// <p>The ID of the organization. This is a mandatory parameter when using administrative API (SigV4) requests.</p>
    #[serde(rename = "OrganizationId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub organization_id: Option<String>,
    /// <p>The timestamp that determines the starting time of the activities. The response includes the activities performed after the specified timestamp.</p>
    #[serde(rename = "StartTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<f64>,
    /// <p>The ID of the user who performed the action. The response includes activities pertaining to this user. This is an optional parameter and is only applicable for administrative API (SigV4) requests.</p>
    #[serde(rename = "UserId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
}

#[derive(Default, Debug, Clone, Deserialize)]
pub struct DescribeActivitiesResponse {
    /// <p>The marker for the next set of results.</p>
    #[serde(rename = "Marker")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
    /// <p>The list of activities for the specified user and time period.</p>
    #[serde(rename = "UserActivities")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_activities: Option<Vec<Activity>>,
}

#[derive(Default, Debug, Clone, Serialize)]
pub struct DescribeCommentsRequest {
    /// <p>Amazon WorkDocs authentication token. Do not set this field when using administrative API actions, as in accessing the API using AWS credentials.</p>
    #[serde(rename = "AuthenticationToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authentication_token: Option<String>,
    /// <p>The ID of the document.</p>
    #[serde(rename = "DocumentId")]
    pub document_id: String,
    /// <p>The maximum number of items to return.</p>
    #[serde(rename = "Limit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// <p>The marker for the next set of results. This marker was received from a previous call.</p>
    #[serde(rename = "Marker")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
    /// <p>The ID of the document version.</p>
    #[serde(rename = "VersionId")]
    pub version_id: String,
}

#[derive(Default, Debug, Clone, Deserialize)]
pub struct DescribeCommentsResponse {
    /// <p>The list of comments for the specified document version.</p>
    #[serde(rename = "Comments")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comments: Option<Vec<Comment>>,
    /// <p>The marker for the next set of results. This marker was received from a previous call.</p>
    #[serde(rename = "Marker")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
}

#[derive(Default, Debug, Clone, Serialize)]
pub struct DescribeDocumentVersionsRequest {
    /// <p>Amazon WorkDocs authentication token. Do not set this field when using administrative API actions, as in accessing the API using AWS credentials.</p>
    #[serde(rename = "AuthenticationToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authentication_token: Option<String>,
    /// <p>The ID of the document.</p>
    #[serde(rename = "DocumentId")]
    pub document_id: String,
    /// <p>Specify "SOURCE" to include initialized versions and a URL for the source document.</p>
    #[serde(rename = "Fields")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fields: Option<String>,
    /// <p>A comma-separated list of values. Specify "INITIALIZED" to include incomplete versions.</p>
    #[serde(rename = "Include")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include: Option<String>,
    /// <p>The maximum number of versions to return with this call.</p>
    #[serde(rename = "Limit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// <p>The marker for the next set of results. (You received this marker from a previous call.)</p>
    #[serde(rename = "Marker")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
}

#[derive(Default, Debug, Clone, Deserialize)]
pub struct DescribeDocumentVersionsResponse {
    /// <p>The document versions.</p>
    #[serde(rename = "DocumentVersions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document_versions: Option<Vec<DocumentVersionMetadata>>,
    /// <p>The marker to use when requesting the next set of results. If there are no additional results, the string is empty.</p>
    #[serde(rename = "Marker")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
}

#[derive(Default, Debug, Clone, Serialize)]
pub struct DescribeFolderContentsRequest {
    /// <p>Amazon WorkDocs authentication token. Do not set this field when using administrative API actions, as in accessing the API using AWS credentials.</p>
    #[serde(rename = "AuthenticationToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authentication_token: Option<String>,
    /// <p>The ID of the folder.</p>
    #[serde(rename = "FolderId")]
    pub folder_id: String,
    /// <p>The contents to include. Specify "INITIALIZED" to include initialized documents.</p>
    #[serde(rename = "Include")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include: Option<String>,
    /// <p>The maximum number of items to return with this call.</p>
    #[serde(rename = "Limit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// <p>The marker for the next set of results. This marker was received from a previous call.</p>
    #[serde(rename = "Marker")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
    /// <p>The order for the contents of the folder.</p>
    #[serde(rename = "Order")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order: Option<String>,
    /// <p>The sorting criteria.</p>
    #[serde(rename = "Sort")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort: Option<String>,
    /// <p>The type of items.</p>
    #[serde(rename = "Type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}

#[derive(Default, Debug, Clone, Deserialize)]
pub struct DescribeFolderContentsResponse {
    /// <p>The documents in the specified folder.</p>
    #[serde(rename = "Documents")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub documents: Option<Vec<DocumentMetadata>>,
    /// <p>The subfolders in the specified folder.</p>
    #[serde(rename = "Folders")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub folders: Option<Vec<FolderMetadata>>,
    /// <p>The marker to use when requesting the next set of results. If there are no additional results, the string is empty.</p>
    #[serde(rename = "Marker")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
}

#[derive(Default, Debug, Clone, Serialize)]
pub struct DescribeGroupsRequest {
    /// <p>Amazon WorkDocs authentication token. Do not set this field when using administrative API actions, as in accessing the API using AWS credentials.</p>
    #[serde(rename = "AuthenticationToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authentication_token: Option<String>,
    /// <p>The maximum number of items to return with this call.</p>
    #[serde(rename = "Limit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// <p>The marker for the next set of results. (You received this marker from a previous call.)</p>
    #[serde(rename = "Marker")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
    /// <p>The ID of the organization.</p>
    #[serde(rename = "OrganizationId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub organization_id: Option<String>,
    /// <p>A query to describe groups by group name.</p>
    #[serde(rename = "SearchQuery")]
    pub search_query: String,
}

#[derive(Default, Debug, Clone, Deserialize)]
pub struct DescribeGroupsResponse {
    /// <p>The list of groups.</p>
    #[serde(rename = "Groups")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub groups: Option<Vec<GroupMetadata>>,
    /// <p>The marker to use when requesting the next set of results. If there are no additional results, the string is empty.</p>
    #[serde(rename = "Marker")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
}

#[derive(Default, Debug, Clone, Serialize)]
pub struct DescribeNotificationSubscriptionsRequest {
    /// <p>The maximum number of items to return with this call.</p>
    #[serde(rename = "Limit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// <p>The marker for the next set of results. (You received this marker from a previous call.)</p>
    #[serde(rename = "Marker")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
    /// <p>The ID of the organization.</p>
    #[serde(rename = "OrganizationId")]
    pub organization_id: String,
}

#[derive(Default, Debug, Clone, Deserialize)]
pub struct DescribeNotificationSubscriptionsResponse {
    /// <p>The marker to use when requesting the next set of results. If there are no additional results, the string is empty.</p>
    #[serde(rename = "Marker")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
    /// <p>The subscriptions.</p>
    #[serde(rename = "Subscriptions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscriptions: Option<Vec<Subscription>>,
}

#[derive(Default, Debug, Clone, Serialize)]
pub struct DescribeResourcePermissionsRequest {
    /// <p>Amazon WorkDocs authentication token. Do not set this field when using administrative API actions, as in accessing the API using AWS credentials.</p>
    #[serde(rename = "AuthenticationToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authentication_token: Option<String>,
    /// <p>The maximum number of items to return with this call.</p>
    #[serde(rename = "Limit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// <p>The marker for the next set of results. (You received this marker from a previous call)</p>
    #[serde(rename = "Marker")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
    /// <p>The ID of the principal to filter permissions by.</p>
    #[serde(rename = "PrincipalId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub principal_id: Option<String>,
    /// <p>The ID of the resource.</p>
    #[serde(rename = "ResourceId")]
    pub resource_id: String,
}

#[derive(Default, Debug, Clone, Deserialize)]
pub struct DescribeResourcePermissionsResponse {
    /// <p>The marker to use when requesting the next set of results. If there are no additional results, the string is empty.</p>
    #[serde(rename = "Marker")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
    /// <p>The principals.</p>
    #[serde(rename = "Principals")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub principals: Option<Vec<Principal>>,
}

#[derive(Default, Debug, Clone, Serialize)]
pub struct DescribeRootFoldersRequest {
    /// <p>Amazon WorkDocs authentication token. Do not set this field when using administrative API actions, as in accessing the API using AWS credentials.</p>
    #[serde(rename = "AuthenticationToken")]
    pub authentication_token: String,
    /// <p>The maximum number of items to return.</p>
    #[serde(rename = "Limit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// <p>The marker for the next set of results. (You received this marker from a previous call.)</p>
    #[serde(rename = "Marker")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
}

#[derive(Default, Debug, Clone, Deserialize)]
pub struct DescribeRootFoldersResponse {
    /// <p>The user's special folders.</p>
    #[serde(rename = "Folders")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub folders: Option<Vec<FolderMetadata>>,
    /// <p>The marker for the next set of results.</p>
    #[serde(rename = "Marker")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
}

#[derive(Default, Debug, Clone, Serialize)]
pub struct DescribeUsersRequest {
    /// <p>Amazon WorkDocs authentication token. Do not set this field when using administrative API actions, as in accessing the API using AWS credentials.</p>
    #[serde(rename = "AuthenticationToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authentication_token: Option<String>,
    /// <p>A comma-separated list of values. Specify "STORAGE_METADATA" to include the user storage quota and utilization information.</p>
    #[serde(rename = "Fields")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fields: Option<String>,
    /// <p>The state of the users. Specify "ALL" to include inactive users.</p>
    #[serde(rename = "Include")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include: Option<String>,
    /// <p>The maximum number of items to return.</p>
    #[serde(rename = "Limit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// <p>The marker for the next set of results. (You received this marker from a previous call.)</p>
    #[serde(rename = "Marker")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
    /// <p>The order for the results.</p>
    #[serde(rename = "Order")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order: Option<String>,
    /// <p>The ID of the organization.</p>
    #[serde(rename = "OrganizationId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub organization_id: Option<String>,
    /// <p>A query to filter users by user name.</p>
    #[serde(rename = "Query")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query: Option<String>,
    /// <p>The sorting criteria.</p>
    #[serde(rename = "Sort")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort: Option<String>,
    /// <p>The IDs of the users.</p>
    #[serde(rename = "UserIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_ids: Option<String>,
}

#[derive(Default, Debug, Clone, Deserialize)]
pub struct DescribeUsersResponse {
    /// <p>The marker to use when requesting the next set of results. If there are no additional results, the string is empty.</p>
    #[serde(rename = "Marker")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
    /// <p>The users.</p>
    #[serde(rename = "Users")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub users: Option<Vec<User>>,
}

/// <p>Describes the document.</p>
#[derive(Default, Debug, Clone, Deserialize)]
pub struct DocumentMetadata {
    /// <p>The time when the document was created.</p>
    #[serde(rename = "CreatedTimestamp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_timestamp: Option<f64>,
    /// <p>The ID of the creator.</p>
    #[serde(rename = "CreatorId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creator_id: Option<String>,
    /// <p>The ID of the document.</p>
    #[serde(rename = "Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// <p>List of labels on the document.</p>
    #[serde(rename = "Labels")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub labels: Option<Vec<String>>,
    /// <p>The latest version of the document.</p>
    #[serde(rename = "LatestVersionMetadata")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latest_version_metadata: Option<DocumentVersionMetadata>,
    /// <p>The time when the document was updated.</p>
    #[serde(rename = "ModifiedTimestamp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modified_timestamp: Option<f64>,
    /// <p>The ID of the parent folder.</p>
    #[serde(rename = "ParentFolderId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_folder_id: Option<String>,
    /// <p>The resource state.</p>
    #[serde(rename = "ResourceState")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_state: Option<String>,
}

/// <p>Describes a version of a document.</p>
#[derive(Default, Debug, Clone, Deserialize)]
pub struct DocumentVersionMetadata {
    /// <p>The timestamp when the content of the document was originally created.</p>
    #[serde(rename = "ContentCreatedTimestamp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_created_timestamp: Option<f64>,
    /// <p>The timestamp when the content of the document was modified.</p>
    #[serde(rename = "ContentModifiedTimestamp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_modified_timestamp: Option<f64>,
    /// <p>The content type of the document.</p>
    #[serde(rename = "ContentType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_type: Option<String>,
    /// <p>The timestamp when the document was first uploaded.</p>
    #[serde(rename = "CreatedTimestamp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_timestamp: Option<f64>,
    /// <p>The ID of the creator.</p>
    #[serde(rename = "CreatorId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creator_id: Option<String>,
    /// <p>The ID of the version.</p>
    #[serde(rename = "Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// <p>The timestamp when the document was last uploaded.</p>
    #[serde(rename = "ModifiedTimestamp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modified_timestamp: Option<f64>,
    /// <p>The name of the version.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The signature of the document.</p>
    #[serde(rename = "Signature")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signature: Option<String>,
    /// <p>The size of the document, in bytes.</p>
    #[serde(rename = "Size")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<i64>,
    /// <p>The source of the document.</p>
    #[serde(rename = "Source")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<::std::collections::HashMap<String, String>>,
    /// <p>The status of the document.</p>
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// <p>The thumbnail of the document.</p>
    #[serde(rename = "Thumbnail")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumbnail: Option<::std::collections::HashMap<String, String>>,
}

/// <p>Describes a folder.</p>
#[derive(Default, Debug, Clone, Deserialize)]
pub struct FolderMetadata {
    /// <p>The time when the folder was created.</p>
    #[serde(rename = "CreatedTimestamp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_timestamp: Option<f64>,
    /// <p>The ID of the creator.</p>
    #[serde(rename = "CreatorId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creator_id: Option<String>,
    /// <p>The ID of the folder.</p>
    #[serde(rename = "Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// <p>List of labels on the folder.</p>
    #[serde(rename = "Labels")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub labels: Option<Vec<String>>,
    /// <p>The size of the latest version of the folder metadata.</p>
    #[serde(rename = "LatestVersionSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latest_version_size: Option<i64>,
    /// <p>The time when the folder was updated.</p>
    #[serde(rename = "ModifiedTimestamp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modified_timestamp: Option<f64>,
    /// <p>The name of the folder.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The ID of the parent folder.</p>
    #[serde(rename = "ParentFolderId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_folder_id: Option<String>,
    /// <p>The resource state of the folder.</p>
    #[serde(rename = "ResourceState")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_state: Option<String>,
    /// <p>The unique identifier created from the subfolders and documents of the folder.</p>
    #[serde(rename = "Signature")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signature: Option<String>,
    /// <p>The size of the folder metadata.</p>
    #[serde(rename = "Size")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<i64>,
}

#[derive(Default, Debug, Clone, Serialize)]
pub struct GetCurrentUserRequest {
    /// <p>Amazon WorkDocs authentication token. Do not set this field when using administrative API actions, as in accessing the API using AWS credentials.</p>
    #[serde(rename = "AuthenticationToken")]
    pub authentication_token: String,
}

#[derive(Default, Debug, Clone, Deserialize)]
pub struct GetCurrentUserResponse {
    /// <p>Metadata of the user.</p>
    #[serde(rename = "User")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<User>,
}

#[derive(Default, Debug, Clone, Serialize)]
pub struct GetDocumentPathRequest {
    /// <p>Amazon WorkDocs authentication token. Do not set this field when using administrative API actions, as in accessing the API using AWS credentials.</p>
    #[serde(rename = "AuthenticationToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authentication_token: Option<String>,
    /// <p>The ID of the document.</p>
    #[serde(rename = "DocumentId")]
    pub document_id: String,
    /// <p>A comma-separated list of values. Specify <code>NAME</code> to include the names of the parent folders.</p>
    #[serde(rename = "Fields")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fields: Option<String>,
    /// <p>The maximum number of levels in the hierarchy to return.</p>
    #[serde(rename = "Limit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// <p>This value is not supported.</p>
    #[serde(rename = "Marker")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
}

#[derive(Default, Debug, Clone, Deserialize)]
pub struct GetDocumentPathResponse {
    /// <p>The path information.</p>
    #[serde(rename = "Path")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<ResourcePath>,
}

#[derive(Default, Debug, Clone, Serialize)]
pub struct GetDocumentRequest {
    /// <p>Amazon WorkDocs authentication token. Do not set this field when using administrative API actions, as in accessing the API using AWS credentials.</p>
    #[serde(rename = "AuthenticationToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authentication_token: Option<String>,
    /// <p>The ID of the document.</p>
    #[serde(rename = "DocumentId")]
    pub document_id: String,
    /// <p>Set this to <code>TRUE</code> to include custom metadata in the response.</p>
    #[serde(rename = "IncludeCustomMetadata")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_custom_metadata: Option<bool>,
}

#[derive(Default, Debug, Clone, Deserialize)]
pub struct GetDocumentResponse {
    /// <p>The custom metadata on the document.</p>
    #[serde(rename = "CustomMetadata")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_metadata: Option<::std::collections::HashMap<String, String>>,
    /// <p>The metadata details of the document.</p>
    #[serde(rename = "Metadata")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<DocumentMetadata>,
}

#[derive(Default, Debug, Clone, Serialize)]
pub struct GetDocumentVersionRequest {
    /// <p>Amazon WorkDocs authentication token. Do not set this field when using administrative API actions, as in accessing the API using AWS credentials.</p>
    #[serde(rename = "AuthenticationToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authentication_token: Option<String>,
    /// <p>The ID of the document.</p>
    #[serde(rename = "DocumentId")]
    pub document_id: String,
    /// <p>A comma-separated list of values. Specify "SOURCE" to include a URL for the source document.</p>
    #[serde(rename = "Fields")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fields: Option<String>,
    /// <p>Set this to TRUE to include custom metadata in the response.</p>
    #[serde(rename = "IncludeCustomMetadata")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_custom_metadata: Option<bool>,
    /// <p>The version ID of the document.</p>
    #[serde(rename = "VersionId")]
    pub version_id: String,
}

#[derive(Default, Debug, Clone, Deserialize)]
pub struct GetDocumentVersionResponse {
    /// <p>The custom metadata on the document version.</p>
    #[serde(rename = "CustomMetadata")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_metadata: Option<::std::collections::HashMap<String, String>>,
    /// <p>The version metadata.</p>
    #[serde(rename = "Metadata")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<DocumentVersionMetadata>,
}

#[derive(Default, Debug, Clone, Serialize)]
pub struct GetFolderPathRequest {
    /// <p>Amazon WorkDocs authentication token. Do not set this field when using administrative API actions, as in accessing the API using AWS credentials.</p>
    #[serde(rename = "AuthenticationToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authentication_token: Option<String>,
    /// <p>A comma-separated list of values. Specify "NAME" to include the names of the parent folders.</p>
    #[serde(rename = "Fields")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fields: Option<String>,
    /// <p>The ID of the folder.</p>
    #[serde(rename = "FolderId")]
    pub folder_id: String,
    /// <p>The maximum number of levels in the hierarchy to return.</p>
    #[serde(rename = "Limit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// <p>This value is not supported.</p>
    #[serde(rename = "Marker")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
}

#[derive(Default, Debug, Clone, Deserialize)]
pub struct GetFolderPathResponse {
    /// <p>The path information.</p>
    #[serde(rename = "Path")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<ResourcePath>,
}

#[derive(Default, Debug, Clone, Serialize)]
pub struct GetFolderRequest {
    /// <p>Amazon WorkDocs authentication token. Do not set this field when using administrative API actions, as in accessing the API using AWS credentials.</p>
    #[serde(rename = "AuthenticationToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authentication_token: Option<String>,
    /// <p>The ID of the folder.</p>
    #[serde(rename = "FolderId")]
    pub folder_id: String,
    /// <p>Set to TRUE to include custom metadata in the response.</p>
    #[serde(rename = "IncludeCustomMetadata")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_custom_metadata: Option<bool>,
}

#[derive(Default, Debug, Clone, Deserialize)]
pub struct GetFolderResponse {
    /// <p>The custom metadata on the folder.</p>
    #[serde(rename = "CustomMetadata")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_metadata: Option<::std::collections::HashMap<String, String>>,
    /// <p>The metadata of the folder.</p>
    #[serde(rename = "Metadata")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<FolderMetadata>,
}

/// <p>Describes the metadata of a user group.</p>
#[derive(Default, Debug, Clone, Deserialize)]
pub struct GroupMetadata {
    /// <p>The ID of the user group.</p>
    #[serde(rename = "Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// <p>The name of the group.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Default, Debug, Clone, Serialize)]
pub struct InitiateDocumentVersionUploadRequest {
    /// <p>Amazon WorkDocs authentication token. Do not set this field when using administrative API actions, as in accessing the API using AWS credentials.</p>
    #[serde(rename = "AuthenticationToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authentication_token: Option<String>,
    /// <p>The timestamp when the content of the document was originally created.</p>
    #[serde(rename = "ContentCreatedTimestamp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_created_timestamp: Option<f64>,
    /// <p>The timestamp when the content of the document was modified.</p>
    #[serde(rename = "ContentModifiedTimestamp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_modified_timestamp: Option<f64>,
    /// <p>The content type of the document.</p>
    #[serde(rename = "ContentType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_type: Option<String>,
    /// <p>The size of the document, in bytes.</p>
    #[serde(rename = "DocumentSizeInBytes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document_size_in_bytes: Option<i64>,
    /// <p>The ID of the document.</p>
    #[serde(rename = "Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// <p>The name of the document.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The ID of the parent folder.</p>
    #[serde(rename = "ParentFolderId")]
    pub parent_folder_id: String,
}

#[derive(Default, Debug, Clone, Deserialize)]
pub struct InitiateDocumentVersionUploadResponse {
    /// <p>The document metadata.</p>
    #[serde(rename = "Metadata")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<DocumentMetadata>,
    /// <p>The upload metadata.</p>
    #[serde(rename = "UploadMetadata")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub upload_metadata: Option<UploadMetadata>,
}

/// <p>Set of options which defines notification preferences of given action.</p>
#[derive(Default, Debug, Clone, Serialize)]
pub struct NotificationOptions {
    /// <p>Text value to be included in the email body.</p>
    #[serde(rename = "EmailMessage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email_message: Option<String>,
    /// <p>Boolean value to indicate an email notification should be sent to the receipients.</p>
    #[serde(rename = "SendEmail")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub send_email: Option<bool>,
}

/// <p>Describes the users or user groups.</p>
#[derive(Default, Debug, Clone, Deserialize)]
pub struct Participants {
    /// <p>The list of user groups.</p>
    #[serde(rename = "Groups")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub groups: Option<Vec<GroupMetadata>>,
    /// <p>The list of users.</p>
    #[serde(rename = "Users")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub users: Option<Vec<UserMetadata>>,
}

/// <p>Describes the permissions.</p>
#[derive(Default, Debug, Clone, Deserialize)]
pub struct PermissionInfo {
    /// <p>The role of the user.</p>
    #[serde(rename = "Role")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role: Option<String>,
    /// <p>The type of permissions.</p>
    #[serde(rename = "Type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}

/// <p>Describes a resource.</p>
#[derive(Default, Debug, Clone, Deserialize)]
pub struct Principal {
    /// <p>The ID of the resource.</p>
    #[serde(rename = "Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// <p>The permission information for the resource.</p>
    #[serde(rename = "Roles")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub roles: Option<Vec<PermissionInfo>>,
    /// <p>The type of resource.</p>
    #[serde(rename = "Type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}

#[derive(Default, Debug, Clone, Serialize)]
pub struct RemoveAllResourcePermissionsRequest {
    /// <p>Amazon WorkDocs authentication token. Do not set this field when using administrative API actions, as in accessing the API using AWS credentials.</p>
    #[serde(rename = "AuthenticationToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authentication_token: Option<String>,
    /// <p>The ID of the resource.</p>
    #[serde(rename = "ResourceId")]
    pub resource_id: String,
}

#[derive(Default, Debug, Clone, Serialize)]
pub struct RemoveResourcePermissionRequest {
    /// <p>Amazon WorkDocs authentication token. Do not set this field when using administrative API actions, as in accessing the API using AWS credentials.</p>
    #[serde(rename = "AuthenticationToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authentication_token: Option<String>,
    /// <p>The principal ID of the resource.</p>
    #[serde(rename = "PrincipalId")]
    pub principal_id: String,
    /// <p>The principal type of the resource.</p>
    #[serde(rename = "PrincipalType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub principal_type: Option<String>,
    /// <p>The ID of the resource.</p>
    #[serde(rename = "ResourceId")]
    pub resource_id: String,
}

/// <p>Describes the metadata of a resource.</p>
#[derive(Default, Debug, Clone, Deserialize)]
pub struct ResourceMetadata {
    /// <p>The ID of the resource.</p>
    #[serde(rename = "Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// <p>The name of the resource.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The original name of the resource before a rename operation.</p>
    #[serde(rename = "OriginalName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub original_name: Option<String>,
    /// <p>The owner of the resource.</p>
    #[serde(rename = "Owner")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner: Option<UserMetadata>,
    /// <p>The parent ID of the resource before a rename operation.</p>
    #[serde(rename = "ParentId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_id: Option<String>,
    /// <p>The type of resource.</p>
    #[serde(rename = "Type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    /// <p>The version ID of the resource. This is an optional field and is filled for action on document version.</p>
    #[serde(rename = "VersionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version_id: Option<String>,
}

/// <p>Describes the path information of a resource.</p>
#[derive(Default, Debug, Clone, Deserialize)]
pub struct ResourcePath {
    /// <p>The components of the resource path.</p>
    #[serde(rename = "Components")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub components: Option<Vec<ResourcePathComponent>>,
}

/// <p>Describes the resource path.</p>
#[derive(Default, Debug, Clone, Deserialize)]
pub struct ResourcePathComponent {
    /// <p>The ID of the resource path.</p>
    #[serde(rename = "Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// <p>The name of the resource path.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

/// <p>Describes the recipient type and ID, if available.</p>
#[derive(Default, Debug, Clone, Serialize)]
pub struct SharePrincipal {
    /// <p>The ID of the recipient.</p>
    #[serde(rename = "Id")]
    pub id: String,
    /// <p>The role of the recipient.</p>
    #[serde(rename = "Role")]
    pub role: String,
    /// <p>The type of the recipient.</p>
    #[serde(rename = "Type")]
    pub type_: String,
}

/// <p>Describes the share results of a resource.</p>
#[derive(Default, Debug, Clone, Deserialize)]
pub struct ShareResult {
    /// <p>The ID of the principal.</p>
    #[serde(rename = "PrincipalId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub principal_id: Option<String>,
    /// <p>The role.</p>
    #[serde(rename = "Role")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role: Option<String>,
    /// <p>The ID of the resource that was shared.</p>
    #[serde(rename = "ShareId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub share_id: Option<String>,
    /// <p>The status.</p>
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// <p>The status message.</p>
    #[serde(rename = "StatusMessage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_message: Option<String>,
}

/// <p>Describes the storage for a user.</p>
#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct StorageRuleType {
    /// <p>The amount of storage allocated, in bytes.</p>
    #[serde(rename = "StorageAllocatedInBytes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage_allocated_in_bytes: Option<i64>,
    /// <p>The type of storage.</p>
    #[serde(rename = "StorageType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage_type: Option<String>,
}

/// <p>Describes a subscription.</p>
#[derive(Default, Debug, Clone, Deserialize)]
pub struct Subscription {
    /// <p>The endpoint of the subscription.</p>
    #[serde(rename = "EndPoint")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_point: Option<String>,
    /// <p>The protocol of the subscription.</p>
    #[serde(rename = "Protocol")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protocol: Option<String>,
    /// <p>The ID of the subscription.</p>
    #[serde(rename = "SubscriptionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscription_id: Option<String>,
}

#[derive(Default, Debug, Clone, Serialize)]
pub struct UpdateDocumentRequest {
    /// <p>Amazon WorkDocs authentication token. Do not set this field when using administrative API actions, as in accessing the API using AWS credentials.</p>
    #[serde(rename = "AuthenticationToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authentication_token: Option<String>,
    /// <p>The ID of the document.</p>
    #[serde(rename = "DocumentId")]
    pub document_id: String,
    /// <p>The name of the document.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The ID of the parent folder.</p>
    #[serde(rename = "ParentFolderId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_folder_id: Option<String>,
    /// <p>The resource state of the document. Only ACTIVE and RECYCLED are supported.</p>
    #[serde(rename = "ResourceState")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_state: Option<String>,
}

#[derive(Default, Debug, Clone, Serialize)]
pub struct UpdateDocumentVersionRequest {
    /// <p>Amazon WorkDocs authentication token. Do not set this field when using administrative API actions, as in accessing the API using AWS credentials.</p>
    #[serde(rename = "AuthenticationToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authentication_token: Option<String>,
    /// <p>The ID of the document.</p>
    #[serde(rename = "DocumentId")]
    pub document_id: String,
    /// <p>The version ID of the document.</p>
    #[serde(rename = "VersionId")]
    pub version_id: String,
    /// <p>The status of the version.</p>
    #[serde(rename = "VersionStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version_status: Option<String>,
}

#[derive(Default, Debug, Clone, Serialize)]
pub struct UpdateFolderRequest {
    /// <p>Amazon WorkDocs authentication token. Do not set this field when using administrative API actions, as in accessing the API using AWS credentials.</p>
    #[serde(rename = "AuthenticationToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authentication_token: Option<String>,
    /// <p>The ID of the folder.</p>
    #[serde(rename = "FolderId")]
    pub folder_id: String,
    /// <p>The name of the folder.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The ID of the parent folder.</p>
    #[serde(rename = "ParentFolderId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_folder_id: Option<String>,
    /// <p>The resource state of the folder. Only ACTIVE and RECYCLED are accepted values from the API.</p>
    #[serde(rename = "ResourceState")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_state: Option<String>,
}

#[derive(Default, Debug, Clone, Serialize)]
pub struct UpdateUserRequest {
    /// <p>Amazon WorkDocs authentication token. Do not set this field when using administrative API actions, as in accessing the API using AWS credentials.</p>
    #[serde(rename = "AuthenticationToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authentication_token: Option<String>,
    /// <p>The given name of the user.</p>
    #[serde(rename = "GivenName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub given_name: Option<String>,
    /// <p>Boolean value to determine whether the user is granted Poweruser privileges.</p>
    #[serde(rename = "GrantPoweruserPrivileges")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub grant_poweruser_privileges: Option<String>,
    /// <p>The locale of the user.</p>
    #[serde(rename = "Locale")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub locale: Option<String>,
    /// <p>The amount of storage for the user.</p>
    #[serde(rename = "StorageRule")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage_rule: Option<StorageRuleType>,
    /// <p>The surname of the user.</p>
    #[serde(rename = "Surname")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub surname: Option<String>,
    /// <p>The time zone ID of the user.</p>
    #[serde(rename = "TimeZoneId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_zone_id: Option<String>,
    /// <p>The type of the user.</p>
    #[serde(rename = "Type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    /// <p>The ID of the user.</p>
    #[serde(rename = "UserId")]
    pub user_id: String,
}

#[derive(Default, Debug, Clone, Deserialize)]
pub struct UpdateUserResponse {
    /// <p>The user information.</p>
    #[serde(rename = "User")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<User>,
}

/// <p>Describes the upload.</p>
#[derive(Default, Debug, Clone, Deserialize)]
pub struct UploadMetadata {
    /// <p>The signed headers.</p>
    #[serde(rename = "SignedHeaders")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signed_headers: Option<::std::collections::HashMap<String, String>>,
    /// <p>The URL of the upload.</p>
    #[serde(rename = "UploadUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub upload_url: Option<String>,
}

/// <p>Describes a user.</p>
#[derive(Default, Debug, Clone, Deserialize)]
pub struct User {
    /// <p>The time when the user was created.</p>
    #[serde(rename = "CreatedTimestamp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_timestamp: Option<f64>,
    /// <p>The email address of the user.</p>
    #[serde(rename = "EmailAddress")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email_address: Option<String>,
    /// <p>The given name of the user.</p>
    #[serde(rename = "GivenName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub given_name: Option<String>,
    /// <p>The ID of the user.</p>
    #[serde(rename = "Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// <p>The locale of the user.</p>
    #[serde(rename = "Locale")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub locale: Option<String>,
    /// <p>The time when the user was modified.</p>
    #[serde(rename = "ModifiedTimestamp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modified_timestamp: Option<f64>,
    /// <p>The ID of the organization.</p>
    #[serde(rename = "OrganizationId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub organization_id: Option<String>,
    /// <p>The ID of the recycle bin folder.</p>
    #[serde(rename = "RecycleBinFolderId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recycle_bin_folder_id: Option<String>,
    /// <p>The ID of the root folder.</p>
    #[serde(rename = "RootFolderId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub root_folder_id: Option<String>,
    /// <p>The status of the user.</p>
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// <p>The storage for the user.</p>
    #[serde(rename = "Storage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage: Option<UserStorageMetadata>,
    /// <p>The surname of the user.</p>
    #[serde(rename = "Surname")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub surname: Option<String>,
    /// <p>The time zone ID of the user.</p>
    #[serde(rename = "TimeZoneId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_zone_id: Option<String>,
    /// <p>The type of user.</p>
    #[serde(rename = "Type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    /// <p>The login name of the user.</p>
    #[serde(rename = "Username")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
}

/// <p>Describes the metadata of the user.</p>
#[derive(Default, Debug, Clone, Deserialize)]
pub struct UserMetadata {
    /// <p>The email address of the user.</p>
    #[serde(rename = "EmailAddress")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email_address: Option<String>,
    /// <p>The given name of the user before a rename operation.</p>
    #[serde(rename = "GivenName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub given_name: Option<String>,
    /// <p>The ID of the user.</p>
    #[serde(rename = "Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// <p>The surname of the user.</p>
    #[serde(rename = "Surname")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub surname: Option<String>,
    /// <p>The name of the user.</p>
    #[serde(rename = "Username")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
}

/// <p>Describes the storage for a user.</p>
#[derive(Default, Debug, Clone, Deserialize)]
pub struct UserStorageMetadata {
    /// <p>The storage for a user.</p>
    #[serde(rename = "StorageRule")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage_rule: Option<StorageRuleType>,
    /// <p>The amount of storage used, in bytes.</p>
    #[serde(rename = "StorageUtilizedInBytes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage_utilized_in_bytes: Option<i64>,
}

/// Errors returned by AbortDocumentVersionUpload
#[derive(Debug, PartialEq)]
pub enum AbortDocumentVersionUploadError {
    /// <p>The resource does not exist.</p>
    EntityNotExists(String),
    /// <p>The AWS Directory Service cannot reach an on-premises instance. Or a dependency under the control of the organization is failing, such as a connected Active Directory.</p>
    FailedDependency(String),
    /// <p>The specified document version is not in the INITIALIZED state.</p>
    ProhibitedState(String),
    /// <p>One or more of the dependencies is unavailable.</p>
    ServiceUnavailable(String),
    /// <p>The operation is not permitted.</p>
    UnauthorizedOperation(String),
    /// <p>The caller does not have access to perform the action on the resource.</p>
    UnauthorizedResourceAccess(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl AbortDocumentVersionUploadError {
    pub fn from_body(body: &str) -> AbortDocumentVersionUploadError {
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
                    "EntityNotExistsException" => AbortDocumentVersionUploadError::EntityNotExists(
                        String::from(error_message),
                    ),
                    "FailedDependencyException" => {
                        AbortDocumentVersionUploadError::FailedDependency(String::from(
                            error_message,
                        ))
                    }
                    "ProhibitedStateException" => AbortDocumentVersionUploadError::ProhibitedState(
                        String::from(error_message),
                    ),
                    "ServiceUnavailableException" => {
                        AbortDocumentVersionUploadError::ServiceUnavailable(String::from(
                            error_message,
                        ))
                    }
                    "UnauthorizedOperationException" => {
                        AbortDocumentVersionUploadError::UnauthorizedOperation(String::from(
                            error_message,
                        ))
                    }
                    "UnauthorizedResourceAccessException" => {
                        AbortDocumentVersionUploadError::UnauthorizedResourceAccess(String::from(
                            error_message,
                        ))
                    }
                    "ValidationException" => {
                        AbortDocumentVersionUploadError::Validation(error_message.to_string())
                    }
                    _ => AbortDocumentVersionUploadError::Unknown(String::from(body)),
                }
            }
            Err(_) => AbortDocumentVersionUploadError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for AbortDocumentVersionUploadError {
    fn from(err: serde_json::error::Error) -> AbortDocumentVersionUploadError {
        AbortDocumentVersionUploadError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for AbortDocumentVersionUploadError {
    fn from(err: CredentialsError) -> AbortDocumentVersionUploadError {
        AbortDocumentVersionUploadError::Credentials(err)
    }
}
impl From<HttpDispatchError> for AbortDocumentVersionUploadError {
    fn from(err: HttpDispatchError) -> AbortDocumentVersionUploadError {
        AbortDocumentVersionUploadError::HttpDispatch(err)
    }
}
impl From<io::Error> for AbortDocumentVersionUploadError {
    fn from(err: io::Error) -> AbortDocumentVersionUploadError {
        AbortDocumentVersionUploadError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for AbortDocumentVersionUploadError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for AbortDocumentVersionUploadError {
    fn description(&self) -> &str {
        match *self {
            AbortDocumentVersionUploadError::EntityNotExists(ref cause) => cause,
            AbortDocumentVersionUploadError::FailedDependency(ref cause) => cause,
            AbortDocumentVersionUploadError::ProhibitedState(ref cause) => cause,
            AbortDocumentVersionUploadError::ServiceUnavailable(ref cause) => cause,
            AbortDocumentVersionUploadError::UnauthorizedOperation(ref cause) => cause,
            AbortDocumentVersionUploadError::UnauthorizedResourceAccess(ref cause) => cause,
            AbortDocumentVersionUploadError::Validation(ref cause) => cause,
            AbortDocumentVersionUploadError::Credentials(ref err) => err.description(),
            AbortDocumentVersionUploadError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            AbortDocumentVersionUploadError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by ActivateUser
#[derive(Debug, PartialEq)]
pub enum ActivateUserError {
    /// <p>The resource does not exist.</p>
    EntityNotExists(String),
    /// <p>The AWS Directory Service cannot reach an on-premises instance. Or a dependency under the control of the organization is failing, such as a connected Active Directory.</p>
    FailedDependency(String),
    /// <p>One or more of the dependencies is unavailable.</p>
    ServiceUnavailable(String),
    /// <p>The operation is not permitted.</p>
    UnauthorizedOperation(String),
    /// <p>The caller does not have access to perform the action on the resource.</p>
    UnauthorizedResourceAccess(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl ActivateUserError {
    pub fn from_body(body: &str) -> ActivateUserError {
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
                    "EntityNotExistsException" => {
                        ActivateUserError::EntityNotExists(String::from(error_message))
                    }
                    "FailedDependencyException" => {
                        ActivateUserError::FailedDependency(String::from(error_message))
                    }
                    "ServiceUnavailableException" => {
                        ActivateUserError::ServiceUnavailable(String::from(error_message))
                    }
                    "UnauthorizedOperationException" => {
                        ActivateUserError::UnauthorizedOperation(String::from(error_message))
                    }
                    "UnauthorizedResourceAccessException" => {
                        ActivateUserError::UnauthorizedResourceAccess(String::from(error_message))
                    }
                    "ValidationException" => {
                        ActivateUserError::Validation(error_message.to_string())
                    }
                    _ => ActivateUserError::Unknown(String::from(body)),
                }
            }
            Err(_) => ActivateUserError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for ActivateUserError {
    fn from(err: serde_json::error::Error) -> ActivateUserError {
        ActivateUserError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for ActivateUserError {
    fn from(err: CredentialsError) -> ActivateUserError {
        ActivateUserError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ActivateUserError {
    fn from(err: HttpDispatchError) -> ActivateUserError {
        ActivateUserError::HttpDispatch(err)
    }
}
impl From<io::Error> for ActivateUserError {
    fn from(err: io::Error) -> ActivateUserError {
        ActivateUserError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for ActivateUserError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ActivateUserError {
    fn description(&self) -> &str {
        match *self {
            ActivateUserError::EntityNotExists(ref cause) => cause,
            ActivateUserError::FailedDependency(ref cause) => cause,
            ActivateUserError::ServiceUnavailable(ref cause) => cause,
            ActivateUserError::UnauthorizedOperation(ref cause) => cause,
            ActivateUserError::UnauthorizedResourceAccess(ref cause) => cause,
            ActivateUserError::Validation(ref cause) => cause,
            ActivateUserError::Credentials(ref err) => err.description(),
            ActivateUserError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            ActivateUserError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by AddResourcePermissions
#[derive(Debug, PartialEq)]
pub enum AddResourcePermissionsError {
    /// <p>The AWS Directory Service cannot reach an on-premises instance. Or a dependency under the control of the organization is failing, such as a connected Active Directory.</p>
    FailedDependency(String),
    /// <p>One or more of the dependencies is unavailable.</p>
    ServiceUnavailable(String),
    /// <p>The operation is not permitted.</p>
    UnauthorizedOperation(String),
    /// <p>The caller does not have access to perform the action on the resource.</p>
    UnauthorizedResourceAccess(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl AddResourcePermissionsError {
    pub fn from_body(body: &str) -> AddResourcePermissionsError {
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
                    "FailedDependencyException" => {
                        AddResourcePermissionsError::FailedDependency(String::from(error_message))
                    }
                    "ServiceUnavailableException" => {
                        AddResourcePermissionsError::ServiceUnavailable(String::from(error_message))
                    }
                    "UnauthorizedOperationException" => {
                        AddResourcePermissionsError::UnauthorizedOperation(String::from(
                            error_message,
                        ))
                    }
                    "UnauthorizedResourceAccessException" => {
                        AddResourcePermissionsError::UnauthorizedResourceAccess(String::from(
                            error_message,
                        ))
                    }
                    "ValidationException" => {
                        AddResourcePermissionsError::Validation(error_message.to_string())
                    }
                    _ => AddResourcePermissionsError::Unknown(String::from(body)),
                }
            }
            Err(_) => AddResourcePermissionsError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for AddResourcePermissionsError {
    fn from(err: serde_json::error::Error) -> AddResourcePermissionsError {
        AddResourcePermissionsError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for AddResourcePermissionsError {
    fn from(err: CredentialsError) -> AddResourcePermissionsError {
        AddResourcePermissionsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for AddResourcePermissionsError {
    fn from(err: HttpDispatchError) -> AddResourcePermissionsError {
        AddResourcePermissionsError::HttpDispatch(err)
    }
}
impl From<io::Error> for AddResourcePermissionsError {
    fn from(err: io::Error) -> AddResourcePermissionsError {
        AddResourcePermissionsError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for AddResourcePermissionsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for AddResourcePermissionsError {
    fn description(&self) -> &str {
        match *self {
            AddResourcePermissionsError::FailedDependency(ref cause) => cause,
            AddResourcePermissionsError::ServiceUnavailable(ref cause) => cause,
            AddResourcePermissionsError::UnauthorizedOperation(ref cause) => cause,
            AddResourcePermissionsError::UnauthorizedResourceAccess(ref cause) => cause,
            AddResourcePermissionsError::Validation(ref cause) => cause,
            AddResourcePermissionsError::Credentials(ref err) => err.description(),
            AddResourcePermissionsError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            AddResourcePermissionsError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by CreateComment
#[derive(Debug, PartialEq)]
pub enum CreateCommentError {
    /// <p>This exception is thrown when the document is locked for comments and user tries to create or delete a comment on that document.</p>
    DocumentLockedForComments(String),
    /// <p>The resource does not exist.</p>
    EntityNotExists(String),
    /// <p>The AWS Directory Service cannot reach an on-premises instance. Or a dependency under the control of the organization is failing, such as a connected Active Directory.</p>
    FailedDependency(String),
    /// <p>The specified document version is not in the INITIALIZED state.</p>
    ProhibitedState(String),
    /// <p>One or more of the dependencies is unavailable.</p>
    ServiceUnavailable(String),
    /// <p>The operation is not permitted.</p>
    UnauthorizedOperation(String),
    /// <p>The caller does not have access to perform the action on the resource.</p>
    UnauthorizedResourceAccess(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl CreateCommentError {
    pub fn from_body(body: &str) -> CreateCommentError {
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
                    "DocumentLockedForCommentsException" => {
                        CreateCommentError::DocumentLockedForComments(String::from(error_message))
                    }
                    "EntityNotExistsException" => {
                        CreateCommentError::EntityNotExists(String::from(error_message))
                    }
                    "FailedDependencyException" => {
                        CreateCommentError::FailedDependency(String::from(error_message))
                    }
                    "ProhibitedStateException" => {
                        CreateCommentError::ProhibitedState(String::from(error_message))
                    }
                    "ServiceUnavailableException" => {
                        CreateCommentError::ServiceUnavailable(String::from(error_message))
                    }
                    "UnauthorizedOperationException" => {
                        CreateCommentError::UnauthorizedOperation(String::from(error_message))
                    }
                    "UnauthorizedResourceAccessException" => {
                        CreateCommentError::UnauthorizedResourceAccess(String::from(error_message))
                    }
                    "ValidationException" => {
                        CreateCommentError::Validation(error_message.to_string())
                    }
                    _ => CreateCommentError::Unknown(String::from(body)),
                }
            }
            Err(_) => CreateCommentError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for CreateCommentError {
    fn from(err: serde_json::error::Error) -> CreateCommentError {
        CreateCommentError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for CreateCommentError {
    fn from(err: CredentialsError) -> CreateCommentError {
        CreateCommentError::Credentials(err)
    }
}
impl From<HttpDispatchError> for CreateCommentError {
    fn from(err: HttpDispatchError) -> CreateCommentError {
        CreateCommentError::HttpDispatch(err)
    }
}
impl From<io::Error> for CreateCommentError {
    fn from(err: io::Error) -> CreateCommentError {
        CreateCommentError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for CreateCommentError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateCommentError {
    fn description(&self) -> &str {
        match *self {
            CreateCommentError::DocumentLockedForComments(ref cause) => cause,
            CreateCommentError::EntityNotExists(ref cause) => cause,
            CreateCommentError::FailedDependency(ref cause) => cause,
            CreateCommentError::ProhibitedState(ref cause) => cause,
            CreateCommentError::ServiceUnavailable(ref cause) => cause,
            CreateCommentError::UnauthorizedOperation(ref cause) => cause,
            CreateCommentError::UnauthorizedResourceAccess(ref cause) => cause,
            CreateCommentError::Validation(ref cause) => cause,
            CreateCommentError::Credentials(ref err) => err.description(),
            CreateCommentError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            CreateCommentError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by CreateCustomMetadata
#[derive(Debug, PartialEq)]
pub enum CreateCustomMetadataError {
    /// <p>The limit has been reached on the number of custom properties for the specified resource.</p>
    CustomMetadataLimitExceeded(String),
    /// <p>The resource does not exist.</p>
    EntityNotExists(String),
    /// <p>The AWS Directory Service cannot reach an on-premises instance. Or a dependency under the control of the organization is failing, such as a connected Active Directory.</p>
    FailedDependency(String),
    /// <p>The specified document version is not in the INITIALIZED state.</p>
    ProhibitedState(String),
    /// <p>One or more of the dependencies is unavailable.</p>
    ServiceUnavailable(String),
    /// <p>The operation is not permitted.</p>
    UnauthorizedOperation(String),
    /// <p>The caller does not have access to perform the action on the resource.</p>
    UnauthorizedResourceAccess(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl CreateCustomMetadataError {
    pub fn from_body(body: &str) -> CreateCustomMetadataError {
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
                    "CustomMetadataLimitExceededException" => {
                        CreateCustomMetadataError::CustomMetadataLimitExceeded(String::from(
                            error_message,
                        ))
                    }
                    "EntityNotExistsException" => {
                        CreateCustomMetadataError::EntityNotExists(String::from(error_message))
                    }
                    "FailedDependencyException" => {
                        CreateCustomMetadataError::FailedDependency(String::from(error_message))
                    }
                    "ProhibitedStateException" => {
                        CreateCustomMetadataError::ProhibitedState(String::from(error_message))
                    }
                    "ServiceUnavailableException" => {
                        CreateCustomMetadataError::ServiceUnavailable(String::from(error_message))
                    }
                    "UnauthorizedOperationException" => {
                        CreateCustomMetadataError::UnauthorizedOperation(String::from(
                            error_message,
                        ))
                    }
                    "UnauthorizedResourceAccessException" => {
                        CreateCustomMetadataError::UnauthorizedResourceAccess(String::from(
                            error_message,
                        ))
                    }
                    "ValidationException" => {
                        CreateCustomMetadataError::Validation(error_message.to_string())
                    }
                    _ => CreateCustomMetadataError::Unknown(String::from(body)),
                }
            }
            Err(_) => CreateCustomMetadataError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for CreateCustomMetadataError {
    fn from(err: serde_json::error::Error) -> CreateCustomMetadataError {
        CreateCustomMetadataError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for CreateCustomMetadataError {
    fn from(err: CredentialsError) -> CreateCustomMetadataError {
        CreateCustomMetadataError::Credentials(err)
    }
}
impl From<HttpDispatchError> for CreateCustomMetadataError {
    fn from(err: HttpDispatchError) -> CreateCustomMetadataError {
        CreateCustomMetadataError::HttpDispatch(err)
    }
}
impl From<io::Error> for CreateCustomMetadataError {
    fn from(err: io::Error) -> CreateCustomMetadataError {
        CreateCustomMetadataError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for CreateCustomMetadataError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateCustomMetadataError {
    fn description(&self) -> &str {
        match *self {
            CreateCustomMetadataError::CustomMetadataLimitExceeded(ref cause) => cause,
            CreateCustomMetadataError::EntityNotExists(ref cause) => cause,
            CreateCustomMetadataError::FailedDependency(ref cause) => cause,
            CreateCustomMetadataError::ProhibitedState(ref cause) => cause,
            CreateCustomMetadataError::ServiceUnavailable(ref cause) => cause,
            CreateCustomMetadataError::UnauthorizedOperation(ref cause) => cause,
            CreateCustomMetadataError::UnauthorizedResourceAccess(ref cause) => cause,
            CreateCustomMetadataError::Validation(ref cause) => cause,
            CreateCustomMetadataError::Credentials(ref err) => err.description(),
            CreateCustomMetadataError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            CreateCustomMetadataError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by CreateFolder
#[derive(Debug, PartialEq)]
pub enum CreateFolderError {
    /// <p>The resource already exists.</p>
    EntityAlreadyExists(String),
    /// <p>The resource does not exist.</p>
    EntityNotExists(String),
    /// <p>The AWS Directory Service cannot reach an on-premises instance. Or a dependency under the control of the organization is failing, such as a connected Active Directory.</p>
    FailedDependency(String),
    /// <p>The maximum of 100,000 folders under the parent folder has been exceeded.</p>
    LimitExceeded(String),
    /// <p>The specified document version is not in the INITIALIZED state.</p>
    ProhibitedState(String),
    /// <p>One or more of the dependencies is unavailable.</p>
    ServiceUnavailable(String),
    /// <p>The operation is not permitted.</p>
    UnauthorizedOperation(String),
    /// <p>The caller does not have access to perform the action on the resource.</p>
    UnauthorizedResourceAccess(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl CreateFolderError {
    pub fn from_body(body: &str) -> CreateFolderError {
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
                    "EntityAlreadyExistsException" => {
                        CreateFolderError::EntityAlreadyExists(String::from(error_message))
                    }
                    "EntityNotExistsException" => {
                        CreateFolderError::EntityNotExists(String::from(error_message))
                    }
                    "FailedDependencyException" => {
                        CreateFolderError::FailedDependency(String::from(error_message))
                    }
                    "LimitExceededException" => {
                        CreateFolderError::LimitExceeded(String::from(error_message))
                    }
                    "ProhibitedStateException" => {
                        CreateFolderError::ProhibitedState(String::from(error_message))
                    }
                    "ServiceUnavailableException" => {
                        CreateFolderError::ServiceUnavailable(String::from(error_message))
                    }
                    "UnauthorizedOperationException" => {
                        CreateFolderError::UnauthorizedOperation(String::from(error_message))
                    }
                    "UnauthorizedResourceAccessException" => {
                        CreateFolderError::UnauthorizedResourceAccess(String::from(error_message))
                    }
                    "ValidationException" => {
                        CreateFolderError::Validation(error_message.to_string())
                    }
                    _ => CreateFolderError::Unknown(String::from(body)),
                }
            }
            Err(_) => CreateFolderError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for CreateFolderError {
    fn from(err: serde_json::error::Error) -> CreateFolderError {
        CreateFolderError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for CreateFolderError {
    fn from(err: CredentialsError) -> CreateFolderError {
        CreateFolderError::Credentials(err)
    }
}
impl From<HttpDispatchError> for CreateFolderError {
    fn from(err: HttpDispatchError) -> CreateFolderError {
        CreateFolderError::HttpDispatch(err)
    }
}
impl From<io::Error> for CreateFolderError {
    fn from(err: io::Error) -> CreateFolderError {
        CreateFolderError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for CreateFolderError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateFolderError {
    fn description(&self) -> &str {
        match *self {
            CreateFolderError::EntityAlreadyExists(ref cause) => cause,
            CreateFolderError::EntityNotExists(ref cause) => cause,
            CreateFolderError::FailedDependency(ref cause) => cause,
            CreateFolderError::LimitExceeded(ref cause) => cause,
            CreateFolderError::ProhibitedState(ref cause) => cause,
            CreateFolderError::ServiceUnavailable(ref cause) => cause,
            CreateFolderError::UnauthorizedOperation(ref cause) => cause,
            CreateFolderError::UnauthorizedResourceAccess(ref cause) => cause,
            CreateFolderError::Validation(ref cause) => cause,
            CreateFolderError::Credentials(ref err) => err.description(),
            CreateFolderError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            CreateFolderError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by CreateLabels
#[derive(Debug, PartialEq)]
pub enum CreateLabelsError {
    /// <p>The resource does not exist.</p>
    EntityNotExists(String),
    /// <p>The AWS Directory Service cannot reach an on-premises instance. Or a dependency under the control of the organization is failing, such as a connected Active Directory.</p>
    FailedDependency(String),
    /// <p>One or more of the dependencies is unavailable.</p>
    ServiceUnavailable(String),
    /// <p>The limit has been reached on the number of labels for the specified resource.</p>
    TooManyLabels(String),
    /// <p>The operation is not permitted.</p>
    UnauthorizedOperation(String),
    /// <p>The caller does not have access to perform the action on the resource.</p>
    UnauthorizedResourceAccess(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl CreateLabelsError {
    pub fn from_body(body: &str) -> CreateLabelsError {
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
                    "EntityNotExistsException" => {
                        CreateLabelsError::EntityNotExists(String::from(error_message))
                    }
                    "FailedDependencyException" => {
                        CreateLabelsError::FailedDependency(String::from(error_message))
                    }
                    "ServiceUnavailableException" => {
                        CreateLabelsError::ServiceUnavailable(String::from(error_message))
                    }
                    "TooManyLabelsException" => {
                        CreateLabelsError::TooManyLabels(String::from(error_message))
                    }
                    "UnauthorizedOperationException" => {
                        CreateLabelsError::UnauthorizedOperation(String::from(error_message))
                    }
                    "UnauthorizedResourceAccessException" => {
                        CreateLabelsError::UnauthorizedResourceAccess(String::from(error_message))
                    }
                    "ValidationException" => {
                        CreateLabelsError::Validation(error_message.to_string())
                    }
                    _ => CreateLabelsError::Unknown(String::from(body)),
                }
            }
            Err(_) => CreateLabelsError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for CreateLabelsError {
    fn from(err: serde_json::error::Error) -> CreateLabelsError {
        CreateLabelsError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for CreateLabelsError {
    fn from(err: CredentialsError) -> CreateLabelsError {
        CreateLabelsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for CreateLabelsError {
    fn from(err: HttpDispatchError) -> CreateLabelsError {
        CreateLabelsError::HttpDispatch(err)
    }
}
impl From<io::Error> for CreateLabelsError {
    fn from(err: io::Error) -> CreateLabelsError {
        CreateLabelsError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for CreateLabelsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateLabelsError {
    fn description(&self) -> &str {
        match *self {
            CreateLabelsError::EntityNotExists(ref cause) => cause,
            CreateLabelsError::FailedDependency(ref cause) => cause,
            CreateLabelsError::ServiceUnavailable(ref cause) => cause,
            CreateLabelsError::TooManyLabels(ref cause) => cause,
            CreateLabelsError::UnauthorizedOperation(ref cause) => cause,
            CreateLabelsError::UnauthorizedResourceAccess(ref cause) => cause,
            CreateLabelsError::Validation(ref cause) => cause,
            CreateLabelsError::Credentials(ref err) => err.description(),
            CreateLabelsError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            CreateLabelsError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by CreateNotificationSubscription
#[derive(Debug, PartialEq)]
pub enum CreateNotificationSubscriptionError {
    /// <p>One or more of the dependencies is unavailable.</p>
    ServiceUnavailable(String),
    /// <p>You've reached the limit on the number of subscriptions for the WorkDocs instance.</p>
    TooManySubscriptions(String),
    /// <p>The caller does not have access to perform the action on the resource.</p>
    UnauthorizedResourceAccess(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl CreateNotificationSubscriptionError {
    pub fn from_body(body: &str) -> CreateNotificationSubscriptionError {
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
                    "ServiceUnavailableException" => {
                        CreateNotificationSubscriptionError::ServiceUnavailable(String::from(
                            error_message,
                        ))
                    }
                    "TooManySubscriptionsException" => {
                        CreateNotificationSubscriptionError::TooManySubscriptions(String::from(
                            error_message,
                        ))
                    }
                    "UnauthorizedResourceAccessException" => {
                        CreateNotificationSubscriptionError::UnauthorizedResourceAccess(
                            String::from(error_message),
                        )
                    }
                    "ValidationException" => {
                        CreateNotificationSubscriptionError::Validation(error_message.to_string())
                    }
                    _ => CreateNotificationSubscriptionError::Unknown(String::from(body)),
                }
            }
            Err(_) => CreateNotificationSubscriptionError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for CreateNotificationSubscriptionError {
    fn from(err: serde_json::error::Error) -> CreateNotificationSubscriptionError {
        CreateNotificationSubscriptionError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for CreateNotificationSubscriptionError {
    fn from(err: CredentialsError) -> CreateNotificationSubscriptionError {
        CreateNotificationSubscriptionError::Credentials(err)
    }
}
impl From<HttpDispatchError> for CreateNotificationSubscriptionError {
    fn from(err: HttpDispatchError) -> CreateNotificationSubscriptionError {
        CreateNotificationSubscriptionError::HttpDispatch(err)
    }
}
impl From<io::Error> for CreateNotificationSubscriptionError {
    fn from(err: io::Error) -> CreateNotificationSubscriptionError {
        CreateNotificationSubscriptionError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for CreateNotificationSubscriptionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateNotificationSubscriptionError {
    fn description(&self) -> &str {
        match *self {
            CreateNotificationSubscriptionError::ServiceUnavailable(ref cause) => cause,
            CreateNotificationSubscriptionError::TooManySubscriptions(ref cause) => cause,
            CreateNotificationSubscriptionError::UnauthorizedResourceAccess(ref cause) => cause,
            CreateNotificationSubscriptionError::Validation(ref cause) => cause,
            CreateNotificationSubscriptionError::Credentials(ref err) => err.description(),
            CreateNotificationSubscriptionError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            CreateNotificationSubscriptionError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by CreateUser
#[derive(Debug, PartialEq)]
pub enum CreateUserError {
    /// <p>The resource already exists.</p>
    EntityAlreadyExists(String),
    /// <p>The AWS Directory Service cannot reach an on-premises instance. Or a dependency under the control of the organization is failing, such as a connected Active Directory.</p>
    FailedDependency(String),
    /// <p>One or more of the dependencies is unavailable.</p>
    ServiceUnavailable(String),
    /// <p>The operation is not permitted.</p>
    UnauthorizedOperation(String),
    /// <p>The caller does not have access to perform the action on the resource.</p>
    UnauthorizedResourceAccess(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl CreateUserError {
    pub fn from_body(body: &str) -> CreateUserError {
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
                    "EntityAlreadyExistsException" => {
                        CreateUserError::EntityAlreadyExists(String::from(error_message))
                    }
                    "FailedDependencyException" => {
                        CreateUserError::FailedDependency(String::from(error_message))
                    }
                    "ServiceUnavailableException" => {
                        CreateUserError::ServiceUnavailable(String::from(error_message))
                    }
                    "UnauthorizedOperationException" => {
                        CreateUserError::UnauthorizedOperation(String::from(error_message))
                    }
                    "UnauthorizedResourceAccessException" => {
                        CreateUserError::UnauthorizedResourceAccess(String::from(error_message))
                    }
                    "ValidationException" => CreateUserError::Validation(error_message.to_string()),
                    _ => CreateUserError::Unknown(String::from(body)),
                }
            }
            Err(_) => CreateUserError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for CreateUserError {
    fn from(err: serde_json::error::Error) -> CreateUserError {
        CreateUserError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for CreateUserError {
    fn from(err: CredentialsError) -> CreateUserError {
        CreateUserError::Credentials(err)
    }
}
impl From<HttpDispatchError> for CreateUserError {
    fn from(err: HttpDispatchError) -> CreateUserError {
        CreateUserError::HttpDispatch(err)
    }
}
impl From<io::Error> for CreateUserError {
    fn from(err: io::Error) -> CreateUserError {
        CreateUserError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for CreateUserError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateUserError {
    fn description(&self) -> &str {
        match *self {
            CreateUserError::EntityAlreadyExists(ref cause) => cause,
            CreateUserError::FailedDependency(ref cause) => cause,
            CreateUserError::ServiceUnavailable(ref cause) => cause,
            CreateUserError::UnauthorizedOperation(ref cause) => cause,
            CreateUserError::UnauthorizedResourceAccess(ref cause) => cause,
            CreateUserError::Validation(ref cause) => cause,
            CreateUserError::Credentials(ref err) => err.description(),
            CreateUserError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            CreateUserError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DeactivateUser
#[derive(Debug, PartialEq)]
pub enum DeactivateUserError {
    /// <p>The resource does not exist.</p>
    EntityNotExists(String),
    /// <p>The AWS Directory Service cannot reach an on-premises instance. Or a dependency under the control of the organization is failing, such as a connected Active Directory.</p>
    FailedDependency(String),
    /// <p>One or more of the dependencies is unavailable.</p>
    ServiceUnavailable(String),
    /// <p>The operation is not permitted.</p>
    UnauthorizedOperation(String),
    /// <p>The caller does not have access to perform the action on the resource.</p>
    UnauthorizedResourceAccess(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl DeactivateUserError {
    pub fn from_body(body: &str) -> DeactivateUserError {
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
                    "EntityNotExistsException" => {
                        DeactivateUserError::EntityNotExists(String::from(error_message))
                    }
                    "FailedDependencyException" => {
                        DeactivateUserError::FailedDependency(String::from(error_message))
                    }
                    "ServiceUnavailableException" => {
                        DeactivateUserError::ServiceUnavailable(String::from(error_message))
                    }
                    "UnauthorizedOperationException" => {
                        DeactivateUserError::UnauthorizedOperation(String::from(error_message))
                    }
                    "UnauthorizedResourceAccessException" => {
                        DeactivateUserError::UnauthorizedResourceAccess(String::from(error_message))
                    }
                    "ValidationException" => {
                        DeactivateUserError::Validation(error_message.to_string())
                    }
                    _ => DeactivateUserError::Unknown(String::from(body)),
                }
            }
            Err(_) => DeactivateUserError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DeactivateUserError {
    fn from(err: serde_json::error::Error) -> DeactivateUserError {
        DeactivateUserError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DeactivateUserError {
    fn from(err: CredentialsError) -> DeactivateUserError {
        DeactivateUserError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DeactivateUserError {
    fn from(err: HttpDispatchError) -> DeactivateUserError {
        DeactivateUserError::HttpDispatch(err)
    }
}
impl From<io::Error> for DeactivateUserError {
    fn from(err: io::Error) -> DeactivateUserError {
        DeactivateUserError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DeactivateUserError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeactivateUserError {
    fn description(&self) -> &str {
        match *self {
            DeactivateUserError::EntityNotExists(ref cause) => cause,
            DeactivateUserError::FailedDependency(ref cause) => cause,
            DeactivateUserError::ServiceUnavailable(ref cause) => cause,
            DeactivateUserError::UnauthorizedOperation(ref cause) => cause,
            DeactivateUserError::UnauthorizedResourceAccess(ref cause) => cause,
            DeactivateUserError::Validation(ref cause) => cause,
            DeactivateUserError::Credentials(ref err) => err.description(),
            DeactivateUserError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            DeactivateUserError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DeleteComment
#[derive(Debug, PartialEq)]
pub enum DeleteCommentError {
    /// <p>This exception is thrown when the document is locked for comments and user tries to create or delete a comment on that document.</p>
    DocumentLockedForComments(String),
    /// <p>The resource does not exist.</p>
    EntityNotExists(String),
    /// <p>The AWS Directory Service cannot reach an on-premises instance. Or a dependency under the control of the organization is failing, such as a connected Active Directory.</p>
    FailedDependency(String),
    /// <p>The specified document version is not in the INITIALIZED state.</p>
    ProhibitedState(String),
    /// <p>One or more of the dependencies is unavailable.</p>
    ServiceUnavailable(String),
    /// <p>The operation is not permitted.</p>
    UnauthorizedOperation(String),
    /// <p>The caller does not have access to perform the action on the resource.</p>
    UnauthorizedResourceAccess(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl DeleteCommentError {
    pub fn from_body(body: &str) -> DeleteCommentError {
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
                    "DocumentLockedForCommentsException" => {
                        DeleteCommentError::DocumentLockedForComments(String::from(error_message))
                    }
                    "EntityNotExistsException" => {
                        DeleteCommentError::EntityNotExists(String::from(error_message))
                    }
                    "FailedDependencyException" => {
                        DeleteCommentError::FailedDependency(String::from(error_message))
                    }
                    "ProhibitedStateException" => {
                        DeleteCommentError::ProhibitedState(String::from(error_message))
                    }
                    "ServiceUnavailableException" => {
                        DeleteCommentError::ServiceUnavailable(String::from(error_message))
                    }
                    "UnauthorizedOperationException" => {
                        DeleteCommentError::UnauthorizedOperation(String::from(error_message))
                    }
                    "UnauthorizedResourceAccessException" => {
                        DeleteCommentError::UnauthorizedResourceAccess(String::from(error_message))
                    }
                    "ValidationException" => {
                        DeleteCommentError::Validation(error_message.to_string())
                    }
                    _ => DeleteCommentError::Unknown(String::from(body)),
                }
            }
            Err(_) => DeleteCommentError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DeleteCommentError {
    fn from(err: serde_json::error::Error) -> DeleteCommentError {
        DeleteCommentError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DeleteCommentError {
    fn from(err: CredentialsError) -> DeleteCommentError {
        DeleteCommentError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DeleteCommentError {
    fn from(err: HttpDispatchError) -> DeleteCommentError {
        DeleteCommentError::HttpDispatch(err)
    }
}
impl From<io::Error> for DeleteCommentError {
    fn from(err: io::Error) -> DeleteCommentError {
        DeleteCommentError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DeleteCommentError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteCommentError {
    fn description(&self) -> &str {
        match *self {
            DeleteCommentError::DocumentLockedForComments(ref cause) => cause,
            DeleteCommentError::EntityNotExists(ref cause) => cause,
            DeleteCommentError::FailedDependency(ref cause) => cause,
            DeleteCommentError::ProhibitedState(ref cause) => cause,
            DeleteCommentError::ServiceUnavailable(ref cause) => cause,
            DeleteCommentError::UnauthorizedOperation(ref cause) => cause,
            DeleteCommentError::UnauthorizedResourceAccess(ref cause) => cause,
            DeleteCommentError::Validation(ref cause) => cause,
            DeleteCommentError::Credentials(ref err) => err.description(),
            DeleteCommentError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            DeleteCommentError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DeleteCustomMetadata
#[derive(Debug, PartialEq)]
pub enum DeleteCustomMetadataError {
    /// <p>The resource does not exist.</p>
    EntityNotExists(String),
    /// <p>The AWS Directory Service cannot reach an on-premises instance. Or a dependency under the control of the organization is failing, such as a connected Active Directory.</p>
    FailedDependency(String),
    /// <p>The specified document version is not in the INITIALIZED state.</p>
    ProhibitedState(String),
    /// <p>One or more of the dependencies is unavailable.</p>
    ServiceUnavailable(String),
    /// <p>The operation is not permitted.</p>
    UnauthorizedOperation(String),
    /// <p>The caller does not have access to perform the action on the resource.</p>
    UnauthorizedResourceAccess(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl DeleteCustomMetadataError {
    pub fn from_body(body: &str) -> DeleteCustomMetadataError {
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
                    "EntityNotExistsException" => {
                        DeleteCustomMetadataError::EntityNotExists(String::from(error_message))
                    }
                    "FailedDependencyException" => {
                        DeleteCustomMetadataError::FailedDependency(String::from(error_message))
                    }
                    "ProhibitedStateException" => {
                        DeleteCustomMetadataError::ProhibitedState(String::from(error_message))
                    }
                    "ServiceUnavailableException" => {
                        DeleteCustomMetadataError::ServiceUnavailable(String::from(error_message))
                    }
                    "UnauthorizedOperationException" => {
                        DeleteCustomMetadataError::UnauthorizedOperation(String::from(
                            error_message,
                        ))
                    }
                    "UnauthorizedResourceAccessException" => {
                        DeleteCustomMetadataError::UnauthorizedResourceAccess(String::from(
                            error_message,
                        ))
                    }
                    "ValidationException" => {
                        DeleteCustomMetadataError::Validation(error_message.to_string())
                    }
                    _ => DeleteCustomMetadataError::Unknown(String::from(body)),
                }
            }
            Err(_) => DeleteCustomMetadataError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DeleteCustomMetadataError {
    fn from(err: serde_json::error::Error) -> DeleteCustomMetadataError {
        DeleteCustomMetadataError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DeleteCustomMetadataError {
    fn from(err: CredentialsError) -> DeleteCustomMetadataError {
        DeleteCustomMetadataError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DeleteCustomMetadataError {
    fn from(err: HttpDispatchError) -> DeleteCustomMetadataError {
        DeleteCustomMetadataError::HttpDispatch(err)
    }
}
impl From<io::Error> for DeleteCustomMetadataError {
    fn from(err: io::Error) -> DeleteCustomMetadataError {
        DeleteCustomMetadataError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DeleteCustomMetadataError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteCustomMetadataError {
    fn description(&self) -> &str {
        match *self {
            DeleteCustomMetadataError::EntityNotExists(ref cause) => cause,
            DeleteCustomMetadataError::FailedDependency(ref cause) => cause,
            DeleteCustomMetadataError::ProhibitedState(ref cause) => cause,
            DeleteCustomMetadataError::ServiceUnavailable(ref cause) => cause,
            DeleteCustomMetadataError::UnauthorizedOperation(ref cause) => cause,
            DeleteCustomMetadataError::UnauthorizedResourceAccess(ref cause) => cause,
            DeleteCustomMetadataError::Validation(ref cause) => cause,
            DeleteCustomMetadataError::Credentials(ref err) => err.description(),
            DeleteCustomMetadataError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DeleteCustomMetadataError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DeleteDocument
#[derive(Debug, PartialEq)]
pub enum DeleteDocumentError {
    /// <p>The resource hierarchy is changing.</p>
    ConcurrentModification(String),
    /// <p>The resource does not exist.</p>
    EntityNotExists(String),
    /// <p>The AWS Directory Service cannot reach an on-premises instance. Or a dependency under the control of the organization is failing, such as a connected Active Directory.</p>
    FailedDependency(String),
    /// <p>The specified document version is not in the INITIALIZED state.</p>
    ProhibitedState(String),
    /// <p>One or more of the dependencies is unavailable.</p>
    ServiceUnavailable(String),
    /// <p>The operation is not permitted.</p>
    UnauthorizedOperation(String),
    /// <p>The caller does not have access to perform the action on the resource.</p>
    UnauthorizedResourceAccess(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl DeleteDocumentError {
    pub fn from_body(body: &str) -> DeleteDocumentError {
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
                    "ConcurrentModificationException" => {
                        DeleteDocumentError::ConcurrentModification(String::from(error_message))
                    }
                    "EntityNotExistsException" => {
                        DeleteDocumentError::EntityNotExists(String::from(error_message))
                    }
                    "FailedDependencyException" => {
                        DeleteDocumentError::FailedDependency(String::from(error_message))
                    }
                    "ProhibitedStateException" => {
                        DeleteDocumentError::ProhibitedState(String::from(error_message))
                    }
                    "ServiceUnavailableException" => {
                        DeleteDocumentError::ServiceUnavailable(String::from(error_message))
                    }
                    "UnauthorizedOperationException" => {
                        DeleteDocumentError::UnauthorizedOperation(String::from(error_message))
                    }
                    "UnauthorizedResourceAccessException" => {
                        DeleteDocumentError::UnauthorizedResourceAccess(String::from(error_message))
                    }
                    "ValidationException" => {
                        DeleteDocumentError::Validation(error_message.to_string())
                    }
                    _ => DeleteDocumentError::Unknown(String::from(body)),
                }
            }
            Err(_) => DeleteDocumentError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DeleteDocumentError {
    fn from(err: serde_json::error::Error) -> DeleteDocumentError {
        DeleteDocumentError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DeleteDocumentError {
    fn from(err: CredentialsError) -> DeleteDocumentError {
        DeleteDocumentError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DeleteDocumentError {
    fn from(err: HttpDispatchError) -> DeleteDocumentError {
        DeleteDocumentError::HttpDispatch(err)
    }
}
impl From<io::Error> for DeleteDocumentError {
    fn from(err: io::Error) -> DeleteDocumentError {
        DeleteDocumentError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DeleteDocumentError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteDocumentError {
    fn description(&self) -> &str {
        match *self {
            DeleteDocumentError::ConcurrentModification(ref cause) => cause,
            DeleteDocumentError::EntityNotExists(ref cause) => cause,
            DeleteDocumentError::FailedDependency(ref cause) => cause,
            DeleteDocumentError::ProhibitedState(ref cause) => cause,
            DeleteDocumentError::ServiceUnavailable(ref cause) => cause,
            DeleteDocumentError::UnauthorizedOperation(ref cause) => cause,
            DeleteDocumentError::UnauthorizedResourceAccess(ref cause) => cause,
            DeleteDocumentError::Validation(ref cause) => cause,
            DeleteDocumentError::Credentials(ref err) => err.description(),
            DeleteDocumentError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            DeleteDocumentError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DeleteFolder
#[derive(Debug, PartialEq)]
pub enum DeleteFolderError {
    /// <p>The resource hierarchy is changing.</p>
    ConcurrentModification(String),
    /// <p>The resource does not exist.</p>
    EntityNotExists(String),
    /// <p>The AWS Directory Service cannot reach an on-premises instance. Or a dependency under the control of the organization is failing, such as a connected Active Directory.</p>
    FailedDependency(String),
    /// <p>The specified document version is not in the INITIALIZED state.</p>
    ProhibitedState(String),
    /// <p>One or more of the dependencies is unavailable.</p>
    ServiceUnavailable(String),
    /// <p>The operation is not permitted.</p>
    UnauthorizedOperation(String),
    /// <p>The caller does not have access to perform the action on the resource.</p>
    UnauthorizedResourceAccess(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl DeleteFolderError {
    pub fn from_body(body: &str) -> DeleteFolderError {
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
                    "ConcurrentModificationException" => {
                        DeleteFolderError::ConcurrentModification(String::from(error_message))
                    }
                    "EntityNotExistsException" => {
                        DeleteFolderError::EntityNotExists(String::from(error_message))
                    }
                    "FailedDependencyException" => {
                        DeleteFolderError::FailedDependency(String::from(error_message))
                    }
                    "ProhibitedStateException" => {
                        DeleteFolderError::ProhibitedState(String::from(error_message))
                    }
                    "ServiceUnavailableException" => {
                        DeleteFolderError::ServiceUnavailable(String::from(error_message))
                    }
                    "UnauthorizedOperationException" => {
                        DeleteFolderError::UnauthorizedOperation(String::from(error_message))
                    }
                    "UnauthorizedResourceAccessException" => {
                        DeleteFolderError::UnauthorizedResourceAccess(String::from(error_message))
                    }
                    "ValidationException" => {
                        DeleteFolderError::Validation(error_message.to_string())
                    }
                    _ => DeleteFolderError::Unknown(String::from(body)),
                }
            }
            Err(_) => DeleteFolderError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DeleteFolderError {
    fn from(err: serde_json::error::Error) -> DeleteFolderError {
        DeleteFolderError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DeleteFolderError {
    fn from(err: CredentialsError) -> DeleteFolderError {
        DeleteFolderError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DeleteFolderError {
    fn from(err: HttpDispatchError) -> DeleteFolderError {
        DeleteFolderError::HttpDispatch(err)
    }
}
impl From<io::Error> for DeleteFolderError {
    fn from(err: io::Error) -> DeleteFolderError {
        DeleteFolderError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DeleteFolderError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteFolderError {
    fn description(&self) -> &str {
        match *self {
            DeleteFolderError::ConcurrentModification(ref cause) => cause,
            DeleteFolderError::EntityNotExists(ref cause) => cause,
            DeleteFolderError::FailedDependency(ref cause) => cause,
            DeleteFolderError::ProhibitedState(ref cause) => cause,
            DeleteFolderError::ServiceUnavailable(ref cause) => cause,
            DeleteFolderError::UnauthorizedOperation(ref cause) => cause,
            DeleteFolderError::UnauthorizedResourceAccess(ref cause) => cause,
            DeleteFolderError::Validation(ref cause) => cause,
            DeleteFolderError::Credentials(ref err) => err.description(),
            DeleteFolderError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            DeleteFolderError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DeleteFolderContents
#[derive(Debug, PartialEq)]
pub enum DeleteFolderContentsError {
    /// <p>The resource does not exist.</p>
    EntityNotExists(String),
    /// <p>The AWS Directory Service cannot reach an on-premises instance. Or a dependency under the control of the organization is failing, such as a connected Active Directory.</p>
    FailedDependency(String),
    /// <p>One or more of the dependencies is unavailable.</p>
    ServiceUnavailable(String),
    /// <p>The operation is not permitted.</p>
    UnauthorizedOperation(String),
    /// <p>The caller does not have access to perform the action on the resource.</p>
    UnauthorizedResourceAccess(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl DeleteFolderContentsError {
    pub fn from_body(body: &str) -> DeleteFolderContentsError {
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
                    "EntityNotExistsException" => {
                        DeleteFolderContentsError::EntityNotExists(String::from(error_message))
                    }
                    "FailedDependencyException" => {
                        DeleteFolderContentsError::FailedDependency(String::from(error_message))
                    }
                    "ServiceUnavailableException" => {
                        DeleteFolderContentsError::ServiceUnavailable(String::from(error_message))
                    }
                    "UnauthorizedOperationException" => {
                        DeleteFolderContentsError::UnauthorizedOperation(String::from(
                            error_message,
                        ))
                    }
                    "UnauthorizedResourceAccessException" => {
                        DeleteFolderContentsError::UnauthorizedResourceAccess(String::from(
                            error_message,
                        ))
                    }
                    "ValidationException" => {
                        DeleteFolderContentsError::Validation(error_message.to_string())
                    }
                    _ => DeleteFolderContentsError::Unknown(String::from(body)),
                }
            }
            Err(_) => DeleteFolderContentsError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DeleteFolderContentsError {
    fn from(err: serde_json::error::Error) -> DeleteFolderContentsError {
        DeleteFolderContentsError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DeleteFolderContentsError {
    fn from(err: CredentialsError) -> DeleteFolderContentsError {
        DeleteFolderContentsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DeleteFolderContentsError {
    fn from(err: HttpDispatchError) -> DeleteFolderContentsError {
        DeleteFolderContentsError::HttpDispatch(err)
    }
}
impl From<io::Error> for DeleteFolderContentsError {
    fn from(err: io::Error) -> DeleteFolderContentsError {
        DeleteFolderContentsError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DeleteFolderContentsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteFolderContentsError {
    fn description(&self) -> &str {
        match *self {
            DeleteFolderContentsError::EntityNotExists(ref cause) => cause,
            DeleteFolderContentsError::FailedDependency(ref cause) => cause,
            DeleteFolderContentsError::ServiceUnavailable(ref cause) => cause,
            DeleteFolderContentsError::UnauthorizedOperation(ref cause) => cause,
            DeleteFolderContentsError::UnauthorizedResourceAccess(ref cause) => cause,
            DeleteFolderContentsError::Validation(ref cause) => cause,
            DeleteFolderContentsError::Credentials(ref err) => err.description(),
            DeleteFolderContentsError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DeleteFolderContentsError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DeleteLabels
#[derive(Debug, PartialEq)]
pub enum DeleteLabelsError {
    /// <p>The resource does not exist.</p>
    EntityNotExists(String),
    /// <p>The AWS Directory Service cannot reach an on-premises instance. Or a dependency under the control of the organization is failing, such as a connected Active Directory.</p>
    FailedDependency(String),
    /// <p>One or more of the dependencies is unavailable.</p>
    ServiceUnavailable(String),
    /// <p>The operation is not permitted.</p>
    UnauthorizedOperation(String),
    /// <p>The caller does not have access to perform the action on the resource.</p>
    UnauthorizedResourceAccess(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl DeleteLabelsError {
    pub fn from_body(body: &str) -> DeleteLabelsError {
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
                    "EntityNotExistsException" => {
                        DeleteLabelsError::EntityNotExists(String::from(error_message))
                    }
                    "FailedDependencyException" => {
                        DeleteLabelsError::FailedDependency(String::from(error_message))
                    }
                    "ServiceUnavailableException" => {
                        DeleteLabelsError::ServiceUnavailable(String::from(error_message))
                    }
                    "UnauthorizedOperationException" => {
                        DeleteLabelsError::UnauthorizedOperation(String::from(error_message))
                    }
                    "UnauthorizedResourceAccessException" => {
                        DeleteLabelsError::UnauthorizedResourceAccess(String::from(error_message))
                    }
                    "ValidationException" => {
                        DeleteLabelsError::Validation(error_message.to_string())
                    }
                    _ => DeleteLabelsError::Unknown(String::from(body)),
                }
            }
            Err(_) => DeleteLabelsError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DeleteLabelsError {
    fn from(err: serde_json::error::Error) -> DeleteLabelsError {
        DeleteLabelsError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DeleteLabelsError {
    fn from(err: CredentialsError) -> DeleteLabelsError {
        DeleteLabelsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DeleteLabelsError {
    fn from(err: HttpDispatchError) -> DeleteLabelsError {
        DeleteLabelsError::HttpDispatch(err)
    }
}
impl From<io::Error> for DeleteLabelsError {
    fn from(err: io::Error) -> DeleteLabelsError {
        DeleteLabelsError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DeleteLabelsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteLabelsError {
    fn description(&self) -> &str {
        match *self {
            DeleteLabelsError::EntityNotExists(ref cause) => cause,
            DeleteLabelsError::FailedDependency(ref cause) => cause,
            DeleteLabelsError::ServiceUnavailable(ref cause) => cause,
            DeleteLabelsError::UnauthorizedOperation(ref cause) => cause,
            DeleteLabelsError::UnauthorizedResourceAccess(ref cause) => cause,
            DeleteLabelsError::Validation(ref cause) => cause,
            DeleteLabelsError::Credentials(ref err) => err.description(),
            DeleteLabelsError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            DeleteLabelsError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DeleteNotificationSubscription
#[derive(Debug, PartialEq)]
pub enum DeleteNotificationSubscriptionError {
    /// <p>The resource does not exist.</p>
    EntityNotExists(String),
    /// <p>The specified document version is not in the INITIALIZED state.</p>
    ProhibitedState(String),
    /// <p>One or more of the dependencies is unavailable.</p>
    ServiceUnavailable(String),
    /// <p>The caller does not have access to perform the action on the resource.</p>
    UnauthorizedResourceAccess(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl DeleteNotificationSubscriptionError {
    pub fn from_body(body: &str) -> DeleteNotificationSubscriptionError {
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
                    "EntityNotExistsException" => {
                        DeleteNotificationSubscriptionError::EntityNotExists(String::from(
                            error_message,
                        ))
                    }
                    "ProhibitedStateException" => {
                        DeleteNotificationSubscriptionError::ProhibitedState(String::from(
                            error_message,
                        ))
                    }
                    "ServiceUnavailableException" => {
                        DeleteNotificationSubscriptionError::ServiceUnavailable(String::from(
                            error_message,
                        ))
                    }
                    "UnauthorizedResourceAccessException" => {
                        DeleteNotificationSubscriptionError::UnauthorizedResourceAccess(
                            String::from(error_message),
                        )
                    }
                    "ValidationException" => {
                        DeleteNotificationSubscriptionError::Validation(error_message.to_string())
                    }
                    _ => DeleteNotificationSubscriptionError::Unknown(String::from(body)),
                }
            }
            Err(_) => DeleteNotificationSubscriptionError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DeleteNotificationSubscriptionError {
    fn from(err: serde_json::error::Error) -> DeleteNotificationSubscriptionError {
        DeleteNotificationSubscriptionError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DeleteNotificationSubscriptionError {
    fn from(err: CredentialsError) -> DeleteNotificationSubscriptionError {
        DeleteNotificationSubscriptionError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DeleteNotificationSubscriptionError {
    fn from(err: HttpDispatchError) -> DeleteNotificationSubscriptionError {
        DeleteNotificationSubscriptionError::HttpDispatch(err)
    }
}
impl From<io::Error> for DeleteNotificationSubscriptionError {
    fn from(err: io::Error) -> DeleteNotificationSubscriptionError {
        DeleteNotificationSubscriptionError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DeleteNotificationSubscriptionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteNotificationSubscriptionError {
    fn description(&self) -> &str {
        match *self {
            DeleteNotificationSubscriptionError::EntityNotExists(ref cause) => cause,
            DeleteNotificationSubscriptionError::ProhibitedState(ref cause) => cause,
            DeleteNotificationSubscriptionError::ServiceUnavailable(ref cause) => cause,
            DeleteNotificationSubscriptionError::UnauthorizedResourceAccess(ref cause) => cause,
            DeleteNotificationSubscriptionError::Validation(ref cause) => cause,
            DeleteNotificationSubscriptionError::Credentials(ref err) => err.description(),
            DeleteNotificationSubscriptionError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DeleteNotificationSubscriptionError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DeleteUser
#[derive(Debug, PartialEq)]
pub enum DeleteUserError {
    /// <p>The resource does not exist.</p>
    EntityNotExists(String),
    /// <p>The AWS Directory Service cannot reach an on-premises instance. Or a dependency under the control of the organization is failing, such as a connected Active Directory.</p>
    FailedDependency(String),
    /// <p>One or more of the dependencies is unavailable.</p>
    ServiceUnavailable(String),
    /// <p>The operation is not permitted.</p>
    UnauthorizedOperation(String),
    /// <p>The caller does not have access to perform the action on the resource.</p>
    UnauthorizedResourceAccess(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl DeleteUserError {
    pub fn from_body(body: &str) -> DeleteUserError {
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
                    "EntityNotExistsException" => {
                        DeleteUserError::EntityNotExists(String::from(error_message))
                    }
                    "FailedDependencyException" => {
                        DeleteUserError::FailedDependency(String::from(error_message))
                    }
                    "ServiceUnavailableException" => {
                        DeleteUserError::ServiceUnavailable(String::from(error_message))
                    }
                    "UnauthorizedOperationException" => {
                        DeleteUserError::UnauthorizedOperation(String::from(error_message))
                    }
                    "UnauthorizedResourceAccessException" => {
                        DeleteUserError::UnauthorizedResourceAccess(String::from(error_message))
                    }
                    "ValidationException" => DeleteUserError::Validation(error_message.to_string()),
                    _ => DeleteUserError::Unknown(String::from(body)),
                }
            }
            Err(_) => DeleteUserError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DeleteUserError {
    fn from(err: serde_json::error::Error) -> DeleteUserError {
        DeleteUserError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DeleteUserError {
    fn from(err: CredentialsError) -> DeleteUserError {
        DeleteUserError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DeleteUserError {
    fn from(err: HttpDispatchError) -> DeleteUserError {
        DeleteUserError::HttpDispatch(err)
    }
}
impl From<io::Error> for DeleteUserError {
    fn from(err: io::Error) -> DeleteUserError {
        DeleteUserError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DeleteUserError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteUserError {
    fn description(&self) -> &str {
        match *self {
            DeleteUserError::EntityNotExists(ref cause) => cause,
            DeleteUserError::FailedDependency(ref cause) => cause,
            DeleteUserError::ServiceUnavailable(ref cause) => cause,
            DeleteUserError::UnauthorizedOperation(ref cause) => cause,
            DeleteUserError::UnauthorizedResourceAccess(ref cause) => cause,
            DeleteUserError::Validation(ref cause) => cause,
            DeleteUserError::Credentials(ref err) => err.description(),
            DeleteUserError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            DeleteUserError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DescribeActivities
#[derive(Debug, PartialEq)]
pub enum DescribeActivitiesError {
    /// <p>The AWS Directory Service cannot reach an on-premises instance. Or a dependency under the control of the organization is failing, such as a connected Active Directory.</p>
    FailedDependency(String),
    /// <p>The pagination marker or limit fields are not valid.</p>
    InvalidArgument(String),
    /// <p>One or more of the dependencies is unavailable.</p>
    ServiceUnavailable(String),
    /// <p>The operation is not permitted.</p>
    UnauthorizedOperation(String),
    /// <p>The caller does not have access to perform the action on the resource.</p>
    UnauthorizedResourceAccess(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl DescribeActivitiesError {
    pub fn from_body(body: &str) -> DescribeActivitiesError {
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
                    "FailedDependencyException" => {
                        DescribeActivitiesError::FailedDependency(String::from(error_message))
                    }
                    "InvalidArgumentException" => {
                        DescribeActivitiesError::InvalidArgument(String::from(error_message))
                    }
                    "ServiceUnavailableException" => {
                        DescribeActivitiesError::ServiceUnavailable(String::from(error_message))
                    }
                    "UnauthorizedOperationException" => {
                        DescribeActivitiesError::UnauthorizedOperation(String::from(error_message))
                    }
                    "UnauthorizedResourceAccessException" => {
                        DescribeActivitiesError::UnauthorizedResourceAccess(String::from(
                            error_message,
                        ))
                    }
                    "ValidationException" => {
                        DescribeActivitiesError::Validation(error_message.to_string())
                    }
                    _ => DescribeActivitiesError::Unknown(String::from(body)),
                }
            }
            Err(_) => DescribeActivitiesError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DescribeActivitiesError {
    fn from(err: serde_json::error::Error) -> DescribeActivitiesError {
        DescribeActivitiesError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DescribeActivitiesError {
    fn from(err: CredentialsError) -> DescribeActivitiesError {
        DescribeActivitiesError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DescribeActivitiesError {
    fn from(err: HttpDispatchError) -> DescribeActivitiesError {
        DescribeActivitiesError::HttpDispatch(err)
    }
}
impl From<io::Error> for DescribeActivitiesError {
    fn from(err: io::Error) -> DescribeActivitiesError {
        DescribeActivitiesError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DescribeActivitiesError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeActivitiesError {
    fn description(&self) -> &str {
        match *self {
            DescribeActivitiesError::FailedDependency(ref cause) => cause,
            DescribeActivitiesError::InvalidArgument(ref cause) => cause,
            DescribeActivitiesError::ServiceUnavailable(ref cause) => cause,
            DescribeActivitiesError::UnauthorizedOperation(ref cause) => cause,
            DescribeActivitiesError::UnauthorizedResourceAccess(ref cause) => cause,
            DescribeActivitiesError::Validation(ref cause) => cause,
            DescribeActivitiesError::Credentials(ref err) => err.description(),
            DescribeActivitiesError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DescribeActivitiesError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DescribeComments
#[derive(Debug, PartialEq)]
pub enum DescribeCommentsError {
    /// <p>The resource does not exist.</p>
    EntityNotExists(String),
    /// <p>The AWS Directory Service cannot reach an on-premises instance. Or a dependency under the control of the organization is failing, such as a connected Active Directory.</p>
    FailedDependency(String),
    /// <p>The specified document version is not in the INITIALIZED state.</p>
    ProhibitedState(String),
    /// <p>One or more of the dependencies is unavailable.</p>
    ServiceUnavailable(String),
    /// <p>The operation is not permitted.</p>
    UnauthorizedOperation(String),
    /// <p>The caller does not have access to perform the action on the resource.</p>
    UnauthorizedResourceAccess(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl DescribeCommentsError {
    pub fn from_body(body: &str) -> DescribeCommentsError {
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
                    "EntityNotExistsException" => {
                        DescribeCommentsError::EntityNotExists(String::from(error_message))
                    }
                    "FailedDependencyException" => {
                        DescribeCommentsError::FailedDependency(String::from(error_message))
                    }
                    "ProhibitedStateException" => {
                        DescribeCommentsError::ProhibitedState(String::from(error_message))
                    }
                    "ServiceUnavailableException" => {
                        DescribeCommentsError::ServiceUnavailable(String::from(error_message))
                    }
                    "UnauthorizedOperationException" => {
                        DescribeCommentsError::UnauthorizedOperation(String::from(error_message))
                    }
                    "UnauthorizedResourceAccessException" => {
                        DescribeCommentsError::UnauthorizedResourceAccess(String::from(
                            error_message,
                        ))
                    }
                    "ValidationException" => {
                        DescribeCommentsError::Validation(error_message.to_string())
                    }
                    _ => DescribeCommentsError::Unknown(String::from(body)),
                }
            }
            Err(_) => DescribeCommentsError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DescribeCommentsError {
    fn from(err: serde_json::error::Error) -> DescribeCommentsError {
        DescribeCommentsError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DescribeCommentsError {
    fn from(err: CredentialsError) -> DescribeCommentsError {
        DescribeCommentsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DescribeCommentsError {
    fn from(err: HttpDispatchError) -> DescribeCommentsError {
        DescribeCommentsError::HttpDispatch(err)
    }
}
impl From<io::Error> for DescribeCommentsError {
    fn from(err: io::Error) -> DescribeCommentsError {
        DescribeCommentsError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DescribeCommentsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeCommentsError {
    fn description(&self) -> &str {
        match *self {
            DescribeCommentsError::EntityNotExists(ref cause) => cause,
            DescribeCommentsError::FailedDependency(ref cause) => cause,
            DescribeCommentsError::ProhibitedState(ref cause) => cause,
            DescribeCommentsError::ServiceUnavailable(ref cause) => cause,
            DescribeCommentsError::UnauthorizedOperation(ref cause) => cause,
            DescribeCommentsError::UnauthorizedResourceAccess(ref cause) => cause,
            DescribeCommentsError::Validation(ref cause) => cause,
            DescribeCommentsError::Credentials(ref err) => err.description(),
            DescribeCommentsError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            DescribeCommentsError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DescribeDocumentVersions
#[derive(Debug, PartialEq)]
pub enum DescribeDocumentVersionsError {
    /// <p>The resource does not exist.</p>
    EntityNotExists(String),
    /// <p>The AWS Directory Service cannot reach an on-premises instance. Or a dependency under the control of the organization is failing, such as a connected Active Directory.</p>
    FailedDependency(String),
    /// <p>The pagination marker or limit fields are not valid.</p>
    InvalidArgument(String),
    /// <p>The specified document version is not in the INITIALIZED state.</p>
    ProhibitedState(String),
    /// <p>One or more of the dependencies is unavailable.</p>
    ServiceUnavailable(String),
    /// <p>The operation is not permitted.</p>
    UnauthorizedOperation(String),
    /// <p>The caller does not have access to perform the action on the resource.</p>
    UnauthorizedResourceAccess(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl DescribeDocumentVersionsError {
    pub fn from_body(body: &str) -> DescribeDocumentVersionsError {
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
                    "EntityNotExistsException" => {
                        DescribeDocumentVersionsError::EntityNotExists(String::from(error_message))
                    }
                    "FailedDependencyException" => {
                        DescribeDocumentVersionsError::FailedDependency(String::from(error_message))
                    }
                    "InvalidArgumentException" => {
                        DescribeDocumentVersionsError::InvalidArgument(String::from(error_message))
                    }
                    "ProhibitedStateException" => {
                        DescribeDocumentVersionsError::ProhibitedState(String::from(error_message))
                    }
                    "ServiceUnavailableException" => {
                        DescribeDocumentVersionsError::ServiceUnavailable(String::from(
                            error_message,
                        ))
                    }
                    "UnauthorizedOperationException" => {
                        DescribeDocumentVersionsError::UnauthorizedOperation(String::from(
                            error_message,
                        ))
                    }
                    "UnauthorizedResourceAccessException" => {
                        DescribeDocumentVersionsError::UnauthorizedResourceAccess(String::from(
                            error_message,
                        ))
                    }
                    "ValidationException" => {
                        DescribeDocumentVersionsError::Validation(error_message.to_string())
                    }
                    _ => DescribeDocumentVersionsError::Unknown(String::from(body)),
                }
            }
            Err(_) => DescribeDocumentVersionsError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DescribeDocumentVersionsError {
    fn from(err: serde_json::error::Error) -> DescribeDocumentVersionsError {
        DescribeDocumentVersionsError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DescribeDocumentVersionsError {
    fn from(err: CredentialsError) -> DescribeDocumentVersionsError {
        DescribeDocumentVersionsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DescribeDocumentVersionsError {
    fn from(err: HttpDispatchError) -> DescribeDocumentVersionsError {
        DescribeDocumentVersionsError::HttpDispatch(err)
    }
}
impl From<io::Error> for DescribeDocumentVersionsError {
    fn from(err: io::Error) -> DescribeDocumentVersionsError {
        DescribeDocumentVersionsError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DescribeDocumentVersionsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeDocumentVersionsError {
    fn description(&self) -> &str {
        match *self {
            DescribeDocumentVersionsError::EntityNotExists(ref cause) => cause,
            DescribeDocumentVersionsError::FailedDependency(ref cause) => cause,
            DescribeDocumentVersionsError::InvalidArgument(ref cause) => cause,
            DescribeDocumentVersionsError::ProhibitedState(ref cause) => cause,
            DescribeDocumentVersionsError::ServiceUnavailable(ref cause) => cause,
            DescribeDocumentVersionsError::UnauthorizedOperation(ref cause) => cause,
            DescribeDocumentVersionsError::UnauthorizedResourceAccess(ref cause) => cause,
            DescribeDocumentVersionsError::Validation(ref cause) => cause,
            DescribeDocumentVersionsError::Credentials(ref err) => err.description(),
            DescribeDocumentVersionsError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DescribeDocumentVersionsError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DescribeFolderContents
#[derive(Debug, PartialEq)]
pub enum DescribeFolderContentsError {
    /// <p>The resource does not exist.</p>
    EntityNotExists(String),
    /// <p>The AWS Directory Service cannot reach an on-premises instance. Or a dependency under the control of the organization is failing, such as a connected Active Directory.</p>
    FailedDependency(String),
    /// <p>The pagination marker or limit fields are not valid.</p>
    InvalidArgument(String),
    /// <p>The specified document version is not in the INITIALIZED state.</p>
    ProhibitedState(String),
    /// <p>One or more of the dependencies is unavailable.</p>
    ServiceUnavailable(String),
    /// <p>The caller does not have access to perform the action on the resource.</p>
    UnauthorizedResourceAccess(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl DescribeFolderContentsError {
    pub fn from_body(body: &str) -> DescribeFolderContentsError {
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
                    "EntityNotExistsException" => {
                        DescribeFolderContentsError::EntityNotExists(String::from(error_message))
                    }
                    "FailedDependencyException" => {
                        DescribeFolderContentsError::FailedDependency(String::from(error_message))
                    }
                    "InvalidArgumentException" => {
                        DescribeFolderContentsError::InvalidArgument(String::from(error_message))
                    }
                    "ProhibitedStateException" => {
                        DescribeFolderContentsError::ProhibitedState(String::from(error_message))
                    }
                    "ServiceUnavailableException" => {
                        DescribeFolderContentsError::ServiceUnavailable(String::from(error_message))
                    }
                    "UnauthorizedResourceAccessException" => {
                        DescribeFolderContentsError::UnauthorizedResourceAccess(String::from(
                            error_message,
                        ))
                    }
                    "ValidationException" => {
                        DescribeFolderContentsError::Validation(error_message.to_string())
                    }
                    _ => DescribeFolderContentsError::Unknown(String::from(body)),
                }
            }
            Err(_) => DescribeFolderContentsError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DescribeFolderContentsError {
    fn from(err: serde_json::error::Error) -> DescribeFolderContentsError {
        DescribeFolderContentsError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DescribeFolderContentsError {
    fn from(err: CredentialsError) -> DescribeFolderContentsError {
        DescribeFolderContentsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DescribeFolderContentsError {
    fn from(err: HttpDispatchError) -> DescribeFolderContentsError {
        DescribeFolderContentsError::HttpDispatch(err)
    }
}
impl From<io::Error> for DescribeFolderContentsError {
    fn from(err: io::Error) -> DescribeFolderContentsError {
        DescribeFolderContentsError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DescribeFolderContentsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeFolderContentsError {
    fn description(&self) -> &str {
        match *self {
            DescribeFolderContentsError::EntityNotExists(ref cause) => cause,
            DescribeFolderContentsError::FailedDependency(ref cause) => cause,
            DescribeFolderContentsError::InvalidArgument(ref cause) => cause,
            DescribeFolderContentsError::ProhibitedState(ref cause) => cause,
            DescribeFolderContentsError::ServiceUnavailable(ref cause) => cause,
            DescribeFolderContentsError::UnauthorizedResourceAccess(ref cause) => cause,
            DescribeFolderContentsError::Validation(ref cause) => cause,
            DescribeFolderContentsError::Credentials(ref err) => err.description(),
            DescribeFolderContentsError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DescribeFolderContentsError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DescribeGroups
#[derive(Debug, PartialEq)]
pub enum DescribeGroupsError {
    /// <p>The AWS Directory Service cannot reach an on-premises instance. Or a dependency under the control of the organization is failing, such as a connected Active Directory.</p>
    FailedDependency(String),
    /// <p>One or more of the dependencies is unavailable.</p>
    ServiceUnavailable(String),
    /// <p>The operation is not permitted.</p>
    UnauthorizedOperation(String),
    /// <p>The caller does not have access to perform the action on the resource.</p>
    UnauthorizedResourceAccess(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl DescribeGroupsError {
    pub fn from_body(body: &str) -> DescribeGroupsError {
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
                    "FailedDependencyException" => {
                        DescribeGroupsError::FailedDependency(String::from(error_message))
                    }
                    "ServiceUnavailableException" => {
                        DescribeGroupsError::ServiceUnavailable(String::from(error_message))
                    }
                    "UnauthorizedOperationException" => {
                        DescribeGroupsError::UnauthorizedOperation(String::from(error_message))
                    }
                    "UnauthorizedResourceAccessException" => {
                        DescribeGroupsError::UnauthorizedResourceAccess(String::from(error_message))
                    }
                    "ValidationException" => {
                        DescribeGroupsError::Validation(error_message.to_string())
                    }
                    _ => DescribeGroupsError::Unknown(String::from(body)),
                }
            }
            Err(_) => DescribeGroupsError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DescribeGroupsError {
    fn from(err: serde_json::error::Error) -> DescribeGroupsError {
        DescribeGroupsError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DescribeGroupsError {
    fn from(err: CredentialsError) -> DescribeGroupsError {
        DescribeGroupsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DescribeGroupsError {
    fn from(err: HttpDispatchError) -> DescribeGroupsError {
        DescribeGroupsError::HttpDispatch(err)
    }
}
impl From<io::Error> for DescribeGroupsError {
    fn from(err: io::Error) -> DescribeGroupsError {
        DescribeGroupsError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DescribeGroupsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeGroupsError {
    fn description(&self) -> &str {
        match *self {
            DescribeGroupsError::FailedDependency(ref cause) => cause,
            DescribeGroupsError::ServiceUnavailable(ref cause) => cause,
            DescribeGroupsError::UnauthorizedOperation(ref cause) => cause,
            DescribeGroupsError::UnauthorizedResourceAccess(ref cause) => cause,
            DescribeGroupsError::Validation(ref cause) => cause,
            DescribeGroupsError::Credentials(ref err) => err.description(),
            DescribeGroupsError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            DescribeGroupsError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DescribeNotificationSubscriptions
#[derive(Debug, PartialEq)]
pub enum DescribeNotificationSubscriptionsError {
    /// <p>The resource does not exist.</p>
    EntityNotExists(String),
    /// <p>One or more of the dependencies is unavailable.</p>
    ServiceUnavailable(String),
    /// <p>The caller does not have access to perform the action on the resource.</p>
    UnauthorizedResourceAccess(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl DescribeNotificationSubscriptionsError {
    pub fn from_body(body: &str) -> DescribeNotificationSubscriptionsError {
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
                    "EntityNotExistsException" => {
                        DescribeNotificationSubscriptionsError::EntityNotExists(String::from(
                            error_message,
                        ))
                    }
                    "ServiceUnavailableException" => {
                        DescribeNotificationSubscriptionsError::ServiceUnavailable(String::from(
                            error_message,
                        ))
                    }
                    "UnauthorizedResourceAccessException" => {
                        DescribeNotificationSubscriptionsError::UnauthorizedResourceAccess(
                            String::from(error_message),
                        )
                    }
                    "ValidationException" => DescribeNotificationSubscriptionsError::Validation(
                        error_message.to_string(),
                    ),
                    _ => DescribeNotificationSubscriptionsError::Unknown(String::from(body)),
                }
            }
            Err(_) => DescribeNotificationSubscriptionsError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DescribeNotificationSubscriptionsError {
    fn from(err: serde_json::error::Error) -> DescribeNotificationSubscriptionsError {
        DescribeNotificationSubscriptionsError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DescribeNotificationSubscriptionsError {
    fn from(err: CredentialsError) -> DescribeNotificationSubscriptionsError {
        DescribeNotificationSubscriptionsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DescribeNotificationSubscriptionsError {
    fn from(err: HttpDispatchError) -> DescribeNotificationSubscriptionsError {
        DescribeNotificationSubscriptionsError::HttpDispatch(err)
    }
}
impl From<io::Error> for DescribeNotificationSubscriptionsError {
    fn from(err: io::Error) -> DescribeNotificationSubscriptionsError {
        DescribeNotificationSubscriptionsError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DescribeNotificationSubscriptionsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeNotificationSubscriptionsError {
    fn description(&self) -> &str {
        match *self {
            DescribeNotificationSubscriptionsError::EntityNotExists(ref cause) => cause,
            DescribeNotificationSubscriptionsError::ServiceUnavailable(ref cause) => cause,
            DescribeNotificationSubscriptionsError::UnauthorizedResourceAccess(ref cause) => cause,
            DescribeNotificationSubscriptionsError::Validation(ref cause) => cause,
            DescribeNotificationSubscriptionsError::Credentials(ref err) => err.description(),
            DescribeNotificationSubscriptionsError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DescribeNotificationSubscriptionsError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DescribeResourcePermissions
#[derive(Debug, PartialEq)]
pub enum DescribeResourcePermissionsError {
    /// <p>The AWS Directory Service cannot reach an on-premises instance. Or a dependency under the control of the organization is failing, such as a connected Active Directory.</p>
    FailedDependency(String),
    /// <p>One or more of the dependencies is unavailable.</p>
    ServiceUnavailable(String),
    /// <p>The operation is not permitted.</p>
    UnauthorizedOperation(String),
    /// <p>The caller does not have access to perform the action on the resource.</p>
    UnauthorizedResourceAccess(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl DescribeResourcePermissionsError {
    pub fn from_body(body: &str) -> DescribeResourcePermissionsError {
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
                    "FailedDependencyException" => {
                        DescribeResourcePermissionsError::FailedDependency(String::from(
                            error_message,
                        ))
                    }
                    "ServiceUnavailableException" => {
                        DescribeResourcePermissionsError::ServiceUnavailable(String::from(
                            error_message,
                        ))
                    }
                    "UnauthorizedOperationException" => {
                        DescribeResourcePermissionsError::UnauthorizedOperation(String::from(
                            error_message,
                        ))
                    }
                    "UnauthorizedResourceAccessException" => {
                        DescribeResourcePermissionsError::UnauthorizedResourceAccess(String::from(
                            error_message,
                        ))
                    }
                    "ValidationException" => {
                        DescribeResourcePermissionsError::Validation(error_message.to_string())
                    }
                    _ => DescribeResourcePermissionsError::Unknown(String::from(body)),
                }
            }
            Err(_) => DescribeResourcePermissionsError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DescribeResourcePermissionsError {
    fn from(err: serde_json::error::Error) -> DescribeResourcePermissionsError {
        DescribeResourcePermissionsError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DescribeResourcePermissionsError {
    fn from(err: CredentialsError) -> DescribeResourcePermissionsError {
        DescribeResourcePermissionsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DescribeResourcePermissionsError {
    fn from(err: HttpDispatchError) -> DescribeResourcePermissionsError {
        DescribeResourcePermissionsError::HttpDispatch(err)
    }
}
impl From<io::Error> for DescribeResourcePermissionsError {
    fn from(err: io::Error) -> DescribeResourcePermissionsError {
        DescribeResourcePermissionsError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DescribeResourcePermissionsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeResourcePermissionsError {
    fn description(&self) -> &str {
        match *self {
            DescribeResourcePermissionsError::FailedDependency(ref cause) => cause,
            DescribeResourcePermissionsError::ServiceUnavailable(ref cause) => cause,
            DescribeResourcePermissionsError::UnauthorizedOperation(ref cause) => cause,
            DescribeResourcePermissionsError::UnauthorizedResourceAccess(ref cause) => cause,
            DescribeResourcePermissionsError::Validation(ref cause) => cause,
            DescribeResourcePermissionsError::Credentials(ref err) => err.description(),
            DescribeResourcePermissionsError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DescribeResourcePermissionsError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DescribeRootFolders
#[derive(Debug, PartialEq)]
pub enum DescribeRootFoldersError {
    /// <p>The AWS Directory Service cannot reach an on-premises instance. Or a dependency under the control of the organization is failing, such as a connected Active Directory.</p>
    FailedDependency(String),
    /// <p>The pagination marker or limit fields are not valid.</p>
    InvalidArgument(String),
    /// <p>One or more of the dependencies is unavailable.</p>
    ServiceUnavailable(String),
    /// <p>The operation is not permitted.</p>
    UnauthorizedOperation(String),
    /// <p>The caller does not have access to perform the action on the resource.</p>
    UnauthorizedResourceAccess(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl DescribeRootFoldersError {
    pub fn from_body(body: &str) -> DescribeRootFoldersError {
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
                    "FailedDependencyException" => {
                        DescribeRootFoldersError::FailedDependency(String::from(error_message))
                    }
                    "InvalidArgumentException" => {
                        DescribeRootFoldersError::InvalidArgument(String::from(error_message))
                    }
                    "ServiceUnavailableException" => {
                        DescribeRootFoldersError::ServiceUnavailable(String::from(error_message))
                    }
                    "UnauthorizedOperationException" => {
                        DescribeRootFoldersError::UnauthorizedOperation(String::from(error_message))
                    }
                    "UnauthorizedResourceAccessException" => {
                        DescribeRootFoldersError::UnauthorizedResourceAccess(String::from(
                            error_message,
                        ))
                    }
                    "ValidationException" => {
                        DescribeRootFoldersError::Validation(error_message.to_string())
                    }
                    _ => DescribeRootFoldersError::Unknown(String::from(body)),
                }
            }
            Err(_) => DescribeRootFoldersError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DescribeRootFoldersError {
    fn from(err: serde_json::error::Error) -> DescribeRootFoldersError {
        DescribeRootFoldersError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DescribeRootFoldersError {
    fn from(err: CredentialsError) -> DescribeRootFoldersError {
        DescribeRootFoldersError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DescribeRootFoldersError {
    fn from(err: HttpDispatchError) -> DescribeRootFoldersError {
        DescribeRootFoldersError::HttpDispatch(err)
    }
}
impl From<io::Error> for DescribeRootFoldersError {
    fn from(err: io::Error) -> DescribeRootFoldersError {
        DescribeRootFoldersError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DescribeRootFoldersError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeRootFoldersError {
    fn description(&self) -> &str {
        match *self {
            DescribeRootFoldersError::FailedDependency(ref cause) => cause,
            DescribeRootFoldersError::InvalidArgument(ref cause) => cause,
            DescribeRootFoldersError::ServiceUnavailable(ref cause) => cause,
            DescribeRootFoldersError::UnauthorizedOperation(ref cause) => cause,
            DescribeRootFoldersError::UnauthorizedResourceAccess(ref cause) => cause,
            DescribeRootFoldersError::Validation(ref cause) => cause,
            DescribeRootFoldersError::Credentials(ref err) => err.description(),
            DescribeRootFoldersError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DescribeRootFoldersError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DescribeUsers
#[derive(Debug, PartialEq)]
pub enum DescribeUsersError {
    /// <p>The AWS Directory Service cannot reach an on-premises instance. Or a dependency under the control of the organization is failing, such as a connected Active Directory.</p>
    FailedDependency(String),
    /// <p>The pagination marker or limit fields are not valid.</p>
    InvalidArgument(String),
    /// <p>One or more of the dependencies is unavailable.</p>
    ServiceUnavailable(String),
    /// <p>The operation is not permitted.</p>
    UnauthorizedOperation(String),
    /// <p>The caller does not have access to perform the action on the resource.</p>
    UnauthorizedResourceAccess(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl DescribeUsersError {
    pub fn from_body(body: &str) -> DescribeUsersError {
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
                    "FailedDependencyException" => {
                        DescribeUsersError::FailedDependency(String::from(error_message))
                    }
                    "InvalidArgumentException" => {
                        DescribeUsersError::InvalidArgument(String::from(error_message))
                    }
                    "ServiceUnavailableException" => {
                        DescribeUsersError::ServiceUnavailable(String::from(error_message))
                    }
                    "UnauthorizedOperationException" => {
                        DescribeUsersError::UnauthorizedOperation(String::from(error_message))
                    }
                    "UnauthorizedResourceAccessException" => {
                        DescribeUsersError::UnauthorizedResourceAccess(String::from(error_message))
                    }
                    "ValidationException" => {
                        DescribeUsersError::Validation(error_message.to_string())
                    }
                    _ => DescribeUsersError::Unknown(String::from(body)),
                }
            }
            Err(_) => DescribeUsersError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DescribeUsersError {
    fn from(err: serde_json::error::Error) -> DescribeUsersError {
        DescribeUsersError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DescribeUsersError {
    fn from(err: CredentialsError) -> DescribeUsersError {
        DescribeUsersError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DescribeUsersError {
    fn from(err: HttpDispatchError) -> DescribeUsersError {
        DescribeUsersError::HttpDispatch(err)
    }
}
impl From<io::Error> for DescribeUsersError {
    fn from(err: io::Error) -> DescribeUsersError {
        DescribeUsersError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DescribeUsersError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeUsersError {
    fn description(&self) -> &str {
        match *self {
            DescribeUsersError::FailedDependency(ref cause) => cause,
            DescribeUsersError::InvalidArgument(ref cause) => cause,
            DescribeUsersError::ServiceUnavailable(ref cause) => cause,
            DescribeUsersError::UnauthorizedOperation(ref cause) => cause,
            DescribeUsersError::UnauthorizedResourceAccess(ref cause) => cause,
            DescribeUsersError::Validation(ref cause) => cause,
            DescribeUsersError::Credentials(ref err) => err.description(),
            DescribeUsersError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            DescribeUsersError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by GetCurrentUser
#[derive(Debug, PartialEq)]
pub enum GetCurrentUserError {
    /// <p>The resource does not exist.</p>
    EntityNotExists(String),
    /// <p>The AWS Directory Service cannot reach an on-premises instance. Or a dependency under the control of the organization is failing, such as a connected Active Directory.</p>
    FailedDependency(String),
    /// <p>One or more of the dependencies is unavailable.</p>
    ServiceUnavailable(String),
    /// <p>The operation is not permitted.</p>
    UnauthorizedOperation(String),
    /// <p>The caller does not have access to perform the action on the resource.</p>
    UnauthorizedResourceAccess(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl GetCurrentUserError {
    pub fn from_body(body: &str) -> GetCurrentUserError {
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
                    "EntityNotExistsException" => {
                        GetCurrentUserError::EntityNotExists(String::from(error_message))
                    }
                    "FailedDependencyException" => {
                        GetCurrentUserError::FailedDependency(String::from(error_message))
                    }
                    "ServiceUnavailableException" => {
                        GetCurrentUserError::ServiceUnavailable(String::from(error_message))
                    }
                    "UnauthorizedOperationException" => {
                        GetCurrentUserError::UnauthorizedOperation(String::from(error_message))
                    }
                    "UnauthorizedResourceAccessException" => {
                        GetCurrentUserError::UnauthorizedResourceAccess(String::from(error_message))
                    }
                    "ValidationException" => {
                        GetCurrentUserError::Validation(error_message.to_string())
                    }
                    _ => GetCurrentUserError::Unknown(String::from(body)),
                }
            }
            Err(_) => GetCurrentUserError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for GetCurrentUserError {
    fn from(err: serde_json::error::Error) -> GetCurrentUserError {
        GetCurrentUserError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for GetCurrentUserError {
    fn from(err: CredentialsError) -> GetCurrentUserError {
        GetCurrentUserError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GetCurrentUserError {
    fn from(err: HttpDispatchError) -> GetCurrentUserError {
        GetCurrentUserError::HttpDispatch(err)
    }
}
impl From<io::Error> for GetCurrentUserError {
    fn from(err: io::Error) -> GetCurrentUserError {
        GetCurrentUserError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for GetCurrentUserError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetCurrentUserError {
    fn description(&self) -> &str {
        match *self {
            GetCurrentUserError::EntityNotExists(ref cause) => cause,
            GetCurrentUserError::FailedDependency(ref cause) => cause,
            GetCurrentUserError::ServiceUnavailable(ref cause) => cause,
            GetCurrentUserError::UnauthorizedOperation(ref cause) => cause,
            GetCurrentUserError::UnauthorizedResourceAccess(ref cause) => cause,
            GetCurrentUserError::Validation(ref cause) => cause,
            GetCurrentUserError::Credentials(ref err) => err.description(),
            GetCurrentUserError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            GetCurrentUserError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by GetDocument
#[derive(Debug, PartialEq)]
pub enum GetDocumentError {
    /// <p>The resource does not exist.</p>
    EntityNotExists(String),
    /// <p>The AWS Directory Service cannot reach an on-premises instance. Or a dependency under the control of the organization is failing, such as a connected Active Directory.</p>
    FailedDependency(String),
    /// <p>The pagination marker or limit fields are not valid.</p>
    InvalidArgument(String),
    /// <p>The password is invalid.</p>
    InvalidPassword(String),
    /// <p>One or more of the dependencies is unavailable.</p>
    ServiceUnavailable(String),
    /// <p>The operation is not permitted.</p>
    UnauthorizedOperation(String),
    /// <p>The caller does not have access to perform the action on the resource.</p>
    UnauthorizedResourceAccess(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl GetDocumentError {
    pub fn from_body(body: &str) -> GetDocumentError {
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
                    "EntityNotExistsException" => {
                        GetDocumentError::EntityNotExists(String::from(error_message))
                    }
                    "FailedDependencyException" => {
                        GetDocumentError::FailedDependency(String::from(error_message))
                    }
                    "InvalidArgumentException" => {
                        GetDocumentError::InvalidArgument(String::from(error_message))
                    }
                    "InvalidPasswordException" => {
                        GetDocumentError::InvalidPassword(String::from(error_message))
                    }
                    "ServiceUnavailableException" => {
                        GetDocumentError::ServiceUnavailable(String::from(error_message))
                    }
                    "UnauthorizedOperationException" => {
                        GetDocumentError::UnauthorizedOperation(String::from(error_message))
                    }
                    "UnauthorizedResourceAccessException" => {
                        GetDocumentError::UnauthorizedResourceAccess(String::from(error_message))
                    }
                    "ValidationException" => {
                        GetDocumentError::Validation(error_message.to_string())
                    }
                    _ => GetDocumentError::Unknown(String::from(body)),
                }
            }
            Err(_) => GetDocumentError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for GetDocumentError {
    fn from(err: serde_json::error::Error) -> GetDocumentError {
        GetDocumentError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for GetDocumentError {
    fn from(err: CredentialsError) -> GetDocumentError {
        GetDocumentError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GetDocumentError {
    fn from(err: HttpDispatchError) -> GetDocumentError {
        GetDocumentError::HttpDispatch(err)
    }
}
impl From<io::Error> for GetDocumentError {
    fn from(err: io::Error) -> GetDocumentError {
        GetDocumentError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for GetDocumentError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetDocumentError {
    fn description(&self) -> &str {
        match *self {
            GetDocumentError::EntityNotExists(ref cause) => cause,
            GetDocumentError::FailedDependency(ref cause) => cause,
            GetDocumentError::InvalidArgument(ref cause) => cause,
            GetDocumentError::InvalidPassword(ref cause) => cause,
            GetDocumentError::ServiceUnavailable(ref cause) => cause,
            GetDocumentError::UnauthorizedOperation(ref cause) => cause,
            GetDocumentError::UnauthorizedResourceAccess(ref cause) => cause,
            GetDocumentError::Validation(ref cause) => cause,
            GetDocumentError::Credentials(ref err) => err.description(),
            GetDocumentError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            GetDocumentError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by GetDocumentPath
#[derive(Debug, PartialEq)]
pub enum GetDocumentPathError {
    /// <p>The resource does not exist.</p>
    EntityNotExists(String),
    /// <p>The AWS Directory Service cannot reach an on-premises instance. Or a dependency under the control of the organization is failing, such as a connected Active Directory.</p>
    FailedDependency(String),
    /// <p>One or more of the dependencies is unavailable.</p>
    ServiceUnavailable(String),
    /// <p>The operation is not permitted.</p>
    UnauthorizedOperation(String),
    /// <p>The caller does not have access to perform the action on the resource.</p>
    UnauthorizedResourceAccess(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl GetDocumentPathError {
    pub fn from_body(body: &str) -> GetDocumentPathError {
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
                    "EntityNotExistsException" => {
                        GetDocumentPathError::EntityNotExists(String::from(error_message))
                    }
                    "FailedDependencyException" => {
                        GetDocumentPathError::FailedDependency(String::from(error_message))
                    }
                    "ServiceUnavailableException" => {
                        GetDocumentPathError::ServiceUnavailable(String::from(error_message))
                    }
                    "UnauthorizedOperationException" => {
                        GetDocumentPathError::UnauthorizedOperation(String::from(error_message))
                    }
                    "UnauthorizedResourceAccessException" => {
                        GetDocumentPathError::UnauthorizedResourceAccess(String::from(
                            error_message,
                        ))
                    }
                    "ValidationException" => {
                        GetDocumentPathError::Validation(error_message.to_string())
                    }
                    _ => GetDocumentPathError::Unknown(String::from(body)),
                }
            }
            Err(_) => GetDocumentPathError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for GetDocumentPathError {
    fn from(err: serde_json::error::Error) -> GetDocumentPathError {
        GetDocumentPathError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for GetDocumentPathError {
    fn from(err: CredentialsError) -> GetDocumentPathError {
        GetDocumentPathError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GetDocumentPathError {
    fn from(err: HttpDispatchError) -> GetDocumentPathError {
        GetDocumentPathError::HttpDispatch(err)
    }
}
impl From<io::Error> for GetDocumentPathError {
    fn from(err: io::Error) -> GetDocumentPathError {
        GetDocumentPathError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for GetDocumentPathError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetDocumentPathError {
    fn description(&self) -> &str {
        match *self {
            GetDocumentPathError::EntityNotExists(ref cause) => cause,
            GetDocumentPathError::FailedDependency(ref cause) => cause,
            GetDocumentPathError::ServiceUnavailable(ref cause) => cause,
            GetDocumentPathError::UnauthorizedOperation(ref cause) => cause,
            GetDocumentPathError::UnauthorizedResourceAccess(ref cause) => cause,
            GetDocumentPathError::Validation(ref cause) => cause,
            GetDocumentPathError::Credentials(ref err) => err.description(),
            GetDocumentPathError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            GetDocumentPathError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by GetDocumentVersion
#[derive(Debug, PartialEq)]
pub enum GetDocumentVersionError {
    /// <p>The resource does not exist.</p>
    EntityNotExists(String),
    /// <p>The AWS Directory Service cannot reach an on-premises instance. Or a dependency under the control of the organization is failing, such as a connected Active Directory.</p>
    FailedDependency(String),
    /// <p>The password is invalid.</p>
    InvalidPassword(String),
    /// <p>The specified document version is not in the INITIALIZED state.</p>
    ProhibitedState(String),
    /// <p>One or more of the dependencies is unavailable.</p>
    ServiceUnavailable(String),
    /// <p>The operation is not permitted.</p>
    UnauthorizedOperation(String),
    /// <p>The caller does not have access to perform the action on the resource.</p>
    UnauthorizedResourceAccess(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl GetDocumentVersionError {
    pub fn from_body(body: &str) -> GetDocumentVersionError {
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
                    "EntityNotExistsException" => {
                        GetDocumentVersionError::EntityNotExists(String::from(error_message))
                    }
                    "FailedDependencyException" => {
                        GetDocumentVersionError::FailedDependency(String::from(error_message))
                    }
                    "InvalidPasswordException" => {
                        GetDocumentVersionError::InvalidPassword(String::from(error_message))
                    }
                    "ProhibitedStateException" => {
                        GetDocumentVersionError::ProhibitedState(String::from(error_message))
                    }
                    "ServiceUnavailableException" => {
                        GetDocumentVersionError::ServiceUnavailable(String::from(error_message))
                    }
                    "UnauthorizedOperationException" => {
                        GetDocumentVersionError::UnauthorizedOperation(String::from(error_message))
                    }
                    "UnauthorizedResourceAccessException" => {
                        GetDocumentVersionError::UnauthorizedResourceAccess(String::from(
                            error_message,
                        ))
                    }
                    "ValidationException" => {
                        GetDocumentVersionError::Validation(error_message.to_string())
                    }
                    _ => GetDocumentVersionError::Unknown(String::from(body)),
                }
            }
            Err(_) => GetDocumentVersionError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for GetDocumentVersionError {
    fn from(err: serde_json::error::Error) -> GetDocumentVersionError {
        GetDocumentVersionError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for GetDocumentVersionError {
    fn from(err: CredentialsError) -> GetDocumentVersionError {
        GetDocumentVersionError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GetDocumentVersionError {
    fn from(err: HttpDispatchError) -> GetDocumentVersionError {
        GetDocumentVersionError::HttpDispatch(err)
    }
}
impl From<io::Error> for GetDocumentVersionError {
    fn from(err: io::Error) -> GetDocumentVersionError {
        GetDocumentVersionError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for GetDocumentVersionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetDocumentVersionError {
    fn description(&self) -> &str {
        match *self {
            GetDocumentVersionError::EntityNotExists(ref cause) => cause,
            GetDocumentVersionError::FailedDependency(ref cause) => cause,
            GetDocumentVersionError::InvalidPassword(ref cause) => cause,
            GetDocumentVersionError::ProhibitedState(ref cause) => cause,
            GetDocumentVersionError::ServiceUnavailable(ref cause) => cause,
            GetDocumentVersionError::UnauthorizedOperation(ref cause) => cause,
            GetDocumentVersionError::UnauthorizedResourceAccess(ref cause) => cause,
            GetDocumentVersionError::Validation(ref cause) => cause,
            GetDocumentVersionError::Credentials(ref err) => err.description(),
            GetDocumentVersionError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            GetDocumentVersionError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by GetFolder
#[derive(Debug, PartialEq)]
pub enum GetFolderError {
    /// <p>The resource does not exist.</p>
    EntityNotExists(String),
    /// <p>The AWS Directory Service cannot reach an on-premises instance. Or a dependency under the control of the organization is failing, such as a connected Active Directory.</p>
    FailedDependency(String),
    /// <p>The pagination marker or limit fields are not valid.</p>
    InvalidArgument(String),
    /// <p>The specified document version is not in the INITIALIZED state.</p>
    ProhibitedState(String),
    /// <p>One or more of the dependencies is unavailable.</p>
    ServiceUnavailable(String),
    /// <p>The operation is not permitted.</p>
    UnauthorizedOperation(String),
    /// <p>The caller does not have access to perform the action on the resource.</p>
    UnauthorizedResourceAccess(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl GetFolderError {
    pub fn from_body(body: &str) -> GetFolderError {
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
                    "EntityNotExistsException" => {
                        GetFolderError::EntityNotExists(String::from(error_message))
                    }
                    "FailedDependencyException" => {
                        GetFolderError::FailedDependency(String::from(error_message))
                    }
                    "InvalidArgumentException" => {
                        GetFolderError::InvalidArgument(String::from(error_message))
                    }
                    "ProhibitedStateException" => {
                        GetFolderError::ProhibitedState(String::from(error_message))
                    }
                    "ServiceUnavailableException" => {
                        GetFolderError::ServiceUnavailable(String::from(error_message))
                    }
                    "UnauthorizedOperationException" => {
                        GetFolderError::UnauthorizedOperation(String::from(error_message))
                    }
                    "UnauthorizedResourceAccessException" => {
                        GetFolderError::UnauthorizedResourceAccess(String::from(error_message))
                    }
                    "ValidationException" => GetFolderError::Validation(error_message.to_string()),
                    _ => GetFolderError::Unknown(String::from(body)),
                }
            }
            Err(_) => GetFolderError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for GetFolderError {
    fn from(err: serde_json::error::Error) -> GetFolderError {
        GetFolderError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for GetFolderError {
    fn from(err: CredentialsError) -> GetFolderError {
        GetFolderError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GetFolderError {
    fn from(err: HttpDispatchError) -> GetFolderError {
        GetFolderError::HttpDispatch(err)
    }
}
impl From<io::Error> for GetFolderError {
    fn from(err: io::Error) -> GetFolderError {
        GetFolderError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for GetFolderError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetFolderError {
    fn description(&self) -> &str {
        match *self {
            GetFolderError::EntityNotExists(ref cause) => cause,
            GetFolderError::FailedDependency(ref cause) => cause,
            GetFolderError::InvalidArgument(ref cause) => cause,
            GetFolderError::ProhibitedState(ref cause) => cause,
            GetFolderError::ServiceUnavailable(ref cause) => cause,
            GetFolderError::UnauthorizedOperation(ref cause) => cause,
            GetFolderError::UnauthorizedResourceAccess(ref cause) => cause,
            GetFolderError::Validation(ref cause) => cause,
            GetFolderError::Credentials(ref err) => err.description(),
            GetFolderError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            GetFolderError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by GetFolderPath
#[derive(Debug, PartialEq)]
pub enum GetFolderPathError {
    /// <p>The resource does not exist.</p>
    EntityNotExists(String),
    /// <p>The AWS Directory Service cannot reach an on-premises instance. Or a dependency under the control of the organization is failing, such as a connected Active Directory.</p>
    FailedDependency(String),
    /// <p>One or more of the dependencies is unavailable.</p>
    ServiceUnavailable(String),
    /// <p>The operation is not permitted.</p>
    UnauthorizedOperation(String),
    /// <p>The caller does not have access to perform the action on the resource.</p>
    UnauthorizedResourceAccess(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl GetFolderPathError {
    pub fn from_body(body: &str) -> GetFolderPathError {
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
                    "EntityNotExistsException" => {
                        GetFolderPathError::EntityNotExists(String::from(error_message))
                    }
                    "FailedDependencyException" => {
                        GetFolderPathError::FailedDependency(String::from(error_message))
                    }
                    "ServiceUnavailableException" => {
                        GetFolderPathError::ServiceUnavailable(String::from(error_message))
                    }
                    "UnauthorizedOperationException" => {
                        GetFolderPathError::UnauthorizedOperation(String::from(error_message))
                    }
                    "UnauthorizedResourceAccessException" => {
                        GetFolderPathError::UnauthorizedResourceAccess(String::from(error_message))
                    }
                    "ValidationException" => {
                        GetFolderPathError::Validation(error_message.to_string())
                    }
                    _ => GetFolderPathError::Unknown(String::from(body)),
                }
            }
            Err(_) => GetFolderPathError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for GetFolderPathError {
    fn from(err: serde_json::error::Error) -> GetFolderPathError {
        GetFolderPathError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for GetFolderPathError {
    fn from(err: CredentialsError) -> GetFolderPathError {
        GetFolderPathError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GetFolderPathError {
    fn from(err: HttpDispatchError) -> GetFolderPathError {
        GetFolderPathError::HttpDispatch(err)
    }
}
impl From<io::Error> for GetFolderPathError {
    fn from(err: io::Error) -> GetFolderPathError {
        GetFolderPathError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for GetFolderPathError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetFolderPathError {
    fn description(&self) -> &str {
        match *self {
            GetFolderPathError::EntityNotExists(ref cause) => cause,
            GetFolderPathError::FailedDependency(ref cause) => cause,
            GetFolderPathError::ServiceUnavailable(ref cause) => cause,
            GetFolderPathError::UnauthorizedOperation(ref cause) => cause,
            GetFolderPathError::UnauthorizedResourceAccess(ref cause) => cause,
            GetFolderPathError::Validation(ref cause) => cause,
            GetFolderPathError::Credentials(ref err) => err.description(),
            GetFolderPathError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            GetFolderPathError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by InitiateDocumentVersionUpload
#[derive(Debug, PartialEq)]
pub enum InitiateDocumentVersionUploadError {
    /// <p>This exception is thrown when a valid checkout ID is not presented on document version upload calls for a document that has been checked out from Web client.</p>
    DraftUploadOutOfSync(String),
    /// <p>The resource already exists.</p>
    EntityAlreadyExists(String),
    /// <p>The resource does not exist.</p>
    EntityNotExists(String),
    /// <p>The AWS Directory Service cannot reach an on-premises instance. Or a dependency under the control of the organization is failing, such as a connected Active Directory.</p>
    FailedDependency(String),
    /// <p>The specified document version is not in the INITIALIZED state.</p>
    ProhibitedState(String),
    /// <p>The resource is already checked out.</p>
    ResourceAlreadyCheckedOut(String),
    /// <p>One or more of the dependencies is unavailable.</p>
    ServiceUnavailable(String),
    /// <p>The storage limit has been exceeded.</p>
    StorageLimitExceeded(String),
    /// <p>The storage limit will be exceeded.</p>
    StorageLimitWillExceed(String),
    /// <p>The operation is not permitted.</p>
    UnauthorizedOperation(String),
    /// <p>The caller does not have access to perform the action on the resource.</p>
    UnauthorizedResourceAccess(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl InitiateDocumentVersionUploadError {
    pub fn from_body(body: &str) -> InitiateDocumentVersionUploadError {
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
                    "DraftUploadOutOfSyncException" => {
                        InitiateDocumentVersionUploadError::DraftUploadOutOfSync(String::from(
                            error_message,
                        ))
                    }
                    "EntityAlreadyExistsException" => {
                        InitiateDocumentVersionUploadError::EntityAlreadyExists(String::from(
                            error_message,
                        ))
                    }
                    "EntityNotExistsException" => {
                        InitiateDocumentVersionUploadError::EntityNotExists(String::from(
                            error_message,
                        ))
                    }
                    "FailedDependencyException" => {
                        InitiateDocumentVersionUploadError::FailedDependency(String::from(
                            error_message,
                        ))
                    }
                    "ProhibitedStateException" => {
                        InitiateDocumentVersionUploadError::ProhibitedState(String::from(
                            error_message,
                        ))
                    }
                    "ResourceAlreadyCheckedOutException" => {
                        InitiateDocumentVersionUploadError::ResourceAlreadyCheckedOut(String::from(
                            error_message,
                        ))
                    }
                    "ServiceUnavailableException" => {
                        InitiateDocumentVersionUploadError::ServiceUnavailable(String::from(
                            error_message,
                        ))
                    }
                    "StorageLimitExceededException" => {
                        InitiateDocumentVersionUploadError::StorageLimitExceeded(String::from(
                            error_message,
                        ))
                    }
                    "StorageLimitWillExceedException" => {
                        InitiateDocumentVersionUploadError::StorageLimitWillExceed(String::from(
                            error_message,
                        ))
                    }
                    "UnauthorizedOperationException" => {
                        InitiateDocumentVersionUploadError::UnauthorizedOperation(String::from(
                            error_message,
                        ))
                    }
                    "UnauthorizedResourceAccessException" => {
                        InitiateDocumentVersionUploadError::UnauthorizedResourceAccess(
                            String::from(error_message),
                        )
                    }
                    "ValidationException" => {
                        InitiateDocumentVersionUploadError::Validation(error_message.to_string())
                    }
                    _ => InitiateDocumentVersionUploadError::Unknown(String::from(body)),
                }
            }
            Err(_) => InitiateDocumentVersionUploadError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for InitiateDocumentVersionUploadError {
    fn from(err: serde_json::error::Error) -> InitiateDocumentVersionUploadError {
        InitiateDocumentVersionUploadError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for InitiateDocumentVersionUploadError {
    fn from(err: CredentialsError) -> InitiateDocumentVersionUploadError {
        InitiateDocumentVersionUploadError::Credentials(err)
    }
}
impl From<HttpDispatchError> for InitiateDocumentVersionUploadError {
    fn from(err: HttpDispatchError) -> InitiateDocumentVersionUploadError {
        InitiateDocumentVersionUploadError::HttpDispatch(err)
    }
}
impl From<io::Error> for InitiateDocumentVersionUploadError {
    fn from(err: io::Error) -> InitiateDocumentVersionUploadError {
        InitiateDocumentVersionUploadError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for InitiateDocumentVersionUploadError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for InitiateDocumentVersionUploadError {
    fn description(&self) -> &str {
        match *self {
            InitiateDocumentVersionUploadError::DraftUploadOutOfSync(ref cause) => cause,
            InitiateDocumentVersionUploadError::EntityAlreadyExists(ref cause) => cause,
            InitiateDocumentVersionUploadError::EntityNotExists(ref cause) => cause,
            InitiateDocumentVersionUploadError::FailedDependency(ref cause) => cause,
            InitiateDocumentVersionUploadError::ProhibitedState(ref cause) => cause,
            InitiateDocumentVersionUploadError::ResourceAlreadyCheckedOut(ref cause) => cause,
            InitiateDocumentVersionUploadError::ServiceUnavailable(ref cause) => cause,
            InitiateDocumentVersionUploadError::StorageLimitExceeded(ref cause) => cause,
            InitiateDocumentVersionUploadError::StorageLimitWillExceed(ref cause) => cause,
            InitiateDocumentVersionUploadError::UnauthorizedOperation(ref cause) => cause,
            InitiateDocumentVersionUploadError::UnauthorizedResourceAccess(ref cause) => cause,
            InitiateDocumentVersionUploadError::Validation(ref cause) => cause,
            InitiateDocumentVersionUploadError::Credentials(ref err) => err.description(),
            InitiateDocumentVersionUploadError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            InitiateDocumentVersionUploadError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by RemoveAllResourcePermissions
#[derive(Debug, PartialEq)]
pub enum RemoveAllResourcePermissionsError {
    /// <p>The AWS Directory Service cannot reach an on-premises instance. Or a dependency under the control of the organization is failing, such as a connected Active Directory.</p>
    FailedDependency(String),
    /// <p>One or more of the dependencies is unavailable.</p>
    ServiceUnavailable(String),
    /// <p>The operation is not permitted.</p>
    UnauthorizedOperation(String),
    /// <p>The caller does not have access to perform the action on the resource.</p>
    UnauthorizedResourceAccess(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl RemoveAllResourcePermissionsError {
    pub fn from_body(body: &str) -> RemoveAllResourcePermissionsError {
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
                    "FailedDependencyException" => {
                        RemoveAllResourcePermissionsError::FailedDependency(String::from(
                            error_message,
                        ))
                    }
                    "ServiceUnavailableException" => {
                        RemoveAllResourcePermissionsError::ServiceUnavailable(String::from(
                            error_message,
                        ))
                    }
                    "UnauthorizedOperationException" => {
                        RemoveAllResourcePermissionsError::UnauthorizedOperation(String::from(
                            error_message,
                        ))
                    }
                    "UnauthorizedResourceAccessException" => {
                        RemoveAllResourcePermissionsError::UnauthorizedResourceAccess(String::from(
                            error_message,
                        ))
                    }
                    "ValidationException" => {
                        RemoveAllResourcePermissionsError::Validation(error_message.to_string())
                    }
                    _ => RemoveAllResourcePermissionsError::Unknown(String::from(body)),
                }
            }
            Err(_) => RemoveAllResourcePermissionsError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for RemoveAllResourcePermissionsError {
    fn from(err: serde_json::error::Error) -> RemoveAllResourcePermissionsError {
        RemoveAllResourcePermissionsError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for RemoveAllResourcePermissionsError {
    fn from(err: CredentialsError) -> RemoveAllResourcePermissionsError {
        RemoveAllResourcePermissionsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for RemoveAllResourcePermissionsError {
    fn from(err: HttpDispatchError) -> RemoveAllResourcePermissionsError {
        RemoveAllResourcePermissionsError::HttpDispatch(err)
    }
}
impl From<io::Error> for RemoveAllResourcePermissionsError {
    fn from(err: io::Error) -> RemoveAllResourcePermissionsError {
        RemoveAllResourcePermissionsError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for RemoveAllResourcePermissionsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for RemoveAllResourcePermissionsError {
    fn description(&self) -> &str {
        match *self {
            RemoveAllResourcePermissionsError::FailedDependency(ref cause) => cause,
            RemoveAllResourcePermissionsError::ServiceUnavailable(ref cause) => cause,
            RemoveAllResourcePermissionsError::UnauthorizedOperation(ref cause) => cause,
            RemoveAllResourcePermissionsError::UnauthorizedResourceAccess(ref cause) => cause,
            RemoveAllResourcePermissionsError::Validation(ref cause) => cause,
            RemoveAllResourcePermissionsError::Credentials(ref err) => err.description(),
            RemoveAllResourcePermissionsError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            RemoveAllResourcePermissionsError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by RemoveResourcePermission
#[derive(Debug, PartialEq)]
pub enum RemoveResourcePermissionError {
    /// <p>The AWS Directory Service cannot reach an on-premises instance. Or a dependency under the control of the organization is failing, such as a connected Active Directory.</p>
    FailedDependency(String),
    /// <p>One or more of the dependencies is unavailable.</p>
    ServiceUnavailable(String),
    /// <p>The operation is not permitted.</p>
    UnauthorizedOperation(String),
    /// <p>The caller does not have access to perform the action on the resource.</p>
    UnauthorizedResourceAccess(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl RemoveResourcePermissionError {
    pub fn from_body(body: &str) -> RemoveResourcePermissionError {
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
                    "FailedDependencyException" => {
                        RemoveResourcePermissionError::FailedDependency(String::from(error_message))
                    }
                    "ServiceUnavailableException" => {
                        RemoveResourcePermissionError::ServiceUnavailable(String::from(
                            error_message,
                        ))
                    }
                    "UnauthorizedOperationException" => {
                        RemoveResourcePermissionError::UnauthorizedOperation(String::from(
                            error_message,
                        ))
                    }
                    "UnauthorizedResourceAccessException" => {
                        RemoveResourcePermissionError::UnauthorizedResourceAccess(String::from(
                            error_message,
                        ))
                    }
                    "ValidationException" => {
                        RemoveResourcePermissionError::Validation(error_message.to_string())
                    }
                    _ => RemoveResourcePermissionError::Unknown(String::from(body)),
                }
            }
            Err(_) => RemoveResourcePermissionError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for RemoveResourcePermissionError {
    fn from(err: serde_json::error::Error) -> RemoveResourcePermissionError {
        RemoveResourcePermissionError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for RemoveResourcePermissionError {
    fn from(err: CredentialsError) -> RemoveResourcePermissionError {
        RemoveResourcePermissionError::Credentials(err)
    }
}
impl From<HttpDispatchError> for RemoveResourcePermissionError {
    fn from(err: HttpDispatchError) -> RemoveResourcePermissionError {
        RemoveResourcePermissionError::HttpDispatch(err)
    }
}
impl From<io::Error> for RemoveResourcePermissionError {
    fn from(err: io::Error) -> RemoveResourcePermissionError {
        RemoveResourcePermissionError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for RemoveResourcePermissionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for RemoveResourcePermissionError {
    fn description(&self) -> &str {
        match *self {
            RemoveResourcePermissionError::FailedDependency(ref cause) => cause,
            RemoveResourcePermissionError::ServiceUnavailable(ref cause) => cause,
            RemoveResourcePermissionError::UnauthorizedOperation(ref cause) => cause,
            RemoveResourcePermissionError::UnauthorizedResourceAccess(ref cause) => cause,
            RemoveResourcePermissionError::Validation(ref cause) => cause,
            RemoveResourcePermissionError::Credentials(ref err) => err.description(),
            RemoveResourcePermissionError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            RemoveResourcePermissionError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by UpdateDocument
#[derive(Debug, PartialEq)]
pub enum UpdateDocumentError {
    /// <p>The resource hierarchy is changing.</p>
    ConcurrentModification(String),
    /// <p>The resource already exists.</p>
    EntityAlreadyExists(String),
    /// <p>The resource does not exist.</p>
    EntityNotExists(String),
    /// <p>The AWS Directory Service cannot reach an on-premises instance. Or a dependency under the control of the organization is failing, such as a connected Active Directory.</p>
    FailedDependency(String),
    /// <p>The maximum of 100,000 folders under the parent folder has been exceeded.</p>
    LimitExceeded(String),
    /// <p>The specified document version is not in the INITIALIZED state.</p>
    ProhibitedState(String),
    /// <p>One or more of the dependencies is unavailable.</p>
    ServiceUnavailable(String),
    /// <p>The operation is not permitted.</p>
    UnauthorizedOperation(String),
    /// <p>The caller does not have access to perform the action on the resource.</p>
    UnauthorizedResourceAccess(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl UpdateDocumentError {
    pub fn from_body(body: &str) -> UpdateDocumentError {
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
                    "ConcurrentModificationException" => {
                        UpdateDocumentError::ConcurrentModification(String::from(error_message))
                    }
                    "EntityAlreadyExistsException" => {
                        UpdateDocumentError::EntityAlreadyExists(String::from(error_message))
                    }
                    "EntityNotExistsException" => {
                        UpdateDocumentError::EntityNotExists(String::from(error_message))
                    }
                    "FailedDependencyException" => {
                        UpdateDocumentError::FailedDependency(String::from(error_message))
                    }
                    "LimitExceededException" => {
                        UpdateDocumentError::LimitExceeded(String::from(error_message))
                    }
                    "ProhibitedStateException" => {
                        UpdateDocumentError::ProhibitedState(String::from(error_message))
                    }
                    "ServiceUnavailableException" => {
                        UpdateDocumentError::ServiceUnavailable(String::from(error_message))
                    }
                    "UnauthorizedOperationException" => {
                        UpdateDocumentError::UnauthorizedOperation(String::from(error_message))
                    }
                    "UnauthorizedResourceAccessException" => {
                        UpdateDocumentError::UnauthorizedResourceAccess(String::from(error_message))
                    }
                    "ValidationException" => {
                        UpdateDocumentError::Validation(error_message.to_string())
                    }
                    _ => UpdateDocumentError::Unknown(String::from(body)),
                }
            }
            Err(_) => UpdateDocumentError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for UpdateDocumentError {
    fn from(err: serde_json::error::Error) -> UpdateDocumentError {
        UpdateDocumentError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for UpdateDocumentError {
    fn from(err: CredentialsError) -> UpdateDocumentError {
        UpdateDocumentError::Credentials(err)
    }
}
impl From<HttpDispatchError> for UpdateDocumentError {
    fn from(err: HttpDispatchError) -> UpdateDocumentError {
        UpdateDocumentError::HttpDispatch(err)
    }
}
impl From<io::Error> for UpdateDocumentError {
    fn from(err: io::Error) -> UpdateDocumentError {
        UpdateDocumentError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for UpdateDocumentError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for UpdateDocumentError {
    fn description(&self) -> &str {
        match *self {
            UpdateDocumentError::ConcurrentModification(ref cause) => cause,
            UpdateDocumentError::EntityAlreadyExists(ref cause) => cause,
            UpdateDocumentError::EntityNotExists(ref cause) => cause,
            UpdateDocumentError::FailedDependency(ref cause) => cause,
            UpdateDocumentError::LimitExceeded(ref cause) => cause,
            UpdateDocumentError::ProhibitedState(ref cause) => cause,
            UpdateDocumentError::ServiceUnavailable(ref cause) => cause,
            UpdateDocumentError::UnauthorizedOperation(ref cause) => cause,
            UpdateDocumentError::UnauthorizedResourceAccess(ref cause) => cause,
            UpdateDocumentError::Validation(ref cause) => cause,
            UpdateDocumentError::Credentials(ref err) => err.description(),
            UpdateDocumentError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            UpdateDocumentError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by UpdateDocumentVersion
#[derive(Debug, PartialEq)]
pub enum UpdateDocumentVersionError {
    /// <p>The resource hierarchy is changing.</p>
    ConcurrentModification(String),
    /// <p>The resource does not exist.</p>
    EntityNotExists(String),
    /// <p>The AWS Directory Service cannot reach an on-premises instance. Or a dependency under the control of the organization is failing, such as a connected Active Directory.</p>
    FailedDependency(String),
    /// <p>The operation is invalid.</p>
    InvalidOperation(String),
    /// <p>The specified document version is not in the INITIALIZED state.</p>
    ProhibitedState(String),
    /// <p>One or more of the dependencies is unavailable.</p>
    ServiceUnavailable(String),
    /// <p>The operation is not permitted.</p>
    UnauthorizedOperation(String),
    /// <p>The caller does not have access to perform the action on the resource.</p>
    UnauthorizedResourceAccess(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl UpdateDocumentVersionError {
    pub fn from_body(body: &str) -> UpdateDocumentVersionError {
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
                    "ConcurrentModificationException" => {
                        UpdateDocumentVersionError::ConcurrentModification(String::from(
                            error_message,
                        ))
                    }
                    "EntityNotExistsException" => {
                        UpdateDocumentVersionError::EntityNotExists(String::from(error_message))
                    }
                    "FailedDependencyException" => {
                        UpdateDocumentVersionError::FailedDependency(String::from(error_message))
                    }
                    "InvalidOperationException" => {
                        UpdateDocumentVersionError::InvalidOperation(String::from(error_message))
                    }
                    "ProhibitedStateException" => {
                        UpdateDocumentVersionError::ProhibitedState(String::from(error_message))
                    }
                    "ServiceUnavailableException" => {
                        UpdateDocumentVersionError::ServiceUnavailable(String::from(error_message))
                    }
                    "UnauthorizedOperationException" => {
                        UpdateDocumentVersionError::UnauthorizedOperation(String::from(
                            error_message,
                        ))
                    }
                    "UnauthorizedResourceAccessException" => {
                        UpdateDocumentVersionError::UnauthorizedResourceAccess(String::from(
                            error_message,
                        ))
                    }
                    "ValidationException" => {
                        UpdateDocumentVersionError::Validation(error_message.to_string())
                    }
                    _ => UpdateDocumentVersionError::Unknown(String::from(body)),
                }
            }
            Err(_) => UpdateDocumentVersionError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for UpdateDocumentVersionError {
    fn from(err: serde_json::error::Error) -> UpdateDocumentVersionError {
        UpdateDocumentVersionError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for UpdateDocumentVersionError {
    fn from(err: CredentialsError) -> UpdateDocumentVersionError {
        UpdateDocumentVersionError::Credentials(err)
    }
}
impl From<HttpDispatchError> for UpdateDocumentVersionError {
    fn from(err: HttpDispatchError) -> UpdateDocumentVersionError {
        UpdateDocumentVersionError::HttpDispatch(err)
    }
}
impl From<io::Error> for UpdateDocumentVersionError {
    fn from(err: io::Error) -> UpdateDocumentVersionError {
        UpdateDocumentVersionError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for UpdateDocumentVersionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for UpdateDocumentVersionError {
    fn description(&self) -> &str {
        match *self {
            UpdateDocumentVersionError::ConcurrentModification(ref cause) => cause,
            UpdateDocumentVersionError::EntityNotExists(ref cause) => cause,
            UpdateDocumentVersionError::FailedDependency(ref cause) => cause,
            UpdateDocumentVersionError::InvalidOperation(ref cause) => cause,
            UpdateDocumentVersionError::ProhibitedState(ref cause) => cause,
            UpdateDocumentVersionError::ServiceUnavailable(ref cause) => cause,
            UpdateDocumentVersionError::UnauthorizedOperation(ref cause) => cause,
            UpdateDocumentVersionError::UnauthorizedResourceAccess(ref cause) => cause,
            UpdateDocumentVersionError::Validation(ref cause) => cause,
            UpdateDocumentVersionError::Credentials(ref err) => err.description(),
            UpdateDocumentVersionError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            UpdateDocumentVersionError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by UpdateFolder
#[derive(Debug, PartialEq)]
pub enum UpdateFolderError {
    /// <p>The resource hierarchy is changing.</p>
    ConcurrentModification(String),
    /// <p>The resource already exists.</p>
    EntityAlreadyExists(String),
    /// <p>The resource does not exist.</p>
    EntityNotExists(String),
    /// <p>The AWS Directory Service cannot reach an on-premises instance. Or a dependency under the control of the organization is failing, such as a connected Active Directory.</p>
    FailedDependency(String),
    /// <p>The maximum of 100,000 folders under the parent folder has been exceeded.</p>
    LimitExceeded(String),
    /// <p>The specified document version is not in the INITIALIZED state.</p>
    ProhibitedState(String),
    /// <p>One or more of the dependencies is unavailable.</p>
    ServiceUnavailable(String),
    /// <p>The operation is not permitted.</p>
    UnauthorizedOperation(String),
    /// <p>The caller does not have access to perform the action on the resource.</p>
    UnauthorizedResourceAccess(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl UpdateFolderError {
    pub fn from_body(body: &str) -> UpdateFolderError {
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
                    "ConcurrentModificationException" => {
                        UpdateFolderError::ConcurrentModification(String::from(error_message))
                    }
                    "EntityAlreadyExistsException" => {
                        UpdateFolderError::EntityAlreadyExists(String::from(error_message))
                    }
                    "EntityNotExistsException" => {
                        UpdateFolderError::EntityNotExists(String::from(error_message))
                    }
                    "FailedDependencyException" => {
                        UpdateFolderError::FailedDependency(String::from(error_message))
                    }
                    "LimitExceededException" => {
                        UpdateFolderError::LimitExceeded(String::from(error_message))
                    }
                    "ProhibitedStateException" => {
                        UpdateFolderError::ProhibitedState(String::from(error_message))
                    }
                    "ServiceUnavailableException" => {
                        UpdateFolderError::ServiceUnavailable(String::from(error_message))
                    }
                    "UnauthorizedOperationException" => {
                        UpdateFolderError::UnauthorizedOperation(String::from(error_message))
                    }
                    "UnauthorizedResourceAccessException" => {
                        UpdateFolderError::UnauthorizedResourceAccess(String::from(error_message))
                    }
                    "ValidationException" => {
                        UpdateFolderError::Validation(error_message.to_string())
                    }
                    _ => UpdateFolderError::Unknown(String::from(body)),
                }
            }
            Err(_) => UpdateFolderError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for UpdateFolderError {
    fn from(err: serde_json::error::Error) -> UpdateFolderError {
        UpdateFolderError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for UpdateFolderError {
    fn from(err: CredentialsError) -> UpdateFolderError {
        UpdateFolderError::Credentials(err)
    }
}
impl From<HttpDispatchError> for UpdateFolderError {
    fn from(err: HttpDispatchError) -> UpdateFolderError {
        UpdateFolderError::HttpDispatch(err)
    }
}
impl From<io::Error> for UpdateFolderError {
    fn from(err: io::Error) -> UpdateFolderError {
        UpdateFolderError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for UpdateFolderError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for UpdateFolderError {
    fn description(&self) -> &str {
        match *self {
            UpdateFolderError::ConcurrentModification(ref cause) => cause,
            UpdateFolderError::EntityAlreadyExists(ref cause) => cause,
            UpdateFolderError::EntityNotExists(ref cause) => cause,
            UpdateFolderError::FailedDependency(ref cause) => cause,
            UpdateFolderError::LimitExceeded(ref cause) => cause,
            UpdateFolderError::ProhibitedState(ref cause) => cause,
            UpdateFolderError::ServiceUnavailable(ref cause) => cause,
            UpdateFolderError::UnauthorizedOperation(ref cause) => cause,
            UpdateFolderError::UnauthorizedResourceAccess(ref cause) => cause,
            UpdateFolderError::Validation(ref cause) => cause,
            UpdateFolderError::Credentials(ref err) => err.description(),
            UpdateFolderError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            UpdateFolderError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by UpdateUser
#[derive(Debug, PartialEq)]
pub enum UpdateUserError {
    /// <p>The last user in the organization is being deactivated.</p>
    DeactivatingLastSystemUser(String),
    /// <p>The resource does not exist.</p>
    EntityNotExists(String),
    /// <p>The AWS Directory Service cannot reach an on-premises instance. Or a dependency under the control of the organization is failing, such as a connected Active Directory.</p>
    FailedDependency(String),
    /// <p>The user is undergoing transfer of ownership.</p>
    IllegalUserState(String),
    /// <p>The pagination marker or limit fields are not valid.</p>
    InvalidArgument(String),
    /// <p>One or more of the dependencies is unavailable.</p>
    ServiceUnavailable(String),
    /// <p>The operation is not permitted.</p>
    UnauthorizedOperation(String),
    /// <p>The caller does not have access to perform the action on the resource.</p>
    UnauthorizedResourceAccess(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl UpdateUserError {
    pub fn from_body(body: &str) -> UpdateUserError {
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
                    "DeactivatingLastSystemUserException" => {
                        UpdateUserError::DeactivatingLastSystemUser(String::from(error_message))
                    }
                    "EntityNotExistsException" => {
                        UpdateUserError::EntityNotExists(String::from(error_message))
                    }
                    "FailedDependencyException" => {
                        UpdateUserError::FailedDependency(String::from(error_message))
                    }
                    "IllegalUserStateException" => {
                        UpdateUserError::IllegalUserState(String::from(error_message))
                    }
                    "InvalidArgumentException" => {
                        UpdateUserError::InvalidArgument(String::from(error_message))
                    }
                    "ServiceUnavailableException" => {
                        UpdateUserError::ServiceUnavailable(String::from(error_message))
                    }
                    "UnauthorizedOperationException" => {
                        UpdateUserError::UnauthorizedOperation(String::from(error_message))
                    }
                    "UnauthorizedResourceAccessException" => {
                        UpdateUserError::UnauthorizedResourceAccess(String::from(error_message))
                    }
                    "ValidationException" => UpdateUserError::Validation(error_message.to_string()),
                    _ => UpdateUserError::Unknown(String::from(body)),
                }
            }
            Err(_) => UpdateUserError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for UpdateUserError {
    fn from(err: serde_json::error::Error) -> UpdateUserError {
        UpdateUserError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for UpdateUserError {
    fn from(err: CredentialsError) -> UpdateUserError {
        UpdateUserError::Credentials(err)
    }
}
impl From<HttpDispatchError> for UpdateUserError {
    fn from(err: HttpDispatchError) -> UpdateUserError {
        UpdateUserError::HttpDispatch(err)
    }
}
impl From<io::Error> for UpdateUserError {
    fn from(err: io::Error) -> UpdateUserError {
        UpdateUserError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for UpdateUserError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for UpdateUserError {
    fn description(&self) -> &str {
        match *self {
            UpdateUserError::DeactivatingLastSystemUser(ref cause) => cause,
            UpdateUserError::EntityNotExists(ref cause) => cause,
            UpdateUserError::FailedDependency(ref cause) => cause,
            UpdateUserError::IllegalUserState(ref cause) => cause,
            UpdateUserError::InvalidArgument(ref cause) => cause,
            UpdateUserError::ServiceUnavailable(ref cause) => cause,
            UpdateUserError::UnauthorizedOperation(ref cause) => cause,
            UpdateUserError::UnauthorizedResourceAccess(ref cause) => cause,
            UpdateUserError::Validation(ref cause) => cause,
            UpdateUserError::Credentials(ref err) => err.description(),
            UpdateUserError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            UpdateUserError::Unknown(ref cause) => cause,
        }
    }
}
/// Trait representing the capabilities of the Amazon WorkDocs API. Amazon WorkDocs clients implement this trait.
pub trait Workdocs {
    /// <p>Aborts the upload of the specified document version that was previously initiated by <a>InitiateDocumentVersionUpload</a>. The client should make this call only when it no longer intends to upload the document version, or fails to do so.</p>
    fn abort_document_version_upload(
        &self,
        input: AbortDocumentVersionUploadRequest,
    ) -> RusotoFuture<(), AbortDocumentVersionUploadError>;

    /// <p>Activates the specified user. Only active users can access Amazon WorkDocs.</p>
    fn activate_user(
        &self,
        input: ActivateUserRequest,
    ) -> RusotoFuture<ActivateUserResponse, ActivateUserError>;

    /// <p>Creates a set of permissions for the specified folder or document. The resource permissions are overwritten if the principals already have different permissions.</p>
    fn add_resource_permissions(
        &self,
        input: AddResourcePermissionsRequest,
    ) -> RusotoFuture<AddResourcePermissionsResponse, AddResourcePermissionsError>;

    /// <p>Adds a new comment to the specified document version.</p>
    fn create_comment(
        &self,
        input: CreateCommentRequest,
    ) -> RusotoFuture<CreateCommentResponse, CreateCommentError>;

    /// <p>Adds one or more custom properties to the specified resource (a folder, document, or version).</p>
    fn create_custom_metadata(
        &self,
        input: CreateCustomMetadataRequest,
    ) -> RusotoFuture<CreateCustomMetadataResponse, CreateCustomMetadataError>;

    /// <p>Creates a folder with the specified name and parent folder.</p>
    fn create_folder(
        &self,
        input: CreateFolderRequest,
    ) -> RusotoFuture<CreateFolderResponse, CreateFolderError>;

    /// <p>Adds the specified list of labels to the given resource (a document or folder)</p>
    fn create_labels(
        &self,
        input: CreateLabelsRequest,
    ) -> RusotoFuture<CreateLabelsResponse, CreateLabelsError>;

    /// <p>Configure WorkDocs to use Amazon SNS notifications.</p> <p>The endpoint receives a confirmation message, and must confirm the subscription. For more information, see <a href="http://docs.aws.amazon.com/sns/latest/dg/SendMessageToHttp.html#SendMessageToHttp.confirm">Confirm the Subscription</a> in the <i>Amazon Simple Notification Service Developer Guide</i>.</p>
    fn create_notification_subscription(
        &self,
        input: CreateNotificationSubscriptionRequest,
    ) -> RusotoFuture<CreateNotificationSubscriptionResponse, CreateNotificationSubscriptionError>;

    /// <p>Creates a user in a Simple AD or Microsoft AD directory. The status of a newly created user is "ACTIVE". New users can access Amazon WorkDocs.</p>
    fn create_user(
        &self,
        input: CreateUserRequest,
    ) -> RusotoFuture<CreateUserResponse, CreateUserError>;

    /// <p>Deactivates the specified user, which revokes the user's access to Amazon WorkDocs.</p>
    fn deactivate_user(
        &self,
        input: DeactivateUserRequest,
    ) -> RusotoFuture<(), DeactivateUserError>;

    /// <p>Deletes the specified comment from the document version.</p>
    fn delete_comment(&self, input: DeleteCommentRequest) -> RusotoFuture<(), DeleteCommentError>;

    /// <p>Deletes custom metadata from the specified resource.</p>
    fn delete_custom_metadata(
        &self,
        input: DeleteCustomMetadataRequest,
    ) -> RusotoFuture<DeleteCustomMetadataResponse, DeleteCustomMetadataError>;

    /// <p>Permanently deletes the specified document and its associated metadata.</p>
    fn delete_document(
        &self,
        input: DeleteDocumentRequest,
    ) -> RusotoFuture<(), DeleteDocumentError>;

    /// <p>Permanently deletes the specified folder and its contents.</p>
    fn delete_folder(&self, input: DeleteFolderRequest) -> RusotoFuture<(), DeleteFolderError>;

    /// <p>Deletes the contents of the specified folder.</p>
    fn delete_folder_contents(
        &self,
        input: DeleteFolderContentsRequest,
    ) -> RusotoFuture<(), DeleteFolderContentsError>;

    /// <p>Deletes the specified list of labels from a resource.</p>
    fn delete_labels(
        &self,
        input: DeleteLabelsRequest,
    ) -> RusotoFuture<DeleteLabelsResponse, DeleteLabelsError>;

    /// <p>Deletes the specified subscription from the specified organization.</p>
    fn delete_notification_subscription(
        &self,
        input: DeleteNotificationSubscriptionRequest,
    ) -> RusotoFuture<(), DeleteNotificationSubscriptionError>;

    /// <p>Deletes the specified user from a Simple AD or Microsoft AD directory.</p>
    fn delete_user(&self, input: DeleteUserRequest) -> RusotoFuture<(), DeleteUserError>;

    /// <p>Describes the user activities in a specified time period.</p>
    fn describe_activities(
        &self,
        input: DescribeActivitiesRequest,
    ) -> RusotoFuture<DescribeActivitiesResponse, DescribeActivitiesError>;

    /// <p>List all the comments for the specified document version.</p>
    fn describe_comments(
        &self,
        input: DescribeCommentsRequest,
    ) -> RusotoFuture<DescribeCommentsResponse, DescribeCommentsError>;

    /// <p>Retrieves the document versions for the specified document.</p> <p>By default, only active versions are returned.</p>
    fn describe_document_versions(
        &self,
        input: DescribeDocumentVersionsRequest,
    ) -> RusotoFuture<DescribeDocumentVersionsResponse, DescribeDocumentVersionsError>;

    /// <p>Describes the contents of the specified folder, including its documents and subfolders.</p> <p>By default, Amazon WorkDocs returns the first 100 active document and folder metadata items. If there are more results, the response includes a marker that you can use to request the next set of results. You can also request initialized documents.</p>
    fn describe_folder_contents(
        &self,
        input: DescribeFolderContentsRequest,
    ) -> RusotoFuture<DescribeFolderContentsResponse, DescribeFolderContentsError>;

    /// <p>Describes the groups specified by query.</p>
    fn describe_groups(
        &self,
        input: DescribeGroupsRequest,
    ) -> RusotoFuture<DescribeGroupsResponse, DescribeGroupsError>;

    /// <p>Lists the specified notification subscriptions.</p>
    fn describe_notification_subscriptions(
        &self,
        input: DescribeNotificationSubscriptionsRequest,
    ) -> RusotoFuture<
        DescribeNotificationSubscriptionsResponse,
        DescribeNotificationSubscriptionsError,
    >;

    /// <p>Describes the permissions of a specified resource.</p>
    fn describe_resource_permissions(
        &self,
        input: DescribeResourcePermissionsRequest,
    ) -> RusotoFuture<DescribeResourcePermissionsResponse, DescribeResourcePermissionsError>;

    /// <p>Describes the current user's special folders; the <code>RootFolder</code> and the <code>RecycleBin</code>. <code>RootFolder</code> is the root of user's files and folders and <code>RecycleBin</code> is the root of recycled items. This is not a valid action for SigV4 (administrative API) clients.</p>
    fn describe_root_folders(
        &self,
        input: DescribeRootFoldersRequest,
    ) -> RusotoFuture<DescribeRootFoldersResponse, DescribeRootFoldersError>;

    /// <p>Describes the specified users. You can describe all users or filter the results (for example, by status or organization).</p> <p>By default, Amazon WorkDocs returns the first 24 active or pending users. If there are more results, the response includes a marker that you can use to request the next set of results.</p>
    fn describe_users(
        &self,
        input: DescribeUsersRequest,
    ) -> RusotoFuture<DescribeUsersResponse, DescribeUsersError>;

    /// <p>Retrieves details of the current user for whom the authentication token was generated. This is not a valid action for SigV4 (administrative API) clients.</p>
    fn get_current_user(
        &self,
        input: GetCurrentUserRequest,
    ) -> RusotoFuture<GetCurrentUserResponse, GetCurrentUserError>;

    /// <p>Retrieves details of a document.</p>
    fn get_document(
        &self,
        input: GetDocumentRequest,
    ) -> RusotoFuture<GetDocumentResponse, GetDocumentError>;

    /// <p>Retrieves the path information (the hierarchy from the root folder) for the requested document.</p> <p>By default, Amazon WorkDocs returns a maximum of 100 levels upwards from the requested document and only includes the IDs of the parent folders in the path. You can limit the maximum number of levels. You can also request the names of the parent folders.</p>
    fn get_document_path(
        &self,
        input: GetDocumentPathRequest,
    ) -> RusotoFuture<GetDocumentPathResponse, GetDocumentPathError>;

    /// <p>Retrieves version metadata for the specified document.</p>
    fn get_document_version(
        &self,
        input: GetDocumentVersionRequest,
    ) -> RusotoFuture<GetDocumentVersionResponse, GetDocumentVersionError>;

    /// <p>Retrieves the metadata of the specified folder.</p>
    fn get_folder(
        &self,
        input: GetFolderRequest,
    ) -> RusotoFuture<GetFolderResponse, GetFolderError>;

    /// <p>Retrieves the path information (the hierarchy from the root folder) for the specified folder.</p> <p>By default, Amazon WorkDocs returns a maximum of 100 levels upwards from the requested folder and only includes the IDs of the parent folders in the path. You can limit the maximum number of levels. You can also request the parent folder names.</p>
    fn get_folder_path(
        &self,
        input: GetFolderPathRequest,
    ) -> RusotoFuture<GetFolderPathResponse, GetFolderPathError>;

    /// <p>Creates a new document object and version object.</p> <p>The client specifies the parent folder ID and name of the document to upload. The ID is optionally specified when creating a new version of an existing document. This is the first step to upload a document. Next, upload the document to the URL returned from the call, and then call <a>UpdateDocumentVersion</a>.</p> <p>To cancel the document upload, call <a>AbortDocumentVersionUpload</a>.</p>
    fn initiate_document_version_upload(
        &self,
        input: InitiateDocumentVersionUploadRequest,
    ) -> RusotoFuture<InitiateDocumentVersionUploadResponse, InitiateDocumentVersionUploadError>;

    /// <p>Removes all the permissions from the specified resource.</p>
    fn remove_all_resource_permissions(
        &self,
        input: RemoveAllResourcePermissionsRequest,
    ) -> RusotoFuture<(), RemoveAllResourcePermissionsError>;

    /// <p>Removes the permission for the specified principal from the specified resource.</p>
    fn remove_resource_permission(
        &self,
        input: RemoveResourcePermissionRequest,
    ) -> RusotoFuture<(), RemoveResourcePermissionError>;

    /// <p>Updates the specified attributes of a document. The user must have access to both the document and its parent folder, if applicable.</p>
    fn update_document(
        &self,
        input: UpdateDocumentRequest,
    ) -> RusotoFuture<(), UpdateDocumentError>;

    /// <p>Changes the status of the document version to ACTIVE. </p> <p>Amazon WorkDocs also sets its document container to ACTIVE. This is the last step in a document upload, after the client uploads the document to an S3-presigned URL returned by <a>InitiateDocumentVersionUpload</a>. </p>
    fn update_document_version(
        &self,
        input: UpdateDocumentVersionRequest,
    ) -> RusotoFuture<(), UpdateDocumentVersionError>;

    /// <p>Updates the specified attributes of the specified folder. The user must have access to both the folder and its parent folder, if applicable.</p>
    fn update_folder(&self, input: UpdateFolderRequest) -> RusotoFuture<(), UpdateFolderError>;

    /// <p>Updates the specified attributes of the specified user, and grants or revokes administrative privileges to the Amazon WorkDocs site.</p>
    fn update_user(
        &self,
        input: UpdateUserRequest,
    ) -> RusotoFuture<UpdateUserResponse, UpdateUserError>;
}
/// A client for the Amazon WorkDocs API.
pub struct WorkdocsClient<P = CredentialsProvider, D = RequestDispatcher>
where
    P: ProvideAwsCredentials,
    D: DispatchSignedRequest,
{
    inner: ClientInner<P, D>,
    region: region::Region,
}

impl WorkdocsClient {
    /// Creates a simple client backed by an implicit event loop.
    ///
    /// The client will use the default credentials provider and tls client.
    ///
    /// See the `rusoto_core::reactor` module for more details.
    pub fn simple(region: region::Region) -> WorkdocsClient {
        WorkdocsClient::new(
            RequestDispatcher::default(),
            CredentialsProvider::default(),
            region,
        )
    }
}

impl<P, D> WorkdocsClient<P, D>
where
    P: ProvideAwsCredentials,
    D: DispatchSignedRequest,
{
    pub fn new(request_dispatcher: D, credentials_provider: P, region: region::Region) -> Self {
        WorkdocsClient {
            inner: ClientInner::new(credentials_provider, request_dispatcher),
            region: region,
        }
    }
}

impl<P, D> Workdocs for WorkdocsClient<P, D>
where
    P: ProvideAwsCredentials + 'static,
    D: DispatchSignedRequest + 'static,
{
    /// <p>Aborts the upload of the specified document version that was previously initiated by <a>InitiateDocumentVersionUpload</a>. The client should make this call only when it no longer intends to upload the document version, or fails to do so.</p>
    fn abort_document_version_upload(
        &self,
        input: AbortDocumentVersionUploadRequest,
    ) -> RusotoFuture<(), AbortDocumentVersionUploadError> {
        let request_uri = format!(
            "/api/v1/documents/{document_id}/versions/{version_id}",
            document_id = input.document_id,
            version_id = input.version_id
        );

        let mut request = SignedRequest::new("DELETE", "workdocs", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        if let Some(ref authentication_token) = input.authentication_token {
            request.add_header("Authentication", &authentication_token.to_string());
        }

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 204 {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let result = ::std::mem::drop(response);

                    result
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(AbortDocumentVersionUploadError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Activates the specified user. Only active users can access Amazon WorkDocs.</p>
    fn activate_user(
        &self,
        input: ActivateUserRequest,
    ) -> RusotoFuture<ActivateUserResponse, ActivateUserError> {
        let request_uri = format!(
            "/api/v1/users/{user_id}/activation",
            user_id = input.user_id
        );

        let mut request = SignedRequest::new("POST", "workdocs", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        if let Some(ref authentication_token) = input.authentication_token {
            request.add_header("Authentication", &authentication_token.to_string());
        }

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result = serde_json::from_slice::<ActivateUserResponse>(&body).unwrap();

                    result
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(ActivateUserError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Creates a set of permissions for the specified folder or document. The resource permissions are overwritten if the principals already have different permissions.</p>
    fn add_resource_permissions(
        &self,
        input: AddResourcePermissionsRequest,
    ) -> RusotoFuture<AddResourcePermissionsResponse, AddResourcePermissionsError> {
        let request_uri = format!(
            "/api/v1/resources/{resource_id}/permissions",
            resource_id = input.resource_id
        );

        let mut request = SignedRequest::new("POST", "workdocs", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        if let Some(ref authentication_token) = input.authentication_token {
            request.add_header("Authentication", &authentication_token.to_string());
        }

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 201 {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result =
                        serde_json::from_slice::<AddResourcePermissionsResponse>(&body).unwrap();

                    result
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(AddResourcePermissionsError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Adds a new comment to the specified document version.</p>
    fn create_comment(
        &self,
        input: CreateCommentRequest,
    ) -> RusotoFuture<CreateCommentResponse, CreateCommentError> {
        let request_uri = format!(
            "/api/v1/documents/{document_id}/versions/{version_id}/comment",
            document_id = input.document_id,
            version_id = input.version_id
        );

        let mut request = SignedRequest::new("POST", "workdocs", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        if let Some(ref authentication_token) = input.authentication_token {
            request.add_header("Authentication", &authentication_token.to_string());
        }

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 201 {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result = serde_json::from_slice::<CreateCommentResponse>(&body).unwrap();

                    result
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(CreateCommentError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Adds one or more custom properties to the specified resource (a folder, document, or version).</p>
    fn create_custom_metadata(
        &self,
        input: CreateCustomMetadataRequest,
    ) -> RusotoFuture<CreateCustomMetadataResponse, CreateCustomMetadataError> {
        let request_uri = format!(
            "/api/v1/resources/{resource_id}/customMetadata",
            resource_id = input.resource_id
        );

        let mut request = SignedRequest::new("PUT", "workdocs", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        if let Some(ref authentication_token) = input.authentication_token {
            request.add_header("Authentication", &authentication_token.to_string());
        }
        let mut params = Params::new();
        if let Some(ref x) = input.version_id {
            params.put("versionid", x);
        }
        request.set_params(params);

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result =
                        serde_json::from_slice::<CreateCustomMetadataResponse>(&body).unwrap();

                    result
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(CreateCustomMetadataError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Creates a folder with the specified name and parent folder.</p>
    fn create_folder(
        &self,
        input: CreateFolderRequest,
    ) -> RusotoFuture<CreateFolderResponse, CreateFolderError> {
        let request_uri = "/api/v1/folders";

        let mut request = SignedRequest::new("POST", "workdocs", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        if let Some(ref authentication_token) = input.authentication_token {
            request.add_header("Authentication", &authentication_token.to_string());
        }

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 201 {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result = serde_json::from_slice::<CreateFolderResponse>(&body).unwrap();

                    result
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(CreateFolderError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Adds the specified list of labels to the given resource (a document or folder)</p>
    fn create_labels(
        &self,
        input: CreateLabelsRequest,
    ) -> RusotoFuture<CreateLabelsResponse, CreateLabelsError> {
        let request_uri = format!(
            "/api/v1/resources/{resource_id}/labels",
            resource_id = input.resource_id
        );

        let mut request = SignedRequest::new("PUT", "workdocs", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        if let Some(ref authentication_token) = input.authentication_token {
            request.add_header("Authentication", &authentication_token.to_string());
        }

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result = serde_json::from_slice::<CreateLabelsResponse>(&body).unwrap();

                    result
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(CreateLabelsError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Configure WorkDocs to use Amazon SNS notifications.</p> <p>The endpoint receives a confirmation message, and must confirm the subscription. For more information, see <a href="http://docs.aws.amazon.com/sns/latest/dg/SendMessageToHttp.html#SendMessageToHttp.confirm">Confirm the Subscription</a> in the <i>Amazon Simple Notification Service Developer Guide</i>.</p>
    fn create_notification_subscription(
        &self,
        input: CreateNotificationSubscriptionRequest,
    ) -> RusotoFuture<CreateNotificationSubscriptionResponse, CreateNotificationSubscriptionError>
    {
        let request_uri = format!(
            "/api/v1/organizations/{organization_id}/subscriptions",
            organization_id = input.organization_id
        );

        let mut request = SignedRequest::new("POST", "workdocs", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result = serde_json::from_slice::<CreateNotificationSubscriptionResponse>(
                        &body,
                    ).unwrap();

                    result
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(CreateNotificationSubscriptionError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Creates a user in a Simple AD or Microsoft AD directory. The status of a newly created user is "ACTIVE". New users can access Amazon WorkDocs.</p>
    fn create_user(
        &self,
        input: CreateUserRequest,
    ) -> RusotoFuture<CreateUserResponse, CreateUserError> {
        let request_uri = "/api/v1/users";

        let mut request = SignedRequest::new("POST", "workdocs", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        if let Some(ref authentication_token) = input.authentication_token {
            request.add_header("Authentication", &authentication_token.to_string());
        }

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 201 {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result = serde_json::from_slice::<CreateUserResponse>(&body).unwrap();

                    result
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(CreateUserError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Deactivates the specified user, which revokes the user's access to Amazon WorkDocs.</p>
    fn deactivate_user(
        &self,
        input: DeactivateUserRequest,
    ) -> RusotoFuture<(), DeactivateUserError> {
        let request_uri = format!(
            "/api/v1/users/{user_id}/activation",
            user_id = input.user_id
        );

        let mut request = SignedRequest::new("DELETE", "workdocs", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        if let Some(ref authentication_token) = input.authentication_token {
            request.add_header("Authentication", &authentication_token.to_string());
        }

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 204 {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let result = ::std::mem::drop(response);

                    result
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(DeactivateUserError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Deletes the specified comment from the document version.</p>
    fn delete_comment(&self, input: DeleteCommentRequest) -> RusotoFuture<(), DeleteCommentError> {
        let request_uri = format!(
            "/api/v1/documents/{document_id}/versions/{version_id}/comment/{comment_id}",
            comment_id = input.comment_id,
            document_id = input.document_id,
            version_id = input.version_id
        );

        let mut request = SignedRequest::new("DELETE", "workdocs", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        if let Some(ref authentication_token) = input.authentication_token {
            request.add_header("Authentication", &authentication_token.to_string());
        }

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 204 {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let result = ::std::mem::drop(response);

                    result
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(DeleteCommentError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Deletes custom metadata from the specified resource.</p>
    fn delete_custom_metadata(
        &self,
        input: DeleteCustomMetadataRequest,
    ) -> RusotoFuture<DeleteCustomMetadataResponse, DeleteCustomMetadataError> {
        let request_uri = format!(
            "/api/v1/resources/{resource_id}/customMetadata",
            resource_id = input.resource_id
        );

        let mut request = SignedRequest::new("DELETE", "workdocs", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        if let Some(ref authentication_token) = input.authentication_token {
            request.add_header("Authentication", &authentication_token.to_string());
        }
        let mut params = Params::new();
        if let Some(ref x) = input.delete_all {
            params.put("deleteAll", x);
        }
        if let Some(ref x) = input.keys {
            for item in x.iter() {
                params.put("keys", item);
            }
        }
        if let Some(ref x) = input.version_id {
            params.put("versionId", x);
        }
        request.set_params(params);

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result =
                        serde_json::from_slice::<DeleteCustomMetadataResponse>(&body).unwrap();

                    result
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(DeleteCustomMetadataError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Permanently deletes the specified document and its associated metadata.</p>
    fn delete_document(
        &self,
        input: DeleteDocumentRequest,
    ) -> RusotoFuture<(), DeleteDocumentError> {
        let request_uri = format!(
            "/api/v1/documents/{document_id}",
            document_id = input.document_id
        );

        let mut request = SignedRequest::new("DELETE", "workdocs", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        if let Some(ref authentication_token) = input.authentication_token {
            request.add_header("Authentication", &authentication_token.to_string());
        }

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 204 {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let result = ::std::mem::drop(response);

                    result
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(DeleteDocumentError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Permanently deletes the specified folder and its contents.</p>
    fn delete_folder(&self, input: DeleteFolderRequest) -> RusotoFuture<(), DeleteFolderError> {
        let request_uri = format!("/api/v1/folders/{folder_id}", folder_id = input.folder_id);

        let mut request = SignedRequest::new("DELETE", "workdocs", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        if let Some(ref authentication_token) = input.authentication_token {
            request.add_header("Authentication", &authentication_token.to_string());
        }

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 204 {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let result = ::std::mem::drop(response);

                    result
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(DeleteFolderError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Deletes the contents of the specified folder.</p>
    fn delete_folder_contents(
        &self,
        input: DeleteFolderContentsRequest,
    ) -> RusotoFuture<(), DeleteFolderContentsError> {
        let request_uri = format!(
            "/api/v1/folders/{folder_id}/contents",
            folder_id = input.folder_id
        );

        let mut request = SignedRequest::new("DELETE", "workdocs", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        if let Some(ref authentication_token) = input.authentication_token {
            request.add_header("Authentication", &authentication_token.to_string());
        }

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 204 {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let result = ::std::mem::drop(response);

                    result
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(DeleteFolderContentsError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Deletes the specified list of labels from a resource.</p>
    fn delete_labels(
        &self,
        input: DeleteLabelsRequest,
    ) -> RusotoFuture<DeleteLabelsResponse, DeleteLabelsError> {
        let request_uri = format!(
            "/api/v1/resources/{resource_id}/labels",
            resource_id = input.resource_id
        );

        let mut request = SignedRequest::new("DELETE", "workdocs", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        if let Some(ref authentication_token) = input.authentication_token {
            request.add_header("Authentication", &authentication_token.to_string());
        }
        let mut params = Params::new();
        if let Some(ref x) = input.delete_all {
            params.put("deleteAll", x);
        }
        if let Some(ref x) = input.labels {
            for item in x.iter() {
                params.put("labels", item);
            }
        }
        request.set_params(params);

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result = serde_json::from_slice::<DeleteLabelsResponse>(&body).unwrap();

                    result
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(DeleteLabelsError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Deletes the specified subscription from the specified organization.</p>
    fn delete_notification_subscription(
        &self,
        input: DeleteNotificationSubscriptionRequest,
    ) -> RusotoFuture<(), DeleteNotificationSubscriptionError> {
        let request_uri = format!(
            "/api/v1/organizations/{organization_id}/subscriptions/{subscription_id}",
            organization_id = input.organization_id,
            subscription_id = input.subscription_id
        );

        let mut request = SignedRequest::new("DELETE", "workdocs", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let result = ::std::mem::drop(response);

                    result
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(DeleteNotificationSubscriptionError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Deletes the specified user from a Simple AD or Microsoft AD directory.</p>
    fn delete_user(&self, input: DeleteUserRequest) -> RusotoFuture<(), DeleteUserError> {
        let request_uri = format!("/api/v1/users/{user_id}", user_id = input.user_id);

        let mut request = SignedRequest::new("DELETE", "workdocs", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        if let Some(ref authentication_token) = input.authentication_token {
            request.add_header("Authentication", &authentication_token.to_string());
        }

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 204 {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let result = ::std::mem::drop(response);

                    result
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(DeleteUserError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Describes the user activities in a specified time period.</p>
    fn describe_activities(
        &self,
        input: DescribeActivitiesRequest,
    ) -> RusotoFuture<DescribeActivitiesResponse, DescribeActivitiesError> {
        let request_uri = "/api/v1/activities";

        let mut request = SignedRequest::new("GET", "workdocs", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        if let Some(ref authentication_token) = input.authentication_token {
            request.add_header("Authentication", &authentication_token.to_string());
        }
        let mut params = Params::new();
        if let Some(ref x) = input.end_time {
            params.put("endTime", x);
        }
        if let Some(ref x) = input.limit {
            params.put("limit", x);
        }
        if let Some(ref x) = input.marker {
            params.put("marker", x);
        }
        if let Some(ref x) = input.organization_id {
            params.put("organizationId", x);
        }
        if let Some(ref x) = input.start_time {
            params.put("startTime", x);
        }
        if let Some(ref x) = input.user_id {
            params.put("userId", x);
        }
        request.set_params(params);

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result =
                        serde_json::from_slice::<DescribeActivitiesResponse>(&body).unwrap();

                    result
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(DescribeActivitiesError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>List all the comments for the specified document version.</p>
    fn describe_comments(
        &self,
        input: DescribeCommentsRequest,
    ) -> RusotoFuture<DescribeCommentsResponse, DescribeCommentsError> {
        let request_uri = format!(
            "/api/v1/documents/{document_id}/versions/{version_id}/comments",
            document_id = input.document_id,
            version_id = input.version_id
        );

        let mut request = SignedRequest::new("GET", "workdocs", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        if let Some(ref authentication_token) = input.authentication_token {
            request.add_header("Authentication", &authentication_token.to_string());
        }
        let mut params = Params::new();
        if let Some(ref x) = input.limit {
            params.put("limit", x);
        }
        if let Some(ref x) = input.marker {
            params.put("marker", x);
        }
        request.set_params(params);

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result = serde_json::from_slice::<DescribeCommentsResponse>(&body).unwrap();

                    result
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(DescribeCommentsError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Retrieves the document versions for the specified document.</p> <p>By default, only active versions are returned.</p>
    fn describe_document_versions(
        &self,
        input: DescribeDocumentVersionsRequest,
    ) -> RusotoFuture<DescribeDocumentVersionsResponse, DescribeDocumentVersionsError> {
        let request_uri = format!(
            "/api/v1/documents/{document_id}/versions",
            document_id = input.document_id
        );

        let mut request = SignedRequest::new("GET", "workdocs", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        if let Some(ref authentication_token) = input.authentication_token {
            request.add_header("Authentication", &authentication_token.to_string());
        }
        let mut params = Params::new();
        if let Some(ref x) = input.fields {
            params.put("fields", x);
        }
        if let Some(ref x) = input.include {
            params.put("include", x);
        }
        if let Some(ref x) = input.limit {
            params.put("limit", x);
        }
        if let Some(ref x) = input.marker {
            params.put("marker", x);
        }
        request.set_params(params);

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result =
                        serde_json::from_slice::<DescribeDocumentVersionsResponse>(&body).unwrap();

                    result
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(DescribeDocumentVersionsError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Describes the contents of the specified folder, including its documents and subfolders.</p> <p>By default, Amazon WorkDocs returns the first 100 active document and folder metadata items. If there are more results, the response includes a marker that you can use to request the next set of results. You can also request initialized documents.</p>
    fn describe_folder_contents(
        &self,
        input: DescribeFolderContentsRequest,
    ) -> RusotoFuture<DescribeFolderContentsResponse, DescribeFolderContentsError> {
        let request_uri = format!(
            "/api/v1/folders/{folder_id}/contents",
            folder_id = input.folder_id
        );

        let mut request = SignedRequest::new("GET", "workdocs", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        if let Some(ref authentication_token) = input.authentication_token {
            request.add_header("Authentication", &authentication_token.to_string());
        }
        let mut params = Params::new();
        if let Some(ref x) = input.include {
            params.put("include", x);
        }
        if let Some(ref x) = input.limit {
            params.put("limit", x);
        }
        if let Some(ref x) = input.marker {
            params.put("marker", x);
        }
        if let Some(ref x) = input.order {
            params.put("order", x);
        }
        if let Some(ref x) = input.sort {
            params.put("sort", x);
        }
        if let Some(ref x) = input.type_ {
            params.put("type", x);
        }
        request.set_params(params);

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result =
                        serde_json::from_slice::<DescribeFolderContentsResponse>(&body).unwrap();

                    result
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(DescribeFolderContentsError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Describes the groups specified by query.</p>
    fn describe_groups(
        &self,
        input: DescribeGroupsRequest,
    ) -> RusotoFuture<DescribeGroupsResponse, DescribeGroupsError> {
        let request_uri = "/api/v1/groups";

        let mut request = SignedRequest::new("GET", "workdocs", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        if let Some(ref authentication_token) = input.authentication_token {
            request.add_header("Authentication", &authentication_token.to_string());
        }
        let mut params = Params::new();
        if let Some(ref x) = input.limit {
            params.put("limit", x);
        }
        if let Some(ref x) = input.marker {
            params.put("marker", x);
        }
        if let Some(ref x) = input.organization_id {
            params.put("organizationId", x);
        }
        params.put("searchQuery", &input.search_query);
        request.set_params(params);

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result = serde_json::from_slice::<DescribeGroupsResponse>(&body).unwrap();

                    result
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(DescribeGroupsError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Lists the specified notification subscriptions.</p>
    fn describe_notification_subscriptions(
        &self,
        input: DescribeNotificationSubscriptionsRequest,
    ) -> RusotoFuture<
        DescribeNotificationSubscriptionsResponse,
        DescribeNotificationSubscriptionsError,
    > {
        let request_uri = format!(
            "/api/v1/organizations/{organization_id}/subscriptions",
            organization_id = input.organization_id
        );

        let mut request = SignedRequest::new("GET", "workdocs", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut params = Params::new();
        if let Some(ref x) = input.limit {
            params.put("limit", x);
        }
        if let Some(ref x) = input.marker {
            params.put("marker", x);
        }
        request.set_params(params);

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result = serde_json::from_slice::<DescribeNotificationSubscriptionsResponse>(
                        &body,
                    ).unwrap();

                    result
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(DescribeNotificationSubscriptionsError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Describes the permissions of a specified resource.</p>
    fn describe_resource_permissions(
        &self,
        input: DescribeResourcePermissionsRequest,
    ) -> RusotoFuture<DescribeResourcePermissionsResponse, DescribeResourcePermissionsError> {
        let request_uri = format!(
            "/api/v1/resources/{resource_id}/permissions",
            resource_id = input.resource_id
        );

        let mut request = SignedRequest::new("GET", "workdocs", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        if let Some(ref authentication_token) = input.authentication_token {
            request.add_header("Authentication", &authentication_token.to_string());
        }
        let mut params = Params::new();
        if let Some(ref x) = input.limit {
            params.put("limit", x);
        }
        if let Some(ref x) = input.marker {
            params.put("marker", x);
        }
        if let Some(ref x) = input.principal_id {
            params.put("principalId", x);
        }
        request.set_params(params);

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result = serde_json::from_slice::<DescribeResourcePermissionsResponse>(
                        &body,
                    ).unwrap();

                    result
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(DescribeResourcePermissionsError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Describes the current user's special folders; the <code>RootFolder</code> and the <code>RecycleBin</code>. <code>RootFolder</code> is the root of user's files and folders and <code>RecycleBin</code> is the root of recycled items. This is not a valid action for SigV4 (administrative API) clients.</p>
    fn describe_root_folders(
        &self,
        input: DescribeRootFoldersRequest,
    ) -> RusotoFuture<DescribeRootFoldersResponse, DescribeRootFoldersError> {
        let request_uri = "/api/v1/me/root";

        let mut request = SignedRequest::new("GET", "workdocs", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.add_header("Authentication", &input.authentication_token);
        let mut params = Params::new();
        if let Some(ref x) = input.limit {
            params.put("limit", x);
        }
        if let Some(ref x) = input.marker {
            params.put("marker", x);
        }
        request.set_params(params);

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result =
                        serde_json::from_slice::<DescribeRootFoldersResponse>(&body).unwrap();

                    result
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(DescribeRootFoldersError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Describes the specified users. You can describe all users or filter the results (for example, by status or organization).</p> <p>By default, Amazon WorkDocs returns the first 24 active or pending users. If there are more results, the response includes a marker that you can use to request the next set of results.</p>
    fn describe_users(
        &self,
        input: DescribeUsersRequest,
    ) -> RusotoFuture<DescribeUsersResponse, DescribeUsersError> {
        let request_uri = "/api/v1/users";

        let mut request = SignedRequest::new("GET", "workdocs", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        if let Some(ref authentication_token) = input.authentication_token {
            request.add_header("Authentication", &authentication_token.to_string());
        }
        let mut params = Params::new();
        if let Some(ref x) = input.fields {
            params.put("fields", x);
        }
        if let Some(ref x) = input.include {
            params.put("include", x);
        }
        if let Some(ref x) = input.limit {
            params.put("limit", x);
        }
        if let Some(ref x) = input.marker {
            params.put("marker", x);
        }
        if let Some(ref x) = input.order {
            params.put("order", x);
        }
        if let Some(ref x) = input.organization_id {
            params.put("organizationId", x);
        }
        if let Some(ref x) = input.query {
            params.put("query", x);
        }
        if let Some(ref x) = input.sort {
            params.put("sort", x);
        }
        if let Some(ref x) = input.user_ids {
            params.put("userIds", x);
        }
        request.set_params(params);

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result = serde_json::from_slice::<DescribeUsersResponse>(&body).unwrap();

                    result
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(DescribeUsersError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Retrieves details of the current user for whom the authentication token was generated. This is not a valid action for SigV4 (administrative API) clients.</p>
    fn get_current_user(
        &self,
        input: GetCurrentUserRequest,
    ) -> RusotoFuture<GetCurrentUserResponse, GetCurrentUserError> {
        let request_uri = "/api/v1/me";

        let mut request = SignedRequest::new("GET", "workdocs", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.add_header("Authentication", &input.authentication_token);

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result = serde_json::from_slice::<GetCurrentUserResponse>(&body).unwrap();

                    result
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(GetCurrentUserError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Retrieves details of a document.</p>
    fn get_document(
        &self,
        input: GetDocumentRequest,
    ) -> RusotoFuture<GetDocumentResponse, GetDocumentError> {
        let request_uri = format!(
            "/api/v1/documents/{document_id}",
            document_id = input.document_id
        );

        let mut request = SignedRequest::new("GET", "workdocs", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        if let Some(ref authentication_token) = input.authentication_token {
            request.add_header("Authentication", &authentication_token.to_string());
        }
        let mut params = Params::new();
        if let Some(ref x) = input.include_custom_metadata {
            params.put("includeCustomMetadata", x);
        }
        request.set_params(params);

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result = serde_json::from_slice::<GetDocumentResponse>(&body).unwrap();

                    result
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(GetDocumentError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Retrieves the path information (the hierarchy from the root folder) for the requested document.</p> <p>By default, Amazon WorkDocs returns a maximum of 100 levels upwards from the requested document and only includes the IDs of the parent folders in the path. You can limit the maximum number of levels. You can also request the names of the parent folders.</p>
    fn get_document_path(
        &self,
        input: GetDocumentPathRequest,
    ) -> RusotoFuture<GetDocumentPathResponse, GetDocumentPathError> {
        let request_uri = format!(
            "/api/v1/documents/{document_id}/path",
            document_id = input.document_id
        );

        let mut request = SignedRequest::new("GET", "workdocs", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        if let Some(ref authentication_token) = input.authentication_token {
            request.add_header("Authentication", &authentication_token.to_string());
        }
        let mut params = Params::new();
        if let Some(ref x) = input.fields {
            params.put("fields", x);
        }
        if let Some(ref x) = input.limit {
            params.put("limit", x);
        }
        if let Some(ref x) = input.marker {
            params.put("marker", x);
        }
        request.set_params(params);

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result = serde_json::from_slice::<GetDocumentPathResponse>(&body).unwrap();

                    result
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(GetDocumentPathError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Retrieves version metadata for the specified document.</p>
    fn get_document_version(
        &self,
        input: GetDocumentVersionRequest,
    ) -> RusotoFuture<GetDocumentVersionResponse, GetDocumentVersionError> {
        let request_uri = format!(
            "/api/v1/documents/{document_id}/versions/{version_id}",
            document_id = input.document_id,
            version_id = input.version_id
        );

        let mut request = SignedRequest::new("GET", "workdocs", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        if let Some(ref authentication_token) = input.authentication_token {
            request.add_header("Authentication", &authentication_token.to_string());
        }
        let mut params = Params::new();
        if let Some(ref x) = input.fields {
            params.put("fields", x);
        }
        if let Some(ref x) = input.include_custom_metadata {
            params.put("includeCustomMetadata", x);
        }
        request.set_params(params);

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result =
                        serde_json::from_slice::<GetDocumentVersionResponse>(&body).unwrap();

                    result
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(GetDocumentVersionError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Retrieves the metadata of the specified folder.</p>
    fn get_folder(
        &self,
        input: GetFolderRequest,
    ) -> RusotoFuture<GetFolderResponse, GetFolderError> {
        let request_uri = format!("/api/v1/folders/{folder_id}", folder_id = input.folder_id);

        let mut request = SignedRequest::new("GET", "workdocs", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        if let Some(ref authentication_token) = input.authentication_token {
            request.add_header("Authentication", &authentication_token.to_string());
        }
        let mut params = Params::new();
        if let Some(ref x) = input.include_custom_metadata {
            params.put("includeCustomMetadata", x);
        }
        request.set_params(params);

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result = serde_json::from_slice::<GetFolderResponse>(&body).unwrap();

                    result
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(GetFolderError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Retrieves the path information (the hierarchy from the root folder) for the specified folder.</p> <p>By default, Amazon WorkDocs returns a maximum of 100 levels upwards from the requested folder and only includes the IDs of the parent folders in the path. You can limit the maximum number of levels. You can also request the parent folder names.</p>
    fn get_folder_path(
        &self,
        input: GetFolderPathRequest,
    ) -> RusotoFuture<GetFolderPathResponse, GetFolderPathError> {
        let request_uri = format!(
            "/api/v1/folders/{folder_id}/path",
            folder_id = input.folder_id
        );

        let mut request = SignedRequest::new("GET", "workdocs", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        if let Some(ref authentication_token) = input.authentication_token {
            request.add_header("Authentication", &authentication_token.to_string());
        }
        let mut params = Params::new();
        if let Some(ref x) = input.fields {
            params.put("fields", x);
        }
        if let Some(ref x) = input.limit {
            params.put("limit", x);
        }
        if let Some(ref x) = input.marker {
            params.put("marker", x);
        }
        request.set_params(params);

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result = serde_json::from_slice::<GetFolderPathResponse>(&body).unwrap();

                    result
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(GetFolderPathError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Creates a new document object and version object.</p> <p>The client specifies the parent folder ID and name of the document to upload. The ID is optionally specified when creating a new version of an existing document. This is the first step to upload a document. Next, upload the document to the URL returned from the call, and then call <a>UpdateDocumentVersion</a>.</p> <p>To cancel the document upload, call <a>AbortDocumentVersionUpload</a>.</p>
    fn initiate_document_version_upload(
        &self,
        input: InitiateDocumentVersionUploadRequest,
    ) -> RusotoFuture<InitiateDocumentVersionUploadResponse, InitiateDocumentVersionUploadError>
    {
        let request_uri = "/api/v1/documents";

        let mut request = SignedRequest::new("POST", "workdocs", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        if let Some(ref authentication_token) = input.authentication_token {
            request.add_header("Authentication", &authentication_token.to_string());
        }

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 201 {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result = serde_json::from_slice::<InitiateDocumentVersionUploadResponse>(
                        &body,
                    ).unwrap();

                    result
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(InitiateDocumentVersionUploadError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Removes all the permissions from the specified resource.</p>
    fn remove_all_resource_permissions(
        &self,
        input: RemoveAllResourcePermissionsRequest,
    ) -> RusotoFuture<(), RemoveAllResourcePermissionsError> {
        let request_uri = format!(
            "/api/v1/resources/{resource_id}/permissions",
            resource_id = input.resource_id
        );

        let mut request = SignedRequest::new("DELETE", "workdocs", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        if let Some(ref authentication_token) = input.authentication_token {
            request.add_header("Authentication", &authentication_token.to_string());
        }

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 204 {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let result = ::std::mem::drop(response);

                    result
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(RemoveAllResourcePermissionsError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Removes the permission for the specified principal from the specified resource.</p>
    fn remove_resource_permission(
        &self,
        input: RemoveResourcePermissionRequest,
    ) -> RusotoFuture<(), RemoveResourcePermissionError> {
        let request_uri = format!(
            "/api/v1/resources/{resource_id}/permissions/{principal_id}",
            principal_id = input.principal_id,
            resource_id = input.resource_id
        );

        let mut request = SignedRequest::new("DELETE", "workdocs", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        if let Some(ref authentication_token) = input.authentication_token {
            request.add_header("Authentication", &authentication_token.to_string());
        }
        let mut params = Params::new();
        if let Some(ref x) = input.principal_type {
            params.put("type", x);
        }
        request.set_params(params);

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 204 {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let result = ::std::mem::drop(response);

                    result
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(RemoveResourcePermissionError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Updates the specified attributes of a document. The user must have access to both the document and its parent folder, if applicable.</p>
    fn update_document(
        &self,
        input: UpdateDocumentRequest,
    ) -> RusotoFuture<(), UpdateDocumentError> {
        let request_uri = format!(
            "/api/v1/documents/{document_id}",
            document_id = input.document_id
        );

        let mut request = SignedRequest::new("PATCH", "workdocs", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        if let Some(ref authentication_token) = input.authentication_token {
            request.add_header("Authentication", &authentication_token.to_string());
        }

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let result = ::std::mem::drop(response);

                    result
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(UpdateDocumentError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Changes the status of the document version to ACTIVE. </p> <p>Amazon WorkDocs also sets its document container to ACTIVE. This is the last step in a document upload, after the client uploads the document to an S3-presigned URL returned by <a>InitiateDocumentVersionUpload</a>. </p>
    fn update_document_version(
        &self,
        input: UpdateDocumentVersionRequest,
    ) -> RusotoFuture<(), UpdateDocumentVersionError> {
        let request_uri = format!(
            "/api/v1/documents/{document_id}/versions/{version_id}",
            document_id = input.document_id,
            version_id = input.version_id
        );

        let mut request = SignedRequest::new("PATCH", "workdocs", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        if let Some(ref authentication_token) = input.authentication_token {
            request.add_header("Authentication", &authentication_token.to_string());
        }

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let result = ::std::mem::drop(response);

                    result
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(UpdateDocumentVersionError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Updates the specified attributes of the specified folder. The user must have access to both the folder and its parent folder, if applicable.</p>
    fn update_folder(&self, input: UpdateFolderRequest) -> RusotoFuture<(), UpdateFolderError> {
        let request_uri = format!("/api/v1/folders/{folder_id}", folder_id = input.folder_id);

        let mut request = SignedRequest::new("PATCH", "workdocs", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        if let Some(ref authentication_token) = input.authentication_token {
            request.add_header("Authentication", &authentication_token.to_string());
        }

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let result = ::std::mem::drop(response);

                    result
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(UpdateFolderError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Updates the specified attributes of the specified user, and grants or revokes administrative privileges to the Amazon WorkDocs site.</p>
    fn update_user(
        &self,
        input: UpdateUserRequest,
    ) -> RusotoFuture<UpdateUserResponse, UpdateUserError> {
        let request_uri = format!("/api/v1/users/{user_id}", user_id = input.user_id);

        let mut request = SignedRequest::new("PATCH", "workdocs", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        if let Some(ref authentication_token) = input.authentication_token {
            request.add_header("Authentication", &authentication_token.to_string());
        }

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result = serde_json::from_slice::<UpdateUserResponse>(&body).unwrap();

                    result
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(UpdateUserError::from_body(
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
