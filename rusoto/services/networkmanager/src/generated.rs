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

use rusoto_core::param::{Params, ServiceParams};
use rusoto_core::proto;
use rusoto_core::signature::SignedRequest;
#[allow(unused_imports)]
use serde::{Deserialize, Serialize};
/// <p>Specifies a location in AWS.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct AWSLocation {
    /// <p>The Amazon Resource Name (ARN) of the subnet the device is located in.</p>
    #[serde(rename = "SubnetArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subnet_arn: Option<String>,
    /// <p>The Zone the device is located in. This can be the ID of an Availability Zone, Local Zone, Wavelength Zone, or an Outpost.</p>
    #[serde(rename = "Zone")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zone: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct AssociateCustomerGatewayRequest {
    /// <p>The Amazon Resource Name (ARN) of the customer gateway. For more information, see <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/list_amazonec2.html#amazonec2-resources-for-iam-policies">Resources Defined by Amazon EC2</a>.</p>
    #[serde(rename = "CustomerGatewayArn")]
    pub customer_gateway_arn: String,
    /// <p>The ID of the device.</p>
    #[serde(rename = "DeviceId")]
    pub device_id: String,
    /// <p>The ID of the global network.</p>
    #[serde(rename = "GlobalNetworkId")]
    pub global_network_id: String,
    /// <p>The ID of the link.</p>
    #[serde(rename = "LinkId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub link_id: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct AssociateCustomerGatewayResponse {
    /// <p>The customer gateway association.</p>
    #[serde(rename = "CustomerGatewayAssociation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_gateway_association: Option<CustomerGatewayAssociation>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct AssociateLinkRequest {
    /// <p>The ID of the device.</p>
    #[serde(rename = "DeviceId")]
    pub device_id: String,
    /// <p>The ID of the global network.</p>
    #[serde(rename = "GlobalNetworkId")]
    pub global_network_id: String,
    /// <p>The ID of the link.</p>
    #[serde(rename = "LinkId")]
    pub link_id: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct AssociateLinkResponse {
    /// <p>The link association.</p>
    #[serde(rename = "LinkAssociation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub link_association: Option<LinkAssociation>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct AssociateTransitGatewayConnectPeerRequest {
    /// <p>The ID of the device.</p>
    #[serde(rename = "DeviceId")]
    pub device_id: String,
    /// <p>The ID of the global network.</p>
    #[serde(rename = "GlobalNetworkId")]
    pub global_network_id: String,
    /// <p>The ID of the link.</p>
    #[serde(rename = "LinkId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub link_id: Option<String>,
    /// <p>The Amazon Resource Name (ARN) of the Connect peer.</p>
    #[serde(rename = "TransitGatewayConnectPeerArn")]
    pub transit_gateway_connect_peer_arn: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct AssociateTransitGatewayConnectPeerResponse {
    /// <p>The transit gateway Connect peer association.</p>
    #[serde(rename = "TransitGatewayConnectPeerAssociation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transit_gateway_connect_peer_association: Option<TransitGatewayConnectPeerAssociation>,
}

/// <p>Describes bandwidth information.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct Bandwidth {
    /// <p>Download speed in Mbps.</p>
    #[serde(rename = "DownloadSpeed")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub download_speed: Option<i64>,
    /// <p>Upload speed in Mbps.</p>
    #[serde(rename = "UploadSpeed")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub upload_speed: Option<i64>,
}

