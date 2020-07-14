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
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct AssociateResolverEndpointIpAddressRequest {
    /// <p>Either the IPv4 address that you want to add to a resolver endpoint or a subnet ID. If you specify a subnet ID, Resolver chooses an IP address for you from the available IPs in the specified subnet.</p>
    #[serde(rename = "IpAddress")]
    pub ip_address: IpAddressUpdate,
    /// <p>The ID of the resolver endpoint that you want to associate IP addresses with.</p>
    #[serde(rename = "ResolverEndpointId")]
    pub resolver_endpoint_id: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct AssociateResolverEndpointIpAddressResponse {
    /// <p>The response to an <code>AssociateResolverEndpointIpAddress</code> request.</p>
    #[serde(rename = "ResolverEndpoint")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resolver_endpoint: Option<ResolverEndpoint>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct AssociateResolverRuleRequest {
    /// <p>A name for the association that you're creating between a resolver rule and a VPC.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The ID of the resolver rule that you want to associate with the VPC. To list the existing resolver rules, use <a>ListResolverRules</a>.</p>
    #[serde(rename = "ResolverRuleId")]
    pub resolver_rule_id: String,
    /// <p>The ID of the VPC that you want to associate the resolver rule with.</p>
    #[serde(rename = "VPCId")]
    pub vpc_id: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct AssociateResolverRuleResponse {
    /// <p>Information about the <code>AssociateResolverRule</code> request, including the status of the request.</p>
    #[serde(rename = "ResolverRuleAssociation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resolver_rule_association: Option<ResolverRuleAssociation>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateResolverEndpointRequest {
    /// <p>A unique string that identifies the request and that allows failed requests to be retried without the risk of executing the operation twice. <code>CreatorRequestId</code> can be any unique string, for example, a date/time stamp. </p>
    #[serde(rename = "CreatorRequestId")]
    pub creator_request_id: String,
    /// <p><p>Specify the applicable value:</p> <ul> <li> <p> <code>INBOUND</code>: Resolver forwards DNS queries to the DNS service for a VPC from your network or another VPC</p> </li> <li> <p> <code>OUTBOUND</code>: Resolver forwards DNS queries from the DNS service for a VPC to your network or another VPC</p> </li> </ul></p>
    #[serde(rename = "Direction")]
    pub direction: String,
    /// <p>The subnets and IP addresses in your VPC that you want DNS queries to pass through on the way from your VPCs to your network (for outbound endpoints) or on the way from your network to your VPCs (for inbound resolver endpoints). </p>
    #[serde(rename = "IpAddresses")]
    pub ip_addresses: Vec<IpAddressRequest>,
    /// <p>A friendly name that lets you easily find a configuration in the Resolver dashboard in the Route 53 console.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The ID of one or more security groups that you want to use to control access to this VPC. The security group that you specify must include one or more inbound rules (for inbound resolver endpoints) or outbound rules (for outbound resolver endpoints).</p>
    #[serde(rename = "SecurityGroupIds")]
    pub security_group_ids: Vec<String>,
    /// <p>A list of the tag keys and values that you want to associate with the endpoint.</p>
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateResolverEndpointResponse {
    /// <p>Information about the <code>CreateResolverEndpoint</code> request, including the status of the request.</p>
    #[serde(rename = "ResolverEndpoint")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resolver_endpoint: Option<ResolverEndpoint>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateResolverRuleRequest {
    /// <p>A unique string that identifies the request and that allows failed requests to be retried without the risk of executing the operation twice. <code>CreatorRequestId</code> can be any unique string, for example, a date/time stamp. </p>
    #[serde(rename = "CreatorRequestId")]
    pub creator_request_id: String,
    /// <p>DNS queries for this domain name are forwarded to the IP addresses that you specify in <code>TargetIps</code>. If a query matches multiple resolver rules (example.com and www.example.com), outbound DNS queries are routed using the resolver rule that contains the most specific domain name (www.example.com).</p>
    #[serde(rename = "DomainName")]
    pub domain_name: String,
    /// <p>A friendly name that lets you easily find a rule in the Resolver dashboard in the Route 53 console.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The ID of the outbound resolver endpoint that you want to use to route DNS queries to the IP addresses that you specify in <code>TargetIps</code>.</p>
    #[serde(rename = "ResolverEndpointId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resolver_endpoint_id: Option<String>,
    /// <p>Specify <code>FORWARD</code>. Other resolver rule types aren't supported.</p>
    #[serde(rename = "RuleType")]
    pub rule_type: String,
    /// <p>A list of the tag keys and values that you want to associate with the endpoint.</p>
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
    /// <p>The IPs that you want Resolver to forward DNS queries to. You can specify only IPv4 addresses. Separate IP addresses with a comma.</p>
    #[serde(rename = "TargetIps")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_ips: Option<Vec<TargetAddress>>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateResolverRuleResponse {
    /// <p>Information about the <code>CreateResolverRule</code> request, including the status of the request.</p>
    #[serde(rename = "ResolverRule")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resolver_rule: Option<ResolverRule>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteResolverEndpointRequest {
    /// <p>The ID of the resolver endpoint that you want to delete.</p>
    #[serde(rename = "ResolverEndpointId")]
    pub resolver_endpoint_id: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DeleteResolverEndpointResponse {
    /// <p>Information about the <code>DeleteResolverEndpoint</code> request, including the status of the request.</p>
    #[serde(rename = "ResolverEndpoint")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resolver_endpoint: Option<ResolverEndpoint>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteResolverRuleRequest {
    /// <p>The ID of the resolver rule that you want to delete.</p>
    #[serde(rename = "ResolverRuleId")]
    pub resolver_rule_id: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DeleteResolverRuleResponse {
    /// <p>Information about the <code>DeleteResolverRule</code> request, including the status of the request.</p>
    #[serde(rename = "ResolverRule")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resolver_rule: Option<ResolverRule>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DisassociateResolverEndpointIpAddressRequest {
    /// <p>The IPv4 address that you want to remove from a resolver endpoint.</p>
    #[serde(rename = "IpAddress")]
    pub ip_address: IpAddressUpdate,
    /// <p>The ID of the resolver endpoint that you want to disassociate an IP address from.</p>
    #[serde(rename = "ResolverEndpointId")]
    pub resolver_endpoint_id: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DisassociateResolverEndpointIpAddressResponse {
    /// <p>The response to an <code>DisassociateResolverEndpointIpAddress</code> request.</p>
    #[serde(rename = "ResolverEndpoint")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resolver_endpoint: Option<ResolverEndpoint>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DisassociateResolverRuleRequest {
    /// <p>The ID of the resolver rule that you want to disassociate from the specified VPC.</p>
    #[serde(rename = "ResolverRuleId")]
    pub resolver_rule_id: String,
    /// <p>The ID of the VPC that you want to disassociate the resolver rule from.</p>
    #[serde(rename = "VPCId")]
    pub vpc_id: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DisassociateResolverRuleResponse {
    /// <p>Information about the <code>DisassociateResolverRule</code> request, including the status of the request.</p>
    #[serde(rename = "ResolverRuleAssociation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resolver_rule_association: Option<ResolverRuleAssociation>,
}

/// <p>For <code>List</code> operations, an optional specification to return a subset of objects, such as resolver endpoints or resolver rules.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct Filter {
    /// <p>When you're using a <code>List</code> operation and you want the operation to return a subset of objects, such as resolver endpoints or resolver rules, the name of the parameter that you want to use to filter objects. For example, to list only inbound resolver endpoints, specify <code>Direction</code> for the value of <code>Name</code>.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>When you're using a <code>List</code> operation and you want the operation to return a subset of objects, such as resolver endpoints or resolver rules, the value of the parameter that you want to use to filter objects. For example, to list only inbound resolver endpoints, specify <code>INBOUND</code> for the value of <code>Values</code>.</p>
    #[serde(rename = "Values")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub values: Option<Vec<String>>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetResolverEndpointRequest {
    /// <p>The ID of the resolver endpoint that you want to get information about.</p>
    #[serde(rename = "ResolverEndpointId")]
    pub resolver_endpoint_id: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetResolverEndpointResponse {
    /// <p>Information about the resolver endpoint that you specified in a <code>GetResolverEndpoint</code> request.</p>
    #[serde(rename = "ResolverEndpoint")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resolver_endpoint: Option<ResolverEndpoint>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetResolverRuleAssociationRequest {
    /// <p>The ID of the resolver rule association that you want to get information about.</p>
    #[serde(rename = "ResolverRuleAssociationId")]
    pub resolver_rule_association_id: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetResolverRuleAssociationResponse {
    /// <p>Information about the resolver rule association that you specified in a <code>GetResolverRuleAssociation</code> request.</p>
    #[serde(rename = "ResolverRuleAssociation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resolver_rule_association: Option<ResolverRuleAssociation>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetResolverRulePolicyRequest {
    /// <p>The ID of the resolver rule policy that you want to get information about.</p>
    #[serde(rename = "Arn")]
    pub arn: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetResolverRulePolicyResponse {
    /// <p>Information about the resolver rule policy that you specified in a <code>GetResolverRulePolicy</code> request.</p>
    #[serde(rename = "ResolverRulePolicy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resolver_rule_policy: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetResolverRuleRequest {
    /// <p>The ID of the resolver rule that you want to get information about.</p>
    #[serde(rename = "ResolverRuleId")]
    pub resolver_rule_id: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetResolverRuleResponse {
    /// <p>Information about the resolver rule that you specified in a <code>GetResolverRule</code> request.</p>
    #[serde(rename = "ResolverRule")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resolver_rule: Option<ResolverRule>,
}

/// <p>In an <a>CreateResolverEndpoint</a> request, a subnet and IP address that you want to use for DNS queries.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct IpAddressRequest {
    /// <p>The IP address that you want to use for DNS queries.</p>
    #[serde(rename = "Ip")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip: Option<String>,
    /// <p>The subnet that contains the IP address.</p>
    #[serde(rename = "SubnetId")]
    pub subnet_id: String,
}

/// <p>In the response to a <a>GetResolverEndpoint</a> request, information about the IP addresses that the resolver endpoint uses for DNS queries.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct IpAddressResponse {
    /// <p>The date and time that the IP address was created, in Unix time format and Coordinated Universal Time (UTC).</p>
    #[serde(rename = "CreationTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time: Option<String>,
    /// <p>One IP address that the resolver endpoint uses for DNS queries.</p>
    #[serde(rename = "Ip")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip: Option<String>,
    /// <p>The ID of one IP address.</p>
    #[serde(rename = "IpId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_id: Option<String>,
    /// <p>The date and time that the IP address was last modified, in Unix time format and Coordinated Universal Time (UTC).</p>
    #[serde(rename = "ModificationTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modification_time: Option<String>,
    /// <p>A status code that gives the current status of the request.</p>
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// <p>A message that provides additional information about the status of the request.</p>
    #[serde(rename = "StatusMessage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_message: Option<String>,
    /// <p>The ID of one subnet.</p>
    #[serde(rename = "SubnetId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subnet_id: Option<String>,
}

/// <p>In an <a>UpdateResolverEndpoint</a> request, information about an IP address to update.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct IpAddressUpdate {
    /// <p>The new IP address.</p>
    #[serde(rename = "Ip")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip: Option<String>,
    /// <p> <i>Only when removing an IP address from a resolver endpoint</i>: The ID of the IP address that you want to remove. To get this ID, use <a>GetResolverEndpoint</a>.</p>
    #[serde(rename = "IpId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_id: Option<String>,
    /// <p>The ID of the subnet that includes the IP address that you want to update. To get this ID, use <a>GetResolverEndpoint</a>.</p>
    #[serde(rename = "SubnetId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subnet_id: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListResolverEndpointIpAddressesRequest {
    /// <p>The maximum number of IP addresses that you want to return in the response to a <code>ListResolverEndpointIpAddresses</code> request. If you don't specify a value for <code>MaxResults</code>, Resolver returns up to 100 IP addresses. </p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>For the first <code>ListResolverEndpointIpAddresses</code> request, omit this value.</p> <p>If the specified resolver endpoint has more than <code>MaxResults</code> IP addresses, you can submit another <code>ListResolverEndpointIpAddresses</code> request to get the next group of IP addresses. In the next request, specify the value of <code>NextToken</code> from the previous response. </p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The ID of the resolver endpoint that you want to get IP addresses for.</p>
    #[serde(rename = "ResolverEndpointId")]
    pub resolver_endpoint_id: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListResolverEndpointIpAddressesResponse {
    /// <p>The IP addresses that DNS queries pass through on their way to your network (outbound endpoint) or on the way to Resolver (inbound endpoint).</p>
    #[serde(rename = "IpAddresses")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_addresses: Option<Vec<IpAddressResponse>>,
    /// <p>The value that you specified for <code>MaxResults</code> in the request.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>If the specified endpoint has more than <code>MaxResults</code> IP addresses, you can submit another <code>ListResolverEndpointIpAddresses</code> request to get the next group of IP addresses. In the next request, specify the value of <code>NextToken</code> from the previous response. </p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListResolverEndpointsRequest {
    /// <p><p>An optional specification to return a subset of resolver endpoints, such as all inbound resolver endpoints.</p> <note> <p>If you submit a second or subsequent <code>ListResolverEndpoints</code> request and specify the <code>NextToken</code> parameter, you must use the same values for <code>Filters</code>, if any, as in the previous request.</p> </note></p>
    #[serde(rename = "Filters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<Vec<Filter>>,
    /// <p>The maximum number of resolver endpoints that you want to return in the response to a <code>ListResolverEndpoints</code> request. If you don't specify a value for <code>MaxResults</code>, Resolver returns up to 100 resolver endpoints. </p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>For the first <code>ListResolverEndpoints</code> request, omit this value.</p> <p>If you have more than <code>MaxResults</code> resolver endpoints, you can submit another <code>ListResolverEndpoints</code> request to get the next group of resolver endpoints. In the next request, specify the value of <code>NextToken</code> from the previous response. </p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListResolverEndpointsResponse {
    /// <p>The value that you specified for <code>MaxResults</code> in the request.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>If more than <code>MaxResults</code> IP addresses match the specified criteria, you can submit another <code>ListResolverEndpoint</code> request to get the next group of results. In the next request, specify the value of <code>NextToken</code> from the previous response. </p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The resolver endpoints that were created by using the current AWS account, and that match the specified filters, if any.</p>
    #[serde(rename = "ResolverEndpoints")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resolver_endpoints: Option<Vec<ResolverEndpoint>>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListResolverRuleAssociationsRequest {
    /// <p><p>An optional specification to return a subset of resolver rules, such as resolver rules that are associated with the same VPC ID.</p> <note> <p>If you submit a second or subsequent <code>ListResolverRuleAssociations</code> request and specify the <code>NextToken</code> parameter, you must use the same values for <code>Filters</code>, if any, as in the previous request.</p> </note></p>
    #[serde(rename = "Filters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<Vec<Filter>>,
    /// <p>The maximum number of rule associations that you want to return in the response to a <code>ListResolverRuleAssociations</code> request. If you don't specify a value for <code>MaxResults</code>, Resolver returns up to 100 rule associations. </p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>For the first <code>ListResolverRuleAssociation</code> request, omit this value.</p> <p>If you have more than <code>MaxResults</code> rule associations, you can submit another <code>ListResolverRuleAssociation</code> request to get the next group of rule associations. In the next request, specify the value of <code>NextToken</code> from the previous response. </p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListResolverRuleAssociationsResponse {
    /// <p>The value that you specified for <code>MaxResults</code> in the request.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>If more than <code>MaxResults</code> rule associations match the specified criteria, you can submit another <code>ListResolverRuleAssociation</code> request to get the next group of results. In the next request, specify the value of <code>NextToken</code> from the previous response. </p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The associations that were created between resolver rules and VPCs using the current AWS account, and that match the specified filters, if any.</p>
    #[serde(rename = "ResolverRuleAssociations")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resolver_rule_associations: Option<Vec<ResolverRuleAssociation>>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListResolverRulesRequest {
    /// <p><p>An optional specification to return a subset of resolver rules, such as all resolver rules that are associated with the same resolver endpoint.</p> <note> <p>If you submit a second or subsequent <code>ListResolverRules</code> request and specify the <code>NextToken</code> parameter, you must use the same values for <code>Filters</code>, if any, as in the previous request.</p> </note></p>
    #[serde(rename = "Filters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<Vec<Filter>>,
    /// <p>The maximum number of resolver rules that you want to return in the response to a <code>ListResolverRules</code> request. If you don't specify a value for <code>MaxResults</code>, Resolver returns up to 100 resolver rules.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>For the first <code>ListResolverRules</code> request, omit this value.</p> <p>If you have more than <code>MaxResults</code> resolver rules, you can submit another <code>ListResolverRules</code> request to get the next group of resolver rules. In the next request, specify the value of <code>NextToken</code> from the previous response. </p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListResolverRulesResponse {
    /// <p>The value that you specified for <code>MaxResults</code> in the request.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>If more than <code>MaxResults</code> resolver rules match the specified criteria, you can submit another <code>ListResolverRules</code> request to get the next group of results. In the next request, specify the value of <code>NextToken</code> from the previous response. </p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The resolver rules that were created using the current AWS account and that match the specified filters, if any.</p>
    #[serde(rename = "ResolverRules")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resolver_rules: Option<Vec<ResolverRule>>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListTagsForResourceRequest {
    /// <p>The maximum number of tags that you want to return in the response to a <code>ListTagsForResource</code> request. If you don't specify a value for <code>MaxResults</code>, Resolver returns up to 100 tags.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>For the first <code>ListTagsForResource</code> request, omit this value.</p> <p>If you have more than <code>MaxResults</code> tags, you can submit another <code>ListTagsForResource</code> request to get the next group of tags for the resource. In the next request, specify the value of <code>NextToken</code> from the previous response. </p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The Amazon Resource Name (ARN) for the resource that you want to list tags for.</p>
    #[serde(rename = "ResourceArn")]
    pub resource_arn: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListTagsForResourceResponse {
    /// <p>If more than <code>MaxResults</code> tags match the specified criteria, you can submit another <code>ListTagsForResource</code> request to get the next group of results. In the next request, specify the value of <code>NextToken</code> from the previous response. </p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The tags that are associated with the resource that you specified in the <code>ListTagsForResource</code> request.</p>
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct PutResolverRulePolicyRequest {
    /// <p>The Amazon Resource Name (ARN) of the account that you want to grant permissions to.</p>
    #[serde(rename = "Arn")]
    pub arn: String,
    /// <p>An AWS Identity and Access Management policy statement that lists the permissions that you want to grant to another AWS account.</p>
    #[serde(rename = "ResolverRulePolicy")]
    pub resolver_rule_policy: String,
}

/// <p>The response to a <code>PutResolverRulePolicy</code> request.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct PutResolverRulePolicyResponse {
    /// <p>Whether the <code>PutResolverRulePolicy</code> request was successful.</p>
    #[serde(rename = "ReturnValue")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub return_value: Option<bool>,
}

/// <p>In the response to a <a>CreateResolverEndpoint</a>, <a>DeleteResolverEndpoint</a>, <a>GetResolverEndpoint</a>, <a>ListResolverEndpoints</a>, or <a>UpdateResolverEndpoint</a> request, a complex type that contains settings for an existing inbound or outbound resolver endpoint.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ResolverEndpoint {
    /// <p>The ARN (Amazon Resource Name) for the resolver endpoint.</p>
    #[serde(rename = "Arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p>The date and time that the endpoint was created, in Unix time format and Coordinated Universal Time (UTC).</p>
    #[serde(rename = "CreationTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time: Option<String>,
    /// <p>A unique string that identifies the request that created the resolver endpoint. The <code>CreatorRequestId</code> allows failed requests to be retried without the risk of executing the operation twice.</p>
    #[serde(rename = "CreatorRequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creator_request_id: Option<String>,
    /// <p><p>Indicates whether the resolver endpoint allows inbound or outbound DNS queries:</p> <ul> <li> <p> <code>INBOUND</code>: allows DNS queries to your VPC from your network or another VPC</p> </li> <li> <p> <code>OUTBOUND</code>: allows DNS queries from your VPC to your network or another VPC</p> </li> </ul></p>
    #[serde(rename = "Direction")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub direction: Option<String>,
    /// <p>The ID of the VPC that you want to create the resolver endpoint in.</p>
    #[serde(rename = "HostVPCId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub host_vpc_id: Option<String>,
    /// <p>The ID of the resolver endpoint.</p>
    #[serde(rename = "Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// <p>The number of IP addresses that the resolver endpoint can use for DNS queries.</p>
    #[serde(rename = "IpAddressCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_address_count: Option<i64>,
    /// <p>The date and time that the endpoint was last modified, in Unix time format and Coordinated Universal Time (UTC).</p>
    #[serde(rename = "ModificationTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modification_time: Option<String>,
    /// <p>The name that you assigned to the resolver endpoint when you submitted a <a>CreateResolverEndpoint</a> request.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The ID of one or more security groups that control access to this VPC. The security group must include one or more inbound resolver rules.</p>
    #[serde(rename = "SecurityGroupIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_group_ids: Option<Vec<String>>,
    /// <p>A code that specifies the current status of the resolver endpoint.</p>
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// <p>A detailed description of the status of the resolver endpoint.</p>
    #[serde(rename = "StatusMessage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_message: Option<String>,
}

/// <p>For queries that originate in your VPC, detailed information about a resolver rule, which specifies how to route DNS queries out of the VPC. The <code>ResolverRule</code> parameter appears in the response to a <a>CreateResolverRule</a>, <a>DeleteResolverRule</a>, <a>GetResolverRule</a>, <a>ListResolverRules</a>, or <a>UpdateResolverRule</a> request.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ResolverRule {
    /// <p>The ARN (Amazon Resource Name) for the resolver rule specified by <code>Id</code>.</p>
    #[serde(rename = "Arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p>A unique string that you specified when you created the resolver rule. <code>CreatorRequestId</code>identifies the request and allows failed requests to be retried without the risk of executing the operation twice. </p>
    #[serde(rename = "CreatorRequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creator_request_id: Option<String>,
    /// <p>DNS queries for this domain name are forwarded to the IP addresses that are specified in <code>TargetIps</code>. If a query matches multiple resolver rules (example.com and www.example.com), the query is routed using the resolver rule that contains the most specific domain name (www.example.com).</p>
    #[serde(rename = "DomainName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_name: Option<String>,
    /// <p>The ID that Resolver assigned to the resolver rule when you created it.</p>
    #[serde(rename = "Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// <p>The name for the resolver rule, which you specified when you created the resolver rule.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>When a rule is shared with another AWS account, the account ID of the account that the rule is shared with.</p>
    #[serde(rename = "OwnerId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner_id: Option<String>,
    /// <p>The ID of the endpoint that the rule is associated with.</p>
    #[serde(rename = "ResolverEndpointId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resolver_endpoint_id: Option<String>,
    /// <p>This value is always <code>FORWARD</code>. Other resolver rule types aren't supported.</p>
    #[serde(rename = "RuleType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rule_type: Option<String>,
    /// <p>Whether the rules is shared and, if so, whether the current account is sharing the rule with another account, or another account is sharing the rule with the current account.</p>
    #[serde(rename = "ShareStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub share_status: Option<String>,
    /// <p>A code that specifies the current status of the resolver rule.</p>
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// <p>A detailed description of the status of a resolver rule.</p>
    #[serde(rename = "StatusMessage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_message: Option<String>,
    /// <p>An array that contains the IP addresses and ports that you want to forward </p>
    #[serde(rename = "TargetIps")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_ips: Option<Vec<TargetAddress>>,
}

/// <p>In the response to an <a>AssociateResolverRule</a>, <a>DisassociateResolverRule</a>, or <a>ListResolverRuleAssociations</a> request, information about an association between a resolver rule and a VPC.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ResolverRuleAssociation {
    /// <p>The ID of the association between a resolver rule and a VPC. Resolver assigns this value when you submit an <a>AssociateResolverRule</a> request.</p>
    #[serde(rename = "Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// <p>The name of an association between a resolver rule and a VPC.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The ID of the resolver rule that you associated with the VPC that is specified by <code>VPCId</code>.</p>
    #[serde(rename = "ResolverRuleId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resolver_rule_id: Option<String>,
    /// <p>A code that specifies the current status of the association between a resolver rule and a VPC.</p>
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// <p>A detailed description of the status of the association between a resolver rule and a VPC.</p>
    #[serde(rename = "StatusMessage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_message: Option<String>,
    /// <p>The ID of the VPC that you associated the resolver rule with.</p>
    #[serde(rename = "VPCId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_id: Option<String>,
}

/// <p>In an <a>UpdateResolverRule</a> request, information about the changes that you want to make.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ResolverRuleConfig {
    /// <p>The new name for the resolver rule. The name that you specify appears in the Resolver dashboard in the Route 53 console. </p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The ID of the new outbound resolver endpoint that you want to use to route DNS queries to the IP addresses that you specify in <code>TargetIps</code>.</p>
    #[serde(rename = "ResolverEndpointId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resolver_endpoint_id: Option<String>,
    /// <p>For DNS queries that originate in your VPC, the new IP addresses that you want to route outbound DNS queries to.</p>
    #[serde(rename = "TargetIps")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_ips: Option<Vec<TargetAddress>>,
}

/// <p>One tag that you want to add to the specified resource. A tag consists of a <code>Key</code> (a name for the tag) and a <code>Value</code>.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct Tag {
    /// <p>The name for the tag. For example, if you want to associate Resolver resources with the account IDs of your customers for billing purposes, the value of <code>Key</code> might be <code>account-id</code>.</p>
    #[serde(rename = "Key")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    /// <p>The value for the tag. For example, if <code>Key</code> is <code>account-id</code>, then <code>Value</code> might be the ID of the customer account that you're creating the resource for.</p>
    #[serde(rename = "Value")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct TagResourceRequest {
    /// <p><p>The Amazon Resource Name (ARN) for the resource that you want to add tags to. To get the ARN for a resource, use the applicable <code>Get</code> or <code>List</code> command: </p> <ul> <li> <p> <a>GetResolverEndpoint</a> </p> </li> <li> <p> <a>GetResolverRule</a> </p> </li> <li> <p> <a>GetResolverRuleAssociation</a> </p> </li> <li> <p> <a>ListResolverEndpoints</a> </p> </li> <li> <p> <a>ListResolverRuleAssociations</a> </p> </li> <li> <p> <a>ListResolverRules</a> </p> </li> </ul></p>
    #[serde(rename = "ResourceArn")]
    pub resource_arn: String,
    /// <p>The tags that you want to add to the specified resource.</p>
    #[serde(rename = "Tags")]
    pub tags: Vec<Tag>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct TagResourceResponse {}

/// <p>In a <a>CreateResolverRule</a> request, an array of the IPs that you want to forward DNS queries to.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct TargetAddress {
    /// <p>One IP address that you want to forward DNS queries to. You can specify only IPv4 addresses.</p>
    #[serde(rename = "Ip")]
    pub ip: String,
    /// <p>The port at <code>Ip</code> that you want to forward DNS queries to.</p>
    #[serde(rename = "Port")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub port: Option<i64>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UntagResourceRequest {
    /// <p><p>The Amazon Resource Name (ARN) for the resource that you want to remove tags from. To get the ARN for a resource, use the applicable <code>Get</code> or <code>List</code> command: </p> <ul> <li> <p> <a>GetResolverEndpoint</a> </p> </li> <li> <p> <a>GetResolverRule</a> </p> </li> <li> <p> <a>GetResolverRuleAssociation</a> </p> </li> <li> <p> <a>ListResolverEndpoints</a> </p> </li> <li> <p> <a>ListResolverRuleAssociations</a> </p> </li> <li> <p> <a>ListResolverRules</a> </p> </li> </ul></p>
    #[serde(rename = "ResourceArn")]
    pub resource_arn: String,
    /// <p>The tags that you want to remove to the specified resource.</p>
    #[serde(rename = "TagKeys")]
    pub tag_keys: Vec<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UntagResourceResponse {}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UpdateResolverEndpointRequest {
    /// <p>The name of the resolver endpoint that you want to update.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The ID of the resolver endpoint that you want to update.</p>
    #[serde(rename = "ResolverEndpointId")]
    pub resolver_endpoint_id: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UpdateResolverEndpointResponse {
    /// <p>The response to an <code>UpdateResolverEndpoint</code> request.</p>
    #[serde(rename = "ResolverEndpoint")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resolver_endpoint: Option<ResolverEndpoint>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UpdateResolverRuleRequest {
    /// <p>The new settings for the resolver rule.</p>
    #[serde(rename = "Config")]
    pub config: ResolverRuleConfig,
    /// <p>The ID of the resolver rule that you want to update.</p>
    #[serde(rename = "ResolverRuleId")]
    pub resolver_rule_id: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UpdateResolverRuleResponse {
    /// <p>The response to an <code>UpdateResolverRule</code> request.</p>
    #[serde(rename = "ResolverRule")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resolver_rule: Option<ResolverRule>,
}

/// Errors returned by AssociateResolverEndpointIpAddress
#[derive(Debug, PartialEq)]
pub enum AssociateResolverEndpointIpAddressError {
    /// <p>We encountered an unknown error. Try again in a few minutes.</p>
    InternalServiceError(String),
    /// <p>One or more parameters in this request are not valid.</p>
    InvalidParameter(String),
    /// <p>The request is invalid.</p>
    InvalidRequest(String),
    /// <p>The request caused one or more limits to be exceeded.</p>
    LimitExceeded(String),
    /// <p>The resource that you tried to create already exists.</p>
    ResourceExists(String),
    /// <p>The specified resource doesn't exist.</p>
    ResourceNotFound(String),
    /// <p>The request was throttled. Try again in a few minutes.</p>
    Throttling(String),
}

impl AssociateResolverEndpointIpAddressError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<AssociateResolverEndpointIpAddressError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServiceErrorException" => {
                    return RusotoError::Service(
                        AssociateResolverEndpointIpAddressError::InternalServiceError(err.msg),
                    )
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(
                        AssociateResolverEndpointIpAddressError::InvalidParameter(err.msg),
                    )
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(
                        AssociateResolverEndpointIpAddressError::InvalidRequest(err.msg),
                    )
                }
                "LimitExceededException" => {
                    return RusotoError::Service(
                        AssociateResolverEndpointIpAddressError::LimitExceeded(err.msg),
                    )
                }
                "ResourceExistsException" => {
                    return RusotoError::Service(
                        AssociateResolverEndpointIpAddressError::ResourceExists(err.msg),
                    )
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(
                        AssociateResolverEndpointIpAddressError::ResourceNotFound(err.msg),
                    )
                }
                "ThrottlingException" => {
                    return RusotoError::Service(
                        AssociateResolverEndpointIpAddressError::Throttling(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for AssociateResolverEndpointIpAddressError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            AssociateResolverEndpointIpAddressError::InternalServiceError(ref cause) => {
                write!(f, "{}", cause)
            }
            AssociateResolverEndpointIpAddressError::InvalidParameter(ref cause) => {
                write!(f, "{}", cause)
            }
            AssociateResolverEndpointIpAddressError::InvalidRequest(ref cause) => {
                write!(f, "{}", cause)
            }
            AssociateResolverEndpointIpAddressError::LimitExceeded(ref cause) => {
                write!(f, "{}", cause)
            }
            AssociateResolverEndpointIpAddressError::ResourceExists(ref cause) => {
                write!(f, "{}", cause)
            }
            AssociateResolverEndpointIpAddressError::ResourceNotFound(ref cause) => {
                write!(f, "{}", cause)
            }
            AssociateResolverEndpointIpAddressError::Throttling(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for AssociateResolverEndpointIpAddressError {}
/// Errors returned by AssociateResolverRule
#[derive(Debug, PartialEq)]
pub enum AssociateResolverRuleError {
    /// <p>We encountered an unknown error. Try again in a few minutes.</p>
    InternalServiceError(String),
    /// <p>One or more parameters in this request are not valid.</p>
    InvalidParameter(String),
    /// <p>The request is invalid.</p>
    InvalidRequest(String),
    /// <p>The resource that you tried to create already exists.</p>
    ResourceExists(String),
    /// <p>The specified resource doesn't exist.</p>
    ResourceNotFound(String),
    /// <p>The specified resource isn't available.</p>
    ResourceUnavailable(String),
    /// <p>The request was throttled. Try again in a few minutes.</p>
    Throttling(String),
}

impl AssociateResolverRuleError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<AssociateResolverRuleError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServiceErrorException" => {
                    return RusotoError::Service(AssociateResolverRuleError::InternalServiceError(
                        err.msg,
                    ))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(AssociateResolverRuleError::InvalidParameter(
                        err.msg,
                    ))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(AssociateResolverRuleError::InvalidRequest(
                        err.msg,
                    ))
                }
                "ResourceExistsException" => {
                    return RusotoError::Service(AssociateResolverRuleError::ResourceExists(
                        err.msg,
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(AssociateResolverRuleError::ResourceNotFound(
                        err.msg,
                    ))
                }
                "ResourceUnavailableException" => {
                    return RusotoError::Service(AssociateResolverRuleError::ResourceUnavailable(
                        err.msg,
                    ))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(AssociateResolverRuleError::Throttling(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for AssociateResolverRuleError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            AssociateResolverRuleError::InternalServiceError(ref cause) => write!(f, "{}", cause),
            AssociateResolverRuleError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            AssociateResolverRuleError::InvalidRequest(ref cause) => write!(f, "{}", cause),
            AssociateResolverRuleError::ResourceExists(ref cause) => write!(f, "{}", cause),
            AssociateResolverRuleError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            AssociateResolverRuleError::ResourceUnavailable(ref cause) => write!(f, "{}", cause),
            AssociateResolverRuleError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for AssociateResolverRuleError {}
/// Errors returned by CreateResolverEndpoint
#[derive(Debug, PartialEq)]
pub enum CreateResolverEndpointError {
    /// <p>We encountered an unknown error. Try again in a few minutes.</p>
    InternalServiceError(String),
    /// <p>One or more parameters in this request are not valid.</p>
    InvalidParameter(String),
    /// <p>The request is invalid.</p>
    InvalidRequest(String),
    /// <p>The request caused one or more limits to be exceeded.</p>
    LimitExceeded(String),
    /// <p>The resource that you tried to create already exists.</p>
    ResourceExists(String),
    /// <p>The specified resource doesn't exist.</p>
    ResourceNotFound(String),
    /// <p>The request was throttled. Try again in a few minutes.</p>
    Throttling(String),
}

impl CreateResolverEndpointError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateResolverEndpointError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServiceErrorException" => {
                    return RusotoError::Service(CreateResolverEndpointError::InternalServiceError(
                        err.msg,
                    ))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(CreateResolverEndpointError::InvalidParameter(
                        err.msg,
                    ))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(CreateResolverEndpointError::InvalidRequest(
                        err.msg,
                    ))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(CreateResolverEndpointError::LimitExceeded(
                        err.msg,
                    ))
                }
                "ResourceExistsException" => {
                    return RusotoError::Service(CreateResolverEndpointError::ResourceExists(
                        err.msg,
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(CreateResolverEndpointError::ResourceNotFound(
                        err.msg,
                    ))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(CreateResolverEndpointError::Throttling(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for CreateResolverEndpointError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreateResolverEndpointError::InternalServiceError(ref cause) => write!(f, "{}", cause),
            CreateResolverEndpointError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            CreateResolverEndpointError::InvalidRequest(ref cause) => write!(f, "{}", cause),
            CreateResolverEndpointError::LimitExceeded(ref cause) => write!(f, "{}", cause),
            CreateResolverEndpointError::ResourceExists(ref cause) => write!(f, "{}", cause),
            CreateResolverEndpointError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            CreateResolverEndpointError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CreateResolverEndpointError {}
/// Errors returned by CreateResolverRule
#[derive(Debug, PartialEq)]
pub enum CreateResolverRuleError {
    /// <p>We encountered an unknown error. Try again in a few minutes.</p>
    InternalServiceError(String),
    /// <p>One or more parameters in this request are not valid.</p>
    InvalidParameter(String),
    /// <p>The request is invalid.</p>
    InvalidRequest(String),
    /// <p>The request caused one or more limits to be exceeded.</p>
    LimitExceeded(String),
    /// <p>The resource that you tried to create already exists.</p>
    ResourceExists(String),
    /// <p>The specified resource doesn't exist.</p>
    ResourceNotFound(String),
    /// <p>The specified resource isn't available.</p>
    ResourceUnavailable(String),
    /// <p>The request was throttled. Try again in a few minutes.</p>
    Throttling(String),
}

impl CreateResolverRuleError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateResolverRuleError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServiceErrorException" => {
                    return RusotoError::Service(CreateResolverRuleError::InternalServiceError(
                        err.msg,
                    ))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(CreateResolverRuleError::InvalidParameter(err.msg))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(CreateResolverRuleError::InvalidRequest(err.msg))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(CreateResolverRuleError::LimitExceeded(err.msg))
                }
                "ResourceExistsException" => {
                    return RusotoError::Service(CreateResolverRuleError::ResourceExists(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(CreateResolverRuleError::ResourceNotFound(err.msg))
                }
                "ResourceUnavailableException" => {
                    return RusotoError::Service(CreateResolverRuleError::ResourceUnavailable(
                        err.msg,
                    ))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(CreateResolverRuleError::Throttling(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for CreateResolverRuleError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreateResolverRuleError::InternalServiceError(ref cause) => write!(f, "{}", cause),
            CreateResolverRuleError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            CreateResolverRuleError::InvalidRequest(ref cause) => write!(f, "{}", cause),
            CreateResolverRuleError::LimitExceeded(ref cause) => write!(f, "{}", cause),
            CreateResolverRuleError::ResourceExists(ref cause) => write!(f, "{}", cause),
            CreateResolverRuleError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            CreateResolverRuleError::ResourceUnavailable(ref cause) => write!(f, "{}", cause),
            CreateResolverRuleError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CreateResolverRuleError {}
/// Errors returned by DeleteResolverEndpoint
#[derive(Debug, PartialEq)]
pub enum DeleteResolverEndpointError {
    /// <p>We encountered an unknown error. Try again in a few minutes.</p>
    InternalServiceError(String),
    /// <p>One or more parameters in this request are not valid.</p>
    InvalidParameter(String),
    /// <p>The request is invalid.</p>
    InvalidRequest(String),
    /// <p>The specified resource doesn't exist.</p>
    ResourceNotFound(String),
    /// <p>The request was throttled. Try again in a few minutes.</p>
    Throttling(String),
}

impl DeleteResolverEndpointError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteResolverEndpointError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServiceErrorException" => {
                    return RusotoError::Service(DeleteResolverEndpointError::InternalServiceError(
                        err.msg,
                    ))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(DeleteResolverEndpointError::InvalidParameter(
                        err.msg,
                    ))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(DeleteResolverEndpointError::InvalidRequest(
                        err.msg,
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(DeleteResolverEndpointError::ResourceNotFound(
                        err.msg,
                    ))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(DeleteResolverEndpointError::Throttling(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DeleteResolverEndpointError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteResolverEndpointError::InternalServiceError(ref cause) => write!(f, "{}", cause),
            DeleteResolverEndpointError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            DeleteResolverEndpointError::InvalidRequest(ref cause) => write!(f, "{}", cause),
            DeleteResolverEndpointError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            DeleteResolverEndpointError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeleteResolverEndpointError {}
/// Errors returned by DeleteResolverRule
#[derive(Debug, PartialEq)]
pub enum DeleteResolverRuleError {
    /// <p>We encountered an unknown error. Try again in a few minutes.</p>
    InternalServiceError(String),
    /// <p>One or more parameters in this request are not valid.</p>
    InvalidParameter(String),
    /// <p>The resource that you tried to update or delete is currently in use.</p>
    ResourceInUse(String),
    /// <p>The specified resource doesn't exist.</p>
    ResourceNotFound(String),
    /// <p>The request was throttled. Try again in a few minutes.</p>
    Throttling(String),
}

impl DeleteResolverRuleError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteResolverRuleError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServiceErrorException" => {
                    return RusotoError::Service(DeleteResolverRuleError::InternalServiceError(
                        err.msg,
                    ))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(DeleteResolverRuleError::InvalidParameter(err.msg))
                }
                "ResourceInUseException" => {
                    return RusotoError::Service(DeleteResolverRuleError::ResourceInUse(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(DeleteResolverRuleError::ResourceNotFound(err.msg))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(DeleteResolverRuleError::Throttling(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DeleteResolverRuleError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteResolverRuleError::InternalServiceError(ref cause) => write!(f, "{}", cause),
            DeleteResolverRuleError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            DeleteResolverRuleError::ResourceInUse(ref cause) => write!(f, "{}", cause),
            DeleteResolverRuleError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            DeleteResolverRuleError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeleteResolverRuleError {}
/// Errors returned by DisassociateResolverEndpointIpAddress
#[derive(Debug, PartialEq)]
pub enum DisassociateResolverEndpointIpAddressError {
    /// <p>We encountered an unknown error. Try again in a few minutes.</p>
    InternalServiceError(String),
    /// <p>One or more parameters in this request are not valid.</p>
    InvalidParameter(String),
    /// <p>The request is invalid.</p>
    InvalidRequest(String),
    /// <p>The resource that you tried to create already exists.</p>
    ResourceExists(String),
    /// <p>The specified resource doesn't exist.</p>
    ResourceNotFound(String),
    /// <p>The request was throttled. Try again in a few minutes.</p>
    Throttling(String),
}

impl DisassociateResolverEndpointIpAddressError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<DisassociateResolverEndpointIpAddressError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServiceErrorException" => {
                    return RusotoError::Service(
                        DisassociateResolverEndpointIpAddressError::InternalServiceError(err.msg),
                    )
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(
                        DisassociateResolverEndpointIpAddressError::InvalidParameter(err.msg),
                    )
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(
                        DisassociateResolverEndpointIpAddressError::InvalidRequest(err.msg),
                    )
                }
                "ResourceExistsException" => {
                    return RusotoError::Service(
                        DisassociateResolverEndpointIpAddressError::ResourceExists(err.msg),
                    )
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(
                        DisassociateResolverEndpointIpAddressError::ResourceNotFound(err.msg),
                    )
                }
                "ThrottlingException" => {
                    return RusotoError::Service(
                        DisassociateResolverEndpointIpAddressError::Throttling(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DisassociateResolverEndpointIpAddressError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DisassociateResolverEndpointIpAddressError::InternalServiceError(ref cause) => {
                write!(f, "{}", cause)
            }
            DisassociateResolverEndpointIpAddressError::InvalidParameter(ref cause) => {
                write!(f, "{}", cause)
            }
            DisassociateResolverEndpointIpAddressError::InvalidRequest(ref cause) => {
                write!(f, "{}", cause)
            }
            DisassociateResolverEndpointIpAddressError::ResourceExists(ref cause) => {
                write!(f, "{}", cause)
            }
            DisassociateResolverEndpointIpAddressError::ResourceNotFound(ref cause) => {
                write!(f, "{}", cause)
            }
            DisassociateResolverEndpointIpAddressError::Throttling(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for DisassociateResolverEndpointIpAddressError {}
/// Errors returned by DisassociateResolverRule
#[derive(Debug, PartialEq)]
pub enum DisassociateResolverRuleError {
    /// <p>We encountered an unknown error. Try again in a few minutes.</p>
    InternalServiceError(String),
    /// <p>One or more parameters in this request are not valid.</p>
    InvalidParameter(String),
    /// <p>The specified resource doesn't exist.</p>
    ResourceNotFound(String),
    /// <p>The request was throttled. Try again in a few minutes.</p>
    Throttling(String),
}

impl DisassociateResolverRuleError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DisassociateResolverRuleError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServiceErrorException" => {
                    return RusotoError::Service(
                        DisassociateResolverRuleError::InternalServiceError(err.msg),
                    )
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(DisassociateResolverRuleError::InvalidParameter(
                        err.msg,
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(DisassociateResolverRuleError::ResourceNotFound(
                        err.msg,
                    ))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(DisassociateResolverRuleError::Throttling(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DisassociateResolverRuleError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DisassociateResolverRuleError::InternalServiceError(ref cause) => {
                write!(f, "{}", cause)
            }
            DisassociateResolverRuleError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            DisassociateResolverRuleError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            DisassociateResolverRuleError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DisassociateResolverRuleError {}
/// Errors returned by GetResolverEndpoint
#[derive(Debug, PartialEq)]
pub enum GetResolverEndpointError {
    /// <p>We encountered an unknown error. Try again in a few minutes.</p>
    InternalServiceError(String),
    /// <p>One or more parameters in this request are not valid.</p>
    InvalidParameter(String),
    /// <p>The specified resource doesn't exist.</p>
    ResourceNotFound(String),
    /// <p>The request was throttled. Try again in a few minutes.</p>
    Throttling(String),
}

impl GetResolverEndpointError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetResolverEndpointError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServiceErrorException" => {
                    return RusotoError::Service(GetResolverEndpointError::InternalServiceError(
                        err.msg,
                    ))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(GetResolverEndpointError::InvalidParameter(
                        err.msg,
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(GetResolverEndpointError::ResourceNotFound(
                        err.msg,
                    ))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(GetResolverEndpointError::Throttling(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for GetResolverEndpointError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetResolverEndpointError::InternalServiceError(ref cause) => write!(f, "{}", cause),
            GetResolverEndpointError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            GetResolverEndpointError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            GetResolverEndpointError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetResolverEndpointError {}
/// Errors returned by GetResolverRule
#[derive(Debug, PartialEq)]
pub enum GetResolverRuleError {
    /// <p>We encountered an unknown error. Try again in a few minutes.</p>
    InternalServiceError(String),
    /// <p>One or more parameters in this request are not valid.</p>
    InvalidParameter(String),
    /// <p>The specified resource doesn't exist.</p>
    ResourceNotFound(String),
    /// <p>The request was throttled. Try again in a few minutes.</p>
    Throttling(String),
}

impl GetResolverRuleError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetResolverRuleError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServiceErrorException" => {
                    return RusotoError::Service(GetResolverRuleError::InternalServiceError(
                        err.msg,
                    ))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(GetResolverRuleError::InvalidParameter(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(GetResolverRuleError::ResourceNotFound(err.msg))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(GetResolverRuleError::Throttling(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for GetResolverRuleError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetResolverRuleError::InternalServiceError(ref cause) => write!(f, "{}", cause),
            GetResolverRuleError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            GetResolverRuleError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            GetResolverRuleError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetResolverRuleError {}
/// Errors returned by GetResolverRuleAssociation
#[derive(Debug, PartialEq)]
pub enum GetResolverRuleAssociationError {
    /// <p>We encountered an unknown error. Try again in a few minutes.</p>
    InternalServiceError(String),
    /// <p>One or more parameters in this request are not valid.</p>
    InvalidParameter(String),
    /// <p>The specified resource doesn't exist.</p>
    ResourceNotFound(String),
    /// <p>The request was throttled. Try again in a few minutes.</p>
    Throttling(String),
}

impl GetResolverRuleAssociationError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<GetResolverRuleAssociationError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServiceErrorException" => {
                    return RusotoError::Service(
                        GetResolverRuleAssociationError::InternalServiceError(err.msg),
                    )
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(GetResolverRuleAssociationError::InvalidParameter(
                        err.msg,
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(GetResolverRuleAssociationError::ResourceNotFound(
                        err.msg,
                    ))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(GetResolverRuleAssociationError::Throttling(
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
impl fmt::Display for GetResolverRuleAssociationError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetResolverRuleAssociationError::InternalServiceError(ref cause) => {
                write!(f, "{}", cause)
            }
            GetResolverRuleAssociationError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            GetResolverRuleAssociationError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            GetResolverRuleAssociationError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetResolverRuleAssociationError {}
/// Errors returned by GetResolverRulePolicy
#[derive(Debug, PartialEq)]
pub enum GetResolverRulePolicyError {
    /// <p>We encountered an unknown error. Try again in a few minutes.</p>
    InternalServiceError(String),
    /// <p>One or more parameters in this request are not valid.</p>
    InvalidParameter(String),
    /// <p>The specified resource doesn't exist.</p>
    UnknownResource(String),
}

impl GetResolverRulePolicyError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetResolverRulePolicyError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServiceErrorException" => {
                    return RusotoError::Service(GetResolverRulePolicyError::InternalServiceError(
                        err.msg,
                    ))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(GetResolverRulePolicyError::InvalidParameter(
                        err.msg,
                    ))
                }
                "UnknownResourceException" => {
                    return RusotoError::Service(GetResolverRulePolicyError::UnknownResource(
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
impl fmt::Display for GetResolverRulePolicyError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetResolverRulePolicyError::InternalServiceError(ref cause) => write!(f, "{}", cause),
            GetResolverRulePolicyError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            GetResolverRulePolicyError::UnknownResource(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetResolverRulePolicyError {}
/// Errors returned by ListResolverEndpointIpAddresses
#[derive(Debug, PartialEq)]
pub enum ListResolverEndpointIpAddressesError {
    /// <p>We encountered an unknown error. Try again in a few minutes.</p>
    InternalServiceError(String),
    /// <p>The value that you specified for <code>NextToken</code> in a <code>List</code> request isn't valid.</p>
    InvalidNextToken(String),
    /// <p>One or more parameters in this request are not valid.</p>
    InvalidParameter(String),
    /// <p>The specified resource doesn't exist.</p>
    ResourceNotFound(String),
    /// <p>The request was throttled. Try again in a few minutes.</p>
    Throttling(String),
}

impl ListResolverEndpointIpAddressesError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<ListResolverEndpointIpAddressesError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServiceErrorException" => {
                    return RusotoError::Service(
                        ListResolverEndpointIpAddressesError::InternalServiceError(err.msg),
                    )
                }
                "InvalidNextTokenException" => {
                    return RusotoError::Service(
                        ListResolverEndpointIpAddressesError::InvalidNextToken(err.msg),
                    )
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(
                        ListResolverEndpointIpAddressesError::InvalidParameter(err.msg),
                    )
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(
                        ListResolverEndpointIpAddressesError::ResourceNotFound(err.msg),
                    )
                }
                "ThrottlingException" => {
                    return RusotoError::Service(ListResolverEndpointIpAddressesError::Throttling(
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
impl fmt::Display for ListResolverEndpointIpAddressesError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListResolverEndpointIpAddressesError::InternalServiceError(ref cause) => {
                write!(f, "{}", cause)
            }
            ListResolverEndpointIpAddressesError::InvalidNextToken(ref cause) => {
                write!(f, "{}", cause)
            }
            ListResolverEndpointIpAddressesError::InvalidParameter(ref cause) => {
                write!(f, "{}", cause)
            }
            ListResolverEndpointIpAddressesError::ResourceNotFound(ref cause) => {
                write!(f, "{}", cause)
            }
            ListResolverEndpointIpAddressesError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListResolverEndpointIpAddressesError {}
/// Errors returned by ListResolverEndpoints
#[derive(Debug, PartialEq)]
pub enum ListResolverEndpointsError {
    /// <p>We encountered an unknown error. Try again in a few minutes.</p>
    InternalServiceError(String),
    /// <p>The value that you specified for <code>NextToken</code> in a <code>List</code> request isn't valid.</p>
    InvalidNextToken(String),
    /// <p>One or more parameters in this request are not valid.</p>
    InvalidParameter(String),
    /// <p>The request is invalid.</p>
    InvalidRequest(String),
    /// <p>The request was throttled. Try again in a few minutes.</p>
    Throttling(String),
}

impl ListResolverEndpointsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListResolverEndpointsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServiceErrorException" => {
                    return RusotoError::Service(ListResolverEndpointsError::InternalServiceError(
                        err.msg,
                    ))
                }
                "InvalidNextTokenException" => {
                    return RusotoError::Service(ListResolverEndpointsError::InvalidNextToken(
                        err.msg,
                    ))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(ListResolverEndpointsError::InvalidParameter(
                        err.msg,
                    ))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(ListResolverEndpointsError::InvalidRequest(
                        err.msg,
                    ))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(ListResolverEndpointsError::Throttling(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ListResolverEndpointsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListResolverEndpointsError::InternalServiceError(ref cause) => write!(f, "{}", cause),
            ListResolverEndpointsError::InvalidNextToken(ref cause) => write!(f, "{}", cause),
            ListResolverEndpointsError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            ListResolverEndpointsError::InvalidRequest(ref cause) => write!(f, "{}", cause),
            ListResolverEndpointsError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListResolverEndpointsError {}
/// Errors returned by ListResolverRuleAssociations
#[derive(Debug, PartialEq)]
pub enum ListResolverRuleAssociationsError {
    /// <p>We encountered an unknown error. Try again in a few minutes.</p>
    InternalServiceError(String),
    /// <p>The value that you specified for <code>NextToken</code> in a <code>List</code> request isn't valid.</p>
    InvalidNextToken(String),
    /// <p>One or more parameters in this request are not valid.</p>
    InvalidParameter(String),
    /// <p>The request is invalid.</p>
    InvalidRequest(String),
    /// <p>The request was throttled. Try again in a few minutes.</p>
    Throttling(String),
}

impl ListResolverRuleAssociationsError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<ListResolverRuleAssociationsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServiceErrorException" => {
                    return RusotoError::Service(
                        ListResolverRuleAssociationsError::InternalServiceError(err.msg),
                    )
                }
                "InvalidNextTokenException" => {
                    return RusotoError::Service(
                        ListResolverRuleAssociationsError::InvalidNextToken(err.msg),
                    )
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(
                        ListResolverRuleAssociationsError::InvalidParameter(err.msg),
                    )
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(ListResolverRuleAssociationsError::InvalidRequest(
                        err.msg,
                    ))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(ListResolverRuleAssociationsError::Throttling(
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
impl fmt::Display for ListResolverRuleAssociationsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListResolverRuleAssociationsError::InternalServiceError(ref cause) => {
                write!(f, "{}", cause)
            }
            ListResolverRuleAssociationsError::InvalidNextToken(ref cause) => {
                write!(f, "{}", cause)
            }
            ListResolverRuleAssociationsError::InvalidParameter(ref cause) => {
                write!(f, "{}", cause)
            }
            ListResolverRuleAssociationsError::InvalidRequest(ref cause) => write!(f, "{}", cause),
            ListResolverRuleAssociationsError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListResolverRuleAssociationsError {}
/// Errors returned by ListResolverRules
#[derive(Debug, PartialEq)]
pub enum ListResolverRulesError {
    /// <p>We encountered an unknown error. Try again in a few minutes.</p>
    InternalServiceError(String),
    /// <p>The value that you specified for <code>NextToken</code> in a <code>List</code> request isn't valid.</p>
    InvalidNextToken(String),
    /// <p>One or more parameters in this request are not valid.</p>
    InvalidParameter(String),
    /// <p>The request is invalid.</p>
    InvalidRequest(String),
    /// <p>The request was throttled. Try again in a few minutes.</p>
    Throttling(String),
}

impl ListResolverRulesError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListResolverRulesError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServiceErrorException" => {
                    return RusotoError::Service(ListResolverRulesError::InternalServiceError(
                        err.msg,
                    ))
                }
                "InvalidNextTokenException" => {
                    return RusotoError::Service(ListResolverRulesError::InvalidNextToken(err.msg))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(ListResolverRulesError::InvalidParameter(err.msg))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(ListResolverRulesError::InvalidRequest(err.msg))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(ListResolverRulesError::Throttling(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ListResolverRulesError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListResolverRulesError::InternalServiceError(ref cause) => write!(f, "{}", cause),
            ListResolverRulesError::InvalidNextToken(ref cause) => write!(f, "{}", cause),
            ListResolverRulesError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            ListResolverRulesError::InvalidRequest(ref cause) => write!(f, "{}", cause),
            ListResolverRulesError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListResolverRulesError {}
/// Errors returned by ListTagsForResource
#[derive(Debug, PartialEq)]
pub enum ListTagsForResourceError {
    /// <p>We encountered an unknown error. Try again in a few minutes.</p>
    InternalServiceError(String),
    /// <p>The value that you specified for <code>NextToken</code> in a <code>List</code> request isn't valid.</p>
    InvalidNextToken(String),
    /// <p>One or more parameters in this request are not valid.</p>
    InvalidParameter(String),
    /// <p>The request is invalid.</p>
    InvalidRequest(String),
    /// <p>The specified resource doesn't exist.</p>
    ResourceNotFound(String),
    /// <p>The request was throttled. Try again in a few minutes.</p>
    Throttling(String),
}

impl ListTagsForResourceError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListTagsForResourceError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServiceErrorException" => {
                    return RusotoError::Service(ListTagsForResourceError::InternalServiceError(
                        err.msg,
                    ))
                }
                "InvalidNextTokenException" => {
                    return RusotoError::Service(ListTagsForResourceError::InvalidNextToken(
                        err.msg,
                    ))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(ListTagsForResourceError::InvalidParameter(
                        err.msg,
                    ))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(ListTagsForResourceError::InvalidRequest(err.msg))
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
            ListTagsForResourceError::InternalServiceError(ref cause) => write!(f, "{}", cause),
            ListTagsForResourceError::InvalidNextToken(ref cause) => write!(f, "{}", cause),
            ListTagsForResourceError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            ListTagsForResourceError::InvalidRequest(ref cause) => write!(f, "{}", cause),
            ListTagsForResourceError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            ListTagsForResourceError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListTagsForResourceError {}
/// Errors returned by PutResolverRulePolicy
#[derive(Debug, PartialEq)]
pub enum PutResolverRulePolicyError {
    /// <p>We encountered an unknown error. Try again in a few minutes.</p>
    InternalServiceError(String),
    /// <p>One or more parameters in this request are not valid.</p>
    InvalidParameter(String),
    /// <p>The specified resolver rule policy is invalid.</p>
    InvalidPolicyDocument(String),
    /// <p>The specified resource doesn't exist.</p>
    UnknownResource(String),
}

impl PutResolverRulePolicyError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<PutResolverRulePolicyError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServiceErrorException" => {
                    return RusotoError::Service(PutResolverRulePolicyError::InternalServiceError(
                        err.msg,
                    ))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(PutResolverRulePolicyError::InvalidParameter(
                        err.msg,
                    ))
                }
                "InvalidPolicyDocument" => {
                    return RusotoError::Service(PutResolverRulePolicyError::InvalidPolicyDocument(
                        err.msg,
                    ))
                }
                "UnknownResourceException" => {
                    return RusotoError::Service(PutResolverRulePolicyError::UnknownResource(
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
impl fmt::Display for PutResolverRulePolicyError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            PutResolverRulePolicyError::InternalServiceError(ref cause) => write!(f, "{}", cause),
            PutResolverRulePolicyError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            PutResolverRulePolicyError::InvalidPolicyDocument(ref cause) => write!(f, "{}", cause),
            PutResolverRulePolicyError::UnknownResource(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for PutResolverRulePolicyError {}
/// Errors returned by TagResource
#[derive(Debug, PartialEq)]
pub enum TagResourceError {
    /// <p>We encountered an unknown error. Try again in a few minutes.</p>
    InternalServiceError(String),
    /// <p>One or more parameters in this request are not valid.</p>
    InvalidParameter(String),
    /// <p>The specified tag is invalid.</p>
    InvalidTag(String),
    /// <p>The request caused one or more limits to be exceeded.</p>
    LimitExceeded(String),
    /// <p>The specified resource doesn't exist.</p>
    ResourceNotFound(String),
    /// <p>The request was throttled. Try again in a few minutes.</p>
    Throttling(String),
}

impl TagResourceError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<TagResourceError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServiceErrorException" => {
                    return RusotoError::Service(TagResourceError::InternalServiceError(err.msg))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(TagResourceError::InvalidParameter(err.msg))
                }
                "InvalidTagException" => {
                    return RusotoError::Service(TagResourceError::InvalidTag(err.msg))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(TagResourceError::LimitExceeded(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(TagResourceError::ResourceNotFound(err.msg))
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
            TagResourceError::InternalServiceError(ref cause) => write!(f, "{}", cause),
            TagResourceError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            TagResourceError::InvalidTag(ref cause) => write!(f, "{}", cause),
            TagResourceError::LimitExceeded(ref cause) => write!(f, "{}", cause),
            TagResourceError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            TagResourceError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for TagResourceError {}
/// Errors returned by UntagResource
#[derive(Debug, PartialEq)]
pub enum UntagResourceError {
    /// <p>We encountered an unknown error. Try again in a few minutes.</p>
    InternalServiceError(String),
    /// <p>One or more parameters in this request are not valid.</p>
    InvalidParameter(String),
    /// <p>The specified resource doesn't exist.</p>
    ResourceNotFound(String),
    /// <p>The request was throttled. Try again in a few minutes.</p>
    Throttling(String),
}

impl UntagResourceError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UntagResourceError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServiceErrorException" => {
                    return RusotoError::Service(UntagResourceError::InternalServiceError(err.msg))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(UntagResourceError::InvalidParameter(err.msg))
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
            UntagResourceError::InternalServiceError(ref cause) => write!(f, "{}", cause),
            UntagResourceError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            UntagResourceError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            UntagResourceError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for UntagResourceError {}
/// Errors returned by UpdateResolverEndpoint
#[derive(Debug, PartialEq)]
pub enum UpdateResolverEndpointError {
    /// <p>We encountered an unknown error. Try again in a few minutes.</p>
    InternalServiceError(String),
    /// <p>One or more parameters in this request are not valid.</p>
    InvalidParameter(String),
    /// <p>The request is invalid.</p>
    InvalidRequest(String),
    /// <p>The specified resource doesn't exist.</p>
    ResourceNotFound(String),
    /// <p>The request was throttled. Try again in a few minutes.</p>
    Throttling(String),
}

impl UpdateResolverEndpointError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdateResolverEndpointError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServiceErrorException" => {
                    return RusotoError::Service(UpdateResolverEndpointError::InternalServiceError(
                        err.msg,
                    ))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(UpdateResolverEndpointError::InvalidParameter(
                        err.msg,
                    ))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(UpdateResolverEndpointError::InvalidRequest(
                        err.msg,
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(UpdateResolverEndpointError::ResourceNotFound(
                        err.msg,
                    ))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(UpdateResolverEndpointError::Throttling(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for UpdateResolverEndpointError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UpdateResolverEndpointError::InternalServiceError(ref cause) => write!(f, "{}", cause),
            UpdateResolverEndpointError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            UpdateResolverEndpointError::InvalidRequest(ref cause) => write!(f, "{}", cause),
            UpdateResolverEndpointError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            UpdateResolverEndpointError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for UpdateResolverEndpointError {}
/// Errors returned by UpdateResolverRule
#[derive(Debug, PartialEq)]
pub enum UpdateResolverRuleError {
    /// <p>We encountered an unknown error. Try again in a few minutes.</p>
    InternalServiceError(String),
    /// <p>One or more parameters in this request are not valid.</p>
    InvalidParameter(String),
    /// <p>The request is invalid.</p>
    InvalidRequest(String),
    /// <p>The request caused one or more limits to be exceeded.</p>
    LimitExceeded(String),
    /// <p>The specified resource doesn't exist.</p>
    ResourceNotFound(String),
    /// <p>The specified resource isn't available.</p>
    ResourceUnavailable(String),
    /// <p>The request was throttled. Try again in a few minutes.</p>
    Throttling(String),
}

impl UpdateResolverRuleError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdateResolverRuleError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServiceErrorException" => {
                    return RusotoError::Service(UpdateResolverRuleError::InternalServiceError(
                        err.msg,
                    ))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(UpdateResolverRuleError::InvalidParameter(err.msg))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(UpdateResolverRuleError::InvalidRequest(err.msg))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(UpdateResolverRuleError::LimitExceeded(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(UpdateResolverRuleError::ResourceNotFound(err.msg))
                }
                "ResourceUnavailableException" => {
                    return RusotoError::Service(UpdateResolverRuleError::ResourceUnavailable(
                        err.msg,
                    ))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(UpdateResolverRuleError::Throttling(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for UpdateResolverRuleError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UpdateResolverRuleError::InternalServiceError(ref cause) => write!(f, "{}", cause),
            UpdateResolverRuleError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            UpdateResolverRuleError::InvalidRequest(ref cause) => write!(f, "{}", cause),
            UpdateResolverRuleError::LimitExceeded(ref cause) => write!(f, "{}", cause),
            UpdateResolverRuleError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            UpdateResolverRuleError::ResourceUnavailable(ref cause) => write!(f, "{}", cause),
            UpdateResolverRuleError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for UpdateResolverRuleError {}
/// Trait representing the capabilities of the Route53Resolver API. Route53Resolver clients implement this trait.
#[async_trait]
pub trait Route53Resolver {
    /// <p>Adds IP addresses to an inbound or an outbound resolver endpoint. If you want to adding more than one IP address, submit one <code>AssociateResolverEndpointIpAddress</code> request for each IP address.</p> <p>To remove an IP address from an endpoint, see <a>DisassociateResolverEndpointIpAddress</a>.</p>
    async fn associate_resolver_endpoint_ip_address(
        &self,
        input: AssociateResolverEndpointIpAddressRequest,
    ) -> Result<
        AssociateResolverEndpointIpAddressResponse,
        RusotoError<AssociateResolverEndpointIpAddressError>,
    >;

    /// <p>Associates a resolver rule with a VPC. When you associate a rule with a VPC, Resolver forwards all DNS queries for the domain name that is specified in the rule and that originate in the VPC. The queries are forwarded to the IP addresses for the DNS resolvers that are specified in the rule. For more information about rules, see <a>CreateResolverRule</a>. </p>
    async fn associate_resolver_rule(
        &self,
        input: AssociateResolverRuleRequest,
    ) -> Result<AssociateResolverRuleResponse, RusotoError<AssociateResolverRuleError>>;

    /// <p><p>Creates a resolver endpoint. There are two types of resolver endpoints, inbound and outbound:</p> <ul> <li> <p>An <i>inbound resolver endpoint</i> forwards DNS queries to the DNS service for a VPC from your network or another VPC.</p> </li> <li> <p>An <i>outbound resolver endpoint</i> forwards DNS queries from the DNS service for a VPC to your network or another VPC.</p> </li> </ul></p>
    async fn create_resolver_endpoint(
        &self,
        input: CreateResolverEndpointRequest,
    ) -> Result<CreateResolverEndpointResponse, RusotoError<CreateResolverEndpointError>>;

    /// <p>For DNS queries that originate in your VPCs, specifies which resolver endpoint the queries pass through, one domain name that you want to forward to your network, and the IP addresses of the DNS resolvers in your network.</p>
    async fn create_resolver_rule(
        &self,
        input: CreateResolverRuleRequest,
    ) -> Result<CreateResolverRuleResponse, RusotoError<CreateResolverRuleError>>;

    /// <p><p>Deletes a resolver endpoint. The effect of deleting a resolver endpoint depends on whether it&#39;s an inbound or an outbound resolver endpoint:</p> <ul> <li> <p> <b>Inbound</b>: DNS queries from your network or another VPC are no longer routed to the DNS service for the specified VPC.</p> </li> <li> <p> <b>Outbound</b>: DNS queries from a VPC are no longer routed to your network or to another VPC.</p> </li> </ul></p>
    async fn delete_resolver_endpoint(
        &self,
        input: DeleteResolverEndpointRequest,
    ) -> Result<DeleteResolverEndpointResponse, RusotoError<DeleteResolverEndpointError>>;

    /// <p>Deletes a resolver rule. Before you can delete a resolver rule, you must disassociate it from all the VPCs that you associated the resolver rule with. For more infomation, see <a>DisassociateResolverRule</a>.</p>
    async fn delete_resolver_rule(
        &self,
        input: DeleteResolverRuleRequest,
    ) -> Result<DeleteResolverRuleResponse, RusotoError<DeleteResolverRuleError>>;

    /// <p>Removes IP addresses from an inbound or an outbound resolver endpoint. If you want to remove more than one IP address, submit one <code>DisassociateResolverEndpointIpAddress</code> request for each IP address.</p> <p>To add an IP address to an endpoint, see <a>AssociateResolverEndpointIpAddress</a>.</p>
    async fn disassociate_resolver_endpoint_ip_address(
        &self,
        input: DisassociateResolverEndpointIpAddressRequest,
    ) -> Result<
        DisassociateResolverEndpointIpAddressResponse,
        RusotoError<DisassociateResolverEndpointIpAddressError>,
    >;

    /// <p><p>Removes the association between a specified resolver rule and a specified VPC.</p> <important> <p>If you disassociate a resolver rule from a VPC, Resolver stops forwarding DNS queries for the domain name that you specified in the resolver rule. </p> </important></p>
    async fn disassociate_resolver_rule(
        &self,
        input: DisassociateResolverRuleRequest,
    ) -> Result<DisassociateResolverRuleResponse, RusotoError<DisassociateResolverRuleError>>;

    /// <p>Gets information about a specified resolver endpoint, such as whether it's an inbound or an outbound resolver endpoint, and the current status of the endpoint.</p>
    async fn get_resolver_endpoint(
        &self,
        input: GetResolverEndpointRequest,
    ) -> Result<GetResolverEndpointResponse, RusotoError<GetResolverEndpointError>>;

    /// <p>Gets information about a specified resolver rule, such as the domain name that the rule forwards DNS queries for and the ID of the outbound resolver endpoint that the rule is associated with.</p>
    async fn get_resolver_rule(
        &self,
        input: GetResolverRuleRequest,
    ) -> Result<GetResolverRuleResponse, RusotoError<GetResolverRuleError>>;

    /// <p>Gets information about an association between a specified resolver rule and a VPC. You associate a resolver rule and a VPC using <a>AssociateResolverRule</a>. </p>
    async fn get_resolver_rule_association(
        &self,
        input: GetResolverRuleAssociationRequest,
    ) -> Result<GetResolverRuleAssociationResponse, RusotoError<GetResolverRuleAssociationError>>;

    /// <p>Gets information about a resolver rule policy. A resolver rule policy specifies the Resolver operations and resources that you want to allow another AWS account to be able to use. </p>
    async fn get_resolver_rule_policy(
        &self,
        input: GetResolverRulePolicyRequest,
    ) -> Result<GetResolverRulePolicyResponse, RusotoError<GetResolverRulePolicyError>>;

    /// <p>Gets the IP addresses for a specified resolver endpoint.</p>
    async fn list_resolver_endpoint_ip_addresses(
        &self,
        input: ListResolverEndpointIpAddressesRequest,
    ) -> Result<
        ListResolverEndpointIpAddressesResponse,
        RusotoError<ListResolverEndpointIpAddressesError>,
    >;

    /// <p>Lists all the resolver endpoints that were created using the current AWS account.</p>
    async fn list_resolver_endpoints(
        &self,
        input: ListResolverEndpointsRequest,
    ) -> Result<ListResolverEndpointsResponse, RusotoError<ListResolverEndpointsError>>;

    /// <p>Lists the associations that were created between resolver rules and VPCs using the current AWS account.</p>
    async fn list_resolver_rule_associations(
        &self,
        input: ListResolverRuleAssociationsRequest,
    ) -> Result<ListResolverRuleAssociationsResponse, RusotoError<ListResolverRuleAssociationsError>>;

    /// <p>Lists the resolver rules that were created using the current AWS account.</p>
    async fn list_resolver_rules(
        &self,
        input: ListResolverRulesRequest,
    ) -> Result<ListResolverRulesResponse, RusotoError<ListResolverRulesError>>;

    /// <p>Lists the tags that you associated with the specified resource.</p>
    async fn list_tags_for_resource(
        &self,
        input: ListTagsForResourceRequest,
    ) -> Result<ListTagsForResourceResponse, RusotoError<ListTagsForResourceError>>;

    /// <p>Specifies the Resolver operations and resources that you want to allow another AWS account to be able to use.</p>
    async fn put_resolver_rule_policy(
        &self,
        input: PutResolverRulePolicyRequest,
    ) -> Result<PutResolverRulePolicyResponse, RusotoError<PutResolverRulePolicyError>>;

    /// <p>Adds one or more tags to a specified resource.</p>
    async fn tag_resource(
        &self,
        input: TagResourceRequest,
    ) -> Result<TagResourceResponse, RusotoError<TagResourceError>>;

    /// <p>Removes one or more tags from a specified resource.</p>
    async fn untag_resource(
        &self,
        input: UntagResourceRequest,
    ) -> Result<UntagResourceResponse, RusotoError<UntagResourceError>>;

    /// <p>Updates the name of an inbound or an outbound resolver endpoint. </p>
    async fn update_resolver_endpoint(
        &self,
        input: UpdateResolverEndpointRequest,
    ) -> Result<UpdateResolverEndpointResponse, RusotoError<UpdateResolverEndpointError>>;

    /// <p>Updates settings for a specified resolver rule. <code>ResolverRuleId</code> is required, and all other parameters are optional. If you don't specify a parameter, it retains its current value.</p>
    async fn update_resolver_rule(
        &self,
        input: UpdateResolverRuleRequest,
    ) -> Result<UpdateResolverRuleResponse, RusotoError<UpdateResolverRuleError>>;
}
/// A client for the Route53Resolver API.
#[derive(Clone)]
pub struct Route53ResolverClient {
    client: Client,
    region: region::Region,
}

impl Route53ResolverClient {
    /// Creates a client backed by the default tokio event loop.
    ///
    /// The client will use the default credentials provider and tls client.
    pub fn new(region: region::Region) -> Route53ResolverClient {
        Route53ResolverClient {
            client: Client::shared(),
            region,
        }
    }

    pub fn new_with<P, D>(
        request_dispatcher: D,
        credentials_provider: P,
        region: region::Region,
    ) -> Route53ResolverClient
    where
        P: ProvideAwsCredentials + Send + Sync + 'static,
        D: DispatchSignedRequest + Send + Sync + 'static,
    {
        Route53ResolverClient {
            client: Client::new_with(credentials_provider, request_dispatcher),
            region,
        }
    }

    pub fn new_with_client(client: Client, region: region::Region) -> Route53ResolverClient {
        Route53ResolverClient { client, region }
    }
}

#[async_trait]
impl Route53Resolver for Route53ResolverClient {
    /// <p>Adds IP addresses to an inbound or an outbound resolver endpoint. If you want to adding more than one IP address, submit one <code>AssociateResolverEndpointIpAddress</code> request for each IP address.</p> <p>To remove an IP address from an endpoint, see <a>DisassociateResolverEndpointIpAddress</a>.</p>
    async fn associate_resolver_endpoint_ip_address(
        &self,
        input: AssociateResolverEndpointIpAddressRequest,
    ) -> Result<
        AssociateResolverEndpointIpAddressResponse,
        RusotoError<AssociateResolverEndpointIpAddressError>,
    > {
        let mut request = SignedRequest::new("POST", "route53resolver", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "Route53Resolver.AssociateResolverEndpointIpAddress",
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
                .deserialize::<AssociateResolverEndpointIpAddressResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(AssociateResolverEndpointIpAddressError::from_response(
                response,
            ))
        }
    }

    /// <p>Associates a resolver rule with a VPC. When you associate a rule with a VPC, Resolver forwards all DNS queries for the domain name that is specified in the rule and that originate in the VPC. The queries are forwarded to the IP addresses for the DNS resolvers that are specified in the rule. For more information about rules, see <a>CreateResolverRule</a>. </p>
    async fn associate_resolver_rule(
        &self,
        input: AssociateResolverRuleRequest,
    ) -> Result<AssociateResolverRuleResponse, RusotoError<AssociateResolverRuleError>> {
        let mut request = SignedRequest::new("POST", "route53resolver", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "Route53Resolver.AssociateResolverRule");
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
                .deserialize::<AssociateResolverRuleResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(AssociateResolverRuleError::from_response(response))
        }
    }

    /// <p><p>Creates a resolver endpoint. There are two types of resolver endpoints, inbound and outbound:</p> <ul> <li> <p>An <i>inbound resolver endpoint</i> forwards DNS queries to the DNS service for a VPC from your network or another VPC.</p> </li> <li> <p>An <i>outbound resolver endpoint</i> forwards DNS queries from the DNS service for a VPC to your network or another VPC.</p> </li> </ul></p>
    async fn create_resolver_endpoint(
        &self,
        input: CreateResolverEndpointRequest,
    ) -> Result<CreateResolverEndpointResponse, RusotoError<CreateResolverEndpointError>> {
        let mut request = SignedRequest::new("POST", "route53resolver", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "Route53Resolver.CreateResolverEndpoint");
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
                .deserialize::<CreateResolverEndpointResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(CreateResolverEndpointError::from_response(response))
        }
    }

    /// <p>For DNS queries that originate in your VPCs, specifies which resolver endpoint the queries pass through, one domain name that you want to forward to your network, and the IP addresses of the DNS resolvers in your network.</p>
    async fn create_resolver_rule(
        &self,
        input: CreateResolverRuleRequest,
    ) -> Result<CreateResolverRuleResponse, RusotoError<CreateResolverRuleError>> {
        let mut request = SignedRequest::new("POST", "route53resolver", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "Route53Resolver.CreateResolverRule");
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
                .deserialize::<CreateResolverRuleResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(CreateResolverRuleError::from_response(response))
        }
    }

    /// <p><p>Deletes a resolver endpoint. The effect of deleting a resolver endpoint depends on whether it&#39;s an inbound or an outbound resolver endpoint:</p> <ul> <li> <p> <b>Inbound</b>: DNS queries from your network or another VPC are no longer routed to the DNS service for the specified VPC.</p> </li> <li> <p> <b>Outbound</b>: DNS queries from a VPC are no longer routed to your network or to another VPC.</p> </li> </ul></p>
    async fn delete_resolver_endpoint(
        &self,
        input: DeleteResolverEndpointRequest,
    ) -> Result<DeleteResolverEndpointResponse, RusotoError<DeleteResolverEndpointError>> {
        let mut request = SignedRequest::new("POST", "route53resolver", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "Route53Resolver.DeleteResolverEndpoint");
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
                .deserialize::<DeleteResolverEndpointResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(DeleteResolverEndpointError::from_response(response))
        }
    }

    /// <p>Deletes a resolver rule. Before you can delete a resolver rule, you must disassociate it from all the VPCs that you associated the resolver rule with. For more infomation, see <a>DisassociateResolverRule</a>.</p>
    async fn delete_resolver_rule(
        &self,
        input: DeleteResolverRuleRequest,
    ) -> Result<DeleteResolverRuleResponse, RusotoError<DeleteResolverRuleError>> {
        let mut request = SignedRequest::new("POST", "route53resolver", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "Route53Resolver.DeleteResolverRule");
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
                .deserialize::<DeleteResolverRuleResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(DeleteResolverRuleError::from_response(response))
        }
    }

    /// <p>Removes IP addresses from an inbound or an outbound resolver endpoint. If you want to remove more than one IP address, submit one <code>DisassociateResolverEndpointIpAddress</code> request for each IP address.</p> <p>To add an IP address to an endpoint, see <a>AssociateResolverEndpointIpAddress</a>.</p>
    async fn disassociate_resolver_endpoint_ip_address(
        &self,
        input: DisassociateResolverEndpointIpAddressRequest,
    ) -> Result<
        DisassociateResolverEndpointIpAddressResponse,
        RusotoError<DisassociateResolverEndpointIpAddressError>,
    > {
        let mut request = SignedRequest::new("POST", "route53resolver", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "Route53Resolver.DisassociateResolverEndpointIpAddress",
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
                .deserialize::<DisassociateResolverEndpointIpAddressResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(DisassociateResolverEndpointIpAddressError::from_response(
                response,
            ))
        }
    }

    /// <p><p>Removes the association between a specified resolver rule and a specified VPC.</p> <important> <p>If you disassociate a resolver rule from a VPC, Resolver stops forwarding DNS queries for the domain name that you specified in the resolver rule. </p> </important></p>
    async fn disassociate_resolver_rule(
        &self,
        input: DisassociateResolverRuleRequest,
    ) -> Result<DisassociateResolverRuleResponse, RusotoError<DisassociateResolverRuleError>> {
        let mut request = SignedRequest::new("POST", "route53resolver", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "Route53Resolver.DisassociateResolverRule");
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
                .deserialize::<DisassociateResolverRuleResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(DisassociateResolverRuleError::from_response(response))
        }
    }

    /// <p>Gets information about a specified resolver endpoint, such as whether it's an inbound or an outbound resolver endpoint, and the current status of the endpoint.</p>
    async fn get_resolver_endpoint(
        &self,
        input: GetResolverEndpointRequest,
    ) -> Result<GetResolverEndpointResponse, RusotoError<GetResolverEndpointError>> {
        let mut request = SignedRequest::new("POST", "route53resolver", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "Route53Resolver.GetResolverEndpoint");
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
                .deserialize::<GetResolverEndpointResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(GetResolverEndpointError::from_response(response))
        }
    }

    /// <p>Gets information about a specified resolver rule, such as the domain name that the rule forwards DNS queries for and the ID of the outbound resolver endpoint that the rule is associated with.</p>
    async fn get_resolver_rule(
        &self,
        input: GetResolverRuleRequest,
    ) -> Result<GetResolverRuleResponse, RusotoError<GetResolverRuleError>> {
        let mut request = SignedRequest::new("POST", "route53resolver", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "Route53Resolver.GetResolverRule");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response).deserialize::<GetResolverRuleResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(GetResolverRuleError::from_response(response))
        }
    }

    /// <p>Gets information about an association between a specified resolver rule and a VPC. You associate a resolver rule and a VPC using <a>AssociateResolverRule</a>. </p>
    async fn get_resolver_rule_association(
        &self,
        input: GetResolverRuleAssociationRequest,
    ) -> Result<GetResolverRuleAssociationResponse, RusotoError<GetResolverRuleAssociationError>>
    {
        let mut request = SignedRequest::new("POST", "route53resolver", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "Route53Resolver.GetResolverRuleAssociation");
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
                .deserialize::<GetResolverRuleAssociationResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(GetResolverRuleAssociationError::from_response(response))
        }
    }

    /// <p>Gets information about a resolver rule policy. A resolver rule policy specifies the Resolver operations and resources that you want to allow another AWS account to be able to use. </p>
    async fn get_resolver_rule_policy(
        &self,
        input: GetResolverRulePolicyRequest,
    ) -> Result<GetResolverRulePolicyResponse, RusotoError<GetResolverRulePolicyError>> {
        let mut request = SignedRequest::new("POST", "route53resolver", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "Route53Resolver.GetResolverRulePolicy");
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
                .deserialize::<GetResolverRulePolicyResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(GetResolverRulePolicyError::from_response(response))
        }
    }

    /// <p>Gets the IP addresses for a specified resolver endpoint.</p>
    async fn list_resolver_endpoint_ip_addresses(
        &self,
        input: ListResolverEndpointIpAddressesRequest,
    ) -> Result<
        ListResolverEndpointIpAddressesResponse,
        RusotoError<ListResolverEndpointIpAddressesError>,
    > {
        let mut request = SignedRequest::new("POST", "route53resolver", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "Route53Resolver.ListResolverEndpointIpAddresses",
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
                .deserialize::<ListResolverEndpointIpAddressesResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(ListResolverEndpointIpAddressesError::from_response(
                response,
            ))
        }
    }

    /// <p>Lists all the resolver endpoints that were created using the current AWS account.</p>
    async fn list_resolver_endpoints(
        &self,
        input: ListResolverEndpointsRequest,
    ) -> Result<ListResolverEndpointsResponse, RusotoError<ListResolverEndpointsError>> {
        let mut request = SignedRequest::new("POST", "route53resolver", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "Route53Resolver.ListResolverEndpoints");
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
                .deserialize::<ListResolverEndpointsResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(ListResolverEndpointsError::from_response(response))
        }
    }

    /// <p>Lists the associations that were created between resolver rules and VPCs using the current AWS account.</p>
    async fn list_resolver_rule_associations(
        &self,
        input: ListResolverRuleAssociationsRequest,
    ) -> Result<ListResolverRuleAssociationsResponse, RusotoError<ListResolverRuleAssociationsError>>
    {
        let mut request = SignedRequest::new("POST", "route53resolver", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "Route53Resolver.ListResolverRuleAssociations",
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
                .deserialize::<ListResolverRuleAssociationsResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(ListResolverRuleAssociationsError::from_response(response))
        }
    }

    /// <p>Lists the resolver rules that were created using the current AWS account.</p>
    async fn list_resolver_rules(
        &self,
        input: ListResolverRulesRequest,
    ) -> Result<ListResolverRulesResponse, RusotoError<ListResolverRulesError>> {
        let mut request = SignedRequest::new("POST", "route53resolver", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "Route53Resolver.ListResolverRules");
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
                .deserialize::<ListResolverRulesResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(ListResolverRulesError::from_response(response))
        }
    }

    /// <p>Lists the tags that you associated with the specified resource.</p>
    async fn list_tags_for_resource(
        &self,
        input: ListTagsForResourceRequest,
    ) -> Result<ListTagsForResourceResponse, RusotoError<ListTagsForResourceError>> {
        let mut request = SignedRequest::new("POST", "route53resolver", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "Route53Resolver.ListTagsForResource");
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
                .deserialize::<ListTagsForResourceResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(ListTagsForResourceError::from_response(response))
        }
    }

    /// <p>Specifies the Resolver operations and resources that you want to allow another AWS account to be able to use.</p>
    async fn put_resolver_rule_policy(
        &self,
        input: PutResolverRulePolicyRequest,
    ) -> Result<PutResolverRulePolicyResponse, RusotoError<PutResolverRulePolicyError>> {
        let mut request = SignedRequest::new("POST", "route53resolver", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "Route53Resolver.PutResolverRulePolicy");
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
                .deserialize::<PutResolverRulePolicyResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(PutResolverRulePolicyError::from_response(response))
        }
    }

    /// <p>Adds one or more tags to a specified resource.</p>
    async fn tag_resource(
        &self,
        input: TagResourceRequest,
    ) -> Result<TagResourceResponse, RusotoError<TagResourceError>> {
        let mut request = SignedRequest::new("POST", "route53resolver", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "Route53Resolver.TagResource");
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

    /// <p>Removes one or more tags from a specified resource.</p>
    async fn untag_resource(
        &self,
        input: UntagResourceRequest,
    ) -> Result<UntagResourceResponse, RusotoError<UntagResourceError>> {
        let mut request = SignedRequest::new("POST", "route53resolver", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "Route53Resolver.UntagResource");
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

    /// <p>Updates the name of an inbound or an outbound resolver endpoint. </p>
    async fn update_resolver_endpoint(
        &self,
        input: UpdateResolverEndpointRequest,
    ) -> Result<UpdateResolverEndpointResponse, RusotoError<UpdateResolverEndpointError>> {
        let mut request = SignedRequest::new("POST", "route53resolver", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "Route53Resolver.UpdateResolverEndpoint");
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
                .deserialize::<UpdateResolverEndpointResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(UpdateResolverEndpointError::from_response(response))
        }
    }

    /// <p>Updates settings for a specified resolver rule. <code>ResolverRuleId</code> is required, and all other parameters are optional. If you don't specify a parameter, it retains its current value.</p>
    async fn update_resolver_rule(
        &self,
        input: UpdateResolverRuleRequest,
    ) -> Result<UpdateResolverRuleResponse, RusotoError<UpdateResolverRuleError>> {
        let mut request = SignedRequest::new("POST", "route53resolver", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "Route53Resolver.UpdateResolverRule");
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
                .deserialize::<UpdateResolverRuleResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(UpdateResolverRuleError::from_response(response))
        }
    }
}
