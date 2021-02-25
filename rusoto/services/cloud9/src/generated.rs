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

impl Cloud9Client {
    fn new_signed_request(&self, http_method: &str, request_uri: &str) -> SignedRequest {
        let mut request = SignedRequest::new(http_method, "cloud9", &self.region, request_uri);

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

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct UnknownConnectionType {
    name: String,
}

#[allow(non_camel_case_types)]
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
#[non_exhaustive]
pub enum ConnectionType {
    ConnectSsh,
    ConnectSsm,
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
            ConnectionType::ConnectSsh => "CONNECT_SSH".to_string(),
            ConnectionType::ConnectSsm => "CONNECT_SSM".to_string(),
            ConnectionType::UnknownVariant(UnknownConnectionType { name: original }) => original,
        }
    }
}

impl<'a> Into<&'a str> for &'a ConnectionType {
    fn into(self) -> &'a str {
        match self {
            ConnectionType::ConnectSsh => &"CONNECT_SSH",
            ConnectionType::ConnectSsm => &"CONNECT_SSM",
            ConnectionType::UnknownVariant(UnknownConnectionType { name: original }) => original,
        }
    }
}

impl From<&str> for ConnectionType {
    fn from(name: &str) -> Self {
        match name {
            "CONNECT_SSH" => ConnectionType::ConnectSsh,
            "CONNECT_SSM" => ConnectionType::ConnectSsm,
            _ => ConnectionType::UnknownVariant(UnknownConnectionType {
                name: name.to_owned(),
            }),
        }
    }
}

impl From<String> for ConnectionType {
    fn from(name: String) -> Self {
        match &*name {
            "CONNECT_SSH" => ConnectionType::ConnectSsh,
            "CONNECT_SSM" => ConnectionType::ConnectSsm,
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
pub struct CreateEnvironmentEC2Request {
    /// <p>The number of minutes until the running instance is shut down after the environment has last been used.</p>
    #[serde(rename = "automaticStopTimeMinutes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub automatic_stop_time_minutes: Option<i64>,
    /// <p>A unique, case-sensitive string that helps AWS Cloud9 to ensure this operation completes no more than one time.</p> <p>For more information, see <a href="http://docs.aws.amazon.com/AWSEC2/latest/APIReference/Run_Instance_Idempotency.html">Client Tokens</a> in the <i>Amazon EC2 API Reference</i>.</p>
    #[serde(rename = "clientRequestToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_request_token: Option<String>,
    /// <p>The connection type used for connecting to an Amazon EC2 environment.</p>
    #[serde(rename = "connectionType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connection_type: Option<ConnectionType>,
    /// <p>The description of the environment to create.</p>
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The type of instance to connect to the environment (for example, <code>t2.micro</code>).</p>
    #[serde(rename = "instanceType")]
    pub instance_type: String,
    /// <p>The name of the environment to create.</p> <p>This name is visible to other AWS IAM users in the same AWS account.</p>
    #[serde(rename = "name")]
    pub name: String,
    /// <p>The Amazon Resource Name (ARN) of the environment owner. This ARN can be the ARN of any AWS IAM principal. If this value is not specified, the ARN defaults to this environment's creator.</p>
    #[serde(rename = "ownerArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner_arn: Option<String>,
    /// <p>The ID of the subnet in Amazon VPC that AWS Cloud9 will use to communicate with the Amazon EC2 instance.</p>
    #[serde(rename = "subnetId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subnet_id: Option<String>,
    /// <p>An array of key-value pairs that will be associated with the new AWS Cloud9 development environment.</p>
    #[serde(rename = "tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateEnvironmentEC2Result {
    /// <p>The ID of the environment that was created.</p>
    #[serde(rename = "environmentId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub environment_id: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateEnvironmentMembershipRequest {
    /// <p>The ID of the environment that contains the environment member you want to add.</p>
    #[serde(rename = "environmentId")]
    pub environment_id: String,
    /// <p><p>The type of environment member permissions you want to associate with this environment member. Available values include:</p> <ul> <li> <p> <code>read-only</code>: Has read-only access to the environment.</p> </li> <li> <p> <code>read-write</code>: Has read-write access to the environment.</p> </li> </ul></p>
    #[serde(rename = "permissions")]
    pub permissions: MemberPermissions,
    /// <p>The Amazon Resource Name (ARN) of the environment member you want to add.</p>
    #[serde(rename = "userArn")]
    pub user_arn: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateEnvironmentMembershipResult {
    /// <p>Information about the environment member that was added.</p>
    #[serde(rename = "membership")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub membership: Option<EnvironmentMember>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteEnvironmentMembershipRequest {
    /// <p>The ID of the environment to delete the environment member from.</p>
    #[serde(rename = "environmentId")]
    pub environment_id: String,
    /// <p>The Amazon Resource Name (ARN) of the environment member to delete from the environment.</p>
    #[serde(rename = "userArn")]
    pub user_arn: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DeleteEnvironmentMembershipResult {}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteEnvironmentRequest {
    /// <p>The ID of the environment to delete.</p>
    #[serde(rename = "environmentId")]
    pub environment_id: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DeleteEnvironmentResult {}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeEnvironmentMembershipsRequest {
    /// <p>The ID of the environment to get environment member information about.</p>
    #[serde(rename = "environmentId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub environment_id: Option<String>,
    /// <p>The maximum number of environment members to get information about.</p>
    #[serde(rename = "maxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>During a previous call, if there are more than 25 items in the list, only the first 25 items are returned, along with a unique string called a <i>next token</i>. To get the next batch of items in the list, call this operation again, adding the next token to the call. To get all of the items in the list, keep calling this operation with each subsequent next token that is returned, until no more next tokens are returned.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The type of environment member permissions to get information about. Available values include:</p> <ul> <li> <p> <code>owner</code>: Owns the environment.</p> </li> <li> <p> <code>read-only</code>: Has read-only access to the environment.</p> </li> <li> <p> <code>read-write</code>: Has read-write access to the environment.</p> </li> </ul> <p>If no value is specified, information about all environment members are returned.</p>
    #[serde(rename = "permissions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permissions: Option<Vec<Permissions>>,
    /// <p>The Amazon Resource Name (ARN) of an individual environment member to get information about. If no value is specified, information about all environment members are returned.</p>
    #[serde(rename = "userArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_arn: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeEnvironmentMembershipsResult {
    /// <p>Information about the environment members for the environment.</p>
    #[serde(rename = "memberships")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub memberships: Option<Vec<EnvironmentMember>>,
    /// <p>If there are more than 25 items in the list, only the first 25 items are returned, along with a unique string called a <i>next token</i>. To get the next batch of items in the list, call this operation again, adding the next token to the call.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeEnvironmentStatusRequest {
    /// <p>The ID of the environment to get status information about.</p>
    #[serde(rename = "environmentId")]
    pub environment_id: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeEnvironmentStatusResult {
    /// <p>Any informational message about the status of the environment.</p>
    #[serde(rename = "message")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    /// <p><p>The status of the environment. Available values include:</p> <ul> <li> <p> <code>connecting</code>: The environment is connecting.</p> </li> <li> <p> <code>creating</code>: The environment is being created.</p> </li> <li> <p> <code>deleting</code>: The environment is being deleted.</p> </li> <li> <p> <code>error</code>: The environment is in an error state.</p> </li> <li> <p> <code>ready</code>: The environment is ready.</p> </li> <li> <p> <code>stopped</code>: The environment is stopped.</p> </li> <li> <p> <code>stopping</code>: The environment is stopping.</p> </li> </ul></p>
    #[serde(rename = "status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<EnvironmentStatus>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeEnvironmentsRequest {
    /// <p>The IDs of individual environments to get information about.</p>
    #[serde(rename = "environmentIds")]
    pub environment_ids: Vec<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeEnvironmentsResult {
    /// <p>Information about the environments that are returned.</p>
    #[serde(rename = "environments")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub environments: Option<Vec<Environment>>,
}

/// <p>Information about an AWS Cloud9 development environment.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct Environment {
    /// <p>The Amazon Resource Name (ARN) of the environment.</p>
    #[serde(rename = "arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p>The connection type used for connecting to an Amazon EC2 environment.</p>
    #[serde(rename = "connectionType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connection_type: Option<ConnectionType>,
    /// <p>The description for the environment.</p>
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The ID of the environment.</p>
    #[serde(rename = "id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// <p>The state of the environment in its creation or deletion lifecycle.</p>
    #[serde(rename = "lifecycle")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lifecycle: Option<EnvironmentLifecycle>,
    /// <p>The name of the environment.</p>
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The Amazon Resource Name (ARN) of the environment owner.</p>
    #[serde(rename = "ownerArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner_arn: Option<String>,
    /// <p><p>The type of environment. Valid values include the following:</p> <ul> <li> <p> <code>ec2</code>: An Amazon Elastic Compute Cloud (Amazon EC2) instance connects to the environment.</p> </li> <li> <p> <code>ssh</code>: Your own server connects to the environment.</p> </li> </ul></p>
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<EnvironmentType>,
}

/// <p>Information about the current creation or deletion lifecycle state of an AWS Cloud9 development environment.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct EnvironmentLifecycle {
    /// <p>If the environment failed to delete, the Amazon Resource Name (ARN) of the related AWS resource.</p>
    #[serde(rename = "failureResource")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_resource: Option<String>,
    /// <p>Any informational message about the lifecycle state of the environment.</p>
    #[serde(rename = "reason")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    /// <p><p>The current creation or deletion lifecycle state of the environment.</p> <ul> <li> <p> <code>CREATING</code>: The environment is in the process of being created.</p> </li> <li> <p> <code>CREATED</code>: The environment was successfully created.</p> </li> <li> <p> <code>CREATE<em>FAILED</code>: The environment failed to be created.</p> </li> <li> <p> <code>DELETING</code>: The environment is in the process of being deleted.</p> </li> <li> <p> <code>DELETE</em>FAILED</code>: The environment failed to delete.</p> </li> </ul></p>
    #[serde(rename = "status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<EnvironmentLifecycleStatus>,
}

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct UnknownEnvironmentLifecycleStatus {
    name: String,
}

#[allow(non_camel_case_types)]
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
#[non_exhaustive]
pub enum EnvironmentLifecycleStatus {
    Created,
    CreateFailed,
    Creating,
    DeleteFailed,
    Deleting,
    #[doc(hidden)]
    UnknownVariant(UnknownEnvironmentLifecycleStatus),
}

impl Default for EnvironmentLifecycleStatus {
    fn default() -> Self {
        "".into()
    }
}

impl fmt::Display for EnvironmentLifecycleStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.into())
    }
}

impl rusoto_core::param::ToParam for EnvironmentLifecycleStatus {
    fn to_param(&self) -> String {
        self.to_string()
    }
}

impl Into<String> for EnvironmentLifecycleStatus {
    fn into(self) -> String {
        match self {
            EnvironmentLifecycleStatus::Created => "CREATED".to_string(),
            EnvironmentLifecycleStatus::CreateFailed => "CREATE_FAILED".to_string(),
            EnvironmentLifecycleStatus::Creating => "CREATING".to_string(),
            EnvironmentLifecycleStatus::DeleteFailed => "DELETE_FAILED".to_string(),
            EnvironmentLifecycleStatus::Deleting => "DELETING".to_string(),
            EnvironmentLifecycleStatus::UnknownVariant(UnknownEnvironmentLifecycleStatus {
                name: original,
            }) => original,
        }
    }
}

impl<'a> Into<&'a str> for &'a EnvironmentLifecycleStatus {
    fn into(self) -> &'a str {
        match self {
            EnvironmentLifecycleStatus::Created => &"CREATED",
            EnvironmentLifecycleStatus::CreateFailed => &"CREATE_FAILED",
            EnvironmentLifecycleStatus::Creating => &"CREATING",
            EnvironmentLifecycleStatus::DeleteFailed => &"DELETE_FAILED",
            EnvironmentLifecycleStatus::Deleting => &"DELETING",
            EnvironmentLifecycleStatus::UnknownVariant(UnknownEnvironmentLifecycleStatus {
                name: original,
            }) => original,
        }
    }
}

impl From<&str> for EnvironmentLifecycleStatus {
    fn from(name: &str) -> Self {
        match name {
            "CREATED" => EnvironmentLifecycleStatus::Created,
            "CREATE_FAILED" => EnvironmentLifecycleStatus::CreateFailed,
            "CREATING" => EnvironmentLifecycleStatus::Creating,
            "DELETE_FAILED" => EnvironmentLifecycleStatus::DeleteFailed,
            "DELETING" => EnvironmentLifecycleStatus::Deleting,
            _ => EnvironmentLifecycleStatus::UnknownVariant(UnknownEnvironmentLifecycleStatus {
                name: name.to_owned(),
            }),
        }
    }
}

impl From<String> for EnvironmentLifecycleStatus {
    fn from(name: String) -> Self {
        match &*name {
            "CREATED" => EnvironmentLifecycleStatus::Created,
            "CREATE_FAILED" => EnvironmentLifecycleStatus::CreateFailed,
            "CREATING" => EnvironmentLifecycleStatus::Creating,
            "DELETE_FAILED" => EnvironmentLifecycleStatus::DeleteFailed,
            "DELETING" => EnvironmentLifecycleStatus::Deleting,
            _ => EnvironmentLifecycleStatus::UnknownVariant(UnknownEnvironmentLifecycleStatus {
                name,
            }),
        }
    }
}

impl ::std::str::FromStr for EnvironmentLifecycleStatus {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(s.into())
    }
}

