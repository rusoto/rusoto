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
use rusoto_core::request::HttpResponse;
use rusoto_core::signature::SignedRequest;
#[allow(unused_imports)]
use serde::{Deserialize, Serialize};

impl AppStreamClient {
    fn new_signed_request(&self, http_method: &str, request_uri: &str) -> SignedRequest {
        let mut request = SignedRequest::new(http_method, "appstream", &self.region, request_uri);
        request.set_endpoint_prefix("appstream2".to_string());

        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request
    }

    async fn sign_and_dispatch<E>(
        &self,
        request: SignedRequest,
        from_response: fn(BufferedHttpResponse) -> RusotoError<E>,
    ) -> Result<HttpResponse, RusotoError<E>> {
        let mut response = self.client.sign_and_dispatch(request).await?;
        if !response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            return Err(from_response(response));
        }

        Ok(response)
    }
}

use serde_json;
/// <p>Describes an interface VPC endpoint (interface endpoint) that lets you create a private connection between the virtual private cloud (VPC) that you specify and AppStream 2.0. When you specify an interface endpoint for a stack, users of the stack can connect to AppStream 2.0 only through that endpoint. When you specify an interface endpoint for an image builder, administrators can connect to the image builder only through that endpoint.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct AccessEndpoint {
    /// <p>The type of interface endpoint.</p>
    #[serde(rename = "EndpointType")]
    pub endpoint_type: AccessEndpointType,
    /// <p>The identifier (ID) of the VPC in which the interface endpoint is used.</p>
    #[serde(rename = "VpceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpce_id: Option<String>,
}

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct UnknownAccessEndpointType {
    name: String,
}

#[allow(non_camel_case_types)]
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
#[non_exhaustive]
pub enum AccessEndpointType {
    Streaming,
    #[doc(hidden)]
    UnknownVariant(UnknownAccessEndpointType),
}

impl Default for AccessEndpointType {
    fn default() -> Self {
        "".into()
    }
}

impl fmt::Display for AccessEndpointType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.into())
    }
}

impl rusoto_core::param::ToParam for AccessEndpointType {
    fn to_param(&self) -> String {
        self.to_string()
    }
}

impl Into<String> for AccessEndpointType {
    fn into(self) -> String {
        match self {
            AccessEndpointType::Streaming => "STREAMING".to_string(),
            AccessEndpointType::UnknownVariant(UnknownAccessEndpointType { name: original }) => {
                original
            }
        }
    }
}

impl<'a> Into<&'a str> for &'a AccessEndpointType {
    fn into(self) -> &'a str {
        match self {
            AccessEndpointType::Streaming => &"STREAMING",
            AccessEndpointType::UnknownVariant(UnknownAccessEndpointType { name: original }) => {
                original
            }
        }
    }
}

impl From<&str> for AccessEndpointType {
    fn from(name: &str) -> Self {
        match name {
            "STREAMING" => AccessEndpointType::Streaming,
            _ => AccessEndpointType::UnknownVariant(UnknownAccessEndpointType {
                name: name.to_owned(),
            }),
        }
    }
}

impl From<String> for AccessEndpointType {
    fn from(name: String) -> Self {
        match &*name {
            "STREAMING" => AccessEndpointType::Streaming,
            _ => AccessEndpointType::UnknownVariant(UnknownAccessEndpointType { name }),
        }
    }
}

impl ::std::str::FromStr for AccessEndpointType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(s.into())
    }
}

impl Serialize for AccessEndpointType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.into())
    }
}

impl<'de> Deserialize<'de> for AccessEndpointType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Ok(String::deserialize(deserializer)?.into())
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct UnknownAction {
    name: String,
}

#[allow(non_camel_case_types)]
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
#[non_exhaustive]
pub enum Action {
    ClipboardCopyFromLocalDevice,
    ClipboardCopyToLocalDevice,
    FileDownload,
    FileUpload,
    PrintingToLocalDevice,
    #[doc(hidden)]
    UnknownVariant(UnknownAction),
}

impl Default for Action {
    fn default() -> Self {
        "".into()
    }
}

impl fmt::Display for Action {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.into())
    }
}

impl rusoto_core::param::ToParam for Action {
    fn to_param(&self) -> String {
        self.to_string()
    }
}

impl Into<String> for Action {
    fn into(self) -> String {
        match self {
            Action::ClipboardCopyFromLocalDevice => "CLIPBOARD_COPY_FROM_LOCAL_DEVICE".to_string(),
            Action::ClipboardCopyToLocalDevice => "CLIPBOARD_COPY_TO_LOCAL_DEVICE".to_string(),
            Action::FileDownload => "FILE_DOWNLOAD".to_string(),
            Action::FileUpload => "FILE_UPLOAD".to_string(),
            Action::PrintingToLocalDevice => "PRINTING_TO_LOCAL_DEVICE".to_string(),
            Action::UnknownVariant(UnknownAction { name: original }) => original,
        }
    }
}

impl<'a> Into<&'a str> for &'a Action {
    fn into(self) -> &'a str {
        match self {
            Action::ClipboardCopyFromLocalDevice => &"CLIPBOARD_COPY_FROM_LOCAL_DEVICE",
            Action::ClipboardCopyToLocalDevice => &"CLIPBOARD_COPY_TO_LOCAL_DEVICE",
            Action::FileDownload => &"FILE_DOWNLOAD",
            Action::FileUpload => &"FILE_UPLOAD",
            Action::PrintingToLocalDevice => &"PRINTING_TO_LOCAL_DEVICE",
            Action::UnknownVariant(UnknownAction { name: original }) => original,
        }
    }
}

impl From<&str> for Action {
    fn from(name: &str) -> Self {
        match name {
            "CLIPBOARD_COPY_FROM_LOCAL_DEVICE" => Action::ClipboardCopyFromLocalDevice,
            "CLIPBOARD_COPY_TO_LOCAL_DEVICE" => Action::ClipboardCopyToLocalDevice,
            "FILE_DOWNLOAD" => Action::FileDownload,
            "FILE_UPLOAD" => Action::FileUpload,
            "PRINTING_TO_LOCAL_DEVICE" => Action::PrintingToLocalDevice,
            _ => Action::UnknownVariant(UnknownAction {
                name: name.to_owned(),
            }),
        }
    }
}

impl From<String> for Action {
    fn from(name: String) -> Self {
        match &*name {
            "CLIPBOARD_COPY_FROM_LOCAL_DEVICE" => Action::ClipboardCopyFromLocalDevice,
            "CLIPBOARD_COPY_TO_LOCAL_DEVICE" => Action::ClipboardCopyToLocalDevice,
            "FILE_DOWNLOAD" => Action::FileDownload,
            "FILE_UPLOAD" => Action::FileUpload,
            "PRINTING_TO_LOCAL_DEVICE" => Action::PrintingToLocalDevice,
            _ => Action::UnknownVariant(UnknownAction { name }),
        }
    }
}

impl ::std::str::FromStr for Action {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(s.into())
    }
}

impl Serialize for Action {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.into())
    }
}

impl<'de> Deserialize<'de> for Action {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Ok(String::deserialize(deserializer)?.into())
    }
}

/// <p>Describes an application in the application catalog.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct Application {
    /// <p>The application name to display.</p>
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

/// <p>The persistent application settings for users of a stack.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ApplicationSettings {
    /// <p>Enables or disables persistent application settings for users during their streaming sessions. </p>
    #[serde(rename = "Enabled")]
    pub enabled: bool,
    /// <p>The path prefix for the S3 bucket where users’ persistent application settings are stored. You can allow the same persistent application settings to be used across multiple stacks by specifying the same settings group for each stack. </p>
    #[serde(rename = "SettingsGroup")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub settings_group: Option<String>,
}

/// <p>Describes the persistent application settings for users of a stack.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ApplicationSettingsResponse {
    /// <p>Specifies whether persistent application settings are enabled for users during their streaming sessions.</p>
    #[serde(rename = "Enabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    /// <p>The S3 bucket where users’ persistent application settings are stored. When persistent application settings are enabled for the first time for an account in an AWS Region, an S3 bucket is created. The bucket is unique to the AWS account and the Region. </p>
    #[serde(rename = "S3BucketName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_bucket_name: Option<String>,
    /// <p>The path prefix for the S3 bucket where users’ persistent application settings are stored.</p>
    #[serde(rename = "SettingsGroup")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub settings_group: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct AssociateFleetRequest {
    /// <p>The name of the fleet. </p>
    #[serde(rename = "FleetName")]
    pub fleet_name: String,
    /// <p>The name of the stack.</p>
    #[serde(rename = "StackName")]
    pub stack_name: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct AssociateFleetResult {}

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct UnknownAuthenticationType {
    name: String,
}

#[allow(non_camel_case_types)]
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
#[non_exhaustive]
pub enum AuthenticationType {
    Api,
    Saml,
    Userpool,
    #[doc(hidden)]
    UnknownVariant(UnknownAuthenticationType),
}

impl Default for AuthenticationType {
    fn default() -> Self {
        "".into()
    }
}

impl fmt::Display for AuthenticationType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.into())
    }
}

impl rusoto_core::param::ToParam for AuthenticationType {
    fn to_param(&self) -> String {
        self.to_string()
    }
}

impl Into<String> for AuthenticationType {
    fn into(self) -> String {
        match self {
            AuthenticationType::Api => "API".to_string(),
            AuthenticationType::Saml => "SAML".to_string(),
            AuthenticationType::Userpool => "USERPOOL".to_string(),
            AuthenticationType::UnknownVariant(UnknownAuthenticationType { name: original }) => {
                original
            }
        }
    }
}

impl<'a> Into<&'a str> for &'a AuthenticationType {
    fn into(self) -> &'a str {
        match self {
            AuthenticationType::Api => &"API",
            AuthenticationType::Saml => &"SAML",
            AuthenticationType::Userpool => &"USERPOOL",
            AuthenticationType::UnknownVariant(UnknownAuthenticationType { name: original }) => {
                original
            }
        }
    }
}

impl From<&str> for AuthenticationType {
    fn from(name: &str) -> Self {
        match name {
            "API" => AuthenticationType::Api,
            "SAML" => AuthenticationType::Saml,
            "USERPOOL" => AuthenticationType::Userpool,
            _ => AuthenticationType::UnknownVariant(UnknownAuthenticationType {
                name: name.to_owned(),
            }),
        }
    }
}

impl From<String> for AuthenticationType {
    fn from(name: String) -> Self {
        match &*name {
            "API" => AuthenticationType::Api,
            "SAML" => AuthenticationType::Saml,
            "USERPOOL" => AuthenticationType::Userpool,
            _ => AuthenticationType::UnknownVariant(UnknownAuthenticationType { name }),
        }
    }
}

impl ::std::str::FromStr for AuthenticationType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(s.into())
    }
}

impl Serialize for AuthenticationType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.into())
    }
}

