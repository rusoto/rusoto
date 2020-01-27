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

use rusoto_core::proto;
use rusoto_core::signature::SignedRequest;
#[allow(unused_imports)]
use serde::{Deserialize, Serialize};
use serde_json;
/// <p>Describes a modification to the configuration of Bring Your Own License (BYOL) for the specified account. </p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct AccountModification {
    /// <p>The IP address range, specified as an IPv4 CIDR block, for the management network interface used for the account.</p>
    #[serde(rename = "DedicatedTenancyManagementCidrRange")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dedicated_tenancy_management_cidr_range: Option<String>,
    /// <p>The status of BYOL (whether BYOL is being enabled or disabled).</p>
    #[serde(rename = "DedicatedTenancySupport")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dedicated_tenancy_support: Option<String>,
    /// <p>The error code that is returned if the configuration of BYOL cannot be modified.</p>
    #[serde(rename = "ErrorCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_code: Option<String>,
    /// <p>The text of the error message that is returned if the configuration of BYOL cannot be modified.</p>
    #[serde(rename = "ErrorMessage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
    /// <p>The state of the modification to the configuration of BYOL.</p>
    #[serde(rename = "ModificationState")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modification_state: Option<String>,
    /// <p>The timestamp when the modification of the BYOL configuration was started.</p>
    #[serde(rename = "StartTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<f64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct AssociateIpGroupsRequest {
    /// <p>The identifier of the directory.</p>
    #[serde(rename = "DirectoryId")]
    pub directory_id: String,
    /// <p>The identifiers of one or more IP access control groups.</p>
    #[serde(rename = "GroupIds")]
    pub group_ids: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct AssociateIpGroupsResult {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct AuthorizeIpRulesRequest {
    /// <p>The identifier of the group.</p>
    #[serde(rename = "GroupId")]
    pub group_id: String,
    /// <p>The rules to add to the group.</p>
    #[serde(rename = "UserRules")]
    pub user_rules: Vec<IpRuleItem>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct AuthorizeIpRulesResult {}

/// <p>Describes an Amazon WorkSpaces client.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ClientProperties {
    /// <p>Specifies whether users can cache their credentials on the Amazon WorkSpaces client. When enabled, users can choose to reconnect to their WorkSpaces without re-entering their credentials. </p>
    #[serde(rename = "ReconnectEnabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reconnect_enabled: Option<String>,
}

/// <p>Information about the Amazon WorkSpaces client.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ClientPropertiesResult {
    /// <p>Information about the Amazon WorkSpaces client.</p>
    #[serde(rename = "ClientProperties")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_properties: Option<ClientProperties>,
    /// <p>The resource identifier, in the form of a directory ID.</p>
    #[serde(rename = "ResourceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_id: Option<String>,
}

