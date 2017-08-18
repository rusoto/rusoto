
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

use std::fmt;
use std::error::Error;

use rusoto_core::region;
use rusoto_core::request::{DispatchSignedRequest, HttpDispatchError};
use rusoto_core::credential::{CredentialsError, ProvideAwsCredentials};

use serde_json;
use rusoto_core::signature::SignedRequest;
use serde_json::Value as SerdeJsonValue;
use serde_json::from_str;
#[doc="<p>Indicates the address family for the BGP peer.</p> <ul> <li> <p> <b>ipv4</b>: IPv4 address family</p> </li> <li> <p> <b>ipv6</b>: IPv6 address family</p> </li> </ul>"]
#[allow(non_camel_case_types)]
#[derive(Clone,Debug,Eq,PartialEq)]
pub enum AddressFamily {
    Ipv4,
    Ipv6,
}

impl Into<String> for AddressFamily {
    fn into(self) -> String {
        let s: &'static str = self.into();
        s.to_owned()
    }
}

impl Into<&'static str> for AddressFamily {
    fn into(self) -> &'static str {
        match self {
            AddressFamily::Ipv4 => "ipv4",
            AddressFamily::Ipv6 => "ipv6",
        }
    }
}

impl ::std::str::FromStr for AddressFamily {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "ipv4" => Ok(AddressFamily::Ipv4),
            "ipv6" => Ok(AddressFamily::Ipv6),
            _ => Err(()),
        }
    }
}

#[doc="<p>Container for the parameters to the AllocateConnectionOnInterconnect operation.</p>"]
#[derive(Default,Debug,Clone,Serialize)]
pub struct AllocateConnectionOnInterconnectRequest {
    #[doc="<p>Bandwidth of the connection.</p> <p>Example: \"<i>500Mbps</i>\"</p> <p>Default: None</p> <p>Values: 50Mbps, 100Mbps, 200Mbps, 300Mbps, 400Mbps, or 500Mbps</p>"]
    #[serde(rename="bandwidth")]
    pub bandwidth: String,
    #[doc="<p>Name of the provisioned connection.</p> <p>Example: \"<i>500M Connection to AWS</i>\"</p> <p>Default: None</p>"]
    #[serde(rename="connectionName")]
    pub connection_name: String,
    #[doc="<p>ID of the interconnect on which the connection will be provisioned.</p> <p>Example: dxcon-456abc78</p> <p>Default: None</p>"]
    #[serde(rename="interconnectId")]
    pub interconnect_id: String,
    #[doc="<p>Numeric account Id of the customer for whom the connection will be provisioned.</p> <p>Example: 123443215678</p> <p>Default: None</p>"]
    #[serde(rename="ownerAccount")]
    pub owner_account: String,
    #[doc="<p>The dedicated VLAN provisioned to the connection.</p> <p>Example: 101</p> <p>Default: None</p>"]
    #[serde(rename="vlan")]
    pub vlan: i64,
}

#[doc="<p>Container for the parameters to theHostedConnection operation.</p>"]
#[derive(Default,Debug,Clone,Serialize)]
pub struct AllocateHostedConnectionRequest {
    #[doc="<p>The bandwidth of the connection.</p> <p>Example: <code>500Mbps</code> </p> <p>Default: None</p> <p>Values: 50Mbps, 100Mbps, 200Mbps, 300Mbps, 400Mbps, or 500Mbps</p>"]
    #[serde(rename="bandwidth")]
    pub bandwidth: String,
    #[doc="<p>The ID of the interconnect or LAG on which the connection will be provisioned.</p> <p>Example: dxcon-456abc78 or dxlag-abc123</p> <p>Default: None</p>"]
    #[serde(rename="connectionId")]
    pub connection_id: String,
    #[doc="<p>The name of the provisioned connection.</p> <p>Example: \"<code>500M Connection to AWS</code>\"</p> <p>Default: None</p>"]
    #[serde(rename="connectionName")]
    pub connection_name: String,
    #[doc="<p>The numeric account ID of the customer for whom the connection will be provisioned.</p> <p>Example: 123443215678</p> <p>Default: None</p>"]
    #[serde(rename="ownerAccount")]
    pub owner_account: String,
    #[doc="<p>The dedicated VLAN provisioned to the hosted connection.</p> <p>Example: 101</p> <p>Default: None</p>"]
    #[serde(rename="vlan")]
    pub vlan: i64,
}

#[doc="<p>Container for the parameters to the AllocatePrivateVirtualInterface operation.</p>"]
#[derive(Default,Debug,Clone,Serialize)]
pub struct AllocatePrivateVirtualInterfaceRequest {
    #[doc="<p>The connection ID on which the private virtual interface is provisioned.</p> <p>Default: None</p>"]
    #[serde(rename="connectionId")]
    pub connection_id: String,
    #[doc="<p>Detailed information for the private virtual interface to be provisioned.</p> <p>Default: None</p>"]
    #[serde(rename="newPrivateVirtualInterfaceAllocation")]
    pub new_private_virtual_interface_allocation: NewPrivateVirtualInterfaceAllocation,
    #[doc="<p>The AWS account that will own the new private virtual interface.</p> <p>Default: None</p>"]
    #[serde(rename="ownerAccount")]
    pub owner_account: String,
}

#[doc="<p>Container for the parameters to the AllocatePublicVirtualInterface operation.</p>"]
#[derive(Default,Debug,Clone,Serialize)]
pub struct AllocatePublicVirtualInterfaceRequest {
    #[doc="<p>The connection ID on which the public virtual interface is provisioned.</p> <p>Default: None</p>"]
    #[serde(rename="connectionId")]
    pub connection_id: String,
    #[doc="<p>Detailed information for the public virtual interface to be provisioned.</p> <p>Default: None</p>"]
    #[serde(rename="newPublicVirtualInterfaceAllocation")]
    pub new_public_virtual_interface_allocation: NewPublicVirtualInterfaceAllocation,
    #[doc="<p>The AWS account that will own the new public virtual interface.</p> <p>Default: None</p>"]
    #[serde(rename="ownerAccount")]
    pub owner_account: String,
}

#[doc="<p>Container for the parameters to the AssociateConnectionWithLag operation.</p>"]
#[derive(Default,Debug,Clone,Serialize)]
pub struct AssociateConnectionWithLagRequest {
    #[doc="<p>The ID of the connection.</p> <p>Example: dxcon-abc123</p> <p>Default: None</p>"]
    #[serde(rename="connectionId")]
    pub connection_id: String,
    #[doc="<p>The ID of the LAG with which to associate the connection.</p> <p>Example: dxlag-abc123</p> <p>Default: None</p>"]
    #[serde(rename="lagId")]
    pub lag_id: String,
}

#[doc="<p>Container for the parameters to the AssociateHostedConnection operation.</p>"]
#[derive(Default,Debug,Clone,Serialize)]
pub struct AssociateHostedConnectionRequest {
    #[doc="<p>The ID of the hosted connection.</p> <p>Example: dxcon-abc123</p> <p>Default: None</p>"]
    #[serde(rename="connectionId")]
    pub connection_id: String,
    #[doc="<p>The ID of the interconnect or the LAG.</p> <p>Example: dxcon-abc123 or dxlag-abc123</p> <p>Default: None</p>"]
    #[serde(rename="parentConnectionId")]
    pub parent_connection_id: String,
}

#[doc="<p>Container for the parameters to the AssociateVirtualInterface operation.</p>"]
#[derive(Default,Debug,Clone,Serialize)]
pub struct AssociateVirtualInterfaceRequest {
    #[doc="<p>The ID of the LAG or connection with which to associate the virtual interface.</p> <p>Example: dxlag-abc123 or dxcon-abc123</p> <p>Default: None</p>"]
    #[serde(rename="connectionId")]
    pub connection_id: String,
    #[doc="<p>The ID of the virtual interface.</p> <p>Example: dxvif-123dfg56</p> <p>Default: None</p>"]
    #[serde(rename="virtualInterfaceId")]
    pub virtual_interface_id: String,
}

#[doc="<p>A structure containing information about a BGP peer.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct BGPPeer {
    #[serde(rename="addressFamily")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub address_family: Option<String>,
    #[serde(rename="amazonAddress")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub amazon_address: Option<String>,
    #[serde(rename="asn")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub asn: Option<i64>,
    #[serde(rename="authKey")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub auth_key: Option<String>,
    #[serde(rename="bgpPeerState")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub bgp_peer_state: Option<String>,
    #[serde(rename="bgpStatus")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub bgp_status: Option<String>,
    #[serde(rename="customerAddress")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub customer_address: Option<String>,
}

#[doc="<p>The state of the BGP peer.</p> <ul> <li> <p> <b>Verifying</b>: The BGP peering addresses or ASN require validation before the BGP peer can be created. This state only applies to BGP peers on a public virtual interface. </p> </li> <li> <p> <b>Pending</b>: The BGP peer has been created, and is in this state until it is ready to be established.</p> </li> <li> <p> <b>Available</b>: The BGP peer can be established.</p> </li> <li> <p> <b>Deleting</b>: The BGP peer is in the process of being deleted.</p> </li> <li> <p> <b>Deleted</b>: The BGP peer has been deleted and cannot be established.</p> </li> </ul>"]
#[allow(non_camel_case_types)]
#[derive(Clone,Debug,Eq,PartialEq)]
pub enum BGPPeerState {
    Available,
    Deleted,
    Deleting,
    Pending,
    Verifying,
}

impl Into<String> for BGPPeerState {
    fn into(self) -> String {
        let s: &'static str = self.into();
        s.to_owned()
    }
}

impl Into<&'static str> for BGPPeerState {
    fn into(self) -> &'static str {
        match self {
            BGPPeerState::Available => "available",
            BGPPeerState::Deleted => "deleted",
            BGPPeerState::Deleting => "deleting",
            BGPPeerState::Pending => "pending",
            BGPPeerState::Verifying => "verifying",
        }
    }
}

impl ::std::str::FromStr for BGPPeerState {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "available" => Ok(BGPPeerState::Available),
            "deleted" => Ok(BGPPeerState::Deleted),
            "deleting" => Ok(BGPPeerState::Deleting),
            "pending" => Ok(BGPPeerState::Pending),
            "verifying" => Ok(BGPPeerState::Verifying),
            _ => Err(()),
        }
    }
}

#[doc="<p>The Up/Down state of the BGP peer.</p> <ul> <li> <p> <b>Up</b>: The BGP peer is established.</p> </li> <li> <p> <b>Down</b>: The BGP peer is down.</p> </li> </ul>"]
#[allow(non_camel_case_types)]
#[derive(Clone,Debug,Eq,PartialEq)]
pub enum BGPStatus {
    Down,
    Up,
}

impl Into<String> for BGPStatus {
    fn into(self) -> String {
        let s: &'static str = self.into();
        s.to_owned()
    }
}

impl Into<&'static str> for BGPStatus {
    fn into(self) -> &'static str {
        match self {
            BGPStatus::Down => "down",
            BGPStatus::Up => "up",
        }
    }
}

impl ::std::str::FromStr for BGPStatus {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "down" => Ok(BGPStatus::Down),
            "up" => Ok(BGPStatus::Up),
            _ => Err(()),
        }
    }
}

#[doc="<p>Container for the parameters to the ConfirmConnection operation.</p>"]
#[derive(Default,Debug,Clone,Serialize)]
pub struct ConfirmConnectionRequest {
    #[serde(rename="connectionId")]
    pub connection_id: String,
}

#[doc="<p>The response received when ConfirmConnection is called.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct ConfirmConnectionResponse {
    #[serde(rename="connectionState")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub connection_state: Option<String>,
}

#[doc="<p>Container for the parameters to the ConfirmPrivateVirtualInterface operation.</p>"]
#[derive(Default,Debug,Clone,Serialize)]
pub struct ConfirmPrivateVirtualInterfaceRequest {
    #[doc="<p>ID of the virtual private gateway that will be attached to the virtual interface.</p> <p> A virtual private gateway can be managed via the Amazon Virtual Private Cloud (VPC) console or the <a href=\"http://docs.aws.amazon.com/AWSEC2/latest/APIReference/ApiReference-query-CreateVpnGateway.html\">EC2 CreateVpnGateway</a> action.</p> <p>Default: None</p>"]
    #[serde(rename="virtualGatewayId")]
    pub virtual_gateway_id: String,
    #[serde(rename="virtualInterfaceId")]
    pub virtual_interface_id: String,
}

#[doc="<p>The response received when ConfirmPrivateVirtualInterface is called.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct ConfirmPrivateVirtualInterfaceResponse {
    #[serde(rename="virtualInterfaceState")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub virtual_interface_state: Option<String>,
}

#[doc="<p>Container for the parameters to the ConfirmPublicVirtualInterface operation.</p>"]
#[derive(Default,Debug,Clone,Serialize)]
pub struct ConfirmPublicVirtualInterfaceRequest {
    #[serde(rename="virtualInterfaceId")]
    pub virtual_interface_id: String,
}

#[doc="<p>The response received when ConfirmPublicVirtualInterface is called.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct ConfirmPublicVirtualInterfaceResponse {
    #[serde(rename="virtualInterfaceState")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub virtual_interface_state: Option<String>,
}

#[doc="<p>A connection represents the physical network connection between the AWS Direct Connect location and the customer.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct Connection {
    #[doc="<p>The Direct Connection endpoint which the physical connection terminates on.</p>"]
    #[serde(rename="awsDevice")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub aws_device: Option<String>,
    #[doc="<p>Bandwidth of the connection.</p> <p>Example: 1Gbps (for regular connections), or 500Mbps (for hosted connections)</p> <p>Default: None</p>"]
    #[serde(rename="bandwidth")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub bandwidth: Option<String>,
    #[serde(rename="connectionId")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub connection_id: Option<String>,
    #[serde(rename="connectionName")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub connection_name: Option<String>,
    #[serde(rename="connectionState")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub connection_state: Option<String>,
    #[serde(rename="lagId")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub lag_id: Option<String>,
    #[doc="<p>The time of the most recent call to <a>DescribeLoa</a> for this connection.</p>"]
    #[serde(rename="loaIssueTime")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub loa_issue_time: Option<f64>,
    #[serde(rename="location")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub location: Option<String>,
    #[doc="<p>The AWS account that will own the new connection.</p>"]
    #[serde(rename="ownerAccount")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub owner_account: Option<String>,
    #[doc="<p>The name of the AWS Direct Connect service provider associated with the connection.</p>"]
    #[serde(rename="partnerName")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub partner_name: Option<String>,
    #[serde(rename="region")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub region: Option<String>,
    #[serde(rename="vlan")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub vlan: Option<i64>,
}

#[doc="<p>State of the connection.</p> <ul> <li> <p> <b>Ordering</b>: The initial state of a hosted connection provisioned on an interconnect. The connection stays in the ordering state until the owner of the hosted connection confirms or declines the connection order.</p> </li> <li> <p> <b>Requested</b>: The initial state of a standard connection. The connection stays in the requested state until the Letter of Authorization (LOA) is sent to the customer.</p> </li> <li> <p> <b>Pending</b>: The connection has been approved, and is being initialized.</p> </li> <li> <p> <b>Available</b>: The network link is up, and the connection is ready for use.</p> </li> <li> <p> <b>Down</b>: The network link is down.</p> </li> <li> <p> <b>Deleting</b>: The connection is in the process of being deleted.</p> </li> <li> <p> <b>Deleted</b>: The connection has been deleted.</p> </li> <li> <p> <b>Rejected</b>: A hosted connection in the 'Ordering' state will enter the 'Rejected' state if it is deleted by the end customer.</p> </li> </ul>"]
#[allow(non_camel_case_types)]
#[derive(Clone,Debug,Eq,PartialEq)]
pub enum ConnectionState {
    Available,
    Deleted,
    Deleting,
    Down,
    Ordering,
    Pending,
    Rejected,
    Requested,
}

impl Into<String> for ConnectionState {
    fn into(self) -> String {
        let s: &'static str = self.into();
        s.to_owned()
    }
}

impl Into<&'static str> for ConnectionState {
    fn into(self) -> &'static str {
        match self {
            ConnectionState::Available => "available",
            ConnectionState::Deleted => "deleted",
            ConnectionState::Deleting => "deleting",
            ConnectionState::Down => "down",
            ConnectionState::Ordering => "ordering",
            ConnectionState::Pending => "pending",
            ConnectionState::Rejected => "rejected",
            ConnectionState::Requested => "requested",
        }
    }
}

impl ::std::str::FromStr for ConnectionState {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "available" => Ok(ConnectionState::Available),
            "deleted" => Ok(ConnectionState::Deleted),
            "deleting" => Ok(ConnectionState::Deleting),
            "down" => Ok(ConnectionState::Down),
            "ordering" => Ok(ConnectionState::Ordering),
            "pending" => Ok(ConnectionState::Pending),
            "rejected" => Ok(ConnectionState::Rejected),
            "requested" => Ok(ConnectionState::Requested),
            _ => Err(()),
        }
    }
}

#[doc="<p>A structure containing a list of connections.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct Connections {
    #[doc="<p>A list of connections.</p>"]
    #[serde(rename="connections")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub connections: Option<Vec<Connection>>,
}

#[doc="<p>Container for the parameters to the CreateBGPPeer operation.</p>"]
#[derive(Default,Debug,Clone,Serialize)]
pub struct CreateBGPPeerRequest {
    #[doc="<p>Detailed information for the BGP peer to be created.</p> <p>Default: None</p>"]
    #[serde(rename="newBGPPeer")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub new_bgp_peer: Option<NewBGPPeer>,
    #[doc="<p>The ID of the virtual interface on which the BGP peer will be provisioned.</p> <p>Example: dxvif-456abc78</p> <p>Default: None</p>"]
    #[serde(rename="virtualInterfaceId")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub virtual_interface_id: Option<String>,
}

#[doc="<p>The response received when CreateBGPPeer is called.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct CreateBGPPeerResponse {
    #[serde(rename="virtualInterface")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub virtual_interface: Option<VirtualInterface>,
}

#[doc="<p>Container for the parameters to the CreateConnection operation.</p>"]
#[derive(Default,Debug,Clone,Serialize)]
pub struct CreateConnectionRequest {
    #[serde(rename="bandwidth")]
    pub bandwidth: String,
    #[serde(rename="connectionName")]
    pub connection_name: String,
    #[serde(rename="lagId")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub lag_id: Option<String>,
    #[serde(rename="location")]
    pub location: String,
}

#[doc="<p>Container for the parameters to the CreateInterconnect operation.</p>"]
#[derive(Default,Debug,Clone,Serialize)]
pub struct CreateInterconnectRequest {
    #[doc="<p>The port bandwidth</p> <p>Example: 1Gbps</p> <p>Default: None</p> <p>Available values: 1Gbps,10Gbps</p>"]
    #[serde(rename="bandwidth")]
    pub bandwidth: String,
    #[doc="<p>The name of the interconnect.</p> <p>Example: \"<i>1G Interconnect to AWS</i>\"</p> <p>Default: None</p>"]
    #[serde(rename="interconnectName")]
    pub interconnect_name: String,
    #[serde(rename="lagId")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub lag_id: Option<String>,
    #[doc="<p>Where the interconnect is located</p> <p>Example: EqSV5</p> <p>Default: None</p>"]
    #[serde(rename="location")]
    pub location: String,
}

