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
#[allow(unused_imports)]
use rusoto_core::pagination::{all_pages, PagedOutput, PagedRequest, RusotoStream};
use rusoto_core::region;
use rusoto_core::request::{BufferedHttpResponse, DispatchSignedRequest};
use rusoto_core::{Client, RusotoError};

use rusoto_core::param::{Params, ServiceParams};
use rusoto_core::proto;
use rusoto_core::signature::SignedRequest;
#[allow(unused_imports)]
use serde::{Deserialize, Serialize};
use serde_json;
/// <p>A policy type that defines the voting rules for the network. The rules decide if a proposal is approved. Approval may be based on criteria such as the percentage of <code>YES</code> votes and the duration of the proposal. The policy applies to all proposals and is specified when the network is created.</p> <p>Applies only to Hyperledger Fabric.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct ApprovalThresholdPolicy {
    /// <p>The duration from the time that a proposal is created until it expires. If members cast neither the required number of <code>YES</code> votes to approve the proposal nor the number of <code>NO</code> votes required to reject it before the duration expires, the proposal is <code>EXPIRED</code> and <code>ProposalActions</code> are not carried out.</p>
    #[serde(rename = "ProposalDurationInHours")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proposal_duration_in_hours: Option<i64>,
    /// <p>Determines whether the vote percentage must be greater than the <code>ThresholdPercentage</code> or must be greater than or equal to the <code>ThreholdPercentage</code> to be approved.</p>
    #[serde(rename = "ThresholdComparator")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub threshold_comparator: Option<String>,
    /// <p>The percentage of votes among all members that must be <code>YES</code> for a proposal to be approved. For example, a <code>ThresholdPercentage</code> value of <code>50</code> indicates 50%. The <code>ThresholdComparator</code> determines the precise comparison. If a <code>ThresholdPercentage</code> value of <code>50</code> is specified on a network with 10 members, along with a <code>ThresholdComparator</code> value of <code>GREATER_THAN</code>, this indicates that 6 <code>YES</code> votes are required for the proposal to be approved.</p>
    #[serde(rename = "ThresholdPercentage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub threshold_percentage: Option<i64>,
}

/// see [ManagedBlockchain::create_member]
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateMemberInput {
    /// <p>A unique, case-sensitive identifier that you provide to ensure the idempotency of the operation. An idempotent operation completes no more than one time. This identifier is required only if you make a service request directly using an HTTP client. It is generated automatically if you use an AWS SDK or the AWS CLI.</p>
    #[serde(rename = "ClientRequestToken")]
    pub client_request_token: String,
    /// <p>The unique identifier of the invitation that is sent to the member to join the network.</p>
    #[serde(rename = "InvitationId")]
    pub invitation_id: String,
    /// <p>Member configuration parameters.</p>
    #[serde(rename = "MemberConfiguration")]
    pub member_configuration: MemberConfiguration,
    /// <p>The unique identifier of the network in which the member is created.</p>
    #[serde(rename = "NetworkId")]
    pub network_id: String,
}

/// see [ManagedBlockchain::create_member]
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateMemberOutput {
    /// <p>The unique identifier of the member.</p>
    #[serde(rename = "MemberId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member_id: Option<String>,
}

/// see [ManagedBlockchain::create_network]
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateNetworkInput {
    /// <p>A unique, case-sensitive identifier that you provide to ensure the idempotency of the operation. An idempotent operation completes no more than one time. This identifier is required only if you make a service request directly using an HTTP client. It is generated automatically if you use an AWS SDK or the AWS CLI.</p>
    #[serde(rename = "ClientRequestToken")]
    pub client_request_token: String,
    /// <p>An optional description for the network.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The blockchain framework that the network uses.</p>
    #[serde(rename = "Framework")]
    pub framework: String,
    /// <p> Configuration properties of the blockchain framework relevant to the network configuration. </p>
    #[serde(rename = "FrameworkConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub framework_configuration: Option<NetworkFrameworkConfiguration>,
    /// <p>The version of the blockchain framework that the network uses.</p>
    #[serde(rename = "FrameworkVersion")]
    pub framework_version: String,
    /// <p>Configuration properties for the first member within the network.</p>
    #[serde(rename = "MemberConfiguration")]
    pub member_configuration: MemberConfiguration,
    /// <p>The name of the network.</p>
    #[serde(rename = "Name")]
    pub name: String,
    /// <p> The voting rules used by the network to determine if a proposal is approved. </p>
    #[serde(rename = "VotingPolicy")]
    pub voting_policy: VotingPolicy,
}

/// see [ManagedBlockchain::create_network]
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateNetworkOutput {
    /// <p>The unique identifier for the first member within the network.</p>
    #[serde(rename = "MemberId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member_id: Option<String>,
    /// <p>The unique identifier for the network.</p>
    #[serde(rename = "NetworkId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_id: Option<String>,
}

/// see [ManagedBlockchain::create_node]
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateNodeInput {
    /// <p>A unique, case-sensitive identifier that you provide to ensure the idempotency of the operation. An idempotent operation completes no more than one time. This identifier is required only if you make a service request directly using an HTTP client. It is generated automatically if you use an AWS SDK or the AWS CLI.</p>
    #[serde(rename = "ClientRequestToken")]
    pub client_request_token: String,
    /// <p>The unique identifier of the member that owns this node.</p> <p>Applies only to Hyperledger Fabric.</p>
    #[serde(rename = "MemberId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member_id: Option<String>,
    /// <p><p>The unique identifier of the network for the node.</p> <p>Ethereum public networks have the following <code>NetworkId</code>s:</p> <ul> <li> <p> <code>n-ethereum-mainnet</code> </p> </li> <li> <p> <code>n-ethereum-rinkeby</code> </p> </li> <li> <p> <code>n-ethereum-ropsten</code> </p> </li> </ul></p>
    #[serde(rename = "NetworkId")]
    pub network_id: String,
    /// <p>The properties of a node configuration.</p>
    #[serde(rename = "NodeConfiguration")]
    pub node_configuration: NodeConfiguration,
}

/// see [ManagedBlockchain::create_node]
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateNodeOutput {
    /// <p>The unique identifier of the node.</p>
    #[serde(rename = "NodeId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_id: Option<String>,
}

/// see [ManagedBlockchain::create_proposal]
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateProposalInput {
    /// <p>The type of actions proposed, such as inviting a member or removing a member. The types of <code>Actions</code> in a proposal are mutually exclusive. For example, a proposal with <code>Invitations</code> actions cannot also contain <code>Removals</code> actions.</p>
    #[serde(rename = "Actions")]
    pub actions: ProposalActions,
    /// <p>A unique, case-sensitive identifier that you provide to ensure the idempotency of the operation. An idempotent operation completes no more than one time. This identifier is required only if you make a service request directly using an HTTP client. It is generated automatically if you use an AWS SDK or the AWS CLI.</p>
    #[serde(rename = "ClientRequestToken")]
    pub client_request_token: String,
    /// <p>A description for the proposal that is visible to voting members, for example, "Proposal to add Example Corp. as member."</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The unique identifier of the member that is creating the proposal. This identifier is especially useful for identifying the member making the proposal when multiple members exist in a single AWS account.</p>
    #[serde(rename = "MemberId")]
    pub member_id: String,
    /// <p> The unique identifier of the network for which the proposal is made.</p>
    #[serde(rename = "NetworkId")]
    pub network_id: String,
}

/// see [ManagedBlockchain::create_proposal]
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateProposalOutput {
    /// <p>The unique identifier of the proposal.</p>
    #[serde(rename = "ProposalId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proposal_id: Option<String>,
}

/// see [ManagedBlockchain::delete_member]
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteMemberInput {
    /// <p>The unique identifier of the member to remove.</p>
    #[serde(rename = "MemberId")]
    pub member_id: String,
    /// <p>The unique identifier of the network from which the member is removed.</p>
    #[serde(rename = "NetworkId")]
    pub network_id: String,
}

/// see [ManagedBlockchain::delete_member]
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DeleteMemberOutput {}

/// see [ManagedBlockchain::delete_node]
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteNodeInput {
    /// <p>The unique identifier of the member that owns this node.</p> <p>Applies only to Hyperledger Fabric and is required for Hyperledger Fabric.</p>
    #[serde(rename = "MemberId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member_id: Option<String>,
    /// <p><p>The unique identifier of the network that the node is on.</p> <p>Ethereum public networks have the following <code>NetworkId</code>s:</p> <ul> <li> <p> <code>n-ethereum-mainnet</code> </p> </li> <li> <p> <code>n-ethereum-rinkeby</code> </p> </li> <li> <p> <code>n-ethereum-ropsten</code> </p> </li> </ul></p>
    #[serde(rename = "NetworkId")]
    pub network_id: String,
    /// <p>The unique identifier of the node.</p>
    #[serde(rename = "NodeId")]
    pub node_id: String,
}

/// see [ManagedBlockchain::delete_node]
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DeleteNodeOutput {}

/// see [ManagedBlockchain::get_member]
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetMemberInput {
    /// <p>The unique identifier of the member.</p>
    #[serde(rename = "MemberId")]
    pub member_id: String,
    /// <p>The unique identifier of the network to which the member belongs.</p>
    #[serde(rename = "NetworkId")]
    pub network_id: String,
}

/// see [ManagedBlockchain::get_member]
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetMemberOutput {
    /// <p>The properties of a member.</p>
    #[serde(rename = "Member")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member: Option<Member>,
}

/// see [ManagedBlockchain::get_network]
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetNetworkInput {
    /// <p>The unique identifier of the network to get information about.</p>
    #[serde(rename = "NetworkId")]
    pub network_id: String,
}

/// see [ManagedBlockchain::get_network]
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetNetworkOutput {
    /// <p>An object containing network configuration parameters.</p>
    #[serde(rename = "Network")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network: Option<Network>,
}

/// see [ManagedBlockchain::get_node]
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetNodeInput {
    /// <p>The unique identifier of the member that owns the node.</p> <p>Applies only to Hyperledger Fabric and is required for Hyperledger Fabric.</p>
    #[serde(rename = "MemberId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member_id: Option<String>,
    /// <p>The unique identifier of the network that the node is on.</p>
    #[serde(rename = "NetworkId")]
    pub network_id: String,
    /// <p>The unique identifier of the node.</p>
    #[serde(rename = "NodeId")]
    pub node_id: String,
}

/// see [ManagedBlockchain::get_node]
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetNodeOutput {
    /// <p>Properties of the node configuration.</p>
    #[serde(rename = "Node")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node: Option<Node>,
}

/// see [ManagedBlockchain::get_proposal]
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetProposalInput {
    /// <p>The unique identifier of the network for which the proposal is made.</p>
    #[serde(rename = "NetworkId")]
    pub network_id: String,
    /// <p>The unique identifier of the proposal.</p>
    #[serde(rename = "ProposalId")]
    pub proposal_id: String,
}

/// see [ManagedBlockchain::get_proposal]
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetProposalOutput {
    /// <p>Information about a proposal.</p>
    #[serde(rename = "Proposal")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proposal: Option<Proposal>,
}

/// <p>An invitation to an AWS account to create a member and join the network.</p> <p>Applies only to Hyperledger Fabric.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct Invitation {
    /// <p>The date and time that the invitation was created.</p>
    #[serde(rename = "CreationDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_date: Option<f64>,
    /// <p>The date and time that the invitation expires. This is the <code>CreationDate</code> plus the <code>ProposalDurationInHours</code> that is specified in the <code>ProposalThresholdPolicy</code>. After this date and time, the invitee can no longer create a member and join the network using this <code>InvitationId</code>.</p>
    #[serde(rename = "ExpirationDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expiration_date: Option<f64>,
    /// <p>The unique identifier for the invitation.</p>
    #[serde(rename = "InvitationId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invitation_id: Option<String>,
    #[serde(rename = "NetworkSummary")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_summary: Option<NetworkSummary>,
    /// <p><p>The status of the invitation:</p> <ul> <li> <p> <code>PENDING</code> - The invitee has not created a member to join the network, and the invitation has not yet expired.</p> </li> <li> <p> <code>ACCEPTING</code> - The invitee has begun creating a member, and creation has not yet completed.</p> </li> <li> <p> <code>ACCEPTED</code> - The invitee created a member and joined the network using the <code>InvitationID</code>.</p> </li> <li> <p> <code>REJECTED</code> - The invitee rejected the invitation.</p> </li> <li> <p> <code>EXPIRED</code> - The invitee neither created a member nor rejected the invitation before the <code>ExpirationDate</code>.</p> </li> </ul></p>
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

