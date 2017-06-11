#[allow(warnings)]
use hyper::Client;
use hyper::status::StatusCode;
use rusoto_core::request::DispatchSignedRequest;
use rusoto_core::region;

use std::fmt;
use std::error::Error;
use rusoto_core::request::HttpDispatchError;
use rusoto_core::credential::{CredentialsError, ProvideAwsCredentials};

use serde_json;
use rusoto_core::signature::SignedRequest;
use serde_json::Value as SerdeJsonValue;
use serde_json::from_str;
#[doc="<p>An entry for a single application in the application catalog.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct Application {
    #[doc="<p>The name of the application shown to the end users.</p>"]
    #[serde(rename="DisplayName")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub display_name: Option<String>,
    #[doc="<p>An application can be disabled after image creation if there is a problem.</p>"]
    #[serde(rename="Enabled")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub enabled: Option<Boolean>,
    #[doc="<p>The URL for the application icon. This URL may be time-limited.</p>"]
    #[serde(rename="IconURL")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub icon_url: Option<String>,
    #[doc="<p>A list of arguments that are passed to the application at launch.</p>"]
    #[serde(rename="LaunchParameters")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub launch_parameters: Option<String>,
    #[doc="<p>The path to the application executable in the instance.</p>"]
    #[serde(rename="LaunchPath")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub launch_path: Option<String>,
    #[doc="<p>Additional attributes that describe the application.</p>"]
    #[serde(rename="Metadata")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub metadata: Option<Metadata>,
    #[doc="<p>The unique identifier for the application.</p>"]
    #[serde(rename="Name")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub name: Option<String>,
}

pub type Applications = Vec<Application>;
pub type Arn = String;
#[derive(Default,Debug,Clone,Serialize)]
pub struct AssociateFleetRequest {
    #[doc="<p>The name of the fleet to associate.</p>"]
    #[serde(rename="FleetName")]
    pub fleet_name: String,
    #[doc="<p>The name of the stack to which the fleet is associated.</p>"]
    #[serde(rename="StackName")]
    pub stack_name: String,
}

#[derive(Default,Debug,Clone,Deserialize)]
pub struct AssociateFleetResult;

pub type AuthenticationType = String;
pub type Boolean = bool;
pub type BooleanObject = bool;
#[doc="<p>The capacity configuration for the fleet.</p>"]
#[derive(Default,Debug,Clone,Serialize)]
pub struct ComputeCapacity {
    #[doc="<p>The desired number of streaming instances.</p>"]
    #[serde(rename="DesiredInstances")]
    pub desired_instances: Integer,
}

#[doc="<p>The capacity information for the fleet.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct ComputeCapacityStatus {
    #[doc="<p>The number of currently available instances that can be used to stream sessions.</p>"]
    #[serde(rename="Available")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub available: Option<Integer>,
    #[doc="<p>The desired number of streaming instances.</p>"]
    #[serde(rename="Desired")]
    pub desired: Integer,
    #[doc="<p>The number of instances that are being used for streaming.</p>"]
    #[serde(rename="InUse")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub in_use: Option<Integer>,
    #[doc="<p>The total number of simultaneous streaming instances that are running.</p>"]
    #[serde(rename="Running")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub running: Option<Integer>,
}

#[doc="<p>Contains the parameters for the new fleet to create.</p>"]
#[derive(Default,Debug,Clone,Serialize)]
pub struct CreateFleetRequest {
    #[doc="<p>The parameters for the capacity allocated to the fleet.</p>"]
    #[serde(rename="ComputeCapacity")]
    pub compute_capacity: ComputeCapacity,
    #[doc="<p>The description of the fleet.</p>"]
    #[serde(rename="Description")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub description: Option<Description>,
    #[doc="<p>The time after disconnection when a session is considered to have ended. If a user who got disconnected reconnects within this timeout interval, the user is connected back to their previous session. The input can be any numeric value in seconds between 60 and 57600. </p>"]
    #[serde(rename="DisconnectTimeoutInSeconds")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub disconnect_timeout_in_seconds: Option<Integer>,
    #[doc="<p>The display name of the fleet.</p>"]
    #[serde(rename="DisplayName")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub display_name: Option<DisplayName>,
    #[doc="<p>Enables or disables default Internet access for the fleet.</p>"]
    #[serde(rename="EnableDefaultInternetAccess")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub enable_default_internet_access: Option<BooleanObject>,
    #[doc="<p>Unique name of the image used by the fleet.</p>"]
    #[serde(rename="ImageName")]
    pub image_name: String,
    #[doc="<p>The instance type of compute resources for the fleet. Fleet instances are launched from this instance type.</p>"]
    #[serde(rename="InstanceType")]
    pub instance_type: String,
    #[doc="<p>The maximum time for which a streaming session can run. The input can be any numeric value in seconds between 600 and 57600.</p>"]
    #[serde(rename="MaxUserDurationInSeconds")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub max_user_duration_in_seconds: Option<Integer>,
    #[doc="<p>A unique identifier for the fleet.</p>"]
    #[serde(rename="Name")]
    pub name: Name,
    #[doc="<p>The VPC configuration for the fleet.</p>"]
    #[serde(rename="VpcConfig")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub vpc_config: Option<VpcConfig>,
}

#[derive(Default,Debug,Clone,Deserialize)]
pub struct CreateFleetResult {
    #[doc="<p>The details for the created fleet.</p>"]
    #[serde(rename="Fleet")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub fleet: Option<Fleet>,
}

#[derive(Default,Debug,Clone,Serialize)]
pub struct CreateStackRequest {
    #[doc="<p>The description displayed to end users on the AppStream 2.0 portal.</p>"]
    #[serde(rename="Description")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub description: Option<Description>,
    #[doc="<p>The name displayed to end users on the AppStream 2.0 portal.</p>"]
    #[serde(rename="DisplayName")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub display_name: Option<DisplayName>,
    #[doc="<p>The unique identifier for this stack.</p>"]
    #[serde(rename="Name")]
    pub name: String,
    #[doc="<p>The storage connectors to be enabled for the stack.</p>"]
    #[serde(rename="StorageConnectors")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub storage_connectors: Option<StorageConnectorList>,
}

#[derive(Default,Debug,Clone,Deserialize)]
pub struct CreateStackResult {
    #[doc="<p>The details for the created stack.</p>"]
    #[serde(rename="Stack")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub stack: Option<Stack>,
}

#[derive(Default,Debug,Clone,Serialize)]
pub struct CreateStreamingURLRequest {
    #[doc="<p>The ID of the application that must be launched after the session starts.</p>"]
    #[serde(rename="ApplicationId")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub application_id: Option<String>,
    #[doc="<p>The fleet for which the URL is generated.</p>"]
    #[serde(rename="FleetName")]
    pub fleet_name: String,
    #[doc="<p>The sessionContext of the streaming URL.</p>"]
    #[serde(rename="SessionContext")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub session_context: Option<String>,
    #[doc="<p>The stack for which the URL is generated.</p>"]
    #[serde(rename="StackName")]
    pub stack_name: String,
    #[doc="<p>A unique user ID for whom the URL is generated.</p>"]
    #[serde(rename="UserId")]
    pub user_id: UserId,
    #[doc="<p>The duration up to which the URL returned by this action is valid. The input can be any numeric value in seconds between 1 and 604800 seconds.</p>"]
    #[serde(rename="Validity")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub validity: Option<Long>,
}

