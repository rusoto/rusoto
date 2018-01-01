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

#[allow(warnings)]
use hyper::Client;
use hyper::status::StatusCode;
use rusoto_core::request::DispatchSignedRequest;
use rusoto_core::region;

use std::fmt;
use std::error::Error;
use std::io;
use std::io::Read;
use rusoto_core::request::HttpDispatchError;
use rusoto_core::credential::{CredentialsError, ProvideAwsCredentials};

use serde_json;
use rusoto_core::signature::SignedRequest;
use serde_json::Value as SerdeJsonValue;
use serde_json::from_str;
/// <p>Contains information about the compute type of a WorkSpace bundle.</p>
#[derive(Default, Debug, Clone, Deserialize)]
pub struct ComputeType {
    /// <p>The name of the compute type for the bundle.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

/// <p>The request of the <a>CreateTags</a> operation.</p>
#[derive(Default, Debug, Clone, Serialize)]
pub struct CreateTagsRequest {
    /// <p>The resource ID of the request.</p>
    #[serde(rename = "ResourceId")]
    pub resource_id: String,
    /// <p>The tags of the request.</p>
    #[serde(rename = "Tags")]
    pub tags: Vec<Tag>,
}

/// <p>The result of the <a>CreateTags</a> operation.</p>
#[derive(Default, Debug, Clone, Deserialize)]
pub struct CreateTagsResult;

/// <p>Contains the inputs for the <a>CreateWorkspaces</a> operation.</p>
#[derive(Default, Debug, Clone, Serialize)]
pub struct CreateWorkspacesRequest {
    /// <p>An array of structures that specify the WorkSpaces to create.</p>
    #[serde(rename = "Workspaces")]
    pub workspaces: Vec<WorkspaceRequest>,
}

/// <p>Contains the result of the <a>CreateWorkspaces</a> operation.</p>
#[derive(Default, Debug, Clone, Deserialize)]
pub struct CreateWorkspacesResult {
    /// <p>An array of structures that represent the WorkSpaces that could not be created.</p>
    #[serde(rename = "FailedRequests")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failed_requests: Option<Vec<FailedCreateWorkspaceRequest>>,
    /// <p>An array of structures that represent the WorkSpaces that were created.</p> <p>Because this operation is asynchronous, the identifier in <code>WorkspaceId</code> is not immediately available. If you immediately call <a>DescribeWorkspaces</a> with this identifier, no information will be returned.</p>
    #[serde(rename = "PendingRequests")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pending_requests: Option<Vec<Workspace>>,
}

/// <p>Contains default WorkSpace creation information.</p>
#[derive(Default, Debug, Clone, Deserialize)]
pub struct DefaultWorkspaceCreationProperties {
    /// <p>The identifier of any custom security groups that are applied to the WorkSpaces when they are created.</p>
    #[serde(rename = "CustomSecurityGroupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_security_group_id: Option<String>,
    /// <p>The organizational unit (OU) in the directory that the WorkSpace machine accounts are placed in.</p>
    #[serde(rename = "DefaultOu")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_ou: Option<String>,
    /// <p>A public IP address will be attached to all WorkSpaces that are created or rebuilt.</p>
    #[serde(rename = "EnableInternetAccess")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_internet_access: Option<bool>,
    /// <p>Specifies if the directory is enabled for Amazon WorkDocs.</p>
    #[serde(rename = "EnableWorkDocs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_work_docs: Option<bool>,
    /// <p>The WorkSpace user is an administrator on the WorkSpace.</p>
    #[serde(rename = "UserEnabledAsLocalAdministrator")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_enabled_as_local_administrator: Option<bool>,
}

/// <p>The request of the <a>DeleteTags</a> operation.</p>
#[derive(Default, Debug, Clone, Serialize)]
pub struct DeleteTagsRequest {
    /// <p>The resource ID of the request.</p>
    #[serde(rename = "ResourceId")]
    pub resource_id: String,
    /// <p>The tag keys of the request.</p>
    #[serde(rename = "TagKeys")]
    pub tag_keys: Vec<String>,
}

/// <p>The result of the <a>DeleteTags</a> operation.</p>
#[derive(Default, Debug, Clone, Deserialize)]
pub struct DeleteTagsResult;

/// <p>The request of the <a>DescribeTags</a> operation.</p>
#[derive(Default, Debug, Clone, Serialize)]
pub struct DescribeTagsRequest {
    /// <p>The resource ID of the request.</p>
    #[serde(rename = "ResourceId")]
    pub resource_id: String,
}

/// <p>The result of the <a>DescribeTags</a> operation.</p>
#[derive(Default, Debug, Clone, Deserialize)]
pub struct DescribeTagsResult {
    /// <p>The list of tags.</p>
    #[serde(rename = "TagList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_list: Option<Vec<Tag>>,
}

/// <p>Contains the inputs for the <a>DescribeWorkspaceBundles</a> operation.</p>
#[derive(Default, Debug, Clone, Serialize)]
pub struct DescribeWorkspaceBundlesRequest {
    /// <p>An array of strings that contains the identifiers of the bundles to retrieve. This parameter cannot be combined with any other filter parameter.</p>
    #[serde(rename = "BundleIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bundle_ids: Option<Vec<String>>,
    /// <p>The <code>NextToken</code> value from a previous call to this operation. Pass null if this is the first call.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The owner of the bundles to retrieve. This parameter cannot be combined with any other filter parameter.</p> <p>This contains one of the following values:</p> <ul> <li> <p>null- Retrieves the bundles that belong to the account making the call.</p> </li> <li> <p> <code>AMAZON</code>- Retrieves the bundles that are provided by AWS.</p> </li> </ul>
    #[serde(rename = "Owner")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner: Option<String>,
}

/// <p>Contains the results of the <a>DescribeWorkspaceBundles</a> operation.</p>
#[derive(Default, Debug, Clone, Deserialize)]
pub struct DescribeWorkspaceBundlesResult {
    /// <p>An array of structures that contain information about the bundles.</p>
    #[serde(rename = "Bundles")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bundles: Option<Vec<WorkspaceBundle>>,
    /// <p>If not null, more results are available. Pass this value for the <code>NextToken</code> parameter in a subsequent call to this operation to retrieve the next set of items. This token is valid for one day and must be used within that time frame.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

/// <p>Contains the inputs for the <a>DescribeWorkspaceDirectories</a> operation.</p>
#[derive(Default, Debug, Clone, Serialize)]
pub struct DescribeWorkspaceDirectoriesRequest {
    /// <p>An array of strings that contains the directory identifiers to retrieve information for. If this member is null, all directories are retrieved.</p>
    #[serde(rename = "DirectoryIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub directory_ids: Option<Vec<String>>,
    /// <p>The <code>NextToken</code> value from a previous call to this operation. Pass null if this is the first call.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

/// <p>Contains the results of the <a>DescribeWorkspaceDirectories</a> operation.</p>
#[derive(Default, Debug, Clone, Deserialize)]
pub struct DescribeWorkspaceDirectoriesResult {
    /// <p>An array of structures that contain information about the directories.</p>
    #[serde(rename = "Directories")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub directories: Option<Vec<WorkspaceDirectory>>,
    /// <p>If not null, more results are available. Pass this value for the <code>NextToken</code> parameter in a subsequent call to this operation to retrieve the next set of items. This token is valid for one day and must be used within that time frame.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, Serialize)]
pub struct DescribeWorkspacesConnectionStatusRequest {
    /// <p>The next token of the request.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>An array of strings that contain the identifiers of the WorkSpaces.</p>
    #[serde(rename = "WorkspaceIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workspace_ids: Option<Vec<String>>,
}

#[derive(Default, Debug, Clone, Deserialize)]
pub struct DescribeWorkspacesConnectionStatusResult {
    /// <p>The next token of the result.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The connection status of the WorkSpace.</p>
    #[serde(rename = "WorkspacesConnectionStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workspaces_connection_status: Option<Vec<WorkspaceConnectionStatus>>,
}

/// <p>Contains the inputs for the <a>DescribeWorkspaces</a> operation.</p>
#[derive(Default, Debug, Clone, Serialize)]
pub struct DescribeWorkspacesRequest {
    /// <p>The identifier of a bundle to obtain the WorkSpaces for. All WorkSpaces that are created from this bundle will be retrieved. This parameter cannot be combined with any other filter parameter.</p>
    #[serde(rename = "BundleId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bundle_id: Option<String>,
    /// <p>Specifies the directory identifier to which to limit the WorkSpaces. Optionally, you can specify a specific directory user with the <code>UserName</code> parameter. This parameter cannot be combined with any other filter parameter.</p>
    #[serde(rename = "DirectoryId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub directory_id: Option<String>,
    /// <p>The maximum number of items to return.</p>
    #[serde(rename = "Limit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// <p>The <code>NextToken</code> value from a previous call to this operation. Pass null if this is the first call.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>Used with the <code>DirectoryId</code> parameter to specify the directory user for whom to obtain the WorkSpace.</p>
    #[serde(rename = "UserName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_name: Option<String>,
    /// <p>An array of strings that contain the identifiers of the WorkSpaces for which to retrieve information. This parameter cannot be combined with any other filter parameter.</p> <p>Because the <a>CreateWorkspaces</a> operation is asynchronous, the identifier it returns is not immediately available. If you immediately call <a>DescribeWorkspaces</a> with this identifier, no information is returned.</p>
    #[serde(rename = "WorkspaceIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workspace_ids: Option<Vec<String>>,
}

/// <p>Contains the results for the <a>DescribeWorkspaces</a> operation.</p>
#[derive(Default, Debug, Clone, Deserialize)]
pub struct DescribeWorkspacesResult {
    /// <p>If not null, more results are available. Pass this value for the <code>NextToken</code> parameter in a subsequent call to this operation to retrieve the next set of items. This token is valid for one day and must be used within that time frame.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>An array of structures that contain the information about the WorkSpaces.</p> <p>Because the <a>CreateWorkspaces</a> operation is asynchronous, some of this information may be incomplete for a newly-created WorkSpace.</p>
    #[serde(rename = "Workspaces")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workspaces: Option<Vec<Workspace>>,
}

/// <p>Contains information about a WorkSpace that could not be created.</p>
#[derive(Default, Debug, Clone, Deserialize)]
pub struct FailedCreateWorkspaceRequest {
    /// <p>The error code.</p>
    #[serde(rename = "ErrorCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_code: Option<String>,
    /// <p>The textual error message.</p>
    #[serde(rename = "ErrorMessage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
    /// <p>A <a>FailedCreateWorkspaceRequest$WorkspaceRequest</a> object that contains the information about the WorkSpace that could not be created.</p>
    #[serde(rename = "WorkspaceRequest")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workspace_request: Option<WorkspaceRequest>,
}

/// <p>Contains information about a WorkSpace that could not be rebooted (<a>RebootWorkspaces</a>), rebuilt (<a>RebuildWorkspaces</a>), terminated (<a>TerminateWorkspaces</a>), started (<a>StartWorkspaces</a>), or stopped (<a>StopWorkspaces</a>).</p>
#[derive(Default, Debug, Clone, Deserialize)]
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

#[derive(Default, Debug, Clone, Serialize)]
pub struct ModifyWorkspacePropertiesRequest {
    /// <p>The ID of the WorkSpace.</p>
    #[serde(rename = "WorkspaceId")]
    pub workspace_id: String,
    /// <p>The WorkSpace properties of the request.</p>
    #[serde(rename = "WorkspaceProperties")]
    pub workspace_properties: WorkspaceProperties,
}

#[derive(Default, Debug, Clone, Deserialize)]
pub struct ModifyWorkspacePropertiesResult;

/// <p>Contains information used with the <a>RebootWorkspaces</a> operation to reboot a WorkSpace.</p>
#[derive(Default, Debug, Clone, Serialize)]
pub struct RebootRequest {
    /// <p>The identifier of the WorkSpace to reboot.</p>
    #[serde(rename = "WorkspaceId")]
    pub workspace_id: String,
}

/// <p>Contains the inputs for the <a>RebootWorkspaces</a> operation.</p>
#[derive(Default, Debug, Clone, Serialize)]
pub struct RebootWorkspacesRequest {
    /// <p>An array of structures that specify the WorkSpaces to reboot.</p>
    #[serde(rename = "RebootWorkspaceRequests")]
    pub reboot_workspace_requests: Vec<RebootRequest>,
}

/// <p>Contains the results of the <a>RebootWorkspaces</a> operation.</p>
#[derive(Default, Debug, Clone, Deserialize)]
pub struct RebootWorkspacesResult {
    /// <p>An array of structures representing any WorkSpaces that could not be rebooted.</p>
    #[serde(rename = "FailedRequests")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failed_requests: Option<Vec<FailedWorkspaceChangeRequest>>,
}

/// <p>Contains information used with the <a>RebuildWorkspaces</a> operation to rebuild a WorkSpace.</p>
#[derive(Default, Debug, Clone, Serialize)]
pub struct RebuildRequest {
    /// <p>The identifier of the WorkSpace to rebuild.</p>
    #[serde(rename = "WorkspaceId")]
    pub workspace_id: String,
}

/// <p>Contains the inputs for the <a>RebuildWorkspaces</a> operation.</p>
#[derive(Default, Debug, Clone, Serialize)]
pub struct RebuildWorkspacesRequest {
    /// <p>An array of structures that specify the WorkSpaces to rebuild.</p>
    #[serde(rename = "RebuildWorkspaceRequests")]
    pub rebuild_workspace_requests: Vec<RebuildRequest>,
}

/// <p>Contains the results of the <a>RebuildWorkspaces</a> operation.</p>
#[derive(Default, Debug, Clone, Deserialize)]
pub struct RebuildWorkspacesResult {
    /// <p>An array of structures representing any WorkSpaces that could not be rebuilt.</p>
    #[serde(rename = "FailedRequests")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failed_requests: Option<Vec<FailedWorkspaceChangeRequest>>,
}

/// <p>Describes the start request.</p>
#[derive(Default, Debug, Clone, Serialize)]
pub struct StartRequest {
    /// <p>The ID of the WorkSpace.</p>
    #[serde(rename = "WorkspaceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workspace_id: Option<String>,
}

#[derive(Default, Debug, Clone, Serialize)]
pub struct StartWorkspacesRequest {
    /// <p>The requests.</p>
    #[serde(rename = "StartWorkspaceRequests")]
    pub start_workspace_requests: Vec<StartRequest>,
}

#[derive(Default, Debug, Clone, Deserialize)]
pub struct StartWorkspacesResult {
    /// <p>The failed requests.</p>
    #[serde(rename = "FailedRequests")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failed_requests: Option<Vec<FailedWorkspaceChangeRequest>>,
}

/// <p>Describes the stop request.</p>
#[derive(Default, Debug, Clone, Serialize)]
pub struct StopRequest {
    /// <p>The ID of the WorkSpace.</p>
    #[serde(rename = "WorkspaceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workspace_id: Option<String>,
}

#[derive(Default, Debug, Clone, Serialize)]
pub struct StopWorkspacesRequest {
    /// <p>The requests.</p>
    #[serde(rename = "StopWorkspaceRequests")]
    pub stop_workspace_requests: Vec<StopRequest>,
}

#[derive(Default, Debug, Clone, Deserialize)]
pub struct StopWorkspacesResult {
    /// <p>The failed requests.</p>
    #[serde(rename = "FailedRequests")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failed_requests: Option<Vec<FailedWorkspaceChangeRequest>>,
}

/// <p>Describes the tag of the WorkSpace.</p>
#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct Tag {
    /// <p>The key of the tag.</p>
    #[serde(rename = "Key")]
    pub key: String,
    /// <p>The value of the tag.</p>
    #[serde(rename = "Value")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

/// <p>Contains information used with the <a>TerminateWorkspaces</a> operation to terminate a WorkSpace.</p>
#[derive(Default, Debug, Clone, Serialize)]
pub struct TerminateRequest {
    /// <p>The identifier of the WorkSpace to terminate.</p>
    #[serde(rename = "WorkspaceId")]
    pub workspace_id: String,
}

/// <p>Contains the inputs for the <a>TerminateWorkspaces</a> operation.</p>
#[derive(Default, Debug, Clone, Serialize)]
pub struct TerminateWorkspacesRequest {
    /// <p>An array of structures that specify the WorkSpaces to terminate.</p>
    #[serde(rename = "TerminateWorkspaceRequests")]
    pub terminate_workspace_requests: Vec<TerminateRequest>,
}

/// <p>Contains the results of the <a>TerminateWorkspaces</a> operation.</p>
#[derive(Default, Debug, Clone, Deserialize)]
pub struct TerminateWorkspacesResult {
    /// <p>An array of structures representing any WorkSpaces that could not be terminated.</p>
    #[serde(rename = "FailedRequests")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failed_requests: Option<Vec<FailedWorkspaceChangeRequest>>,
}

/// <p>Contains information about the user storage for a WorkSpace bundle.</p>
#[derive(Default, Debug, Clone, Deserialize)]
pub struct UserStorage {
    /// <p>The amount of user storage for the bundle.</p>
    #[serde(rename = "Capacity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capacity: Option<String>,
}

/// <p>Contains information about a WorkSpace.</p>
#[derive(Default, Debug, Clone, Deserialize)]
pub struct Workspace {
    /// <p>The identifier of the bundle that the WorkSpace was created from.</p>
    #[serde(rename = "BundleId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bundle_id: Option<String>,
    /// <p>The name of the WorkSpace as seen by the operating system.</p>
    #[serde(rename = "ComputerName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub computer_name: Option<String>,
    /// <p>The identifier of the AWS Directory Service directory that the WorkSpace belongs to.</p>
    #[serde(rename = "DirectoryId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub directory_id: Option<String>,
    /// <p>If the WorkSpace could not be created, this contains the error code.</p>
    #[serde(rename = "ErrorCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_code: Option<String>,
    /// <p>If the WorkSpace could not be created, this contains a textual error message that describes the failure.</p>
    #[serde(rename = "ErrorMessage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
    /// <p>The IP address of the WorkSpace.</p>
    #[serde(rename = "IpAddress")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_address: Option<String>,
    /// <p>Specifies whether the data stored on the root volume, or C: drive, is encrypted.</p>
    #[serde(rename = "RootVolumeEncryptionEnabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub root_volume_encryption_enabled: Option<bool>,
    /// <p>The operational state of the WorkSpace.</p>
    #[serde(rename = "State")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    /// <p>The identifier of the subnet that the WorkSpace is in.</p>
    #[serde(rename = "SubnetId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subnet_id: Option<String>,
    /// <p>The user that the WorkSpace is assigned to.</p>
    #[serde(rename = "UserName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_name: Option<String>,
    /// <p>Specifies whether the data stored on the user volume, or D: drive, is encrypted.</p>
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
    #[serde(rename = "WorkspaceProperties")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workspace_properties: Option<WorkspaceProperties>,
}

/// <p>Contains information about a WorkSpace bundle.</p>
#[derive(Default, Debug, Clone, Deserialize)]
pub struct WorkspaceBundle {
    /// <p>The bundle identifier.</p>
    #[serde(rename = "BundleId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bundle_id: Option<String>,
    /// <p>A <a>ComputeType</a> object that specifies the compute type for the bundle.</p>
    #[serde(rename = "ComputeType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compute_type: Option<ComputeType>,
    /// <p>The bundle description.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The name of the bundle.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The owner of the bundle. This contains the owner's account identifier, or <code>AMAZON</code> if the bundle is provided by AWS.</p>
    #[serde(rename = "Owner")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner: Option<String>,
    /// <p>A <a>UserStorage</a> object that specifies the amount of user storage that the bundle contains.</p>
    #[serde(rename = "UserStorage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_storage: Option<UserStorage>,
}

/// <p>Describes the connection status of a WorkSpace.</p>
#[derive(Default, Debug, Clone, Deserialize)]
pub struct WorkspaceConnectionStatus {
    /// <p>The connection state of the WorkSpace. Returns UNKOWN if the WorkSpace is in a Stopped state.</p>
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

/// <p>Contains information about an AWS Directory Service directory for use with Amazon WorkSpaces.</p>
#[derive(Default, Debug, Clone, Deserialize)]
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
    /// <p>An array of strings that contains the IP addresses of the DNS servers for the directory.</p>
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
    /// <p>An array of strings that contains the identifiers of the subnets used with the directory.</p>
    #[serde(rename = "SubnetIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subnet_ids: Option<Vec<String>>,
    /// <p>A structure that specifies the default creation properties for all WorkSpaces in the directory.</p>
    #[serde(rename = "WorkspaceCreationProperties")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workspace_creation_properties: Option<DefaultWorkspaceCreationProperties>,
    /// <p>The identifier of the security group that is assigned to new WorkSpaces.</p>
    #[serde(rename = "WorkspaceSecurityGroupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workspace_security_group_id: Option<String>,
}

/// <p>Describes the properties of a WorkSpace.</p>
#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct WorkspaceProperties {
    /// <p>The running mode of the WorkSpace. AlwaysOn WorkSpaces are billed monthly. AutoStop WorkSpaces are billed by the hour and stopped when no longer being used in order to save on costs.</p>
    #[serde(rename = "RunningMode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub running_mode: Option<String>,
    /// <p>The time after a user logs off when WorkSpaces are automatically stopped. Configured in 60 minute intervals.</p>
    #[serde(rename = "RunningModeAutoStopTimeoutInMinutes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub running_mode_auto_stop_timeout_in_minutes: Option<i64>,
}

/// <p>Contains information about a WorkSpace creation request.</p>
#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct WorkspaceRequest {
    /// <p>The identifier of the bundle to create the WorkSpace from. You can use the <a>DescribeWorkspaceBundles</a> operation to obtain a list of the bundles that are available.</p>
    #[serde(rename = "BundleId")]
    pub bundle_id: String,
    /// <p>The identifier of the AWS Directory Service directory to create the WorkSpace in. You can use the <a>DescribeWorkspaceDirectories</a> operation to obtain a list of the directories that are available.</p>
    #[serde(rename = "DirectoryId")]
    pub directory_id: String,
    /// <p>Specifies whether the data stored on the root volume, or C: drive, is encrypted.</p>
    #[serde(rename = "RootVolumeEncryptionEnabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub root_volume_encryption_enabled: Option<bool>,
    /// <p>The tags of the WorkSpace request.</p>
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
    /// <p>The username that the WorkSpace is assigned to. This username must exist in the AWS Directory Service directory specified by the <code>DirectoryId</code> member.</p>
    #[serde(rename = "UserName")]
    pub user_name: String,
    /// <p>Specifies whether the data stored on the user volume, or D: drive, is encrypted.</p>
    #[serde(rename = "UserVolumeEncryptionEnabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_volume_encryption_enabled: Option<bool>,
    /// <p>The KMS key used to encrypt data stored on your WorkSpace.</p>
    #[serde(rename = "VolumeEncryptionKey")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volume_encryption_key: Option<String>,
    #[serde(rename = "WorkspaceProperties")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workspace_properties: Option<WorkspaceProperties>,
}

/// Errors returned by CreateTags
#[derive(Debug, PartialEq)]
pub enum CreateTagsError {
    ///<p>One or more parameter values are not valid.</p>
    InvalidParameterValues(String),
    ///<p>Your resource limits have been exceeded.</p>
    ResourceLimitExceeded(String),
    ///<p>The resource could not be found.</p>
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
                let raw_error_type = json.get("__type")
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
    ///<p>One or more parameter values are not valid.</p>
    InvalidParameterValues(String),
    ///<p>Your resource limits have been exceeded.</p>
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
                let raw_error_type = json.get("__type")
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
/// Errors returned by DeleteTags
#[derive(Debug, PartialEq)]
pub enum DeleteTagsError {
    ///<p>One or more parameter values are not valid.</p>
    InvalidParameterValues(String),
    ///<p>The resource could not be found.</p>
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
                let raw_error_type = json.get("__type")
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
/// Errors returned by DescribeTags
#[derive(Debug, PartialEq)]
pub enum DescribeTagsError {
    ///<p>The resource could not be found.</p>
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
                let raw_error_type = json.get("__type")
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
    ///<p>One or more parameter values are not valid.</p>
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
                let raw_error_type = json.get("__type")
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
    ///<p>One or more parameter values are not valid.</p>
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
                let raw_error_type = json.get("__type")
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
    ///<p>One or more parameter values are not valid.</p>
    InvalidParameterValues(String),
    ///<p>The specified resource is not available.</p>
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
                let raw_error_type = json.get("__type")
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
    ///<p>One or more parameter values are not valid.</p>
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
                let raw_error_type = json.get("__type")
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
/// Errors returned by ModifyWorkspaceProperties
#[derive(Debug, PartialEq)]
pub enum ModifyWorkspacePropertiesError {
    ///<p>The user is not authorized to access a resource.</p>
    AccessDenied(String),
    ///<p>One or more parameter values are not valid.</p>
    InvalidParameterValues(String),
    ///<p>The state of the WorkSpace is not valid for this operation.</p>
    InvalidResourceState(String),
    ///<p>The properties of this WorkSpace are currently being modified. Try again in a moment.</p>
    OperationInProgress(String),
    ///<p>The resource could not be found.</p>
    ResourceNotFound(String),
    ///<p>The specified resource is not available.</p>
    ResourceUnavailable(String),
    ///<p>The configuration of this WorkSpace is not supported for this operation. For more information, see the <a href="http://docs.aws.amazon.com/workspaces/latest/adminguide/">Amazon WorkSpaces Administration Guide</a>. </p>
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
                let raw_error_type = json.get("__type")
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
                let raw_error_type = json.get("__type")
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
                let raw_error_type = json.get("__type")
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
                let raw_error_type = json.get("__type")
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
                let raw_error_type = json.get("__type")
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
                let raw_error_type = json.get("__type")
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
/// Trait representing the capabilities of the Amazon WorkSpaces API. Amazon WorkSpaces clients implement this trait.
pub trait Workspaces {
    #[doc = "<p>Creates tags for a WorkSpace.</p>"]
    fn create_tags(&self, input: &CreateTagsRequest) -> Result<CreateTagsResult, CreateTagsError>;