impl<'de> Deserialize<'de> for AuthenticationType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Ok(String::deserialize(deserializer)?.into())
    }
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct BatchAssociateUserStackRequest {
    /// <p>The list of UserStackAssociation objects.</p>
    #[serde(rename = "UserStackAssociations")]
    pub user_stack_associations: Vec<UserStackAssociation>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct BatchAssociateUserStackResult {
    /// <p>The list of UserStackAssociationError objects.</p>
    #[serde(rename = "errors")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub errors: Option<Vec<UserStackAssociationError>>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct BatchDisassociateUserStackRequest {
    /// <p>The list of UserStackAssociation objects.</p>
    #[serde(rename = "UserStackAssociations")]
    pub user_stack_associations: Vec<UserStackAssociation>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct BatchDisassociateUserStackResult {
    /// <p>The list of UserStackAssociationError objects.</p>
    #[serde(rename = "errors")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub errors: Option<Vec<UserStackAssociationError>>,
}

/// <p>Describes the capacity for a fleet.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ComputeCapacity {
    /// <p>The desired number of streaming instances.</p>
    #[serde(rename = "DesiredInstances")]
    pub desired_instances: i64,
}

/// <p>Describes the capacity status for a fleet.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CopyImageResponse {
    /// <p>The name of the destination image.</p>
    #[serde(rename = "DestinationImageName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination_image_name: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateDirectoryConfigRequest {
    /// <p>The fully qualified name of the directory (for example, corp.example.com).</p>
    #[serde(rename = "DirectoryName")]
    pub directory_name: String,
    /// <p>The distinguished names of the organizational units for computer accounts.</p>
    #[serde(rename = "OrganizationalUnitDistinguishedNames")]
    pub organizational_unit_distinguished_names: Vec<String>,
    /// <p>The credentials for the service account used by the fleet or image builder to connect to the directory.</p>
    #[serde(rename = "ServiceAccountCredentials")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_account_credentials: Option<ServiceAccountCredentials>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateDirectoryConfigResult {
    /// <p>Information about the directory configuration.</p>
    #[serde(rename = "DirectoryConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub directory_config: Option<DirectoryConfig>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateFleetRequest {
    /// <p>The desired capacity for the fleet.</p>
    #[serde(rename = "ComputeCapacity")]
    pub compute_capacity: ComputeCapacity,
    /// <p>The description to display.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The amount of time that a streaming session remains active after users disconnect. If users try to reconnect to the streaming session after a disconnection or network interruption within this time interval, they are connected to their previous session. Otherwise, they are connected to a new session with a new streaming instance. </p> <p>Specify a value between 60 and 360000.</p>
    #[serde(rename = "DisconnectTimeoutInSeconds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disconnect_timeout_in_seconds: Option<i64>,
    /// <p>The fleet name to display.</p>
    #[serde(rename = "DisplayName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    /// <p>The name of the directory and organizational unit (OU) to use to join the fleet to a Microsoft Active Directory domain. </p>
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
    pub fleet_type: Option<FleetType>,
    /// <p>The Amazon Resource Name (ARN) of the IAM role to apply to the fleet. To assume a role, a fleet instance calls the AWS Security Token Service (STS) <code>AssumeRole</code> API operation and passes the ARN of the role to use. The operation creates a new session with temporary credentials. AppStream 2.0 retrieves the temporary credentials and creates the <b>appstream_machine_role</b> credential profile on the instance.</p> <p>For more information, see <a href="https://docs.aws.amazon.com/appstream2/latest/developerguide/using-iam-roles-to-grant-permissions-to-applications-scripts-streaming-instances.html">Using an IAM Role to Grant Permissions to Applications and Scripts Running on AppStream 2.0 Streaming Instances</a> in the <i>Amazon AppStream 2.0 Administration Guide</i>.</p>
    #[serde(rename = "IamRoleArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iam_role_arn: Option<String>,
    /// <p><p>The amount of time that users can be idle (inactive) before they are disconnected from their streaming session and the <code>DisconnectTimeoutInSeconds</code> time interval begins. Users are notified before they are disconnected due to inactivity. If they try to reconnect to the streaming session before the time interval specified in <code>DisconnectTimeoutInSeconds</code> elapses, they are connected to their previous session. Users are considered idle when they stop providing keyboard or mouse input during their streaming session. File uploads and downloads, audio in, audio out, and pixels changing do not qualify as user activity. If users continue to be idle after the time interval in <code>IdleDisconnectTimeoutInSeconds</code> elapses, they are disconnected.</p> <p>To prevent users from being disconnected due to inactivity, specify a value of 0. Otherwise, specify a value between 60 and 3600. The default value is 0.</p> <note> <p>If you enable this feature, we recommend that you specify a value that corresponds exactly to a whole number of minutes (for example, 60, 120, and 180). If you don&#39;t do this, the value is rounded to the nearest minute. For example, if you specify a value of 70, users are disconnected after 1 minute of inactivity. If you specify a value that is at the midpoint between two different minutes, the value is rounded up. For example, if you specify a value of 90, users are disconnected after 2 minutes of inactivity. </p> </note></p>
    #[serde(rename = "IdleDisconnectTimeoutInSeconds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub idle_disconnect_timeout_in_seconds: Option<i64>,
    /// <p>The ARN of the public, private, or shared image to use.</p>
    #[serde(rename = "ImageArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_arn: Option<String>,
    /// <p>The name of the image used to create the fleet.</p>
    #[serde(rename = "ImageName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_name: Option<String>,
    /// <p><p>The instance type to use when launching fleet instances. The following instance types are available:</p> <ul> <li> <p>stream.standard.medium</p> </li> <li> <p>stream.standard.large</p> </li> <li> <p>stream.compute.large</p> </li> <li> <p>stream.compute.xlarge</p> </li> <li> <p>stream.compute.2xlarge</p> </li> <li> <p>stream.compute.4xlarge</p> </li> <li> <p>stream.compute.8xlarge</p> </li> <li> <p>stream.memory.large</p> </li> <li> <p>stream.memory.xlarge</p> </li> <li> <p>stream.memory.2xlarge</p> </li> <li> <p>stream.memory.4xlarge</p> </li> <li> <p>stream.memory.8xlarge</p> </li> <li> <p>stream.memory.z1d.large</p> </li> <li> <p>stream.memory.z1d.xlarge</p> </li> <li> <p>stream.memory.z1d.2xlarge</p> </li> <li> <p>stream.memory.z1d.3xlarge</p> </li> <li> <p>stream.memory.z1d.6xlarge</p> </li> <li> <p>stream.memory.z1d.12xlarge</p> </li> <li> <p>stream.graphics-design.large</p> </li> <li> <p>stream.graphics-design.xlarge</p> </li> <li> <p>stream.graphics-design.2xlarge</p> </li> <li> <p>stream.graphics-design.4xlarge</p> </li> <li> <p>stream.graphics-desktop.2xlarge</p> </li> <li> <p>stream.graphics.g4dn.xlarge</p> </li> <li> <p>stream.graphics.g4dn.2xlarge</p> </li> <li> <p>stream.graphics.g4dn.4xlarge</p> </li> <li> <p>stream.graphics.g4dn.8xlarge</p> </li> <li> <p>stream.graphics.g4dn.12xlarge</p> </li> <li> <p>stream.graphics.g4dn.16xlarge</p> </li> <li> <p>stream.graphics-pro.4xlarge</p> </li> <li> <p>stream.graphics-pro.8xlarge</p> </li> <li> <p>stream.graphics-pro.16xlarge</p> </li> </ul></p>
    #[serde(rename = "InstanceType")]
    pub instance_type: String,
    /// <p>The maximum amount of time that a streaming session can remain active, in seconds. If users are still connected to a streaming instance five minutes before this limit is reached, they are prompted to save any open documents before being disconnected. After this time elapses, the instance is terminated and replaced by a new instance.</p> <p>Specify a value between 600 and 360000.</p>
    #[serde(rename = "MaxUserDurationInSeconds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_user_duration_in_seconds: Option<i64>,
    /// <p>A unique name for the fleet.</p>
    #[serde(rename = "Name")]
    pub name: String,
    /// <p>The AppStream 2.0 view that is displayed to your users when they stream from the fleet. When <code>APP</code> is specified, only the windows of applications opened by users display. When <code>DESKTOP</code> is specified, the standard desktop that is provided by the operating system displays.</p> <p>The default value is <code>APP</code>.</p>
    #[serde(rename = "StreamView")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream_view: Option<StreamView>,
    /// <p>The tags to associate with the fleet. A tag is a key-value pair, and the value is optional. For example, Environment=Test. If you do not specify a value, Environment=. </p> <p>If you do not specify a value, the value is set to an empty string.</p> <p>Generally allowed characters are: letters, numbers, and spaces representable in UTF-8, and the following special characters: </p> <p>_ . : / = + \ - @</p> <p>For more information, see <a href="https://docs.aws.amazon.com/appstream2/latest/developerguide/tagging-basic.html">Tagging Your Resources</a> in the <i>Amazon AppStream 2.0 Administration Guide</i>.</p>
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<::std::collections::HashMap<String, String>>,
    /// <p>The VPC configuration for the fleet.</p>
    #[serde(rename = "VpcConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_config: Option<VpcConfig>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateFleetResult {
    /// <p>Information about the fleet.</p>
    #[serde(rename = "Fleet")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fleet: Option<Fleet>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateImageBuilderRequest {
    /// <p>The list of interface VPC endpoint (interface endpoint) objects. Administrators can connect to the image builder only through the specified endpoints.</p>
    #[serde(rename = "AccessEndpoints")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_endpoints: Option<Vec<AccessEndpoint>>,
    /// <p>The version of the AppStream 2.0 agent to use for this image builder. To use the latest version of the AppStream 2.0 agent, specify [LATEST]. </p>
    #[serde(rename = "AppstreamAgentVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub appstream_agent_version: Option<String>,
    /// <p>The description to display.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The image builder name to display.</p>
    #[serde(rename = "DisplayName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    /// <p>The name of the directory and organizational unit (OU) to use to join the image builder to a Microsoft Active Directory domain. </p>
    #[serde(rename = "DomainJoinInfo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_join_info: Option<DomainJoinInfo>,
    /// <p>Enables or disables default internet access for the image builder.</p>
    #[serde(rename = "EnableDefaultInternetAccess")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_default_internet_access: Option<bool>,
    /// <p>The Amazon Resource Name (ARN) of the IAM role to apply to the image builder. To assume a role, the image builder calls the AWS Security Token Service (STS) <code>AssumeRole</code> API operation and passes the ARN of the role to use. The operation creates a new session with temporary credentials. AppStream 2.0 retrieves the temporary credentials and creates the <b>appstream_machine_role</b> credential profile on the instance.</p> <p>For more information, see <a href="https://docs.aws.amazon.com/appstream2/latest/developerguide/using-iam-roles-to-grant-permissions-to-applications-scripts-streaming-instances.html">Using an IAM Role to Grant Permissions to Applications and Scripts Running on AppStream 2.0 Streaming Instances</a> in the <i>Amazon AppStream 2.0 Administration Guide</i>.</p>
    #[serde(rename = "IamRoleArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iam_role_arn: Option<String>,
    /// <p>The ARN of the public, private, or shared image to use.</p>
    #[serde(rename = "ImageArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_arn: Option<String>,
    /// <p>The name of the image used to create the image builder.</p>
    #[serde(rename = "ImageName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_name: Option<String>,
    /// <p><p>The instance type to use when launching the image builder. The following instance types are available:</p> <ul> <li> <p>stream.standard.medium</p> </li> <li> <p>stream.standard.large</p> </li> <li> <p>stream.compute.large</p> </li> <li> <p>stream.compute.xlarge</p> </li> <li> <p>stream.compute.2xlarge</p> </li> <li> <p>stream.compute.4xlarge</p> </li> <li> <p>stream.compute.8xlarge</p> </li> <li> <p>stream.memory.large</p> </li> <li> <p>stream.memory.xlarge</p> </li> <li> <p>stream.memory.2xlarge</p> </li> <li> <p>stream.memory.4xlarge</p> </li> <li> <p>stream.memory.8xlarge</p> </li> <li> <p>stream.memory.z1d.large</p> </li> <li> <p>stream.memory.z1d.xlarge</p> </li> <li> <p>stream.memory.z1d.2xlarge</p> </li> <li> <p>stream.memory.z1d.3xlarge</p> </li> <li> <p>stream.memory.z1d.6xlarge</p> </li> <li> <p>stream.memory.z1d.12xlarge</p> </li> <li> <p>stream.graphics-design.large</p> </li> <li> <p>stream.graphics-design.xlarge</p> </li> <li> <p>stream.graphics-design.2xlarge</p> </li> <li> <p>stream.graphics-design.4xlarge</p> </li> <li> <p>stream.graphics-desktop.2xlarge</p> </li> <li> <p>stream.graphics.g4dn.xlarge</p> </li> <li> <p>stream.graphics.g4dn.2xlarge</p> </li> <li> <p>stream.graphics.g4dn.4xlarge</p> </li> <li> <p>stream.graphics.g4dn.8xlarge</p> </li> <li> <p>stream.graphics.g4dn.12xlarge</p> </li> <li> <p>stream.graphics.g4dn.16xlarge</p> </li> <li> <p>stream.graphics-pro.4xlarge</p> </li> <li> <p>stream.graphics-pro.8xlarge</p> </li> <li> <p>stream.graphics-pro.16xlarge</p> </li> </ul></p>
    #[serde(rename = "InstanceType")]
    pub instance_type: String,
    /// <p>A unique name for the image builder.</p>
    #[serde(rename = "Name")]
    pub name: String,
    /// <p>The tags to associate with the image builder. A tag is a key-value pair, and the value is optional. For example, Environment=Test. If you do not specify a value, Environment=. </p> <p>Generally allowed characters are: letters, numbers, and spaces representable in UTF-8, and the following special characters: </p> <p>_ . : / = + \ - @</p> <p>If you do not specify a value, the value is set to an empty string.</p> <p>For more information about tags, see <a href="https://docs.aws.amazon.com/appstream2/latest/developerguide/tagging-basic.html">Tagging Your Resources</a> in the <i>Amazon AppStream 2.0 Administration Guide</i>.</p>
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<::std::collections::HashMap<String, String>>,
    /// <p>The VPC configuration for the image builder. You can specify only one subnet.</p>
    #[serde(rename = "VpcConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_config: Option<VpcConfig>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateImageBuilderResult {
    /// <p>Information about the image builder.</p>
    #[serde(rename = "ImageBuilder")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_builder: Option<ImageBuilder>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateImageBuilderStreamingURLRequest {
    /// <p>The name of the image builder.</p>
    #[serde(rename = "Name")]
    pub name: String,
    /// <p>The time that the streaming URL will be valid, in seconds. Specify a value between 1 and 604800 seconds. The default is 3600 seconds.</p>
    #[serde(rename = "Validity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub validity: Option<i64>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateStackRequest {
    /// <p>The list of interface VPC endpoint (interface endpoint) objects. Users of the stack can connect to AppStream 2.0 only through the specified endpoints.</p>
    #[serde(rename = "AccessEndpoints")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_endpoints: Option<Vec<AccessEndpoint>>,
    /// <p>The persistent application settings for users of a stack. When these settings are enabled, changes that users make to applications and Windows settings are automatically saved after each session and applied to the next session.</p>
    #[serde(rename = "ApplicationSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_settings: Option<ApplicationSettings>,
    /// <p>The description to display.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The stack name to display.</p>
    #[serde(rename = "DisplayName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    /// <p>The domains where AppStream 2.0 streaming sessions can be embedded in an iframe. You must approve the domains that you want to host embedded AppStream 2.0 streaming sessions. </p>
    #[serde(rename = "EmbedHostDomains")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub embed_host_domains: Option<Vec<String>>,
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
    /// <p>The tags to associate with the stack. A tag is a key-value pair, and the value is optional. For example, Environment=Test. If you do not specify a value, Environment=. </p> <p>If you do not specify a value, the value is set to an empty string.</p> <p>Generally allowed characters are: letters, numbers, and spaces representable in UTF-8, and the following special characters: </p> <p>_ . : / = + \ - @</p> <p>For more information about tags, see <a href="https://docs.aws.amazon.com/appstream2/latest/developerguide/tagging-basic.html">Tagging Your Resources</a> in the <i>Amazon AppStream 2.0 Administration Guide</i>.</p>
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<::std::collections::HashMap<String, String>>,
    /// <p>The actions that are enabled or disabled for users during their streaming sessions. By default, these actions are enabled. </p>
    #[serde(rename = "UserSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_settings: Option<Vec<UserSetting>>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateStackResult {
    /// <p>Information about the stack.</p>
    #[serde(rename = "Stack")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stack: Option<Stack>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateStreamingURLRequest {
    /// <p>The name of the application to launch after the session starts. This is the name that you specified as <b>Name</b> in the Image Assistant.</p>
    #[serde(rename = "ApplicationId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_id: Option<String>,
    /// <p>The name of the fleet.</p>
    #[serde(rename = "FleetName")]
    pub fleet_name: String,
    /// <p>The session context. For more information, see <a href="https://docs.aws.amazon.com/appstream2/latest/developerguide/managing-stacks-fleets.html#managing-stacks-fleets-parameters">Session Context</a> in the <i>Amazon AppStream 2.0 Administration Guide</i>.</p>
    #[serde(rename = "SessionContext")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub session_context: Option<String>,
    /// <p>The name of the stack.</p>
    #[serde(rename = "StackName")]
    pub stack_name: String,
    /// <p>The identifier of the user.</p>
    #[serde(rename = "UserId")]
    pub user_id: String,
    /// <p>The time that the streaming URL will be valid, in seconds. Specify a value between 1 and 604800 seconds. The default is 60 seconds.</p>
    #[serde(rename = "Validity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub validity: Option<i64>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateUsageReportSubscriptionRequest {}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateUsageReportSubscriptionResult {
    /// <p>The Amazon S3 bucket where generated reports are stored.</p> <p>If you enabled on-instance session scripts and Amazon S3 logging for your session script configuration, AppStream 2.0 created an S3 bucket to store the script output. The bucket is unique to your account and Region. When you enable usage reporting in this case, AppStream 2.0 uses the same bucket to store your usage reports. If you haven't already enabled on-instance session scripts, when you enable usage reports, AppStream 2.0 creates a new S3 bucket.</p>
    #[serde(rename = "S3BucketName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_bucket_name: Option<String>,
    /// <p>The schedule for generating usage reports.</p>
    #[serde(rename = "Schedule")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schedule: Option<UsageReportSchedule>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateUserRequest {
    /// <p>The authentication type for the user. You must specify USERPOOL. </p>
    #[serde(rename = "AuthenticationType")]
    pub authentication_type: AuthenticationType,
    /// <p>The first name, or given name, of the user.</p>
    #[serde(rename = "FirstName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_name: Option<String>,
    /// <p>The last name, or surname, of the user.</p>
    #[serde(rename = "LastName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_name: Option<String>,
    /// <p><p>The action to take for the welcome email that is sent to a user after the user is created in the user pool. If you specify SUPPRESS, no email is sent. If you specify RESEND, do not specify the first name or last name of the user. If the value is null, the email is sent. </p> <note> <p>The temporary password in the welcome email is valid for only 7 days. If users don’t set their passwords within 7 days, you must send them a new welcome email.</p> </note></p>
    #[serde(rename = "MessageAction")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_action: Option<MessageAction>,
    /// <p><p>The email address of the user.</p> <note> <p>Users&#39; email addresses are case-sensitive. During login, if they specify an email address that doesn&#39;t use the same capitalization as the email address specified when their user pool account was created, a &quot;user does not exist&quot; error message displays.</p> </note></p>
    #[serde(rename = "UserName")]
    pub user_name: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateUserResult {}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteDirectoryConfigRequest {
    /// <p>The name of the directory configuration.</p>
    #[serde(rename = "DirectoryName")]
    pub directory_name: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DeleteDirectoryConfigResult {}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteFleetRequest {
    /// <p>The name of the fleet.</p>
    #[serde(rename = "Name")]
    pub name: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DeleteFleetResult {}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteImageBuilderRequest {
    /// <p>The name of the image builder.</p>
    #[serde(rename = "Name")]
    pub name: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DeleteImageBuilderResult {
    /// <p>Information about the image builder.</p>
    #[serde(rename = "ImageBuilder")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_builder: Option<ImageBuilder>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteImagePermissionsRequest {
    /// <p>The name of the private image.</p>
    #[serde(rename = "Name")]
    pub name: String,
    /// <p>The 12-digit identifier of the AWS account for which to delete image permissions.</p>
    #[serde(rename = "SharedAccountId")]
    pub shared_account_id: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DeleteImagePermissionsResult {}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteImageRequest {
    /// <p>The name of the image.</p>
    #[serde(rename = "Name")]
    pub name: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DeleteImageResult {
    /// <p>Information about the image.</p>
    #[serde(rename = "Image")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image: Option<Image>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteStackRequest {
    /// <p>The name of the stack.</p>
    #[serde(rename = "Name")]
    pub name: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DeleteStackResult {}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteUsageReportSubscriptionRequest {}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DeleteUsageReportSubscriptionResult {}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteUserRequest {
    /// <p>The authentication type for the user. You must specify USERPOOL.</p>
    #[serde(rename = "AuthenticationType")]
    pub authentication_type: AuthenticationType,
    /// <p><p>The email address of the user.</p> <note> <p>Users&#39; email addresses are case-sensitive.</p> </note></p>
    #[serde(rename = "UserName")]
    pub user_name: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DeleteUserResult {}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeImagePermissionsRequest {
    /// <p>The maximum size of each page of results.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>The name of the private image for which to describe permissions. The image must be one that you own. </p>
    #[serde(rename = "Name")]
    pub name: String,
    /// <p>The pagination token to use to retrieve the next page of results for this operation. If this value is null, it retrieves the first page.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The 12-digit identifier of one or more AWS accounts with which the image is shared.</p>
    #[serde(rename = "SharedAwsAccountIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shared_aws_account_ids: Option<Vec<String>>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeImagePermissionsResult {
    /// <p>The name of the private image.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The pagination token to use to retrieve the next page of results for this operation. If there are no more pages, this value is null.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The permissions for a private image that you own. </p>
    #[serde(rename = "SharedImagePermissionsList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shared_image_permissions_list: Option<Vec<SharedImagePermissions>>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeImagesRequest {
    /// <p>The ARNs of the public, private, and shared images to describe.</p>
    #[serde(rename = "Arns")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arns: Option<Vec<String>>,
    /// <p>The maximum size of each page of results.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>The names of the public or private images to describe.</p>
    #[serde(rename = "Names")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub names: Option<Vec<String>>,
    /// <p>The pagination token to use to retrieve the next page of results for this operation. If this value is null, it retrieves the first page.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The type of image (public, private, or shared) to describe. </p>
    #[serde(rename = "Type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<VisibilityType>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeImagesResult {
    /// <p>Information about the images.</p>
    #[serde(rename = "Images")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub images: Option<Vec<Image>>,
    /// <p>The pagination token to use to retrieve the next page of results for this operation. If there are no more pages, this value is null.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeSessionsRequest {
    /// <p>The authentication method. Specify <code>API</code> for a user authenticated using a streaming URL or <code>SAML</code> for a SAML federated user. The default is to authenticate users using a streaming URL.</p>
    #[serde(rename = "AuthenticationType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authentication_type: Option<AuthenticationType>,
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
    /// <p>The user identifier (ID). If you specify a user ID, you must also specify the authentication type.</p>
    #[serde(rename = "UserId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeUsageReportSubscriptionsRequest {
    /// <p>The maximum size of each page of results.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>The pagination token to use to retrieve the next page of results for this operation. If this value is null, it retrieves the first page.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeUsageReportSubscriptionsResult {
    /// <p>The pagination token to use to retrieve the next page of results for this operation. If there are no more pages, this value is null.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>Information about the usage report subscription.</p>
    #[serde(rename = "UsageReportSubscriptions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub usage_report_subscriptions: Option<Vec<UsageReportSubscription>>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeUserStackAssociationsRequest {
    /// <p>The authentication type for the user who is associated with the stack. You must specify USERPOOL.</p>
    #[serde(rename = "AuthenticationType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authentication_type: Option<AuthenticationType>,
    /// <p>The maximum size of each page of results.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>The pagination token to use to retrieve the next page of results for this operation. If this value is null, it retrieves the first page.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The name of the stack that is associated with the user.</p>
    #[serde(rename = "StackName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stack_name: Option<String>,
    /// <p><p>The email address of the user who is associated with the stack.</p> <note> <p>Users&#39; email addresses are case-sensitive.</p> </note></p>
    #[serde(rename = "UserName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_name: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeUserStackAssociationsResult {
    /// <p>The pagination token to use to retrieve the next page of results for this operation. If there are no more pages, this value is null.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The UserStackAssociation objects.</p>
    #[serde(rename = "UserStackAssociations")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_stack_associations: Option<Vec<UserStackAssociation>>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeUsersRequest {
    /// <p>The authentication type for the users in the user pool to describe. You must specify USERPOOL.</p>
    #[serde(rename = "AuthenticationType")]
    pub authentication_type: AuthenticationType,
    /// <p>The maximum size of each page of results.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>The pagination token to use to retrieve the next page of results for this operation. If this value is null, it retrieves the first page.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeUsersResult {
    /// <p>The pagination token to use to retrieve the next page of results for this operation. If there are no more pages, this value is null.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>Information about users in the user pool.</p>
    #[serde(rename = "Users")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub users: Option<Vec<User>>,
}

/// <p>Describes the configuration information required to join fleets and image builders to Microsoft Active Directory domains.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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
    /// <p>The credentials for the service account used by the fleet or image builder to connect to the directory.</p>
    #[serde(rename = "ServiceAccountCredentials")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_account_credentials: Option<ServiceAccountCredentials>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DisableUserRequest {
    /// <p>The authentication type for the user. You must specify USERPOOL.</p>
    #[serde(rename = "AuthenticationType")]
    pub authentication_type: AuthenticationType,
    /// <p><p>The email address of the user.</p> <note> <p>Users&#39; email addresses are case-sensitive.</p> </note></p>
    #[serde(rename = "UserName")]
    pub user_name: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DisableUserResult {}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DisassociateFleetRequest {
    /// <p>The name of the fleet.</p>
    #[serde(rename = "FleetName")]
    pub fleet_name: String,
    /// <p>The name of the stack.</p>
    #[serde(rename = "StackName")]
    pub stack_name: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DisassociateFleetResult {}

/// <p>Describes the configuration information required to join fleets and image builders to Microsoft Active Directory domains.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
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

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct EnableUserRequest {
    /// <p>The authentication type for the user. You must specify USERPOOL.</p>
    #[serde(rename = "AuthenticationType")]
    pub authentication_type: AuthenticationType,
    /// <p><p>The email address of the user.</p> <note> <p>Users&#39; email addresses are case-sensitive. During login, if they specify an email address that doesn&#39;t use the same capitalization as the email address specified when their user pool account was created, a &quot;user does not exist&quot; error message displays. </p> </note></p>
    #[serde(rename = "UserName")]
    pub user_name: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct EnableUserResult {}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ExpireSessionRequest {
    /// <p>The identifier of the streaming session.</p>
    #[serde(rename = "SessionId")]
    pub session_id: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ExpireSessionResult {}

/// <p>Describes a fleet.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct Fleet {
    /// <p>The Amazon Resource Name (ARN) for the fleet.</p>
    #[serde(rename = "Arn")]
    pub arn: String,
    /// <p>The capacity status for the fleet.</p>
    #[serde(rename = "ComputeCapacityStatus")]
    pub compute_capacity_status: ComputeCapacityStatus,
    /// <p>The time the fleet was created.</p>
    #[serde(rename = "CreatedTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_time: Option<f64>,
    /// <p>The description to display.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The amount of time that a streaming session remains active after users disconnect. If they try to reconnect to the streaming session after a disconnection or network interruption within this time interval, they are connected to their previous session. Otherwise, they are connected to a new session with a new streaming instance.</p> <p>Specify a value between 60 and 360000.</p>
    #[serde(rename = "DisconnectTimeoutInSeconds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disconnect_timeout_in_seconds: Option<i64>,
    /// <p>The fleet name to display.</p>
    #[serde(rename = "DisplayName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    /// <p>The name of the directory and organizational unit (OU) to use to join the fleet to a Microsoft Active Directory domain. </p>
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
    pub fleet_type: Option<FleetType>,
    /// <p>The ARN of the IAM role that is applied to the fleet. To assume a role, the fleet instance calls the AWS Security Token Service (STS) <code>AssumeRole</code> API operation and passes the ARN of the role to use. The operation creates a new session with temporary credentials. AppStream 2.0 retrieves the temporary credentials and creates the <b>appstream_machine_role</b> credential profile on the instance.</p> <p>For more information, see <a href="https://docs.aws.amazon.com/appstream2/latest/developerguide/using-iam-roles-to-grant-permissions-to-applications-scripts-streaming-instances.html">Using an IAM Role to Grant Permissions to Applications and Scripts Running on AppStream 2.0 Streaming Instances</a> in the <i>Amazon AppStream 2.0 Administration Guide</i>.</p>
    #[serde(rename = "IamRoleArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iam_role_arn: Option<String>,
    /// <p><p>The amount of time that users can be idle (inactive) before they are disconnected from their streaming session and the <code>DisconnectTimeoutInSeconds</code> time interval begins. Users are notified before they are disconnected due to inactivity. If users try to reconnect to the streaming session before the time interval specified in <code>DisconnectTimeoutInSeconds</code> elapses, they are connected to their previous session. Users are considered idle when they stop providing keyboard or mouse input during their streaming session. File uploads and downloads, audio in, audio out, and pixels changing do not qualify as user activity. If users continue to be idle after the time interval in <code>IdleDisconnectTimeoutInSeconds</code> elapses, they are disconnected.</p> <p>To prevent users from being disconnected due to inactivity, specify a value of 0. Otherwise, specify a value between 60 and 3600. The default value is 0.</p> <note> <p>If you enable this feature, we recommend that you specify a value that corresponds exactly to a whole number of minutes (for example, 60, 120, and 180). If you don&#39;t do this, the value is rounded to the nearest minute. For example, if you specify a value of 70, users are disconnected after 1 minute of inactivity. If you specify a value that is at the midpoint between two different minutes, the value is rounded up. For example, if you specify a value of 90, users are disconnected after 2 minutes of inactivity. </p> </note></p>
    #[serde(rename = "IdleDisconnectTimeoutInSeconds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub idle_disconnect_timeout_in_seconds: Option<i64>,
    /// <p>The ARN for the public, private, or shared image.</p>
    #[serde(rename = "ImageArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_arn: Option<String>,
    /// <p>The name of the image used to create the fleet.</p>
    #[serde(rename = "ImageName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_name: Option<String>,
    /// <p><p>The instance type to use when launching fleet instances. The following instance types are available:</p> <ul> <li> <p>stream.standard.medium</p> </li> <li> <p>stream.standard.large</p> </li> <li> <p>stream.compute.large</p> </li> <li> <p>stream.compute.xlarge</p> </li> <li> <p>stream.compute.2xlarge</p> </li> <li> <p>stream.compute.4xlarge</p> </li> <li> <p>stream.compute.8xlarge</p> </li> <li> <p>stream.memory.large</p> </li> <li> <p>stream.memory.xlarge</p> </li> <li> <p>stream.memory.2xlarge</p> </li> <li> <p>stream.memory.4xlarge</p> </li> <li> <p>stream.memory.8xlarge</p> </li> <li> <p>stream.memory.z1d.large</p> </li> <li> <p>stream.memory.z1d.xlarge</p> </li> <li> <p>stream.memory.z1d.2xlarge</p> </li> <li> <p>stream.memory.z1d.3xlarge</p> </li> <li> <p>stream.memory.z1d.6xlarge</p> </li> <li> <p>stream.memory.z1d.12xlarge</p> </li> <li> <p>stream.graphics-design.large</p> </li> <li> <p>stream.graphics-design.xlarge</p> </li> <li> <p>stream.graphics-design.2xlarge</p> </li> <li> <p>stream.graphics-design.4xlarge</p> </li> <li> <p>stream.graphics-desktop.2xlarge</p> </li> <li> <p>stream.graphics.g4dn.xlarge</p> </li> <li> <p>stream.graphics.g4dn.2xlarge</p> </li> <li> <p>stream.graphics.g4dn.4xlarge</p> </li> <li> <p>stream.graphics.g4dn.8xlarge</p> </li> <li> <p>stream.graphics.g4dn.12xlarge</p> </li> <li> <p>stream.graphics.g4dn.16xlarge</p> </li> <li> <p>stream.graphics-pro.4xlarge</p> </li> <li> <p>stream.graphics-pro.8xlarge</p> </li> <li> <p>stream.graphics-pro.16xlarge</p> </li> </ul></p>
    #[serde(rename = "InstanceType")]
    pub instance_type: String,
    /// <p>The maximum amount of time that a streaming session can remain active, in seconds. If users are still connected to a streaming instance five minutes before this limit is reached, they are prompted to save any open documents before being disconnected. After this time elapses, the instance is terminated and replaced by a new instance. </p> <p>Specify a value between 600 and 360000.</p>
    #[serde(rename = "MaxUserDurationInSeconds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_user_duration_in_seconds: Option<i64>,
    /// <p>The name of the fleet.</p>
    #[serde(rename = "Name")]
    pub name: String,
    /// <p>The current state for the fleet.</p>
    #[serde(rename = "State")]
    pub state: FleetState,
    /// <p>The AppStream 2.0 view that is displayed to your users when they stream from the fleet. When <code>APP</code> is specified, only the windows of applications opened by users display. When <code>DESKTOP</code> is specified, the standard desktop that is provided by the operating system displays.</p> <p>The default value is <code>APP</code>.</p>
    #[serde(rename = "StreamView")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream_view: Option<StreamView>,
    /// <p>The VPC configuration for the fleet.</p>
    #[serde(rename = "VpcConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_config: Option<VpcConfig>,
}

/// <p>The fleet attribute.</p>

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct UnknownFleetAttribute {
    name: String,
}

#[allow(non_camel_case_types)]
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
#[non_exhaustive]
pub enum FleetAttribute {
    DomainJoinInfo,
    IamRoleArn,
    VpcConfiguration,
    VpcConfigurationSecurityGroupIds,
    #[doc(hidden)]
    UnknownVariant(UnknownFleetAttribute),
}

impl Default for FleetAttribute {
    fn default() -> Self {
        "".into()
    }
}

impl fmt::Display for FleetAttribute {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.into())
    }
}

impl rusoto_core::param::ToParam for FleetAttribute {
    fn to_param(&self) -> String {
        self.to_string()
    }
}

impl Into<String> for FleetAttribute {
    fn into(self) -> String {
        match self {
            FleetAttribute::DomainJoinInfo => "DOMAIN_JOIN_INFO".to_string(),
            FleetAttribute::IamRoleArn => "IAM_ROLE_ARN".to_string(),
            FleetAttribute::VpcConfiguration => "VPC_CONFIGURATION".to_string(),
            FleetAttribute::VpcConfigurationSecurityGroupIds => {
                "VPC_CONFIGURATION_SECURITY_GROUP_IDS".to_string()
            }
            FleetAttribute::UnknownVariant(UnknownFleetAttribute { name: original }) => original,
        }
    }
}

impl<'a> Into<&'a str> for &'a FleetAttribute {
    fn into(self) -> &'a str {
        match self {
            FleetAttribute::DomainJoinInfo => &"DOMAIN_JOIN_INFO",
            FleetAttribute::IamRoleArn => &"IAM_ROLE_ARN",
            FleetAttribute::VpcConfiguration => &"VPC_CONFIGURATION",
            FleetAttribute::VpcConfigurationSecurityGroupIds => {
                &"VPC_CONFIGURATION_SECURITY_GROUP_IDS"
            }
            FleetAttribute::UnknownVariant(UnknownFleetAttribute { name: original }) => original,
        }
    }
}

impl From<&str> for FleetAttribute {
    fn from(name: &str) -> Self {
        match name {
            "DOMAIN_JOIN_INFO" => FleetAttribute::DomainJoinInfo,
            "IAM_ROLE_ARN" => FleetAttribute::IamRoleArn,
            "VPC_CONFIGURATION" => FleetAttribute::VpcConfiguration,
            "VPC_CONFIGURATION_SECURITY_GROUP_IDS" => {
                FleetAttribute::VpcConfigurationSecurityGroupIds
            }
            _ => FleetAttribute::UnknownVariant(UnknownFleetAttribute {
                name: name.to_owned(),
            }),
        }
    }
}

impl From<String> for FleetAttribute {
    fn from(name: String) -> Self {
        match &*name {
            "DOMAIN_JOIN_INFO" => FleetAttribute::DomainJoinInfo,
            "IAM_ROLE_ARN" => FleetAttribute::IamRoleArn,
            "VPC_CONFIGURATION" => FleetAttribute::VpcConfiguration,
            "VPC_CONFIGURATION_SECURITY_GROUP_IDS" => {
                FleetAttribute::VpcConfigurationSecurityGroupIds
            }
            _ => FleetAttribute::UnknownVariant(UnknownFleetAttribute { name }),
        }
    }
}

impl ::std::str::FromStr for FleetAttribute {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(s.into())
    }
}

impl Serialize for FleetAttribute {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.into())
    }
}

#[cfg(feature = "deserialize_structs")]
impl<'de> Deserialize<'de> for FleetAttribute {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Ok(String::deserialize(deserializer)?.into())
    }
}

/// <p>Describes a fleet error.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct FleetError {
    /// <p>The error code.</p>
    #[serde(rename = "ErrorCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_code: Option<FleetErrorCode>,
    /// <p>The error message.</p>
    #[serde(rename = "ErrorMessage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
}

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct UnknownFleetErrorCode {
    name: String,
}

#[allow(non_camel_case_types)]
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
#[non_exhaustive]
pub enum FleetErrorCode {
    DomainJoinErrorAccessDenied,
    DomainJoinErrorDsMachineAccountQuotaExceeded,
    DomainJoinErrorFileNotFound,
    DomainJoinErrorInvalidParameter,
    DomainJoinErrorLogonFailure,
    DomainJoinErrorMoreData,
    DomainJoinErrorNotSupported,
    DomainJoinErrorNoSuchDomain,
    DomainJoinInternalServiceError,
    DomainJoinNerrInvalidWorkgroupName,
    DomainJoinNerrPasswordExpired,
    DomainJoinNerrWorkstationNotStarted,
    IamServiceRoleIsMissing,
    IamServiceRoleMissingDescribeSecurityGroupsAction,
    IamServiceRoleMissingDescribeSubnetAction,
    IamServiceRoleMissingEniCreateAction,
    IamServiceRoleMissingEniDeleteAction,
    IamServiceRoleMissingEniDescribeAction,
    IgwNotAttached,
    ImageNotFound,
    InternalServiceError,
    InvalidSubnetConfiguration,
    MachineRoleIsMissing,
    NetworkInterfaceLimitExceeded,
    SecurityGroupsNotFound,
    StsDisabledInRegion,
    SubnetHasInsufficientIpAddresses,
    SubnetNotFound,
    #[doc(hidden)]
    UnknownVariant(UnknownFleetErrorCode),
}

impl Default for FleetErrorCode {
    fn default() -> Self {
        "".into()
    }
}

impl fmt::Display for FleetErrorCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.into())
    }
}

impl rusoto_core::param::ToParam for FleetErrorCode {
    fn to_param(&self) -> String {
        self.to_string()
    }
}

impl Into<String> for FleetErrorCode {
    fn into(self) -> String {
        match self {
            FleetErrorCode::DomainJoinErrorAccessDenied => {
                "DOMAIN_JOIN_ERROR_ACCESS_DENIED".to_string()
            }
            FleetErrorCode::DomainJoinErrorDsMachineAccountQuotaExceeded => {
                "DOMAIN_JOIN_ERROR_DS_MACHINE_ACCOUNT_QUOTA_EXCEEDED".to_string()
            }
            FleetErrorCode::DomainJoinErrorFileNotFound => {
                "DOMAIN_JOIN_ERROR_FILE_NOT_FOUND".to_string()
            }
            FleetErrorCode::DomainJoinErrorInvalidParameter => {
                "DOMAIN_JOIN_ERROR_INVALID_PARAMETER".to_string()
            }
            FleetErrorCode::DomainJoinErrorLogonFailure => {
                "DOMAIN_JOIN_ERROR_LOGON_FAILURE".to_string()
            }
            FleetErrorCode::DomainJoinErrorMoreData => "DOMAIN_JOIN_ERROR_MORE_DATA".to_string(),
            FleetErrorCode::DomainJoinErrorNotSupported => {
                "DOMAIN_JOIN_ERROR_NOT_SUPPORTED".to_string()
            }
            FleetErrorCode::DomainJoinErrorNoSuchDomain => {
                "DOMAIN_JOIN_ERROR_NO_SUCH_DOMAIN".to_string()
            }
            FleetErrorCode::DomainJoinInternalServiceError => {
                "DOMAIN_JOIN_INTERNAL_SERVICE_ERROR".to_string()
            }
            FleetErrorCode::DomainJoinNerrInvalidWorkgroupName => {
                "DOMAIN_JOIN_NERR_INVALID_WORKGROUP_NAME".to_string()
            }
            FleetErrorCode::DomainJoinNerrPasswordExpired => {
                "DOMAIN_JOIN_NERR_PASSWORD_EXPIRED".to_string()
            }
            FleetErrorCode::DomainJoinNerrWorkstationNotStarted => {
                "DOMAIN_JOIN_NERR_WORKSTATION_NOT_STARTED".to_string()
            }
            FleetErrorCode::IamServiceRoleIsMissing => "IAM_SERVICE_ROLE_IS_MISSING".to_string(),
            FleetErrorCode::IamServiceRoleMissingDescribeSecurityGroupsAction => {
                "IAM_SERVICE_ROLE_MISSING_DESCRIBE_SECURITY_GROUPS_ACTION".to_string()
            }
            FleetErrorCode::IamServiceRoleMissingDescribeSubnetAction => {
                "IAM_SERVICE_ROLE_MISSING_DESCRIBE_SUBNET_ACTION".to_string()
            }
            FleetErrorCode::IamServiceRoleMissingEniCreateAction => {
                "IAM_SERVICE_ROLE_MISSING_ENI_CREATE_ACTION".to_string()
            }
            FleetErrorCode::IamServiceRoleMissingEniDeleteAction => {
                "IAM_SERVICE_ROLE_MISSING_ENI_DELETE_ACTION".to_string()
            }
            FleetErrorCode::IamServiceRoleMissingEniDescribeAction => {
                "IAM_SERVICE_ROLE_MISSING_ENI_DESCRIBE_ACTION".to_string()
            }
            FleetErrorCode::IgwNotAttached => "IGW_NOT_ATTACHED".to_string(),
            FleetErrorCode::ImageNotFound => "IMAGE_NOT_FOUND".to_string(),
            FleetErrorCode::InternalServiceError => "INTERNAL_SERVICE_ERROR".to_string(),
            FleetErrorCode::InvalidSubnetConfiguration => {
                "INVALID_SUBNET_CONFIGURATION".to_string()
            }
            FleetErrorCode::MachineRoleIsMissing => "MACHINE_ROLE_IS_MISSING".to_string(),
            FleetErrorCode::NetworkInterfaceLimitExceeded => {
                "NETWORK_INTERFACE_LIMIT_EXCEEDED".to_string()
            }
            FleetErrorCode::SecurityGroupsNotFound => "SECURITY_GROUPS_NOT_FOUND".to_string(),
            FleetErrorCode::StsDisabledInRegion => "STS_DISABLED_IN_REGION".to_string(),
            FleetErrorCode::SubnetHasInsufficientIpAddresses => {
                "SUBNET_HAS_INSUFFICIENT_IP_ADDRESSES".to_string()
            }
            FleetErrorCode::SubnetNotFound => "SUBNET_NOT_FOUND".to_string(),
            FleetErrorCode::UnknownVariant(UnknownFleetErrorCode { name: original }) => original,
        }
    }
}

impl<'a> Into<&'a str> for &'a FleetErrorCode {
    fn into(self) -> &'a str {
        match self {
            FleetErrorCode::DomainJoinErrorAccessDenied => &"DOMAIN_JOIN_ERROR_ACCESS_DENIED",
            FleetErrorCode::DomainJoinErrorDsMachineAccountQuotaExceeded => {
                &"DOMAIN_JOIN_ERROR_DS_MACHINE_ACCOUNT_QUOTA_EXCEEDED"
            }
            FleetErrorCode::DomainJoinErrorFileNotFound => &"DOMAIN_JOIN_ERROR_FILE_NOT_FOUND",
            FleetErrorCode::DomainJoinErrorInvalidParameter => {
                &"DOMAIN_JOIN_ERROR_INVALID_PARAMETER"
            }
            FleetErrorCode::DomainJoinErrorLogonFailure => &"DOMAIN_JOIN_ERROR_LOGON_FAILURE",
            FleetErrorCode::DomainJoinErrorMoreData => &"DOMAIN_JOIN_ERROR_MORE_DATA",
            FleetErrorCode::DomainJoinErrorNotSupported => &"DOMAIN_JOIN_ERROR_NOT_SUPPORTED",
            FleetErrorCode::DomainJoinErrorNoSuchDomain => &"DOMAIN_JOIN_ERROR_NO_SUCH_DOMAIN",
            FleetErrorCode::DomainJoinInternalServiceError => &"DOMAIN_JOIN_INTERNAL_SERVICE_ERROR",
            FleetErrorCode::DomainJoinNerrInvalidWorkgroupName => {
                &"DOMAIN_JOIN_NERR_INVALID_WORKGROUP_NAME"
            }
            FleetErrorCode::DomainJoinNerrPasswordExpired => &"DOMAIN_JOIN_NERR_PASSWORD_EXPIRED",
            FleetErrorCode::DomainJoinNerrWorkstationNotStarted => {
                &"DOMAIN_JOIN_NERR_WORKSTATION_NOT_STARTED"
            }
            FleetErrorCode::IamServiceRoleIsMissing => &"IAM_SERVICE_ROLE_IS_MISSING",
            FleetErrorCode::IamServiceRoleMissingDescribeSecurityGroupsAction => {
                &"IAM_SERVICE_ROLE_MISSING_DESCRIBE_SECURITY_GROUPS_ACTION"
            }
            FleetErrorCode::IamServiceRoleMissingDescribeSubnetAction => {
                &"IAM_SERVICE_ROLE_MISSING_DESCRIBE_SUBNET_ACTION"
            }
            FleetErrorCode::IamServiceRoleMissingEniCreateAction => {
                &"IAM_SERVICE_ROLE_MISSING_ENI_CREATE_ACTION"
            }
            FleetErrorCode::IamServiceRoleMissingEniDeleteAction => {
                &"IAM_SERVICE_ROLE_MISSING_ENI_DELETE_ACTION"
            }
            FleetErrorCode::IamServiceRoleMissingEniDescribeAction => {
                &"IAM_SERVICE_ROLE_MISSING_ENI_DESCRIBE_ACTION"
            }
            FleetErrorCode::IgwNotAttached => &"IGW_NOT_ATTACHED",
            FleetErrorCode::ImageNotFound => &"IMAGE_NOT_FOUND",
            FleetErrorCode::InternalServiceError => &"INTERNAL_SERVICE_ERROR",
            FleetErrorCode::InvalidSubnetConfiguration => &"INVALID_SUBNET_CONFIGURATION",
            FleetErrorCode::MachineRoleIsMissing => &"MACHINE_ROLE_IS_MISSING",
            FleetErrorCode::NetworkInterfaceLimitExceeded => &"NETWORK_INTERFACE_LIMIT_EXCEEDED",
            FleetErrorCode::SecurityGroupsNotFound => &"SECURITY_GROUPS_NOT_FOUND",
            FleetErrorCode::StsDisabledInRegion => &"STS_DISABLED_IN_REGION",
            FleetErrorCode::SubnetHasInsufficientIpAddresses => {
                &"SUBNET_HAS_INSUFFICIENT_IP_ADDRESSES"
            }
            FleetErrorCode::SubnetNotFound => &"SUBNET_NOT_FOUND",
            FleetErrorCode::UnknownVariant(UnknownFleetErrorCode { name: original }) => original,
        }
    }
}

