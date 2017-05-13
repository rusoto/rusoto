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
#[doc="<p>Autonomous system (AS) number for Border Gateway Protocol (BGP) configuration.</p> <p>Example: 65000</p>"]
pub type ASN = i64;
#[doc="<p>Indicates the address family for the BGP peer.</p> <ul> <li> <p> <b>ipv4</b>: IPv4 address family</p> </li> <li> <p> <b>ipv6</b>: IPv6 address family</p> </li> </ul>"]
pub type AddressFamily = String;
#[doc="<p>Container for the parameters to the AllocateConnectionOnInterconnect operation.</p>"]
#[derive(Default,Debug,Clone,Serialize)]
            pub struct AllocateConnectionOnInterconnectRequest {
                #[doc="<p>Bandwidth of the connection.</p> <p>Example: \"<i>500Mbps</i>\"</p> <p>Default: None</p> <p>Values: 50M, 100M, 200M, 300M, 400M, or 500M</p>"]
#[serde(rename="bandwidth")]
pub bandwidth: Bandwidth,
#[doc="<p>Name of the provisioned connection.</p> <p>Example: \"<i>500M Connection to AWS</i>\"</p> <p>Default: None</p>"]
#[serde(rename="connectionName")]
pub connection_name: ConnectionName,
#[doc="<p>ID of the interconnect on which the connection will be provisioned.</p> <p>Example: dxcon-456abc78</p> <p>Default: None</p>"]
#[serde(rename="interconnectId")]
pub interconnect_id: InterconnectId,
#[doc="<p>Numeric account Id of the customer for whom the connection will be provisioned.</p> <p>Example: 123443215678</p> <p>Default: None</p>"]
#[serde(rename="ownerAccount")]
pub owner_account: OwnerAccount,
#[doc="<p>The dedicated VLAN provisioned to the connection.</p> <p>Example: 101</p> <p>Default: None</p>"]
#[serde(rename="vlan")]
pub vlan: VLAN,
            }
            
#[doc="<p>Container for the parameters to the AllocatePrivateVirtualInterface operation.</p>"]
#[derive(Default,Debug,Clone,Serialize)]
            pub struct AllocatePrivateVirtualInterfaceRequest {
                #[doc="<p>The connection ID on which the private virtual interface is provisioned.</p> <p>Default: None</p>"]
#[serde(rename="connectionId")]
pub connection_id: ConnectionId,
#[doc="<p>Detailed information for the private virtual interface to be provisioned.</p> <p>Default: None</p>"]
#[serde(rename="newPrivateVirtualInterfaceAllocation")]
pub new_private_virtual_interface_allocation: NewPrivateVirtualInterfaceAllocation,
#[doc="<p>The AWS account that will own the new private virtual interface.</p> <p>Default: None</p>"]
#[serde(rename="ownerAccount")]
pub owner_account: OwnerAccount,
            }
            
#[doc="<p>Container for the parameters to the AllocatePublicVirtualInterface operation.</p>"]
#[derive(Default,Debug,Clone,Serialize)]
            pub struct AllocatePublicVirtualInterfaceRequest {
                #[doc="<p>The connection ID on which the public virtual interface is provisioned.</p> <p>Default: None</p>"]
#[serde(rename="connectionId")]
pub connection_id: ConnectionId,
#[doc="<p>Detailed information for the public virtual interface to be provisioned.</p> <p>Default: None</p>"]
#[serde(rename="newPublicVirtualInterfaceAllocation")]
pub new_public_virtual_interface_allocation: NewPublicVirtualInterfaceAllocation,
#[doc="<p>The AWS account that will own the new public virtual interface.</p> <p>Default: None</p>"]
#[serde(rename="ownerAccount")]
pub owner_account: OwnerAccount,
            }
            
#[doc="<p>IP address assigned to the Amazon interface.</p> <p>Example: 192.168.1.1/30 or 2001:db8::1/125</p>"]
pub type AmazonAddress = String;
#[doc="<p>Authentication key for BGP configuration.</p> <p>Example: asdf34example</p>"]
pub type BGPAuthKey = String;
#[doc="<p>A structure containing information about a BGP peer.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
            pub struct BGPPeer {
                #[serde(rename="addressFamily")]
pub address_family: Option<AddressFamily>,
#[serde(rename="amazonAddress")]
pub amazon_address: Option<AmazonAddress>,
#[serde(rename="asn")]
pub asn: Option<ASN>,
#[serde(rename="authKey")]
pub auth_key: Option<BGPAuthKey>,
#[serde(rename="bgpPeerState")]
pub bgp_peer_state: Option<BGPPeerState>,
#[serde(rename="bgpStatus")]
pub bgp_status: Option<BGPStatus>,
#[serde(rename="customerAddress")]
pub customer_address: Option<CustomerAddress>,
            }
            
#[doc="<p>A list of the BGP peers configured on this virtual interface.</p>"]
pub type BGPPeerList = Vec<BGPPeer>;
#[doc="<p>The state of the BGP peer.</p> <ul> <li> <p> <b>Verifying</b>: The BGP peering addresses or ASN require validation before the BGP peer can be created. This state only applies to BGP peers on a public virtual interface. </p> </li> <li> <p> <b>Pending</b>: The BGP peer has been created, and is in this state until it is ready to be established.</p> </li> <li> <p> <b>Available</b>: The BGP peer can be established.</p> </li> <li> <p> <b>Deleting</b>: The BGP peer is in the process of being deleted.</p> </li> <li> <p> <b>Deleted</b>: The BGP peer has been deleted and cannot be established.</p> </li> </ul>"]
pub type BGPPeerState = String;
#[doc="<p>The Up/Down state of the BGP peer.</p> <ul> <li> <p> <b>Up</b>: The BGP peer is established.</p> </li> <li> <p> <b>Down</b>: The BGP peer is down.</p> </li> </ul>"]
pub type BGPStatus = String;
#[doc="<p>Bandwidth of the connection.</p> <p>Example: 1Gbps</p> <p>Default: None</p>"]
pub type Bandwidth = String;
pub type CIDR = String;
#[doc="<p>Container for the parameters to the ConfirmConnection operation.</p>"]
#[derive(Default,Debug,Clone,Serialize)]
            pub struct ConfirmConnectionRequest {
                #[serde(rename="connectionId")]
pub connection_id: ConnectionId,
            }
            
#[doc="<p>The response received when ConfirmConnection is called.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
            pub struct ConfirmConnectionResponse {
                #[serde(rename="connectionState")]
pub connection_state: Option<ConnectionState>,
            }
            
#[doc="<p>Container for the parameters to the ConfirmPrivateVirtualInterface operation.</p>"]
#[derive(Default,Debug,Clone,Serialize)]
            pub struct ConfirmPrivateVirtualInterfaceRequest {
                #[doc="<p>ID of the virtual private gateway that will be attached to the virtual interface.</p> <p> A virtual private gateway can be managed via the Amazon Virtual Private Cloud (VPC) console or the <a href=\"http://docs.aws.amazon.com/AWSEC2/latest/APIReference/ApiReference-query-CreateVpnGateway.html\">EC2 CreateVpnGateway</a> action.</p> <p>Default: None</p>"]
#[serde(rename="virtualGatewayId")]
pub virtual_gateway_id: VirtualGatewayId,
#[serde(rename="virtualInterfaceId")]
pub virtual_interface_id: VirtualInterfaceId,
            }
            
#[doc="<p>The response received when ConfirmPrivateVirtualInterface is called.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
            pub struct ConfirmPrivateVirtualInterfaceResponse {
                #[serde(rename="virtualInterfaceState")]
pub virtual_interface_state: Option<VirtualInterfaceState>,
            }
            
#[doc="<p>Container for the parameters to the ConfirmPublicVirtualInterface operation.</p>"]
#[derive(Default,Debug,Clone,Serialize)]
            pub struct ConfirmPublicVirtualInterfaceRequest {
                #[serde(rename="virtualInterfaceId")]
pub virtual_interface_id: VirtualInterfaceId,
            }
            
#[doc="<p>The response received when ConfirmPublicVirtualInterface is called.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
            pub struct ConfirmPublicVirtualInterfaceResponse {
                #[serde(rename="virtualInterfaceState")]
pub virtual_interface_state: Option<VirtualInterfaceState>,
            }
            
#[doc="<p>A connection represents the physical network connection between the AWS Direct Connect location and the customer.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
            pub struct Connection {
                #[doc="<p>Bandwidth of the connection.</p> <p>Example: 1Gbps (for regular connections), or 500Mbps (for hosted connections)</p> <p>Default: None</p>"]
#[serde(rename="bandwidth")]
pub bandwidth: Option<Bandwidth>,
#[serde(rename="connectionId")]
pub connection_id: Option<ConnectionId>,
#[serde(rename="connectionName")]
pub connection_name: Option<ConnectionName>,
#[serde(rename="connectionState")]
pub connection_state: Option<ConnectionState>,
#[doc="<p>The time of the most recent call to DescribeConnectionLoa for this Connection.</p>"]
#[serde(rename="loaIssueTime")]
pub loa_issue_time: Option<LoaIssueTime>,
#[serde(rename="location")]
pub location: Option<LocationCode>,
#[doc="<p>The AWS account that will own the new connection.</p>"]
#[serde(rename="ownerAccount")]
pub owner_account: Option<OwnerAccount>,
#[doc="<p>The name of the AWS Direct Connect service provider associated with the connection.</p>"]
#[serde(rename="partnerName")]
pub partner_name: Option<PartnerName>,
#[serde(rename="region")]
pub region: Option<Region>,
#[serde(rename="vlan")]
pub vlan: Option<VLAN>,
            }
            
#[doc="<p>ID of the connection.</p> <p>Example: dxcon-fg5678gh</p> <p>Default: None</p>"]
pub type ConnectionId = String;
#[doc="<p>A list of connections.</p>"]
pub type ConnectionList = Vec<Connection>;
#[doc="<p>The name of the connection.</p> <p>Example: \"<i>My Connection to AWS</i>\"</p> <p>Default: None</p>"]
pub type ConnectionName = String;
#[doc="<p>State of the connection.</p> <ul> <li> <p> <b>Ordering</b>: The initial state of a hosted connection provisioned on an interconnect. The connection stays in the ordering state until the owner of the hosted connection confirms or declines the connection order.</p> </li> <li> <p> <b>Requested</b>: The initial state of a standard connection. The connection stays in the requested state until the Letter of Authorization (LOA) is sent to the customer.</p> </li> <li> <p> <b>Pending</b>: The connection has been approved, and is being initialized.</p> </li> <li> <p> <b>Available</b>: The network link is up, and the connection is ready for use.</p> </li> <li> <p> <b>Down</b>: The network link is down.</p> </li> <li> <p> <b>Deleting</b>: The connection is in the process of being deleted.</p> </li> <li> <p> <b>Deleted</b>: The connection has been deleted.</p> </li> <li> <p> <b>Rejected</b>: A hosted connection in the 'Ordering' state will enter the 'Rejected' state if it is deleted by the end customer.</p> </li> </ul>"]
pub type ConnectionState = String;
#[doc="<p>A structure containing a list of connections.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
            pub struct Connections {
                #[doc="<p>A list of connections.</p>"]
#[serde(rename="connections")]
pub connections: Option<ConnectionList>,
            }
            
#[doc="<p>Container for the parameters to the CreateBGPPeer operation.</p>"]
#[derive(Default,Debug,Clone,Serialize)]
            pub struct CreateBGPPeerRequest {
                #[doc="<p>Detailed information for the BGP peer to be created.</p> <p>Default: None</p>"]
#[serde(rename="newBGPPeer")]
pub new_bgp_peer: Option<NewBGPPeer>,
#[doc="<p>The ID of the virtual interface on which the BGP peer will be provisioned.</p> <p>Example: dxvif-456abc78</p> <p>Default: None</p>"]
#[serde(rename="virtualInterfaceId")]
pub virtual_interface_id: Option<VirtualInterfaceId>,
            }
            
#[doc="<p>The response received when CreateBGPPeer is called.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
            pub struct CreateBGPPeerResponse {
                #[serde(rename="virtualInterface")]
pub virtual_interface: Option<VirtualInterface>,
            }
            
#[doc="<p>Container for the parameters to the CreateConnection operation.</p>"]
#[derive(Default,Debug,Clone,Serialize)]
            pub struct CreateConnectionRequest {
                #[serde(rename="bandwidth")]
pub bandwidth: Bandwidth,
#[serde(rename="connectionName")]
pub connection_name: ConnectionName,
#[serde(rename="location")]
pub location: LocationCode,
            }
            
#[doc="<p>Container for the parameters to the CreateInterconnect operation.</p>"]
#[derive(Default,Debug,Clone,Serialize)]
            pub struct CreateInterconnectRequest {
                #[doc="<p>The port bandwidth</p> <p>Example: 1Gbps</p> <p>Default: None</p> <p>Available values: 1Gbps,10Gbps</p>"]
#[serde(rename="bandwidth")]
pub bandwidth: Bandwidth,
#[doc="<p>The name of the interconnect.</p> <p>Example: \"<i>1G Interconnect to AWS</i>\"</p> <p>Default: None</p>"]
#[serde(rename="interconnectName")]
pub interconnect_name: InterconnectName,
#[doc="<p>Where the interconnect is located</p> <p>Example: EqSV5</p> <p>Default: None</p>"]
#[serde(rename="location")]
pub location: LocationCode,
            }
            
#[doc="<p>Container for the parameters to the CreatePrivateVirtualInterface operation.</p>"]
#[derive(Default,Debug,Clone,Serialize)]
            pub struct CreatePrivateVirtualInterfaceRequest {
                #[serde(rename="connectionId")]
pub connection_id: ConnectionId,
#[doc="<p>Detailed information for the private virtual interface to be created.</p> <p>Default: None</p>"]
#[serde(rename="newPrivateVirtualInterface")]
pub new_private_virtual_interface: NewPrivateVirtualInterface,
            }
            
#[doc="<p>Container for the parameters to the CreatePublicVirtualInterface operation.</p>"]
#[derive(Default,Debug,Clone,Serialize)]
            pub struct CreatePublicVirtualInterfaceRequest {
                #[serde(rename="connectionId")]
pub connection_id: ConnectionId,
#[doc="<p>Detailed information for the public virtual interface to be created.</p> <p>Default: None</p>"]
#[serde(rename="newPublicVirtualInterface")]
pub new_public_virtual_interface: NewPublicVirtualInterface,
            }
            
#[doc="<p>IP address assigned to the customer interface.</p> <p>Example: 192.168.1.2/30 or 2001:db8::2/125</p>"]
pub type CustomerAddress = String;
#[doc="<p>Container for the parameters to the DeleteBGPPeer operation.</p>"]
#[derive(Default,Debug,Clone,Serialize)]
            pub struct DeleteBGPPeerRequest {
                #[serde(rename="asn")]
pub asn: Option<ASN>,
#[serde(rename="customerAddress")]
pub customer_address: Option<CustomerAddress>,
#[doc="<p>The ID of the virtual interface from which the BGP peer will be deleted.</p> <p>Example: dxvif-456abc78</p> <p>Default: None</p>"]
#[serde(rename="virtualInterfaceId")]
pub virtual_interface_id: Option<VirtualInterfaceId>,
            }
            
#[doc="<p>The response received when DeleteBGPPeer is called.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
            pub struct DeleteBGPPeerResponse {
                #[serde(rename="virtualInterface")]
pub virtual_interface: Option<VirtualInterface>,
            }
            
#[doc="<p>Container for the parameters to the DeleteConnection operation.</p>"]
#[derive(Default,Debug,Clone,Serialize)]
            pub struct DeleteConnectionRequest {
                #[serde(rename="connectionId")]
pub connection_id: ConnectionId,
            }
            
#[doc="<p>Container for the parameters to the DeleteInterconnect operation.</p>"]
#[derive(Default,Debug,Clone,Serialize)]
            pub struct DeleteInterconnectRequest {
                #[serde(rename="interconnectId")]
pub interconnect_id: InterconnectId,
            }
            