#[derive(Default,Debug,Clone,Deserialize)]
pub struct CreateStreamingURLResult {
    #[doc="<p>Elapsed seconds after the Unix epoch, at which time this URL expires.</p>"]
    #[serde(rename="Expires")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub expires: Option<Timestamp>,
    #[doc="<p>The URL to start the AppStream 2.0 streaming session.</p>"]
    #[serde(rename="StreamingURL")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub streaming_url: Option<String>,
}

#[derive(Default,Debug,Clone,Serialize)]
pub struct DeleteFleetRequest {
    #[doc="<p>The name of the fleet to be deleted.</p>"]
    #[serde(rename="Name")]
    pub name: String,
}

#[derive(Default,Debug,Clone,Deserialize)]
pub struct DeleteFleetResult;

#[derive(Default,Debug,Clone,Serialize)]
pub struct DeleteStackRequest {
    #[doc="<p>The name of the stack to delete.</p>"]
    #[serde(rename="Name")]
    pub name: String,
}

#[derive(Default,Debug,Clone,Deserialize)]
pub struct DeleteStackResult;

#[derive(Default,Debug,Clone,Serialize)]
pub struct DescribeFleetsRequest {
    #[doc="<p>The fleet names to describe. Use null to describe all the fleets for the AWS account.</p>"]
    #[serde(rename="Names")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub names: Option<StringList>,
    #[doc="<p>The pagination token to use to retrieve the next page of results for this operation. If this value is null, it retrieves the first page.</p>"]
    #[serde(rename="NextToken")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default,Debug,Clone,Deserialize)]
pub struct DescribeFleetsResult {
    #[doc="<p>The list of fleet details.</p>"]
    #[serde(rename="Fleets")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub fleets: Option<FleetList>,
    #[doc="<p>The pagination token to use to retrieve the next page of results for this operation. If there are no more pages, this value is null.</p>"]
    #[serde(rename="NextToken")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default,Debug,Clone,Serialize)]
pub struct DescribeImagesRequest {
    #[doc="<p>A specific list of images to describe.</p>"]
    #[serde(rename="Names")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub names: Option<StringList>,
}

#[derive(Default,Debug,Clone,Deserialize)]
pub struct DescribeImagesResult {
    #[doc="<p>The list of images.</p>"]
    #[serde(rename="Images")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub images: Option<ImageList>,
}

#[derive(Default,Debug,Clone,Serialize)]
pub struct DescribeSessionsRequest {
    #[doc="<p>The authentication method of the user. It can be <code>API</code> for a user authenticated using a streaming URL, or <code>SAML</code> for a SAML federated user. If an authentication type is not provided, the operation defaults to users authenticated using a streaming URL.</p>"]
    #[serde(rename="AuthenticationType")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub authentication_type: Option<AuthenticationType>,
    #[doc="<p>The name of the fleet for which to list sessions.</p>"]
    #[serde(rename="FleetName")]
    pub fleet_name: String,
    #[doc="<p>The size of each page of results. The default value is 20 and the maximum supported value is 50.</p>"]
    #[serde(rename="Limit")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub limit: Option<Integer>,
    #[doc="<p>The pagination token to use to retrieve the next page of results for this operation. If this value is null, it retrieves the first page.</p>"]
    #[serde(rename="NextToken")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub next_token: Option<String>,
    #[doc="<p>The name of the stack for which to list sessions.</p>"]
    #[serde(rename="StackName")]
    pub stack_name: String,
    #[doc="<p>The user for whom to list sessions. Use null to describe all the sessions for the stack and fleet.</p>"]
    #[serde(rename="UserId")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub user_id: Option<UserId>,
}

#[derive(Default,Debug,Clone,Deserialize)]
pub struct DescribeSessionsResult {
    #[doc="<p>The pagination token to use to retrieve the next page of results for this operation. If there are no more pages, this value is null.</p>"]
    #[serde(rename="NextToken")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub next_token: Option<String>,
    #[doc="<p>The list of streaming sessions.</p>"]
    #[serde(rename="Sessions")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub sessions: Option<SessionList>,
}

#[derive(Default,Debug,Clone,Serialize)]
pub struct DescribeStacksRequest {
    #[doc="<p>The stack names to describe. Use null to describe all the stacks for the AWS account.</p>"]
    #[serde(rename="Names")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub names: Option<StringList>,
    #[doc="<p>The pagination token to use to retrieve the next page of results for this operation. If this value is null, it retrieves the first page.</p>"]
    #[serde(rename="NextToken")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default,Debug,Clone,Deserialize)]
pub struct DescribeStacksResult {
    #[doc="<p>The pagination token to use to retrieve the next page of results for this operation. If there are no more pages, this value is null.</p>"]
    #[serde(rename="NextToken")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub next_token: Option<String>,
    #[doc="<p>The list of stack details.</p>"]
    #[serde(rename="Stacks")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub stacks: Option<StackList>,
}

pub type Description = String;
#[derive(Default,Debug,Clone,Serialize)]
pub struct DisassociateFleetRequest {
    #[doc="<p>The name of the fleet to disassociate.</p>"]
    #[serde(rename="FleetName")]
    pub fleet_name: String,
    #[doc="<p>The name of the stack with which the fleet is associated.</p>"]
    #[serde(rename="StackName")]
    pub stack_name: String,
}

#[derive(Default,Debug,Clone,Deserialize)]
pub struct DisassociateFleetResult;

pub type DisplayName = String;
#[doc="<p>The error message in the exception.</p>"]
pub type ErrorMessage = String;
#[derive(Default,Debug,Clone,Serialize)]
pub struct ExpireSessionRequest {
    #[doc="<p>The unique identifier of the streaming session to be stopped.</p>"]
    #[serde(rename="SessionId")]
    pub session_id: String,
}

#[derive(Default,Debug,Clone,Deserialize)]
pub struct ExpireSessionResult;

#[doc="<p>Contains the parameters for a fleet.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct Fleet {
    #[doc="<p>The ARN for the fleet.</p>"]
    #[serde(rename="Arn")]
    pub arn: Arn,
    #[doc="<p>The capacity information for the fleet.</p>"]
    #[serde(rename="ComputeCapacityStatus")]
    pub compute_capacity_status: ComputeCapacityStatus,
    #[doc="<p>The time at which the fleet was created.</p>"]
    #[serde(rename="CreatedTime")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub created_time: Option<Timestamp>,
    #[doc="<p>The description displayed to end users on the AppStream 2.0 portal.</p>"]
    #[serde(rename="Description")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub description: Option<String>,
    #[doc="<p>The time after disconnection when a session is considered to have ended. If a user who got disconnected reconnects within this timeout interval, the user is connected back to their previous session. The input can be any numeric value in seconds between 60 and 57600.</p>"]
    #[serde(rename="DisconnectTimeoutInSeconds")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub disconnect_timeout_in_seconds: Option<Integer>,
    #[doc="<p>The name displayed to end users on the AppStream 2.0 portal.</p>"]
    #[serde(rename="DisplayName")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub display_name: Option<String>,
    #[doc="<p>Whether default Internet access is enabled for the fleet. </p>"]
    #[serde(rename="EnableDefaultInternetAccess")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub enable_default_internet_access: Option<BooleanObject>,
    #[doc="<p>The list of fleet errors is appended to this list.</p>"]
    #[serde(rename="FleetErrors")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub fleet_errors: Option<FleetErrors>,
    #[doc="<p>The image used by the fleet.</p>"]
    #[serde(rename="ImageName")]
    pub image_name: String,
    #[doc="<p>The instance type of compute resources for the fleet. The fleet instances are launched from this instance type. </p>"]
    #[serde(rename="InstanceType")]
    pub instance_type: String,
    #[doc="<p>The maximum time for which a streaming session can run. The value can be any numeric value in seconds between 600 and 57600.</p>"]
    #[serde(rename="MaxUserDurationInSeconds")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub max_user_duration_in_seconds: Option<Integer>,
    #[doc="<p>The name of the fleet.</p>"]
    #[serde(rename="Name")]
    pub name: String,
    #[doc="<p>The current state for the fleet.</p>"]
    #[serde(rename="State")]
    pub state: FleetState,
    #[doc="<p>The VPC configuration for the fleet.</p>"]
    #[serde(rename="VpcConfig")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub vpc_config: Option<VpcConfig>,
}

#[doc="<p>The details of the fleet error.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct FleetError {
    #[doc="<p>The error code for the fleet error.</p>"]
    #[serde(rename="ErrorCode")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub error_code: Option<FleetErrorCode>,
    #[doc="<p>The error message generated when the fleet has errors.</p>"]
    #[serde(rename="ErrorMessage")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub error_message: Option<String>,
}

pub type FleetErrorCode = String;
pub type FleetErrors = Vec<FleetError>;
#[doc="<p>A list of fleets.</p>"]
pub type FleetList = Vec<Fleet>;
pub type FleetState = String;
#[doc="<p>New streaming instances are booted from images. The image stores the application catalog and is connected to fleets.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct Image {
    #[doc="<p>The applications associated with an image.</p>"]
    #[serde(rename="Applications")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub applications: Option<Applications>,
    #[doc="<p>The ARN for the image.</p>"]
    #[serde(rename="Arn")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub arn: Option<Arn>,
    #[doc="<p>The source image ARN from which this image was created.</p>"]
    #[serde(rename="BaseImageArn")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub base_image_arn: Option<Arn>,
    #[doc="<p>The timestamp when the image was created.</p>"]
    #[serde(rename="CreatedTime")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub created_time: Option<Timestamp>,
    #[doc="<p>A meaningful description for the image.</p>"]
    #[serde(rename="Description")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub description: Option<String>,
    #[doc="<p>The display name for the image.</p>"]
    #[serde(rename="DisplayName")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub display_name: Option<String>,
    #[doc="<p>Whether an image builder can be launched from this image.</p>"]
    #[serde(rename="ImageBuilderSupported")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub image_builder_supported: Option<Boolean>,
    #[doc="<p>The unique identifier for the image.</p>"]
    #[serde(rename="Name")]
    pub name: String,
    #[doc="<p>The operating system platform of the image.</p>"]
    #[serde(rename="Platform")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub platform: Option<PlatformType>,
    #[doc="<p>The AWS release date of the public base image. For private images, this date is the release date of the base image from which the image was created.</p>"]
    #[serde(rename="PublicBaseImageReleasedDate")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub public_base_image_released_date: Option<Timestamp>,
    #[doc="<p>The image starts in the <b>PENDING</b> state, and then moves to <b>AVAILABLE</b> if image creation succeeds and <b>FAILED</b> if image creation has failed.</p>"]
    #[serde(rename="State")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub state: Option<ImageState>,
    #[doc="<p>The reason why the last state change occurred.</p>"]
    #[serde(rename="StateChangeReason")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub state_change_reason: Option<ImageStateChangeReason>,
    #[doc="<p>The visibility of an image to the user; images can be public or private.</p>"]
    #[serde(rename="Visibility")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub visibility: Option<VisibilityType>,
}