impl From<&str> for FleetErrorCode {
    fn from(name: &str) -> Self {
        match name {
            "DOMAIN_JOIN_ERROR_ACCESS_DENIED" => FleetErrorCode::DomainJoinErrorAccessDenied,
            "DOMAIN_JOIN_ERROR_DS_MACHINE_ACCOUNT_QUOTA_EXCEEDED" => {
                FleetErrorCode::DomainJoinErrorDsMachineAccountQuotaExceeded
            }
            "DOMAIN_JOIN_ERROR_FILE_NOT_FOUND" => FleetErrorCode::DomainJoinErrorFileNotFound,
            "DOMAIN_JOIN_ERROR_INVALID_PARAMETER" => {
                FleetErrorCode::DomainJoinErrorInvalidParameter
            }
            "DOMAIN_JOIN_ERROR_LOGON_FAILURE" => FleetErrorCode::DomainJoinErrorLogonFailure,
            "DOMAIN_JOIN_ERROR_MORE_DATA" => FleetErrorCode::DomainJoinErrorMoreData,
            "DOMAIN_JOIN_ERROR_NOT_SUPPORTED" => FleetErrorCode::DomainJoinErrorNotSupported,
            "DOMAIN_JOIN_ERROR_NO_SUCH_DOMAIN" => FleetErrorCode::DomainJoinErrorNoSuchDomain,
            "DOMAIN_JOIN_INTERNAL_SERVICE_ERROR" => FleetErrorCode::DomainJoinInternalServiceError,
            "DOMAIN_JOIN_NERR_INVALID_WORKGROUP_NAME" => {
                FleetErrorCode::DomainJoinNerrInvalidWorkgroupName
            }
            "DOMAIN_JOIN_NERR_PASSWORD_EXPIRED" => FleetErrorCode::DomainJoinNerrPasswordExpired,
            "DOMAIN_JOIN_NERR_WORKSTATION_NOT_STARTED" => {
                FleetErrorCode::DomainJoinNerrWorkstationNotStarted
            }
            "IAM_SERVICE_ROLE_IS_MISSING" => FleetErrorCode::IamServiceRoleIsMissing,
            "IAM_SERVICE_ROLE_MISSING_DESCRIBE_SECURITY_GROUPS_ACTION" => {
                FleetErrorCode::IamServiceRoleMissingDescribeSecurityGroupsAction
            }
            "IAM_SERVICE_ROLE_MISSING_DESCRIBE_SUBNET_ACTION" => {
                FleetErrorCode::IamServiceRoleMissingDescribeSubnetAction
            }
            "IAM_SERVICE_ROLE_MISSING_ENI_CREATE_ACTION" => {
                FleetErrorCode::IamServiceRoleMissingEniCreateAction
            }
            "IAM_SERVICE_ROLE_MISSING_ENI_DELETE_ACTION" => {
                FleetErrorCode::IamServiceRoleMissingEniDeleteAction
            }
            "IAM_SERVICE_ROLE_MISSING_ENI_DESCRIBE_ACTION" => {
                FleetErrorCode::IamServiceRoleMissingEniDescribeAction
            }
            "IGW_NOT_ATTACHED" => FleetErrorCode::IgwNotAttached,
            "IMAGE_NOT_FOUND" => FleetErrorCode::ImageNotFound,
            "INTERNAL_SERVICE_ERROR" => FleetErrorCode::InternalServiceError,
            "INVALID_SUBNET_CONFIGURATION" => FleetErrorCode::InvalidSubnetConfiguration,
            "MACHINE_ROLE_IS_MISSING" => FleetErrorCode::MachineRoleIsMissing,
            "NETWORK_INTERFACE_LIMIT_EXCEEDED" => FleetErrorCode::NetworkInterfaceLimitExceeded,
            "SECURITY_GROUPS_NOT_FOUND" => FleetErrorCode::SecurityGroupsNotFound,
            "STS_DISABLED_IN_REGION" => FleetErrorCode::StsDisabledInRegion,
            "SUBNET_HAS_INSUFFICIENT_IP_ADDRESSES" => {
                FleetErrorCode::SubnetHasInsufficientIpAddresses
            }
            "SUBNET_NOT_FOUND" => FleetErrorCode::SubnetNotFound,
            _ => FleetErrorCode::UnknownVariant(UnknownFleetErrorCode {
                name: name.to_owned(),
            }),
        }
    }
}

impl From<String> for FleetErrorCode {
    fn from(name: String) -> Self {
        match &*name {
            "DOMAIN_JOIN_ERROR_ACCESS_DENIED" => FleetErrorCode::DomainJoinErrorAccessDenied,
            "DOMAIN_JOIN_ERROR_DS_MACHINE_ACCOUNT_QUOTA_EXCEEDED" => {
                FleetErrorCode::DomainJoinErrorDsMachineAccountQuotaExceeded
            }
            "DOMAIN_JOIN_ERROR_FILE_NOT_FOUND" => FleetErrorCode::DomainJoinErrorFileNotFound,
            "DOMAIN_JOIN_ERROR_INVALID_PARAMETER" => {
                FleetErrorCode::DomainJoinErrorInvalidParameter
            }
            "DOMAIN_JOIN_ERROR_LOGON_FAILURE" => FleetErrorCode::DomainJoinErrorLogonFailure,
            "DOMAIN_JOIN_ERROR_MORE_DATA" => FleetErrorCode::DomainJoinErrorMoreData,
            "DOMAIN_JOIN_ERROR_NOT_SUPPORTED" => FleetErrorCode::DomainJoinErrorNotSupported,
            "DOMAIN_JOIN_ERROR_NO_SUCH_DOMAIN" => FleetErrorCode::DomainJoinErrorNoSuchDomain,
            "DOMAIN_JOIN_INTERNAL_SERVICE_ERROR" => FleetErrorCode::DomainJoinInternalServiceError,
            "DOMAIN_JOIN_NERR_INVALID_WORKGROUP_NAME" => {
                FleetErrorCode::DomainJoinNerrInvalidWorkgroupName
            }
            "DOMAIN_JOIN_NERR_PASSWORD_EXPIRED" => FleetErrorCode::DomainJoinNerrPasswordExpired,
            "DOMAIN_JOIN_NERR_WORKSTATION_NOT_STARTED" => {
                FleetErrorCode::DomainJoinNerrWorkstationNotStarted
            }
            "IAM_SERVICE_ROLE_IS_MISSING" => FleetErrorCode::IamServiceRoleIsMissing,
            "IAM_SERVICE_ROLE_MISSING_DESCRIBE_SECURITY_GROUPS_ACTION" => {
                FleetErrorCode::IamServiceRoleMissingDescribeSecurityGroupsAction
            }
            "IAM_SERVICE_ROLE_MISSING_DESCRIBE_SUBNET_ACTION" => {
                FleetErrorCode::IamServiceRoleMissingDescribeSubnetAction
            }
            "IAM_SERVICE_ROLE_MISSING_ENI_CREATE_ACTION" => {
                FleetErrorCode::IamServiceRoleMissingEniCreateAction
            }
            "IAM_SERVICE_ROLE_MISSING_ENI_DELETE_ACTION" => {
                FleetErrorCode::IamServiceRoleMissingEniDeleteAction
            }
            "IAM_SERVICE_ROLE_MISSING_ENI_DESCRIBE_ACTION" => {
                FleetErrorCode::IamServiceRoleMissingEniDescribeAction
            }
            "IGW_NOT_ATTACHED" => FleetErrorCode::IgwNotAttached,
            "IMAGE_NOT_FOUND" => FleetErrorCode::ImageNotFound,
            "INTERNAL_SERVICE_ERROR" => FleetErrorCode::InternalServiceError,
            "INVALID_SUBNET_CONFIGURATION" => FleetErrorCode::InvalidSubnetConfiguration,
            "MACHINE_ROLE_IS_MISSING" => FleetErrorCode::MachineRoleIsMissing,
            "NETWORK_INTERFACE_LIMIT_EXCEEDED" => FleetErrorCode::NetworkInterfaceLimitExceeded,
            "SECURITY_GROUPS_NOT_FOUND" => FleetErrorCode::SecurityGroupsNotFound,
            "STS_DISABLED_IN_REGION" => FleetErrorCode::StsDisabledInRegion,
            "SUBNET_HAS_INSUFFICIENT_IP_ADDRESSES" => {
                FleetErrorCode::SubnetHasInsufficientIpAddresses
            }
            "SUBNET_NOT_FOUND" => FleetErrorCode::SubnetNotFound,
            _ => FleetErrorCode::UnknownVariant(UnknownFleetErrorCode { name }),
        }
    }
}

impl ::std::str::FromStr for FleetErrorCode {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(s.into())
    }
}

#[cfg(any(test, feature = "serialize_structs"))]
impl Serialize for FleetErrorCode {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.into())
    }
}

impl<'de> Deserialize<'de> for FleetErrorCode {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Ok(String::deserialize(deserializer)?.into())
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct UnknownFleetState {
    name: String,
}

#[allow(non_camel_case_types)]
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
#[non_exhaustive]
pub enum FleetState {
    Running,
    Starting,
    Stopped,
    Stopping,
    #[doc(hidden)]
    UnknownVariant(UnknownFleetState),
}

impl Default for FleetState {
    fn default() -> Self {
        "".into()
    }
}

impl fmt::Display for FleetState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.into())
    }
}

impl rusoto_core::param::ToParam for FleetState {
    fn to_param(&self) -> String {
        self.to_string()
    }
}

impl Into<String> for FleetState {
    fn into(self) -> String {
        match self {
            FleetState::Running => "RUNNING".to_string(),
            FleetState::Starting => "STARTING".to_string(),
            FleetState::Stopped => "STOPPED".to_string(),
            FleetState::Stopping => "STOPPING".to_string(),
            FleetState::UnknownVariant(UnknownFleetState { name: original }) => original,
        }
    }
}

impl<'a> Into<&'a str> for &'a FleetState {
    fn into(self) -> &'a str {
        match self {
            FleetState::Running => &"RUNNING",
            FleetState::Starting => &"STARTING",
            FleetState::Stopped => &"STOPPED",
            FleetState::Stopping => &"STOPPING",
            FleetState::UnknownVariant(UnknownFleetState { name: original }) => original,
        }
    }
}

impl From<&str> for FleetState {
    fn from(name: &str) -> Self {
        match name {
            "RUNNING" => FleetState::Running,
            "STARTING" => FleetState::Starting,
            "STOPPED" => FleetState::Stopped,
            "STOPPING" => FleetState::Stopping,
            _ => FleetState::UnknownVariant(UnknownFleetState {
                name: name.to_owned(),
            }),
        }
    }
}

impl From<String> for FleetState {
    fn from(name: String) -> Self {
        match &*name {
            "RUNNING" => FleetState::Running,
            "STARTING" => FleetState::Starting,
            "STOPPED" => FleetState::Stopped,
            "STOPPING" => FleetState::Stopping,
            _ => FleetState::UnknownVariant(UnknownFleetState { name }),
        }
    }
}

impl ::std::str::FromStr for FleetState {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(s.into())
    }
}

#[cfg(any(test, feature = "serialize_structs"))]
impl Serialize for FleetState {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.into())
    }
}

impl<'de> Deserialize<'de> for FleetState {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Ok(String::deserialize(deserializer)?.into())
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct UnknownFleetType {
    name: String,
}

#[allow(non_camel_case_types)]
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
#[non_exhaustive]
pub enum FleetType {
    AlwaysOn,
    OnDemand,
    #[doc(hidden)]
    UnknownVariant(UnknownFleetType),
}

impl Default for FleetType {
    fn default() -> Self {
        "".into()
    }
}

impl fmt::Display for FleetType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.into())
    }
}

impl rusoto_core::param::ToParam for FleetType {
    fn to_param(&self) -> String {
        self.to_string()
    }
}

impl Into<String> for FleetType {
    fn into(self) -> String {
        match self {
            FleetType::AlwaysOn => "ALWAYS_ON".to_string(),
            FleetType::OnDemand => "ON_DEMAND".to_string(),
            FleetType::UnknownVariant(UnknownFleetType { name: original }) => original,
        }
    }
}

impl<'a> Into<&'a str> for &'a FleetType {
    fn into(self) -> &'a str {
        match self {
            FleetType::AlwaysOn => &"ALWAYS_ON",
            FleetType::OnDemand => &"ON_DEMAND",
            FleetType::UnknownVariant(UnknownFleetType { name: original }) => original,
        }
    }
}

impl From<&str> for FleetType {
    fn from(name: &str) -> Self {
        match name {
            "ALWAYS_ON" => FleetType::AlwaysOn,
            "ON_DEMAND" => FleetType::OnDemand,
            _ => FleetType::UnknownVariant(UnknownFleetType {
                name: name.to_owned(),
            }),
        }
    }
}

impl From<String> for FleetType {
    fn from(name: String) -> Self {
        match &*name {
            "ALWAYS_ON" => FleetType::AlwaysOn,
            "ON_DEMAND" => FleetType::OnDemand,
            _ => FleetType::UnknownVariant(UnknownFleetType { name }),
        }
    }
}

impl ::std::str::FromStr for FleetType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(s.into())
    }
}

impl Serialize for FleetType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.into())
    }
}

impl<'de> Deserialize<'de> for FleetType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Ok(String::deserialize(deserializer)?.into())
    }
}

/// <p>Describes an image.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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
    /// <p>The description to display.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The image name to display.</p>
    #[serde(rename = "DisplayName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    /// <p>The name of the image builder that was used to create the private image. If the image is shared, this value is null.</p>
    #[serde(rename = "ImageBuilderName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_builder_name: Option<String>,
    /// <p>Indicates whether an image builder can be launched from this image.</p>
    #[serde(rename = "ImageBuilderSupported")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_builder_supported: Option<bool>,
    /// <p>The permissions to provide to the destination AWS account for the specified image.</p>
    #[serde(rename = "ImagePermissions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_permissions: Option<ImagePermissions>,
    /// <p>The name of the image.</p>
    #[serde(rename = "Name")]
    pub name: String,
    /// <p>The operating system platform of the image.</p>
    #[serde(rename = "Platform")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub platform: Option<PlatformType>,
    /// <p>The release date of the public base image. For private images, this date is the release date of the base image from which the image was created.</p>
    #[serde(rename = "PublicBaseImageReleasedDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub public_base_image_released_date: Option<f64>,
    /// <p>The image starts in the <code>PENDING</code> state. If image creation succeeds, the state is <code>AVAILABLE</code>. If image creation fails, the state is <code>FAILED</code>.</p>
    #[serde(rename = "State")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<ImageState>,
    /// <p>The reason why the last state change occurred.</p>
    #[serde(rename = "StateChangeReason")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state_change_reason: Option<ImageStateChangeReason>,
    /// <p>Indicates whether the image is public or private.</p>
    #[serde(rename = "Visibility")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub visibility: Option<VisibilityType>,
}

/// <p>Describes a virtual machine that is used to create an image. </p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ImageBuilder {
    /// <p>The list of virtual private cloud (VPC) interface endpoint objects. Administrators can connect to the image builder only through the specified endpoints.</p>
    #[serde(rename = "AccessEndpoints")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_endpoints: Option<Vec<AccessEndpoint>>,
    /// <p>The version of the AppStream 2.0 agent that is currently being used by the image builder. </p>
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
    /// <p>The description to display.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The image builder name to display.</p>
    #[serde(rename = "DisplayName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    /// <p>The name of the directory and organizational unit (OU) to use to join the image builder to a Microsoft Active Directory domain. </p>
    #[serde(rename = "DomainJoinInfo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_join_info: Option<DomainJoinInfo>,
    /// <p>Enables or disables default internet access for the image builder.</p>
    #[serde(rename = "EnableDefaultInternetAccess")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_default_internet_access: Option<bool>,
    /// <p>The ARN of the IAM role that is applied to the image builder. To assume a role, the image builder calls the AWS Security Token Service (STS) <code>AssumeRole</code> API operation and passes the ARN of the role to use. The operation creates a new session with temporary credentials. AppStream 2.0 retrieves the temporary credentials and creates the <b>appstream_machine_role</b> credential profile on the instance.</p> <p>For more information, see <a href="https://docs.aws.amazon.com/appstream2/latest/developerguide/using-iam-roles-to-grant-permissions-to-applications-scripts-streaming-instances.html">Using an IAM Role to Grant Permissions to Applications and Scripts Running on AppStream 2.0 Streaming Instances</a> in the <i>Amazon AppStream 2.0 Administration Guide</i>.</p>
    #[serde(rename = "IamRoleArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iam_role_arn: Option<String>,
    /// <p>The ARN of the image from which this builder was created.</p>
    #[serde(rename = "ImageArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_arn: Option<String>,
    /// <p>The image builder errors.</p>
    #[serde(rename = "ImageBuilderErrors")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_builder_errors: Option<Vec<ResourceError>>,
    /// <p><p>The instance type for the image builder. The following instance types are available:</p> <ul> <li> <p>stream.standard.medium</p> </li> <li> <p>stream.standard.large</p> </li> <li> <p>stream.compute.large</p> </li> <li> <p>stream.compute.xlarge</p> </li> <li> <p>stream.compute.2xlarge</p> </li> <li> <p>stream.compute.4xlarge</p> </li> <li> <p>stream.compute.8xlarge</p> </li> <li> <p>stream.memory.large</p> </li> <li> <p>stream.memory.xlarge</p> </li> <li> <p>stream.memory.2xlarge</p> </li> <li> <p>stream.memory.4xlarge</p> </li> <li> <p>stream.memory.8xlarge</p> </li> <li> <p>stream.memory.z1d.large</p> </li> <li> <p>stream.memory.z1d.xlarge</p> </li> <li> <p>stream.memory.z1d.2xlarge</p> </li> <li> <p>stream.memory.z1d.3xlarge</p> </li> <li> <p>stream.memory.z1d.6xlarge</p> </li> <li> <p>stream.memory.z1d.12xlarge</p> </li> <li> <p>stream.graphics-design.large</p> </li> <li> <p>stream.graphics-design.xlarge</p> </li> <li> <p>stream.graphics-design.2xlarge</p> </li> <li> <p>stream.graphics-design.4xlarge</p> </li> <li> <p>stream.graphics-desktop.2xlarge</p> </li> <li> <p>stream.graphics.g4dn.xlarge</p> </li> <li> <p>stream.graphics.g4dn.2xlarge</p> </li> <li> <p>stream.graphics.g4dn.4xlarge</p> </li> <li> <p>stream.graphics.g4dn.8xlarge</p> </li> <li> <p>stream.graphics.g4dn.12xlarge</p> </li> <li> <p>stream.graphics.g4dn.16xlarge</p> </li> <li> <p>stream.graphics-pro.4xlarge</p> </li> <li> <p>stream.graphics-pro.8xlarge</p> </li> <li> <p>stream.graphics-pro.16xlarge</p> </li> </ul></p>
    #[serde(rename = "InstanceType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_type: Option<String>,
    /// <p>The name of the image builder.</p>
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "NetworkAccessConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_access_configuration: Option<NetworkAccessConfiguration>,
    /// <p>The operating system platform of the image builder.</p>
    #[serde(rename = "Platform")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub platform: Option<PlatformType>,
    /// <p>The state of the image builder.</p>
    #[serde(rename = "State")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<ImageBuilderState>,
    /// <p>The reason why the last state change occurred.</p>
    #[serde(rename = "StateChangeReason")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state_change_reason: Option<ImageBuilderStateChangeReason>,
    /// <p>The VPC configuration of the image builder.</p>
    #[serde(rename = "VpcConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_config: Option<VpcConfig>,
}

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct UnknownImageBuilderState {
    name: String,
}

#[allow(non_camel_case_types)]
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
#[non_exhaustive]
pub enum ImageBuilderState {
    Deleting,
    Failed,
    Pending,
    Rebooting,
    Running,
    Snapshotting,
    Stopped,
    Stopping,
    UpdatingAgent,
    #[doc(hidden)]
    UnknownVariant(UnknownImageBuilderState),
}

impl Default for ImageBuilderState {
    fn default() -> Self {
        "".into()
    }
}

impl fmt::Display for ImageBuilderState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.into())
    }
}

impl rusoto_core::param::ToParam for ImageBuilderState {
    fn to_param(&self) -> String {
        self.to_string()
    }
}

impl Into<String> for ImageBuilderState {
    fn into(self) -> String {
        match self {
            ImageBuilderState::Deleting => "DELETING".to_string(),
            ImageBuilderState::Failed => "FAILED".to_string(),
            ImageBuilderState::Pending => "PENDING".to_string(),
            ImageBuilderState::Rebooting => "REBOOTING".to_string(),
            ImageBuilderState::Running => "RUNNING".to_string(),
            ImageBuilderState::Snapshotting => "SNAPSHOTTING".to_string(),
            ImageBuilderState::Stopped => "STOPPED".to_string(),
            ImageBuilderState::Stopping => "STOPPING".to_string(),
            ImageBuilderState::UpdatingAgent => "UPDATING_AGENT".to_string(),
            ImageBuilderState::UnknownVariant(UnknownImageBuilderState { name: original }) => {
                original
            }
        }
    }
}

impl<'a> Into<&'a str> for &'a ImageBuilderState {
    fn into(self) -> &'a str {
        match self {
            ImageBuilderState::Deleting => &"DELETING",
            ImageBuilderState::Failed => &"FAILED",
            ImageBuilderState::Pending => &"PENDING",
            ImageBuilderState::Rebooting => &"REBOOTING",
            ImageBuilderState::Running => &"RUNNING",
            ImageBuilderState::Snapshotting => &"SNAPSHOTTING",
            ImageBuilderState::Stopped => &"STOPPED",
            ImageBuilderState::Stopping => &"STOPPING",
            ImageBuilderState::UpdatingAgent => &"UPDATING_AGENT",
            ImageBuilderState::UnknownVariant(UnknownImageBuilderState { name: original }) => {
                original
            }
        }
    }
}

impl From<&str> for ImageBuilderState {
    fn from(name: &str) -> Self {
        match name {
            "DELETING" => ImageBuilderState::Deleting,
            "FAILED" => ImageBuilderState::Failed,
            "PENDING" => ImageBuilderState::Pending,
            "REBOOTING" => ImageBuilderState::Rebooting,
            "RUNNING" => ImageBuilderState::Running,
            "SNAPSHOTTING" => ImageBuilderState::Snapshotting,
            "STOPPED" => ImageBuilderState::Stopped,
            "STOPPING" => ImageBuilderState::Stopping,
            "UPDATING_AGENT" => ImageBuilderState::UpdatingAgent,
            _ => ImageBuilderState::UnknownVariant(UnknownImageBuilderState {
                name: name.to_owned(),
            }),
        }
    }
}

impl From<String> for ImageBuilderState {
    fn from(name: String) -> Self {
        match &*name {
            "DELETING" => ImageBuilderState::Deleting,
            "FAILED" => ImageBuilderState::Failed,
            "PENDING" => ImageBuilderState::Pending,
            "REBOOTING" => ImageBuilderState::Rebooting,
            "RUNNING" => ImageBuilderState::Running,
            "SNAPSHOTTING" => ImageBuilderState::Snapshotting,
            "STOPPED" => ImageBuilderState::Stopped,
            "STOPPING" => ImageBuilderState::Stopping,
            "UPDATING_AGENT" => ImageBuilderState::UpdatingAgent,
            _ => ImageBuilderState::UnknownVariant(UnknownImageBuilderState { name }),
        }
    }
}

impl ::std::str::FromStr for ImageBuilderState {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(s.into())
    }
}

#[cfg(any(test, feature = "serialize_structs"))]
impl Serialize for ImageBuilderState {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.into())
    }
}

impl<'de> Deserialize<'de> for ImageBuilderState {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Ok(String::deserialize(deserializer)?.into())
    }
}

/// <p>Describes the reason why the last image builder state change occurred.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ImageBuilderStateChangeReason {
    /// <p>The state change reason code.</p>
    #[serde(rename = "Code")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<ImageBuilderStateChangeReasonCode>,
    /// <p>The state change reason message.</p>
    #[serde(rename = "Message")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct UnknownImageBuilderStateChangeReasonCode {
    name: String,
}

#[allow(non_camel_case_types)]
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
#[non_exhaustive]
pub enum ImageBuilderStateChangeReasonCode {
    ImageUnavailable,
    InternalError,
    #[doc(hidden)]
    UnknownVariant(UnknownImageBuilderStateChangeReasonCode),
}

impl Default for ImageBuilderStateChangeReasonCode {
    fn default() -> Self {
        "".into()
    }
}

impl fmt::Display for ImageBuilderStateChangeReasonCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.into())
    }
}

impl rusoto_core::param::ToParam for ImageBuilderStateChangeReasonCode {
    fn to_param(&self) -> String {
        self.to_string()
    }
}

impl Into<String> for ImageBuilderStateChangeReasonCode {
    fn into(self) -> String {
        match self {
            ImageBuilderStateChangeReasonCode::ImageUnavailable => "IMAGE_UNAVAILABLE".to_string(),
            ImageBuilderStateChangeReasonCode::InternalError => "INTERNAL_ERROR".to_string(),
            ImageBuilderStateChangeReasonCode::UnknownVariant(
                UnknownImageBuilderStateChangeReasonCode { name: original },
            ) => original,
        }
    }
}

impl<'a> Into<&'a str> for &'a ImageBuilderStateChangeReasonCode {
    fn into(self) -> &'a str {
        match self {
            ImageBuilderStateChangeReasonCode::ImageUnavailable => &"IMAGE_UNAVAILABLE",
            ImageBuilderStateChangeReasonCode::InternalError => &"INTERNAL_ERROR",
            ImageBuilderStateChangeReasonCode::UnknownVariant(
                UnknownImageBuilderStateChangeReasonCode { name: original },
            ) => original,
        }
    }
}

impl From<&str> for ImageBuilderStateChangeReasonCode {
    fn from(name: &str) -> Self {
        match name {
            "IMAGE_UNAVAILABLE" => ImageBuilderStateChangeReasonCode::ImageUnavailable,
            "INTERNAL_ERROR" => ImageBuilderStateChangeReasonCode::InternalError,
            _ => ImageBuilderStateChangeReasonCode::UnknownVariant(
                UnknownImageBuilderStateChangeReasonCode {
                    name: name.to_owned(),
                },
            ),
        }
    }
}

impl From<String> for ImageBuilderStateChangeReasonCode {
    fn from(name: String) -> Self {
        match &*name {
            "IMAGE_UNAVAILABLE" => ImageBuilderStateChangeReasonCode::ImageUnavailable,
            "INTERNAL_ERROR" => ImageBuilderStateChangeReasonCode::InternalError,
            _ => ImageBuilderStateChangeReasonCode::UnknownVariant(
                UnknownImageBuilderStateChangeReasonCode { name },
            ),
        }
    }
}

impl ::std::str::FromStr for ImageBuilderStateChangeReasonCode {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(s.into())
    }
}

#[cfg(any(test, feature = "serialize_structs"))]
impl Serialize for ImageBuilderStateChangeReasonCode {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.into())
    }
}

impl<'de> Deserialize<'de> for ImageBuilderStateChangeReasonCode {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Ok(String::deserialize(deserializer)?.into())
    }
}

/// <p>Describes the permissions for an image. </p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct ImagePermissions {
    /// <p>Indicates whether the image can be used for a fleet.</p>
    #[serde(rename = "allowFleet")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_fleet: Option<bool>,
    /// <p>Indicates whether the image can be used for an image builder.</p>
    #[serde(rename = "allowImageBuilder")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_image_builder: Option<bool>,
}

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct UnknownImageState {
    name: String,
}

#[allow(non_camel_case_types)]
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
#[non_exhaustive]
pub enum ImageState {
    Available,
    Copying,
    Deleting,
    Failed,
    Pending,
    #[doc(hidden)]
    UnknownVariant(UnknownImageState),
}

impl Default for ImageState {
    fn default() -> Self {
        "".into()
    }
}

impl fmt::Display for ImageState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.into())
    }
}

impl rusoto_core::param::ToParam for ImageState {
    fn to_param(&self) -> String {
        self.to_string()
    }
}

impl Into<String> for ImageState {
    fn into(self) -> String {
        match self {
            ImageState::Available => "AVAILABLE".to_string(),
            ImageState::Copying => "COPYING".to_string(),
            ImageState::Deleting => "DELETING".to_string(),
            ImageState::Failed => "FAILED".to_string(),
            ImageState::Pending => "PENDING".to_string(),
            ImageState::UnknownVariant(UnknownImageState { name: original }) => original,
        }
    }
}

impl<'a> Into<&'a str> for &'a ImageState {
    fn into(self) -> &'a str {
        match self {
            ImageState::Available => &"AVAILABLE",
            ImageState::Copying => &"COPYING",
            ImageState::Deleting => &"DELETING",
            ImageState::Failed => &"FAILED",
            ImageState::Pending => &"PENDING",
            ImageState::UnknownVariant(UnknownImageState { name: original }) => original,
        }
    }
}

impl From<&str> for ImageState {
    fn from(name: &str) -> Self {
        match name {
            "AVAILABLE" => ImageState::Available,
            "COPYING" => ImageState::Copying,
            "DELETING" => ImageState::Deleting,
            "FAILED" => ImageState::Failed,
            "PENDING" => ImageState::Pending,
            _ => ImageState::UnknownVariant(UnknownImageState {
                name: name.to_owned(),
            }),
        }
    }
}

impl From<String> for ImageState {
    fn from(name: String) -> Self {
        match &*name {
            "AVAILABLE" => ImageState::Available,
            "COPYING" => ImageState::Copying,
            "DELETING" => ImageState::Deleting,
            "FAILED" => ImageState::Failed,
            "PENDING" => ImageState::Pending,
            _ => ImageState::UnknownVariant(UnknownImageState { name }),
        }
    }
}

impl ::std::str::FromStr for ImageState {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(s.into())
    }
}

#[cfg(any(test, feature = "serialize_structs"))]
impl Serialize for ImageState {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.into())
    }
}

impl<'de> Deserialize<'de> for ImageState {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Ok(String::deserialize(deserializer)?.into())
    }
}

/// <p>Describes the reason why the last image state change occurred.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ImageStateChangeReason {
    /// <p>The state change reason code.</p>
    #[serde(rename = "Code")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<ImageStateChangeReasonCode>,
    /// <p>The state change reason message.</p>
    #[serde(rename = "Message")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct UnknownImageStateChangeReasonCode {
    name: String,
}

#[allow(non_camel_case_types)]
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
#[non_exhaustive]
pub enum ImageStateChangeReasonCode {
    ImageBuilderNotAvailable,
    ImageCopyFailure,
    InternalError,
    #[doc(hidden)]
    UnknownVariant(UnknownImageStateChangeReasonCode),
}

impl Default for ImageStateChangeReasonCode {
    fn default() -> Self {
        "".into()
    }
}