#[doc="<p>The response received when DeleteInterconnect is called.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
            pub struct DeleteInterconnectResponse {
                #[serde(rename="interconnectState")]
pub interconnect_state: Option<InterconnectState>,
            }
            
#[doc="<p>Container for the parameters to the DeleteVirtualInterface operation.</p>"]
#[derive(Default,Debug,Clone,Serialize)]
            pub struct DeleteVirtualInterfaceRequest {
                #[serde(rename="virtualInterfaceId")]
pub virtual_interface_id: VirtualInterfaceId,
            }
            
#[doc="<p>The response received when DeleteVirtualInterface is called.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
            pub struct DeleteVirtualInterfaceResponse {
                #[serde(rename="virtualInterfaceState")]
pub virtual_interface_state: Option<VirtualInterfaceState>,
            }
            
#[doc="<p>Container for the parameters to the DescribeConnectionLoa operation.</p>"]
#[derive(Default,Debug,Clone,Serialize)]
            pub struct DescribeConnectionLoaRequest {
                #[serde(rename="connectionId")]
pub connection_id: ConnectionId,
#[serde(rename="loaContentType")]
pub loa_content_type: Option<LoaContentType>,
#[doc="<p>The name of the APN partner or service provider who establishes connectivity on your behalf. If you supply this parameter, the LOA-CFA lists the provider name alongside your company name as the requester of the cross connect.</p> <p>Default: None</p>"]
#[serde(rename="providerName")]
pub provider_name: Option<ProviderName>,
            }
            
#[doc="<p>The response received when DescribeConnectionLoa is called.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
            pub struct DescribeConnectionLoaResponse {
                #[serde(rename="loa")]
pub loa: Option<Loa>,
            }
            
#[doc="<p>Container for the parameters to the DescribeConnectionsOnInterconnect operation.</p>"]
#[derive(Default,Debug,Clone,Serialize)]
            pub struct DescribeConnectionsOnInterconnectRequest {
                #[doc="<p>ID of the interconnect on which a list of connection is provisioned.</p> <p>Example: dxcon-abc123</p> <p>Default: None</p>"]
#[serde(rename="interconnectId")]
pub interconnect_id: InterconnectId,
            }
            
#[doc="<p>Container for the parameters to the DescribeConnections operation.</p>"]
#[derive(Default,Debug,Clone,Serialize)]
            pub struct DescribeConnectionsRequest {
                #[serde(rename="connectionId")]
pub connection_id: Option<ConnectionId>,
            }
            
#[doc="<p>Container for the parameters to the DescribeInterconnectLoa operation.</p>"]
#[derive(Default,Debug,Clone,Serialize)]
            pub struct DescribeInterconnectLoaRequest {
                #[serde(rename="interconnectId")]
pub interconnect_id: InterconnectId,
#[serde(rename="loaContentType")]
pub loa_content_type: Option<LoaContentType>,
#[doc="<p>The name of the service provider who establishes connectivity on your behalf. If you supply this parameter, the LOA-CFA lists the provider name alongside your company name as the requester of the cross connect.</p> <p>Default: None</p>"]
#[serde(rename="providerName")]
pub provider_name: Option<ProviderName>,
            }
            
#[doc="<p>The response received when DescribeInterconnectLoa is called.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
            pub struct DescribeInterconnectLoaResponse {
                #[serde(rename="loa")]
pub loa: Option<Loa>,
            }
            
#[doc="<p>Container for the parameters to the DescribeInterconnects operation.</p>"]
#[derive(Default,Debug,Clone,Serialize)]
            pub struct DescribeInterconnectsRequest {
                #[serde(rename="interconnectId")]
pub interconnect_id: Option<InterconnectId>,
            }
            
#[doc="<p>Container for the parameters to the DescribeTags operation.</p>"]
#[derive(Default,Debug,Clone,Serialize)]
            pub struct DescribeTagsRequest {
                #[doc="<p>The Amazon Resource Names (ARNs) of the Direct Connect resources.</p>"]
#[serde(rename="resourceArns")]
pub resource_arns: ResourceArnList,
            }
            
#[doc="<p>The response received when DescribeTags is called.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
            pub struct DescribeTagsResponse {
                #[doc="<p>Information about the tags.</p>"]
#[serde(rename="resourceTags")]
pub resource_tags: Option<ResourceTagList>,
            }
            
#[doc="<p>Container for the parameters to the DescribeVirtualInterfaces operation.</p>"]
#[derive(Default,Debug,Clone,Serialize)]
            pub struct DescribeVirtualInterfacesRequest {
                #[serde(rename="connectionId")]
pub connection_id: Option<ConnectionId>,
#[serde(rename="virtualInterfaceId")]
pub virtual_interface_id: Option<VirtualInterfaceId>,
            }
            
pub type ErrorMessage = String;
#[doc="<p>An interconnect is a connection that can host other connections.</p> <p>Like a standard AWS Direct Connect connection, an interconnect represents the physical connection between an AWS Direct Connect partner's network and a specific Direct Connect location. An AWS Direct Connect partner who owns an interconnect can provision hosted connections on the interconnect for their end customers, thereby providing the end customers with connectivity to AWS services.</p> <p>The resources of the interconnect, including bandwidth and VLAN numbers, are shared by all of the hosted connections on the interconnect, and the owner of the interconnect determines how these resources are assigned.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
            pub struct Interconnect {
                #[serde(rename="bandwidth")]
pub bandwidth: Option<Bandwidth>,
#[serde(rename="interconnectId")]
pub interconnect_id: Option<InterconnectId>,
#[serde(rename="interconnectName")]
pub interconnect_name: Option<InterconnectName>,
#[serde(rename="interconnectState")]
pub interconnect_state: Option<InterconnectState>,
#[doc="<p>The time of the most recent call to DescribeInterconnectLoa for this Interconnect.</p>"]
#[serde(rename="loaIssueTime")]
pub loa_issue_time: Option<LoaIssueTime>,
#[serde(rename="location")]
pub location: Option<LocationCode>,
#[serde(rename="region")]
pub region: Option<Region>,
            }
            
#[doc="<p>The ID of the interconnect.</p> <p>Example: dxcon-abc123</p>"]
pub type InterconnectId = String;
#[doc="<p>A list of interconnects.</p>"]
pub type InterconnectList = Vec<Interconnect>;
#[doc="<p>The name of the interconnect.</p> <p>Example: \"<i>1G Interconnect to AWS</i>\"</p>"]
pub type InterconnectName = String;
#[doc="<p>State of the interconnect.</p> <ul> <li> <p> <b>Requested</b>: The initial state of an interconnect. The interconnect stays in the requested state until the Letter of Authorization (LOA) is sent to the customer.</p> </li> <li> <p> <b>Pending</b>&gt;: The interconnect has been approved, and is being initialized.</p> </li> <li> <p> <b>Available</b>: The network link is up, and the interconnect is ready for use.</p> </li> <li> <p> <b>Down</b>: The network link is down.</p> </li> <li> <p> <b>Deleting</b>: The interconnect is in the process of being deleted.</p> </li> <li> <p> <b>Deleted</b>: The interconnect has been deleted.</p> </li> </ul>"]
pub type InterconnectState = String;
#[doc="<p>A structure containing a list of interconnects.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
            pub struct Interconnects {
                #[doc="<p>A list of interconnects.</p>"]
#[serde(rename="interconnects")]
pub interconnects: Option<InterconnectList>,
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
pub loa_content: Option<LoaContent>,
#[serde(rename="loaContentType")]
pub loa_content_type: Option<LoaContentType>,
            }
            
#[doc="<p>The binary contents of the LOA-CFA document.</p>"]
pub type LoaContent = Vec<u8>;
#[doc="<p>A standard media type indicating the content type of the LOA-CFA document. Currently, the only supported value is \"application/pdf\".</p> <p>Default: application/pdf</p>"]
pub type LoaContentType = String;
pub type LoaIssueTime = f64;
#[doc="<p>An AWS Direct Connect location where connections and interconnects can be requested.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
            pub struct Location {
                #[doc="<p>The code used to indicate the AWS Direct Connect location.</p>"]
#[serde(rename="locationCode")]
pub location_code: Option<LocationCode>,
#[doc="<p>The name of the AWS Direct Connect location. The name includes the colocation partner name and the physical site of the lit building.</p>"]
#[serde(rename="locationName")]
pub location_name: Option<LocationName>,
            }
            
#[doc="<p>Where the connection is located.</p> <p>Example: EqSV5</p> <p>Default: None</p>"]
pub type LocationCode = String;
pub type LocationList = Vec<Location>;
pub type LocationName = String;
#[doc="<p>A location is a network facility where AWS Direct Connect routers are available to be connected. Generally, these are colocation hubs where many network providers have equipment, and where cross connects can be delivered. Locations include a name and facility code, and must be provided when creating a connection.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
            pub struct Locations {
                #[doc="<p>A list of colocation hubs where network providers have equipment. Most regions have multiple locations available.</p>"]
#[serde(rename="locations")]
pub locations: Option<LocationList>,
            }
            
#[doc="<p>A structure containing information about a new BGP peer.</p>"]
#[derive(Default,Debug,Clone,Serialize)]
            pub struct NewBGPPeer {
                #[serde(rename="addressFamily")]
pub address_family: Option<AddressFamily>,
#[serde(rename="amazonAddress")]
pub amazon_address: Option<AmazonAddress>,
#[serde(rename="asn")]
pub asn: Option<ASN>,
#[serde(rename="authKey")]
pub auth_key: Option<BGPAuthKey>,
#[serde(rename="customerAddress")]
pub customer_address: Option<CustomerAddress>,
            }
            
#[doc="<p>A structure containing information about a new private virtual interface.</p>"]
#[derive(Default,Debug,Clone,Serialize)]
            pub struct NewPrivateVirtualInterface {
                #[serde(rename="addressFamily")]
pub address_family: Option<AddressFamily>,
#[serde(rename="amazonAddress")]
pub amazon_address: Option<AmazonAddress>,
#[serde(rename="asn")]
pub asn: ASN,
#[serde(rename="authKey")]
pub auth_key: Option<BGPAuthKey>,
#[serde(rename="customerAddress")]
pub customer_address: Option<CustomerAddress>,
#[serde(rename="virtualGatewayId")]
pub virtual_gateway_id: VirtualGatewayId,
#[serde(rename="virtualInterfaceName")]
pub virtual_interface_name: VirtualInterfaceName,
#[serde(rename="vlan")]
pub vlan: VLAN,
            }
            
#[doc="<p>A structure containing information about a private virtual interface that will be provisioned on a connection.</p>"]
#[derive(Default,Debug,Clone,Serialize)]
            pub struct NewPrivateVirtualInterfaceAllocation {
                #[serde(rename="addressFamily")]
pub address_family: Option<AddressFamily>,
#[serde(rename="amazonAddress")]
pub amazon_address: Option<AmazonAddress>,
#[serde(rename="asn")]
pub asn: ASN,
#[serde(rename="authKey")]
pub auth_key: Option<BGPAuthKey>,
#[serde(rename="customerAddress")]
pub customer_address: Option<CustomerAddress>,
#[serde(rename="virtualInterfaceName")]
pub virtual_interface_name: VirtualInterfaceName,
#[serde(rename="vlan")]
pub vlan: VLAN,
            }
            
#[doc="<p>A structure containing information about a new public virtual interface.</p>"]
#[derive(Default,Debug,Clone,Serialize)]
            pub struct NewPublicVirtualInterface {
                #[serde(rename="addressFamily")]
pub address_family: Option<AddressFamily>,
#[serde(rename="amazonAddress")]
pub amazon_address: Option<AmazonAddress>,
#[serde(rename="asn")]
pub asn: ASN,
#[serde(rename="authKey")]
pub auth_key: Option<BGPAuthKey>,
#[serde(rename="customerAddress")]
pub customer_address: Option<CustomerAddress>,
#[serde(rename="routeFilterPrefixes")]
pub route_filter_prefixes: Option<RouteFilterPrefixList>,
#[serde(rename="virtualInterfaceName")]
pub virtual_interface_name: VirtualInterfaceName,
#[serde(rename="vlan")]
pub vlan: VLAN,
            }
            
#[doc="<p>A structure containing information about a public virtual interface that will be provisioned on a connection.</p>"]
#[derive(Default,Debug,Clone,Serialize)]
            pub struct NewPublicVirtualInterfaceAllocation {
                #[serde(rename="addressFamily")]
pub address_family: Option<AddressFamily>,
#[serde(rename="amazonAddress")]
pub amazon_address: Option<AmazonAddress>,
#[serde(rename="asn")]
pub asn: ASN,
#[serde(rename="authKey")]
pub auth_key: Option<BGPAuthKey>,
#[serde(rename="customerAddress")]
pub customer_address: Option<CustomerAddress>,
#[serde(rename="routeFilterPrefixes")]
pub route_filter_prefixes: Option<RouteFilterPrefixList>,
#[serde(rename="virtualInterfaceName")]
pub virtual_interface_name: VirtualInterfaceName,
#[serde(rename="vlan")]
pub vlan: VLAN,
            }
            
pub type OwnerAccount = String;
pub type PartnerName = String;
pub type ProviderName = String;
#[doc="<p>The AWS region where the connection is located.</p> <p>Example: us-east-1</p> <p>Default: None</p>"]
pub type Region = String;
pub type ResourceArn = String;
pub type ResourceArnList = Vec<ResourceArn>;
#[doc="<p>The tags associated with a Direct Connect resource.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
            pub struct ResourceTag {
                #[doc="<p>The Amazon Resource Name (ARN) of the Direct Connect resource.</p>"]
#[serde(rename="resourceArn")]
pub resource_arn: Option<ResourceArn>,
#[doc="<p>The tags.</p>"]
#[serde(rename="tags")]
pub tags: Option<TagList>,
            }
            
pub type ResourceTagList = Vec<ResourceTag>;
#[doc="<p>A route filter prefix that the customer can advertise through Border Gateway Protocol (BGP) over a public virtual interface.</p>"]
#[derive(Default,Debug,Clone,Serialize,Deserialize)]
            pub struct RouteFilterPrefix {
                #[doc="<p>CIDR notation for the advertised route. Multiple routes are separated by commas.</p> <p>IPv6 CIDRs must be at least a /64 or shorter</p> <p>Example: 10.10.10.0/24,10.10.11.0/24,2001:db8::/64</p>"]
#[serde(rename="cidr")]
pub cidr: Option<CIDR>,
            }
            
#[doc="<p>A list of routes to be advertised to the AWS network in this region (public virtual interface).</p>"]
pub type RouteFilterPrefixList = Vec<RouteFilterPrefix>;
pub type RouterConfig = String;
#[doc="<p>Information about a tag.</p>"]
#[derive(Default,Debug,Clone,Serialize,Deserialize)]
            pub struct Tag {
                #[doc="<p>The key of the tag.</p>"]
#[serde(rename="key")]
pub key: TagKey,
#[doc="<p>The value of the tag.</p>"]
#[serde(rename="value")]
pub value: Option<TagValue>,
            }
            
pub type TagKey = String;
pub type TagKeyList = Vec<TagKey>;
pub type TagList = Vec<Tag>;
#[doc="<p>Container for the parameters to the TagResource operation.</p>"]
#[derive(Default,Debug,Clone,Serialize)]
            pub struct TagResourceRequest {
                #[doc="<p>The Amazon Resource Name (ARN) of the Direct Connect resource.</p> <p>Example: arn:aws:directconnect:us-east-1:123456789012:dxcon/dxcon-fg5678gh</p>"]
#[serde(rename="resourceArn")]
pub resource_arn: ResourceArn,
#[doc="<p>The list of tags to add.</p>"]
#[serde(rename="tags")]
pub tags: TagList,
            }
            
#[doc="<p>The response received when TagResource is called.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
            pub struct TagResourceResponse;
            