/// <p>An action to invite a specific AWS account to create a member and join the network. The <code>InviteAction</code> is carried out when a <code>Proposal</code> is <code>APPROVED</code>.</p> <p>Applies only to Hyperledger Fabric.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct InviteAction {
    /// <p>The AWS account ID to invite.</p>
    #[serde(rename = "Principal")]
    pub principal: String,
}

/// see [ManagedBlockchain::list_invitations]
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListInvitationsInput {
    /// <p>The maximum number of invitations to return.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>The pagination token that indicates the next set of results to retrieve.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

/// see [ManagedBlockchain::list_invitations]
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListInvitationsOutput {
    /// <p>The invitations for the network.</p>
    #[serde(rename = "Invitations")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invitations: Option<Vec<Invitation>>,
    /// <p>The pagination token that indicates the next set of results to retrieve.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

/// see [ManagedBlockchain::list_members]
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListMembersInput {
    /// <p>An optional Boolean value. If provided, the request is limited either to members that the current AWS account owns (<code>true</code>) or that other AWS accounts own (<code>false</code>). If omitted, all members are listed.</p>
    #[serde(rename = "IsOwned")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_owned: Option<bool>,
    /// <p>The maximum number of members to return in the request.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>The optional name of the member to list.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The unique identifier of the network for which to list members.</p>
    #[serde(rename = "NetworkId")]
    pub network_id: String,
    /// <p>The pagination token that indicates the next set of results to retrieve.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>An optional status specifier. If provided, only members currently in this status are listed.</p>
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

/// see [ManagedBlockchain::list_members]
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListMembersOutput {
    /// <p>An array of <code>MemberSummary</code> objects. Each object contains details about a network member.</p>
    #[serde(rename = "Members")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub members: Option<Vec<MemberSummary>>,
    /// <p>The pagination token that indicates the next set of results to retrieve.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

/// see [ManagedBlockchain::list_networks]
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListNetworksInput {
    /// <p>An optional framework specifier. If provided, only networks of this framework type are listed.</p>
    #[serde(rename = "Framework")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub framework: Option<String>,
    /// <p>The maximum number of networks to list.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>The name of the network.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The pagination token that indicates the next set of results to retrieve.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>An optional status specifier. If provided, only networks currently in this status are listed.</p> <p>Applies only to Hyperledger Fabric.</p>
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

/// see [ManagedBlockchain::list_networks]
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListNetworksOutput {
    /// <p>An array of <code>NetworkSummary</code> objects that contain configuration properties for each network.</p>
    #[serde(rename = "Networks")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub networks: Option<Vec<NetworkSummary>>,
    /// <p>The pagination token that indicates the next set of results to retrieve.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

/// see [ManagedBlockchain::list_nodes]
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListNodesInput {
    /// <p>The maximum number of nodes to list.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>The unique identifier of the member who owns the nodes to list.</p> <p>Applies only to Hyperledger Fabric and is required for Hyperledger Fabric.</p>
    #[serde(rename = "MemberId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member_id: Option<String>,
    /// <p>The unique identifier of the network for which to list nodes.</p>
    #[serde(rename = "NetworkId")]
    pub network_id: String,
    /// <p>The pagination token that indicates the next set of results to retrieve.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>An optional status specifier. If provided, only nodes currently in this status are listed.</p>
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

/// see [ManagedBlockchain::list_nodes]
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListNodesOutput {
    /// <p>The pagination token that indicates the next set of results to retrieve.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>An array of <code>NodeSummary</code> objects that contain configuration properties for each node.</p>
    #[serde(rename = "Nodes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nodes: Option<Vec<NodeSummary>>,
}

/// see [ManagedBlockchain::list_proposal_votes]
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListProposalVotesInput {
    /// <p> The maximum number of votes to return. </p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p> The unique identifier of the network. </p>
    #[serde(rename = "NetworkId")]
    pub network_id: String,
    /// <p> The pagination token that indicates the next set of results to retrieve. </p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p> The unique identifier of the proposal. </p>
    #[serde(rename = "ProposalId")]
    pub proposal_id: String,
}

/// see [ManagedBlockchain::list_proposal_votes]
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListProposalVotesOutput {
    /// <p> The pagination token that indicates the next set of results to retrieve. </p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p> The list of votes. </p>
    #[serde(rename = "ProposalVotes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proposal_votes: Option<Vec<VoteSummary>>,
}

/// see [ManagedBlockchain::list_proposals]
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListProposalsInput {
    /// <p> The maximum number of proposals to return. </p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p> The unique identifier of the network. </p>
    #[serde(rename = "NetworkId")]
    pub network_id: String,
    /// <p> The pagination token that indicates the next set of results to retrieve. </p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

/// see [ManagedBlockchain::list_proposals]
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListProposalsOutput {
    /// <p>The pagination token that indicates the next set of results to retrieve.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The summary of each proposal made on the network.</p>
    #[serde(rename = "Proposals")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proposals: Option<Vec<ProposalSummary>>,
}

/// <p>A configuration for logging events.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct LogConfiguration {
    /// <p>Indicates whether logging is enabled.</p>
    #[serde(rename = "Enabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
}

/// <p>A collection of log configurations.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct LogConfigurations {
    /// <p>Parameters for publishing logs to Amazon CloudWatch Logs.</p>
    #[serde(rename = "Cloudwatch")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloudwatch: Option<LogConfiguration>,
}

/// <p>Member configuration properties.</p> <p>Applies only to Hyperledger Fabric.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct Member {
    /// <p>The date and time that the member was created.</p>
    #[serde(rename = "CreationDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_date: Option<f64>,
    /// <p>An optional description for the member.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>Attributes relevant to a member for the blockchain framework that the Managed Blockchain network uses.</p>
    #[serde(rename = "FrameworkAttributes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub framework_attributes: Option<MemberFrameworkAttributes>,
    /// <p>The unique identifier of the member.</p>
    #[serde(rename = "Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// <p>Configuration properties for logging events associated with a member.</p>
    #[serde(rename = "LogPublishingConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_publishing_configuration: Option<MemberLogPublishingConfiguration>,
    /// <p>The name of the member.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The unique identifier of the network to which the member belongs.</p>
    #[serde(rename = "NetworkId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_id: Option<String>,
    /// <p><p>The status of a member.</p> <ul> <li> <p> <code>CREATING</code> - The AWS account is in the process of creating a member.</p> </li> <li> <p> <code>AVAILABLE</code> - The member has been created and can participate in the network.</p> </li> <li> <p> <code>CREATE_FAILED</code> - The AWS account attempted to create a member and creation failed.</p> </li> <li> <p> <code>DELETING</code> - The member and all associated resources are in the process of being deleted. Either the AWS account that owns the member deleted it, or the member is being deleted as the result of an <code>APPROVED</code> <code>PROPOSAL</code> to remove the member.</p> </li> <li> <p> <code>DELETED</code> - The member can no longer participate on the network and all associated resources are deleted. Either the AWS account that owns the member deleted it, or the member is being deleted as the result of an <code>APPROVED</code> <code>PROPOSAL</code> to remove the member.</p> </li> </ul></p>
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

/// <p>Configuration properties of the member.</p> <p>Applies only to Hyperledger Fabric.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct MemberConfiguration {
    /// <p>An optional description of the member.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>Configuration properties of the blockchain framework relevant to the member.</p>
    #[serde(rename = "FrameworkConfiguration")]
    pub framework_configuration: MemberFrameworkConfiguration,
    /// <p>Configuration properties for logging events associated with a member of a Managed Blockchain network.</p>
    #[serde(rename = "LogPublishingConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_publishing_configuration: Option<MemberLogPublishingConfiguration>,
    /// <p>The name of the member.</p>
    #[serde(rename = "Name")]
    pub name: String,
}

/// <p>Attributes of Hyperledger Fabric for a member in a Managed Blockchain network using the Hyperledger Fabric framework.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct MemberFabricAttributes {
    /// <p>The user name for the initial administrator user for the member.</p>
    #[serde(rename = "AdminUsername")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub admin_username: Option<String>,
    /// <p>The endpoint used to access the member's certificate authority.</p>
    #[serde(rename = "CaEndpoint")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ca_endpoint: Option<String>,
}

/// <p>Configuration properties for Hyperledger Fabric for a member in a Managed Blockchain network using the Hyperledger Fabric framework.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct MemberFabricConfiguration {
    /// <p>The password for the member's initial administrative user. The <code>AdminPassword</code> must be at least eight characters long and no more than 32 characters. It must contain at least one uppercase letter, one lowercase letter, and one digit. It cannot have a single quotation mark (‘), a double quotation marks (“), a forward slash(/), a backward slash(\), @, or a space.</p>
    #[serde(rename = "AdminPassword")]
    pub admin_password: String,
    /// <p>The user name for the member's initial administrative user.</p>
    #[serde(rename = "AdminUsername")]
    pub admin_username: String,
}

/// <p>Configuration properties for logging events associated with a member of a Managed Blockchain network using the Hyperledger Fabric framework.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct MemberFabricLogPublishingConfiguration {
    /// <p>Configuration properties for logging events associated with a member's Certificate Authority (CA). CA logs help you determine when a member in your account joins the network, or when new peers register with a member CA.</p>
    #[serde(rename = "CaLogs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ca_logs: Option<LogConfigurations>,
}

/// <p>Attributes relevant to a member for the blockchain framework that the Managed Blockchain network uses.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct MemberFrameworkAttributes {
    /// <p>Attributes of Hyperledger Fabric relevant to a member on a Managed Blockchain network that uses Hyperledger Fabric.</p>
    #[serde(rename = "Fabric")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fabric: Option<MemberFabricAttributes>,
}

/// <p>Configuration properties relevant to a member for the blockchain framework that the Managed Blockchain network uses.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct MemberFrameworkConfiguration {
    /// <p>Attributes of Hyperledger Fabric for a member on a Managed Blockchain network that uses Hyperledger Fabric.</p>
    #[serde(rename = "Fabric")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fabric: Option<MemberFabricConfiguration>,
}

/// <p>Configuration properties for logging events associated with a member of a Managed Blockchain network.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct MemberLogPublishingConfiguration {
    /// <p>Configuration properties for logging events associated with a member of a Managed Blockchain network using the Hyperledger Fabric framework.</p>
    #[serde(rename = "Fabric")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fabric: Option<MemberFabricLogPublishingConfiguration>,
}

/// <p>A summary of configuration properties for a member.</p> <p>Applies only to Hyperledger Fabric.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct MemberSummary {
    /// <p>The date and time that the member was created.</p>
    #[serde(rename = "CreationDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_date: Option<f64>,
    /// <p>An optional description of the member.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The unique identifier of the member.</p>
    #[serde(rename = "Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// <p>An indicator of whether the member is owned by your AWS account or a different AWS account.</p>
    #[serde(rename = "IsOwned")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_owned: Option<bool>,
    /// <p>The name of the member.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p><p>The status of the member.</p> <ul> <li> <p> <code>CREATING</code> - The AWS account is in the process of creating a member.</p> </li> <li> <p> <code>AVAILABLE</code> - The member has been created and can participate in the network.</p> </li> <li> <p> <code>CREATE_FAILED</code> - The AWS account attempted to create a member and creation failed.</p> </li> <li> <p> <code>DELETING</code> - The member and all associated resources are in the process of being deleted. Either the AWS account that owns the member deleted it, or the member is being deleted as the result of an <code>APPROVED</code> <code>PROPOSAL</code> to remove the member.</p> </li> <li> <p> <code>DELETED</code> - The member can no longer participate on the network and all associated resources are deleted. Either the AWS account that owns the member deleted it, or the member is being deleted as the result of an <code>APPROVED</code> <code>PROPOSAL</code> to remove the member.</p> </li> </ul></p>
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

/// <p>Network configuration properties.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct Network {
    /// <p>The date and time that the network was created.</p>
    #[serde(rename = "CreationDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_date: Option<f64>,
    /// <p>Attributes of the blockchain framework for the network.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The blockchain framework that the network uses.</p>
    #[serde(rename = "Framework")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub framework: Option<String>,
    /// <p>Attributes of the blockchain framework that the network uses.</p>
    #[serde(rename = "FrameworkAttributes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub framework_attributes: Option<NetworkFrameworkAttributes>,
    /// <p>The version of the blockchain framework that the network uses.</p>
    #[serde(rename = "FrameworkVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub framework_version: Option<String>,
    /// <p>The unique identifier of the network.</p>
    #[serde(rename = "Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// <p>The name of the network.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The current status of the network.</p>
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// <p>The voting rules for the network to decide if a proposal is accepted.</p>
    #[serde(rename = "VotingPolicy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub voting_policy: Option<VotingPolicy>,
    /// <p>The VPC endpoint service name of the VPC endpoint service of the network. Members use the VPC endpoint service name to create a VPC endpoint to access network resources.</p>
    #[serde(rename = "VpcEndpointServiceName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_endpoint_service_name: Option<String>,
}

