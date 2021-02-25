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
use serde_json;
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct AbortDocumentVersionUploadRequest {
    /// <p>Amazon WorkDocs authentication token. Not required when using AWS administrator credentials to access the API.</p>
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

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ActivateUserRequest {
    /// <p>Amazon WorkDocs authentication token. Not required when using AWS administrator credentials to access the API.</p>
    #[serde(rename = "AuthenticationToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authentication_token: Option<String>,
    /// <p>The ID of the user.</p>
    #[serde(rename = "UserId")]
    pub user_id: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ActivateUserResponse {
    /// <p>The user information.</p>
    #[serde(rename = "User")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<User>,
}

/// <p>Describes the activity information.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
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
    pub type_: Option<ActivityType>,
}

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct UnknownActivityType {
    name: String,
}

#[allow(non_camel_case_types)]
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
#[non_exhaustive]
pub enum ActivityType {
    DocumentAnnotationAdded,
    DocumentAnnotationDeleted,
    DocumentCheckedIn,
    DocumentCheckedOut,
    DocumentCommentAdded,
    DocumentCommentDeleted,
    DocumentMoved,
    DocumentRecycled,
    DocumentRenamed,
    DocumentRestored,
    DocumentReverted,
    DocumentShareableLinkCreated,
    DocumentShareableLinkPermissionChanged,
    DocumentShareableLinkRemoved,
    DocumentShared,
    DocumentSharePermissionChanged,
    DocumentUnshared,
    DocumentVersionDeleted,
    DocumentVersionDownloaded,
    DocumentVersionUploaded,
    DocumentVersionViewed,
    FolderCreated,
    FolderDeleted,
    FolderMoved,
    FolderRecycled,
    FolderRenamed,
    FolderRestored,
    FolderShareableLinkCreated,
    FolderShareableLinkPermissionChanged,
    FolderShareableLinkRemoved,
    FolderShared,
    FolderSharePermissionChanged,
    FolderUnshared,
    #[doc(hidden)]
    UnknownVariant(UnknownActivityType),
}

impl Default for ActivityType {
    fn default() -> Self {
        "".into()
    }
}

impl fmt::Display for ActivityType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.into())
    }
}

impl rusoto_core::param::ToParam for ActivityType {
    fn to_param(&self) -> String {
        self.to_string()
    }
}

impl Into<String> for ActivityType {
    fn into(self) -> String {
        match self {
            ActivityType::DocumentAnnotationAdded => "DOCUMENT_ANNOTATION_ADDED".to_string(),
            ActivityType::DocumentAnnotationDeleted => "DOCUMENT_ANNOTATION_DELETED".to_string(),
            ActivityType::DocumentCheckedIn => "DOCUMENT_CHECKED_IN".to_string(),
            ActivityType::DocumentCheckedOut => "DOCUMENT_CHECKED_OUT".to_string(),
            ActivityType::DocumentCommentAdded => "DOCUMENT_COMMENT_ADDED".to_string(),
            ActivityType::DocumentCommentDeleted => "DOCUMENT_COMMENT_DELETED".to_string(),
            ActivityType::DocumentMoved => "DOCUMENT_MOVED".to_string(),
            ActivityType::DocumentRecycled => "DOCUMENT_RECYCLED".to_string(),
            ActivityType::DocumentRenamed => "DOCUMENT_RENAMED".to_string(),
            ActivityType::DocumentRestored => "DOCUMENT_RESTORED".to_string(),
            ActivityType::DocumentReverted => "DOCUMENT_REVERTED".to_string(),
            ActivityType::DocumentShareableLinkCreated => {
                "DOCUMENT_SHAREABLE_LINK_CREATED".to_string()
            }
            ActivityType::DocumentShareableLinkPermissionChanged => {
                "DOCUMENT_SHAREABLE_LINK_PERMISSION_CHANGED".to_string()
            }
            ActivityType::DocumentShareableLinkRemoved => {
                "DOCUMENT_SHAREABLE_LINK_REMOVED".to_string()
            }
            ActivityType::DocumentShared => "DOCUMENT_SHARED".to_string(),
            ActivityType::DocumentSharePermissionChanged => {
                "DOCUMENT_SHARE_PERMISSION_CHANGED".to_string()
            }
            ActivityType::DocumentUnshared => "DOCUMENT_UNSHARED".to_string(),
            ActivityType::DocumentVersionDeleted => "DOCUMENT_VERSION_DELETED".to_string(),
            ActivityType::DocumentVersionDownloaded => "DOCUMENT_VERSION_DOWNLOADED".to_string(),
            ActivityType::DocumentVersionUploaded => "DOCUMENT_VERSION_UPLOADED".to_string(),
            ActivityType::DocumentVersionViewed => "DOCUMENT_VERSION_VIEWED".to_string(),
            ActivityType::FolderCreated => "FOLDER_CREATED".to_string(),
            ActivityType::FolderDeleted => "FOLDER_DELETED".to_string(),
            ActivityType::FolderMoved => "FOLDER_MOVED".to_string(),
            ActivityType::FolderRecycled => "FOLDER_RECYCLED".to_string(),
            ActivityType::FolderRenamed => "FOLDER_RENAMED".to_string(),
            ActivityType::FolderRestored => "FOLDER_RESTORED".to_string(),
            ActivityType::FolderShareableLinkCreated => "FOLDER_SHAREABLE_LINK_CREATED".to_string(),
            ActivityType::FolderShareableLinkPermissionChanged => {
                "FOLDER_SHAREABLE_LINK_PERMISSION_CHANGED".to_string()
            }
            ActivityType::FolderShareableLinkRemoved => "FOLDER_SHAREABLE_LINK_REMOVED".to_string(),
            ActivityType::FolderShared => "FOLDER_SHARED".to_string(),
            ActivityType::FolderSharePermissionChanged => {
                "FOLDER_SHARE_PERMISSION_CHANGED".to_string()
            }
            ActivityType::FolderUnshared => "FOLDER_UNSHARED".to_string(),
            ActivityType::UnknownVariant(UnknownActivityType { name: original }) => original,
        }
    }
}

impl<'a> Into<&'a str> for &'a ActivityType {
    fn into(self) -> &'a str {
        match self {
            ActivityType::DocumentAnnotationAdded => &"DOCUMENT_ANNOTATION_ADDED",
            ActivityType::DocumentAnnotationDeleted => &"DOCUMENT_ANNOTATION_DELETED",
            ActivityType::DocumentCheckedIn => &"DOCUMENT_CHECKED_IN",
            ActivityType::DocumentCheckedOut => &"DOCUMENT_CHECKED_OUT",
            ActivityType::DocumentCommentAdded => &"DOCUMENT_COMMENT_ADDED",
            ActivityType::DocumentCommentDeleted => &"DOCUMENT_COMMENT_DELETED",
            ActivityType::DocumentMoved => &"DOCUMENT_MOVED",
            ActivityType::DocumentRecycled => &"DOCUMENT_RECYCLED",
            ActivityType::DocumentRenamed => &"DOCUMENT_RENAMED",
            ActivityType::DocumentRestored => &"DOCUMENT_RESTORED",
            ActivityType::DocumentReverted => &"DOCUMENT_REVERTED",
            ActivityType::DocumentShareableLinkCreated => &"DOCUMENT_SHAREABLE_LINK_CREATED",
            ActivityType::DocumentShareableLinkPermissionChanged => {
                &"DOCUMENT_SHAREABLE_LINK_PERMISSION_CHANGED"
            }
            ActivityType::DocumentShareableLinkRemoved => &"DOCUMENT_SHAREABLE_LINK_REMOVED",
            ActivityType::DocumentShared => &"DOCUMENT_SHARED",
            ActivityType::DocumentSharePermissionChanged => &"DOCUMENT_SHARE_PERMISSION_CHANGED",
            ActivityType::DocumentUnshared => &"DOCUMENT_UNSHARED",
            ActivityType::DocumentVersionDeleted => &"DOCUMENT_VERSION_DELETED",
            ActivityType::DocumentVersionDownloaded => &"DOCUMENT_VERSION_DOWNLOADED",
            ActivityType::DocumentVersionUploaded => &"DOCUMENT_VERSION_UPLOADED",
            ActivityType::DocumentVersionViewed => &"DOCUMENT_VERSION_VIEWED",
            ActivityType::FolderCreated => &"FOLDER_CREATED",
            ActivityType::FolderDeleted => &"FOLDER_DELETED",
            ActivityType::FolderMoved => &"FOLDER_MOVED",
            ActivityType::FolderRecycled => &"FOLDER_RECYCLED",
            ActivityType::FolderRenamed => &"FOLDER_RENAMED",
            ActivityType::FolderRestored => &"FOLDER_RESTORED",
            ActivityType::FolderShareableLinkCreated => &"FOLDER_SHAREABLE_LINK_CREATED",
            ActivityType::FolderShareableLinkPermissionChanged => {
                &"FOLDER_SHAREABLE_LINK_PERMISSION_CHANGED"
            }
            ActivityType::FolderShareableLinkRemoved => &"FOLDER_SHAREABLE_LINK_REMOVED",
            ActivityType::FolderShared => &"FOLDER_SHARED",
            ActivityType::FolderSharePermissionChanged => &"FOLDER_SHARE_PERMISSION_CHANGED",
            ActivityType::FolderUnshared => &"FOLDER_UNSHARED",
            ActivityType::UnknownVariant(UnknownActivityType { name: original }) => original,
        }
    }
}

impl From<&str> for ActivityType {
    fn from(name: &str) -> Self {
        match name {
            "DOCUMENT_ANNOTATION_ADDED" => ActivityType::DocumentAnnotationAdded,
            "DOCUMENT_ANNOTATION_DELETED" => ActivityType::DocumentAnnotationDeleted,
            "DOCUMENT_CHECKED_IN" => ActivityType::DocumentCheckedIn,
            "DOCUMENT_CHECKED_OUT" => ActivityType::DocumentCheckedOut,
            "DOCUMENT_COMMENT_ADDED" => ActivityType::DocumentCommentAdded,
            "DOCUMENT_COMMENT_DELETED" => ActivityType::DocumentCommentDeleted,
            "DOCUMENT_MOVED" => ActivityType::DocumentMoved,
            "DOCUMENT_RECYCLED" => ActivityType::DocumentRecycled,
            "DOCUMENT_RENAMED" => ActivityType::DocumentRenamed,
            "DOCUMENT_RESTORED" => ActivityType::DocumentRestored,
            "DOCUMENT_REVERTED" => ActivityType::DocumentReverted,
            "DOCUMENT_SHAREABLE_LINK_CREATED" => ActivityType::DocumentShareableLinkCreated,
            "DOCUMENT_SHAREABLE_LINK_PERMISSION_CHANGED" => {
                ActivityType::DocumentShareableLinkPermissionChanged
            }
            "DOCUMENT_SHAREABLE_LINK_REMOVED" => ActivityType::DocumentShareableLinkRemoved,
            "DOCUMENT_SHARED" => ActivityType::DocumentShared,
            "DOCUMENT_SHARE_PERMISSION_CHANGED" => ActivityType::DocumentSharePermissionChanged,
            "DOCUMENT_UNSHARED" => ActivityType::DocumentUnshared,
            "DOCUMENT_VERSION_DELETED" => ActivityType::DocumentVersionDeleted,
            "DOCUMENT_VERSION_DOWNLOADED" => ActivityType::DocumentVersionDownloaded,
            "DOCUMENT_VERSION_UPLOADED" => ActivityType::DocumentVersionUploaded,
            "DOCUMENT_VERSION_VIEWED" => ActivityType::DocumentVersionViewed,
            "FOLDER_CREATED" => ActivityType::FolderCreated,
            "FOLDER_DELETED" => ActivityType::FolderDeleted,
            "FOLDER_MOVED" => ActivityType::FolderMoved,
            "FOLDER_RECYCLED" => ActivityType::FolderRecycled,
            "FOLDER_RENAMED" => ActivityType::FolderRenamed,
            "FOLDER_RESTORED" => ActivityType::FolderRestored,
            "FOLDER_SHAREABLE_LINK_CREATED" => ActivityType::FolderShareableLinkCreated,
            "FOLDER_SHAREABLE_LINK_PERMISSION_CHANGED" => {
                ActivityType::FolderShareableLinkPermissionChanged
            }
            "FOLDER_SHAREABLE_LINK_REMOVED" => ActivityType::FolderShareableLinkRemoved,
            "FOLDER_SHARED" => ActivityType::FolderShared,
            "FOLDER_SHARE_PERMISSION_CHANGED" => ActivityType::FolderSharePermissionChanged,
            "FOLDER_UNSHARED" => ActivityType::FolderUnshared,
            _ => ActivityType::UnknownVariant(UnknownActivityType {
                name: name.to_owned(),
            }),
        }
    }
}