pub type TagValue = String;
#[doc="<p>Container for the parameters to the UntagResource operation.</p>"]
#[derive(Default,Debug,Clone,Serialize)]
            pub struct UntagResourceRequest {
                #[doc="<p>The Amazon Resource Name (ARN) of the Direct Connect resource.</p>"]
#[serde(rename="resourceArn")]
pub resource_arn: ResourceArn,
#[doc="<p>The list of tag keys to remove.</p>"]
#[serde(rename="tagKeys")]
pub tag_keys: TagKeyList,
            }
            
#[doc="<p>The response received when UntagResource is called.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
            pub struct UntagResourceResponse;
            
#[doc="<p>The VLAN ID.</p> <p>Example: 101</p>"]
pub type VLAN = i64;
#[doc="<p>You can create one or more AWS Direct Connect private virtual interfaces linking to your virtual private gateway.</p> <p>Virtual private gateways can be managed using the Amazon Virtual Private Cloud (Amazon VPC) console or the <a href=\"http://docs.aws.amazon.com/AWSEC2/latest/APIReference/ApiReference-query-CreateVpnGateway.html\">Amazon EC2 CreateVpnGateway action</a>.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
            pub struct VirtualGateway {
                #[serde(rename="virtualGatewayId")]
pub virtual_gateway_id: Option<VirtualGatewayId>,
#[serde(rename="virtualGatewayState")]
pub virtual_gateway_state: Option<VirtualGatewayState>,
            }
            
#[doc="<p>The ID of the virtual private gateway to a VPC. This only applies to private virtual interfaces.</p> <p>Example: vgw-123er56</p>"]
pub type VirtualGatewayId = String;
#[doc="<p>A list of virtual private gateways.</p>"]
pub type VirtualGatewayList = Vec<VirtualGateway>;
#[doc="<p>State of the virtual private gateway.</p> <ul> <li> <p> <b>Pending</b>: This is the initial state after calling <i>CreateVpnGateway</i>.</p> </li> <li> <p> <b>Available</b>: Ready for use by a private virtual interface.</p> </li> <li> <p> <b>Deleting</b>: This is the initial state after calling <i>DeleteVpnGateway</i>.</p> </li> <li> <p> <b>Deleted</b>: In this state, a private virtual interface is unable to send traffic over this gateway.</p> </li> </ul>"]
pub type VirtualGatewayState = String;
#[doc="<p>A structure containing a list of virtual private gateways.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
            pub struct VirtualGateways {
                #[doc="<p>A list of virtual private gateways.</p>"]
#[serde(rename="virtualGateways")]
pub virtual_gateways: Option<VirtualGatewayList>,
            }
            
#[doc="<p>A virtual interface (VLAN) transmits the traffic between the AWS Direct Connect location and the customer.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
            pub struct VirtualInterface {
                #[serde(rename="addressFamily")]
pub address_family: Option<AddressFamily>,
#[serde(rename="amazonAddress")]
pub amazon_address: Option<AmazonAddress>,
#[serde(rename="asn")]
pub asn: Option<ASN>,
#[serde(rename="authKey")]
pub auth_key: Option<BGPAuthKey>,
#[serde(rename="bgpPeers")]
pub bgp_peers: Option<BGPPeerList>,
#[serde(rename="connectionId")]
pub connection_id: Option<ConnectionId>,
#[serde(rename="customerAddress")]
pub customer_address: Option<CustomerAddress>,
#[doc="<p>Information for generating the customer router configuration.</p>"]
#[serde(rename="customerRouterConfig")]
pub customer_router_config: Option<RouterConfig>,
#[serde(rename="location")]
pub location: Option<LocationCode>,
#[doc="<p>The AWS account that will own the new virtual interface.</p>"]
#[serde(rename="ownerAccount")]
pub owner_account: Option<OwnerAccount>,
#[serde(rename="routeFilterPrefixes")]
pub route_filter_prefixes: Option<RouteFilterPrefixList>,
#[serde(rename="virtualGatewayId")]
pub virtual_gateway_id: Option<VirtualGatewayId>,
#[serde(rename="virtualInterfaceId")]
pub virtual_interface_id: Option<VirtualInterfaceId>,
#[serde(rename="virtualInterfaceName")]
pub virtual_interface_name: Option<VirtualInterfaceName>,
#[serde(rename="virtualInterfaceState")]
pub virtual_interface_state: Option<VirtualInterfaceState>,
#[serde(rename="virtualInterfaceType")]
pub virtual_interface_type: Option<VirtualInterfaceType>,
#[serde(rename="vlan")]
pub vlan: Option<VLAN>,
            }
            
#[doc="<p>ID of the virtual interface.</p> <p>Example: dxvif-123dfg56</p> <p>Default: None</p>"]
pub type VirtualInterfaceId = String;
#[doc="<p>A list of virtual interfaces.</p>"]
pub type VirtualInterfaceList = Vec<VirtualInterface>;
#[doc="<p>The name of the virtual interface assigned by the customer.</p> <p>Example: \"My VPC\"</p>"]
pub type VirtualInterfaceName = String;
#[doc="<p>State of the virtual interface.</p> <ul> <li> <p> <b>Confirming</b>: The creation of the virtual interface is pending confirmation from the virtual interface owner. If the owner of the virtual interface is different from the owner of the connection on which it is provisioned, then the virtual interface will remain in this state until it is confirmed by the virtual interface owner.</p> </li> <li> <p> <b>Verifying</b>: This state only applies to public virtual interfaces. Each public virtual interface needs validation before the virtual interface can be created.</p> </li> <li> <p> <b>Pending</b>: A virtual interface is in this state from the time that it is created until the virtual interface is ready to forward traffic.</p> </li> <li> <p> <b>Available</b>: A virtual interface that is able to forward traffic.</p> </li> <li> <p> <b>Down</b>: A virtual interface that is BGP down.</p> </li> <li> <p> <b>Deleting</b>: A virtual interface is in this state immediately after calling <i>DeleteVirtualInterface</i> until it can no longer forward traffic.</p> </li> <li> <p> <b>Deleted</b>: A virtual interface that cannot forward traffic.</p> </li> <li> <p> <b>Rejected</b>: The virtual interface owner has declined creation of the virtual interface. If a virtual interface in the 'Confirming' state is deleted by the virtual interface owner, the virtual interface will enter the 'Rejected' state.</p> </li> </ul>"]
pub type VirtualInterfaceState = String;
#[doc="<p>The type of virtual interface.</p> <p>Example: private (Amazon VPC) or public (Amazon S3, Amazon DynamoDB, and so on.)</p>"]
pub type VirtualInterfaceType = String;
#[doc="<p>A structure containing a list of virtual interfaces.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
            pub struct VirtualInterfaces {
                #[doc="<p>A list of virtual interfaces.</p>"]
#[serde(rename="virtualInterfaces")]
pub virtual_interfaces: Option<VirtualInterfaceList>,
            }
            
/// Errors returned by AllocateConnectionOnInterconnect
                #[derive(Debug, PartialEq)]
                pub enum AllocateConnectionOnInterconnectError {
                    
///<p>The API was called with invalid parameters. The error message will contain additional details about the cause.</p>
DirectConnectClient(String),
///<p>A server-side error occurred during the API call. The error message will contain additional details about the cause.</p>
DirectConnectServer(String),/// An error occurred dispatching the HTTP request
HttpDispatch(HttpDispatchError),/// An error was encountered with AWS credentials.
Credentials(CredentialsError),/// A validation error occurred.  Details from AWS are provided.
Validation(String),/// An unknown error occurred.  The raw HTTP response is provided.
Unknown(String)
                }

                
                impl AllocateConnectionOnInterconnectError {
                    pub fn from_body(body: &str) -> AllocateConnectionOnInterconnectError {
                        match from_str::<SerdeJsonValue>(body) {
                            Ok(json) => {
                                let raw_error_type = json.get("__type").and_then(|e| e.as_str()).unwrap_or("Unknown");
                                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                                let error_type = pieces.last().expect("Expected error type");

                                match *error_type {
                                    "DirectConnectClientException" => AllocateConnectionOnInterconnectError::DirectConnectClient(String::from(error_message)),
"DirectConnectServerException" => AllocateConnectionOnInterconnectError::DirectConnectServer(String::from(error_message)),
"ValidationException" => AllocateConnectionOnInterconnectError::Validation(error_message.to_string()),
_ => AllocateConnectionOnInterconnectError::Unknown(String::from(body))
                                }
                            },
                            Err(_) => AllocateConnectionOnInterconnectError::Unknown(String::from(body))
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
AllocateConnectionOnInterconnectError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
AllocateConnectionOnInterconnectError::Unknown(ref cause) => cause
                        }
                    }
                 }
/// Errors returned by AllocatePrivateVirtualInterface
                #[derive(Debug, PartialEq)]
                pub enum AllocatePrivateVirtualInterfaceError {
                    
///<p>The API was called with invalid parameters. The error message will contain additional details about the cause.</p>
DirectConnectClient(String),
///<p>A server-side error occurred during the API call. The error message will contain additional details about the cause.</p>
DirectConnectServer(String),/// An error occurred dispatching the HTTP request
HttpDispatch(HttpDispatchError),/// An error was encountered with AWS credentials.
Credentials(CredentialsError),/// A validation error occurred.  Details from AWS are provided.
Validation(String),/// An unknown error occurred.  The raw HTTP response is provided.
Unknown(String)
                }

                
                impl AllocatePrivateVirtualInterfaceError {
                    pub fn from_body(body: &str) -> AllocatePrivateVirtualInterfaceError {
                        match from_str::<SerdeJsonValue>(body) {
                            Ok(json) => {
                                let raw_error_type = json.get("__type").and_then(|e| e.as_str()).unwrap_or("Unknown");
                                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                                let error_type = pieces.last().expect("Expected error type");

                                match *error_type {
                                    "DirectConnectClientException" => AllocatePrivateVirtualInterfaceError::DirectConnectClient(String::from(error_message)),
"DirectConnectServerException" => AllocatePrivateVirtualInterfaceError::DirectConnectServer(String::from(error_message)),
"ValidationException" => AllocatePrivateVirtualInterfaceError::Validation(error_message.to_string()),
_ => AllocatePrivateVirtualInterfaceError::Unknown(String::from(body))
                                }
                            },
                            Err(_) => AllocatePrivateVirtualInterfaceError::Unknown(String::from(body))
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
AllocatePrivateVirtualInterfaceError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
AllocatePrivateVirtualInterfaceError::Unknown(ref cause) => cause
                        }
                    }
                 }
/// Errors returned by AllocatePublicVirtualInterface
                #[derive(Debug, PartialEq)]
                pub enum AllocatePublicVirtualInterfaceError {
                    
///<p>The API was called with invalid parameters. The error message will contain additional details about the cause.</p>
DirectConnectClient(String),
///<p>A server-side error occurred during the API call. The error message will contain additional details about the cause.</p>
DirectConnectServer(String),/// An error occurred dispatching the HTTP request
HttpDispatch(HttpDispatchError),/// An error was encountered with AWS credentials.
Credentials(CredentialsError),/// A validation error occurred.  Details from AWS are provided.
Validation(String),/// An unknown error occurred.  The raw HTTP response is provided.
Unknown(String)
                }

                
                impl AllocatePublicVirtualInterfaceError {
                    pub fn from_body(body: &str) -> AllocatePublicVirtualInterfaceError {
                        match from_str::<SerdeJsonValue>(body) {
                            Ok(json) => {
                                let raw_error_type = json.get("__type").and_then(|e| e.as_str()).unwrap_or("Unknown");
                                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                                let error_type = pieces.last().expect("Expected error type");

                                match *error_type {
                                    "DirectConnectClientException" => AllocatePublicVirtualInterfaceError::DirectConnectClient(String::from(error_message)),
"DirectConnectServerException" => AllocatePublicVirtualInterfaceError::DirectConnectServer(String::from(error_message)),
"ValidationException" => AllocatePublicVirtualInterfaceError::Validation(error_message.to_string()),
_ => AllocatePublicVirtualInterfaceError::Unknown(String::from(body))
                                }
                            },
                            Err(_) => AllocatePublicVirtualInterfaceError::Unknown(String::from(body))
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
AllocatePublicVirtualInterfaceError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
AllocatePublicVirtualInterfaceError::Unknown(ref cause) => cause
                        }
                    }
                 }
/// Errors returned by ConfirmConnection
                #[derive(Debug, PartialEq)]
                pub enum ConfirmConnectionError {
                    
///<p>The API was called with invalid parameters. The error message will contain additional details about the cause.</p>
DirectConnectClient(String),
///<p>A server-side error occurred during the API call. The error message will contain additional details about the cause.</p>
DirectConnectServer(String),/// An error occurred dispatching the HTTP request
HttpDispatch(HttpDispatchError),/// An error was encountered with AWS credentials.
Credentials(CredentialsError),/// A validation error occurred.  Details from AWS are provided.
Validation(String),/// An unknown error occurred.  The raw HTTP response is provided.
Unknown(String)
                }

                
                impl ConfirmConnectionError {
                    pub fn from_body(body: &str) -> ConfirmConnectionError {
                        match from_str::<SerdeJsonValue>(body) {
                            Ok(json) => {
                                let raw_error_type = json.get("__type").and_then(|e| e.as_str()).unwrap_or("Unknown");
                                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                                let error_type = pieces.last().expect("Expected error type");

                                match *error_type {
                                    "DirectConnectClientException" => ConfirmConnectionError::DirectConnectClient(String::from(error_message)),
"DirectConnectServerException" => ConfirmConnectionError::DirectConnectServer(String::from(error_message)),
"ValidationException" => ConfirmConnectionError::Validation(error_message.to_string()),
_ => ConfirmConnectionError::Unknown(String::from(body))
                                }
                            },
                            Err(_) => ConfirmConnectionError::Unknown(String::from(body))
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
ConfirmConnectionError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
ConfirmConnectionError::Unknown(ref cause) => cause
                        }
                    }
                 }
/// Errors returned by ConfirmPrivateVirtualInterface
                #[derive(Debug, PartialEq)]
                pub enum ConfirmPrivateVirtualInterfaceError {
                    
///<p>The API was called with invalid parameters. The error message will contain additional details about the cause.</p>
DirectConnectClient(String),
///<p>A server-side error occurred during the API call. The error message will contain additional details about the cause.</p>
DirectConnectServer(String),/// An error occurred dispatching the HTTP request
HttpDispatch(HttpDispatchError),/// An error was encountered with AWS credentials.
Credentials(CredentialsError),/// A validation error occurred.  Details from AWS are provided.
Validation(String),/// An unknown error occurred.  The raw HTTP response is provided.
Unknown(String)
                }

                
                impl ConfirmPrivateVirtualInterfaceError {
                    pub fn from_body(body: &str) -> ConfirmPrivateVirtualInterfaceError {
                        match from_str::<SerdeJsonValue>(body) {
                            Ok(json) => {
                                let raw_error_type = json.get("__type").and_then(|e| e.as_str()).unwrap_or("Unknown");
                                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                                let error_type = pieces.last().expect("Expected error type");

                                match *error_type {
                                    "DirectConnectClientException" => ConfirmPrivateVirtualInterfaceError::DirectConnectClient(String::from(error_message)),
"DirectConnectServerException" => ConfirmPrivateVirtualInterfaceError::DirectConnectServer(String::from(error_message)),
"ValidationException" => ConfirmPrivateVirtualInterfaceError::Validation(error_message.to_string()),
_ => ConfirmPrivateVirtualInterfaceError::Unknown(String::from(body))
                                }
                            },
                            Err(_) => ConfirmPrivateVirtualInterfaceError::Unknown(String::from(body))
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
ConfirmPrivateVirtualInterfaceError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
ConfirmPrivateVirtualInterfaceError::Unknown(ref cause) => cause
                        }
                    }
                 }