/// <p>Attributes of Ethereum for a network.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct NetworkEthereumAttributes {
    /// <p><p>The Ethereum <code>CHAIN_ID</code> associated with the Ethereum network. Chain IDs are as follows:</p> <ul> <li> <p>mainnet = <code>1</code> </p> </li> <li> <p>rinkeby = <code>4</code> </p> </li> <li> <p>ropsten = <code>3</code> </p> </li> </ul></p>
    #[serde(rename = "ChainId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chain_id: Option<String>,
}

/// <p>Attributes of Hyperledger Fabric for a network.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct NetworkFabricAttributes {
    /// <p>The edition of Amazon Managed Blockchain that Hyperledger Fabric uses. For more information, see <a href="http://aws.amazon.com/managed-blockchain/pricing/">Amazon Managed Blockchain Pricing</a>.</p>
    #[serde(rename = "Edition")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub edition: Option<String>,
    /// <p>The endpoint of the ordering service for the network.</p>
    #[serde(rename = "OrderingServiceEndpoint")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ordering_service_endpoint: Option<String>,
}

/// <p>Hyperledger Fabric configuration properties for the network.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct NetworkFabricConfiguration {
    /// <p>The edition of Amazon Managed Blockchain that the network uses. For more information, see <a href="http://aws.amazon.com/managed-blockchain/pricing/">Amazon Managed Blockchain Pricing</a>.</p>
    #[serde(rename = "Edition")]
    pub edition: String,
}

/// <p>Attributes relevant to the network for the blockchain framework that the network uses.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct NetworkFrameworkAttributes {
    /// <p>Attributes of an Ethereum network for Managed Blockchain resources participating in an Ethereum network.</p>
    #[serde(rename = "Ethereum")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ethereum: Option<NetworkEthereumAttributes>,
    /// <p>Attributes of Hyperledger Fabric for a Managed Blockchain network that uses Hyperledger Fabric.</p>
    #[serde(rename = "Fabric")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fabric: Option<NetworkFabricAttributes>,
}

/// <p> Configuration properties relevant to the network for the blockchain framework that the network uses. </p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct NetworkFrameworkConfiguration {
    /// <p> Hyperledger Fabric configuration properties for a Managed Blockchain network that uses Hyperledger Fabric. </p>
    #[serde(rename = "Fabric")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fabric: Option<NetworkFabricConfiguration>,
}

/// <p>A summary of network configuration properties.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct NetworkSummary {
    /// <p>The date and time that the network was created.</p>
    #[serde(rename = "CreationDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_date: Option<f64>,
    /// <p>An optional description of the network.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The blockchain framework that the network uses.</p>
    #[serde(rename = "Framework")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub framework: Option<String>,
    /// <p>The version of the blockchain framework that the network uses.</p>
    #[serde(rename = "FrameworkVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub framework_version: Option<String>,
    /// <p>The unique identifier of the network.</p>
    #[serde(rename = "Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// <p>The name of the network.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The current status of the network.</p>
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

/// <p>Configuration properties of a node.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct Node {
    /// <p>The Availability Zone in which the node exists.</p>
    #[serde(rename = "AvailabilityZone")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub availability_zone: Option<String>,
    /// <p>The date and time that the node was created.</p>
    #[serde(rename = "CreationDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_date: Option<f64>,
    /// <p>Attributes of the blockchain framework being used.</p>
    #[serde(rename = "FrameworkAttributes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub framework_attributes: Option<NodeFrameworkAttributes>,
    /// <p>The unique identifier of the node.</p>
    #[serde(rename = "Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// <p>The instance type of the node.</p>
    #[serde(rename = "InstanceType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_type: Option<String>,
    /// <p>Configuration properties for logging events associated with a peer node on a Hyperledger Fabric network on Managed Blockchain.</p>
    #[serde(rename = "LogPublishingConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_publishing_configuration: Option<NodeLogPublishingConfiguration>,
    /// <p>The unique identifier of the member to which the node belongs.</p> <p>Applies only to Hyperledger Fabric.</p>
    #[serde(rename = "MemberId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member_id: Option<String>,
    /// <p>The unique identifier of the network that the node is on.</p>
    #[serde(rename = "NetworkId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_id: Option<String>,
    /// <p>The state database that the node uses. Values are <code>LevelDB</code> or <code>CouchDB</code>.</p> <p>Applies only to Hyperledger Fabric.</p>
    #[serde(rename = "StateDB")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state_db: Option<String>,
    /// <p>The status of the node.</p>
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

/// <p>Configuration properties of a node.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct NodeConfiguration {
    /// <p>The Availability Zone in which the node exists.</p>
    #[serde(rename = "AvailabilityZone")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub availability_zone: Option<String>,
    /// <p>The Amazon Managed Blockchain instance type for the node.</p>
    #[serde(rename = "InstanceType")]
    pub instance_type: String,
    /// <p>Configuration properties for logging events associated with a peer node on a Hyperledger Fabric network on Managed Blockchain. </p>
    #[serde(rename = "LogPublishingConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_publishing_configuration: Option<NodeLogPublishingConfiguration>,
    /// <p>The state database that the node uses. Values are <code>LevelDB</code> or <code>CouchDB</code>. When using an Amazon Managed Blockchain network with Hyperledger Fabric version 1.4 or later, the default is <code>CouchDB</code>.</p> <p>Applies only to Hyperledger Fabric.</p>
    #[serde(rename = "StateDB")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state_db: Option<String>,
}

/// <p>Attributes of an Ethereum node.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct NodeEthereumAttributes {
    /// <p>The endpoint on which the Ethereum node listens to run Ethereum JSON-RPC methods over HTTP connections from a client. Use this endpoint in client code for smart contracts when using an HTTP connection. Connections to this endpoint are authenticated using <a href="https://docs.aws.amazon.com/general/latest/gr/signature-version-4.html">Signature Version 4</a>.</p>
    #[serde(rename = "HttpEndpoint")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub http_endpoint: Option<String>,
    /// <p>The endpoint on which the Ethereum node listens to run Ethereum JSON-RPC methods over WebSockets connections from a client. Use this endpoint in client code for smart contracts when using a WebSockets connection. Connections to this endpoint are authenticated using <a href="https://docs.aws.amazon.com/general/latest/gr/signature-version-4.html">Signature Version 4</a>.</p>
    #[serde(rename = "WebSocketEndpoint")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub web_socket_endpoint: Option<String>,
}

/// <p>Attributes of Hyperledger Fabric for a peer node on a Hyperledger Fabric network on Managed Blockchain.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct NodeFabricAttributes {
    /// <p>The endpoint that identifies the peer node for all services except peer channel-based event services.</p>
    #[serde(rename = "PeerEndpoint")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub peer_endpoint: Option<String>,
    /// <p>The endpoint that identifies the peer node for peer channel-based event services.</p>
    #[serde(rename = "PeerEventEndpoint")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub peer_event_endpoint: Option<String>,
}

/// <p>Configuration properties for logging events associated with a peer node owned by a member in a Managed Blockchain network.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct NodeFabricLogPublishingConfiguration {
    /// <p>Configuration properties for logging events associated with chaincode execution on a peer node. Chaincode logs contain the results of instantiating, invoking, and querying the chaincode. A peer can run multiple instances of chaincode. When enabled, a log stream is created for all chaincodes, with an individual log stream for each chaincode.</p>
    #[serde(rename = "ChaincodeLogs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chaincode_logs: Option<LogConfigurations>,
    /// <p>Configuration properties for a peer node log. Peer node logs contain messages generated when your client submits transaction proposals to peer nodes, requests to join channels, enrolls an admin peer, and lists the chaincode instances on a peer node. </p>
    #[serde(rename = "PeerLogs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub peer_logs: Option<LogConfigurations>,
}

/// <p>Attributes relevant to a node on a Managed Blockchain network for the blockchain framework that the network uses.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct NodeFrameworkAttributes {
    /// <p>Attributes of Ethereum for a node on a Managed Blockchain network that uses Ethereum.</p>
    #[serde(rename = "Ethereum")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ethereum: Option<NodeEthereumAttributes>,
    /// <p>Attributes of Hyperledger Fabric for a peer node on a Managed Blockchain network that uses Hyperledger Fabric.</p>
    #[serde(rename = "Fabric")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fabric: Option<NodeFabricAttributes>,
}

/// <p>Configuration properties for logging events associated with a peer node on a Hyperledger Fabric network on Managed Blockchain.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct NodeLogPublishingConfiguration {
    /// <p>Configuration properties for logging events associated with a node that is owned by a member of a Managed Blockchain network using the Hyperledger Fabric framework.</p>
    #[serde(rename = "Fabric")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fabric: Option<NodeFabricLogPublishingConfiguration>,
}

/// <p>A summary of configuration properties for a node.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct NodeSummary {
    /// <p>The Availability Zone in which the node exists.</p>
    #[serde(rename = "AvailabilityZone")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub availability_zone: Option<String>,
    /// <p>The date and time that the node was created.</p>
    #[serde(rename = "CreationDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_date: Option<f64>,
    /// <p>The unique identifier of the node.</p>
    #[serde(rename = "Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// <p>The EC2 instance type for the node.</p>
    #[serde(rename = "InstanceType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_type: Option<String>,
    /// <p>The status of the node.</p>
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

/// <p>Properties of a proposal on a Managed Blockchain network.</p> <p>Applies only to Hyperledger Fabric.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct Proposal {
    /// <p>The actions to perform on the network if the proposal is <code>APPROVED</code>.</p>
    #[serde(rename = "Actions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub actions: Option<ProposalActions>,
    /// <p> The date and time that the proposal was created. </p>
    #[serde(rename = "CreationDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_date: Option<f64>,
    /// <p>The description of the proposal.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p> The date and time that the proposal expires. This is the <code>CreationDate</code> plus the <code>ProposalDurationInHours</code> that is specified in the <code>ProposalThresholdPolicy</code>. After this date and time, if members have not cast enough votes to determine the outcome according to the voting policy, the proposal is <code>EXPIRED</code> and <code>Actions</code> are not carried out. </p>
    #[serde(rename = "ExpirationDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expiration_date: Option<f64>,
    /// <p>The unique identifier of the network for which the proposal is made.</p>
    #[serde(rename = "NetworkId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_id: Option<String>,
    /// <p> The current total of <code>NO</code> votes cast on the proposal by members. </p>
    #[serde(rename = "NoVoteCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_vote_count: Option<i64>,
    /// <p> The number of votes remaining to be cast on the proposal by members. In other words, the number of members minus the sum of <code>YES</code> votes and <code>NO</code> votes. </p>
    #[serde(rename = "OutstandingVoteCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub outstanding_vote_count: Option<i64>,
    /// <p>The unique identifier of the proposal.</p>
    #[serde(rename = "ProposalId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proposal_id: Option<String>,
    /// <p>The unique identifier of the member that created the proposal.</p>
    #[serde(rename = "ProposedByMemberId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proposed_by_member_id: Option<String>,
    /// <p>The name of the member that created the proposal.</p>
    #[serde(rename = "ProposedByMemberName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proposed_by_member_name: Option<String>,
    /// <p><p>The status of the proposal. Values are as follows:</p> <ul> <li> <p> <code>IN<em>PROGRESS</code> - The proposal is active and open for member voting.</p> </li> <li> <p> <code>APPROVED</code> - The proposal was approved with sufficient <code>YES</code> votes among members according to the <code>VotingPolicy</code> specified for the <code>Network</code>. The specified proposal actions are carried out.</p> </li> <li> <p> <code>REJECTED</code> - The proposal was rejected with insufficient <code>YES</code> votes among members according to the <code>VotingPolicy</code> specified for the <code>Network</code>. The specified <code>ProposalActions</code> are not carried out.</p> </li> <li> <p> <code>EXPIRED</code> - Members did not cast the number of votes required to determine the proposal outcome before the proposal expired. The specified <code>ProposalActions</code> are not carried out.</p> </li> <li> <p> <code>ACTION</em>FAILED</code> - One or more of the specified <code>ProposalActions</code> in a proposal that was approved could not be completed because of an error. The <code>ACTION_FAILED</code> status occurs even if only one ProposalAction fails and other actions are successful.</p> </li> </ul></p>
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// <p> The current total of <code>YES</code> votes cast on the proposal by members. </p>
    #[serde(rename = "YesVoteCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub yes_vote_count: Option<i64>,
}

