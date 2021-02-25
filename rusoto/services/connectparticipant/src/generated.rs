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

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct UnknownArtifactStatus {
    name: String,
}

#[allow(non_camel_case_types)]
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
#[non_exhaustive]
pub enum ArtifactStatus {
    Approved,
    InProgress,
    Rejected,
    #[doc(hidden)]
    UnknownVariant(UnknownArtifactStatus),
}

impl Default for ArtifactStatus {
    fn default() -> Self {
        "".into()
    }
}

impl fmt::Display for ArtifactStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.into())
    }
}

impl rusoto_core::param::ToParam for ArtifactStatus {
    fn to_param(&self) -> String {
        self.to_string()
    }
}

impl Into<String> for ArtifactStatus {
    fn into(self) -> String {
        match self {
            ArtifactStatus::Approved => "APPROVED".to_string(),
            ArtifactStatus::InProgress => "IN_PROGRESS".to_string(),
            ArtifactStatus::Rejected => "REJECTED".to_string(),
            ArtifactStatus::UnknownVariant(UnknownArtifactStatus { name: original }) => original,
        }
    }
}

impl<'a> Into<&'a str> for &'a ArtifactStatus {
    fn into(self) -> &'a str {
        match self {
            ArtifactStatus::Approved => &"APPROVED",
            ArtifactStatus::InProgress => &"IN_PROGRESS",
            ArtifactStatus::Rejected => &"REJECTED",
            ArtifactStatus::UnknownVariant(UnknownArtifactStatus { name: original }) => original,
        }
    }
}

impl From<&str> for ArtifactStatus {
    fn from(name: &str) -> Self {
        match name {
            "APPROVED" => ArtifactStatus::Approved,
            "IN_PROGRESS" => ArtifactStatus::InProgress,
            "REJECTED" => ArtifactStatus::Rejected,
            _ => ArtifactStatus::UnknownVariant(UnknownArtifactStatus {
                name: name.to_owned(),
            }),
        }
    }
}

impl From<String> for ArtifactStatus {
    fn from(name: String) -> Self {
        match &*name {
            "APPROVED" => ArtifactStatus::Approved,
            "IN_PROGRESS" => ArtifactStatus::InProgress,
            "REJECTED" => ArtifactStatus::Rejected,
            _ => ArtifactStatus::UnknownVariant(UnknownArtifactStatus { name }),
        }
    }
}

impl ::std::str::FromStr for ArtifactStatus {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(s.into())
    }
}

#[cfg(any(test, feature = "serialize_structs"))]
impl Serialize for ArtifactStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.into())
    }
}

impl<'de> Deserialize<'de> for ArtifactStatus {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Ok(String::deserialize(deserializer)?.into())
    }
}

/// <p>The case-insensitive input to indicate standard MIME type that describes the format of the file that will be uploaded.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct AttachmentItem {
    /// <p>A unique identifier for the attachment.</p>
    #[serde(rename = "AttachmentId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attachment_id: Option<String>,
    /// <p>A case-sensitive name of the attachment being uploaded.</p>
    #[serde(rename = "AttachmentName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attachment_name: Option<String>,
    /// <p>Describes the MIME file type of the attachment. For a list of supported file types, see <a href="https://docs.aws.amazon.com/connect/latest/adminguide/amazon-connect-service-limits.html#feature-limits">Feature specifications</a> in the <i>Amazon Connect Administrator Guide</i>.</p>
    #[serde(rename = "ContentType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_type: Option<String>,
    /// <p>Status of the attachment.</p>
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<ArtifactStatus>,
}

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct UnknownChatItemType {
    name: String,
}

#[allow(non_camel_case_types)]
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
#[non_exhaustive]
pub enum ChatItemType {
    Attachment,
    ChatEnded,
    ConnectionAck,
    Event,
    Message,
    ParticipantJoined,
    ParticipantLeft,
    TransferFailed,
    TransferSucceeded,
    Typing,
    #[doc(hidden)]
    UnknownVariant(UnknownChatItemType),
}

impl Default for ChatItemType {
    fn default() -> Self {
        "".into()
    }
}

impl fmt::Display for ChatItemType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.into())
    }
}

impl rusoto_core::param::ToParam for ChatItemType {
    fn to_param(&self) -> String {
        self.to_string()
    }
}

impl Into<String> for ChatItemType {
    fn into(self) -> String {
        match self {
            ChatItemType::Attachment => "ATTACHMENT".to_string(),
            ChatItemType::ChatEnded => "CHAT_ENDED".to_string(),
            ChatItemType::ConnectionAck => "CONNECTION_ACK".to_string(),
            ChatItemType::Event => "EVENT".to_string(),
            ChatItemType::Message => "MESSAGE".to_string(),
            ChatItemType::ParticipantJoined => "PARTICIPANT_JOINED".to_string(),
            ChatItemType::ParticipantLeft => "PARTICIPANT_LEFT".to_string(),
            ChatItemType::TransferFailed => "TRANSFER_FAILED".to_string(),
            ChatItemType::TransferSucceeded => "TRANSFER_SUCCEEDED".to_string(),
            ChatItemType::Typing => "TYPING".to_string(),
            ChatItemType::UnknownVariant(UnknownChatItemType { name: original }) => original,
        }
    }
}

impl<'a> Into<&'a str> for &'a ChatItemType {
    fn into(self) -> &'a str {
        match self {
            ChatItemType::Attachment => &"ATTACHMENT",
            ChatItemType::ChatEnded => &"CHAT_ENDED",
            ChatItemType::ConnectionAck => &"CONNECTION_ACK",
            ChatItemType::Event => &"EVENT",
            ChatItemType::Message => &"MESSAGE",
            ChatItemType::ParticipantJoined => &"PARTICIPANT_JOINED",
            ChatItemType::ParticipantLeft => &"PARTICIPANT_LEFT",
            ChatItemType::TransferFailed => &"TRANSFER_FAILED",
            ChatItemType::TransferSucceeded => &"TRANSFER_SUCCEEDED",
            ChatItemType::Typing => &"TYPING",
            ChatItemType::UnknownVariant(UnknownChatItemType { name: original }) => original,
        }
    }
}