    #[doc="<p>Creates one or more WorkSpaces.</p> <note> <p>This operation is asynchronous and returns before the WorkSpaces are created.</p> </note>"]
    fn create_workspaces(
        &self,
        input: &CreateWorkspacesRequest,
    ) -> Result<CreateWorkspacesResult, CreateWorkspacesError>;

    #[doc = "<p>Deletes tags from a WorkSpace.</p>"]
    fn delete_tags(&self, input: &DeleteTagsRequest) -> Result<DeleteTagsResult, DeleteTagsError>;

    #[doc = "<p>Describes tags for a WorkSpace.</p>"]
    fn describe_tags(
        &self,
        input: &DescribeTagsRequest,
    ) -> Result<DescribeTagsResult, DescribeTagsError>;

    #[doc="<p>Obtains information about the WorkSpace bundles that are available to your account in the specified region.</p> <p>You can filter the results with either the <code>BundleIds</code> parameter, or the <code>Owner</code> parameter, but not both.</p> <p>This operation supports pagination with the use of the <code>NextToken</code> request and response parameters. If more results are available, the <code>NextToken</code> response member contains a token that you pass in the next call to this operation to retrieve the next set of items.</p>"]
    fn describe_workspace_bundles(
        &self,
        input: &DescribeWorkspaceBundlesRequest,
    ) -> Result<DescribeWorkspaceBundlesResult, DescribeWorkspaceBundlesError>;