#[doc="<p>Container for the parameters to the CreateLag operation.</p>"]
#[derive(Default,Debug,Clone,Serialize)]
pub struct CreateLagRequest {
    #[doc="<p>The ID of an existing connection to migrate to the LAG.</p> <p>Default: None</p>"]
    #[serde(rename="connectionId")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub connection_id: Option<String>,
    #[doc="<p>The bandwidth of the individual physical connections bundled by the LAG.</p> <p>Default: None</p> <p>Available values: 1Gbps, 10Gbps</p>"]
    #[serde(rename="connectionsBandwidth")]
    pub connections_bandwidth: String,
    #[doc="<p>The name of the LAG.</p> <p>Example: \"<code>3x10G LAG to AWS</code>\"</p> <p>Default: None</p>"]
    #[serde(rename="lagName")]
    pub lag_name: String,
    #[doc="<p>The AWS Direct Connect location in which the LAG should be allocated.</p> <p>Example: EqSV5</p> <p>Default: None</p>"]
    #[serde(rename="location")]
    pub location: String,
    #[doc="<p>The number of physical connections initially provisioned and bundled by the LAG.</p> <p>Default: None</p>"]
    #[serde(rename="numberOfConnections")]
    pub number_of_connections: i64,
}

#[doc="<p>Container for the parameters to the CreatePrivateVirtualInterface operation.</p>"]
#[derive(Default,Debug,Clone,Serialize)]
pub struct CreatePrivateVirtualInterfaceRequest {
    #[serde(rename="connectionId")]
    pub connection_id: String,
    #[doc="<p>Detailed information for the private virtual interface to be created.</p> <p>Default: None</p>"]
    #[serde(rename="newPrivateVirtualInterface")]
    pub new_private_virtual_interface: NewPrivateVirtualInterface,
}

#[doc="<p>Container for the parameters to the CreatePublicVirtualInterface operation.</p>"]
#[derive(Default,Debug,Clone,Serialize)]
pub struct CreatePublicVirtualInterfaceRequest {
    #[serde(rename="connectionId")]
    pub connection_id: String,
    #[doc="<p>Detailed information for the public virtual interface to be created.</p> <p>Default: None</p>"]
    #[serde(rename="newPublicVirtualInterface")]
    pub new_public_virtual_interface: NewPublicVirtualInterface,
}

#[doc="<p>Container for the parameters to the DeleteBGPPeer operation.</p>"]
#[derive(Default,Debug,Clone,Serialize)]
pub struct DeleteBGPPeerRequest {
    #[serde(rename="asn")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub asn: Option<i64>,
    #[serde(rename="customerAddress")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub customer_address: Option<String>,
    #[doc="<p>The ID of the virtual interface from which the BGP peer will be deleted.</p> <p>Example: dxvif-456abc78</p> <p>Default: None</p>"]
    #[serde(rename="virtualInterfaceId")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub virtual_interface_id: Option<String>,
}

#[doc="<p>The response received when DeleteBGPPeer is called.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct DeleteBGPPeerResponse {
    #[serde(rename="virtualInterface")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub virtual_interface: Option<VirtualInterface>,
}

#[doc="<p>Container for the parameters to the DeleteConnection operation.</p>"]
#[derive(Default,Debug,Clone,Serialize)]
pub struct DeleteConnectionRequest {
    #[serde(rename="connectionId")]
    pub connection_id: String,
}

#[doc="<p>Container for the parameters to the DeleteInterconnect operation.</p>"]
#[derive(Default,Debug,Clone,Serialize)]
pub struct DeleteInterconnectRequest {
    #[serde(rename="interconnectId")]
    pub interconnect_id: String,
}

#[doc="<p>The response received when DeleteInterconnect is called.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct DeleteInterconnectResponse {
    #[serde(rename="interconnectState")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub interconnect_state: Option<String>,
}

#[doc="<p>Container for the parameters to the DeleteLag operation.</p>"]
#[derive(Default,Debug,Clone,Serialize)]
pub struct DeleteLagRequest {
    #[doc="<p>The ID of the LAG to delete.</p> <p>Example: dxlag-abc123</p> <p>Default: None</p>"]
    #[serde(rename="lagId")]
    pub lag_id: String,
}

#[doc="<p>Container for the parameters to the DeleteVirtualInterface operation.</p>"]
#[derive(Default,Debug,Clone,Serialize)]
pub struct DeleteVirtualInterfaceRequest {
    #[serde(rename="virtualInterfaceId")]
    pub virtual_interface_id: String,
}

#[doc="<p>The response received when DeleteVirtualInterface is called.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct DeleteVirtualInterfaceResponse {
    #[serde(rename="virtualInterfaceState")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub virtual_interface_state: Option<String>,
}

#[doc="<p>Container for the parameters to the DescribeConnectionLoa operation.</p>"]
#[derive(Default,Debug,Clone,Serialize)]
pub struct DescribeConnectionLoaRequest {
    #[serde(rename="connectionId")]
    pub connection_id: String,
    #[serde(rename="loaContentType")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub loa_content_type: Option<String>,
    #[doc="<p>The name of the APN partner or service provider who establishes connectivity on your behalf. If you supply this parameter, the LOA-CFA lists the provider name alongside your company name as the requester of the cross connect.</p> <p>Default: None</p>"]
    #[serde(rename="providerName")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub provider_name: Option<String>,
}

#[doc="<p>The response received when DescribeConnectionLoa is called.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct DescribeConnectionLoaResponse {
    #[serde(rename="loa")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub loa: Option<Loa>,
}

#[doc="<p>Container for the parameters to the DescribeConnectionsOnInterconnect operation.</p>"]
#[derive(Default,Debug,Clone,Serialize)]
pub struct DescribeConnectionsOnInterconnectRequest {
    #[doc="<p>ID of the interconnect on which a list of connection is provisioned.</p> <p>Example: dxcon-abc123</p> <p>Default: None</p>"]
    #[serde(rename="interconnectId")]
    pub interconnect_id: String,
}

#[doc="<p>Container for the parameters to the DescribeConnections operation.</p>"]
#[derive(Default,Debug,Clone,Serialize)]
pub struct DescribeConnectionsRequest {
    #[serde(rename="connectionId")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub connection_id: Option<String>,
}

#[doc="<p>Container for the parameters to the DescribeHostedConnections operation.</p>"]
#[derive(Default,Debug,Clone,Serialize)]
pub struct DescribeHostedConnectionsRequest {
    #[doc="<p>The ID of the interconnect or LAG on which the hosted connections are provisioned.</p> <p>Example: dxcon-abc123 or dxlag-abc123</p> <p>Default: None</p>"]
    #[serde(rename="connectionId")]
    pub connection_id: String,
}

#[doc="<p>Container for the parameters to the DescribeInterconnectLoa operation.</p>"]
#[derive(Default,Debug,Clone,Serialize)]
pub struct DescribeInterconnectLoaRequest {
    #[serde(rename="interconnectId")]
    pub interconnect_id: String,
    #[serde(rename="loaContentType")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub loa_content_type: Option<String>,
    #[doc="<p>The name of the service provider who establishes connectivity on your behalf. If you supply this parameter, the LOA-CFA lists the provider name alongside your company name as the requester of the cross connect.</p> <p>Default: None</p>"]
    #[serde(rename="providerName")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub provider_name: Option<String>,
}

#[doc="<p>The response received when DescribeInterconnectLoa is called.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct DescribeInterconnectLoaResponse {
    #[serde(rename="loa")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub loa: Option<Loa>,
}

#[doc="<p>Container for the parameters to the DescribeInterconnects operation.</p>"]
#[derive(Default,Debug,Clone,Serialize)]
pub struct DescribeInterconnectsRequest {
    #[serde(rename="interconnectId")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub interconnect_id: Option<String>,
}

#[doc="<p>Container for the parameters to the DescribeLags operation.</p>"]
#[derive(Default,Debug,Clone,Serialize)]
pub struct DescribeLagsRequest {
    #[doc="<p>The ID of the LAG.</p> <p>Example: dxlag-abc123</p> <p>Default: None</p>"]
    #[serde(rename="lagId")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub lag_id: Option<String>,
}

#[doc="<p>Container for the parameters to the DescribeLoa operation.</p>"]
#[derive(Default,Debug,Clone,Serialize)]
pub struct DescribeLoaRequest {
    #[doc="<p>The ID of a connection, LAG, or interconnect for which to get the LOA-CFA information.</p> <p>Example: dxcon-abc123 or dxlag-abc123</p> <p>Default: None</p>"]
    #[serde(rename="connectionId")]
    pub connection_id: String,
    #[doc="<p>A standard media type indicating the content type of the LOA-CFA document. Currently, the only supported value is \"application/pdf\".</p> <p>Default: application/pdf</p>"]
    #[serde(rename="loaContentType")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub loa_content_type: Option<String>,
    #[doc="<p>The name of the service provider who establishes connectivity on your behalf. If you supply this parameter, the LOA-CFA lists the provider name alongside your company name as the requester of the cross connect.</p> <p>Default: None</p>"]
    #[serde(rename="providerName")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub provider_name: Option<String>,
}

#[doc="<p>Container for the parameters to the DescribeTags operation.</p>"]
#[derive(Default,Debug,Clone,Serialize)]
pub struct DescribeTagsRequest {
    #[doc="<p>The Amazon Resource Names (ARNs) of the Direct Connect resources.</p>"]
    #[serde(rename="resourceArns")]
    pub resource_arns: Vec<String>,
}

#[doc="<p>The response received when DescribeTags is called.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct DescribeTagsResponse {
    #[doc="<p>Information about the tags.</p>"]
    #[serde(rename="resourceTags")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub resource_tags: Option<Vec<ResourceTag>>,
}

#[doc="<p>Container for the parameters to the DescribeVirtualInterfaces operation.</p>"]
#[derive(Default,Debug,Clone,Serialize)]
pub struct DescribeVirtualInterfacesRequest {
    #[serde(rename="connectionId")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub connection_id: Option<String>,
    #[serde(rename="virtualInterfaceId")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub virtual_interface_id: Option<String>,
}

#[doc="<p>Container for the parameters to the DisassociateConnectionFromLag operation.</p>"]
#[derive(Default,Debug,Clone,Serialize)]
pub struct DisassociateConnectionFromLagRequest {
    #[doc="<p>The ID of the connection to disassociate from the LAG.</p> <p>Example: dxcon-abc123</p> <p>Default: None</p>"]
    #[serde(rename="connectionId")]
    pub connection_id: String,
    #[doc="<p>The ID of the LAG.</p> <p>Example: dxlag-abc123</p> <p>Default: None</p>"]
    #[serde(rename="lagId")]
    pub lag_id: String,
}

#[doc="<p>An interconnect is a connection that can host other connections.</p> <p>Like a standard AWS Direct Connect connection, an interconnect represents the physical connection between an AWS Direct Connect partner's network and a specific Direct Connect location. An AWS Direct Connect partner who owns an interconnect can provision hosted connections on the interconnect for their end customers, thereby providing the end customers with connectivity to AWS services.</p> <p>The resources of the interconnect, including bandwidth and VLAN numbers, are shared by all of the hosted connections on the interconnect, and the owner of the interconnect determines how these resources are assigned.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct Interconnect {
    #[doc="<p>The Direct Connection endpoint which the physical connection terminates on.</p>"]
    #[serde(rename="awsDevice")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub aws_device: Option<String>,
    #[serde(rename="bandwidth")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub bandwidth: Option<String>,
    #[serde(rename="interconnectId")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub interconnect_id: Option<String>,
    #[serde(rename="interconnectName")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub interconnect_name: Option<String>,
    #[serde(rename="interconnectState")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub interconnect_state: Option<String>,
    #[serde(rename="lagId")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub lag_id: Option<String>,
    #[doc="<p>The time of the most recent call to DescribeInterconnectLoa for this Interconnect.</p>"]
    #[serde(rename="loaIssueTime")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub loa_issue_time: Option<f64>,
    #[serde(rename="location")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub location: Option<String>,
    #[serde(rename="region")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub region: Option<String>,
}

#[doc="<p>State of the interconnect.</p> <ul> <li> <p> <b>Requested</b>: The initial state of an interconnect. The interconnect stays in the requested state until the Letter of Authorization (LOA) is sent to the customer.</p> </li> <li> <p> <b>Pending</b>: The interconnect has been approved, and is being initialized.</p> </li> <li> <p> <b>Available</b>: The network link is up, and the interconnect is ready for use.</p> </li> <li> <p> <b>Down</b>: The network link is down.</p> </li> <li> <p> <b>Deleting</b>: The interconnect is in the process of being deleted.</p> </li> <li> <p> <b>Deleted</b>: The interconnect has been deleted.</p> </li> </ul>"]
#[allow(non_camel_case_types)]
#[derive(Clone,Debug,Eq,PartialEq)]
pub enum InterconnectState {
    Available,
    Deleted,
    Deleting,
    Down,
    Pending,
    Requested,
}

impl Into<String> for InterconnectState {
    fn into(self) -> String {
        let s: &'static str = self.into();
        s.to_owned()
    }
}

impl Into<&'static str> for InterconnectState {
    fn into(self) -> &'static str {
        match self {
            InterconnectState::Available => "available",
            InterconnectState::Deleted => "deleted",
            InterconnectState::Deleting => "deleting",
            InterconnectState::Down => "down",
            InterconnectState::Pending => "pending",
            InterconnectState::Requested => "requested",
        }
    }
}

impl ::std::str::FromStr for InterconnectState {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "available" => Ok(InterconnectState::Available),
            "deleted" => Ok(InterconnectState::Deleted),
            "deleting" => Ok(InterconnectState::Deleting),
            "down" => Ok(InterconnectState::Down),
            "pending" => Ok(InterconnectState::Pending),
            "requested" => Ok(InterconnectState::Requested),
            _ => Err(()),
        }
    }
}

#[doc="<p>A structure containing a list of interconnects.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct Interconnects {
    #[doc="<p>A list of interconnects.</p>"]
    #[serde(rename="interconnects")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub interconnects: Option<Vec<Interconnect>>,
}

#[doc="<p>Describes a link aggregation group (LAG). A LAG is a connection that uses the Link Aggregation Control Protocol (LACP) to logically aggregate a bundle of physical connections. Like an interconnect, it can host other connections. All connections in a LAG must terminate on the same physical AWS Direct Connect endpoint, and must be the same bandwidth.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct Lag {
    #[doc="<p>Indicates whether the LAG can host other connections.</p> <note> <p>This is intended for use by AWS Direct Connect partners only.</p> </note>"]
    #[serde(rename="allowsHostedConnections")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub allows_hosted_connections: Option<bool>,
    #[doc="<p>The AWS Direct Connection endpoint that hosts the LAG.</p>"]
    #[serde(rename="awsDevice")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub aws_device: Option<String>,
    #[doc="<p>A list of connections bundled by this LAG.</p>"]
    #[serde(rename="connections")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub connections: Option<Vec<Connection>>,
    #[doc="<p>The individual bandwidth of the physical connections bundled by the LAG.</p> <p>Available values: 1Gbps, 10Gbps</p>"]
    #[serde(rename="connectionsBandwidth")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub connections_bandwidth: Option<String>,
    #[serde(rename="lagId")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub lag_id: Option<String>,
    #[doc="<p>The name of the LAG.</p>"]
    #[serde(rename="lagName")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub lag_name: Option<String>,
    #[serde(rename="lagState")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub lag_state: Option<String>,
    #[serde(rename="location")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub location: Option<String>,
    #[doc="<p>The minimum number of physical connections that must be operational for the LAG itself to be operational. If the number of operational connections drops below this setting, the LAG state changes to <code>down</code>. This value can help to ensure that a LAG is not overutilized if a significant number of its bundled connections go down.</p>"]
    #[serde(rename="minimumLinks")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub minimum_links: Option<i64>,
    #[doc="<p>The number of physical connections bundled by the LAG, up to a maximum of 10.</p>"]
    #[serde(rename="numberOfConnections")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub number_of_connections: Option<i64>,
    #[doc="<p>The owner of the LAG.</p>"]
    #[serde(rename="ownerAccount")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub owner_account: Option<String>,
    #[serde(rename="region")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub region: Option<String>,
}

#[doc="<p>The state of the LAG.</p> <ul> <li> <p> <b>Requested</b>: The initial state of a LAG. The LAG stays in the requested state until the Letter of Authorization (LOA) is available.</p> </li> <li> <p> <b>Pending</b>: The LAG has been approved, and is being initialized.</p> </li> <li> <p> <b>Available</b>: The network link is established, and the LAG is ready for use.</p> </li> <li> <p> <b>Down</b>: The network link is down.</p> </li> <li> <p> <b>Deleting</b>: The LAG is in the process of being deleted.</p> </li> <li> <p> <b>Deleted</b>: The LAG has been deleted.</p> </li> </ul>"]
#[allow(non_camel_case_types)]
#[derive(Clone,Debug,Eq,PartialEq)]
pub enum LagState {
    Available,
    Deleted,
    Deleting,
    Down,
    Pending,
    Requested,
}

impl Into<String> for LagState {
    fn into(self) -> String {
        let s: &'static str = self.into();
        s.to_owned()
    }
}

impl Into<&'static str> for LagState {
    fn into(self) -> &'static str {
        match self {
            LagState::Available => "available",
            LagState::Deleted => "deleted",
            LagState::Deleting => "deleting",
            LagState::Down => "down",
            LagState::Pending => "pending",
            LagState::Requested => "requested",
        }
    }
}

impl ::std::str::FromStr for LagState {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "available" => Ok(LagState::Available),
            "deleted" => Ok(LagState::Deleted),
            "deleting" => Ok(LagState::Deleting),
            "down" => Ok(LagState::Down),
            "pending" => Ok(LagState::Pending),
            "requested" => Ok(LagState::Requested),
            _ => Err(()),
        }
    }
}

#[doc="<p>A structure containing a list of LAGs.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct Lags {
    #[doc="<p>A list of LAGs.</p>"]
    #[serde(rename="lags")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub lags: Option<Vec<Lag>>,
}

#[doc="<p>A structure containing the Letter of Authorization - Connecting Facility Assignment (LOA-CFA) for a connection.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct Loa {
    #[serde(rename="loaContent")]
    #[serde(
                            deserialize_with="::rusoto_core::serialization::SerdeBlob::deserialize_blob",
                            serialize_with="::rusoto_core::serialization::SerdeBlob::serialize_blob",
                            default,
                        )]
    pub loa_content: Option<Vec<u8>>,
    #[serde(rename="loaContentType")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub loa_content_type: Option<String>,
}

#[doc="<p>A standard media type indicating the content type of the LOA-CFA document. Currently, the only supported value is \"application/pdf\".</p> <p>Default: application/pdf</p>"]
#[allow(non_camel_case_types)]
#[derive(Clone,Debug,Eq,PartialEq)]
pub enum LoaContentType {
    ApplicationPdf,
}

impl Into<String> for LoaContentType {
    fn into(self) -> String {
        let s: &'static str = self.into();
        s.to_owned()
    }
}

