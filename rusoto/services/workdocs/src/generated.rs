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

use rusoto_core::param::{Params, ServiceParams};
use rusoto_core::proto;
use rusoto_core::signature::SignedRequest;
#[allow(unused_imports)]
use serde::{Deserialize, Serialize};
use serde_json;
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ActivateUserRequest {
    /// <p>Amazon WorkDocs authentication token. Do not set this field when using administrative API actions, as in accessing the API using AWS credentials.</p>
    #[serde(rename = "AuthenticationToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authentication_token: Option<String>,
    /// <p>The ID of the user.</p>
    #[serde(rename = "UserId")]
    pub user_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ActivateUserResponse {
    /// <p>The user information.</p>
    #[serde(rename = "User")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<User>,
}

/// <p>Describes the activity information.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct Activity {
    /// <p>Metadata of the commenting activity. This is an optional field and is filled for commenting activities.</p>
    #[serde(rename = "CommentMetadata")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment_metadata: Option<CommentMetadata>,
    /// <p>The user who performed the action.</p>
    #[serde(rename = "Initiator")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub initiator: Option<UserMetadata>,
    /// <p>Indicates whether an activity is indirect or direct. An indirect activity results from a direct activity performed on a parent resource. For example, sharing a parent folder (the direct activity) shares all of the subfolders and documents within the parent folder (the indirect activity).</p>
    #[serde(rename = "IsIndirectActivity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_indirect_activity: Option<bool>,
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

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct AddResourcePermissionsResponse {
    /// <p>The share results.</p>
    #[serde(rename = "ShareResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub share_results: Option<Vec<ShareResult>>,
}

/// <p>Describes a comment.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateCommentResponse {
    /// <p>The comment that has been created.</p>
    #[serde(rename = "Comment")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment: Option<Comment>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateCustomMetadataResponse {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateFolderResponse {
    /// <p>The metadata of the folder.</p>
    #[serde(rename = "Metadata")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<FolderMetadata>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateLabelsResponse {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateNotificationSubscriptionRequest {
    /// <p>The endpoint to receive the notifications. If the protocol is HTTPS, the endpoint is a URL that begins with <code>https</code>.</p>
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

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateNotificationSubscriptionResponse {
    /// <p>The subscription.</p>
    #[serde(rename = "Subscription")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscription: Option<Subscription>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateUserResponse {
    /// <p>The user information.</p>
    #[serde(rename = "User")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<User>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeactivateUserRequest {
    /// <p>Amazon WorkDocs authentication token. Do not set this field when using administrative API actions, as in accessing the API using AWS credentials.</p>
    #[serde(rename = "AuthenticationToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authentication_token: Option<String>,
    /// <p>The ID of the user.</p>
    #[serde(rename = "UserId")]
    pub user_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DeleteCustomMetadataResponse {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteDocumentRequest {
    /// <p>Amazon WorkDocs authentication token. Do not set this field when using administrative API actions, as in accessing the API using AWS credentials.</p>
    #[serde(rename = "AuthenticationToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authentication_token: Option<String>,
    /// <p>The ID of the document.</p>
    #[serde(rename = "DocumentId")]
    pub document_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteFolderContentsRequest {
    /// <p>Amazon WorkDocs authentication token. Do not set this field when using administrative API actions, as in accessing the API using AWS credentials.</p>
    #[serde(rename = "AuthenticationToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authentication_token: Option<String>,
    /// <p>The ID of the folder.</p>
    #[serde(rename = "FolderId")]
    pub folder_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteFolderRequest {
    /// <p>Amazon WorkDocs authentication token. Do not set this field when using administrative API actions, as in accessing the API using AWS credentials.</p>
    #[serde(rename = "AuthenticationToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authentication_token: Option<String>,
    /// <p>The ID of the folder.</p>
    #[serde(rename = "FolderId")]
    pub folder_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DeleteLabelsResponse {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteNotificationSubscriptionRequest {
    /// <p>The ID of the organization.</p>
    #[serde(rename = "OrganizationId")]
    pub organization_id: String,
    /// <p>The ID of the subscription.</p>
    #[serde(rename = "SubscriptionId")]
    pub subscription_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteUserRequest {
    /// <p>Amazon WorkDocs authentication token. Do not set this field when using administrative API actions, as in accessing the API using AWS credentials.</p>
    #[serde(rename = "AuthenticationToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authentication_token: Option<String>,
    /// <p>The ID of the user.</p>
    #[serde(rename = "UserId")]
    pub user_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeActivitiesRequest {
    /// <p>Specifies which activity types to include in the response. If this field is left empty, all activity types are returned.</p>
    #[serde(rename = "ActivityTypes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub activity_types: Option<String>,
    /// <p>Amazon WorkDocs authentication token. Do not set this field when using administrative API actions, as in accessing the API using AWS credentials.</p>
    #[serde(rename = "AuthenticationToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authentication_token: Option<String>,
    /// <p>The timestamp that determines the end time of the activities. The response includes the activities performed before the specified timestamp.</p>
    #[serde(rename = "EndTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<f64>,
    /// <p>Includes indirect activities. An indirect activity results from a direct activity performed on a parent resource. For example, sharing a parent folder (the direct activity) shares all of the subfolders and documents within the parent folder (the indirect activity).</p>
    #[serde(rename = "IncludeIndirectActivities")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_indirect_activities: Option<bool>,
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
    /// <p>The document or folder ID for which to describe activity types.</p>
    #[serde(rename = "ResourceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_id: Option<String>,
    /// <p>The timestamp that determines the starting time of the activities. The response includes the activities performed after the specified timestamp.</p>
    #[serde(rename = "StartTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<f64>,
    /// <p>The ID of the user who performed the action. The response includes activities pertaining to this user. This is an optional parameter and is only applicable for administrative API (SigV4) requests.</p>
    #[serde(rename = "UserId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetCurrentUserRequest {
    /// <p>Amazon WorkDocs authentication token. Do not set this field when using administrative API actions, as in accessing the API using AWS credentials.</p>
    #[serde(rename = "AuthenticationToken")]
    pub authentication_token: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetCurrentUserResponse {
    /// <p>Metadata of the user.</p>
    #[serde(rename = "User")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<User>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetDocumentPathResponse {
    /// <p>The path information.</p>
    #[serde(rename = "Path")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<ResourcePath>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetFolderPathResponse {
    /// <p>The path information.</p>
    #[serde(rename = "Path")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<ResourcePath>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetResourcesRequest {
    /// <p>The Amazon WorkDocs authentication token. Do not set this field when using administrative API actions, as in accessing the API operation using AWS credentials.</p>
    #[serde(rename = "AuthenticationToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authentication_token: Option<String>,
    /// <p>The collection type.</p>
    #[serde(rename = "CollectionType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub collection_type: Option<String>,
    /// <p>The maximum number of resources to return.</p>
    #[serde(rename = "Limit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// <p>The marker for the next set of results. This marker was received from a previous call.</p>
    #[serde(rename = "Marker")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
    /// <p>The user ID for the resource collection. This is a required field for accessing the API operation using IAM credentials.</p>
    #[serde(rename = "UserId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetResourcesResponse {
    /// <p>The documents in the specified collection.</p>
    #[serde(rename = "Documents")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub documents: Option<Vec<DocumentMetadata>>,
    /// <p>The folders in the specified folder.</p>
    #[serde(rename = "Folders")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub folders: Option<Vec<FolderMetadata>>,
    /// <p>The marker to use when requesting the next set of results. If there are no additional results, the string is empty.</p>
    #[serde(rename = "Marker")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
}

/// <p>Describes the metadata of a user group.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct RemoveAllResourcePermissionsRequest {
    /// <p>Amazon WorkDocs authentication token. Do not set this field when using administrative API actions, as in accessing the API using AWS credentials.</p>
    #[serde(rename = "AuthenticationToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authentication_token: Option<String>,
    /// <p>The ID of the resource.</p>
    #[serde(rename = "ResourceId")]
    pub resource_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ResourcePath {
    /// <p>The components of the resource path.</p>
    #[serde(rename = "Components")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub components: Option<Vec<ResourcePathComponent>>,
}

/// <p>Describes the resource path.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ShareResult {
    /// <p>The ID of the invited user.</p>
    #[serde(rename = "InviteePrincipalId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invitee_principal_id: Option<String>,
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
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
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
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UpdateUserResponse {
    /// <p>The user information.</p>
    #[serde(rename = "User")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<User>,
}

/// <p>Describes the upload.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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
}

impl AbortDocumentVersionUploadError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<AbortDocumentVersionUploadError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "EntityNotExistsException" => {
                    return RusotoError::Service(AbortDocumentVersionUploadError::EntityNotExists(
                        err.msg,
                    ))
                }
                "FailedDependencyException" => {
                    return RusotoError::Service(AbortDocumentVersionUploadError::FailedDependency(
                        err.msg,
                    ))
                }
                "ProhibitedStateException" => {
                    return RusotoError::Service(AbortDocumentVersionUploadError::ProhibitedState(
                        err.msg,
                    ))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(
                        AbortDocumentVersionUploadError::ServiceUnavailable(err.msg),
                    )
                }
                "UnauthorizedOperationException" => {
                    return RusotoError::Service(
                        AbortDocumentVersionUploadError::UnauthorizedOperation(err.msg),
                    )
                }
                "UnauthorizedResourceAccessException" => {
                    return RusotoError::Service(
                        AbortDocumentVersionUploadError::UnauthorizedResourceAccess(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for AbortDocumentVersionUploadError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            AbortDocumentVersionUploadError::EntityNotExists(ref cause) => write!(f, "{}", cause),
            AbortDocumentVersionUploadError::FailedDependency(ref cause) => write!(f, "{}", cause),
            AbortDocumentVersionUploadError::ProhibitedState(ref cause) => write!(f, "{}", cause),
            AbortDocumentVersionUploadError::ServiceUnavailable(ref cause) => {
                write!(f, "{}", cause)
            }
            AbortDocumentVersionUploadError::UnauthorizedOperation(ref cause) => {
                write!(f, "{}", cause)
            }
            AbortDocumentVersionUploadError::UnauthorizedResourceAccess(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for AbortDocumentVersionUploadError {}
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
}

impl ActivateUserError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ActivateUserError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "EntityNotExistsException" => {
                    return RusotoError::Service(ActivateUserError::EntityNotExists(err.msg))
                }
                "FailedDependencyException" => {
                    return RusotoError::Service(ActivateUserError::FailedDependency(err.msg))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(ActivateUserError::ServiceUnavailable(err.msg))
                }
                "UnauthorizedOperationException" => {
                    return RusotoError::Service(ActivateUserError::UnauthorizedOperation(err.msg))
                }
                "UnauthorizedResourceAccessException" => {
                    return RusotoError::Service(ActivateUserError::UnauthorizedResourceAccess(
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
impl fmt::Display for ActivateUserError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ActivateUserError::EntityNotExists(ref cause) => write!(f, "{}", cause),
            ActivateUserError::FailedDependency(ref cause) => write!(f, "{}", cause),
            ActivateUserError::ServiceUnavailable(ref cause) => write!(f, "{}", cause),
            ActivateUserError::UnauthorizedOperation(ref cause) => write!(f, "{}", cause),
            ActivateUserError::UnauthorizedResourceAccess(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ActivateUserError {}
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
}

impl AddResourcePermissionsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<AddResourcePermissionsError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "FailedDependencyException" => {
                    return RusotoError::Service(AddResourcePermissionsError::FailedDependency(
                        err.msg,
                    ))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(AddResourcePermissionsError::ServiceUnavailable(
                        err.msg,
                    ))
                }
                "UnauthorizedOperationException" => {
                    return RusotoError::Service(
                        AddResourcePermissionsError::UnauthorizedOperation(err.msg),
                    )
                }
                "UnauthorizedResourceAccessException" => {
                    return RusotoError::Service(
                        AddResourcePermissionsError::UnauthorizedResourceAccess(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for AddResourcePermissionsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            AddResourcePermissionsError::FailedDependency(ref cause) => write!(f, "{}", cause),
            AddResourcePermissionsError::ServiceUnavailable(ref cause) => write!(f, "{}", cause),
            AddResourcePermissionsError::UnauthorizedOperation(ref cause) => write!(f, "{}", cause),
            AddResourcePermissionsError::UnauthorizedResourceAccess(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for AddResourcePermissionsError {}
/// Errors returned by CreateComment
#[derive(Debug, PartialEq)]
pub enum CreateCommentError {
    /// <p>This exception is thrown when the document is locked for comments and user tries to create or delete a comment on that document.</p>
    DocumentLockedForComments(String),
    /// <p>The resource does not exist.</p>
    EntityNotExists(String),
    /// <p>The AWS Directory Service cannot reach an on-premises instance. Or a dependency under the control of the organization is failing, such as a connected Active Directory.</p>
    FailedDependency(String),
    /// <p>The requested operation is not allowed on the specified comment object.</p>
    InvalidCommentOperation(String),
    /// <p>The specified document version is not in the INITIALIZED state.</p>
    ProhibitedState(String),
    /// <p>One or more of the dependencies is unavailable.</p>
    ServiceUnavailable(String),
    /// <p>The operation is not permitted.</p>
    UnauthorizedOperation(String),
    /// <p>The caller does not have access to perform the action on the resource.</p>
    UnauthorizedResourceAccess(String),
}

impl CreateCommentError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateCommentError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "DocumentLockedForCommentsException" => {
                    return RusotoError::Service(CreateCommentError::DocumentLockedForComments(
                        err.msg,
                    ))
                }
                "EntityNotExistsException" => {
                    return RusotoError::Service(CreateCommentError::EntityNotExists(err.msg))
                }
                "FailedDependencyException" => {
                    return RusotoError::Service(CreateCommentError::FailedDependency(err.msg))
                }
                "InvalidCommentOperationException" => {
                    return RusotoError::Service(CreateCommentError::InvalidCommentOperation(
                        err.msg,
                    ))
                }
                "ProhibitedStateException" => {
                    return RusotoError::Service(CreateCommentError::ProhibitedState(err.msg))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(CreateCommentError::ServiceUnavailable(err.msg))
                }
                "UnauthorizedOperationException" => {
                    return RusotoError::Service(CreateCommentError::UnauthorizedOperation(err.msg))
                }
                "UnauthorizedResourceAccessException" => {
                    return RusotoError::Service(CreateCommentError::UnauthorizedResourceAccess(
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
impl fmt::Display for CreateCommentError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreateCommentError::DocumentLockedForComments(ref cause) => write!(f, "{}", cause),
            CreateCommentError::EntityNotExists(ref cause) => write!(f, "{}", cause),
            CreateCommentError::FailedDependency(ref cause) => write!(f, "{}", cause),
            CreateCommentError::InvalidCommentOperation(ref cause) => write!(f, "{}", cause),
            CreateCommentError::ProhibitedState(ref cause) => write!(f, "{}", cause),
            CreateCommentError::ServiceUnavailable(ref cause) => write!(f, "{}", cause),
            CreateCommentError::UnauthorizedOperation(ref cause) => write!(f, "{}", cause),
            CreateCommentError::UnauthorizedResourceAccess(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CreateCommentError {}
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
}

impl CreateCustomMetadataError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateCustomMetadataError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "CustomMetadataLimitExceededException" => {
                    return RusotoError::Service(
                        CreateCustomMetadataError::CustomMetadataLimitExceeded(err.msg),
                    )
                }
                "EntityNotExistsException" => {
                    return RusotoError::Service(CreateCustomMetadataError::EntityNotExists(
                        err.msg,
                    ))
                }
                "FailedDependencyException" => {
                    return RusotoError::Service(CreateCustomMetadataError::FailedDependency(
                        err.msg,
                    ))
                }
                "ProhibitedStateException" => {
                    return RusotoError::Service(CreateCustomMetadataError::ProhibitedState(
                        err.msg,
                    ))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(CreateCustomMetadataError::ServiceUnavailable(
                        err.msg,
                    ))
                }
                "UnauthorizedOperationException" => {
                    return RusotoError::Service(CreateCustomMetadataError::UnauthorizedOperation(
                        err.msg,
                    ))
                }
                "UnauthorizedResourceAccessException" => {
                    return RusotoError::Service(
                        CreateCustomMetadataError::UnauthorizedResourceAccess(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for CreateCustomMetadataError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreateCustomMetadataError::CustomMetadataLimitExceeded(ref cause) => {
                write!(f, "{}", cause)
            }
            CreateCustomMetadataError::EntityNotExists(ref cause) => write!(f, "{}", cause),
            CreateCustomMetadataError::FailedDependency(ref cause) => write!(f, "{}", cause),
            CreateCustomMetadataError::ProhibitedState(ref cause) => write!(f, "{}", cause),
            CreateCustomMetadataError::ServiceUnavailable(ref cause) => write!(f, "{}", cause),
            CreateCustomMetadataError::UnauthorizedOperation(ref cause) => write!(f, "{}", cause),
            CreateCustomMetadataError::UnauthorizedResourceAccess(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for CreateCustomMetadataError {}
/// Errors returned by CreateFolder
#[derive(Debug, PartialEq)]
pub enum CreateFolderError {
    /// <p>Another operation is in progress on the resource that conflicts with the current operation.</p>
    ConflictingOperation(String),
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
}

impl CreateFolderError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateFolderError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "ConflictingOperationException" => {
                    return RusotoError::Service(CreateFolderError::ConflictingOperation(err.msg))
                }
                "EntityAlreadyExistsException" => {
                    return RusotoError::Service(CreateFolderError::EntityAlreadyExists(err.msg))
                }
                "EntityNotExistsException" => {
                    return RusotoError::Service(CreateFolderError::EntityNotExists(err.msg))
                }
                "FailedDependencyException" => {
                    return RusotoError::Service(CreateFolderError::FailedDependency(err.msg))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(CreateFolderError::LimitExceeded(err.msg))
                }
                "ProhibitedStateException" => {
                    return RusotoError::Service(CreateFolderError::ProhibitedState(err.msg))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(CreateFolderError::ServiceUnavailable(err.msg))
                }
                "UnauthorizedOperationException" => {
                    return RusotoError::Service(CreateFolderError::UnauthorizedOperation(err.msg))
                }
                "UnauthorizedResourceAccessException" => {
                    return RusotoError::Service(CreateFolderError::UnauthorizedResourceAccess(
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
impl fmt::Display for CreateFolderError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreateFolderError::ConflictingOperation(ref cause) => write!(f, "{}", cause),
            CreateFolderError::EntityAlreadyExists(ref cause) => write!(f, "{}", cause),
            CreateFolderError::EntityNotExists(ref cause) => write!(f, "{}", cause),
            CreateFolderError::FailedDependency(ref cause) => write!(f, "{}", cause),
            CreateFolderError::LimitExceeded(ref cause) => write!(f, "{}", cause),
            CreateFolderError::ProhibitedState(ref cause) => write!(f, "{}", cause),
            CreateFolderError::ServiceUnavailable(ref cause) => write!(f, "{}", cause),
            CreateFolderError::UnauthorizedOperation(ref cause) => write!(f, "{}", cause),
            CreateFolderError::UnauthorizedResourceAccess(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CreateFolderError {}
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
}

impl CreateLabelsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateLabelsError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "EntityNotExistsException" => {
                    return RusotoError::Service(CreateLabelsError::EntityNotExists(err.msg))
                }
                "FailedDependencyException" => {
                    return RusotoError::Service(CreateLabelsError::FailedDependency(err.msg))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(CreateLabelsError::ServiceUnavailable(err.msg))
                }
                "TooManyLabelsException" => {
                    return RusotoError::Service(CreateLabelsError::TooManyLabels(err.msg))
                }
                "UnauthorizedOperationException" => {
                    return RusotoError::Service(CreateLabelsError::UnauthorizedOperation(err.msg))
                }
                "UnauthorizedResourceAccessException" => {
                    return RusotoError::Service(CreateLabelsError::UnauthorizedResourceAccess(
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
impl fmt::Display for CreateLabelsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreateLabelsError::EntityNotExists(ref cause) => write!(f, "{}", cause),
            CreateLabelsError::FailedDependency(ref cause) => write!(f, "{}", cause),
            CreateLabelsError::ServiceUnavailable(ref cause) => write!(f, "{}", cause),
            CreateLabelsError::TooManyLabels(ref cause) => write!(f, "{}", cause),
            CreateLabelsError::UnauthorizedOperation(ref cause) => write!(f, "{}", cause),
            CreateLabelsError::UnauthorizedResourceAccess(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CreateLabelsError {}
/// Errors returned by CreateNotificationSubscription
#[derive(Debug, PartialEq)]
pub enum CreateNotificationSubscriptionError {
    /// <p>One or more of the dependencies is unavailable.</p>
    ServiceUnavailable(String),
    /// <p>You've reached the limit on the number of subscriptions for the WorkDocs instance.</p>
    TooManySubscriptions(String),
    /// <p>The caller does not have access to perform the action on the resource.</p>
    UnauthorizedResourceAccess(String),
}

impl CreateNotificationSubscriptionError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<CreateNotificationSubscriptionError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "ServiceUnavailableException" => {
                    return RusotoError::Service(
                        CreateNotificationSubscriptionError::ServiceUnavailable(err.msg),
                    )
                }
                "TooManySubscriptionsException" => {
                    return RusotoError::Service(
                        CreateNotificationSubscriptionError::TooManySubscriptions(err.msg),
                    )
                }
                "UnauthorizedResourceAccessException" => {
                    return RusotoError::Service(
                        CreateNotificationSubscriptionError::UnauthorizedResourceAccess(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for CreateNotificationSubscriptionError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreateNotificationSubscriptionError::ServiceUnavailable(ref cause) => {
                write!(f, "{}", cause)
            }
            CreateNotificationSubscriptionError::TooManySubscriptions(ref cause) => {
                write!(f, "{}", cause)
            }
            CreateNotificationSubscriptionError::UnauthorizedResourceAccess(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for CreateNotificationSubscriptionError {}
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
}

impl CreateUserError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateUserError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "EntityAlreadyExistsException" => {
                    return RusotoError::Service(CreateUserError::EntityAlreadyExists(err.msg))
                }
                "FailedDependencyException" => {
                    return RusotoError::Service(CreateUserError::FailedDependency(err.msg))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(CreateUserError::ServiceUnavailable(err.msg))
                }
                "UnauthorizedOperationException" => {
                    return RusotoError::Service(CreateUserError::UnauthorizedOperation(err.msg))
                }
                "UnauthorizedResourceAccessException" => {
                    return RusotoError::Service(CreateUserError::UnauthorizedResourceAccess(
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
impl fmt::Display for CreateUserError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreateUserError::EntityAlreadyExists(ref cause) => write!(f, "{}", cause),
            CreateUserError::FailedDependency(ref cause) => write!(f, "{}", cause),
            CreateUserError::ServiceUnavailable(ref cause) => write!(f, "{}", cause),
            CreateUserError::UnauthorizedOperation(ref cause) => write!(f, "{}", cause),
            CreateUserError::UnauthorizedResourceAccess(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CreateUserError {}
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
}

impl DeactivateUserError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeactivateUserError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "EntityNotExistsException" => {
                    return RusotoError::Service(DeactivateUserError::EntityNotExists(err.msg))
                }
                "FailedDependencyException" => {
                    return RusotoError::Service(DeactivateUserError::FailedDependency(err.msg))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(DeactivateUserError::ServiceUnavailable(err.msg))
                }
                "UnauthorizedOperationException" => {
                    return RusotoError::Service(DeactivateUserError::UnauthorizedOperation(
                        err.msg,
                    ))
                }
                "UnauthorizedResourceAccessException" => {
                    return RusotoError::Service(DeactivateUserError::UnauthorizedResourceAccess(
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
impl fmt::Display for DeactivateUserError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeactivateUserError::EntityNotExists(ref cause) => write!(f, "{}", cause),
            DeactivateUserError::FailedDependency(ref cause) => write!(f, "{}", cause),
            DeactivateUserError::ServiceUnavailable(ref cause) => write!(f, "{}", cause),
            DeactivateUserError::UnauthorizedOperation(ref cause) => write!(f, "{}", cause),
            DeactivateUserError::UnauthorizedResourceAccess(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeactivateUserError {}
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
}

impl DeleteCommentError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteCommentError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "DocumentLockedForCommentsException" => {
                    return RusotoError::Service(DeleteCommentError::DocumentLockedForComments(
                        err.msg,
                    ))
                }
                "EntityNotExistsException" => {
                    return RusotoError::Service(DeleteCommentError::EntityNotExists(err.msg))
                }
                "FailedDependencyException" => {
                    return RusotoError::Service(DeleteCommentError::FailedDependency(err.msg))
                }
                "ProhibitedStateException" => {
                    return RusotoError::Service(DeleteCommentError::ProhibitedState(err.msg))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(DeleteCommentError::ServiceUnavailable(err.msg))
                }
                "UnauthorizedOperationException" => {
                    return RusotoError::Service(DeleteCommentError::UnauthorizedOperation(err.msg))
                }
                "UnauthorizedResourceAccessException" => {
                    return RusotoError::Service(DeleteCommentError::UnauthorizedResourceAccess(
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
impl fmt::Display for DeleteCommentError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteCommentError::DocumentLockedForComments(ref cause) => write!(f, "{}", cause),
            DeleteCommentError::EntityNotExists(ref cause) => write!(f, "{}", cause),
            DeleteCommentError::FailedDependency(ref cause) => write!(f, "{}", cause),
            DeleteCommentError::ProhibitedState(ref cause) => write!(f, "{}", cause),
            DeleteCommentError::ServiceUnavailable(ref cause) => write!(f, "{}", cause),
            DeleteCommentError::UnauthorizedOperation(ref cause) => write!(f, "{}", cause),
            DeleteCommentError::UnauthorizedResourceAccess(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeleteCommentError {}
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
}

impl DeleteCustomMetadataError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteCustomMetadataError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "EntityNotExistsException" => {
                    return RusotoError::Service(DeleteCustomMetadataError::EntityNotExists(
                        err.msg,
                    ))
                }
                "FailedDependencyException" => {
                    return RusotoError::Service(DeleteCustomMetadataError::FailedDependency(
                        err.msg,
                    ))
                }
                "ProhibitedStateException" => {
                    return RusotoError::Service(DeleteCustomMetadataError::ProhibitedState(
                        err.msg,
                    ))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(DeleteCustomMetadataError::ServiceUnavailable(
                        err.msg,
                    ))
                }
                "UnauthorizedOperationException" => {
                    return RusotoError::Service(DeleteCustomMetadataError::UnauthorizedOperation(
                        err.msg,
                    ))
                }
                "UnauthorizedResourceAccessException" => {
                    return RusotoError::Service(
                        DeleteCustomMetadataError::UnauthorizedResourceAccess(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for DeleteCustomMetadataError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteCustomMetadataError::EntityNotExists(ref cause) => write!(f, "{}", cause),
            DeleteCustomMetadataError::FailedDependency(ref cause) => write!(f, "{}", cause),
            DeleteCustomMetadataError::ProhibitedState(ref cause) => write!(f, "{}", cause),
            DeleteCustomMetadataError::ServiceUnavailable(ref cause) => write!(f, "{}", cause),
            DeleteCustomMetadataError::UnauthorizedOperation(ref cause) => write!(f, "{}", cause),
            DeleteCustomMetadataError::UnauthorizedResourceAccess(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for DeleteCustomMetadataError {}
/// Errors returned by DeleteDocument
#[derive(Debug, PartialEq)]
pub enum DeleteDocumentError {
    /// <p>The resource hierarchy is changing.</p>
    ConcurrentModification(String),
    /// <p>Another operation is in progress on the resource that conflicts with the current operation.</p>
    ConflictingOperation(String),
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
}

impl DeleteDocumentError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteDocumentError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "ConcurrentModificationException" => {
                    return RusotoError::Service(DeleteDocumentError::ConcurrentModification(
                        err.msg,
                    ))
                }
                "ConflictingOperationException" => {
                    return RusotoError::Service(DeleteDocumentError::ConflictingOperation(err.msg))
                }
                "EntityNotExistsException" => {
                    return RusotoError::Service(DeleteDocumentError::EntityNotExists(err.msg))
                }
                "FailedDependencyException" => {
                    return RusotoError::Service(DeleteDocumentError::FailedDependency(err.msg))
                }
                "ProhibitedStateException" => {
                    return RusotoError::Service(DeleteDocumentError::ProhibitedState(err.msg))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(DeleteDocumentError::ServiceUnavailable(err.msg))
                }
                "UnauthorizedOperationException" => {
                    return RusotoError::Service(DeleteDocumentError::UnauthorizedOperation(
                        err.msg,
                    ))
                }
                "UnauthorizedResourceAccessException" => {
                    return RusotoError::Service(DeleteDocumentError::UnauthorizedResourceAccess(
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
impl fmt::Display for DeleteDocumentError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteDocumentError::ConcurrentModification(ref cause) => write!(f, "{}", cause),
            DeleteDocumentError::ConflictingOperation(ref cause) => write!(f, "{}", cause),
            DeleteDocumentError::EntityNotExists(ref cause) => write!(f, "{}", cause),
            DeleteDocumentError::FailedDependency(ref cause) => write!(f, "{}", cause),
            DeleteDocumentError::ProhibitedState(ref cause) => write!(f, "{}", cause),
            DeleteDocumentError::ServiceUnavailable(ref cause) => write!(f, "{}", cause),
            DeleteDocumentError::UnauthorizedOperation(ref cause) => write!(f, "{}", cause),
            DeleteDocumentError::UnauthorizedResourceAccess(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeleteDocumentError {}
/// Errors returned by DeleteFolder
#[derive(Debug, PartialEq)]
pub enum DeleteFolderError {
    /// <p>The resource hierarchy is changing.</p>
    ConcurrentModification(String),
    /// <p>Another operation is in progress on the resource that conflicts with the current operation.</p>
    ConflictingOperation(String),
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
}

impl DeleteFolderError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteFolderError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "ConcurrentModificationException" => {
                    return RusotoError::Service(DeleteFolderError::ConcurrentModification(err.msg))
                }
                "ConflictingOperationException" => {
                    return RusotoError::Service(DeleteFolderError::ConflictingOperation(err.msg))
                }
                "EntityNotExistsException" => {
                    return RusotoError::Service(DeleteFolderError::EntityNotExists(err.msg))
                }
                "FailedDependencyException" => {
                    return RusotoError::Service(DeleteFolderError::FailedDependency(err.msg))
                }
                "ProhibitedStateException" => {
                    return RusotoError::Service(DeleteFolderError::ProhibitedState(err.msg))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(DeleteFolderError::ServiceUnavailable(err.msg))
                }
                "UnauthorizedOperationException" => {
                    return RusotoError::Service(DeleteFolderError::UnauthorizedOperation(err.msg))
                }
                "UnauthorizedResourceAccessException" => {
                    return RusotoError::Service(DeleteFolderError::UnauthorizedResourceAccess(
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
impl fmt::Display for DeleteFolderError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteFolderError::ConcurrentModification(ref cause) => write!(f, "{}", cause),
            DeleteFolderError::ConflictingOperation(ref cause) => write!(f, "{}", cause),
            DeleteFolderError::EntityNotExists(ref cause) => write!(f, "{}", cause),
            DeleteFolderError::FailedDependency(ref cause) => write!(f, "{}", cause),
            DeleteFolderError::ProhibitedState(ref cause) => write!(f, "{}", cause),
            DeleteFolderError::ServiceUnavailable(ref cause) => write!(f, "{}", cause),
            DeleteFolderError::UnauthorizedOperation(ref cause) => write!(f, "{}", cause),
            DeleteFolderError::UnauthorizedResourceAccess(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeleteFolderError {}
/// Errors returned by DeleteFolderContents
#[derive(Debug, PartialEq)]
pub enum DeleteFolderContentsError {
    /// <p>Another operation is in progress on the resource that conflicts with the current operation.</p>
    ConflictingOperation(String),
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
}

impl DeleteFolderContentsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteFolderContentsError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "ConflictingOperationException" => {
                    return RusotoError::Service(DeleteFolderContentsError::ConflictingOperation(
                        err.msg,
                    ))
                }
                "EntityNotExistsException" => {
                    return RusotoError::Service(DeleteFolderContentsError::EntityNotExists(
                        err.msg,
                    ))
                }
                "FailedDependencyException" => {
                    return RusotoError::Service(DeleteFolderContentsError::FailedDependency(
                        err.msg,
                    ))
                }
                "ProhibitedStateException" => {
                    return RusotoError::Service(DeleteFolderContentsError::ProhibitedState(
                        err.msg,
                    ))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(DeleteFolderContentsError::ServiceUnavailable(
                        err.msg,
                    ))
                }
                "UnauthorizedOperationException" => {
                    return RusotoError::Service(DeleteFolderContentsError::UnauthorizedOperation(
                        err.msg,
                    ))
                }
                "UnauthorizedResourceAccessException" => {
                    return RusotoError::Service(
                        DeleteFolderContentsError::UnauthorizedResourceAccess(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for DeleteFolderContentsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteFolderContentsError::ConflictingOperation(ref cause) => write!(f, "{}", cause),
            DeleteFolderContentsError::EntityNotExists(ref cause) => write!(f, "{}", cause),
            DeleteFolderContentsError::FailedDependency(ref cause) => write!(f, "{}", cause),
            DeleteFolderContentsError::ProhibitedState(ref cause) => write!(f, "{}", cause),
            DeleteFolderContentsError::ServiceUnavailable(ref cause) => write!(f, "{}", cause),
            DeleteFolderContentsError::UnauthorizedOperation(ref cause) => write!(f, "{}", cause),
            DeleteFolderContentsError::UnauthorizedResourceAccess(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for DeleteFolderContentsError {}
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
}

impl DeleteLabelsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteLabelsError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "EntityNotExistsException" => {
                    return RusotoError::Service(DeleteLabelsError::EntityNotExists(err.msg))
                }
                "FailedDependencyException" => {
                    return RusotoError::Service(DeleteLabelsError::FailedDependency(err.msg))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(DeleteLabelsError::ServiceUnavailable(err.msg))
                }
                "UnauthorizedOperationException" => {
                    return RusotoError::Service(DeleteLabelsError::UnauthorizedOperation(err.msg))
                }
                "UnauthorizedResourceAccessException" => {
                    return RusotoError::Service(DeleteLabelsError::UnauthorizedResourceAccess(
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
impl fmt::Display for DeleteLabelsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteLabelsError::EntityNotExists(ref cause) => write!(f, "{}", cause),
            DeleteLabelsError::FailedDependency(ref cause) => write!(f, "{}", cause),
            DeleteLabelsError::ServiceUnavailable(ref cause) => write!(f, "{}", cause),
            DeleteLabelsError::UnauthorizedOperation(ref cause) => write!(f, "{}", cause),
            DeleteLabelsError::UnauthorizedResourceAccess(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeleteLabelsError {}
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
}

impl DeleteNotificationSubscriptionError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<DeleteNotificationSubscriptionError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "EntityNotExistsException" => {
                    return RusotoError::Service(
                        DeleteNotificationSubscriptionError::EntityNotExists(err.msg),
                    )
                }
                "ProhibitedStateException" => {
                    return RusotoError::Service(
                        DeleteNotificationSubscriptionError::ProhibitedState(err.msg),
                    )
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(
                        DeleteNotificationSubscriptionError::ServiceUnavailable(err.msg),
                    )
                }
                "UnauthorizedResourceAccessException" => {
                    return RusotoError::Service(
                        DeleteNotificationSubscriptionError::UnauthorizedResourceAccess(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for DeleteNotificationSubscriptionError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteNotificationSubscriptionError::EntityNotExists(ref cause) => {
                write!(f, "{}", cause)
            }
            DeleteNotificationSubscriptionError::ProhibitedState(ref cause) => {
                write!(f, "{}", cause)
            }
            DeleteNotificationSubscriptionError::ServiceUnavailable(ref cause) => {
                write!(f, "{}", cause)
            }
            DeleteNotificationSubscriptionError::UnauthorizedResourceAccess(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for DeleteNotificationSubscriptionError {}
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
}

impl DeleteUserError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteUserError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "EntityNotExistsException" => {
                    return RusotoError::Service(DeleteUserError::EntityNotExists(err.msg))
                }
                "FailedDependencyException" => {
                    return RusotoError::Service(DeleteUserError::FailedDependency(err.msg))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(DeleteUserError::ServiceUnavailable(err.msg))
                }
                "UnauthorizedOperationException" => {
                    return RusotoError::Service(DeleteUserError::UnauthorizedOperation(err.msg))
                }
                "UnauthorizedResourceAccessException" => {
                    return RusotoError::Service(DeleteUserError::UnauthorizedResourceAccess(
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
impl fmt::Display for DeleteUserError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteUserError::EntityNotExists(ref cause) => write!(f, "{}", cause),
            DeleteUserError::FailedDependency(ref cause) => write!(f, "{}", cause),
            DeleteUserError::ServiceUnavailable(ref cause) => write!(f, "{}", cause),
            DeleteUserError::UnauthorizedOperation(ref cause) => write!(f, "{}", cause),
            DeleteUserError::UnauthorizedResourceAccess(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeleteUserError {}
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
}

impl DescribeActivitiesError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeActivitiesError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "FailedDependencyException" => {
                    return RusotoError::Service(DescribeActivitiesError::FailedDependency(err.msg))
                }
                "InvalidArgumentException" => {
                    return RusotoError::Service(DescribeActivitiesError::InvalidArgument(err.msg))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(DescribeActivitiesError::ServiceUnavailable(
                        err.msg,
                    ))
                }
                "UnauthorizedOperationException" => {
                    return RusotoError::Service(DescribeActivitiesError::UnauthorizedOperation(
                        err.msg,
                    ))
                }
                "UnauthorizedResourceAccessException" => {
                    return RusotoError::Service(
                        DescribeActivitiesError::UnauthorizedResourceAccess(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for DescribeActivitiesError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeActivitiesError::FailedDependency(ref cause) => write!(f, "{}", cause),
            DescribeActivitiesError::InvalidArgument(ref cause) => write!(f, "{}", cause),
            DescribeActivitiesError::ServiceUnavailable(ref cause) => write!(f, "{}", cause),
            DescribeActivitiesError::UnauthorizedOperation(ref cause) => write!(f, "{}", cause),
            DescribeActivitiesError::UnauthorizedResourceAccess(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for DescribeActivitiesError {}
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
}

impl DescribeCommentsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeCommentsError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "EntityNotExistsException" => {
                    return RusotoError::Service(DescribeCommentsError::EntityNotExists(err.msg))
                }
                "FailedDependencyException" => {
                    return RusotoError::Service(DescribeCommentsError::FailedDependency(err.msg))
                }
                "ProhibitedStateException" => {
                    return RusotoError::Service(DescribeCommentsError::ProhibitedState(err.msg))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(DescribeCommentsError::ServiceUnavailable(err.msg))
                }
                "UnauthorizedOperationException" => {
                    return RusotoError::Service(DescribeCommentsError::UnauthorizedOperation(
                        err.msg,
                    ))
                }
                "UnauthorizedResourceAccessException" => {
                    return RusotoError::Service(DescribeCommentsError::UnauthorizedResourceAccess(
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
impl fmt::Display for DescribeCommentsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeCommentsError::EntityNotExists(ref cause) => write!(f, "{}", cause),
            DescribeCommentsError::FailedDependency(ref cause) => write!(f, "{}", cause),
            DescribeCommentsError::ProhibitedState(ref cause) => write!(f, "{}", cause),
            DescribeCommentsError::ServiceUnavailable(ref cause) => write!(f, "{}", cause),
            DescribeCommentsError::UnauthorizedOperation(ref cause) => write!(f, "{}", cause),
            DescribeCommentsError::UnauthorizedResourceAccess(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DescribeCommentsError {}
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
}

impl DescribeDocumentVersionsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeDocumentVersionsError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "EntityNotExistsException" => {
                    return RusotoError::Service(DescribeDocumentVersionsError::EntityNotExists(
                        err.msg,
                    ))
                }
                "FailedDependencyException" => {
                    return RusotoError::Service(DescribeDocumentVersionsError::FailedDependency(
                        err.msg,
                    ))
                }
                "InvalidArgumentException" => {
                    return RusotoError::Service(DescribeDocumentVersionsError::InvalidArgument(
                        err.msg,
                    ))
                }
                "ProhibitedStateException" => {
                    return RusotoError::Service(DescribeDocumentVersionsError::ProhibitedState(
                        err.msg,
                    ))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(DescribeDocumentVersionsError::ServiceUnavailable(
                        err.msg,
                    ))
                }
                "UnauthorizedOperationException" => {
                    return RusotoError::Service(
                        DescribeDocumentVersionsError::UnauthorizedOperation(err.msg),
                    )
                }
                "UnauthorizedResourceAccessException" => {
                    return RusotoError::Service(
                        DescribeDocumentVersionsError::UnauthorizedResourceAccess(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for DescribeDocumentVersionsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeDocumentVersionsError::EntityNotExists(ref cause) => write!(f, "{}", cause),
            DescribeDocumentVersionsError::FailedDependency(ref cause) => write!(f, "{}", cause),
            DescribeDocumentVersionsError::InvalidArgument(ref cause) => write!(f, "{}", cause),
            DescribeDocumentVersionsError::ProhibitedState(ref cause) => write!(f, "{}", cause),
            DescribeDocumentVersionsError::ServiceUnavailable(ref cause) => write!(f, "{}", cause),
            DescribeDocumentVersionsError::UnauthorizedOperation(ref cause) => {
                write!(f, "{}", cause)
            }
            DescribeDocumentVersionsError::UnauthorizedResourceAccess(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for DescribeDocumentVersionsError {}
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
}

impl DescribeFolderContentsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeFolderContentsError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "EntityNotExistsException" => {
                    return RusotoError::Service(DescribeFolderContentsError::EntityNotExists(
                        err.msg,
                    ))
                }
                "FailedDependencyException" => {
                    return RusotoError::Service(DescribeFolderContentsError::FailedDependency(
                        err.msg,
                    ))
                }
                "InvalidArgumentException" => {
                    return RusotoError::Service(DescribeFolderContentsError::InvalidArgument(
                        err.msg,
                    ))
                }
                "ProhibitedStateException" => {
                    return RusotoError::Service(DescribeFolderContentsError::ProhibitedState(
                        err.msg,
                    ))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(DescribeFolderContentsError::ServiceUnavailable(
                        err.msg,
                    ))
                }
                "UnauthorizedResourceAccessException" => {
                    return RusotoError::Service(
                        DescribeFolderContentsError::UnauthorizedResourceAccess(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for DescribeFolderContentsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeFolderContentsError::EntityNotExists(ref cause) => write!(f, "{}", cause),
            DescribeFolderContentsError::FailedDependency(ref cause) => write!(f, "{}", cause),
            DescribeFolderContentsError::InvalidArgument(ref cause) => write!(f, "{}", cause),
            DescribeFolderContentsError::ProhibitedState(ref cause) => write!(f, "{}", cause),
            DescribeFolderContentsError::ServiceUnavailable(ref cause) => write!(f, "{}", cause),
            DescribeFolderContentsError::UnauthorizedResourceAccess(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for DescribeFolderContentsError {}
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
}

impl DescribeGroupsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeGroupsError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "FailedDependencyException" => {
                    return RusotoError::Service(DescribeGroupsError::FailedDependency(err.msg))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(DescribeGroupsError::ServiceUnavailable(err.msg))
                }
                "UnauthorizedOperationException" => {
                    return RusotoError::Service(DescribeGroupsError::UnauthorizedOperation(
                        err.msg,
                    ))
                }
                "UnauthorizedResourceAccessException" => {
                    return RusotoError::Service(DescribeGroupsError::UnauthorizedResourceAccess(
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
impl fmt::Display for DescribeGroupsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeGroupsError::FailedDependency(ref cause) => write!(f, "{}", cause),
            DescribeGroupsError::ServiceUnavailable(ref cause) => write!(f, "{}", cause),
            DescribeGroupsError::UnauthorizedOperation(ref cause) => write!(f, "{}", cause),
            DescribeGroupsError::UnauthorizedResourceAccess(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DescribeGroupsError {}
/// Errors returned by DescribeNotificationSubscriptions
#[derive(Debug, PartialEq)]
pub enum DescribeNotificationSubscriptionsError {
    /// <p>The resource does not exist.</p>
    EntityNotExists(String),
    /// <p>One or more of the dependencies is unavailable.</p>
    ServiceUnavailable(String),
    /// <p>The caller does not have access to perform the action on the resource.</p>
    UnauthorizedResourceAccess(String),
}

impl DescribeNotificationSubscriptionsError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<DescribeNotificationSubscriptionsError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "EntityNotExistsException" => {
                    return RusotoError::Service(
                        DescribeNotificationSubscriptionsError::EntityNotExists(err.msg),
                    )
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(
                        DescribeNotificationSubscriptionsError::ServiceUnavailable(err.msg),
                    )
                }
                "UnauthorizedResourceAccessException" => {
                    return RusotoError::Service(
                        DescribeNotificationSubscriptionsError::UnauthorizedResourceAccess(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for DescribeNotificationSubscriptionsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeNotificationSubscriptionsError::EntityNotExists(ref cause) => {
                write!(f, "{}", cause)
            }
            DescribeNotificationSubscriptionsError::ServiceUnavailable(ref cause) => {
                write!(f, "{}", cause)
            }
            DescribeNotificationSubscriptionsError::UnauthorizedResourceAccess(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for DescribeNotificationSubscriptionsError {}
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
}

impl DescribeResourcePermissionsError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<DescribeResourcePermissionsError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "FailedDependencyException" => {
                    return RusotoError::Service(
                        DescribeResourcePermissionsError::FailedDependency(err.msg),
                    )
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(
                        DescribeResourcePermissionsError::ServiceUnavailable(err.msg),
                    )
                }
                "UnauthorizedOperationException" => {
                    return RusotoError::Service(
                        DescribeResourcePermissionsError::UnauthorizedOperation(err.msg),
                    )
                }
                "UnauthorizedResourceAccessException" => {
                    return RusotoError::Service(
                        DescribeResourcePermissionsError::UnauthorizedResourceAccess(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for DescribeResourcePermissionsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeResourcePermissionsError::FailedDependency(ref cause) => write!(f, "{}", cause),
            DescribeResourcePermissionsError::ServiceUnavailable(ref cause) => {
                write!(f, "{}", cause)
            }
            DescribeResourcePermissionsError::UnauthorizedOperation(ref cause) => {
                write!(f, "{}", cause)
            }
            DescribeResourcePermissionsError::UnauthorizedResourceAccess(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for DescribeResourcePermissionsError {}
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
}

impl DescribeRootFoldersError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeRootFoldersError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "FailedDependencyException" => {
                    return RusotoError::Service(DescribeRootFoldersError::FailedDependency(
                        err.msg,
                    ))
                }
                "InvalidArgumentException" => {
                    return RusotoError::Service(DescribeRootFoldersError::InvalidArgument(err.msg))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(DescribeRootFoldersError::ServiceUnavailable(
                        err.msg,
                    ))
                }
                "UnauthorizedOperationException" => {
                    return RusotoError::Service(DescribeRootFoldersError::UnauthorizedOperation(
                        err.msg,
                    ))
                }
                "UnauthorizedResourceAccessException" => {
                    return RusotoError::Service(
                        DescribeRootFoldersError::UnauthorizedResourceAccess(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for DescribeRootFoldersError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeRootFoldersError::FailedDependency(ref cause) => write!(f, "{}", cause),
            DescribeRootFoldersError::InvalidArgument(ref cause) => write!(f, "{}", cause),
            DescribeRootFoldersError::ServiceUnavailable(ref cause) => write!(f, "{}", cause),
            DescribeRootFoldersError::UnauthorizedOperation(ref cause) => write!(f, "{}", cause),
            DescribeRootFoldersError::UnauthorizedResourceAccess(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for DescribeRootFoldersError {}
/// Errors returned by DescribeUsers
#[derive(Debug, PartialEq)]
pub enum DescribeUsersError {
    /// <p>The resource does not exist.</p>
    EntityNotExists(String),
    /// <p>The AWS Directory Service cannot reach an on-premises instance. Or a dependency under the control of the organization is failing, such as a connected Active Directory.</p>
    FailedDependency(String),
    /// <p>The pagination marker or limit fields are not valid.</p>
    InvalidArgument(String),
    /// <p>The response is too large to return. The request must include a filter to reduce the size of the response.</p>
    RequestedEntityTooLarge(String),
    /// <p>One or more of the dependencies is unavailable.</p>
    ServiceUnavailable(String),
    /// <p>The operation is not permitted.</p>
    UnauthorizedOperation(String),
    /// <p>The caller does not have access to perform the action on the resource.</p>
    UnauthorizedResourceAccess(String),
}

impl DescribeUsersError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeUsersError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "EntityNotExistsException" => {
                    return RusotoError::Service(DescribeUsersError::EntityNotExists(err.msg))
                }
                "FailedDependencyException" => {
                    return RusotoError::Service(DescribeUsersError::FailedDependency(err.msg))
                }
                "InvalidArgumentException" => {
                    return RusotoError::Service(DescribeUsersError::InvalidArgument(err.msg))
                }
                "RequestedEntityTooLargeException" => {
                    return RusotoError::Service(DescribeUsersError::RequestedEntityTooLarge(
                        err.msg,
                    ))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(DescribeUsersError::ServiceUnavailable(err.msg))
                }
                "UnauthorizedOperationException" => {
                    return RusotoError::Service(DescribeUsersError::UnauthorizedOperation(err.msg))
                }
                "UnauthorizedResourceAccessException" => {
                    return RusotoError::Service(DescribeUsersError::UnauthorizedResourceAccess(
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
impl fmt::Display for DescribeUsersError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeUsersError::EntityNotExists(ref cause) => write!(f, "{}", cause),
            DescribeUsersError::FailedDependency(ref cause) => write!(f, "{}", cause),
            DescribeUsersError::InvalidArgument(ref cause) => write!(f, "{}", cause),
            DescribeUsersError::RequestedEntityTooLarge(ref cause) => write!(f, "{}", cause),
            DescribeUsersError::ServiceUnavailable(ref cause) => write!(f, "{}", cause),
            DescribeUsersError::UnauthorizedOperation(ref cause) => write!(f, "{}", cause),
            DescribeUsersError::UnauthorizedResourceAccess(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DescribeUsersError {}
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
}

impl GetCurrentUserError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetCurrentUserError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "EntityNotExistsException" => {
                    return RusotoError::Service(GetCurrentUserError::EntityNotExists(err.msg))
                }
                "FailedDependencyException" => {
                    return RusotoError::Service(GetCurrentUserError::FailedDependency(err.msg))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(GetCurrentUserError::ServiceUnavailable(err.msg))
                }
                "UnauthorizedOperationException" => {
                    return RusotoError::Service(GetCurrentUserError::UnauthorizedOperation(
                        err.msg,
                    ))
                }
                "UnauthorizedResourceAccessException" => {
                    return RusotoError::Service(GetCurrentUserError::UnauthorizedResourceAccess(
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
impl fmt::Display for GetCurrentUserError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetCurrentUserError::EntityNotExists(ref cause) => write!(f, "{}", cause),
            GetCurrentUserError::FailedDependency(ref cause) => write!(f, "{}", cause),
            GetCurrentUserError::ServiceUnavailable(ref cause) => write!(f, "{}", cause),
            GetCurrentUserError::UnauthorizedOperation(ref cause) => write!(f, "{}", cause),
            GetCurrentUserError::UnauthorizedResourceAccess(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetCurrentUserError {}
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
}

impl GetDocumentError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetDocumentError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "EntityNotExistsException" => {
                    return RusotoError::Service(GetDocumentError::EntityNotExists(err.msg))
                }
                "FailedDependencyException" => {
                    return RusotoError::Service(GetDocumentError::FailedDependency(err.msg))
                }
                "InvalidArgumentException" => {
                    return RusotoError::Service(GetDocumentError::InvalidArgument(err.msg))
                }
                "InvalidPasswordException" => {
                    return RusotoError::Service(GetDocumentError::InvalidPassword(err.msg))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(GetDocumentError::ServiceUnavailable(err.msg))
                }
                "UnauthorizedOperationException" => {
                    return RusotoError::Service(GetDocumentError::UnauthorizedOperation(err.msg))
                }
                "UnauthorizedResourceAccessException" => {
                    return RusotoError::Service(GetDocumentError::UnauthorizedResourceAccess(
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
impl fmt::Display for GetDocumentError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetDocumentError::EntityNotExists(ref cause) => write!(f, "{}", cause),
            GetDocumentError::FailedDependency(ref cause) => write!(f, "{}", cause),
            GetDocumentError::InvalidArgument(ref cause) => write!(f, "{}", cause),
            GetDocumentError::InvalidPassword(ref cause) => write!(f, "{}", cause),
            GetDocumentError::ServiceUnavailable(ref cause) => write!(f, "{}", cause),
            GetDocumentError::UnauthorizedOperation(ref cause) => write!(f, "{}", cause),
            GetDocumentError::UnauthorizedResourceAccess(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetDocumentError {}
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
}

impl GetDocumentPathError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetDocumentPathError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "EntityNotExistsException" => {
                    return RusotoError::Service(GetDocumentPathError::EntityNotExists(err.msg))
                }
                "FailedDependencyException" => {
                    return RusotoError::Service(GetDocumentPathError::FailedDependency(err.msg))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(GetDocumentPathError::ServiceUnavailable(err.msg))
                }
                "UnauthorizedOperationException" => {
                    return RusotoError::Service(GetDocumentPathError::UnauthorizedOperation(
                        err.msg,
                    ))
                }
                "UnauthorizedResourceAccessException" => {
                    return RusotoError::Service(GetDocumentPathError::UnauthorizedResourceAccess(
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
impl fmt::Display for GetDocumentPathError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetDocumentPathError::EntityNotExists(ref cause) => write!(f, "{}", cause),
            GetDocumentPathError::FailedDependency(ref cause) => write!(f, "{}", cause),
            GetDocumentPathError::ServiceUnavailable(ref cause) => write!(f, "{}", cause),
            GetDocumentPathError::UnauthorizedOperation(ref cause) => write!(f, "{}", cause),
            GetDocumentPathError::UnauthorizedResourceAccess(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetDocumentPathError {}
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
}

impl GetDocumentVersionError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetDocumentVersionError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "EntityNotExistsException" => {
                    return RusotoError::Service(GetDocumentVersionError::EntityNotExists(err.msg))
                }
                "FailedDependencyException" => {
                    return RusotoError::Service(GetDocumentVersionError::FailedDependency(err.msg))
                }
                "InvalidPasswordException" => {
                    return RusotoError::Service(GetDocumentVersionError::InvalidPassword(err.msg))
                }
                "ProhibitedStateException" => {
                    return RusotoError::Service(GetDocumentVersionError::ProhibitedState(err.msg))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(GetDocumentVersionError::ServiceUnavailable(
                        err.msg,
                    ))
                }
                "UnauthorizedOperationException" => {
                    return RusotoError::Service(GetDocumentVersionError::UnauthorizedOperation(
                        err.msg,
                    ))
                }
                "UnauthorizedResourceAccessException" => {
                    return RusotoError::Service(
                        GetDocumentVersionError::UnauthorizedResourceAccess(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for GetDocumentVersionError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetDocumentVersionError::EntityNotExists(ref cause) => write!(f, "{}", cause),
            GetDocumentVersionError::FailedDependency(ref cause) => write!(f, "{}", cause),
            GetDocumentVersionError::InvalidPassword(ref cause) => write!(f, "{}", cause),
            GetDocumentVersionError::ProhibitedState(ref cause) => write!(f, "{}", cause),
            GetDocumentVersionError::ServiceUnavailable(ref cause) => write!(f, "{}", cause),
            GetDocumentVersionError::UnauthorizedOperation(ref cause) => write!(f, "{}", cause),
            GetDocumentVersionError::UnauthorizedResourceAccess(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for GetDocumentVersionError {}
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
}

impl GetFolderError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetFolderError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "EntityNotExistsException" => {
                    return RusotoError::Service(GetFolderError::EntityNotExists(err.msg))
                }
                "FailedDependencyException" => {
                    return RusotoError::Service(GetFolderError::FailedDependency(err.msg))
                }
                "InvalidArgumentException" => {
                    return RusotoError::Service(GetFolderError::InvalidArgument(err.msg))
                }
                "ProhibitedStateException" => {
                    return RusotoError::Service(GetFolderError::ProhibitedState(err.msg))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(GetFolderError::ServiceUnavailable(err.msg))
                }
                "UnauthorizedOperationException" => {
                    return RusotoError::Service(GetFolderError::UnauthorizedOperation(err.msg))
                }
                "UnauthorizedResourceAccessException" => {
                    return RusotoError::Service(GetFolderError::UnauthorizedResourceAccess(
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
impl fmt::Display for GetFolderError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetFolderError::EntityNotExists(ref cause) => write!(f, "{}", cause),
            GetFolderError::FailedDependency(ref cause) => write!(f, "{}", cause),
            GetFolderError::InvalidArgument(ref cause) => write!(f, "{}", cause),
            GetFolderError::ProhibitedState(ref cause) => write!(f, "{}", cause),
            GetFolderError::ServiceUnavailable(ref cause) => write!(f, "{}", cause),
            GetFolderError::UnauthorizedOperation(ref cause) => write!(f, "{}", cause),
            GetFolderError::UnauthorizedResourceAccess(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetFolderError {}
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
}

impl GetFolderPathError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetFolderPathError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "EntityNotExistsException" => {
                    return RusotoError::Service(GetFolderPathError::EntityNotExists(err.msg))
                }
                "FailedDependencyException" => {
                    return RusotoError::Service(GetFolderPathError::FailedDependency(err.msg))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(GetFolderPathError::ServiceUnavailable(err.msg))
                }
                "UnauthorizedOperationException" => {
                    return RusotoError::Service(GetFolderPathError::UnauthorizedOperation(err.msg))
                }
                "UnauthorizedResourceAccessException" => {
                    return RusotoError::Service(GetFolderPathError::UnauthorizedResourceAccess(
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
impl fmt::Display for GetFolderPathError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetFolderPathError::EntityNotExists(ref cause) => write!(f, "{}", cause),
            GetFolderPathError::FailedDependency(ref cause) => write!(f, "{}", cause),
            GetFolderPathError::ServiceUnavailable(ref cause) => write!(f, "{}", cause),
            GetFolderPathError::UnauthorizedOperation(ref cause) => write!(f, "{}", cause),
            GetFolderPathError::UnauthorizedResourceAccess(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetFolderPathError {}
/// Errors returned by GetResources
#[derive(Debug, PartialEq)]
pub enum GetResourcesError {
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
}

impl GetResourcesError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetResourcesError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "FailedDependencyException" => {
                    return RusotoError::Service(GetResourcesError::FailedDependency(err.msg))
                }
                "InvalidArgumentException" => {
                    return RusotoError::Service(GetResourcesError::InvalidArgument(err.msg))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(GetResourcesError::ServiceUnavailable(err.msg))
                }
                "UnauthorizedOperationException" => {
                    return RusotoError::Service(GetResourcesError::UnauthorizedOperation(err.msg))
                }
                "UnauthorizedResourceAccessException" => {
                    return RusotoError::Service(GetResourcesError::UnauthorizedResourceAccess(
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
impl fmt::Display for GetResourcesError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetResourcesError::FailedDependency(ref cause) => write!(f, "{}", cause),
            GetResourcesError::InvalidArgument(ref cause) => write!(f, "{}", cause),
            GetResourcesError::ServiceUnavailable(ref cause) => write!(f, "{}", cause),
            GetResourcesError::UnauthorizedOperation(ref cause) => write!(f, "{}", cause),
            GetResourcesError::UnauthorizedResourceAccess(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetResourcesError {}
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
}

impl InitiateDocumentVersionUploadError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<InitiateDocumentVersionUploadError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "DraftUploadOutOfSyncException" => {
                    return RusotoError::Service(
                        InitiateDocumentVersionUploadError::DraftUploadOutOfSync(err.msg),
                    )
                }
                "EntityAlreadyExistsException" => {
                    return RusotoError::Service(
                        InitiateDocumentVersionUploadError::EntityAlreadyExists(err.msg),
                    )
                }
                "EntityNotExistsException" => {
                    return RusotoError::Service(
                        InitiateDocumentVersionUploadError::EntityNotExists(err.msg),
                    )
                }
                "FailedDependencyException" => {
                    return RusotoError::Service(
                        InitiateDocumentVersionUploadError::FailedDependency(err.msg),
                    )
                }
                "ProhibitedStateException" => {
                    return RusotoError::Service(
                        InitiateDocumentVersionUploadError::ProhibitedState(err.msg),
                    )
                }
                "ResourceAlreadyCheckedOutException" => {
                    return RusotoError::Service(
                        InitiateDocumentVersionUploadError::ResourceAlreadyCheckedOut(err.msg),
                    )
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(
                        InitiateDocumentVersionUploadError::ServiceUnavailable(err.msg),
                    )
                }
                "StorageLimitExceededException" => {
                    return RusotoError::Service(
                        InitiateDocumentVersionUploadError::StorageLimitExceeded(err.msg),
                    )
                }
                "StorageLimitWillExceedException" => {
                    return RusotoError::Service(
                        InitiateDocumentVersionUploadError::StorageLimitWillExceed(err.msg),
                    )
                }
                "UnauthorizedOperationException" => {
                    return RusotoError::Service(
                        InitiateDocumentVersionUploadError::UnauthorizedOperation(err.msg),
                    )
                }
                "UnauthorizedResourceAccessException" => {
                    return RusotoError::Service(
                        InitiateDocumentVersionUploadError::UnauthorizedResourceAccess(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for InitiateDocumentVersionUploadError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            InitiateDocumentVersionUploadError::DraftUploadOutOfSync(ref cause) => {
                write!(f, "{}", cause)
            }
            InitiateDocumentVersionUploadError::EntityAlreadyExists(ref cause) => {
                write!(f, "{}", cause)
            }
            InitiateDocumentVersionUploadError::EntityNotExists(ref cause) => {
                write!(f, "{}", cause)
            }
            InitiateDocumentVersionUploadError::FailedDependency(ref cause) => {
                write!(f, "{}", cause)
            }
            InitiateDocumentVersionUploadError::ProhibitedState(ref cause) => {
                write!(f, "{}", cause)
            }
            InitiateDocumentVersionUploadError::ResourceAlreadyCheckedOut(ref cause) => {
                write!(f, "{}", cause)
            }
            InitiateDocumentVersionUploadError::ServiceUnavailable(ref cause) => {
                write!(f, "{}", cause)
            }
            InitiateDocumentVersionUploadError::StorageLimitExceeded(ref cause) => {
                write!(f, "{}", cause)
            }
            InitiateDocumentVersionUploadError::StorageLimitWillExceed(ref cause) => {
                write!(f, "{}", cause)
            }
            InitiateDocumentVersionUploadError::UnauthorizedOperation(ref cause) => {
                write!(f, "{}", cause)
            }
            InitiateDocumentVersionUploadError::UnauthorizedResourceAccess(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for InitiateDocumentVersionUploadError {}
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
}

impl RemoveAllResourcePermissionsError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<RemoveAllResourcePermissionsError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "FailedDependencyException" => {
                    return RusotoError::Service(
                        RemoveAllResourcePermissionsError::FailedDependency(err.msg),
                    )
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(
                        RemoveAllResourcePermissionsError::ServiceUnavailable(err.msg),
                    )
                }
                "UnauthorizedOperationException" => {
                    return RusotoError::Service(
                        RemoveAllResourcePermissionsError::UnauthorizedOperation(err.msg),
                    )
                }
                "UnauthorizedResourceAccessException" => {
                    return RusotoError::Service(
                        RemoveAllResourcePermissionsError::UnauthorizedResourceAccess(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for RemoveAllResourcePermissionsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            RemoveAllResourcePermissionsError::FailedDependency(ref cause) => {
                write!(f, "{}", cause)
            }
            RemoveAllResourcePermissionsError::ServiceUnavailable(ref cause) => {
                write!(f, "{}", cause)
            }
            RemoveAllResourcePermissionsError::UnauthorizedOperation(ref cause) => {
                write!(f, "{}", cause)
            }
            RemoveAllResourcePermissionsError::UnauthorizedResourceAccess(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for RemoveAllResourcePermissionsError {}
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
}

impl RemoveResourcePermissionError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<RemoveResourcePermissionError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "FailedDependencyException" => {
                    return RusotoError::Service(RemoveResourcePermissionError::FailedDependency(
                        err.msg,
                    ))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(RemoveResourcePermissionError::ServiceUnavailable(
                        err.msg,
                    ))
                }
                "UnauthorizedOperationException" => {
                    return RusotoError::Service(
                        RemoveResourcePermissionError::UnauthorizedOperation(err.msg),
                    )
                }
                "UnauthorizedResourceAccessException" => {
                    return RusotoError::Service(
                        RemoveResourcePermissionError::UnauthorizedResourceAccess(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for RemoveResourcePermissionError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            RemoveResourcePermissionError::FailedDependency(ref cause) => write!(f, "{}", cause),
            RemoveResourcePermissionError::ServiceUnavailable(ref cause) => write!(f, "{}", cause),
            RemoveResourcePermissionError::UnauthorizedOperation(ref cause) => {
                write!(f, "{}", cause)
            }
            RemoveResourcePermissionError::UnauthorizedResourceAccess(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for RemoveResourcePermissionError {}
/// Errors returned by UpdateDocument
#[derive(Debug, PartialEq)]
pub enum UpdateDocumentError {
    /// <p>The resource hierarchy is changing.</p>
    ConcurrentModification(String),
    /// <p>Another operation is in progress on the resource that conflicts with the current operation.</p>
    ConflictingOperation(String),
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
}

impl UpdateDocumentError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdateDocumentError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "ConcurrentModificationException" => {
                    return RusotoError::Service(UpdateDocumentError::ConcurrentModification(
                        err.msg,
                    ))
                }
                "ConflictingOperationException" => {
                    return RusotoError::Service(UpdateDocumentError::ConflictingOperation(err.msg))
                }
                "EntityAlreadyExistsException" => {
                    return RusotoError::Service(UpdateDocumentError::EntityAlreadyExists(err.msg))
                }
                "EntityNotExistsException" => {
                    return RusotoError::Service(UpdateDocumentError::EntityNotExists(err.msg))
                }
                "FailedDependencyException" => {
                    return RusotoError::Service(UpdateDocumentError::FailedDependency(err.msg))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(UpdateDocumentError::LimitExceeded(err.msg))
                }
                "ProhibitedStateException" => {
                    return RusotoError::Service(UpdateDocumentError::ProhibitedState(err.msg))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(UpdateDocumentError::ServiceUnavailable(err.msg))
                }
                "UnauthorizedOperationException" => {
                    return RusotoError::Service(UpdateDocumentError::UnauthorizedOperation(
                        err.msg,
                    ))
                }
                "UnauthorizedResourceAccessException" => {
                    return RusotoError::Service(UpdateDocumentError::UnauthorizedResourceAccess(
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
impl fmt::Display for UpdateDocumentError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UpdateDocumentError::ConcurrentModification(ref cause) => write!(f, "{}", cause),
            UpdateDocumentError::ConflictingOperation(ref cause) => write!(f, "{}", cause),
            UpdateDocumentError::EntityAlreadyExists(ref cause) => write!(f, "{}", cause),
            UpdateDocumentError::EntityNotExists(ref cause) => write!(f, "{}", cause),
            UpdateDocumentError::FailedDependency(ref cause) => write!(f, "{}", cause),
            UpdateDocumentError::LimitExceeded(ref cause) => write!(f, "{}", cause),
            UpdateDocumentError::ProhibitedState(ref cause) => write!(f, "{}", cause),
            UpdateDocumentError::ServiceUnavailable(ref cause) => write!(f, "{}", cause),
            UpdateDocumentError::UnauthorizedOperation(ref cause) => write!(f, "{}", cause),
            UpdateDocumentError::UnauthorizedResourceAccess(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for UpdateDocumentError {}
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
}

impl UpdateDocumentVersionError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdateDocumentVersionError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "ConcurrentModificationException" => {
                    return RusotoError::Service(
                        UpdateDocumentVersionError::ConcurrentModification(err.msg),
                    )
                }
                "EntityNotExistsException" => {
                    return RusotoError::Service(UpdateDocumentVersionError::EntityNotExists(
                        err.msg,
                    ))
                }
                "FailedDependencyException" => {
                    return RusotoError::Service(UpdateDocumentVersionError::FailedDependency(
                        err.msg,
                    ))
                }
                "InvalidOperationException" => {
                    return RusotoError::Service(UpdateDocumentVersionError::InvalidOperation(
                        err.msg,
                    ))
                }
                "ProhibitedStateException" => {
                    return RusotoError::Service(UpdateDocumentVersionError::ProhibitedState(
                        err.msg,
                    ))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(UpdateDocumentVersionError::ServiceUnavailable(
                        err.msg,
                    ))
                }
                "UnauthorizedOperationException" => {
                    return RusotoError::Service(UpdateDocumentVersionError::UnauthorizedOperation(
                        err.msg,
                    ))
                }
                "UnauthorizedResourceAccessException" => {
                    return RusotoError::Service(
                        UpdateDocumentVersionError::UnauthorizedResourceAccess(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for UpdateDocumentVersionError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UpdateDocumentVersionError::ConcurrentModification(ref cause) => write!(f, "{}", cause),
            UpdateDocumentVersionError::EntityNotExists(ref cause) => write!(f, "{}", cause),
            UpdateDocumentVersionError::FailedDependency(ref cause) => write!(f, "{}", cause),
            UpdateDocumentVersionError::InvalidOperation(ref cause) => write!(f, "{}", cause),
            UpdateDocumentVersionError::ProhibitedState(ref cause) => write!(f, "{}", cause),
            UpdateDocumentVersionError::ServiceUnavailable(ref cause) => write!(f, "{}", cause),
            UpdateDocumentVersionError::UnauthorizedOperation(ref cause) => write!(f, "{}", cause),
            UpdateDocumentVersionError::UnauthorizedResourceAccess(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for UpdateDocumentVersionError {}
/// Errors returned by UpdateFolder
#[derive(Debug, PartialEq)]
pub enum UpdateFolderError {
    /// <p>The resource hierarchy is changing.</p>
    ConcurrentModification(String),
    /// <p>Another operation is in progress on the resource that conflicts with the current operation.</p>
    ConflictingOperation(String),
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
}

impl UpdateFolderError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdateFolderError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "ConcurrentModificationException" => {
                    return RusotoError::Service(UpdateFolderError::ConcurrentModification(err.msg))
                }
                "ConflictingOperationException" => {
                    return RusotoError::Service(UpdateFolderError::ConflictingOperation(err.msg))
                }
                "EntityAlreadyExistsException" => {
                    return RusotoError::Service(UpdateFolderError::EntityAlreadyExists(err.msg))
                }
                "EntityNotExistsException" => {
                    return RusotoError::Service(UpdateFolderError::EntityNotExists(err.msg))
                }
                "FailedDependencyException" => {
                    return RusotoError::Service(UpdateFolderError::FailedDependency(err.msg))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(UpdateFolderError::LimitExceeded(err.msg))
                }
                "ProhibitedStateException" => {
                    return RusotoError::Service(UpdateFolderError::ProhibitedState(err.msg))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(UpdateFolderError::ServiceUnavailable(err.msg))
                }
                "UnauthorizedOperationException" => {
                    return RusotoError::Service(UpdateFolderError::UnauthorizedOperation(err.msg))
                }
                "UnauthorizedResourceAccessException" => {
                    return RusotoError::Service(UpdateFolderError::UnauthorizedResourceAccess(
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
impl fmt::Display for UpdateFolderError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UpdateFolderError::ConcurrentModification(ref cause) => write!(f, "{}", cause),
            UpdateFolderError::ConflictingOperation(ref cause) => write!(f, "{}", cause),
            UpdateFolderError::EntityAlreadyExists(ref cause) => write!(f, "{}", cause),
            UpdateFolderError::EntityNotExists(ref cause) => write!(f, "{}", cause),
            UpdateFolderError::FailedDependency(ref cause) => write!(f, "{}", cause),
            UpdateFolderError::LimitExceeded(ref cause) => write!(f, "{}", cause),
            UpdateFolderError::ProhibitedState(ref cause) => write!(f, "{}", cause),
            UpdateFolderError::ServiceUnavailable(ref cause) => write!(f, "{}", cause),
            UpdateFolderError::UnauthorizedOperation(ref cause) => write!(f, "{}", cause),
            UpdateFolderError::UnauthorizedResourceAccess(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for UpdateFolderError {}
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
}

impl UpdateUserError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdateUserError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "DeactivatingLastSystemUserException" => {
                    return RusotoError::Service(UpdateUserError::DeactivatingLastSystemUser(
                        err.msg,
                    ))
                }
                "EntityNotExistsException" => {
                    return RusotoError::Service(UpdateUserError::EntityNotExists(err.msg))
                }
                "FailedDependencyException" => {
                    return RusotoError::Service(UpdateUserError::FailedDependency(err.msg))
                }
                "IllegalUserStateException" => {
                    return RusotoError::Service(UpdateUserError::IllegalUserState(err.msg))
                }
                "InvalidArgumentException" => {
                    return RusotoError::Service(UpdateUserError::InvalidArgument(err.msg))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(UpdateUserError::ServiceUnavailable(err.msg))
                }
                "UnauthorizedOperationException" => {
                    return RusotoError::Service(UpdateUserError::UnauthorizedOperation(err.msg))
                }
                "UnauthorizedResourceAccessException" => {
                    return RusotoError::Service(UpdateUserError::UnauthorizedResourceAccess(
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
impl fmt::Display for UpdateUserError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UpdateUserError::DeactivatingLastSystemUser(ref cause) => write!(f, "{}", cause),
            UpdateUserError::EntityNotExists(ref cause) => write!(f, "{}", cause),
            UpdateUserError::FailedDependency(ref cause) => write!(f, "{}", cause),
            UpdateUserError::IllegalUserState(ref cause) => write!(f, "{}", cause),
            UpdateUserError::InvalidArgument(ref cause) => write!(f, "{}", cause),
            UpdateUserError::ServiceUnavailable(ref cause) => write!(f, "{}", cause),
            UpdateUserError::UnauthorizedOperation(ref cause) => write!(f, "{}", cause),
            UpdateUserError::UnauthorizedResourceAccess(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for UpdateUserError {}
/// Trait representing the capabilities of the Amazon WorkDocs API. Amazon WorkDocs clients implement this trait.
#[async_trait]
pub trait Workdocs {
    /// <p>Aborts the upload of the specified document version that was previously initiated by <a>InitiateDocumentVersionUpload</a>. The client should make this call only when it no longer intends to upload the document version, or fails to do so.</p>
    async fn abort_document_version_upload(
        &self,
        input: AbortDocumentVersionUploadRequest,
    ) -> Result<(), RusotoError<AbortDocumentVersionUploadError>>;

    /// <p>Activates the specified user. Only active users can access Amazon WorkDocs.</p>
    async fn activate_user(
        &self,
        input: ActivateUserRequest,
    ) -> Result<ActivateUserResponse, RusotoError<ActivateUserError>>;

    /// <p>Creates a set of permissions for the specified folder or document. The resource permissions are overwritten if the principals already have different permissions.</p>
    async fn add_resource_permissions(
        &self,
        input: AddResourcePermissionsRequest,
    ) -> Result<AddResourcePermissionsResponse, RusotoError<AddResourcePermissionsError>>;

    /// <p>Adds a new comment to the specified document version.</p>
    async fn create_comment(
        &self,
        input: CreateCommentRequest,
    ) -> Result<CreateCommentResponse, RusotoError<CreateCommentError>>;

    /// <p>Adds one or more custom properties to the specified resource (a folder, document, or version).</p>
    async fn create_custom_metadata(
        &self,
        input: CreateCustomMetadataRequest,
    ) -> Result<CreateCustomMetadataResponse, RusotoError<CreateCustomMetadataError>>;

    /// <p>Creates a folder with the specified name and parent folder.</p>
    async fn create_folder(
        &self,
        input: CreateFolderRequest,
    ) -> Result<CreateFolderResponse, RusotoError<CreateFolderError>>;

    /// <p>Adds the specified list of labels to the given resource (a document or folder)</p>
    async fn create_labels(
        &self,
        input: CreateLabelsRequest,
    ) -> Result<CreateLabelsResponse, RusotoError<CreateLabelsError>>;

    /// <p>Configure Amazon WorkDocs to use Amazon SNS notifications. The endpoint receives a confirmation message, and must confirm the subscription.</p> <p>For more information, see <a href="https://docs.aws.amazon.com/workdocs/latest/developerguide/subscribe-notifications.html">Subscribe to Notifications</a> in the <i>Amazon WorkDocs Developer Guide</i>.</p>
    async fn create_notification_subscription(
        &self,
        input: CreateNotificationSubscriptionRequest,
    ) -> Result<
        CreateNotificationSubscriptionResponse,
        RusotoError<CreateNotificationSubscriptionError>,
    >;

    /// <p>Creates a user in a Simple AD or Microsoft AD directory. The status of a newly created user is "ACTIVE". New users can access Amazon WorkDocs.</p>
    async fn create_user(
        &self,
        input: CreateUserRequest,
    ) -> Result<CreateUserResponse, RusotoError<CreateUserError>>;

    /// <p>Deactivates the specified user, which revokes the user's access to Amazon WorkDocs.</p>
    async fn deactivate_user(
        &self,
        input: DeactivateUserRequest,
    ) -> Result<(), RusotoError<DeactivateUserError>>;

    /// <p>Deletes the specified comment from the document version.</p>
    async fn delete_comment(
        &self,
        input: DeleteCommentRequest,
    ) -> Result<(), RusotoError<DeleteCommentError>>;

    /// <p>Deletes custom metadata from the specified resource.</p>
    async fn delete_custom_metadata(
        &self,
        input: DeleteCustomMetadataRequest,
    ) -> Result<DeleteCustomMetadataResponse, RusotoError<DeleteCustomMetadataError>>;

    /// <p>Permanently deletes the specified document and its associated metadata.</p>
    async fn delete_document(
        &self,
        input: DeleteDocumentRequest,
    ) -> Result<(), RusotoError<DeleteDocumentError>>;

    /// <p>Permanently deletes the specified folder and its contents.</p>
    async fn delete_folder(
        &self,
        input: DeleteFolderRequest,
    ) -> Result<(), RusotoError<DeleteFolderError>>;

    /// <p>Deletes the contents of the specified folder.</p>
    async fn delete_folder_contents(
        &self,
        input: DeleteFolderContentsRequest,
    ) -> Result<(), RusotoError<DeleteFolderContentsError>>;

    /// <p>Deletes the specified list of labels from a resource.</p>
    async fn delete_labels(
        &self,
        input: DeleteLabelsRequest,
    ) -> Result<DeleteLabelsResponse, RusotoError<DeleteLabelsError>>;

    /// <p>Deletes the specified subscription from the specified organization.</p>
    async fn delete_notification_subscription(
        &self,
        input: DeleteNotificationSubscriptionRequest,
    ) -> Result<(), RusotoError<DeleteNotificationSubscriptionError>>;

    /// <p>Deletes the specified user from a Simple AD or Microsoft AD directory.</p>
    async fn delete_user(
        &self,
        input: DeleteUserRequest,
    ) -> Result<(), RusotoError<DeleteUserError>>;

    /// <p>Describes the user activities in a specified time period.</p>
    async fn describe_activities(
        &self,
        input: DescribeActivitiesRequest,
    ) -> Result<DescribeActivitiesResponse, RusotoError<DescribeActivitiesError>>;

    /// <p>List all the comments for the specified document version.</p>
    async fn describe_comments(
        &self,
        input: DescribeCommentsRequest,
    ) -> Result<DescribeCommentsResponse, RusotoError<DescribeCommentsError>>;

    /// <p>Retrieves the document versions for the specified document.</p> <p>By default, only active versions are returned.</p>
    async fn describe_document_versions(
        &self,
        input: DescribeDocumentVersionsRequest,
    ) -> Result<DescribeDocumentVersionsResponse, RusotoError<DescribeDocumentVersionsError>>;

    /// <p>Describes the contents of the specified folder, including its documents and subfolders.</p> <p>By default, Amazon WorkDocs returns the first 100 active document and folder metadata items. If there are more results, the response includes a marker that you can use to request the next set of results. You can also request initialized documents.</p>
    async fn describe_folder_contents(
        &self,
        input: DescribeFolderContentsRequest,
    ) -> Result<DescribeFolderContentsResponse, RusotoError<DescribeFolderContentsError>>;

    /// <p>Describes the groups specified by the query. Groups are defined by the underlying Active Directory.</p>
    async fn describe_groups(
        &self,
        input: DescribeGroupsRequest,
    ) -> Result<DescribeGroupsResponse, RusotoError<DescribeGroupsError>>;

    /// <p>Lists the specified notification subscriptions.</p>
    async fn describe_notification_subscriptions(
        &self,
        input: DescribeNotificationSubscriptionsRequest,
    ) -> Result<
        DescribeNotificationSubscriptionsResponse,
        RusotoError<DescribeNotificationSubscriptionsError>,
    >;

    /// <p>Describes the permissions of a specified resource.</p>
    async fn describe_resource_permissions(
        &self,
        input: DescribeResourcePermissionsRequest,
    ) -> Result<DescribeResourcePermissionsResponse, RusotoError<DescribeResourcePermissionsError>>;

    /// <p>Describes the current user's special folders; the <code>RootFolder</code> and the <code>RecycleBin</code>. <code>RootFolder</code> is the root of user's files and folders and <code>RecycleBin</code> is the root of recycled items. This is not a valid action for SigV4 (administrative API) clients.</p> <p>This action requires an authentication token. To get an authentication token, register an application with Amazon WorkDocs. For more information, see <a href="https://docs.aws.amazon.com/workdocs/latest/developerguide/wd-auth-user.html">Authentication and Access Control for User Applications</a> in the <i>Amazon WorkDocs Developer Guide</i>.</p>
    async fn describe_root_folders(
        &self,
        input: DescribeRootFoldersRequest,
    ) -> Result<DescribeRootFoldersResponse, RusotoError<DescribeRootFoldersError>>;

    /// <p>Describes the specified users. You can describe all users or filter the results (for example, by status or organization).</p> <p>By default, Amazon WorkDocs returns the first 24 active or pending users. If there are more results, the response includes a marker that you can use to request the next set of results.</p>
    async fn describe_users(
        &self,
        input: DescribeUsersRequest,
    ) -> Result<DescribeUsersResponse, RusotoError<DescribeUsersError>>;

    /// <p>Retrieves details of the current user for whom the authentication token was generated. This is not a valid action for SigV4 (administrative API) clients.</p>
    async fn get_current_user(
        &self,
        input: GetCurrentUserRequest,
    ) -> Result<GetCurrentUserResponse, RusotoError<GetCurrentUserError>>;

    /// <p>Retrieves details of a document.</p>
    async fn get_document(
        &self,
        input: GetDocumentRequest,
    ) -> Result<GetDocumentResponse, RusotoError<GetDocumentError>>;

    /// <p>Retrieves the path information (the hierarchy from the root folder) for the requested document.</p> <p>By default, Amazon WorkDocs returns a maximum of 100 levels upwards from the requested document and only includes the IDs of the parent folders in the path. You can limit the maximum number of levels. You can also request the names of the parent folders.</p>
    async fn get_document_path(
        &self,
        input: GetDocumentPathRequest,
    ) -> Result<GetDocumentPathResponse, RusotoError<GetDocumentPathError>>;

    /// <p>Retrieves version metadata for the specified document.</p>
    async fn get_document_version(
        &self,
        input: GetDocumentVersionRequest,
    ) -> Result<GetDocumentVersionResponse, RusotoError<GetDocumentVersionError>>;

    /// <p>Retrieves the metadata of the specified folder.</p>
    async fn get_folder(
        &self,
        input: GetFolderRequest,
    ) -> Result<GetFolderResponse, RusotoError<GetFolderError>>;

    /// <p>Retrieves the path information (the hierarchy from the root folder) for the specified folder.</p> <p>By default, Amazon WorkDocs returns a maximum of 100 levels upwards from the requested folder and only includes the IDs of the parent folders in the path. You can limit the maximum number of levels. You can also request the parent folder names.</p>
    async fn get_folder_path(
        &self,
        input: GetFolderPathRequest,
    ) -> Result<GetFolderPathResponse, RusotoError<GetFolderPathError>>;

    /// <p>Retrieves a collection of resources, including folders and documents. The only <code>CollectionType</code> supported is <code>SHARED_WITH_ME</code>.</p>
    async fn get_resources(
        &self,
        input: GetResourcesRequest,
    ) -> Result<GetResourcesResponse, RusotoError<GetResourcesError>>;

    /// <p>Creates a new document object and version object.</p> <p>The client specifies the parent folder ID and name of the document to upload. The ID is optionally specified when creating a new version of an existing document. This is the first step to upload a document. Next, upload the document to the URL returned from the call, and then call <a>UpdateDocumentVersion</a>.</p> <p>To cancel the document upload, call <a>AbortDocumentVersionUpload</a>.</p>
    async fn initiate_document_version_upload(
        &self,
        input: InitiateDocumentVersionUploadRequest,
    ) -> Result<
        InitiateDocumentVersionUploadResponse,
        RusotoError<InitiateDocumentVersionUploadError>,
    >;

    /// <p>Removes all the permissions from the specified resource.</p>
    async fn remove_all_resource_permissions(
        &self,
        input: RemoveAllResourcePermissionsRequest,
    ) -> Result<(), RusotoError<RemoveAllResourcePermissionsError>>;

    /// <p>Removes the permission for the specified principal from the specified resource.</p>
    async fn remove_resource_permission(
        &self,
        input: RemoveResourcePermissionRequest,
    ) -> Result<(), RusotoError<RemoveResourcePermissionError>>;

    /// <p>Updates the specified attributes of a document. The user must have access to both the document and its parent folder, if applicable.</p>
    async fn update_document(
        &self,
        input: UpdateDocumentRequest,
    ) -> Result<(), RusotoError<UpdateDocumentError>>;

    /// <p>Changes the status of the document version to ACTIVE. </p> <p>Amazon WorkDocs also sets its document container to ACTIVE. This is the last step in a document upload, after the client uploads the document to an S3-presigned URL returned by <a>InitiateDocumentVersionUpload</a>. </p>
    async fn update_document_version(
        &self,
        input: UpdateDocumentVersionRequest,
    ) -> Result<(), RusotoError<UpdateDocumentVersionError>>;

    /// <p>Updates the specified attributes of the specified folder. The user must have access to both the folder and its parent folder, if applicable.</p>
    async fn update_folder(
        &self,
        input: UpdateFolderRequest,
    ) -> Result<(), RusotoError<UpdateFolderError>>;

    /// <p>Updates the specified attributes of the specified user, and grants or revokes administrative privileges to the Amazon WorkDocs site.</p>
    async fn update_user(
        &self,
        input: UpdateUserRequest,
    ) -> Result<UpdateUserResponse, RusotoError<UpdateUserError>>;
}
/// A client for the Amazon WorkDocs API.
#[derive(Clone)]
pub struct WorkdocsClient {
    client: Client,
    region: region::Region,
}

impl WorkdocsClient {
    /// Creates a client backed by the default tokio event loop.
    ///
    /// The client will use the default credentials provider and tls client.
    pub fn new(region: region::Region) -> WorkdocsClient {
        WorkdocsClient {
            client: Client::shared(),
            region,
        }
    }

    pub fn new_with<P, D>(
        request_dispatcher: D,
        credentials_provider: P,
        region: region::Region,
    ) -> WorkdocsClient
    where
        P: ProvideAwsCredentials + Send + Sync + 'static,
        D: DispatchSignedRequest + Send + Sync + 'static,
    {
        WorkdocsClient {
            client: Client::new_with(credentials_provider, request_dispatcher),
            region,
        }
    }

    pub fn new_with_client(client: Client, region: region::Region) -> WorkdocsClient {
        WorkdocsClient { client, region }
    }
}

#[async_trait]
impl Workdocs for WorkdocsClient {
    /// <p>Aborts the upload of the specified document version that was previously initiated by <a>InitiateDocumentVersionUpload</a>. The client should make this call only when it no longer intends to upload the document version, or fails to do so.</p>
    async fn abort_document_version_upload(
        &self,
        input: AbortDocumentVersionUploadRequest,
    ) -> Result<(), RusotoError<AbortDocumentVersionUploadError>> {
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

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 204 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = ::std::mem::drop(response);

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(AbortDocumentVersionUploadError::from_response(response))
        }
    }

    /// <p>Activates the specified user. Only active users can access Amazon WorkDocs.</p>
    async fn activate_user(
        &self,
        input: ActivateUserRequest,
    ) -> Result<ActivateUserResponse, RusotoError<ActivateUserError>> {
        let request_uri = format!(
            "/api/v1/users/{user_id}/activation",
            user_id = input.user_id
        );

        let mut request = SignedRequest::new("POST", "workdocs", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        if let Some(ref authentication_token) = input.authentication_token {
            request.add_header("Authentication", &authentication_token.to_string());
        }

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<ActivateUserResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(ActivateUserError::from_response(response))
        }
    }

    /// <p>Creates a set of permissions for the specified folder or document. The resource permissions are overwritten if the principals already have different permissions.</p>
    async fn add_resource_permissions(
        &self,
        input: AddResourcePermissionsRequest,
    ) -> Result<AddResourcePermissionsResponse, RusotoError<AddResourcePermissionsError>> {
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

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 201 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<AddResourcePermissionsResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(AddResourcePermissionsError::from_response(response))
        }
    }

    /// <p>Adds a new comment to the specified document version.</p>
    async fn create_comment(
        &self,
        input: CreateCommentRequest,
    ) -> Result<CreateCommentResponse, RusotoError<CreateCommentError>> {
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

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 201 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<CreateCommentResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(CreateCommentError::from_response(response))
        }
    }

    /// <p>Adds one or more custom properties to the specified resource (a folder, document, or version).</p>
    async fn create_custom_metadata(
        &self,
        input: CreateCustomMetadataRequest,
    ) -> Result<CreateCustomMetadataResponse, RusotoError<CreateCustomMetadataError>> {
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

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<CreateCustomMetadataResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(CreateCustomMetadataError::from_response(response))
        }
    }

    /// <p>Creates a folder with the specified name and parent folder.</p>
    async fn create_folder(
        &self,
        input: CreateFolderRequest,
    ) -> Result<CreateFolderResponse, RusotoError<CreateFolderError>> {
        let request_uri = "/api/v1/folders";

        let mut request = SignedRequest::new("POST", "workdocs", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        if let Some(ref authentication_token) = input.authentication_token {
            request.add_header("Authentication", &authentication_token.to_string());
        }

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 201 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<CreateFolderResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(CreateFolderError::from_response(response))
        }
    }

    /// <p>Adds the specified list of labels to the given resource (a document or folder)</p>
    async fn create_labels(
        &self,
        input: CreateLabelsRequest,
    ) -> Result<CreateLabelsResponse, RusotoError<CreateLabelsError>> {
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

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<CreateLabelsResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(CreateLabelsError::from_response(response))
        }
    }

    /// <p>Configure Amazon WorkDocs to use Amazon SNS notifications. The endpoint receives a confirmation message, and must confirm the subscription.</p> <p>For more information, see <a href="https://docs.aws.amazon.com/workdocs/latest/developerguide/subscribe-notifications.html">Subscribe to Notifications</a> in the <i>Amazon WorkDocs Developer Guide</i>.</p>
    async fn create_notification_subscription(
        &self,
        input: CreateNotificationSubscriptionRequest,
    ) -> Result<
        CreateNotificationSubscriptionResponse,
        RusotoError<CreateNotificationSubscriptionError>,
    > {
        let request_uri = format!(
            "/api/v1/organizations/{organization_id}/subscriptions",
            organization_id = input.organization_id
        );

        let mut request = SignedRequest::new("POST", "workdocs", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<CreateNotificationSubscriptionResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(CreateNotificationSubscriptionError::from_response(response))
        }
    }

    /// <p>Creates a user in a Simple AD or Microsoft AD directory. The status of a newly created user is "ACTIVE". New users can access Amazon WorkDocs.</p>
    async fn create_user(
        &self,
        input: CreateUserRequest,
    ) -> Result<CreateUserResponse, RusotoError<CreateUserError>> {
        let request_uri = "/api/v1/users";

        let mut request = SignedRequest::new("POST", "workdocs", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        if let Some(ref authentication_token) = input.authentication_token {
            request.add_header("Authentication", &authentication_token.to_string());
        }

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 201 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<CreateUserResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(CreateUserError::from_response(response))
        }
    }

    /// <p>Deactivates the specified user, which revokes the user's access to Amazon WorkDocs.</p>
    async fn deactivate_user(
        &self,
        input: DeactivateUserRequest,
    ) -> Result<(), RusotoError<DeactivateUserError>> {
        let request_uri = format!(
            "/api/v1/users/{user_id}/activation",
            user_id = input.user_id
        );

        let mut request = SignedRequest::new("DELETE", "workdocs", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        if let Some(ref authentication_token) = input.authentication_token {
            request.add_header("Authentication", &authentication_token.to_string());
        }

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 204 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = ::std::mem::drop(response);

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(DeactivateUserError::from_response(response))
        }
    }

    /// <p>Deletes the specified comment from the document version.</p>
    async fn delete_comment(
        &self,
        input: DeleteCommentRequest,
    ) -> Result<(), RusotoError<DeleteCommentError>> {
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

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 204 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = ::std::mem::drop(response);

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(DeleteCommentError::from_response(response))
        }
    }

    /// <p>Deletes custom metadata from the specified resource.</p>
    async fn delete_custom_metadata(
        &self,
        input: DeleteCustomMetadataRequest,
    ) -> Result<DeleteCustomMetadataResponse, RusotoError<DeleteCustomMetadataError>> {
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

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<DeleteCustomMetadataResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(DeleteCustomMetadataError::from_response(response))
        }
    }

    /// <p>Permanently deletes the specified document and its associated metadata.</p>
    async fn delete_document(
        &self,
        input: DeleteDocumentRequest,
    ) -> Result<(), RusotoError<DeleteDocumentError>> {
        let request_uri = format!(
            "/api/v1/documents/{document_id}",
            document_id = input.document_id
        );

        let mut request = SignedRequest::new("DELETE", "workdocs", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        if let Some(ref authentication_token) = input.authentication_token {
            request.add_header("Authentication", &authentication_token.to_string());
        }

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 204 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = ::std::mem::drop(response);

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(DeleteDocumentError::from_response(response))
        }
    }

    /// <p>Permanently deletes the specified folder and its contents.</p>
    async fn delete_folder(
        &self,
        input: DeleteFolderRequest,
    ) -> Result<(), RusotoError<DeleteFolderError>> {
        let request_uri = format!("/api/v1/folders/{folder_id}", folder_id = input.folder_id);

        let mut request = SignedRequest::new("DELETE", "workdocs", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        if let Some(ref authentication_token) = input.authentication_token {
            request.add_header("Authentication", &authentication_token.to_string());
        }

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 204 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = ::std::mem::drop(response);

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(DeleteFolderError::from_response(response))
        }
    }

    /// <p>Deletes the contents of the specified folder.</p>
    async fn delete_folder_contents(
        &self,
        input: DeleteFolderContentsRequest,
    ) -> Result<(), RusotoError<DeleteFolderContentsError>> {
        let request_uri = format!(
            "/api/v1/folders/{folder_id}/contents",
            folder_id = input.folder_id
        );

        let mut request = SignedRequest::new("DELETE", "workdocs", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        if let Some(ref authentication_token) = input.authentication_token {
            request.add_header("Authentication", &authentication_token.to_string());
        }

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 204 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = ::std::mem::drop(response);

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(DeleteFolderContentsError::from_response(response))
        }
    }

    /// <p>Deletes the specified list of labels from a resource.</p>
    async fn delete_labels(
        &self,
        input: DeleteLabelsRequest,
    ) -> Result<DeleteLabelsResponse, RusotoError<DeleteLabelsError>> {
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

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<DeleteLabelsResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(DeleteLabelsError::from_response(response))
        }
    }

    /// <p>Deletes the specified subscription from the specified organization.</p>
    async fn delete_notification_subscription(
        &self,
        input: DeleteNotificationSubscriptionRequest,
    ) -> Result<(), RusotoError<DeleteNotificationSubscriptionError>> {
        let request_uri = format!(
            "/api/v1/organizations/{organization_id}/subscriptions/{subscription_id}",
            organization_id = input.organization_id,
            subscription_id = input.subscription_id
        );

        let mut request = SignedRequest::new("DELETE", "workdocs", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = ::std::mem::drop(response);

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(DeleteNotificationSubscriptionError::from_response(response))
        }
    }

    /// <p>Deletes the specified user from a Simple AD or Microsoft AD directory.</p>
    async fn delete_user(
        &self,
        input: DeleteUserRequest,
    ) -> Result<(), RusotoError<DeleteUserError>> {
        let request_uri = format!("/api/v1/users/{user_id}", user_id = input.user_id);

        let mut request = SignedRequest::new("DELETE", "workdocs", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        if let Some(ref authentication_token) = input.authentication_token {
            request.add_header("Authentication", &authentication_token.to_string());
        }

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 204 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = ::std::mem::drop(response);

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(DeleteUserError::from_response(response))
        }
    }

    /// <p>Describes the user activities in a specified time period.</p>
    async fn describe_activities(
        &self,
        input: DescribeActivitiesRequest,
    ) -> Result<DescribeActivitiesResponse, RusotoError<DescribeActivitiesError>> {
        let request_uri = "/api/v1/activities";

        let mut request = SignedRequest::new("GET", "workdocs", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        if let Some(ref authentication_token) = input.authentication_token {
            request.add_header("Authentication", &authentication_token.to_string());
        }
        let mut params = Params::new();
        if let Some(ref x) = input.activity_types {
            params.put("activityTypes", x);
        }
        if let Some(ref x) = input.end_time {
            params.put("endTime", x);
        }
        if let Some(ref x) = input.include_indirect_activities {
            params.put("includeIndirectActivities", x);
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
        if let Some(ref x) = input.resource_id {
            params.put("resourceId", x);
        }
        if let Some(ref x) = input.start_time {
            params.put("startTime", x);
        }
        if let Some(ref x) = input.user_id {
            params.put("userId", x);
        }
        request.set_params(params);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<DescribeActivitiesResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(DescribeActivitiesError::from_response(response))
        }
    }

    /// <p>List all the comments for the specified document version.</p>
    async fn describe_comments(
        &self,
        input: DescribeCommentsRequest,
    ) -> Result<DescribeCommentsResponse, RusotoError<DescribeCommentsError>> {
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

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<DescribeCommentsResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(DescribeCommentsError::from_response(response))
        }
    }

    /// <p>Retrieves the document versions for the specified document.</p> <p>By default, only active versions are returned.</p>
    async fn describe_document_versions(
        &self,
        input: DescribeDocumentVersionsRequest,
    ) -> Result<DescribeDocumentVersionsResponse, RusotoError<DescribeDocumentVersionsError>> {
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

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<DescribeDocumentVersionsResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(DescribeDocumentVersionsError::from_response(response))
        }
    }

    /// <p>Describes the contents of the specified folder, including its documents and subfolders.</p> <p>By default, Amazon WorkDocs returns the first 100 active document and folder metadata items. If there are more results, the response includes a marker that you can use to request the next set of results. You can also request initialized documents.</p>
    async fn describe_folder_contents(
        &self,
        input: DescribeFolderContentsRequest,
    ) -> Result<DescribeFolderContentsResponse, RusotoError<DescribeFolderContentsError>> {
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

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<DescribeFolderContentsResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(DescribeFolderContentsError::from_response(response))
        }
    }

    /// <p>Describes the groups specified by the query. Groups are defined by the underlying Active Directory.</p>
    async fn describe_groups(
        &self,
        input: DescribeGroupsRequest,
    ) -> Result<DescribeGroupsResponse, RusotoError<DescribeGroupsError>> {
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

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<DescribeGroupsResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(DescribeGroupsError::from_response(response))
        }
    }

    /// <p>Lists the specified notification subscriptions.</p>
    async fn describe_notification_subscriptions(
        &self,
        input: DescribeNotificationSubscriptionsRequest,
    ) -> Result<
        DescribeNotificationSubscriptionsResponse,
        RusotoError<DescribeNotificationSubscriptionsError>,
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

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<DescribeNotificationSubscriptionsResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(DescribeNotificationSubscriptionsError::from_response(
                response,
            ))
        }
    }

    /// <p>Describes the permissions of a specified resource.</p>
    async fn describe_resource_permissions(
        &self,
        input: DescribeResourcePermissionsRequest,
    ) -> Result<DescribeResourcePermissionsResponse, RusotoError<DescribeResourcePermissionsError>>
    {
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

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<DescribeResourcePermissionsResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(DescribeResourcePermissionsError::from_response(response))
        }
    }

    /// <p>Describes the current user's special folders; the <code>RootFolder</code> and the <code>RecycleBin</code>. <code>RootFolder</code> is the root of user's files and folders and <code>RecycleBin</code> is the root of recycled items. This is not a valid action for SigV4 (administrative API) clients.</p> <p>This action requires an authentication token. To get an authentication token, register an application with Amazon WorkDocs. For more information, see <a href="https://docs.aws.amazon.com/workdocs/latest/developerguide/wd-auth-user.html">Authentication and Access Control for User Applications</a> in the <i>Amazon WorkDocs Developer Guide</i>.</p>
    async fn describe_root_folders(
        &self,
        input: DescribeRootFoldersRequest,
    ) -> Result<DescribeRootFoldersResponse, RusotoError<DescribeRootFoldersError>> {
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

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<DescribeRootFoldersResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(DescribeRootFoldersError::from_response(response))
        }
    }

    /// <p>Describes the specified users. You can describe all users or filter the results (for example, by status or organization).</p> <p>By default, Amazon WorkDocs returns the first 24 active or pending users. If there are more results, the response includes a marker that you can use to request the next set of results.</p>
    async fn describe_users(
        &self,
        input: DescribeUsersRequest,
    ) -> Result<DescribeUsersResponse, RusotoError<DescribeUsersError>> {
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

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<DescribeUsersResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(DescribeUsersError::from_response(response))
        }
    }

    /// <p>Retrieves details of the current user for whom the authentication token was generated. This is not a valid action for SigV4 (administrative API) clients.</p>
    async fn get_current_user(
        &self,
        input: GetCurrentUserRequest,
    ) -> Result<GetCurrentUserResponse, RusotoError<GetCurrentUserError>> {
        let request_uri = "/api/v1/me";

        let mut request = SignedRequest::new("GET", "workdocs", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.add_header("Authentication", &input.authentication_token);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<GetCurrentUserResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(GetCurrentUserError::from_response(response))
        }
    }

    /// <p>Retrieves details of a document.</p>
    async fn get_document(
        &self,
        input: GetDocumentRequest,
    ) -> Result<GetDocumentResponse, RusotoError<GetDocumentError>> {
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

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<GetDocumentResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(GetDocumentError::from_response(response))
        }
    }

    /// <p>Retrieves the path information (the hierarchy from the root folder) for the requested document.</p> <p>By default, Amazon WorkDocs returns a maximum of 100 levels upwards from the requested document and only includes the IDs of the parent folders in the path. You can limit the maximum number of levels. You can also request the names of the parent folders.</p>
    async fn get_document_path(
        &self,
        input: GetDocumentPathRequest,
    ) -> Result<GetDocumentPathResponse, RusotoError<GetDocumentPathError>> {
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

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<GetDocumentPathResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(GetDocumentPathError::from_response(response))
        }
    }

    /// <p>Retrieves version metadata for the specified document.</p>
    async fn get_document_version(
        &self,
        input: GetDocumentVersionRequest,
    ) -> Result<GetDocumentVersionResponse, RusotoError<GetDocumentVersionError>> {
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

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<GetDocumentVersionResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(GetDocumentVersionError::from_response(response))
        }
    }

    /// <p>Retrieves the metadata of the specified folder.</p>
    async fn get_folder(
        &self,
        input: GetFolderRequest,
    ) -> Result<GetFolderResponse, RusotoError<GetFolderError>> {
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

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<GetFolderResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(GetFolderError::from_response(response))
        }
    }

    /// <p>Retrieves the path information (the hierarchy from the root folder) for the specified folder.</p> <p>By default, Amazon WorkDocs returns a maximum of 100 levels upwards from the requested folder and only includes the IDs of the parent folders in the path. You can limit the maximum number of levels. You can also request the parent folder names.</p>
    async fn get_folder_path(
        &self,
        input: GetFolderPathRequest,
    ) -> Result<GetFolderPathResponse, RusotoError<GetFolderPathError>> {
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

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<GetFolderPathResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(GetFolderPathError::from_response(response))
        }
    }

    /// <p>Retrieves a collection of resources, including folders and documents. The only <code>CollectionType</code> supported is <code>SHARED_WITH_ME</code>.</p>
    async fn get_resources(
        &self,
        input: GetResourcesRequest,
    ) -> Result<GetResourcesResponse, RusotoError<GetResourcesError>> {
        let request_uri = "/api/v1/resources";

        let mut request = SignedRequest::new("GET", "workdocs", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        if let Some(ref authentication_token) = input.authentication_token {
            request.add_header("Authentication", &authentication_token.to_string());
        }
        let mut params = Params::new();
        if let Some(ref x) = input.collection_type {
            params.put("collectionType", x);
        }
        if let Some(ref x) = input.limit {
            params.put("limit", x);
        }
        if let Some(ref x) = input.marker {
            params.put("marker", x);
        }
        if let Some(ref x) = input.user_id {
            params.put("userId", x);
        }
        request.set_params(params);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<GetResourcesResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(GetResourcesError::from_response(response))
        }
    }

    /// <p>Creates a new document object and version object.</p> <p>The client specifies the parent folder ID and name of the document to upload. The ID is optionally specified when creating a new version of an existing document. This is the first step to upload a document. Next, upload the document to the URL returned from the call, and then call <a>UpdateDocumentVersion</a>.</p> <p>To cancel the document upload, call <a>AbortDocumentVersionUpload</a>.</p>
    async fn initiate_document_version_upload(
        &self,
        input: InitiateDocumentVersionUploadRequest,
    ) -> Result<
        InitiateDocumentVersionUploadResponse,
        RusotoError<InitiateDocumentVersionUploadError>,
    > {
        let request_uri = "/api/v1/documents";

        let mut request = SignedRequest::new("POST", "workdocs", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        if let Some(ref authentication_token) = input.authentication_token {
            request.add_header("Authentication", &authentication_token.to_string());
        }

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 201 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<InitiateDocumentVersionUploadResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(InitiateDocumentVersionUploadError::from_response(response))
        }
    }

    /// <p>Removes all the permissions from the specified resource.</p>
    async fn remove_all_resource_permissions(
        &self,
        input: RemoveAllResourcePermissionsRequest,
    ) -> Result<(), RusotoError<RemoveAllResourcePermissionsError>> {
        let request_uri = format!(
            "/api/v1/resources/{resource_id}/permissions",
            resource_id = input.resource_id
        );

        let mut request = SignedRequest::new("DELETE", "workdocs", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        if let Some(ref authentication_token) = input.authentication_token {
            request.add_header("Authentication", &authentication_token.to_string());
        }

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 204 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = ::std::mem::drop(response);

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(RemoveAllResourcePermissionsError::from_response(response))
        }
    }

    /// <p>Removes the permission for the specified principal from the specified resource.</p>
    async fn remove_resource_permission(
        &self,
        input: RemoveResourcePermissionRequest,
    ) -> Result<(), RusotoError<RemoveResourcePermissionError>> {
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

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 204 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = ::std::mem::drop(response);

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(RemoveResourcePermissionError::from_response(response))
        }
    }

    /// <p>Updates the specified attributes of a document. The user must have access to both the document and its parent folder, if applicable.</p>
    async fn update_document(
        &self,
        input: UpdateDocumentRequest,
    ) -> Result<(), RusotoError<UpdateDocumentError>> {
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

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = ::std::mem::drop(response);

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(UpdateDocumentError::from_response(response))
        }
    }

    /// <p>Changes the status of the document version to ACTIVE. </p> <p>Amazon WorkDocs also sets its document container to ACTIVE. This is the last step in a document upload, after the client uploads the document to an S3-presigned URL returned by <a>InitiateDocumentVersionUpload</a>. </p>
    async fn update_document_version(
        &self,
        input: UpdateDocumentVersionRequest,
    ) -> Result<(), RusotoError<UpdateDocumentVersionError>> {
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

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = ::std::mem::drop(response);

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(UpdateDocumentVersionError::from_response(response))
        }
    }

    /// <p>Updates the specified attributes of the specified folder. The user must have access to both the folder and its parent folder, if applicable.</p>
    async fn update_folder(
        &self,
        input: UpdateFolderRequest,
    ) -> Result<(), RusotoError<UpdateFolderError>> {
        let request_uri = format!("/api/v1/folders/{folder_id}", folder_id = input.folder_id);

        let mut request = SignedRequest::new("PATCH", "workdocs", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        if let Some(ref authentication_token) = input.authentication_token {
            request.add_header("Authentication", &authentication_token.to_string());
        }

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = ::std::mem::drop(response);

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(UpdateFolderError::from_response(response))
        }
    }

    /// <p>Updates the specified attributes of the specified user, and grants or revokes administrative privileges to the Amazon WorkDocs site.</p>
    async fn update_user(
        &self,
        input: UpdateUserRequest,
    ) -> Result<UpdateUserResponse, RusotoError<UpdateUserError>> {
        let request_uri = format!("/api/v1/users/{user_id}", user_id = input.user_id);

        let mut request = SignedRequest::new("PATCH", "workdocs", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        if let Some(ref authentication_token) = input.authentication_token {
            request.add_header("Authentication", &authentication_token.to_string());
        }

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<UpdateUserResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(UpdateUserError::from_response(response))
        }
    }
}
