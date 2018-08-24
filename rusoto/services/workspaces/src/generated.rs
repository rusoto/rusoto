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
use rusoto_core::request::DispatchSignedRequest;
use rusoto_core::{Client, RusotoFuture};

use rusoto_core::credential::{CredentialsError, ProvideAwsCredentials};
use rusoto_core::request::HttpDispatchError;

use rusoto_core::signature::SignedRequest;
use serde_json;
use serde_json::from_str;
use serde_json::Value as SerdeJsonValue;
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct AssociateIpGroupsRequest {
    /// <p>The ID of the directory.</p>
    #[serde(rename = "DirectoryId")]
    pub directory_id: String,
    /// <p>The IDs of one or more IP access control groups.</p>
    #[serde(rename = "GroupIds")]
    pub group_ids: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct AssociateIpGroupsResult {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct AuthorizeIpRulesRequest {
    /// <p>The ID of the group.</p>
    #[serde(rename = "GroupId")]
    pub group_id: String,
    /// <p>The rules to add to the group.</p>
    #[serde(rename = "UserRules")]
    pub user_rules: Vec<IpRuleItem>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct AuthorizeIpRulesResult {}

/// <p>Information about the compute type.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct ComputeType {
    /// <p>The compute type.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct CreateIpGroupRequest {
    /// <p>The description of the group.</p>
    #[serde(rename = "GroupDesc")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_desc: Option<String>,
    /// <p>The name of the group.</p>
    #[serde(rename = "GroupName")]
    pub group_name: String,
    /// <p>The rules to add to the group.</p>
    #[serde(rename = "UserRules")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_rules: Option<Vec<IpRuleItem>>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct CreateIpGroupResult {
    /// <p>The ID of the group.</p>
    #[serde(rename = "GroupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_id: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct CreateTagsRequest {
    /// <p>The ID of the WorkSpace. To find this ID, use <a>DescribeWorkspaces</a>.</p>
    #[serde(rename = "ResourceId")]
    pub resource_id: String,
    /// <p>The tags. Each WorkSpace can have a maximum of 50 tags.</p>
    #[serde(rename = "Tags")]
    pub tags: Vec<Tag>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct CreateTagsResult {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct CreateWorkspacesRequest {
    /// <p>The WorkSpaces to create. You can specify up to 25 WorkSpaces.</p>
    #[serde(rename = "Workspaces")]
    pub workspaces: Vec<WorkspaceRequest>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct CreateWorkspacesResult {
    /// <p>Information about the WorkSpaces that could not be created.</p>
    #[serde(rename = "FailedRequests")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failed_requests: Option<Vec<FailedCreateWorkspaceRequest>>,
    /// <p>Information about the WorkSpaces that were created.</p> <p>Because this operation is asynchronous, the identifier returned is not immediately available for use with other operations. For example, if you call <a>DescribeWorkspaces</a> before the WorkSpace is created, the information returned can be incomplete.</p>
    #[serde(rename = "PendingRequests")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pending_requests: Option<Vec<Workspace>>,
}

/// <p>Information about defaults used to create a WorkSpace.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct DefaultWorkspaceCreationProperties {
    /// <p>The identifier of any security groups to apply to WorkSpaces when they are created.</p>
    #[serde(rename = "CustomSecurityGroupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_security_group_id: Option<String>,
    /// <p>The organizational unit (OU) in the directory for the WorkSpace machine accounts.</p>
    #[serde(rename = "DefaultOu")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_ou: Option<String>,
    /// <p>The public IP address to attach to all WorkSpaces that are created or rebuilt.</p>
    #[serde(rename = "EnableInternetAccess")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_internet_access: Option<bool>,
    /// <p>Indicates whether the directory is enabled for Amazon WorkDocs.</p>
    #[serde(rename = "EnableWorkDocs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_work_docs: Option<bool>,
    /// <p>Indicates whether the WorkSpace user is an administrator on the WorkSpace.</p>
    #[serde(rename = "UserEnabledAsLocalAdministrator")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_enabled_as_local_administrator: Option<bool>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DeleteIpGroupRequest {
    /// <p>The ID of the IP access control group.</p>
    #[serde(rename = "GroupId")]
    pub group_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct DeleteIpGroupResult {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DeleteTagsRequest {
    /// <p>The ID of the WorkSpace. To find this ID, use <a>DescribeWorkspaces</a>.</p>
    #[serde(rename = "ResourceId")]
    pub resource_id: String,
    /// <p>The tag keys.</p>
    #[serde(rename = "TagKeys")]
    pub tag_keys: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct DeleteTagsResult {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DescribeIpGroupsRequest {
    /// <p>The IDs of one or more IP access control groups.</p>
    #[serde(rename = "GroupIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_ids: Option<Vec<String>>,
    /// <p>The maximum number of items to return.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>The token for the next set of results. (You received this token from a previous call.)</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct DescribeIpGroupsResult {
    /// <p>The token to use to retrieve the next set of results, or null if there are no more results available. This token is valid for one day and must be used within that time frame.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>Information about the IP access control groups.</p>
    #[serde(rename = "Result")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<Vec<WorkspacesIpGroup>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DescribeTagsRequest {
    /// <p>The ID of the WorkSpace. To find this ID, use <a>DescribeWorkspaces</a>.</p>
    #[serde(rename = "ResourceId")]
    pub resource_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct DescribeTagsResult {
    /// <p>The tags.</p>
    #[serde(rename = "TagList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_list: Option<Vec<Tag>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DescribeWorkspaceBundlesRequest {
    /// <p>The IDs of the bundles. This parameter cannot be combined with any other filter.</p>
    #[serde(rename = "BundleIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bundle_ids: Option<Vec<String>>,
    /// <p>The token for the next set of results. (You received this token from a previous call.)</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The owner of the bundles. This parameter cannot be combined with any other filter.</p> <p>Specify <code>AMAZON</code> to describe the bundles provided by AWS or null to describe the bundles that belong to your account.</p>
    #[serde(rename = "Owner")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct DescribeWorkspaceBundlesResult {
    /// <p>Information about the bundles.</p>
    #[serde(rename = "Bundles")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bundles: Option<Vec<WorkspaceBundle>>,
    /// <p>The token to use to retrieve the next set of results, or null if there are no more results available. This token is valid for one day and must be used within that time frame.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DescribeWorkspaceDirectoriesRequest {
    /// <p>The identifiers of the directories. If the value is null, all directories are retrieved.</p>
    #[serde(rename = "DirectoryIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub directory_ids: Option<Vec<String>>,
    /// <p>The token for the next set of results. (You received this token from a previous call.)</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct DescribeWorkspaceDirectoriesResult {
    /// <p>Information about the directories.</p>
    #[serde(rename = "Directories")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub directories: Option<Vec<WorkspaceDirectory>>,
    /// <p>The token to use to retrieve the next set of results, or null if there are no more results available. This token is valid for one day and must be used within that time frame.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DescribeWorkspacesConnectionStatusRequest {
    /// <p>The token for the next set of results. (You received this token from a previous call.)</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The identifiers of the WorkSpaces. You can specify up to 25 WorkSpaces.</p>
    #[serde(rename = "WorkspaceIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workspace_ids: Option<Vec<String>>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct DescribeWorkspacesConnectionStatusResult {
    /// <p>The token to use to retrieve the next set of results, or null if there are no more results available.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>Information about the connection status of the WorkSpace.</p>
    #[serde(rename = "WorkspacesConnectionStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workspaces_connection_status: Option<Vec<WorkspaceConnectionStatus>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DescribeWorkspacesRequest {
    /// <p>The ID of the bundle. All WorkSpaces that are created from this bundle are retrieved. This parameter cannot be combined with any other filter.</p>
    #[serde(rename = "BundleId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bundle_id: Option<String>,
    /// <p>The ID of the directory. In addition, you can optionally specify a specific directory user (see <code>UserName</code>). This parameter cannot be combined with any other filter.</p>
    #[serde(rename = "DirectoryId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub directory_id: Option<String>,
    /// <p>The maximum number of items to return.</p>
    #[serde(rename = "Limit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// <p>The token for the next set of results. (You received this token from a previous call.)</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The name of the directory user. You must specify this parameter with <code>DirectoryId</code>.</p>
    #[serde(rename = "UserName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_name: Option<String>,
    /// <p>The IDs of the WorkSpaces. This parameter cannot be combined with any other filter.</p> <p>Because the <a>CreateWorkspaces</a> operation is asynchronous, the identifier it returns is not immediately available. If you immediately call <a>DescribeWorkspaces</a> with this identifier, no information is returned.</p>
    #[serde(rename = "WorkspaceIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workspace_ids: Option<Vec<String>>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct DescribeWorkspacesResult {
    /// <p>The token to use to retrieve the next set of results, or null if there are no more results available. This token is valid for one day and must be used within that time frame.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>Information about the WorkSpaces.</p> <p>Because <a>CreateWorkspaces</a> is an asynchronous operation, some of the returned information could be incomplete.</p>
    #[serde(rename = "Workspaces")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workspaces: Option<Vec<Workspace>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DisassociateIpGroupsRequest {
    /// <p>The ID of the directory.</p>
    #[serde(rename = "DirectoryId")]
    pub directory_id: String,
    /// <p>The IDs of one or more IP access control groups.</p>
    #[serde(rename = "GroupIds")]
    pub group_ids: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct DisassociateIpGroupsResult {}

/// <p>Information about a WorkSpace that could not be created.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct FailedCreateWorkspaceRequest {
    /// <p>The error code.</p>
    #[serde(rename = "ErrorCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_code: Option<String>,
    /// <p>The textual error message.</p>
    #[serde(rename = "ErrorMessage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
    /// <p>Information about the WorkSpace.</p>
    #[serde(rename = "WorkspaceRequest")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workspace_request: Option<WorkspaceRequest>,
}

/// <p>Information about a WorkSpace that could not be rebooted (<a>RebootWorkspaces</a>), rebuilt (<a>RebuildWorkspaces</a>), terminated (<a>TerminateWorkspaces</a>), started (<a>StartWorkspaces</a>), or stopped (<a>StopWorkspaces</a>).</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct FailedWorkspaceChangeRequest {
    /// <p>The error code.</p>
    #[serde(rename = "ErrorCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_code: Option<String>,
    /// <p>The textual error message.</p>
    #[serde(rename = "ErrorMessage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
    /// <p>The identifier of the WorkSpace.</p>
    #[serde(rename = "WorkspaceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workspace_id: Option<String>,
}

/// <p>Information about a rule for an IP access control group.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct IpRuleItem {
    /// <p>The IP address range, in CIDR notation.</p>
    #[serde(rename = "ipRule")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_rule: Option<String>,
    /// <p>The description.</p>
    #[serde(rename = "ruleDesc")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rule_desc: Option<String>,
}

/// <p>Information about a WorkSpace modification.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct ModificationState {
    /// <p>The resource.</p>
    #[serde(rename = "Resource")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource: Option<String>,
    /// <p>The modification state.</p>
    #[serde(rename = "State")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ModifyWorkspacePropertiesRequest {
    /// <p>The ID of the WorkSpace.</p>
    #[serde(rename = "WorkspaceId")]
    pub workspace_id: String,
    /// <p>The properties of the WorkSpace.</p>
    #[serde(rename = "WorkspaceProperties")]
    pub workspace_properties: WorkspaceProperties,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct ModifyWorkspacePropertiesResult {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ModifyWorkspaceStateRequest {
    /// <p>The ID of the WorkSpace.</p>
    #[serde(rename = "WorkspaceId")]
    pub workspace_id: String,
    /// <p>The WorkSpace state.</p>
    #[serde(rename = "WorkspaceState")]
    pub workspace_state: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct ModifyWorkspaceStateResult {}

/// <p>Information used to reboot a WorkSpace.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct RebootRequest {
    /// <p>The ID of the WorkSpace.</p>
    #[serde(rename = "WorkspaceId")]
    pub workspace_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct RebootWorkspacesRequest {
    /// <p>The WorkSpaces to reboot. You can specify up to 25 WorkSpaces.</p>
    #[serde(rename = "RebootWorkspaceRequests")]
    pub reboot_workspace_requests: Vec<RebootRequest>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct RebootWorkspacesResult {
    /// <p>Information about the WorkSpaces that could not be rebooted.</p>
    #[serde(rename = "FailedRequests")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failed_requests: Option<Vec<FailedWorkspaceChangeRequest>>,
}

/// <p>Information used to rebuild a WorkSpace.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct RebuildRequest {
    /// <p>The ID of the WorkSpace.</p>
    #[serde(rename = "WorkspaceId")]
    pub workspace_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct RebuildWorkspacesRequest {
    /// <p>The WorkSpace to rebuild. You can specify a single WorkSpace.</p>
    #[serde(rename = "RebuildWorkspaceRequests")]
    pub rebuild_workspace_requests: Vec<RebuildRequest>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct RebuildWorkspacesResult {
    /// <p>Information about the WorkSpace if it could not be rebuilt.</p>
    #[serde(rename = "FailedRequests")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failed_requests: Option<Vec<FailedWorkspaceChangeRequest>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct RevokeIpRulesRequest {
    /// <p>The ID of the group.</p>
    #[serde(rename = "GroupId")]
    pub group_id: String,
    /// <p>The rules to remove from the group.</p>
    #[serde(rename = "UserRules")]
    pub user_rules: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct RevokeIpRulesResult {}

/// <p>Information about the root volume for a WorkSpace bundle.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct RootStorage {
    /// <p>The size of the root volume.</p>
    #[serde(rename = "Capacity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capacity: Option<String>,
}

/// <p>Information used to start a WorkSpace.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct StartRequest {
    /// <p>The ID of the WorkSpace.</p>
    #[serde(rename = "WorkspaceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workspace_id: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct StartWorkspacesRequest {
    /// <p>The WorkSpaces to start. You can specify up to 25 WorkSpaces.</p>
    #[serde(rename = "StartWorkspaceRequests")]
    pub start_workspace_requests: Vec<StartRequest>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct StartWorkspacesResult {
    /// <p>Information about the WorkSpaces that could not be started.</p>
    #[serde(rename = "FailedRequests")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failed_requests: Option<Vec<FailedWorkspaceChangeRequest>>,
}

/// <p>Information used to stop a WorkSpace.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct StopRequest {
    /// <p>The ID of the WorkSpace.</p>
    #[serde(rename = "WorkspaceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workspace_id: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct StopWorkspacesRequest {
    /// <p>The WorkSpaces to stop. You can specify up to 25 WorkSpaces.</p>
    #[serde(rename = "StopWorkspaceRequests")]
    pub stop_workspace_requests: Vec<StopRequest>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct StopWorkspacesResult {
    /// <p>Information about the WorkSpaces that could not be stopped.</p>
    #[serde(rename = "FailedRequests")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failed_requests: Option<Vec<FailedWorkspaceChangeRequest>>,
}

/// <p>Information about a tag.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Tag {
    /// <p>The key of the tag.</p>
    #[serde(rename = "Key")]
    pub key: String,
    /// <p>The value of the tag.</p>
    #[serde(rename = "Value")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

/// <p>Information used to terminate a WorkSpace.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct TerminateRequest {
    /// <p>The ID of the WorkSpace.</p>
    #[serde(rename = "WorkspaceId")]
    pub workspace_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct TerminateWorkspacesRequest {
    /// <p>The WorkSpaces to terminate. You can specify up to 25 WorkSpaces.</p>
    #[serde(rename = "TerminateWorkspaceRequests")]
    pub terminate_workspace_requests: Vec<TerminateRequest>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct TerminateWorkspacesResult {
    /// <p>Information about the WorkSpaces that could not be terminated.</p>
    #[serde(rename = "FailedRequests")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failed_requests: Option<Vec<FailedWorkspaceChangeRequest>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct UpdateRulesOfIpGroupRequest {
    /// <p>The ID of the group.</p>
    #[serde(rename = "GroupId")]
    pub group_id: String,
    /// <p>One or more rules.</p>
    #[serde(rename = "UserRules")]
    pub user_rules: Vec<IpRuleItem>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct UpdateRulesOfIpGroupResult {}

/// <p>Information about the user storage for a WorkSpace bundle.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct UserStorage {
    /// <p>The size of the user storage.</p>
    #[serde(rename = "Capacity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capacity: Option<String>,
}

/// <p>Information about a WorkSpace.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct Workspace {
    /// <p>The identifier of the bundle used to create the WorkSpace.</p>
    #[serde(rename = "BundleId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bundle_id: Option<String>,
    /// <p>The name of the WorkSpace, as seen by the operating system.</p>
    #[serde(rename = "ComputerName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub computer_name: Option<String>,
    /// <p>The identifier of the AWS Directory Service directory for the WorkSpace.</p>
    #[serde(rename = "DirectoryId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub directory_id: Option<String>,
    /// <p>If the WorkSpace could not be created, contains the error code.</p>
    #[serde(rename = "ErrorCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_code: Option<String>,
    /// <p>If the WorkSpace could not be created, contains a textual error message that describes the failure.</p>
    #[serde(rename = "ErrorMessage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
    /// <p>The IP address of the WorkSpace.</p>
    #[serde(rename = "IpAddress")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_address: Option<String>,
    /// <p>The modification states of the WorkSpace.</p>
    #[serde(rename = "ModificationStates")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modification_states: Option<Vec<ModificationState>>,
    /// <p>Indicates whether the data stored on the root volume is encrypted.</p>
    #[serde(rename = "RootVolumeEncryptionEnabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub root_volume_encryption_enabled: Option<bool>,
    /// <p>The operational state of the WorkSpace.</p>
    #[serde(rename = "State")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    /// <p>The identifier of the subnet for the WorkSpace.</p>
    #[serde(rename = "SubnetId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subnet_id: Option<String>,
    /// <p>The user for the WorkSpace.</p>
    #[serde(rename = "UserName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_name: Option<String>,
    /// <p>Indicates whether the data stored on the user volume is encrypted.</p>
    #[serde(rename = "UserVolumeEncryptionEnabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_volume_encryption_enabled: Option<bool>,
    /// <p>The KMS key used to encrypt data stored on your WorkSpace.</p>
    #[serde(rename = "VolumeEncryptionKey")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volume_encryption_key: Option<String>,
    /// <p>The identifier of the WorkSpace.</p>
    #[serde(rename = "WorkspaceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workspace_id: Option<String>,
    /// <p>The properties of the WorkSpace.</p>
    #[serde(rename = "WorkspaceProperties")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workspace_properties: Option<WorkspaceProperties>,
}

/// <p>Information about a WorkSpace bundle.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct WorkspaceBundle {
    /// <p>The bundle identifier.</p>
    #[serde(rename = "BundleId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bundle_id: Option<String>,
    /// <p>The compute type. For more information, see <a href="http://aws.amazon.com/workspaces/details/#Amazon_WorkSpaces_Bundles">Amazon WorkSpaces Bundles</a>.</p>
    #[serde(rename = "ComputeType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compute_type: Option<ComputeType>,
    /// <p>A description.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The name of the bundle.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The owner of the bundle. This is the account identifier of the owner, or <code>AMAZON</code> if the bundle is provided by AWS.</p>
    #[serde(rename = "Owner")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner: Option<String>,
    /// <p>The size of the root volume.</p>
    #[serde(rename = "RootStorage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub root_storage: Option<RootStorage>,
    /// <p>The size of the user storage.</p>
    #[serde(rename = "UserStorage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_storage: Option<UserStorage>,
}

/// <p>Describes the connection status of a WorkSpace.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct WorkspaceConnectionStatus {
    /// <p>The connection state of the WorkSpace. The connection state is unknown if the WorkSpace is stopped.</p>
    #[serde(rename = "ConnectionState")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connection_state: Option<String>,
    /// <p>The timestamp of the connection state check.</p>
    #[serde(rename = "ConnectionStateCheckTimestamp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connection_state_check_timestamp: Option<f64>,
    /// <p>The timestamp of the last known user connection.</p>
    #[serde(rename = "LastKnownUserConnectionTimestamp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_known_user_connection_timestamp: Option<f64>,
    /// <p>The ID of the WorkSpace.</p>
    #[serde(rename = "WorkspaceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workspace_id: Option<String>,
}

/// <p>Information about an AWS Directory Service directory for use with Amazon WorkSpaces.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct WorkspaceDirectory {
    /// <p>The directory alias.</p>
    #[serde(rename = "Alias")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alias: Option<String>,
    /// <p>The user name for the service account.</p>
    #[serde(rename = "CustomerUserName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_user_name: Option<String>,
    /// <p>The directory identifier.</p>
    #[serde(rename = "DirectoryId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub directory_id: Option<String>,
    /// <p>The name of the directory.</p>
    #[serde(rename = "DirectoryName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub directory_name: Option<String>,
    /// <p>The directory type.</p>
    #[serde(rename = "DirectoryType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub directory_type: Option<String>,
    /// <p>The IP addresses of the DNS servers for the directory.</p>
    #[serde(rename = "DnsIpAddresses")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dns_ip_addresses: Option<Vec<String>>,
    /// <p>The identifier of the IAM role. This is the role that allows Amazon WorkSpaces to make calls to other services, such as Amazon EC2, on your behalf.</p>
    #[serde(rename = "IamRoleId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iam_role_id: Option<String>,
    /// <p>The registration code for the directory. This is the code that users enter in their Amazon WorkSpaces client application to connect to the directory.</p>
    #[serde(rename = "RegistrationCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub registration_code: Option<String>,
    /// <p>The state of the directory's registration with Amazon WorkSpaces</p>
    #[serde(rename = "State")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    /// <p>The identifiers of the subnets used with the directory.</p>
    #[serde(rename = "SubnetIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subnet_ids: Option<Vec<String>>,
    /// <p>The default creation properties for all WorkSpaces in the directory.</p>
    #[serde(rename = "WorkspaceCreationProperties")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workspace_creation_properties: Option<DefaultWorkspaceCreationProperties>,
    /// <p>The identifier of the security group that is assigned to new WorkSpaces.</p>
    #[serde(rename = "WorkspaceSecurityGroupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workspace_security_group_id: Option<String>,
    /// <p>The identifiers of the IP access control groups associated with the directory.</p>
    #[serde(rename = "ipGroupIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_group_ids: Option<Vec<String>>,
}

/// <p>Information about a WorkSpace.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WorkspaceProperties {
    /// <p>The compute type. For more information, see <a href="http://aws.amazon.com/workspaces/details/#Amazon_WorkSpaces_Bundles">Amazon WorkSpaces Bundles</a>.</p>
    #[serde(rename = "ComputeTypeName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compute_type_name: Option<String>,
    /// <p>The size of the root volume.</p>
    #[serde(rename = "RootVolumeSizeGib")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub root_volume_size_gib: Option<i64>,
    /// <p>The running mode. For more information, see <a href="http://docs.aws.amazon.com/workspaces/latest/adminguide/running-mode.html">Manage the WorkSpace Running Mode</a>.</p>
    #[serde(rename = "RunningMode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub running_mode: Option<String>,
    /// <p>The time after a user logs off when WorkSpaces are automatically stopped. Configured in 60 minute intervals.</p>
    #[serde(rename = "RunningModeAutoStopTimeoutInMinutes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub running_mode_auto_stop_timeout_in_minutes: Option<i64>,
    /// <p>The size of the user storage.</p>
    #[serde(rename = "UserVolumeSizeGib")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_volume_size_gib: Option<i64>,
}

/// <p>Information used to create a WorkSpace.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WorkspaceRequest {
    /// <p>The identifier of the bundle for the WorkSpace. You can use <a>DescribeWorkspaceBundles</a> to list the available bundles.</p>
    #[serde(rename = "BundleId")]
    pub bundle_id: String,
    /// <p>The identifier of the AWS Directory Service directory for the WorkSpace. You can use <a>DescribeWorkspaceDirectories</a> to list the available directories.</p>
    #[serde(rename = "DirectoryId")]
    pub directory_id: String,
    /// <p>Indicates whether the data stored on the root volume is encrypted.</p>
    #[serde(rename = "RootVolumeEncryptionEnabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub root_volume_encryption_enabled: Option<bool>,
    /// <p>The tags for the WorkSpace.</p>
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
    /// <p>The username of the user for the WorkSpace. This username must exist in the AWS Directory Service directory for the WorkSpace.</p>
    #[serde(rename = "UserName")]
    pub user_name: String,
    /// <p>Indicates whether the data stored on the user volume is encrypted.</p>
    #[serde(rename = "UserVolumeEncryptionEnabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_volume_encryption_enabled: Option<bool>,
    /// <p>The KMS key used to encrypt data stored on your WorkSpace.</p>
    #[serde(rename = "VolumeEncryptionKey")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volume_encryption_key: Option<String>,
    /// <p>The WorkSpace properties.</p>
    #[serde(rename = "WorkspaceProperties")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workspace_properties: Option<WorkspaceProperties>,
}

/// <p>Information about an IP access control group.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct WorkspacesIpGroup {
    /// <p>The description of the group.</p>
    #[serde(rename = "groupDesc")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_desc: Option<String>,
    /// <p>The ID of the group.</p>
    #[serde(rename = "groupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_id: Option<String>,
    /// <p>The name of the group.</p>
    #[serde(rename = "groupName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_name: Option<String>,
    /// <p>The rules.</p>
    #[serde(rename = "userRules")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_rules: Option<Vec<IpRuleItem>>,
}

/// Errors returned by AssociateIpGroups
#[derive(Debug, PartialEq)]
pub enum AssociateIpGroupsError {
    /// <p>The user is not authorized to access a resource.</p>
    AccessDenied(String),
    /// <p>One or more parameter values are not valid.</p>
    InvalidParameterValues(String),
    /// <p>The state of the resource is not valid for this operation.</p>
    InvalidResourceState(String),
    /// <p>This operation is not supported.</p>
    OperationNotSupported(String),
    /// <p>Your resource limits have been exceeded.</p>
    ResourceLimitExceeded(String),
    /// <p>The resource could not be found.</p>
    ResourceNotFound(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl AssociateIpGroupsError {
    pub fn from_body(body: &str) -> AssociateIpGroupsError {
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
                    "AccessDeniedException" => {
                        AssociateIpGroupsError::AccessDenied(String::from(error_message))
                    }
                    "InvalidParameterValuesException" => {
                        AssociateIpGroupsError::InvalidParameterValues(String::from(error_message))
                    }
                    "InvalidResourceStateException" => {
                        AssociateIpGroupsError::InvalidResourceState(String::from(error_message))
                    }
                    "OperationNotSupportedException" => {
                        AssociateIpGroupsError::OperationNotSupported(String::from(error_message))
                    }
                    "ResourceLimitExceededException" => {
                        AssociateIpGroupsError::ResourceLimitExceeded(String::from(error_message))
                    }
                    "ResourceNotFoundException" => {
                        AssociateIpGroupsError::ResourceNotFound(String::from(error_message))
                    }
                    "ValidationException" => {
                        AssociateIpGroupsError::Validation(error_message.to_string())
                    }
                    _ => AssociateIpGroupsError::Unknown(String::from(body)),
                }
            }
            Err(_) => AssociateIpGroupsError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for AssociateIpGroupsError {
    fn from(err: serde_json::error::Error) -> AssociateIpGroupsError {
        AssociateIpGroupsError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for AssociateIpGroupsError {
    fn from(err: CredentialsError) -> AssociateIpGroupsError {
        AssociateIpGroupsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for AssociateIpGroupsError {
    fn from(err: HttpDispatchError) -> AssociateIpGroupsError {
        AssociateIpGroupsError::HttpDispatch(err)
    }
}
impl From<io::Error> for AssociateIpGroupsError {
    fn from(err: io::Error) -> AssociateIpGroupsError {
        AssociateIpGroupsError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for AssociateIpGroupsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for AssociateIpGroupsError {
    fn description(&self) -> &str {
        match *self {
            AssociateIpGroupsError::AccessDenied(ref cause) => cause,
            AssociateIpGroupsError::InvalidParameterValues(ref cause) => cause,
            AssociateIpGroupsError::InvalidResourceState(ref cause) => cause,
            AssociateIpGroupsError::OperationNotSupported(ref cause) => cause,
            AssociateIpGroupsError::ResourceLimitExceeded(ref cause) => cause,
            AssociateIpGroupsError::ResourceNotFound(ref cause) => cause,
            AssociateIpGroupsError::Validation(ref cause) => cause,
            AssociateIpGroupsError::Credentials(ref err) => err.description(),
            AssociateIpGroupsError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            AssociateIpGroupsError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by AuthorizeIpRules
#[derive(Debug, PartialEq)]
pub enum AuthorizeIpRulesError {
    /// <p>The user is not authorized to access a resource.</p>
    AccessDenied(String),
    /// <p>One or more parameter values are not valid.</p>
    InvalidParameterValues(String),
    /// <p>The state of the resource is not valid for this operation.</p>
    InvalidResourceState(String),
    /// <p>Your resource limits have been exceeded.</p>
    ResourceLimitExceeded(String),
    /// <p>The resource could not be found.</p>
    ResourceNotFound(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl AuthorizeIpRulesError {
    pub fn from_body(body: &str) -> AuthorizeIpRulesError {
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
                    "AccessDeniedException" => {
                        AuthorizeIpRulesError::AccessDenied(String::from(error_message))
                    }
                    "InvalidParameterValuesException" => {
                        AuthorizeIpRulesError::InvalidParameterValues(String::from(error_message))
                    }
                    "InvalidResourceStateException" => {
                        AuthorizeIpRulesError::InvalidResourceState(String::from(error_message))
                    }
                    "ResourceLimitExceededException" => {
                        AuthorizeIpRulesError::ResourceLimitExceeded(String::from(error_message))
                    }
                    "ResourceNotFoundException" => {
                        AuthorizeIpRulesError::ResourceNotFound(String::from(error_message))
                    }
                    "ValidationException" => {
                        AuthorizeIpRulesError::Validation(error_message.to_string())
                    }
                    _ => AuthorizeIpRulesError::Unknown(String::from(body)),
                }
            }
            Err(_) => AuthorizeIpRulesError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for AuthorizeIpRulesError {
    fn from(err: serde_json::error::Error) -> AuthorizeIpRulesError {
        AuthorizeIpRulesError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for AuthorizeIpRulesError {
    fn from(err: CredentialsError) -> AuthorizeIpRulesError {
        AuthorizeIpRulesError::Credentials(err)
    }
}
impl From<HttpDispatchError> for AuthorizeIpRulesError {
    fn from(err: HttpDispatchError) -> AuthorizeIpRulesError {
        AuthorizeIpRulesError::HttpDispatch(err)
    }
}
impl From<io::Error> for AuthorizeIpRulesError {
    fn from(err: io::Error) -> AuthorizeIpRulesError {
        AuthorizeIpRulesError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for AuthorizeIpRulesError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for AuthorizeIpRulesError {
    fn description(&self) -> &str {
        match *self {
            AuthorizeIpRulesError::AccessDenied(ref cause) => cause,
            AuthorizeIpRulesError::InvalidParameterValues(ref cause) => cause,
            AuthorizeIpRulesError::InvalidResourceState(ref cause) => cause,
            AuthorizeIpRulesError::ResourceLimitExceeded(ref cause) => cause,
            AuthorizeIpRulesError::ResourceNotFound(ref cause) => cause,
            AuthorizeIpRulesError::Validation(ref cause) => cause,
            AuthorizeIpRulesError::Credentials(ref err) => err.description(),
            AuthorizeIpRulesError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            AuthorizeIpRulesError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by CreateIpGroup
#[derive(Debug, PartialEq)]
pub enum CreateIpGroupError {
    /// <p>The user is not authorized to access a resource.</p>
    AccessDenied(String),
    /// <p>One or more parameter values are not valid.</p>
    InvalidParameterValues(String),
    /// <p>The specified resource already exists.</p>
    ResourceAlreadyExists(String),
    /// <p>The resource could not be created.</p>
    ResourceCreationFailed(String),
    /// <p>Your resource limits have been exceeded.</p>
    ResourceLimitExceeded(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl CreateIpGroupError {
    pub fn from_body(body: &str) -> CreateIpGroupError {
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
                    "AccessDeniedException" => {
                        CreateIpGroupError::AccessDenied(String::from(error_message))
                    }
                    "InvalidParameterValuesException" => {
                        CreateIpGroupError::InvalidParameterValues(String::from(error_message))
                    }
                    "ResourceAlreadyExistsException" => {
                        CreateIpGroupError::ResourceAlreadyExists(String::from(error_message))
                    }
                    "ResourceCreationFailedException" => {
                        CreateIpGroupError::ResourceCreationFailed(String::from(error_message))
                    }
                    "ResourceLimitExceededException" => {
                        CreateIpGroupError::ResourceLimitExceeded(String::from(error_message))
                    }
                    "ValidationException" => {
                        CreateIpGroupError::Validation(error_message.to_string())
                    }
                    _ => CreateIpGroupError::Unknown(String::from(body)),
                }
            }
            Err(_) => CreateIpGroupError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for CreateIpGroupError {
    fn from(err: serde_json::error::Error) -> CreateIpGroupError {
        CreateIpGroupError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for CreateIpGroupError {
    fn from(err: CredentialsError) -> CreateIpGroupError {
        CreateIpGroupError::Credentials(err)
    }
}
impl From<HttpDispatchError> for CreateIpGroupError {
    fn from(err: HttpDispatchError) -> CreateIpGroupError {
        CreateIpGroupError::HttpDispatch(err)
    }
}
impl From<io::Error> for CreateIpGroupError {
    fn from(err: io::Error) -> CreateIpGroupError {
        CreateIpGroupError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for CreateIpGroupError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateIpGroupError {
    fn description(&self) -> &str {
        match *self {
            CreateIpGroupError::AccessDenied(ref cause) => cause,
            CreateIpGroupError::InvalidParameterValues(ref cause) => cause,
            CreateIpGroupError::ResourceAlreadyExists(ref cause) => cause,
            CreateIpGroupError::ResourceCreationFailed(ref cause) => cause,
            CreateIpGroupError::ResourceLimitExceeded(ref cause) => cause,
            CreateIpGroupError::Validation(ref cause) => cause,
            CreateIpGroupError::Credentials(ref err) => err.description(),
            CreateIpGroupError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            CreateIpGroupError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by CreateTags
#[derive(Debug, PartialEq)]
pub enum CreateTagsError {
    /// <p>One or more parameter values are not valid.</p>
    InvalidParameterValues(String),
    /// <p>Your resource limits have been exceeded.</p>
    ResourceLimitExceeded(String),
    /// <p>The resource could not be found.</p>
    ResourceNotFound(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl CreateTagsError {
    pub fn from_body(body: &str) -> CreateTagsError {
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
                    "InvalidParameterValuesException" => {
                        CreateTagsError::InvalidParameterValues(String::from(error_message))
                    }
                    "ResourceLimitExceededException" => {
                        CreateTagsError::ResourceLimitExceeded(String::from(error_message))
                    }
                    "ResourceNotFoundException" => {
                        CreateTagsError::ResourceNotFound(String::from(error_message))
                    }
                    "ValidationException" => CreateTagsError::Validation(error_message.to_string()),
                    _ => CreateTagsError::Unknown(String::from(body)),
                }
            }
            Err(_) => CreateTagsError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for CreateTagsError {
    fn from(err: serde_json::error::Error) -> CreateTagsError {
        CreateTagsError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for CreateTagsError {
    fn from(err: CredentialsError) -> CreateTagsError {
        CreateTagsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for CreateTagsError {
    fn from(err: HttpDispatchError) -> CreateTagsError {
        CreateTagsError::HttpDispatch(err)
    }
}
impl From<io::Error> for CreateTagsError {
    fn from(err: io::Error) -> CreateTagsError {
        CreateTagsError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for CreateTagsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateTagsError {
    fn description(&self) -> &str {
        match *self {
            CreateTagsError::InvalidParameterValues(ref cause) => cause,
            CreateTagsError::ResourceLimitExceeded(ref cause) => cause,
            CreateTagsError::ResourceNotFound(ref cause) => cause,
            CreateTagsError::Validation(ref cause) => cause,
            CreateTagsError::Credentials(ref err) => err.description(),
            CreateTagsError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            CreateTagsError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by CreateWorkspaces
#[derive(Debug, PartialEq)]
pub enum CreateWorkspacesError {
    /// <p>One or more parameter values are not valid.</p>
    InvalidParameterValues(String),
    /// <p>Your resource limits have been exceeded.</p>
    ResourceLimitExceeded(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl CreateWorkspacesError {
    pub fn from_body(body: &str) -> CreateWorkspacesError {
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
                    "InvalidParameterValuesException" => {
                        CreateWorkspacesError::InvalidParameterValues(String::from(error_message))
                    }
                    "ResourceLimitExceededException" => {
                        CreateWorkspacesError::ResourceLimitExceeded(String::from(error_message))
                    }
                    "ValidationException" => {
                        CreateWorkspacesError::Validation(error_message.to_string())
                    }
                    _ => CreateWorkspacesError::Unknown(String::from(body)),
                }
            }
            Err(_) => CreateWorkspacesError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for CreateWorkspacesError {
    fn from(err: serde_json::error::Error) -> CreateWorkspacesError {
        CreateWorkspacesError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for CreateWorkspacesError {
    fn from(err: CredentialsError) -> CreateWorkspacesError {
        CreateWorkspacesError::Credentials(err)
    }
}
impl From<HttpDispatchError> for CreateWorkspacesError {
    fn from(err: HttpDispatchError) -> CreateWorkspacesError {
        CreateWorkspacesError::HttpDispatch(err)
    }
}
impl From<io::Error> for CreateWorkspacesError {
    fn from(err: io::Error) -> CreateWorkspacesError {
        CreateWorkspacesError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for CreateWorkspacesError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateWorkspacesError {
    fn description(&self) -> &str {
        match *self {
            CreateWorkspacesError::InvalidParameterValues(ref cause) => cause,
            CreateWorkspacesError::ResourceLimitExceeded(ref cause) => cause,
            CreateWorkspacesError::Validation(ref cause) => cause,
            CreateWorkspacesError::Credentials(ref err) => err.description(),
            CreateWorkspacesError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            CreateWorkspacesError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DeleteIpGroup
#[derive(Debug, PartialEq)]
pub enum DeleteIpGroupError {
    /// <p>The user is not authorized to access a resource.</p>
    AccessDenied(String),
    /// <p>One or more parameter values are not valid.</p>
    InvalidParameterValues(String),
    /// <p>The resource is associated with a directory.</p>
    ResourceAssociated(String),
    /// <p>The resource could not be found.</p>
    ResourceNotFound(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl DeleteIpGroupError {
    pub fn from_body(body: &str) -> DeleteIpGroupError {
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
                    "AccessDeniedException" => {
                        DeleteIpGroupError::AccessDenied(String::from(error_message))
                    }
                    "InvalidParameterValuesException" => {
                        DeleteIpGroupError::InvalidParameterValues(String::from(error_message))
                    }
                    "ResourceAssociatedException" => {
                        DeleteIpGroupError::ResourceAssociated(String::from(error_message))
                    }
                    "ResourceNotFoundException" => {
                        DeleteIpGroupError::ResourceNotFound(String::from(error_message))
                    }
                    "ValidationException" => {
                        DeleteIpGroupError::Validation(error_message.to_string())
                    }
                    _ => DeleteIpGroupError::Unknown(String::from(body)),
                }
            }
            Err(_) => DeleteIpGroupError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DeleteIpGroupError {
    fn from(err: serde_json::error::Error) -> DeleteIpGroupError {
        DeleteIpGroupError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DeleteIpGroupError {
    fn from(err: CredentialsError) -> DeleteIpGroupError {
        DeleteIpGroupError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DeleteIpGroupError {
    fn from(err: HttpDispatchError) -> DeleteIpGroupError {
        DeleteIpGroupError::HttpDispatch(err)
    }
}
impl From<io::Error> for DeleteIpGroupError {
    fn from(err: io::Error) -> DeleteIpGroupError {
        DeleteIpGroupError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DeleteIpGroupError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteIpGroupError {
    fn description(&self) -> &str {
        match *self {
            DeleteIpGroupError::AccessDenied(ref cause) => cause,
            DeleteIpGroupError::InvalidParameterValues(ref cause) => cause,
            DeleteIpGroupError::ResourceAssociated(ref cause) => cause,
            DeleteIpGroupError::ResourceNotFound(ref cause) => cause,
            DeleteIpGroupError::Validation(ref cause) => cause,
            DeleteIpGroupError::Credentials(ref err) => err.description(),
            DeleteIpGroupError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            DeleteIpGroupError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DeleteTags
#[derive(Debug, PartialEq)]
pub enum DeleteTagsError {
    /// <p>One or more parameter values are not valid.</p>
    InvalidParameterValues(String),
    /// <p>The resource could not be found.</p>
    ResourceNotFound(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl DeleteTagsError {
    pub fn from_body(body: &str) -> DeleteTagsError {
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
                    "InvalidParameterValuesException" => {
                        DeleteTagsError::InvalidParameterValues(String::from(error_message))
                    }
                    "ResourceNotFoundException" => {
                        DeleteTagsError::ResourceNotFound(String::from(error_message))
                    }
                    "ValidationException" => DeleteTagsError::Validation(error_message.to_string()),
                    _ => DeleteTagsError::Unknown(String::from(body)),
                }
            }
            Err(_) => DeleteTagsError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DeleteTagsError {
    fn from(err: serde_json::error::Error) -> DeleteTagsError {
        DeleteTagsError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DeleteTagsError {
    fn from(err: CredentialsError) -> DeleteTagsError {
        DeleteTagsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DeleteTagsError {
    fn from(err: HttpDispatchError) -> DeleteTagsError {
        DeleteTagsError::HttpDispatch(err)
    }
}
impl From<io::Error> for DeleteTagsError {
    fn from(err: io::Error) -> DeleteTagsError {
        DeleteTagsError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DeleteTagsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteTagsError {
    fn description(&self) -> &str {
        match *self {
            DeleteTagsError::InvalidParameterValues(ref cause) => cause,
            DeleteTagsError::ResourceNotFound(ref cause) => cause,
            DeleteTagsError::Validation(ref cause) => cause,
            DeleteTagsError::Credentials(ref err) => err.description(),
            DeleteTagsError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            DeleteTagsError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DescribeIpGroups
#[derive(Debug, PartialEq)]
pub enum DescribeIpGroupsError {
    /// <p>The user is not authorized to access a resource.</p>
    AccessDenied(String),
    /// <p>One or more parameter values are not valid.</p>
    InvalidParameterValues(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl DescribeIpGroupsError {
    pub fn from_body(body: &str) -> DescribeIpGroupsError {
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
                    "AccessDeniedException" => {
                        DescribeIpGroupsError::AccessDenied(String::from(error_message))
                    }
                    "InvalidParameterValuesException" => {
                        DescribeIpGroupsError::InvalidParameterValues(String::from(error_message))
                    }
                    "ValidationException" => {
                        DescribeIpGroupsError::Validation(error_message.to_string())
                    }
                    _ => DescribeIpGroupsError::Unknown(String::from(body)),
                }
            }
            Err(_) => DescribeIpGroupsError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DescribeIpGroupsError {
    fn from(err: serde_json::error::Error) -> DescribeIpGroupsError {
        DescribeIpGroupsError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DescribeIpGroupsError {
    fn from(err: CredentialsError) -> DescribeIpGroupsError {
        DescribeIpGroupsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DescribeIpGroupsError {
    fn from(err: HttpDispatchError) -> DescribeIpGroupsError {
        DescribeIpGroupsError::HttpDispatch(err)
    }
}
impl From<io::Error> for DescribeIpGroupsError {
    fn from(err: io::Error) -> DescribeIpGroupsError {
        DescribeIpGroupsError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DescribeIpGroupsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeIpGroupsError {
    fn description(&self) -> &str {
        match *self {
            DescribeIpGroupsError::AccessDenied(ref cause) => cause,
            DescribeIpGroupsError::InvalidParameterValues(ref cause) => cause,
            DescribeIpGroupsError::Validation(ref cause) => cause,
            DescribeIpGroupsError::Credentials(ref err) => err.description(),
            DescribeIpGroupsError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            DescribeIpGroupsError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DescribeTags
#[derive(Debug, PartialEq)]
pub enum DescribeTagsError {
    /// <p>The resource could not be found.</p>
    ResourceNotFound(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl DescribeTagsError {
    pub fn from_body(body: &str) -> DescribeTagsError {
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
                    "ResourceNotFoundException" => {
                        DescribeTagsError::ResourceNotFound(String::from(error_message))
                    }
                    "ValidationException" => {
                        DescribeTagsError::Validation(error_message.to_string())
                    }
                    _ => DescribeTagsError::Unknown(String::from(body)),
                }
            }
            Err(_) => DescribeTagsError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DescribeTagsError {
    fn from(err: serde_json::error::Error) -> DescribeTagsError {
        DescribeTagsError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DescribeTagsError {
    fn from(err: CredentialsError) -> DescribeTagsError {
        DescribeTagsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DescribeTagsError {
    fn from(err: HttpDispatchError) -> DescribeTagsError {
        DescribeTagsError::HttpDispatch(err)
    }
}
impl From<io::Error> for DescribeTagsError {
    fn from(err: io::Error) -> DescribeTagsError {
        DescribeTagsError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DescribeTagsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeTagsError {
    fn description(&self) -> &str {
        match *self {
            DescribeTagsError::ResourceNotFound(ref cause) => cause,
            DescribeTagsError::Validation(ref cause) => cause,
            DescribeTagsError::Credentials(ref err) => err.description(),
            DescribeTagsError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            DescribeTagsError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DescribeWorkspaceBundles
#[derive(Debug, PartialEq)]
pub enum DescribeWorkspaceBundlesError {
    /// <p>One or more parameter values are not valid.</p>
    InvalidParameterValues(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl DescribeWorkspaceBundlesError {
    pub fn from_body(body: &str) -> DescribeWorkspaceBundlesError {
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
                    "InvalidParameterValuesException" => {
                        DescribeWorkspaceBundlesError::InvalidParameterValues(String::from(
                            error_message,
                        ))
                    }
                    "ValidationException" => {
                        DescribeWorkspaceBundlesError::Validation(error_message.to_string())
                    }
                    _ => DescribeWorkspaceBundlesError::Unknown(String::from(body)),
                }
            }
            Err(_) => DescribeWorkspaceBundlesError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DescribeWorkspaceBundlesError {
    fn from(err: serde_json::error::Error) -> DescribeWorkspaceBundlesError {
        DescribeWorkspaceBundlesError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DescribeWorkspaceBundlesError {
    fn from(err: CredentialsError) -> DescribeWorkspaceBundlesError {
        DescribeWorkspaceBundlesError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DescribeWorkspaceBundlesError {
    fn from(err: HttpDispatchError) -> DescribeWorkspaceBundlesError {
        DescribeWorkspaceBundlesError::HttpDispatch(err)
    }
}
impl From<io::Error> for DescribeWorkspaceBundlesError {
    fn from(err: io::Error) -> DescribeWorkspaceBundlesError {
        DescribeWorkspaceBundlesError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DescribeWorkspaceBundlesError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeWorkspaceBundlesError {
    fn description(&self) -> &str {
        match *self {
            DescribeWorkspaceBundlesError::InvalidParameterValues(ref cause) => cause,
            DescribeWorkspaceBundlesError::Validation(ref cause) => cause,
            DescribeWorkspaceBundlesError::Credentials(ref err) => err.description(),
            DescribeWorkspaceBundlesError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DescribeWorkspaceBundlesError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DescribeWorkspaceDirectories
#[derive(Debug, PartialEq)]
pub enum DescribeWorkspaceDirectoriesError {
    /// <p>One or more parameter values are not valid.</p>
    InvalidParameterValues(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl DescribeWorkspaceDirectoriesError {
    pub fn from_body(body: &str) -> DescribeWorkspaceDirectoriesError {
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
                    "InvalidParameterValuesException" => {
                        DescribeWorkspaceDirectoriesError::InvalidParameterValues(String::from(
                            error_message,
                        ))
                    }
                    "ValidationException" => {
                        DescribeWorkspaceDirectoriesError::Validation(error_message.to_string())
                    }
                    _ => DescribeWorkspaceDirectoriesError::Unknown(String::from(body)),
                }
            }
            Err(_) => DescribeWorkspaceDirectoriesError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DescribeWorkspaceDirectoriesError {
    fn from(err: serde_json::error::Error) -> DescribeWorkspaceDirectoriesError {
        DescribeWorkspaceDirectoriesError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DescribeWorkspaceDirectoriesError {
    fn from(err: CredentialsError) -> DescribeWorkspaceDirectoriesError {
        DescribeWorkspaceDirectoriesError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DescribeWorkspaceDirectoriesError {
    fn from(err: HttpDispatchError) -> DescribeWorkspaceDirectoriesError {
        DescribeWorkspaceDirectoriesError::HttpDispatch(err)
    }
}
impl From<io::Error> for DescribeWorkspaceDirectoriesError {
    fn from(err: io::Error) -> DescribeWorkspaceDirectoriesError {
        DescribeWorkspaceDirectoriesError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DescribeWorkspaceDirectoriesError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeWorkspaceDirectoriesError {
    fn description(&self) -> &str {
        match *self {
            DescribeWorkspaceDirectoriesError::InvalidParameterValues(ref cause) => cause,
            DescribeWorkspaceDirectoriesError::Validation(ref cause) => cause,
            DescribeWorkspaceDirectoriesError::Credentials(ref err) => err.description(),
            DescribeWorkspaceDirectoriesError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DescribeWorkspaceDirectoriesError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DescribeWorkspaces
#[derive(Debug, PartialEq)]
pub enum DescribeWorkspacesError {
    /// <p>One or more parameter values are not valid.</p>
    InvalidParameterValues(String),
    /// <p>The specified resource is not available.</p>
    ResourceUnavailable(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl DescribeWorkspacesError {
    pub fn from_body(body: &str) -> DescribeWorkspacesError {
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
                    "InvalidParameterValuesException" => {
                        DescribeWorkspacesError::InvalidParameterValues(String::from(error_message))
                    }
                    "ResourceUnavailableException" => {
                        DescribeWorkspacesError::ResourceUnavailable(String::from(error_message))
                    }
                    "ValidationException" => {
                        DescribeWorkspacesError::Validation(error_message.to_string())
                    }
                    _ => DescribeWorkspacesError::Unknown(String::from(body)),
                }
            }
            Err(_) => DescribeWorkspacesError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DescribeWorkspacesError {
    fn from(err: serde_json::error::Error) -> DescribeWorkspacesError {
        DescribeWorkspacesError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DescribeWorkspacesError {
    fn from(err: CredentialsError) -> DescribeWorkspacesError {
        DescribeWorkspacesError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DescribeWorkspacesError {
    fn from(err: HttpDispatchError) -> DescribeWorkspacesError {
        DescribeWorkspacesError::HttpDispatch(err)
    }
}
impl From<io::Error> for DescribeWorkspacesError {
    fn from(err: io::Error) -> DescribeWorkspacesError {
        DescribeWorkspacesError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DescribeWorkspacesError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeWorkspacesError {
    fn description(&self) -> &str {
        match *self {
            DescribeWorkspacesError::InvalidParameterValues(ref cause) => cause,
            DescribeWorkspacesError::ResourceUnavailable(ref cause) => cause,
            DescribeWorkspacesError::Validation(ref cause) => cause,
            DescribeWorkspacesError::Credentials(ref err) => err.description(),
            DescribeWorkspacesError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DescribeWorkspacesError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DescribeWorkspacesConnectionStatus
#[derive(Debug, PartialEq)]
pub enum DescribeWorkspacesConnectionStatusError {
    /// <p>One or more parameter values are not valid.</p>
    InvalidParameterValues(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl DescribeWorkspacesConnectionStatusError {
    pub fn from_body(body: &str) -> DescribeWorkspacesConnectionStatusError {
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
                    "InvalidParameterValuesException" => {
                        DescribeWorkspacesConnectionStatusError::InvalidParameterValues(
                            String::from(error_message),
                        )
                    }
                    "ValidationException" => DescribeWorkspacesConnectionStatusError::Validation(
                        error_message.to_string(),
                    ),
                    _ => DescribeWorkspacesConnectionStatusError::Unknown(String::from(body)),
                }
            }
            Err(_) => DescribeWorkspacesConnectionStatusError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DescribeWorkspacesConnectionStatusError {
    fn from(err: serde_json::error::Error) -> DescribeWorkspacesConnectionStatusError {
        DescribeWorkspacesConnectionStatusError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DescribeWorkspacesConnectionStatusError {
    fn from(err: CredentialsError) -> DescribeWorkspacesConnectionStatusError {
        DescribeWorkspacesConnectionStatusError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DescribeWorkspacesConnectionStatusError {
    fn from(err: HttpDispatchError) -> DescribeWorkspacesConnectionStatusError {
        DescribeWorkspacesConnectionStatusError::HttpDispatch(err)
    }
}
impl From<io::Error> for DescribeWorkspacesConnectionStatusError {
    fn from(err: io::Error) -> DescribeWorkspacesConnectionStatusError {
        DescribeWorkspacesConnectionStatusError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DescribeWorkspacesConnectionStatusError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeWorkspacesConnectionStatusError {
    fn description(&self) -> &str {
        match *self {
            DescribeWorkspacesConnectionStatusError::InvalidParameterValues(ref cause) => cause,
            DescribeWorkspacesConnectionStatusError::Validation(ref cause) => cause,
            DescribeWorkspacesConnectionStatusError::Credentials(ref err) => err.description(),
            DescribeWorkspacesConnectionStatusError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DescribeWorkspacesConnectionStatusError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DisassociateIpGroups
#[derive(Debug, PartialEq)]
pub enum DisassociateIpGroupsError {
    /// <p>The user is not authorized to access a resource.</p>
    AccessDenied(String),
    /// <p>One or more parameter values are not valid.</p>
    InvalidParameterValues(String),
    /// <p>The state of the resource is not valid for this operation.</p>
    InvalidResourceState(String),
    /// <p>The resource could not be found.</p>
    ResourceNotFound(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl DisassociateIpGroupsError {
    pub fn from_body(body: &str) -> DisassociateIpGroupsError {
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
                    "AccessDeniedException" => {
                        DisassociateIpGroupsError::AccessDenied(String::from(error_message))
                    }
                    "InvalidParameterValuesException" => {
                        DisassociateIpGroupsError::InvalidParameterValues(String::from(
                            error_message,
                        ))
                    }
                    "InvalidResourceStateException" => {
                        DisassociateIpGroupsError::InvalidResourceState(String::from(error_message))
                    }
                    "ResourceNotFoundException" => {
                        DisassociateIpGroupsError::ResourceNotFound(String::from(error_message))
                    }
                    "ValidationException" => {
                        DisassociateIpGroupsError::Validation(error_message.to_string())
                    }
                    _ => DisassociateIpGroupsError::Unknown(String::from(body)),
                }
            }
            Err(_) => DisassociateIpGroupsError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DisassociateIpGroupsError {
    fn from(err: serde_json::error::Error) -> DisassociateIpGroupsError {
        DisassociateIpGroupsError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DisassociateIpGroupsError {
    fn from(err: CredentialsError) -> DisassociateIpGroupsError {
        DisassociateIpGroupsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DisassociateIpGroupsError {
    fn from(err: HttpDispatchError) -> DisassociateIpGroupsError {
        DisassociateIpGroupsError::HttpDispatch(err)
    }
}
impl From<io::Error> for DisassociateIpGroupsError {
    fn from(err: io::Error) -> DisassociateIpGroupsError {
        DisassociateIpGroupsError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DisassociateIpGroupsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DisassociateIpGroupsError {
    fn description(&self) -> &str {
        match *self {
            DisassociateIpGroupsError::AccessDenied(ref cause) => cause,
            DisassociateIpGroupsError::InvalidParameterValues(ref cause) => cause,
            DisassociateIpGroupsError::InvalidResourceState(ref cause) => cause,
            DisassociateIpGroupsError::ResourceNotFound(ref cause) => cause,
            DisassociateIpGroupsError::Validation(ref cause) => cause,
            DisassociateIpGroupsError::Credentials(ref err) => err.description(),
            DisassociateIpGroupsError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DisassociateIpGroupsError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by ModifyWorkspaceProperties
#[derive(Debug, PartialEq)]
pub enum ModifyWorkspacePropertiesError {
    /// <p>The user is not authorized to access a resource.</p>
    AccessDenied(String),
    /// <p>One or more parameter values are not valid.</p>
    InvalidParameterValues(String),
    /// <p>The state of the resource is not valid for this operation.</p>
    InvalidResourceState(String),
    /// <p>The properties of this WorkSpace are currently being modified. Try again in a moment.</p>
    OperationInProgress(String),
    /// <p>The resource could not be found.</p>
    ResourceNotFound(String),
    /// <p>The specified resource is not available.</p>
    ResourceUnavailable(String),
    /// <p>The configuration of this WorkSpace is not supported for this operation. For more information, see the <a href="http://docs.aws.amazon.com/workspaces/latest/adminguide/">Amazon WorkSpaces Administration Guide</a>. </p>
    UnsupportedWorkspaceConfiguration(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl ModifyWorkspacePropertiesError {
    pub fn from_body(body: &str) -> ModifyWorkspacePropertiesError {
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
                    "AccessDeniedException" => {
                        ModifyWorkspacePropertiesError::AccessDenied(String::from(error_message))
                    }
                    "InvalidParameterValuesException" => {
                        ModifyWorkspacePropertiesError::InvalidParameterValues(String::from(
                            error_message,
                        ))
                    }
                    "InvalidResourceStateException" => {
                        ModifyWorkspacePropertiesError::InvalidResourceState(String::from(
                            error_message,
                        ))
                    }
                    "OperationInProgressException" => {
                        ModifyWorkspacePropertiesError::OperationInProgress(String::from(
                            error_message,
                        ))
                    }
                    "ResourceNotFoundException" => {
                        ModifyWorkspacePropertiesError::ResourceNotFound(String::from(
                            error_message,
                        ))
                    }
                    "ResourceUnavailableException" => {
                        ModifyWorkspacePropertiesError::ResourceUnavailable(String::from(
                            error_message,
                        ))
                    }
                    "UnsupportedWorkspaceConfigurationException" => {
                        ModifyWorkspacePropertiesError::UnsupportedWorkspaceConfiguration(
                            String::from(error_message),
                        )
                    }
                    "ValidationException" => {
                        ModifyWorkspacePropertiesError::Validation(error_message.to_string())
                    }
                    _ => ModifyWorkspacePropertiesError::Unknown(String::from(body)),
                }
            }
            Err(_) => ModifyWorkspacePropertiesError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for ModifyWorkspacePropertiesError {
    fn from(err: serde_json::error::Error) -> ModifyWorkspacePropertiesError {
        ModifyWorkspacePropertiesError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for ModifyWorkspacePropertiesError {
    fn from(err: CredentialsError) -> ModifyWorkspacePropertiesError {
        ModifyWorkspacePropertiesError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ModifyWorkspacePropertiesError {
    fn from(err: HttpDispatchError) -> ModifyWorkspacePropertiesError {
        ModifyWorkspacePropertiesError::HttpDispatch(err)
    }
}
impl From<io::Error> for ModifyWorkspacePropertiesError {
    fn from(err: io::Error) -> ModifyWorkspacePropertiesError {
        ModifyWorkspacePropertiesError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for ModifyWorkspacePropertiesError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ModifyWorkspacePropertiesError {
    fn description(&self) -> &str {
        match *self {
            ModifyWorkspacePropertiesError::AccessDenied(ref cause) => cause,
            ModifyWorkspacePropertiesError::InvalidParameterValues(ref cause) => cause,
            ModifyWorkspacePropertiesError::InvalidResourceState(ref cause) => cause,
            ModifyWorkspacePropertiesError::OperationInProgress(ref cause) => cause,
            ModifyWorkspacePropertiesError::ResourceNotFound(ref cause) => cause,
            ModifyWorkspacePropertiesError::ResourceUnavailable(ref cause) => cause,
            ModifyWorkspacePropertiesError::UnsupportedWorkspaceConfiguration(ref cause) => cause,
            ModifyWorkspacePropertiesError::Validation(ref cause) => cause,
            ModifyWorkspacePropertiesError::Credentials(ref err) => err.description(),
            ModifyWorkspacePropertiesError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            ModifyWorkspacePropertiesError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by ModifyWorkspaceState
#[derive(Debug, PartialEq)]
pub enum ModifyWorkspaceStateError {
    /// <p>One or more parameter values are not valid.</p>
    InvalidParameterValues(String),
    /// <p>The state of the resource is not valid for this operation.</p>
    InvalidResourceState(String),
    /// <p>The resource could not be found.</p>
    ResourceNotFound(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl ModifyWorkspaceStateError {
    pub fn from_body(body: &str) -> ModifyWorkspaceStateError {
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
                    "InvalidParameterValuesException" => {
                        ModifyWorkspaceStateError::InvalidParameterValues(String::from(
                            error_message,
                        ))
                    }
                    "InvalidResourceStateException" => {
                        ModifyWorkspaceStateError::InvalidResourceState(String::from(error_message))
                    }
                    "ResourceNotFoundException" => {
                        ModifyWorkspaceStateError::ResourceNotFound(String::from(error_message))
                    }
                    "ValidationException" => {
                        ModifyWorkspaceStateError::Validation(error_message.to_string())
                    }
                    _ => ModifyWorkspaceStateError::Unknown(String::from(body)),
                }
            }
            Err(_) => ModifyWorkspaceStateError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for ModifyWorkspaceStateError {
    fn from(err: serde_json::error::Error) -> ModifyWorkspaceStateError {
        ModifyWorkspaceStateError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for ModifyWorkspaceStateError {
    fn from(err: CredentialsError) -> ModifyWorkspaceStateError {
        ModifyWorkspaceStateError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ModifyWorkspaceStateError {
    fn from(err: HttpDispatchError) -> ModifyWorkspaceStateError {
        ModifyWorkspaceStateError::HttpDispatch(err)
    }
}
impl From<io::Error> for ModifyWorkspaceStateError {
    fn from(err: io::Error) -> ModifyWorkspaceStateError {
        ModifyWorkspaceStateError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for ModifyWorkspaceStateError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ModifyWorkspaceStateError {
    fn description(&self) -> &str {
        match *self {
            ModifyWorkspaceStateError::InvalidParameterValues(ref cause) => cause,
            ModifyWorkspaceStateError::InvalidResourceState(ref cause) => cause,
            ModifyWorkspaceStateError::ResourceNotFound(ref cause) => cause,
            ModifyWorkspaceStateError::Validation(ref cause) => cause,
            ModifyWorkspaceStateError::Credentials(ref err) => err.description(),
            ModifyWorkspaceStateError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            ModifyWorkspaceStateError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by RebootWorkspaces
#[derive(Debug, PartialEq)]
pub enum RebootWorkspacesError {
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl RebootWorkspacesError {
    pub fn from_body(body: &str) -> RebootWorkspacesError {
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
                    "ValidationException" => {
                        RebootWorkspacesError::Validation(error_message.to_string())
                    }
                    _ => RebootWorkspacesError::Unknown(String::from(body)),
                }
            }
            Err(_) => RebootWorkspacesError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for RebootWorkspacesError {
    fn from(err: serde_json::error::Error) -> RebootWorkspacesError {
        RebootWorkspacesError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for RebootWorkspacesError {
    fn from(err: CredentialsError) -> RebootWorkspacesError {
        RebootWorkspacesError::Credentials(err)
    }
}
impl From<HttpDispatchError> for RebootWorkspacesError {
    fn from(err: HttpDispatchError) -> RebootWorkspacesError {
        RebootWorkspacesError::HttpDispatch(err)
    }
}
impl From<io::Error> for RebootWorkspacesError {
    fn from(err: io::Error) -> RebootWorkspacesError {
        RebootWorkspacesError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for RebootWorkspacesError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for RebootWorkspacesError {
    fn description(&self) -> &str {
        match *self {
            RebootWorkspacesError::Validation(ref cause) => cause,
            RebootWorkspacesError::Credentials(ref err) => err.description(),
            RebootWorkspacesError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            RebootWorkspacesError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by RebuildWorkspaces
#[derive(Debug, PartialEq)]
pub enum RebuildWorkspacesError {
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl RebuildWorkspacesError {
    pub fn from_body(body: &str) -> RebuildWorkspacesError {
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
                    "ValidationException" => {
                        RebuildWorkspacesError::Validation(error_message.to_string())
                    }
                    _ => RebuildWorkspacesError::Unknown(String::from(body)),
                }
            }
            Err(_) => RebuildWorkspacesError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for RebuildWorkspacesError {
    fn from(err: serde_json::error::Error) -> RebuildWorkspacesError {
        RebuildWorkspacesError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for RebuildWorkspacesError {
    fn from(err: CredentialsError) -> RebuildWorkspacesError {
        RebuildWorkspacesError::Credentials(err)
    }
}
impl From<HttpDispatchError> for RebuildWorkspacesError {
    fn from(err: HttpDispatchError) -> RebuildWorkspacesError {
        RebuildWorkspacesError::HttpDispatch(err)
    }
}
impl From<io::Error> for RebuildWorkspacesError {
    fn from(err: io::Error) -> RebuildWorkspacesError {
        RebuildWorkspacesError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for RebuildWorkspacesError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for RebuildWorkspacesError {
    fn description(&self) -> &str {
        match *self {
            RebuildWorkspacesError::Validation(ref cause) => cause,
            RebuildWorkspacesError::Credentials(ref err) => err.description(),
            RebuildWorkspacesError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            RebuildWorkspacesError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by RevokeIpRules
#[derive(Debug, PartialEq)]
pub enum RevokeIpRulesError {
    /// <p>The user is not authorized to access a resource.</p>
    AccessDenied(String),
    /// <p>One or more parameter values are not valid.</p>
    InvalidParameterValues(String),
    /// <p>The state of the resource is not valid for this operation.</p>
    InvalidResourceState(String),
    /// <p>The resource could not be found.</p>
    ResourceNotFound(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl RevokeIpRulesError {
    pub fn from_body(body: &str) -> RevokeIpRulesError {
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
                    "AccessDeniedException" => {
                        RevokeIpRulesError::AccessDenied(String::from(error_message))
                    }
                    "InvalidParameterValuesException" => {
                        RevokeIpRulesError::InvalidParameterValues(String::from(error_message))
                    }
                    "InvalidResourceStateException" => {
                        RevokeIpRulesError::InvalidResourceState(String::from(error_message))
                    }
                    "ResourceNotFoundException" => {
                        RevokeIpRulesError::ResourceNotFound(String::from(error_message))
                    }
                    "ValidationException" => {
                        RevokeIpRulesError::Validation(error_message.to_string())
                    }
                    _ => RevokeIpRulesError::Unknown(String::from(body)),
                }
            }
            Err(_) => RevokeIpRulesError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for RevokeIpRulesError {
    fn from(err: serde_json::error::Error) -> RevokeIpRulesError {
        RevokeIpRulesError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for RevokeIpRulesError {
    fn from(err: CredentialsError) -> RevokeIpRulesError {
        RevokeIpRulesError::Credentials(err)
    }
}
impl From<HttpDispatchError> for RevokeIpRulesError {
    fn from(err: HttpDispatchError) -> RevokeIpRulesError {
        RevokeIpRulesError::HttpDispatch(err)
    }
}
impl From<io::Error> for RevokeIpRulesError {
    fn from(err: io::Error) -> RevokeIpRulesError {
        RevokeIpRulesError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for RevokeIpRulesError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for RevokeIpRulesError {
    fn description(&self) -> &str {
        match *self {
            RevokeIpRulesError::AccessDenied(ref cause) => cause,
            RevokeIpRulesError::InvalidParameterValues(ref cause) => cause,
            RevokeIpRulesError::InvalidResourceState(ref cause) => cause,
            RevokeIpRulesError::ResourceNotFound(ref cause) => cause,
            RevokeIpRulesError::Validation(ref cause) => cause,
            RevokeIpRulesError::Credentials(ref err) => err.description(),
            RevokeIpRulesError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            RevokeIpRulesError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by StartWorkspaces
#[derive(Debug, PartialEq)]
pub enum StartWorkspacesError {
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl StartWorkspacesError {
    pub fn from_body(body: &str) -> StartWorkspacesError {
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
                    "ValidationException" => {
                        StartWorkspacesError::Validation(error_message.to_string())
                    }
                    _ => StartWorkspacesError::Unknown(String::from(body)),
                }
            }
            Err(_) => StartWorkspacesError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for StartWorkspacesError {
    fn from(err: serde_json::error::Error) -> StartWorkspacesError {
        StartWorkspacesError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for StartWorkspacesError {
    fn from(err: CredentialsError) -> StartWorkspacesError {
        StartWorkspacesError::Credentials(err)
    }
}
impl From<HttpDispatchError> for StartWorkspacesError {
    fn from(err: HttpDispatchError) -> StartWorkspacesError {
        StartWorkspacesError::HttpDispatch(err)
    }
}
impl From<io::Error> for StartWorkspacesError {
    fn from(err: io::Error) -> StartWorkspacesError {
        StartWorkspacesError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for StartWorkspacesError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for StartWorkspacesError {
    fn description(&self) -> &str {
        match *self {
            StartWorkspacesError::Validation(ref cause) => cause,
            StartWorkspacesError::Credentials(ref err) => err.description(),
            StartWorkspacesError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            StartWorkspacesError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by StopWorkspaces
#[derive(Debug, PartialEq)]
pub enum StopWorkspacesError {
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl StopWorkspacesError {
    pub fn from_body(body: &str) -> StopWorkspacesError {
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
                    "ValidationException" => {
                        StopWorkspacesError::Validation(error_message.to_string())
                    }
                    _ => StopWorkspacesError::Unknown(String::from(body)),
                }
            }
            Err(_) => StopWorkspacesError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for StopWorkspacesError {
    fn from(err: serde_json::error::Error) -> StopWorkspacesError {
        StopWorkspacesError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for StopWorkspacesError {
    fn from(err: CredentialsError) -> StopWorkspacesError {
        StopWorkspacesError::Credentials(err)
    }
}
impl From<HttpDispatchError> for StopWorkspacesError {
    fn from(err: HttpDispatchError) -> StopWorkspacesError {
        StopWorkspacesError::HttpDispatch(err)
    }
}
impl From<io::Error> for StopWorkspacesError {
    fn from(err: io::Error) -> StopWorkspacesError {
        StopWorkspacesError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for StopWorkspacesError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for StopWorkspacesError {
    fn description(&self) -> &str {
        match *self {
            StopWorkspacesError::Validation(ref cause) => cause,
            StopWorkspacesError::Credentials(ref err) => err.description(),
            StopWorkspacesError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            StopWorkspacesError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by TerminateWorkspaces
#[derive(Debug, PartialEq)]
pub enum TerminateWorkspacesError {
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl TerminateWorkspacesError {
    pub fn from_body(body: &str) -> TerminateWorkspacesError {
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
                    "ValidationException" => {
                        TerminateWorkspacesError::Validation(error_message.to_string())
                    }
                    _ => TerminateWorkspacesError::Unknown(String::from(body)),
                }
            }
            Err(_) => TerminateWorkspacesError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for TerminateWorkspacesError {
    fn from(err: serde_json::error::Error) -> TerminateWorkspacesError {
        TerminateWorkspacesError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for TerminateWorkspacesError {
    fn from(err: CredentialsError) -> TerminateWorkspacesError {
        TerminateWorkspacesError::Credentials(err)
    }
}
impl From<HttpDispatchError> for TerminateWorkspacesError {
    fn from(err: HttpDispatchError) -> TerminateWorkspacesError {
        TerminateWorkspacesError::HttpDispatch(err)
    }
}
impl From<io::Error> for TerminateWorkspacesError {
    fn from(err: io::Error) -> TerminateWorkspacesError {
        TerminateWorkspacesError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for TerminateWorkspacesError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for TerminateWorkspacesError {
    fn description(&self) -> &str {
        match *self {
            TerminateWorkspacesError::Validation(ref cause) => cause,
            TerminateWorkspacesError::Credentials(ref err) => err.description(),
            TerminateWorkspacesError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            TerminateWorkspacesError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by UpdateRulesOfIpGroup
#[derive(Debug, PartialEq)]
pub enum UpdateRulesOfIpGroupError {
    /// <p>The user is not authorized to access a resource.</p>
    AccessDenied(String),
    /// <p>One or more parameter values are not valid.</p>
    InvalidParameterValues(String),
    /// <p>The state of the resource is not valid for this operation.</p>
    InvalidResourceState(String),
    /// <p>Your resource limits have been exceeded.</p>
    ResourceLimitExceeded(String),
    /// <p>The resource could not be found.</p>
    ResourceNotFound(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl UpdateRulesOfIpGroupError {
    pub fn from_body(body: &str) -> UpdateRulesOfIpGroupError {
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
                    "AccessDeniedException" => {
                        UpdateRulesOfIpGroupError::AccessDenied(String::from(error_message))
                    }
                    "InvalidParameterValuesException" => {
                        UpdateRulesOfIpGroupError::InvalidParameterValues(String::from(
                            error_message,
                        ))
                    }
                    "InvalidResourceStateException" => {
                        UpdateRulesOfIpGroupError::InvalidResourceState(String::from(error_message))
                    }
                    "ResourceLimitExceededException" => {
                        UpdateRulesOfIpGroupError::ResourceLimitExceeded(String::from(
                            error_message,
                        ))
                    }
                    "ResourceNotFoundException" => {
                        UpdateRulesOfIpGroupError::ResourceNotFound(String::from(error_message))
                    }
                    "ValidationException" => {
                        UpdateRulesOfIpGroupError::Validation(error_message.to_string())
                    }
                    _ => UpdateRulesOfIpGroupError::Unknown(String::from(body)),
                }
            }
            Err(_) => UpdateRulesOfIpGroupError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for UpdateRulesOfIpGroupError {
    fn from(err: serde_json::error::Error) -> UpdateRulesOfIpGroupError {
        UpdateRulesOfIpGroupError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for UpdateRulesOfIpGroupError {
    fn from(err: CredentialsError) -> UpdateRulesOfIpGroupError {
        UpdateRulesOfIpGroupError::Credentials(err)
    }
}
impl From<HttpDispatchError> for UpdateRulesOfIpGroupError {
    fn from(err: HttpDispatchError) -> UpdateRulesOfIpGroupError {
        UpdateRulesOfIpGroupError::HttpDispatch(err)
    }
}
impl From<io::Error> for UpdateRulesOfIpGroupError {
    fn from(err: io::Error) -> UpdateRulesOfIpGroupError {
        UpdateRulesOfIpGroupError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for UpdateRulesOfIpGroupError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for UpdateRulesOfIpGroupError {
    fn description(&self) -> &str {
        match *self {
            UpdateRulesOfIpGroupError::AccessDenied(ref cause) => cause,
            UpdateRulesOfIpGroupError::InvalidParameterValues(ref cause) => cause,
            UpdateRulesOfIpGroupError::InvalidResourceState(ref cause) => cause,
            UpdateRulesOfIpGroupError::ResourceLimitExceeded(ref cause) => cause,
            UpdateRulesOfIpGroupError::ResourceNotFound(ref cause) => cause,
            UpdateRulesOfIpGroupError::Validation(ref cause) => cause,
            UpdateRulesOfIpGroupError::Credentials(ref err) => err.description(),
            UpdateRulesOfIpGroupError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            UpdateRulesOfIpGroupError::Unknown(ref cause) => cause,
        }
    }
}
/// Trait representing the capabilities of the Amazon WorkSpaces API. Amazon WorkSpaces clients implement this trait.
pub trait Workspaces {
    /// <p>Associates the specified IP access control group with the specified directory.</p>
    fn associate_ip_groups(
        &self,
        input: AssociateIpGroupsRequest,
    ) -> RusotoFuture<AssociateIpGroupsResult, AssociateIpGroupsError>;

    /// <p>Adds one or more rules to the specified IP access control group.</p> <p>This action gives users permission to access their WorkSpaces from the CIDR address ranges specified in the rules.</p>
    fn authorize_ip_rules(
        &self,
        input: AuthorizeIpRulesRequest,
    ) -> RusotoFuture<AuthorizeIpRulesResult, AuthorizeIpRulesError>;

    /// <p>Creates an IP access control group.</p> <p>An IP access control group provides you with the ability to control the IP addresses from which users are allowed to access their WorkSpaces. To specify the CIDR address ranges, add rules to your IP access control group and then associate the group with your directory. You can add rules when you create the group or at any time using <a>AuthorizeIpRules</a>.</p> <p>There is a default IP access control group associated with your directory. If you don't associate an IP access control group with your directory, the default group is used. The default group includes a default rule that allows users to access their WorkSpaces from anywhere. You cannot modify the default IP access control group for your directory.</p>
    fn create_ip_group(
        &self,
        input: CreateIpGroupRequest,
    ) -> RusotoFuture<CreateIpGroupResult, CreateIpGroupError>;

    /// <p>Creates the specified tags for the specified WorkSpace.</p>
    fn create_tags(
        &self,
        input: CreateTagsRequest,
    ) -> RusotoFuture<CreateTagsResult, CreateTagsError>;

    /// <p>Creates one or more WorkSpaces.</p> <p>This operation is asynchronous and returns before the WorkSpaces are created.</p>
    fn create_workspaces(
        &self,
        input: CreateWorkspacesRequest,
    ) -> RusotoFuture<CreateWorkspacesResult, CreateWorkspacesError>;

    /// <p>Deletes the specified IP access control group.</p> <p>You cannot delete an IP access control group that is associated with a directory.</p>
    fn delete_ip_group(
        &self,
        input: DeleteIpGroupRequest,
    ) -> RusotoFuture<DeleteIpGroupResult, DeleteIpGroupError>;

    /// <p>Deletes the specified tags from the specified WorkSpace.</p>
    fn delete_tags(
        &self,
        input: DeleteTagsRequest,
    ) -> RusotoFuture<DeleteTagsResult, DeleteTagsError>;

    /// <p>Describes one or more of your IP access control groups.</p>
    fn describe_ip_groups(
        &self,
        input: DescribeIpGroupsRequest,
    ) -> RusotoFuture<DescribeIpGroupsResult, DescribeIpGroupsError>;

    /// <p>Describes the specified tags for the specified WorkSpace.</p>
    fn describe_tags(
        &self,
        input: DescribeTagsRequest,
    ) -> RusotoFuture<DescribeTagsResult, DescribeTagsError>;

    /// <p>Describes the available WorkSpace bundles.</p> <p>You can filter the results using either bundle ID or owner, but not both.</p>
    fn describe_workspace_bundles(
        &self,
        input: DescribeWorkspaceBundlesRequest,
    ) -> RusotoFuture<DescribeWorkspaceBundlesResult, DescribeWorkspaceBundlesError>;

    /// <p>Describes the available AWS Directory Service directories that are registered with Amazon WorkSpaces.</p>
    fn describe_workspace_directories(
        &self,
        input: DescribeWorkspaceDirectoriesRequest,
    ) -> RusotoFuture<DescribeWorkspaceDirectoriesResult, DescribeWorkspaceDirectoriesError>;

    /// <p>Describes the specified WorkSpaces.</p> <p>You can filter the results using bundle ID, directory ID, or owner, but you can specify only one filter at a time.</p>
    fn describe_workspaces(
        &self,
        input: DescribeWorkspacesRequest,
    ) -> RusotoFuture<DescribeWorkspacesResult, DescribeWorkspacesError>;

    /// <p>Describes the connection status of the specified WorkSpaces.</p>
    fn describe_workspaces_connection_status(
        &self,
        input: DescribeWorkspacesConnectionStatusRequest,
    ) -> RusotoFuture<
        DescribeWorkspacesConnectionStatusResult,
        DescribeWorkspacesConnectionStatusError,
    >;

    /// <p>Disassociates the specified IP access control group from the specified directory.</p>
    fn disassociate_ip_groups(
        &self,
        input: DisassociateIpGroupsRequest,
    ) -> RusotoFuture<DisassociateIpGroupsResult, DisassociateIpGroupsError>;

    /// <p>Modifies the specified WorkSpace properties.</p>
    fn modify_workspace_properties(
        &self,
        input: ModifyWorkspacePropertiesRequest,
    ) -> RusotoFuture<ModifyWorkspacePropertiesResult, ModifyWorkspacePropertiesError>;

    /// <p>Sets the state of the specified WorkSpace.</p> <p>To maintain a WorkSpace without being interrupted, set the WorkSpace state to <code>ADMIN_MAINTENANCE</code>. WorkSpaces in this state do not respond to requests to reboot, stop, start, or rebuild. An AutoStop WorkSpace in this state is not stopped. Users can log into a WorkSpace in the <code>ADMIN_MAINTENANCE</code> state.</p>
    fn modify_workspace_state(
        &self,
        input: ModifyWorkspaceStateRequest,
    ) -> RusotoFuture<ModifyWorkspaceStateResult, ModifyWorkspaceStateError>;

    /// <p>Reboots the specified WorkSpaces.</p> <p>You cannot reboot a WorkSpace unless its state is <code>AVAILABLE</code> or <code>UNHEALTHY</code>.</p> <p>This operation is asynchronous and returns before the WorkSpaces have rebooted.</p>
    fn reboot_workspaces(
        &self,
        input: RebootWorkspacesRequest,
    ) -> RusotoFuture<RebootWorkspacesResult, RebootWorkspacesError>;

    /// <p>Rebuilds the specified WorkSpace.</p> <p>You cannot rebuild a WorkSpace unless its state is <code>AVAILABLE</code>, <code>ERROR</code>, or <code>UNHEALTHY</code>.</p> <p>Rebuilding a WorkSpace is a potentially destructive action that can result in the loss of data. For more information, see <a href="http://docs.aws.amazon.com/workspaces/latest/adminguide/reset-workspace.html">Rebuild a WorkSpace</a>.</p> <p>This operation is asynchronous and returns before the WorkSpaces have been completely rebuilt.</p>
    fn rebuild_workspaces(
        &self,
        input: RebuildWorkspacesRequest,
    ) -> RusotoFuture<RebuildWorkspacesResult, RebuildWorkspacesError>;

    /// <p>Removes one or more rules from the specified IP access control group.</p>
    fn revoke_ip_rules(
        &self,
        input: RevokeIpRulesRequest,
    ) -> RusotoFuture<RevokeIpRulesResult, RevokeIpRulesError>;

    /// <p>Starts the specified WorkSpaces.</p> <p>You cannot start a WorkSpace unless it has a running mode of <code>AutoStop</code> and a state of <code>STOPPED</code>.</p>
    fn start_workspaces(
        &self,
        input: StartWorkspacesRequest,
    ) -> RusotoFuture<StartWorkspacesResult, StartWorkspacesError>;

    /// <p> Stops the specified WorkSpaces.</p> <p>You cannot stop a WorkSpace unless it has a running mode of <code>AutoStop</code> and a state of <code>AVAILABLE</code>, <code>IMPAIRED</code>, <code>UNHEALTHY</code>, or <code>ERROR</code>.</p>
    fn stop_workspaces(
        &self,
        input: StopWorkspacesRequest,
    ) -> RusotoFuture<StopWorkspacesResult, StopWorkspacesError>;

    /// <p>Terminates the specified WorkSpaces.</p> <p>Terminating a WorkSpace is a permanent action and cannot be undone. The user's data is destroyed. If you need to archive any user data, contact Amazon Web Services before terminating the WorkSpace.</p> <p>You can terminate a WorkSpace that is in any state except <code>SUSPENDED</code>.</p> <p>This operation is asynchronous and returns before the WorkSpaces have been completely terminated.</p>
    fn terminate_workspaces(
        &self,
        input: TerminateWorkspacesRequest,
    ) -> RusotoFuture<TerminateWorkspacesResult, TerminateWorkspacesError>;

    /// <p>Replaces the current rules of the specified IP access control group with the specified rules.</p>
    fn update_rules_of_ip_group(
        &self,
        input: UpdateRulesOfIpGroupRequest,
    ) -> RusotoFuture<UpdateRulesOfIpGroupResult, UpdateRulesOfIpGroupError>;
}
/// A client for the Amazon WorkSpaces API.
pub struct WorkspacesClient {
    client: Client,
    region: region::Region,
}

impl WorkspacesClient {
    /// Creates a client backed by the default tokio event loop.
    ///
    /// The client will use the default credentials provider and tls client.
    pub fn new(region: region::Region) -> WorkspacesClient {
        WorkspacesClient {
            client: Client::shared(),
            region: region,
        }
    }

    pub fn new_with<P, D>(
        request_dispatcher: D,
        credentials_provider: P,
        region: region::Region,
    ) -> WorkspacesClient
    where
        P: ProvideAwsCredentials + Send + Sync + 'static,
        P::Future: Send,
        D: DispatchSignedRequest + Send + Sync + 'static,
        D::Future: Send,
    {
        WorkspacesClient {
            client: Client::new_with(credentials_provider, request_dispatcher),
            region: region,
        }
    }
}

impl Workspaces for WorkspacesClient {
    /// <p>Associates the specified IP access control group with the specified directory.</p>
    fn associate_ip_groups(
        &self,
        input: AssociateIpGroupsRequest,
    ) -> RusotoFuture<AssociateIpGroupsResult, AssociateIpGroupsError> {
        let mut request = SignedRequest::new("POST", "workspaces", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "WorkspacesService.AssociateIpGroups");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<AssociateIpGroupsResult>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(AssociateIpGroupsError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        })
    }

    /// <p>Adds one or more rules to the specified IP access control group.</p> <p>This action gives users permission to access their WorkSpaces from the CIDR address ranges specified in the rules.</p>
    fn authorize_ip_rules(
        &self,
        input: AuthorizeIpRulesRequest,
    ) -> RusotoFuture<AuthorizeIpRulesResult, AuthorizeIpRulesError> {
        let mut request = SignedRequest::new("POST", "workspaces", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "WorkspacesService.AuthorizeIpRules");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<AuthorizeIpRulesResult>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(AuthorizeIpRulesError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        })
    }

    /// <p>Creates an IP access control group.</p> <p>An IP access control group provides you with the ability to control the IP addresses from which users are allowed to access their WorkSpaces. To specify the CIDR address ranges, add rules to your IP access control group and then associate the group with your directory. You can add rules when you create the group or at any time using <a>AuthorizeIpRules</a>.</p> <p>There is a default IP access control group associated with your directory. If you don't associate an IP access control group with your directory, the default group is used. The default group includes a default rule that allows users to access their WorkSpaces from anywhere. You cannot modify the default IP access control group for your directory.</p>
    fn create_ip_group(
        &self,
        input: CreateIpGroupRequest,
    ) -> RusotoFuture<CreateIpGroupResult, CreateIpGroupError> {
        let mut request = SignedRequest::new("POST", "workspaces", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "WorkspacesService.CreateIpGroup");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<CreateIpGroupResult>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(CreateIpGroupError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        })
    }

    /// <p>Creates the specified tags for the specified WorkSpace.</p>
    fn create_tags(
        &self,
        input: CreateTagsRequest,
    ) -> RusotoFuture<CreateTagsResult, CreateTagsError> {
        let mut request = SignedRequest::new("POST", "workspaces", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "WorkspacesService.CreateTags");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<CreateTagsResult>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(CreateTagsError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        })
    }

    /// <p>Creates one or more WorkSpaces.</p> <p>This operation is asynchronous and returns before the WorkSpaces are created.</p>
    fn create_workspaces(
        &self,
        input: CreateWorkspacesRequest,
    ) -> RusotoFuture<CreateWorkspacesResult, CreateWorkspacesError> {
        let mut request = SignedRequest::new("POST", "workspaces", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "WorkspacesService.CreateWorkspaces");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<CreateWorkspacesResult>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(CreateWorkspacesError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        })
    }

    /// <p>Deletes the specified IP access control group.</p> <p>You cannot delete an IP access control group that is associated with a directory.</p>
    fn delete_ip_group(
        &self,
        input: DeleteIpGroupRequest,
    ) -> RusotoFuture<DeleteIpGroupResult, DeleteIpGroupError> {
        let mut request = SignedRequest::new("POST", "workspaces", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "WorkspacesService.DeleteIpGroup");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<DeleteIpGroupResult>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(DeleteIpGroupError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        })
    }

    /// <p>Deletes the specified tags from the specified WorkSpace.</p>
    fn delete_tags(
        &self,
        input: DeleteTagsRequest,
    ) -> RusotoFuture<DeleteTagsResult, DeleteTagsError> {
        let mut request = SignedRequest::new("POST", "workspaces", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "WorkspacesService.DeleteTags");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<DeleteTagsResult>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(DeleteTagsError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        })
    }

    /// <p>Describes one or more of your IP access control groups.</p>
    fn describe_ip_groups(
        &self,
        input: DescribeIpGroupsRequest,
    ) -> RusotoFuture<DescribeIpGroupsResult, DescribeIpGroupsError> {
        let mut request = SignedRequest::new("POST", "workspaces", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "WorkspacesService.DescribeIpGroups");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<DescribeIpGroupsResult>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(DescribeIpGroupsError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        })
    }

    /// <p>Describes the specified tags for the specified WorkSpace.</p>
    fn describe_tags(
        &self,
        input: DescribeTagsRequest,
    ) -> RusotoFuture<DescribeTagsResult, DescribeTagsError> {
        let mut request = SignedRequest::new("POST", "workspaces", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "WorkspacesService.DescribeTags");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<DescribeTagsResult>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(DescribeTagsError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        })
    }

    /// <p>Describes the available WorkSpace bundles.</p> <p>You can filter the results using either bundle ID or owner, but not both.</p>
    fn describe_workspace_bundles(
        &self,
        input: DescribeWorkspaceBundlesRequest,
    ) -> RusotoFuture<DescribeWorkspaceBundlesResult, DescribeWorkspaceBundlesError> {
        let mut request = SignedRequest::new("POST", "workspaces", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "WorkspacesService.DescribeWorkspaceBundles");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<DescribeWorkspaceBundlesResult>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(DescribeWorkspaceBundlesError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        })
    }

    /// <p>Describes the available AWS Directory Service directories that are registered with Amazon WorkSpaces.</p>
    fn describe_workspace_directories(
        &self,
        input: DescribeWorkspaceDirectoriesRequest,
    ) -> RusotoFuture<DescribeWorkspaceDirectoriesResult, DescribeWorkspaceDirectoriesError> {
        let mut request = SignedRequest::new("POST", "workspaces", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "WorkspacesService.DescribeWorkspaceDirectories",
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

                    serde_json::from_str::<DescribeWorkspaceDirectoriesResult>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(DescribeWorkspaceDirectoriesError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        })
    }

    /// <p>Describes the specified WorkSpaces.</p> <p>You can filter the results using bundle ID, directory ID, or owner, but you can specify only one filter at a time.</p>
    fn describe_workspaces(
        &self,
        input: DescribeWorkspacesRequest,
    ) -> RusotoFuture<DescribeWorkspacesResult, DescribeWorkspacesError> {
        let mut request = SignedRequest::new("POST", "workspaces", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "WorkspacesService.DescribeWorkspaces");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<DescribeWorkspacesResult>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(DescribeWorkspacesError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        })
    }

    /// <p>Describes the connection status of the specified WorkSpaces.</p>
    fn describe_workspaces_connection_status(
        &self,
        input: DescribeWorkspacesConnectionStatusRequest,
    ) -> RusotoFuture<
        DescribeWorkspacesConnectionStatusResult,
        DescribeWorkspacesConnectionStatusError,
    > {
        let mut request = SignedRequest::new("POST", "workspaces", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "WorkspacesService.DescribeWorkspacesConnectionStatus",
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

                    serde_json::from_str::<DescribeWorkspacesConnectionStatusResult>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(DescribeWorkspacesConnectionStatusError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        })
    }

    /// <p>Disassociates the specified IP access control group from the specified directory.</p>
    fn disassociate_ip_groups(
        &self,
        input: DisassociateIpGroupsRequest,
    ) -> RusotoFuture<DisassociateIpGroupsResult, DisassociateIpGroupsError> {
        let mut request = SignedRequest::new("POST", "workspaces", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "WorkspacesService.DisassociateIpGroups");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<DisassociateIpGroupsResult>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(DisassociateIpGroupsError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        })
    }

    /// <p>Modifies the specified WorkSpace properties.</p>
    fn modify_workspace_properties(
        &self,
        input: ModifyWorkspacePropertiesRequest,
    ) -> RusotoFuture<ModifyWorkspacePropertiesResult, ModifyWorkspacePropertiesError> {
        let mut request = SignedRequest::new("POST", "workspaces", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "WorkspacesService.ModifyWorkspaceProperties",
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

                    serde_json::from_str::<ModifyWorkspacePropertiesResult>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(ModifyWorkspacePropertiesError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        })
    }

    /// <p>Sets the state of the specified WorkSpace.</p> <p>To maintain a WorkSpace without being interrupted, set the WorkSpace state to <code>ADMIN_MAINTENANCE</code>. WorkSpaces in this state do not respond to requests to reboot, stop, start, or rebuild. An AutoStop WorkSpace in this state is not stopped. Users can log into a WorkSpace in the <code>ADMIN_MAINTENANCE</code> state.</p>
    fn modify_workspace_state(
        &self,
        input: ModifyWorkspaceStateRequest,
    ) -> RusotoFuture<ModifyWorkspaceStateResult, ModifyWorkspaceStateError> {
        let mut request = SignedRequest::new("POST", "workspaces", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "WorkspacesService.ModifyWorkspaceState");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<ModifyWorkspaceStateResult>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(ModifyWorkspaceStateError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        })
    }

    /// <p>Reboots the specified WorkSpaces.</p> <p>You cannot reboot a WorkSpace unless its state is <code>AVAILABLE</code> or <code>UNHEALTHY</code>.</p> <p>This operation is asynchronous and returns before the WorkSpaces have rebooted.</p>
    fn reboot_workspaces(
        &self,
        input: RebootWorkspacesRequest,
    ) -> RusotoFuture<RebootWorkspacesResult, RebootWorkspacesError> {
        let mut request = SignedRequest::new("POST", "workspaces", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "WorkspacesService.RebootWorkspaces");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<RebootWorkspacesResult>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(RebootWorkspacesError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        })
    }

    /// <p>Rebuilds the specified WorkSpace.</p> <p>You cannot rebuild a WorkSpace unless its state is <code>AVAILABLE</code>, <code>ERROR</code>, or <code>UNHEALTHY</code>.</p> <p>Rebuilding a WorkSpace is a potentially destructive action that can result in the loss of data. For more information, see <a href="http://docs.aws.amazon.com/workspaces/latest/adminguide/reset-workspace.html">Rebuild a WorkSpace</a>.</p> <p>This operation is asynchronous and returns before the WorkSpaces have been completely rebuilt.</p>
    fn rebuild_workspaces(
        &self,
        input: RebuildWorkspacesRequest,
    ) -> RusotoFuture<RebuildWorkspacesResult, RebuildWorkspacesError> {
        let mut request = SignedRequest::new("POST", "workspaces", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "WorkspacesService.RebuildWorkspaces");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<RebuildWorkspacesResult>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(RebuildWorkspacesError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        })
    }

    /// <p>Removes one or more rules from the specified IP access control group.</p>
    fn revoke_ip_rules(
        &self,
        input: RevokeIpRulesRequest,
    ) -> RusotoFuture<RevokeIpRulesResult, RevokeIpRulesError> {
        let mut request = SignedRequest::new("POST", "workspaces", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "WorkspacesService.RevokeIpRules");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<RevokeIpRulesResult>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(RevokeIpRulesError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        })
    }

    /// <p>Starts the specified WorkSpaces.</p> <p>You cannot start a WorkSpace unless it has a running mode of <code>AutoStop</code> and a state of <code>STOPPED</code>.</p>
    fn start_workspaces(
        &self,
        input: StartWorkspacesRequest,
    ) -> RusotoFuture<StartWorkspacesResult, StartWorkspacesError> {
        let mut request = SignedRequest::new("POST", "workspaces", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "WorkspacesService.StartWorkspaces");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<StartWorkspacesResult>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(StartWorkspacesError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        })
    }

    /// <p> Stops the specified WorkSpaces.</p> <p>You cannot stop a WorkSpace unless it has a running mode of <code>AutoStop</code> and a state of <code>AVAILABLE</code>, <code>IMPAIRED</code>, <code>UNHEALTHY</code>, or <code>ERROR</code>.</p>
    fn stop_workspaces(
        &self,
        input: StopWorkspacesRequest,
    ) -> RusotoFuture<StopWorkspacesResult, StopWorkspacesError> {
        let mut request = SignedRequest::new("POST", "workspaces", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "WorkspacesService.StopWorkspaces");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<StopWorkspacesResult>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(StopWorkspacesError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        })
    }

    /// <p>Terminates the specified WorkSpaces.</p> <p>Terminating a WorkSpace is a permanent action and cannot be undone. The user's data is destroyed. If you need to archive any user data, contact Amazon Web Services before terminating the WorkSpace.</p> <p>You can terminate a WorkSpace that is in any state except <code>SUSPENDED</code>.</p> <p>This operation is asynchronous and returns before the WorkSpaces have been completely terminated.</p>
    fn terminate_workspaces(
        &self,
        input: TerminateWorkspacesRequest,
    ) -> RusotoFuture<TerminateWorkspacesResult, TerminateWorkspacesError> {
        let mut request = SignedRequest::new("POST", "workspaces", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "WorkspacesService.TerminateWorkspaces");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<TerminateWorkspacesResult>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(TerminateWorkspacesError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        })
    }

    /// <p>Replaces the current rules of the specified IP access control group with the specified rules.</p>
    fn update_rules_of_ip_group(
        &self,
        input: UpdateRulesOfIpGroupRequest,
    ) -> RusotoFuture<UpdateRulesOfIpGroupResult, UpdateRulesOfIpGroupError> {
        let mut request = SignedRequest::new("POST", "workspaces", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "WorkspacesService.UpdateRulesOfIpGroup");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<UpdateRulesOfIpGroupResult>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(UpdateRulesOfIpGroupError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        })
    }
}

#[cfg(test)]
mod protocol_tests {}
