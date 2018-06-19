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

use hyper::StatusCode;
use rusoto_core::signature::SignedRequest;
use serde_json;
use serde_json::from_str;
use serde_json::Value as SerdeJsonValue;
/// <p>Describes an application in the application catalog.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct Application {
    /// <p>The application name for display.</p>
    #[serde(rename = "DisplayName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    /// <p>If there is a problem, the application can be disabled after image creation.</p>
    #[serde(rename = "Enabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    /// <p>The URL for the application icon. This URL might be time-limited.</p>
    #[serde(rename = "IconURL")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon_url: Option<String>,
    /// <p>The arguments that are passed to the application at launch.</p>
    #[serde(rename = "LaunchParameters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub launch_parameters: Option<String>,
    /// <p>The path to the application executable in the instance.</p>
    #[serde(rename = "LaunchPath")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub launch_path: Option<String>,
    /// <p>Additional attributes that describe the application.</p>
    #[serde(rename = "Metadata")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<::std::collections::HashMap<String, String>>,
    /// <p>The name of the application.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct AssociateFleetRequest {
    /// <p>The name of the fleet. </p>
    #[serde(rename = "FleetName")]
    pub fleet_name: String,
    /// <p>The name of the stack.</p>
    #[serde(rename = "StackName")]
    pub stack_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct AssociateFleetResult {}

/// <p>Describes the capacity for a fleet.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ComputeCapacity {
    /// <p>The desired number of streaming instances.</p>
    #[serde(rename = "DesiredInstances")]
    pub desired_instances: i64,
}