pub type ImageList = Vec<Image>;
pub type ImageState = String;
#[doc="<p>The reason why the last state change occurred.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct ImageStateChangeReason {
    #[doc="<p>The state change reason code of the image.</p>"]
    #[serde(rename="Code")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub code: Option<ImageStateChangeReasonCode>,
    #[doc="<p>The state change reason message to the end user.</p>"]
    #[serde(rename="Message")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub message: Option<String>,
}

pub type ImageStateChangeReasonCode = String;
pub type Integer = i64;
#[derive(Default,Debug,Clone,Serialize)]
pub struct ListAssociatedFleetsRequest {
    #[doc="<p>The pagination token to use to retrieve the next page of results for this operation. If this value is null, it retrieves the first page.</p>"]
    #[serde(rename="NextToken")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub next_token: Option<String>,
    #[doc="<p>The name of the stack whose associated fleets are listed.</p>"]
    #[serde(rename="StackName")]
    pub stack_name: String,
}

#[doc="<p>The response from a successful operation.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct ListAssociatedFleetsResult {
    #[doc="<p>The names of associated fleets.</p>"]
    #[serde(rename="Names")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub names: Option<StringList>,
    #[doc="<p>The pagination token to use to retrieve the next page of results for this operation. If there are no more pages, this value is null.</p>"]
    #[serde(rename="NextToken")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default,Debug,Clone,Serialize)]
pub struct ListAssociatedStacksRequest {
    #[doc="<p>The name of the fleet whose associated stacks are listed.</p>"]
    #[serde(rename="FleetName")]
    pub fleet_name: String,
    #[doc="<p>The pagination token to use to retrieve the next page of results for this operation. If this value is null, it retrieves the first page.</p>"]
    #[serde(rename="NextToken")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub next_token: Option<String>,
}

#[doc="<p>The response from a successful operation.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct ListAssociatedStacksResult {
    #[doc="<p>The names of associated stacks.</p>"]
    #[serde(rename="Names")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub names: Option<StringList>,
    #[doc="<p>The pagination token to use to retrieve the next page of results for this operation. If there are no more pages, this value is null.</p>"]
    #[serde(rename="NextToken")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub next_token: Option<String>,
}

pub type Long = i64;
pub type Metadata = ::std::collections::HashMap<String, String>;
pub type Name = String;
pub type PlatformType = String;
#[doc="<p>The ARN of the resource.</p>"]
pub type ResourceIdentifier = String;
#[doc="<p>Contains the parameters for a streaming session.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct Session {
    #[doc="<p>The authentication method of the user for whom the session was created. It can be <code>API</code> for a user authenticated using a streaming URL or <code>SAML</code> for a SAML federated user.</p>"]
    #[serde(rename="AuthenticationType")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub authentication_type: Option<AuthenticationType>,
    #[doc="<p>The name of the fleet for which the streaming session was created.</p>"]
    #[serde(rename="FleetName")]
    pub fleet_name: String,
    #[doc="<p>The unique ID for a streaming session.</p>"]
    #[serde(rename="Id")]
    pub id: String,
    #[doc="<p>The name of the stack for which the streaming session was created.</p>"]
    #[serde(rename="StackName")]
    pub stack_name: String,
    #[doc="<p>The current state of the streaming session.</p>"]
    #[serde(rename="State")]
    pub state: SessionState,
    #[doc="<p>The identifier of the user for whom the session was created.</p>"]
    #[serde(rename="UserId")]
    pub user_id: UserId,
}