impl Into<&'static str> for LoaContentType {
    fn into(self) -> &'static str {
        match self {
            LoaContentType::ApplicationPdf => "application/pdf",
        }
    }
}

impl ::std::str::FromStr for LoaContentType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "application/pdf" => Ok(LoaContentType::ApplicationPdf),
            _ => Err(()),
        }
    }
}

#[doc="<p>An AWS Direct Connect location where connections and interconnects can be requested.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct Location {
    #[doc="<p>The code used to indicate the AWS Direct Connect location.</p>"]
    #[serde(rename="locationCode")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub location_code: Option<String>,
    #[doc="<p>The name of the AWS Direct Connect location. The name includes the colocation partner name and the physical site of the lit building.</p>"]
    #[serde(rename="locationName")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub location_name: Option<String>,
}

#[doc="<p>A location is a network facility where AWS Direct Connect routers are available to be connected. Generally, these are colocation hubs where many network providers have equipment, and where cross connects can be delivered. Locations include a name and facility code, and must be provided when creating a connection.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct Locations {
    #[doc="<p>A list of colocation hubs where network providers have equipment. Most regions have multiple locations available.</p>"]
    #[serde(rename="locations")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub locations: Option<Vec<Location>>,
}

#[doc="<p>A structure containing information about a new BGP peer.</p>"]
#[derive(Default,Debug,Clone,Serialize)]
pub struct NewBGPPeer {
    #[serde(rename="addressFamily")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub address_family: Option<String>,
    #[serde(rename="amazonAddress")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub amazon_address: Option<String>,
    #[serde(rename="asn")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub asn: Option<i64>,
    #[serde(rename="authKey")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub auth_key: Option<String>,
    #[serde(rename="customerAddress")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub customer_address: Option<String>,
}

#[doc="<p>A structure containing information about a new private virtual interface.</p>"]
#[derive(Default,Debug,Clone,Serialize)]
pub struct NewPrivateVirtualInterface {
    #[serde(rename="addressFamily")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub address_family: Option<String>,
    #[serde(rename="amazonAddress")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub amazon_address: Option<String>,
    #[serde(rename="asn")]
    pub asn: i64,
    #[serde(rename="authKey")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub auth_key: Option<String>,
    #[serde(rename="customerAddress")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub customer_address: Option<String>,
    #[serde(rename="virtualGatewayId")]
    pub virtual_gateway_id: String,
    #[serde(rename="virtualInterfaceName")]
    pub virtual_interface_name: String,
    #[serde(rename="vlan")]
    pub vlan: i64,
}

#[doc="<p>A structure containing information about a private virtual interface that will be provisioned on a connection.</p>"]
#[derive(Default,Debug,Clone,Serialize)]
pub struct NewPrivateVirtualInterfaceAllocation {
    #[serde(rename="addressFamily")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub address_family: Option<String>,
    #[serde(rename="amazonAddress")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub amazon_address: Option<String>,
    #[serde(rename="asn")]
    pub asn: i64,
    #[serde(rename="authKey")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub auth_key: Option<String>,
    #[serde(rename="customerAddress")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub customer_address: Option<String>,
    #[serde(rename="virtualInterfaceName")]
    pub virtual_interface_name: String,
    #[serde(rename="vlan")]
    pub vlan: i64,
}

#[doc="<p>A structure containing information about a new public virtual interface.</p>"]
#[derive(Default,Debug,Clone,Serialize)]
pub struct NewPublicVirtualInterface {
    #[serde(rename="addressFamily")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub address_family: Option<String>,
    #[serde(rename="amazonAddress")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub amazon_address: Option<String>,
    #[serde(rename="asn")]
    pub asn: i64,
    #[serde(rename="authKey")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub auth_key: Option<String>,
    #[serde(rename="customerAddress")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub customer_address: Option<String>,
    #[serde(rename="routeFilterPrefixes")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub route_filter_prefixes: Option<Vec<RouteFilterPrefix>>,
    #[serde(rename="virtualInterfaceName")]
    pub virtual_interface_name: String,
    #[serde(rename="vlan")]
    pub vlan: i64,
}

#[doc="<p>A structure containing information about a public virtual interface that will be provisioned on a connection.</p>"]
#[derive(Default,Debug,Clone,Serialize)]
pub struct NewPublicVirtualInterfaceAllocation {
    #[serde(rename="addressFamily")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub address_family: Option<String>,
    #[serde(rename="amazonAddress")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub amazon_address: Option<String>,
    #[serde(rename="asn")]
    pub asn: i64,
    #[serde(rename="authKey")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub auth_key: Option<String>,
    #[serde(rename="customerAddress")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub customer_address: Option<String>,
    #[serde(rename="routeFilterPrefixes")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub route_filter_prefixes: Option<Vec<RouteFilterPrefix>>,
    #[serde(rename="virtualInterfaceName")]
    pub virtual_interface_name: String,
    #[serde(rename="vlan")]
    pub vlan: i64,
}

#[doc="<p>The tags associated with a Direct Connect resource.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct ResourceTag {
    #[doc="<p>The Amazon Resource Name (ARN) of the Direct Connect resource.</p>"]
    #[serde(rename="resourceArn")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub resource_arn: Option<String>,
    #[doc="<p>The tags.</p>"]
    #[serde(rename="tags")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

#[doc="<p>A route filter prefix that the customer can advertise through Border Gateway Protocol (BGP) over a public virtual interface.</p>"]
#[derive(Default,Debug,Clone,Serialize,Deserialize)]
pub struct RouteFilterPrefix {
    #[doc="<p>CIDR notation for the advertised route. Multiple routes are separated by commas.</p> <p>IPv6 CIDRs must be at least a /64 or shorter</p> <p>Example: 10.10.10.0/24,10.10.11.0/24,2001:db8::/64</p>"]
    #[serde(rename="cidr")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub cidr: Option<String>,
}

#[doc="<p>Information about a tag.</p>"]
#[derive(Default,Debug,Clone,Serialize,Deserialize)]
pub struct Tag {
    #[doc="<p>The key of the tag.</p>"]
    #[serde(rename="key")]
    pub key: String,
    #[doc="<p>The value of the tag.</p>"]
    #[serde(rename="value")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub value: Option<String>,
}

#[doc="<p>Container for the parameters to the TagResource operation.</p>"]
#[derive(Default,Debug,Clone,Serialize)]
pub struct TagResourceRequest {
    #[doc="<p>The Amazon Resource Name (ARN) of the Direct Connect resource.</p> <p>Example: arn:aws:directconnect:us-east-1:123456789012:dxcon/dxcon-fg5678gh</p>"]
    #[serde(rename="resourceArn")]
    pub resource_arn: String,
    #[doc="<p>The list of tags to add.</p>"]
    #[serde(rename="tags")]
    pub tags: Vec<Tag>,
}

#[doc="<p>The response received when TagResource is called.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct TagResourceResponse;

#[doc="<p>Container for the parameters to the UntagResource operation.</p>"]
#[derive(Default,Debug,Clone,Serialize)]
pub struct UntagResourceRequest {
    #[doc="<p>The Amazon Resource Name (ARN) of the Direct Connect resource.</p>"]
    #[serde(rename="resourceArn")]
    pub resource_arn: String,
    #[doc="<p>The list of tag keys to remove.</p>"]
    #[serde(rename="tagKeys")]
    pub tag_keys: Vec<String>,
}

#[doc="<p>The response received when UntagResource is called.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct UntagResourceResponse;

#[doc="<p>Container for the parameters to the UpdateLag operation.</p>"]
#[derive(Default,Debug,Clone,Serialize)]
pub struct UpdateLagRequest {
    #[doc="<p>The ID of the LAG to update.</p> <p>Example: dxlag-abc123</p> <p>Default: None</p>"]
    #[serde(rename="lagId")]
    pub lag_id: String,
    #[doc="<p>The name for the LAG.</p> <p>Example: \"<code>3x10G LAG to AWS</code>\"</p> <p>Default: None</p>"]
    #[serde(rename="lagName")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub lag_name: Option<String>,
    #[doc="<p>The minimum number of physical connections that must be operational for the LAG itself to be operational.</p> <p>Default: None</p>"]
    #[serde(rename="minimumLinks")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub minimum_links: Option<i64>,
}

#[doc="<p>You can create one or more AWS Direct Connect private virtual interfaces linking to your virtual private gateway.</p> <p>Virtual private gateways can be managed using the Amazon Virtual Private Cloud (Amazon VPC) console or the <a href=\"http://docs.aws.amazon.com/AWSEC2/latest/APIReference/ApiReference-query-CreateVpnGateway.html\">Amazon EC2 CreateVpnGateway action</a>.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct VirtualGateway {
    #[serde(rename="virtualGatewayId")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub virtual_gateway_id: Option<String>,
    #[serde(rename="virtualGatewayState")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub virtual_gateway_state: Option<String>,
}

#[doc="<p>A structure containing a list of virtual private gateways.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct VirtualGateways {
    #[doc="<p>A list of virtual private gateways.</p>"]
    #[serde(rename="virtualGateways")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub virtual_gateways: Option<Vec<VirtualGateway>>,
}

#[doc="<p>A virtual interface (VLAN) transmits the traffic between the AWS Direct Connect location and the customer.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct VirtualInterface {
    #[serde(rename="addressFamily")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub address_family: Option<String>,
    #[serde(rename="amazonAddress")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub amazon_address: Option<String>,
    #[serde(rename="asn")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub asn: Option<i64>,
    #[serde(rename="authKey")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub auth_key: Option<String>,
    #[serde(rename="bgpPeers")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub bgp_peers: Option<Vec<BGPPeer>>,
    #[serde(rename="connectionId")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub connection_id: Option<String>,
    #[serde(rename="customerAddress")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub customer_address: Option<String>,
    #[doc="<p>Information for generating the customer router configuration.</p>"]
    #[serde(rename="customerRouterConfig")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub customer_router_config: Option<String>,
    #[serde(rename="location")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub location: Option<String>,
    #[doc="<p>The AWS account that will own the new virtual interface.</p>"]
    #[serde(rename="ownerAccount")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub owner_account: Option<String>,
    #[serde(rename="routeFilterPrefixes")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub route_filter_prefixes: Option<Vec<RouteFilterPrefix>>,
    #[serde(rename="virtualGatewayId")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub virtual_gateway_id: Option<String>,
    #[serde(rename="virtualInterfaceId")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub virtual_interface_id: Option<String>,
    #[serde(rename="virtualInterfaceName")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub virtual_interface_name: Option<String>,
    #[serde(rename="virtualInterfaceState")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub virtual_interface_state: Option<String>,
    #[serde(rename="virtualInterfaceType")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub virtual_interface_type: Option<String>,
    #[serde(rename="vlan")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub vlan: Option<i64>,
}

#[doc="<p>State of the virtual interface.</p> <ul> <li> <p> <b>Confirming</b>: The creation of the virtual interface is pending confirmation from the virtual interface owner. If the owner of the virtual interface is different from the owner of the connection on which it is provisioned, then the virtual interface will remain in this state until it is confirmed by the virtual interface owner.</p> </li> <li> <p> <b>Verifying</b>: This state only applies to public virtual interfaces. Each public virtual interface needs validation before the virtual interface can be created.</p> </li> <li> <p> <b>Pending</b>: A virtual interface is in this state from the time that it is created until the virtual interface is ready to forward traffic.</p> </li> <li> <p> <b>Available</b>: A virtual interface that is able to forward traffic.</p> </li> <li> <p> <b>Down</b>: A virtual interface that is BGP down.</p> </li> <li> <p> <b>Deleting</b>: A virtual interface is in this state immediately after calling <a>DeleteVirtualInterface</a> until it can no longer forward traffic.</p> </li> <li> <p> <b>Deleted</b>: A virtual interface that cannot forward traffic.</p> </li> <li> <p> <b>Rejected</b>: The virtual interface owner has declined creation of the virtual interface. If a virtual interface in the 'Confirming' state is deleted by the virtual interface owner, the virtual interface will enter the 'Rejected' state.</p> </li> </ul>"]
#[allow(non_camel_case_types)]
#[derive(Clone,Debug,Eq,PartialEq)]
pub enum VirtualInterfaceState {
    Available,
    Confirming,
    Deleted,
    Deleting,
    Down,
    Pending,
    Rejected,
    Verifying,
}

impl Into<String> for VirtualInterfaceState {
    fn into(self) -> String {
        let s: &'static str = self.into();
        s.to_owned()
    }
}

impl Into<&'static str> for VirtualInterfaceState {
    fn into(self) -> &'static str {
        match self {
            VirtualInterfaceState::Available => "available",
            VirtualInterfaceState::Confirming => "confirming",
            VirtualInterfaceState::Deleted => "deleted",
            VirtualInterfaceState::Deleting => "deleting",
            VirtualInterfaceState::Down => "down",
            VirtualInterfaceState::Pending => "pending",
            VirtualInterfaceState::Rejected => "rejected",
            VirtualInterfaceState::Verifying => "verifying",
        }
    }
}

impl ::std::str::FromStr for VirtualInterfaceState {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "available" => Ok(VirtualInterfaceState::Available),
            "confirming" => Ok(VirtualInterfaceState::Confirming),
            "deleted" => Ok(VirtualInterfaceState::Deleted),
            "deleting" => Ok(VirtualInterfaceState::Deleting),
            "down" => Ok(VirtualInterfaceState::Down),
            "pending" => Ok(VirtualInterfaceState::Pending),
            "rejected" => Ok(VirtualInterfaceState::Rejected),
            "verifying" => Ok(VirtualInterfaceState::Verifying),
            _ => Err(()),
        }
    }
}

#[doc="<p>A structure containing a list of virtual interfaces.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct VirtualInterfaces {
    #[doc="<p>A list of virtual interfaces.</p>"]
    #[serde(rename="virtualInterfaces")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub virtual_interfaces: Option<Vec<VirtualInterface>>,
}