impl fmt::Display for ImageStateChangeReasonCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.into())
    }
}

impl rusoto_core::param::ToParam for ImageStateChangeReasonCode {
    fn to_param(&self) -> String {
        self.to_string()
    }
}

impl Into<String> for ImageStateChangeReasonCode {
    fn into(self) -> String {
        match self {
            ImageStateChangeReasonCode::ImageBuilderNotAvailable => {
                "IMAGE_BUILDER_NOT_AVAILABLE".to_string()
            }
            ImageStateChangeReasonCode::ImageCopyFailure => "IMAGE_COPY_FAILURE".to_string(),
            ImageStateChangeReasonCode::InternalError => "INTERNAL_ERROR".to_string(),
            ImageStateChangeReasonCode::UnknownVariant(UnknownImageStateChangeReasonCode {
                name: original,
            }) => original,
        }
    }
}

impl<'a> Into<&'a str> for &'a ImageStateChangeReasonCode {
    fn into(self) -> &'a str {
        match self {
            ImageStateChangeReasonCode::ImageBuilderNotAvailable => &"IMAGE_BUILDER_NOT_AVAILABLE",
            ImageStateChangeReasonCode::ImageCopyFailure => &"IMAGE_COPY_FAILURE",
            ImageStateChangeReasonCode::InternalError => &"INTERNAL_ERROR",
            ImageStateChangeReasonCode::UnknownVariant(UnknownImageStateChangeReasonCode {
                name: original,
            }) => original,
        }
    }
}

impl From<&str> for ImageStateChangeReasonCode {
    fn from(name: &str) -> Self {
        match name {
            "IMAGE_BUILDER_NOT_AVAILABLE" => ImageStateChangeReasonCode::ImageBuilderNotAvailable,
            "IMAGE_COPY_FAILURE" => ImageStateChangeReasonCode::ImageCopyFailure,
            "INTERNAL_ERROR" => ImageStateChangeReasonCode::InternalError,
            _ => ImageStateChangeReasonCode::UnknownVariant(UnknownImageStateChangeReasonCode {
                name: name.to_owned(),
            }),
        }
    }
}

impl From<String> for ImageStateChangeReasonCode {
    fn from(name: String) -> Self {
        match &*name {
            "IMAGE_BUILDER_NOT_AVAILABLE" => ImageStateChangeReasonCode::ImageBuilderNotAvailable,
            "IMAGE_COPY_FAILURE" => ImageStateChangeReasonCode::ImageCopyFailure,
            "INTERNAL_ERROR" => ImageStateChangeReasonCode::InternalError,
            _ => ImageStateChangeReasonCode::UnknownVariant(UnknownImageStateChangeReasonCode {
                name,
            }),
        }
    }
}

impl ::std::str::FromStr for ImageStateChangeReasonCode {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(s.into())
    }
}

#[cfg(any(test, feature = "serialize_structs"))]
impl Serialize for ImageStateChangeReasonCode {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.into())
    }
}

impl<'de> Deserialize<'de> for ImageStateChangeReasonCode {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Ok(String::deserialize(deserializer)?.into())
    }
}

/// <p>Describes the error that is returned when a usage report can't be generated.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct LastReportGenerationExecutionError {
    /// <p>The error code for the error that is returned when a usage report can't be generated.</p>
    #[serde(rename = "ErrorCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_code: Option<UsageReportExecutionErrorCode>,
    /// <p>The error message for the error that is returned when a usage report can't be generated.</p>
    #[serde(rename = "ErrorMessage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListAssociatedFleetsRequest {
    /// <p>The pagination token to use to retrieve the next page of results for this operation. If this value is null, it retrieves the first page.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The name of the stack.</p>
    #[serde(rename = "StackName")]
    pub stack_name: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListAssociatedFleetsResult {
    /// <p>The name of the fleet.</p>
    #[serde(rename = "Names")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub names: Option<Vec<String>>,
    /// <p>The pagination token to use to retrieve the next page of results for this operation. If there are no more pages, this value is null.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListAssociatedStacksRequest {
    /// <p>The name of the fleet.</p>
    #[serde(rename = "FleetName")]
    pub fleet_name: String,
    /// <p>The pagination token to use to retrieve the next page of results for this operation. If this value is null, it retrieves the first page.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListAssociatedStacksResult {
    /// <p>The name of the stack.</p>
    #[serde(rename = "Names")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub names: Option<Vec<String>>,
    /// <p>The pagination token to use to retrieve the next page of results for this operation. If there are no more pages, this value is null.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListTagsForResourceRequest {
    /// <p>The Amazon Resource Name (ARN) of the resource.</p>
    #[serde(rename = "ResourceArn")]
    pub resource_arn: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListTagsForResourceResponse {
    /// <p>The information about the tags.</p>
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<::std::collections::HashMap<String, String>>,
}

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct UnknownMessageAction {
    name: String,
}

#[allow(non_camel_case_types)]
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
#[non_exhaustive]
pub enum MessageAction {
    Resend,
    Suppress,
    #[doc(hidden)]
    UnknownVariant(UnknownMessageAction),
}

impl Default for MessageAction {
    fn default() -> Self {
        "".into()
    }
}

impl fmt::Display for MessageAction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.into())
    }
}

impl rusoto_core::param::ToParam for MessageAction {
    fn to_param(&self) -> String {
        self.to_string()
    }
}

impl Into<String> for MessageAction {
    fn into(self) -> String {
        match self {
            MessageAction::Resend => "RESEND".to_string(),
            MessageAction::Suppress => "SUPPRESS".to_string(),
            MessageAction::UnknownVariant(UnknownMessageAction { name: original }) => original,
        }
    }
}

impl<'a> Into<&'a str> for &'a MessageAction {
    fn into(self) -> &'a str {
        match self {
            MessageAction::Resend => &"RESEND",
            MessageAction::Suppress => &"SUPPRESS",
            MessageAction::UnknownVariant(UnknownMessageAction { name: original }) => original,
        }
    }
}

impl From<&str> for MessageAction {
    fn from(name: &str) -> Self {
        match name {
            "RESEND" => MessageAction::Resend,
            "SUPPRESS" => MessageAction::Suppress,
            _ => MessageAction::UnknownVariant(UnknownMessageAction {
                name: name.to_owned(),
            }),
        }
    }
}

impl From<String> for MessageAction {
    fn from(name: String) -> Self {
        match &*name {
            "RESEND" => MessageAction::Resend,
            "SUPPRESS" => MessageAction::Suppress,
            _ => MessageAction::UnknownVariant(UnknownMessageAction { name }),
        }
    }
}

impl ::std::str::FromStr for MessageAction {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(s.into())
    }
}

impl Serialize for MessageAction {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.into())
    }
}

#[cfg(feature = "deserialize_structs")]
impl<'de> Deserialize<'de> for MessageAction {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Ok(String::deserialize(deserializer)?.into())
    }
}

/// <p>Describes the network details of the fleet or image builder instance.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct NetworkAccessConfiguration {
    /// <p>The resource identifier of the elastic network interface that is attached to instances in your VPC. All network interfaces have the eni-xxxxxxxx resource identifier.</p>
    #[serde(rename = "EniId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub eni_id: Option<String>,
    /// <p>The private IP address of the elastic network interface that is attached to instances in your VPC.</p>
    #[serde(rename = "EniPrivateIpAddress")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub eni_private_ip_address: Option<String>,
}

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct UnknownPermission {
    name: String,
}

#[allow(non_camel_case_types)]
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
#[non_exhaustive]
pub enum Permission {
    Disabled,
    Enabled,
    #[doc(hidden)]
    UnknownVariant(UnknownPermission),
}

impl Default for Permission {
    fn default() -> Self {
        "".into()
    }
}

impl fmt::Display for Permission {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.into())
    }
}

impl rusoto_core::param::ToParam for Permission {
    fn to_param(&self) -> String {
        self.to_string()
    }
}

impl Into<String> for Permission {
    fn into(self) -> String {
        match self {
            Permission::Disabled => "DISABLED".to_string(),
            Permission::Enabled => "ENABLED".to_string(),
            Permission::UnknownVariant(UnknownPermission { name: original }) => original,
        }
    }
}

impl<'a> Into<&'a str> for &'a Permission {
    fn into(self) -> &'a str {
        match self {
            Permission::Disabled => &"DISABLED",
            Permission::Enabled => &"ENABLED",
            Permission::UnknownVariant(UnknownPermission { name: original }) => original,
        }
    }
}

impl From<&str> for Permission {
    fn from(name: &str) -> Self {
        match name {
            "DISABLED" => Permission::Disabled,
            "ENABLED" => Permission::Enabled,
            _ => Permission::UnknownVariant(UnknownPermission {
                name: name.to_owned(),
            }),
        }
    }
}

impl From<String> for Permission {
    fn from(name: String) -> Self {
        match &*name {
            "DISABLED" => Permission::Disabled,
            "ENABLED" => Permission::Enabled,
            _ => Permission::UnknownVariant(UnknownPermission { name }),
        }
    }
}

impl ::std::str::FromStr for Permission {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(s.into())
    }
}

impl Serialize for Permission {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.into())
    }
}

impl<'de> Deserialize<'de> for Permission {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Ok(String::deserialize(deserializer)?.into())
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct UnknownPlatformType {
    name: String,
}

#[allow(non_camel_case_types)]
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
#[non_exhaustive]
pub enum PlatformType {
    Windows,
    WindowsServer2016,
    WindowsServer2019,
    #[doc(hidden)]
    UnknownVariant(UnknownPlatformType),
}

impl Default for PlatformType {
    fn default() -> Self {
        "".into()
    }
}

impl fmt::Display for PlatformType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.into())
    }
}

impl rusoto_core::param::ToParam for PlatformType {
    fn to_param(&self) -> String {
        self.to_string()
    }
}

impl Into<String> for PlatformType {
    fn into(self) -> String {
        match self {
            PlatformType::Windows => "WINDOWS".to_string(),
            PlatformType::WindowsServer2016 => "WINDOWS_SERVER_2016".to_string(),
            PlatformType::WindowsServer2019 => "WINDOWS_SERVER_2019".to_string(),
            PlatformType::UnknownVariant(UnknownPlatformType { name: original }) => original,
        }
    }
}

impl<'a> Into<&'a str> for &'a PlatformType {
    fn into(self) -> &'a str {
        match self {
            PlatformType::Windows => &"WINDOWS",
            PlatformType::WindowsServer2016 => &"WINDOWS_SERVER_2016",
            PlatformType::WindowsServer2019 => &"WINDOWS_SERVER_2019",
            PlatformType::UnknownVariant(UnknownPlatformType { name: original }) => original,
        }
    }
}

impl From<&str> for PlatformType {
    fn from(name: &str) -> Self {
        match name {
            "WINDOWS" => PlatformType::Windows,
            "WINDOWS_SERVER_2016" => PlatformType::WindowsServer2016,
            "WINDOWS_SERVER_2019" => PlatformType::WindowsServer2019,
            _ => PlatformType::UnknownVariant(UnknownPlatformType {
                name: name.to_owned(),
            }),
        }
    }
}

impl From<String> for PlatformType {
    fn from(name: String) -> Self {
        match &*name {
            "WINDOWS" => PlatformType::Windows,
            "WINDOWS_SERVER_2016" => PlatformType::WindowsServer2016,
            "WINDOWS_SERVER_2019" => PlatformType::WindowsServer2019,
            _ => PlatformType::UnknownVariant(UnknownPlatformType { name }),
        }
    }
}

impl ::std::str::FromStr for PlatformType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(s.into())
    }
}

#[cfg(any(test, feature = "serialize_structs"))]
impl Serialize for PlatformType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.into())
    }
}

impl<'de> Deserialize<'de> for PlatformType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Ok(String::deserialize(deserializer)?.into())
    }
}

/// <p>Describes a resource error.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ResourceError {
    /// <p>The error code.</p>
    #[serde(rename = "ErrorCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_code: Option<FleetErrorCode>,
    /// <p>The error message.</p>
    #[serde(rename = "ErrorMessage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
    /// <p>The time the error occurred.</p>
    #[serde(rename = "ErrorTimestamp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_timestamp: Option<f64>,
}

/// <p>Describes the credentials for the service account used by the fleet or image builder to connect to the directory.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct ServiceAccountCredentials {
    /// <p>The user name of the account. This account must have the following privileges: create computer objects, join computers to the domain, and change/reset the password on descendant computer objects for the organizational units specified.</p>
    #[serde(rename = "AccountName")]
    pub account_name: String,
    /// <p>The password for the account.</p>
    #[serde(rename = "AccountPassword")]
    pub account_password: String,
}

/// <p>Describes a streaming session.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct Session {
    /// <p>The authentication method. The user is authenticated using a streaming URL (<code>API</code>) or SAML 2.0 federation (<code>SAML</code>).</p>
    #[serde(rename = "AuthenticationType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authentication_type: Option<AuthenticationType>,
    /// <p>Specifies whether a user is connected to the streaming session.</p>
    #[serde(rename = "ConnectionState")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connection_state: Option<SessionConnectionState>,
    /// <p>The name of the fleet for the streaming session.</p>
    #[serde(rename = "FleetName")]
    pub fleet_name: String,
    /// <p>The identifier of the streaming session.</p>
    #[serde(rename = "Id")]
    pub id: String,
    /// <p>The time when the streaming session is set to expire. This time is based on the <code>MaxUserDurationinSeconds</code> value, which determines the maximum length of time that a streaming session can run. A streaming session might end earlier than the time specified in <code>SessionMaxExpirationTime</code>, when the <code>DisconnectTimeOutInSeconds</code> elapses or the user chooses to end his or her session. If the <code>DisconnectTimeOutInSeconds</code> elapses, or the user chooses to end his or her session, the streaming instance is terminated and the streaming session ends.</p>
    #[serde(rename = "MaxExpirationTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_expiration_time: Option<f64>,
    /// <p>The network details for the streaming session.</p>
    #[serde(rename = "NetworkAccessConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_access_configuration: Option<NetworkAccessConfiguration>,
    /// <p>The name of the stack for the streaming session.</p>
    #[serde(rename = "StackName")]
    pub stack_name: String,
    /// <p>The time when a streaming instance is dedicated for the user.</p>
    #[serde(rename = "StartTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<f64>,
    /// <p>The current state of the streaming session.</p>
    #[serde(rename = "State")]
    pub state: SessionState,
    /// <p>The identifier of the user for whom the session was created.</p>
    #[serde(rename = "UserId")]
    pub user_id: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct UnknownSessionConnectionState {
    name: String,
}

#[allow(non_camel_case_types)]
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
#[non_exhaustive]
pub enum SessionConnectionState {
    Connected,
    NotConnected,
    #[doc(hidden)]
    UnknownVariant(UnknownSessionConnectionState),
}

impl Default for SessionConnectionState {
    fn default() -> Self {
        "".into()
    }
}

impl fmt::Display for SessionConnectionState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.into())
    }
}

impl rusoto_core::param::ToParam for SessionConnectionState {
    fn to_param(&self) -> String {
        self.to_string()
    }
}

impl Into<String> for SessionConnectionState {
    fn into(self) -> String {
        match self {
            SessionConnectionState::Connected => "CONNECTED".to_string(),
            SessionConnectionState::NotConnected => "NOT_CONNECTED".to_string(),
            SessionConnectionState::UnknownVariant(UnknownSessionConnectionState {
                name: original,
            }) => original,
        }
    }
}

impl<'a> Into<&'a str> for &'a SessionConnectionState {
    fn into(self) -> &'a str {
        match self {
            SessionConnectionState::Connected => &"CONNECTED",
            SessionConnectionState::NotConnected => &"NOT_CONNECTED",
            SessionConnectionState::UnknownVariant(UnknownSessionConnectionState {
                name: original,
            }) => original,
        }
    }
}

impl From<&str> for SessionConnectionState {
    fn from(name: &str) -> Self {
        match name {
            "CONNECTED" => SessionConnectionState::Connected,
            "NOT_CONNECTED" => SessionConnectionState::NotConnected,
            _ => SessionConnectionState::UnknownVariant(UnknownSessionConnectionState {
                name: name.to_owned(),
            }),
        }
    }
}

impl From<String> for SessionConnectionState {
    fn from(name: String) -> Self {
        match &*name {
            "CONNECTED" => SessionConnectionState::Connected,
            "NOT_CONNECTED" => SessionConnectionState::NotConnected,
            _ => SessionConnectionState::UnknownVariant(UnknownSessionConnectionState { name }),
        }
    }
}

impl ::std::str::FromStr for SessionConnectionState {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(s.into())
    }
}

#[cfg(any(test, feature = "serialize_structs"))]
impl Serialize for SessionConnectionState {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.into())
    }
}

impl<'de> Deserialize<'de> for SessionConnectionState {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Ok(String::deserialize(deserializer)?.into())
    }
}

/// <p>Possible values for the state of a streaming session.</p>

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct UnknownSessionState {
    name: String,
}

#[allow(non_camel_case_types)]
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
#[non_exhaustive]
pub enum SessionState {
    Active,
    Expired,
    Pending,
    #[doc(hidden)]
    UnknownVariant(UnknownSessionState),
}

impl Default for SessionState {
    fn default() -> Self {
        "".into()
    }
}

impl fmt::Display for SessionState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.into())
    }
}

impl rusoto_core::param::ToParam for SessionState {
    fn to_param(&self) -> String {
        self.to_string()
    }
}

impl Into<String> for SessionState {
    fn into(self) -> String {
        match self {
            SessionState::Active => "ACTIVE".to_string(),
            SessionState::Expired => "EXPIRED".to_string(),
            SessionState::Pending => "PENDING".to_string(),
            SessionState::UnknownVariant(UnknownSessionState { name: original }) => original,
        }
    }
}

impl<'a> Into<&'a str> for &'a SessionState {
    fn into(self) -> &'a str {
        match self {
            SessionState::Active => &"ACTIVE",
            SessionState::Expired => &"EXPIRED",
            SessionState::Pending => &"PENDING",
            SessionState::UnknownVariant(UnknownSessionState { name: original }) => original,
        }
    }
}

impl From<&str> for SessionState {
    fn from(name: &str) -> Self {
        match name {
            "ACTIVE" => SessionState::Active,
            "EXPIRED" => SessionState::Expired,
            "PENDING" => SessionState::Pending,
            _ => SessionState::UnknownVariant(UnknownSessionState {
                name: name.to_owned(),
            }),
        }
    }
}

impl From<String> for SessionState {
    fn from(name: String) -> Self {
        match &*name {
            "ACTIVE" => SessionState::Active,
            "EXPIRED" => SessionState::Expired,
            "PENDING" => SessionState::Pending,
            _ => SessionState::UnknownVariant(UnknownSessionState { name }),
        }
    }
}

impl ::std::str::FromStr for SessionState {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(s.into())
    }
}

#[cfg(any(test, feature = "serialize_structs"))]
impl Serialize for SessionState {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.into())
    }
}

impl<'de> Deserialize<'de> for SessionState {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Ok(String::deserialize(deserializer)?.into())
    }
}

/// <p>Describes the permissions that are available to the specified AWS account for a shared image.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct SharedImagePermissions {
    /// <p>Describes the permissions for a shared image.</p>
    #[serde(rename = "imagePermissions")]
    pub image_permissions: ImagePermissions,
    /// <p>The 12-digit identifier of the AWS account with which the image is shared.</p>
    #[serde(rename = "sharedAccountId")]
    pub shared_account_id: String,
}

/// <p>Describes a stack.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct Stack {
    /// <p>The list of virtual private cloud (VPC) interface endpoint objects. Users of the stack can connect to AppStream 2.0 only through the specified endpoints. </p>
    #[serde(rename = "AccessEndpoints")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_endpoints: Option<Vec<AccessEndpoint>>,
    /// <p>The persistent application settings for users of the stack.</p>
    #[serde(rename = "ApplicationSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_settings: Option<ApplicationSettingsResponse>,
    /// <p>The ARN of the stack.</p>
    #[serde(rename = "Arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p>The time the stack was created.</p>
    #[serde(rename = "CreatedTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_time: Option<f64>,
    /// <p>The description to display.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The stack name to display.</p>
    #[serde(rename = "DisplayName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    /// <p>The domains where AppStream 2.0 streaming sessions can be embedded in an iframe. You must approve the domains that you want to host embedded AppStream 2.0 streaming sessions.</p>
    #[serde(rename = "EmbedHostDomains")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub embed_host_domains: Option<Vec<String>>,
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
    /// <p>The actions that are enabled or disabled for users during their streaming sessions. By default these actions are enabled.</p>
    #[serde(rename = "UserSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_settings: Option<Vec<UserSetting>>,
}

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct UnknownStackAttribute {
    name: String,
}

#[allow(non_camel_case_types)]
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
#[non_exhaustive]
pub enum StackAttribute {
    AccessEndpoints,
    EmbedHostDomains,
    FeedbackUrl,
    IamRoleArn,
    RedirectUrl,
    StorageConnectors,
    StorageConnectorGoogleDrive,
    StorageConnectorHomefolders,
    StorageConnectorOneDrive,
    ThemeName,
    UserSettings,
    #[doc(hidden)]
    UnknownVariant(UnknownStackAttribute),
}

impl Default for StackAttribute {
    fn default() -> Self {
        "".into()
    }
}

impl fmt::Display for StackAttribute {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.into())
    }
}

impl rusoto_core::param::ToParam for StackAttribute {
    fn to_param(&self) -> String {
        self.to_string()
    }
}

impl Into<String> for StackAttribute {
    fn into(self) -> String {
        match self {
            StackAttribute::AccessEndpoints => "ACCESS_ENDPOINTS".to_string(),
            StackAttribute::EmbedHostDomains => "EMBED_HOST_DOMAINS".to_string(),
            StackAttribute::FeedbackUrl => "FEEDBACK_URL".to_string(),
            StackAttribute::IamRoleArn => "IAM_ROLE_ARN".to_string(),
            StackAttribute::RedirectUrl => "REDIRECT_URL".to_string(),
            StackAttribute::StorageConnectors => "STORAGE_CONNECTORS".to_string(),
            StackAttribute::StorageConnectorGoogleDrive => {
                "STORAGE_CONNECTOR_GOOGLE_DRIVE".to_string()
            }
            StackAttribute::StorageConnectorHomefolders => {
                "STORAGE_CONNECTOR_HOMEFOLDERS".to_string()
            }
            StackAttribute::StorageConnectorOneDrive => "STORAGE_CONNECTOR_ONE_DRIVE".to_string(),
            StackAttribute::ThemeName => "THEME_NAME".to_string(),
            StackAttribute::UserSettings => "USER_SETTINGS".to_string(),
            StackAttribute::UnknownVariant(UnknownStackAttribute { name: original }) => original,
        }
    }
}

impl<'a> Into<&'a str> for &'a StackAttribute {
    fn into(self) -> &'a str {
        match self {
            StackAttribute::AccessEndpoints => &"ACCESS_ENDPOINTS",
            StackAttribute::EmbedHostDomains => &"EMBED_HOST_DOMAINS",
            StackAttribute::FeedbackUrl => &"FEEDBACK_URL",
            StackAttribute::IamRoleArn => &"IAM_ROLE_ARN",
            StackAttribute::RedirectUrl => &"REDIRECT_URL",
            StackAttribute::StorageConnectors => &"STORAGE_CONNECTORS",
            StackAttribute::StorageConnectorGoogleDrive => &"STORAGE_CONNECTOR_GOOGLE_DRIVE",
            StackAttribute::StorageConnectorHomefolders => &"STORAGE_CONNECTOR_HOMEFOLDERS",
            StackAttribute::StorageConnectorOneDrive => &"STORAGE_CONNECTOR_ONE_DRIVE",
            StackAttribute::ThemeName => &"THEME_NAME",
            StackAttribute::UserSettings => &"USER_SETTINGS",
            StackAttribute::UnknownVariant(UnknownStackAttribute { name: original }) => original,
        }
    }
}

impl From<&str> for StackAttribute {
    fn from(name: &str) -> Self {
        match name {
            "ACCESS_ENDPOINTS" => StackAttribute::AccessEndpoints,
            "EMBED_HOST_DOMAINS" => StackAttribute::EmbedHostDomains,
            "FEEDBACK_URL" => StackAttribute::FeedbackUrl,
            "IAM_ROLE_ARN" => StackAttribute::IamRoleArn,
            "REDIRECT_URL" => StackAttribute::RedirectUrl,
            "STORAGE_CONNECTORS" => StackAttribute::StorageConnectors,
            "STORAGE_CONNECTOR_GOOGLE_DRIVE" => StackAttribute::StorageConnectorGoogleDrive,
            "STORAGE_CONNECTOR_HOMEFOLDERS" => StackAttribute::StorageConnectorHomefolders,
            "STORAGE_CONNECTOR_ONE_DRIVE" => StackAttribute::StorageConnectorOneDrive,
            "THEME_NAME" => StackAttribute::ThemeName,
            "USER_SETTINGS" => StackAttribute::UserSettings,
            _ => StackAttribute::UnknownVariant(UnknownStackAttribute {
                name: name.to_owned(),
            }),
        }
    }
}

impl From<String> for StackAttribute {
    fn from(name: String) -> Self {
        match &*name {
            "ACCESS_ENDPOINTS" => StackAttribute::AccessEndpoints,
            "EMBED_HOST_DOMAINS" => StackAttribute::EmbedHostDomains,
            "FEEDBACK_URL" => StackAttribute::FeedbackUrl,
            "IAM_ROLE_ARN" => StackAttribute::IamRoleArn,
            "REDIRECT_URL" => StackAttribute::RedirectUrl,
            "STORAGE_CONNECTORS" => StackAttribute::StorageConnectors,
            "STORAGE_CONNECTOR_GOOGLE_DRIVE" => StackAttribute::StorageConnectorGoogleDrive,
            "STORAGE_CONNECTOR_HOMEFOLDERS" => StackAttribute::StorageConnectorHomefolders,
            "STORAGE_CONNECTOR_ONE_DRIVE" => StackAttribute::StorageConnectorOneDrive,
            "THEME_NAME" => StackAttribute::ThemeName,
            "USER_SETTINGS" => StackAttribute::UserSettings,
            _ => StackAttribute::UnknownVariant(UnknownStackAttribute { name }),
        }
    }
}

impl ::std::str::FromStr for StackAttribute {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(s.into())
    }
}

impl Serialize for StackAttribute {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.into())
    }
}

#[cfg(feature = "deserialize_structs")]
impl<'de> Deserialize<'de> for StackAttribute {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Ok(String::deserialize(deserializer)?.into())
    }
}

/// <p>Describes a stack error.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct StackError {
    /// <p>The error code.</p>
    #[serde(rename = "ErrorCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_code: Option<StackErrorCode>,
    /// <p>The error message.</p>
    #[serde(rename = "ErrorMessage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
}

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct UnknownStackErrorCode {
    name: String,
}

#[allow(non_camel_case_types)]
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
#[non_exhaustive]
pub enum StackErrorCode {
    InternalServiceError,
    StorageConnectorError,
    #[doc(hidden)]
    UnknownVariant(UnknownStackErrorCode),
}

impl Default for StackErrorCode {
    fn default() -> Self {
        "".into()
    }
}

impl fmt::Display for StackErrorCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.into())
    }
}

impl rusoto_core::param::ToParam for StackErrorCode {
    fn to_param(&self) -> String {
        self.to_string()
    }
}

impl Into<String> for StackErrorCode {
    fn into(self) -> String {
        match self {
            StackErrorCode::InternalServiceError => "INTERNAL_SERVICE_ERROR".to_string(),
            StackErrorCode::StorageConnectorError => "STORAGE_CONNECTOR_ERROR".to_string(),
            StackErrorCode::UnknownVariant(UnknownStackErrorCode { name: original }) => original,
        }
    }
}

impl<'a> Into<&'a str> for &'a StackErrorCode {
    fn into(self) -> &'a str {
        match self {
            StackErrorCode::InternalServiceError => &"INTERNAL_SERVICE_ERROR",
            StackErrorCode::StorageConnectorError => &"STORAGE_CONNECTOR_ERROR",
            StackErrorCode::UnknownVariant(UnknownStackErrorCode { name: original }) => original,
        }
    }
}

impl From<&str> for StackErrorCode {
    fn from(name: &str) -> Self {
        match name {
            "INTERNAL_SERVICE_ERROR" => StackErrorCode::InternalServiceError,
            "STORAGE_CONNECTOR_ERROR" => StackErrorCode::StorageConnectorError,
            _ => StackErrorCode::UnknownVariant(UnknownStackErrorCode {
                name: name.to_owned(),
            }),
        }
    }
}

impl From<String> for StackErrorCode {
    fn from(name: String) -> Self {
        match &*name {
            "INTERNAL_SERVICE_ERROR" => StackErrorCode::InternalServiceError,
            "STORAGE_CONNECTOR_ERROR" => StackErrorCode::StorageConnectorError,
            _ => StackErrorCode::UnknownVariant(UnknownStackErrorCode { name }),
        }
    }
}

impl ::std::str::FromStr for StackErrorCode {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(s.into())
    }
}

#[cfg(any(test, feature = "serialize_structs"))]
impl Serialize for StackErrorCode {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.into())
    }
}