/// <p> The actions to carry out if a proposal is <code>APPROVED</code>. </p> <p>Applies only to Hyperledger Fabric.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct ProposalActions {
    /// <p> The actions to perform for an <code>APPROVED</code> proposal to invite an AWS account to create a member and join the network. </p>
    #[serde(rename = "Invitations")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invitations: Option<Vec<InviteAction>>,
    /// <p> The actions to perform for an <code>APPROVED</code> proposal to remove a member from the network, which deletes the member and all associated member resources from the network. </p>
    #[serde(rename = "Removals")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub removals: Option<Vec<RemoveAction>>,
}

/// <p>Properties of a proposal.</p> <p>Applies only to Hyperledger Fabric.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ProposalSummary {
    /// <p> The date and time that the proposal was created. </p>
    #[serde(rename = "CreationDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_date: Option<f64>,
    /// <p> The description of the proposal. </p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p> The date and time that the proposal expires. This is the <code>CreationDate</code> plus the <code>ProposalDurationInHours</code> that is specified in the <code>ProposalThresholdPolicy</code>. After this date and time, if members have not cast enough votes to determine the outcome according to the voting policy, the proposal is <code>EXPIRED</code> and <code>Actions</code> are not carried out. </p>
    #[serde(rename = "ExpirationDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expiration_date: Option<f64>,
    /// <p> The unique identifier of the proposal. </p>
    #[serde(rename = "ProposalId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proposal_id: Option<String>,
    /// <p> The unique identifier of the member that created the proposal. </p>
    #[serde(rename = "ProposedByMemberId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proposed_by_member_id: Option<String>,
    /// <p> The name of the member that created the proposal. </p>
    #[serde(rename = "ProposedByMemberName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proposed_by_member_name: Option<String>,
    /// <p><p>The status of the proposal. Values are as follows:</p> <ul> <li> <p> <code>IN<em>PROGRESS</code> - The proposal is active and open for member voting.</p> </li> <li> <p> <code>APPROVED</code> - The proposal was approved with sufficient <code>YES</code> votes among members according to the <code>VotingPolicy</code> specified for the <code>Network</code>. The specified proposal actions are carried out.</p> </li> <li> <p> <code>REJECTED</code> - The proposal was rejected with insufficient <code>YES</code> votes among members according to the <code>VotingPolicy</code> specified for the <code>Network</code>. The specified <code>ProposalActions</code> are not carried out.</p> </li> <li> <p> <code>EXPIRED</code> - Members did not cast the number of votes required to determine the proposal outcome before the proposal expired. The specified <code>ProposalActions</code> are not carried out.</p> </li> <li> <p> <code>ACTION</em>FAILED</code> - One or more of the specified <code>ProposalActions</code> in a proposal that was approved could not be completed because of an error.</p> </li> </ul></p>
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

/// see [ManagedBlockchain::reject_invitation]
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct RejectInvitationInput {
    /// <p>The unique identifier of the invitation to reject.</p>
    #[serde(rename = "InvitationId")]
    pub invitation_id: String,
}

/// see [ManagedBlockchain::reject_invitation]
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct RejectInvitationOutput {}

/// <p>An action to remove a member from a Managed Blockchain network as the result of a removal proposal that is <code>APPROVED</code>. The member and all associated resources are deleted from the network.</p> <p>Applies only to Hyperledger Fabric.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct RemoveAction {
    /// <p>The unique identifier of the member to remove.</p>
    #[serde(rename = "MemberId")]
    pub member_id: String,
}

/// see [ManagedBlockchain::update_member]
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UpdateMemberInput {
    /// <p>Configuration properties for publishing to Amazon CloudWatch Logs.</p>
    #[serde(rename = "LogPublishingConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_publishing_configuration: Option<MemberLogPublishingConfiguration>,
    /// <p>The unique identifier of the member.</p>
    #[serde(rename = "MemberId")]
    pub member_id: String,
    /// <p>The unique identifier of the Managed Blockchain network to which the member belongs.</p>
    #[serde(rename = "NetworkId")]
    pub network_id: String,
}

/// see [ManagedBlockchain::update_member]
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UpdateMemberOutput {}

/// see [ManagedBlockchain::update_node]
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UpdateNodeInput {
    /// <p>Configuration properties for publishing to Amazon CloudWatch Logs.</p>
    #[serde(rename = "LogPublishingConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_publishing_configuration: Option<NodeLogPublishingConfiguration>,
    /// <p>The unique identifier of the member that owns the node.</p> <p>Applies only to Hyperledger Fabric.</p>
    #[serde(rename = "MemberId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member_id: Option<String>,
    /// <p>The unique identifier of the network that the node is on.</p>
    #[serde(rename = "NetworkId")]
    pub network_id: String,
    /// <p>The unique identifier of the node.</p>
    #[serde(rename = "NodeId")]
    pub node_id: String,
}

/// see [ManagedBlockchain::update_node]
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UpdateNodeOutput {}

/// see [ManagedBlockchain::vote_on_proposal]
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct VoteOnProposalInput {
    /// <p> The unique identifier of the network. </p>
    #[serde(rename = "NetworkId")]
    pub network_id: String,
    /// <p> The unique identifier of the proposal. </p>
    #[serde(rename = "ProposalId")]
    pub proposal_id: String,
    /// <p> The value of the vote. </p>
    #[serde(rename = "Vote")]
    pub vote: String,
    /// <p>The unique identifier of the member casting the vote. </p>
    #[serde(rename = "VoterMemberId")]
    pub voter_member_id: String,
}

/// see [ManagedBlockchain::vote_on_proposal]
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct VoteOnProposalOutput {}

/// <p> Properties of an individual vote that a member cast for a proposal. </p> <p>Applies only to Hyperledger Fabric.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct VoteSummary {
    /// <p> The unique identifier of the member that cast the vote. </p>
    #[serde(rename = "MemberId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member_id: Option<String>,
    /// <p> The name of the member that cast the vote. </p>
    #[serde(rename = "MemberName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member_name: Option<String>,
    /// <p> The vote value, either <code>YES</code> or <code>NO</code>. </p>
    #[serde(rename = "Vote")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vote: Option<String>,
}

/// <p> The voting rules for the network to decide if a proposal is accepted </p> <p>Applies only to Hyperledger Fabric.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct VotingPolicy {
    /// <p>Defines the rules for the network for voting on proposals, such as the percentage of <code>YES</code> votes required for the proposal to be approved and the duration of the proposal. The policy applies to all proposals and is specified when the network is created.</p>
    #[serde(rename = "ApprovalThresholdPolicy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub approval_threshold_policy: Option<ApprovalThresholdPolicy>,
}

/// Errors returned by CreateMember
#[derive(Debug, PartialEq)]
pub enum CreateMemberError {
    /// <p>You do not have sufficient access to perform this action.</p>
    AccessDenied(String),
    /// <p>The request processing has failed because of an unknown error, exception or failure.</p>
    InternalServiceError(String),
    /// <p>The action or operation requested is invalid. Verify that the action is typed correctly.</p>
    InvalidRequest(String),
    /// <p>A resource request is issued for a resource that already exists.</p>
    ResourceAlreadyExists(String),
    /// <p>The maximum number of resources of that type already exist. Ensure the resources requested are within the boundaries of the service edition and your account limits.</p>
    ResourceLimitExceeded(String),
    /// <p>A requested resource does not exist on the network. It may have been deleted or referenced inaccurately.</p>
    ResourceNotFound(String),
    /// <p>The requested resource exists but is not in a status that can complete the operation.</p>
    ResourceNotReady(String),
    /// <p>The request or operation could not be performed because a service is throttling requests. The most common source of throttling errors is launching EC2 instances such that your service limit for EC2 instances is exceeded. Request a limit increase or delete unused resources if possible.</p>
    Throttling(String),
}