/// <p>Describes the capacity status for a fleet.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct ComputeCapacityStatus {
    /// <p>The number of currently available instances that can be used to stream sessions.</p>
    #[serde(rename = "Available")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub available: Option<i64>,
    /// <p>The desired number of streaming instances.</p>
    #[serde(rename = "Desired")]
    pub desired: i64,
    /// <p>The number of instances in use for streaming.</p>
    #[serde(rename = "InUse")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub in_use: Option<i64>,
    /// <p>The total number of simultaneous streaming instances that are running.</p>
    #[serde(rename = "Running")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub running: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct CopyImageRequest {
    /// <p>The description that the image will have when it is copied to the destination.</p>
    #[serde(rename = "DestinationImageDescription")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination_image_description: Option<String>,
    /// <p>The name that the image will have when it is copied to the destination.</p>
    #[serde(rename = "DestinationImageName")]
    pub destination_image_name: String,
    /// <p>The destination region to which the image will be copied. This parameter is required, even if you are copying an image within the same region.</p>
    #[serde(rename = "DestinationRegion")]
    pub destination_region: String,
    /// <p>The name of the image to copy.</p>
    #[serde(rename = "SourceImageName")]
    pub source_image_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct CopyImageResponse {
    /// <p>The name of the destination image.</p>
    #[serde(rename = "DestinationImageName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination_image_name: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct CreateDirectoryConfigRequest {
    /// <p>The fully qualified name of the directory (for example, corp.example.com).</p>
    #[serde(rename = "DirectoryName")]
    pub directory_name: String,
    /// <p>The distinguished names of the organizational units for computer accounts.</p>
    #[serde(rename = "OrganizationalUnitDistinguishedNames")]
    pub organizational_unit_distinguished_names: Vec<String>,
    /// <p>The credentials for the service account used by the streaming instance to connect to the directory.</p>
    #[serde(rename = "ServiceAccountCredentials")]
    pub service_account_credentials: ServiceAccountCredentials,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct CreateDirectoryConfigResult {
    /// <p>Information about the directory configuration.</p>
    #[serde(rename = "DirectoryConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub directory_config: Option<DirectoryConfig>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct CreateFleetRequest {
    /// <p>The desired capacity for the fleet.</p>
    #[serde(rename = "ComputeCapacity")]
    pub compute_capacity: ComputeCapacity,
    /// <p>The description for display.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The time after disconnection when a session is considered to have ended, in seconds. If a user who was disconnected reconnects within this time interval, the user is connected to their previous session. Specify a value between 60 and 57600.</p>
    #[serde(rename = "DisconnectTimeoutInSeconds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disconnect_timeout_in_seconds: Option<i64>,
    /// <p>The fleet name for display.</p>
    #[serde(rename = "DisplayName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    /// <p>The information needed to join a Microsoft Active Directory domain.</p>
    #[serde(rename = "DomainJoinInfo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_join_info: Option<DomainJoinInfo>,
    /// <p>Enables or disables default internet access for the fleet.</p>
    #[serde(rename = "EnableDefaultInternetAccess")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_default_internet_access: Option<bool>,
    /// <p><p>The fleet type.</p> <dl> <dt>ALWAYS<em>ON</dt> <dd> <p>Provides users with instant-on access to their apps. You are charged for all running instances in your fleet, even if no users are streaming apps.</p> </dd> <dt>ON</em>DEMAND</dt> <dd> <p>Provide users with access to applications after they connect, which takes one to two minutes. You are charged for instance streaming when users are connected and a small hourly fee for instances that are not streaming apps.</p> </dd> </dl></p>
    #[serde(rename = "FleetType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fleet_type: Option<String>,
    /// <p>The name of the image used to create the fleet.</p>
    #[serde(rename = "ImageName")]
    pub image_name: String,
    /// <p><p>The instance type to use when launching fleet instances. The following instance types are available:</p> <ul> <li> <p>stream.standard.medium</p> </li> <li> <p>stream.standard.large</p> </li> <li> <p>stream.compute.large</p> </li> <li> <p>stream.compute.xlarge</p> </li> <li> <p>stream.compute.2xlarge</p> </li> <li> <p>stream.compute.4xlarge</p> </li> <li> <p>stream.compute.8xlarge</p> </li> <li> <p>stream.memory.large</p> </li> <li> <p>stream.memory.xlarge</p> </li> <li> <p>stream.memory.2xlarge</p> </li> <li> <p>stream.memory.4xlarge</p> </li> <li> <p>stream.memory.8xlarge</p> </li> <li> <p>stream.graphics-design.large</p> </li> <li> <p>stream.graphics-design.xlarge</p> </li> <li> <p>stream.graphics-design.2xlarge</p> </li> <li> <p>stream.graphics-design.4xlarge</p> </li> <li> <p>stream.graphics-desktop.2xlarge</p> </li> <li> <p>stream.graphics-pro.4xlarge</p> </li> <li> <p>stream.graphics-pro.8xlarge</p> </li> <li> <p>stream.graphics-pro.16xlarge</p> </li> </ul></p>
    #[serde(rename = "InstanceType")]
    pub instance_type: String,
    /// <p>The maximum time that a streaming session can run, in seconds. Specify a value between 600 and 57600.</p>
    #[serde(rename = "MaxUserDurationInSeconds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_user_duration_in_seconds: Option<i64>,
    /// <p>A unique name for the fleet.</p>
    #[serde(rename = "Name")]
    pub name: String,
    /// <p>The VPC configuration for the fleet.</p>
    #[serde(rename = "VpcConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_config: Option<VpcConfig>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct CreateFleetResult {
    /// <p>Information about the fleet.</p>
    #[serde(rename = "Fleet")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fleet: Option<Fleet>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct CreateImageBuilderRequest {
    /// <p>The version of the AppStream 2.0 agent to use for this image builder. To use the latest version of the AppStream 2.0 agent, specify [LATEST]. </p>
    #[serde(rename = "AppstreamAgentVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub appstream_agent_version: Option<String>,
    /// <p>The description for display.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The image builder name for display.</p>
    #[serde(rename = "DisplayName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    /// <p>The information needed to join a Microsoft Active Directory domain.</p>
    #[serde(rename = "DomainJoinInfo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_join_info: Option<DomainJoinInfo>,
    /// <p>Enables or disables default internet access for the image builder.</p>
    #[serde(rename = "EnableDefaultInternetAccess")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_default_internet_access: Option<bool>,
    /// <p>The name of the image used to create the builder.</p>
    #[serde(rename = "ImageName")]
    pub image_name: String,
    /// <p>The instance type to use when launching the image builder.</p>
    #[serde(rename = "InstanceType")]
    pub instance_type: String,
    /// <p>A unique name for the image builder.</p>
    #[serde(rename = "Name")]
    pub name: String,
    /// <p>The VPC configuration for the image builder. You can specify only one subnet.</p>
    #[serde(rename = "VpcConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_config: Option<VpcConfig>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct CreateImageBuilderResult {
    /// <p>Information about the image builder.</p>
    #[serde(rename = "ImageBuilder")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_builder: Option<ImageBuilder>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct CreateImageBuilderStreamingURLRequest {
    /// <p>The name of the image builder.</p>
    #[serde(rename = "Name")]
    pub name: String,
    /// <p>The time that the streaming URL will be valid, in seconds. Specify a value between 1 and 604800 seconds. The default is 3600 seconds.</p>
    #[serde(rename = "Validity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub validity: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct CreateImageBuilderStreamingURLResult {
    /// <p>The elapsed time, in seconds after the Unix epoch, when this URL expires.</p>
    #[serde(rename = "Expires")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expires: Option<f64>,
    /// <p>The URL to start the AppStream 2.0 streaming session.</p>
    #[serde(rename = "StreamingURL")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub streaming_url: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct CreateStackRequest {
    /// <p>The description for display.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The stack name for display.</p>
    #[serde(rename = "DisplayName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    /// <p>The URL that users are redirected to after they click the Send Feedback link. If no URL is specified, no Send Feedback link is displayed.</p>
    #[serde(rename = "FeedbackURL")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub feedback_url: Option<String>,
    /// <p>The name of the stack.</p>
    #[serde(rename = "Name")]
    pub name: String,
    /// <p>The URL that users are redirected to after their streaming session ends.</p>
    #[serde(rename = "RedirectURL")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub redirect_url: Option<String>,
    /// <p>The storage connectors to enable.</p>
    #[serde(rename = "StorageConnectors")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage_connectors: Option<Vec<StorageConnector>>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct CreateStackResult {
    /// <p>Information about the stack.</p>
    #[serde(rename = "Stack")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stack: Option<Stack>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct CreateStreamingURLRequest {
    /// <p>The name of the application to launch after the session starts. This is the name that you specified as <b>Name</b> in the Image Assistant.</p>
    #[serde(rename = "ApplicationId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_id: Option<String>,
    /// <p>The name of the fleet.</p>
    #[serde(rename = "FleetName")]
    pub fleet_name: String,
    /// <p>The session context. For more information, see <a href="http://docs.aws.amazon.com/appstream2/latest/developerguide/managing-stacks-fleets.html#managing-stacks-fleets-parameters">Session Context</a> in the <i>Amazon AppStream 2.0 Developer Guide</i>.</p>
    #[serde(rename = "SessionContext")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub session_context: Option<String>,
    /// <p>The name of the stack.</p>
    #[serde(rename = "StackName")]
    pub stack_name: String,
    /// <p>The ID of the user.</p>
    #[serde(rename = "UserId")]
    pub user_id: String,
    /// <p>The time that the streaming URL will be valid, in seconds. Specify a value between 1 and 604800 seconds. The default is 60 seconds.</p>
    #[serde(rename = "Validity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub validity: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct CreateStreamingURLResult {
    /// <p>The elapsed time, in seconds after the Unix epoch, when this URL expires.</p>
    #[serde(rename = "Expires")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expires: Option<f64>,
    /// <p>The URL to start the AppStream 2.0 streaming session.</p>
    #[serde(rename = "StreamingURL")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub streaming_url: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DeleteDirectoryConfigRequest {
    /// <p>The name of the directory configuration.</p>
    #[serde(rename = "DirectoryName")]
    pub directory_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct DeleteDirectoryConfigResult {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DeleteFleetRequest {
    /// <p>The name of the fleet.</p>
    #[serde(rename = "Name")]
    pub name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct DeleteFleetResult {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DeleteImageBuilderRequest {
    /// <p>The name of the image builder.</p>
    #[serde(rename = "Name")]
    pub name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct DeleteImageBuilderResult {
    /// <p>Information about the image builder.</p>
    #[serde(rename = "ImageBuilder")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_builder: Option<ImageBuilder>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DeleteImageRequest {
    /// <p>The name of the image.</p>
    #[serde(rename = "Name")]
    pub name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct DeleteImageResult {
    /// <p>Information about the image.</p>
    #[serde(rename = "Image")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image: Option<Image>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DeleteStackRequest {
    /// <p>The name of the stack.</p>
    #[serde(rename = "Name")]
    pub name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct DeleteStackResult {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DescribeDirectoryConfigsRequest {
    /// <p>The directory names.</p>
    #[serde(rename = "DirectoryNames")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub directory_names: Option<Vec<String>>,
    /// <p>The maximum size of each page of results.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>The pagination token to use to retrieve the next page of results for this operation. If this value is null, it retrieves the first page.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct DescribeDirectoryConfigsResult {
    /// <p>Information about the directory configurations. Note that although the response syntax in this topic includes the account password, this password is not returned in the actual response. </p>
    #[serde(rename = "DirectoryConfigs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub directory_configs: Option<Vec<DirectoryConfig>>,
    /// <p>The pagination token to use to retrieve the next page of results for this operation. If there are no more pages, this value is null.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DescribeFleetsRequest {
    /// <p>The names of the fleets to describe.</p>
    #[serde(rename = "Names")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub names: Option<Vec<String>>,
    /// <p>The pagination token to use to retrieve the next page of results for this operation. If this value is null, it retrieves the first page.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct DescribeFleetsResult {
    /// <p>Information about the fleets.</p>
    #[serde(rename = "Fleets")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fleets: Option<Vec<Fleet>>,
    /// <p>The pagination token to use to retrieve the next page of results for this operation. If there are no more pages, this value is null.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DescribeImageBuildersRequest {
    /// <p>The maximum size of each page of results.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>The names of the image builders to describe.</p>
    #[serde(rename = "Names")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub names: Option<Vec<String>>,
    /// <p>The pagination token to use to retrieve the next page of results for this operation. If this value is null, it retrieves the first page.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct DescribeImageBuildersResult {
    /// <p>Information about the image builders.</p>
    #[serde(rename = "ImageBuilders")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_builders: Option<Vec<ImageBuilder>>,
    /// <p>The pagination token to use to retrieve the next page of results for this operation. If there are no more pages, this value is null.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DescribeImagesRequest {
    /// <p>The names of the images to describe.</p>
    #[serde(rename = "Names")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub names: Option<Vec<String>>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct DescribeImagesResult {
    /// <p>Information about the images.</p>
    #[serde(rename = "Images")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub images: Option<Vec<Image>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DescribeSessionsRequest {
    /// <p>The authentication method. Specify <code>API</code> for a user authenticated using a streaming URL or <code>SAML</code> for a SAML federated user. The default is to authenticate users using a streaming URL.</p>
    #[serde(rename = "AuthenticationType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authentication_type: Option<String>,
    /// <p>The name of the fleet. This value is case-sensitive.</p>
    #[serde(rename = "FleetName")]
    pub fleet_name: String,
    /// <p>The size of each page of results. The default value is 20 and the maximum value is 50.</p>
    #[serde(rename = "Limit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// <p>The pagination token to use to retrieve the next page of results for this operation. If this value is null, it retrieves the first page.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The name of the stack. This value is case-sensitive.</p>
    #[serde(rename = "StackName")]
    pub stack_name: String,
    /// <p>The user ID.</p>
    #[serde(rename = "UserId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct DescribeSessionsResult {
    /// <p>The pagination token to use to retrieve the next page of results for this operation. If there are no more pages, this value is null.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>Information about the streaming sessions.</p>
    #[serde(rename = "Sessions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sessions: Option<Vec<Session>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DescribeStacksRequest {
    /// <p>The names of the stacks to describe.</p>
    #[serde(rename = "Names")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub names: Option<Vec<String>>,
    /// <p>The pagination token to use to retrieve the next page of results for this operation. If this value is null, it retrieves the first page.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct DescribeStacksResult {
    /// <p>The pagination token to use to retrieve the next page of results for this operation. If there are no more pages, this value is null.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>Information about the stacks.</p>
    #[serde(rename = "Stacks")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stacks: Option<Vec<Stack>>,
}

/// <p>Configuration information for the directory used to join domains.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct DirectoryConfig {
    /// <p>The time the directory configuration was created.</p>
    #[serde(rename = "CreatedTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_time: Option<f64>,
    /// <p>The fully qualified name of the directory (for example, corp.example.com).</p>
    #[serde(rename = "DirectoryName")]
    pub directory_name: String,
    /// <p>The distinguished names of the organizational units for computer accounts.</p>
    #[serde(rename = "OrganizationalUnitDistinguishedNames")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub organizational_unit_distinguished_names: Option<Vec<String>>,
    /// <p>The credentials for the service account used by the streaming instance to connect to the directory.</p>
    #[serde(rename = "ServiceAccountCredentials")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_account_credentials: Option<ServiceAccountCredentials>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DisassociateFleetRequest {
    /// <p>The name of the fleet.</p>
    #[serde(rename = "FleetName")]
    pub fleet_name: String,
    /// <p>The name of the stack.</p>
    #[serde(rename = "StackName")]
    pub stack_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct DisassociateFleetResult {}

/// <p>Contains the information needed to join a Microsoft Active Directory domain.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DomainJoinInfo {
    /// <p>The fully qualified name of the directory (for example, corp.example.com).</p>
    #[serde(rename = "DirectoryName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub directory_name: Option<String>,
    /// <p>The distinguished name of the organizational unit for computer accounts.</p>
    #[serde(rename = "OrganizationalUnitDistinguishedName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub organizational_unit_distinguished_name: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ExpireSessionRequest {
    /// <p>The ID of the streaming session.</p>
    #[serde(rename = "SessionId")]
    pub session_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct ExpireSessionResult {}

/// <p>Contains the parameters for a fleet.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct Fleet {
    /// <p>The ARN for the fleet.</p>
    #[serde(rename = "Arn")]
    pub arn: String,
    /// <p>The capacity status for the fleet.</p>
    #[serde(rename = "ComputeCapacityStatus")]
    pub compute_capacity_status: ComputeCapacityStatus,
    /// <p>The time the fleet was created.</p>
    #[serde(rename = "CreatedTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_time: Option<f64>,
    /// <p>The description for display.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The time after disconnection when a session is considered to have ended, in seconds. If a user who was disconnected reconnects within this time interval, the user is connected to their previous session. Specify a value between 60 and 57600.</p>
    #[serde(rename = "DisconnectTimeoutInSeconds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disconnect_timeout_in_seconds: Option<i64>,
    /// <p>The fleet name for display.</p>
    #[serde(rename = "DisplayName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    /// <p>The information needed to join a Microsoft Active Directory domain.</p>
    #[serde(rename = "DomainJoinInfo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_join_info: Option<DomainJoinInfo>,
    /// <p>Indicates whether default internet access is enabled for the fleet.</p>
    #[serde(rename = "EnableDefaultInternetAccess")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_default_internet_access: Option<bool>,
    /// <p>The fleet errors.</p>
    #[serde(rename = "FleetErrors")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fleet_errors: Option<Vec<FleetError>>,
    /// <p><p>The fleet type.</p> <dl> <dt>ALWAYS<em>ON</dt> <dd> <p>Provides users with instant-on access to their apps. You are charged for all running instances in your fleet, even if no users are streaming apps.</p> </dd> <dt>ON</em>DEMAND</dt> <dd> <p>Provide users with access to applications after they connect, which takes one to two minutes. You are charged for instance streaming when users are connected and a small hourly fee for instances that are not streaming apps.</p> </dd> </dl></p>
    #[serde(rename = "FleetType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fleet_type: Option<String>,
    /// <p>The name of the image used to create the fleet.</p>
    #[serde(rename = "ImageName")]
    pub image_name: String,
    /// <p>The instance type to use when launching fleet instances.</p>
    #[serde(rename = "InstanceType")]
    pub instance_type: String,
    /// <p>The maximum time that a streaming session can run, in seconds. Specify a value between 600 and 57600.</p>
    #[serde(rename = "MaxUserDurationInSeconds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_user_duration_in_seconds: Option<i64>,
    /// <p>The name of the fleet.</p>
    #[serde(rename = "Name")]
    pub name: String,
    /// <p>The current state for the fleet.</p>
    #[serde(rename = "State")]
    pub state: String,
    /// <p>The VPC configuration for the fleet.</p>
    #[serde(rename = "VpcConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_config: Option<VpcConfig>,
}

/// <p>Describes a fleet error.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct FleetError {
    /// <p>The error code.</p>
    #[serde(rename = "ErrorCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_code: Option<String>,
    /// <p>The error message.</p>
    #[serde(rename = "ErrorMessage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
}

/// <p>Describes an image.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct Image {
    /// <p>The applications associated with the image.</p>
    #[serde(rename = "Applications")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub applications: Option<Vec<Application>>,
    /// <p>The version of the AppStream 2.0 agent to use for instances that are launched from this image. </p>
    #[serde(rename = "AppstreamAgentVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub appstream_agent_version: Option<String>,
    /// <p>The ARN of the image.</p>
    #[serde(rename = "Arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p>The ARN of the image from which this image was created.</p>
    #[serde(rename = "BaseImageArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub base_image_arn: Option<String>,
    /// <p>The time the image was created.</p>
    #[serde(rename = "CreatedTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_time: Option<f64>,
    /// <p>The description for display.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The image name for display.</p>
    #[serde(rename = "DisplayName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    /// <p>Indicates whether an image builder can be launched from this image.</p>
    #[serde(rename = "ImageBuilderSupported")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_builder_supported: Option<bool>,
    /// <p>The name of the image.</p>
    #[serde(rename = "Name")]
    pub name: String,
    /// <p>The operating system platform of the image.</p>
    #[serde(rename = "Platform")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub platform: Option<String>,
    /// <p>The release date of the public base image. For private images, this date is the release date of the base image from which the image was created.</p>
    #[serde(rename = "PublicBaseImageReleasedDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub public_base_image_released_date: Option<f64>,
    /// <p>The image starts in the <code>PENDING</code> state. If image creation succeeds, the state is <code>AVAILABLE</code>. If image creation fails, the state is <code>FAILED</code>.</p>
    #[serde(rename = "State")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    /// <p>The reason why the last state change occurred.</p>
    #[serde(rename = "StateChangeReason")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state_change_reason: Option<ImageStateChangeReason>,
    /// <p>Indicates whether the image is public or private.</p>
    #[serde(rename = "Visibility")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub visibility: Option<String>,
}

/// <p>Describes a streaming instance used for editing an image. New images are created from a snapshot through an image builder.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct ImageBuilder {
    /// <p>The version of the AppStream 2.0 agent that is currently being used by this image builder. </p>
    #[serde(rename = "AppstreamAgentVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub appstream_agent_version: Option<String>,
    /// <p>The ARN for the image builder.</p>
    #[serde(rename = "Arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p>The time stamp when the image builder was created.</p>
    #[serde(rename = "CreatedTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_time: Option<f64>,
    /// <p>The description for display.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The image builder name for display.</p>
    #[serde(rename = "DisplayName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    /// <p>The information needed to join a Microsoft Active Directory domain.</p>
    #[serde(rename = "DomainJoinInfo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_join_info: Option<DomainJoinInfo>,
    /// <p>Enables or disables default internet access for the image builder.</p>
    #[serde(rename = "EnableDefaultInternetAccess")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_default_internet_access: Option<bool>,
    /// <p>The ARN of the image from which this builder was created.</p>
    #[serde(rename = "ImageArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_arn: Option<String>,
    /// <p>The image builder errors.</p>
    #[serde(rename = "ImageBuilderErrors")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_builder_errors: Option<Vec<ResourceError>>,
    /// <p>The instance type for the image builder.</p>
    #[serde(rename = "InstanceType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_type: Option<String>,
    /// <p>The name of the image builder.</p>
    #[serde(rename = "Name")]
    pub name: String,
    /// <p>The operating system platform of the image builder.</p>
    #[serde(rename = "Platform")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub platform: Option<String>,
    /// <p>The state of the image builder.</p>
    #[serde(rename = "State")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    /// <p>The reason why the last state change occurred.</p>
    #[serde(rename = "StateChangeReason")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state_change_reason: Option<ImageBuilderStateChangeReason>,
    /// <p>The VPC configuration of the image builder.</p>
    #[serde(rename = "VpcConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_config: Option<VpcConfig>,
}

/// <p>Describes the reason why the last image builder state change occurred.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct ImageBuilderStateChangeReason {
    /// <p>The state change reason code.</p>
    #[serde(rename = "Code")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    /// <p>The state change reason message.</p>
    #[serde(rename = "Message")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

/// <p>Describes the reason why the last image state change occurred.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct ImageStateChangeReason {
    /// <p>The state change reason code.</p>
    #[serde(rename = "Code")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    /// <p>The state change reason message.</p>
    #[serde(rename = "Message")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ListAssociatedFleetsRequest {
    /// <p>The pagination token to use to retrieve the next page of results for this operation. If this value is null, it retrieves the first page.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The name of the stack.</p>
    #[serde(rename = "StackName")]
    pub stack_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct ListAssociatedFleetsResult {
    /// <p>The names of the fleets.</p>
    #[serde(rename = "Names")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub names: Option<Vec<String>>,
    /// <p>The pagination token to use to retrieve the next page of results for this operation. If there are no more pages, this value is null.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ListAssociatedStacksRequest {
    /// <p>The name of the fleet.</p>
    #[serde(rename = "FleetName")]
    pub fleet_name: String,
    /// <p>The pagination token to use to retrieve the next page of results for this operation. If this value is null, it retrieves the first page.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct ListAssociatedStacksResult {
    /// <p>The names of the stacks.</p>
    #[serde(rename = "Names")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub names: Option<Vec<String>>,
    /// <p>The pagination token to use to retrieve the next page of results for this operation. If there are no more pages, this value is null.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ListTagsForResourceRequest {
    /// <p>The Amazon Resource Name (ARN) of the resource.</p>
    #[serde(rename = "ResourceArn")]
    pub resource_arn: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct ListTagsForResourceResponse {
    /// <p>The information about the tags.</p>
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<::std::collections::HashMap<String, String>>,
}

/// <p>Describes a resource error.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct ResourceError {
    /// <p>The error code.</p>
    #[serde(rename = "ErrorCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_code: Option<String>,
    /// <p>The error message.</p>
    #[serde(rename = "ErrorMessage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
    /// <p>The time the error occurred.</p>
    #[serde(rename = "ErrorTimestamp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_timestamp: Option<f64>,
}

/// <p>Describes the credentials for the service account used by the streaming instance to connect to the directory.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ServiceAccountCredentials {
    /// <p>The user name of the account. This account must have the following privileges: create computer objects, join computers to the domain, and change/reset the password on descendant computer objects for the organizational units specified.</p>
    #[serde(rename = "AccountName")]
    pub account_name: String,
    /// <p>The password for the account.</p>
    #[serde(rename = "AccountPassword")]
    pub account_password: String,
}

/// <p>Describes a streaming session.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct Session {
    /// <p>The authentication method. The user is authenticated using a streaming URL (<code>API</code>) or SAML federation (<code>SAML</code>).</p>
    #[serde(rename = "AuthenticationType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authentication_type: Option<String>,
    /// <p>The name of the fleet for the streaming session.</p>
    #[serde(rename = "FleetName")]
    pub fleet_name: String,
    /// <p>The ID of the streaming session.</p>
    #[serde(rename = "Id")]
    pub id: String,
    /// <p>The name of the stack for the streaming session.</p>
    #[serde(rename = "StackName")]
    pub stack_name: String,
    /// <p>The current state of the streaming session.</p>
    #[serde(rename = "State")]
    pub state: String,
    /// <p>The identifier of the user for whom the session was created.</p>
    #[serde(rename = "UserId")]
    pub user_id: String,
}

/// <p>Describes a stack.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct Stack {
    /// <p>The ARN of the stack.</p>
    #[serde(rename = "Arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p>The time the stack was created.</p>
    #[serde(rename = "CreatedTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_time: Option<f64>,
    /// <p>The description for display.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The stack name for display.</p>
    #[serde(rename = "DisplayName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    /// <p>The URL that users are redirected to after they click the Send Feedback link. If no URL is specified, no Send Feedback link is displayed.</p>
    #[serde(rename = "FeedbackURL")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub feedback_url: Option<String>,
    /// <p>The name of the stack.</p>
    #[serde(rename = "Name")]
    pub name: String,
    /// <p>The URL that users are redirected to after their streaming session ends.</p>
    #[serde(rename = "RedirectURL")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub redirect_url: Option<String>,
    /// <p>The errors for the stack.</p>
    #[serde(rename = "StackErrors")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stack_errors: Option<Vec<StackError>>,
    /// <p>The storage connectors to enable.</p>
    #[serde(rename = "StorageConnectors")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage_connectors: Option<Vec<StorageConnector>>,
}

/// <p>Describes a stack error.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct StackError {
    /// <p>The error code.</p>
    #[serde(rename = "ErrorCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_code: Option<String>,
    /// <p>The error message.</p>
    #[serde(rename = "ErrorMessage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct StartFleetRequest {
    /// <p>The name of the fleet.</p>
    #[serde(rename = "Name")]
    pub name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct StartFleetResult {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct StartImageBuilderRequest {
    /// <p>The version of the AppStream 2.0 agent to use for this image builder. To use the latest version of the AppStream 2.0 agent, specify [LATEST]. </p>
    #[serde(rename = "AppstreamAgentVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub appstream_agent_version: Option<String>,
    /// <p>The name of the image builder.</p>
    #[serde(rename = "Name")]
    pub name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct StartImageBuilderResult {
    /// <p>Information about the image builder.</p>
    #[serde(rename = "ImageBuilder")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_builder: Option<ImageBuilder>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct StopFleetRequest {
    /// <p>The name of the fleet.</p>
    #[serde(rename = "Name")]
    pub name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct StopFleetResult {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct StopImageBuilderRequest {
    /// <p>The name of the image builder.</p>
    #[serde(rename = "Name")]
    pub name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct StopImageBuilderResult {
    /// <p>Information about the image builder.</p>
    #[serde(rename = "ImageBuilder")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_builder: Option<ImageBuilder>,
}

/// <p>Describes a storage connector.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StorageConnector {
    /// <p>The type of storage connector.</p>
    #[serde(rename = "ConnectorType")]
    pub connector_type: String,
    /// <p>The ARN of the storage connector.</p>
    #[serde(rename = "ResourceIdentifier")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_identifier: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct TagResourceRequest {
    /// <p>The Amazon Resource Name (ARN) of the resource.</p>
    #[serde(rename = "ResourceArn")]
    pub resource_arn: String,
    /// <p>The tags to associate. A tag is a key-value pair (the value is optional). For example, <code>Environment=Test</code>, or, if you do not specify a value, <code>Environment=</code>. </p> <p>If you do not specify a value, we set the value to an empty string.</p>
    #[serde(rename = "Tags")]
    pub tags: ::std::collections::HashMap<String, String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct TagResourceResponse {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct UntagResourceRequest {
    /// <p>The Amazon Resource Name (ARN) of the resource.</p>
    #[serde(rename = "ResourceArn")]
    pub resource_arn: String,
    /// <p>The tag keys for the tags to disassociate.</p>
    #[serde(rename = "TagKeys")]
    pub tag_keys: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct UntagResourceResponse {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct UpdateDirectoryConfigRequest {
    /// <p>The name of the directory configuration.</p>
    #[serde(rename = "DirectoryName")]
    pub directory_name: String,
    /// <p>The distinguished names of the organizational units for computer accounts.</p>
    #[serde(rename = "OrganizationalUnitDistinguishedNames")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub organizational_unit_distinguished_names: Option<Vec<String>>,
    /// <p>The credentials for the service account used by the streaming instance to connect to the directory.</p>
    #[serde(rename = "ServiceAccountCredentials")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_account_credentials: Option<ServiceAccountCredentials>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct UpdateDirectoryConfigResult {
    /// <p>Information about the directory configuration.</p>
    #[serde(rename = "DirectoryConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub directory_config: Option<DirectoryConfig>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct UpdateFleetRequest {
    /// <p>The fleet attributes to delete.</p>
    #[serde(rename = "AttributesToDelete")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attributes_to_delete: Option<Vec<String>>,
    /// <p>The desired capacity for the fleet.</p>
    #[serde(rename = "ComputeCapacity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compute_capacity: Option<ComputeCapacity>,
    /// <p>The description for display.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The time after disconnection when a session is considered to have ended, in seconds. If a user who was disconnected reconnects within this time interval, the user is connected to their previous session. Specify a value between 60 and 57600.</p>
    #[serde(rename = "DisconnectTimeoutInSeconds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disconnect_timeout_in_seconds: Option<i64>,
    /// <p>The fleet name for display.</p>
    #[serde(rename = "DisplayName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    /// <p>The information needed to join a Microsoft Active Directory domain.</p>
    #[serde(rename = "DomainJoinInfo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_join_info: Option<DomainJoinInfo>,
    /// <p>Enables or disables default internet access for the fleet.</p>
    #[serde(rename = "EnableDefaultInternetAccess")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_default_internet_access: Option<bool>,
    /// <p>The name of the image used to create the fleet.</p>
    #[serde(rename = "ImageName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_name: Option<String>,
    /// <p><p>The instance type to use when launching fleet instances. The following instance types are available:</p> <ul> <li> <p>stream.standard.medium</p> </li> <li> <p>stream.standard.large</p> </li> <li> <p>stream.compute.large</p> </li> <li> <p>stream.compute.xlarge</p> </li> <li> <p>stream.compute.2xlarge</p> </li> <li> <p>stream.compute.4xlarge</p> </li> <li> <p>stream.compute.8xlarge</p> </li> <li> <p>stream.memory.large</p> </li> <li> <p>stream.memory.xlarge</p> </li> <li> <p>stream.memory.2xlarge</p> </li> <li> <p>stream.memory.4xlarge</p> </li> <li> <p>stream.memory.8xlarge</p> </li> <li> <p>stream.graphics-design.large</p> </li> <li> <p>stream.graphics-design.xlarge</p> </li> <li> <p>stream.graphics-design.2xlarge</p> </li> <li> <p>stream.graphics-design.4xlarge</p> </li> <li> <p>stream.graphics-desktop.2xlarge</p> </li> <li> <p>stream.graphics-pro.4xlarge</p> </li> <li> <p>stream.graphics-pro.8xlarge</p> </li> <li> <p>stream.graphics-pro.16xlarge</p> </li> </ul></p>
    #[serde(rename = "InstanceType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_type: Option<String>,
    /// <p>The maximum time that a streaming session can run, in seconds. Specify a value between 600 and 57600.</p>
    #[serde(rename = "MaxUserDurationInSeconds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_user_duration_in_seconds: Option<i64>,
    /// <p>A unique name for the fleet.</p>
    #[serde(rename = "Name")]
    pub name: String,
    /// <p>The VPC configuration for the fleet.</p>
    #[serde(rename = "VpcConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_config: Option<VpcConfig>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct UpdateFleetResult {
    /// <p>Information about the fleet.</p>
    #[serde(rename = "Fleet")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fleet: Option<Fleet>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct UpdateStackRequest {
    /// <p>The stack attributes to delete.</p>
    #[serde(rename = "AttributesToDelete")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attributes_to_delete: Option<Vec<String>>,
    /// <p>The description for display.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The stack name for display.</p>
    #[serde(rename = "DisplayName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    /// <p>The URL that users are redirected to after they click the Send Feedback link. If no URL is specified, no Send Feedback link is displayed.</p>
    #[serde(rename = "FeedbackURL")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub feedback_url: Option<String>,
    /// <p>The name of the stack.</p>
    #[serde(rename = "Name")]
    pub name: String,
    /// <p>The URL that users are redirected to after their streaming session ends.</p>
    #[serde(rename = "RedirectURL")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub redirect_url: Option<String>,
    /// <p>The storage connectors to enable.</p>
    #[serde(rename = "StorageConnectors")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage_connectors: Option<Vec<StorageConnector>>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct UpdateStackResult {
    /// <p>Information about the stack.</p>
    #[serde(rename = "Stack")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stack: Option<Stack>,
}

/// <p>Describes VPC configuration information.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct VpcConfig {
    /// <p>The security groups for the fleet.</p>
    #[serde(rename = "SecurityGroupIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_group_ids: Option<Vec<String>>,
    /// <p>The subnets to which a network interface is established from the fleet instance.</p>
    #[serde(rename = "SubnetIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subnet_ids: Option<Vec<String>>,
}

/// Errors returned by AssociateFleet
#[derive(Debug, PartialEq)]
pub enum AssociateFleetError {
    /// <p>An API error occurred. Wait a few minutes and try again.</p>
    ConcurrentModification(String),
    /// <p>The image does not support storage connectors.</p>
    IncompatibleImage(String),
    /// <p>The resource cannot be created because your AWS account is suspended. For assistance, contact AWS Support. </p>
    InvalidAccountStatus(String),
    /// <p>The requested limit exceeds the permitted limit for an account.</p>
    LimitExceeded(String),
    /// <p>The attempted operation is not permitted.</p>
    OperationNotPermitted(String),
    /// <p>The specified resource was not found.</p>
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

impl AssociateFleetError {
    pub fn from_body(body: &str) -> AssociateFleetError {
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
                        AssociateFleetError::ConcurrentModification(String::from(error_message))
                    }
                    "IncompatibleImageException" => {
                        AssociateFleetError::IncompatibleImage(String::from(error_message))
                    }
                    "InvalidAccountStatusException" => {
                        AssociateFleetError::InvalidAccountStatus(String::from(error_message))
                    }
                    "LimitExceededException" => {
                        AssociateFleetError::LimitExceeded(String::from(error_message))
                    }
                    "OperationNotPermittedException" => {
                        AssociateFleetError::OperationNotPermitted(String::from(error_message))
                    }
                    "ResourceNotFoundException" => {
                        AssociateFleetError::ResourceNotFound(String::from(error_message))
                    }
                    "ValidationException" => {
                        AssociateFleetError::Validation(error_message.to_string())
                    }
                    _ => AssociateFleetError::Unknown(String::from(body)),
                }
            }
            Err(_) => AssociateFleetError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for AssociateFleetError {
    fn from(err: serde_json::error::Error) -> AssociateFleetError {
        AssociateFleetError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for AssociateFleetError {
    fn from(err: CredentialsError) -> AssociateFleetError {
        AssociateFleetError::Credentials(err)
    }
}
impl From<HttpDispatchError> for AssociateFleetError {
    fn from(err: HttpDispatchError) -> AssociateFleetError {
        AssociateFleetError::HttpDispatch(err)
    }
}
impl From<io::Error> for AssociateFleetError {
    fn from(err: io::Error) -> AssociateFleetError {
        AssociateFleetError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for AssociateFleetError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for AssociateFleetError {
    fn description(&self) -> &str {
        match *self {
            AssociateFleetError::ConcurrentModification(ref cause) => cause,
            AssociateFleetError::IncompatibleImage(ref cause) => cause,
            AssociateFleetError::InvalidAccountStatus(ref cause) => cause,
            AssociateFleetError::LimitExceeded(ref cause) => cause,
            AssociateFleetError::OperationNotPermitted(ref cause) => cause,
            AssociateFleetError::ResourceNotFound(ref cause) => cause,
            AssociateFleetError::Validation(ref cause) => cause,
            AssociateFleetError::Credentials(ref err) => err.description(),
            AssociateFleetError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            AssociateFleetError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by CopyImage
#[derive(Debug, PartialEq)]
pub enum CopyImageError {
    /// <p>The image does not support storage connectors.</p>
    IncompatibleImage(String),
    /// <p>The resource cannot be created because your AWS account is suspended. For assistance, contact AWS Support. </p>
    InvalidAccountStatus(String),
    /// <p>The requested limit exceeds the permitted limit for an account.</p>
    LimitExceeded(String),
    /// <p>The specified resource already exists.</p>
    ResourceAlreadyExists(String),
    /// <p>The specified resource exists and is not in use, but isn't available.</p>
    ResourceNotAvailable(String),
    /// <p>The specified resource was not found.</p>
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

impl CopyImageError {
    pub fn from_body(body: &str) -> CopyImageError {
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
                    "IncompatibleImageException" => {
                        CopyImageError::IncompatibleImage(String::from(error_message))
                    }
                    "InvalidAccountStatusException" => {
                        CopyImageError::InvalidAccountStatus(String::from(error_message))
                    }
                    "LimitExceededException" => {
                        CopyImageError::LimitExceeded(String::from(error_message))
                    }
                    "ResourceAlreadyExistsException" => {
                        CopyImageError::ResourceAlreadyExists(String::from(error_message))
                    }
                    "ResourceNotAvailableException" => {
                        CopyImageError::ResourceNotAvailable(String::from(error_message))
                    }
                    "ResourceNotFoundException" => {
                        CopyImageError::ResourceNotFound(String::from(error_message))
                    }
                    "ValidationException" => CopyImageError::Validation(error_message.to_string()),
                    _ => CopyImageError::Unknown(String::from(body)),
                }
            }
            Err(_) => CopyImageError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for CopyImageError {
    fn from(err: serde_json::error::Error) -> CopyImageError {
        CopyImageError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for CopyImageError {
    fn from(err: CredentialsError) -> CopyImageError {
        CopyImageError::Credentials(err)
    }
}
impl From<HttpDispatchError> for CopyImageError {
    fn from(err: HttpDispatchError) -> CopyImageError {
        CopyImageError::HttpDispatch(err)
    }
}
impl From<io::Error> for CopyImageError {
    fn from(err: io::Error) -> CopyImageError {
        CopyImageError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for CopyImageError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CopyImageError {
    fn description(&self) -> &str {
        match *self {
            CopyImageError::IncompatibleImage(ref cause) => cause,
            CopyImageError::InvalidAccountStatus(ref cause) => cause,
            CopyImageError::LimitExceeded(ref cause) => cause,
            CopyImageError::ResourceAlreadyExists(ref cause) => cause,
            CopyImageError::ResourceNotAvailable(ref cause) => cause,
            CopyImageError::ResourceNotFound(ref cause) => cause,
            CopyImageError::Validation(ref cause) => cause,
            CopyImageError::Credentials(ref err) => err.description(),
            CopyImageError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            CopyImageError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by CreateDirectoryConfig
#[derive(Debug, PartialEq)]
pub enum CreateDirectoryConfigError {
    /// <p>The resource cannot be created because your AWS account is suspended. For assistance, contact AWS Support. </p>
    InvalidAccountStatus(String),
    /// <p>The requested limit exceeds the permitted limit for an account.</p>
    LimitExceeded(String),
    /// <p>The specified resource already exists.</p>
    ResourceAlreadyExists(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl CreateDirectoryConfigError {
    pub fn from_body(body: &str) -> CreateDirectoryConfigError {
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
                    "InvalidAccountStatusException" => {
                        CreateDirectoryConfigError::InvalidAccountStatus(String::from(
                            error_message,
                        ))
                    }
                    "LimitExceededException" => {
                        CreateDirectoryConfigError::LimitExceeded(String::from(error_message))
                    }
                    "ResourceAlreadyExistsException" => {
                        CreateDirectoryConfigError::ResourceAlreadyExists(String::from(
                            error_message,
                        ))
                    }
                    "ValidationException" => {
                        CreateDirectoryConfigError::Validation(error_message.to_string())
                    }
                    _ => CreateDirectoryConfigError::Unknown(String::from(body)),
                }
            }
            Err(_) => CreateDirectoryConfigError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for CreateDirectoryConfigError {
    fn from(err: serde_json::error::Error) -> CreateDirectoryConfigError {
        CreateDirectoryConfigError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for CreateDirectoryConfigError {
    fn from(err: CredentialsError) -> CreateDirectoryConfigError {
        CreateDirectoryConfigError::Credentials(err)
    }
}
impl From<HttpDispatchError> for CreateDirectoryConfigError {
    fn from(err: HttpDispatchError) -> CreateDirectoryConfigError {
        CreateDirectoryConfigError::HttpDispatch(err)
    }
}
impl From<io::Error> for CreateDirectoryConfigError {
    fn from(err: io::Error) -> CreateDirectoryConfigError {
        CreateDirectoryConfigError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for CreateDirectoryConfigError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateDirectoryConfigError {
    fn description(&self) -> &str {
        match *self {
            CreateDirectoryConfigError::InvalidAccountStatus(ref cause) => cause,
            CreateDirectoryConfigError::LimitExceeded(ref cause) => cause,
            CreateDirectoryConfigError::ResourceAlreadyExists(ref cause) => cause,
            CreateDirectoryConfigError::Validation(ref cause) => cause,
            CreateDirectoryConfigError::Credentials(ref err) => err.description(),
            CreateDirectoryConfigError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            CreateDirectoryConfigError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by CreateFleet
#[derive(Debug, PartialEq)]
pub enum CreateFleetError {
    /// <p>An API error occurred. Wait a few minutes and try again.</p>
    ConcurrentModification(String),
    /// <p>The image does not support storage connectors.</p>
    IncompatibleImage(String),
    /// <p>The resource cannot be created because your AWS account is suspended. For assistance, contact AWS Support. </p>
    InvalidAccountStatus(String),
    /// <p>Indicates an incorrect combination of parameters, or a missing parameter.</p>
    InvalidParameterCombination(String),
    /// <p>The specified role is invalid.</p>
    InvalidRole(String),
    /// <p>The requested limit exceeds the permitted limit for an account.</p>
    LimitExceeded(String),
    /// <p>The specified resource already exists.</p>
    ResourceAlreadyExists(String),
    /// <p>The specified resource exists and is not in use, but isn't available.</p>
    ResourceNotAvailable(String),
    /// <p>The specified resource was not found.</p>
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

impl CreateFleetError {
    pub fn from_body(body: &str) -> CreateFleetError {
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
                        CreateFleetError::ConcurrentModification(String::from(error_message))
                    }
                    "IncompatibleImageException" => {
                        CreateFleetError::IncompatibleImage(String::from(error_message))
                    }
                    "InvalidAccountStatusException" => {
                        CreateFleetError::InvalidAccountStatus(String::from(error_message))
                    }
                    "InvalidParameterCombinationException" => {
                        CreateFleetError::InvalidParameterCombination(String::from(error_message))
                    }
                    "InvalidRoleException" => {
                        CreateFleetError::InvalidRole(String::from(error_message))
                    }
                    "LimitExceededException" => {
                        CreateFleetError::LimitExceeded(String::from(error_message))
                    }
                    "ResourceAlreadyExistsException" => {
                        CreateFleetError::ResourceAlreadyExists(String::from(error_message))
                    }
                    "ResourceNotAvailableException" => {
                        CreateFleetError::ResourceNotAvailable(String::from(error_message))
                    }
                    "ResourceNotFoundException" => {
                        CreateFleetError::ResourceNotFound(String::from(error_message))
                    }
                    "ValidationException" => {
                        CreateFleetError::Validation(error_message.to_string())
                    }
                    _ => CreateFleetError::Unknown(String::from(body)),
                }
            }
            Err(_) => CreateFleetError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for CreateFleetError {
    fn from(err: serde_json::error::Error) -> CreateFleetError {
        CreateFleetError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for CreateFleetError {
    fn from(err: CredentialsError) -> CreateFleetError {
        CreateFleetError::Credentials(err)
    }
}
impl From<HttpDispatchError> for CreateFleetError {
    fn from(err: HttpDispatchError) -> CreateFleetError {
        CreateFleetError::HttpDispatch(err)
    }
}
impl From<io::Error> for CreateFleetError {
    fn from(err: io::Error) -> CreateFleetError {
        CreateFleetError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for CreateFleetError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateFleetError {
    fn description(&self) -> &str {
        match *self {
            CreateFleetError::ConcurrentModification(ref cause) => cause,
            CreateFleetError::IncompatibleImage(ref cause) => cause,
            CreateFleetError::InvalidAccountStatus(ref cause) => cause,
            CreateFleetError::InvalidParameterCombination(ref cause) => cause,
            CreateFleetError::InvalidRole(ref cause) => cause,
            CreateFleetError::LimitExceeded(ref cause) => cause,
            CreateFleetError::ResourceAlreadyExists(ref cause) => cause,
            CreateFleetError::ResourceNotAvailable(ref cause) => cause,
            CreateFleetError::ResourceNotFound(ref cause) => cause,
            CreateFleetError::Validation(ref cause) => cause,
            CreateFleetError::Credentials(ref err) => err.description(),
            CreateFleetError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            CreateFleetError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by CreateImageBuilder
#[derive(Debug, PartialEq)]
pub enum CreateImageBuilderError {
    /// <p>An API error occurred. Wait a few minutes and try again.</p>
    ConcurrentModification(String),
    /// <p>The image does not support storage connectors.</p>
    IncompatibleImage(String),
    /// <p>The resource cannot be created because your AWS account is suspended. For assistance, contact AWS Support. </p>
    InvalidAccountStatus(String),
    /// <p>Indicates an incorrect combination of parameters, or a missing parameter.</p>
    InvalidParameterCombination(String),
    /// <p>The specified role is invalid.</p>
    InvalidRole(String),
    /// <p>The requested limit exceeds the permitted limit for an account.</p>
    LimitExceeded(String),
    /// <p>The specified resource already exists.</p>
    ResourceAlreadyExists(String),
    /// <p>The specified resource exists and is not in use, but isn't available.</p>
    ResourceNotAvailable(String),
    /// <p>The specified resource was not found.</p>
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

impl CreateImageBuilderError {
    pub fn from_body(body: &str) -> CreateImageBuilderError {
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
                        CreateImageBuilderError::ConcurrentModification(String::from(error_message))
                    }
                    "IncompatibleImageException" => {
                        CreateImageBuilderError::IncompatibleImage(String::from(error_message))
                    }
                    "InvalidAccountStatusException" => {
                        CreateImageBuilderError::InvalidAccountStatus(String::from(error_message))
                    }
                    "InvalidParameterCombinationException" => {
                        CreateImageBuilderError::InvalidParameterCombination(String::from(
                            error_message,
                        ))
                    }
                    "InvalidRoleException" => {
                        CreateImageBuilderError::InvalidRole(String::from(error_message))
                    }
                    "LimitExceededException" => {
                        CreateImageBuilderError::LimitExceeded(String::from(error_message))
                    }
                    "ResourceAlreadyExistsException" => {
                        CreateImageBuilderError::ResourceAlreadyExists(String::from(error_message))
                    }
                    "ResourceNotAvailableException" => {
                        CreateImageBuilderError::ResourceNotAvailable(String::from(error_message))
                    }
                    "ResourceNotFoundException" => {
                        CreateImageBuilderError::ResourceNotFound(String::from(error_message))
                    }
                    "ValidationException" => {
                        CreateImageBuilderError::Validation(error_message.to_string())
                    }
                    _ => CreateImageBuilderError::Unknown(String::from(body)),
                }
            }
            Err(_) => CreateImageBuilderError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for CreateImageBuilderError {
    fn from(err: serde_json::error::Error) -> CreateImageBuilderError {
        CreateImageBuilderError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for CreateImageBuilderError {
    fn from(err: CredentialsError) -> CreateImageBuilderError {
        CreateImageBuilderError::Credentials(err)
    }
}
impl From<HttpDispatchError> for CreateImageBuilderError {
    fn from(err: HttpDispatchError) -> CreateImageBuilderError {
        CreateImageBuilderError::HttpDispatch(err)
    }
}
impl From<io::Error> for CreateImageBuilderError {
    fn from(err: io::Error) -> CreateImageBuilderError {
        CreateImageBuilderError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for CreateImageBuilderError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateImageBuilderError {
    fn description(&self) -> &str {
        match *self {
            CreateImageBuilderError::ConcurrentModification(ref cause) => cause,
            CreateImageBuilderError::IncompatibleImage(ref cause) => cause,
            CreateImageBuilderError::InvalidAccountStatus(ref cause) => cause,
            CreateImageBuilderError::InvalidParameterCombination(ref cause) => cause,
            CreateImageBuilderError::InvalidRole(ref cause) => cause,
            CreateImageBuilderError::LimitExceeded(ref cause) => cause,
            CreateImageBuilderError::ResourceAlreadyExists(ref cause) => cause,
            CreateImageBuilderError::ResourceNotAvailable(ref cause) => cause,
            CreateImageBuilderError::ResourceNotFound(ref cause) => cause,
            CreateImageBuilderError::Validation(ref cause) => cause,
            CreateImageBuilderError::Credentials(ref err) => err.description(),
            CreateImageBuilderError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            CreateImageBuilderError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by CreateImageBuilderStreamingURL
#[derive(Debug, PartialEq)]
pub enum CreateImageBuilderStreamingURLError {
    /// <p>The attempted operation is not permitted.</p>
    OperationNotPermitted(String),
    /// <p>The specified resource was not found.</p>
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

impl CreateImageBuilderStreamingURLError {
    pub fn from_body(body: &str) -> CreateImageBuilderStreamingURLError {
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
                    "OperationNotPermittedException" => {
                        CreateImageBuilderStreamingURLError::OperationNotPermitted(String::from(
                            error_message,
                        ))
                    }
                    "ResourceNotFoundException" => {
                        CreateImageBuilderStreamingURLError::ResourceNotFound(String::from(
                            error_message,
                        ))
                    }
                    "ValidationException" => {
                        CreateImageBuilderStreamingURLError::Validation(error_message.to_string())
                    }
                    _ => CreateImageBuilderStreamingURLError::Unknown(String::from(body)),
                }
            }
            Err(_) => CreateImageBuilderStreamingURLError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for CreateImageBuilderStreamingURLError {
    fn from(err: serde_json::error::Error) -> CreateImageBuilderStreamingURLError {
        CreateImageBuilderStreamingURLError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for CreateImageBuilderStreamingURLError {
    fn from(err: CredentialsError) -> CreateImageBuilderStreamingURLError {
        CreateImageBuilderStreamingURLError::Credentials(err)
    }
}
impl From<HttpDispatchError> for CreateImageBuilderStreamingURLError {
    fn from(err: HttpDispatchError) -> CreateImageBuilderStreamingURLError {
        CreateImageBuilderStreamingURLError::HttpDispatch(err)
    }
}
impl From<io::Error> for CreateImageBuilderStreamingURLError {
    fn from(err: io::Error) -> CreateImageBuilderStreamingURLError {
        CreateImageBuilderStreamingURLError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for CreateImageBuilderStreamingURLError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateImageBuilderStreamingURLError {
    fn description(&self) -> &str {
        match *self {
            CreateImageBuilderStreamingURLError::OperationNotPermitted(ref cause) => cause,
            CreateImageBuilderStreamingURLError::ResourceNotFound(ref cause) => cause,
            CreateImageBuilderStreamingURLError::Validation(ref cause) => cause,
            CreateImageBuilderStreamingURLError::Credentials(ref err) => err.description(),
            CreateImageBuilderStreamingURLError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            CreateImageBuilderStreamingURLError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by CreateStack
#[derive(Debug, PartialEq)]
pub enum CreateStackError {
    /// <p>An API error occurred. Wait a few minutes and try again.</p>
    ConcurrentModification(String),
    /// <p>The resource cannot be created because your AWS account is suspended. For assistance, contact AWS Support. </p>
    InvalidAccountStatus(String),
    /// <p>Indicates an incorrect combination of parameters, or a missing parameter.</p>
    InvalidParameterCombination(String),
    /// <p>The specified role is invalid.</p>
    InvalidRole(String),
    /// <p>The requested limit exceeds the permitted limit for an account.</p>
    LimitExceeded(String),
    /// <p>The specified resource already exists.</p>
    ResourceAlreadyExists(String),
    /// <p>The specified resource was not found.</p>
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

impl CreateStackError {
    pub fn from_body(body: &str) -> CreateStackError {
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
                        CreateStackError::ConcurrentModification(String::from(error_message))
                    }
                    "InvalidAccountStatusException" => {
                        CreateStackError::InvalidAccountStatus(String::from(error_message))
                    }
                    "InvalidParameterCombinationException" => {
                        CreateStackError::InvalidParameterCombination(String::from(error_message))
                    }
                    "InvalidRoleException" => {
                        CreateStackError::InvalidRole(String::from(error_message))
                    }
                    "LimitExceededException" => {
                        CreateStackError::LimitExceeded(String::from(error_message))
                    }
                    "ResourceAlreadyExistsException" => {
                        CreateStackError::ResourceAlreadyExists(String::from(error_message))
                    }
                    "ResourceNotFoundException" => {
                        CreateStackError::ResourceNotFound(String::from(error_message))
                    }
                    "ValidationException" => {
                        CreateStackError::Validation(error_message.to_string())
                    }
                    _ => CreateStackError::Unknown(String::from(body)),
                }
            }
            Err(_) => CreateStackError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for CreateStackError {
    fn from(err: serde_json::error::Error) -> CreateStackError {
        CreateStackError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for CreateStackError {
    fn from(err: CredentialsError) -> CreateStackError {
        CreateStackError::Credentials(err)
    }
}
impl From<HttpDispatchError> for CreateStackError {
    fn from(err: HttpDispatchError) -> CreateStackError {
        CreateStackError::HttpDispatch(err)
    }
}
impl From<io::Error> for CreateStackError {
    fn from(err: io::Error) -> CreateStackError {
        CreateStackError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for CreateStackError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateStackError {
    fn description(&self) -> &str {
        match *self {
            CreateStackError::ConcurrentModification(ref cause) => cause,
            CreateStackError::InvalidAccountStatus(ref cause) => cause,
            CreateStackError::InvalidParameterCombination(ref cause) => cause,
            CreateStackError::InvalidRole(ref cause) => cause,
            CreateStackError::LimitExceeded(ref cause) => cause,
            CreateStackError::ResourceAlreadyExists(ref cause) => cause,
            CreateStackError::ResourceNotFound(ref cause) => cause,
            CreateStackError::Validation(ref cause) => cause,
            CreateStackError::Credentials(ref err) => err.description(),
            CreateStackError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            CreateStackError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by CreateStreamingURL
#[derive(Debug, PartialEq)]
pub enum CreateStreamingURLError {
    /// <p>Indicates an incorrect combination of parameters, or a missing parameter.</p>
    InvalidParameterCombination(String),
    /// <p>The attempted operation is not permitted.</p>
    OperationNotPermitted(String),
    /// <p>The specified resource exists and is not in use, but isn't available.</p>
    ResourceNotAvailable(String),
    /// <p>The specified resource was not found.</p>
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

impl CreateStreamingURLError {
    pub fn from_body(body: &str) -> CreateStreamingURLError {
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
                    "InvalidParameterCombinationException" => {
                        CreateStreamingURLError::InvalidParameterCombination(String::from(
                            error_message,
                        ))
                    }
                    "OperationNotPermittedException" => {
                        CreateStreamingURLError::OperationNotPermitted(String::from(error_message))
                    }
                    "ResourceNotAvailableException" => {
                        CreateStreamingURLError::ResourceNotAvailable(String::from(error_message))
                    }
                    "ResourceNotFoundException" => {
                        CreateStreamingURLError::ResourceNotFound(String::from(error_message))
                    }
                    "ValidationException" => {
                        CreateStreamingURLError::Validation(error_message.to_string())
                    }
                    _ => CreateStreamingURLError::Unknown(String::from(body)),
                }
            }
            Err(_) => CreateStreamingURLError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for CreateStreamingURLError {
    fn from(err: serde_json::error::Error) -> CreateStreamingURLError {
        CreateStreamingURLError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for CreateStreamingURLError {
    fn from(err: CredentialsError) -> CreateStreamingURLError {
        CreateStreamingURLError::Credentials(err)
    }
}
impl From<HttpDispatchError> for CreateStreamingURLError {
    fn from(err: HttpDispatchError) -> CreateStreamingURLError {
        CreateStreamingURLError::HttpDispatch(err)
    }
}
impl From<io::Error> for CreateStreamingURLError {
    fn from(err: io::Error) -> CreateStreamingURLError {
        CreateStreamingURLError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for CreateStreamingURLError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateStreamingURLError {
    fn description(&self) -> &str {
        match *self {
            CreateStreamingURLError::InvalidParameterCombination(ref cause) => cause,
            CreateStreamingURLError::OperationNotPermitted(ref cause) => cause,
            CreateStreamingURLError::ResourceNotAvailable(ref cause) => cause,
            CreateStreamingURLError::ResourceNotFound(ref cause) => cause,
            CreateStreamingURLError::Validation(ref cause) => cause,
            CreateStreamingURLError::Credentials(ref err) => err.description(),
            CreateStreamingURLError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            CreateStreamingURLError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DeleteDirectoryConfig
#[derive(Debug, PartialEq)]
pub enum DeleteDirectoryConfigError {
    /// <p>The specified resource is in use.</p>
    ResourceInUse(String),
    /// <p>The specified resource was not found.</p>
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

impl DeleteDirectoryConfigError {
    pub fn from_body(body: &str) -> DeleteDirectoryConfigError {
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
                    "ResourceInUseException" => {
                        DeleteDirectoryConfigError::ResourceInUse(String::from(error_message))
                    }
                    "ResourceNotFoundException" => {
                        DeleteDirectoryConfigError::ResourceNotFound(String::from(error_message))
                    }
                    "ValidationException" => {
                        DeleteDirectoryConfigError::Validation(error_message.to_string())
                    }
                    _ => DeleteDirectoryConfigError::Unknown(String::from(body)),
                }
            }
            Err(_) => DeleteDirectoryConfigError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DeleteDirectoryConfigError {
    fn from(err: serde_json::error::Error) -> DeleteDirectoryConfigError {
        DeleteDirectoryConfigError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DeleteDirectoryConfigError {
    fn from(err: CredentialsError) -> DeleteDirectoryConfigError {
        DeleteDirectoryConfigError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DeleteDirectoryConfigError {
    fn from(err: HttpDispatchError) -> DeleteDirectoryConfigError {
        DeleteDirectoryConfigError::HttpDispatch(err)
    }
}
impl From<io::Error> for DeleteDirectoryConfigError {
    fn from(err: io::Error) -> DeleteDirectoryConfigError {
        DeleteDirectoryConfigError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DeleteDirectoryConfigError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteDirectoryConfigError {
    fn description(&self) -> &str {
        match *self {
            DeleteDirectoryConfigError::ResourceInUse(ref cause) => cause,
            DeleteDirectoryConfigError::ResourceNotFound(ref cause) => cause,
            DeleteDirectoryConfigError::Validation(ref cause) => cause,
            DeleteDirectoryConfigError::Credentials(ref err) => err.description(),
            DeleteDirectoryConfigError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DeleteDirectoryConfigError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DeleteFleet
#[derive(Debug, PartialEq)]
pub enum DeleteFleetError {
    /// <p>An API error occurred. Wait a few minutes and try again.</p>
    ConcurrentModification(String),
    /// <p>The specified resource is in use.</p>
    ResourceInUse(String),
    /// <p>The specified resource was not found.</p>
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

impl DeleteFleetError {
    pub fn from_body(body: &str) -> DeleteFleetError {
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
                        DeleteFleetError::ConcurrentModification(String::from(error_message))
                    }
                    "ResourceInUseException" => {
                        DeleteFleetError::ResourceInUse(String::from(error_message))
                    }
                    "ResourceNotFoundException" => {
                        DeleteFleetError::ResourceNotFound(String::from(error_message))
                    }
                    "ValidationException" => {
                        DeleteFleetError::Validation(error_message.to_string())
                    }
                    _ => DeleteFleetError::Unknown(String::from(body)),
                }
            }
            Err(_) => DeleteFleetError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DeleteFleetError {
    fn from(err: serde_json::error::Error) -> DeleteFleetError {
        DeleteFleetError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DeleteFleetError {
    fn from(err: CredentialsError) -> DeleteFleetError {
        DeleteFleetError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DeleteFleetError {
    fn from(err: HttpDispatchError) -> DeleteFleetError {
        DeleteFleetError::HttpDispatch(err)
    }
}
impl From<io::Error> for DeleteFleetError {
    fn from(err: io::Error) -> DeleteFleetError {
        DeleteFleetError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DeleteFleetError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteFleetError {
    fn description(&self) -> &str {
        match *self {
            DeleteFleetError::ConcurrentModification(ref cause) => cause,
            DeleteFleetError::ResourceInUse(ref cause) => cause,
            DeleteFleetError::ResourceNotFound(ref cause) => cause,
            DeleteFleetError::Validation(ref cause) => cause,
            DeleteFleetError::Credentials(ref err) => err.description(),
            DeleteFleetError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            DeleteFleetError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DeleteImage
#[derive(Debug, PartialEq)]
pub enum DeleteImageError {
    /// <p>An API error occurred. Wait a few minutes and try again.</p>
    ConcurrentModification(String),
    /// <p>The attempted operation is not permitted.</p>
    OperationNotPermitted(String),
    /// <p>The specified resource is in use.</p>
    ResourceInUse(String),
    /// <p>The specified resource was not found.</p>
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

impl DeleteImageError {
    pub fn from_body(body: &str) -> DeleteImageError {
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
                        DeleteImageError::ConcurrentModification(String::from(error_message))
                    }
                    "OperationNotPermittedException" => {
                        DeleteImageError::OperationNotPermitted(String::from(error_message))
                    }
                    "ResourceInUseException" => {
                        DeleteImageError::ResourceInUse(String::from(error_message))
                    }
                    "ResourceNotFoundException" => {
                        DeleteImageError::ResourceNotFound(String::from(error_message))
                    }
                    "ValidationException" => {
                        DeleteImageError::Validation(error_message.to_string())
                    }
                    _ => DeleteImageError::Unknown(String::from(body)),
                }
            }
            Err(_) => DeleteImageError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DeleteImageError {
    fn from(err: serde_json::error::Error) -> DeleteImageError {
        DeleteImageError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DeleteImageError {
    fn from(err: CredentialsError) -> DeleteImageError {
        DeleteImageError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DeleteImageError {
    fn from(err: HttpDispatchError) -> DeleteImageError {
        DeleteImageError::HttpDispatch(err)
    }
}
impl From<io::Error> for DeleteImageError {
    fn from(err: io::Error) -> DeleteImageError {
        DeleteImageError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DeleteImageError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteImageError {
    fn description(&self) -> &str {
        match *self {
            DeleteImageError::ConcurrentModification(ref cause) => cause,
            DeleteImageError::OperationNotPermitted(ref cause) => cause,
            DeleteImageError::ResourceInUse(ref cause) => cause,
            DeleteImageError::ResourceNotFound(ref cause) => cause,
            DeleteImageError::Validation(ref cause) => cause,
            DeleteImageError::Credentials(ref err) => err.description(),
            DeleteImageError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            DeleteImageError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DeleteImageBuilder
#[derive(Debug, PartialEq)]
pub enum DeleteImageBuilderError {
    /// <p>An API error occurred. Wait a few minutes and try again.</p>
    ConcurrentModification(String),
    /// <p>The attempted operation is not permitted.</p>
    OperationNotPermitted(String),
    /// <p>The specified resource was not found.</p>
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

impl DeleteImageBuilderError {
    pub fn from_body(body: &str) -> DeleteImageBuilderError {
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
                        DeleteImageBuilderError::ConcurrentModification(String::from(error_message))
                    }
                    "OperationNotPermittedException" => {
                        DeleteImageBuilderError::OperationNotPermitted(String::from(error_message))
                    }
                    "ResourceNotFoundException" => {
                        DeleteImageBuilderError::ResourceNotFound(String::from(error_message))
                    }
                    "ValidationException" => {
                        DeleteImageBuilderError::Validation(error_message.to_string())
                    }
                    _ => DeleteImageBuilderError::Unknown(String::from(body)),
                }
            }
            Err(_) => DeleteImageBuilderError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DeleteImageBuilderError {
    fn from(err: serde_json::error::Error) -> DeleteImageBuilderError {
        DeleteImageBuilderError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DeleteImageBuilderError {
    fn from(err: CredentialsError) -> DeleteImageBuilderError {
        DeleteImageBuilderError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DeleteImageBuilderError {
    fn from(err: HttpDispatchError) -> DeleteImageBuilderError {
        DeleteImageBuilderError::HttpDispatch(err)
    }
}
impl From<io::Error> for DeleteImageBuilderError {
    fn from(err: io::Error) -> DeleteImageBuilderError {
        DeleteImageBuilderError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DeleteImageBuilderError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteImageBuilderError {
    fn description(&self) -> &str {
        match *self {
            DeleteImageBuilderError::ConcurrentModification(ref cause) => cause,
            DeleteImageBuilderError::OperationNotPermitted(ref cause) => cause,
            DeleteImageBuilderError::ResourceNotFound(ref cause) => cause,
            DeleteImageBuilderError::Validation(ref cause) => cause,
            DeleteImageBuilderError::Credentials(ref err) => err.description(),
            DeleteImageBuilderError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DeleteImageBuilderError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DeleteStack
#[derive(Debug, PartialEq)]
pub enum DeleteStackError {
    /// <p>An API error occurred. Wait a few minutes and try again.</p>
    ConcurrentModification(String),
    /// <p>The specified resource is in use.</p>
    ResourceInUse(String),
    /// <p>The specified resource was not found.</p>
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

impl DeleteStackError {
    pub fn from_body(body: &str) -> DeleteStackError {
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
                        DeleteStackError::ConcurrentModification(String::from(error_message))
                    }
                    "ResourceInUseException" => {
                        DeleteStackError::ResourceInUse(String::from(error_message))
                    }
                    "ResourceNotFoundException" => {
                        DeleteStackError::ResourceNotFound(String::from(error_message))
                    }
                    "ValidationException" => {
                        DeleteStackError::Validation(error_message.to_string())
                    }
                    _ => DeleteStackError::Unknown(String::from(body)),
                }
            }
            Err(_) => DeleteStackError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DeleteStackError {
    fn from(err: serde_json::error::Error) -> DeleteStackError {
        DeleteStackError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DeleteStackError {
    fn from(err: CredentialsError) -> DeleteStackError {
        DeleteStackError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DeleteStackError {
    fn from(err: HttpDispatchError) -> DeleteStackError {
        DeleteStackError::HttpDispatch(err)
    }
}
impl From<io::Error> for DeleteStackError {
    fn from(err: io::Error) -> DeleteStackError {
        DeleteStackError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DeleteStackError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteStackError {
    fn description(&self) -> &str {
        match *self {
            DeleteStackError::ConcurrentModification(ref cause) => cause,
            DeleteStackError::ResourceInUse(ref cause) => cause,
            DeleteStackError::ResourceNotFound(ref cause) => cause,
            DeleteStackError::Validation(ref cause) => cause,
            DeleteStackError::Credentials(ref err) => err.description(),
            DeleteStackError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            DeleteStackError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DescribeDirectoryConfigs
#[derive(Debug, PartialEq)]
pub enum DescribeDirectoryConfigsError {
    /// <p>The specified resource was not found.</p>
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

impl DescribeDirectoryConfigsError {
    pub fn from_body(body: &str) -> DescribeDirectoryConfigsError {
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
                        DescribeDirectoryConfigsError::ResourceNotFound(String::from(error_message))
                    }
                    "ValidationException" => {
                        DescribeDirectoryConfigsError::Validation(error_message.to_string())
                    }
                    _ => DescribeDirectoryConfigsError::Unknown(String::from(body)),
                }
            }
            Err(_) => DescribeDirectoryConfigsError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DescribeDirectoryConfigsError {
    fn from(err: serde_json::error::Error) -> DescribeDirectoryConfigsError {
        DescribeDirectoryConfigsError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DescribeDirectoryConfigsError {
    fn from(err: CredentialsError) -> DescribeDirectoryConfigsError {
        DescribeDirectoryConfigsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DescribeDirectoryConfigsError {
    fn from(err: HttpDispatchError) -> DescribeDirectoryConfigsError {
        DescribeDirectoryConfigsError::HttpDispatch(err)
    }
}
impl From<io::Error> for DescribeDirectoryConfigsError {
    fn from(err: io::Error) -> DescribeDirectoryConfigsError {
        DescribeDirectoryConfigsError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DescribeDirectoryConfigsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeDirectoryConfigsError {
    fn description(&self) -> &str {
        match *self {
            DescribeDirectoryConfigsError::ResourceNotFound(ref cause) => cause,
            DescribeDirectoryConfigsError::Validation(ref cause) => cause,
            DescribeDirectoryConfigsError::Credentials(ref err) => err.description(),
            DescribeDirectoryConfigsError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DescribeDirectoryConfigsError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DescribeFleets
#[derive(Debug, PartialEq)]
pub enum DescribeFleetsError {
    /// <p>The specified resource was not found.</p>
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

impl DescribeFleetsError {
    pub fn from_body(body: &str) -> DescribeFleetsError {
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
                        DescribeFleetsError::ResourceNotFound(String::from(error_message))
                    }
                    "ValidationException" => {
                        DescribeFleetsError::Validation(error_message.to_string())
                    }
                    _ => DescribeFleetsError::Unknown(String::from(body)),
                }
            }
            Err(_) => DescribeFleetsError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DescribeFleetsError {
    fn from(err: serde_json::error::Error) -> DescribeFleetsError {
        DescribeFleetsError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DescribeFleetsError {
    fn from(err: CredentialsError) -> DescribeFleetsError {
        DescribeFleetsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DescribeFleetsError {
    fn from(err: HttpDispatchError) -> DescribeFleetsError {
        DescribeFleetsError::HttpDispatch(err)
    }
}
impl From<io::Error> for DescribeFleetsError {
    fn from(err: io::Error) -> DescribeFleetsError {
        DescribeFleetsError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DescribeFleetsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeFleetsError {
    fn description(&self) -> &str {
        match *self {
            DescribeFleetsError::ResourceNotFound(ref cause) => cause,
            DescribeFleetsError::Validation(ref cause) => cause,
            DescribeFleetsError::Credentials(ref err) => err.description(),
            DescribeFleetsError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            DescribeFleetsError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DescribeImageBuilders
#[derive(Debug, PartialEq)]
pub enum DescribeImageBuildersError {
    /// <p>The specified resource was not found.</p>
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

impl DescribeImageBuildersError {
    pub fn from_body(body: &str) -> DescribeImageBuildersError {
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
                        DescribeImageBuildersError::ResourceNotFound(String::from(error_message))
                    }
                    "ValidationException" => {
                        DescribeImageBuildersError::Validation(error_message.to_string())
                    }
                    _ => DescribeImageBuildersError::Unknown(String::from(body)),
                }
            }
            Err(_) => DescribeImageBuildersError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DescribeImageBuildersError {
    fn from(err: serde_json::error::Error) -> DescribeImageBuildersError {
        DescribeImageBuildersError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DescribeImageBuildersError {
    fn from(err: CredentialsError) -> DescribeImageBuildersError {
        DescribeImageBuildersError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DescribeImageBuildersError {
    fn from(err: HttpDispatchError) -> DescribeImageBuildersError {
        DescribeImageBuildersError::HttpDispatch(err)
    }
}
impl From<io::Error> for DescribeImageBuildersError {
    fn from(err: io::Error) -> DescribeImageBuildersError {
        DescribeImageBuildersError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DescribeImageBuildersError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeImageBuildersError {
    fn description(&self) -> &str {
        match *self {
            DescribeImageBuildersError::ResourceNotFound(ref cause) => cause,
            DescribeImageBuildersError::Validation(ref cause) => cause,
            DescribeImageBuildersError::Credentials(ref err) => err.description(),
            DescribeImageBuildersError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DescribeImageBuildersError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DescribeImages
#[derive(Debug, PartialEq)]
pub enum DescribeImagesError {
    /// <p>The specified resource was not found.</p>
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

impl DescribeImagesError {
    pub fn from_body(body: &str) -> DescribeImagesError {
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
                        DescribeImagesError::ResourceNotFound(String::from(error_message))
                    }
                    "ValidationException" => {
                        DescribeImagesError::Validation(error_message.to_string())
                    }
                    _ => DescribeImagesError::Unknown(String::from(body)),
                }
            }
            Err(_) => DescribeImagesError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DescribeImagesError {
    fn from(err: serde_json::error::Error) -> DescribeImagesError {
        DescribeImagesError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DescribeImagesError {
    fn from(err: CredentialsError) -> DescribeImagesError {
        DescribeImagesError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DescribeImagesError {
    fn from(err: HttpDispatchError) -> DescribeImagesError {
        DescribeImagesError::HttpDispatch(err)
    }
}
impl From<io::Error> for DescribeImagesError {
    fn from(err: io::Error) -> DescribeImagesError {
        DescribeImagesError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DescribeImagesError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeImagesError {
    fn description(&self) -> &str {
        match *self {
            DescribeImagesError::ResourceNotFound(ref cause) => cause,
            DescribeImagesError::Validation(ref cause) => cause,
            DescribeImagesError::Credentials(ref err) => err.description(),
            DescribeImagesError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            DescribeImagesError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DescribeSessions
#[derive(Debug, PartialEq)]
pub enum DescribeSessionsError {
    /// <p>Indicates an incorrect combination of parameters, or a missing parameter.</p>
    InvalidParameterCombination(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl DescribeSessionsError {
    pub fn from_body(body: &str) -> DescribeSessionsError {
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
                    "InvalidParameterCombinationException" => {
                        DescribeSessionsError::InvalidParameterCombination(String::from(
                            error_message,
                        ))
                    }
                    "ValidationException" => {
                        DescribeSessionsError::Validation(error_message.to_string())
                    }
                    _ => DescribeSessionsError::Unknown(String::from(body)),
                }
            }
            Err(_) => DescribeSessionsError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DescribeSessionsError {
    fn from(err: serde_json::error::Error) -> DescribeSessionsError {
        DescribeSessionsError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DescribeSessionsError {
    fn from(err: CredentialsError) -> DescribeSessionsError {
        DescribeSessionsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DescribeSessionsError {
    fn from(err: HttpDispatchError) -> DescribeSessionsError {
        DescribeSessionsError::HttpDispatch(err)
    }
}
impl From<io::Error> for DescribeSessionsError {
    fn from(err: io::Error) -> DescribeSessionsError {
        DescribeSessionsError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DescribeSessionsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeSessionsError {
    fn description(&self) -> &str {
        match *self {
            DescribeSessionsError::InvalidParameterCombination(ref cause) => cause,
            DescribeSessionsError::Validation(ref cause) => cause,
            DescribeSessionsError::Credentials(ref err) => err.description(),
            DescribeSessionsError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            DescribeSessionsError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DescribeStacks
#[derive(Debug, PartialEq)]
pub enum DescribeStacksError {
    /// <p>The specified resource was not found.</p>
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

impl DescribeStacksError {
    pub fn from_body(body: &str) -> DescribeStacksError {
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
                        DescribeStacksError::ResourceNotFound(String::from(error_message))
                    }
                    "ValidationException" => {
                        DescribeStacksError::Validation(error_message.to_string())
                    }
                    _ => DescribeStacksError::Unknown(String::from(body)),
                }
            }
            Err(_) => DescribeStacksError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DescribeStacksError {
    fn from(err: serde_json::error::Error) -> DescribeStacksError {
        DescribeStacksError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DescribeStacksError {
    fn from(err: CredentialsError) -> DescribeStacksError {
        DescribeStacksError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DescribeStacksError {
    fn from(err: HttpDispatchError) -> DescribeStacksError {
        DescribeStacksError::HttpDispatch(err)
    }
}
impl From<io::Error> for DescribeStacksError {
    fn from(err: io::Error) -> DescribeStacksError {
        DescribeStacksError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DescribeStacksError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeStacksError {
    fn description(&self) -> &str {
        match *self {
            DescribeStacksError::ResourceNotFound(ref cause) => cause,
            DescribeStacksError::Validation(ref cause) => cause,
            DescribeStacksError::Credentials(ref err) => err.description(),
            DescribeStacksError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            DescribeStacksError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DisassociateFleet
#[derive(Debug, PartialEq)]
pub enum DisassociateFleetError {
    /// <p>An API error occurred. Wait a few minutes and try again.</p>
    ConcurrentModification(String),
    /// <p>The specified resource is in use.</p>
    ResourceInUse(String),
    /// <p>The specified resource was not found.</p>
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

impl DisassociateFleetError {
    pub fn from_body(body: &str) -> DisassociateFleetError {
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
                        DisassociateFleetError::ConcurrentModification(String::from(error_message))
                    }
                    "ResourceInUseException" => {
                        DisassociateFleetError::ResourceInUse(String::from(error_message))
                    }
                    "ResourceNotFoundException" => {
                        DisassociateFleetError::ResourceNotFound(String::from(error_message))
                    }
                    "ValidationException" => {
                        DisassociateFleetError::Validation(error_message.to_string())
                    }
                    _ => DisassociateFleetError::Unknown(String::from(body)),
                }
            }
            Err(_) => DisassociateFleetError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DisassociateFleetError {
    fn from(err: serde_json::error::Error) -> DisassociateFleetError {
        DisassociateFleetError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DisassociateFleetError {
    fn from(err: CredentialsError) -> DisassociateFleetError {
        DisassociateFleetError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DisassociateFleetError {
    fn from(err: HttpDispatchError) -> DisassociateFleetError {
        DisassociateFleetError::HttpDispatch(err)
    }
}
impl From<io::Error> for DisassociateFleetError {
    fn from(err: io::Error) -> DisassociateFleetError {
        DisassociateFleetError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DisassociateFleetError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DisassociateFleetError {
    fn description(&self) -> &str {
        match *self {
            DisassociateFleetError::ConcurrentModification(ref cause) => cause,
            DisassociateFleetError::ResourceInUse(ref cause) => cause,
            DisassociateFleetError::ResourceNotFound(ref cause) => cause,
            DisassociateFleetError::Validation(ref cause) => cause,
            DisassociateFleetError::Credentials(ref err) => err.description(),
            DisassociateFleetError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DisassociateFleetError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by ExpireSession
#[derive(Debug, PartialEq)]
pub enum ExpireSessionError {
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl ExpireSessionError {
    pub fn from_body(body: &str) -> ExpireSessionError {
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
                        ExpireSessionError::Validation(error_message.to_string())
                    }
                    _ => ExpireSessionError::Unknown(String::from(body)),
                }
            }
            Err(_) => ExpireSessionError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for ExpireSessionError {
    fn from(err: serde_json::error::Error) -> ExpireSessionError {
        ExpireSessionError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for ExpireSessionError {
    fn from(err: CredentialsError) -> ExpireSessionError {
        ExpireSessionError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ExpireSessionError {
    fn from(err: HttpDispatchError) -> ExpireSessionError {
        ExpireSessionError::HttpDispatch(err)
    }
}
impl From<io::Error> for ExpireSessionError {
    fn from(err: io::Error) -> ExpireSessionError {
        ExpireSessionError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for ExpireSessionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ExpireSessionError {
    fn description(&self) -> &str {
        match *self {
            ExpireSessionError::Validation(ref cause) => cause,
            ExpireSessionError::Credentials(ref err) => err.description(),
            ExpireSessionError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            ExpireSessionError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by ListAssociatedFleets
#[derive(Debug, PartialEq)]
pub enum ListAssociatedFleetsError {
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl ListAssociatedFleetsError {
    pub fn from_body(body: &str) -> ListAssociatedFleetsError {
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
                        ListAssociatedFleetsError::Validation(error_message.to_string())
                    }
                    _ => ListAssociatedFleetsError::Unknown(String::from(body)),
                }
            }
            Err(_) => ListAssociatedFleetsError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for ListAssociatedFleetsError {
    fn from(err: serde_json::error::Error) -> ListAssociatedFleetsError {
        ListAssociatedFleetsError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for ListAssociatedFleetsError {
    fn from(err: CredentialsError) -> ListAssociatedFleetsError {
        ListAssociatedFleetsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ListAssociatedFleetsError {
    fn from(err: HttpDispatchError) -> ListAssociatedFleetsError {
        ListAssociatedFleetsError::HttpDispatch(err)
    }
}
impl From<io::Error> for ListAssociatedFleetsError {
    fn from(err: io::Error) -> ListAssociatedFleetsError {
        ListAssociatedFleetsError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for ListAssociatedFleetsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListAssociatedFleetsError {
    fn description(&self) -> &str {
        match *self {
            ListAssociatedFleetsError::Validation(ref cause) => cause,
            ListAssociatedFleetsError::Credentials(ref err) => err.description(),
            ListAssociatedFleetsError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            ListAssociatedFleetsError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by ListAssociatedStacks
#[derive(Debug, PartialEq)]
pub enum ListAssociatedStacksError {
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl ListAssociatedStacksError {
    pub fn from_body(body: &str) -> ListAssociatedStacksError {
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
                        ListAssociatedStacksError::Validation(error_message.to_string())
                    }
                    _ => ListAssociatedStacksError::Unknown(String::from(body)),
                }
            }
            Err(_) => ListAssociatedStacksError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for ListAssociatedStacksError {
    fn from(err: serde_json::error::Error) -> ListAssociatedStacksError {
        ListAssociatedStacksError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for ListAssociatedStacksError {
    fn from(err: CredentialsError) -> ListAssociatedStacksError {
        ListAssociatedStacksError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ListAssociatedStacksError {
    fn from(err: HttpDispatchError) -> ListAssociatedStacksError {
        ListAssociatedStacksError::HttpDispatch(err)
    }
}
impl From<io::Error> for ListAssociatedStacksError {
    fn from(err: io::Error) -> ListAssociatedStacksError {
        ListAssociatedStacksError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for ListAssociatedStacksError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListAssociatedStacksError {
    fn description(&self) -> &str {
        match *self {
            ListAssociatedStacksError::Validation(ref cause) => cause,
            ListAssociatedStacksError::Credentials(ref err) => err.description(),
            ListAssociatedStacksError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            ListAssociatedStacksError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by ListTagsForResource
#[derive(Debug, PartialEq)]
pub enum ListTagsForResourceError {
    /// <p>The specified resource was not found.</p>
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

impl ListTagsForResourceError {
    pub fn from_body(body: &str) -> ListTagsForResourceError {
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
                        ListTagsForResourceError::ResourceNotFound(String::from(error_message))
                    }
                    "ValidationException" => {
                        ListTagsForResourceError::Validation(error_message.to_string())
                    }
                    _ => ListTagsForResourceError::Unknown(String::from(body)),
                }
            }
            Err(_) => ListTagsForResourceError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for ListTagsForResourceError {
    fn from(err: serde_json::error::Error) -> ListTagsForResourceError {
        ListTagsForResourceError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for ListTagsForResourceError {
    fn from(err: CredentialsError) -> ListTagsForResourceError {
        ListTagsForResourceError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ListTagsForResourceError {
    fn from(err: HttpDispatchError) -> ListTagsForResourceError {
        ListTagsForResourceError::HttpDispatch(err)
    }
}
impl From<io::Error> for ListTagsForResourceError {
    fn from(err: io::Error) -> ListTagsForResourceError {
        ListTagsForResourceError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for ListTagsForResourceError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListTagsForResourceError {
    fn description(&self) -> &str {
        match *self {
            ListTagsForResourceError::ResourceNotFound(ref cause) => cause,
            ListTagsForResourceError::Validation(ref cause) => cause,
            ListTagsForResourceError::Credentials(ref err) => err.description(),
            ListTagsForResourceError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            ListTagsForResourceError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by StartFleet
#[derive(Debug, PartialEq)]
pub enum StartFleetError {
    /// <p>An API error occurred. Wait a few minutes and try again.</p>
    ConcurrentModification(String),
    /// <p>The resource cannot be created because your AWS account is suspended. For assistance, contact AWS Support. </p>
    InvalidAccountStatus(String),
    /// <p>The requested limit exceeds the permitted limit for an account.</p>
    LimitExceeded(String),
    /// <p>The attempted operation is not permitted.</p>
    OperationNotPermitted(String),
    /// <p>The specified resource was not found.</p>
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

impl StartFleetError {
    pub fn from_body(body: &str) -> StartFleetError {
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
                        StartFleetError::ConcurrentModification(String::from(error_message))
                    }
                    "InvalidAccountStatusException" => {
                        StartFleetError::InvalidAccountStatus(String::from(error_message))
                    }
                    "LimitExceededException" => {
                        StartFleetError::LimitExceeded(String::from(error_message))
                    }
                    "OperationNotPermittedException" => {
                        StartFleetError::OperationNotPermitted(String::from(error_message))
                    }
                    "ResourceNotFoundException" => {
                        StartFleetError::ResourceNotFound(String::from(error_message))
                    }
                    "ValidationException" => StartFleetError::Validation(error_message.to_string()),
                    _ => StartFleetError::Unknown(String::from(body)),
                }
            }
            Err(_) => StartFleetError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for StartFleetError {
    fn from(err: serde_json::error::Error) -> StartFleetError {
        StartFleetError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for StartFleetError {
    fn from(err: CredentialsError) -> StartFleetError {
        StartFleetError::Credentials(err)
    }
}
impl From<HttpDispatchError> for StartFleetError {
    fn from(err: HttpDispatchError) -> StartFleetError {
        StartFleetError::HttpDispatch(err)
    }
}
impl From<io::Error> for StartFleetError {
    fn from(err: io::Error) -> StartFleetError {
        StartFleetError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for StartFleetError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for StartFleetError {
    fn description(&self) -> &str {
        match *self {
            StartFleetError::ConcurrentModification(ref cause) => cause,
            StartFleetError::InvalidAccountStatus(ref cause) => cause,
            StartFleetError::LimitExceeded(ref cause) => cause,
            StartFleetError::OperationNotPermitted(ref cause) => cause,
            StartFleetError::ResourceNotFound(ref cause) => cause,
            StartFleetError::Validation(ref cause) => cause,
            StartFleetError::Credentials(ref err) => err.description(),
            StartFleetError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            StartFleetError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by StartImageBuilder
#[derive(Debug, PartialEq)]
pub enum StartImageBuilderError {
    /// <p>An API error occurred. Wait a few minutes and try again.</p>
    ConcurrentModification(String),
    /// <p>The image does not support storage connectors.</p>
    IncompatibleImage(String),
    /// <p>The resource cannot be created because your AWS account is suspended. For assistance, contact AWS Support. </p>
    InvalidAccountStatus(String),
    /// <p>The specified resource exists and is not in use, but isn't available.</p>
    ResourceNotAvailable(String),
    /// <p>The specified resource was not found.</p>
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

impl StartImageBuilderError {
    pub fn from_body(body: &str) -> StartImageBuilderError {
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
                        StartImageBuilderError::ConcurrentModification(String::from(error_message))
                    }
                    "IncompatibleImageException" => {
                        StartImageBuilderError::IncompatibleImage(String::from(error_message))
                    }
                    "InvalidAccountStatusException" => {
                        StartImageBuilderError::InvalidAccountStatus(String::from(error_message))
                    }
                    "ResourceNotAvailableException" => {
                        StartImageBuilderError::ResourceNotAvailable(String::from(error_message))
                    }
                    "ResourceNotFoundException" => {
                        StartImageBuilderError::ResourceNotFound(String::from(error_message))
                    }
                    "ValidationException" => {
                        StartImageBuilderError::Validation(error_message.to_string())
                    }
                    _ => StartImageBuilderError::Unknown(String::from(body)),
                }
            }
            Err(_) => StartImageBuilderError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for StartImageBuilderError {
    fn from(err: serde_json::error::Error) -> StartImageBuilderError {
        StartImageBuilderError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for StartImageBuilderError {
    fn from(err: CredentialsError) -> StartImageBuilderError {
        StartImageBuilderError::Credentials(err)
    }
}
impl From<HttpDispatchError> for StartImageBuilderError {
    fn from(err: HttpDispatchError) -> StartImageBuilderError {
        StartImageBuilderError::HttpDispatch(err)
    }
}
impl From<io::Error> for StartImageBuilderError {
    fn from(err: io::Error) -> StartImageBuilderError {
        StartImageBuilderError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for StartImageBuilderError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for StartImageBuilderError {
    fn description(&self) -> &str {
        match *self {
            StartImageBuilderError::ConcurrentModification(ref cause) => cause,
            StartImageBuilderError::IncompatibleImage(ref cause) => cause,
            StartImageBuilderError::InvalidAccountStatus(ref cause) => cause,
            StartImageBuilderError::ResourceNotAvailable(ref cause) => cause,
            StartImageBuilderError::ResourceNotFound(ref cause) => cause,
            StartImageBuilderError::Validation(ref cause) => cause,
            StartImageBuilderError::Credentials(ref err) => err.description(),
            StartImageBuilderError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            StartImageBuilderError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by StopFleet
#[derive(Debug, PartialEq)]
pub enum StopFleetError {
    /// <p>An API error occurred. Wait a few minutes and try again.</p>
    ConcurrentModification(String),
    /// <p>The specified resource was not found.</p>
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

impl StopFleetError {
    pub fn from_body(body: &str) -> StopFleetError {
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
                        StopFleetError::ConcurrentModification(String::from(error_message))
                    }
                    "ResourceNotFoundException" => {
                        StopFleetError::ResourceNotFound(String::from(error_message))
                    }
                    "ValidationException" => StopFleetError::Validation(error_message.to_string()),
                    _ => StopFleetError::Unknown(String::from(body)),
                }
            }
            Err(_) => StopFleetError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for StopFleetError {
    fn from(err: serde_json::error::Error) -> StopFleetError {
        StopFleetError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for StopFleetError {
    fn from(err: CredentialsError) -> StopFleetError {
        StopFleetError::Credentials(err)
    }
}
impl From<HttpDispatchError> for StopFleetError {
    fn from(err: HttpDispatchError) -> StopFleetError {
        StopFleetError::HttpDispatch(err)
    }
}
impl From<io::Error> for StopFleetError {
    fn from(err: io::Error) -> StopFleetError {
        StopFleetError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for StopFleetError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for StopFleetError {
    fn description(&self) -> &str {
        match *self {
            StopFleetError::ConcurrentModification(ref cause) => cause,
            StopFleetError::ResourceNotFound(ref cause) => cause,
            StopFleetError::Validation(ref cause) => cause,
            StopFleetError::Credentials(ref err) => err.description(),
            StopFleetError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            StopFleetError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by StopImageBuilder
#[derive(Debug, PartialEq)]
pub enum StopImageBuilderError {
    /// <p>An API error occurred. Wait a few minutes and try again.</p>
    ConcurrentModification(String),
    /// <p>The attempted operation is not permitted.</p>
    OperationNotPermitted(String),
    /// <p>The specified resource was not found.</p>
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

impl StopImageBuilderError {
    pub fn from_body(body: &str) -> StopImageBuilderError {
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
                        StopImageBuilderError::ConcurrentModification(String::from(error_message))
                    }
                    "OperationNotPermittedException" => {
                        StopImageBuilderError::OperationNotPermitted(String::from(error_message))
                    }
                    "ResourceNotFoundException" => {
                        StopImageBuilderError::ResourceNotFound(String::from(error_message))
                    }
                    "ValidationException" => {
                        StopImageBuilderError::Validation(error_message.to_string())
                    }
                    _ => StopImageBuilderError::Unknown(String::from(body)),
                }
            }
            Err(_) => StopImageBuilderError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for StopImageBuilderError {
    fn from(err: serde_json::error::Error) -> StopImageBuilderError {
        StopImageBuilderError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for StopImageBuilderError {
    fn from(err: CredentialsError) -> StopImageBuilderError {
        StopImageBuilderError::Credentials(err)
    }
}
impl From<HttpDispatchError> for StopImageBuilderError {
    fn from(err: HttpDispatchError) -> StopImageBuilderError {
        StopImageBuilderError::HttpDispatch(err)
    }
}
impl From<io::Error> for StopImageBuilderError {
    fn from(err: io::Error) -> StopImageBuilderError {
        StopImageBuilderError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for StopImageBuilderError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for StopImageBuilderError {
    fn description(&self) -> &str {
        match *self {
            StopImageBuilderError::ConcurrentModification(ref cause) => cause,
            StopImageBuilderError::OperationNotPermitted(ref cause) => cause,
            StopImageBuilderError::ResourceNotFound(ref cause) => cause,
            StopImageBuilderError::Validation(ref cause) => cause,
            StopImageBuilderError::Credentials(ref err) => err.description(),
            StopImageBuilderError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            StopImageBuilderError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by TagResource
#[derive(Debug, PartialEq)]
pub enum TagResourceError {
    /// <p>The resource cannot be created because your AWS account is suspended. For assistance, contact AWS Support. </p>
    InvalidAccountStatus(String),
    /// <p>The requested limit exceeds the permitted limit for an account.</p>
    LimitExceeded(String),
    /// <p>The specified resource was not found.</p>
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

impl TagResourceError {
    pub fn from_body(body: &str) -> TagResourceError {
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
                    "InvalidAccountStatusException" => {
                        TagResourceError::InvalidAccountStatus(String::from(error_message))
                    }
                    "LimitExceededException" => {
                        TagResourceError::LimitExceeded(String::from(error_message))
                    }
                    "ResourceNotFoundException" => {
                        TagResourceError::ResourceNotFound(String::from(error_message))
                    }
                    "ValidationException" => {
                        TagResourceError::Validation(error_message.to_string())
                    }
                    _ => TagResourceError::Unknown(String::from(body)),
                }
            }
            Err(_) => TagResourceError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for TagResourceError {
    fn from(err: serde_json::error::Error) -> TagResourceError {
        TagResourceError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for TagResourceError {
    fn from(err: CredentialsError) -> TagResourceError {
        TagResourceError::Credentials(err)
    }
}
impl From<HttpDispatchError> for TagResourceError {
    fn from(err: HttpDispatchError) -> TagResourceError {
        TagResourceError::HttpDispatch(err)
    }
}
impl From<io::Error> for TagResourceError {
    fn from(err: io::Error) -> TagResourceError {
        TagResourceError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for TagResourceError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for TagResourceError {
    fn description(&self) -> &str {
        match *self {
            TagResourceError::InvalidAccountStatus(ref cause) => cause,
            TagResourceError::LimitExceeded(ref cause) => cause,
            TagResourceError::ResourceNotFound(ref cause) => cause,
            TagResourceError::Validation(ref cause) => cause,
            TagResourceError::Credentials(ref err) => err.description(),
            TagResourceError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            TagResourceError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by UntagResource
#[derive(Debug, PartialEq)]
pub enum UntagResourceError {
    /// <p>The specified resource was not found.</p>
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

impl UntagResourceError {
    pub fn from_body(body: &str) -> UntagResourceError {
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
                        UntagResourceError::ResourceNotFound(String::from(error_message))
                    }
                    "ValidationException" => {
                        UntagResourceError::Validation(error_message.to_string())
                    }
                    _ => UntagResourceError::Unknown(String::from(body)),
                }
            }
            Err(_) => UntagResourceError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for UntagResourceError {
    fn from(err: serde_json::error::Error) -> UntagResourceError {
        UntagResourceError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for UntagResourceError {
    fn from(err: CredentialsError) -> UntagResourceError {
        UntagResourceError::Credentials(err)
    }
}
impl From<HttpDispatchError> for UntagResourceError {
    fn from(err: HttpDispatchError) -> UntagResourceError {
        UntagResourceError::HttpDispatch(err)
    }
}
impl From<io::Error> for UntagResourceError {
    fn from(err: io::Error) -> UntagResourceError {
        UntagResourceError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for UntagResourceError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for UntagResourceError {
    fn description(&self) -> &str {
        match *self {
            UntagResourceError::ResourceNotFound(ref cause) => cause,
            UntagResourceError::Validation(ref cause) => cause,
            UntagResourceError::Credentials(ref err) => err.description(),
            UntagResourceError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            UntagResourceError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by UpdateDirectoryConfig
#[derive(Debug, PartialEq)]
pub enum UpdateDirectoryConfigError {
    /// <p>An API error occurred. Wait a few minutes and try again.</p>
    ConcurrentModification(String),
    /// <p>The specified resource is in use.</p>
    ResourceInUse(String),
    /// <p>The specified resource was not found.</p>
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

impl UpdateDirectoryConfigError {
    pub fn from_body(body: &str) -> UpdateDirectoryConfigError {
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
                        UpdateDirectoryConfigError::ConcurrentModification(String::from(
                            error_message,
                        ))
                    }
                    "ResourceInUseException" => {
                        UpdateDirectoryConfigError::ResourceInUse(String::from(error_message))
                    }
                    "ResourceNotFoundException" => {
                        UpdateDirectoryConfigError::ResourceNotFound(String::from(error_message))
                    }
                    "ValidationException" => {
                        UpdateDirectoryConfigError::Validation(error_message.to_string())
                    }
                    _ => UpdateDirectoryConfigError::Unknown(String::from(body)),
                }
            }
            Err(_) => UpdateDirectoryConfigError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for UpdateDirectoryConfigError {
    fn from(err: serde_json::error::Error) -> UpdateDirectoryConfigError {
        UpdateDirectoryConfigError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for UpdateDirectoryConfigError {
    fn from(err: CredentialsError) -> UpdateDirectoryConfigError {
        UpdateDirectoryConfigError::Credentials(err)
    }
}
impl From<HttpDispatchError> for UpdateDirectoryConfigError {
    fn from(err: HttpDispatchError) -> UpdateDirectoryConfigError {
        UpdateDirectoryConfigError::HttpDispatch(err)
    }
}
impl From<io::Error> for UpdateDirectoryConfigError {
    fn from(err: io::Error) -> UpdateDirectoryConfigError {
        UpdateDirectoryConfigError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for UpdateDirectoryConfigError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for UpdateDirectoryConfigError {
    fn description(&self) -> &str {
        match *self {
            UpdateDirectoryConfigError::ConcurrentModification(ref cause) => cause,
            UpdateDirectoryConfigError::ResourceInUse(ref cause) => cause,
            UpdateDirectoryConfigError::ResourceNotFound(ref cause) => cause,
            UpdateDirectoryConfigError::Validation(ref cause) => cause,
            UpdateDirectoryConfigError::Credentials(ref err) => err.description(),
            UpdateDirectoryConfigError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            UpdateDirectoryConfigError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by UpdateFleet
#[derive(Debug, PartialEq)]
pub enum UpdateFleetError {
    /// <p>An API error occurred. Wait a few minutes and try again.</p>
    ConcurrentModification(String),
    /// <p>The image does not support storage connectors.</p>
    IncompatibleImage(String),
    /// <p>The resource cannot be created because your AWS account is suspended. For assistance, contact AWS Support. </p>
    InvalidAccountStatus(String),
    /// <p>Indicates an incorrect combination of parameters, or a missing parameter.</p>
    InvalidParameterCombination(String),
    /// <p>The specified role is invalid.</p>
    InvalidRole(String),
    /// <p>The requested limit exceeds the permitted limit for an account.</p>
    LimitExceeded(String),
    /// <p>The attempted operation is not permitted.</p>
    OperationNotPermitted(String),
    /// <p>The specified resource is in use.</p>
    ResourceInUse(String),
    /// <p>The specified resource exists and is not in use, but isn't available.</p>
    ResourceNotAvailable(String),
    /// <p>The specified resource was not found.</p>
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

impl UpdateFleetError {
    pub fn from_body(body: &str) -> UpdateFleetError {
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
                        UpdateFleetError::ConcurrentModification(String::from(error_message))
                    }
                    "IncompatibleImageException" => {
                        UpdateFleetError::IncompatibleImage(String::from(error_message))
                    }
                    "InvalidAccountStatusException" => {
                        UpdateFleetError::InvalidAccountStatus(String::from(error_message))
                    }
                    "InvalidParameterCombinationException" => {
                        UpdateFleetError::InvalidParameterCombination(String::from(error_message))
                    }
                    "InvalidRoleException" => {
                        UpdateFleetError::InvalidRole(String::from(error_message))
                    }
                    "LimitExceededException" => {
                        UpdateFleetError::LimitExceeded(String::from(error_message))
                    }
                    "OperationNotPermittedException" => {
                        UpdateFleetError::OperationNotPermitted(String::from(error_message))
                    }
                    "ResourceInUseException" => {
                        UpdateFleetError::ResourceInUse(String::from(error_message))
                    }
                    "ResourceNotAvailableException" => {
                        UpdateFleetError::ResourceNotAvailable(String::from(error_message))
                    }
                    "ResourceNotFoundException" => {
                        UpdateFleetError::ResourceNotFound(String::from(error_message))
                    }
                    "ValidationException" => {
                        UpdateFleetError::Validation(error_message.to_string())
                    }
                    _ => UpdateFleetError::Unknown(String::from(body)),
                }
            }
            Err(_) => UpdateFleetError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for UpdateFleetError {
    fn from(err: serde_json::error::Error) -> UpdateFleetError {
        UpdateFleetError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for UpdateFleetError {
    fn from(err: CredentialsError) -> UpdateFleetError {
        UpdateFleetError::Credentials(err)
    }
}
impl From<HttpDispatchError> for UpdateFleetError {
    fn from(err: HttpDispatchError) -> UpdateFleetError {
        UpdateFleetError::HttpDispatch(err)
    }
}
impl From<io::Error> for UpdateFleetError {
    fn from(err: io::Error) -> UpdateFleetError {
        UpdateFleetError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for UpdateFleetError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for UpdateFleetError {
    fn description(&self) -> &str {
        match *self {
            UpdateFleetError::ConcurrentModification(ref cause) => cause,
            UpdateFleetError::IncompatibleImage(ref cause) => cause,
            UpdateFleetError::InvalidAccountStatus(ref cause) => cause,
            UpdateFleetError::InvalidParameterCombination(ref cause) => cause,
            UpdateFleetError::InvalidRole(ref cause) => cause,
            UpdateFleetError::LimitExceeded(ref cause) => cause,
            UpdateFleetError::OperationNotPermitted(ref cause) => cause,
            UpdateFleetError::ResourceInUse(ref cause) => cause,
            UpdateFleetError::ResourceNotAvailable(ref cause) => cause,
            UpdateFleetError::ResourceNotFound(ref cause) => cause,
            UpdateFleetError::Validation(ref cause) => cause,
            UpdateFleetError::Credentials(ref err) => err.description(),
            UpdateFleetError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            UpdateFleetError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by UpdateStack
#[derive(Debug, PartialEq)]
pub enum UpdateStackError {
    /// <p>The image does not support storage connectors.</p>
    IncompatibleImage(String),
    /// <p>The resource cannot be created because your AWS account is suspended. For assistance, contact AWS Support. </p>
    InvalidAccountStatus(String),
    /// <p>Indicates an incorrect combination of parameters, or a missing parameter.</p>
    InvalidParameterCombination(String),
    /// <p>The specified role is invalid.</p>
    InvalidRole(String),
    /// <p>The requested limit exceeds the permitted limit for an account.</p>
    LimitExceeded(String),
    /// <p>The specified resource is in use.</p>
    ResourceInUse(String),
    /// <p>The specified resource was not found.</p>
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

impl UpdateStackError {
    pub fn from_body(body: &str) -> UpdateStackError {
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
                    "IncompatibleImageException" => {
                        UpdateStackError::IncompatibleImage(String::from(error_message))
                    }
                    "InvalidAccountStatusException" => {
                        UpdateStackError::InvalidAccountStatus(String::from(error_message))
                    }
                    "InvalidParameterCombinationException" => {
                        UpdateStackError::InvalidParameterCombination(String::from(error_message))
                    }
                    "InvalidRoleException" => {
                        UpdateStackError::InvalidRole(String::from(error_message))
                    }
                    "LimitExceededException" => {
                        UpdateStackError::LimitExceeded(String::from(error_message))
                    }
                    "ResourceInUseException" => {
                        UpdateStackError::ResourceInUse(String::from(error_message))
                    }
                    "ResourceNotFoundException" => {
                        UpdateStackError::ResourceNotFound(String::from(error_message))
                    }
                    "ValidationException" => {
                        UpdateStackError::Validation(error_message.to_string())
                    }
                    _ => UpdateStackError::Unknown(String::from(body)),
                }
            }
            Err(_) => UpdateStackError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for UpdateStackError {
    fn from(err: serde_json::error::Error) -> UpdateStackError {
        UpdateStackError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for UpdateStackError {
    fn from(err: CredentialsError) -> UpdateStackError {
        UpdateStackError::Credentials(err)
    }
}
impl From<HttpDispatchError> for UpdateStackError {
    fn from(err: HttpDispatchError) -> UpdateStackError {
        UpdateStackError::HttpDispatch(err)
    }
}
impl From<io::Error> for UpdateStackError {
    fn from(err: io::Error) -> UpdateStackError {
        UpdateStackError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for UpdateStackError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for UpdateStackError {
    fn description(&self) -> &str {
        match *self {
            UpdateStackError::IncompatibleImage(ref cause) => cause,
            UpdateStackError::InvalidAccountStatus(ref cause) => cause,
            UpdateStackError::InvalidParameterCombination(ref cause) => cause,
            UpdateStackError::InvalidRole(ref cause) => cause,
            UpdateStackError::LimitExceeded(ref cause) => cause,
            UpdateStackError::ResourceInUse(ref cause) => cause,
            UpdateStackError::ResourceNotFound(ref cause) => cause,
            UpdateStackError::Validation(ref cause) => cause,
            UpdateStackError::Credentials(ref err) => err.description(),
            UpdateStackError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            UpdateStackError::Unknown(ref cause) => cause,
        }
    }
}
/// Trait representing the capabilities of the Amazon AppStream API. Amazon AppStream clients implement this trait.
pub trait AppStream {
    /// <p>Associates the specified fleet with the specified stack.</p>
    fn associate_fleet(
        &self,
        input: AssociateFleetRequest,
    ) -> RusotoFuture<AssociateFleetResult, AssociateFleetError>;

    /// <p>Copies the image within the same region or to a new region within the same AWS account. Note that any tags you added to the image will not be copied.</p>
    fn copy_image(
        &self,
        input: CopyImageRequest,
    ) -> RusotoFuture<CopyImageResponse, CopyImageError>;

    /// <p>Creates a directory configuration.</p>
    fn create_directory_config(
        &self,
        input: CreateDirectoryConfigRequest,
    ) -> RusotoFuture<CreateDirectoryConfigResult, CreateDirectoryConfigError>;

    /// <p>Creates a fleet.</p>
    fn create_fleet(
        &self,
        input: CreateFleetRequest,
    ) -> RusotoFuture<CreateFleetResult, CreateFleetError>;

    /// <p>Creates an image builder.</p> <p>The initial state of the builder is <code>PENDING</code>. When it is ready, the state is <code>RUNNING</code>.</p>
    fn create_image_builder(
        &self,
        input: CreateImageBuilderRequest,
    ) -> RusotoFuture<CreateImageBuilderResult, CreateImageBuilderError>;

    /// <p>Creates a URL to start an image builder streaming session.</p>
    fn create_image_builder_streaming_url(
        &self,
        input: CreateImageBuilderStreamingURLRequest,
    ) -> RusotoFuture<CreateImageBuilderStreamingURLResult, CreateImageBuilderStreamingURLError>;

    /// <p>Creates a stack.</p>
    fn create_stack(
        &self,
        input: CreateStackRequest,
    ) -> RusotoFuture<CreateStackResult, CreateStackError>;

    /// <p>Creates a URL to start a streaming session for the specified user.</p>
    fn create_streaming_url(
        &self,
        input: CreateStreamingURLRequest,
    ) -> RusotoFuture<CreateStreamingURLResult, CreateStreamingURLError>;

    /// <p>Deletes the specified directory configuration.</p>
    fn delete_directory_config(
        &self,
        input: DeleteDirectoryConfigRequest,
    ) -> RusotoFuture<DeleteDirectoryConfigResult, DeleteDirectoryConfigError>;

    /// <p>Deletes the specified fleet.</p>
    fn delete_fleet(
        &self,
        input: DeleteFleetRequest,
    ) -> RusotoFuture<DeleteFleetResult, DeleteFleetError>;

    /// <p>Deletes the specified image. You cannot delete an image that is currently in use. After you delete an image, you cannot provision new capacity using the image.</p>
    fn delete_image(
        &self,
        input: DeleteImageRequest,
    ) -> RusotoFuture<DeleteImageResult, DeleteImageError>;

    /// <p>Deletes the specified image builder and releases the capacity.</p>
    fn delete_image_builder(
        &self,
        input: DeleteImageBuilderRequest,
    ) -> RusotoFuture<DeleteImageBuilderResult, DeleteImageBuilderError>;

    /// <p>Deletes the specified stack. After this operation completes, the environment can no longer be activated and any reservations made for the stack are released.</p>
    fn delete_stack(
        &self,
        input: DeleteStackRequest,
    ) -> RusotoFuture<DeleteStackResult, DeleteStackError>;

    /// <p>Describes the specified directory configurations. Note that although the response syntax in this topic includes the account password, this password is not returned in the actual response. </p>
    fn describe_directory_configs(
        &self,
        input: DescribeDirectoryConfigsRequest,
    ) -> RusotoFuture<DescribeDirectoryConfigsResult, DescribeDirectoryConfigsError>;

    /// <p>Describes the specified fleets or all fleets in the account.</p>
    fn describe_fleets(
        &self,
        input: DescribeFleetsRequest,
    ) -> RusotoFuture<DescribeFleetsResult, DescribeFleetsError>;

    /// <p>Describes the specified image builders or all image builders in the account.</p>
    fn describe_image_builders(
        &self,
        input: DescribeImageBuildersRequest,
    ) -> RusotoFuture<DescribeImageBuildersResult, DescribeImageBuildersError>;

    /// <p>Describes the specified images or all images in the account.</p>
    fn describe_images(
        &self,
        input: DescribeImagesRequest,
    ) -> RusotoFuture<DescribeImagesResult, DescribeImagesError>;

    /// <p>Describes the streaming sessions for the specified stack and fleet. If a user ID is provided, only the streaming sessions for only that user are returned. If an authentication type is not provided, the default is to authenticate users using a streaming URL.</p>
    fn describe_sessions(
        &self,
        input: DescribeSessionsRequest,
    ) -> RusotoFuture<DescribeSessionsResult, DescribeSessionsError>;

    /// <p>Describes the specified stacks or all stacks in the account.</p>
    fn describe_stacks(
        &self,
        input: DescribeStacksRequest,
    ) -> RusotoFuture<DescribeStacksResult, DescribeStacksError>;

    /// <p>Disassociates the specified fleet from the specified stack.</p>
    fn disassociate_fleet(
        &self,
        input: DisassociateFleetRequest,
    ) -> RusotoFuture<DisassociateFleetResult, DisassociateFleetError>;

    /// <p>Stops the specified streaming session.</p>
    fn expire_session(
        &self,
        input: ExpireSessionRequest,
    ) -> RusotoFuture<ExpireSessionResult, ExpireSessionError>;

    /// <p>Lists the fleets associated with the specified stack.</p>
    fn list_associated_fleets(
        &self,
        input: ListAssociatedFleetsRequest,
    ) -> RusotoFuture<ListAssociatedFleetsResult, ListAssociatedFleetsError>;

    /// <p>Lists the stacks associated with the specified fleet.</p>
    fn list_associated_stacks(
        &self,
        input: ListAssociatedStacksRequest,
    ) -> RusotoFuture<ListAssociatedStacksResult, ListAssociatedStacksError>;

    /// <p>Lists the tags for the specified AppStream 2.0 resource. You can tag AppStream 2.0 image builders, images, fleets, and stacks.</p> <p>For more information about tags, see <a href="http://docs.aws.amazon.com/appstream2/latest/developerguide/tagging-basic.html">Tagging Your Resources</a> in the <i>Amazon AppStream 2.0 Developer Guide</i>.</p>
    fn list_tags_for_resource(
        &self,
        input: ListTagsForResourceRequest,
    ) -> RusotoFuture<ListTagsForResourceResponse, ListTagsForResourceError>;

    /// <p>Starts the specified fleet.</p>
    fn start_fleet(
        &self,
        input: StartFleetRequest,
    ) -> RusotoFuture<StartFleetResult, StartFleetError>;

    /// <p>Starts the specified image builder.</p>
    fn start_image_builder(
        &self,
        input: StartImageBuilderRequest,
    ) -> RusotoFuture<StartImageBuilderResult, StartImageBuilderError>;

    /// <p>Stops the specified fleet.</p>
    fn stop_fleet(&self, input: StopFleetRequest) -> RusotoFuture<StopFleetResult, StopFleetError>;

    /// <p>Stops the specified image builder.</p>
    fn stop_image_builder(
        &self,
        input: StopImageBuilderRequest,
    ) -> RusotoFuture<StopImageBuilderResult, StopImageBuilderError>;

    /// <p>Adds or overwrites one or more tags for the specified AppStream 2.0 resource. You can tag AppStream 2.0 image builders, images, fleets, and stacks.</p> <p>Each tag consists of a key and an optional value. If a resource already has a tag with the same key, this operation updates its value.</p> <p>To list the current tags for your resources, use <a>ListTagsForResource</a>. To disassociate tags from your resources, use <a>UntagResource</a>.</p> <p>For more information about tags, see <a href="http://docs.aws.amazon.com/appstream2/latest/developerguide/tagging-basic.html">Tagging Your Resources</a> in the <i>Amazon AppStream 2.0 Developer Guide</i>.</p>
    fn tag_resource(
        &self,
        input: TagResourceRequest,
    ) -> RusotoFuture<TagResourceResponse, TagResourceError>;

    /// <p>Disassociates the specified tags from the specified AppStream 2.0 resource.</p> <p>To list the current tags for your resources, use <a>ListTagsForResource</a>.</p> <p>For more information about tags, see <a href="http://docs.aws.amazon.com/appstream2/latest/developerguide/tagging-basic.html">Tagging Your Resources</a> in the <i>Amazon AppStream 2.0 Developer Guide</i>.</p>
    fn untag_resource(
        &self,
        input: UntagResourceRequest,
    ) -> RusotoFuture<UntagResourceResponse, UntagResourceError>;

    /// <p>Updates the specified directory configuration.</p>
    fn update_directory_config(
        &self,
        input: UpdateDirectoryConfigRequest,
    ) -> RusotoFuture<UpdateDirectoryConfigResult, UpdateDirectoryConfigError>;

    /// <p>Updates the specified fleet.</p> <p>If the fleet is in the <code>STOPPED</code> state, you can update any attribute except the fleet name. If the fleet is in the <code>RUNNING</code> state, you can update the <code>DisplayName</code> and <code>ComputeCapacity</code> attributes. If the fleet is in the <code>STARTING</code> or <code>STOPPING</code> state, you can't update it.</p>
    fn update_fleet(
        &self,
        input: UpdateFleetRequest,
    ) -> RusotoFuture<UpdateFleetResult, UpdateFleetError>;

    /// <p>Updates the specified stack.</p>
    fn update_stack(
        &self,
        input: UpdateStackRequest,
    ) -> RusotoFuture<UpdateStackResult, UpdateStackError>;
}
/// A client for the Amazon AppStream API.
pub struct AppStreamClient<P = CredentialsProvider, D = RequestDispatcher>
where
    P: ProvideAwsCredentials,
    D: DispatchSignedRequest,
{
    inner: ClientInner<P, D>,
    region: region::Region,
}

impl AppStreamClient {
    /// Creates a simple client backed by an implicit event loop.
    ///
    /// The client will use the default credentials provider and tls client.
    ///
    /// See the `rusoto_core::reactor` module for more details.
    pub fn simple(region: region::Region) -> AppStreamClient {
        AppStreamClient::new(
            RequestDispatcher::default(),
            CredentialsProvider::default(),
            region,
        )
    }
}

impl<P, D> AppStreamClient<P, D>
where
    P: ProvideAwsCredentials,
    D: DispatchSignedRequest,
{
    pub fn new(request_dispatcher: D, credentials_provider: P, region: region::Region) -> Self {
        AppStreamClient {
            inner: ClientInner::new(credentials_provider, request_dispatcher),
            region: region,
        }
    }
}

impl<P, D> AppStream for AppStreamClient<P, D>
where
    P: ProvideAwsCredentials + 'static,
    D: DispatchSignedRequest + 'static,
{
    /// <p>Associates the specified fleet with the specified stack.</p>
    fn associate_fleet(
        &self,
        input: AssociateFleetRequest,
    ) -> RusotoFuture<AssociateFleetResult, AssociateFleetError> {
        let mut request = SignedRequest::new("POST", "appstream", &self.region, "/");
        request.set_endpoint_prefix("appstream2".to_string());
        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "PhotonAdminProxyService.AssociateFleet");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<AssociateFleetResult>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(AssociateFleetError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Copies the image within the same region or to a new region within the same AWS account. Note that any tags you added to the image will not be copied.</p>
    fn copy_image(
        &self,
        input: CopyImageRequest,
    ) -> RusotoFuture<CopyImageResponse, CopyImageError> {
        let mut request = SignedRequest::new("POST", "appstream", &self.region, "/");
        request.set_endpoint_prefix("appstream2".to_string());
        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "PhotonAdminProxyService.CopyImage");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<CopyImageResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(CopyImageError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Creates a directory configuration.</p>
    fn create_directory_config(
        &self,
        input: CreateDirectoryConfigRequest,
    ) -> RusotoFuture<CreateDirectoryConfigResult, CreateDirectoryConfigError> {
        let mut request = SignedRequest::new("POST", "appstream", &self.region, "/");
        request.set_endpoint_prefix("appstream2".to_string());
        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "PhotonAdminProxyService.CreateDirectoryConfig",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<CreateDirectoryConfigResult>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(CreateDirectoryConfigError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Creates a fleet.</p>
    fn create_fleet(
        &self,
        input: CreateFleetRequest,
    ) -> RusotoFuture<CreateFleetResult, CreateFleetError> {
        let mut request = SignedRequest::new("POST", "appstream", &self.region, "/");
        request.set_endpoint_prefix("appstream2".to_string());
        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "PhotonAdminProxyService.CreateFleet");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<CreateFleetResult>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(CreateFleetError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Creates an image builder.</p> <p>The initial state of the builder is <code>PENDING</code>. When it is ready, the state is <code>RUNNING</code>.</p>
    fn create_image_builder(
        &self,
        input: CreateImageBuilderRequest,
    ) -> RusotoFuture<CreateImageBuilderResult, CreateImageBuilderError> {
        let mut request = SignedRequest::new("POST", "appstream", &self.region, "/");
        request.set_endpoint_prefix("appstream2".to_string());
        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "PhotonAdminProxyService.CreateImageBuilder");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<CreateImageBuilderResult>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(CreateImageBuilderError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Creates a URL to start an image builder streaming session.</p>
    fn create_image_builder_streaming_url(
        &self,
        input: CreateImageBuilderStreamingURLRequest,
    ) -> RusotoFuture<CreateImageBuilderStreamingURLResult, CreateImageBuilderStreamingURLError>
    {
        let mut request = SignedRequest::new("POST", "appstream", &self.region, "/");
        request.set_endpoint_prefix("appstream2".to_string());
        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "PhotonAdminProxyService.CreateImageBuilderStreamingURL",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<CreateImageBuilderStreamingURLResult>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(CreateImageBuilderStreamingURLError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Creates a stack.</p>
    fn create_stack(
        &self,
        input: CreateStackRequest,
    ) -> RusotoFuture<CreateStackResult, CreateStackError> {
        let mut request = SignedRequest::new("POST", "appstream", &self.region, "/");
        request.set_endpoint_prefix("appstream2".to_string());
        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "PhotonAdminProxyService.CreateStack");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<CreateStackResult>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(CreateStackError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Creates a URL to start a streaming session for the specified user.</p>
    fn create_streaming_url(
        &self,
        input: CreateStreamingURLRequest,
    ) -> RusotoFuture<CreateStreamingURLResult, CreateStreamingURLError> {
        let mut request = SignedRequest::new("POST", "appstream", &self.region, "/");
        request.set_endpoint_prefix("appstream2".to_string());
        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "PhotonAdminProxyService.CreateStreamingURL");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<CreateStreamingURLResult>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(CreateStreamingURLError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Deletes the specified directory configuration.</p>
    fn delete_directory_config(
        &self,
        input: DeleteDirectoryConfigRequest,
    ) -> RusotoFuture<DeleteDirectoryConfigResult, DeleteDirectoryConfigError> {
        let mut request = SignedRequest::new("POST", "appstream", &self.region, "/");
        request.set_endpoint_prefix("appstream2".to_string());
        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "PhotonAdminProxyService.DeleteDirectoryConfig",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<DeleteDirectoryConfigResult>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(DeleteDirectoryConfigError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Deletes the specified fleet.</p>
    fn delete_fleet(
        &self,
        input: DeleteFleetRequest,
    ) -> RusotoFuture<DeleteFleetResult, DeleteFleetError> {
        let mut request = SignedRequest::new("POST", "appstream", &self.region, "/");
        request.set_endpoint_prefix("appstream2".to_string());
        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "PhotonAdminProxyService.DeleteFleet");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<DeleteFleetResult>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(DeleteFleetError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Deletes the specified image. You cannot delete an image that is currently in use. After you delete an image, you cannot provision new capacity using the image.</p>
    fn delete_image(
        &self,
        input: DeleteImageRequest,
    ) -> RusotoFuture<DeleteImageResult, DeleteImageError> {
        let mut request = SignedRequest::new("POST", "appstream", &self.region, "/");
        request.set_endpoint_prefix("appstream2".to_string());
        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "PhotonAdminProxyService.DeleteImage");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<DeleteImageResult>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(DeleteImageError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Deletes the specified image builder and releases the capacity.</p>
    fn delete_image_builder(
        &self,
        input: DeleteImageBuilderRequest,
    ) -> RusotoFuture<DeleteImageBuilderResult, DeleteImageBuilderError> {
        let mut request = SignedRequest::new("POST", "appstream", &self.region, "/");
        request.set_endpoint_prefix("appstream2".to_string());
        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "PhotonAdminProxyService.DeleteImageBuilder");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<DeleteImageBuilderResult>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(DeleteImageBuilderError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Deletes the specified stack. After this operation completes, the environment can no longer be activated and any reservations made for the stack are released.</p>
    fn delete_stack(
        &self,
        input: DeleteStackRequest,
    ) -> RusotoFuture<DeleteStackResult, DeleteStackError> {
        let mut request = SignedRequest::new("POST", "appstream", &self.region, "/");
        request.set_endpoint_prefix("appstream2".to_string());
        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "PhotonAdminProxyService.DeleteStack");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<DeleteStackResult>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(DeleteStackError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Describes the specified directory configurations. Note that although the response syntax in this topic includes the account password, this password is not returned in the actual response. </p>
    fn describe_directory_configs(
        &self,
        input: DescribeDirectoryConfigsRequest,
    ) -> RusotoFuture<DescribeDirectoryConfigsResult, DescribeDirectoryConfigsError> {
        let mut request = SignedRequest::new("POST", "appstream", &self.region, "/");
        request.set_endpoint_prefix("appstream2".to_string());
        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "PhotonAdminProxyService.DescribeDirectoryConfigs",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<DescribeDirectoryConfigsResult>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(DescribeDirectoryConfigsError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Describes the specified fleets or all fleets in the account.</p>
    fn describe_fleets(
        &self,
        input: DescribeFleetsRequest,
    ) -> RusotoFuture<DescribeFleetsResult, DescribeFleetsError> {
        let mut request = SignedRequest::new("POST", "appstream", &self.region, "/");
        request.set_endpoint_prefix("appstream2".to_string());
        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "PhotonAdminProxyService.DescribeFleets");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<DescribeFleetsResult>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(DescribeFleetsError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Describes the specified image builders or all image builders in the account.</p>
    fn describe_image_builders(
        &self,
        input: DescribeImageBuildersRequest,
    ) -> RusotoFuture<DescribeImageBuildersResult, DescribeImageBuildersError> {
        let mut request = SignedRequest::new("POST", "appstream", &self.region, "/");
        request.set_endpoint_prefix("appstream2".to_string());
        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "PhotonAdminProxyService.DescribeImageBuilders",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<DescribeImageBuildersResult>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(DescribeImageBuildersError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Describes the specified images or all images in the account.</p>
    fn describe_images(
        &self,
        input: DescribeImagesRequest,
    ) -> RusotoFuture<DescribeImagesResult, DescribeImagesError> {
        let mut request = SignedRequest::new("POST", "appstream", &self.region, "/");
        request.set_endpoint_prefix("appstream2".to_string());
        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "PhotonAdminProxyService.DescribeImages");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<DescribeImagesResult>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(DescribeImagesError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Describes the streaming sessions for the specified stack and fleet. If a user ID is provided, only the streaming sessions for only that user are returned. If an authentication type is not provided, the default is to authenticate users using a streaming URL.</p>
    fn describe_sessions(
        &self,
        input: DescribeSessionsRequest,
    ) -> RusotoFuture<DescribeSessionsResult, DescribeSessionsError> {
        let mut request = SignedRequest::new("POST", "appstream", &self.region, "/");
        request.set_endpoint_prefix("appstream2".to_string());
        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "PhotonAdminProxyService.DescribeSessions");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<DescribeSessionsResult>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(DescribeSessionsError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Describes the specified stacks or all stacks in the account.</p>
    fn describe_stacks(
        &self,
        input: DescribeStacksRequest,
    ) -> RusotoFuture<DescribeStacksResult, DescribeStacksError> {
        let mut request = SignedRequest::new("POST", "appstream", &self.region, "/");
        request.set_endpoint_prefix("appstream2".to_string());
        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "PhotonAdminProxyService.DescribeStacks");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<DescribeStacksResult>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(DescribeStacksError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Disassociates the specified fleet from the specified stack.</p>
    fn disassociate_fleet(
        &self,
        input: DisassociateFleetRequest,
    ) -> RusotoFuture<DisassociateFleetResult, DisassociateFleetError> {
        let mut request = SignedRequest::new("POST", "appstream", &self.region, "/");
        request.set_endpoint_prefix("appstream2".to_string());
        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "PhotonAdminProxyService.DisassociateFleet");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<DisassociateFleetResult>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(DisassociateFleetError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Stops the specified streaming session.</p>
    fn expire_session(
        &self,
        input: ExpireSessionRequest,
    ) -> RusotoFuture<ExpireSessionResult, ExpireSessionError> {
        let mut request = SignedRequest::new("POST", "appstream", &self.region, "/");
        request.set_endpoint_prefix("appstream2".to_string());
        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "PhotonAdminProxyService.ExpireSession");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<ExpireSessionResult>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(ExpireSessionError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Lists the fleets associated with the specified stack.</p>
    fn list_associated_fleets(
        &self,
        input: ListAssociatedFleetsRequest,
    ) -> RusotoFuture<ListAssociatedFleetsResult, ListAssociatedFleetsError> {
        let mut request = SignedRequest::new("POST", "appstream", &self.region, "/");
        request.set_endpoint_prefix("appstream2".to_string());
        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "PhotonAdminProxyService.ListAssociatedFleets",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<ListAssociatedFleetsResult>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(ListAssociatedFleetsError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Lists the stacks associated with the specified fleet.</p>
    fn list_associated_stacks(
        &self,
        input: ListAssociatedStacksRequest,
    ) -> RusotoFuture<ListAssociatedStacksResult, ListAssociatedStacksError> {
        let mut request = SignedRequest::new("POST", "appstream", &self.region, "/");
        request.set_endpoint_prefix("appstream2".to_string());
        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "PhotonAdminProxyService.ListAssociatedStacks",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<ListAssociatedStacksResult>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(ListAssociatedStacksError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Lists the tags for the specified AppStream 2.0 resource. You can tag AppStream 2.0 image builders, images, fleets, and stacks.</p> <p>For more information about tags, see <a href="http://docs.aws.amazon.com/appstream2/latest/developerguide/tagging-basic.html">Tagging Your Resources</a> in the <i>Amazon AppStream 2.0 Developer Guide</i>.</p>
    fn list_tags_for_resource(
        &self,
        input: ListTagsForResourceRequest,
    ) -> RusotoFuture<ListTagsForResourceResponse, ListTagsForResourceError> {
        let mut request = SignedRequest::new("POST", "appstream", &self.region, "/");
        request.set_endpoint_prefix("appstream2".to_string());
        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "PhotonAdminProxyService.ListTagsForResource",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<ListTagsForResourceResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(ListTagsForResourceError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Starts the specified fleet.</p>
    fn start_fleet(
        &self,
        input: StartFleetRequest,
    ) -> RusotoFuture<StartFleetResult, StartFleetError> {
        let mut request = SignedRequest::new("POST", "appstream", &self.region, "/");
        request.set_endpoint_prefix("appstream2".to_string());
        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "PhotonAdminProxyService.StartFleet");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<StartFleetResult>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(StartFleetError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Starts the specified image builder.</p>
    fn start_image_builder(
        &self,
        input: StartImageBuilderRequest,
    ) -> RusotoFuture<StartImageBuilderResult, StartImageBuilderError> {
        let mut request = SignedRequest::new("POST", "appstream", &self.region, "/");
        request.set_endpoint_prefix("appstream2".to_string());
        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "PhotonAdminProxyService.StartImageBuilder");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<StartImageBuilderResult>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(StartImageBuilderError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Stops the specified fleet.</p>
    fn stop_fleet(&self, input: StopFleetRequest) -> RusotoFuture<StopFleetResult, StopFleetError> {
        let mut request = SignedRequest::new("POST", "appstream", &self.region, "/");
        request.set_endpoint_prefix("appstream2".to_string());
        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "PhotonAdminProxyService.StopFleet");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<StopFleetResult>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(StopFleetError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Stops the specified image builder.</p>
    fn stop_image_builder(
        &self,
        input: StopImageBuilderRequest,
    ) -> RusotoFuture<StopImageBuilderResult, StopImageBuilderError> {
        let mut request = SignedRequest::new("POST", "appstream", &self.region, "/");
        request.set_endpoint_prefix("appstream2".to_string());
        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "PhotonAdminProxyService.StopImageBuilder");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<StopImageBuilderResult>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(StopImageBuilderError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Adds or overwrites one or more tags for the specified AppStream 2.0 resource. You can tag AppStream 2.0 image builders, images, fleets, and stacks.</p> <p>Each tag consists of a key and an optional value. If a resource already has a tag with the same key, this operation updates its value.</p> <p>To list the current tags for your resources, use <a>ListTagsForResource</a>. To disassociate tags from your resources, use <a>UntagResource</a>.</p> <p>For more information about tags, see <a href="http://docs.aws.amazon.com/appstream2/latest/developerguide/tagging-basic.html">Tagging Your Resources</a> in the <i>Amazon AppStream 2.0 Developer Guide</i>.</p>
    fn tag_resource(
        &self,
        input: TagResourceRequest,
    ) -> RusotoFuture<TagResourceResponse, TagResourceError> {
        let mut request = SignedRequest::new("POST", "appstream", &self.region, "/");
        request.set_endpoint_prefix("appstream2".to_string());
        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "PhotonAdminProxyService.TagResource");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<TagResourceResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(TagResourceError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Disassociates the specified tags from the specified AppStream 2.0 resource.</p> <p>To list the current tags for your resources, use <a>ListTagsForResource</a>.</p> <p>For more information about tags, see <a href="http://docs.aws.amazon.com/appstream2/latest/developerguide/tagging-basic.html">Tagging Your Resources</a> in the <i>Amazon AppStream 2.0 Developer Guide</i>.</p>
    fn untag_resource(
        &self,
        input: UntagResourceRequest,
    ) -> RusotoFuture<UntagResourceResponse, UntagResourceError> {
        let mut request = SignedRequest::new("POST", "appstream", &self.region, "/");
        request.set_endpoint_prefix("appstream2".to_string());
        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "PhotonAdminProxyService.UntagResource");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<UntagResourceResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(UntagResourceError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Updates the specified directory configuration.</p>
    fn update_directory_config(
        &self,
        input: UpdateDirectoryConfigRequest,
    ) -> RusotoFuture<UpdateDirectoryConfigResult, UpdateDirectoryConfigError> {
        let mut request = SignedRequest::new("POST", "appstream", &self.region, "/");
        request.set_endpoint_prefix("appstream2".to_string());
        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "PhotonAdminProxyService.UpdateDirectoryConfig",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<UpdateDirectoryConfigResult>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(UpdateDirectoryConfigError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Updates the specified fleet.</p> <p>If the fleet is in the <code>STOPPED</code> state, you can update any attribute except the fleet name. If the fleet is in the <code>RUNNING</code> state, you can update the <code>DisplayName</code> and <code>ComputeCapacity</code> attributes. If the fleet is in the <code>STARTING</code> or <code>STOPPING</code> state, you can't update it.</p>
    fn update_fleet(
        &self,
        input: UpdateFleetRequest,
    ) -> RusotoFuture<UpdateFleetResult, UpdateFleetError> {
        let mut request = SignedRequest::new("POST", "appstream", &self.region, "/");
        request.set_endpoint_prefix("appstream2".to_string());
        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "PhotonAdminProxyService.UpdateFleet");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<UpdateFleetResult>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(UpdateFleetError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Updates the specified stack.</p>
    fn update_stack(
        &self,
        input: UpdateStackRequest,
    ) -> RusotoFuture<UpdateStackResult, UpdateStackError> {
        let mut request = SignedRequest::new("POST", "appstream", &self.region, "/");
        request.set_endpoint_prefix("appstream2".to_string());
        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "PhotonAdminProxyService.UpdateStack");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<UpdateStackResult>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(UpdateStackError::from_body(
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