#[cfg(any(test, feature = "serialize_structs"))]
impl Serialize for EnvironmentLifecycleStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.into())
    }
}

impl<'de> Deserialize<'de> for EnvironmentLifecycleStatus {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Ok(String::deserialize(deserializer)?.into())
    }
}

/// <p>Information about an environment member for an AWS Cloud9 development environment.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct EnvironmentMember {
    /// <p>The ID of the environment for the environment member.</p>
    #[serde(rename = "environmentId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub environment_id: Option<String>,
    /// <p>The time, expressed in epoch time format, when the environment member last opened the environment.</p>
    #[serde(rename = "lastAccess")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_access: Option<f64>,
    /// <p><p>The type of environment member permissions associated with this environment member. Available values include:</p> <ul> <li> <p> <code>owner</code>: Owns the environment.</p> </li> <li> <p> <code>read-only</code>: Has read-only access to the environment.</p> </li> <li> <p> <code>read-write</code>: Has read-write access to the environment.</p> </li> </ul></p>
    #[serde(rename = "permissions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permissions: Option<Permissions>,
    /// <p>The Amazon Resource Name (ARN) of the environment member.</p>
    #[serde(rename = "userArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_arn: Option<String>,
    /// <p>The user ID in AWS Identity and Access Management (AWS IAM) of the environment member.</p>
    #[serde(rename = "userId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
}

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct UnknownEnvironmentStatus {
    name: String,
}

#[allow(non_camel_case_types)]
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
#[non_exhaustive]
pub enum EnvironmentStatus {
    Connecting,
    Creating,
    Deleting,
    Error,
    Ready,
    Stopped,
    Stopping,
    #[doc(hidden)]
    UnknownVariant(UnknownEnvironmentStatus),
}

impl Default for EnvironmentStatus {
    fn default() -> Self {
        "".into()
    }
}

impl fmt::Display for EnvironmentStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.into())
    }
}

impl rusoto_core::param::ToParam for EnvironmentStatus {
    fn to_param(&self) -> String {
        self.to_string()
    }
}

impl Into<String> for EnvironmentStatus {
    fn into(self) -> String {
        match self {
            EnvironmentStatus::Connecting => "connecting".to_string(),
            EnvironmentStatus::Creating => "creating".to_string(),
            EnvironmentStatus::Deleting => "deleting".to_string(),
            EnvironmentStatus::Error => "error".to_string(),
            EnvironmentStatus::Ready => "ready".to_string(),
            EnvironmentStatus::Stopped => "stopped".to_string(),
            EnvironmentStatus::Stopping => "stopping".to_string(),
            EnvironmentStatus::UnknownVariant(UnknownEnvironmentStatus { name: original }) => {
                original
            }
        }
    }
}