impl<'de> Deserialize<'de> for StackErrorCode {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Ok(String::deserialize(deserializer)?.into())
    }
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct StartFleetRequest {
    /// <p>The name of the fleet.</p>
    #[serde(rename = "Name")]
    pub name: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct StartFleetResult {}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct StartImageBuilderRequest {
    /// <p>The version of the AppStream 2.0 agent to use for this image builder. To use the latest version of the AppStream 2.0 agent, specify [LATEST]. </p>
    #[serde(rename = "AppstreamAgentVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub appstream_agent_version: Option<String>,
    /// <p>The name of the image builder.</p>
    #[serde(rename = "Name")]
    pub name: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct StartImageBuilderResult {
    /// <p>Information about the image builder.</p>
    #[serde(rename = "ImageBuilder")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_builder: Option<ImageBuilder>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct StopFleetRequest {
    /// <p>The name of the fleet.</p>
    #[serde(rename = "Name")]
    pub name: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct StopFleetResult {}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct StopImageBuilderRequest {
    /// <p>The name of the image builder.</p>
    #[serde(rename = "Name")]
    pub name: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct StopImageBuilderResult {
    /// <p>Information about the image builder.</p>
    #[serde(rename = "ImageBuilder")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_builder: Option<ImageBuilder>,
}

/// <p>Describes a connector that enables persistent storage for users.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct StorageConnector {
    /// <p>The type of storage connector.</p>
    #[serde(rename = "ConnectorType")]
    pub connector_type: StorageConnectorType,
    /// <p>The names of the domains for the account.</p>
    #[serde(rename = "Domains")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domains: Option<Vec<String>>,
    /// <p>The ARN of the storage connector.</p>
    #[serde(rename = "ResourceIdentifier")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_identifier: Option<String>,
}

/// <p>The type of storage connector.</p>

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct UnknownStorageConnectorType {
    name: String,
}

#[allow(non_camel_case_types)]
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
#[non_exhaustive]
pub enum StorageConnectorType {
    GoogleDrive,
    Homefolders,
    OneDrive,
    #[doc(hidden)]
    UnknownVariant(UnknownStorageConnectorType),
}

impl Default for StorageConnectorType {
    fn default() -> Self {
        "".into()
    }
}

impl fmt::Display for StorageConnectorType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.into())
    }
}

impl rusoto_core::param::ToParam for StorageConnectorType {
    fn to_param(&self) -> String {
        self.to_string()
    }
}

impl Into<String> for StorageConnectorType {
    fn into(self) -> String {
        match self {
            StorageConnectorType::GoogleDrive => "GOOGLE_DRIVE".to_string(),
            StorageConnectorType::Homefolders => "HOMEFOLDERS".to_string(),
            StorageConnectorType::OneDrive => "ONE_DRIVE".to_string(),
            StorageConnectorType::UnknownVariant(UnknownStorageConnectorType {
                name: original,
            }) => original,
        }
    }
}

impl<'a> Into<&'a str> for &'a StorageConnectorType {
    fn into(self) -> &'a str {
        match self {
            StorageConnectorType::GoogleDrive => &"GOOGLE_DRIVE",
            StorageConnectorType::Homefolders => &"HOMEFOLDERS",
            StorageConnectorType::OneDrive => &"ONE_DRIVE",
            StorageConnectorType::UnknownVariant(UnknownStorageConnectorType {
                name: original,
            }) => original,
        }
    }
}

impl From<&str> for StorageConnectorType {
    fn from(name: &str) -> Self {
        match name {
            "GOOGLE_DRIVE" => StorageConnectorType::GoogleDrive,
            "HOMEFOLDERS" => StorageConnectorType::Homefolders,
            "ONE_DRIVE" => StorageConnectorType::OneDrive,
            _ => StorageConnectorType::UnknownVariant(UnknownStorageConnectorType {
                name: name.to_owned(),
            }),
        }
    }
}

impl From<String> for StorageConnectorType {
    fn from(name: String) -> Self {
        match &*name {
            "GOOGLE_DRIVE" => StorageConnectorType::GoogleDrive,
            "HOMEFOLDERS" => StorageConnectorType::Homefolders,
            "ONE_DRIVE" => StorageConnectorType::OneDrive,
            _ => StorageConnectorType::UnknownVariant(UnknownStorageConnectorType { name }),
        }
    }
}

impl ::std::str::FromStr for StorageConnectorType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(s.into())
    }
}

impl Serialize for StorageConnectorType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.into())
    }
}

impl<'de> Deserialize<'de> for StorageConnectorType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Ok(String::deserialize(deserializer)?.into())
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct UnknownStreamView {
    name: String,
}

#[allow(non_camel_case_types)]
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
#[non_exhaustive]
pub enum StreamView {
    App,
    Desktop,
    #[doc(hidden)]
    UnknownVariant(UnknownStreamView),
}

impl Default for StreamView {
    fn default() -> Self {
        "".into()
    }
}

impl fmt::Display for StreamView {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.into())
    }
}

impl rusoto_core::param::ToParam for StreamView {
    fn to_param(&self) -> String {
        self.to_string()
    }
}

impl Into<String> for StreamView {
    fn into(self) -> String {
        match self {
            StreamView::App => "APP".to_string(),
            StreamView::Desktop => "DESKTOP".to_string(),
            StreamView::UnknownVariant(UnknownStreamView { name: original }) => original,
        }
    }
}

impl<'a> Into<&'a str> for &'a StreamView {
    fn into(self) -> &'a str {
        match self {
            StreamView::App => &"APP",
            StreamView::Desktop => &"DESKTOP",
            StreamView::UnknownVariant(UnknownStreamView { name: original }) => original,
        }
    }
}

impl From<&str> for StreamView {
    fn from(name: &str) -> Self {
        match name {
            "APP" => StreamView::App,
            "DESKTOP" => StreamView::Desktop,
            _ => StreamView::UnknownVariant(UnknownStreamView {
                name: name.to_owned(),
            }),
        }
    }
}

impl From<String> for StreamView {
    fn from(name: String) -> Self {
        match &*name {
            "APP" => StreamView::App,
            "DESKTOP" => StreamView::Desktop,
            _ => StreamView::UnknownVariant(UnknownStreamView { name }),
        }
    }
}

impl ::std::str::FromStr for StreamView {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(s.into())
    }
}

impl Serialize for StreamView {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.into())
    }
}

impl<'de> Deserialize<'de> for StreamView {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Ok(String::deserialize(deserializer)?.into())
    }
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct TagResourceRequest {
    /// <p>The Amazon Resource Name (ARN) of the resource.</p>
    #[serde(rename = "ResourceArn")]
    pub resource_arn: String,
    /// <p>The tags to associate. A tag is a key-value pair, and the value is optional. For example, Environment=Test. If you do not specify a value, Environment=. </p> <p>If you do not specify a value, the value is set to an empty string.</p> <p>Generally allowed characters are: letters, numbers, and spaces representable in UTF-8, and the following special characters: </p> <p>_ . : / = + \ - @</p>
    #[serde(rename = "Tags")]
    pub tags: ::std::collections::HashMap<String, String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct TagResourceResponse {}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UntagResourceRequest {
    /// <p>The Amazon Resource Name (ARN) of the resource.</p>
    #[serde(rename = "ResourceArn")]
    pub resource_arn: String,
    /// <p>The tag keys for the tags to disassociate.</p>
    #[serde(rename = "TagKeys")]
    pub tag_keys: Vec<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UntagResourceResponse {}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UpdateDirectoryConfigRequest {
    /// <p>The name of the Directory Config object.</p>
    #[serde(rename = "DirectoryName")]
    pub directory_name: String,
    /// <p>The distinguished names of the organizational units for computer accounts.</p>
    #[serde(rename = "OrganizationalUnitDistinguishedNames")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub organizational_unit_distinguished_names: Option<Vec<String>>,
    /// <p>The credentials for the service account used by the fleet or image builder to connect to the directory.</p>
    #[serde(rename = "ServiceAccountCredentials")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_account_credentials: Option<ServiceAccountCredentials>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UpdateDirectoryConfigResult {
    /// <p>Information about the Directory Config object.</p>
    #[serde(rename = "DirectoryConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub directory_config: Option<DirectoryConfig>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UpdateFleetRequest {
    /// <p>The fleet attributes to delete.</p>
    #[serde(rename = "AttributesToDelete")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attributes_to_delete: Option<Vec<FleetAttribute>>,
    /// <p>The desired capacity for the fleet.</p>
    #[serde(rename = "ComputeCapacity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compute_capacity: Option<ComputeCapacity>,
    /// <p>The description to display.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The amount of time that a streaming session remains active after users disconnect. If users try to reconnect to the streaming session after a disconnection or network interruption within this time interval, they are connected to their previous session. Otherwise, they are connected to a new session with a new streaming instance. </p> <p>Specify a value between 60 and 360000.</p>
    #[serde(rename = "DisconnectTimeoutInSeconds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disconnect_timeout_in_seconds: Option<i64>,
    /// <p>The fleet name to display.</p>
    #[serde(rename = "DisplayName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    /// <p>The name of the directory and organizational unit (OU) to use to join the fleet to a Microsoft Active Directory domain. </p>
    #[serde(rename = "DomainJoinInfo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_join_info: Option<DomainJoinInfo>,
    /// <p>Enables or disables default internet access for the fleet.</p>
    #[serde(rename = "EnableDefaultInternetAccess")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_default_internet_access: Option<bool>,
    /// <p>The Amazon Resource Name (ARN) of the IAM role to apply to the fleet. To assume a role, a fleet instance calls the AWS Security Token Service (STS) <code>AssumeRole</code> API operation and passes the ARN of the role to use. The operation creates a new session with temporary credentials. AppStream 2.0 retrieves the temporary credentials and creates the <b>appstream_machine_role</b> credential profile on the instance.</p> <p>For more information, see <a href="https://docs.aws.amazon.com/appstream2/latest/developerguide/using-iam-roles-to-grant-permissions-to-applications-scripts-streaming-instances.html">Using an IAM Role to Grant Permissions to Applications and Scripts Running on AppStream 2.0 Streaming Instances</a> in the <i>Amazon AppStream 2.0 Administration Guide</i>.</p>
    #[serde(rename = "IamRoleArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iam_role_arn: Option<String>,
    /// <p><p>The amount of time that users can be idle (inactive) before they are disconnected from their streaming session and the <code>DisconnectTimeoutInSeconds</code> time interval begins. Users are notified before they are disconnected due to inactivity. If users try to reconnect to the streaming session before the time interval specified in <code>DisconnectTimeoutInSeconds</code> elapses, they are connected to their previous session. Users are considered idle when they stop providing keyboard or mouse input during their streaming session. File uploads and downloads, audio in, audio out, and pixels changing do not qualify as user activity. If users continue to be idle after the time interval in <code>IdleDisconnectTimeoutInSeconds</code> elapses, they are disconnected. </p> <p>To prevent users from being disconnected due to inactivity, specify a value of 0. Otherwise, specify a value between 60 and 3600. The default value is 0.</p> <note> <p>If you enable this feature, we recommend that you specify a value that corresponds exactly to a whole number of minutes (for example, 60, 120, and 180). If you don&#39;t do this, the value is rounded to the nearest minute. For example, if you specify a value of 70, users are disconnected after 1 minute of inactivity. If you specify a value that is at the midpoint between two different minutes, the value is rounded up. For example, if you specify a value of 90, users are disconnected after 2 minutes of inactivity. </p> </note></p>
    #[serde(rename = "IdleDisconnectTimeoutInSeconds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub idle_disconnect_timeout_in_seconds: Option<i64>,
    /// <p>The ARN of the public, private, or shared image to use.</p>
    #[serde(rename = "ImageArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_arn: Option<String>,
    /// <p>The name of the image used to create the fleet.</p>
    #[serde(rename = "ImageName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_name: Option<String>,
    /// <p><p>The instance type to use when launching fleet instances. The following instance types are available:</p> <ul> <li> <p>stream.standard.medium</p> </li> <li> <p>stream.standard.large</p> </li> <li> <p>stream.compute.large</p> </li> <li> <p>stream.compute.xlarge</p> </li> <li> <p>stream.compute.2xlarge</p> </li> <li> <p>stream.compute.4xlarge</p> </li> <li> <p>stream.compute.8xlarge</p> </li> <li> <p>stream.memory.large</p> </li> <li> <p>stream.memory.xlarge</p> </li> <li> <p>stream.memory.2xlarge</p> </li> <li> <p>stream.memory.4xlarge</p> </li> <li> <p>stream.memory.8xlarge</p> </li> <li> <p>stream.memory.z1d.large</p> </li> <li> <p>stream.memory.z1d.xlarge</p> </li> <li> <p>stream.memory.z1d.2xlarge</p> </li> <li> <p>stream.memory.z1d.3xlarge</p> </li> <li> <p>stream.memory.z1d.6xlarge</p> </li> <li> <p>stream.memory.z1d.12xlarge</p> </li> <li> <p>stream.graphics-design.large</p> </li> <li> <p>stream.graphics-design.xlarge</p> </li> <li> <p>stream.graphics-design.2xlarge</p> </li> <li> <p>stream.graphics-design.4xlarge</p> </li> <li> <p>stream.graphics-desktop.2xlarge</p> </li> <li> <p>stream.graphics.g4dn.xlarge</p> </li> <li> <p>stream.graphics.g4dn.2xlarge</p> </li> <li> <p>stream.graphics.g4dn.4xlarge</p> </li> <li> <p>stream.graphics.g4dn.8xlarge</p> </li> <li> <p>stream.graphics.g4dn.12xlarge</p> </li> <li> <p>stream.graphics.g4dn.16xlarge</p> </li> <li> <p>stream.graphics-pro.4xlarge</p> </li> <li> <p>stream.graphics-pro.8xlarge</p> </li> <li> <p>stream.graphics-pro.16xlarge</p> </li> </ul></p>
    #[serde(rename = "InstanceType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_type: Option<String>,
    /// <p>The maximum amount of time that a streaming session can remain active, in seconds. If users are still connected to a streaming instance five minutes before this limit is reached, they are prompted to save any open documents before being disconnected. After this time elapses, the instance is terminated and replaced by a new instance.</p> <p>Specify a value between 600 and 360000.</p>
    #[serde(rename = "MaxUserDurationInSeconds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_user_duration_in_seconds: Option<i64>,
    /// <p>A unique name for the fleet.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The AppStream 2.0 view that is displayed to your users when they stream from the fleet. When <code>APP</code> is specified, only the windows of applications opened by users display. When <code>DESKTOP</code> is specified, the standard desktop that is provided by the operating system displays.</p> <p>The default value is <code>APP</code>.</p>
    #[serde(rename = "StreamView")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream_view: Option<StreamView>,
    /// <p>The VPC configuration for the fleet.</p>
    #[serde(rename = "VpcConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_config: Option<VpcConfig>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UpdateFleetResult {
    /// <p>Information about the fleet.</p>
    #[serde(rename = "Fleet")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fleet: Option<Fleet>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UpdateImagePermissionsRequest {
    /// <p>The permissions for the image.</p>
    #[serde(rename = "ImagePermissions")]
    pub image_permissions: ImagePermissions,
    /// <p>The name of the private image.</p>
    #[serde(rename = "Name")]
    pub name: String,
    /// <p>The 12-digit identifier of the AWS account for which you want add or update image permissions.</p>
    #[serde(rename = "SharedAccountId")]
    pub shared_account_id: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UpdateImagePermissionsResult {}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UpdateStackRequest {
    /// <p>The list of interface VPC endpoint (interface endpoint) objects. Users of the stack can connect to AppStream 2.0 only through the specified endpoints.</p>
    #[serde(rename = "AccessEndpoints")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_endpoints: Option<Vec<AccessEndpoint>>,
    /// <p>The persistent application settings for users of a stack. When these settings are enabled, changes that users make to applications and Windows settings are automatically saved after each session and applied to the next session.</p>
    #[serde(rename = "ApplicationSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_settings: Option<ApplicationSettings>,
    /// <p>The stack attributes to delete.</p>
    #[serde(rename = "AttributesToDelete")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attributes_to_delete: Option<Vec<StackAttribute>>,
    /// <p>The description to display.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The stack name to display.</p>
    #[serde(rename = "DisplayName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    /// <p>The domains where AppStream 2.0 streaming sessions can be embedded in an iframe. You must approve the domains that you want to host embedded AppStream 2.0 streaming sessions. </p>
    #[serde(rename = "EmbedHostDomains")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub embed_host_domains: Option<Vec<String>>,
    /// <p>The URL that users are redirected to after they choose the Send Feedback link. If no URL is specified, no Send Feedback link is displayed.</p>
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
    /// <p>The actions that are enabled or disabled for users during their streaming sessions. By default, these actions are enabled.</p>
    #[serde(rename = "UserSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_settings: Option<Vec<UserSetting>>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UpdateStackResult {
    /// <p>Information about the stack.</p>
    #[serde(rename = "Stack")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stack: Option<Stack>,
}

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct UnknownUsageReportExecutionErrorCode {
    name: String,
}

#[allow(non_camel_case_types)]
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
#[non_exhaustive]
pub enum UsageReportExecutionErrorCode {
    AccessDenied,
    InternalServiceError,
    ResourceNotFound,
    #[doc(hidden)]
    UnknownVariant(UnknownUsageReportExecutionErrorCode),
}

impl Default for UsageReportExecutionErrorCode {
    fn default() -> Self {
        "".into()
    }
}

impl fmt::Display for UsageReportExecutionErrorCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.into())
    }
}

impl rusoto_core::param::ToParam for UsageReportExecutionErrorCode {
    fn to_param(&self) -> String {
        self.to_string()
    }
}

impl Into<String> for UsageReportExecutionErrorCode {
    fn into(self) -> String {
        match self {
            UsageReportExecutionErrorCode::AccessDenied => "ACCESS_DENIED".to_string(),
            UsageReportExecutionErrorCode::InternalServiceError => {
                "INTERNAL_SERVICE_ERROR".to_string()
            }
            UsageReportExecutionErrorCode::ResourceNotFound => "RESOURCE_NOT_FOUND".to_string(),
            UsageReportExecutionErrorCode::UnknownVariant(
                UnknownUsageReportExecutionErrorCode { name: original },
            ) => original,
        }
    }
}

impl<'a> Into<&'a str> for &'a UsageReportExecutionErrorCode {
    fn into(self) -> &'a str {
        match self {
            UsageReportExecutionErrorCode::AccessDenied => &"ACCESS_DENIED",
            UsageReportExecutionErrorCode::InternalServiceError => &"INTERNAL_SERVICE_ERROR",
            UsageReportExecutionErrorCode::ResourceNotFound => &"RESOURCE_NOT_FOUND",
            UsageReportExecutionErrorCode::UnknownVariant(
                UnknownUsageReportExecutionErrorCode { name: original },
            ) => original,
        }
    }
}

impl From<&str> for UsageReportExecutionErrorCode {
    fn from(name: &str) -> Self {
        match name {
            "ACCESS_DENIED" => UsageReportExecutionErrorCode::AccessDenied,
            "INTERNAL_SERVICE_ERROR" => UsageReportExecutionErrorCode::InternalServiceError,
            "RESOURCE_NOT_FOUND" => UsageReportExecutionErrorCode::ResourceNotFound,
            _ => UsageReportExecutionErrorCode::UnknownVariant(
                UnknownUsageReportExecutionErrorCode {
                    name: name.to_owned(),
                },
            ),
        }
    }
}

impl From<String> for UsageReportExecutionErrorCode {
    fn from(name: String) -> Self {
        match &*name {
            "ACCESS_DENIED" => UsageReportExecutionErrorCode::AccessDenied,
            "INTERNAL_SERVICE_ERROR" => UsageReportExecutionErrorCode::InternalServiceError,
            "RESOURCE_NOT_FOUND" => UsageReportExecutionErrorCode::ResourceNotFound,
            _ => UsageReportExecutionErrorCode::UnknownVariant(
                UnknownUsageReportExecutionErrorCode { name },
            ),
        }
    }
}

impl ::std::str::FromStr for UsageReportExecutionErrorCode {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(s.into())
    }
}

#[cfg(any(test, feature = "serialize_structs"))]
impl Serialize for UsageReportExecutionErrorCode {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.into())
    }
}

impl<'de> Deserialize<'de> for UsageReportExecutionErrorCode {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Ok(String::deserialize(deserializer)?.into())
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct UnknownUsageReportSchedule {
    name: String,
}

#[allow(non_camel_case_types)]
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
#[non_exhaustive]
pub enum UsageReportSchedule {
    Daily,
    #[doc(hidden)]
    UnknownVariant(UnknownUsageReportSchedule),
}

impl Default for UsageReportSchedule {
    fn default() -> Self {
        "".into()
    }
}

impl fmt::Display for UsageReportSchedule {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.into())
    }
}

impl rusoto_core::param::ToParam for UsageReportSchedule {
    fn to_param(&self) -> String {
        self.to_string()
    }
}

impl Into<String> for UsageReportSchedule {
    fn into(self) -> String {
        match self {
            UsageReportSchedule::Daily => "DAILY".to_string(),
            UsageReportSchedule::UnknownVariant(UnknownUsageReportSchedule { name: original }) => {
                original
            }
        }
    }
}

impl<'a> Into<&'a str> for &'a UsageReportSchedule {
    fn into(self) -> &'a str {
        match self {
            UsageReportSchedule::Daily => &"DAILY",
            UsageReportSchedule::UnknownVariant(UnknownUsageReportSchedule { name: original }) => {
                original
            }
        }
    }
}

impl From<&str> for UsageReportSchedule {
    fn from(name: &str) -> Self {
        match name {
            "DAILY" => UsageReportSchedule::Daily,
            _ => UsageReportSchedule::UnknownVariant(UnknownUsageReportSchedule {
                name: name.to_owned(),
            }),
        }
    }
}

impl From<String> for UsageReportSchedule {
    fn from(name: String) -> Self {
        match &*name {
            "DAILY" => UsageReportSchedule::Daily,
            _ => UsageReportSchedule::UnknownVariant(UnknownUsageReportSchedule { name }),
        }
    }
}

impl ::std::str::FromStr for UsageReportSchedule {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(s.into())
    }
}

#[cfg(any(test, feature = "serialize_structs"))]
impl Serialize for UsageReportSchedule {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.into())
    }
}

impl<'de> Deserialize<'de> for UsageReportSchedule {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Ok(String::deserialize(deserializer)?.into())
    }
}

/// <p>Describes information about the usage report subscription.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UsageReportSubscription {
    /// <p>The time when the last usage report was generated.</p>
    #[serde(rename = "LastGeneratedReportDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_generated_report_date: Option<f64>,
    /// <p>The Amazon S3 bucket where generated reports are stored.</p> <p>If you enabled on-instance session scripts and Amazon S3 logging for your session script configuration, AppStream 2.0 created an S3 bucket to store the script output. The bucket is unique to your account and Region. When you enable usage reporting in this case, AppStream 2.0 uses the same bucket to store your usage reports. If you haven't already enabled on-instance session scripts, when you enable usage reports, AppStream 2.0 creates a new S3 bucket.</p>
    #[serde(rename = "S3BucketName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_bucket_name: Option<String>,
    /// <p>The schedule for generating usage reports.</p>
    #[serde(rename = "Schedule")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schedule: Option<UsageReportSchedule>,
    /// <p>The errors that were returned if usage reports couldn't be generated.</p>
    #[serde(rename = "SubscriptionErrors")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscription_errors: Option<Vec<LastReportGenerationExecutionError>>,
}

/// <p>Describes a user in the user pool.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct User {
    /// <p>The ARN of the user.</p>
    #[serde(rename = "Arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p>The authentication type for the user.</p>
    #[serde(rename = "AuthenticationType")]
    pub authentication_type: AuthenticationType,
    /// <p>The date and time the user was created in the user pool.</p>
    #[serde(rename = "CreatedTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_time: Option<f64>,
    /// <p>Specifies whether the user in the user pool is enabled.</p>
    #[serde(rename = "Enabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    /// <p>The first name, or given name, of the user.</p>
    #[serde(rename = "FirstName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_name: Option<String>,
    /// <p>The last name, or surname, of the user.</p>
    #[serde(rename = "LastName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_name: Option<String>,
    /// <p><p>The status of the user in the user pool. The status can be one of the following:</p> <ul> <li> <p>UNCONFIRMED – The user is created but not confirmed.</p> </li> <li> <p>CONFIRMED – The user is confirmed.</p> </li> <li> <p>ARCHIVED – The user is no longer active.</p> </li> <li> <p>COMPROMISED – The user is disabled because of a potential security threat.</p> </li> <li> <p>UNKNOWN – The user status is not known.</p> </li> </ul></p>
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// <p><p>The email address of the user.</p> <note> <p>Users&#39; email addresses are case-sensitive.</p> </note></p>
    #[serde(rename = "UserName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_name: Option<String>,
}

/// <p>Describes an action and whether the action is enabled or disabled for users during their streaming sessions.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct UserSetting {
    /// <p>The action that is enabled or disabled.</p>
    #[serde(rename = "Action")]
    pub action: Action,
    /// <p>Indicates whether the action is enabled or disabled.</p>
    #[serde(rename = "Permission")]
    pub permission: Permission,
}

/// <p>Describes a user in the user pool and the associated stack.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct UserStackAssociation {
    /// <p>The authentication type for the user.</p>
    #[serde(rename = "AuthenticationType")]
    pub authentication_type: AuthenticationType,
    /// <p>Specifies whether a welcome email is sent to a user after the user is created in the user pool.</p>
    #[serde(rename = "SendEmailNotification")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub send_email_notification: Option<bool>,
    /// <p>The name of the stack that is associated with the user.</p>
    #[serde(rename = "StackName")]
    pub stack_name: String,
    /// <p><p>The email address of the user who is associated with the stack.</p> <note> <p>Users&#39; email addresses are case-sensitive.</p> </note></p>
    #[serde(rename = "UserName")]
    pub user_name: String,
}

/// <p>Describes the error that is returned when a user can’t be associated with or disassociated from a stack. </p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UserStackAssociationError {
    /// <p>The error code for the error that is returned when a user can’t be associated with or disassociated from a stack.</p>
    #[serde(rename = "ErrorCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_code: Option<UserStackAssociationErrorCode>,
    /// <p>The error message for the error that is returned when a user can’t be associated with or disassociated from a stack.</p>
    #[serde(rename = "ErrorMessage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
    /// <p>Information about the user and associated stack.</p>
    #[serde(rename = "UserStackAssociation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_stack_association: Option<UserStackAssociation>,
}

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct UnknownUserStackAssociationErrorCode {
    name: String,
}

#[allow(non_camel_case_types)]
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
#[non_exhaustive]
pub enum UserStackAssociationErrorCode {
    DirectoryNotFound,
    InternalError,
    StackNotFound,
    UserNameNotFound,
    #[doc(hidden)]
    UnknownVariant(UnknownUserStackAssociationErrorCode),
}

impl Default for UserStackAssociationErrorCode {
    fn default() -> Self {
        "".into()
    }
}

impl fmt::Display for UserStackAssociationErrorCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.into())
    }
}

impl rusoto_core::param::ToParam for UserStackAssociationErrorCode {
    fn to_param(&self) -> String {
        self.to_string()
    }
}

impl Into<String> for UserStackAssociationErrorCode {
    fn into(self) -> String {
        match self {
            UserStackAssociationErrorCode::DirectoryNotFound => "DIRECTORY_NOT_FOUND".to_string(),
            UserStackAssociationErrorCode::InternalError => "INTERNAL_ERROR".to_string(),
            UserStackAssociationErrorCode::StackNotFound => "STACK_NOT_FOUND".to_string(),
            UserStackAssociationErrorCode::UserNameNotFound => "USER_NAME_NOT_FOUND".to_string(),
            UserStackAssociationErrorCode::UnknownVariant(
                UnknownUserStackAssociationErrorCode { name: original },
            ) => original,
        }
    }
}

impl<'a> Into<&'a str> for &'a UserStackAssociationErrorCode {
    fn into(self) -> &'a str {
        match self {
            UserStackAssociationErrorCode::DirectoryNotFound => &"DIRECTORY_NOT_FOUND",
            UserStackAssociationErrorCode::InternalError => &"INTERNAL_ERROR",
            UserStackAssociationErrorCode::StackNotFound => &"STACK_NOT_FOUND",
            UserStackAssociationErrorCode::UserNameNotFound => &"USER_NAME_NOT_FOUND",
            UserStackAssociationErrorCode::UnknownVariant(
                UnknownUserStackAssociationErrorCode { name: original },
            ) => original,
        }
    }
}

impl From<&str> for UserStackAssociationErrorCode {
    fn from(name: &str) -> Self {
        match name {
            "DIRECTORY_NOT_FOUND" => UserStackAssociationErrorCode::DirectoryNotFound,
            "INTERNAL_ERROR" => UserStackAssociationErrorCode::InternalError,
            "STACK_NOT_FOUND" => UserStackAssociationErrorCode::StackNotFound,
            "USER_NAME_NOT_FOUND" => UserStackAssociationErrorCode::UserNameNotFound,
            _ => UserStackAssociationErrorCode::UnknownVariant(
                UnknownUserStackAssociationErrorCode {
                    name: name.to_owned(),
                },
            ),
        }
    }
}

impl From<String> for UserStackAssociationErrorCode {
    fn from(name: String) -> Self {
        match &*name {
            "DIRECTORY_NOT_FOUND" => UserStackAssociationErrorCode::DirectoryNotFound,
            "INTERNAL_ERROR" => UserStackAssociationErrorCode::InternalError,
            "STACK_NOT_FOUND" => UserStackAssociationErrorCode::StackNotFound,
            "USER_NAME_NOT_FOUND" => UserStackAssociationErrorCode::UserNameNotFound,
            _ => UserStackAssociationErrorCode::UnknownVariant(
                UnknownUserStackAssociationErrorCode { name },
            ),
        }
    }
}

impl ::std::str::FromStr for UserStackAssociationErrorCode {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(s.into())
    }
}

#[cfg(any(test, feature = "serialize_structs"))]
impl Serialize for UserStackAssociationErrorCode {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.into())
    }
}

impl<'de> Deserialize<'de> for UserStackAssociationErrorCode {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Ok(String::deserialize(deserializer)?.into())
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct UnknownVisibilityType {
    name: String,
}

#[allow(non_camel_case_types)]
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
#[non_exhaustive]
pub enum VisibilityType {
    Private,
    Public,
    Shared,
    #[doc(hidden)]
    UnknownVariant(UnknownVisibilityType),
}

impl Default for VisibilityType {
    fn default() -> Self {
        "".into()
    }
}

impl fmt::Display for VisibilityType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.into())
    }
}

impl rusoto_core::param::ToParam for VisibilityType {
    fn to_param(&self) -> String {
        self.to_string()
    }
}