impl From<String> for ActivityType {
    fn from(name: String) -> Self {
        match &*name {
            "DOCUMENT_ANNOTATION_ADDED" => ActivityType::DocumentAnnotationAdded,
            "DOCUMENT_ANNOTATION_DELETED" => ActivityType::DocumentAnnotationDeleted,
            "DOCUMENT_CHECKED_IN" => ActivityType::DocumentCheckedIn,
            "DOCUMENT_CHECKED_OUT" => ActivityType::DocumentCheckedOut,
            "DOCUMENT_COMMENT_ADDED" => ActivityType::DocumentCommentAdded,
            "DOCUMENT_COMMENT_DELETED" => ActivityType::DocumentCommentDeleted,
            "DOCUMENT_MOVED" => ActivityType::DocumentMoved,
            "DOCUMENT_RECYCLED" => ActivityType::DocumentRecycled,
            "DOCUMENT_RENAMED" => ActivityType::DocumentRenamed,
            "DOCUMENT_RESTORED" => ActivityType::DocumentRestored,
            "DOCUMENT_REVERTED" => ActivityType::DocumentReverted,
            "DOCUMENT_SHAREABLE_LINK_CREATED" => ActivityType::DocumentShareableLinkCreated,
            "DOCUMENT_SHAREABLE_LINK_PERMISSION_CHANGED" => {
                ActivityType::DocumentShareableLinkPermissionChanged
            }
            "DOCUMENT_SHAREABLE_LINK_REMOVED" => ActivityType::DocumentShareableLinkRemoved,
            "DOCUMENT_SHARED" => ActivityType::DocumentShared,
            "DOCUMENT_SHARE_PERMISSION_CHANGED" => ActivityType::DocumentSharePermissionChanged,
            "DOCUMENT_UNSHARED" => ActivityType::DocumentUnshared,
            "DOCUMENT_VERSION_DELETED" => ActivityType::DocumentVersionDeleted,
            "DOCUMENT_VERSION_DOWNLOADED" => ActivityType::DocumentVersionDownloaded,
            "DOCUMENT_VERSION_UPLOADED" => ActivityType::DocumentVersionUploaded,
            "DOCUMENT_VERSION_VIEWED" => ActivityType::DocumentVersionViewed,
            "FOLDER_CREATED" => ActivityType::FolderCreated,
            "FOLDER_DELETED" => ActivityType::FolderDeleted,
            "FOLDER_MOVED" => ActivityType::FolderMoved,
            "FOLDER_RECYCLED" => ActivityType::FolderRecycled,
            "FOLDER_RENAMED" => ActivityType::FolderRenamed,
            "FOLDER_RESTORED" => ActivityType::FolderRestored,
            "FOLDER_SHAREABLE_LINK_CREATED" => ActivityType::FolderShareableLinkCreated,
            "FOLDER_SHAREABLE_LINK_PERMISSION_CHANGED" => {
                ActivityType::FolderShareableLinkPermissionChanged
            }
            "FOLDER_SHAREABLE_LINK_REMOVED" => ActivityType::FolderShareableLinkRemoved,
            "FOLDER_SHARED" => ActivityType::FolderShared,
            "FOLDER_SHARE_PERMISSION_CHANGED" => ActivityType::FolderSharePermissionChanged,
            "FOLDER_UNSHARED" => ActivityType::FolderUnshared,
            _ => ActivityType::UnknownVariant(UnknownActivityType { name }),
        }
    }
}

impl ::std::str::FromStr for ActivityType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(s.into())
    }
}

#[cfg(any(test, feature = "serialize_structs"))]
impl Serialize for ActivityType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.into())
    }
}

impl<'de> Deserialize<'de> for ActivityType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Ok(String::deserialize(deserializer)?.into())
    }
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct AddResourcePermissionsRequest {
    /// <p>Amazon WorkDocs authentication token. Not required when using AWS administrator credentials to access the API.</p>
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

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct AddResourcePermissionsResponse {
    /// <p>The share results.</p>
    #[serde(rename = "ShareResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub share_results: Option<Vec<ShareResult>>,
}

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct UnknownBooleanEnumType {
    name: String,
}

#[allow(non_camel_case_types)]
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
#[non_exhaustive]
pub enum BooleanEnumType {
    False,
    True,
    #[doc(hidden)]
    UnknownVariant(UnknownBooleanEnumType),
}

impl Default for BooleanEnumType {
    fn default() -> Self {
        "".into()
    }
}

impl fmt::Display for BooleanEnumType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.into())
    }
}

impl rusoto_core::param::ToParam for BooleanEnumType {
    fn to_param(&self) -> String {
        self.to_string()
    }
}

impl Into<String> for BooleanEnumType {
    fn into(self) -> String {
        match self {
            BooleanEnumType::False => "FALSE".to_string(),
            BooleanEnumType::True => "TRUE".to_string(),
            BooleanEnumType::UnknownVariant(UnknownBooleanEnumType { name: original }) => original,
        }
    }
}

impl<'a> Into<&'a str> for &'a BooleanEnumType {
    fn into(self) -> &'a str {
        match self {
            BooleanEnumType::False => &"FALSE",
            BooleanEnumType::True => &"TRUE",
            BooleanEnumType::UnknownVariant(UnknownBooleanEnumType { name: original }) => original,
        }
    }
}

impl From<&str> for BooleanEnumType {
    fn from(name: &str) -> Self {
        match name {
            "FALSE" => BooleanEnumType::False,
            "TRUE" => BooleanEnumType::True,
            _ => BooleanEnumType::UnknownVariant(UnknownBooleanEnumType {
                name: name.to_owned(),
            }),
        }
    }
}

impl From<String> for BooleanEnumType {
    fn from(name: String) -> Self {
        match &*name {
            "FALSE" => BooleanEnumType::False,
            "TRUE" => BooleanEnumType::True,
            _ => BooleanEnumType::UnknownVariant(UnknownBooleanEnumType { name }),
        }
    }
}

impl ::std::str::FromStr for BooleanEnumType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(s.into())
    }
}

impl Serialize for BooleanEnumType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.into())
    }
}

#[cfg(feature = "deserialize_structs")]
impl<'de> Deserialize<'de> for BooleanEnumType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Ok(String::deserialize(deserializer)?.into())
    }
}

/// <p>Describes a comment.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
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
    pub status: Option<CommentStatusType>,
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
    pub visibility: Option<CommentVisibilityType>,
}

/// <p>Describes the metadata of a comment.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CommentMetadata {
    /// <p>The ID of the comment.</p>
    #[serde(rename = "CommentId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment_id: Option<String>,
    /// <p>The status of the comment.</p>
    #[serde(rename = "CommentStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment_status: Option<CommentStatusType>,
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

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct UnknownCommentStatusType {
    name: String,
}

#[allow(non_camel_case_types)]
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
#[non_exhaustive]
pub enum CommentStatusType {
    Deleted,
    Draft,
    Published,
    #[doc(hidden)]
    UnknownVariant(UnknownCommentStatusType),
}

impl Default for CommentStatusType {
    fn default() -> Self {
        "".into()
    }
}

impl fmt::Display for CommentStatusType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.into())
    }
}

impl rusoto_core::param::ToParam for CommentStatusType {
    fn to_param(&self) -> String {
        self.to_string()
    }
}

impl Into<String> for CommentStatusType {
    fn into(self) -> String {
        match self {
            CommentStatusType::Deleted => "DELETED".to_string(),
            CommentStatusType::Draft => "DRAFT".to_string(),
            CommentStatusType::Published => "PUBLISHED".to_string(),
            CommentStatusType::UnknownVariant(UnknownCommentStatusType { name: original }) => {
                original
            }
        }
    }
}

impl<'a> Into<&'a str> for &'a CommentStatusType {
    fn into(self) -> &'a str {
        match self {
            CommentStatusType::Deleted => &"DELETED",
            CommentStatusType::Draft => &"DRAFT",
            CommentStatusType::Published => &"PUBLISHED",
            CommentStatusType::UnknownVariant(UnknownCommentStatusType { name: original }) => {
                original
            }
        }
    }
}

impl From<&str> for CommentStatusType {
    fn from(name: &str) -> Self {
        match name {
            "DELETED" => CommentStatusType::Deleted,
            "DRAFT" => CommentStatusType::Draft,
            "PUBLISHED" => CommentStatusType::Published,
            _ => CommentStatusType::UnknownVariant(UnknownCommentStatusType {
                name: name.to_owned(),
            }),
        }
    }
}

impl From<String> for CommentStatusType {
    fn from(name: String) -> Self {
        match &*name {
            "DELETED" => CommentStatusType::Deleted,
            "DRAFT" => CommentStatusType::Draft,
            "PUBLISHED" => CommentStatusType::Published,
            _ => CommentStatusType::UnknownVariant(UnknownCommentStatusType { name }),
        }
    }
}

impl ::std::str::FromStr for CommentStatusType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(s.into())
    }
}

#[cfg(any(test, feature = "serialize_structs"))]
impl Serialize for CommentStatusType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.into())
    }
}

impl<'de> Deserialize<'de> for CommentStatusType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Ok(String::deserialize(deserializer)?.into())
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct UnknownCommentVisibilityType {
    name: String,
}

#[allow(non_camel_case_types)]
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
#[non_exhaustive]
pub enum CommentVisibilityType {
    Private,
    Public,
    #[doc(hidden)]
    UnknownVariant(UnknownCommentVisibilityType),
}

impl Default for CommentVisibilityType {
    fn default() -> Self {
        "".into()
    }
}

impl fmt::Display for CommentVisibilityType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.into())
    }
}

impl rusoto_core::param::ToParam for CommentVisibilityType {
    fn to_param(&self) -> String {
        self.to_string()
    }
}

impl Into<String> for CommentVisibilityType {
    fn into(self) -> String {
        match self {
            CommentVisibilityType::Private => "PRIVATE".to_string(),
            CommentVisibilityType::Public => "PUBLIC".to_string(),
            CommentVisibilityType::UnknownVariant(UnknownCommentVisibilityType {
                name: original,
            }) => original,
        }
    }
}

impl<'a> Into<&'a str> for &'a CommentVisibilityType {
    fn into(self) -> &'a str {
        match self {
            CommentVisibilityType::Private => &"PRIVATE",
            CommentVisibilityType::Public => &"PUBLIC",
            CommentVisibilityType::UnknownVariant(UnknownCommentVisibilityType {
                name: original,
            }) => original,
        }
    }
}

impl From<&str> for CommentVisibilityType {
    fn from(name: &str) -> Self {
        match name {
            "PRIVATE" => CommentVisibilityType::Private,
            "PUBLIC" => CommentVisibilityType::Public,
            _ => CommentVisibilityType::UnknownVariant(UnknownCommentVisibilityType {
                name: name.to_owned(),
            }),
        }
    }
}

impl From<String> for CommentVisibilityType {
    fn from(name: String) -> Self {
        match &*name {
            "PRIVATE" => CommentVisibilityType::Private,
            "PUBLIC" => CommentVisibilityType::Public,
            _ => CommentVisibilityType::UnknownVariant(UnknownCommentVisibilityType { name }),
        }
    }
}

impl ::std::str::FromStr for CommentVisibilityType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(s.into())
    }
}

impl Serialize for CommentVisibilityType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.into())
    }
}

