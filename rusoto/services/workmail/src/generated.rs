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
pub struct AssociateDelegateToResourceRequest {
    /// <p>The member (user or group) to associate to the resource.</p>
    #[serde(rename = "EntityId")]
    pub entity_id: String,
    /// <p>The organization under which the resource exists.</p>
    #[serde(rename = "OrganizationId")]
    pub organization_id: String,
    /// <p>The resource for which members are associated.</p>
    #[serde(rename = "ResourceId")]
    pub resource_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct AssociateDelegateToResourceResponse {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct AssociateMemberToGroupRequest {
    /// <p>The group for which the member is associated.</p>
    #[serde(rename = "GroupId")]
    pub group_id: String,
    /// <p>The member to associate to the group.</p>
    #[serde(rename = "MemberId")]
    pub member_id: String,
    /// <p>The organization under which the group exists.</p>
    #[serde(rename = "OrganizationId")]
    pub organization_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct AssociateMemberToGroupResponse {}

/// <p>At least one delegate must be associated to the resource to disable automatic replies from the resource.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BookingOptions {
    /// <p>The resource's ability to automatically reply to requests. If disabled, delegates must be associated to the resource.</p>
    #[serde(rename = "AutoAcceptRequests")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_accept_requests: Option<bool>,
    /// <p>The resource's ability to automatically decline any conflicting requests.</p>
    #[serde(rename = "AutoDeclineConflictingRequests")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_decline_conflicting_requests: Option<bool>,
    /// <p>The resource's ability to automatically decline any recurring requests.</p>
    #[serde(rename = "AutoDeclineRecurringRequests")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_decline_recurring_requests: Option<bool>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct CreateAliasRequest {
    /// <p>The alias to add to the user.</p>
    #[serde(rename = "Alias")]
    pub alias: String,
    /// <p>The alias is added to this Amazon WorkMail entity.</p>
    #[serde(rename = "EntityId")]
    pub entity_id: String,
    /// <p>The organization under which the member exists.</p>
    #[serde(rename = "OrganizationId")]
    pub organization_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct CreateAliasResponse {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct CreateGroupRequest {
    /// <p>The name of the group.</p>
    #[serde(rename = "Name")]
    pub name: String,
    /// <p>The organization under which the group is to be created.</p>
    #[serde(rename = "OrganizationId")]
    pub organization_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct CreateGroupResponse {
    /// <p>The ID of the group.</p>
    #[serde(rename = "GroupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_id: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct CreateResourceRequest {
    /// <p>The name of the created resource.</p>
    #[serde(rename = "Name")]
    pub name: String,
    /// <p>The identifier associated with the organization for which the resource is created.</p>
    #[serde(rename = "OrganizationId")]
    pub organization_id: String,
    /// <p>The type of the created resource.</p>
    #[serde(rename = "Type")]
    pub type_: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct CreateResourceResponse {
    /// <p>The identifier of the created resource.</p>
    #[serde(rename = "ResourceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_id: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct CreateUserRequest {
    /// <p>The display name for the user to be created.</p>
    #[serde(rename = "DisplayName")]
    pub display_name: String,
    /// <p>The name for the user to be created.</p>
    #[serde(rename = "Name")]
    pub name: String,
    /// <p>The identifier of the organization for which the user is created.</p>
    #[serde(rename = "OrganizationId")]
    pub organization_id: String,
    /// <p>The password for the user to be created.</p>
    #[serde(rename = "Password")]
    pub password: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct CreateUserResponse {
    /// <p>The information regarding the newly created user.</p>
    #[serde(rename = "UserId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
}

/// <p>The name of the attribute, which is one of the values defined in the UserAttribute enumeration.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct Delegate {
    /// <p>The identifier for the user or group is associated as the resource's delegate.</p>
    #[serde(rename = "Id")]
    pub id: String,
    /// <p>The type of the delegate: user or group.</p>
    #[serde(rename = "Type")]
    pub type_: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DeleteAliasRequest {
    /// <p>The aliases to be removed from the user's set of aliases. Duplicate entries in the list are collapsed into single entries (the list is transformed into a set).</p>
    #[serde(rename = "Alias")]
    pub alias: String,
    /// <p>The identifier for the Amazon WorkMail entity to have the aliases removed.</p>
    #[serde(rename = "EntityId")]
    pub entity_id: String,
    /// <p>The identifier for the organization under which the user exists.</p>
    #[serde(rename = "OrganizationId")]
    pub organization_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct DeleteAliasResponse {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DeleteGroupRequest {
    /// <p>The identifier of the group to be deleted.</p>
    #[serde(rename = "GroupId")]
    pub group_id: String,
    /// <p>The organization that contains the group.</p>
    #[serde(rename = "OrganizationId")]
    pub organization_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct DeleteGroupResponse {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DeleteMailboxPermissionsRequest {
    /// <p>The identifier of the entity (user or group) for which to delete mailbox permissions.</p>
    #[serde(rename = "EntityId")]
    pub entity_id: String,
    /// <p>The identifier of the entity (user or group) for which to delete granted permissions.</p>
    #[serde(rename = "GranteeId")]
    pub grantee_id: String,
    /// <p>The identifier of the organization under which the entity (user or group) exists.</p>
    #[serde(rename = "OrganizationId")]
    pub organization_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct DeleteMailboxPermissionsResponse {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DeleteResourceRequest {
    /// <p>The identifier associated with the organization for which the resource is deleted.</p>
    #[serde(rename = "OrganizationId")]
    pub organization_id: String,
    /// <p>The identifier of the resource to be deleted.</p>
    #[serde(rename = "ResourceId")]
    pub resource_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct DeleteResourceResponse {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DeleteUserRequest {
    /// <p>The organization that contains the user.</p>
    #[serde(rename = "OrganizationId")]
    pub organization_id: String,
    /// <p>The identifier of the user to be deleted.</p>
    #[serde(rename = "UserId")]
    pub user_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct DeleteUserResponse {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DeregisterFromWorkMailRequest {
    /// <p>The identifier for the entity to be updated.</p>
    #[serde(rename = "EntityId")]
    pub entity_id: String,
    /// <p>The identifier for the organization under which the Amazon WorkMail entity exists.</p>
    #[serde(rename = "OrganizationId")]
    pub organization_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct DeregisterFromWorkMailResponse {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DescribeGroupRequest {
    /// <p>The identifier for the group to be described.</p>
    #[serde(rename = "GroupId")]
    pub group_id: String,
    /// <p>The identifier for the organization under which the group exists.</p>
    #[serde(rename = "OrganizationId")]
    pub organization_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct DescribeGroupResponse {
    /// <p>The date and time when a user was deregistered from Amazon WorkMail, in UNIX epoch time format.</p>
    #[serde(rename = "DisabledDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disabled_date: Option<f64>,
    /// <p>The email of the described group.</p>
    #[serde(rename = "Email")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    /// <p>The date and time when a user was registered to Amazon WorkMail, in UNIX epoch time format.</p>
    #[serde(rename = "EnabledDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled_date: Option<f64>,
    /// <p>The identifier of the described group.</p>
    #[serde(rename = "GroupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_id: Option<String>,
    /// <p>The name of the described group.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The state of the user: enabled (registered to Amazon WorkMail) or disabled (deregistered or never registered to Amazon WorkMail).</p>
    #[serde(rename = "State")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DescribeOrganizationRequest {
    /// <p>The identifier for the organization to be described.</p>
    #[serde(rename = "OrganizationId")]
    pub organization_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct DescribeOrganizationResponse {
    /// <p>The alias for an organization.</p>
    #[serde(rename = "Alias")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alias: Option<String>,
    /// <p>The date at which the organization became usable in the Amazon WorkMail context, in UNIX epoch time format.</p>
    #[serde(rename = "CompletedDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub completed_date: Option<f64>,
    /// <p>The default mail domain associated with the organization.</p>
    #[serde(rename = "DefaultMailDomain")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_mail_domain: Option<String>,
    /// <p>The identifier for the directory associated with an Amazon WorkMail organization.</p>
    #[serde(rename = "DirectoryId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub directory_id: Option<String>,
    /// <p>The type of directory associated with the Amazon WorkMail organization.</p>
    #[serde(rename = "DirectoryType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub directory_type: Option<String>,
    /// <p>The (optional) error message indicating if unexpected behavior was encountered with regards to the organization.</p>
    #[serde(rename = "ErrorMessage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
    /// <p>The identifier of an organization.</p>
    #[serde(rename = "OrganizationId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub organization_id: Option<String>,
    /// <p>The state of an organization.</p>
    #[serde(rename = "State")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DescribeResourceRequest {
    /// <p>The identifier associated with the organization for which the resource is described.</p>
    #[serde(rename = "OrganizationId")]
    pub organization_id: String,
    /// <p>The identifier of the resource to be described.</p>
    #[serde(rename = "ResourceId")]
    pub resource_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct DescribeResourceResponse {
    /// <p>The booking options for the described resource.</p>
    #[serde(rename = "BookingOptions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub booking_options: Option<BookingOptions>,
    /// <p>The date and time when a resource was registered from Amazon WorkMail, in UNIX epoch time format.</p>
    #[serde(rename = "DisabledDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disabled_date: Option<f64>,
    /// <p>The email of the described resource.</p>
    #[serde(rename = "Email")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    /// <p>The date and time when a resource was registered to Amazon WorkMail, in UNIX epoch time format.</p>
    #[serde(rename = "EnabledDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled_date: Option<f64>,
    /// <p>The name of the described resource.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The identifier of the described resource.</p>
    #[serde(rename = "ResourceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_id: Option<String>,
    /// <p>The state of the resource: enabled (registered to Amazon WorkMail) or disabled (deregistered or never registered to Amazon WorkMail).</p>
    #[serde(rename = "State")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    /// <p>The type of the described resource.</p>
    #[serde(rename = "Type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DescribeUserRequest {
    /// <p>The identifier for the organization under which the user exists.</p>
    #[serde(rename = "OrganizationId")]
    pub organization_id: String,
    /// <p>The identifier for the user to be described.</p>
    #[serde(rename = "UserId")]
    pub user_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct DescribeUserResponse {
    /// <p>The date and time at which the user was disabled for Amazon WorkMail usage, in UNIX epoch time format.</p>
    #[serde(rename = "DisabledDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disabled_date: Option<f64>,
    /// <p>The display name of the user.</p>
    #[serde(rename = "DisplayName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    /// <p>The email of the user.</p>
    #[serde(rename = "Email")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    /// <p>The date and time at which the user was enabled for Amazon WorkMail usage, in UNIX epoch time format.</p>
    #[serde(rename = "EnabledDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled_date: Option<f64>,
    /// <p>The name for the user.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The state of a user: enabled (registered to Amazon WorkMail) or disabled (deregistered or never registered to Amazon WorkMail).</p>
    #[serde(rename = "State")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    /// <p>The identifier for the described user.</p>
    #[serde(rename = "UserId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    /// <p>In certain cases other entities are modeled as users. If interoperability is enabled, resources are imported into Amazon WorkMail as users. Because different Amazon WorkMail organizations rely on different directory types, administrators can distinguish between a user that is not registered to Amazon WorkMail (is disabled and has a user role) and the administrative users of the directory. The values are USER, RESOURCE, and SYSTEM_USER.</p>
    #[serde(rename = "UserRole")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_role: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DisassociateDelegateFromResourceRequest {
    /// <p>The identifier for the member (user, group) to be removed from the resource's delegates.</p>
    #[serde(rename = "EntityId")]
    pub entity_id: String,
    /// <p>The identifier for the organization under which the resource exists.</p>
    #[serde(rename = "OrganizationId")]
    pub organization_id: String,
    /// <p>The identifier of the resource from which delegates' set members are removed. </p>
    #[serde(rename = "ResourceId")]
    pub resource_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct DisassociateDelegateFromResourceResponse {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DisassociateMemberFromGroupRequest {
    /// <p>The identifier for the group from which members are removed.</p>
    #[serde(rename = "GroupId")]
    pub group_id: String,
    /// <p>The identifier for the member to be removed to the group.</p>
    #[serde(rename = "MemberId")]
    pub member_id: String,
    /// <p>The identifier for the organization under which the group exists.</p>
    #[serde(rename = "OrganizationId")]
    pub organization_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct DisassociateMemberFromGroupResponse {}

/// <p>The representation of an Amazon WorkMail group.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct Group {
    /// <p>The date indicating when the group was disabled from Amazon WorkMail use.</p>
    #[serde(rename = "DisabledDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disabled_date: Option<f64>,
    /// <p>The email of the group.</p>
    #[serde(rename = "Email")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    /// <p>The date indicating when the group was enabled for Amazon WorkMail use.</p>
    #[serde(rename = "EnabledDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled_date: Option<f64>,
    /// <p>The identifier of the group.</p>
    #[serde(rename = "Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// <p>The name of the group.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The state of the group, which can be ENABLED, DISABLED, or DELETED.</p>
    #[serde(rename = "State")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ListAliasesRequest {
    /// <p>The identifier for the entity for which to list the aliases.</p>
    #[serde(rename = "EntityId")]
    pub entity_id: String,
    /// <p>The maximum number of results to return in a single call.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>The token to use to retrieve the next page of results. The first call does not contain any tokens.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The identifier for the organization under which the entity exists.</p>
    #[serde(rename = "OrganizationId")]
    pub organization_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct ListAliasesResponse {
    /// <p>The entity's paginated aliases.</p>
    #[serde(rename = "Aliases")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aliases: Option<Vec<String>>,
    /// <p>The token to use to retrieve the next page of results. The value is "null" when there are no more results to return.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ListGroupMembersRequest {
    /// <p>The identifier for the group to which the members are associated.</p>
    #[serde(rename = "GroupId")]
    pub group_id: String,
    /// <p>The maximum number of results to return in a single call.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p> The token to use to retrieve the next page of results. The first call does not contain any tokens.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The identifier for the organization under which the group exists.</p>
    #[serde(rename = "OrganizationId")]
    pub organization_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct ListGroupMembersResponse {
    /// <p>The members associated to the group.</p>
    #[serde(rename = "Members")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub members: Option<Vec<Member>>,
    /// <p>The token to use to retrieve the next page of results. The first call does not contain any tokens.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ListGroupsRequest {
    /// <p>The maximum number of results to return in a single call.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>The token to use to retrieve the next page of results. The first call does not contain any tokens.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The identifier for the organization under which the groups exist.</p>
    #[serde(rename = "OrganizationId")]
    pub organization_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct ListGroupsResponse {
    /// <p>The overview of groups for an organization.</p>
    #[serde(rename = "Groups")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub groups: Option<Vec<Group>>,
    /// <p>The token to use to retrieve the next page of results. The value is "null" when there are no more results to return.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ListMailboxPermissionsRequest {
    /// <p>The identifier of the entity (user or group) for which to list mailbox permissions.</p>
    #[serde(rename = "EntityId")]
    pub entity_id: String,
    /// <p>The maximum number of results to return in a single call.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>The token to use to retrieve the next page of results. The first call does not contain any tokens.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The identifier of the organization under which the entity (user or group) exists.</p>
    #[serde(rename = "OrganizationId")]
    pub organization_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct ListMailboxPermissionsResponse {
    /// <p>The token to use to retrieve the next page of results. The value is "null" when there are no more results to return.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>One page of the entity's mailbox permissions.</p>
    #[serde(rename = "Permissions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permissions: Option<Vec<Permission>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ListOrganizationsRequest {
    /// <p>The maximum number of results to return in a single call.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>The token to use to retrieve the next page of results. The first call does not contain any tokens.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct ListOrganizationsResponse {
    /// <p>The token to use to retrieve the next page of results. The value is "null" when there are no more results to return.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The overview of owned organizations presented as a list of organization summaries.</p>
    #[serde(rename = "OrganizationSummaries")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub organization_summaries: Option<Vec<OrganizationSummary>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ListResourceDelegatesRequest {
    /// <p>The number of maximum results in a page.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>The token used to paginate through the delegates associated with a resource.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The identifier for the organization that contains the resource for which delegates are listed.</p>
    #[serde(rename = "OrganizationId")]
    pub organization_id: String,
    /// <p>The identifier for the resource whose delegates are listed.</p>
    #[serde(rename = "ResourceId")]
    pub resource_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct ListResourceDelegatesResponse {
    /// <p>One page of the resource's delegates.</p>
    #[serde(rename = "Delegates")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delegates: Option<Vec<Delegate>>,
    /// <p>The token used to paginate through the delegates associated with a resource. While results are still available, it has an associated value. When the last page is reached, the token is empty. </p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ListResourcesRequest {
    /// <p>The maximum number of results to return in a single call.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>The token to use to retrieve the next page of results. The first call does not contain any tokens.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The identifier for the organization under which the resources exist.</p>
    #[serde(rename = "OrganizationId")]
    pub organization_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct ListResourcesResponse {
    /// <p> The token used to paginate through all the organization's resources. While results are still available, it has an associated value. When the last page is reached, the token is empty.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>One page of the organization's resource representation.</p>
    #[serde(rename = "Resources")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resources: Option<Vec<Resource>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ListUsersRequest {
    /// <p>The maximum number of results to return in a single call.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>TBD</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The identifier for the organization under which the users exist.</p>
    #[serde(rename = "OrganizationId")]
    pub organization_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct ListUsersResponse {
    /// <p> The token to use to retrieve the next page of results. This value is `null` when there are no more results to return.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The overview of users for an organization.</p>
    #[serde(rename = "Users")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub users: Option<Vec<User>>,
}

/// <p>The representation of a group member (user or group).</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct Member {
    /// <p>The date indicating when the member was disabled from Amazon WorkMail use.</p>
    #[serde(rename = "DisabledDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disabled_date: Option<f64>,
    /// <p>The date indicating when the member was enabled for Amazon WorkMail use.</p>
    #[serde(rename = "EnabledDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled_date: Option<f64>,
    /// <p>The identifier of the member.</p>
    #[serde(rename = "Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// <p>The name of the member.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The state of the member, which can be ENABLED, DISABLED, or DELETED.</p>
    #[serde(rename = "State")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    /// <p>A member can be a user or group.</p>
    #[serde(rename = "Type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}

/// <p>The brief overview associated with an organization.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct OrganizationSummary {
    /// <p>The alias associated with the organization.</p>
    #[serde(rename = "Alias")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alias: Option<String>,
    /// <p>The error message associated with the organization. It is only present if unexpected behavior has occurred with regards to the organization. It provides insight or solutions regarding unexpected behavior.</p>
    #[serde(rename = "ErrorMessage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
    /// <p>The identifier associated with the organization.</p>
    #[serde(rename = "OrganizationId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub organization_id: Option<String>,
    /// <p>The state associated with the organization.</p>
    #[serde(rename = "State")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
}

/// <p>Permission granted to an entity (user, group) to access a certain aspect of another entity's mailbox.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct Permission {
    /// <p>The identifier of the entity (user or group) to which the permissions are granted.</p>
    #[serde(rename = "GranteeId")]
    pub grantee_id: String,
    /// <p>The type of entity (user, group) of the entity referred to in GranteeId.</p>
    #[serde(rename = "GranteeType")]
    pub grantee_type: String,
    /// <p>The permissions granted to the grantee. SEND_AS allows the grantee to send email as the owner of the mailbox (the grantee is not mentioned on these emails). SEND_ON_BEHALF allows the grantee to send email on behalf of the owner of the mailbox (the grantee is not mentioned as the physical sender of these emails). FULL_ACCESS allows the grantee full access to the mailbox, irrespective of other folder-level permissions set on the mailbox.</p>
    #[serde(rename = "PermissionValues")]
    pub permission_values: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct PutMailboxPermissionsRequest {
    /// <p>The identifier of the entity (user or group) for which to update mailbox permissions.</p>
    #[serde(rename = "EntityId")]
    pub entity_id: String,
    /// <p>The identifier of the entity (user or group) to which to grant the permissions.</p>
    #[serde(rename = "GranteeId")]
    pub grantee_id: String,
    /// <p>The identifier of the organization under which the entity (user or group) exists.</p>
    #[serde(rename = "OrganizationId")]
    pub organization_id: String,
    /// <p>The permissions granted to the grantee. SEND_AS allows the grantee to send email as the owner of the mailbox (the grantee is not mentioned on these emails). SEND_ON_BEHALF allows the grantee to send email on behalf of the owner of the mailbox (the grantee is not mentioned as the physical sender of these emails). FULL_ACCESS allows the grantee full access to the mailbox, irrespective of other folder-level permissions set on the mailbox.</p>
    #[serde(rename = "PermissionValues")]
    pub permission_values: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct PutMailboxPermissionsResponse {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct RegisterToWorkMailRequest {
    /// <p>The email for the entity to be updated.</p>
    #[serde(rename = "Email")]
    pub email: String,
    /// <p>The identifier for the entity to be updated.</p>
    #[serde(rename = "EntityId")]
    pub entity_id: String,
    /// <p>The identifier for the organization under which the Amazon WorkMail entity exists.</p>
    #[serde(rename = "OrganizationId")]
    pub organization_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct RegisterToWorkMailResponse {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ResetPasswordRequest {
    /// <p>The identifier of the organization that contains the user for which the password is reset.</p>
    #[serde(rename = "OrganizationId")]
    pub organization_id: String,
    /// <p>The new password for the user.</p>
    #[serde(rename = "Password")]
    pub password: String,
    /// <p>The identifier of the user for whom the password is reset.</p>
    #[serde(rename = "UserId")]
    pub user_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct ResetPasswordResponse {}

/// <p>The overview for a resource containing relevant data regarding it.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct Resource {
    /// <p>The date indicating when the resource was disabled from Amazon WorkMail use.</p>
    #[serde(rename = "DisabledDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disabled_date: Option<f64>,
    /// <p>The email of the resource.</p>
    #[serde(rename = "Email")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    /// <p>The date indicating when the resource was enabled for Amazon WorkMail use.</p>
    #[serde(rename = "EnabledDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled_date: Option<f64>,
    /// <p>The identifier of the resource.</p>
    #[serde(rename = "Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// <p>The name of the resource.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The state of the resource, which can be ENABLED, DISABLED, or DELETED.</p>
    #[serde(rename = "State")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    /// <p>The type of the resource: equipment or room.</p>
    #[serde(rename = "Type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct UpdatePrimaryEmailAddressRequest {
    /// <p>The value of the email to be updated as primary.</p>
    #[serde(rename = "Email")]
    pub email: String,
    /// <p>The entity to update (user, group, or resource).</p>
    #[serde(rename = "EntityId")]
    pub entity_id: String,
    /// <p>The organization that contains the entity to update.</p>
    #[serde(rename = "OrganizationId")]
    pub organization_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct UpdatePrimaryEmailAddressResponse {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct UpdateResourceRequest {
    /// <p>The resource's booking options to be updated.</p>
    #[serde(rename = "BookingOptions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub booking_options: Option<BookingOptions>,
    /// <p>The name of the resource to be updated.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The identifier associated with the organization for which the resource is updated.</p>
    #[serde(rename = "OrganizationId")]
    pub organization_id: String,
    /// <p>The identifier of the resource to be updated.</p>
    #[serde(rename = "ResourceId")]
    pub resource_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct UpdateResourceResponse {}

/// <p>The representation of an Amazon WorkMail user.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct User {
    /// <p>The date indicating when the user was disabled from Amazon WorkMail use.</p>
    #[serde(rename = "DisabledDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disabled_date: Option<f64>,
    /// <p>The display name of the user.</p>
    #[serde(rename = "DisplayName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    /// <p>The email of the user.</p>
    #[serde(rename = "Email")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    /// <p>The date indicating when the user was enabled for Amazon WorkMail use.</p>
    #[serde(rename = "EnabledDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled_date: Option<f64>,
    /// <p>The identifier of the user.</p>
    #[serde(rename = "Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// <p>The name of the user.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The state of the user, which can be ENABLED, DISABLED, or DELETED.</p>
    #[serde(rename = "State")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    /// <p>The role of the user.</p>
    #[serde(rename = "UserRole")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_role: Option<String>,
}

/// Errors returned by AssociateDelegateToResource
#[derive(Debug, PartialEq)]
pub enum AssociateDelegateToResourceError {
    /// <p>The identifier supplied for the entity is valid, but it does not exist in your organization.</p>
    EntityNotFound(String),
    /// <p>You are performing an operation on an entity that isn't in the expected state, such as trying to update a deleted user.</p>
    EntityState(String),
    /// <p>One or more of the input parameters don't match the service's restrictions.</p>
    InvalidParameter(String),
    /// <p>An operation received a valid organization identifier that either doesn't belong or exist in the system.</p>
    OrganizationNotFound(String),
    /// <p>The organization must have a valid state (Active or Synchronizing) to perform certain operations on the organization or its entities.</p>
    OrganizationState(String),
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

impl AssociateDelegateToResourceError {
    pub fn from_response(res: BufferedHttpResponse) -> AssociateDelegateToResourceError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "EntityNotFoundException" => {
                    return AssociateDelegateToResourceError::EntityNotFound(String::from(
                        error_message,
                    ));
                }
                "EntityStateException" => {
                    return AssociateDelegateToResourceError::EntityState(String::from(
                        error_message,
                    ));
                }
                "InvalidParameterException" => {
                    return AssociateDelegateToResourceError::InvalidParameter(String::from(
                        error_message,
                    ));
                }
                "OrganizationNotFoundException" => {
                    return AssociateDelegateToResourceError::OrganizationNotFound(String::from(
                        error_message,
                    ));
                }
                "OrganizationStateException" => {
                    return AssociateDelegateToResourceError::OrganizationState(String::from(
                        error_message,
                    ));
                }
                "ValidationException" => {
                    return AssociateDelegateToResourceError::Validation(error_message.to_string());
                }
                _ => {}
            }
        }
        return AssociateDelegateToResourceError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for AssociateDelegateToResourceError {
    fn from(err: serde_json::error::Error) -> AssociateDelegateToResourceError {
        AssociateDelegateToResourceError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for AssociateDelegateToResourceError {
    fn from(err: CredentialsError) -> AssociateDelegateToResourceError {
        AssociateDelegateToResourceError::Credentials(err)
    }
}
impl From<HttpDispatchError> for AssociateDelegateToResourceError {
    fn from(err: HttpDispatchError) -> AssociateDelegateToResourceError {
        AssociateDelegateToResourceError::HttpDispatch(err)
    }
}
impl From<io::Error> for AssociateDelegateToResourceError {
    fn from(err: io::Error) -> AssociateDelegateToResourceError {
        AssociateDelegateToResourceError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for AssociateDelegateToResourceError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for AssociateDelegateToResourceError {
    fn description(&self) -> &str {
        match *self {
            AssociateDelegateToResourceError::EntityNotFound(ref cause) => cause,
            AssociateDelegateToResourceError::EntityState(ref cause) => cause,
            AssociateDelegateToResourceError::InvalidParameter(ref cause) => cause,
            AssociateDelegateToResourceError::OrganizationNotFound(ref cause) => cause,
            AssociateDelegateToResourceError::OrganizationState(ref cause) => cause,
            AssociateDelegateToResourceError::Validation(ref cause) => cause,
            AssociateDelegateToResourceError::Credentials(ref err) => err.description(),
            AssociateDelegateToResourceError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            AssociateDelegateToResourceError::ParseError(ref cause) => cause,
            AssociateDelegateToResourceError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by AssociateMemberToGroup
#[derive(Debug, PartialEq)]
pub enum AssociateMemberToGroupError {
    /// <p>The Directory Service doesn't recognize the credentials supplied by the Amazon WorkMail service.</p>
    DirectoryServiceAuthenticationFailed(String),
    /// <p>The directory that you are trying to perform operations on isn't available.</p>
    DirectoryUnavailable(String),
    /// <p>The identifier supplied for the entity is valid, but it does not exist in your organization.</p>
    EntityNotFound(String),
    /// <p>You are performing an operation on an entity that isn't in the expected state, such as trying to update a deleted user.</p>
    EntityState(String),
    /// <p>One or more of the input parameters don't match the service's restrictions.</p>
    InvalidParameter(String),
    /// <p>An operation received a valid organization identifier that either doesn't belong or exist in the system.</p>
    OrganizationNotFound(String),
    /// <p>The organization must have a valid state (Active or Synchronizing) to perform certain operations on the organization or its entities.</p>
    OrganizationState(String),
    /// <p>You can't perform a write operation against a read-only directory.</p>
    UnsupportedOperation(String),
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

impl AssociateMemberToGroupError {
    pub fn from_response(res: BufferedHttpResponse) -> AssociateMemberToGroupError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "DirectoryServiceAuthenticationFailedException" => {
                    return AssociateMemberToGroupError::DirectoryServiceAuthenticationFailed(
                        String::from(error_message),
                    );
                }
                "DirectoryUnavailableException" => {
                    return AssociateMemberToGroupError::DirectoryUnavailable(String::from(
                        error_message,
                    ));
                }
                "EntityNotFoundException" => {
                    return AssociateMemberToGroupError::EntityNotFound(String::from(error_message));
                }
                "EntityStateException" => {
                    return AssociateMemberToGroupError::EntityState(String::from(error_message));
                }
                "InvalidParameterException" => {
                    return AssociateMemberToGroupError::InvalidParameter(String::from(
                        error_message,
                    ));
                }
                "OrganizationNotFoundException" => {
                    return AssociateMemberToGroupError::OrganizationNotFound(String::from(
                        error_message,
                    ));
                }
                "OrganizationStateException" => {
                    return AssociateMemberToGroupError::OrganizationState(String::from(
                        error_message,
                    ));
                }
                "UnsupportedOperationException" => {
                    return AssociateMemberToGroupError::UnsupportedOperation(String::from(
                        error_message,
                    ));
                }
                "ValidationException" => {
                    return AssociateMemberToGroupError::Validation(error_message.to_string());
                }
                _ => {}
            }
        }
        return AssociateMemberToGroupError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for AssociateMemberToGroupError {
    fn from(err: serde_json::error::Error) -> AssociateMemberToGroupError {
        AssociateMemberToGroupError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for AssociateMemberToGroupError {
    fn from(err: CredentialsError) -> AssociateMemberToGroupError {
        AssociateMemberToGroupError::Credentials(err)
    }
}
impl From<HttpDispatchError> for AssociateMemberToGroupError {
    fn from(err: HttpDispatchError) -> AssociateMemberToGroupError {
        AssociateMemberToGroupError::HttpDispatch(err)
    }
}
impl From<io::Error> for AssociateMemberToGroupError {
    fn from(err: io::Error) -> AssociateMemberToGroupError {
        AssociateMemberToGroupError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for AssociateMemberToGroupError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for AssociateMemberToGroupError {
    fn description(&self) -> &str {
        match *self {
            AssociateMemberToGroupError::DirectoryServiceAuthenticationFailed(ref cause) => cause,
            AssociateMemberToGroupError::DirectoryUnavailable(ref cause) => cause,
            AssociateMemberToGroupError::EntityNotFound(ref cause) => cause,
            AssociateMemberToGroupError::EntityState(ref cause) => cause,
            AssociateMemberToGroupError::InvalidParameter(ref cause) => cause,
            AssociateMemberToGroupError::OrganizationNotFound(ref cause) => cause,
            AssociateMemberToGroupError::OrganizationState(ref cause) => cause,
            AssociateMemberToGroupError::UnsupportedOperation(ref cause) => cause,
            AssociateMemberToGroupError::Validation(ref cause) => cause,
            AssociateMemberToGroupError::Credentials(ref err) => err.description(),
            AssociateMemberToGroupError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            AssociateMemberToGroupError::ParseError(ref cause) => cause,
            AssociateMemberToGroupError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by CreateAlias
#[derive(Debug, PartialEq)]
pub enum CreateAliasError {
    /// <p>The email address that you're trying to assign is already created for a different user, group, or resource.</p>
    EmailAddressInUse(String),
    /// <p>The identifier supplied for the entity is valid, but it does not exist in your organization.</p>
    EntityNotFound(String),
    /// <p>You are performing an operation on an entity that isn't in the expected state, such as trying to update a deleted user.</p>
    EntityState(String),
    /// <p>One or more of the input parameters don't match the service's restrictions.</p>
    InvalidParameter(String),
    /// <p>For an email or alias to be created in Amazon WorkMail, the included domain must be defined in the organization.</p>
    MailDomainNotFound(String),
    /// <p>After a domain has been added to the organization, it must be verified. The domain is not yet verified.</p>
    MailDomainState(String),
    /// <p>An operation received a valid organization identifier that either doesn't belong or exist in the system.</p>
    OrganizationNotFound(String),
    /// <p>The organization must have a valid state (Active or Synchronizing) to perform certain operations on the organization or its entities.</p>
    OrganizationState(String),
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

impl CreateAliasError {
    pub fn from_response(res: BufferedHttpResponse) -> CreateAliasError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "EmailAddressInUseException" => {
                    return CreateAliasError::EmailAddressInUse(String::from(error_message));
                }
                "EntityNotFoundException" => {
                    return CreateAliasError::EntityNotFound(String::from(error_message));
                }
                "EntityStateException" => {
                    return CreateAliasError::EntityState(String::from(error_message));
                }
                "InvalidParameterException" => {
                    return CreateAliasError::InvalidParameter(String::from(error_message));
                }
                "MailDomainNotFoundException" => {
                    return CreateAliasError::MailDomainNotFound(String::from(error_message));
                }
                "MailDomainStateException" => {
                    return CreateAliasError::MailDomainState(String::from(error_message));
                }
                "OrganizationNotFoundException" => {
                    return CreateAliasError::OrganizationNotFound(String::from(error_message));
                }
                "OrganizationStateException" => {
                    return CreateAliasError::OrganizationState(String::from(error_message));
                }
                "ValidationException" => {
                    return CreateAliasError::Validation(error_message.to_string());
                }
                _ => {}
            }
        }
        return CreateAliasError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for CreateAliasError {
    fn from(err: serde_json::error::Error) -> CreateAliasError {
        CreateAliasError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for CreateAliasError {
    fn from(err: CredentialsError) -> CreateAliasError {
        CreateAliasError::Credentials(err)
    }
}
impl From<HttpDispatchError> for CreateAliasError {
    fn from(err: HttpDispatchError) -> CreateAliasError {
        CreateAliasError::HttpDispatch(err)
    }
}
impl From<io::Error> for CreateAliasError {
    fn from(err: io::Error) -> CreateAliasError {
        CreateAliasError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for CreateAliasError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateAliasError {
    fn description(&self) -> &str {
        match *self {
            CreateAliasError::EmailAddressInUse(ref cause) => cause,
            CreateAliasError::EntityNotFound(ref cause) => cause,
            CreateAliasError::EntityState(ref cause) => cause,
            CreateAliasError::InvalidParameter(ref cause) => cause,
            CreateAliasError::MailDomainNotFound(ref cause) => cause,
            CreateAliasError::MailDomainState(ref cause) => cause,
            CreateAliasError::OrganizationNotFound(ref cause) => cause,
            CreateAliasError::OrganizationState(ref cause) => cause,
            CreateAliasError::Validation(ref cause) => cause,
            CreateAliasError::Credentials(ref err) => err.description(),
            CreateAliasError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            CreateAliasError::ParseError(ref cause) => cause,
            CreateAliasError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by CreateGroup
#[derive(Debug, PartialEq)]
pub enum CreateGroupError {
    /// <p>The Directory Service doesn't recognize the credentials supplied by the Amazon WorkMail service.</p>
    DirectoryServiceAuthenticationFailed(String),
    /// <p>The directory that you are trying to perform operations on isn't available.</p>
    DirectoryUnavailable(String),
    /// <p>One or more of the input parameters don't match the service's restrictions.</p>
    InvalidParameter(String),
    /// <p>The entity (user, group, or user) name isn't unique in Amazon WorkMail.</p>
    NameAvailability(String),
    /// <p>An operation received a valid organization identifier that either doesn't belong or exist in the system.</p>
    OrganizationNotFound(String),
    /// <p>The organization must have a valid state (Active or Synchronizing) to perform certain operations on the organization or its entities.</p>
    OrganizationState(String),
    /// <p>This entity name is not allowed in Amazon WorkMail.</p>
    ReservedName(String),
    /// <p>You can't perform a write operation against a read-only directory.</p>
    UnsupportedOperation(String),
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

impl CreateGroupError {
    pub fn from_response(res: BufferedHttpResponse) -> CreateGroupError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "DirectoryServiceAuthenticationFailedException" => {
                    return CreateGroupError::DirectoryServiceAuthenticationFailed(String::from(
                        error_message,
                    ));
                }
                "DirectoryUnavailableException" => {
                    return CreateGroupError::DirectoryUnavailable(String::from(error_message));
                }
                "InvalidParameterException" => {
                    return CreateGroupError::InvalidParameter(String::from(error_message));
                }
                "NameAvailabilityException" => {
                    return CreateGroupError::NameAvailability(String::from(error_message));
                }
                "OrganizationNotFoundException" => {
                    return CreateGroupError::OrganizationNotFound(String::from(error_message));
                }
                "OrganizationStateException" => {
                    return CreateGroupError::OrganizationState(String::from(error_message));
                }
                "ReservedNameException" => {
                    return CreateGroupError::ReservedName(String::from(error_message));
                }
                "UnsupportedOperationException" => {
                    return CreateGroupError::UnsupportedOperation(String::from(error_message));
                }
                "ValidationException" => {
                    return CreateGroupError::Validation(error_message.to_string());
                }
                _ => {}
            }
        }
        return CreateGroupError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for CreateGroupError {
    fn from(err: serde_json::error::Error) -> CreateGroupError {
        CreateGroupError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for CreateGroupError {
    fn from(err: CredentialsError) -> CreateGroupError {
        CreateGroupError::Credentials(err)
    }
}
impl From<HttpDispatchError> for CreateGroupError {
    fn from(err: HttpDispatchError) -> CreateGroupError {
        CreateGroupError::HttpDispatch(err)
    }
}
impl From<io::Error> for CreateGroupError {
    fn from(err: io::Error) -> CreateGroupError {
        CreateGroupError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for CreateGroupError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateGroupError {
    fn description(&self) -> &str {
        match *self {
            CreateGroupError::DirectoryServiceAuthenticationFailed(ref cause) => cause,
            CreateGroupError::DirectoryUnavailable(ref cause) => cause,
            CreateGroupError::InvalidParameter(ref cause) => cause,
            CreateGroupError::NameAvailability(ref cause) => cause,
            CreateGroupError::OrganizationNotFound(ref cause) => cause,
            CreateGroupError::OrganizationState(ref cause) => cause,
            CreateGroupError::ReservedName(ref cause) => cause,
            CreateGroupError::UnsupportedOperation(ref cause) => cause,
            CreateGroupError::Validation(ref cause) => cause,
            CreateGroupError::Credentials(ref err) => err.description(),
            CreateGroupError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            CreateGroupError::ParseError(ref cause) => cause,
            CreateGroupError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by CreateResource
#[derive(Debug, PartialEq)]
pub enum CreateResourceError {
    /// <p>The Directory Service doesn't recognize the credentials supplied by the Amazon WorkMail service.</p>
    DirectoryServiceAuthenticationFailed(String),
    /// <p>The directory that you are trying to perform operations on isn't available.</p>
    DirectoryUnavailable(String),
    /// <p>One or more of the input parameters don't match the service's restrictions.</p>
    InvalidParameter(String),
    /// <p>The entity (user, group, or user) name isn't unique in Amazon WorkMail.</p>
    NameAvailability(String),
    /// <p>An operation received a valid organization identifier that either doesn't belong or exist in the system.</p>
    OrganizationNotFound(String),
    /// <p>The organization must have a valid state (Active or Synchronizing) to perform certain operations on the organization or its entities.</p>
    OrganizationState(String),
    /// <p>This entity name is not allowed in Amazon WorkMail.</p>
    ReservedName(String),
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

impl CreateResourceError {
    pub fn from_response(res: BufferedHttpResponse) -> CreateResourceError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "DirectoryServiceAuthenticationFailedException" => {
                    return CreateResourceError::DirectoryServiceAuthenticationFailed(String::from(
                        error_message,
                    ));
                }
                "DirectoryUnavailableException" => {
                    return CreateResourceError::DirectoryUnavailable(String::from(error_message));
                }
                "InvalidParameterException" => {
                    return CreateResourceError::InvalidParameter(String::from(error_message));
                }
                "NameAvailabilityException" => {
                    return CreateResourceError::NameAvailability(String::from(error_message));
                }
                "OrganizationNotFoundException" => {
                    return CreateResourceError::OrganizationNotFound(String::from(error_message));
                }
                "OrganizationStateException" => {
                    return CreateResourceError::OrganizationState(String::from(error_message));
                }
                "ReservedNameException" => {
                    return CreateResourceError::ReservedName(String::from(error_message));
                }
                "ValidationException" => {
                    return CreateResourceError::Validation(error_message.to_string());
                }
                _ => {}
            }
        }
        return CreateResourceError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for CreateResourceError {
    fn from(err: serde_json::error::Error) -> CreateResourceError {
        CreateResourceError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for CreateResourceError {
    fn from(err: CredentialsError) -> CreateResourceError {
        CreateResourceError::Credentials(err)
    }
}
impl From<HttpDispatchError> for CreateResourceError {
    fn from(err: HttpDispatchError) -> CreateResourceError {
        CreateResourceError::HttpDispatch(err)
    }
}
impl From<io::Error> for CreateResourceError {
    fn from(err: io::Error) -> CreateResourceError {
        CreateResourceError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for CreateResourceError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateResourceError {
    fn description(&self) -> &str {
        match *self {
            CreateResourceError::DirectoryServiceAuthenticationFailed(ref cause) => cause,
            CreateResourceError::DirectoryUnavailable(ref cause) => cause,
            CreateResourceError::InvalidParameter(ref cause) => cause,
            CreateResourceError::NameAvailability(ref cause) => cause,
            CreateResourceError::OrganizationNotFound(ref cause) => cause,
            CreateResourceError::OrganizationState(ref cause) => cause,
            CreateResourceError::ReservedName(ref cause) => cause,
            CreateResourceError::Validation(ref cause) => cause,
            CreateResourceError::Credentials(ref err) => err.description(),
            CreateResourceError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            CreateResourceError::ParseError(ref cause) => cause,
            CreateResourceError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by CreateUser
#[derive(Debug, PartialEq)]
pub enum CreateUserError {
    /// <p>The Directory Service doesn't recognize the credentials supplied by the Amazon WorkMail service.</p>
    DirectoryServiceAuthenticationFailed(String),
    /// <p>The directory that you are trying to perform operations on isn't available.</p>
    DirectoryUnavailable(String),
    /// <p>One or more of the input parameters don't match the service's restrictions.</p>
    InvalidParameter(String),
    /// <p>The supplied password doesn't match the minimum security constraints, such as length or use of special characters.</p>
    InvalidPassword(String),
    /// <p>The entity (user, group, or user) name isn't unique in Amazon WorkMail.</p>
    NameAvailability(String),
    /// <p>An operation received a valid organization identifier that either doesn't belong or exist in the system.</p>
    OrganizationNotFound(String),
    /// <p>The organization must have a valid state (Active or Synchronizing) to perform certain operations on the organization or its entities.</p>
    OrganizationState(String),
    /// <p>This entity name is not allowed in Amazon WorkMail.</p>
    ReservedName(String),
    /// <p>You can't perform a write operation against a read-only directory.</p>
    UnsupportedOperation(String),
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

impl CreateUserError {
    pub fn from_response(res: BufferedHttpResponse) -> CreateUserError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "DirectoryServiceAuthenticationFailedException" => {
                    return CreateUserError::DirectoryServiceAuthenticationFailed(String::from(
                        error_message,
                    ));
                }
                "DirectoryUnavailableException" => {
                    return CreateUserError::DirectoryUnavailable(String::from(error_message));
                }
                "InvalidParameterException" => {
                    return CreateUserError::InvalidParameter(String::from(error_message));
                }
                "InvalidPasswordException" => {
                    return CreateUserError::InvalidPassword(String::from(error_message));
                }
                "NameAvailabilityException" => {
                    return CreateUserError::NameAvailability(String::from(error_message));
                }
                "OrganizationNotFoundException" => {
                    return CreateUserError::OrganizationNotFound(String::from(error_message));
                }
                "OrganizationStateException" => {
                    return CreateUserError::OrganizationState(String::from(error_message));
                }
                "ReservedNameException" => {
                    return CreateUserError::ReservedName(String::from(error_message));
                }
                "UnsupportedOperationException" => {
                    return CreateUserError::UnsupportedOperation(String::from(error_message));
                }
                "ValidationException" => {
                    return CreateUserError::Validation(error_message.to_string());
                }
                _ => {}
            }
        }
        return CreateUserError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for CreateUserError {
    fn from(err: serde_json::error::Error) -> CreateUserError {
        CreateUserError::ParseError(err.description().to_string())
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
            CreateUserError::DirectoryServiceAuthenticationFailed(ref cause) => cause,
            CreateUserError::DirectoryUnavailable(ref cause) => cause,
            CreateUserError::InvalidParameter(ref cause) => cause,
            CreateUserError::InvalidPassword(ref cause) => cause,
            CreateUserError::NameAvailability(ref cause) => cause,
            CreateUserError::OrganizationNotFound(ref cause) => cause,
            CreateUserError::OrganizationState(ref cause) => cause,
            CreateUserError::ReservedName(ref cause) => cause,
            CreateUserError::UnsupportedOperation(ref cause) => cause,
            CreateUserError::Validation(ref cause) => cause,
            CreateUserError::Credentials(ref err) => err.description(),
            CreateUserError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            CreateUserError::ParseError(ref cause) => cause,
            CreateUserError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by DeleteAlias
#[derive(Debug, PartialEq)]
pub enum DeleteAliasError {
    /// <p>The identifier supplied for the entity is valid, but it does not exist in your organization.</p>
    EntityNotFound(String),
    /// <p>You are performing an operation on an entity that isn't in the expected state, such as trying to update a deleted user.</p>
    EntityState(String),
    /// <p>One or more of the input parameters don't match the service's restrictions.</p>
    InvalidParameter(String),
    /// <p>An operation received a valid organization identifier that either doesn't belong or exist in the system.</p>
    OrganizationNotFound(String),
    /// <p>The organization must have a valid state (Active or Synchronizing) to perform certain operations on the organization or its entities.</p>
    OrganizationState(String),
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

impl DeleteAliasError {
    pub fn from_response(res: BufferedHttpResponse) -> DeleteAliasError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "EntityNotFoundException" => {
                    return DeleteAliasError::EntityNotFound(String::from(error_message));
                }
                "EntityStateException" => {
                    return DeleteAliasError::EntityState(String::from(error_message));
                }
                "InvalidParameterException" => {
                    return DeleteAliasError::InvalidParameter(String::from(error_message));
                }
                "OrganizationNotFoundException" => {
                    return DeleteAliasError::OrganizationNotFound(String::from(error_message));
                }
                "OrganizationStateException" => {
                    return DeleteAliasError::OrganizationState(String::from(error_message));
                }
                "ValidationException" => {
                    return DeleteAliasError::Validation(error_message.to_string());
                }
                _ => {}
            }
        }
        return DeleteAliasError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for DeleteAliasError {
    fn from(err: serde_json::error::Error) -> DeleteAliasError {
        DeleteAliasError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for DeleteAliasError {
    fn from(err: CredentialsError) -> DeleteAliasError {
        DeleteAliasError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DeleteAliasError {
    fn from(err: HttpDispatchError) -> DeleteAliasError {
        DeleteAliasError::HttpDispatch(err)
    }
}
impl From<io::Error> for DeleteAliasError {
    fn from(err: io::Error) -> DeleteAliasError {
        DeleteAliasError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DeleteAliasError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteAliasError {
    fn description(&self) -> &str {
        match *self {
            DeleteAliasError::EntityNotFound(ref cause) => cause,
            DeleteAliasError::EntityState(ref cause) => cause,
            DeleteAliasError::InvalidParameter(ref cause) => cause,
            DeleteAliasError::OrganizationNotFound(ref cause) => cause,
            DeleteAliasError::OrganizationState(ref cause) => cause,
            DeleteAliasError::Validation(ref cause) => cause,
            DeleteAliasError::Credentials(ref err) => err.description(),
            DeleteAliasError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            DeleteAliasError::ParseError(ref cause) => cause,
            DeleteAliasError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by DeleteGroup
#[derive(Debug, PartialEq)]
pub enum DeleteGroupError {
    /// <p>The Directory Service doesn't recognize the credentials supplied by the Amazon WorkMail service.</p>
    DirectoryServiceAuthenticationFailed(String),
    /// <p>The directory that you are trying to perform operations on isn't available.</p>
    DirectoryUnavailable(String),
    /// <p>You are performing an operation on an entity that isn't in the expected state, such as trying to update a deleted user.</p>
    EntityState(String),
    /// <p>One or more of the input parameters don't match the service's restrictions.</p>
    InvalidParameter(String),
    /// <p>An operation received a valid organization identifier that either doesn't belong or exist in the system.</p>
    OrganizationNotFound(String),
    /// <p>The organization must have a valid state (Active or Synchronizing) to perform certain operations on the organization or its entities.</p>
    OrganizationState(String),
    /// <p>You can't perform a write operation against a read-only directory.</p>
    UnsupportedOperation(String),
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

impl DeleteGroupError {
    pub fn from_response(res: BufferedHttpResponse) -> DeleteGroupError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "DirectoryServiceAuthenticationFailedException" => {
                    return DeleteGroupError::DirectoryServiceAuthenticationFailed(String::from(
                        error_message,
                    ));
                }
                "DirectoryUnavailableException" => {
                    return DeleteGroupError::DirectoryUnavailable(String::from(error_message));
                }
                "EntityStateException" => {
                    return DeleteGroupError::EntityState(String::from(error_message));
                }
                "InvalidParameterException" => {
                    return DeleteGroupError::InvalidParameter(String::from(error_message));
                }
                "OrganizationNotFoundException" => {
                    return DeleteGroupError::OrganizationNotFound(String::from(error_message));
                }
                "OrganizationStateException" => {
                    return DeleteGroupError::OrganizationState(String::from(error_message));
                }
                "UnsupportedOperationException" => {
                    return DeleteGroupError::UnsupportedOperation(String::from(error_message));
                }
                "ValidationException" => {
                    return DeleteGroupError::Validation(error_message.to_string());
                }
                _ => {}
            }
        }
        return DeleteGroupError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for DeleteGroupError {
    fn from(err: serde_json::error::Error) -> DeleteGroupError {
        DeleteGroupError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for DeleteGroupError {
    fn from(err: CredentialsError) -> DeleteGroupError {
        DeleteGroupError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DeleteGroupError {
    fn from(err: HttpDispatchError) -> DeleteGroupError {
        DeleteGroupError::HttpDispatch(err)
    }
}
impl From<io::Error> for DeleteGroupError {
    fn from(err: io::Error) -> DeleteGroupError {
        DeleteGroupError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DeleteGroupError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteGroupError {
    fn description(&self) -> &str {
        match *self {
            DeleteGroupError::DirectoryServiceAuthenticationFailed(ref cause) => cause,
            DeleteGroupError::DirectoryUnavailable(ref cause) => cause,
            DeleteGroupError::EntityState(ref cause) => cause,
            DeleteGroupError::InvalidParameter(ref cause) => cause,
            DeleteGroupError::OrganizationNotFound(ref cause) => cause,
            DeleteGroupError::OrganizationState(ref cause) => cause,
            DeleteGroupError::UnsupportedOperation(ref cause) => cause,
            DeleteGroupError::Validation(ref cause) => cause,
            DeleteGroupError::Credentials(ref err) => err.description(),
            DeleteGroupError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            DeleteGroupError::ParseError(ref cause) => cause,
            DeleteGroupError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by DeleteMailboxPermissions
#[derive(Debug, PartialEq)]
pub enum DeleteMailboxPermissionsError {
    /// <p>The identifier supplied for the entity is valid, but it does not exist in your organization.</p>
    EntityNotFound(String),
    /// <p>You are performing an operation on an entity that isn't in the expected state, such as trying to update a deleted user.</p>
    EntityState(String),
    /// <p>One or more of the input parameters don't match the service's restrictions.</p>
    InvalidParameter(String),
    /// <p>An operation received a valid organization identifier that either doesn't belong or exist in the system.</p>
    OrganizationNotFound(String),
    /// <p>The organization must have a valid state (Active or Synchronizing) to perform certain operations on the organization or its entities.</p>
    OrganizationState(String),
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

impl DeleteMailboxPermissionsError {
    pub fn from_response(res: BufferedHttpResponse) -> DeleteMailboxPermissionsError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "EntityNotFoundException" => {
                    return DeleteMailboxPermissionsError::EntityNotFound(String::from(
                        error_message,
                    ));
                }
                "EntityStateException" => {
                    return DeleteMailboxPermissionsError::EntityState(String::from(error_message));
                }
                "InvalidParameterException" => {
                    return DeleteMailboxPermissionsError::InvalidParameter(String::from(
                        error_message,
                    ));
                }
                "OrganizationNotFoundException" => {
                    return DeleteMailboxPermissionsError::OrganizationNotFound(String::from(
                        error_message,
                    ));
                }
                "OrganizationStateException" => {
                    return DeleteMailboxPermissionsError::OrganizationState(String::from(
                        error_message,
                    ));
                }
                "ValidationException" => {
                    return DeleteMailboxPermissionsError::Validation(error_message.to_string());
                }
                _ => {}
            }
        }
        return DeleteMailboxPermissionsError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for DeleteMailboxPermissionsError {
    fn from(err: serde_json::error::Error) -> DeleteMailboxPermissionsError {
        DeleteMailboxPermissionsError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for DeleteMailboxPermissionsError {
    fn from(err: CredentialsError) -> DeleteMailboxPermissionsError {
        DeleteMailboxPermissionsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DeleteMailboxPermissionsError {
    fn from(err: HttpDispatchError) -> DeleteMailboxPermissionsError {
        DeleteMailboxPermissionsError::HttpDispatch(err)
    }
}
impl From<io::Error> for DeleteMailboxPermissionsError {
    fn from(err: io::Error) -> DeleteMailboxPermissionsError {
        DeleteMailboxPermissionsError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DeleteMailboxPermissionsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteMailboxPermissionsError {
    fn description(&self) -> &str {
        match *self {
            DeleteMailboxPermissionsError::EntityNotFound(ref cause) => cause,
            DeleteMailboxPermissionsError::EntityState(ref cause) => cause,
            DeleteMailboxPermissionsError::InvalidParameter(ref cause) => cause,
            DeleteMailboxPermissionsError::OrganizationNotFound(ref cause) => cause,
            DeleteMailboxPermissionsError::OrganizationState(ref cause) => cause,
            DeleteMailboxPermissionsError::Validation(ref cause) => cause,
            DeleteMailboxPermissionsError::Credentials(ref err) => err.description(),
            DeleteMailboxPermissionsError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DeleteMailboxPermissionsError::ParseError(ref cause) => cause,
            DeleteMailboxPermissionsError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by DeleteResource
#[derive(Debug, PartialEq)]
pub enum DeleteResourceError {
    /// <p>You are performing an operation on an entity that isn't in the expected state, such as trying to update a deleted user.</p>
    EntityState(String),
    /// <p>One or more of the input parameters don't match the service's restrictions.</p>
    InvalidParameter(String),
    /// <p>An operation received a valid organization identifier that either doesn't belong or exist in the system.</p>
    OrganizationNotFound(String),
    /// <p>The organization must have a valid state (Active or Synchronizing) to perform certain operations on the organization or its entities.</p>
    OrganizationState(String),
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

impl DeleteResourceError {
    pub fn from_response(res: BufferedHttpResponse) -> DeleteResourceError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "EntityStateException" => {
                    return DeleteResourceError::EntityState(String::from(error_message));
                }
                "InvalidParameterException" => {
                    return DeleteResourceError::InvalidParameter(String::from(error_message));
                }
                "OrganizationNotFoundException" => {
                    return DeleteResourceError::OrganizationNotFound(String::from(error_message));
                }
                "OrganizationStateException" => {
                    return DeleteResourceError::OrganizationState(String::from(error_message));
                }
                "ValidationException" => {
                    return DeleteResourceError::Validation(error_message.to_string());
                }
                _ => {}
            }
        }
        return DeleteResourceError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for DeleteResourceError {
    fn from(err: serde_json::error::Error) -> DeleteResourceError {
        DeleteResourceError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for DeleteResourceError {
    fn from(err: CredentialsError) -> DeleteResourceError {
        DeleteResourceError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DeleteResourceError {
    fn from(err: HttpDispatchError) -> DeleteResourceError {
        DeleteResourceError::HttpDispatch(err)
    }
}
impl From<io::Error> for DeleteResourceError {
    fn from(err: io::Error) -> DeleteResourceError {
        DeleteResourceError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DeleteResourceError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteResourceError {
    fn description(&self) -> &str {
        match *self {
            DeleteResourceError::EntityState(ref cause) => cause,
            DeleteResourceError::InvalidParameter(ref cause) => cause,
            DeleteResourceError::OrganizationNotFound(ref cause) => cause,
            DeleteResourceError::OrganizationState(ref cause) => cause,
            DeleteResourceError::Validation(ref cause) => cause,
            DeleteResourceError::Credentials(ref err) => err.description(),
            DeleteResourceError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            DeleteResourceError::ParseError(ref cause) => cause,
            DeleteResourceError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by DeleteUser
#[derive(Debug, PartialEq)]
pub enum DeleteUserError {
    /// <p>The Directory Service doesn't recognize the credentials supplied by the Amazon WorkMail service.</p>
    DirectoryServiceAuthenticationFailed(String),
    /// <p>The directory that you are trying to perform operations on isn't available.</p>
    DirectoryUnavailable(String),
    /// <p>You are performing an operation on an entity that isn't in the expected state, such as trying to update a deleted user.</p>
    EntityState(String),
    /// <p>One or more of the input parameters don't match the service's restrictions.</p>
    InvalidParameter(String),
    /// <p>An operation received a valid organization identifier that either doesn't belong or exist in the system.</p>
    OrganizationNotFound(String),
    /// <p>The organization must have a valid state (Active or Synchronizing) to perform certain operations on the organization or its entities.</p>
    OrganizationState(String),
    /// <p>You can't perform a write operation against a read-only directory.</p>
    UnsupportedOperation(String),
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

impl DeleteUserError {
    pub fn from_response(res: BufferedHttpResponse) -> DeleteUserError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "DirectoryServiceAuthenticationFailedException" => {
                    return DeleteUserError::DirectoryServiceAuthenticationFailed(String::from(
                        error_message,
                    ));
                }
                "DirectoryUnavailableException" => {
                    return DeleteUserError::DirectoryUnavailable(String::from(error_message));
                }
                "EntityStateException" => {
                    return DeleteUserError::EntityState(String::from(error_message));
                }
                "InvalidParameterException" => {
                    return DeleteUserError::InvalidParameter(String::from(error_message));
                }
                "OrganizationNotFoundException" => {
                    return DeleteUserError::OrganizationNotFound(String::from(error_message));
                }
                "OrganizationStateException" => {
                    return DeleteUserError::OrganizationState(String::from(error_message));
                }
                "UnsupportedOperationException" => {
                    return DeleteUserError::UnsupportedOperation(String::from(error_message));
                }
                "ValidationException" => {
                    return DeleteUserError::Validation(error_message.to_string());
                }
                _ => {}
            }
        }
        return DeleteUserError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for DeleteUserError {
    fn from(err: serde_json::error::Error) -> DeleteUserError {
        DeleteUserError::ParseError(err.description().to_string())
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
            DeleteUserError::DirectoryServiceAuthenticationFailed(ref cause) => cause,
            DeleteUserError::DirectoryUnavailable(ref cause) => cause,
            DeleteUserError::EntityState(ref cause) => cause,
            DeleteUserError::InvalidParameter(ref cause) => cause,
            DeleteUserError::OrganizationNotFound(ref cause) => cause,
            DeleteUserError::OrganizationState(ref cause) => cause,
            DeleteUserError::UnsupportedOperation(ref cause) => cause,
            DeleteUserError::Validation(ref cause) => cause,
            DeleteUserError::Credentials(ref err) => err.description(),
            DeleteUserError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            DeleteUserError::ParseError(ref cause) => cause,
            DeleteUserError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by DeregisterFromWorkMail
#[derive(Debug, PartialEq)]
pub enum DeregisterFromWorkMailError {
    /// <p>The identifier supplied for the entity is valid, but it does not exist in your organization.</p>
    EntityNotFound(String),
    /// <p>You are performing an operation on an entity that isn't in the expected state, such as trying to update a deleted user.</p>
    EntityState(String),
    /// <p>One or more of the input parameters don't match the service's restrictions.</p>
    InvalidParameter(String),
    /// <p>An operation received a valid organization identifier that either doesn't belong or exist in the system.</p>
    OrganizationNotFound(String),
    /// <p>The organization must have a valid state (Active or Synchronizing) to perform certain operations on the organization or its entities.</p>
    OrganizationState(String),
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

impl DeregisterFromWorkMailError {
    pub fn from_response(res: BufferedHttpResponse) -> DeregisterFromWorkMailError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "EntityNotFoundException" => {
                    return DeregisterFromWorkMailError::EntityNotFound(String::from(error_message));
                }
                "EntityStateException" => {
                    return DeregisterFromWorkMailError::EntityState(String::from(error_message));
                }
                "InvalidParameterException" => {
                    return DeregisterFromWorkMailError::InvalidParameter(String::from(
                        error_message,
                    ));
                }
                "OrganizationNotFoundException" => {
                    return DeregisterFromWorkMailError::OrganizationNotFound(String::from(
                        error_message,
                    ));
                }
                "OrganizationStateException" => {
                    return DeregisterFromWorkMailError::OrganizationState(String::from(
                        error_message,
                    ));
                }
                "ValidationException" => {
                    return DeregisterFromWorkMailError::Validation(error_message.to_string());
                }
                _ => {}
            }
        }
        return DeregisterFromWorkMailError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for DeregisterFromWorkMailError {
    fn from(err: serde_json::error::Error) -> DeregisterFromWorkMailError {
        DeregisterFromWorkMailError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for DeregisterFromWorkMailError {
    fn from(err: CredentialsError) -> DeregisterFromWorkMailError {
        DeregisterFromWorkMailError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DeregisterFromWorkMailError {
    fn from(err: HttpDispatchError) -> DeregisterFromWorkMailError {
        DeregisterFromWorkMailError::HttpDispatch(err)
    }
}
impl From<io::Error> for DeregisterFromWorkMailError {
    fn from(err: io::Error) -> DeregisterFromWorkMailError {
        DeregisterFromWorkMailError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DeregisterFromWorkMailError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeregisterFromWorkMailError {
    fn description(&self) -> &str {
        match *self {
            DeregisterFromWorkMailError::EntityNotFound(ref cause) => cause,
            DeregisterFromWorkMailError::EntityState(ref cause) => cause,
            DeregisterFromWorkMailError::InvalidParameter(ref cause) => cause,
            DeregisterFromWorkMailError::OrganizationNotFound(ref cause) => cause,
            DeregisterFromWorkMailError::OrganizationState(ref cause) => cause,
            DeregisterFromWorkMailError::Validation(ref cause) => cause,
            DeregisterFromWorkMailError::Credentials(ref err) => err.description(),
            DeregisterFromWorkMailError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DeregisterFromWorkMailError::ParseError(ref cause) => cause,
            DeregisterFromWorkMailError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by DescribeGroup
#[derive(Debug, PartialEq)]
pub enum DescribeGroupError {
    /// <p>The identifier supplied for the entity is valid, but it does not exist in your organization.</p>
    EntityNotFound(String),
    /// <p>One or more of the input parameters don't match the service's restrictions.</p>
    InvalidParameter(String),
    /// <p>An operation received a valid organization identifier that either doesn't belong or exist in the system.</p>
    OrganizationNotFound(String),
    /// <p>The organization must have a valid state (Active or Synchronizing) to perform certain operations on the organization or its entities.</p>
    OrganizationState(String),
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

impl DescribeGroupError {
    pub fn from_response(res: BufferedHttpResponse) -> DescribeGroupError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "EntityNotFoundException" => {
                    return DescribeGroupError::EntityNotFound(String::from(error_message));
                }
                "InvalidParameterException" => {
                    return DescribeGroupError::InvalidParameter(String::from(error_message));
                }
                "OrganizationNotFoundException" => {
                    return DescribeGroupError::OrganizationNotFound(String::from(error_message));
                }
                "OrganizationStateException" => {
                    return DescribeGroupError::OrganizationState(String::from(error_message));
                }
                "ValidationException" => {
                    return DescribeGroupError::Validation(error_message.to_string());
                }
                _ => {}
            }
        }
        return DescribeGroupError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for DescribeGroupError {
    fn from(err: serde_json::error::Error) -> DescribeGroupError {
        DescribeGroupError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for DescribeGroupError {
    fn from(err: CredentialsError) -> DescribeGroupError {
        DescribeGroupError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DescribeGroupError {
    fn from(err: HttpDispatchError) -> DescribeGroupError {
        DescribeGroupError::HttpDispatch(err)
    }
}
impl From<io::Error> for DescribeGroupError {
    fn from(err: io::Error) -> DescribeGroupError {
        DescribeGroupError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DescribeGroupError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeGroupError {
    fn description(&self) -> &str {
        match *self {
            DescribeGroupError::EntityNotFound(ref cause) => cause,
            DescribeGroupError::InvalidParameter(ref cause) => cause,
            DescribeGroupError::OrganizationNotFound(ref cause) => cause,
            DescribeGroupError::OrganizationState(ref cause) => cause,
            DescribeGroupError::Validation(ref cause) => cause,
            DescribeGroupError::Credentials(ref err) => err.description(),
            DescribeGroupError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            DescribeGroupError::ParseError(ref cause) => cause,
            DescribeGroupError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by DescribeOrganization
#[derive(Debug, PartialEq)]
pub enum DescribeOrganizationError {
    /// <p>One or more of the input parameters don't match the service's restrictions.</p>
    InvalidParameter(String),
    /// <p>An operation received a valid organization identifier that either doesn't belong or exist in the system.</p>
    OrganizationNotFound(String),
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

impl DescribeOrganizationError {
    pub fn from_response(res: BufferedHttpResponse) -> DescribeOrganizationError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "InvalidParameterException" => {
                    return DescribeOrganizationError::InvalidParameter(String::from(error_message));
                }
                "OrganizationNotFoundException" => {
                    return DescribeOrganizationError::OrganizationNotFound(String::from(
                        error_message,
                    ));
                }
                "ValidationException" => {
                    return DescribeOrganizationError::Validation(error_message.to_string());
                }
                _ => {}
            }
        }
        return DescribeOrganizationError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for DescribeOrganizationError {
    fn from(err: serde_json::error::Error) -> DescribeOrganizationError {
        DescribeOrganizationError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for DescribeOrganizationError {
    fn from(err: CredentialsError) -> DescribeOrganizationError {
        DescribeOrganizationError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DescribeOrganizationError {
    fn from(err: HttpDispatchError) -> DescribeOrganizationError {
        DescribeOrganizationError::HttpDispatch(err)
    }
}
impl From<io::Error> for DescribeOrganizationError {
    fn from(err: io::Error) -> DescribeOrganizationError {
        DescribeOrganizationError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DescribeOrganizationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeOrganizationError {
    fn description(&self) -> &str {
        match *self {
            DescribeOrganizationError::InvalidParameter(ref cause) => cause,
            DescribeOrganizationError::OrganizationNotFound(ref cause) => cause,
            DescribeOrganizationError::Validation(ref cause) => cause,
            DescribeOrganizationError::Credentials(ref err) => err.description(),
            DescribeOrganizationError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DescribeOrganizationError::ParseError(ref cause) => cause,
            DescribeOrganizationError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by DescribeResource
#[derive(Debug, PartialEq)]
pub enum DescribeResourceError {
    /// <p>The identifier supplied for the entity is valid, but it does not exist in your organization.</p>
    EntityNotFound(String),
    /// <p>One or more of the input parameters don't match the service's restrictions.</p>
    InvalidParameter(String),
    /// <p>An operation received a valid organization identifier that either doesn't belong or exist in the system.</p>
    OrganizationNotFound(String),
    /// <p>The organization must have a valid state (Active or Synchronizing) to perform certain operations on the organization or its entities.</p>
    OrganizationState(String),
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

impl DescribeResourceError {
    pub fn from_response(res: BufferedHttpResponse) -> DescribeResourceError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "EntityNotFoundException" => {
                    return DescribeResourceError::EntityNotFound(String::from(error_message));
                }
                "InvalidParameterException" => {
                    return DescribeResourceError::InvalidParameter(String::from(error_message));
                }
                "OrganizationNotFoundException" => {
                    return DescribeResourceError::OrganizationNotFound(String::from(error_message));
                }
                "OrganizationStateException" => {
                    return DescribeResourceError::OrganizationState(String::from(error_message));
                }
                "ValidationException" => {
                    return DescribeResourceError::Validation(error_message.to_string());
                }
                _ => {}
            }
        }
        return DescribeResourceError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for DescribeResourceError {
    fn from(err: serde_json::error::Error) -> DescribeResourceError {
        DescribeResourceError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for DescribeResourceError {
    fn from(err: CredentialsError) -> DescribeResourceError {
        DescribeResourceError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DescribeResourceError {
    fn from(err: HttpDispatchError) -> DescribeResourceError {
        DescribeResourceError::HttpDispatch(err)
    }
}
impl From<io::Error> for DescribeResourceError {
    fn from(err: io::Error) -> DescribeResourceError {
        DescribeResourceError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DescribeResourceError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeResourceError {
    fn description(&self) -> &str {
        match *self {
            DescribeResourceError::EntityNotFound(ref cause) => cause,
            DescribeResourceError::InvalidParameter(ref cause) => cause,
            DescribeResourceError::OrganizationNotFound(ref cause) => cause,
            DescribeResourceError::OrganizationState(ref cause) => cause,
            DescribeResourceError::Validation(ref cause) => cause,
            DescribeResourceError::Credentials(ref err) => err.description(),
            DescribeResourceError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            DescribeResourceError::ParseError(ref cause) => cause,
            DescribeResourceError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by DescribeUser
#[derive(Debug, PartialEq)]
pub enum DescribeUserError {
    /// <p>The identifier supplied for the entity is valid, but it does not exist in your organization.</p>
    EntityNotFound(String),
    /// <p>One or more of the input parameters don't match the service's restrictions.</p>
    InvalidParameter(String),
    /// <p>An operation received a valid organization identifier that either doesn't belong or exist in the system.</p>
    OrganizationNotFound(String),
    /// <p>The organization must have a valid state (Active or Synchronizing) to perform certain operations on the organization or its entities.</p>
    OrganizationState(String),
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

impl DescribeUserError {
    pub fn from_response(res: BufferedHttpResponse) -> DescribeUserError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "EntityNotFoundException" => {
                    return DescribeUserError::EntityNotFound(String::from(error_message));
                }
                "InvalidParameterException" => {
                    return DescribeUserError::InvalidParameter(String::from(error_message));
                }
                "OrganizationNotFoundException" => {
                    return DescribeUserError::OrganizationNotFound(String::from(error_message));
                }
                "OrganizationStateException" => {
                    return DescribeUserError::OrganizationState(String::from(error_message));
                }
                "ValidationException" => {
                    return DescribeUserError::Validation(error_message.to_string());
                }
                _ => {}
            }
        }
        return DescribeUserError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for DescribeUserError {
    fn from(err: serde_json::error::Error) -> DescribeUserError {
        DescribeUserError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for DescribeUserError {
    fn from(err: CredentialsError) -> DescribeUserError {
        DescribeUserError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DescribeUserError {
    fn from(err: HttpDispatchError) -> DescribeUserError {
        DescribeUserError::HttpDispatch(err)
    }
}
impl From<io::Error> for DescribeUserError {
    fn from(err: io::Error) -> DescribeUserError {
        DescribeUserError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DescribeUserError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeUserError {
    fn description(&self) -> &str {
        match *self {
            DescribeUserError::EntityNotFound(ref cause) => cause,
            DescribeUserError::InvalidParameter(ref cause) => cause,
            DescribeUserError::OrganizationNotFound(ref cause) => cause,
            DescribeUserError::OrganizationState(ref cause) => cause,
            DescribeUserError::Validation(ref cause) => cause,
            DescribeUserError::Credentials(ref err) => err.description(),
            DescribeUserError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            DescribeUserError::ParseError(ref cause) => cause,
            DescribeUserError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by DisassociateDelegateFromResource
#[derive(Debug, PartialEq)]
pub enum DisassociateDelegateFromResourceError {
    /// <p>The identifier supplied for the entity is valid, but it does not exist in your organization.</p>
    EntityNotFound(String),
    /// <p>You are performing an operation on an entity that isn't in the expected state, such as trying to update a deleted user.</p>
    EntityState(String),
    /// <p>One or more of the input parameters don't match the service's restrictions.</p>
    InvalidParameter(String),
    /// <p>An operation received a valid organization identifier that either doesn't belong or exist in the system.</p>
    OrganizationNotFound(String),
    /// <p>The organization must have a valid state (Active or Synchronizing) to perform certain operations on the organization or its entities.</p>
    OrganizationState(String),
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

impl DisassociateDelegateFromResourceError {
    pub fn from_response(res: BufferedHttpResponse) -> DisassociateDelegateFromResourceError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "EntityNotFoundException" => {
                    return DisassociateDelegateFromResourceError::EntityNotFound(String::from(
                        error_message,
                    ));
                }
                "EntityStateException" => {
                    return DisassociateDelegateFromResourceError::EntityState(String::from(
                        error_message,
                    ));
                }
                "InvalidParameterException" => {
                    return DisassociateDelegateFromResourceError::InvalidParameter(String::from(
                        error_message,
                    ));
                }
                "OrganizationNotFoundException" => {
                    return DisassociateDelegateFromResourceError::OrganizationNotFound(
                        String::from(error_message),
                    );
                }
                "OrganizationStateException" => {
                    return DisassociateDelegateFromResourceError::OrganizationState(String::from(
                        error_message,
                    ));
                }
                "ValidationException" => {
                    return DisassociateDelegateFromResourceError::Validation(
                        error_message.to_string(),
                    );
                }
                _ => {}
            }
        }
        return DisassociateDelegateFromResourceError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for DisassociateDelegateFromResourceError {
    fn from(err: serde_json::error::Error) -> DisassociateDelegateFromResourceError {
        DisassociateDelegateFromResourceError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for DisassociateDelegateFromResourceError {
    fn from(err: CredentialsError) -> DisassociateDelegateFromResourceError {
        DisassociateDelegateFromResourceError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DisassociateDelegateFromResourceError {
    fn from(err: HttpDispatchError) -> DisassociateDelegateFromResourceError {
        DisassociateDelegateFromResourceError::HttpDispatch(err)
    }
}
impl From<io::Error> for DisassociateDelegateFromResourceError {
    fn from(err: io::Error) -> DisassociateDelegateFromResourceError {
        DisassociateDelegateFromResourceError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DisassociateDelegateFromResourceError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DisassociateDelegateFromResourceError {
    fn description(&self) -> &str {
        match *self {
            DisassociateDelegateFromResourceError::EntityNotFound(ref cause) => cause,
            DisassociateDelegateFromResourceError::EntityState(ref cause) => cause,
            DisassociateDelegateFromResourceError::InvalidParameter(ref cause) => cause,
            DisassociateDelegateFromResourceError::OrganizationNotFound(ref cause) => cause,
            DisassociateDelegateFromResourceError::OrganizationState(ref cause) => cause,
            DisassociateDelegateFromResourceError::Validation(ref cause) => cause,
            DisassociateDelegateFromResourceError::Credentials(ref err) => err.description(),
            DisassociateDelegateFromResourceError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DisassociateDelegateFromResourceError::ParseError(ref cause) => cause,
            DisassociateDelegateFromResourceError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by DisassociateMemberFromGroup
#[derive(Debug, PartialEq)]
pub enum DisassociateMemberFromGroupError {
    /// <p>The Directory Service doesn't recognize the credentials supplied by the Amazon WorkMail service.</p>
    DirectoryServiceAuthenticationFailed(String),
    /// <p>The directory that you are trying to perform operations on isn't available.</p>
    DirectoryUnavailable(String),
    /// <p>The identifier supplied for the entity is valid, but it does not exist in your organization.</p>
    EntityNotFound(String),
    /// <p>You are performing an operation on an entity that isn't in the expected state, such as trying to update a deleted user.</p>
    EntityState(String),
    /// <p>One or more of the input parameters don't match the service's restrictions.</p>
    InvalidParameter(String),
    /// <p>An operation received a valid organization identifier that either doesn't belong or exist in the system.</p>
    OrganizationNotFound(String),
    /// <p>The organization must have a valid state (Active or Synchronizing) to perform certain operations on the organization or its entities.</p>
    OrganizationState(String),
    /// <p>You can't perform a write operation against a read-only directory.</p>
    UnsupportedOperation(String),
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

impl DisassociateMemberFromGroupError {
    pub fn from_response(res: BufferedHttpResponse) -> DisassociateMemberFromGroupError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "DirectoryServiceAuthenticationFailedException" => {
                    return DisassociateMemberFromGroupError::DirectoryServiceAuthenticationFailed(
                        String::from(error_message),
                    );
                }
                "DirectoryUnavailableException" => {
                    return DisassociateMemberFromGroupError::DirectoryUnavailable(String::from(
                        error_message,
                    ));
                }
                "EntityNotFoundException" => {
                    return DisassociateMemberFromGroupError::EntityNotFound(String::from(
                        error_message,
                    ));
                }
                "EntityStateException" => {
                    return DisassociateMemberFromGroupError::EntityState(String::from(
                        error_message,
                    ));
                }
                "InvalidParameterException" => {
                    return DisassociateMemberFromGroupError::InvalidParameter(String::from(
                        error_message,
                    ));
                }
                "OrganizationNotFoundException" => {
                    return DisassociateMemberFromGroupError::OrganizationNotFound(String::from(
                        error_message,
                    ));
                }
                "OrganizationStateException" => {
                    return DisassociateMemberFromGroupError::OrganizationState(String::from(
                        error_message,
                    ));
                }
                "UnsupportedOperationException" => {
                    return DisassociateMemberFromGroupError::UnsupportedOperation(String::from(
                        error_message,
                    ));
                }
                "ValidationException" => {
                    return DisassociateMemberFromGroupError::Validation(error_message.to_string());
                }
                _ => {}
            }
        }
        return DisassociateMemberFromGroupError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for DisassociateMemberFromGroupError {
    fn from(err: serde_json::error::Error) -> DisassociateMemberFromGroupError {
        DisassociateMemberFromGroupError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for DisassociateMemberFromGroupError {
    fn from(err: CredentialsError) -> DisassociateMemberFromGroupError {
        DisassociateMemberFromGroupError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DisassociateMemberFromGroupError {
    fn from(err: HttpDispatchError) -> DisassociateMemberFromGroupError {
        DisassociateMemberFromGroupError::HttpDispatch(err)
    }
}
impl From<io::Error> for DisassociateMemberFromGroupError {
    fn from(err: io::Error) -> DisassociateMemberFromGroupError {
        DisassociateMemberFromGroupError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DisassociateMemberFromGroupError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DisassociateMemberFromGroupError {
    fn description(&self) -> &str {
        match *self {
            DisassociateMemberFromGroupError::DirectoryServiceAuthenticationFailed(ref cause) => {
                cause
            }
            DisassociateMemberFromGroupError::DirectoryUnavailable(ref cause) => cause,
            DisassociateMemberFromGroupError::EntityNotFound(ref cause) => cause,
            DisassociateMemberFromGroupError::EntityState(ref cause) => cause,
            DisassociateMemberFromGroupError::InvalidParameter(ref cause) => cause,
            DisassociateMemberFromGroupError::OrganizationNotFound(ref cause) => cause,
            DisassociateMemberFromGroupError::OrganizationState(ref cause) => cause,
            DisassociateMemberFromGroupError::UnsupportedOperation(ref cause) => cause,
            DisassociateMemberFromGroupError::Validation(ref cause) => cause,
            DisassociateMemberFromGroupError::Credentials(ref err) => err.description(),
            DisassociateMemberFromGroupError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DisassociateMemberFromGroupError::ParseError(ref cause) => cause,
            DisassociateMemberFromGroupError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by ListAliases
#[derive(Debug, PartialEq)]
pub enum ListAliasesError {
    /// <p>The identifier supplied for the entity is valid, but it does not exist in your organization.</p>
    EntityNotFound(String),
    /// <p>You are performing an operation on an entity that isn't in the expected state, such as trying to update a deleted user.</p>
    EntityState(String),
    /// <p>One or more of the input parameters don't match the service's restrictions.</p>
    InvalidParameter(String),
    /// <p>An operation received a valid organization identifier that either doesn't belong or exist in the system.</p>
    OrganizationNotFound(String),
    /// <p>The organization must have a valid state (Active or Synchronizing) to perform certain operations on the organization or its entities.</p>
    OrganizationState(String),
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

impl ListAliasesError {
    pub fn from_response(res: BufferedHttpResponse) -> ListAliasesError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "EntityNotFoundException" => {
                    return ListAliasesError::EntityNotFound(String::from(error_message));
                }
                "EntityStateException" => {
                    return ListAliasesError::EntityState(String::from(error_message));
                }
                "InvalidParameterException" => {
                    return ListAliasesError::InvalidParameter(String::from(error_message));
                }
                "OrganizationNotFoundException" => {
                    return ListAliasesError::OrganizationNotFound(String::from(error_message));
                }
                "OrganizationStateException" => {
                    return ListAliasesError::OrganizationState(String::from(error_message));
                }
                "ValidationException" => {
                    return ListAliasesError::Validation(error_message.to_string());
                }
                _ => {}
            }
        }
        return ListAliasesError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for ListAliasesError {
    fn from(err: serde_json::error::Error) -> ListAliasesError {
        ListAliasesError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for ListAliasesError {
    fn from(err: CredentialsError) -> ListAliasesError {
        ListAliasesError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ListAliasesError {
    fn from(err: HttpDispatchError) -> ListAliasesError {
        ListAliasesError::HttpDispatch(err)
    }
}
impl From<io::Error> for ListAliasesError {
    fn from(err: io::Error) -> ListAliasesError {
        ListAliasesError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for ListAliasesError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListAliasesError {
    fn description(&self) -> &str {
        match *self {
            ListAliasesError::EntityNotFound(ref cause) => cause,
            ListAliasesError::EntityState(ref cause) => cause,
            ListAliasesError::InvalidParameter(ref cause) => cause,
            ListAliasesError::OrganizationNotFound(ref cause) => cause,
            ListAliasesError::OrganizationState(ref cause) => cause,
            ListAliasesError::Validation(ref cause) => cause,
            ListAliasesError::Credentials(ref err) => err.description(),
            ListAliasesError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            ListAliasesError::ParseError(ref cause) => cause,
            ListAliasesError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by ListGroupMembers
#[derive(Debug, PartialEq)]
pub enum ListGroupMembersError {
    /// <p>The identifier supplied for the entity is valid, but it does not exist in your organization.</p>
    EntityNotFound(String),
    /// <p>You are performing an operation on an entity that isn't in the expected state, such as trying to update a deleted user.</p>
    EntityState(String),
    /// <p>One or more of the input parameters don't match the service's restrictions.</p>
    InvalidParameter(String),
    /// <p>An operation received a valid organization identifier that either doesn't belong or exist in the system.</p>
    OrganizationNotFound(String),
    /// <p>The organization must have a valid state (Active or Synchronizing) to perform certain operations on the organization or its entities.</p>
    OrganizationState(String),
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

impl ListGroupMembersError {
    pub fn from_response(res: BufferedHttpResponse) -> ListGroupMembersError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "EntityNotFoundException" => {
                    return ListGroupMembersError::EntityNotFound(String::from(error_message));
                }
                "EntityStateException" => {
                    return ListGroupMembersError::EntityState(String::from(error_message));
                }
                "InvalidParameterException" => {
                    return ListGroupMembersError::InvalidParameter(String::from(error_message));
                }
                "OrganizationNotFoundException" => {
                    return ListGroupMembersError::OrganizationNotFound(String::from(error_message));
                }
                "OrganizationStateException" => {
                    return ListGroupMembersError::OrganizationState(String::from(error_message));
                }
                "ValidationException" => {
                    return ListGroupMembersError::Validation(error_message.to_string());
                }
                _ => {}
            }
        }
        return ListGroupMembersError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for ListGroupMembersError {
    fn from(err: serde_json::error::Error) -> ListGroupMembersError {
        ListGroupMembersError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for ListGroupMembersError {
    fn from(err: CredentialsError) -> ListGroupMembersError {
        ListGroupMembersError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ListGroupMembersError {
    fn from(err: HttpDispatchError) -> ListGroupMembersError {
        ListGroupMembersError::HttpDispatch(err)
    }
}
impl From<io::Error> for ListGroupMembersError {
    fn from(err: io::Error) -> ListGroupMembersError {
        ListGroupMembersError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for ListGroupMembersError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListGroupMembersError {
    fn description(&self) -> &str {
        match *self {
            ListGroupMembersError::EntityNotFound(ref cause) => cause,
            ListGroupMembersError::EntityState(ref cause) => cause,
            ListGroupMembersError::InvalidParameter(ref cause) => cause,
            ListGroupMembersError::OrganizationNotFound(ref cause) => cause,
            ListGroupMembersError::OrganizationState(ref cause) => cause,
            ListGroupMembersError::Validation(ref cause) => cause,
            ListGroupMembersError::Credentials(ref err) => err.description(),
            ListGroupMembersError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            ListGroupMembersError::ParseError(ref cause) => cause,
            ListGroupMembersError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by ListGroups
#[derive(Debug, PartialEq)]
pub enum ListGroupsError {
    /// <p>The identifier supplied for the entity is valid, but it does not exist in your organization.</p>
    EntityNotFound(String),
    /// <p>One or more of the input parameters don't match the service's restrictions.</p>
    InvalidParameter(String),
    /// <p>An operation received a valid organization identifier that either doesn't belong or exist in the system.</p>
    OrganizationNotFound(String),
    /// <p>The organization must have a valid state (Active or Synchronizing) to perform certain operations on the organization or its entities.</p>
    OrganizationState(String),
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

impl ListGroupsError {
    pub fn from_response(res: BufferedHttpResponse) -> ListGroupsError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "EntityNotFoundException" => {
                    return ListGroupsError::EntityNotFound(String::from(error_message));
                }
                "InvalidParameterException" => {
                    return ListGroupsError::InvalidParameter(String::from(error_message));
                }
                "OrganizationNotFoundException" => {
                    return ListGroupsError::OrganizationNotFound(String::from(error_message));
                }
                "OrganizationStateException" => {
                    return ListGroupsError::OrganizationState(String::from(error_message));
                }
                "ValidationException" => {
                    return ListGroupsError::Validation(error_message.to_string());
                }
                _ => {}
            }
        }
        return ListGroupsError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for ListGroupsError {
    fn from(err: serde_json::error::Error) -> ListGroupsError {
        ListGroupsError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for ListGroupsError {
    fn from(err: CredentialsError) -> ListGroupsError {
        ListGroupsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ListGroupsError {
    fn from(err: HttpDispatchError) -> ListGroupsError {
        ListGroupsError::HttpDispatch(err)
    }
}
impl From<io::Error> for ListGroupsError {
    fn from(err: io::Error) -> ListGroupsError {
        ListGroupsError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for ListGroupsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListGroupsError {
    fn description(&self) -> &str {
        match *self {
            ListGroupsError::EntityNotFound(ref cause) => cause,
            ListGroupsError::InvalidParameter(ref cause) => cause,
            ListGroupsError::OrganizationNotFound(ref cause) => cause,
            ListGroupsError::OrganizationState(ref cause) => cause,
            ListGroupsError::Validation(ref cause) => cause,
            ListGroupsError::Credentials(ref err) => err.description(),
            ListGroupsError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            ListGroupsError::ParseError(ref cause) => cause,
            ListGroupsError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by ListMailboxPermissions
#[derive(Debug, PartialEq)]
pub enum ListMailboxPermissionsError {
    /// <p>The identifier supplied for the entity is valid, but it does not exist in your organization.</p>
    EntityNotFound(String),
    /// <p>One or more of the input parameters don't match the service's restrictions.</p>
    InvalidParameter(String),
    /// <p>An operation received a valid organization identifier that either doesn't belong or exist in the system.</p>
    OrganizationNotFound(String),
    /// <p>The organization must have a valid state (Active or Synchronizing) to perform certain operations on the organization or its entities.</p>
    OrganizationState(String),
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

impl ListMailboxPermissionsError {
    pub fn from_response(res: BufferedHttpResponse) -> ListMailboxPermissionsError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "EntityNotFoundException" => {
                    return ListMailboxPermissionsError::EntityNotFound(String::from(error_message));
                }
                "InvalidParameterException" => {
                    return ListMailboxPermissionsError::InvalidParameter(String::from(
                        error_message,
                    ));
                }
                "OrganizationNotFoundException" => {
                    return ListMailboxPermissionsError::OrganizationNotFound(String::from(
                        error_message,
                    ));
                }
                "OrganizationStateException" => {
                    return ListMailboxPermissionsError::OrganizationState(String::from(
                        error_message,
                    ));
                }
                "ValidationException" => {
                    return ListMailboxPermissionsError::Validation(error_message.to_string());
                }
                _ => {}
            }
        }
        return ListMailboxPermissionsError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for ListMailboxPermissionsError {
    fn from(err: serde_json::error::Error) -> ListMailboxPermissionsError {
        ListMailboxPermissionsError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for ListMailboxPermissionsError {
    fn from(err: CredentialsError) -> ListMailboxPermissionsError {
        ListMailboxPermissionsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ListMailboxPermissionsError {
    fn from(err: HttpDispatchError) -> ListMailboxPermissionsError {
        ListMailboxPermissionsError::HttpDispatch(err)
    }
}
impl From<io::Error> for ListMailboxPermissionsError {
    fn from(err: io::Error) -> ListMailboxPermissionsError {
        ListMailboxPermissionsError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for ListMailboxPermissionsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListMailboxPermissionsError {
    fn description(&self) -> &str {
        match *self {
            ListMailboxPermissionsError::EntityNotFound(ref cause) => cause,
            ListMailboxPermissionsError::InvalidParameter(ref cause) => cause,
            ListMailboxPermissionsError::OrganizationNotFound(ref cause) => cause,
            ListMailboxPermissionsError::OrganizationState(ref cause) => cause,
            ListMailboxPermissionsError::Validation(ref cause) => cause,
            ListMailboxPermissionsError::Credentials(ref err) => err.description(),
            ListMailboxPermissionsError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            ListMailboxPermissionsError::ParseError(ref cause) => cause,
            ListMailboxPermissionsError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by ListOrganizations
#[derive(Debug, PartialEq)]
pub enum ListOrganizationsError {
    /// <p>One or more of the input parameters don't match the service's restrictions.</p>
    InvalidParameter(String),
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

impl ListOrganizationsError {
    pub fn from_response(res: BufferedHttpResponse) -> ListOrganizationsError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "InvalidParameterException" => {
                    return ListOrganizationsError::InvalidParameter(String::from(error_message));
                }
                "ValidationException" => {
                    return ListOrganizationsError::Validation(error_message.to_string());
                }
                _ => {}
            }
        }
        return ListOrganizationsError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for ListOrganizationsError {
    fn from(err: serde_json::error::Error) -> ListOrganizationsError {
        ListOrganizationsError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for ListOrganizationsError {
    fn from(err: CredentialsError) -> ListOrganizationsError {
        ListOrganizationsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ListOrganizationsError {
    fn from(err: HttpDispatchError) -> ListOrganizationsError {
        ListOrganizationsError::HttpDispatch(err)
    }
}
impl From<io::Error> for ListOrganizationsError {
    fn from(err: io::Error) -> ListOrganizationsError {
        ListOrganizationsError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for ListOrganizationsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListOrganizationsError {
    fn description(&self) -> &str {
        match *self {
            ListOrganizationsError::InvalidParameter(ref cause) => cause,
            ListOrganizationsError::Validation(ref cause) => cause,
            ListOrganizationsError::Credentials(ref err) => err.description(),
            ListOrganizationsError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            ListOrganizationsError::ParseError(ref cause) => cause,
            ListOrganizationsError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by ListResourceDelegates
#[derive(Debug, PartialEq)]
pub enum ListResourceDelegatesError {
    /// <p>The identifier supplied for the entity is valid, but it does not exist in your organization.</p>
    EntityNotFound(String),
    /// <p>You are performing an operation on an entity that isn't in the expected state, such as trying to update a deleted user.</p>
    EntityState(String),
    /// <p>One or more of the input parameters don't match the service's restrictions.</p>
    InvalidParameter(String),
    /// <p>An operation received a valid organization identifier that either doesn't belong or exist in the system.</p>
    OrganizationNotFound(String),
    /// <p>The organization must have a valid state (Active or Synchronizing) to perform certain operations on the organization or its entities.</p>
    OrganizationState(String),
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

impl ListResourceDelegatesError {
    pub fn from_response(res: BufferedHttpResponse) -> ListResourceDelegatesError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "EntityNotFoundException" => {
                    return ListResourceDelegatesError::EntityNotFound(String::from(error_message));
                }
                "EntityStateException" => {
                    return ListResourceDelegatesError::EntityState(String::from(error_message));
                }
                "InvalidParameterException" => {
                    return ListResourceDelegatesError::InvalidParameter(String::from(error_message));
                }
                "OrganizationNotFoundException" => {
                    return ListResourceDelegatesError::OrganizationNotFound(String::from(
                        error_message,
                    ));
                }
                "OrganizationStateException" => {
                    return ListResourceDelegatesError::OrganizationState(String::from(
                        error_message,
                    ));
                }
                "ValidationException" => {
                    return ListResourceDelegatesError::Validation(error_message.to_string());
                }
                _ => {}
            }
        }
        return ListResourceDelegatesError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for ListResourceDelegatesError {
    fn from(err: serde_json::error::Error) -> ListResourceDelegatesError {
        ListResourceDelegatesError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for ListResourceDelegatesError {
    fn from(err: CredentialsError) -> ListResourceDelegatesError {
        ListResourceDelegatesError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ListResourceDelegatesError {
    fn from(err: HttpDispatchError) -> ListResourceDelegatesError {
        ListResourceDelegatesError::HttpDispatch(err)
    }
}
impl From<io::Error> for ListResourceDelegatesError {
    fn from(err: io::Error) -> ListResourceDelegatesError {
        ListResourceDelegatesError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for ListResourceDelegatesError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListResourceDelegatesError {
    fn description(&self) -> &str {
        match *self {
            ListResourceDelegatesError::EntityNotFound(ref cause) => cause,
            ListResourceDelegatesError::EntityState(ref cause) => cause,
            ListResourceDelegatesError::InvalidParameter(ref cause) => cause,
            ListResourceDelegatesError::OrganizationNotFound(ref cause) => cause,
            ListResourceDelegatesError::OrganizationState(ref cause) => cause,
            ListResourceDelegatesError::Validation(ref cause) => cause,
            ListResourceDelegatesError::Credentials(ref err) => err.description(),
            ListResourceDelegatesError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            ListResourceDelegatesError::ParseError(ref cause) => cause,
            ListResourceDelegatesError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by ListResources
#[derive(Debug, PartialEq)]
pub enum ListResourcesError {
    /// <p>One or more of the input parameters don't match the service's restrictions.</p>
    InvalidParameter(String),
    /// <p>An operation received a valid organization identifier that either doesn't belong or exist in the system.</p>
    OrganizationNotFound(String),
    /// <p>The organization must have a valid state (Active or Synchronizing) to perform certain operations on the organization or its entities.</p>
    OrganizationState(String),
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
                "InvalidParameterException" => {
                    return ListResourcesError::InvalidParameter(String::from(error_message));
                }
                "OrganizationNotFoundException" => {
                    return ListResourcesError::OrganizationNotFound(String::from(error_message));
                }
                "OrganizationStateException" => {
                    return ListResourcesError::OrganizationState(String::from(error_message));
                }
                "ValidationException" => {
                    return ListResourcesError::Validation(error_message.to_string());
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
            ListResourcesError::InvalidParameter(ref cause) => cause,
            ListResourcesError::OrganizationNotFound(ref cause) => cause,
            ListResourcesError::OrganizationState(ref cause) => cause,
            ListResourcesError::Validation(ref cause) => cause,
            ListResourcesError::Credentials(ref err) => err.description(),
            ListResourcesError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            ListResourcesError::ParseError(ref cause) => cause,
            ListResourcesError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by ListUsers
#[derive(Debug, PartialEq)]
pub enum ListUsersError {
    /// <p>One or more of the input parameters don't match the service's restrictions.</p>
    InvalidParameter(String),
    /// <p>An operation received a valid organization identifier that either doesn't belong or exist in the system.</p>
    OrganizationNotFound(String),
    /// <p>The organization must have a valid state (Active or Synchronizing) to perform certain operations on the organization or its entities.</p>
    OrganizationState(String),
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

impl ListUsersError {
    pub fn from_response(res: BufferedHttpResponse) -> ListUsersError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "InvalidParameterException" => {
                    return ListUsersError::InvalidParameter(String::from(error_message));
                }
                "OrganizationNotFoundException" => {
                    return ListUsersError::OrganizationNotFound(String::from(error_message));
                }
                "OrganizationStateException" => {
                    return ListUsersError::OrganizationState(String::from(error_message));
                }
                "ValidationException" => {
                    return ListUsersError::Validation(error_message.to_string());
                }
                _ => {}
            }
        }
        return ListUsersError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for ListUsersError {
    fn from(err: serde_json::error::Error) -> ListUsersError {
        ListUsersError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for ListUsersError {
    fn from(err: CredentialsError) -> ListUsersError {
        ListUsersError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ListUsersError {
    fn from(err: HttpDispatchError) -> ListUsersError {
        ListUsersError::HttpDispatch(err)
    }
}
impl From<io::Error> for ListUsersError {
    fn from(err: io::Error) -> ListUsersError {
        ListUsersError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for ListUsersError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListUsersError {
    fn description(&self) -> &str {
        match *self {
            ListUsersError::InvalidParameter(ref cause) => cause,
            ListUsersError::OrganizationNotFound(ref cause) => cause,
            ListUsersError::OrganizationState(ref cause) => cause,
            ListUsersError::Validation(ref cause) => cause,
            ListUsersError::Credentials(ref err) => err.description(),
            ListUsersError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            ListUsersError::ParseError(ref cause) => cause,
            ListUsersError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by PutMailboxPermissions
#[derive(Debug, PartialEq)]
pub enum PutMailboxPermissionsError {
    /// <p>The identifier supplied for the entity is valid, but it does not exist in your organization.</p>
    EntityNotFound(String),
    /// <p>You are performing an operation on an entity that isn't in the expected state, such as trying to update a deleted user.</p>
    EntityState(String),
    /// <p>One or more of the input parameters don't match the service's restrictions.</p>
    InvalidParameter(String),
    /// <p>An operation received a valid organization identifier that either doesn't belong or exist in the system.</p>
    OrganizationNotFound(String),
    /// <p>The organization must have a valid state (Active or Synchronizing) to perform certain operations on the organization or its entities.</p>
    OrganizationState(String),
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

impl PutMailboxPermissionsError {
    pub fn from_response(res: BufferedHttpResponse) -> PutMailboxPermissionsError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "EntityNotFoundException" => {
                    return PutMailboxPermissionsError::EntityNotFound(String::from(error_message));
                }
                "EntityStateException" => {
                    return PutMailboxPermissionsError::EntityState(String::from(error_message));
                }
                "InvalidParameterException" => {
                    return PutMailboxPermissionsError::InvalidParameter(String::from(error_message));
                }
                "OrganizationNotFoundException" => {
                    return PutMailboxPermissionsError::OrganizationNotFound(String::from(
                        error_message,
                    ));
                }
                "OrganizationStateException" => {
                    return PutMailboxPermissionsError::OrganizationState(String::from(
                        error_message,
                    ));
                }
                "ValidationException" => {
                    return PutMailboxPermissionsError::Validation(error_message.to_string());
                }
                _ => {}
            }
        }
        return PutMailboxPermissionsError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for PutMailboxPermissionsError {
    fn from(err: serde_json::error::Error) -> PutMailboxPermissionsError {
        PutMailboxPermissionsError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for PutMailboxPermissionsError {
    fn from(err: CredentialsError) -> PutMailboxPermissionsError {
        PutMailboxPermissionsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for PutMailboxPermissionsError {
    fn from(err: HttpDispatchError) -> PutMailboxPermissionsError {
        PutMailboxPermissionsError::HttpDispatch(err)
    }
}
impl From<io::Error> for PutMailboxPermissionsError {
    fn from(err: io::Error) -> PutMailboxPermissionsError {
        PutMailboxPermissionsError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for PutMailboxPermissionsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for PutMailboxPermissionsError {
    fn description(&self) -> &str {
        match *self {
            PutMailboxPermissionsError::EntityNotFound(ref cause) => cause,
            PutMailboxPermissionsError::EntityState(ref cause) => cause,
            PutMailboxPermissionsError::InvalidParameter(ref cause) => cause,
            PutMailboxPermissionsError::OrganizationNotFound(ref cause) => cause,
            PutMailboxPermissionsError::OrganizationState(ref cause) => cause,
            PutMailboxPermissionsError::Validation(ref cause) => cause,
            PutMailboxPermissionsError::Credentials(ref err) => err.description(),
            PutMailboxPermissionsError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            PutMailboxPermissionsError::ParseError(ref cause) => cause,
            PutMailboxPermissionsError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by RegisterToWorkMail
#[derive(Debug, PartialEq)]
pub enum RegisterToWorkMailError {
    /// <p>The Directory Service doesn't recognize the credentials supplied by the Amazon WorkMail service.</p>
    DirectoryServiceAuthenticationFailed(String),
    /// <p>The directory that you are trying to perform operations on isn't available.</p>
    DirectoryUnavailable(String),
    /// <p>The email address that you're trying to assign is already created for a different user, group, or resource.</p>
    EmailAddressInUse(String),
    /// <p>The user, group, or resource that you're trying to register is already registered.</p>
    EntityAlreadyRegistered(String),
    /// <p>The identifier supplied for the entity is valid, but it does not exist in your organization.</p>
    EntityNotFound(String),
    /// <p>You are performing an operation on an entity that isn't in the expected state, such as trying to update a deleted user.</p>
    EntityState(String),
    /// <p>One or more of the input parameters don't match the service's restrictions.</p>
    InvalidParameter(String),
    /// <p>For an email or alias to be created in Amazon WorkMail, the included domain must be defined in the organization.</p>
    MailDomainNotFound(String),
    /// <p>After a domain has been added to the organization, it must be verified. The domain is not yet verified.</p>
    MailDomainState(String),
    /// <p>An operation received a valid organization identifier that either doesn't belong or exist in the system.</p>
    OrganizationNotFound(String),
    /// <p>The organization must have a valid state (Active or Synchronizing) to perform certain operations on the organization or its entities.</p>
    OrganizationState(String),
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

impl RegisterToWorkMailError {
    pub fn from_response(res: BufferedHttpResponse) -> RegisterToWorkMailError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "DirectoryServiceAuthenticationFailedException" => {
                    return RegisterToWorkMailError::DirectoryServiceAuthenticationFailed(
                        String::from(error_message),
                    );
                }
                "DirectoryUnavailableException" => {
                    return RegisterToWorkMailError::DirectoryUnavailable(String::from(
                        error_message,
                    ));
                }
                "EmailAddressInUseException" => {
                    return RegisterToWorkMailError::EmailAddressInUse(String::from(error_message));
                }
                "EntityAlreadyRegisteredException" => {
                    return RegisterToWorkMailError::EntityAlreadyRegistered(String::from(
                        error_message,
                    ));
                }
                "EntityNotFoundException" => {
                    return RegisterToWorkMailError::EntityNotFound(String::from(error_message));
                }
                "EntityStateException" => {
                    return RegisterToWorkMailError::EntityState(String::from(error_message));
                }
                "InvalidParameterException" => {
                    return RegisterToWorkMailError::InvalidParameter(String::from(error_message));
                }
                "MailDomainNotFoundException" => {
                    return RegisterToWorkMailError::MailDomainNotFound(String::from(error_message));
                }
                "MailDomainStateException" => {
                    return RegisterToWorkMailError::MailDomainState(String::from(error_message));
                }
                "OrganizationNotFoundException" => {
                    return RegisterToWorkMailError::OrganizationNotFound(String::from(
                        error_message,
                    ));
                }
                "OrganizationStateException" => {
                    return RegisterToWorkMailError::OrganizationState(String::from(error_message));
                }
                "ValidationException" => {
                    return RegisterToWorkMailError::Validation(error_message.to_string());
                }
                _ => {}
            }
        }
        return RegisterToWorkMailError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for RegisterToWorkMailError {
    fn from(err: serde_json::error::Error) -> RegisterToWorkMailError {
        RegisterToWorkMailError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for RegisterToWorkMailError {
    fn from(err: CredentialsError) -> RegisterToWorkMailError {
        RegisterToWorkMailError::Credentials(err)
    }
}
impl From<HttpDispatchError> for RegisterToWorkMailError {
    fn from(err: HttpDispatchError) -> RegisterToWorkMailError {
        RegisterToWorkMailError::HttpDispatch(err)
    }
}
impl From<io::Error> for RegisterToWorkMailError {
    fn from(err: io::Error) -> RegisterToWorkMailError {
        RegisterToWorkMailError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for RegisterToWorkMailError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for RegisterToWorkMailError {
    fn description(&self) -> &str {
        match *self {
            RegisterToWorkMailError::DirectoryServiceAuthenticationFailed(ref cause) => cause,
            RegisterToWorkMailError::DirectoryUnavailable(ref cause) => cause,
            RegisterToWorkMailError::EmailAddressInUse(ref cause) => cause,
            RegisterToWorkMailError::EntityAlreadyRegistered(ref cause) => cause,
            RegisterToWorkMailError::EntityNotFound(ref cause) => cause,
            RegisterToWorkMailError::EntityState(ref cause) => cause,
            RegisterToWorkMailError::InvalidParameter(ref cause) => cause,
            RegisterToWorkMailError::MailDomainNotFound(ref cause) => cause,
            RegisterToWorkMailError::MailDomainState(ref cause) => cause,
            RegisterToWorkMailError::OrganizationNotFound(ref cause) => cause,
            RegisterToWorkMailError::OrganizationState(ref cause) => cause,
            RegisterToWorkMailError::Validation(ref cause) => cause,
            RegisterToWorkMailError::Credentials(ref err) => err.description(),
            RegisterToWorkMailError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            RegisterToWorkMailError::ParseError(ref cause) => cause,
            RegisterToWorkMailError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by ResetPassword
#[derive(Debug, PartialEq)]
pub enum ResetPasswordError {
    /// <p>The Directory Service doesn't recognize the credentials supplied by the Amazon WorkMail service.</p>
    DirectoryServiceAuthenticationFailed(String),
    /// <p>The directory that you are trying to perform operations on isn't available.</p>
    DirectoryUnavailable(String),
    /// <p>The identifier supplied for the entity is valid, but it does not exist in your organization.</p>
    EntityNotFound(String),
    /// <p>You are performing an operation on an entity that isn't in the expected state, such as trying to update a deleted user.</p>
    EntityState(String),
    /// <p>One or more of the input parameters don't match the service's restrictions.</p>
    InvalidParameter(String),
    /// <p>The supplied password doesn't match the minimum security constraints, such as length or use of special characters.</p>
    InvalidPassword(String),
    /// <p>An operation received a valid organization identifier that either doesn't belong or exist in the system.</p>
    OrganizationNotFound(String),
    /// <p>The organization must have a valid state (Active or Synchronizing) to perform certain operations on the organization or its entities.</p>
    OrganizationState(String),
    /// <p>You can't perform a write operation against a read-only directory.</p>
    UnsupportedOperation(String),
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

impl ResetPasswordError {
    pub fn from_response(res: BufferedHttpResponse) -> ResetPasswordError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "DirectoryServiceAuthenticationFailedException" => {
                    return ResetPasswordError::DirectoryServiceAuthenticationFailed(String::from(
                        error_message,
                    ));
                }
                "DirectoryUnavailableException" => {
                    return ResetPasswordError::DirectoryUnavailable(String::from(error_message));
                }
                "EntityNotFoundException" => {
                    return ResetPasswordError::EntityNotFound(String::from(error_message));
                }
                "EntityStateException" => {
                    return ResetPasswordError::EntityState(String::from(error_message));
                }
                "InvalidParameterException" => {
                    return ResetPasswordError::InvalidParameter(String::from(error_message));
                }
                "InvalidPasswordException" => {
                    return ResetPasswordError::InvalidPassword(String::from(error_message));
                }
                "OrganizationNotFoundException" => {
                    return ResetPasswordError::OrganizationNotFound(String::from(error_message));
                }
                "OrganizationStateException" => {
                    return ResetPasswordError::OrganizationState(String::from(error_message));
                }
                "UnsupportedOperationException" => {
                    return ResetPasswordError::UnsupportedOperation(String::from(error_message));
                }
                "ValidationException" => {
                    return ResetPasswordError::Validation(error_message.to_string());
                }
                _ => {}
            }
        }
        return ResetPasswordError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for ResetPasswordError {
    fn from(err: serde_json::error::Error) -> ResetPasswordError {
        ResetPasswordError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for ResetPasswordError {
    fn from(err: CredentialsError) -> ResetPasswordError {
        ResetPasswordError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ResetPasswordError {
    fn from(err: HttpDispatchError) -> ResetPasswordError {
        ResetPasswordError::HttpDispatch(err)
    }
}
impl From<io::Error> for ResetPasswordError {
    fn from(err: io::Error) -> ResetPasswordError {
        ResetPasswordError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for ResetPasswordError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ResetPasswordError {
    fn description(&self) -> &str {
        match *self {
            ResetPasswordError::DirectoryServiceAuthenticationFailed(ref cause) => cause,
            ResetPasswordError::DirectoryUnavailable(ref cause) => cause,
            ResetPasswordError::EntityNotFound(ref cause) => cause,
            ResetPasswordError::EntityState(ref cause) => cause,
            ResetPasswordError::InvalidParameter(ref cause) => cause,
            ResetPasswordError::InvalidPassword(ref cause) => cause,
            ResetPasswordError::OrganizationNotFound(ref cause) => cause,
            ResetPasswordError::OrganizationState(ref cause) => cause,
            ResetPasswordError::UnsupportedOperation(ref cause) => cause,
            ResetPasswordError::Validation(ref cause) => cause,
            ResetPasswordError::Credentials(ref err) => err.description(),
            ResetPasswordError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            ResetPasswordError::ParseError(ref cause) => cause,
            ResetPasswordError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by UpdatePrimaryEmailAddress
#[derive(Debug, PartialEq)]
pub enum UpdatePrimaryEmailAddressError {
    /// <p>The Directory Service doesn't recognize the credentials supplied by the Amazon WorkMail service.</p>
    DirectoryServiceAuthenticationFailed(String),
    /// <p>The directory that you are trying to perform operations on isn't available.</p>
    DirectoryUnavailable(String),
    /// <p>The email address that you're trying to assign is already created for a different user, group, or resource.</p>
    EmailAddressInUse(String),
    /// <p>The identifier supplied for the entity is valid, but it does not exist in your organization.</p>
    EntityNotFound(String),
    /// <p>You are performing an operation on an entity that isn't in the expected state, such as trying to update a deleted user.</p>
    EntityState(String),
    /// <p>One or more of the input parameters don't match the service's restrictions.</p>
    InvalidParameter(String),
    /// <p>For an email or alias to be created in Amazon WorkMail, the included domain must be defined in the organization.</p>
    MailDomainNotFound(String),
    /// <p>After a domain has been added to the organization, it must be verified. The domain is not yet verified.</p>
    MailDomainState(String),
    /// <p>An operation received a valid organization identifier that either doesn't belong or exist in the system.</p>
    OrganizationNotFound(String),
    /// <p>The organization must have a valid state (Active or Synchronizing) to perform certain operations on the organization or its entities.</p>
    OrganizationState(String),
    /// <p>You can't perform a write operation against a read-only directory.</p>
    UnsupportedOperation(String),
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

impl UpdatePrimaryEmailAddressError {
    pub fn from_response(res: BufferedHttpResponse) -> UpdatePrimaryEmailAddressError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "DirectoryServiceAuthenticationFailedException" => {
                    return UpdatePrimaryEmailAddressError::DirectoryServiceAuthenticationFailed(
                        String::from(error_message),
                    );
                }
                "DirectoryUnavailableException" => {
                    return UpdatePrimaryEmailAddressError::DirectoryUnavailable(String::from(
                        error_message,
                    ));
                }
                "EmailAddressInUseException" => {
                    return UpdatePrimaryEmailAddressError::EmailAddressInUse(String::from(
                        error_message,
                    ));
                }
                "EntityNotFoundException" => {
                    return UpdatePrimaryEmailAddressError::EntityNotFound(String::from(
                        error_message,
                    ));
                }
                "EntityStateException" => {
                    return UpdatePrimaryEmailAddressError::EntityState(String::from(error_message));
                }
                "InvalidParameterException" => {
                    return UpdatePrimaryEmailAddressError::InvalidParameter(String::from(
                        error_message,
                    ));
                }
                "MailDomainNotFoundException" => {
                    return UpdatePrimaryEmailAddressError::MailDomainNotFound(String::from(
                        error_message,
                    ));
                }
                "MailDomainStateException" => {
                    return UpdatePrimaryEmailAddressError::MailDomainState(String::from(
                        error_message,
                    ));
                }
                "OrganizationNotFoundException" => {
                    return UpdatePrimaryEmailAddressError::OrganizationNotFound(String::from(
                        error_message,
                    ));
                }
                "OrganizationStateException" => {
                    return UpdatePrimaryEmailAddressError::OrganizationState(String::from(
                        error_message,
                    ));
                }
                "UnsupportedOperationException" => {
                    return UpdatePrimaryEmailAddressError::UnsupportedOperation(String::from(
                        error_message,
                    ));
                }
                "ValidationException" => {
                    return UpdatePrimaryEmailAddressError::Validation(error_message.to_string());
                }
                _ => {}
            }
        }
        return UpdatePrimaryEmailAddressError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for UpdatePrimaryEmailAddressError {
    fn from(err: serde_json::error::Error) -> UpdatePrimaryEmailAddressError {
        UpdatePrimaryEmailAddressError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for UpdatePrimaryEmailAddressError {
    fn from(err: CredentialsError) -> UpdatePrimaryEmailAddressError {
        UpdatePrimaryEmailAddressError::Credentials(err)
    }
}
impl From<HttpDispatchError> for UpdatePrimaryEmailAddressError {
    fn from(err: HttpDispatchError) -> UpdatePrimaryEmailAddressError {
        UpdatePrimaryEmailAddressError::HttpDispatch(err)
    }
}
impl From<io::Error> for UpdatePrimaryEmailAddressError {
    fn from(err: io::Error) -> UpdatePrimaryEmailAddressError {
        UpdatePrimaryEmailAddressError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for UpdatePrimaryEmailAddressError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for UpdatePrimaryEmailAddressError {
    fn description(&self) -> &str {
        match *self {
            UpdatePrimaryEmailAddressError::DirectoryServiceAuthenticationFailed(ref cause) => {
                cause
            }
            UpdatePrimaryEmailAddressError::DirectoryUnavailable(ref cause) => cause,
            UpdatePrimaryEmailAddressError::EmailAddressInUse(ref cause) => cause,
            UpdatePrimaryEmailAddressError::EntityNotFound(ref cause) => cause,
            UpdatePrimaryEmailAddressError::EntityState(ref cause) => cause,
            UpdatePrimaryEmailAddressError::InvalidParameter(ref cause) => cause,
            UpdatePrimaryEmailAddressError::MailDomainNotFound(ref cause) => cause,
            UpdatePrimaryEmailAddressError::MailDomainState(ref cause) => cause,
            UpdatePrimaryEmailAddressError::OrganizationNotFound(ref cause) => cause,
            UpdatePrimaryEmailAddressError::OrganizationState(ref cause) => cause,
            UpdatePrimaryEmailAddressError::UnsupportedOperation(ref cause) => cause,
            UpdatePrimaryEmailAddressError::Validation(ref cause) => cause,
            UpdatePrimaryEmailAddressError::Credentials(ref err) => err.description(),
            UpdatePrimaryEmailAddressError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            UpdatePrimaryEmailAddressError::ParseError(ref cause) => cause,
            UpdatePrimaryEmailAddressError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by UpdateResource
#[derive(Debug, PartialEq)]
pub enum UpdateResourceError {
    /// <p>The directory that you are trying to perform operations on isn't available.</p>
    DirectoryUnavailable(String),
    /// <p>The email address that you're trying to assign is already created for a different user, group, or resource.</p>
    EmailAddressInUse(String),
    /// <p>The identifier supplied for the entity is valid, but it does not exist in your organization.</p>
    EntityNotFound(String),
    /// <p>You are performing an operation on an entity that isn't in the expected state, such as trying to update a deleted user.</p>
    EntityState(String),
    /// <p>The configuration for a resource isn't valid. A resource must either be able to auto-respond to requests or have at least one delegate associated that can do it on its behalf.</p>
    InvalidConfiguration(String),
    /// <p>For an email or alias to be created in Amazon WorkMail, the included domain must be defined in the organization.</p>
    MailDomainNotFound(String),
    /// <p>After a domain has been added to the organization, it must be verified. The domain is not yet verified.</p>
    MailDomainState(String),
    /// <p>The entity (user, group, or user) name isn't unique in Amazon WorkMail.</p>
    NameAvailability(String),
    /// <p>An operation received a valid organization identifier that either doesn't belong or exist in the system.</p>
    OrganizationNotFound(String),
    /// <p>The organization must have a valid state (Active or Synchronizing) to perform certain operations on the organization or its entities.</p>
    OrganizationState(String),
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

impl UpdateResourceError {
    pub fn from_response(res: BufferedHttpResponse) -> UpdateResourceError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "DirectoryUnavailableException" => {
                    return UpdateResourceError::DirectoryUnavailable(String::from(error_message));
                }
                "EmailAddressInUseException" => {
                    return UpdateResourceError::EmailAddressInUse(String::from(error_message));
                }
                "EntityNotFoundException" => {
                    return UpdateResourceError::EntityNotFound(String::from(error_message));
                }
                "EntityStateException" => {
                    return UpdateResourceError::EntityState(String::from(error_message));
                }
                "InvalidConfigurationException" => {
                    return UpdateResourceError::InvalidConfiguration(String::from(error_message));
                }
                "MailDomainNotFoundException" => {
                    return UpdateResourceError::MailDomainNotFound(String::from(error_message));
                }
                "MailDomainStateException" => {
                    return UpdateResourceError::MailDomainState(String::from(error_message));
                }
                "NameAvailabilityException" => {
                    return UpdateResourceError::NameAvailability(String::from(error_message));
                }
                "OrganizationNotFoundException" => {
                    return UpdateResourceError::OrganizationNotFound(String::from(error_message));
                }
                "OrganizationStateException" => {
                    return UpdateResourceError::OrganizationState(String::from(error_message));
                }
                "ValidationException" => {
                    return UpdateResourceError::Validation(error_message.to_string());
                }
                _ => {}
            }
        }
        return UpdateResourceError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for UpdateResourceError {
    fn from(err: serde_json::error::Error) -> UpdateResourceError {
        UpdateResourceError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for UpdateResourceError {
    fn from(err: CredentialsError) -> UpdateResourceError {
        UpdateResourceError::Credentials(err)
    }
}
impl From<HttpDispatchError> for UpdateResourceError {
    fn from(err: HttpDispatchError) -> UpdateResourceError {
        UpdateResourceError::HttpDispatch(err)
    }
}
impl From<io::Error> for UpdateResourceError {
    fn from(err: io::Error) -> UpdateResourceError {
        UpdateResourceError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for UpdateResourceError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for UpdateResourceError {
    fn description(&self) -> &str {
        match *self {
            UpdateResourceError::DirectoryUnavailable(ref cause) => cause,
            UpdateResourceError::EmailAddressInUse(ref cause) => cause,
            UpdateResourceError::EntityNotFound(ref cause) => cause,
            UpdateResourceError::EntityState(ref cause) => cause,
            UpdateResourceError::InvalidConfiguration(ref cause) => cause,
            UpdateResourceError::MailDomainNotFound(ref cause) => cause,
            UpdateResourceError::MailDomainState(ref cause) => cause,
            UpdateResourceError::NameAvailability(ref cause) => cause,
            UpdateResourceError::OrganizationNotFound(ref cause) => cause,
            UpdateResourceError::OrganizationState(ref cause) => cause,
            UpdateResourceError::Validation(ref cause) => cause,
            UpdateResourceError::Credentials(ref err) => err.description(),
            UpdateResourceError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            UpdateResourceError::ParseError(ref cause) => cause,
            UpdateResourceError::Unknown(_) => "unknown error",
        }
    }
}
/// Trait representing the capabilities of the Amazon WorkMail API. Amazon WorkMail clients implement this trait.
pub trait Workmail {
    /// <p>Adds a member to the resource's set of delegates.</p>
    fn associate_delegate_to_resource(
        &self,
        input: AssociateDelegateToResourceRequest,
    ) -> RusotoFuture<AssociateDelegateToResourceResponse, AssociateDelegateToResourceError>;

    /// <p>Adds a member to the group's set.</p>
    fn associate_member_to_group(
        &self,
        input: AssociateMemberToGroupRequest,
    ) -> RusotoFuture<AssociateMemberToGroupResponse, AssociateMemberToGroupError>;

    /// <p>Adds an alias to the set of a given member of Amazon WorkMail.</p>
    fn create_alias(
        &self,
        input: CreateAliasRequest,
    ) -> RusotoFuture<CreateAliasResponse, CreateAliasError>;

    /// <p>Creates a group that can be used in Amazon WorkMail by calling the RegisterToWorkMail operation.</p>
    fn create_group(
        &self,
        input: CreateGroupRequest,
    ) -> RusotoFuture<CreateGroupResponse, CreateGroupError>;

    /// <p>Creates a new Amazon WorkMail resource. The available types are equipment and room.</p>
    fn create_resource(
        &self,
        input: CreateResourceRequest,
    ) -> RusotoFuture<CreateResourceResponse, CreateResourceError>;

    /// <p>Creates a user who can be used in Amazon WorkMail by calling the RegisterToWorkMail operation.</p>
    fn create_user(
        &self,
        input: CreateUserRequest,
    ) -> RusotoFuture<CreateUserResponse, CreateUserError>;

    /// <p>Remove the alias from a set of aliases for a given user.</p>
    fn delete_alias(
        &self,
        input: DeleteAliasRequest,
    ) -> RusotoFuture<DeleteAliasResponse, DeleteAliasError>;

    /// <p>Deletes a group from Amazon WorkMail.</p>
    fn delete_group(
        &self,
        input: DeleteGroupRequest,
    ) -> RusotoFuture<DeleteGroupResponse, DeleteGroupError>;

    /// <p>Deletes permissions granted to a user or group.</p>
    fn delete_mailbox_permissions(
        &self,
        input: DeleteMailboxPermissionsRequest,
    ) -> RusotoFuture<DeleteMailboxPermissionsResponse, DeleteMailboxPermissionsError>;

    /// <p>Deletes the specified resource. </p>
    fn delete_resource(
        &self,
        input: DeleteResourceRequest,
    ) -> RusotoFuture<DeleteResourceResponse, DeleteResourceError>;

    /// <p>Deletes a user from Amazon WorkMail and all subsequent systems. The action can't be undone. The mailbox is kept as-is for a minimum of 30 days, without any means to restore it. </p>
    fn delete_user(
        &self,
        input: DeleteUserRequest,
    ) -> RusotoFuture<DeleteUserResponse, DeleteUserError>;

    /// <p>Mark a user, group, or resource as no longer used in Amazon WorkMail. This action disassociates the mailbox and schedules it for clean-up. Amazon WorkMail keeps mailboxes for 30 days before they are permanently removed. The functionality in the console is <i>Disable</i>.</p>
    fn deregister_from_work_mail(
        &self,
        input: DeregisterFromWorkMailRequest,
    ) -> RusotoFuture<DeregisterFromWorkMailResponse, DeregisterFromWorkMailError>;

    /// <p>Returns the data available for the group.</p>
    fn describe_group(
        &self,
        input: DescribeGroupRequest,
    ) -> RusotoFuture<DescribeGroupResponse, DescribeGroupError>;

    /// <p>Provides more information regarding a given organization based on its identifier.</p>
    fn describe_organization(
        &self,
        input: DescribeOrganizationRequest,
    ) -> RusotoFuture<DescribeOrganizationResponse, DescribeOrganizationError>;

    /// <p>Returns the data available for the resource.</p>
    fn describe_resource(
        &self,
        input: DescribeResourceRequest,
    ) -> RusotoFuture<DescribeResourceResponse, DescribeResourceError>;

    /// <p>Provides information regarding the user.</p>
    fn describe_user(
        &self,
        input: DescribeUserRequest,
    ) -> RusotoFuture<DescribeUserResponse, DescribeUserError>;

    /// <p>Removes a member from the resource's set of delegates.</p>
    fn disassociate_delegate_from_resource(
        &self,
        input: DisassociateDelegateFromResourceRequest,
    ) -> RusotoFuture<DisassociateDelegateFromResourceResponse, DisassociateDelegateFromResourceError>;

    /// <p>Removes a member from a group.</p>
    fn disassociate_member_from_group(
        &self,
        input: DisassociateMemberFromGroupRequest,
    ) -> RusotoFuture<DisassociateMemberFromGroupResponse, DisassociateMemberFromGroupError>;

    /// <p>Creates a paginated call to list the aliases associated with a given entity.</p>
    fn list_aliases(
        &self,
        input: ListAliasesRequest,
    ) -> RusotoFuture<ListAliasesResponse, ListAliasesError>;

    /// <p>Returns an overview of the members of a group.</p>
    fn list_group_members(
        &self,
        input: ListGroupMembersRequest,
    ) -> RusotoFuture<ListGroupMembersResponse, ListGroupMembersError>;

    /// <p>Returns summaries of the organization's groups.</p>
    fn list_groups(
        &self,
        input: ListGroupsRequest,
    ) -> RusotoFuture<ListGroupsResponse, ListGroupsError>;

    /// <p>Lists the mailbox permissions associated with a mailbox.</p>
    fn list_mailbox_permissions(
        &self,
        input: ListMailboxPermissionsRequest,
    ) -> RusotoFuture<ListMailboxPermissionsResponse, ListMailboxPermissionsError>;

    /// <p>Returns summaries of the customer's non-deleted organizations.</p>
    fn list_organizations(
        &self,
        input: ListOrganizationsRequest,
    ) -> RusotoFuture<ListOrganizationsResponse, ListOrganizationsError>;

    /// <p>Lists the delegates associated with a resource. Users and groups can be resource delegates and answer requests on behalf of the resource.</p>
    fn list_resource_delegates(
        &self,
        input: ListResourceDelegatesRequest,
    ) -> RusotoFuture<ListResourceDelegatesResponse, ListResourceDelegatesError>;

    /// <p>Returns summaries of the organization's resources.</p>
    fn list_resources(
        &self,
        input: ListResourcesRequest,
    ) -> RusotoFuture<ListResourcesResponse, ListResourcesError>;

    /// <p>Returns summaries of the organization's users.</p>
    fn list_users(
        &self,
        input: ListUsersRequest,
    ) -> RusotoFuture<ListUsersResponse, ListUsersError>;

    /// <p>Sets permissions for a user or group. This replaces any pre-existing permissions set for the entity.</p>
    fn put_mailbox_permissions(
        &self,
        input: PutMailboxPermissionsRequest,
    ) -> RusotoFuture<PutMailboxPermissionsResponse, PutMailboxPermissionsError>;

    /// <p>Registers an existing and disabled user, group, or resource/entity for Amazon WorkMail use by associating a mailbox and calendaring capabilities. It performs no change if the entity is enabled and fails if the entity is deleted. This operation results in the accumulation of costs. For more information, see <a href="http://aws.amazon.com/workmail/pricing">Pricing</a>. The equivalent console functionality for this operation is <i>Enable</i>. Users can either be created by calling the CreateUser API or they can be synchronized from your directory. For more information, see DeregisterFromWorkMail.</p>
    fn register_to_work_mail(
        &self,
        input: RegisterToWorkMailRequest,
    ) -> RusotoFuture<RegisterToWorkMailResponse, RegisterToWorkMailError>;

    /// <p>Allows the administrator to reset the password for a user.</p>
    fn reset_password(
        &self,
        input: ResetPasswordRequest,
    ) -> RusotoFuture<ResetPasswordResponse, ResetPasswordError>;

    /// <p>Updates the primary email for an entity. The current email is moved into the list of aliases (or swapped between an existing alias and the current primary email) and the email provided in the input is promoted as the primary.</p>
    fn update_primary_email_address(
        &self,
        input: UpdatePrimaryEmailAddressRequest,
    ) -> RusotoFuture<UpdatePrimaryEmailAddressResponse, UpdatePrimaryEmailAddressError>;

    /// <p>Updates data for the resource. It must be preceded by a describe call in order to have the latest information. The dataset in the request should be the one expected when performing another describe call.</p>
    fn update_resource(
        &self,
        input: UpdateResourceRequest,
    ) -> RusotoFuture<UpdateResourceResponse, UpdateResourceError>;
}
/// A client for the Amazon WorkMail API.
pub struct WorkmailClient {
    client: Client,
    region: region::Region,
}

impl WorkmailClient {
    /// Creates a client backed by the default tokio event loop.
    ///
    /// The client will use the default credentials provider and tls client.
    pub fn new(region: region::Region) -> WorkmailClient {
        WorkmailClient {
            client: Client::shared(),
            region: region,
        }
    }

    pub fn new_with<P, D>(
        request_dispatcher: D,
        credentials_provider: P,
        region: region::Region,
    ) -> WorkmailClient
    where
        P: ProvideAwsCredentials + Send + Sync + 'static,
        P::Future: Send,
        D: DispatchSignedRequest + Send + Sync + 'static,
        D::Future: Send,
    {
        WorkmailClient {
            client: Client::new_with(credentials_provider, request_dispatcher),
            region: region,
        }
    }
}

impl Workmail for WorkmailClient {
    /// <p>Adds a member to the resource's set of delegates.</p>
    fn associate_delegate_to_resource(
        &self,
        input: AssociateDelegateToResourceRequest,
    ) -> RusotoFuture<AssociateDelegateToResourceResponse, AssociateDelegateToResourceError> {
        let mut request = SignedRequest::new("POST", "workmail", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "WorkMailService.AssociateDelegateToResource",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<AssociateDelegateToResourceResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(AssociateDelegateToResourceError::from_response(response))
                }))
            }
        })
    }

    /// <p>Adds a member to the group's set.</p>
    fn associate_member_to_group(
        &self,
        input: AssociateMemberToGroupRequest,
    ) -> RusotoFuture<AssociateMemberToGroupResponse, AssociateMemberToGroupError> {
        let mut request = SignedRequest::new("POST", "workmail", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "WorkMailService.AssociateMemberToGroup");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<AssociateMemberToGroupResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(AssociateMemberToGroupError::from_response(response))
                    }),
                )
            }
        })
    }

    /// <p>Adds an alias to the set of a given member of Amazon WorkMail.</p>
    fn create_alias(
        &self,
        input: CreateAliasRequest,
    ) -> RusotoFuture<CreateAliasResponse, CreateAliasError> {
        let mut request = SignedRequest::new("POST", "workmail", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "WorkMailService.CreateAlias");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<CreateAliasResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(CreateAliasError::from_response(response))),
                )
            }
        })
    }

    /// <p>Creates a group that can be used in Amazon WorkMail by calling the RegisterToWorkMail operation.</p>
    fn create_group(
        &self,
        input: CreateGroupRequest,
    ) -> RusotoFuture<CreateGroupResponse, CreateGroupError> {
        let mut request = SignedRequest::new("POST", "workmail", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "WorkMailService.CreateGroup");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<CreateGroupResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(CreateGroupError::from_response(response))),
                )
            }
        })
    }

    /// <p>Creates a new Amazon WorkMail resource. The available types are equipment and room.</p>
    fn create_resource(
        &self,
        input: CreateResourceRequest,
    ) -> RusotoFuture<CreateResourceResponse, CreateResourceError> {
        let mut request = SignedRequest::new("POST", "workmail", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "WorkMailService.CreateResource");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<CreateResourceResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(CreateResourceError::from_response(response))),
                )
            }
        })
    }

    /// <p>Creates a user who can be used in Amazon WorkMail by calling the RegisterToWorkMail operation.</p>
    fn create_user(
        &self,
        input: CreateUserRequest,
    ) -> RusotoFuture<CreateUserResponse, CreateUserError> {
        let mut request = SignedRequest::new("POST", "workmail", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "WorkMailService.CreateUser");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<CreateUserResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(CreateUserError::from_response(response))),
                )
            }
        })
    }

    /// <p>Remove the alias from a set of aliases for a given user.</p>
    fn delete_alias(
        &self,
        input: DeleteAliasRequest,
    ) -> RusotoFuture<DeleteAliasResponse, DeleteAliasError> {
        let mut request = SignedRequest::new("POST", "workmail", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "WorkMailService.DeleteAlias");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<DeleteAliasResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(DeleteAliasError::from_response(response))),
                )
            }
        })
    }

    /// <p>Deletes a group from Amazon WorkMail.</p>
    fn delete_group(
        &self,
        input: DeleteGroupRequest,
    ) -> RusotoFuture<DeleteGroupResponse, DeleteGroupError> {
        let mut request = SignedRequest::new("POST", "workmail", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "WorkMailService.DeleteGroup");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<DeleteGroupResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(DeleteGroupError::from_response(response))),
                )
            }
        })
    }

    /// <p>Deletes permissions granted to a user or group.</p>
    fn delete_mailbox_permissions(
        &self,
        input: DeleteMailboxPermissionsRequest,
    ) -> RusotoFuture<DeleteMailboxPermissionsResponse, DeleteMailboxPermissionsError> {
        let mut request = SignedRequest::new("POST", "workmail", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "WorkMailService.DeleteMailboxPermissions");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<DeleteMailboxPermissionsResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(DeleteMailboxPermissionsError::from_response(response))
                }))
            }
        })
    }

    /// <p>Deletes the specified resource. </p>
    fn delete_resource(
        &self,
        input: DeleteResourceRequest,
    ) -> RusotoFuture<DeleteResourceResponse, DeleteResourceError> {
        let mut request = SignedRequest::new("POST", "workmail", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "WorkMailService.DeleteResource");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<DeleteResourceResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(DeleteResourceError::from_response(response))),
                )
            }
        })
    }

    /// <p>Deletes a user from Amazon WorkMail and all subsequent systems. The action can't be undone. The mailbox is kept as-is for a minimum of 30 days, without any means to restore it. </p>
    fn delete_user(
        &self,
        input: DeleteUserRequest,
    ) -> RusotoFuture<DeleteUserResponse, DeleteUserError> {
        let mut request = SignedRequest::new("POST", "workmail", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "WorkMailService.DeleteUser");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<DeleteUserResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(DeleteUserError::from_response(response))),
                )
            }
        })
    }

    /// <p>Mark a user, group, or resource as no longer used in Amazon WorkMail. This action disassociates the mailbox and schedules it for clean-up. Amazon WorkMail keeps mailboxes for 30 days before they are permanently removed. The functionality in the console is <i>Disable</i>.</p>
    fn deregister_from_work_mail(
        &self,
        input: DeregisterFromWorkMailRequest,
    ) -> RusotoFuture<DeregisterFromWorkMailResponse, DeregisterFromWorkMailError> {
        let mut request = SignedRequest::new("POST", "workmail", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "WorkMailService.DeregisterFromWorkMail");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<DeregisterFromWorkMailResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(DeregisterFromWorkMailError::from_response(response))
                    }),
                )
            }
        })
    }

    /// <p>Returns the data available for the group.</p>
    fn describe_group(
        &self,
        input: DescribeGroupRequest,
    ) -> RusotoFuture<DescribeGroupResponse, DescribeGroupError> {
        let mut request = SignedRequest::new("POST", "workmail", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "WorkMailService.DescribeGroup");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<DescribeGroupResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(DescribeGroupError::from_response(response))),
                )
            }
        })
    }

    /// <p>Provides more information regarding a given organization based on its identifier.</p>
    fn describe_organization(
        &self,
        input: DescribeOrganizationRequest,
    ) -> RusotoFuture<DescribeOrganizationResponse, DescribeOrganizationError> {
        let mut request = SignedRequest::new("POST", "workmail", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "WorkMailService.DescribeOrganization");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<DescribeOrganizationResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(DescribeOrganizationError::from_response(response))
                    }),
                )
            }
        })
    }

    /// <p>Returns the data available for the resource.</p>
    fn describe_resource(
        &self,
        input: DescribeResourceRequest,
    ) -> RusotoFuture<DescribeResourceResponse, DescribeResourceError> {
        let mut request = SignedRequest::new("POST", "workmail", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "WorkMailService.DescribeResource");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<DescribeResourceResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(DescribeResourceError::from_response(response))),
                )
            }
        })
    }

    /// <p>Provides information regarding the user.</p>
    fn describe_user(
        &self,
        input: DescribeUserRequest,
    ) -> RusotoFuture<DescribeUserResponse, DescribeUserError> {
        let mut request = SignedRequest::new("POST", "workmail", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "WorkMailService.DescribeUser");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<DescribeUserResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(DescribeUserError::from_response(response))),
                )
            }
        })
    }

    /// <p>Removes a member from the resource's set of delegates.</p>
    fn disassociate_delegate_from_resource(
        &self,
        input: DisassociateDelegateFromResourceRequest,
    ) -> RusotoFuture<DisassociateDelegateFromResourceResponse, DisassociateDelegateFromResourceError>
    {
        let mut request = SignedRequest::new("POST", "workmail", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "WorkMailService.DisassociateDelegateFromResource",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<DisassociateDelegateFromResourceResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(DisassociateDelegateFromResourceError::from_response(
                        response,
                    ))
                }))
            }
        })
    }

    /// <p>Removes a member from a group.</p>
    fn disassociate_member_from_group(
        &self,
        input: DisassociateMemberFromGroupRequest,
    ) -> RusotoFuture<DisassociateMemberFromGroupResponse, DisassociateMemberFromGroupError> {
        let mut request = SignedRequest::new("POST", "workmail", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "WorkMailService.DisassociateMemberFromGroup",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<DisassociateMemberFromGroupResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(DisassociateMemberFromGroupError::from_response(response))
                }))
            }
        })
    }

    /// <p>Creates a paginated call to list the aliases associated with a given entity.</p>
    fn list_aliases(
        &self,
        input: ListAliasesRequest,
    ) -> RusotoFuture<ListAliasesResponse, ListAliasesError> {
        let mut request = SignedRequest::new("POST", "workmail", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "WorkMailService.ListAliases");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<ListAliasesResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(ListAliasesError::from_response(response))),
                )
            }
        })
    }

    /// <p>Returns an overview of the members of a group.</p>
    fn list_group_members(
        &self,
        input: ListGroupMembersRequest,
    ) -> RusotoFuture<ListGroupMembersResponse, ListGroupMembersError> {
        let mut request = SignedRequest::new("POST", "workmail", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "WorkMailService.ListGroupMembers");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<ListGroupMembersResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(ListGroupMembersError::from_response(response))),
                )
            }
        })
    }

    /// <p>Returns summaries of the organization's groups.</p>
    fn list_groups(
        &self,
        input: ListGroupsRequest,
    ) -> RusotoFuture<ListGroupsResponse, ListGroupsError> {
        let mut request = SignedRequest::new("POST", "workmail", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "WorkMailService.ListGroups");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<ListGroupsResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(ListGroupsError::from_response(response))),
                )
            }
        })
    }

    /// <p>Lists the mailbox permissions associated with a mailbox.</p>
    fn list_mailbox_permissions(
        &self,
        input: ListMailboxPermissionsRequest,
    ) -> RusotoFuture<ListMailboxPermissionsResponse, ListMailboxPermissionsError> {
        let mut request = SignedRequest::new("POST", "workmail", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "WorkMailService.ListMailboxPermissions");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<ListMailboxPermissionsResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(ListMailboxPermissionsError::from_response(response))
                    }),
                )
            }
        })
    }

    /// <p>Returns summaries of the customer's non-deleted organizations.</p>
    fn list_organizations(
        &self,
        input: ListOrganizationsRequest,
    ) -> RusotoFuture<ListOrganizationsResponse, ListOrganizationsError> {
        let mut request = SignedRequest::new("POST", "workmail", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "WorkMailService.ListOrganizations");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<ListOrganizationsResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(ListOrganizationsError::from_response(response))),
                )
            }
        })
    }

    /// <p>Lists the delegates associated with a resource. Users and groups can be resource delegates and answer requests on behalf of the resource.</p>
    fn list_resource_delegates(
        &self,
        input: ListResourceDelegatesRequest,
    ) -> RusotoFuture<ListResourceDelegatesResponse, ListResourceDelegatesError> {
        let mut request = SignedRequest::new("POST", "workmail", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "WorkMailService.ListResourceDelegates");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<ListResourceDelegatesResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(ListResourceDelegatesError::from_response(response))
                    }),
                )
            }
        })
    }

    /// <p>Returns summaries of the organization's resources.</p>
    fn list_resources(
        &self,
        input: ListResourcesRequest,
    ) -> RusotoFuture<ListResourcesResponse, ListResourcesError> {
        let mut request = SignedRequest::new("POST", "workmail", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "WorkMailService.ListResources");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<ListResourcesResponse>(
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

    /// <p>Returns summaries of the organization's users.</p>
    fn list_users(
        &self,
        input: ListUsersRequest,
    ) -> RusotoFuture<ListUsersResponse, ListUsersError> {
        let mut request = SignedRequest::new("POST", "workmail", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "WorkMailService.ListUsers");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<ListUsersResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(ListUsersError::from_response(response))),
                )
            }
        })
    }

    /// <p>Sets permissions for a user or group. This replaces any pre-existing permissions set for the entity.</p>
    fn put_mailbox_permissions(
        &self,
        input: PutMailboxPermissionsRequest,
    ) -> RusotoFuture<PutMailboxPermissionsResponse, PutMailboxPermissionsError> {
        let mut request = SignedRequest::new("POST", "workmail", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "WorkMailService.PutMailboxPermissions");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<PutMailboxPermissionsResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(PutMailboxPermissionsError::from_response(response))
                    }),
                )
            }
        })
    }

    /// <p>Registers an existing and disabled user, group, or resource/entity for Amazon WorkMail use by associating a mailbox and calendaring capabilities. It performs no change if the entity is enabled and fails if the entity is deleted. This operation results in the accumulation of costs. For more information, see <a href="http://aws.amazon.com/workmail/pricing">Pricing</a>. The equivalent console functionality for this operation is <i>Enable</i>. Users can either be created by calling the CreateUser API or they can be synchronized from your directory. For more information, see DeregisterFromWorkMail.</p>
    fn register_to_work_mail(
        &self,
        input: RegisterToWorkMailRequest,
    ) -> RusotoFuture<RegisterToWorkMailResponse, RegisterToWorkMailError> {
        let mut request = SignedRequest::new("POST", "workmail", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "WorkMailService.RegisterToWorkMail");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<RegisterToWorkMailResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(RegisterToWorkMailError::from_response(response))),
                )
            }
        })
    }

    /// <p>Allows the administrator to reset the password for a user.</p>
    fn reset_password(
        &self,
        input: ResetPasswordRequest,
    ) -> RusotoFuture<ResetPasswordResponse, ResetPasswordError> {
        let mut request = SignedRequest::new("POST", "workmail", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "WorkMailService.ResetPassword");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<ResetPasswordResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(ResetPasswordError::from_response(response))),
                )
            }
        })
    }

    /// <p>Updates the primary email for an entity. The current email is moved into the list of aliases (or swapped between an existing alias and the current primary email) and the email provided in the input is promoted as the primary.</p>
    fn update_primary_email_address(
        &self,
        input: UpdatePrimaryEmailAddressRequest,
    ) -> RusotoFuture<UpdatePrimaryEmailAddressResponse, UpdatePrimaryEmailAddressError> {
        let mut request = SignedRequest::new("POST", "workmail", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "WorkMailService.UpdatePrimaryEmailAddress");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<UpdatePrimaryEmailAddressResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(UpdatePrimaryEmailAddressError::from_response(response))
                }))
            }
        })
    }

    /// <p>Updates data for the resource. It must be preceded by a describe call in order to have the latest information. The dataset in the request should be the one expected when performing another describe call.</p>
    fn update_resource(
        &self,
        input: UpdateResourceRequest,
    ) -> RusotoFuture<UpdateResourceResponse, UpdateResourceError> {
        let mut request = SignedRequest::new("POST", "workmail", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "WorkMailService.UpdateResource");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<UpdateResourceResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(UpdateResourceError::from_response(response))),
                )
            }
        })
    }
}

impl<T: ?Sized + Workmail> Workmail for ::std::rc::Rc<T> {
    /// <p>Adds a member to the resource's set of delegates.</p>
    fn associate_delegate_to_resource(
        &self,
        input: AssociateDelegateToResourceRequest,
    ) -> RusotoFuture<AssociateDelegateToResourceResponse, AssociateDelegateToResourceError> {
        Workmail::associate_delegate_to_resource(&(**self), input)
    }

    /// <p>Adds a member to the group's set.</p>
    fn associate_member_to_group(
        &self,
        input: AssociateMemberToGroupRequest,
    ) -> RusotoFuture<AssociateMemberToGroupResponse, AssociateMemberToGroupError> {
        Workmail::associate_member_to_group(&(**self), input)
    }

    /// <p>Adds an alias to the set of a given member of Amazon WorkMail.</p>
    fn create_alias(
        &self,
        input: CreateAliasRequest,
    ) -> RusotoFuture<CreateAliasResponse, CreateAliasError> {
        Workmail::create_alias(&(**self), input)
    }

    /// <p>Creates a group that can be used in Amazon WorkMail by calling the RegisterToWorkMail operation.</p>
    fn create_group(
        &self,
        input: CreateGroupRequest,
    ) -> RusotoFuture<CreateGroupResponse, CreateGroupError> {
        Workmail::create_group(&(**self), input)
    }

    /// <p>Creates a new Amazon WorkMail resource. The available types are equipment and room.</p>
    fn create_resource(
        &self,
        input: CreateResourceRequest,
    ) -> RusotoFuture<CreateResourceResponse, CreateResourceError> {
        Workmail::create_resource(&(**self), input)
    }

    /// <p>Creates a user who can be used in Amazon WorkMail by calling the RegisterToWorkMail operation.</p>
    fn create_user(
        &self,
        input: CreateUserRequest,
    ) -> RusotoFuture<CreateUserResponse, CreateUserError> {
        Workmail::create_user(&(**self), input)
    }

    /// <p>Remove the alias from a set of aliases for a given user.</p>
    fn delete_alias(
        &self,
        input: DeleteAliasRequest,
    ) -> RusotoFuture<DeleteAliasResponse, DeleteAliasError> {
        Workmail::delete_alias(&(**self), input)
    }

    /// <p>Deletes a group from Amazon WorkMail.</p>
    fn delete_group(
        &self,
        input: DeleteGroupRequest,
    ) -> RusotoFuture<DeleteGroupResponse, DeleteGroupError> {
        Workmail::delete_group(&(**self), input)
    }

    /// <p>Deletes permissions granted to a user or group.</p>
    fn delete_mailbox_permissions(
        &self,
        input: DeleteMailboxPermissionsRequest,
    ) -> RusotoFuture<DeleteMailboxPermissionsResponse, DeleteMailboxPermissionsError> {
        Workmail::delete_mailbox_permissions(&(**self), input)
    }

    /// <p>Deletes the specified resource. </p>
    fn delete_resource(
        &self,
        input: DeleteResourceRequest,
    ) -> RusotoFuture<DeleteResourceResponse, DeleteResourceError> {
        Workmail::delete_resource(&(**self), input)
    }

    /// <p>Deletes a user from Amazon WorkMail and all subsequent systems. The action can't be undone. The mailbox is kept as-is for a minimum of 30 days, without any means to restore it. </p>
    fn delete_user(
        &self,
        input: DeleteUserRequest,
    ) -> RusotoFuture<DeleteUserResponse, DeleteUserError> {
        Workmail::delete_user(&(**self), input)
    }

    /// <p>Mark a user, group, or resource as no longer used in Amazon WorkMail. This action disassociates the mailbox and schedules it for clean-up. Amazon WorkMail keeps mailboxes for 30 days before they are permanently removed. The functionality in the console is <i>Disable</i>.</p>
    fn deregister_from_work_mail(
        &self,
        input: DeregisterFromWorkMailRequest,
    ) -> RusotoFuture<DeregisterFromWorkMailResponse, DeregisterFromWorkMailError> {
        Workmail::deregister_from_work_mail(&(**self), input)
    }

    /// <p>Returns the data available for the group.</p>
    fn describe_group(
        &self,
        input: DescribeGroupRequest,
    ) -> RusotoFuture<DescribeGroupResponse, DescribeGroupError> {
        Workmail::describe_group(&(**self), input)
    }

    /// <p>Provides more information regarding a given organization based on its identifier.</p>
    fn describe_organization(
        &self,
        input: DescribeOrganizationRequest,
    ) -> RusotoFuture<DescribeOrganizationResponse, DescribeOrganizationError> {
        Workmail::describe_organization(&(**self), input)
    }

    /// <p>Returns the data available for the resource.</p>
    fn describe_resource(
        &self,
        input: DescribeResourceRequest,
    ) -> RusotoFuture<DescribeResourceResponse, DescribeResourceError> {
        Workmail::describe_resource(&(**self), input)
    }

    /// <p>Provides information regarding the user.</p>
    fn describe_user(
        &self,
        input: DescribeUserRequest,
    ) -> RusotoFuture<DescribeUserResponse, DescribeUserError> {
        Workmail::describe_user(&(**self), input)
    }

    /// <p>Removes a member from the resource's set of delegates.</p>
    fn disassociate_delegate_from_resource(
        &self,
        input: DisassociateDelegateFromResourceRequest,
    ) -> RusotoFuture<DisassociateDelegateFromResourceResponse, DisassociateDelegateFromResourceError>
    {
        Workmail::disassociate_delegate_from_resource(&(**self), input)
    }

    /// <p>Removes a member from a group.</p>
    fn disassociate_member_from_group(
        &self,
        input: DisassociateMemberFromGroupRequest,
    ) -> RusotoFuture<DisassociateMemberFromGroupResponse, DisassociateMemberFromGroupError> {
        Workmail::disassociate_member_from_group(&(**self), input)
    }

    /// <p>Creates a paginated call to list the aliases associated with a given entity.</p>
    fn list_aliases(
        &self,
        input: ListAliasesRequest,
    ) -> RusotoFuture<ListAliasesResponse, ListAliasesError> {
        Workmail::list_aliases(&(**self), input)
    }

    /// <p>Returns an overview of the members of a group.</p>
    fn list_group_members(
        &self,
        input: ListGroupMembersRequest,
    ) -> RusotoFuture<ListGroupMembersResponse, ListGroupMembersError> {
        Workmail::list_group_members(&(**self), input)
    }

    /// <p>Returns summaries of the organization's groups.</p>
    fn list_groups(
        &self,
        input: ListGroupsRequest,
    ) -> RusotoFuture<ListGroupsResponse, ListGroupsError> {
        Workmail::list_groups(&(**self), input)
    }

    /// <p>Lists the mailbox permissions associated with a mailbox.</p>
    fn list_mailbox_permissions(
        &self,
        input: ListMailboxPermissionsRequest,
    ) -> RusotoFuture<ListMailboxPermissionsResponse, ListMailboxPermissionsError> {
        Workmail::list_mailbox_permissions(&(**self), input)
    }

    /// <p>Returns summaries of the customer's non-deleted organizations.</p>
    fn list_organizations(
        &self,
        input: ListOrganizationsRequest,
    ) -> RusotoFuture<ListOrganizationsResponse, ListOrganizationsError> {
        Workmail::list_organizations(&(**self), input)
    }

    /// <p>Lists the delegates associated with a resource. Users and groups can be resource delegates and answer requests on behalf of the resource.</p>
    fn list_resource_delegates(
        &self,
        input: ListResourceDelegatesRequest,
    ) -> RusotoFuture<ListResourceDelegatesResponse, ListResourceDelegatesError> {
        Workmail::list_resource_delegates(&(**self), input)
    }

    /// <p>Returns summaries of the organization's resources.</p>
    fn list_resources(
        &self,
        input: ListResourcesRequest,
    ) -> RusotoFuture<ListResourcesResponse, ListResourcesError> {
        Workmail::list_resources(&(**self), input)
    }

    /// <p>Returns summaries of the organization's users.</p>
    fn list_users(
        &self,
        input: ListUsersRequest,
    ) -> RusotoFuture<ListUsersResponse, ListUsersError> {
        Workmail::list_users(&(**self), input)
    }

    /// <p>Sets permissions for a user or group. This replaces any pre-existing permissions set for the entity.</p>
    fn put_mailbox_permissions(
        &self,
        input: PutMailboxPermissionsRequest,
    ) -> RusotoFuture<PutMailboxPermissionsResponse, PutMailboxPermissionsError> {
        Workmail::put_mailbox_permissions(&(**self), input)
    }

    /// <p>Registers an existing and disabled user, group, or resource/entity for Amazon WorkMail use by associating a mailbox and calendaring capabilities. It performs no change if the entity is enabled and fails if the entity is deleted. This operation results in the accumulation of costs. For more information, see <a href="http://aws.amazon.com/workmail/pricing">Pricing</a>. The equivalent console functionality for this operation is <i>Enable</i>. Users can either be created by calling the CreateUser API or they can be synchronized from your directory. For more information, see DeregisterFromWorkMail.</p>
    fn register_to_work_mail(
        &self,
        input: RegisterToWorkMailRequest,
    ) -> RusotoFuture<RegisterToWorkMailResponse, RegisterToWorkMailError> {
        Workmail::register_to_work_mail(&(**self), input)
    }

    /// <p>Allows the administrator to reset the password for a user.</p>
    fn reset_password(
        &self,
        input: ResetPasswordRequest,
    ) -> RusotoFuture<ResetPasswordResponse, ResetPasswordError> {
        Workmail::reset_password(&(**self), input)
    }

    /// <p>Updates the primary email for an entity. The current email is moved into the list of aliases (or swapped between an existing alias and the current primary email) and the email provided in the input is promoted as the primary.</p>
    fn update_primary_email_address(
        &self,
        input: UpdatePrimaryEmailAddressRequest,
    ) -> RusotoFuture<UpdatePrimaryEmailAddressResponse, UpdatePrimaryEmailAddressError> {
        Workmail::update_primary_email_address(&(**self), input)
    }

    /// <p>Updates data for the resource. It must be preceded by a describe call in order to have the latest information. The dataset in the request should be the one expected when performing another describe call.</p>
    fn update_resource(
        &self,
        input: UpdateResourceRequest,
    ) -> RusotoFuture<UpdateResourceResponse, UpdateResourceError> {
        Workmail::update_resource(&(**self), input)
    }
}

#[cfg(test)]
mod protocol_tests {}