impl CreateMemberError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateMemberError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(CreateMemberError::AccessDenied(err.msg))
                }
                "InternalServiceErrorException" => {
                    return RusotoError::Service(CreateMemberError::InternalServiceError(err.msg))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(CreateMemberError::InvalidRequest(err.msg))
                }
                "ResourceAlreadyExistsException" => {
                    return RusotoError::Service(CreateMemberError::ResourceAlreadyExists(err.msg))
                }
                "ResourceLimitExceededException" => {
                    return RusotoError::Service(CreateMemberError::ResourceLimitExceeded(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(CreateMemberError::ResourceNotFound(err.msg))
                }
                "ResourceNotReadyException" => {
                    return RusotoError::Service(CreateMemberError::ResourceNotReady(err.msg))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(CreateMemberError::Throttling(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for CreateMemberError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreateMemberError::AccessDenied(ref cause) => write!(f, "{}", cause),
            CreateMemberError::InternalServiceError(ref cause) => write!(f, "{}", cause),
            CreateMemberError::InvalidRequest(ref cause) => write!(f, "{}", cause),
            CreateMemberError::ResourceAlreadyExists(ref cause) => write!(f, "{}", cause),
            CreateMemberError::ResourceLimitExceeded(ref cause) => write!(f, "{}", cause),
            CreateMemberError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            CreateMemberError::ResourceNotReady(ref cause) => write!(f, "{}", cause),
            CreateMemberError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CreateMemberError {}
/// Errors returned by CreateNetwork
#[derive(Debug, PartialEq)]
pub enum CreateNetworkError {
    /// <p>You do not have sufficient access to perform this action.</p>
    AccessDenied(String),
    /// <p>The request processing has failed because of an unknown error, exception or failure.</p>
    InternalServiceError(String),
    /// <p>The action or operation requested is invalid. Verify that the action is typed correctly.</p>
    InvalidRequest(String),
    /// <p>A resource request is issued for a resource that already exists.</p>
    ResourceAlreadyExists(String),
    /// <p>The maximum number of resources of that type already exist. Ensure the resources requested are within the boundaries of the service edition and your account limits.</p>
    ResourceLimitExceeded(String),
    /// <p>The request or operation could not be performed because a service is throttling requests. The most common source of throttling errors is launching EC2 instances such that your service limit for EC2 instances is exceeded. Request a limit increase or delete unused resources if possible.</p>
    Throttling(String),
}

impl CreateNetworkError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateNetworkError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(CreateNetworkError::AccessDenied(err.msg))
                }
                "InternalServiceErrorException" => {
                    return RusotoError::Service(CreateNetworkError::InternalServiceError(err.msg))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(CreateNetworkError::InvalidRequest(err.msg))
                }
                "ResourceAlreadyExistsException" => {
                    return RusotoError::Service(CreateNetworkError::ResourceAlreadyExists(err.msg))
                }
                "ResourceLimitExceededException" => {
                    return RusotoError::Service(CreateNetworkError::ResourceLimitExceeded(err.msg))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(CreateNetworkError::Throttling(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for CreateNetworkError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreateNetworkError::AccessDenied(ref cause) => write!(f, "{}", cause),
            CreateNetworkError::InternalServiceError(ref cause) => write!(f, "{}", cause),
            CreateNetworkError::InvalidRequest(ref cause) => write!(f, "{}", cause),
            CreateNetworkError::ResourceAlreadyExists(ref cause) => write!(f, "{}", cause),
            CreateNetworkError::ResourceLimitExceeded(ref cause) => write!(f, "{}", cause),
            CreateNetworkError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CreateNetworkError {}
/// Errors returned by CreateNode
#[derive(Debug, PartialEq)]
pub enum CreateNodeError {
    /// <p>You do not have sufficient access to perform this action.</p>
    AccessDenied(String),
    /// <p>The request processing has failed because of an unknown error, exception or failure.</p>
    InternalServiceError(String),
    /// <p>The action or operation requested is invalid. Verify that the action is typed correctly.</p>
    InvalidRequest(String),
    /// <p>A resource request is issued for a resource that already exists.</p>
    ResourceAlreadyExists(String),
    /// <p>The maximum number of resources of that type already exist. Ensure the resources requested are within the boundaries of the service edition and your account limits.</p>
    ResourceLimitExceeded(String),
    /// <p>A requested resource does not exist on the network. It may have been deleted or referenced inaccurately.</p>
    ResourceNotFound(String),
    /// <p>The requested resource exists but is not in a status that can complete the operation.</p>
    ResourceNotReady(String),
    /// <p>The request or operation could not be performed because a service is throttling requests. The most common source of throttling errors is launching EC2 instances such that your service limit for EC2 instances is exceeded. Request a limit increase or delete unused resources if possible.</p>
    Throttling(String),
}

impl CreateNodeError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateNodeError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(CreateNodeError::AccessDenied(err.msg))
                }
                "InternalServiceErrorException" => {
                    return RusotoError::Service(CreateNodeError::InternalServiceError(err.msg))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(CreateNodeError::InvalidRequest(err.msg))
                }
                "ResourceAlreadyExistsException" => {
                    return RusotoError::Service(CreateNodeError::ResourceAlreadyExists(err.msg))
                }
                "ResourceLimitExceededException" => {
                    return RusotoError::Service(CreateNodeError::ResourceLimitExceeded(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(CreateNodeError::ResourceNotFound(err.msg))
                }
                "ResourceNotReadyException" => {
                    return RusotoError::Service(CreateNodeError::ResourceNotReady(err.msg))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(CreateNodeError::Throttling(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for CreateNodeError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreateNodeError::AccessDenied(ref cause) => write!(f, "{}", cause),
            CreateNodeError::InternalServiceError(ref cause) => write!(f, "{}", cause),
            CreateNodeError::InvalidRequest(ref cause) => write!(f, "{}", cause),
            CreateNodeError::ResourceAlreadyExists(ref cause) => write!(f, "{}", cause),
            CreateNodeError::ResourceLimitExceeded(ref cause) => write!(f, "{}", cause),
            CreateNodeError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            CreateNodeError::ResourceNotReady(ref cause) => write!(f, "{}", cause),
            CreateNodeError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CreateNodeError {}
/// Errors returned by CreateProposal
#[derive(Debug, PartialEq)]
pub enum CreateProposalError {
    /// <p>You do not have sufficient access to perform this action.</p>
    AccessDenied(String),
    /// <p>The request processing has failed because of an unknown error, exception or failure.</p>
    InternalServiceError(String),
    /// <p>The action or operation requested is invalid. Verify that the action is typed correctly.</p>
    InvalidRequest(String),
    /// <p>A requested resource does not exist on the network. It may have been deleted or referenced inaccurately.</p>
    ResourceNotFound(String),
    /// <p>The requested resource exists but is not in a status that can complete the operation.</p>
    ResourceNotReady(String),
    /// <p>The request or operation could not be performed because a service is throttling requests. The most common source of throttling errors is launching EC2 instances such that your service limit for EC2 instances is exceeded. Request a limit increase or delete unused resources if possible.</p>
    Throttling(String),
}

impl CreateProposalError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateProposalError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(CreateProposalError::AccessDenied(err.msg))
                }
                "InternalServiceErrorException" => {
                    return RusotoError::Service(CreateProposalError::InternalServiceError(err.msg))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(CreateProposalError::InvalidRequest(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(CreateProposalError::ResourceNotFound(err.msg))
                }
                "ResourceNotReadyException" => {
                    return RusotoError::Service(CreateProposalError::ResourceNotReady(err.msg))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(CreateProposalError::Throttling(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for CreateProposalError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreateProposalError::AccessDenied(ref cause) => write!(f, "{}", cause),
            CreateProposalError::InternalServiceError(ref cause) => write!(f, "{}", cause),
            CreateProposalError::InvalidRequest(ref cause) => write!(f, "{}", cause),
            CreateProposalError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            CreateProposalError::ResourceNotReady(ref cause) => write!(f, "{}", cause),
            CreateProposalError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CreateProposalError {}
/// Errors returned by DeleteMember
#[derive(Debug, PartialEq)]
pub enum DeleteMemberError {
    /// <p>You do not have sufficient access to perform this action.</p>
    AccessDenied(String),
    /// <p>The request processing has failed because of an unknown error, exception or failure.</p>
    InternalServiceError(String),
    /// <p>The action or operation requested is invalid. Verify that the action is typed correctly.</p>
    InvalidRequest(String),
    /// <p>A requested resource does not exist on the network. It may have been deleted or referenced inaccurately.</p>
    ResourceNotFound(String),
    /// <p>The requested resource exists but is not in a status that can complete the operation.</p>
    ResourceNotReady(String),
    /// <p>The request or operation could not be performed because a service is throttling requests. The most common source of throttling errors is launching EC2 instances such that your service limit for EC2 instances is exceeded. Request a limit increase or delete unused resources if possible.</p>
    Throttling(String),
}

impl DeleteMemberError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteMemberError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(DeleteMemberError::AccessDenied(err.msg))
                }
                "InternalServiceErrorException" => {
                    return RusotoError::Service(DeleteMemberError::InternalServiceError(err.msg))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(DeleteMemberError::InvalidRequest(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(DeleteMemberError::ResourceNotFound(err.msg))
                }
                "ResourceNotReadyException" => {
                    return RusotoError::Service(DeleteMemberError::ResourceNotReady(err.msg))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(DeleteMemberError::Throttling(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DeleteMemberError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteMemberError::AccessDenied(ref cause) => write!(f, "{}", cause),
            DeleteMemberError::InternalServiceError(ref cause) => write!(f, "{}", cause),
            DeleteMemberError::InvalidRequest(ref cause) => write!(f, "{}", cause),
            DeleteMemberError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            DeleteMemberError::ResourceNotReady(ref cause) => write!(f, "{}", cause),
            DeleteMemberError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeleteMemberError {}
/// Errors returned by DeleteNode
#[derive(Debug, PartialEq)]
pub enum DeleteNodeError {
    /// <p>You do not have sufficient access to perform this action.</p>
    AccessDenied(String),
    /// <p>The request processing has failed because of an unknown error, exception or failure.</p>
    InternalServiceError(String),
    /// <p>The action or operation requested is invalid. Verify that the action is typed correctly.</p>
    InvalidRequest(String),
    /// <p>A requested resource does not exist on the network. It may have been deleted or referenced inaccurately.</p>
    ResourceNotFound(String),
    /// <p>The requested resource exists but is not in a status that can complete the operation.</p>
    ResourceNotReady(String),
    /// <p>The request or operation could not be performed because a service is throttling requests. The most common source of throttling errors is launching EC2 instances such that your service limit for EC2 instances is exceeded. Request a limit increase or delete unused resources if possible.</p>
    Throttling(String),
}

impl DeleteNodeError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteNodeError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(DeleteNodeError::AccessDenied(err.msg))
                }
                "InternalServiceErrorException" => {
                    return RusotoError::Service(DeleteNodeError::InternalServiceError(err.msg))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(DeleteNodeError::InvalidRequest(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(DeleteNodeError::ResourceNotFound(err.msg))
                }
                "ResourceNotReadyException" => {
                    return RusotoError::Service(DeleteNodeError::ResourceNotReady(err.msg))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(DeleteNodeError::Throttling(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DeleteNodeError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteNodeError::AccessDenied(ref cause) => write!(f, "{}", cause),
            DeleteNodeError::InternalServiceError(ref cause) => write!(f, "{}", cause),
            DeleteNodeError::InvalidRequest(ref cause) => write!(f, "{}", cause),
            DeleteNodeError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            DeleteNodeError::ResourceNotReady(ref cause) => write!(f, "{}", cause),
            DeleteNodeError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeleteNodeError {}
/// Errors returned by GetMember
#[derive(Debug, PartialEq)]
pub enum GetMemberError {
    /// <p>You do not have sufficient access to perform this action.</p>
    AccessDenied(String),
    /// <p>The request processing has failed because of an unknown error, exception or failure.</p>
    InternalServiceError(String),
    /// <p>The action or operation requested is invalid. Verify that the action is typed correctly.</p>
    InvalidRequest(String),
    /// <p>A requested resource does not exist on the network. It may have been deleted or referenced inaccurately.</p>
    ResourceNotFound(String),
    /// <p>The request or operation could not be performed because a service is throttling requests. The most common source of throttling errors is launching EC2 instances such that your service limit for EC2 instances is exceeded. Request a limit increase or delete unused resources if possible.</p>
    Throttling(String),
}

impl GetMemberError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetMemberError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(GetMemberError::AccessDenied(err.msg))
                }
                "InternalServiceErrorException" => {
                    return RusotoError::Service(GetMemberError::InternalServiceError(err.msg))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(GetMemberError::InvalidRequest(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(GetMemberError::ResourceNotFound(err.msg))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(GetMemberError::Throttling(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for GetMemberError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetMemberError::AccessDenied(ref cause) => write!(f, "{}", cause),
            GetMemberError::InternalServiceError(ref cause) => write!(f, "{}", cause),
            GetMemberError::InvalidRequest(ref cause) => write!(f, "{}", cause),
            GetMemberError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            GetMemberError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetMemberError {}
/// Errors returned by GetNetwork
#[derive(Debug, PartialEq)]
pub enum GetNetworkError {
    /// <p>You do not have sufficient access to perform this action.</p>
    AccessDenied(String),
    /// <p>The request processing has failed because of an unknown error, exception or failure.</p>
    InternalServiceError(String),
    /// <p>The action or operation requested is invalid. Verify that the action is typed correctly.</p>
    InvalidRequest(String),
    /// <p>A requested resource does not exist on the network. It may have been deleted or referenced inaccurately.</p>
    ResourceNotFound(String),
    /// <p>The request or operation could not be performed because a service is throttling requests. The most common source of throttling errors is launching EC2 instances such that your service limit for EC2 instances is exceeded. Request a limit increase or delete unused resources if possible.</p>
    Throttling(String),
}

impl GetNetworkError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetNetworkError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(GetNetworkError::AccessDenied(err.msg))
                }
                "InternalServiceErrorException" => {
                    return RusotoError::Service(GetNetworkError::InternalServiceError(err.msg))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(GetNetworkError::InvalidRequest(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(GetNetworkError::ResourceNotFound(err.msg))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(GetNetworkError::Throttling(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for GetNetworkError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetNetworkError::AccessDenied(ref cause) => write!(f, "{}", cause),
            GetNetworkError::InternalServiceError(ref cause) => write!(f, "{}", cause),
            GetNetworkError::InvalidRequest(ref cause) => write!(f, "{}", cause),
            GetNetworkError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            GetNetworkError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetNetworkError {}
/// Errors returned by GetNode
#[derive(Debug, PartialEq)]
pub enum GetNodeError {
    /// <p>You do not have sufficient access to perform this action.</p>
    AccessDenied(String),
    /// <p>The request processing has failed because of an unknown error, exception or failure.</p>
    InternalServiceError(String),
    /// <p>The action or operation requested is invalid. Verify that the action is typed correctly.</p>
    InvalidRequest(String),
    /// <p>A requested resource does not exist on the network. It may have been deleted or referenced inaccurately.</p>
    ResourceNotFound(String),
    /// <p>The request or operation could not be performed because a service is throttling requests. The most common source of throttling errors is launching EC2 instances such that your service limit for EC2 instances is exceeded. Request a limit increase or delete unused resources if possible.</p>
    Throttling(String),
}

impl GetNodeError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetNodeError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(GetNodeError::AccessDenied(err.msg))
                }
                "InternalServiceErrorException" => {
                    return RusotoError::Service(GetNodeError::InternalServiceError(err.msg))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(GetNodeError::InvalidRequest(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(GetNodeError::ResourceNotFound(err.msg))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(GetNodeError::Throttling(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for GetNodeError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetNodeError::AccessDenied(ref cause) => write!(f, "{}", cause),
            GetNodeError::InternalServiceError(ref cause) => write!(f, "{}", cause),
            GetNodeError::InvalidRequest(ref cause) => write!(f, "{}", cause),
            GetNodeError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            GetNodeError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetNodeError {}
/// Errors returned by GetProposal
#[derive(Debug, PartialEq)]
pub enum GetProposalError {
    /// <p>You do not have sufficient access to perform this action.</p>
    AccessDenied(String),
    /// <p>The request processing has failed because of an unknown error, exception or failure.</p>
    InternalServiceError(String),
    /// <p>The action or operation requested is invalid. Verify that the action is typed correctly.</p>
    InvalidRequest(String),
    /// <p>A requested resource does not exist on the network. It may have been deleted or referenced inaccurately.</p>
    ResourceNotFound(String),
    /// <p>The request or operation could not be performed because a service is throttling requests. The most common source of throttling errors is launching EC2 instances such that your service limit for EC2 instances is exceeded. Request a limit increase or delete unused resources if possible.</p>
    Throttling(String),
}

impl GetProposalError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetProposalError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(GetProposalError::AccessDenied(err.msg))
                }
                "InternalServiceErrorException" => {
                    return RusotoError::Service(GetProposalError::InternalServiceError(err.msg))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(GetProposalError::InvalidRequest(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(GetProposalError::ResourceNotFound(err.msg))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(GetProposalError::Throttling(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for GetProposalError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetProposalError::AccessDenied(ref cause) => write!(f, "{}", cause),
            GetProposalError::InternalServiceError(ref cause) => write!(f, "{}", cause),
            GetProposalError::InvalidRequest(ref cause) => write!(f, "{}", cause),
            GetProposalError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            GetProposalError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetProposalError {}
/// Errors returned by ListInvitations
#[derive(Debug, PartialEq)]
pub enum ListInvitationsError {
    /// <p>You do not have sufficient access to perform this action.</p>
    AccessDenied(String),
    /// <p>The request processing has failed because of an unknown error, exception or failure.</p>
    InternalServiceError(String),
    /// <p>The action or operation requested is invalid. Verify that the action is typed correctly.</p>
    InvalidRequest(String),
    /// <p>The maximum number of resources of that type already exist. Ensure the resources requested are within the boundaries of the service edition and your account limits.</p>
    ResourceLimitExceeded(String),
    /// <p>A requested resource does not exist on the network. It may have been deleted or referenced inaccurately.</p>
    ResourceNotFound(String),
    /// <p>The request or operation could not be performed because a service is throttling requests. The most common source of throttling errors is launching EC2 instances such that your service limit for EC2 instances is exceeded. Request a limit increase or delete unused resources if possible.</p>
    Throttling(String),
}

impl ListInvitationsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListInvitationsError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(ListInvitationsError::AccessDenied(err.msg))
                }
                "InternalServiceErrorException" => {
                    return RusotoError::Service(ListInvitationsError::InternalServiceError(
                        err.msg,
                    ))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(ListInvitationsError::InvalidRequest(err.msg))
                }
                "ResourceLimitExceededException" => {
                    return RusotoError::Service(ListInvitationsError::ResourceLimitExceeded(
                        err.msg,
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(ListInvitationsError::ResourceNotFound(err.msg))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(ListInvitationsError::Throttling(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ListInvitationsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListInvitationsError::AccessDenied(ref cause) => write!(f, "{}", cause),
            ListInvitationsError::InternalServiceError(ref cause) => write!(f, "{}", cause),
            ListInvitationsError::InvalidRequest(ref cause) => write!(f, "{}", cause),
            ListInvitationsError::ResourceLimitExceeded(ref cause) => write!(f, "{}", cause),
            ListInvitationsError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            ListInvitationsError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListInvitationsError {}
/// Errors returned by ListMembers
#[derive(Debug, PartialEq)]
pub enum ListMembersError {
    /// <p>You do not have sufficient access to perform this action.</p>
    AccessDenied(String),
    /// <p>The request processing has failed because of an unknown error, exception or failure.</p>
    InternalServiceError(String),
    /// <p>The action or operation requested is invalid. Verify that the action is typed correctly.</p>
    InvalidRequest(String),
    /// <p>The request or operation could not be performed because a service is throttling requests. The most common source of throttling errors is launching EC2 instances such that your service limit for EC2 instances is exceeded. Request a limit increase or delete unused resources if possible.</p>
    Throttling(String),
}

impl ListMembersError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListMembersError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(ListMembersError::AccessDenied(err.msg))
                }
                "InternalServiceErrorException" => {
                    return RusotoError::Service(ListMembersError::InternalServiceError(err.msg))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(ListMembersError::InvalidRequest(err.msg))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(ListMembersError::Throttling(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ListMembersError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListMembersError::AccessDenied(ref cause) => write!(f, "{}", cause),
            ListMembersError::InternalServiceError(ref cause) => write!(f, "{}", cause),
            ListMembersError::InvalidRequest(ref cause) => write!(f, "{}", cause),
            ListMembersError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListMembersError {}
/// Errors returned by ListNetworks
#[derive(Debug, PartialEq)]
pub enum ListNetworksError {
    /// <p>You do not have sufficient access to perform this action.</p>
    AccessDenied(String),
    /// <p>The request processing has failed because of an unknown error, exception or failure.</p>
    InternalServiceError(String),
    /// <p>The action or operation requested is invalid. Verify that the action is typed correctly.</p>
    InvalidRequest(String),
    /// <p>The request or operation could not be performed because a service is throttling requests. The most common source of throttling errors is launching EC2 instances such that your service limit for EC2 instances is exceeded. Request a limit increase or delete unused resources if possible.</p>
    Throttling(String),
}

impl ListNetworksError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListNetworksError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(ListNetworksError::AccessDenied(err.msg))
                }
                "InternalServiceErrorException" => {
                    return RusotoError::Service(ListNetworksError::InternalServiceError(err.msg))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(ListNetworksError::InvalidRequest(err.msg))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(ListNetworksError::Throttling(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ListNetworksError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListNetworksError::AccessDenied(ref cause) => write!(f, "{}", cause),
            ListNetworksError::InternalServiceError(ref cause) => write!(f, "{}", cause),
            ListNetworksError::InvalidRequest(ref cause) => write!(f, "{}", cause),
            ListNetworksError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListNetworksError {}
/// Errors returned by ListNodes
#[derive(Debug, PartialEq)]
pub enum ListNodesError {
    /// <p>You do not have sufficient access to perform this action.</p>
    AccessDenied(String),
    /// <p>The request processing has failed because of an unknown error, exception or failure.</p>
    InternalServiceError(String),
    /// <p>The action or operation requested is invalid. Verify that the action is typed correctly.</p>
    InvalidRequest(String),
    /// <p>The request or operation could not be performed because a service is throttling requests. The most common source of throttling errors is launching EC2 instances such that your service limit for EC2 instances is exceeded. Request a limit increase or delete unused resources if possible.</p>
    Throttling(String),
}

impl ListNodesError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListNodesError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(ListNodesError::AccessDenied(err.msg))
                }
                "InternalServiceErrorException" => {
                    return RusotoError::Service(ListNodesError::InternalServiceError(err.msg))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(ListNodesError::InvalidRequest(err.msg))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(ListNodesError::Throttling(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ListNodesError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListNodesError::AccessDenied(ref cause) => write!(f, "{}", cause),
            ListNodesError::InternalServiceError(ref cause) => write!(f, "{}", cause),
            ListNodesError::InvalidRequest(ref cause) => write!(f, "{}", cause),
            ListNodesError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListNodesError {}
/// Errors returned by ListProposalVotes
#[derive(Debug, PartialEq)]
pub enum ListProposalVotesError {
    /// <p>You do not have sufficient access to perform this action.</p>
    AccessDenied(String),
    /// <p>The request processing has failed because of an unknown error, exception or failure.</p>
    InternalServiceError(String),
    /// <p>The action or operation requested is invalid. Verify that the action is typed correctly.</p>
    InvalidRequest(String),
    /// <p>The request or operation could not be performed because a service is throttling requests. The most common source of throttling errors is launching EC2 instances such that your service limit for EC2 instances is exceeded. Request a limit increase or delete unused resources if possible.</p>
    Throttling(String),
}

impl ListProposalVotesError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListProposalVotesError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(ListProposalVotesError::AccessDenied(err.msg))
                }
                "InternalServiceErrorException" => {
                    return RusotoError::Service(ListProposalVotesError::InternalServiceError(
                        err.msg,
                    ))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(ListProposalVotesError::InvalidRequest(err.msg))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(ListProposalVotesError::Throttling(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ListProposalVotesError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListProposalVotesError::AccessDenied(ref cause) => write!(f, "{}", cause),
            ListProposalVotesError::InternalServiceError(ref cause) => write!(f, "{}", cause),
            ListProposalVotesError::InvalidRequest(ref cause) => write!(f, "{}", cause),
            ListProposalVotesError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListProposalVotesError {}
/// Errors returned by ListProposals
#[derive(Debug, PartialEq)]
pub enum ListProposalsError {
    /// <p>You do not have sufficient access to perform this action.</p>
    AccessDenied(String),
    /// <p>The request processing has failed because of an unknown error, exception or failure.</p>
    InternalServiceError(String),
    /// <p>The action or operation requested is invalid. Verify that the action is typed correctly.</p>
    InvalidRequest(String),
    /// <p>A requested resource does not exist on the network. It may have been deleted or referenced inaccurately.</p>
    ResourceNotFound(String),
    /// <p>The request or operation could not be performed because a service is throttling requests. The most common source of throttling errors is launching EC2 instances such that your service limit for EC2 instances is exceeded. Request a limit increase or delete unused resources if possible.</p>
    Throttling(String),
}

impl ListProposalsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListProposalsError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(ListProposalsError::AccessDenied(err.msg))
                }
                "InternalServiceErrorException" => {
                    return RusotoError::Service(ListProposalsError::InternalServiceError(err.msg))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(ListProposalsError::InvalidRequest(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(ListProposalsError::ResourceNotFound(err.msg))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(ListProposalsError::Throttling(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ListProposalsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListProposalsError::AccessDenied(ref cause) => write!(f, "{}", cause),
            ListProposalsError::InternalServiceError(ref cause) => write!(f, "{}", cause),
            ListProposalsError::InvalidRequest(ref cause) => write!(f, "{}", cause),
            ListProposalsError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            ListProposalsError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListProposalsError {}
/// Errors returned by RejectInvitation
#[derive(Debug, PartialEq)]
pub enum RejectInvitationError {
    /// <p>You do not have sufficient access to perform this action.</p>
    AccessDenied(String),
    /// <p><p/></p>
    IllegalAction(String),
    /// <p>The request processing has failed because of an unknown error, exception or failure.</p>
    InternalServiceError(String),
    /// <p>The action or operation requested is invalid. Verify that the action is typed correctly.</p>
    InvalidRequest(String),
    /// <p>A requested resource does not exist on the network. It may have been deleted or referenced inaccurately.</p>
    ResourceNotFound(String),
    /// <p>The request or operation could not be performed because a service is throttling requests. The most common source of throttling errors is launching EC2 instances such that your service limit for EC2 instances is exceeded. Request a limit increase or delete unused resources if possible.</p>
    Throttling(String),
}

impl RejectInvitationError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<RejectInvitationError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(RejectInvitationError::AccessDenied(err.msg))
                }
                "IllegalActionException" => {
                    return RusotoError::Service(RejectInvitationError::IllegalAction(err.msg))
                }
                "InternalServiceErrorException" => {
                    return RusotoError::Service(RejectInvitationError::InternalServiceError(
                        err.msg,
                    ))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(RejectInvitationError::InvalidRequest(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(RejectInvitationError::ResourceNotFound(err.msg))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(RejectInvitationError::Throttling(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for RejectInvitationError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            RejectInvitationError::AccessDenied(ref cause) => write!(f, "{}", cause),
            RejectInvitationError::IllegalAction(ref cause) => write!(f, "{}", cause),
            RejectInvitationError::InternalServiceError(ref cause) => write!(f, "{}", cause),
            RejectInvitationError::InvalidRequest(ref cause) => write!(f, "{}", cause),
            RejectInvitationError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            RejectInvitationError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for RejectInvitationError {}
/// Errors returned by UpdateMember
#[derive(Debug, PartialEq)]
pub enum UpdateMemberError {
    /// <p>You do not have sufficient access to perform this action.</p>
    AccessDenied(String),
    /// <p>The request processing has failed because of an unknown error, exception or failure.</p>
    InternalServiceError(String),
    /// <p>The action or operation requested is invalid. Verify that the action is typed correctly.</p>
    InvalidRequest(String),
    /// <p>A requested resource does not exist on the network. It may have been deleted or referenced inaccurately.</p>
    ResourceNotFound(String),
    /// <p>The request or operation could not be performed because a service is throttling requests. The most common source of throttling errors is launching EC2 instances such that your service limit for EC2 instances is exceeded. Request a limit increase or delete unused resources if possible.</p>
    Throttling(String),
}

impl UpdateMemberError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdateMemberError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(UpdateMemberError::AccessDenied(err.msg))
                }
                "InternalServiceErrorException" => {
                    return RusotoError::Service(UpdateMemberError::InternalServiceError(err.msg))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(UpdateMemberError::InvalidRequest(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(UpdateMemberError::ResourceNotFound(err.msg))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(UpdateMemberError::Throttling(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for UpdateMemberError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UpdateMemberError::AccessDenied(ref cause) => write!(f, "{}", cause),
            UpdateMemberError::InternalServiceError(ref cause) => write!(f, "{}", cause),
            UpdateMemberError::InvalidRequest(ref cause) => write!(f, "{}", cause),
            UpdateMemberError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            UpdateMemberError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for UpdateMemberError {}
/// Errors returned by UpdateNode
#[derive(Debug, PartialEq)]
pub enum UpdateNodeError {
    /// <p>You do not have sufficient access to perform this action.</p>
    AccessDenied(String),
    /// <p>The request processing has failed because of an unknown error, exception or failure.</p>
    InternalServiceError(String),
    /// <p>The action or operation requested is invalid. Verify that the action is typed correctly.</p>
    InvalidRequest(String),
    /// <p>A requested resource does not exist on the network. It may have been deleted or referenced inaccurately.</p>
    ResourceNotFound(String),
    /// <p>The request or operation could not be performed because a service is throttling requests. The most common source of throttling errors is launching EC2 instances such that your service limit for EC2 instances is exceeded. Request a limit increase or delete unused resources if possible.</p>
    Throttling(String),
}

impl UpdateNodeError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdateNodeError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(UpdateNodeError::AccessDenied(err.msg))
                }
                "InternalServiceErrorException" => {
                    return RusotoError::Service(UpdateNodeError::InternalServiceError(err.msg))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(UpdateNodeError::InvalidRequest(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(UpdateNodeError::ResourceNotFound(err.msg))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(UpdateNodeError::Throttling(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for UpdateNodeError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UpdateNodeError::AccessDenied(ref cause) => write!(f, "{}", cause),
            UpdateNodeError::InternalServiceError(ref cause) => write!(f, "{}", cause),
            UpdateNodeError::InvalidRequest(ref cause) => write!(f, "{}", cause),
            UpdateNodeError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            UpdateNodeError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for UpdateNodeError {}
/// Errors returned by VoteOnProposal
#[derive(Debug, PartialEq)]
pub enum VoteOnProposalError {
    /// <p>You do not have sufficient access to perform this action.</p>
    AccessDenied(String),
    /// <p><p/></p>
    IllegalAction(String),
    /// <p>The request processing has failed because of an unknown error, exception or failure.</p>
    InternalServiceError(String),
    /// <p>The action or operation requested is invalid. Verify that the action is typed correctly.</p>
    InvalidRequest(String),
    /// <p>A requested resource does not exist on the network. It may have been deleted or referenced inaccurately.</p>
    ResourceNotFound(String),
    /// <p>The request or operation could not be performed because a service is throttling requests. The most common source of throttling errors is launching EC2 instances such that your service limit for EC2 instances is exceeded. Request a limit increase or delete unused resources if possible.</p>
    Throttling(String),
}

impl VoteOnProposalError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<VoteOnProposalError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(VoteOnProposalError::AccessDenied(err.msg))
                }
                "IllegalActionException" => {
                    return RusotoError::Service(VoteOnProposalError::IllegalAction(err.msg))
                }
                "InternalServiceErrorException" => {
                    return RusotoError::Service(VoteOnProposalError::InternalServiceError(err.msg))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(VoteOnProposalError::InvalidRequest(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(VoteOnProposalError::ResourceNotFound(err.msg))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(VoteOnProposalError::Throttling(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for VoteOnProposalError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            VoteOnProposalError::AccessDenied(ref cause) => write!(f, "{}", cause),
            VoteOnProposalError::IllegalAction(ref cause) => write!(f, "{}", cause),
            VoteOnProposalError::InternalServiceError(ref cause) => write!(f, "{}", cause),
            VoteOnProposalError::InvalidRequest(ref cause) => write!(f, "{}", cause),
            VoteOnProposalError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            VoteOnProposalError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for VoteOnProposalError {}
/// Trait representing the capabilities of the ManagedBlockchain API. ManagedBlockchain clients implement this trait.
#[async_trait]
pub trait ManagedBlockchain: Clone + Sync + Send + 'static {
    /// <p>Creates a member within a Managed Blockchain network.</p> <p>Applies only to Hyperledger Fabric.</p>
    async fn create_member(
        &self,
        input: CreateMemberInput,
    ) -> Result<CreateMemberOutput, RusotoError<CreateMemberError>>;

    /// <p>Creates a new blockchain network using Amazon Managed Blockchain.</p> <p>Applies only to Hyperledger Fabric.</p>
    async fn create_network(
        &self,
        input: CreateNetworkInput,
    ) -> Result<CreateNetworkOutput, RusotoError<CreateNetworkError>>;

    /// <p>Creates a node on the specified blockchain network.</p> <p>Applies to Hyperledger Fabric and Ethereum.</p>
    async fn create_node(
        &self,
        input: CreateNodeInput,
    ) -> Result<CreateNodeOutput, RusotoError<CreateNodeError>>;

    /// <p>Creates a proposal for a change to the network that other members of the network can vote on, for example, a proposal to add a new member to the network. Any member can create a proposal.</p> <p>Applies only to Hyperledger Fabric.</p>
    async fn create_proposal(
        &self,
        input: CreateProposalInput,
    ) -> Result<CreateProposalOutput, RusotoError<CreateProposalError>>;

    /// <p>Deletes a member. Deleting a member removes the member and all associated resources from the network. <code>DeleteMember</code> can only be called for a specified <code>MemberId</code> if the principal performing the action is associated with the AWS account that owns the member. In all other cases, the <code>DeleteMember</code> action is carried out as the result of an approved proposal to remove a member. If <code>MemberId</code> is the last member in a network specified by the last AWS account, the network is deleted also.</p> <p>Applies only to Hyperledger Fabric.</p>
    async fn delete_member(
        &self,
        input: DeleteMemberInput,
    ) -> Result<DeleteMemberOutput, RusotoError<DeleteMemberError>>;

    /// <p>Deletes a node that your AWS account owns. All data on the node is lost and cannot be recovered.</p> <p>Applies to Hyperledger Fabric and Ethereum.</p>
    async fn delete_node(
        &self,
        input: DeleteNodeInput,
    ) -> Result<DeleteNodeOutput, RusotoError<DeleteNodeError>>;

    /// <p>Returns detailed information about a member.</p> <p>Applies only to Hyperledger Fabric.</p>
    async fn get_member(
        &self,
        input: GetMemberInput,
    ) -> Result<GetMemberOutput, RusotoError<GetMemberError>>;

    /// <p>Returns detailed information about a network.</p> <p>Applies to Hyperledger Fabric and Ethereum.</p>
    async fn get_network(
        &self,
        input: GetNetworkInput,
    ) -> Result<GetNetworkOutput, RusotoError<GetNetworkError>>;

    /// <p>Returns detailed information about a node.</p> <p>Applies to Hyperledger Fabric and Ethereum.</p>
    async fn get_node(
        &self,
        input: GetNodeInput,
    ) -> Result<GetNodeOutput, RusotoError<GetNodeError>>;

    /// <p>Returns detailed information about a proposal.</p> <p>Applies only to Hyperledger Fabric.</p>
    async fn get_proposal(
        &self,
        input: GetProposalInput,
    ) -> Result<GetProposalOutput, RusotoError<GetProposalError>>;

    /// <p>Returns a list of all invitations for the current AWS account.</p> <p>Applies only to Hyperledger Fabric.</p>
    async fn list_invitations(
        &self,
        input: ListInvitationsInput,
    ) -> Result<ListInvitationsOutput, RusotoError<ListInvitationsError>>;

    /// <p>Returns a list of the members in a network and properties of their configurations.</p> <p>Applies only to Hyperledger Fabric.</p>
    async fn list_members(
        &self,
        input: ListMembersInput,
    ) -> Result<ListMembersOutput, RusotoError<ListMembersError>>;

    /// <p>Returns information about the networks in which the current AWS account participates.</p> <p>Applies to Hyperledger Fabric and Ethereum.</p>
    async fn list_networks(
        &self,
        input: ListNetworksInput,
    ) -> Result<ListNetworksOutput, RusotoError<ListNetworksError>>;

    /// <p>Returns information about the nodes within a network.</p> <p>Applies to Hyperledger Fabric and Ethereum.</p>
    async fn list_nodes(
        &self,
        input: ListNodesInput,
    ) -> Result<ListNodesOutput, RusotoError<ListNodesError>>;

    /// <p>Returns the list of votes for a specified proposal, including the value of each vote and the unique identifier of the member that cast the vote.</p> <p>Applies only to Hyperledger Fabric.</p>
    async fn list_proposal_votes(
        &self,
        input: ListProposalVotesInput,
    ) -> Result<ListProposalVotesOutput, RusotoError<ListProposalVotesError>>;

    /// <p>Returns a list of proposals for the network.</p> <p>Applies only to Hyperledger Fabric.</p>
    async fn list_proposals(
        &self,
        input: ListProposalsInput,
    ) -> Result<ListProposalsOutput, RusotoError<ListProposalsError>>;

    /// <p>Rejects an invitation to join a network. This action can be called by a principal in an AWS account that has received an invitation to create a member and join a network.</p> <p>Applies only to Hyperledger Fabric.</p>
    async fn reject_invitation(
        &self,
        input: RejectInvitationInput,
    ) -> Result<RejectInvitationOutput, RusotoError<RejectInvitationError>>;

    /// <p>Updates a member configuration with new parameters.</p> <p>Applies only to Hyperledger Fabric.</p>
    async fn update_member(
        &self,
        input: UpdateMemberInput,
    ) -> Result<UpdateMemberOutput, RusotoError<UpdateMemberError>>;

    /// <p>Updates a node configuration with new parameters.</p> <p>Applies only to Hyperledger Fabric.</p>
    async fn update_node(
        &self,
        input: UpdateNodeInput,
    ) -> Result<UpdateNodeOutput, RusotoError<UpdateNodeError>>;

    /// <p>Casts a vote for a specified <code>ProposalId</code> on behalf of a member. The member to vote as, specified by <code>VoterMemberId</code>, must be in the same AWS account as the principal that calls the action.</p> <p>Applies only to Hyperledger Fabric.</p>
    async fn vote_on_proposal(
        &self,
        input: VoteOnProposalInput,
    ) -> Result<VoteOnProposalOutput, RusotoError<VoteOnProposalError>>;
}
/// A client for the ManagedBlockchain API.
#[derive(Clone)]
pub struct ManagedBlockchainClient {
    client: Client,
    region: region::Region,
}

impl ManagedBlockchainClient {
    /// Creates a client backed by the default tokio event loop.
    ///
    /// The client will use the default credentials provider and tls client.
    pub fn new(region: region::Region) -> ManagedBlockchainClient {
        ManagedBlockchainClient {
            client: Client::shared(),
            region,
        }
    }

    pub fn new_with<P, D>(
        request_dispatcher: D,
        credentials_provider: P,
        region: region::Region,
    ) -> ManagedBlockchainClient
    where
        P: ProvideAwsCredentials + Send + Sync + 'static,
        D: DispatchSignedRequest + Send + Sync + 'static,
    {
        ManagedBlockchainClient {
            client: Client::new_with(credentials_provider, request_dispatcher),
            region,
        }
    }

    pub fn new_with_client(client: Client, region: region::Region) -> ManagedBlockchainClient {
        ManagedBlockchainClient { client, region }
    }
}

#[async_trait]
impl ManagedBlockchain for ManagedBlockchainClient {
    /// <p>Creates a member within a Managed Blockchain network.</p> <p>Applies only to Hyperledger Fabric.</p>
    #[allow(unused_mut)]
    async fn create_member(
        &self,
        input: CreateMemberInput,
    ) -> Result<CreateMemberOutput, RusotoError<CreateMemberError>> {
        let request_uri = format!(
            "/networks/{network_id}/members",
            network_id = input.network_id
        );

        let mut request =
            SignedRequest::new("POST", "managedblockchain", &self.region, &request_uri);
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
                .deserialize::<CreateMemberOutput, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(CreateMemberError::from_response(response))
        }
    }

    /// <p>Creates a new blockchain network using Amazon Managed Blockchain.</p> <p>Applies only to Hyperledger Fabric.</p>
    #[allow(unused_mut)]
    async fn create_network(
        &self,
        input: CreateNetworkInput,
    ) -> Result<CreateNetworkOutput, RusotoError<CreateNetworkError>> {
        let request_uri = "/networks";

        let mut request =
            SignedRequest::new("POST", "managedblockchain", &self.region, &request_uri);
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
                .deserialize::<CreateNetworkOutput, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(CreateNetworkError::from_response(response))
        }
    }

    /// <p>Creates a node on the specified blockchain network.</p> <p>Applies to Hyperledger Fabric and Ethereum.</p>
    #[allow(unused_mut)]
    async fn create_node(
        &self,
        input: CreateNodeInput,
    ) -> Result<CreateNodeOutput, RusotoError<CreateNodeError>> {
        let request_uri = format!(
            "/networks/{network_id}/nodes",
            network_id = input.network_id
        );

        let mut request =
            SignedRequest::new("POST", "managedblockchain", &self.region, &request_uri);
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
                .deserialize::<CreateNodeOutput, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(CreateNodeError::from_response(response))
        }
    }

    /// <p>Creates a proposal for a change to the network that other members of the network can vote on, for example, a proposal to add a new member to the network. Any member can create a proposal.</p> <p>Applies only to Hyperledger Fabric.</p>
    #[allow(unused_mut)]
    async fn create_proposal(
        &self,
        input: CreateProposalInput,
    ) -> Result<CreateProposalOutput, RusotoError<CreateProposalError>> {
        let request_uri = format!(
            "/networks/{network_id}/proposals",
            network_id = input.network_id
        );

        let mut request =
            SignedRequest::new("POST", "managedblockchain", &self.region, &request_uri);
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
                .deserialize::<CreateProposalOutput, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(CreateProposalError::from_response(response))
        }
    }

    /// <p>Deletes a member. Deleting a member removes the member and all associated resources from the network. <code>DeleteMember</code> can only be called for a specified <code>MemberId</code> if the principal performing the action is associated with the AWS account that owns the member. In all other cases, the <code>DeleteMember</code> action is carried out as the result of an approved proposal to remove a member. If <code>MemberId</code> is the last member in a network specified by the last AWS account, the network is deleted also.</p> <p>Applies only to Hyperledger Fabric.</p>
    #[allow(unused_mut)]
    async fn delete_member(
        &self,
        input: DeleteMemberInput,
    ) -> Result<DeleteMemberOutput, RusotoError<DeleteMemberError>> {
        let request_uri = format!(
            "/networks/{network_id}/members/{member_id}",
            member_id = input.member_id,
            network_id = input.network_id
        );

        let mut request =
            SignedRequest::new("DELETE", "managedblockchain", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<DeleteMemberOutput, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(DeleteMemberError::from_response(response))
        }
    }

    /// <p>Deletes a node that your AWS account owns. All data on the node is lost and cannot be recovered.</p> <p>Applies to Hyperledger Fabric and Ethereum.</p>
    #[allow(unused_mut)]
    async fn delete_node(
        &self,
        input: DeleteNodeInput,
    ) -> Result<DeleteNodeOutput, RusotoError<DeleteNodeError>> {
        let request_uri = format!(
            "/networks/{network_id}/nodes/{node_id}",
            network_id = input.network_id,
            node_id = input.node_id
        );

        let mut request =
            SignedRequest::new("DELETE", "managedblockchain", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut params = Params::new();
        if let Some(ref x) = input.member_id {
            params.put("memberId", x);
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
                .deserialize::<DeleteNodeOutput, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(DeleteNodeError::from_response(response))
        }
    }

    /// <p>Returns detailed information about a member.</p> <p>Applies only to Hyperledger Fabric.</p>
    #[allow(unused_mut)]
    async fn get_member(
        &self,
        input: GetMemberInput,
    ) -> Result<GetMemberOutput, RusotoError<GetMemberError>> {
        let request_uri = format!(
            "/networks/{network_id}/members/{member_id}",
            member_id = input.member_id,
            network_id = input.network_id
        );

        let mut request =
            SignedRequest::new("GET", "managedblockchain", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result =
                proto::json::ResponsePayload::new(&response).deserialize::<GetMemberOutput, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(GetMemberError::from_response(response))
        }
    }

    /// <p>Returns detailed information about a network.</p> <p>Applies to Hyperledger Fabric and Ethereum.</p>
    #[allow(unused_mut)]
    async fn get_network(
        &self,
        input: GetNetworkInput,
    ) -> Result<GetNetworkOutput, RusotoError<GetNetworkError>> {
        let request_uri = format!("/networks/{network_id}", network_id = input.network_id);

        let mut request =
            SignedRequest::new("GET", "managedblockchain", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<GetNetworkOutput, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(GetNetworkError::from_response(response))
        }
    }

    /// <p>Returns detailed information about a node.</p> <p>Applies to Hyperledger Fabric and Ethereum.</p>
    #[allow(unused_mut)]
    async fn get_node(
        &self,
        input: GetNodeInput,
    ) -> Result<GetNodeOutput, RusotoError<GetNodeError>> {
        let request_uri = format!(
            "/networks/{network_id}/nodes/{node_id}",
            network_id = input.network_id,
            node_id = input.node_id
        );

        let mut request =
            SignedRequest::new("GET", "managedblockchain", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut params = Params::new();
        if let Some(ref x) = input.member_id {
            params.put("memberId", x);
        }
        request.set_params(params);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result =
                proto::json::ResponsePayload::new(&response).deserialize::<GetNodeOutput, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(GetNodeError::from_response(response))
        }
    }

    /// <p>Returns detailed information about a proposal.</p> <p>Applies only to Hyperledger Fabric.</p>
    #[allow(unused_mut)]
    async fn get_proposal(
        &self,
        input: GetProposalInput,
    ) -> Result<GetProposalOutput, RusotoError<GetProposalError>> {
        let request_uri = format!(
            "/networks/{network_id}/proposals/{proposal_id}",
            network_id = input.network_id,
            proposal_id = input.proposal_id
        );

        let mut request =
            SignedRequest::new("GET", "managedblockchain", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<GetProposalOutput, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(GetProposalError::from_response(response))
        }
    }

    /// <p>Returns a list of all invitations for the current AWS account.</p> <p>Applies only to Hyperledger Fabric.</p>
    #[allow(unused_mut)]
    async fn list_invitations(
        &self,
        input: ListInvitationsInput,
    ) -> Result<ListInvitationsOutput, RusotoError<ListInvitationsError>> {
        let request_uri = "/invitations";

        let mut request =
            SignedRequest::new("GET", "managedblockchain", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut params = Params::new();
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
                .deserialize::<ListInvitationsOutput, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(ListInvitationsError::from_response(response))
        }
    }

    /// <p>Returns a list of the members in a network and properties of their configurations.</p> <p>Applies only to Hyperledger Fabric.</p>
    #[allow(unused_mut)]
    async fn list_members(
        &self,
        input: ListMembersInput,
    ) -> Result<ListMembersOutput, RusotoError<ListMembersError>> {
        let request_uri = format!(
            "/networks/{network_id}/members",
            network_id = input.network_id
        );

        let mut request =
            SignedRequest::new("GET", "managedblockchain", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut params = Params::new();
        if let Some(ref x) = input.is_owned {
            params.put("isOwned", x);
        }
        if let Some(ref x) = input.max_results {
            params.put("maxResults", x);
        }
        if let Some(ref x) = input.name {
            params.put("name", x);
        }
        if let Some(ref x) = input.next_token {
            params.put("nextToken", x);
        }
        if let Some(ref x) = input.status {
            params.put("status", x);
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
                .deserialize::<ListMembersOutput, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(ListMembersError::from_response(response))
        }
    }

    /// <p>Returns information about the networks in which the current AWS account participates.</p> <p>Applies to Hyperledger Fabric and Ethereum.</p>
    #[allow(unused_mut)]
    async fn list_networks(
        &self,
        input: ListNetworksInput,
    ) -> Result<ListNetworksOutput, RusotoError<ListNetworksError>> {
        let request_uri = "/networks";

        let mut request =
            SignedRequest::new("GET", "managedblockchain", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut params = Params::new();
        if let Some(ref x) = input.framework {
            params.put("framework", x);
        }
        if let Some(ref x) = input.max_results {
            params.put("maxResults", x);
        }
        if let Some(ref x) = input.name {
            params.put("name", x);
        }
        if let Some(ref x) = input.next_token {
            params.put("nextToken", x);
        }
        if let Some(ref x) = input.status {
            params.put("status", x);
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
                .deserialize::<ListNetworksOutput, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(ListNetworksError::from_response(response))
        }
    }

    /// <p>Returns information about the nodes within a network.</p> <p>Applies to Hyperledger Fabric and Ethereum.</p>
    #[allow(unused_mut)]
    async fn list_nodes(
        &self,
        input: ListNodesInput,
    ) -> Result<ListNodesOutput, RusotoError<ListNodesError>> {
        let request_uri = format!(
            "/networks/{network_id}/nodes",
            network_id = input.network_id
        );

        let mut request =
            SignedRequest::new("GET", "managedblockchain", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut params = Params::new();
        if let Some(ref x) = input.max_results {
            params.put("maxResults", x);
        }
        if let Some(ref x) = input.member_id {
            params.put("memberId", x);
        }
        if let Some(ref x) = input.next_token {
            params.put("nextToken", x);
        }
        if let Some(ref x) = input.status {
            params.put("status", x);
        }
        request.set_params(params);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result =
                proto::json::ResponsePayload::new(&response).deserialize::<ListNodesOutput, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(ListNodesError::from_response(response))
        }
    }

    /// <p>Returns the list of votes for a specified proposal, including the value of each vote and the unique identifier of the member that cast the vote.</p> <p>Applies only to Hyperledger Fabric.</p>
    #[allow(unused_mut)]
    async fn list_proposal_votes(
        &self,
        input: ListProposalVotesInput,
    ) -> Result<ListProposalVotesOutput, RusotoError<ListProposalVotesError>> {
        let request_uri = format!(
            "/networks/{network_id}/proposals/{proposal_id}/votes",
            network_id = input.network_id,
            proposal_id = input.proposal_id
        );

        let mut request =
            SignedRequest::new("GET", "managedblockchain", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut params = Params::new();
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
                .deserialize::<ListProposalVotesOutput, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(ListProposalVotesError::from_response(response))
        }
    }

    /// <p>Returns a list of proposals for the network.</p> <p>Applies only to Hyperledger Fabric.</p>
    #[allow(unused_mut)]
    async fn list_proposals(
        &self,
        input: ListProposalsInput,
    ) -> Result<ListProposalsOutput, RusotoError<ListProposalsError>> {
        let request_uri = format!(
            "/networks/{network_id}/proposals",
            network_id = input.network_id
        );

        let mut request =
            SignedRequest::new("GET", "managedblockchain", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut params = Params::new();
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
                .deserialize::<ListProposalsOutput, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(ListProposalsError::from_response(response))
        }
    }

    /// <p>Rejects an invitation to join a network. This action can be called by a principal in an AWS account that has received an invitation to create a member and join a network.</p> <p>Applies only to Hyperledger Fabric.</p>
    #[allow(unused_mut)]
    async fn reject_invitation(
        &self,
        input: RejectInvitationInput,
    ) -> Result<RejectInvitationOutput, RusotoError<RejectInvitationError>> {
        let request_uri = format!(
            "/invitations/{invitation_id}",
            invitation_id = input.invitation_id
        );

        let mut request =
            SignedRequest::new("DELETE", "managedblockchain", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<RejectInvitationOutput, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(RejectInvitationError::from_response(response))
        }
    }

    /// <p>Updates a member configuration with new parameters.</p> <p>Applies only to Hyperledger Fabric.</p>
    #[allow(unused_mut)]
    async fn update_member(
        &self,
        input: UpdateMemberInput,
    ) -> Result<UpdateMemberOutput, RusotoError<UpdateMemberError>> {
        let request_uri = format!(
            "/networks/{network_id}/members/{member_id}",
            member_id = input.member_id,
            network_id = input.network_id
        );

        let mut request =
            SignedRequest::new("PATCH", "managedblockchain", &self.region, &request_uri);
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
                .deserialize::<UpdateMemberOutput, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(UpdateMemberError::from_response(response))
        }
    }

    /// <p>Updates a node configuration with new parameters.</p> <p>Applies only to Hyperledger Fabric.</p>
    #[allow(unused_mut)]
    async fn update_node(
        &self,
        input: UpdateNodeInput,
    ) -> Result<UpdateNodeOutput, RusotoError<UpdateNodeError>> {
        let request_uri = format!(
            "/networks/{network_id}/nodes/{node_id}",
            network_id = input.network_id,
            node_id = input.node_id
        );

        let mut request =
            SignedRequest::new("PATCH", "managedblockchain", &self.region, &request_uri);
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
                .deserialize::<UpdateNodeOutput, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(UpdateNodeError::from_response(response))
        }
    }

    /// <p>Casts a vote for a specified <code>ProposalId</code> on behalf of a member. The member to vote as, specified by <code>VoterMemberId</code>, must be in the same AWS account as the principal that calls the action.</p> <p>Applies only to Hyperledger Fabric.</p>
    #[allow(unused_mut)]
    async fn vote_on_proposal(
        &self,
        input: VoteOnProposalInput,
    ) -> Result<VoteOnProposalOutput, RusotoError<VoteOnProposalError>> {
        let request_uri = format!(
            "/networks/{network_id}/proposals/{proposal_id}/votes",
            network_id = input.network_id,
            proposal_id = input.proposal_id
        );

        let mut request =
            SignedRequest::new("POST", "managedblockchain", &self.region, &request_uri);
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
                .deserialize::<VoteOnProposalOutput, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(VoteOnProposalError::from_response(response))
        }
    }
}