impl<'a> Into<&'a str> for &'a EnvironmentStatus {
    fn into(self) -> &'a str {
        match self {
            EnvironmentStatus::Connecting => &"connecting",
            EnvironmentStatus::Creating => &"creating",
            EnvironmentStatus::Deleting => &"deleting",
            EnvironmentStatus::Error => &"error",
            EnvironmentStatus::Ready => &"ready",
            EnvironmentStatus::Stopped => &"stopped",
            EnvironmentStatus::Stopping => &"stopping",
            EnvironmentStatus::UnknownVariant(UnknownEnvironmentStatus { name: original }) => {
                original
            }
        }
    }
}

impl From<&str> for EnvironmentStatus {
    fn from(name: &str) -> Self {
        match name {
            "connecting" => EnvironmentStatus::Connecting,
            "creating" => EnvironmentStatus::Creating,
            "deleting" => EnvironmentStatus::Deleting,
            "error" => EnvironmentStatus::Error,
            "ready" => EnvironmentStatus::Ready,
            "stopped" => EnvironmentStatus::Stopped,
            "stopping" => EnvironmentStatus::Stopping,
            _ => EnvironmentStatus::UnknownVariant(UnknownEnvironmentStatus {
                name: name.to_owned(),
            }),
        }
    }
}

impl From<String> for EnvironmentStatus {
    fn from(name: String) -> Self {
        match &*name {
            "connecting" => EnvironmentStatus::Connecting,
            "creating" => EnvironmentStatus::Creating,
            "deleting" => EnvironmentStatus::Deleting,
            "error" => EnvironmentStatus::Error,
            "ready" => EnvironmentStatus::Ready,
            "stopped" => EnvironmentStatus::Stopped,
            "stopping" => EnvironmentStatus::Stopping,
            _ => EnvironmentStatus::UnknownVariant(UnknownEnvironmentStatus { name }),
        }
    }
}

impl ::std::str::FromStr for EnvironmentStatus {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(s.into())
    }
}

#[cfg(any(test, feature = "serialize_structs"))]
impl Serialize for EnvironmentStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.into())
    }
}

impl<'de> Deserialize<'de> for EnvironmentStatus {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Ok(String::deserialize(deserializer)?.into())
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct UnknownEnvironmentType {
    name: String,
}

#[allow(non_camel_case_types)]
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
#[non_exhaustive]
pub enum EnvironmentType {
    Ec2,
    Ssh,
    #[doc(hidden)]
    UnknownVariant(UnknownEnvironmentType),
}

impl Default for EnvironmentType {
    fn default() -> Self {
        "".into()
    }
}

impl fmt::Display for EnvironmentType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.into())
    }
}

impl rusoto_core::param::ToParam for EnvironmentType {
    fn to_param(&self) -> String {
        self.to_string()
    }
}

impl Into<String> for EnvironmentType {
    fn into(self) -> String {
        match self {
            EnvironmentType::Ec2 => "ec2".to_string(),
            EnvironmentType::Ssh => "ssh".to_string(),
            EnvironmentType::UnknownVariant(UnknownEnvironmentType { name: original }) => original,
        }
    }
}

impl<'a> Into<&'a str> for &'a EnvironmentType {
    fn into(self) -> &'a str {
        match self {
            EnvironmentType::Ec2 => &"ec2",
            EnvironmentType::Ssh => &"ssh",
            EnvironmentType::UnknownVariant(UnknownEnvironmentType { name: original }) => original,
        }
    }
}

impl From<&str> for EnvironmentType {
    fn from(name: &str) -> Self {
        match name {
            "ec2" => EnvironmentType::Ec2,
            "ssh" => EnvironmentType::Ssh,
            _ => EnvironmentType::UnknownVariant(UnknownEnvironmentType {
                name: name.to_owned(),
            }),
        }
    }
}

impl From<String> for EnvironmentType {
    fn from(name: String) -> Self {
        match &*name {
            "ec2" => EnvironmentType::Ec2,
            "ssh" => EnvironmentType::Ssh,
            _ => EnvironmentType::UnknownVariant(UnknownEnvironmentType { name }),
        }
    }
}

impl ::std::str::FromStr for EnvironmentType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(s.into())
    }
}

#[cfg(any(test, feature = "serialize_structs"))]
impl Serialize for EnvironmentType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.into())
    }
}

impl<'de> Deserialize<'de> for EnvironmentType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Ok(String::deserialize(deserializer)?.into())
    }
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListEnvironmentsRequest {
    /// <p>The maximum number of environments to get identifiers for.</p>
    #[serde(rename = "maxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>During a previous call, if there are more than 25 items in the list, only the first 25 items are returned, along with a unique string called a <i>next token</i>. To get the next batch of items in the list, call this operation again, adding the next token to the call. To get all of the items in the list, keep calling this operation with each subsequent next token that is returned, until no more next tokens are returned.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListEnvironmentsResult {
    /// <p>The list of environment identifiers.</p>
    #[serde(rename = "environmentIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub environment_ids: Option<Vec<String>>,
    /// <p>If there are more than 25 items in the list, only the first 25 items are returned, along with a unique string called a <i>next token</i>. To get the next batch of items in the list, call this operation again, adding the next token to the call.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListTagsForResourceRequest {
    /// <p>The Amazon Resource Name (ARN) of the AWS Cloud9 development environment to get the tags for.</p>
    #[serde(rename = "ResourceARN")]
    pub resource_arn: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListTagsForResourceResponse {
    /// <p>The list of tags associated with the AWS Cloud9 development environment.</p>
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct UnknownMemberPermissions {
    name: String,
}

#[allow(non_camel_case_types)]
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
#[non_exhaustive]
pub enum MemberPermissions {
    ReadOnly,
    ReadWrite,
    #[doc(hidden)]
    UnknownVariant(UnknownMemberPermissions),
}

impl Default for MemberPermissions {
    fn default() -> Self {
        "".into()
    }
}

impl fmt::Display for MemberPermissions {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.into())
    }
}

impl rusoto_core::param::ToParam for MemberPermissions {
    fn to_param(&self) -> String {
        self.to_string()
    }
}

impl Into<String> for MemberPermissions {
    fn into(self) -> String {
        match self {
            MemberPermissions::ReadOnly => "read-only".to_string(),
            MemberPermissions::ReadWrite => "read-write".to_string(),
            MemberPermissions::UnknownVariant(UnknownMemberPermissions { name: original }) => {
                original
            }
        }
    }
}

impl<'a> Into<&'a str> for &'a MemberPermissions {
    fn into(self) -> &'a str {
        match self {
            MemberPermissions::ReadOnly => &"read-only",
            MemberPermissions::ReadWrite => &"read-write",
            MemberPermissions::UnknownVariant(UnknownMemberPermissions { name: original }) => {
                original
            }
        }
    }
}

impl From<&str> for MemberPermissions {
    fn from(name: &str) -> Self {
        match name {
            "read-only" => MemberPermissions::ReadOnly,
            "read-write" => MemberPermissions::ReadWrite,
            _ => MemberPermissions::UnknownVariant(UnknownMemberPermissions {
                name: name.to_owned(),
            }),
        }
    }
}

impl From<String> for MemberPermissions {
    fn from(name: String) -> Self {
        match &*name {
            "read-only" => MemberPermissions::ReadOnly,
            "read-write" => MemberPermissions::ReadWrite,
            _ => MemberPermissions::UnknownVariant(UnknownMemberPermissions { name }),
        }
    }
}

impl ::std::str::FromStr for MemberPermissions {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(s.into())
    }
}

impl Serialize for MemberPermissions {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.into())
    }
}

#[cfg(feature = "deserialize_structs")]
impl<'de> Deserialize<'de> for MemberPermissions {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Ok(String::deserialize(deserializer)?.into())
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct UnknownPermissions {
    name: String,
}

#[allow(non_camel_case_types)]
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
#[non_exhaustive]
pub enum Permissions {
    Owner,
    ReadOnly,
    ReadWrite,
    #[doc(hidden)]
    UnknownVariant(UnknownPermissions),
}

impl Default for Permissions {
    fn default() -> Self {
        "".into()
    }
}

impl fmt::Display for Permissions {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.into())
    }
}