impl<'de> Deserialize<'de> for CommentVisibilityType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Ok(String::deserialize(deserializer)?.into())
    }
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateCommentRequest {
    /// <p>Amazon WorkDocs authentication token. Not required when using AWS administrator credentials to access the API.</p>
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
    pub visibility: Option<CommentVisibilityType>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateCommentResponse {
    /// <p>The comment that has been created.</p>
    #[serde(rename = "Comment")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment: Option<Comment>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateCustomMetadataRequest {
    /// <p>Amazon WorkDocs authentication token. Not required when using AWS administrator credentials to access the API.</p>
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

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateCustomMetadataResponse {}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateFolderRequest {
    /// <p>Amazon WorkDocs authentication token. Not required when using AWS administrator credentials to access the API.</p>
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

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateFolderResponse {
    /// <p>The metadata of the folder.</p>
    #[serde(rename = "Metadata")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<FolderMetadata>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateLabelsRequest {
    /// <p>Amazon WorkDocs authentication token. Not required when using AWS administrator credentials to access the API.</p>
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

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateLabelsResponse {}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
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
    pub protocol: SubscriptionProtocolType,
    /// <p>The notification type.</p>
    #[serde(rename = "SubscriptionType")]
    pub subscription_type: SubscriptionType,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateNotificationSubscriptionResponse {
    /// <p>The subscription.</p>
    #[serde(rename = "Subscription")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscription: Option<Subscription>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateUserRequest {
    /// <p>Amazon WorkDocs authentication token. Not required when using AWS administrator credentials to access the API.</p>
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

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateUserResponse {
    /// <p>The user information.</p>
    #[serde(rename = "User")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<User>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeactivateUserRequest {
    /// <p>Amazon WorkDocs authentication token. Not required when using AWS administrator credentials to access the API.</p>
    #[serde(rename = "AuthenticationToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authentication_token: Option<String>,
    /// <p>The ID of the user.</p>
    #[serde(rename = "UserId")]
    pub user_id: String,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteCommentRequest {
    /// <p>Amazon WorkDocs authentication token. Not required when using AWS administrator credentials to access the API.</p>
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

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteCustomMetadataRequest {
    /// <p>Amazon WorkDocs authentication token. Not required when using AWS administrator credentials to access the API.</p>
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

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DeleteCustomMetadataResponse {}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteDocumentRequest {
    /// <p>Amazon WorkDocs authentication token. Not required when using AWS administrator credentials to access the API.</p>
    #[serde(rename = "AuthenticationToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authentication_token: Option<String>,
    /// <p>The ID of the document.</p>
    #[serde(rename = "DocumentId")]
    pub document_id: String,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteFolderContentsRequest {
    /// <p>Amazon WorkDocs authentication token. Not required when using AWS administrator credentials to access the API.</p>
    #[serde(rename = "AuthenticationToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authentication_token: Option<String>,
    /// <p>The ID of the folder.</p>
    #[serde(rename = "FolderId")]
    pub folder_id: String,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteFolderRequest {
    /// <p>Amazon WorkDocs authentication token. Not required when using AWS administrator credentials to access the API.</p>
    #[serde(rename = "AuthenticationToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authentication_token: Option<String>,
    /// <p>The ID of the folder.</p>
    #[serde(rename = "FolderId")]
    pub folder_id: String,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteLabelsRequest {
    /// <p>Amazon WorkDocs authentication token. Not required when using AWS administrator credentials to access the API.</p>
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

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DeleteLabelsResponse {}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteNotificationSubscriptionRequest {
    /// <p>The ID of the organization.</p>
    #[serde(rename = "OrganizationId")]
    pub organization_id: String,
    /// <p>The ID of the subscription.</p>
    #[serde(rename = "SubscriptionId")]
    pub subscription_id: String,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
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

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeActivitiesRequest {
    /// <p>Specifies which activity types to include in the response. If this field is left empty, all activity types are returned.</p>
    #[serde(rename = "ActivityTypes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub activity_types: Option<String>,
    /// <p>Amazon WorkDocs authentication token. Not required when using AWS administrator credentials to access the API.</p>
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

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
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

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeCommentsRequest {
    /// <p>Amazon WorkDocs authentication token. Not required when using AWS administrator credentials to access the API.</p>
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

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
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

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeDocumentVersionsRequest {
    /// <p>Amazon WorkDocs authentication token. Not required when using AWS administrator credentials to access the API.</p>
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

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
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

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeFolderContentsRequest {
    /// <p>Amazon WorkDocs authentication token. Not required when using AWS administrator credentials to access the API.</p>
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
    pub order: Option<OrderType>,
    /// <p>The sorting criteria.</p>
    #[serde(rename = "Sort")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort: Option<ResourceSortType>,
    /// <p>The type of items.</p>
    #[serde(rename = "Type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<FolderContentType>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
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

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeGroupsRequest {
    /// <p>Amazon WorkDocs authentication token. Not required when using AWS administrator credentials to access the API.</p>
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

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
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

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
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

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
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

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeResourcePermissionsRequest {
    /// <p>Amazon WorkDocs authentication token. Not required when using AWS administrator credentials to access the API.</p>
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

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
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

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeRootFoldersRequest {
    /// <p>Amazon WorkDocs authentication token.</p>
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

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
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

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeUsersRequest {
    /// <p>Amazon WorkDocs authentication token. Not required when using AWS administrator credentials to access the API.</p>
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
    pub include: Option<UserFilterType>,
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
    pub order: Option<OrderType>,
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
    pub sort: Option<UserSortType>,
    /// <p>The IDs of the users.</p>
    #[serde(rename = "UserIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_ids: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
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
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
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
    pub resource_state: Option<ResourceStateType>,
}

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct UnknownDocumentSourceType {
    name: String,
}

#[allow(non_camel_case_types)]
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
#[non_exhaustive]
pub enum DocumentSourceType {
    Original,
    WithComments,
    #[doc(hidden)]
    UnknownVariant(UnknownDocumentSourceType),
}

impl Default for DocumentSourceType {
    fn default() -> Self {
        "".into()
    }
}

impl fmt::Display for DocumentSourceType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.into())
    }
}

impl rusoto_core::param::ToParam for DocumentSourceType {
    fn to_param(&self) -> String {
        self.to_string()
    }
}

impl Into<String> for DocumentSourceType {
    fn into(self) -> String {
        match self {
            DocumentSourceType::Original => "ORIGINAL".to_string(),
            DocumentSourceType::WithComments => "WITH_COMMENTS".to_string(),
            DocumentSourceType::UnknownVariant(UnknownDocumentSourceType { name: original }) => {
                original
            }
        }
    }
}

impl<'a> Into<&'a str> for &'a DocumentSourceType {
    fn into(self) -> &'a str {
        match self {
            DocumentSourceType::Original => &"ORIGINAL",
            DocumentSourceType::WithComments => &"WITH_COMMENTS",
            DocumentSourceType::UnknownVariant(UnknownDocumentSourceType { name: original }) => {
                original
            }
        }
    }
}

impl From<&str> for DocumentSourceType {
    fn from(name: &str) -> Self {
        match name {
            "ORIGINAL" => DocumentSourceType::Original,
            "WITH_COMMENTS" => DocumentSourceType::WithComments,
            _ => DocumentSourceType::UnknownVariant(UnknownDocumentSourceType {
                name: name.to_owned(),
            }),
        }
    }
}

impl From<String> for DocumentSourceType {
    fn from(name: String) -> Self {
        match &*name {
            "ORIGINAL" => DocumentSourceType::Original,
            "WITH_COMMENTS" => DocumentSourceType::WithComments,
            _ => DocumentSourceType::UnknownVariant(UnknownDocumentSourceType { name }),
        }
    }
}

impl ::std::str::FromStr for DocumentSourceType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(s.into())
    }
}

#[cfg(any(test, feature = "serialize_structs"))]
impl Serialize for DocumentSourceType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.into())
    }
}

impl<'de> Deserialize<'de> for DocumentSourceType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Ok(String::deserialize(deserializer)?.into())
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct UnknownDocumentStatusType {
    name: String,
}

#[allow(non_camel_case_types)]
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
#[non_exhaustive]
pub enum DocumentStatusType {
    Active,
    Initialized,
    #[doc(hidden)]
    UnknownVariant(UnknownDocumentStatusType),
}

impl Default for DocumentStatusType {
    fn default() -> Self {
        "".into()
    }
}

impl fmt::Display for DocumentStatusType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.into())
    }
}

impl rusoto_core::param::ToParam for DocumentStatusType {
    fn to_param(&self) -> String {
        self.to_string()
    }
}

impl Into<String> for DocumentStatusType {
    fn into(self) -> String {
        match self {
            DocumentStatusType::Active => "ACTIVE".to_string(),
            DocumentStatusType::Initialized => "INITIALIZED".to_string(),
            DocumentStatusType::UnknownVariant(UnknownDocumentStatusType { name: original }) => {
                original
            }
        }
    }
}

impl<'a> Into<&'a str> for &'a DocumentStatusType {
    fn into(self) -> &'a str {
        match self {
            DocumentStatusType::Active => &"ACTIVE",
            DocumentStatusType::Initialized => &"INITIALIZED",
            DocumentStatusType::UnknownVariant(UnknownDocumentStatusType { name: original }) => {
                original
            }
        }
    }
}

impl From<&str> for DocumentStatusType {
    fn from(name: &str) -> Self {
        match name {
            "ACTIVE" => DocumentStatusType::Active,
            "INITIALIZED" => DocumentStatusType::Initialized,
            _ => DocumentStatusType::UnknownVariant(UnknownDocumentStatusType {
                name: name.to_owned(),
            }),
        }
    }
}

impl From<String> for DocumentStatusType {
    fn from(name: String) -> Self {
        match &*name {
            "ACTIVE" => DocumentStatusType::Active,
            "INITIALIZED" => DocumentStatusType::Initialized,
            _ => DocumentStatusType::UnknownVariant(UnknownDocumentStatusType { name }),
        }
    }
}

impl ::std::str::FromStr for DocumentStatusType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(s.into())
    }
}

#[cfg(any(test, feature = "serialize_structs"))]
impl Serialize for DocumentStatusType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.into())
    }
}

impl<'de> Deserialize<'de> for DocumentStatusType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Ok(String::deserialize(deserializer)?.into())
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct UnknownDocumentThumbnailType {
    name: String,
}

#[allow(non_camel_case_types)]
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
#[non_exhaustive]
pub enum DocumentThumbnailType {
    Large,
    Small,
    SmallHq,
    #[doc(hidden)]
    UnknownVariant(UnknownDocumentThumbnailType),
}

impl Default for DocumentThumbnailType {
    fn default() -> Self {
        "".into()
    }
}

impl fmt::Display for DocumentThumbnailType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.into())
    }
}

impl rusoto_core::param::ToParam for DocumentThumbnailType {
    fn to_param(&self) -> String {
        self.to_string()
    }
}

impl Into<String> for DocumentThumbnailType {
    fn into(self) -> String {
        match self {
            DocumentThumbnailType::Large => "LARGE".to_string(),
            DocumentThumbnailType::Small => "SMALL".to_string(),
            DocumentThumbnailType::SmallHq => "SMALL_HQ".to_string(),
            DocumentThumbnailType::UnknownVariant(UnknownDocumentThumbnailType {
                name: original,
            }) => original,
        }
    }
}

impl<'a> Into<&'a str> for &'a DocumentThumbnailType {
    fn into(self) -> &'a str {
        match self {
            DocumentThumbnailType::Large => &"LARGE",
            DocumentThumbnailType::Small => &"SMALL",
            DocumentThumbnailType::SmallHq => &"SMALL_HQ",
            DocumentThumbnailType::UnknownVariant(UnknownDocumentThumbnailType {
                name: original,
            }) => original,
        }
    }
}

impl From<&str> for DocumentThumbnailType {
    fn from(name: &str) -> Self {
        match name {
            "LARGE" => DocumentThumbnailType::Large,
            "SMALL" => DocumentThumbnailType::Small,
            "SMALL_HQ" => DocumentThumbnailType::SmallHq,
            _ => DocumentThumbnailType::UnknownVariant(UnknownDocumentThumbnailType {
                name: name.to_owned(),
            }),
        }
    }
}

impl From<String> for DocumentThumbnailType {
    fn from(name: String) -> Self {
        match &*name {
            "LARGE" => DocumentThumbnailType::Large,
            "SMALL" => DocumentThumbnailType::Small,
            "SMALL_HQ" => DocumentThumbnailType::SmallHq,
            _ => DocumentThumbnailType::UnknownVariant(UnknownDocumentThumbnailType { name }),
        }
    }
}

impl ::std::str::FromStr for DocumentThumbnailType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(s.into())
    }
}

#[cfg(any(test, feature = "serialize_structs"))]
impl Serialize for DocumentThumbnailType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.into())
    }
}

impl<'de> Deserialize<'de> for DocumentThumbnailType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Ok(String::deserialize(deserializer)?.into())
    }
}

/// <p>Describes a version of a document.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
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
    pub source: Option<::std::collections::HashMap<DocumentSourceType, String>>,
    /// <p>The status of the document.</p>
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<DocumentStatusType>,
    /// <p>The thumbnail of the document.</p>
    #[serde(rename = "Thumbnail")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumbnail: Option<::std::collections::HashMap<DocumentThumbnailType, String>>,
}

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct UnknownDocumentVersionStatus {
    name: String,
}

#[allow(non_camel_case_types)]
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
#[non_exhaustive]
pub enum DocumentVersionStatus {
    Active,
    #[doc(hidden)]
    UnknownVariant(UnknownDocumentVersionStatus),
}

impl Default for DocumentVersionStatus {
    fn default() -> Self {
        "".into()
    }
}

impl fmt::Display for DocumentVersionStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.into())
    }
}

impl rusoto_core::param::ToParam for DocumentVersionStatus {
    fn to_param(&self) -> String {
        self.to_string()
    }
}

impl Into<String> for DocumentVersionStatus {
    fn into(self) -> String {
        match self {
            DocumentVersionStatus::Active => "ACTIVE".to_string(),
            DocumentVersionStatus::UnknownVariant(UnknownDocumentVersionStatus {
                name: original,
            }) => original,
        }
    }
}

impl<'a> Into<&'a str> for &'a DocumentVersionStatus {
    fn into(self) -> &'a str {
        match self {
            DocumentVersionStatus::Active => &"ACTIVE",
            DocumentVersionStatus::UnknownVariant(UnknownDocumentVersionStatus {
                name: original,
            }) => original,
        }
    }
}

impl From<&str> for DocumentVersionStatus {
    fn from(name: &str) -> Self {
        match name {
            "ACTIVE" => DocumentVersionStatus::Active,
            _ => DocumentVersionStatus::UnknownVariant(UnknownDocumentVersionStatus {
                name: name.to_owned(),
            }),
        }
    }
}

impl From<String> for DocumentVersionStatus {
    fn from(name: String) -> Self {
        match &*name {
            "ACTIVE" => DocumentVersionStatus::Active,
            _ => DocumentVersionStatus::UnknownVariant(UnknownDocumentVersionStatus { name }),
        }
    }
}

impl ::std::str::FromStr for DocumentVersionStatus {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(s.into())
    }
}

impl Serialize for DocumentVersionStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.into())
    }
}

#[cfg(feature = "deserialize_structs")]
impl<'de> Deserialize<'de> for DocumentVersionStatus {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Ok(String::deserialize(deserializer)?.into())
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct UnknownFolderContentType {
    name: String,
}

#[allow(non_camel_case_types)]
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
#[non_exhaustive]
pub enum FolderContentType {
    All,
    Document,
    Folder,
    #[doc(hidden)]
    UnknownVariant(UnknownFolderContentType),
}

impl Default for FolderContentType {
    fn default() -> Self {
        "".into()
    }
}

impl fmt::Display for FolderContentType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.into())
    }
}

impl rusoto_core::param::ToParam for FolderContentType {
    fn to_param(&self) -> String {
        self.to_string()
    }
}

impl Into<String> for FolderContentType {
    fn into(self) -> String {
        match self {
            FolderContentType::All => "ALL".to_string(),
            FolderContentType::Document => "DOCUMENT".to_string(),
            FolderContentType::Folder => "FOLDER".to_string(),
            FolderContentType::UnknownVariant(UnknownFolderContentType { name: original }) => {
                original
            }
        }
    }
}

impl<'a> Into<&'a str> for &'a FolderContentType {
    fn into(self) -> &'a str {
        match self {
            FolderContentType::All => &"ALL",
            FolderContentType::Document => &"DOCUMENT",
            FolderContentType::Folder => &"FOLDER",
            FolderContentType::UnknownVariant(UnknownFolderContentType { name: original }) => {
                original
            }
        }
    }
}

