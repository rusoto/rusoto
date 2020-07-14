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
/// <p>A resource that is used to connect third-party source providers with services like AWS CodePipeline.</p> <p>Note: A connection created through CloudFormation, the CLI, or the SDK is in `PENDING` status by default. You can make its status `AVAILABLE` by updating the connection in the console.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct Connection {
    /// <p><p>The Amazon Resource Name (ARN) of the connection. The ARN is used as the connection reference when the connection is shared between AWS services.</p> <note> <p>The ARN is never reused if the connection is deleted.</p> </note></p>
    #[serde(rename = "ConnectionArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connection_arn: Option<String>,
    /// <p>The name of the connection. Connection names must be unique in an AWS user account.</p>
    #[serde(rename = "ConnectionName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connection_name: Option<String>,
    /// <p>The current status of the connection. </p>
    #[serde(rename = "ConnectionStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connection_status: Option<String>,
    /// <p>The Amazon Resource Name (ARN) of the host associated with the connection.</p>
    #[serde(rename = "HostArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub host_arn: Option<String>,
    /// <p>The identifier of the external provider where your third-party code repository is configured. For Bitbucket, this is the account ID of the owner of the Bitbucket repository.</p>
    #[serde(rename = "OwnerAccountId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner_account_id: Option<String>,
    /// <p>The name of the external provider where your third-party code repository is configured. The valid provider type is Bitbucket.</p>
    #[serde(rename = "ProviderType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider_type: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateConnectionInput {
    /// <p>The name of the connection to be created. The name must be unique in the calling AWS account.</p>
    #[serde(rename = "ConnectionName")]
    pub connection_name: String,
    /// <p>The Amazon Resource Name (ARN) of the host associated with the connection to be created.</p>
    #[serde(rename = "HostArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub host_arn: Option<String>,
    /// <p>The name of the external provider where your third-party code repository is configured. The valid provider type is Bitbucket.</p>
    #[serde(rename = "ProviderType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider_type: Option<String>,
    /// <p>The key-value pair to use when tagging the resource.</p>
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateConnectionOutput {
    /// <p><p>The Amazon Resource Name (ARN) of the connection to be created. The ARN is used as the connection reference when the connection is shared between AWS services.</p> <note> <p>The ARN is never reused if the connection is deleted.</p> </note></p>
    #[serde(rename = "ConnectionArn")]
    pub connection_arn: String,
    /// <p>Specifies the tags applied to the resource.</p>
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateHostInput {
    /// <p>The name of the host to be created. The name must be unique in the calling AWS account.</p>
    #[serde(rename = "Name")]
    pub name: String,
    /// <p>The endpoint of the infrastructure to be represented by the host after it is created.</p>
    #[serde(rename = "ProviderEndpoint")]
    pub provider_endpoint: String,
    /// <p>The name of the installed provider to be associated with your connection. The host resource represents the infrastructure where your provider type is installed. The valid provider type is GitHub Enterprise Server.</p>
    #[serde(rename = "ProviderType")]
    pub provider_type: String,
    /// <p>The VPC configuration to be provisioned for the host. A VPC must be configured and the infrastructure to be represented by the host must already be connected to the VPC.</p>
    #[serde(rename = "VpcConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_configuration: Option<VpcConfiguration>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateHostOutput {
    /// <p>The Amazon Resource Name (ARN) of the host to be created.</p>
    #[serde(rename = "HostArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub host_arn: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteConnectionInput {
    /// <p><p>The Amazon Resource Name (ARN) of the connection to be deleted.</p> <note> <p>The ARN is never reused if the connection is deleted.</p> </note></p>
    #[serde(rename = "ConnectionArn")]
    pub connection_arn: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DeleteConnectionOutput {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteHostInput {
    /// <p>The Amazon Resource Name (ARN) of the host to be deleted.</p>
    #[serde(rename = "HostArn")]
    pub host_arn: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DeleteHostOutput {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetConnectionInput {
    /// <p>The Amazon Resource Name (ARN) of a connection.</p>
    #[serde(rename = "ConnectionArn")]
    pub connection_arn: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetConnectionOutput {
    /// <p>The connection details, such as status, owner, and provider type.</p>
    #[serde(rename = "Connection")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connection: Option<Connection>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetHostInput {
    /// <p>The Amazon Resource Name (ARN) of the requested host.</p>
    #[serde(rename = "HostArn")]
    pub host_arn: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetHostOutput {
    /// <p>The name of the requested host.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The endpoint of the infrastructure represented by the requested host.</p>
    #[serde(rename = "ProviderEndpoint")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider_endpoint: Option<String>,
    /// <p>The provider type of the requested host, such as GitHub Enterprise Server.</p>
    #[serde(rename = "ProviderType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider_type: Option<String>,
    /// <p>The status of the requested host.</p>
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// <p>The VPC configuration of the requested host.</p>
    #[serde(rename = "VpcConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_configuration: Option<VpcConfiguration>,
}

/// <p><p>A resource that represents the infrastructure where a third-party provider is installed. The host is used when you create connections to an installed third-party provider type, such as GitHub Enterprise Server. You create one host for all connections to that provider.</p> <note> <p>A host created through the CLI or the SDK is in <code>PENDING</code> status by default. You can make its status <code>AVAILABLE</code> by setting up the host in the console.</p> </note></p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct Host {
    /// <p>The Amazon Resource Name (ARN) of the host.</p>
    #[serde(rename = "HostArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub host_arn: Option<String>,
    /// <p>The name of the host.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The endpoint of the infrastructure where your provider type is installed.</p>
    #[serde(rename = "ProviderEndpoint")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider_endpoint: Option<String>,
    /// <p>The name of the installed provider to be associated with your connection. The host resource represents the infrastructure where your provider type is installed. The valid provider type is GitHub Enterprise Server.</p>
    #[serde(rename = "ProviderType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider_type: Option<String>,
    /// <p>The status of the host, such as PENDING, AVAILABLE, VPC_CONFIG_DELETING, VPC_CONFIG_INITIALIZING, and VPC_CONFIG_FAILED_INITIALIZATION.</p>
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// <p>The status description for the host.</p>
    #[serde(rename = "StatusMessage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_message: Option<String>,
    /// <p>The VPC configuration provisioned for the host.</p>
    #[serde(rename = "VpcConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_configuration: Option<VpcConfiguration>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListConnectionsInput {
    /// <p>Filters the list of connections to those associated with a specified host.</p>
    #[serde(rename = "HostArnFilter")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub host_arn_filter: Option<String>,
    /// <p>The maximum number of results to return in a single call. To retrieve the remaining results, make another call with the returned <code>nextToken</code> value.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>The token that was returned from the previous <code>ListConnections</code> call, which can be used to return the next set of connections in the list.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>Filters the list of connections to those associated with a specified provider, such as Bitbucket.</p>
    #[serde(rename = "ProviderTypeFilter")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider_type_filter: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListConnectionsOutput {
    /// <p>A list of connections and the details for each connection, such as status, owner, and provider type.</p>
    #[serde(rename = "Connections")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connections: Option<Vec<Connection>>,
    /// <p>A token that can be used in the next <code>ListConnections</code> call. To view all items in the list, continue to call this operation with each subsequent token until no more <code>nextToken</code> values are returned.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListHostsInput {
    /// <p>The maximum number of results to return in a single call. To retrieve the remaining results, make another call with the returned <code>nextToken</code> value.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>The token that was returned from the previous <code>ListHosts</code> call, which can be used to return the next set of hosts in the list.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListHostsOutput {
    /// <p>A list of hosts and the details for each host, such as status, endpoint, and provider type.</p>
    #[serde(rename = "Hosts")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hosts: Option<Vec<Host>>,
    /// <p>A token that can be used in the next <code>ListHosts</code> call. To view all items in the list, continue to call this operation with each subsequent token until no more <code>nextToken</code> values are returned.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListTagsForResourceInput {
    /// <p>The Amazon Resource Name (ARN) of the resource for which you want to get information about tags, if any.</p>
    #[serde(rename = "ResourceArn")]
    pub resource_arn: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListTagsForResourceOutput {
    /// <p>A list of tag key and value pairs associated with the specified resource.</p>
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

/// <p>A tag is a key-value pair that is used to manage the resource.</p> <p>This tag is available for use by AWS services that support tags.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Tag {
    /// <p>The tag's key.</p>
    #[serde(rename = "Key")]
    pub key: String,
    /// <p>The tag's value.</p>
    #[serde(rename = "Value")]
    pub value: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct TagResourceInput {
    /// <p>The Amazon Resource Name (ARN) of the resource to which you want to add or update tags.</p>
    #[serde(rename = "ResourceArn")]
    pub resource_arn: String,
    /// <p>The tags you want to modify or add to the resource.</p>
    #[serde(rename = "Tags")]
    pub tags: Vec<Tag>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct TagResourceOutput {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UntagResourceInput {
    /// <p>The Amazon Resource Name (ARN) of the resource to remove tags from.</p>
    #[serde(rename = "ResourceArn")]
    pub resource_arn: String,
    /// <p>The list of keys for the tags to be removed from the resource.</p>
    #[serde(rename = "TagKeys")]
    pub tag_keys: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UntagResourceOutput {}

/// <p>The VPC configuration provisioned for the host.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct VpcConfiguration {
    /// <p>The ID of the security group or security groups associated with the Amazon VPC connected to the infrastructure where your provider type is installed.</p>
    #[serde(rename = "SecurityGroupIds")]
    pub security_group_ids: Vec<String>,
    /// <p>The ID of the subnet or subnets associated with the Amazon VPC connected to the infrastructure where your provider type is installed.</p>
    #[serde(rename = "SubnetIds")]
    pub subnet_ids: Vec<String>,
    /// <p>The value of the Transport Layer Security (TLS) certificate associated with the infrastructure where your provider type is installed.</p>
    #[serde(rename = "TlsCertificate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tls_certificate: Option<String>,
    /// <p>The ID of the Amazon VPC connected to the infrastructure where your provider type is installed.</p>
    #[serde(rename = "VpcId")]
    pub vpc_id: String,
}

/// Errors returned by CreateConnection
#[derive(Debug, PartialEq)]
pub enum CreateConnectionError {
    /// <p>Exceeded the maximum limit for connections.</p>
    LimitExceeded(String),
    /// <p>Resource not found. Verify the connection resource ARN and try again.</p>
    ResourceNotFound(String),
    /// <p>Resource not found. Verify the ARN for the host resource and try again.</p>
    ResourceUnavailable(String),
}

impl CreateConnectionError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateConnectionError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "LimitExceededException" => {
                    return RusotoError::Service(CreateConnectionError::LimitExceeded(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(CreateConnectionError::ResourceNotFound(err.msg))
                }
                "ResourceUnavailableException" => {
                    return RusotoError::Service(CreateConnectionError::ResourceUnavailable(
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
impl fmt::Display for CreateConnectionError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreateConnectionError::LimitExceeded(ref cause) => write!(f, "{}", cause),
            CreateConnectionError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            CreateConnectionError::ResourceUnavailable(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CreateConnectionError {}
/// Errors returned by CreateHost
#[derive(Debug, PartialEq)]
pub enum CreateHostError {
    /// <p>Exceeded the maximum limit for connections.</p>
    LimitExceeded(String),
}

impl CreateHostError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateHostError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "LimitExceededException" => {
                    return RusotoError::Service(CreateHostError::LimitExceeded(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for CreateHostError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreateHostError::LimitExceeded(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CreateHostError {}
/// Errors returned by DeleteConnection
#[derive(Debug, PartialEq)]
pub enum DeleteConnectionError {
    /// <p>Resource not found. Verify the connection resource ARN and try again.</p>
    ResourceNotFound(String),
}

impl DeleteConnectionError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteConnectionError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ResourceNotFoundException" => {
                    return RusotoError::Service(DeleteConnectionError::ResourceNotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DeleteConnectionError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteConnectionError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeleteConnectionError {}
/// Errors returned by DeleteHost
#[derive(Debug, PartialEq)]
pub enum DeleteHostError {
    /// <p>Resource not found. Verify the connection resource ARN and try again.</p>
    ResourceNotFound(String),
    /// <p>Resource not found. Verify the ARN for the host resource and try again.</p>
    ResourceUnavailable(String),
}

impl DeleteHostError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteHostError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ResourceNotFoundException" => {
                    return RusotoError::Service(DeleteHostError::ResourceNotFound(err.msg))
                }
                "ResourceUnavailableException" => {
                    return RusotoError::Service(DeleteHostError::ResourceUnavailable(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DeleteHostError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteHostError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            DeleteHostError::ResourceUnavailable(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeleteHostError {}
/// Errors returned by GetConnection
#[derive(Debug, PartialEq)]
pub enum GetConnectionError {
    /// <p>Resource not found. Verify the connection resource ARN and try again.</p>
    ResourceNotFound(String),
    /// <p>Resource not found. Verify the ARN for the host resource and try again.</p>
    ResourceUnavailable(String),
}

impl GetConnectionError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetConnectionError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ResourceNotFoundException" => {
                    return RusotoError::Service(GetConnectionError::ResourceNotFound(err.msg))
                }
                "ResourceUnavailableException" => {
                    return RusotoError::Service(GetConnectionError::ResourceUnavailable(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for GetConnectionError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetConnectionError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            GetConnectionError::ResourceUnavailable(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetConnectionError {}
/// Errors returned by GetHost
#[derive(Debug, PartialEq)]
pub enum GetHostError {
    /// <p>Resource not found. Verify the connection resource ARN and try again.</p>
    ResourceNotFound(String),
}

impl GetHostError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetHostError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ResourceNotFoundException" => {
                    return RusotoError::Service(GetHostError::ResourceNotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for GetHostError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetHostError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetHostError {}
/// Errors returned by ListConnections
#[derive(Debug, PartialEq)]
pub enum ListConnectionsError {}

impl ListConnectionsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListConnectionsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ListConnectionsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {}
    }
}
impl Error for ListConnectionsError {}
/// Errors returned by ListHosts
#[derive(Debug, PartialEq)]
pub enum ListHostsError {}

impl ListHostsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListHostsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ListHostsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {}
    }
}
impl Error for ListHostsError {}
/// Errors returned by ListTagsForResource
#[derive(Debug, PartialEq)]
pub enum ListTagsForResourceError {
    /// <p>Resource not found. Verify the connection resource ARN and try again.</p>
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
/// Errors returned by TagResource
#[derive(Debug, PartialEq)]
pub enum TagResourceError {
    /// <p>Exceeded the maximum limit for connections.</p>
    LimitExceeded(String),
    /// <p>Resource not found. Verify the connection resource ARN and try again.</p>
    ResourceNotFound(String),
}

impl TagResourceError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<TagResourceError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
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
            TagResourceError::LimitExceeded(ref cause) => write!(f, "{}", cause),
            TagResourceError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for TagResourceError {}
/// Errors returned by UntagResource
#[derive(Debug, PartialEq)]
pub enum UntagResourceError {
    /// <p>Resource not found. Verify the connection resource ARN and try again.</p>
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
/// Trait representing the capabilities of the AWS CodeStar connections API. AWS CodeStar connections clients implement this trait.
#[async_trait]
pub trait CodeStarConnections {
    /// <p>Creates a connection that can then be given to other AWS services like CodePipeline so that it can access third-party code repositories. The connection is in pending status until the third-party connection handshake is completed from the console.</p>
    async fn create_connection(
        &self,
        input: CreateConnectionInput,
    ) -> Result<CreateConnectionOutput, RusotoError<CreateConnectionError>>;

    /// <p><p>Creates a resource that represents the infrastructure where a third-party provider is installed. The host is used when you create connections to an installed third-party provider type, such as GitHub Enterprise Server. You create one host for all connections to that provider.</p> <note> <p>A host created through the CLI or the SDK is in <code>PENDING</code> status by default. You can make its status <code>AVAILABLE</code> by setting up the host in the console.</p> </note></p>
    async fn create_host(
        &self,
        input: CreateHostInput,
    ) -> Result<CreateHostOutput, RusotoError<CreateHostError>>;

    /// <p>The connection to be deleted.</p>
    async fn delete_connection(
        &self,
        input: DeleteConnectionInput,
    ) -> Result<DeleteConnectionOutput, RusotoError<DeleteConnectionError>>;

    /// <p><p>The host to be deleted. Before you delete a host, all connections associated to the host must be deleted.</p> <note> <p>A host cannot be deleted if it is in the VPC<em>CONFIG</em>INITIALIZING or VPC<em>CONFIG</em>DELETING state.</p> </note></p>
    async fn delete_host(
        &self,
        input: DeleteHostInput,
    ) -> Result<DeleteHostOutput, RusotoError<DeleteHostError>>;

    /// <p>Returns the connection ARN and details such as status, owner, and provider type.</p>
    async fn get_connection(
        &self,
        input: GetConnectionInput,
    ) -> Result<GetConnectionOutput, RusotoError<GetConnectionError>>;

    /// <p>Returns the host ARN and details such as status, provider type, endpoint, and, if applicable, the VPC configuration.</p>
    async fn get_host(
        &self,
        input: GetHostInput,
    ) -> Result<GetHostOutput, RusotoError<GetHostError>>;

    /// <p>Lists the connections associated with your account.</p>
    async fn list_connections(
        &self,
        input: ListConnectionsInput,
    ) -> Result<ListConnectionsOutput, RusotoError<ListConnectionsError>>;

    /// <p>Lists the hosts associated with your account.</p>
    async fn list_hosts(
        &self,
        input: ListHostsInput,
    ) -> Result<ListHostsOutput, RusotoError<ListHostsError>>;

    /// <p>Gets the set of key-value pairs (metadata) that are used to manage the resource.</p>
    async fn list_tags_for_resource(
        &self,
        input: ListTagsForResourceInput,
    ) -> Result<ListTagsForResourceOutput, RusotoError<ListTagsForResourceError>>;

    /// <p>Adds to or modifies the tags of the given resource. Tags are metadata that can be used to manage a resource.</p>
    async fn tag_resource(
        &self,
        input: TagResourceInput,
    ) -> Result<TagResourceOutput, RusotoError<TagResourceError>>;

    /// <p>Removes tags from an AWS resource.</p>
    async fn untag_resource(
        &self,
        input: UntagResourceInput,
    ) -> Result<UntagResourceOutput, RusotoError<UntagResourceError>>;
}
/// A client for the AWS CodeStar connections API.
#[derive(Clone)]
pub struct CodeStarConnectionsClient {
    client: Client,
    region: region::Region,
}

impl CodeStarConnectionsClient {
    /// Creates a client backed by the default tokio event loop.
    ///
    /// The client will use the default credentials provider and tls client.
    pub fn new(region: region::Region) -> CodeStarConnectionsClient {
        CodeStarConnectionsClient {
            client: Client::shared(),
            region,
        }
    }

    pub fn new_with<P, D>(
        request_dispatcher: D,
        credentials_provider: P,
        region: region::Region,
    ) -> CodeStarConnectionsClient
    where
        P: ProvideAwsCredentials + Send + Sync + 'static,
        D: DispatchSignedRequest + Send + Sync + 'static,
    {
        CodeStarConnectionsClient {
            client: Client::new_with(credentials_provider, request_dispatcher),
            region,
        }
    }

    pub fn new_with_client(client: Client, region: region::Region) -> CodeStarConnectionsClient {
        CodeStarConnectionsClient { client, region }
    }
}

#[async_trait]
impl CodeStarConnections for CodeStarConnectionsClient {
    /// <p>Creates a connection that can then be given to other AWS services like CodePipeline so that it can access third-party code repositories. The connection is in pending status until the third-party connection handshake is completed from the console.</p>
    async fn create_connection(
        &self,
        input: CreateConnectionInput,
    ) -> Result<CreateConnectionOutput, RusotoError<CreateConnectionError>> {
        let mut request = SignedRequest::new("POST", "codestar-connections", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.0".to_owned());
        request.add_header(
            "x-amz-target",
            "com.amazonaws.codestar.connections.CodeStar_connections_20191201.CreateConnection",
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
            proto::json::ResponsePayload::new(&response).deserialize::<CreateConnectionOutput, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(CreateConnectionError::from_response(response))
        }
    }

    /// <p><p>Creates a resource that represents the infrastructure where a third-party provider is installed. The host is used when you create connections to an installed third-party provider type, such as GitHub Enterprise Server. You create one host for all connections to that provider.</p> <note> <p>A host created through the CLI or the SDK is in <code>PENDING</code> status by default. You can make its status <code>AVAILABLE</code> by setting up the host in the console.</p> </note></p>
    async fn create_host(
        &self,
        input: CreateHostInput,
    ) -> Result<CreateHostOutput, RusotoError<CreateHostError>> {
        let mut request = SignedRequest::new("POST", "codestar-connections", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.0".to_owned());
        request.add_header(
            "x-amz-target",
            "com.amazonaws.codestar.connections.CodeStar_connections_20191201.CreateHost",
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
            proto::json::ResponsePayload::new(&response).deserialize::<CreateHostOutput, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(CreateHostError::from_response(response))
        }
    }

    /// <p>The connection to be deleted.</p>
    async fn delete_connection(
        &self,
        input: DeleteConnectionInput,
    ) -> Result<DeleteConnectionOutput, RusotoError<DeleteConnectionError>> {
        let mut request = SignedRequest::new("POST", "codestar-connections", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.0".to_owned());
        request.add_header(
            "x-amz-target",
            "com.amazonaws.codestar.connections.CodeStar_connections_20191201.DeleteConnection",
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
            proto::json::ResponsePayload::new(&response).deserialize::<DeleteConnectionOutput, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(DeleteConnectionError::from_response(response))
        }
    }

    /// <p><p>The host to be deleted. Before you delete a host, all connections associated to the host must be deleted.</p> <note> <p>A host cannot be deleted if it is in the VPC<em>CONFIG</em>INITIALIZING or VPC<em>CONFIG</em>DELETING state.</p> </note></p>
    async fn delete_host(
        &self,
        input: DeleteHostInput,
    ) -> Result<DeleteHostOutput, RusotoError<DeleteHostError>> {
        let mut request = SignedRequest::new("POST", "codestar-connections", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.0".to_owned());
        request.add_header(
            "x-amz-target",
            "com.amazonaws.codestar.connections.CodeStar_connections_20191201.DeleteHost",
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
            proto::json::ResponsePayload::new(&response).deserialize::<DeleteHostOutput, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(DeleteHostError::from_response(response))
        }
    }

    /// <p>Returns the connection ARN and details such as status, owner, and provider type.</p>
    async fn get_connection(
        &self,
        input: GetConnectionInput,
    ) -> Result<GetConnectionOutput, RusotoError<GetConnectionError>> {
        let mut request = SignedRequest::new("POST", "codestar-connections", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.0".to_owned());
        request.add_header(
            "x-amz-target",
            "com.amazonaws.codestar.connections.CodeStar_connections_20191201.GetConnection",
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
            proto::json::ResponsePayload::new(&response).deserialize::<GetConnectionOutput, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(GetConnectionError::from_response(response))
        }
    }

    /// <p>Returns the host ARN and details such as status, provider type, endpoint, and, if applicable, the VPC configuration.</p>
    async fn get_host(
        &self,
        input: GetHostInput,
    ) -> Result<GetHostOutput, RusotoError<GetHostError>> {
        let mut request = SignedRequest::new("POST", "codestar-connections", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.0".to_owned());
        request.add_header(
            "x-amz-target",
            "com.amazonaws.codestar.connections.CodeStar_connections_20191201.GetHost",
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
            proto::json::ResponsePayload::new(&response).deserialize::<GetHostOutput, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(GetHostError::from_response(response))
        }
    }

    /// <p>Lists the connections associated with your account.</p>
    async fn list_connections(
        &self,
        input: ListConnectionsInput,
    ) -> Result<ListConnectionsOutput, RusotoError<ListConnectionsError>> {
        let mut request = SignedRequest::new("POST", "codestar-connections", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.0".to_owned());
        request.add_header(
            "x-amz-target",
            "com.amazonaws.codestar.connections.CodeStar_connections_20191201.ListConnections",
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
            proto::json::ResponsePayload::new(&response).deserialize::<ListConnectionsOutput, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(ListConnectionsError::from_response(response))
        }
    }

    /// <p>Lists the hosts associated with your account.</p>
    async fn list_hosts(
        &self,
        input: ListHostsInput,
    ) -> Result<ListHostsOutput, RusotoError<ListHostsError>> {
        let mut request = SignedRequest::new("POST", "codestar-connections", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.0".to_owned());
        request.add_header(
            "x-amz-target",
            "com.amazonaws.codestar.connections.CodeStar_connections_20191201.ListHosts",
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
            proto::json::ResponsePayload::new(&response).deserialize::<ListHostsOutput, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(ListHostsError::from_response(response))
        }
    }

    /// <p>Gets the set of key-value pairs (metadata) that are used to manage the resource.</p>
    async fn list_tags_for_resource(
        &self,
        input: ListTagsForResourceInput,
    ) -> Result<ListTagsForResourceOutput, RusotoError<ListTagsForResourceError>> {
        let mut request = SignedRequest::new("POST", "codestar-connections", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.0".to_owned());
        request.add_header(
            "x-amz-target",
            "com.amazonaws.codestar.connections.CodeStar_connections_20191201.ListTagsForResource",
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
                .deserialize::<ListTagsForResourceOutput, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(ListTagsForResourceError::from_response(response))
        }
    }

    /// <p>Adds to or modifies the tags of the given resource. Tags are metadata that can be used to manage a resource.</p>
    async fn tag_resource(
        &self,
        input: TagResourceInput,
    ) -> Result<TagResourceOutput, RusotoError<TagResourceError>> {
        let mut request = SignedRequest::new("POST", "codestar-connections", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.0".to_owned());
        request.add_header(
            "x-amz-target",
            "com.amazonaws.codestar.connections.CodeStar_connections_20191201.TagResource",
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
            proto::json::ResponsePayload::new(&response).deserialize::<TagResourceOutput, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(TagResourceError::from_response(response))
        }
    }

    /// <p>Removes tags from an AWS resource.</p>
    async fn untag_resource(
        &self,
        input: UntagResourceInput,
    ) -> Result<UntagResourceOutput, RusotoError<UntagResourceError>> {
        let mut request = SignedRequest::new("POST", "codestar-connections", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.0".to_owned());
        request.add_header(
            "x-amz-target",
            "com.amazonaws.codestar.connections.CodeStar_connections_20191201.UntagResource",
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
            proto::json::ResponsePayload::new(&response).deserialize::<UntagResourceOutput, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(UntagResourceError::from_response(response))
        }
    }
}