/// <p>Describes a connection.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct Connection {
    /// <p>The ID of the second device in the connection.</p>
    #[serde(rename = "ConnectedDeviceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connected_device_id: Option<String>,
    /// <p>The ID of the link for the second device in the connection.</p>
    #[serde(rename = "ConnectedLinkId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connected_link_id: Option<String>,
    /// <p>The Amazon Resource Name (ARN) of the connection.</p>
    #[serde(rename = "ConnectionArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connection_arn: Option<String>,
    /// <p>The ID of the connection.</p>
    #[serde(rename = "ConnectionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connection_id: Option<String>,
    /// <p>The date and time that the connection was created.</p>
    #[serde(rename = "CreatedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<f64>,
    /// <p>The description of the connection.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The ID of the first device in the connection.</p>
    #[serde(rename = "DeviceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_id: Option<String>,
    /// <p>The ID of the global network.</p>
    #[serde(rename = "GlobalNetworkId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub global_network_id: Option<String>,
    /// <p>The ID of the link for the first device in the connection.</p>
    #[serde(rename = "LinkId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub link_id: Option<String>,
    /// <p>The state of the connection.</p>
    #[serde(rename = "State")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    /// <p>The tags for the connection.</p>
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateConnectionRequest {
    /// <p>The ID of the second device in the connection.</p>
    #[serde(rename = "ConnectedDeviceId")]
    pub connected_device_id: String,
    /// <p>The ID of the link for the second device.</p>
    #[serde(rename = "ConnectedLinkId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connected_link_id: Option<String>,
    /// <p>A description of the connection.</p> <p>Length Constraints: Maximum length of 256 characters.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The ID of the first device in the connection.</p>
    #[serde(rename = "DeviceId")]
    pub device_id: String,
    /// <p>The ID of the global network.</p>
    #[serde(rename = "GlobalNetworkId")]
    pub global_network_id: String,
    /// <p>The ID of the link for the first device.</p>
    #[serde(rename = "LinkId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub link_id: Option<String>,
    /// <p>The tags to apply to the resource during creation.</p>
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateConnectionResponse {
    /// <p>Information about the connection.</p>
    #[serde(rename = "Connection")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connection: Option<Connection>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateDeviceRequest {
    /// <p>The AWS location of the device.</p>
    #[serde(rename = "AWSLocation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aws_location: Option<AWSLocation>,
    /// <p>A description of the device.</p> <p>Length Constraints: Maximum length of 256 characters.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The ID of the global network.</p>
    #[serde(rename = "GlobalNetworkId")]
    pub global_network_id: String,
    /// <p>The location of the device.</p>
    #[serde(rename = "Location")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<Location>,
    /// <p>The model of the device.</p> <p>Length Constraints: Maximum length of 128 characters.</p>
    #[serde(rename = "Model")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model: Option<String>,
    /// <p>The serial number of the device.</p> <p>Length Constraints: Maximum length of 128 characters.</p>
    #[serde(rename = "SerialNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub serial_number: Option<String>,
    /// <p>The ID of the site.</p>
    #[serde(rename = "SiteId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub site_id: Option<String>,
    /// <p>The tags to apply to the resource during creation.</p>
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
    /// <p>The type of the device.</p>
    #[serde(rename = "Type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    /// <p>The vendor of the device.</p> <p>Length Constraints: Maximum length of 128 characters.</p>
    #[serde(rename = "Vendor")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vendor: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateDeviceResponse {
    /// <p>Information about the device.</p>
    #[serde(rename = "Device")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device: Option<Device>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateGlobalNetworkRequest {
    /// <p>A description of the global network.</p> <p>Length Constraints: Maximum length of 256 characters.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The tags to apply to the resource during creation.</p>
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateGlobalNetworkResponse {
    /// <p>Information about the global network object.</p>
    #[serde(rename = "GlobalNetwork")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub global_network: Option<GlobalNetwork>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateLinkRequest {
    /// <p> The upload speed and download speed in Mbps. </p>
    #[serde(rename = "Bandwidth")]
    pub bandwidth: Bandwidth,
    /// <p>A description of the link.</p> <p>Length Constraints: Maximum length of 256 characters.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The ID of the global network.</p>
    #[serde(rename = "GlobalNetworkId")]
    pub global_network_id: String,
    /// <p>The provider of the link.</p> <p>Constraints: Cannot include the following characters: | \ ^</p> <p>Length Constraints: Maximum length of 128 characters.</p>
    #[serde(rename = "Provider")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider: Option<String>,
    /// <p>The ID of the site.</p>
    #[serde(rename = "SiteId")]
    pub site_id: String,
    /// <p>The tags to apply to the resource during creation.</p>
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
    /// <p>The type of the link.</p> <p>Constraints: Cannot include the following characters: | \ ^</p> <p>Length Constraints: Maximum length of 128 characters.</p>
    #[serde(rename = "Type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateLinkResponse {
    /// <p>Information about the link.</p>
    #[serde(rename = "Link")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub link: Option<Link>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateSiteRequest {
    /// <p>A description of your site.</p> <p>Length Constraints: Maximum length of 256 characters.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The ID of the global network.</p>
    #[serde(rename = "GlobalNetworkId")]
    pub global_network_id: String,
    /// <p><p>The site location. This information is used for visualization in the Network Manager console. If you specify the address, the latitude and longitude are automatically calculated.</p> <ul> <li> <p> <code>Address</code>: The physical address of the site.</p> </li> <li> <p> <code>Latitude</code>: The latitude of the site. </p> </li> <li> <p> <code>Longitude</code>: The longitude of the site.</p> </li> </ul></p>
    #[serde(rename = "Location")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<Location>,
    /// <p>The tags to apply to the resource during creation.</p>
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateSiteResponse {
    /// <p>Information about the site.</p>
    #[serde(rename = "Site")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub site: Option<Site>,
}

/// <p>Describes the association between a customer gateway, a device, and a link.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CustomerGatewayAssociation {
    /// <p>The Amazon Resource Name (ARN) of the customer gateway.</p>
    #[serde(rename = "CustomerGatewayArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_gateway_arn: Option<String>,
    /// <p>The ID of the device.</p>
    #[serde(rename = "DeviceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_id: Option<String>,
    /// <p>The ID of the global network.</p>
    #[serde(rename = "GlobalNetworkId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub global_network_id: Option<String>,
    /// <p>The ID of the link.</p>
    #[serde(rename = "LinkId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub link_id: Option<String>,
    /// <p>The association state.</p>
    #[serde(rename = "State")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteConnectionRequest {
    /// <p>The ID of the connection.</p>
    #[serde(rename = "ConnectionId")]
    pub connection_id: String,
    /// <p>The ID of the global network.</p>
    #[serde(rename = "GlobalNetworkId")]
    pub global_network_id: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DeleteConnectionResponse {
    /// <p>Information about the connection.</p>
    #[serde(rename = "Connection")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connection: Option<Connection>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteDeviceRequest {
    /// <p>The ID of the device.</p>
    #[serde(rename = "DeviceId")]
    pub device_id: String,
    /// <p>The ID of the global network.</p>
    #[serde(rename = "GlobalNetworkId")]
    pub global_network_id: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DeleteDeviceResponse {
    /// <p>Information about the device.</p>
    #[serde(rename = "Device")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device: Option<Device>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteGlobalNetworkRequest {
    /// <p>The ID of the global network.</p>
    #[serde(rename = "GlobalNetworkId")]
    pub global_network_id: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DeleteGlobalNetworkResponse {
    /// <p>Information about the global network.</p>
    #[serde(rename = "GlobalNetwork")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub global_network: Option<GlobalNetwork>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteLinkRequest {
    /// <p>The ID of the global network.</p>
    #[serde(rename = "GlobalNetworkId")]
    pub global_network_id: String,
    /// <p>The ID of the link.</p>
    #[serde(rename = "LinkId")]
    pub link_id: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DeleteLinkResponse {
    /// <p>Information about the link.</p>
    #[serde(rename = "Link")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub link: Option<Link>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteSiteRequest {
    /// <p>The ID of the global network.</p>
    #[serde(rename = "GlobalNetworkId")]
    pub global_network_id: String,
    /// <p>The ID of the site.</p>
    #[serde(rename = "SiteId")]
    pub site_id: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DeleteSiteResponse {
    /// <p>Information about the site.</p>
    #[serde(rename = "Site")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub site: Option<Site>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeregisterTransitGatewayRequest {
    /// <p>The ID of the global network.</p>
    #[serde(rename = "GlobalNetworkId")]
    pub global_network_id: String,
    /// <p>The Amazon Resource Name (ARN) of the transit gateway.</p>
    #[serde(rename = "TransitGatewayArn")]
    pub transit_gateway_arn: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DeregisterTransitGatewayResponse {
    /// <p>The transit gateway registration information.</p>
    #[serde(rename = "TransitGatewayRegistration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transit_gateway_registration: Option<TransitGatewayRegistration>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeGlobalNetworksRequest {
    /// <p>The IDs of one or more global networks. The maximum is 10.</p>
    #[serde(rename = "GlobalNetworkIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub global_network_ids: Option<Vec<String>>,
    /// <p>The maximum number of results to return.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>The token for the next page of results.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeGlobalNetworksResponse {
    /// <p>Information about the global networks.</p>
    #[serde(rename = "GlobalNetworks")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub global_networks: Option<Vec<GlobalNetwork>>,
    /// <p>The token for the next page of results.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

/// <p>Describes a device.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct Device {
    /// <p>The AWS location of the device.</p>
    #[serde(rename = "AWSLocation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aws_location: Option<AWSLocation>,
    /// <p>The date and time that the site was created.</p>
    #[serde(rename = "CreatedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<f64>,
    /// <p>The description of the device.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The Amazon Resource Name (ARN) of the device.</p>
    #[serde(rename = "DeviceArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_arn: Option<String>,
    /// <p>The ID of the device.</p>
    #[serde(rename = "DeviceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_id: Option<String>,
    /// <p>The ID of the global network.</p>
    #[serde(rename = "GlobalNetworkId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub global_network_id: Option<String>,
    /// <p>The site location.</p>
    #[serde(rename = "Location")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<Location>,
    /// <p>The device model.</p>
    #[serde(rename = "Model")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model: Option<String>,
    /// <p>The device serial number.</p>
    #[serde(rename = "SerialNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub serial_number: Option<String>,
    /// <p>The site ID.</p>
    #[serde(rename = "SiteId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub site_id: Option<String>,
    /// <p>The device state.</p>
    #[serde(rename = "State")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    /// <p>The tags for the device.</p>
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
    /// <p>The device type.</p>
    #[serde(rename = "Type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    /// <p>The device vendor.</p>
    #[serde(rename = "Vendor")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vendor: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DisassociateCustomerGatewayRequest {
    /// <p>The Amazon Resource Name (ARN) of the customer gateway. For more information, see <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/list_amazonec2.html#amazonec2-resources-for-iam-policies">Resources Defined by Amazon EC2</a>.</p>
    #[serde(rename = "CustomerGatewayArn")]
    pub customer_gateway_arn: String,
    /// <p>The ID of the global network.</p>
    #[serde(rename = "GlobalNetworkId")]
    pub global_network_id: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DisassociateCustomerGatewayResponse {
    /// <p>Information about the customer gateway association.</p>
    #[serde(rename = "CustomerGatewayAssociation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_gateway_association: Option<CustomerGatewayAssociation>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DisassociateLinkRequest {
    /// <p>The ID of the device.</p>
    #[serde(rename = "DeviceId")]
    pub device_id: String,
    /// <p>The ID of the global network.</p>
    #[serde(rename = "GlobalNetworkId")]
    pub global_network_id: String,
    /// <p>The ID of the link.</p>
    #[serde(rename = "LinkId")]
    pub link_id: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DisassociateLinkResponse {
    /// <p>Information about the link association.</p>
    #[serde(rename = "LinkAssociation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub link_association: Option<LinkAssociation>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DisassociateTransitGatewayConnectPeerRequest {
    /// <p>The ID of the global network.</p>
    #[serde(rename = "GlobalNetworkId")]
    pub global_network_id: String,
    /// <p>The Amazon Resource Name (ARN) of the transit gateway Connect peer.</p>
    #[serde(rename = "TransitGatewayConnectPeerArn")]
    pub transit_gateway_connect_peer_arn: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DisassociateTransitGatewayConnectPeerResponse {
    /// <p>The transit gateway Connect peer association.</p>
    #[serde(rename = "TransitGatewayConnectPeerAssociation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transit_gateway_connect_peer_association: Option<TransitGatewayConnectPeerAssociation>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetConnectionsRequest {
    /// <p>One or more connection IDs.</p>
    #[serde(rename = "ConnectionIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connection_ids: Option<Vec<String>>,
    /// <p>The ID of the device.</p>
    #[serde(rename = "DeviceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_id: Option<String>,
    /// <p>The ID of the global network.</p>
    #[serde(rename = "GlobalNetworkId")]
    pub global_network_id: String,
    /// <p>The maximum number of results to return.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>The token for the next page of results.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetConnectionsResponse {
    /// <p>Information about the connections.</p>
    #[serde(rename = "Connections")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connections: Option<Vec<Connection>>,
    /// <p>The token to use for the next page of results.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetCustomerGatewayAssociationsRequest {
    /// <p>One or more customer gateway Amazon Resource Names (ARNs). For more information, see <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/list_amazonec2.html#amazonec2-resources-for-iam-policies">Resources Defined by Amazon EC2</a>. The maximum is 10.</p>
    #[serde(rename = "CustomerGatewayArns")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_gateway_arns: Option<Vec<String>>,
    /// <p>The ID of the global network.</p>
    #[serde(rename = "GlobalNetworkId")]
    pub global_network_id: String,
    /// <p>The maximum number of results to return.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>The token for the next page of results.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetCustomerGatewayAssociationsResponse {
    /// <p>The customer gateway associations.</p>
    #[serde(rename = "CustomerGatewayAssociations")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_gateway_associations: Option<Vec<CustomerGatewayAssociation>>,
    /// <p>The token for the next page of results.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetDevicesRequest {
    /// <p>One or more device IDs. The maximum is 10.</p>
    #[serde(rename = "DeviceIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_ids: Option<Vec<String>>,
    /// <p>The ID of the global network.</p>
    #[serde(rename = "GlobalNetworkId")]
    pub global_network_id: String,
    /// <p>The maximum number of results to return.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>The token for the next page of results.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The ID of the site.</p>
    #[serde(rename = "SiteId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub site_id: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetDevicesResponse {
    /// <p>The devices.</p>
    #[serde(rename = "Devices")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub devices: Option<Vec<Device>>,
    /// <p>The token for the next page of results.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetLinkAssociationsRequest {
    /// <p>The ID of the device.</p>
    #[serde(rename = "DeviceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_id: Option<String>,
    /// <p>The ID of the global network.</p>
    #[serde(rename = "GlobalNetworkId")]
    pub global_network_id: String,
    /// <p>The ID of the link.</p>
    #[serde(rename = "LinkId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub link_id: Option<String>,
    /// <p>The maximum number of results to return.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>The token for the next page of results.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetLinkAssociationsResponse {
    /// <p>The link associations.</p>
    #[serde(rename = "LinkAssociations")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub link_associations: Option<Vec<LinkAssociation>>,
    /// <p>The token for the next page of results.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetLinksRequest {
    /// <p>The ID of the global network.</p>
    #[serde(rename = "GlobalNetworkId")]
    pub global_network_id: String,
    /// <p>One or more link IDs. The maximum is 10.</p>
    #[serde(rename = "LinkIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub link_ids: Option<Vec<String>>,
    /// <p>The maximum number of results to return.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>The token for the next page of results.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The link provider.</p>
    #[serde(rename = "Provider")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider: Option<String>,
    /// <p>The ID of the site.</p>
    #[serde(rename = "SiteId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub site_id: Option<String>,
    /// <p>The link type.</p>
    #[serde(rename = "Type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetLinksResponse {
    /// <p>The links.</p>
    #[serde(rename = "Links")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub links: Option<Vec<Link>>,
    /// <p>The token for the next page of results.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetSitesRequest {
    /// <p>The ID of the global network.</p>
    #[serde(rename = "GlobalNetworkId")]
    pub global_network_id: String,
    /// <p>The maximum number of results to return.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>The token for the next page of results.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>One or more site IDs. The maximum is 10.</p>
    #[serde(rename = "SiteIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub site_ids: Option<Vec<String>>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetSitesResponse {
    /// <p>The token for the next page of results.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The sites.</p>
    #[serde(rename = "Sites")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sites: Option<Vec<Site>>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetTransitGatewayConnectPeerAssociationsRequest {
    /// <p>The ID of the global network.</p>
    #[serde(rename = "GlobalNetworkId")]
    pub global_network_id: String,
    /// <p>The maximum number of results to return.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>The token for the next page of results.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>One or more transit gateway Connect peer Amazon Resource Names (ARNs).</p>
    #[serde(rename = "TransitGatewayConnectPeerArns")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transit_gateway_connect_peer_arns: Option<Vec<String>>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetTransitGatewayConnectPeerAssociationsResponse {
    /// <p>The token to use for the next page of results.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>Information about the transit gateway Connect peer associations.</p>
    #[serde(rename = "TransitGatewayConnectPeerAssociations")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transit_gateway_connect_peer_associations:
        Option<Vec<TransitGatewayConnectPeerAssociation>>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetTransitGatewayRegistrationsRequest {
    /// <p>The ID of the global network.</p>
    #[serde(rename = "GlobalNetworkId")]
    pub global_network_id: String,
    /// <p>The maximum number of results to return.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>The token for the next page of results.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The Amazon Resource Names (ARNs) of one or more transit gateways. The maximum is 10.</p>
    #[serde(rename = "TransitGatewayArns")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transit_gateway_arns: Option<Vec<String>>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetTransitGatewayRegistrationsResponse {
    /// <p>The token for the next page of results.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The transit gateway registrations.</p>
    #[serde(rename = "TransitGatewayRegistrations")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transit_gateway_registrations: Option<Vec<TransitGatewayRegistration>>,
}

/// <p>Describes a global network.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GlobalNetwork {
    /// <p>The date and time that the global network was created.</p>
    #[serde(rename = "CreatedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<f64>,
    /// <p>The description of the global network.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The Amazon Resource Name (ARN) of the global network.</p>
    #[serde(rename = "GlobalNetworkArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub global_network_arn: Option<String>,
    /// <p>The ID of the global network.</p>
    #[serde(rename = "GlobalNetworkId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub global_network_id: Option<String>,
    /// <p>The state of the global network.</p>
    #[serde(rename = "State")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    /// <p>The tags for the global network.</p>
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

/// <p>Describes a link.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct Link {
    /// <p>The bandwidth for the link.</p>
    #[serde(rename = "Bandwidth")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bandwidth: Option<Bandwidth>,
    /// <p>The date and time that the link was created.</p>
    #[serde(rename = "CreatedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<f64>,
    /// <p>The description of the link.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The ID of the global network.</p>
    #[serde(rename = "GlobalNetworkId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub global_network_id: Option<String>,
    /// <p>The Amazon Resource Name (ARN) of the link.</p>
    #[serde(rename = "LinkArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub link_arn: Option<String>,
    /// <p>The ID of the link.</p>
    #[serde(rename = "LinkId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub link_id: Option<String>,
    /// <p>The provider of the link.</p>
    #[serde(rename = "Provider")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider: Option<String>,
    /// <p>The ID of the site.</p>
    #[serde(rename = "SiteId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub site_id: Option<String>,
    /// <p>The state of the link.</p>
    #[serde(rename = "State")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    /// <p>The tags for the link.</p>
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
    /// <p>The type of the link.</p>
    #[serde(rename = "Type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}

/// <p>Describes the association between a device and a link.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct LinkAssociation {
    /// <p>The device ID for the link association.</p>
    #[serde(rename = "DeviceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_id: Option<String>,
    /// <p>The ID of the global network.</p>
    #[serde(rename = "GlobalNetworkId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub global_network_id: Option<String>,
    /// <p>The state of the association.</p>
    #[serde(rename = "LinkAssociationState")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub link_association_state: Option<String>,
    /// <p>The ID of the link.</p>
    #[serde(rename = "LinkId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub link_id: Option<String>,
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
    /// <p>The list of tags.</p>
    #[serde(rename = "TagList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_list: Option<Vec<Tag>>,
}

/// <p>Describes a location.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct Location {
    /// <p>The physical address.</p>
    #[serde(rename = "Address")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
    /// <p>The latitude.</p>
    #[serde(rename = "Latitude")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latitude: Option<String>,
    /// <p>The longitude.</p>
    #[serde(rename = "Longitude")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub longitude: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct RegisterTransitGatewayRequest {
    /// <p>The ID of the global network.</p>
    #[serde(rename = "GlobalNetworkId")]
    pub global_network_id: String,
    /// <p>The Amazon Resource Name (ARN) of the transit gateway. For more information, see <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/list_amazonec2.html#amazonec2-resources-for-iam-policies">Resources Defined by Amazon EC2</a>.</p>
    #[serde(rename = "TransitGatewayArn")]
    pub transit_gateway_arn: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct RegisterTransitGatewayResponse {
    /// <p>Information about the transit gateway registration.</p>
    #[serde(rename = "TransitGatewayRegistration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transit_gateway_registration: Option<TransitGatewayRegistration>,
}

/// <p>Describes a site.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct Site {
    /// <p>The date and time that the site was created.</p>
    #[serde(rename = "CreatedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<f64>,
    /// <p>The description of the site.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The ID of the global network.</p>
    #[serde(rename = "GlobalNetworkId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub global_network_id: Option<String>,
    /// <p>The location of the site.</p>
    #[serde(rename = "Location")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<Location>,
    /// <p>The Amazon Resource Name (ARN) of the site.</p>
    #[serde(rename = "SiteArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub site_arn: Option<String>,
    /// <p>The ID of the site.</p>
    #[serde(rename = "SiteId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub site_id: Option<String>,
    /// <p>The state of the site.</p>
    #[serde(rename = "State")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    /// <p>The tags for the site.</p>
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

/// <p>Describes a tag.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct Tag {
    /// <p>The tag key.</p> <p>Length Constraints: Maximum length of 128 characters.</p>
    #[serde(rename = "Key")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    /// <p>The tag value.</p> <p>Length Constraints: Maximum length of 256 characters.</p>
    #[serde(rename = "Value")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct TagResourceRequest {
    /// <p>The Amazon Resource Name (ARN) of the resource.</p>
    #[serde(rename = "ResourceArn")]
    pub resource_arn: String,
    /// <p>The tags to apply to the specified resource.</p>
    #[serde(rename = "Tags")]
    pub tags: Vec<Tag>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct TagResourceResponse {}

/// <p>Describes a transit gateway Connect peer association.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct TransitGatewayConnectPeerAssociation {
    /// <p>The ID of the device.</p>
    #[serde(rename = "DeviceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_id: Option<String>,
    /// <p>The ID of the global network.</p>
    #[serde(rename = "GlobalNetworkId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub global_network_id: Option<String>,
    /// <p>The ID of the link.</p>
    #[serde(rename = "LinkId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub link_id: Option<String>,
    /// <p>The state of the association.</p>
    #[serde(rename = "State")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    /// <p>The Amazon Resource Name (ARN) of the transit gateway Connect peer.</p>
    #[serde(rename = "TransitGatewayConnectPeerArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transit_gateway_connect_peer_arn: Option<String>,
}

/// <p>Describes the registration of a transit gateway to a global network.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct TransitGatewayRegistration {
    /// <p>The ID of the global network.</p>
    #[serde(rename = "GlobalNetworkId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub global_network_id: Option<String>,
    /// <p>The state of the transit gateway registration.</p>
    #[serde(rename = "State")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<TransitGatewayRegistrationStateReason>,
    /// <p>The Amazon Resource Name (ARN) of the transit gateway.</p>
    #[serde(rename = "TransitGatewayArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transit_gateway_arn: Option<String>,
}

/// <p>Describes the status of a transit gateway registration.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct TransitGatewayRegistrationStateReason {
    /// <p>The code for the state reason.</p>
    #[serde(rename = "Code")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    /// <p>The message for the state reason.</p>
    #[serde(rename = "Message")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UntagResourceRequest {
    /// <p>The Amazon Resource Name (ARN) of the resource.</p>
    #[serde(rename = "ResourceArn")]
    pub resource_arn: String,
    /// <p>The tag keys to remove from the specified resource.</p>
    #[serde(rename = "TagKeys")]
    pub tag_keys: Vec<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UntagResourceResponse {}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UpdateConnectionRequest {
    /// <p>The ID of the link for the second device in the connection.</p>
    #[serde(rename = "ConnectedLinkId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connected_link_id: Option<String>,
    /// <p>The ID of the connection.</p>
    #[serde(rename = "ConnectionId")]
    pub connection_id: String,
    /// <p>A description of the connection.</p> <p>Length Constraints: Maximum length of 256 characters.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The ID of the global network.</p>
    #[serde(rename = "GlobalNetworkId")]
    pub global_network_id: String,
    /// <p>The ID of the link for the first device in the connection.</p>
    #[serde(rename = "LinkId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub link_id: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UpdateConnectionResponse {
    /// <p>Information about the connection.</p>
    #[serde(rename = "Connection")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connection: Option<Connection>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UpdateDeviceRequest {
    /// <p>The AWS location of the device.</p>
    #[serde(rename = "AWSLocation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aws_location: Option<AWSLocation>,
    /// <p>A description of the device.</p> <p>Length Constraints: Maximum length of 256 characters.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The ID of the device.</p>
    #[serde(rename = "DeviceId")]
    pub device_id: String,
    /// <p>The ID of the global network.</p>
    #[serde(rename = "GlobalNetworkId")]
    pub global_network_id: String,
    #[serde(rename = "Location")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<Location>,
    /// <p>The model of the device.</p> <p>Length Constraints: Maximum length of 128 characters.</p>
    #[serde(rename = "Model")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model: Option<String>,
    /// <p>The serial number of the device.</p> <p>Length Constraints: Maximum length of 128 characters.</p>
    #[serde(rename = "SerialNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub serial_number: Option<String>,
    /// <p>The ID of the site.</p>
    #[serde(rename = "SiteId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub site_id: Option<String>,
    /// <p>The type of the device.</p>
    #[serde(rename = "Type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    /// <p>The vendor of the device.</p> <p>Length Constraints: Maximum length of 128 characters.</p>
    #[serde(rename = "Vendor")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vendor: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UpdateDeviceResponse {
    /// <p>Information about the device.</p>
    #[serde(rename = "Device")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device: Option<Device>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UpdateGlobalNetworkRequest {
    /// <p>A description of the global network.</p> <p>Length Constraints: Maximum length of 256 characters.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The ID of your global network.</p>
    #[serde(rename = "GlobalNetworkId")]
    pub global_network_id: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UpdateGlobalNetworkResponse {
    /// <p>Information about the global network object.</p>
    #[serde(rename = "GlobalNetwork")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub global_network: Option<GlobalNetwork>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UpdateLinkRequest {
    /// <p>The upload and download speed in Mbps. </p>
    #[serde(rename = "Bandwidth")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bandwidth: Option<Bandwidth>,
    /// <p>A description of the link.</p> <p>Length Constraints: Maximum length of 256 characters.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The ID of the global network.</p>
    #[serde(rename = "GlobalNetworkId")]
    pub global_network_id: String,
    /// <p>The ID of the link.</p>
    #[serde(rename = "LinkId")]
    pub link_id: String,
    /// <p>The provider of the link.</p> <p>Length Constraints: Maximum length of 128 characters.</p>
    #[serde(rename = "Provider")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider: Option<String>,
    /// <p>The type of the link.</p> <p>Length Constraints: Maximum length of 128 characters.</p>
    #[serde(rename = "Type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UpdateLinkResponse {
    /// <p>Information about the link.</p>
    #[serde(rename = "Link")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub link: Option<Link>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UpdateSiteRequest {
    /// <p>A description of your site.</p> <p>Length Constraints: Maximum length of 256 characters.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The ID of the global network.</p>
    #[serde(rename = "GlobalNetworkId")]
    pub global_network_id: String,
    /// <p><p>The site location:</p> <ul> <li> <p> <code>Address</code>: The physical address of the site.</p> </li> <li> <p> <code>Latitude</code>: The latitude of the site. </p> </li> <li> <p> <code>Longitude</code>: The longitude of the site.</p> </li> </ul></p>
    #[serde(rename = "Location")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<Location>,
    /// <p>The ID of your site.</p>
    #[serde(rename = "SiteId")]
    pub site_id: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UpdateSiteResponse {
    /// <p>Information about the site.</p>
    #[serde(rename = "Site")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub site: Option<Site>,
}

/// <p>Describes a validation exception for a field.</p>
#[derive(Clone, Debug, Default, PartialEq)]
pub struct ValidationExceptionField {
    /// <p>The message for the field.</p>
    pub message: String,
    /// <p>The name of the field.</p>
    pub name: String,
}

/// Errors returned by AssociateCustomerGateway
#[derive(Debug, PartialEq)]
pub enum AssociateCustomerGatewayError {
    /// <p>You do not have sufficient access to perform this action.</p>
    AccessDenied(String),
    /// <p>There was a conflict processing the request. Updating or deleting the resource can cause an inconsistent state.</p>
    Conflict(String),
    /// <p>The request has failed due to an internal error.</p>
    InternalServer(String),
    /// <p>The specified resource could not be found.</p>
    ResourceNotFound(String),
    /// <p>A service limit was exceeded.</p>
    ServiceQuotaExceeded(String),
    /// <p>The request was denied due to request throttling.</p>
    Throttling(String),
}

impl AssociateCustomerGatewayError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<AssociateCustomerGatewayError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            #[allow(clippy::single_match)]
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(AssociateCustomerGatewayError::AccessDenied(
                        err.msg,
                    ))
                }
                "ConflictException" => {
                    return RusotoError::Service(AssociateCustomerGatewayError::Conflict(err.msg))
                }
                "InternalServerException" => {
                    return RusotoError::Service(AssociateCustomerGatewayError::InternalServer(
                        err.msg,
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(AssociateCustomerGatewayError::ResourceNotFound(
                        err.msg,
                    ))
                }
                "ServiceQuotaExceededException" => {
                    return RusotoError::Service(
                        AssociateCustomerGatewayError::ServiceQuotaExceeded(err.msg),
                    )
                }
                "ThrottlingException" => {
                    return RusotoError::Service(AssociateCustomerGatewayError::Throttling(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for AssociateCustomerGatewayError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            AssociateCustomerGatewayError::AccessDenied(ref cause) => write!(f, "{}", cause),
            AssociateCustomerGatewayError::Conflict(ref cause) => write!(f, "{}", cause),
            AssociateCustomerGatewayError::InternalServer(ref cause) => write!(f, "{}", cause),
            AssociateCustomerGatewayError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            AssociateCustomerGatewayError::ServiceQuotaExceeded(ref cause) => {
                write!(f, "{}", cause)
            }
            AssociateCustomerGatewayError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for AssociateCustomerGatewayError {}
/// Errors returned by AssociateLink
#[derive(Debug, PartialEq)]
pub enum AssociateLinkError {
    /// <p>You do not have sufficient access to perform this action.</p>
    AccessDenied(String),
    /// <p>There was a conflict processing the request. Updating or deleting the resource can cause an inconsistent state.</p>
    Conflict(String),
    /// <p>The request has failed due to an internal error.</p>
    InternalServer(String),
    /// <p>The specified resource could not be found.</p>
    ResourceNotFound(String),
    /// <p>A service limit was exceeded.</p>
    ServiceQuotaExceeded(String),
    /// <p>The request was denied due to request throttling.</p>
    Throttling(String),
}

impl AssociateLinkError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<AssociateLinkError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            #[allow(clippy::single_match)]
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(AssociateLinkError::AccessDenied(err.msg))
                }
                "ConflictException" => {
                    return RusotoError::Service(AssociateLinkError::Conflict(err.msg))
                }
                "InternalServerException" => {
                    return RusotoError::Service(AssociateLinkError::InternalServer(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(AssociateLinkError::ResourceNotFound(err.msg))
                }
                "ServiceQuotaExceededException" => {
                    return RusotoError::Service(AssociateLinkError::ServiceQuotaExceeded(err.msg))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(AssociateLinkError::Throttling(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for AssociateLinkError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            AssociateLinkError::AccessDenied(ref cause) => write!(f, "{}", cause),
            AssociateLinkError::Conflict(ref cause) => write!(f, "{}", cause),
            AssociateLinkError::InternalServer(ref cause) => write!(f, "{}", cause),
            AssociateLinkError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            AssociateLinkError::ServiceQuotaExceeded(ref cause) => write!(f, "{}", cause),
            AssociateLinkError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for AssociateLinkError {}
/// Errors returned by AssociateTransitGatewayConnectPeer
#[derive(Debug, PartialEq)]
pub enum AssociateTransitGatewayConnectPeerError {
    /// <p>You do not have sufficient access to perform this action.</p>
    AccessDenied(String),
    /// <p>There was a conflict processing the request. Updating or deleting the resource can cause an inconsistent state.</p>
    Conflict(String),
    /// <p>The request has failed due to an internal error.</p>
    InternalServer(String),
    /// <p>The specified resource could not be found.</p>
    ResourceNotFound(String),
    /// <p>A service limit was exceeded.</p>
    ServiceQuotaExceeded(String),
    /// <p>The request was denied due to request throttling.</p>
    Throttling(String),
}

impl AssociateTransitGatewayConnectPeerError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<AssociateTransitGatewayConnectPeerError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            #[allow(clippy::single_match)]
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(
                        AssociateTransitGatewayConnectPeerError::AccessDenied(err.msg),
                    )
                }
                "ConflictException" => {
                    return RusotoError::Service(AssociateTransitGatewayConnectPeerError::Conflict(
                        err.msg,
                    ))
                }
                "InternalServerException" => {
                    return RusotoError::Service(
                        AssociateTransitGatewayConnectPeerError::InternalServer(err.msg),
                    )
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(
                        AssociateTransitGatewayConnectPeerError::ResourceNotFound(err.msg),
                    )
                }
                "ServiceQuotaExceededException" => {
                    return RusotoError::Service(
                        AssociateTransitGatewayConnectPeerError::ServiceQuotaExceeded(err.msg),
                    )
                }
                "ThrottlingException" => {
                    return RusotoError::Service(
                        AssociateTransitGatewayConnectPeerError::Throttling(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for AssociateTransitGatewayConnectPeerError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            AssociateTransitGatewayConnectPeerError::AccessDenied(ref cause) => {
                write!(f, "{}", cause)
            }
            AssociateTransitGatewayConnectPeerError::Conflict(ref cause) => write!(f, "{}", cause),
            AssociateTransitGatewayConnectPeerError::InternalServer(ref cause) => {
                write!(f, "{}", cause)
            }
            AssociateTransitGatewayConnectPeerError::ResourceNotFound(ref cause) => {
                write!(f, "{}", cause)
            }
            AssociateTransitGatewayConnectPeerError::ServiceQuotaExceeded(ref cause) => {
                write!(f, "{}", cause)
            }
            AssociateTransitGatewayConnectPeerError::Throttling(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for AssociateTransitGatewayConnectPeerError {}
/// Errors returned by CreateConnection
#[derive(Debug, PartialEq)]
pub enum CreateConnectionError {
    /// <p>You do not have sufficient access to perform this action.</p>
    AccessDenied(String),
    /// <p>There was a conflict processing the request. Updating or deleting the resource can cause an inconsistent state.</p>
    Conflict(String),
    /// <p>The request has failed due to an internal error.</p>
    InternalServer(String),
    /// <p>A service limit was exceeded.</p>
    ServiceQuotaExceeded(String),
    /// <p>The request was denied due to request throttling.</p>
    Throttling(String),
}

impl CreateConnectionError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateConnectionError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            #[allow(clippy::single_match)]
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(CreateConnectionError::AccessDenied(err.msg))
                }
                "ConflictException" => {
                    return RusotoError::Service(CreateConnectionError::Conflict(err.msg))
                }
                "InternalServerException" => {
                    return RusotoError::Service(CreateConnectionError::InternalServer(err.msg))
                }
                "ServiceQuotaExceededException" => {
                    return RusotoError::Service(CreateConnectionError::ServiceQuotaExceeded(
                        err.msg,
                    ))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(CreateConnectionError::Throttling(err.msg))
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
            CreateConnectionError::AccessDenied(ref cause) => write!(f, "{}", cause),
            CreateConnectionError::Conflict(ref cause) => write!(f, "{}", cause),
            CreateConnectionError::InternalServer(ref cause) => write!(f, "{}", cause),
            CreateConnectionError::ServiceQuotaExceeded(ref cause) => write!(f, "{}", cause),
            CreateConnectionError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CreateConnectionError {}
/// Errors returned by CreateDevice
#[derive(Debug, PartialEq)]
pub enum CreateDeviceError {
    /// <p>You do not have sufficient access to perform this action.</p>
    AccessDenied(String),
    /// <p>There was a conflict processing the request. Updating or deleting the resource can cause an inconsistent state.</p>
    Conflict(String),
    /// <p>The request has failed due to an internal error.</p>
    InternalServer(String),
    /// <p>The specified resource could not be found.</p>
    ResourceNotFound(String),
    /// <p>A service limit was exceeded.</p>
    ServiceQuotaExceeded(String),
    /// <p>The request was denied due to request throttling.</p>
    Throttling(String),
}

impl CreateDeviceError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateDeviceError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            #[allow(clippy::single_match)]
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(CreateDeviceError::AccessDenied(err.msg))
                }
                "ConflictException" => {
                    return RusotoError::Service(CreateDeviceError::Conflict(err.msg))
                }
                "InternalServerException" => {
                    return RusotoError::Service(CreateDeviceError::InternalServer(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(CreateDeviceError::ResourceNotFound(err.msg))
                }
                "ServiceQuotaExceededException" => {
                    return RusotoError::Service(CreateDeviceError::ServiceQuotaExceeded(err.msg))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(CreateDeviceError::Throttling(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for CreateDeviceError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreateDeviceError::AccessDenied(ref cause) => write!(f, "{}", cause),
            CreateDeviceError::Conflict(ref cause) => write!(f, "{}", cause),
            CreateDeviceError::InternalServer(ref cause) => write!(f, "{}", cause),
            CreateDeviceError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            CreateDeviceError::ServiceQuotaExceeded(ref cause) => write!(f, "{}", cause),
            CreateDeviceError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CreateDeviceError {}
/// Errors returned by CreateGlobalNetwork
#[derive(Debug, PartialEq)]
pub enum CreateGlobalNetworkError {
    /// <p>You do not have sufficient access to perform this action.</p>
    AccessDenied(String),
    /// <p>There was a conflict processing the request. Updating or deleting the resource can cause an inconsistent state.</p>
    Conflict(String),
    /// <p>The request has failed due to an internal error.</p>
    InternalServer(String),
    /// <p>A service limit was exceeded.</p>
    ServiceQuotaExceeded(String),
    /// <p>The request was denied due to request throttling.</p>
    Throttling(String),
}

impl CreateGlobalNetworkError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateGlobalNetworkError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            #[allow(clippy::single_match)]
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(CreateGlobalNetworkError::AccessDenied(err.msg))
                }
                "ConflictException" => {
                    return RusotoError::Service(CreateGlobalNetworkError::Conflict(err.msg))
                }
                "InternalServerException" => {
                    return RusotoError::Service(CreateGlobalNetworkError::InternalServer(err.msg))
                }
                "ServiceQuotaExceededException" => {
                    return RusotoError::Service(CreateGlobalNetworkError::ServiceQuotaExceeded(
                        err.msg,
                    ))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(CreateGlobalNetworkError::Throttling(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for CreateGlobalNetworkError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreateGlobalNetworkError::AccessDenied(ref cause) => write!(f, "{}", cause),
            CreateGlobalNetworkError::Conflict(ref cause) => write!(f, "{}", cause),
            CreateGlobalNetworkError::InternalServer(ref cause) => write!(f, "{}", cause),
            CreateGlobalNetworkError::ServiceQuotaExceeded(ref cause) => write!(f, "{}", cause),
            CreateGlobalNetworkError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CreateGlobalNetworkError {}
/// Errors returned by CreateLink
#[derive(Debug, PartialEq)]
pub enum CreateLinkError {
    /// <p>You do not have sufficient access to perform this action.</p>
    AccessDenied(String),
    /// <p>There was a conflict processing the request. Updating or deleting the resource can cause an inconsistent state.</p>
    Conflict(String),
    /// <p>The request has failed due to an internal error.</p>
    InternalServer(String),
    /// <p>The specified resource could not be found.</p>
    ResourceNotFound(String),
    /// <p>A service limit was exceeded.</p>
    ServiceQuotaExceeded(String),
    /// <p>The request was denied due to request throttling.</p>
    Throttling(String),
}

impl CreateLinkError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateLinkError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            #[allow(clippy::single_match)]
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(CreateLinkError::AccessDenied(err.msg))
                }
                "ConflictException" => {
                    return RusotoError::Service(CreateLinkError::Conflict(err.msg))
                }
                "InternalServerException" => {
                    return RusotoError::Service(CreateLinkError::InternalServer(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(CreateLinkError::ResourceNotFound(err.msg))
                }
                "ServiceQuotaExceededException" => {
                    return RusotoError::Service(CreateLinkError::ServiceQuotaExceeded(err.msg))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(CreateLinkError::Throttling(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for CreateLinkError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreateLinkError::AccessDenied(ref cause) => write!(f, "{}", cause),
            CreateLinkError::Conflict(ref cause) => write!(f, "{}", cause),
            CreateLinkError::InternalServer(ref cause) => write!(f, "{}", cause),
            CreateLinkError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            CreateLinkError::ServiceQuotaExceeded(ref cause) => write!(f, "{}", cause),
            CreateLinkError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CreateLinkError {}
/// Errors returned by CreateSite
#[derive(Debug, PartialEq)]
pub enum CreateSiteError {
    /// <p>You do not have sufficient access to perform this action.</p>
    AccessDenied(String),
    /// <p>There was a conflict processing the request. Updating or deleting the resource can cause an inconsistent state.</p>
    Conflict(String),
    /// <p>The request has failed due to an internal error.</p>
    InternalServer(String),
    /// <p>The specified resource could not be found.</p>
    ResourceNotFound(String),
    /// <p>A service limit was exceeded.</p>
    ServiceQuotaExceeded(String),
    /// <p>The request was denied due to request throttling.</p>
    Throttling(String),
}

impl CreateSiteError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateSiteError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            #[allow(clippy::single_match)]
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(CreateSiteError::AccessDenied(err.msg))
                }
                "ConflictException" => {
                    return RusotoError::Service(CreateSiteError::Conflict(err.msg))
                }
                "InternalServerException" => {
                    return RusotoError::Service(CreateSiteError::InternalServer(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(CreateSiteError::ResourceNotFound(err.msg))
                }
                "ServiceQuotaExceededException" => {
                    return RusotoError::Service(CreateSiteError::ServiceQuotaExceeded(err.msg))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(CreateSiteError::Throttling(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for CreateSiteError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreateSiteError::AccessDenied(ref cause) => write!(f, "{}", cause),
            CreateSiteError::Conflict(ref cause) => write!(f, "{}", cause),
            CreateSiteError::InternalServer(ref cause) => write!(f, "{}", cause),
            CreateSiteError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            CreateSiteError::ServiceQuotaExceeded(ref cause) => write!(f, "{}", cause),
            CreateSiteError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CreateSiteError {}
/// Errors returned by DeleteConnection
#[derive(Debug, PartialEq)]
pub enum DeleteConnectionError {
    /// <p>You do not have sufficient access to perform this action.</p>
    AccessDenied(String),
    /// <p>There was a conflict processing the request. Updating or deleting the resource can cause an inconsistent state.</p>
    Conflict(String),
    /// <p>The request has failed due to an internal error.</p>
    InternalServer(String),
    /// <p>The specified resource could not be found.</p>
    ResourceNotFound(String),
    /// <p>The request was denied due to request throttling.</p>
    Throttling(String),
}

impl DeleteConnectionError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteConnectionError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            #[allow(clippy::single_match)]
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(DeleteConnectionError::AccessDenied(err.msg))
                }
                "ConflictException" => {
                    return RusotoError::Service(DeleteConnectionError::Conflict(err.msg))
                }
                "InternalServerException" => {
                    return RusotoError::Service(DeleteConnectionError::InternalServer(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(DeleteConnectionError::ResourceNotFound(err.msg))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(DeleteConnectionError::Throttling(err.msg))
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
            DeleteConnectionError::AccessDenied(ref cause) => write!(f, "{}", cause),
            DeleteConnectionError::Conflict(ref cause) => write!(f, "{}", cause),
            DeleteConnectionError::InternalServer(ref cause) => write!(f, "{}", cause),
            DeleteConnectionError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            DeleteConnectionError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeleteConnectionError {}
/// Errors returned by DeleteDevice
#[derive(Debug, PartialEq)]
pub enum DeleteDeviceError {
    /// <p>You do not have sufficient access to perform this action.</p>
    AccessDenied(String),
    /// <p>There was a conflict processing the request. Updating or deleting the resource can cause an inconsistent state.</p>
    Conflict(String),
    /// <p>The request has failed due to an internal error.</p>
    InternalServer(String),
    /// <p>The specified resource could not be found.</p>
    ResourceNotFound(String),
    /// <p>The request was denied due to request throttling.</p>
    Throttling(String),
}

impl DeleteDeviceError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteDeviceError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            #[allow(clippy::single_match)]
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(DeleteDeviceError::AccessDenied(err.msg))
                }
                "ConflictException" => {
                    return RusotoError::Service(DeleteDeviceError::Conflict(err.msg))
                }
                "InternalServerException" => {
                    return RusotoError::Service(DeleteDeviceError::InternalServer(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(DeleteDeviceError::ResourceNotFound(err.msg))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(DeleteDeviceError::Throttling(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DeleteDeviceError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteDeviceError::AccessDenied(ref cause) => write!(f, "{}", cause),
            DeleteDeviceError::Conflict(ref cause) => write!(f, "{}", cause),
            DeleteDeviceError::InternalServer(ref cause) => write!(f, "{}", cause),
            DeleteDeviceError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            DeleteDeviceError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeleteDeviceError {}
/// Errors returned by DeleteGlobalNetwork
#[derive(Debug, PartialEq)]
pub enum DeleteGlobalNetworkError {
    /// <p>You do not have sufficient access to perform this action.</p>
    AccessDenied(String),
    /// <p>There was a conflict processing the request. Updating or deleting the resource can cause an inconsistent state.</p>
    Conflict(String),
    /// <p>The request has failed due to an internal error.</p>
    InternalServer(String),
    /// <p>The specified resource could not be found.</p>
    ResourceNotFound(String),
    /// <p>The request was denied due to request throttling.</p>
    Throttling(String),
}

impl DeleteGlobalNetworkError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteGlobalNetworkError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            #[allow(clippy::single_match)]
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(DeleteGlobalNetworkError::AccessDenied(err.msg))
                }
                "ConflictException" => {
                    return RusotoError::Service(DeleteGlobalNetworkError::Conflict(err.msg))
                }
                "InternalServerException" => {
                    return RusotoError::Service(DeleteGlobalNetworkError::InternalServer(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(DeleteGlobalNetworkError::ResourceNotFound(
                        err.msg,
                    ))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(DeleteGlobalNetworkError::Throttling(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DeleteGlobalNetworkError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteGlobalNetworkError::AccessDenied(ref cause) => write!(f, "{}", cause),
            DeleteGlobalNetworkError::Conflict(ref cause) => write!(f, "{}", cause),
            DeleteGlobalNetworkError::InternalServer(ref cause) => write!(f, "{}", cause),
            DeleteGlobalNetworkError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            DeleteGlobalNetworkError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeleteGlobalNetworkError {}
/// Errors returned by DeleteLink
#[derive(Debug, PartialEq)]
pub enum DeleteLinkError {
    /// <p>You do not have sufficient access to perform this action.</p>
    AccessDenied(String),
    /// <p>There was a conflict processing the request. Updating or deleting the resource can cause an inconsistent state.</p>
    Conflict(String),
    /// <p>The request has failed due to an internal error.</p>
    InternalServer(String),
    /// <p>The specified resource could not be found.</p>
    ResourceNotFound(String),
    /// <p>The request was denied due to request throttling.</p>
    Throttling(String),
}

impl DeleteLinkError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteLinkError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            #[allow(clippy::single_match)]
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(DeleteLinkError::AccessDenied(err.msg))
                }
                "ConflictException" => {
                    return RusotoError::Service(DeleteLinkError::Conflict(err.msg))
                }
                "InternalServerException" => {
                    return RusotoError::Service(DeleteLinkError::InternalServer(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(DeleteLinkError::ResourceNotFound(err.msg))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(DeleteLinkError::Throttling(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DeleteLinkError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteLinkError::AccessDenied(ref cause) => write!(f, "{}", cause),
            DeleteLinkError::Conflict(ref cause) => write!(f, "{}", cause),
            DeleteLinkError::InternalServer(ref cause) => write!(f, "{}", cause),
            DeleteLinkError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            DeleteLinkError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeleteLinkError {}
/// Errors returned by DeleteSite
#[derive(Debug, PartialEq)]
pub enum DeleteSiteError {
    /// <p>You do not have sufficient access to perform this action.</p>
    AccessDenied(String),
    /// <p>There was a conflict processing the request. Updating or deleting the resource can cause an inconsistent state.</p>
    Conflict(String),
    /// <p>The request has failed due to an internal error.</p>
    InternalServer(String),
    /// <p>The specified resource could not be found.</p>
    ResourceNotFound(String),
    /// <p>The request was denied due to request throttling.</p>
    Throttling(String),
}

impl DeleteSiteError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteSiteError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            #[allow(clippy::single_match)]
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(DeleteSiteError::AccessDenied(err.msg))
                }
                "ConflictException" => {
                    return RusotoError::Service(DeleteSiteError::Conflict(err.msg))
                }
                "InternalServerException" => {
                    return RusotoError::Service(DeleteSiteError::InternalServer(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(DeleteSiteError::ResourceNotFound(err.msg))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(DeleteSiteError::Throttling(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DeleteSiteError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteSiteError::AccessDenied(ref cause) => write!(f, "{}", cause),
            DeleteSiteError::Conflict(ref cause) => write!(f, "{}", cause),
            DeleteSiteError::InternalServer(ref cause) => write!(f, "{}", cause),
            DeleteSiteError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            DeleteSiteError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeleteSiteError {}
/// Errors returned by DeregisterTransitGateway
#[derive(Debug, PartialEq)]
pub enum DeregisterTransitGatewayError {
    /// <p>You do not have sufficient access to perform this action.</p>
    AccessDenied(String),
    /// <p>There was a conflict processing the request. Updating or deleting the resource can cause an inconsistent state.</p>
    Conflict(String),
    /// <p>The request has failed due to an internal error.</p>
    InternalServer(String),
    /// <p>The specified resource could not be found.</p>
    ResourceNotFound(String),
    /// <p>The request was denied due to request throttling.</p>
    Throttling(String),
}

impl DeregisterTransitGatewayError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeregisterTransitGatewayError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            #[allow(clippy::single_match)]
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(DeregisterTransitGatewayError::AccessDenied(
                        err.msg,
                    ))
                }
                "ConflictException" => {
                    return RusotoError::Service(DeregisterTransitGatewayError::Conflict(err.msg))
                }
                "InternalServerException" => {
                    return RusotoError::Service(DeregisterTransitGatewayError::InternalServer(
                        err.msg,
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(DeregisterTransitGatewayError::ResourceNotFound(
                        err.msg,
                    ))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(DeregisterTransitGatewayError::Throttling(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DeregisterTransitGatewayError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeregisterTransitGatewayError::AccessDenied(ref cause) => write!(f, "{}", cause),
            DeregisterTransitGatewayError::Conflict(ref cause) => write!(f, "{}", cause),
            DeregisterTransitGatewayError::InternalServer(ref cause) => write!(f, "{}", cause),
            DeregisterTransitGatewayError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            DeregisterTransitGatewayError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeregisterTransitGatewayError {}
/// Errors returned by DescribeGlobalNetworks
#[derive(Debug, PartialEq)]
pub enum DescribeGlobalNetworksError {
    /// <p>You do not have sufficient access to perform this action.</p>
    AccessDenied(String),
    /// <p>The request has failed due to an internal error.</p>
    InternalServer(String),
    /// <p>The specified resource could not be found.</p>
    ResourceNotFound(String),
    /// <p>The request was denied due to request throttling.</p>
    Throttling(String),
}

impl DescribeGlobalNetworksError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeGlobalNetworksError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            #[allow(clippy::single_match)]
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(DescribeGlobalNetworksError::AccessDenied(err.msg))
                }
                "InternalServerException" => {
                    return RusotoError::Service(DescribeGlobalNetworksError::InternalServer(
                        err.msg,
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(DescribeGlobalNetworksError::ResourceNotFound(
                        err.msg,
                    ))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(DescribeGlobalNetworksError::Throttling(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DescribeGlobalNetworksError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeGlobalNetworksError::AccessDenied(ref cause) => write!(f, "{}", cause),
            DescribeGlobalNetworksError::InternalServer(ref cause) => write!(f, "{}", cause),
            DescribeGlobalNetworksError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            DescribeGlobalNetworksError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DescribeGlobalNetworksError {}
/// Errors returned by DisassociateCustomerGateway
#[derive(Debug, PartialEq)]
pub enum DisassociateCustomerGatewayError {
    /// <p>You do not have sufficient access to perform this action.</p>
    AccessDenied(String),
    /// <p>There was a conflict processing the request. Updating or deleting the resource can cause an inconsistent state.</p>
    Conflict(String),
    /// <p>The request has failed due to an internal error.</p>
    InternalServer(String),
    /// <p>The specified resource could not be found.</p>
    ResourceNotFound(String),
    /// <p>The request was denied due to request throttling.</p>
    Throttling(String),
}

impl DisassociateCustomerGatewayError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<DisassociateCustomerGatewayError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            #[allow(clippy::single_match)]
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(DisassociateCustomerGatewayError::AccessDenied(
                        err.msg,
                    ))
                }
                "ConflictException" => {
                    return RusotoError::Service(DisassociateCustomerGatewayError::Conflict(
                        err.msg,
                    ))
                }
                "InternalServerException" => {
                    return RusotoError::Service(DisassociateCustomerGatewayError::InternalServer(
                        err.msg,
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(
                        DisassociateCustomerGatewayError::ResourceNotFound(err.msg),
                    )
                }
                "ThrottlingException" => {
                    return RusotoError::Service(DisassociateCustomerGatewayError::Throttling(
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
impl fmt::Display for DisassociateCustomerGatewayError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DisassociateCustomerGatewayError::AccessDenied(ref cause) => write!(f, "{}", cause),
            DisassociateCustomerGatewayError::Conflict(ref cause) => write!(f, "{}", cause),
            DisassociateCustomerGatewayError::InternalServer(ref cause) => write!(f, "{}", cause),
            DisassociateCustomerGatewayError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            DisassociateCustomerGatewayError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DisassociateCustomerGatewayError {}
/// Errors returned by DisassociateLink
#[derive(Debug, PartialEq)]
pub enum DisassociateLinkError {
    /// <p>You do not have sufficient access to perform this action.</p>
    AccessDenied(String),
    /// <p>There was a conflict processing the request. Updating or deleting the resource can cause an inconsistent state.</p>
    Conflict(String),
    /// <p>The request has failed due to an internal error.</p>
    InternalServer(String),
    /// <p>The specified resource could not be found.</p>
    ResourceNotFound(String),
    /// <p>The request was denied due to request throttling.</p>
    Throttling(String),
}

impl DisassociateLinkError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DisassociateLinkError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            #[allow(clippy::single_match)]
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(DisassociateLinkError::AccessDenied(err.msg))
                }
                "ConflictException" => {
                    return RusotoError::Service(DisassociateLinkError::Conflict(err.msg))
                }
                "InternalServerException" => {
                    return RusotoError::Service(DisassociateLinkError::InternalServer(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(DisassociateLinkError::ResourceNotFound(err.msg))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(DisassociateLinkError::Throttling(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DisassociateLinkError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DisassociateLinkError::AccessDenied(ref cause) => write!(f, "{}", cause),
            DisassociateLinkError::Conflict(ref cause) => write!(f, "{}", cause),
            DisassociateLinkError::InternalServer(ref cause) => write!(f, "{}", cause),
            DisassociateLinkError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            DisassociateLinkError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DisassociateLinkError {}
/// Errors returned by DisassociateTransitGatewayConnectPeer
#[derive(Debug, PartialEq)]
pub enum DisassociateTransitGatewayConnectPeerError {
    /// <p>You do not have sufficient access to perform this action.</p>
    AccessDenied(String),
    /// <p>There was a conflict processing the request. Updating or deleting the resource can cause an inconsistent state.</p>
    Conflict(String),
    /// <p>The request has failed due to an internal error.</p>
    InternalServer(String),
    /// <p>The specified resource could not be found.</p>
    ResourceNotFound(String),
    /// <p>The request was denied due to request throttling.</p>
    Throttling(String),
}

impl DisassociateTransitGatewayConnectPeerError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<DisassociateTransitGatewayConnectPeerError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            #[allow(clippy::single_match)]
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(
                        DisassociateTransitGatewayConnectPeerError::AccessDenied(err.msg),
                    )
                }
                "ConflictException" => {
                    return RusotoError::Service(
                        DisassociateTransitGatewayConnectPeerError::Conflict(err.msg),
                    )
                }
                "InternalServerException" => {
                    return RusotoError::Service(
                        DisassociateTransitGatewayConnectPeerError::InternalServer(err.msg),
                    )
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(
                        DisassociateTransitGatewayConnectPeerError::ResourceNotFound(err.msg),
                    )
                }
                "ThrottlingException" => {
                    return RusotoError::Service(
                        DisassociateTransitGatewayConnectPeerError::Throttling(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DisassociateTransitGatewayConnectPeerError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DisassociateTransitGatewayConnectPeerError::AccessDenied(ref cause) => {
                write!(f, "{}", cause)
            }
            DisassociateTransitGatewayConnectPeerError::Conflict(ref cause) => {
                write!(f, "{}", cause)
            }
            DisassociateTransitGatewayConnectPeerError::InternalServer(ref cause) => {
                write!(f, "{}", cause)
            }
            DisassociateTransitGatewayConnectPeerError::ResourceNotFound(ref cause) => {
                write!(f, "{}", cause)
            }
            DisassociateTransitGatewayConnectPeerError::Throttling(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for DisassociateTransitGatewayConnectPeerError {}
/// Errors returned by GetConnections
#[derive(Debug, PartialEq)]
pub enum GetConnectionsError {
    /// <p>You do not have sufficient access to perform this action.</p>
    AccessDenied(String),
    /// <p>The request has failed due to an internal error.</p>
    InternalServer(String),
    /// <p>The specified resource could not be found.</p>
    ResourceNotFound(String),
    /// <p>The request was denied due to request throttling.</p>
    Throttling(String),
}

impl GetConnectionsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetConnectionsError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            #[allow(clippy::single_match)]
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(GetConnectionsError::AccessDenied(err.msg))
                }
                "InternalServerException" => {
                    return RusotoError::Service(GetConnectionsError::InternalServer(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(GetConnectionsError::ResourceNotFound(err.msg))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(GetConnectionsError::Throttling(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for GetConnectionsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetConnectionsError::AccessDenied(ref cause) => write!(f, "{}", cause),
            GetConnectionsError::InternalServer(ref cause) => write!(f, "{}", cause),
            GetConnectionsError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            GetConnectionsError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetConnectionsError {}
/// Errors returned by GetCustomerGatewayAssociations
#[derive(Debug, PartialEq)]
pub enum GetCustomerGatewayAssociationsError {
    /// <p>You do not have sufficient access to perform this action.</p>
    AccessDenied(String),
    /// <p>There was a conflict processing the request. Updating or deleting the resource can cause an inconsistent state.</p>
    Conflict(String),
    /// <p>The request has failed due to an internal error.</p>
    InternalServer(String),
    /// <p>The specified resource could not be found.</p>
    ResourceNotFound(String),
    /// <p>The request was denied due to request throttling.</p>
    Throttling(String),
}

impl GetCustomerGatewayAssociationsError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<GetCustomerGatewayAssociationsError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            #[allow(clippy::single_match)]
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(GetCustomerGatewayAssociationsError::AccessDenied(
                        err.msg,
                    ))
                }
                "ConflictException" => {
                    return RusotoError::Service(GetCustomerGatewayAssociationsError::Conflict(
                        err.msg,
                    ))
                }
                "InternalServerException" => {
                    return RusotoError::Service(
                        GetCustomerGatewayAssociationsError::InternalServer(err.msg),
                    )
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(
                        GetCustomerGatewayAssociationsError::ResourceNotFound(err.msg),
                    )
                }
                "ThrottlingException" => {
                    return RusotoError::Service(GetCustomerGatewayAssociationsError::Throttling(
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
impl fmt::Display for GetCustomerGatewayAssociationsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetCustomerGatewayAssociationsError::AccessDenied(ref cause) => write!(f, "{}", cause),
            GetCustomerGatewayAssociationsError::Conflict(ref cause) => write!(f, "{}", cause),
            GetCustomerGatewayAssociationsError::InternalServer(ref cause) => {
                write!(f, "{}", cause)
            }
            GetCustomerGatewayAssociationsError::ResourceNotFound(ref cause) => {
                write!(f, "{}", cause)
            }
            GetCustomerGatewayAssociationsError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetCustomerGatewayAssociationsError {}
/// Errors returned by GetDevices
#[derive(Debug, PartialEq)]
pub enum GetDevicesError {
    /// <p>You do not have sufficient access to perform this action.</p>
    AccessDenied(String),
    /// <p>The request has failed due to an internal error.</p>
    InternalServer(String),
    /// <p>The specified resource could not be found.</p>
    ResourceNotFound(String),
    /// <p>The request was denied due to request throttling.</p>
    Throttling(String),
}

impl GetDevicesError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetDevicesError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            #[allow(clippy::single_match)]
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(GetDevicesError::AccessDenied(err.msg))
                }
                "InternalServerException" => {
                    return RusotoError::Service(GetDevicesError::InternalServer(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(GetDevicesError::ResourceNotFound(err.msg))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(GetDevicesError::Throttling(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for GetDevicesError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetDevicesError::AccessDenied(ref cause) => write!(f, "{}", cause),
            GetDevicesError::InternalServer(ref cause) => write!(f, "{}", cause),
            GetDevicesError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            GetDevicesError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetDevicesError {}
/// Errors returned by GetLinkAssociations
#[derive(Debug, PartialEq)]
pub enum GetLinkAssociationsError {
    /// <p>You do not have sufficient access to perform this action.</p>
    AccessDenied(String),
    /// <p>The request has failed due to an internal error.</p>
    InternalServer(String),
    /// <p>The specified resource could not be found.</p>
    ResourceNotFound(String),
    /// <p>The request was denied due to request throttling.</p>
    Throttling(String),
}

impl GetLinkAssociationsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetLinkAssociationsError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            #[allow(clippy::single_match)]
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(GetLinkAssociationsError::AccessDenied(err.msg))
                }
                "InternalServerException" => {
                    return RusotoError::Service(GetLinkAssociationsError::InternalServer(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(GetLinkAssociationsError::ResourceNotFound(
                        err.msg,
                    ))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(GetLinkAssociationsError::Throttling(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for GetLinkAssociationsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetLinkAssociationsError::AccessDenied(ref cause) => write!(f, "{}", cause),
            GetLinkAssociationsError::InternalServer(ref cause) => write!(f, "{}", cause),
            GetLinkAssociationsError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            GetLinkAssociationsError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetLinkAssociationsError {}
/// Errors returned by GetLinks
#[derive(Debug, PartialEq)]
pub enum GetLinksError {
    /// <p>You do not have sufficient access to perform this action.</p>
    AccessDenied(String),
    /// <p>The request has failed due to an internal error.</p>
    InternalServer(String),
    /// <p>The specified resource could not be found.</p>
    ResourceNotFound(String),
    /// <p>The request was denied due to request throttling.</p>
    Throttling(String),
}

impl GetLinksError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetLinksError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            #[allow(clippy::single_match)]
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(GetLinksError::AccessDenied(err.msg))
                }
                "InternalServerException" => {
                    return RusotoError::Service(GetLinksError::InternalServer(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(GetLinksError::ResourceNotFound(err.msg))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(GetLinksError::Throttling(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for GetLinksError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetLinksError::AccessDenied(ref cause) => write!(f, "{}", cause),
            GetLinksError::InternalServer(ref cause) => write!(f, "{}", cause),
            GetLinksError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            GetLinksError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetLinksError {}
/// Errors returned by GetSites
#[derive(Debug, PartialEq)]
pub enum GetSitesError {
    /// <p>You do not have sufficient access to perform this action.</p>
    AccessDenied(String),
    /// <p>The request has failed due to an internal error.</p>
    InternalServer(String),
    /// <p>The specified resource could not be found.</p>
    ResourceNotFound(String),
    /// <p>The request was denied due to request throttling.</p>
    Throttling(String),
}

impl GetSitesError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetSitesError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            #[allow(clippy::single_match)]
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(GetSitesError::AccessDenied(err.msg))
                }
                "InternalServerException" => {
                    return RusotoError::Service(GetSitesError::InternalServer(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(GetSitesError::ResourceNotFound(err.msg))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(GetSitesError::Throttling(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for GetSitesError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetSitesError::AccessDenied(ref cause) => write!(f, "{}", cause),
            GetSitesError::InternalServer(ref cause) => write!(f, "{}", cause),
            GetSitesError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            GetSitesError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetSitesError {}
/// Errors returned by GetTransitGatewayConnectPeerAssociations
#[derive(Debug, PartialEq)]
pub enum GetTransitGatewayConnectPeerAssociationsError {
    /// <p>You do not have sufficient access to perform this action.</p>
    AccessDenied(String),
    /// <p>There was a conflict processing the request. Updating or deleting the resource can cause an inconsistent state.</p>
    Conflict(String),
    /// <p>The request has failed due to an internal error.</p>
    InternalServer(String),
    /// <p>The specified resource could not be found.</p>
    ResourceNotFound(String),
    /// <p>The request was denied due to request throttling.</p>
    Throttling(String),
}

impl GetTransitGatewayConnectPeerAssociationsError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<GetTransitGatewayConnectPeerAssociationsError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            #[allow(clippy::single_match)]
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(
                        GetTransitGatewayConnectPeerAssociationsError::AccessDenied(err.msg),
                    )
                }
                "ConflictException" => {
                    return RusotoError::Service(
                        GetTransitGatewayConnectPeerAssociationsError::Conflict(err.msg),
                    )
                }
                "InternalServerException" => {
                    return RusotoError::Service(
                        GetTransitGatewayConnectPeerAssociationsError::InternalServer(err.msg),
                    )
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(
                        GetTransitGatewayConnectPeerAssociationsError::ResourceNotFound(err.msg),
                    )
                }
                "ThrottlingException" => {
                    return RusotoError::Service(
                        GetTransitGatewayConnectPeerAssociationsError::Throttling(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for GetTransitGatewayConnectPeerAssociationsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetTransitGatewayConnectPeerAssociationsError::AccessDenied(ref cause) => {
                write!(f, "{}", cause)
            }
            GetTransitGatewayConnectPeerAssociationsError::Conflict(ref cause) => {
                write!(f, "{}", cause)
            }
            GetTransitGatewayConnectPeerAssociationsError::InternalServer(ref cause) => {
                write!(f, "{}", cause)
            }
            GetTransitGatewayConnectPeerAssociationsError::ResourceNotFound(ref cause) => {
                write!(f, "{}", cause)
            }
            GetTransitGatewayConnectPeerAssociationsError::Throttling(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for GetTransitGatewayConnectPeerAssociationsError {}
/// Errors returned by GetTransitGatewayRegistrations
#[derive(Debug, PartialEq)]
pub enum GetTransitGatewayRegistrationsError {
    /// <p>You do not have sufficient access to perform this action.</p>
    AccessDenied(String),
    /// <p>The request has failed due to an internal error.</p>
    InternalServer(String),
    /// <p>The specified resource could not be found.</p>
    ResourceNotFound(String),
    /// <p>The request was denied due to request throttling.</p>
    Throttling(String),
}

impl GetTransitGatewayRegistrationsError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<GetTransitGatewayRegistrationsError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            #[allow(clippy::single_match)]
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(GetTransitGatewayRegistrationsError::AccessDenied(
                        err.msg,
                    ))
                }
                "InternalServerException" => {
                    return RusotoError::Service(
                        GetTransitGatewayRegistrationsError::InternalServer(err.msg),
                    )
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(
                        GetTransitGatewayRegistrationsError::ResourceNotFound(err.msg),
                    )
                }
                "ThrottlingException" => {
                    return RusotoError::Service(GetTransitGatewayRegistrationsError::Throttling(
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
impl fmt::Display for GetTransitGatewayRegistrationsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetTransitGatewayRegistrationsError::AccessDenied(ref cause) => write!(f, "{}", cause),
            GetTransitGatewayRegistrationsError::InternalServer(ref cause) => {
                write!(f, "{}", cause)
            }
            GetTransitGatewayRegistrationsError::ResourceNotFound(ref cause) => {
                write!(f, "{}", cause)
            }
            GetTransitGatewayRegistrationsError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetTransitGatewayRegistrationsError {}
/// Errors returned by ListTagsForResource
#[derive(Debug, PartialEq)]
pub enum ListTagsForResourceError {
    /// <p>You do not have sufficient access to perform this action.</p>
    AccessDenied(String),
    /// <p>The request has failed due to an internal error.</p>
    InternalServer(String),
    /// <p>The specified resource could not be found.</p>
    ResourceNotFound(String),
    /// <p>The request was denied due to request throttling.</p>
    Throttling(String),
}

impl ListTagsForResourceError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListTagsForResourceError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            #[allow(clippy::single_match)]
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(ListTagsForResourceError::AccessDenied(err.msg))
                }
                "InternalServerException" => {
                    return RusotoError::Service(ListTagsForResourceError::InternalServer(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(ListTagsForResourceError::ResourceNotFound(
                        err.msg,
                    ))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(ListTagsForResourceError::Throttling(err.msg))
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
            ListTagsForResourceError::AccessDenied(ref cause) => write!(f, "{}", cause),
            ListTagsForResourceError::InternalServer(ref cause) => write!(f, "{}", cause),
            ListTagsForResourceError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            ListTagsForResourceError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListTagsForResourceError {}
/// Errors returned by RegisterTransitGateway
#[derive(Debug, PartialEq)]
pub enum RegisterTransitGatewayError {
    /// <p>You do not have sufficient access to perform this action.</p>
    AccessDenied(String),
    /// <p>There was a conflict processing the request. Updating or deleting the resource can cause an inconsistent state.</p>
    Conflict(String),
    /// <p>The request has failed due to an internal error.</p>
    InternalServer(String),
    /// <p>The specified resource could not be found.</p>
    ResourceNotFound(String),
    /// <p>The request was denied due to request throttling.</p>
    Throttling(String),
}

impl RegisterTransitGatewayError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<RegisterTransitGatewayError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            #[allow(clippy::single_match)]
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(RegisterTransitGatewayError::AccessDenied(err.msg))
                }
                "ConflictException" => {
                    return RusotoError::Service(RegisterTransitGatewayError::Conflict(err.msg))
                }
                "InternalServerException" => {
                    return RusotoError::Service(RegisterTransitGatewayError::InternalServer(
                        err.msg,
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(RegisterTransitGatewayError::ResourceNotFound(
                        err.msg,
                    ))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(RegisterTransitGatewayError::Throttling(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for RegisterTransitGatewayError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            RegisterTransitGatewayError::AccessDenied(ref cause) => write!(f, "{}", cause),
            RegisterTransitGatewayError::Conflict(ref cause) => write!(f, "{}", cause),
            RegisterTransitGatewayError::InternalServer(ref cause) => write!(f, "{}", cause),
            RegisterTransitGatewayError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            RegisterTransitGatewayError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for RegisterTransitGatewayError {}
/// Errors returned by TagResource
#[derive(Debug, PartialEq)]
pub enum TagResourceError {
    /// <p>You do not have sufficient access to perform this action.</p>
    AccessDenied(String),
    /// <p>There was a conflict processing the request. Updating or deleting the resource can cause an inconsistent state.</p>
    Conflict(String),
    /// <p>The request has failed due to an internal error.</p>
    InternalServer(String),
    /// <p>The specified resource could not be found.</p>
    ResourceNotFound(String),
    /// <p>A service limit was exceeded.</p>
    ServiceQuotaExceeded(String),
    /// <p>The request was denied due to request throttling.</p>
    Throttling(String),
}

impl TagResourceError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<TagResourceError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            #[allow(clippy::single_match)]
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(TagResourceError::AccessDenied(err.msg))
                }
                "ConflictException" => {
                    return RusotoError::Service(TagResourceError::Conflict(err.msg))
                }
                "InternalServerException" => {
                    return RusotoError::Service(TagResourceError::InternalServer(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(TagResourceError::ResourceNotFound(err.msg))
                }
                "ServiceQuotaExceededException" => {
                    return RusotoError::Service(TagResourceError::ServiceQuotaExceeded(err.msg))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(TagResourceError::Throttling(err.msg))
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
            TagResourceError::AccessDenied(ref cause) => write!(f, "{}", cause),
            TagResourceError::Conflict(ref cause) => write!(f, "{}", cause),
            TagResourceError::InternalServer(ref cause) => write!(f, "{}", cause),
            TagResourceError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            TagResourceError::ServiceQuotaExceeded(ref cause) => write!(f, "{}", cause),
            TagResourceError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for TagResourceError {}
/// Errors returned by UntagResource
#[derive(Debug, PartialEq)]
pub enum UntagResourceError {
    /// <p>You do not have sufficient access to perform this action.</p>
    AccessDenied(String),
    /// <p>There was a conflict processing the request. Updating or deleting the resource can cause an inconsistent state.</p>
    Conflict(String),
    /// <p>The request has failed due to an internal error.</p>
    InternalServer(String),
    /// <p>The specified resource could not be found.</p>
    ResourceNotFound(String),
    /// <p>The request was denied due to request throttling.</p>
    Throttling(String),
}

impl UntagResourceError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UntagResourceError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            #[allow(clippy::single_match)]
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(UntagResourceError::AccessDenied(err.msg))
                }
                "ConflictException" => {
                    return RusotoError::Service(UntagResourceError::Conflict(err.msg))
                }
                "InternalServerException" => {
                    return RusotoError::Service(UntagResourceError::InternalServer(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(UntagResourceError::ResourceNotFound(err.msg))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(UntagResourceError::Throttling(err.msg))
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
            UntagResourceError::AccessDenied(ref cause) => write!(f, "{}", cause),
            UntagResourceError::Conflict(ref cause) => write!(f, "{}", cause),
            UntagResourceError::InternalServer(ref cause) => write!(f, "{}", cause),
            UntagResourceError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            UntagResourceError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for UntagResourceError {}
/// Errors returned by UpdateConnection
#[derive(Debug, PartialEq)]
pub enum UpdateConnectionError {
    /// <p>You do not have sufficient access to perform this action.</p>
    AccessDenied(String),
    /// <p>There was a conflict processing the request. Updating or deleting the resource can cause an inconsistent state.</p>
    Conflict(String),
    /// <p>The request has failed due to an internal error.</p>
    InternalServer(String),
    /// <p>The specified resource could not be found.</p>
    ResourceNotFound(String),
    /// <p>The request was denied due to request throttling.</p>
    Throttling(String),
}

impl UpdateConnectionError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdateConnectionError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            #[allow(clippy::single_match)]
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(UpdateConnectionError::AccessDenied(err.msg))
                }
                "ConflictException" => {
                    return RusotoError::Service(UpdateConnectionError::Conflict(err.msg))
                }
                "InternalServerException" => {
                    return RusotoError::Service(UpdateConnectionError::InternalServer(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(UpdateConnectionError::ResourceNotFound(err.msg))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(UpdateConnectionError::Throttling(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for UpdateConnectionError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UpdateConnectionError::AccessDenied(ref cause) => write!(f, "{}", cause),
            UpdateConnectionError::Conflict(ref cause) => write!(f, "{}", cause),
            UpdateConnectionError::InternalServer(ref cause) => write!(f, "{}", cause),
            UpdateConnectionError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            UpdateConnectionError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for UpdateConnectionError {}
/// Errors returned by UpdateDevice
#[derive(Debug, PartialEq)]
pub enum UpdateDeviceError {
    /// <p>You do not have sufficient access to perform this action.</p>
    AccessDenied(String),
    /// <p>There was a conflict processing the request. Updating or deleting the resource can cause an inconsistent state.</p>
    Conflict(String),
    /// <p>The request has failed due to an internal error.</p>
    InternalServer(String),
    /// <p>The specified resource could not be found.</p>
    ResourceNotFound(String),
    /// <p>The request was denied due to request throttling.</p>
    Throttling(String),
}

impl UpdateDeviceError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdateDeviceError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            #[allow(clippy::single_match)]
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(UpdateDeviceError::AccessDenied(err.msg))
                }
                "ConflictException" => {
                    return RusotoError::Service(UpdateDeviceError::Conflict(err.msg))
                }
                "InternalServerException" => {
                    return RusotoError::Service(UpdateDeviceError::InternalServer(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(UpdateDeviceError::ResourceNotFound(err.msg))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(UpdateDeviceError::Throttling(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for UpdateDeviceError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UpdateDeviceError::AccessDenied(ref cause) => write!(f, "{}", cause),
            UpdateDeviceError::Conflict(ref cause) => write!(f, "{}", cause),
            UpdateDeviceError::InternalServer(ref cause) => write!(f, "{}", cause),
            UpdateDeviceError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            UpdateDeviceError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for UpdateDeviceError {}
/// Errors returned by UpdateGlobalNetwork
#[derive(Debug, PartialEq)]
pub enum UpdateGlobalNetworkError {
    /// <p>You do not have sufficient access to perform this action.</p>
    AccessDenied(String),
    /// <p>There was a conflict processing the request. Updating or deleting the resource can cause an inconsistent state.</p>
    Conflict(String),
    /// <p>The request has failed due to an internal error.</p>
    InternalServer(String),
    /// <p>The specified resource could not be found.</p>
    ResourceNotFound(String),
    /// <p>The request was denied due to request throttling.</p>
    Throttling(String),
}

impl UpdateGlobalNetworkError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdateGlobalNetworkError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            #[allow(clippy::single_match)]
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(UpdateGlobalNetworkError::AccessDenied(err.msg))
                }
                "ConflictException" => {
                    return RusotoError::Service(UpdateGlobalNetworkError::Conflict(err.msg))
                }
                "InternalServerException" => {
                    return RusotoError::Service(UpdateGlobalNetworkError::InternalServer(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(UpdateGlobalNetworkError::ResourceNotFound(
                        err.msg,
                    ))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(UpdateGlobalNetworkError::Throttling(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for UpdateGlobalNetworkError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UpdateGlobalNetworkError::AccessDenied(ref cause) => write!(f, "{}", cause),
            UpdateGlobalNetworkError::Conflict(ref cause) => write!(f, "{}", cause),
            UpdateGlobalNetworkError::InternalServer(ref cause) => write!(f, "{}", cause),
            UpdateGlobalNetworkError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            UpdateGlobalNetworkError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for UpdateGlobalNetworkError {}
/// Errors returned by UpdateLink
#[derive(Debug, PartialEq)]
pub enum UpdateLinkError {
    /// <p>You do not have sufficient access to perform this action.</p>
    AccessDenied(String),
    /// <p>There was a conflict processing the request. Updating or deleting the resource can cause an inconsistent state.</p>
    Conflict(String),
    /// <p>The request has failed due to an internal error.</p>
    InternalServer(String),
    /// <p>The specified resource could not be found.</p>
    ResourceNotFound(String),
    /// <p>A service limit was exceeded.</p>
    ServiceQuotaExceeded(String),
    /// <p>The request was denied due to request throttling.</p>
    Throttling(String),
}

impl UpdateLinkError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdateLinkError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            #[allow(clippy::single_match)]
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(UpdateLinkError::AccessDenied(err.msg))
                }
                "ConflictException" => {
                    return RusotoError::Service(UpdateLinkError::Conflict(err.msg))
                }
                "InternalServerException" => {
                    return RusotoError::Service(UpdateLinkError::InternalServer(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(UpdateLinkError::ResourceNotFound(err.msg))
                }
                "ServiceQuotaExceededException" => {
                    return RusotoError::Service(UpdateLinkError::ServiceQuotaExceeded(err.msg))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(UpdateLinkError::Throttling(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for UpdateLinkError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UpdateLinkError::AccessDenied(ref cause) => write!(f, "{}", cause),
            UpdateLinkError::Conflict(ref cause) => write!(f, "{}", cause),
            UpdateLinkError::InternalServer(ref cause) => write!(f, "{}", cause),
            UpdateLinkError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            UpdateLinkError::ServiceQuotaExceeded(ref cause) => write!(f, "{}", cause),
            UpdateLinkError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for UpdateLinkError {}
/// Errors returned by UpdateSite
#[derive(Debug, PartialEq)]
pub enum UpdateSiteError {
    /// <p>You do not have sufficient access to perform this action.</p>
    AccessDenied(String),
    /// <p>There was a conflict processing the request. Updating or deleting the resource can cause an inconsistent state.</p>
    Conflict(String),
    /// <p>The request has failed due to an internal error.</p>
    InternalServer(String),
    /// <p>The specified resource could not be found.</p>
    ResourceNotFound(String),
    /// <p>The request was denied due to request throttling.</p>
    Throttling(String),
}

impl UpdateSiteError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdateSiteError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            #[allow(clippy::single_match)]
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(UpdateSiteError::AccessDenied(err.msg))
                }
                "ConflictException" => {
                    return RusotoError::Service(UpdateSiteError::Conflict(err.msg))
                }
                "InternalServerException" => {
                    return RusotoError::Service(UpdateSiteError::InternalServer(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(UpdateSiteError::ResourceNotFound(err.msg))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(UpdateSiteError::Throttling(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for UpdateSiteError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UpdateSiteError::AccessDenied(ref cause) => write!(f, "{}", cause),
            UpdateSiteError::Conflict(ref cause) => write!(f, "{}", cause),
            UpdateSiteError::InternalServer(ref cause) => write!(f, "{}", cause),
            UpdateSiteError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            UpdateSiteError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for UpdateSiteError {}
/// Trait representing the capabilities of the NetworkManager API. NetworkManager clients implement this trait.
#[async_trait]
pub trait NetworkManager {
    /// <p>Associates a customer gateway with a device and optionally, with a link. If you specify a link, it must be associated with the specified device. </p> <p>You can only associate customer gateways that are connected to a VPN attachment on a transit gateway. The transit gateway must be registered in your global network. When you register a transit gateway, customer gateways that are connected to the transit gateway are automatically included in the global network. To list customer gateways that are connected to a transit gateway, use the <a href="https://docs.aws.amazon.com/AWSEC2/latest/APIReference/API_DescribeVpnConnections.html">DescribeVpnConnections</a> EC2 API and filter by <code>transit-gateway-id</code>.</p> <p>You cannot associate a customer gateway with more than one device and link. </p>
    async fn associate_customer_gateway(
        &self,
        input: AssociateCustomerGatewayRequest,
    ) -> Result<AssociateCustomerGatewayResponse, RusotoError<AssociateCustomerGatewayError>>;

    /// <p>Associates a link to a device. A device can be associated to multiple links and a link can be associated to multiple devices. The device and link must be in the same global network and the same site.</p>
    async fn associate_link(
        &self,
        input: AssociateLinkRequest,
    ) -> Result<AssociateLinkResponse, RusotoError<AssociateLinkError>>;

    /// <p>Associates a transit gateway Connect peer with a device, and optionally, with a link. If you specify a link, it must be associated with the specified device. </p> <p>You can only associate transit gateway Connect peers that have been created on a transit gateway that's registered in your global network.</p> <p>You cannot associate a transit gateway Connect peer with more than one device and link. </p>
    async fn associate_transit_gateway_connect_peer(
        &self,
        input: AssociateTransitGatewayConnectPeerRequest,
    ) -> Result<
        AssociateTransitGatewayConnectPeerResponse,
        RusotoError<AssociateTransitGatewayConnectPeerError>,
    >;

    /// <p>Creates a connection between two devices. The devices can be a physical or virtual appliance that connects to a third-party appliance in a VPC, or a physical appliance that connects to another physical appliance in an on-premises network.</p>
    async fn create_connection(
        &self,
        input: CreateConnectionRequest,
    ) -> Result<CreateConnectionResponse, RusotoError<CreateConnectionError>>;

    /// <p>Creates a new device in a global network. If you specify both a site ID and a location, the location of the site is used for visualization in the Network Manager console.</p>
    async fn create_device(
        &self,
        input: CreateDeviceRequest,
    ) -> Result<CreateDeviceResponse, RusotoError<CreateDeviceError>>;

    /// <p>Creates a new, empty global network.</p>
    async fn create_global_network(
        &self,
        input: CreateGlobalNetworkRequest,
    ) -> Result<CreateGlobalNetworkResponse, RusotoError<CreateGlobalNetworkError>>;

    /// <p>Creates a new link for a specified site.</p>
    async fn create_link(
        &self,
        input: CreateLinkRequest,
    ) -> Result<CreateLinkResponse, RusotoError<CreateLinkError>>;

    /// <p>Creates a new site in a global network.</p>
    async fn create_site(
        &self,
        input: CreateSiteRequest,
    ) -> Result<CreateSiteResponse, RusotoError<CreateSiteError>>;

    /// <p>Deletes the specified connection in your global network.</p>
    async fn delete_connection(
        &self,
        input: DeleteConnectionRequest,
    ) -> Result<DeleteConnectionResponse, RusotoError<DeleteConnectionError>>;

    /// <p>Deletes an existing device. You must first disassociate the device from any links and customer gateways.</p>
    async fn delete_device(
        &self,
        input: DeleteDeviceRequest,
    ) -> Result<DeleteDeviceResponse, RusotoError<DeleteDeviceError>>;

    /// <p>Deletes an existing global network. You must first delete all global network objects (devices, links, and sites) and deregister all transit gateways.</p>
    async fn delete_global_network(
        &self,
        input: DeleteGlobalNetworkRequest,
    ) -> Result<DeleteGlobalNetworkResponse, RusotoError<DeleteGlobalNetworkError>>;

    /// <p>Deletes an existing link. You must first disassociate the link from any devices and customer gateways.</p>
    async fn delete_link(
        &self,
        input: DeleteLinkRequest,
    ) -> Result<DeleteLinkResponse, RusotoError<DeleteLinkError>>;

    /// <p>Deletes an existing site. The site cannot be associated with any device or link.</p>
    async fn delete_site(
        &self,
        input: DeleteSiteRequest,
    ) -> Result<DeleteSiteResponse, RusotoError<DeleteSiteError>>;

    /// <p>Deregisters a transit gateway from your global network. This action does not delete your transit gateway, or modify any of its attachments. This action removes any customer gateway associations.</p>
    async fn deregister_transit_gateway(
        &self,
        input: DeregisterTransitGatewayRequest,
    ) -> Result<DeregisterTransitGatewayResponse, RusotoError<DeregisterTransitGatewayError>>;

    /// <p>Describes one or more global networks. By default, all global networks are described. To describe the objects in your global network, you must use the appropriate <code>Get*</code> action. For example, to list the transit gateways in your global network, use <a>GetTransitGatewayRegistrations</a>.</p>
    async fn describe_global_networks(
        &self,
        input: DescribeGlobalNetworksRequest,
    ) -> Result<DescribeGlobalNetworksResponse, RusotoError<DescribeGlobalNetworksError>>;

    /// <p>Disassociates a customer gateway from a device and a link.</p>
    async fn disassociate_customer_gateway(
        &self,
        input: DisassociateCustomerGatewayRequest,
    ) -> Result<DisassociateCustomerGatewayResponse, RusotoError<DisassociateCustomerGatewayError>>;

    /// <p>Disassociates an existing device from a link. You must first disassociate any customer gateways that are associated with the link.</p>
    async fn disassociate_link(
        &self,
        input: DisassociateLinkRequest,
    ) -> Result<DisassociateLinkResponse, RusotoError<DisassociateLinkError>>;

    /// <p>Disassociates a transit gateway Connect peer from a device and link.</p>
    async fn disassociate_transit_gateway_connect_peer(
        &self,
        input: DisassociateTransitGatewayConnectPeerRequest,
    ) -> Result<
        DisassociateTransitGatewayConnectPeerResponse,
        RusotoError<DisassociateTransitGatewayConnectPeerError>,
    >;

    /// <p>Gets information about one or more of your connections in a global network.</p>
    async fn get_connections(
        &self,
        input: GetConnectionsRequest,
    ) -> Result<GetConnectionsResponse, RusotoError<GetConnectionsError>>;

    /// <p>Gets the association information for customer gateways that are associated with devices and links in your global network.</p>
    async fn get_customer_gateway_associations(
        &self,
        input: GetCustomerGatewayAssociationsRequest,
    ) -> Result<
        GetCustomerGatewayAssociationsResponse,
        RusotoError<GetCustomerGatewayAssociationsError>,
    >;

    /// <p>Gets information about one or more of your devices in a global network.</p>
    async fn get_devices(
        &self,
        input: GetDevicesRequest,
    ) -> Result<GetDevicesResponse, RusotoError<GetDevicesError>>;

    /// <p>Gets the link associations for a device or a link. Either the device ID or the link ID must be specified.</p>
    async fn get_link_associations(
        &self,
        input: GetLinkAssociationsRequest,
    ) -> Result<GetLinkAssociationsResponse, RusotoError<GetLinkAssociationsError>>;

    /// <p>Gets information about one or more links in a specified global network.</p> <p>If you specify the site ID, you cannot specify the type or provider in the same request. You can specify the type and provider in the same request.</p>
    async fn get_links(
        &self,
        input: GetLinksRequest,
    ) -> Result<GetLinksResponse, RusotoError<GetLinksError>>;

    /// <p>Gets information about one or more of your sites in a global network.</p>
    async fn get_sites(
        &self,
        input: GetSitesRequest,
    ) -> Result<GetSitesResponse, RusotoError<GetSitesError>>;

    /// <p>Gets information about one or more of your transit gateway Connect peer associations in a global network.</p>
    async fn get_transit_gateway_connect_peer_associations(
        &self,
        input: GetTransitGatewayConnectPeerAssociationsRequest,
    ) -> Result<
        GetTransitGatewayConnectPeerAssociationsResponse,
        RusotoError<GetTransitGatewayConnectPeerAssociationsError>,
    >;

    /// <p>Gets information about the transit gateway registrations in a specified global network.</p>
    async fn get_transit_gateway_registrations(
        &self,
        input: GetTransitGatewayRegistrationsRequest,
    ) -> Result<
        GetTransitGatewayRegistrationsResponse,
        RusotoError<GetTransitGatewayRegistrationsError>,
    >;

    /// <p>Lists the tags for a specified resource.</p>
    async fn list_tags_for_resource(
        &self,
        input: ListTagsForResourceRequest,
    ) -> Result<ListTagsForResourceResponse, RusotoError<ListTagsForResourceError>>;

    /// <p>Registers a transit gateway in your global network. The transit gateway can be in any AWS Region, but it must be owned by the same AWS account that owns the global network. You cannot register a transit gateway in more than one global network.</p>
    async fn register_transit_gateway(
        &self,
        input: RegisterTransitGatewayRequest,
    ) -> Result<RegisterTransitGatewayResponse, RusotoError<RegisterTransitGatewayError>>;

    /// <p>Tags a specified resource.</p>
    async fn tag_resource(
        &self,
        input: TagResourceRequest,
    ) -> Result<TagResourceResponse, RusotoError<TagResourceError>>;

    /// <p>Removes tags from a specified resource.</p>
    async fn untag_resource(
        &self,
        input: UntagResourceRequest,
    ) -> Result<UntagResourceResponse, RusotoError<UntagResourceError>>;

    /// <p>Updates the information for an existing connection. To remove information for any of the parameters, specify an empty string.</p>
    async fn update_connection(
        &self,
        input: UpdateConnectionRequest,
    ) -> Result<UpdateConnectionResponse, RusotoError<UpdateConnectionError>>;

    /// <p>Updates the details for an existing device. To remove information for any of the parameters, specify an empty string.</p>
    async fn update_device(
        &self,
        input: UpdateDeviceRequest,
    ) -> Result<UpdateDeviceResponse, RusotoError<UpdateDeviceError>>;

    /// <p>Updates an existing global network. To remove information for any of the parameters, specify an empty string.</p>
    async fn update_global_network(
        &self,
        input: UpdateGlobalNetworkRequest,
    ) -> Result<UpdateGlobalNetworkResponse, RusotoError<UpdateGlobalNetworkError>>;

    /// <p>Updates the details for an existing link. To remove information for any of the parameters, specify an empty string.</p>
    async fn update_link(
        &self,
        input: UpdateLinkRequest,
    ) -> Result<UpdateLinkResponse, RusotoError<UpdateLinkError>>;

    /// <p>Updates the information for an existing site. To remove information for any of the parameters, specify an empty string.</p>
    async fn update_site(
        &self,
        input: UpdateSiteRequest,
    ) -> Result<UpdateSiteResponse, RusotoError<UpdateSiteError>>;
}
/// A client for the NetworkManager API.
#[derive(Clone)]
pub struct NetworkManagerClient {
    client: Client,
    region: region::Region,
}

impl NetworkManagerClient {
    /// Creates a client backed by the default tokio event loop.
    ///
    /// The client will use the default credentials provider and tls client.
    pub fn new(region: region::Region) -> NetworkManagerClient {
        NetworkManagerClient {
            client: Client::shared(),
            region,
        }
    }

    pub fn new_with<P, D>(
        request_dispatcher: D,
        credentials_provider: P,
        region: region::Region,
    ) -> NetworkManagerClient
    where
        P: ProvideAwsCredentials + Send + Sync + 'static,
        D: DispatchSignedRequest + Send + Sync + 'static,
    {
        NetworkManagerClient {
            client: Client::new_with(credentials_provider, request_dispatcher),
            region,
        }
    }

    pub fn new_with_client(client: Client, region: region::Region) -> NetworkManagerClient {
        NetworkManagerClient { client, region }
    }
}

#[async_trait]
impl NetworkManager for NetworkManagerClient {
    /// <p>Associates a customer gateway with a device and optionally, with a link. If you specify a link, it must be associated with the specified device. </p> <p>You can only associate customer gateways that are connected to a VPN attachment on a transit gateway. The transit gateway must be registered in your global network. When you register a transit gateway, customer gateways that are connected to the transit gateway are automatically included in the global network. To list customer gateways that are connected to a transit gateway, use the <a href="https://docs.aws.amazon.com/AWSEC2/latest/APIReference/API_DescribeVpnConnections.html">DescribeVpnConnections</a> EC2 API and filter by <code>transit-gateway-id</code>.</p> <p>You cannot associate a customer gateway with more than one device and link. </p>
    #[allow(unused_mut)]
    async fn associate_customer_gateway(
        &self,
        input: AssociateCustomerGatewayRequest,
    ) -> Result<AssociateCustomerGatewayResponse, RusotoError<AssociateCustomerGatewayError>> {
        #![allow(clippy::needless_update)]
        let request_uri = format!(
            "/global-networks/{global_network_id}/customer-gateway-associations",
            global_network_id = input.global_network_id
        );

        let mut request = SignedRequest::new("POST", "networkmanager", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<AssociateCustomerGatewayResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(AssociateCustomerGatewayError::from_response(response))
        }
    }

    /// <p>Associates a link to a device. A device can be associated to multiple links and a link can be associated to multiple devices. The device and link must be in the same global network and the same site.</p>
    #[allow(unused_mut)]
    async fn associate_link(
        &self,
        input: AssociateLinkRequest,
    ) -> Result<AssociateLinkResponse, RusotoError<AssociateLinkError>> {
        #![allow(clippy::needless_update)]
        let request_uri = format!(
            "/global-networks/{global_network_id}/link-associations",
            global_network_id = input.global_network_id
        );

        let mut request = SignedRequest::new("POST", "networkmanager", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<AssociateLinkResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(AssociateLinkError::from_response(response))
        }
    }

    /// <p>Associates a transit gateway Connect peer with a device, and optionally, with a link. If you specify a link, it must be associated with the specified device. </p> <p>You can only associate transit gateway Connect peers that have been created on a transit gateway that's registered in your global network.</p> <p>You cannot associate a transit gateway Connect peer with more than one device and link. </p>
    #[allow(unused_mut)]
    async fn associate_transit_gateway_connect_peer(
        &self,
        input: AssociateTransitGatewayConnectPeerRequest,
    ) -> Result<
        AssociateTransitGatewayConnectPeerResponse,
        RusotoError<AssociateTransitGatewayConnectPeerError>,
    > {
        #![allow(clippy::needless_update)]
        let request_uri = format!(
            "/global-networks/{global_network_id}/transit-gateway-connect-peer-associations",
            global_network_id = input.global_network_id
        );

        let mut request = SignedRequest::new("POST", "networkmanager", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<AssociateTransitGatewayConnectPeerResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(AssociateTransitGatewayConnectPeerError::from_response(
                response,
            ))
        }
    }

    /// <p>Creates a connection between two devices. The devices can be a physical or virtual appliance that connects to a third-party appliance in a VPC, or a physical appliance that connects to another physical appliance in an on-premises network.</p>
    #[allow(unused_mut)]
    async fn create_connection(
        &self,
        input: CreateConnectionRequest,
    ) -> Result<CreateConnectionResponse, RusotoError<CreateConnectionError>> {
        #![allow(clippy::needless_update)]
        let request_uri = format!(
            "/global-networks/{global_network_id}/connections",
            global_network_id = input.global_network_id
        );

        let mut request = SignedRequest::new("POST", "networkmanager", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<CreateConnectionResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(CreateConnectionError::from_response(response))
        }
    }

    /// <p>Creates a new device in a global network. If you specify both a site ID and a location, the location of the site is used for visualization in the Network Manager console.</p>
    #[allow(unused_mut)]
    async fn create_device(
        &self,
        input: CreateDeviceRequest,
    ) -> Result<CreateDeviceResponse, RusotoError<CreateDeviceError>> {
        #![allow(clippy::needless_update)]
        let request_uri = format!(
            "/global-networks/{global_network_id}/devices",
            global_network_id = input.global_network_id
        );

        let mut request = SignedRequest::new("POST", "networkmanager", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<CreateDeviceResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(CreateDeviceError::from_response(response))
        }
    }

    /// <p>Creates a new, empty global network.</p>
    #[allow(unused_mut)]
    async fn create_global_network(
        &self,
        input: CreateGlobalNetworkRequest,
    ) -> Result<CreateGlobalNetworkResponse, RusotoError<CreateGlobalNetworkError>> {
        #![allow(clippy::needless_update)]
        let request_uri = "/global-networks";

        let mut request = SignedRequest::new("POST", "networkmanager", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<CreateGlobalNetworkResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(CreateGlobalNetworkError::from_response(response))
        }
    }

    /// <p>Creates a new link for a specified site.</p>
    #[allow(unused_mut)]
    async fn create_link(
        &self,
        input: CreateLinkRequest,
    ) -> Result<CreateLinkResponse, RusotoError<CreateLinkError>> {
        #![allow(clippy::needless_update)]
        let request_uri = format!(
            "/global-networks/{global_network_id}/links",
            global_network_id = input.global_network_id
        );

        let mut request = SignedRequest::new("POST", "networkmanager", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<CreateLinkResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(CreateLinkError::from_response(response))
        }
    }

    /// <p>Creates a new site in a global network.</p>
    #[allow(unused_mut)]
    async fn create_site(
        &self,
        input: CreateSiteRequest,
    ) -> Result<CreateSiteResponse, RusotoError<CreateSiteError>> {
        #![allow(clippy::needless_update)]
        let request_uri = format!(
            "/global-networks/{global_network_id}/sites",
            global_network_id = input.global_network_id
        );

        let mut request = SignedRequest::new("POST", "networkmanager", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<CreateSiteResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(CreateSiteError::from_response(response))
        }
    }

    /// <p>Deletes the specified connection in your global network.</p>
    #[allow(unused_mut)]
    async fn delete_connection(
        &self,
        input: DeleteConnectionRequest,
    ) -> Result<DeleteConnectionResponse, RusotoError<DeleteConnectionError>> {
        #![allow(clippy::needless_update)]
        let request_uri = format!(
            "/global-networks/{global_network_id}/connections/{connection_id}",
            connection_id = input.connection_id,
            global_network_id = input.global_network_id
        );

        let mut request =
            SignedRequest::new("DELETE", "networkmanager", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<DeleteConnectionResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(DeleteConnectionError::from_response(response))
        }
    }

    /// <p>Deletes an existing device. You must first disassociate the device from any links and customer gateways.</p>
    #[allow(unused_mut)]
    async fn delete_device(
        &self,
        input: DeleteDeviceRequest,
    ) -> Result<DeleteDeviceResponse, RusotoError<DeleteDeviceError>> {
        #![allow(clippy::needless_update)]
        let request_uri = format!(
            "/global-networks/{global_network_id}/devices/{device_id}",
            device_id = input.device_id,
            global_network_id = input.global_network_id
        );

        let mut request =
            SignedRequest::new("DELETE", "networkmanager", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<DeleteDeviceResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(DeleteDeviceError::from_response(response))
        }
    }

    /// <p>Deletes an existing global network. You must first delete all global network objects (devices, links, and sites) and deregister all transit gateways.</p>
    #[allow(unused_mut)]
    async fn delete_global_network(
        &self,
        input: DeleteGlobalNetworkRequest,
    ) -> Result<DeleteGlobalNetworkResponse, RusotoError<DeleteGlobalNetworkError>> {
        #![allow(clippy::needless_update)]
        let request_uri = format!(
            "/global-networks/{global_network_id}",
            global_network_id = input.global_network_id
        );

        let mut request =
            SignedRequest::new("DELETE", "networkmanager", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<DeleteGlobalNetworkResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(DeleteGlobalNetworkError::from_response(response))
        }
    }

    /// <p>Deletes an existing link. You must first disassociate the link from any devices and customer gateways.</p>
    #[allow(unused_mut)]
    async fn delete_link(
        &self,
        input: DeleteLinkRequest,
    ) -> Result<DeleteLinkResponse, RusotoError<DeleteLinkError>> {
        #![allow(clippy::needless_update)]
        let request_uri = format!(
            "/global-networks/{global_network_id}/links/{link_id}",
            global_network_id = input.global_network_id,
            link_id = input.link_id
        );

        let mut request =
            SignedRequest::new("DELETE", "networkmanager", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<DeleteLinkResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(DeleteLinkError::from_response(response))
        }
    }

    /// <p>Deletes an existing site. The site cannot be associated with any device or link.</p>
    #[allow(unused_mut)]
    async fn delete_site(
        &self,
        input: DeleteSiteRequest,
    ) -> Result<DeleteSiteResponse, RusotoError<DeleteSiteError>> {
        #![allow(clippy::needless_update)]
        let request_uri = format!(
            "/global-networks/{global_network_id}/sites/{site_id}",
            global_network_id = input.global_network_id,
            site_id = input.site_id
        );

        let mut request =
            SignedRequest::new("DELETE", "networkmanager", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<DeleteSiteResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(DeleteSiteError::from_response(response))
        }
    }

    /// <p>Deregisters a transit gateway from your global network. This action does not delete your transit gateway, or modify any of its attachments. This action removes any customer gateway associations.</p>
    #[allow(unused_mut)]
    async fn deregister_transit_gateway(
        &self,
        input: DeregisterTransitGatewayRequest,
    ) -> Result<DeregisterTransitGatewayResponse, RusotoError<DeregisterTransitGatewayError>> {
        #![allow(clippy::needless_update)]
        let request_uri = format!("/global-networks/{global_network_id}/transit-gateway-registrations/{transit_gateway_arn}", global_network_id = input.global_network_id, transit_gateway_arn = input.transit_gateway_arn);

        let mut request =
            SignedRequest::new("DELETE", "networkmanager", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<DeregisterTransitGatewayResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(DeregisterTransitGatewayError::from_response(response))
        }
    }

    /// <p>Describes one or more global networks. By default, all global networks are described. To describe the objects in your global network, you must use the appropriate <code>Get*</code> action. For example, to list the transit gateways in your global network, use <a>GetTransitGatewayRegistrations</a>.</p>
    #[allow(unused_mut)]
    async fn describe_global_networks(
        &self,
        input: DescribeGlobalNetworksRequest,
    ) -> Result<DescribeGlobalNetworksResponse, RusotoError<DescribeGlobalNetworksError>> {
        #![allow(clippy::needless_update)]
        let request_uri = "/global-networks";

        let mut request = SignedRequest::new("GET", "networkmanager", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut params = Params::new();
        if let Some(ref x) = input.global_network_ids {
            for item in x.iter() {
                params.put("globalNetworkIds", item);
            }
        }
        if let Some(ref x) = input.max_results {
            params.put("maxResults", x);
        }
        if let Some(ref x) = input.next_token {
            params.put("nextToken", x);
        }
        request.set_params(params);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<DescribeGlobalNetworksResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(DescribeGlobalNetworksError::from_response(response))
        }
    }

    /// <p>Disassociates a customer gateway from a device and a link.</p>
    #[allow(unused_mut)]
    async fn disassociate_customer_gateway(
        &self,
        input: DisassociateCustomerGatewayRequest,
    ) -> Result<DisassociateCustomerGatewayResponse, RusotoError<DisassociateCustomerGatewayError>>
    {
        #![allow(clippy::needless_update)]
        let request_uri = format!("/global-networks/{global_network_id}/customer-gateway-associations/{customer_gateway_arn}", customer_gateway_arn = input.customer_gateway_arn, global_network_id = input.global_network_id);

        let mut request =
            SignedRequest::new("DELETE", "networkmanager", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<DisassociateCustomerGatewayResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(DisassociateCustomerGatewayError::from_response(response))
        }
    }

    /// <p>Disassociates an existing device from a link. You must first disassociate any customer gateways that are associated with the link.</p>
    #[allow(unused_mut)]
    async fn disassociate_link(
        &self,
        input: DisassociateLinkRequest,
    ) -> Result<DisassociateLinkResponse, RusotoError<DisassociateLinkError>> {
        #![allow(clippy::needless_update)]
        let request_uri = format!(
            "/global-networks/{global_network_id}/link-associations",
            global_network_id = input.global_network_id
        );

        let mut request =
            SignedRequest::new("DELETE", "networkmanager", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut params = Params::new();
        params.put("deviceId", &input.device_id);
        params.put("linkId", &input.link_id);
        request.set_params(params);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<DisassociateLinkResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(DisassociateLinkError::from_response(response))
        }
    }

    /// <p>Disassociates a transit gateway Connect peer from a device and link.</p>
    #[allow(unused_mut)]
    async fn disassociate_transit_gateway_connect_peer(
        &self,
        input: DisassociateTransitGatewayConnectPeerRequest,
    ) -> Result<
        DisassociateTransitGatewayConnectPeerResponse,
        RusotoError<DisassociateTransitGatewayConnectPeerError>,
    > {
        #![allow(clippy::needless_update)]
        let request_uri = format!("/global-networks/{global_network_id}/transit-gateway-connect-peer-associations/{transit_gateway_connect_peer_arn}", global_network_id = input.global_network_id, transit_gateway_connect_peer_arn = input.transit_gateway_connect_peer_arn);

        let mut request =
            SignedRequest::new("DELETE", "networkmanager", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<DisassociateTransitGatewayConnectPeerResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(DisassociateTransitGatewayConnectPeerError::from_response(
                response,
            ))
        }
    }

    /// <p>Gets information about one or more of your connections in a global network.</p>
    #[allow(unused_mut)]
    async fn get_connections(
        &self,
        input: GetConnectionsRequest,
    ) -> Result<GetConnectionsResponse, RusotoError<GetConnectionsError>> {
        #![allow(clippy::needless_update)]
        let request_uri = format!(
            "/global-networks/{global_network_id}/connections",
            global_network_id = input.global_network_id
        );

        let mut request = SignedRequest::new("GET", "networkmanager", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut params = Params::new();
        if let Some(ref x) = input.connection_ids {
            for item in x.iter() {
                params.put("connectionIds", item);
            }
        }
        if let Some(ref x) = input.device_id {
            params.put("deviceId", x);
        }
        if let Some(ref x) = input.max_results {
            params.put("maxResults", x);
        }
        if let Some(ref x) = input.next_token {
            params.put("nextToken", x);
        }
        request.set_params(params);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<GetConnectionsResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(GetConnectionsError::from_response(response))
        }
    }

    /// <p>Gets the association information for customer gateways that are associated with devices and links in your global network.</p>
    #[allow(unused_mut)]
    async fn get_customer_gateway_associations(
        &self,
        input: GetCustomerGatewayAssociationsRequest,
    ) -> Result<
        GetCustomerGatewayAssociationsResponse,
        RusotoError<GetCustomerGatewayAssociationsError>,
    > {
        #![allow(clippy::needless_update)]
        let request_uri = format!(
            "/global-networks/{global_network_id}/customer-gateway-associations",
            global_network_id = input.global_network_id
        );

        let mut request = SignedRequest::new("GET", "networkmanager", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut params = Params::new();
        if let Some(ref x) = input.customer_gateway_arns {
            for item in x.iter() {
                params.put("customerGatewayArns", item);
            }
        }
        if let Some(ref x) = input.max_results {
            params.put("maxResults", x);
        }
        if let Some(ref x) = input.next_token {
            params.put("nextToken", x);
        }
        request.set_params(params);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<GetCustomerGatewayAssociationsResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(GetCustomerGatewayAssociationsError::from_response(response))
        }
    }

    /// <p>Gets information about one or more of your devices in a global network.</p>
    #[allow(unused_mut)]
    async fn get_devices(
        &self,
        input: GetDevicesRequest,
    ) -> Result<GetDevicesResponse, RusotoError<GetDevicesError>> {
        #![allow(clippy::needless_update)]
        let request_uri = format!(
            "/global-networks/{global_network_id}/devices",
            global_network_id = input.global_network_id
        );

        let mut request = SignedRequest::new("GET", "networkmanager", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut params = Params::new();
        if let Some(ref x) = input.device_ids {
            for item in x.iter() {
                params.put("deviceIds", item);
            }
        }
        if let Some(ref x) = input.max_results {
            params.put("maxResults", x);
        }
        if let Some(ref x) = input.next_token {
            params.put("nextToken", x);
        }
        if let Some(ref x) = input.site_id {
            params.put("siteId", x);
        }
        request.set_params(params);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<GetDevicesResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(GetDevicesError::from_response(response))
        }
    }

    /// <p>Gets the link associations for a device or a link. Either the device ID or the link ID must be specified.</p>
    #[allow(unused_mut)]
    async fn get_link_associations(
        &self,
        input: GetLinkAssociationsRequest,
    ) -> Result<GetLinkAssociationsResponse, RusotoError<GetLinkAssociationsError>> {
        #![allow(clippy::needless_update)]
        let request_uri = format!(
            "/global-networks/{global_network_id}/link-associations",
            global_network_id = input.global_network_id
        );

        let mut request = SignedRequest::new("GET", "networkmanager", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut params = Params::new();
        if let Some(ref x) = input.device_id {
            params.put("deviceId", x);
        }
        if let Some(ref x) = input.link_id {
            params.put("linkId", x);
        }
        if let Some(ref x) = input.max_results {
            params.put("maxResults", x);
        }
        if let Some(ref x) = input.next_token {
            params.put("nextToken", x);
        }
        request.set_params(params);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<GetLinkAssociationsResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(GetLinkAssociationsError::from_response(response))
        }
    }

    /// <p>Gets information about one or more links in a specified global network.</p> <p>If you specify the site ID, you cannot specify the type or provider in the same request. You can specify the type and provider in the same request.</p>
    #[allow(unused_mut)]
    async fn get_links(
        &self,
        input: GetLinksRequest,
    ) -> Result<GetLinksResponse, RusotoError<GetLinksError>> {
        #![allow(clippy::needless_update)]
        let request_uri = format!(
            "/global-networks/{global_network_id}/links",
            global_network_id = input.global_network_id
        );

        let mut request = SignedRequest::new("GET", "networkmanager", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut params = Params::new();
        if let Some(ref x) = input.link_ids {
            for item in x.iter() {
                params.put("linkIds", item);
            }
        }
        if let Some(ref x) = input.max_results {
            params.put("maxResults", x);
        }
        if let Some(ref x) = input.next_token {
            params.put("nextToken", x);
        }
        if let Some(ref x) = input.provider {
            params.put("provider", x);
        }
        if let Some(ref x) = input.site_id {
            params.put("siteId", x);
        }
        if let Some(ref x) = input.type_ {
            params.put("type", x);
        }
        request.set_params(params);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<GetLinksResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(GetLinksError::from_response(response))
        }
    }

    /// <p>Gets information about one or more of your sites in a global network.</p>
    #[allow(unused_mut)]
    async fn get_sites(
        &self,
        input: GetSitesRequest,
    ) -> Result<GetSitesResponse, RusotoError<GetSitesError>> {
        #![allow(clippy::needless_update)]
        let request_uri = format!(
            "/global-networks/{global_network_id}/sites",
            global_network_id = input.global_network_id
        );

        let mut request = SignedRequest::new("GET", "networkmanager", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut params = Params::new();
        if let Some(ref x) = input.max_results {
            params.put("maxResults", x);
        }
        if let Some(ref x) = input.next_token {
            params.put("nextToken", x);
        }
        if let Some(ref x) = input.site_ids {
            for item in x.iter() {
                params.put("siteIds", item);
            }
        }
        request.set_params(params);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<GetSitesResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(GetSitesError::from_response(response))
        }
    }

    /// <p>Gets information about one or more of your transit gateway Connect peer associations in a global network.</p>
    #[allow(unused_mut)]
    async fn get_transit_gateway_connect_peer_associations(
        &self,
        input: GetTransitGatewayConnectPeerAssociationsRequest,
    ) -> Result<
        GetTransitGatewayConnectPeerAssociationsResponse,
        RusotoError<GetTransitGatewayConnectPeerAssociationsError>,
    > {
        #![allow(clippy::needless_update)]
        let request_uri = format!(
            "/global-networks/{global_network_id}/transit-gateway-connect-peer-associations",
            global_network_id = input.global_network_id
        );

        let mut request = SignedRequest::new("GET", "networkmanager", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut params = Params::new();
        if let Some(ref x) = input.max_results {
            params.put("maxResults", x);
        }
        if let Some(ref x) = input.next_token {
            params.put("nextToken", x);
        }
        if let Some(ref x) = input.transit_gateway_connect_peer_arns {
            for item in x.iter() {
                params.put("transitGatewayConnectPeerArns", item);
            }
        }
        request.set_params(params);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<GetTransitGatewayConnectPeerAssociationsResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(GetTransitGatewayConnectPeerAssociationsError::from_response(response))
        }
    }

    /// <p>Gets information about the transit gateway registrations in a specified global network.</p>
    #[allow(unused_mut)]
    async fn get_transit_gateway_registrations(
        &self,
        input: GetTransitGatewayRegistrationsRequest,
    ) -> Result<
        GetTransitGatewayRegistrationsResponse,
        RusotoError<GetTransitGatewayRegistrationsError>,
    > {
        #![allow(clippy::needless_update)]
        let request_uri = format!(
            "/global-networks/{global_network_id}/transit-gateway-registrations",
            global_network_id = input.global_network_id
        );

        let mut request = SignedRequest::new("GET", "networkmanager", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut params = Params::new();
        if let Some(ref x) = input.max_results {
            params.put("maxResults", x);
        }
        if let Some(ref x) = input.next_token {
            params.put("nextToken", x);
        }
        if let Some(ref x) = input.transit_gateway_arns {
            for item in x.iter() {
                params.put("transitGatewayArns", item);
            }
        }
        request.set_params(params);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<GetTransitGatewayRegistrationsResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(GetTransitGatewayRegistrationsError::from_response(response))
        }
    }

    /// <p>Lists the tags for a specified resource.</p>
    #[allow(unused_mut)]
    async fn list_tags_for_resource(
        &self,
        input: ListTagsForResourceRequest,
    ) -> Result<ListTagsForResourceResponse, RusotoError<ListTagsForResourceError>> {
        #![allow(clippy::needless_update)]
        let request_uri = format!("/tags/{resource_arn}", resource_arn = input.resource_arn);

        let mut request = SignedRequest::new("GET", "networkmanager", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<ListTagsForResourceResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(ListTagsForResourceError::from_response(response))
        }
    }

    /// <p>Registers a transit gateway in your global network. The transit gateway can be in any AWS Region, but it must be owned by the same AWS account that owns the global network. You cannot register a transit gateway in more than one global network.</p>
    #[allow(unused_mut)]
    async fn register_transit_gateway(
        &self,
        input: RegisterTransitGatewayRequest,
    ) -> Result<RegisterTransitGatewayResponse, RusotoError<RegisterTransitGatewayError>> {
        #![allow(clippy::needless_update)]
        let request_uri = format!(
            "/global-networks/{global_network_id}/transit-gateway-registrations",
            global_network_id = input.global_network_id
        );

        let mut request = SignedRequest::new("POST", "networkmanager", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<RegisterTransitGatewayResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(RegisterTransitGatewayError::from_response(response))
        }
    }

    /// <p>Tags a specified resource.</p>
    #[allow(unused_mut)]
    async fn tag_resource(
        &self,
        input: TagResourceRequest,
    ) -> Result<TagResourceResponse, RusotoError<TagResourceError>> {
        #![allow(clippy::needless_update)]
        let request_uri = format!("/tags/{resource_arn}", resource_arn = input.resource_arn);

        let mut request = SignedRequest::new("POST", "networkmanager", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<TagResourceResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(TagResourceError::from_response(response))
        }
    }

    /// <p>Removes tags from a specified resource.</p>
    #[allow(unused_mut)]
    async fn untag_resource(
        &self,
        input: UntagResourceRequest,
    ) -> Result<UntagResourceResponse, RusotoError<UntagResourceError>> {
        #![allow(clippy::needless_update)]
        let request_uri = format!("/tags/{resource_arn}", resource_arn = input.resource_arn);

        let mut request =
            SignedRequest::new("DELETE", "networkmanager", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut params = Params::new();
        for item in input.tag_keys.iter() {
            params.put("tagKeys", item);
        }
        request.set_params(params);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<UntagResourceResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(UntagResourceError::from_response(response))
        }
    }

    /// <p>Updates the information for an existing connection. To remove information for any of the parameters, specify an empty string.</p>
    #[allow(unused_mut)]
    async fn update_connection(
        &self,
        input: UpdateConnectionRequest,
    ) -> Result<UpdateConnectionResponse, RusotoError<UpdateConnectionError>> {
        #![allow(clippy::needless_update)]
        let request_uri = format!(
            "/global-networks/{global_network_id}/connections/{connection_id}",
            connection_id = input.connection_id,
            global_network_id = input.global_network_id
        );

        let mut request = SignedRequest::new("PATCH", "networkmanager", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<UpdateConnectionResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(UpdateConnectionError::from_response(response))
        }
    }

    /// <p>Updates the details for an existing device. To remove information for any of the parameters, specify an empty string.</p>
    #[allow(unused_mut)]
    async fn update_device(
        &self,
        input: UpdateDeviceRequest,
    ) -> Result<UpdateDeviceResponse, RusotoError<UpdateDeviceError>> {
        #![allow(clippy::needless_update)]
        let request_uri = format!(
            "/global-networks/{global_network_id}/devices/{device_id}",
            device_id = input.device_id,
            global_network_id = input.global_network_id
        );

        let mut request = SignedRequest::new("PATCH", "networkmanager", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<UpdateDeviceResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(UpdateDeviceError::from_response(response))
        }
    }

    /// <p>Updates an existing global network. To remove information for any of the parameters, specify an empty string.</p>
    #[allow(unused_mut)]
    async fn update_global_network(
        &self,
        input: UpdateGlobalNetworkRequest,
    ) -> Result<UpdateGlobalNetworkResponse, RusotoError<UpdateGlobalNetworkError>> {
        #![allow(clippy::needless_update)]
        let request_uri = format!(
            "/global-networks/{global_network_id}",
            global_network_id = input.global_network_id
        );

        let mut request = SignedRequest::new("PATCH", "networkmanager", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<UpdateGlobalNetworkResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(UpdateGlobalNetworkError::from_response(response))
        }
    }

    /// <p>Updates the details for an existing link. To remove information for any of the parameters, specify an empty string.</p>
    #[allow(unused_mut)]
    async fn update_link(
        &self,
        input: UpdateLinkRequest,
    ) -> Result<UpdateLinkResponse, RusotoError<UpdateLinkError>> {
        #![allow(clippy::needless_update)]
        let request_uri = format!(
            "/global-networks/{global_network_id}/links/{link_id}",
            global_network_id = input.global_network_id,
            link_id = input.link_id
        );

        let mut request = SignedRequest::new("PATCH", "networkmanager", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<UpdateLinkResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(UpdateLinkError::from_response(response))
        }
    }

    /// <p>Updates the information for an existing site. To remove information for any of the parameters, specify an empty string.</p>
    #[allow(unused_mut)]
    async fn update_site(
        &self,
        input: UpdateSiteRequest,
    ) -> Result<UpdateSiteResponse, RusotoError<UpdateSiteError>> {
        #![allow(clippy::needless_update)]
        let request_uri = format!(
            "/global-networks/{global_network_id}/sites/{site_id}",
            global_network_id = input.global_network_id,
            site_id = input.site_id
        );

        let mut request = SignedRequest::new("PATCH", "networkmanager", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<UpdateSiteResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(UpdateSiteError::from_response(response))
        }
    }
}
