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

impl Route53ResolverClient {
    fn new_signed_request(&self, http_method: &str, request_uri: &str) -> SignedRequest {
        let mut request =
            SignedRequest::new(http_method, "route53resolver", &self.region, request_uri);

        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request
    }

    async fn sign_and_dispatch(
        &self,
        request: SignedRequest,
    ) -> Result<HttpResponse, RusotoError<std::convert::Infallible>> {
        let mut response = self.client.sign_and_dispatch(request).await?;
        if !response.status.is_success() {
            let response = response.buffer().await?;
            return Err(RusotoError::Unknown(response));
        }

        Ok(response)
    }
}

use serde_json;
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct AssociateResolverEndpointIpAddressRequest {
    /// <p>Either the IPv4 address that you want to add to a Resolver endpoint or a subnet ID. If you specify a subnet ID, Resolver chooses an IP address for you from the available IPs in the specified subnet.</p>
    #[serde(rename = "IpAddress")]
    pub ip_address: IpAddressUpdate,
    /// <p>The ID of the Resolver endpoint that you want to associate IP addresses with.</p>
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
pub struct AssociateResolverQueryLogConfigRequest {
    /// <p>The ID of the query logging configuration that you want to associate a VPC with.</p>
    #[serde(rename = "ResolverQueryLogConfigId")]
    pub resolver_query_log_config_id: String,
    /// <p><p>The ID of an Amazon VPC that you want this query logging configuration to log queries for.</p> <note> <p>The VPCs and the query logging configuration must be in the same Region.</p> </note></p>
    #[serde(rename = "ResourceId")]
    pub resource_id: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct AssociateResolverQueryLogConfigResponse {
    /// <p>A complex type that contains settings for a specified association between an Amazon VPC and a query logging configuration.</p>
    #[serde(rename = "ResolverQueryLogConfigAssociation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resolver_query_log_config_association: Option<ResolverQueryLogConfigAssociation>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct AssociateResolverRuleRequest {
    /// <p>A name for the association that you're creating between a Resolver rule and a VPC.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The ID of the Resolver rule that you want to associate with the VPC. To list the existing Resolver rules, use <a href="https://docs.aws.amazon.com/Route53/latest/APIReference/API_route53resolver_ListResolverRules.html">ListResolverRules</a>.</p>
    #[serde(rename = "ResolverRuleId")]
    pub resolver_rule_id: String,
    /// <p>The ID of the VPC that you want to associate the Resolver rule with.</p>
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
    /// <p><p>Specify the applicable value:</p> <ul> <li> <p> <code>INBOUND</code>: Resolver forwards DNS queries to the DNS service for a VPC from your network</p> </li> <li> <p> <code>OUTBOUND</code>: Resolver forwards DNS queries from the DNS service for a VPC to your network</p> </li> </ul></p>
    #[serde(rename = "Direction")]
    pub direction: String,
    /// <p>The subnets and IP addresses in your VPC that DNS queries originate from (for outbound endpoints) or that you forward DNS queries to (for inbound endpoints). The subnet ID uniquely identifies a VPC. </p>
    #[serde(rename = "IpAddresses")]
    pub ip_addresses: Vec<IpAddressRequest>,
    /// <p>A friendly name that lets you easily find a configuration in the Resolver dashboard in the Route 53 console.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The ID of one or more security groups that you want to use to control access to this VPC. The security group that you specify must include one or more inbound rules (for inbound Resolver endpoints) or outbound rules (for outbound Resolver endpoints). Inbound and outbound rules must allow TCP and UDP access. For inbound access, open port 53. For outbound access, open the port that you're using for DNS queries on your network.</p>
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
pub struct CreateResolverQueryLogConfigRequest {
    /// <p>A unique string that identifies the request and that allows failed requests to be retried without the risk of executing the operation twice. <code>CreatorRequestId</code> can be any unique string, for example, a date/time stamp. </p>
    #[serde(rename = "CreatorRequestId")]
    pub creator_request_id: String,
    /// <p><p>The ARN of the resource that you want Resolver to send query logs. You can send query logs to an S3 bucket, a CloudWatch Logs log group, or a Kinesis Data Firehose delivery stream. Examples of valid values include the following:</p> <ul> <li> <p> <b>S3 bucket</b>: </p> <p> <code>arn:aws:s3:::examplebucket</code> </p> <p>You can optionally append a file prefix to the end of the ARN.</p> <p> <code>arn:aws:s3:::examplebucket/development/</code> </p> </li> <li> <p> <b>CloudWatch Logs log group</b>: </p> <p> <code>arn:aws:logs:us-west-1:123456789012:log-group:/mystack-testgroup-12ABC1AB12A1:*</code> </p> </li> <li> <p> <b>Kinesis Data Firehose delivery stream</b>:</p> <p> <code>arn:aws:kinesis:us-east-2:0123456789:stream/my<em>stream</em>name</code> </p> </li> </ul></p>
    #[serde(rename = "DestinationArn")]
    pub destination_arn: String,
    /// <p>The name that you want to give the query logging configuration</p>
    #[serde(rename = "Name")]
    pub name: String,
    /// <p>A list of the tag keys and values that you want to associate with the query logging configuration.</p>
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateResolverQueryLogConfigResponse {
    /// <p>Information about the <code>CreateResolverQueryLogConfig</code> request, including the status of the request.</p>
    #[serde(rename = "ResolverQueryLogConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resolver_query_log_config: Option<ResolverQueryLogConfig>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateResolverRuleRequest {
    /// <p>A unique string that identifies the request and that allows failed requests to be retried without the risk of executing the operation twice. <code>CreatorRequestId</code> can be any unique string, for example, a date/time stamp. </p>
    #[serde(rename = "CreatorRequestId")]
    pub creator_request_id: String,
    /// <p>DNS queries for this domain name are forwarded to the IP addresses that you specify in <code>TargetIps</code>. If a query matches multiple Resolver rules (example.com and www.example.com), outbound DNS queries are routed using the Resolver rule that contains the most specific domain name (www.example.com).</p>
    #[serde(rename = "DomainName")]
    pub domain_name: String,
    /// <p>A friendly name that lets you easily find a rule in the Resolver dashboard in the Route 53 console.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The ID of the outbound Resolver endpoint that you want to use to route DNS queries to the IP addresses that you specify in <code>TargetIps</code>.</p>
    #[serde(rename = "ResolverEndpointId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resolver_endpoint_id: Option<String>,
    /// <p>When you want to forward DNS queries for specified domain name to resolvers on your network, specify <code>FORWARD</code>.</p> <p>When you have a forwarding rule to forward DNS queries for a domain to your network and you want Resolver to process queries for a subdomain of that domain, specify <code>SYSTEM</code>.</p> <p>For example, to forward DNS queries for example.com to resolvers on your network, you create a rule and specify <code>FORWARD</code> for <code>RuleType</code>. To then have Resolver process queries for apex.example.com, you create a rule and specify <code>SYSTEM</code> for <code>RuleType</code>.</p> <p>Currently, only Resolver can create rules that have a value of <code>RECURSIVE</code> for <code>RuleType</code>.</p>
    #[serde(rename = "RuleType")]
    pub rule_type: String,
    /// <p>A list of the tag keys and values that you want to associate with the endpoint.</p>
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
    /// <p>The IPs that you want Resolver to forward DNS queries to. You can specify only IPv4 addresses. Separate IP addresses with a comma.</p> <p> <code>TargetIps</code> is available only when the value of <code>Rule type</code> is <code>FORWARD</code>.</p>
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
    /// <p>The ID of the Resolver endpoint that you want to delete.</p>
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
pub struct DeleteResolverQueryLogConfigRequest {
    /// <p>The ID of the query logging configuration that you want to delete.</p>
    #[serde(rename = "ResolverQueryLogConfigId")]
    pub resolver_query_log_config_id: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DeleteResolverQueryLogConfigResponse {
    /// <p>Information about the query logging configuration that you deleted, including the status of the request.</p>
    #[serde(rename = "ResolverQueryLogConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resolver_query_log_config: Option<ResolverQueryLogConfig>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteResolverRuleRequest {
    /// <p>The ID of the Resolver rule that you want to delete.</p>
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
    /// <p>The IPv4 address that you want to remove from a Resolver endpoint.</p>
    #[serde(rename = "IpAddress")]
    pub ip_address: IpAddressUpdate,
    /// <p>The ID of the Resolver endpoint that you want to disassociate an IP address from.</p>
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
pub struct DisassociateResolverQueryLogConfigRequest {
    /// <p>The ID of the query logging configuration that you want to disassociate a specified VPC from.</p>
    #[serde(rename = "ResolverQueryLogConfigId")]
    pub resolver_query_log_config_id: String,
    /// <p>The ID of the Amazon VPC that you want to disassociate from a specified query logging configuration.</p>
    #[serde(rename = "ResourceId")]
    pub resource_id: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DisassociateResolverQueryLogConfigResponse {
    /// <p>A complex type that contains settings for the association that you deleted between an Amazon VPC and a query logging configuration.</p>
    #[serde(rename = "ResolverQueryLogConfigAssociation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resolver_query_log_config_association: Option<ResolverQueryLogConfigAssociation>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DisassociateResolverRuleRequest {
    /// <p>The ID of the Resolver rule that you want to disassociate from the specified VPC.</p>
    #[serde(rename = "ResolverRuleId")]
    pub resolver_rule_id: String,
    /// <p>The ID of the VPC that you want to disassociate the Resolver rule from.</p>
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

/// <p>For Resolver list operations (<a href="https://docs.aws.amazon.com/Route53/latest/APIReference/API_route53resolver_ListResolverEndpoints.html">ListResolverEndpoints</a>, <a href="https://docs.aws.amazon.com/Route53/latest/APIReference/API_route53resolver_ListResolverRules.html">ListResolverRules</a>, <a href="https://docs.aws.amazon.com/Route53/latest/APIReference/API_route53resolver_ListResolverRuleAssociations.html">ListResolverRuleAssociations</a>, <a href="https://docs.aws.amazon.com/Route53/latest/APIReference/API_route53resolver_ListResolverQueryLogConfigs.html">ListResolverQueryLogConfigs</a>, <a href="https://docs.aws.amazon.com/Route53/latest/APIReference/API_route53resolver_ListResolverQueryLogConfigAssociations.html">ListResolverQueryLogConfigAssociations</a>), and <a href="https://docs.aws.amazon.com/Route53/latest/APIReference/API_route53resolver_ListResolverDnssecConfigs.html">ListResolverDnssecConfigs</a>), an optional specification to return a subset of objects.</p> <p>To filter objects, such as Resolver endpoints or Resolver rules, you specify <code>Name</code> and <code>Values</code>. For example, to list only inbound Resolver endpoints, specify <code>Direction</code> for <code>Name</code> and specify <code>INBOUND</code> for <code>Values</code>. </p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct Filter {
    /// <p><p>The name of the parameter that you want to use to filter objects.</p> <p>The valid values for <code>Name</code> depend on the action that you&#39;re including the filter in, <a href="https://docs.aws.amazon.com/Route53/latest/APIReference/API_route53resolver_ListResolverEndpoints.html">ListResolverEndpoints</a>, <a href="https://docs.aws.amazon.com/Route53/latest/APIReference/API_route53resolver_ListResolverRules.html">ListResolverRules</a>, <a href="https://docs.aws.amazon.com/Route53/latest/APIReference/API_route53resolver_ListResolverRuleAssociations.html">ListResolverRuleAssociations</a>, <a href="https://docs.aws.amazon.com/Route53/latest/APIReference/API_route53resolver_ListResolverQueryLogConfigs.html">ListResolverQueryLogConfigs</a>, or <a href="https://docs.aws.amazon.com/Route53/latest/APIReference/API_route53resolver_ListResolverQueryLogConfigAssociations.html">ListResolverQueryLogConfigAssociations</a>.</p> <note> <p>In early versions of Resolver, values for <code>Name</code> were listed as uppercase, with underscore (<em>) delimiters. For example, <code>CreatorRequestId</code> was originally listed as <code>CREATOR</em>REQUEST<em>ID</code>. Uppercase values for <code>Name</code> are still supported.</p> </note> <p> <b>ListResolverEndpoints</b> </p> <p>Valid values for <code>Name</code> include the following:</p> <ul> <li> <p> <code>CreatorRequestId</code>: The value that you specified when you created the Resolver endpoint.</p> </li> <li> <p> <code>Direction</code>: Whether you want to return inbound or outbound Resolver endpoints. If you specify <code>DIRECTION</code> for <code>Name</code>, specify <code>INBOUND</code> or <code>OUTBOUND</code> for <code>Values</code>.</p> </li> <li> <p> <code>HostVpcId</code>: The ID of the VPC that inbound DNS queries pass through on the way from your network to your VPCs in a region, or the VPC that outbound queries pass through on the way from your VPCs to your network. In a &lt;a href=&quot;https://docs.aws.amazon.com/Route53/latest/APIReference/API</em>route53resolver<em>CreateResolverEndpoint.html&quot;&gt;CreateResolverEndpoint</a> request, <code>SubnetId</code> indirectly identifies the VPC. In a &lt;a href=&quot;https://docs.aws.amazon.com/Route53/latest/APIReference/API</em>route53resolver<em>GetResolverEndpoint.html&quot;&gt;GetResolverEndpoint</a> request, the VPC ID for a Resolver endpoint is returned in the <code>HostVPCId</code> element. </p> </li> <li> <p> <code>IpAddressCount</code>: The number of IP addresses that you have associated with the Resolver endpoint.</p> </li> <li> <p> <code>Name</code>: The name of the Resolver endpoint.</p> </li> <li> <p> <code>SecurityGroupIds</code>: The IDs of the VPC security groups that you specified when you created the Resolver endpoint.</p> </li> <li> <p> <code>Status</code>: The status of the Resolver endpoint. If you specify <code>Status</code> for <code>Name</code>, specify one of the following status codes for <code>Values</code>: <code>CREATING</code>, <code>OPERATIONAL</code>, <code>UPDATING</code>, <code>AUTO</em>RECOVERING</code>, <code>ACTION<em>NEEDED</code>, or <code>DELETING</code>. For more information, see <code>Status</code> in &lt;a href=&quot;https://docs.aws.amazon.com/Route53/latest/APIReference/API</em>route53resolver<em>ResolverEndpoint.html&quot;&gt;ResolverEndpoint</a>.</p> </li> </ul> <p> <b>ListResolverRules</b> </p> <p>Valid values for <code>Name</code> include the following:</p> <ul> <li> <p> <code>CreatorRequestId</code>: The value that you specified when you created the Resolver rule.</p> </li> <li> <p> <code>DomainName</code>: The domain name for which Resolver is forwarding DNS queries to your network. In the value that you specify for <code>Values</code>, include a trailing dot (.) after the domain name. For example, if the domain name is example.com, specify the following value. Note the &quot;.&quot; after <code>com</code>:</p> <p> <code>example.com.</code> </p> </li> <li> <p> <code>Name</code>: The name of the Resolver rule.</p> </li> <li> <p> <code>ResolverEndpointId</code>: The ID of the Resolver endpoint that the Resolver rule is associated with.</p> <note> <p>You can filter on the Resolver endpoint only for rules that have a value of <code>FORWARD</code> for <code>RuleType</code>.</p> </note> </li> <li> <p> <code>Status</code>: The status of the Resolver rule. If you specify <code>Status</code> for <code>Name</code>, specify one of the following status codes for <code>Values</code>: <code>COMPLETE</code>, <code>DELETING</code>, <code>UPDATING</code>, or <code>FAILED</code>.</p> </li> <li> <p> <code>Type</code>: The type of the Resolver rule. If you specify <code>TYPE</code> for <code>Name</code>, specify <code>FORWARD</code> or <code>SYSTEM</code> for <code>Values</code>.</p> </li> </ul> <p> <b>ListResolverRuleAssociations</b> </p> <p>Valid values for <code>Name</code> include the following:</p> <ul> <li> <p> <code>Name</code>: The name of the Resolver rule association.</p> </li> <li> <p> <code>ResolverRuleId</code>: The ID of the Resolver rule that is associated with one or more VPCs.</p> </li> <li> <p> <code>Status</code>: The status of the Resolver rule association. If you specify <code>Status</code> for <code>Name</code>, specify one of the following status codes for <code>Values</code>: <code>CREATING</code>, <code>COMPLETE</code>, <code>DELETING</code>, or <code>FAILED</code>.</p> </li> <li> <p> <code>VPCId</code>: The ID of the VPC that the Resolver rule is associated with.</p> </li> </ul> <p> <b>ListResolverQueryLogConfigs</b> </p> <p>Valid values for <code>Name</code> include the following:</p> <ul> <li> <p> <code>Arn</code>: The ARN for the query logging configuration.</p> </li> <li> <p> <code>AssociationCount</code>: The number of VPCs that are associated with the query logging configuration.</p> </li> <li> <p> <code>CreationTime</code>: The date and time that the query logging configuration was created, in Unix time format and Coordinated Universal Time (UTC). </p> </li> <li> <p> <code>CreatorRequestId</code>: A unique string that identifies the request that created the query logging configuration.</p> </li> <li> <p> <code>Destination</code>: The AWS service that you want to forward query logs to. Valid values include the following:</p> <ul> <li> <p> <code>S3</code> </p> </li> <li> <p> <code>CloudWatchLogs</code> </p> </li> <li> <p> <code>KinesisFirehose</code> </p> </li> </ul> </li> <li> <p> <code>DestinationArn</code>: The ARN of the location that Resolver is sending query logs to. This value can be the ARN for an S3 bucket, a CloudWatch Logs log group, or a Kinesis Data Firehose delivery stream.</p> </li> <li> <p> <code>Id</code>: The ID of the query logging configuration</p> </li> <li> <p> <code>Name</code>: The name of the query logging configuration</p> </li> <li> <p> <code>OwnerId</code>: The AWS account ID for the account that created the query logging configuration.</p> </li> <li> <p> <code>ShareStatus</code>: An indication of whether the query logging configuration is shared with other AWS accounts, or was shared with the current account by another AWS account. Valid values include: <code>NOT</em>SHARED</code>, <code>SHARED<em>WITH</em>ME</code>, or <code>SHARED<em>BY</em>ME</code>.</p> </li> <li> <p> <code>Status</code>: The status of the query logging configuration. If you specify <code>Status</code> for <code>Name</code>, specify the applicable status code for <code>Values</code>: <code>CREATING</code>, <code>CREATED</code>, <code>DELETING</code>, or <code>FAILED</code>. For more information, see <a href="https://docs.aws.amazon.com/Route53/latest/APIReference/API_route53resolver_ResolverQueryLogConfig.html#Route53Resolver-Type-route53resolver_ResolverQueryLogConfig-Status">Status</a>. </p> </li> </ul> <p> <b>ListResolverQueryLogConfigAssociations</b> </p> <p>Valid values for <code>Name</code> include the following:</p> <ul> <li> <p> <code>CreationTime</code>: The date and time that the VPC was associated with the query logging configuration, in Unix time format and Coordinated Universal Time (UTC).</p> </li> <li> <p> <code>Error</code>: If the value of <code>Status</code> is <code>FAILED</code>, specify the cause: <code>DESTINATION<em>NOT</em>FOUND</code> or <code>ACCESS<em>DENIED</code>.</p> </li> <li> <p> <code>Id</code>: The ID of the query logging association.</p> </li> <li> <p> <code>ResolverQueryLogConfigId</code>: The ID of the query logging configuration that a VPC is associated with.</p> </li> <li> <p> <code>ResourceId</code>: The ID of the Amazon VPC that is associated with the query logging configuration.</p> </li> <li> <p> <code>Status</code>: The status of the query logging association. If you specify <code>Status</code> for <code>Name</code>, specify the applicable status code for <code>Values</code>: <code>CREATING</code>, <code>CREATED</code>, <code>DELETING</code>, or <code>FAILED</code>. For more information, see &lt;a href=&quot;https://docs.aws.amazon.com/Route53/latest/APIReference/API</em>route53resolver<em>ResolverQueryLogConfigAssociation.html#Route53Resolver-Type-route53resolver</em>ResolverQueryLogConfigAssociation-Status&quot;&gt;Status</a>. </p> </li> </ul></p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>When you're using a <code>List</code> operation and you want the operation to return a subset of objects, such as Resolver endpoints or Resolver rules, the value of the parameter that you want to use to filter objects. For example, to list only inbound Resolver endpoints, specify <code>Direction</code> for <code>Name</code> and specify <code>INBOUND</code> for <code>Values</code>.</p>
    #[serde(rename = "Values")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub values: Option<Vec<String>>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetResolverDnssecConfigRequest {
    /// <p>The ID of the virtual private cloud (VPC) for the DNSSEC validation status.</p>
    #[serde(rename = "ResourceId")]
    pub resource_id: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetResolverDnssecConfigResponse {
    /// <p>The information about a configuration for DNSSEC validation.</p>
    #[serde(rename = "ResolverDNSSECConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resolver_dnssec_config: Option<ResolverDnssecConfig>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetResolverEndpointRequest {
    /// <p>The ID of the Resolver endpoint that you want to get information about.</p>
    #[serde(rename = "ResolverEndpointId")]
    pub resolver_endpoint_id: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetResolverEndpointResponse {
    /// <p>Information about the Resolver endpoint that you specified in a <code>GetResolverEndpoint</code> request.</p>
    #[serde(rename = "ResolverEndpoint")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resolver_endpoint: Option<ResolverEndpoint>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetResolverQueryLogConfigAssociationRequest {
    /// <p>The ID of the Resolver query logging configuration association that you want to get information about.</p>
    #[serde(rename = "ResolverQueryLogConfigAssociationId")]
    pub resolver_query_log_config_association_id: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetResolverQueryLogConfigAssociationResponse {
    /// <p>Information about the Resolver query logging configuration association that you specified in a <code>GetQueryLogConfigAssociation</code> request.</p>
    #[serde(rename = "ResolverQueryLogConfigAssociation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resolver_query_log_config_association: Option<ResolverQueryLogConfigAssociation>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetResolverQueryLogConfigPolicyRequest {
    /// <p>The ARN of the query logging configuration that you want to get the query logging policy for.</p>
    #[serde(rename = "Arn")]
    pub arn: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetResolverQueryLogConfigPolicyResponse {
    /// <p>Information about the query logging policy for the query logging configuration that you specified in a <code>GetResolverQueryLogConfigPolicy</code> request.</p>
    #[serde(rename = "ResolverQueryLogConfigPolicy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resolver_query_log_config_policy: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetResolverQueryLogConfigRequest {
    /// <p>The ID of the Resolver query logging configuration that you want to get information about.</p>
    #[serde(rename = "ResolverQueryLogConfigId")]
    pub resolver_query_log_config_id: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetResolverQueryLogConfigResponse {
    /// <p>Information about the Resolver query logging configuration that you specified in a <code>GetQueryLogConfig</code> request.</p>
    #[serde(rename = "ResolverQueryLogConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resolver_query_log_config: Option<ResolverQueryLogConfig>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetResolverRuleAssociationRequest {
    /// <p>The ID of the Resolver rule association that you want to get information about.</p>
    #[serde(rename = "ResolverRuleAssociationId")]
    pub resolver_rule_association_id: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetResolverRuleAssociationResponse {
    /// <p>Information about the Resolver rule association that you specified in a <code>GetResolverRuleAssociation</code> request.</p>
    #[serde(rename = "ResolverRuleAssociation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resolver_rule_association: Option<ResolverRuleAssociation>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetResolverRulePolicyRequest {
    /// <p>The ID of the Resolver rule that you want to get the Resolver rule policy for.</p>
    #[serde(rename = "Arn")]
    pub arn: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetResolverRulePolicyResponse {
    /// <p>The Resolver rule policy for the rule that you specified in a <code>GetResolverRulePolicy</code> request.</p>
    #[serde(rename = "ResolverRulePolicy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resolver_rule_policy: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetResolverRuleRequest {
    /// <p>The ID of the Resolver rule that you want to get information about.</p>
    #[serde(rename = "ResolverRuleId")]
    pub resolver_rule_id: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetResolverRuleResponse {
    /// <p>Information about the Resolver rule that you specified in a <code>GetResolverRule</code> request.</p>
    #[serde(rename = "ResolverRule")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resolver_rule: Option<ResolverRule>,
}

/// <p>In a <a href="https://docs.aws.amazon.com/Route53/latest/APIReference/API_route53resolver_CreateResolverEndpoint.html">CreateResolverEndpoint</a> request, the IP address that DNS queries originate from (for outbound endpoints) or that you forward DNS queries to (for inbound endpoints). <code>IpAddressRequest</code> also includes the ID of the subnet that contains the IP address.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct IpAddressRequest {
    /// <p>The IP address that you want to use for DNS queries.</p>
    #[serde(rename = "Ip")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip: Option<String>,
    /// <p>The ID of the subnet that contains the IP address. </p>
    #[serde(rename = "SubnetId")]
    pub subnet_id: String,
}

/// <p>In the response to a <a href="https://docs.aws.amazon.com/Route53/latest/APIReference/API_route53resolver_GetResolverEndpoint.html">GetResolverEndpoint</a> request, information about the IP addresses that the Resolver endpoint uses for DNS queries.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct IpAddressResponse {
    /// <p>The date and time that the IP address was created, in Unix time format and Coordinated Universal Time (UTC).</p>
    #[serde(rename = "CreationTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time: Option<String>,
    /// <p>One IP address that the Resolver endpoint uses for DNS queries.</p>
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

/// <p>In an <a href="https://docs.aws.amazon.com/Route53/latest/APIReference/API_route53resolver_UpdateResolverEndpoint.html">UpdateResolverEndpoint</a> request, information about an IP address to update.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct IpAddressUpdate {
    /// <p>The new IP address.</p>
    #[serde(rename = "Ip")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip: Option<String>,
    /// <p> <i>Only when removing an IP address from a Resolver endpoint</i>: The ID of the IP address that you want to remove. To get this ID, use <a href="https://docs.aws.amazon.com/Route53/latest/APIReference/API_route53resolver_GetResolverEndpoint.html">GetResolverEndpoint</a>.</p>
    #[serde(rename = "IpId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_id: Option<String>,
    /// <p>The ID of the subnet that includes the IP address that you want to update. To get this ID, use <a href="https://docs.aws.amazon.com/Route53/latest/APIReference/API_route53resolver_GetResolverEndpoint.html">GetResolverEndpoint</a>.</p>
    #[serde(rename = "SubnetId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subnet_id: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListResolverDnssecConfigsRequest {
    /// <p>An optional specification to return a subset of objects.</p>
    #[serde(rename = "Filters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<Vec<Filter>>,
    /// <p> <i>Optional</i>: An integer that specifies the maximum number of DNSSEC configuration results that you want Amazon Route 53 to return. If you don't specify a value for <code>MaxResults</code>, Route 53 returns up to 100 configuration per page.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>(Optional) If the current AWS account has more than <code>MaxResults</code> DNSSEC configurations, use <code>NextToken</code> to get the second and subsequent pages of results.</p> <p>For the first <code>ListResolverDnssecConfigs</code> request, omit this value.</p> <p>For the second and subsequent requests, get the value of <code>NextToken</code> from the previous response and specify that value for <code>NextToken</code> in the request.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListResolverDnssecConfigsResponse {
    /// <p>If a response includes the last of the DNSSEC configurations that are associated with the current AWS account, <code>NextToken</code> doesn't appear in the response.</p> <p>If a response doesn't include the last of the configurations, you can get more configurations by submitting another <a href="https://docs.aws.amazon.com/Route53/latest/APIReference/API_ListResolverDnssecConfigs.html">ListResolverDnssecConfigs</a> request. Get the value of <code>NextToken</code> that Amazon Route 53 returned in the previous response and include it in <code>NextToken</code> in the next request.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>An array that contains one <a href="https://docs.aws.amazon.com/Route53/latest/APIReference/API_ResolverDnssecConfig.html">ResolverDnssecConfig</a> element for each configuration for DNSSEC validation that is associated with the current AWS account.</p>
    #[serde(rename = "ResolverDnssecConfigs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resolver_dnssec_configs: Option<Vec<ResolverDnssecConfig>>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListResolverEndpointIpAddressesRequest {
    /// <p>The maximum number of IP addresses that you want to return in the response to a <code>ListResolverEndpointIpAddresses</code> request. If you don't specify a value for <code>MaxResults</code>, Resolver returns up to 100 IP addresses. </p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>For the first <code>ListResolverEndpointIpAddresses</code> request, omit this value.</p> <p>If the specified Resolver endpoint has more than <code>MaxResults</code> IP addresses, you can submit another <code>ListResolverEndpointIpAddresses</code> request to get the next group of IP addresses. In the next request, specify the value of <code>NextToken</code> from the previous response. </p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The ID of the Resolver endpoint that you want to get IP addresses for.</p>
    #[serde(rename = "ResolverEndpointId")]
    pub resolver_endpoint_id: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListResolverEndpointIpAddressesResponse {
    /// <p>Information about the IP addresses in your VPC that DNS queries originate from (for outbound endpoints) or that you forward DNS queries to (for inbound endpoints).</p>
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
    /// <p><p>An optional specification to return a subset of Resolver endpoints, such as all inbound Resolver endpoints.</p> <note> <p>If you submit a second or subsequent <code>ListResolverEndpoints</code> request and specify the <code>NextToken</code> parameter, you must use the same values for <code>Filters</code>, if any, as in the previous request.</p> </note></p>
    #[serde(rename = "Filters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<Vec<Filter>>,
    /// <p>The maximum number of Resolver endpoints that you want to return in the response to a <code>ListResolverEndpoints</code> request. If you don't specify a value for <code>MaxResults</code>, Resolver returns up to 100 Resolver endpoints. </p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>For the first <code>ListResolverEndpoints</code> request, omit this value.</p> <p>If you have more than <code>MaxResults</code> Resolver endpoints, you can submit another <code>ListResolverEndpoints</code> request to get the next group of Resolver endpoints. In the next request, specify the value of <code>NextToken</code> from the previous response. </p>
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
    /// <p>The Resolver endpoints that were created by using the current AWS account, and that match the specified filters, if any.</p>
    #[serde(rename = "ResolverEndpoints")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resolver_endpoints: Option<Vec<ResolverEndpoint>>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListResolverQueryLogConfigAssociationsRequest {
    /// <p><p>An optional specification to return a subset of query logging associations.</p> <note> <p>If you submit a second or subsequent <code>ListResolverQueryLogConfigAssociations</code> request and specify the <code>NextToken</code> parameter, you must use the same values for <code>Filters</code>, if any, as in the previous request.</p> </note></p>
    #[serde(rename = "Filters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<Vec<Filter>>,
    /// <p>The maximum number of query logging associations that you want to return in the response to a <code>ListResolverQueryLogConfigAssociations</code> request. If you don't specify a value for <code>MaxResults</code>, Resolver returns up to 100 query logging associations. </p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>For the first <code>ListResolverQueryLogConfigAssociations</code> request, omit this value.</p> <p>If there are more than <code>MaxResults</code> query logging associations that match the values that you specify for <code>Filters</code>, you can submit another <code>ListResolverQueryLogConfigAssociations</code> request to get the next group of associations. In the next request, specify the value of <code>NextToken</code> from the previous response. </p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p><p>The element that you want Resolver to sort query logging associations by. </p> <note> <p>If you submit a second or subsequent <code>ListResolverQueryLogConfigAssociations</code> request and specify the <code>NextToken</code> parameter, you must use the same value for <code>SortBy</code>, if any, as in the previous request.</p> </note> <p>Valid values include the following elements:</p> <ul> <li> <p> <code>CreationTime</code>: The ID of the query logging association.</p> </li> <li> <p> <code>Error</code>: If the value of <code>Status</code> is <code>FAILED</code>, the value of <code>Error</code> indicates the cause: </p> <ul> <li> <p> <code>DESTINATION<em>NOT</em>FOUND</code>: The specified destination (for example, an Amazon S3 bucket) was deleted.</p> </li> <li> <p> <code>ACCESS_DENIED</code>: Permissions don&#39;t allow sending logs to the destination.</p> </li> </ul> <p>If <code>Status</code> is a value other than <code>FAILED</code>, <code>ERROR</code> is null.</p> </li> <li> <p> <code>Id</code>: The ID of the query logging association</p> </li> <li> <p> <code>ResolverQueryLogConfigId</code>: The ID of the query logging configuration</p> </li> <li> <p> <code>ResourceId</code>: The ID of the VPC that is associated with the query logging configuration</p> </li> <li> <p> <code>Status</code>: The current status of the configuration. Valid values include the following:</p> <ul> <li> <p> <code>CREATING</code>: Resolver is creating an association between an Amazon VPC and a query logging configuration.</p> </li> <li> <p> <code>CREATED</code>: The association between an Amazon VPC and a query logging configuration was successfully created. Resolver is logging queries that originate in the specified VPC.</p> </li> <li> <p> <code>DELETING</code>: Resolver is deleting this query logging association.</p> </li> <li> <p> <code>FAILED</code>: Resolver either couldn&#39;t create or couldn&#39;t delete the query logging association. Here are two common causes:</p> <ul> <li> <p>The specified destination (for example, an Amazon S3 bucket) was deleted.</p> </li> <li> <p>Permissions don&#39;t allow sending logs to the destination.</p> </li> </ul> </li> </ul> </li> </ul></p>
    #[serde(rename = "SortBy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_by: Option<String>,
    /// <p><p>If you specified a value for <code>SortBy</code>, the order that you want query logging associations to be listed in, <code>ASCENDING</code> or <code>DESCENDING</code>.</p> <note> <p>If you submit a second or subsequent <code>ListResolverQueryLogConfigAssociations</code> request and specify the <code>NextToken</code> parameter, you must use the same value for <code>SortOrder</code>, if any, as in the previous request.</p> </note></p>
    #[serde(rename = "SortOrder")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_order: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListResolverQueryLogConfigAssociationsResponse {
    /// <p>If there are more than <code>MaxResults</code> query logging associations, you can submit another <code>ListResolverQueryLogConfigAssociations</code> request to get the next group of associations. In the next request, specify the value of <code>NextToken</code> from the previous response. </p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>A list that contains one <code>ResolverQueryLogConfigAssociations</code> element for each query logging association that matches the values that you specified for <code>Filter</code>.</p>
    #[serde(rename = "ResolverQueryLogConfigAssociations")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resolver_query_log_config_associations: Option<Vec<ResolverQueryLogConfigAssociation>>,
    /// <p>The total number of query logging associations that were created by the current account in the specified Region. This count can differ from the number of associations that are returned in a <code>ListResolverQueryLogConfigAssociations</code> response, depending on the values that you specify in the request.</p>
    #[serde(rename = "TotalCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_count: Option<i64>,
    /// <p>The total number of query logging associations that were created by the current account in the specified Region and that match the filters that were specified in the <code>ListResolverQueryLogConfigAssociations</code> request. For the total number of associations that were created by the current account in the specified Region, see <code>TotalCount</code>.</p>
    #[serde(rename = "TotalFilteredCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_filtered_count: Option<i64>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListResolverQueryLogConfigsRequest {
    /// <p><p>An optional specification to return a subset of query logging configurations.</p> <note> <p>If you submit a second or subsequent <code>ListResolverQueryLogConfigs</code> request and specify the <code>NextToken</code> parameter, you must use the same values for <code>Filters</code>, if any, as in the previous request.</p> </note></p>
    #[serde(rename = "Filters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<Vec<Filter>>,
    /// <p>The maximum number of query logging configurations that you want to return in the response to a <code>ListResolverQueryLogConfigs</code> request. If you don't specify a value for <code>MaxResults</code>, Resolver returns up to 100 query logging configurations. </p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>For the first <code>ListResolverQueryLogConfigs</code> request, omit this value.</p> <p>If there are more than <code>MaxResults</code> query logging configurations that match the values that you specify for <code>Filters</code>, you can submit another <code>ListResolverQueryLogConfigs</code> request to get the next group of configurations. In the next request, specify the value of <code>NextToken</code> from the previous response. </p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p><p>The element that you want Resolver to sort query logging configurations by. </p> <note> <p>If you submit a second or subsequent <code>ListResolverQueryLogConfigs</code> request and specify the <code>NextToken</code> parameter, you must use the same value for <code>SortBy</code>, if any, as in the previous request.</p> </note> <p>Valid values include the following elements:</p> <ul> <li> <p> <code>Arn</code>: The ARN of the query logging configuration</p> </li> <li> <p> <code>AssociationCount</code>: The number of VPCs that are associated with the specified configuration </p> </li> <li> <p> <code>CreationTime</code>: The date and time that Resolver returned when the configuration was created</p> </li> <li> <p> <code>CreatorRequestId</code>: The value that was specified for <code>CreatorRequestId</code> when the configuration was created</p> </li> <li> <p> <code>DestinationArn</code>: The location that logs are sent to</p> </li> <li> <p> <code>Id</code>: The ID of the configuration</p> </li> <li> <p> <code>Name</code>: The name of the configuration</p> </li> <li> <p> <code>OwnerId</code>: The AWS account number of the account that created the configuration</p> </li> <li> <p> <code>ShareStatus</code>: Whether the configuration is shared with other AWS accounts or shared with the current account by another AWS account. Sharing is configured through AWS Resource Access Manager (AWS RAM).</p> </li> <li> <p> <code>Status</code>: The current status of the configuration. Valid values include the following:</p> <ul> <li> <p> <code>CREATING</code>: Resolver is creating the query logging configuration.</p> </li> <li> <p> <code>CREATED</code>: The query logging configuration was successfully created. Resolver is logging queries that originate in the specified VPC.</p> </li> <li> <p> <code>DELETING</code>: Resolver is deleting this query logging configuration.</p> </li> <li> <p> <code>FAILED</code>: Resolver either couldn&#39;t create or couldn&#39;t delete the query logging configuration. Here are two common causes:</p> <ul> <li> <p>The specified destination (for example, an Amazon S3 bucket) was deleted.</p> </li> <li> <p>Permissions don&#39;t allow sending logs to the destination.</p> </li> </ul> </li> </ul> </li> </ul></p>
    #[serde(rename = "SortBy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_by: Option<String>,
    /// <p><p>If you specified a value for <code>SortBy</code>, the order that you want query logging configurations to be listed in, <code>ASCENDING</code> or <code>DESCENDING</code>.</p> <note> <p>If you submit a second or subsequent <code>ListResolverQueryLogConfigs</code> request and specify the <code>NextToken</code> parameter, you must use the same value for <code>SortOrder</code>, if any, as in the previous request.</p> </note></p>
    #[serde(rename = "SortOrder")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_order: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListResolverQueryLogConfigsResponse {
    /// <p>If there are more than <code>MaxResults</code> query logging configurations, you can submit another <code>ListResolverQueryLogConfigs</code> request to get the next group of configurations. In the next request, specify the value of <code>NextToken</code> from the previous response. </p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>A list that contains one <code>ResolverQueryLogConfig</code> element for each query logging configuration that matches the values that you specified for <code>Filter</code>.</p>
    #[serde(rename = "ResolverQueryLogConfigs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resolver_query_log_configs: Option<Vec<ResolverQueryLogConfig>>,
    /// <p>The total number of query logging configurations that were created by the current account in the specified Region. This count can differ from the number of query logging configurations that are returned in a <code>ListResolverQueryLogConfigs</code> response, depending on the values that you specify in the request.</p>
    #[serde(rename = "TotalCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_count: Option<i64>,
    /// <p>The total number of query logging configurations that were created by the current account in the specified Region and that match the filters that were specified in the <code>ListResolverQueryLogConfigs</code> request. For the total number of query logging configurations that were created by the current account in the specified Region, see <code>TotalCount</code>.</p>
    #[serde(rename = "TotalFilteredCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_filtered_count: Option<i64>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListResolverRuleAssociationsRequest {
    /// <p><p>An optional specification to return a subset of Resolver rules, such as Resolver rules that are associated with the same VPC ID.</p> <note> <p>If you submit a second or subsequent <code>ListResolverRuleAssociations</code> request and specify the <code>NextToken</code> parameter, you must use the same values for <code>Filters</code>, if any, as in the previous request.</p> </note></p>
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
    /// <p>The associations that were created between Resolver rules and VPCs using the current AWS account, and that match the specified filters, if any.</p>
    #[serde(rename = "ResolverRuleAssociations")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resolver_rule_associations: Option<Vec<ResolverRuleAssociation>>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListResolverRulesRequest {
    /// <p><p>An optional specification to return a subset of Resolver rules, such as all Resolver rules that are associated with the same Resolver endpoint.</p> <note> <p>If you submit a second or subsequent <code>ListResolverRules</code> request and specify the <code>NextToken</code> parameter, you must use the same values for <code>Filters</code>, if any, as in the previous request.</p> </note></p>
    #[serde(rename = "Filters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<Vec<Filter>>,
    /// <p>The maximum number of Resolver rules that you want to return in the response to a <code>ListResolverRules</code> request. If you don't specify a value for <code>MaxResults</code>, Resolver returns up to 100 Resolver rules.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>For the first <code>ListResolverRules</code> request, omit this value.</p> <p>If you have more than <code>MaxResults</code> Resolver rules, you can submit another <code>ListResolverRules</code> request to get the next group of Resolver rules. In the next request, specify the value of <code>NextToken</code> from the previous response. </p>
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
    /// <p>If more than <code>MaxResults</code> Resolver rules match the specified criteria, you can submit another <code>ListResolverRules</code> request to get the next group of results. In the next request, specify the value of <code>NextToken</code> from the previous response. </p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The Resolver rules that were created using the current AWS account and that match the specified filters, if any.</p>
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
pub struct PutResolverQueryLogConfigPolicyRequest {
    /// <p>The Amazon Resource Name (ARN) of the account that you want to share rules with.</p>
    #[serde(rename = "Arn")]
    pub arn: String,
    /// <p>An AWS Identity and Access Management policy statement that lists the query logging configurations that you want to share with another AWS account and the operations that you want the account to be able to perform. You can specify the following operations in the <code>Actions</code> section of the statement:</p> <ul> <li> <p> <code>route53resolver:AssociateResolverQueryLogConfig</code> </p> </li> <li> <p> <code>route53resolver:DisassociateResolverQueryLogConfig</code> </p> </li> <li> <p> <code>route53resolver:ListResolverQueryLogConfigAssociations</code> </p> </li> <li> <p> <code>route53resolver:ListResolverQueryLogConfigs</code> </p> </li> </ul> <p>In the <code>Resource</code> section of the statement, you specify the ARNs for the query logging configurations that you want to share with the account that you specified in <code>Arn</code>. </p>
    #[serde(rename = "ResolverQueryLogConfigPolicy")]
    pub resolver_query_log_config_policy: String,
}

/// <p>The response to a <code>PutResolverQueryLogConfigPolicy</code> request.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct PutResolverQueryLogConfigPolicyResponse {
    /// <p>Whether the <code>PutResolverQueryLogConfigPolicy</code> request was successful.</p>
    #[serde(rename = "ReturnValue")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub return_value: Option<bool>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct PutResolverRulePolicyRequest {
    /// <p>The Amazon Resource Name (ARN) of the rule that you want to share with another account.</p>
    #[serde(rename = "Arn")]
    pub arn: String,
    /// <p>An AWS Identity and Access Management policy statement that lists the rules that you want to share with another AWS account and the operations that you want the account to be able to perform. You can specify the following operations in the <code>Action</code> section of the statement:</p> <ul> <li> <p> <code>route53resolver:GetResolverRule</code> </p> </li> <li> <p> <code>route53resolver:AssociateResolverRule</code> </p> </li> <li> <p> <code>route53resolver:DisassociateResolverRule</code> </p> </li> <li> <p> <code>route53resolver:ListResolverRules</code> </p> </li> <li> <p> <code>route53resolver:ListResolverRuleAssociations</code> </p> </li> </ul> <p>In the <code>Resource</code> section of the statement, specify the ARN for the rule that you want to share with another account. Specify the same ARN that you specified in <code>Arn</code>.</p>
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

/// <p>A complex type that contains information about a configuration for DNSSEC validation.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ResolverDnssecConfig {
    /// <p>The ID for a configuration for DNSSEC validation.</p>
    #[serde(rename = "Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// <p>The owner account ID of the virtual private cloud (VPC) for a configuration for DNSSEC validation.</p>
    #[serde(rename = "OwnerId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner_id: Option<String>,
    /// <p>The ID of the virtual private cloud (VPC) that you're configuring the DNSSEC validation status for.</p>
    #[serde(rename = "ResourceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_id: Option<String>,
    /// <p><p>The validation status for a DNSSEC configuration. The status can be one of the following:</p> <ul> <li> <p> <b>ENABLING:</b> DNSSEC validation is being enabled but is not complete.</p> </li> <li> <p> <b>ENABLED:</b> DNSSEC validation is enabled.</p> </li> <li> <p> <b>DISABLING:</b> DNSSEC validation is being disabled but is not complete.</p> </li> <li> <p> <b>DISABLED</b> DNSSEC validation is disabled.</p> </li> </ul></p>
    #[serde(rename = "ValidationStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub validation_status: Option<String>,
}

/// <p>In the response to a <a href="https://docs.aws.amazon.com/Route53/latest/APIReference/API_route53resolver_CreateResolverEndpoint.html">CreateResolverEndpoint</a>, <a href="https://docs.aws.amazon.com/Route53/latest/APIReference/API_route53resolver_DeleteResolverEndpoint.html">DeleteResolverEndpoint</a>, <a href="https://docs.aws.amazon.com/Route53/latest/APIReference/API_route53resolver_GetResolverEndpoint.html">GetResolverEndpoint</a>, <a href="https://docs.aws.amazon.com/Route53/latest/APIReference/API_route53resolver_ListResolverEndpoints.html">ListResolverEndpoints</a>, or <a href="https://docs.aws.amazon.com/Route53/latest/APIReference/API_route53resolver_UpdateResolverEndpoint.html">UpdateResolverEndpoint</a> request, a complex type that contains settings for an existing inbound or outbound Resolver endpoint.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ResolverEndpoint {
    /// <p>The ARN (Amazon Resource Name) for the Resolver endpoint.</p>
    #[serde(rename = "Arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p>The date and time that the endpoint was created, in Unix time format and Coordinated Universal Time (UTC).</p>
    #[serde(rename = "CreationTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time: Option<String>,
    /// <p>A unique string that identifies the request that created the Resolver endpoint. The <code>CreatorRequestId</code> allows failed requests to be retried without the risk of executing the operation twice.</p>
    #[serde(rename = "CreatorRequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creator_request_id: Option<String>,
    /// <p><p>Indicates whether the Resolver endpoint allows inbound or outbound DNS queries:</p> <ul> <li> <p> <code>INBOUND</code>: allows DNS queries to your VPC from your network</p> </li> <li> <p> <code>OUTBOUND</code>: allows DNS queries from your VPC to your network</p> </li> </ul></p>
    #[serde(rename = "Direction")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub direction: Option<String>,
    /// <p>The ID of the VPC that you want to create the Resolver endpoint in.</p>
    #[serde(rename = "HostVPCId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub host_vpc_id: Option<String>,
    /// <p>The ID of the Resolver endpoint.</p>
    #[serde(rename = "Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// <p>The number of IP addresses that the Resolver endpoint can use for DNS queries.</p>
    #[serde(rename = "IpAddressCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_address_count: Option<i64>,
    /// <p>The date and time that the endpoint was last modified, in Unix time format and Coordinated Universal Time (UTC).</p>
    #[serde(rename = "ModificationTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modification_time: Option<String>,
    /// <p>The name that you assigned to the Resolver endpoint when you submitted a <a href="https://docs.aws.amazon.com/Route53/latest/APIReference/API_route53resolver_CreateResolverEndpoint.html">CreateResolverEndpoint</a> request.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The ID of one or more security groups that control access to this VPC. The security group must include one or more inbound rules (for inbound endpoints) or outbound rules (for outbound endpoints). Inbound and outbound rules must allow TCP and UDP access. For inbound access, open port 53. For outbound access, open the port that you're using for DNS queries on your network.</p>
    #[serde(rename = "SecurityGroupIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_group_ids: Option<Vec<String>>,
    /// <p><p>A code that specifies the current status of the Resolver endpoint. Valid values include the following:</p> <ul> <li> <p> <code>CREATING</code>: Resolver is creating and configuring one or more Amazon VPC network interfaces for this endpoint.</p> </li> <li> <p> <code>OPERATIONAL</code>: The Amazon VPC network interfaces for this endpoint are correctly configured and able to pass inbound or outbound DNS queries between your network and Resolver.</p> </li> <li> <p> <code>UPDATING</code>: Resolver is associating or disassociating one or more network interfaces with this endpoint.</p> </li> <li> <p> <code>AUTO<em>RECOVERING</code>: Resolver is trying to recover one or more of the network interfaces that are associated with this endpoint. During the recovery process, the endpoint functions with limited capacity because of the limit on the number of DNS queries per IP address (per network interface). For the current limit, see <a href="https://docs.aws.amazon.com/Route53/latest/DeveloperGuide/DNSLimitations.html#limits-api-entities-resolver">Limits on Route 53 Resolver</a>.</p> </li> <li> <p> <code>ACTION</em>NEEDED</code>: This endpoint is unhealthy, and Resolver can&#39;t automatically recover it. To resolve the problem, we recommend that you check each IP address that you associated with the endpoint. For each IP address that isn&#39;t available, add another IP address and then delete the IP address that isn&#39;t available. (An endpoint must always include at least two IP addresses.) A status of <code>ACTION_NEEDED</code> can have a variety of causes. Here are two common causes:</p> <ul> <li> <p>One or more of the network interfaces that are associated with the endpoint were deleted using Amazon VPC.</p> </li> <li> <p>The network interface couldn&#39;t be created for some reason that&#39;s outside the control of Resolver.</p> </li> </ul> </li> <li> <p> <code>DELETING</code>: Resolver is deleting this endpoint and the associated network interfaces.</p> </li> </ul></p>
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// <p>A detailed description of the status of the Resolver endpoint.</p>
    #[serde(rename = "StatusMessage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_message: Option<String>,
}

/// <p>In the response to a <a href="https://docs.aws.amazon.com/Route53/latest/APIReference/API_route53resolver_CreateResolverQueryLogConfig.html">CreateResolverQueryLogConfig</a>, <a href="https://docs.aws.amazon.com/Route53/latest/APIReference/API_route53resolver_DeleteResolverQueryLogConfig.html">DeleteResolverQueryLogConfig</a>, <a href="https://docs.aws.amazon.com/Route53/latest/APIReference/API_route53resolver_GetResolverQueryLogConfig.html">GetResolverQueryLogConfig</a>, or <a href="https://docs.aws.amazon.com/Route53/latest/APIReference/API_route53resolver_ListResolverQueryLogConfigs.html">ListResolverQueryLogConfigs</a> request, a complex type that contains settings for one query logging configuration.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ResolverQueryLogConfig {
    /// <p>The ARN for the query logging configuration.</p>
    #[serde(rename = "Arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p>The number of VPCs that are associated with the query logging configuration.</p>
    #[serde(rename = "AssociationCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub association_count: Option<i64>,
    /// <p>The date and time that the query logging configuration was created, in Unix time format and Coordinated Universal Time (UTC).</p>
    #[serde(rename = "CreationTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time: Option<String>,
    /// <p>A unique string that identifies the request that created the query logging configuration. The <code>CreatorRequestId</code> allows failed requests to be retried without the risk of executing the operation twice.</p>
    #[serde(rename = "CreatorRequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creator_request_id: Option<String>,
    /// <p>The ARN of the resource that you want Resolver to send query logs: an Amazon S3 bucket, a CloudWatch Logs log group, or a Kinesis Data Firehose delivery stream.</p>
    #[serde(rename = "DestinationArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination_arn: Option<String>,
    /// <p>The ID for the query logging configuration.</p>
    #[serde(rename = "Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// <p>The name of the query logging configuration. </p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The AWS account ID for the account that created the query logging configuration. </p>
    #[serde(rename = "OwnerId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner_id: Option<String>,
    /// <p>An indication of whether the query logging configuration is shared with other AWS accounts, or was shared with the current account by another AWS account. Sharing is configured through AWS Resource Access Manager (AWS RAM).</p>
    #[serde(rename = "ShareStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub share_status: Option<String>,
    /// <p><p>The status of the specified query logging configuration. Valid values include the following:</p> <ul> <li> <p> <code>CREATING</code>: Resolver is creating the query logging configuration.</p> </li> <li> <p> <code>CREATED</code>: The query logging configuration was successfully created. Resolver is logging queries that originate in the specified VPC.</p> </li> <li> <p> <code>DELETING</code>: Resolver is deleting this query logging configuration.</p> </li> <li> <p> <code>FAILED</code>: Resolver can&#39;t deliver logs to the location that is specified in the query logging configuration. Here are two common causes:</p> <ul> <li> <p>The specified destination (for example, an Amazon S3 bucket) was deleted.</p> </li> <li> <p>Permissions don&#39;t allow sending logs to the destination.</p> </li> </ul> </li> </ul></p>
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

/// <p>In the response to an <a href="https://docs.aws.amazon.com/Route53/latest/APIReference/API_route53resolver_AssociateResolverQueryLogConfig.html">AssociateResolverQueryLogConfig</a>, <a href="https://docs.aws.amazon.com/Route53/latest/APIReference/API_route53resolver_DisassociateResolverQueryLogConfig.html">DisassociateResolverQueryLogConfig</a>, <a href="https://docs.aws.amazon.com/Route53/latest/APIReference/API_route53resolver_GetResolverQueryLogConfigAssociation.html">GetResolverQueryLogConfigAssociation</a>, or <a href="https://docs.aws.amazon.com/Route53/latest/APIReference/API_route53resolver_ListResolverQueryLogConfigAssociations.html">ListResolverQueryLogConfigAssociations</a>, request, a complex type that contains settings for a specified association between an Amazon VPC and a query logging configuration.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ResolverQueryLogConfigAssociation {
    /// <p>The date and time that the VPC was associated with the query logging configuration, in Unix time format and Coordinated Universal Time (UTC).</p>
    #[serde(rename = "CreationTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time: Option<String>,
    /// <p>If the value of <code>Status</code> is <code>FAILED</code>, the value of <code>Error</code> indicates the cause:</p> <ul> <li> <p> <code>DESTINATION_NOT_FOUND</code>: The specified destination (for example, an Amazon S3 bucket) was deleted.</p> </li> <li> <p> <code>ACCESS_DENIED</code>: Permissions don't allow sending logs to the destination.</p> </li> </ul> <p>If the value of <code>Status</code> is a value other than <code>FAILED</code>, <code>Error</code> is null. </p>
    #[serde(rename = "Error")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
    /// <p>Contains additional information about the error. If the value or <code>Error</code> is null, the value of <code>ErrorMessage</code> also is null.</p>
    #[serde(rename = "ErrorMessage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
    /// <p>The ID of the query logging association.</p>
    #[serde(rename = "Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// <p>The ID of the query logging configuration that a VPC is associated with.</p>
    #[serde(rename = "ResolverQueryLogConfigId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resolver_query_log_config_id: Option<String>,
    /// <p>The ID of the Amazon VPC that is associated with the query logging configuration.</p>
    #[serde(rename = "ResourceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_id: Option<String>,
    /// <p><p>The status of the specified query logging association. Valid values include the following:</p> <ul> <li> <p> <code>CREATING</code>: Resolver is creating an association between an Amazon VPC and a query logging configuration.</p> </li> <li> <p> <code>CREATED</code>: The association between an Amazon VPC and a query logging configuration was successfully created. Resolver is logging queries that originate in the specified VPC.</p> </li> <li> <p> <code>DELETING</code>: Resolver is deleting this query logging association.</p> </li> <li> <p> <code>FAILED</code>: Resolver either couldn&#39;t create or couldn&#39;t delete the query logging association.</p> </li> </ul></p>
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

/// <p>For queries that originate in your VPC, detailed information about a Resolver rule, which specifies how to route DNS queries out of the VPC. The <code>ResolverRule</code> parameter appears in the response to a <a href="https://docs.aws.amazon.com/Route53/latest/APIReference/API_route53resolver_CreateResolverRule.html">CreateResolverRule</a>, <a href="https://docs.aws.amazon.com/Route53/latest/APIReference/API_route53resolver_DeleteResolverRule.html">DeleteResolverRule</a>, <a href="https://docs.aws.amazon.com/Route53/latest/APIReference/API_route53resolver_GetResolverRule.html">GetResolverRule</a>, <a href="https://docs.aws.amazon.com/Route53/latest/APIReference/API_route53resolver_ListResolverRules.html">ListResolverRules</a>, or <a href="https://docs.aws.amazon.com/Route53/latest/APIReference/API_route53resolver_UpdateResolverRule.html">UpdateResolverRule</a> request.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ResolverRule {
    /// <p>The ARN (Amazon Resource Name) for the Resolver rule specified by <code>Id</code>.</p>
    #[serde(rename = "Arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p>The date and time that the Resolver rule was created, in Unix time format and Coordinated Universal Time (UTC).</p>
    #[serde(rename = "CreationTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time: Option<String>,
    /// <p>A unique string that you specified when you created the Resolver rule. <code>CreatorRequestId</code> identifies the request and allows failed requests to be retried without the risk of executing the operation twice. </p>
    #[serde(rename = "CreatorRequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creator_request_id: Option<String>,
    /// <p>DNS queries for this domain name are forwarded to the IP addresses that are specified in <code>TargetIps</code>. If a query matches multiple Resolver rules (example.com and www.example.com), the query is routed using the Resolver rule that contains the most specific domain name (www.example.com).</p>
    #[serde(rename = "DomainName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_name: Option<String>,
    /// <p>The ID that Resolver assigned to the Resolver rule when you created it.</p>
    #[serde(rename = "Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// <p>The date and time that the Resolver rule was last updated, in Unix time format and Coordinated Universal Time (UTC).</p>
    #[serde(rename = "ModificationTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modification_time: Option<String>,
    /// <p>The name for the Resolver rule, which you specified when you created the Resolver rule.</p>
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
    /// <p>When you want to forward DNS queries for specified domain name to resolvers on your network, specify <code>FORWARD</code>.</p> <p>When you have a forwarding rule to forward DNS queries for a domain to your network and you want Resolver to process queries for a subdomain of that domain, specify <code>SYSTEM</code>.</p> <p>For example, to forward DNS queries for example.com to resolvers on your network, you create a rule and specify <code>FORWARD</code> for <code>RuleType</code>. To then have Resolver process queries for apex.example.com, you create a rule and specify <code>SYSTEM</code> for <code>RuleType</code>.</p> <p>Currently, only Resolver can create rules that have a value of <code>RECURSIVE</code> for <code>RuleType</code>.</p>
    #[serde(rename = "RuleType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rule_type: Option<String>,
    /// <p>Whether the rules is shared and, if so, whether the current account is sharing the rule with another account, or another account is sharing the rule with the current account.</p>
    #[serde(rename = "ShareStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub share_status: Option<String>,
    /// <p>A code that specifies the current status of the Resolver rule.</p>
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// <p>A detailed description of the status of a Resolver rule.</p>
    #[serde(rename = "StatusMessage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_message: Option<String>,
    /// <p>An array that contains the IP addresses and ports that an outbound endpoint forwards DNS queries to. Typically, these are the IP addresses of DNS resolvers on your network. Specify IPv4 addresses. IPv6 is not supported.</p>
    #[serde(rename = "TargetIps")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_ips: Option<Vec<TargetAddress>>,
}

/// <p>In the response to an <a href="https://docs.aws.amazon.com/Route53/latest/APIReference/API_route53resolver_AssociateResolverRule.html">AssociateResolverRule</a>, <a href="https://docs.aws.amazon.com/Route53/latest/APIReference/API_route53resolver_DisassociateResolverRule.html">DisassociateResolverRule</a>, or <a href="https://docs.aws.amazon.com/Route53/latest/APIReference/API_route53resolver_ListResolverRuleAssociations.html">ListResolverRuleAssociations</a> request, provides information about an association between a Resolver rule and a VPC. The association determines which DNS queries that originate in the VPC are forwarded to your network. </p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ResolverRuleAssociation {
    /// <p>The ID of the association between a Resolver rule and a VPC. Resolver assigns this value when you submit an <a href="https://docs.aws.amazon.com/Route53/latest/APIReference/API_route53resolver_AssociateResolverRule.html">AssociateResolverRule</a> request.</p>
    #[serde(rename = "Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// <p>The name of an association between a Resolver rule and a VPC.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The ID of the Resolver rule that you associated with the VPC that is specified by <code>VPCId</code>.</p>
    #[serde(rename = "ResolverRuleId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resolver_rule_id: Option<String>,
    /// <p>A code that specifies the current status of the association between a Resolver rule and a VPC.</p>
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// <p>A detailed description of the status of the association between a Resolver rule and a VPC.</p>
    #[serde(rename = "StatusMessage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_message: Option<String>,
    /// <p>The ID of the VPC that you associated the Resolver rule with.</p>
    #[serde(rename = "VPCId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_id: Option<String>,
}

/// <p>In an <a href="https://docs.aws.amazon.com/Route53/latest/APIReference/API_route53resolver_UpdateResolverRule.html">UpdateResolverRule</a> request, information about the changes that you want to make.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ResolverRuleConfig {
    /// <p>The new name for the Resolver rule. The name that you specify appears in the Resolver dashboard in the Route 53 console. </p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The ID of the new outbound Resolver endpoint that you want to use to route DNS queries to the IP addresses that you specify in <code>TargetIps</code>.</p>
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
    pub key: String,
    /// <p>The value for the tag. For example, if <code>Key</code> is <code>account-id</code>, then <code>Value</code> might be the ID of the customer account that you're creating the resource for.</p>
    #[serde(rename = "Value")]
    pub value: String,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct TagResourceRequest {
    /// <p><p>The Amazon Resource Name (ARN) for the resource that you want to add tags to. To get the ARN for a resource, use the applicable <code>Get</code> or <code>List</code> command: </p> <ul> <li> <p> <a href="https://docs.aws.amazon.com/Route53/latest/APIReference/API_route53resolver_GetResolverEndpoint.html">GetResolverEndpoint</a> </p> </li> <li> <p> <a href="https://docs.aws.amazon.com/Route53/latest/APIReference/API_route53resolver_GetResolverRule.html">GetResolverRule</a> </p> </li> <li> <p> <a href="https://docs.aws.amazon.com/Route53/latest/APIReference/API_route53resolver_GetResolverRuleAssociation.html">GetResolverRuleAssociation</a> </p> </li> <li> <p> <a href="https://docs.aws.amazon.com/Route53/latest/APIReference/API_route53resolver_ListResolverEndpoints.html">ListResolverEndpoints</a> </p> </li> <li> <p> <a href="https://docs.aws.amazon.com/Route53/latest/APIReference/API_route53resolver_ListResolverRuleAssociations.html">ListResolverRuleAssociations</a> </p> </li> <li> <p> <a href="https://docs.aws.amazon.com/Route53/latest/APIReference/API_route53resolver_ListResolverRules.html">ListResolverRules</a> </p> </li> </ul></p>
    #[serde(rename = "ResourceArn")]
    pub resource_arn: String,
    /// <p>The tags that you want to add to the specified resource.</p>
    #[serde(rename = "Tags")]
    pub tags: Vec<Tag>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct TagResourceResponse {}

/// <p>In a <a href="https://docs.aws.amazon.com/Route53/latest/APIReference/API_route53resolver_CreateResolverRule.html">CreateResolverRule</a> request, an array of the IPs that you want to forward DNS queries to.</p>
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
    /// <p><p>The Amazon Resource Name (ARN) for the resource that you want to remove tags from. To get the ARN for a resource, use the applicable <code>Get</code> or <code>List</code> command: </p> <ul> <li> <p> <a href="https://docs.aws.amazon.com/Route53/latest/APIReference/API_route53resolver_GetResolverEndpoint.html">GetResolverEndpoint</a> </p> </li> <li> <p> <a href="https://docs.aws.amazon.com/Route53/latest/APIReference/API_route53resolver_GetResolverRule.html">GetResolverRule</a> </p> </li> <li> <p> <a href="https://docs.aws.amazon.com/Route53/latest/APIReference/API_route53resolver_GetResolverRuleAssociation.html">GetResolverRuleAssociation</a> </p> </li> <li> <p> <a href="https://docs.aws.amazon.com/Route53/latest/APIReference/API_route53resolver_ListResolverEndpoints.html">ListResolverEndpoints</a> </p> </li> <li> <p> <a href="https://docs.aws.amazon.com/Route53/latest/APIReference/API_route53resolver_ListResolverRuleAssociations.html">ListResolverRuleAssociations</a> </p> </li> <li> <p> <a href="https://docs.aws.amazon.com/Route53/latest/APIReference/API_route53resolver_ListResolverRules.html">ListResolverRules</a> </p> </li> </ul></p>
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
pub struct UpdateResolverDnssecConfigRequest {
    /// <p>The ID of the virtual private cloud (VPC) that you're updating the DNSSEC validation status for.</p>
    #[serde(rename = "ResourceId")]
    pub resource_id: String,
    /// <p>The new value that you are specifying for DNSSEC validation for the VPC. The value can be <code>ENABLE</code> or <code>DISABLE</code>. Be aware that it can take time for a validation status change to be completed.</p>
    #[serde(rename = "Validation")]
    pub validation: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UpdateResolverDnssecConfigResponse {
    /// <p>A complex type that contains settings for the specified DNSSEC configuration.</p>
    #[serde(rename = "ResolverDNSSECConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resolver_dnssec_config: Option<ResolverDnssecConfig>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UpdateResolverEndpointRequest {
    /// <p>The name of the Resolver endpoint that you want to update.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The ID of the Resolver endpoint that you want to update.</p>
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
    /// <p>The new settings for the Resolver rule.</p>
    #[serde(rename = "Config")]
    pub config: ResolverRuleConfig,
    /// <p>The ID of the Resolver rule that you want to update.</p>
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

    fn refine(
        err: RusotoError<std::convert::Infallible>,
    ) -> RusotoError<AssociateResolverEndpointIpAddressError> {
        match err {
            RusotoError::Service(err) => match err {},
            RusotoError::HttpDispatch(err) => RusotoError::HttpDispatch(err),
            RusotoError::Credentials(err) => RusotoError::Credentials(err),
            RusotoError::Validation(err) => RusotoError::Validation(err),
            RusotoError::ParseError(err) => RusotoError::ParseError(err),
            RusotoError::Unknown(res) => Self::from_response(res),
            RusotoError::Blocking => RusotoError::Blocking,
        }
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
/// Errors returned by AssociateResolverQueryLogConfig
#[derive(Debug, PartialEq)]
pub enum AssociateResolverQueryLogConfigError {
    /// <p>The current account doesn't have the IAM permissions required to perform the specified Resolver operation.</p>
    AccessDenied(String),
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

impl AssociateResolverQueryLogConfigError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<AssociateResolverQueryLogConfigError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(
                        AssociateResolverQueryLogConfigError::AccessDenied(err.msg),
                    )
                }
                "InternalServiceErrorException" => {
                    return RusotoError::Service(
                        AssociateResolverQueryLogConfigError::InternalServiceError(err.msg),
                    )
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(
                        AssociateResolverQueryLogConfigError::InvalidParameter(err.msg),
                    )
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(
                        AssociateResolverQueryLogConfigError::InvalidRequest(err.msg),
                    )
                }
                "LimitExceededException" => {
                    return RusotoError::Service(
                        AssociateResolverQueryLogConfigError::LimitExceeded(err.msg),
                    )
                }
                "ResourceExistsException" => {
                    return RusotoError::Service(
                        AssociateResolverQueryLogConfigError::ResourceExists(err.msg),
                    )
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(
                        AssociateResolverQueryLogConfigError::ResourceNotFound(err.msg),
                    )
                }
                "ThrottlingException" => {
                    return RusotoError::Service(AssociateResolverQueryLogConfigError::Throttling(
                        err.msg,
                    ))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }

    fn refine(
        err: RusotoError<std::convert::Infallible>,
    ) -> RusotoError<AssociateResolverQueryLogConfigError> {
        match err {
            RusotoError::Service(err) => match err {},
            RusotoError::HttpDispatch(err) => RusotoError::HttpDispatch(err),
            RusotoError::Credentials(err) => RusotoError::Credentials(err),
            RusotoError::Validation(err) => RusotoError::Validation(err),
            RusotoError::ParseError(err) => RusotoError::ParseError(err),
            RusotoError::Unknown(res) => Self::from_response(res),
            RusotoError::Blocking => RusotoError::Blocking,
        }
    }
}
impl fmt::Display for AssociateResolverQueryLogConfigError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            AssociateResolverQueryLogConfigError::AccessDenied(ref cause) => write!(f, "{}", cause),
            AssociateResolverQueryLogConfigError::InternalServiceError(ref cause) => {
                write!(f, "{}", cause)
            }
            AssociateResolverQueryLogConfigError::InvalidParameter(ref cause) => {
                write!(f, "{}", cause)
            }
            AssociateResolverQueryLogConfigError::InvalidRequest(ref cause) => {
                write!(f, "{}", cause)
            }
            AssociateResolverQueryLogConfigError::LimitExceeded(ref cause) => {
                write!(f, "{}", cause)
            }
            AssociateResolverQueryLogConfigError::ResourceExists(ref cause) => {
                write!(f, "{}", cause)
            }
            AssociateResolverQueryLogConfigError::ResourceNotFound(ref cause) => {
                write!(f, "{}", cause)
            }
            AssociateResolverQueryLogConfigError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for AssociateResolverQueryLogConfigError {}
/// Errors returned by AssociateResolverRule
#[derive(Debug, PartialEq)]
pub enum AssociateResolverRuleError {
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
                "LimitExceededException" => {
                    return RusotoError::Service(AssociateResolverRuleError::LimitExceeded(err.msg))
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

    fn refine(
        err: RusotoError<std::convert::Infallible>,
    ) -> RusotoError<AssociateResolverRuleError> {
        match err {
            RusotoError::Service(err) => match err {},
            RusotoError::HttpDispatch(err) => RusotoError::HttpDispatch(err),
            RusotoError::Credentials(err) => RusotoError::Credentials(err),
            RusotoError::Validation(err) => RusotoError::Validation(err),
            RusotoError::ParseError(err) => RusotoError::ParseError(err),
            RusotoError::Unknown(res) => Self::from_response(res),
            RusotoError::Blocking => RusotoError::Blocking,
        }
    }
}
impl fmt::Display for AssociateResolverRuleError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            AssociateResolverRuleError::InternalServiceError(ref cause) => write!(f, "{}", cause),
            AssociateResolverRuleError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            AssociateResolverRuleError::InvalidRequest(ref cause) => write!(f, "{}", cause),
            AssociateResolverRuleError::LimitExceeded(ref cause) => write!(f, "{}", cause),
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

    fn refine(
        err: RusotoError<std::convert::Infallible>,
    ) -> RusotoError<CreateResolverEndpointError> {
        match err {
            RusotoError::Service(err) => match err {},
            RusotoError::HttpDispatch(err) => RusotoError::HttpDispatch(err),
            RusotoError::Credentials(err) => RusotoError::Credentials(err),
            RusotoError::Validation(err) => RusotoError::Validation(err),
            RusotoError::ParseError(err) => RusotoError::ParseError(err),
            RusotoError::Unknown(res) => Self::from_response(res),
            RusotoError::Blocking => RusotoError::Blocking,
        }
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
/// Errors returned by CreateResolverQueryLogConfig
#[derive(Debug, PartialEq)]
pub enum CreateResolverQueryLogConfigError {
    /// <p>The current account doesn't have the IAM permissions required to perform the specified Resolver operation.</p>
    AccessDenied(String),
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

impl CreateResolverQueryLogConfigError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<CreateResolverQueryLogConfigError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(CreateResolverQueryLogConfigError::AccessDenied(
                        err.msg,
                    ))
                }
                "InternalServiceErrorException" => {
                    return RusotoError::Service(
                        CreateResolverQueryLogConfigError::InternalServiceError(err.msg),
                    )
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(
                        CreateResolverQueryLogConfigError::InvalidParameter(err.msg),
                    )
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(CreateResolverQueryLogConfigError::InvalidRequest(
                        err.msg,
                    ))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(CreateResolverQueryLogConfigError::LimitExceeded(
                        err.msg,
                    ))
                }
                "ResourceExistsException" => {
                    return RusotoError::Service(CreateResolverQueryLogConfigError::ResourceExists(
                        err.msg,
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(
                        CreateResolverQueryLogConfigError::ResourceNotFound(err.msg),
                    )
                }
                "ThrottlingException" => {
                    return RusotoError::Service(CreateResolverQueryLogConfigError::Throttling(
                        err.msg,
                    ))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }

    fn refine(
        err: RusotoError<std::convert::Infallible>,
    ) -> RusotoError<CreateResolverQueryLogConfigError> {
        match err {
            RusotoError::Service(err) => match err {},
            RusotoError::HttpDispatch(err) => RusotoError::HttpDispatch(err),
            RusotoError::Credentials(err) => RusotoError::Credentials(err),
            RusotoError::Validation(err) => RusotoError::Validation(err),
            RusotoError::ParseError(err) => RusotoError::ParseError(err),
            RusotoError::Unknown(res) => Self::from_response(res),
            RusotoError::Blocking => RusotoError::Blocking,
        }
    }
}
impl fmt::Display for CreateResolverQueryLogConfigError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreateResolverQueryLogConfigError::AccessDenied(ref cause) => write!(f, "{}", cause),
            CreateResolverQueryLogConfigError::InternalServiceError(ref cause) => {
                write!(f, "{}", cause)
            }
            CreateResolverQueryLogConfigError::InvalidParameter(ref cause) => {
                write!(f, "{}", cause)
            }
            CreateResolverQueryLogConfigError::InvalidRequest(ref cause) => write!(f, "{}", cause),
            CreateResolverQueryLogConfigError::LimitExceeded(ref cause) => write!(f, "{}", cause),
            CreateResolverQueryLogConfigError::ResourceExists(ref cause) => write!(f, "{}", cause),
            CreateResolverQueryLogConfigError::ResourceNotFound(ref cause) => {
                write!(f, "{}", cause)
            }
            CreateResolverQueryLogConfigError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CreateResolverQueryLogConfigError {}
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

    fn refine(err: RusotoError<std::convert::Infallible>) -> RusotoError<CreateResolverRuleError> {
        match err {
            RusotoError::Service(err) => match err {},
            RusotoError::HttpDispatch(err) => RusotoError::HttpDispatch(err),
            RusotoError::Credentials(err) => RusotoError::Credentials(err),
            RusotoError::Validation(err) => RusotoError::Validation(err),
            RusotoError::ParseError(err) => RusotoError::ParseError(err),
            RusotoError::Unknown(res) => Self::from_response(res),
            RusotoError::Blocking => RusotoError::Blocking,
        }
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

    fn refine(
        err: RusotoError<std::convert::Infallible>,
    ) -> RusotoError<DeleteResolverEndpointError> {
        match err {
            RusotoError::Service(err) => match err {},
            RusotoError::HttpDispatch(err) => RusotoError::HttpDispatch(err),
            RusotoError::Credentials(err) => RusotoError::Credentials(err),
            RusotoError::Validation(err) => RusotoError::Validation(err),
            RusotoError::ParseError(err) => RusotoError::ParseError(err),
            RusotoError::Unknown(res) => Self::from_response(res),
            RusotoError::Blocking => RusotoError::Blocking,
        }
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
/// Errors returned by DeleteResolverQueryLogConfig
#[derive(Debug, PartialEq)]
pub enum DeleteResolverQueryLogConfigError {
    /// <p>The current account doesn't have the IAM permissions required to perform the specified Resolver operation.</p>
    AccessDenied(String),
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

impl DeleteResolverQueryLogConfigError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<DeleteResolverQueryLogConfigError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(DeleteResolverQueryLogConfigError::AccessDenied(
                        err.msg,
                    ))
                }
                "InternalServiceErrorException" => {
                    return RusotoError::Service(
                        DeleteResolverQueryLogConfigError::InternalServiceError(err.msg),
                    )
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(
                        DeleteResolverQueryLogConfigError::InvalidParameter(err.msg),
                    )
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(DeleteResolverQueryLogConfigError::InvalidRequest(
                        err.msg,
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(
                        DeleteResolverQueryLogConfigError::ResourceNotFound(err.msg),
                    )
                }
                "ThrottlingException" => {
                    return RusotoError::Service(DeleteResolverQueryLogConfigError::Throttling(
                        err.msg,
                    ))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }

    fn refine(
        err: RusotoError<std::convert::Infallible>,
    ) -> RusotoError<DeleteResolverQueryLogConfigError> {
        match err {
            RusotoError::Service(err) => match err {},
            RusotoError::HttpDispatch(err) => RusotoError::HttpDispatch(err),
            RusotoError::Credentials(err) => RusotoError::Credentials(err),
            RusotoError::Validation(err) => RusotoError::Validation(err),
            RusotoError::ParseError(err) => RusotoError::ParseError(err),
            RusotoError::Unknown(res) => Self::from_response(res),
            RusotoError::Blocking => RusotoError::Blocking,
        }
    }
}
impl fmt::Display for DeleteResolverQueryLogConfigError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteResolverQueryLogConfigError::AccessDenied(ref cause) => write!(f, "{}", cause),
            DeleteResolverQueryLogConfigError::InternalServiceError(ref cause) => {
                write!(f, "{}", cause)
            }
            DeleteResolverQueryLogConfigError::InvalidParameter(ref cause) => {
                write!(f, "{}", cause)
            }
            DeleteResolverQueryLogConfigError::InvalidRequest(ref cause) => write!(f, "{}", cause),
            DeleteResolverQueryLogConfigError::ResourceNotFound(ref cause) => {
                write!(f, "{}", cause)
            }
            DeleteResolverQueryLogConfigError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeleteResolverQueryLogConfigError {}
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

    fn refine(err: RusotoError<std::convert::Infallible>) -> RusotoError<DeleteResolverRuleError> {
        match err {
            RusotoError::Service(err) => match err {},
            RusotoError::HttpDispatch(err) => RusotoError::HttpDispatch(err),
            RusotoError::Credentials(err) => RusotoError::Credentials(err),
            RusotoError::Validation(err) => RusotoError::Validation(err),
            RusotoError::ParseError(err) => RusotoError::ParseError(err),
            RusotoError::Unknown(res) => Self::from_response(res),
            RusotoError::Blocking => RusotoError::Blocking,
        }
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

    fn refine(
        err: RusotoError<std::convert::Infallible>,
    ) -> RusotoError<DisassociateResolverEndpointIpAddressError> {
        match err {
            RusotoError::Service(err) => match err {},
            RusotoError::HttpDispatch(err) => RusotoError::HttpDispatch(err),
            RusotoError::Credentials(err) => RusotoError::Credentials(err),
            RusotoError::Validation(err) => RusotoError::Validation(err),
            RusotoError::ParseError(err) => RusotoError::ParseError(err),
            RusotoError::Unknown(res) => Self::from_response(res),
            RusotoError::Blocking => RusotoError::Blocking,
        }
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
/// Errors returned by DisassociateResolverQueryLogConfig
#[derive(Debug, PartialEq)]
pub enum DisassociateResolverQueryLogConfigError {
    /// <p>The current account doesn't have the IAM permissions required to perform the specified Resolver operation.</p>
    AccessDenied(String),
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

impl DisassociateResolverQueryLogConfigError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<DisassociateResolverQueryLogConfigError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(
                        DisassociateResolverQueryLogConfigError::AccessDenied(err.msg),
                    )
                }
                "InternalServiceErrorException" => {
                    return RusotoError::Service(
                        DisassociateResolverQueryLogConfigError::InternalServiceError(err.msg),
                    )
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(
                        DisassociateResolverQueryLogConfigError::InvalidParameter(err.msg),
                    )
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(
                        DisassociateResolverQueryLogConfigError::InvalidRequest(err.msg),
                    )
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(
                        DisassociateResolverQueryLogConfigError::ResourceNotFound(err.msg),
                    )
                }
                "ThrottlingException" => {
                    return RusotoError::Service(
                        DisassociateResolverQueryLogConfigError::Throttling(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }

    fn refine(
        err: RusotoError<std::convert::Infallible>,
    ) -> RusotoError<DisassociateResolverQueryLogConfigError> {
        match err {
            RusotoError::Service(err) => match err {},
            RusotoError::HttpDispatch(err) => RusotoError::HttpDispatch(err),
            RusotoError::Credentials(err) => RusotoError::Credentials(err),
            RusotoError::Validation(err) => RusotoError::Validation(err),
            RusotoError::ParseError(err) => RusotoError::ParseError(err),
            RusotoError::Unknown(res) => Self::from_response(res),
            RusotoError::Blocking => RusotoError::Blocking,
        }
    }
}
impl fmt::Display for DisassociateResolverQueryLogConfigError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DisassociateResolverQueryLogConfigError::AccessDenied(ref cause) => {
                write!(f, "{}", cause)
            }
            DisassociateResolverQueryLogConfigError::InternalServiceError(ref cause) => {
                write!(f, "{}", cause)
            }
            DisassociateResolverQueryLogConfigError::InvalidParameter(ref cause) => {
                write!(f, "{}", cause)
            }
            DisassociateResolverQueryLogConfigError::InvalidRequest(ref cause) => {
                write!(f, "{}", cause)
            }
            DisassociateResolverQueryLogConfigError::ResourceNotFound(ref cause) => {
                write!(f, "{}", cause)
            }
            DisassociateResolverQueryLogConfigError::Throttling(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for DisassociateResolverQueryLogConfigError {}
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

    fn refine(
        err: RusotoError<std::convert::Infallible>,
    ) -> RusotoError<DisassociateResolverRuleError> {
        match err {
            RusotoError::Service(err) => match err {},
            RusotoError::HttpDispatch(err) => RusotoError::HttpDispatch(err),
            RusotoError::Credentials(err) => RusotoError::Credentials(err),
            RusotoError::Validation(err) => RusotoError::Validation(err),
            RusotoError::ParseError(err) => RusotoError::ParseError(err),
            RusotoError::Unknown(res) => Self::from_response(res),
            RusotoError::Blocking => RusotoError::Blocking,
        }
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
/// Errors returned by GetResolverDnssecConfig
#[derive(Debug, PartialEq)]
pub enum GetResolverDnssecConfigError {
    /// <p>The current account doesn't have the IAM permissions required to perform the specified Resolver operation.</p>
    AccessDenied(String),
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

impl GetResolverDnssecConfigError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetResolverDnssecConfigError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(GetResolverDnssecConfigError::AccessDenied(
                        err.msg,
                    ))
                }
                "InternalServiceErrorException" => {
                    return RusotoError::Service(
                        GetResolverDnssecConfigError::InternalServiceError(err.msg),
                    )
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(GetResolverDnssecConfigError::InvalidParameter(
                        err.msg,
                    ))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(GetResolverDnssecConfigError::InvalidRequest(
                        err.msg,
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(GetResolverDnssecConfigError::ResourceNotFound(
                        err.msg,
                    ))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(GetResolverDnssecConfigError::Throttling(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }

    fn refine(
        err: RusotoError<std::convert::Infallible>,
    ) -> RusotoError<GetResolverDnssecConfigError> {
        match err {
            RusotoError::Service(err) => match err {},
            RusotoError::HttpDispatch(err) => RusotoError::HttpDispatch(err),
            RusotoError::Credentials(err) => RusotoError::Credentials(err),
            RusotoError::Validation(err) => RusotoError::Validation(err),
            RusotoError::ParseError(err) => RusotoError::ParseError(err),
            RusotoError::Unknown(res) => Self::from_response(res),
            RusotoError::Blocking => RusotoError::Blocking,
        }
    }
}
impl fmt::Display for GetResolverDnssecConfigError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetResolverDnssecConfigError::AccessDenied(ref cause) => write!(f, "{}", cause),
            GetResolverDnssecConfigError::InternalServiceError(ref cause) => write!(f, "{}", cause),
            GetResolverDnssecConfigError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            GetResolverDnssecConfigError::InvalidRequest(ref cause) => write!(f, "{}", cause),
            GetResolverDnssecConfigError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            GetResolverDnssecConfigError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetResolverDnssecConfigError {}
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

    fn refine(err: RusotoError<std::convert::Infallible>) -> RusotoError<GetResolverEndpointError> {
        match err {
            RusotoError::Service(err) => match err {},
            RusotoError::HttpDispatch(err) => RusotoError::HttpDispatch(err),
            RusotoError::Credentials(err) => RusotoError::Credentials(err),
            RusotoError::Validation(err) => RusotoError::Validation(err),
            RusotoError::ParseError(err) => RusotoError::ParseError(err),
            RusotoError::Unknown(res) => Self::from_response(res),
            RusotoError::Blocking => RusotoError::Blocking,
        }
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
/// Errors returned by GetResolverQueryLogConfig
#[derive(Debug, PartialEq)]
pub enum GetResolverQueryLogConfigError {
    /// <p>The current account doesn't have the IAM permissions required to perform the specified Resolver operation.</p>
    AccessDenied(String),
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

impl GetResolverQueryLogConfigError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetResolverQueryLogConfigError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(GetResolverQueryLogConfigError::AccessDenied(
                        err.msg,
                    ))
                }
                "InternalServiceErrorException" => {
                    return RusotoError::Service(
                        GetResolverQueryLogConfigError::InternalServiceError(err.msg),
                    )
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(GetResolverQueryLogConfigError::InvalidParameter(
                        err.msg,
                    ))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(GetResolverQueryLogConfigError::InvalidRequest(
                        err.msg,
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(GetResolverQueryLogConfigError::ResourceNotFound(
                        err.msg,
                    ))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(GetResolverQueryLogConfigError::Throttling(
                        err.msg,
                    ))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }

    fn refine(
        err: RusotoError<std::convert::Infallible>,
    ) -> RusotoError<GetResolverQueryLogConfigError> {
        match err {
            RusotoError::Service(err) => match err {},
            RusotoError::HttpDispatch(err) => RusotoError::HttpDispatch(err),
            RusotoError::Credentials(err) => RusotoError::Credentials(err),
            RusotoError::Validation(err) => RusotoError::Validation(err),
            RusotoError::ParseError(err) => RusotoError::ParseError(err),
            RusotoError::Unknown(res) => Self::from_response(res),
            RusotoError::Blocking => RusotoError::Blocking,
        }
    }
}
impl fmt::Display for GetResolverQueryLogConfigError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetResolverQueryLogConfigError::AccessDenied(ref cause) => write!(f, "{}", cause),
            GetResolverQueryLogConfigError::InternalServiceError(ref cause) => {
                write!(f, "{}", cause)
            }
            GetResolverQueryLogConfigError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            GetResolverQueryLogConfigError::InvalidRequest(ref cause) => write!(f, "{}", cause),
            GetResolverQueryLogConfigError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            GetResolverQueryLogConfigError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetResolverQueryLogConfigError {}
/// Errors returned by GetResolverQueryLogConfigAssociation
#[derive(Debug, PartialEq)]
pub enum GetResolverQueryLogConfigAssociationError {
    /// <p>The current account doesn't have the IAM permissions required to perform the specified Resolver operation.</p>
    AccessDenied(String),
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

impl GetResolverQueryLogConfigAssociationError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<GetResolverQueryLogConfigAssociationError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(
                        GetResolverQueryLogConfigAssociationError::AccessDenied(err.msg),
                    )
                }
                "InternalServiceErrorException" => {
                    return RusotoError::Service(
                        GetResolverQueryLogConfigAssociationError::InternalServiceError(err.msg),
                    )
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(
                        GetResolverQueryLogConfigAssociationError::InvalidParameter(err.msg),
                    )
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(
                        GetResolverQueryLogConfigAssociationError::InvalidRequest(err.msg),
                    )
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(
                        GetResolverQueryLogConfigAssociationError::ResourceNotFound(err.msg),
                    )
                }
                "ThrottlingException" => {
                    return RusotoError::Service(
                        GetResolverQueryLogConfigAssociationError::Throttling(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }

    fn refine(
        err: RusotoError<std::convert::Infallible>,
    ) -> RusotoError<GetResolverQueryLogConfigAssociationError> {
        match err {
            RusotoError::Service(err) => match err {},
            RusotoError::HttpDispatch(err) => RusotoError::HttpDispatch(err),
            RusotoError::Credentials(err) => RusotoError::Credentials(err),
            RusotoError::Validation(err) => RusotoError::Validation(err),
            RusotoError::ParseError(err) => RusotoError::ParseError(err),
            RusotoError::Unknown(res) => Self::from_response(res),
            RusotoError::Blocking => RusotoError::Blocking,
        }
    }
}
impl fmt::Display for GetResolverQueryLogConfigAssociationError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetResolverQueryLogConfigAssociationError::AccessDenied(ref cause) => {
                write!(f, "{}", cause)
            }
            GetResolverQueryLogConfigAssociationError::InternalServiceError(ref cause) => {
                write!(f, "{}", cause)
            }
            GetResolverQueryLogConfigAssociationError::InvalidParameter(ref cause) => {
                write!(f, "{}", cause)
            }
            GetResolverQueryLogConfigAssociationError::InvalidRequest(ref cause) => {
                write!(f, "{}", cause)
            }
            GetResolverQueryLogConfigAssociationError::ResourceNotFound(ref cause) => {
                write!(f, "{}", cause)
            }
            GetResolverQueryLogConfigAssociationError::Throttling(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for GetResolverQueryLogConfigAssociationError {}
/// Errors returned by GetResolverQueryLogConfigPolicy
#[derive(Debug, PartialEq)]
pub enum GetResolverQueryLogConfigPolicyError {
    /// <p>The current account doesn't have the IAM permissions required to perform the specified Resolver operation.</p>
    AccessDenied(String),
    /// <p>We encountered an unknown error. Try again in a few minutes.</p>
    InternalServiceError(String),
    /// <p>One or more parameters in this request are not valid.</p>
    InvalidParameter(String),
    /// <p>The request is invalid.</p>
    InvalidRequest(String),
    /// <p>The specified resource doesn't exist.</p>
    UnknownResource(String),
}

impl GetResolverQueryLogConfigPolicyError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<GetResolverQueryLogConfigPolicyError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(
                        GetResolverQueryLogConfigPolicyError::AccessDenied(err.msg),
                    )
                }
                "InternalServiceErrorException" => {
                    return RusotoError::Service(
                        GetResolverQueryLogConfigPolicyError::InternalServiceError(err.msg),
                    )
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(
                        GetResolverQueryLogConfigPolicyError::InvalidParameter(err.msg),
                    )
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(
                        GetResolverQueryLogConfigPolicyError::InvalidRequest(err.msg),
                    )
                }
                "UnknownResourceException" => {
                    return RusotoError::Service(
                        GetResolverQueryLogConfigPolicyError::UnknownResource(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }

    fn refine(
        err: RusotoError<std::convert::Infallible>,
    ) -> RusotoError<GetResolverQueryLogConfigPolicyError> {
        match err {
            RusotoError::Service(err) => match err {},
            RusotoError::HttpDispatch(err) => RusotoError::HttpDispatch(err),
            RusotoError::Credentials(err) => RusotoError::Credentials(err),
            RusotoError::Validation(err) => RusotoError::Validation(err),
            RusotoError::ParseError(err) => RusotoError::ParseError(err),
            RusotoError::Unknown(res) => Self::from_response(res),
            RusotoError::Blocking => RusotoError::Blocking,
        }
    }
}
impl fmt::Display for GetResolverQueryLogConfigPolicyError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetResolverQueryLogConfigPolicyError::AccessDenied(ref cause) => write!(f, "{}", cause),
            GetResolverQueryLogConfigPolicyError::InternalServiceError(ref cause) => {
                write!(f, "{}", cause)
            }
            GetResolverQueryLogConfigPolicyError::InvalidParameter(ref cause) => {
                write!(f, "{}", cause)
            }
            GetResolverQueryLogConfigPolicyError::InvalidRequest(ref cause) => {
                write!(f, "{}", cause)
            }
            GetResolverQueryLogConfigPolicyError::UnknownResource(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for GetResolverQueryLogConfigPolicyError {}
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

    fn refine(err: RusotoError<std::convert::Infallible>) -> RusotoError<GetResolverRuleError> {
        match err {
            RusotoError::Service(err) => match err {},
            RusotoError::HttpDispatch(err) => RusotoError::HttpDispatch(err),
            RusotoError::Credentials(err) => RusotoError::Credentials(err),
            RusotoError::Validation(err) => RusotoError::Validation(err),
            RusotoError::ParseError(err) => RusotoError::ParseError(err),
            RusotoError::Unknown(res) => Self::from_response(res),
            RusotoError::Blocking => RusotoError::Blocking,
        }
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

    fn refine(
        err: RusotoError<std::convert::Infallible>,
    ) -> RusotoError<GetResolverRuleAssociationError> {
        match err {
            RusotoError::Service(err) => match err {},
            RusotoError::HttpDispatch(err) => RusotoError::HttpDispatch(err),
            RusotoError::Credentials(err) => RusotoError::Credentials(err),
            RusotoError::Validation(err) => RusotoError::Validation(err),
            RusotoError::ParseError(err) => RusotoError::ParseError(err),
            RusotoError::Unknown(res) => Self::from_response(res),
            RusotoError::Blocking => RusotoError::Blocking,
        }
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

    fn refine(
        err: RusotoError<std::convert::Infallible>,
    ) -> RusotoError<GetResolverRulePolicyError> {
        match err {
            RusotoError::Service(err) => match err {},
            RusotoError::HttpDispatch(err) => RusotoError::HttpDispatch(err),
            RusotoError::Credentials(err) => RusotoError::Credentials(err),
            RusotoError::Validation(err) => RusotoError::Validation(err),
            RusotoError::ParseError(err) => RusotoError::ParseError(err),
            RusotoError::Unknown(res) => Self::from_response(res),
            RusotoError::Blocking => RusotoError::Blocking,
        }
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
/// Errors returned by ListResolverDnssecConfigs
#[derive(Debug, PartialEq)]
pub enum ListResolverDnssecConfigsError {
    /// <p>The current account doesn't have the IAM permissions required to perform the specified Resolver operation.</p>
    AccessDenied(String),
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

impl ListResolverDnssecConfigsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListResolverDnssecConfigsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(ListResolverDnssecConfigsError::AccessDenied(
                        err.msg,
                    ))
                }
                "InternalServiceErrorException" => {
                    return RusotoError::Service(
                        ListResolverDnssecConfigsError::InternalServiceError(err.msg),
                    )
                }
                "InvalidNextTokenException" => {
                    return RusotoError::Service(ListResolverDnssecConfigsError::InvalidNextToken(
                        err.msg,
                    ))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(ListResolverDnssecConfigsError::InvalidParameter(
                        err.msg,
                    ))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(ListResolverDnssecConfigsError::InvalidRequest(
                        err.msg,
                    ))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(ListResolverDnssecConfigsError::Throttling(
                        err.msg,
                    ))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }

    fn refine(
        err: RusotoError<std::convert::Infallible>,
    ) -> RusotoError<ListResolverDnssecConfigsError> {
        match err {
            RusotoError::Service(err) => match err {},
            RusotoError::HttpDispatch(err) => RusotoError::HttpDispatch(err),
            RusotoError::Credentials(err) => RusotoError::Credentials(err),
            RusotoError::Validation(err) => RusotoError::Validation(err),
            RusotoError::ParseError(err) => RusotoError::ParseError(err),
            RusotoError::Unknown(res) => Self::from_response(res),
            RusotoError::Blocking => RusotoError::Blocking,
        }
    }
}
impl fmt::Display for ListResolverDnssecConfigsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListResolverDnssecConfigsError::AccessDenied(ref cause) => write!(f, "{}", cause),
            ListResolverDnssecConfigsError::InternalServiceError(ref cause) => {
                write!(f, "{}", cause)
            }
            ListResolverDnssecConfigsError::InvalidNextToken(ref cause) => write!(f, "{}", cause),
            ListResolverDnssecConfigsError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            ListResolverDnssecConfigsError::InvalidRequest(ref cause) => write!(f, "{}", cause),
            ListResolverDnssecConfigsError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListResolverDnssecConfigsError {}
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

    fn refine(
        err: RusotoError<std::convert::Infallible>,
    ) -> RusotoError<ListResolverEndpointIpAddressesError> {
        match err {
            RusotoError::Service(err) => match err {},
            RusotoError::HttpDispatch(err) => RusotoError::HttpDispatch(err),
            RusotoError::Credentials(err) => RusotoError::Credentials(err),
            RusotoError::Validation(err) => RusotoError::Validation(err),
            RusotoError::ParseError(err) => RusotoError::ParseError(err),
            RusotoError::Unknown(res) => Self::from_response(res),
            RusotoError::Blocking => RusotoError::Blocking,
        }
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

    fn refine(
        err: RusotoError<std::convert::Infallible>,
    ) -> RusotoError<ListResolverEndpointsError> {
        match err {
            RusotoError::Service(err) => match err {},
            RusotoError::HttpDispatch(err) => RusotoError::HttpDispatch(err),
            RusotoError::Credentials(err) => RusotoError::Credentials(err),
            RusotoError::Validation(err) => RusotoError::Validation(err),
            RusotoError::ParseError(err) => RusotoError::ParseError(err),
            RusotoError::Unknown(res) => Self::from_response(res),
            RusotoError::Blocking => RusotoError::Blocking,
        }
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
/// Errors returned by ListResolverQueryLogConfigAssociations
#[derive(Debug, PartialEq)]
pub enum ListResolverQueryLogConfigAssociationsError {
    /// <p>The current account doesn't have the IAM permissions required to perform the specified Resolver operation.</p>
    AccessDenied(String),
    /// <p>We encountered an unknown error. Try again in a few minutes.</p>
    InternalServiceError(String),
    /// <p>One or more parameters in this request are not valid.</p>
    InvalidParameter(String),
    /// <p>The request is invalid.</p>
    InvalidRequest(String),
    /// <p>The request caused one or more limits to be exceeded.</p>
    LimitExceeded(String),
    /// <p>The request was throttled. Try again in a few minutes.</p>
    Throttling(String),
}

impl ListResolverQueryLogConfigAssociationsError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<ListResolverQueryLogConfigAssociationsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(
                        ListResolverQueryLogConfigAssociationsError::AccessDenied(err.msg),
                    )
                }
                "InternalServiceErrorException" => {
                    return RusotoError::Service(
                        ListResolverQueryLogConfigAssociationsError::InternalServiceError(err.msg),
                    )
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(
                        ListResolverQueryLogConfigAssociationsError::InvalidParameter(err.msg),
                    )
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(
                        ListResolverQueryLogConfigAssociationsError::InvalidRequest(err.msg),
                    )
                }
                "LimitExceededException" => {
                    return RusotoError::Service(
                        ListResolverQueryLogConfigAssociationsError::LimitExceeded(err.msg),
                    )
                }
                "ThrottlingException" => {
                    return RusotoError::Service(
                        ListResolverQueryLogConfigAssociationsError::Throttling(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }

    fn refine(
        err: RusotoError<std::convert::Infallible>,
    ) -> RusotoError<ListResolverQueryLogConfigAssociationsError> {
        match err {
            RusotoError::Service(err) => match err {},
            RusotoError::HttpDispatch(err) => RusotoError::HttpDispatch(err),
            RusotoError::Credentials(err) => RusotoError::Credentials(err),
            RusotoError::Validation(err) => RusotoError::Validation(err),
            RusotoError::ParseError(err) => RusotoError::ParseError(err),
            RusotoError::Unknown(res) => Self::from_response(res),
            RusotoError::Blocking => RusotoError::Blocking,
        }
    }
}
impl fmt::Display for ListResolverQueryLogConfigAssociationsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListResolverQueryLogConfigAssociationsError::AccessDenied(ref cause) => {
                write!(f, "{}", cause)
            }
            ListResolverQueryLogConfigAssociationsError::InternalServiceError(ref cause) => {
                write!(f, "{}", cause)
            }
            ListResolverQueryLogConfigAssociationsError::InvalidParameter(ref cause) => {
                write!(f, "{}", cause)
            }
            ListResolverQueryLogConfigAssociationsError::InvalidRequest(ref cause) => {
                write!(f, "{}", cause)
            }
            ListResolverQueryLogConfigAssociationsError::LimitExceeded(ref cause) => {
                write!(f, "{}", cause)
            }
            ListResolverQueryLogConfigAssociationsError::Throttling(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for ListResolverQueryLogConfigAssociationsError {}
/// Errors returned by ListResolverQueryLogConfigs
#[derive(Debug, PartialEq)]
pub enum ListResolverQueryLogConfigsError {
    /// <p>The current account doesn't have the IAM permissions required to perform the specified Resolver operation.</p>
    AccessDenied(String),
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

impl ListResolverQueryLogConfigsError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<ListResolverQueryLogConfigsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(ListResolverQueryLogConfigsError::AccessDenied(
                        err.msg,
                    ))
                }
                "InternalServiceErrorException" => {
                    return RusotoError::Service(
                        ListResolverQueryLogConfigsError::InternalServiceError(err.msg),
                    )
                }
                "InvalidNextTokenException" => {
                    return RusotoError::Service(
                        ListResolverQueryLogConfigsError::InvalidNextToken(err.msg),
                    )
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(
                        ListResolverQueryLogConfigsError::InvalidParameter(err.msg),
                    )
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(ListResolverQueryLogConfigsError::InvalidRequest(
                        err.msg,
                    ))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(ListResolverQueryLogConfigsError::Throttling(
                        err.msg,
                    ))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }

    fn refine(
        err: RusotoError<std::convert::Infallible>,
    ) -> RusotoError<ListResolverQueryLogConfigsError> {
        match err {
            RusotoError::Service(err) => match err {},
            RusotoError::HttpDispatch(err) => RusotoError::HttpDispatch(err),
            RusotoError::Credentials(err) => RusotoError::Credentials(err),
            RusotoError::Validation(err) => RusotoError::Validation(err),
            RusotoError::ParseError(err) => RusotoError::ParseError(err),
            RusotoError::Unknown(res) => Self::from_response(res),
            RusotoError::Blocking => RusotoError::Blocking,
        }
    }
}
impl fmt::Display for ListResolverQueryLogConfigsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListResolverQueryLogConfigsError::AccessDenied(ref cause) => write!(f, "{}", cause),
            ListResolverQueryLogConfigsError::InternalServiceError(ref cause) => {
                write!(f, "{}", cause)
            }
            ListResolverQueryLogConfigsError::InvalidNextToken(ref cause) => write!(f, "{}", cause),
            ListResolverQueryLogConfigsError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            ListResolverQueryLogConfigsError::InvalidRequest(ref cause) => write!(f, "{}", cause),
            ListResolverQueryLogConfigsError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListResolverQueryLogConfigsError {}
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

    fn refine(
        err: RusotoError<std::convert::Infallible>,
    ) -> RusotoError<ListResolverRuleAssociationsError> {
        match err {
            RusotoError::Service(err) => match err {},
            RusotoError::HttpDispatch(err) => RusotoError::HttpDispatch(err),
            RusotoError::Credentials(err) => RusotoError::Credentials(err),
            RusotoError::Validation(err) => RusotoError::Validation(err),
            RusotoError::ParseError(err) => RusotoError::ParseError(err),
            RusotoError::Unknown(res) => Self::from_response(res),
            RusotoError::Blocking => RusotoError::Blocking,
        }
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

    fn refine(err: RusotoError<std::convert::Infallible>) -> RusotoError<ListResolverRulesError> {
        match err {
            RusotoError::Service(err) => match err {},
            RusotoError::HttpDispatch(err) => RusotoError::HttpDispatch(err),
            RusotoError::Credentials(err) => RusotoError::Credentials(err),
            RusotoError::Validation(err) => RusotoError::Validation(err),
            RusotoError::ParseError(err) => RusotoError::ParseError(err),
            RusotoError::Unknown(res) => Self::from_response(res),
            RusotoError::Blocking => RusotoError::Blocking,
        }
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

    fn refine(err: RusotoError<std::convert::Infallible>) -> RusotoError<ListTagsForResourceError> {
        match err {
            RusotoError::Service(err) => match err {},
            RusotoError::HttpDispatch(err) => RusotoError::HttpDispatch(err),
            RusotoError::Credentials(err) => RusotoError::Credentials(err),
            RusotoError::Validation(err) => RusotoError::Validation(err),
            RusotoError::ParseError(err) => RusotoError::ParseError(err),
            RusotoError::Unknown(res) => Self::from_response(res),
            RusotoError::Blocking => RusotoError::Blocking,
        }
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
/// Errors returned by PutResolverQueryLogConfigPolicy
#[derive(Debug, PartialEq)]
pub enum PutResolverQueryLogConfigPolicyError {
    /// <p>The current account doesn't have the IAM permissions required to perform the specified Resolver operation.</p>
    AccessDenied(String),
    /// <p>We encountered an unknown error. Try again in a few minutes.</p>
    InternalServiceError(String),
    /// <p>One or more parameters in this request are not valid.</p>
    InvalidParameter(String),
    /// <p>The specified Resolver rule policy is invalid.</p>
    InvalidPolicyDocument(String),
    /// <p>The request is invalid.</p>
    InvalidRequest(String),
    /// <p>The specified resource doesn't exist.</p>
    UnknownResource(String),
}

impl PutResolverQueryLogConfigPolicyError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<PutResolverQueryLogConfigPolicyError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(
                        PutResolverQueryLogConfigPolicyError::AccessDenied(err.msg),
                    )
                }
                "InternalServiceErrorException" => {
                    return RusotoError::Service(
                        PutResolverQueryLogConfigPolicyError::InternalServiceError(err.msg),
                    )
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(
                        PutResolverQueryLogConfigPolicyError::InvalidParameter(err.msg),
                    )
                }
                "InvalidPolicyDocument" => {
                    return RusotoError::Service(
                        PutResolverQueryLogConfigPolicyError::InvalidPolicyDocument(err.msg),
                    )
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(
                        PutResolverQueryLogConfigPolicyError::InvalidRequest(err.msg),
                    )
                }
                "UnknownResourceException" => {
                    return RusotoError::Service(
                        PutResolverQueryLogConfigPolicyError::UnknownResource(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }

    fn refine(
        err: RusotoError<std::convert::Infallible>,
    ) -> RusotoError<PutResolverQueryLogConfigPolicyError> {
        match err {
            RusotoError::Service(err) => match err {},
            RusotoError::HttpDispatch(err) => RusotoError::HttpDispatch(err),
            RusotoError::Credentials(err) => RusotoError::Credentials(err),
            RusotoError::Validation(err) => RusotoError::Validation(err),
            RusotoError::ParseError(err) => RusotoError::ParseError(err),
            RusotoError::Unknown(res) => Self::from_response(res),
            RusotoError::Blocking => RusotoError::Blocking,
        }
    }
}
impl fmt::Display for PutResolverQueryLogConfigPolicyError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            PutResolverQueryLogConfigPolicyError::AccessDenied(ref cause) => write!(f, "{}", cause),
            PutResolverQueryLogConfigPolicyError::InternalServiceError(ref cause) => {
                write!(f, "{}", cause)
            }
            PutResolverQueryLogConfigPolicyError::InvalidParameter(ref cause) => {
                write!(f, "{}", cause)
            }
            PutResolverQueryLogConfigPolicyError::InvalidPolicyDocument(ref cause) => {
                write!(f, "{}", cause)
            }
            PutResolverQueryLogConfigPolicyError::InvalidRequest(ref cause) => {
                write!(f, "{}", cause)
            }
            PutResolverQueryLogConfigPolicyError::UnknownResource(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for PutResolverQueryLogConfigPolicyError {}
/// Errors returned by PutResolverRulePolicy
#[derive(Debug, PartialEq)]
pub enum PutResolverRulePolicyError {
    /// <p>We encountered an unknown error. Try again in a few minutes.</p>
    InternalServiceError(String),
    /// <p>One or more parameters in this request are not valid.</p>
    InvalidParameter(String),
    /// <p>The specified Resolver rule policy is invalid.</p>
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

    fn refine(
        err: RusotoError<std::convert::Infallible>,
    ) -> RusotoError<PutResolverRulePolicyError> {
        match err {
            RusotoError::Service(err) => match err {},
            RusotoError::HttpDispatch(err) => RusotoError::HttpDispatch(err),
            RusotoError::Credentials(err) => RusotoError::Credentials(err),
            RusotoError::Validation(err) => RusotoError::Validation(err),
            RusotoError::ParseError(err) => RusotoError::ParseError(err),
            RusotoError::Unknown(res) => Self::from_response(res),
            RusotoError::Blocking => RusotoError::Blocking,
        }
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
    /// <p>The request is invalid.</p>
    InvalidRequest(String),
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
                "InvalidRequestException" => {
                    return RusotoError::Service(TagResourceError::InvalidRequest(err.msg))
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

    fn refine(err: RusotoError<std::convert::Infallible>) -> RusotoError<TagResourceError> {
        match err {
            RusotoError::Service(err) => match err {},
            RusotoError::HttpDispatch(err) => RusotoError::HttpDispatch(err),
            RusotoError::Credentials(err) => RusotoError::Credentials(err),
            RusotoError::Validation(err) => RusotoError::Validation(err),
            RusotoError::ParseError(err) => RusotoError::ParseError(err),
            RusotoError::Unknown(res) => Self::from_response(res),
            RusotoError::Blocking => RusotoError::Blocking,
        }
    }
}
impl fmt::Display for TagResourceError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            TagResourceError::InternalServiceError(ref cause) => write!(f, "{}", cause),
            TagResourceError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            TagResourceError::InvalidRequest(ref cause) => write!(f, "{}", cause),
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
    /// <p>The request is invalid.</p>
    InvalidRequest(String),
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
                "InvalidRequestException" => {
                    return RusotoError::Service(UntagResourceError::InvalidRequest(err.msg))
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

    fn refine(err: RusotoError<std::convert::Infallible>) -> RusotoError<UntagResourceError> {
        match err {
            RusotoError::Service(err) => match err {},
            RusotoError::HttpDispatch(err) => RusotoError::HttpDispatch(err),
            RusotoError::Credentials(err) => RusotoError::Credentials(err),
            RusotoError::Validation(err) => RusotoError::Validation(err),
            RusotoError::ParseError(err) => RusotoError::ParseError(err),
            RusotoError::Unknown(res) => Self::from_response(res),
            RusotoError::Blocking => RusotoError::Blocking,
        }
    }
}
impl fmt::Display for UntagResourceError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UntagResourceError::InternalServiceError(ref cause) => write!(f, "{}", cause),
            UntagResourceError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            UntagResourceError::InvalidRequest(ref cause) => write!(f, "{}", cause),
            UntagResourceError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            UntagResourceError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for UntagResourceError {}
/// Errors returned by UpdateResolverDnssecConfig
#[derive(Debug, PartialEq)]
pub enum UpdateResolverDnssecConfigError {
    /// <p>The current account doesn't have the IAM permissions required to perform the specified Resolver operation.</p>
    AccessDenied(String),
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

impl UpdateResolverDnssecConfigError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<UpdateResolverDnssecConfigError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(UpdateResolverDnssecConfigError::AccessDenied(
                        err.msg,
                    ))
                }
                "InternalServiceErrorException" => {
                    return RusotoError::Service(
                        UpdateResolverDnssecConfigError::InternalServiceError(err.msg),
                    )
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(UpdateResolverDnssecConfigError::InvalidParameter(
                        err.msg,
                    ))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(UpdateResolverDnssecConfigError::InvalidRequest(
                        err.msg,
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(UpdateResolverDnssecConfigError::ResourceNotFound(
                        err.msg,
                    ))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(UpdateResolverDnssecConfigError::Throttling(
                        err.msg,
                    ))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }

    fn refine(
        err: RusotoError<std::convert::Infallible>,
    ) -> RusotoError<UpdateResolverDnssecConfigError> {
        match err {
            RusotoError::Service(err) => match err {},
            RusotoError::HttpDispatch(err) => RusotoError::HttpDispatch(err),
            RusotoError::Credentials(err) => RusotoError::Credentials(err),
            RusotoError::Validation(err) => RusotoError::Validation(err),
            RusotoError::ParseError(err) => RusotoError::ParseError(err),
            RusotoError::Unknown(res) => Self::from_response(res),
            RusotoError::Blocking => RusotoError::Blocking,
        }
    }
}
impl fmt::Display for UpdateResolverDnssecConfigError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UpdateResolverDnssecConfigError::AccessDenied(ref cause) => write!(f, "{}", cause),
            UpdateResolverDnssecConfigError::InternalServiceError(ref cause) => {
                write!(f, "{}", cause)
            }
            UpdateResolverDnssecConfigError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            UpdateResolverDnssecConfigError::InvalidRequest(ref cause) => write!(f, "{}", cause),
            UpdateResolverDnssecConfigError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            UpdateResolverDnssecConfigError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for UpdateResolverDnssecConfigError {}
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

    fn refine(
        err: RusotoError<std::convert::Infallible>,
    ) -> RusotoError<UpdateResolverEndpointError> {
        match err {
            RusotoError::Service(err) => match err {},
            RusotoError::HttpDispatch(err) => RusotoError::HttpDispatch(err),
            RusotoError::Credentials(err) => RusotoError::Credentials(err),
            RusotoError::Validation(err) => RusotoError::Validation(err),
            RusotoError::ParseError(err) => RusotoError::ParseError(err),
            RusotoError::Unknown(res) => Self::from_response(res),
            RusotoError::Blocking => RusotoError::Blocking,
        }
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

    fn refine(err: RusotoError<std::convert::Infallible>) -> RusotoError<UpdateResolverRuleError> {
        match err {
            RusotoError::Service(err) => match err {},
            RusotoError::HttpDispatch(err) => RusotoError::HttpDispatch(err),
            RusotoError::Credentials(err) => RusotoError::Credentials(err),
            RusotoError::Validation(err) => RusotoError::Validation(err),
            RusotoError::ParseError(err) => RusotoError::ParseError(err),
            RusotoError::Unknown(res) => Self::from_response(res),
            RusotoError::Blocking => RusotoError::Blocking,
        }
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
    /// <p>Adds IP addresses to an inbound or an outbound Resolver endpoint. If you want to add more than one IP address, submit one <code>AssociateResolverEndpointIpAddress</code> request for each IP address.</p> <p>To remove an IP address from an endpoint, see <a href="https://docs.aws.amazon.com/Route53/latest/APIReference/API_route53resolver_DisassociateResolverEndpointIpAddress.html">DisassociateResolverEndpointIpAddress</a>. </p>
    async fn associate_resolver_endpoint_ip_address(
        &self,
        input: AssociateResolverEndpointIpAddressRequest,
    ) -> Result<
        AssociateResolverEndpointIpAddressResponse,
        RusotoError<AssociateResolverEndpointIpAddressError>,
    >;

    /// <p>Associates an Amazon VPC with a specified query logging configuration. Route 53 Resolver logs DNS queries that originate in all of the Amazon VPCs that are associated with a specified query logging configuration. To associate more than one VPC with a configuration, submit one <code>AssociateResolverQueryLogConfig</code> request for each VPC.</p> <note> <p>The VPCs that you associate with a query logging configuration must be in the same Region as the configuration.</p> </note> <p>To remove a VPC from a query logging configuration, see <a href="https://docs.aws.amazon.com/Route53/latest/APIReference/API_route53resolver_DisassociateResolverQueryLogConfig.html">DisassociateResolverQueryLogConfig</a>. </p>
    async fn associate_resolver_query_log_config(
        &self,
        input: AssociateResolverQueryLogConfigRequest,
    ) -> Result<
        AssociateResolverQueryLogConfigResponse,
        RusotoError<AssociateResolverQueryLogConfigError>,
    >;

    /// <p>Associates a Resolver rule with a VPC. When you associate a rule with a VPC, Resolver forwards all DNS queries for the domain name that is specified in the rule and that originate in the VPC. The queries are forwarded to the IP addresses for the DNS resolvers that are specified in the rule. For more information about rules, see <a href="https://docs.aws.amazon.com/Route53/latest/APIReference/API_route53resolver_CreateResolverRule.html">CreateResolverRule</a>. </p>
    async fn associate_resolver_rule(
        &self,
        input: AssociateResolverRuleRequest,
    ) -> Result<AssociateResolverRuleResponse, RusotoError<AssociateResolverRuleError>>;

    /// <p><p>Creates a Resolver endpoint. There are two types of Resolver endpoints, inbound and outbound:</p> <ul> <li> <p>An <i>inbound Resolver endpoint</i> forwards DNS queries to the DNS service for a VPC from your network.</p> </li> <li> <p>An <i>outbound Resolver endpoint</i> forwards DNS queries from the DNS service for a VPC to your network.</p> </li> </ul></p>
    async fn create_resolver_endpoint(
        &self,
        input: CreateResolverEndpointRequest,
    ) -> Result<CreateResolverEndpointResponse, RusotoError<CreateResolverEndpointError>>;

    /// <p>Creates a Resolver query logging configuration, which defines where you want Resolver to save DNS query logs that originate in your VPCs. Resolver can log queries only for VPCs that are in the same Region as the query logging configuration.</p> <p>To specify which VPCs you want to log queries for, you use <code>AssociateResolverQueryLogConfig</code>. For more information, see <a href="https://docs.aws.amazon.com/Route53/latest/APIReference/API_route53resolver_AssociateResolverQueryLogConfig.html">AssociateResolverQueryLogConfig</a>. </p> <p>You can optionally use AWS Resource Access Manager (AWS RAM) to share a query logging configuration with other AWS accounts. The other accounts can then associate VPCs with the configuration. The query logs that Resolver creates for a configuration include all DNS queries that originate in all VPCs that are associated with the configuration.</p>
    async fn create_resolver_query_log_config(
        &self,
        input: CreateResolverQueryLogConfigRequest,
    ) -> Result<CreateResolverQueryLogConfigResponse, RusotoError<CreateResolverQueryLogConfigError>>;

    /// <p>For DNS queries that originate in your VPCs, specifies which Resolver endpoint the queries pass through, one domain name that you want to forward to your network, and the IP addresses of the DNS resolvers in your network.</p>
    async fn create_resolver_rule(
        &self,
        input: CreateResolverRuleRequest,
    ) -> Result<CreateResolverRuleResponse, RusotoError<CreateResolverRuleError>>;

    /// <p><p>Deletes a Resolver endpoint. The effect of deleting a Resolver endpoint depends on whether it&#39;s an inbound or an outbound Resolver endpoint:</p> <ul> <li> <p> <b>Inbound</b>: DNS queries from your network are no longer routed to the DNS service for the specified VPC.</p> </li> <li> <p> <b>Outbound</b>: DNS queries from a VPC are no longer routed to your network.</p> </li> </ul></p>
    async fn delete_resolver_endpoint(
        &self,
        input: DeleteResolverEndpointRequest,
    ) -> Result<DeleteResolverEndpointResponse, RusotoError<DeleteResolverEndpointError>>;

    /// <p>Deletes a query logging configuration. When you delete a configuration, Resolver stops logging DNS queries for all of the Amazon VPCs that are associated with the configuration. This also applies if the query logging configuration is shared with other AWS accounts, and the other accounts have associated VPCs with the shared configuration.</p> <p>Before you can delete a query logging configuration, you must first disassociate all VPCs from the configuration. See <a href="https://docs.aws.amazon.com/Route53/latest/APIReference/API_route53resolver_DisassociateResolverQueryLogConfig.html">DisassociateResolverQueryLogConfig</a>.</p> <p>If you used Resource Access Manager (RAM) to share a query logging configuration with other accounts, you must stop sharing the configuration before you can delete a configuration. The accounts that you shared the configuration with can first disassociate VPCs that they associated with the configuration, but that's not necessary. If you stop sharing the configuration, those VPCs are automatically disassociated from the configuration.</p>
    async fn delete_resolver_query_log_config(
        &self,
        input: DeleteResolverQueryLogConfigRequest,
    ) -> Result<DeleteResolverQueryLogConfigResponse, RusotoError<DeleteResolverQueryLogConfigError>>;

    /// <p>Deletes a Resolver rule. Before you can delete a Resolver rule, you must disassociate it from all the VPCs that you associated the Resolver rule with. For more information, see <a href="https://docs.aws.amazon.com/Route53/latest/APIReference/API_route53resolver_DisassociateResolverRule.html">DisassociateResolverRule</a>.</p>
    async fn delete_resolver_rule(
        &self,
        input: DeleteResolverRuleRequest,
    ) -> Result<DeleteResolverRuleResponse, RusotoError<DeleteResolverRuleError>>;

    /// <p>Removes IP addresses from an inbound or an outbound Resolver endpoint. If you want to remove more than one IP address, submit one <code>DisassociateResolverEndpointIpAddress</code> request for each IP address.</p> <p>To add an IP address to an endpoint, see <a href="https://docs.aws.amazon.com/Route53/latest/APIReference/API_route53resolver_AssociateResolverEndpointIpAddress.html">AssociateResolverEndpointIpAddress</a>. </p>
    async fn disassociate_resolver_endpoint_ip_address(
        &self,
        input: DisassociateResolverEndpointIpAddressRequest,
    ) -> Result<
        DisassociateResolverEndpointIpAddressResponse,
        RusotoError<DisassociateResolverEndpointIpAddressError>,
    >;

    /// <p><p>Disassociates a VPC from a query logging configuration.</p> <note> <p>Before you can delete a query logging configuration, you must first disassociate all VPCs from the configuration. If you used Resource Access Manager (RAM) to share a query logging configuration with other accounts, VPCs can be disassociated from the configuration in the following ways:</p> <ul> <li> <p>The accounts that you shared the configuration with can disassociate VPCs from the configuration.</p> </li> <li> <p>You can stop sharing the configuration.</p> </li> </ul> </note></p>
    async fn disassociate_resolver_query_log_config(
        &self,
        input: DisassociateResolverQueryLogConfigRequest,
    ) -> Result<
        DisassociateResolverQueryLogConfigResponse,
        RusotoError<DisassociateResolverQueryLogConfigError>,
    >;

    /// <p><p>Removes the association between a specified Resolver rule and a specified VPC.</p> <important> <p>If you disassociate a Resolver rule from a VPC, Resolver stops forwarding DNS queries for the domain name that you specified in the Resolver rule. </p> </important></p>
    async fn disassociate_resolver_rule(
        &self,
        input: DisassociateResolverRuleRequest,
    ) -> Result<DisassociateResolverRuleResponse, RusotoError<DisassociateResolverRuleError>>;

    /// <p>Gets DNSSEC validation information for a specified resource.</p>
    async fn get_resolver_dnssec_config(
        &self,
        input: GetResolverDnssecConfigRequest,
    ) -> Result<GetResolverDnssecConfigResponse, RusotoError<GetResolverDnssecConfigError>>;

    /// <p>Gets information about a specified Resolver endpoint, such as whether it's an inbound or an outbound Resolver endpoint, and the current status of the endpoint.</p>
    async fn get_resolver_endpoint(
        &self,
        input: GetResolverEndpointRequest,
    ) -> Result<GetResolverEndpointResponse, RusotoError<GetResolverEndpointError>>;

    /// <p>Gets information about a specified Resolver query logging configuration, such as the number of VPCs that the configuration is logging queries for and the location that logs are sent to. </p>
    async fn get_resolver_query_log_config(
        &self,
        input: GetResolverQueryLogConfigRequest,
    ) -> Result<GetResolverQueryLogConfigResponse, RusotoError<GetResolverQueryLogConfigError>>;

    /// <p>Gets information about a specified association between a Resolver query logging configuration and an Amazon VPC. When you associate a VPC with a query logging configuration, Resolver logs DNS queries that originate in that VPC.</p>
    async fn get_resolver_query_log_config_association(
        &self,
        input: GetResolverQueryLogConfigAssociationRequest,
    ) -> Result<
        GetResolverQueryLogConfigAssociationResponse,
        RusotoError<GetResolverQueryLogConfigAssociationError>,
    >;

    /// <p>Gets information about a query logging policy. A query logging policy specifies the Resolver query logging operations and resources that you want to allow another AWS account to be able to use.</p>
    async fn get_resolver_query_log_config_policy(
        &self,
        input: GetResolverQueryLogConfigPolicyRequest,
    ) -> Result<
        GetResolverQueryLogConfigPolicyResponse,
        RusotoError<GetResolverQueryLogConfigPolicyError>,
    >;

    /// <p>Gets information about a specified Resolver rule, such as the domain name that the rule forwards DNS queries for and the ID of the outbound Resolver endpoint that the rule is associated with.</p>
    async fn get_resolver_rule(
        &self,
        input: GetResolverRuleRequest,
    ) -> Result<GetResolverRuleResponse, RusotoError<GetResolverRuleError>>;

    /// <p>Gets information about an association between a specified Resolver rule and a VPC. You associate a Resolver rule and a VPC using <a href="https://docs.aws.amazon.com/Route53/latest/APIReference/API_route53resolver_AssociateResolverRule.html">AssociateResolverRule</a>. </p>
    async fn get_resolver_rule_association(
        &self,
        input: GetResolverRuleAssociationRequest,
    ) -> Result<GetResolverRuleAssociationResponse, RusotoError<GetResolverRuleAssociationError>>;

    /// <p>Gets information about the Resolver rule policy for a specified rule. A Resolver rule policy includes the rule that you want to share with another account, the account that you want to share the rule with, and the Resolver operations that you want to allow the account to use. </p>
    async fn get_resolver_rule_policy(
        &self,
        input: GetResolverRulePolicyRequest,
    ) -> Result<GetResolverRulePolicyResponse, RusotoError<GetResolverRulePolicyError>>;

    /// <p>Lists the configurations for DNSSEC validation that are associated with the current AWS account.</p>
    async fn list_resolver_dnssec_configs(
        &self,
        input: ListResolverDnssecConfigsRequest,
    ) -> Result<ListResolverDnssecConfigsResponse, RusotoError<ListResolverDnssecConfigsError>>;

    /// <p>Gets the IP addresses for a specified Resolver endpoint.</p>
    async fn list_resolver_endpoint_ip_addresses(
        &self,
        input: ListResolverEndpointIpAddressesRequest,
    ) -> Result<
        ListResolverEndpointIpAddressesResponse,
        RusotoError<ListResolverEndpointIpAddressesError>,
    >;

    /// <p>Lists all the Resolver endpoints that were created using the current AWS account.</p>
    async fn list_resolver_endpoints(
        &self,
        input: ListResolverEndpointsRequest,
    ) -> Result<ListResolverEndpointsResponse, RusotoError<ListResolverEndpointsError>>;

    /// <p>Lists information about associations between Amazon VPCs and query logging configurations.</p>
    async fn list_resolver_query_log_config_associations(
        &self,
        input: ListResolverQueryLogConfigAssociationsRequest,
    ) -> Result<
        ListResolverQueryLogConfigAssociationsResponse,
        RusotoError<ListResolverQueryLogConfigAssociationsError>,
    >;

    /// <p>Lists information about the specified query logging configurations. Each configuration defines where you want Resolver to save DNS query logs and specifies the VPCs that you want to log queries for.</p>
    async fn list_resolver_query_log_configs(
        &self,
        input: ListResolverQueryLogConfigsRequest,
    ) -> Result<ListResolverQueryLogConfigsResponse, RusotoError<ListResolverQueryLogConfigsError>>;

    /// <p>Lists the associations that were created between Resolver rules and VPCs using the current AWS account.</p>
    async fn list_resolver_rule_associations(
        &self,
        input: ListResolverRuleAssociationsRequest,
    ) -> Result<ListResolverRuleAssociationsResponse, RusotoError<ListResolverRuleAssociationsError>>;

    /// <p>Lists the Resolver rules that were created using the current AWS account.</p>
    async fn list_resolver_rules(
        &self,
        input: ListResolverRulesRequest,
    ) -> Result<ListResolverRulesResponse, RusotoError<ListResolverRulesError>>;

    /// <p>Lists the tags that you associated with the specified resource.</p>
    async fn list_tags_for_resource(
        &self,
        input: ListTagsForResourceRequest,
    ) -> Result<ListTagsForResourceResponse, RusotoError<ListTagsForResourceError>>;

    /// <p>Specifies an AWS account that you want to share a query logging configuration with, the query logging configuration that you want to share, and the operations that you want the account to be able to perform on the configuration.</p>
    async fn put_resolver_query_log_config_policy(
        &self,
        input: PutResolverQueryLogConfigPolicyRequest,
    ) -> Result<
        PutResolverQueryLogConfigPolicyResponse,
        RusotoError<PutResolverQueryLogConfigPolicyError>,
    >;

    /// <p>Specifies an AWS rule that you want to share with another account, the account that you want to share the rule with, and the operations that you want the account to be able to perform on the rule.</p>
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

    /// <p>Updates an existing DNSSEC validation configuration. If there is no existing DNSSEC validation configuration, one is created.</p>
    async fn update_resolver_dnssec_config(
        &self,
        input: UpdateResolverDnssecConfigRequest,
    ) -> Result<UpdateResolverDnssecConfigResponse, RusotoError<UpdateResolverDnssecConfigError>>;

    /// <p>Updates the name of an inbound or an outbound Resolver endpoint. </p>
    async fn update_resolver_endpoint(
        &self,
        input: UpdateResolverEndpointRequest,
    ) -> Result<UpdateResolverEndpointResponse, RusotoError<UpdateResolverEndpointError>>;

    /// <p>Updates settings for a specified Resolver rule. <code>ResolverRuleId</code> is required, and all other parameters are optional. If you don't specify a parameter, it retains its current value.</p>
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
    /// <p>Adds IP addresses to an inbound or an outbound Resolver endpoint. If you want to add more than one IP address, submit one <code>AssociateResolverEndpointIpAddress</code> request for each IP address.</p> <p>To remove an IP address from an endpoint, see <a href="https://docs.aws.amazon.com/Route53/latest/APIReference/API_route53resolver_DisassociateResolverEndpointIpAddress.html">DisassociateResolverEndpointIpAddress</a>. </p>
    async fn associate_resolver_endpoint_ip_address(
        &self,
        input: AssociateResolverEndpointIpAddressRequest,
    ) -> Result<
        AssociateResolverEndpointIpAddressResponse,
        RusotoError<AssociateResolverEndpointIpAddressError>,
    > {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "Route53Resolver.AssociateResolverEndpointIpAddress",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request)
            .await
            .map_err(AssociateResolverEndpointIpAddressError::refine)?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response)
            .deserialize::<AssociateResolverEndpointIpAddressResponse, _>()
    }

    /// <p>Associates an Amazon VPC with a specified query logging configuration. Route 53 Resolver logs DNS queries that originate in all of the Amazon VPCs that are associated with a specified query logging configuration. To associate more than one VPC with a configuration, submit one <code>AssociateResolverQueryLogConfig</code> request for each VPC.</p> <note> <p>The VPCs that you associate with a query logging configuration must be in the same Region as the configuration.</p> </note> <p>To remove a VPC from a query logging configuration, see <a href="https://docs.aws.amazon.com/Route53/latest/APIReference/API_route53resolver_DisassociateResolverQueryLogConfig.html">DisassociateResolverQueryLogConfig</a>. </p>
    async fn associate_resolver_query_log_config(
        &self,
        input: AssociateResolverQueryLogConfigRequest,
    ) -> Result<
        AssociateResolverQueryLogConfigResponse,
        RusotoError<AssociateResolverQueryLogConfigError>,
    > {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "Route53Resolver.AssociateResolverQueryLogConfig",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request)
            .await
            .map_err(AssociateResolverQueryLogConfigError::refine)?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response)
            .deserialize::<AssociateResolverQueryLogConfigResponse, _>()
    }

    /// <p>Associates a Resolver rule with a VPC. When you associate a rule with a VPC, Resolver forwards all DNS queries for the domain name that is specified in the rule and that originate in the VPC. The queries are forwarded to the IP addresses for the DNS resolvers that are specified in the rule. For more information about rules, see <a href="https://docs.aws.amazon.com/Route53/latest/APIReference/API_route53resolver_CreateResolverRule.html">CreateResolverRule</a>. </p>
    async fn associate_resolver_rule(
        &self,
        input: AssociateResolverRuleRequest,
    ) -> Result<AssociateResolverRuleResponse, RusotoError<AssociateResolverRuleError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "Route53Resolver.AssociateResolverRule");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request)
            .await
            .map_err(AssociateResolverRuleError::refine)?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response)
            .deserialize::<AssociateResolverRuleResponse, _>()
    }

    /// <p><p>Creates a Resolver endpoint. There are two types of Resolver endpoints, inbound and outbound:</p> <ul> <li> <p>An <i>inbound Resolver endpoint</i> forwards DNS queries to the DNS service for a VPC from your network.</p> </li> <li> <p>An <i>outbound Resolver endpoint</i> forwards DNS queries from the DNS service for a VPC to your network.</p> </li> </ul></p>
    async fn create_resolver_endpoint(
        &self,
        input: CreateResolverEndpointRequest,
    ) -> Result<CreateResolverEndpointResponse, RusotoError<CreateResolverEndpointError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "Route53Resolver.CreateResolverEndpoint");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request)
            .await
            .map_err(CreateResolverEndpointError::refine)?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response)
            .deserialize::<CreateResolverEndpointResponse, _>()
    }

    /// <p>Creates a Resolver query logging configuration, which defines where you want Resolver to save DNS query logs that originate in your VPCs. Resolver can log queries only for VPCs that are in the same Region as the query logging configuration.</p> <p>To specify which VPCs you want to log queries for, you use <code>AssociateResolverQueryLogConfig</code>. For more information, see <a href="https://docs.aws.amazon.com/Route53/latest/APIReference/API_route53resolver_AssociateResolverQueryLogConfig.html">AssociateResolverQueryLogConfig</a>. </p> <p>You can optionally use AWS Resource Access Manager (AWS RAM) to share a query logging configuration with other AWS accounts. The other accounts can then associate VPCs with the configuration. The query logs that Resolver creates for a configuration include all DNS queries that originate in all VPCs that are associated with the configuration.</p>
    async fn create_resolver_query_log_config(
        &self,
        input: CreateResolverQueryLogConfigRequest,
    ) -> Result<CreateResolverQueryLogConfigResponse, RusotoError<CreateResolverQueryLogConfigError>>
    {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "Route53Resolver.CreateResolverQueryLogConfig",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request)
            .await
            .map_err(CreateResolverQueryLogConfigError::refine)?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response)
            .deserialize::<CreateResolverQueryLogConfigResponse, _>()
    }

    /// <p>For DNS queries that originate in your VPCs, specifies which Resolver endpoint the queries pass through, one domain name that you want to forward to your network, and the IP addresses of the DNS resolvers in your network.</p>
    async fn create_resolver_rule(
        &self,
        input: CreateResolverRuleRequest,
    ) -> Result<CreateResolverRuleResponse, RusotoError<CreateResolverRuleError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "Route53Resolver.CreateResolverRule");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request)
            .await
            .map_err(CreateResolverRuleError::refine)?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<CreateResolverRuleResponse, _>()
    }

    /// <p><p>Deletes a Resolver endpoint. The effect of deleting a Resolver endpoint depends on whether it&#39;s an inbound or an outbound Resolver endpoint:</p> <ul> <li> <p> <b>Inbound</b>: DNS queries from your network are no longer routed to the DNS service for the specified VPC.</p> </li> <li> <p> <b>Outbound</b>: DNS queries from a VPC are no longer routed to your network.</p> </li> </ul></p>
    async fn delete_resolver_endpoint(
        &self,
        input: DeleteResolverEndpointRequest,
    ) -> Result<DeleteResolverEndpointResponse, RusotoError<DeleteResolverEndpointError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "Route53Resolver.DeleteResolverEndpoint");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request)
            .await
            .map_err(DeleteResolverEndpointError::refine)?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response)
            .deserialize::<DeleteResolverEndpointResponse, _>()
    }

    /// <p>Deletes a query logging configuration. When you delete a configuration, Resolver stops logging DNS queries for all of the Amazon VPCs that are associated with the configuration. This also applies if the query logging configuration is shared with other AWS accounts, and the other accounts have associated VPCs with the shared configuration.</p> <p>Before you can delete a query logging configuration, you must first disassociate all VPCs from the configuration. See <a href="https://docs.aws.amazon.com/Route53/latest/APIReference/API_route53resolver_DisassociateResolverQueryLogConfig.html">DisassociateResolverQueryLogConfig</a>.</p> <p>If you used Resource Access Manager (RAM) to share a query logging configuration with other accounts, you must stop sharing the configuration before you can delete a configuration. The accounts that you shared the configuration with can first disassociate VPCs that they associated with the configuration, but that's not necessary. If you stop sharing the configuration, those VPCs are automatically disassociated from the configuration.</p>
    async fn delete_resolver_query_log_config(
        &self,
        input: DeleteResolverQueryLogConfigRequest,
    ) -> Result<DeleteResolverQueryLogConfigResponse, RusotoError<DeleteResolverQueryLogConfigError>>
    {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "Route53Resolver.DeleteResolverQueryLogConfig",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request)
            .await
            .map_err(DeleteResolverQueryLogConfigError::refine)?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response)
            .deserialize::<DeleteResolverQueryLogConfigResponse, _>()
    }

    /// <p>Deletes a Resolver rule. Before you can delete a Resolver rule, you must disassociate it from all the VPCs that you associated the Resolver rule with. For more information, see <a href="https://docs.aws.amazon.com/Route53/latest/APIReference/API_route53resolver_DisassociateResolverRule.html">DisassociateResolverRule</a>.</p>
    async fn delete_resolver_rule(
        &self,
        input: DeleteResolverRuleRequest,
    ) -> Result<DeleteResolverRuleResponse, RusotoError<DeleteResolverRuleError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "Route53Resolver.DeleteResolverRule");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request)
            .await
            .map_err(DeleteResolverRuleError::refine)?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<DeleteResolverRuleResponse, _>()
    }

    /// <p>Removes IP addresses from an inbound or an outbound Resolver endpoint. If you want to remove more than one IP address, submit one <code>DisassociateResolverEndpointIpAddress</code> request for each IP address.</p> <p>To add an IP address to an endpoint, see <a href="https://docs.aws.amazon.com/Route53/latest/APIReference/API_route53resolver_AssociateResolverEndpointIpAddress.html">AssociateResolverEndpointIpAddress</a>. </p>
    async fn disassociate_resolver_endpoint_ip_address(
        &self,
        input: DisassociateResolverEndpointIpAddressRequest,
    ) -> Result<
        DisassociateResolverEndpointIpAddressResponse,
        RusotoError<DisassociateResolverEndpointIpAddressError>,
    > {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "Route53Resolver.DisassociateResolverEndpointIpAddress",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request)
            .await
            .map_err(DisassociateResolverEndpointIpAddressError::refine)?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response)
            .deserialize::<DisassociateResolverEndpointIpAddressResponse, _>()
    }

    /// <p><p>Disassociates a VPC from a query logging configuration.</p> <note> <p>Before you can delete a query logging configuration, you must first disassociate all VPCs from the configuration. If you used Resource Access Manager (RAM) to share a query logging configuration with other accounts, VPCs can be disassociated from the configuration in the following ways:</p> <ul> <li> <p>The accounts that you shared the configuration with can disassociate VPCs from the configuration.</p> </li> <li> <p>You can stop sharing the configuration.</p> </li> </ul> </note></p>
    async fn disassociate_resolver_query_log_config(
        &self,
        input: DisassociateResolverQueryLogConfigRequest,
    ) -> Result<
        DisassociateResolverQueryLogConfigResponse,
        RusotoError<DisassociateResolverQueryLogConfigError>,
    > {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "Route53Resolver.DisassociateResolverQueryLogConfig",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request)
            .await
            .map_err(DisassociateResolverQueryLogConfigError::refine)?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response)
            .deserialize::<DisassociateResolverQueryLogConfigResponse, _>()
    }

    /// <p><p>Removes the association between a specified Resolver rule and a specified VPC.</p> <important> <p>If you disassociate a Resolver rule from a VPC, Resolver stops forwarding DNS queries for the domain name that you specified in the Resolver rule. </p> </important></p>
    async fn disassociate_resolver_rule(
        &self,
        input: DisassociateResolverRuleRequest,
    ) -> Result<DisassociateResolverRuleResponse, RusotoError<DisassociateResolverRuleError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "Route53Resolver.DisassociateResolverRule");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request)
            .await
            .map_err(DisassociateResolverRuleError::refine)?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response)
            .deserialize::<DisassociateResolverRuleResponse, _>()
    }

    /// <p>Gets DNSSEC validation information for a specified resource.</p>
    async fn get_resolver_dnssec_config(
        &self,
        input: GetResolverDnssecConfigRequest,
    ) -> Result<GetResolverDnssecConfigResponse, RusotoError<GetResolverDnssecConfigError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "Route53Resolver.GetResolverDnssecConfig");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request)
            .await
            .map_err(GetResolverDnssecConfigError::refine)?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response)
            .deserialize::<GetResolverDnssecConfigResponse, _>()
    }

    /// <p>Gets information about a specified Resolver endpoint, such as whether it's an inbound or an outbound Resolver endpoint, and the current status of the endpoint.</p>
    async fn get_resolver_endpoint(
        &self,
        input: GetResolverEndpointRequest,
    ) -> Result<GetResolverEndpointResponse, RusotoError<GetResolverEndpointError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "Route53Resolver.GetResolverEndpoint");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request)
            .await
            .map_err(GetResolverEndpointError::refine)?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<GetResolverEndpointResponse, _>()
    }

    /// <p>Gets information about a specified Resolver query logging configuration, such as the number of VPCs that the configuration is logging queries for and the location that logs are sent to. </p>
    async fn get_resolver_query_log_config(
        &self,
        input: GetResolverQueryLogConfigRequest,
    ) -> Result<GetResolverQueryLogConfigResponse, RusotoError<GetResolverQueryLogConfigError>>
    {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "Route53Resolver.GetResolverQueryLogConfig");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request)
            .await
            .map_err(GetResolverQueryLogConfigError::refine)?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response)
            .deserialize::<GetResolverQueryLogConfigResponse, _>()
    }

    /// <p>Gets information about a specified association between a Resolver query logging configuration and an Amazon VPC. When you associate a VPC with a query logging configuration, Resolver logs DNS queries that originate in that VPC.</p>
    async fn get_resolver_query_log_config_association(
        &self,
        input: GetResolverQueryLogConfigAssociationRequest,
    ) -> Result<
        GetResolverQueryLogConfigAssociationResponse,
        RusotoError<GetResolverQueryLogConfigAssociationError>,
    > {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "Route53Resolver.GetResolverQueryLogConfigAssociation",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request)
            .await
            .map_err(GetResolverQueryLogConfigAssociationError::refine)?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response)
            .deserialize::<GetResolverQueryLogConfigAssociationResponse, _>()
    }

    /// <p>Gets information about a query logging policy. A query logging policy specifies the Resolver query logging operations and resources that you want to allow another AWS account to be able to use.</p>
    async fn get_resolver_query_log_config_policy(
        &self,
        input: GetResolverQueryLogConfigPolicyRequest,
    ) -> Result<
        GetResolverQueryLogConfigPolicyResponse,
        RusotoError<GetResolverQueryLogConfigPolicyError>,
    > {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "Route53Resolver.GetResolverQueryLogConfigPolicy",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request)
            .await
            .map_err(GetResolverQueryLogConfigPolicyError::refine)?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response)
            .deserialize::<GetResolverQueryLogConfigPolicyResponse, _>()
    }

    /// <p>Gets information about a specified Resolver rule, such as the domain name that the rule forwards DNS queries for and the ID of the outbound Resolver endpoint that the rule is associated with.</p>
    async fn get_resolver_rule(
        &self,
        input: GetResolverRuleRequest,
    ) -> Result<GetResolverRuleResponse, RusotoError<GetResolverRuleError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "Route53Resolver.GetResolverRule");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request)
            .await
            .map_err(GetResolverRuleError::refine)?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<GetResolverRuleResponse, _>()
    }

    /// <p>Gets information about an association between a specified Resolver rule and a VPC. You associate a Resolver rule and a VPC using <a href="https://docs.aws.amazon.com/Route53/latest/APIReference/API_route53resolver_AssociateResolverRule.html">AssociateResolverRule</a>. </p>
    async fn get_resolver_rule_association(
        &self,
        input: GetResolverRuleAssociationRequest,
    ) -> Result<GetResolverRuleAssociationResponse, RusotoError<GetResolverRuleAssociationError>>
    {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "Route53Resolver.GetResolverRuleAssociation");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request)
            .await
            .map_err(GetResolverRuleAssociationError::refine)?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response)
            .deserialize::<GetResolverRuleAssociationResponse, _>()
    }

    /// <p>Gets information about the Resolver rule policy for a specified rule. A Resolver rule policy includes the rule that you want to share with another account, the account that you want to share the rule with, and the Resolver operations that you want to allow the account to use. </p>
    async fn get_resolver_rule_policy(
        &self,
        input: GetResolverRulePolicyRequest,
    ) -> Result<GetResolverRulePolicyResponse, RusotoError<GetResolverRulePolicyError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "Route53Resolver.GetResolverRulePolicy");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request)
            .await
            .map_err(GetResolverRulePolicyError::refine)?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response)
            .deserialize::<GetResolverRulePolicyResponse, _>()
    }

    /// <p>Lists the configurations for DNSSEC validation that are associated with the current AWS account.</p>
    async fn list_resolver_dnssec_configs(
        &self,
        input: ListResolverDnssecConfigsRequest,
    ) -> Result<ListResolverDnssecConfigsResponse, RusotoError<ListResolverDnssecConfigsError>>
    {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "Route53Resolver.ListResolverDnssecConfigs");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request)
            .await
            .map_err(ListResolverDnssecConfigsError::refine)?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response)
            .deserialize::<ListResolverDnssecConfigsResponse, _>()
    }

    /// <p>Gets the IP addresses for a specified Resolver endpoint.</p>
    async fn list_resolver_endpoint_ip_addresses(
        &self,
        input: ListResolverEndpointIpAddressesRequest,
    ) -> Result<
        ListResolverEndpointIpAddressesResponse,
        RusotoError<ListResolverEndpointIpAddressesError>,
    > {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "Route53Resolver.ListResolverEndpointIpAddresses",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request)
            .await
            .map_err(ListResolverEndpointIpAddressesError::refine)?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response)
            .deserialize::<ListResolverEndpointIpAddressesResponse, _>()
    }

    /// <p>Lists all the Resolver endpoints that were created using the current AWS account.</p>
    async fn list_resolver_endpoints(
        &self,
        input: ListResolverEndpointsRequest,
    ) -> Result<ListResolverEndpointsResponse, RusotoError<ListResolverEndpointsError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "Route53Resolver.ListResolverEndpoints");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request)
            .await
            .map_err(ListResolverEndpointsError::refine)?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response)
            .deserialize::<ListResolverEndpointsResponse, _>()
    }

    /// <p>Lists information about associations between Amazon VPCs and query logging configurations.</p>
    async fn list_resolver_query_log_config_associations(
        &self,
        input: ListResolverQueryLogConfigAssociationsRequest,
    ) -> Result<
        ListResolverQueryLogConfigAssociationsResponse,
        RusotoError<ListResolverQueryLogConfigAssociationsError>,
    > {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "Route53Resolver.ListResolverQueryLogConfigAssociations",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request)
            .await
            .map_err(ListResolverQueryLogConfigAssociationsError::refine)?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response)
            .deserialize::<ListResolverQueryLogConfigAssociationsResponse, _>()
    }

    /// <p>Lists information about the specified query logging configurations. Each configuration defines where you want Resolver to save DNS query logs and specifies the VPCs that you want to log queries for.</p>
    async fn list_resolver_query_log_configs(
        &self,
        input: ListResolverQueryLogConfigsRequest,
    ) -> Result<ListResolverQueryLogConfigsResponse, RusotoError<ListResolverQueryLogConfigsError>>
    {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "Route53Resolver.ListResolverQueryLogConfigs",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request)
            .await
            .map_err(ListResolverQueryLogConfigsError::refine)?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response)
            .deserialize::<ListResolverQueryLogConfigsResponse, _>()
    }

    /// <p>Lists the associations that were created between Resolver rules and VPCs using the current AWS account.</p>
    async fn list_resolver_rule_associations(
        &self,
        input: ListResolverRuleAssociationsRequest,
    ) -> Result<ListResolverRuleAssociationsResponse, RusotoError<ListResolverRuleAssociationsError>>
    {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "Route53Resolver.ListResolverRuleAssociations",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request)
            .await
            .map_err(ListResolverRuleAssociationsError::refine)?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response)
            .deserialize::<ListResolverRuleAssociationsResponse, _>()
    }

    /// <p>Lists the Resolver rules that were created using the current AWS account.</p>
    async fn list_resolver_rules(
        &self,
        input: ListResolverRulesRequest,
    ) -> Result<ListResolverRulesResponse, RusotoError<ListResolverRulesError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "Route53Resolver.ListResolverRules");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request)
            .await
            .map_err(ListResolverRulesError::refine)?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<ListResolverRulesResponse, _>()
    }

    /// <p>Lists the tags that you associated with the specified resource.</p>
    async fn list_tags_for_resource(
        &self,
        input: ListTagsForResourceRequest,
    ) -> Result<ListTagsForResourceResponse, RusotoError<ListTagsForResourceError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "Route53Resolver.ListTagsForResource");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request)
            .await
            .map_err(ListTagsForResourceError::refine)?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<ListTagsForResourceResponse, _>()
    }

    /// <p>Specifies an AWS account that you want to share a query logging configuration with, the query logging configuration that you want to share, and the operations that you want the account to be able to perform on the configuration.</p>
    async fn put_resolver_query_log_config_policy(
        &self,
        input: PutResolverQueryLogConfigPolicyRequest,
    ) -> Result<
        PutResolverQueryLogConfigPolicyResponse,
        RusotoError<PutResolverQueryLogConfigPolicyError>,
    > {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "Route53Resolver.PutResolverQueryLogConfigPolicy",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request)
            .await
            .map_err(PutResolverQueryLogConfigPolicyError::refine)?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response)
            .deserialize::<PutResolverQueryLogConfigPolicyResponse, _>()
    }

    /// <p>Specifies an AWS rule that you want to share with another account, the account that you want to share the rule with, and the operations that you want the account to be able to perform on the rule.</p>
    async fn put_resolver_rule_policy(
        &self,
        input: PutResolverRulePolicyRequest,
    ) -> Result<PutResolverRulePolicyResponse, RusotoError<PutResolverRulePolicyError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "Route53Resolver.PutResolverRulePolicy");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request)
            .await
            .map_err(PutResolverRulePolicyError::refine)?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response)
            .deserialize::<PutResolverRulePolicyResponse, _>()
    }

    /// <p>Adds one or more tags to a specified resource.</p>
    async fn tag_resource(
        &self,
        input: TagResourceRequest,
    ) -> Result<TagResourceResponse, RusotoError<TagResourceError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "Route53Resolver.TagResource");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request)
            .await
            .map_err(TagResourceError::refine)?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<TagResourceResponse, _>()
    }

    /// <p>Removes one or more tags from a specified resource.</p>
    async fn untag_resource(
        &self,
        input: UntagResourceRequest,
    ) -> Result<UntagResourceResponse, RusotoError<UntagResourceError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "Route53Resolver.UntagResource");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request)
            .await
            .map_err(UntagResourceError::refine)?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<UntagResourceResponse, _>()
    }

    /// <p>Updates an existing DNSSEC validation configuration. If there is no existing DNSSEC validation configuration, one is created.</p>
    async fn update_resolver_dnssec_config(
        &self,
        input: UpdateResolverDnssecConfigRequest,
    ) -> Result<UpdateResolverDnssecConfigResponse, RusotoError<UpdateResolverDnssecConfigError>>
    {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "Route53Resolver.UpdateResolverDnssecConfig");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request)
            .await
            .map_err(UpdateResolverDnssecConfigError::refine)?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response)
            .deserialize::<UpdateResolverDnssecConfigResponse, _>()
    }

    /// <p>Updates the name of an inbound or an outbound Resolver endpoint. </p>
    async fn update_resolver_endpoint(
        &self,
        input: UpdateResolverEndpointRequest,
    ) -> Result<UpdateResolverEndpointResponse, RusotoError<UpdateResolverEndpointError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "Route53Resolver.UpdateResolverEndpoint");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request)
            .await
            .map_err(UpdateResolverEndpointError::refine)?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response)
            .deserialize::<UpdateResolverEndpointResponse, _>()
    }

    /// <p>Updates settings for a specified Resolver rule. <code>ResolverRuleId</code> is required, and all other parameters are optional. If you don't specify a parameter, it retains its current value.</p>
    async fn update_resolver_rule(
        &self,
        input: UpdateResolverRuleRequest,
    ) -> Result<UpdateResolverRuleResponse, RusotoError<UpdateResolverRuleError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "Route53Resolver.UpdateResolverRule");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request)
            .await
            .map_err(UpdateResolverRuleError::refine)?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<UpdateResolverRuleResponse, _>()
    }
}