impl From<&str> for ChatItemType {
    fn from(name: &str) -> Self {
        match name {
            "ATTACHMENT" => ChatItemType::Attachment,
            "CHAT_ENDED" => ChatItemType::ChatEnded,
            "CONNECTION_ACK" => ChatItemType::ConnectionAck,
            "EVENT" => ChatItemType::Event,
            "MESSAGE" => ChatItemType::Message,
            "PARTICIPANT_JOINED" => ChatItemType::ParticipantJoined,
            "PARTICIPANT_LEFT" => ChatItemType::ParticipantLeft,
            "TRANSFER_FAILED" => ChatItemType::TransferFailed,
            "TRANSFER_SUCCEEDED" => ChatItemType::TransferSucceeded,
            "TYPING" => ChatItemType::Typing,
            _ => ChatItemType::UnknownVariant(UnknownChatItemType {
                name: name.to_owned(),
            }),
        }
    }
}

impl From<String> for ChatItemType {
    fn from(name: String) -> Self {
        match &*name {
            "ATTACHMENT" => ChatItemType::Attachment,
            "CHAT_ENDED" => ChatItemType::ChatEnded,
            "CONNECTION_ACK" => ChatItemType::ConnectionAck,
            "EVENT" => ChatItemType::Event,
            "MESSAGE" => ChatItemType::Message,
            "PARTICIPANT_JOINED" => ChatItemType::ParticipantJoined,
            "PARTICIPANT_LEFT" => ChatItemType::ParticipantLeft,
            "TRANSFER_FAILED" => ChatItemType::TransferFailed,
            "TRANSFER_SUCCEEDED" => ChatItemType::TransferSucceeded,
            "TYPING" => ChatItemType::Typing,
            _ => ChatItemType::UnknownVariant(UnknownChatItemType { name }),
        }
    }
}

impl ::std::str::FromStr for ChatItemType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(s.into())
    }
}

#[cfg(any(test, feature = "serialize_structs"))]
impl Serialize for ChatItemType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.into())
    }
}

impl<'de> Deserialize<'de> for ChatItemType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Ok(String::deserialize(deserializer)?.into())
    }
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CompleteAttachmentUploadRequest {
    /// <p>A list of unique identifiers for the attachments.</p>
    #[serde(rename = "AttachmentIds")]
    pub attachment_ids: Vec<String>,
    /// <p>A unique, case-sensitive identifier that you provide to ensure the idempotency of the request.</p>
    #[serde(rename = "ClientToken")]
    pub client_token: String,
    /// <p>The authentication token associated with the participant's connection.</p>
    #[serde(rename = "ConnectionToken")]
    pub connection_token: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CompleteAttachmentUploadResponse {}

/// <p>Connection credentials. </p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ConnectionCredentials {
    /// <p>The connection token.</p>
    #[serde(rename = "ConnectionToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connection_token: Option<String>,
    /// <p>The expiration of the token.</p> <p>It's specified in ISO 8601 format: yyyy-MM-ddThh:mm:ss.SSSZ. For example, 2019-11-08T02:41:28.172Z.</p>
    #[serde(rename = "Expiry")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expiry: Option<String>,
}

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct UnknownConnectionType {
    name: String,
}

#[allow(non_camel_case_types)]
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
#[non_exhaustive]
pub enum ConnectionType {
    ConnectionCredentials,
    Websocket,
    #[doc(hidden)]
    UnknownVariant(UnknownConnectionType),
}

impl Default for ConnectionType {
    fn default() -> Self {
        "".into()
    }
}

impl fmt::Display for ConnectionType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.into())
    }
}

impl rusoto_core::param::ToParam for ConnectionType {
    fn to_param(&self) -> String {
        self.to_string()
    }
}

impl Into<String> for ConnectionType {
    fn into(self) -> String {
        match self {
            ConnectionType::ConnectionCredentials => "CONNECTION_CREDENTIALS".to_string(),
            ConnectionType::Websocket => "WEBSOCKET".to_string(),
            ConnectionType::UnknownVariant(UnknownConnectionType { name: original }) => original,
        }
    }
}

impl<'a> Into<&'a str> for &'a ConnectionType {
    fn into(self) -> &'a str {
        match self {
            ConnectionType::ConnectionCredentials => &"CONNECTION_CREDENTIALS",
            ConnectionType::Websocket => &"WEBSOCKET",
            ConnectionType::UnknownVariant(UnknownConnectionType { name: original }) => original,
        }
    }
}

impl From<&str> for ConnectionType {
    fn from(name: &str) -> Self {
        match name {
            "CONNECTION_CREDENTIALS" => ConnectionType::ConnectionCredentials,
            "WEBSOCKET" => ConnectionType::Websocket,
            _ => ConnectionType::UnknownVariant(UnknownConnectionType {
                name: name.to_owned(),
            }),
        }
    }
}

impl From<String> for ConnectionType {
    fn from(name: String) -> Self {
        match &*name {
            "CONNECTION_CREDENTIALS" => ConnectionType::ConnectionCredentials,
            "WEBSOCKET" => ConnectionType::Websocket,
            _ => ConnectionType::UnknownVariant(UnknownConnectionType { name }),
        }
    }
}

impl ::std::str::FromStr for ConnectionType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(s.into())
    }
}

impl Serialize for ConnectionType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.into())
    }
}