/// Errors returned by ConfirmPublicVirtualInterface
                #[derive(Debug, PartialEq)]
                pub enum ConfirmPublicVirtualInterfaceError {
                    
///<p>The API was called with invalid parameters. The error message will contain additional details about the cause.</p>
DirectConnectClient(String),
///<p>A server-side error occurred during the API call. The error message will contain additional details about the cause.</p>
DirectConnectServer(String),/// An error occurred dispatching the HTTP request
HttpDispatch(HttpDispatchError),/// An error was encountered with AWS credentials.
Credentials(CredentialsError),/// A validation error occurred.  Details from AWS are provided.
Validation(String),/// An unknown error occurred.  The raw HTTP response is provided.
Unknown(String)
                }

                
                impl ConfirmPublicVirtualInterfaceError {
                    pub fn from_body(body: &str) -> ConfirmPublicVirtualInterfaceError {
                        match from_str::<SerdeJsonValue>(body) {
                            Ok(json) => {
                                let raw_error_type = json.get("__type").and_then(|e| e.as_str()).unwrap_or("Unknown");
                                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                                let error_type = pieces.last().expect("Expected error type");

                                match *error_type {
                                    "DirectConnectClientException" => ConfirmPublicVirtualInterfaceError::DirectConnectClient(String::from(error_message)),
"DirectConnectServerException" => ConfirmPublicVirtualInterfaceError::DirectConnectServer(String::from(error_message)),
"ValidationException" => ConfirmPublicVirtualInterfaceError::Validation(error_message.to_string()),
_ => ConfirmPublicVirtualInterfaceError::Unknown(String::from(body))
                                }
                            },
                            Err(_) => ConfirmPublicVirtualInterfaceError::Unknown(String::from(body))
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
ConfirmPublicVirtualInterfaceError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
ConfirmPublicVirtualInterfaceError::Unknown(ref cause) => cause
                        }
                    }
                 }
/// Errors returned by CreateBGPPeer
                #[derive(Debug, PartialEq)]
                pub enum CreateBGPPeerError {
                    
///<p>The API was called with invalid parameters. The error message will contain additional details about the cause.</p>
DirectConnectClient(String),
///<p>A server-side error occurred during the API call. The error message will contain additional details about the cause.</p>
DirectConnectServer(String),/// An error occurred dispatching the HTTP request
HttpDispatch(HttpDispatchError),/// An error was encountered with AWS credentials.
Credentials(CredentialsError),/// A validation error occurred.  Details from AWS are provided.
Validation(String),/// An unknown error occurred.  The raw HTTP response is provided.
Unknown(String)
                }

                
                impl CreateBGPPeerError {
                    pub fn from_body(body: &str) -> CreateBGPPeerError {
                        match from_str::<SerdeJsonValue>(body) {
                            Ok(json) => {
                                let raw_error_type = json.get("__type").and_then(|e| e.as_str()).unwrap_or("Unknown");
                                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                                let error_type = pieces.last().expect("Expected error type");

                                match *error_type {
                                    "DirectConnectClientException" => CreateBGPPeerError::DirectConnectClient(String::from(error_message)),
"DirectConnectServerException" => CreateBGPPeerError::DirectConnectServer(String::from(error_message)),
"ValidationException" => CreateBGPPeerError::Validation(error_message.to_string()),
_ => CreateBGPPeerError::Unknown(String::from(body))
                                }
                            },
                            Err(_) => CreateBGPPeerError::Unknown(String::from(body))
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
CreateBGPPeerError::Unknown(ref cause) => cause
                        }
                    }
                 }
/// Errors returned by CreateConnection
                #[derive(Debug, PartialEq)]
                pub enum CreateConnectionError {
                    
///<p>The API was called with invalid parameters. The error message will contain additional details about the cause.</p>
DirectConnectClient(String),
///<p>A server-side error occurred during the API call. The error message will contain additional details about the cause.</p>
DirectConnectServer(String),/// An error occurred dispatching the HTTP request
HttpDispatch(HttpDispatchError),/// An error was encountered with AWS credentials.
Credentials(CredentialsError),/// A validation error occurred.  Details from AWS are provided.
Validation(String),/// An unknown error occurred.  The raw HTTP response is provided.
Unknown(String)
                }

                
                impl CreateConnectionError {
                    pub fn from_body(body: &str) -> CreateConnectionError {
                        match from_str::<SerdeJsonValue>(body) {
                            Ok(json) => {
                                let raw_error_type = json.get("__type").and_then(|e| e.as_str()).unwrap_or("Unknown");
                                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                                let error_type = pieces.last().expect("Expected error type");

                                match *error_type {
                                    "DirectConnectClientException" => CreateConnectionError::DirectConnectClient(String::from(error_message)),
"DirectConnectServerException" => CreateConnectionError::DirectConnectServer(String::from(error_message)),
"ValidationException" => CreateConnectionError::Validation(error_message.to_string()),
_ => CreateConnectionError::Unknown(String::from(body))
                                }
                            },
                            Err(_) => CreateConnectionError::Unknown(String::from(body))
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
CreateConnectionError::Unknown(ref cause) => cause
                        }
                    }
                 }
/// Errors returned by CreateInterconnect
                #[derive(Debug, PartialEq)]
                pub enum CreateInterconnectError {
                    
///<p>The API was called with invalid parameters. The error message will contain additional details about the cause.</p>
DirectConnectClient(String),
///<p>A server-side error occurred during the API call. The error message will contain additional details about the cause.</p>
DirectConnectServer(String),/// An error occurred dispatching the HTTP request
HttpDispatch(HttpDispatchError),/// An error was encountered with AWS credentials.
Credentials(CredentialsError),/// A validation error occurred.  Details from AWS are provided.
Validation(String),/// An unknown error occurred.  The raw HTTP response is provided.
Unknown(String)
                }

                
                impl CreateInterconnectError {
                    pub fn from_body(body: &str) -> CreateInterconnectError {
                        match from_str::<SerdeJsonValue>(body) {
                            Ok(json) => {
                                let raw_error_type = json.get("__type").and_then(|e| e.as_str()).unwrap_or("Unknown");
                                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                                let error_type = pieces.last().expect("Expected error type");

                                match *error_type {
                                    "DirectConnectClientException" => CreateInterconnectError::DirectConnectClient(String::from(error_message)),
"DirectConnectServerException" => CreateInterconnectError::DirectConnectServer(String::from(error_message)),
"ValidationException" => CreateInterconnectError::Validation(error_message.to_string()),
_ => CreateInterconnectError::Unknown(String::from(body))
                                }
                            },
                            Err(_) => CreateInterconnectError::Unknown(String::from(body))
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
CreateInterconnectError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
CreateInterconnectError::Unknown(ref cause) => cause
                        }
                    }
                 }
/// Errors returned by CreatePrivateVirtualInterface
                #[derive(Debug, PartialEq)]
                pub enum CreatePrivateVirtualInterfaceError {
                    
///<p>The API was called with invalid parameters. The error message will contain additional details about the cause.</p>
DirectConnectClient(String),
///<p>A server-side error occurred during the API call. The error message will contain additional details about the cause.</p>
DirectConnectServer(String),/// An error occurred dispatching the HTTP request
HttpDispatch(HttpDispatchError),/// An error was encountered with AWS credentials.
Credentials(CredentialsError),/// A validation error occurred.  Details from AWS are provided.
Validation(String),/// An unknown error occurred.  The raw HTTP response is provided.
Unknown(String)
                }

                
                impl CreatePrivateVirtualInterfaceError {
                    pub fn from_body(body: &str) -> CreatePrivateVirtualInterfaceError {
                        match from_str::<SerdeJsonValue>(body) {
                            Ok(json) => {
                                let raw_error_type = json.get("__type").and_then(|e| e.as_str()).unwrap_or("Unknown");
                                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                                let error_type = pieces.last().expect("Expected error type");

                                match *error_type {
                                    "DirectConnectClientException" => CreatePrivateVirtualInterfaceError::DirectConnectClient(String::from(error_message)),
"DirectConnectServerException" => CreatePrivateVirtualInterfaceError::DirectConnectServer(String::from(error_message)),
"ValidationException" => CreatePrivateVirtualInterfaceError::Validation(error_message.to_string()),
_ => CreatePrivateVirtualInterfaceError::Unknown(String::from(body))
                                }
                            },
                            Err(_) => CreatePrivateVirtualInterfaceError::Unknown(String::from(body))
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
CreatePrivateVirtualInterfaceError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
CreatePrivateVirtualInterfaceError::Unknown(ref cause) => cause
                        }
                    }
                 }
/// Errors returned by CreatePublicVirtualInterface
                #[derive(Debug, PartialEq)]
                pub enum CreatePublicVirtualInterfaceError {
                    
///<p>The API was called with invalid parameters. The error message will contain additional details about the cause.</p>
DirectConnectClient(String),
///<p>A server-side error occurred during the API call. The error message will contain additional details about the cause.</p>
DirectConnectServer(String),/// An error occurred dispatching the HTTP request
HttpDispatch(HttpDispatchError),/// An error was encountered with AWS credentials.
Credentials(CredentialsError),/// A validation error occurred.  Details from AWS are provided.
Validation(String),/// An unknown error occurred.  The raw HTTP response is provided.
Unknown(String)
                }

                
                impl CreatePublicVirtualInterfaceError {
                    pub fn from_body(body: &str) -> CreatePublicVirtualInterfaceError {
                        match from_str::<SerdeJsonValue>(body) {
                            Ok(json) => {
                                let raw_error_type = json.get("__type").and_then(|e| e.as_str()).unwrap_or("Unknown");
                                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                                let error_type = pieces.last().expect("Expected error type");

                                match *error_type {
                                    "DirectConnectClientException" => CreatePublicVirtualInterfaceError::DirectConnectClient(String::from(error_message)),
"DirectConnectServerException" => CreatePublicVirtualInterfaceError::DirectConnectServer(String::from(error_message)),
"ValidationException" => CreatePublicVirtualInterfaceError::Validation(error_message.to_string()),
_ => CreatePublicVirtualInterfaceError::Unknown(String::from(body))
                                }
                            },
                            Err(_) => CreatePublicVirtualInterfaceError::Unknown(String::from(body))
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
CreatePublicVirtualInterfaceError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
CreatePublicVirtualInterfaceError::Unknown(ref cause) => cause
                        }
                    }
                 }
/// Errors returned by DeleteBGPPeer
                #[derive(Debug, PartialEq)]
                pub enum DeleteBGPPeerError {
                    
///<p>The API was called with invalid parameters. The error message will contain additional details about the cause.</p>
DirectConnectClient(String),
///<p>A server-side error occurred during the API call. The error message will contain additional details about the cause.</p>
DirectConnectServer(String),/// An error occurred dispatching the HTTP request
HttpDispatch(HttpDispatchError),/// An error was encountered with AWS credentials.
Credentials(CredentialsError),/// A validation error occurred.  Details from AWS are provided.
Validation(String),/// An unknown error occurred.  The raw HTTP response is provided.
Unknown(String)
                }

                
                impl DeleteBGPPeerError {
                    pub fn from_body(body: &str) -> DeleteBGPPeerError {
                        match from_str::<SerdeJsonValue>(body) {
                            Ok(json) => {
                                let raw_error_type = json.get("__type").and_then(|e| e.as_str()).unwrap_or("Unknown");
                                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                                let error_type = pieces.last().expect("Expected error type");

                                match *error_type {
                                    "DirectConnectClientException" => DeleteBGPPeerError::DirectConnectClient(String::from(error_message)),
"DirectConnectServerException" => DeleteBGPPeerError::DirectConnectServer(String::from(error_message)),
"ValidationException" => DeleteBGPPeerError::Validation(error_message.to_string()),
_ => DeleteBGPPeerError::Unknown(String::from(body))
                                }
                            },
                            Err(_) => DeleteBGPPeerError::Unknown(String::from(body))
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
DeleteBGPPeerError::Unknown(ref cause) => cause
                        }
                    }
                 }
/// Errors returned by DeleteConnection
                #[derive(Debug, PartialEq)]
                pub enum DeleteConnectionError {
                    
///<p>The API was called with invalid parameters. The error message will contain additional details about the cause.</p>
DirectConnectClient(String),
///<p>A server-side error occurred during the API call. The error message will contain additional details about the cause.</p>
DirectConnectServer(String),/// An error occurred dispatching the HTTP request
HttpDispatch(HttpDispatchError),/// An error was encountered with AWS credentials.
Credentials(CredentialsError),/// A validation error occurred.  Details from AWS are provided.
Validation(String),/// An unknown error occurred.  The raw HTTP response is provided.
Unknown(String)
                }

                
                impl DeleteConnectionError {
                    pub fn from_body(body: &str) -> DeleteConnectionError {
                        match from_str::<SerdeJsonValue>(body) {
                            Ok(json) => {
                                let raw_error_type = json.get("__type").and_then(|e| e.as_str()).unwrap_or("Unknown");
                                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                                let error_type = pieces.last().expect("Expected error type");

                                match *error_type {
                                    "DirectConnectClientException" => DeleteConnectionError::DirectConnectClient(String::from(error_message)),
"DirectConnectServerException" => DeleteConnectionError::DirectConnectServer(String::from(error_message)),
"ValidationException" => DeleteConnectionError::Validation(error_message.to_string()),
_ => DeleteConnectionError::Unknown(String::from(body))
                                }
                            },
                            Err(_) => DeleteConnectionError::Unknown(String::from(body))
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
DeleteConnectionError::Unknown(ref cause) => cause
                        }
                    }
                 }
/// Errors returned by DeleteInterconnect
                #[derive(Debug, PartialEq)]
                pub enum DeleteInterconnectError {
                    
///<p>The API was called with invalid parameters. The error message will contain additional details about the cause.</p>
DirectConnectClient(String),
///<p>A server-side error occurred during the API call. The error message will contain additional details about the cause.</p>
DirectConnectServer(String),/// An error occurred dispatching the HTTP request
HttpDispatch(HttpDispatchError),/// An error was encountered with AWS credentials.
Credentials(CredentialsError),/// A validation error occurred.  Details from AWS are provided.
Validation(String),/// An unknown error occurred.  The raw HTTP response is provided.
Unknown(String)
                }

                
                impl DeleteInterconnectError {
                    pub fn from_body(body: &str) -> DeleteInterconnectError {
                        match from_str::<SerdeJsonValue>(body) {
                            Ok(json) => {
                                let raw_error_type = json.get("__type").and_then(|e| e.as_str()).unwrap_or("Unknown");
                                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                                let error_type = pieces.last().expect("Expected error type");

                                match *error_type {
                                    "DirectConnectClientException" => DeleteInterconnectError::DirectConnectClient(String::from(error_message)),
"DirectConnectServerException" => DeleteInterconnectError::DirectConnectServer(String::from(error_message)),
"ValidationException" => DeleteInterconnectError::Validation(error_message.to_string()),
_ => DeleteInterconnectError::Unknown(String::from(body))
                                }
                            },
                            Err(_) => DeleteInterconnectError::Unknown(String::from(body))
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
DeleteInterconnectError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
DeleteInterconnectError::Unknown(ref cause) => cause
                        }
                    }
                 }
/// Errors returned by DeleteVirtualInterface
                #[derive(Debug, PartialEq)]
                pub enum DeleteVirtualInterfaceError {
                    
///<p>The API was called with invalid parameters. The error message will contain additional details about the cause.</p>
DirectConnectClient(String),
///<p>A server-side error occurred during the API call. The error message will contain additional details about the cause.</p>
DirectConnectServer(String),/// An error occurred dispatching the HTTP request
HttpDispatch(HttpDispatchError),/// An error was encountered with AWS credentials.
Credentials(CredentialsError),/// A validation error occurred.  Details from AWS are provided.
Validation(String),/// An unknown error occurred.  The raw HTTP response is provided.
Unknown(String)
                }

                
                impl DeleteVirtualInterfaceError {
                    pub fn from_body(body: &str) -> DeleteVirtualInterfaceError {
                        match from_str::<SerdeJsonValue>(body) {
                            Ok(json) => {
                                let raw_error_type = json.get("__type").and_then(|e| e.as_str()).unwrap_or("Unknown");
                                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                                let error_type = pieces.last().expect("Expected error type");

                                match *error_type {
                                    "DirectConnectClientException" => DeleteVirtualInterfaceError::DirectConnectClient(String::from(error_message)),
"DirectConnectServerException" => DeleteVirtualInterfaceError::DirectConnectServer(String::from(error_message)),
"ValidationException" => DeleteVirtualInterfaceError::Validation(error_message.to_string()),
_ => DeleteVirtualInterfaceError::Unknown(String::from(body))
                                }
                            },
                            Err(_) => DeleteVirtualInterfaceError::Unknown(String::from(body))
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
DeleteVirtualInterfaceError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
DeleteVirtualInterfaceError::Unknown(ref cause) => cause
                        }
                    }
                 }