impl rusoto_core::param::ToParam for Permissions {
    fn to_param(&self) -> String {
        self.to_string()
    }
}

impl Into<String> for Permissions {
    fn into(self) -> String {
        match self {
            Permissions::Owner => "owner".to_string(),
            Permissions::ReadOnly => "read-only".to_string(),
            Permissions::ReadWrite => "read-write".to_string(),
            Permissions::UnknownVariant(UnknownPermissions { name: original }) => original,
        }
    }
}

impl<'a> Into<&'a str> for &'a Permissions {
    fn into(self) -> &'a str {
        match self {
            Permissions::Owner => &"owner",
            Permissions::ReadOnly => &"read-only",
            Permissions::ReadWrite => &"read-write",
            Permissions::UnknownVariant(UnknownPermissions { name: original }) => original,
        }
    }
}

impl From<&str> for Permissions {
    fn from(name: &str) -> Self {
        match name {
            "owner" => Permissions::Owner,
            "read-only" => Permissions::ReadOnly,
            "read-write" => Permissions::ReadWrite,
            _ => Permissions::UnknownVariant(UnknownPermissions {
                name: name.to_owned(),
            }),
        }
    }
}

impl From<String> for Permissions {
    fn from(name: String) -> Self {
        match &*name {
            "owner" => Permissions::Owner,
            "read-only" => Permissions::ReadOnly,
            "read-write" => Permissions::ReadWrite,
            _ => Permissions::UnknownVariant(UnknownPermissions { name }),
        }
    }
}

impl ::std::str::FromStr for Permissions {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(s.into())
    }
}

impl Serialize for Permissions {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.into())
    }
}

impl<'de> Deserialize<'de> for Permissions {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Ok(String::deserialize(deserializer)?.into())
    }
}