    #[doc="<p>Retrieves information about the AWS Directory Service directories in the region that are registered with Amazon WorkSpaces and are available to your account.</p> <p>This operation supports pagination with the use of the <code>NextToken</code> request and response parameters. If more results are available, the <code>NextToken</code> response member contains a token that you pass in the next call to this operation to retrieve the next set of items.</p>"]
    fn describe_workspace_directories(
        &self,
        input: &DescribeWorkspaceDirectoriesRequest,
    ) -> Result<DescribeWorkspaceDirectoriesResult, DescribeWorkspaceDirectoriesError>;

    #[doc="<p>Obtains information about the specified WorkSpaces.</p> <p>Only one of the filter parameters, such as <code>BundleId</code>, <code>DirectoryId</code>, or <code>WorkspaceIds</code>, can be specified at a time.</p> <p>This operation supports pagination with the use of the <code>NextToken</code> request and response parameters. If more results are available, the <code>NextToken</code> response member contains a token that you pass in the next call to this operation to retrieve the next set of items.</p>"]
    fn describe_workspaces(
        &self,
        input: &DescribeWorkspacesRequest,
    ) -> Result<DescribeWorkspacesResult, DescribeWorkspacesError>;

    #[doc = "<p>Describes the connection status of a specified WorkSpace.</p>"]
    fn describe_workspaces_connection_status(
        &self,
        input: &DescribeWorkspacesConnectionStatusRequest,
    ) -> Result<DescribeWorkspacesConnectionStatusResult, DescribeWorkspacesConnectionStatusError>;