/// Errors returned by DescribeConnectionLoa
                #[derive(Debug, PartialEq)]
                pub enum DescribeConnectionLoaError {
                    
///<p>The API was called with invalid parameters. The error message will contain additional details about the cause.</p>
DirectConnectClient(String),
///<p>A server-side error occurred during the API call. The error message will contain additional details about the cause.</p>
DirectConnectServer(String),/// An error occurred dispatching the HTTP request
HttpDispatch(HttpDispatchError),/// An error was encountered with AWS credentials.
Credentials(CredentialsError),/// A validation error occurred.  Details from AWS are provided.
Validation(String),/// An unknown error occurred.  The raw HTTP response is provided.
Unknown(String)
                }

                
                impl DescribeConnectionLoaError {
                    pub fn from_body(body: &str) -> DescribeConnectionLoaError {
                        match from_str::<SerdeJsonValue>(body) {
                            Ok(json) => {
                                let raw_error_type = json.get("__type").and_then(|e| e.as_str()).unwrap_or("Unknown");
                                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                                let error_type = pieces.last().expect("Expected error type");

                                match *error_type {
                                    "DirectConnectClientException" => DescribeConnectionLoaError::DirectConnectClient(String::from(error_message)),
"DirectConnectServerException" => DescribeConnectionLoaError::DirectConnectServer(String::from(error_message)),
"ValidationException" => DescribeConnectionLoaError::Validation(error_message.to_string()),
_ => DescribeConnectionLoaError::Unknown(String::from(body))
                                }
                            },
                            Err(_) => DescribeConnectionLoaError::Unknown(String::from(body))
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
DescribeConnectionLoaError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
DescribeConnectionLoaError::Unknown(ref cause) => cause
                        }
                    }
                 }
/// Errors returned by DescribeConnections
                #[derive(Debug, PartialEq)]
                pub enum DescribeConnectionsError {
                    
///<p>The API was called with invalid parameters. The error message will contain additional details about the cause.</p>
DirectConnectClient(String),
///<p>A server-side error occurred during the API call. The error message will contain additional details about the cause.</p>
DirectConnectServer(String),/// An error occurred dispatching the HTTP request
HttpDispatch(HttpDispatchError),/// An error was encountered with AWS credentials.
Credentials(CredentialsError),/// A validation error occurred.  Details from AWS are provided.
Validation(String),/// An unknown error occurred.  The raw HTTP response is provided.
Unknown(String)
                }

                
                impl DescribeConnectionsError {
                    pub fn from_body(body: &str) -> DescribeConnectionsError {
                        match from_str::<SerdeJsonValue>(body) {
                            Ok(json) => {
                                let raw_error_type = json.get("__type").and_then(|e| e.as_str()).unwrap_or("Unknown");
                                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                                let error_type = pieces.last().expect("Expected error type");

                                match *error_type {
                                    "DirectConnectClientException" => DescribeConnectionsError::DirectConnectClient(String::from(error_message)),
"DirectConnectServerException" => DescribeConnectionsError::DirectConnectServer(String::from(error_message)),
"ValidationException" => DescribeConnectionsError::Validation(error_message.to_string()),
_ => DescribeConnectionsError::Unknown(String::from(body))
                                }
                            },
                            Err(_) => DescribeConnectionsError::Unknown(String::from(body))
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
DescribeConnectionsError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
DescribeConnectionsError::Unknown(ref cause) => cause
                        }
                    }
                 }
/// Errors returned by DescribeConnectionsOnInterconnect
                #[derive(Debug, PartialEq)]
                pub enum DescribeConnectionsOnInterconnectError {
                    
///<p>The API was called with invalid parameters. The error message will contain additional details about the cause.</p>
DirectConnectClient(String),
///<p>A server-side error occurred during the API call. The error message will contain additional details about the cause.</p>
DirectConnectServer(String),/// An error occurred dispatching the HTTP request
HttpDispatch(HttpDispatchError),/// An error was encountered with AWS credentials.
Credentials(CredentialsError),/// A validation error occurred.  Details from AWS are provided.
Validation(String),/// An unknown error occurred.  The raw HTTP response is provided.
Unknown(String)
                }

                
                impl DescribeConnectionsOnInterconnectError {
                    pub fn from_body(body: &str) -> DescribeConnectionsOnInterconnectError {
                        match from_str::<SerdeJsonValue>(body) {
                            Ok(json) => {
                                let raw_error_type = json.get("__type").and_then(|e| e.as_str()).unwrap_or("Unknown");
                                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                                let error_type = pieces.last().expect("Expected error type");

                                match *error_type {
                                    "DirectConnectClientException" => DescribeConnectionsOnInterconnectError::DirectConnectClient(String::from(error_message)),
"DirectConnectServerException" => DescribeConnectionsOnInterconnectError::DirectConnectServer(String::from(error_message)),
"ValidationException" => DescribeConnectionsOnInterconnectError::Validation(error_message.to_string()),
_ => DescribeConnectionsOnInterconnectError::Unknown(String::from(body))
                                }
                            },
                            Err(_) => DescribeConnectionsOnInterconnectError::Unknown(String::from(body))
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
DescribeConnectionsOnInterconnectError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
DescribeConnectionsOnInterconnectError::Unknown(ref cause) => cause
                        }
                    }
                 }
/// Errors returned by DescribeInterconnectLoa
                #[derive(Debug, PartialEq)]
                pub enum DescribeInterconnectLoaError {
                    
///<p>The API was called with invalid parameters. The error message will contain additional details about the cause.</p>
DirectConnectClient(String),
///<p>A server-side error occurred during the API call. The error message will contain additional details about the cause.</p>
DirectConnectServer(String),/// An error occurred dispatching the HTTP request
HttpDispatch(HttpDispatchError),/// An error was encountered with AWS credentials.
Credentials(CredentialsError),/// A validation error occurred.  Details from AWS are provided.
Validation(String),/// An unknown error occurred.  The raw HTTP response is provided.
Unknown(String)
                }

                
                impl DescribeInterconnectLoaError {
                    pub fn from_body(body: &str) -> DescribeInterconnectLoaError {
                        match from_str::<SerdeJsonValue>(body) {
                            Ok(json) => {
                                let raw_error_type = json.get("__type").and_then(|e| e.as_str()).unwrap_or("Unknown");
                                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                                let error_type = pieces.last().expect("Expected error type");

                                match *error_type {
                                    "DirectConnectClientException" => DescribeInterconnectLoaError::DirectConnectClient(String::from(error_message)),
"DirectConnectServerException" => DescribeInterconnectLoaError::DirectConnectServer(String::from(error_message)),
"ValidationException" => DescribeInterconnectLoaError::Validation(error_message.to_string()),
_ => DescribeInterconnectLoaError::Unknown(String::from(body))
                                }
                            },
                            Err(_) => DescribeInterconnectLoaError::Unknown(String::from(body))
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
DescribeInterconnectLoaError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
DescribeInterconnectLoaError::Unknown(ref cause) => cause
                        }
                    }
                 }
/// Errors returned by DescribeInterconnects
                #[derive(Debug, PartialEq)]
                pub enum DescribeInterconnectsError {
                    
///<p>The API was called with invalid parameters. The error message will contain additional details about the cause.</p>
DirectConnectClient(String),
///<p>A server-side error occurred during the API call. The error message will contain additional details about the cause.</p>
DirectConnectServer(String),/// An error occurred dispatching the HTTP request
HttpDispatch(HttpDispatchError),/// An error was encountered with AWS credentials.
Credentials(CredentialsError),/// A validation error occurred.  Details from AWS are provided.
Validation(String),/// An unknown error occurred.  The raw HTTP response is provided.
Unknown(String)
                }

                
                impl DescribeInterconnectsError {
                    pub fn from_body(body: &str) -> DescribeInterconnectsError {
                        match from_str::<SerdeJsonValue>(body) {
                            Ok(json) => {
                                let raw_error_type = json.get("__type").and_then(|e| e.as_str()).unwrap_or("Unknown");
                                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                                let error_type = pieces.last().expect("Expected error type");

                                match *error_type {
                                    "DirectConnectClientException" => DescribeInterconnectsError::DirectConnectClient(String::from(error_message)),
"DirectConnectServerException" => DescribeInterconnectsError::DirectConnectServer(String::from(error_message)),
"ValidationException" => DescribeInterconnectsError::Validation(error_message.to_string()),
_ => DescribeInterconnectsError::Unknown(String::from(body))
                                }
                            },
                            Err(_) => DescribeInterconnectsError::Unknown(String::from(body))
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
DescribeInterconnectsError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
DescribeInterconnectsError::Unknown(ref cause) => cause
                        }
                    }
                 }
/// Errors returned by DescribeLocations
                #[derive(Debug, PartialEq)]
                pub enum DescribeLocationsError {
                    
///<p>The API was called with invalid parameters. The error message will contain additional details about the cause.</p>
DirectConnectClient(String),
///<p>A server-side error occurred during the API call. The error message will contain additional details about the cause.</p>
DirectConnectServer(String),/// An error occurred dispatching the HTTP request
HttpDispatch(HttpDispatchError),/// An error was encountered with AWS credentials.
Credentials(CredentialsError),/// A validation error occurred.  Details from AWS are provided.
Validation(String),/// An unknown error occurred.  The raw HTTP response is provided.
Unknown(String)
                }

                
                impl DescribeLocationsError {
                    pub fn from_body(body: &str) -> DescribeLocationsError {
                        match from_str::<SerdeJsonValue>(body) {
                            Ok(json) => {
                                let raw_error_type = json.get("__type").and_then(|e| e.as_str()).unwrap_or("Unknown");
                                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                                let error_type = pieces.last().expect("Expected error type");

                                match *error_type {
                                    "DirectConnectClientException" => DescribeLocationsError::DirectConnectClient(String::from(error_message)),
"DirectConnectServerException" => DescribeLocationsError::DirectConnectServer(String::from(error_message)),
"ValidationException" => DescribeLocationsError::Validation(error_message.to_string()),
_ => DescribeLocationsError::Unknown(String::from(body))
                                }
                            },
                            Err(_) => DescribeLocationsError::Unknown(String::from(body))
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
DescribeLocationsError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
DescribeLocationsError::Unknown(ref cause) => cause
                        }
                    }
                 }
/// Errors returned by DescribeTags
                #[derive(Debug, PartialEq)]
                pub enum DescribeTagsError {
                    
///<p>The API was called with invalid parameters. The error message will contain additional details about the cause.</p>
DirectConnectClient(String),
///<p>A server-side error occurred during the API call. The error message will contain additional details about the cause.</p>
DirectConnectServer(String),/// An error occurred dispatching the HTTP request
HttpDispatch(HttpDispatchError),/// An error was encountered with AWS credentials.
Credentials(CredentialsError),/// A validation error occurred.  Details from AWS are provided.
Validation(String),/// An unknown error occurred.  The raw HTTP response is provided.
Unknown(String)
                }

                
                impl DescribeTagsError {
                    pub fn from_body(body: &str) -> DescribeTagsError {
                        match from_str::<SerdeJsonValue>(body) {
                            Ok(json) => {
                                let raw_error_type = json.get("__type").and_then(|e| e.as_str()).unwrap_or("Unknown");
                                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                                let error_type = pieces.last().expect("Expected error type");

                                match *error_type {
                                    "DirectConnectClientException" => DescribeTagsError::DirectConnectClient(String::from(error_message)),
"DirectConnectServerException" => DescribeTagsError::DirectConnectServer(String::from(error_message)),
"ValidationException" => DescribeTagsError::Validation(error_message.to_string()),
_ => DescribeTagsError::Unknown(String::from(body))
                                }
                            },
                            Err(_) => DescribeTagsError::Unknown(String::from(body))
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
DescribeTagsError::Unknown(ref cause) => cause
                        }
                    }
                 }
/// Errors returned by DescribeVirtualGateways
                #[derive(Debug, PartialEq)]
                pub enum DescribeVirtualGatewaysError {
                    
///<p>The API was called with invalid parameters. The error message will contain additional details about the cause.</p>
DirectConnectClient(String),
///<p>A server-side error occurred during the API call. The error message will contain additional details about the cause.</p>
DirectConnectServer(String),/// An error occurred dispatching the HTTP request
HttpDispatch(HttpDispatchError),/// An error was encountered with AWS credentials.
Credentials(CredentialsError),/// A validation error occurred.  Details from AWS are provided.
Validation(String),/// An unknown error occurred.  The raw HTTP response is provided.
Unknown(String)
                }

                
                impl DescribeVirtualGatewaysError {
                    pub fn from_body(body: &str) -> DescribeVirtualGatewaysError {
                        match from_str::<SerdeJsonValue>(body) {
                            Ok(json) => {
                                let raw_error_type = json.get("__type").and_then(|e| e.as_str()).unwrap_or("Unknown");
                                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                                let error_type = pieces.last().expect("Expected error type");

                                match *error_type {
                                    "DirectConnectClientException" => DescribeVirtualGatewaysError::DirectConnectClient(String::from(error_message)),
"DirectConnectServerException" => DescribeVirtualGatewaysError::DirectConnectServer(String::from(error_message)),
"ValidationException" => DescribeVirtualGatewaysError::Validation(error_message.to_string()),
_ => DescribeVirtualGatewaysError::Unknown(String::from(body))
                                }
                            },
                            Err(_) => DescribeVirtualGatewaysError::Unknown(String::from(body))
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
DescribeVirtualGatewaysError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
DescribeVirtualGatewaysError::Unknown(ref cause) => cause
                        }
                    }
                 }