impl From<&str> for FolderContentType {
    fn from(name: &str) -> Self {
        match name {
            "ALL" => FolderContentType::All,
            "DOCUMENT" => FolderContentType::Document,
            "FOLDER" => FolderContentType::Folder,
            _ => FolderContentType::UnknownVariant(UnknownFolderContentType {
                name: name.to_owned(),
            }),
        }
    }
}

impl From<String> for FolderContentType {
    fn from(name: String) -> Self {
        match &*name {
            "ALL" => FolderContentType::All,
            "DOCUMENT" => FolderContentType::Document,
            "FOLDER" => FolderContentType::Folder,
            _ => FolderContentType::UnknownVariant(UnknownFolderContentType { name }),
        }
    }
}

impl ::std::str::FromStr for FolderContentType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(s.into())
    }
}

impl Serialize for FolderContentType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.into())
    }
}

#[cfg(feature = "deserialize_structs")]
impl<'de> Deserialize<'de> for FolderContentType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Ok(String::deserialize(deserializer)?.into())
    }
}

/// <p>Describes a folder.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
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
    pub resource_state: Option<ResourceStateType>,
    /// <p>The unique identifier created from the subfolders and documents of the folder.</p>
    #[serde(rename = "Signature")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signature: Option<String>,
    /// <p>The size of the folder metadata.</p>
    #[serde(rename = "Size")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<i64>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetCurrentUserRequest {
    /// <p>Amazon WorkDocs authentication token.</p>
    #[serde(rename = "AuthenticationToken")]
    pub authentication_token: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetCurrentUserResponse {
    /// <p>Metadata of the user.</p>
    #[serde(rename = "User")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<User>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetDocumentPathRequest {
    /// <p>Amazon WorkDocs authentication token. Not required when using AWS administrator credentials to access the API.</p>
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

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetDocumentPathResponse {
    /// <p>The path information.</p>
    #[serde(rename = "Path")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<ResourcePath>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetDocumentRequest {
    /// <p>Amazon WorkDocs authentication token. Not required when using AWS administrator credentials to access the API.</p>
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

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
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

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetDocumentVersionRequest {
    /// <p>Amazon WorkDocs authentication token. Not required when using AWS administrator credentials to access the API.</p>
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

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
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

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetFolderPathRequest {
    /// <p>Amazon WorkDocs authentication token. Not required when using AWS administrator credentials to access the API.</p>
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

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetFolderPathResponse {
    /// <p>The path information.</p>
    #[serde(rename = "Path")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<ResourcePath>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetFolderRequest {
    /// <p>Amazon WorkDocs authentication token. Not required when using AWS administrator credentials to access the API.</p>
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

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
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

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetResourcesRequest {
    /// <p>The Amazon WorkDocs authentication token. Not required when using AWS administrator credentials to access the API.</p>
    #[serde(rename = "AuthenticationToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authentication_token: Option<String>,
    /// <p>The collection type.</p>
    #[serde(rename = "CollectionType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub collection_type: Option<ResourceCollectionType>,
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

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
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
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
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

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct InitiateDocumentVersionUploadRequest {
    /// <p>Amazon WorkDocs authentication token. Not required when using AWS administrator credentials to access the API.</p>
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

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
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

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct UnknownLocaleType {
    name: String,
}

#[allow(non_camel_case_types)]
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
#[non_exhaustive]
pub enum LocaleType {
    De,
    Default,
    En,
    Es,
    Fr,
    Ja,
    Ko,
    PtBR,
    Ru,
    ZhCN,
    ZhTW,
    #[doc(hidden)]
    UnknownVariant(UnknownLocaleType),
}

impl Default for LocaleType {
    fn default() -> Self {
        "".into()
    }
}

impl fmt::Display for LocaleType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.into())
    }
}

impl rusoto_core::param::ToParam for LocaleType {
    fn to_param(&self) -> String {
        self.to_string()
    }
}

impl Into<String> for LocaleType {
    fn into(self) -> String {
        match self {
            LocaleType::De => "de".to_string(),
            LocaleType::Default => "default".to_string(),
            LocaleType::En => "en".to_string(),
            LocaleType::Es => "es".to_string(),
            LocaleType::Fr => "fr".to_string(),
            LocaleType::Ja => "ja".to_string(),
            LocaleType::Ko => "ko".to_string(),
            LocaleType::PtBR => "pt_BR".to_string(),
            LocaleType::Ru => "ru".to_string(),
            LocaleType::ZhCN => "zh_CN".to_string(),
            LocaleType::ZhTW => "zh_TW".to_string(),
            LocaleType::UnknownVariant(UnknownLocaleType { name: original }) => original,
        }
    }
}

impl<'a> Into<&'a str> for &'a LocaleType {
    fn into(self) -> &'a str {
        match self {
            LocaleType::De => &"de",
            LocaleType::Default => &"default",
            LocaleType::En => &"en",
            LocaleType::Es => &"es",
            LocaleType::Fr => &"fr",
            LocaleType::Ja => &"ja",
            LocaleType::Ko => &"ko",
            LocaleType::PtBR => &"pt_BR",
            LocaleType::Ru => &"ru",
            LocaleType::ZhCN => &"zh_CN",
            LocaleType::ZhTW => &"zh_TW",
            LocaleType::UnknownVariant(UnknownLocaleType { name: original }) => original,
        }
    }
}

impl From<&str> for LocaleType {
    fn from(name: &str) -> Self {
        match name {
            "de" => LocaleType::De,
            "default" => LocaleType::Default,
            "en" => LocaleType::En,
            "es" => LocaleType::Es,
            "fr" => LocaleType::Fr,
            "ja" => LocaleType::Ja,
            "ko" => LocaleType::Ko,
            "pt_BR" => LocaleType::PtBR,
            "ru" => LocaleType::Ru,
            "zh_CN" => LocaleType::ZhCN,
            "zh_TW" => LocaleType::ZhTW,
            _ => LocaleType::UnknownVariant(UnknownLocaleType {
                name: name.to_owned(),
            }),
        }
    }
}

impl From<String> for LocaleType {
    fn from(name: String) -> Self {
        match &*name {
            "de" => LocaleType::De,
            "default" => LocaleType::Default,
            "en" => LocaleType::En,
            "es" => LocaleType::Es,
            "fr" => LocaleType::Fr,
            "ja" => LocaleType::Ja,
            "ko" => LocaleType::Ko,
            "pt_BR" => LocaleType::PtBR,
            "ru" => LocaleType::Ru,
            "zh_CN" => LocaleType::ZhCN,
            "zh_TW" => LocaleType::ZhTW,
            _ => LocaleType::UnknownVariant(UnknownLocaleType { name }),
        }
    }
}

impl ::std::str::FromStr for LocaleType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(s.into())
    }
}

impl Serialize for LocaleType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.into())
    }
}

impl<'de> Deserialize<'de> for LocaleType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Ok(String::deserialize(deserializer)?.into())
    }
}

/// <p>Set of options which defines notification preferences of given action.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
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

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct UnknownOrderType {
    name: String,
}

#[allow(non_camel_case_types)]
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
#[non_exhaustive]
pub enum OrderType {
    Ascending,
    Descending,
    #[doc(hidden)]
    UnknownVariant(UnknownOrderType),
}

impl Default for OrderType {
    fn default() -> Self {
        "".into()
    }
}

impl fmt::Display for OrderType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.into())
    }
}

impl rusoto_core::param::ToParam for OrderType {
    fn to_param(&self) -> String {
        self.to_string()
    }
}

impl Into<String> for OrderType {
    fn into(self) -> String {
        match self {
            OrderType::Ascending => "ASCENDING".to_string(),
            OrderType::Descending => "DESCENDING".to_string(),
            OrderType::UnknownVariant(UnknownOrderType { name: original }) => original,
        }
    }
}

impl<'a> Into<&'a str> for &'a OrderType {
    fn into(self) -> &'a str {
        match self {
            OrderType::Ascending => &"ASCENDING",
            OrderType::Descending => &"DESCENDING",
            OrderType::UnknownVariant(UnknownOrderType { name: original }) => original,
        }
    }
}

impl From<&str> for OrderType {
    fn from(name: &str) -> Self {
        match name {
            "ASCENDING" => OrderType::Ascending,
            "DESCENDING" => OrderType::Descending,
            _ => OrderType::UnknownVariant(UnknownOrderType {
                name: name.to_owned(),
            }),
        }
    }
}

impl From<String> for OrderType {
    fn from(name: String) -> Self {
        match &*name {
            "ASCENDING" => OrderType::Ascending,
            "DESCENDING" => OrderType::Descending,
            _ => OrderType::UnknownVariant(UnknownOrderType { name }),
        }
    }
}

impl ::std::str::FromStr for OrderType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(s.into())
    }
}

impl Serialize for OrderType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.into())
    }
}

#[cfg(feature = "deserialize_structs")]
impl<'de> Deserialize<'de> for OrderType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Ok(String::deserialize(deserializer)?.into())
    }
}

/// <p>Describes the users or user groups.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
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
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct PermissionInfo {
    /// <p>The role of the user.</p>
    #[serde(rename = "Role")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role: Option<RoleType>,
    /// <p>The type of permissions.</p>
    #[serde(rename = "Type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<RolePermissionType>,
}

/// <p>Describes a resource.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
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
    pub type_: Option<PrincipalType>,
}

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct UnknownPrincipalType {
    name: String,
}

#[allow(non_camel_case_types)]
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
#[non_exhaustive]
pub enum PrincipalType {
    Anonymous,
    Group,
    Invite,
    Organization,
    User,
    #[doc(hidden)]
    UnknownVariant(UnknownPrincipalType),
}

impl Default for PrincipalType {
    fn default() -> Self {
        "".into()
    }
}

impl fmt::Display for PrincipalType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.into())
    }
}

impl rusoto_core::param::ToParam for PrincipalType {
    fn to_param(&self) -> String {
        self.to_string()
    }
}

impl Into<String> for PrincipalType {
    fn into(self) -> String {
        match self {
            PrincipalType::Anonymous => "ANONYMOUS".to_string(),
            PrincipalType::Group => "GROUP".to_string(),
            PrincipalType::Invite => "INVITE".to_string(),
            PrincipalType::Organization => "ORGANIZATION".to_string(),
            PrincipalType::User => "USER".to_string(),
            PrincipalType::UnknownVariant(UnknownPrincipalType { name: original }) => original,
        }
    }
}

impl<'a> Into<&'a str> for &'a PrincipalType {
    fn into(self) -> &'a str {
        match self {
            PrincipalType::Anonymous => &"ANONYMOUS",
            PrincipalType::Group => &"GROUP",
            PrincipalType::Invite => &"INVITE",
            PrincipalType::Organization => &"ORGANIZATION",
            PrincipalType::User => &"USER",
            PrincipalType::UnknownVariant(UnknownPrincipalType { name: original }) => original,
        }
    }
}

impl From<&str> for PrincipalType {
    fn from(name: &str) -> Self {
        match name {
            "ANONYMOUS" => PrincipalType::Anonymous,
            "GROUP" => PrincipalType::Group,
            "INVITE" => PrincipalType::Invite,
            "ORGANIZATION" => PrincipalType::Organization,
            "USER" => PrincipalType::User,
            _ => PrincipalType::UnknownVariant(UnknownPrincipalType {
                name: name.to_owned(),
            }),
        }
    }
}

impl From<String> for PrincipalType {
    fn from(name: String) -> Self {
        match &*name {
            "ANONYMOUS" => PrincipalType::Anonymous,
            "GROUP" => PrincipalType::Group,
            "INVITE" => PrincipalType::Invite,
            "ORGANIZATION" => PrincipalType::Organization,
            "USER" => PrincipalType::User,
            _ => PrincipalType::UnknownVariant(UnknownPrincipalType { name }),
        }
    }
}

impl ::std::str::FromStr for PrincipalType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(s.into())
    }
}

impl Serialize for PrincipalType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.into())
    }
}

impl<'de> Deserialize<'de> for PrincipalType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Ok(String::deserialize(deserializer)?.into())
    }
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct RemoveAllResourcePermissionsRequest {
    /// <p>Amazon WorkDocs authentication token. Not required when using AWS administrator credentials to access the API.</p>
    #[serde(rename = "AuthenticationToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authentication_token: Option<String>,
    /// <p>The ID of the resource.</p>
    #[serde(rename = "ResourceId")]
    pub resource_id: String,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct RemoveResourcePermissionRequest {
    /// <p>Amazon WorkDocs authentication token. Not required when using AWS administrator credentials to access the API.</p>
    #[serde(rename = "AuthenticationToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authentication_token: Option<String>,
    /// <p>The principal ID of the resource.</p>
    #[serde(rename = "PrincipalId")]
    pub principal_id: String,
    /// <p>The principal type of the resource.</p>
    #[serde(rename = "PrincipalType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub principal_type: Option<PrincipalType>,
    /// <p>The ID of the resource.</p>
    #[serde(rename = "ResourceId")]
    pub resource_id: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct UnknownResourceCollectionType {
    name: String,
}

#[allow(non_camel_case_types)]
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
#[non_exhaustive]
pub enum ResourceCollectionType {
    SharedWithMe,
    #[doc(hidden)]
    UnknownVariant(UnknownResourceCollectionType),
}

impl Default for ResourceCollectionType {
    fn default() -> Self {
        "".into()
    }
}

impl fmt::Display for ResourceCollectionType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.into())
    }
}

impl rusoto_core::param::ToParam for ResourceCollectionType {
    fn to_param(&self) -> String {
        self.to_string()
    }
}

