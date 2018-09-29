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
pub struct CreatePrivateDnsNamespaceRequest {
    /// <p>A unique string that identifies the request and that allows failed <code>CreatePrivateDnsNamespace</code> requests to be retried without the risk of executing the operation twice. <code>CreatorRequestId</code> can be any unique string, for example, a date/time stamp.</p>
    #[serde(rename = "CreatorRequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creator_request_id: Option<String>,
    /// <p>A description for the namespace.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The name that you want to assign to this namespace. When you create a namespace, Amazon Route 53 automatically creates a hosted zone that has the same name as the namespace.</p>
    #[serde(rename = "Name")]
    pub name: String,
    /// <p>The ID of the Amazon VPC that you want to associate the namespace with.</p>
    #[serde(rename = "Vpc")]
    pub vpc: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct CreatePrivateDnsNamespaceResponse {
    /// <p>A value that you can use to determine whether the request completed successfully. To get the status of the operation, see <a>GetOperation</a>.</p>
    #[serde(rename = "OperationId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operation_id: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct CreatePublicDnsNamespaceRequest {
    /// <p>A unique string that identifies the request and that allows failed <code>CreatePublicDnsNamespace</code> requests to be retried without the risk of executing the operation twice. <code>CreatorRequestId</code> can be any unique string, for example, a date/time stamp.</p>
    #[serde(rename = "CreatorRequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creator_request_id: Option<String>,
    /// <p>A description for the namespace.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The name that you want to assign to this namespace.</p>
    #[serde(rename = "Name")]
    pub name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct CreatePublicDnsNamespaceResponse {
    /// <p>A value that you can use to determine whether the request completed successfully. To get the status of the operation, see <a>GetOperation</a>.</p>
    #[serde(rename = "OperationId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operation_id: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct CreateServiceRequest {
    /// <p>A unique string that identifies the request and that allows failed <code>CreateService</code> requests to be retried without the risk of executing the operation twice. <code>CreatorRequestId</code> can be any unique string, for example, a date/time stamp.</p>
    #[serde(rename = "CreatorRequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creator_request_id: Option<String>,
    /// <p>A description for the service.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>A complex type that contains information about the records that you want Route 53 to create when you register an instance. </p>
    #[serde(rename = "DnsConfig")]
    pub dns_config: DnsConfig,
    /// <p> <i>Public DNS namespaces only.</i> A complex type that contains settings for an optional health check. If you specify settings for a health check, Route 53 associates the health check with all the records that you specify in <code>DnsConfig</code>.</p> <p>For information about the charges for health checks, see <a href="http://aws.amazon.com/route53/pricing">Route 53 Pricing</a>.</p>
    #[serde(rename = "HealthCheckConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub health_check_config: Option<HealthCheckConfig>,
    #[serde(rename = "HealthCheckCustomConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub health_check_custom_config: Option<HealthCheckCustomConfig>,
    /// <p>The name that you want to assign to the service.</p>
    #[serde(rename = "Name")]
    pub name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct CreateServiceResponse {
    /// <p>A complex type that contains information about the new service.</p>
    #[serde(rename = "Service")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service: Option<Service>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DeleteNamespaceRequest {
    /// <p>The ID of the namespace that you want to delete.</p>
    #[serde(rename = "Id")]
    pub id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct DeleteNamespaceResponse {
    /// <p>A value that you can use to determine whether the request completed successfully. To get the status of the operation, see <a>GetOperation</a>.</p>
    #[serde(rename = "OperationId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operation_id: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DeleteServiceRequest {
    /// <p>The ID of the service that you want to delete.</p>
    #[serde(rename = "Id")]
    pub id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct DeleteServiceResponse {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DeregisterInstanceRequest {
    /// <p>The value that you specified for <code>Id</code> in the <a>RegisterInstance</a> request.</p>
    #[serde(rename = "InstanceId")]
    pub instance_id: String,
    /// <p>The ID of the service that the instance is associated with.</p>
    #[serde(rename = "ServiceId")]
    pub service_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct DeregisterInstanceResponse {
    /// <p>A value that you can use to determine whether the request completed successfully. For more information, see <a>GetOperation</a>.</p>
    #[serde(rename = "OperationId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operation_id: Option<String>,
}

/// <p>A complex type that contains information about the records that you want Amazon Route 53 to create when you register an instance.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DnsConfig {
    /// <p>An array that contains one <code>DnsRecord</code> object for each record that you want Route 53 to create when you register an instance.</p>
    #[serde(rename = "DnsRecords")]
    pub dns_records: Vec<DnsRecord>,
    /// <p>The ID of the namespace to use for DNS configuration.</p>
    #[serde(rename = "NamespaceId")]
    pub namespace_id: String,
    /// <p>The routing policy that you want to apply to all records that Route 53 creates when you register an instance and specify this service.</p> <note> <p>If you want to use this service to register instances that create alias records, specify <code>WEIGHTED</code> for the routing policy.</p> </note> <p>You can specify the following values:</p> <p> <b>MULTIVALUE</b> </p> <p>If you define a health check for the service and the health check is healthy, Route 53 returns the applicable value for up to eight instances.</p> <p>For example, suppose the service includes configurations for one A record and a health check, and you use the service to register 10 instances. Route 53 responds to DNS queries with IP addresses for up to eight healthy instances. If fewer than eight instances are healthy, Route 53 responds to every DNS query with the IP addresses for all of the healthy instances.</p> <p>If you don't define a health check for the service, Route 53 assumes that all instances are healthy and returns the values for up to eight instances.</p> <p>For more information about the multivalue routing policy, see <a href="http://docs.aws.amazon.com/Route53/latest/DeveloperGuide/routing-policy.html#routing-policy-multivalue">Multivalue Answer Routing</a> in the <i>Route 53 Developer Guide</i>.</p> <p> <b>WEIGHTED</b> </p> <p>Route 53 returns the applicable value from one randomly selected instance from among the instances that you registered using the same service. Currently, all records have the same weight, so you can't route more or less traffic to any instances.</p> <p>For example, suppose the service includes configurations for one A record and a health check, and you use the service to register 10 instances. Route 53 responds to DNS queries with the IP address for one randomly selected instance from among the healthy instances. If no instances are healthy, Route 53 responds to DNS queries as if all of the instances were healthy.</p> <p>If you don't define a health check for the service, Route 53 assumes that all instances are healthy and returns the applicable value for one randomly selected instance.</p> <p>For more information about the weighted routing policy, see <a href="http://docs.aws.amazon.com/Route53/latest/DeveloperGuide/routing-policy.html#routing-policy-weighted">Weighted Routing</a> in the <i>Route 53 Developer Guide</i>.</p>
    #[serde(rename = "RoutingPolicy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub routing_policy: Option<String>,
}

/// <p>A complex type that contains information about changes to the records that Route 53 creates when you register an instance.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DnsConfigChange {
    /// <p>An array that contains one <code>DnsRecord</code> object for each record that you want Route 53 to create when you register an instance.</p>
    #[serde(rename = "DnsRecords")]
    pub dns_records: Vec<DnsRecord>,
}

/// <p>A complex type that contains the ID for the hosted zone that Route 53 creates when you create a namespace.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct DnsProperties {
    /// <p>The ID for the hosted zone that Route 53 creates when you create a namespace.</p>
    #[serde(rename = "HostedZoneId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hosted_zone_id: Option<String>,
}

/// <p>A complex type that contains information about the records that you want Route 53 to create when you register an instance.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DnsRecord {
    /// <p><p>The amount of time, in seconds, that you want DNS resolvers to cache the settings for this record.</p> <note> <p>Alias records don&#39;t include a TTL because Route 53 uses the TTL for the AWS resource that an alias record routes traffic to. If you include the <code>AWS<em>ALIAS</em>DNS_NAME</code> attribute when you submit a <a>RegisterInstance</a> request, the <code>TTL</code> value is ignored. Always specify a TTL for the service; you can use a service to register instances that create either alias or non-alias records.</p> </note></p>
    #[serde(rename = "TTL")]
    pub ttl: i64,
    /// <p>The type of the resource, which indicates the type of value that Route 53 returns in response to DNS queries.</p> <p>Note the following:</p> <ul> <li> <p> <b>A, AAAA, and SRV records: You can specify settings for a maximum of one A, one AAAA, and one SRV record. You can specify them in any combination.</b> </p> </li> <li> <p> <b>CNAME records:</b> If you specify <code>CNAME</code> for <code>Type</code>, you can't define any other records. This is a limitation of DNS—you can't create a CNAME record and any other type of record that has the same name as a CNAME record.</p> </li> <li> <p> <b>Alias records:</b> If you want Route 53 to create an alias record when you register an instance, specify <code>A</code> or <code>AAAA</code> for <code>Type</code>.</p> </li> <li> <p> <b>All records:</b> You specify settings other than <code>TTL</code> and <code>Type</code> when you register an instance.</p> </li> </ul> <p>The following values are supported:</p> <p> <b>A</b> </p> <p>Route 53 returns the IP address of the resource in IPv4 format, such as 192.0.2.44.</p> <p> <b>AAAA</b> </p> <p>Route 53 returns the IP address of the resource in IPv6 format, such as 2001:0db8:85a3:0000:0000:abcd:0001:2345.</p> <p> <b>CNAME</b> </p> <p>Route 53 returns the domain name of the resource, such as www.example.com. Note the following:</p> <ul> <li> <p>You specify the domain name that you want to route traffic to when you register an instance. For more information, see <a>RegisterInstanceRequest$Attributes</a>.</p> </li> <li> <p>You must specify <code>WEIGHTED</code> for the value of <code>RoutingPolicy</code>.</p> </li> <li> <p>You can't specify both <code>CNAME</code> for <code>Type</code> and settings for <code>HealthCheckConfig</code>. If you do, the request will fail with an <code>InvalidInput</code> error.</p> </li> </ul> <p> <b>SRV</b> </p> <p>Route 53 returns the value for an SRV record. The value for an SRV record uses the following values:</p> <p> <code>priority weight port service-hostname</code> </p> <p>Note the following about the values:</p> <ul> <li> <p>The values of <code>priority</code> and <code>weight</code> are both set to <code>1</code> and can't be changed. </p> </li> <li> <p>The value of <code>port</code> comes from the value that you specify for the <code>AWS_INSTANCE_PORT</code> attribute when you submit a <a>RegisterInstance</a> request. </p> </li> <li> <p>The value of <code>service-hostname</code> is a concatenation of the following values:</p> <ul> <li> <p>The value that you specify for <code>InstanceId</code> when you register an instance.</p> </li> <li> <p>The name of the service.</p> </li> <li> <p>The name of the namespace. </p> </li> </ul> <p>For example, if the value of <code>InstanceId</code> is <code>test</code>, the name of the service is <code>backend</code>, and the name of the namespace is <code>example.com</code>, the value of <code>service-hostname</code> is:</p> <p> <code>test.backend.example.com</code> </p> </li> </ul> <p>If you specify settings for an SRV record and if you specify values for <code>AWS_INSTANCE_IPV4</code>, <code>AWS_INSTANCE_IPV6</code>, or both in the <code>RegisterInstance</code> request, Route 53 automatically creates <code>A</code> and/or <code>AAAA</code> records that have the same name as the value of <code>service-hostname</code> in the SRV record. You can ignore these records.</p>
    #[serde(rename = "Type")]
    pub type_: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetInstanceRequest {
    /// <p>The ID of the instance that you want to get information about.</p>
    #[serde(rename = "InstanceId")]
    pub instance_id: String,
    /// <p>The ID of the service that the instance is associated with.</p>
    #[serde(rename = "ServiceId")]
    pub service_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct GetInstanceResponse {
    /// <p>A complex type that contains information about a specified instance.</p>
    #[serde(rename = "Instance")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance: Option<Instance>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetInstancesHealthStatusRequest {
    /// <p><p>An array that contains the IDs of all the instances that you want to get the health status for.</p> <p>If you omit <code>Instances</code>, Amazon Route 53 returns the health status for all the instances that are associated with the specified service.</p> <note> <p>To get the IDs for the instances that you&#39;ve registered by using a specified service, submit a <a>ListInstances</a> request.</p> </note></p>
    #[serde(rename = "Instances")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instances: Option<Vec<String>>,
    /// <p>The maximum number of instances that you want Route 53 to return in the response to a <code>GetInstancesHealthStatus</code> request. If you don't specify a value for <code>MaxResults</code>, Route 53 returns up to 100 instances.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>For the first <code>GetInstancesHealthStatus</code> request, omit this value.</p> <p>If more than <code>MaxResults</code> instances match the specified criteria, you can submit another <code>GetInstancesHealthStatus</code> request to get the next group of results. Specify the value of <code>NextToken</code> from the previous response in the next request.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The ID of the service that the instance is associated with.</p>
    #[serde(rename = "ServiceId")]
    pub service_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct GetInstancesHealthStatusResponse {
    /// <p>If more than <code>MaxResults</code> instances match the specified criteria, you can submit another <code>GetInstancesHealthStatus</code> request to get the next group of results. Specify the value of <code>NextToken</code> from the previous response in the next request.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>A complex type that contains the IDs and the health status of the instances that you specified in the <code>GetInstancesHealthStatus</code> request.</p>
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<::std::collections::HashMap<String, String>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetNamespaceRequest {
    /// <p>The ID of the namespace that you want to get information about.</p>
    #[serde(rename = "Id")]
    pub id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct GetNamespaceResponse {
    /// <p>A complex type that contains information about the specified namespace.</p>
    #[serde(rename = "Namespace")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub namespace: Option<Namespace>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetOperationRequest {
    /// <p>The ID of the operation that you want to get more information about.</p>
    #[serde(rename = "OperationId")]
    pub operation_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct GetOperationResponse {
    /// <p>A complex type that contains information about the operation.</p>
    #[serde(rename = "Operation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operation: Option<Operation>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetServiceRequest {
    /// <p>The ID of the service that you want to get settings for.</p>
    #[serde(rename = "Id")]
    pub id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct GetServiceResponse {
    /// <p>A complex type that contains information about the service.</p>
    #[serde(rename = "Service")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service: Option<Service>,
}

/// <p> <i>Public DNS namespaces only.</i> A complex type that contains settings for an optional health check. If you specify settings for a health check, Amazon Route 53 associates the health check with all the records that you specify in <code>DnsConfig</code>.</p> <p> <b>A and AAAA records</b> </p> <p>If <code>DnsConfig</code> includes configurations for both A and AAAA records, Route 53 creates a health check that uses the IPv4 address to check the health of the resource. If the endpoint that is specified by the IPv4 address is unhealthy, Route 53 considers both the A and AAAA records to be unhealthy. </p> <p> <b>CNAME records</b> </p> <p>You can't specify settings for <code>HealthCheckConfig</code> when the <code>DNSConfig</code> includes <code>CNAME</code> for the value of <code>Type</code>. If you do, the <code>CreateService</code> request will fail with an <code>InvalidInput</code> error.</p> <p> <b>Request interval</b> </p> <p>The health check uses 30 seconds as the request interval. This is the number of seconds between the time that each Route 53 health checker gets a response from your endpoint and the time that it sends the next health check request. A health checker in each data center around the world sends your endpoint a health check request every 30 seconds. On average, your endpoint receives a health check request about every two seconds. Health checkers in different data centers don't coordinate with one another, so you'll sometimes see several requests per second followed by a few seconds with no health checks at all.</p> <p> <b>Health checking regions</b> </p> <p>Health checkers perform checks from all Route 53 health-checking regions. For a list of the current regions, see <a href="http://docs.aws.amazon.com/Route53/latest/APIReference/API_HealthCheckConfig.html#Route53-Type-HealthCheckConfig-Regions">Regions</a>.</p> <p> <b>Alias records</b> </p> <p>When you register an instance, if you include the <code>AWS_ALIAS_DNS_NAME</code> attribute, Route 53 creates an alias record. Note the following:</p> <ul> <li> <p>Route 53 automatically sets <code>EvaluateTargetHealth</code> to true for alias records. When <code>EvaluateTargetHealth</code> is true, the alias record inherits the health of the referenced AWS resource. such as an ELB load balancer. For more information, see <a href="http://docs.aws.amazon.com/Route53/latest/APIReference/API_AliasTarget.html#Route53-Type-AliasTarget-EvaluateTargetHealth">EvaluateTargetHealth</a>.</p> </li> <li> <p>If you include <code>HealthCheckConfig</code> and then use the service to register an instance that creates an alias record, Route 53 doesn't create the health check.</p> </li> </ul> <p>For information about the charges for health checks, see <a href="http://aws.amazon.com/route53/pricing">Route 53 Pricing</a>.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct HealthCheckConfig {
    /// <p>The number of consecutive health checks that an endpoint must pass or fail for Route 53 to change the current status of the endpoint from unhealthy to healthy or vice versa. For more information, see <a href="http://docs.aws.amazon.com/Route53/latest/DeveloperGuide/dns-failover-determining-health-of-endpoints.html">How Route 53 Determines Whether an Endpoint Is Healthy</a> in the <i>Route 53 Developer Guide</i>.</p>
    #[serde(rename = "FailureThreshold")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_threshold: Option<i64>,
    /// <p>The path that you want Route 53 to request when performing health checks. The path can be any value for which your endpoint will return an HTTP status code of 2xx or 3xx when the endpoint is healthy, such as the file <code>/docs/route53-health-check.html</code>. Route 53 automatically adds the DNS name for the service and a leading forward slash (<code>/</code>) character. </p>
    #[serde(rename = "ResourcePath")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_path: Option<String>,
    /// <p>The type of health check that you want to create, which indicates how Route 53 determines whether an endpoint is healthy.</p> <important> <p>You can't change the value of <code>Type</code> after you create a health check.</p> </important> <p>You can create the following types of health checks:</p> <ul> <li> <p> <b>HTTP</b>: Route 53 tries to establish a TCP connection. If successful, Route 53 submits an HTTP request and waits for an HTTP status code of 200 or greater and less than 400.</p> </li> <li> <p> <b>HTTPS</b>: Route 53 tries to establish a TCP connection. If successful, Route 53 submits an HTTPS request and waits for an HTTP status code of 200 or greater and less than 400.</p> <important> <p>If you specify HTTPS for the value of <code>Type</code>, the endpoint must support TLS v1.0 or later.</p> </important> </li> <li> <p> <b>TCP</b>: Route 53 tries to establish a TCP connection.</p> </li> </ul> <p>For more information, see <a href="http://docs.aws.amazon.com/Route53/latest/DeveloperGuide/dns-failover-determining-health-of-endpoints.html">How Route 53 Determines Whether an Endpoint Is Healthy</a> in the <i>Route 53 Developer Guide</i>.</p>
    #[serde(rename = "Type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct HealthCheckCustomConfig {
    #[serde(rename = "FailureThreshold")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_threshold: Option<i64>,
}

/// <p>A complex type that contains information about an instance that Amazon Route 53 creates when you submit a <code>RegisterInstance</code> request.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct Instance {
    /// <p>A string map that contains the following information for the service that you specify in <code>ServiceId</code>:</p> <ul> <li> <p>The attributes that apply to the records that are defined in the service. </p> </li> <li> <p>For each attribute, the applicable value.</p> </li> </ul> <p>Supported attribute keys include the following:</p> <p> <b>AWS_ALIAS_DNS_NAME</b> </p> <p> <b/> </p> <p>If you want Route 53 to create an alias record that routes traffic to an Elastic Load Balancing load balancer, specify the DNS name that is associated with the load balancer. For information about how to get the DNS name, see "DNSName" in the topic <a href="http://docs.aws.amazon.com/http:/docs.aws.amazon.com/Route53/latest/APIReference/API_AliasTarget.html">AliasTarget</a>.</p> <p>Note the following:</p> <ul> <li> <p>The configuration for the service that is specified by <code>ServiceId</code> must include settings for an A record, an AAAA record, or both.</p> </li> <li> <p>In the service that is specified by <code>ServiceId</code>, the value of <code>RoutingPolicy</code> must be <code>WEIGHTED</code>.</p> </li> <li> <p>If the service that is specified by <code>ServiceId</code> includes <code>HealthCheckConfig</code> settings, Route 53 will create the health check, but it won't associate the health check with the alias record.</p> </li> <li> <p>Auto naming currently doesn't support creating alias records that route traffic to AWS resources other than ELB load balancers.</p> </li> <li> <p>If you specify a value for <code>AWS_ALIAS_DNS_NAME</code>, don't specify values for any of the <code>AWS_INSTANCE</code> attributes.</p> </li> </ul> <p> <b>AWS_INSTANCE_CNAME</b> </p> <p>If the service configuration includes a CNAME record, the domain name that you want Route 53 to return in response to DNS queries, for example, <code>example.com</code>.</p> <p>This value is required if the service specified by <code>ServiceId</code> includes settings for an CNAME record.</p> <p> <b>AWS_INSTANCE_IPV4</b> </p> <p>If the service configuration includes an A record, the IPv4 address that you want Route 53 to return in response to DNS queries, for example, <code>192.0.2.44</code>.</p> <p>This value is required if the service specified by <code>ServiceId</code> includes settings for an A record. If the service includes settings for an SRV record, you must specify a value for <code>AWS_INSTANCE_IPV4</code>, <code>AWS_INSTANCE_IPV6</code>, or both.</p> <p> <b>AWS_INSTANCE_IPV6</b> </p> <p>If the service configuration includes an AAAA record, the IPv6 address that you want Route 53 to return in response to DNS queries, for example, <code>2001:0db8:85a3:0000:0000:abcd:0001:2345</code>.</p> <p>This value is required if the service specified by <code>ServiceId</code> includes settings for an AAAA record. If the service includes settings for an SRV record, you must specify a value for <code>AWS_INSTANCE_IPV4</code>, <code>AWS_INSTANCE_IPV6</code>, or both.</p> <p> <b>AWS_INSTANCE_PORT</b> </p> <p>If the service includes an SRV record, the value that you want Route 53 to return for the port.</p> <p>If the service includes <code>HealthCheckConfig</code>, the port on the endpoint that you want Route 53 to send requests to. </p> <p>This value is required if you specified settings for an SRV record when you created the service.</p>
    #[serde(rename = "Attributes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attributes: Option<::std::collections::HashMap<String, String>>,
    /// <p>A unique string that identifies the request and that allows failed <code>RegisterInstance</code> requests to be retried without the risk of executing the operation twice. You must use a unique <code>CreatorRequestId</code> string every time you submit a <code>RegisterInstance</code> request if you're registering additional instances for the same namespace and service. <code>CreatorRequestId</code> can be any unique string, for example, a date/time stamp.</p>
    #[serde(rename = "CreatorRequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creator_request_id: Option<String>,
    /// <p><p>An identifier that you want to associate with the instance. Note the following:</p> <ul> <li> <p>If the service that is specified by <code>ServiceId</code> includes settings for an SRV record, the value of <code>InstanceId</code> is automatically included as part of the value for the SRV record. For more information, see <a>DnsRecord$Type</a>.</p> </li> <li> <p>You can use this value to update an existing instance.</p> </li> <li> <p>To register a new instance, you must specify a value that is unique among instances that you register by using the same service. </p> </li> <li> <p>If you specify an existing <code>InstanceId</code> and <code>ServiceId</code>, Route 53 updates the existing records. If there&#39;s also an existing health check, Route 53 deletes the old health check and creates a new one. </p> <note> <p>The health check isn&#39;t deleted immediately, so it will still appear for a while if you submit a <code>ListHealthChecks</code> request, for example.</p> </note> </li> </ul></p>
    #[serde(rename = "Id")]
    pub id: String,
}

/// <p>A complex type that contains information about the instances that you registered by using a specified service.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct InstanceSummary {
    /// <p><p>A string map that contains the following information:</p> <ul> <li> <p>The attributes that are associate with the instance. </p> </li> <li> <p>For each attribute, the applicable value.</p> </li> </ul> <p>Supported attribute keys include the following:</p> <ul> <li> <p> <code>AWS<em>ALIAS</em>DNS<em>NAME</code>: For an alias record that routes traffic to an Elastic Load Balancing load balancer, the DNS name that is associated with the load balancer. </p> </li> <li> <p> <code>AWS</em>INSTANCE<em>CNAME</code>: For a CNAME record, the domain name that Route 53 returns in response to DNS queries, for example, <code>example.com</code>.</p> </li> <li> <p> <code>AWS</em>INSTANCE<em>IPV4</code>: For an A record, the IPv4 address that Route 53 returns in response to DNS queries, for example, <code>192.0.2.44</code>.</p> </li> <li> <p> <code>AWS</em>INSTANCE<em>IPV6</code>: For an AAAA record, the IPv6 address that Route 53 returns in response to DNS queries, for example, <code>2001:0db8:85a3:0000:0000:abcd:0001:2345</code>.</p> </li> <li> <p> <code>AWS</em>INSTANCE_PORT</code>: For an SRV record, the value that Route 53 returns for the port. In addition, if the service includes <code>HealthCheckConfig</code>, the port on the endpoint that Route 53 sends requests to.</p> </li> </ul></p>
    #[serde(rename = "Attributes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attributes: Option<::std::collections::HashMap<String, String>>,
    /// <p>The ID for an instance that you created by using a specified service.</p>
    #[serde(rename = "Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ListInstancesRequest {
    /// <p>The maximum number of instances that you want Amazon Route 53 to return in the response to a <code>ListInstances</code> request. If you don't specify a value for <code>MaxResults</code>, Route 53 returns up to 100 instances.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>For the first <code>ListInstances</code> request, omit this value.</p> <p>If more than <code>MaxResults</code> instances match the specified criteria, you can submit another <code>ListInstances</code> request to get the next group of results. Specify the value of <code>NextToken</code> from the previous response in the next request.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The ID of the service that you want to list instances for.</p>
    #[serde(rename = "ServiceId")]
    pub service_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct ListInstancesResponse {
    /// <p>Summary information about the instances that are associated with the specified service.</p>
    #[serde(rename = "Instances")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instances: Option<Vec<InstanceSummary>>,
    /// <p>If more than <code>MaxResults</code> instances match the specified criteria, you can submit another <code>ListInstances</code> request to get the next group of results. Specify the value of <code>NextToken</code> from the previous response in the next request.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ListNamespacesRequest {
    /// <p>A complex type that contains specifications for the namespaces that you want to list.</p> <p>If you specify more than one filter, a namespace must match all filters to be returned by <code>ListNamespaces</code>.</p>
    #[serde(rename = "Filters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<Vec<NamespaceFilter>>,
    /// <p>The maximum number of namespaces that you want Amazon Route 53 to return in the response to a <code>ListNamespaces</code> request. If you don't specify a value for <code>MaxResults</code>, Route 53 returns up to 100 namespaces.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p><p>For the first <code>ListNamespaces</code> request, omit this value.</p> <p>If the response contains <code>NextToken</code>, submit another <code>ListNamespaces</code> request to get the next group of results. Specify the value of <code>NextToken</code> from the previous response in the next request.</p> <note> <p>Route 53 gets <code>MaxResults</code> namespaces and then filters them based on the specified criteria. It&#39;s possible that no namespaces in the first <code>MaxResults</code> namespaces matched the specified criteria but that subsequent groups of <code>MaxResults</code> namespaces do contain namespaces that match the criteria.</p> </note></p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct ListNamespacesResponse {
    /// <p>An array that contains one <code>NamespaceSummary</code> object for each namespace that matches the specified filter criteria.</p>
    #[serde(rename = "Namespaces")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub namespaces: Option<Vec<NamespaceSummary>>,
    /// <p><p>If the response contains <code>NextToken</code>, submit another <code>ListNamespaces</code> request to get the next group of results. Specify the value of <code>NextToken</code> from the previous response in the next request.</p> <note> <p>Route 53 gets <code>MaxResults</code> namespaces and then filters them based on the specified criteria. It&#39;s possible that no namespaces in the first <code>MaxResults</code> namespaces matched the specified criteria but that subsequent groups of <code>MaxResults</code> namespaces do contain namespaces that match the criteria.</p> </note></p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ListOperationsRequest {
    /// <p>A complex type that contains specifications for the operations that you want to list, for example, operations that you started between a specified start date and end date.</p> <p>If you specify more than one filter, an operation must match all filters to be returned by <code>ListOperations</code>.</p>
    #[serde(rename = "Filters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<Vec<OperationFilter>>,
    /// <p>The maximum number of items that you want Amazon Route 53 to return in the response to a <code>ListOperations</code> request. If you don't specify a value for <code>MaxResults</code>, Route 53 returns up to 100 operations.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p><p>For the first <code>ListOperations</code> request, omit this value.</p> <p>If the response contains <code>NextToken</code>, submit another <code>ListOperations</code> request to get the next group of results. Specify the value of <code>NextToken</code> from the previous response in the next request.</p> <note> <p>Route 53 gets <code>MaxResults</code> operations and then filters them based on the specified criteria. It&#39;s possible that no operations in the first <code>MaxResults</code> operations matched the specified criteria but that subsequent groups of <code>MaxResults</code> operations do contain operations that match the criteria.</p> </note></p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct ListOperationsResponse {
    /// <p><p>If the response contains <code>NextToken</code>, submit another <code>ListOperations</code> request to get the next group of results. Specify the value of <code>NextToken</code> from the previous response in the next request.</p> <note> <p>Route 53 gets <code>MaxResults</code> operations and then filters them based on the specified criteria. It&#39;s possible that no operations in the first <code>MaxResults</code> operations matched the specified criteria but that subsequent groups of <code>MaxResults</code> operations do contain operations that match the criteria.</p> </note></p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>Summary information about the operations that match the specified criteria.</p>
    #[serde(rename = "Operations")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operations: Option<Vec<OperationSummary>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ListServicesRequest {
    /// <p>A complex type that contains specifications for the namespaces that you want to list services for. </p> <p>If you specify more than one filter, an operation must match all filters to be returned by <code>ListServices</code>.</p>
    #[serde(rename = "Filters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<Vec<ServiceFilter>>,
    /// <p>The maximum number of services that you want Amazon Route 53 to return in the response to a <code>ListServices</code> request. If you don't specify a value for <code>MaxResults</code>, Route 53 returns up to 100 services.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p><p>For the first <code>ListServices</code> request, omit this value.</p> <p>If the response contains <code>NextToken</code>, submit another <code>ListServices</code> request to get the next group of results. Specify the value of <code>NextToken</code> from the previous response in the next request.</p> <note> <p>Route 53 gets <code>MaxResults</code> services and then filters them based on the specified criteria. It&#39;s possible that no services in the first <code>MaxResults</code> services matched the specified criteria but that subsequent groups of <code>MaxResults</code> services do contain services that match the criteria.</p> </note></p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct ListServicesResponse {
    /// <p><p>If the response contains <code>NextToken</code>, submit another <code>ListServices</code> request to get the next group of results. Specify the value of <code>NextToken</code> from the previous response in the next request.</p> <note> <p>Route 53 gets <code>MaxResults</code> services and then filters them based on the specified criteria. It&#39;s possible that no services in the first <code>MaxResults</code> services matched the specified criteria but that subsequent groups of <code>MaxResults</code> services do contain services that match the criteria.</p> </note></p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>An array that contains one <code>ServiceSummary</code> object for each service that matches the specified filter criteria.</p>
    #[serde(rename = "Services")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub services: Option<Vec<ServiceSummary>>,
}

/// <p>A complex type that contains information about a specified namespace.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct Namespace {
    /// <p>The Amazon Resource Name (ARN) that Route 53 assigns to the namespace when you create it.</p>
    #[serde(rename = "Arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p>The date that the namespace was created, in Unix date/time format and Coordinated Universal Time (UTC). The value of <code>CreateDate</code> is accurate to milliseconds. For example, the value <code>1516925490.087</code> represents Friday, January 26, 2018 12:11:30.087 AM.</p>
    #[serde(rename = "CreateDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_date: Option<f64>,
    /// <p>A unique string that identifies the request and that allows failed requests to be retried without the risk of executing an operation twice. </p>
    #[serde(rename = "CreatorRequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creator_request_id: Option<String>,
    /// <p>The description that you specify for the namespace when you create it.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The ID of a namespace.</p>
    #[serde(rename = "Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// <p>The name of the namespace, such as <code>example.com</code>.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>A complex type that contains information that's specific to the type of the namespace.</p>
    #[serde(rename = "Properties")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<NamespaceProperties>,
    /// <p>The number of services that are associated with the namespace.</p>
    #[serde(rename = "ServiceCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_count: Option<i64>,
    /// <p>The type of the namespace. Valid values are <code>DNS_PUBLIC</code> and <code>DNS_PRIVATE</code>.</p>
    #[serde(rename = "Type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}

/// <p>A complex type that identifies the namespaces that you want to list. You can choose to list public or private namespaces.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct NamespaceFilter {
    /// <p><p>The operator that you want to use to determine whether <code>ListNamespaces</code> returns a namespace. Valid values for <code>condition</code> include:</p> <ul> <li> <p> <code>EQ</code>: When you specify <code>EQ</code> for the condition, you can choose to list only public namespaces or private namespaces, but not both. <code>EQ</code> is the default condition and can be omitted.</p> </li> <li> <p> <code>IN</code>: When you specify <code>IN</code> for the condition, you can choose to list public namespaces, private namespaces, or both. </p> </li> <li> <p> <code>BETWEEN</code>: Not applicable</p> </li> </ul></p>
    #[serde(rename = "Condition")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub condition: Option<String>,
    /// <p>Specify <code>TYPE</code>.</p>
    #[serde(rename = "Name")]
    pub name: String,
    /// <p>If you specify <code>EQ</code> for <code>Condition</code>, specify either <code>DNS_PUBLIC</code> or <code>DNS_PRIVATE</code>.</p> <p>If you specify <code>IN</code> for <code>Condition</code>, you can specify <code>DNS_PUBLIC</code>, <code>DNS_PRIVATE</code>, or both.</p>
    #[serde(rename = "Values")]
    pub values: Vec<String>,
}

/// <p>A complex type that contains information that is specific to the namespace type.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct NamespaceProperties {
    /// <p>A complex type that contains the ID for the hosted zone that Route 53 creates when you create a namespace.</p>
    #[serde(rename = "DnsProperties")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dns_properties: Option<DnsProperties>,
}

/// <p>A complex type that contains information about a namespace.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct NamespaceSummary {
    /// <p>The Amazon Resource Name (ARN) that Route 53 assigns to the namespace when you create it.</p>
    #[serde(rename = "Arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p>The ID of the namespace.</p>
    #[serde(rename = "Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// <p>The name of the namespace. When you create a namespace, Route 53 automatically creates a hosted zone that has the same name as the namespace.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The type of the namespace, either public or private.</p>
    #[serde(rename = "Type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}

/// <p>A complex type that contains information about a specified operation.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct Operation {
    /// <p>The date and time that the request was submitted, in Unix date/time format and Coordinated Universal Time (UTC). The value of <code>CreateDate</code> is accurate to milliseconds. For example, the value <code>1516925490.087</code> represents Friday, January 26, 2018 12:11:30.087 AM.</p>
    #[serde(rename = "CreateDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_date: Option<f64>,
    /// <p><p>The code associated with <code>ErrorMessage</code>. Values for <code>ErrorCode</code> include the following:</p> <ul> <li> <p> <code>ACCESS<em>DENIED</code> </p> </li> <li> <p> <code>CANNOT</em>CREATE<em>HOSTED</em>ZONE</code> </p> </li> <li> <p> <code>EXPIRED<em>TOKEN</code> </p> </li> <li> <p> <code>HOSTED</em>ZONE<em>NOT</em>FOUND</code> </p> </li> <li> <p> <code>INTERNAL<em>FAILURE</code> </p> </li> <li> <p> <code>INVALID</em>CHANGE<em>BATCH</code> </p> </li> <li> <p> <code>THROTTLED</em>REQUEST</code> </p> </li> </ul></p>
    #[serde(rename = "ErrorCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_code: Option<String>,
    /// <p>If the value of <code>Status</code> is <code>FAIL</code>, the reason that the operation failed.</p>
    #[serde(rename = "ErrorMessage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
    /// <p>The ID of the operation that you want to get information about.</p>
    #[serde(rename = "Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// <p><p>The status of the operation. Values include the following:</p> <ul> <li> <p> <b>SUBMITTED</b>: This is the initial state immediately after you submit a request.</p> </li> <li> <p> <b>PENDING</b>: Route 53 is performing the operation.</p> </li> <li> <p> <b>SUCCESS</b>: The operation succeeded.</p> </li> <li> <p> <b>FAIL</b>: The operation failed. For the failure reason, see <code>ErrorMessage</code>.</p> </li> </ul></p>
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// <p><p>The name of the target entity that is associated with the operation:</p> <ul> <li> <p> <b>NAMESPACE</b>: The namespace ID is returned in the <code>ResourceId</code> property.</p> </li> <li> <p> <b>SERVICE</b>: The service ID is returned in the <code>ResourceId</code> property.</p> </li> <li> <p> <b>INSTANCE</b>: The instance ID is returned in the <code>ResourceId</code> property.</p> </li> </ul></p>
    #[serde(rename = "Targets")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub targets: Option<::std::collections::HashMap<String, String>>,
    /// <p>The name of the operation that is associated with the specified ID.</p>
    #[serde(rename = "Type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    /// <p>The date and time that the value of <code>Status</code> changed to the current value, in Unix date/time format and Coordinated Universal Time (UTC). The value of <code>UpdateDate</code> is accurate to milliseconds. For example, the value <code>1516925490.087</code> represents Friday, January 26, 2018 12:11:30.087 AM.</p>
    #[serde(rename = "UpdateDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_date: Option<f64>,
}

/// <p>A complex type that lets you select the operations that you want to list.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct OperationFilter {
    /// <p><p>The operator that you want to use to determine whether an operation matches the specified value. Valid values for condition include:</p> <ul> <li> <p> <code>EQ</code>: When you specify <code>EQ</code> for the condition, you can specify only one value. <code>EQ</code> is supported for <code>NAMESPACE<em>ID</code>, <code>SERVICE</em>ID</code>, <code>STATUS</code>, and <code>TYPE</code>. <code>EQ</code> is the default condition and can be omitted.</p> </li> <li> <p> <code>IN</code>: When you specify <code>IN</code> for the condition, you can specify a list of one or more values. <code>IN</code> is supported for <code>STATUS</code> and <code>TYPE</code>. An operation must match one of the specified values to be returned in the response.</p> </li> <li> <p> <code>BETWEEN</code>: Specify a start date and an end date in Unix date/time format and Coordinated Universal Time (UTC). The start date must be the first value. <code>BETWEEN</code> is supported for <code>UPDATE_DATE</code>. </p> </li> </ul></p>
    #[serde(rename = "Condition")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub condition: Option<String>,
    /// <p><p>Specify the operations that you want to get:</p> <ul> <li> <p> <b>NAMESPACE<em>ID</b>: Gets operations related to specified namespaces.</p> </li> <li> <p> <b>SERVICE</em>ID</b>: Gets operations related to specified services.</p> </li> <li> <p> <b>STATUS</b>: Gets operations based on the status of the operations: <code>SUBMITTED</code>, <code>PENDING</code>, <code>SUCCEED</code>, or <code>FAIL</code>.</p> </li> <li> <p> <b>TYPE</b>: Gets specified types of operation.</p> </li> <li> <p> <b>UPDATE_DATE</b>: Gets operations that changed status during a specified date/time range. </p> </li> </ul></p>
    #[serde(rename = "Name")]
    pub name: String,
    /// <p><p>Specify values that are applicable to the value that you specify for <code>Name</code>: </p> <ul> <li> <p> <b>NAMESPACE<em>ID</b>: Specify one namespace ID.</p> </li> <li> <p> <b>SERVICE</em>ID</b>: Specify one service ID.</p> </li> <li> <p> <b>STATUS</b>: Specify one or more statuses: <code>SUBMITTED</code>, <code>PENDING</code>, <code>SUCCEED</code>, or <code>FAIL</code>.</p> </li> <li> <p> <b>TYPE</b>: Specify one or more of the following types: <code>CREATE<em>NAMESPACE</code>, <code>DELETE</em>NAMESPACE</code>, <code>UPDATE<em>SERVICE</code>, <code>REGISTER</em>INSTANCE</code>, or <code>DEREGISTER<em>INSTANCE</code>.</p> </li> <li> <p> <b>UPDATE</em>DATE</b>: Specify a start date and an end date in Unix date/time format and Coordinated Universal Time (UTC). The start date must be the first value.</p> </li> </ul></p>
    #[serde(rename = "Values")]
    pub values: Vec<String>,
}

/// <p>A complex type that contains information about an operation that matches the criteria that you specified in a <a>ListOperations</a> request.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct OperationSummary {
    /// <p>The ID for an operation.</p>
    #[serde(rename = "Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// <p><p>The status of the operation. Values include the following:</p> <ul> <li> <p> <b>SUBMITTED</b>: This is the initial state immediately after you submit a request.</p> </li> <li> <p> <b>PENDING</b>: Route 53 is performing the operation.</p> </li> <li> <p> <b>SUCCESS</b>: The operation succeeded.</p> </li> <li> <p> <b>FAIL</b>: The operation failed. For the failure reason, see <code>ErrorMessage</code>.</p> </li> </ul></p>
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct RegisterInstanceRequest {
    /// <p>A string map that contains the following information for the service that you specify in <code>ServiceId</code>:</p> <ul> <li> <p>The attributes that apply to the records that are defined in the service. </p> </li> <li> <p>For each attribute, the applicable value.</p> </li> </ul> <p>Supported attribute keys include the following:</p> <p> <b>AWS_ALIAS_DNS_NAME</b> </p> <p> <b/> </p> <p>If you want Route 53 to create an alias record that routes traffic to an Elastic Load Balancing load balancer, specify the DNS name that is associated with the load balancer. For information about how to get the DNS name, see "DNSName" in the topic <a href="http://docs.aws.amazon.com/http:/docs.aws.amazon.com/Route53/latest/APIReference/API_AliasTarget.html">AliasTarget</a>.</p> <p>Note the following:</p> <ul> <li> <p>The configuration for the service that is specified by <code>ServiceId</code> must include settings for an A record, an AAAA record, or both.</p> </li> <li> <p>In the service that is specified by <code>ServiceId</code>, the value of <code>RoutingPolicy</code> must be <code>WEIGHTED</code>.</p> </li> <li> <p>If the service that is specified by <code>ServiceId</code> includes <code>HealthCheckConfig</code> settings, Route 53 will create the health check, but it won't associate the health check with the alias record.</p> </li> <li> <p>Auto naming currently doesn't support creating alias records that route traffic to AWS resources other than ELB load balancers.</p> </li> <li> <p>If you specify a value for <code>AWS_ALIAS_DNS_NAME</code>, don't specify values for any of the <code>AWS_INSTANCE</code> attributes.</p> </li> </ul> <p> <b>AWS_INSTANCE_CNAME</b> </p> <p>If the service configuration includes a CNAME record, the domain name that you want Route 53 to return in response to DNS queries, for example, <code>example.com</code>.</p> <p>This value is required if the service specified by <code>ServiceId</code> includes settings for an CNAME record.</p> <p> <b>AWS_INSTANCE_IPV4</b> </p> <p>If the service configuration includes an A record, the IPv4 address that you want Route 53 to return in response to DNS queries, for example, <code>192.0.2.44</code>.</p> <p>This value is required if the service specified by <code>ServiceId</code> includes settings for an A record. If the service includes settings for an SRV record, you must specify a value for <code>AWS_INSTANCE_IPV4</code>, <code>AWS_INSTANCE_IPV6</code>, or both.</p> <p> <b>AWS_INSTANCE_IPV6</b> </p> <p>If the service configuration includes an AAAA record, the IPv6 address that you want Route 53 to return in response to DNS queries, for example, <code>2001:0db8:85a3:0000:0000:abcd:0001:2345</code>.</p> <p>This value is required if the service specified by <code>ServiceId</code> includes settings for an AAAA record. If the service includes settings for an SRV record, you must specify a value for <code>AWS_INSTANCE_IPV4</code>, <code>AWS_INSTANCE_IPV6</code>, or both.</p> <p> <b>AWS_INSTANCE_PORT</b> </p> <p>If the service includes an SRV record, the value that you want Route 53 to return for the port.</p> <p>If the service includes <code>HealthCheckConfig</code>, the port on the endpoint that you want Route 53 to send requests to. </p> <p>This value is required if you specified settings for an SRV record when you created the service.</p>
    #[serde(rename = "Attributes")]
    pub attributes: ::std::collections::HashMap<String, String>,
    /// <p>A unique string that identifies the request and that allows failed <code>RegisterInstance</code> requests to be retried without the risk of executing the operation twice. You must use a unique <code>CreatorRequestId</code> string every time you submit a <code>RegisterInstance</code> request if you're registering additional instances for the same namespace and service. <code>CreatorRequestId</code> can be any unique string, for example, a date/time stamp.</p>
    #[serde(rename = "CreatorRequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creator_request_id: Option<String>,
    /// <p><p>An identifier that you want to associate with the instance. Note the following:</p> <ul> <li> <p>If the service that is specified by <code>ServiceId</code> includes settings for an SRV record, the value of <code>InstanceId</code> is automatically included as part of the value for the SRV record. For more information, see <a>DnsRecord$Type</a>.</p> </li> <li> <p>You can use this value to update an existing instance.</p> </li> <li> <p>To register a new instance, you must specify a value that is unique among instances that you register by using the same service. </p> </li> <li> <p>If you specify an existing <code>InstanceId</code> and <code>ServiceId</code>, Route 53 updates the existing records. If there&#39;s also an existing health check, Route 53 deletes the old health check and creates a new one. </p> <note> <p>The health check isn&#39;t deleted immediately, so it will still appear for a while if you submit a <code>ListHealthChecks</code> request, for example.</p> </note> </li> </ul></p>
    #[serde(rename = "InstanceId")]
    pub instance_id: String,
    /// <p>The ID of the service that you want to use for settings for the records and health check that Route 53 will create.</p>
    #[serde(rename = "ServiceId")]
    pub service_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct RegisterInstanceResponse {
    /// <p>A value that you can use to determine whether the request completed successfully. To get the status of the operation, see <a>GetOperation</a>.</p>
    #[serde(rename = "OperationId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operation_id: Option<String>,
}

/// <p>A complex type that contains information about the specified service.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct Service {
    /// <p>The Amazon Resource Name (ARN) that Route 53 assigns to the service when you create it.</p>
    #[serde(rename = "Arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p>The date and time that the service was created, in Unix format and Coordinated Universal Time (UTC). The value of <code>CreateDate</code> is accurate to milliseconds. For example, the value <code>1516925490.087</code> represents Friday, January 26, 2018 12:11:30.087 AM.</p>
    #[serde(rename = "CreateDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_date: Option<f64>,
    /// <p>A unique string that identifies the request and that allows failed requests to be retried without the risk of executing the operation twice. <code>CreatorRequestId</code> can be any unique string, for example, a date/time stamp.</p>
    #[serde(rename = "CreatorRequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creator_request_id: Option<String>,
    /// <p>The description of the service.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>A complex type that contains information about the records that you want Route 53 to create when you register an instance.</p>
    #[serde(rename = "DnsConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dns_config: Option<DnsConfig>,
    /// <p> <i>Public DNS namespaces only.</i> A complex type that contains settings for an optional health check. If you specify settings for a health check, Route 53 associates the health check with all the records that you specify in <code>DnsConfig</code>.</p> <p>For information about the charges for health checks, see <a href="http://aws.amazon.com/route53/pricing">Route 53 Pricing</a>.</p>
    #[serde(rename = "HealthCheckConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub health_check_config: Option<HealthCheckConfig>,
    #[serde(rename = "HealthCheckCustomConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub health_check_custom_config: Option<HealthCheckCustomConfig>,
    /// <p>The ID that Route 53 assigned to the service when you created it.</p>
    #[serde(rename = "Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// <p>The number of instances that are currently associated with the service. Instances that were previously associated with the service but that have been deleted are not included in the count.</p>
    #[serde(rename = "InstanceCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_count: Option<i64>,
    /// <p>The name of the service.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

/// <p>A complex type that contains changes to an existing service.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ServiceChange {
    /// <p>A description for the service.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>A complex type that contains information about the records that you want Route 53 to create when you register an instance.</p>
    #[serde(rename = "DnsConfig")]
    pub dns_config: DnsConfigChange,
    #[serde(rename = "HealthCheckConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub health_check_config: Option<HealthCheckConfig>,
}

/// <p>A complex type that lets you specify the namespaces that you want to list services for.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ServiceFilter {
    /// <p><p>The operator that you want to use to determine whether a service is returned by <code>ListServices</code>. Valid values for <code>Condition</code> include the following:</p> <ul> <li> <p> <code>EQ</code>: When you specify <code>EQ</code>, specify one namespace ID for <code>Values</code>. <code>EQ</code> is the default condition and can be omitted.</p> </li> <li> <p> <code>IN</code>: When you specify <code>IN</code>, specify a list of the IDs for the namespaces that you want <code>ListServices</code> to return a list of services for.</p> </li> <li> <p> <code>BETWEEN</code>: Not applicable.</p> </li> </ul></p>
    #[serde(rename = "Condition")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub condition: Option<String>,
    /// <p>Specify <code>NAMESPACE_ID</code>.</p>
    #[serde(rename = "Name")]
    pub name: String,
    /// <p>The values that are applicable to the value that you specify for <code>Condition</code> to filter the list of services.</p>
    #[serde(rename = "Values")]
    pub values: Vec<String>,
}

/// <p>A complex type that contains information about a specified service.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct ServiceSummary {
    /// <p>The Amazon Resource Name (ARN) that Route 53 assigns to the service when you create it.</p>
    #[serde(rename = "Arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p>The description that you specify when you create the service.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The ID that Route 53 assigned to the service when you created it.</p>
    #[serde(rename = "Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// <p>The number of instances that are currently associated with the service. Instances that were previously associated with the service but that have been deleted are not included in the count.</p>
    #[serde(rename = "InstanceCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_count: Option<i64>,
    /// <p>The name of the service.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct UpdateInstanceCustomHealthStatusRequest {
    #[serde(rename = "InstanceId")]
    pub instance_id: String,
    #[serde(rename = "ServiceId")]
    pub service_id: String,
    #[serde(rename = "Status")]
    pub status: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct UpdateServiceRequest {
    /// <p>The ID of the service that you want to update.</p>
    #[serde(rename = "Id")]
    pub id: String,
    /// <p>A complex type that contains the new settings for the service.</p>
    #[serde(rename = "Service")]
    pub service: ServiceChange,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct UpdateServiceResponse {
    /// <p>A value that you can use to determine whether the request completed successfully. To get the status of the operation, see <a>GetOperation</a>.</p>
    #[serde(rename = "OperationId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operation_id: Option<String>,
}

/// Errors returned by CreatePrivateDnsNamespace
#[derive(Debug, PartialEq)]
pub enum CreatePrivateDnsNamespaceError {
    /// <p>The operation is already in progress.</p>
    DuplicateRequest(String),
    /// <p>One or more specified values aren't valid. For example, when you're creating a namespace, the value of <code>Name</code> might not be a valid DNS name.</p>
    InvalidInput(String),
    /// <p>The namespace that you're trying to create already exists.</p>
    NamespaceAlreadyExists(String),
    /// <p>The resource can't be created because you've reached the limit on the number of resources.</p>
    ResourceLimitExceeded(String),
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

impl CreatePrivateDnsNamespaceError {
    pub fn from_response(res: BufferedHttpResponse) -> CreatePrivateDnsNamespaceError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "DuplicateRequest" => {
                    return CreatePrivateDnsNamespaceError::DuplicateRequest(String::from(
                        error_message,
                    ))
                }
                "InvalidInput" => {
                    return CreatePrivateDnsNamespaceError::InvalidInput(String::from(error_message))
                }
                "NamespaceAlreadyExists" => {
                    return CreatePrivateDnsNamespaceError::NamespaceAlreadyExists(String::from(
                        error_message,
                    ))
                }
                "ResourceLimitExceeded" => {
                    return CreatePrivateDnsNamespaceError::ResourceLimitExceeded(String::from(
                        error_message,
                    ))
                }
                "ValidationException" => {
                    return CreatePrivateDnsNamespaceError::Validation(error_message.to_string())
                }
                _ => {}
            }
        }
        return CreatePrivateDnsNamespaceError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for CreatePrivateDnsNamespaceError {
    fn from(err: serde_json::error::Error) -> CreatePrivateDnsNamespaceError {
        CreatePrivateDnsNamespaceError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for CreatePrivateDnsNamespaceError {
    fn from(err: CredentialsError) -> CreatePrivateDnsNamespaceError {
        CreatePrivateDnsNamespaceError::Credentials(err)
    }
}
impl From<HttpDispatchError> for CreatePrivateDnsNamespaceError {
    fn from(err: HttpDispatchError) -> CreatePrivateDnsNamespaceError {
        CreatePrivateDnsNamespaceError::HttpDispatch(err)
    }
}
impl From<io::Error> for CreatePrivateDnsNamespaceError {
    fn from(err: io::Error) -> CreatePrivateDnsNamespaceError {
        CreatePrivateDnsNamespaceError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for CreatePrivateDnsNamespaceError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreatePrivateDnsNamespaceError {
    fn description(&self) -> &str {
        match *self {
            CreatePrivateDnsNamespaceError::DuplicateRequest(ref cause) => cause,
            CreatePrivateDnsNamespaceError::InvalidInput(ref cause) => cause,
            CreatePrivateDnsNamespaceError::NamespaceAlreadyExists(ref cause) => cause,
            CreatePrivateDnsNamespaceError::ResourceLimitExceeded(ref cause) => cause,
            CreatePrivateDnsNamespaceError::Validation(ref cause) => cause,
            CreatePrivateDnsNamespaceError::Credentials(ref err) => err.description(),
            CreatePrivateDnsNamespaceError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            CreatePrivateDnsNamespaceError::ParseError(ref cause) => cause,
            CreatePrivateDnsNamespaceError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by CreatePublicDnsNamespace
#[derive(Debug, PartialEq)]
pub enum CreatePublicDnsNamespaceError {
    /// <p>The operation is already in progress.</p>
    DuplicateRequest(String),
    /// <p>One or more specified values aren't valid. For example, when you're creating a namespace, the value of <code>Name</code> might not be a valid DNS name.</p>
    InvalidInput(String),
    /// <p>The namespace that you're trying to create already exists.</p>
    NamespaceAlreadyExists(String),
    /// <p>The resource can't be created because you've reached the limit on the number of resources.</p>
    ResourceLimitExceeded(String),
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

impl CreatePublicDnsNamespaceError {
    pub fn from_response(res: BufferedHttpResponse) -> CreatePublicDnsNamespaceError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "DuplicateRequest" => {
                    return CreatePublicDnsNamespaceError::DuplicateRequest(String::from(
                        error_message,
                    ))
                }
                "InvalidInput" => {
                    return CreatePublicDnsNamespaceError::InvalidInput(String::from(error_message))
                }
                "NamespaceAlreadyExists" => {
                    return CreatePublicDnsNamespaceError::NamespaceAlreadyExists(String::from(
                        error_message,
                    ))
                }
                "ResourceLimitExceeded" => {
                    return CreatePublicDnsNamespaceError::ResourceLimitExceeded(String::from(
                        error_message,
                    ))
                }
                "ValidationException" => {
                    return CreatePublicDnsNamespaceError::Validation(error_message.to_string())
                }
                _ => {}
            }
        }
        return CreatePublicDnsNamespaceError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for CreatePublicDnsNamespaceError {
    fn from(err: serde_json::error::Error) -> CreatePublicDnsNamespaceError {
        CreatePublicDnsNamespaceError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for CreatePublicDnsNamespaceError {
    fn from(err: CredentialsError) -> CreatePublicDnsNamespaceError {
        CreatePublicDnsNamespaceError::Credentials(err)
    }
}
impl From<HttpDispatchError> for CreatePublicDnsNamespaceError {
    fn from(err: HttpDispatchError) -> CreatePublicDnsNamespaceError {
        CreatePublicDnsNamespaceError::HttpDispatch(err)
    }
}
impl From<io::Error> for CreatePublicDnsNamespaceError {
    fn from(err: io::Error) -> CreatePublicDnsNamespaceError {
        CreatePublicDnsNamespaceError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for CreatePublicDnsNamespaceError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreatePublicDnsNamespaceError {
    fn description(&self) -> &str {
        match *self {
            CreatePublicDnsNamespaceError::DuplicateRequest(ref cause) => cause,
            CreatePublicDnsNamespaceError::InvalidInput(ref cause) => cause,
            CreatePublicDnsNamespaceError::NamespaceAlreadyExists(ref cause) => cause,
            CreatePublicDnsNamespaceError::ResourceLimitExceeded(ref cause) => cause,
            CreatePublicDnsNamespaceError::Validation(ref cause) => cause,
            CreatePublicDnsNamespaceError::Credentials(ref err) => err.description(),
            CreatePublicDnsNamespaceError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            CreatePublicDnsNamespaceError::ParseError(ref cause) => cause,
            CreatePublicDnsNamespaceError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by CreateService
#[derive(Debug, PartialEq)]
pub enum CreateServiceError {
    /// <p>One or more specified values aren't valid. For example, when you're creating a namespace, the value of <code>Name</code> might not be a valid DNS name.</p>
    InvalidInput(String),
    /// <p>No namespace exists with the specified ID.</p>
    NamespaceNotFound(String),
    /// <p>The resource can't be created because you've reached the limit on the number of resources.</p>
    ResourceLimitExceeded(String),
    /// <p>The service can't be created because a service with the same name already exists.</p>
    ServiceAlreadyExists(String),
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

impl CreateServiceError {
    pub fn from_response(res: BufferedHttpResponse) -> CreateServiceError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "InvalidInput" => {
                    return CreateServiceError::InvalidInput(String::from(error_message))
                }
                "NamespaceNotFound" => {
                    return CreateServiceError::NamespaceNotFound(String::from(error_message))
                }
                "ResourceLimitExceeded" => {
                    return CreateServiceError::ResourceLimitExceeded(String::from(error_message))
                }
                "ServiceAlreadyExists" => {
                    return CreateServiceError::ServiceAlreadyExists(String::from(error_message))
                }
                "ValidationException" => {
                    return CreateServiceError::Validation(error_message.to_string())
                }
                _ => {}
            }
        }
        return CreateServiceError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for CreateServiceError {
    fn from(err: serde_json::error::Error) -> CreateServiceError {
        CreateServiceError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for CreateServiceError {
    fn from(err: CredentialsError) -> CreateServiceError {
        CreateServiceError::Credentials(err)
    }
}
impl From<HttpDispatchError> for CreateServiceError {
    fn from(err: HttpDispatchError) -> CreateServiceError {
        CreateServiceError::HttpDispatch(err)
    }
}
impl From<io::Error> for CreateServiceError {
    fn from(err: io::Error) -> CreateServiceError {
        CreateServiceError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for CreateServiceError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateServiceError {
    fn description(&self) -> &str {
        match *self {
            CreateServiceError::InvalidInput(ref cause) => cause,
            CreateServiceError::NamespaceNotFound(ref cause) => cause,
            CreateServiceError::ResourceLimitExceeded(ref cause) => cause,
            CreateServiceError::ServiceAlreadyExists(ref cause) => cause,
            CreateServiceError::Validation(ref cause) => cause,
            CreateServiceError::Credentials(ref err) => err.description(),
            CreateServiceError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            CreateServiceError::ParseError(ref cause) => cause,
            CreateServiceError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by DeleteNamespace
#[derive(Debug, PartialEq)]
pub enum DeleteNamespaceError {
    /// <p>The operation is already in progress.</p>
    DuplicateRequest(String),
    /// <p>One or more specified values aren't valid. For example, when you're creating a namespace, the value of <code>Name</code> might not be a valid DNS name.</p>
    InvalidInput(String),
    /// <p>No namespace exists with the specified ID.</p>
    NamespaceNotFound(String),
    /// <p>The specified resource can't be deleted because it contains other resources. For example, you can't delete a service that contains any instances.</p>
    ResourceInUse(String),
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

impl DeleteNamespaceError {
    pub fn from_response(res: BufferedHttpResponse) -> DeleteNamespaceError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "DuplicateRequest" => {
                    return DeleteNamespaceError::DuplicateRequest(String::from(error_message))
                }
                "InvalidInput" => {
                    return DeleteNamespaceError::InvalidInput(String::from(error_message))
                }
                "NamespaceNotFound" => {
                    return DeleteNamespaceError::NamespaceNotFound(String::from(error_message))
                }
                "ResourceInUse" => {
                    return DeleteNamespaceError::ResourceInUse(String::from(error_message))
                }
                "ValidationException" => {
                    return DeleteNamespaceError::Validation(error_message.to_string())
                }
                _ => {}
            }
        }
        return DeleteNamespaceError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for DeleteNamespaceError {
    fn from(err: serde_json::error::Error) -> DeleteNamespaceError {
        DeleteNamespaceError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for DeleteNamespaceError {
    fn from(err: CredentialsError) -> DeleteNamespaceError {
        DeleteNamespaceError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DeleteNamespaceError {
    fn from(err: HttpDispatchError) -> DeleteNamespaceError {
        DeleteNamespaceError::HttpDispatch(err)
    }
}
impl From<io::Error> for DeleteNamespaceError {
    fn from(err: io::Error) -> DeleteNamespaceError {
        DeleteNamespaceError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DeleteNamespaceError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteNamespaceError {
    fn description(&self) -> &str {
        match *self {
            DeleteNamespaceError::DuplicateRequest(ref cause) => cause,
            DeleteNamespaceError::InvalidInput(ref cause) => cause,
            DeleteNamespaceError::NamespaceNotFound(ref cause) => cause,
            DeleteNamespaceError::ResourceInUse(ref cause) => cause,
            DeleteNamespaceError::Validation(ref cause) => cause,
            DeleteNamespaceError::Credentials(ref err) => err.description(),
            DeleteNamespaceError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            DeleteNamespaceError::ParseError(ref cause) => cause,
            DeleteNamespaceError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by DeleteService
#[derive(Debug, PartialEq)]
pub enum DeleteServiceError {
    /// <p>One or more specified values aren't valid. For example, when you're creating a namespace, the value of <code>Name</code> might not be a valid DNS name.</p>
    InvalidInput(String),
    /// <p>The specified resource can't be deleted because it contains other resources. For example, you can't delete a service that contains any instances.</p>
    ResourceInUse(String),
    /// <p>No service exists with the specified ID.</p>
    ServiceNotFound(String),
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

impl DeleteServiceError {
    pub fn from_response(res: BufferedHttpResponse) -> DeleteServiceError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "InvalidInput" => {
                    return DeleteServiceError::InvalidInput(String::from(error_message))
                }
                "ResourceInUse" => {
                    return DeleteServiceError::ResourceInUse(String::from(error_message))
                }
                "ServiceNotFound" => {
                    return DeleteServiceError::ServiceNotFound(String::from(error_message))
                }
                "ValidationException" => {
                    return DeleteServiceError::Validation(error_message.to_string())
                }
                _ => {}
            }
        }
        return DeleteServiceError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for DeleteServiceError {
    fn from(err: serde_json::error::Error) -> DeleteServiceError {
        DeleteServiceError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for DeleteServiceError {
    fn from(err: CredentialsError) -> DeleteServiceError {
        DeleteServiceError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DeleteServiceError {
    fn from(err: HttpDispatchError) -> DeleteServiceError {
        DeleteServiceError::HttpDispatch(err)
    }
}
impl From<io::Error> for DeleteServiceError {
    fn from(err: io::Error) -> DeleteServiceError {
        DeleteServiceError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DeleteServiceError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteServiceError {
    fn description(&self) -> &str {
        match *self {
            DeleteServiceError::InvalidInput(ref cause) => cause,
            DeleteServiceError::ResourceInUse(ref cause) => cause,
            DeleteServiceError::ServiceNotFound(ref cause) => cause,
            DeleteServiceError::Validation(ref cause) => cause,
            DeleteServiceError::Credentials(ref err) => err.description(),
            DeleteServiceError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            DeleteServiceError::ParseError(ref cause) => cause,
            DeleteServiceError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by DeregisterInstance
#[derive(Debug, PartialEq)]
pub enum DeregisterInstanceError {
    /// <p>The operation is already in progress.</p>
    DuplicateRequest(String),
    /// <p>No instance exists with the specified ID, or the instance was recently registered, and information about the instance hasn't propagated yet.</p>
    InstanceNotFound(String),
    /// <p>One or more specified values aren't valid. For example, when you're creating a namespace, the value of <code>Name</code> might not be a valid DNS name.</p>
    InvalidInput(String),
    /// <p>The specified resource can't be deleted because it contains other resources. For example, you can't delete a service that contains any instances.</p>
    ResourceInUse(String),
    /// <p>No service exists with the specified ID.</p>
    ServiceNotFound(String),
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

impl DeregisterInstanceError {
    pub fn from_response(res: BufferedHttpResponse) -> DeregisterInstanceError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "DuplicateRequest" => {
                    return DeregisterInstanceError::DuplicateRequest(String::from(error_message))
                }
                "InstanceNotFound" => {
                    return DeregisterInstanceError::InstanceNotFound(String::from(error_message))
                }
                "InvalidInput" => {
                    return DeregisterInstanceError::InvalidInput(String::from(error_message))
                }
                "ResourceInUse" => {
                    return DeregisterInstanceError::ResourceInUse(String::from(error_message))
                }
                "ServiceNotFound" => {
                    return DeregisterInstanceError::ServiceNotFound(String::from(error_message))
                }
                "ValidationException" => {
                    return DeregisterInstanceError::Validation(error_message.to_string())
                }
                _ => {}
            }
        }
        return DeregisterInstanceError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for DeregisterInstanceError {
    fn from(err: serde_json::error::Error) -> DeregisterInstanceError {
        DeregisterInstanceError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for DeregisterInstanceError {
    fn from(err: CredentialsError) -> DeregisterInstanceError {
        DeregisterInstanceError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DeregisterInstanceError {
    fn from(err: HttpDispatchError) -> DeregisterInstanceError {
        DeregisterInstanceError::HttpDispatch(err)
    }
}
impl From<io::Error> for DeregisterInstanceError {
    fn from(err: io::Error) -> DeregisterInstanceError {
        DeregisterInstanceError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DeregisterInstanceError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeregisterInstanceError {
    fn description(&self) -> &str {
        match *self {
            DeregisterInstanceError::DuplicateRequest(ref cause) => cause,
            DeregisterInstanceError::InstanceNotFound(ref cause) => cause,
            DeregisterInstanceError::InvalidInput(ref cause) => cause,
            DeregisterInstanceError::ResourceInUse(ref cause) => cause,
            DeregisterInstanceError::ServiceNotFound(ref cause) => cause,
            DeregisterInstanceError::Validation(ref cause) => cause,
            DeregisterInstanceError::Credentials(ref err) => err.description(),
            DeregisterInstanceError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DeregisterInstanceError::ParseError(ref cause) => cause,
            DeregisterInstanceError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by GetInstance
#[derive(Debug, PartialEq)]
pub enum GetInstanceError {
    /// <p>No instance exists with the specified ID, or the instance was recently registered, and information about the instance hasn't propagated yet.</p>
    InstanceNotFound(String),
    /// <p>One or more specified values aren't valid. For example, when you're creating a namespace, the value of <code>Name</code> might not be a valid DNS name.</p>
    InvalidInput(String),
    /// <p>No service exists with the specified ID.</p>
    ServiceNotFound(String),
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

impl GetInstanceError {
    pub fn from_response(res: BufferedHttpResponse) -> GetInstanceError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "InstanceNotFound" => {
                    return GetInstanceError::InstanceNotFound(String::from(error_message))
                }
                "InvalidInput" => {
                    return GetInstanceError::InvalidInput(String::from(error_message))
                }
                "ServiceNotFound" => {
                    return GetInstanceError::ServiceNotFound(String::from(error_message))
                }
                "ValidationException" => {
                    return GetInstanceError::Validation(error_message.to_string())
                }
                _ => {}
            }
        }
        return GetInstanceError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for GetInstanceError {
    fn from(err: serde_json::error::Error) -> GetInstanceError {
        GetInstanceError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for GetInstanceError {
    fn from(err: CredentialsError) -> GetInstanceError {
        GetInstanceError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GetInstanceError {
    fn from(err: HttpDispatchError) -> GetInstanceError {
        GetInstanceError::HttpDispatch(err)
    }
}
impl From<io::Error> for GetInstanceError {
    fn from(err: io::Error) -> GetInstanceError {
        GetInstanceError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for GetInstanceError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetInstanceError {
    fn description(&self) -> &str {
        match *self {
            GetInstanceError::InstanceNotFound(ref cause) => cause,
            GetInstanceError::InvalidInput(ref cause) => cause,
            GetInstanceError::ServiceNotFound(ref cause) => cause,
            GetInstanceError::Validation(ref cause) => cause,
            GetInstanceError::Credentials(ref err) => err.description(),
            GetInstanceError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            GetInstanceError::ParseError(ref cause) => cause,
            GetInstanceError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by GetInstancesHealthStatus
#[derive(Debug, PartialEq)]
pub enum GetInstancesHealthStatusError {
    /// <p>No instance exists with the specified ID, or the instance was recently registered, and information about the instance hasn't propagated yet.</p>
    InstanceNotFound(String),
    /// <p>One or more specified values aren't valid. For example, when you're creating a namespace, the value of <code>Name</code> might not be a valid DNS name.</p>
    InvalidInput(String),
    /// <p>No service exists with the specified ID.</p>
    ServiceNotFound(String),
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

impl GetInstancesHealthStatusError {
    pub fn from_response(res: BufferedHttpResponse) -> GetInstancesHealthStatusError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "InstanceNotFound" => {
                    return GetInstancesHealthStatusError::InstanceNotFound(String::from(
                        error_message,
                    ))
                }
                "InvalidInput" => {
                    return GetInstancesHealthStatusError::InvalidInput(String::from(error_message))
                }
                "ServiceNotFound" => {
                    return GetInstancesHealthStatusError::ServiceNotFound(String::from(
                        error_message,
                    ))
                }
                "ValidationException" => {
                    return GetInstancesHealthStatusError::Validation(error_message.to_string())
                }
                _ => {}
            }
        }
        return GetInstancesHealthStatusError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for GetInstancesHealthStatusError {
    fn from(err: serde_json::error::Error) -> GetInstancesHealthStatusError {
        GetInstancesHealthStatusError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for GetInstancesHealthStatusError {
    fn from(err: CredentialsError) -> GetInstancesHealthStatusError {
        GetInstancesHealthStatusError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GetInstancesHealthStatusError {
    fn from(err: HttpDispatchError) -> GetInstancesHealthStatusError {
        GetInstancesHealthStatusError::HttpDispatch(err)
    }
}
impl From<io::Error> for GetInstancesHealthStatusError {
    fn from(err: io::Error) -> GetInstancesHealthStatusError {
        GetInstancesHealthStatusError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for GetInstancesHealthStatusError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetInstancesHealthStatusError {
    fn description(&self) -> &str {
        match *self {
            GetInstancesHealthStatusError::InstanceNotFound(ref cause) => cause,
            GetInstancesHealthStatusError::InvalidInput(ref cause) => cause,
            GetInstancesHealthStatusError::ServiceNotFound(ref cause) => cause,
            GetInstancesHealthStatusError::Validation(ref cause) => cause,
            GetInstancesHealthStatusError::Credentials(ref err) => err.description(),
            GetInstancesHealthStatusError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            GetInstancesHealthStatusError::ParseError(ref cause) => cause,
            GetInstancesHealthStatusError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by GetNamespace
#[derive(Debug, PartialEq)]
pub enum GetNamespaceError {
    /// <p>One or more specified values aren't valid. For example, when you're creating a namespace, the value of <code>Name</code> might not be a valid DNS name.</p>
    InvalidInput(String),
    /// <p>No namespace exists with the specified ID.</p>
    NamespaceNotFound(String),
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

impl GetNamespaceError {
    pub fn from_response(res: BufferedHttpResponse) -> GetNamespaceError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "InvalidInput" => {
                    return GetNamespaceError::InvalidInput(String::from(error_message))
                }
                "NamespaceNotFound" => {
                    return GetNamespaceError::NamespaceNotFound(String::from(error_message))
                }
                "ValidationException" => {
                    return GetNamespaceError::Validation(error_message.to_string())
                }
                _ => {}
            }
        }
        return GetNamespaceError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for GetNamespaceError {
    fn from(err: serde_json::error::Error) -> GetNamespaceError {
        GetNamespaceError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for GetNamespaceError {
    fn from(err: CredentialsError) -> GetNamespaceError {
        GetNamespaceError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GetNamespaceError {
    fn from(err: HttpDispatchError) -> GetNamespaceError {
        GetNamespaceError::HttpDispatch(err)
    }
}
impl From<io::Error> for GetNamespaceError {
    fn from(err: io::Error) -> GetNamespaceError {
        GetNamespaceError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for GetNamespaceError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetNamespaceError {
    fn description(&self) -> &str {
        match *self {
            GetNamespaceError::InvalidInput(ref cause) => cause,
            GetNamespaceError::NamespaceNotFound(ref cause) => cause,
            GetNamespaceError::Validation(ref cause) => cause,
            GetNamespaceError::Credentials(ref err) => err.description(),
            GetNamespaceError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            GetNamespaceError::ParseError(ref cause) => cause,
            GetNamespaceError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by GetOperation
#[derive(Debug, PartialEq)]
pub enum GetOperationError {
    /// <p>No operation exists with the specified ID.</p>
    OperationNotFound(String),
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

impl GetOperationError {
    pub fn from_response(res: BufferedHttpResponse) -> GetOperationError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "OperationNotFound" => {
                    return GetOperationError::OperationNotFound(String::from(error_message))
                }
                "ValidationException" => {
                    return GetOperationError::Validation(error_message.to_string())
                }
                _ => {}
            }
        }
        return GetOperationError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for GetOperationError {
    fn from(err: serde_json::error::Error) -> GetOperationError {
        GetOperationError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for GetOperationError {
    fn from(err: CredentialsError) -> GetOperationError {
        GetOperationError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GetOperationError {
    fn from(err: HttpDispatchError) -> GetOperationError {
        GetOperationError::HttpDispatch(err)
    }
}
impl From<io::Error> for GetOperationError {
    fn from(err: io::Error) -> GetOperationError {
        GetOperationError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for GetOperationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetOperationError {
    fn description(&self) -> &str {
        match *self {
            GetOperationError::OperationNotFound(ref cause) => cause,
            GetOperationError::Validation(ref cause) => cause,
            GetOperationError::Credentials(ref err) => err.description(),
            GetOperationError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            GetOperationError::ParseError(ref cause) => cause,
            GetOperationError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by GetService
#[derive(Debug, PartialEq)]
pub enum GetServiceError {
    /// <p>One or more specified values aren't valid. For example, when you're creating a namespace, the value of <code>Name</code> might not be a valid DNS name.</p>
    InvalidInput(String),
    /// <p>No service exists with the specified ID.</p>
    ServiceNotFound(String),
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

impl GetServiceError {
    pub fn from_response(res: BufferedHttpResponse) -> GetServiceError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "InvalidInput" => return GetServiceError::InvalidInput(String::from(error_message)),
                "ServiceNotFound" => {
                    return GetServiceError::ServiceNotFound(String::from(error_message))
                }
                "ValidationException" => {
                    return GetServiceError::Validation(error_message.to_string())
                }
                _ => {}
            }
        }
        return GetServiceError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for GetServiceError {
    fn from(err: serde_json::error::Error) -> GetServiceError {
        GetServiceError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for GetServiceError {
    fn from(err: CredentialsError) -> GetServiceError {
        GetServiceError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GetServiceError {
    fn from(err: HttpDispatchError) -> GetServiceError {
        GetServiceError::HttpDispatch(err)
    }
}
impl From<io::Error> for GetServiceError {
    fn from(err: io::Error) -> GetServiceError {
        GetServiceError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for GetServiceError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetServiceError {
    fn description(&self) -> &str {
        match *self {
            GetServiceError::InvalidInput(ref cause) => cause,
            GetServiceError::ServiceNotFound(ref cause) => cause,
            GetServiceError::Validation(ref cause) => cause,
            GetServiceError::Credentials(ref err) => err.description(),
            GetServiceError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            GetServiceError::ParseError(ref cause) => cause,
            GetServiceError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by ListInstances
#[derive(Debug, PartialEq)]
pub enum ListInstancesError {
    /// <p>One or more specified values aren't valid. For example, when you're creating a namespace, the value of <code>Name</code> might not be a valid DNS name.</p>
    InvalidInput(String),
    /// <p>No service exists with the specified ID.</p>
    ServiceNotFound(String),
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

impl ListInstancesError {
    pub fn from_response(res: BufferedHttpResponse) -> ListInstancesError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "InvalidInput" => {
                    return ListInstancesError::InvalidInput(String::from(error_message))
                }
                "ServiceNotFound" => {
                    return ListInstancesError::ServiceNotFound(String::from(error_message))
                }
                "ValidationException" => {
                    return ListInstancesError::Validation(error_message.to_string())
                }
                _ => {}
            }
        }
        return ListInstancesError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for ListInstancesError {
    fn from(err: serde_json::error::Error) -> ListInstancesError {
        ListInstancesError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for ListInstancesError {
    fn from(err: CredentialsError) -> ListInstancesError {
        ListInstancesError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ListInstancesError {
    fn from(err: HttpDispatchError) -> ListInstancesError {
        ListInstancesError::HttpDispatch(err)
    }
}
impl From<io::Error> for ListInstancesError {
    fn from(err: io::Error) -> ListInstancesError {
        ListInstancesError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for ListInstancesError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListInstancesError {
    fn description(&self) -> &str {
        match *self {
            ListInstancesError::InvalidInput(ref cause) => cause,
            ListInstancesError::ServiceNotFound(ref cause) => cause,
            ListInstancesError::Validation(ref cause) => cause,
            ListInstancesError::Credentials(ref err) => err.description(),
            ListInstancesError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            ListInstancesError::ParseError(ref cause) => cause,
            ListInstancesError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by ListNamespaces
#[derive(Debug, PartialEq)]
pub enum ListNamespacesError {
    /// <p>One or more specified values aren't valid. For example, when you're creating a namespace, the value of <code>Name</code> might not be a valid DNS name.</p>
    InvalidInput(String),
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

impl ListNamespacesError {
    pub fn from_response(res: BufferedHttpResponse) -> ListNamespacesError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "InvalidInput" => {
                    return ListNamespacesError::InvalidInput(String::from(error_message))
                }
                "ValidationException" => {
                    return ListNamespacesError::Validation(error_message.to_string())
                }
                _ => {}
            }
        }
        return ListNamespacesError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for ListNamespacesError {
    fn from(err: serde_json::error::Error) -> ListNamespacesError {
        ListNamespacesError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for ListNamespacesError {
    fn from(err: CredentialsError) -> ListNamespacesError {
        ListNamespacesError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ListNamespacesError {
    fn from(err: HttpDispatchError) -> ListNamespacesError {
        ListNamespacesError::HttpDispatch(err)
    }
}
impl From<io::Error> for ListNamespacesError {
    fn from(err: io::Error) -> ListNamespacesError {
        ListNamespacesError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for ListNamespacesError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListNamespacesError {
    fn description(&self) -> &str {
        match *self {
            ListNamespacesError::InvalidInput(ref cause) => cause,
            ListNamespacesError::Validation(ref cause) => cause,
            ListNamespacesError::Credentials(ref err) => err.description(),
            ListNamespacesError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            ListNamespacesError::ParseError(ref cause) => cause,
            ListNamespacesError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by ListOperations
#[derive(Debug, PartialEq)]
pub enum ListOperationsError {
    /// <p>One or more specified values aren't valid. For example, when you're creating a namespace, the value of <code>Name</code> might not be a valid DNS name.</p>
    InvalidInput(String),
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

impl ListOperationsError {
    pub fn from_response(res: BufferedHttpResponse) -> ListOperationsError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "InvalidInput" => {
                    return ListOperationsError::InvalidInput(String::from(error_message))
                }
                "ValidationException" => {
                    return ListOperationsError::Validation(error_message.to_string())
                }
                _ => {}
            }
        }
        return ListOperationsError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for ListOperationsError {
    fn from(err: serde_json::error::Error) -> ListOperationsError {
        ListOperationsError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for ListOperationsError {
    fn from(err: CredentialsError) -> ListOperationsError {
        ListOperationsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ListOperationsError {
    fn from(err: HttpDispatchError) -> ListOperationsError {
        ListOperationsError::HttpDispatch(err)
    }
}
impl From<io::Error> for ListOperationsError {
    fn from(err: io::Error) -> ListOperationsError {
        ListOperationsError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for ListOperationsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListOperationsError {
    fn description(&self) -> &str {
        match *self {
            ListOperationsError::InvalidInput(ref cause) => cause,
            ListOperationsError::Validation(ref cause) => cause,
            ListOperationsError::Credentials(ref err) => err.description(),
            ListOperationsError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            ListOperationsError::ParseError(ref cause) => cause,
            ListOperationsError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by ListServices
#[derive(Debug, PartialEq)]
pub enum ListServicesError {
    /// <p>One or more specified values aren't valid. For example, when you're creating a namespace, the value of <code>Name</code> might not be a valid DNS name.</p>
    InvalidInput(String),
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

impl ListServicesError {
    pub fn from_response(res: BufferedHttpResponse) -> ListServicesError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "InvalidInput" => {
                    return ListServicesError::InvalidInput(String::from(error_message))
                }
                "ValidationException" => {
                    return ListServicesError::Validation(error_message.to_string())
                }
                _ => {}
            }
        }
        return ListServicesError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for ListServicesError {
    fn from(err: serde_json::error::Error) -> ListServicesError {
        ListServicesError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for ListServicesError {
    fn from(err: CredentialsError) -> ListServicesError {
        ListServicesError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ListServicesError {
    fn from(err: HttpDispatchError) -> ListServicesError {
        ListServicesError::HttpDispatch(err)
    }
}
impl From<io::Error> for ListServicesError {
    fn from(err: io::Error) -> ListServicesError {
        ListServicesError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for ListServicesError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListServicesError {
    fn description(&self) -> &str {
        match *self {
            ListServicesError::InvalidInput(ref cause) => cause,
            ListServicesError::Validation(ref cause) => cause,
            ListServicesError::Credentials(ref err) => err.description(),
            ListServicesError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            ListServicesError::ParseError(ref cause) => cause,
            ListServicesError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by RegisterInstance
#[derive(Debug, PartialEq)]
pub enum RegisterInstanceError {
    /// <p>The operation is already in progress.</p>
    DuplicateRequest(String),
    /// <p>One or more specified values aren't valid. For example, when you're creating a namespace, the value of <code>Name</code> might not be a valid DNS name.</p>
    InvalidInput(String),
    /// <p>The specified resource can't be deleted because it contains other resources. For example, you can't delete a service that contains any instances.</p>
    ResourceInUse(String),
    /// <p>The resource can't be created because you've reached the limit on the number of resources.</p>
    ResourceLimitExceeded(String),
    /// <p>No service exists with the specified ID.</p>
    ServiceNotFound(String),
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

impl RegisterInstanceError {
    pub fn from_response(res: BufferedHttpResponse) -> RegisterInstanceError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "DuplicateRequest" => {
                    return RegisterInstanceError::DuplicateRequest(String::from(error_message))
                }
                "InvalidInput" => {
                    return RegisterInstanceError::InvalidInput(String::from(error_message))
                }
                "ResourceInUse" => {
                    return RegisterInstanceError::ResourceInUse(String::from(error_message))
                }
                "ResourceLimitExceeded" => {
                    return RegisterInstanceError::ResourceLimitExceeded(String::from(error_message))
                }
                "ServiceNotFound" => {
                    return RegisterInstanceError::ServiceNotFound(String::from(error_message))
                }
                "ValidationException" => {
                    return RegisterInstanceError::Validation(error_message.to_string())
                }
                _ => {}
            }
        }
        return RegisterInstanceError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for RegisterInstanceError {
    fn from(err: serde_json::error::Error) -> RegisterInstanceError {
        RegisterInstanceError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for RegisterInstanceError {
    fn from(err: CredentialsError) -> RegisterInstanceError {
        RegisterInstanceError::Credentials(err)
    }
}
impl From<HttpDispatchError> for RegisterInstanceError {
    fn from(err: HttpDispatchError) -> RegisterInstanceError {
        RegisterInstanceError::HttpDispatch(err)
    }
}
impl From<io::Error> for RegisterInstanceError {
    fn from(err: io::Error) -> RegisterInstanceError {
        RegisterInstanceError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for RegisterInstanceError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for RegisterInstanceError {
    fn description(&self) -> &str {
        match *self {
            RegisterInstanceError::DuplicateRequest(ref cause) => cause,
            RegisterInstanceError::InvalidInput(ref cause) => cause,
            RegisterInstanceError::ResourceInUse(ref cause) => cause,
            RegisterInstanceError::ResourceLimitExceeded(ref cause) => cause,
            RegisterInstanceError::ServiceNotFound(ref cause) => cause,
            RegisterInstanceError::Validation(ref cause) => cause,
            RegisterInstanceError::Credentials(ref err) => err.description(),
            RegisterInstanceError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            RegisterInstanceError::ParseError(ref cause) => cause,
            RegisterInstanceError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by UpdateInstanceCustomHealthStatus
#[derive(Debug, PartialEq)]
pub enum UpdateInstanceCustomHealthStatusError {
    CustomHealthNotFound(String),
    /// <p>No instance exists with the specified ID, or the instance was recently registered, and information about the instance hasn't propagated yet.</p>
    InstanceNotFound(String),
    /// <p>One or more specified values aren't valid. For example, when you're creating a namespace, the value of <code>Name</code> might not be a valid DNS name.</p>
    InvalidInput(String),
    /// <p>No service exists with the specified ID.</p>
    ServiceNotFound(String),
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

impl UpdateInstanceCustomHealthStatusError {
    pub fn from_response(res: BufferedHttpResponse) -> UpdateInstanceCustomHealthStatusError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "CustomHealthNotFound" => {
                    return UpdateInstanceCustomHealthStatusError::CustomHealthNotFound(
                        String::from(error_message),
                    )
                }
                "InstanceNotFound" => {
                    return UpdateInstanceCustomHealthStatusError::InstanceNotFound(String::from(
                        error_message,
                    ))
                }
                "InvalidInput" => {
                    return UpdateInstanceCustomHealthStatusError::InvalidInput(String::from(
                        error_message,
                    ))
                }
                "ServiceNotFound" => {
                    return UpdateInstanceCustomHealthStatusError::ServiceNotFound(String::from(
                        error_message,
                    ))
                }
                "ValidationException" => {
                    return UpdateInstanceCustomHealthStatusError::Validation(
                        error_message.to_string(),
                    )
                }
                _ => {}
            }
        }
        return UpdateInstanceCustomHealthStatusError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for UpdateInstanceCustomHealthStatusError {
    fn from(err: serde_json::error::Error) -> UpdateInstanceCustomHealthStatusError {
        UpdateInstanceCustomHealthStatusError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for UpdateInstanceCustomHealthStatusError {
    fn from(err: CredentialsError) -> UpdateInstanceCustomHealthStatusError {
        UpdateInstanceCustomHealthStatusError::Credentials(err)
    }
}
impl From<HttpDispatchError> for UpdateInstanceCustomHealthStatusError {
    fn from(err: HttpDispatchError) -> UpdateInstanceCustomHealthStatusError {
        UpdateInstanceCustomHealthStatusError::HttpDispatch(err)
    }
}
impl From<io::Error> for UpdateInstanceCustomHealthStatusError {
    fn from(err: io::Error) -> UpdateInstanceCustomHealthStatusError {
        UpdateInstanceCustomHealthStatusError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for UpdateInstanceCustomHealthStatusError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for UpdateInstanceCustomHealthStatusError {
    fn description(&self) -> &str {
        match *self {
            UpdateInstanceCustomHealthStatusError::CustomHealthNotFound(ref cause) => cause,
            UpdateInstanceCustomHealthStatusError::InstanceNotFound(ref cause) => cause,
            UpdateInstanceCustomHealthStatusError::InvalidInput(ref cause) => cause,
            UpdateInstanceCustomHealthStatusError::ServiceNotFound(ref cause) => cause,
            UpdateInstanceCustomHealthStatusError::Validation(ref cause) => cause,
            UpdateInstanceCustomHealthStatusError::Credentials(ref err) => err.description(),
            UpdateInstanceCustomHealthStatusError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            UpdateInstanceCustomHealthStatusError::ParseError(ref cause) => cause,
            UpdateInstanceCustomHealthStatusError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by UpdateService
#[derive(Debug, PartialEq)]
pub enum UpdateServiceError {
    /// <p>The operation is already in progress.</p>
    DuplicateRequest(String),
    /// <p>One or more specified values aren't valid. For example, when you're creating a namespace, the value of <code>Name</code> might not be a valid DNS name.</p>
    InvalidInput(String),
    /// <p>No service exists with the specified ID.</p>
    ServiceNotFound(String),
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

impl UpdateServiceError {
    pub fn from_response(res: BufferedHttpResponse) -> UpdateServiceError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "DuplicateRequest" => {
                    return UpdateServiceError::DuplicateRequest(String::from(error_message))
                }
                "InvalidInput" => {
                    return UpdateServiceError::InvalidInput(String::from(error_message))
                }
                "ServiceNotFound" => {
                    return UpdateServiceError::ServiceNotFound(String::from(error_message))
                }
                "ValidationException" => {
                    return UpdateServiceError::Validation(error_message.to_string())
                }
                _ => {}
            }
        }
        return UpdateServiceError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for UpdateServiceError {
    fn from(err: serde_json::error::Error) -> UpdateServiceError {
        UpdateServiceError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for UpdateServiceError {
    fn from(err: CredentialsError) -> UpdateServiceError {
        UpdateServiceError::Credentials(err)
    }
}
impl From<HttpDispatchError> for UpdateServiceError {
    fn from(err: HttpDispatchError) -> UpdateServiceError {
        UpdateServiceError::HttpDispatch(err)
    }
}
impl From<io::Error> for UpdateServiceError {
    fn from(err: io::Error) -> UpdateServiceError {
        UpdateServiceError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for UpdateServiceError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for UpdateServiceError {
    fn description(&self) -> &str {
        match *self {
            UpdateServiceError::DuplicateRequest(ref cause) => cause,
            UpdateServiceError::InvalidInput(ref cause) => cause,
            UpdateServiceError::ServiceNotFound(ref cause) => cause,
            UpdateServiceError::Validation(ref cause) => cause,
            UpdateServiceError::Credentials(ref err) => err.description(),
            UpdateServiceError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            UpdateServiceError::ParseError(ref cause) => cause,
            UpdateServiceError::Unknown(_) => "unknown error",
        }
    }
}
/// Trait representing the capabilities of the ServiceDiscovery API. ServiceDiscovery clients implement this trait.
pub trait ServiceDiscovery {
    /// <p>Creates a private namespace based on DNS, which will be visible only inside a specified Amazon VPC. The namespace defines your service naming scheme. For example, if you name your namespace <code>example.com</code> and name your service <code>backend</code>, the resulting DNS name for the service will be <code>backend.example.com</code>. For the current limit on the number of namespaces that you can create using the same AWS account, see <a href="http://docs.aws.amazon.com/Route53/latest/DeveloperGuide/DNSLimitations.html#limits-api-entities-autonaming">Limits on Auto Naming</a> in the <i>Route 53 Developer Guide</i>.</p>
    fn create_private_dns_namespace(
        &self,
        input: CreatePrivateDnsNamespaceRequest,
    ) -> RusotoFuture<CreatePrivateDnsNamespaceResponse, CreatePrivateDnsNamespaceError>;

    /// <p>Creates a public namespace based on DNS, which will be visible on the internet. The namespace defines your service naming scheme. For example, if you name your namespace <code>example.com</code> and name your service <code>backend</code>, the resulting DNS name for the service will be <code>backend.example.com</code>. For the current limit on the number of namespaces that you can create using the same AWS account, see <a href="http://docs.aws.amazon.com/Route53/latest/DeveloperGuide/DNSLimitations.html#limits-api-entities-autonaming">Limits on Auto Naming</a> in the <i>Route 53 Developer Guide</i>.</p>
    fn create_public_dns_namespace(
        &self,
        input: CreatePublicDnsNamespaceRequest,
    ) -> RusotoFuture<CreatePublicDnsNamespaceResponse, CreatePublicDnsNamespaceError>;

    /// <p>Creates a service, which defines the configuration for the following entities:</p> <ul> <li> <p>Up to three records (A, AAAA, and SRV) or one CNAME record</p> </li> <li> <p>Optionally, a health check</p> </li> </ul> <p>After you create the service, you can submit a <a>RegisterInstance</a> request, and Amazon Route 53 uses the values in the configuration to create the specified entities.</p> <p>For the current limit on the number of instances that you can register using the same namespace and using the same service, see <a href="http://docs.aws.amazon.com/Route53/latest/DeveloperGuide/DNSLimitations.html#limits-api-entities-autonaming">Limits on Auto Naming</a> in the <i>Route 53 Developer Guide</i>.</p>
    fn create_service(
        &self,
        input: CreateServiceRequest,
    ) -> RusotoFuture<CreateServiceResponse, CreateServiceError>;

    /// <p>Deletes a namespace from the current account. If the namespace still contains one or more services, the request fails.</p>
    fn delete_namespace(
        &self,
        input: DeleteNamespaceRequest,
    ) -> RusotoFuture<DeleteNamespaceResponse, DeleteNamespaceError>;

    /// <p>Deletes a specified service. If the service still contains one or more registered instances, the request fails.</p>
    fn delete_service(
        &self,
        input: DeleteServiceRequest,
    ) -> RusotoFuture<DeleteServiceResponse, DeleteServiceError>;

    /// <p>Deletes the records and the health check, if any, that Amazon Route 53 created for the specified instance.</p>
    fn deregister_instance(
        &self,
        input: DeregisterInstanceRequest,
    ) -> RusotoFuture<DeregisterInstanceResponse, DeregisterInstanceError>;

    /// <p>Gets information about a specified instance.</p>
    fn get_instance(
        &self,
        input: GetInstanceRequest,
    ) -> RusotoFuture<GetInstanceResponse, GetInstanceError>;

    /// <p><p>Gets the current health status (<code>Healthy</code>, <code>Unhealthy</code>, or <code>Unknown</code>) of one or more instances that are associated with a specified service.</p> <note> <p>There is a brief delay between when you register an instance and when the health status for the instance is available. </p> </note></p>
    fn get_instances_health_status(
        &self,
        input: GetInstancesHealthStatusRequest,
    ) -> RusotoFuture<GetInstancesHealthStatusResponse, GetInstancesHealthStatusError>;

    /// <p>Gets information about a namespace.</p>
    fn get_namespace(
        &self,
        input: GetNamespaceRequest,
    ) -> RusotoFuture<GetNamespaceResponse, GetNamespaceError>;

    /// <p><p>Gets information about any operation that returns an operation ID in the response, such as a <code>CreateService</code> request.</p> <note> <p>To get a list of operations that match specified criteria, see <a>ListOperations</a>.</p> </note></p>
    fn get_operation(
        &self,
        input: GetOperationRequest,
    ) -> RusotoFuture<GetOperationResponse, GetOperationError>;

    /// <p>Gets the settings for a specified service.</p>
    fn get_service(
        &self,
        input: GetServiceRequest,
    ) -> RusotoFuture<GetServiceResponse, GetServiceError>;

    /// <p>Lists summary information about the instances that you registered by using a specified service.</p>
    fn list_instances(
        &self,
        input: ListInstancesRequest,
    ) -> RusotoFuture<ListInstancesResponse, ListInstancesError>;

    /// <p>Lists summary information about the namespaces that were created by the current AWS account.</p>
    fn list_namespaces(
        &self,
        input: ListNamespacesRequest,
    ) -> RusotoFuture<ListNamespacesResponse, ListNamespacesError>;

    /// <p>Lists operations that match the criteria that you specify.</p>
    fn list_operations(
        &self,
        input: ListOperationsRequest,
    ) -> RusotoFuture<ListOperationsResponse, ListOperationsError>;

    /// <p>Lists summary information for all the services that are associated with one or more specified namespaces.</p>
    fn list_services(
        &self,
        input: ListServicesRequest,
    ) -> RusotoFuture<ListServicesResponse, ListServicesError>;

    /// <p>Creates or updates one or more records and optionally a health check based on the settings in a specified service. When you submit a <code>RegisterInstance</code> request, Amazon Route 53 does the following:</p> <ul> <li> <p>For each DNS record that you define in the service specified by <code>ServiceId</code>, creates or updates a record in the hosted zone that is associated with the corresponding namespace</p> </li> <li> <p>If the service includes <code>HealthCheckConfig</code>, creates or updates a health check based on the settings in the health check configuration</p> </li> <li> <p>Associates the health check, if any, with each of the records</p> </li> </ul> <important> <p>One <code>RegisterInstance</code> request must complete before you can submit another request and specify the same service ID and instance ID.</p> </important> <p>For more information, see <a>CreateService</a>.</p> <p>When Route 53 receives a DNS query for the specified DNS name, it returns the applicable value:</p> <ul> <li> <p> <b>If the health check is healthy</b>: returns all the records</p> </li> <li> <p> <b>If the health check is unhealthy</b>: returns the applicable value for the last healthy instance</p> </li> <li> <p> <b>If you didn't specify a health check configuration</b>: returns all the records</p> </li> </ul> <p>For the current limit on the number of instances that you can register using the same namespace and using the same service, see <a href="http://docs.aws.amazon.com/Route53/latest/DeveloperGuide/DNSLimitations.html#limits-api-entities-autonaming">Limits on Auto Naming</a> in the <i>Route 53 Developer Guide</i>.</p>
    fn register_instance(
        &self,
        input: RegisterInstanceRequest,
    ) -> RusotoFuture<RegisterInstanceResponse, RegisterInstanceError>;

    fn update_instance_custom_health_status(
        &self,
        input: UpdateInstanceCustomHealthStatusRequest,
    ) -> RusotoFuture<(), UpdateInstanceCustomHealthStatusError>;

    /// <p>Submits a request to perform the following operations:</p> <ul> <li> <p>Add or delete <code>DnsRecords</code> configurations</p> </li> <li> <p>Update the TTL setting for existing <code>DnsRecords</code> configurations</p> </li> <li> <p>Add, update, or delete <code>HealthCheckConfig</code> for a specified service</p> </li> </ul> <p>You must specify all <code>DnsRecords</code> configurations (and, optionally, <code>HealthCheckConfig</code>) that you want to appear in the updated service. Any current configurations that don't appear in an <code>UpdateService</code> request are deleted.</p> <p>When you update the TTL setting for a service, Amazon Route 53 also updates the corresponding settings in all the records and health checks that were created by using the specified service.</p>
    fn update_service(
        &self,
        input: UpdateServiceRequest,
    ) -> RusotoFuture<UpdateServiceResponse, UpdateServiceError>;
}
/// A client for the ServiceDiscovery API.
pub struct ServiceDiscoveryClient {
    client: Client,
    region: region::Region,
}

impl ServiceDiscoveryClient {
    /// Creates a client backed by the default tokio event loop.
    ///
    /// The client will use the default credentials provider and tls client.
    pub fn new(region: region::Region) -> ServiceDiscoveryClient {
        ServiceDiscoveryClient {
            client: Client::shared(),
            region: region,
        }
    }

    pub fn new_with<P, D>(
        request_dispatcher: D,
        credentials_provider: P,
        region: region::Region,
    ) -> ServiceDiscoveryClient
    where
        P: ProvideAwsCredentials + Send + Sync + 'static,
        P::Future: Send,
        D: DispatchSignedRequest + Send + Sync + 'static,
        D::Future: Send,
    {
        ServiceDiscoveryClient {
            client: Client::new_with(credentials_provider, request_dispatcher),
            region: region,
        }
    }
}

impl ServiceDiscovery for ServiceDiscoveryClient {
    /// <p>Creates a private namespace based on DNS, which will be visible only inside a specified Amazon VPC. The namespace defines your service naming scheme. For example, if you name your namespace <code>example.com</code> and name your service <code>backend</code>, the resulting DNS name for the service will be <code>backend.example.com</code>. For the current limit on the number of namespaces that you can create using the same AWS account, see <a href="http://docs.aws.amazon.com/Route53/latest/DeveloperGuide/DNSLimitations.html#limits-api-entities-autonaming">Limits on Auto Naming</a> in the <i>Route 53 Developer Guide</i>.</p>
    fn create_private_dns_namespace(
        &self,
        input: CreatePrivateDnsNamespaceRequest,
    ) -> RusotoFuture<CreatePrivateDnsNamespaceResponse, CreatePrivateDnsNamespaceError> {
        let mut request = SignedRequest::new("POST", "servicediscovery", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "Route53AutoNaming_v20170314.CreatePrivateDnsNamespace",
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

                    serde_json::from_str::<CreatePrivateDnsNamespaceResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(CreatePrivateDnsNamespaceError::from_response(response))
                }))
            }
        })
    }

    /// <p>Creates a public namespace based on DNS, which will be visible on the internet. The namespace defines your service naming scheme. For example, if you name your namespace <code>example.com</code> and name your service <code>backend</code>, the resulting DNS name for the service will be <code>backend.example.com</code>. For the current limit on the number of namespaces that you can create using the same AWS account, see <a href="http://docs.aws.amazon.com/Route53/latest/DeveloperGuide/DNSLimitations.html#limits-api-entities-autonaming">Limits on Auto Naming</a> in the <i>Route 53 Developer Guide</i>.</p>
    fn create_public_dns_namespace(
        &self,
        input: CreatePublicDnsNamespaceRequest,
    ) -> RusotoFuture<CreatePublicDnsNamespaceResponse, CreatePublicDnsNamespaceError> {
        let mut request = SignedRequest::new("POST", "servicediscovery", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "Route53AutoNaming_v20170314.CreatePublicDnsNamespace",
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

                    serde_json::from_str::<CreatePublicDnsNamespaceResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(CreatePublicDnsNamespaceError::from_response(response))
                }))
            }
        })
    }

    /// <p>Creates a service, which defines the configuration for the following entities:</p> <ul> <li> <p>Up to three records (A, AAAA, and SRV) or one CNAME record</p> </li> <li> <p>Optionally, a health check</p> </li> </ul> <p>After you create the service, you can submit a <a>RegisterInstance</a> request, and Amazon Route 53 uses the values in the configuration to create the specified entities.</p> <p>For the current limit on the number of instances that you can register using the same namespace and using the same service, see <a href="http://docs.aws.amazon.com/Route53/latest/DeveloperGuide/DNSLimitations.html#limits-api-entities-autonaming">Limits on Auto Naming</a> in the <i>Route 53 Developer Guide</i>.</p>
    fn create_service(
        &self,
        input: CreateServiceRequest,
    ) -> RusotoFuture<CreateServiceResponse, CreateServiceError> {
        let mut request = SignedRequest::new("POST", "servicediscovery", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "Route53AutoNaming_v20170314.CreateService");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<CreateServiceResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(CreateServiceError::from_response(response))),
                )
            }
        })
    }

    /// <p>Deletes a namespace from the current account. If the namespace still contains one or more services, the request fails.</p>
    fn delete_namespace(
        &self,
        input: DeleteNamespaceRequest,
    ) -> RusotoFuture<DeleteNamespaceResponse, DeleteNamespaceError> {
        let mut request = SignedRequest::new("POST", "servicediscovery", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "Route53AutoNaming_v20170314.DeleteNamespace",
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

                    serde_json::from_str::<DeleteNamespaceResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(DeleteNamespaceError::from_response(response))),
                )
            }
        })
    }

    /// <p>Deletes a specified service. If the service still contains one or more registered instances, the request fails.</p>
    fn delete_service(
        &self,
        input: DeleteServiceRequest,
    ) -> RusotoFuture<DeleteServiceResponse, DeleteServiceError> {
        let mut request = SignedRequest::new("POST", "servicediscovery", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "Route53AutoNaming_v20170314.DeleteService");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<DeleteServiceResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(DeleteServiceError::from_response(response))),
                )
            }
        })
    }

    /// <p>Deletes the records and the health check, if any, that Amazon Route 53 created for the specified instance.</p>
    fn deregister_instance(
        &self,
        input: DeregisterInstanceRequest,
    ) -> RusotoFuture<DeregisterInstanceResponse, DeregisterInstanceError> {
        let mut request = SignedRequest::new("POST", "servicediscovery", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "Route53AutoNaming_v20170314.DeregisterInstance",
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

                    serde_json::from_str::<DeregisterInstanceResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(DeregisterInstanceError::from_response(response))),
                )
            }
        })
    }

    /// <p>Gets information about a specified instance.</p>
    fn get_instance(
        &self,
        input: GetInstanceRequest,
    ) -> RusotoFuture<GetInstanceResponse, GetInstanceError> {
        let mut request = SignedRequest::new("POST", "servicediscovery", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "Route53AutoNaming_v20170314.GetInstance");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<GetInstanceResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(GetInstanceError::from_response(response))),
                )
            }
        })
    }

    /// <p><p>Gets the current health status (<code>Healthy</code>, <code>Unhealthy</code>, or <code>Unknown</code>) of one or more instances that are associated with a specified service.</p> <note> <p>There is a brief delay between when you register an instance and when the health status for the instance is available. </p> </note></p>
    fn get_instances_health_status(
        &self,
        input: GetInstancesHealthStatusRequest,
    ) -> RusotoFuture<GetInstancesHealthStatusResponse, GetInstancesHealthStatusError> {
        let mut request = SignedRequest::new("POST", "servicediscovery", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "Route53AutoNaming_v20170314.GetInstancesHealthStatus",
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

                    serde_json::from_str::<GetInstancesHealthStatusResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(GetInstancesHealthStatusError::from_response(response))
                }))
            }
        })
    }

    /// <p>Gets information about a namespace.</p>
    fn get_namespace(
        &self,
        input: GetNamespaceRequest,
    ) -> RusotoFuture<GetNamespaceResponse, GetNamespaceError> {
        let mut request = SignedRequest::new("POST", "servicediscovery", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "Route53AutoNaming_v20170314.GetNamespace");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<GetNamespaceResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(GetNamespaceError::from_response(response))),
                )
            }
        })
    }

    /// <p><p>Gets information about any operation that returns an operation ID in the response, such as a <code>CreateService</code> request.</p> <note> <p>To get a list of operations that match specified criteria, see <a>ListOperations</a>.</p> </note></p>
    fn get_operation(
        &self,
        input: GetOperationRequest,
    ) -> RusotoFuture<GetOperationResponse, GetOperationError> {
        let mut request = SignedRequest::new("POST", "servicediscovery", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "Route53AutoNaming_v20170314.GetOperation");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<GetOperationResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(GetOperationError::from_response(response))),
                )
            }
        })
    }

    /// <p>Gets the settings for a specified service.</p>
    fn get_service(
        &self,
        input: GetServiceRequest,
    ) -> RusotoFuture<GetServiceResponse, GetServiceError> {
        let mut request = SignedRequest::new("POST", "servicediscovery", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "Route53AutoNaming_v20170314.GetService");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<GetServiceResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(GetServiceError::from_response(response))),
                )
            }
        })
    }

    /// <p>Lists summary information about the instances that you registered by using a specified service.</p>
    fn list_instances(
        &self,
        input: ListInstancesRequest,
    ) -> RusotoFuture<ListInstancesResponse, ListInstancesError> {
        let mut request = SignedRequest::new("POST", "servicediscovery", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "Route53AutoNaming_v20170314.ListInstances");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<ListInstancesResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(ListInstancesError::from_response(response))),
                )
            }
        })
    }

    /// <p>Lists summary information about the namespaces that were created by the current AWS account.</p>
    fn list_namespaces(
        &self,
        input: ListNamespacesRequest,
    ) -> RusotoFuture<ListNamespacesResponse, ListNamespacesError> {
        let mut request = SignedRequest::new("POST", "servicediscovery", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "Route53AutoNaming_v20170314.ListNamespaces");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<ListNamespacesResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(ListNamespacesError::from_response(response))),
                )
            }
        })
    }

    /// <p>Lists operations that match the criteria that you specify.</p>
    fn list_operations(
        &self,
        input: ListOperationsRequest,
    ) -> RusotoFuture<ListOperationsResponse, ListOperationsError> {
        let mut request = SignedRequest::new("POST", "servicediscovery", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "Route53AutoNaming_v20170314.ListOperations");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<ListOperationsResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(ListOperationsError::from_response(response))),
                )
            }
        })
    }

    /// <p>Lists summary information for all the services that are associated with one or more specified namespaces.</p>
    fn list_services(
        &self,
        input: ListServicesRequest,
    ) -> RusotoFuture<ListServicesResponse, ListServicesError> {
        let mut request = SignedRequest::new("POST", "servicediscovery", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "Route53AutoNaming_v20170314.ListServices");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<ListServicesResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(ListServicesError::from_response(response))),
                )
            }
        })
    }

    /// <p>Creates or updates one or more records and optionally a health check based on the settings in a specified service. When you submit a <code>RegisterInstance</code> request, Amazon Route 53 does the following:</p> <ul> <li> <p>For each DNS record that you define in the service specified by <code>ServiceId</code>, creates or updates a record in the hosted zone that is associated with the corresponding namespace</p> </li> <li> <p>If the service includes <code>HealthCheckConfig</code>, creates or updates a health check based on the settings in the health check configuration</p> </li> <li> <p>Associates the health check, if any, with each of the records</p> </li> </ul> <important> <p>One <code>RegisterInstance</code> request must complete before you can submit another request and specify the same service ID and instance ID.</p> </important> <p>For more information, see <a>CreateService</a>.</p> <p>When Route 53 receives a DNS query for the specified DNS name, it returns the applicable value:</p> <ul> <li> <p> <b>If the health check is healthy</b>: returns all the records</p> </li> <li> <p> <b>If the health check is unhealthy</b>: returns the applicable value for the last healthy instance</p> </li> <li> <p> <b>If you didn't specify a health check configuration</b>: returns all the records</p> </li> </ul> <p>For the current limit on the number of instances that you can register using the same namespace and using the same service, see <a href="http://docs.aws.amazon.com/Route53/latest/DeveloperGuide/DNSLimitations.html#limits-api-entities-autonaming">Limits on Auto Naming</a> in the <i>Route 53 Developer Guide</i>.</p>
    fn register_instance(
        &self,
        input: RegisterInstanceRequest,
    ) -> RusotoFuture<RegisterInstanceResponse, RegisterInstanceError> {
        let mut request = SignedRequest::new("POST", "servicediscovery", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "Route53AutoNaming_v20170314.RegisterInstance",
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

                    serde_json::from_str::<RegisterInstanceResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(RegisterInstanceError::from_response(response))),
                )
            }
        })
    }

    fn update_instance_custom_health_status(
        &self,
        input: UpdateInstanceCustomHealthStatusRequest,
    ) -> RusotoFuture<(), UpdateInstanceCustomHealthStatusError> {
        let mut request = SignedRequest::new("POST", "servicediscovery", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "Route53AutoNaming_v20170314.UpdateInstanceCustomHealthStatus",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(future::ok(::std::mem::drop(response)))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(UpdateInstanceCustomHealthStatusError::from_response(
                        response,
                    ))
                }))
            }
        })
    }

    /// <p>Submits a request to perform the following operations:</p> <ul> <li> <p>Add or delete <code>DnsRecords</code> configurations</p> </li> <li> <p>Update the TTL setting for existing <code>DnsRecords</code> configurations</p> </li> <li> <p>Add, update, or delete <code>HealthCheckConfig</code> for a specified service</p> </li> </ul> <p>You must specify all <code>DnsRecords</code> configurations (and, optionally, <code>HealthCheckConfig</code>) that you want to appear in the updated service. Any current configurations that don't appear in an <code>UpdateService</code> request are deleted.</p> <p>When you update the TTL setting for a service, Amazon Route 53 also updates the corresponding settings in all the records and health checks that were created by using the specified service.</p>
    fn update_service(
        &self,
        input: UpdateServiceRequest,
    ) -> RusotoFuture<UpdateServiceResponse, UpdateServiceError> {
        let mut request = SignedRequest::new("POST", "servicediscovery", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "Route53AutoNaming_v20170314.UpdateService");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<UpdateServiceResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(UpdateServiceError::from_response(response))),
                )
            }
        })
    }
}

#[cfg(test)]
mod protocol_tests {}