/// Errors returned by AllocateConnectionOnInterconnect
#[derive(Debug, PartialEq)]
pub enum AllocateConnectionOnInterconnectError {
    ///<p>The API was called with invalid parameters. The error message will contain additional details about the cause.</p>
    DirectConnectClient(String),
    ///<p>A server-side error occurred during the API call. The error message will contain additional details about the cause.</p>
    DirectConnectServer(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl AllocateConnectionOnInterconnectError {
    pub fn from_body(body: &str) -> AllocateConnectionOnInterconnectError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "DirectConnectClientException" => AllocateConnectionOnInterconnectError::DirectConnectClient(String::from(error_message)),
                    "DirectConnectServerException" => AllocateConnectionOnInterconnectError::DirectConnectServer(String::from(error_message)),
                    "ValidationException" => {
                        AllocateConnectionOnInterconnectError::Validation(error_message.to_string())
                    }
                    _ => AllocateConnectionOnInterconnectError::Unknown(String::from(body)),
                }
            }
            Err(_) => AllocateConnectionOnInterconnectError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for AllocateConnectionOnInterconnectError {
    fn from(err: serde_json::error::Error) -> AllocateConnectionOnInterconnectError {
        AllocateConnectionOnInterconnectError::Unknown(err.description().to_string())
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
            AllocateConnectionOnInterconnectError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by AllocateHostedConnection
#[derive(Debug, PartialEq)]
pub enum AllocateHostedConnectionError {
    ///<p>The API was called with invalid parameters. The error message will contain additional details about the cause.</p>
    DirectConnectClient(String),
    ///<p>A server-side error occurred during the API call. The error message will contain additional details about the cause.</p>
    DirectConnectServer(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl AllocateHostedConnectionError {
    pub fn from_body(body: &str) -> AllocateHostedConnectionError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "DirectConnectClientException" => AllocateHostedConnectionError::DirectConnectClient(String::from(error_message)),
                    "DirectConnectServerException" => AllocateHostedConnectionError::DirectConnectServer(String::from(error_message)),
                    "ValidationException" => {
                        AllocateHostedConnectionError::Validation(error_message.to_string())
                    }
                    _ => AllocateHostedConnectionError::Unknown(String::from(body)),
                }
            }
            Err(_) => AllocateHostedConnectionError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for AllocateHostedConnectionError {
    fn from(err: serde_json::error::Error) -> AllocateHostedConnectionError {
        AllocateHostedConnectionError::Unknown(err.description().to_string())
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
            AllocateHostedConnectionError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by AllocatePrivateVirtualInterface
#[derive(Debug, PartialEq)]
pub enum AllocatePrivateVirtualInterfaceError {
    ///<p>The API was called with invalid parameters. The error message will contain additional details about the cause.</p>
    DirectConnectClient(String),
    ///<p>A server-side error occurred during the API call. The error message will contain additional details about the cause.</p>
    DirectConnectServer(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl AllocatePrivateVirtualInterfaceError {
    pub fn from_body(body: &str) -> AllocatePrivateVirtualInterfaceError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "DirectConnectClientException" => AllocatePrivateVirtualInterfaceError::DirectConnectClient(String::from(error_message)),
                    "DirectConnectServerException" => AllocatePrivateVirtualInterfaceError::DirectConnectServer(String::from(error_message)),
                    "ValidationException" => {
                        AllocatePrivateVirtualInterfaceError::Validation(error_message.to_string())
                    }
                    _ => AllocatePrivateVirtualInterfaceError::Unknown(String::from(body)),
                }
            }
            Err(_) => AllocatePrivateVirtualInterfaceError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for AllocatePrivateVirtualInterfaceError {
    fn from(err: serde_json::error::Error) -> AllocatePrivateVirtualInterfaceError {
        AllocatePrivateVirtualInterfaceError::Unknown(err.description().to_string())
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
            AllocatePrivateVirtualInterfaceError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by AllocatePublicVirtualInterface
#[derive(Debug, PartialEq)]
pub enum AllocatePublicVirtualInterfaceError {
    ///<p>The API was called with invalid parameters. The error message will contain additional details about the cause.</p>
    DirectConnectClient(String),
    ///<p>A server-side error occurred during the API call. The error message will contain additional details about the cause.</p>
    DirectConnectServer(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl AllocatePublicVirtualInterfaceError {
    pub fn from_body(body: &str) -> AllocatePublicVirtualInterfaceError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "DirectConnectClientException" => AllocatePublicVirtualInterfaceError::DirectConnectClient(String::from(error_message)),
                    "DirectConnectServerException" => AllocatePublicVirtualInterfaceError::DirectConnectServer(String::from(error_message)),
                    "ValidationException" => {
                        AllocatePublicVirtualInterfaceError::Validation(error_message.to_string())
                    }
                    _ => AllocatePublicVirtualInterfaceError::Unknown(String::from(body)),
                }
            }
            Err(_) => AllocatePublicVirtualInterfaceError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for AllocatePublicVirtualInterfaceError {
    fn from(err: serde_json::error::Error) -> AllocatePublicVirtualInterfaceError {
        AllocatePublicVirtualInterfaceError::Unknown(err.description().to_string())
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
            AllocatePublicVirtualInterfaceError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by AssociateConnectionWithLag
#[derive(Debug, PartialEq)]
pub enum AssociateConnectionWithLagError {
    ///<p>The API was called with invalid parameters. The error message will contain additional details about the cause.</p>
    DirectConnectClient(String),
    ///<p>A server-side error occurred during the API call. The error message will contain additional details about the cause.</p>
    DirectConnectServer(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl AssociateConnectionWithLagError {
    pub fn from_body(body: &str) -> AssociateConnectionWithLagError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "DirectConnectClientException" => AssociateConnectionWithLagError::DirectConnectClient(String::from(error_message)),
                    "DirectConnectServerException" => AssociateConnectionWithLagError::DirectConnectServer(String::from(error_message)),
                    "ValidationException" => {
                        AssociateConnectionWithLagError::Validation(error_message.to_string())
                    }
                    _ => AssociateConnectionWithLagError::Unknown(String::from(body)),
                }
            }
            Err(_) => AssociateConnectionWithLagError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for AssociateConnectionWithLagError {
    fn from(err: serde_json::error::Error) -> AssociateConnectionWithLagError {
        AssociateConnectionWithLagError::Unknown(err.description().to_string())
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
            AssociateConnectionWithLagError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by AssociateHostedConnection
#[derive(Debug, PartialEq)]
pub enum AssociateHostedConnectionError {
    ///<p>The API was called with invalid parameters. The error message will contain additional details about the cause.</p>
    DirectConnectClient(String),
    ///<p>A server-side error occurred during the API call. The error message will contain additional details about the cause.</p>
    DirectConnectServer(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl AssociateHostedConnectionError {
    pub fn from_body(body: &str) -> AssociateHostedConnectionError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "DirectConnectClientException" => AssociateHostedConnectionError::DirectConnectClient(String::from(error_message)),
                    "DirectConnectServerException" => AssociateHostedConnectionError::DirectConnectServer(String::from(error_message)),
                    "ValidationException" => {
                        AssociateHostedConnectionError::Validation(error_message.to_string())
                    }
                    _ => AssociateHostedConnectionError::Unknown(String::from(body)),
                }
            }
            Err(_) => AssociateHostedConnectionError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for AssociateHostedConnectionError {
    fn from(err: serde_json::error::Error) -> AssociateHostedConnectionError {
        AssociateHostedConnectionError::Unknown(err.description().to_string())
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
            AssociateHostedConnectionError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by AssociateVirtualInterface
#[derive(Debug, PartialEq)]
pub enum AssociateVirtualInterfaceError {
    ///<p>The API was called with invalid parameters. The error message will contain additional details about the cause.</p>
    DirectConnectClient(String),
    ///<p>A server-side error occurred during the API call. The error message will contain additional details about the cause.</p>
    DirectConnectServer(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl AssociateVirtualInterfaceError {
    pub fn from_body(body: &str) -> AssociateVirtualInterfaceError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "DirectConnectClientException" => AssociateVirtualInterfaceError::DirectConnectClient(String::from(error_message)),
                    "DirectConnectServerException" => AssociateVirtualInterfaceError::DirectConnectServer(String::from(error_message)),
                    "ValidationException" => {
                        AssociateVirtualInterfaceError::Validation(error_message.to_string())
                    }
                    _ => AssociateVirtualInterfaceError::Unknown(String::from(body)),
                }
            }
            Err(_) => AssociateVirtualInterfaceError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for AssociateVirtualInterfaceError {
    fn from(err: serde_json::error::Error) -> AssociateVirtualInterfaceError {
        AssociateVirtualInterfaceError::Unknown(err.description().to_string())
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
            AssociateVirtualInterfaceError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by ConfirmConnection
#[derive(Debug, PartialEq)]
pub enum ConfirmConnectionError {
    ///<p>The API was called with invalid parameters. The error message will contain additional details about the cause.</p>
    DirectConnectClient(String),
    ///<p>A server-side error occurred during the API call. The error message will contain additional details about the cause.</p>
    DirectConnectServer(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl ConfirmConnectionError {
    pub fn from_body(body: &str) -> ConfirmConnectionError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "DirectConnectClientException" => {
                        ConfirmConnectionError::DirectConnectClient(String::from(error_message))
                    }
                    "DirectConnectServerException" => {
                        ConfirmConnectionError::DirectConnectServer(String::from(error_message))
                    }
                    "ValidationException" => {
                        ConfirmConnectionError::Validation(error_message.to_string())
                    }
                    _ => ConfirmConnectionError::Unknown(String::from(body)),
                }
            }
            Err(_) => ConfirmConnectionError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for ConfirmConnectionError {
    fn from(err: serde_json::error::Error) -> ConfirmConnectionError {
        ConfirmConnectionError::Unknown(err.description().to_string())
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
            ConfirmConnectionError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by ConfirmPrivateVirtualInterface
#[derive(Debug, PartialEq)]
pub enum ConfirmPrivateVirtualInterfaceError {
    ///<p>The API was called with invalid parameters. The error message will contain additional details about the cause.</p>
    DirectConnectClient(String),
    ///<p>A server-side error occurred during the API call. The error message will contain additional details about the cause.</p>
    DirectConnectServer(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl ConfirmPrivateVirtualInterfaceError {
    pub fn from_body(body: &str) -> ConfirmPrivateVirtualInterfaceError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "DirectConnectClientException" => ConfirmPrivateVirtualInterfaceError::DirectConnectClient(String::from(error_message)),
                    "DirectConnectServerException" => ConfirmPrivateVirtualInterfaceError::DirectConnectServer(String::from(error_message)),
                    "ValidationException" => {
                        ConfirmPrivateVirtualInterfaceError::Validation(error_message.to_string())
                    }
                    _ => ConfirmPrivateVirtualInterfaceError::Unknown(String::from(body)),
                }
            }
            Err(_) => ConfirmPrivateVirtualInterfaceError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for ConfirmPrivateVirtualInterfaceError {
    fn from(err: serde_json::error::Error) -> ConfirmPrivateVirtualInterfaceError {
        ConfirmPrivateVirtualInterfaceError::Unknown(err.description().to_string())
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
            ConfirmPrivateVirtualInterfaceError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by ConfirmPublicVirtualInterface
#[derive(Debug, PartialEq)]
pub enum ConfirmPublicVirtualInterfaceError {
    ///<p>The API was called with invalid parameters. The error message will contain additional details about the cause.</p>
    DirectConnectClient(String),
    ///<p>A server-side error occurred during the API call. The error message will contain additional details about the cause.</p>
    DirectConnectServer(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl ConfirmPublicVirtualInterfaceError {
    pub fn from_body(body: &str) -> ConfirmPublicVirtualInterfaceError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "DirectConnectClientException" => ConfirmPublicVirtualInterfaceError::DirectConnectClient(String::from(error_message)),
                    "DirectConnectServerException" => ConfirmPublicVirtualInterfaceError::DirectConnectServer(String::from(error_message)),
                    "ValidationException" => {
                        ConfirmPublicVirtualInterfaceError::Validation(error_message.to_string())
                    }
                    _ => ConfirmPublicVirtualInterfaceError::Unknown(String::from(body)),
                }
            }
            Err(_) => ConfirmPublicVirtualInterfaceError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for ConfirmPublicVirtualInterfaceError {
    fn from(err: serde_json::error::Error) -> ConfirmPublicVirtualInterfaceError {
        ConfirmPublicVirtualInterfaceError::Unknown(err.description().to_string())
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
            ConfirmPublicVirtualInterfaceError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by CreateBGPPeer
#[derive(Debug, PartialEq)]
pub enum CreateBGPPeerError {
    ///<p>The API was called with invalid parameters. The error message will contain additional details about the cause.</p>
    DirectConnectClient(String),
    ///<p>A server-side error occurred during the API call. The error message will contain additional details about the cause.</p>
    DirectConnectServer(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl CreateBGPPeerError {
    pub fn from_body(body: &str) -> CreateBGPPeerError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "DirectConnectClientException" => {
                        CreateBGPPeerError::DirectConnectClient(String::from(error_message))
                    }
                    "DirectConnectServerException" => {
                        CreateBGPPeerError::DirectConnectServer(String::from(error_message))
                    }
                    "ValidationException" => {
                        CreateBGPPeerError::Validation(error_message.to_string())
                    }
                    _ => CreateBGPPeerError::Unknown(String::from(body)),
                }
            }
            Err(_) => CreateBGPPeerError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for CreateBGPPeerError {
    fn from(err: serde_json::error::Error) -> CreateBGPPeerError {
        CreateBGPPeerError::Unknown(err.description().to_string())
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
            CreateBGPPeerError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by CreateConnection
#[derive(Debug, PartialEq)]
pub enum CreateConnectionError {
    ///<p>The API was called with invalid parameters. The error message will contain additional details about the cause.</p>
    DirectConnectClient(String),
    ///<p>A server-side error occurred during the API call. The error message will contain additional details about the cause.</p>
    DirectConnectServer(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl CreateConnectionError {
    pub fn from_body(body: &str) -> CreateConnectionError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "DirectConnectClientException" => {
                        CreateConnectionError::DirectConnectClient(String::from(error_message))
                    }
                    "DirectConnectServerException" => {
                        CreateConnectionError::DirectConnectServer(String::from(error_message))
                    }
                    "ValidationException" => {
                        CreateConnectionError::Validation(error_message.to_string())
                    }
                    _ => CreateConnectionError::Unknown(String::from(body)),
                }
            }
            Err(_) => CreateConnectionError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for CreateConnectionError {
    fn from(err: serde_json::error::Error) -> CreateConnectionError {
        CreateConnectionError::Unknown(err.description().to_string())
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
            CreateConnectionError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by CreateInterconnect
#[derive(Debug, PartialEq)]
pub enum CreateInterconnectError {
    ///<p>The API was called with invalid parameters. The error message will contain additional details about the cause.</p>
    DirectConnectClient(String),
    ///<p>A server-side error occurred during the API call. The error message will contain additional details about the cause.</p>
    DirectConnectServer(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl CreateInterconnectError {
    pub fn from_body(body: &str) -> CreateInterconnectError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "DirectConnectClientException" => {
                        CreateInterconnectError::DirectConnectClient(String::from(error_message))
                    }
                    "DirectConnectServerException" => {
                        CreateInterconnectError::DirectConnectServer(String::from(error_message))
                    }
                    "ValidationException" => {
                        CreateInterconnectError::Validation(error_message.to_string())
                    }
                    _ => CreateInterconnectError::Unknown(String::from(body)),
                }
            }
            Err(_) => CreateInterconnectError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for CreateInterconnectError {
    fn from(err: serde_json::error::Error) -> CreateInterconnectError {
        CreateInterconnectError::Unknown(err.description().to_string())
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
            CreateInterconnectError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by CreateLag
#[derive(Debug, PartialEq)]
pub enum CreateLagError {
    ///<p>The API was called with invalid parameters. The error message will contain additional details about the cause.</p>
    DirectConnectClient(String),
    ///<p>A server-side error occurred during the API call. The error message will contain additional details about the cause.</p>
    DirectConnectServer(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl CreateLagError {
    pub fn from_body(body: &str) -> CreateLagError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "DirectConnectClientException" => {
                        CreateLagError::DirectConnectClient(String::from(error_message))
                    }
                    "DirectConnectServerException" => {
                        CreateLagError::DirectConnectServer(String::from(error_message))
                    }
                    "ValidationException" => CreateLagError::Validation(error_message.to_string()),
                    _ => CreateLagError::Unknown(String::from(body)),
                }
            }
            Err(_) => CreateLagError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for CreateLagError {
    fn from(err: serde_json::error::Error) -> CreateLagError {
        CreateLagError::Unknown(err.description().to_string())
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
            CreateLagError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by CreatePrivateVirtualInterface
#[derive(Debug, PartialEq)]
pub enum CreatePrivateVirtualInterfaceError {
    ///<p>The API was called with invalid parameters. The error message will contain additional details about the cause.</p>
    DirectConnectClient(String),
    ///<p>A server-side error occurred during the API call. The error message will contain additional details about the cause.</p>
    DirectConnectServer(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl CreatePrivateVirtualInterfaceError {
    pub fn from_body(body: &str) -> CreatePrivateVirtualInterfaceError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "DirectConnectClientException" => CreatePrivateVirtualInterfaceError::DirectConnectClient(String::from(error_message)),
                    "DirectConnectServerException" => CreatePrivateVirtualInterfaceError::DirectConnectServer(String::from(error_message)),
                    "ValidationException" => {
                        CreatePrivateVirtualInterfaceError::Validation(error_message.to_string())
                    }
                    _ => CreatePrivateVirtualInterfaceError::Unknown(String::from(body)),
                }
            }
            Err(_) => CreatePrivateVirtualInterfaceError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for CreatePrivateVirtualInterfaceError {
    fn from(err: serde_json::error::Error) -> CreatePrivateVirtualInterfaceError {
        CreatePrivateVirtualInterfaceError::Unknown(err.description().to_string())
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
            CreatePrivateVirtualInterfaceError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by CreatePublicVirtualInterface
#[derive(Debug, PartialEq)]
pub enum CreatePublicVirtualInterfaceError {
    ///<p>The API was called with invalid parameters. The error message will contain additional details about the cause.</p>
    DirectConnectClient(String),
    ///<p>A server-side error occurred during the API call. The error message will contain additional details about the cause.</p>
    DirectConnectServer(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl CreatePublicVirtualInterfaceError {
    pub fn from_body(body: &str) -> CreatePublicVirtualInterfaceError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "DirectConnectClientException" => CreatePublicVirtualInterfaceError::DirectConnectClient(String::from(error_message)),
                    "DirectConnectServerException" => CreatePublicVirtualInterfaceError::DirectConnectServer(String::from(error_message)),
                    "ValidationException" => {
                        CreatePublicVirtualInterfaceError::Validation(error_message.to_string())
                    }
                    _ => CreatePublicVirtualInterfaceError::Unknown(String::from(body)),
                }
            }
            Err(_) => CreatePublicVirtualInterfaceError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for CreatePublicVirtualInterfaceError {
    fn from(err: serde_json::error::Error) -> CreatePublicVirtualInterfaceError {
        CreatePublicVirtualInterfaceError::Unknown(err.description().to_string())
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
            CreatePublicVirtualInterfaceError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DeleteBGPPeer
#[derive(Debug, PartialEq)]
pub enum DeleteBGPPeerError {
    ///<p>The API was called with invalid parameters. The error message will contain additional details about the cause.</p>
    DirectConnectClient(String),
    ///<p>A server-side error occurred during the API call. The error message will contain additional details about the cause.</p>
    DirectConnectServer(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl DeleteBGPPeerError {
    pub fn from_body(body: &str) -> DeleteBGPPeerError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "DirectConnectClientException" => {
                        DeleteBGPPeerError::DirectConnectClient(String::from(error_message))
                    }
                    "DirectConnectServerException" => {
                        DeleteBGPPeerError::DirectConnectServer(String::from(error_message))
                    }
                    "ValidationException" => {
                        DeleteBGPPeerError::Validation(error_message.to_string())
                    }
                    _ => DeleteBGPPeerError::Unknown(String::from(body)),
                }
            }
            Err(_) => DeleteBGPPeerError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DeleteBGPPeerError {
    fn from(err: serde_json::error::Error) -> DeleteBGPPeerError {
        DeleteBGPPeerError::Unknown(err.description().to_string())
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
            DeleteBGPPeerError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DeleteConnection
#[derive(Debug, PartialEq)]
pub enum DeleteConnectionError {
    ///<p>The API was called with invalid parameters. The error message will contain additional details about the cause.</p>
    DirectConnectClient(String),
    ///<p>A server-side error occurred during the API call. The error message will contain additional details about the cause.</p>
    DirectConnectServer(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl DeleteConnectionError {
    pub fn from_body(body: &str) -> DeleteConnectionError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "DirectConnectClientException" => {
                        DeleteConnectionError::DirectConnectClient(String::from(error_message))
                    }
                    "DirectConnectServerException" => {
                        DeleteConnectionError::DirectConnectServer(String::from(error_message))
                    }
                    "ValidationException" => {
                        DeleteConnectionError::Validation(error_message.to_string())
                    }
                    _ => DeleteConnectionError::Unknown(String::from(body)),
                }
            }
            Err(_) => DeleteConnectionError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DeleteConnectionError {
    fn from(err: serde_json::error::Error) -> DeleteConnectionError {
        DeleteConnectionError::Unknown(err.description().to_string())
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
            DeleteConnectionError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DeleteInterconnect
#[derive(Debug, PartialEq)]
pub enum DeleteInterconnectError {
    ///<p>The API was called with invalid parameters. The error message will contain additional details about the cause.</p>
    DirectConnectClient(String),
    ///<p>A server-side error occurred during the API call. The error message will contain additional details about the cause.</p>
    DirectConnectServer(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl DeleteInterconnectError {
    pub fn from_body(body: &str) -> DeleteInterconnectError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "DirectConnectClientException" => {
                        DeleteInterconnectError::DirectConnectClient(String::from(error_message))
                    }
                    "DirectConnectServerException" => {
                        DeleteInterconnectError::DirectConnectServer(String::from(error_message))
                    }
                    "ValidationException" => {
                        DeleteInterconnectError::Validation(error_message.to_string())
                    }
                    _ => DeleteInterconnectError::Unknown(String::from(body)),
                }
            }
            Err(_) => DeleteInterconnectError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DeleteInterconnectError {
    fn from(err: serde_json::error::Error) -> DeleteInterconnectError {
        DeleteInterconnectError::Unknown(err.description().to_string())
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
            DeleteInterconnectError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DeleteLag
#[derive(Debug, PartialEq)]
pub enum DeleteLagError {
    ///<p>The API was called with invalid parameters. The error message will contain additional details about the cause.</p>
    DirectConnectClient(String),
    ///<p>A server-side error occurred during the API call. The error message will contain additional details about the cause.</p>
    DirectConnectServer(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl DeleteLagError {
    pub fn from_body(body: &str) -> DeleteLagError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "DirectConnectClientException" => {
                        DeleteLagError::DirectConnectClient(String::from(error_message))
                    }
                    "DirectConnectServerException" => {
                        DeleteLagError::DirectConnectServer(String::from(error_message))
                    }
                    "ValidationException" => DeleteLagError::Validation(error_message.to_string()),
                    _ => DeleteLagError::Unknown(String::from(body)),
                }
            }
            Err(_) => DeleteLagError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DeleteLagError {
    fn from(err: serde_json::error::Error) -> DeleteLagError {
        DeleteLagError::Unknown(err.description().to_string())
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
            DeleteLagError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DeleteVirtualInterface
#[derive(Debug, PartialEq)]
pub enum DeleteVirtualInterfaceError {
    ///<p>The API was called with invalid parameters. The error message will contain additional details about the cause.</p>
    DirectConnectClient(String),
    ///<p>A server-side error occurred during the API call. The error message will contain additional details about the cause.</p>
    DirectConnectServer(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl DeleteVirtualInterfaceError {
    pub fn from_body(body: &str) -> DeleteVirtualInterfaceError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "DirectConnectClientException" => DeleteVirtualInterfaceError::DirectConnectClient(String::from(error_message)),
                    "DirectConnectServerException" => DeleteVirtualInterfaceError::DirectConnectServer(String::from(error_message)),
                    "ValidationException" => {
                        DeleteVirtualInterfaceError::Validation(error_message.to_string())
                    }
                    _ => DeleteVirtualInterfaceError::Unknown(String::from(body)),
                }
            }
            Err(_) => DeleteVirtualInterfaceError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DeleteVirtualInterfaceError {
    fn from(err: serde_json::error::Error) -> DeleteVirtualInterfaceError {
        DeleteVirtualInterfaceError::Unknown(err.description().to_string())
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
            DeleteVirtualInterfaceError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DescribeConnectionLoa
#[derive(Debug, PartialEq)]
pub enum DescribeConnectionLoaError {
    ///<p>The API was called with invalid parameters. The error message will contain additional details about the cause.</p>
    DirectConnectClient(String),
    ///<p>A server-side error occurred during the API call. The error message will contain additional details about the cause.</p>
    DirectConnectServer(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl DescribeConnectionLoaError {
    pub fn from_body(body: &str) -> DescribeConnectionLoaError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "DirectConnectClientException" => {
                        DescribeConnectionLoaError::DirectConnectClient(String::from(error_message))
                    }
                    "DirectConnectServerException" => {
                        DescribeConnectionLoaError::DirectConnectServer(String::from(error_message))
                    }
                    "ValidationException" => {
                        DescribeConnectionLoaError::Validation(error_message.to_string())
                    }
                    _ => DescribeConnectionLoaError::Unknown(String::from(body)),
                }
            }
            Err(_) => DescribeConnectionLoaError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DescribeConnectionLoaError {
    fn from(err: serde_json::error::Error) -> DescribeConnectionLoaError {
        DescribeConnectionLoaError::Unknown(err.description().to_string())
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
            DescribeConnectionLoaError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DescribeConnections
#[derive(Debug, PartialEq)]
pub enum DescribeConnectionsError {
    ///<p>The API was called with invalid parameters. The error message will contain additional details about the cause.</p>
    DirectConnectClient(String),
    ///<p>A server-side error occurred during the API call. The error message will contain additional details about the cause.</p>
    DirectConnectServer(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl DescribeConnectionsError {
    pub fn from_body(body: &str) -> DescribeConnectionsError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "DirectConnectClientException" => {
                        DescribeConnectionsError::DirectConnectClient(String::from(error_message))
                    }
                    "DirectConnectServerException" => {
                        DescribeConnectionsError::DirectConnectServer(String::from(error_message))
                    }
                    "ValidationException" => {
                        DescribeConnectionsError::Validation(error_message.to_string())
                    }
                    _ => DescribeConnectionsError::Unknown(String::from(body)),
                }
            }
            Err(_) => DescribeConnectionsError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DescribeConnectionsError {
    fn from(err: serde_json::error::Error) -> DescribeConnectionsError {
        DescribeConnectionsError::Unknown(err.description().to_string())
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
            DescribeConnectionsError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DescribeConnectionsOnInterconnect
#[derive(Debug, PartialEq)]
pub enum DescribeConnectionsOnInterconnectError {
    ///<p>The API was called with invalid parameters. The error message will contain additional details about the cause.</p>
    DirectConnectClient(String),
    ///<p>A server-side error occurred during the API call. The error message will contain additional details about the cause.</p>
    DirectConnectServer(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl DescribeConnectionsOnInterconnectError {
    pub fn from_body(body: &str) -> DescribeConnectionsOnInterconnectError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "DirectConnectClientException" => DescribeConnectionsOnInterconnectError::DirectConnectClient(String::from(error_message)),
                    "DirectConnectServerException" => DescribeConnectionsOnInterconnectError::DirectConnectServer(String::from(error_message)),
                    "ValidationException" => {
                        DescribeConnectionsOnInterconnectError::Validation(error_message
                                                                               .to_string())
                    }
                    _ => DescribeConnectionsOnInterconnectError::Unknown(String::from(body)),
                }
            }
            Err(_) => DescribeConnectionsOnInterconnectError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DescribeConnectionsOnInterconnectError {
    fn from(err: serde_json::error::Error) -> DescribeConnectionsOnInterconnectError {
        DescribeConnectionsOnInterconnectError::Unknown(err.description().to_string())
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
            DescribeConnectionsOnInterconnectError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DescribeHostedConnections
#[derive(Debug, PartialEq)]
pub enum DescribeHostedConnectionsError {
    ///<p>The API was called with invalid parameters. The error message will contain additional details about the cause.</p>
    DirectConnectClient(String),
    ///<p>A server-side error occurred during the API call. The error message will contain additional details about the cause.</p>
    DirectConnectServer(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl DescribeHostedConnectionsError {
    pub fn from_body(body: &str) -> DescribeHostedConnectionsError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "DirectConnectClientException" => DescribeHostedConnectionsError::DirectConnectClient(String::from(error_message)),
                    "DirectConnectServerException" => DescribeHostedConnectionsError::DirectConnectServer(String::from(error_message)),
                    "ValidationException" => {
                        DescribeHostedConnectionsError::Validation(error_message.to_string())
                    }
                    _ => DescribeHostedConnectionsError::Unknown(String::from(body)),
                }
            }
            Err(_) => DescribeHostedConnectionsError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DescribeHostedConnectionsError {
    fn from(err: serde_json::error::Error) -> DescribeHostedConnectionsError {
        DescribeHostedConnectionsError::Unknown(err.description().to_string())
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
            DescribeHostedConnectionsError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DescribeInterconnectLoa
#[derive(Debug, PartialEq)]
pub enum DescribeInterconnectLoaError {
    ///<p>The API was called with invalid parameters. The error message will contain additional details about the cause.</p>
    DirectConnectClient(String),
    ///<p>A server-side error occurred during the API call. The error message will contain additional details about the cause.</p>
    DirectConnectServer(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl DescribeInterconnectLoaError {
    pub fn from_body(body: &str) -> DescribeInterconnectLoaError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "DirectConnectClientException" => DescribeInterconnectLoaError::DirectConnectClient(String::from(error_message)),
                    "DirectConnectServerException" => DescribeInterconnectLoaError::DirectConnectServer(String::from(error_message)),
                    "ValidationException" => {
                        DescribeInterconnectLoaError::Validation(error_message.to_string())
                    }
                    _ => DescribeInterconnectLoaError::Unknown(String::from(body)),
                }
            }
            Err(_) => DescribeInterconnectLoaError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DescribeInterconnectLoaError {
    fn from(err: serde_json::error::Error) -> DescribeInterconnectLoaError {
        DescribeInterconnectLoaError::Unknown(err.description().to_string())
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
            DescribeInterconnectLoaError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DescribeInterconnects
#[derive(Debug, PartialEq)]
pub enum DescribeInterconnectsError {
    ///<p>The API was called with invalid parameters. The error message will contain additional details about the cause.</p>
    DirectConnectClient(String),
    ///<p>A server-side error occurred during the API call. The error message will contain additional details about the cause.</p>
    DirectConnectServer(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl DescribeInterconnectsError {
    pub fn from_body(body: &str) -> DescribeInterconnectsError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "DirectConnectClientException" => {
                        DescribeInterconnectsError::DirectConnectClient(String::from(error_message))
                    }
                    "DirectConnectServerException" => {
                        DescribeInterconnectsError::DirectConnectServer(String::from(error_message))
                    }
                    "ValidationException" => {
                        DescribeInterconnectsError::Validation(error_message.to_string())
                    }
                    _ => DescribeInterconnectsError::Unknown(String::from(body)),
                }
            }
            Err(_) => DescribeInterconnectsError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DescribeInterconnectsError {
    fn from(err: serde_json::error::Error) -> DescribeInterconnectsError {
        DescribeInterconnectsError::Unknown(err.description().to_string())
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
            DescribeInterconnectsError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DescribeLags
#[derive(Debug, PartialEq)]
pub enum DescribeLagsError {
    ///<p>The API was called with invalid parameters. The error message will contain additional details about the cause.</p>
    DirectConnectClient(String),
    ///<p>A server-side error occurred during the API call. The error message will contain additional details about the cause.</p>
    DirectConnectServer(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl DescribeLagsError {
    pub fn from_body(body: &str) -> DescribeLagsError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "DirectConnectClientException" => {
                        DescribeLagsError::DirectConnectClient(String::from(error_message))
                    }
                    "DirectConnectServerException" => {
                        DescribeLagsError::DirectConnectServer(String::from(error_message))
                    }
                    "ValidationException" => {
                        DescribeLagsError::Validation(error_message.to_string())
                    }
                    _ => DescribeLagsError::Unknown(String::from(body)),
                }
            }
            Err(_) => DescribeLagsError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DescribeLagsError {
    fn from(err: serde_json::error::Error) -> DescribeLagsError {
        DescribeLagsError::Unknown(err.description().to_string())
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
            DescribeLagsError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DescribeLoa
#[derive(Debug, PartialEq)]
pub enum DescribeLoaError {
    ///<p>The API was called with invalid parameters. The error message will contain additional details about the cause.</p>
    DirectConnectClient(String),
    ///<p>A server-side error occurred during the API call. The error message will contain additional details about the cause.</p>
    DirectConnectServer(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl DescribeLoaError {
    pub fn from_body(body: &str) -> DescribeLoaError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "DirectConnectClientException" => {
                        DescribeLoaError::DirectConnectClient(String::from(error_message))
                    }
                    "DirectConnectServerException" => {
                        DescribeLoaError::DirectConnectServer(String::from(error_message))
                    }
                    "ValidationException" => {
                        DescribeLoaError::Validation(error_message.to_string())
                    }
                    _ => DescribeLoaError::Unknown(String::from(body)),
                }
            }
            Err(_) => DescribeLoaError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DescribeLoaError {
    fn from(err: serde_json::error::Error) -> DescribeLoaError {
        DescribeLoaError::Unknown(err.description().to_string())
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
            DescribeLoaError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DescribeLocations
#[derive(Debug, PartialEq)]
pub enum DescribeLocationsError {
    ///<p>The API was called with invalid parameters. The error message will contain additional details about the cause.</p>
    DirectConnectClient(String),
    ///<p>A server-side error occurred during the API call. The error message will contain additional details about the cause.</p>
    DirectConnectServer(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl DescribeLocationsError {
    pub fn from_body(body: &str) -> DescribeLocationsError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "DirectConnectClientException" => {
                        DescribeLocationsError::DirectConnectClient(String::from(error_message))
                    }
                    "DirectConnectServerException" => {
                        DescribeLocationsError::DirectConnectServer(String::from(error_message))
                    }
                    "ValidationException" => {
                        DescribeLocationsError::Validation(error_message.to_string())
                    }
                    _ => DescribeLocationsError::Unknown(String::from(body)),
                }
            }
            Err(_) => DescribeLocationsError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DescribeLocationsError {
    fn from(err: serde_json::error::Error) -> DescribeLocationsError {
        DescribeLocationsError::Unknown(err.description().to_string())
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
            DescribeLocationsError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DescribeTags
#[derive(Debug, PartialEq)]
pub enum DescribeTagsError {
    ///<p>The API was called with invalid parameters. The error message will contain additional details about the cause.</p>
    DirectConnectClient(String),
    ///<p>A server-side error occurred during the API call. The error message will contain additional details about the cause.</p>
    DirectConnectServer(String),
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
                    "DirectConnectClientException" => {
                        DescribeTagsError::DirectConnectClient(String::from(error_message))
                    }
                    "DirectConnectServerException" => {
                        DescribeTagsError::DirectConnectServer(String::from(error_message))
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
            DescribeTagsError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DescribeVirtualGateways
#[derive(Debug, PartialEq)]
pub enum DescribeVirtualGatewaysError {
    ///<p>The API was called with invalid parameters. The error message will contain additional details about the cause.</p>
    DirectConnectClient(String),
    ///<p>A server-side error occurred during the API call. The error message will contain additional details about the cause.</p>
    DirectConnectServer(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl DescribeVirtualGatewaysError {
    pub fn from_body(body: &str) -> DescribeVirtualGatewaysError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "DirectConnectClientException" => DescribeVirtualGatewaysError::DirectConnectClient(String::from(error_message)),
                    "DirectConnectServerException" => DescribeVirtualGatewaysError::DirectConnectServer(String::from(error_message)),
                    "ValidationException" => {
                        DescribeVirtualGatewaysError::Validation(error_message.to_string())
                    }
                    _ => DescribeVirtualGatewaysError::Unknown(String::from(body)),
                }
            }
            Err(_) => DescribeVirtualGatewaysError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DescribeVirtualGatewaysError {
    fn from(err: serde_json::error::Error) -> DescribeVirtualGatewaysError {
        DescribeVirtualGatewaysError::Unknown(err.description().to_string())
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
            DescribeVirtualGatewaysError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DescribeVirtualInterfaces
#[derive(Debug, PartialEq)]
pub enum DescribeVirtualInterfacesError {
    ///<p>The API was called with invalid parameters. The error message will contain additional details about the cause.</p>
    DirectConnectClient(String),
    ///<p>A server-side error occurred during the API call. The error message will contain additional details about the cause.</p>
    DirectConnectServer(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl DescribeVirtualInterfacesError {
    pub fn from_body(body: &str) -> DescribeVirtualInterfacesError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "DirectConnectClientException" => DescribeVirtualInterfacesError::DirectConnectClient(String::from(error_message)),
                    "DirectConnectServerException" => DescribeVirtualInterfacesError::DirectConnectServer(String::from(error_message)),
                    "ValidationException" => {
                        DescribeVirtualInterfacesError::Validation(error_message.to_string())
                    }
                    _ => DescribeVirtualInterfacesError::Unknown(String::from(body)),
                }
            }
            Err(_) => DescribeVirtualInterfacesError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DescribeVirtualInterfacesError {
    fn from(err: serde_json::error::Error) -> DescribeVirtualInterfacesError {
        DescribeVirtualInterfacesError::Unknown(err.description().to_string())
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
            DescribeVirtualInterfacesError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DisassociateConnectionFromLag
#[derive(Debug, PartialEq)]
pub enum DisassociateConnectionFromLagError {
    ///<p>The API was called with invalid parameters. The error message will contain additional details about the cause.</p>
    DirectConnectClient(String),
    ///<p>A server-side error occurred during the API call. The error message will contain additional details about the cause.</p>
    DirectConnectServer(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl DisassociateConnectionFromLagError {
    pub fn from_body(body: &str) -> DisassociateConnectionFromLagError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "DirectConnectClientException" => DisassociateConnectionFromLagError::DirectConnectClient(String::from(error_message)),
                    "DirectConnectServerException" => DisassociateConnectionFromLagError::DirectConnectServer(String::from(error_message)),
                    "ValidationException" => {
                        DisassociateConnectionFromLagError::Validation(error_message.to_string())
                    }
                    _ => DisassociateConnectionFromLagError::Unknown(String::from(body)),
                }
            }
            Err(_) => DisassociateConnectionFromLagError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DisassociateConnectionFromLagError {
    fn from(err: serde_json::error::Error) -> DisassociateConnectionFromLagError {
        DisassociateConnectionFromLagError::Unknown(err.description().to_string())
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
            DisassociateConnectionFromLagError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by TagResource
#[derive(Debug, PartialEq)]
pub enum TagResourceError {
    ///<p>The API was called with invalid parameters. The error message will contain additional details about the cause.</p>
    DirectConnectClient(String),
    ///<p>A server-side error occurred during the API call. The error message will contain additional details about the cause.</p>
    DirectConnectServer(String),
    ///<p>A tag key was specified more than once.</p>
    DuplicateTagKeys(String),
    ///<p>You have reached the limit on the number of tags that can be assigned to a Direct Connect resource.</p>
    TooManyTags(String),
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
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "DirectConnectClientException" => {
                        TagResourceError::DirectConnectClient(String::from(error_message))
                    }
                    "DirectConnectServerException" => {
                        TagResourceError::DirectConnectServer(String::from(error_message))
                    }
                    "DuplicateTagKeysException" => {
                        TagResourceError::DuplicateTagKeys(String::from(error_message))
                    }
                    "TooManyTagsException" => {
                        TagResourceError::TooManyTags(String::from(error_message))
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
            TagResourceError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by UntagResource
#[derive(Debug, PartialEq)]
pub enum UntagResourceError {
    ///<p>The API was called with invalid parameters. The error message will contain additional details about the cause.</p>
    DirectConnectClient(String),
    ///<p>A server-side error occurred during the API call. The error message will contain additional details about the cause.</p>
    DirectConnectServer(String),
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
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "DirectConnectClientException" => {
                        UntagResourceError::DirectConnectClient(String::from(error_message))
                    }
                    "DirectConnectServerException" => {
                        UntagResourceError::DirectConnectServer(String::from(error_message))
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
            UntagResourceError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by UpdateLag
#[derive(Debug, PartialEq)]
pub enum UpdateLagError {
    ///<p>The API was called with invalid parameters. The error message will contain additional details about the cause.</p>
    DirectConnectClient(String),
    ///<p>A server-side error occurred during the API call. The error message will contain additional details about the cause.</p>
    DirectConnectServer(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl UpdateLagError {
    pub fn from_body(body: &str) -> UpdateLagError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "DirectConnectClientException" => {
                        UpdateLagError::DirectConnectClient(String::from(error_message))
                    }
                    "DirectConnectServerException" => {
                        UpdateLagError::DirectConnectServer(String::from(error_message))
                    }
                    "ValidationException" => UpdateLagError::Validation(error_message.to_string()),
                    _ => UpdateLagError::Unknown(String::from(body)),
                }
            }
            Err(_) => UpdateLagError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for UpdateLagError {
    fn from(err: serde_json::error::Error) -> UpdateLagError {
        UpdateLagError::Unknown(err.description().to_string())
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
            UpdateLagError::Unknown(ref cause) => cause,
        }
    }
}
/// Trait representing the capabilities of the AWS Direct Connect API. AWS Direct Connect clients implement this trait.
pub trait DirectConnect {
    #[doc="<p>Deprecated in favor of <a>AllocateHostedConnection</a>.</p> <p>Creates a hosted connection on an interconnect.</p> <p>Allocates a VLAN number and a specified amount of bandwidth for use by a hosted connection on the given interconnect.</p> <note> <p>This is intended for use by AWS Direct Connect partners only.</p> </note>"]
    fn allocate_connection_on_interconnect
        (&self,
         input: &AllocateConnectionOnInterconnectRequest)
         -> Result<Connection, AllocateConnectionOnInterconnectError>;


    #[doc="<p>Creates a hosted connection on an interconnect or a link aggregation group (LAG).</p> <p>Allocates a VLAN number and a specified amount of bandwidth for use by a hosted connection on the given interconnect or LAG.</p> <note> <p>This is intended for use by AWS Direct Connect partners only.</p> </note>"]
    fn allocate_hosted_connection(&self,
                                  input: &AllocateHostedConnectionRequest)
                                  -> Result<Connection, AllocateHostedConnectionError>;


    #[doc="<p>Provisions a private virtual interface to be owned by another AWS customer.</p> <p>Virtual interfaces created using this action must be confirmed by the virtual interface owner by using the <a>ConfirmPrivateVirtualInterface</a> action. Until then, the virtual interface will be in 'Confirming' state, and will not be available for handling traffic.</p>"]
    fn allocate_private_virtual_interface
        (&self,
         input: &AllocatePrivateVirtualInterfaceRequest)
         -> Result<VirtualInterface, AllocatePrivateVirtualInterfaceError>;


    #[doc="<p>Provisions a public virtual interface to be owned by a different customer.</p> <p>The owner of a connection calls this function to provision a public virtual interface which will be owned by another AWS customer.</p> <p>Virtual interfaces created using this function must be confirmed by the virtual interface owner by calling ConfirmPublicVirtualInterface. Until this step has been completed, the virtual interface will be in 'Confirming' state, and will not be available for handling traffic.</p> <p>When creating an IPv6 public virtual interface (addressFamily is 'ipv6'), the customer and amazon address fields should be left blank to use auto-assigned IPv6 space. Custom IPv6 Addresses are currently not supported.</p>"]
    fn allocate_public_virtual_interface
        (&self,
         input: &AllocatePublicVirtualInterfaceRequest)
         -> Result<VirtualInterface, AllocatePublicVirtualInterfaceError>;


    #[doc="<p>Associates an existing connection with a link aggregation group (LAG). The connection is interrupted and re-established as a member of the LAG (connectivity to AWS will be interrupted). The connection must be hosted on the same AWS Direct Connect endpoint as the LAG, and its bandwidth must match the bandwidth for the LAG. You can reassociate a connection that's currently associated with a different LAG; however, if removing the connection will cause the original LAG to fall below its setting for minimum number of operational connections, the request fails.</p> <p>Any virtual interfaces that are directly associated with the connection are automatically re-associated with the LAG. If the connection was originally associated with a different LAG, the virtual interfaces remain associated with the original LAG.</p> <p>For interconnects, any hosted connections are automatically re-associated with the LAG. If the interconnect was originally associated with a different LAG, the hosted connections remain associated with the original LAG.</p>"]
    fn associate_connection_with_lag(&self,
                                     input: &AssociateConnectionWithLagRequest)
                                     -> Result<Connection, AssociateConnectionWithLagError>;


    #[doc="<p>Associates a hosted connection and its virtual interfaces with a link aggregation group (LAG) or interconnect. If the target interconnect or LAG has an existing hosted connection with a conflicting VLAN number or IP address, the operation fails. This action temporarily interrupts the hosted connection's connectivity to AWS as it is being migrated.</p> <note> <p>This is intended for use by AWS Direct Connect partners only.</p> </note>"]
    fn associate_hosted_connection(&self,
                                   input: &AssociateHostedConnectionRequest)
                                   -> Result<Connection, AssociateHostedConnectionError>;


    #[doc="<p>Associates a virtual interface with a specified link aggregation group (LAG) or connection. Connectivity to AWS is temporarily interrupted as the virtual interface is being migrated. If the target connection or LAG has an associated virtual interface with a conflicting VLAN number or a conflicting IP address, the operation fails. </p> <p>Virtual interfaces associated with a hosted connection cannot be associated with a LAG; hosted connections must be migrated along with their virtual interfaces using <a>AssociateHostedConnection</a>.</p> <p>Hosted virtual interfaces (an interface for which the owner of the connection is not the owner of physical connection) can only be reassociated by the owner of the physical connection.</p>"]
    fn associate_virtual_interface(&self,
                                   input: &AssociateVirtualInterfaceRequest)
                                   -> Result<VirtualInterface, AssociateVirtualInterfaceError>;


    #[doc="<p>Confirm the creation of a hosted connection on an interconnect.</p> <p>Upon creation, the hosted connection is initially in the 'Ordering' state, and will remain in this state until the owner calls ConfirmConnection to confirm creation of the hosted connection.</p>"]
    fn confirm_connection(&self,
                          input: &ConfirmConnectionRequest)
                          -> Result<ConfirmConnectionResponse, ConfirmConnectionError>;


    #[doc="<p>Accept ownership of a private virtual interface created by another customer.</p> <p>After the virtual interface owner calls this function, the virtual interface will be created and attached to the given virtual private gateway, and will be available for handling traffic.</p>"]
    fn confirm_private_virtual_interface
        (&self,
         input: &ConfirmPrivateVirtualInterfaceRequest)
         -> Result<ConfirmPrivateVirtualInterfaceResponse, ConfirmPrivateVirtualInterfaceError>;


    #[doc="<p>Accept ownership of a public virtual interface created by another customer.</p> <p>After the virtual interface owner calls this function, the specified virtual interface will be created and made available for handling traffic.</p>"]
    fn confirm_public_virtual_interface
        (&self,
         input: &ConfirmPublicVirtualInterfaceRequest)
         -> Result<ConfirmPublicVirtualInterfaceResponse, ConfirmPublicVirtualInterfaceError>;


    #[doc="<p>Creates a new BGP peer on a specified virtual interface. The BGP peer cannot be in the same address family (IPv4/IPv6) of an existing BGP peer on the virtual interface.</p> <p>You must create a BGP peer for the corresponding address family in order to access AWS resources that also use that address family.</p> <p>When creating a IPv6 BGP peer, the Amazon address and customer address fields must be left blank. IPv6 addresses are automatically assigned from Amazon's pool of IPv6 addresses; you cannot specify custom IPv6 addresses.</p> <p>For a public virtual interface, the Autonomous System Number (ASN) must be private or already whitelisted for the virtual interface.</p>"]
    fn create_bgp_peer(&self,
                       input: &CreateBGPPeerRequest)
                       -> Result<CreateBGPPeerResponse, CreateBGPPeerError>;


    #[doc="<p>Creates a new connection between the customer network and a specific AWS Direct Connect location.</p> <p>A connection links your internal network to an AWS Direct Connect location over a standard 1 gigabit or 10 gigabit Ethernet fiber-optic cable. One end of the cable is connected to your router, the other to an AWS Direct Connect router. An AWS Direct Connect location provides access to Amazon Web Services in the region it is associated with. You can establish connections with AWS Direct Connect locations in multiple regions, but a connection in one region does not provide connectivity to other regions.</p> <p>You can automatically add the new connection to a link aggregation group (LAG) by specifying a LAG ID in the request. This ensures that the new connection is allocated on the same AWS Direct Connect endpoint that hosts the specified LAG. If there are no available ports on the endpoint, the request fails and no connection will be created.</p>"]
    fn create_connection(&self,
                         input: &CreateConnectionRequest)
                         -> Result<Connection, CreateConnectionError>;


    #[doc="<p>Creates a new interconnect between a AWS Direct Connect partner's network and a specific AWS Direct Connect location.</p> <p>An interconnect is a connection which is capable of hosting other connections. The AWS Direct Connect partner can use an interconnect to provide sub-1Gbps AWS Direct Connect service to tier 2 customers who do not have their own connections. Like a standard connection, an interconnect links the AWS Direct Connect partner's network to an AWS Direct Connect location over a standard 1 Gbps or 10 Gbps Ethernet fiber-optic cable. One end is connected to the partner's router, the other to an AWS Direct Connect router.</p> <p>You can automatically add the new interconnect to a link aggregation group (LAG) by specifying a LAG ID in the request. This ensures that the new interconnect is allocated on the same AWS Direct Connect endpoint that hosts the specified LAG. If there are no available ports on the endpoint, the request fails and no interconnect will be created.</p> <p>For each end customer, the AWS Direct Connect partner provisions a connection on their interconnect by calling AllocateConnectionOnInterconnect. The end customer can then connect to AWS resources by creating a virtual interface on their connection, using the VLAN assigned to them by the AWS Direct Connect partner.</p> <note> <p>This is intended for use by AWS Direct Connect partners only.</p> </note>"]
    fn create_interconnect(&self,
                           input: &CreateInterconnectRequest)
                           -> Result<Interconnect, CreateInterconnectError>;


    #[doc="<p>Creates a new link aggregation group (LAG) with the specified number of bundled physical connections between the customer network and a specific AWS Direct Connect location. A LAG is a logical interface that uses the Link Aggregation Control Protocol (LACP) to aggregate multiple 1 gigabit or 10 gigabit interfaces, allowing you to treat them as a single interface.</p> <p>All connections in a LAG must use the same bandwidth (for example, 10 Gbps), and must terminate at the same AWS Direct Connect endpoint.</p> <p>You can have up to 10 connections per LAG. Regardless of this limit, if you request more connections for the LAG than AWS Direct Connect can allocate on a single endpoint, no LAG is created.</p> <p>You can specify an existing physical connection or interconnect to include in the LAG (which counts towards the total number of connections). Doing so interrupts the current physical connection or hosted connections, and re-establishes them as a member of the LAG. The LAG will be created on the same AWS Direct Connect endpoint to which the connection terminates. Any virtual interfaces associated with the connection are automatically disassociated and re-associated with the LAG. The connection ID does not change.</p> <p>If the AWS account used to create a LAG is a registered AWS Direct Connect partner, the LAG is automatically enabled to host sub-connections. For a LAG owned by a partner, any associated virtual interfaces cannot be directly configured.</p>"]
    fn create_lag(&self, input: &CreateLagRequest) -> Result<Lag, CreateLagError>;


    #[doc="<p>Creates a new private virtual interface. A virtual interface is the VLAN that transports AWS Direct Connect traffic. A private virtual interface supports sending traffic to a single virtual private cloud (VPC).</p>"]
    fn create_private_virtual_interface
        (&self,
         input: &CreatePrivateVirtualInterfaceRequest)
         -> Result<VirtualInterface, CreatePrivateVirtualInterfaceError>;


    #[doc="<p>Creates a new public virtual interface. A virtual interface is the VLAN that transports AWS Direct Connect traffic. A public virtual interface supports sending traffic to public services of AWS such as Amazon Simple Storage Service (Amazon S3).</p> <p>When creating an IPv6 public virtual interface (addressFamily is 'ipv6'), the customer and amazon address fields should be left blank to use auto-assigned IPv6 space. Custom IPv6 Addresses are currently not supported.</p>"]
    fn create_public_virtual_interface
        (&self,
         input: &CreatePublicVirtualInterfaceRequest)
         -> Result<VirtualInterface, CreatePublicVirtualInterfaceError>;


    #[doc="<p>Deletes a BGP peer on the specified virtual interface that matches the specified customer address and ASN. You cannot delete the last BGP peer from a virtual interface.</p>"]
    fn delete_bgp_peer(&self,
                       input: &DeleteBGPPeerRequest)
                       -> Result<DeleteBGPPeerResponse, DeleteBGPPeerError>;


    #[doc="<p>Deletes the connection.</p> <p>Deleting a connection only stops the AWS Direct Connect port hour and data transfer charges. You need to cancel separately with the providers any services or charges for cross-connects or network circuits that connect you to the AWS Direct Connect location.</p>"]
    fn delete_connection(&self,
                         input: &DeleteConnectionRequest)
                         -> Result<Connection, DeleteConnectionError>;


    #[doc="<p>Deletes the specified interconnect.</p> <note> <p>This is intended for use by AWS Direct Connect partners only.</p> </note>"]
    fn delete_interconnect(&self,
                           input: &DeleteInterconnectRequest)
                           -> Result<DeleteInterconnectResponse, DeleteInterconnectError>;


    #[doc="<p>Deletes a link aggregation group (LAG). You cannot delete a LAG if it has active virtual interfaces or hosted connections.</p>"]
    fn delete_lag(&self, input: &DeleteLagRequest) -> Result<Lag, DeleteLagError>;


    #[doc="<p>Deletes a virtual interface.</p>"]
    fn delete_virtual_interface
        (&self,
         input: &DeleteVirtualInterfaceRequest)
         -> Result<DeleteVirtualInterfaceResponse, DeleteVirtualInterfaceError>;


    #[doc="<p>Deprecated in favor of <a>DescribeLoa</a>.</p> <p>Returns the LOA-CFA for a Connection.</p> <p>The Letter of Authorization - Connecting Facility Assignment (LOA-CFA) is a document that your APN partner or service provider uses when establishing your cross connect to AWS at the colocation facility. For more information, see <a href=\"http://docs.aws.amazon.com/directconnect/latest/UserGuide/Colocation.html\">Requesting Cross Connects at AWS Direct Connect Locations</a> in the AWS Direct Connect user guide.</p>"]
    fn describe_connection_loa
        (&self,
         input: &DescribeConnectionLoaRequest)
         -> Result<DescribeConnectionLoaResponse, DescribeConnectionLoaError>;


    #[doc="<p>Displays all connections in this region.</p> <p>If a connection ID is provided, the call returns only that particular connection.</p>"]
    fn describe_connections(&self,
                            input: &DescribeConnectionsRequest)
                            -> Result<Connections, DescribeConnectionsError>;


    #[doc="<p>Deprecated in favor of <a>DescribeHostedConnections</a>.</p> <p>Returns a list of connections that have been provisioned on the given interconnect.</p> <note> <p>This is intended for use by AWS Direct Connect partners only.</p> </note>"]
    fn describe_connections_on_interconnect
        (&self,
         input: &DescribeConnectionsOnInterconnectRequest)
         -> Result<Connections, DescribeConnectionsOnInterconnectError>;


    #[doc="<p>Returns a list of hosted connections that have been provisioned on the given interconnect or link aggregation group (LAG).</p> <note> <p>This is intended for use by AWS Direct Connect partners only.</p> </note>"]
    fn describe_hosted_connections(&self,
                                   input: &DescribeHostedConnectionsRequest)
                                   -> Result<Connections, DescribeHostedConnectionsError>;


    #[doc="<p>Deprecated in favor of <a>DescribeLoa</a>.</p> <p>Returns the LOA-CFA for an Interconnect.</p> <p>The Letter of Authorization - Connecting Facility Assignment (LOA-CFA) is a document that is used when establishing your cross connect to AWS at the colocation facility. For more information, see <a href=\"http://docs.aws.amazon.com/directconnect/latest/UserGuide/Colocation.html\">Requesting Cross Connects at AWS Direct Connect Locations</a> in the AWS Direct Connect user guide.</p>"]
    fn describe_interconnect_loa
        (&self,
         input: &DescribeInterconnectLoaRequest)
         -> Result<DescribeInterconnectLoaResponse, DescribeInterconnectLoaError>;


    #[doc="<p>Returns a list of interconnects owned by the AWS account.</p> <p>If an interconnect ID is provided, it will only return this particular interconnect.</p>"]
    fn describe_interconnects(&self,
                              input: &DescribeInterconnectsRequest)
                              -> Result<Interconnects, DescribeInterconnectsError>;


    #[doc="<p>Describes the link aggregation groups (LAGs) in your account. </p> <p>If a LAG ID is provided, only information about the specified LAG is returned.</p>"]
    fn describe_lags(&self, input: &DescribeLagsRequest) -> Result<Lags, DescribeLagsError>;


    #[doc="<p>Returns the LOA-CFA for a connection, interconnect, or link aggregation group (LAG).</p> <p>The Letter of Authorization - Connecting Facility Assignment (LOA-CFA) is a document that is used when establishing your cross connect to AWS at the colocation facility. For more information, see <a href=\"http://docs.aws.amazon.com/directconnect/latest/UserGuide/Colocation.html\">Requesting Cross Connects at AWS Direct Connect Locations</a> in the AWS Direct Connect user guide.</p>"]
    fn describe_loa(&self, input: &DescribeLoaRequest) -> Result<Loa, DescribeLoaError>;


    #[doc="<p>Returns the list of AWS Direct Connect locations in the current AWS region. These are the locations that may be selected when calling CreateConnection or CreateInterconnect.</p>"]
    fn describe_locations(&self) -> Result<Locations, DescribeLocationsError>;


    #[doc="<p>Describes the tags associated with the specified Direct Connect resources.</p>"]
    fn describe_tags(&self,
                     input: &DescribeTagsRequest)
                     -> Result<DescribeTagsResponse, DescribeTagsError>;


    #[doc="<p>Returns a list of virtual private gateways owned by the AWS account.</p> <p>You can create one or more AWS Direct Connect private virtual interfaces linking to a virtual private gateway. A virtual private gateway can be managed via Amazon Virtual Private Cloud (VPC) console or the <a href=\"http://docs.aws.amazon.com/AWSEC2/latest/APIReference/ApiReference-query-CreateVpnGateway.html\">EC2 CreateVpnGateway</a> action.</p>"]
    fn describe_virtual_gateways(&self) -> Result<VirtualGateways, DescribeVirtualGatewaysError>;


    #[doc="<p>Displays all virtual interfaces for an AWS account. Virtual interfaces deleted fewer than 15 minutes before you make the request are also returned. If you specify a connection ID, only the virtual interfaces associated with the connection are returned. If you specify a virtual interface ID, then only a single virtual interface is returned.</p> <p>A virtual interface (VLAN) transmits the traffic between the AWS Direct Connect location and the customer.</p>"]
    fn describe_virtual_interfaces(&self,
                                   input: &DescribeVirtualInterfacesRequest)
                                   -> Result<VirtualInterfaces, DescribeVirtualInterfacesError>;


    #[doc="<p>Disassociates a connection from a link aggregation group (LAG). The connection is interrupted and re-established as a standalone connection (the connection is not deleted; to delete the connection, use the <a>DeleteConnection</a> request). If the LAG has associated virtual interfaces or hosted connections, they remain associated with the LAG. A disassociated connection owned by an AWS Direct Connect partner is automatically converted to an interconnect.</p> <p>If disassociating the connection will cause the LAG to fall below its setting for minimum number of operational connections, the request fails, except when it's the last member of the LAG. If all connections are disassociated, the LAG continues to exist as an empty LAG with no physical connections. </p>"]
    fn disassociate_connection_from_lag
        (&self,
         input: &DisassociateConnectionFromLagRequest)
         -> Result<Connection, DisassociateConnectionFromLagError>;


    #[doc="<p>Adds the specified tags to the specified Direct Connect resource. Each Direct Connect resource can have a maximum of 50 tags.</p> <p>Each tag consists of a key and an optional value. If a tag with the same key is already associated with the Direct Connect resource, this action updates its value.</p>"]
    fn tag_resource(&self,
                    input: &TagResourceRequest)
                    -> Result<TagResourceResponse, TagResourceError>;


    #[doc="<p>Removes one or more tags from the specified Direct Connect resource.</p>"]
    fn untag_resource(&self,
                      input: &UntagResourceRequest)
                      -> Result<UntagResourceResponse, UntagResourceError>;


    #[doc="<p>Updates the attributes of a link aggregation group (LAG). </p> <p>You can update the following attributes: </p> <ul> <li> <p>The name of the LAG.</p> </li> <li> <p>The value for the minimum number of connections that must be operational for the LAG itself to be operational. </p> </li> </ul> <p>When you create a LAG, the default value for the minimum number of operational connections is zero (0). If you update this value, and the number of operational connections falls below the specified value, the LAG will automatically go down to avoid overutilization of the remaining connections. Adjusting this value should be done with care as it could force the LAG down if the value is set higher than the current number of operational connections.</p>"]
    fn update_lag(&self, input: &UpdateLagRequest) -> Result<Lag, UpdateLagError>;
}
/// A client for the AWS Direct Connect API.
pub struct DirectConnectClient<P, D>
    where P: ProvideAwsCredentials,
          D: DispatchSignedRequest
{
    credentials_provider: P,
    region: region::Region,
    dispatcher: D,
}

impl<P, D> DirectConnectClient<P, D>
    where P: ProvideAwsCredentials,
          D: DispatchSignedRequest
{
    pub fn new(request_dispatcher: D, credentials_provider: P, region: region::Region) -> Self {
        DirectConnectClient {
            credentials_provider: credentials_provider,
            region: region,
            dispatcher: request_dispatcher,
        }
    }
}

impl<P, D> DirectConnect for DirectConnectClient<P, D>
    where P: ProvideAwsCredentials,
          D: DispatchSignedRequest
{
    #[doc="<p>Deprecated in favor of <a>AllocateHostedConnection</a>.</p> <p>Creates a hosted connection on an interconnect.</p> <p>Allocates a VLAN number and a specified amount of bandwidth for use by a hosted connection on the given interconnect.</p> <note> <p>This is intended for use by AWS Direct Connect partners only.</p> </note>"]
    fn allocate_connection_on_interconnect
        (&self,
         input: &AllocateConnectionOnInterconnectRequest)
         -> Result<Connection, AllocateConnectionOnInterconnectError> {
        let mut request = SignedRequest::new("POST", "directconnect", self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target",
                           "OvertureService.AllocateConnectionOnInterconnect");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign(&try!(self.credentials_provider.credentials()));

        let response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            ::hyper::status::StatusCode::Ok => {
                Ok(serde_json::from_str::<Connection>(String::from_utf8_lossy(&response.body)
                                                          .as_ref())
                           .unwrap())
            }
            _ => Err(AllocateConnectionOnInterconnectError::from_body(String::from_utf8_lossy(&response.body).as_ref())),
        }
    }


    #[doc="<p>Creates a hosted connection on an interconnect or a link aggregation group (LAG).</p> <p>Allocates a VLAN number and a specified amount of bandwidth for use by a hosted connection on the given interconnect or LAG.</p> <note> <p>This is intended for use by AWS Direct Connect partners only.</p> </note>"]
    fn allocate_hosted_connection(&self,
                                  input: &AllocateHostedConnectionRequest)
                                  -> Result<Connection, AllocateHostedConnectionError> {
        let mut request = SignedRequest::new("POST", "directconnect", self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "OvertureService.AllocateHostedConnection");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign(&try!(self.credentials_provider.credentials()));

        let response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            ::hyper::status::StatusCode::Ok => {
                Ok(serde_json::from_str::<Connection>(String::from_utf8_lossy(&response.body)
                                                          .as_ref())
                           .unwrap())
            }
            _ => Err(AllocateHostedConnectionError::from_body(String::from_utf8_lossy(&response.body).as_ref())),
        }
    }


    #[doc="<p>Provisions a private virtual interface to be owned by another AWS customer.</p> <p>Virtual interfaces created using this action must be confirmed by the virtual interface owner by using the <a>ConfirmPrivateVirtualInterface</a> action. Until then, the virtual interface will be in 'Confirming' state, and will not be available for handling traffic.</p>"]
    fn allocate_private_virtual_interface
        (&self,
         input: &AllocatePrivateVirtualInterfaceRequest)
         -> Result<VirtualInterface, AllocatePrivateVirtualInterfaceError> {
        let mut request = SignedRequest::new("POST", "directconnect", self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target",
                           "OvertureService.AllocatePrivateVirtualInterface");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign(&try!(self.credentials_provider.credentials()));

        let response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            ::hyper::status::StatusCode::Ok => {
                            Ok(serde_json::from_str::<VirtualInterface>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
                        }
            _ => Err(AllocatePrivateVirtualInterfaceError::from_body(String::from_utf8_lossy(&response.body).as_ref())),
        }
    }


    #[doc="<p>Provisions a public virtual interface to be owned by a different customer.</p> <p>The owner of a connection calls this function to provision a public virtual interface which will be owned by another AWS customer.</p> <p>Virtual interfaces created using this function must be confirmed by the virtual interface owner by calling ConfirmPublicVirtualInterface. Until this step has been completed, the virtual interface will be in 'Confirming' state, and will not be available for handling traffic.</p> <p>When creating an IPv6 public virtual interface (addressFamily is 'ipv6'), the customer and amazon address fields should be left blank to use auto-assigned IPv6 space. Custom IPv6 Addresses are currently not supported.</p>"]
    fn allocate_public_virtual_interface
        (&self,
         input: &AllocatePublicVirtualInterfaceRequest)
         -> Result<VirtualInterface, AllocatePublicVirtualInterfaceError> {
        let mut request = SignedRequest::new("POST", "directconnect", self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target",
                           "OvertureService.AllocatePublicVirtualInterface");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign(&try!(self.credentials_provider.credentials()));

        let response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            ::hyper::status::StatusCode::Ok => {
                            Ok(serde_json::from_str::<VirtualInterface>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
                        }
            _ => Err(AllocatePublicVirtualInterfaceError::from_body(String::from_utf8_lossy(&response.body).as_ref())),
        }
    }


    #[doc="<p>Associates an existing connection with a link aggregation group (LAG). The connection is interrupted and re-established as a member of the LAG (connectivity to AWS will be interrupted). The connection must be hosted on the same AWS Direct Connect endpoint as the LAG, and its bandwidth must match the bandwidth for the LAG. You can reassociate a connection that's currently associated with a different LAG; however, if removing the connection will cause the original LAG to fall below its setting for minimum number of operational connections, the request fails.</p> <p>Any virtual interfaces that are directly associated with the connection are automatically re-associated with the LAG. If the connection was originally associated with a different LAG, the virtual interfaces remain associated with the original LAG.</p> <p>For interconnects, any hosted connections are automatically re-associated with the LAG. If the interconnect was originally associated with a different LAG, the hosted connections remain associated with the original LAG.</p>"]
    fn associate_connection_with_lag(&self,
                                     input: &AssociateConnectionWithLagRequest)
                                     -> Result<Connection, AssociateConnectionWithLagError> {
        let mut request = SignedRequest::new("POST", "directconnect", self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "OvertureService.AssociateConnectionWithLag");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign(&try!(self.credentials_provider.credentials()));

        let response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            ::hyper::status::StatusCode::Ok => {
                Ok(serde_json::from_str::<Connection>(String::from_utf8_lossy(&response.body)
                                                          .as_ref())
                           .unwrap())
            }
            _ => Err(AssociateConnectionWithLagError::from_body(String::from_utf8_lossy(&response.body).as_ref())),
        }
    }


    #[doc="<p>Associates a hosted connection and its virtual interfaces with a link aggregation group (LAG) or interconnect. If the target interconnect or LAG has an existing hosted connection with a conflicting VLAN number or IP address, the operation fails. This action temporarily interrupts the hosted connection's connectivity to AWS as it is being migrated.</p> <note> <p>This is intended for use by AWS Direct Connect partners only.</p> </note>"]
    fn associate_hosted_connection(&self,
                                   input: &AssociateHostedConnectionRequest)
                                   -> Result<Connection, AssociateHostedConnectionError> {
        let mut request = SignedRequest::new("POST", "directconnect", self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "OvertureService.AssociateHostedConnection");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign(&try!(self.credentials_provider.credentials()));

        let response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            ::hyper::status::StatusCode::Ok => {
                Ok(serde_json::from_str::<Connection>(String::from_utf8_lossy(&response.body)
                                                          .as_ref())
                           .unwrap())
            }
            _ => Err(AssociateHostedConnectionError::from_body(String::from_utf8_lossy(&response.body).as_ref())),
        }
    }


    #[doc="<p>Associates a virtual interface with a specified link aggregation group (LAG) or connection. Connectivity to AWS is temporarily interrupted as the virtual interface is being migrated. If the target connection or LAG has an associated virtual interface with a conflicting VLAN number or a conflicting IP address, the operation fails. </p> <p>Virtual interfaces associated with a hosted connection cannot be associated with a LAG; hosted connections must be migrated along with their virtual interfaces using <a>AssociateHostedConnection</a>.</p> <p>Hosted virtual interfaces (an interface for which the owner of the connection is not the owner of physical connection) can only be reassociated by the owner of the physical connection.</p>"]
    fn associate_virtual_interface(&self,
                                   input: &AssociateVirtualInterfaceRequest)
                                   -> Result<VirtualInterface, AssociateVirtualInterfaceError> {
        let mut request = SignedRequest::new("POST", "directconnect", self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "OvertureService.AssociateVirtualInterface");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign(&try!(self.credentials_provider.credentials()));

        let response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            ::hyper::status::StatusCode::Ok => {
                            Ok(serde_json::from_str::<VirtualInterface>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
                        }
            _ => Err(AssociateVirtualInterfaceError::from_body(String::from_utf8_lossy(&response.body).as_ref())),
        }
    }


    #[doc="<p>Confirm the creation of a hosted connection on an interconnect.</p> <p>Upon creation, the hosted connection is initially in the 'Ordering' state, and will remain in this state until the owner calls ConfirmConnection to confirm creation of the hosted connection.</p>"]
    fn confirm_connection(&self,
                          input: &ConfirmConnectionRequest)
                          -> Result<ConfirmConnectionResponse, ConfirmConnectionError> {
        let mut request = SignedRequest::new("POST", "directconnect", self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "OvertureService.ConfirmConnection");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign(&try!(self.credentials_provider.credentials()));

        let response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            ::hyper::status::StatusCode::Ok => {
                            Ok(serde_json::from_str::<ConfirmConnectionResponse>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
                        }
            _ => {
                Err(ConfirmConnectionError::from_body(String::from_utf8_lossy(&response.body)
                                                          .as_ref()))
            }
        }
    }


    #[doc="<p>Accept ownership of a private virtual interface created by another customer.</p> <p>After the virtual interface owner calls this function, the virtual interface will be created and attached to the given virtual private gateway, and will be available for handling traffic.</p>"]
    fn confirm_private_virtual_interface
        (&self,
         input: &ConfirmPrivateVirtualInterfaceRequest)
         -> Result<ConfirmPrivateVirtualInterfaceResponse, ConfirmPrivateVirtualInterfaceError> {
        let mut request = SignedRequest::new("POST", "directconnect", self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target",
                           "OvertureService.ConfirmPrivateVirtualInterface");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign(&try!(self.credentials_provider.credentials()));

        let response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            ::hyper::status::StatusCode::Ok => {
                            Ok(serde_json::from_str::<ConfirmPrivateVirtualInterfaceResponse>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
                        }
            _ => Err(ConfirmPrivateVirtualInterfaceError::from_body(String::from_utf8_lossy(&response.body).as_ref())),
        }
    }


    #[doc="<p>Accept ownership of a public virtual interface created by another customer.</p> <p>After the virtual interface owner calls this function, the specified virtual interface will be created and made available for handling traffic.</p>"]
    fn confirm_public_virtual_interface
        (&self,
         input: &ConfirmPublicVirtualInterfaceRequest)
         -> Result<ConfirmPublicVirtualInterfaceResponse, ConfirmPublicVirtualInterfaceError> {
        let mut request = SignedRequest::new("POST", "directconnect", self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target",
                           "OvertureService.ConfirmPublicVirtualInterface");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign(&try!(self.credentials_provider.credentials()));

        let response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            ::hyper::status::StatusCode::Ok => {
                            Ok(serde_json::from_str::<ConfirmPublicVirtualInterfaceResponse>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
                        }
            _ => Err(ConfirmPublicVirtualInterfaceError::from_body(String::from_utf8_lossy(&response.body).as_ref())),
        }
    }


    #[doc="<p>Creates a new BGP peer on a specified virtual interface. The BGP peer cannot be in the same address family (IPv4/IPv6) of an existing BGP peer on the virtual interface.</p> <p>You must create a BGP peer for the corresponding address family in order to access AWS resources that also use that address family.</p> <p>When creating a IPv6 BGP peer, the Amazon address and customer address fields must be left blank. IPv6 addresses are automatically assigned from Amazon's pool of IPv6 addresses; you cannot specify custom IPv6 addresses.</p> <p>For a public virtual interface, the Autonomous System Number (ASN) must be private or already whitelisted for the virtual interface.</p>"]
    fn create_bgp_peer(&self,
                       input: &CreateBGPPeerRequest)
                       -> Result<CreateBGPPeerResponse, CreateBGPPeerError> {
        let mut request = SignedRequest::new("POST", "directconnect", self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "OvertureService.CreateBGPPeer");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign(&try!(self.credentials_provider.credentials()));

        let response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            ::hyper::status::StatusCode::Ok => {
                            Ok(serde_json::from_str::<CreateBGPPeerResponse>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
                        }
            _ => {
                Err(CreateBGPPeerError::from_body(String::from_utf8_lossy(&response.body).as_ref()))
            }
        }
    }


    #[doc="<p>Creates a new connection between the customer network and a specific AWS Direct Connect location.</p> <p>A connection links your internal network to an AWS Direct Connect location over a standard 1 gigabit or 10 gigabit Ethernet fiber-optic cable. One end of the cable is connected to your router, the other to an AWS Direct Connect router. An AWS Direct Connect location provides access to Amazon Web Services in the region it is associated with. You can establish connections with AWS Direct Connect locations in multiple regions, but a connection in one region does not provide connectivity to other regions.</p> <p>You can automatically add the new connection to a link aggregation group (LAG) by specifying a LAG ID in the request. This ensures that the new connection is allocated on the same AWS Direct Connect endpoint that hosts the specified LAG. If there are no available ports on the endpoint, the request fails and no connection will be created.</p>"]
    fn create_connection(&self,
                         input: &CreateConnectionRequest)
                         -> Result<Connection, CreateConnectionError> {
        let mut request = SignedRequest::new("POST", "directconnect", self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "OvertureService.CreateConnection");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign(&try!(self.credentials_provider.credentials()));

        let response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            ::hyper::status::StatusCode::Ok => {
                Ok(serde_json::from_str::<Connection>(String::from_utf8_lossy(&response.body)
                                                          .as_ref())
                           .unwrap())
            }
            _ => {
                Err(CreateConnectionError::from_body(String::from_utf8_lossy(&response.body)
                                                         .as_ref()))
            }
        }
    }


    #[doc="<p>Creates a new interconnect between a AWS Direct Connect partner's network and a specific AWS Direct Connect location.</p> <p>An interconnect is a connection which is capable of hosting other connections. The AWS Direct Connect partner can use an interconnect to provide sub-1Gbps AWS Direct Connect service to tier 2 customers who do not have their own connections. Like a standard connection, an interconnect links the AWS Direct Connect partner's network to an AWS Direct Connect location over a standard 1 Gbps or 10 Gbps Ethernet fiber-optic cable. One end is connected to the partner's router, the other to an AWS Direct Connect router.</p> <p>You can automatically add the new interconnect to a link aggregation group (LAG) by specifying a LAG ID in the request. This ensures that the new interconnect is allocated on the same AWS Direct Connect endpoint that hosts the specified LAG. If there are no available ports on the endpoint, the request fails and no interconnect will be created.</p> <p>For each end customer, the AWS Direct Connect partner provisions a connection on their interconnect by calling AllocateConnectionOnInterconnect. The end customer can then connect to AWS resources by creating a virtual interface on their connection, using the VLAN assigned to them by the AWS Direct Connect partner.</p> <note> <p>This is intended for use by AWS Direct Connect partners only.</p> </note>"]
    fn create_interconnect(&self,
                           input: &CreateInterconnectRequest)
                           -> Result<Interconnect, CreateInterconnectError> {
        let mut request = SignedRequest::new("POST", "directconnect", self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "OvertureService.CreateInterconnect");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign(&try!(self.credentials_provider.credentials()));

        let response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            ::hyper::status::StatusCode::Ok => {
                Ok(serde_json::from_str::<Interconnect>(String::from_utf8_lossy(&response.body)
                                                            .as_ref())
                           .unwrap())
            }
            _ => {
                Err(CreateInterconnectError::from_body(String::from_utf8_lossy(&response.body)
                                                           .as_ref()))
            }
        }
    }


    #[doc="<p>Creates a new link aggregation group (LAG) with the specified number of bundled physical connections between the customer network and a specific AWS Direct Connect location. A LAG is a logical interface that uses the Link Aggregation Control Protocol (LACP) to aggregate multiple 1 gigabit or 10 gigabit interfaces, allowing you to treat them as a single interface.</p> <p>All connections in a LAG must use the same bandwidth (for example, 10 Gbps), and must terminate at the same AWS Direct Connect endpoint.</p> <p>You can have up to 10 connections per LAG. Regardless of this limit, if you request more connections for the LAG than AWS Direct Connect can allocate on a single endpoint, no LAG is created.</p> <p>You can specify an existing physical connection or interconnect to include in the LAG (which counts towards the total number of connections). Doing so interrupts the current physical connection or hosted connections, and re-establishes them as a member of the LAG. The LAG will be created on the same AWS Direct Connect endpoint to which the connection terminates. Any virtual interfaces associated with the connection are automatically disassociated and re-associated with the LAG. The connection ID does not change.</p> <p>If the AWS account used to create a LAG is a registered AWS Direct Connect partner, the LAG is automatically enabled to host sub-connections. For a LAG owned by a partner, any associated virtual interfaces cannot be directly configured.</p>"]
    fn create_lag(&self, input: &CreateLagRequest) -> Result<Lag, CreateLagError> {
        let mut request = SignedRequest::new("POST", "directconnect", self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "OvertureService.CreateLag");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign(&try!(self.credentials_provider.credentials()));

        let response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            ::hyper::status::StatusCode::Ok => {
                Ok(serde_json::from_str::<Lag>(String::from_utf8_lossy(&response.body).as_ref())
                       .unwrap())
            }
            _ => Err(CreateLagError::from_body(String::from_utf8_lossy(&response.body).as_ref())),
        }
    }


    #[doc="<p>Creates a new private virtual interface. A virtual interface is the VLAN that transports AWS Direct Connect traffic. A private virtual interface supports sending traffic to a single virtual private cloud (VPC).</p>"]
    fn create_private_virtual_interface
        (&self,
         input: &CreatePrivateVirtualInterfaceRequest)
         -> Result<VirtualInterface, CreatePrivateVirtualInterfaceError> {
        let mut request = SignedRequest::new("POST", "directconnect", self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target",
                           "OvertureService.CreatePrivateVirtualInterface");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign(&try!(self.credentials_provider.credentials()));

        let response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            ::hyper::status::StatusCode::Ok => {
                            Ok(serde_json::from_str::<VirtualInterface>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
                        }
            _ => Err(CreatePrivateVirtualInterfaceError::from_body(String::from_utf8_lossy(&response.body).as_ref())),
        }
    }


    #[doc="<p>Creates a new public virtual interface. A virtual interface is the VLAN that transports AWS Direct Connect traffic. A public virtual interface supports sending traffic to public services of AWS such as Amazon Simple Storage Service (Amazon S3).</p> <p>When creating an IPv6 public virtual interface (addressFamily is 'ipv6'), the customer and amazon address fields should be left blank to use auto-assigned IPv6 space. Custom IPv6 Addresses are currently not supported.</p>"]
    fn create_public_virtual_interface
        (&self,
         input: &CreatePublicVirtualInterfaceRequest)
         -> Result<VirtualInterface, CreatePublicVirtualInterfaceError> {
        let mut request = SignedRequest::new("POST", "directconnect", self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target",
                           "OvertureService.CreatePublicVirtualInterface");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign(&try!(self.credentials_provider.credentials()));

        let response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            ::hyper::status::StatusCode::Ok => {
                            Ok(serde_json::from_str::<VirtualInterface>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
                        }
            _ => Err(CreatePublicVirtualInterfaceError::from_body(String::from_utf8_lossy(&response.body).as_ref())),
        }
    }


    #[doc="<p>Deletes a BGP peer on the specified virtual interface that matches the specified customer address and ASN. You cannot delete the last BGP peer from a virtual interface.</p>"]
    fn delete_bgp_peer(&self,
                       input: &DeleteBGPPeerRequest)
                       -> Result<DeleteBGPPeerResponse, DeleteBGPPeerError> {
        let mut request = SignedRequest::new("POST", "directconnect", self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "OvertureService.DeleteBGPPeer");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign(&try!(self.credentials_provider.credentials()));

        let response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            ::hyper::status::StatusCode::Ok => {
                            Ok(serde_json::from_str::<DeleteBGPPeerResponse>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
                        }
            _ => {
                Err(DeleteBGPPeerError::from_body(String::from_utf8_lossy(&response.body).as_ref()))
            }
        }
    }


    #[doc="<p>Deletes the connection.</p> <p>Deleting a connection only stops the AWS Direct Connect port hour and data transfer charges. You need to cancel separately with the providers any services or charges for cross-connects or network circuits that connect you to the AWS Direct Connect location.</p>"]
    fn delete_connection(&self,
                         input: &DeleteConnectionRequest)
                         -> Result<Connection, DeleteConnectionError> {
        let mut request = SignedRequest::new("POST", "directconnect", self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "OvertureService.DeleteConnection");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign(&try!(self.credentials_provider.credentials()));

        let response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            ::hyper::status::StatusCode::Ok => {
                Ok(serde_json::from_str::<Connection>(String::from_utf8_lossy(&response.body)
                                                          .as_ref())
                           .unwrap())
            }
            _ => {
                Err(DeleteConnectionError::from_body(String::from_utf8_lossy(&response.body)
                                                         .as_ref()))
            }
        }
    }


    #[doc="<p>Deletes the specified interconnect.</p> <note> <p>This is intended for use by AWS Direct Connect partners only.</p> </note>"]
    fn delete_interconnect(&self,
                           input: &DeleteInterconnectRequest)
                           -> Result<DeleteInterconnectResponse, DeleteInterconnectError> {
        let mut request = SignedRequest::new("POST", "directconnect", self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "OvertureService.DeleteInterconnect");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign(&try!(self.credentials_provider.credentials()));

        let response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            ::hyper::status::StatusCode::Ok => {
                            Ok(serde_json::from_str::<DeleteInterconnectResponse>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
                        }
            _ => {
                Err(DeleteInterconnectError::from_body(String::from_utf8_lossy(&response.body)
                                                           .as_ref()))
            }
        }
    }


    #[doc="<p>Deletes a link aggregation group (LAG). You cannot delete a LAG if it has active virtual interfaces or hosted connections.</p>"]
    fn delete_lag(&self, input: &DeleteLagRequest) -> Result<Lag, DeleteLagError> {
        let mut request = SignedRequest::new("POST", "directconnect", self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "OvertureService.DeleteLag");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign(&try!(self.credentials_provider.credentials()));

        let response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            ::hyper::status::StatusCode::Ok => {
                Ok(serde_json::from_str::<Lag>(String::from_utf8_lossy(&response.body).as_ref())
                       .unwrap())
            }
            _ => Err(DeleteLagError::from_body(String::from_utf8_lossy(&response.body).as_ref())),
        }
    }


    #[doc="<p>Deletes a virtual interface.</p>"]
    fn delete_virtual_interface
        (&self,
         input: &DeleteVirtualInterfaceRequest)
         -> Result<DeleteVirtualInterfaceResponse, DeleteVirtualInterfaceError> {
        let mut request = SignedRequest::new("POST", "directconnect", self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "OvertureService.DeleteVirtualInterface");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign(&try!(self.credentials_provider.credentials()));

        let response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            ::hyper::status::StatusCode::Ok => {
                            Ok(serde_json::from_str::<DeleteVirtualInterfaceResponse>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
                        }
            _ => {
                Err(DeleteVirtualInterfaceError::from_body(String::from_utf8_lossy(&response.body)
                                                               .as_ref()))
            }
        }
    }


    #[doc="<p>Deprecated in favor of <a>DescribeLoa</a>.</p> <p>Returns the LOA-CFA for a Connection.</p> <p>The Letter of Authorization - Connecting Facility Assignment (LOA-CFA) is a document that your APN partner or service provider uses when establishing your cross connect to AWS at the colocation facility. For more information, see <a href=\"http://docs.aws.amazon.com/directconnect/latest/UserGuide/Colocation.html\">Requesting Cross Connects at AWS Direct Connect Locations</a> in the AWS Direct Connect user guide.</p>"]
    fn describe_connection_loa
        (&self,
         input: &DescribeConnectionLoaRequest)
         -> Result<DescribeConnectionLoaResponse, DescribeConnectionLoaError> {
        let mut request = SignedRequest::new("POST", "directconnect", self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "OvertureService.DescribeConnectionLoa");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign(&try!(self.credentials_provider.credentials()));

        let response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            ::hyper::status::StatusCode::Ok => {
                            Ok(serde_json::from_str::<DescribeConnectionLoaResponse>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
                        }
            _ => {
                Err(DescribeConnectionLoaError::from_body(String::from_utf8_lossy(&response.body)
                                                              .as_ref()))
            }
        }
    }


    #[doc="<p>Displays all connections in this region.</p> <p>If a connection ID is provided, the call returns only that particular connection.</p>"]
    fn describe_connections(&self,
                            input: &DescribeConnectionsRequest)
                            -> Result<Connections, DescribeConnectionsError> {
        let mut request = SignedRequest::new("POST", "directconnect", self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "OvertureService.DescribeConnections");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign(&try!(self.credentials_provider.credentials()));

        let response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            ::hyper::status::StatusCode::Ok => {
                Ok(serde_json::from_str::<Connections>(String::from_utf8_lossy(&response.body)
                                                           .as_ref())
                           .unwrap())
            }
            _ => {
                Err(DescribeConnectionsError::from_body(String::from_utf8_lossy(&response.body)
                                                            .as_ref()))
            }
        }
    }


    #[doc="<p>Deprecated in favor of <a>DescribeHostedConnections</a>.</p> <p>Returns a list of connections that have been provisioned on the given interconnect.</p> <note> <p>This is intended for use by AWS Direct Connect partners only.</p> </note>"]
    fn describe_connections_on_interconnect
        (&self,
         input: &DescribeConnectionsOnInterconnectRequest)
         -> Result<Connections, DescribeConnectionsOnInterconnectError> {
        let mut request = SignedRequest::new("POST", "directconnect", self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target",
                           "OvertureService.DescribeConnectionsOnInterconnect");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign(&try!(self.credentials_provider.credentials()));

        let response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            ::hyper::status::StatusCode::Ok => {
                Ok(serde_json::from_str::<Connections>(String::from_utf8_lossy(&response.body)
                                                           .as_ref())
                           .unwrap())
            }
            _ => Err(DescribeConnectionsOnInterconnectError::from_body(String::from_utf8_lossy(&response.body).as_ref())),
        }
    }


    #[doc="<p>Returns a list of hosted connections that have been provisioned on the given interconnect or link aggregation group (LAG).</p> <note> <p>This is intended for use by AWS Direct Connect partners only.</p> </note>"]
    fn describe_hosted_connections(&self,
                                   input: &DescribeHostedConnectionsRequest)
                                   -> Result<Connections, DescribeHostedConnectionsError> {
        let mut request = SignedRequest::new("POST", "directconnect", self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "OvertureService.DescribeHostedConnections");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign(&try!(self.credentials_provider.credentials()));

        let response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            ::hyper::status::StatusCode::Ok => {
                Ok(serde_json::from_str::<Connections>(String::from_utf8_lossy(&response.body)
                                                           .as_ref())
                           .unwrap())
            }
            _ => Err(DescribeHostedConnectionsError::from_body(String::from_utf8_lossy(&response.body).as_ref())),
        }
    }


    #[doc="<p>Deprecated in favor of <a>DescribeLoa</a>.</p> <p>Returns the LOA-CFA for an Interconnect.</p> <p>The Letter of Authorization - Connecting Facility Assignment (LOA-CFA) is a document that is used when establishing your cross connect to AWS at the colocation facility. For more information, see <a href=\"http://docs.aws.amazon.com/directconnect/latest/UserGuide/Colocation.html\">Requesting Cross Connects at AWS Direct Connect Locations</a> in the AWS Direct Connect user guide.</p>"]
    fn describe_interconnect_loa
        (&self,
         input: &DescribeInterconnectLoaRequest)
         -> Result<DescribeInterconnectLoaResponse, DescribeInterconnectLoaError> {
        let mut request = SignedRequest::new("POST", "directconnect", self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "OvertureService.DescribeInterconnectLoa");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign(&try!(self.credentials_provider.credentials()));

        let response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            ::hyper::status::StatusCode::Ok => {
                            Ok(serde_json::from_str::<DescribeInterconnectLoaResponse>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
                        }
            _ => Err(DescribeInterconnectLoaError::from_body(String::from_utf8_lossy(&response.body).as_ref())),
        }
    }


    #[doc="<p>Returns a list of interconnects owned by the AWS account.</p> <p>If an interconnect ID is provided, it will only return this particular interconnect.</p>"]
    fn describe_interconnects(&self,
                              input: &DescribeInterconnectsRequest)
                              -> Result<Interconnects, DescribeInterconnectsError> {
        let mut request = SignedRequest::new("POST", "directconnect", self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "OvertureService.DescribeInterconnects");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign(&try!(self.credentials_provider.credentials()));

        let response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            ::hyper::status::StatusCode::Ok => {
                Ok(serde_json::from_str::<Interconnects>(String::from_utf8_lossy(&response.body)
                                                             .as_ref())
                           .unwrap())
            }
            _ => {
                Err(DescribeInterconnectsError::from_body(String::from_utf8_lossy(&response.body)
                                                              .as_ref()))
            }
        }
    }


    #[doc="<p>Describes the link aggregation groups (LAGs) in your account. </p> <p>If a LAG ID is provided, only information about the specified LAG is returned.</p>"]
    fn describe_lags(&self, input: &DescribeLagsRequest) -> Result<Lags, DescribeLagsError> {
        let mut request = SignedRequest::new("POST", "directconnect", self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "OvertureService.DescribeLags");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign(&try!(self.credentials_provider.credentials()));

        let response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            ::hyper::status::StatusCode::Ok => {
                Ok(serde_json::from_str::<Lags>(String::from_utf8_lossy(&response.body).as_ref())
                       .unwrap())
            }
            _ => {
                Err(DescribeLagsError::from_body(String::from_utf8_lossy(&response.body).as_ref()))
            }
        }
    }


    #[doc="<p>Returns the LOA-CFA for a connection, interconnect, or link aggregation group (LAG).</p> <p>The Letter of Authorization - Connecting Facility Assignment (LOA-CFA) is a document that is used when establishing your cross connect to AWS at the colocation facility. For more information, see <a href=\"http://docs.aws.amazon.com/directconnect/latest/UserGuide/Colocation.html\">Requesting Cross Connects at AWS Direct Connect Locations</a> in the AWS Direct Connect user guide.</p>"]
    fn describe_loa(&self, input: &DescribeLoaRequest) -> Result<Loa, DescribeLoaError> {
        let mut request = SignedRequest::new("POST", "directconnect", self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "OvertureService.DescribeLoa");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign(&try!(self.credentials_provider.credentials()));

        let response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            ::hyper::status::StatusCode::Ok => {
                Ok(serde_json::from_str::<Loa>(String::from_utf8_lossy(&response.body).as_ref())
                       .unwrap())
            }
            _ => Err(DescribeLoaError::from_body(String::from_utf8_lossy(&response.body).as_ref())),
        }
    }


    #[doc="<p>Returns the list of AWS Direct Connect locations in the current AWS region. These are the locations that may be selected when calling CreateConnection or CreateInterconnect.</p>"]
    fn describe_locations(&self) -> Result<Locations, DescribeLocationsError> {
        let mut request = SignedRequest::new("POST", "directconnect", self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "OvertureService.DescribeLocations");
        request.set_payload(Some(b"{}".to_vec()));

        request.sign(&try!(self.credentials_provider.credentials()));

        let response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            ::hyper::status::StatusCode::Ok => {
                Ok(serde_json::from_str::<Locations>(String::from_utf8_lossy(&response.body)
                                                         .as_ref())
                           .unwrap())
            }
            _ => {
                Err(DescribeLocationsError::from_body(String::from_utf8_lossy(&response.body)
                                                          .as_ref()))
            }
        }
    }


    #[doc="<p>Describes the tags associated with the specified Direct Connect resources.</p>"]
    fn describe_tags(&self,
                     input: &DescribeTagsRequest)
                     -> Result<DescribeTagsResponse, DescribeTagsError> {
        let mut request = SignedRequest::new("POST", "directconnect", self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "OvertureService.DescribeTags");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign(&try!(self.credentials_provider.credentials()));

        let response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            ::hyper::status::StatusCode::Ok => {
                            Ok(serde_json::from_str::<DescribeTagsResponse>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
                        }
            _ => {
                Err(DescribeTagsError::from_body(String::from_utf8_lossy(&response.body).as_ref()))
            }
        }
    }


    #[doc="<p>Returns a list of virtual private gateways owned by the AWS account.</p> <p>You can create one or more AWS Direct Connect private virtual interfaces linking to a virtual private gateway. A virtual private gateway can be managed via Amazon Virtual Private Cloud (VPC) console or the <a href=\"http://docs.aws.amazon.com/AWSEC2/latest/APIReference/ApiReference-query-CreateVpnGateway.html\">EC2 CreateVpnGateway</a> action.</p>"]
    fn describe_virtual_gateways(&self) -> Result<VirtualGateways, DescribeVirtualGatewaysError> {
        let mut request = SignedRequest::new("POST", "directconnect", self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "OvertureService.DescribeVirtualGateways");
        request.set_payload(Some(b"{}".to_vec()));

        request.sign(&try!(self.credentials_provider.credentials()));

        let response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            ::hyper::status::StatusCode::Ok => {
                Ok(serde_json::from_str::<VirtualGateways>(String::from_utf8_lossy(&response.body)
                                                               .as_ref())
                           .unwrap())
            }
            _ => Err(DescribeVirtualGatewaysError::from_body(String::from_utf8_lossy(&response.body).as_ref())),
        }
    }


    #[doc="<p>Displays all virtual interfaces for an AWS account. Virtual interfaces deleted fewer than 15 minutes before you make the request are also returned. If you specify a connection ID, only the virtual interfaces associated with the connection are returned. If you specify a virtual interface ID, then only a single virtual interface is returned.</p> <p>A virtual interface (VLAN) transmits the traffic between the AWS Direct Connect location and the customer.</p>"]
    fn describe_virtual_interfaces(&self,
                                   input: &DescribeVirtualInterfacesRequest)
                                   -> Result<VirtualInterfaces, DescribeVirtualInterfacesError> {
        let mut request = SignedRequest::new("POST", "directconnect", self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "OvertureService.DescribeVirtualInterfaces");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign(&try!(self.credentials_provider.credentials()));

        let response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            ::hyper::status::StatusCode::Ok => {
                            Ok(serde_json::from_str::<VirtualInterfaces>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
                        }
            _ => Err(DescribeVirtualInterfacesError::from_body(String::from_utf8_lossy(&response.body).as_ref())),
        }
    }


    #[doc="<p>Disassociates a connection from a link aggregation group (LAG). The connection is interrupted and re-established as a standalone connection (the connection is not deleted; to delete the connection, use the <a>DeleteConnection</a> request). If the LAG has associated virtual interfaces or hosted connections, they remain associated with the LAG. A disassociated connection owned by an AWS Direct Connect partner is automatically converted to an interconnect.</p> <p>If disassociating the connection will cause the LAG to fall below its setting for minimum number of operational connections, the request fails, except when it's the last member of the LAG. If all connections are disassociated, the LAG continues to exist as an empty LAG with no physical connections. </p>"]
    fn disassociate_connection_from_lag
        (&self,
         input: &DisassociateConnectionFromLagRequest)
         -> Result<Connection, DisassociateConnectionFromLagError> {
        let mut request = SignedRequest::new("POST", "directconnect", self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target",
                           "OvertureService.DisassociateConnectionFromLag");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign(&try!(self.credentials_provider.credentials()));

        let response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            ::hyper::status::StatusCode::Ok => {
                Ok(serde_json::from_str::<Connection>(String::from_utf8_lossy(&response.body)
                                                          .as_ref())
                           .unwrap())
            }
            _ => Err(DisassociateConnectionFromLagError::from_body(String::from_utf8_lossy(&response.body).as_ref())),
        }
    }


    #[doc="<p>Adds the specified tags to the specified Direct Connect resource. Each Direct Connect resource can have a maximum of 50 tags.</p> <p>Each tag consists of a key and an optional value. If a tag with the same key is already associated with the Direct Connect resource, this action updates its value.</p>"]
    fn tag_resource(&self,
                    input: &TagResourceRequest)
                    -> Result<TagResourceResponse, TagResourceError> {
        let mut request = SignedRequest::new("POST", "directconnect", self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "OvertureService.TagResource");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign(&try!(self.credentials_provider.credentials()));

        let response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            ::hyper::status::StatusCode::Ok => {
                            Ok(serde_json::from_str::<TagResourceResponse>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
                        }
            _ => Err(TagResourceError::from_body(String::from_utf8_lossy(&response.body).as_ref())),
        }
    }


    #[doc="<p>Removes one or more tags from the specified Direct Connect resource.</p>"]
    fn untag_resource(&self,
                      input: &UntagResourceRequest)
                      -> Result<UntagResourceResponse, UntagResourceError> {
        let mut request = SignedRequest::new("POST", "directconnect", self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "OvertureService.UntagResource");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign(&try!(self.credentials_provider.credentials()));

        let response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            ::hyper::status::StatusCode::Ok => {
                            Ok(serde_json::from_str::<UntagResourceResponse>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
                        }
            _ => {
                Err(UntagResourceError::from_body(String::from_utf8_lossy(&response.body).as_ref()))
            }
        }
    }


    #[doc="<p>Updates the attributes of a link aggregation group (LAG). </p> <p>You can update the following attributes: </p> <ul> <li> <p>The name of the LAG.</p> </li> <li> <p>The value for the minimum number of connections that must be operational for the LAG itself to be operational. </p> </li> </ul> <p>When you create a LAG, the default value for the minimum number of operational connections is zero (0). If you update this value, and the number of operational connections falls below the specified value, the LAG will automatically go down to avoid overutilization of the remaining connections. Adjusting this value should be done with care as it could force the LAG down if the value is set higher than the current number of operational connections.</p>"]
    fn update_lag(&self, input: &UpdateLagRequest) -> Result<Lag, UpdateLagError> {
        let mut request = SignedRequest::new("POST", "directconnect", self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "OvertureService.UpdateLag");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign(&try!(self.credentials_provider.credentials()));

        let response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            ::hyper::status::StatusCode::Ok => {
                Ok(serde_json::from_str::<Lag>(String::from_utf8_lossy(&response.body).as_ref())
                       .unwrap())
            }
            _ => Err(UpdateLagError::from_body(String::from_utf8_lossy(&response.body).as_ref())),
        }
    }
}

#[cfg(test)]
mod protocol_tests {}