    #[doc="<p>Modifies the WorkSpace properties, including the running mode and AutoStop time.</p>"]
    fn modify_workspace_properties(
        &self,
        input: &ModifyWorkspacePropertiesRequest,
    ) -> Result<ModifyWorkspacePropertiesResult, ModifyWorkspacePropertiesError>;

    #[doc="<p>Reboots the specified WorkSpaces.</p> <p>To be able to reboot a WorkSpace, the WorkSpace must have a <b>State</b> of <code>AVAILABLE</code>, <code>IMPAIRED</code>, or <code>INOPERABLE</code>.</p> <note> <p>This operation is asynchronous and returns before the WorkSpaces have rebooted.</p> </note>"]
    fn reboot_workspaces(
        &self,
        input: &RebootWorkspacesRequest,
    ) -> Result<RebootWorkspacesResult, RebootWorkspacesError>;

    #[doc="<p>Rebuilds the specified WorkSpaces.</p> <p>Rebuilding a WorkSpace is a potentially destructive action that can result in the loss of data. Rebuilding a WorkSpace causes the following to occur:</p> <ul> <li> <p>The system is restored to the image of the bundle that the WorkSpace is created from. Any applications that have been installed, or system settings that have been made since the WorkSpace was created will be lost.</p> </li> <li> <p>The data drive (D drive) is re-created from the last automatic snapshot taken of the data drive. The current contents of the data drive are overwritten. Automatic snapshots of the data drive are taken every 12 hours, so the snapshot can be as much as 12 hours old.</p> </li> </ul> <p>To be able to rebuild a WorkSpace, the WorkSpace must have a <b>State</b> of <code>AVAILABLE</code> or <code>ERROR</code>.</p> <note> <p>This operation is asynchronous and returns before the WorkSpaces have been completely rebuilt.</p> </note>"]
    fn rebuild_workspaces(
        &self,
        input: &RebuildWorkspacesRequest,
    ) -> Result<RebuildWorkspacesResult, RebuildWorkspacesError>;

