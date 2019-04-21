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

use rusoto_core::proto;
use rusoto_core::signature::SignedRequest;
use serde_json;
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
}

impl AssociateDelegateToResourceError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<AssociateDelegateToResourceError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "EntityNotFoundException" => {
                    return RusotoError::Service(AssociateDelegateToResourceError::EntityNotFound(
                        err.msg,
                    ))
                }
                "EntityStateException" => {
                    return RusotoError::Service(AssociateDelegateToResourceError::EntityState(
                        err.msg,
                    ))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(
                        AssociateDelegateToResourceError::InvalidParameter(err.msg),
                    )
                }
                "OrganizationNotFoundException" => {
                    return RusotoError::Service(
                        AssociateDelegateToResourceError::OrganizationNotFound(err.msg),
                    )
                }
                "OrganizationStateException" => {
                    return RusotoError::Service(
                        AssociateDelegateToResourceError::OrganizationState(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
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
}

impl AssociateMemberToGroupError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<AssociateMemberToGroupError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "DirectoryServiceAuthenticationFailedException" => {
                    return RusotoError::Service(
                        AssociateMemberToGroupError::DirectoryServiceAuthenticationFailed(err.msg),
                    )
                }
                "DirectoryUnavailableException" => {
                    return RusotoError::Service(AssociateMemberToGroupError::DirectoryUnavailable(
                        err.msg,
                    ))
                }
                "EntityNotFoundException" => {
                    return RusotoError::Service(AssociateMemberToGroupError::EntityNotFound(
                        err.msg,
                    ))
                }
                "EntityStateException" => {
                    return RusotoError::Service(AssociateMemberToGroupError::EntityState(err.msg))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(AssociateMemberToGroupError::InvalidParameter(
                        err.msg,
                    ))
                }
                "OrganizationNotFoundException" => {
                    return RusotoError::Service(AssociateMemberToGroupError::OrganizationNotFound(
                        err.msg,
                    ))
                }
                "OrganizationStateException" => {
                    return RusotoError::Service(AssociateMemberToGroupError::OrganizationState(
                        err.msg,
                    ))
                }
                "UnsupportedOperationException" => {
                    return RusotoError::Service(AssociateMemberToGroupError::UnsupportedOperation(
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
}

impl CreateAliasError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateAliasError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "EmailAddressInUseException" => {
                    return RusotoError::Service(CreateAliasError::EmailAddressInUse(err.msg))
                }
                "EntityNotFoundException" => {
                    return RusotoError::Service(CreateAliasError::EntityNotFound(err.msg))
                }
                "EntityStateException" => {
                    return RusotoError::Service(CreateAliasError::EntityState(err.msg))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(CreateAliasError::InvalidParameter(err.msg))
                }
                "MailDomainNotFoundException" => {
                    return RusotoError::Service(CreateAliasError::MailDomainNotFound(err.msg))
                }
                "MailDomainStateException" => {
                    return RusotoError::Service(CreateAliasError::MailDomainState(err.msg))
                }
                "OrganizationNotFoundException" => {
                    return RusotoError::Service(CreateAliasError::OrganizationNotFound(err.msg))
                }
                "OrganizationStateException" => {
                    return RusotoError::Service(CreateAliasError::OrganizationState(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
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
}

impl CreateGroupError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateGroupError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "DirectoryServiceAuthenticationFailedException" => {
                    return RusotoError::Service(
                        CreateGroupError::DirectoryServiceAuthenticationFailed(err.msg),
                    )
                }
                "DirectoryUnavailableException" => {
                    return RusotoError::Service(CreateGroupError::DirectoryUnavailable(err.msg))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(CreateGroupError::InvalidParameter(err.msg))
                }
                "NameAvailabilityException" => {
                    return RusotoError::Service(CreateGroupError::NameAvailability(err.msg))
                }
                "OrganizationNotFoundException" => {
                    return RusotoError::Service(CreateGroupError::OrganizationNotFound(err.msg))
                }
                "OrganizationStateException" => {
                    return RusotoError::Service(CreateGroupError::OrganizationState(err.msg))
                }
                "ReservedNameException" => {
                    return RusotoError::Service(CreateGroupError::ReservedName(err.msg))
                }
                "UnsupportedOperationException" => {
                    return RusotoError::Service(CreateGroupError::UnsupportedOperation(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
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
}

impl CreateResourceError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateResourceError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "DirectoryServiceAuthenticationFailedException" => {
                    return RusotoError::Service(
                        CreateResourceError::DirectoryServiceAuthenticationFailed(err.msg),
                    )
                }
                "DirectoryUnavailableException" => {
                    return RusotoError::Service(CreateResourceError::DirectoryUnavailable(err.msg))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(CreateResourceError::InvalidParameter(err.msg))
                }
                "NameAvailabilityException" => {
                    return RusotoError::Service(CreateResourceError::NameAvailability(err.msg))
                }
                "OrganizationNotFoundException" => {
                    return RusotoError::Service(CreateResourceError::OrganizationNotFound(err.msg))
                }
                "OrganizationStateException" => {
                    return RusotoError::Service(CreateResourceError::OrganizationState(err.msg))
                }
                "ReservedNameException" => {
                    return RusotoError::Service(CreateResourceError::ReservedName(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
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
}

impl CreateUserError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateUserError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "DirectoryServiceAuthenticationFailedException" => {
                    return RusotoError::Service(
                        CreateUserError::DirectoryServiceAuthenticationFailed(err.msg),
                    )
                }
                "DirectoryUnavailableException" => {
                    return RusotoError::Service(CreateUserError::DirectoryUnavailable(err.msg))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(CreateUserError::InvalidParameter(err.msg))
                }
                "InvalidPasswordException" => {
                    return RusotoError::Service(CreateUserError::InvalidPassword(err.msg))
                }
                "NameAvailabilityException" => {
                    return RusotoError::Service(CreateUserError::NameAvailability(err.msg))
                }
                "OrganizationNotFoundException" => {
                    return RusotoError::Service(CreateUserError::OrganizationNotFound(err.msg))
                }
                "OrganizationStateException" => {
                    return RusotoError::Service(CreateUserError::OrganizationState(err.msg))
                }
                "ReservedNameException" => {
                    return RusotoError::Service(CreateUserError::ReservedName(err.msg))
                }
                "UnsupportedOperationException" => {
                    return RusotoError::Service(CreateUserError::UnsupportedOperation(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
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
}

impl DeleteAliasError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteAliasError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "EntityNotFoundException" => {
                    return RusotoError::Service(DeleteAliasError::EntityNotFound(err.msg))
                }
                "EntityStateException" => {
                    return RusotoError::Service(DeleteAliasError::EntityState(err.msg))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(DeleteAliasError::InvalidParameter(err.msg))
                }
                "OrganizationNotFoundException" => {
                    return RusotoError::Service(DeleteAliasError::OrganizationNotFound(err.msg))
                }
                "OrganizationStateException" => {
                    return RusotoError::Service(DeleteAliasError::OrganizationState(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
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
}

impl DeleteGroupError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteGroupError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "DirectoryServiceAuthenticationFailedException" => {
                    return RusotoError::Service(
                        DeleteGroupError::DirectoryServiceAuthenticationFailed(err.msg),
                    )
                }
                "DirectoryUnavailableException" => {
                    return RusotoError::Service(DeleteGroupError::DirectoryUnavailable(err.msg))
                }
                "EntityStateException" => {
                    return RusotoError::Service(DeleteGroupError::EntityState(err.msg))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(DeleteGroupError::InvalidParameter(err.msg))
                }
                "OrganizationNotFoundException" => {
                    return RusotoError::Service(DeleteGroupError::OrganizationNotFound(err.msg))
                }
                "OrganizationStateException" => {
                    return RusotoError::Service(DeleteGroupError::OrganizationState(err.msg))
                }
                "UnsupportedOperationException" => {
                    return RusotoError::Service(DeleteGroupError::UnsupportedOperation(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
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
}

impl DeleteMailboxPermissionsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteMailboxPermissionsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "EntityNotFoundException" => {
                    return RusotoError::Service(DeleteMailboxPermissionsError::EntityNotFound(
                        err.msg,
                    ))
                }
                "EntityStateException" => {
                    return RusotoError::Service(DeleteMailboxPermissionsError::EntityState(
                        err.msg,
                    ))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(DeleteMailboxPermissionsError::InvalidParameter(
                        err.msg,
                    ))
                }
                "OrganizationNotFoundException" => {
                    return RusotoError::Service(
                        DeleteMailboxPermissionsError::OrganizationNotFound(err.msg),
                    )
                }
                "OrganizationStateException" => {
                    return RusotoError::Service(DeleteMailboxPermissionsError::OrganizationState(
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
}

impl DeleteResourceError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteResourceError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "EntityStateException" => {
                    return RusotoError::Service(DeleteResourceError::EntityState(err.msg))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(DeleteResourceError::InvalidParameter(err.msg))
                }
                "OrganizationNotFoundException" => {
                    return RusotoError::Service(DeleteResourceError::OrganizationNotFound(err.msg))
                }
                "OrganizationStateException" => {
                    return RusotoError::Service(DeleteResourceError::OrganizationState(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
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
}

impl DeleteUserError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteUserError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "DirectoryServiceAuthenticationFailedException" => {
                    return RusotoError::Service(
                        DeleteUserError::DirectoryServiceAuthenticationFailed(err.msg),
                    )
                }
                "DirectoryUnavailableException" => {
                    return RusotoError::Service(DeleteUserError::DirectoryUnavailable(err.msg))
                }
                "EntityStateException" => {
                    return RusotoError::Service(DeleteUserError::EntityState(err.msg))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(DeleteUserError::InvalidParameter(err.msg))
                }
                "OrganizationNotFoundException" => {
                    return RusotoError::Service(DeleteUserError::OrganizationNotFound(err.msg))
                }
                "OrganizationStateException" => {
                    return RusotoError::Service(DeleteUserError::OrganizationState(err.msg))
                }
                "UnsupportedOperationException" => {
                    return RusotoError::Service(DeleteUserError::UnsupportedOperation(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
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
}

impl DeregisterFromWorkMailError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeregisterFromWorkMailError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "EntityNotFoundException" => {
                    return RusotoError::Service(DeregisterFromWorkMailError::EntityNotFound(
                        err.msg,
                    ))
                }
                "EntityStateException" => {
                    return RusotoError::Service(DeregisterFromWorkMailError::EntityState(err.msg))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(DeregisterFromWorkMailError::InvalidParameter(
                        err.msg,
                    ))
                }
                "OrganizationNotFoundException" => {
                    return RusotoError::Service(DeregisterFromWorkMailError::OrganizationNotFound(
                        err.msg,
                    ))
                }
                "OrganizationStateException" => {
                    return RusotoError::Service(DeregisterFromWorkMailError::OrganizationState(
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
}

impl DescribeGroupError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeGroupError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "EntityNotFoundException" => {
                    return RusotoError::Service(DescribeGroupError::EntityNotFound(err.msg))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(DescribeGroupError::InvalidParameter(err.msg))
                }
                "OrganizationNotFoundException" => {
                    return RusotoError::Service(DescribeGroupError::OrganizationNotFound(err.msg))
                }
                "OrganizationStateException" => {
                    return RusotoError::Service(DescribeGroupError::OrganizationState(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
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
}

impl DescribeOrganizationError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeOrganizationError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidParameterException" => {
                    return RusotoError::Service(DescribeOrganizationError::InvalidParameter(
                        err.msg,
                    ))
                }
                "OrganizationNotFoundException" => {
                    return RusotoError::Service(DescribeOrganizationError::OrganizationNotFound(
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
}

impl DescribeResourceError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeResourceError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "EntityNotFoundException" => {
                    return RusotoError::Service(DescribeResourceError::EntityNotFound(err.msg))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(DescribeResourceError::InvalidParameter(err.msg))
                }
                "OrganizationNotFoundException" => {
                    return RusotoError::Service(DescribeResourceError::OrganizationNotFound(
                        err.msg,
                    ))
                }
                "OrganizationStateException" => {
                    return RusotoError::Service(DescribeResourceError::OrganizationState(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
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
}

impl DescribeUserError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeUserError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "EntityNotFoundException" => {
                    return RusotoError::Service(DescribeUserError::EntityNotFound(err.msg))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(DescribeUserError::InvalidParameter(err.msg))
                }
                "OrganizationNotFoundException" => {
                    return RusotoError::Service(DescribeUserError::OrganizationNotFound(err.msg))
                }
                "OrganizationStateException" => {
                    return RusotoError::Service(DescribeUserError::OrganizationState(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
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
}

impl DisassociateDelegateFromResourceError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<DisassociateDelegateFromResourceError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "EntityNotFoundException" => {
                    return RusotoError::Service(
                        DisassociateDelegateFromResourceError::EntityNotFound(err.msg),
                    )
                }
                "EntityStateException" => {
                    return RusotoError::Service(
                        DisassociateDelegateFromResourceError::EntityState(err.msg),
                    )
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(
                        DisassociateDelegateFromResourceError::InvalidParameter(err.msg),
                    )
                }
                "OrganizationNotFoundException" => {
                    return RusotoError::Service(
                        DisassociateDelegateFromResourceError::OrganizationNotFound(err.msg),
                    )
                }
                "OrganizationStateException" => {
                    return RusotoError::Service(
                        DisassociateDelegateFromResourceError::OrganizationState(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
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
}

impl DisassociateMemberFromGroupError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<DisassociateMemberFromGroupError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "DirectoryServiceAuthenticationFailedException" => {
                    return RusotoError::Service(
                        DisassociateMemberFromGroupError::DirectoryServiceAuthenticationFailed(
                            err.msg,
                        ),
                    )
                }
                "DirectoryUnavailableException" => {
                    return RusotoError::Service(
                        DisassociateMemberFromGroupError::DirectoryUnavailable(err.msg),
                    )
                }
                "EntityNotFoundException" => {
                    return RusotoError::Service(DisassociateMemberFromGroupError::EntityNotFound(
                        err.msg,
                    ))
                }
                "EntityStateException" => {
                    return RusotoError::Service(DisassociateMemberFromGroupError::EntityState(
                        err.msg,
                    ))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(
                        DisassociateMemberFromGroupError::InvalidParameter(err.msg),
                    )
                }
                "OrganizationNotFoundException" => {
                    return RusotoError::Service(
                        DisassociateMemberFromGroupError::OrganizationNotFound(err.msg),
                    )
                }
                "OrganizationStateException" => {
                    return RusotoError::Service(
                        DisassociateMemberFromGroupError::OrganizationState(err.msg),
                    )
                }
                "UnsupportedOperationException" => {
                    return RusotoError::Service(
                        DisassociateMemberFromGroupError::UnsupportedOperation(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
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
}

impl ListAliasesError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListAliasesError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "EntityNotFoundException" => {
                    return RusotoError::Service(ListAliasesError::EntityNotFound(err.msg))
                }
                "EntityStateException" => {
                    return RusotoError::Service(ListAliasesError::EntityState(err.msg))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(ListAliasesError::InvalidParameter(err.msg))
                }
                "OrganizationNotFoundException" => {
                    return RusotoError::Service(ListAliasesError::OrganizationNotFound(err.msg))
                }
                "OrganizationStateException" => {
                    return RusotoError::Service(ListAliasesError::OrganizationState(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
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
}

impl ListGroupMembersError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListGroupMembersError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "EntityNotFoundException" => {
                    return RusotoError::Service(ListGroupMembersError::EntityNotFound(err.msg))
                }
                "EntityStateException" => {
                    return RusotoError::Service(ListGroupMembersError::EntityState(err.msg))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(ListGroupMembersError::InvalidParameter(err.msg))
                }
                "OrganizationNotFoundException" => {
                    return RusotoError::Service(ListGroupMembersError::OrganizationNotFound(
                        err.msg,
                    ))
                }
                "OrganizationStateException" => {
                    return RusotoError::Service(ListGroupMembersError::OrganizationState(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
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
}

impl ListGroupsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListGroupsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "EntityNotFoundException" => {
                    return RusotoError::Service(ListGroupsError::EntityNotFound(err.msg))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(ListGroupsError::InvalidParameter(err.msg))
                }
                "OrganizationNotFoundException" => {
                    return RusotoError::Service(ListGroupsError::OrganizationNotFound(err.msg))
                }
                "OrganizationStateException" => {
                    return RusotoError::Service(ListGroupsError::OrganizationState(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
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
}

impl ListMailboxPermissionsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListMailboxPermissionsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "EntityNotFoundException" => {
                    return RusotoError::Service(ListMailboxPermissionsError::EntityNotFound(
                        err.msg,
                    ))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(ListMailboxPermissionsError::InvalidParameter(
                        err.msg,
                    ))
                }
                "OrganizationNotFoundException" => {
                    return RusotoError::Service(ListMailboxPermissionsError::OrganizationNotFound(
                        err.msg,
                    ))
                }
                "OrganizationStateException" => {
                    return RusotoError::Service(ListMailboxPermissionsError::OrganizationState(
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
        }
    }
}
/// Errors returned by ListOrganizations
#[derive(Debug, PartialEq)]
pub enum ListOrganizationsError {
    /// <p>One or more of the input parameters don't match the service's restrictions.</p>
    InvalidParameter(String),
}

impl ListOrganizationsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListOrganizationsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidParameterException" => {
                    return RusotoError::Service(ListOrganizationsError::InvalidParameter(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
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
}

impl ListResourceDelegatesError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListResourceDelegatesError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "EntityNotFoundException" => {
                    return RusotoError::Service(ListResourceDelegatesError::EntityNotFound(
                        err.msg,
                    ))
                }
                "EntityStateException" => {
                    return RusotoError::Service(ListResourceDelegatesError::EntityState(err.msg))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(ListResourceDelegatesError::InvalidParameter(
                        err.msg,
                    ))
                }
                "OrganizationNotFoundException" => {
                    return RusotoError::Service(ListResourceDelegatesError::OrganizationNotFound(
                        err.msg,
                    ))
                }
                "OrganizationStateException" => {
                    return RusotoError::Service(ListResourceDelegatesError::OrganizationState(
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
}

impl ListResourcesError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListResourcesError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidParameterException" => {
                    return RusotoError::Service(ListResourcesError::InvalidParameter(err.msg))
                }
                "OrganizationNotFoundException" => {
                    return RusotoError::Service(ListResourcesError::OrganizationNotFound(err.msg))
                }
                "OrganizationStateException" => {
                    return RusotoError::Service(ListResourcesError::OrganizationState(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
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
}

impl ListUsersError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListUsersError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidParameterException" => {
                    return RusotoError::Service(ListUsersError::InvalidParameter(err.msg))
                }
                "OrganizationNotFoundException" => {
                    return RusotoError::Service(ListUsersError::OrganizationNotFound(err.msg))
                }
                "OrganizationStateException" => {
                    return RusotoError::Service(ListUsersError::OrganizationState(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
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
}

impl PutMailboxPermissionsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<PutMailboxPermissionsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "EntityNotFoundException" => {
                    return RusotoError::Service(PutMailboxPermissionsError::EntityNotFound(
                        err.msg,
                    ))
                }
                "EntityStateException" => {
                    return RusotoError::Service(PutMailboxPermissionsError::EntityState(err.msg))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(PutMailboxPermissionsError::InvalidParameter(
                        err.msg,
                    ))
                }
                "OrganizationNotFoundException" => {
                    return RusotoError::Service(PutMailboxPermissionsError::OrganizationNotFound(
                        err.msg,
                    ))
                }
                "OrganizationStateException" => {
                    return RusotoError::Service(PutMailboxPermissionsError::OrganizationState(
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
}

impl RegisterToWorkMailError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<RegisterToWorkMailError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "DirectoryServiceAuthenticationFailedException" => {
                    return RusotoError::Service(
                        RegisterToWorkMailError::DirectoryServiceAuthenticationFailed(err.msg),
                    )
                }
                "DirectoryUnavailableException" => {
                    return RusotoError::Service(RegisterToWorkMailError::DirectoryUnavailable(
                        err.msg,
                    ))
                }
                "EmailAddressInUseException" => {
                    return RusotoError::Service(RegisterToWorkMailError::EmailAddressInUse(
                        err.msg,
                    ))
                }
                "EntityAlreadyRegisteredException" => {
                    return RusotoError::Service(RegisterToWorkMailError::EntityAlreadyRegistered(
                        err.msg,
                    ))
                }
                "EntityNotFoundException" => {
                    return RusotoError::Service(RegisterToWorkMailError::EntityNotFound(err.msg))
                }
                "EntityStateException" => {
                    return RusotoError::Service(RegisterToWorkMailError::EntityState(err.msg))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(RegisterToWorkMailError::InvalidParameter(err.msg))
                }
                "MailDomainNotFoundException" => {
                    return RusotoError::Service(RegisterToWorkMailError::MailDomainNotFound(
                        err.msg,
                    ))
                }
                "MailDomainStateException" => {
                    return RusotoError::Service(RegisterToWorkMailError::MailDomainState(err.msg))
                }
                "OrganizationNotFoundException" => {
                    return RusotoError::Service(RegisterToWorkMailError::OrganizationNotFound(
                        err.msg,
                    ))
                }
                "OrganizationStateException" => {
                    return RusotoError::Service(RegisterToWorkMailError::OrganizationState(
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
}

impl ResetPasswordError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ResetPasswordError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "DirectoryServiceAuthenticationFailedException" => {
                    return RusotoError::Service(
                        ResetPasswordError::DirectoryServiceAuthenticationFailed(err.msg),
                    )
                }
                "DirectoryUnavailableException" => {
                    return RusotoError::Service(ResetPasswordError::DirectoryUnavailable(err.msg))
                }
                "EntityNotFoundException" => {
                    return RusotoError::Service(ResetPasswordError::EntityNotFound(err.msg))
                }
                "EntityStateException" => {
                    return RusotoError::Service(ResetPasswordError::EntityState(err.msg))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(ResetPasswordError::InvalidParameter(err.msg))
                }
                "InvalidPasswordException" => {
                    return RusotoError::Service(ResetPasswordError::InvalidPassword(err.msg))
                }
                "OrganizationNotFoundException" => {
                    return RusotoError::Service(ResetPasswordError::OrganizationNotFound(err.msg))
                }
                "OrganizationStateException" => {
                    return RusotoError::Service(ResetPasswordError::OrganizationState(err.msg))
                }
                "UnsupportedOperationException" => {
                    return RusotoError::Service(ResetPasswordError::UnsupportedOperation(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
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
}

impl UpdatePrimaryEmailAddressError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdatePrimaryEmailAddressError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "DirectoryServiceAuthenticationFailedException" => {
                    return RusotoError::Service(
                        UpdatePrimaryEmailAddressError::DirectoryServiceAuthenticationFailed(
                            err.msg,
                        ),
                    )
                }
                "DirectoryUnavailableException" => {
                    return RusotoError::Service(
                        UpdatePrimaryEmailAddressError::DirectoryUnavailable(err.msg),
                    )
                }
                "EmailAddressInUseException" => {
                    return RusotoError::Service(UpdatePrimaryEmailAddressError::EmailAddressInUse(
                        err.msg,
                    ))
                }
                "EntityNotFoundException" => {
                    return RusotoError::Service(UpdatePrimaryEmailAddressError::EntityNotFound(
                        err.msg,
                    ))
                }
                "EntityStateException" => {
                    return RusotoError::Service(UpdatePrimaryEmailAddressError::EntityState(
                        err.msg,
                    ))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(UpdatePrimaryEmailAddressError::InvalidParameter(
                        err.msg,
                    ))
                }
                "MailDomainNotFoundException" => {
                    return RusotoError::Service(
                        UpdatePrimaryEmailAddressError::MailDomainNotFound(err.msg),
                    )
                }
                "MailDomainStateException" => {
                    return RusotoError::Service(UpdatePrimaryEmailAddressError::MailDomainState(
                        err.msg,
                    ))
                }
                "OrganizationNotFoundException" => {
                    return RusotoError::Service(
                        UpdatePrimaryEmailAddressError::OrganizationNotFound(err.msg),
                    )
                }
                "OrganizationStateException" => {
                    return RusotoError::Service(UpdatePrimaryEmailAddressError::OrganizationState(
                        err.msg,
                    ))
                }
                "UnsupportedOperationException" => {
                    return RusotoError::Service(
                        UpdatePrimaryEmailAddressError::UnsupportedOperation(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
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
}

impl UpdateResourceError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdateResourceError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "DirectoryUnavailableException" => {
                    return RusotoError::Service(UpdateResourceError::DirectoryUnavailable(err.msg))
                }
                "EmailAddressInUseException" => {
                    return RusotoError::Service(UpdateResourceError::EmailAddressInUse(err.msg))
                }
                "EntityNotFoundException" => {
                    return RusotoError::Service(UpdateResourceError::EntityNotFound(err.msg))
                }
                "EntityStateException" => {
                    return RusotoError::Service(UpdateResourceError::EntityState(err.msg))
                }
                "InvalidConfigurationException" => {
                    return RusotoError::Service(UpdateResourceError::InvalidConfiguration(err.msg))
                }
                "MailDomainNotFoundException" => {
                    return RusotoError::Service(UpdateResourceError::MailDomainNotFound(err.msg))
                }
                "MailDomainStateException" => {
                    return RusotoError::Service(UpdateResourceError::MailDomainState(err.msg))
                }
                "NameAvailabilityException" => {
                    return RusotoError::Service(UpdateResourceError::NameAvailability(err.msg))
                }
                "OrganizationNotFoundException" => {
                    return RusotoError::Service(UpdateResourceError::OrganizationNotFound(err.msg))
                }
                "OrganizationStateException" => {
                    return RusotoError::Service(UpdateResourceError::OrganizationState(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
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
#[derive(Clone)]
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
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body.as_ref() == b"null" {
                        body = bytes::Bytes::from_static(b"{}");
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
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body.as_ref() == b"null" {
                        body = bytes::Bytes::from_static(b"{}");
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
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body.as_ref() == b"null" {
                        body = bytes::Bytes::from_static(b"{}");
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
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body.as_ref() == b"null" {
                        body = bytes::Bytes::from_static(b"{}");
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
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body.as_ref() == b"null" {
                        body = bytes::Bytes::from_static(b"{}");
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
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body.as_ref() == b"null" {
                        body = bytes::Bytes::from_static(b"{}");
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
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body.as_ref() == b"null" {
                        body = bytes::Bytes::from_static(b"{}");
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
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body.as_ref() == b"null" {
                        body = bytes::Bytes::from_static(b"{}");
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
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body.as_ref() == b"null" {
                        body = bytes::Bytes::from_static(b"{}");
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
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body.as_ref() == b"null" {
                        body = bytes::Bytes::from_static(b"{}");
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
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body.as_ref() == b"null" {
                        body = bytes::Bytes::from_static(b"{}");
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
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body.as_ref() == b"null" {
                        body = bytes::Bytes::from_static(b"{}");
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
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body.as_ref() == b"null" {
                        body = bytes::Bytes::from_static(b"{}");
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
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body.as_ref() == b"null" {
                        body = bytes::Bytes::from_static(b"{}");
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
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body.as_ref() == b"null" {
                        body = bytes::Bytes::from_static(b"{}");
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
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body.as_ref() == b"null" {
                        body = bytes::Bytes::from_static(b"{}");
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
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body.as_ref() == b"null" {
                        body = bytes::Bytes::from_static(b"{}");
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
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body.as_ref() == b"null" {
                        body = bytes::Bytes::from_static(b"{}");
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
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body.as_ref() == b"null" {
                        body = bytes::Bytes::from_static(b"{}");
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
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body.as_ref() == b"null" {
                        body = bytes::Bytes::from_static(b"{}");
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
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body.as_ref() == b"null" {
                        body = bytes::Bytes::from_static(b"{}");
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
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body.as_ref() == b"null" {
                        body = bytes::Bytes::from_static(b"{}");
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
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body.as_ref() == b"null" {
                        body = bytes::Bytes::from_static(b"{}");
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
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body.as_ref() == b"null" {
                        body = bytes::Bytes::from_static(b"{}");
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
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body.as_ref() == b"null" {
                        body = bytes::Bytes::from_static(b"{}");
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
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body.as_ref() == b"null" {
                        body = bytes::Bytes::from_static(b"{}");
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
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body.as_ref() == b"null" {
                        body = bytes::Bytes::from_static(b"{}");
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
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body.as_ref() == b"null" {
                        body = bytes::Bytes::from_static(b"{}");
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
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body.as_ref() == b"null" {
                        body = bytes::Bytes::from_static(b"{}");
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
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body.as_ref() == b"null" {
                        body = bytes::Bytes::from_static(b"{}");
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
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body.as_ref() == b"null" {
                        body = bytes::Bytes::from_static(b"{}");
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

#[cfg(test)]
mod protocol_tests {}