#[cfg(feature = "deserialize_structs")]
impl<'de> Deserialize<'de> for ConnectionType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Ok(String::deserialize(deserializer)?.into())
    }
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateParticipantConnectionRequest {
    /// <p>This is a header parameter.</p> <p>The Participant Token as obtained from <a href="https://docs.aws.amazon.com/connect/latest/APIReference/API_StartChatContact.html">StartChatContact</a> API response.</p>
    #[serde(rename = "ParticipantToken")]
    pub participant_token: String,
    /// <p>Type of connection information required.</p>
    #[serde(rename = "Type")]
    pub type_: Vec<ConnectionType>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateParticipantConnectionResponse {
    /// <p>Creates the participant's connection credentials. The authentication token associated with the participant's connection.</p>
    #[serde(rename = "ConnectionCredentials")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connection_credentials: Option<ConnectionCredentials>,
    /// <p>Creates the participant's websocket connection.</p>
    #[serde(rename = "Websocket")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub websocket: Option<Websocket>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DisconnectParticipantRequest {
    /// <p>A unique, case-sensitive identifier that you provide to ensure the idempotency of the request.</p>
    #[serde(rename = "ClientToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
    /// <p>The authentication token associated with the participant's connection.</p>
    #[serde(rename = "ConnectionToken")]
    pub connection_token: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DisconnectParticipantResponse {}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetAttachmentRequest {
    /// <p>A unique identifier for the attachment.</p>
    #[serde(rename = "AttachmentId")]
    pub attachment_id: String,
    /// <p>The authentication token associated with the participant's connection.</p>
    #[serde(rename = "ConnectionToken")]
    pub connection_token: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetAttachmentResponse {
    /// <p>The pre-signed URL using which file would be downloaded from Amazon S3 by the API caller.</p>
    #[serde(rename = "Url")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    /// <p>The expiration time of the URL in ISO timestamp. It's specified in ISO 8601 format: yyyy-MM-ddThh:mm:ss.SSSZ. For example, 2019-11-08T02:41:28.172Z.</p>
    #[serde(rename = "UrlExpiry")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url_expiry: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetTranscriptRequest {
    /// <p>The authentication token associated with the participant's connection.</p>
    #[serde(rename = "ConnectionToken")]
    pub connection_token: String,
    /// <p>The contactId from the current contact chain for which transcript is needed.</p>
    #[serde(rename = "ContactId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contact_id: Option<String>,
    /// <p>The maximum number of results to return in the page. Default: 10. </p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>The pagination token. Use the value returned previously in the next subsequent request to retrieve the next set of results.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The direction from StartPosition from which to retrieve message. Default: BACKWARD when no StartPosition is provided, FORWARD with StartPosition. </p>
    #[serde(rename = "ScanDirection")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scan_direction: Option<ScanDirection>,
    /// <p>The sort order for the records. Default: DESCENDING.</p>
    #[serde(rename = "SortOrder")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_order: Option<SortKey>,
    /// <p>A filtering option for where to start.</p>
    #[serde(rename = "StartPosition")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_position: Option<StartPosition>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetTranscriptResponse {
    /// <p>The initial contact ID for the contact. </p>
    #[serde(rename = "InitialContactId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub initial_contact_id: Option<String>,
    /// <p>The pagination token. Use the value returned previously in the next subsequent request to retrieve the next set of results.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The list of messages in the session.</p>
    #[serde(rename = "Transcript")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transcript: Option<Vec<Item>>,
}

/// <p>An item - message or event - that has been sent. </p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct Item {
    /// <p>The time when the message or event was sent.</p> <p>It's specified in ISO 8601 format: yyyy-MM-ddThh:mm:ss.SSSZ. For example, 2019-11-08T02:41:28.172Z.</p>
    #[serde(rename = "AbsoluteTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub absolute_time: Option<String>,
    /// <p>Provides information about the attachments.</p>
    #[serde(rename = "Attachments")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attachments: Option<Vec<AttachmentItem>>,
    /// <p>The content of the message or event.</p>
    #[serde(rename = "Content")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
    /// <p>The type of content of the item.</p>
    #[serde(rename = "ContentType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_type: Option<String>,
    /// <p>The chat display name of the sender.</p>
    #[serde(rename = "DisplayName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    /// <p>The ID of the item.</p>
    #[serde(rename = "Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// <p>The ID of the sender in the session.</p>
    #[serde(rename = "ParticipantId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub participant_id: Option<String>,
    /// <p>The role of the sender. For example, is it a customer, agent, or system.</p>
    #[serde(rename = "ParticipantRole")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub participant_role: Option<ParticipantRole>,
    /// <p>Type of the item: message or event. </p>
    #[serde(rename = "Type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<ChatItemType>,
}

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct UnknownParticipantRole {
    name: String,
}

#[allow(non_camel_case_types)]
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
#[non_exhaustive]
pub enum ParticipantRole {
    Agent,
    Customer,
    System,
    #[doc(hidden)]
    UnknownVariant(UnknownParticipantRole),
}

impl Default for ParticipantRole {
    fn default() -> Self {
        "".into()
    }
}

impl fmt::Display for ParticipantRole {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.into())
    }
}

impl rusoto_core::param::ToParam for ParticipantRole {
    fn to_param(&self) -> String {
        self.to_string()
    }
}

impl Into<String> for ParticipantRole {
    fn into(self) -> String {
        match self {
            ParticipantRole::Agent => "AGENT".to_string(),
            ParticipantRole::Customer => "CUSTOMER".to_string(),
            ParticipantRole::System => "SYSTEM".to_string(),
            ParticipantRole::UnknownVariant(UnknownParticipantRole { name: original }) => original,
        }
    }
}

impl<'a> Into<&'a str> for &'a ParticipantRole {
    fn into(self) -> &'a str {
        match self {
            ParticipantRole::Agent => &"AGENT",
            ParticipantRole::Customer => &"CUSTOMER",
            ParticipantRole::System => &"SYSTEM",
            ParticipantRole::UnknownVariant(UnknownParticipantRole { name: original }) => original,
        }
    }
}

impl From<&str> for ParticipantRole {
    fn from(name: &str) -> Self {
        match name {
            "AGENT" => ParticipantRole::Agent,
            "CUSTOMER" => ParticipantRole::Customer,
            "SYSTEM" => ParticipantRole::System,
            _ => ParticipantRole::UnknownVariant(UnknownParticipantRole {
                name: name.to_owned(),
            }),
        }
    }
}

impl From<String> for ParticipantRole {
    fn from(name: String) -> Self {
        match &*name {
            "AGENT" => ParticipantRole::Agent,
            "CUSTOMER" => ParticipantRole::Customer,
            "SYSTEM" => ParticipantRole::System,
            _ => ParticipantRole::UnknownVariant(UnknownParticipantRole { name }),
        }
    }
}

impl ::std::str::FromStr for ParticipantRole {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(s.into())
    }
}

#[cfg(any(test, feature = "serialize_structs"))]
impl Serialize for ParticipantRole {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.into())
    }
}