/// Errors returned by DescribeVirtualInterfaces
                #[derive(Debug, PartialEq)]
                pub enum DescribeVirtualInterfacesError {
                    
///<p>The API was called with invalid parameters. The error message will contain additional details about the cause.</p>
DirectConnectClient(String),
///<p>A server-side error occurred during the API call. The error message will contain additional details about the cause.</p>
DirectConnectServer(String),/// An error occurred dispatching the HTTP request
HttpDispatch(HttpDispatchError),/// An error was encountered with AWS credentials.
Credentials(CredentialsError),/// A validation error occurred.  Details from AWS are provided.
Validation(String),/// An unknown error occurred.  The raw HTTP response is provided.
Unknown(String)
                }

                
                impl DescribeVirtualInterfacesError {
                    pub fn from_body(body: &str) -> DescribeVirtualInterfacesError {
                        match from_str::<SerdeJsonValue>(body) {
                            Ok(json) => {
                                let raw_error_type = json.get("__type").and_then(|e| e.as_str()).unwrap_or("Unknown");
                                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                                let error_type = pieces.last().expect("Expected error type");

                                match *error_type {
                                    "DirectConnectClientException" => DescribeVirtualInterfacesError::DirectConnectClient(String::from(error_message)),
"DirectConnectServerException" => DescribeVirtualInterfacesError::DirectConnectServer(String::from(error_message)),
"ValidationException" => DescribeVirtualInterfacesError::Validation(error_message.to_string()),
_ => DescribeVirtualInterfacesError::Unknown(String::from(body))
                                }
                            },
                            Err(_) => DescribeVirtualInterfacesError::Unknown(String::from(body))
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
DescribeVirtualInterfacesError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
DescribeVirtualInterfacesError::Unknown(ref cause) => cause
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
TooManyTags(String),/// An error occurred dispatching the HTTP request
HttpDispatch(HttpDispatchError),/// An error was encountered with AWS credentials.
Credentials(CredentialsError),/// A validation error occurred.  Details from AWS are provided.
Validation(String),/// An unknown error occurred.  The raw HTTP response is provided.
Unknown(String)
                }

                
                impl TagResourceError {
                    pub fn from_body(body: &str) -> TagResourceError {
                        match from_str::<SerdeJsonValue>(body) {
                            Ok(json) => {
                                let raw_error_type = json.get("__type").and_then(|e| e.as_str()).unwrap_or("Unknown");
                                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                                let error_type = pieces.last().expect("Expected error type");

                                match *error_type {
                                    "DirectConnectClientException" => TagResourceError::DirectConnectClient(String::from(error_message)),
"DirectConnectServerException" => TagResourceError::DirectConnectServer(String::from(error_message)),
"DuplicateTagKeysException" => TagResourceError::DuplicateTagKeys(String::from(error_message)),
"TooManyTagsException" => TagResourceError::TooManyTags(String::from(error_message)),
"ValidationException" => TagResourceError::Validation(error_message.to_string()),
_ => TagResourceError::Unknown(String::from(body))
                                }
                            },
                            Err(_) => TagResourceError::Unknown(String::from(body))
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
TagResourceError::Unknown(ref cause) => cause
                        }
                    }
                 }
/// Errors returned by UntagResource
                #[derive(Debug, PartialEq)]
                pub enum UntagResourceError {
                    
///<p>The API was called with invalid parameters. The error message will contain additional details about the cause.</p>
DirectConnectClient(String),
///<p>A server-side error occurred during the API call. The error message will contain additional details about the cause.</p>
DirectConnectServer(String),/// An error occurred dispatching the HTTP request
HttpDispatch(HttpDispatchError),/// An error was encountered with AWS credentials.
Credentials(CredentialsError),/// A validation error occurred.  Details from AWS are provided.
Validation(String),/// An unknown error occurred.  The raw HTTP response is provided.
Unknown(String)
                }

                
                impl UntagResourceError {
                    pub fn from_body(body: &str) -> UntagResourceError {
                        match from_str::<SerdeJsonValue>(body) {
                            Ok(json) => {
                                let raw_error_type = json.get("__type").and_then(|e| e.as_str()).unwrap_or("Unknown");
                                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                                let error_type = pieces.last().expect("Expected error type");

                                match *error_type {
                                    "DirectConnectClientException" => UntagResourceError::DirectConnectClient(String::from(error_message)),
"DirectConnectServerException" => UntagResourceError::DirectConnectServer(String::from(error_message)),
"ValidationException" => UntagResourceError::Validation(error_message.to_string()),
_ => UntagResourceError::Unknown(String::from(body))
                                }
                            },
                            Err(_) => UntagResourceError::Unknown(String::from(body))
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
UntagResourceError::Unknown(ref cause) => cause
                        }
                    }
                 }
/// Trait representing the capabilities of the AWS Direct Connect API. AWS Direct Connect clients implement this trait.
        pub trait DirectConnect {
        

                #[doc="<p>Creates a hosted connection on an interconnect.</p> <p>Allocates a VLAN number and a specified amount of bandwidth for use by a hosted connection on the given interconnect.</p> <note> <p>This is intended for use by AWS Direct Connect partners only.</p> </note>"]
                fn allocate_connection_on_interconnect(&self, input: &AllocateConnectionOnInterconnectRequest)  -> Result<Connection, AllocateConnectionOnInterconnectError>;
                

                #[doc="<p>Provisions a private virtual interface to be owned by a different customer.</p> <p>The owner of a connection calls this function to provision a private virtual interface which will be owned by another AWS customer.</p> <p>Virtual interfaces created using this function must be confirmed by the virtual interface owner by calling ConfirmPrivateVirtualInterface. Until this step has been completed, the virtual interface will be in 'Confirming' state, and will not be available for handling traffic.</p>"]
                fn allocate_private_virtual_interface(&self, input: &AllocatePrivateVirtualInterfaceRequest)  -> Result<VirtualInterface, AllocatePrivateVirtualInterfaceError>;
                

                #[doc="<p>Provisions a public virtual interface to be owned by a different customer.</p> <p>The owner of a connection calls this function to provision a public virtual interface which will be owned by another AWS customer.</p> <p>Virtual interfaces created using this function must be confirmed by the virtual interface owner by calling ConfirmPublicVirtualInterface. Until this step has been completed, the virtual interface will be in 'Confirming' state, and will not be available for handling traffic.</p> <p>When creating an IPv6 public virtual interface (addressFamily is 'ipv6'), the customer and amazon address fields should be left blank to use auto-assigned IPv6 space. Custom IPv6 Addresses are currently not supported.</p>"]
                fn allocate_public_virtual_interface(&self, input: &AllocatePublicVirtualInterfaceRequest)  -> Result<VirtualInterface, AllocatePublicVirtualInterfaceError>;
                

                #[doc="<p>Confirm the creation of a hosted connection on an interconnect.</p> <p>Upon creation, the hosted connection is initially in the 'Ordering' state, and will remain in this state until the owner calls ConfirmConnection to confirm creation of the hosted connection.</p>"]
                fn confirm_connection(&self, input: &ConfirmConnectionRequest)  -> Result<ConfirmConnectionResponse, ConfirmConnectionError>;
                

                #[doc="<p>Accept ownership of a private virtual interface created by another customer.</p> <p>After the virtual interface owner calls this function, the virtual interface will be created and attached to the given virtual private gateway, and will be available for handling traffic.</p>"]
                fn confirm_private_virtual_interface(&self, input: &ConfirmPrivateVirtualInterfaceRequest)  -> Result<ConfirmPrivateVirtualInterfaceResponse, ConfirmPrivateVirtualInterfaceError>;
                

                #[doc="<p>Accept ownership of a public virtual interface created by another customer.</p> <p>After the virtual interface owner calls this function, the specified virtual interface will be created and made available for handling traffic.</p>"]
                fn confirm_public_virtual_interface(&self, input: &ConfirmPublicVirtualInterfaceRequest)  -> Result<ConfirmPublicVirtualInterfaceResponse, ConfirmPublicVirtualInterfaceError>;
                

                #[doc="<p>Creates a new BGP peer on a specified virtual interface. The BGP peer cannot be in the same address family (IPv4/IPv6) of an existing BGP peer on the virtual interface.</p> <p>You must create a BGP peer for the corresponding address family in order to access AWS resources that also use that address family.</p> <p>When creating a IPv6 BGP peer, the Amazon address and customer address fields must be left blank. IPv6 addresses are automatically assigned from Amazon's pool of IPv6 addresses; you cannot specify custom IPv6 addresses.</p> <p>For a public virtual interface, the Autonomous System Number (ASN) must be private or already whitelisted for the virtual interface.</p>"]
                fn create_bgp_peer(&self, input: &CreateBGPPeerRequest)  -> Result<CreateBGPPeerResponse, CreateBGPPeerError>;
                

                #[doc="<p>Creates a new connection between the customer network and a specific AWS Direct Connect location.</p> <p>A connection links your internal network to an AWS Direct Connect location over a standard 1 gigabit or 10 gigabit Ethernet fiber-optic cable. One end of the cable is connected to your router, the other to an AWS Direct Connect router. An AWS Direct Connect location provides access to Amazon Web Services in the region it is associated with. You can establish connections with AWS Direct Connect locations in multiple regions, but a connection in one region does not provide connectivity to other regions.</p>"]
                fn create_connection(&self, input: &CreateConnectionRequest)  -> Result<Connection, CreateConnectionError>;
                

                #[doc="<p>Creates a new interconnect between a AWS Direct Connect partner's network and a specific AWS Direct Connect location.</p> <p>An interconnect is a connection which is capable of hosting other connections. The AWS Direct Connect partner can use an interconnect to provide sub-1Gbps AWS Direct Connect service to tier 2 customers who do not have their own connections. Like a standard connection, an interconnect links the AWS Direct Connect partner's network to an AWS Direct Connect location over a standard 1 Gbps or 10 Gbps Ethernet fiber-optic cable. One end is connected to the partner's router, the other to an AWS Direct Connect router.</p> <p>For each end customer, the AWS Direct Connect partner provisions a connection on their interconnect by calling AllocateConnectionOnInterconnect. The end customer can then connect to AWS resources by creating a virtual interface on their connection, using the VLAN assigned to them by the AWS Direct Connect partner.</p> <note> <p>This is intended for use by AWS Direct Connect partners only.</p> </note>"]
                fn create_interconnect(&self, input: &CreateInterconnectRequest)  -> Result<Interconnect, CreateInterconnectError>;
                

                #[doc="<p>Creates a new private virtual interface. A virtual interface is the VLAN that transports AWS Direct Connect traffic. A private virtual interface supports sending traffic to a single virtual private cloud (VPC).</p>"]
                fn create_private_virtual_interface(&self, input: &CreatePrivateVirtualInterfaceRequest)  -> Result<VirtualInterface, CreatePrivateVirtualInterfaceError>;
                

                #[doc="<p>Creates a new public virtual interface. A virtual interface is the VLAN that transports AWS Direct Connect traffic. A public virtual interface supports sending traffic to public services of AWS such as Amazon Simple Storage Service (Amazon S3).</p> <p>When creating an IPv6 public virtual interface (addressFamily is 'ipv6'), the customer and amazon address fields should be left blank to use auto-assigned IPv6 space. Custom IPv6 Addresses are currently not supported.</p>"]
                fn create_public_virtual_interface(&self, input: &CreatePublicVirtualInterfaceRequest)  -> Result<VirtualInterface, CreatePublicVirtualInterfaceError>;
                

                #[doc="<p>Deletes a BGP peer on the specified virtual interface that matches the specified customer address and ASN. You cannot delete the last BGP peer from a virtual interface.</p>"]
                fn delete_bgp_peer(&self, input: &DeleteBGPPeerRequest)  -> Result<DeleteBGPPeerResponse, DeleteBGPPeerError>;
                

                #[doc="<p>Deletes the connection.</p> <p>Deleting a connection only stops the AWS Direct Connect port hour and data transfer charges. You need to cancel separately with the providers any services or charges for cross-connects or network circuits that connect you to the AWS Direct Connect location.</p>"]
                fn delete_connection(&self, input: &DeleteConnectionRequest)  -> Result<Connection, DeleteConnectionError>;
                

                #[doc="<p>Deletes the specified interconnect.</p> <note> <p>This is intended for use by AWS Direct Connect partners only.</p> </note>"]
                fn delete_interconnect(&self, input: &DeleteInterconnectRequest)  -> Result<DeleteInterconnectResponse, DeleteInterconnectError>;
                

                #[doc="<p>Deletes a virtual interface.</p>"]
                fn delete_virtual_interface(&self, input: &DeleteVirtualInterfaceRequest)  -> Result<DeleteVirtualInterfaceResponse, DeleteVirtualInterfaceError>;
                

                #[doc="<p>Returns the LOA-CFA for a Connection.</p> <p>The Letter of Authorization - Connecting Facility Assignment (LOA-CFA) is a document that your APN partner or service provider uses when establishing your cross connect to AWS at the colocation facility. For more information, see <a href=\"http://docs.aws.amazon.com/directconnect/latest/UserGuide/Colocation.html\">Requesting Cross Connects at AWS Direct Connect Locations</a> in the AWS Direct Connect user guide.</p>"]
                fn describe_connection_loa(&self, input: &DescribeConnectionLoaRequest)  -> Result<DescribeConnectionLoaResponse, DescribeConnectionLoaError>;
                

                #[doc="<p>Displays all connections in this region.</p> <p>If a connection ID is provided, the call returns only that particular connection.</p>"]
                fn describe_connections(&self, input: &DescribeConnectionsRequest)  -> Result<Connections, DescribeConnectionsError>;
                

                #[doc="<p>Return a list of connections that have been provisioned on the given interconnect.</p> <note> <p>This is intended for use by AWS Direct Connect partners only.</p> </note>"]
                fn describe_connections_on_interconnect(&self, input: &DescribeConnectionsOnInterconnectRequest)  -> Result<Connections, DescribeConnectionsOnInterconnectError>;
                

                #[doc="<p>Returns the LOA-CFA for an Interconnect.</p> <p>The Letter of Authorization - Connecting Facility Assignment (LOA-CFA) is a document that is used when establishing your cross connect to AWS at the colocation facility. For more information, see <a href=\"http://docs.aws.amazon.com/directconnect/latest/UserGuide/Colocation.html\">Requesting Cross Connects at AWS Direct Connect Locations</a> in the AWS Direct Connect user guide.</p>"]
                fn describe_interconnect_loa(&self, input: &DescribeInterconnectLoaRequest)  -> Result<DescribeInterconnectLoaResponse, DescribeInterconnectLoaError>;
                

                #[doc="<p>Returns a list of interconnects owned by the AWS account.</p> <p>If an interconnect ID is provided, it will only return this particular interconnect.</p>"]
                fn describe_interconnects(&self, input: &DescribeInterconnectsRequest)  -> Result<Interconnects, DescribeInterconnectsError>;
                

                #[doc="<p>Returns the list of AWS Direct Connect locations in the current AWS region. These are the locations that may be selected when calling CreateConnection or CreateInterconnect.</p>"]
                fn describe_locations(&self)  -> Result<Locations, DescribeLocationsError>;
                

                #[doc="<p>Describes the tags associated with the specified Direct Connect resources.</p>"]
                fn describe_tags(&self, input: &DescribeTagsRequest)  -> Result<DescribeTagsResponse, DescribeTagsError>;
                

                #[doc="<p>Returns a list of virtual private gateways owned by the AWS account.</p> <p>You can create one or more AWS Direct Connect private virtual interfaces linking to a virtual private gateway. A virtual private gateway can be managed via Amazon Virtual Private Cloud (VPC) console or the <a href=\"http://docs.aws.amazon.com/AWSEC2/latest/APIReference/ApiReference-query-CreateVpnGateway.html\">EC2 CreateVpnGateway</a> action.</p>"]
                fn describe_virtual_gateways(&self)  -> Result<VirtualGateways, DescribeVirtualGatewaysError>;
                

                #[doc="<p>Displays all virtual interfaces for an AWS account. Virtual interfaces deleted fewer than 15 minutes before DescribeVirtualInterfaces is called are also returned. If a connection ID is included then only virtual interfaces associated with this connection will be returned. If a virtual interface ID is included then only a single virtual interface will be returned.</p> <p>A virtual interface (VLAN) transmits the traffic between the AWS Direct Connect location and the customer.</p> <p>If a connection ID is provided, only virtual interfaces provisioned on the specified connection will be returned. If a virtual interface ID is provided, only this particular virtual interface will be returned.</p>"]
                fn describe_virtual_interfaces(&self, input: &DescribeVirtualInterfacesRequest)  -> Result<VirtualInterfaces, DescribeVirtualInterfacesError>;
                

                #[doc="<p>Adds the specified tags to the specified Direct Connect resource. Each Direct Connect resource can have a maximum of 50 tags.</p> <p>Each tag consists of a key and an optional value. If a tag with the same key is already associated with the Direct Connect resource, this action updates its value.</p>"]
                fn tag_resource(&self, input: &TagResourceRequest)  -> Result<TagResourceResponse, TagResourceError>;
                

                #[doc="<p>Removes one or more tags from the specified Direct Connect resource.</p>"]
                fn untag_resource(&self, input: &UntagResourceRequest)  -> Result<UntagResourceResponse, UntagResourceError>;
                
}
/// A client for the AWS Direct Connect API.
        pub struct DirectConnectClient<P, D> where P: ProvideAwsCredentials, D: DispatchSignedRequest {
            credentials_provider: P,
            region: region::Region,
            dispatcher: D,
        }

        impl<P, D> DirectConnectClient<P, D> where P: ProvideAwsCredentials, D: DispatchSignedRequest {
            pub fn new(request_dispatcher: D, credentials_provider: P, region: region::Region) -> Self {
                  DirectConnectClient {
                    credentials_provider: credentials_provider,
                    region: region,
                    dispatcher: request_dispatcher
                }
            }
        }

        impl<P, D> DirectConnect for DirectConnectClient<P, D> where P: ProvideAwsCredentials, D: DispatchSignedRequest {
        

                #[doc="<p>Creates a hosted connection on an interconnect.</p> <p>Allocates a VLAN number and a specified amount of bandwidth for use by a hosted connection on the given interconnect.</p> <note> <p>This is intended for use by AWS Direct Connect partners only.</p> </note>"]
                fn allocate_connection_on_interconnect(&self, input: &AllocateConnectionOnInterconnectRequest)  -> Result<Connection, AllocateConnectionOnInterconnectError> {
                    let mut request = SignedRequest::new("POST", "directconnect", self.region, "/");
                    
                    request.set_content_type("application/x-amz-json-1.1".to_owned());
                    request.add_header("x-amz-target", "OvertureService.AllocateConnectionOnInterconnect");
                    let encoded = serde_json::to_string(input).unwrap();
         request.set_payload(Some(encoded.into_bytes()));
         
                    request.sign(&try!(self.credentials_provider.credentials()));

                    let response = try!(self.dispatcher.dispatch(&request));

                    match response.status {
                        StatusCode::Ok => {
                            Ok(serde_json::from_str::<Connection>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
                        }
                        _ => Err(AllocateConnectionOnInterconnectError::from_body(String::from_utf8_lossy(&response.body).as_ref())),
                    }
                }
                

                #[doc="<p>Provisions a private virtual interface to be owned by a different customer.</p> <p>The owner of a connection calls this function to provision a private virtual interface which will be owned by another AWS customer.</p> <p>Virtual interfaces created using this function must be confirmed by the virtual interface owner by calling ConfirmPrivateVirtualInterface. Until this step has been completed, the virtual interface will be in 'Confirming' state, and will not be available for handling traffic.</p>"]
                fn allocate_private_virtual_interface(&self, input: &AllocatePrivateVirtualInterfaceRequest)  -> Result<VirtualInterface, AllocatePrivateVirtualInterfaceError> {
                    let mut request = SignedRequest::new("POST", "directconnect", self.region, "/");
                    
                    request.set_content_type("application/x-amz-json-1.1".to_owned());
                    request.add_header("x-amz-target", "OvertureService.AllocatePrivateVirtualInterface");
                    let encoded = serde_json::to_string(input).unwrap();
         request.set_payload(Some(encoded.into_bytes()));
         
                    request.sign(&try!(self.credentials_provider.credentials()));

                    let response = try!(self.dispatcher.dispatch(&request));

                    match response.status {
                        StatusCode::Ok => {
                            Ok(serde_json::from_str::<VirtualInterface>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
                        }
                        _ => Err(AllocatePrivateVirtualInterfaceError::from_body(String::from_utf8_lossy(&response.body).as_ref())),
                    }
                }
                

                #[doc="<p>Provisions a public virtual interface to be owned by a different customer.</p> <p>The owner of a connection calls this function to provision a public virtual interface which will be owned by another AWS customer.</p> <p>Virtual interfaces created using this function must be confirmed by the virtual interface owner by calling ConfirmPublicVirtualInterface. Until this step has been completed, the virtual interface will be in 'Confirming' state, and will not be available for handling traffic.</p> <p>When creating an IPv6 public virtual interface (addressFamily is 'ipv6'), the customer and amazon address fields should be left blank to use auto-assigned IPv6 space. Custom IPv6 Addresses are currently not supported.</p>"]
                fn allocate_public_virtual_interface(&self, input: &AllocatePublicVirtualInterfaceRequest)  -> Result<VirtualInterface, AllocatePublicVirtualInterfaceError> {
                    let mut request = SignedRequest::new("POST", "directconnect", self.region, "/");
                    
                    request.set_content_type("application/x-amz-json-1.1".to_owned());
                    request.add_header("x-amz-target", "OvertureService.AllocatePublicVirtualInterface");
                    let encoded = serde_json::to_string(input).unwrap();
         request.set_payload(Some(encoded.into_bytes()));
         
                    request.sign(&try!(self.credentials_provider.credentials()));

                    let response = try!(self.dispatcher.dispatch(&request));

                    match response.status {
                        StatusCode::Ok => {
                            Ok(serde_json::from_str::<VirtualInterface>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
                        }
                        _ => Err(AllocatePublicVirtualInterfaceError::from_body(String::from_utf8_lossy(&response.body).as_ref())),
                    }
                }
                

                #[doc="<p>Confirm the creation of a hosted connection on an interconnect.</p> <p>Upon creation, the hosted connection is initially in the 'Ordering' state, and will remain in this state until the owner calls ConfirmConnection to confirm creation of the hosted connection.</p>"]
                fn confirm_connection(&self, input: &ConfirmConnectionRequest)  -> Result<ConfirmConnectionResponse, ConfirmConnectionError> {
                    let mut request = SignedRequest::new("POST", "directconnect", self.region, "/");
                    
                    request.set_content_type("application/x-amz-json-1.1".to_owned());
                    request.add_header("x-amz-target", "OvertureService.ConfirmConnection");
                    let encoded = serde_json::to_string(input).unwrap();
         request.set_payload(Some(encoded.into_bytes()));
         
                    request.sign(&try!(self.credentials_provider.credentials()));

                    let response = try!(self.dispatcher.dispatch(&request));

                    match response.status {
                        StatusCode::Ok => {
                            Ok(serde_json::from_str::<ConfirmConnectionResponse>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
                        }
                        _ => Err(ConfirmConnectionError::from_body(String::from_utf8_lossy(&response.body).as_ref())),
                    }
                }
                

                #[doc="<p>Accept ownership of a private virtual interface created by another customer.</p> <p>After the virtual interface owner calls this function, the virtual interface will be created and attached to the given virtual private gateway, and will be available for handling traffic.</p>"]
                fn confirm_private_virtual_interface(&self, input: &ConfirmPrivateVirtualInterfaceRequest)  -> Result<ConfirmPrivateVirtualInterfaceResponse, ConfirmPrivateVirtualInterfaceError> {
                    let mut request = SignedRequest::new("POST", "directconnect", self.region, "/");
                    
                    request.set_content_type("application/x-amz-json-1.1".to_owned());
                    request.add_header("x-amz-target", "OvertureService.ConfirmPrivateVirtualInterface");
                    let encoded = serde_json::to_string(input).unwrap();
         request.set_payload(Some(encoded.into_bytes()));
         
                    request.sign(&try!(self.credentials_provider.credentials()));

                    let response = try!(self.dispatcher.dispatch(&request));

                    match response.status {
                        StatusCode::Ok => {
                            Ok(serde_json::from_str::<ConfirmPrivateVirtualInterfaceResponse>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
                        }
                        _ => Err(ConfirmPrivateVirtualInterfaceError::from_body(String::from_utf8_lossy(&response.body).as_ref())),
                    }
                }
                

                #[doc="<p>Accept ownership of a public virtual interface created by another customer.</p> <p>After the virtual interface owner calls this function, the specified virtual interface will be created and made available for handling traffic.</p>"]
                fn confirm_public_virtual_interface(&self, input: &ConfirmPublicVirtualInterfaceRequest)  -> Result<ConfirmPublicVirtualInterfaceResponse, ConfirmPublicVirtualInterfaceError> {
                    let mut request = SignedRequest::new("POST", "directconnect", self.region, "/");
                    
                    request.set_content_type("application/x-amz-json-1.1".to_owned());
                    request.add_header("x-amz-target", "OvertureService.ConfirmPublicVirtualInterface");
                    let encoded = serde_json::to_string(input).unwrap();
         request.set_payload(Some(encoded.into_bytes()));
         
                    request.sign(&try!(self.credentials_provider.credentials()));

                    let response = try!(self.dispatcher.dispatch(&request));

                    match response.status {
                        StatusCode::Ok => {
                            Ok(serde_json::from_str::<ConfirmPublicVirtualInterfaceResponse>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
                        }
                        _ => Err(ConfirmPublicVirtualInterfaceError::from_body(String::from_utf8_lossy(&response.body).as_ref())),
                    }
                }
                

                #[doc="<p>Creates a new BGP peer on a specified virtual interface. The BGP peer cannot be in the same address family (IPv4/IPv6) of an existing BGP peer on the virtual interface.</p> <p>You must create a BGP peer for the corresponding address family in order to access AWS resources that also use that address family.</p> <p>When creating a IPv6 BGP peer, the Amazon address and customer address fields must be left blank. IPv6 addresses are automatically assigned from Amazon's pool of IPv6 addresses; you cannot specify custom IPv6 addresses.</p> <p>For a public virtual interface, the Autonomous System Number (ASN) must be private or already whitelisted for the virtual interface.</p>"]
                fn create_bgp_peer(&self, input: &CreateBGPPeerRequest)  -> Result<CreateBGPPeerResponse, CreateBGPPeerError> {
                    let mut request = SignedRequest::new("POST", "directconnect", self.region, "/");
                    
                    request.set_content_type("application/x-amz-json-1.1".to_owned());
                    request.add_header("x-amz-target", "OvertureService.CreateBGPPeer");
                    let encoded = serde_json::to_string(input).unwrap();
         request.set_payload(Some(encoded.into_bytes()));
         
                    request.sign(&try!(self.credentials_provider.credentials()));

                    let response = try!(self.dispatcher.dispatch(&request));

                    match response.status {
                        StatusCode::Ok => {
                            Ok(serde_json::from_str::<CreateBGPPeerResponse>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
                        }
                        _ => Err(CreateBGPPeerError::from_body(String::from_utf8_lossy(&response.body).as_ref())),
                    }
                }
                

                #[doc="<p>Creates a new connection between the customer network and a specific AWS Direct Connect location.</p> <p>A connection links your internal network to an AWS Direct Connect location over a standard 1 gigabit or 10 gigabit Ethernet fiber-optic cable. One end of the cable is connected to your router, the other to an AWS Direct Connect router. An AWS Direct Connect location provides access to Amazon Web Services in the region it is associated with. You can establish connections with AWS Direct Connect locations in multiple regions, but a connection in one region does not provide connectivity to other regions.</p>"]
                fn create_connection(&self, input: &CreateConnectionRequest)  -> Result<Connection, CreateConnectionError> {
                    let mut request = SignedRequest::new("POST", "directconnect", self.region, "/");
                    
                    request.set_content_type("application/x-amz-json-1.1".to_owned());
                    request.add_header("x-amz-target", "OvertureService.CreateConnection");
                    let encoded = serde_json::to_string(input).unwrap();
         request.set_payload(Some(encoded.into_bytes()));
         
                    request.sign(&try!(self.credentials_provider.credentials()));

                    let response = try!(self.dispatcher.dispatch(&request));

                    match response.status {
                        StatusCode::Ok => {
                            Ok(serde_json::from_str::<Connection>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
                        }
                        _ => Err(CreateConnectionError::from_body(String::from_utf8_lossy(&response.body).as_ref())),
                    }
                }
                

                #[doc="<p>Creates a new interconnect between a AWS Direct Connect partner's network and a specific AWS Direct Connect location.</p> <p>An interconnect is a connection which is capable of hosting other connections. The AWS Direct Connect partner can use an interconnect to provide sub-1Gbps AWS Direct Connect service to tier 2 customers who do not have their own connections. Like a standard connection, an interconnect links the AWS Direct Connect partner's network to an AWS Direct Connect location over a standard 1 Gbps or 10 Gbps Ethernet fiber-optic cable. One end is connected to the partner's router, the other to an AWS Direct Connect router.</p> <p>For each end customer, the AWS Direct Connect partner provisions a connection on their interconnect by calling AllocateConnectionOnInterconnect. The end customer can then connect to AWS resources by creating a virtual interface on their connection, using the VLAN assigned to them by the AWS Direct Connect partner.</p> <note> <p>This is intended for use by AWS Direct Connect partners only.</p> </note>"]
                fn create_interconnect(&self, input: &CreateInterconnectRequest)  -> Result<Interconnect, CreateInterconnectError> {
                    let mut request = SignedRequest::new("POST", "directconnect", self.region, "/");
                    
                    request.set_content_type("application/x-amz-json-1.1".to_owned());
                    request.add_header("x-amz-target", "OvertureService.CreateInterconnect");
                    let encoded = serde_json::to_string(input).unwrap();
         request.set_payload(Some(encoded.into_bytes()));
         
                    request.sign(&try!(self.credentials_provider.credentials()));

                    let response = try!(self.dispatcher.dispatch(&request));

                    match response.status {
                        StatusCode::Ok => {
                            Ok(serde_json::from_str::<Interconnect>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
                        }
                        _ => Err(CreateInterconnectError::from_body(String::from_utf8_lossy(&response.body).as_ref())),
                    }
                }
                

                #[doc="<p>Creates a new private virtual interface. A virtual interface is the VLAN that transports AWS Direct Connect traffic. A private virtual interface supports sending traffic to a single virtual private cloud (VPC).</p>"]
                fn create_private_virtual_interface(&self, input: &CreatePrivateVirtualInterfaceRequest)  -> Result<VirtualInterface, CreatePrivateVirtualInterfaceError> {
                    let mut request = SignedRequest::new("POST", "directconnect", self.region, "/");
                    
                    request.set_content_type("application/x-amz-json-1.1".to_owned());
                    request.add_header("x-amz-target", "OvertureService.CreatePrivateVirtualInterface");
                    let encoded = serde_json::to_string(input).unwrap();
         request.set_payload(Some(encoded.into_bytes()));
         
                    request.sign(&try!(self.credentials_provider.credentials()));

                    let response = try!(self.dispatcher.dispatch(&request));

                    match response.status {
                        StatusCode::Ok => {
                            Ok(serde_json::from_str::<VirtualInterface>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
                        }
                        _ => Err(CreatePrivateVirtualInterfaceError::from_body(String::from_utf8_lossy(&response.body).as_ref())),
                    }
                }
                

                #[doc="<p>Creates a new public virtual interface. A virtual interface is the VLAN that transports AWS Direct Connect traffic. A public virtual interface supports sending traffic to public services of AWS such as Amazon Simple Storage Service (Amazon S3).</p> <p>When creating an IPv6 public virtual interface (addressFamily is 'ipv6'), the customer and amazon address fields should be left blank to use auto-assigned IPv6 space. Custom IPv6 Addresses are currently not supported.</p>"]
                fn create_public_virtual_interface(&self, input: &CreatePublicVirtualInterfaceRequest)  -> Result<VirtualInterface, CreatePublicVirtualInterfaceError> {
                    let mut request = SignedRequest::new("POST", "directconnect", self.region, "/");
                    
                    request.set_content_type("application/x-amz-json-1.1".to_owned());
                    request.add_header("x-amz-target", "OvertureService.CreatePublicVirtualInterface");
                    let encoded = serde_json::to_string(input).unwrap();
         request.set_payload(Some(encoded.into_bytes()));
         
                    request.sign(&try!(self.credentials_provider.credentials()));

                    let response = try!(self.dispatcher.dispatch(&request));

                    match response.status {
                        StatusCode::Ok => {
                            Ok(serde_json::from_str::<VirtualInterface>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
                        }
                        _ => Err(CreatePublicVirtualInterfaceError::from_body(String::from_utf8_lossy(&response.body).as_ref())),
                    }
                }
                

                #[doc="<p>Deletes a BGP peer on the specified virtual interface that matches the specified customer address and ASN. You cannot delete the last BGP peer from a virtual interface.</p>"]
                fn delete_bgp_peer(&self, input: &DeleteBGPPeerRequest)  -> Result<DeleteBGPPeerResponse, DeleteBGPPeerError> {
                    let mut request = SignedRequest::new("POST", "directconnect", self.region, "/");
                    
                    request.set_content_type("application/x-amz-json-1.1".to_owned());
                    request.add_header("x-amz-target", "OvertureService.DeleteBGPPeer");
                    let encoded = serde_json::to_string(input).unwrap();
         request.set_payload(Some(encoded.into_bytes()));
         
                    request.sign(&try!(self.credentials_provider.credentials()));

                    let response = try!(self.dispatcher.dispatch(&request));

                    match response.status {
                        StatusCode::Ok => {
                            Ok(serde_json::from_str::<DeleteBGPPeerResponse>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
                        }
                        _ => Err(DeleteBGPPeerError::from_body(String::from_utf8_lossy(&response.body).as_ref())),
                    }
                }
                

                #[doc="<p>Deletes the connection.</p> <p>Deleting a connection only stops the AWS Direct Connect port hour and data transfer charges. You need to cancel separately with the providers any services or charges for cross-connects or network circuits that connect you to the AWS Direct Connect location.</p>"]
                fn delete_connection(&self, input: &DeleteConnectionRequest)  -> Result<Connection, DeleteConnectionError> {
                    let mut request = SignedRequest::new("POST", "directconnect", self.region, "/");
                    
                    request.set_content_type("application/x-amz-json-1.1".to_owned());
                    request.add_header("x-amz-target", "OvertureService.DeleteConnection");
                    let encoded = serde_json::to_string(input).unwrap();
         request.set_payload(Some(encoded.into_bytes()));
         
                    request.sign(&try!(self.credentials_provider.credentials()));

                    let response = try!(self.dispatcher.dispatch(&request));

                    match response.status {
                        StatusCode::Ok => {
                            Ok(serde_json::from_str::<Connection>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
                        }
                        _ => Err(DeleteConnectionError::from_body(String::from_utf8_lossy(&response.body).as_ref())),
                    }
                }
                

                #[doc="<p>Deletes the specified interconnect.</p> <note> <p>This is intended for use by AWS Direct Connect partners only.</p> </note>"]
                fn delete_interconnect(&self, input: &DeleteInterconnectRequest)  -> Result<DeleteInterconnectResponse, DeleteInterconnectError> {
                    let mut request = SignedRequest::new("POST", "directconnect", self.region, "/");
                    
                    request.set_content_type("application/x-amz-json-1.1".to_owned());
                    request.add_header("x-amz-target", "OvertureService.DeleteInterconnect");
                    let encoded = serde_json::to_string(input).unwrap();
         request.set_payload(Some(encoded.into_bytes()));
         
                    request.sign(&try!(self.credentials_provider.credentials()));

                    let response = try!(self.dispatcher.dispatch(&request));

                    match response.status {
                        StatusCode::Ok => {
                            Ok(serde_json::from_str::<DeleteInterconnectResponse>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
                        }
                        _ => Err(DeleteInterconnectError::from_body(String::from_utf8_lossy(&response.body).as_ref())),
                    }
                }
                

                #[doc="<p>Deletes a virtual interface.</p>"]
                fn delete_virtual_interface(&self, input: &DeleteVirtualInterfaceRequest)  -> Result<DeleteVirtualInterfaceResponse, DeleteVirtualInterfaceError> {
                    let mut request = SignedRequest::new("POST", "directconnect", self.region, "/");
                    
                    request.set_content_type("application/x-amz-json-1.1".to_owned());
                    request.add_header("x-amz-target", "OvertureService.DeleteVirtualInterface");
                    let encoded = serde_json::to_string(input).unwrap();
         request.set_payload(Some(encoded.into_bytes()));
         
                    request.sign(&try!(self.credentials_provider.credentials()));

                    let response = try!(self.dispatcher.dispatch(&request));

                    match response.status {
                        StatusCode::Ok => {
                            Ok(serde_json::from_str::<DeleteVirtualInterfaceResponse>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
                        }
                        _ => Err(DeleteVirtualInterfaceError::from_body(String::from_utf8_lossy(&response.body).as_ref())),
                    }
                }
                

                #[doc="<p>Returns the LOA-CFA for a Connection.</p> <p>The Letter of Authorization - Connecting Facility Assignment (LOA-CFA) is a document that your APN partner or service provider uses when establishing your cross connect to AWS at the colocation facility. For more information, see <a href=\"http://docs.aws.amazon.com/directconnect/latest/UserGuide/Colocation.html\">Requesting Cross Connects at AWS Direct Connect Locations</a> in the AWS Direct Connect user guide.</p>"]
                fn describe_connection_loa(&self, input: &DescribeConnectionLoaRequest)  -> Result<DescribeConnectionLoaResponse, DescribeConnectionLoaError> {
                    let mut request = SignedRequest::new("POST", "directconnect", self.region, "/");
                    
                    request.set_content_type("application/x-amz-json-1.1".to_owned());
                    request.add_header("x-amz-target", "OvertureService.DescribeConnectionLoa");
                    let encoded = serde_json::to_string(input).unwrap();
         request.set_payload(Some(encoded.into_bytes()));
         
                    request.sign(&try!(self.credentials_provider.credentials()));

                    let response = try!(self.dispatcher.dispatch(&request));

                    match response.status {
                        StatusCode::Ok => {
                            Ok(serde_json::from_str::<DescribeConnectionLoaResponse>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
                        }
                        _ => Err(DescribeConnectionLoaError::from_body(String::from_utf8_lossy(&response.body).as_ref())),
                    }
                }
                

                #[doc="<p>Displays all connections in this region.</p> <p>If a connection ID is provided, the call returns only that particular connection.</p>"]
                fn describe_connections(&self, input: &DescribeConnectionsRequest)  -> Result<Connections, DescribeConnectionsError> {
                    let mut request = SignedRequest::new("POST", "directconnect", self.region, "/");
                    
                    request.set_content_type("application/x-amz-json-1.1".to_owned());
                    request.add_header("x-amz-target", "OvertureService.DescribeConnections");
                    let encoded = serde_json::to_string(input).unwrap();
         request.set_payload(Some(encoded.into_bytes()));
         
                    request.sign(&try!(self.credentials_provider.credentials()));

                    let response = try!(self.dispatcher.dispatch(&request));

                    match response.status {
                        StatusCode::Ok => {
                            Ok(serde_json::from_str::<Connections>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
                        }
                        _ => Err(DescribeConnectionsError::from_body(String::from_utf8_lossy(&response.body).as_ref())),
                    }
                }
                

                #[doc="<p>Return a list of connections that have been provisioned on the given interconnect.</p> <note> <p>This is intended for use by AWS Direct Connect partners only.</p> </note>"]
                fn describe_connections_on_interconnect(&self, input: &DescribeConnectionsOnInterconnectRequest)  -> Result<Connections, DescribeConnectionsOnInterconnectError> {
                    let mut request = SignedRequest::new("POST", "directconnect", self.region, "/");
                    
                    request.set_content_type("application/x-amz-json-1.1".to_owned());
                    request.add_header("x-amz-target", "OvertureService.DescribeConnectionsOnInterconnect");
                    let encoded = serde_json::to_string(input).unwrap();
         request.set_payload(Some(encoded.into_bytes()));
         
                    request.sign(&try!(self.credentials_provider.credentials()));

                    let response = try!(self.dispatcher.dispatch(&request));

                    match response.status {
                        StatusCode::Ok => {
                            Ok(serde_json::from_str::<Connections>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
                        }
                        _ => Err(DescribeConnectionsOnInterconnectError::from_body(String::from_utf8_lossy(&response.body).as_ref())),
                    }
                }
                

                #[doc="<p>Returns the LOA-CFA for an Interconnect.</p> <p>The Letter of Authorization - Connecting Facility Assignment (LOA-CFA) is a document that is used when establishing your cross connect to AWS at the colocation facility. For more information, see <a href=\"http://docs.aws.amazon.com/directconnect/latest/UserGuide/Colocation.html\">Requesting Cross Connects at AWS Direct Connect Locations</a> in the AWS Direct Connect user guide.</p>"]
                fn describe_interconnect_loa(&self, input: &DescribeInterconnectLoaRequest)  -> Result<DescribeInterconnectLoaResponse, DescribeInterconnectLoaError> {
                    let mut request = SignedRequest::new("POST", "directconnect", self.region, "/");
                    
                    request.set_content_type("application/x-amz-json-1.1".to_owned());
                    request.add_header("x-amz-target", "OvertureService.DescribeInterconnectLoa");
                    let encoded = serde_json::to_string(input).unwrap();
         request.set_payload(Some(encoded.into_bytes()));
         
                    request.sign(&try!(self.credentials_provider.credentials()));

                    let response = try!(self.dispatcher.dispatch(&request));

                    match response.status {
                        StatusCode::Ok => {
                            Ok(serde_json::from_str::<DescribeInterconnectLoaResponse>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
                        }
                        _ => Err(DescribeInterconnectLoaError::from_body(String::from_utf8_lossy(&response.body).as_ref())),
                    }
                }
                

                #[doc="<p>Returns a list of interconnects owned by the AWS account.</p> <p>If an interconnect ID is provided, it will only return this particular interconnect.</p>"]
                fn describe_interconnects(&self, input: &DescribeInterconnectsRequest)  -> Result<Interconnects, DescribeInterconnectsError> {
                    let mut request = SignedRequest::new("POST", "directconnect", self.region, "/");
                    
                    request.set_content_type("application/x-amz-json-1.1".to_owned());
                    request.add_header("x-amz-target", "OvertureService.DescribeInterconnects");
                    let encoded = serde_json::to_string(input).unwrap();
         request.set_payload(Some(encoded.into_bytes()));
         
                    request.sign(&try!(self.credentials_provider.credentials()));

                    let response = try!(self.dispatcher.dispatch(&request));

                    match response.status {
                        StatusCode::Ok => {
                            Ok(serde_json::from_str::<Interconnects>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
                        }
                        _ => Err(DescribeInterconnectsError::from_body(String::from_utf8_lossy(&response.body).as_ref())),
                    }
                }
                

                #[doc="<p>Returns the list of AWS Direct Connect locations in the current AWS region. These are the locations that may be selected when calling CreateConnection or CreateInterconnect.</p>"]
                fn describe_locations(&self)  -> Result<Locations, DescribeLocationsError> {
                    let mut request = SignedRequest::new("POST", "directconnect", self.region, "/");
                    
                    request.set_content_type("application/x-amz-json-1.1".to_owned());
                    request.add_header("x-amz-target", "OvertureService.DescribeLocations");
                    
                    request.sign(&try!(self.credentials_provider.credentials()));

                    let response = try!(self.dispatcher.dispatch(&request));

                    match response.status {
                        StatusCode::Ok => {
                            Ok(serde_json::from_str::<Locations>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
                        }
                        _ => Err(DescribeLocationsError::from_body(String::from_utf8_lossy(&response.body).as_ref())),
                    }
                }
                

                #[doc="<p>Describes the tags associated with the specified Direct Connect resources.</p>"]
                fn describe_tags(&self, input: &DescribeTagsRequest)  -> Result<DescribeTagsResponse, DescribeTagsError> {
                    let mut request = SignedRequest::new("POST", "directconnect", self.region, "/");
                    
                    request.set_content_type("application/x-amz-json-1.1".to_owned());
                    request.add_header("x-amz-target", "OvertureService.DescribeTags");
                    let encoded = serde_json::to_string(input).unwrap();
         request.set_payload(Some(encoded.into_bytes()));
         
                    request.sign(&try!(self.credentials_provider.credentials()));

                    let response = try!(self.dispatcher.dispatch(&request));

                    match response.status {
                        StatusCode::Ok => {
                            Ok(serde_json::from_str::<DescribeTagsResponse>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
                        }
                        _ => Err(DescribeTagsError::from_body(String::from_utf8_lossy(&response.body).as_ref())),
                    }
                }
                

                #[doc="<p>Returns a list of virtual private gateways owned by the AWS account.</p> <p>You can create one or more AWS Direct Connect private virtual interfaces linking to a virtual private gateway. A virtual private gateway can be managed via Amazon Virtual Private Cloud (VPC) console or the <a href=\"http://docs.aws.amazon.com/AWSEC2/latest/APIReference/ApiReference-query-CreateVpnGateway.html\">EC2 CreateVpnGateway</a> action.</p>"]
                fn describe_virtual_gateways(&self)  -> Result<VirtualGateways, DescribeVirtualGatewaysError> {
                    let mut request = SignedRequest::new("POST", "directconnect", self.region, "/");
                    
                    request.set_content_type("application/x-amz-json-1.1".to_owned());
                    request.add_header("x-amz-target", "OvertureService.DescribeVirtualGateways");
                    
                    request.sign(&try!(self.credentials_provider.credentials()));

                    let response = try!(self.dispatcher.dispatch(&request));

                    match response.status {
                        StatusCode::Ok => {
                            Ok(serde_json::from_str::<VirtualGateways>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
                        }
                        _ => Err(DescribeVirtualGatewaysError::from_body(String::from_utf8_lossy(&response.body).as_ref())),
                    }
                }
                

                #[doc="<p>Displays all virtual interfaces for an AWS account. Virtual interfaces deleted fewer than 15 minutes before DescribeVirtualInterfaces is called are also returned. If a connection ID is included then only virtual interfaces associated with this connection will be returned. If a virtual interface ID is included then only a single virtual interface will be returned.</p> <p>A virtual interface (VLAN) transmits the traffic between the AWS Direct Connect location and the customer.</p> <p>If a connection ID is provided, only virtual interfaces provisioned on the specified connection will be returned. If a virtual interface ID is provided, only this particular virtual interface will be returned.</p>"]
                fn describe_virtual_interfaces(&self, input: &DescribeVirtualInterfacesRequest)  -> Result<VirtualInterfaces, DescribeVirtualInterfacesError> {
                    let mut request = SignedRequest::new("POST", "directconnect", self.region, "/");
                    
                    request.set_content_type("application/x-amz-json-1.1".to_owned());
                    request.add_header("x-amz-target", "OvertureService.DescribeVirtualInterfaces");
                    let encoded = serde_json::to_string(input).unwrap();
         request.set_payload(Some(encoded.into_bytes()));
         
                    request.sign(&try!(self.credentials_provider.credentials()));

                    let response = try!(self.dispatcher.dispatch(&request));

                    match response.status {
                        StatusCode::Ok => {
                            Ok(serde_json::from_str::<VirtualInterfaces>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
                        }
                        _ => Err(DescribeVirtualInterfacesError::from_body(String::from_utf8_lossy(&response.body).as_ref())),
                    }
                }
                

                #[doc="<p>Adds the specified tags to the specified Direct Connect resource. Each Direct Connect resource can have a maximum of 50 tags.</p> <p>Each tag consists of a key and an optional value. If a tag with the same key is already associated with the Direct Connect resource, this action updates its value.</p>"]
                fn tag_resource(&self, input: &TagResourceRequest)  -> Result<TagResourceResponse, TagResourceError> {
                    let mut request = SignedRequest::new("POST", "directconnect", self.region, "/");
                    
                    request.set_content_type("application/x-amz-json-1.1".to_owned());
                    request.add_header("x-amz-target", "OvertureService.TagResource");
                    let encoded = serde_json::to_string(input).unwrap();
         request.set_payload(Some(encoded.into_bytes()));
         
                    request.sign(&try!(self.credentials_provider.credentials()));

                    let response = try!(self.dispatcher.dispatch(&request));

                    match response.status {
                        StatusCode::Ok => {
                            Ok(serde_json::from_str::<TagResourceResponse>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
                        }
                        _ => Err(TagResourceError::from_body(String::from_utf8_lossy(&response.body).as_ref())),
                    }
                }
                

                #[doc="<p>Removes one or more tags from the specified Direct Connect resource.</p>"]
                fn untag_resource(&self, input: &UntagResourceRequest)  -> Result<UntagResourceResponse, UntagResourceError> {
                    let mut request = SignedRequest::new("POST", "directconnect", self.region, "/");
                    
                    request.set_content_type("application/x-amz-json-1.1".to_owned());
                    request.add_header("x-amz-target", "OvertureService.UntagResource");
                    let encoded = serde_json::to_string(input).unwrap();
         request.set_payload(Some(encoded.into_bytes()));
         
                    request.sign(&try!(self.credentials_provider.credentials()));

                    let response = try!(self.dispatcher.dispatch(&request));

                    match response.status {
                        StatusCode::Ok => {
                            Ok(serde_json::from_str::<UntagResourceResponse>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
                        }
                        _ => Err(UntagResourceError::from_body(String::from_utf8_lossy(&response.body).as_ref())),
                    }
                }
                
}

            #[cfg(test)]
            mod protocol_tests {
                
            }
            
