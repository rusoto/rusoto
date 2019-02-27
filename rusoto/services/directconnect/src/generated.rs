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
use rusoto_core::request::{BufferedHttpResponse, DispatchSignedRequest};
use rusoto_core::{Client, RusotoFuture};

use rusoto_core::credential::{CredentialsError, ProvideAwsCredentials};
use rusoto_core::request::HttpDispatchError;

use rusoto_core::signature::SignedRequest;
use serde_json;
use serde_json::from_slice;
use serde_json::Value as SerdeJsonValue;
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct AllocateConnectionOnInterconnectRequest {
    /// <p>The bandwidth of the connection, in Mbps. The possible values are 50Mbps, 100Mbps, 200Mbps, 300Mbps, 400Mbps, and 500Mbps.</p>
    #[serde(rename = "bandwidth")]
    pub bandwidth: String,
    /// <p>The name of the provisioned connection.</p>
    #[serde(rename = "connectionName")]
    pub connection_name: String,
    /// <p>The ID of the interconnect on which the connection will be provisioned. For example, dxcon-456abc78.</p>
    #[serde(rename = "interconnectId")]
    pub interconnect_id: String,
    /// <p>The ID of the AWS account of the customer for whom the connection will be provisioned.</p>
    #[serde(rename = "ownerAccount")]
    pub owner_account: String,
    /// <p>The dedicated VLAN provisioned to the connection.</p>
    #[serde(rename = "vlan")]
    pub vlan: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct AllocateHostedConnectionRequest {
    /// <p>The bandwidth of the hosted connection, in Mbps. The possible values are 50Mbps, 100Mbps, 200Mbps, 300Mbps, 400Mbps, and 500Mbps.</p>
    #[serde(rename = "bandwidth")]
    pub bandwidth: String,
    /// <p>The ID of the interconnect or LAG.</p>
    #[serde(rename = "connectionId")]
    pub connection_id: String,
    /// <p>The name of the hosted connection.</p>
    #[serde(rename = "connectionName")]
    pub connection_name: String,
    /// <p>The ID of the AWS account ID of the customer for the connection.</p>
    #[serde(rename = "ownerAccount")]
    pub owner_account: String,
    /// <p>The dedicated VLAN provisioned to the hosted connection.</p>
    #[serde(rename = "vlan")]
    pub vlan: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct AllocatePrivateVirtualInterfaceRequest {
    /// <p>The ID of the connection on which the private virtual interface is provisioned.</p>
    #[serde(rename = "connectionId")]
    pub connection_id: String,
    /// <p>Information about the private virtual interface.</p>
    #[serde(rename = "newPrivateVirtualInterfaceAllocation")]
    pub new_private_virtual_interface_allocation: NewPrivateVirtualInterfaceAllocation,
    /// <p>The ID of the AWS account that owns the virtual private interface.</p>
    #[serde(rename = "ownerAccount")]
    pub owner_account: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct AllocatePublicVirtualInterfaceRequest {
    /// <p>The ID of the connection on which the public virtual interface is provisioned.</p>
    #[serde(rename = "connectionId")]
    pub connection_id: String,
    /// <p>Information about the public virtual interface.</p>
    #[serde(rename = "newPublicVirtualInterfaceAllocation")]
    pub new_public_virtual_interface_allocation: NewPublicVirtualInterfaceAllocation,
    /// <p>The ID of the AWS account that owns the public virtual interface.</p>
    #[serde(rename = "ownerAccount")]
    pub owner_account: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct AssociateConnectionWithLagRequest {
    /// <p>The ID of the connection. For example, dxcon-abc123.</p>
    #[serde(rename = "connectionId")]
    pub connection_id: String,
    /// <p>The ID of the LAG with which to associate the connection. For example, dxlag-abc123.</p>
    #[serde(rename = "lagId")]
    pub lag_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct AssociateHostedConnectionRequest {
    /// <p>The ID of the hosted connection.</p>
    #[serde(rename = "connectionId")]
    pub connection_id: String,
    /// <p>The ID of the interconnect or the LAG.</p>
    #[serde(rename = "parentConnectionId")]
    pub parent_connection_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct AssociateVirtualInterfaceRequest {
    /// <p>The ID of the LAG or connection.</p>
    #[serde(rename = "connectionId")]
    pub connection_id: String,
    /// <p>The ID of the virtual interface.</p>
    #[serde(rename = "virtualInterfaceId")]
    pub virtual_interface_id: String,
}

/// <p>Information about a BGP peer.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct BGPPeer {
    /// <p>The address family for the BGP peer.</p>
    #[serde(rename = "addressFamily")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address_family: Option<String>,
    /// <p>The IP address assigned to the Amazon interface.</p>
    #[serde(rename = "amazonAddress")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amazon_address: Option<String>,
    /// <p>The autonomous system (AS) number for Border Gateway Protocol (BGP) configuration.</p>
    #[serde(rename = "asn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asn: Option<i64>,
    /// <p>The authentication key for BGP configuration.</p>
    #[serde(rename = "authKey")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auth_key: Option<String>,
    /// <p>The Direct Connect endpoint on which the BGP peer terminates.</p>
    #[serde(rename = "awsDeviceV2")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aws_device_v2: Option<String>,
    /// <p>The ID of the BGP peer.</p>
    #[serde(rename = "bgpPeerId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bgp_peer_id: Option<String>,
    /// <p><p>The state of the BGP peer. The following are the possible values:</p> <ul> <li> <p> <code>verifying</code>: The BGP peering addresses or ASN require validation before the BGP peer can be created. This state applies only to public virtual interfaces.</p> </li> <li> <p> <code>pending</code>: The BGP peer is created, and remains in this state until it is ready to be established.</p> </li> <li> <p> <code>available</code>: The BGP peer is ready to be established.</p> </li> <li> <p> <code>deleting</code>: The BGP peer is being deleted.</p> </li> <li> <p> <code>deleted</code>: The BGP peer is deleted and cannot be established.</p> </li> </ul></p>
    #[serde(rename = "bgpPeerState")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bgp_peer_state: Option<String>,
    /// <p><p>The status of the BGP peer. The following are the possible values:</p> <ul> <li> <p> <code>up</code>: The BGP peer is established. This state does not indicate the state of the routing function. Ensure that you are receiving routes over the BGP session.</p> </li> <li> <p> <code>down</code>: The BGP peer is down.</p> </li> <li> <p> <code>unknown</code>: The BGP peer status is not available.</p> </li> </ul></p>
    #[serde(rename = "bgpStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bgp_status: Option<String>,
    /// <p>The IP address assigned to the customer interface.</p>
    #[serde(rename = "customerAddress")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_address: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ConfirmConnectionRequest {
    /// <p>The ID of the hosted connection.</p>
    #[serde(rename = "connectionId")]
    pub connection_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct ConfirmConnectionResponse {
    /// <p><p>The state of the connection. The following are the possible values:</p> <ul> <li> <p> <code>ordering</code>: The initial state of a hosted connection provisioned on an interconnect. The connection stays in the ordering state until the owner of the hosted connection confirms or declines the connection order.</p> </li> <li> <p> <code>requested</code>: The initial state of a standard connection. The connection stays in the requested state until the Letter of Authorization (LOA) is sent to the customer.</p> </li> <li> <p> <code>pending</code>: The connection has been approved and is being initialized.</p> </li> <li> <p> <code>available</code>: The network link is up and the connection is ready for use.</p> </li> <li> <p> <code>down</code>: The network link is down.</p> </li> <li> <p> <code>deleting</code>: The connection is being deleted.</p> </li> <li> <p> <code>deleted</code>: The connection has been deleted.</p> </li> <li> <p> <code>rejected</code>: A hosted connection in the <code>ordering</code> state enters the <code>rejected</code> state if it is deleted by the customer.</p> </li> <li> <p> <code>unknown</code>: The state of the connection is not available.</p> </li> </ul></p>
    #[serde(rename = "connectionState")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connection_state: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ConfirmPrivateVirtualInterfaceRequest {
    /// <p>The ID of the Direct Connect gateway.</p>
    #[serde(rename = "directConnectGatewayId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub direct_connect_gateway_id: Option<String>,
    /// <p>The ID of the virtual private gateway.</p>
    #[serde(rename = "virtualGatewayId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub virtual_gateway_id: Option<String>,
    /// <p>The ID of the virtual interface.</p>
    #[serde(rename = "virtualInterfaceId")]
    pub virtual_interface_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct ConfirmPrivateVirtualInterfaceResponse {
    /// <p><p>The state of the virtual interface. The following are the possible values:</p> <ul> <li> <p> <code>confirming</code>: The creation of the virtual interface is pending confirmation from the virtual interface owner. If the owner of the virtual interface is different from the owner of the connection on which it is provisioned, then the virtual interface will remain in this state until it is confirmed by the virtual interface owner.</p> </li> <li> <p> <code>verifying</code>: This state only applies to public virtual interfaces. Each public virtual interface needs validation before the virtual interface can be created.</p> </li> <li> <p> <code>pending</code>: A virtual interface is in this state from the time that it is created until the virtual interface is ready to forward traffic.</p> </li> <li> <p> <code>available</code>: A virtual interface that is able to forward traffic.</p> </li> <li> <p> <code>down</code>: A virtual interface that is BGP down.</p> </li> <li> <p> <code>deleting</code>: A virtual interface is in this state immediately after calling <a>DeleteVirtualInterface</a> until it can no longer forward traffic.</p> </li> <li> <p> <code>deleted</code>: A virtual interface that cannot forward traffic.</p> </li> <li> <p> <code>rejected</code>: The virtual interface owner has declined creation of the virtual interface. If a virtual interface in the <code>Confirming</code> state is deleted by the virtual interface owner, the virtual interface enters the <code>Rejected</code> state.</p> </li> <li> <p> <code>unknown</code>: The state of the virtual interface is not available.</p> </li> </ul></p>
    #[serde(rename = "virtualInterfaceState")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub virtual_interface_state: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ConfirmPublicVirtualInterfaceRequest {
    /// <p>The ID of the virtual interface.</p>
    #[serde(rename = "virtualInterfaceId")]
    pub virtual_interface_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct ConfirmPublicVirtualInterfaceResponse {
    /// <p><p>The state of the virtual interface. The following are the possible values:</p> <ul> <li> <p> <code>confirming</code>: The creation of the virtual interface is pending confirmation from the virtual interface owner. If the owner of the virtual interface is different from the owner of the connection on which it is provisioned, then the virtual interface will remain in this state until it is confirmed by the virtual interface owner.</p> </li> <li> <p> <code>verifying</code>: This state only applies to public virtual interfaces. Each public virtual interface needs validation before the virtual interface can be created.</p> </li> <li> <p> <code>pending</code>: A virtual interface is in this state from the time that it is created until the virtual interface is ready to forward traffic.</p> </li> <li> <p> <code>available</code>: A virtual interface that is able to forward traffic.</p> </li> <li> <p> <code>down</code>: A virtual interface that is BGP down.</p> </li> <li> <p> <code>deleting</code>: A virtual interface is in this state immediately after calling <a>DeleteVirtualInterface</a> until it can no longer forward traffic.</p> </li> <li> <p> <code>deleted</code>: A virtual interface that cannot forward traffic.</p> </li> <li> <p> <code>rejected</code>: The virtual interface owner has declined creation of the virtual interface. If a virtual interface in the <code>Confirming</code> state is deleted by the virtual interface owner, the virtual interface enters the <code>Rejected</code> state.</p> </li> <li> <p> <code>unknown</code>: The state of the virtual interface is not available.</p> </li> </ul></p>
    #[serde(rename = "virtualInterfaceState")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub virtual_interface_state: Option<String>,
}

/// <p>Information about an AWS Direct Connect connection.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct Connection {
    /// <p>The Direct Connect endpoint on which the physical connection terminates.</p>
    #[serde(rename = "awsDevice")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aws_device: Option<String>,
    /// <p>The Direct Connect endpoint on which the physical connection terminates.</p>
    #[serde(rename = "awsDeviceV2")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aws_device_v2: Option<String>,
    /// <p>The bandwidth of the connection.</p>
    #[serde(rename = "bandwidth")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bandwidth: Option<String>,
    /// <p>The ID of the connection.</p>
    #[serde(rename = "connectionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connection_id: Option<String>,
    /// <p>The name of the connection.</p>
    #[serde(rename = "connectionName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connection_name: Option<String>,
    /// <p><p>The state of the connection. The following are the possible values:</p> <ul> <li> <p> <code>ordering</code>: The initial state of a hosted connection provisioned on an interconnect. The connection stays in the ordering state until the owner of the hosted connection confirms or declines the connection order.</p> </li> <li> <p> <code>requested</code>: The initial state of a standard connection. The connection stays in the requested state until the Letter of Authorization (LOA) is sent to the customer.</p> </li> <li> <p> <code>pending</code>: The connection has been approved and is being initialized.</p> </li> <li> <p> <code>available</code>: The network link is up and the connection is ready for use.</p> </li> <li> <p> <code>down</code>: The network link is down.</p> </li> <li> <p> <code>deleting</code>: The connection is being deleted.</p> </li> <li> <p> <code>deleted</code>: The connection has been deleted.</p> </li> <li> <p> <code>rejected</code>: A hosted connection in the <code>ordering</code> state enters the <code>rejected</code> state if it is deleted by the customer.</p> </li> <li> <p> <code>unknown</code>: The state of the connection is not available.</p> </li> </ul></p>
    #[serde(rename = "connectionState")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connection_state: Option<String>,
    /// <p>Indicates whether the connection supports a secondary BGP peer in the same address family (IPv4/IPv6).</p>
    #[serde(rename = "hasLogicalRedundancy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_logical_redundancy: Option<String>,
    /// <p>Indicates whether jumbo frames (9001 MTU) are supported.</p>
    #[serde(rename = "jumboFrameCapable")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub jumbo_frame_capable: Option<bool>,
    /// <p>The ID of the LAG.</p>
    #[serde(rename = "lagId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lag_id: Option<String>,
    /// <p>The time of the most recent call to <a>DescribeLoa</a> for this connection.</p>
    #[serde(rename = "loaIssueTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub loa_issue_time: Option<f64>,
    /// <p>The location of the connection.</p>
    #[serde(rename = "location")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    /// <p>The ID of the AWS account that owns the connection.</p>
    #[serde(rename = "ownerAccount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner_account: Option<String>,
    /// <p>The name of the AWS Direct Connect service provider associated with the connection.</p>
    #[serde(rename = "partnerName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub partner_name: Option<String>,
    /// <p>The AWS Region where the connection is located.</p>
    #[serde(rename = "region")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
    /// <p>The ID of the VLAN.</p>
    #[serde(rename = "vlan")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vlan: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct Connections {
    /// <p>The connections.</p>
    #[serde(rename = "connections")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connections: Option<Vec<Connection>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct CreateBGPPeerRequest {
    /// <p>Information about the BGP peer.</p>
    #[serde(rename = "newBGPPeer")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub new_bgp_peer: Option<NewBGPPeer>,
    /// <p>The ID of the virtual interface.</p>
    #[serde(rename = "virtualInterfaceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub virtual_interface_id: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct CreateBGPPeerResponse {
    /// <p>The virtual interface.</p>
    #[serde(rename = "virtualInterface")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub virtual_interface: Option<VirtualInterface>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct CreateConnectionRequest {
    /// <p>The bandwidth of the connection.</p>
    #[serde(rename = "bandwidth")]
    pub bandwidth: String,
    /// <p>The name of the connection.</p>
    #[serde(rename = "connectionName")]
    pub connection_name: String,
    /// <p>The ID of the LAG.</p>
    #[serde(rename = "lagId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lag_id: Option<String>,
    /// <p>The location of the connection.</p>
    #[serde(rename = "location")]
    pub location: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct CreateDirectConnectGatewayAssociationRequest {
    /// <p>The ID of the Direct Connect gateway.</p>
    #[serde(rename = "directConnectGatewayId")]
    pub direct_connect_gateway_id: String,
    /// <p>The ID of the virtual private gateway.</p>
    #[serde(rename = "virtualGatewayId")]
    pub virtual_gateway_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct CreateDirectConnectGatewayAssociationResult {
    /// <p>The association to be created.</p>
    #[serde(rename = "directConnectGatewayAssociation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub direct_connect_gateway_association: Option<DirectConnectGatewayAssociation>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct CreateDirectConnectGatewayRequest {
    /// <p>The autonomous system number (ASN) for Border Gateway Protocol (BGP) to be configured on the Amazon side of the connection. The ASN must be in the private range of 64,512 to 65,534 or 4,200,000,000 to 4,294,967,294. The default is 64512.</p>
    #[serde(rename = "amazonSideAsn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amazon_side_asn: Option<i64>,
    /// <p>The name of the Direct Connect gateway.</p>
    #[serde(rename = "directConnectGatewayName")]
    pub direct_connect_gateway_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct CreateDirectConnectGatewayResult {
    /// <p>The Direct Connect gateway.</p>
    #[serde(rename = "directConnectGateway")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub direct_connect_gateway: Option<DirectConnectGateway>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct CreateInterconnectRequest {
    /// <p>The port bandwidth, in Gbps. The possible values are 1 and 10.</p>
    #[serde(rename = "bandwidth")]
    pub bandwidth: String,
    /// <p>The name of the interconnect.</p>
    #[serde(rename = "interconnectName")]
    pub interconnect_name: String,
    /// <p>The ID of the LAG.</p>
    #[serde(rename = "lagId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lag_id: Option<String>,
    /// <p>The location of the interconnect.</p>
    #[serde(rename = "location")]
    pub location: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct CreateLagRequest {
    /// <p>The ID of an existing connection to migrate to the LAG.</p>
    #[serde(rename = "connectionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connection_id: Option<String>,
    /// <p>The bandwidth of the individual physical connections bundled by the LAG. The possible values are 1Gbps and 10Gbps.</p>
    #[serde(rename = "connectionsBandwidth")]
    pub connections_bandwidth: String,
    /// <p>The name of the LAG.</p>
    #[serde(rename = "lagName")]
    pub lag_name: String,
    /// <p>The location for the LAG.</p>
    #[serde(rename = "location")]
    pub location: String,
    /// <p>The number of physical connections initially provisioned and bundled by the LAG.</p>
    #[serde(rename = "numberOfConnections")]
    pub number_of_connections: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct CreatePrivateVirtualInterfaceRequest {
    /// <p>The ID of the connection.</p>
    #[serde(rename = "connectionId")]
    pub connection_id: String,
    /// <p>Information about the private virtual interface.</p>
    #[serde(rename = "newPrivateVirtualInterface")]
    pub new_private_virtual_interface: NewPrivateVirtualInterface,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct CreatePublicVirtualInterfaceRequest {
    /// <p>The ID of the connection.</p>
    #[serde(rename = "connectionId")]
    pub connection_id: String,
    /// <p>Information about the public virtual interface.</p>
    #[serde(rename = "newPublicVirtualInterface")]
    pub new_public_virtual_interface: NewPublicVirtualInterface,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DeleteBGPPeerRequest {
    /// <p>The autonomous system (AS) number for Border Gateway Protocol (BGP) configuration.</p>
    #[serde(rename = "asn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asn: Option<i64>,
    /// <p>The ID of the BGP peer.</p>
    #[serde(rename = "bgpPeerId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bgp_peer_id: Option<String>,
    /// <p>The IP address assigned to the customer interface.</p>
    #[serde(rename = "customerAddress")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_address: Option<String>,
    /// <p>The ID of the virtual interface.</p>
    #[serde(rename = "virtualInterfaceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub virtual_interface_id: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct DeleteBGPPeerResponse {
    /// <p>The virtual interface.</p>
    #[serde(rename = "virtualInterface")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub virtual_interface: Option<VirtualInterface>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DeleteConnectionRequest {
    /// <p>The ID of the connection.</p>
    #[serde(rename = "connectionId")]
    pub connection_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DeleteDirectConnectGatewayAssociationRequest {
    /// <p>The ID of the Direct Connect gateway.</p>
    #[serde(rename = "directConnectGatewayId")]
    pub direct_connect_gateway_id: String,
    /// <p>The ID of the virtual private gateway.</p>
    #[serde(rename = "virtualGatewayId")]
    pub virtual_gateway_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct DeleteDirectConnectGatewayAssociationResult {
    /// <p>The association to be deleted.</p>
    #[serde(rename = "directConnectGatewayAssociation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub direct_connect_gateway_association: Option<DirectConnectGatewayAssociation>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DeleteDirectConnectGatewayRequest {
    /// <p>The ID of the Direct Connect gateway.</p>
    #[serde(rename = "directConnectGatewayId")]
    pub direct_connect_gateway_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct DeleteDirectConnectGatewayResult {
    /// <p>The Direct Connect gateway.</p>
    #[serde(rename = "directConnectGateway")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub direct_connect_gateway: Option<DirectConnectGateway>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DeleteInterconnectRequest {
    /// <p>The ID of the interconnect.</p>
    #[serde(rename = "interconnectId")]
    pub interconnect_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct DeleteInterconnectResponse {
    /// <p><p>The state of the interconnect. The following are the possible values:</p> <ul> <li> <p> <code>requested</code>: The initial state of an interconnect. The interconnect stays in the requested state until the Letter of Authorization (LOA) is sent to the customer.</p> </li> <li> <p> <code>pending</code>: The interconnect is approved, and is being initialized.</p> </li> <li> <p> <code>available</code>: The network link is up, and the interconnect is ready for use.</p> </li> <li> <p> <code>down</code>: The network link is down.</p> </li> <li> <p> <code>deleting</code>: The interconnect is being deleted.</p> </li> <li> <p> <code>deleted</code>: The interconnect is deleted.</p> </li> <li> <p> <code>unknown</code>: The state of the interconnect is not available.</p> </li> </ul></p>
    #[serde(rename = "interconnectState")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interconnect_state: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DeleteLagRequest {
    /// <p>The ID of the LAG.</p>
    #[serde(rename = "lagId")]
    pub lag_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DeleteVirtualInterfaceRequest {
    /// <p>The ID of the virtual interface.</p>
    #[serde(rename = "virtualInterfaceId")]
    pub virtual_interface_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct DeleteVirtualInterfaceResponse {
    /// <p><p>The state of the virtual interface. The following are the possible values:</p> <ul> <li> <p> <code>confirming</code>: The creation of the virtual interface is pending confirmation from the virtual interface owner. If the owner of the virtual interface is different from the owner of the connection on which it is provisioned, then the virtual interface will remain in this state until it is confirmed by the virtual interface owner.</p> </li> <li> <p> <code>verifying</code>: This state only applies to public virtual interfaces. Each public virtual interface needs validation before the virtual interface can be created.</p> </li> <li> <p> <code>pending</code>: A virtual interface is in this state from the time that it is created until the virtual interface is ready to forward traffic.</p> </li> <li> <p> <code>available</code>: A virtual interface that is able to forward traffic.</p> </li> <li> <p> <code>down</code>: A virtual interface that is BGP down.</p> </li> <li> <p> <code>deleting</code>: A virtual interface is in this state immediately after calling <a>DeleteVirtualInterface</a> until it can no longer forward traffic.</p> </li> <li> <p> <code>deleted</code>: A virtual interface that cannot forward traffic.</p> </li> <li> <p> <code>rejected</code>: The virtual interface owner has declined creation of the virtual interface. If a virtual interface in the <code>Confirming</code> state is deleted by the virtual interface owner, the virtual interface enters the <code>Rejected</code> state.</p> </li> <li> <p> <code>unknown</code>: The state of the virtual interface is not available.</p> </li> </ul></p>
    #[serde(rename = "virtualInterfaceState")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub virtual_interface_state: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DescribeConnectionLoaRequest {
    /// <p>The ID of the connection.</p>
    #[serde(rename = "connectionId")]
    pub connection_id: String,
    /// <p>The standard media type for the LOA-CFA document. The only supported value is application/pdf.</p>
    #[serde(rename = "loaContentType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub loa_content_type: Option<String>,
    /// <p>The name of the APN partner or service provider who establishes connectivity on your behalf. If you specify this parameter, the LOA-CFA lists the provider name alongside your company name as the requester of the cross connect.</p>
    #[serde(rename = "providerName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider_name: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct DescribeConnectionLoaResponse {
    /// <p>The Letter of Authorization - Connecting Facility Assignment (LOA-CFA).</p>
    #[serde(rename = "loa")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub loa: Option<Loa>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DescribeConnectionsOnInterconnectRequest {
    /// <p>The ID of the interconnect.</p>
    #[serde(rename = "interconnectId")]
    pub interconnect_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DescribeConnectionsRequest {
    /// <p>The ID of the connection.</p>
    #[serde(rename = "connectionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connection_id: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DescribeDirectConnectGatewayAssociationsRequest {
    /// <p>The ID of the Direct Connect gateway.</p>
    #[serde(rename = "directConnectGatewayId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub direct_connect_gateway_id: Option<String>,
    /// <p>The maximum number of associations to return per page.</p>
    #[serde(rename = "maxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>The token provided in the previous call to retrieve the next page.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The ID of the virtual private gateway.</p>
    #[serde(rename = "virtualGatewayId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub virtual_gateway_id: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct DescribeDirectConnectGatewayAssociationsResult {
    /// <p>The associations.</p>
    #[serde(rename = "directConnectGatewayAssociations")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub direct_connect_gateway_associations: Option<Vec<DirectConnectGatewayAssociation>>,
    /// <p>The token to retrieve the next page.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DescribeDirectConnectGatewayAttachmentsRequest {
    /// <p>The ID of the Direct Connect gateway.</p>
    #[serde(rename = "directConnectGatewayId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub direct_connect_gateway_id: Option<String>,
    /// <p>The maximum number of attachments to return per page.</p>
    #[serde(rename = "maxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>The token provided in the previous call to retrieve the next page.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The ID of the virtual interface.</p>
    #[serde(rename = "virtualInterfaceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub virtual_interface_id: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct DescribeDirectConnectGatewayAttachmentsResult {
    /// <p>The attachments.</p>
    #[serde(rename = "directConnectGatewayAttachments")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub direct_connect_gateway_attachments: Option<Vec<DirectConnectGatewayAttachment>>,
    /// <p>The token to retrieve the next page.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DescribeDirectConnectGatewaysRequest {
    /// <p>The ID of the Direct Connect gateway.</p>
    #[serde(rename = "directConnectGatewayId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub direct_connect_gateway_id: Option<String>,
    /// <p>The maximum number of Direct Connect gateways to return per page.</p>
    #[serde(rename = "maxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>The token provided in the previous call to retrieve the next page.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct DescribeDirectConnectGatewaysResult {
    /// <p>The Direct Connect gateways.</p>
    #[serde(rename = "directConnectGateways")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub direct_connect_gateways: Option<Vec<DirectConnectGateway>>,
    /// <p>The token to retrieve the next page.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DescribeHostedConnectionsRequest {
    /// <p>The ID of the interconnect or LAG.</p>
    #[serde(rename = "connectionId")]
    pub connection_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DescribeInterconnectLoaRequest {
    /// <p>The ID of the interconnect.</p>
    #[serde(rename = "interconnectId")]
    pub interconnect_id: String,
    /// <p>The standard media type for the LOA-CFA document. The only supported value is application/pdf.</p>
    #[serde(rename = "loaContentType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub loa_content_type: Option<String>,
    /// <p>The name of the service provider who establishes connectivity on your behalf. If you supply this parameter, the LOA-CFA lists the provider name alongside your company name as the requester of the cross connect.</p>
    #[serde(rename = "providerName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider_name: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct DescribeInterconnectLoaResponse {
    /// <p>The Letter of Authorization - Connecting Facility Assignment (LOA-CFA).</p>
    #[serde(rename = "loa")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub loa: Option<Loa>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DescribeInterconnectsRequest {
    /// <p>The ID of the interconnect.</p>
    #[serde(rename = "interconnectId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interconnect_id: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DescribeLagsRequest {
    /// <p>The ID of the LAG.</p>
    #[serde(rename = "lagId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lag_id: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DescribeLoaRequest {
    /// <p>The ID of a connection, LAG, or interconnect.</p>
    #[serde(rename = "connectionId")]
    pub connection_id: String,
    /// <p>The standard media type for the LOA-CFA document. The only supported value is application/pdf.</p>
    #[serde(rename = "loaContentType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub loa_content_type: Option<String>,
    /// <p>The name of the service provider who establishes connectivity on your behalf. If you specify this parameter, the LOA-CFA lists the provider name alongside your company name as the requester of the cross connect.</p>
    #[serde(rename = "providerName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider_name: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DescribeTagsRequest {
    /// <p>The Amazon Resource Names (ARNs) of the resources.</p>
    #[serde(rename = "resourceArns")]
    pub resource_arns: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct DescribeTagsResponse {
    /// <p>Information about the tags.</p>
    #[serde(rename = "resourceTags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_tags: Option<Vec<ResourceTag>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DescribeVirtualInterfacesRequest {
    /// <p>The ID of the connection.</p>
    #[serde(rename = "connectionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connection_id: Option<String>,
    /// <p>The ID of the virtual interface.</p>
    #[serde(rename = "virtualInterfaceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub virtual_interface_id: Option<String>,
}

/// <p>Information about a Direct Connect gateway, which enables you to connect virtual interfaces and virtual private gateways.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct DirectConnectGateway {
    /// <p>The autonomous system number (ASN) for the Amazon side of the connection.</p>
    #[serde(rename = "amazonSideAsn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amazon_side_asn: Option<i64>,
    /// <p>The ID of the Direct Connect gateway.</p>
    #[serde(rename = "directConnectGatewayId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub direct_connect_gateway_id: Option<String>,
    /// <p>The name of the Direct Connect gateway.</p>
    #[serde(rename = "directConnectGatewayName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub direct_connect_gateway_name: Option<String>,
    /// <p><p>The state of the Direct Connect gateway. The following are the possible values:</p> <ul> <li> <p> <code>pending</code>: The initial state after calling <a>CreateDirectConnectGateway</a>.</p> </li> <li> <p> <code>available</code>: The Direct Connect gateway is ready for use.</p> </li> <li> <p> <code>deleting</code>: The initial state after calling <a>DeleteDirectConnectGateway</a>.</p> </li> <li> <p> <code>deleted</code>: The Direct Connect gateway is deleted and cannot pass traffic.</p> </li> </ul></p>
    #[serde(rename = "directConnectGatewayState")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub direct_connect_gateway_state: Option<String>,
    /// <p>The ID of the AWS account that owns the Direct Connect gateway.</p>
    #[serde(rename = "ownerAccount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner_account: Option<String>,
    /// <p>The error message if the state of an object failed to advance.</p>
    #[serde(rename = "stateChangeError")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state_change_error: Option<String>,
}

/// <p>Information about an association between a Direct Connect gateway and a virtual private gateway.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct DirectConnectGatewayAssociation {
    /// <p><p>The state of the association. The following are the possible values:</p> <ul> <li> <p> <code>associating</code>: The initial state after calling <a>CreateDirectConnectGatewayAssociation</a>.</p> </li> <li> <p> <code>associated</code>: The Direct Connect gateway and virtual private gateway are successfully associated and ready to pass traffic.</p> </li> <li> <p> <code>disassociating</code>: The initial state after calling <a>DeleteDirectConnectGatewayAssociation</a>.</p> </li> <li> <p> <code>disassociated</code>: The virtual private gateway is disassociated from the Direct Connect gateway. Traffic flow between the Direct Connect gateway and virtual private gateway is stopped.</p> </li> </ul></p>
    #[serde(rename = "associationState")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub association_state: Option<String>,
    /// <p>The ID of the Direct Connect gateway.</p>
    #[serde(rename = "directConnectGatewayId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub direct_connect_gateway_id: Option<String>,
    /// <p>The error message if the state of an object failed to advance.</p>
    #[serde(rename = "stateChangeError")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state_change_error: Option<String>,
    /// <p>The ID of the virtual private gateway. Applies only to private virtual interfaces.</p>
    #[serde(rename = "virtualGatewayId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub virtual_gateway_id: Option<String>,
    /// <p>The ID of the AWS account that owns the virtual private gateway.</p>
    #[serde(rename = "virtualGatewayOwnerAccount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub virtual_gateway_owner_account: Option<String>,
    /// <p>The AWS Region where the virtual private gateway is located.</p>
    #[serde(rename = "virtualGatewayRegion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub virtual_gateway_region: Option<String>,
}

/// <p>Information about an attachment between a Direct Connect gateway and a virtual interface.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct DirectConnectGatewayAttachment {
    /// <p><p>The state of the attachment. The following are the possible values:</p> <ul> <li> <p> <code>attaching</code>: The initial state after a virtual interface is created using the Direct Connect gateway.</p> </li> <li> <p> <code>attached</code>: The Direct Connect gateway and virtual interface are attached and ready to pass traffic.</p> </li> <li> <p> <code>detaching</code>: The initial state after calling <a>DeleteVirtualInterface</a>.</p> </li> <li> <p> <code>detached</code>: The virtual interface is detached from the Direct Connect gateway. Traffic flow between the Direct Connect gateway and virtual interface is stopped.</p> </li> </ul></p>
    #[serde(rename = "attachmentState")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attachment_state: Option<String>,
    /// <p>The ID of the Direct Connect gateway.</p>
    #[serde(rename = "directConnectGatewayId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub direct_connect_gateway_id: Option<String>,
    /// <p>The error message if the state of an object failed to advance.</p>
    #[serde(rename = "stateChangeError")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state_change_error: Option<String>,
    /// <p>The ID of the virtual interface.</p>
    #[serde(rename = "virtualInterfaceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub virtual_interface_id: Option<String>,
    /// <p>The ID of the AWS account that owns the virtual interface.</p>
    #[serde(rename = "virtualInterfaceOwnerAccount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub virtual_interface_owner_account: Option<String>,
    /// <p>The AWS Region where the virtual interface is located.</p>
    #[serde(rename = "virtualInterfaceRegion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub virtual_interface_region: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DisassociateConnectionFromLagRequest {
    /// <p>The ID of the connection. For example, dxcon-abc123.</p>
    #[serde(rename = "connectionId")]
    pub connection_id: String,
    /// <p>The ID of the LAG. For example, dxlag-abc123.</p>
    #[serde(rename = "lagId")]
    pub lag_id: String,
}

/// <p>Information about an interconnect.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct Interconnect {
    /// <p>The Direct Connect endpoint on which the physical connection terminates.</p>
    #[serde(rename = "awsDevice")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aws_device: Option<String>,
    /// <p>The Direct Connect endpoint on which the physical connection terminates.</p>
    #[serde(rename = "awsDeviceV2")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aws_device_v2: Option<String>,
    /// <p>The bandwidth of the connection.</p>
    #[serde(rename = "bandwidth")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bandwidth: Option<String>,
    /// <p>Indicates whether the interconnect supports a secondary BGP in the same address family (IPv4/IPv6).</p>
    #[serde(rename = "hasLogicalRedundancy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_logical_redundancy: Option<String>,
    /// <p>The ID of the interconnect.</p>
    #[serde(rename = "interconnectId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interconnect_id: Option<String>,
    /// <p>The name of the interconnect.</p>
    #[serde(rename = "interconnectName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interconnect_name: Option<String>,
    /// <p><p>The state of the interconnect. The following are the possible values:</p> <ul> <li> <p> <code>requested</code>: The initial state of an interconnect. The interconnect stays in the requested state until the Letter of Authorization (LOA) is sent to the customer.</p> </li> <li> <p> <code>pending</code>: The interconnect is approved, and is being initialized.</p> </li> <li> <p> <code>available</code>: The network link is up, and the interconnect is ready for use.</p> </li> <li> <p> <code>down</code>: The network link is down.</p> </li> <li> <p> <code>deleting</code>: The interconnect is being deleted.</p> </li> <li> <p> <code>deleted</code>: The interconnect is deleted.</p> </li> <li> <p> <code>unknown</code>: The state of the interconnect is not available.</p> </li> </ul></p>
    #[serde(rename = "interconnectState")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interconnect_state: Option<String>,
    /// <p>Indicates whether jumbo frames (9001 MTU) are supported.</p>
    #[serde(rename = "jumboFrameCapable")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub jumbo_frame_capable: Option<bool>,
    /// <p>The ID of the LAG.</p>
    #[serde(rename = "lagId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lag_id: Option<String>,
    /// <p>The time of the most recent call to <a>DescribeLoa</a> for this connection.</p>
    #[serde(rename = "loaIssueTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub loa_issue_time: Option<f64>,
    /// <p>The location of the connection.</p>
    #[serde(rename = "location")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    /// <p>The AWS Region where the connection is located.</p>
    #[serde(rename = "region")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct Interconnects {
    /// <p>The interconnects.</p>
    #[serde(rename = "interconnects")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interconnects: Option<Vec<Interconnect>>,
}

/// <p>Information about a link aggregation group (LAG).</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct Lag {
    /// <p>Indicates whether the LAG can host other connections.</p>
    #[serde(rename = "allowsHostedConnections")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allows_hosted_connections: Option<bool>,
    /// <p>The Direct Connect endpoint that hosts the LAG.</p>
    #[serde(rename = "awsDevice")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aws_device: Option<String>,
    /// <p>The Direct Connect endpoint that hosts the LAG.</p>
    #[serde(rename = "awsDeviceV2")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aws_device_v2: Option<String>,
    /// <p>The connections bundled by the LAG.</p>
    #[serde(rename = "connections")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connections: Option<Vec<Connection>>,
    /// <p>The individual bandwidth of the physical connections bundled by the LAG. The possible values are 1Gbps and 10Gbps.</p>
    #[serde(rename = "connectionsBandwidth")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connections_bandwidth: Option<String>,
    /// <p>Indicates whether the LAG supports a secondary BGP peer in the same address family (IPv4/IPv6).</p>
    #[serde(rename = "hasLogicalRedundancy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_logical_redundancy: Option<String>,
    /// <p>Indicates whether jumbo frames (9001 MTU) are supported.</p>
    #[serde(rename = "jumboFrameCapable")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub jumbo_frame_capable: Option<bool>,
    /// <p>The ID of the LAG.</p>
    #[serde(rename = "lagId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lag_id: Option<String>,
    /// <p>The name of the LAG.</p>
    #[serde(rename = "lagName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lag_name: Option<String>,
    /// <p><p>The state of the LAG. The following are the possible values:</p> <ul> <li> <p> <code>requested</code>: The initial state of a LAG. The LAG stays in the requested state until the Letter of Authorization (LOA) is available.</p> </li> <li> <p> <code>pending</code>: The LAG has been approved and is being initialized.</p> </li> <li> <p> <code>available</code>: The network link is established and the LAG is ready for use.</p> </li> <li> <p> <code>down</code>: The network link is down.</p> </li> <li> <p> <code>deleting</code>: The LAG is being deleted.</p> </li> <li> <p> <code>deleted</code>: The LAG is deleted.</p> </li> <li> <p> <code>unknown</code>: The state of the LAG is not available.</p> </li> </ul></p>
    #[serde(rename = "lagState")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lag_state: Option<String>,
    /// <p>The location of the LAG.</p>
    #[serde(rename = "location")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    /// <p>The minimum number of physical connections that must be operational for the LAG itself to be operational.</p>
    #[serde(rename = "minimumLinks")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minimum_links: Option<i64>,
    /// <p>The number of physical connections bundled by the LAG, up to a maximum of 10.</p>
    #[serde(rename = "numberOfConnections")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub number_of_connections: Option<i64>,
    /// <p>The ID of the AWS account that owns the LAG.</p>
    #[serde(rename = "ownerAccount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner_account: Option<String>,
    /// <p>The AWS Region where the connection is located.</p>
    #[serde(rename = "region")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct Lags {
    /// <p>The LAGs.</p>
    #[serde(rename = "lags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lags: Option<Vec<Lag>>,
}

/// <p>Information about a Letter of Authorization - Connecting Facility Assignment (LOA-CFA) for a connection.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct Loa {
    /// <p>The binary contents of the LOA-CFA document.</p>
    #[serde(rename = "loaContent")]
    #[serde(
        deserialize_with = "::rusoto_core::serialization::SerdeBlob::deserialize_blob",
        serialize_with = "::rusoto_core::serialization::SerdeBlob::serialize_blob",
        default
    )]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub loa_content: Option<Vec<u8>>,
    /// <p>The standard media type for the LOA-CFA document. The only supported value is application/pdf.</p>
    #[serde(rename = "loaContentType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub loa_content_type: Option<String>,
}

/// <p>Information about an AWS Direct Connect location.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct Location {
    /// <p>The code for the location.</p>
    #[serde(rename = "locationCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location_code: Option<String>,
    /// <p>The name of the location. This includes the name of the colocation partner and the physical site of the building.</p>
    #[serde(rename = "locationName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location_name: Option<String>,
    /// <p>The AWS Region for the location.</p>
    #[serde(rename = "region")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct Locations {
    /// <p>The locations.</p>
    #[serde(rename = "locations")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub locations: Option<Vec<Location>>,
}

/// <p>Information about a new BGP peer.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct NewBGPPeer {
    /// <p>The address family for the BGP peer.</p>
    #[serde(rename = "addressFamily")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address_family: Option<String>,
    /// <p>The IP address assigned to the Amazon interface.</p>
    #[serde(rename = "amazonAddress")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amazon_address: Option<String>,
    /// <p>The autonomous system (AS) number for Border Gateway Protocol (BGP) configuration.</p>
    #[serde(rename = "asn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asn: Option<i64>,
    /// <p>The authentication key for BGP configuration.</p>
    #[serde(rename = "authKey")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auth_key: Option<String>,
    /// <p>The IP address assigned to the customer interface.</p>
    #[serde(rename = "customerAddress")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_address: Option<String>,
}

/// <p>Information about a private virtual interface.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct NewPrivateVirtualInterface {
    /// <p>The address family for the BGP peer.</p>
    #[serde(rename = "addressFamily")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address_family: Option<String>,
    /// <p>The IP address assigned to the Amazon interface.</p>
    #[serde(rename = "amazonAddress")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amazon_address: Option<String>,
    /// <p>The autonomous system (AS) number for Border Gateway Protocol (BGP) configuration.</p>
    #[serde(rename = "asn")]
    pub asn: i64,
    /// <p>The authentication key for BGP configuration.</p>
    #[serde(rename = "authKey")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auth_key: Option<String>,
    /// <p>The IP address assigned to the customer interface.</p>
    #[serde(rename = "customerAddress")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_address: Option<String>,
    /// <p>The ID of the Direct Connect gateway.</p>
    #[serde(rename = "directConnectGatewayId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub direct_connect_gateway_id: Option<String>,
    /// <p>The maximum transmission unit (MTU), in bytes. The supported values are 1500 and 9001. The default value is 1500.</p>
    #[serde(rename = "mtu")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mtu: Option<i64>,
    /// <p>The ID of the virtual private gateway.</p>
    #[serde(rename = "virtualGatewayId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub virtual_gateway_id: Option<String>,
    /// <p>The name of the virtual interface assigned by the customer network.</p>
    #[serde(rename = "virtualInterfaceName")]
    pub virtual_interface_name: String,
    /// <p>The ID of the VLAN.</p>
    #[serde(rename = "vlan")]
    pub vlan: i64,
}

/// <p>Information about a private virtual interface to be provisioned on a connection.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct NewPrivateVirtualInterfaceAllocation {
    /// <p>The address family for the BGP peer.</p>
    #[serde(rename = "addressFamily")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address_family: Option<String>,
    /// <p>The IP address assigned to the Amazon interface.</p>
    #[serde(rename = "amazonAddress")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amazon_address: Option<String>,
    /// <p>The autonomous system (AS) number for Border Gateway Protocol (BGP) configuration.</p>
    #[serde(rename = "asn")]
    pub asn: i64,
    /// <p>The authentication key for BGP configuration.</p>
    #[serde(rename = "authKey")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auth_key: Option<String>,
    /// <p>The IP address assigned to the customer interface.</p>
    #[serde(rename = "customerAddress")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_address: Option<String>,
    /// <p>The maximum transmission unit (MTU), in bytes. The supported values are 1500 and 9001. The default value is 1500.</p>
    #[serde(rename = "mtu")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mtu: Option<i64>,
    /// <p>The name of the virtual interface assigned by the customer network.</p>
    #[serde(rename = "virtualInterfaceName")]
    pub virtual_interface_name: String,
    /// <p>The ID of the VLAN.</p>
    #[serde(rename = "vlan")]
    pub vlan: i64,
}

/// <p>Information about a public virtual interface.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct NewPublicVirtualInterface {
    /// <p>The address family for the BGP peer.</p>
    #[serde(rename = "addressFamily")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address_family: Option<String>,
    /// <p>The IP address assigned to the Amazon interface.</p>
    #[serde(rename = "amazonAddress")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amazon_address: Option<String>,
    /// <p>The autonomous system (AS) number for Border Gateway Protocol (BGP) configuration.</p>
    #[serde(rename = "asn")]
    pub asn: i64,
    /// <p>The authentication key for BGP configuration.</p>
    #[serde(rename = "authKey")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auth_key: Option<String>,
    /// <p>The IP address assigned to the customer interface.</p>
    #[serde(rename = "customerAddress")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_address: Option<String>,
    /// <p>The routes to be advertised to the AWS network in this Region. Applies to public virtual interfaces.</p>
    #[serde(rename = "routeFilterPrefixes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub route_filter_prefixes: Option<Vec<RouteFilterPrefix>>,
    /// <p>The name of the virtual interface assigned by the customer network.</p>
    #[serde(rename = "virtualInterfaceName")]
    pub virtual_interface_name: String,
    /// <p>The ID of the VLAN.</p>
    #[serde(rename = "vlan")]
    pub vlan: i64,
}

/// <p>Information about a public virtual interface to be provisioned on a connection.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct NewPublicVirtualInterfaceAllocation {
    /// <p>The address family for the BGP peer.</p>
    #[serde(rename = "addressFamily")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address_family: Option<String>,
    /// <p>The IP address assigned to the Amazon interface.</p>
    #[serde(rename = "amazonAddress")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amazon_address: Option<String>,
    /// <p>The autonomous system (AS) number for Border Gateway Protocol (BGP) configuration.</p>
    #[serde(rename = "asn")]
    pub asn: i64,
    /// <p>The authentication key for BGP configuration.</p>
    #[serde(rename = "authKey")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auth_key: Option<String>,
    /// <p>The IP address assigned to the customer interface.</p>
    #[serde(rename = "customerAddress")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_address: Option<String>,
    /// <p>The routes to be advertised to the AWS network in this Region. Applies to public virtual interfaces.</p>
    #[serde(rename = "routeFilterPrefixes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub route_filter_prefixes: Option<Vec<RouteFilterPrefix>>,
    /// <p>The name of the virtual interface assigned by the customer network.</p>
    #[serde(rename = "virtualInterfaceName")]
    pub virtual_interface_name: String,
    /// <p>The ID of the VLAN.</p>
    #[serde(rename = "vlan")]
    pub vlan: i64,
}

/// <p>Information about a tag associated with an AWS Direct Connect resource.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct ResourceTag {
    /// <p>The Amazon Resource Name (ARN) of the resource.</p>
    #[serde(rename = "resourceArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_arn: Option<String>,
    /// <p>The tags.</p>
    #[serde(rename = "tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

/// <p>Information about a route filter prefix that a customer can advertise through Border Gateway Protocol (BGP) over a public virtual interface.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RouteFilterPrefix {
    /// <p>The CIDR block for the advertised route. Separate multiple routes using commas. An IPv6 CIDR must use /64 or shorter.</p>
    #[serde(rename = "cidr")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cidr: Option<String>,
}

/// <p>Information about a tag.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Tag {
    /// <p>The key.</p>
    #[serde(rename = "key")]
    pub key: String,
    /// <p>The value.</p>
    #[serde(rename = "value")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct TagResourceRequest {
    /// <p>The Amazon Resource Name (ARN) of the resource.</p>
    #[serde(rename = "resourceArn")]
    pub resource_arn: String,
    /// <p>The tags to add.</p>
    #[serde(rename = "tags")]
    pub tags: Vec<Tag>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct TagResourceResponse {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct UntagResourceRequest {
    /// <p>The Amazon Resource Name (ARN) of the resource.</p>
    #[serde(rename = "resourceArn")]
    pub resource_arn: String,
    /// <p>The tag keys of the tags to remove.</p>
    #[serde(rename = "tagKeys")]
    pub tag_keys: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct UntagResourceResponse {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct UpdateLagRequest {
    /// <p>The ID of the LAG.</p>
    #[serde(rename = "lagId")]
    pub lag_id: String,
    /// <p>The name of the LAG.</p>
    #[serde(rename = "lagName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lag_name: Option<String>,
    /// <p>The minimum number of physical connections that must be operational for the LAG itself to be operational.</p>
    #[serde(rename = "minimumLinks")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minimum_links: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct UpdateVirtualInterfaceAttributesRequest {
    /// <p>The maximum transmission unit (MTU), in bytes. The supported values are 1500 and 9001. The default value is 1500.</p>
    #[serde(rename = "mtu")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mtu: Option<i64>,
    /// <p>The ID of the virtual private interface.</p>
    #[serde(rename = "virtualInterfaceId")]
    pub virtual_interface_id: String,
}

/// <p>Information about a virtual private gateway for a private virtual interface.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct VirtualGateway {
    /// <p>The ID of the virtual private gateway.</p>
    #[serde(rename = "virtualGatewayId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub virtual_gateway_id: Option<String>,
    /// <p><p>The state of the virtual private gateway. The following are the possible values:</p> <ul> <li> <p> <code>pending</code>: Initial state after creating the virtual private gateway.</p> </li> <li> <p> <code>available</code>: Ready for use by a private virtual interface.</p> </li> <li> <p> <code>deleting</code>: Initial state after deleting the virtual private gateway.</p> </li> <li> <p> <code>deleted</code>: The virtual private gateway is deleted. The private virtual interface is unable to send traffic over this gateway.</p> </li> </ul></p>
    #[serde(rename = "virtualGatewayState")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub virtual_gateway_state: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct VirtualGateways {
    /// <p>The virtual private gateways.</p>
    #[serde(rename = "virtualGateways")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub virtual_gateways: Option<Vec<VirtualGateway>>,
}

/// <p>Information about a virtual interface.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct VirtualInterface {
    /// <p>The address family for the BGP peer.</p>
    #[serde(rename = "addressFamily")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address_family: Option<String>,
    /// <p>The IP address assigned to the Amazon interface.</p>
    #[serde(rename = "amazonAddress")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amazon_address: Option<String>,
    /// <p>The autonomous system number (ASN) for the Amazon side of the connection.</p>
    #[serde(rename = "amazonSideAsn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amazon_side_asn: Option<i64>,
    /// <p>The autonomous system (AS) number for Border Gateway Protocol (BGP) configuration.</p>
    #[serde(rename = "asn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asn: Option<i64>,
    /// <p>The authentication key for BGP configuration.</p>
    #[serde(rename = "authKey")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auth_key: Option<String>,
    /// <p>The Direct Connect endpoint on which the virtual interface terminates.</p>
    #[serde(rename = "awsDeviceV2")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aws_device_v2: Option<String>,
    /// <p>The BGP peers configured on this virtual interface.</p>
    #[serde(rename = "bgpPeers")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bgp_peers: Option<Vec<BGPPeer>>,
    /// <p>The ID of the connection.</p>
    #[serde(rename = "connectionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connection_id: Option<String>,
    /// <p>The IP address assigned to the customer interface.</p>
    #[serde(rename = "customerAddress")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_address: Option<String>,
    /// <p>The customer router configuration.</p>
    #[serde(rename = "customerRouterConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_router_config: Option<String>,
    /// <p>The ID of the Direct Connect gateway.</p>
    #[serde(rename = "directConnectGatewayId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub direct_connect_gateway_id: Option<String>,
    /// <p>Indicates whether jumbo frames (9001 MTU) are supported.</p>
    #[serde(rename = "jumboFrameCapable")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub jumbo_frame_capable: Option<bool>,
    /// <p>The location of the connection.</p>
    #[serde(rename = "location")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    /// <p>The maximum transmission unit (MTU), in bytes. The supported values are 1500 and 9001. The default value is 1500.</p>
    #[serde(rename = "mtu")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mtu: Option<i64>,
    /// <p>The ID of the AWS account that owns the virtual interface.</p>
    #[serde(rename = "ownerAccount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner_account: Option<String>,
    /// <p>The AWS Region where the virtual interface is located.</p>
    #[serde(rename = "region")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
    /// <p>The routes to be advertised to the AWS network in this Region. Applies to public virtual interfaces.</p>
    #[serde(rename = "routeFilterPrefixes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub route_filter_prefixes: Option<Vec<RouteFilterPrefix>>,
    /// <p>The ID of the virtual private gateway. Applies only to private virtual interfaces.</p>
    #[serde(rename = "virtualGatewayId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub virtual_gateway_id: Option<String>,
    /// <p>The ID of the virtual interface.</p>
    #[serde(rename = "virtualInterfaceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub virtual_interface_id: Option<String>,
    /// <p>The name of the virtual interface assigned by the customer network.</p>
    #[serde(rename = "virtualInterfaceName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub virtual_interface_name: Option<String>,
    /// <p><p>The state of the virtual interface. The following are the possible values:</p> <ul> <li> <p> <code>confirming</code>: The creation of the virtual interface is pending confirmation from the virtual interface owner. If the owner of the virtual interface is different from the owner of the connection on which it is provisioned, then the virtual interface will remain in this state until it is confirmed by the virtual interface owner.</p> </li> <li> <p> <code>verifying</code>: This state only applies to public virtual interfaces. Each public virtual interface needs validation before the virtual interface can be created.</p> </li> <li> <p> <code>pending</code>: A virtual interface is in this state from the time that it is created until the virtual interface is ready to forward traffic.</p> </li> <li> <p> <code>available</code>: A virtual interface that is able to forward traffic.</p> </li> <li> <p> <code>down</code>: A virtual interface that is BGP down.</p> </li> <li> <p> <code>deleting</code>: A virtual interface is in this state immediately after calling <a>DeleteVirtualInterface</a> until it can no longer forward traffic.</p> </li> <li> <p> <code>deleted</code>: A virtual interface that cannot forward traffic.</p> </li> <li> <p> <code>rejected</code>: The virtual interface owner has declined creation of the virtual interface. If a virtual interface in the <code>Confirming</code> state is deleted by the virtual interface owner, the virtual interface enters the <code>Rejected</code> state.</p> </li> <li> <p> <code>unknown</code>: The state of the virtual interface is not available.</p> </li> </ul></p>
    #[serde(rename = "virtualInterfaceState")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub virtual_interface_state: Option<String>,
    /// <p>The type of virtual interface. The possible values are <code>private</code> and <code>public</code>.</p>
    #[serde(rename = "virtualInterfaceType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub virtual_interface_type: Option<String>,
    /// <p>The ID of the VLAN.</p>
    #[serde(rename = "vlan")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vlan: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct VirtualInterfaces {
    /// <p>The virtual interfaces</p>
    #[serde(rename = "virtualInterfaces")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub virtual_interfaces: Option<Vec<VirtualInterface>>,
}

/// Errors returned by AllocateConnectionOnInterconnect
#[derive(Debug, PartialEq)]
pub enum AllocateConnectionOnInterconnectError {
    /// <p>One or more parameters are not valid.</p>
    DirectConnectClient(String),
    /// <p>A server-side error occurred.</p>
    DirectConnectServer(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl AllocateConnectionOnInterconnectError {
    pub fn from_response(res: BufferedHttpResponse) -> AllocateConnectionOnInterconnectError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "DirectConnectClientException" => {
                    return AllocateConnectionOnInterconnectError::DirectConnectClient(String::from(
                        error_message,
                    ));
                }
                "DirectConnectServerException" => {
                    return AllocateConnectionOnInterconnectError::DirectConnectServer(String::from(
                        error_message,
                    ));
                }
                "ValidationException" => {
                    return AllocateConnectionOnInterconnectError::Validation(
                        error_message.to_string(),
                    );
                }
                _ => {}
            }
        }
        return AllocateConnectionOnInterconnectError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for AllocateConnectionOnInterconnectError {
    fn from(err: serde_json::error::Error) -> AllocateConnectionOnInterconnectError {
        AllocateConnectionOnInterconnectError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for AllocateConnectionOnInterconnectError {
    fn from(err: CredentialsError) -> AllocateConnectionOnInterconnectError {
        AllocateConnectionOnInterconnectError::Credentials(err)
    }
}
impl From<HttpDispatchError> for AllocateConnectionOnInterconnectError {
    fn from(err: HttpDispatchError) -> AllocateConnectionOnInterconnectError {
        AllocateConnectionOnInterconnectError::HttpDispatch(err)
    }
}
impl From<io::Error> for AllocateConnectionOnInterconnectError {
    fn from(err: io::Error) -> AllocateConnectionOnInterconnectError {
        AllocateConnectionOnInterconnectError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for AllocateConnectionOnInterconnectError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for AllocateConnectionOnInterconnectError {
    fn description(&self) -> &str {
        match *self {
            AllocateConnectionOnInterconnectError::DirectConnectClient(ref cause) => cause,
            AllocateConnectionOnInterconnectError::DirectConnectServer(ref cause) => cause,
            AllocateConnectionOnInterconnectError::Validation(ref cause) => cause,
            AllocateConnectionOnInterconnectError::Credentials(ref err) => err.description(),
            AllocateConnectionOnInterconnectError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            AllocateConnectionOnInterconnectError::ParseError(ref cause) => cause,
            AllocateConnectionOnInterconnectError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by AllocateHostedConnection
#[derive(Debug, PartialEq)]
pub enum AllocateHostedConnectionError {
    /// <p>One or more parameters are not valid.</p>
    DirectConnectClient(String),
    /// <p>A server-side error occurred.</p>
    DirectConnectServer(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl AllocateHostedConnectionError {
    pub fn from_response(res: BufferedHttpResponse) -> AllocateHostedConnectionError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "DirectConnectClientException" => {
                    return AllocateHostedConnectionError::DirectConnectClient(String::from(
                        error_message,
                    ));
                }
                "DirectConnectServerException" => {
                    return AllocateHostedConnectionError::DirectConnectServer(String::from(
                        error_message,
                    ));
                }
                "ValidationException" => {
                    return AllocateHostedConnectionError::Validation(error_message.to_string());
                }
                _ => {}
            }
        }
        return AllocateHostedConnectionError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for AllocateHostedConnectionError {
    fn from(err: serde_json::error::Error) -> AllocateHostedConnectionError {
        AllocateHostedConnectionError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for AllocateHostedConnectionError {
    fn from(err: CredentialsError) -> AllocateHostedConnectionError {
        AllocateHostedConnectionError::Credentials(err)
    }
}
impl From<HttpDispatchError> for AllocateHostedConnectionError {
    fn from(err: HttpDispatchError) -> AllocateHostedConnectionError {
        AllocateHostedConnectionError::HttpDispatch(err)
    }
}
impl From<io::Error> for AllocateHostedConnectionError {
    fn from(err: io::Error) -> AllocateHostedConnectionError {
        AllocateHostedConnectionError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for AllocateHostedConnectionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for AllocateHostedConnectionError {
    fn description(&self) -> &str {
        match *self {
            AllocateHostedConnectionError::DirectConnectClient(ref cause) => cause,
            AllocateHostedConnectionError::DirectConnectServer(ref cause) => cause,
            AllocateHostedConnectionError::Validation(ref cause) => cause,
            AllocateHostedConnectionError::Credentials(ref err) => err.description(),
            AllocateHostedConnectionError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            AllocateHostedConnectionError::ParseError(ref cause) => cause,
            AllocateHostedConnectionError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by AllocatePrivateVirtualInterface
#[derive(Debug, PartialEq)]
pub enum AllocatePrivateVirtualInterfaceError {
    /// <p>One or more parameters are not valid.</p>
    DirectConnectClient(String),
    /// <p>A server-side error occurred.</p>
    DirectConnectServer(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl AllocatePrivateVirtualInterfaceError {
    pub fn from_response(res: BufferedHttpResponse) -> AllocatePrivateVirtualInterfaceError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "DirectConnectClientException" => {
                    return AllocatePrivateVirtualInterfaceError::DirectConnectClient(String::from(
                        error_message,
                    ));
                }
                "DirectConnectServerException" => {
                    return AllocatePrivateVirtualInterfaceError::DirectConnectServer(String::from(
                        error_message,
                    ));
                }
                "ValidationException" => {
                    return AllocatePrivateVirtualInterfaceError::Validation(
                        error_message.to_string(),
                    );
                }
                _ => {}
            }
        }
        return AllocatePrivateVirtualInterfaceError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for AllocatePrivateVirtualInterfaceError {
    fn from(err: serde_json::error::Error) -> AllocatePrivateVirtualInterfaceError {
        AllocatePrivateVirtualInterfaceError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for AllocatePrivateVirtualInterfaceError {
    fn from(err: CredentialsError) -> AllocatePrivateVirtualInterfaceError {
        AllocatePrivateVirtualInterfaceError::Credentials(err)
    }
}
impl From<HttpDispatchError> for AllocatePrivateVirtualInterfaceError {
    fn from(err: HttpDispatchError) -> AllocatePrivateVirtualInterfaceError {
        AllocatePrivateVirtualInterfaceError::HttpDispatch(err)
    }
}
impl From<io::Error> for AllocatePrivateVirtualInterfaceError {
    fn from(err: io::Error) -> AllocatePrivateVirtualInterfaceError {
        AllocatePrivateVirtualInterfaceError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for AllocatePrivateVirtualInterfaceError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for AllocatePrivateVirtualInterfaceError {
    fn description(&self) -> &str {
        match *self {
            AllocatePrivateVirtualInterfaceError::DirectConnectClient(ref cause) => cause,
            AllocatePrivateVirtualInterfaceError::DirectConnectServer(ref cause) => cause,
            AllocatePrivateVirtualInterfaceError::Validation(ref cause) => cause,
            AllocatePrivateVirtualInterfaceError::Credentials(ref err) => err.description(),
            AllocatePrivateVirtualInterfaceError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            AllocatePrivateVirtualInterfaceError::ParseError(ref cause) => cause,
            AllocatePrivateVirtualInterfaceError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by AllocatePublicVirtualInterface
#[derive(Debug, PartialEq)]
pub enum AllocatePublicVirtualInterfaceError {
    /// <p>One or more parameters are not valid.</p>
    DirectConnectClient(String),
    /// <p>A server-side error occurred.</p>
    DirectConnectServer(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl AllocatePublicVirtualInterfaceError {
    pub fn from_response(res: BufferedHttpResponse) -> AllocatePublicVirtualInterfaceError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "DirectConnectClientException" => {
                    return AllocatePublicVirtualInterfaceError::DirectConnectClient(String::from(
                        error_message,
                    ));
                }
                "DirectConnectServerException" => {
                    return AllocatePublicVirtualInterfaceError::DirectConnectServer(String::from(
                        error_message,
                    ));
                }
                "ValidationException" => {
                    return AllocatePublicVirtualInterfaceError::Validation(
                        error_message.to_string(),
                    );
                }
                _ => {}
            }
        }
        return AllocatePublicVirtualInterfaceError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for AllocatePublicVirtualInterfaceError {
    fn from(err: serde_json::error::Error) -> AllocatePublicVirtualInterfaceError {
        AllocatePublicVirtualInterfaceError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for AllocatePublicVirtualInterfaceError {
    fn from(err: CredentialsError) -> AllocatePublicVirtualInterfaceError {
        AllocatePublicVirtualInterfaceError::Credentials(err)
    }
}
impl From<HttpDispatchError> for AllocatePublicVirtualInterfaceError {
    fn from(err: HttpDispatchError) -> AllocatePublicVirtualInterfaceError {
        AllocatePublicVirtualInterfaceError::HttpDispatch(err)
    }
}
impl From<io::Error> for AllocatePublicVirtualInterfaceError {
    fn from(err: io::Error) -> AllocatePublicVirtualInterfaceError {
        AllocatePublicVirtualInterfaceError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for AllocatePublicVirtualInterfaceError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for AllocatePublicVirtualInterfaceError {
    fn description(&self) -> &str {
        match *self {
            AllocatePublicVirtualInterfaceError::DirectConnectClient(ref cause) => cause,
            AllocatePublicVirtualInterfaceError::DirectConnectServer(ref cause) => cause,
            AllocatePublicVirtualInterfaceError::Validation(ref cause) => cause,
            AllocatePublicVirtualInterfaceError::Credentials(ref err) => err.description(),
            AllocatePublicVirtualInterfaceError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            AllocatePublicVirtualInterfaceError::ParseError(ref cause) => cause,
            AllocatePublicVirtualInterfaceError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by AssociateConnectionWithLag
#[derive(Debug, PartialEq)]
pub enum AssociateConnectionWithLagError {
    /// <p>One or more parameters are not valid.</p>
    DirectConnectClient(String),
    /// <p>A server-side error occurred.</p>
    DirectConnectServer(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl AssociateConnectionWithLagError {
    pub fn from_response(res: BufferedHttpResponse) -> AssociateConnectionWithLagError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "DirectConnectClientException" => {
                    return AssociateConnectionWithLagError::DirectConnectClient(String::from(
                        error_message,
                    ));
                }
                "DirectConnectServerException" => {
                    return AssociateConnectionWithLagError::DirectConnectServer(String::from(
                        error_message,
                    ));
                }
                "ValidationException" => {
                    return AssociateConnectionWithLagError::Validation(error_message.to_string());
                }
                _ => {}
            }
        }
        return AssociateConnectionWithLagError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for AssociateConnectionWithLagError {
    fn from(err: serde_json::error::Error) -> AssociateConnectionWithLagError {
        AssociateConnectionWithLagError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for AssociateConnectionWithLagError {
    fn from(err: CredentialsError) -> AssociateConnectionWithLagError {
        AssociateConnectionWithLagError::Credentials(err)
    }
}
impl From<HttpDispatchError> for AssociateConnectionWithLagError {
    fn from(err: HttpDispatchError) -> AssociateConnectionWithLagError {
        AssociateConnectionWithLagError::HttpDispatch(err)
    }
}
impl From<io::Error> for AssociateConnectionWithLagError {
    fn from(err: io::Error) -> AssociateConnectionWithLagError {
        AssociateConnectionWithLagError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for AssociateConnectionWithLagError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for AssociateConnectionWithLagError {
    fn description(&self) -> &str {
        match *self {
            AssociateConnectionWithLagError::DirectConnectClient(ref cause) => cause,
            AssociateConnectionWithLagError::DirectConnectServer(ref cause) => cause,
            AssociateConnectionWithLagError::Validation(ref cause) => cause,
            AssociateConnectionWithLagError::Credentials(ref err) => err.description(),
            AssociateConnectionWithLagError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            AssociateConnectionWithLagError::ParseError(ref cause) => cause,
            AssociateConnectionWithLagError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by AssociateHostedConnection
#[derive(Debug, PartialEq)]
pub enum AssociateHostedConnectionError {
    /// <p>One or more parameters are not valid.</p>
    DirectConnectClient(String),
    /// <p>A server-side error occurred.</p>
    DirectConnectServer(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl AssociateHostedConnectionError {
    pub fn from_response(res: BufferedHttpResponse) -> AssociateHostedConnectionError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "DirectConnectClientException" => {
                    return AssociateHostedConnectionError::DirectConnectClient(String::from(
                        error_message,
                    ));
                }
                "DirectConnectServerException" => {
                    return AssociateHostedConnectionError::DirectConnectServer(String::from(
                        error_message,
                    ));
                }
                "ValidationException" => {
                    return AssociateHostedConnectionError::Validation(error_message.to_string());
                }
                _ => {}
            }
        }
        return AssociateHostedConnectionError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for AssociateHostedConnectionError {
    fn from(err: serde_json::error::Error) -> AssociateHostedConnectionError {
        AssociateHostedConnectionError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for AssociateHostedConnectionError {
    fn from(err: CredentialsError) -> AssociateHostedConnectionError {
        AssociateHostedConnectionError::Credentials(err)
    }
}
impl From<HttpDispatchError> for AssociateHostedConnectionError {
    fn from(err: HttpDispatchError) -> AssociateHostedConnectionError {
        AssociateHostedConnectionError::HttpDispatch(err)
    }
}
impl From<io::Error> for AssociateHostedConnectionError {
    fn from(err: io::Error) -> AssociateHostedConnectionError {
        AssociateHostedConnectionError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for AssociateHostedConnectionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for AssociateHostedConnectionError {
    fn description(&self) -> &str {
        match *self {
            AssociateHostedConnectionError::DirectConnectClient(ref cause) => cause,
            AssociateHostedConnectionError::DirectConnectServer(ref cause) => cause,
            AssociateHostedConnectionError::Validation(ref cause) => cause,
            AssociateHostedConnectionError::Credentials(ref err) => err.description(),
            AssociateHostedConnectionError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            AssociateHostedConnectionError::ParseError(ref cause) => cause,
            AssociateHostedConnectionError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by AssociateVirtualInterface
#[derive(Debug, PartialEq)]
pub enum AssociateVirtualInterfaceError {
    /// <p>One or more parameters are not valid.</p>
    DirectConnectClient(String),
    /// <p>A server-side error occurred.</p>
    DirectConnectServer(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl AssociateVirtualInterfaceError {
    pub fn from_response(res: BufferedHttpResponse) -> AssociateVirtualInterfaceError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "DirectConnectClientException" => {
                    return AssociateVirtualInterfaceError::DirectConnectClient(String::from(
                        error_message,
                    ));
                }
                "DirectConnectServerException" => {
                    return AssociateVirtualInterfaceError::DirectConnectServer(String::from(
                        error_message,
                    ));
                }
                "ValidationException" => {
                    return AssociateVirtualInterfaceError::Validation(error_message.to_string());
                }
                _ => {}
            }
        }
        return AssociateVirtualInterfaceError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for AssociateVirtualInterfaceError {
    fn from(err: serde_json::error::Error) -> AssociateVirtualInterfaceError {
        AssociateVirtualInterfaceError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for AssociateVirtualInterfaceError {
    fn from(err: CredentialsError) -> AssociateVirtualInterfaceError {
        AssociateVirtualInterfaceError::Credentials(err)
    }
}
impl From<HttpDispatchError> for AssociateVirtualInterfaceError {
    fn from(err: HttpDispatchError) -> AssociateVirtualInterfaceError {
        AssociateVirtualInterfaceError::HttpDispatch(err)
    }
}
impl From<io::Error> for AssociateVirtualInterfaceError {
    fn from(err: io::Error) -> AssociateVirtualInterfaceError {
        AssociateVirtualInterfaceError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for AssociateVirtualInterfaceError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for AssociateVirtualInterfaceError {
    fn description(&self) -> &str {
        match *self {
            AssociateVirtualInterfaceError::DirectConnectClient(ref cause) => cause,
            AssociateVirtualInterfaceError::DirectConnectServer(ref cause) => cause,
            AssociateVirtualInterfaceError::Validation(ref cause) => cause,
            AssociateVirtualInterfaceError::Credentials(ref err) => err.description(),
            AssociateVirtualInterfaceError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            AssociateVirtualInterfaceError::ParseError(ref cause) => cause,
            AssociateVirtualInterfaceError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by ConfirmConnection
#[derive(Debug, PartialEq)]
pub enum ConfirmConnectionError {
    /// <p>One or more parameters are not valid.</p>
    DirectConnectClient(String),
    /// <p>A server-side error occurred.</p>
    DirectConnectServer(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl ConfirmConnectionError {
    pub fn from_response(res: BufferedHttpResponse) -> ConfirmConnectionError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "DirectConnectClientException" => {
                    return ConfirmConnectionError::DirectConnectClient(String::from(error_message));
                }
                "DirectConnectServerException" => {
                    return ConfirmConnectionError::DirectConnectServer(String::from(error_message));
                }
                "ValidationException" => {
                    return ConfirmConnectionError::Validation(error_message.to_string());
                }
                _ => {}
            }
        }
        return ConfirmConnectionError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for ConfirmConnectionError {
    fn from(err: serde_json::error::Error) -> ConfirmConnectionError {
        ConfirmConnectionError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for ConfirmConnectionError {
    fn from(err: CredentialsError) -> ConfirmConnectionError {
        ConfirmConnectionError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ConfirmConnectionError {
    fn from(err: HttpDispatchError) -> ConfirmConnectionError {
        ConfirmConnectionError::HttpDispatch(err)
    }
}
impl From<io::Error> for ConfirmConnectionError {
    fn from(err: io::Error) -> ConfirmConnectionError {
        ConfirmConnectionError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for ConfirmConnectionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ConfirmConnectionError {
    fn description(&self) -> &str {
        match *self {
            ConfirmConnectionError::DirectConnectClient(ref cause) => cause,
            ConfirmConnectionError::DirectConnectServer(ref cause) => cause,
            ConfirmConnectionError::Validation(ref cause) => cause,
            ConfirmConnectionError::Credentials(ref err) => err.description(),
            ConfirmConnectionError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            ConfirmConnectionError::ParseError(ref cause) => cause,
            ConfirmConnectionError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by ConfirmPrivateVirtualInterface
#[derive(Debug, PartialEq)]
pub enum ConfirmPrivateVirtualInterfaceError {
    /// <p>One or more parameters are not valid.</p>
    DirectConnectClient(String),
    /// <p>A server-side error occurred.</p>
    DirectConnectServer(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl ConfirmPrivateVirtualInterfaceError {
    pub fn from_response(res: BufferedHttpResponse) -> ConfirmPrivateVirtualInterfaceError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "DirectConnectClientException" => {
                    return ConfirmPrivateVirtualInterfaceError::DirectConnectClient(String::from(
                        error_message,
                    ));
                }
                "DirectConnectServerException" => {
                    return ConfirmPrivateVirtualInterfaceError::DirectConnectServer(String::from(
                        error_message,
                    ));
                }
                "ValidationException" => {
                    return ConfirmPrivateVirtualInterfaceError::Validation(
                        error_message.to_string(),
                    );
                }
                _ => {}
            }
        }
        return ConfirmPrivateVirtualInterfaceError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for ConfirmPrivateVirtualInterfaceError {
    fn from(err: serde_json::error::Error) -> ConfirmPrivateVirtualInterfaceError {
        ConfirmPrivateVirtualInterfaceError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for ConfirmPrivateVirtualInterfaceError {
    fn from(err: CredentialsError) -> ConfirmPrivateVirtualInterfaceError {
        ConfirmPrivateVirtualInterfaceError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ConfirmPrivateVirtualInterfaceError {
    fn from(err: HttpDispatchError) -> ConfirmPrivateVirtualInterfaceError {
        ConfirmPrivateVirtualInterfaceError::HttpDispatch(err)
    }
}
impl From<io::Error> for ConfirmPrivateVirtualInterfaceError {
    fn from(err: io::Error) -> ConfirmPrivateVirtualInterfaceError {
        ConfirmPrivateVirtualInterfaceError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for ConfirmPrivateVirtualInterfaceError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ConfirmPrivateVirtualInterfaceError {
    fn description(&self) -> &str {
        match *self {
            ConfirmPrivateVirtualInterfaceError::DirectConnectClient(ref cause) => cause,
            ConfirmPrivateVirtualInterfaceError::DirectConnectServer(ref cause) => cause,
            ConfirmPrivateVirtualInterfaceError::Validation(ref cause) => cause,
            ConfirmPrivateVirtualInterfaceError::Credentials(ref err) => err.description(),
            ConfirmPrivateVirtualInterfaceError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            ConfirmPrivateVirtualInterfaceError::ParseError(ref cause) => cause,
            ConfirmPrivateVirtualInterfaceError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by ConfirmPublicVirtualInterface
#[derive(Debug, PartialEq)]
pub enum ConfirmPublicVirtualInterfaceError {
    /// <p>One or more parameters are not valid.</p>
    DirectConnectClient(String),
    /// <p>A server-side error occurred.</p>
    DirectConnectServer(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl ConfirmPublicVirtualInterfaceError {
    pub fn from_response(res: BufferedHttpResponse) -> ConfirmPublicVirtualInterfaceError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "DirectConnectClientException" => {
                    return ConfirmPublicVirtualInterfaceError::DirectConnectClient(String::from(
                        error_message,
                    ));
                }
                "DirectConnectServerException" => {
                    return ConfirmPublicVirtualInterfaceError::DirectConnectServer(String::from(
                        error_message,
                    ));
                }
                "ValidationException" => {
                    return ConfirmPublicVirtualInterfaceError::Validation(error_message.to_string());
                }
                _ => {}
            }
        }
        return ConfirmPublicVirtualInterfaceError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for ConfirmPublicVirtualInterfaceError {
    fn from(err: serde_json::error::Error) -> ConfirmPublicVirtualInterfaceError {
        ConfirmPublicVirtualInterfaceError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for ConfirmPublicVirtualInterfaceError {
    fn from(err: CredentialsError) -> ConfirmPublicVirtualInterfaceError {
        ConfirmPublicVirtualInterfaceError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ConfirmPublicVirtualInterfaceError {
    fn from(err: HttpDispatchError) -> ConfirmPublicVirtualInterfaceError {
        ConfirmPublicVirtualInterfaceError::HttpDispatch(err)
    }
}
impl From<io::Error> for ConfirmPublicVirtualInterfaceError {
    fn from(err: io::Error) -> ConfirmPublicVirtualInterfaceError {
        ConfirmPublicVirtualInterfaceError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for ConfirmPublicVirtualInterfaceError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ConfirmPublicVirtualInterfaceError {
    fn description(&self) -> &str {
        match *self {
            ConfirmPublicVirtualInterfaceError::DirectConnectClient(ref cause) => cause,
            ConfirmPublicVirtualInterfaceError::DirectConnectServer(ref cause) => cause,
            ConfirmPublicVirtualInterfaceError::Validation(ref cause) => cause,
            ConfirmPublicVirtualInterfaceError::Credentials(ref err) => err.description(),
            ConfirmPublicVirtualInterfaceError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            ConfirmPublicVirtualInterfaceError::ParseError(ref cause) => cause,
            ConfirmPublicVirtualInterfaceError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by CreateBGPPeer
#[derive(Debug, PartialEq)]
pub enum CreateBGPPeerError {
    /// <p>One or more parameters are not valid.</p>
    DirectConnectClient(String),
    /// <p>A server-side error occurred.</p>
    DirectConnectServer(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl CreateBGPPeerError {
    pub fn from_response(res: BufferedHttpResponse) -> CreateBGPPeerError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "DirectConnectClientException" => {
                    return CreateBGPPeerError::DirectConnectClient(String::from(error_message));
                }
                "DirectConnectServerException" => {
                    return CreateBGPPeerError::DirectConnectServer(String::from(error_message));
                }
                "ValidationException" => {
                    return CreateBGPPeerError::Validation(error_message.to_string());
                }
                _ => {}
            }
        }
        return CreateBGPPeerError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for CreateBGPPeerError {
    fn from(err: serde_json::error::Error) -> CreateBGPPeerError {
        CreateBGPPeerError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for CreateBGPPeerError {
    fn from(err: CredentialsError) -> CreateBGPPeerError {
        CreateBGPPeerError::Credentials(err)
    }
}
impl From<HttpDispatchError> for CreateBGPPeerError {
    fn from(err: HttpDispatchError) -> CreateBGPPeerError {
        CreateBGPPeerError::HttpDispatch(err)
    }
}
impl From<io::Error> for CreateBGPPeerError {
    fn from(err: io::Error) -> CreateBGPPeerError {
        CreateBGPPeerError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for CreateBGPPeerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateBGPPeerError {
    fn description(&self) -> &str {
        match *self {
            CreateBGPPeerError::DirectConnectClient(ref cause) => cause,
            CreateBGPPeerError::DirectConnectServer(ref cause) => cause,
            CreateBGPPeerError::Validation(ref cause) => cause,
            CreateBGPPeerError::Credentials(ref err) => err.description(),
            CreateBGPPeerError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            CreateBGPPeerError::ParseError(ref cause) => cause,
            CreateBGPPeerError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by CreateConnection
#[derive(Debug, PartialEq)]
pub enum CreateConnectionError {
    /// <p>One or more parameters are not valid.</p>
    DirectConnectClient(String),
    /// <p>A server-side error occurred.</p>
    DirectConnectServer(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl CreateConnectionError {
    pub fn from_response(res: BufferedHttpResponse) -> CreateConnectionError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "DirectConnectClientException" => {
                    return CreateConnectionError::DirectConnectClient(String::from(error_message));
                }
                "DirectConnectServerException" => {
                    return CreateConnectionError::DirectConnectServer(String::from(error_message));
                }
                "ValidationException" => {
                    return CreateConnectionError::Validation(error_message.to_string());
                }
                _ => {}
            }
        }
        return CreateConnectionError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for CreateConnectionError {
    fn from(err: serde_json::error::Error) -> CreateConnectionError {
        CreateConnectionError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for CreateConnectionError {
    fn from(err: CredentialsError) -> CreateConnectionError {
        CreateConnectionError::Credentials(err)
    }
}
impl From<HttpDispatchError> for CreateConnectionError {
    fn from(err: HttpDispatchError) -> CreateConnectionError {
        CreateConnectionError::HttpDispatch(err)
    }
}
impl From<io::Error> for CreateConnectionError {
    fn from(err: io::Error) -> CreateConnectionError {
        CreateConnectionError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for CreateConnectionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateConnectionError {
    fn description(&self) -> &str {
        match *self {
            CreateConnectionError::DirectConnectClient(ref cause) => cause,
            CreateConnectionError::DirectConnectServer(ref cause) => cause,
            CreateConnectionError::Validation(ref cause) => cause,
            CreateConnectionError::Credentials(ref err) => err.description(),
            CreateConnectionError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            CreateConnectionError::ParseError(ref cause) => cause,
            CreateConnectionError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by CreateDirectConnectGateway
#[derive(Debug, PartialEq)]
pub enum CreateDirectConnectGatewayError {
    /// <p>One or more parameters are not valid.</p>
    DirectConnectClient(String),
    /// <p>A server-side error occurred.</p>
    DirectConnectServer(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl CreateDirectConnectGatewayError {
    pub fn from_response(res: BufferedHttpResponse) -> CreateDirectConnectGatewayError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "DirectConnectClientException" => {
                    return CreateDirectConnectGatewayError::DirectConnectClient(String::from(
                        error_message,
                    ));
                }
                "DirectConnectServerException" => {
                    return CreateDirectConnectGatewayError::DirectConnectServer(String::from(
                        error_message,
                    ));
                }
                "ValidationException" => {
                    return CreateDirectConnectGatewayError::Validation(error_message.to_string());
                }
                _ => {}
            }
        }
        return CreateDirectConnectGatewayError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for CreateDirectConnectGatewayError {
    fn from(err: serde_json::error::Error) -> CreateDirectConnectGatewayError {
        CreateDirectConnectGatewayError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for CreateDirectConnectGatewayError {
    fn from(err: CredentialsError) -> CreateDirectConnectGatewayError {
        CreateDirectConnectGatewayError::Credentials(err)
    }
}
impl From<HttpDispatchError> for CreateDirectConnectGatewayError {
    fn from(err: HttpDispatchError) -> CreateDirectConnectGatewayError {
        CreateDirectConnectGatewayError::HttpDispatch(err)
    }
}
impl From<io::Error> for CreateDirectConnectGatewayError {
    fn from(err: io::Error) -> CreateDirectConnectGatewayError {
        CreateDirectConnectGatewayError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for CreateDirectConnectGatewayError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateDirectConnectGatewayError {
    fn description(&self) -> &str {
        match *self {
            CreateDirectConnectGatewayError::DirectConnectClient(ref cause) => cause,
            CreateDirectConnectGatewayError::DirectConnectServer(ref cause) => cause,
            CreateDirectConnectGatewayError::Validation(ref cause) => cause,
            CreateDirectConnectGatewayError::Credentials(ref err) => err.description(),
            CreateDirectConnectGatewayError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            CreateDirectConnectGatewayError::ParseError(ref cause) => cause,
            CreateDirectConnectGatewayError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by CreateDirectConnectGatewayAssociation
#[derive(Debug, PartialEq)]
pub enum CreateDirectConnectGatewayAssociationError {
    /// <p>One or more parameters are not valid.</p>
    DirectConnectClient(String),
    /// <p>A server-side error occurred.</p>
    DirectConnectServer(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl CreateDirectConnectGatewayAssociationError {
    pub fn from_response(res: BufferedHttpResponse) -> CreateDirectConnectGatewayAssociationError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "DirectConnectClientException" => {
                    return CreateDirectConnectGatewayAssociationError::DirectConnectClient(
                        String::from(error_message),
                    );
                }
                "DirectConnectServerException" => {
                    return CreateDirectConnectGatewayAssociationError::DirectConnectServer(
                        String::from(error_message),
                    );
                }
                "ValidationException" => {
                    return CreateDirectConnectGatewayAssociationError::Validation(
                        error_message.to_string(),
                    );
                }
                _ => {}
            }
        }
        return CreateDirectConnectGatewayAssociationError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for CreateDirectConnectGatewayAssociationError {
    fn from(err: serde_json::error::Error) -> CreateDirectConnectGatewayAssociationError {
        CreateDirectConnectGatewayAssociationError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for CreateDirectConnectGatewayAssociationError {
    fn from(err: CredentialsError) -> CreateDirectConnectGatewayAssociationError {
        CreateDirectConnectGatewayAssociationError::Credentials(err)
    }
}
impl From<HttpDispatchError> for CreateDirectConnectGatewayAssociationError {
    fn from(err: HttpDispatchError) -> CreateDirectConnectGatewayAssociationError {
        CreateDirectConnectGatewayAssociationError::HttpDispatch(err)
    }
}
impl From<io::Error> for CreateDirectConnectGatewayAssociationError {
    fn from(err: io::Error) -> CreateDirectConnectGatewayAssociationError {
        CreateDirectConnectGatewayAssociationError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for CreateDirectConnectGatewayAssociationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateDirectConnectGatewayAssociationError {
    fn description(&self) -> &str {
        match *self {
            CreateDirectConnectGatewayAssociationError::DirectConnectClient(ref cause) => cause,
            CreateDirectConnectGatewayAssociationError::DirectConnectServer(ref cause) => cause,
            CreateDirectConnectGatewayAssociationError::Validation(ref cause) => cause,
            CreateDirectConnectGatewayAssociationError::Credentials(ref err) => err.description(),
            CreateDirectConnectGatewayAssociationError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            CreateDirectConnectGatewayAssociationError::ParseError(ref cause) => cause,
            CreateDirectConnectGatewayAssociationError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by CreateInterconnect
#[derive(Debug, PartialEq)]
pub enum CreateInterconnectError {
    /// <p>One or more parameters are not valid.</p>
    DirectConnectClient(String),
    /// <p>A server-side error occurred.</p>
    DirectConnectServer(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl CreateInterconnectError {
    pub fn from_response(res: BufferedHttpResponse) -> CreateInterconnectError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "DirectConnectClientException" => {
                    return CreateInterconnectError::DirectConnectClient(String::from(error_message));
                }
                "DirectConnectServerException" => {
                    return CreateInterconnectError::DirectConnectServer(String::from(error_message));
                }
                "ValidationException" => {
                    return CreateInterconnectError::Validation(error_message.to_string());
                }
                _ => {}
            }
        }
        return CreateInterconnectError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for CreateInterconnectError {
    fn from(err: serde_json::error::Error) -> CreateInterconnectError {
        CreateInterconnectError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for CreateInterconnectError {
    fn from(err: CredentialsError) -> CreateInterconnectError {
        CreateInterconnectError::Credentials(err)
    }
}
impl From<HttpDispatchError> for CreateInterconnectError {
    fn from(err: HttpDispatchError) -> CreateInterconnectError {
        CreateInterconnectError::HttpDispatch(err)
    }
}
impl From<io::Error> for CreateInterconnectError {
    fn from(err: io::Error) -> CreateInterconnectError {
        CreateInterconnectError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for CreateInterconnectError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateInterconnectError {
    fn description(&self) -> &str {
        match *self {
            CreateInterconnectError::DirectConnectClient(ref cause) => cause,
            CreateInterconnectError::DirectConnectServer(ref cause) => cause,
            CreateInterconnectError::Validation(ref cause) => cause,
            CreateInterconnectError::Credentials(ref err) => err.description(),
            CreateInterconnectError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            CreateInterconnectError::ParseError(ref cause) => cause,
            CreateInterconnectError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by CreateLag
#[derive(Debug, PartialEq)]
pub enum CreateLagError {
    /// <p>One or more parameters are not valid.</p>
    DirectConnectClient(String),
    /// <p>A server-side error occurred.</p>
    DirectConnectServer(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl CreateLagError {
    pub fn from_response(res: BufferedHttpResponse) -> CreateLagError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "DirectConnectClientException" => {
                    return CreateLagError::DirectConnectClient(String::from(error_message));
                }
                "DirectConnectServerException" => {
                    return CreateLagError::DirectConnectServer(String::from(error_message));
                }
                "ValidationException" => {
                    return CreateLagError::Validation(error_message.to_string());
                }
                _ => {}
            }
        }
        return CreateLagError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for CreateLagError {
    fn from(err: serde_json::error::Error) -> CreateLagError {
        CreateLagError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for CreateLagError {
    fn from(err: CredentialsError) -> CreateLagError {
        CreateLagError::Credentials(err)
    }
}
impl From<HttpDispatchError> for CreateLagError {
    fn from(err: HttpDispatchError) -> CreateLagError {
        CreateLagError::HttpDispatch(err)
    }
}
impl From<io::Error> for CreateLagError {
    fn from(err: io::Error) -> CreateLagError {
        CreateLagError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for CreateLagError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateLagError {
    fn description(&self) -> &str {
        match *self {
            CreateLagError::DirectConnectClient(ref cause) => cause,
            CreateLagError::DirectConnectServer(ref cause) => cause,
            CreateLagError::Validation(ref cause) => cause,
            CreateLagError::Credentials(ref err) => err.description(),
            CreateLagError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            CreateLagError::ParseError(ref cause) => cause,
            CreateLagError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by CreatePrivateVirtualInterface
#[derive(Debug, PartialEq)]
pub enum CreatePrivateVirtualInterfaceError {
    /// <p>One or more parameters are not valid.</p>
    DirectConnectClient(String),
    /// <p>A server-side error occurred.</p>
    DirectConnectServer(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl CreatePrivateVirtualInterfaceError {
    pub fn from_response(res: BufferedHttpResponse) -> CreatePrivateVirtualInterfaceError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "DirectConnectClientException" => {
                    return CreatePrivateVirtualInterfaceError::DirectConnectClient(String::from(
                        error_message,
                    ));
                }
                "DirectConnectServerException" => {
                    return CreatePrivateVirtualInterfaceError::DirectConnectServer(String::from(
                        error_message,
                    ));
                }
                "ValidationException" => {
                    return CreatePrivateVirtualInterfaceError::Validation(error_message.to_string());
                }
                _ => {}
            }
        }
        return CreatePrivateVirtualInterfaceError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for CreatePrivateVirtualInterfaceError {
    fn from(err: serde_json::error::Error) -> CreatePrivateVirtualInterfaceError {
        CreatePrivateVirtualInterfaceError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for CreatePrivateVirtualInterfaceError {
    fn from(err: CredentialsError) -> CreatePrivateVirtualInterfaceError {
        CreatePrivateVirtualInterfaceError::Credentials(err)
    }
}
impl From<HttpDispatchError> for CreatePrivateVirtualInterfaceError {
    fn from(err: HttpDispatchError) -> CreatePrivateVirtualInterfaceError {
        CreatePrivateVirtualInterfaceError::HttpDispatch(err)
    }
}
impl From<io::Error> for CreatePrivateVirtualInterfaceError {
    fn from(err: io::Error) -> CreatePrivateVirtualInterfaceError {
        CreatePrivateVirtualInterfaceError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for CreatePrivateVirtualInterfaceError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreatePrivateVirtualInterfaceError {
    fn description(&self) -> &str {
        match *self {
            CreatePrivateVirtualInterfaceError::DirectConnectClient(ref cause) => cause,
            CreatePrivateVirtualInterfaceError::DirectConnectServer(ref cause) => cause,
            CreatePrivateVirtualInterfaceError::Validation(ref cause) => cause,
            CreatePrivateVirtualInterfaceError::Credentials(ref err) => err.description(),
            CreatePrivateVirtualInterfaceError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            CreatePrivateVirtualInterfaceError::ParseError(ref cause) => cause,
            CreatePrivateVirtualInterfaceError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by CreatePublicVirtualInterface
#[derive(Debug, PartialEq)]
pub enum CreatePublicVirtualInterfaceError {
    /// <p>One or more parameters are not valid.</p>
    DirectConnectClient(String),
    /// <p>A server-side error occurred.</p>
    DirectConnectServer(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl CreatePublicVirtualInterfaceError {
    pub fn from_response(res: BufferedHttpResponse) -> CreatePublicVirtualInterfaceError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "DirectConnectClientException" => {
                    return CreatePublicVirtualInterfaceError::DirectConnectClient(String::from(
                        error_message,
                    ));
                }
                "DirectConnectServerException" => {
                    return CreatePublicVirtualInterfaceError::DirectConnectServer(String::from(
                        error_message,
                    ));
                }
                "ValidationException" => {
                    return CreatePublicVirtualInterfaceError::Validation(error_message.to_string());
                }
                _ => {}
            }
        }
        return CreatePublicVirtualInterfaceError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for CreatePublicVirtualInterfaceError {
    fn from(err: serde_json::error::Error) -> CreatePublicVirtualInterfaceError {
        CreatePublicVirtualInterfaceError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for CreatePublicVirtualInterfaceError {
    fn from(err: CredentialsError) -> CreatePublicVirtualInterfaceError {
        CreatePublicVirtualInterfaceError::Credentials(err)
    }
}
impl From<HttpDispatchError> for CreatePublicVirtualInterfaceError {
    fn from(err: HttpDispatchError) -> CreatePublicVirtualInterfaceError {
        CreatePublicVirtualInterfaceError::HttpDispatch(err)
    }
}
impl From<io::Error> for CreatePublicVirtualInterfaceError {
    fn from(err: io::Error) -> CreatePublicVirtualInterfaceError {
        CreatePublicVirtualInterfaceError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for CreatePublicVirtualInterfaceError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreatePublicVirtualInterfaceError {
    fn description(&self) -> &str {
        match *self {
            CreatePublicVirtualInterfaceError::DirectConnectClient(ref cause) => cause,
            CreatePublicVirtualInterfaceError::DirectConnectServer(ref cause) => cause,
            CreatePublicVirtualInterfaceError::Validation(ref cause) => cause,
            CreatePublicVirtualInterfaceError::Credentials(ref err) => err.description(),
            CreatePublicVirtualInterfaceError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            CreatePublicVirtualInterfaceError::ParseError(ref cause) => cause,
            CreatePublicVirtualInterfaceError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by DeleteBGPPeer
#[derive(Debug, PartialEq)]
pub enum DeleteBGPPeerError {
    /// <p>One or more parameters are not valid.</p>
    DirectConnectClient(String),
    /// <p>A server-side error occurred.</p>
    DirectConnectServer(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl DeleteBGPPeerError {
    pub fn from_response(res: BufferedHttpResponse) -> DeleteBGPPeerError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "DirectConnectClientException" => {
                    return DeleteBGPPeerError::DirectConnectClient(String::from(error_message));
                }
                "DirectConnectServerException" => {
                    return DeleteBGPPeerError::DirectConnectServer(String::from(error_message));
                }
                "ValidationException" => {
                    return DeleteBGPPeerError::Validation(error_message.to_string());
                }
                _ => {}
            }
        }
        return DeleteBGPPeerError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for DeleteBGPPeerError {
    fn from(err: serde_json::error::Error) -> DeleteBGPPeerError {
        DeleteBGPPeerError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for DeleteBGPPeerError {
    fn from(err: CredentialsError) -> DeleteBGPPeerError {
        DeleteBGPPeerError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DeleteBGPPeerError {
    fn from(err: HttpDispatchError) -> DeleteBGPPeerError {
        DeleteBGPPeerError::HttpDispatch(err)
    }
}
impl From<io::Error> for DeleteBGPPeerError {
    fn from(err: io::Error) -> DeleteBGPPeerError {
        DeleteBGPPeerError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DeleteBGPPeerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteBGPPeerError {
    fn description(&self) -> &str {
        match *self {
            DeleteBGPPeerError::DirectConnectClient(ref cause) => cause,
            DeleteBGPPeerError::DirectConnectServer(ref cause) => cause,
            DeleteBGPPeerError::Validation(ref cause) => cause,
            DeleteBGPPeerError::Credentials(ref err) => err.description(),
            DeleteBGPPeerError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            DeleteBGPPeerError::ParseError(ref cause) => cause,
            DeleteBGPPeerError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by DeleteConnection
#[derive(Debug, PartialEq)]
pub enum DeleteConnectionError {
    /// <p>One or more parameters are not valid.</p>
    DirectConnectClient(String),
    /// <p>A server-side error occurred.</p>
    DirectConnectServer(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl DeleteConnectionError {
    pub fn from_response(res: BufferedHttpResponse) -> DeleteConnectionError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "DirectConnectClientException" => {
                    return DeleteConnectionError::DirectConnectClient(String::from(error_message));
                }
                "DirectConnectServerException" => {
                    return DeleteConnectionError::DirectConnectServer(String::from(error_message));
                }
                "ValidationException" => {
                    return DeleteConnectionError::Validation(error_message.to_string());
                }
                _ => {}
            }
        }
        return DeleteConnectionError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for DeleteConnectionError {
    fn from(err: serde_json::error::Error) -> DeleteConnectionError {
        DeleteConnectionError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for DeleteConnectionError {
    fn from(err: CredentialsError) -> DeleteConnectionError {
        DeleteConnectionError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DeleteConnectionError {
    fn from(err: HttpDispatchError) -> DeleteConnectionError {
        DeleteConnectionError::HttpDispatch(err)
    }
}
impl From<io::Error> for DeleteConnectionError {
    fn from(err: io::Error) -> DeleteConnectionError {
        DeleteConnectionError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DeleteConnectionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteConnectionError {
    fn description(&self) -> &str {
        match *self {
            DeleteConnectionError::DirectConnectClient(ref cause) => cause,
            DeleteConnectionError::DirectConnectServer(ref cause) => cause,
            DeleteConnectionError::Validation(ref cause) => cause,
            DeleteConnectionError::Credentials(ref err) => err.description(),
            DeleteConnectionError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            DeleteConnectionError::ParseError(ref cause) => cause,
            DeleteConnectionError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by DeleteDirectConnectGateway
#[derive(Debug, PartialEq)]
pub enum DeleteDirectConnectGatewayError {
    /// <p>One or more parameters are not valid.</p>
    DirectConnectClient(String),
    /// <p>A server-side error occurred.</p>
    DirectConnectServer(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl DeleteDirectConnectGatewayError {
    pub fn from_response(res: BufferedHttpResponse) -> DeleteDirectConnectGatewayError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "DirectConnectClientException" => {
                    return DeleteDirectConnectGatewayError::DirectConnectClient(String::from(
                        error_message,
                    ));
                }
                "DirectConnectServerException" => {
                    return DeleteDirectConnectGatewayError::DirectConnectServer(String::from(
                        error_message,
                    ));
                }
                "ValidationException" => {
                    return DeleteDirectConnectGatewayError::Validation(error_message.to_string());
                }
                _ => {}
            }
        }
        return DeleteDirectConnectGatewayError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for DeleteDirectConnectGatewayError {
    fn from(err: serde_json::error::Error) -> DeleteDirectConnectGatewayError {
        DeleteDirectConnectGatewayError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for DeleteDirectConnectGatewayError {
    fn from(err: CredentialsError) -> DeleteDirectConnectGatewayError {
        DeleteDirectConnectGatewayError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DeleteDirectConnectGatewayError {
    fn from(err: HttpDispatchError) -> DeleteDirectConnectGatewayError {
        DeleteDirectConnectGatewayError::HttpDispatch(err)
    }
}
impl From<io::Error> for DeleteDirectConnectGatewayError {
    fn from(err: io::Error) -> DeleteDirectConnectGatewayError {
        DeleteDirectConnectGatewayError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DeleteDirectConnectGatewayError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteDirectConnectGatewayError {
    fn description(&self) -> &str {
        match *self {
            DeleteDirectConnectGatewayError::DirectConnectClient(ref cause) => cause,
            DeleteDirectConnectGatewayError::DirectConnectServer(ref cause) => cause,
            DeleteDirectConnectGatewayError::Validation(ref cause) => cause,
            DeleteDirectConnectGatewayError::Credentials(ref err) => err.description(),
            DeleteDirectConnectGatewayError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DeleteDirectConnectGatewayError::ParseError(ref cause) => cause,
            DeleteDirectConnectGatewayError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by DeleteDirectConnectGatewayAssociation
#[derive(Debug, PartialEq)]
pub enum DeleteDirectConnectGatewayAssociationError {
    /// <p>One or more parameters are not valid.</p>
    DirectConnectClient(String),
    /// <p>A server-side error occurred.</p>
    DirectConnectServer(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl DeleteDirectConnectGatewayAssociationError {
    pub fn from_response(res: BufferedHttpResponse) -> DeleteDirectConnectGatewayAssociationError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "DirectConnectClientException" => {
                    return DeleteDirectConnectGatewayAssociationError::DirectConnectClient(
                        String::from(error_message),
                    );
                }
                "DirectConnectServerException" => {
                    return DeleteDirectConnectGatewayAssociationError::DirectConnectServer(
                        String::from(error_message),
                    );
                }
                "ValidationException" => {
                    return DeleteDirectConnectGatewayAssociationError::Validation(
                        error_message.to_string(),
                    );
                }
                _ => {}
            }
        }
        return DeleteDirectConnectGatewayAssociationError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for DeleteDirectConnectGatewayAssociationError {
    fn from(err: serde_json::error::Error) -> DeleteDirectConnectGatewayAssociationError {
        DeleteDirectConnectGatewayAssociationError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for DeleteDirectConnectGatewayAssociationError {
    fn from(err: CredentialsError) -> DeleteDirectConnectGatewayAssociationError {
        DeleteDirectConnectGatewayAssociationError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DeleteDirectConnectGatewayAssociationError {
    fn from(err: HttpDispatchError) -> DeleteDirectConnectGatewayAssociationError {
        DeleteDirectConnectGatewayAssociationError::HttpDispatch(err)
    }
}
impl From<io::Error> for DeleteDirectConnectGatewayAssociationError {
    fn from(err: io::Error) -> DeleteDirectConnectGatewayAssociationError {
        DeleteDirectConnectGatewayAssociationError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DeleteDirectConnectGatewayAssociationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteDirectConnectGatewayAssociationError {
    fn description(&self) -> &str {
        match *self {
            DeleteDirectConnectGatewayAssociationError::DirectConnectClient(ref cause) => cause,
            DeleteDirectConnectGatewayAssociationError::DirectConnectServer(ref cause) => cause,
            DeleteDirectConnectGatewayAssociationError::Validation(ref cause) => cause,
            DeleteDirectConnectGatewayAssociationError::Credentials(ref err) => err.description(),
            DeleteDirectConnectGatewayAssociationError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DeleteDirectConnectGatewayAssociationError::ParseError(ref cause) => cause,
            DeleteDirectConnectGatewayAssociationError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by DeleteInterconnect
#[derive(Debug, PartialEq)]
pub enum DeleteInterconnectError {
    /// <p>One or more parameters are not valid.</p>
    DirectConnectClient(String),
    /// <p>A server-side error occurred.</p>
    DirectConnectServer(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl DeleteInterconnectError {
    pub fn from_response(res: BufferedHttpResponse) -> DeleteInterconnectError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "DirectConnectClientException" => {
                    return DeleteInterconnectError::DirectConnectClient(String::from(error_message));
                }
                "DirectConnectServerException" => {
                    return DeleteInterconnectError::DirectConnectServer(String::from(error_message));
                }
                "ValidationException" => {
                    return DeleteInterconnectError::Validation(error_message.to_string());
                }
                _ => {}
            }
        }
        return DeleteInterconnectError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for DeleteInterconnectError {
    fn from(err: serde_json::error::Error) -> DeleteInterconnectError {
        DeleteInterconnectError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for DeleteInterconnectError {
    fn from(err: CredentialsError) -> DeleteInterconnectError {
        DeleteInterconnectError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DeleteInterconnectError {
    fn from(err: HttpDispatchError) -> DeleteInterconnectError {
        DeleteInterconnectError::HttpDispatch(err)
    }
}
impl From<io::Error> for DeleteInterconnectError {
    fn from(err: io::Error) -> DeleteInterconnectError {
        DeleteInterconnectError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DeleteInterconnectError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteInterconnectError {
    fn description(&self) -> &str {
        match *self {
            DeleteInterconnectError::DirectConnectClient(ref cause) => cause,
            DeleteInterconnectError::DirectConnectServer(ref cause) => cause,
            DeleteInterconnectError::Validation(ref cause) => cause,
            DeleteInterconnectError::Credentials(ref err) => err.description(),
            DeleteInterconnectError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DeleteInterconnectError::ParseError(ref cause) => cause,
            DeleteInterconnectError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by DeleteLag
#[derive(Debug, PartialEq)]
pub enum DeleteLagError {
    /// <p>One or more parameters are not valid.</p>
    DirectConnectClient(String),
    /// <p>A server-side error occurred.</p>
    DirectConnectServer(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl DeleteLagError {
    pub fn from_response(res: BufferedHttpResponse) -> DeleteLagError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "DirectConnectClientException" => {
                    return DeleteLagError::DirectConnectClient(String::from(error_message));
                }
                "DirectConnectServerException" => {
                    return DeleteLagError::DirectConnectServer(String::from(error_message));
                }
                "ValidationException" => {
                    return DeleteLagError::Validation(error_message.to_string());
                }
                _ => {}
            }
        }
        return DeleteLagError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for DeleteLagError {
    fn from(err: serde_json::error::Error) -> DeleteLagError {
        DeleteLagError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for DeleteLagError {
    fn from(err: CredentialsError) -> DeleteLagError {
        DeleteLagError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DeleteLagError {
    fn from(err: HttpDispatchError) -> DeleteLagError {
        DeleteLagError::HttpDispatch(err)
    }
}
impl From<io::Error> for DeleteLagError {
    fn from(err: io::Error) -> DeleteLagError {
        DeleteLagError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DeleteLagError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteLagError {
    fn description(&self) -> &str {
        match *self {
            DeleteLagError::DirectConnectClient(ref cause) => cause,
            DeleteLagError::DirectConnectServer(ref cause) => cause,
            DeleteLagError::Validation(ref cause) => cause,
            DeleteLagError::Credentials(ref err) => err.description(),
            DeleteLagError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            DeleteLagError::ParseError(ref cause) => cause,
            DeleteLagError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by DeleteVirtualInterface
#[derive(Debug, PartialEq)]
pub enum DeleteVirtualInterfaceError {
    /// <p>One or more parameters are not valid.</p>
    DirectConnectClient(String),
    /// <p>A server-side error occurred.</p>
    DirectConnectServer(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl DeleteVirtualInterfaceError {
    pub fn from_response(res: BufferedHttpResponse) -> DeleteVirtualInterfaceError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "DirectConnectClientException" => {
                    return DeleteVirtualInterfaceError::DirectConnectClient(String::from(
                        error_message,
                    ));
                }
                "DirectConnectServerException" => {
                    return DeleteVirtualInterfaceError::DirectConnectServer(String::from(
                        error_message,
                    ));
                }
                "ValidationException" => {
                    return DeleteVirtualInterfaceError::Validation(error_message.to_string());
                }
                _ => {}
            }
        }
        return DeleteVirtualInterfaceError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for DeleteVirtualInterfaceError {
    fn from(err: serde_json::error::Error) -> DeleteVirtualInterfaceError {
        DeleteVirtualInterfaceError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for DeleteVirtualInterfaceError {
    fn from(err: CredentialsError) -> DeleteVirtualInterfaceError {
        DeleteVirtualInterfaceError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DeleteVirtualInterfaceError {
    fn from(err: HttpDispatchError) -> DeleteVirtualInterfaceError {
        DeleteVirtualInterfaceError::HttpDispatch(err)
    }
}
impl From<io::Error> for DeleteVirtualInterfaceError {
    fn from(err: io::Error) -> DeleteVirtualInterfaceError {
        DeleteVirtualInterfaceError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DeleteVirtualInterfaceError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteVirtualInterfaceError {
    fn description(&self) -> &str {
        match *self {
            DeleteVirtualInterfaceError::DirectConnectClient(ref cause) => cause,
            DeleteVirtualInterfaceError::DirectConnectServer(ref cause) => cause,
            DeleteVirtualInterfaceError::Validation(ref cause) => cause,
            DeleteVirtualInterfaceError::Credentials(ref err) => err.description(),
            DeleteVirtualInterfaceError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DeleteVirtualInterfaceError::ParseError(ref cause) => cause,
            DeleteVirtualInterfaceError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by DescribeConnectionLoa
#[derive(Debug, PartialEq)]
pub enum DescribeConnectionLoaError {
    /// <p>One or more parameters are not valid.</p>
    DirectConnectClient(String),
    /// <p>A server-side error occurred.</p>
    DirectConnectServer(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl DescribeConnectionLoaError {
    pub fn from_response(res: BufferedHttpResponse) -> DescribeConnectionLoaError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "DirectConnectClientException" => {
                    return DescribeConnectionLoaError::DirectConnectClient(String::from(
                        error_message,
                    ));
                }
                "DirectConnectServerException" => {
                    return DescribeConnectionLoaError::DirectConnectServer(String::from(
                        error_message,
                    ));
                }
                "ValidationException" => {
                    return DescribeConnectionLoaError::Validation(error_message.to_string());
                }
                _ => {}
            }
        }
        return DescribeConnectionLoaError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for DescribeConnectionLoaError {
    fn from(err: serde_json::error::Error) -> DescribeConnectionLoaError {
        DescribeConnectionLoaError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for DescribeConnectionLoaError {
    fn from(err: CredentialsError) -> DescribeConnectionLoaError {
        DescribeConnectionLoaError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DescribeConnectionLoaError {
    fn from(err: HttpDispatchError) -> DescribeConnectionLoaError {
        DescribeConnectionLoaError::HttpDispatch(err)
    }
}
impl From<io::Error> for DescribeConnectionLoaError {
    fn from(err: io::Error) -> DescribeConnectionLoaError {
        DescribeConnectionLoaError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DescribeConnectionLoaError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeConnectionLoaError {
    fn description(&self) -> &str {
        match *self {
            DescribeConnectionLoaError::DirectConnectClient(ref cause) => cause,
            DescribeConnectionLoaError::DirectConnectServer(ref cause) => cause,
            DescribeConnectionLoaError::Validation(ref cause) => cause,
            DescribeConnectionLoaError::Credentials(ref err) => err.description(),
            DescribeConnectionLoaError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DescribeConnectionLoaError::ParseError(ref cause) => cause,
            DescribeConnectionLoaError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by DescribeConnections
#[derive(Debug, PartialEq)]
pub enum DescribeConnectionsError {
    /// <p>One or more parameters are not valid.</p>
    DirectConnectClient(String),
    /// <p>A server-side error occurred.</p>
    DirectConnectServer(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl DescribeConnectionsError {
    pub fn from_response(res: BufferedHttpResponse) -> DescribeConnectionsError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "DirectConnectClientException" => {
                    return DescribeConnectionsError::DirectConnectClient(String::from(
                        error_message,
                    ));
                }
                "DirectConnectServerException" => {
                    return DescribeConnectionsError::DirectConnectServer(String::from(
                        error_message,
                    ));
                }
                "ValidationException" => {
                    return DescribeConnectionsError::Validation(error_message.to_string());
                }
                _ => {}
            }
        }
        return DescribeConnectionsError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for DescribeConnectionsError {
    fn from(err: serde_json::error::Error) -> DescribeConnectionsError {
        DescribeConnectionsError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for DescribeConnectionsError {
    fn from(err: CredentialsError) -> DescribeConnectionsError {
        DescribeConnectionsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DescribeConnectionsError {
    fn from(err: HttpDispatchError) -> DescribeConnectionsError {
        DescribeConnectionsError::HttpDispatch(err)
    }
}
impl From<io::Error> for DescribeConnectionsError {
    fn from(err: io::Error) -> DescribeConnectionsError {
        DescribeConnectionsError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DescribeConnectionsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeConnectionsError {
    fn description(&self) -> &str {
        match *self {
            DescribeConnectionsError::DirectConnectClient(ref cause) => cause,
            DescribeConnectionsError::DirectConnectServer(ref cause) => cause,
            DescribeConnectionsError::Validation(ref cause) => cause,
            DescribeConnectionsError::Credentials(ref err) => err.description(),
            DescribeConnectionsError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DescribeConnectionsError::ParseError(ref cause) => cause,
            DescribeConnectionsError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by DescribeConnectionsOnInterconnect
#[derive(Debug, PartialEq)]
pub enum DescribeConnectionsOnInterconnectError {
    /// <p>One or more parameters are not valid.</p>
    DirectConnectClient(String),
    /// <p>A server-side error occurred.</p>
    DirectConnectServer(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl DescribeConnectionsOnInterconnectError {
    pub fn from_response(res: BufferedHttpResponse) -> DescribeConnectionsOnInterconnectError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "DirectConnectClientException" => {
                    return DescribeConnectionsOnInterconnectError::DirectConnectClient(
                        String::from(error_message),
                    );
                }
                "DirectConnectServerException" => {
                    return DescribeConnectionsOnInterconnectError::DirectConnectServer(
                        String::from(error_message),
                    );
                }
                "ValidationException" => {
                    return DescribeConnectionsOnInterconnectError::Validation(
                        error_message.to_string(),
                    );
                }
                _ => {}
            }
        }
        return DescribeConnectionsOnInterconnectError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for DescribeConnectionsOnInterconnectError {
    fn from(err: serde_json::error::Error) -> DescribeConnectionsOnInterconnectError {
        DescribeConnectionsOnInterconnectError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for DescribeConnectionsOnInterconnectError {
    fn from(err: CredentialsError) -> DescribeConnectionsOnInterconnectError {
        DescribeConnectionsOnInterconnectError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DescribeConnectionsOnInterconnectError {
    fn from(err: HttpDispatchError) -> DescribeConnectionsOnInterconnectError {
        DescribeConnectionsOnInterconnectError::HttpDispatch(err)
    }
}
impl From<io::Error> for DescribeConnectionsOnInterconnectError {
    fn from(err: io::Error) -> DescribeConnectionsOnInterconnectError {
        DescribeConnectionsOnInterconnectError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DescribeConnectionsOnInterconnectError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeConnectionsOnInterconnectError {
    fn description(&self) -> &str {
        match *self {
            DescribeConnectionsOnInterconnectError::DirectConnectClient(ref cause) => cause,
            DescribeConnectionsOnInterconnectError::DirectConnectServer(ref cause) => cause,
            DescribeConnectionsOnInterconnectError::Validation(ref cause) => cause,
            DescribeConnectionsOnInterconnectError::Credentials(ref err) => err.description(),
            DescribeConnectionsOnInterconnectError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DescribeConnectionsOnInterconnectError::ParseError(ref cause) => cause,
            DescribeConnectionsOnInterconnectError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by DescribeDirectConnectGatewayAssociations
#[derive(Debug, PartialEq)]
pub enum DescribeDirectConnectGatewayAssociationsError {
    /// <p>One or more parameters are not valid.</p>
    DirectConnectClient(String),
    /// <p>A server-side error occurred.</p>
    DirectConnectServer(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl DescribeDirectConnectGatewayAssociationsError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> DescribeDirectConnectGatewayAssociationsError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "DirectConnectClientException" => {
                    return DescribeDirectConnectGatewayAssociationsError::DirectConnectClient(
                        String::from(error_message),
                    );
                }
                "DirectConnectServerException" => {
                    return DescribeDirectConnectGatewayAssociationsError::DirectConnectServer(
                        String::from(error_message),
                    );
                }
                "ValidationException" => {
                    return DescribeDirectConnectGatewayAssociationsError::Validation(
                        error_message.to_string(),
                    );
                }
                _ => {}
            }
        }
        return DescribeDirectConnectGatewayAssociationsError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for DescribeDirectConnectGatewayAssociationsError {
    fn from(err: serde_json::error::Error) -> DescribeDirectConnectGatewayAssociationsError {
        DescribeDirectConnectGatewayAssociationsError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for DescribeDirectConnectGatewayAssociationsError {
    fn from(err: CredentialsError) -> DescribeDirectConnectGatewayAssociationsError {
        DescribeDirectConnectGatewayAssociationsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DescribeDirectConnectGatewayAssociationsError {
    fn from(err: HttpDispatchError) -> DescribeDirectConnectGatewayAssociationsError {
        DescribeDirectConnectGatewayAssociationsError::HttpDispatch(err)
    }
}
impl From<io::Error> for DescribeDirectConnectGatewayAssociationsError {
    fn from(err: io::Error) -> DescribeDirectConnectGatewayAssociationsError {
        DescribeDirectConnectGatewayAssociationsError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DescribeDirectConnectGatewayAssociationsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeDirectConnectGatewayAssociationsError {
    fn description(&self) -> &str {
        match *self {
            DescribeDirectConnectGatewayAssociationsError::DirectConnectClient(ref cause) => cause,
            DescribeDirectConnectGatewayAssociationsError::DirectConnectServer(ref cause) => cause,
            DescribeDirectConnectGatewayAssociationsError::Validation(ref cause) => cause,
            DescribeDirectConnectGatewayAssociationsError::Credentials(ref err) => {
                err.description()
            }
            DescribeDirectConnectGatewayAssociationsError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DescribeDirectConnectGatewayAssociationsError::ParseError(ref cause) => cause,
            DescribeDirectConnectGatewayAssociationsError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by DescribeDirectConnectGatewayAttachments
#[derive(Debug, PartialEq)]
pub enum DescribeDirectConnectGatewayAttachmentsError {
    /// <p>One or more parameters are not valid.</p>
    DirectConnectClient(String),
    /// <p>A server-side error occurred.</p>
    DirectConnectServer(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl DescribeDirectConnectGatewayAttachmentsError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> DescribeDirectConnectGatewayAttachmentsError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "DirectConnectClientException" => {
                    return DescribeDirectConnectGatewayAttachmentsError::DirectConnectClient(
                        String::from(error_message),
                    );
                }
                "DirectConnectServerException" => {
                    return DescribeDirectConnectGatewayAttachmentsError::DirectConnectServer(
                        String::from(error_message),
                    );
                }
                "ValidationException" => {
                    return DescribeDirectConnectGatewayAttachmentsError::Validation(
                        error_message.to_string(),
                    );
                }
                _ => {}
            }
        }
        return DescribeDirectConnectGatewayAttachmentsError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for DescribeDirectConnectGatewayAttachmentsError {
    fn from(err: serde_json::error::Error) -> DescribeDirectConnectGatewayAttachmentsError {
        DescribeDirectConnectGatewayAttachmentsError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for DescribeDirectConnectGatewayAttachmentsError {
    fn from(err: CredentialsError) -> DescribeDirectConnectGatewayAttachmentsError {
        DescribeDirectConnectGatewayAttachmentsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DescribeDirectConnectGatewayAttachmentsError {
    fn from(err: HttpDispatchError) -> DescribeDirectConnectGatewayAttachmentsError {
        DescribeDirectConnectGatewayAttachmentsError::HttpDispatch(err)
    }
}
impl From<io::Error> for DescribeDirectConnectGatewayAttachmentsError {
    fn from(err: io::Error) -> DescribeDirectConnectGatewayAttachmentsError {
        DescribeDirectConnectGatewayAttachmentsError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DescribeDirectConnectGatewayAttachmentsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeDirectConnectGatewayAttachmentsError {
    fn description(&self) -> &str {
        match *self {
            DescribeDirectConnectGatewayAttachmentsError::DirectConnectClient(ref cause) => cause,
            DescribeDirectConnectGatewayAttachmentsError::DirectConnectServer(ref cause) => cause,
            DescribeDirectConnectGatewayAttachmentsError::Validation(ref cause) => cause,
            DescribeDirectConnectGatewayAttachmentsError::Credentials(ref err) => err.description(),
            DescribeDirectConnectGatewayAttachmentsError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DescribeDirectConnectGatewayAttachmentsError::ParseError(ref cause) => cause,
            DescribeDirectConnectGatewayAttachmentsError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by DescribeDirectConnectGateways
#[derive(Debug, PartialEq)]
pub enum DescribeDirectConnectGatewaysError {
    /// <p>One or more parameters are not valid.</p>
    DirectConnectClient(String),
    /// <p>A server-side error occurred.</p>
    DirectConnectServer(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl DescribeDirectConnectGatewaysError {
    pub fn from_response(res: BufferedHttpResponse) -> DescribeDirectConnectGatewaysError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "DirectConnectClientException" => {
                    return DescribeDirectConnectGatewaysError::DirectConnectClient(String::from(
                        error_message,
                    ));
                }
                "DirectConnectServerException" => {
                    return DescribeDirectConnectGatewaysError::DirectConnectServer(String::from(
                        error_message,
                    ));
                }
                "ValidationException" => {
                    return DescribeDirectConnectGatewaysError::Validation(error_message.to_string());
                }
                _ => {}
            }
        }
        return DescribeDirectConnectGatewaysError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for DescribeDirectConnectGatewaysError {
    fn from(err: serde_json::error::Error) -> DescribeDirectConnectGatewaysError {
        DescribeDirectConnectGatewaysError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for DescribeDirectConnectGatewaysError {
    fn from(err: CredentialsError) -> DescribeDirectConnectGatewaysError {
        DescribeDirectConnectGatewaysError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DescribeDirectConnectGatewaysError {
    fn from(err: HttpDispatchError) -> DescribeDirectConnectGatewaysError {
        DescribeDirectConnectGatewaysError::HttpDispatch(err)
    }
}
impl From<io::Error> for DescribeDirectConnectGatewaysError {
    fn from(err: io::Error) -> DescribeDirectConnectGatewaysError {
        DescribeDirectConnectGatewaysError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DescribeDirectConnectGatewaysError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeDirectConnectGatewaysError {
    fn description(&self) -> &str {
        match *self {
            DescribeDirectConnectGatewaysError::DirectConnectClient(ref cause) => cause,
            DescribeDirectConnectGatewaysError::DirectConnectServer(ref cause) => cause,
            DescribeDirectConnectGatewaysError::Validation(ref cause) => cause,
            DescribeDirectConnectGatewaysError::Credentials(ref err) => err.description(),
            DescribeDirectConnectGatewaysError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DescribeDirectConnectGatewaysError::ParseError(ref cause) => cause,
            DescribeDirectConnectGatewaysError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by DescribeHostedConnections
#[derive(Debug, PartialEq)]
pub enum DescribeHostedConnectionsError {
    /// <p>One or more parameters are not valid.</p>
    DirectConnectClient(String),
    /// <p>A server-side error occurred.</p>
    DirectConnectServer(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl DescribeHostedConnectionsError {
    pub fn from_response(res: BufferedHttpResponse) -> DescribeHostedConnectionsError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "DirectConnectClientException" => {
                    return DescribeHostedConnectionsError::DirectConnectClient(String::from(
                        error_message,
                    ));
                }
                "DirectConnectServerException" => {
                    return DescribeHostedConnectionsError::DirectConnectServer(String::from(
                        error_message,
                    ));
                }
                "ValidationException" => {
                    return DescribeHostedConnectionsError::Validation(error_message.to_string());
                }
                _ => {}
            }
        }
        return DescribeHostedConnectionsError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for DescribeHostedConnectionsError {
    fn from(err: serde_json::error::Error) -> DescribeHostedConnectionsError {
        DescribeHostedConnectionsError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for DescribeHostedConnectionsError {
    fn from(err: CredentialsError) -> DescribeHostedConnectionsError {
        DescribeHostedConnectionsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DescribeHostedConnectionsError {
    fn from(err: HttpDispatchError) -> DescribeHostedConnectionsError {
        DescribeHostedConnectionsError::HttpDispatch(err)
    }
}
impl From<io::Error> for DescribeHostedConnectionsError {
    fn from(err: io::Error) -> DescribeHostedConnectionsError {
        DescribeHostedConnectionsError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DescribeHostedConnectionsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeHostedConnectionsError {
    fn description(&self) -> &str {
        match *self {
            DescribeHostedConnectionsError::DirectConnectClient(ref cause) => cause,
            DescribeHostedConnectionsError::DirectConnectServer(ref cause) => cause,
            DescribeHostedConnectionsError::Validation(ref cause) => cause,
            DescribeHostedConnectionsError::Credentials(ref err) => err.description(),
            DescribeHostedConnectionsError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DescribeHostedConnectionsError::ParseError(ref cause) => cause,
            DescribeHostedConnectionsError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by DescribeInterconnectLoa
#[derive(Debug, PartialEq)]
pub enum DescribeInterconnectLoaError {
    /// <p>One or more parameters are not valid.</p>
    DirectConnectClient(String),
    /// <p>A server-side error occurred.</p>
    DirectConnectServer(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl DescribeInterconnectLoaError {
    pub fn from_response(res: BufferedHttpResponse) -> DescribeInterconnectLoaError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "DirectConnectClientException" => {
                    return DescribeInterconnectLoaError::DirectConnectClient(String::from(
                        error_message,
                    ));
                }
                "DirectConnectServerException" => {
                    return DescribeInterconnectLoaError::DirectConnectServer(String::from(
                        error_message,
                    ));
                }
                "ValidationException" => {
                    return DescribeInterconnectLoaError::Validation(error_message.to_string());
                }
                _ => {}
            }
        }
        return DescribeInterconnectLoaError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for DescribeInterconnectLoaError {
    fn from(err: serde_json::error::Error) -> DescribeInterconnectLoaError {
        DescribeInterconnectLoaError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for DescribeInterconnectLoaError {
    fn from(err: CredentialsError) -> DescribeInterconnectLoaError {
        DescribeInterconnectLoaError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DescribeInterconnectLoaError {
    fn from(err: HttpDispatchError) -> DescribeInterconnectLoaError {
        DescribeInterconnectLoaError::HttpDispatch(err)
    }
}
impl From<io::Error> for DescribeInterconnectLoaError {
    fn from(err: io::Error) -> DescribeInterconnectLoaError {
        DescribeInterconnectLoaError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DescribeInterconnectLoaError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeInterconnectLoaError {
    fn description(&self) -> &str {
        match *self {
            DescribeInterconnectLoaError::DirectConnectClient(ref cause) => cause,
            DescribeInterconnectLoaError::DirectConnectServer(ref cause) => cause,
            DescribeInterconnectLoaError::Validation(ref cause) => cause,
            DescribeInterconnectLoaError::Credentials(ref err) => err.description(),
            DescribeInterconnectLoaError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DescribeInterconnectLoaError::ParseError(ref cause) => cause,
            DescribeInterconnectLoaError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by DescribeInterconnects
#[derive(Debug, PartialEq)]
pub enum DescribeInterconnectsError {
    /// <p>One or more parameters are not valid.</p>
    DirectConnectClient(String),
    /// <p>A server-side error occurred.</p>
    DirectConnectServer(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl DescribeInterconnectsError {
    pub fn from_response(res: BufferedHttpResponse) -> DescribeInterconnectsError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "DirectConnectClientException" => {
                    return DescribeInterconnectsError::DirectConnectClient(String::from(
                        error_message,
                    ));
                }
                "DirectConnectServerException" => {
                    return DescribeInterconnectsError::DirectConnectServer(String::from(
                        error_message,
                    ));
                }
                "ValidationException" => {
                    return DescribeInterconnectsError::Validation(error_message.to_string());
                }
                _ => {}
            }
        }
        return DescribeInterconnectsError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for DescribeInterconnectsError {
    fn from(err: serde_json::error::Error) -> DescribeInterconnectsError {
        DescribeInterconnectsError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for DescribeInterconnectsError {
    fn from(err: CredentialsError) -> DescribeInterconnectsError {
        DescribeInterconnectsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DescribeInterconnectsError {
    fn from(err: HttpDispatchError) -> DescribeInterconnectsError {
        DescribeInterconnectsError::HttpDispatch(err)
    }
}
impl From<io::Error> for DescribeInterconnectsError {
    fn from(err: io::Error) -> DescribeInterconnectsError {
        DescribeInterconnectsError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DescribeInterconnectsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeInterconnectsError {
    fn description(&self) -> &str {
        match *self {
            DescribeInterconnectsError::DirectConnectClient(ref cause) => cause,
            DescribeInterconnectsError::DirectConnectServer(ref cause) => cause,
            DescribeInterconnectsError::Validation(ref cause) => cause,
            DescribeInterconnectsError::Credentials(ref err) => err.description(),
            DescribeInterconnectsError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DescribeInterconnectsError::ParseError(ref cause) => cause,
            DescribeInterconnectsError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by DescribeLags
#[derive(Debug, PartialEq)]
pub enum DescribeLagsError {
    /// <p>One or more parameters are not valid.</p>
    DirectConnectClient(String),
    /// <p>A server-side error occurred.</p>
    DirectConnectServer(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl DescribeLagsError {
    pub fn from_response(res: BufferedHttpResponse) -> DescribeLagsError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "DirectConnectClientException" => {
                    return DescribeLagsError::DirectConnectClient(String::from(error_message));
                }
                "DirectConnectServerException" => {
                    return DescribeLagsError::DirectConnectServer(String::from(error_message));
                }
                "ValidationException" => {
                    return DescribeLagsError::Validation(error_message.to_string());
                }
                _ => {}
            }
        }
        return DescribeLagsError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for DescribeLagsError {
    fn from(err: serde_json::error::Error) -> DescribeLagsError {
        DescribeLagsError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for DescribeLagsError {
    fn from(err: CredentialsError) -> DescribeLagsError {
        DescribeLagsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DescribeLagsError {
    fn from(err: HttpDispatchError) -> DescribeLagsError {
        DescribeLagsError::HttpDispatch(err)
    }
}
impl From<io::Error> for DescribeLagsError {
    fn from(err: io::Error) -> DescribeLagsError {
        DescribeLagsError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DescribeLagsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeLagsError {
    fn description(&self) -> &str {
        match *self {
            DescribeLagsError::DirectConnectClient(ref cause) => cause,
            DescribeLagsError::DirectConnectServer(ref cause) => cause,
            DescribeLagsError::Validation(ref cause) => cause,
            DescribeLagsError::Credentials(ref err) => err.description(),
            DescribeLagsError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            DescribeLagsError::ParseError(ref cause) => cause,
            DescribeLagsError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by DescribeLoa
#[derive(Debug, PartialEq)]
pub enum DescribeLoaError {
    /// <p>One or more parameters are not valid.</p>
    DirectConnectClient(String),
    /// <p>A server-side error occurred.</p>
    DirectConnectServer(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl DescribeLoaError {
    pub fn from_response(res: BufferedHttpResponse) -> DescribeLoaError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "DirectConnectClientException" => {
                    return DescribeLoaError::DirectConnectClient(String::from(error_message));
                }
                "DirectConnectServerException" => {
                    return DescribeLoaError::DirectConnectServer(String::from(error_message));
                }
                "ValidationException" => {
                    return DescribeLoaError::Validation(error_message.to_string());
                }
                _ => {}
            }
        }
        return DescribeLoaError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for DescribeLoaError {
    fn from(err: serde_json::error::Error) -> DescribeLoaError {
        DescribeLoaError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for DescribeLoaError {
    fn from(err: CredentialsError) -> DescribeLoaError {
        DescribeLoaError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DescribeLoaError {
    fn from(err: HttpDispatchError) -> DescribeLoaError {
        DescribeLoaError::HttpDispatch(err)
    }
}
impl From<io::Error> for DescribeLoaError {
    fn from(err: io::Error) -> DescribeLoaError {
        DescribeLoaError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DescribeLoaError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeLoaError {
    fn description(&self) -> &str {
        match *self {
            DescribeLoaError::DirectConnectClient(ref cause) => cause,
            DescribeLoaError::DirectConnectServer(ref cause) => cause,
            DescribeLoaError::Validation(ref cause) => cause,
            DescribeLoaError::Credentials(ref err) => err.description(),
            DescribeLoaError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            DescribeLoaError::ParseError(ref cause) => cause,
            DescribeLoaError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by DescribeLocations
#[derive(Debug, PartialEq)]
pub enum DescribeLocationsError {
    /// <p>One or more parameters are not valid.</p>
    DirectConnectClient(String),
    /// <p>A server-side error occurred.</p>
    DirectConnectServer(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl DescribeLocationsError {
    pub fn from_response(res: BufferedHttpResponse) -> DescribeLocationsError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "DirectConnectClientException" => {
                    return DescribeLocationsError::DirectConnectClient(String::from(error_message));
                }
                "DirectConnectServerException" => {
                    return DescribeLocationsError::DirectConnectServer(String::from(error_message));
                }
                "ValidationException" => {
                    return DescribeLocationsError::Validation(error_message.to_string());
                }
                _ => {}
            }
        }
        return DescribeLocationsError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for DescribeLocationsError {
    fn from(err: serde_json::error::Error) -> DescribeLocationsError {
        DescribeLocationsError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for DescribeLocationsError {
    fn from(err: CredentialsError) -> DescribeLocationsError {
        DescribeLocationsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DescribeLocationsError {
    fn from(err: HttpDispatchError) -> DescribeLocationsError {
        DescribeLocationsError::HttpDispatch(err)
    }
}
impl From<io::Error> for DescribeLocationsError {
    fn from(err: io::Error) -> DescribeLocationsError {
        DescribeLocationsError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DescribeLocationsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeLocationsError {
    fn description(&self) -> &str {
        match *self {
            DescribeLocationsError::DirectConnectClient(ref cause) => cause,
            DescribeLocationsError::DirectConnectServer(ref cause) => cause,
            DescribeLocationsError::Validation(ref cause) => cause,
            DescribeLocationsError::Credentials(ref err) => err.description(),
            DescribeLocationsError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DescribeLocationsError::ParseError(ref cause) => cause,
            DescribeLocationsError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by DescribeTags
#[derive(Debug, PartialEq)]
pub enum DescribeTagsError {
    /// <p>One or more parameters are not valid.</p>
    DirectConnectClient(String),
    /// <p>A server-side error occurred.</p>
    DirectConnectServer(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl DescribeTagsError {
    pub fn from_response(res: BufferedHttpResponse) -> DescribeTagsError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "DirectConnectClientException" => {
                    return DescribeTagsError::DirectConnectClient(String::from(error_message));
                }
                "DirectConnectServerException" => {
                    return DescribeTagsError::DirectConnectServer(String::from(error_message));
                }
                "ValidationException" => {
                    return DescribeTagsError::Validation(error_message.to_string());
                }
                _ => {}
            }
        }
        return DescribeTagsError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for DescribeTagsError {
    fn from(err: serde_json::error::Error) -> DescribeTagsError {
        DescribeTagsError::ParseError(err.description().to_string())
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
            DescribeTagsError::DirectConnectClient(ref cause) => cause,
            DescribeTagsError::DirectConnectServer(ref cause) => cause,
            DescribeTagsError::Validation(ref cause) => cause,
            DescribeTagsError::Credentials(ref err) => err.description(),
            DescribeTagsError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            DescribeTagsError::ParseError(ref cause) => cause,
            DescribeTagsError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by DescribeVirtualGateways
#[derive(Debug, PartialEq)]
pub enum DescribeVirtualGatewaysError {
    /// <p>One or more parameters are not valid.</p>
    DirectConnectClient(String),
    /// <p>A server-side error occurred.</p>
    DirectConnectServer(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl DescribeVirtualGatewaysError {
    pub fn from_response(res: BufferedHttpResponse) -> DescribeVirtualGatewaysError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "DirectConnectClientException" => {
                    return DescribeVirtualGatewaysError::DirectConnectClient(String::from(
                        error_message,
                    ));
                }
                "DirectConnectServerException" => {
                    return DescribeVirtualGatewaysError::DirectConnectServer(String::from(
                        error_message,
                    ));
                }
                "ValidationException" => {
                    return DescribeVirtualGatewaysError::Validation(error_message.to_string());
                }
                _ => {}
            }
        }
        return DescribeVirtualGatewaysError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for DescribeVirtualGatewaysError {
    fn from(err: serde_json::error::Error) -> DescribeVirtualGatewaysError {
        DescribeVirtualGatewaysError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for DescribeVirtualGatewaysError {
    fn from(err: CredentialsError) -> DescribeVirtualGatewaysError {
        DescribeVirtualGatewaysError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DescribeVirtualGatewaysError {
    fn from(err: HttpDispatchError) -> DescribeVirtualGatewaysError {
        DescribeVirtualGatewaysError::HttpDispatch(err)
    }
}
impl From<io::Error> for DescribeVirtualGatewaysError {
    fn from(err: io::Error) -> DescribeVirtualGatewaysError {
        DescribeVirtualGatewaysError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DescribeVirtualGatewaysError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeVirtualGatewaysError {
    fn description(&self) -> &str {
        match *self {
            DescribeVirtualGatewaysError::DirectConnectClient(ref cause) => cause,
            DescribeVirtualGatewaysError::DirectConnectServer(ref cause) => cause,
            DescribeVirtualGatewaysError::Validation(ref cause) => cause,
            DescribeVirtualGatewaysError::Credentials(ref err) => err.description(),
            DescribeVirtualGatewaysError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DescribeVirtualGatewaysError::ParseError(ref cause) => cause,
            DescribeVirtualGatewaysError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by DescribeVirtualInterfaces
#[derive(Debug, PartialEq)]
pub enum DescribeVirtualInterfacesError {
    /// <p>One or more parameters are not valid.</p>
    DirectConnectClient(String),
    /// <p>A server-side error occurred.</p>
    DirectConnectServer(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl DescribeVirtualInterfacesError {
    pub fn from_response(res: BufferedHttpResponse) -> DescribeVirtualInterfacesError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "DirectConnectClientException" => {
                    return DescribeVirtualInterfacesError::DirectConnectClient(String::from(
                        error_message,
                    ));
                }
                "DirectConnectServerException" => {
                    return DescribeVirtualInterfacesError::DirectConnectServer(String::from(
                        error_message,
                    ));
                }
                "ValidationException" => {
                    return DescribeVirtualInterfacesError::Validation(error_message.to_string());
                }
                _ => {}
            }
        }
        return DescribeVirtualInterfacesError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for DescribeVirtualInterfacesError {
    fn from(err: serde_json::error::Error) -> DescribeVirtualInterfacesError {
        DescribeVirtualInterfacesError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for DescribeVirtualInterfacesError {
    fn from(err: CredentialsError) -> DescribeVirtualInterfacesError {
        DescribeVirtualInterfacesError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DescribeVirtualInterfacesError {
    fn from(err: HttpDispatchError) -> DescribeVirtualInterfacesError {
        DescribeVirtualInterfacesError::HttpDispatch(err)
    }
}
impl From<io::Error> for DescribeVirtualInterfacesError {
    fn from(err: io::Error) -> DescribeVirtualInterfacesError {
        DescribeVirtualInterfacesError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DescribeVirtualInterfacesError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeVirtualInterfacesError {
    fn description(&self) -> &str {
        match *self {
            DescribeVirtualInterfacesError::DirectConnectClient(ref cause) => cause,
            DescribeVirtualInterfacesError::DirectConnectServer(ref cause) => cause,
            DescribeVirtualInterfacesError::Validation(ref cause) => cause,
            DescribeVirtualInterfacesError::Credentials(ref err) => err.description(),
            DescribeVirtualInterfacesError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DescribeVirtualInterfacesError::ParseError(ref cause) => cause,
            DescribeVirtualInterfacesError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by DisassociateConnectionFromLag
#[derive(Debug, PartialEq)]
pub enum DisassociateConnectionFromLagError {
    /// <p>One or more parameters are not valid.</p>
    DirectConnectClient(String),
    /// <p>A server-side error occurred.</p>
    DirectConnectServer(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl DisassociateConnectionFromLagError {
    pub fn from_response(res: BufferedHttpResponse) -> DisassociateConnectionFromLagError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "DirectConnectClientException" => {
                    return DisassociateConnectionFromLagError::DirectConnectClient(String::from(
                        error_message,
                    ));
                }
                "DirectConnectServerException" => {
                    return DisassociateConnectionFromLagError::DirectConnectServer(String::from(
                        error_message,
                    ));
                }
                "ValidationException" => {
                    return DisassociateConnectionFromLagError::Validation(error_message.to_string());
                }
                _ => {}
            }
        }
        return DisassociateConnectionFromLagError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for DisassociateConnectionFromLagError {
    fn from(err: serde_json::error::Error) -> DisassociateConnectionFromLagError {
        DisassociateConnectionFromLagError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for DisassociateConnectionFromLagError {
    fn from(err: CredentialsError) -> DisassociateConnectionFromLagError {
        DisassociateConnectionFromLagError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DisassociateConnectionFromLagError {
    fn from(err: HttpDispatchError) -> DisassociateConnectionFromLagError {
        DisassociateConnectionFromLagError::HttpDispatch(err)
    }
}
impl From<io::Error> for DisassociateConnectionFromLagError {
    fn from(err: io::Error) -> DisassociateConnectionFromLagError {
        DisassociateConnectionFromLagError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DisassociateConnectionFromLagError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DisassociateConnectionFromLagError {
    fn description(&self) -> &str {
        match *self {
            DisassociateConnectionFromLagError::DirectConnectClient(ref cause) => cause,
            DisassociateConnectionFromLagError::DirectConnectServer(ref cause) => cause,
            DisassociateConnectionFromLagError::Validation(ref cause) => cause,
            DisassociateConnectionFromLagError::Credentials(ref err) => err.description(),
            DisassociateConnectionFromLagError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DisassociateConnectionFromLagError::ParseError(ref cause) => cause,
            DisassociateConnectionFromLagError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by TagResource
#[derive(Debug, PartialEq)]
pub enum TagResourceError {
    /// <p>One or more parameters are not valid.</p>
    DirectConnectClient(String),
    /// <p>A server-side error occurred.</p>
    DirectConnectServer(String),
    /// <p>A tag key was specified more than once.</p>
    DuplicateTagKeys(String),
    /// <p>You have reached the limit on the number of tags that can be assigned.</p>
    TooManyTags(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl TagResourceError {
    pub fn from_response(res: BufferedHttpResponse) -> TagResourceError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "DirectConnectClientException" => {
                    return TagResourceError::DirectConnectClient(String::from(error_message));
                }
                "DirectConnectServerException" => {
                    return TagResourceError::DirectConnectServer(String::from(error_message));
                }
                "DuplicateTagKeysException" => {
                    return TagResourceError::DuplicateTagKeys(String::from(error_message));
                }
                "TooManyTagsException" => {
                    return TagResourceError::TooManyTags(String::from(error_message));
                }
                "ValidationException" => {
                    return TagResourceError::Validation(error_message.to_string());
                }
                _ => {}
            }
        }
        return TagResourceError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for TagResourceError {
    fn from(err: serde_json::error::Error) -> TagResourceError {
        TagResourceError::ParseError(err.description().to_string())
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
            TagResourceError::DirectConnectClient(ref cause) => cause,
            TagResourceError::DirectConnectServer(ref cause) => cause,
            TagResourceError::DuplicateTagKeys(ref cause) => cause,
            TagResourceError::TooManyTags(ref cause) => cause,
            TagResourceError::Validation(ref cause) => cause,
            TagResourceError::Credentials(ref err) => err.description(),
            TagResourceError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            TagResourceError::ParseError(ref cause) => cause,
            TagResourceError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by UntagResource
#[derive(Debug, PartialEq)]
pub enum UntagResourceError {
    /// <p>One or more parameters are not valid.</p>
    DirectConnectClient(String),
    /// <p>A server-side error occurred.</p>
    DirectConnectServer(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl UntagResourceError {
    pub fn from_response(res: BufferedHttpResponse) -> UntagResourceError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "DirectConnectClientException" => {
                    return UntagResourceError::DirectConnectClient(String::from(error_message));
                }
                "DirectConnectServerException" => {
                    return UntagResourceError::DirectConnectServer(String::from(error_message));
                }
                "ValidationException" => {
                    return UntagResourceError::Validation(error_message.to_string());
                }
                _ => {}
            }
        }
        return UntagResourceError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for UntagResourceError {
    fn from(err: serde_json::error::Error) -> UntagResourceError {
        UntagResourceError::ParseError(err.description().to_string())
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
            UntagResourceError::DirectConnectClient(ref cause) => cause,
            UntagResourceError::DirectConnectServer(ref cause) => cause,
            UntagResourceError::Validation(ref cause) => cause,
            UntagResourceError::Credentials(ref err) => err.description(),
            UntagResourceError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            UntagResourceError::ParseError(ref cause) => cause,
            UntagResourceError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by UpdateLag
#[derive(Debug, PartialEq)]
pub enum UpdateLagError {
    /// <p>One or more parameters are not valid.</p>
    DirectConnectClient(String),
    /// <p>A server-side error occurred.</p>
    DirectConnectServer(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl UpdateLagError {
    pub fn from_response(res: BufferedHttpResponse) -> UpdateLagError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "DirectConnectClientException" => {
                    return UpdateLagError::DirectConnectClient(String::from(error_message));
                }
                "DirectConnectServerException" => {
                    return UpdateLagError::DirectConnectServer(String::from(error_message));
                }
                "ValidationException" => {
                    return UpdateLagError::Validation(error_message.to_string());
                }
                _ => {}
            }
        }
        return UpdateLagError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for UpdateLagError {
    fn from(err: serde_json::error::Error) -> UpdateLagError {
        UpdateLagError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for UpdateLagError {
    fn from(err: CredentialsError) -> UpdateLagError {
        UpdateLagError::Credentials(err)
    }
}
impl From<HttpDispatchError> for UpdateLagError {
    fn from(err: HttpDispatchError) -> UpdateLagError {
        UpdateLagError::HttpDispatch(err)
    }
}
impl From<io::Error> for UpdateLagError {
    fn from(err: io::Error) -> UpdateLagError {
        UpdateLagError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for UpdateLagError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for UpdateLagError {
    fn description(&self) -> &str {
        match *self {
            UpdateLagError::DirectConnectClient(ref cause) => cause,
            UpdateLagError::DirectConnectServer(ref cause) => cause,
            UpdateLagError::Validation(ref cause) => cause,
            UpdateLagError::Credentials(ref err) => err.description(),
            UpdateLagError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            UpdateLagError::ParseError(ref cause) => cause,
            UpdateLagError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by UpdateVirtualInterfaceAttributes
#[derive(Debug, PartialEq)]
pub enum UpdateVirtualInterfaceAttributesError {
    /// <p>One or more parameters are not valid.</p>
    DirectConnectClient(String),
    /// <p>A server-side error occurred.</p>
    DirectConnectServer(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl UpdateVirtualInterfaceAttributesError {
    pub fn from_response(res: BufferedHttpResponse) -> UpdateVirtualInterfaceAttributesError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "DirectConnectClientException" => {
                    return UpdateVirtualInterfaceAttributesError::DirectConnectClient(String::from(
                        error_message,
                    ));
                }
                "DirectConnectServerException" => {
                    return UpdateVirtualInterfaceAttributesError::DirectConnectServer(String::from(
                        error_message,
                    ));
                }
                "ValidationException" => {
                    return UpdateVirtualInterfaceAttributesError::Validation(
                        error_message.to_string(),
                    );
                }
                _ => {}
            }
        }
        return UpdateVirtualInterfaceAttributesError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for UpdateVirtualInterfaceAttributesError {
    fn from(err: serde_json::error::Error) -> UpdateVirtualInterfaceAttributesError {
        UpdateVirtualInterfaceAttributesError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for UpdateVirtualInterfaceAttributesError {
    fn from(err: CredentialsError) -> UpdateVirtualInterfaceAttributesError {
        UpdateVirtualInterfaceAttributesError::Credentials(err)
    }
}
impl From<HttpDispatchError> for UpdateVirtualInterfaceAttributesError {
    fn from(err: HttpDispatchError) -> UpdateVirtualInterfaceAttributesError {
        UpdateVirtualInterfaceAttributesError::HttpDispatch(err)
    }
}
impl From<io::Error> for UpdateVirtualInterfaceAttributesError {
    fn from(err: io::Error) -> UpdateVirtualInterfaceAttributesError {
        UpdateVirtualInterfaceAttributesError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for UpdateVirtualInterfaceAttributesError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for UpdateVirtualInterfaceAttributesError {
    fn description(&self) -> &str {
        match *self {
            UpdateVirtualInterfaceAttributesError::DirectConnectClient(ref cause) => cause,
            UpdateVirtualInterfaceAttributesError::DirectConnectServer(ref cause) => cause,
            UpdateVirtualInterfaceAttributesError::Validation(ref cause) => cause,
            UpdateVirtualInterfaceAttributesError::Credentials(ref err) => err.description(),
            UpdateVirtualInterfaceAttributesError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            UpdateVirtualInterfaceAttributesError::ParseError(ref cause) => cause,
            UpdateVirtualInterfaceAttributesError::Unknown(_) => "unknown error",
        }
    }
}
/// Trait representing the capabilities of the AWS Direct Connect API. AWS Direct Connect clients implement this trait.
pub trait DirectConnect {
    /// <p><p>Deprecated. Use <a>AllocateHostedConnection</a> instead.</p> <p>Creates a hosted connection on an interconnect.</p> <p>Allocates a VLAN number and a specified amount of bandwidth for use by a hosted connection on the specified interconnect.</p> <note> <p>Intended for use by AWS Direct Connect partners only.</p> </note></p>
    fn allocate_connection_on_interconnect(
        &self,
        input: AllocateConnectionOnInterconnectRequest,
    ) -> RusotoFuture<Connection, AllocateConnectionOnInterconnectError>;

    /// <p><p>Creates a hosted connection on the specified interconnect or a link aggregation group (LAG).</p> <p>Allocates a VLAN number and a specified amount of bandwidth for use by a hosted connection on the specified interconnect or LAG.</p> <note> <p>Intended for use by AWS Direct Connect partners only.</p> </note></p>
    fn allocate_hosted_connection(
        &self,
        input: AllocateHostedConnectionRequest,
    ) -> RusotoFuture<Connection, AllocateHostedConnectionError>;

    /// <p>Provisions a private virtual interface to be owned by the specified AWS account.</p> <p>Virtual interfaces created using this action must be confirmed by the owner using <a>ConfirmPrivateVirtualInterface</a>. Until then, the virtual interface is in the <code>Confirming</code> state and is not available to handle traffic.</p>
    fn allocate_private_virtual_interface(
        &self,
        input: AllocatePrivateVirtualInterfaceRequest,
    ) -> RusotoFuture<VirtualInterface, AllocatePrivateVirtualInterfaceError>;

    /// <p>Provisions a public virtual interface to be owned by the specified AWS account.</p> <p>The owner of a connection calls this function to provision a public virtual interface to be owned by the specified AWS account.</p> <p>Virtual interfaces created using this function must be confirmed by the owner using <a>ConfirmPublicVirtualInterface</a>. Until this step has been completed, the virtual interface is in the <code>confirming</code> state and is not available to handle traffic.</p> <p>When creating an IPv6 public virtual interface, omit the Amazon address and customer address. IPv6 addresses are automatically assigned from the Amazon pool of IPv6 addresses; you cannot specify custom IPv6 addresses.</p>
    fn allocate_public_virtual_interface(
        &self,
        input: AllocatePublicVirtualInterfaceRequest,
    ) -> RusotoFuture<VirtualInterface, AllocatePublicVirtualInterfaceError>;

    /// <p>Associates an existing connection with a link aggregation group (LAG). The connection is interrupted and re-established as a member of the LAG (connectivity to AWS is interrupted). The connection must be hosted on the same AWS Direct Connect endpoint as the LAG, and its bandwidth must match the bandwidth for the LAG. You can re-associate a connection that's currently associated with a different LAG; however, if removing the connection would cause the original LAG to fall below its setting for minimum number of operational connections, the request fails.</p> <p>Any virtual interfaces that are directly associated with the connection are automatically re-associated with the LAG. If the connection was originally associated with a different LAG, the virtual interfaces remain associated with the original LAG.</p> <p>For interconnects, any hosted connections are automatically re-associated with the LAG. If the interconnect was originally associated with a different LAG, the hosted connections remain associated with the original LAG.</p>
    fn associate_connection_with_lag(
        &self,
        input: AssociateConnectionWithLagRequest,
    ) -> RusotoFuture<Connection, AssociateConnectionWithLagError>;

    /// <p><p>Associates a hosted connection and its virtual interfaces with a link aggregation group (LAG) or interconnect. If the target interconnect or LAG has an existing hosted connection with a conflicting VLAN number or IP address, the operation fails. This action temporarily interrupts the hosted connection&#39;s connectivity to AWS as it is being migrated.</p> <note> <p>Intended for use by AWS Direct Connect partners only.</p> </note></p>
    fn associate_hosted_connection(
        &self,
        input: AssociateHostedConnectionRequest,
    ) -> RusotoFuture<Connection, AssociateHostedConnectionError>;

    /// <p>Associates a virtual interface with a specified link aggregation group (LAG) or connection. Connectivity to AWS is temporarily interrupted as the virtual interface is being migrated. If the target connection or LAG has an associated virtual interface with a conflicting VLAN number or a conflicting IP address, the operation fails.</p> <p>Virtual interfaces associated with a hosted connection cannot be associated with a LAG; hosted connections must be migrated along with their virtual interfaces using <a>AssociateHostedConnection</a>.</p> <p>To reassociate a virtual interface to a new connection or LAG, the requester must own either the virtual interface itself or the connection to which the virtual interface is currently associated. Additionally, the requester must own the connection or LAG for the association.</p>
    fn associate_virtual_interface(
        &self,
        input: AssociateVirtualInterfaceRequest,
    ) -> RusotoFuture<VirtualInterface, AssociateVirtualInterfaceError>;

    /// <p>Confirms the creation of the specified hosted connection on an interconnect.</p> <p>Upon creation, the hosted connection is initially in the <code>Ordering</code> state, and remains in this state until the owner confirms creation of the hosted connection.</p>
    fn confirm_connection(
        &self,
        input: ConfirmConnectionRequest,
    ) -> RusotoFuture<ConfirmConnectionResponse, ConfirmConnectionError>;

    /// <p>Accepts ownership of a private virtual interface created by another AWS account.</p> <p>After the virtual interface owner makes this call, the virtual interface is created and attached to the specified virtual private gateway or Direct Connect gateway, and is made available to handle traffic.</p>
    fn confirm_private_virtual_interface(
        &self,
        input: ConfirmPrivateVirtualInterfaceRequest,
    ) -> RusotoFuture<ConfirmPrivateVirtualInterfaceResponse, ConfirmPrivateVirtualInterfaceError>;

    /// <p>Accepts ownership of a public virtual interface created by another AWS account.</p> <p>After the virtual interface owner makes this call, the specified virtual interface is created and made available to handle traffic.</p>
    fn confirm_public_virtual_interface(
        &self,
        input: ConfirmPublicVirtualInterfaceRequest,
    ) -> RusotoFuture<ConfirmPublicVirtualInterfaceResponse, ConfirmPublicVirtualInterfaceError>;

    /// <p>Creates a BGP peer on the specified virtual interface.</p> <p>You must create a BGP peer for the corresponding address family (IPv4/IPv6) in order to access AWS resources that also use that address family.</p> <p>If logical redundancy is not supported by the connection, interconnect, or LAG, the BGP peer cannot be in the same address family as an existing BGP peer on the virtual interface.</p> <p>When creating a IPv6 BGP peer, omit the Amazon address and customer address. IPv6 addresses are automatically assigned from the Amazon pool of IPv6 addresses; you cannot specify custom IPv6 addresses.</p> <p>For a public virtual interface, the Autonomous System Number (ASN) must be private or already whitelisted for the virtual interface.</p>
    fn create_bgp_peer(
        &self,
        input: CreateBGPPeerRequest,
    ) -> RusotoFuture<CreateBGPPeerResponse, CreateBGPPeerError>;

    /// <p>Creates a connection between a customer network and a specific AWS Direct Connect location.</p> <p>A connection links your internal network to an AWS Direct Connect location over a standard Ethernet fiber-optic cable. One end of the cable is connected to your router, the other to an AWS Direct Connect router.</p> <p>To find the locations for your Region, use <a>DescribeLocations</a>.</p> <p>You can automatically add the new connection to a link aggregation group (LAG) by specifying a LAG ID in the request. This ensures that the new connection is allocated on the same AWS Direct Connect endpoint that hosts the specified LAG. If there are no available ports on the endpoint, the request fails and no connection is created.</p>
    fn create_connection(
        &self,
        input: CreateConnectionRequest,
    ) -> RusotoFuture<Connection, CreateConnectionError>;

    /// <p>Creates a Direct Connect gateway, which is an intermediate object that enables you to connect a set of virtual interfaces and virtual private gateways. A Direct Connect gateway is global and visible in any AWS Region after it is created. The virtual interfaces and virtual private gateways that are connected through a Direct Connect gateway can be in different AWS Regions. This enables you to connect to a VPC in any Region, regardless of the Region in which the virtual interfaces are located, and pass traffic between them.</p>
    fn create_direct_connect_gateway(
        &self,
        input: CreateDirectConnectGatewayRequest,
    ) -> RusotoFuture<CreateDirectConnectGatewayResult, CreateDirectConnectGatewayError>;

    /// <p>Creates an association between a Direct Connect gateway and a virtual private gateway. The virtual private gateway must be attached to a VPC and must not be associated with another Direct Connect gateway.</p>
    fn create_direct_connect_gateway_association(
        &self,
        input: CreateDirectConnectGatewayAssociationRequest,
    ) -> RusotoFuture<
        CreateDirectConnectGatewayAssociationResult,
        CreateDirectConnectGatewayAssociationError,
    >;

    /// <p><p>Creates an interconnect between an AWS Direct Connect partner&#39;s network and a specific AWS Direct Connect location.</p> <p>An interconnect is a connection which is capable of hosting other connections. The partner can use an interconnect to provide sub-1Gbps AWS Direct Connect service to tier 2 customers who do not have their own connections. Like a standard connection, an interconnect links the partner&#39;s network to an AWS Direct Connect location over a standard Ethernet fiber-optic cable. One end is connected to the partner&#39;s router, the other to an AWS Direct Connect router.</p> <p>You can automatically add the new interconnect to a link aggregation group (LAG) by specifying a LAG ID in the request. This ensures that the new interconnect is allocated on the same AWS Direct Connect endpoint that hosts the specified LAG. If there are no available ports on the endpoint, the request fails and no interconnect is created.</p> <p>For each end customer, the AWS Direct Connect partner provisions a connection on their interconnect by calling <a>AllocateConnectionOnInterconnect</a>. The end customer can then connect to AWS resources by creating a virtual interface on their connection, using the VLAN assigned to them by the partner.</p> <note> <p>Intended for use by AWS Direct Connect partners only.</p> </note></p>
    fn create_interconnect(
        &self,
        input: CreateInterconnectRequest,
    ) -> RusotoFuture<Interconnect, CreateInterconnectError>;

    /// <p>Creates a link aggregation group (LAG) with the specified number of bundled physical connections between the customer network and a specific AWS Direct Connect location. A LAG is a logical interface that uses the Link Aggregation Control Protocol (LACP) to aggregate multiple interfaces, enabling you to treat them as a single interface.</p> <p>All connections in a LAG must use the same bandwidth and must terminate at the same AWS Direct Connect endpoint.</p> <p>You can have up to 10 connections per LAG. Regardless of this limit, if you request more connections for the LAG than AWS Direct Connect can allocate on a single endpoint, no LAG is created.</p> <p>You can specify an existing physical connection or interconnect to include in the LAG (which counts towards the total number of connections). Doing so interrupts the current physical connection or hosted connections, and re-establishes them as a member of the LAG. The LAG will be created on the same AWS Direct Connect endpoint to which the connection terminates. Any virtual interfaces associated with the connection are automatically disassociated and re-associated with the LAG. The connection ID does not change.</p> <p>If the AWS account used to create a LAG is a registered AWS Direct Connect partner, the LAG is automatically enabled to host sub-connections. For a LAG owned by a partner, any associated virtual interfaces cannot be directly configured.</p>
    fn create_lag(&self, input: CreateLagRequest) -> RusotoFuture<Lag, CreateLagError>;

    /// <p>Creates a private virtual interface. A virtual interface is the VLAN that transports AWS Direct Connect traffic. A private virtual interface can be connected to either a Direct Connect gateway or a Virtual Private Gateway (VGW). Connecting the private virtual interface to a Direct Connect gateway enables the possibility for connecting to multiple VPCs, including VPCs in different AWS Regions. Connecting the private virtual interface to a VGW only provides access to a single VPC within the same Region.</p>
    fn create_private_virtual_interface(
        &self,
        input: CreatePrivateVirtualInterfaceRequest,
    ) -> RusotoFuture<VirtualInterface, CreatePrivateVirtualInterfaceError>;

    /// <p>Creates a public virtual interface. A virtual interface is the VLAN that transports AWS Direct Connect traffic. A public virtual interface supports sending traffic to public services of AWS such as Amazon S3.</p> <p>When creating an IPv6 public virtual interface (<code>addressFamily</code> is <code>ipv6</code>), leave the <code>customer</code> and <code>amazon</code> address fields blank to use auto-assigned IPv6 space. Custom IPv6 addresses are not supported.</p>
    fn create_public_virtual_interface(
        &self,
        input: CreatePublicVirtualInterfaceRequest,
    ) -> RusotoFuture<VirtualInterface, CreatePublicVirtualInterfaceError>;

    /// <p>Deletes the specified BGP peer on the specified virtual interface with the specified customer address and ASN.</p> <p>You cannot delete the last BGP peer from a virtual interface.</p>
    fn delete_bgp_peer(
        &self,
        input: DeleteBGPPeerRequest,
    ) -> RusotoFuture<DeleteBGPPeerResponse, DeleteBGPPeerError>;

    /// <p>Deletes the specified connection.</p> <p>Deleting a connection only stops the AWS Direct Connect port hour and data transfer charges. If you are partnering with any third parties to connect with the AWS Direct Connect location, you must cancel your service with them separately.</p>
    fn delete_connection(
        &self,
        input: DeleteConnectionRequest,
    ) -> RusotoFuture<Connection, DeleteConnectionError>;

    /// <p>Deletes the specified Direct Connect gateway. You must first delete all virtual interfaces that are attached to the Direct Connect gateway and disassociate all virtual private gateways that are associated with the Direct Connect gateway.</p>
    fn delete_direct_connect_gateway(
        &self,
        input: DeleteDirectConnectGatewayRequest,
    ) -> RusotoFuture<DeleteDirectConnectGatewayResult, DeleteDirectConnectGatewayError>;

    /// <p>Deletes the association between the specified Direct Connect gateway and virtual private gateway.</p>
    fn delete_direct_connect_gateway_association(
        &self,
        input: DeleteDirectConnectGatewayAssociationRequest,
    ) -> RusotoFuture<
        DeleteDirectConnectGatewayAssociationResult,
        DeleteDirectConnectGatewayAssociationError,
    >;

    /// <p><p>Deletes the specified interconnect.</p> <note> <p>Intended for use by AWS Direct Connect partners only.</p> </note></p>
    fn delete_interconnect(
        &self,
        input: DeleteInterconnectRequest,
    ) -> RusotoFuture<DeleteInterconnectResponse, DeleteInterconnectError>;

    /// <p>Deletes the specified link aggregation group (LAG). You cannot delete a LAG if it has active virtual interfaces or hosted connections.</p>
    fn delete_lag(&self, input: DeleteLagRequest) -> RusotoFuture<Lag, DeleteLagError>;

    /// <p>Deletes a virtual interface.</p>
    fn delete_virtual_interface(
        &self,
        input: DeleteVirtualInterfaceRequest,
    ) -> RusotoFuture<DeleteVirtualInterfaceResponse, DeleteVirtualInterfaceError>;

    /// <p>Deprecated. Use <a>DescribeLoa</a> instead.</p> <p>Gets the LOA-CFA for a connection.</p> <p>The Letter of Authorization - Connecting Facility Assignment (LOA-CFA) is a document that your APN partner or service provider uses when establishing your cross connect to AWS at the colocation facility. For more information, see <a href="https://docs.aws.amazon.com/directconnect/latest/UserGuide/Colocation.html">Requesting Cross Connects at AWS Direct Connect Locations</a> in the <i>AWS Direct Connect User Guide</i>.</p>
    fn describe_connection_loa(
        &self,
        input: DescribeConnectionLoaRequest,
    ) -> RusotoFuture<DescribeConnectionLoaResponse, DescribeConnectionLoaError>;

    /// <p>Displays the specified connection or all connections in this Region.</p>
    fn describe_connections(
        &self,
        input: DescribeConnectionsRequest,
    ) -> RusotoFuture<Connections, DescribeConnectionsError>;

    /// <p><p>Deprecated. Use <a>DescribeHostedConnections</a> instead.</p> <p>Lists the connections that have been provisioned on the specified interconnect.</p> <note> <p>Intended for use by AWS Direct Connect partners only.</p> </note></p>
    fn describe_connections_on_interconnect(
        &self,
        input: DescribeConnectionsOnInterconnectRequest,
    ) -> RusotoFuture<Connections, DescribeConnectionsOnInterconnectError>;

    /// <p>Lists the associations between your Direct Connect gateways and virtual private gateways. You must specify a Direct Connect gateway, a virtual private gateway, or both. If you specify a Direct Connect gateway, the response contains all virtual private gateways associated with the Direct Connect gateway. If you specify a virtual private gateway, the response contains all Direct Connect gateways associated with the virtual private gateway. If you specify both, the response contains the association between the Direct Connect gateway and the virtual private gateway.</p>
    fn describe_direct_connect_gateway_associations(
        &self,
        input: DescribeDirectConnectGatewayAssociationsRequest,
    ) -> RusotoFuture<
        DescribeDirectConnectGatewayAssociationsResult,
        DescribeDirectConnectGatewayAssociationsError,
    >;

    /// <p>Lists the attachments between your Direct Connect gateways and virtual interfaces. You must specify a Direct Connect gateway, a virtual interface, or both. If you specify a Direct Connect gateway, the response contains all virtual interfaces attached to the Direct Connect gateway. If you specify a virtual interface, the response contains all Direct Connect gateways attached to the virtual interface. If you specify both, the response contains the attachment between the Direct Connect gateway and the virtual interface.</p>
    fn describe_direct_connect_gateway_attachments(
        &self,
        input: DescribeDirectConnectGatewayAttachmentsRequest,
    ) -> RusotoFuture<
        DescribeDirectConnectGatewayAttachmentsResult,
        DescribeDirectConnectGatewayAttachmentsError,
    >;

    /// <p>Lists all your Direct Connect gateways or only the specified Direct Connect gateway. Deleted Direct Connect gateways are not returned.</p>
    fn describe_direct_connect_gateways(
        &self,
        input: DescribeDirectConnectGatewaysRequest,
    ) -> RusotoFuture<DescribeDirectConnectGatewaysResult, DescribeDirectConnectGatewaysError>;

    /// <p><p>Lists the hosted connections that have been provisioned on the specified interconnect or link aggregation group (LAG).</p> <note> <p>Intended for use by AWS Direct Connect partners only.</p> </note></p>
    fn describe_hosted_connections(
        &self,
        input: DescribeHostedConnectionsRequest,
    ) -> RusotoFuture<Connections, DescribeHostedConnectionsError>;

    /// <p>Deprecated. Use <a>DescribeLoa</a> instead.</p> <p>Gets the LOA-CFA for the specified interconnect.</p> <p>The Letter of Authorization - Connecting Facility Assignment (LOA-CFA) is a document that is used when establishing your cross connect to AWS at the colocation facility. For more information, see <a href="https://docs.aws.amazon.com/directconnect/latest/UserGuide/Colocation.html">Requesting Cross Connects at AWS Direct Connect Locations</a> in the <i>AWS Direct Connect User Guide</i>.</p>
    fn describe_interconnect_loa(
        &self,
        input: DescribeInterconnectLoaRequest,
    ) -> RusotoFuture<DescribeInterconnectLoaResponse, DescribeInterconnectLoaError>;

    /// <p>Lists the interconnects owned by the AWS account or only the specified interconnect.</p>
    fn describe_interconnects(
        &self,
        input: DescribeInterconnectsRequest,
    ) -> RusotoFuture<Interconnects, DescribeInterconnectsError>;

    /// <p>Describes all your link aggregation groups (LAG) or the specified LAG.</p>
    fn describe_lags(&self, input: DescribeLagsRequest) -> RusotoFuture<Lags, DescribeLagsError>;

    /// <p>Gets the LOA-CFA for a connection, interconnect, or link aggregation group (LAG).</p> <p>The Letter of Authorization - Connecting Facility Assignment (LOA-CFA) is a document that is used when establishing your cross connect to AWS at the colocation facility. For more information, see <a href="https://docs.aws.amazon.com/directconnect/latest/UserGuide/Colocation.html">Requesting Cross Connects at AWS Direct Connect Locations</a> in the <i>AWS Direct Connect User Guide</i>.</p>
    fn describe_loa(&self, input: DescribeLoaRequest) -> RusotoFuture<Loa, DescribeLoaError>;

    /// <p>Lists the AWS Direct Connect locations in the current AWS Region. These are the locations that can be selected when calling <a>CreateConnection</a> or <a>CreateInterconnect</a>.</p>
    fn describe_locations(&self) -> RusotoFuture<Locations, DescribeLocationsError>;

    /// <p>Describes the tags associated with the specified AWS Direct Connect resources.</p>
    fn describe_tags(
        &self,
        input: DescribeTagsRequest,
    ) -> RusotoFuture<DescribeTagsResponse, DescribeTagsError>;

    /// <p>Lists the virtual private gateways owned by the AWS account.</p> <p>You can create one or more AWS Direct Connect private virtual interfaces linked to a virtual private gateway.</p>
    fn describe_virtual_gateways(
        &self,
    ) -> RusotoFuture<VirtualGateways, DescribeVirtualGatewaysError>;

    /// <p>Displays all virtual interfaces for an AWS account. Virtual interfaces deleted fewer than 15 minutes before you make the request are also returned. If you specify a connection ID, only the virtual interfaces associated with the connection are returned. If you specify a virtual interface ID, then only a single virtual interface is returned.</p> <p>A virtual interface (VLAN) transmits the traffic between the AWS Direct Connect location and the customer network.</p>
    fn describe_virtual_interfaces(
        &self,
        input: DescribeVirtualInterfacesRequest,
    ) -> RusotoFuture<VirtualInterfaces, DescribeVirtualInterfacesError>;

    /// <p>Disassociates a connection from a link aggregation group (LAG). The connection is interrupted and re-established as a standalone connection (the connection is not deleted; to delete the connection, use the <a>DeleteConnection</a> request). If the LAG has associated virtual interfaces or hosted connections, they remain associated with the LAG. A disassociated connection owned by an AWS Direct Connect partner is automatically converted to an interconnect.</p> <p>If disassociating the connection would cause the LAG to fall below its setting for minimum number of operational connections, the request fails, except when it's the last member of the LAG. If all connections are disassociated, the LAG continues to exist as an empty LAG with no physical connections. </p>
    fn disassociate_connection_from_lag(
        &self,
        input: DisassociateConnectionFromLagRequest,
    ) -> RusotoFuture<Connection, DisassociateConnectionFromLagError>;

    /// <p>Adds the specified tags to the specified AWS Direct Connect resource. Each resource can have a maximum of 50 tags.</p> <p>Each tag consists of a key and an optional value. If a tag with the same key is already associated with the resource, this action updates its value.</p>
    fn tag_resource(
        &self,
        input: TagResourceRequest,
    ) -> RusotoFuture<TagResourceResponse, TagResourceError>;

    /// <p>Removes one or more tags from the specified AWS Direct Connect resource.</p>
    fn untag_resource(
        &self,
        input: UntagResourceRequest,
    ) -> RusotoFuture<UntagResourceResponse, UntagResourceError>;

    /// <p>Updates the attributes of the specified link aggregation group (LAG).</p> <p>You can update the following attributes:</p> <ul> <li> <p>The name of the LAG.</p> </li> <li> <p>The value for the minimum number of connections that must be operational for the LAG itself to be operational. </p> </li> </ul> <p>When you create a LAG, the default value for the minimum number of operational connections is zero (0). If you update this value and the number of operational connections falls below the specified value, the LAG automatically goes down to avoid over-utilization of the remaining connections. Adjust this value with care, as it could force the LAG down if it is set higher than the current number of operational connections.</p>
    fn update_lag(&self, input: UpdateLagRequest) -> RusotoFuture<Lag, UpdateLagError>;

    /// <p>Updates the specified attributes of the specified virtual private interface.</p> <p>Setting the MTU of a virtual interface to 9001 (jumbo frames) can cause an update to the underlying physical connection if it wasn't updated to support jumbo frames. Updating the connection disrupts network connectivity for all virtual interfaces associated with the connection for up to 30 seconds. To check whether your connection supports jumbo frames, call <a>DescribeConnections</a>. To check whether your virtual interface supports jumbo frames, call <a>DescribeVirtualInterfaces</a>.</p>
    fn update_virtual_interface_attributes(
        &self,
        input: UpdateVirtualInterfaceAttributesRequest,
    ) -> RusotoFuture<VirtualInterface, UpdateVirtualInterfaceAttributesError>;
}
/// A client for the AWS Direct Connect API.
#[derive(Clone)]
pub struct DirectConnectClient {
    client: Client,
    region: region::Region,
}

impl DirectConnectClient {
    /// Creates a client backed by the default tokio event loop.
    ///
    /// The client will use the default credentials provider and tls client.
    pub fn new(region: region::Region) -> DirectConnectClient {
        DirectConnectClient {
            client: Client::shared(),
            region: region,
        }
    }

    pub fn new_with<P, D>(
        request_dispatcher: D,
        credentials_provider: P,
        region: region::Region,
    ) -> DirectConnectClient
    where
        P: ProvideAwsCredentials + Send + Sync + 'static,
        P::Future: Send,
        D: DispatchSignedRequest + Send + Sync + 'static,
        D::Future: Send,
    {
        DirectConnectClient {
            client: Client::new_with(credentials_provider, request_dispatcher),
            region: region,
        }
    }
}

impl DirectConnect for DirectConnectClient {
    /// <p><p>Deprecated. Use <a>AllocateHostedConnection</a> instead.</p> <p>Creates a hosted connection on an interconnect.</p> <p>Allocates a VLAN number and a specified amount of bandwidth for use by a hosted connection on the specified interconnect.</p> <note> <p>Intended for use by AWS Direct Connect partners only.</p> </note></p>
    fn allocate_connection_on_interconnect(
        &self,
        input: AllocateConnectionOnInterconnectRequest,
    ) -> RusotoFuture<Connection, AllocateConnectionOnInterconnectError> {
        let mut request = SignedRequest::new("POST", "directconnect", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "OvertureService.AllocateConnectionOnInterconnect",
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

                    serde_json::from_str::<Connection>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(AllocateConnectionOnInterconnectError::from_response(
                        response,
                    ))
                }))
            }
        })
    }

    /// <p><p>Creates a hosted connection on the specified interconnect or a link aggregation group (LAG).</p> <p>Allocates a VLAN number and a specified amount of bandwidth for use by a hosted connection on the specified interconnect or LAG.</p> <note> <p>Intended for use by AWS Direct Connect partners only.</p> </note></p>
    fn allocate_hosted_connection(
        &self,
        input: AllocateHostedConnectionRequest,
    ) -> RusotoFuture<Connection, AllocateHostedConnectionError> {
        let mut request = SignedRequest::new("POST", "directconnect", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "OvertureService.AllocateHostedConnection");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<Connection>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(AllocateHostedConnectionError::from_response(response))
                }))
            }
        })
    }

    /// <p>Provisions a private virtual interface to be owned by the specified AWS account.</p> <p>Virtual interfaces created using this action must be confirmed by the owner using <a>ConfirmPrivateVirtualInterface</a>. Until then, the virtual interface is in the <code>Confirming</code> state and is not available to handle traffic.</p>
    fn allocate_private_virtual_interface(
        &self,
        input: AllocatePrivateVirtualInterfaceRequest,
    ) -> RusotoFuture<VirtualInterface, AllocatePrivateVirtualInterfaceError> {
        let mut request = SignedRequest::new("POST", "directconnect", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "OvertureService.AllocatePrivateVirtualInterface",
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

                    serde_json::from_str::<VirtualInterface>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(AllocatePrivateVirtualInterfaceError::from_response(
                        response,
                    ))
                }))
            }
        })
    }

    /// <p>Provisions a public virtual interface to be owned by the specified AWS account.</p> <p>The owner of a connection calls this function to provision a public virtual interface to be owned by the specified AWS account.</p> <p>Virtual interfaces created using this function must be confirmed by the owner using <a>ConfirmPublicVirtualInterface</a>. Until this step has been completed, the virtual interface is in the <code>confirming</code> state and is not available to handle traffic.</p> <p>When creating an IPv6 public virtual interface, omit the Amazon address and customer address. IPv6 addresses are automatically assigned from the Amazon pool of IPv6 addresses; you cannot specify custom IPv6 addresses.</p>
    fn allocate_public_virtual_interface(
        &self,
        input: AllocatePublicVirtualInterfaceRequest,
    ) -> RusotoFuture<VirtualInterface, AllocatePublicVirtualInterfaceError> {
        let mut request = SignedRequest::new("POST", "directconnect", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "OvertureService.AllocatePublicVirtualInterface",
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

                    serde_json::from_str::<VirtualInterface>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(AllocatePublicVirtualInterfaceError::from_response(response))
                }))
            }
        })
    }

    /// <p>Associates an existing connection with a link aggregation group (LAG). The connection is interrupted and re-established as a member of the LAG (connectivity to AWS is interrupted). The connection must be hosted on the same AWS Direct Connect endpoint as the LAG, and its bandwidth must match the bandwidth for the LAG. You can re-associate a connection that's currently associated with a different LAG; however, if removing the connection would cause the original LAG to fall below its setting for minimum number of operational connections, the request fails.</p> <p>Any virtual interfaces that are directly associated with the connection are automatically re-associated with the LAG. If the connection was originally associated with a different LAG, the virtual interfaces remain associated with the original LAG.</p> <p>For interconnects, any hosted connections are automatically re-associated with the LAG. If the interconnect was originally associated with a different LAG, the hosted connections remain associated with the original LAG.</p>
    fn associate_connection_with_lag(
        &self,
        input: AssociateConnectionWithLagRequest,
    ) -> RusotoFuture<Connection, AssociateConnectionWithLagError> {
        let mut request = SignedRequest::new("POST", "directconnect", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "OvertureService.AssociateConnectionWithLag");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<Connection>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(AssociateConnectionWithLagError::from_response(response))
                }))
            }
        })
    }

    /// <p><p>Associates a hosted connection and its virtual interfaces with a link aggregation group (LAG) or interconnect. If the target interconnect or LAG has an existing hosted connection with a conflicting VLAN number or IP address, the operation fails. This action temporarily interrupts the hosted connection&#39;s connectivity to AWS as it is being migrated.</p> <note> <p>Intended for use by AWS Direct Connect partners only.</p> </note></p>
    fn associate_hosted_connection(
        &self,
        input: AssociateHostedConnectionRequest,
    ) -> RusotoFuture<Connection, AssociateHostedConnectionError> {
        let mut request = SignedRequest::new("POST", "directconnect", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "OvertureService.AssociateHostedConnection");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<Connection>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(AssociateHostedConnectionError::from_response(response))
                }))
            }
        })
    }

    /// <p>Associates a virtual interface with a specified link aggregation group (LAG) or connection. Connectivity to AWS is temporarily interrupted as the virtual interface is being migrated. If the target connection or LAG has an associated virtual interface with a conflicting VLAN number or a conflicting IP address, the operation fails.</p> <p>Virtual interfaces associated with a hosted connection cannot be associated with a LAG; hosted connections must be migrated along with their virtual interfaces using <a>AssociateHostedConnection</a>.</p> <p>To reassociate a virtual interface to a new connection or LAG, the requester must own either the virtual interface itself or the connection to which the virtual interface is currently associated. Additionally, the requester must own the connection or LAG for the association.</p>
    fn associate_virtual_interface(
        &self,
        input: AssociateVirtualInterfaceRequest,
    ) -> RusotoFuture<VirtualInterface, AssociateVirtualInterfaceError> {
        let mut request = SignedRequest::new("POST", "directconnect", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "OvertureService.AssociateVirtualInterface");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<VirtualInterface>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(AssociateVirtualInterfaceError::from_response(response))
                }))
            }
        })
    }

    /// <p>Confirms the creation of the specified hosted connection on an interconnect.</p> <p>Upon creation, the hosted connection is initially in the <code>Ordering</code> state, and remains in this state until the owner confirms creation of the hosted connection.</p>
    fn confirm_connection(
        &self,
        input: ConfirmConnectionRequest,
    ) -> RusotoFuture<ConfirmConnectionResponse, ConfirmConnectionError> {
        let mut request = SignedRequest::new("POST", "directconnect", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "OvertureService.ConfirmConnection");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<ConfirmConnectionResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(ConfirmConnectionError::from_response(response))),
                )
            }
        })
    }

    /// <p>Accepts ownership of a private virtual interface created by another AWS account.</p> <p>After the virtual interface owner makes this call, the virtual interface is created and attached to the specified virtual private gateway or Direct Connect gateway, and is made available to handle traffic.</p>
    fn confirm_private_virtual_interface(
        &self,
        input: ConfirmPrivateVirtualInterfaceRequest,
    ) -> RusotoFuture<ConfirmPrivateVirtualInterfaceResponse, ConfirmPrivateVirtualInterfaceError>
    {
        let mut request = SignedRequest::new("POST", "directconnect", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "OvertureService.ConfirmPrivateVirtualInterface",
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

                    serde_json::from_str::<ConfirmPrivateVirtualInterfaceResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(ConfirmPrivateVirtualInterfaceError::from_response(response))
                }))
            }
        })
    }

    /// <p>Accepts ownership of a public virtual interface created by another AWS account.</p> <p>After the virtual interface owner makes this call, the specified virtual interface is created and made available to handle traffic.</p>
    fn confirm_public_virtual_interface(
        &self,
        input: ConfirmPublicVirtualInterfaceRequest,
    ) -> RusotoFuture<ConfirmPublicVirtualInterfaceResponse, ConfirmPublicVirtualInterfaceError>
    {
        let mut request = SignedRequest::new("POST", "directconnect", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "OvertureService.ConfirmPublicVirtualInterface",
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

                    serde_json::from_str::<ConfirmPublicVirtualInterfaceResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(ConfirmPublicVirtualInterfaceError::from_response(response))
                }))
            }
        })
    }

    /// <p>Creates a BGP peer on the specified virtual interface.</p> <p>You must create a BGP peer for the corresponding address family (IPv4/IPv6) in order to access AWS resources that also use that address family.</p> <p>If logical redundancy is not supported by the connection, interconnect, or LAG, the BGP peer cannot be in the same address family as an existing BGP peer on the virtual interface.</p> <p>When creating a IPv6 BGP peer, omit the Amazon address and customer address. IPv6 addresses are automatically assigned from the Amazon pool of IPv6 addresses; you cannot specify custom IPv6 addresses.</p> <p>For a public virtual interface, the Autonomous System Number (ASN) must be private or already whitelisted for the virtual interface.</p>
    fn create_bgp_peer(
        &self,
        input: CreateBGPPeerRequest,
    ) -> RusotoFuture<CreateBGPPeerResponse, CreateBGPPeerError> {
        let mut request = SignedRequest::new("POST", "directconnect", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "OvertureService.CreateBGPPeer");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<CreateBGPPeerResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(CreateBGPPeerError::from_response(response))),
                )
            }
        })
    }

    /// <p>Creates a connection between a customer network and a specific AWS Direct Connect location.</p> <p>A connection links your internal network to an AWS Direct Connect location over a standard Ethernet fiber-optic cable. One end of the cable is connected to your router, the other to an AWS Direct Connect router.</p> <p>To find the locations for your Region, use <a>DescribeLocations</a>.</p> <p>You can automatically add the new connection to a link aggregation group (LAG) by specifying a LAG ID in the request. This ensures that the new connection is allocated on the same AWS Direct Connect endpoint that hosts the specified LAG. If there are no available ports on the endpoint, the request fails and no connection is created.</p>
    fn create_connection(
        &self,
        input: CreateConnectionRequest,
    ) -> RusotoFuture<Connection, CreateConnectionError> {
        let mut request = SignedRequest::new("POST", "directconnect", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "OvertureService.CreateConnection");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<Connection>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(CreateConnectionError::from_response(response))),
                )
            }
        })
    }

    /// <p>Creates a Direct Connect gateway, which is an intermediate object that enables you to connect a set of virtual interfaces and virtual private gateways. A Direct Connect gateway is global and visible in any AWS Region after it is created. The virtual interfaces and virtual private gateways that are connected through a Direct Connect gateway can be in different AWS Regions. This enables you to connect to a VPC in any Region, regardless of the Region in which the virtual interfaces are located, and pass traffic between them.</p>
    fn create_direct_connect_gateway(
        &self,
        input: CreateDirectConnectGatewayRequest,
    ) -> RusotoFuture<CreateDirectConnectGatewayResult, CreateDirectConnectGatewayError> {
        let mut request = SignedRequest::new("POST", "directconnect", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "OvertureService.CreateDirectConnectGateway");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<CreateDirectConnectGatewayResult>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(CreateDirectConnectGatewayError::from_response(response))
                }))
            }
        })
    }

    /// <p>Creates an association between a Direct Connect gateway and a virtual private gateway. The virtual private gateway must be attached to a VPC and must not be associated with another Direct Connect gateway.</p>
    fn create_direct_connect_gateway_association(
        &self,
        input: CreateDirectConnectGatewayAssociationRequest,
    ) -> RusotoFuture<
        CreateDirectConnectGatewayAssociationResult,
        CreateDirectConnectGatewayAssociationError,
    > {
        let mut request = SignedRequest::new("POST", "directconnect", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "OvertureService.CreateDirectConnectGatewayAssociation",
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

                    serde_json::from_str::<CreateDirectConnectGatewayAssociationResult>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(CreateDirectConnectGatewayAssociationError::from_response(
                        response,
                    ))
                }))
            }
        })
    }

    /// <p><p>Creates an interconnect between an AWS Direct Connect partner&#39;s network and a specific AWS Direct Connect location.</p> <p>An interconnect is a connection which is capable of hosting other connections. The partner can use an interconnect to provide sub-1Gbps AWS Direct Connect service to tier 2 customers who do not have their own connections. Like a standard connection, an interconnect links the partner&#39;s network to an AWS Direct Connect location over a standard Ethernet fiber-optic cable. One end is connected to the partner&#39;s router, the other to an AWS Direct Connect router.</p> <p>You can automatically add the new interconnect to a link aggregation group (LAG) by specifying a LAG ID in the request. This ensures that the new interconnect is allocated on the same AWS Direct Connect endpoint that hosts the specified LAG. If there are no available ports on the endpoint, the request fails and no interconnect is created.</p> <p>For each end customer, the AWS Direct Connect partner provisions a connection on their interconnect by calling <a>AllocateConnectionOnInterconnect</a>. The end customer can then connect to AWS resources by creating a virtual interface on their connection, using the VLAN assigned to them by the partner.</p> <note> <p>Intended for use by AWS Direct Connect partners only.</p> </note></p>
    fn create_interconnect(
        &self,
        input: CreateInterconnectRequest,
    ) -> RusotoFuture<Interconnect, CreateInterconnectError> {
        let mut request = SignedRequest::new("POST", "directconnect", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "OvertureService.CreateInterconnect");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<Interconnect>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(CreateInterconnectError::from_response(response))),
                )
            }
        })
    }

    /// <p>Creates a link aggregation group (LAG) with the specified number of bundled physical connections between the customer network and a specific AWS Direct Connect location. A LAG is a logical interface that uses the Link Aggregation Control Protocol (LACP) to aggregate multiple interfaces, enabling you to treat them as a single interface.</p> <p>All connections in a LAG must use the same bandwidth and must terminate at the same AWS Direct Connect endpoint.</p> <p>You can have up to 10 connections per LAG. Regardless of this limit, if you request more connections for the LAG than AWS Direct Connect can allocate on a single endpoint, no LAG is created.</p> <p>You can specify an existing physical connection or interconnect to include in the LAG (which counts towards the total number of connections). Doing so interrupts the current physical connection or hosted connections, and re-establishes them as a member of the LAG. The LAG will be created on the same AWS Direct Connect endpoint to which the connection terminates. Any virtual interfaces associated with the connection are automatically disassociated and re-associated with the LAG. The connection ID does not change.</p> <p>If the AWS account used to create a LAG is a registered AWS Direct Connect partner, the LAG is automatically enabled to host sub-connections. For a LAG owned by a partner, any associated virtual interfaces cannot be directly configured.</p>
    fn create_lag(&self, input: CreateLagRequest) -> RusotoFuture<Lag, CreateLagError> {
        let mut request = SignedRequest::new("POST", "directconnect", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "OvertureService.CreateLag");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<Lag>(String::from_utf8_lossy(body.as_ref()).as_ref())
                        .unwrap()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(CreateLagError::from_response(response))),
                )
            }
        })
    }

    /// <p>Creates a private virtual interface. A virtual interface is the VLAN that transports AWS Direct Connect traffic. A private virtual interface can be connected to either a Direct Connect gateway or a Virtual Private Gateway (VGW). Connecting the private virtual interface to a Direct Connect gateway enables the possibility for connecting to multiple VPCs, including VPCs in different AWS Regions. Connecting the private virtual interface to a VGW only provides access to a single VPC within the same Region.</p>
    fn create_private_virtual_interface(
        &self,
        input: CreatePrivateVirtualInterfaceRequest,
    ) -> RusotoFuture<VirtualInterface, CreatePrivateVirtualInterfaceError> {
        let mut request = SignedRequest::new("POST", "directconnect", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "OvertureService.CreatePrivateVirtualInterface",
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

                    serde_json::from_str::<VirtualInterface>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(CreatePrivateVirtualInterfaceError::from_response(response))
                }))
            }
        })
    }

    /// <p>Creates a public virtual interface. A virtual interface is the VLAN that transports AWS Direct Connect traffic. A public virtual interface supports sending traffic to public services of AWS such as Amazon S3.</p> <p>When creating an IPv6 public virtual interface (<code>addressFamily</code> is <code>ipv6</code>), leave the <code>customer</code> and <code>amazon</code> address fields blank to use auto-assigned IPv6 space. Custom IPv6 addresses are not supported.</p>
    fn create_public_virtual_interface(
        &self,
        input: CreatePublicVirtualInterfaceRequest,
    ) -> RusotoFuture<VirtualInterface, CreatePublicVirtualInterfaceError> {
        let mut request = SignedRequest::new("POST", "directconnect", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "OvertureService.CreatePublicVirtualInterface",
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

                    serde_json::from_str::<VirtualInterface>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(CreatePublicVirtualInterfaceError::from_response(response))
                }))
            }
        })
    }

    /// <p>Deletes the specified BGP peer on the specified virtual interface with the specified customer address and ASN.</p> <p>You cannot delete the last BGP peer from a virtual interface.</p>
    fn delete_bgp_peer(
        &self,
        input: DeleteBGPPeerRequest,
    ) -> RusotoFuture<DeleteBGPPeerResponse, DeleteBGPPeerError> {
        let mut request = SignedRequest::new("POST", "directconnect", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "OvertureService.DeleteBGPPeer");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<DeleteBGPPeerResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(DeleteBGPPeerError::from_response(response))),
                )
            }
        })
    }

    /// <p>Deletes the specified connection.</p> <p>Deleting a connection only stops the AWS Direct Connect port hour and data transfer charges. If you are partnering with any third parties to connect with the AWS Direct Connect location, you must cancel your service with them separately.</p>
    fn delete_connection(
        &self,
        input: DeleteConnectionRequest,
    ) -> RusotoFuture<Connection, DeleteConnectionError> {
        let mut request = SignedRequest::new("POST", "directconnect", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "OvertureService.DeleteConnection");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<Connection>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(DeleteConnectionError::from_response(response))),
                )
            }
        })
    }

    /// <p>Deletes the specified Direct Connect gateway. You must first delete all virtual interfaces that are attached to the Direct Connect gateway and disassociate all virtual private gateways that are associated with the Direct Connect gateway.</p>
    fn delete_direct_connect_gateway(
        &self,
        input: DeleteDirectConnectGatewayRequest,
    ) -> RusotoFuture<DeleteDirectConnectGatewayResult, DeleteDirectConnectGatewayError> {
        let mut request = SignedRequest::new("POST", "directconnect", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "OvertureService.DeleteDirectConnectGateway");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<DeleteDirectConnectGatewayResult>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(DeleteDirectConnectGatewayError::from_response(response))
                }))
            }
        })
    }

    /// <p>Deletes the association between the specified Direct Connect gateway and virtual private gateway.</p>
    fn delete_direct_connect_gateway_association(
        &self,
        input: DeleteDirectConnectGatewayAssociationRequest,
    ) -> RusotoFuture<
        DeleteDirectConnectGatewayAssociationResult,
        DeleteDirectConnectGatewayAssociationError,
    > {
        let mut request = SignedRequest::new("POST", "directconnect", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "OvertureService.DeleteDirectConnectGatewayAssociation",
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

                    serde_json::from_str::<DeleteDirectConnectGatewayAssociationResult>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(DeleteDirectConnectGatewayAssociationError::from_response(
                        response,
                    ))
                }))
            }
        })
    }

    /// <p><p>Deletes the specified interconnect.</p> <note> <p>Intended for use by AWS Direct Connect partners only.</p> </note></p>
    fn delete_interconnect(
        &self,
        input: DeleteInterconnectRequest,
    ) -> RusotoFuture<DeleteInterconnectResponse, DeleteInterconnectError> {
        let mut request = SignedRequest::new("POST", "directconnect", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "OvertureService.DeleteInterconnect");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<DeleteInterconnectResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(DeleteInterconnectError::from_response(response))),
                )
            }
        })
    }

    /// <p>Deletes the specified link aggregation group (LAG). You cannot delete a LAG if it has active virtual interfaces or hosted connections.</p>
    fn delete_lag(&self, input: DeleteLagRequest) -> RusotoFuture<Lag, DeleteLagError> {
        let mut request = SignedRequest::new("POST", "directconnect", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "OvertureService.DeleteLag");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<Lag>(String::from_utf8_lossy(body.as_ref()).as_ref())
                        .unwrap()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(DeleteLagError::from_response(response))),
                )
            }
        })
    }

    /// <p>Deletes a virtual interface.</p>
    fn delete_virtual_interface(
        &self,
        input: DeleteVirtualInterfaceRequest,
    ) -> RusotoFuture<DeleteVirtualInterfaceResponse, DeleteVirtualInterfaceError> {
        let mut request = SignedRequest::new("POST", "directconnect", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "OvertureService.DeleteVirtualInterface");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<DeleteVirtualInterfaceResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(DeleteVirtualInterfaceError::from_response(response))
                    }),
                )
            }
        })
    }

    /// <p>Deprecated. Use <a>DescribeLoa</a> instead.</p> <p>Gets the LOA-CFA for a connection.</p> <p>The Letter of Authorization - Connecting Facility Assignment (LOA-CFA) is a document that your APN partner or service provider uses when establishing your cross connect to AWS at the colocation facility. For more information, see <a href="https://docs.aws.amazon.com/directconnect/latest/UserGuide/Colocation.html">Requesting Cross Connects at AWS Direct Connect Locations</a> in the <i>AWS Direct Connect User Guide</i>.</p>
    fn describe_connection_loa(
        &self,
        input: DescribeConnectionLoaRequest,
    ) -> RusotoFuture<DescribeConnectionLoaResponse, DescribeConnectionLoaError> {
        let mut request = SignedRequest::new("POST", "directconnect", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "OvertureService.DescribeConnectionLoa");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<DescribeConnectionLoaResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(DescribeConnectionLoaError::from_response(response))
                    }),
                )
            }
        })
    }

    /// <p>Displays the specified connection or all connections in this Region.</p>
    fn describe_connections(
        &self,
        input: DescribeConnectionsRequest,
    ) -> RusotoFuture<Connections, DescribeConnectionsError> {
        let mut request = SignedRequest::new("POST", "directconnect", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "OvertureService.DescribeConnections");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<Connections>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(DescribeConnectionsError::from_response(response))
                    }),
                )
            }
        })
    }

    /// <p><p>Deprecated. Use <a>DescribeHostedConnections</a> instead.</p> <p>Lists the connections that have been provisioned on the specified interconnect.</p> <note> <p>Intended for use by AWS Direct Connect partners only.</p> </note></p>
    fn describe_connections_on_interconnect(
        &self,
        input: DescribeConnectionsOnInterconnectRequest,
    ) -> RusotoFuture<Connections, DescribeConnectionsOnInterconnectError> {
        let mut request = SignedRequest::new("POST", "directconnect", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "OvertureService.DescribeConnectionsOnInterconnect",
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

                    serde_json::from_str::<Connections>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(DescribeConnectionsOnInterconnectError::from_response(
                        response,
                    ))
                }))
            }
        })
    }

    /// <p>Lists the associations between your Direct Connect gateways and virtual private gateways. You must specify a Direct Connect gateway, a virtual private gateway, or both. If you specify a Direct Connect gateway, the response contains all virtual private gateways associated with the Direct Connect gateway. If you specify a virtual private gateway, the response contains all Direct Connect gateways associated with the virtual private gateway. If you specify both, the response contains the association between the Direct Connect gateway and the virtual private gateway.</p>
    fn describe_direct_connect_gateway_associations(
        &self,
        input: DescribeDirectConnectGatewayAssociationsRequest,
    ) -> RusotoFuture<
        DescribeDirectConnectGatewayAssociationsResult,
        DescribeDirectConnectGatewayAssociationsError,
    > {
        let mut request = SignedRequest::new("POST", "directconnect", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "OvertureService.DescribeDirectConnectGatewayAssociations",
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

                    serde_json::from_str::<DescribeDirectConnectGatewayAssociationsResult>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(DescribeDirectConnectGatewayAssociationsError::from_response(response))
                }))
            }
        })
    }

    /// <p>Lists the attachments between your Direct Connect gateways and virtual interfaces. You must specify a Direct Connect gateway, a virtual interface, or both. If you specify a Direct Connect gateway, the response contains all virtual interfaces attached to the Direct Connect gateway. If you specify a virtual interface, the response contains all Direct Connect gateways attached to the virtual interface. If you specify both, the response contains the attachment between the Direct Connect gateway and the virtual interface.</p>
    fn describe_direct_connect_gateway_attachments(
        &self,
        input: DescribeDirectConnectGatewayAttachmentsRequest,
    ) -> RusotoFuture<
        DescribeDirectConnectGatewayAttachmentsResult,
        DescribeDirectConnectGatewayAttachmentsError,
    > {
        let mut request = SignedRequest::new("POST", "directconnect", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "OvertureService.DescribeDirectConnectGatewayAttachments",
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

                    serde_json::from_str::<DescribeDirectConnectGatewayAttachmentsResult>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(DescribeDirectConnectGatewayAttachmentsError::from_response(
                        response,
                    ))
                }))
            }
        })
    }

    /// <p>Lists all your Direct Connect gateways or only the specified Direct Connect gateway. Deleted Direct Connect gateways are not returned.</p>
    fn describe_direct_connect_gateways(
        &self,
        input: DescribeDirectConnectGatewaysRequest,
    ) -> RusotoFuture<DescribeDirectConnectGatewaysResult, DescribeDirectConnectGatewaysError> {
        let mut request = SignedRequest::new("POST", "directconnect", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "OvertureService.DescribeDirectConnectGateways",
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

                    serde_json::from_str::<DescribeDirectConnectGatewaysResult>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(DescribeDirectConnectGatewaysError::from_response(response))
                }))
            }
        })
    }

    /// <p><p>Lists the hosted connections that have been provisioned on the specified interconnect or link aggregation group (LAG).</p> <note> <p>Intended for use by AWS Direct Connect partners only.</p> </note></p>
    fn describe_hosted_connections(
        &self,
        input: DescribeHostedConnectionsRequest,
    ) -> RusotoFuture<Connections, DescribeHostedConnectionsError> {
        let mut request = SignedRequest::new("POST", "directconnect", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "OvertureService.DescribeHostedConnections");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<Connections>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(DescribeHostedConnectionsError::from_response(response))
                }))
            }
        })
    }

    /// <p>Deprecated. Use <a>DescribeLoa</a> instead.</p> <p>Gets the LOA-CFA for the specified interconnect.</p> <p>The Letter of Authorization - Connecting Facility Assignment (LOA-CFA) is a document that is used when establishing your cross connect to AWS at the colocation facility. For more information, see <a href="https://docs.aws.amazon.com/directconnect/latest/UserGuide/Colocation.html">Requesting Cross Connects at AWS Direct Connect Locations</a> in the <i>AWS Direct Connect User Guide</i>.</p>
    fn describe_interconnect_loa(
        &self,
        input: DescribeInterconnectLoaRequest,
    ) -> RusotoFuture<DescribeInterconnectLoaResponse, DescribeInterconnectLoaError> {
        let mut request = SignedRequest::new("POST", "directconnect", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "OvertureService.DescribeInterconnectLoa");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<DescribeInterconnectLoaResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(DescribeInterconnectLoaError::from_response(response))
                }))
            }
        })
    }

    /// <p>Lists the interconnects owned by the AWS account or only the specified interconnect.</p>
    fn describe_interconnects(
        &self,
        input: DescribeInterconnectsRequest,
    ) -> RusotoFuture<Interconnects, DescribeInterconnectsError> {
        let mut request = SignedRequest::new("POST", "directconnect", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "OvertureService.DescribeInterconnects");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<Interconnects>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(DescribeInterconnectsError::from_response(response))
                    }),
                )
            }
        })
    }

    /// <p>Describes all your link aggregation groups (LAG) or the specified LAG.</p>
    fn describe_lags(&self, input: DescribeLagsRequest) -> RusotoFuture<Lags, DescribeLagsError> {
        let mut request = SignedRequest::new("POST", "directconnect", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "OvertureService.DescribeLags");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<Lags>(String::from_utf8_lossy(body.as_ref()).as_ref())
                        .unwrap()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(DescribeLagsError::from_response(response))),
                )
            }
        })
    }

    /// <p>Gets the LOA-CFA for a connection, interconnect, or link aggregation group (LAG).</p> <p>The Letter of Authorization - Connecting Facility Assignment (LOA-CFA) is a document that is used when establishing your cross connect to AWS at the colocation facility. For more information, see <a href="https://docs.aws.amazon.com/directconnect/latest/UserGuide/Colocation.html">Requesting Cross Connects at AWS Direct Connect Locations</a> in the <i>AWS Direct Connect User Guide</i>.</p>
    fn describe_loa(&self, input: DescribeLoaRequest) -> RusotoFuture<Loa, DescribeLoaError> {
        let mut request = SignedRequest::new("POST", "directconnect", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "OvertureService.DescribeLoa");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<Loa>(String::from_utf8_lossy(body.as_ref()).as_ref())
                        .unwrap()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(DescribeLoaError::from_response(response))),
                )
            }
        })
    }

    /// <p>Lists the AWS Direct Connect locations in the current AWS Region. These are the locations that can be selected when calling <a>CreateConnection</a> or <a>CreateInterconnect</a>.</p>
    fn describe_locations(&self) -> RusotoFuture<Locations, DescribeLocationsError> {
        let mut request = SignedRequest::new("POST", "directconnect", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "OvertureService.DescribeLocations");
        request.set_payload(Some(b"{}".to_vec()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<Locations>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(DescribeLocationsError::from_response(response))),
                )
            }
        })
    }

    /// <p>Describes the tags associated with the specified AWS Direct Connect resources.</p>
    fn describe_tags(
        &self,
        input: DescribeTagsRequest,
    ) -> RusotoFuture<DescribeTagsResponse, DescribeTagsError> {
        let mut request = SignedRequest::new("POST", "directconnect", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "OvertureService.DescribeTags");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<DescribeTagsResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(DescribeTagsError::from_response(response))),
                )
            }
        })
    }

    /// <p>Lists the virtual private gateways owned by the AWS account.</p> <p>You can create one or more AWS Direct Connect private virtual interfaces linked to a virtual private gateway.</p>
    fn describe_virtual_gateways(
        &self,
    ) -> RusotoFuture<VirtualGateways, DescribeVirtualGatewaysError> {
        let mut request = SignedRequest::new("POST", "directconnect", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "OvertureService.DescribeVirtualGateways");
        request.set_payload(Some(b"{}".to_vec()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<VirtualGateways>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(DescribeVirtualGatewaysError::from_response(response))
                }))
            }
        })
    }

    /// <p>Displays all virtual interfaces for an AWS account. Virtual interfaces deleted fewer than 15 minutes before you make the request are also returned. If you specify a connection ID, only the virtual interfaces associated with the connection are returned. If you specify a virtual interface ID, then only a single virtual interface is returned.</p> <p>A virtual interface (VLAN) transmits the traffic between the AWS Direct Connect location and the customer network.</p>
    fn describe_virtual_interfaces(
        &self,
        input: DescribeVirtualInterfacesRequest,
    ) -> RusotoFuture<VirtualInterfaces, DescribeVirtualInterfacesError> {
        let mut request = SignedRequest::new("POST", "directconnect", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "OvertureService.DescribeVirtualInterfaces");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<VirtualInterfaces>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(DescribeVirtualInterfacesError::from_response(response))
                }))
            }
        })
    }

    /// <p>Disassociates a connection from a link aggregation group (LAG). The connection is interrupted and re-established as a standalone connection (the connection is not deleted; to delete the connection, use the <a>DeleteConnection</a> request). If the LAG has associated virtual interfaces or hosted connections, they remain associated with the LAG. A disassociated connection owned by an AWS Direct Connect partner is automatically converted to an interconnect.</p> <p>If disassociating the connection would cause the LAG to fall below its setting for minimum number of operational connections, the request fails, except when it's the last member of the LAG. If all connections are disassociated, the LAG continues to exist as an empty LAG with no physical connections. </p>
    fn disassociate_connection_from_lag(
        &self,
        input: DisassociateConnectionFromLagRequest,
    ) -> RusotoFuture<Connection, DisassociateConnectionFromLagError> {
        let mut request = SignedRequest::new("POST", "directconnect", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "OvertureService.DisassociateConnectionFromLag",
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

                    serde_json::from_str::<Connection>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(DisassociateConnectionFromLagError::from_response(response))
                }))
            }
        })
    }

    /// <p>Adds the specified tags to the specified AWS Direct Connect resource. Each resource can have a maximum of 50 tags.</p> <p>Each tag consists of a key and an optional value. If a tag with the same key is already associated with the resource, this action updates its value.</p>
    fn tag_resource(
        &self,
        input: TagResourceRequest,
    ) -> RusotoFuture<TagResourceResponse, TagResourceError> {
        let mut request = SignedRequest::new("POST", "directconnect", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "OvertureService.TagResource");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<TagResourceResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(TagResourceError::from_response(response))),
                )
            }
        })
    }

    /// <p>Removes one or more tags from the specified AWS Direct Connect resource.</p>
    fn untag_resource(
        &self,
        input: UntagResourceRequest,
    ) -> RusotoFuture<UntagResourceResponse, UntagResourceError> {
        let mut request = SignedRequest::new("POST", "directconnect", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "OvertureService.UntagResource");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<UntagResourceResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(UntagResourceError::from_response(response))),
                )
            }
        })
    }

    /// <p>Updates the attributes of the specified link aggregation group (LAG).</p> <p>You can update the following attributes:</p> <ul> <li> <p>The name of the LAG.</p> </li> <li> <p>The value for the minimum number of connections that must be operational for the LAG itself to be operational. </p> </li> </ul> <p>When you create a LAG, the default value for the minimum number of operational connections is zero (0). If you update this value and the number of operational connections falls below the specified value, the LAG automatically goes down to avoid over-utilization of the remaining connections. Adjust this value with care, as it could force the LAG down if it is set higher than the current number of operational connections.</p>
    fn update_lag(&self, input: UpdateLagRequest) -> RusotoFuture<Lag, UpdateLagError> {
        let mut request = SignedRequest::new("POST", "directconnect", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "OvertureService.UpdateLag");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<Lag>(String::from_utf8_lossy(body.as_ref()).as_ref())
                        .unwrap()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(UpdateLagError::from_response(response))),
                )
            }
        })
    }

    /// <p>Updates the specified attributes of the specified virtual private interface.</p> <p>Setting the MTU of a virtual interface to 9001 (jumbo frames) can cause an update to the underlying physical connection if it wasn't updated to support jumbo frames. Updating the connection disrupts network connectivity for all virtual interfaces associated with the connection for up to 30 seconds. To check whether your connection supports jumbo frames, call <a>DescribeConnections</a>. To check whether your virtual interface supports jumbo frames, call <a>DescribeVirtualInterfaces</a>.</p>
    fn update_virtual_interface_attributes(
        &self,
        input: UpdateVirtualInterfaceAttributesRequest,
    ) -> RusotoFuture<VirtualInterface, UpdateVirtualInterfaceAttributesError> {
        let mut request = SignedRequest::new("POST", "directconnect", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "OvertureService.UpdateVirtualInterfaceAttributes",
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

                    serde_json::from_str::<VirtualInterface>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(UpdateVirtualInterfaceAttributesError::from_response(
                        response,
                    ))
                }))
            }
        })
    }
}

#[cfg(test)]
mod protocol_tests {}
