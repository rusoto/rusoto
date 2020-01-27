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
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct AcceptDirectConnectGatewayAssociationProposalRequest {
    /// <p>The ID of the AWS account that owns the virtual private gateway or transit gateway.</p>
    #[serde(rename = "associatedGatewayOwnerAccount")]
    pub associated_gateway_owner_account: String,
    /// <p>The ID of the Direct Connect gateway.</p>
    #[serde(rename = "directConnectGatewayId")]
    pub direct_connect_gateway_id: String,
    /// <p>Overrides the Amazon VPC prefixes advertised to the Direct Connect gateway.</p> <p>For information about how to set the prefixes, see <a href="https://docs.aws.amazon.com/directconnect/latest/UserGuide/multi-account-associate-vgw.html#allowed-prefixes">Allowed Prefixes</a> in the <i>AWS Direct Connect User Guide</i>.</p>
    #[serde(rename = "overrideAllowedPrefixesToDirectConnectGateway")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub override_allowed_prefixes_to_direct_connect_gateway: Option<Vec<RouteFilterPrefix>>,
    /// <p>The ID of the request proposal.</p>
    #[serde(rename = "proposalId")]
    pub proposal_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct AcceptDirectConnectGatewayAssociationProposalResult {
    #[serde(rename = "directConnectGatewayAssociation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub direct_connect_gateway_association: Option<DirectConnectGatewayAssociation>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct AllocateConnectionOnInterconnectRequest {
    /// <p>The bandwidth of the connection. The possible values are 50Mbps, 100Mbps, 200Mbps, 300Mbps, 400Mbps, 500Mbps, 1Gbps, 2Gbps, 5Gbps, and 10Gbps. Note that only those AWS Direct Connect Partners who have met specific requirements are allowed to create a 1Gbps, 2Gbps, 5Gbps or 10Gbps hosted connection.</p>
    #[serde(rename = "bandwidth")]
    pub bandwidth: String,
    /// <p>The name of the provisioned connection.</p>
    #[serde(rename = "connectionName")]
    pub connection_name: String,
    /// <p>The ID of the interconnect on which the connection will be provisioned.</p>
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
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct AllocateHostedConnectionRequest {
    /// <p>The bandwidth of the connection. The possible values are 50Mbps, 100Mbps, 200Mbps, 300Mbps, 400Mbps, 500Mbps, 1Gbps, 2Gbps, 5Gbps, and 10Gbps. Note that only those AWS Direct Connect Partners who have met specific requirements are allowed to create a 1Gbps, 2Gbps, 5Gbps or 10Gbps hosted connection. </p>
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
    /// <p>The tags associated with the connection.</p>
    #[serde(rename = "tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
    /// <p>The dedicated VLAN provisioned to the hosted connection.</p>
    #[serde(rename = "vlan")]
    pub vlan: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct AllocateTransitVirtualInterfaceRequest {
    /// <p>The ID of the connection on which the transit virtual interface is provisioned.</p>
    #[serde(rename = "connectionId")]
    pub connection_id: String,
    /// <p>Information about the transit virtual interface.</p>
    #[serde(rename = "newTransitVirtualInterfaceAllocation")]
    pub new_transit_virtual_interface_allocation: NewTransitVirtualInterfaceAllocation,
    /// <p>The ID of the AWS account that owns the transit virtual interface.</p>
    #[serde(rename = "ownerAccount")]
    pub owner_account: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct AllocateTransitVirtualInterfaceResult {
    #[serde(rename = "virtualInterface")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub virtual_interface: Option<VirtualInterface>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct AssociateConnectionWithLagRequest {
    /// <p>The ID of the connection.</p>
    #[serde(rename = "connectionId")]
    pub connection_id: String,
    /// <p>The ID of the LAG with which to associate the connection.</p>
    #[serde(rename = "lagId")]
    pub lag_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct AssociateHostedConnectionRequest {
    /// <p>The ID of the hosted connection.</p>
    #[serde(rename = "connectionId")]
    pub connection_id: String,
    /// <p>The ID of the interconnect or the LAG.</p>
    #[serde(rename = "parentConnectionId")]
    pub parent_connection_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct AssociateVirtualInterfaceRequest {
    /// <p>The ID of the LAG or connection.</p>
    #[serde(rename = "connectionId")]
    pub connection_id: String,
    /// <p>The ID of the virtual interface.</p>
    #[serde(rename = "virtualInterfaceId")]
    pub virtual_interface_id: String,
}

/// <p>Information about the associated gateway.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct AssociatedGateway {
    /// <p>The ID of the associated gateway.</p>
    #[serde(rename = "id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// <p>The ID of the AWS account that owns the associated virtual private gateway or transit gateway.</p>
    #[serde(rename = "ownerAccount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner_account: Option<String>,
    /// <p>The Region where the associated gateway is located.</p>
    #[serde(rename = "region")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
    /// <p>The type of associated gateway.</p>
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}

/// <p>Information about a BGP peer.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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
    /// <p>The authentication key for BGP configuration. This string has a minimum length of 6 characters and and a maximun lenth of 80 characters.</p>
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
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ConfirmConnectionRequest {
    /// <p>The ID of the hosted connection.</p>
    #[serde(rename = "connectionId")]
    pub connection_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ConfirmConnectionResponse {
    /// <p><p>The state of the connection. The following are the possible values:</p> <ul> <li> <p> <code>ordering</code>: The initial state of a hosted connection provisioned on an interconnect. The connection stays in the ordering state until the owner of the hosted connection confirms or declines the connection order.</p> </li> <li> <p> <code>requested</code>: The initial state of a standard connection. The connection stays in the requested state until the Letter of Authorization (LOA) is sent to the customer.</p> </li> <li> <p> <code>pending</code>: The connection has been approved and is being initialized.</p> </li> <li> <p> <code>available</code>: The network link is up and the connection is ready for use.</p> </li> <li> <p> <code>down</code>: The network link is down.</p> </li> <li> <p> <code>deleting</code>: The connection is being deleted.</p> </li> <li> <p> <code>deleted</code>: The connection has been deleted.</p> </li> <li> <p> <code>rejected</code>: A hosted connection in the <code>ordering</code> state enters the <code>rejected</code> state if it is deleted by the customer.</p> </li> <li> <p> <code>unknown</code>: The state of the connection is not available.</p> </li> </ul></p>
    #[serde(rename = "connectionState")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connection_state: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ConfirmPrivateVirtualInterfaceResponse {
    /// <p><p>The state of the virtual interface. The following are the possible values:</p> <ul> <li> <p> <code>confirming</code>: The creation of the virtual interface is pending confirmation from the virtual interface owner. If the owner of the virtual interface is different from the owner of the connection on which it is provisioned, then the virtual interface will remain in this state until it is confirmed by the virtual interface owner.</p> </li> <li> <p> <code>verifying</code>: This state only applies to public virtual interfaces. Each public virtual interface needs validation before the virtual interface can be created.</p> </li> <li> <p> <code>pending</code>: A virtual interface is in this state from the time that it is created until the virtual interface is ready to forward traffic.</p> </li> <li> <p> <code>available</code>: A virtual interface that is able to forward traffic.</p> </li> <li> <p> <code>down</code>: A virtual interface that is BGP down.</p> </li> <li> <p> <code>deleting</code>: A virtual interface is in this state immediately after calling <a>DeleteVirtualInterface</a> until it can no longer forward traffic.</p> </li> <li> <p> <code>deleted</code>: A virtual interface that cannot forward traffic.</p> </li> <li> <p> <code>rejected</code>: The virtual interface owner has declined creation of the virtual interface. If a virtual interface in the <code>Confirming</code> state is deleted by the virtual interface owner, the virtual interface enters the <code>Rejected</code> state.</p> </li> <li> <p> <code>unknown</code>: The state of the virtual interface is not available.</p> </li> </ul></p>
    #[serde(rename = "virtualInterfaceState")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub virtual_interface_state: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ConfirmPublicVirtualInterfaceRequest {
    /// <p>The ID of the virtual interface.</p>
    #[serde(rename = "virtualInterfaceId")]
    pub virtual_interface_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ConfirmPublicVirtualInterfaceResponse {
    /// <p><p>The state of the virtual interface. The following are the possible values:</p> <ul> <li> <p> <code>confirming</code>: The creation of the virtual interface is pending confirmation from the virtual interface owner. If the owner of the virtual interface is different from the owner of the connection on which it is provisioned, then the virtual interface will remain in this state until it is confirmed by the virtual interface owner.</p> </li> <li> <p> <code>verifying</code>: This state only applies to public virtual interfaces. Each public virtual interface needs validation before the virtual interface can be created.</p> </li> <li> <p> <code>pending</code>: A virtual interface is in this state from the time that it is created until the virtual interface is ready to forward traffic.</p> </li> <li> <p> <code>available</code>: A virtual interface that is able to forward traffic.</p> </li> <li> <p> <code>down</code>: A virtual interface that is BGP down.</p> </li> <li> <p> <code>deleting</code>: A virtual interface is in this state immediately after calling <a>DeleteVirtualInterface</a> until it can no longer forward traffic.</p> </li> <li> <p> <code>deleted</code>: A virtual interface that cannot forward traffic.</p> </li> <li> <p> <code>rejected</code>: The virtual interface owner has declined creation of the virtual interface. If a virtual interface in the <code>Confirming</code> state is deleted by the virtual interface owner, the virtual interface enters the <code>Rejected</code> state.</p> </li> <li> <p> <code>unknown</code>: The state of the virtual interface is not available.</p> </li> </ul></p>
    #[serde(rename = "virtualInterfaceState")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub virtual_interface_state: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ConfirmTransitVirtualInterfaceRequest {
    /// <p>The ID of the Direct Connect gateway.</p>
    #[serde(rename = "directConnectGatewayId")]
    pub direct_connect_gateway_id: String,
    /// <p>The ID of the virtual interface.</p>
    #[serde(rename = "virtualInterfaceId")]
    pub virtual_interface_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ConfirmTransitVirtualInterfaceResponse {
    /// <p><p>The state of the virtual interface. The following are the possible values:</p> <ul> <li> <p> <code>confirming</code>: The creation of the virtual interface is pending confirmation from the virtual interface owner. If the owner of the virtual interface is different from the owner of the connection on which it is provisioned, then the virtual interface will remain in this state until it is confirmed by the virtual interface owner.</p> </li> <li> <p> <code>verifying</code>: This state only applies to public virtual interfaces. Each public virtual interface needs validation before the virtual interface can be created.</p> </li> <li> <p> <code>pending</code>: A virtual interface is in this state from the time that it is created until the virtual interface is ready to forward traffic.</p> </li> <li> <p> <code>available</code>: A virtual interface that is able to forward traffic.</p> </li> <li> <p> <code>down</code>: A virtual interface that is BGP down.</p> </li> <li> <p> <code>deleting</code>: A virtual interface is in this state immediately after calling <a>DeleteVirtualInterface</a> until it can no longer forward traffic.</p> </li> <li> <p> <code>deleted</code>: A virtual interface that cannot forward traffic.</p> </li> <li> <p> <code>rejected</code>: The virtual interface owner has declined creation of the virtual interface. If a virtual interface in the <code>Confirming</code> state is deleted by the virtual interface owner, the virtual interface enters the <code>Rejected</code> state.</p> </li> <li> <p> <code>unknown</code>: The state of the virtual interface is not available.</p> </li> </ul></p>
    #[serde(rename = "virtualInterfaceState")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub virtual_interface_state: Option<String>,
}

/// <p>Information about an AWS Direct Connect connection.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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
    /// <p>The name of the service provider associated with the connection.</p>
    #[serde(rename = "providerName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider_name: Option<String>,
    /// <p>The AWS Region where the connection is located.</p>
    #[serde(rename = "region")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
    /// <p>The tags associated with the connection.</p>
    #[serde(rename = "tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
    /// <p>The ID of the VLAN.</p>
    #[serde(rename = "vlan")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vlan: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct Connections {
    /// <p>The connections.</p>
    #[serde(rename = "connections")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connections: Option<Vec<Connection>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateBGPPeerResponse {
    /// <p>The virtual interface.</p>
    #[serde(rename = "virtualInterface")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub virtual_interface: Option<VirtualInterface>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
    /// <p>The name of the service provider associated with the requested connection.</p>
    #[serde(rename = "providerName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider_name: Option<String>,
    /// <p>The tags to associate with the lag.</p>
    #[serde(rename = "tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateDirectConnectGatewayAssociationProposalRequest {
    /// <p>The Amazon VPC prefixes to advertise to the Direct Connect gateway.</p>
    #[serde(rename = "addAllowedPrefixesToDirectConnectGateway")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub add_allowed_prefixes_to_direct_connect_gateway: Option<Vec<RouteFilterPrefix>>,
    /// <p>The ID of the Direct Connect gateway.</p>
    #[serde(rename = "directConnectGatewayId")]
    pub direct_connect_gateway_id: String,
    /// <p>The ID of the AWS account that owns the Direct Connect gateway.</p>
    #[serde(rename = "directConnectGatewayOwnerAccount")]
    pub direct_connect_gateway_owner_account: String,
    /// <p>The ID of the virtual private gateway or transit gateway.</p>
    #[serde(rename = "gatewayId")]
    pub gateway_id: String,
    /// <p>The Amazon VPC prefixes to no longer advertise to the Direct Connect gateway.</p>
    #[serde(rename = "removeAllowedPrefixesToDirectConnectGateway")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remove_allowed_prefixes_to_direct_connect_gateway: Option<Vec<RouteFilterPrefix>>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateDirectConnectGatewayAssociationProposalResult {
    /// <p>Information about the Direct Connect gateway proposal.</p>
    #[serde(rename = "directConnectGatewayAssociationProposal")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub direct_connect_gateway_association_proposal:
        Option<DirectConnectGatewayAssociationProposal>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateDirectConnectGatewayAssociationRequest {
    /// <p>The Amazon VPC prefixes to advertise to the Direct Connect gateway</p> <p>This parameter is required when you create an association to a transit gateway.</p> <p>For information about how to set the prefixes, see <a href="https://docs.aws.amazon.com/directconnect/latest/UserGuide/multi-account-associate-vgw.html#allowed-prefixes">Allowed Prefixes</a> in the <i>AWS Direct Connect User Guide</i>.</p>
    #[serde(rename = "addAllowedPrefixesToDirectConnectGateway")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub add_allowed_prefixes_to_direct_connect_gateway: Option<Vec<RouteFilterPrefix>>,
    /// <p>The ID of the Direct Connect gateway.</p>
    #[serde(rename = "directConnectGatewayId")]
    pub direct_connect_gateway_id: String,
    /// <p>The ID of the virtual private gateway or transit gateway.</p>
    #[serde(rename = "gatewayId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gateway_id: Option<String>,
    /// <p>The ID of the virtual private gateway.</p>
    #[serde(rename = "virtualGatewayId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub virtual_gateway_id: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateDirectConnectGatewayAssociationResult {
    /// <p>The association to be created.</p>
    #[serde(rename = "directConnectGatewayAssociation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub direct_connect_gateway_association: Option<DirectConnectGatewayAssociation>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateDirectConnectGatewayResult {
    /// <p>The Direct Connect gateway.</p>
    #[serde(rename = "directConnectGateway")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub direct_connect_gateway: Option<DirectConnectGateway>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
    /// <p>The name of the service provider associated with the interconnect.</p>
    #[serde(rename = "providerName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider_name: Option<String>,
    /// <p>The tags to associate with the interconnect.</p>
    #[serde(rename = "tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateLagRequest {
    /// <p>The tags to associate with the automtically created LAGs.</p>
    #[serde(rename = "childConnectionTags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub child_connection_tags: Option<Vec<Tag>>,
    /// <p>The ID of an existing connection to migrate to the LAG.</p>
    #[serde(rename = "connectionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connection_id: Option<String>,
    /// <p>The bandwidth of the individual physical connections bundled by the LAG. The possible values are 50Mbps, 100Mbps, 200Mbps, 300Mbps, 400Mbps, 500Mbps, 1Gbps, 2Gbps, 5Gbps, and 10Gbps. </p>
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
    /// <p>The name of the service provider associated with the LAG.</p>
    #[serde(rename = "providerName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider_name: Option<String>,
    /// <p>The tags to associate with the LAG.</p>
    #[serde(rename = "tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreatePrivateVirtualInterfaceRequest {
    /// <p>The ID of the connection.</p>
    #[serde(rename = "connectionId")]
    pub connection_id: String,
    /// <p>Information about the private virtual interface.</p>
    #[serde(rename = "newPrivateVirtualInterface")]
    pub new_private_virtual_interface: NewPrivateVirtualInterface,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreatePublicVirtualInterfaceRequest {
    /// <p>The ID of the connection.</p>
    #[serde(rename = "connectionId")]
    pub connection_id: String,
    /// <p>Information about the public virtual interface.</p>
    #[serde(rename = "newPublicVirtualInterface")]
    pub new_public_virtual_interface: NewPublicVirtualInterface,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateTransitVirtualInterfaceRequest {
    /// <p>The ID of the connection.</p>
    #[serde(rename = "connectionId")]
    pub connection_id: String,
    /// <p>Information about the transit virtual interface.</p>
    #[serde(rename = "newTransitVirtualInterface")]
    pub new_transit_virtual_interface: NewTransitVirtualInterface,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateTransitVirtualInterfaceResult {
    #[serde(rename = "virtualInterface")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub virtual_interface: Option<VirtualInterface>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DeleteBGPPeerResponse {
    /// <p>The virtual interface.</p>
    #[serde(rename = "virtualInterface")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub virtual_interface: Option<VirtualInterface>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteConnectionRequest {
    /// <p>The ID of the connection.</p>
    #[serde(rename = "connectionId")]
    pub connection_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteDirectConnectGatewayAssociationProposalRequest {
    /// <p>The ID of the proposal.</p>
    #[serde(rename = "proposalId")]
    pub proposal_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DeleteDirectConnectGatewayAssociationProposalResult {
    /// <p>The ID of the associated gateway.</p>
    #[serde(rename = "directConnectGatewayAssociationProposal")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub direct_connect_gateway_association_proposal:
        Option<DirectConnectGatewayAssociationProposal>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteDirectConnectGatewayAssociationRequest {
    /// <p>The ID of the Direct Connect gateway association.</p>
    #[serde(rename = "associationId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub association_id: Option<String>,
    /// <p>The ID of the Direct Connect gateway.</p>
    #[serde(rename = "directConnectGatewayId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub direct_connect_gateway_id: Option<String>,
    /// <p>The ID of the virtual private gateway.</p>
    #[serde(rename = "virtualGatewayId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub virtual_gateway_id: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DeleteDirectConnectGatewayAssociationResult {
    /// <p>Information about the deleted association.</p>
    #[serde(rename = "directConnectGatewayAssociation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub direct_connect_gateway_association: Option<DirectConnectGatewayAssociation>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteDirectConnectGatewayRequest {
    /// <p>The ID of the Direct Connect gateway.</p>
    #[serde(rename = "directConnectGatewayId")]
    pub direct_connect_gateway_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DeleteDirectConnectGatewayResult {
    /// <p>The Direct Connect gateway.</p>
    #[serde(rename = "directConnectGateway")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub direct_connect_gateway: Option<DirectConnectGateway>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteInterconnectRequest {
    /// <p>The ID of the interconnect.</p>
    #[serde(rename = "interconnectId")]
    pub interconnect_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DeleteInterconnectResponse {
    /// <p><p>The state of the interconnect. The following are the possible values:</p> <ul> <li> <p> <code>requested</code>: The initial state of an interconnect. The interconnect stays in the requested state until the Letter of Authorization (LOA) is sent to the customer.</p> </li> <li> <p> <code>pending</code>: The interconnect is approved, and is being initialized.</p> </li> <li> <p> <code>available</code>: The network link is up, and the interconnect is ready for use.</p> </li> <li> <p> <code>down</code>: The network link is down.</p> </li> <li> <p> <code>deleting</code>: The interconnect is being deleted.</p> </li> <li> <p> <code>deleted</code>: The interconnect is deleted.</p> </li> <li> <p> <code>unknown</code>: The state of the interconnect is not available.</p> </li> </ul></p>
    #[serde(rename = "interconnectState")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interconnect_state: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteLagRequest {
    /// <p>The ID of the LAG.</p>
    #[serde(rename = "lagId")]
    pub lag_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteVirtualInterfaceRequest {
    /// <p>The ID of the virtual interface.</p>
    #[serde(rename = "virtualInterfaceId")]
    pub virtual_interface_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DeleteVirtualInterfaceResponse {
    /// <p><p>The state of the virtual interface. The following are the possible values:</p> <ul> <li> <p> <code>confirming</code>: The creation of the virtual interface is pending confirmation from the virtual interface owner. If the owner of the virtual interface is different from the owner of the connection on which it is provisioned, then the virtual interface will remain in this state until it is confirmed by the virtual interface owner.</p> </li> <li> <p> <code>verifying</code>: This state only applies to public virtual interfaces. Each public virtual interface needs validation before the virtual interface can be created.</p> </li> <li> <p> <code>pending</code>: A virtual interface is in this state from the time that it is created until the virtual interface is ready to forward traffic.</p> </li> <li> <p> <code>available</code>: A virtual interface that is able to forward traffic.</p> </li> <li> <p> <code>down</code>: A virtual interface that is BGP down.</p> </li> <li> <p> <code>deleting</code>: A virtual interface is in this state immediately after calling <a>DeleteVirtualInterface</a> until it can no longer forward traffic.</p> </li> <li> <p> <code>deleted</code>: A virtual interface that cannot forward traffic.</p> </li> <li> <p> <code>rejected</code>: The virtual interface owner has declined creation of the virtual interface. If a virtual interface in the <code>Confirming</code> state is deleted by the virtual interface owner, the virtual interface enters the <code>Rejected</code> state.</p> </li> <li> <p> <code>unknown</code>: The state of the virtual interface is not available.</p> </li> </ul></p>
    #[serde(rename = "virtualInterfaceState")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub virtual_interface_state: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeConnectionLoaResponse {
    /// <p>The Letter of Authorization - Connecting Facility Assignment (LOA-CFA).</p>
    #[serde(rename = "loa")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub loa: Option<Loa>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeConnectionsOnInterconnectRequest {
    /// <p>The ID of the interconnect.</p>
    #[serde(rename = "interconnectId")]
    pub interconnect_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeConnectionsRequest {
    /// <p>The ID of the connection.</p>
    #[serde(rename = "connectionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connection_id: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeDirectConnectGatewayAssociationProposalsRequest {
    /// <p>The ID of the associated gateway.</p>
    #[serde(rename = "associatedGatewayId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub associated_gateway_id: Option<String>,
    /// <p>The ID of the Direct Connect gateway.</p>
    #[serde(rename = "directConnectGatewayId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub direct_connect_gateway_id: Option<String>,
    /// <p>The maximum number of results to return with a single call. To retrieve the remaining results, make another call with the returned <code>nextToken</code> value.</p> <p>If <code>MaxResults</code> is given a value larger than 100, only 100 results are returned.</p>
    #[serde(rename = "maxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>The token for the next page of results.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The ID of the proposal.</p>
    #[serde(rename = "proposalId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proposal_id: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeDirectConnectGatewayAssociationProposalsResult {
    /// <p>Describes the Direct Connect gateway association proposals.</p>
    #[serde(rename = "directConnectGatewayAssociationProposals")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub direct_connect_gateway_association_proposals:
        Option<Vec<DirectConnectGatewayAssociationProposal>>,
    /// <p>The token to use to retrieve the next page of results. This value is <code>null</code> when there are no more results to return.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeDirectConnectGatewayAssociationsRequest {
    /// <p>The ID of the associated gateway.</p>
    #[serde(rename = "associatedGatewayId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub associated_gateway_id: Option<String>,
    /// <p>The ID of the Direct Connect gateway association.</p>
    #[serde(rename = "associationId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub association_id: Option<String>,
    /// <p>The ID of the Direct Connect gateway.</p>
    #[serde(rename = "directConnectGatewayId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub direct_connect_gateway_id: Option<String>,
    /// <p>The maximum number of results to return with a single call. To retrieve the remaining results, make another call with the returned <code>nextToken</code> value.</p> <p>If <code>MaxResults</code> is given a value larger than 100, only 100 results are returned.</p>
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeDirectConnectGatewayAssociationsResult {
    /// <p>Information about the associations.</p>
    #[serde(rename = "directConnectGatewayAssociations")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub direct_connect_gateway_associations: Option<Vec<DirectConnectGatewayAssociation>>,
    /// <p>The token to retrieve the next page.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeDirectConnectGatewayAttachmentsRequest {
    /// <p>The ID of the Direct Connect gateway.</p>
    #[serde(rename = "directConnectGatewayId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub direct_connect_gateway_id: Option<String>,
    /// <p>The maximum number of results to return with a single call. To retrieve the remaining results, make another call with the returned <code>nextToken</code> value.</p> <p>If <code>MaxResults</code> is given a value larger than 100, only 100 results are returned.</p>
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeDirectConnectGatewaysRequest {
    /// <p>The ID of the Direct Connect gateway.</p>
    #[serde(rename = "directConnectGatewayId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub direct_connect_gateway_id: Option<String>,
    /// <p>The maximum number of results to return with a single call. To retrieve the remaining results, make another call with the returned <code>nextToken</code> value.</p> <p>If <code>MaxResults</code> is given a value larger than 100, only 100 results are returned.</p>
    #[serde(rename = "maxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>The token provided in the previous call to retrieve the next page.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeHostedConnectionsRequest {
    /// <p>The ID of the interconnect or LAG.</p>
    #[serde(rename = "connectionId")]
    pub connection_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeInterconnectLoaResponse {
    /// <p>The Letter of Authorization - Connecting Facility Assignment (LOA-CFA).</p>
    #[serde(rename = "loa")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub loa: Option<Loa>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeInterconnectsRequest {
    /// <p>The ID of the interconnect.</p>
    #[serde(rename = "interconnectId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interconnect_id: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeLagsRequest {
    /// <p>The ID of the LAG.</p>
    #[serde(rename = "lagId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lag_id: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeTagsRequest {
    /// <p>The Amazon Resource Names (ARNs) of the resources.</p>
    #[serde(rename = "resourceArns")]
    pub resource_arns: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeTagsResponse {
    /// <p>Information about the tags.</p>
    #[serde(rename = "resourceTags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_tags: Option<Vec<ResourceTag>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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

/// <p>Information about a Direct Connect gateway, which enables you to connect virtual interfaces and virtual private gateway or transit gateways.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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

/// <p>Information about an association between a Direct Connect gateway and a virtual private gateway or transit gateway.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DirectConnectGatewayAssociation {
    /// <p>The Amazon VPC prefixes to advertise to the Direct Connect gateway.</p>
    #[serde(rename = "allowedPrefixesToDirectConnectGateway")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allowed_prefixes_to_direct_connect_gateway: Option<Vec<RouteFilterPrefix>>,
    /// <p>Information about the associated gateway.</p>
    #[serde(rename = "associatedGateway")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub associated_gateway: Option<AssociatedGateway>,
    /// <p>The ID of the Direct Connect gateway association.</p>
    #[serde(rename = "associationId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub association_id: Option<String>,
    /// <p><p>The state of the association. The following are the possible values:</p> <ul> <li> <p> <code>associating</code>: The initial state after calling <a>CreateDirectConnectGatewayAssociation</a>.</p> </li> <li> <p> <code>associated</code>: The Direct Connect gateway and virtual private gateway or transit gateway are successfully associated and ready to pass traffic.</p> </li> <li> <p> <code>disassociating</code>: The initial state after calling <a>DeleteDirectConnectGatewayAssociation</a>.</p> </li> <li> <p> <code>disassociated</code>: The virtual private gateway or transit gateway is disassociated from the Direct Connect gateway. Traffic flow between the Direct Connect gateway and virtual private gateway or transit gateway is stopped.</p> </li> </ul></p>
    #[serde(rename = "associationState")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub association_state: Option<String>,
    /// <p>The ID of the Direct Connect gateway.</p>
    #[serde(rename = "directConnectGatewayId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub direct_connect_gateway_id: Option<String>,
    /// <p>The ID of the AWS account that owns the associated gateway.</p>
    #[serde(rename = "directConnectGatewayOwnerAccount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub direct_connect_gateway_owner_account: Option<String>,
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

/// <p>Information about the proposal request to attach a virtual private gateway to a Direct Connect gateway. </p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DirectConnectGatewayAssociationProposal {
    /// <p>Information about the associated gateway.</p>
    #[serde(rename = "associatedGateway")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub associated_gateway: Option<AssociatedGateway>,
    /// <p>The ID of the Direct Connect gateway.</p>
    #[serde(rename = "directConnectGatewayId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub direct_connect_gateway_id: Option<String>,
    /// <p>The ID of the AWS account that owns the Direct Connect gateway.</p>
    #[serde(rename = "directConnectGatewayOwnerAccount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub direct_connect_gateway_owner_account: Option<String>,
    /// <p>The existing Amazon VPC prefixes advertised to the Direct Connect gateway.</p>
    #[serde(rename = "existingAllowedPrefixesToDirectConnectGateway")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub existing_allowed_prefixes_to_direct_connect_gateway: Option<Vec<RouteFilterPrefix>>,
    /// <p>The ID of the association proposal.</p>
    #[serde(rename = "proposalId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proposal_id: Option<String>,
    /// <p><p>The state of the proposal. The following are possible values:</p> <ul> <li> <p> <code>accepted</code>: The proposal has been accepted. The Direct Connect gateway association is available to use in this state.</p> </li> <li> <p> <code>deleted</code>: The proposal has been deleted by the owner that made the proposal. The Direct Connect gateway association cannot be used in this state.</p> </li> <li> <p> <code>requested</code>: The proposal has been requested. The Direct Connect gateway association cannot be used in this state.</p> </li> </ul></p>
    #[serde(rename = "proposalState")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proposal_state: Option<String>,
    /// <p>The Amazon VPC prefixes to advertise to the Direct Connect gateway.</p>
    #[serde(rename = "requestedAllowedPrefixesToDirectConnectGateway")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested_allowed_prefixes_to_direct_connect_gateway: Option<Vec<RouteFilterPrefix>>,
}

/// <p>Information about an attachment between a Direct Connect gateway and a virtual interface.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DirectConnectGatewayAttachment {
    /// <p><p>The state of the attachment. The following are the possible values:</p> <ul> <li> <p> <code>attaching</code>: The initial state after a virtual interface is created using the Direct Connect gateway.</p> </li> <li> <p> <code>attached</code>: The Direct Connect gateway and virtual interface are attached and ready to pass traffic.</p> </li> <li> <p> <code>detaching</code>: The initial state after calling <a>DeleteVirtualInterface</a>.</p> </li> <li> <p> <code>detached</code>: The virtual interface is detached from the Direct Connect gateway. Traffic flow between the Direct Connect gateway and virtual interface is stopped.</p> </li> </ul></p>
    #[serde(rename = "attachmentState")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attachment_state: Option<String>,
    /// <p>The type of attachment.</p>
    #[serde(rename = "attachmentType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attachment_type: Option<String>,
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
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DisassociateConnectionFromLagRequest {
    /// <p>The ID of the connection.</p>
    #[serde(rename = "connectionId")]
    pub connection_id: String,
    /// <p>The ID of the LAG.</p>
    #[serde(rename = "lagId")]
    pub lag_id: String,
}

/// <p>Information about an interconnect.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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
    /// <p>The name of the service provider associated with the interconnect.</p>
    #[serde(rename = "providerName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider_name: Option<String>,
    /// <p>The AWS Region where the connection is located.</p>
    #[serde(rename = "region")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
    /// <p>The tags associated with the interconnect.</p>
    #[serde(rename = "tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct Interconnects {
    /// <p>The interconnects.</p>
    #[serde(rename = "interconnects")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interconnects: Option<Vec<Interconnect>>,
}

/// <p>Information about a link aggregation group (LAG).</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct Lag {
    /// <p>Indicates whether the LAG can host other connections.</p>
    #[serde(rename = "allowsHostedConnections")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allows_hosted_connections: Option<bool>,
    /// <p>The AWS Direct Connect endpoint that hosts the LAG.</p>
    #[serde(rename = "awsDevice")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aws_device: Option<String>,
    /// <p>The AWS Direct Connect endpoint that hosts the LAG.</p>
    #[serde(rename = "awsDeviceV2")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aws_device_v2: Option<String>,
    /// <p>The connections bundled by the LAG.</p>
    #[serde(rename = "connections")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connections: Option<Vec<Connection>>,
    /// <p>The individual bandwidth of the physical connections bundled by the LAG. The possible values are 1Gbps and 10Gbps. </p>
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
    /// <p>The name of the service provider associated with the LAG.</p>
    #[serde(rename = "providerName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider_name: Option<String>,
    /// <p>The AWS Region where the connection is located.</p>
    #[serde(rename = "region")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
    /// <p>The tags associated with the LAG.</p>
    #[serde(rename = "tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct Lags {
    /// <p>The LAGs.</p>
    #[serde(rename = "lags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lags: Option<Vec<Lag>>,
}

/// <p>Information about a Letter of Authorization - Connecting Facility Assignment (LOA-CFA) for a connection.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct Loa {
    /// <p>The binary contents of the LOA-CFA document.</p>
    #[serde(rename = "loaContent")]
    #[serde(
        deserialize_with = "::rusoto_core::serialization::SerdeBlob::deserialize_blob",
        serialize_with = "::rusoto_core::serialization::SerdeBlob::serialize_blob",
        default
    )]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub loa_content: Option<bytes::Bytes>,
    /// <p>The standard media type for the LOA-CFA document. The only supported value is application/pdf.</p>
    #[serde(rename = "loaContentType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub loa_content_type: Option<String>,
}

/// <p>Information about an AWS Direct Connect location.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct Location {
    /// <p>The available port speeds for the location.</p>
    #[serde(rename = "availablePortSpeeds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub available_port_speeds: Option<Vec<String>>,
    /// <p>The name of the service provider for the location.</p>
    #[serde(rename = "availableProviders")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub available_providers: Option<Vec<String>>,
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct Locations {
    /// <p>The locations.</p>
    #[serde(rename = "locations")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub locations: Option<Vec<Location>>,
}

/// <p>Information about a new BGP peer.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
    /// <p>The authentication key for BGP configuration. This string has a minimum length of 6 characters and and a maximun lenth of 80 characters.</p>
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
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct NewPrivateVirtualInterface {
    /// <p>The address family for the BGP peer.</p>
    #[serde(rename = "addressFamily")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address_family: Option<String>,
    /// <p>The IP address assigned to the Amazon interface.</p>
    #[serde(rename = "amazonAddress")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amazon_address: Option<String>,
    /// <p>The autonomous system (AS) number for Border Gateway Protocol (BGP) configuration.</p> <p>The valid values are 1-2147483647.</p>
    #[serde(rename = "asn")]
    pub asn: i64,
    /// <p>The authentication key for BGP configuration. This string has a minimum length of 6 characters and and a maximun lenth of 80 characters.</p>
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
    /// <p>The tags associated with the private virtual interface.</p>
    #[serde(rename = "tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
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
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct NewPrivateVirtualInterfaceAllocation {
    /// <p>The address family for the BGP peer.</p>
    #[serde(rename = "addressFamily")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address_family: Option<String>,
    /// <p>The IP address assigned to the Amazon interface.</p>
    #[serde(rename = "amazonAddress")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amazon_address: Option<String>,
    /// <p>The autonomous system (AS) number for Border Gateway Protocol (BGP) configuration.</p> <p>The valid values are 1-2147483647.</p>
    #[serde(rename = "asn")]
    pub asn: i64,
    /// <p>The authentication key for BGP configuration. This string has a minimum length of 6 characters and and a maximun lenth of 80 characters.</p>
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
    /// <p>The tags associated with the private virtual interface.</p>
    #[serde(rename = "tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
    /// <p>The name of the virtual interface assigned by the customer network.</p>
    #[serde(rename = "virtualInterfaceName")]
    pub virtual_interface_name: String,
    /// <p>The ID of the VLAN.</p>
    #[serde(rename = "vlan")]
    pub vlan: i64,
}

/// <p>Information about a public virtual interface.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct NewPublicVirtualInterface {
    /// <p>The address family for the BGP peer.</p>
    #[serde(rename = "addressFamily")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address_family: Option<String>,
    /// <p>The IP address assigned to the Amazon interface.</p>
    #[serde(rename = "amazonAddress")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amazon_address: Option<String>,
    /// <p>The autonomous system (AS) number for Border Gateway Protocol (BGP) configuration.</p> <p>The valid values are 1-2147483647.</p>
    #[serde(rename = "asn")]
    pub asn: i64,
    /// <p>The authentication key for BGP configuration. This string has a minimum length of 6 characters and and a maximun lenth of 80 characters.</p>
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
    /// <p>The tags associated with the public virtual interface.</p>
    #[serde(rename = "tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
    /// <p>The name of the virtual interface assigned by the customer network.</p>
    #[serde(rename = "virtualInterfaceName")]
    pub virtual_interface_name: String,
    /// <p>The ID of the VLAN.</p>
    #[serde(rename = "vlan")]
    pub vlan: i64,
}

/// <p>Information about a public virtual interface to be provisioned on a connection.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct NewPublicVirtualInterfaceAllocation {
    /// <p>The address family for the BGP peer.</p>
    #[serde(rename = "addressFamily")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address_family: Option<String>,
    /// <p>The IP address assigned to the Amazon interface.</p>
    #[serde(rename = "amazonAddress")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amazon_address: Option<String>,
    /// <p>The autonomous system (AS) number for Border Gateway Protocol (BGP) configuration.</p> <p>The valid values are 1-2147483647.</p>
    #[serde(rename = "asn")]
    pub asn: i64,
    /// <p>The authentication key for BGP configuration. This string has a minimum length of 6 characters and and a maximun lenth of 80 characters.</p>
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
    /// <p>The tags associated with the public virtual interface.</p>
    #[serde(rename = "tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
    /// <p>The name of the virtual interface assigned by the customer network.</p>
    #[serde(rename = "virtualInterfaceName")]
    pub virtual_interface_name: String,
    /// <p>The ID of the VLAN.</p>
    #[serde(rename = "vlan")]
    pub vlan: i64,
}

/// <p>Information about a transit virtual interface.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct NewTransitVirtualInterface {
    /// <p>The address family for the BGP peer.</p>
    #[serde(rename = "addressFamily")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address_family: Option<String>,
    /// <p>The IP address assigned to the Amazon interface.</p>
    #[serde(rename = "amazonAddress")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amazon_address: Option<String>,
    /// <p>The autonomous system (AS) number for Border Gateway Protocol (BGP) configuration.</p> <p>The valid values are 1-2147483647.</p>
    #[serde(rename = "asn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asn: Option<i64>,
    /// <p>The authentication key for BGP configuration. This string has a minimum length of 6 characters and and a maximun lenth of 80 characters.</p>
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
    /// <p>The tags associated with the transitive virtual interface.</p>
    #[serde(rename = "tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
    /// <p>The name of the virtual interface assigned by the customer network.</p>
    #[serde(rename = "virtualInterfaceName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub virtual_interface_name: Option<String>,
    /// <p>The ID of the VLAN.</p>
    #[serde(rename = "vlan")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vlan: Option<i64>,
}

/// <p>Information about a transit virtual interface to be provisioned on a connection.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct NewTransitVirtualInterfaceAllocation {
    /// <p>The address family for the BGP peer.</p>
    #[serde(rename = "addressFamily")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address_family: Option<String>,
    /// <p>The IP address assigned to the Amazon interface.</p>
    #[serde(rename = "amazonAddress")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amazon_address: Option<String>,
    /// <p>The autonomous system (AS) number for Border Gateway Protocol (BGP) configuration.</p> <p>The valid values are 1-2147483647.</p>
    #[serde(rename = "asn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asn: Option<i64>,
    /// <p>The authentication key for BGP configuration. This string has a minimum length of 6 characters and and a maximun lenth of 80 characters.</p>
    #[serde(rename = "authKey")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auth_key: Option<String>,
    /// <p>The IP address assigned to the customer interface.</p>
    #[serde(rename = "customerAddress")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_address: Option<String>,
    /// <p>The maximum transmission unit (MTU), in bytes. The supported values are 1500 and 9001. The default value is 1500. </p>
    #[serde(rename = "mtu")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mtu: Option<i64>,
    /// <p>The tags associated with the transitive virtual interface.</p>
    #[serde(rename = "tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
    /// <p>The name of the virtual interface assigned by the customer network.</p>
    #[serde(rename = "virtualInterfaceName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub virtual_interface_name: Option<String>,
    /// <p>The ID of the VLAN.</p>
    #[serde(rename = "vlan")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vlan: Option<i64>,
}

/// <p>Information about a tag associated with an AWS Direct Connect resource.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct TagResourceRequest {
    /// <p>The Amazon Resource Name (ARN) of the resource.</p>
    #[serde(rename = "resourceArn")]
    pub resource_arn: String,
    /// <p>The tags to add.</p>
    #[serde(rename = "tags")]
    pub tags: Vec<Tag>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct TagResourceResponse {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UntagResourceRequest {
    /// <p>The Amazon Resource Name (ARN) of the resource.</p>
    #[serde(rename = "resourceArn")]
    pub resource_arn: String,
    /// <p>The tag keys of the tags to remove.</p>
    #[serde(rename = "tagKeys")]
    pub tag_keys: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UntagResourceResponse {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UpdateDirectConnectGatewayAssociationRequest {
    /// <p>The Amazon VPC prefixes to advertise to the Direct Connect gateway.</p>
    #[serde(rename = "addAllowedPrefixesToDirectConnectGateway")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub add_allowed_prefixes_to_direct_connect_gateway: Option<Vec<RouteFilterPrefix>>,
    /// <p>The ID of the Direct Connect gateway association.</p>
    #[serde(rename = "associationId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub association_id: Option<String>,
    /// <p>The Amazon VPC prefixes to no longer advertise to the Direct Connect gateway.</p>
    #[serde(rename = "removeAllowedPrefixesToDirectConnectGateway")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remove_allowed_prefixes_to_direct_connect_gateway: Option<Vec<RouteFilterPrefix>>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UpdateDirectConnectGatewayAssociationResult {
    #[serde(rename = "directConnectGatewayAssociation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub direct_connect_gateway_association: Option<DirectConnectGatewayAssociation>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct VirtualGateways {
    /// <p>The virtual private gateways.</p>
    #[serde(rename = "virtualGateways")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub virtual_gateways: Option<Vec<VirtualGateway>>,
}

/// <p>Information about a virtual interface.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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
    /// <p>The autonomous system (AS) number for Border Gateway Protocol (BGP) configuration.</p> <p>The valid values are 1-2147483647.</p>
    #[serde(rename = "asn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asn: Option<i64>,
    /// <p>The authentication key for BGP configuration. This string has a minimum length of 6 characters and and a maximun lenth of 80 characters.</p>
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
    /// <p>The tags associated with the virtual interface.</p>
    #[serde(rename = "tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct VirtualInterfaces {
    /// <p>The virtual interfaces</p>
    #[serde(rename = "virtualInterfaces")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub virtual_interfaces: Option<Vec<VirtualInterface>>,
}

/// Errors returned by AcceptDirectConnectGatewayAssociationProposal
#[derive(Debug, PartialEq)]
pub enum AcceptDirectConnectGatewayAssociationProposalError {
    /// <p>One or more parameters are not valid.</p>
    DirectConnectClient(String),
    /// <p>A server-side error occurred.</p>
    DirectConnectServer(String),
}

impl AcceptDirectConnectGatewayAssociationProposalError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<AcceptDirectConnectGatewayAssociationProposalError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "DirectConnectClientException" => {
                    return RusotoError::Service(
                        AcceptDirectConnectGatewayAssociationProposalError::DirectConnectClient(
                            err.msg,
                        ),
                    )
                }
                "DirectConnectServerException" => {
                    return RusotoError::Service(
                        AcceptDirectConnectGatewayAssociationProposalError::DirectConnectServer(
                            err.msg,
                        ),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for AcceptDirectConnectGatewayAssociationProposalError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            AcceptDirectConnectGatewayAssociationProposalError::DirectConnectClient(ref cause) => {
                write!(f, "{}", cause)
            }
            AcceptDirectConnectGatewayAssociationProposalError::DirectConnectServer(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for AcceptDirectConnectGatewayAssociationProposalError {}
/// Errors returned by AllocateConnectionOnInterconnect
#[derive(Debug, PartialEq)]
pub enum AllocateConnectionOnInterconnectError {
    /// <p>One or more parameters are not valid.</p>
    DirectConnectClient(String),
    /// <p>A server-side error occurred.</p>
    DirectConnectServer(String),
}

impl AllocateConnectionOnInterconnectError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<AllocateConnectionOnInterconnectError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "DirectConnectClientException" => {
                    return RusotoError::Service(
                        AllocateConnectionOnInterconnectError::DirectConnectClient(err.msg),
                    )
                }
                "DirectConnectServerException" => {
                    return RusotoError::Service(
                        AllocateConnectionOnInterconnectError::DirectConnectServer(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for AllocateConnectionOnInterconnectError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            AllocateConnectionOnInterconnectError::DirectConnectClient(ref cause) => {
                write!(f, "{}", cause)
            }
            AllocateConnectionOnInterconnectError::DirectConnectServer(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for AllocateConnectionOnInterconnectError {}
/// Errors returned by AllocateHostedConnection
#[derive(Debug, PartialEq)]
pub enum AllocateHostedConnectionError {
    /// <p>One or more parameters are not valid.</p>
    DirectConnectClient(String),
    /// <p>A server-side error occurred.</p>
    DirectConnectServer(String),
    /// <p>A tag key was specified more than once.</p>
    DuplicateTagKeys(String),
    /// <p>You have reached the limit on the number of tags that can be assigned.</p>
    TooManyTags(String),
}

impl AllocateHostedConnectionError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<AllocateHostedConnectionError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "DirectConnectClientException" => {
                    return RusotoError::Service(
                        AllocateHostedConnectionError::DirectConnectClient(err.msg),
                    )
                }
                "DirectConnectServerException" => {
                    return RusotoError::Service(
                        AllocateHostedConnectionError::DirectConnectServer(err.msg),
                    )
                }
                "DuplicateTagKeysException" => {
                    return RusotoError::Service(AllocateHostedConnectionError::DuplicateTagKeys(
                        err.msg,
                    ))
                }
                "TooManyTagsException" => {
                    return RusotoError::Service(AllocateHostedConnectionError::TooManyTags(
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
impl fmt::Display for AllocateHostedConnectionError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            AllocateHostedConnectionError::DirectConnectClient(ref cause) => write!(f, "{}", cause),
            AllocateHostedConnectionError::DirectConnectServer(ref cause) => write!(f, "{}", cause),
            AllocateHostedConnectionError::DuplicateTagKeys(ref cause) => write!(f, "{}", cause),
            AllocateHostedConnectionError::TooManyTags(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for AllocateHostedConnectionError {}
/// Errors returned by AllocatePrivateVirtualInterface
#[derive(Debug, PartialEq)]
pub enum AllocatePrivateVirtualInterfaceError {
    /// <p>One or more parameters are not valid.</p>
    DirectConnectClient(String),
    /// <p>A server-side error occurred.</p>
    DirectConnectServer(String),
    /// <p>A tag key was specified more than once.</p>
    DuplicateTagKeys(String),
    /// <p>You have reached the limit on the number of tags that can be assigned.</p>
    TooManyTags(String),
}

impl AllocatePrivateVirtualInterfaceError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<AllocatePrivateVirtualInterfaceError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "DirectConnectClientException" => {
                    return RusotoError::Service(
                        AllocatePrivateVirtualInterfaceError::DirectConnectClient(err.msg),
                    )
                }
                "DirectConnectServerException" => {
                    return RusotoError::Service(
                        AllocatePrivateVirtualInterfaceError::DirectConnectServer(err.msg),
                    )
                }
                "DuplicateTagKeysException" => {
                    return RusotoError::Service(
                        AllocatePrivateVirtualInterfaceError::DuplicateTagKeys(err.msg),
                    )
                }
                "TooManyTagsException" => {
                    return RusotoError::Service(AllocatePrivateVirtualInterfaceError::TooManyTags(
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
impl fmt::Display for AllocatePrivateVirtualInterfaceError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            AllocatePrivateVirtualInterfaceError::DirectConnectClient(ref cause) => {
                write!(f, "{}", cause)
            }
            AllocatePrivateVirtualInterfaceError::DirectConnectServer(ref cause) => {
                write!(f, "{}", cause)
            }
            AllocatePrivateVirtualInterfaceError::DuplicateTagKeys(ref cause) => {
                write!(f, "{}", cause)
            }
            AllocatePrivateVirtualInterfaceError::TooManyTags(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for AllocatePrivateVirtualInterfaceError {}
/// Errors returned by AllocatePublicVirtualInterface
#[derive(Debug, PartialEq)]
pub enum AllocatePublicVirtualInterfaceError {
    /// <p>One or more parameters are not valid.</p>
    DirectConnectClient(String),
    /// <p>A server-side error occurred.</p>
    DirectConnectServer(String),
    /// <p>A tag key was specified more than once.</p>
    DuplicateTagKeys(String),
    /// <p>You have reached the limit on the number of tags that can be assigned.</p>
    TooManyTags(String),
}

impl AllocatePublicVirtualInterfaceError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<AllocatePublicVirtualInterfaceError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "DirectConnectClientException" => {
                    return RusotoError::Service(
                        AllocatePublicVirtualInterfaceError::DirectConnectClient(err.msg),
                    )
                }
                "DirectConnectServerException" => {
                    return RusotoError::Service(
                        AllocatePublicVirtualInterfaceError::DirectConnectServer(err.msg),
                    )
                }
                "DuplicateTagKeysException" => {
                    return RusotoError::Service(
                        AllocatePublicVirtualInterfaceError::DuplicateTagKeys(err.msg),
                    )
                }
                "TooManyTagsException" => {
                    return RusotoError::Service(AllocatePublicVirtualInterfaceError::TooManyTags(
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
impl fmt::Display for AllocatePublicVirtualInterfaceError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            AllocatePublicVirtualInterfaceError::DirectConnectClient(ref cause) => {
                write!(f, "{}", cause)
            }
            AllocatePublicVirtualInterfaceError::DirectConnectServer(ref cause) => {
                write!(f, "{}", cause)
            }
            AllocatePublicVirtualInterfaceError::DuplicateTagKeys(ref cause) => {
                write!(f, "{}", cause)
            }
            AllocatePublicVirtualInterfaceError::TooManyTags(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for AllocatePublicVirtualInterfaceError {}
/// Errors returned by AllocateTransitVirtualInterface
#[derive(Debug, PartialEq)]
pub enum AllocateTransitVirtualInterfaceError {
    /// <p>One or more parameters are not valid.</p>
    DirectConnectClient(String),
    /// <p>A server-side error occurred.</p>
    DirectConnectServer(String),
    /// <p>A tag key was specified more than once.</p>
    DuplicateTagKeys(String),
    /// <p>You have reached the limit on the number of tags that can be assigned.</p>
    TooManyTags(String),
}

impl AllocateTransitVirtualInterfaceError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<AllocateTransitVirtualInterfaceError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "DirectConnectClientException" => {
                    return RusotoError::Service(
                        AllocateTransitVirtualInterfaceError::DirectConnectClient(err.msg),
                    )
                }
                "DirectConnectServerException" => {
                    return RusotoError::Service(
                        AllocateTransitVirtualInterfaceError::DirectConnectServer(err.msg),
                    )
                }
                "DuplicateTagKeysException" => {
                    return RusotoError::Service(
                        AllocateTransitVirtualInterfaceError::DuplicateTagKeys(err.msg),
                    )
                }
                "TooManyTagsException" => {
                    return RusotoError::Service(AllocateTransitVirtualInterfaceError::TooManyTags(
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
impl fmt::Display for AllocateTransitVirtualInterfaceError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            AllocateTransitVirtualInterfaceError::DirectConnectClient(ref cause) => {
                write!(f, "{}", cause)
            }
            AllocateTransitVirtualInterfaceError::DirectConnectServer(ref cause) => {
                write!(f, "{}", cause)
            }
            AllocateTransitVirtualInterfaceError::DuplicateTagKeys(ref cause) => {
                write!(f, "{}", cause)
            }
            AllocateTransitVirtualInterfaceError::TooManyTags(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for AllocateTransitVirtualInterfaceError {}
/// Errors returned by AssociateConnectionWithLag
#[derive(Debug, PartialEq)]
pub enum AssociateConnectionWithLagError {
    /// <p>One or more parameters are not valid.</p>
    DirectConnectClient(String),
    /// <p>A server-side error occurred.</p>
    DirectConnectServer(String),
}

impl AssociateConnectionWithLagError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<AssociateConnectionWithLagError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "DirectConnectClientException" => {
                    return RusotoError::Service(
                        AssociateConnectionWithLagError::DirectConnectClient(err.msg),
                    )
                }
                "DirectConnectServerException" => {
                    return RusotoError::Service(
                        AssociateConnectionWithLagError::DirectConnectServer(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for AssociateConnectionWithLagError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            AssociateConnectionWithLagError::DirectConnectClient(ref cause) => {
                write!(f, "{}", cause)
            }
            AssociateConnectionWithLagError::DirectConnectServer(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for AssociateConnectionWithLagError {}
/// Errors returned by AssociateHostedConnection
#[derive(Debug, PartialEq)]
pub enum AssociateHostedConnectionError {
    /// <p>One or more parameters are not valid.</p>
    DirectConnectClient(String),
    /// <p>A server-side error occurred.</p>
    DirectConnectServer(String),
}

impl AssociateHostedConnectionError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<AssociateHostedConnectionError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "DirectConnectClientException" => {
                    return RusotoError::Service(
                        AssociateHostedConnectionError::DirectConnectClient(err.msg),
                    )
                }
                "DirectConnectServerException" => {
                    return RusotoError::Service(
                        AssociateHostedConnectionError::DirectConnectServer(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for AssociateHostedConnectionError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            AssociateHostedConnectionError::DirectConnectClient(ref cause) => {
                write!(f, "{}", cause)
            }
            AssociateHostedConnectionError::DirectConnectServer(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for AssociateHostedConnectionError {}
/// Errors returned by AssociateVirtualInterface
#[derive(Debug, PartialEq)]
pub enum AssociateVirtualInterfaceError {
    /// <p>One or more parameters are not valid.</p>
    DirectConnectClient(String),
    /// <p>A server-side error occurred.</p>
    DirectConnectServer(String),
}

impl AssociateVirtualInterfaceError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<AssociateVirtualInterfaceError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "DirectConnectClientException" => {
                    return RusotoError::Service(
                        AssociateVirtualInterfaceError::DirectConnectClient(err.msg),
                    )
                }
                "DirectConnectServerException" => {
                    return RusotoError::Service(
                        AssociateVirtualInterfaceError::DirectConnectServer(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for AssociateVirtualInterfaceError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            AssociateVirtualInterfaceError::DirectConnectClient(ref cause) => {
                write!(f, "{}", cause)
            }
            AssociateVirtualInterfaceError::DirectConnectServer(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for AssociateVirtualInterfaceError {}
/// Errors returned by ConfirmConnection
#[derive(Debug, PartialEq)]
pub enum ConfirmConnectionError {
    /// <p>One or more parameters are not valid.</p>
    DirectConnectClient(String),
    /// <p>A server-side error occurred.</p>
    DirectConnectServer(String),
}

impl ConfirmConnectionError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ConfirmConnectionError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "DirectConnectClientException" => {
                    return RusotoError::Service(ConfirmConnectionError::DirectConnectClient(
                        err.msg,
                    ))
                }
                "DirectConnectServerException" => {
                    return RusotoError::Service(ConfirmConnectionError::DirectConnectServer(
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
impl fmt::Display for ConfirmConnectionError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ConfirmConnectionError::DirectConnectClient(ref cause) => write!(f, "{}", cause),
            ConfirmConnectionError::DirectConnectServer(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ConfirmConnectionError {}
/// Errors returned by ConfirmPrivateVirtualInterface
#[derive(Debug, PartialEq)]
pub enum ConfirmPrivateVirtualInterfaceError {
    /// <p>One or more parameters are not valid.</p>
    DirectConnectClient(String),
    /// <p>A server-side error occurred.</p>
    DirectConnectServer(String),
}

impl ConfirmPrivateVirtualInterfaceError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<ConfirmPrivateVirtualInterfaceError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "DirectConnectClientException" => {
                    return RusotoError::Service(
                        ConfirmPrivateVirtualInterfaceError::DirectConnectClient(err.msg),
                    )
                }
                "DirectConnectServerException" => {
                    return RusotoError::Service(
                        ConfirmPrivateVirtualInterfaceError::DirectConnectServer(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ConfirmPrivateVirtualInterfaceError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ConfirmPrivateVirtualInterfaceError::DirectConnectClient(ref cause) => {
                write!(f, "{}", cause)
            }
            ConfirmPrivateVirtualInterfaceError::DirectConnectServer(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for ConfirmPrivateVirtualInterfaceError {}
/// Errors returned by ConfirmPublicVirtualInterface
#[derive(Debug, PartialEq)]
pub enum ConfirmPublicVirtualInterfaceError {
    /// <p>One or more parameters are not valid.</p>
    DirectConnectClient(String),
    /// <p>A server-side error occurred.</p>
    DirectConnectServer(String),
}

impl ConfirmPublicVirtualInterfaceError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<ConfirmPublicVirtualInterfaceError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "DirectConnectClientException" => {
                    return RusotoError::Service(
                        ConfirmPublicVirtualInterfaceError::DirectConnectClient(err.msg),
                    )
                }
                "DirectConnectServerException" => {
                    return RusotoError::Service(
                        ConfirmPublicVirtualInterfaceError::DirectConnectServer(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ConfirmPublicVirtualInterfaceError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ConfirmPublicVirtualInterfaceError::DirectConnectClient(ref cause) => {
                write!(f, "{}", cause)
            }
            ConfirmPublicVirtualInterfaceError::DirectConnectServer(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for ConfirmPublicVirtualInterfaceError {}
/// Errors returned by ConfirmTransitVirtualInterface
#[derive(Debug, PartialEq)]
pub enum ConfirmTransitVirtualInterfaceError {
    /// <p>One or more parameters are not valid.</p>
    DirectConnectClient(String),
    /// <p>A server-side error occurred.</p>
    DirectConnectServer(String),
}

impl ConfirmTransitVirtualInterfaceError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<ConfirmTransitVirtualInterfaceError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "DirectConnectClientException" => {
                    return RusotoError::Service(
                        ConfirmTransitVirtualInterfaceError::DirectConnectClient(err.msg),
                    )
                }
                "DirectConnectServerException" => {
                    return RusotoError::Service(
                        ConfirmTransitVirtualInterfaceError::DirectConnectServer(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ConfirmTransitVirtualInterfaceError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ConfirmTransitVirtualInterfaceError::DirectConnectClient(ref cause) => {
                write!(f, "{}", cause)
            }
            ConfirmTransitVirtualInterfaceError::DirectConnectServer(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for ConfirmTransitVirtualInterfaceError {}
/// Errors returned by CreateBGPPeer
#[derive(Debug, PartialEq)]
pub enum CreateBGPPeerError {
    /// <p>One or more parameters are not valid.</p>
    DirectConnectClient(String),
    /// <p>A server-side error occurred.</p>
    DirectConnectServer(String),
}

impl CreateBGPPeerError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateBGPPeerError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "DirectConnectClientException" => {
                    return RusotoError::Service(CreateBGPPeerError::DirectConnectClient(err.msg))
                }
                "DirectConnectServerException" => {
                    return RusotoError::Service(CreateBGPPeerError::DirectConnectServer(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for CreateBGPPeerError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreateBGPPeerError::DirectConnectClient(ref cause) => write!(f, "{}", cause),
            CreateBGPPeerError::DirectConnectServer(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CreateBGPPeerError {}
/// Errors returned by CreateConnection
#[derive(Debug, PartialEq)]
pub enum CreateConnectionError {
    /// <p>One or more parameters are not valid.</p>
    DirectConnectClient(String),
    /// <p>A server-side error occurred.</p>
    DirectConnectServer(String),
    /// <p>A tag key was specified more than once.</p>
    DuplicateTagKeys(String),
    /// <p>You have reached the limit on the number of tags that can be assigned.</p>
    TooManyTags(String),
}

impl CreateConnectionError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateConnectionError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "DirectConnectClientException" => {
                    return RusotoError::Service(CreateConnectionError::DirectConnectClient(
                        err.msg,
                    ))
                }
                "DirectConnectServerException" => {
                    return RusotoError::Service(CreateConnectionError::DirectConnectServer(
                        err.msg,
                    ))
                }
                "DuplicateTagKeysException" => {
                    return RusotoError::Service(CreateConnectionError::DuplicateTagKeys(err.msg))
                }
                "TooManyTagsException" => {
                    return RusotoError::Service(CreateConnectionError::TooManyTags(err.msg))
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
            CreateConnectionError::DirectConnectClient(ref cause) => write!(f, "{}", cause),
            CreateConnectionError::DirectConnectServer(ref cause) => write!(f, "{}", cause),
            CreateConnectionError::DuplicateTagKeys(ref cause) => write!(f, "{}", cause),
            CreateConnectionError::TooManyTags(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CreateConnectionError {}
/// Errors returned by CreateDirectConnectGateway
#[derive(Debug, PartialEq)]
pub enum CreateDirectConnectGatewayError {
    /// <p>One or more parameters are not valid.</p>
    DirectConnectClient(String),
    /// <p>A server-side error occurred.</p>
    DirectConnectServer(String),
}

impl CreateDirectConnectGatewayError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<CreateDirectConnectGatewayError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "DirectConnectClientException" => {
                    return RusotoError::Service(
                        CreateDirectConnectGatewayError::DirectConnectClient(err.msg),
                    )
                }
                "DirectConnectServerException" => {
                    return RusotoError::Service(
                        CreateDirectConnectGatewayError::DirectConnectServer(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for CreateDirectConnectGatewayError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreateDirectConnectGatewayError::DirectConnectClient(ref cause) => {
                write!(f, "{}", cause)
            }
            CreateDirectConnectGatewayError::DirectConnectServer(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for CreateDirectConnectGatewayError {}
/// Errors returned by CreateDirectConnectGatewayAssociation
#[derive(Debug, PartialEq)]
pub enum CreateDirectConnectGatewayAssociationError {
    /// <p>One or more parameters are not valid.</p>
    DirectConnectClient(String),
    /// <p>A server-side error occurred.</p>
    DirectConnectServer(String),
}

impl CreateDirectConnectGatewayAssociationError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<CreateDirectConnectGatewayAssociationError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "DirectConnectClientException" => {
                    return RusotoError::Service(
                        CreateDirectConnectGatewayAssociationError::DirectConnectClient(err.msg),
                    )
                }
                "DirectConnectServerException" => {
                    return RusotoError::Service(
                        CreateDirectConnectGatewayAssociationError::DirectConnectServer(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for CreateDirectConnectGatewayAssociationError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreateDirectConnectGatewayAssociationError::DirectConnectClient(ref cause) => {
                write!(f, "{}", cause)
            }
            CreateDirectConnectGatewayAssociationError::DirectConnectServer(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for CreateDirectConnectGatewayAssociationError {}
/// Errors returned by CreateDirectConnectGatewayAssociationProposal
#[derive(Debug, PartialEq)]
pub enum CreateDirectConnectGatewayAssociationProposalError {
    /// <p>One or more parameters are not valid.</p>
    DirectConnectClient(String),
    /// <p>A server-side error occurred.</p>
    DirectConnectServer(String),
}

impl CreateDirectConnectGatewayAssociationProposalError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<CreateDirectConnectGatewayAssociationProposalError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "DirectConnectClientException" => {
                    return RusotoError::Service(
                        CreateDirectConnectGatewayAssociationProposalError::DirectConnectClient(
                            err.msg,
                        ),
                    )
                }
                "DirectConnectServerException" => {
                    return RusotoError::Service(
                        CreateDirectConnectGatewayAssociationProposalError::DirectConnectServer(
                            err.msg,
                        ),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for CreateDirectConnectGatewayAssociationProposalError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreateDirectConnectGatewayAssociationProposalError::DirectConnectClient(ref cause) => {
                write!(f, "{}", cause)
            }
            CreateDirectConnectGatewayAssociationProposalError::DirectConnectServer(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for CreateDirectConnectGatewayAssociationProposalError {}
/// Errors returned by CreateInterconnect
#[derive(Debug, PartialEq)]
pub enum CreateInterconnectError {
    /// <p>One or more parameters are not valid.</p>
    DirectConnectClient(String),
    /// <p>A server-side error occurred.</p>
    DirectConnectServer(String),
    /// <p>A tag key was specified more than once.</p>
    DuplicateTagKeys(String),
    /// <p>You have reached the limit on the number of tags that can be assigned.</p>
    TooManyTags(String),
}

impl CreateInterconnectError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateInterconnectError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "DirectConnectClientException" => {
                    return RusotoError::Service(CreateInterconnectError::DirectConnectClient(
                        err.msg,
                    ))
                }
                "DirectConnectServerException" => {
                    return RusotoError::Service(CreateInterconnectError::DirectConnectServer(
                        err.msg,
                    ))
                }
                "DuplicateTagKeysException" => {
                    return RusotoError::Service(CreateInterconnectError::DuplicateTagKeys(err.msg))
                }
                "TooManyTagsException" => {
                    return RusotoError::Service(CreateInterconnectError::TooManyTags(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for CreateInterconnectError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreateInterconnectError::DirectConnectClient(ref cause) => write!(f, "{}", cause),
            CreateInterconnectError::DirectConnectServer(ref cause) => write!(f, "{}", cause),
            CreateInterconnectError::DuplicateTagKeys(ref cause) => write!(f, "{}", cause),
            CreateInterconnectError::TooManyTags(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CreateInterconnectError {}
/// Errors returned by CreateLag
#[derive(Debug, PartialEq)]
pub enum CreateLagError {
    /// <p>One or more parameters are not valid.</p>
    DirectConnectClient(String),
    /// <p>A server-side error occurred.</p>
    DirectConnectServer(String),
    /// <p>A tag key was specified more than once.</p>
    DuplicateTagKeys(String),
    /// <p>You have reached the limit on the number of tags that can be assigned.</p>
    TooManyTags(String),
}

impl CreateLagError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateLagError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "DirectConnectClientException" => {
                    return RusotoError::Service(CreateLagError::DirectConnectClient(err.msg))
                }
                "DirectConnectServerException" => {
                    return RusotoError::Service(CreateLagError::DirectConnectServer(err.msg))
                }
                "DuplicateTagKeysException" => {
                    return RusotoError::Service(CreateLagError::DuplicateTagKeys(err.msg))
                }
                "TooManyTagsException" => {
                    return RusotoError::Service(CreateLagError::TooManyTags(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for CreateLagError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreateLagError::DirectConnectClient(ref cause) => write!(f, "{}", cause),
            CreateLagError::DirectConnectServer(ref cause) => write!(f, "{}", cause),
            CreateLagError::DuplicateTagKeys(ref cause) => write!(f, "{}", cause),
            CreateLagError::TooManyTags(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CreateLagError {}
/// Errors returned by CreatePrivateVirtualInterface
#[derive(Debug, PartialEq)]
pub enum CreatePrivateVirtualInterfaceError {
    /// <p>One or more parameters are not valid.</p>
    DirectConnectClient(String),
    /// <p>A server-side error occurred.</p>
    DirectConnectServer(String),
    /// <p>A tag key was specified more than once.</p>
    DuplicateTagKeys(String),
    /// <p>You have reached the limit on the number of tags that can be assigned.</p>
    TooManyTags(String),
}

impl CreatePrivateVirtualInterfaceError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<CreatePrivateVirtualInterfaceError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "DirectConnectClientException" => {
                    return RusotoError::Service(
                        CreatePrivateVirtualInterfaceError::DirectConnectClient(err.msg),
                    )
                }
                "DirectConnectServerException" => {
                    return RusotoError::Service(
                        CreatePrivateVirtualInterfaceError::DirectConnectServer(err.msg),
                    )
                }
                "DuplicateTagKeysException" => {
                    return RusotoError::Service(
                        CreatePrivateVirtualInterfaceError::DuplicateTagKeys(err.msg),
                    )
                }
                "TooManyTagsException" => {
                    return RusotoError::Service(CreatePrivateVirtualInterfaceError::TooManyTags(
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
impl fmt::Display for CreatePrivateVirtualInterfaceError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreatePrivateVirtualInterfaceError::DirectConnectClient(ref cause) => {
                write!(f, "{}", cause)
            }
            CreatePrivateVirtualInterfaceError::DirectConnectServer(ref cause) => {
                write!(f, "{}", cause)
            }
            CreatePrivateVirtualInterfaceError::DuplicateTagKeys(ref cause) => {
                write!(f, "{}", cause)
            }
            CreatePrivateVirtualInterfaceError::TooManyTags(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CreatePrivateVirtualInterfaceError {}
/// Errors returned by CreatePublicVirtualInterface
#[derive(Debug, PartialEq)]
pub enum CreatePublicVirtualInterfaceError {
    /// <p>One or more parameters are not valid.</p>
    DirectConnectClient(String),
    /// <p>A server-side error occurred.</p>
    DirectConnectServer(String),
    /// <p>A tag key was specified more than once.</p>
    DuplicateTagKeys(String),
    /// <p>You have reached the limit on the number of tags that can be assigned.</p>
    TooManyTags(String),
}

impl CreatePublicVirtualInterfaceError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<CreatePublicVirtualInterfaceError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "DirectConnectClientException" => {
                    return RusotoError::Service(
                        CreatePublicVirtualInterfaceError::DirectConnectClient(err.msg),
                    )
                }
                "DirectConnectServerException" => {
                    return RusotoError::Service(
                        CreatePublicVirtualInterfaceError::DirectConnectServer(err.msg),
                    )
                }
                "DuplicateTagKeysException" => {
                    return RusotoError::Service(
                        CreatePublicVirtualInterfaceError::DuplicateTagKeys(err.msg),
                    )
                }
                "TooManyTagsException" => {
                    return RusotoError::Service(CreatePublicVirtualInterfaceError::TooManyTags(
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
impl fmt::Display for CreatePublicVirtualInterfaceError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreatePublicVirtualInterfaceError::DirectConnectClient(ref cause) => {
                write!(f, "{}", cause)
            }
            CreatePublicVirtualInterfaceError::DirectConnectServer(ref cause) => {
                write!(f, "{}", cause)
            }
            CreatePublicVirtualInterfaceError::DuplicateTagKeys(ref cause) => {
                write!(f, "{}", cause)
            }
            CreatePublicVirtualInterfaceError::TooManyTags(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CreatePublicVirtualInterfaceError {}
/// Errors returned by CreateTransitVirtualInterface
#[derive(Debug, PartialEq)]
pub enum CreateTransitVirtualInterfaceError {
    /// <p>One or more parameters are not valid.</p>
    DirectConnectClient(String),
    /// <p>A server-side error occurred.</p>
    DirectConnectServer(String),
    /// <p>A tag key was specified more than once.</p>
    DuplicateTagKeys(String),
    /// <p>You have reached the limit on the number of tags that can be assigned.</p>
    TooManyTags(String),
}

impl CreateTransitVirtualInterfaceError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<CreateTransitVirtualInterfaceError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "DirectConnectClientException" => {
                    return RusotoError::Service(
                        CreateTransitVirtualInterfaceError::DirectConnectClient(err.msg),
                    )
                }
                "DirectConnectServerException" => {
                    return RusotoError::Service(
                        CreateTransitVirtualInterfaceError::DirectConnectServer(err.msg),
                    )
                }
                "DuplicateTagKeysException" => {
                    return RusotoError::Service(
                        CreateTransitVirtualInterfaceError::DuplicateTagKeys(err.msg),
                    )
                }
                "TooManyTagsException" => {
                    return RusotoError::Service(CreateTransitVirtualInterfaceError::TooManyTags(
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
impl fmt::Display for CreateTransitVirtualInterfaceError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreateTransitVirtualInterfaceError::DirectConnectClient(ref cause) => {
                write!(f, "{}", cause)
            }
            CreateTransitVirtualInterfaceError::DirectConnectServer(ref cause) => {
                write!(f, "{}", cause)
            }
            CreateTransitVirtualInterfaceError::DuplicateTagKeys(ref cause) => {
                write!(f, "{}", cause)
            }
            CreateTransitVirtualInterfaceError::TooManyTags(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CreateTransitVirtualInterfaceError {}
/// Errors returned by DeleteBGPPeer
#[derive(Debug, PartialEq)]
pub enum DeleteBGPPeerError {
    /// <p>One or more parameters are not valid.</p>
    DirectConnectClient(String),
    /// <p>A server-side error occurred.</p>
    DirectConnectServer(String),
}

impl DeleteBGPPeerError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteBGPPeerError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "DirectConnectClientException" => {
                    return RusotoError::Service(DeleteBGPPeerError::DirectConnectClient(err.msg))
                }
                "DirectConnectServerException" => {
                    return RusotoError::Service(DeleteBGPPeerError::DirectConnectServer(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DeleteBGPPeerError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteBGPPeerError::DirectConnectClient(ref cause) => write!(f, "{}", cause),
            DeleteBGPPeerError::DirectConnectServer(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeleteBGPPeerError {}
/// Errors returned by DeleteConnection
#[derive(Debug, PartialEq)]
pub enum DeleteConnectionError {
    /// <p>One or more parameters are not valid.</p>
    DirectConnectClient(String),
    /// <p>A server-side error occurred.</p>
    DirectConnectServer(String),
}

impl DeleteConnectionError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteConnectionError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "DirectConnectClientException" => {
                    return RusotoError::Service(DeleteConnectionError::DirectConnectClient(
                        err.msg,
                    ))
                }
                "DirectConnectServerException" => {
                    return RusotoError::Service(DeleteConnectionError::DirectConnectServer(
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
impl fmt::Display for DeleteConnectionError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteConnectionError::DirectConnectClient(ref cause) => write!(f, "{}", cause),
            DeleteConnectionError::DirectConnectServer(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeleteConnectionError {}
/// Errors returned by DeleteDirectConnectGateway
#[derive(Debug, PartialEq)]
pub enum DeleteDirectConnectGatewayError {
    /// <p>One or more parameters are not valid.</p>
    DirectConnectClient(String),
    /// <p>A server-side error occurred.</p>
    DirectConnectServer(String),
}

impl DeleteDirectConnectGatewayError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<DeleteDirectConnectGatewayError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "DirectConnectClientException" => {
                    return RusotoError::Service(
                        DeleteDirectConnectGatewayError::DirectConnectClient(err.msg),
                    )
                }
                "DirectConnectServerException" => {
                    return RusotoError::Service(
                        DeleteDirectConnectGatewayError::DirectConnectServer(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DeleteDirectConnectGatewayError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteDirectConnectGatewayError::DirectConnectClient(ref cause) => {
                write!(f, "{}", cause)
            }
            DeleteDirectConnectGatewayError::DirectConnectServer(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for DeleteDirectConnectGatewayError {}
/// Errors returned by DeleteDirectConnectGatewayAssociation
#[derive(Debug, PartialEq)]
pub enum DeleteDirectConnectGatewayAssociationError {
    /// <p>One or more parameters are not valid.</p>
    DirectConnectClient(String),
    /// <p>A server-side error occurred.</p>
    DirectConnectServer(String),
}

impl DeleteDirectConnectGatewayAssociationError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<DeleteDirectConnectGatewayAssociationError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "DirectConnectClientException" => {
                    return RusotoError::Service(
                        DeleteDirectConnectGatewayAssociationError::DirectConnectClient(err.msg),
                    )
                }
                "DirectConnectServerException" => {
                    return RusotoError::Service(
                        DeleteDirectConnectGatewayAssociationError::DirectConnectServer(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DeleteDirectConnectGatewayAssociationError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteDirectConnectGatewayAssociationError::DirectConnectClient(ref cause) => {
                write!(f, "{}", cause)
            }
            DeleteDirectConnectGatewayAssociationError::DirectConnectServer(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for DeleteDirectConnectGatewayAssociationError {}
/// Errors returned by DeleteDirectConnectGatewayAssociationProposal
#[derive(Debug, PartialEq)]
pub enum DeleteDirectConnectGatewayAssociationProposalError {
    /// <p>One or more parameters are not valid.</p>
    DirectConnectClient(String),
    /// <p>A server-side error occurred.</p>
    DirectConnectServer(String),
}

impl DeleteDirectConnectGatewayAssociationProposalError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<DeleteDirectConnectGatewayAssociationProposalError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "DirectConnectClientException" => {
                    return RusotoError::Service(
                        DeleteDirectConnectGatewayAssociationProposalError::DirectConnectClient(
                            err.msg,
                        ),
                    )
                }
                "DirectConnectServerException" => {
                    return RusotoError::Service(
                        DeleteDirectConnectGatewayAssociationProposalError::DirectConnectServer(
                            err.msg,
                        ),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DeleteDirectConnectGatewayAssociationProposalError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteDirectConnectGatewayAssociationProposalError::DirectConnectClient(ref cause) => {
                write!(f, "{}", cause)
            }
            DeleteDirectConnectGatewayAssociationProposalError::DirectConnectServer(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for DeleteDirectConnectGatewayAssociationProposalError {}
/// Errors returned by DeleteInterconnect
#[derive(Debug, PartialEq)]
pub enum DeleteInterconnectError {
    /// <p>One or more parameters are not valid.</p>
    DirectConnectClient(String),
    /// <p>A server-side error occurred.</p>
    DirectConnectServer(String),
}

impl DeleteInterconnectError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteInterconnectError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "DirectConnectClientException" => {
                    return RusotoError::Service(DeleteInterconnectError::DirectConnectClient(
                        err.msg,
                    ))
                }
                "DirectConnectServerException" => {
                    return RusotoError::Service(DeleteInterconnectError::DirectConnectServer(
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
impl fmt::Display for DeleteInterconnectError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteInterconnectError::DirectConnectClient(ref cause) => write!(f, "{}", cause),
            DeleteInterconnectError::DirectConnectServer(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeleteInterconnectError {}
/// Errors returned by DeleteLag
#[derive(Debug, PartialEq)]
pub enum DeleteLagError {
    /// <p>One or more parameters are not valid.</p>
    DirectConnectClient(String),
    /// <p>A server-side error occurred.</p>
    DirectConnectServer(String),
}

impl DeleteLagError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteLagError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "DirectConnectClientException" => {
                    return RusotoError::Service(DeleteLagError::DirectConnectClient(err.msg))
                }
                "DirectConnectServerException" => {
                    return RusotoError::Service(DeleteLagError::DirectConnectServer(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DeleteLagError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteLagError::DirectConnectClient(ref cause) => write!(f, "{}", cause),
            DeleteLagError::DirectConnectServer(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeleteLagError {}
/// Errors returned by DeleteVirtualInterface
#[derive(Debug, PartialEq)]
pub enum DeleteVirtualInterfaceError {
    /// <p>One or more parameters are not valid.</p>
    DirectConnectClient(String),
    /// <p>A server-side error occurred.</p>
    DirectConnectServer(String),
}

impl DeleteVirtualInterfaceError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteVirtualInterfaceError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "DirectConnectClientException" => {
                    return RusotoError::Service(DeleteVirtualInterfaceError::DirectConnectClient(
                        err.msg,
                    ))
                }
                "DirectConnectServerException" => {
                    return RusotoError::Service(DeleteVirtualInterfaceError::DirectConnectServer(
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
impl fmt::Display for DeleteVirtualInterfaceError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteVirtualInterfaceError::DirectConnectClient(ref cause) => write!(f, "{}", cause),
            DeleteVirtualInterfaceError::DirectConnectServer(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeleteVirtualInterfaceError {}
/// Errors returned by DescribeConnectionLoa
#[derive(Debug, PartialEq)]
pub enum DescribeConnectionLoaError {
    /// <p>One or more parameters are not valid.</p>
    DirectConnectClient(String),
    /// <p>A server-side error occurred.</p>
    DirectConnectServer(String),
}

impl DescribeConnectionLoaError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeConnectionLoaError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "DirectConnectClientException" => {
                    return RusotoError::Service(DescribeConnectionLoaError::DirectConnectClient(
                        err.msg,
                    ))
                }
                "DirectConnectServerException" => {
                    return RusotoError::Service(DescribeConnectionLoaError::DirectConnectServer(
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
impl fmt::Display for DescribeConnectionLoaError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeConnectionLoaError::DirectConnectClient(ref cause) => write!(f, "{}", cause),
            DescribeConnectionLoaError::DirectConnectServer(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DescribeConnectionLoaError {}
/// Errors returned by DescribeConnections
#[derive(Debug, PartialEq)]
pub enum DescribeConnectionsError {
    /// <p>One or more parameters are not valid.</p>
    DirectConnectClient(String),
    /// <p>A server-side error occurred.</p>
    DirectConnectServer(String),
}

impl DescribeConnectionsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeConnectionsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "DirectConnectClientException" => {
                    return RusotoError::Service(DescribeConnectionsError::DirectConnectClient(
                        err.msg,
                    ))
                }
                "DirectConnectServerException" => {
                    return RusotoError::Service(DescribeConnectionsError::DirectConnectServer(
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
impl fmt::Display for DescribeConnectionsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeConnectionsError::DirectConnectClient(ref cause) => write!(f, "{}", cause),
            DescribeConnectionsError::DirectConnectServer(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DescribeConnectionsError {}
/// Errors returned by DescribeConnectionsOnInterconnect
#[derive(Debug, PartialEq)]
pub enum DescribeConnectionsOnInterconnectError {
    /// <p>One or more parameters are not valid.</p>
    DirectConnectClient(String),
    /// <p>A server-side error occurred.</p>
    DirectConnectServer(String),
}

impl DescribeConnectionsOnInterconnectError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<DescribeConnectionsOnInterconnectError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "DirectConnectClientException" => {
                    return RusotoError::Service(
                        DescribeConnectionsOnInterconnectError::DirectConnectClient(err.msg),
                    )
                }
                "DirectConnectServerException" => {
                    return RusotoError::Service(
                        DescribeConnectionsOnInterconnectError::DirectConnectServer(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DescribeConnectionsOnInterconnectError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeConnectionsOnInterconnectError::DirectConnectClient(ref cause) => {
                write!(f, "{}", cause)
            }
            DescribeConnectionsOnInterconnectError::DirectConnectServer(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for DescribeConnectionsOnInterconnectError {}
/// Errors returned by DescribeDirectConnectGatewayAssociationProposals
#[derive(Debug, PartialEq)]
pub enum DescribeDirectConnectGatewayAssociationProposalsError {
    /// <p>One or more parameters are not valid.</p>
    DirectConnectClient(String),
    /// <p>A server-side error occurred.</p>
    DirectConnectServer(String),
}

impl DescribeDirectConnectGatewayAssociationProposalsError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<DescribeDirectConnectGatewayAssociationProposalsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "DirectConnectClientException" => {
                    return RusotoError::Service(
                        DescribeDirectConnectGatewayAssociationProposalsError::DirectConnectClient(
                            err.msg,
                        ),
                    )
                }
                "DirectConnectServerException" => {
                    return RusotoError::Service(
                        DescribeDirectConnectGatewayAssociationProposalsError::DirectConnectServer(
                            err.msg,
                        ),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DescribeDirectConnectGatewayAssociationProposalsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeDirectConnectGatewayAssociationProposalsError::DirectConnectClient(
                ref cause,
            ) => write!(f, "{}", cause),
            DescribeDirectConnectGatewayAssociationProposalsError::DirectConnectServer(
                ref cause,
            ) => write!(f, "{}", cause),
        }
    }
}
impl Error for DescribeDirectConnectGatewayAssociationProposalsError {}
/// Errors returned by DescribeDirectConnectGatewayAssociations
#[derive(Debug, PartialEq)]
pub enum DescribeDirectConnectGatewayAssociationsError {
    /// <p>One or more parameters are not valid.</p>
    DirectConnectClient(String),
    /// <p>A server-side error occurred.</p>
    DirectConnectServer(String),
}

impl DescribeDirectConnectGatewayAssociationsError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<DescribeDirectConnectGatewayAssociationsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "DirectConnectClientException" => {
                    return RusotoError::Service(
                        DescribeDirectConnectGatewayAssociationsError::DirectConnectClient(err.msg),
                    )
                }
                "DirectConnectServerException" => {
                    return RusotoError::Service(
                        DescribeDirectConnectGatewayAssociationsError::DirectConnectServer(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DescribeDirectConnectGatewayAssociationsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeDirectConnectGatewayAssociationsError::DirectConnectClient(ref cause) => {
                write!(f, "{}", cause)
            }
            DescribeDirectConnectGatewayAssociationsError::DirectConnectServer(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for DescribeDirectConnectGatewayAssociationsError {}
/// Errors returned by DescribeDirectConnectGatewayAttachments
#[derive(Debug, PartialEq)]
pub enum DescribeDirectConnectGatewayAttachmentsError {
    /// <p>One or more parameters are not valid.</p>
    DirectConnectClient(String),
    /// <p>A server-side error occurred.</p>
    DirectConnectServer(String),
}

impl DescribeDirectConnectGatewayAttachmentsError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<DescribeDirectConnectGatewayAttachmentsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "DirectConnectClientException" => {
                    return RusotoError::Service(
                        DescribeDirectConnectGatewayAttachmentsError::DirectConnectClient(err.msg),
                    )
                }
                "DirectConnectServerException" => {
                    return RusotoError::Service(
                        DescribeDirectConnectGatewayAttachmentsError::DirectConnectServer(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DescribeDirectConnectGatewayAttachmentsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeDirectConnectGatewayAttachmentsError::DirectConnectClient(ref cause) => {
                write!(f, "{}", cause)
            }
            DescribeDirectConnectGatewayAttachmentsError::DirectConnectServer(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for DescribeDirectConnectGatewayAttachmentsError {}
/// Errors returned by DescribeDirectConnectGateways
#[derive(Debug, PartialEq)]
pub enum DescribeDirectConnectGatewaysError {
    /// <p>One or more parameters are not valid.</p>
    DirectConnectClient(String),
    /// <p>A server-side error occurred.</p>
    DirectConnectServer(String),
}

impl DescribeDirectConnectGatewaysError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<DescribeDirectConnectGatewaysError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "DirectConnectClientException" => {
                    return RusotoError::Service(
                        DescribeDirectConnectGatewaysError::DirectConnectClient(err.msg),
                    )
                }
                "DirectConnectServerException" => {
                    return RusotoError::Service(
                        DescribeDirectConnectGatewaysError::DirectConnectServer(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DescribeDirectConnectGatewaysError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeDirectConnectGatewaysError::DirectConnectClient(ref cause) => {
                write!(f, "{}", cause)
            }
            DescribeDirectConnectGatewaysError::DirectConnectServer(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for DescribeDirectConnectGatewaysError {}
/// Errors returned by DescribeHostedConnections
#[derive(Debug, PartialEq)]
pub enum DescribeHostedConnectionsError {
    /// <p>One or more parameters are not valid.</p>
    DirectConnectClient(String),
    /// <p>A server-side error occurred.</p>
    DirectConnectServer(String),
}

impl DescribeHostedConnectionsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeHostedConnectionsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "DirectConnectClientException" => {
                    return RusotoError::Service(
                        DescribeHostedConnectionsError::DirectConnectClient(err.msg),
                    )
                }
                "DirectConnectServerException" => {
                    return RusotoError::Service(
                        DescribeHostedConnectionsError::DirectConnectServer(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DescribeHostedConnectionsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeHostedConnectionsError::DirectConnectClient(ref cause) => {
                write!(f, "{}", cause)
            }
            DescribeHostedConnectionsError::DirectConnectServer(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for DescribeHostedConnectionsError {}
/// Errors returned by DescribeInterconnectLoa
#[derive(Debug, PartialEq)]
pub enum DescribeInterconnectLoaError {
    /// <p>One or more parameters are not valid.</p>
    DirectConnectClient(String),
    /// <p>A server-side error occurred.</p>
    DirectConnectServer(String),
}

impl DescribeInterconnectLoaError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeInterconnectLoaError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "DirectConnectClientException" => {
                    return RusotoError::Service(DescribeInterconnectLoaError::DirectConnectClient(
                        err.msg,
                    ))
                }
                "DirectConnectServerException" => {
                    return RusotoError::Service(DescribeInterconnectLoaError::DirectConnectServer(
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
impl fmt::Display for DescribeInterconnectLoaError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeInterconnectLoaError::DirectConnectClient(ref cause) => write!(f, "{}", cause),
            DescribeInterconnectLoaError::DirectConnectServer(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DescribeInterconnectLoaError {}
/// Errors returned by DescribeInterconnects
#[derive(Debug, PartialEq)]
pub enum DescribeInterconnectsError {
    /// <p>One or more parameters are not valid.</p>
    DirectConnectClient(String),
    /// <p>A server-side error occurred.</p>
    DirectConnectServer(String),
}

impl DescribeInterconnectsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeInterconnectsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "DirectConnectClientException" => {
                    return RusotoError::Service(DescribeInterconnectsError::DirectConnectClient(
                        err.msg,
                    ))
                }
                "DirectConnectServerException" => {
                    return RusotoError::Service(DescribeInterconnectsError::DirectConnectServer(
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
impl fmt::Display for DescribeInterconnectsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeInterconnectsError::DirectConnectClient(ref cause) => write!(f, "{}", cause),
            DescribeInterconnectsError::DirectConnectServer(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DescribeInterconnectsError {}
/// Errors returned by DescribeLags
#[derive(Debug, PartialEq)]
pub enum DescribeLagsError {
    /// <p>One or more parameters are not valid.</p>
    DirectConnectClient(String),
    /// <p>A server-side error occurred.</p>
    DirectConnectServer(String),
}

impl DescribeLagsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeLagsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "DirectConnectClientException" => {
                    return RusotoError::Service(DescribeLagsError::DirectConnectClient(err.msg))
                }
                "DirectConnectServerException" => {
                    return RusotoError::Service(DescribeLagsError::DirectConnectServer(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DescribeLagsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeLagsError::DirectConnectClient(ref cause) => write!(f, "{}", cause),
            DescribeLagsError::DirectConnectServer(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DescribeLagsError {}
/// Errors returned by DescribeLoa
#[derive(Debug, PartialEq)]
pub enum DescribeLoaError {
    /// <p>One or more parameters are not valid.</p>
    DirectConnectClient(String),
    /// <p>A server-side error occurred.</p>
    DirectConnectServer(String),
}

impl DescribeLoaError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeLoaError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "DirectConnectClientException" => {
                    return RusotoError::Service(DescribeLoaError::DirectConnectClient(err.msg))
                }
                "DirectConnectServerException" => {
                    return RusotoError::Service(DescribeLoaError::DirectConnectServer(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DescribeLoaError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeLoaError::DirectConnectClient(ref cause) => write!(f, "{}", cause),
            DescribeLoaError::DirectConnectServer(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DescribeLoaError {}
/// Errors returned by DescribeLocations
#[derive(Debug, PartialEq)]
pub enum DescribeLocationsError {
    /// <p>One or more parameters are not valid.</p>
    DirectConnectClient(String),
    /// <p>A server-side error occurred.</p>
    DirectConnectServer(String),
}

impl DescribeLocationsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeLocationsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "DirectConnectClientException" => {
                    return RusotoError::Service(DescribeLocationsError::DirectConnectClient(
                        err.msg,
                    ))
                }
                "DirectConnectServerException" => {
                    return RusotoError::Service(DescribeLocationsError::DirectConnectServer(
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
impl fmt::Display for DescribeLocationsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeLocationsError::DirectConnectClient(ref cause) => write!(f, "{}", cause),
            DescribeLocationsError::DirectConnectServer(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DescribeLocationsError {}
/// Errors returned by DescribeTags
#[derive(Debug, PartialEq)]
pub enum DescribeTagsError {
    /// <p>One or more parameters are not valid.</p>
    DirectConnectClient(String),
    /// <p>A server-side error occurred.</p>
    DirectConnectServer(String),
}

impl DescribeTagsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeTagsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "DirectConnectClientException" => {
                    return RusotoError::Service(DescribeTagsError::DirectConnectClient(err.msg))
                }
                "DirectConnectServerException" => {
                    return RusotoError::Service(DescribeTagsError::DirectConnectServer(err.msg))
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
            DescribeTagsError::DirectConnectClient(ref cause) => write!(f, "{}", cause),
            DescribeTagsError::DirectConnectServer(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DescribeTagsError {}
/// Errors returned by DescribeVirtualGateways
#[derive(Debug, PartialEq)]
pub enum DescribeVirtualGatewaysError {
    /// <p>One or more parameters are not valid.</p>
    DirectConnectClient(String),
    /// <p>A server-side error occurred.</p>
    DirectConnectServer(String),
}

impl DescribeVirtualGatewaysError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeVirtualGatewaysError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "DirectConnectClientException" => {
                    return RusotoError::Service(DescribeVirtualGatewaysError::DirectConnectClient(
                        err.msg,
                    ))
                }
                "DirectConnectServerException" => {
                    return RusotoError::Service(DescribeVirtualGatewaysError::DirectConnectServer(
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
impl fmt::Display for DescribeVirtualGatewaysError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeVirtualGatewaysError::DirectConnectClient(ref cause) => write!(f, "{}", cause),
            DescribeVirtualGatewaysError::DirectConnectServer(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DescribeVirtualGatewaysError {}
/// Errors returned by DescribeVirtualInterfaces
#[derive(Debug, PartialEq)]
pub enum DescribeVirtualInterfacesError {
    /// <p>One or more parameters are not valid.</p>
    DirectConnectClient(String),
    /// <p>A server-side error occurred.</p>
    DirectConnectServer(String),
}

impl DescribeVirtualInterfacesError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeVirtualInterfacesError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "DirectConnectClientException" => {
                    return RusotoError::Service(
                        DescribeVirtualInterfacesError::DirectConnectClient(err.msg),
                    )
                }
                "DirectConnectServerException" => {
                    return RusotoError::Service(
                        DescribeVirtualInterfacesError::DirectConnectServer(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DescribeVirtualInterfacesError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeVirtualInterfacesError::DirectConnectClient(ref cause) => {
                write!(f, "{}", cause)
            }
            DescribeVirtualInterfacesError::DirectConnectServer(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for DescribeVirtualInterfacesError {}
/// Errors returned by DisassociateConnectionFromLag
#[derive(Debug, PartialEq)]
pub enum DisassociateConnectionFromLagError {
    /// <p>One or more parameters are not valid.</p>
    DirectConnectClient(String),
    /// <p>A server-side error occurred.</p>
    DirectConnectServer(String),
}

impl DisassociateConnectionFromLagError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<DisassociateConnectionFromLagError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "DirectConnectClientException" => {
                    return RusotoError::Service(
                        DisassociateConnectionFromLagError::DirectConnectClient(err.msg),
                    )
                }
                "DirectConnectServerException" => {
                    return RusotoError::Service(
                        DisassociateConnectionFromLagError::DirectConnectServer(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DisassociateConnectionFromLagError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DisassociateConnectionFromLagError::DirectConnectClient(ref cause) => {
                write!(f, "{}", cause)
            }
            DisassociateConnectionFromLagError::DirectConnectServer(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for DisassociateConnectionFromLagError {}
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
}

impl TagResourceError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<TagResourceError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "DirectConnectClientException" => {
                    return RusotoError::Service(TagResourceError::DirectConnectClient(err.msg))
                }
                "DirectConnectServerException" => {
                    return RusotoError::Service(TagResourceError::DirectConnectServer(err.msg))
                }
                "DuplicateTagKeysException" => {
                    return RusotoError::Service(TagResourceError::DuplicateTagKeys(err.msg))
                }
                "TooManyTagsException" => {
                    return RusotoError::Service(TagResourceError::TooManyTags(err.msg))
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
            TagResourceError::DirectConnectClient(ref cause) => write!(f, "{}", cause),
            TagResourceError::DirectConnectServer(ref cause) => write!(f, "{}", cause),
            TagResourceError::DuplicateTagKeys(ref cause) => write!(f, "{}", cause),
            TagResourceError::TooManyTags(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for TagResourceError {}
/// Errors returned by UntagResource
#[derive(Debug, PartialEq)]
pub enum UntagResourceError {
    /// <p>One or more parameters are not valid.</p>
    DirectConnectClient(String),
    /// <p>A server-side error occurred.</p>
    DirectConnectServer(String),
}

impl UntagResourceError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UntagResourceError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "DirectConnectClientException" => {
                    return RusotoError::Service(UntagResourceError::DirectConnectClient(err.msg))
                }
                "DirectConnectServerException" => {
                    return RusotoError::Service(UntagResourceError::DirectConnectServer(err.msg))
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
            UntagResourceError::DirectConnectClient(ref cause) => write!(f, "{}", cause),
            UntagResourceError::DirectConnectServer(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for UntagResourceError {}
/// Errors returned by UpdateDirectConnectGatewayAssociation
#[derive(Debug, PartialEq)]
pub enum UpdateDirectConnectGatewayAssociationError {
    /// <p>One or more parameters are not valid.</p>
    DirectConnectClient(String),
    /// <p>A server-side error occurred.</p>
    DirectConnectServer(String),
}

impl UpdateDirectConnectGatewayAssociationError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<UpdateDirectConnectGatewayAssociationError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "DirectConnectClientException" => {
                    return RusotoError::Service(
                        UpdateDirectConnectGatewayAssociationError::DirectConnectClient(err.msg),
                    )
                }
                "DirectConnectServerException" => {
                    return RusotoError::Service(
                        UpdateDirectConnectGatewayAssociationError::DirectConnectServer(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for UpdateDirectConnectGatewayAssociationError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UpdateDirectConnectGatewayAssociationError::DirectConnectClient(ref cause) => {
                write!(f, "{}", cause)
            }
            UpdateDirectConnectGatewayAssociationError::DirectConnectServer(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for UpdateDirectConnectGatewayAssociationError {}
/// Errors returned by UpdateLag
#[derive(Debug, PartialEq)]
pub enum UpdateLagError {
    /// <p>One or more parameters are not valid.</p>
    DirectConnectClient(String),
    /// <p>A server-side error occurred.</p>
    DirectConnectServer(String),
}

impl UpdateLagError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdateLagError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "DirectConnectClientException" => {
                    return RusotoError::Service(UpdateLagError::DirectConnectClient(err.msg))
                }
                "DirectConnectServerException" => {
                    return RusotoError::Service(UpdateLagError::DirectConnectServer(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for UpdateLagError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UpdateLagError::DirectConnectClient(ref cause) => write!(f, "{}", cause),
            UpdateLagError::DirectConnectServer(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for UpdateLagError {}
/// Errors returned by UpdateVirtualInterfaceAttributes
#[derive(Debug, PartialEq)]
pub enum UpdateVirtualInterfaceAttributesError {
    /// <p>One or more parameters are not valid.</p>
    DirectConnectClient(String),
    /// <p>A server-side error occurred.</p>
    DirectConnectServer(String),
}

impl UpdateVirtualInterfaceAttributesError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<UpdateVirtualInterfaceAttributesError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "DirectConnectClientException" => {
                    return RusotoError::Service(
                        UpdateVirtualInterfaceAttributesError::DirectConnectClient(err.msg),
                    )
                }
                "DirectConnectServerException" => {
                    return RusotoError::Service(
                        UpdateVirtualInterfaceAttributesError::DirectConnectServer(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for UpdateVirtualInterfaceAttributesError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UpdateVirtualInterfaceAttributesError::DirectConnectClient(ref cause) => {
                write!(f, "{}", cause)
            }
            UpdateVirtualInterfaceAttributesError::DirectConnectServer(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for UpdateVirtualInterfaceAttributesError {}
/// Trait representing the capabilities of the AWS Direct Connect API. AWS Direct Connect clients implement this trait.
#[async_trait]
pub trait DirectConnect {
    /// <p>Accepts a proposal request to attach a virtual private gateway or transit gateway to a Direct Connect gateway.</p>
    async fn accept_direct_connect_gateway_association_proposal(
        &self,
        input: AcceptDirectConnectGatewayAssociationProposalRequest,
    ) -> Result<
        AcceptDirectConnectGatewayAssociationProposalResult,
        RusotoError<AcceptDirectConnectGatewayAssociationProposalError>,
    >;

    /// <p><p>Deprecated. Use <a>AllocateHostedConnection</a> instead.</p> <p>Creates a hosted connection on an interconnect.</p> <p>Allocates a VLAN number and a specified amount of bandwidth for use by a hosted connection on the specified interconnect.</p> <note> <p>Intended for use by AWS Direct Connect Partners only.</p> </note></p>
    async fn allocate_connection_on_interconnect(
        &self,
        input: AllocateConnectionOnInterconnectRequest,
    ) -> Result<Connection, RusotoError<AllocateConnectionOnInterconnectError>>;

    /// <p><p>Creates a hosted connection on the specified interconnect or a link aggregation group (LAG) of interconnects.</p> <p>Allocates a VLAN number and a specified amount of capacity (bandwidth) for use by a hosted connection on the specified interconnect or LAG of interconnects. AWS polices the hosted connection for the specified capacity and the AWS Direct Connect Partner must also police the hosted connection for the specified capacity.</p> <note> <p>Intended for use by AWS Direct Connect Partners only.</p> </note></p>
    async fn allocate_hosted_connection(
        &self,
        input: AllocateHostedConnectionRequest,
    ) -> Result<Connection, RusotoError<AllocateHostedConnectionError>>;

    /// <p>Provisions a private virtual interface to be owned by the specified AWS account.</p> <p>Virtual interfaces created using this action must be confirmed by the owner using <a>ConfirmPrivateVirtualInterface</a>. Until then, the virtual interface is in the <code>Confirming</code> state and is not available to handle traffic.</p>
    async fn allocate_private_virtual_interface(
        &self,
        input: AllocatePrivateVirtualInterfaceRequest,
    ) -> Result<VirtualInterface, RusotoError<AllocatePrivateVirtualInterfaceError>>;

    /// <p>Provisions a public virtual interface to be owned by the specified AWS account.</p> <p>The owner of a connection calls this function to provision a public virtual interface to be owned by the specified AWS account.</p> <p>Virtual interfaces created using this function must be confirmed by the owner using <a>ConfirmPublicVirtualInterface</a>. Until this step has been completed, the virtual interface is in the <code>confirming</code> state and is not available to handle traffic.</p> <p>When creating an IPv6 public virtual interface, omit the Amazon address and customer address. IPv6 addresses are automatically assigned from the Amazon pool of IPv6 addresses; you cannot specify custom IPv6 addresses.</p>
    async fn allocate_public_virtual_interface(
        &self,
        input: AllocatePublicVirtualInterfaceRequest,
    ) -> Result<VirtualInterface, RusotoError<AllocatePublicVirtualInterfaceError>>;

    /// <p>Provisions a transit virtual interface to be owned by the specified AWS account. Use this type of interface to connect a transit gateway to your Direct Connect gateway.</p> <p>The owner of a connection provisions a transit virtual interface to be owned by the specified AWS account.</p> <p>After you create a transit virtual interface, it must be confirmed by the owner using <a>ConfirmTransitVirtualInterface</a>. Until this step has been completed, the transit virtual interface is in the <code>requested</code> state and is not available to handle traffic.</p>
    async fn allocate_transit_virtual_interface(
        &self,
        input: AllocateTransitVirtualInterfaceRequest,
    ) -> Result<
        AllocateTransitVirtualInterfaceResult,
        RusotoError<AllocateTransitVirtualInterfaceError>,
    >;

    /// <p>Associates an existing connection with a link aggregation group (LAG). The connection is interrupted and re-established as a member of the LAG (connectivity to AWS is interrupted). The connection must be hosted on the same AWS Direct Connect endpoint as the LAG, and its bandwidth must match the bandwidth for the LAG. You can re-associate a connection that's currently associated with a different LAG; however, if removing the connection would cause the original LAG to fall below its setting for minimum number of operational connections, the request fails.</p> <p>Any virtual interfaces that are directly associated with the connection are automatically re-associated with the LAG. If the connection was originally associated with a different LAG, the virtual interfaces remain associated with the original LAG.</p> <p>For interconnects, any hosted connections are automatically re-associated with the LAG. If the interconnect was originally associated with a different LAG, the hosted connections remain associated with the original LAG.</p>
    async fn associate_connection_with_lag(
        &self,
        input: AssociateConnectionWithLagRequest,
    ) -> Result<Connection, RusotoError<AssociateConnectionWithLagError>>;

    /// <p><p>Associates a hosted connection and its virtual interfaces with a link aggregation group (LAG) or interconnect. If the target interconnect or LAG has an existing hosted connection with a conflicting VLAN number or IP address, the operation fails. This action temporarily interrupts the hosted connection&#39;s connectivity to AWS as it is being migrated.</p> <note> <p>Intended for use by AWS Direct Connect Partners only.</p> </note></p>
    async fn associate_hosted_connection(
        &self,
        input: AssociateHostedConnectionRequest,
    ) -> Result<Connection, RusotoError<AssociateHostedConnectionError>>;

    /// <p>Associates a virtual interface with a specified link aggregation group (LAG) or connection. Connectivity to AWS is temporarily interrupted as the virtual interface is being migrated. If the target connection or LAG has an associated virtual interface with a conflicting VLAN number or a conflicting IP address, the operation fails.</p> <p>Virtual interfaces associated with a hosted connection cannot be associated with a LAG; hosted connections must be migrated along with their virtual interfaces using <a>AssociateHostedConnection</a>.</p> <p>To reassociate a virtual interface to a new connection or LAG, the requester must own either the virtual interface itself or the connection to which the virtual interface is currently associated. Additionally, the requester must own the connection or LAG for the association.</p>
    async fn associate_virtual_interface(
        &self,
        input: AssociateVirtualInterfaceRequest,
    ) -> Result<VirtualInterface, RusotoError<AssociateVirtualInterfaceError>>;

    /// <p>Confirms the creation of the specified hosted connection on an interconnect.</p> <p>Upon creation, the hosted connection is initially in the <code>Ordering</code> state, and remains in this state until the owner confirms creation of the hosted connection.</p>
    async fn confirm_connection(
        &self,
        input: ConfirmConnectionRequest,
    ) -> Result<ConfirmConnectionResponse, RusotoError<ConfirmConnectionError>>;

    /// <p>Accepts ownership of a private virtual interface created by another AWS account.</p> <p>After the virtual interface owner makes this call, the virtual interface is created and attached to the specified virtual private gateway or Direct Connect gateway, and is made available to handle traffic.</p>
    async fn confirm_private_virtual_interface(
        &self,
        input: ConfirmPrivateVirtualInterfaceRequest,
    ) -> Result<
        ConfirmPrivateVirtualInterfaceResponse,
        RusotoError<ConfirmPrivateVirtualInterfaceError>,
    >;

    /// <p>Accepts ownership of a public virtual interface created by another AWS account.</p> <p>After the virtual interface owner makes this call, the specified virtual interface is created and made available to handle traffic.</p>
    async fn confirm_public_virtual_interface(
        &self,
        input: ConfirmPublicVirtualInterfaceRequest,
    ) -> Result<
        ConfirmPublicVirtualInterfaceResponse,
        RusotoError<ConfirmPublicVirtualInterfaceError>,
    >;

    /// <p>Accepts ownership of a transit virtual interface created by another AWS account.</p> <p> After the owner of the transit virtual interface makes this call, the specified transit virtual interface is created and made available to handle traffic.</p>
    async fn confirm_transit_virtual_interface(
        &self,
        input: ConfirmTransitVirtualInterfaceRequest,
    ) -> Result<
        ConfirmTransitVirtualInterfaceResponse,
        RusotoError<ConfirmTransitVirtualInterfaceError>,
    >;

    /// <p>Creates a BGP peer on the specified virtual interface.</p> <p>You must create a BGP peer for the corresponding address family (IPv4/IPv6) in order to access AWS resources that also use that address family.</p> <p>If logical redundancy is not supported by the connection, interconnect, or LAG, the BGP peer cannot be in the same address family as an existing BGP peer on the virtual interface.</p> <p>When creating a IPv6 BGP peer, omit the Amazon address and customer address. IPv6 addresses are automatically assigned from the Amazon pool of IPv6 addresses; you cannot specify custom IPv6 addresses.</p> <p>For a public virtual interface, the Autonomous System Number (ASN) must be private or already whitelisted for the virtual interface.</p>
    async fn create_bgp_peer(
        &self,
        input: CreateBGPPeerRequest,
    ) -> Result<CreateBGPPeerResponse, RusotoError<CreateBGPPeerError>>;

    /// <p>Creates a connection between a customer network and a specific AWS Direct Connect location.</p> <p>A connection links your internal network to an AWS Direct Connect location over a standard Ethernet fiber-optic cable. One end of the cable is connected to your router, the other to an AWS Direct Connect router.</p> <p>To find the locations for your Region, use <a>DescribeLocations</a>.</p> <p>You can automatically add the new connection to a link aggregation group (LAG) by specifying a LAG ID in the request. This ensures that the new connection is allocated on the same AWS Direct Connect endpoint that hosts the specified LAG. If there are no available ports on the endpoint, the request fails and no connection is created.</p>
    async fn create_connection(
        &self,
        input: CreateConnectionRequest,
    ) -> Result<Connection, RusotoError<CreateConnectionError>>;

    /// <p>Creates a Direct Connect gateway, which is an intermediate object that enables you to connect a set of virtual interfaces and virtual private gateways. A Direct Connect gateway is global and visible in any AWS Region after it is created. The virtual interfaces and virtual private gateways that are connected through a Direct Connect gateway can be in different AWS Regions. This enables you to connect to a VPC in any Region, regardless of the Region in which the virtual interfaces are located, and pass traffic between them.</p>
    async fn create_direct_connect_gateway(
        &self,
        input: CreateDirectConnectGatewayRequest,
    ) -> Result<CreateDirectConnectGatewayResult, RusotoError<CreateDirectConnectGatewayError>>;

    /// <p>Creates an association between a Direct Connect gateway and a virtual private gateway. The virtual private gateway must be attached to a VPC and must not be associated with another Direct Connect gateway.</p>
    async fn create_direct_connect_gateway_association(
        &self,
        input: CreateDirectConnectGatewayAssociationRequest,
    ) -> Result<
        CreateDirectConnectGatewayAssociationResult,
        RusotoError<CreateDirectConnectGatewayAssociationError>,
    >;

    /// <p>Creates a proposal to associate the specified virtual private gateway or transit gateway with the specified Direct Connect gateway.</p> <p>You can only associate a Direct Connect gateway and virtual private gateway or transit gateway when the account that owns the Direct Connect gateway and the account that owns the virtual private gateway or transit gateway have the same AWS Payer ID.</p>
    async fn create_direct_connect_gateway_association_proposal(
        &self,
        input: CreateDirectConnectGatewayAssociationProposalRequest,
    ) -> Result<
        CreateDirectConnectGatewayAssociationProposalResult,
        RusotoError<CreateDirectConnectGatewayAssociationProposalError>,
    >;

    /// <p><p>Creates an interconnect between an AWS Direct Connect Partner&#39;s network and a specific AWS Direct Connect location.</p> <p>An interconnect is a connection that is capable of hosting other connections. The AWS Direct Connect partner can use an interconnect to provide AWS Direct Connect hosted connections to customers through their own network services. Like a standard connection, an interconnect links the partner&#39;s network to an AWS Direct Connect location over a standard Ethernet fiber-optic cable. One end is connected to the partner&#39;s router, the other to an AWS Direct Connect router.</p> <p>You can automatically add the new interconnect to a link aggregation group (LAG) by specifying a LAG ID in the request. This ensures that the new interconnect is allocated on the same AWS Direct Connect endpoint that hosts the specified LAG. If there are no available ports on the endpoint, the request fails and no interconnect is created.</p> <p>For each end customer, the AWS Direct Connect Partner provisions a connection on their interconnect by calling <a>AllocateHostedConnection</a>. The end customer can then connect to AWS resources by creating a virtual interface on their connection, using the VLAN assigned to them by the AWS Direct Connect Partner.</p> <note> <p>Intended for use by AWS Direct Connect Partners only.</p> </note></p>
    async fn create_interconnect(
        &self,
        input: CreateInterconnectRequest,
    ) -> Result<Interconnect, RusotoError<CreateInterconnectError>>;

    /// <p>Creates a link aggregation group (LAG) with the specified number of bundled physical connections between the customer network and a specific AWS Direct Connect location. A LAG is a logical interface that uses the Link Aggregation Control Protocol (LACP) to aggregate multiple interfaces, enabling you to treat them as a single interface.</p> <p>All connections in a LAG must use the same bandwidth and must terminate at the same AWS Direct Connect endpoint.</p> <p>You can have up to 10 connections per LAG. Regardless of this limit, if you request more connections for the LAG than AWS Direct Connect can allocate on a single endpoint, no LAG is created.</p> <p>You can specify an existing physical connection or interconnect to include in the LAG (which counts towards the total number of connections). Doing so interrupts the current physical connection or hosted connections, and re-establishes them as a member of the LAG. The LAG will be created on the same AWS Direct Connect endpoint to which the connection terminates. Any virtual interfaces associated with the connection are automatically disassociated and re-associated with the LAG. The connection ID does not change.</p> <p>If the AWS account used to create a LAG is a registered AWS Direct Connect Partner, the LAG is automatically enabled to host sub-connections. For a LAG owned by a partner, any associated virtual interfaces cannot be directly configured.</p>
    async fn create_lag(&self, input: CreateLagRequest)
        -> Result<Lag, RusotoError<CreateLagError>>;

    /// <p>Creates a private virtual interface. A virtual interface is the VLAN that transports AWS Direct Connect traffic. A private virtual interface can be connected to either a Direct Connect gateway or a Virtual Private Gateway (VGW). Connecting the private virtual interface to a Direct Connect gateway enables the possibility for connecting to multiple VPCs, including VPCs in different AWS Regions. Connecting the private virtual interface to a VGW only provides access to a single VPC within the same Region.</p>
    async fn create_private_virtual_interface(
        &self,
        input: CreatePrivateVirtualInterfaceRequest,
    ) -> Result<VirtualInterface, RusotoError<CreatePrivateVirtualInterfaceError>>;

    /// <p>Creates a public virtual interface. A virtual interface is the VLAN that transports AWS Direct Connect traffic. A public virtual interface supports sending traffic to public services of AWS such as Amazon S3.</p> <p>When creating an IPv6 public virtual interface (<code>addressFamily</code> is <code>ipv6</code>), leave the <code>customer</code> and <code>amazon</code> address fields blank to use auto-assigned IPv6 space. Custom IPv6 addresses are not supported.</p>
    async fn create_public_virtual_interface(
        &self,
        input: CreatePublicVirtualInterfaceRequest,
    ) -> Result<VirtualInterface, RusotoError<CreatePublicVirtualInterfaceError>>;

    /// <p><p>Creates a transit virtual interface. A transit virtual interface should be used to access one or more transit gateways associated with Direct Connect gateways. A transit virtual interface enables the connection of multiple VPCs attached to a transit gateway to a Direct Connect gateway.</p> <important> <p>If you associate your transit gateway with one or more Direct Connect gateways, the Autonomous System Number (ASN) used by the transit gateway and the Direct Connect gateway must be different. For example, if you use the default ASN 64512 for both your the transit gateway and Direct Connect gateway, the association request fails.</p> </important></p>
    async fn create_transit_virtual_interface(
        &self,
        input: CreateTransitVirtualInterfaceRequest,
    ) -> Result<CreateTransitVirtualInterfaceResult, RusotoError<CreateTransitVirtualInterfaceError>>;

    /// <p>Deletes the specified BGP peer on the specified virtual interface with the specified customer address and ASN.</p> <p>You cannot delete the last BGP peer from a virtual interface.</p>
    async fn delete_bgp_peer(
        &self,
        input: DeleteBGPPeerRequest,
    ) -> Result<DeleteBGPPeerResponse, RusotoError<DeleteBGPPeerError>>;

    /// <p>Deletes the specified connection.</p> <p>Deleting a connection only stops the AWS Direct Connect port hour and data transfer charges. If you are partnering with any third parties to connect with the AWS Direct Connect location, you must cancel your service with them separately.</p>
    async fn delete_connection(
        &self,
        input: DeleteConnectionRequest,
    ) -> Result<Connection, RusotoError<DeleteConnectionError>>;

    /// <p>Deletes the specified Direct Connect gateway. You must first delete all virtual interfaces that are attached to the Direct Connect gateway and disassociate all virtual private gateways associated with the Direct Connect gateway.</p>
    async fn delete_direct_connect_gateway(
        &self,
        input: DeleteDirectConnectGatewayRequest,
    ) -> Result<DeleteDirectConnectGatewayResult, RusotoError<DeleteDirectConnectGatewayError>>;

    /// <p>Deletes the association between the specified Direct Connect gateway and virtual private gateway.</p> <p>We recommend that you specify the <code>associationID</code> to delete the association. Alternatively, if you own virtual gateway and a Direct Connect gateway association, you can specify the <code>virtualGatewayId</code> and <code>directConnectGatewayId</code> to delete an association.</p>
    async fn delete_direct_connect_gateway_association(
        &self,
        input: DeleteDirectConnectGatewayAssociationRequest,
    ) -> Result<
        DeleteDirectConnectGatewayAssociationResult,
        RusotoError<DeleteDirectConnectGatewayAssociationError>,
    >;

    /// <p>Deletes the association proposal request between the specified Direct Connect gateway and virtual private gateway or transit gateway.</p>
    async fn delete_direct_connect_gateway_association_proposal(
        &self,
        input: DeleteDirectConnectGatewayAssociationProposalRequest,
    ) -> Result<
        DeleteDirectConnectGatewayAssociationProposalResult,
        RusotoError<DeleteDirectConnectGatewayAssociationProposalError>,
    >;

    /// <p><p>Deletes the specified interconnect.</p> <note> <p>Intended for use by AWS Direct Connect Partners only.</p> </note></p>
    async fn delete_interconnect(
        &self,
        input: DeleteInterconnectRequest,
    ) -> Result<DeleteInterconnectResponse, RusotoError<DeleteInterconnectError>>;

    /// <p>Deletes the specified link aggregation group (LAG). You cannot delete a LAG if it has active virtual interfaces or hosted connections.</p>
    async fn delete_lag(&self, input: DeleteLagRequest)
        -> Result<Lag, RusotoError<DeleteLagError>>;

    /// <p>Deletes a virtual interface.</p>
    async fn delete_virtual_interface(
        &self,
        input: DeleteVirtualInterfaceRequest,
    ) -> Result<DeleteVirtualInterfaceResponse, RusotoError<DeleteVirtualInterfaceError>>;

    /// <p>Deprecated. Use <a>DescribeLoa</a> instead.</p> <p>Gets the LOA-CFA for a connection.</p> <p>The Letter of Authorization - Connecting Facility Assignment (LOA-CFA) is a document that your APN partner or service provider uses when establishing your cross connect to AWS at the colocation facility. For more information, see <a href="https://docs.aws.amazon.com/directconnect/latest/UserGuide/Colocation.html">Requesting Cross Connects at AWS Direct Connect Locations</a> in the <i>AWS Direct Connect User Guide</i>.</p>
    async fn describe_connection_loa(
        &self,
        input: DescribeConnectionLoaRequest,
    ) -> Result<DescribeConnectionLoaResponse, RusotoError<DescribeConnectionLoaError>>;

    /// <p>Displays the specified connection or all connections in this Region.</p>
    async fn describe_connections(
        &self,
        input: DescribeConnectionsRequest,
    ) -> Result<Connections, RusotoError<DescribeConnectionsError>>;

    /// <p><p>Deprecated. Use <a>DescribeHostedConnections</a> instead.</p> <p>Lists the connections that have been provisioned on the specified interconnect.</p> <note> <p>Intended for use by AWS Direct Connect Partners only.</p> </note></p>
    async fn describe_connections_on_interconnect(
        &self,
        input: DescribeConnectionsOnInterconnectRequest,
    ) -> Result<Connections, RusotoError<DescribeConnectionsOnInterconnectError>>;

    /// <p>Describes one or more association proposals for connection between a virtual private gateway or transit gateway and a Direct Connect gateway. </p>
    async fn describe_direct_connect_gateway_association_proposals(
        &self,
        input: DescribeDirectConnectGatewayAssociationProposalsRequest,
    ) -> Result<
        DescribeDirectConnectGatewayAssociationProposalsResult,
        RusotoError<DescribeDirectConnectGatewayAssociationProposalsError>,
    >;

    /// <p>Lists the associations between your Direct Connect gateways and virtual private gateways. You must specify a Direct Connect gateway, a virtual private gateway, or both. If you specify a Direct Connect gateway, the response contains all virtual private gateways associated with the Direct Connect gateway. If you specify a virtual private gateway, the response contains all Direct Connect gateways associated with the virtual private gateway. If you specify both, the response contains the association between the Direct Connect gateway and the virtual private gateway.</p>
    async fn describe_direct_connect_gateway_associations(
        &self,
        input: DescribeDirectConnectGatewayAssociationsRequest,
    ) -> Result<
        DescribeDirectConnectGatewayAssociationsResult,
        RusotoError<DescribeDirectConnectGatewayAssociationsError>,
    >;

    /// <p>Lists the attachments between your Direct Connect gateways and virtual interfaces. You must specify a Direct Connect gateway, a virtual interface, or both. If you specify a Direct Connect gateway, the response contains all virtual interfaces attached to the Direct Connect gateway. If you specify a virtual interface, the response contains all Direct Connect gateways attached to the virtual interface. If you specify both, the response contains the attachment between the Direct Connect gateway and the virtual interface.</p>
    async fn describe_direct_connect_gateway_attachments(
        &self,
        input: DescribeDirectConnectGatewayAttachmentsRequest,
    ) -> Result<
        DescribeDirectConnectGatewayAttachmentsResult,
        RusotoError<DescribeDirectConnectGatewayAttachmentsError>,
    >;

    /// <p>Lists all your Direct Connect gateways or only the specified Direct Connect gateway. Deleted Direct Connect gateways are not returned.</p>
    async fn describe_direct_connect_gateways(
        &self,
        input: DescribeDirectConnectGatewaysRequest,
    ) -> Result<DescribeDirectConnectGatewaysResult, RusotoError<DescribeDirectConnectGatewaysError>>;

    /// <p><p>Lists the hosted connections that have been provisioned on the specified interconnect or link aggregation group (LAG).</p> <note> <p>Intended for use by AWS Direct Connect Partners only.</p> </note></p>
    async fn describe_hosted_connections(
        &self,
        input: DescribeHostedConnectionsRequest,
    ) -> Result<Connections, RusotoError<DescribeHostedConnectionsError>>;

    /// <p>Deprecated. Use <a>DescribeLoa</a> instead.</p> <p>Gets the LOA-CFA for the specified interconnect.</p> <p>The Letter of Authorization - Connecting Facility Assignment (LOA-CFA) is a document that is used when establishing your cross connect to AWS at the colocation facility. For more information, see <a href="https://docs.aws.amazon.com/directconnect/latest/UserGuide/Colocation.html">Requesting Cross Connects at AWS Direct Connect Locations</a> in the <i>AWS Direct Connect User Guide</i>.</p>
    async fn describe_interconnect_loa(
        &self,
        input: DescribeInterconnectLoaRequest,
    ) -> Result<DescribeInterconnectLoaResponse, RusotoError<DescribeInterconnectLoaError>>;

    /// <p>Lists the interconnects owned by the AWS account or only the specified interconnect.</p>
    async fn describe_interconnects(
        &self,
        input: DescribeInterconnectsRequest,
    ) -> Result<Interconnects, RusotoError<DescribeInterconnectsError>>;

    /// <p>Describes all your link aggregation groups (LAG) or the specified LAG.</p>
    async fn describe_lags(
        &self,
        input: DescribeLagsRequest,
    ) -> Result<Lags, RusotoError<DescribeLagsError>>;

    /// <p>Gets the LOA-CFA for a connection, interconnect, or link aggregation group (LAG).</p> <p>The Letter of Authorization - Connecting Facility Assignment (LOA-CFA) is a document that is used when establishing your cross connect to AWS at the colocation facility. For more information, see <a href="https://docs.aws.amazon.com/directconnect/latest/UserGuide/Colocation.html">Requesting Cross Connects at AWS Direct Connect Locations</a> in the <i>AWS Direct Connect User Guide</i>.</p>
    async fn describe_loa(
        &self,
        input: DescribeLoaRequest,
    ) -> Result<Loa, RusotoError<DescribeLoaError>>;

    /// <p>Lists the AWS Direct Connect locations in the current AWS Region. These are the locations that can be selected when calling <a>CreateConnection</a> or <a>CreateInterconnect</a>.</p>
    async fn describe_locations(&self) -> Result<Locations, RusotoError<DescribeLocationsError>>;

    /// <p>Describes the tags associated with the specified AWS Direct Connect resources.</p>
    async fn describe_tags(
        &self,
        input: DescribeTagsRequest,
    ) -> Result<DescribeTagsResponse, RusotoError<DescribeTagsError>>;

    /// <p>Lists the virtual private gateways owned by the AWS account.</p> <p>You can create one or more AWS Direct Connect private virtual interfaces linked to a virtual private gateway.</p>
    async fn describe_virtual_gateways(
        &self,
    ) -> Result<VirtualGateways, RusotoError<DescribeVirtualGatewaysError>>;

    /// <p>Displays all virtual interfaces for an AWS account. Virtual interfaces deleted fewer than 15 minutes before you make the request are also returned. If you specify a connection ID, only the virtual interfaces associated with the connection are returned. If you specify a virtual interface ID, then only a single virtual interface is returned.</p> <p>A virtual interface (VLAN) transmits the traffic between the AWS Direct Connect location and the customer network.</p>
    async fn describe_virtual_interfaces(
        &self,
        input: DescribeVirtualInterfacesRequest,
    ) -> Result<VirtualInterfaces, RusotoError<DescribeVirtualInterfacesError>>;

    /// <p>Disassociates a connection from a link aggregation group (LAG). The connection is interrupted and re-established as a standalone connection (the connection is not deleted; to delete the connection, use the <a>DeleteConnection</a> request). If the LAG has associated virtual interfaces or hosted connections, they remain associated with the LAG. A disassociated connection owned by an AWS Direct Connect Partner is automatically converted to an interconnect.</p> <p>If disassociating the connection would cause the LAG to fall below its setting for minimum number of operational connections, the request fails, except when it's the last member of the LAG. If all connections are disassociated, the LAG continues to exist as an empty LAG with no physical connections. </p>
    async fn disassociate_connection_from_lag(
        &self,
        input: DisassociateConnectionFromLagRequest,
    ) -> Result<Connection, RusotoError<DisassociateConnectionFromLagError>>;

    /// <p>Adds the specified tags to the specified AWS Direct Connect resource. Each resource can have a maximum of 50 tags.</p> <p>Each tag consists of a key and an optional value. If a tag with the same key is already associated with the resource, this action updates its value.</p>
    async fn tag_resource(
        &self,
        input: TagResourceRequest,
    ) -> Result<TagResourceResponse, RusotoError<TagResourceError>>;

    /// <p>Removes one or more tags from the specified AWS Direct Connect resource.</p>
    async fn untag_resource(
        &self,
        input: UntagResourceRequest,
    ) -> Result<UntagResourceResponse, RusotoError<UntagResourceError>>;

    /// <p>Updates the specified attributes of the Direct Connect gateway association.</p> <p>Add or remove prefixes from the association.</p>
    async fn update_direct_connect_gateway_association(
        &self,
        input: UpdateDirectConnectGatewayAssociationRequest,
    ) -> Result<
        UpdateDirectConnectGatewayAssociationResult,
        RusotoError<UpdateDirectConnectGatewayAssociationError>,
    >;

    /// <p>Updates the attributes of the specified link aggregation group (LAG).</p> <p>You can update the following attributes:</p> <ul> <li> <p>The name of the LAG.</p> </li> <li> <p>The value for the minimum number of connections that must be operational for the LAG itself to be operational. </p> </li> </ul> <p>When you create a LAG, the default value for the minimum number of operational connections is zero (0). If you update this value and the number of operational connections falls below the specified value, the LAG automatically goes down to avoid over-utilization of the remaining connections. Adjust this value with care, as it could force the LAG down if it is set higher than the current number of operational connections.</p>
    async fn update_lag(&self, input: UpdateLagRequest)
        -> Result<Lag, RusotoError<UpdateLagError>>;

    /// <p>Updates the specified attributes of the specified virtual private interface.</p> <p>Setting the MTU of a virtual interface to 9001 (jumbo frames) can cause an update to the underlying physical connection if it wasn't updated to support jumbo frames. Updating the connection disrupts network connectivity for all virtual interfaces associated with the connection for up to 30 seconds. To check whether your connection supports jumbo frames, call <a>DescribeConnections</a>. To check whether your virtual interface supports jumbo frames, call <a>DescribeVirtualInterfaces</a>.</p>
    async fn update_virtual_interface_attributes(
        &self,
        input: UpdateVirtualInterfaceAttributesRequest,
    ) -> Result<VirtualInterface, RusotoError<UpdateVirtualInterfaceAttributesError>>;
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
            region,
        }
    }

    pub fn new_with<P, D>(
        request_dispatcher: D,
        credentials_provider: P,
        region: region::Region,
    ) -> DirectConnectClient
    where
        P: ProvideAwsCredentials + Send + Sync + 'static,
        D: DispatchSignedRequest + Send + Sync + 'static,
    {
        DirectConnectClient {
            client: Client::new_with(credentials_provider, request_dispatcher),
            region,
        }
    }

    pub fn new_with_client(client: Client, region: region::Region) -> DirectConnectClient {
        DirectConnectClient { client, region }
    }
}

#[async_trait]
impl DirectConnect for DirectConnectClient {
    /// <p>Accepts a proposal request to attach a virtual private gateway or transit gateway to a Direct Connect gateway.</p>
    async fn accept_direct_connect_gateway_association_proposal(
        &self,
        input: AcceptDirectConnectGatewayAssociationProposalRequest,
    ) -> Result<
        AcceptDirectConnectGatewayAssociationProposalResult,
        RusotoError<AcceptDirectConnectGatewayAssociationProposalError>,
    > {
        let mut request = SignedRequest::new("POST", "directconnect", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "OvertureService.AcceptDirectConnectGatewayAssociationProposal",
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
                .deserialize::<AcceptDirectConnectGatewayAssociationProposalResult, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(AcceptDirectConnectGatewayAssociationProposalError::from_response(response))
        }
    }

    /// <p><p>Deprecated. Use <a>AllocateHostedConnection</a> instead.</p> <p>Creates a hosted connection on an interconnect.</p> <p>Allocates a VLAN number and a specified amount of bandwidth for use by a hosted connection on the specified interconnect.</p> <note> <p>Intended for use by AWS Direct Connect Partners only.</p> </note></p>
    async fn allocate_connection_on_interconnect(
        &self,
        input: AllocateConnectionOnInterconnectRequest,
    ) -> Result<Connection, RusotoError<AllocateConnectionOnInterconnectError>> {
        let mut request = SignedRequest::new("POST", "directconnect", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "OvertureService.AllocateConnectionOnInterconnect",
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
            proto::json::ResponsePayload::new(&response).deserialize::<Connection, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(AllocateConnectionOnInterconnectError::from_response(
                response,
            ))
        }
    }

    /// <p><p>Creates a hosted connection on the specified interconnect or a link aggregation group (LAG) of interconnects.</p> <p>Allocates a VLAN number and a specified amount of capacity (bandwidth) for use by a hosted connection on the specified interconnect or LAG of interconnects. AWS polices the hosted connection for the specified capacity and the AWS Direct Connect Partner must also police the hosted connection for the specified capacity.</p> <note> <p>Intended for use by AWS Direct Connect Partners only.</p> </note></p>
    async fn allocate_hosted_connection(
        &self,
        input: AllocateHostedConnectionRequest,
    ) -> Result<Connection, RusotoError<AllocateHostedConnectionError>> {
        let mut request = SignedRequest::new("POST", "directconnect", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "OvertureService.AllocateHostedConnection");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response).deserialize::<Connection, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(AllocateHostedConnectionError::from_response(response))
        }
    }

    /// <p>Provisions a private virtual interface to be owned by the specified AWS account.</p> <p>Virtual interfaces created using this action must be confirmed by the owner using <a>ConfirmPrivateVirtualInterface</a>. Until then, the virtual interface is in the <code>Confirming</code> state and is not available to handle traffic.</p>
    async fn allocate_private_virtual_interface(
        &self,
        input: AllocatePrivateVirtualInterfaceRequest,
    ) -> Result<VirtualInterface, RusotoError<AllocatePrivateVirtualInterfaceError>> {
        let mut request = SignedRequest::new("POST", "directconnect", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "OvertureService.AllocatePrivateVirtualInterface",
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
            proto::json::ResponsePayload::new(&response).deserialize::<VirtualInterface, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(AllocatePrivateVirtualInterfaceError::from_response(
                response,
            ))
        }
    }

    /// <p>Provisions a public virtual interface to be owned by the specified AWS account.</p> <p>The owner of a connection calls this function to provision a public virtual interface to be owned by the specified AWS account.</p> <p>Virtual interfaces created using this function must be confirmed by the owner using <a>ConfirmPublicVirtualInterface</a>. Until this step has been completed, the virtual interface is in the <code>confirming</code> state and is not available to handle traffic.</p> <p>When creating an IPv6 public virtual interface, omit the Amazon address and customer address. IPv6 addresses are automatically assigned from the Amazon pool of IPv6 addresses; you cannot specify custom IPv6 addresses.</p>
    async fn allocate_public_virtual_interface(
        &self,
        input: AllocatePublicVirtualInterfaceRequest,
    ) -> Result<VirtualInterface, RusotoError<AllocatePublicVirtualInterfaceError>> {
        let mut request = SignedRequest::new("POST", "directconnect", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "OvertureService.AllocatePublicVirtualInterface",
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
            proto::json::ResponsePayload::new(&response).deserialize::<VirtualInterface, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(AllocatePublicVirtualInterfaceError::from_response(response))
        }
    }

    /// <p>Provisions a transit virtual interface to be owned by the specified AWS account. Use this type of interface to connect a transit gateway to your Direct Connect gateway.</p> <p>The owner of a connection provisions a transit virtual interface to be owned by the specified AWS account.</p> <p>After you create a transit virtual interface, it must be confirmed by the owner using <a>ConfirmTransitVirtualInterface</a>. Until this step has been completed, the transit virtual interface is in the <code>requested</code> state and is not available to handle traffic.</p>
    async fn allocate_transit_virtual_interface(
        &self,
        input: AllocateTransitVirtualInterfaceRequest,
    ) -> Result<
        AllocateTransitVirtualInterfaceResult,
        RusotoError<AllocateTransitVirtualInterfaceError>,
    > {
        let mut request = SignedRequest::new("POST", "directconnect", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "OvertureService.AllocateTransitVirtualInterface",
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
                .deserialize::<AllocateTransitVirtualInterfaceResult, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(AllocateTransitVirtualInterfaceError::from_response(
                response,
            ))
        }
    }

    /// <p>Associates an existing connection with a link aggregation group (LAG). The connection is interrupted and re-established as a member of the LAG (connectivity to AWS is interrupted). The connection must be hosted on the same AWS Direct Connect endpoint as the LAG, and its bandwidth must match the bandwidth for the LAG. You can re-associate a connection that's currently associated with a different LAG; however, if removing the connection would cause the original LAG to fall below its setting for minimum number of operational connections, the request fails.</p> <p>Any virtual interfaces that are directly associated with the connection are automatically re-associated with the LAG. If the connection was originally associated with a different LAG, the virtual interfaces remain associated with the original LAG.</p> <p>For interconnects, any hosted connections are automatically re-associated with the LAG. If the interconnect was originally associated with a different LAG, the hosted connections remain associated with the original LAG.</p>
    async fn associate_connection_with_lag(
        &self,
        input: AssociateConnectionWithLagRequest,
    ) -> Result<Connection, RusotoError<AssociateConnectionWithLagError>> {
        let mut request = SignedRequest::new("POST", "directconnect", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "OvertureService.AssociateConnectionWithLag");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response).deserialize::<Connection, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(AssociateConnectionWithLagError::from_response(response))
        }
    }

    /// <p><p>Associates a hosted connection and its virtual interfaces with a link aggregation group (LAG) or interconnect. If the target interconnect or LAG has an existing hosted connection with a conflicting VLAN number or IP address, the operation fails. This action temporarily interrupts the hosted connection&#39;s connectivity to AWS as it is being migrated.</p> <note> <p>Intended for use by AWS Direct Connect Partners only.</p> </note></p>
    async fn associate_hosted_connection(
        &self,
        input: AssociateHostedConnectionRequest,
    ) -> Result<Connection, RusotoError<AssociateHostedConnectionError>> {
        let mut request = SignedRequest::new("POST", "directconnect", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "OvertureService.AssociateHostedConnection");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response).deserialize::<Connection, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(AssociateHostedConnectionError::from_response(response))
        }
    }

    /// <p>Associates a virtual interface with a specified link aggregation group (LAG) or connection. Connectivity to AWS is temporarily interrupted as the virtual interface is being migrated. If the target connection or LAG has an associated virtual interface with a conflicting VLAN number or a conflicting IP address, the operation fails.</p> <p>Virtual interfaces associated with a hosted connection cannot be associated with a LAG; hosted connections must be migrated along with their virtual interfaces using <a>AssociateHostedConnection</a>.</p> <p>To reassociate a virtual interface to a new connection or LAG, the requester must own either the virtual interface itself or the connection to which the virtual interface is currently associated. Additionally, the requester must own the connection or LAG for the association.</p>
    async fn associate_virtual_interface(
        &self,
        input: AssociateVirtualInterfaceRequest,
    ) -> Result<VirtualInterface, RusotoError<AssociateVirtualInterfaceError>> {
        let mut request = SignedRequest::new("POST", "directconnect", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "OvertureService.AssociateVirtualInterface");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response).deserialize::<VirtualInterface, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(AssociateVirtualInterfaceError::from_response(response))
        }
    }

    /// <p>Confirms the creation of the specified hosted connection on an interconnect.</p> <p>Upon creation, the hosted connection is initially in the <code>Ordering</code> state, and remains in this state until the owner confirms creation of the hosted connection.</p>
    async fn confirm_connection(
        &self,
        input: ConfirmConnectionRequest,
    ) -> Result<ConfirmConnectionResponse, RusotoError<ConfirmConnectionError>> {
        let mut request = SignedRequest::new("POST", "directconnect", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "OvertureService.ConfirmConnection");
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
                .deserialize::<ConfirmConnectionResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(ConfirmConnectionError::from_response(response))
        }
    }

    /// <p>Accepts ownership of a private virtual interface created by another AWS account.</p> <p>After the virtual interface owner makes this call, the virtual interface is created and attached to the specified virtual private gateway or Direct Connect gateway, and is made available to handle traffic.</p>
    async fn confirm_private_virtual_interface(
        &self,
        input: ConfirmPrivateVirtualInterfaceRequest,
    ) -> Result<
        ConfirmPrivateVirtualInterfaceResponse,
        RusotoError<ConfirmPrivateVirtualInterfaceError>,
    > {
        let mut request = SignedRequest::new("POST", "directconnect", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "OvertureService.ConfirmPrivateVirtualInterface",
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
                .deserialize::<ConfirmPrivateVirtualInterfaceResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(ConfirmPrivateVirtualInterfaceError::from_response(response))
        }
    }

    /// <p>Accepts ownership of a public virtual interface created by another AWS account.</p> <p>After the virtual interface owner makes this call, the specified virtual interface is created and made available to handle traffic.</p>
    async fn confirm_public_virtual_interface(
        &self,
        input: ConfirmPublicVirtualInterfaceRequest,
    ) -> Result<
        ConfirmPublicVirtualInterfaceResponse,
        RusotoError<ConfirmPublicVirtualInterfaceError>,
    > {
        let mut request = SignedRequest::new("POST", "directconnect", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "OvertureService.ConfirmPublicVirtualInterface",
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
                .deserialize::<ConfirmPublicVirtualInterfaceResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(ConfirmPublicVirtualInterfaceError::from_response(response))
        }
    }

    /// <p>Accepts ownership of a transit virtual interface created by another AWS account.</p> <p> After the owner of the transit virtual interface makes this call, the specified transit virtual interface is created and made available to handle traffic.</p>
    async fn confirm_transit_virtual_interface(
        &self,
        input: ConfirmTransitVirtualInterfaceRequest,
    ) -> Result<
        ConfirmTransitVirtualInterfaceResponse,
        RusotoError<ConfirmTransitVirtualInterfaceError>,
    > {
        let mut request = SignedRequest::new("POST", "directconnect", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "OvertureService.ConfirmTransitVirtualInterface",
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
                .deserialize::<ConfirmTransitVirtualInterfaceResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(ConfirmTransitVirtualInterfaceError::from_response(response))
        }
    }

    /// <p>Creates a BGP peer on the specified virtual interface.</p> <p>You must create a BGP peer for the corresponding address family (IPv4/IPv6) in order to access AWS resources that also use that address family.</p> <p>If logical redundancy is not supported by the connection, interconnect, or LAG, the BGP peer cannot be in the same address family as an existing BGP peer on the virtual interface.</p> <p>When creating a IPv6 BGP peer, omit the Amazon address and customer address. IPv6 addresses are automatically assigned from the Amazon pool of IPv6 addresses; you cannot specify custom IPv6 addresses.</p> <p>For a public virtual interface, the Autonomous System Number (ASN) must be private or already whitelisted for the virtual interface.</p>
    async fn create_bgp_peer(
        &self,
        input: CreateBGPPeerRequest,
    ) -> Result<CreateBGPPeerResponse, RusotoError<CreateBGPPeerError>> {
        let mut request = SignedRequest::new("POST", "directconnect", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "OvertureService.CreateBGPPeer");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response).deserialize::<CreateBGPPeerResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(CreateBGPPeerError::from_response(response))
        }
    }

    /// <p>Creates a connection between a customer network and a specific AWS Direct Connect location.</p> <p>A connection links your internal network to an AWS Direct Connect location over a standard Ethernet fiber-optic cable. One end of the cable is connected to your router, the other to an AWS Direct Connect router.</p> <p>To find the locations for your Region, use <a>DescribeLocations</a>.</p> <p>You can automatically add the new connection to a link aggregation group (LAG) by specifying a LAG ID in the request. This ensures that the new connection is allocated on the same AWS Direct Connect endpoint that hosts the specified LAG. If there are no available ports on the endpoint, the request fails and no connection is created.</p>
    async fn create_connection(
        &self,
        input: CreateConnectionRequest,
    ) -> Result<Connection, RusotoError<CreateConnectionError>> {
        let mut request = SignedRequest::new("POST", "directconnect", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "OvertureService.CreateConnection");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response).deserialize::<Connection, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(CreateConnectionError::from_response(response))
        }
    }

    /// <p>Creates a Direct Connect gateway, which is an intermediate object that enables you to connect a set of virtual interfaces and virtual private gateways. A Direct Connect gateway is global and visible in any AWS Region after it is created. The virtual interfaces and virtual private gateways that are connected through a Direct Connect gateway can be in different AWS Regions. This enables you to connect to a VPC in any Region, regardless of the Region in which the virtual interfaces are located, and pass traffic between them.</p>
    async fn create_direct_connect_gateway(
        &self,
        input: CreateDirectConnectGatewayRequest,
    ) -> Result<CreateDirectConnectGatewayResult, RusotoError<CreateDirectConnectGatewayError>>
    {
        let mut request = SignedRequest::new("POST", "directconnect", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "OvertureService.CreateDirectConnectGateway");
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
                .deserialize::<CreateDirectConnectGatewayResult, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(CreateDirectConnectGatewayError::from_response(response))
        }
    }

    /// <p>Creates an association between a Direct Connect gateway and a virtual private gateway. The virtual private gateway must be attached to a VPC and must not be associated with another Direct Connect gateway.</p>
    async fn create_direct_connect_gateway_association(
        &self,
        input: CreateDirectConnectGatewayAssociationRequest,
    ) -> Result<
        CreateDirectConnectGatewayAssociationResult,
        RusotoError<CreateDirectConnectGatewayAssociationError>,
    > {
        let mut request = SignedRequest::new("POST", "directconnect", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "OvertureService.CreateDirectConnectGatewayAssociation",
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
                .deserialize::<CreateDirectConnectGatewayAssociationResult, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(CreateDirectConnectGatewayAssociationError::from_response(
                response,
            ))
        }
    }

    /// <p>Creates a proposal to associate the specified virtual private gateway or transit gateway with the specified Direct Connect gateway.</p> <p>You can only associate a Direct Connect gateway and virtual private gateway or transit gateway when the account that owns the Direct Connect gateway and the account that owns the virtual private gateway or transit gateway have the same AWS Payer ID.</p>
    async fn create_direct_connect_gateway_association_proposal(
        &self,
        input: CreateDirectConnectGatewayAssociationProposalRequest,
    ) -> Result<
        CreateDirectConnectGatewayAssociationProposalResult,
        RusotoError<CreateDirectConnectGatewayAssociationProposalError>,
    > {
        let mut request = SignedRequest::new("POST", "directconnect", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "OvertureService.CreateDirectConnectGatewayAssociationProposal",
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
                .deserialize::<CreateDirectConnectGatewayAssociationProposalResult, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(CreateDirectConnectGatewayAssociationProposalError::from_response(response))
        }
    }

    /// <p><p>Creates an interconnect between an AWS Direct Connect Partner&#39;s network and a specific AWS Direct Connect location.</p> <p>An interconnect is a connection that is capable of hosting other connections. The AWS Direct Connect partner can use an interconnect to provide AWS Direct Connect hosted connections to customers through their own network services. Like a standard connection, an interconnect links the partner&#39;s network to an AWS Direct Connect location over a standard Ethernet fiber-optic cable. One end is connected to the partner&#39;s router, the other to an AWS Direct Connect router.</p> <p>You can automatically add the new interconnect to a link aggregation group (LAG) by specifying a LAG ID in the request. This ensures that the new interconnect is allocated on the same AWS Direct Connect endpoint that hosts the specified LAG. If there are no available ports on the endpoint, the request fails and no interconnect is created.</p> <p>For each end customer, the AWS Direct Connect Partner provisions a connection on their interconnect by calling <a>AllocateHostedConnection</a>. The end customer can then connect to AWS resources by creating a virtual interface on their connection, using the VLAN assigned to them by the AWS Direct Connect Partner.</p> <note> <p>Intended for use by AWS Direct Connect Partners only.</p> </note></p>
    async fn create_interconnect(
        &self,
        input: CreateInterconnectRequest,
    ) -> Result<Interconnect, RusotoError<CreateInterconnectError>> {
        let mut request = SignedRequest::new("POST", "directconnect", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "OvertureService.CreateInterconnect");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response).deserialize::<Interconnect, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(CreateInterconnectError::from_response(response))
        }
    }

    /// <p>Creates a link aggregation group (LAG) with the specified number of bundled physical connections between the customer network and a specific AWS Direct Connect location. A LAG is a logical interface that uses the Link Aggregation Control Protocol (LACP) to aggregate multiple interfaces, enabling you to treat them as a single interface.</p> <p>All connections in a LAG must use the same bandwidth and must terminate at the same AWS Direct Connect endpoint.</p> <p>You can have up to 10 connections per LAG. Regardless of this limit, if you request more connections for the LAG than AWS Direct Connect can allocate on a single endpoint, no LAG is created.</p> <p>You can specify an existing physical connection or interconnect to include in the LAG (which counts towards the total number of connections). Doing so interrupts the current physical connection or hosted connections, and re-establishes them as a member of the LAG. The LAG will be created on the same AWS Direct Connect endpoint to which the connection terminates. Any virtual interfaces associated with the connection are automatically disassociated and re-associated with the LAG. The connection ID does not change.</p> <p>If the AWS account used to create a LAG is a registered AWS Direct Connect Partner, the LAG is automatically enabled to host sub-connections. For a LAG owned by a partner, any associated virtual interfaces cannot be directly configured.</p>
    async fn create_lag(
        &self,
        input: CreateLagRequest,
    ) -> Result<Lag, RusotoError<CreateLagError>> {
        let mut request = SignedRequest::new("POST", "directconnect", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "OvertureService.CreateLag");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response).deserialize::<Lag, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(CreateLagError::from_response(response))
        }
    }

    /// <p>Creates a private virtual interface. A virtual interface is the VLAN that transports AWS Direct Connect traffic. A private virtual interface can be connected to either a Direct Connect gateway or a Virtual Private Gateway (VGW). Connecting the private virtual interface to a Direct Connect gateway enables the possibility for connecting to multiple VPCs, including VPCs in different AWS Regions. Connecting the private virtual interface to a VGW only provides access to a single VPC within the same Region.</p>
    async fn create_private_virtual_interface(
        &self,
        input: CreatePrivateVirtualInterfaceRequest,
    ) -> Result<VirtualInterface, RusotoError<CreatePrivateVirtualInterfaceError>> {
        let mut request = SignedRequest::new("POST", "directconnect", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "OvertureService.CreatePrivateVirtualInterface",
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
            proto::json::ResponsePayload::new(&response).deserialize::<VirtualInterface, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(CreatePrivateVirtualInterfaceError::from_response(response))
        }
    }

    /// <p>Creates a public virtual interface. A virtual interface is the VLAN that transports AWS Direct Connect traffic. A public virtual interface supports sending traffic to public services of AWS such as Amazon S3.</p> <p>When creating an IPv6 public virtual interface (<code>addressFamily</code> is <code>ipv6</code>), leave the <code>customer</code> and <code>amazon</code> address fields blank to use auto-assigned IPv6 space. Custom IPv6 addresses are not supported.</p>
    async fn create_public_virtual_interface(
        &self,
        input: CreatePublicVirtualInterfaceRequest,
    ) -> Result<VirtualInterface, RusotoError<CreatePublicVirtualInterfaceError>> {
        let mut request = SignedRequest::new("POST", "directconnect", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "OvertureService.CreatePublicVirtualInterface",
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
            proto::json::ResponsePayload::new(&response).deserialize::<VirtualInterface, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(CreatePublicVirtualInterfaceError::from_response(response))
        }
    }

    /// <p><p>Creates a transit virtual interface. A transit virtual interface should be used to access one or more transit gateways associated with Direct Connect gateways. A transit virtual interface enables the connection of multiple VPCs attached to a transit gateway to a Direct Connect gateway.</p> <important> <p>If you associate your transit gateway with one or more Direct Connect gateways, the Autonomous System Number (ASN) used by the transit gateway and the Direct Connect gateway must be different. For example, if you use the default ASN 64512 for both your the transit gateway and Direct Connect gateway, the association request fails.</p> </important></p>
    async fn create_transit_virtual_interface(
        &self,
        input: CreateTransitVirtualInterfaceRequest,
    ) -> Result<CreateTransitVirtualInterfaceResult, RusotoError<CreateTransitVirtualInterfaceError>>
    {
        let mut request = SignedRequest::new("POST", "directconnect", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "OvertureService.CreateTransitVirtualInterface",
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
                .deserialize::<CreateTransitVirtualInterfaceResult, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(CreateTransitVirtualInterfaceError::from_response(response))
        }
    }

    /// <p>Deletes the specified BGP peer on the specified virtual interface with the specified customer address and ASN.</p> <p>You cannot delete the last BGP peer from a virtual interface.</p>
    async fn delete_bgp_peer(
        &self,
        input: DeleteBGPPeerRequest,
    ) -> Result<DeleteBGPPeerResponse, RusotoError<DeleteBGPPeerError>> {
        let mut request = SignedRequest::new("POST", "directconnect", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "OvertureService.DeleteBGPPeer");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response).deserialize::<DeleteBGPPeerResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(DeleteBGPPeerError::from_response(response))
        }
    }

    /// <p>Deletes the specified connection.</p> <p>Deleting a connection only stops the AWS Direct Connect port hour and data transfer charges. If you are partnering with any third parties to connect with the AWS Direct Connect location, you must cancel your service with them separately.</p>
    async fn delete_connection(
        &self,
        input: DeleteConnectionRequest,
    ) -> Result<Connection, RusotoError<DeleteConnectionError>> {
        let mut request = SignedRequest::new("POST", "directconnect", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "OvertureService.DeleteConnection");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response).deserialize::<Connection, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(DeleteConnectionError::from_response(response))
        }
    }

    /// <p>Deletes the specified Direct Connect gateway. You must first delete all virtual interfaces that are attached to the Direct Connect gateway and disassociate all virtual private gateways associated with the Direct Connect gateway.</p>
    async fn delete_direct_connect_gateway(
        &self,
        input: DeleteDirectConnectGatewayRequest,
    ) -> Result<DeleteDirectConnectGatewayResult, RusotoError<DeleteDirectConnectGatewayError>>
    {
        let mut request = SignedRequest::new("POST", "directconnect", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "OvertureService.DeleteDirectConnectGateway");
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
                .deserialize::<DeleteDirectConnectGatewayResult, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(DeleteDirectConnectGatewayError::from_response(response))
        }
    }

    /// <p>Deletes the association between the specified Direct Connect gateway and virtual private gateway.</p> <p>We recommend that you specify the <code>associationID</code> to delete the association. Alternatively, if you own virtual gateway and a Direct Connect gateway association, you can specify the <code>virtualGatewayId</code> and <code>directConnectGatewayId</code> to delete an association.</p>
    async fn delete_direct_connect_gateway_association(
        &self,
        input: DeleteDirectConnectGatewayAssociationRequest,
    ) -> Result<
        DeleteDirectConnectGatewayAssociationResult,
        RusotoError<DeleteDirectConnectGatewayAssociationError>,
    > {
        let mut request = SignedRequest::new("POST", "directconnect", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "OvertureService.DeleteDirectConnectGatewayAssociation",
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
                .deserialize::<DeleteDirectConnectGatewayAssociationResult, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(DeleteDirectConnectGatewayAssociationError::from_response(
                response,
            ))
        }
    }

    /// <p>Deletes the association proposal request between the specified Direct Connect gateway and virtual private gateway or transit gateway.</p>
    async fn delete_direct_connect_gateway_association_proposal(
        &self,
        input: DeleteDirectConnectGatewayAssociationProposalRequest,
    ) -> Result<
        DeleteDirectConnectGatewayAssociationProposalResult,
        RusotoError<DeleteDirectConnectGatewayAssociationProposalError>,
    > {
        let mut request = SignedRequest::new("POST", "directconnect", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "OvertureService.DeleteDirectConnectGatewayAssociationProposal",
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
                .deserialize::<DeleteDirectConnectGatewayAssociationProposalResult, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(DeleteDirectConnectGatewayAssociationProposalError::from_response(response))
        }
    }

    /// <p><p>Deletes the specified interconnect.</p> <note> <p>Intended for use by AWS Direct Connect Partners only.</p> </note></p>
    async fn delete_interconnect(
        &self,
        input: DeleteInterconnectRequest,
    ) -> Result<DeleteInterconnectResponse, RusotoError<DeleteInterconnectError>> {
        let mut request = SignedRequest::new("POST", "directconnect", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "OvertureService.DeleteInterconnect");
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
                .deserialize::<DeleteInterconnectResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(DeleteInterconnectError::from_response(response))
        }
    }

    /// <p>Deletes the specified link aggregation group (LAG). You cannot delete a LAG if it has active virtual interfaces or hosted connections.</p>
    async fn delete_lag(
        &self,
        input: DeleteLagRequest,
    ) -> Result<Lag, RusotoError<DeleteLagError>> {
        let mut request = SignedRequest::new("POST", "directconnect", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "OvertureService.DeleteLag");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response).deserialize::<Lag, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(DeleteLagError::from_response(response))
        }
    }

    /// <p>Deletes a virtual interface.</p>
    async fn delete_virtual_interface(
        &self,
        input: DeleteVirtualInterfaceRequest,
    ) -> Result<DeleteVirtualInterfaceResponse, RusotoError<DeleteVirtualInterfaceError>> {
        let mut request = SignedRequest::new("POST", "directconnect", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "OvertureService.DeleteVirtualInterface");
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
                .deserialize::<DeleteVirtualInterfaceResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(DeleteVirtualInterfaceError::from_response(response))
        }
    }

    /// <p>Deprecated. Use <a>DescribeLoa</a> instead.</p> <p>Gets the LOA-CFA for a connection.</p> <p>The Letter of Authorization - Connecting Facility Assignment (LOA-CFA) is a document that your APN partner or service provider uses when establishing your cross connect to AWS at the colocation facility. For more information, see <a href="https://docs.aws.amazon.com/directconnect/latest/UserGuide/Colocation.html">Requesting Cross Connects at AWS Direct Connect Locations</a> in the <i>AWS Direct Connect User Guide</i>.</p>
    async fn describe_connection_loa(
        &self,
        input: DescribeConnectionLoaRequest,
    ) -> Result<DescribeConnectionLoaResponse, RusotoError<DescribeConnectionLoaError>> {
        let mut request = SignedRequest::new("POST", "directconnect", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "OvertureService.DescribeConnectionLoa");
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
                .deserialize::<DescribeConnectionLoaResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(DescribeConnectionLoaError::from_response(response))
        }
    }

    /// <p>Displays the specified connection or all connections in this Region.</p>
    async fn describe_connections(
        &self,
        input: DescribeConnectionsRequest,
    ) -> Result<Connections, RusotoError<DescribeConnectionsError>> {
        let mut request = SignedRequest::new("POST", "directconnect", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "OvertureService.DescribeConnections");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response).deserialize::<Connections, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(DescribeConnectionsError::from_response(response))
        }
    }

    /// <p><p>Deprecated. Use <a>DescribeHostedConnections</a> instead.</p> <p>Lists the connections that have been provisioned on the specified interconnect.</p> <note> <p>Intended for use by AWS Direct Connect Partners only.</p> </note></p>
    async fn describe_connections_on_interconnect(
        &self,
        input: DescribeConnectionsOnInterconnectRequest,
    ) -> Result<Connections, RusotoError<DescribeConnectionsOnInterconnectError>> {
        let mut request = SignedRequest::new("POST", "directconnect", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "OvertureService.DescribeConnectionsOnInterconnect",
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
            proto::json::ResponsePayload::new(&response).deserialize::<Connections, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(DescribeConnectionsOnInterconnectError::from_response(
                response,
            ))
        }
    }

    /// <p>Describes one or more association proposals for connection between a virtual private gateway or transit gateway and a Direct Connect gateway. </p>
    async fn describe_direct_connect_gateway_association_proposals(
        &self,
        input: DescribeDirectConnectGatewayAssociationProposalsRequest,
    ) -> Result<
        DescribeDirectConnectGatewayAssociationProposalsResult,
        RusotoError<DescribeDirectConnectGatewayAssociationProposalsError>,
    > {
        let mut request = SignedRequest::new("POST", "directconnect", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "OvertureService.DescribeDirectConnectGatewayAssociationProposals",
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
                .deserialize::<DescribeDirectConnectGatewayAssociationProposalsResult, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(DescribeDirectConnectGatewayAssociationProposalsError::from_response(response))
        }
    }

    /// <p>Lists the associations between your Direct Connect gateways and virtual private gateways. You must specify a Direct Connect gateway, a virtual private gateway, or both. If you specify a Direct Connect gateway, the response contains all virtual private gateways associated with the Direct Connect gateway. If you specify a virtual private gateway, the response contains all Direct Connect gateways associated with the virtual private gateway. If you specify both, the response contains the association between the Direct Connect gateway and the virtual private gateway.</p>
    async fn describe_direct_connect_gateway_associations(
        &self,
        input: DescribeDirectConnectGatewayAssociationsRequest,
    ) -> Result<
        DescribeDirectConnectGatewayAssociationsResult,
        RusotoError<DescribeDirectConnectGatewayAssociationsError>,
    > {
        let mut request = SignedRequest::new("POST", "directconnect", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "OvertureService.DescribeDirectConnectGatewayAssociations",
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
                .deserialize::<DescribeDirectConnectGatewayAssociationsResult, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(DescribeDirectConnectGatewayAssociationsError::from_response(response))
        }
    }

    /// <p>Lists the attachments between your Direct Connect gateways and virtual interfaces. You must specify a Direct Connect gateway, a virtual interface, or both. If you specify a Direct Connect gateway, the response contains all virtual interfaces attached to the Direct Connect gateway. If you specify a virtual interface, the response contains all Direct Connect gateways attached to the virtual interface. If you specify both, the response contains the attachment between the Direct Connect gateway and the virtual interface.</p>
    async fn describe_direct_connect_gateway_attachments(
        &self,
        input: DescribeDirectConnectGatewayAttachmentsRequest,
    ) -> Result<
        DescribeDirectConnectGatewayAttachmentsResult,
        RusotoError<DescribeDirectConnectGatewayAttachmentsError>,
    > {
        let mut request = SignedRequest::new("POST", "directconnect", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "OvertureService.DescribeDirectConnectGatewayAttachments",
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
                .deserialize::<DescribeDirectConnectGatewayAttachmentsResult, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(DescribeDirectConnectGatewayAttachmentsError::from_response(
                response,
            ))
        }
    }

    /// <p>Lists all your Direct Connect gateways or only the specified Direct Connect gateway. Deleted Direct Connect gateways are not returned.</p>
    async fn describe_direct_connect_gateways(
        &self,
        input: DescribeDirectConnectGatewaysRequest,
    ) -> Result<DescribeDirectConnectGatewaysResult, RusotoError<DescribeDirectConnectGatewaysError>>
    {
        let mut request = SignedRequest::new("POST", "directconnect", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "OvertureService.DescribeDirectConnectGateways",
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
                .deserialize::<DescribeDirectConnectGatewaysResult, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(DescribeDirectConnectGatewaysError::from_response(response))
        }
    }

    /// <p><p>Lists the hosted connections that have been provisioned on the specified interconnect or link aggregation group (LAG).</p> <note> <p>Intended for use by AWS Direct Connect Partners only.</p> </note></p>
    async fn describe_hosted_connections(
        &self,
        input: DescribeHostedConnectionsRequest,
    ) -> Result<Connections, RusotoError<DescribeHostedConnectionsError>> {
        let mut request = SignedRequest::new("POST", "directconnect", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "OvertureService.DescribeHostedConnections");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response).deserialize::<Connections, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(DescribeHostedConnectionsError::from_response(response))
        }
    }

    /// <p>Deprecated. Use <a>DescribeLoa</a> instead.</p> <p>Gets the LOA-CFA for the specified interconnect.</p> <p>The Letter of Authorization - Connecting Facility Assignment (LOA-CFA) is a document that is used when establishing your cross connect to AWS at the colocation facility. For more information, see <a href="https://docs.aws.amazon.com/directconnect/latest/UserGuide/Colocation.html">Requesting Cross Connects at AWS Direct Connect Locations</a> in the <i>AWS Direct Connect User Guide</i>.</p>
    async fn describe_interconnect_loa(
        &self,
        input: DescribeInterconnectLoaRequest,
    ) -> Result<DescribeInterconnectLoaResponse, RusotoError<DescribeInterconnectLoaError>> {
        let mut request = SignedRequest::new("POST", "directconnect", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "OvertureService.DescribeInterconnectLoa");
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
                .deserialize::<DescribeInterconnectLoaResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(DescribeInterconnectLoaError::from_response(response))
        }
    }

    /// <p>Lists the interconnects owned by the AWS account or only the specified interconnect.</p>
    async fn describe_interconnects(
        &self,
        input: DescribeInterconnectsRequest,
    ) -> Result<Interconnects, RusotoError<DescribeInterconnectsError>> {
        let mut request = SignedRequest::new("POST", "directconnect", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "OvertureService.DescribeInterconnects");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response).deserialize::<Interconnects, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(DescribeInterconnectsError::from_response(response))
        }
    }

    /// <p>Describes all your link aggregation groups (LAG) or the specified LAG.</p>
    async fn describe_lags(
        &self,
        input: DescribeLagsRequest,
    ) -> Result<Lags, RusotoError<DescribeLagsError>> {
        let mut request = SignedRequest::new("POST", "directconnect", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "OvertureService.DescribeLags");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response).deserialize::<Lags, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(DescribeLagsError::from_response(response))
        }
    }

    /// <p>Gets the LOA-CFA for a connection, interconnect, or link aggregation group (LAG).</p> <p>The Letter of Authorization - Connecting Facility Assignment (LOA-CFA) is a document that is used when establishing your cross connect to AWS at the colocation facility. For more information, see <a href="https://docs.aws.amazon.com/directconnect/latest/UserGuide/Colocation.html">Requesting Cross Connects at AWS Direct Connect Locations</a> in the <i>AWS Direct Connect User Guide</i>.</p>
    async fn describe_loa(
        &self,
        input: DescribeLoaRequest,
    ) -> Result<Loa, RusotoError<DescribeLoaError>> {
        let mut request = SignedRequest::new("POST", "directconnect", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "OvertureService.DescribeLoa");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response).deserialize::<Loa, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(DescribeLoaError::from_response(response))
        }
    }

    /// <p>Lists the AWS Direct Connect locations in the current AWS Region. These are the locations that can be selected when calling <a>CreateConnection</a> or <a>CreateInterconnect</a>.</p>
    async fn describe_locations(&self) -> Result<Locations, RusotoError<DescribeLocationsError>> {
        let mut request = SignedRequest::new("POST", "directconnect", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "OvertureService.DescribeLocations");
        request.set_payload(Some(bytes::Bytes::from_static(b"{}")));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response).deserialize::<Locations, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(DescribeLocationsError::from_response(response))
        }
    }

    /// <p>Describes the tags associated with the specified AWS Direct Connect resources.</p>
    async fn describe_tags(
        &self,
        input: DescribeTagsRequest,
    ) -> Result<DescribeTagsResponse, RusotoError<DescribeTagsError>> {
        let mut request = SignedRequest::new("POST", "directconnect", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "OvertureService.DescribeTags");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response).deserialize::<DescribeTagsResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(DescribeTagsError::from_response(response))
        }
    }

    /// <p>Lists the virtual private gateways owned by the AWS account.</p> <p>You can create one or more AWS Direct Connect private virtual interfaces linked to a virtual private gateway.</p>
    async fn describe_virtual_gateways(
        &self,
    ) -> Result<VirtualGateways, RusotoError<DescribeVirtualGatewaysError>> {
        let mut request = SignedRequest::new("POST", "directconnect", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "OvertureService.DescribeVirtualGateways");
        request.set_payload(Some(bytes::Bytes::from_static(b"{}")));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response).deserialize::<VirtualGateways, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(DescribeVirtualGatewaysError::from_response(response))
        }
    }

    /// <p>Displays all virtual interfaces for an AWS account. Virtual interfaces deleted fewer than 15 minutes before you make the request are also returned. If you specify a connection ID, only the virtual interfaces associated with the connection are returned. If you specify a virtual interface ID, then only a single virtual interface is returned.</p> <p>A virtual interface (VLAN) transmits the traffic between the AWS Direct Connect location and the customer network.</p>
    async fn describe_virtual_interfaces(
        &self,
        input: DescribeVirtualInterfacesRequest,
    ) -> Result<VirtualInterfaces, RusotoError<DescribeVirtualInterfacesError>> {
        let mut request = SignedRequest::new("POST", "directconnect", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "OvertureService.DescribeVirtualInterfaces");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response).deserialize::<VirtualInterfaces, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(DescribeVirtualInterfacesError::from_response(response))
        }
    }

    /// <p>Disassociates a connection from a link aggregation group (LAG). The connection is interrupted and re-established as a standalone connection (the connection is not deleted; to delete the connection, use the <a>DeleteConnection</a> request). If the LAG has associated virtual interfaces or hosted connections, they remain associated with the LAG. A disassociated connection owned by an AWS Direct Connect Partner is automatically converted to an interconnect.</p> <p>If disassociating the connection would cause the LAG to fall below its setting for minimum number of operational connections, the request fails, except when it's the last member of the LAG. If all connections are disassociated, the LAG continues to exist as an empty LAG with no physical connections. </p>
    async fn disassociate_connection_from_lag(
        &self,
        input: DisassociateConnectionFromLagRequest,
    ) -> Result<Connection, RusotoError<DisassociateConnectionFromLagError>> {
        let mut request = SignedRequest::new("POST", "directconnect", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "OvertureService.DisassociateConnectionFromLag",
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
            proto::json::ResponsePayload::new(&response).deserialize::<Connection, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(DisassociateConnectionFromLagError::from_response(response))
        }
    }

    /// <p>Adds the specified tags to the specified AWS Direct Connect resource. Each resource can have a maximum of 50 tags.</p> <p>Each tag consists of a key and an optional value. If a tag with the same key is already associated with the resource, this action updates its value.</p>
    async fn tag_resource(
        &self,
        input: TagResourceRequest,
    ) -> Result<TagResourceResponse, RusotoError<TagResourceError>> {
        let mut request = SignedRequest::new("POST", "directconnect", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "OvertureService.TagResource");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response).deserialize::<TagResourceResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(TagResourceError::from_response(response))
        }
    }

    /// <p>Removes one or more tags from the specified AWS Direct Connect resource.</p>
    async fn untag_resource(
        &self,
        input: UntagResourceRequest,
    ) -> Result<UntagResourceResponse, RusotoError<UntagResourceError>> {
        let mut request = SignedRequest::new("POST", "directconnect", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "OvertureService.UntagResource");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response).deserialize::<UntagResourceResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(UntagResourceError::from_response(response))
        }
    }

    /// <p>Updates the specified attributes of the Direct Connect gateway association.</p> <p>Add or remove prefixes from the association.</p>
    async fn update_direct_connect_gateway_association(
        &self,
        input: UpdateDirectConnectGatewayAssociationRequest,
    ) -> Result<
        UpdateDirectConnectGatewayAssociationResult,
        RusotoError<UpdateDirectConnectGatewayAssociationError>,
    > {
        let mut request = SignedRequest::new("POST", "directconnect", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "OvertureService.UpdateDirectConnectGatewayAssociation",
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
                .deserialize::<UpdateDirectConnectGatewayAssociationResult, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(UpdateDirectConnectGatewayAssociationError::from_response(
                response,
            ))
        }
    }

    /// <p>Updates the attributes of the specified link aggregation group (LAG).</p> <p>You can update the following attributes:</p> <ul> <li> <p>The name of the LAG.</p> </li> <li> <p>The value for the minimum number of connections that must be operational for the LAG itself to be operational. </p> </li> </ul> <p>When you create a LAG, the default value for the minimum number of operational connections is zero (0). If you update this value and the number of operational connections falls below the specified value, the LAG automatically goes down to avoid over-utilization of the remaining connections. Adjust this value with care, as it could force the LAG down if it is set higher than the current number of operational connections.</p>
    async fn update_lag(
        &self,
        input: UpdateLagRequest,
    ) -> Result<Lag, RusotoError<UpdateLagError>> {
        let mut request = SignedRequest::new("POST", "directconnect", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "OvertureService.UpdateLag");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response).deserialize::<Lag, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(UpdateLagError::from_response(response))
        }
    }

    /// <p>Updates the specified attributes of the specified virtual private interface.</p> <p>Setting the MTU of a virtual interface to 9001 (jumbo frames) can cause an update to the underlying physical connection if it wasn't updated to support jumbo frames. Updating the connection disrupts network connectivity for all virtual interfaces associated with the connection for up to 30 seconds. To check whether your connection supports jumbo frames, call <a>DescribeConnections</a>. To check whether your virtual interface supports jumbo frames, call <a>DescribeVirtualInterfaces</a>.</p>
    async fn update_virtual_interface_attributes(
        &self,
        input: UpdateVirtualInterfaceAttributesRequest,
    ) -> Result<VirtualInterface, RusotoError<UpdateVirtualInterfaceAttributesError>> {
        let mut request = SignedRequest::new("POST", "directconnect", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "OvertureService.UpdateVirtualInterfaceAttributes",
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
            proto::json::ResponsePayload::new(&response).deserialize::<VirtualInterface, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(UpdateVirtualInterfaceAttributesError::from_response(
                response,
            ))
        }
    }
}