impl Into<String> for VisibilityType {
    fn into(self) -> String {
        match self {
            VisibilityType::Private => "PRIVATE".to_string(),
            VisibilityType::Public => "PUBLIC".to_string(),
            VisibilityType::Shared => "SHARED".to_string(),
            VisibilityType::UnknownVariant(UnknownVisibilityType { name: original }) => original,
        }
    }
}

impl<'a> Into<&'a str> for &'a VisibilityType {
    fn into(self) -> &'a str {
        match self {
            VisibilityType::Private => &"PRIVATE",
            VisibilityType::Public => &"PUBLIC",
            VisibilityType::Shared => &"SHARED",
            VisibilityType::UnknownVariant(UnknownVisibilityType { name: original }) => original,
        }
    }
}

impl From<&str> for VisibilityType {
    fn from(name: &str) -> Self {
        match name {
            "PRIVATE" => VisibilityType::Private,
            "PUBLIC" => VisibilityType::Public,
            "SHARED" => VisibilityType::Shared,
            _ => VisibilityType::UnknownVariant(UnknownVisibilityType {
                name: name.to_owned(),
            }),
        }
    }
}

impl From<String> for VisibilityType {
    fn from(name: String) -> Self {
        match &*name {
            "PRIVATE" => VisibilityType::Private,
            "PUBLIC" => VisibilityType::Public,
            "SHARED" => VisibilityType::Shared,
            _ => VisibilityType::UnknownVariant(UnknownVisibilityType { name }),
        }
    }
}

impl ::std::str::FromStr for VisibilityType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(s.into())
    }
}

impl Serialize for VisibilityType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.into())
    }
}

impl<'de> Deserialize<'de> for VisibilityType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Ok(String::deserialize(deserializer)?.into())
    }
}

/// <p>Describes VPC configuration information for fleets and image builders.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct VpcConfig {
    /// <p>The identifiers of the security groups for the fleet or image builder.</p>
    #[serde(rename = "SecurityGroupIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_group_ids: Option<Vec<String>>,
    /// <p>The identifiers of the subnets to which a network interface is attached from the fleet instance or image builder instance. Fleet instances use one or more subnets. Image builder instances use one subnet.</p>
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
}