impl Into<String> for ResourceCollectionType {
    fn into(self) -> String {
        match self {
            ResourceCollectionType::SharedWithMe => "SHARED_WITH_ME".to_string(),
            ResourceCollectionType::UnknownVariant(UnknownResourceCollectionType {
                name: original,
            }) => original,
        }
    }
}

impl<'a> Into<&'a str> for &'a ResourceCollectionType {
    fn into(self) -> &'a str {
        match self {
            ResourceCollectionType::SharedWithMe => &"SHARED_WITH_ME",
            ResourceCollectionType::UnknownVariant(UnknownResourceCollectionType {
                name: original,
            }) => original,
        }
    }
}

impl From<&str> for ResourceCollectionType {
    fn from(name: &str) -> Self {
        match name {
            "SHARED_WITH_ME" => ResourceCollectionType::SharedWithMe,
            _ => ResourceCollectionType::UnknownVariant(UnknownResourceCollectionType {
                name: name.to_owned(),
            }),
        }
    }
}

impl From<String> for ResourceCollectionType {
    fn from(name: String) -> Self {
        match &*name {
            "SHARED_WITH_ME" => ResourceCollectionType::SharedWithMe,
            _ => ResourceCollectionType::UnknownVariant(UnknownResourceCollectionType { name }),
        }
    }
}

impl ::std::str::FromStr for ResourceCollectionType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(s.into())
    }
}

impl Serialize for ResourceCollectionType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.into())
    }
}

#[cfg(feature = "deserialize_structs")]
impl<'de> Deserialize<'de> for ResourceCollectionType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Ok(String::deserialize(deserializer)?.into())
    }
}

/// <p>Describes the metadata of a resource.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
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
    pub type_: Option<ResourceType>,
    /// <p>The version ID of the resource. This is an optional field and is filled for action on document version.</p>
    #[serde(rename = "VersionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version_id: Option<String>,
}

/// <p>Describes the path information of a resource.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ResourcePath {
    /// <p>The components of the resource path.</p>
    #[serde(rename = "Components")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub components: Option<Vec<ResourcePathComponent>>,
}

/// <p>Describes the resource path.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
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

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct UnknownResourceSortType {
    name: String,
}

#[allow(non_camel_case_types)]
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
#[non_exhaustive]
pub enum ResourceSortType {
    Date,
    Name,
    #[doc(hidden)]
    UnknownVariant(UnknownResourceSortType),
}

impl Default for ResourceSortType {
    fn default() -> Self {
        "".into()
    }
}

impl fmt::Display for ResourceSortType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.into())
    }
}

impl rusoto_core::param::ToParam for ResourceSortType {
    fn to_param(&self) -> String {
        self.to_string()
    }
}

impl Into<String> for ResourceSortType {
    fn into(self) -> String {
        match self {
            ResourceSortType::Date => "DATE".to_string(),
            ResourceSortType::Name => "NAME".to_string(),
            ResourceSortType::UnknownVariant(UnknownResourceSortType { name: original }) => {
                original
            }
        }
    }
}

impl<'a> Into<&'a str> for &'a ResourceSortType {
    fn into(self) -> &'a str {
        match self {
            ResourceSortType::Date => &"DATE",
            ResourceSortType::Name => &"NAME",
            ResourceSortType::UnknownVariant(UnknownResourceSortType { name: original }) => {
                original
            }
        }
    }
}

impl From<&str> for ResourceSortType {
    fn from(name: &str) -> Self {
        match name {
            "DATE" => ResourceSortType::Date,
            "NAME" => ResourceSortType::Name,
            _ => ResourceSortType::UnknownVariant(UnknownResourceSortType {
                name: name.to_owned(),
            }),
        }
    }
}

impl From<String> for ResourceSortType {
    fn from(name: String) -> Self {
        match &*name {
            "DATE" => ResourceSortType::Date,
            "NAME" => ResourceSortType::Name,
            _ => ResourceSortType::UnknownVariant(UnknownResourceSortType { name }),
        }
    }
}

impl ::std::str::FromStr for ResourceSortType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(s.into())
    }
}

impl Serialize for ResourceSortType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.into())
    }
}

#[cfg(feature = "deserialize_structs")]
impl<'de> Deserialize<'de> for ResourceSortType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Ok(String::deserialize(deserializer)?.into())
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct UnknownResourceStateType {
    name: String,
}

#[allow(non_camel_case_types)]
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
#[non_exhaustive]
pub enum ResourceStateType {
    Active,
    Recycled,
    Recycling,
    Restoring,
    #[doc(hidden)]
    UnknownVariant(UnknownResourceStateType),
}

impl Default for ResourceStateType {
    fn default() -> Self {
        "".into()
    }
}

impl fmt::Display for ResourceStateType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.into())
    }
}

impl rusoto_core::param::ToParam for ResourceStateType {
    fn to_param(&self) -> String {
        self.to_string()
    }
}

impl Into<String> for ResourceStateType {
    fn into(self) -> String {
        match self {
            ResourceStateType::Active => "ACTIVE".to_string(),
            ResourceStateType::Recycled => "RECYCLED".to_string(),
            ResourceStateType::Recycling => "RECYCLING".to_string(),
            ResourceStateType::Restoring => "RESTORING".to_string(),
            ResourceStateType::UnknownVariant(UnknownResourceStateType { name: original }) => {
                original
            }
        }
    }
}

impl<'a> Into<&'a str> for &'a ResourceStateType {
    fn into(self) -> &'a str {
        match self {
            ResourceStateType::Active => &"ACTIVE",
            ResourceStateType::Recycled => &"RECYCLED",
            ResourceStateType::Recycling => &"RECYCLING",
            ResourceStateType::Restoring => &"RESTORING",
            ResourceStateType::UnknownVariant(UnknownResourceStateType { name: original }) => {
                original
            }
        }
    }
}

impl From<&str> for ResourceStateType {
    fn from(name: &str) -> Self {
        match name {
            "ACTIVE" => ResourceStateType::Active,
            "RECYCLED" => ResourceStateType::Recycled,
            "RECYCLING" => ResourceStateType::Recycling,
            "RESTORING" => ResourceStateType::Restoring,
            _ => ResourceStateType::UnknownVariant(UnknownResourceStateType {
                name: name.to_owned(),
            }),
        }
    }
}

impl From<String> for ResourceStateType {
    fn from(name: String) -> Self {
        match &*name {
            "ACTIVE" => ResourceStateType::Active,
            "RECYCLED" => ResourceStateType::Recycled,
            "RECYCLING" => ResourceStateType::Recycling,
            "RESTORING" => ResourceStateType::Restoring,
            _ => ResourceStateType::UnknownVariant(UnknownResourceStateType { name }),
        }
    }
}

impl ::std::str::FromStr for ResourceStateType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(s.into())
    }
}

impl Serialize for ResourceStateType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.into())
    }
}

impl<'de> Deserialize<'de> for ResourceStateType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Ok(String::deserialize(deserializer)?.into())
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct UnknownResourceType {
    name: String,
}

#[allow(non_camel_case_types)]
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
#[non_exhaustive]
pub enum ResourceType {
    Document,
    Folder,
    #[doc(hidden)]
    UnknownVariant(UnknownResourceType),
}

impl Default for ResourceType {
    fn default() -> Self {
        "".into()
    }
}

impl fmt::Display for ResourceType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.into())
    }
}

impl rusoto_core::param::ToParam for ResourceType {
    fn to_param(&self) -> String {
        self.to_string()
    }
}

impl Into<String> for ResourceType {
    fn into(self) -> String {
        match self {
            ResourceType::Document => "DOCUMENT".to_string(),
            ResourceType::Folder => "FOLDER".to_string(),
            ResourceType::UnknownVariant(UnknownResourceType { name: original }) => original,
        }
    }
}

impl<'a> Into<&'a str> for &'a ResourceType {
    fn into(self) -> &'a str {
        match self {
            ResourceType::Document => &"DOCUMENT",
            ResourceType::Folder => &"FOLDER",
            ResourceType::UnknownVariant(UnknownResourceType { name: original }) => original,
        }
    }
}

impl From<&str> for ResourceType {
    fn from(name: &str) -> Self {
        match name {
            "DOCUMENT" => ResourceType::Document,
            "FOLDER" => ResourceType::Folder,
            _ => ResourceType::UnknownVariant(UnknownResourceType {
                name: name.to_owned(),
            }),
        }
    }
}

impl From<String> for ResourceType {
    fn from(name: String) -> Self {
        match &*name {
            "DOCUMENT" => ResourceType::Document,
            "FOLDER" => ResourceType::Folder,
            _ => ResourceType::UnknownVariant(UnknownResourceType { name }),
        }
    }
}

impl ::std::str::FromStr for ResourceType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(s.into())
    }
}

#[cfg(any(test, feature = "serialize_structs"))]
impl Serialize for ResourceType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.into())
    }
}

impl<'de> Deserialize<'de> for ResourceType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Ok(String::deserialize(deserializer)?.into())
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct UnknownRolePermissionType {
    name: String,
}

#[allow(non_camel_case_types)]
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
#[non_exhaustive]
pub enum RolePermissionType {
    Direct,
    Inherited,
    #[doc(hidden)]
    UnknownVariant(UnknownRolePermissionType),
}

impl Default for RolePermissionType {
    fn default() -> Self {
        "".into()
    }
}

impl fmt::Display for RolePermissionType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.into())
    }
}

impl rusoto_core::param::ToParam for RolePermissionType {
    fn to_param(&self) -> String {
        self.to_string()
    }
}

impl Into<String> for RolePermissionType {
    fn into(self) -> String {
        match self {
            RolePermissionType::Direct => "DIRECT".to_string(),
            RolePermissionType::Inherited => "INHERITED".to_string(),
            RolePermissionType::UnknownVariant(UnknownRolePermissionType { name: original }) => {
                original
            }
        }
    }
}

impl<'a> Into<&'a str> for &'a RolePermissionType {
    fn into(self) -> &'a str {
        match self {
            RolePermissionType::Direct => &"DIRECT",
            RolePermissionType::Inherited => &"INHERITED",
            RolePermissionType::UnknownVariant(UnknownRolePermissionType { name: original }) => {
                original
            }
        }
    }
}

impl From<&str> for RolePermissionType {
    fn from(name: &str) -> Self {
        match name {
            "DIRECT" => RolePermissionType::Direct,
            "INHERITED" => RolePermissionType::Inherited,
            _ => RolePermissionType::UnknownVariant(UnknownRolePermissionType {
                name: name.to_owned(),
            }),
        }
    }
}

impl From<String> for RolePermissionType {
    fn from(name: String) -> Self {
        match &*name {
            "DIRECT" => RolePermissionType::Direct,
            "INHERITED" => RolePermissionType::Inherited,
            _ => RolePermissionType::UnknownVariant(UnknownRolePermissionType { name }),
        }
    }
}

impl ::std::str::FromStr for RolePermissionType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(s.into())
    }
}

#[cfg(any(test, feature = "serialize_structs"))]
impl Serialize for RolePermissionType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.into())
    }
}

impl<'de> Deserialize<'de> for RolePermissionType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Ok(String::deserialize(deserializer)?.into())
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct UnknownRoleType {
    name: String,
}

#[allow(non_camel_case_types)]
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
#[non_exhaustive]
pub enum RoleType {
    Contributor,
    Coowner,
    Owner,
    Viewer,
    #[doc(hidden)]
    UnknownVariant(UnknownRoleType),
}

impl Default for RoleType {
    fn default() -> Self {
        "".into()
    }
}

impl fmt::Display for RoleType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.into())
    }
}

impl rusoto_core::param::ToParam for RoleType {
    fn to_param(&self) -> String {
        self.to_string()
    }
}

impl Into<String> for RoleType {
    fn into(self) -> String {
        match self {
            RoleType::Contributor => "CONTRIBUTOR".to_string(),
            RoleType::Coowner => "COOWNER".to_string(),
            RoleType::Owner => "OWNER".to_string(),
            RoleType::Viewer => "VIEWER".to_string(),
            RoleType::UnknownVariant(UnknownRoleType { name: original }) => original,
        }
    }
}

impl<'a> Into<&'a str> for &'a RoleType {
    fn into(self) -> &'a str {
        match self {
            RoleType::Contributor => &"CONTRIBUTOR",
            RoleType::Coowner => &"COOWNER",
            RoleType::Owner => &"OWNER",
            RoleType::Viewer => &"VIEWER",
            RoleType::UnknownVariant(UnknownRoleType { name: original }) => original,
        }
    }
}

impl From<&str> for RoleType {
    fn from(name: &str) -> Self {
        match name {
            "CONTRIBUTOR" => RoleType::Contributor,
            "COOWNER" => RoleType::Coowner,
            "OWNER" => RoleType::Owner,
            "VIEWER" => RoleType::Viewer,
            _ => RoleType::UnknownVariant(UnknownRoleType {
                name: name.to_owned(),
            }),
        }
    }
}

impl From<String> for RoleType {
    fn from(name: String) -> Self {
        match &*name {
            "CONTRIBUTOR" => RoleType::Contributor,
            "COOWNER" => RoleType::Coowner,
            "OWNER" => RoleType::Owner,
            "VIEWER" => RoleType::Viewer,
            _ => RoleType::UnknownVariant(UnknownRoleType { name }),
        }
    }
}

impl ::std::str::FromStr for RoleType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(s.into())
    }
}

impl Serialize for RoleType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.into())
    }
}

impl<'de> Deserialize<'de> for RoleType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Ok(String::deserialize(deserializer)?.into())
    }
}