/// <p>Describes the compute type.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ComputeType {
    /// <p>The compute type.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CopyWorkspaceImageRequest {
    /// <p>A description of the image.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The name of the image.</p>
    #[serde(rename = "Name")]
    pub name: String,
    /// <p>The identifier of the source image.</p>
    #[serde(rename = "SourceImageId")]
    pub source_image_id: String,
    /// <p>The identifier of the source Region.</p>
    #[serde(rename = "SourceRegion")]
    pub source_region: String,
    /// <p>The tags for the image.</p>
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CopyWorkspaceImageResult {
    /// <p>The identifier of the image.</p>
    #[serde(rename = "ImageId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_id: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateIpGroupRequest {
    /// <p>The description of the group.</p>
    #[serde(rename = "GroupDesc")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_desc: Option<String>,
    /// <p>The name of the group.</p>
    #[serde(rename = "GroupName")]
    pub group_name: String,
    /// <p>The tags. Each WorkSpaces resource can have a maximum of 50 tags.</p>
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
    /// <p>The rules to add to the group.</p>
    #[serde(rename = "UserRules")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_rules: Option<Vec<IpRuleItem>>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateIpGroupResult {
    /// <p>The identifier of the group.</p>
    #[serde(rename = "GroupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_id: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateTagsRequest {
    /// <p>The identifier of the WorkSpaces resource. The supported resource types are WorkSpaces, registered directories, images, custom bundles, and IP access control groups.</p>
    #[serde(rename = "ResourceId")]
    pub resource_id: String,
    /// <p>The tags. Each WorkSpaces resource can have a maximum of 50 tags. If you want to add new tags to a set of existing tags, you must submit all of the existing tags along with the new ones.</p>
    #[serde(rename = "Tags")]
    pub tags: Vec<Tag>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateTagsResult {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateWorkspacesRequest {
    /// <p>The WorkSpaces to create. You can specify up to 25 WorkSpaces.</p>
    #[serde(rename = "Workspaces")]
    pub workspaces: Vec<WorkspaceRequest>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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

/// <p>Describes the default values that are used to create WorkSpaces. For more information, see <a href="https://docs.aws.amazon.com/workspaces/latest/adminguide/update-directory-details.html">Update Directory Details for Your WorkSpaces</a>.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DefaultWorkspaceCreationProperties {
    /// <p>The identifier of any security groups to apply to WorkSpaces when they are created.</p>
    #[serde(rename = "CustomSecurityGroupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_security_group_id: Option<String>,
    /// <p>The organizational unit (OU) in the directory for the WorkSpace machine accounts.</p>
    #[serde(rename = "DefaultOu")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_ou: Option<String>,
    /// <p>Specifies whether to automatically assign an Elastic public IP address to WorkSpaces in this directory by default. If enabled, the Elastic public IP address allows outbound internet access from your WorkSpaces when you’re using an internet gateway in the Amazon VPC in which your WorkSpaces are located. If you're using a Network Address Translation (NAT) gateway for outbound internet access from your VPC, or if your WorkSpaces are in public subnets and you manually assign them Elastic IP addresses, you should disable this setting. This setting applies to new WorkSpaces that you launch or to existing WorkSpaces that you rebuild. For more information, see <a href="https://docs.aws.amazon.com/workspaces/latest/adminguide/amazon-workspaces-vpc.html"> Configure a VPC for Amazon WorkSpaces</a>.</p>
    #[serde(rename = "EnableInternetAccess")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_internet_access: Option<bool>,
    /// <p>Specifies whether maintenance mode is enabled for WorkSpaces. For more information, see <a href="https://docs.aws.amazon.com/workspaces/latest/adminguide/workspace-maintenance.html">WorkSpace Maintenance</a>.</p>
    #[serde(rename = "EnableMaintenanceMode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_maintenance_mode: Option<bool>,
    /// <p>Specifies whether the directory is enabled for Amazon WorkDocs.</p>
    #[serde(rename = "EnableWorkDocs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_work_docs: Option<bool>,
    /// <p>Specifies whether WorkSpace users are local administrators on their WorkSpaces.</p>
    #[serde(rename = "UserEnabledAsLocalAdministrator")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_enabled_as_local_administrator: Option<bool>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteIpGroupRequest {
    /// <p>The identifier of the IP access control group.</p>
    #[serde(rename = "GroupId")]
    pub group_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DeleteIpGroupResult {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteTagsRequest {
    /// <p>The identifier of the WorkSpaces resource. The supported resource types are WorkSpaces, registered directories, images, custom bundles, and IP access control groups.</p>
    #[serde(rename = "ResourceId")]
    pub resource_id: String,
    /// <p>The tag keys.</p>
    #[serde(rename = "TagKeys")]
    pub tag_keys: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DeleteTagsResult {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteWorkspaceImageRequest {
    /// <p>The identifier of the image.</p>
    #[serde(rename = "ImageId")]
    pub image_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DeleteWorkspaceImageResult {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeregisterWorkspaceDirectoryRequest {
    /// <p>The identifier of the directory. If any WorkSpaces are registered to this directory, you must remove them before you deregister the directory, or you will receive an OperationNotSupportedException error.</p>
    #[serde(rename = "DirectoryId")]
    pub directory_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DeregisterWorkspaceDirectoryResult {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeAccountModificationsRequest {
    /// <p>If you received a <code>NextToken</code> from a previous call that was paginated, provide this token to receive the next set of results.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeAccountModificationsResult {
    /// <p>The list of modifications to the configuration of BYOL.</p>
    #[serde(rename = "AccountModifications")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_modifications: Option<Vec<AccountModification>>,
    /// <p>The token to use to retrieve the next set of results, or null if no more results are available.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeAccountRequest {}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeAccountResult {
    /// <p>The IP address range, specified as an IPv4 CIDR block, used for the management network interface.</p> <p>The management network interface is connected to a secure Amazon WorkSpaces management network. It is used for interactive streaming of the WorkSpace desktop to Amazon WorkSpaces clients, and to allow Amazon WorkSpaces to manage the WorkSpace.</p>
    #[serde(rename = "DedicatedTenancyManagementCidrRange")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dedicated_tenancy_management_cidr_range: Option<String>,
    /// <p>The status of BYOL (whether BYOL is enabled or disabled).</p>
    #[serde(rename = "DedicatedTenancySupport")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dedicated_tenancy_support: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeClientPropertiesRequest {
    /// <p>The resource identifier, in the form of directory IDs.</p>
    #[serde(rename = "ResourceIds")]
    pub resource_ids: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeClientPropertiesResult {
    /// <p>Information about the specified Amazon WorkSpaces clients.</p>
    #[serde(rename = "ClientPropertiesList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_properties_list: Option<Vec<ClientPropertiesResult>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeIpGroupsRequest {
    /// <p>The identifiers of one or more IP access control groups.</p>
    #[serde(rename = "GroupIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_ids: Option<Vec<String>>,
    /// <p>The maximum number of items to return.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>If you received a <code>NextToken</code> from a previous call that was paginated, provide this token to receive the next set of results.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeIpGroupsResult {
    /// <p>The token to use to retrieve the next set of results, or null if no more results are available.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>Information about the IP access control groups.</p>
    #[serde(rename = "Result")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<Vec<WorkspacesIpGroup>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeTagsRequest {
    /// <p>The identifier of the WorkSpaces resource. The supported resource types are WorkSpaces, registered directories, images, custom bundles, and IP access control groups.</p>
    #[serde(rename = "ResourceId")]
    pub resource_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeTagsResult {
    /// <p>The tags.</p>
    #[serde(rename = "TagList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_list: Option<Vec<Tag>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeWorkspaceBundlesRequest {
    /// <p>The identifiers of the bundles. You cannot combine this parameter with any other filter.</p>
    #[serde(rename = "BundleIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bundle_ids: Option<Vec<String>>,
    /// <p>The token for the next set of results. (You received this token from a previous call.)</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The owner of the bundles. You cannot combine this parameter with any other filter.</p> <p>Specify <code>AMAZON</code> to describe the bundles provided by AWS or null to describe the bundles that belong to your account.</p>
    #[serde(rename = "Owner")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeWorkspaceDirectoriesRequest {
    /// <p>The identifiers of the directories. If the value is null, all directories are retrieved.</p>
    #[serde(rename = "DirectoryIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub directory_ids: Option<Vec<String>>,
    /// <p>The maximum number of directories to return.</p>
    #[serde(rename = "Limit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// <p>If you received a <code>NextToken</code> from a previous call that was paginated, provide this token to receive the next set of results.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeWorkspaceDirectoriesResult {
    /// <p>Information about the directories.</p>
    #[serde(rename = "Directories")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub directories: Option<Vec<WorkspaceDirectory>>,
    /// <p>The token to use to retrieve the next set of results, or null if no more results are available.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeWorkspaceImagesRequest {
    /// <p>The identifier of the image.</p>
    #[serde(rename = "ImageIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_ids: Option<Vec<String>>,
    /// <p>The maximum number of items to return.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>If you received a <code>NextToken</code> from a previous call that was paginated, provide this token to receive the next set of results.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeWorkspaceImagesResult {
    /// <p>Information about the images.</p>
    #[serde(rename = "Images")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub images: Option<Vec<WorkspaceImage>>,
    /// <p>The token to use to retrieve the next set of results, or null if no more results are available.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeWorkspaceSnapshotsRequest {
    /// <p>The identifier of the WorkSpace.</p>
    #[serde(rename = "WorkspaceId")]
    pub workspace_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeWorkspaceSnapshotsResult {
    /// <p>Information about the snapshots that can be used to rebuild a WorkSpace. These snapshots include the user volume.</p>
    #[serde(rename = "RebuildSnapshots")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rebuild_snapshots: Option<Vec<Snapshot>>,
    /// <p>Information about the snapshots that can be used to restore a WorkSpace. These snapshots include both the root volume and the user volume.</p>
    #[serde(rename = "RestoreSnapshots")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub restore_snapshots: Option<Vec<Snapshot>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeWorkspacesConnectionStatusRequest {
    /// <p>If you received a <code>NextToken</code> from a previous call that was paginated, provide this token to receive the next set of results.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The identifiers of the WorkSpaces. You can specify up to 25 WorkSpaces.</p>
    #[serde(rename = "WorkspaceIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workspace_ids: Option<Vec<String>>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeWorkspacesConnectionStatusResult {
    /// <p>The token to use to retrieve the next set of results, or null if no more results are available.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>Information about the connection status of the WorkSpace.</p>
    #[serde(rename = "WorkspacesConnectionStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workspaces_connection_status: Option<Vec<WorkspaceConnectionStatus>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeWorkspacesRequest {
    /// <p>The identifier of the bundle. All WorkSpaces that are created from this bundle are retrieved. You cannot combine this parameter with any other filter.</p>
    #[serde(rename = "BundleId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bundle_id: Option<String>,
    /// <p>The identifier of the directory. In addition, you can optionally specify a specific directory user (see <code>UserName</code>). You cannot combine this parameter with any other filter.</p>
    #[serde(rename = "DirectoryId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub directory_id: Option<String>,
    /// <p>The maximum number of items to return.</p>
    #[serde(rename = "Limit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// <p>If you received a <code>NextToken</code> from a previous call that was paginated, provide this token to receive the next set of results.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The name of the directory user. You must specify this parameter with <code>DirectoryId</code>.</p>
    #[serde(rename = "UserName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_name: Option<String>,
    /// <p>The identifiers of the WorkSpaces. You cannot combine this parameter with any other filter.</p> <p>Because the <a>CreateWorkspaces</a> operation is asynchronous, the identifier it returns is not immediately available. If you immediately call <a>DescribeWorkspaces</a> with this identifier, no information is returned.</p>
    #[serde(rename = "WorkspaceIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workspace_ids: Option<Vec<String>>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeWorkspacesResult {
    /// <p>The token to use to retrieve the next set of results, or null if no more results are available.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>Information about the WorkSpaces.</p> <p>Because <a>CreateWorkspaces</a> is an asynchronous operation, some of the returned information could be incomplete.</p>
    #[serde(rename = "Workspaces")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workspaces: Option<Vec<Workspace>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DisassociateIpGroupsRequest {
    /// <p>The identifier of the directory.</p>
    #[serde(rename = "DirectoryId")]
    pub directory_id: String,
    /// <p>The identifiers of one or more IP access control groups.</p>
    #[serde(rename = "GroupIds")]
    pub group_ids: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DisassociateIpGroupsResult {}

/// <p>Describes a WorkSpace that cannot be created.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct FailedCreateWorkspaceRequest {
    /// <p>The error code that is returned if the WorkSpace cannot be created.</p>
    #[serde(rename = "ErrorCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_code: Option<String>,
    /// <p>The text of the error message that is returned if the WorkSpace cannot be created.</p>
    #[serde(rename = "ErrorMessage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
    /// <p>Information about the WorkSpace.</p>
    #[serde(rename = "WorkspaceRequest")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workspace_request: Option<WorkspaceRequest>,
}

/// <p>Describes a WorkSpace that could not be rebooted. (<a>RebootWorkspaces</a>), rebuilt (<a>RebuildWorkspaces</a>), restored (<a>RestoreWorkspace</a>), terminated (<a>TerminateWorkspaces</a>), started (<a>StartWorkspaces</a>), or stopped (<a>StopWorkspaces</a>).</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct FailedWorkspaceChangeRequest {
    /// <p>The error code that is returned if the WorkSpace cannot be rebooted.</p>
    #[serde(rename = "ErrorCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_code: Option<String>,
    /// <p>The text of the error message that is returned if the WorkSpace cannot be rebooted.</p>
    #[serde(rename = "ErrorMessage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
    /// <p>The identifier of the WorkSpace.</p>
    #[serde(rename = "WorkspaceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workspace_id: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ImportWorkspaceImageRequest {
    /// <p>The identifier of the EC2 image.</p>
    #[serde(rename = "Ec2ImageId")]
    pub ec_2_image_id: String,
    /// <p>The description of the WorkSpace image.</p>
    #[serde(rename = "ImageDescription")]
    pub image_description: String,
    /// <p>The name of the WorkSpace image.</p>
    #[serde(rename = "ImageName")]
    pub image_name: String,
    /// <p>The ingestion process to be used when importing the image.</p>
    #[serde(rename = "IngestionProcess")]
    pub ingestion_process: String,
    /// <p>The tags. Each WorkSpaces resource can have a maximum of 50 tags.</p>
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ImportWorkspaceImageResult {
    /// <p>The identifier of the WorkSpace image.</p>
    #[serde(rename = "ImageId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_id: Option<String>,
}

/// <p>Describes a rule for an IP access control group.</p>
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

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListAvailableManagementCidrRangesRequest {
    /// <p>The IP address range to search. Specify an IP address range that is compatible with your network and in CIDR notation (that is, specify the range as an IPv4 CIDR block).</p>
    #[serde(rename = "ManagementCidrRangeConstraint")]
    pub management_cidr_range_constraint: String,
    /// <p>The maximum number of items to return.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>If you received a <code>NextToken</code> from a previous call that was paginated, provide this token to receive the next set of results.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListAvailableManagementCidrRangesResult {
    /// <p>The list of available IP address ranges, specified as IPv4 CIDR blocks.</p>
    #[serde(rename = "ManagementCidrRanges")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub management_cidr_ranges: Option<Vec<String>>,
    /// <p>The token to use to retrieve the next set of results, or null if no more results are available.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct MigrateWorkspaceRequest {
    /// <p>The identifier of the target bundle type to migrate the WorkSpace to.</p>
    #[serde(rename = "BundleId")]
    pub bundle_id: String,
    /// <p>The identifier of the WorkSpace to migrate from.</p>
    #[serde(rename = "SourceWorkspaceId")]
    pub source_workspace_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct MigrateWorkspaceResult {
    /// <p>The original identifier of the WorkSpace that is being migrated.</p>
    #[serde(rename = "SourceWorkspaceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_workspace_id: Option<String>,
    /// <p>The new identifier of the WorkSpace that is being migrated. If the migration does not succeed, the target WorkSpace ID will not be used, and the WorkSpace will still have the original WorkSpace ID.</p>
    #[serde(rename = "TargetWorkspaceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_workspace_id: Option<String>,
}

/// <p>Describes a WorkSpace modification.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ModifyAccountRequest {
    /// <p>The IP address range, specified as an IPv4 CIDR block, for the management network interface. Specify an IP address range that is compatible with your network and in CIDR notation (that is, specify the range as an IPv4 CIDR block). The CIDR block size must be /16 (for example, 203.0.113.25/16). It must also be specified as available by the <code>ListAvailableManagementCidrRanges</code> operation.</p>
    #[serde(rename = "DedicatedTenancyManagementCidrRange")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dedicated_tenancy_management_cidr_range: Option<String>,
    /// <p>The status of BYOL.</p>
    #[serde(rename = "DedicatedTenancySupport")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dedicated_tenancy_support: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ModifyAccountResult {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ModifyClientPropertiesRequest {
    /// <p>Information about the Amazon WorkSpaces client.</p>
    #[serde(rename = "ClientProperties")]
    pub client_properties: ClientProperties,
    /// <p>The resource identifiers, in the form of directory IDs.</p>
    #[serde(rename = "ResourceId")]
    pub resource_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ModifyClientPropertiesResult {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ModifySelfservicePermissionsRequest {
    /// <p>The identifier of the directory.</p>
    #[serde(rename = "ResourceId")]
    pub resource_id: String,
    /// <p>The permissions to enable or disable self-service capabilities.</p>
    #[serde(rename = "SelfservicePermissions")]
    pub selfservice_permissions: SelfservicePermissions,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ModifySelfservicePermissionsResult {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ModifyWorkspaceAccessPropertiesRequest {
    /// <p>The identifier of the directory.</p>
    #[serde(rename = "ResourceId")]
    pub resource_id: String,
    /// <p>The device types and operating systems to enable or disable for access.</p>
    #[serde(rename = "WorkspaceAccessProperties")]
    pub workspace_access_properties: WorkspaceAccessProperties,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ModifyWorkspaceAccessPropertiesResult {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ModifyWorkspaceCreationPropertiesRequest {
    /// <p>The identifier of the directory.</p>
    #[serde(rename = "ResourceId")]
    pub resource_id: String,
    /// <p>The default properties for creating WorkSpaces.</p>
    #[serde(rename = "WorkspaceCreationProperties")]
    pub workspace_creation_properties: WorkspaceCreationProperties,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ModifyWorkspaceCreationPropertiesResult {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ModifyWorkspacePropertiesRequest {
    /// <p>The identifier of the WorkSpace.</p>
    #[serde(rename = "WorkspaceId")]
    pub workspace_id: String,
    /// <p>The properties of the WorkSpace.</p>
    #[serde(rename = "WorkspaceProperties")]
    pub workspace_properties: WorkspaceProperties,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ModifyWorkspacePropertiesResult {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ModifyWorkspaceStateRequest {
    /// <p>The identifier of the WorkSpace.</p>
    #[serde(rename = "WorkspaceId")]
    pub workspace_id: String,
    /// <p>The WorkSpace state.</p>
    #[serde(rename = "WorkspaceState")]
    pub workspace_state: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ModifyWorkspaceStateResult {}

/// <p>The operating system that the image is running.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct OperatingSystem {
    /// <p>The operating system.</p>
    #[serde(rename = "Type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}

/// <p>Describes the information used to reboot a WorkSpace.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct RebootRequest {
    /// <p>The identifier of the WorkSpace.</p>
    #[serde(rename = "WorkspaceId")]
    pub workspace_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct RebootWorkspacesRequest {
    /// <p>The WorkSpaces to reboot. You can specify up to 25 WorkSpaces.</p>
    #[serde(rename = "RebootWorkspaceRequests")]
    pub reboot_workspace_requests: Vec<RebootRequest>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct RebootWorkspacesResult {
    /// <p>Information about the WorkSpaces that could not be rebooted.</p>
    #[serde(rename = "FailedRequests")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failed_requests: Option<Vec<FailedWorkspaceChangeRequest>>,
}

/// <p>Describes the information used to rebuild a WorkSpace.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct RebuildRequest {
    /// <p>The identifier of the WorkSpace.</p>
    #[serde(rename = "WorkspaceId")]
    pub workspace_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct RebuildWorkspacesRequest {
    /// <p>The WorkSpace to rebuild. You can specify a single WorkSpace.</p>
    #[serde(rename = "RebuildWorkspaceRequests")]
    pub rebuild_workspace_requests: Vec<RebuildRequest>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct RebuildWorkspacesResult {
    /// <p>Information about the WorkSpace that could not be rebuilt.</p>
    #[serde(rename = "FailedRequests")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failed_requests: Option<Vec<FailedWorkspaceChangeRequest>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct RegisterWorkspaceDirectoryRequest {
    /// <p>The identifier of the directory. You cannot register a directory if it does not have a status of Active. If the directory does not have a status of Active, you will receive an InvalidResourceStateException error. If you have already registered the maximum number of directories that you can register with Amazon WorkSpaces, you will receive a ResourceLimitExceededException error. Deregister directories that you are not using for WorkSpaces, and try again.</p>
    #[serde(rename = "DirectoryId")]
    pub directory_id: String,
    /// <p>Indicates whether self-service capabilities are enabled or disabled.</p>
    #[serde(rename = "EnableSelfService")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_self_service: Option<bool>,
    /// <p>Indicates whether Amazon WorkDocs is enabled or disabled. If you have enabled this parameter and WorkDocs is not available in the Region, you will receive an OperationNotSupportedException error. Set <code>EnableWorkDocs</code> to disabled, and try again.</p>
    #[serde(rename = "EnableWorkDocs")]
    pub enable_work_docs: bool,
    /// <p>The identifiers of the subnets for your virtual private cloud (VPC). Make sure that the subnets are in supported Availability Zones. The subnets must also be in separate Availability Zones. If these conditions are not met, you will receive an OperationNotSupportedException error.</p>
    #[serde(rename = "SubnetIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subnet_ids: Option<Vec<String>>,
    /// <p>The tags associated with the directory.</p>
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
    /// <p>Indicates whether your WorkSpace directory is dedicated or shared. To use Bring Your Own License (BYOL) images, this value must be set to <code>DEDICATED</code> and your AWS account must be enabled for BYOL. If your account has not been enabled for BYOL, you will receive an InvalidParameterValuesException error. For more information about BYOL images, see <a href="https://docs.aws.amazon.com/workspaces/latest/adminguide/byol-windows-images.html">Bring Your Own Windows Desktop Images</a>.</p>
    #[serde(rename = "Tenancy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tenancy: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct RegisterWorkspaceDirectoryResult {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct RestoreWorkspaceRequest {
    /// <p>The identifier of the WorkSpace.</p>
    #[serde(rename = "WorkspaceId")]
    pub workspace_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct RestoreWorkspaceResult {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct RevokeIpRulesRequest {
    /// <p>The identifier of the group.</p>
    #[serde(rename = "GroupId")]
    pub group_id: String,
    /// <p>The rules to remove from the group.</p>
    #[serde(rename = "UserRules")]
    pub user_rules: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct RevokeIpRulesResult {}

/// <p>Describes the root volume for a WorkSpace bundle.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct RootStorage {
    /// <p>The size of the root volume.</p>
    #[serde(rename = "Capacity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capacity: Option<String>,
}

/// <p>Describes the self-service permissions for a directory. For more information, see <a href="https://docs.aws.amazon.com/workspaces/latest/adminguide/enable-user-self-service-workspace-management.html">Enable Self-Service WorkSpace Management Capabilities for Your Users</a>.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SelfservicePermissions {
    /// <p>Specifies whether users can change the compute type (bundle) for their WorkSpace.</p>
    #[serde(rename = "ChangeComputeType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub change_compute_type: Option<String>,
    /// <p>Specifies whether users can increase the volume size of the drives on their WorkSpace.</p>
    #[serde(rename = "IncreaseVolumeSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub increase_volume_size: Option<String>,
    /// <p>Specifies whether users can rebuild the operating system of a WorkSpace to its original state.</p>
    #[serde(rename = "RebuildWorkspace")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rebuild_workspace: Option<String>,
    /// <p>Specifies whether users can restart their WorkSpace.</p>
    #[serde(rename = "RestartWorkspace")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub restart_workspace: Option<String>,
    /// <p>Specifies whether users can switch the running mode of their WorkSpace.</p>
    #[serde(rename = "SwitchRunningMode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub switch_running_mode: Option<String>,
}

/// <p>Describes a snapshot.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct Snapshot {
    /// <p>The time when the snapshot was created.</p>
    #[serde(rename = "SnapshotTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub snapshot_time: Option<f64>,
}

/// <p>Information used to start a WorkSpace.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct StartRequest {
    /// <p>The identifier of the WorkSpace.</p>
    #[serde(rename = "WorkspaceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workspace_id: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct StartWorkspacesRequest {
    /// <p>The WorkSpaces to start. You can specify up to 25 WorkSpaces.</p>
    #[serde(rename = "StartWorkspaceRequests")]
    pub start_workspace_requests: Vec<StartRequest>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct StartWorkspacesResult {
    /// <p>Information about the WorkSpaces that could not be started.</p>
    #[serde(rename = "FailedRequests")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failed_requests: Option<Vec<FailedWorkspaceChangeRequest>>,
}

/// <p>Describes the information used to stop a WorkSpace.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct StopRequest {
    /// <p>The identifier of the WorkSpace.</p>
    #[serde(rename = "WorkspaceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workspace_id: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct StopWorkspacesRequest {
    /// <p>The WorkSpaces to stop. You can specify up to 25 WorkSpaces.</p>
    #[serde(rename = "StopWorkspaceRequests")]
    pub stop_workspace_requests: Vec<StopRequest>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct StopWorkspacesResult {
    /// <p>Information about the WorkSpaces that could not be stopped.</p>
    #[serde(rename = "FailedRequests")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failed_requests: Option<Vec<FailedWorkspaceChangeRequest>>,
}

/// <p>Describes a tag.</p>
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

/// <p>Describes the information used to terminate a WorkSpace.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct TerminateRequest {
    /// <p>The identifier of the WorkSpace.</p>
    #[serde(rename = "WorkspaceId")]
    pub workspace_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct TerminateWorkspacesRequest {
    /// <p>The WorkSpaces to terminate. You can specify up to 25 WorkSpaces.</p>
    #[serde(rename = "TerminateWorkspaceRequests")]
    pub terminate_workspace_requests: Vec<TerminateRequest>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct TerminateWorkspacesResult {
    /// <p>Information about the WorkSpaces that could not be terminated.</p>
    #[serde(rename = "FailedRequests")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failed_requests: Option<Vec<FailedWorkspaceChangeRequest>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UpdateRulesOfIpGroupRequest {
    /// <p>The identifier of the group.</p>
    #[serde(rename = "GroupId")]
    pub group_id: String,
    /// <p>One or more rules.</p>
    #[serde(rename = "UserRules")]
    pub user_rules: Vec<IpRuleItem>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UpdateRulesOfIpGroupResult {}

/// <p>Describes the user storage for a WorkSpace bundle.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UserStorage {
    /// <p>The size of the user storage.</p>
    #[serde(rename = "Capacity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capacity: Option<String>,
}

/// <p>Describes a WorkSpace.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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
    /// <p>The error code that is returned if the WorkSpace cannot be created.</p>
    #[serde(rename = "ErrorCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_code: Option<String>,
    /// <p>The text of the error message that is returned if the WorkSpace cannot be created.</p>
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
    /// <p>The symmetric AWS KMS customer master key (CMK) used to encrypt data stored on your WorkSpace. Amazon WorkSpaces does not support asymmetric CMKs.</p>
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

/// <p>The device types and operating systems that can be used to access a WorkSpace. For more information, see <a href="https://docs.aws.amazon.com/workspaces/latest/adminguide/workspaces-network-requirements.html">Amazon WorkSpaces Client Network Requirements</a>.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WorkspaceAccessProperties {
    /// <p>Indicates whether users can use Android devices to access their WorkSpaces.</p>
    #[serde(rename = "DeviceTypeAndroid")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_type_android: Option<String>,
    /// <p>Indicates whether users can use Chromebooks to access their WorkSpaces.</p>
    #[serde(rename = "DeviceTypeChromeOs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_type_chrome_os: Option<String>,
    /// <p>Indicates whether users can use iOS devices to access their WorkSpaces.</p>
    #[serde(rename = "DeviceTypeIos")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_type_ios: Option<String>,
    /// <p>Indicates whether users can use macOS clients to access their WorkSpaces. To restrict WorkSpaces access to trusted devices (also known as managed devices) with valid certificates, specify a value of <code>TRUST</code>. For more information, see <a href="https://docs.aws.amazon.com/workspaces/latest/adminguide/trusted-devices.html">Restrict WorkSpaces Access to Trusted Devices</a>. </p>
    #[serde(rename = "DeviceTypeOsx")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_type_osx: Option<String>,
    /// <p>Indicates whether users can access their WorkSpaces through a web browser.</p>
    #[serde(rename = "DeviceTypeWeb")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_type_web: Option<String>,
    /// <p>Indicates whether users can use Windows clients to access their WorkSpaces. To restrict WorkSpaces access to trusted devices (also known as managed devices) with valid certificates, specify a value of <code>TRUST</code>. For more information, see <a href="https://docs.aws.amazon.com/workspaces/latest/adminguide/trusted-devices.html">Restrict WorkSpaces Access to Trusted Devices</a>. </p>
    #[serde(rename = "DeviceTypeWindows")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_type_windows: Option<String>,
    /// <p>Indicates whether users can use zero client devices to access their WorkSpaces.</p>
    #[serde(rename = "DeviceTypeZeroClient")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_type_zero_client: Option<String>,
}

/// <p>Describes a WorkSpace bundle.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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
    /// <p>The image identifier of the bundle.</p>
    #[serde(rename = "ImageId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_id: Option<String>,
    /// <p>The last time that the bundle was updated.</p>
    #[serde(rename = "LastUpdatedTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_time: Option<f64>,
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct WorkspaceConnectionStatus {
    /// <p>The connection state of the WorkSpace. The connection state is unknown if the WorkSpace is stopped.</p>
    #[serde(rename = "ConnectionState")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connection_state: Option<String>,
    /// <p>The timestamp of the connection status check.</p>
    #[serde(rename = "ConnectionStateCheckTimestamp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connection_state_check_timestamp: Option<f64>,
    /// <p>The timestamp of the last known user connection.</p>
    #[serde(rename = "LastKnownUserConnectionTimestamp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_known_user_connection_timestamp: Option<f64>,
    /// <p>The identifier of the WorkSpace.</p>
    #[serde(rename = "WorkspaceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workspace_id: Option<String>,
}

/// <p>Describes the default properties that are used for creating WorkSpaces. For more information, see <a href="https://docs.aws.amazon.com/workspaces/latest/adminguide/update-directory-details.html">Update Directory Details for Your WorkSpaces</a>. </p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct WorkspaceCreationProperties {
    /// <p>The identifier of your custom security group.</p>
    #[serde(rename = "CustomSecurityGroupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_security_group_id: Option<String>,
    /// <p>The default organizational unit (OU) for your WorkSpace directories.</p>
    #[serde(rename = "DefaultOu")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_ou: Option<String>,
    /// <p>Indicates whether internet access is enabled for your WorkSpaces.</p>
    #[serde(rename = "EnableInternetAccess")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_internet_access: Option<bool>,
    /// <p>Indicates whether maintenance mode is enabled for your WorkSpaces. For more information, see <a href="https://docs.aws.amazon.com/workspaces/latest/adminguide/workspace-maintenance.html">WorkSpace Maintenance</a>. </p>
    #[serde(rename = "EnableMaintenanceMode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_maintenance_mode: Option<bool>,
    /// <p>Indicates whether users are local administrators of their WorkSpaces.</p>
    #[serde(rename = "UserEnabledAsLocalAdministrator")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_enabled_as_local_administrator: Option<bool>,
}

/// <p>Describes a directory that is used with Amazon WorkSpaces.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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
    /// <p>The default self-service permissions for WorkSpaces in the directory.</p>
    #[serde(rename = "SelfservicePermissions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub selfservice_permissions: Option<SelfservicePermissions>,
    /// <p>The state of the directory's registration with Amazon WorkSpaces.</p>
    #[serde(rename = "State")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    /// <p>The identifiers of the subnets used with the directory.</p>
    #[serde(rename = "SubnetIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subnet_ids: Option<Vec<String>>,
    /// <p>Specifies whether the directory is dedicated or shared. To use Bring Your Own License (BYOL), this value must be set to <code>DEDICATED</code>. For more information, see <a href="https://docs.aws.amazon.com/workspaces/latest/adminguide/byol-windows-images.html">Bring Your Own Windows Desktop Images</a>.</p>
    #[serde(rename = "Tenancy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tenancy: Option<String>,
    /// <p>The devices and operating systems that users can use to access WorkSpaces.</p>
    #[serde(rename = "WorkspaceAccessProperties")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workspace_access_properties: Option<WorkspaceAccessProperties>,
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

/// <p>Describes a WorkSpace image.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct WorkspaceImage {
    /// <p>The description of the image.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The error code that is returned for the image.</p>
    #[serde(rename = "ErrorCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_code: Option<String>,
    /// <p>The text of the error message that is returned for the image.</p>
    #[serde(rename = "ErrorMessage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
    /// <p>The identifier of the image.</p>
    #[serde(rename = "ImageId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_id: Option<String>,
    /// <p>The name of the image.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The operating system that the image is running. </p>
    #[serde(rename = "OperatingSystem")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operating_system: Option<OperatingSystem>,
    /// <p>Specifies whether the image is running on dedicated hardware. When Bring Your Own License (BYOL) is enabled, this value is set to <code>DEDICATED</code>. For more information, see <a href="https://docs.aws.amazon.com/workspaces/latest/adminguide/byol-windows-images.html">Bring Your Own Windows Desktop Images</a>.</p>
    #[serde(rename = "RequiredTenancy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub required_tenancy: Option<String>,
    /// <p>The status of the image.</p>
    #[serde(rename = "State")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
}

/// <p>Describes a WorkSpace.</p>
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
    /// <p>The running mode. For more information, see <a href="https://docs.aws.amazon.com/workspaces/latest/adminguide/running-mode.html">Manage the WorkSpace Running Mode</a>.</p>
    #[serde(rename = "RunningMode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub running_mode: Option<String>,
    /// <p>The time after a user logs off when WorkSpaces are automatically stopped. Configured in 60-minute intervals.</p>
    #[serde(rename = "RunningModeAutoStopTimeoutInMinutes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub running_mode_auto_stop_timeout_in_minutes: Option<i64>,
    /// <p>The size of the user storage.</p>
    #[serde(rename = "UserVolumeSizeGib")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_volume_size_gib: Option<i64>,
}

/// <p>Describes the information used to create a WorkSpace.</p>
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
    /// <p>The user name of the user for the WorkSpace. This user name must exist in the AWS Directory Service directory for the WorkSpace.</p>
    #[serde(rename = "UserName")]
    pub user_name: String,
    /// <p>Indicates whether the data stored on the user volume is encrypted.</p>
    #[serde(rename = "UserVolumeEncryptionEnabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_volume_encryption_enabled: Option<bool>,
    /// <p>The symmetric AWS KMS customer master key (CMK) used to encrypt data stored on your WorkSpace. Amazon WorkSpaces does not support asymmetric CMKs.</p>
    #[serde(rename = "VolumeEncryptionKey")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volume_encryption_key: Option<String>,
    /// <p>The WorkSpace properties.</p>
    #[serde(rename = "WorkspaceProperties")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workspace_properties: Option<WorkspaceProperties>,
}

/// <p>Describes an IP access control group.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct WorkspacesIpGroup {
    /// <p>The description of the group.</p>
    #[serde(rename = "groupDesc")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_desc: Option<String>,
    /// <p>The identifier of the group.</p>
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
}

impl AssociateIpGroupsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<AssociateIpGroupsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(AssociateIpGroupsError::AccessDenied(err.msg))
                }
                "InvalidParameterValuesException" => {
                    return RusotoError::Service(AssociateIpGroupsError::InvalidParameterValues(
                        err.msg,
                    ))
                }
                "InvalidResourceStateException" => {
                    return RusotoError::Service(AssociateIpGroupsError::InvalidResourceState(
                        err.msg,
                    ))
                }
                "OperationNotSupportedException" => {
                    return RusotoError::Service(AssociateIpGroupsError::OperationNotSupported(
                        err.msg,
                    ))
                }
                "ResourceLimitExceededException" => {
                    return RusotoError::Service(AssociateIpGroupsError::ResourceLimitExceeded(
                        err.msg,
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(AssociateIpGroupsError::ResourceNotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for AssociateIpGroupsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            AssociateIpGroupsError::AccessDenied(ref cause) => write!(f, "{}", cause),
            AssociateIpGroupsError::InvalidParameterValues(ref cause) => write!(f, "{}", cause),
            AssociateIpGroupsError::InvalidResourceState(ref cause) => write!(f, "{}", cause),
            AssociateIpGroupsError::OperationNotSupported(ref cause) => write!(f, "{}", cause),
            AssociateIpGroupsError::ResourceLimitExceeded(ref cause) => write!(f, "{}", cause),
            AssociateIpGroupsError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for AssociateIpGroupsError {}
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
}

impl AuthorizeIpRulesError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<AuthorizeIpRulesError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(AuthorizeIpRulesError::AccessDenied(err.msg))
                }
                "InvalidParameterValuesException" => {
                    return RusotoError::Service(AuthorizeIpRulesError::InvalidParameterValues(
                        err.msg,
                    ))
                }
                "InvalidResourceStateException" => {
                    return RusotoError::Service(AuthorizeIpRulesError::InvalidResourceState(
                        err.msg,
                    ))
                }
                "ResourceLimitExceededException" => {
                    return RusotoError::Service(AuthorizeIpRulesError::ResourceLimitExceeded(
                        err.msg,
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(AuthorizeIpRulesError::ResourceNotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for AuthorizeIpRulesError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            AuthorizeIpRulesError::AccessDenied(ref cause) => write!(f, "{}", cause),
            AuthorizeIpRulesError::InvalidParameterValues(ref cause) => write!(f, "{}", cause),
            AuthorizeIpRulesError::InvalidResourceState(ref cause) => write!(f, "{}", cause),
            AuthorizeIpRulesError::ResourceLimitExceeded(ref cause) => write!(f, "{}", cause),
            AuthorizeIpRulesError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for AuthorizeIpRulesError {}
/// Errors returned by CopyWorkspaceImage
#[derive(Debug, PartialEq)]
pub enum CopyWorkspaceImageError {
    /// <p>The user is not authorized to access a resource.</p>
    AccessDenied(String),
    /// <p>One or more parameter values are not valid.</p>
    InvalidParameterValues(String),
    /// <p>This operation is not supported.</p>
    OperationNotSupported(String),
    /// <p>The specified resource already exists.</p>
    ResourceAlreadyExists(String),
    /// <p>Your resource limits have been exceeded.</p>
    ResourceLimitExceeded(String),
    /// <p>The resource could not be found.</p>
    ResourceNotFound(String),
    /// <p>The specified resource is not available.</p>
    ResourceUnavailable(String),
}

impl CopyWorkspaceImageError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CopyWorkspaceImageError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(CopyWorkspaceImageError::AccessDenied(err.msg))
                }
                "InvalidParameterValuesException" => {
                    return RusotoError::Service(CopyWorkspaceImageError::InvalidParameterValues(
                        err.msg,
                    ))
                }
                "OperationNotSupportedException" => {
                    return RusotoError::Service(CopyWorkspaceImageError::OperationNotSupported(
                        err.msg,
                    ))
                }
                "ResourceAlreadyExistsException" => {
                    return RusotoError::Service(CopyWorkspaceImageError::ResourceAlreadyExists(
                        err.msg,
                    ))
                }
                "ResourceLimitExceededException" => {
                    return RusotoError::Service(CopyWorkspaceImageError::ResourceLimitExceeded(
                        err.msg,
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(CopyWorkspaceImageError::ResourceNotFound(err.msg))
                }
                "ResourceUnavailableException" => {
                    return RusotoError::Service(CopyWorkspaceImageError::ResourceUnavailable(
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
impl fmt::Display for CopyWorkspaceImageError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CopyWorkspaceImageError::AccessDenied(ref cause) => write!(f, "{}", cause),
            CopyWorkspaceImageError::InvalidParameterValues(ref cause) => write!(f, "{}", cause),
            CopyWorkspaceImageError::OperationNotSupported(ref cause) => write!(f, "{}", cause),
            CopyWorkspaceImageError::ResourceAlreadyExists(ref cause) => write!(f, "{}", cause),
            CopyWorkspaceImageError::ResourceLimitExceeded(ref cause) => write!(f, "{}", cause),
            CopyWorkspaceImageError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            CopyWorkspaceImageError::ResourceUnavailable(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CopyWorkspaceImageError {}
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
}

impl CreateIpGroupError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateIpGroupError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(CreateIpGroupError::AccessDenied(err.msg))
                }
                "InvalidParameterValuesException" => {
                    return RusotoError::Service(CreateIpGroupError::InvalidParameterValues(
                        err.msg,
                    ))
                }
                "ResourceAlreadyExistsException" => {
                    return RusotoError::Service(CreateIpGroupError::ResourceAlreadyExists(err.msg))
                }
                "ResourceCreationFailedException" => {
                    return RusotoError::Service(CreateIpGroupError::ResourceCreationFailed(
                        err.msg,
                    ))
                }
                "ResourceLimitExceededException" => {
                    return RusotoError::Service(CreateIpGroupError::ResourceLimitExceeded(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for CreateIpGroupError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreateIpGroupError::AccessDenied(ref cause) => write!(f, "{}", cause),
            CreateIpGroupError::InvalidParameterValues(ref cause) => write!(f, "{}", cause),
            CreateIpGroupError::ResourceAlreadyExists(ref cause) => write!(f, "{}", cause),
            CreateIpGroupError::ResourceCreationFailed(ref cause) => write!(f, "{}", cause),
            CreateIpGroupError::ResourceLimitExceeded(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CreateIpGroupError {}
/// Errors returned by CreateTags
#[derive(Debug, PartialEq)]
pub enum CreateTagsError {
    /// <p>One or more parameter values are not valid.</p>
    InvalidParameterValues(String),
    /// <p>Your resource limits have been exceeded.</p>
    ResourceLimitExceeded(String),
    /// <p>The resource could not be found.</p>
    ResourceNotFound(String),
}

impl CreateTagsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateTagsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidParameterValuesException" => {
                    return RusotoError::Service(CreateTagsError::InvalidParameterValues(err.msg))
                }
                "ResourceLimitExceededException" => {
                    return RusotoError::Service(CreateTagsError::ResourceLimitExceeded(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(CreateTagsError::ResourceNotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for CreateTagsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreateTagsError::InvalidParameterValues(ref cause) => write!(f, "{}", cause),
            CreateTagsError::ResourceLimitExceeded(ref cause) => write!(f, "{}", cause),
            CreateTagsError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CreateTagsError {}
/// Errors returned by CreateWorkspaces
#[derive(Debug, PartialEq)]
pub enum CreateWorkspacesError {
    /// <p>One or more parameter values are not valid.</p>
    InvalidParameterValues(String),
    /// <p>Your resource limits have been exceeded.</p>
    ResourceLimitExceeded(String),
}

impl CreateWorkspacesError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateWorkspacesError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidParameterValuesException" => {
                    return RusotoError::Service(CreateWorkspacesError::InvalidParameterValues(
                        err.msg,
                    ))
                }
                "ResourceLimitExceededException" => {
                    return RusotoError::Service(CreateWorkspacesError::ResourceLimitExceeded(
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
impl fmt::Display for CreateWorkspacesError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreateWorkspacesError::InvalidParameterValues(ref cause) => write!(f, "{}", cause),
            CreateWorkspacesError::ResourceLimitExceeded(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CreateWorkspacesError {}
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
}

impl DeleteIpGroupError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteIpGroupError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(DeleteIpGroupError::AccessDenied(err.msg))
                }
                "InvalidParameterValuesException" => {
                    return RusotoError::Service(DeleteIpGroupError::InvalidParameterValues(
                        err.msg,
                    ))
                }
                "ResourceAssociatedException" => {
                    return RusotoError::Service(DeleteIpGroupError::ResourceAssociated(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(DeleteIpGroupError::ResourceNotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DeleteIpGroupError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteIpGroupError::AccessDenied(ref cause) => write!(f, "{}", cause),
            DeleteIpGroupError::InvalidParameterValues(ref cause) => write!(f, "{}", cause),
            DeleteIpGroupError::ResourceAssociated(ref cause) => write!(f, "{}", cause),
            DeleteIpGroupError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeleteIpGroupError {}
/// Errors returned by DeleteTags
#[derive(Debug, PartialEq)]
pub enum DeleteTagsError {
    /// <p>One or more parameter values are not valid.</p>
    InvalidParameterValues(String),
    /// <p>The resource could not be found.</p>
    ResourceNotFound(String),
}

impl DeleteTagsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteTagsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidParameterValuesException" => {
                    return RusotoError::Service(DeleteTagsError::InvalidParameterValues(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(DeleteTagsError::ResourceNotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DeleteTagsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteTagsError::InvalidParameterValues(ref cause) => write!(f, "{}", cause),
            DeleteTagsError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeleteTagsError {}
/// Errors returned by DeleteWorkspaceImage
#[derive(Debug, PartialEq)]
pub enum DeleteWorkspaceImageError {
    /// <p>The user is not authorized to access a resource.</p>
    AccessDenied(String),
    /// <p>The state of the resource is not valid for this operation.</p>
    InvalidResourceState(String),
    /// <p>The resource is associated with a directory.</p>
    ResourceAssociated(String),
}

impl DeleteWorkspaceImageError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteWorkspaceImageError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(DeleteWorkspaceImageError::AccessDenied(err.msg))
                }
                "InvalidResourceStateException" => {
                    return RusotoError::Service(DeleteWorkspaceImageError::InvalidResourceState(
                        err.msg,
                    ))
                }
                "ResourceAssociatedException" => {
                    return RusotoError::Service(DeleteWorkspaceImageError::ResourceAssociated(
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
impl fmt::Display for DeleteWorkspaceImageError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteWorkspaceImageError::AccessDenied(ref cause) => write!(f, "{}", cause),
            DeleteWorkspaceImageError::InvalidResourceState(ref cause) => write!(f, "{}", cause),
            DeleteWorkspaceImageError::ResourceAssociated(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeleteWorkspaceImageError {}
/// Errors returned by DeregisterWorkspaceDirectory
#[derive(Debug, PartialEq)]
pub enum DeregisterWorkspaceDirectoryError {
    /// <p>The user is not authorized to access a resource.</p>
    AccessDenied(String),
    /// <p>One or more parameter values are not valid.</p>
    InvalidParameterValues(String),
    /// <p>The state of the resource is not valid for this operation.</p>
    InvalidResourceState(String),
    /// <p>This operation is not supported.</p>
    OperationNotSupported(String),
    /// <p>The resource could not be found.</p>
    ResourceNotFound(String),
}

impl DeregisterWorkspaceDirectoryError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<DeregisterWorkspaceDirectoryError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(DeregisterWorkspaceDirectoryError::AccessDenied(
                        err.msg,
                    ))
                }
                "InvalidParameterValuesException" => {
                    return RusotoError::Service(
                        DeregisterWorkspaceDirectoryError::InvalidParameterValues(err.msg),
                    )
                }
                "InvalidResourceStateException" => {
                    return RusotoError::Service(
                        DeregisterWorkspaceDirectoryError::InvalidResourceState(err.msg),
                    )
                }
                "OperationNotSupportedException" => {
                    return RusotoError::Service(
                        DeregisterWorkspaceDirectoryError::OperationNotSupported(err.msg),
                    )
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(
                        DeregisterWorkspaceDirectoryError::ResourceNotFound(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DeregisterWorkspaceDirectoryError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeregisterWorkspaceDirectoryError::AccessDenied(ref cause) => write!(f, "{}", cause),
            DeregisterWorkspaceDirectoryError::InvalidParameterValues(ref cause) => {
                write!(f, "{}", cause)
            }
            DeregisterWorkspaceDirectoryError::InvalidResourceState(ref cause) => {
                write!(f, "{}", cause)
            }
            DeregisterWorkspaceDirectoryError::OperationNotSupported(ref cause) => {
                write!(f, "{}", cause)
            }
            DeregisterWorkspaceDirectoryError::ResourceNotFound(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for DeregisterWorkspaceDirectoryError {}
/// Errors returned by DescribeAccount
#[derive(Debug, PartialEq)]
pub enum DescribeAccountError {
    /// <p>The user is not authorized to access a resource.</p>
    AccessDenied(String),
}

impl DescribeAccountError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeAccountError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(DescribeAccountError::AccessDenied(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DescribeAccountError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeAccountError::AccessDenied(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DescribeAccountError {}
/// Errors returned by DescribeAccountModifications
#[derive(Debug, PartialEq)]
pub enum DescribeAccountModificationsError {
    /// <p>The user is not authorized to access a resource.</p>
    AccessDenied(String),
}

impl DescribeAccountModificationsError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<DescribeAccountModificationsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(DescribeAccountModificationsError::AccessDenied(
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
impl fmt::Display for DescribeAccountModificationsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeAccountModificationsError::AccessDenied(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DescribeAccountModificationsError {}
/// Errors returned by DescribeClientProperties
#[derive(Debug, PartialEq)]
pub enum DescribeClientPropertiesError {
    /// <p>The user is not authorized to access a resource.</p>
    AccessDenied(String),
    /// <p>One or more parameter values are not valid.</p>
    InvalidParameterValues(String),
    /// <p>The resource could not be found.</p>
    ResourceNotFound(String),
}

impl DescribeClientPropertiesError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeClientPropertiesError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(DescribeClientPropertiesError::AccessDenied(
                        err.msg,
                    ))
                }
                "InvalidParameterValuesException" => {
                    return RusotoError::Service(
                        DescribeClientPropertiesError::InvalidParameterValues(err.msg),
                    )
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(DescribeClientPropertiesError::ResourceNotFound(
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
impl fmt::Display for DescribeClientPropertiesError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeClientPropertiesError::AccessDenied(ref cause) => write!(f, "{}", cause),
            DescribeClientPropertiesError::InvalidParameterValues(ref cause) => {
                write!(f, "{}", cause)
            }
            DescribeClientPropertiesError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DescribeClientPropertiesError {}
/// Errors returned by DescribeIpGroups
#[derive(Debug, PartialEq)]
pub enum DescribeIpGroupsError {
    /// <p>The user is not authorized to access a resource.</p>
    AccessDenied(String),
    /// <p>One or more parameter values are not valid.</p>
    InvalidParameterValues(String),
}

impl DescribeIpGroupsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeIpGroupsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(DescribeIpGroupsError::AccessDenied(err.msg))
                }
                "InvalidParameterValuesException" => {
                    return RusotoError::Service(DescribeIpGroupsError::InvalidParameterValues(
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
impl fmt::Display for DescribeIpGroupsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeIpGroupsError::AccessDenied(ref cause) => write!(f, "{}", cause),
            DescribeIpGroupsError::InvalidParameterValues(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DescribeIpGroupsError {}
/// Errors returned by DescribeTags
#[derive(Debug, PartialEq)]
pub enum DescribeTagsError {
    /// <p>The resource could not be found.</p>
    ResourceNotFound(String),
}

impl DescribeTagsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeTagsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ResourceNotFoundException" => {
                    return RusotoError::Service(DescribeTagsError::ResourceNotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DescribeTagsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeTagsError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DescribeTagsError {}
/// Errors returned by DescribeWorkspaceBundles
#[derive(Debug, PartialEq)]
pub enum DescribeWorkspaceBundlesError {
    /// <p>One or more parameter values are not valid.</p>
    InvalidParameterValues(String),
}

impl DescribeWorkspaceBundlesError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeWorkspaceBundlesError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidParameterValuesException" => {
                    return RusotoError::Service(
                        DescribeWorkspaceBundlesError::InvalidParameterValues(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DescribeWorkspaceBundlesError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeWorkspaceBundlesError::InvalidParameterValues(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for DescribeWorkspaceBundlesError {}
/// Errors returned by DescribeWorkspaceDirectories
#[derive(Debug, PartialEq)]
pub enum DescribeWorkspaceDirectoriesError {
    /// <p>One or more parameter values are not valid.</p>
    InvalidParameterValues(String),
}

impl DescribeWorkspaceDirectoriesError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<DescribeWorkspaceDirectoriesError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidParameterValuesException" => {
                    return RusotoError::Service(
                        DescribeWorkspaceDirectoriesError::InvalidParameterValues(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DescribeWorkspaceDirectoriesError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeWorkspaceDirectoriesError::InvalidParameterValues(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for DescribeWorkspaceDirectoriesError {}
/// Errors returned by DescribeWorkspaceImages
#[derive(Debug, PartialEq)]
pub enum DescribeWorkspaceImagesError {
    /// <p>The user is not authorized to access a resource.</p>
    AccessDenied(String),
}

impl DescribeWorkspaceImagesError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeWorkspaceImagesError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(DescribeWorkspaceImagesError::AccessDenied(
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
impl fmt::Display for DescribeWorkspaceImagesError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeWorkspaceImagesError::AccessDenied(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DescribeWorkspaceImagesError {}
/// Errors returned by DescribeWorkspaceSnapshots
#[derive(Debug, PartialEq)]
pub enum DescribeWorkspaceSnapshotsError {
    /// <p>The user is not authorized to access a resource.</p>
    AccessDenied(String),
    /// <p>One or more parameter values are not valid.</p>
    InvalidParameterValues(String),
    /// <p>The resource could not be found.</p>
    ResourceNotFound(String),
}

impl DescribeWorkspaceSnapshotsError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<DescribeWorkspaceSnapshotsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(DescribeWorkspaceSnapshotsError::AccessDenied(
                        err.msg,
                    ))
                }
                "InvalidParameterValuesException" => {
                    return RusotoError::Service(
                        DescribeWorkspaceSnapshotsError::InvalidParameterValues(err.msg),
                    )
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(DescribeWorkspaceSnapshotsError::ResourceNotFound(
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
impl fmt::Display for DescribeWorkspaceSnapshotsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeWorkspaceSnapshotsError::AccessDenied(ref cause) => write!(f, "{}", cause),
            DescribeWorkspaceSnapshotsError::InvalidParameterValues(ref cause) => {
                write!(f, "{}", cause)
            }
            DescribeWorkspaceSnapshotsError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DescribeWorkspaceSnapshotsError {}
/// Errors returned by DescribeWorkspaces
#[derive(Debug, PartialEq)]
pub enum DescribeWorkspacesError {
    /// <p>One or more parameter values are not valid.</p>
    InvalidParameterValues(String),
    /// <p>The specified resource is not available.</p>
    ResourceUnavailable(String),
}

impl DescribeWorkspacesError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeWorkspacesError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidParameterValuesException" => {
                    return RusotoError::Service(DescribeWorkspacesError::InvalidParameterValues(
                        err.msg,
                    ))
                }
                "ResourceUnavailableException" => {
                    return RusotoError::Service(DescribeWorkspacesError::ResourceUnavailable(
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
impl fmt::Display for DescribeWorkspacesError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeWorkspacesError::InvalidParameterValues(ref cause) => write!(f, "{}", cause),
            DescribeWorkspacesError::ResourceUnavailable(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DescribeWorkspacesError {}
/// Errors returned by DescribeWorkspacesConnectionStatus
#[derive(Debug, PartialEq)]
pub enum DescribeWorkspacesConnectionStatusError {
    /// <p>One or more parameter values are not valid.</p>
    InvalidParameterValues(String),
}

impl DescribeWorkspacesConnectionStatusError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<DescribeWorkspacesConnectionStatusError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidParameterValuesException" => {
                    return RusotoError::Service(
                        DescribeWorkspacesConnectionStatusError::InvalidParameterValues(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DescribeWorkspacesConnectionStatusError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeWorkspacesConnectionStatusError::InvalidParameterValues(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for DescribeWorkspacesConnectionStatusError {}
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
}

impl DisassociateIpGroupsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DisassociateIpGroupsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(DisassociateIpGroupsError::AccessDenied(err.msg))
                }
                "InvalidParameterValuesException" => {
                    return RusotoError::Service(DisassociateIpGroupsError::InvalidParameterValues(
                        err.msg,
                    ))
                }
                "InvalidResourceStateException" => {
                    return RusotoError::Service(DisassociateIpGroupsError::InvalidResourceState(
                        err.msg,
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(DisassociateIpGroupsError::ResourceNotFound(
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
impl fmt::Display for DisassociateIpGroupsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DisassociateIpGroupsError::AccessDenied(ref cause) => write!(f, "{}", cause),
            DisassociateIpGroupsError::InvalidParameterValues(ref cause) => write!(f, "{}", cause),
            DisassociateIpGroupsError::InvalidResourceState(ref cause) => write!(f, "{}", cause),
            DisassociateIpGroupsError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DisassociateIpGroupsError {}
/// Errors returned by ImportWorkspaceImage
#[derive(Debug, PartialEq)]
pub enum ImportWorkspaceImageError {
    /// <p>The user is not authorized to access a resource.</p>
    AccessDenied(String),
    /// <p>One or more parameter values are not valid.</p>
    InvalidParameterValues(String),
    /// <p>This operation is not supported.</p>
    OperationNotSupported(String),
    /// <p>The specified resource already exists.</p>
    ResourceAlreadyExists(String),
    /// <p>Your resource limits have been exceeded.</p>
    ResourceLimitExceeded(String),
    /// <p>The resource could not be found.</p>
    ResourceNotFound(String),
}

impl ImportWorkspaceImageError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ImportWorkspaceImageError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(ImportWorkspaceImageError::AccessDenied(err.msg))
                }
                "InvalidParameterValuesException" => {
                    return RusotoError::Service(ImportWorkspaceImageError::InvalidParameterValues(
                        err.msg,
                    ))
                }
                "OperationNotSupportedException" => {
                    return RusotoError::Service(ImportWorkspaceImageError::OperationNotSupported(
                        err.msg,
                    ))
                }
                "ResourceAlreadyExistsException" => {
                    return RusotoError::Service(ImportWorkspaceImageError::ResourceAlreadyExists(
                        err.msg,
                    ))
                }
                "ResourceLimitExceededException" => {
                    return RusotoError::Service(ImportWorkspaceImageError::ResourceLimitExceeded(
                        err.msg,
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(ImportWorkspaceImageError::ResourceNotFound(
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
impl fmt::Display for ImportWorkspaceImageError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ImportWorkspaceImageError::AccessDenied(ref cause) => write!(f, "{}", cause),
            ImportWorkspaceImageError::InvalidParameterValues(ref cause) => write!(f, "{}", cause),
            ImportWorkspaceImageError::OperationNotSupported(ref cause) => write!(f, "{}", cause),
            ImportWorkspaceImageError::ResourceAlreadyExists(ref cause) => write!(f, "{}", cause),
            ImportWorkspaceImageError::ResourceLimitExceeded(ref cause) => write!(f, "{}", cause),
            ImportWorkspaceImageError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ImportWorkspaceImageError {}
/// Errors returned by ListAvailableManagementCidrRanges
#[derive(Debug, PartialEq)]
pub enum ListAvailableManagementCidrRangesError {
    /// <p>The user is not authorized to access a resource.</p>
    AccessDenied(String),
    /// <p>One or more parameter values are not valid.</p>
    InvalidParameterValues(String),
}

impl ListAvailableManagementCidrRangesError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<ListAvailableManagementCidrRangesError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(
                        ListAvailableManagementCidrRangesError::AccessDenied(err.msg),
                    )
                }
                "InvalidParameterValuesException" => {
                    return RusotoError::Service(
                        ListAvailableManagementCidrRangesError::InvalidParameterValues(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ListAvailableManagementCidrRangesError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListAvailableManagementCidrRangesError::AccessDenied(ref cause) => {
                write!(f, "{}", cause)
            }
            ListAvailableManagementCidrRangesError::InvalidParameterValues(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for ListAvailableManagementCidrRangesError {}
/// Errors returned by MigrateWorkspace
#[derive(Debug, PartialEq)]
pub enum MigrateWorkspaceError {
    /// <p>The user is not authorized to access a resource.</p>
    AccessDenied(String),
    /// <p>One or more parameter values are not valid.</p>
    InvalidParameterValues(String),
    /// <p>The properties of this WorkSpace are currently being modified. Try again in a moment.</p>
    OperationInProgress(String),
    /// <p>This operation is not supported.</p>
    OperationNotSupported(String),
    /// <p>The resource could not be found.</p>
    ResourceNotFound(String),
    /// <p>The specified resource is not available.</p>
    ResourceUnavailable(String),
}

impl MigrateWorkspaceError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<MigrateWorkspaceError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(MigrateWorkspaceError::AccessDenied(err.msg))
                }
                "InvalidParameterValuesException" => {
                    return RusotoError::Service(MigrateWorkspaceError::InvalidParameterValues(
                        err.msg,
                    ))
                }
                "OperationInProgressException" => {
                    return RusotoError::Service(MigrateWorkspaceError::OperationInProgress(
                        err.msg,
                    ))
                }
                "OperationNotSupportedException" => {
                    return RusotoError::Service(MigrateWorkspaceError::OperationNotSupported(
                        err.msg,
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(MigrateWorkspaceError::ResourceNotFound(err.msg))
                }
                "ResourceUnavailableException" => {
                    return RusotoError::Service(MigrateWorkspaceError::ResourceUnavailable(
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
impl fmt::Display for MigrateWorkspaceError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            MigrateWorkspaceError::AccessDenied(ref cause) => write!(f, "{}", cause),
            MigrateWorkspaceError::InvalidParameterValues(ref cause) => write!(f, "{}", cause),
            MigrateWorkspaceError::OperationInProgress(ref cause) => write!(f, "{}", cause),
            MigrateWorkspaceError::OperationNotSupported(ref cause) => write!(f, "{}", cause),
            MigrateWorkspaceError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            MigrateWorkspaceError::ResourceUnavailable(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for MigrateWorkspaceError {}
/// Errors returned by ModifyAccount
#[derive(Debug, PartialEq)]
pub enum ModifyAccountError {
    /// <p>The user is not authorized to access a resource.</p>
    AccessDenied(String),
    /// <p>One or more parameter values are not valid.</p>
    InvalidParameterValues(String),
    /// <p>The state of the resource is not valid for this operation.</p>
    InvalidResourceState(String),
    /// <p>The resource could not be found.</p>
    ResourceNotFound(String),
    /// <p>The specified resource is not available.</p>
    ResourceUnavailable(String),
}

impl ModifyAccountError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ModifyAccountError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(ModifyAccountError::AccessDenied(err.msg))
                }
                "InvalidParameterValuesException" => {
                    return RusotoError::Service(ModifyAccountError::InvalidParameterValues(
                        err.msg,
                    ))
                }
                "InvalidResourceStateException" => {
                    return RusotoError::Service(ModifyAccountError::InvalidResourceState(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(ModifyAccountError::ResourceNotFound(err.msg))
                }
                "ResourceUnavailableException" => {
                    return RusotoError::Service(ModifyAccountError::ResourceUnavailable(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ModifyAccountError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ModifyAccountError::AccessDenied(ref cause) => write!(f, "{}", cause),
            ModifyAccountError::InvalidParameterValues(ref cause) => write!(f, "{}", cause),
            ModifyAccountError::InvalidResourceState(ref cause) => write!(f, "{}", cause),
            ModifyAccountError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            ModifyAccountError::ResourceUnavailable(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ModifyAccountError {}
/// Errors returned by ModifyClientProperties
#[derive(Debug, PartialEq)]
pub enum ModifyClientPropertiesError {
    /// <p>The user is not authorized to access a resource.</p>
    AccessDenied(String),
    /// <p>One or more parameter values are not valid.</p>
    InvalidParameterValues(String),
    /// <p>The resource could not be found.</p>
    ResourceNotFound(String),
}

impl ModifyClientPropertiesError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ModifyClientPropertiesError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(ModifyClientPropertiesError::AccessDenied(err.msg))
                }
                "InvalidParameterValuesException" => {
                    return RusotoError::Service(
                        ModifyClientPropertiesError::InvalidParameterValues(err.msg),
                    )
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(ModifyClientPropertiesError::ResourceNotFound(
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
impl fmt::Display for ModifyClientPropertiesError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ModifyClientPropertiesError::AccessDenied(ref cause) => write!(f, "{}", cause),
            ModifyClientPropertiesError::InvalidParameterValues(ref cause) => {
                write!(f, "{}", cause)
            }
            ModifyClientPropertiesError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ModifyClientPropertiesError {}
/// Errors returned by ModifySelfservicePermissions
#[derive(Debug, PartialEq)]
pub enum ModifySelfservicePermissionsError {
    /// <p>The user is not authorized to access a resource.</p>
    AccessDenied(String),
    /// <p>One or more parameter values are not valid.</p>
    InvalidParameterValues(String),
    /// <p>The resource could not be found.</p>
    ResourceNotFound(String),
}

impl ModifySelfservicePermissionsError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<ModifySelfservicePermissionsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(ModifySelfservicePermissionsError::AccessDenied(
                        err.msg,
                    ))
                }
                "InvalidParameterValuesException" => {
                    return RusotoError::Service(
                        ModifySelfservicePermissionsError::InvalidParameterValues(err.msg),
                    )
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(
                        ModifySelfservicePermissionsError::ResourceNotFound(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ModifySelfservicePermissionsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ModifySelfservicePermissionsError::AccessDenied(ref cause) => write!(f, "{}", cause),
            ModifySelfservicePermissionsError::InvalidParameterValues(ref cause) => {
                write!(f, "{}", cause)
            }
            ModifySelfservicePermissionsError::ResourceNotFound(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for ModifySelfservicePermissionsError {}
/// Errors returned by ModifyWorkspaceAccessProperties
#[derive(Debug, PartialEq)]
pub enum ModifyWorkspaceAccessPropertiesError {
    /// <p>The user is not authorized to access a resource.</p>
    AccessDenied(String),
    /// <p>The resource could not be found.</p>
    ResourceNotFound(String),
}

impl ModifyWorkspaceAccessPropertiesError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<ModifyWorkspaceAccessPropertiesError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(
                        ModifyWorkspaceAccessPropertiesError::AccessDenied(err.msg),
                    )
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(
                        ModifyWorkspaceAccessPropertiesError::ResourceNotFound(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ModifyWorkspaceAccessPropertiesError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ModifyWorkspaceAccessPropertiesError::AccessDenied(ref cause) => write!(f, "{}", cause),
            ModifyWorkspaceAccessPropertiesError::ResourceNotFound(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for ModifyWorkspaceAccessPropertiesError {}
/// Errors returned by ModifyWorkspaceCreationProperties
#[derive(Debug, PartialEq)]
pub enum ModifyWorkspaceCreationPropertiesError {
    /// <p>The user is not authorized to access a resource.</p>
    AccessDenied(String),
    /// <p>One or more parameter values are not valid.</p>
    InvalidParameterValues(String),
    /// <p>The resource could not be found.</p>
    ResourceNotFound(String),
}

impl ModifyWorkspaceCreationPropertiesError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<ModifyWorkspaceCreationPropertiesError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(
                        ModifyWorkspaceCreationPropertiesError::AccessDenied(err.msg),
                    )
                }
                "InvalidParameterValuesException" => {
                    return RusotoError::Service(
                        ModifyWorkspaceCreationPropertiesError::InvalidParameterValues(err.msg),
                    )
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(
                        ModifyWorkspaceCreationPropertiesError::ResourceNotFound(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ModifyWorkspaceCreationPropertiesError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ModifyWorkspaceCreationPropertiesError::AccessDenied(ref cause) => {
                write!(f, "{}", cause)
            }
            ModifyWorkspaceCreationPropertiesError::InvalidParameterValues(ref cause) => {
                write!(f, "{}", cause)
            }
            ModifyWorkspaceCreationPropertiesError::ResourceNotFound(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for ModifyWorkspaceCreationPropertiesError {}
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
    /// <p>The configuration of this WorkSpace is not supported for this operation. For more information, see <a href="https://docs.aws.amazon.com/workspaces/latest/adminguide/required-service-components.html">Required Configuration and Service Components for WorkSpaces </a>.</p>
    UnsupportedWorkspaceConfiguration(String),
}

impl ModifyWorkspacePropertiesError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ModifyWorkspacePropertiesError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(ModifyWorkspacePropertiesError::AccessDenied(
                        err.msg,
                    ))
                }
                "InvalidParameterValuesException" => {
                    return RusotoError::Service(
                        ModifyWorkspacePropertiesError::InvalidParameterValues(err.msg),
                    )
                }
                "InvalidResourceStateException" => {
                    return RusotoError::Service(
                        ModifyWorkspacePropertiesError::InvalidResourceState(err.msg),
                    )
                }
                "OperationInProgressException" => {
                    return RusotoError::Service(
                        ModifyWorkspacePropertiesError::OperationInProgress(err.msg),
                    )
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(ModifyWorkspacePropertiesError::ResourceNotFound(
                        err.msg,
                    ))
                }
                "ResourceUnavailableException" => {
                    return RusotoError::Service(
                        ModifyWorkspacePropertiesError::ResourceUnavailable(err.msg),
                    )
                }
                "UnsupportedWorkspaceConfigurationException" => {
                    return RusotoError::Service(
                        ModifyWorkspacePropertiesError::UnsupportedWorkspaceConfiguration(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ModifyWorkspacePropertiesError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ModifyWorkspacePropertiesError::AccessDenied(ref cause) => write!(f, "{}", cause),
            ModifyWorkspacePropertiesError::InvalidParameterValues(ref cause) => {
                write!(f, "{}", cause)
            }
            ModifyWorkspacePropertiesError::InvalidResourceState(ref cause) => {
                write!(f, "{}", cause)
            }
            ModifyWorkspacePropertiesError::OperationInProgress(ref cause) => {
                write!(f, "{}", cause)
            }
            ModifyWorkspacePropertiesError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            ModifyWorkspacePropertiesError::ResourceUnavailable(ref cause) => {
                write!(f, "{}", cause)
            }
            ModifyWorkspacePropertiesError::UnsupportedWorkspaceConfiguration(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for ModifyWorkspacePropertiesError {}
/// Errors returned by ModifyWorkspaceState
#[derive(Debug, PartialEq)]
pub enum ModifyWorkspaceStateError {
    /// <p>One or more parameter values are not valid.</p>
    InvalidParameterValues(String),
    /// <p>The state of the resource is not valid for this operation.</p>
    InvalidResourceState(String),
    /// <p>The resource could not be found.</p>
    ResourceNotFound(String),
}

impl ModifyWorkspaceStateError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ModifyWorkspaceStateError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidParameterValuesException" => {
                    return RusotoError::Service(ModifyWorkspaceStateError::InvalidParameterValues(
                        err.msg,
                    ))
                }
                "InvalidResourceStateException" => {
                    return RusotoError::Service(ModifyWorkspaceStateError::InvalidResourceState(
                        err.msg,
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(ModifyWorkspaceStateError::ResourceNotFound(
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
impl fmt::Display for ModifyWorkspaceStateError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ModifyWorkspaceStateError::InvalidParameterValues(ref cause) => write!(f, "{}", cause),
            ModifyWorkspaceStateError::InvalidResourceState(ref cause) => write!(f, "{}", cause),
            ModifyWorkspaceStateError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ModifyWorkspaceStateError {}
/// Errors returned by RebootWorkspaces
#[derive(Debug, PartialEq)]
pub enum RebootWorkspacesError {}

impl RebootWorkspacesError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<RebootWorkspacesError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for RebootWorkspacesError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {}
    }
}
impl Error for RebootWorkspacesError {}
/// Errors returned by RebuildWorkspaces
#[derive(Debug, PartialEq)]
pub enum RebuildWorkspacesError {}

impl RebuildWorkspacesError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<RebuildWorkspacesError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for RebuildWorkspacesError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {}
    }
}
impl Error for RebuildWorkspacesError {}
/// Errors returned by RegisterWorkspaceDirectory
#[derive(Debug, PartialEq)]
pub enum RegisterWorkspaceDirectoryError {
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
    /// <p>The configuration of this network is not supported for this operation, or your network configuration conflicts with the Amazon WorkSpaces management network IP range. For more information, see <a href="https://docs.aws.amazon.com/workspaces/latest/adminguide/amazon-workspaces-vpc.html"> Configure a VPC for Amazon WorkSpaces</a>.</p>
    UnsupportedNetworkConfiguration(String),
    /// <p>The workspaces_DefaultRole role could not be found. If this is the first time you are registering a directory, you will need to create the workspaces_DefaultRole role before you can register a directory. For more information, see <a href="https://docs.aws.amazon.com/workspaces/latest/adminguide/workspaces-access-control.html#create-default-role">Creating the workspaces_DefaultRole Role</a>.</p>
    WorkspacesDefaultRoleNotFound(String),
}

impl RegisterWorkspaceDirectoryError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<RegisterWorkspaceDirectoryError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(RegisterWorkspaceDirectoryError::AccessDenied(
                        err.msg,
                    ))
                }
                "InvalidParameterValuesException" => {
                    return RusotoError::Service(
                        RegisterWorkspaceDirectoryError::InvalidParameterValues(err.msg),
                    )
                }
                "InvalidResourceStateException" => {
                    return RusotoError::Service(
                        RegisterWorkspaceDirectoryError::InvalidResourceState(err.msg),
                    )
                }
                "OperationNotSupportedException" => {
                    return RusotoError::Service(
                        RegisterWorkspaceDirectoryError::OperationNotSupported(err.msg),
                    )
                }
                "ResourceLimitExceededException" => {
                    return RusotoError::Service(
                        RegisterWorkspaceDirectoryError::ResourceLimitExceeded(err.msg),
                    )
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(RegisterWorkspaceDirectoryError::ResourceNotFound(
                        err.msg,
                    ))
                }
                "UnsupportedNetworkConfigurationException" => {
                    return RusotoError::Service(
                        RegisterWorkspaceDirectoryError::UnsupportedNetworkConfiguration(err.msg),
                    )
                }
                "WorkspacesDefaultRoleNotFoundException" => {
                    return RusotoError::Service(
                        RegisterWorkspaceDirectoryError::WorkspacesDefaultRoleNotFound(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for RegisterWorkspaceDirectoryError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            RegisterWorkspaceDirectoryError::AccessDenied(ref cause) => write!(f, "{}", cause),
            RegisterWorkspaceDirectoryError::InvalidParameterValues(ref cause) => {
                write!(f, "{}", cause)
            }
            RegisterWorkspaceDirectoryError::InvalidResourceState(ref cause) => {
                write!(f, "{}", cause)
            }
            RegisterWorkspaceDirectoryError::OperationNotSupported(ref cause) => {
                write!(f, "{}", cause)
            }
            RegisterWorkspaceDirectoryError::ResourceLimitExceeded(ref cause) => {
                write!(f, "{}", cause)
            }
            RegisterWorkspaceDirectoryError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            RegisterWorkspaceDirectoryError::UnsupportedNetworkConfiguration(ref cause) => {
                write!(f, "{}", cause)
            }
            RegisterWorkspaceDirectoryError::WorkspacesDefaultRoleNotFound(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for RegisterWorkspaceDirectoryError {}
/// Errors returned by RestoreWorkspace
#[derive(Debug, PartialEq)]
pub enum RestoreWorkspaceError {
    /// <p>The user is not authorized to access a resource.</p>
    AccessDenied(String),
    /// <p>One or more parameter values are not valid.</p>
    InvalidParameterValues(String),
    /// <p>The resource could not be found.</p>
    ResourceNotFound(String),
}

impl RestoreWorkspaceError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<RestoreWorkspaceError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(RestoreWorkspaceError::AccessDenied(err.msg))
                }
                "InvalidParameterValuesException" => {
                    return RusotoError::Service(RestoreWorkspaceError::InvalidParameterValues(
                        err.msg,
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(RestoreWorkspaceError::ResourceNotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for RestoreWorkspaceError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            RestoreWorkspaceError::AccessDenied(ref cause) => write!(f, "{}", cause),
            RestoreWorkspaceError::InvalidParameterValues(ref cause) => write!(f, "{}", cause),
            RestoreWorkspaceError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for RestoreWorkspaceError {}
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
}

impl RevokeIpRulesError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<RevokeIpRulesError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(RevokeIpRulesError::AccessDenied(err.msg))
                }
                "InvalidParameterValuesException" => {
                    return RusotoError::Service(RevokeIpRulesError::InvalidParameterValues(
                        err.msg,
                    ))
                }
                "InvalidResourceStateException" => {
                    return RusotoError::Service(RevokeIpRulesError::InvalidResourceState(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(RevokeIpRulesError::ResourceNotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for RevokeIpRulesError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            RevokeIpRulesError::AccessDenied(ref cause) => write!(f, "{}", cause),
            RevokeIpRulesError::InvalidParameterValues(ref cause) => write!(f, "{}", cause),
            RevokeIpRulesError::InvalidResourceState(ref cause) => write!(f, "{}", cause),
            RevokeIpRulesError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for RevokeIpRulesError {}
/// Errors returned by StartWorkspaces
#[derive(Debug, PartialEq)]
pub enum StartWorkspacesError {}

impl StartWorkspacesError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<StartWorkspacesError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for StartWorkspacesError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {}
    }
}
impl Error for StartWorkspacesError {}
/// Errors returned by StopWorkspaces
#[derive(Debug, PartialEq)]
pub enum StopWorkspacesError {}

impl StopWorkspacesError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<StopWorkspacesError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for StopWorkspacesError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {}
    }
}
impl Error for StopWorkspacesError {}
/// Errors returned by TerminateWorkspaces
#[derive(Debug, PartialEq)]
pub enum TerminateWorkspacesError {}

impl TerminateWorkspacesError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<TerminateWorkspacesError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for TerminateWorkspacesError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {}
    }
}
impl Error for TerminateWorkspacesError {}
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
}

impl UpdateRulesOfIpGroupError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdateRulesOfIpGroupError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(UpdateRulesOfIpGroupError::AccessDenied(err.msg))
                }
                "InvalidParameterValuesException" => {
                    return RusotoError::Service(UpdateRulesOfIpGroupError::InvalidParameterValues(
                        err.msg,
                    ))
                }
                "InvalidResourceStateException" => {
                    return RusotoError::Service(UpdateRulesOfIpGroupError::InvalidResourceState(
                        err.msg,
                    ))
                }
                "ResourceLimitExceededException" => {
                    return RusotoError::Service(UpdateRulesOfIpGroupError::ResourceLimitExceeded(
                        err.msg,
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(UpdateRulesOfIpGroupError::ResourceNotFound(
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
impl fmt::Display for UpdateRulesOfIpGroupError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UpdateRulesOfIpGroupError::AccessDenied(ref cause) => write!(f, "{}", cause),
            UpdateRulesOfIpGroupError::InvalidParameterValues(ref cause) => write!(f, "{}", cause),
            UpdateRulesOfIpGroupError::InvalidResourceState(ref cause) => write!(f, "{}", cause),
            UpdateRulesOfIpGroupError::ResourceLimitExceeded(ref cause) => write!(f, "{}", cause),
            UpdateRulesOfIpGroupError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for UpdateRulesOfIpGroupError {}
/// Trait representing the capabilities of the Amazon WorkSpaces API. Amazon WorkSpaces clients implement this trait.
#[async_trait]
pub trait Workspaces {
    /// <p>Associates the specified IP access control group with the specified directory.</p>
    async fn associate_ip_groups(
        &self,
        input: AssociateIpGroupsRequest,
    ) -> Result<AssociateIpGroupsResult, RusotoError<AssociateIpGroupsError>>;

    /// <p>Adds one or more rules to the specified IP access control group.</p> <p>This action gives users permission to access their WorkSpaces from the CIDR address ranges specified in the rules.</p>
    async fn authorize_ip_rules(
        &self,
        input: AuthorizeIpRulesRequest,
    ) -> Result<AuthorizeIpRulesResult, RusotoError<AuthorizeIpRulesError>>;

    /// <p>Copies the specified image from the specified Region to the current Region.</p>
    async fn copy_workspace_image(
        &self,
        input: CopyWorkspaceImageRequest,
    ) -> Result<CopyWorkspaceImageResult, RusotoError<CopyWorkspaceImageError>>;

    /// <p>Creates an IP access control group.</p> <p>An IP access control group provides you with the ability to control the IP addresses from which users are allowed to access their WorkSpaces. To specify the CIDR address ranges, add rules to your IP access control group and then associate the group with your directory. You can add rules when you create the group or at any time using <a>AuthorizeIpRules</a>.</p> <p>There is a default IP access control group associated with your directory. If you don't associate an IP access control group with your directory, the default group is used. The default group includes a default rule that allows users to access their WorkSpaces from anywhere. You cannot modify the default IP access control group for your directory.</p>
    async fn create_ip_group(
        &self,
        input: CreateIpGroupRequest,
    ) -> Result<CreateIpGroupResult, RusotoError<CreateIpGroupError>>;

    /// <p>Creates the specified tags for the specified WorkSpaces resource.</p>
    async fn create_tags(
        &self,
        input: CreateTagsRequest,
    ) -> Result<CreateTagsResult, RusotoError<CreateTagsError>>;

    /// <p>Creates one or more WorkSpaces.</p> <p>This operation is asynchronous and returns before the WorkSpaces are created.</p>
    async fn create_workspaces(
        &self,
        input: CreateWorkspacesRequest,
    ) -> Result<CreateWorkspacesResult, RusotoError<CreateWorkspacesError>>;

    /// <p>Deletes the specified IP access control group.</p> <p>You cannot delete an IP access control group that is associated with a directory.</p>
    async fn delete_ip_group(
        &self,
        input: DeleteIpGroupRequest,
    ) -> Result<DeleteIpGroupResult, RusotoError<DeleteIpGroupError>>;

    /// <p>Deletes the specified tags from the specified WorkSpaces resource.</p>
    async fn delete_tags(
        &self,
        input: DeleteTagsRequest,
    ) -> Result<DeleteTagsResult, RusotoError<DeleteTagsError>>;

    /// <p>Deletes the specified image from your account. To delete an image, you must first delete any bundles that are associated with the image and un-share the image if it is shared with other accounts. </p>
    async fn delete_workspace_image(
        &self,
        input: DeleteWorkspaceImageRequest,
    ) -> Result<DeleteWorkspaceImageResult, RusotoError<DeleteWorkspaceImageError>>;

    /// <p>Deregisters the specified directory. This operation is asynchronous and returns before the WorkSpace directory is deregistered. If any WorkSpaces are registered to this directory, you must remove them before you can deregister the directory.</p>
    async fn deregister_workspace_directory(
        &self,
        input: DeregisterWorkspaceDirectoryRequest,
    ) -> Result<DeregisterWorkspaceDirectoryResult, RusotoError<DeregisterWorkspaceDirectoryError>>;

    /// <p>Retrieves a list that describes the configuration of Bring Your Own License (BYOL) for the specified account.</p>
    async fn describe_account(
        &self,
    ) -> Result<DescribeAccountResult, RusotoError<DescribeAccountError>>;

    /// <p>Retrieves a list that describes modifications to the configuration of Bring Your Own License (BYOL) for the specified account.</p>
    async fn describe_account_modifications(
        &self,
        input: DescribeAccountModificationsRequest,
    ) -> Result<DescribeAccountModificationsResult, RusotoError<DescribeAccountModificationsError>>;

    /// <p>Retrieves a list that describes one or more specified Amazon WorkSpaces clients.</p>
    async fn describe_client_properties(
        &self,
        input: DescribeClientPropertiesRequest,
    ) -> Result<DescribeClientPropertiesResult, RusotoError<DescribeClientPropertiesError>>;

    /// <p>Describes one or more of your IP access control groups.</p>
    async fn describe_ip_groups(
        &self,
        input: DescribeIpGroupsRequest,
    ) -> Result<DescribeIpGroupsResult, RusotoError<DescribeIpGroupsError>>;

    /// <p>Describes the specified tags for the specified WorkSpaces resource.</p>
    async fn describe_tags(
        &self,
        input: DescribeTagsRequest,
    ) -> Result<DescribeTagsResult, RusotoError<DescribeTagsError>>;

    /// <p>Retrieves a list that describes the available WorkSpace bundles.</p> <p>You can filter the results using either bundle ID or owner, but not both.</p>
    async fn describe_workspace_bundles(
        &self,
        input: DescribeWorkspaceBundlesRequest,
    ) -> Result<DescribeWorkspaceBundlesResult, RusotoError<DescribeWorkspaceBundlesError>>;

    /// <p>Describes the available directories that are registered with Amazon WorkSpaces.</p>
    async fn describe_workspace_directories(
        &self,
        input: DescribeWorkspaceDirectoriesRequest,
    ) -> Result<DescribeWorkspaceDirectoriesResult, RusotoError<DescribeWorkspaceDirectoriesError>>;

    /// <p>Retrieves a list that describes one or more specified images, if the image identifiers are provided. Otherwise, all images in the account are described. </p>
    async fn describe_workspace_images(
        &self,
        input: DescribeWorkspaceImagesRequest,
    ) -> Result<DescribeWorkspaceImagesResult, RusotoError<DescribeWorkspaceImagesError>>;

    /// <p>Describes the snapshots for the specified WorkSpace.</p>
    async fn describe_workspace_snapshots(
        &self,
        input: DescribeWorkspaceSnapshotsRequest,
    ) -> Result<DescribeWorkspaceSnapshotsResult, RusotoError<DescribeWorkspaceSnapshotsError>>;

    /// <p>Describes the specified WorkSpaces.</p> <p>You can filter the results by using the bundle identifier, directory identifier, or owner, but you can specify only one filter at a time.</p>
    async fn describe_workspaces(
        &self,
        input: DescribeWorkspacesRequest,
    ) -> Result<DescribeWorkspacesResult, RusotoError<DescribeWorkspacesError>>;

    /// <p>Describes the connection status of the specified WorkSpaces.</p>
    async fn describe_workspaces_connection_status(
        &self,
        input: DescribeWorkspacesConnectionStatusRequest,
    ) -> Result<
        DescribeWorkspacesConnectionStatusResult,
        RusotoError<DescribeWorkspacesConnectionStatusError>,
    >;

    /// <p>Disassociates the specified IP access control group from the specified directory.</p>
    async fn disassociate_ip_groups(
        &self,
        input: DisassociateIpGroupsRequest,
    ) -> Result<DisassociateIpGroupsResult, RusotoError<DisassociateIpGroupsError>>;

    /// <p>Imports the specified Windows 7 or Windows 10 Bring Your Own License (BYOL) image into Amazon WorkSpaces. The image must be an already licensed EC2 image that is in your AWS account, and you must own the image. </p>
    async fn import_workspace_image(
        &self,
        input: ImportWorkspaceImageRequest,
    ) -> Result<ImportWorkspaceImageResult, RusotoError<ImportWorkspaceImageError>>;

    /// <p>Retrieves a list of IP address ranges, specified as IPv4 CIDR blocks, that you can use for the network management interface when you enable Bring Your Own License (BYOL). </p> <p>The management network interface is connected to a secure Amazon WorkSpaces management network. It is used for interactive streaming of the WorkSpace desktop to Amazon WorkSpaces clients, and to allow Amazon WorkSpaces to manage the WorkSpace.</p>
    async fn list_available_management_cidr_ranges(
        &self,
        input: ListAvailableManagementCidrRangesRequest,
    ) -> Result<
        ListAvailableManagementCidrRangesResult,
        RusotoError<ListAvailableManagementCidrRangesError>,
    >;

    /// <p>Migrates a WorkSpace from one operating system or bundle type to another, while retaining the data on the user volume.</p> <p>The migration process recreates the WorkSpace by using a new root volume from the target bundle image and the user volume from the last available snapshot of the original WorkSpace. During migration, the original <code>D:\Users\%USERNAME%</code> user profile folder is renamed to <code>D:\Users\%USERNAME%MMddyyTHHmmss%.NotMigrated</code>. A new <code>D:\Users\%USERNAME%\</code> folder is generated by the new OS. Certain files in the old user profile are moved to the new user profile.</p> <p>For available migration scenarios, details about what happens during migration, and best practices, see <a href="https://docs.aws.amazon.com/workspaces/latest/adminguide/migrate-workspaces.html">Migrate a WorkSpace</a>.</p>
    async fn migrate_workspace(
        &self,
        input: MigrateWorkspaceRequest,
    ) -> Result<MigrateWorkspaceResult, RusotoError<MigrateWorkspaceError>>;

    /// <p>Modifies the configuration of Bring Your Own License (BYOL) for the specified account.</p>
    async fn modify_account(
        &self,
        input: ModifyAccountRequest,
    ) -> Result<ModifyAccountResult, RusotoError<ModifyAccountError>>;

    /// <p>Modifies the properties of the specified Amazon WorkSpaces clients.</p>
    async fn modify_client_properties(
        &self,
        input: ModifyClientPropertiesRequest,
    ) -> Result<ModifyClientPropertiesResult, RusotoError<ModifyClientPropertiesError>>;

    /// <p>Modifies the self-service WorkSpace management capabilities for your users. For more information, see <a href="https://docs.aws.amazon.com/workspaces/latest/adminguide/enable-user-self-service-workspace-management.html">Enable Self-Service WorkSpace Management Capabilities for Your Users</a>.</p>
    async fn modify_selfservice_permissions(
        &self,
        input: ModifySelfservicePermissionsRequest,
    ) -> Result<ModifySelfservicePermissionsResult, RusotoError<ModifySelfservicePermissionsError>>;

    /// <p>Specifies which devices and operating systems users can use to access their WorkSpaces. For more information, see <a href="https://docs.aws.amazon.com/workspaces/latest/adminguide/update-directory-details.html#control-device-access"> Control Device Access</a>.</p>
    async fn modify_workspace_access_properties(
        &self,
        input: ModifyWorkspaceAccessPropertiesRequest,
    ) -> Result<
        ModifyWorkspaceAccessPropertiesResult,
        RusotoError<ModifyWorkspaceAccessPropertiesError>,
    >;

    /// <p>Modify the default properties used to create WorkSpaces.</p>
    async fn modify_workspace_creation_properties(
        &self,
        input: ModifyWorkspaceCreationPropertiesRequest,
    ) -> Result<
        ModifyWorkspaceCreationPropertiesResult,
        RusotoError<ModifyWorkspaceCreationPropertiesError>,
    >;

    /// <p>Modifies the specified WorkSpace properties.</p>
    async fn modify_workspace_properties(
        &self,
        input: ModifyWorkspacePropertiesRequest,
    ) -> Result<ModifyWorkspacePropertiesResult, RusotoError<ModifyWorkspacePropertiesError>>;

    /// <p>Sets the state of the specified WorkSpace.</p> <p>To maintain a WorkSpace without being interrupted, set the WorkSpace state to <code>ADMIN_MAINTENANCE</code>. WorkSpaces in this state do not respond to requests to reboot, stop, start, rebuild, or restore. An AutoStop WorkSpace in this state is not stopped. Users cannot log into a WorkSpace in the <code>ADMIN_MAINTENANCE</code> state.</p>
    async fn modify_workspace_state(
        &self,
        input: ModifyWorkspaceStateRequest,
    ) -> Result<ModifyWorkspaceStateResult, RusotoError<ModifyWorkspaceStateError>>;

    /// <p>Reboots the specified WorkSpaces.</p> <p>You cannot reboot a WorkSpace unless its state is <code>AVAILABLE</code> or <code>UNHEALTHY</code>.</p> <p>This operation is asynchronous and returns before the WorkSpaces have rebooted.</p>
    async fn reboot_workspaces(
        &self,
        input: RebootWorkspacesRequest,
    ) -> Result<RebootWorkspacesResult, RusotoError<RebootWorkspacesError>>;

    /// <p>Rebuilds the specified WorkSpace.</p> <p>You cannot rebuild a WorkSpace unless its state is <code>AVAILABLE</code>, <code>ERROR</code>, <code>UNHEALTHY</code>, or <code>STOPPED</code>.</p> <p>Rebuilding a WorkSpace is a potentially destructive action that can result in the loss of data. For more information, see <a href="https://docs.aws.amazon.com/workspaces/latest/adminguide/reset-workspace.html">Rebuild a WorkSpace</a>.</p> <p>This operation is asynchronous and returns before the WorkSpaces have been completely rebuilt.</p>
    async fn rebuild_workspaces(
        &self,
        input: RebuildWorkspacesRequest,
    ) -> Result<RebuildWorkspacesResult, RusotoError<RebuildWorkspacesError>>;

    /// <p>Registers the specified directory. This operation is asynchronous and returns before the WorkSpace directory is registered. If this is the first time you are registering a directory, you will need to create the workspaces_DefaultRole role before you can register a directory. For more information, see <a href="https://docs.aws.amazon.com/workspaces/latest/adminguide/workspaces-access-control.html#create-default-role"> Creating the workspaces_DefaultRole Role</a>.</p>
    async fn register_workspace_directory(
        &self,
        input: RegisterWorkspaceDirectoryRequest,
    ) -> Result<RegisterWorkspaceDirectoryResult, RusotoError<RegisterWorkspaceDirectoryError>>;

    /// <p>Restores the specified WorkSpace to its last known healthy state.</p> <p>You cannot restore a WorkSpace unless its state is <code> AVAILABLE</code>, <code>ERROR</code>, <code>UNHEALTHY</code>, or <code>STOPPED</code>.</p> <p>Restoring a WorkSpace is a potentially destructive action that can result in the loss of data. For more information, see <a href="https://docs.aws.amazon.com/workspaces/latest/adminguide/restore-workspace.html">Restore a WorkSpace</a>.</p> <p>This operation is asynchronous and returns before the WorkSpace is completely restored.</p>
    async fn restore_workspace(
        &self,
        input: RestoreWorkspaceRequest,
    ) -> Result<RestoreWorkspaceResult, RusotoError<RestoreWorkspaceError>>;

    /// <p>Removes one or more rules from the specified IP access control group.</p>
    async fn revoke_ip_rules(
        &self,
        input: RevokeIpRulesRequest,
    ) -> Result<RevokeIpRulesResult, RusotoError<RevokeIpRulesError>>;

    /// <p>Starts the specified WorkSpaces.</p> <p>You cannot start a WorkSpace unless it has a running mode of <code>AutoStop</code> and a state of <code>STOPPED</code>.</p>
    async fn start_workspaces(
        &self,
        input: StartWorkspacesRequest,
    ) -> Result<StartWorkspacesResult, RusotoError<StartWorkspacesError>>;

    /// <p> Stops the specified WorkSpaces.</p> <p>You cannot stop a WorkSpace unless it has a running mode of <code>AutoStop</code> and a state of <code>AVAILABLE</code>, <code>IMPAIRED</code>, <code>UNHEALTHY</code>, or <code>ERROR</code>.</p>
    async fn stop_workspaces(
        &self,
        input: StopWorkspacesRequest,
    ) -> Result<StopWorkspacesResult, RusotoError<StopWorkspacesError>>;

    /// <p>Terminates the specified WorkSpaces.</p> <p>Terminating a WorkSpace is a permanent action and cannot be undone. The user's data is destroyed. If you need to archive any user data, contact Amazon Web Services before terminating the WorkSpace.</p> <p>You can terminate a WorkSpace that is in any state except <code>SUSPENDED</code>.</p> <p>This operation is asynchronous and returns before the WorkSpaces have been completely terminated.</p>
    async fn terminate_workspaces(
        &self,
        input: TerminateWorkspacesRequest,
    ) -> Result<TerminateWorkspacesResult, RusotoError<TerminateWorkspacesError>>;

    /// <p>Replaces the current rules of the specified IP access control group with the specified rules.</p>
    async fn update_rules_of_ip_group(
        &self,
        input: UpdateRulesOfIpGroupRequest,
    ) -> Result<UpdateRulesOfIpGroupResult, RusotoError<UpdateRulesOfIpGroupError>>;
}
/// A client for the Amazon WorkSpaces API.
#[derive(Clone)]
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
            region,
        }
    }

    pub fn new_with<P, D>(
        request_dispatcher: D,
        credentials_provider: P,
        region: region::Region,
    ) -> WorkspacesClient
    where
        P: ProvideAwsCredentials + Send + Sync + 'static,
        D: DispatchSignedRequest + Send + Sync + 'static,
    {
        WorkspacesClient {
            client: Client::new_with(credentials_provider, request_dispatcher),
            region,
        }
    }

    pub fn new_with_client(client: Client, region: region::Region) -> WorkspacesClient {
        WorkspacesClient { client, region }
    }
}

#[async_trait]
impl Workspaces for WorkspacesClient {
    /// <p>Associates the specified IP access control group with the specified directory.</p>
    async fn associate_ip_groups(
        &self,
        input: AssociateIpGroupsRequest,
    ) -> Result<AssociateIpGroupsResult, RusotoError<AssociateIpGroupsError>> {
        let mut request = SignedRequest::new("POST", "workspaces", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "WorkspacesService.AssociateIpGroups");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response).deserialize::<AssociateIpGroupsResult, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(AssociateIpGroupsError::from_response(response))
        }
    }

    /// <p>Adds one or more rules to the specified IP access control group.</p> <p>This action gives users permission to access their WorkSpaces from the CIDR address ranges specified in the rules.</p>
    async fn authorize_ip_rules(
        &self,
        input: AuthorizeIpRulesRequest,
    ) -> Result<AuthorizeIpRulesResult, RusotoError<AuthorizeIpRulesError>> {
        let mut request = SignedRequest::new("POST", "workspaces", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "WorkspacesService.AuthorizeIpRules");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response).deserialize::<AuthorizeIpRulesResult, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(AuthorizeIpRulesError::from_response(response))
        }
    }

    /// <p>Copies the specified image from the specified Region to the current Region.</p>
    async fn copy_workspace_image(
        &self,
        input: CopyWorkspaceImageRequest,
    ) -> Result<CopyWorkspaceImageResult, RusotoError<CopyWorkspaceImageError>> {
        let mut request = SignedRequest::new("POST", "workspaces", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "WorkspacesService.CopyWorkspaceImage");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response)
                .deserialize::<CopyWorkspaceImageResult, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(CopyWorkspaceImageError::from_response(response))
        }
    }

    /// <p>Creates an IP access control group.</p> <p>An IP access control group provides you with the ability to control the IP addresses from which users are allowed to access their WorkSpaces. To specify the CIDR address ranges, add rules to your IP access control group and then associate the group with your directory. You can add rules when you create the group or at any time using <a>AuthorizeIpRules</a>.</p> <p>There is a default IP access control group associated with your directory. If you don't associate an IP access control group with your directory, the default group is used. The default group includes a default rule that allows users to access their WorkSpaces from anywhere. You cannot modify the default IP access control group for your directory.</p>
    async fn create_ip_group(
        &self,
        input: CreateIpGroupRequest,
    ) -> Result<CreateIpGroupResult, RusotoError<CreateIpGroupError>> {
        let mut request = SignedRequest::new("POST", "workspaces", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "WorkspacesService.CreateIpGroup");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response).deserialize::<CreateIpGroupResult, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(CreateIpGroupError::from_response(response))
        }
    }

    /// <p>Creates the specified tags for the specified WorkSpaces resource.</p>
    async fn create_tags(
        &self,
        input: CreateTagsRequest,
    ) -> Result<CreateTagsResult, RusotoError<CreateTagsError>> {
        let mut request = SignedRequest::new("POST", "workspaces", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "WorkspacesService.CreateTags");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response).deserialize::<CreateTagsResult, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(CreateTagsError::from_response(response))
        }
    }

    /// <p>Creates one or more WorkSpaces.</p> <p>This operation is asynchronous and returns before the WorkSpaces are created.</p>
    async fn create_workspaces(
        &self,
        input: CreateWorkspacesRequest,
    ) -> Result<CreateWorkspacesResult, RusotoError<CreateWorkspacesError>> {
        let mut request = SignedRequest::new("POST", "workspaces", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "WorkspacesService.CreateWorkspaces");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response).deserialize::<CreateWorkspacesResult, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(CreateWorkspacesError::from_response(response))
        }
    }

    /// <p>Deletes the specified IP access control group.</p> <p>You cannot delete an IP access control group that is associated with a directory.</p>
    async fn delete_ip_group(
        &self,
        input: DeleteIpGroupRequest,
    ) -> Result<DeleteIpGroupResult, RusotoError<DeleteIpGroupError>> {
        let mut request = SignedRequest::new("POST", "workspaces", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "WorkspacesService.DeleteIpGroup");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response).deserialize::<DeleteIpGroupResult, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(DeleteIpGroupError::from_response(response))
        }
    }

    /// <p>Deletes the specified tags from the specified WorkSpaces resource.</p>
    async fn delete_tags(
        &self,
        input: DeleteTagsRequest,
    ) -> Result<DeleteTagsResult, RusotoError<DeleteTagsError>> {
        let mut request = SignedRequest::new("POST", "workspaces", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "WorkspacesService.DeleteTags");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response).deserialize::<DeleteTagsResult, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(DeleteTagsError::from_response(response))
        }
    }

    /// <p>Deletes the specified image from your account. To delete an image, you must first delete any bundles that are associated with the image and un-share the image if it is shared with other accounts. </p>
    async fn delete_workspace_image(
        &self,
        input: DeleteWorkspaceImageRequest,
    ) -> Result<DeleteWorkspaceImageResult, RusotoError<DeleteWorkspaceImageError>> {
        let mut request = SignedRequest::new("POST", "workspaces", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "WorkspacesService.DeleteWorkspaceImage");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response)
                .deserialize::<DeleteWorkspaceImageResult, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(DeleteWorkspaceImageError::from_response(response))
        }
    }

    /// <p>Deregisters the specified directory. This operation is asynchronous and returns before the WorkSpace directory is deregistered. If any WorkSpaces are registered to this directory, you must remove them before you can deregister the directory.</p>
    async fn deregister_workspace_directory(
        &self,
        input: DeregisterWorkspaceDirectoryRequest,
    ) -> Result<DeregisterWorkspaceDirectoryResult, RusotoError<DeregisterWorkspaceDirectoryError>>
    {
        let mut request = SignedRequest::new("POST", "workspaces", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "WorkspacesService.DeregisterWorkspaceDirectory",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response)
                .deserialize::<DeregisterWorkspaceDirectoryResult, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(DeregisterWorkspaceDirectoryError::from_response(response))
        }
    }

    /// <p>Retrieves a list that describes the configuration of Bring Your Own License (BYOL) for the specified account.</p>
    async fn describe_account(
        &self,
    ) -> Result<DescribeAccountResult, RusotoError<DescribeAccountError>> {
        let mut request = SignedRequest::new("POST", "workspaces", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "WorkspacesService.DescribeAccount");
        request.set_payload(Some(bytes::Bytes::from_static(b"{}")));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response).deserialize::<DescribeAccountResult, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(DescribeAccountError::from_response(response))
        }
    }

    /// <p>Retrieves a list that describes modifications to the configuration of Bring Your Own License (BYOL) for the specified account.</p>
    async fn describe_account_modifications(
        &self,
        input: DescribeAccountModificationsRequest,
    ) -> Result<DescribeAccountModificationsResult, RusotoError<DescribeAccountModificationsError>>
    {
        let mut request = SignedRequest::new("POST", "workspaces", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "WorkspacesService.DescribeAccountModifications",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response)
                .deserialize::<DescribeAccountModificationsResult, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(DescribeAccountModificationsError::from_response(response))
        }
    }

    /// <p>Retrieves a list that describes one or more specified Amazon WorkSpaces clients.</p>
    async fn describe_client_properties(
        &self,
        input: DescribeClientPropertiesRequest,
    ) -> Result<DescribeClientPropertiesResult, RusotoError<DescribeClientPropertiesError>> {
        let mut request = SignedRequest::new("POST", "workspaces", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "WorkspacesService.DescribeClientProperties");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response)
                .deserialize::<DescribeClientPropertiesResult, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(DescribeClientPropertiesError::from_response(response))
        }
    }

    /// <p>Describes one or more of your IP access control groups.</p>
    async fn describe_ip_groups(
        &self,
        input: DescribeIpGroupsRequest,
    ) -> Result<DescribeIpGroupsResult, RusotoError<DescribeIpGroupsError>> {
        let mut request = SignedRequest::new("POST", "workspaces", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "WorkspacesService.DescribeIpGroups");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response).deserialize::<DescribeIpGroupsResult, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(DescribeIpGroupsError::from_response(response))
        }
    }

    /// <p>Describes the specified tags for the specified WorkSpaces resource.</p>
    async fn describe_tags(
        &self,
        input: DescribeTagsRequest,
    ) -> Result<DescribeTagsResult, RusotoError<DescribeTagsError>> {
        let mut request = SignedRequest::new("POST", "workspaces", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "WorkspacesService.DescribeTags");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response).deserialize::<DescribeTagsResult, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(DescribeTagsError::from_response(response))
        }
    }

    /// <p>Retrieves a list that describes the available WorkSpace bundles.</p> <p>You can filter the results using either bundle ID or owner, but not both.</p>
    async fn describe_workspace_bundles(
        &self,
        input: DescribeWorkspaceBundlesRequest,
    ) -> Result<DescribeWorkspaceBundlesResult, RusotoError<DescribeWorkspaceBundlesError>> {
        let mut request = SignedRequest::new("POST", "workspaces", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "WorkspacesService.DescribeWorkspaceBundles");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response)
                .deserialize::<DescribeWorkspaceBundlesResult, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(DescribeWorkspaceBundlesError::from_response(response))
        }
    }

    /// <p>Describes the available directories that are registered with Amazon WorkSpaces.</p>
    async fn describe_workspace_directories(
        &self,
        input: DescribeWorkspaceDirectoriesRequest,
    ) -> Result<DescribeWorkspaceDirectoriesResult, RusotoError<DescribeWorkspaceDirectoriesError>>
    {
        let mut request = SignedRequest::new("POST", "workspaces", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "WorkspacesService.DescribeWorkspaceDirectories",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response)
                .deserialize::<DescribeWorkspaceDirectoriesResult, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(DescribeWorkspaceDirectoriesError::from_response(response))
        }
    }

    /// <p>Retrieves a list that describes one or more specified images, if the image identifiers are provided. Otherwise, all images in the account are described. </p>
    async fn describe_workspace_images(
        &self,
        input: DescribeWorkspaceImagesRequest,
    ) -> Result<DescribeWorkspaceImagesResult, RusotoError<DescribeWorkspaceImagesError>> {
        let mut request = SignedRequest::new("POST", "workspaces", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "WorkspacesService.DescribeWorkspaceImages");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response)
                .deserialize::<DescribeWorkspaceImagesResult, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(DescribeWorkspaceImagesError::from_response(response))
        }
    }

    /// <p>Describes the snapshots for the specified WorkSpace.</p>
    async fn describe_workspace_snapshots(
        &self,
        input: DescribeWorkspaceSnapshotsRequest,
    ) -> Result<DescribeWorkspaceSnapshotsResult, RusotoError<DescribeWorkspaceSnapshotsError>>
    {
        let mut request = SignedRequest::new("POST", "workspaces", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "WorkspacesService.DescribeWorkspaceSnapshots",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response)
                .deserialize::<DescribeWorkspaceSnapshotsResult, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(DescribeWorkspaceSnapshotsError::from_response(response))
        }
    }

    /// <p>Describes the specified WorkSpaces.</p> <p>You can filter the results by using the bundle identifier, directory identifier, or owner, but you can specify only one filter at a time.</p>
    async fn describe_workspaces(
        &self,
        input: DescribeWorkspacesRequest,
    ) -> Result<DescribeWorkspacesResult, RusotoError<DescribeWorkspacesError>> {
        let mut request = SignedRequest::new("POST", "workspaces", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "WorkspacesService.DescribeWorkspaces");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response)
                .deserialize::<DescribeWorkspacesResult, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(DescribeWorkspacesError::from_response(response))
        }
    }

    /// <p>Describes the connection status of the specified WorkSpaces.</p>
    async fn describe_workspaces_connection_status(
        &self,
        input: DescribeWorkspacesConnectionStatusRequest,
    ) -> Result<
        DescribeWorkspacesConnectionStatusResult,
        RusotoError<DescribeWorkspacesConnectionStatusError>,
    > {
        let mut request = SignedRequest::new("POST", "workspaces", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "WorkspacesService.DescribeWorkspacesConnectionStatus",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response)
                .deserialize::<DescribeWorkspacesConnectionStatusResult, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(DescribeWorkspacesConnectionStatusError::from_response(
                response,
            ))
        }
    }

    /// <p>Disassociates the specified IP access control group from the specified directory.</p>
    async fn disassociate_ip_groups(
        &self,
        input: DisassociateIpGroupsRequest,
    ) -> Result<DisassociateIpGroupsResult, RusotoError<DisassociateIpGroupsError>> {
        let mut request = SignedRequest::new("POST", "workspaces", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "WorkspacesService.DisassociateIpGroups");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response)
                .deserialize::<DisassociateIpGroupsResult, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(DisassociateIpGroupsError::from_response(response))
        }
    }

    /// <p>Imports the specified Windows 7 or Windows 10 Bring Your Own License (BYOL) image into Amazon WorkSpaces. The image must be an already licensed EC2 image that is in your AWS account, and you must own the image. </p>
    async fn import_workspace_image(
        &self,
        input: ImportWorkspaceImageRequest,
    ) -> Result<ImportWorkspaceImageResult, RusotoError<ImportWorkspaceImageError>> {
        let mut request = SignedRequest::new("POST", "workspaces", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "WorkspacesService.ImportWorkspaceImage");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response)
                .deserialize::<ImportWorkspaceImageResult, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(ImportWorkspaceImageError::from_response(response))
        }
    }

    /// <p>Retrieves a list of IP address ranges, specified as IPv4 CIDR blocks, that you can use for the network management interface when you enable Bring Your Own License (BYOL). </p> <p>The management network interface is connected to a secure Amazon WorkSpaces management network. It is used for interactive streaming of the WorkSpace desktop to Amazon WorkSpaces clients, and to allow Amazon WorkSpaces to manage the WorkSpace.</p>
    async fn list_available_management_cidr_ranges(
        &self,
        input: ListAvailableManagementCidrRangesRequest,
    ) -> Result<
        ListAvailableManagementCidrRangesResult,
        RusotoError<ListAvailableManagementCidrRangesError>,
    > {
        let mut request = SignedRequest::new("POST", "workspaces", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "WorkspacesService.ListAvailableManagementCidrRanges",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response)
                .deserialize::<ListAvailableManagementCidrRangesResult, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(ListAvailableManagementCidrRangesError::from_response(
                response,
            ))
        }
    }

    /// <p>Migrates a WorkSpace from one operating system or bundle type to another, while retaining the data on the user volume.</p> <p>The migration process recreates the WorkSpace by using a new root volume from the target bundle image and the user volume from the last available snapshot of the original WorkSpace. During migration, the original <code>D:\Users\%USERNAME%</code> user profile folder is renamed to <code>D:\Users\%USERNAME%MMddyyTHHmmss%.NotMigrated</code>. A new <code>D:\Users\%USERNAME%\</code> folder is generated by the new OS. Certain files in the old user profile are moved to the new user profile.</p> <p>For available migration scenarios, details about what happens during migration, and best practices, see <a href="https://docs.aws.amazon.com/workspaces/latest/adminguide/migrate-workspaces.html">Migrate a WorkSpace</a>.</p>
    async fn migrate_workspace(
        &self,
        input: MigrateWorkspaceRequest,
    ) -> Result<MigrateWorkspaceResult, RusotoError<MigrateWorkspaceError>> {
        let mut request = SignedRequest::new("POST", "workspaces", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "WorkspacesService.MigrateWorkspace");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response).deserialize::<MigrateWorkspaceResult, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(MigrateWorkspaceError::from_response(response))
        }
    }

    /// <p>Modifies the configuration of Bring Your Own License (BYOL) for the specified account.</p>
    async fn modify_account(
        &self,
        input: ModifyAccountRequest,
    ) -> Result<ModifyAccountResult, RusotoError<ModifyAccountError>> {
        let mut request = SignedRequest::new("POST", "workspaces", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "WorkspacesService.ModifyAccount");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response).deserialize::<ModifyAccountResult, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(ModifyAccountError::from_response(response))
        }
    }

    /// <p>Modifies the properties of the specified Amazon WorkSpaces clients.</p>
    async fn modify_client_properties(
        &self,
        input: ModifyClientPropertiesRequest,
    ) -> Result<ModifyClientPropertiesResult, RusotoError<ModifyClientPropertiesError>> {
        let mut request = SignedRequest::new("POST", "workspaces", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "WorkspacesService.ModifyClientProperties");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response)
                .deserialize::<ModifyClientPropertiesResult, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(ModifyClientPropertiesError::from_response(response))
        }
    }

    /// <p>Modifies the self-service WorkSpace management capabilities for your users. For more information, see <a href="https://docs.aws.amazon.com/workspaces/latest/adminguide/enable-user-self-service-workspace-management.html">Enable Self-Service WorkSpace Management Capabilities for Your Users</a>.</p>
    async fn modify_selfservice_permissions(
        &self,
        input: ModifySelfservicePermissionsRequest,
    ) -> Result<ModifySelfservicePermissionsResult, RusotoError<ModifySelfservicePermissionsError>>
    {
        let mut request = SignedRequest::new("POST", "workspaces", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "WorkspacesService.ModifySelfservicePermissions",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response)
                .deserialize::<ModifySelfservicePermissionsResult, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(ModifySelfservicePermissionsError::from_response(response))
        }
    }

    /// <p>Specifies which devices and operating systems users can use to access their WorkSpaces. For more information, see <a href="https://docs.aws.amazon.com/workspaces/latest/adminguide/update-directory-details.html#control-device-access"> Control Device Access</a>.</p>
    async fn modify_workspace_access_properties(
        &self,
        input: ModifyWorkspaceAccessPropertiesRequest,
    ) -> Result<
        ModifyWorkspaceAccessPropertiesResult,
        RusotoError<ModifyWorkspaceAccessPropertiesError>,
    > {
        let mut request = SignedRequest::new("POST", "workspaces", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "WorkspacesService.ModifyWorkspaceAccessProperties",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response)
                .deserialize::<ModifyWorkspaceAccessPropertiesResult, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(ModifyWorkspaceAccessPropertiesError::from_response(
                response,
            ))
        }
    }

    /// <p>Modify the default properties used to create WorkSpaces.</p>
    async fn modify_workspace_creation_properties(
        &self,
        input: ModifyWorkspaceCreationPropertiesRequest,
    ) -> Result<
        ModifyWorkspaceCreationPropertiesResult,
        RusotoError<ModifyWorkspaceCreationPropertiesError>,
    > {
        let mut request = SignedRequest::new("POST", "workspaces", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "WorkspacesService.ModifyWorkspaceCreationProperties",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response)
                .deserialize::<ModifyWorkspaceCreationPropertiesResult, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(ModifyWorkspaceCreationPropertiesError::from_response(
                response,
            ))
        }
    }

    /// <p>Modifies the specified WorkSpace properties.</p>
    async fn modify_workspace_properties(
        &self,
        input: ModifyWorkspacePropertiesRequest,
    ) -> Result<ModifyWorkspacePropertiesResult, RusotoError<ModifyWorkspacePropertiesError>> {
        let mut request = SignedRequest::new("POST", "workspaces", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "WorkspacesService.ModifyWorkspaceProperties",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response)
                .deserialize::<ModifyWorkspacePropertiesResult, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(ModifyWorkspacePropertiesError::from_response(response))
        }
    }

    /// <p>Sets the state of the specified WorkSpace.</p> <p>To maintain a WorkSpace without being interrupted, set the WorkSpace state to <code>ADMIN_MAINTENANCE</code>. WorkSpaces in this state do not respond to requests to reboot, stop, start, rebuild, or restore. An AutoStop WorkSpace in this state is not stopped. Users cannot log into a WorkSpace in the <code>ADMIN_MAINTENANCE</code> state.</p>
    async fn modify_workspace_state(
        &self,
        input: ModifyWorkspaceStateRequest,
    ) -> Result<ModifyWorkspaceStateResult, RusotoError<ModifyWorkspaceStateError>> {
        let mut request = SignedRequest::new("POST", "workspaces", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "WorkspacesService.ModifyWorkspaceState");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response)
                .deserialize::<ModifyWorkspaceStateResult, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(ModifyWorkspaceStateError::from_response(response))
        }
    }

    /// <p>Reboots the specified WorkSpaces.</p> <p>You cannot reboot a WorkSpace unless its state is <code>AVAILABLE</code> or <code>UNHEALTHY</code>.</p> <p>This operation is asynchronous and returns before the WorkSpaces have rebooted.</p>
    async fn reboot_workspaces(
        &self,
        input: RebootWorkspacesRequest,
    ) -> Result<RebootWorkspacesResult, RusotoError<RebootWorkspacesError>> {
        let mut request = SignedRequest::new("POST", "workspaces", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "WorkspacesService.RebootWorkspaces");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response).deserialize::<RebootWorkspacesResult, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(RebootWorkspacesError::from_response(response))
        }
    }

    /// <p>Rebuilds the specified WorkSpace.</p> <p>You cannot rebuild a WorkSpace unless its state is <code>AVAILABLE</code>, <code>ERROR</code>, <code>UNHEALTHY</code>, or <code>STOPPED</code>.</p> <p>Rebuilding a WorkSpace is a potentially destructive action that can result in the loss of data. For more information, see <a href="https://docs.aws.amazon.com/workspaces/latest/adminguide/reset-workspace.html">Rebuild a WorkSpace</a>.</p> <p>This operation is asynchronous and returns before the WorkSpaces have been completely rebuilt.</p>
    async fn rebuild_workspaces(
        &self,
        input: RebuildWorkspacesRequest,
    ) -> Result<RebuildWorkspacesResult, RusotoError<RebuildWorkspacesError>> {
        let mut request = SignedRequest::new("POST", "workspaces", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "WorkspacesService.RebuildWorkspaces");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response).deserialize::<RebuildWorkspacesResult, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(RebuildWorkspacesError::from_response(response))
        }
    }

    /// <p>Registers the specified directory. This operation is asynchronous and returns before the WorkSpace directory is registered. If this is the first time you are registering a directory, you will need to create the workspaces_DefaultRole role before you can register a directory. For more information, see <a href="https://docs.aws.amazon.com/workspaces/latest/adminguide/workspaces-access-control.html#create-default-role"> Creating the workspaces_DefaultRole Role</a>.</p>
    async fn register_workspace_directory(
        &self,
        input: RegisterWorkspaceDirectoryRequest,
    ) -> Result<RegisterWorkspaceDirectoryResult, RusotoError<RegisterWorkspaceDirectoryError>>
    {
        let mut request = SignedRequest::new("POST", "workspaces", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "WorkspacesService.RegisterWorkspaceDirectory",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response)
                .deserialize::<RegisterWorkspaceDirectoryResult, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(RegisterWorkspaceDirectoryError::from_response(response))
        }
    }

    /// <p>Restores the specified WorkSpace to its last known healthy state.</p> <p>You cannot restore a WorkSpace unless its state is <code> AVAILABLE</code>, <code>ERROR</code>, <code>UNHEALTHY</code>, or <code>STOPPED</code>.</p> <p>Restoring a WorkSpace is a potentially destructive action that can result in the loss of data. For more information, see <a href="https://docs.aws.amazon.com/workspaces/latest/adminguide/restore-workspace.html">Restore a WorkSpace</a>.</p> <p>This operation is asynchronous and returns before the WorkSpace is completely restored.</p>
    async fn restore_workspace(
        &self,
        input: RestoreWorkspaceRequest,
    ) -> Result<RestoreWorkspaceResult, RusotoError<RestoreWorkspaceError>> {
        let mut request = SignedRequest::new("POST", "workspaces", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "WorkspacesService.RestoreWorkspace");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response).deserialize::<RestoreWorkspaceResult, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(RestoreWorkspaceError::from_response(response))
        }
    }

    /// <p>Removes one or more rules from the specified IP access control group.</p>
    async fn revoke_ip_rules(
        &self,
        input: RevokeIpRulesRequest,
    ) -> Result<RevokeIpRulesResult, RusotoError<RevokeIpRulesError>> {
        let mut request = SignedRequest::new("POST", "workspaces", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "WorkspacesService.RevokeIpRules");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response).deserialize::<RevokeIpRulesResult, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(RevokeIpRulesError::from_response(response))
        }
    }

    /// <p>Starts the specified WorkSpaces.</p> <p>You cannot start a WorkSpace unless it has a running mode of <code>AutoStop</code> and a state of <code>STOPPED</code>.</p>
    async fn start_workspaces(
        &self,
        input: StartWorkspacesRequest,
    ) -> Result<StartWorkspacesResult, RusotoError<StartWorkspacesError>> {
        let mut request = SignedRequest::new("POST", "workspaces", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "WorkspacesService.StartWorkspaces");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response).deserialize::<StartWorkspacesResult, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(StartWorkspacesError::from_response(response))
        }
    }

    /// <p> Stops the specified WorkSpaces.</p> <p>You cannot stop a WorkSpace unless it has a running mode of <code>AutoStop</code> and a state of <code>AVAILABLE</code>, <code>IMPAIRED</code>, <code>UNHEALTHY</code>, or <code>ERROR</code>.</p>
    async fn stop_workspaces(
        &self,
        input: StopWorkspacesRequest,
    ) -> Result<StopWorkspacesResult, RusotoError<StopWorkspacesError>> {
        let mut request = SignedRequest::new("POST", "workspaces", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "WorkspacesService.StopWorkspaces");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response).deserialize::<StopWorkspacesResult, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(StopWorkspacesError::from_response(response))
        }
    }

    /// <p>Terminates the specified WorkSpaces.</p> <p>Terminating a WorkSpace is a permanent action and cannot be undone. The user's data is destroyed. If you need to archive any user data, contact Amazon Web Services before terminating the WorkSpace.</p> <p>You can terminate a WorkSpace that is in any state except <code>SUSPENDED</code>.</p> <p>This operation is asynchronous and returns before the WorkSpaces have been completely terminated.</p>
    async fn terminate_workspaces(
        &self,
        input: TerminateWorkspacesRequest,
    ) -> Result<TerminateWorkspacesResult, RusotoError<TerminateWorkspacesError>> {
        let mut request = SignedRequest::new("POST", "workspaces", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "WorkspacesService.TerminateWorkspaces");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response)
                .deserialize::<TerminateWorkspacesResult, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(TerminateWorkspacesError::from_response(response))
        }
    }

    /// <p>Replaces the current rules of the specified IP access control group with the specified rules.</p>
    async fn update_rules_of_ip_group(
        &self,
        input: UpdateRulesOfIpGroupRequest,
    ) -> Result<UpdateRulesOfIpGroupResult, RusotoError<UpdateRulesOfIpGroupError>> {
        let mut request = SignedRequest::new("POST", "workspaces", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "WorkspacesService.UpdateRulesOfIpGroup");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response)
                .deserialize::<UpdateRulesOfIpGroupResult, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(UpdateRulesOfIpGroupError::from_response(response))
        }
    }
}