impl AssociateFleetError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<AssociateFleetError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ConcurrentModificationException" => {
                    return RusotoError::Service(AssociateFleetError::ConcurrentModification(
                        err.msg,
                    ))
                }
                "IncompatibleImageException" => {
                    return RusotoError::Service(AssociateFleetError::IncompatibleImage(err.msg))
                }
                "InvalidAccountStatusException" => {
                    return RusotoError::Service(AssociateFleetError::InvalidAccountStatus(err.msg))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(AssociateFleetError::LimitExceeded(err.msg))
                }
                "OperationNotPermittedException" => {
                    return RusotoError::Service(AssociateFleetError::OperationNotPermitted(
                        err.msg,
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(AssociateFleetError::ResourceNotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for AssociateFleetError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            AssociateFleetError::ConcurrentModification(ref cause) => write!(f, "{}", cause),
            AssociateFleetError::IncompatibleImage(ref cause) => write!(f, "{}", cause),
            AssociateFleetError::InvalidAccountStatus(ref cause) => write!(f, "{}", cause),
            AssociateFleetError::LimitExceeded(ref cause) => write!(f, "{}", cause),
            AssociateFleetError::OperationNotPermitted(ref cause) => write!(f, "{}", cause),
            AssociateFleetError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for AssociateFleetError {}
/// Errors returned by BatchAssociateUserStack
#[derive(Debug, PartialEq)]
pub enum BatchAssociateUserStackError {
    /// <p>Indicates an incorrect combination of parameters, or a missing parameter.</p>
    InvalidParameterCombination(String),
    /// <p>The attempted operation is not permitted.</p>
    OperationNotPermitted(String),
}

impl BatchAssociateUserStackError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<BatchAssociateUserStackError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidParameterCombinationException" => {
                    return RusotoError::Service(
                        BatchAssociateUserStackError::InvalidParameterCombination(err.msg),
                    )
                }
                "OperationNotPermittedException" => {
                    return RusotoError::Service(
                        BatchAssociateUserStackError::OperationNotPermitted(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for BatchAssociateUserStackError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            BatchAssociateUserStackError::InvalidParameterCombination(ref cause) => {
                write!(f, "{}", cause)
            }
            BatchAssociateUserStackError::OperationNotPermitted(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for BatchAssociateUserStackError {}
/// Errors returned by BatchDisassociateUserStack
#[derive(Debug, PartialEq)]
pub enum BatchDisassociateUserStackError {
    /// <p>Indicates an incorrect combination of parameters, or a missing parameter.</p>
    InvalidParameterCombination(String),
    /// <p>The attempted operation is not permitted.</p>
    OperationNotPermitted(String),
}

impl BatchDisassociateUserStackError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<BatchDisassociateUserStackError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidParameterCombinationException" => {
                    return RusotoError::Service(
                        BatchDisassociateUserStackError::InvalidParameterCombination(err.msg),
                    )
                }
                "OperationNotPermittedException" => {
                    return RusotoError::Service(
                        BatchDisassociateUserStackError::OperationNotPermitted(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for BatchDisassociateUserStackError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            BatchDisassociateUserStackError::InvalidParameterCombination(ref cause) => {
                write!(f, "{}", cause)
            }
            BatchDisassociateUserStackError::OperationNotPermitted(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for BatchDisassociateUserStackError {}
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
}

impl CopyImageError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CopyImageError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "IncompatibleImageException" => {
                    return RusotoError::Service(CopyImageError::IncompatibleImage(err.msg))
                }
                "InvalidAccountStatusException" => {
                    return RusotoError::Service(CopyImageError::InvalidAccountStatus(err.msg))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(CopyImageError::LimitExceeded(err.msg))
                }
                "ResourceAlreadyExistsException" => {
                    return RusotoError::Service(CopyImageError::ResourceAlreadyExists(err.msg))
                }
                "ResourceNotAvailableException" => {
                    return RusotoError::Service(CopyImageError::ResourceNotAvailable(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(CopyImageError::ResourceNotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for CopyImageError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CopyImageError::IncompatibleImage(ref cause) => write!(f, "{}", cause),
            CopyImageError::InvalidAccountStatus(ref cause) => write!(f, "{}", cause),
            CopyImageError::LimitExceeded(ref cause) => write!(f, "{}", cause),
            CopyImageError::ResourceAlreadyExists(ref cause) => write!(f, "{}", cause),
            CopyImageError::ResourceNotAvailable(ref cause) => write!(f, "{}", cause),
            CopyImageError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CopyImageError {}
/// Errors returned by CreateDirectoryConfig
#[derive(Debug, PartialEq)]
pub enum CreateDirectoryConfigError {
    /// <p>The resource cannot be created because your AWS account is suspended. For assistance, contact AWS Support. </p>
    InvalidAccountStatus(String),
    /// <p>The specified role is invalid.</p>
    InvalidRole(String),
    /// <p>The requested limit exceeds the permitted limit for an account.</p>
    LimitExceeded(String),
    /// <p>The attempted operation is not permitted.</p>
    OperationNotPermitted(String),
    /// <p>The specified resource already exists.</p>
    ResourceAlreadyExists(String),
    /// <p>The specified resource was not found.</p>
    ResourceNotFound(String),
}

impl CreateDirectoryConfigError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateDirectoryConfigError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidAccountStatusException" => {
                    return RusotoError::Service(CreateDirectoryConfigError::InvalidAccountStatus(
                        err.msg,
                    ))
                }
                "InvalidRoleException" => {
                    return RusotoError::Service(CreateDirectoryConfigError::InvalidRole(err.msg))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(CreateDirectoryConfigError::LimitExceeded(err.msg))
                }
                "OperationNotPermittedException" => {
                    return RusotoError::Service(CreateDirectoryConfigError::OperationNotPermitted(
                        err.msg,
                    ))
                }
                "ResourceAlreadyExistsException" => {
                    return RusotoError::Service(CreateDirectoryConfigError::ResourceAlreadyExists(
                        err.msg,
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(CreateDirectoryConfigError::ResourceNotFound(
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
impl fmt::Display for CreateDirectoryConfigError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreateDirectoryConfigError::InvalidAccountStatus(ref cause) => write!(f, "{}", cause),
            CreateDirectoryConfigError::InvalidRole(ref cause) => write!(f, "{}", cause),
            CreateDirectoryConfigError::LimitExceeded(ref cause) => write!(f, "{}", cause),
            CreateDirectoryConfigError::OperationNotPermitted(ref cause) => write!(f, "{}", cause),
            CreateDirectoryConfigError::ResourceAlreadyExists(ref cause) => write!(f, "{}", cause),
            CreateDirectoryConfigError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CreateDirectoryConfigError {}
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
    /// <p>The attempted operation is not permitted.</p>
    OperationNotPermitted(String),
    /// <p>AppStream 2.0 can’t process the request right now because the Describe calls from your AWS account are being throttled by Amazon EC2. Try again later.</p>
    RequestLimitExceeded(String),
    /// <p>The specified resource already exists.</p>
    ResourceAlreadyExists(String),
    /// <p>The specified resource exists and is not in use, but isn't available.</p>
    ResourceNotAvailable(String),
    /// <p>The specified resource was not found.</p>
    ResourceNotFound(String),
}

impl CreateFleetError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateFleetError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ConcurrentModificationException" => {
                    return RusotoError::Service(CreateFleetError::ConcurrentModification(err.msg))
                }
                "IncompatibleImageException" => {
                    return RusotoError::Service(CreateFleetError::IncompatibleImage(err.msg))
                }
                "InvalidAccountStatusException" => {
                    return RusotoError::Service(CreateFleetError::InvalidAccountStatus(err.msg))
                }
                "InvalidParameterCombinationException" => {
                    return RusotoError::Service(CreateFleetError::InvalidParameterCombination(
                        err.msg,
                    ))
                }
                "InvalidRoleException" => {
                    return RusotoError::Service(CreateFleetError::InvalidRole(err.msg))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(CreateFleetError::LimitExceeded(err.msg))
                }
                "OperationNotPermittedException" => {
                    return RusotoError::Service(CreateFleetError::OperationNotPermitted(err.msg))
                }
                "RequestLimitExceededException" => {
                    return RusotoError::Service(CreateFleetError::RequestLimitExceeded(err.msg))
                }
                "ResourceAlreadyExistsException" => {
                    return RusotoError::Service(CreateFleetError::ResourceAlreadyExists(err.msg))
                }
                "ResourceNotAvailableException" => {
                    return RusotoError::Service(CreateFleetError::ResourceNotAvailable(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(CreateFleetError::ResourceNotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for CreateFleetError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreateFleetError::ConcurrentModification(ref cause) => write!(f, "{}", cause),
            CreateFleetError::IncompatibleImage(ref cause) => write!(f, "{}", cause),
            CreateFleetError::InvalidAccountStatus(ref cause) => write!(f, "{}", cause),
            CreateFleetError::InvalidParameterCombination(ref cause) => write!(f, "{}", cause),
            CreateFleetError::InvalidRole(ref cause) => write!(f, "{}", cause),
            CreateFleetError::LimitExceeded(ref cause) => write!(f, "{}", cause),
            CreateFleetError::OperationNotPermitted(ref cause) => write!(f, "{}", cause),
            CreateFleetError::RequestLimitExceeded(ref cause) => write!(f, "{}", cause),
            CreateFleetError::ResourceAlreadyExists(ref cause) => write!(f, "{}", cause),
            CreateFleetError::ResourceNotAvailable(ref cause) => write!(f, "{}", cause),
            CreateFleetError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CreateFleetError {}
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
    /// <p>The attempted operation is not permitted.</p>
    OperationNotPermitted(String),
    /// <p>AppStream 2.0 can’t process the request right now because the Describe calls from your AWS account are being throttled by Amazon EC2. Try again later.</p>
    RequestLimitExceeded(String),
    /// <p>The specified resource already exists.</p>
    ResourceAlreadyExists(String),
    /// <p>The specified resource exists and is not in use, but isn't available.</p>
    ResourceNotAvailable(String),
    /// <p>The specified resource was not found.</p>
    ResourceNotFound(String),
}

impl CreateImageBuilderError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateImageBuilderError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ConcurrentModificationException" => {
                    return RusotoError::Service(CreateImageBuilderError::ConcurrentModification(
                        err.msg,
                    ))
                }
                "IncompatibleImageException" => {
                    return RusotoError::Service(CreateImageBuilderError::IncompatibleImage(
                        err.msg,
                    ))
                }
                "InvalidAccountStatusException" => {
                    return RusotoError::Service(CreateImageBuilderError::InvalidAccountStatus(
                        err.msg,
                    ))
                }
                "InvalidParameterCombinationException" => {
                    return RusotoError::Service(
                        CreateImageBuilderError::InvalidParameterCombination(err.msg),
                    )
                }
                "InvalidRoleException" => {
                    return RusotoError::Service(CreateImageBuilderError::InvalidRole(err.msg))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(CreateImageBuilderError::LimitExceeded(err.msg))
                }
                "OperationNotPermittedException" => {
                    return RusotoError::Service(CreateImageBuilderError::OperationNotPermitted(
                        err.msg,
                    ))
                }
                "RequestLimitExceededException" => {
                    return RusotoError::Service(CreateImageBuilderError::RequestLimitExceeded(
                        err.msg,
                    ))
                }
                "ResourceAlreadyExistsException" => {
                    return RusotoError::Service(CreateImageBuilderError::ResourceAlreadyExists(
                        err.msg,
                    ))
                }
                "ResourceNotAvailableException" => {
                    return RusotoError::Service(CreateImageBuilderError::ResourceNotAvailable(
                        err.msg,
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(CreateImageBuilderError::ResourceNotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for CreateImageBuilderError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreateImageBuilderError::ConcurrentModification(ref cause) => write!(f, "{}", cause),
            CreateImageBuilderError::IncompatibleImage(ref cause) => write!(f, "{}", cause),
            CreateImageBuilderError::InvalidAccountStatus(ref cause) => write!(f, "{}", cause),
            CreateImageBuilderError::InvalidParameterCombination(ref cause) => {
                write!(f, "{}", cause)
            }
            CreateImageBuilderError::InvalidRole(ref cause) => write!(f, "{}", cause),
            CreateImageBuilderError::LimitExceeded(ref cause) => write!(f, "{}", cause),
            CreateImageBuilderError::OperationNotPermitted(ref cause) => write!(f, "{}", cause),
            CreateImageBuilderError::RequestLimitExceeded(ref cause) => write!(f, "{}", cause),
            CreateImageBuilderError::ResourceAlreadyExists(ref cause) => write!(f, "{}", cause),
            CreateImageBuilderError::ResourceNotAvailable(ref cause) => write!(f, "{}", cause),
            CreateImageBuilderError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CreateImageBuilderError {}
/// Errors returned by CreateImageBuilderStreamingURL
#[derive(Debug, PartialEq)]
pub enum CreateImageBuilderStreamingURLError {
    /// <p>The attempted operation is not permitted.</p>
    OperationNotPermitted(String),
    /// <p>The specified resource was not found.</p>
    ResourceNotFound(String),
}

impl CreateImageBuilderStreamingURLError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<CreateImageBuilderStreamingURLError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "OperationNotPermittedException" => {
                    return RusotoError::Service(
                        CreateImageBuilderStreamingURLError::OperationNotPermitted(err.msg),
                    )
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(
                        CreateImageBuilderStreamingURLError::ResourceNotFound(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for CreateImageBuilderStreamingURLError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreateImageBuilderStreamingURLError::OperationNotPermitted(ref cause) => {
                write!(f, "{}", cause)
            }
            CreateImageBuilderStreamingURLError::ResourceNotFound(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for CreateImageBuilderStreamingURLError {}
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
}

impl CreateStackError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateStackError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ConcurrentModificationException" => {
                    return RusotoError::Service(CreateStackError::ConcurrentModification(err.msg))
                }
                "InvalidAccountStatusException" => {
                    return RusotoError::Service(CreateStackError::InvalidAccountStatus(err.msg))
                }
                "InvalidParameterCombinationException" => {
                    return RusotoError::Service(CreateStackError::InvalidParameterCombination(
                        err.msg,
                    ))
                }
                "InvalidRoleException" => {
                    return RusotoError::Service(CreateStackError::InvalidRole(err.msg))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(CreateStackError::LimitExceeded(err.msg))
                }
                "ResourceAlreadyExistsException" => {
                    return RusotoError::Service(CreateStackError::ResourceAlreadyExists(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(CreateStackError::ResourceNotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for CreateStackError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreateStackError::ConcurrentModification(ref cause) => write!(f, "{}", cause),
            CreateStackError::InvalidAccountStatus(ref cause) => write!(f, "{}", cause),
            CreateStackError::InvalidParameterCombination(ref cause) => write!(f, "{}", cause),
            CreateStackError::InvalidRole(ref cause) => write!(f, "{}", cause),
            CreateStackError::LimitExceeded(ref cause) => write!(f, "{}", cause),
            CreateStackError::ResourceAlreadyExists(ref cause) => write!(f, "{}", cause),
            CreateStackError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CreateStackError {}
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
}

impl CreateStreamingURLError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateStreamingURLError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidParameterCombinationException" => {
                    return RusotoError::Service(
                        CreateStreamingURLError::InvalidParameterCombination(err.msg),
                    )
                }
                "OperationNotPermittedException" => {
                    return RusotoError::Service(CreateStreamingURLError::OperationNotPermitted(
                        err.msg,
                    ))
                }
                "ResourceNotAvailableException" => {
                    return RusotoError::Service(CreateStreamingURLError::ResourceNotAvailable(
                        err.msg,
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(CreateStreamingURLError::ResourceNotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for CreateStreamingURLError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreateStreamingURLError::InvalidParameterCombination(ref cause) => {
                write!(f, "{}", cause)
            }
            CreateStreamingURLError::OperationNotPermitted(ref cause) => write!(f, "{}", cause),
            CreateStreamingURLError::ResourceNotAvailable(ref cause) => write!(f, "{}", cause),
            CreateStreamingURLError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CreateStreamingURLError {}
/// Errors returned by CreateUsageReportSubscription
#[derive(Debug, PartialEq)]
pub enum CreateUsageReportSubscriptionError {
    /// <p>The resource cannot be created because your AWS account is suspended. For assistance, contact AWS Support. </p>
    InvalidAccountStatus(String),
    /// <p>The specified role is invalid.</p>
    InvalidRole(String),
    /// <p>The requested limit exceeds the permitted limit for an account.</p>
    LimitExceeded(String),
}

impl CreateUsageReportSubscriptionError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<CreateUsageReportSubscriptionError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidAccountStatusException" => {
                    return RusotoError::Service(
                        CreateUsageReportSubscriptionError::InvalidAccountStatus(err.msg),
                    )
                }
                "InvalidRoleException" => {
                    return RusotoError::Service(CreateUsageReportSubscriptionError::InvalidRole(
                        err.msg,
                    ))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(CreateUsageReportSubscriptionError::LimitExceeded(
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
impl fmt::Display for CreateUsageReportSubscriptionError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreateUsageReportSubscriptionError::InvalidAccountStatus(ref cause) => {
                write!(f, "{}", cause)
            }
            CreateUsageReportSubscriptionError::InvalidRole(ref cause) => write!(f, "{}", cause),
            CreateUsageReportSubscriptionError::LimitExceeded(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CreateUsageReportSubscriptionError {}
/// Errors returned by CreateUser
#[derive(Debug, PartialEq)]
pub enum CreateUserError {
    /// <p>The resource cannot be created because your AWS account is suspended. For assistance, contact AWS Support. </p>
    InvalidAccountStatus(String),
    /// <p>Indicates an incorrect combination of parameters, or a missing parameter.</p>
    InvalidParameterCombination(String),
    /// <p>The requested limit exceeds the permitted limit for an account.</p>
    LimitExceeded(String),
    /// <p>The attempted operation is not permitted.</p>
    OperationNotPermitted(String),
    /// <p>The specified resource already exists.</p>
    ResourceAlreadyExists(String),
}

impl CreateUserError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateUserError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidAccountStatusException" => {
                    return RusotoError::Service(CreateUserError::InvalidAccountStatus(err.msg))
                }
                "InvalidParameterCombinationException" => {
                    return RusotoError::Service(CreateUserError::InvalidParameterCombination(
                        err.msg,
                    ))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(CreateUserError::LimitExceeded(err.msg))
                }
                "OperationNotPermittedException" => {
                    return RusotoError::Service(CreateUserError::OperationNotPermitted(err.msg))
                }
                "ResourceAlreadyExistsException" => {
                    return RusotoError::Service(CreateUserError::ResourceAlreadyExists(err.msg))
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
            CreateUserError::InvalidAccountStatus(ref cause) => write!(f, "{}", cause),
            CreateUserError::InvalidParameterCombination(ref cause) => write!(f, "{}", cause),
            CreateUserError::LimitExceeded(ref cause) => write!(f, "{}", cause),
            CreateUserError::OperationNotPermitted(ref cause) => write!(f, "{}", cause),
            CreateUserError::ResourceAlreadyExists(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CreateUserError {}
/// Errors returned by DeleteDirectoryConfig
#[derive(Debug, PartialEq)]
pub enum DeleteDirectoryConfigError {
    /// <p>The specified resource is in use.</p>
    ResourceInUse(String),
    /// <p>The specified resource was not found.</p>
    ResourceNotFound(String),
}

impl DeleteDirectoryConfigError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteDirectoryConfigError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ResourceInUseException" => {
                    return RusotoError::Service(DeleteDirectoryConfigError::ResourceInUse(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(DeleteDirectoryConfigError::ResourceNotFound(
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
impl fmt::Display for DeleteDirectoryConfigError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteDirectoryConfigError::ResourceInUse(ref cause) => write!(f, "{}", cause),
            DeleteDirectoryConfigError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeleteDirectoryConfigError {}
/// Errors returned by DeleteFleet
#[derive(Debug, PartialEq)]
pub enum DeleteFleetError {
    /// <p>An API error occurred. Wait a few minutes and try again.</p>
    ConcurrentModification(String),
    /// <p>The specified resource is in use.</p>
    ResourceInUse(String),
    /// <p>The specified resource was not found.</p>
    ResourceNotFound(String),
}

impl DeleteFleetError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteFleetError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ConcurrentModificationException" => {
                    return RusotoError::Service(DeleteFleetError::ConcurrentModification(err.msg))
                }
                "ResourceInUseException" => {
                    return RusotoError::Service(DeleteFleetError::ResourceInUse(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(DeleteFleetError::ResourceNotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DeleteFleetError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteFleetError::ConcurrentModification(ref cause) => write!(f, "{}", cause),
            DeleteFleetError::ResourceInUse(ref cause) => write!(f, "{}", cause),
            DeleteFleetError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeleteFleetError {}
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
}

impl DeleteImageError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteImageError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ConcurrentModificationException" => {
                    return RusotoError::Service(DeleteImageError::ConcurrentModification(err.msg))
                }
                "OperationNotPermittedException" => {
                    return RusotoError::Service(DeleteImageError::OperationNotPermitted(err.msg))
                }
                "ResourceInUseException" => {
                    return RusotoError::Service(DeleteImageError::ResourceInUse(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(DeleteImageError::ResourceNotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DeleteImageError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteImageError::ConcurrentModification(ref cause) => write!(f, "{}", cause),
            DeleteImageError::OperationNotPermitted(ref cause) => write!(f, "{}", cause),
            DeleteImageError::ResourceInUse(ref cause) => write!(f, "{}", cause),
            DeleteImageError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeleteImageError {}
/// Errors returned by DeleteImageBuilder
#[derive(Debug, PartialEq)]
pub enum DeleteImageBuilderError {
    /// <p>An API error occurred. Wait a few minutes and try again.</p>
    ConcurrentModification(String),
    /// <p>The attempted operation is not permitted.</p>
    OperationNotPermitted(String),
    /// <p>The specified resource was not found.</p>
    ResourceNotFound(String),
}

impl DeleteImageBuilderError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteImageBuilderError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ConcurrentModificationException" => {
                    return RusotoError::Service(DeleteImageBuilderError::ConcurrentModification(
                        err.msg,
                    ))
                }
                "OperationNotPermittedException" => {
                    return RusotoError::Service(DeleteImageBuilderError::OperationNotPermitted(
                        err.msg,
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(DeleteImageBuilderError::ResourceNotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DeleteImageBuilderError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteImageBuilderError::ConcurrentModification(ref cause) => write!(f, "{}", cause),
            DeleteImageBuilderError::OperationNotPermitted(ref cause) => write!(f, "{}", cause),
            DeleteImageBuilderError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeleteImageBuilderError {}
/// Errors returned by DeleteImagePermissions
#[derive(Debug, PartialEq)]
pub enum DeleteImagePermissionsError {
    /// <p>The specified resource exists and is not in use, but isn't available.</p>
    ResourceNotAvailable(String),
    /// <p>The specified resource was not found.</p>
    ResourceNotFound(String),
}

impl DeleteImagePermissionsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteImagePermissionsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ResourceNotAvailableException" => {
                    return RusotoError::Service(DeleteImagePermissionsError::ResourceNotAvailable(
                        err.msg,
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(DeleteImagePermissionsError::ResourceNotFound(
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
impl fmt::Display for DeleteImagePermissionsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteImagePermissionsError::ResourceNotAvailable(ref cause) => write!(f, "{}", cause),
            DeleteImagePermissionsError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeleteImagePermissionsError {}
/// Errors returned by DeleteStack
#[derive(Debug, PartialEq)]
pub enum DeleteStackError {
    /// <p>An API error occurred. Wait a few minutes and try again.</p>
    ConcurrentModification(String),
    /// <p>The specified resource is in use.</p>
    ResourceInUse(String),
    /// <p>The specified resource was not found.</p>
    ResourceNotFound(String),
}

impl DeleteStackError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteStackError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ConcurrentModificationException" => {
                    return RusotoError::Service(DeleteStackError::ConcurrentModification(err.msg))
                }
                "ResourceInUseException" => {
                    return RusotoError::Service(DeleteStackError::ResourceInUse(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(DeleteStackError::ResourceNotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DeleteStackError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteStackError::ConcurrentModification(ref cause) => write!(f, "{}", cause),
            DeleteStackError::ResourceInUse(ref cause) => write!(f, "{}", cause),
            DeleteStackError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeleteStackError {}
/// Errors returned by DeleteUsageReportSubscription
#[derive(Debug, PartialEq)]
pub enum DeleteUsageReportSubscriptionError {
    /// <p>The resource cannot be created because your AWS account is suspended. For assistance, contact AWS Support. </p>
    InvalidAccountStatus(String),
    /// <p>The specified resource was not found.</p>
    ResourceNotFound(String),
}

impl DeleteUsageReportSubscriptionError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<DeleteUsageReportSubscriptionError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidAccountStatusException" => {
                    return RusotoError::Service(
                        DeleteUsageReportSubscriptionError::InvalidAccountStatus(err.msg),
                    )
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(
                        DeleteUsageReportSubscriptionError::ResourceNotFound(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DeleteUsageReportSubscriptionError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteUsageReportSubscriptionError::InvalidAccountStatus(ref cause) => {
                write!(f, "{}", cause)
            }
            DeleteUsageReportSubscriptionError::ResourceNotFound(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for DeleteUsageReportSubscriptionError {}
/// Errors returned by DeleteUser
#[derive(Debug, PartialEq)]
pub enum DeleteUserError {
    /// <p>The specified resource was not found.</p>
    ResourceNotFound(String),
}

impl DeleteUserError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteUserError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ResourceNotFoundException" => {
                    return RusotoError::Service(DeleteUserError::ResourceNotFound(err.msg))
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
            DeleteUserError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeleteUserError {}
/// Errors returned by DescribeDirectoryConfigs
#[derive(Debug, PartialEq)]
pub enum DescribeDirectoryConfigsError {
    /// <p>The specified resource was not found.</p>
    ResourceNotFound(String),
}

impl DescribeDirectoryConfigsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeDirectoryConfigsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ResourceNotFoundException" => {
                    return RusotoError::Service(DescribeDirectoryConfigsError::ResourceNotFound(
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
impl fmt::Display for DescribeDirectoryConfigsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeDirectoryConfigsError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DescribeDirectoryConfigsError {}
/// Errors returned by DescribeFleets
#[derive(Debug, PartialEq)]
pub enum DescribeFleetsError {
    /// <p>The specified resource was not found.</p>
    ResourceNotFound(String),
}

impl DescribeFleetsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeFleetsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ResourceNotFoundException" => {
                    return RusotoError::Service(DescribeFleetsError::ResourceNotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DescribeFleetsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeFleetsError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DescribeFleetsError {}
/// Errors returned by DescribeImageBuilders
#[derive(Debug, PartialEq)]
pub enum DescribeImageBuildersError {
    /// <p>The specified resource was not found.</p>
    ResourceNotFound(String),
}

impl DescribeImageBuildersError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeImageBuildersError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ResourceNotFoundException" => {
                    return RusotoError::Service(DescribeImageBuildersError::ResourceNotFound(
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
impl fmt::Display for DescribeImageBuildersError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeImageBuildersError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DescribeImageBuildersError {}
/// Errors returned by DescribeImagePermissions
#[derive(Debug, PartialEq)]
pub enum DescribeImagePermissionsError {
    /// <p>The specified resource was not found.</p>
    ResourceNotFound(String),
}

impl DescribeImagePermissionsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeImagePermissionsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ResourceNotFoundException" => {
                    return RusotoError::Service(DescribeImagePermissionsError::ResourceNotFound(
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
impl fmt::Display for DescribeImagePermissionsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeImagePermissionsError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DescribeImagePermissionsError {}
/// Errors returned by DescribeImages
#[derive(Debug, PartialEq)]
pub enum DescribeImagesError {
    /// <p>Indicates an incorrect combination of parameters, or a missing parameter.</p>
    InvalidParameterCombination(String),
    /// <p>The specified resource was not found.</p>
    ResourceNotFound(String),
}

impl DescribeImagesError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeImagesError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidParameterCombinationException" => {
                    return RusotoError::Service(DescribeImagesError::InvalidParameterCombination(
                        err.msg,
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(DescribeImagesError::ResourceNotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DescribeImagesError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeImagesError::InvalidParameterCombination(ref cause) => write!(f, "{}", cause),
            DescribeImagesError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DescribeImagesError {}
/// Errors returned by DescribeSessions
#[derive(Debug, PartialEq)]
pub enum DescribeSessionsError {
    /// <p>Indicates an incorrect combination of parameters, or a missing parameter.</p>
    InvalidParameterCombination(String),
}

impl DescribeSessionsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeSessionsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidParameterCombinationException" => {
                    return RusotoError::Service(
                        DescribeSessionsError::InvalidParameterCombination(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DescribeSessionsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeSessionsError::InvalidParameterCombination(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DescribeSessionsError {}
/// Errors returned by DescribeStacks
#[derive(Debug, PartialEq)]
pub enum DescribeStacksError {
    /// <p>The specified resource was not found.</p>
    ResourceNotFound(String),
}

impl DescribeStacksError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeStacksError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ResourceNotFoundException" => {
                    return RusotoError::Service(DescribeStacksError::ResourceNotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DescribeStacksError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeStacksError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DescribeStacksError {}
/// Errors returned by DescribeUsageReportSubscriptions
#[derive(Debug, PartialEq)]
pub enum DescribeUsageReportSubscriptionsError {
    /// <p>The resource cannot be created because your AWS account is suspended. For assistance, contact AWS Support. </p>
    InvalidAccountStatus(String),
    /// <p>The specified resource was not found.</p>
    ResourceNotFound(String),
}

impl DescribeUsageReportSubscriptionsError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<DescribeUsageReportSubscriptionsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidAccountStatusException" => {
                    return RusotoError::Service(
                        DescribeUsageReportSubscriptionsError::InvalidAccountStatus(err.msg),
                    )
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(
                        DescribeUsageReportSubscriptionsError::ResourceNotFound(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DescribeUsageReportSubscriptionsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeUsageReportSubscriptionsError::InvalidAccountStatus(ref cause) => {
                write!(f, "{}", cause)
            }
            DescribeUsageReportSubscriptionsError::ResourceNotFound(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for DescribeUsageReportSubscriptionsError {}
/// Errors returned by DescribeUserStackAssociations
#[derive(Debug, PartialEq)]
pub enum DescribeUserStackAssociationsError {
    /// <p>Indicates an incorrect combination of parameters, or a missing parameter.</p>
    InvalidParameterCombination(String),
    /// <p>The attempted operation is not permitted.</p>
    OperationNotPermitted(String),
}

impl DescribeUserStackAssociationsError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<DescribeUserStackAssociationsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidParameterCombinationException" => {
                    return RusotoError::Service(
                        DescribeUserStackAssociationsError::InvalidParameterCombination(err.msg),
                    )
                }
                "OperationNotPermittedException" => {
                    return RusotoError::Service(
                        DescribeUserStackAssociationsError::OperationNotPermitted(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DescribeUserStackAssociationsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeUserStackAssociationsError::InvalidParameterCombination(ref cause) => {
                write!(f, "{}", cause)
            }
            DescribeUserStackAssociationsError::OperationNotPermitted(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for DescribeUserStackAssociationsError {}
/// Errors returned by DescribeUsers
#[derive(Debug, PartialEq)]
pub enum DescribeUsersError {
    /// <p>Indicates an incorrect combination of parameters, or a missing parameter.</p>
    InvalidParameterCombination(String),
    /// <p>The specified resource was not found.</p>
    ResourceNotFound(String),
}

impl DescribeUsersError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeUsersError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidParameterCombinationException" => {
                    return RusotoError::Service(DescribeUsersError::InvalidParameterCombination(
                        err.msg,
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(DescribeUsersError::ResourceNotFound(err.msg))
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
            DescribeUsersError::InvalidParameterCombination(ref cause) => write!(f, "{}", cause),
            DescribeUsersError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DescribeUsersError {}
/// Errors returned by DisableUser
#[derive(Debug, PartialEq)]
pub enum DisableUserError {
    /// <p>The specified resource was not found.</p>
    ResourceNotFound(String),
}

impl DisableUserError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DisableUserError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ResourceNotFoundException" => {
                    return RusotoError::Service(DisableUserError::ResourceNotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DisableUserError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DisableUserError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DisableUserError {}
/// Errors returned by DisassociateFleet
#[derive(Debug, PartialEq)]
pub enum DisassociateFleetError {
    /// <p>An API error occurred. Wait a few minutes and try again.</p>
    ConcurrentModification(String),
    /// <p>The attempted operation is not permitted.</p>
    OperationNotPermitted(String),
    /// <p>The specified resource is in use.</p>
    ResourceInUse(String),
    /// <p>The specified resource was not found.</p>
    ResourceNotFound(String),
}

impl DisassociateFleetError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DisassociateFleetError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ConcurrentModificationException" => {
                    return RusotoError::Service(DisassociateFleetError::ConcurrentModification(
                        err.msg,
                    ))
                }
                "OperationNotPermittedException" => {
                    return RusotoError::Service(DisassociateFleetError::OperationNotPermitted(
                        err.msg,
                    ))
                }
                "ResourceInUseException" => {
                    return RusotoError::Service(DisassociateFleetError::ResourceInUse(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(DisassociateFleetError::ResourceNotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DisassociateFleetError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DisassociateFleetError::ConcurrentModification(ref cause) => write!(f, "{}", cause),
            DisassociateFleetError::OperationNotPermitted(ref cause) => write!(f, "{}", cause),
            DisassociateFleetError::ResourceInUse(ref cause) => write!(f, "{}", cause),
            DisassociateFleetError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DisassociateFleetError {}
/// Errors returned by EnableUser
#[derive(Debug, PartialEq)]
pub enum EnableUserError {
    /// <p>The resource cannot be created because your AWS account is suspended. For assistance, contact AWS Support. </p>
    InvalidAccountStatus(String),
    /// <p>The specified resource was not found.</p>
    ResourceNotFound(String),
}

impl EnableUserError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<EnableUserError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidAccountStatusException" => {
                    return RusotoError::Service(EnableUserError::InvalidAccountStatus(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(EnableUserError::ResourceNotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for EnableUserError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            EnableUserError::InvalidAccountStatus(ref cause) => write!(f, "{}", cause),
            EnableUserError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for EnableUserError {}
/// Errors returned by ExpireSession
#[derive(Debug, PartialEq)]
pub enum ExpireSessionError {}

impl ExpireSessionError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ExpireSessionError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ExpireSessionError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {}
    }
}
impl Error for ExpireSessionError {}
/// Errors returned by ListAssociatedFleets
#[derive(Debug, PartialEq)]
pub enum ListAssociatedFleetsError {}

impl ListAssociatedFleetsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListAssociatedFleetsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ListAssociatedFleetsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {}
    }
}
impl Error for ListAssociatedFleetsError {}
/// Errors returned by ListAssociatedStacks
#[derive(Debug, PartialEq)]
pub enum ListAssociatedStacksError {}

impl ListAssociatedStacksError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListAssociatedStacksError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ListAssociatedStacksError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {}
    }
}
impl Error for ListAssociatedStacksError {}
/// Errors returned by ListTagsForResource
#[derive(Debug, PartialEq)]
pub enum ListTagsForResourceError {
    /// <p>The specified resource was not found.</p>
    ResourceNotFound(String),
}

impl ListTagsForResourceError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListTagsForResourceError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ResourceNotFoundException" => {
                    return RusotoError::Service(ListTagsForResourceError::ResourceNotFound(
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
impl fmt::Display for ListTagsForResourceError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListTagsForResourceError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListTagsForResourceError {}
/// Errors returned by StartFleet
#[derive(Debug, PartialEq)]
pub enum StartFleetError {
    /// <p>An API error occurred. Wait a few minutes and try again.</p>
    ConcurrentModification(String),
    /// <p>The resource cannot be created because your AWS account is suspended. For assistance, contact AWS Support. </p>
    InvalidAccountStatus(String),
    /// <p>The specified role is invalid.</p>
    InvalidRole(String),
    /// <p>The requested limit exceeds the permitted limit for an account.</p>
    LimitExceeded(String),
    /// <p>The attempted operation is not permitted.</p>
    OperationNotPermitted(String),
    /// <p>AppStream 2.0 can’t process the request right now because the Describe calls from your AWS account are being throttled by Amazon EC2. Try again later.</p>
    RequestLimitExceeded(String),
    /// <p>The specified resource exists and is not in use, but isn't available.</p>
    ResourceNotAvailable(String),
    /// <p>The specified resource was not found.</p>
    ResourceNotFound(String),
}

impl StartFleetError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<StartFleetError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ConcurrentModificationException" => {
                    return RusotoError::Service(StartFleetError::ConcurrentModification(err.msg))
                }
                "InvalidAccountStatusException" => {
                    return RusotoError::Service(StartFleetError::InvalidAccountStatus(err.msg))
                }
                "InvalidRoleException" => {
                    return RusotoError::Service(StartFleetError::InvalidRole(err.msg))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(StartFleetError::LimitExceeded(err.msg))
                }
                "OperationNotPermittedException" => {
                    return RusotoError::Service(StartFleetError::OperationNotPermitted(err.msg))
                }
                "RequestLimitExceededException" => {
                    return RusotoError::Service(StartFleetError::RequestLimitExceeded(err.msg))
                }
                "ResourceNotAvailableException" => {
                    return RusotoError::Service(StartFleetError::ResourceNotAvailable(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(StartFleetError::ResourceNotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for StartFleetError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            StartFleetError::ConcurrentModification(ref cause) => write!(f, "{}", cause),
            StartFleetError::InvalidAccountStatus(ref cause) => write!(f, "{}", cause),
            StartFleetError::InvalidRole(ref cause) => write!(f, "{}", cause),
            StartFleetError::LimitExceeded(ref cause) => write!(f, "{}", cause),
            StartFleetError::OperationNotPermitted(ref cause) => write!(f, "{}", cause),
            StartFleetError::RequestLimitExceeded(ref cause) => write!(f, "{}", cause),
            StartFleetError::ResourceNotAvailable(ref cause) => write!(f, "{}", cause),
            StartFleetError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for StartFleetError {}
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
}

impl StartImageBuilderError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<StartImageBuilderError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ConcurrentModificationException" => {
                    return RusotoError::Service(StartImageBuilderError::ConcurrentModification(
                        err.msg,
                    ))
                }
                "IncompatibleImageException" => {
                    return RusotoError::Service(StartImageBuilderError::IncompatibleImage(err.msg))
                }
                "InvalidAccountStatusException" => {
                    return RusotoError::Service(StartImageBuilderError::InvalidAccountStatus(
                        err.msg,
                    ))
                }
                "ResourceNotAvailableException" => {
                    return RusotoError::Service(StartImageBuilderError::ResourceNotAvailable(
                        err.msg,
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(StartImageBuilderError::ResourceNotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for StartImageBuilderError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            StartImageBuilderError::ConcurrentModification(ref cause) => write!(f, "{}", cause),
            StartImageBuilderError::IncompatibleImage(ref cause) => write!(f, "{}", cause),
            StartImageBuilderError::InvalidAccountStatus(ref cause) => write!(f, "{}", cause),
            StartImageBuilderError::ResourceNotAvailable(ref cause) => write!(f, "{}", cause),
            StartImageBuilderError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for StartImageBuilderError {}
/// Errors returned by StopFleet
#[derive(Debug, PartialEq)]
pub enum StopFleetError {
    /// <p>An API error occurred. Wait a few minutes and try again.</p>
    ConcurrentModification(String),
    /// <p>The specified resource was not found.</p>
    ResourceNotFound(String),
}

impl StopFleetError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<StopFleetError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ConcurrentModificationException" => {
                    return RusotoError::Service(StopFleetError::ConcurrentModification(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(StopFleetError::ResourceNotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for StopFleetError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            StopFleetError::ConcurrentModification(ref cause) => write!(f, "{}", cause),
            StopFleetError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for StopFleetError {}
/// Errors returned by StopImageBuilder
#[derive(Debug, PartialEq)]
pub enum StopImageBuilderError {
    /// <p>An API error occurred. Wait a few minutes and try again.</p>
    ConcurrentModification(String),
    /// <p>The attempted operation is not permitted.</p>
    OperationNotPermitted(String),
    /// <p>The specified resource was not found.</p>
    ResourceNotFound(String),
}

impl StopImageBuilderError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<StopImageBuilderError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ConcurrentModificationException" => {
                    return RusotoError::Service(StopImageBuilderError::ConcurrentModification(
                        err.msg,
                    ))
                }
                "OperationNotPermittedException" => {
                    return RusotoError::Service(StopImageBuilderError::OperationNotPermitted(
                        err.msg,
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(StopImageBuilderError::ResourceNotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for StopImageBuilderError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            StopImageBuilderError::ConcurrentModification(ref cause) => write!(f, "{}", cause),
            StopImageBuilderError::OperationNotPermitted(ref cause) => write!(f, "{}", cause),
            StopImageBuilderError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for StopImageBuilderError {}
/// Errors returned by TagResource
#[derive(Debug, PartialEq)]
pub enum TagResourceError {
    /// <p>The resource cannot be created because your AWS account is suspended. For assistance, contact AWS Support. </p>
    InvalidAccountStatus(String),
    /// <p>The requested limit exceeds the permitted limit for an account.</p>
    LimitExceeded(String),
    /// <p>The specified resource was not found.</p>
    ResourceNotFound(String),
}

impl TagResourceError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<TagResourceError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidAccountStatusException" => {
                    return RusotoError::Service(TagResourceError::InvalidAccountStatus(err.msg))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(TagResourceError::LimitExceeded(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(TagResourceError::ResourceNotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for TagResourceError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            TagResourceError::InvalidAccountStatus(ref cause) => write!(f, "{}", cause),
            TagResourceError::LimitExceeded(ref cause) => write!(f, "{}", cause),
            TagResourceError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for TagResourceError {}
/// Errors returned by UntagResource
#[derive(Debug, PartialEq)]
pub enum UntagResourceError {
    /// <p>The specified resource was not found.</p>
    ResourceNotFound(String),
}

impl UntagResourceError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UntagResourceError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ResourceNotFoundException" => {
                    return RusotoError::Service(UntagResourceError::ResourceNotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for UntagResourceError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UntagResourceError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for UntagResourceError {}
/// Errors returned by UpdateDirectoryConfig
#[derive(Debug, PartialEq)]
pub enum UpdateDirectoryConfigError {
    /// <p>An API error occurred. Wait a few minutes and try again.</p>
    ConcurrentModification(String),
    /// <p>The specified role is invalid.</p>
    InvalidRole(String),
    /// <p>The attempted operation is not permitted.</p>
    OperationNotPermitted(String),
    /// <p>The specified resource is in use.</p>
    ResourceInUse(String),
    /// <p>The specified resource was not found.</p>
    ResourceNotFound(String),
}

impl UpdateDirectoryConfigError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdateDirectoryConfigError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ConcurrentModificationException" => {
                    return RusotoError::Service(
                        UpdateDirectoryConfigError::ConcurrentModification(err.msg),
                    )
                }
                "InvalidRoleException" => {
                    return RusotoError::Service(UpdateDirectoryConfigError::InvalidRole(err.msg))
                }
                "OperationNotPermittedException" => {
                    return RusotoError::Service(UpdateDirectoryConfigError::OperationNotPermitted(
                        err.msg,
                    ))
                }
                "ResourceInUseException" => {
                    return RusotoError::Service(UpdateDirectoryConfigError::ResourceInUse(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(UpdateDirectoryConfigError::ResourceNotFound(
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
impl fmt::Display for UpdateDirectoryConfigError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UpdateDirectoryConfigError::ConcurrentModification(ref cause) => write!(f, "{}", cause),
            UpdateDirectoryConfigError::InvalidRole(ref cause) => write!(f, "{}", cause),
            UpdateDirectoryConfigError::OperationNotPermitted(ref cause) => write!(f, "{}", cause),
            UpdateDirectoryConfigError::ResourceInUse(ref cause) => write!(f, "{}", cause),
            UpdateDirectoryConfigError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for UpdateDirectoryConfigError {}
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
    /// <p>AppStream 2.0 can’t process the request right now because the Describe calls from your AWS account are being throttled by Amazon EC2. Try again later.</p>
    RequestLimitExceeded(String),
    /// <p>The specified resource is in use.</p>
    ResourceInUse(String),
    /// <p>The specified resource exists and is not in use, but isn't available.</p>
    ResourceNotAvailable(String),
    /// <p>The specified resource was not found.</p>
    ResourceNotFound(String),
}

impl UpdateFleetError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdateFleetError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ConcurrentModificationException" => {
                    return RusotoError::Service(UpdateFleetError::ConcurrentModification(err.msg))
                }
                "IncompatibleImageException" => {
                    return RusotoError::Service(UpdateFleetError::IncompatibleImage(err.msg))
                }
                "InvalidAccountStatusException" => {
                    return RusotoError::Service(UpdateFleetError::InvalidAccountStatus(err.msg))
                }
                "InvalidParameterCombinationException" => {
                    return RusotoError::Service(UpdateFleetError::InvalidParameterCombination(
                        err.msg,
                    ))
                }
                "InvalidRoleException" => {
                    return RusotoError::Service(UpdateFleetError::InvalidRole(err.msg))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(UpdateFleetError::LimitExceeded(err.msg))
                }
                "OperationNotPermittedException" => {
                    return RusotoError::Service(UpdateFleetError::OperationNotPermitted(err.msg))
                }
                "RequestLimitExceededException" => {
                    return RusotoError::Service(UpdateFleetError::RequestLimitExceeded(err.msg))
                }
                "ResourceInUseException" => {
                    return RusotoError::Service(UpdateFleetError::ResourceInUse(err.msg))
                }
                "ResourceNotAvailableException" => {
                    return RusotoError::Service(UpdateFleetError::ResourceNotAvailable(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(UpdateFleetError::ResourceNotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for UpdateFleetError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UpdateFleetError::ConcurrentModification(ref cause) => write!(f, "{}", cause),
            UpdateFleetError::IncompatibleImage(ref cause) => write!(f, "{}", cause),
            UpdateFleetError::InvalidAccountStatus(ref cause) => write!(f, "{}", cause),
            UpdateFleetError::InvalidParameterCombination(ref cause) => write!(f, "{}", cause),
            UpdateFleetError::InvalidRole(ref cause) => write!(f, "{}", cause),
            UpdateFleetError::LimitExceeded(ref cause) => write!(f, "{}", cause),
            UpdateFleetError::OperationNotPermitted(ref cause) => write!(f, "{}", cause),
            UpdateFleetError::RequestLimitExceeded(ref cause) => write!(f, "{}", cause),
            UpdateFleetError::ResourceInUse(ref cause) => write!(f, "{}", cause),
            UpdateFleetError::ResourceNotAvailable(ref cause) => write!(f, "{}", cause),
            UpdateFleetError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for UpdateFleetError {}
/// Errors returned by UpdateImagePermissions
#[derive(Debug, PartialEq)]
pub enum UpdateImagePermissionsError {
    /// <p>The requested limit exceeds the permitted limit for an account.</p>
    LimitExceeded(String),
    /// <p>The specified resource exists and is not in use, but isn't available.</p>
    ResourceNotAvailable(String),
    /// <p>The specified resource was not found.</p>
    ResourceNotFound(String),
}

impl UpdateImagePermissionsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdateImagePermissionsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "LimitExceededException" => {
                    return RusotoError::Service(UpdateImagePermissionsError::LimitExceeded(
                        err.msg,
                    ))
                }
                "ResourceNotAvailableException" => {
                    return RusotoError::Service(UpdateImagePermissionsError::ResourceNotAvailable(
                        err.msg,
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(UpdateImagePermissionsError::ResourceNotFound(
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
impl fmt::Display for UpdateImagePermissionsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UpdateImagePermissionsError::LimitExceeded(ref cause) => write!(f, "{}", cause),
            UpdateImagePermissionsError::ResourceNotAvailable(ref cause) => write!(f, "{}", cause),
            UpdateImagePermissionsError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for UpdateImagePermissionsError {}
/// Errors returned by UpdateStack
#[derive(Debug, PartialEq)]
pub enum UpdateStackError {
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
    /// <p>The specified resource was not found.</p>
    ResourceNotFound(String),
}

impl UpdateStackError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdateStackError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ConcurrentModificationException" => {
                    return RusotoError::Service(UpdateStackError::ConcurrentModification(err.msg))
                }
                "IncompatibleImageException" => {
                    return RusotoError::Service(UpdateStackError::IncompatibleImage(err.msg))
                }
                "InvalidAccountStatusException" => {
                    return RusotoError::Service(UpdateStackError::InvalidAccountStatus(err.msg))
                }
                "InvalidParameterCombinationException" => {
                    return RusotoError::Service(UpdateStackError::InvalidParameterCombination(
                        err.msg,
                    ))
                }
                "InvalidRoleException" => {
                    return RusotoError::Service(UpdateStackError::InvalidRole(err.msg))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(UpdateStackError::LimitExceeded(err.msg))
                }
                "OperationNotPermittedException" => {
                    return RusotoError::Service(UpdateStackError::OperationNotPermitted(err.msg))
                }
                "ResourceInUseException" => {
                    return RusotoError::Service(UpdateStackError::ResourceInUse(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(UpdateStackError::ResourceNotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for UpdateStackError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UpdateStackError::ConcurrentModification(ref cause) => write!(f, "{}", cause),
            UpdateStackError::IncompatibleImage(ref cause) => write!(f, "{}", cause),
            UpdateStackError::InvalidAccountStatus(ref cause) => write!(f, "{}", cause),
            UpdateStackError::InvalidParameterCombination(ref cause) => write!(f, "{}", cause),
            UpdateStackError::InvalidRole(ref cause) => write!(f, "{}", cause),
            UpdateStackError::LimitExceeded(ref cause) => write!(f, "{}", cause),
            UpdateStackError::OperationNotPermitted(ref cause) => write!(f, "{}", cause),
            UpdateStackError::ResourceInUse(ref cause) => write!(f, "{}", cause),
            UpdateStackError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for UpdateStackError {}
/// Trait representing the capabilities of the Amazon AppStream API. Amazon AppStream clients implement this trait.
#[async_trait]
pub trait AppStream {
    /// <p>Associates the specified fleet with the specified stack.</p>
    async fn associate_fleet(
        &self,
        input: AssociateFleetRequest,
    ) -> Result<AssociateFleetResult, RusotoError<AssociateFleetError>>;

    /// <p>Associates the specified users with the specified stacks. Users in a user pool cannot be assigned to stacks with fleets that are joined to an Active Directory domain.</p>
    async fn batch_associate_user_stack(
        &self,
        input: BatchAssociateUserStackRequest,
    ) -> Result<BatchAssociateUserStackResult, RusotoError<BatchAssociateUserStackError>>;

    /// <p>Disassociates the specified users from the specified stacks.</p>
    async fn batch_disassociate_user_stack(
        &self,
        input: BatchDisassociateUserStackRequest,
    ) -> Result<BatchDisassociateUserStackResult, RusotoError<BatchDisassociateUserStackError>>;

    /// <p>Copies the image within the same region or to a new region within the same AWS account. Note that any tags you added to the image will not be copied.</p>
    async fn copy_image(
        &self,
        input: CopyImageRequest,
    ) -> Result<CopyImageResponse, RusotoError<CopyImageError>>;

    /// <p>Creates a Directory Config object in AppStream 2.0. This object includes the configuration information required to join fleets and image builders to Microsoft Active Directory domains.</p>
    async fn create_directory_config(
        &self,
        input: CreateDirectoryConfigRequest,
    ) -> Result<CreateDirectoryConfigResult, RusotoError<CreateDirectoryConfigError>>;

    /// <p>Creates a fleet. A fleet consists of streaming instances that run a specified image.</p>
    async fn create_fleet(
        &self,
        input: CreateFleetRequest,
    ) -> Result<CreateFleetResult, RusotoError<CreateFleetError>>;

    /// <p>Creates an image builder. An image builder is a virtual machine that is used to create an image.</p> <p>The initial state of the builder is <code>PENDING</code>. When it is ready, the state is <code>RUNNING</code>.</p>
    async fn create_image_builder(
        &self,
        input: CreateImageBuilderRequest,
    ) -> Result<CreateImageBuilderResult, RusotoError<CreateImageBuilderError>>;

    /// <p>Creates a URL to start an image builder streaming session.</p>
    async fn create_image_builder_streaming_url(
        &self,
        input: CreateImageBuilderStreamingURLRequest,
    ) -> Result<
        CreateImageBuilderStreamingURLResult,
        RusotoError<CreateImageBuilderStreamingURLError>,
    >;

    /// <p>Creates a stack to start streaming applications to users. A stack consists of an associated fleet, user access policies, and storage configurations. </p>
    async fn create_stack(
        &self,
        input: CreateStackRequest,
    ) -> Result<CreateStackResult, RusotoError<CreateStackError>>;

    /// <p>Creates a temporary URL to start an AppStream 2.0 streaming session for the specified user. A streaming URL enables application streaming to be tested without user setup. </p>
    async fn create_streaming_url(
        &self,
        input: CreateStreamingURLRequest,
    ) -> Result<CreateStreamingURLResult, RusotoError<CreateStreamingURLError>>;

    /// <p>Creates a usage report subscription. Usage reports are generated daily.</p>
    async fn create_usage_report_subscription(
        &self,
    ) -> Result<CreateUsageReportSubscriptionResult, RusotoError<CreateUsageReportSubscriptionError>>;

    /// <p>Creates a new user in the user pool.</p>
    async fn create_user(
        &self,
        input: CreateUserRequest,
    ) -> Result<CreateUserResult, RusotoError<CreateUserError>>;

    /// <p>Deletes the specified Directory Config object from AppStream 2.0. This object includes the information required to join streaming instances to an Active Directory domain.</p>
    async fn delete_directory_config(
        &self,
        input: DeleteDirectoryConfigRequest,
    ) -> Result<DeleteDirectoryConfigResult, RusotoError<DeleteDirectoryConfigError>>;

    /// <p>Deletes the specified fleet.</p>
    async fn delete_fleet(
        &self,
        input: DeleteFleetRequest,
    ) -> Result<DeleteFleetResult, RusotoError<DeleteFleetError>>;

    /// <p>Deletes the specified image. You cannot delete an image when it is in use. After you delete an image, you cannot provision new capacity using the image.</p>
    async fn delete_image(
        &self,
        input: DeleteImageRequest,
    ) -> Result<DeleteImageResult, RusotoError<DeleteImageError>>;

    /// <p>Deletes the specified image builder and releases the capacity.</p>
    async fn delete_image_builder(
        &self,
        input: DeleteImageBuilderRequest,
    ) -> Result<DeleteImageBuilderResult, RusotoError<DeleteImageBuilderError>>;

    /// <p>Deletes permissions for the specified private image. After you delete permissions for an image, AWS accounts to which you previously granted these permissions can no longer use the image.</p>
    async fn delete_image_permissions(
        &self,
        input: DeleteImagePermissionsRequest,
    ) -> Result<DeleteImagePermissionsResult, RusotoError<DeleteImagePermissionsError>>;

    /// <p>Deletes the specified stack. After the stack is deleted, the application streaming environment provided by the stack is no longer available to users. Also, any reservations made for application streaming sessions for the stack are released.</p>
    async fn delete_stack(
        &self,
        input: DeleteStackRequest,
    ) -> Result<DeleteStackResult, RusotoError<DeleteStackError>>;

    /// <p>Disables usage report generation.</p>
    async fn delete_usage_report_subscription(
        &self,
    ) -> Result<DeleteUsageReportSubscriptionResult, RusotoError<DeleteUsageReportSubscriptionError>>;

    /// <p>Deletes a user from the user pool.</p>
    async fn delete_user(
        &self,
        input: DeleteUserRequest,
    ) -> Result<DeleteUserResult, RusotoError<DeleteUserError>>;

    /// <p>Retrieves a list that describes one or more specified Directory Config objects for AppStream 2.0, if the names for these objects are provided. Otherwise, all Directory Config objects in the account are described. These objects include the configuration information required to join fleets and image builders to Microsoft Active Directory domains. </p> <p>Although the response syntax in this topic includes the account password, this password is not returned in the actual response.</p>
    async fn describe_directory_configs(
        &self,
        input: DescribeDirectoryConfigsRequest,
    ) -> Result<DescribeDirectoryConfigsResult, RusotoError<DescribeDirectoryConfigsError>>;

    /// <p>Retrieves a list that describes one or more specified fleets, if the fleet names are provided. Otherwise, all fleets in the account are described.</p>
    async fn describe_fleets(
        &self,
        input: DescribeFleetsRequest,
    ) -> Result<DescribeFleetsResult, RusotoError<DescribeFleetsError>>;

    /// <p>Retrieves a list that describes one or more specified image builders, if the image builder names are provided. Otherwise, all image builders in the account are described.</p>
    async fn describe_image_builders(
        &self,
        input: DescribeImageBuildersRequest,
    ) -> Result<DescribeImageBuildersResult, RusotoError<DescribeImageBuildersError>>;

    /// <p>Retrieves a list that describes the permissions for shared AWS account IDs on a private image that you own. </p>
    async fn describe_image_permissions(
        &self,
        input: DescribeImagePermissionsRequest,
    ) -> Result<DescribeImagePermissionsResult, RusotoError<DescribeImagePermissionsError>>;

    /// <p>Retrieves a list that describes one or more specified images, if the image names or image ARNs are provided. Otherwise, all images in the account are described.</p>
    async fn describe_images(
        &self,
        input: DescribeImagesRequest,
    ) -> Result<DescribeImagesResult, RusotoError<DescribeImagesError>>;

    /// <p>Retrieves a list that describes the streaming sessions for a specified stack and fleet. If a UserId is provided for the stack and fleet, only streaming sessions for that user are described. If an authentication type is not provided, the default is to authenticate users using a streaming URL.</p>
    async fn describe_sessions(
        &self,
        input: DescribeSessionsRequest,
    ) -> Result<DescribeSessionsResult, RusotoError<DescribeSessionsError>>;

    /// <p>Retrieves a list that describes one or more specified stacks, if the stack names are provided. Otherwise, all stacks in the account are described.</p>
    async fn describe_stacks(
        &self,
        input: DescribeStacksRequest,
    ) -> Result<DescribeStacksResult, RusotoError<DescribeStacksError>>;

    /// <p>Retrieves a list that describes one or more usage report subscriptions.</p>
    async fn describe_usage_report_subscriptions(
        &self,
        input: DescribeUsageReportSubscriptionsRequest,
    ) -> Result<
        DescribeUsageReportSubscriptionsResult,
        RusotoError<DescribeUsageReportSubscriptionsError>,
    >;

    /// <p><p>Retrieves a list that describes the UserStackAssociation objects. You must specify either or both of the following:</p> <ul> <li> <p>The stack name</p> </li> <li> <p>The user name (email address of the user associated with the stack) and the authentication type for the user</p> </li> </ul></p>
    async fn describe_user_stack_associations(
        &self,
        input: DescribeUserStackAssociationsRequest,
    ) -> Result<DescribeUserStackAssociationsResult, RusotoError<DescribeUserStackAssociationsError>>;

    /// <p>Retrieves a list that describes one or more specified users in the user pool.</p>
    async fn describe_users(
        &self,
        input: DescribeUsersRequest,
    ) -> Result<DescribeUsersResult, RusotoError<DescribeUsersError>>;

    /// <p>Disables the specified user in the user pool. Users can't sign in to AppStream 2.0 until they are re-enabled. This action does not delete the user. </p>
    async fn disable_user(
        &self,
        input: DisableUserRequest,
    ) -> Result<DisableUserResult, RusotoError<DisableUserError>>;

    /// <p>Disassociates the specified fleet from the specified stack.</p>
    async fn disassociate_fleet(
        &self,
        input: DisassociateFleetRequest,
    ) -> Result<DisassociateFleetResult, RusotoError<DisassociateFleetError>>;

    /// <p>Enables a user in the user pool. After being enabled, users can sign in to AppStream 2.0 and open applications from the stacks to which they are assigned.</p>
    async fn enable_user(
        &self,
        input: EnableUserRequest,
    ) -> Result<EnableUserResult, RusotoError<EnableUserError>>;

    /// <p>Immediately stops the specified streaming session.</p>
    async fn expire_session(
        &self,
        input: ExpireSessionRequest,
    ) -> Result<ExpireSessionResult, RusotoError<ExpireSessionError>>;

    /// <p>Retrieves the name of the fleet that is associated with the specified stack.</p>
    async fn list_associated_fleets(
        &self,
        input: ListAssociatedFleetsRequest,
    ) -> Result<ListAssociatedFleetsResult, RusotoError<ListAssociatedFleetsError>>;

    /// <p>Retrieves the name of the stack with which the specified fleet is associated.</p>
    async fn list_associated_stacks(
        &self,
        input: ListAssociatedStacksRequest,
    ) -> Result<ListAssociatedStacksResult, RusotoError<ListAssociatedStacksError>>;

    /// <p>Retrieves a list of all tags for the specified AppStream 2.0 resource. You can tag AppStream 2.0 image builders, images, fleets, and stacks.</p> <p>For more information about tags, see <a href="https://docs.aws.amazon.com/appstream2/latest/developerguide/tagging-basic.html">Tagging Your Resources</a> in the <i>Amazon AppStream 2.0 Administration Guide</i>.</p>
    async fn list_tags_for_resource(
        &self,
        input: ListTagsForResourceRequest,
    ) -> Result<ListTagsForResourceResponse, RusotoError<ListTagsForResourceError>>;

    /// <p>Starts the specified fleet.</p>
    async fn start_fleet(
        &self,
        input: StartFleetRequest,
    ) -> Result<StartFleetResult, RusotoError<StartFleetError>>;

    /// <p>Starts the specified image builder.</p>
    async fn start_image_builder(
        &self,
        input: StartImageBuilderRequest,
    ) -> Result<StartImageBuilderResult, RusotoError<StartImageBuilderError>>;

    /// <p>Stops the specified fleet.</p>
    async fn stop_fleet(
        &self,
        input: StopFleetRequest,
    ) -> Result<StopFleetResult, RusotoError<StopFleetError>>;

    /// <p>Stops the specified image builder.</p>
    async fn stop_image_builder(
        &self,
        input: StopImageBuilderRequest,
    ) -> Result<StopImageBuilderResult, RusotoError<StopImageBuilderError>>;

    /// <p>Adds or overwrites one or more tags for the specified AppStream 2.0 resource. You can tag AppStream 2.0 image builders, images, fleets, and stacks.</p> <p>Each tag consists of a key and an optional value. If a resource already has a tag with the same key, this operation updates its value.</p> <p>To list the current tags for your resources, use <a>ListTagsForResource</a>. To disassociate tags from your resources, use <a>UntagResource</a>.</p> <p>For more information about tags, see <a href="https://docs.aws.amazon.com/appstream2/latest/developerguide/tagging-basic.html">Tagging Your Resources</a> in the <i>Amazon AppStream 2.0 Administration Guide</i>.</p>
    async fn tag_resource(
        &self,
        input: TagResourceRequest,
    ) -> Result<TagResourceResponse, RusotoError<TagResourceError>>;

    /// <p>Disassociates one or more specified tags from the specified AppStream 2.0 resource.</p> <p>To list the current tags for your resources, use <a>ListTagsForResource</a>.</p> <p>For more information about tags, see <a href="https://docs.aws.amazon.com/appstream2/latest/developerguide/tagging-basic.html">Tagging Your Resources</a> in the <i>Amazon AppStream 2.0 Administration Guide</i>.</p>
    async fn untag_resource(
        &self,
        input: UntagResourceRequest,
    ) -> Result<UntagResourceResponse, RusotoError<UntagResourceError>>;

    /// <p>Updates the specified Directory Config object in AppStream 2.0. This object includes the configuration information required to join fleets and image builders to Microsoft Active Directory domains.</p>
    async fn update_directory_config(
        &self,
        input: UpdateDirectoryConfigRequest,
    ) -> Result<UpdateDirectoryConfigResult, RusotoError<UpdateDirectoryConfigError>>;

    /// <p>Updates the specified fleet.</p> <p>If the fleet is in the <code>STOPPED</code> state, you can update any attribute except the fleet name. If the fleet is in the <code>RUNNING</code> state, you can update the <code>DisplayName</code>, <code>ComputeCapacity</code>, <code>ImageARN</code>, <code>ImageName</code>, <code>IdleDisconnectTimeoutInSeconds</code>, and <code>DisconnectTimeoutInSeconds</code> attributes. If the fleet is in the <code>STARTING</code> or <code>STOPPING</code> state, you can't update it.</p>
    async fn update_fleet(
        &self,
        input: UpdateFleetRequest,
    ) -> Result<UpdateFleetResult, RusotoError<UpdateFleetError>>;

    /// <p>Adds or updates permissions for the specified private image. </p>
    async fn update_image_permissions(
        &self,
        input: UpdateImagePermissionsRequest,
    ) -> Result<UpdateImagePermissionsResult, RusotoError<UpdateImagePermissionsError>>;

    /// <p>Updates the specified fields for the specified stack.</p>
    async fn update_stack(
        &self,
        input: UpdateStackRequest,
    ) -> Result<UpdateStackResult, RusotoError<UpdateStackError>>;
}
/// A client for the Amazon AppStream API.
#[derive(Clone)]
pub struct AppStreamClient {
    client: Client,
    region: region::Region,
}

impl AppStreamClient {
    /// Creates a client backed by the default tokio event loop.
    ///
    /// The client will use the default credentials provider and tls client.
    pub fn new(region: region::Region) -> AppStreamClient {
        AppStreamClient {
            client: Client::shared(),
            region,
        }
    }

    pub fn new_with<P, D>(
        request_dispatcher: D,
        credentials_provider: P,
        region: region::Region,
    ) -> AppStreamClient
    where
        P: ProvideAwsCredentials + Send + Sync + 'static,
        D: DispatchSignedRequest + Send + Sync + 'static,
    {
        AppStreamClient {
            client: Client::new_with(credentials_provider, request_dispatcher),
            region,
        }
    }

    pub fn new_with_client(client: Client, region: region::Region) -> AppStreamClient {
        AppStreamClient { client, region }
    }
}

#[async_trait]
impl AppStream for AppStreamClient {
    /// <p>Associates the specified fleet with the specified stack.</p>
    async fn associate_fleet(
        &self,
        input: AssociateFleetRequest,
    ) -> Result<AssociateFleetResult, RusotoError<AssociateFleetError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "PhotonAdminProxyService.AssociateFleet");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, AssociateFleetError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<AssociateFleetResult, _>()
    }

    /// <p>Associates the specified users with the specified stacks. Users in a user pool cannot be assigned to stacks with fleets that are joined to an Active Directory domain.</p>
    async fn batch_associate_user_stack(
        &self,
        input: BatchAssociateUserStackRequest,
    ) -> Result<BatchAssociateUserStackResult, RusotoError<BatchAssociateUserStackError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "PhotonAdminProxyService.BatchAssociateUserStack",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, BatchAssociateUserStackError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response)
            .deserialize::<BatchAssociateUserStackResult, _>()
    }

    /// <p>Disassociates the specified users from the specified stacks.</p>
    async fn batch_disassociate_user_stack(
        &self,
        input: BatchDisassociateUserStackRequest,
    ) -> Result<BatchDisassociateUserStackResult, RusotoError<BatchDisassociateUserStackError>>
    {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "PhotonAdminProxyService.BatchDisassociateUserStack",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, BatchDisassociateUserStackError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response)
            .deserialize::<BatchDisassociateUserStackResult, _>()
    }

    /// <p>Copies the image within the same region or to a new region within the same AWS account. Note that any tags you added to the image will not be copied.</p>
    async fn copy_image(
        &self,
        input: CopyImageRequest,
    ) -> Result<CopyImageResponse, RusotoError<CopyImageError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "PhotonAdminProxyService.CopyImage");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, CopyImageError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<CopyImageResponse, _>()
    }

    /// <p>Creates a Directory Config object in AppStream 2.0. This object includes the configuration information required to join fleets and image builders to Microsoft Active Directory domains.</p>
    async fn create_directory_config(
        &self,
        input: CreateDirectoryConfigRequest,
    ) -> Result<CreateDirectoryConfigResult, RusotoError<CreateDirectoryConfigError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "PhotonAdminProxyService.CreateDirectoryConfig",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, CreateDirectoryConfigError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<CreateDirectoryConfigResult, _>()
    }

    /// <p>Creates a fleet. A fleet consists of streaming instances that run a specified image.</p>
    async fn create_fleet(
        &self,
        input: CreateFleetRequest,
    ) -> Result<CreateFleetResult, RusotoError<CreateFleetError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "PhotonAdminProxyService.CreateFleet");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, CreateFleetError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<CreateFleetResult, _>()
    }

    /// <p>Creates an image builder. An image builder is a virtual machine that is used to create an image.</p> <p>The initial state of the builder is <code>PENDING</code>. When it is ready, the state is <code>RUNNING</code>.</p>
    async fn create_image_builder(
        &self,
        input: CreateImageBuilderRequest,
    ) -> Result<CreateImageBuilderResult, RusotoError<CreateImageBuilderError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "PhotonAdminProxyService.CreateImageBuilder");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, CreateImageBuilderError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<CreateImageBuilderResult, _>()
    }

    /// <p>Creates a URL to start an image builder streaming session.</p>
    async fn create_image_builder_streaming_url(
        &self,
        input: CreateImageBuilderStreamingURLRequest,
    ) -> Result<
        CreateImageBuilderStreamingURLResult,
        RusotoError<CreateImageBuilderStreamingURLError>,
    > {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "PhotonAdminProxyService.CreateImageBuilderStreamingURL",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, CreateImageBuilderStreamingURLError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response)
            .deserialize::<CreateImageBuilderStreamingURLResult, _>()
    }

    /// <p>Creates a stack to start streaming applications to users. A stack consists of an associated fleet, user access policies, and storage configurations. </p>
    async fn create_stack(
        &self,
        input: CreateStackRequest,
    ) -> Result<CreateStackResult, RusotoError<CreateStackError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "PhotonAdminProxyService.CreateStack");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, CreateStackError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<CreateStackResult, _>()
    }

    /// <p>Creates a temporary URL to start an AppStream 2.0 streaming session for the specified user. A streaming URL enables application streaming to be tested without user setup. </p>
    async fn create_streaming_url(
        &self,
        input: CreateStreamingURLRequest,
    ) -> Result<CreateStreamingURLResult, RusotoError<CreateStreamingURLError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "PhotonAdminProxyService.CreateStreamingURL");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, CreateStreamingURLError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<CreateStreamingURLResult, _>()
    }

    /// <p>Creates a usage report subscription. Usage reports are generated daily.</p>
    async fn create_usage_report_subscription(
        &self,
    ) -> Result<CreateUsageReportSubscriptionResult, RusotoError<CreateUsageReportSubscriptionError>>
    {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "PhotonAdminProxyService.CreateUsageReportSubscription",
        );
        request.set_payload(Some(bytes::Bytes::from_static(b"{}")));

        let response = self
            .sign_and_dispatch(request, CreateUsageReportSubscriptionError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response)
            .deserialize::<CreateUsageReportSubscriptionResult, _>()
    }

    /// <p>Creates a new user in the user pool.</p>
    async fn create_user(
        &self,
        input: CreateUserRequest,
    ) -> Result<CreateUserResult, RusotoError<CreateUserError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "PhotonAdminProxyService.CreateUser");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, CreateUserError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<CreateUserResult, _>()
    }

    /// <p>Deletes the specified Directory Config object from AppStream 2.0. This object includes the information required to join streaming instances to an Active Directory domain.</p>
    async fn delete_directory_config(
        &self,
        input: DeleteDirectoryConfigRequest,
    ) -> Result<DeleteDirectoryConfigResult, RusotoError<DeleteDirectoryConfigError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "PhotonAdminProxyService.DeleteDirectoryConfig",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, DeleteDirectoryConfigError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<DeleteDirectoryConfigResult, _>()
    }

    /// <p>Deletes the specified fleet.</p>
    async fn delete_fleet(
        &self,
        input: DeleteFleetRequest,
    ) -> Result<DeleteFleetResult, RusotoError<DeleteFleetError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "PhotonAdminProxyService.DeleteFleet");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, DeleteFleetError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<DeleteFleetResult, _>()
    }

    /// <p>Deletes the specified image. You cannot delete an image when it is in use. After you delete an image, you cannot provision new capacity using the image.</p>
    async fn delete_image(
        &self,
        input: DeleteImageRequest,
    ) -> Result<DeleteImageResult, RusotoError<DeleteImageError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "PhotonAdminProxyService.DeleteImage");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, DeleteImageError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<DeleteImageResult, _>()
    }

    /// <p>Deletes the specified image builder and releases the capacity.</p>
    async fn delete_image_builder(
        &self,
        input: DeleteImageBuilderRequest,
    ) -> Result<DeleteImageBuilderResult, RusotoError<DeleteImageBuilderError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "PhotonAdminProxyService.DeleteImageBuilder");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, DeleteImageBuilderError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<DeleteImageBuilderResult, _>()
    }

    /// <p>Deletes permissions for the specified private image. After you delete permissions for an image, AWS accounts to which you previously granted these permissions can no longer use the image.</p>
    async fn delete_image_permissions(
        &self,
        input: DeleteImagePermissionsRequest,
    ) -> Result<DeleteImagePermissionsResult, RusotoError<DeleteImagePermissionsError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "PhotonAdminProxyService.DeleteImagePermissions",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, DeleteImagePermissionsError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response)
            .deserialize::<DeleteImagePermissionsResult, _>()
    }

    /// <p>Deletes the specified stack. After the stack is deleted, the application streaming environment provided by the stack is no longer available to users. Also, any reservations made for application streaming sessions for the stack are released.</p>
    async fn delete_stack(
        &self,
        input: DeleteStackRequest,
    ) -> Result<DeleteStackResult, RusotoError<DeleteStackError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "PhotonAdminProxyService.DeleteStack");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, DeleteStackError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<DeleteStackResult, _>()
    }

    /// <p>Disables usage report generation.</p>
    async fn delete_usage_report_subscription(
        &self,
    ) -> Result<DeleteUsageReportSubscriptionResult, RusotoError<DeleteUsageReportSubscriptionError>>
    {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "PhotonAdminProxyService.DeleteUsageReportSubscription",
        );
        request.set_payload(Some(bytes::Bytes::from_static(b"{}")));

        let response = self
            .sign_and_dispatch(request, DeleteUsageReportSubscriptionError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response)
            .deserialize::<DeleteUsageReportSubscriptionResult, _>()
    }

    /// <p>Deletes a user from the user pool.</p>
    async fn delete_user(
        &self,
        input: DeleteUserRequest,
    ) -> Result<DeleteUserResult, RusotoError<DeleteUserError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "PhotonAdminProxyService.DeleteUser");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, DeleteUserError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<DeleteUserResult, _>()
    }

    /// <p>Retrieves a list that describes one or more specified Directory Config objects for AppStream 2.0, if the names for these objects are provided. Otherwise, all Directory Config objects in the account are described. These objects include the configuration information required to join fleets and image builders to Microsoft Active Directory domains. </p> <p>Although the response syntax in this topic includes the account password, this password is not returned in the actual response.</p>
    async fn describe_directory_configs(
        &self,
        input: DescribeDirectoryConfigsRequest,
    ) -> Result<DescribeDirectoryConfigsResult, RusotoError<DescribeDirectoryConfigsError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "PhotonAdminProxyService.DescribeDirectoryConfigs",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, DescribeDirectoryConfigsError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response)
            .deserialize::<DescribeDirectoryConfigsResult, _>()
    }

    /// <p>Retrieves a list that describes one or more specified fleets, if the fleet names are provided. Otherwise, all fleets in the account are described.</p>
    async fn describe_fleets(
        &self,
        input: DescribeFleetsRequest,
    ) -> Result<DescribeFleetsResult, RusotoError<DescribeFleetsError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "PhotonAdminProxyService.DescribeFleets");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, DescribeFleetsError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<DescribeFleetsResult, _>()
    }

    /// <p>Retrieves a list that describes one or more specified image builders, if the image builder names are provided. Otherwise, all image builders in the account are described.</p>
    async fn describe_image_builders(
        &self,
        input: DescribeImageBuildersRequest,
    ) -> Result<DescribeImageBuildersResult, RusotoError<DescribeImageBuildersError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "PhotonAdminProxyService.DescribeImageBuilders",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, DescribeImageBuildersError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<DescribeImageBuildersResult, _>()
    }

    /// <p>Retrieves a list that describes the permissions for shared AWS account IDs on a private image that you own. </p>
    async fn describe_image_permissions(
        &self,
        input: DescribeImagePermissionsRequest,
    ) -> Result<DescribeImagePermissionsResult, RusotoError<DescribeImagePermissionsError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "PhotonAdminProxyService.DescribeImagePermissions",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, DescribeImagePermissionsError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response)
            .deserialize::<DescribeImagePermissionsResult, _>()
    }

    /// <p>Retrieves a list that describes one or more specified images, if the image names or image ARNs are provided. Otherwise, all images in the account are described.</p>
    async fn describe_images(
        &self,
        input: DescribeImagesRequest,
    ) -> Result<DescribeImagesResult, RusotoError<DescribeImagesError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "PhotonAdminProxyService.DescribeImages");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, DescribeImagesError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<DescribeImagesResult, _>()
    }

    /// <p>Retrieves a list that describes the streaming sessions for a specified stack and fleet. If a UserId is provided for the stack and fleet, only streaming sessions for that user are described. If an authentication type is not provided, the default is to authenticate users using a streaming URL.</p>
    async fn describe_sessions(
        &self,
        input: DescribeSessionsRequest,
    ) -> Result<DescribeSessionsResult, RusotoError<DescribeSessionsError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "PhotonAdminProxyService.DescribeSessions");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, DescribeSessionsError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<DescribeSessionsResult, _>()
    }

    /// <p>Retrieves a list that describes one or more specified stacks, if the stack names are provided. Otherwise, all stacks in the account are described.</p>
    async fn describe_stacks(
        &self,
        input: DescribeStacksRequest,
    ) -> Result<DescribeStacksResult, RusotoError<DescribeStacksError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "PhotonAdminProxyService.DescribeStacks");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, DescribeStacksError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<DescribeStacksResult, _>()
    }

    /// <p>Retrieves a list that describes one or more usage report subscriptions.</p>
    async fn describe_usage_report_subscriptions(
        &self,
        input: DescribeUsageReportSubscriptionsRequest,
    ) -> Result<
        DescribeUsageReportSubscriptionsResult,
        RusotoError<DescribeUsageReportSubscriptionsError>,
    > {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "PhotonAdminProxyService.DescribeUsageReportSubscriptions",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(
                request,
                DescribeUsageReportSubscriptionsError::from_response,
            )
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response)
            .deserialize::<DescribeUsageReportSubscriptionsResult, _>()
    }

    /// <p><p>Retrieves a list that describes the UserStackAssociation objects. You must specify either or both of the following:</p> <ul> <li> <p>The stack name</p> </li> <li> <p>The user name (email address of the user associated with the stack) and the authentication type for the user</p> </li> </ul></p>
    async fn describe_user_stack_associations(
        &self,
        input: DescribeUserStackAssociationsRequest,
    ) -> Result<DescribeUserStackAssociationsResult, RusotoError<DescribeUserStackAssociationsError>>
    {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "PhotonAdminProxyService.DescribeUserStackAssociations",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, DescribeUserStackAssociationsError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response)
            .deserialize::<DescribeUserStackAssociationsResult, _>()
    }

    /// <p>Retrieves a list that describes one or more specified users in the user pool.</p>
    async fn describe_users(
        &self,
        input: DescribeUsersRequest,
    ) -> Result<DescribeUsersResult, RusotoError<DescribeUsersError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "PhotonAdminProxyService.DescribeUsers");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, DescribeUsersError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<DescribeUsersResult, _>()
    }

    /// <p>Disables the specified user in the user pool. Users can't sign in to AppStream 2.0 until they are re-enabled. This action does not delete the user. </p>
    async fn disable_user(
        &self,
        input: DisableUserRequest,
    ) -> Result<DisableUserResult, RusotoError<DisableUserError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "PhotonAdminProxyService.DisableUser");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, DisableUserError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<DisableUserResult, _>()
    }

    /// <p>Disassociates the specified fleet from the specified stack.</p>
    async fn disassociate_fleet(
        &self,
        input: DisassociateFleetRequest,
    ) -> Result<DisassociateFleetResult, RusotoError<DisassociateFleetError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "PhotonAdminProxyService.DisassociateFleet");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, DisassociateFleetError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<DisassociateFleetResult, _>()
    }

    /// <p>Enables a user in the user pool. After being enabled, users can sign in to AppStream 2.0 and open applications from the stacks to which they are assigned.</p>
    async fn enable_user(
        &self,
        input: EnableUserRequest,
    ) -> Result<EnableUserResult, RusotoError<EnableUserError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "PhotonAdminProxyService.EnableUser");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, EnableUserError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<EnableUserResult, _>()
    }

    /// <p>Immediately stops the specified streaming session.</p>
    async fn expire_session(
        &self,
        input: ExpireSessionRequest,
    ) -> Result<ExpireSessionResult, RusotoError<ExpireSessionError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "PhotonAdminProxyService.ExpireSession");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, ExpireSessionError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<ExpireSessionResult, _>()
    }

    /// <p>Retrieves the name of the fleet that is associated with the specified stack.</p>
    async fn list_associated_fleets(
        &self,
        input: ListAssociatedFleetsRequest,
    ) -> Result<ListAssociatedFleetsResult, RusotoError<ListAssociatedFleetsError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "PhotonAdminProxyService.ListAssociatedFleets",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, ListAssociatedFleetsError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<ListAssociatedFleetsResult, _>()
    }

    /// <p>Retrieves the name of the stack with which the specified fleet is associated.</p>
    async fn list_associated_stacks(
        &self,
        input: ListAssociatedStacksRequest,
    ) -> Result<ListAssociatedStacksResult, RusotoError<ListAssociatedStacksError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "PhotonAdminProxyService.ListAssociatedStacks",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, ListAssociatedStacksError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<ListAssociatedStacksResult, _>()
    }

    /// <p>Retrieves a list of all tags for the specified AppStream 2.0 resource. You can tag AppStream 2.0 image builders, images, fleets, and stacks.</p> <p>For more information about tags, see <a href="https://docs.aws.amazon.com/appstream2/latest/developerguide/tagging-basic.html">Tagging Your Resources</a> in the <i>Amazon AppStream 2.0 Administration Guide</i>.</p>
    async fn list_tags_for_resource(
        &self,
        input: ListTagsForResourceRequest,
    ) -> Result<ListTagsForResourceResponse, RusotoError<ListTagsForResourceError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "PhotonAdminProxyService.ListTagsForResource",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, ListTagsForResourceError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<ListTagsForResourceResponse, _>()
    }

    /// <p>Starts the specified fleet.</p>
    async fn start_fleet(
        &self,
        input: StartFleetRequest,
    ) -> Result<StartFleetResult, RusotoError<StartFleetError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "PhotonAdminProxyService.StartFleet");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, StartFleetError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<StartFleetResult, _>()
    }

    /// <p>Starts the specified image builder.</p>
    async fn start_image_builder(
        &self,
        input: StartImageBuilderRequest,
    ) -> Result<StartImageBuilderResult, RusotoError<StartImageBuilderError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "PhotonAdminProxyService.StartImageBuilder");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, StartImageBuilderError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<StartImageBuilderResult, _>()
    }

    /// <p>Stops the specified fleet.</p>
    async fn stop_fleet(
        &self,
        input: StopFleetRequest,
    ) -> Result<StopFleetResult, RusotoError<StopFleetError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "PhotonAdminProxyService.StopFleet");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, StopFleetError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<StopFleetResult, _>()
    }

    /// <p>Stops the specified image builder.</p>
    async fn stop_image_builder(
        &self,
        input: StopImageBuilderRequest,
    ) -> Result<StopImageBuilderResult, RusotoError<StopImageBuilderError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "PhotonAdminProxyService.StopImageBuilder");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, StopImageBuilderError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<StopImageBuilderResult, _>()
    }

    /// <p>Adds or overwrites one or more tags for the specified AppStream 2.0 resource. You can tag AppStream 2.0 image builders, images, fleets, and stacks.</p> <p>Each tag consists of a key and an optional value. If a resource already has a tag with the same key, this operation updates its value.</p> <p>To list the current tags for your resources, use <a>ListTagsForResource</a>. To disassociate tags from your resources, use <a>UntagResource</a>.</p> <p>For more information about tags, see <a href="https://docs.aws.amazon.com/appstream2/latest/developerguide/tagging-basic.html">Tagging Your Resources</a> in the <i>Amazon AppStream 2.0 Administration Guide</i>.</p>
    async fn tag_resource(
        &self,
        input: TagResourceRequest,
    ) -> Result<TagResourceResponse, RusotoError<TagResourceError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "PhotonAdminProxyService.TagResource");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, TagResourceError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<TagResourceResponse, _>()
    }

    /// <p>Disassociates one or more specified tags from the specified AppStream 2.0 resource.</p> <p>To list the current tags for your resources, use <a>ListTagsForResource</a>.</p> <p>For more information about tags, see <a href="https://docs.aws.amazon.com/appstream2/latest/developerguide/tagging-basic.html">Tagging Your Resources</a> in the <i>Amazon AppStream 2.0 Administration Guide</i>.</p>
    async fn untag_resource(
        &self,
        input: UntagResourceRequest,
    ) -> Result<UntagResourceResponse, RusotoError<UntagResourceError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "PhotonAdminProxyService.UntagResource");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, UntagResourceError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<UntagResourceResponse, _>()
    }

    /// <p>Updates the specified Directory Config object in AppStream 2.0. This object includes the configuration information required to join fleets and image builders to Microsoft Active Directory domains.</p>
    async fn update_directory_config(
        &self,
        input: UpdateDirectoryConfigRequest,
    ) -> Result<UpdateDirectoryConfigResult, RusotoError<UpdateDirectoryConfigError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "PhotonAdminProxyService.UpdateDirectoryConfig",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, UpdateDirectoryConfigError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<UpdateDirectoryConfigResult, _>()
    }

    /// <p>Updates the specified fleet.</p> <p>If the fleet is in the <code>STOPPED</code> state, you can update any attribute except the fleet name. If the fleet is in the <code>RUNNING</code> state, you can update the <code>DisplayName</code>, <code>ComputeCapacity</code>, <code>ImageARN</code>, <code>ImageName</code>, <code>IdleDisconnectTimeoutInSeconds</code>, and <code>DisconnectTimeoutInSeconds</code> attributes. If the fleet is in the <code>STARTING</code> or <code>STOPPING</code> state, you can't update it.</p>
    async fn update_fleet(
        &self,
        input: UpdateFleetRequest,
    ) -> Result<UpdateFleetResult, RusotoError<UpdateFleetError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "PhotonAdminProxyService.UpdateFleet");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, UpdateFleetError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<UpdateFleetResult, _>()
    }

    /// <p>Adds or updates permissions for the specified private image. </p>
    async fn update_image_permissions(
        &self,
        input: UpdateImagePermissionsRequest,
    ) -> Result<UpdateImagePermissionsResult, RusotoError<UpdateImagePermissionsError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "PhotonAdminProxyService.UpdateImagePermissions",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, UpdateImagePermissionsError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response)
            .deserialize::<UpdateImagePermissionsResult, _>()
    }

    /// <p>Updates the specified fields for the specified stack.</p>
    async fn update_stack(
        &self,
        input: UpdateStackRequest,
    ) -> Result<UpdateStackResult, RusotoError<UpdateStackError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "PhotonAdminProxyService.UpdateStack");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, UpdateStackError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<UpdateStackResult, _>()
    }
}