#[doc="<p>List of sessions.</p>"]
pub type SessionList = Vec<Session>;
#[doc="<p>Possible values for the state of a streaming session.</p>"]
pub type SessionState = String;
#[doc="<p>Details about a stack.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct Stack {
    #[doc="<p>The ARN of the stack.</p>"]
    #[serde(rename="Arn")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub arn: Option<Arn>,
    #[doc="<p>The timestamp when the stack was created.</p>"]
    #[serde(rename="CreatedTime")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub created_time: Option<Timestamp>,
    #[doc="<p>A meaningful description for the stack.</p>"]
    #[serde(rename="Description")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub description: Option<String>,
    #[doc="<p>A display name for the stack.</p>"]
    #[serde(rename="DisplayName")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub display_name: Option<String>,
    #[doc="<p>The unique identifier of the stack.</p>"]
    #[serde(rename="Name")]
    pub name: String,
    #[doc="<p>The list of errors associated with the stack.</p>"]
    #[serde(rename="StackErrors")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub stack_errors: Option<StackErrors>,
    #[doc="<p>The storage connectors to be enabled for the stack.</p>"]
    #[serde(rename="StorageConnectors")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub storage_connectors: Option<StorageConnectorList>,
}

#[doc="<p>Contains the parameters for a stack error.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct StackError {
    #[doc="<p>The error code of a stack error.</p>"]
    #[serde(rename="ErrorCode")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub error_code: Option<StackErrorCode>,
    #[doc="<p>The error message of a stack error.</p>"]
    #[serde(rename="ErrorMessage")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub error_message: Option<String>,
}

pub type StackErrorCode = String;
#[doc="<p>A list of stack errors.</p>"]
pub type StackErrors = Vec<StackError>;
#[doc="<p>A list of stacks.</p>"]
pub type StackList = Vec<Stack>;
#[derive(Default,Debug,Clone,Serialize)]
pub struct StartFleetRequest {
    #[doc="<p>The name of the fleet to start.</p>"]
    #[serde(rename="Name")]
    pub name: String,
}

#[derive(Default,Debug,Clone,Deserialize)]
pub struct StartFleetResult;

#[derive(Default,Debug,Clone,Serialize)]
pub struct StopFleetRequest {
    #[doc="<p>The name of the fleet to stop.</p>"]
    #[serde(rename="Name")]
    pub name: String,
}

#[derive(Default,Debug,Clone,Deserialize)]
pub struct StopFleetResult;

#[doc="<p>Contains the parameters for a storage connector.</p>"]
#[derive(Default,Debug,Clone,Serialize,Deserialize)]
pub struct StorageConnector {
    #[doc="<p>The type of storage connector. The possible values include: HOMEFOLDERS.</p>"]
    #[serde(rename="ConnectorType")]
    pub connector_type: StorageConnectorType,
    #[doc="<p>The ARN associated with the storage connector.</p>"]
    #[serde(rename="ResourceIdentifier")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub resource_identifier: Option<ResourceIdentifier>,
}

#[doc="<p>A list of storage connectors.</p>"]
pub type StorageConnectorList = Vec<StorageConnector>;
#[doc="<p>The type of storage connector. The possible values include: HOMEFOLDERS.</p>"]
pub type StorageConnectorType = String;
pub type StringList = Vec<String>;
#[doc="<p>A list of subnet IDs.</p>"]
pub type SubnetIdList = Vec<String>;
pub type Timestamp = f64;
#[derive(Default,Debug,Clone,Serialize)]
pub struct UpdateFleetRequest {
    #[doc="<p>The parameters for the capacity allocated to the fleet. </p>"]
    #[serde(rename="ComputeCapacity")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub compute_capacity: Option<ComputeCapacity>,
    #[doc="<p>Delete the VPC association for the specified fleet.</p>"]
    #[serde(rename="DeleteVpcConfig")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub delete_vpc_config: Option<Boolean>,
    #[doc="<p>The description displayed to end users on the AppStream 2.0 portal.</p>"]
    #[serde(rename="Description")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub description: Option<Description>,
    #[doc="<p>The time after disconnection when a session is considered to have ended. If a user who got disconnected reconnects within this timeout interval, the user is connected back to their previous session. The input can be any numeric value in seconds between 60 and 57600.</p>"]
    #[serde(rename="DisconnectTimeoutInSeconds")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub disconnect_timeout_in_seconds: Option<Integer>,
    #[doc="<p>The name displayed to end users on the AppStream 2.0 portal.</p>"]
    #[serde(rename="DisplayName")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub display_name: Option<DisplayName>,
    #[doc="<p>Enables or disables default Internet access for the fleet.</p>"]
    #[serde(rename="EnableDefaultInternetAccess")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub enable_default_internet_access: Option<BooleanObject>,
    #[doc="<p>The image name from which a fleet is created.</p>"]
    #[serde(rename="ImageName")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub image_name: Option<String>,
    #[doc="<p>The instance type of compute resources for the fleet. Fleet instances are launched from this instance type.</p>"]
    #[serde(rename="InstanceType")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub instance_type: Option<String>,
    #[doc="<p>The maximum time for which a streaming session can run. The input can be any numeric value in seconds between 600 and 57600.</p>"]
    #[serde(rename="MaxUserDurationInSeconds")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub max_user_duration_in_seconds: Option<Integer>,
    #[doc="<p>The name of the fleet.</p>"]
    #[serde(rename="Name")]
    pub name: String,
    #[doc="<p>The VPC configuration for the fleet.</p>"]
    #[serde(rename="VpcConfig")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub vpc_config: Option<VpcConfig>,
}

#[derive(Default,Debug,Clone,Deserialize)]
pub struct UpdateFleetResult {
    #[doc="<p>A list of fleet details.</p>"]
    #[serde(rename="Fleet")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub fleet: Option<Fleet>,
}

#[derive(Default,Debug,Clone,Serialize)]
pub struct UpdateStackRequest {
    #[doc="<p>Remove all the storage connectors currently enabled for the stack.</p>"]
    #[serde(rename="DeleteStorageConnectors")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub delete_storage_connectors: Option<Boolean>,
    #[doc="<p>The description displayed to end users on the AppStream 2.0 portal.</p>"]
    #[serde(rename="Description")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub description: Option<Description>,
    #[doc="<p>The name displayed to end users on the AppStream 2.0 portal.</p>"]
    #[serde(rename="DisplayName")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub display_name: Option<DisplayName>,
    #[doc="<p>The name of the stack to update.</p>"]
    #[serde(rename="Name")]
    pub name: String,
    #[doc="<p>The storage connectors to be enabled for the stack.</p>"]
    #[serde(rename="StorageConnectors")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub storage_connectors: Option<StorageConnectorList>,
}