/// <p>Describes the recipient type and ID, if available.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct SharePrincipal {
    /// <p>The ID of the recipient.</p>
    #[serde(rename = "Id")]
    pub id: String,
    /// <p>The role of the recipient.</p>
    #[serde(rename = "Role")]
    pub role: RoleType,
    /// <p>The type of the recipient.</p>
    #[serde(rename = "Type")]
    pub type_: PrincipalType,
}

/// <p>Describes the share results of a resource.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
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
    pub role: Option<RoleType>,
    /// <p>The ID of the resource that was shared.</p>
    #[serde(rename = "ShareId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub share_id: Option<String>,
    /// <p>The status.</p>
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<ShareStatusType>,
    /// <p>The status message.</p>
    #[serde(rename = "StatusMessage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_message: Option<String>,
}

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct UnknownShareStatusType {
    name: String,
}

#[allow(non_camel_case_types)]
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
#[non_exhaustive]
pub enum ShareStatusType {
    Failure,
    Success,
    #[doc(hidden)]
    UnknownVariant(UnknownShareStatusType),
}

impl Default for ShareStatusType {
    fn default() -> Self {
        "".into()
    }
}

impl fmt::Display for ShareStatusType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.into())
    }
}

impl rusoto_core::param::ToParam for ShareStatusType {
    fn to_param(&self) -> String {
        self.to_string()
    }
}

impl Into<String> for ShareStatusType {
    fn into(self) -> String {
        match self {
            ShareStatusType::Failure => "FAILURE".to_string(),
            ShareStatusType::Success => "SUCCESS".to_string(),
            ShareStatusType::UnknownVariant(UnknownShareStatusType { name: original }) => original,
        }
    }
}

impl<'a> Into<&'a str> for &'a ShareStatusType {
    fn into(self) -> &'a str {
        match self {
            ShareStatusType::Failure => &"FAILURE",
            ShareStatusType::Success => &"SUCCESS",
            ShareStatusType::UnknownVariant(UnknownShareStatusType { name: original }) => original,
        }
    }
}

impl From<&str> for ShareStatusType {
    fn from(name: &str) -> Self {
        match name {
            "FAILURE" => ShareStatusType::Failure,
            "SUCCESS" => ShareStatusType::Success,
            _ => ShareStatusType::UnknownVariant(UnknownShareStatusType {
                name: name.to_owned(),
            }),
        }
    }
}

impl From<String> for ShareStatusType {
    fn from(name: String) -> Self {
        match &*name {
            "FAILURE" => ShareStatusType::Failure,
            "SUCCESS" => ShareStatusType::Success,
            _ => ShareStatusType::UnknownVariant(UnknownShareStatusType { name }),
        }
    }
}

impl ::std::str::FromStr for ShareStatusType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(s.into())
    }
}

#[cfg(any(test, feature = "serialize_structs"))]
impl Serialize for ShareStatusType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.into())
    }
}

impl<'de> Deserialize<'de> for ShareStatusType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Ok(String::deserialize(deserializer)?.into())
    }
}

/// <p>Describes the storage for a user.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct StorageRuleType {
    /// <p>The amount of storage allocated, in bytes.</p>
    #[serde(rename = "StorageAllocatedInBytes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage_allocated_in_bytes: Option<i64>,
    /// <p>The type of storage.</p>
    #[serde(rename = "StorageType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage_type: Option<StorageType>,
}

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct UnknownStorageType {
    name: String,
}

#[allow(non_camel_case_types)]
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
#[non_exhaustive]
pub enum StorageType {
    Quota,
    Unlimited,
    #[doc(hidden)]
    UnknownVariant(UnknownStorageType),
}

impl Default for StorageType {
    fn default() -> Self {
        "".into()
    }
}

impl fmt::Display for StorageType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.into())
    }
}

impl rusoto_core::param::ToParam for StorageType {
    fn to_param(&self) -> String {
        self.to_string()
    }
}

impl Into<String> for StorageType {
    fn into(self) -> String {
        match self {
            StorageType::Quota => "QUOTA".to_string(),
            StorageType::Unlimited => "UNLIMITED".to_string(),
            StorageType::UnknownVariant(UnknownStorageType { name: original }) => original,
        }
    }
}

impl<'a> Into<&'a str> for &'a StorageType {
    fn into(self) -> &'a str {
        match self {
            StorageType::Quota => &"QUOTA",
            StorageType::Unlimited => &"UNLIMITED",
            StorageType::UnknownVariant(UnknownStorageType { name: original }) => original,
        }
    }
}

impl From<&str> for StorageType {
    fn from(name: &str) -> Self {
        match name {
            "QUOTA" => StorageType::Quota,
            "UNLIMITED" => StorageType::Unlimited,
            _ => StorageType::UnknownVariant(UnknownStorageType {
                name: name.to_owned(),
            }),
        }
    }
}

impl From<String> for StorageType {
    fn from(name: String) -> Self {
        match &*name {
            "QUOTA" => StorageType::Quota,
            "UNLIMITED" => StorageType::Unlimited,
            _ => StorageType::UnknownVariant(UnknownStorageType { name }),
        }
    }
}

impl ::std::str::FromStr for StorageType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(s.into())
    }
}

impl Serialize for StorageType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.into())
    }
}

impl<'de> Deserialize<'de> for StorageType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Ok(String::deserialize(deserializer)?.into())
    }
}

/// <p>Describes a subscription.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct Subscription {
    /// <p>The endpoint of the subscription.</p>
    #[serde(rename = "EndPoint")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_point: Option<String>,
    /// <p>The protocol of the subscription.</p>
    #[serde(rename = "Protocol")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protocol: Option<SubscriptionProtocolType>,
    /// <p>The ID of the subscription.</p>
    #[serde(rename = "SubscriptionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscription_id: Option<String>,
}

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct UnknownSubscriptionProtocolType {
    name: String,
}

#[allow(non_camel_case_types)]
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
#[non_exhaustive]
pub enum SubscriptionProtocolType {
    Https,
    #[doc(hidden)]
    UnknownVariant(UnknownSubscriptionProtocolType),
}

impl Default for SubscriptionProtocolType {
    fn default() -> Self {
        "".into()
    }
}

impl fmt::Display for SubscriptionProtocolType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.into())
    }
}

impl rusoto_core::param::ToParam for SubscriptionProtocolType {
    fn to_param(&self) -> String {
        self.to_string()
    }
}

impl Into<String> for SubscriptionProtocolType {
    fn into(self) -> String {
        match self {
            SubscriptionProtocolType::Https => "HTTPS".to_string(),
            SubscriptionProtocolType::UnknownVariant(UnknownSubscriptionProtocolType {
                name: original,
            }) => original,
        }
    }
}

impl<'a> Into<&'a str> for &'a SubscriptionProtocolType {
    fn into(self) -> &'a str {
        match self {
            SubscriptionProtocolType::Https => &"HTTPS",
            SubscriptionProtocolType::UnknownVariant(UnknownSubscriptionProtocolType {
                name: original,
            }) => original,
        }
    }
}

impl From<&str> for SubscriptionProtocolType {
    fn from(name: &str) -> Self {
        match name {
            "HTTPS" => SubscriptionProtocolType::Https,
            _ => SubscriptionProtocolType::UnknownVariant(UnknownSubscriptionProtocolType {
                name: name.to_owned(),
            }),
        }
    }
}

impl From<String> for SubscriptionProtocolType {
    fn from(name: String) -> Self {
        match &*name {
            "HTTPS" => SubscriptionProtocolType::Https,
            _ => SubscriptionProtocolType::UnknownVariant(UnknownSubscriptionProtocolType { name }),
        }
    }
}

impl ::std::str::FromStr for SubscriptionProtocolType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(s.into())
    }
}

impl Serialize for SubscriptionProtocolType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.into())
    }
}

impl<'de> Deserialize<'de> for SubscriptionProtocolType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Ok(String::deserialize(deserializer)?.into())
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct UnknownSubscriptionType {
    name: String,
}

#[allow(non_camel_case_types)]
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
#[non_exhaustive]
pub enum SubscriptionType {
    All,
    #[doc(hidden)]
    UnknownVariant(UnknownSubscriptionType),
}

impl Default for SubscriptionType {
    fn default() -> Self {
        "".into()
    }
}

impl fmt::Display for SubscriptionType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.into())
    }
}

impl rusoto_core::param::ToParam for SubscriptionType {
    fn to_param(&self) -> String {
        self.to_string()
    }
}

impl Into<String> for SubscriptionType {
    fn into(self) -> String {
        match self {
            SubscriptionType::All => "ALL".to_string(),
            SubscriptionType::UnknownVariant(UnknownSubscriptionType { name: original }) => {
                original
            }
        }
    }
}

impl<'a> Into<&'a str> for &'a SubscriptionType {
    fn into(self) -> &'a str {
        match self {
            SubscriptionType::All => &"ALL",
            SubscriptionType::UnknownVariant(UnknownSubscriptionType { name: original }) => {
                original
            }
        }
    }
}

impl From<&str> for SubscriptionType {
    fn from(name: &str) -> Self {
        match name {
            "ALL" => SubscriptionType::All,
            _ => SubscriptionType::UnknownVariant(UnknownSubscriptionType {
                name: name.to_owned(),
            }),
        }
    }
}

impl From<String> for SubscriptionType {
    fn from(name: String) -> Self {
        match &*name {
            "ALL" => SubscriptionType::All,
            _ => SubscriptionType::UnknownVariant(UnknownSubscriptionType { name }),
        }
    }
}

impl ::std::str::FromStr for SubscriptionType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(s.into())
    }
}

impl Serialize for SubscriptionType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.into())
    }
}

#[cfg(feature = "deserialize_structs")]
impl<'de> Deserialize<'de> for SubscriptionType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Ok(String::deserialize(deserializer)?.into())
    }
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UpdateDocumentRequest {
    /// <p>Amazon WorkDocs authentication token. Not required when using AWS administrator credentials to access the API.</p>
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
    pub resource_state: Option<ResourceStateType>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UpdateDocumentVersionRequest {
    /// <p>Amazon WorkDocs authentication token. Not required when using AWS administrator credentials to access the API.</p>
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
    pub version_status: Option<DocumentVersionStatus>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UpdateFolderRequest {
    /// <p>Amazon WorkDocs authentication token. Not required when using AWS administrator credentials to access the API.</p>
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
    pub resource_state: Option<ResourceStateType>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UpdateUserRequest {
    /// <p>Amazon WorkDocs authentication token. Not required when using AWS administrator credentials to access the API.</p>
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
    pub grant_poweruser_privileges: Option<BooleanEnumType>,
    /// <p>The locale of the user.</p>
    #[serde(rename = "Locale")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub locale: Option<LocaleType>,
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
    pub type_: Option<UserType>,
    /// <p>The ID of the user.</p>
    #[serde(rename = "UserId")]
    pub user_id: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UpdateUserResponse {
    /// <p>The user information.</p>
    #[serde(rename = "User")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<User>,
}

/// <p>Describes the upload.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
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
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
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
    pub locale: Option<LocaleType>,
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
    pub status: Option<UserStatusType>,
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
    pub type_: Option<UserType>,
    /// <p>The login name of the user.</p>
    #[serde(rename = "Username")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
}

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct UnknownUserFilterType {
    name: String,
}

#[allow(non_camel_case_types)]
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
#[non_exhaustive]
pub enum UserFilterType {
    ActivePending,
    All,
    #[doc(hidden)]
    UnknownVariant(UnknownUserFilterType),
}

impl Default for UserFilterType {
    fn default() -> Self {
        "".into()
    }
}

impl fmt::Display for UserFilterType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.into())
    }
}

impl rusoto_core::param::ToParam for UserFilterType {
    fn to_param(&self) -> String {
        self.to_string()
    }
}

impl Into<String> for UserFilterType {
    fn into(self) -> String {
        match self {
            UserFilterType::ActivePending => "ACTIVE_PENDING".to_string(),
            UserFilterType::All => "ALL".to_string(),
            UserFilterType::UnknownVariant(UnknownUserFilterType { name: original }) => original,
        }
    }
}

impl<'a> Into<&'a str> for &'a UserFilterType {
    fn into(self) -> &'a str {
        match self {
            UserFilterType::ActivePending => &"ACTIVE_PENDING",
            UserFilterType::All => &"ALL",
            UserFilterType::UnknownVariant(UnknownUserFilterType { name: original }) => original,
        }
    }
}

impl From<&str> for UserFilterType {
    fn from(name: &str) -> Self {
        match name {
            "ACTIVE_PENDING" => UserFilterType::ActivePending,
            "ALL" => UserFilterType::All,
            _ => UserFilterType::UnknownVariant(UnknownUserFilterType {
                name: name.to_owned(),
            }),
        }
    }
}

impl From<String> for UserFilterType {
    fn from(name: String) -> Self {
        match &*name {
            "ACTIVE_PENDING" => UserFilterType::ActivePending,
            "ALL" => UserFilterType::All,
            _ => UserFilterType::UnknownVariant(UnknownUserFilterType { name }),
        }
    }
}

impl ::std::str::FromStr for UserFilterType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(s.into())
    }
}

impl Serialize for UserFilterType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.into())
    }
}

#[cfg(feature = "deserialize_structs")]
impl<'de> Deserialize<'de> for UserFilterType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Ok(String::deserialize(deserializer)?.into())
    }
}

/// <p>Describes the metadata of the user.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
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

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct UnknownUserSortType {
    name: String,
}

#[allow(non_camel_case_types)]
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
#[non_exhaustive]
pub enum UserSortType {
    FullName,
    StorageLimit,
    StorageUsed,
    UserName,
    UserStatus,
    #[doc(hidden)]
    UnknownVariant(UnknownUserSortType),
}

impl Default for UserSortType {
    fn default() -> Self {
        "".into()
    }
}