impl<'de> Deserialize<'de> for ParticipantRole {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Ok(String::deserialize(deserializer)?.into())
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct UnknownScanDirection {
    name: String,
}

#[allow(non_camel_case_types)]
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
#[non_exhaustive]
pub enum ScanDirection {
    Backward,
    Forward,
    #[doc(hidden)]
    UnknownVariant(UnknownScanDirection),
}

impl Default for ScanDirection {
    fn default() -> Self {
        "".into()
    }
}

impl fmt::Display for ScanDirection {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.into())
    }
}

impl rusoto_core::param::ToParam for ScanDirection {
    fn to_param(&self) -> String {
        self.to_string()
    }
}

impl Into<String> for ScanDirection {
    fn into(self) -> String {
        match self {
            ScanDirection::Backward => "BACKWARD".to_string(),
            ScanDirection::Forward => "FORWARD".to_string(),
            ScanDirection::UnknownVariant(UnknownScanDirection { name: original }) => original,
        }
    }
}

impl<'a> Into<&'a str> for &'a ScanDirection {
    fn into(self) -> &'a str {
        match self {
            ScanDirection::Backward => &"BACKWARD",
            ScanDirection::Forward => &"FORWARD",
            ScanDirection::UnknownVariant(UnknownScanDirection { name: original }) => original,
        }
    }
}

impl From<&str> for ScanDirection {
    fn from(name: &str) -> Self {
        match name {
            "BACKWARD" => ScanDirection::Backward,
            "FORWARD" => ScanDirection::Forward,
            _ => ScanDirection::UnknownVariant(UnknownScanDirection {
                name: name.to_owned(),
            }),
        }
    }
}

impl From<String> for ScanDirection {
    fn from(name: String) -> Self {
        match &*name {
            "BACKWARD" => ScanDirection::Backward,
            "FORWARD" => ScanDirection::Forward,
            _ => ScanDirection::UnknownVariant(UnknownScanDirection { name }),
        }
    }
}

impl ::std::str::FromStr for ScanDirection {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(s.into())
    }
}

impl Serialize for ScanDirection {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.into())
    }
}

#[cfg(feature = "deserialize_structs")]
impl<'de> Deserialize<'de> for ScanDirection {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Ok(String::deserialize(deserializer)?.into())
    }
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct SendEventRequest {
    /// <p>A unique, case-sensitive identifier that you provide to ensure the idempotency of the request.</p>
    #[serde(rename = "ClientToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
    /// <p>The authentication token associated with the participant's connection.</p>
    #[serde(rename = "ConnectionToken")]
    pub connection_token: String,
    /// <p>The content of the event to be sent (for example, message text). This is not yet supported.</p>
    #[serde(rename = "Content")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
    /// <p><p>The content type of the request. Supported types are:</p> <ul> <li> <p>application/vnd.amazonaws.connect.event.typing</p> </li> <li> <p>application/vnd.amazonaws.connect.event.connection.acknowledged</p> </li> </ul></p>
    #[serde(rename = "ContentType")]
    pub content_type: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct SendEventResponse {
    /// <p>The time when the event was sent.</p> <p>It's specified in ISO 8601 format: yyyy-MM-ddThh:mm:ss.SSSZ. For example, 2019-11-08T02:41:28.172Z.</p>
    #[serde(rename = "AbsoluteTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub absolute_time: Option<String>,
    /// <p>The ID of the response.</p>
    #[serde(rename = "Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct SendMessageRequest {
    /// <p>A unique, case-sensitive identifier that you provide to ensure the idempotency of the request.</p>
    #[serde(rename = "ClientToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
    /// <p>The authentication token associated with the connection.</p>
    #[serde(rename = "ConnectionToken")]
    pub connection_token: String,
    /// <p>The content of the message.</p>
    #[serde(rename = "Content")]
    pub content: String,
    /// <p>The type of the content. Supported types are text/plain.</p>
    #[serde(rename = "ContentType")]
    pub content_type: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct SendMessageResponse {
    /// <p>The time when the message was sent.</p> <p>It's specified in ISO 8601 format: yyyy-MM-ddThh:mm:ss.SSSZ. For example, 2019-11-08T02:41:28.172Z.</p>
    #[serde(rename = "AbsoluteTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub absolute_time: Option<String>,
    /// <p>The ID of the message.</p>
    #[serde(rename = "Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct UnknownSortKey {
    name: String,
}

#[allow(non_camel_case_types)]
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
#[non_exhaustive]
pub enum SortKey {
    Ascending,
    Descending,
    #[doc(hidden)]
    UnknownVariant(UnknownSortKey),
}

impl Default for SortKey {
    fn default() -> Self {
        "".into()
    }
}

impl fmt::Display for SortKey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.into())
    }
}

impl rusoto_core::param::ToParam for SortKey {
    fn to_param(&self) -> String {
        self.to_string()
    }
}

impl Into<String> for SortKey {
    fn into(self) -> String {
        match self {
            SortKey::Ascending => "ASCENDING".to_string(),
            SortKey::Descending => "DESCENDING".to_string(),
            SortKey::UnknownVariant(UnknownSortKey { name: original }) => original,
        }
    }
}

impl<'a> Into<&'a str> for &'a SortKey {
    fn into(self) -> &'a str {
        match self {
            SortKey::Ascending => &"ASCENDING",
            SortKey::Descending => &"DESCENDING",
            SortKey::UnknownVariant(UnknownSortKey { name: original }) => original,
        }
    }
}

impl From<&str> for SortKey {
    fn from(name: &str) -> Self {
        match name {
            "ASCENDING" => SortKey::Ascending,
            "DESCENDING" => SortKey::Descending,
            _ => SortKey::UnknownVariant(UnknownSortKey {
                name: name.to_owned(),
            }),
        }
    }
}

impl From<String> for SortKey {
    fn from(name: String) -> Self {
        match &*name {
            "ASCENDING" => SortKey::Ascending,
            "DESCENDING" => SortKey::Descending,
            _ => SortKey::UnknownVariant(UnknownSortKey { name }),
        }
    }
}

impl ::std::str::FromStr for SortKey {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(s.into())
    }
}