/// <p>Metadata that is associated with AWS resources. In particular, a name-value pair that can be associated with an AWS Cloud9 development environment. There are two types of tags: <i>user tags</i> and <i>system tags</i>. A user tag is created by the user. A system tag is automatically created by AWS services. A system tag is prefixed with "aws:" and cannot be modified by the user.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct Tag {
    /// <p>The <b>name</b> part of a tag.</p>
    #[serde(rename = "Key")]
    pub key: String,
    /// <p>The <b>value</b> part of a tag.</p>
    #[serde(rename = "Value")]
    pub value: String,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct TagResourceRequest {
    /// <p>The Amazon Resource Name (ARN) of the AWS Cloud9 development environment to add tags to.</p>
    #[serde(rename = "ResourceARN")]
    pub resource_arn: String,
    /// <p>The list of tags to add to the given AWS Cloud9 development environment.</p>
    #[serde(rename = "Tags")]
    pub tags: Vec<Tag>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct TagResourceResponse {}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UntagResourceRequest {
    /// <p>The Amazon Resource Name (ARN) of the AWS Cloud9 development environment to remove tags from.</p>
    #[serde(rename = "ResourceARN")]
    pub resource_arn: String,
    /// <p>The tag names of the tags to remove from the given AWS Cloud9 development environment.</p>
    #[serde(rename = "TagKeys")]
    pub tag_keys: Vec<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UntagResourceResponse {}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UpdateEnvironmentMembershipRequest {
    /// <p>The ID of the environment for the environment member whose settings you want to change.</p>
    #[serde(rename = "environmentId")]
    pub environment_id: String,
    /// <p><p>The replacement type of environment member permissions you want to associate with this environment member. Available values include:</p> <ul> <li> <p> <code>read-only</code>: Has read-only access to the environment.</p> </li> <li> <p> <code>read-write</code>: Has read-write access to the environment.</p> </li> </ul></p>
    #[serde(rename = "permissions")]
    pub permissions: MemberPermissions,
    /// <p>The Amazon Resource Name (ARN) of the environment member whose settings you want to change.</p>
    #[serde(rename = "userArn")]
    pub user_arn: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UpdateEnvironmentMembershipResult {
    /// <p>Information about the environment member whose settings were changed.</p>
    #[serde(rename = "membership")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub membership: Option<EnvironmentMember>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UpdateEnvironmentRequest {
    /// <p>Any new or replacement description for the environment.</p>
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The ID of the environment to change settings.</p>
    #[serde(rename = "environmentId")]
    pub environment_id: String,
    /// <p>A replacement name for the environment.</p>
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UpdateEnvironmentResult {}

/// Errors returned by CreateEnvironmentEC2
#[derive(Debug, PartialEq)]
pub enum CreateEnvironmentEC2Error {
    /// <p>The target request is invalid.</p>
    BadRequest(String),
    /// <p>A conflict occurred.</p>
    Conflict(String),
    /// <p>An access permissions issue occurred.</p>
    Forbidden(String),
    /// <p>An internal server error occurred.</p>
    InternalServerError(String),
    /// <p>A service limit was exceeded.</p>
    LimitExceeded(String),
    /// <p>The target resource cannot be found.</p>
    NotFound(String),
    /// <p>Too many service requests were made over the given time period.</p>
    TooManyRequests(String),
}

impl CreateEnvironmentEC2Error {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateEnvironmentEC2Error> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(CreateEnvironmentEC2Error::BadRequest(err.msg))
                }
                "ConflictException" => {
                    return RusotoError::Service(CreateEnvironmentEC2Error::Conflict(err.msg))
                }
                "ForbiddenException" => {
                    return RusotoError::Service(CreateEnvironmentEC2Error::Forbidden(err.msg))
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(CreateEnvironmentEC2Error::InternalServerError(
                        err.msg,
                    ))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(CreateEnvironmentEC2Error::LimitExceeded(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(CreateEnvironmentEC2Error::NotFound(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(CreateEnvironmentEC2Error::TooManyRequests(
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
impl fmt::Display for CreateEnvironmentEC2Error {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreateEnvironmentEC2Error::BadRequest(ref cause) => write!(f, "{}", cause),
            CreateEnvironmentEC2Error::Conflict(ref cause) => write!(f, "{}", cause),
            CreateEnvironmentEC2Error::Forbidden(ref cause) => write!(f, "{}", cause),
            CreateEnvironmentEC2Error::InternalServerError(ref cause) => write!(f, "{}", cause),
            CreateEnvironmentEC2Error::LimitExceeded(ref cause) => write!(f, "{}", cause),
            CreateEnvironmentEC2Error::NotFound(ref cause) => write!(f, "{}", cause),
            CreateEnvironmentEC2Error::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CreateEnvironmentEC2Error {}
/// Errors returned by CreateEnvironmentMembership
#[derive(Debug, PartialEq)]
pub enum CreateEnvironmentMembershipError {
    /// <p>The target request is invalid.</p>
    BadRequest(String),
    /// <p>A conflict occurred.</p>
    Conflict(String),
    /// <p>An access permissions issue occurred.</p>
    Forbidden(String),
    /// <p>An internal server error occurred.</p>
    InternalServerError(String),
    /// <p>A service limit was exceeded.</p>
    LimitExceeded(String),
    /// <p>The target resource cannot be found.</p>
    NotFound(String),
    /// <p>Too many service requests were made over the given time period.</p>
    TooManyRequests(String),
}

impl CreateEnvironmentMembershipError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<CreateEnvironmentMembershipError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(CreateEnvironmentMembershipError::BadRequest(
                        err.msg,
                    ))
                }
                "ConflictException" => {
                    return RusotoError::Service(CreateEnvironmentMembershipError::Conflict(
                        err.msg,
                    ))
                }
                "ForbiddenException" => {
                    return RusotoError::Service(CreateEnvironmentMembershipError::Forbidden(
                        err.msg,
                    ))
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(
                        CreateEnvironmentMembershipError::InternalServerError(err.msg),
                    )
                }
                "LimitExceededException" => {
                    return RusotoError::Service(CreateEnvironmentMembershipError::LimitExceeded(
                        err.msg,
                    ))
                }
                "NotFoundException" => {
                    return RusotoError::Service(CreateEnvironmentMembershipError::NotFound(
                        err.msg,
                    ))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(CreateEnvironmentMembershipError::TooManyRequests(
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
impl fmt::Display for CreateEnvironmentMembershipError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreateEnvironmentMembershipError::BadRequest(ref cause) => write!(f, "{}", cause),
            CreateEnvironmentMembershipError::Conflict(ref cause) => write!(f, "{}", cause),
            CreateEnvironmentMembershipError::Forbidden(ref cause) => write!(f, "{}", cause),
            CreateEnvironmentMembershipError::InternalServerError(ref cause) => {
                write!(f, "{}", cause)
            }
            CreateEnvironmentMembershipError::LimitExceeded(ref cause) => write!(f, "{}", cause),
            CreateEnvironmentMembershipError::NotFound(ref cause) => write!(f, "{}", cause),
            CreateEnvironmentMembershipError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CreateEnvironmentMembershipError {}
/// Errors returned by DeleteEnvironment
#[derive(Debug, PartialEq)]
pub enum DeleteEnvironmentError {
    /// <p>The target request is invalid.</p>
    BadRequest(String),
    /// <p>A conflict occurred.</p>
    Conflict(String),
    /// <p>An access permissions issue occurred.</p>
    Forbidden(String),
    /// <p>An internal server error occurred.</p>
    InternalServerError(String),
    /// <p>A service limit was exceeded.</p>
    LimitExceeded(String),
    /// <p>The target resource cannot be found.</p>
    NotFound(String),
    /// <p>Too many service requests were made over the given time period.</p>
    TooManyRequests(String),
}

impl DeleteEnvironmentError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteEnvironmentError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(DeleteEnvironmentError::BadRequest(err.msg))
                }
                "ConflictException" => {
                    return RusotoError::Service(DeleteEnvironmentError::Conflict(err.msg))
                }
                "ForbiddenException" => {
                    return RusotoError::Service(DeleteEnvironmentError::Forbidden(err.msg))
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(DeleteEnvironmentError::InternalServerError(
                        err.msg,
                    ))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(DeleteEnvironmentError::LimitExceeded(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(DeleteEnvironmentError::NotFound(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(DeleteEnvironmentError::TooManyRequests(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DeleteEnvironmentError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteEnvironmentError::BadRequest(ref cause) => write!(f, "{}", cause),
            DeleteEnvironmentError::Conflict(ref cause) => write!(f, "{}", cause),
            DeleteEnvironmentError::Forbidden(ref cause) => write!(f, "{}", cause),
            DeleteEnvironmentError::InternalServerError(ref cause) => write!(f, "{}", cause),
            DeleteEnvironmentError::LimitExceeded(ref cause) => write!(f, "{}", cause),
            DeleteEnvironmentError::NotFound(ref cause) => write!(f, "{}", cause),
            DeleteEnvironmentError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeleteEnvironmentError {}
/// Errors returned by DeleteEnvironmentMembership
#[derive(Debug, PartialEq)]
pub enum DeleteEnvironmentMembershipError {
    /// <p>The target request is invalid.</p>
    BadRequest(String),
    /// <p>A conflict occurred.</p>
    Conflict(String),
    /// <p>An access permissions issue occurred.</p>
    Forbidden(String),
    /// <p>An internal server error occurred.</p>
    InternalServerError(String),
    /// <p>A service limit was exceeded.</p>
    LimitExceeded(String),
    /// <p>The target resource cannot be found.</p>
    NotFound(String),
    /// <p>Too many service requests were made over the given time period.</p>
    TooManyRequests(String),
}

impl DeleteEnvironmentMembershipError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<DeleteEnvironmentMembershipError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(DeleteEnvironmentMembershipError::BadRequest(
                        err.msg,
                    ))
                }
                "ConflictException" => {
                    return RusotoError::Service(DeleteEnvironmentMembershipError::Conflict(
                        err.msg,
                    ))
                }
                "ForbiddenException" => {
                    return RusotoError::Service(DeleteEnvironmentMembershipError::Forbidden(
                        err.msg,
                    ))
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(
                        DeleteEnvironmentMembershipError::InternalServerError(err.msg),
                    )
                }
                "LimitExceededException" => {
                    return RusotoError::Service(DeleteEnvironmentMembershipError::LimitExceeded(
                        err.msg,
                    ))
                }
                "NotFoundException" => {
                    return RusotoError::Service(DeleteEnvironmentMembershipError::NotFound(
                        err.msg,
                    ))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(DeleteEnvironmentMembershipError::TooManyRequests(
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
impl fmt::Display for DeleteEnvironmentMembershipError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteEnvironmentMembershipError::BadRequest(ref cause) => write!(f, "{}", cause),
            DeleteEnvironmentMembershipError::Conflict(ref cause) => write!(f, "{}", cause),
            DeleteEnvironmentMembershipError::Forbidden(ref cause) => write!(f, "{}", cause),
            DeleteEnvironmentMembershipError::InternalServerError(ref cause) => {
                write!(f, "{}", cause)
            }
            DeleteEnvironmentMembershipError::LimitExceeded(ref cause) => write!(f, "{}", cause),
            DeleteEnvironmentMembershipError::NotFound(ref cause) => write!(f, "{}", cause),
            DeleteEnvironmentMembershipError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeleteEnvironmentMembershipError {}
/// Errors returned by DescribeEnvironmentMemberships
#[derive(Debug, PartialEq)]
pub enum DescribeEnvironmentMembershipsError {
    /// <p>The target request is invalid.</p>
    BadRequest(String),
    /// <p>A conflict occurred.</p>
    Conflict(String),
    /// <p>An access permissions issue occurred.</p>
    Forbidden(String),
    /// <p>An internal server error occurred.</p>
    InternalServerError(String),
    /// <p>A service limit was exceeded.</p>
    LimitExceeded(String),
    /// <p>The target resource cannot be found.</p>
    NotFound(String),
    /// <p>Too many service requests were made over the given time period.</p>
    TooManyRequests(String),
}

impl DescribeEnvironmentMembershipsError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<DescribeEnvironmentMembershipsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(DescribeEnvironmentMembershipsError::BadRequest(
                        err.msg,
                    ))
                }
                "ConflictException" => {
                    return RusotoError::Service(DescribeEnvironmentMembershipsError::Conflict(
                        err.msg,
                    ))
                }
                "ForbiddenException" => {
                    return RusotoError::Service(DescribeEnvironmentMembershipsError::Forbidden(
                        err.msg,
                    ))
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(
                        DescribeEnvironmentMembershipsError::InternalServerError(err.msg),
                    )
                }
                "LimitExceededException" => {
                    return RusotoError::Service(
                        DescribeEnvironmentMembershipsError::LimitExceeded(err.msg),
                    )
                }
                "NotFoundException" => {
                    return RusotoError::Service(DescribeEnvironmentMembershipsError::NotFound(
                        err.msg,
                    ))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(
                        DescribeEnvironmentMembershipsError::TooManyRequests(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DescribeEnvironmentMembershipsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeEnvironmentMembershipsError::BadRequest(ref cause) => write!(f, "{}", cause),
            DescribeEnvironmentMembershipsError::Conflict(ref cause) => write!(f, "{}", cause),
            DescribeEnvironmentMembershipsError::Forbidden(ref cause) => write!(f, "{}", cause),
            DescribeEnvironmentMembershipsError::InternalServerError(ref cause) => {
                write!(f, "{}", cause)
            }
            DescribeEnvironmentMembershipsError::LimitExceeded(ref cause) => write!(f, "{}", cause),
            DescribeEnvironmentMembershipsError::NotFound(ref cause) => write!(f, "{}", cause),
            DescribeEnvironmentMembershipsError::TooManyRequests(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for DescribeEnvironmentMembershipsError {}
/// Errors returned by DescribeEnvironmentStatus
#[derive(Debug, PartialEq)]
pub enum DescribeEnvironmentStatusError {
    /// <p>The target request is invalid.</p>
    BadRequest(String),
    /// <p>A conflict occurred.</p>
    Conflict(String),
    /// <p>An access permissions issue occurred.</p>
    Forbidden(String),
    /// <p>An internal server error occurred.</p>
    InternalServerError(String),
    /// <p>A service limit was exceeded.</p>
    LimitExceeded(String),
    /// <p>The target resource cannot be found.</p>
    NotFound(String),
    /// <p>Too many service requests were made over the given time period.</p>
    TooManyRequests(String),
}

impl DescribeEnvironmentStatusError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeEnvironmentStatusError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(DescribeEnvironmentStatusError::BadRequest(
                        err.msg,
                    ))
                }
                "ConflictException" => {
                    return RusotoError::Service(DescribeEnvironmentStatusError::Conflict(err.msg))
                }
                "ForbiddenException" => {
                    return RusotoError::Service(DescribeEnvironmentStatusError::Forbidden(err.msg))
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(
                        DescribeEnvironmentStatusError::InternalServerError(err.msg),
                    )
                }
                "LimitExceededException" => {
                    return RusotoError::Service(DescribeEnvironmentStatusError::LimitExceeded(
                        err.msg,
                    ))
                }
                "NotFoundException" => {
                    return RusotoError::Service(DescribeEnvironmentStatusError::NotFound(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(DescribeEnvironmentStatusError::TooManyRequests(
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
impl fmt::Display for DescribeEnvironmentStatusError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeEnvironmentStatusError::BadRequest(ref cause) => write!(f, "{}", cause),
            DescribeEnvironmentStatusError::Conflict(ref cause) => write!(f, "{}", cause),
            DescribeEnvironmentStatusError::Forbidden(ref cause) => write!(f, "{}", cause),
            DescribeEnvironmentStatusError::InternalServerError(ref cause) => {
                write!(f, "{}", cause)
            }
            DescribeEnvironmentStatusError::LimitExceeded(ref cause) => write!(f, "{}", cause),
            DescribeEnvironmentStatusError::NotFound(ref cause) => write!(f, "{}", cause),
            DescribeEnvironmentStatusError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DescribeEnvironmentStatusError {}
/// Errors returned by DescribeEnvironments
#[derive(Debug, PartialEq)]
pub enum DescribeEnvironmentsError {
    /// <p>The target request is invalid.</p>
    BadRequest(String),
    /// <p>A conflict occurred.</p>
    Conflict(String),
    /// <p>An access permissions issue occurred.</p>
    Forbidden(String),
    /// <p>An internal server error occurred.</p>
    InternalServerError(String),
    /// <p>A service limit was exceeded.</p>
    LimitExceeded(String),
    /// <p>The target resource cannot be found.</p>
    NotFound(String),
    /// <p>Too many service requests were made over the given time period.</p>
    TooManyRequests(String),
}

impl DescribeEnvironmentsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeEnvironmentsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(DescribeEnvironmentsError::BadRequest(err.msg))
                }
                "ConflictException" => {
                    return RusotoError::Service(DescribeEnvironmentsError::Conflict(err.msg))
                }
                "ForbiddenException" => {
                    return RusotoError::Service(DescribeEnvironmentsError::Forbidden(err.msg))
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(DescribeEnvironmentsError::InternalServerError(
                        err.msg,
                    ))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(DescribeEnvironmentsError::LimitExceeded(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(DescribeEnvironmentsError::NotFound(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(DescribeEnvironmentsError::TooManyRequests(
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
impl fmt::Display for DescribeEnvironmentsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeEnvironmentsError::BadRequest(ref cause) => write!(f, "{}", cause),
            DescribeEnvironmentsError::Conflict(ref cause) => write!(f, "{}", cause),
            DescribeEnvironmentsError::Forbidden(ref cause) => write!(f, "{}", cause),
            DescribeEnvironmentsError::InternalServerError(ref cause) => write!(f, "{}", cause),
            DescribeEnvironmentsError::LimitExceeded(ref cause) => write!(f, "{}", cause),
            DescribeEnvironmentsError::NotFound(ref cause) => write!(f, "{}", cause),
            DescribeEnvironmentsError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DescribeEnvironmentsError {}
/// Errors returned by ListEnvironments
#[derive(Debug, PartialEq)]
pub enum ListEnvironmentsError {
    /// <p>The target request is invalid.</p>
    BadRequest(String),
    /// <p>A conflict occurred.</p>
    Conflict(String),
    /// <p>An access permissions issue occurred.</p>
    Forbidden(String),
    /// <p>An internal server error occurred.</p>
    InternalServerError(String),
    /// <p>A service limit was exceeded.</p>
    LimitExceeded(String),
    /// <p>The target resource cannot be found.</p>
    NotFound(String),
    /// <p>Too many service requests were made over the given time period.</p>
    TooManyRequests(String),
}

impl ListEnvironmentsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListEnvironmentsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(ListEnvironmentsError::BadRequest(err.msg))
                }
                "ConflictException" => {
                    return RusotoError::Service(ListEnvironmentsError::Conflict(err.msg))
                }
                "ForbiddenException" => {
                    return RusotoError::Service(ListEnvironmentsError::Forbidden(err.msg))
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(ListEnvironmentsError::InternalServerError(
                        err.msg,
                    ))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(ListEnvironmentsError::LimitExceeded(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(ListEnvironmentsError::NotFound(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(ListEnvironmentsError::TooManyRequests(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ListEnvironmentsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListEnvironmentsError::BadRequest(ref cause) => write!(f, "{}", cause),
            ListEnvironmentsError::Conflict(ref cause) => write!(f, "{}", cause),
            ListEnvironmentsError::Forbidden(ref cause) => write!(f, "{}", cause),
            ListEnvironmentsError::InternalServerError(ref cause) => write!(f, "{}", cause),
            ListEnvironmentsError::LimitExceeded(ref cause) => write!(f, "{}", cause),
            ListEnvironmentsError::NotFound(ref cause) => write!(f, "{}", cause),
            ListEnvironmentsError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListEnvironmentsError {}
/// Errors returned by ListTagsForResource
#[derive(Debug, PartialEq)]
pub enum ListTagsForResourceError {
    /// <p>The target request is invalid.</p>
    BadRequest(String),
    /// <p>An internal server error occurred.</p>
    InternalServerError(String),
    /// <p>The target resource cannot be found.</p>
    NotFound(String),
}

impl ListTagsForResourceError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListTagsForResourceError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(ListTagsForResourceError::BadRequest(err.msg))
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(ListTagsForResourceError::InternalServerError(
                        err.msg,
                    ))
                }
                "NotFoundException" => {
                    return RusotoError::Service(ListTagsForResourceError::NotFound(err.msg))
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
            ListTagsForResourceError::BadRequest(ref cause) => write!(f, "{}", cause),
            ListTagsForResourceError::InternalServerError(ref cause) => write!(f, "{}", cause),
            ListTagsForResourceError::NotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListTagsForResourceError {}
/// Errors returned by TagResource
#[derive(Debug, PartialEq)]
pub enum TagResourceError {
    /// <p>The target request is invalid.</p>
    BadRequest(String),
    /// <p>A concurrent access issue occurred.</p>
    ConcurrentAccess(String),
    /// <p>An internal server error occurred.</p>
    InternalServerError(String),
    /// <p>The target resource cannot be found.</p>
    NotFound(String),
}

impl TagResourceError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<TagResourceError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(TagResourceError::BadRequest(err.msg))
                }
                "ConcurrentAccessException" => {
                    return RusotoError::Service(TagResourceError::ConcurrentAccess(err.msg))
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(TagResourceError::InternalServerError(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(TagResourceError::NotFound(err.msg))
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
            TagResourceError::BadRequest(ref cause) => write!(f, "{}", cause),
            TagResourceError::ConcurrentAccess(ref cause) => write!(f, "{}", cause),
            TagResourceError::InternalServerError(ref cause) => write!(f, "{}", cause),
            TagResourceError::NotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for TagResourceError {}
/// Errors returned by UntagResource
#[derive(Debug, PartialEq)]
pub enum UntagResourceError {
    /// <p>The target request is invalid.</p>
    BadRequest(String),
    /// <p>A concurrent access issue occurred.</p>
    ConcurrentAccess(String),
    /// <p>An internal server error occurred.</p>
    InternalServerError(String),
    /// <p>The target resource cannot be found.</p>
    NotFound(String),
}

impl UntagResourceError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UntagResourceError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(UntagResourceError::BadRequest(err.msg))
                }
                "ConcurrentAccessException" => {
                    return RusotoError::Service(UntagResourceError::ConcurrentAccess(err.msg))
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(UntagResourceError::InternalServerError(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(UntagResourceError::NotFound(err.msg))
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
            UntagResourceError::BadRequest(ref cause) => write!(f, "{}", cause),
            UntagResourceError::ConcurrentAccess(ref cause) => write!(f, "{}", cause),
            UntagResourceError::InternalServerError(ref cause) => write!(f, "{}", cause),
            UntagResourceError::NotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for UntagResourceError {}
/// Errors returned by UpdateEnvironment
#[derive(Debug, PartialEq)]
pub enum UpdateEnvironmentError {
    /// <p>The target request is invalid.</p>
    BadRequest(String),
    /// <p>A conflict occurred.</p>
    Conflict(String),
    /// <p>An access permissions issue occurred.</p>
    Forbidden(String),
    /// <p>An internal server error occurred.</p>
    InternalServerError(String),
    /// <p>A service limit was exceeded.</p>
    LimitExceeded(String),
    /// <p>The target resource cannot be found.</p>
    NotFound(String),
    /// <p>Too many service requests were made over the given time period.</p>
    TooManyRequests(String),
}

impl UpdateEnvironmentError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdateEnvironmentError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(UpdateEnvironmentError::BadRequest(err.msg))
                }
                "ConflictException" => {
                    return RusotoError::Service(UpdateEnvironmentError::Conflict(err.msg))
                }
                "ForbiddenException" => {
                    return RusotoError::Service(UpdateEnvironmentError::Forbidden(err.msg))
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(UpdateEnvironmentError::InternalServerError(
                        err.msg,
                    ))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(UpdateEnvironmentError::LimitExceeded(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(UpdateEnvironmentError::NotFound(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(UpdateEnvironmentError::TooManyRequests(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for UpdateEnvironmentError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UpdateEnvironmentError::BadRequest(ref cause) => write!(f, "{}", cause),
            UpdateEnvironmentError::Conflict(ref cause) => write!(f, "{}", cause),
            UpdateEnvironmentError::Forbidden(ref cause) => write!(f, "{}", cause),
            UpdateEnvironmentError::InternalServerError(ref cause) => write!(f, "{}", cause),
            UpdateEnvironmentError::LimitExceeded(ref cause) => write!(f, "{}", cause),
            UpdateEnvironmentError::NotFound(ref cause) => write!(f, "{}", cause),
            UpdateEnvironmentError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for UpdateEnvironmentError {}
/// Errors returned by UpdateEnvironmentMembership
#[derive(Debug, PartialEq)]
pub enum UpdateEnvironmentMembershipError {
    /// <p>The target request is invalid.</p>
    BadRequest(String),
    /// <p>A conflict occurred.</p>
    Conflict(String),
    /// <p>An access permissions issue occurred.</p>
    Forbidden(String),
    /// <p>An internal server error occurred.</p>
    InternalServerError(String),
    /// <p>A service limit was exceeded.</p>
    LimitExceeded(String),
    /// <p>The target resource cannot be found.</p>
    NotFound(String),
    /// <p>Too many service requests were made over the given time period.</p>
    TooManyRequests(String),
}

impl UpdateEnvironmentMembershipError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<UpdateEnvironmentMembershipError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(UpdateEnvironmentMembershipError::BadRequest(
                        err.msg,
                    ))
                }
                "ConflictException" => {
                    return RusotoError::Service(UpdateEnvironmentMembershipError::Conflict(
                        err.msg,
                    ))
                }
                "ForbiddenException" => {
                    return RusotoError::Service(UpdateEnvironmentMembershipError::Forbidden(
                        err.msg,
                    ))
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(
                        UpdateEnvironmentMembershipError::InternalServerError(err.msg),
                    )
                }
                "LimitExceededException" => {
                    return RusotoError::Service(UpdateEnvironmentMembershipError::LimitExceeded(
                        err.msg,
                    ))
                }
                "NotFoundException" => {
                    return RusotoError::Service(UpdateEnvironmentMembershipError::NotFound(
                        err.msg,
                    ))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(UpdateEnvironmentMembershipError::TooManyRequests(
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
impl fmt::Display for UpdateEnvironmentMembershipError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UpdateEnvironmentMembershipError::BadRequest(ref cause) => write!(f, "{}", cause),
            UpdateEnvironmentMembershipError::Conflict(ref cause) => write!(f, "{}", cause),
            UpdateEnvironmentMembershipError::Forbidden(ref cause) => write!(f, "{}", cause),
            UpdateEnvironmentMembershipError::InternalServerError(ref cause) => {
                write!(f, "{}", cause)
            }
            UpdateEnvironmentMembershipError::LimitExceeded(ref cause) => write!(f, "{}", cause),
            UpdateEnvironmentMembershipError::NotFound(ref cause) => write!(f, "{}", cause),
            UpdateEnvironmentMembershipError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for UpdateEnvironmentMembershipError {}
/// Trait representing the capabilities of the AWS Cloud9 API. AWS Cloud9 clients implement this trait.
#[async_trait]
pub trait Cloud9 {
    /// <p>Creates an AWS Cloud9 development environment, launches an Amazon Elastic Compute Cloud (Amazon EC2) instance, and then connects from the instance to the environment.</p>
    async fn create_environment_ec2(
        &self,
        input: CreateEnvironmentEC2Request,
    ) -> Result<CreateEnvironmentEC2Result, RusotoError<CreateEnvironmentEC2Error>>;

    /// <p>Adds an environment member to an AWS Cloud9 development environment.</p>
    async fn create_environment_membership(
        &self,
        input: CreateEnvironmentMembershipRequest,
    ) -> Result<CreateEnvironmentMembershipResult, RusotoError<CreateEnvironmentMembershipError>>;

    /// <p>Deletes an AWS Cloud9 development environment. If an Amazon EC2 instance is connected to the environment, also terminates the instance.</p>
    async fn delete_environment(
        &self,
        input: DeleteEnvironmentRequest,
    ) -> Result<DeleteEnvironmentResult, RusotoError<DeleteEnvironmentError>>;

    /// <p>Deletes an environment member from an AWS Cloud9 development environment.</p>
    async fn delete_environment_membership(
        &self,
        input: DeleteEnvironmentMembershipRequest,
    ) -> Result<DeleteEnvironmentMembershipResult, RusotoError<DeleteEnvironmentMembershipError>>;

    /// <p>Gets information about environment members for an AWS Cloud9 development environment.</p>
    async fn describe_environment_memberships(
        &self,
        input: DescribeEnvironmentMembershipsRequest,
    ) -> Result<
        DescribeEnvironmentMembershipsResult,
        RusotoError<DescribeEnvironmentMembershipsError>,
    >;

    /// <p>Gets status information for an AWS Cloud9 development environment.</p>
    async fn describe_environment_status(
        &self,
        input: DescribeEnvironmentStatusRequest,
    ) -> Result<DescribeEnvironmentStatusResult, RusotoError<DescribeEnvironmentStatusError>>;

    /// <p>Gets information about AWS Cloud9 development environments.</p>
    async fn describe_environments(
        &self,
        input: DescribeEnvironmentsRequest,
    ) -> Result<DescribeEnvironmentsResult, RusotoError<DescribeEnvironmentsError>>;

    /// <p>Gets a list of AWS Cloud9 development environment identifiers.</p>
    async fn list_environments(
        &self,
        input: ListEnvironmentsRequest,
    ) -> Result<ListEnvironmentsResult, RusotoError<ListEnvironmentsError>>;

    /// <p>Gets a list of the tags associated with an AWS Cloud9 development environment.</p>
    async fn list_tags_for_resource(
        &self,
        input: ListTagsForResourceRequest,
    ) -> Result<ListTagsForResourceResponse, RusotoError<ListTagsForResourceError>>;

    /// <p><p>Adds tags to an AWS Cloud9 development environment.</p> <important> <p>Tags that you add to an AWS Cloud9 environment by using this method will NOT be automatically propagated to underlying resources.</p> </important></p>
    async fn tag_resource(
        &self,
        input: TagResourceRequest,
    ) -> Result<TagResourceResponse, RusotoError<TagResourceError>>;

    /// <p>Removes tags from an AWS Cloud9 development environment.</p>
    async fn untag_resource(
        &self,
        input: UntagResourceRequest,
    ) -> Result<UntagResourceResponse, RusotoError<UntagResourceError>>;

    /// <p>Changes the settings of an existing AWS Cloud9 development environment.</p>
    async fn update_environment(
        &self,
        input: UpdateEnvironmentRequest,
    ) -> Result<UpdateEnvironmentResult, RusotoError<UpdateEnvironmentError>>;

    /// <p>Changes the settings of an existing environment member for an AWS Cloud9 development environment.</p>
    async fn update_environment_membership(
        &self,
        input: UpdateEnvironmentMembershipRequest,
    ) -> Result<UpdateEnvironmentMembershipResult, RusotoError<UpdateEnvironmentMembershipError>>;
}
/// A client for the AWS Cloud9 API.
#[derive(Clone)]
pub struct Cloud9Client {
    client: Client,
    region: region::Region,
}

impl Cloud9Client {
    /// Creates a client backed by the default tokio event loop.
    ///
    /// The client will use the default credentials provider and tls client.
    pub fn new(region: region::Region) -> Cloud9Client {
        Cloud9Client {
            client: Client::shared(),
            region,
        }
    }

    pub fn new_with<P, D>(
        request_dispatcher: D,
        credentials_provider: P,
        region: region::Region,
    ) -> Cloud9Client
    where
        P: ProvideAwsCredentials + Send + Sync + 'static,
        D: DispatchSignedRequest + Send + Sync + 'static,
    {
        Cloud9Client {
            client: Client::new_with(credentials_provider, request_dispatcher),
            region,
        }
    }

    pub fn new_with_client(client: Client, region: region::Region) -> Cloud9Client {
        Cloud9Client { client, region }
    }
}

#[async_trait]
impl Cloud9 for Cloud9Client {
    /// <p>Creates an AWS Cloud9 development environment, launches an Amazon Elastic Compute Cloud (Amazon EC2) instance, and then connects from the instance to the environment.</p>
    async fn create_environment_ec2(
        &self,
        input: CreateEnvironmentEC2Request,
    ) -> Result<CreateEnvironmentEC2Result, RusotoError<CreateEnvironmentEC2Error>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "AWSCloud9WorkspaceManagementService.CreateEnvironmentEC2",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, CreateEnvironmentEC2Error::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<CreateEnvironmentEC2Result, _>()
    }

    /// <p>Adds an environment member to an AWS Cloud9 development environment.</p>
    async fn create_environment_membership(
        &self,
        input: CreateEnvironmentMembershipRequest,
    ) -> Result<CreateEnvironmentMembershipResult, RusotoError<CreateEnvironmentMembershipError>>
    {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "AWSCloud9WorkspaceManagementService.CreateEnvironmentMembership",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, CreateEnvironmentMembershipError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response)
            .deserialize::<CreateEnvironmentMembershipResult, _>()
    }

    /// <p>Deletes an AWS Cloud9 development environment. If an Amazon EC2 instance is connected to the environment, also terminates the instance.</p>
    async fn delete_environment(
        &self,
        input: DeleteEnvironmentRequest,
    ) -> Result<DeleteEnvironmentResult, RusotoError<DeleteEnvironmentError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "AWSCloud9WorkspaceManagementService.DeleteEnvironment",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, DeleteEnvironmentError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<DeleteEnvironmentResult, _>()
    }

    /// <p>Deletes an environment member from an AWS Cloud9 development environment.</p>
    async fn delete_environment_membership(
        &self,
        input: DeleteEnvironmentMembershipRequest,
    ) -> Result<DeleteEnvironmentMembershipResult, RusotoError<DeleteEnvironmentMembershipError>>
    {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "AWSCloud9WorkspaceManagementService.DeleteEnvironmentMembership",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, DeleteEnvironmentMembershipError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response)
            .deserialize::<DeleteEnvironmentMembershipResult, _>()
    }

    /// <p>Gets information about environment members for an AWS Cloud9 development environment.</p>
    async fn describe_environment_memberships(
        &self,
        input: DescribeEnvironmentMembershipsRequest,
    ) -> Result<
        DescribeEnvironmentMembershipsResult,
        RusotoError<DescribeEnvironmentMembershipsError>,
    > {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "AWSCloud9WorkspaceManagementService.DescribeEnvironmentMemberships",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, DescribeEnvironmentMembershipsError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response)
            .deserialize::<DescribeEnvironmentMembershipsResult, _>()
    }

    /// <p>Gets status information for an AWS Cloud9 development environment.</p>
    async fn describe_environment_status(
        &self,
        input: DescribeEnvironmentStatusRequest,
    ) -> Result<DescribeEnvironmentStatusResult, RusotoError<DescribeEnvironmentStatusError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "AWSCloud9WorkspaceManagementService.DescribeEnvironmentStatus",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, DescribeEnvironmentStatusError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response)
            .deserialize::<DescribeEnvironmentStatusResult, _>()
    }

    /// <p>Gets information about AWS Cloud9 development environments.</p>
    async fn describe_environments(
        &self,
        input: DescribeEnvironmentsRequest,
    ) -> Result<DescribeEnvironmentsResult, RusotoError<DescribeEnvironmentsError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "AWSCloud9WorkspaceManagementService.DescribeEnvironments",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, DescribeEnvironmentsError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<DescribeEnvironmentsResult, _>()
    }

    /// <p>Gets a list of AWS Cloud9 development environment identifiers.</p>
    async fn list_environments(
        &self,
        input: ListEnvironmentsRequest,
    ) -> Result<ListEnvironmentsResult, RusotoError<ListEnvironmentsError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "AWSCloud9WorkspaceManagementService.ListEnvironments",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, ListEnvironmentsError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<ListEnvironmentsResult, _>()
    }

    /// <p>Gets a list of the tags associated with an AWS Cloud9 development environment.</p>
    async fn list_tags_for_resource(
        &self,
        input: ListTagsForResourceRequest,
    ) -> Result<ListTagsForResourceResponse, RusotoError<ListTagsForResourceError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "AWSCloud9WorkspaceManagementService.ListTagsForResource",
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

    /// <p><p>Adds tags to an AWS Cloud9 development environment.</p> <important> <p>Tags that you add to an AWS Cloud9 environment by using this method will NOT be automatically propagated to underlying resources.</p> </important></p>
    async fn tag_resource(
        &self,
        input: TagResourceRequest,
    ) -> Result<TagResourceResponse, RusotoError<TagResourceError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "AWSCloud9WorkspaceManagementService.TagResource",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, TagResourceError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<TagResourceResponse, _>()
    }

    /// <p>Removes tags from an AWS Cloud9 development environment.</p>
    async fn untag_resource(
        &self,
        input: UntagResourceRequest,
    ) -> Result<UntagResourceResponse, RusotoError<UntagResourceError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "AWSCloud9WorkspaceManagementService.UntagResource",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, UntagResourceError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<UntagResourceResponse, _>()
    }

    /// <p>Changes the settings of an existing AWS Cloud9 development environment.</p>
    async fn update_environment(
        &self,
        input: UpdateEnvironmentRequest,
    ) -> Result<UpdateEnvironmentResult, RusotoError<UpdateEnvironmentError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "AWSCloud9WorkspaceManagementService.UpdateEnvironment",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, UpdateEnvironmentError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<UpdateEnvironmentResult, _>()
    }

    /// <p>Changes the settings of an existing environment member for an AWS Cloud9 development environment.</p>
    async fn update_environment_membership(
        &self,
        input: UpdateEnvironmentMembershipRequest,
    ) -> Result<UpdateEnvironmentMembershipResult, RusotoError<UpdateEnvironmentMembershipError>>
    {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "AWSCloud9WorkspaceManagementService.UpdateEnvironmentMembership",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, UpdateEnvironmentMembershipError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response)
            .deserialize::<UpdateEnvironmentMembershipResult, _>()
    }
}