impl fmt::Display for UserSortType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.into())
    }
}

impl rusoto_core::param::ToParam for UserSortType {
    fn to_param(&self) -> String {
        self.to_string()
    }
}

impl Into<String> for UserSortType {
    fn into(self) -> String {
        match self {
            UserSortType::FullName => "FULL_NAME".to_string(),
            UserSortType::StorageLimit => "STORAGE_LIMIT".to_string(),
            UserSortType::StorageUsed => "STORAGE_USED".to_string(),
            UserSortType::UserName => "USER_NAME".to_string(),
            UserSortType::UserStatus => "USER_STATUS".to_string(),
            UserSortType::UnknownVariant(UnknownUserSortType { name: original }) => original,
        }
    }
}

impl<'a> Into<&'a str> for &'a UserSortType {
    fn into(self) -> &'a str {
        match self {
            UserSortType::FullName => &"FULL_NAME",
            UserSortType::StorageLimit => &"STORAGE_LIMIT",
            UserSortType::StorageUsed => &"STORAGE_USED",
            UserSortType::UserName => &"USER_NAME",
            UserSortType::UserStatus => &"USER_STATUS",
            UserSortType::UnknownVariant(UnknownUserSortType { name: original }) => original,
        }
    }
}

impl From<&str> for UserSortType {
    fn from(name: &str) -> Self {
        match name {
            "FULL_NAME" => UserSortType::FullName,
            "STORAGE_LIMIT" => UserSortType::StorageLimit,
            "STORAGE_USED" => UserSortType::StorageUsed,
            "USER_NAME" => UserSortType::UserName,
            "USER_STATUS" => UserSortType::UserStatus,
            _ => UserSortType::UnknownVariant(UnknownUserSortType {
                name: name.to_owned(),
            }),
        }
    }
}

impl From<String> for UserSortType {
    fn from(name: String) -> Self {
        match &*name {
            "FULL_NAME" => UserSortType::FullName,
            "STORAGE_LIMIT" => UserSortType::StorageLimit,
            "STORAGE_USED" => UserSortType::StorageUsed,
            "USER_NAME" => UserSortType::UserName,
            "USER_STATUS" => UserSortType::UserStatus,
            _ => UserSortType::UnknownVariant(UnknownUserSortType { name }),
        }
    }
}

impl ::std::str::FromStr for UserSortType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(s.into())
    }
}

impl Serialize for UserSortType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.into())
    }
}

#[cfg(feature = "deserialize_structs")]
impl<'de> Deserialize<'de> for UserSortType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Ok(String::deserialize(deserializer)?.into())
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct UnknownUserStatusType {
    name: String,
}

#[allow(non_camel_case_types)]
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
#[non_exhaustive]
pub enum UserStatusType {
    Active,
    Inactive,
    Pending,
    #[doc(hidden)]
    UnknownVariant(UnknownUserStatusType),
}

impl Default for UserStatusType {
    fn default() -> Self {
        "".into()
    }
}

impl fmt::Display for UserStatusType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.into())
    }
}

impl rusoto_core::param::ToParam for UserStatusType {
    fn to_param(&self) -> String {
        self.to_string()
    }
}

impl Into<String> for UserStatusType {
    fn into(self) -> String {
        match self {
            UserStatusType::Active => "ACTIVE".to_string(),
            UserStatusType::Inactive => "INACTIVE".to_string(),
            UserStatusType::Pending => "PENDING".to_string(),
            UserStatusType::UnknownVariant(UnknownUserStatusType { name: original }) => original,
        }
    }
}

impl<'a> Into<&'a str> for &'a UserStatusType {
    fn into(self) -> &'a str {
        match self {
            UserStatusType::Active => &"ACTIVE",
            UserStatusType::Inactive => &"INACTIVE",
            UserStatusType::Pending => &"PENDING",
            UserStatusType::UnknownVariant(UnknownUserStatusType { name: original }) => original,
        }
    }
}

impl From<&str> for UserStatusType {
    fn from(name: &str) -> Self {
        match name {
            "ACTIVE" => UserStatusType::Active,
            "INACTIVE" => UserStatusType::Inactive,
            "PENDING" => UserStatusType::Pending,
            _ => UserStatusType::UnknownVariant(UnknownUserStatusType {
                name: name.to_owned(),
            }),
        }
    }
}

impl From<String> for UserStatusType {
    fn from(name: String) -> Self {
        match &*name {
            "ACTIVE" => UserStatusType::Active,
            "INACTIVE" => UserStatusType::Inactive,
            "PENDING" => UserStatusType::Pending,
            _ => UserStatusType::UnknownVariant(UnknownUserStatusType { name }),
        }
    }
}

impl ::std::str::FromStr for UserStatusType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(s.into())
    }
}

#[cfg(any(test, feature = "serialize_structs"))]
impl Serialize for UserStatusType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.into())
    }
}

impl<'de> Deserialize<'de> for UserStatusType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Ok(String::deserialize(deserializer)?.into())
    }
}

/// <p>Describes the storage for a user.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
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

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct UnknownUserType {
    name: String,
}

#[allow(non_camel_case_types)]
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
#[non_exhaustive]
pub enum UserType {
    Admin,
    Minimaluser,
    Poweruser,
    User,
    Workspacesuser,
    #[doc(hidden)]
    UnknownVariant(UnknownUserType),
}

impl Default for UserType {
    fn default() -> Self {
        "".into()
    }
}

impl fmt::Display for UserType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.into())
    }
}

impl rusoto_core::param::ToParam for UserType {
    fn to_param(&self) -> String {
        self.to_string()
    }
}

impl Into<String> for UserType {
    fn into(self) -> String {
        match self {
            UserType::Admin => "ADMIN".to_string(),
            UserType::Minimaluser => "MINIMALUSER".to_string(),
            UserType::Poweruser => "POWERUSER".to_string(),
            UserType::User => "USER".to_string(),
            UserType::Workspacesuser => "WORKSPACESUSER".to_string(),
            UserType::UnknownVariant(UnknownUserType { name: original }) => original,
        }
    }
}

impl<'a> Into<&'a str> for &'a UserType {
    fn into(self) -> &'a str {
        match self {
            UserType::Admin => &"ADMIN",
            UserType::Minimaluser => &"MINIMALUSER",
            UserType::Poweruser => &"POWERUSER",
            UserType::User => &"USER",
            UserType::Workspacesuser => &"WORKSPACESUSER",
            UserType::UnknownVariant(UnknownUserType { name: original }) => original,
        }
    }
}

impl From<&str> for UserType {
    fn from(name: &str) -> Self {
        match name {
            "ADMIN" => UserType::Admin,
            "MINIMALUSER" => UserType::Minimaluser,
            "POWERUSER" => UserType::Poweruser,
            "USER" => UserType::User,
            "WORKSPACESUSER" => UserType::Workspacesuser,
            _ => UserType::UnknownVariant(UnknownUserType {
                name: name.to_owned(),
            }),
        }
    }
}

impl From<String> for UserType {
    fn from(name: String) -> Self {
        match &*name {
            "ADMIN" => UserType::Admin,
            "MINIMALUSER" => UserType::Minimaluser,
            "POWERUSER" => UserType::Poweruser,
            "USER" => UserType::User,
            "WORKSPACESUSER" => UserType::Workspacesuser,
            _ => UserType::UnknownVariant(UnknownUserType { name }),
        }
    }
}

impl ::std::str::FromStr for UserType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(s.into())
    }
}

impl Serialize for UserType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.into())
    }
}