impl Serialize for SortKey {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.into())
    }
}

#[cfg(feature = "deserialize_structs")]
impl<'de> Deserialize<'de> for SortKey {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Ok(String::deserialize(deserializer)?.into())
    }
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct StartAttachmentUploadRequest {
    /// <p>A case-sensitive name of the attachment being uploaded.</p>
    #[serde(rename = "AttachmentName")]
    pub attachment_name: String,
    /// <p>The size of the attachment in bytes.</p>
    #[serde(rename = "AttachmentSizeInBytes")]
    pub attachment_size_in_bytes: i64,
    /// <p>A unique case sensitive identifier to support idempotency of request.</p>
    #[serde(rename = "ClientToken")]
    pub client_token: String,
    /// <p>The authentication token associated with the participant's connection.</p>
    #[serde(rename = "ConnectionToken")]
    pub connection_token: String,
    /// <p>Describes the MIME file type of the attachment. For a list of supported file types, see <a href="https://docs.aws.amazon.com/connect/latest/adminguide/amazon-connect-service-limits.html#feature-limits">Feature specifications</a> in the <i>Amazon Connect Administrator Guide</i>.</p>
    #[serde(rename = "ContentType")]
    pub content_type: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct StartAttachmentUploadResponse {
    /// <p>A unique identifier for the attachment.</p>
    #[serde(rename = "AttachmentId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attachment_id: Option<String>,
    /// <p>Fields to be used while uploading the attachment.</p>
    #[serde(rename = "UploadMetadata")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub upload_metadata: Option<UploadMetadata>,
}

/// <p>A filtering option for where to start. For example, if you sent 100 messages, start with message 50. </p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct StartPosition {
    /// <p>The time in ISO format where to start.</p> <p>It's specified in ISO 8601 format: yyyy-MM-ddThh:mm:ss.SSSZ. For example, 2019-11-08T02:41:28.172Z.</p>
    #[serde(rename = "AbsoluteTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub absolute_time: Option<String>,
    /// <p>The ID of the message or event where to start. </p>
    #[serde(rename = "Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// <p>The start position of the most recent message where you want to start. </p>
    #[serde(rename = "MostRecent")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub most_recent: Option<i64>,
}

/// <p>Fields to be used while uploading the attachment.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UploadMetadata {
    /// <p>The headers to be provided while uploading the file to the URL.</p>
    #[serde(rename = "HeadersToInclude")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub headers_to_include: Option<::std::collections::HashMap<String, String>>,
    /// <p>The pre-signed URL using which file would be downloaded from Amazon S3 by the API caller.</p>
    #[serde(rename = "Url")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    /// <p>The expiration time of the URL in ISO timestamp. It's specified in ISO 8601 format: yyyy-MM-ddThh:mm:ss.SSSZ. For example, 2019-11-08T02:41:28.172Z.</p>
    #[serde(rename = "UrlExpiry")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url_expiry: Option<String>,
}

/// <p>The websocket for the participant's connection.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct Websocket {
    /// <p>The URL expiration timestamp in ISO date format.</p> <p>It's specified in ISO 8601 format: yyyy-MM-ddThh:mm:ss.SSSZ. For example, 2019-11-08T02:41:28.172Z.</p>
    #[serde(rename = "ConnectionExpiry")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connection_expiry: Option<String>,
    /// <p>The URL of the websocket.</p>
    #[serde(rename = "Url")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

/// Errors returned by CompleteAttachmentUpload
#[derive(Debug, PartialEq)]
pub enum CompleteAttachmentUploadError {
    /// <p>You do not have sufficient access to perform this action.</p>
    AccessDenied(String),
    /// <p>An attachment with that identifier is already being uploaded.</p>
    Conflict(String),
    /// <p>This exception occurs when there is an internal failure in the Amazon Connect service.</p>
    InternalServer(String),
    /// <p>The number of attachments per contact exceeds the quota.</p>
    ServiceQuotaExceeded(String),
    /// <p>The request was denied due to request throttling.</p>
    Throttling(String),
}