    #[doc="<p>Starts the specified WorkSpaces. The WorkSpaces must have a running mode of AutoStop and a state of STOPPED.</p>"]
    fn start_workspaces(
        &self,
        input: &StartWorkspacesRequest,
    ) -> Result<StartWorkspacesResult, StartWorkspacesError>;

    #[doc="<p> Stops the specified WorkSpaces. The WorkSpaces must have a running mode of AutoStop and a state of AVAILABLE, IMPAIRED, UNHEALTHY, or ERROR.</p>"]
    fn stop_workspaces(
        &self,
        input: &StopWorkspacesRequest,
    ) -> Result<StopWorkspacesResult, StopWorkspacesError>;

    #[doc="<p>Terminates the specified WorkSpaces.</p> <p>Terminating a WorkSpace is a permanent action and cannot be undone. The user's data is not maintained and will be destroyed. If you need to archive any user data, contact Amazon Web Services before terminating the WorkSpace.</p> <p>You can terminate a WorkSpace that is in any state except <code>SUSPENDED</code>.</p> <note> <p>This operation is asynchronous and returns before the WorkSpaces have been completely terminated.</p> </note>"]
    fn terminate_workspaces(
        &self,
        input: &TerminateWorkspacesRequest,
    ) -> Result<TerminateWorkspacesResult, TerminateWorkspacesError>;
}
/// A client for the Amazon WorkSpaces API.
pub struct WorkspacesClient<P, D>
where
    P: ProvideAwsCredentials,
    D: DispatchSignedRequest,
{
    credentials_provider: P,
    region: region::Region,
    dispatcher: D,
}

impl<P, D> WorkspacesClient<P, D>
where
    P: ProvideAwsCredentials,
    D: DispatchSignedRequest,
{
    pub fn new(request_dispatcher: D, credentials_provider: P, region: region::Region) -> Self {
        WorkspacesClient {
            credentials_provider: credentials_provider,
            region: region,
            dispatcher: request_dispatcher,
        }
    }
}

impl<P, D> Workspaces for WorkspacesClient<P, D>
where
    P: ProvideAwsCredentials,
    D: DispatchSignedRequest,
{
    #[doc = "<p>Creates tags for a WorkSpace.</p>"]
    fn create_tags(&self, input: &CreateTagsRequest) -> Result<CreateTagsResult, CreateTagsError> {
        let mut request = SignedRequest::new("POST", "workspaces", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "WorkspacesService.CreateTags");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign_with_plus(&try!(self.credentials_provider.credentials()), true);

        let mut response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Ok(serde_json::from_str::<CreateTagsResult>(
                    String::from_utf8_lossy(&body).as_ref(),
                ).unwrap())
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(CreateTagsError::from_body(
                    String::from_utf8_lossy(&body).as_ref(),
                ))
            }
        }
    }