impl<'de> Deserialize<'de> for UserType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Ok(String::deserialize(deserializer)?.into())
    }
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
        RusotoError::Unknown(res)
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
        RusotoError::Unknown(res)
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
        RusotoError::Unknown(res)
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
        RusotoError::Unknown(res)
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
        RusotoError::Unknown(res)
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
        RusotoError::Unknown(res)
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
        RusotoError::Unknown(res)
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
        RusotoError::Unknown(res)
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
        RusotoError::Unknown(res)
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
        RusotoError::Unknown(res)
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
        RusotoError::Unknown(res)
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
        RusotoError::Unknown(res)
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
        RusotoError::Unknown(res)
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
        RusotoError::Unknown(res)
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
        RusotoError::Unknown(res)
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
        RusotoError::Unknown(res)
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
        RusotoError::Unknown(res)
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
        RusotoError::Unknown(res)
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
        RusotoError::Unknown(res)
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
        RusotoError::Unknown(res)
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
        RusotoError::Unknown(res)
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
        RusotoError::Unknown(res)
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
        RusotoError::Unknown(res)
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
        RusotoError::Unknown(res)
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
        RusotoError::Unknown(res)
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
        RusotoError::Unknown(res)
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
        RusotoError::Unknown(res)
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
        RusotoError::Unknown(res)
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
        RusotoError::Unknown(res)
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
        RusotoError::Unknown(res)
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
        RusotoError::Unknown(res)
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
        RusotoError::Unknown(res)
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
        RusotoError::Unknown(res)
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
        RusotoError::Unknown(res)
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
        RusotoError::Unknown(res)
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
        RusotoError::Unknown(res)
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
        RusotoError::Unknown(res)
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
        RusotoError::Unknown(res)
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
        RusotoError::Unknown(res)
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
        RusotoError::Unknown(res)
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
        RusotoError::Unknown(res)
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

    /// <p>Retrieves details of the current user for whom the authentication token was generated. This is not a valid action for SigV4 (administrative API) clients.</p> <p>This action requires an authentication token. To get an authentication token, register an application with Amazon WorkDocs. For more information, see <a href="https://docs.aws.amazon.com/workdocs/latest/developerguide/wd-auth-user.html">Authentication and Access Control for User Applications</a> in the <i>Amazon WorkDocs Developer Guide</i>.</p>
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
    #[allow(unused_mut)]
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

        request.add_optional_header("Authentication", input.authentication_token.as_ref());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 204 {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = ::std::mem::drop(response);

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(AbortDocumentVersionUploadError::from_response(response))
        }
    }

    /// <p>Activates the specified user. Only active users can access Amazon WorkDocs.</p>
    #[allow(unused_mut)]
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

        request.add_optional_header("Authentication", input.authentication_token.as_ref());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<ActivateUserResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(ActivateUserError::from_response(response))
        }
    }

    /// <p>Creates a set of permissions for the specified folder or document. The resource permissions are overwritten if the principals already have different permissions.</p>
    #[allow(unused_mut)]
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
        request.add_optional_header("Authentication", input.authentication_token.as_ref());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 201 {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<AddResourcePermissionsResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(AddResourcePermissionsError::from_response(response))
        }
    }

    /// <p>Adds a new comment to the specified document version.</p>
    #[allow(unused_mut)]
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
        request.add_optional_header("Authentication", input.authentication_token.as_ref());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 201 {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<CreateCommentResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(CreateCommentError::from_response(response))
        }
    }

    /// <p>Adds one or more custom properties to the specified resource (a folder, document, or version).</p>
    #[allow(unused_mut)]
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
        request.add_optional_header("Authentication", input.authentication_token.as_ref());
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
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<CreateCustomMetadataResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(CreateCustomMetadataError::from_response(response))
        }
    }

    /// <p>Creates a folder with the specified name and parent folder.</p>
    #[allow(unused_mut)]
    async fn create_folder(
        &self,
        input: CreateFolderRequest,
    ) -> Result<CreateFolderResponse, RusotoError<CreateFolderError>> {
        let request_uri = "/api/v1/folders";

        let mut request = SignedRequest::new("POST", "workdocs", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);
        request.add_optional_header("Authentication", input.authentication_token.as_ref());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 201 {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<CreateFolderResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(CreateFolderError::from_response(response))
        }
    }

    /// <p>Adds the specified list of labels to the given resource (a document or folder)</p>
    #[allow(unused_mut)]
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
        request.add_optional_header("Authentication", input.authentication_token.as_ref());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<CreateLabelsResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(CreateLabelsError::from_response(response))
        }
    }

    /// <p>Configure Amazon WorkDocs to use Amazon SNS notifications. The endpoint receives a confirmation message, and must confirm the subscription.</p> <p>For more information, see <a href="https://docs.aws.amazon.com/workdocs/latest/developerguide/subscribe-notifications.html">Subscribe to Notifications</a> in the <i>Amazon WorkDocs Developer Guide</i>.</p>
    #[allow(unused_mut)]
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
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<CreateNotificationSubscriptionResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(CreateNotificationSubscriptionError::from_response(response))
        }
    }

    /// <p>Creates a user in a Simple AD or Microsoft AD directory. The status of a newly created user is "ACTIVE". New users can access Amazon WorkDocs.</p>
    #[allow(unused_mut)]
    async fn create_user(
        &self,
        input: CreateUserRequest,
    ) -> Result<CreateUserResponse, RusotoError<CreateUserError>> {
        let request_uri = "/api/v1/users";

        let mut request = SignedRequest::new("POST", "workdocs", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);
        request.add_optional_header("Authentication", input.authentication_token.as_ref());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 201 {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<CreateUserResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(CreateUserError::from_response(response))
        }
    }

    /// <p>Deactivates the specified user, which revokes the user's access to Amazon WorkDocs.</p>
    #[allow(unused_mut)]
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

        request.add_optional_header("Authentication", input.authentication_token.as_ref());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 204 {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = ::std::mem::drop(response);

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(DeactivateUserError::from_response(response))
        }
    }

    /// <p>Deletes the specified comment from the document version.</p>
    #[allow(unused_mut)]
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

        request.add_optional_header("Authentication", input.authentication_token.as_ref());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 204 {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = ::std::mem::drop(response);

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(DeleteCommentError::from_response(response))
        }
    }

    /// <p>Deletes custom metadata from the specified resource.</p>
    #[allow(unused_mut)]
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

        request.add_optional_header("Authentication", input.authentication_token.as_ref());
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
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<DeleteCustomMetadataResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(DeleteCustomMetadataError::from_response(response))
        }
    }

    /// <p>Permanently deletes the specified document and its associated metadata.</p>
    #[allow(unused_mut)]
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

        request.add_optional_header("Authentication", input.authentication_token.as_ref());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 204 {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = ::std::mem::drop(response);

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(DeleteDocumentError::from_response(response))
        }
    }

    /// <p>Permanently deletes the specified folder and its contents.</p>
    #[allow(unused_mut)]
    async fn delete_folder(
        &self,
        input: DeleteFolderRequest,
    ) -> Result<(), RusotoError<DeleteFolderError>> {
        let request_uri = format!("/api/v1/folders/{folder_id}", folder_id = input.folder_id);

        let mut request = SignedRequest::new("DELETE", "workdocs", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.add_optional_header("Authentication", input.authentication_token.as_ref());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 204 {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = ::std::mem::drop(response);

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(DeleteFolderError::from_response(response))
        }
    }

    /// <p>Deletes the contents of the specified folder.</p>
    #[allow(unused_mut)]
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

        request.add_optional_header("Authentication", input.authentication_token.as_ref());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 204 {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = ::std::mem::drop(response);

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(DeleteFolderContentsError::from_response(response))
        }
    }

    /// <p>Deletes the specified list of labels from a resource.</p>
    #[allow(unused_mut)]
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

        request.add_optional_header("Authentication", input.authentication_token.as_ref());
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
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<DeleteLabelsResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(DeleteLabelsError::from_response(response))
        }
    }

    /// <p>Deletes the specified subscription from the specified organization.</p>
    #[allow(unused_mut)]
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
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = ::std::mem::drop(response);

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(DeleteNotificationSubscriptionError::from_response(response))
        }
    }

    /// <p>Deletes the specified user from a Simple AD or Microsoft AD directory.</p>
    #[allow(unused_mut)]
    async fn delete_user(
        &self,
        input: DeleteUserRequest,
    ) -> Result<(), RusotoError<DeleteUserError>> {
        let request_uri = format!("/api/v1/users/{user_id}", user_id = input.user_id);

        let mut request = SignedRequest::new("DELETE", "workdocs", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.add_optional_header("Authentication", input.authentication_token.as_ref());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 204 {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = ::std::mem::drop(response);

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(DeleteUserError::from_response(response))
        }
    }

    /// <p>Describes the user activities in a specified time period.</p>
    #[allow(unused_mut)]
    async fn describe_activities(
        &self,
        input: DescribeActivitiesRequest,
    ) -> Result<DescribeActivitiesResponse, RusotoError<DescribeActivitiesError>> {
        let request_uri = "/api/v1/activities";

        let mut request = SignedRequest::new("GET", "workdocs", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.add_optional_header("Authentication", input.authentication_token.as_ref());
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
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<DescribeActivitiesResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(DescribeActivitiesError::from_response(response))
        }
    }

    /// <p>List all the comments for the specified document version.</p>
    #[allow(unused_mut)]
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

        request.add_optional_header("Authentication", input.authentication_token.as_ref());
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
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<DescribeCommentsResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(DescribeCommentsError::from_response(response))
        }
    }

    /// <p>Retrieves the document versions for the specified document.</p> <p>By default, only active versions are returned.</p>
    #[allow(unused_mut)]
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

        request.add_optional_header("Authentication", input.authentication_token.as_ref());
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
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<DescribeDocumentVersionsResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(DescribeDocumentVersionsError::from_response(response))
        }
    }

    /// <p>Describes the contents of the specified folder, including its documents and subfolders.</p> <p>By default, Amazon WorkDocs returns the first 100 active document and folder metadata items. If there are more results, the response includes a marker that you can use to request the next set of results. You can also request initialized documents.</p>
    #[allow(unused_mut)]
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

        request.add_optional_header("Authentication", input.authentication_token.as_ref());
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
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<DescribeFolderContentsResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(DescribeFolderContentsError::from_response(response))
        }
    }

    /// <p>Describes the groups specified by the query. Groups are defined by the underlying Active Directory.</p>
    #[allow(unused_mut)]
    async fn describe_groups(
        &self,
        input: DescribeGroupsRequest,
    ) -> Result<DescribeGroupsResponse, RusotoError<DescribeGroupsError>> {
        let request_uri = "/api/v1/groups";

        let mut request = SignedRequest::new("GET", "workdocs", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.add_optional_header("Authentication", input.authentication_token.as_ref());
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
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<DescribeGroupsResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(DescribeGroupsError::from_response(response))
        }
    }

    /// <p>Lists the specified notification subscriptions.</p>
    #[allow(unused_mut)]
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
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
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
    #[allow(unused_mut)]
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

        request.add_optional_header("Authentication", input.authentication_token.as_ref());
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
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<DescribeResourcePermissionsResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(DescribeResourcePermissionsError::from_response(response))
        }
    }

    /// <p>Describes the current user's special folders; the <code>RootFolder</code> and the <code>RecycleBin</code>. <code>RootFolder</code> is the root of user's files and folders and <code>RecycleBin</code> is the root of recycled items. This is not a valid action for SigV4 (administrative API) clients.</p> <p>This action requires an authentication token. To get an authentication token, register an application with Amazon WorkDocs. For more information, see <a href="https://docs.aws.amazon.com/workdocs/latest/developerguide/wd-auth-user.html">Authentication and Access Control for User Applications</a> in the <i>Amazon WorkDocs Developer Guide</i>.</p>
    #[allow(unused_mut)]
    async fn describe_root_folders(
        &self,
        input: DescribeRootFoldersRequest,
    ) -> Result<DescribeRootFoldersResponse, RusotoError<DescribeRootFoldersError>> {
        let request_uri = "/api/v1/me/root";

        let mut request = SignedRequest::new("GET", "workdocs", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.add_header("Authentication", &input.authentication_token.to_string());
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
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<DescribeRootFoldersResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(DescribeRootFoldersError::from_response(response))
        }
    }

    /// <p>Describes the specified users. You can describe all users or filter the results (for example, by status or organization).</p> <p>By default, Amazon WorkDocs returns the first 24 active or pending users. If there are more results, the response includes a marker that you can use to request the next set of results.</p>
    #[allow(unused_mut)]
    async fn describe_users(
        &self,
        input: DescribeUsersRequest,
    ) -> Result<DescribeUsersResponse, RusotoError<DescribeUsersError>> {
        let request_uri = "/api/v1/users";

        let mut request = SignedRequest::new("GET", "workdocs", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.add_optional_header("Authentication", input.authentication_token.as_ref());
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
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<DescribeUsersResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(DescribeUsersError::from_response(response))
        }
    }

    /// <p>Retrieves details of the current user for whom the authentication token was generated. This is not a valid action for SigV4 (administrative API) clients.</p> <p>This action requires an authentication token. To get an authentication token, register an application with Amazon WorkDocs. For more information, see <a href="https://docs.aws.amazon.com/workdocs/latest/developerguide/wd-auth-user.html">Authentication and Access Control for User Applications</a> in the <i>Amazon WorkDocs Developer Guide</i>.</p>
    #[allow(unused_mut)]
    async fn get_current_user(
        &self,
        input: GetCurrentUserRequest,
    ) -> Result<GetCurrentUserResponse, RusotoError<GetCurrentUserError>> {
        let request_uri = "/api/v1/me";

        let mut request = SignedRequest::new("GET", "workdocs", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.add_header("Authentication", &input.authentication_token.to_string());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<GetCurrentUserResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(GetCurrentUserError::from_response(response))
        }
    }

    /// <p>Retrieves details of a document.</p>
    #[allow(unused_mut)]
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

        request.add_optional_header("Authentication", input.authentication_token.as_ref());
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
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<GetDocumentResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(GetDocumentError::from_response(response))
        }
    }

    /// <p>Retrieves the path information (the hierarchy from the root folder) for the requested document.</p> <p>By default, Amazon WorkDocs returns a maximum of 100 levels upwards from the requested document and only includes the IDs of the parent folders in the path. You can limit the maximum number of levels. You can also request the names of the parent folders.</p>
    #[allow(unused_mut)]
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

        request.add_optional_header("Authentication", input.authentication_token.as_ref());
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
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<GetDocumentPathResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(GetDocumentPathError::from_response(response))
        }
    }

    /// <p>Retrieves version metadata for the specified document.</p>
    #[allow(unused_mut)]
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

        request.add_optional_header("Authentication", input.authentication_token.as_ref());
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
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<GetDocumentVersionResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(GetDocumentVersionError::from_response(response))
        }
    }

    /// <p>Retrieves the metadata of the specified folder.</p>
    #[allow(unused_mut)]
    async fn get_folder(
        &self,
        input: GetFolderRequest,
    ) -> Result<GetFolderResponse, RusotoError<GetFolderError>> {
        let request_uri = format!("/api/v1/folders/{folder_id}", folder_id = input.folder_id);

        let mut request = SignedRequest::new("GET", "workdocs", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.add_optional_header("Authentication", input.authentication_token.as_ref());
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
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<GetFolderResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(GetFolderError::from_response(response))
        }
    }

    /// <p>Retrieves the path information (the hierarchy from the root folder) for the specified folder.</p> <p>By default, Amazon WorkDocs returns a maximum of 100 levels upwards from the requested folder and only includes the IDs of the parent folders in the path. You can limit the maximum number of levels. You can also request the parent folder names.</p>
    #[allow(unused_mut)]
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

        request.add_optional_header("Authentication", input.authentication_token.as_ref());
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
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<GetFolderPathResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(GetFolderPathError::from_response(response))
        }
    }

    /// <p>Retrieves a collection of resources, including folders and documents. The only <code>CollectionType</code> supported is <code>SHARED_WITH_ME</code>.</p>
    #[allow(unused_mut)]
    async fn get_resources(
        &self,
        input: GetResourcesRequest,
    ) -> Result<GetResourcesResponse, RusotoError<GetResourcesError>> {
        let request_uri = "/api/v1/resources";

        let mut request = SignedRequest::new("GET", "workdocs", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.add_optional_header("Authentication", input.authentication_token.as_ref());
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
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<GetResourcesResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(GetResourcesError::from_response(response))
        }
    }

    /// <p>Creates a new document object and version object.</p> <p>The client specifies the parent folder ID and name of the document to upload. The ID is optionally specified when creating a new version of an existing document. This is the first step to upload a document. Next, upload the document to the URL returned from the call, and then call <a>UpdateDocumentVersion</a>.</p> <p>To cancel the document upload, call <a>AbortDocumentVersionUpload</a>.</p>
    #[allow(unused_mut)]
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
        request.add_optional_header("Authentication", input.authentication_token.as_ref());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 201 {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<InitiateDocumentVersionUploadResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(InitiateDocumentVersionUploadError::from_response(response))
        }
    }

    /// <p>Removes all the permissions from the specified resource.</p>
    #[allow(unused_mut)]
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

        request.add_optional_header("Authentication", input.authentication_token.as_ref());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 204 {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = ::std::mem::drop(response);

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(RemoveAllResourcePermissionsError::from_response(response))
        }
    }

    /// <p>Removes the permission for the specified principal from the specified resource.</p>
    #[allow(unused_mut)]
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

        request.add_optional_header("Authentication", input.authentication_token.as_ref());
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
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = ::std::mem::drop(response);

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(RemoveResourcePermissionError::from_response(response))
        }
    }

    /// <p>Updates the specified attributes of a document. The user must have access to both the document and its parent folder, if applicable.</p>
    #[allow(unused_mut)]
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
        request.add_optional_header("Authentication", input.authentication_token.as_ref());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = ::std::mem::drop(response);

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(UpdateDocumentError::from_response(response))
        }
    }

    /// <p>Changes the status of the document version to ACTIVE. </p> <p>Amazon WorkDocs also sets its document container to ACTIVE. This is the last step in a document upload, after the client uploads the document to an S3-presigned URL returned by <a>InitiateDocumentVersionUpload</a>. </p>
    #[allow(unused_mut)]
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
        request.add_optional_header("Authentication", input.authentication_token.as_ref());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = ::std::mem::drop(response);

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(UpdateDocumentVersionError::from_response(response))
        }
    }

    /// <p>Updates the specified attributes of the specified folder. The user must have access to both the folder and its parent folder, if applicable.</p>
    #[allow(unused_mut)]
    async fn update_folder(
        &self,
        input: UpdateFolderRequest,
    ) -> Result<(), RusotoError<UpdateFolderError>> {
        let request_uri = format!("/api/v1/folders/{folder_id}", folder_id = input.folder_id);

        let mut request = SignedRequest::new("PATCH", "workdocs", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);
        request.add_optional_header("Authentication", input.authentication_token.as_ref());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = ::std::mem::drop(response);

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(UpdateFolderError::from_response(response))
        }
    }

    /// <p>Updates the specified attributes of the specified user, and grants or revokes administrative privileges to the Amazon WorkDocs site.</p>
    #[allow(unused_mut)]
    async fn update_user(
        &self,
        input: UpdateUserRequest,
    ) -> Result<UpdateUserResponse, RusotoError<UpdateUserError>> {
        let request_uri = format!("/api/v1/users/{user_id}", user_id = input.user_id);

        let mut request = SignedRequest::new("PATCH", "workdocs", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);
        request.add_optional_header("Authentication", input.authentication_token.as_ref());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<UpdateUserResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(UpdateUserError::from_response(response))
        }
    }
}