impl CompleteAttachmentUploadError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CompleteAttachmentUploadError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(CompleteAttachmentUploadError::AccessDenied(
                        err.msg,
                    ))
                }
                "ConflictException" => {
                    return RusotoError::Service(CompleteAttachmentUploadError::Conflict(err.msg))
                }
                "InternalServerException" => {
                    return RusotoError::Service(CompleteAttachmentUploadError::InternalServer(
                        err.msg,
                    ))
                }
                "ServiceQuotaExceededException" => {
                    return RusotoError::Service(
                        CompleteAttachmentUploadError::ServiceQuotaExceeded(err.msg),
                    )
                }
                "ThrottlingException" => {
                    return RusotoError::Service(CompleteAttachmentUploadError::Throttling(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for CompleteAttachmentUploadError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CompleteAttachmentUploadError::AccessDenied(ref cause) => write!(f, "{}", cause),
            CompleteAttachmentUploadError::Conflict(ref cause) => write!(f, "{}", cause),
            CompleteAttachmentUploadError::InternalServer(ref cause) => write!(f, "{}", cause),
            CompleteAttachmentUploadError::ServiceQuotaExceeded(ref cause) => {
                write!(f, "{}", cause)
            }
            CompleteAttachmentUploadError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CompleteAttachmentUploadError {}
/// Errors returned by CreateParticipantConnection
#[derive(Debug, PartialEq)]
pub enum CreateParticipantConnectionError {
    /// <p>You do not have sufficient access to perform this action.</p>
    AccessDenied(String),
    /// <p>This exception occurs when there is an internal failure in the Amazon Connect service.</p>
    InternalServer(String),
    /// <p>The request was denied due to request throttling.</p>
    Throttling(String),
}

impl CreateParticipantConnectionError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<CreateParticipantConnectionError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(CreateParticipantConnectionError::AccessDenied(
                        err.msg,
                    ))
                }
                "InternalServerException" => {
                    return RusotoError::Service(CreateParticipantConnectionError::InternalServer(
                        err.msg,
                    ))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(CreateParticipantConnectionError::Throttling(
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
impl fmt::Display for CreateParticipantConnectionError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreateParticipantConnectionError::AccessDenied(ref cause) => write!(f, "{}", cause),
            CreateParticipantConnectionError::InternalServer(ref cause) => write!(f, "{}", cause),
            CreateParticipantConnectionError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CreateParticipantConnectionError {}
/// Errors returned by DisconnectParticipant
#[derive(Debug, PartialEq)]
pub enum DisconnectParticipantError {
    /// <p>You do not have sufficient access to perform this action.</p>
    AccessDenied(String),
    /// <p>This exception occurs when there is an internal failure in the Amazon Connect service.</p>
    InternalServer(String),
    /// <p>The request was denied due to request throttling.</p>
    Throttling(String),
}

impl DisconnectParticipantError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DisconnectParticipantError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(DisconnectParticipantError::AccessDenied(err.msg))
                }
                "InternalServerException" => {
                    return RusotoError::Service(DisconnectParticipantError::InternalServer(
                        err.msg,
                    ))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(DisconnectParticipantError::Throttling(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DisconnectParticipantError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DisconnectParticipantError::AccessDenied(ref cause) => write!(f, "{}", cause),
            DisconnectParticipantError::InternalServer(ref cause) => write!(f, "{}", cause),
            DisconnectParticipantError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DisconnectParticipantError {}
/// Errors returned by GetAttachment
#[derive(Debug, PartialEq)]
pub enum GetAttachmentError {
    /// <p>You do not have sufficient access to perform this action.</p>
    AccessDenied(String),
    /// <p>This exception occurs when there is an internal failure in the Amazon Connect service.</p>
    InternalServer(String),
    /// <p>The request was denied due to request throttling.</p>
    Throttling(String),
}

impl GetAttachmentError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetAttachmentError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(GetAttachmentError::AccessDenied(err.msg))
                }
                "InternalServerException" => {
                    return RusotoError::Service(GetAttachmentError::InternalServer(err.msg))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(GetAttachmentError::Throttling(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for GetAttachmentError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetAttachmentError::AccessDenied(ref cause) => write!(f, "{}", cause),
            GetAttachmentError::InternalServer(ref cause) => write!(f, "{}", cause),
            GetAttachmentError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetAttachmentError {}
/// Errors returned by GetTranscript
#[derive(Debug, PartialEq)]
pub enum GetTranscriptError {
    /// <p>You do not have sufficient access to perform this action.</p>
    AccessDenied(String),
    /// <p>This exception occurs when there is an internal failure in the Amazon Connect service.</p>
    InternalServer(String),
    /// <p>The request was denied due to request throttling.</p>
    Throttling(String),
}

impl GetTranscriptError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetTranscriptError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(GetTranscriptError::AccessDenied(err.msg))
                }
                "InternalServerException" => {
                    return RusotoError::Service(GetTranscriptError::InternalServer(err.msg))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(GetTranscriptError::Throttling(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for GetTranscriptError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetTranscriptError::AccessDenied(ref cause) => write!(f, "{}", cause),
            GetTranscriptError::InternalServer(ref cause) => write!(f, "{}", cause),
            GetTranscriptError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetTranscriptError {}
/// Errors returned by SendEvent
#[derive(Debug, PartialEq)]
pub enum SendEventError {
    /// <p>You do not have sufficient access to perform this action.</p>
    AccessDenied(String),
    /// <p>This exception occurs when there is an internal failure in the Amazon Connect service.</p>
    InternalServer(String),
    /// <p>The request was denied due to request throttling.</p>
    Throttling(String),
}

impl SendEventError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<SendEventError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(SendEventError::AccessDenied(err.msg))
                }
                "InternalServerException" => {
                    return RusotoError::Service(SendEventError::InternalServer(err.msg))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(SendEventError::Throttling(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for SendEventError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            SendEventError::AccessDenied(ref cause) => write!(f, "{}", cause),
            SendEventError::InternalServer(ref cause) => write!(f, "{}", cause),
            SendEventError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for SendEventError {}
/// Errors returned by SendMessage
#[derive(Debug, PartialEq)]
pub enum SendMessageError {
    /// <p>You do not have sufficient access to perform this action.</p>
    AccessDenied(String),
    /// <p>This exception occurs when there is an internal failure in the Amazon Connect service.</p>
    InternalServer(String),
    /// <p>The request was denied due to request throttling.</p>
    Throttling(String),
}

impl SendMessageError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<SendMessageError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(SendMessageError::AccessDenied(err.msg))
                }
                "InternalServerException" => {
                    return RusotoError::Service(SendMessageError::InternalServer(err.msg))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(SendMessageError::Throttling(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for SendMessageError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            SendMessageError::AccessDenied(ref cause) => write!(f, "{}", cause),
            SendMessageError::InternalServer(ref cause) => write!(f, "{}", cause),
            SendMessageError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for SendMessageError {}
/// Errors returned by StartAttachmentUpload
#[derive(Debug, PartialEq)]
pub enum StartAttachmentUploadError {
    /// <p>You do not have sufficient access to perform this action.</p>
    AccessDenied(String),
    /// <p>This exception occurs when there is an internal failure in the Amazon Connect service.</p>
    InternalServer(String),
    /// <p>The number of attachments per contact exceeds the quota.</p>
    ServiceQuotaExceeded(String),
    /// <p>The request was denied due to request throttling.</p>
    Throttling(String),
}

impl StartAttachmentUploadError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<StartAttachmentUploadError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(StartAttachmentUploadError::AccessDenied(err.msg))
                }
                "InternalServerException" => {
                    return RusotoError::Service(StartAttachmentUploadError::InternalServer(
                        err.msg,
                    ))
                }
                "ServiceQuotaExceededException" => {
                    return RusotoError::Service(StartAttachmentUploadError::ServiceQuotaExceeded(
                        err.msg,
                    ))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(StartAttachmentUploadError::Throttling(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for StartAttachmentUploadError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            StartAttachmentUploadError::AccessDenied(ref cause) => write!(f, "{}", cause),
            StartAttachmentUploadError::InternalServer(ref cause) => write!(f, "{}", cause),
            StartAttachmentUploadError::ServiceQuotaExceeded(ref cause) => write!(f, "{}", cause),
            StartAttachmentUploadError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for StartAttachmentUploadError {}
/// Trait representing the capabilities of the Amazon Connect Participant API. Amazon Connect Participant clients implement this trait.
#[async_trait]
pub trait ConnectParticipant {
    /// <p>Allows you to confirm that the attachment has been uploaded using the pre-signed URL provided in StartAttachmentUpload API. </p>
    async fn complete_attachment_upload(
        &self,
        input: CompleteAttachmentUploadRequest,
    ) -> Result<CompleteAttachmentUploadResponse, RusotoError<CompleteAttachmentUploadError>>;

    /// <p><p>Creates the participant&#39;s connection. Note that ParticipantToken is used for invoking this API instead of ConnectionToken.</p> <p>The participant token is valid for the lifetime of the participant – until they are part of a contact.</p> <p>The response URL for <code>WEBSOCKET</code> Type has a connect expiry timeout of 100s. Clients must manually connect to the returned websocket URL and subscribe to the desired topic. </p> <p>For chat, you need to publish the following on the established websocket connection:</p> <p> <code>{&quot;topic&quot;:&quot;aws/subscribe&quot;,&quot;content&quot;:{&quot;topics&quot;:[&quot;aws/chat&quot;]}}</code> </p> <p>Upon websocket URL expiry, as specified in the response ConnectionExpiry parameter, clients need to call this API again to obtain a new websocket URL and perform the same steps as before.</p> <note> <p>The Amazon Connect Participant Service APIs do not use <a href="https://docs.aws.amazon.com/general/latest/gr/signature-version-4.html">Signature Version 4 authentication</a>.</p> </note></p>
    async fn create_participant_connection(
        &self,
        input: CreateParticipantConnectionRequest,
    ) -> Result<CreateParticipantConnectionResponse, RusotoError<CreateParticipantConnectionError>>;

    /// <p>Disconnects a participant. Note that ConnectionToken is used for invoking this API instead of ParticipantToken.</p> <p>The Amazon Connect Participant Service APIs do not use <a href="https://docs.aws.amazon.com/general/latest/gr/signature-version-4.html">Signature Version 4 authentication</a>.</p>
    async fn disconnect_participant(
        &self,
        input: DisconnectParticipantRequest,
    ) -> Result<DisconnectParticipantResponse, RusotoError<DisconnectParticipantError>>;

    /// <p>Provides a pre-signed URL for download of a completed attachment. This is an asynchronous API for use with active contacts.</p>
    async fn get_attachment(
        &self,
        input: GetAttachmentRequest,
    ) -> Result<GetAttachmentResponse, RusotoError<GetAttachmentError>>;

    /// <p>Retrieves a transcript of the session, including details about any attachments. Note that ConnectionToken is used for invoking this API instead of ParticipantToken.</p> <p>The Amazon Connect Participant Service APIs do not use <a href="https://docs.aws.amazon.com/general/latest/gr/signature-version-4.html">Signature Version 4 authentication</a>.</p>
    async fn get_transcript(
        &self,
        input: GetTranscriptRequest,
    ) -> Result<GetTranscriptResponse, RusotoError<GetTranscriptError>>;

    /// <p>Sends an event. Note that ConnectionToken is used for invoking this API instead of ParticipantToken.</p> <p>The Amazon Connect Participant Service APIs do not use <a href="https://docs.aws.amazon.com/general/latest/gr/signature-version-4.html">Signature Version 4 authentication</a>.</p>
    async fn send_event(
        &self,
        input: SendEventRequest,
    ) -> Result<SendEventResponse, RusotoError<SendEventError>>;

    /// <p><p>Sends a message. Note that ConnectionToken is used for invoking this API instead of ParticipantToken.</p> <note> <p>The Amazon Connect Participant Service APIs do not use <a href="https://docs.aws.amazon.com/general/latest/gr/signature-version-4.html">Signature Version 4 authentication</a>.</p> </note></p>
    async fn send_message(
        &self,
        input: SendMessageRequest,
    ) -> Result<SendMessageResponse, RusotoError<SendMessageError>>;

    /// <p>Provides a pre-signed Amazon S3 URL in response for uploading the file directly to S3.</p>
    async fn start_attachment_upload(
        &self,
        input: StartAttachmentUploadRequest,
    ) -> Result<StartAttachmentUploadResponse, RusotoError<StartAttachmentUploadError>>;
}
/// A client for the Amazon Connect Participant API.
#[derive(Clone)]
pub struct ConnectParticipantClient {
    client: Client,
    region: region::Region,
}

impl ConnectParticipantClient {
    /// Creates a client backed by the default tokio event loop.
    ///
    /// The client will use the default credentials provider and tls client.
    pub fn new(region: region::Region) -> ConnectParticipantClient {
        ConnectParticipantClient {
            client: Client::shared(),
            region,
        }
    }

    pub fn new_with<P, D>(
        request_dispatcher: D,
        credentials_provider: P,
        region: region::Region,
    ) -> ConnectParticipantClient
    where
        P: ProvideAwsCredentials + Send + Sync + 'static,
        D: DispatchSignedRequest + Send + Sync + 'static,
    {
        ConnectParticipantClient {
            client: Client::new_with(credentials_provider, request_dispatcher),
            region,
        }
    }

    pub fn new_with_client(client: Client, region: region::Region) -> ConnectParticipantClient {
        ConnectParticipantClient { client, region }
    }
}

#[async_trait]
impl ConnectParticipant for ConnectParticipantClient {
    /// <p>Allows you to confirm that the attachment has been uploaded using the pre-signed URL provided in StartAttachmentUpload API. </p>
    #[allow(unused_mut)]
    async fn complete_attachment_upload(
        &self,
        input: CompleteAttachmentUploadRequest,
    ) -> Result<CompleteAttachmentUploadResponse, RusotoError<CompleteAttachmentUploadError>> {
        let request_uri = "/participant/complete-attachment-upload";

        let mut request = SignedRequest::new("POST", "execute-api", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("participant.connect".to_string());
        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);
        request.add_header("X-Amz-Bearer", &input.connection_token.to_string());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<CompleteAttachmentUploadResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(CompleteAttachmentUploadError::from_response(response))
        }
    }

    /// <p><p>Creates the participant&#39;s connection. Note that ParticipantToken is used for invoking this API instead of ConnectionToken.</p> <p>The participant token is valid for the lifetime of the participant – until they are part of a contact.</p> <p>The response URL for <code>WEBSOCKET</code> Type has a connect expiry timeout of 100s. Clients must manually connect to the returned websocket URL and subscribe to the desired topic. </p> <p>For chat, you need to publish the following on the established websocket connection:</p> <p> <code>{&quot;topic&quot;:&quot;aws/subscribe&quot;,&quot;content&quot;:{&quot;topics&quot;:[&quot;aws/chat&quot;]}}</code> </p> <p>Upon websocket URL expiry, as specified in the response ConnectionExpiry parameter, clients need to call this API again to obtain a new websocket URL and perform the same steps as before.</p> <note> <p>The Amazon Connect Participant Service APIs do not use <a href="https://docs.aws.amazon.com/general/latest/gr/signature-version-4.html">Signature Version 4 authentication</a>.</p> </note></p>
    #[allow(unused_mut)]
    async fn create_participant_connection(
        &self,
        input: CreateParticipantConnectionRequest,
    ) -> Result<CreateParticipantConnectionResponse, RusotoError<CreateParticipantConnectionError>>
    {
        let request_uri = "/participant/connection";

        let mut request = SignedRequest::new("POST", "execute-api", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("participant.connect".to_string());
        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);
        request.add_header("X-Amz-Bearer", &input.participant_token.to_string());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<CreateParticipantConnectionResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(CreateParticipantConnectionError::from_response(response))
        }
    }

    /// <p>Disconnects a participant. Note that ConnectionToken is used for invoking this API instead of ParticipantToken.</p> <p>The Amazon Connect Participant Service APIs do not use <a href="https://docs.aws.amazon.com/general/latest/gr/signature-version-4.html">Signature Version 4 authentication</a>.</p>
    #[allow(unused_mut)]
    async fn disconnect_participant(
        &self,
        input: DisconnectParticipantRequest,
    ) -> Result<DisconnectParticipantResponse, RusotoError<DisconnectParticipantError>> {
        let request_uri = "/participant/disconnect";

        let mut request = SignedRequest::new("POST", "execute-api", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("participant.connect".to_string());
        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);
        request.add_header("X-Amz-Bearer", &input.connection_token.to_string());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<DisconnectParticipantResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(DisconnectParticipantError::from_response(response))
        }
    }

    /// <p>Provides a pre-signed URL for download of a completed attachment. This is an asynchronous API for use with active contacts.</p>
    #[allow(unused_mut)]
    async fn get_attachment(
        &self,
        input: GetAttachmentRequest,
    ) -> Result<GetAttachmentResponse, RusotoError<GetAttachmentError>> {
        let request_uri = "/participant/attachment";

        let mut request = SignedRequest::new("POST", "execute-api", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("participant.connect".to_string());
        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);
        request.add_header("X-Amz-Bearer", &input.connection_token.to_string());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<GetAttachmentResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(GetAttachmentError::from_response(response))
        }
    }

    /// <p>Retrieves a transcript of the session, including details about any attachments. Note that ConnectionToken is used for invoking this API instead of ParticipantToken.</p> <p>The Amazon Connect Participant Service APIs do not use <a href="https://docs.aws.amazon.com/general/latest/gr/signature-version-4.html">Signature Version 4 authentication</a>.</p>
    #[allow(unused_mut)]
    async fn get_transcript(
        &self,
        input: GetTranscriptRequest,
    ) -> Result<GetTranscriptResponse, RusotoError<GetTranscriptError>> {
        let request_uri = "/participant/transcript";

        let mut request = SignedRequest::new("POST", "execute-api", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("participant.connect".to_string());
        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);
        request.add_header("X-Amz-Bearer", &input.connection_token.to_string());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<GetTranscriptResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(GetTranscriptError::from_response(response))
        }
    }

    /// <p>Sends an event. Note that ConnectionToken is used for invoking this API instead of ParticipantToken.</p> <p>The Amazon Connect Participant Service APIs do not use <a href="https://docs.aws.amazon.com/general/latest/gr/signature-version-4.html">Signature Version 4 authentication</a>.</p>
    #[allow(unused_mut)]
    async fn send_event(
        &self,
        input: SendEventRequest,
    ) -> Result<SendEventResponse, RusotoError<SendEventError>> {
        let request_uri = "/participant/event";

        let mut request = SignedRequest::new("POST", "execute-api", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("participant.connect".to_string());
        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);
        request.add_header("X-Amz-Bearer", &input.connection_token.to_string());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<SendEventResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(SendEventError::from_response(response))
        }
    }

    /// <p><p>Sends a message. Note that ConnectionToken is used for invoking this API instead of ParticipantToken.</p> <note> <p>The Amazon Connect Participant Service APIs do not use <a href="https://docs.aws.amazon.com/general/latest/gr/signature-version-4.html">Signature Version 4 authentication</a>.</p> </note></p>
    #[allow(unused_mut)]
    async fn send_message(
        &self,
        input: SendMessageRequest,
    ) -> Result<SendMessageResponse, RusotoError<SendMessageError>> {
        let request_uri = "/participant/message";

        let mut request = SignedRequest::new("POST", "execute-api", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("participant.connect".to_string());
        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);
        request.add_header("X-Amz-Bearer", &input.connection_token.to_string());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<SendMessageResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(SendMessageError::from_response(response))
        }
    }

    /// <p>Provides a pre-signed Amazon S3 URL in response for uploading the file directly to S3.</p>
    #[allow(unused_mut)]
    async fn start_attachment_upload(
        &self,
        input: StartAttachmentUploadRequest,
    ) -> Result<StartAttachmentUploadResponse, RusotoError<StartAttachmentUploadError>> {
        let request_uri = "/participant/start-attachment-upload";

        let mut request = SignedRequest::new("POST", "execute-api", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("participant.connect".to_string());
        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);
        request.add_header("X-Amz-Bearer", &input.connection_token.to_string());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<StartAttachmentUploadResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(StartAttachmentUploadError::from_response(response))
        }
    }
}