    #[doc="<p>Creates one or more WorkSpaces.</p> <note> <p>This operation is asynchronous and returns before the WorkSpaces are created.</p> </note>"]
    fn create_workspaces(
        &self,
        input: &CreateWorkspacesRequest,
    ) -> Result<CreateWorkspacesResult, CreateWorkspacesError> {
        let mut request = SignedRequest::new("POST", "workspaces", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "WorkspacesService.CreateWorkspaces");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign_with_plus(&try!(self.credentials_provider.credentials()), true);

        let mut response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Ok(serde_json::from_str::<CreateWorkspacesResult>(
                    String::from_utf8_lossy(&body).as_ref(),
                ).unwrap())
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(CreateWorkspacesError::from_body(
                    String::from_utf8_lossy(&body).as_ref(),
                ))
            }
        }
    }

    #[doc = "<p>Deletes tags from a WorkSpace.</p>"]
    fn delete_tags(&self, input: &DeleteTagsRequest) -> Result<DeleteTagsResult, DeleteTagsError> {
        let mut request = SignedRequest::new("POST", "workspaces", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "WorkspacesService.DeleteTags");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign_with_plus(&try!(self.credentials_provider.credentials()), true);

        let mut response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Ok(serde_json::from_str::<DeleteTagsResult>(
                    String::from_utf8_lossy(&body).as_ref(),
                ).unwrap())
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(DeleteTagsError::from_body(
                    String::from_utf8_lossy(&body).as_ref(),
                ))
            }
        }
    }