#[derive(Default,Debug,Clone,Deserialize)]
pub struct UpdateStackResult {
    #[doc="<p>A list of stack details.</p>"]
    #[serde(rename="Stack")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub stack: Option<Stack>,
}

pub type UserId = String;
pub type VisibilityType = String;
#[doc="<p>VPC configuration information.</p>"]
#[derive(Default,Debug,Clone,Serialize,Deserialize)]
pub struct VpcConfig {
    #[doc="<p>The list of subnets to which a network interface is established from the fleet instance.</p>"]
    #[serde(rename="SubnetIds")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub subnet_ids: Option<SubnetIdList>,
}

/// Errors returned by AssociateFleet
#[derive(Debug, PartialEq)]
pub enum AssociateFleetError {
    ///<p>An API error occurred. Wait a few minutes and try again.</p>
    ConcurrentModification(String),
    ///<p>The image does not support storage connectors.</p>
    IncompatibleImage(String),
    ///<p>The requested limit exceeds the permitted limit for an account.</p>
    LimitExceeded(String),
    ///<p>The specified resource was not found.</p>
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
                let raw_error_type = json.get("__type")
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
                    "LimitExceededException" => {
                        AssociateFleetError::LimitExceeded(String::from(error_message))
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
            AssociateFleetError::LimitExceeded(ref cause) => cause,
            AssociateFleetError::ResourceNotFound(ref cause) => cause,
            AssociateFleetError::Validation(ref cause) => cause,
            AssociateFleetError::Credentials(ref err) => err.description(),
            AssociateFleetError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            AssociateFleetError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by CreateFleet
#[derive(Debug, PartialEq)]
pub enum CreateFleetError {
    ///<p>An API error occurred. Wait a few minutes and try again.</p>
    ConcurrentModification(String),
    ///<p>The specified role is invalid.</p>
    InvalidRole(String),
    ///<p>The requested limit exceeds the permitted limit for an account.</p>
    LimitExceeded(String),
    ///<p>The specified resource already exists.</p>
    ResourceAlreadyExists(String),
    ///<p>The specified resource exists and is not in use, but isn't available.</p>
    ResourceNotAvailable(String),
    ///<p>The specified resource was not found.</p>
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
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "ConcurrentModificationException" => {
                        CreateFleetError::ConcurrentModification(String::from(error_message))
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
impl fmt::Display for CreateFleetError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateFleetError {
    fn description(&self) -> &str {
        match *self {
            CreateFleetError::ConcurrentModification(ref cause) => cause,
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
/// Errors returned by CreateStack
#[derive(Debug, PartialEq)]
pub enum CreateStackError {
    ///<p>An API error occurred. Wait a few minutes and try again.</p>
    ConcurrentModification(String),
    ///<p>Indicates an incorrect combination of parameters, or a missing parameter.</p>
    InvalidParameterCombination(String),
    ///<p>The specified role is invalid.</p>
    InvalidRole(String),
    ///<p>The requested limit exceeds the permitted limit for an account.</p>
    LimitExceeded(String),
    ///<p>The specified resource already exists.</p>
    ResourceAlreadyExists(String),
    ///<p>The specified resource was not found.</p>
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
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "ConcurrentModificationException" => {
                        CreateStackError::ConcurrentModification(String::from(error_message))
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
impl fmt::Display for CreateStackError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateStackError {
    fn description(&self) -> &str {
        match *self {
            CreateStackError::ConcurrentModification(ref cause) => cause,
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
    ///<p>Indicates an incorrect combination of parameters, or a missing parameter.</p>
    InvalidParameterCombination(String),
    ///<p>The attempted operation is not permitted.</p>
    OperationNotPermitted(String),
    ///<p>The specified resource exists and is not in use, but isn't available.</p>
    ResourceNotAvailable(String),
    ///<p>The specified resource was not found.</p>
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
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InvalidParameterCombinationException" => CreateStreamingURLError::InvalidParameterCombination(String::from(error_message)),
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
/// Errors returned by DeleteFleet
#[derive(Debug, PartialEq)]
pub enum DeleteFleetError {
    ///<p>An API error occurred. Wait a few minutes and try again.</p>
    ConcurrentModification(String),
    ///<p>The specified resource is in use.</p>
    ResourceInUse(String),
    ///<p>The specified resource was not found.</p>
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
                let raw_error_type = json.get("__type")
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
/// Errors returned by DeleteStack
#[derive(Debug, PartialEq)]
pub enum DeleteStackError {
    ///<p>An API error occurred. Wait a few minutes and try again.</p>
    ConcurrentModification(String),
    ///<p>The specified resource is in use.</p>
    ResourceInUse(String),
    ///<p>The specified resource was not found.</p>
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
                let raw_error_type = json.get("__type")
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
/// Errors returned by DescribeFleets
#[derive(Debug, PartialEq)]
pub enum DescribeFleetsError {
    ///<p>The specified resource was not found.</p>
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
                let raw_error_type = json.get("__type")
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
/// Errors returned by DescribeImages
#[derive(Debug, PartialEq)]
pub enum DescribeImagesError {
    ///<p>The specified resource was not found.</p>
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
                let raw_error_type = json.get("__type")
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
    ///<p>Indicates an incorrect combination of parameters, or a missing parameter.</p>
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
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InvalidParameterCombinationException" => DescribeSessionsError::InvalidParameterCombination(String::from(error_message)),
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
    ///<p>The specified resource was not found.</p>
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
                let raw_error_type = json.get("__type")
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
    ///<p>An API error occurred. Wait a few minutes and try again.</p>
    ConcurrentModification(String),
    ///<p>The specified resource is in use.</p>
    ResourceInUse(String),
    ///<p>The specified resource was not found.</p>
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
                let raw_error_type = json.get("__type")
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
                let raw_error_type = json.get("__type")
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
                let raw_error_type = json.get("__type")
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
                let raw_error_type = json.get("__type")
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
/// Errors returned by StartFleet
#[derive(Debug, PartialEq)]
pub enum StartFleetError {
    ///<p>An API error occurred. Wait a few minutes and try again.</p>
    ConcurrentModification(String),
    ///<p>The requested limit exceeds the permitted limit for an account.</p>
    LimitExceeded(String),
    ///<p>The attempted operation is not permitted.</p>
    OperationNotPermitted(String),
    ///<p>The specified resource was not found.</p>
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
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "ConcurrentModificationException" => {
                        StartFleetError::ConcurrentModification(String::from(error_message))
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
impl fmt::Display for StartFleetError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for StartFleetError {
    fn description(&self) -> &str {
        match *self {
            StartFleetError::ConcurrentModification(ref cause) => cause,
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
/// Errors returned by StopFleet
#[derive(Debug, PartialEq)]
pub enum StopFleetError {
    ///<p>An API error occurred. Wait a few minutes and try again.</p>
    ConcurrentModification(String),
    ///<p>The specified resource was not found.</p>
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
                let raw_error_type = json.get("__type")
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
/// Errors returned by UpdateFleet
#[derive(Debug, PartialEq)]
pub enum UpdateFleetError {
    ///<p>An API error occurred. Wait a few minutes and try again.</p>
    ConcurrentModification(String),
    ///<p>The image does not support storage connectors.</p>
    IncompatibleImage(String),
    ///<p>Indicates an incorrect combination of parameters, or a missing parameter.</p>
    InvalidParameterCombination(String),
    ///<p>The specified role is invalid.</p>
    InvalidRole(String),
    ///<p>The requested limit exceeds the permitted limit for an account.</p>
    LimitExceeded(String),
    ///<p>The specified resource is in use.</p>
    ResourceInUse(String),
    ///<p>The specified resource exists and is not in use, but isn't available.</p>
    ResourceNotAvailable(String),
    ///<p>The specified resource was not found.</p>
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
                let raw_error_type = json.get("__type")
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
                    "InvalidParameterCombinationException" => {
                        UpdateFleetError::InvalidParameterCombination(String::from(error_message))
                    }
                    "InvalidRoleException" => {
                        UpdateFleetError::InvalidRole(String::from(error_message))
                    }
                    "LimitExceededException" => {
                        UpdateFleetError::LimitExceeded(String::from(error_message))
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
            UpdateFleetError::InvalidParameterCombination(ref cause) => cause,
            UpdateFleetError::InvalidRole(ref cause) => cause,
            UpdateFleetError::LimitExceeded(ref cause) => cause,
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
    ///<p>The image does not support storage connectors.</p>
    IncompatibleImage(String),
    ///<p>Indicates an incorrect combination of parameters, or a missing parameter.</p>
    InvalidParameterCombination(String),
    ///<p>The specified role is invalid.</p>
    InvalidRole(String),
    ///<p>The requested limit exceeds the permitted limit for an account.</p>
    LimitExceeded(String),
    ///<p>The specified resource is in use.</p>
    ResourceInUse(String),
    ///<p>The specified resource was not found.</p>
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
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "IncompatibleImageException" => {
                        UpdateStackError::IncompatibleImage(String::from(error_message))
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
impl fmt::Display for UpdateStackError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for UpdateStackError {
    fn description(&self) -> &str {
        match *self {
            UpdateStackError::IncompatibleImage(ref cause) => cause,
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
    #[doc="<p>Associate a fleet to a stack.</p>"]
    fn associate_fleet(&self,
                       input: &AssociateFleetRequest)
                       -> Result<AssociateFleetResult, AssociateFleetError>;


    #[doc="<p>Creates a new fleet.</p>"]
    fn create_fleet(&self,
                    input: &CreateFleetRequest)
                    -> Result<CreateFleetResult, CreateFleetError>;


    #[doc="<p>Create a new stack.</p>"]
    fn create_stack(&self,
                    input: &CreateStackRequest)
                    -> Result<CreateStackResult, CreateStackError>;


    #[doc="<p>Creates a URL to start an AppStream 2.0 streaming session for a user. By default, the URL is valid only for 1 minute from the time that it is generated.</p>"]
    fn create_streaming_url(&self,
                            input: &CreateStreamingURLRequest)
                            -> Result<CreateStreamingURLResult, CreateStreamingURLError>;


    #[doc="<p>Deletes a fleet.</p>"]
    fn delete_fleet(&self,
                    input: &DeleteFleetRequest)
                    -> Result<DeleteFleetResult, DeleteFleetError>;


    #[doc="<p>Deletes the stack. After this operation completes, the environment can no longer be activated, and any reservations made for the stack are released.</p>"]
    fn delete_stack(&self,
                    input: &DeleteStackRequest)
                    -> Result<DeleteStackResult, DeleteStackError>;


    #[doc="<p>If fleet names are provided, this operation describes the specified fleets; otherwise, all the fleets in the account are described.</p>"]
    fn describe_fleets(&self,
                       input: &DescribeFleetsRequest)
                       -> Result<DescribeFleetsResult, DescribeFleetsError>;


    #[doc="<p>Describes the images. If a list of names is not provided, all images in your account are returned. This operation does not return a paginated result.</p>"]
    fn describe_images(&self,
                       input: &DescribeImagesRequest)
                       -> Result<DescribeImagesResult, DescribeImagesError>;


    #[doc="<p>Describes the streaming sessions for a stack and a fleet. If a user ID is provided, this operation returns streaming sessions for only that user. Pass this value for the <code>nextToken</code> parameter in a subsequent call to this operation to retrieve the next set of items. If an authentication type is not provided, the operation defaults to users authenticated using a streaming URL.</p>"]
    fn describe_sessions(&self,
                         input: &DescribeSessionsRequest)
                         -> Result<DescribeSessionsResult, DescribeSessionsError>;


    #[doc="<p>If stack names are not provided, this operation describes the specified stacks; otherwise, all stacks in the account are described. Pass the <code>nextToken</code> value in a subsequent call to this operation to retrieve the next set of items.</p>"]
    fn describe_stacks(&self,
                       input: &DescribeStacksRequest)
                       -> Result<DescribeStacksResult, DescribeStacksError>;


    #[doc="<p>Disassociates a fleet from a stack.</p>"]
    fn disassociate_fleet(&self,
                          input: &DisassociateFleetRequest)
                          -> Result<DisassociateFleetResult, DisassociateFleetError>;


    #[doc="<p>This operation immediately stops a streaming session.</p>"]
    fn expire_session(&self,
                      input: &ExpireSessionRequest)
                      -> Result<ExpireSessionResult, ExpireSessionError>;


    #[doc="<p>Lists all fleets associated with the stack.</p>"]
    fn list_associated_fleets(&self,
                              input: &ListAssociatedFleetsRequest)
                              -> Result<ListAssociatedFleetsResult, ListAssociatedFleetsError>;


    #[doc="<p>Lists all stacks to which the specified fleet is associated.</p>"]
    fn list_associated_stacks(&self,
                              input: &ListAssociatedStacksRequest)
                              -> Result<ListAssociatedStacksResult, ListAssociatedStacksError>;


    #[doc="<p>Starts a fleet.</p>"]
    fn start_fleet(&self, input: &StartFleetRequest) -> Result<StartFleetResult, StartFleetError>;


    #[doc="<p>Stops a fleet.</p>"]
    fn stop_fleet(&self, input: &StopFleetRequest) -> Result<StopFleetResult, StopFleetError>;


    #[doc="<p>Updates an existing fleet. All the attributes except the fleet name can be updated in the <b>STOPPED</b> state. When a fleet is in the <b>RUNNING</b> state, only <code>DisplayName</code> and <code>ComputeCapacity</code> can be updated. A fleet cannot be updated in a status of <b>STARTING</b> or <b>STOPPING</b>.</p>"]
    fn update_fleet(&self,
                    input: &UpdateFleetRequest)
                    -> Result<UpdateFleetResult, UpdateFleetError>;


    #[doc="<p>Updates the specified fields in the stack with the specified name.</p>"]
    fn update_stack(&self,
                    input: &UpdateStackRequest)
                    -> Result<UpdateStackResult, UpdateStackError>;
}
/// A client for the Amazon AppStream API.
pub struct AppStreamClient<P, D>
    where P: ProvideAwsCredentials,
          D: DispatchSignedRequest
{
    credentials_provider: P,
    region: region::Region,
    dispatcher: D,
}

impl<P, D> AppStreamClient<P, D>
    where P: ProvideAwsCredentials,
          D: DispatchSignedRequest
{
    pub fn new(request_dispatcher: D, credentials_provider: P, region: region::Region) -> Self {
        AppStreamClient {
            credentials_provider: credentials_provider,
            region: region,
            dispatcher: request_dispatcher,
        }
    }
}

impl<P, D> AppStream for AppStreamClient<P, D>
    where P: ProvideAwsCredentials,
          D: DispatchSignedRequest
{
    #[doc="<p>Associate a fleet to a stack.</p>"]
    fn associate_fleet(&self,
                       input: &AssociateFleetRequest)
                       -> Result<AssociateFleetResult, AssociateFleetError> {
        let mut request = SignedRequest::new("POST", "appstream", self.region, "/");
        request.set_endpoint_prefix("appstream2".to_string());
        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "PhotonAdminProxyService.AssociateFleet");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign(&try!(self.credentials_provider.credentials()));

        let response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                            Ok(serde_json::from_str::<AssociateFleetResult>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
                        }
            _ => {
                Err(AssociateFleetError::from_body(String::from_utf8_lossy(&response.body)
                                                       .as_ref()))
            }
        }
    }


    #[doc="<p>Creates a new fleet.</p>"]
    fn create_fleet(&self,
                    input: &CreateFleetRequest)
                    -> Result<CreateFleetResult, CreateFleetError> {
        let mut request = SignedRequest::new("POST", "appstream", self.region, "/");
        request.set_endpoint_prefix("appstream2".to_string());
        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "PhotonAdminProxyService.CreateFleet");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign(&try!(self.credentials_provider.credentials()));

        let response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                            Ok(serde_json::from_str::<CreateFleetResult>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
                        }
            _ => Err(CreateFleetError::from_body(String::from_utf8_lossy(&response.body).as_ref())),
        }
    }


    #[doc="<p>Create a new stack.</p>"]
    fn create_stack(&self,
                    input: &CreateStackRequest)
                    -> Result<CreateStackResult, CreateStackError> {
        let mut request = SignedRequest::new("POST", "appstream", self.region, "/");
        request.set_endpoint_prefix("appstream2".to_string());
        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "PhotonAdminProxyService.CreateStack");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign(&try!(self.credentials_provider.credentials()));

        let response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                            Ok(serde_json::from_str::<CreateStackResult>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
                        }
            _ => Err(CreateStackError::from_body(String::from_utf8_lossy(&response.body).as_ref())),
        }
    }


    #[doc="<p>Creates a URL to start an AppStream 2.0 streaming session for a user. By default, the URL is valid only for 1 minute from the time that it is generated.</p>"]
    fn create_streaming_url(&self,
                            input: &CreateStreamingURLRequest)
                            -> Result<CreateStreamingURLResult, CreateStreamingURLError> {
        let mut request = SignedRequest::new("POST", "appstream", self.region, "/");
        request.set_endpoint_prefix("appstream2".to_string());
        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "PhotonAdminProxyService.CreateStreamingURL");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign(&try!(self.credentials_provider.credentials()));

        let response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                            Ok(serde_json::from_str::<CreateStreamingURLResult>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
                        }
            _ => {
                Err(CreateStreamingURLError::from_body(String::from_utf8_lossy(&response.body)
                                                           .as_ref()))
            }
        }
    }


    #[doc="<p>Deletes a fleet.</p>"]
    fn delete_fleet(&self,
                    input: &DeleteFleetRequest)
                    -> Result<DeleteFleetResult, DeleteFleetError> {
        let mut request = SignedRequest::new("POST", "appstream", self.region, "/");
        request.set_endpoint_prefix("appstream2".to_string());
        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "PhotonAdminProxyService.DeleteFleet");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign(&try!(self.credentials_provider.credentials()));

        let response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                            Ok(serde_json::from_str::<DeleteFleetResult>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
                        }
            _ => Err(DeleteFleetError::from_body(String::from_utf8_lossy(&response.body).as_ref())),
        }
    }


    #[doc="<p>Deletes the stack. After this operation completes, the environment can no longer be activated, and any reservations made for the stack are released.</p>"]
    fn delete_stack(&self,
                    input: &DeleteStackRequest)
                    -> Result<DeleteStackResult, DeleteStackError> {
        let mut request = SignedRequest::new("POST", "appstream", self.region, "/");
        request.set_endpoint_prefix("appstream2".to_string());
        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "PhotonAdminProxyService.DeleteStack");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign(&try!(self.credentials_provider.credentials()));

        let response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                            Ok(serde_json::from_str::<DeleteStackResult>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
                        }
            _ => Err(DeleteStackError::from_body(String::from_utf8_lossy(&response.body).as_ref())),
        }
    }


    #[doc="<p>If fleet names are provided, this operation describes the specified fleets; otherwise, all the fleets in the account are described.</p>"]
    fn describe_fleets(&self,
                       input: &DescribeFleetsRequest)
                       -> Result<DescribeFleetsResult, DescribeFleetsError> {
        let mut request = SignedRequest::new("POST", "appstream", self.region, "/");
        request.set_endpoint_prefix("appstream2".to_string());
        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "PhotonAdminProxyService.DescribeFleets");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign(&try!(self.credentials_provider.credentials()));

        let response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                            Ok(serde_json::from_str::<DescribeFleetsResult>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
                        }
            _ => {
                Err(DescribeFleetsError::from_body(String::from_utf8_lossy(&response.body)
                                                       .as_ref()))
            }
        }
    }


    #[doc="<p>Describes the images. If a list of names is not provided, all images in your account are returned. This operation does not return a paginated result.</p>"]
    fn describe_images(&self,
                       input: &DescribeImagesRequest)
                       -> Result<DescribeImagesResult, DescribeImagesError> {
        let mut request = SignedRequest::new("POST", "appstream", self.region, "/");
        request.set_endpoint_prefix("appstream2".to_string());
        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "PhotonAdminProxyService.DescribeImages");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign(&try!(self.credentials_provider.credentials()));

        let response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                            Ok(serde_json::from_str::<DescribeImagesResult>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
                        }
            _ => {
                Err(DescribeImagesError::from_body(String::from_utf8_lossy(&response.body)
                                                       .as_ref()))
            }
        }
    }


    #[doc="<p>Describes the streaming sessions for a stack and a fleet. If a user ID is provided, this operation returns streaming sessions for only that user. Pass this value for the <code>nextToken</code> parameter in a subsequent call to this operation to retrieve the next set of items. If an authentication type is not provided, the operation defaults to users authenticated using a streaming URL.</p>"]
    fn describe_sessions(&self,
                         input: &DescribeSessionsRequest)
                         -> Result<DescribeSessionsResult, DescribeSessionsError> {
        let mut request = SignedRequest::new("POST", "appstream", self.region, "/");
        request.set_endpoint_prefix("appstream2".to_string());
        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "PhotonAdminProxyService.DescribeSessions");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign(&try!(self.credentials_provider.credentials()));

        let response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                            Ok(serde_json::from_str::<DescribeSessionsResult>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
                        }
            _ => {
                Err(DescribeSessionsError::from_body(String::from_utf8_lossy(&response.body)
                                                         .as_ref()))
            }
        }
    }


    #[doc="<p>If stack names are not provided, this operation describes the specified stacks; otherwise, all stacks in the account are described. Pass the <code>nextToken</code> value in a subsequent call to this operation to retrieve the next set of items.</p>"]
    fn describe_stacks(&self,
                       input: &DescribeStacksRequest)
                       -> Result<DescribeStacksResult, DescribeStacksError> {
        let mut request = SignedRequest::new("POST", "appstream", self.region, "/");
        request.set_endpoint_prefix("appstream2".to_string());
        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "PhotonAdminProxyService.DescribeStacks");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign(&try!(self.credentials_provider.credentials()));

        let response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                            Ok(serde_json::from_str::<DescribeStacksResult>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
                        }
            _ => {
                Err(DescribeStacksError::from_body(String::from_utf8_lossy(&response.body)
                                                       .as_ref()))
            }
        }
    }


    #[doc="<p>Disassociates a fleet from a stack.</p>"]
    fn disassociate_fleet(&self,
                          input: &DisassociateFleetRequest)
                          -> Result<DisassociateFleetResult, DisassociateFleetError> {
        let mut request = SignedRequest::new("POST", "appstream", self.region, "/");
        request.set_endpoint_prefix("appstream2".to_string());
        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "PhotonAdminProxyService.DisassociateFleet");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign(&try!(self.credentials_provider.credentials()));

        let response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                            Ok(serde_json::from_str::<DisassociateFleetResult>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
                        }
            _ => {
                Err(DisassociateFleetError::from_body(String::from_utf8_lossy(&response.body)
                                                          .as_ref()))
            }
        }
    }


    #[doc="<p>This operation immediately stops a streaming session.</p>"]
    fn expire_session(&self,
                      input: &ExpireSessionRequest)
                      -> Result<ExpireSessionResult, ExpireSessionError> {
        let mut request = SignedRequest::new("POST", "appstream", self.region, "/");
        request.set_endpoint_prefix("appstream2".to_string());
        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "PhotonAdminProxyService.ExpireSession");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign(&try!(self.credentials_provider.credentials()));

        let response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                            Ok(serde_json::from_str::<ExpireSessionResult>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
                        }
            _ => {
                Err(ExpireSessionError::from_body(String::from_utf8_lossy(&response.body).as_ref()))
            }
        }
    }


    #[doc="<p>Lists all fleets associated with the stack.</p>"]
    fn list_associated_fleets(&self,
                              input: &ListAssociatedFleetsRequest)
                              -> Result<ListAssociatedFleetsResult, ListAssociatedFleetsError> {
        let mut request = SignedRequest::new("POST", "appstream", self.region, "/");
        request.set_endpoint_prefix("appstream2".to_string());
        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target",
                           "PhotonAdminProxyService.ListAssociatedFleets");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign(&try!(self.credentials_provider.credentials()));

        let response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                            Ok(serde_json::from_str::<ListAssociatedFleetsResult>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
                        }
            _ => {
                Err(ListAssociatedFleetsError::from_body(String::from_utf8_lossy(&response.body)
                                                             .as_ref()))
            }
        }
    }


    #[doc="<p>Lists all stacks to which the specified fleet is associated.</p>"]
    fn list_associated_stacks(&self,
                              input: &ListAssociatedStacksRequest)
                              -> Result<ListAssociatedStacksResult, ListAssociatedStacksError> {
        let mut request = SignedRequest::new("POST", "appstream", self.region, "/");
        request.set_endpoint_prefix("appstream2".to_string());
        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target",
                           "PhotonAdminProxyService.ListAssociatedStacks");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign(&try!(self.credentials_provider.credentials()));

        let response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                            Ok(serde_json::from_str::<ListAssociatedStacksResult>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
                        }
            _ => {
                Err(ListAssociatedStacksError::from_body(String::from_utf8_lossy(&response.body)
                                                             .as_ref()))
            }
        }
    }


    #[doc="<p>Starts a fleet.</p>"]
    fn start_fleet(&self, input: &StartFleetRequest) -> Result<StartFleetResult, StartFleetError> {
        let mut request = SignedRequest::new("POST", "appstream", self.region, "/");
        request.set_endpoint_prefix("appstream2".to_string());
        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "PhotonAdminProxyService.StartFleet");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign(&try!(self.credentials_provider.credentials()));

        let response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                            Ok(serde_json::from_str::<StartFleetResult>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
                        }
            _ => Err(StartFleetError::from_body(String::from_utf8_lossy(&response.body).as_ref())),
        }
    }


    #[doc="<p>Stops a fleet.</p>"]
    fn stop_fleet(&self, input: &StopFleetRequest) -> Result<StopFleetResult, StopFleetError> {
        let mut request = SignedRequest::new("POST", "appstream", self.region, "/");
        request.set_endpoint_prefix("appstream2".to_string());
        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "PhotonAdminProxyService.StopFleet");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign(&try!(self.credentials_provider.credentials()));

        let response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                Ok(serde_json::from_str::<StopFleetResult>(String::from_utf8_lossy(&response.body)
                                                               .as_ref())
                           .unwrap())
            }
            _ => Err(StopFleetError::from_body(String::from_utf8_lossy(&response.body).as_ref())),
        }
    }


    #[doc="<p>Updates an existing fleet. All the attributes except the fleet name can be updated in the <b>STOPPED</b> state. When a fleet is in the <b>RUNNING</b> state, only <code>DisplayName</code> and <code>ComputeCapacity</code> can be updated. A fleet cannot be updated in a status of <b>STARTING</b> or <b>STOPPING</b>.</p>"]
    fn update_fleet(&self,
                    input: &UpdateFleetRequest)
                    -> Result<UpdateFleetResult, UpdateFleetError> {
        let mut request = SignedRequest::new("POST", "appstream", self.region, "/");
        request.set_endpoint_prefix("appstream2".to_string());
        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "PhotonAdminProxyService.UpdateFleet");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign(&try!(self.credentials_provider.credentials()));

        let response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                            Ok(serde_json::from_str::<UpdateFleetResult>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
                        }
            _ => Err(UpdateFleetError::from_body(String::from_utf8_lossy(&response.body).as_ref())),
        }
    }


    #[doc="<p>Updates the specified fields in the stack with the specified name.</p>"]
    fn update_stack(&self,
                    input: &UpdateStackRequest)
                    -> Result<UpdateStackResult, UpdateStackError> {
        let mut request = SignedRequest::new("POST", "appstream", self.region, "/");
        request.set_endpoint_prefix("appstream2".to_string());
        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "PhotonAdminProxyService.UpdateStack");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign(&try!(self.credentials_provider.credentials()));

        let response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                            Ok(serde_json::from_str::<UpdateStackResult>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
                        }
            _ => Err(UpdateStackError::from_body(String::from_utf8_lossy(&response.body).as_ref())),
        }
    }
}

#[cfg(test)]
mod protocol_tests {}