    #[doc = "<p>Describes tags for a WorkSpace.</p>"]
    fn describe_tags(
        &self,
        input: &DescribeTagsRequest,
    ) -> Result<DescribeTagsResult, DescribeTagsError> {
        let mut request = SignedRequest::new("POST", "workspaces", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "WorkspacesService.DescribeTags");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign_with_plus(&try!(self.credentials_provider.credentials()), true);

        let mut response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Ok(serde_json::from_str::<DescribeTagsResult>(
                    String::from_utf8_lossy(&body).as_ref(),
                ).unwrap())
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(DescribeTagsError::from_body(
                    String::from_utf8_lossy(&body).as_ref(),
                ))
            }
        }
    }

    #[doc="<p>Obtains information about the WorkSpace bundles that are available to your account in the specified region.</p> <p>You can filter the results with either the <code>BundleIds</code> parameter, or the <code>Owner</code> parameter, but not both.</p> <p>This operation supports pagination with the use of the <code>NextToken</code> request and response parameters. If more results are available, the <code>NextToken</code> response member contains a token that you pass in the next call to this operation to retrieve the next set of items.</p>"]
    fn describe_workspace_bundles(
        &self,
        input: &DescribeWorkspaceBundlesRequest,
    ) -> Result<DescribeWorkspaceBundlesResult, DescribeWorkspaceBundlesError> {
        let mut request = SignedRequest::new("POST", "workspaces", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "WorkspacesService.DescribeWorkspaceBundles");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign_with_plus(&try!(self.credentials_provider.credentials()), true);

        let mut response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Ok(serde_json::from_str::<DescribeWorkspaceBundlesResult>(
                    String::from_utf8_lossy(&body).as_ref(),
                ).unwrap())
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(DescribeWorkspaceBundlesError::from_body(
                    String::from_utf8_lossy(&body).as_ref(),
                ))
            }
        }
    }

    #[doc="<p>Retrieves information about the AWS Directory Service directories in the region that are registered with Amazon WorkSpaces and are available to your account.</p> <p>This operation supports pagination with the use of the <code>NextToken</code> request and response parameters. If more results are available, the <code>NextToken</code> response member contains a token that you pass in the next call to this operation to retrieve the next set of items.</p>"]
    fn describe_workspace_directories(
        &self,
        input: &DescribeWorkspaceDirectoriesRequest,
    ) -> Result<DescribeWorkspaceDirectoriesResult, DescribeWorkspaceDirectoriesError> {
        let mut request = SignedRequest::new("POST", "workspaces", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "WorkspacesService.DescribeWorkspaceDirectories",
        );
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign_with_plus(&try!(self.credentials_provider.credentials()), true);

        let mut response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Ok(serde_json::from_str::<DescribeWorkspaceDirectoriesResult>(
                    String::from_utf8_lossy(&body).as_ref(),
                ).unwrap())
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(DescribeWorkspaceDirectoriesError::from_body(
                    String::from_utf8_lossy(&body).as_ref(),
                ))
            }
        }
    }

    #[doc="<p>Obtains information about the specified WorkSpaces.</p> <p>Only one of the filter parameters, such as <code>BundleId</code>, <code>DirectoryId</code>, or <code>WorkspaceIds</code>, can be specified at a time.</p> <p>This operation supports pagination with the use of the <code>NextToken</code> request and response parameters. If more results are available, the <code>NextToken</code> response member contains a token that you pass in the next call to this operation to retrieve the next set of items.</p>"]
    fn describe_workspaces(
        &self,
        input: &DescribeWorkspacesRequest,
    ) -> Result<DescribeWorkspacesResult, DescribeWorkspacesError> {
        let mut request = SignedRequest::new("POST", "workspaces", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "WorkspacesService.DescribeWorkspaces");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign_with_plus(&try!(self.credentials_provider.credentials()), true);

        let mut response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Ok(serde_json::from_str::<DescribeWorkspacesResult>(
                    String::from_utf8_lossy(&body).as_ref(),
                ).unwrap())
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(DescribeWorkspacesError::from_body(
                    String::from_utf8_lossy(&body).as_ref(),
                ))
            }
        }
    }

    #[doc = "<p>Describes the connection status of a specified WorkSpace.</p>"]
    fn describe_workspaces_connection_status(
        &self,
        input: &DescribeWorkspacesConnectionStatusRequest,
    ) -> Result<DescribeWorkspacesConnectionStatusResult, DescribeWorkspacesConnectionStatusError>
    {
        let mut request = SignedRequest::new("POST", "workspaces", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "WorkspacesService.DescribeWorkspacesConnectionStatus",
        );
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign_with_plus(&try!(self.credentials_provider.credentials()), true);

        let mut response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Ok(
                    serde_json::from_str::<DescribeWorkspacesConnectionStatusResult>(
                        String::from_utf8_lossy(&body).as_ref(),
                    ).unwrap(),
                )
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(DescribeWorkspacesConnectionStatusError::from_body(
                    String::from_utf8_lossy(&body).as_ref(),
                ))
            }
        }
    }

    #[doc="<p>Modifies the WorkSpace properties, including the running mode and AutoStop time.</p>"]
    fn modify_workspace_properties(
        &self,
        input: &ModifyWorkspacePropertiesRequest,
    ) -> Result<ModifyWorkspacePropertiesResult, ModifyWorkspacePropertiesError> {
        let mut request = SignedRequest::new("POST", "workspaces", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "WorkspacesService.ModifyWorkspaceProperties",
        );
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign_with_plus(&try!(self.credentials_provider.credentials()), true);

        let mut response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Ok(serde_json::from_str::<ModifyWorkspacePropertiesResult>(
                    String::from_utf8_lossy(&body).as_ref(),
                ).unwrap())
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(ModifyWorkspacePropertiesError::from_body(
                    String::from_utf8_lossy(&body).as_ref(),
                ))
            }
        }
    }

    #[doc="<p>Reboots the specified WorkSpaces.</p> <p>To be able to reboot a WorkSpace, the WorkSpace must have a <b>State</b> of <code>AVAILABLE</code>, <code>IMPAIRED</code>, or <code>INOPERABLE</code>.</p> <note> <p>This operation is asynchronous and returns before the WorkSpaces have rebooted.</p> </note>"]
    fn reboot_workspaces(
        &self,
        input: &RebootWorkspacesRequest,
    ) -> Result<RebootWorkspacesResult, RebootWorkspacesError> {
        let mut request = SignedRequest::new("POST", "workspaces", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "WorkspacesService.RebootWorkspaces");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign_with_plus(&try!(self.credentials_provider.credentials()), true);

        let mut response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Ok(serde_json::from_str::<RebootWorkspacesResult>(
                    String::from_utf8_lossy(&body).as_ref(),
                ).unwrap())
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(RebootWorkspacesError::from_body(
                    String::from_utf8_lossy(&body).as_ref(),
                ))
            }
        }
    }

    #[doc="<p>Rebuilds the specified WorkSpaces.</p> <p>Rebuilding a WorkSpace is a potentially destructive action that can result in the loss of data. Rebuilding a WorkSpace causes the following to occur:</p> <ul> <li> <p>The system is restored to the image of the bundle that the WorkSpace is created from. Any applications that have been installed, or system settings that have been made since the WorkSpace was created will be lost.</p> </li> <li> <p>The data drive (D drive) is re-created from the last automatic snapshot taken of the data drive. The current contents of the data drive are overwritten. Automatic snapshots of the data drive are taken every 12 hours, so the snapshot can be as much as 12 hours old.</p> </li> </ul> <p>To be able to rebuild a WorkSpace, the WorkSpace must have a <b>State</b> of <code>AVAILABLE</code> or <code>ERROR</code>.</p> <note> <p>This operation is asynchronous and returns before the WorkSpaces have been completely rebuilt.</p> </note>"]
    fn rebuild_workspaces(
        &self,
        input: &RebuildWorkspacesRequest,
    ) -> Result<RebuildWorkspacesResult, RebuildWorkspacesError> {
        let mut request = SignedRequest::new("POST", "workspaces", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "WorkspacesService.RebuildWorkspaces");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign_with_plus(&try!(self.credentials_provider.credentials()), true);

        let mut response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Ok(serde_json::from_str::<RebuildWorkspacesResult>(
                    String::from_utf8_lossy(&body).as_ref(),
                ).unwrap())
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(RebuildWorkspacesError::from_body(
                    String::from_utf8_lossy(&body).as_ref(),
                ))
            }
        }
    }

    #[doc="<p>Starts the specified WorkSpaces. The WorkSpaces must have a running mode of AutoStop and a state of STOPPED.</p>"]
    fn start_workspaces(
        &self,
        input: &StartWorkspacesRequest,
    ) -> Result<StartWorkspacesResult, StartWorkspacesError> {
        let mut request = SignedRequest::new("POST", "workspaces", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "WorkspacesService.StartWorkspaces");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign_with_plus(&try!(self.credentials_provider.credentials()), true);

        let mut response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Ok(serde_json::from_str::<StartWorkspacesResult>(
                    String::from_utf8_lossy(&body).as_ref(),
                ).unwrap())
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(StartWorkspacesError::from_body(
                    String::from_utf8_lossy(&body).as_ref(),
                ))
            }
        }
    }

    #[doc="<p> Stops the specified WorkSpaces. The WorkSpaces must have a running mode of AutoStop and a state of AVAILABLE, IMPAIRED, UNHEALTHY, or ERROR.</p>"]
    fn stop_workspaces(
        &self,
        input: &StopWorkspacesRequest,
    ) -> Result<StopWorkspacesResult, StopWorkspacesError> {
        let mut request = SignedRequest::new("POST", "workspaces", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "WorkspacesService.StopWorkspaces");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign_with_plus(&try!(self.credentials_provider.credentials()), true);

        let mut response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Ok(serde_json::from_str::<StopWorkspacesResult>(
                    String::from_utf8_lossy(&body).as_ref(),
                ).unwrap())
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(StopWorkspacesError::from_body(
                    String::from_utf8_lossy(&body).as_ref(),
                ))
            }
        }
    }

    #[doc="<p>Terminates the specified WorkSpaces.</p> <p>Terminating a WorkSpace is a permanent action and cannot be undone. The user's data is not maintained and will be destroyed. If you need to archive any user data, contact Amazon Web Services before terminating the WorkSpace.</p> <p>You can terminate a WorkSpace that is in any state except <code>SUSPENDED</code>.</p> <note> <p>This operation is asynchronous and returns before the WorkSpaces have been completely terminated.</p> </note>"]
    fn terminate_workspaces(
        &self,
        input: &TerminateWorkspacesRequest,
    ) -> Result<TerminateWorkspacesResult, TerminateWorkspacesError> {
        let mut request = SignedRequest::new("POST", "workspaces", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "WorkspacesService.TerminateWorkspaces");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign_with_plus(&try!(self.credentials_provider.credentials()), true);

        let mut response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Ok(serde_json::from_str::<TerminateWorkspacesResult>(
                    String::from_utf8_lossy(&body).as_ref(),
                ).unwrap())
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(TerminateWorkspacesError::from_body(
                    String::from_utf8_lossy(&body).as_ref(),
                ))
            }
        }
    }
}

#[cfg(test)]
mod protocol_tests {}
