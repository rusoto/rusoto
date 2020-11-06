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

impl GlobalAcceleratorClient {
    fn new_signed_request(&self, http_method: &str, request_uri: &str) -> SignedRequest {
        let mut request =
            SignedRequest::new(http_method, "globalaccelerator", &self.region, request_uri);

        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request
    }

    async fn sign_and_dispatch<E>(
        &self,
        request: SignedRequest,
        from_response: fn(BufferedHttpResponse) -> RusotoError<E>,
    ) -> Result<HttpResponse, RusotoError<E>> {
        let mut response = self.client.sign_and_dispatch(request).await?;
        if !response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            return Err(from_response(response));
        }

        Ok(response)
    }
}

use serde_json;
/// <p>An accelerator is a complex type that includes one or more listeners that process inbound connections and then direct traffic to one or more endpoint groups, each of which includes endpoints, such as load balancers.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct Accelerator {
    /// <p>The Amazon Resource Name (ARN) of the accelerator.</p>
    #[serde(rename = "AcceleratorArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accelerator_arn: Option<String>,
    /// <p>The date and time that the accelerator was created.</p>
    #[serde(rename = "CreatedTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_time: Option<f64>,
    /// <p>The Domain Name System (DNS) name that Global Accelerator creates that points to your accelerator's static IP addresses. </p> <p>The naming convention for the DNS name is the following: A lowercase letter a, followed by a 16-bit random hex string, followed by .awsglobalaccelerator.com. For example: a1234567890abcdef.awsglobalaccelerator.com.</p> <p>For more information about the default DNS name, see <a href="https://docs.aws.amazon.com/global-accelerator/latest/dg/about-accelerators.html#about-accelerators.dns-addressing"> Support for DNS Addressing in Global Accelerator</a> in the <i>AWS Global Accelerator Developer Guide</i>.</p>
    #[serde(rename = "DnsName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dns_name: Option<String>,
    /// <p>Indicates whether the accelerator is enabled. The value is true or false. The default value is true. </p> <p>If the value is set to true, the accelerator cannot be deleted. If set to false, accelerator can be deleted.</p>
    #[serde(rename = "Enabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    /// <p>The value for the address type must be IPv4. </p>
    #[serde(rename = "IpAddressType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_address_type: Option<String>,
    /// <p>The static IP addresses that Global Accelerator associates with the accelerator.</p>
    #[serde(rename = "IpSets")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_sets: Option<Vec<IpSet>>,
    /// <p>The date and time that the accelerator was last modified.</p>
    #[serde(rename = "LastModifiedTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_time: Option<f64>,
    /// <p>The name of the accelerator. The name must contain only alphanumeric characters or hyphens (-), and must not begin or end with a hyphen.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>Describes the deployment status of the accelerator.</p>
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

/// <p>Attributes of an accelerator.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct AcceleratorAttributes {
    /// <p>Indicates whether flow logs are enabled. The default value is false. If the value is true, <code>FlowLogsS3Bucket</code> and <code>FlowLogsS3Prefix</code> must be specified.</p> <p>For more information, see <a href="https://docs.aws.amazon.com/global-accelerator/latest/dg/monitoring-global-accelerator.flow-logs.html">Flow Logs</a> in the <i>AWS Global Accelerator Developer Guide</i>.</p>
    #[serde(rename = "FlowLogsEnabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flow_logs_enabled: Option<bool>,
    /// <p>The name of the Amazon S3 bucket for the flow logs. Attribute is required if <code>FlowLogsEnabled</code> is <code>true</code>. The bucket must exist and have a bucket policy that grants AWS Global Accelerator permission to write to the bucket.</p>
    #[serde(rename = "FlowLogsS3Bucket")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flow_logs_s3_bucket: Option<String>,
    /// <p>The prefix for the location in the Amazon S3 bucket for the flow logs. Attribute is required if <code>FlowLogsEnabled</code> is <code>true</code>.</p> <p>If you don’t specify a prefix, the flow logs are stored in the root of the bucket. If you specify slash (/) for the S3 bucket prefix, the log file bucket folder structure will include a double slash (//), like the following:</p> <p>s3-bucket_name//AWSLogs/aws_account_id</p>
    #[serde(rename = "FlowLogsS3Prefix")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flow_logs_s3_prefix: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct AdvertiseByoipCidrRequest {
    /// <p>The address range, in CIDR notation. This must be the exact range that you provisioned. You can't advertise only a portion of the provisioned range.</p>
    #[serde(rename = "Cidr")]
    pub cidr: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct AdvertiseByoipCidrResponse {
    /// <p>Information about the address range.</p>
    #[serde(rename = "ByoipCidr")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub byoip_cidr: Option<ByoipCidr>,
}

/// <p><p>Information about an IP address range that is provisioned for use with your AWS resources through bring your own IP address (BYOIP).</p> <p>The following describes each BYOIP <code>State</code> that your IP address range can be in.</p> <ul> <li> <p> <b>PENDING<em>PROVISIONING</b> — You’ve submitted a request to provision an IP address range but it is not yet provisioned with AWS Global Accelerator.</p> </li> <li> <p> <b>READY</b> — The address range is provisioned with AWS Global Accelerator and can be advertised.</p> </li> <li> <p> <b>PENDING</em>ADVERTISING</b> — You’ve submitted a request for AWS Global Accelerator to advertise an address range but it is not yet being advertised.</p> </li> <li> <p> <b>ADVERTISING</b> — The address range is being advertised by AWS Global Accelerator.</p> </li> <li> <p> <b>PENDING<em>WITHDRAWING</b> — You’ve submitted a request to withdraw an address range from being advertised but it is still being advertised by AWS Global Accelerator.</p> </li> <li> <p> <b>PENDING</em>DEPROVISIONING</b> — You’ve submitted a request to deprovision an address range from AWS Global Accelerator but it is still provisioned.</p> </li> <li> <p> <b>DEPROVISIONED</b> — The address range is deprovisioned from AWS Global Accelerator.</p> </li> <li> <p> <b>FAILED<em>PROVISION </b> — The request to provision the address range from AWS Global Accelerator was not successful. Please make sure that you provide all of the correct information, and try again. If the request fails a second time, contact AWS support.</p> </li> <li> <p> <b>FAILED</em>ADVERTISING</b> — The request for AWS Global Accelerator to advertise the address range was not successful. Please make sure that you provide all of the correct information, and try again. If the request fails a second time, contact AWS support.</p> </li> <li> <p> <b>FAILED<em>WITHDRAW</b> — The request to withdraw the address range from advertising by AWS Global Accelerator was not successful. Please make sure that you provide all of the correct information, and try again. If the request fails a second time, contact AWS support.</p> </li> <li> <p> <b>FAILED</em>DEPROVISION </b> — The request to deprovision the address range from AWS Global Accelerator was not successful. Please make sure that you provide all of the correct information, and try again. If the request fails a second time, contact AWS support.</p> </li> </ul></p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ByoipCidr {
    /// <p>The address range, in CIDR notation.</p>
    #[serde(rename = "Cidr")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cidr: Option<String>,
    /// <p>A history of status changes for an IP address range that you bring to AWS Global Accelerator through bring your own IP address (BYOIP).</p>
    #[serde(rename = "Events")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub events: Option<Vec<ByoipCidrEvent>>,
    /// <p>The state of the address pool.</p>
    #[serde(rename = "State")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
}

/// <p>A complex type that contains a <code>Message</code> and a <code>Timestamp</code> value for changes that you make in the status an IP address range that you bring to AWS Global Accelerator through bring your own IP address (BYOIP).</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ByoipCidrEvent {
    /// <p>A string that contains an <code>Event</code> message describing changes that you make in the status of an IP address range that you bring to AWS Global Accelerator through bring your own IP address (BYOIP).</p>
    #[serde(rename = "Message")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    /// <p>A timestamp when you make a status change for an IP address range that you bring to AWS Global Accelerator through bring your own IP address (BYOIP).</p>
    #[serde(rename = "Timestamp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<f64>,
}

/// <p>Provides authorization for Amazon to bring a specific IP address range to a specific AWS account using bring your own IP addresses (BYOIP). </p> <p>For more information, see <a href="https://docs.aws.amazon.com/global-accelerator/latest/dg/using-byoip.html">Bring Your Own IP Addresses (BYOIP)</a> in the <i>AWS Global Accelerator Developer Guide</i>.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CidrAuthorizationContext {
    /// <p>The plain-text authorization message for the prefix and account.</p>
    #[serde(rename = "Message")]
    pub message: String,
    /// <p>The signed authorization message for the prefix and account.</p>
    #[serde(rename = "Signature")]
    pub signature: String,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateAcceleratorRequest {
    /// <p>Indicates whether an accelerator is enabled. The value is true or false. The default value is true. </p> <p>If the value is set to true, an accelerator cannot be deleted. If set to false, the accelerator can be deleted.</p>
    #[serde(rename = "Enabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    /// <p>A unique, case-sensitive identifier that you provide to ensure the idempotency—that is, the uniqueness—of an accelerator.</p>
    #[serde(rename = "IdempotencyToken")]
    pub idempotency_token: String,
    /// <p>The value for the address type must be IPv4. </p>
    #[serde(rename = "IpAddressType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_address_type: Option<String>,
    /// <p>Optionally, if you've added your own IP address pool to Global Accelerator (BYOIP), you can choose IP addresses from your own pool to use for the accelerator's static IP addresses when you create an accelerator. You can specify one or two addresses, separated by a comma. Do not include the /32 suffix.</p> <p>Only one IP address from each of your IP address ranges can be used for each accelerator. If you specify only one IP address from your IP address range, Global Accelerator assigns a second static IP address for the accelerator from the AWS IP address pool.</p> <p> Note that you can't update IP addresses for an existing accelerator. To change them, you must create a new accelerator with the new addresses.</p> <p>For more information, see <a href="https://docs.aws.amazon.com/global-accelerator/latest/dg/using-byoip.html">Bring Your Own IP Addresses (BYOIP)</a> in the <i>AWS Global Accelerator Developer Guide</i>.</p>
    #[serde(rename = "IpAddresses")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_addresses: Option<Vec<String>>,
    /// <p>The name of an accelerator. The name can have a maximum of 32 characters, must contain only alphanumeric characters or hyphens (-), and must not begin or end with a hyphen.</p>
    #[serde(rename = "Name")]
    pub name: String,
    /// <p>Create tags for an accelerator.</p> <p>For more information, see <a href="https://docs.aws.amazon.com/global-accelerator/latest/dg/tagging-in-global-accelerator.html">Tagging in AWS Global Accelerator</a> in the <i>AWS Global Accelerator Developer Guide</i>.</p>
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateAcceleratorResponse {
    /// <p>The accelerator that is created by specifying a listener and the supported IP address types.</p>
    #[serde(rename = "Accelerator")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accelerator: Option<Accelerator>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateEndpointGroupRequest {
    /// <p>The list of endpoint objects.</p>
    #[serde(rename = "EndpointConfigurations")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoint_configurations: Option<Vec<EndpointConfiguration>>,
    /// <p>The AWS Region where the endpoint group is located. A listener can have only one endpoint group in a specific Region.</p>
    #[serde(rename = "EndpointGroupRegion")]
    pub endpoint_group_region: String,
    /// <p>The time—10 seconds or 30 seconds—between each health check for an endpoint. The default value is 30.</p>
    #[serde(rename = "HealthCheckIntervalSeconds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub health_check_interval_seconds: Option<i64>,
    /// <p>If the protocol is HTTP/S, then this specifies the path that is the destination for health check targets. The default value is slash (/).</p>
    #[serde(rename = "HealthCheckPath")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub health_check_path: Option<String>,
    /// <p>The port that AWS Global Accelerator uses to check the health of endpoints that are part of this endpoint group. The default port is the listener port that this endpoint group is associated with. If listener port is a list of ports, Global Accelerator uses the first port in the list.</p>
    #[serde(rename = "HealthCheckPort")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub health_check_port: Option<i64>,
    /// <p>The protocol that AWS Global Accelerator uses to check the health of endpoints that are part of this endpoint group. The default value is TCP.</p>
    #[serde(rename = "HealthCheckProtocol")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub health_check_protocol: Option<String>,
    /// <p>A unique, case-sensitive identifier that you provide to ensure the idempotency—that is, the uniqueness—of the request.</p>
    #[serde(rename = "IdempotencyToken")]
    pub idempotency_token: String,
    /// <p>The Amazon Resource Name (ARN) of the listener.</p>
    #[serde(rename = "ListenerArn")]
    pub listener_arn: String,
    /// <p>Override specific listener ports used to route traffic to endpoints that are part of this endpoint group. For example, you can create a port override in which the listener receives user traffic on ports 80 and 443, but your accelerator routes that traffic to ports 1080 and 1443, respectively, on the endpoints.</p> <p>For more information, see <a href="https://docs.aws.amazon.com/global-accelerator/latest/dg/about-endpoint-groups-port-override.html"> Port overrides</a> in the <i>AWS Global Accelerator Developer Guide</i>.</p>
    #[serde(rename = "PortOverrides")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub port_overrides: Option<Vec<PortOverride>>,
    /// <p>The number of consecutive health checks required to set the state of a healthy endpoint to unhealthy, or to set an unhealthy endpoint to healthy. The default value is 3.</p>
    #[serde(rename = "ThresholdCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub threshold_count: Option<i64>,
    /// <p>The percentage of traffic to send to an AWS Region. Additional traffic is distributed to other endpoint groups for this listener. </p> <p>Use this action to increase (dial up) or decrease (dial down) traffic to a specific Region. The percentage is applied to the traffic that would otherwise have been routed to the Region based on optimal routing.</p> <p>The default value is 100.</p>
    #[serde(rename = "TrafficDialPercentage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub traffic_dial_percentage: Option<f32>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateEndpointGroupResponse {
    /// <p>The information about the endpoint group that was created.</p>
    #[serde(rename = "EndpointGroup")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoint_group: Option<EndpointGroup>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateListenerRequest {
    /// <p>The Amazon Resource Name (ARN) of your accelerator.</p>
    #[serde(rename = "AcceleratorArn")]
    pub accelerator_arn: String,
    /// <p>Client affinity lets you direct all requests from a user to the same endpoint, if you have stateful applications, regardless of the port and protocol of the client request. Client affinity gives you control over whether to always route each client to the same specific endpoint.</p> <p>AWS Global Accelerator uses a consistent-flow hashing algorithm to choose the optimal endpoint for a connection. If client affinity is <code>NONE</code>, Global Accelerator uses the "five-tuple" (5-tuple) properties—source IP address, source port, destination IP address, destination port, and protocol—to select the hash value, and then chooses the best endpoint. However, with this setting, if someone uses different ports to connect to Global Accelerator, their connections might not be always routed to the same endpoint because the hash value changes. </p> <p>If you want a given client to always be routed to the same endpoint, set client affinity to <code>SOURCE_IP</code> instead. When you use the <code>SOURCE_IP</code> setting, Global Accelerator uses the "two-tuple" (2-tuple) properties— source (client) IP address and destination IP address—to select the hash value.</p> <p>The default value is <code>NONE</code>.</p>
    #[serde(rename = "ClientAffinity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_affinity: Option<String>,
    /// <p>A unique, case-sensitive identifier that you provide to ensure the idempotency—that is, the uniqueness—of the request.</p>
    #[serde(rename = "IdempotencyToken")]
    pub idempotency_token: String,
    /// <p>The list of port ranges to support for connections from clients to your accelerator.</p>
    #[serde(rename = "PortRanges")]
    pub port_ranges: Vec<PortRange>,
    /// <p>The protocol for connections from clients to your accelerator.</p>
    #[serde(rename = "Protocol")]
    pub protocol: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateListenerResponse {
    /// <p>The listener that you've created.</p>
    #[serde(rename = "Listener")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub listener: Option<Listener>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteAcceleratorRequest {
    /// <p>The Amazon Resource Name (ARN) of an accelerator.</p>
    #[serde(rename = "AcceleratorArn")]
    pub accelerator_arn: String,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteEndpointGroupRequest {
    /// <p>The Amazon Resource Name (ARN) of the endpoint group to delete.</p>
    #[serde(rename = "EndpointGroupArn")]
    pub endpoint_group_arn: String,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteListenerRequest {
    /// <p>The Amazon Resource Name (ARN) of the listener.</p>
    #[serde(rename = "ListenerArn")]
    pub listener_arn: String,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeprovisionByoipCidrRequest {
    /// <p>The address range, in CIDR notation. The prefix must be the same prefix that you specified when you provisioned the address range.</p>
    #[serde(rename = "Cidr")]
    pub cidr: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DeprovisionByoipCidrResponse {
    /// <p>Information about the address range.</p>
    #[serde(rename = "ByoipCidr")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub byoip_cidr: Option<ByoipCidr>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeAcceleratorAttributesRequest {
    /// <p>The Amazon Resource Name (ARN) of the accelerator with the attributes that you want to describe.</p>
    #[serde(rename = "AcceleratorArn")]
    pub accelerator_arn: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeAcceleratorAttributesResponse {
    /// <p>The attributes of the accelerator.</p>
    #[serde(rename = "AcceleratorAttributes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accelerator_attributes: Option<AcceleratorAttributes>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeAcceleratorRequest {
    /// <p>The Amazon Resource Name (ARN) of the accelerator to describe.</p>
    #[serde(rename = "AcceleratorArn")]
    pub accelerator_arn: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeAcceleratorResponse {
    /// <p>The description of the accelerator.</p>
    #[serde(rename = "Accelerator")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accelerator: Option<Accelerator>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeEndpointGroupRequest {
    /// <p>The Amazon Resource Name (ARN) of the endpoint group to describe.</p>
    #[serde(rename = "EndpointGroupArn")]
    pub endpoint_group_arn: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeEndpointGroupResponse {
    /// <p>The description of an endpoint group.</p>
    #[serde(rename = "EndpointGroup")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoint_group: Option<EndpointGroup>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeListenerRequest {
    /// <p>The Amazon Resource Name (ARN) of the listener to describe.</p>
    #[serde(rename = "ListenerArn")]
    pub listener_arn: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeListenerResponse {
    /// <p>The description of a listener.</p>
    #[serde(rename = "Listener")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub listener: Option<Listener>,
}

/// <p>A complex type for endpoints. A resource must be valid and active when you add it as an endpoint.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct EndpointConfiguration {
    /// <p>Indicates whether client IP address preservation is enabled for an Application Load Balancer endpoint. The value is true or false. The default value is true for new accelerators. </p> <p>If the value is set to true, the client's IP address is preserved in the <code>X-Forwarded-For</code> request header as traffic travels to applications on the Application Load Balancer endpoint fronted by the accelerator.</p> <p>For more information, see <a href="https://docs.aws.amazon.com/global-accelerator/latest/dg/preserve-client-ip-address.html"> Preserve Client IP Addresses in AWS Global Accelerator</a> in the <i>AWS Global Accelerator Developer Guide</i>.</p>
    #[serde(rename = "ClientIPPreservationEnabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_ip_preservation_enabled: Option<bool>,
    /// <p>An ID for the endpoint. If the endpoint is a Network Load Balancer or Application Load Balancer, this is the Amazon Resource Name (ARN) of the resource. If the endpoint is an Elastic IP address, this is the Elastic IP address allocation ID. For Amazon EC2 instances, this is the EC2 instance ID. A resource must be valid and active when you add it as an endpoint.</p> <p>An Application Load Balancer can be either internal or internet-facing.</p>
    #[serde(rename = "EndpointId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoint_id: Option<String>,
    /// <p>The weight associated with the endpoint. When you add weights to endpoints, you configure AWS Global Accelerator to route traffic based on proportions that you specify. For example, you might specify endpoint weights of 4, 5, 5, and 6 (sum=20). The result is that 4/20 of your traffic, on average, is routed to the first endpoint, 5/20 is routed both to the second and third endpoints, and 6/20 is routed to the last endpoint. For more information, see <a href="https://docs.aws.amazon.com/global-accelerator/latest/dg/about-endpoints-endpoint-weights.html">Endpoint Weights</a> in the <i>AWS Global Accelerator Developer Guide</i>.</p>
    #[serde(rename = "Weight")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub weight: Option<i64>,
}

/// <p>A complex type for an endpoint. Each endpoint group can include one or more endpoints, such as load balancers.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct EndpointDescription {
    /// <p>Indicates whether client IP address preservation is enabled for an Application Load Balancer endpoint. The value is true or false. The default value is true for new accelerators. </p> <p>If the value is set to true, the client's IP address is preserved in the <code>X-Forwarded-For</code> request header as traffic travels to applications on the Application Load Balancer endpoint fronted by the accelerator.</p> <p>For more information, see <a href="https://docs.aws.amazon.com/global-accelerator/latest/dg/introduction-how-it-works-client-ip.html"> Viewing Client IP Addresses in AWS Global Accelerator</a> in the <i>AWS Global Accelerator Developer Guide</i>.</p>
    #[serde(rename = "ClientIPPreservationEnabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_ip_preservation_enabled: Option<bool>,
    /// <p>An ID for the endpoint. If the endpoint is a Network Load Balancer or Application Load Balancer, this is the Amazon Resource Name (ARN) of the resource. If the endpoint is an Elastic IP address, this is the Elastic IP address allocation ID. For EC2 instances, this is the EC2 instance ID. </p> <p>An Application Load Balancer can be either internal or internet-facing.</p>
    #[serde(rename = "EndpointId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoint_id: Option<String>,
    /// <p><p>The reason code associated with why the endpoint is not healthy. If the endpoint state is healthy, a reason code is not provided.</p> <p>If the endpoint state is <b>unhealthy</b>, the reason code can be one of the following values:</p> <ul> <li> <p> <b>Timeout</b>: The health check requests to the endpoint are timing out before returning a status.</p> </li> <li> <p> <b>Failed</b>: The health check failed, for example because the endpoint response was invalid (malformed).</p> </li> </ul> <p>If the endpoint state is <b>initial</b>, the reason code can be one of the following values:</p> <ul> <li> <p> <b>ProvisioningInProgress</b>: The endpoint is in the process of being provisioned.</p> </li> <li> <p> <b>InitialHealthChecking</b>: Global Accelerator is still setting up the minimum number of health checks for the endpoint that are required to determine its health status.</p> </li> </ul></p>
    #[serde(rename = "HealthReason")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub health_reason: Option<String>,
    /// <p>The health status of the endpoint.</p>
    #[serde(rename = "HealthState")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub health_state: Option<String>,
    /// <p>The weight associated with the endpoint. When you add weights to endpoints, you configure AWS Global Accelerator to route traffic based on proportions that you specify. For example, you might specify endpoint weights of 4, 5, 5, and 6 (sum=20). The result is that 4/20 of your traffic, on average, is routed to the first endpoint, 5/20 is routed both to the second and third endpoints, and 6/20 is routed to the last endpoint. For more information, see <a href="https://docs.aws.amazon.com/global-accelerator/latest/dg/about-endpoints-endpoint-weights.html">Endpoint Weights</a> in the <i>AWS Global Accelerator Developer Guide</i>. </p>
    #[serde(rename = "Weight")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub weight: Option<i64>,
}

/// <p>A complex type for the endpoint group. An AWS Region can have only one endpoint group for a specific listener. </p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct EndpointGroup {
    /// <p>The list of endpoint objects.</p>
    #[serde(rename = "EndpointDescriptions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoint_descriptions: Option<Vec<EndpointDescription>>,
    /// <p>The Amazon Resource Name (ARN) of the endpoint group.</p>
    #[serde(rename = "EndpointGroupArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoint_group_arn: Option<String>,
    /// <p>The AWS Region where the endpoint group is located.</p>
    #[serde(rename = "EndpointGroupRegion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoint_group_region: Option<String>,
    /// <p>The time—10 seconds or 30 seconds—between health checks for each endpoint. The default value is 30.</p>
    #[serde(rename = "HealthCheckIntervalSeconds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub health_check_interval_seconds: Option<i64>,
    /// <p>If the protocol is HTTP/S, then this value provides the ping path that Global Accelerator uses for the destination on the endpoints for health checks. The default is slash (/).</p>
    #[serde(rename = "HealthCheckPath")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub health_check_path: Option<String>,
    /// <p>The port that Global Accelerator uses to perform health checks on endpoints that are part of this endpoint group. </p> <p>The default port is the port for the listener that this endpoint group is associated with. If the listener port is a list, Global Accelerator uses the first specified port in the list of ports.</p>
    #[serde(rename = "HealthCheckPort")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub health_check_port: Option<i64>,
    /// <p>The protocol that Global Accelerator uses to perform health checks on endpoints that are part of this endpoint group. The default value is TCP.</p>
    #[serde(rename = "HealthCheckProtocol")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub health_check_protocol: Option<String>,
    /// <p>Allows you to override the destination ports used to route traffic to an endpoint. Using a port override lets you to map a list of external destination ports (that your users send traffic to) to a list of internal destination ports that you want an application endpoint to receive traffic on. </p>
    #[serde(rename = "PortOverrides")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub port_overrides: Option<Vec<PortOverride>>,
    /// <p>The number of consecutive health checks required to set the state of a healthy endpoint to unhealthy, or to set an unhealthy endpoint to healthy. The default value is 3.</p>
    #[serde(rename = "ThresholdCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub threshold_count: Option<i64>,
    /// <p>The percentage of traffic to send to an AWS Region. Additional traffic is distributed to other endpoint groups for this listener. </p> <p>Use this action to increase (dial up) or decrease (dial down) traffic to a specific Region. The percentage is applied to the traffic that would otherwise have been routed to the Region based on optimal routing.</p> <p>The default value is 100.</p>
    #[serde(rename = "TrafficDialPercentage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub traffic_dial_percentage: Option<f32>,
}

/// <p>A complex type for the set of IP addresses for an accelerator.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct IpSet {
    /// <p>The array of IP addresses in the IP address set. An IP address set can have a maximum of two IP addresses.</p>
    #[serde(rename = "IpAddresses")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_addresses: Option<Vec<String>>,
    /// <p>The types of IP addresses included in this IP set.</p>
    #[serde(rename = "IpFamily")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_family: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListAcceleratorsRequest {
    /// <p>The number of Global Accelerator objects that you want to return with this call. The default value is 10.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>The token for the next set of results. You receive this token from a previous call.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListAcceleratorsResponse {
    /// <p>The list of accelerators for a customer account.</p>
    #[serde(rename = "Accelerators")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accelerators: Option<Vec<Accelerator>>,
    /// <p>The token for the next set of results. You receive this token from a previous call.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListByoipCidrsRequest {
    /// <p>The maximum number of results to return with a single call. To retrieve the remaining results, make another call with the returned <code>nextToken</code> value.</p>
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
pub struct ListByoipCidrsResponse {
    /// <p>Information about your address ranges.</p>
    #[serde(rename = "ByoipCidrs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub byoip_cidrs: Option<Vec<ByoipCidr>>,
    /// <p>The token for the next page of results.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListEndpointGroupsRequest {
    /// <p>The Amazon Resource Name (ARN) of the listener.</p>
    #[serde(rename = "ListenerArn")]
    pub listener_arn: String,
    /// <p>The number of endpoint group objects that you want to return with this call. The default value is 10.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>The token for the next set of results. You receive this token from a previous call.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListEndpointGroupsResponse {
    /// <p>The list of the endpoint groups associated with a listener.</p>
    #[serde(rename = "EndpointGroups")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoint_groups: Option<Vec<EndpointGroup>>,
    /// <p>The token for the next set of results. You receive this token from a previous call.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListListenersRequest {
    /// <p>The Amazon Resource Name (ARN) of the accelerator for which you want to list listener objects.</p>
    #[serde(rename = "AcceleratorArn")]
    pub accelerator_arn: String,
    /// <p>The number of listener objects that you want to return with this call. The default value is 10.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>The token for the next set of results. You receive this token from a previous call.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListListenersResponse {
    /// <p>The list of listeners for an accelerator.</p>
    #[serde(rename = "Listeners")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub listeners: Option<Vec<Listener>>,
    /// <p>The token for the next set of results. You receive this token from a previous call.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListTagsForResourceRequest {
    /// <p>The Amazon Resource Name (ARN) of the accelerator to list tags for. An ARN uniquely identifies an accelerator.</p>
    #[serde(rename = "ResourceArn")]
    pub resource_arn: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListTagsForResourceResponse {
    /// <p>Root level tag for the Tags parameters.</p>
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

/// <p>A complex type for a listener.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct Listener {
    /// <p>Client affinity lets you direct all requests from a user to the same endpoint, if you have stateful applications, regardless of the port and protocol of the client request. Client affinity gives you control over whether to always route each client to the same specific endpoint.</p> <p>AWS Global Accelerator uses a consistent-flow hashing algorithm to choose the optimal endpoint for a connection. If client affinity is <code>NONE</code>, Global Accelerator uses the "five-tuple" (5-tuple) properties—source IP address, source port, destination IP address, destination port, and protocol—to select the hash value, and then chooses the best endpoint. However, with this setting, if someone uses different ports to connect to Global Accelerator, their connections might not be always routed to the same endpoint because the hash value changes. </p> <p>If you want a given client to always be routed to the same endpoint, set client affinity to <code>SOURCE_IP</code> instead. When you use the <code>SOURCE_IP</code> setting, Global Accelerator uses the "two-tuple" (2-tuple) properties— source (client) IP address and destination IP address—to select the hash value.</p> <p>The default value is <code>NONE</code>.</p>
    #[serde(rename = "ClientAffinity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_affinity: Option<String>,
    /// <p>The Amazon Resource Name (ARN) of the listener.</p>
    #[serde(rename = "ListenerArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub listener_arn: Option<String>,
    /// <p>The list of port ranges for the connections from clients to the accelerator.</p>
    #[serde(rename = "PortRanges")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub port_ranges: Option<Vec<PortRange>>,
    /// <p>The protocol for the connections from clients to the accelerator.</p>
    #[serde(rename = "Protocol")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protocol: Option<String>,
}

/// <p>Override specific listener ports used to route traffic to endpoints that are part of an endpoint group. For example, you can create a port override in which the listener receives user traffic on ports 80 and 443, but your accelerator routes that traffic to ports 1080 and 1443, respectively, on the endpoints.</p> <p>For more information, see <a href="https://docs.aws.amazon.com/global-accelerator/latest/dg/about-endpoint-groups-port-override.html"> Port overrides</a> in the <i>AWS Global Accelerator Developer Guide</i>.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct PortOverride {
    /// <p>The endpoint port that you want a listener port to be mapped to. This is the port on the endpoint, such as the Application Load Balancer or Amazon EC2 instance.</p>
    #[serde(rename = "EndpointPort")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoint_port: Option<i64>,
    /// <p>The listener port that you want to map to a specific endpoint port. This is the port that user traffic arrives to the Global Accelerator on.</p>
    #[serde(rename = "ListenerPort")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub listener_port: Option<i64>,
}

/// <p>A complex type for a range of ports for a listener.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct PortRange {
    /// <p>The first port in the range of ports, inclusive.</p>
    #[serde(rename = "FromPort")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from_port: Option<i64>,
    /// <p>The last port in the range of ports, inclusive.</p>
    #[serde(rename = "ToPort")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to_port: Option<i64>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ProvisionByoipCidrRequest {
    /// <p>The public IPv4 address range, in CIDR notation. The most specific IP prefix that you can specify is /24. The address range cannot overlap with another address range that you've brought to this or another Region.</p>
    #[serde(rename = "Cidr")]
    pub cidr: String,
    /// <p>A signed document that proves that you are authorized to bring the specified IP address range to Amazon using BYOIP. </p>
    #[serde(rename = "CidrAuthorizationContext")]
    pub cidr_authorization_context: CidrAuthorizationContext,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ProvisionByoipCidrResponse {
    /// <p>Information about the address range.</p>
    #[serde(rename = "ByoipCidr")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub byoip_cidr: Option<ByoipCidr>,
}

/// <p>A complex type that contains a <code>Tag</code> key and <code>Tag</code> value.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct Tag {
    /// <p>A string that contains a <code>Tag</code> key.</p>
    #[serde(rename = "Key")]
    pub key: String,
    /// <p>A string that contains a <code>Tag</code> value.</p>
    #[serde(rename = "Value")]
    pub value: String,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct TagResourceRequest {
    /// <p>The Amazon Resource Name (ARN) of the Global Accelerator resource to add tags to. An ARN uniquely identifies a resource.</p>
    #[serde(rename = "ResourceArn")]
    pub resource_arn: String,
    /// <p>The tags to add to a resource. A tag consists of a key and a value that you define.</p>
    #[serde(rename = "Tags")]
    pub tags: Vec<Tag>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct TagResourceResponse {}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UntagResourceRequest {
    /// <p>The Amazon Resource Name (ARN) of the Global Accelerator resource to remove tags from. An ARN uniquely identifies a resource.</p>
    #[serde(rename = "ResourceArn")]
    pub resource_arn: String,
    /// <p>The tag key pairs that you want to remove from the specified resources.</p>
    #[serde(rename = "TagKeys")]
    pub tag_keys: Vec<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UntagResourceResponse {}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UpdateAcceleratorAttributesRequest {
    /// <p>The Amazon Resource Name (ARN) of the accelerator that you want to update.</p>
    #[serde(rename = "AcceleratorArn")]
    pub accelerator_arn: String,
    /// <p>Update whether flow logs are enabled. The default value is false. If the value is true, <code>FlowLogsS3Bucket</code> and <code>FlowLogsS3Prefix</code> must be specified.</p> <p>For more information, see <a href="https://docs.aws.amazon.com/global-accelerator/latest/dg/monitoring-global-accelerator.flow-logs.html">Flow Logs</a> in the <i>AWS Global Accelerator Developer Guide</i>.</p>
    #[serde(rename = "FlowLogsEnabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flow_logs_enabled: Option<bool>,
    /// <p>The name of the Amazon S3 bucket for the flow logs. Attribute is required if <code>FlowLogsEnabled</code> is <code>true</code>. The bucket must exist and have a bucket policy that grants AWS Global Accelerator permission to write to the bucket.</p>
    #[serde(rename = "FlowLogsS3Bucket")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flow_logs_s3_bucket: Option<String>,
    /// <p>Update the prefix for the location in the Amazon S3 bucket for the flow logs. Attribute is required if <code>FlowLogsEnabled</code> is <code>true</code>. </p> <p>If you don’t specify a prefix, the flow logs are stored in the root of the bucket. If you specify slash (/) for the S3 bucket prefix, the log file bucket folder structure will include a double slash (//), like the following:</p> <p>s3-bucket_name//AWSLogs/aws_account_id</p>
    #[serde(rename = "FlowLogsS3Prefix")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flow_logs_s3_prefix: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UpdateAcceleratorAttributesResponse {
    /// <p>Updated attributes for the accelerator.</p>
    #[serde(rename = "AcceleratorAttributes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accelerator_attributes: Option<AcceleratorAttributes>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UpdateAcceleratorRequest {
    /// <p>The Amazon Resource Name (ARN) of the accelerator to update.</p>
    #[serde(rename = "AcceleratorArn")]
    pub accelerator_arn: String,
    /// <p>Indicates whether an accelerator is enabled. The value is true or false. The default value is true. </p> <p>If the value is set to true, the accelerator cannot be deleted. If set to false, the accelerator can be deleted.</p>
    #[serde(rename = "Enabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    /// <p>The value for the address type must be IPv4. </p>
    #[serde(rename = "IpAddressType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_address_type: Option<String>,
    /// <p>The name of the accelerator. The name can have a maximum of 32 characters, must contain only alphanumeric characters or hyphens (-), and must not begin or end with a hyphen.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UpdateAcceleratorResponse {
    /// <p>Information about the updated accelerator.</p>
    #[serde(rename = "Accelerator")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accelerator: Option<Accelerator>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UpdateEndpointGroupRequest {
    /// <p>The list of endpoint objects. A resource must be valid and active when you add it as an endpoint.</p>
    #[serde(rename = "EndpointConfigurations")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoint_configurations: Option<Vec<EndpointConfiguration>>,
    /// <p>The Amazon Resource Name (ARN) of the endpoint group.</p>
    #[serde(rename = "EndpointGroupArn")]
    pub endpoint_group_arn: String,
    /// <p>The time—10 seconds or 30 seconds—between each health check for an endpoint. The default value is 30.</p>
    #[serde(rename = "HealthCheckIntervalSeconds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub health_check_interval_seconds: Option<i64>,
    /// <p>If the protocol is HTTP/S, then this specifies the path that is the destination for health check targets. The default value is slash (/).</p>
    #[serde(rename = "HealthCheckPath")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub health_check_path: Option<String>,
    /// <p>The port that AWS Global Accelerator uses to check the health of endpoints that are part of this endpoint group. The default port is the listener port that this endpoint group is associated with. If the listener port is a list of ports, Global Accelerator uses the first port in the list.</p>
    #[serde(rename = "HealthCheckPort")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub health_check_port: Option<i64>,
    /// <p>The protocol that AWS Global Accelerator uses to check the health of endpoints that are part of this endpoint group. The default value is TCP.</p>
    #[serde(rename = "HealthCheckProtocol")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub health_check_protocol: Option<String>,
    /// <p>Override specific listener ports used to route traffic to endpoints that are part of this endpoint group. For example, you can create a port override in which the listener receives user traffic on ports 80 and 443, but your accelerator routes that traffic to ports 1080 and 1443, respectively, on the endpoints.</p> <p>For more information, see <a href="https://docs.aws.amazon.com/global-accelerator/latest/dg/about-endpoint-groups-port-override.html"> Port overrides</a> in the <i>AWS Global Accelerator Developer Guide</i>.</p>
    #[serde(rename = "PortOverrides")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub port_overrides: Option<Vec<PortOverride>>,
    /// <p>The number of consecutive health checks required to set the state of a healthy endpoint to unhealthy, or to set an unhealthy endpoint to healthy. The default value is 3.</p>
    #[serde(rename = "ThresholdCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub threshold_count: Option<i64>,
    /// <p>The percentage of traffic to send to an AWS Region. Additional traffic is distributed to other endpoint groups for this listener. </p> <p>Use this action to increase (dial up) or decrease (dial down) traffic to a specific Region. The percentage is applied to the traffic that would otherwise have been routed to the Region based on optimal routing.</p> <p>The default value is 100.</p>
    #[serde(rename = "TrafficDialPercentage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub traffic_dial_percentage: Option<f32>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UpdateEndpointGroupResponse {
    /// <p>The information about the endpoint group that was updated.</p>
    #[serde(rename = "EndpointGroup")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoint_group: Option<EndpointGroup>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UpdateListenerRequest {
    /// <p>Client affinity lets you direct all requests from a user to the same endpoint, if you have stateful applications, regardless of the port and protocol of the client request. Client affinity gives you control over whether to always route each client to the same specific endpoint.</p> <p>AWS Global Accelerator uses a consistent-flow hashing algorithm to choose the optimal endpoint for a connection. If client affinity is <code>NONE</code>, Global Accelerator uses the "five-tuple" (5-tuple) properties—source IP address, source port, destination IP address, destination port, and protocol—to select the hash value, and then chooses the best endpoint. However, with this setting, if someone uses different ports to connect to Global Accelerator, their connections might not be always routed to the same endpoint because the hash value changes. </p> <p>If you want a given client to always be routed to the same endpoint, set client affinity to <code>SOURCE_IP</code> instead. When you use the <code>SOURCE_IP</code> setting, Global Accelerator uses the "two-tuple" (2-tuple) properties— source (client) IP address and destination IP address—to select the hash value.</p> <p>The default value is <code>NONE</code>.</p>
    #[serde(rename = "ClientAffinity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_affinity: Option<String>,
    /// <p>The Amazon Resource Name (ARN) of the listener to update.</p>
    #[serde(rename = "ListenerArn")]
    pub listener_arn: String,
    /// <p>The updated list of port ranges for the connections from clients to the accelerator.</p>
    #[serde(rename = "PortRanges")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub port_ranges: Option<Vec<PortRange>>,
    /// <p>The updated protocol for the connections from clients to the accelerator.</p>
    #[serde(rename = "Protocol")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protocol: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UpdateListenerResponse {
    /// <p>Information for the updated listener.</p>
    #[serde(rename = "Listener")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub listener: Option<Listener>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct WithdrawByoipCidrRequest {
    /// <p>The address range, in CIDR notation.</p>
    #[serde(rename = "Cidr")]
    pub cidr: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct WithdrawByoipCidrResponse {
    /// <p>Information about the address pool.</p>
    #[serde(rename = "ByoipCidr")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub byoip_cidr: Option<ByoipCidr>,
}

/// Errors returned by AdvertiseByoipCidr
#[derive(Debug, PartialEq)]
pub enum AdvertiseByoipCidrError {
    /// <p>You don't have access permission.</p>
    AccessDenied(String),
    /// <p>The CIDR that you specified was not found or is incorrect.</p>
    ByoipCidrNotFound(String),
    /// <p>The CIDR that you specified is not valid for this action. For example, the state of the CIDR might be incorrect for this action.</p>
    IncorrectCidrState(String),
    /// <p>There was an internal error for AWS Global Accelerator.</p>
    InternalServiceError(String),
    /// <p>An argument that you specified is invalid.</p>
    InvalidArgument(String),
}

impl AdvertiseByoipCidrError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<AdvertiseByoipCidrError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(AdvertiseByoipCidrError::AccessDenied(err.msg))
                }
                "ByoipCidrNotFoundException" => {
                    return RusotoError::Service(AdvertiseByoipCidrError::ByoipCidrNotFound(
                        err.msg,
                    ))
                }
                "IncorrectCidrStateException" => {
                    return RusotoError::Service(AdvertiseByoipCidrError::IncorrectCidrState(
                        err.msg,
                    ))
                }
                "InternalServiceErrorException" => {
                    return RusotoError::Service(AdvertiseByoipCidrError::InternalServiceError(
                        err.msg,
                    ))
                }
                "InvalidArgumentException" => {
                    return RusotoError::Service(AdvertiseByoipCidrError::InvalidArgument(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for AdvertiseByoipCidrError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            AdvertiseByoipCidrError::AccessDenied(ref cause) => write!(f, "{}", cause),
            AdvertiseByoipCidrError::ByoipCidrNotFound(ref cause) => write!(f, "{}", cause),
            AdvertiseByoipCidrError::IncorrectCidrState(ref cause) => write!(f, "{}", cause),
            AdvertiseByoipCidrError::InternalServiceError(ref cause) => write!(f, "{}", cause),
            AdvertiseByoipCidrError::InvalidArgument(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for AdvertiseByoipCidrError {}
/// Errors returned by CreateAccelerator
#[derive(Debug, PartialEq)]
pub enum CreateAcceleratorError {
    /// <p>There was an internal error for AWS Global Accelerator.</p>
    InternalServiceError(String),
    /// <p>An argument that you specified is invalid.</p>
    InvalidArgument(String),
    /// <p>Processing your request would cause you to exceed an AWS Global Accelerator limit.</p>
    LimitExceeded(String),
}

impl CreateAcceleratorError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateAcceleratorError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServiceErrorException" => {
                    return RusotoError::Service(CreateAcceleratorError::InternalServiceError(
                        err.msg,
                    ))
                }
                "InvalidArgumentException" => {
                    return RusotoError::Service(CreateAcceleratorError::InvalidArgument(err.msg))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(CreateAcceleratorError::LimitExceeded(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for CreateAcceleratorError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreateAcceleratorError::InternalServiceError(ref cause) => write!(f, "{}", cause),
            CreateAcceleratorError::InvalidArgument(ref cause) => write!(f, "{}", cause),
            CreateAcceleratorError::LimitExceeded(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CreateAcceleratorError {}
/// Errors returned by CreateEndpointGroup
#[derive(Debug, PartialEq)]
pub enum CreateEndpointGroupError {
    /// <p>The accelerator that you specified doesn't exist.</p>
    AcceleratorNotFound(String),
    /// <p>You don't have access permission.</p>
    AccessDenied(String),
    /// <p>The endpoint group that you specified already exists.</p>
    EndpointGroupAlreadyExists(String),
    /// <p>There was an internal error for AWS Global Accelerator.</p>
    InternalServiceError(String),
    /// <p>An argument that you specified is invalid.</p>
    InvalidArgument(String),
    /// <p>Processing your request would cause you to exceed an AWS Global Accelerator limit.</p>
    LimitExceeded(String),
    /// <p>The listener that you specified doesn't exist.</p>
    ListenerNotFound(String),
}

impl CreateEndpointGroupError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateEndpointGroupError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AcceleratorNotFoundException" => {
                    return RusotoError::Service(CreateEndpointGroupError::AcceleratorNotFound(
                        err.msg,
                    ))
                }
                "AccessDeniedException" => {
                    return RusotoError::Service(CreateEndpointGroupError::AccessDenied(err.msg))
                }
                "EndpointGroupAlreadyExistsException" => {
                    return RusotoError::Service(
                        CreateEndpointGroupError::EndpointGroupAlreadyExists(err.msg),
                    )
                }
                "InternalServiceErrorException" => {
                    return RusotoError::Service(CreateEndpointGroupError::InternalServiceError(
                        err.msg,
                    ))
                }
                "InvalidArgumentException" => {
                    return RusotoError::Service(CreateEndpointGroupError::InvalidArgument(err.msg))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(CreateEndpointGroupError::LimitExceeded(err.msg))
                }
                "ListenerNotFoundException" => {
                    return RusotoError::Service(CreateEndpointGroupError::ListenerNotFound(
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
impl fmt::Display for CreateEndpointGroupError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreateEndpointGroupError::AcceleratorNotFound(ref cause) => write!(f, "{}", cause),
            CreateEndpointGroupError::AccessDenied(ref cause) => write!(f, "{}", cause),
            CreateEndpointGroupError::EndpointGroupAlreadyExists(ref cause) => {
                write!(f, "{}", cause)
            }
            CreateEndpointGroupError::InternalServiceError(ref cause) => write!(f, "{}", cause),
            CreateEndpointGroupError::InvalidArgument(ref cause) => write!(f, "{}", cause),
            CreateEndpointGroupError::LimitExceeded(ref cause) => write!(f, "{}", cause),
            CreateEndpointGroupError::ListenerNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CreateEndpointGroupError {}
/// Errors returned by CreateListener
#[derive(Debug, PartialEq)]
pub enum CreateListenerError {
    /// <p>The accelerator that you specified doesn't exist.</p>
    AcceleratorNotFound(String),
    /// <p>There was an internal error for AWS Global Accelerator.</p>
    InternalServiceError(String),
    /// <p>An argument that you specified is invalid.</p>
    InvalidArgument(String),
    /// <p>The port numbers that you specified are not valid numbers or are not unique for this accelerator.</p>
    InvalidPortRange(String),
    /// <p>Processing your request would cause you to exceed an AWS Global Accelerator limit.</p>
    LimitExceeded(String),
}

impl CreateListenerError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateListenerError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AcceleratorNotFoundException" => {
                    return RusotoError::Service(CreateListenerError::AcceleratorNotFound(err.msg))
                }
                "InternalServiceErrorException" => {
                    return RusotoError::Service(CreateListenerError::InternalServiceError(err.msg))
                }
                "InvalidArgumentException" => {
                    return RusotoError::Service(CreateListenerError::InvalidArgument(err.msg))
                }
                "InvalidPortRangeException" => {
                    return RusotoError::Service(CreateListenerError::InvalidPortRange(err.msg))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(CreateListenerError::LimitExceeded(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for CreateListenerError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreateListenerError::AcceleratorNotFound(ref cause) => write!(f, "{}", cause),
            CreateListenerError::InternalServiceError(ref cause) => write!(f, "{}", cause),
            CreateListenerError::InvalidArgument(ref cause) => write!(f, "{}", cause),
            CreateListenerError::InvalidPortRange(ref cause) => write!(f, "{}", cause),
            CreateListenerError::LimitExceeded(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CreateListenerError {}
/// Errors returned by DeleteAccelerator
#[derive(Debug, PartialEq)]
pub enum DeleteAcceleratorError {
    /// <p>The accelerator that you specified could not be disabled.</p>
    AcceleratorNotDisabled(String),
    /// <p>The accelerator that you specified doesn't exist.</p>
    AcceleratorNotFound(String),
    /// <p>The accelerator that you specified has a listener associated with it. You must remove all dependent resources from an accelerator before you can delete it.</p>
    AssociatedListenerFound(String),
    /// <p>There was an internal error for AWS Global Accelerator.</p>
    InternalServiceError(String),
    /// <p>An argument that you specified is invalid.</p>
    InvalidArgument(String),
}

impl DeleteAcceleratorError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteAcceleratorError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AcceleratorNotDisabledException" => {
                    return RusotoError::Service(DeleteAcceleratorError::AcceleratorNotDisabled(
                        err.msg,
                    ))
                }
                "AcceleratorNotFoundException" => {
                    return RusotoError::Service(DeleteAcceleratorError::AcceleratorNotFound(
                        err.msg,
                    ))
                }
                "AssociatedListenerFoundException" => {
                    return RusotoError::Service(DeleteAcceleratorError::AssociatedListenerFound(
                        err.msg,
                    ))
                }
                "InternalServiceErrorException" => {
                    return RusotoError::Service(DeleteAcceleratorError::InternalServiceError(
                        err.msg,
                    ))
                }
                "InvalidArgumentException" => {
                    return RusotoError::Service(DeleteAcceleratorError::InvalidArgument(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DeleteAcceleratorError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteAcceleratorError::AcceleratorNotDisabled(ref cause) => write!(f, "{}", cause),
            DeleteAcceleratorError::AcceleratorNotFound(ref cause) => write!(f, "{}", cause),
            DeleteAcceleratorError::AssociatedListenerFound(ref cause) => write!(f, "{}", cause),
            DeleteAcceleratorError::InternalServiceError(ref cause) => write!(f, "{}", cause),
            DeleteAcceleratorError::InvalidArgument(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeleteAcceleratorError {}
/// Errors returned by DeleteEndpointGroup
#[derive(Debug, PartialEq)]
pub enum DeleteEndpointGroupError {
    /// <p>The endpoint group that you specified doesn't exist.</p>
    EndpointGroupNotFound(String),
    /// <p>There was an internal error for AWS Global Accelerator.</p>
    InternalServiceError(String),
    /// <p>An argument that you specified is invalid.</p>
    InvalidArgument(String),
}

impl DeleteEndpointGroupError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteEndpointGroupError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "EndpointGroupNotFoundException" => {
                    return RusotoError::Service(DeleteEndpointGroupError::EndpointGroupNotFound(
                        err.msg,
                    ))
                }
                "InternalServiceErrorException" => {
                    return RusotoError::Service(DeleteEndpointGroupError::InternalServiceError(
                        err.msg,
                    ))
                }
                "InvalidArgumentException" => {
                    return RusotoError::Service(DeleteEndpointGroupError::InvalidArgument(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DeleteEndpointGroupError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteEndpointGroupError::EndpointGroupNotFound(ref cause) => write!(f, "{}", cause),
            DeleteEndpointGroupError::InternalServiceError(ref cause) => write!(f, "{}", cause),
            DeleteEndpointGroupError::InvalidArgument(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeleteEndpointGroupError {}
/// Errors returned by DeleteListener
#[derive(Debug, PartialEq)]
pub enum DeleteListenerError {
    /// <p>The listener that you specified has an endpoint group associated with it. You must remove all dependent resources from a listener before you can delete it.</p>
    AssociatedEndpointGroupFound(String),
    /// <p>There was an internal error for AWS Global Accelerator.</p>
    InternalServiceError(String),
    /// <p>An argument that you specified is invalid.</p>
    InvalidArgument(String),
    /// <p>The listener that you specified doesn't exist.</p>
    ListenerNotFound(String),
}

impl DeleteListenerError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteListenerError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AssociatedEndpointGroupFoundException" => {
                    return RusotoError::Service(DeleteListenerError::AssociatedEndpointGroupFound(
                        err.msg,
                    ))
                }
                "InternalServiceErrorException" => {
                    return RusotoError::Service(DeleteListenerError::InternalServiceError(err.msg))
                }
                "InvalidArgumentException" => {
                    return RusotoError::Service(DeleteListenerError::InvalidArgument(err.msg))
                }
                "ListenerNotFoundException" => {
                    return RusotoError::Service(DeleteListenerError::ListenerNotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DeleteListenerError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteListenerError::AssociatedEndpointGroupFound(ref cause) => write!(f, "{}", cause),
            DeleteListenerError::InternalServiceError(ref cause) => write!(f, "{}", cause),
            DeleteListenerError::InvalidArgument(ref cause) => write!(f, "{}", cause),
            DeleteListenerError::ListenerNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeleteListenerError {}
/// Errors returned by DeprovisionByoipCidr
#[derive(Debug, PartialEq)]
pub enum DeprovisionByoipCidrError {
    /// <p>You don't have access permission.</p>
    AccessDenied(String),
    /// <p>The CIDR that you specified was not found or is incorrect.</p>
    ByoipCidrNotFound(String),
    /// <p>The CIDR that you specified is not valid for this action. For example, the state of the CIDR might be incorrect for this action.</p>
    IncorrectCidrState(String),
    /// <p>There was an internal error for AWS Global Accelerator.</p>
    InternalServiceError(String),
    /// <p>An argument that you specified is invalid.</p>
    InvalidArgument(String),
}

impl DeprovisionByoipCidrError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeprovisionByoipCidrError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(DeprovisionByoipCidrError::AccessDenied(err.msg))
                }
                "ByoipCidrNotFoundException" => {
                    return RusotoError::Service(DeprovisionByoipCidrError::ByoipCidrNotFound(
                        err.msg,
                    ))
                }
                "IncorrectCidrStateException" => {
                    return RusotoError::Service(DeprovisionByoipCidrError::IncorrectCidrState(
                        err.msg,
                    ))
                }
                "InternalServiceErrorException" => {
                    return RusotoError::Service(DeprovisionByoipCidrError::InternalServiceError(
                        err.msg,
                    ))
                }
                "InvalidArgumentException" => {
                    return RusotoError::Service(DeprovisionByoipCidrError::InvalidArgument(
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
impl fmt::Display for DeprovisionByoipCidrError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeprovisionByoipCidrError::AccessDenied(ref cause) => write!(f, "{}", cause),
            DeprovisionByoipCidrError::ByoipCidrNotFound(ref cause) => write!(f, "{}", cause),
            DeprovisionByoipCidrError::IncorrectCidrState(ref cause) => write!(f, "{}", cause),
            DeprovisionByoipCidrError::InternalServiceError(ref cause) => write!(f, "{}", cause),
            DeprovisionByoipCidrError::InvalidArgument(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeprovisionByoipCidrError {}
/// Errors returned by DescribeAccelerator
#[derive(Debug, PartialEq)]
pub enum DescribeAcceleratorError {
    /// <p>The accelerator that you specified doesn't exist.</p>
    AcceleratorNotFound(String),
    /// <p>There was an internal error for AWS Global Accelerator.</p>
    InternalServiceError(String),
    /// <p>An argument that you specified is invalid.</p>
    InvalidArgument(String),
}

impl DescribeAcceleratorError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeAcceleratorError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AcceleratorNotFoundException" => {
                    return RusotoError::Service(DescribeAcceleratorError::AcceleratorNotFound(
                        err.msg,
                    ))
                }
                "InternalServiceErrorException" => {
                    return RusotoError::Service(DescribeAcceleratorError::InternalServiceError(
                        err.msg,
                    ))
                }
                "InvalidArgumentException" => {
                    return RusotoError::Service(DescribeAcceleratorError::InvalidArgument(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DescribeAcceleratorError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeAcceleratorError::AcceleratorNotFound(ref cause) => write!(f, "{}", cause),
            DescribeAcceleratorError::InternalServiceError(ref cause) => write!(f, "{}", cause),
            DescribeAcceleratorError::InvalidArgument(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DescribeAcceleratorError {}
/// Errors returned by DescribeAcceleratorAttributes
#[derive(Debug, PartialEq)]
pub enum DescribeAcceleratorAttributesError {
    /// <p>The accelerator that you specified doesn't exist.</p>
    AcceleratorNotFound(String),
    /// <p>There was an internal error for AWS Global Accelerator.</p>
    InternalServiceError(String),
    /// <p>An argument that you specified is invalid.</p>
    InvalidArgument(String),
}

impl DescribeAcceleratorAttributesError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<DescribeAcceleratorAttributesError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AcceleratorNotFoundException" => {
                    return RusotoError::Service(
                        DescribeAcceleratorAttributesError::AcceleratorNotFound(err.msg),
                    )
                }
                "InternalServiceErrorException" => {
                    return RusotoError::Service(
                        DescribeAcceleratorAttributesError::InternalServiceError(err.msg),
                    )
                }
                "InvalidArgumentException" => {
                    return RusotoError::Service(
                        DescribeAcceleratorAttributesError::InvalidArgument(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DescribeAcceleratorAttributesError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeAcceleratorAttributesError::AcceleratorNotFound(ref cause) => {
                write!(f, "{}", cause)
            }
            DescribeAcceleratorAttributesError::InternalServiceError(ref cause) => {
                write!(f, "{}", cause)
            }
            DescribeAcceleratorAttributesError::InvalidArgument(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for DescribeAcceleratorAttributesError {}
/// Errors returned by DescribeEndpointGroup
#[derive(Debug, PartialEq)]
pub enum DescribeEndpointGroupError {
    /// <p>The endpoint group that you specified doesn't exist.</p>
    EndpointGroupNotFound(String),
    /// <p>There was an internal error for AWS Global Accelerator.</p>
    InternalServiceError(String),
    /// <p>An argument that you specified is invalid.</p>
    InvalidArgument(String),
}

impl DescribeEndpointGroupError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeEndpointGroupError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "EndpointGroupNotFoundException" => {
                    return RusotoError::Service(DescribeEndpointGroupError::EndpointGroupNotFound(
                        err.msg,
                    ))
                }
                "InternalServiceErrorException" => {
                    return RusotoError::Service(DescribeEndpointGroupError::InternalServiceError(
                        err.msg,
                    ))
                }
                "InvalidArgumentException" => {
                    return RusotoError::Service(DescribeEndpointGroupError::InvalidArgument(
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
impl fmt::Display for DescribeEndpointGroupError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeEndpointGroupError::EndpointGroupNotFound(ref cause) => write!(f, "{}", cause),
            DescribeEndpointGroupError::InternalServiceError(ref cause) => write!(f, "{}", cause),
            DescribeEndpointGroupError::InvalidArgument(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DescribeEndpointGroupError {}
/// Errors returned by DescribeListener
#[derive(Debug, PartialEq)]
pub enum DescribeListenerError {
    /// <p>There was an internal error for AWS Global Accelerator.</p>
    InternalServiceError(String),
    /// <p>An argument that you specified is invalid.</p>
    InvalidArgument(String),
    /// <p>The listener that you specified doesn't exist.</p>
    ListenerNotFound(String),
}

impl DescribeListenerError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeListenerError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServiceErrorException" => {
                    return RusotoError::Service(DescribeListenerError::InternalServiceError(
                        err.msg,
                    ))
                }
                "InvalidArgumentException" => {
                    return RusotoError::Service(DescribeListenerError::InvalidArgument(err.msg))
                }
                "ListenerNotFoundException" => {
                    return RusotoError::Service(DescribeListenerError::ListenerNotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DescribeListenerError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeListenerError::InternalServiceError(ref cause) => write!(f, "{}", cause),
            DescribeListenerError::InvalidArgument(ref cause) => write!(f, "{}", cause),
            DescribeListenerError::ListenerNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DescribeListenerError {}
/// Errors returned by ListAccelerators
#[derive(Debug, PartialEq)]
pub enum ListAcceleratorsError {
    /// <p>There was an internal error for AWS Global Accelerator.</p>
    InternalServiceError(String),
    /// <p>An argument that you specified is invalid.</p>
    InvalidArgument(String),
    /// <p>There isn't another item to return.</p>
    InvalidNextToken(String),
}

impl ListAcceleratorsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListAcceleratorsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServiceErrorException" => {
                    return RusotoError::Service(ListAcceleratorsError::InternalServiceError(
                        err.msg,
                    ))
                }
                "InvalidArgumentException" => {
                    return RusotoError::Service(ListAcceleratorsError::InvalidArgument(err.msg))
                }
                "InvalidNextTokenException" => {
                    return RusotoError::Service(ListAcceleratorsError::InvalidNextToken(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ListAcceleratorsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListAcceleratorsError::InternalServiceError(ref cause) => write!(f, "{}", cause),
            ListAcceleratorsError::InvalidArgument(ref cause) => write!(f, "{}", cause),
            ListAcceleratorsError::InvalidNextToken(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListAcceleratorsError {}
/// Errors returned by ListByoipCidrs
#[derive(Debug, PartialEq)]
pub enum ListByoipCidrsError {
    /// <p>You don't have access permission.</p>
    AccessDenied(String),
    /// <p>There was an internal error for AWS Global Accelerator.</p>
    InternalServiceError(String),
    /// <p>An argument that you specified is invalid.</p>
    InvalidArgument(String),
    /// <p>There isn't another item to return.</p>
    InvalidNextToken(String),
}

impl ListByoipCidrsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListByoipCidrsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(ListByoipCidrsError::AccessDenied(err.msg))
                }
                "InternalServiceErrorException" => {
                    return RusotoError::Service(ListByoipCidrsError::InternalServiceError(err.msg))
                }
                "InvalidArgumentException" => {
                    return RusotoError::Service(ListByoipCidrsError::InvalidArgument(err.msg))
                }
                "InvalidNextTokenException" => {
                    return RusotoError::Service(ListByoipCidrsError::InvalidNextToken(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ListByoipCidrsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListByoipCidrsError::AccessDenied(ref cause) => write!(f, "{}", cause),
            ListByoipCidrsError::InternalServiceError(ref cause) => write!(f, "{}", cause),
            ListByoipCidrsError::InvalidArgument(ref cause) => write!(f, "{}", cause),
            ListByoipCidrsError::InvalidNextToken(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListByoipCidrsError {}
/// Errors returned by ListEndpointGroups
#[derive(Debug, PartialEq)]
pub enum ListEndpointGroupsError {
    /// <p>There was an internal error for AWS Global Accelerator.</p>
    InternalServiceError(String),
    /// <p>An argument that you specified is invalid.</p>
    InvalidArgument(String),
    /// <p>There isn't another item to return.</p>
    InvalidNextToken(String),
    /// <p>The listener that you specified doesn't exist.</p>
    ListenerNotFound(String),
}

impl ListEndpointGroupsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListEndpointGroupsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServiceErrorException" => {
                    return RusotoError::Service(ListEndpointGroupsError::InternalServiceError(
                        err.msg,
                    ))
                }
                "InvalidArgumentException" => {
                    return RusotoError::Service(ListEndpointGroupsError::InvalidArgument(err.msg))
                }
                "InvalidNextTokenException" => {
                    return RusotoError::Service(ListEndpointGroupsError::InvalidNextToken(err.msg))
                }
                "ListenerNotFoundException" => {
                    return RusotoError::Service(ListEndpointGroupsError::ListenerNotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ListEndpointGroupsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListEndpointGroupsError::InternalServiceError(ref cause) => write!(f, "{}", cause),
            ListEndpointGroupsError::InvalidArgument(ref cause) => write!(f, "{}", cause),
            ListEndpointGroupsError::InvalidNextToken(ref cause) => write!(f, "{}", cause),
            ListEndpointGroupsError::ListenerNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListEndpointGroupsError {}
/// Errors returned by ListListeners
#[derive(Debug, PartialEq)]
pub enum ListListenersError {
    /// <p>The accelerator that you specified doesn't exist.</p>
    AcceleratorNotFound(String),
    /// <p>There was an internal error for AWS Global Accelerator.</p>
    InternalServiceError(String),
    /// <p>An argument that you specified is invalid.</p>
    InvalidArgument(String),
    /// <p>There isn't another item to return.</p>
    InvalidNextToken(String),
}

impl ListListenersError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListListenersError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AcceleratorNotFoundException" => {
                    return RusotoError::Service(ListListenersError::AcceleratorNotFound(err.msg))
                }
                "InternalServiceErrorException" => {
                    return RusotoError::Service(ListListenersError::InternalServiceError(err.msg))
                }
                "InvalidArgumentException" => {
                    return RusotoError::Service(ListListenersError::InvalidArgument(err.msg))
                }
                "InvalidNextTokenException" => {
                    return RusotoError::Service(ListListenersError::InvalidNextToken(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ListListenersError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListListenersError::AcceleratorNotFound(ref cause) => write!(f, "{}", cause),
            ListListenersError::InternalServiceError(ref cause) => write!(f, "{}", cause),
            ListListenersError::InvalidArgument(ref cause) => write!(f, "{}", cause),
            ListListenersError::InvalidNextToken(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListListenersError {}
/// Errors returned by ListTagsForResource
#[derive(Debug, PartialEq)]
pub enum ListTagsForResourceError {
    /// <p>The accelerator that you specified doesn't exist.</p>
    AcceleratorNotFound(String),
    /// <p>There was an internal error for AWS Global Accelerator.</p>
    InternalServiceError(String),
    /// <p>An argument that you specified is invalid.</p>
    InvalidArgument(String),
}

impl ListTagsForResourceError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListTagsForResourceError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AcceleratorNotFoundException" => {
                    return RusotoError::Service(ListTagsForResourceError::AcceleratorNotFound(
                        err.msg,
                    ))
                }
                "InternalServiceErrorException" => {
                    return RusotoError::Service(ListTagsForResourceError::InternalServiceError(
                        err.msg,
                    ))
                }
                "InvalidArgumentException" => {
                    return RusotoError::Service(ListTagsForResourceError::InvalidArgument(err.msg))
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
            ListTagsForResourceError::AcceleratorNotFound(ref cause) => write!(f, "{}", cause),
            ListTagsForResourceError::InternalServiceError(ref cause) => write!(f, "{}", cause),
            ListTagsForResourceError::InvalidArgument(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListTagsForResourceError {}
/// Errors returned by ProvisionByoipCidr
#[derive(Debug, PartialEq)]
pub enum ProvisionByoipCidrError {
    /// <p>You don't have access permission.</p>
    AccessDenied(String),
    /// <p>The CIDR that you specified is not valid for this action. For example, the state of the CIDR might be incorrect for this action.</p>
    IncorrectCidrState(String),
    /// <p>There was an internal error for AWS Global Accelerator.</p>
    InternalServiceError(String),
    /// <p>An argument that you specified is invalid.</p>
    InvalidArgument(String),
    /// <p>Processing your request would cause you to exceed an AWS Global Accelerator limit.</p>
    LimitExceeded(String),
}

impl ProvisionByoipCidrError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ProvisionByoipCidrError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(ProvisionByoipCidrError::AccessDenied(err.msg))
                }
                "IncorrectCidrStateException" => {
                    return RusotoError::Service(ProvisionByoipCidrError::IncorrectCidrState(
                        err.msg,
                    ))
                }
                "InternalServiceErrorException" => {
                    return RusotoError::Service(ProvisionByoipCidrError::InternalServiceError(
                        err.msg,
                    ))
                }
                "InvalidArgumentException" => {
                    return RusotoError::Service(ProvisionByoipCidrError::InvalidArgument(err.msg))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(ProvisionByoipCidrError::LimitExceeded(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ProvisionByoipCidrError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ProvisionByoipCidrError::AccessDenied(ref cause) => write!(f, "{}", cause),
            ProvisionByoipCidrError::IncorrectCidrState(ref cause) => write!(f, "{}", cause),
            ProvisionByoipCidrError::InternalServiceError(ref cause) => write!(f, "{}", cause),
            ProvisionByoipCidrError::InvalidArgument(ref cause) => write!(f, "{}", cause),
            ProvisionByoipCidrError::LimitExceeded(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ProvisionByoipCidrError {}
/// Errors returned by TagResource
#[derive(Debug, PartialEq)]
pub enum TagResourceError {
    /// <p>The accelerator that you specified doesn't exist.</p>
    AcceleratorNotFound(String),
    /// <p>There was an internal error for AWS Global Accelerator.</p>
    InternalServiceError(String),
    /// <p>An argument that you specified is invalid.</p>
    InvalidArgument(String),
}

impl TagResourceError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<TagResourceError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AcceleratorNotFoundException" => {
                    return RusotoError::Service(TagResourceError::AcceleratorNotFound(err.msg))
                }
                "InternalServiceErrorException" => {
                    return RusotoError::Service(TagResourceError::InternalServiceError(err.msg))
                }
                "InvalidArgumentException" => {
                    return RusotoError::Service(TagResourceError::InvalidArgument(err.msg))
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
            TagResourceError::AcceleratorNotFound(ref cause) => write!(f, "{}", cause),
            TagResourceError::InternalServiceError(ref cause) => write!(f, "{}", cause),
            TagResourceError::InvalidArgument(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for TagResourceError {}
/// Errors returned by UntagResource
#[derive(Debug, PartialEq)]
pub enum UntagResourceError {
    /// <p>The accelerator that you specified doesn't exist.</p>
    AcceleratorNotFound(String),
    /// <p>There was an internal error for AWS Global Accelerator.</p>
    InternalServiceError(String),
    /// <p>An argument that you specified is invalid.</p>
    InvalidArgument(String),
}

impl UntagResourceError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UntagResourceError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AcceleratorNotFoundException" => {
                    return RusotoError::Service(UntagResourceError::AcceleratorNotFound(err.msg))
                }
                "InternalServiceErrorException" => {
                    return RusotoError::Service(UntagResourceError::InternalServiceError(err.msg))
                }
                "InvalidArgumentException" => {
                    return RusotoError::Service(UntagResourceError::InvalidArgument(err.msg))
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
            UntagResourceError::AcceleratorNotFound(ref cause) => write!(f, "{}", cause),
            UntagResourceError::InternalServiceError(ref cause) => write!(f, "{}", cause),
            UntagResourceError::InvalidArgument(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for UntagResourceError {}
/// Errors returned by UpdateAccelerator
#[derive(Debug, PartialEq)]
pub enum UpdateAcceleratorError {
    /// <p>The accelerator that you specified doesn't exist.</p>
    AcceleratorNotFound(String),
    /// <p>There was an internal error for AWS Global Accelerator.</p>
    InternalServiceError(String),
    /// <p>An argument that you specified is invalid.</p>
    InvalidArgument(String),
}

impl UpdateAcceleratorError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdateAcceleratorError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AcceleratorNotFoundException" => {
                    return RusotoError::Service(UpdateAcceleratorError::AcceleratorNotFound(
                        err.msg,
                    ))
                }
                "InternalServiceErrorException" => {
                    return RusotoError::Service(UpdateAcceleratorError::InternalServiceError(
                        err.msg,
                    ))
                }
                "InvalidArgumentException" => {
                    return RusotoError::Service(UpdateAcceleratorError::InvalidArgument(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for UpdateAcceleratorError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UpdateAcceleratorError::AcceleratorNotFound(ref cause) => write!(f, "{}", cause),
            UpdateAcceleratorError::InternalServiceError(ref cause) => write!(f, "{}", cause),
            UpdateAcceleratorError::InvalidArgument(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for UpdateAcceleratorError {}
/// Errors returned by UpdateAcceleratorAttributes
#[derive(Debug, PartialEq)]
pub enum UpdateAcceleratorAttributesError {
    /// <p>The accelerator that you specified doesn't exist.</p>
    AcceleratorNotFound(String),
    /// <p>You don't have access permission.</p>
    AccessDenied(String),
    /// <p>There was an internal error for AWS Global Accelerator.</p>
    InternalServiceError(String),
    /// <p>An argument that you specified is invalid.</p>
    InvalidArgument(String),
}

impl UpdateAcceleratorAttributesError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<UpdateAcceleratorAttributesError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AcceleratorNotFoundException" => {
                    return RusotoError::Service(
                        UpdateAcceleratorAttributesError::AcceleratorNotFound(err.msg),
                    )
                }
                "AccessDeniedException" => {
                    return RusotoError::Service(UpdateAcceleratorAttributesError::AccessDenied(
                        err.msg,
                    ))
                }
                "InternalServiceErrorException" => {
                    return RusotoError::Service(
                        UpdateAcceleratorAttributesError::InternalServiceError(err.msg),
                    )
                }
                "InvalidArgumentException" => {
                    return RusotoError::Service(UpdateAcceleratorAttributesError::InvalidArgument(
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
impl fmt::Display for UpdateAcceleratorAttributesError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UpdateAcceleratorAttributesError::AcceleratorNotFound(ref cause) => {
                write!(f, "{}", cause)
            }
            UpdateAcceleratorAttributesError::AccessDenied(ref cause) => write!(f, "{}", cause),
            UpdateAcceleratorAttributesError::InternalServiceError(ref cause) => {
                write!(f, "{}", cause)
            }
            UpdateAcceleratorAttributesError::InvalidArgument(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for UpdateAcceleratorAttributesError {}
/// Errors returned by UpdateEndpointGroup
#[derive(Debug, PartialEq)]
pub enum UpdateEndpointGroupError {
    /// <p>You don't have access permission.</p>
    AccessDenied(String),
    /// <p>The endpoint group that you specified doesn't exist.</p>
    EndpointGroupNotFound(String),
    /// <p>There was an internal error for AWS Global Accelerator.</p>
    InternalServiceError(String),
    /// <p>An argument that you specified is invalid.</p>
    InvalidArgument(String),
    /// <p>Processing your request would cause you to exceed an AWS Global Accelerator limit.</p>
    LimitExceeded(String),
}

impl UpdateEndpointGroupError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdateEndpointGroupError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(UpdateEndpointGroupError::AccessDenied(err.msg))
                }
                "EndpointGroupNotFoundException" => {
                    return RusotoError::Service(UpdateEndpointGroupError::EndpointGroupNotFound(
                        err.msg,
                    ))
                }
                "InternalServiceErrorException" => {
                    return RusotoError::Service(UpdateEndpointGroupError::InternalServiceError(
                        err.msg,
                    ))
                }
                "InvalidArgumentException" => {
                    return RusotoError::Service(UpdateEndpointGroupError::InvalidArgument(err.msg))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(UpdateEndpointGroupError::LimitExceeded(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for UpdateEndpointGroupError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UpdateEndpointGroupError::AccessDenied(ref cause) => write!(f, "{}", cause),
            UpdateEndpointGroupError::EndpointGroupNotFound(ref cause) => write!(f, "{}", cause),
            UpdateEndpointGroupError::InternalServiceError(ref cause) => write!(f, "{}", cause),
            UpdateEndpointGroupError::InvalidArgument(ref cause) => write!(f, "{}", cause),
            UpdateEndpointGroupError::LimitExceeded(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for UpdateEndpointGroupError {}
/// Errors returned by UpdateListener
#[derive(Debug, PartialEq)]
pub enum UpdateListenerError {
    /// <p>There was an internal error for AWS Global Accelerator.</p>
    InternalServiceError(String),
    /// <p>An argument that you specified is invalid.</p>
    InvalidArgument(String),
    /// <p>The port numbers that you specified are not valid numbers or are not unique for this accelerator.</p>
    InvalidPortRange(String),
    /// <p>Processing your request would cause you to exceed an AWS Global Accelerator limit.</p>
    LimitExceeded(String),
    /// <p>The listener that you specified doesn't exist.</p>
    ListenerNotFound(String),
}

impl UpdateListenerError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdateListenerError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServiceErrorException" => {
                    return RusotoError::Service(UpdateListenerError::InternalServiceError(err.msg))
                }
                "InvalidArgumentException" => {
                    return RusotoError::Service(UpdateListenerError::InvalidArgument(err.msg))
                }
                "InvalidPortRangeException" => {
                    return RusotoError::Service(UpdateListenerError::InvalidPortRange(err.msg))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(UpdateListenerError::LimitExceeded(err.msg))
                }
                "ListenerNotFoundException" => {
                    return RusotoError::Service(UpdateListenerError::ListenerNotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for UpdateListenerError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UpdateListenerError::InternalServiceError(ref cause) => write!(f, "{}", cause),
            UpdateListenerError::InvalidArgument(ref cause) => write!(f, "{}", cause),
            UpdateListenerError::InvalidPortRange(ref cause) => write!(f, "{}", cause),
            UpdateListenerError::LimitExceeded(ref cause) => write!(f, "{}", cause),
            UpdateListenerError::ListenerNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for UpdateListenerError {}
/// Errors returned by WithdrawByoipCidr
#[derive(Debug, PartialEq)]
pub enum WithdrawByoipCidrError {
    /// <p>You don't have access permission.</p>
    AccessDenied(String),
    /// <p>The CIDR that you specified was not found or is incorrect.</p>
    ByoipCidrNotFound(String),
    /// <p>The CIDR that you specified is not valid for this action. For example, the state of the CIDR might be incorrect for this action.</p>
    IncorrectCidrState(String),
    /// <p>There was an internal error for AWS Global Accelerator.</p>
    InternalServiceError(String),
    /// <p>An argument that you specified is invalid.</p>
    InvalidArgument(String),
}

impl WithdrawByoipCidrError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<WithdrawByoipCidrError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(WithdrawByoipCidrError::AccessDenied(err.msg))
                }
                "ByoipCidrNotFoundException" => {
                    return RusotoError::Service(WithdrawByoipCidrError::ByoipCidrNotFound(err.msg))
                }
                "IncorrectCidrStateException" => {
                    return RusotoError::Service(WithdrawByoipCidrError::IncorrectCidrState(
                        err.msg,
                    ))
                }
                "InternalServiceErrorException" => {
                    return RusotoError::Service(WithdrawByoipCidrError::InternalServiceError(
                        err.msg,
                    ))
                }
                "InvalidArgumentException" => {
                    return RusotoError::Service(WithdrawByoipCidrError::InvalidArgument(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for WithdrawByoipCidrError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            WithdrawByoipCidrError::AccessDenied(ref cause) => write!(f, "{}", cause),
            WithdrawByoipCidrError::ByoipCidrNotFound(ref cause) => write!(f, "{}", cause),
            WithdrawByoipCidrError::IncorrectCidrState(ref cause) => write!(f, "{}", cause),
            WithdrawByoipCidrError::InternalServiceError(ref cause) => write!(f, "{}", cause),
            WithdrawByoipCidrError::InvalidArgument(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for WithdrawByoipCidrError {}
/// Trait representing the capabilities of the AWS Global Accelerator API. AWS Global Accelerator clients implement this trait.
#[async_trait]
pub trait GlobalAccelerator {
    /// <p>Advertises an IPv4 address range that is provisioned for use with your AWS resources through bring your own IP addresses (BYOIP). It can take a few minutes before traffic to the specified addresses starts routing to AWS because of propagation delays. To see an AWS CLI example of advertising an address range, scroll down to <b>Example</b>.</p> <p>To stop advertising the BYOIP address range, use <a href="https://docs.aws.amazon.com/global-accelerator/latest/api/WithdrawByoipCidr.html"> WithdrawByoipCidr</a>.</p> <p>For more information, see <a href="https://docs.aws.amazon.com/global-accelerator/latest/dg/using-byoip.html">Bring Your Own IP Addresses (BYOIP)</a> in the <i>AWS Global Accelerator Developer Guide</i>.</p>
    async fn advertise_byoip_cidr(
        &self,
        input: AdvertiseByoipCidrRequest,
    ) -> Result<AdvertiseByoipCidrResponse, RusotoError<AdvertiseByoipCidrError>>;

    /// <p><p>Create an accelerator. An accelerator includes one or more listeners that process inbound connections and direct traffic to one or more endpoint groups, each of which includes endpoints, such as Network Load Balancers. To see an AWS CLI example of creating an accelerator, scroll down to <b>Example</b>.</p> <important> <p>Global Accelerator is a global service that supports endpoints in multiple AWS Regions but you must specify the US West (Oregon) Region to create or update accelerators.</p> </important></p>
    async fn create_accelerator(
        &self,
        input: CreateAcceleratorRequest,
    ) -> Result<CreateAcceleratorResponse, RusotoError<CreateAcceleratorError>>;

    /// <p>Create an endpoint group for the specified listener. An endpoint group is a collection of endpoints in one AWS Region. A resource must be valid and active when you add it as an endpoint.</p> <p>To see an AWS CLI example of creating an endpoint group, scroll down to <b>Example</b>.</p>
    async fn create_endpoint_group(
        &self,
        input: CreateEndpointGroupRequest,
    ) -> Result<CreateEndpointGroupResponse, RusotoError<CreateEndpointGroupError>>;

    /// <p>Create a listener to process inbound connections from clients to an accelerator. Connections arrive to assigned static IP addresses on a port, port range, or list of port ranges that you specify. To see an AWS CLI example of creating a listener, scroll down to <b>Example</b>.</p>
    async fn create_listener(
        &self,
        input: CreateListenerRequest,
    ) -> Result<CreateListenerResponse, RusotoError<CreateListenerError>>;

    /// <p><p>Delete an accelerator. Before you can delete an accelerator, you must disable it and remove all dependent resources (listeners and endpoint groups). To disable the accelerator, update the accelerator to set <code>Enabled</code> to false.</p> <important> <p>When you create an accelerator, by default, Global Accelerator provides you with a set of two static IP addresses. Alternatively, you can bring your own IP address ranges to Global Accelerator and assign IP addresses from those ranges. </p> <p>The IP addresses are assigned to your accelerator for as long as it exists, even if you disable the accelerator and it no longer accepts or routes traffic. However, when you <i>delete</i> an accelerator, you lose the static IP addresses that are assigned to the accelerator, so you can no longer route traffic by using them. As a best practice, ensure that you have permissions in place to avoid inadvertently deleting accelerators. You can use IAM policies with Global Accelerator to limit the users who have permissions to delete an accelerator. For more information, see <a href="https://docs.aws.amazon.com/global-accelerator/latest/dg/auth-and-access-control.html">Authentication and Access Control</a> in the <i>AWS Global Accelerator Developer Guide</i>.</p> </important></p>
    async fn delete_accelerator(
        &self,
        input: DeleteAcceleratorRequest,
    ) -> Result<(), RusotoError<DeleteAcceleratorError>>;

    /// <p>Delete an endpoint group from a listener.</p>
    async fn delete_endpoint_group(
        &self,
        input: DeleteEndpointGroupRequest,
    ) -> Result<(), RusotoError<DeleteEndpointGroupError>>;

    /// <p>Delete a listener from an accelerator.</p>
    async fn delete_listener(
        &self,
        input: DeleteListenerRequest,
    ) -> Result<(), RusotoError<DeleteListenerError>>;

    /// <p>Releases the specified address range that you provisioned to use with your AWS resources through bring your own IP addresses (BYOIP) and deletes the corresponding address pool. To see an AWS CLI example of deprovisioning an address range, scroll down to <b>Example</b>.</p> <p>Before you can release an address range, you must stop advertising it by using <a href="https://docs.aws.amazon.com/global-accelerator/latest/api/WithdrawByoipCidr.html">WithdrawByoipCidr</a> and you must not have any accelerators that are using static IP addresses allocated from its address range. </p> <p>For more information, see <a href="https://docs.aws.amazon.com/global-accelerator/latest/dg/using-byoip.html">Bring Your Own IP Addresses (BYOIP)</a> in the <i>AWS Global Accelerator Developer Guide</i>.</p>
    async fn deprovision_byoip_cidr(
        &self,
        input: DeprovisionByoipCidrRequest,
    ) -> Result<DeprovisionByoipCidrResponse, RusotoError<DeprovisionByoipCidrError>>;

    /// <p>Describe an accelerator. To see an AWS CLI example of describing an accelerator, scroll down to <b>Example</b>.</p>
    async fn describe_accelerator(
        &self,
        input: DescribeAcceleratorRequest,
    ) -> Result<DescribeAcceleratorResponse, RusotoError<DescribeAcceleratorError>>;

    /// <p>Describe the attributes of an accelerator. To see an AWS CLI example of describing the attributes of an accelerator, scroll down to <b>Example</b>.</p>
    async fn describe_accelerator_attributes(
        &self,
        input: DescribeAcceleratorAttributesRequest,
    ) -> Result<
        DescribeAcceleratorAttributesResponse,
        RusotoError<DescribeAcceleratorAttributesError>,
    >;

    /// <p>Describe an endpoint group. To see an AWS CLI example of describing an endpoint group, scroll down to <b>Example</b>.</p>
    async fn describe_endpoint_group(
        &self,
        input: DescribeEndpointGroupRequest,
    ) -> Result<DescribeEndpointGroupResponse, RusotoError<DescribeEndpointGroupError>>;

    /// <p>Describe a listener. To see an AWS CLI example of describing a listener, scroll down to <b>Example</b>.</p>
    async fn describe_listener(
        &self,
        input: DescribeListenerRequest,
    ) -> Result<DescribeListenerResponse, RusotoError<DescribeListenerError>>;

    /// <p>List the accelerators for an AWS account. To see an AWS CLI example of listing the accelerators for an AWS account, scroll down to <b>Example</b>.</p>
    async fn list_accelerators(
        &self,
        input: ListAcceleratorsRequest,
    ) -> Result<ListAcceleratorsResponse, RusotoError<ListAcceleratorsError>>;

    /// <p>Lists the IP address ranges that were specified in calls to <a href="https://docs.aws.amazon.com/global-accelerator/latest/api/ProvisionByoipCidr.html">ProvisionByoipCidr</a>, including the current state and a history of state changes.</p> <p>To see an AWS CLI example of listing BYOIP CIDR addresses, scroll down to <b>Example</b>.</p>
    async fn list_byoip_cidrs(
        &self,
        input: ListByoipCidrsRequest,
    ) -> Result<ListByoipCidrsResponse, RusotoError<ListByoipCidrsError>>;

    /// <p>List the endpoint groups that are associated with a listener. To see an AWS CLI example of listing the endpoint groups for listener, scroll down to <b>Example</b>.</p>
    async fn list_endpoint_groups(
        &self,
        input: ListEndpointGroupsRequest,
    ) -> Result<ListEndpointGroupsResponse, RusotoError<ListEndpointGroupsError>>;

    /// <p>List the listeners for an accelerator. To see an AWS CLI example of listing the listeners for an accelerator, scroll down to <b>Example</b>.</p>
    async fn list_listeners(
        &self,
        input: ListListenersRequest,
    ) -> Result<ListListenersResponse, RusotoError<ListListenersError>>;

    /// <p>List all tags for an accelerator. To see an AWS CLI example of listing tags for an accelerator, scroll down to <b>Example</b>.</p> <p>For more information, see <a href="https://docs.aws.amazon.com/global-accelerator/latest/dg/tagging-in-global-accelerator.html">Tagging in AWS Global Accelerator</a> in the <i>AWS Global Accelerator Developer Guide</i>. </p>
    async fn list_tags_for_resource(
        &self,
        input: ListTagsForResourceRequest,
    ) -> Result<ListTagsForResourceResponse, RusotoError<ListTagsForResourceError>>;

    /// <p>Provisions an IP address range to use with your AWS resources through bring your own IP addresses (BYOIP) and creates a corresponding address pool. After the address range is provisioned, it is ready to be advertised using <a href="https://docs.aws.amazon.com/global-accelerator/latest/api/AdvertiseByoipCidr.html"> AdvertiseByoipCidr</a>.</p> <p>To see an AWS CLI example of provisioning an address range for BYOIP, scroll down to <b>Example</b>.</p> <p>For more information, see <a href="https://docs.aws.amazon.com/global-accelerator/latest/dg/using-byoip.html">Bring Your Own IP Addresses (BYOIP)</a> in the <i>AWS Global Accelerator Developer Guide</i>.</p>
    async fn provision_byoip_cidr(
        &self,
        input: ProvisionByoipCidrRequest,
    ) -> Result<ProvisionByoipCidrResponse, RusotoError<ProvisionByoipCidrError>>;

    /// <p>Add tags to an accelerator resource. To see an AWS CLI example of adding tags to an accelerator, scroll down to <b>Example</b>.</p> <p>For more information, see <a href="https://docs.aws.amazon.com/global-accelerator/latest/dg/tagging-in-global-accelerator.html">Tagging in AWS Global Accelerator</a> in the <i>AWS Global Accelerator Developer Guide</i>. </p>
    async fn tag_resource(
        &self,
        input: TagResourceRequest,
    ) -> Result<TagResourceResponse, RusotoError<TagResourceError>>;

    /// <p>Remove tags from a Global Accelerator resource. When you specify a tag key, the action removes both that key and its associated value. To see an AWS CLI example of removing tags from an accelerator, scroll down to <b>Example</b>. The operation succeeds even if you attempt to remove tags from an accelerator that was already removed.</p> <p>For more information, see <a href="https://docs.aws.amazon.com/global-accelerator/latest/dg/tagging-in-global-accelerator.html">Tagging in AWS Global Accelerator</a> in the <i>AWS Global Accelerator Developer Guide</i>.</p>
    async fn untag_resource(
        &self,
        input: UntagResourceRequest,
    ) -> Result<UntagResourceResponse, RusotoError<UntagResourceError>>;

    /// <p><p>Update an accelerator. To see an AWS CLI example of updating an accelerator, scroll down to <b>Example</b>.</p> <important> <p>Global Accelerator is a global service that supports endpoints in multiple AWS Regions but you must specify the US West (Oregon) Region to create or update accelerators.</p> </important></p>
    async fn update_accelerator(
        &self,
        input: UpdateAcceleratorRequest,
    ) -> Result<UpdateAcceleratorResponse, RusotoError<UpdateAcceleratorError>>;

    /// <p>Update the attributes for an accelerator. To see an AWS CLI example of updating an accelerator to enable flow logs, scroll down to <b>Example</b>.</p>
    async fn update_accelerator_attributes(
        &self,
        input: UpdateAcceleratorAttributesRequest,
    ) -> Result<UpdateAcceleratorAttributesResponse, RusotoError<UpdateAcceleratorAttributesError>>;

    /// <p>Update an endpoint group. A resource must be valid and active when you add it as an endpoint.</p> <p>To see an AWS CLI example of updating an endpoint group, scroll down to <b>Example</b>. </p>
    async fn update_endpoint_group(
        &self,
        input: UpdateEndpointGroupRequest,
    ) -> Result<UpdateEndpointGroupResponse, RusotoError<UpdateEndpointGroupError>>;

    /// <p>Update a listener. To see an AWS CLI example of updating listener, scroll down to <b>Example</b>.</p>
    async fn update_listener(
        &self,
        input: UpdateListenerRequest,
    ) -> Result<UpdateListenerResponse, RusotoError<UpdateListenerError>>;

    /// <p>Stops advertising an address range that is provisioned as an address pool. You can perform this operation at most once every 10 seconds, even if you specify different address ranges each time. To see an AWS CLI example of withdrawing an address range for BYOIP so it will no longer be advertised by AWS, scroll down to <b>Example</b>.</p> <p>It can take a few minutes before traffic to the specified addresses stops routing to AWS because of propagation delays.</p> <p>For more information, see <a href="https://docs.aws.amazon.com/global-accelerator/latest/dg/using-byoip.html">Bring Your Own IP Addresses (BYOIP)</a> in the <i>AWS Global Accelerator Developer Guide</i>.</p>
    async fn withdraw_byoip_cidr(
        &self,
        input: WithdrawByoipCidrRequest,
    ) -> Result<WithdrawByoipCidrResponse, RusotoError<WithdrawByoipCidrError>>;
}
/// A client for the AWS Global Accelerator API.
#[derive(Clone)]
pub struct GlobalAcceleratorClient {
    client: Client,
    region: region::Region,
}

impl GlobalAcceleratorClient {
    /// Creates a client backed by the default tokio event loop.
    ///
    /// The client will use the default credentials provider and tls client.
    pub fn new(region: region::Region) -> GlobalAcceleratorClient {
        GlobalAcceleratorClient {
            client: Client::shared(),
            region,
        }
    }

    pub fn new_with<P, D>(
        request_dispatcher: D,
        credentials_provider: P,
        region: region::Region,
    ) -> GlobalAcceleratorClient
    where
        P: ProvideAwsCredentials + Send + Sync + 'static,
        D: DispatchSignedRequest + Send + Sync + 'static,
    {
        GlobalAcceleratorClient {
            client: Client::new_with(credentials_provider, request_dispatcher),
            region,
        }
    }

    pub fn new_with_client(client: Client, region: region::Region) -> GlobalAcceleratorClient {
        GlobalAcceleratorClient { client, region }
    }
}

#[async_trait]
impl GlobalAccelerator for GlobalAcceleratorClient {
    /// <p>Advertises an IPv4 address range that is provisioned for use with your AWS resources through bring your own IP addresses (BYOIP). It can take a few minutes before traffic to the specified addresses starts routing to AWS because of propagation delays. To see an AWS CLI example of advertising an address range, scroll down to <b>Example</b>.</p> <p>To stop advertising the BYOIP address range, use <a href="https://docs.aws.amazon.com/global-accelerator/latest/api/WithdrawByoipCidr.html"> WithdrawByoipCidr</a>.</p> <p>For more information, see <a href="https://docs.aws.amazon.com/global-accelerator/latest/dg/using-byoip.html">Bring Your Own IP Addresses (BYOIP)</a> in the <i>AWS Global Accelerator Developer Guide</i>.</p>
    async fn advertise_byoip_cidr(
        &self,
        input: AdvertiseByoipCidrRequest,
    ) -> Result<AdvertiseByoipCidrResponse, RusotoError<AdvertiseByoipCidrError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "GlobalAccelerator_V20180706.AdvertiseByoipCidr",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, AdvertiseByoipCidrError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<AdvertiseByoipCidrResponse, _>()
    }

    /// <p><p>Create an accelerator. An accelerator includes one or more listeners that process inbound connections and direct traffic to one or more endpoint groups, each of which includes endpoints, such as Network Load Balancers. To see an AWS CLI example of creating an accelerator, scroll down to <b>Example</b>.</p> <important> <p>Global Accelerator is a global service that supports endpoints in multiple AWS Regions but you must specify the US West (Oregon) Region to create or update accelerators.</p> </important></p>
    async fn create_accelerator(
        &self,
        input: CreateAcceleratorRequest,
    ) -> Result<CreateAcceleratorResponse, RusotoError<CreateAcceleratorError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "GlobalAccelerator_V20180706.CreateAccelerator",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, CreateAcceleratorError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<CreateAcceleratorResponse, _>()
    }

    /// <p>Create an endpoint group for the specified listener. An endpoint group is a collection of endpoints in one AWS Region. A resource must be valid and active when you add it as an endpoint.</p> <p>To see an AWS CLI example of creating an endpoint group, scroll down to <b>Example</b>.</p>
    async fn create_endpoint_group(
        &self,
        input: CreateEndpointGroupRequest,
    ) -> Result<CreateEndpointGroupResponse, RusotoError<CreateEndpointGroupError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "GlobalAccelerator_V20180706.CreateEndpointGroup",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, CreateEndpointGroupError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<CreateEndpointGroupResponse, _>()
    }

    /// <p>Create a listener to process inbound connections from clients to an accelerator. Connections arrive to assigned static IP addresses on a port, port range, or list of port ranges that you specify. To see an AWS CLI example of creating a listener, scroll down to <b>Example</b>.</p>
    async fn create_listener(
        &self,
        input: CreateListenerRequest,
    ) -> Result<CreateListenerResponse, RusotoError<CreateListenerError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "GlobalAccelerator_V20180706.CreateListener");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, CreateListenerError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<CreateListenerResponse, _>()
    }

    /// <p><p>Delete an accelerator. Before you can delete an accelerator, you must disable it and remove all dependent resources (listeners and endpoint groups). To disable the accelerator, update the accelerator to set <code>Enabled</code> to false.</p> <important> <p>When you create an accelerator, by default, Global Accelerator provides you with a set of two static IP addresses. Alternatively, you can bring your own IP address ranges to Global Accelerator and assign IP addresses from those ranges. </p> <p>The IP addresses are assigned to your accelerator for as long as it exists, even if you disable the accelerator and it no longer accepts or routes traffic. However, when you <i>delete</i> an accelerator, you lose the static IP addresses that are assigned to the accelerator, so you can no longer route traffic by using them. As a best practice, ensure that you have permissions in place to avoid inadvertently deleting accelerators. You can use IAM policies with Global Accelerator to limit the users who have permissions to delete an accelerator. For more information, see <a href="https://docs.aws.amazon.com/global-accelerator/latest/dg/auth-and-access-control.html">Authentication and Access Control</a> in the <i>AWS Global Accelerator Developer Guide</i>.</p> </important></p>
    async fn delete_accelerator(
        &self,
        input: DeleteAcceleratorRequest,
    ) -> Result<(), RusotoError<DeleteAcceleratorError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "GlobalAccelerator_V20180706.DeleteAccelerator",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, DeleteAcceleratorError::from_response)
            .await?;
        std::mem::drop(response);
        Ok(())
    }

    /// <p>Delete an endpoint group from a listener.</p>
    async fn delete_endpoint_group(
        &self,
        input: DeleteEndpointGroupRequest,
    ) -> Result<(), RusotoError<DeleteEndpointGroupError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "GlobalAccelerator_V20180706.DeleteEndpointGroup",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, DeleteEndpointGroupError::from_response)
            .await?;
        std::mem::drop(response);
        Ok(())
    }

    /// <p>Delete a listener from an accelerator.</p>
    async fn delete_listener(
        &self,
        input: DeleteListenerRequest,
    ) -> Result<(), RusotoError<DeleteListenerError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "GlobalAccelerator_V20180706.DeleteListener");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, DeleteListenerError::from_response)
            .await?;
        std::mem::drop(response);
        Ok(())
    }

    /// <p>Releases the specified address range that you provisioned to use with your AWS resources through bring your own IP addresses (BYOIP) and deletes the corresponding address pool. To see an AWS CLI example of deprovisioning an address range, scroll down to <b>Example</b>.</p> <p>Before you can release an address range, you must stop advertising it by using <a href="https://docs.aws.amazon.com/global-accelerator/latest/api/WithdrawByoipCidr.html">WithdrawByoipCidr</a> and you must not have any accelerators that are using static IP addresses allocated from its address range. </p> <p>For more information, see <a href="https://docs.aws.amazon.com/global-accelerator/latest/dg/using-byoip.html">Bring Your Own IP Addresses (BYOIP)</a> in the <i>AWS Global Accelerator Developer Guide</i>.</p>
    async fn deprovision_byoip_cidr(
        &self,
        input: DeprovisionByoipCidrRequest,
    ) -> Result<DeprovisionByoipCidrResponse, RusotoError<DeprovisionByoipCidrError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "GlobalAccelerator_V20180706.DeprovisionByoipCidr",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, DeprovisionByoipCidrError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response)
            .deserialize::<DeprovisionByoipCidrResponse, _>()
    }

    /// <p>Describe an accelerator. To see an AWS CLI example of describing an accelerator, scroll down to <b>Example</b>.</p>
    async fn describe_accelerator(
        &self,
        input: DescribeAcceleratorRequest,
    ) -> Result<DescribeAcceleratorResponse, RusotoError<DescribeAcceleratorError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "GlobalAccelerator_V20180706.DescribeAccelerator",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, DescribeAcceleratorError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<DescribeAcceleratorResponse, _>()
    }

    /// <p>Describe the attributes of an accelerator. To see an AWS CLI example of describing the attributes of an accelerator, scroll down to <b>Example</b>.</p>
    async fn describe_accelerator_attributes(
        &self,
        input: DescribeAcceleratorAttributesRequest,
    ) -> Result<
        DescribeAcceleratorAttributesResponse,
        RusotoError<DescribeAcceleratorAttributesError>,
    > {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "GlobalAccelerator_V20180706.DescribeAcceleratorAttributes",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, DescribeAcceleratorAttributesError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response)
            .deserialize::<DescribeAcceleratorAttributesResponse, _>()
    }

    /// <p>Describe an endpoint group. To see an AWS CLI example of describing an endpoint group, scroll down to <b>Example</b>.</p>
    async fn describe_endpoint_group(
        &self,
        input: DescribeEndpointGroupRequest,
    ) -> Result<DescribeEndpointGroupResponse, RusotoError<DescribeEndpointGroupError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "GlobalAccelerator_V20180706.DescribeEndpointGroup",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, DescribeEndpointGroupError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response)
            .deserialize::<DescribeEndpointGroupResponse, _>()
    }

    /// <p>Describe a listener. To see an AWS CLI example of describing a listener, scroll down to <b>Example</b>.</p>
    async fn describe_listener(
        &self,
        input: DescribeListenerRequest,
    ) -> Result<DescribeListenerResponse, RusotoError<DescribeListenerError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "GlobalAccelerator_V20180706.DescribeListener",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, DescribeListenerError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<DescribeListenerResponse, _>()
    }

    /// <p>List the accelerators for an AWS account. To see an AWS CLI example of listing the accelerators for an AWS account, scroll down to <b>Example</b>.</p>
    async fn list_accelerators(
        &self,
        input: ListAcceleratorsRequest,
    ) -> Result<ListAcceleratorsResponse, RusotoError<ListAcceleratorsError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "GlobalAccelerator_V20180706.ListAccelerators",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, ListAcceleratorsError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<ListAcceleratorsResponse, _>()
    }

    /// <p>Lists the IP address ranges that were specified in calls to <a href="https://docs.aws.amazon.com/global-accelerator/latest/api/ProvisionByoipCidr.html">ProvisionByoipCidr</a>, including the current state and a history of state changes.</p> <p>To see an AWS CLI example of listing BYOIP CIDR addresses, scroll down to <b>Example</b>.</p>
    async fn list_byoip_cidrs(
        &self,
        input: ListByoipCidrsRequest,
    ) -> Result<ListByoipCidrsResponse, RusotoError<ListByoipCidrsError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "GlobalAccelerator_V20180706.ListByoipCidrs");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, ListByoipCidrsError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<ListByoipCidrsResponse, _>()
    }

    /// <p>List the endpoint groups that are associated with a listener. To see an AWS CLI example of listing the endpoint groups for listener, scroll down to <b>Example</b>.</p>
    async fn list_endpoint_groups(
        &self,
        input: ListEndpointGroupsRequest,
    ) -> Result<ListEndpointGroupsResponse, RusotoError<ListEndpointGroupsError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "GlobalAccelerator_V20180706.ListEndpointGroups",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, ListEndpointGroupsError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<ListEndpointGroupsResponse, _>()
    }

    /// <p>List the listeners for an accelerator. To see an AWS CLI example of listing the listeners for an accelerator, scroll down to <b>Example</b>.</p>
    async fn list_listeners(
        &self,
        input: ListListenersRequest,
    ) -> Result<ListListenersResponse, RusotoError<ListListenersError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "GlobalAccelerator_V20180706.ListListeners");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, ListListenersError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<ListListenersResponse, _>()
    }

    /// <p>List all tags for an accelerator. To see an AWS CLI example of listing tags for an accelerator, scroll down to <b>Example</b>.</p> <p>For more information, see <a href="https://docs.aws.amazon.com/global-accelerator/latest/dg/tagging-in-global-accelerator.html">Tagging in AWS Global Accelerator</a> in the <i>AWS Global Accelerator Developer Guide</i>. </p>
    async fn list_tags_for_resource(
        &self,
        input: ListTagsForResourceRequest,
    ) -> Result<ListTagsForResourceResponse, RusotoError<ListTagsForResourceError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "GlobalAccelerator_V20180706.ListTagsForResource",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, ListTagsForResourceError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<ListTagsForResourceResponse, _>()
    }

    /// <p>Provisions an IP address range to use with your AWS resources through bring your own IP addresses (BYOIP) and creates a corresponding address pool. After the address range is provisioned, it is ready to be advertised using <a href="https://docs.aws.amazon.com/global-accelerator/latest/api/AdvertiseByoipCidr.html"> AdvertiseByoipCidr</a>.</p> <p>To see an AWS CLI example of provisioning an address range for BYOIP, scroll down to <b>Example</b>.</p> <p>For more information, see <a href="https://docs.aws.amazon.com/global-accelerator/latest/dg/using-byoip.html">Bring Your Own IP Addresses (BYOIP)</a> in the <i>AWS Global Accelerator Developer Guide</i>.</p>
    async fn provision_byoip_cidr(
        &self,
        input: ProvisionByoipCidrRequest,
    ) -> Result<ProvisionByoipCidrResponse, RusotoError<ProvisionByoipCidrError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "GlobalAccelerator_V20180706.ProvisionByoipCidr",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, ProvisionByoipCidrError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<ProvisionByoipCidrResponse, _>()
    }

    /// <p>Add tags to an accelerator resource. To see an AWS CLI example of adding tags to an accelerator, scroll down to <b>Example</b>.</p> <p>For more information, see <a href="https://docs.aws.amazon.com/global-accelerator/latest/dg/tagging-in-global-accelerator.html">Tagging in AWS Global Accelerator</a> in the <i>AWS Global Accelerator Developer Guide</i>. </p>
    async fn tag_resource(
        &self,
        input: TagResourceRequest,
    ) -> Result<TagResourceResponse, RusotoError<TagResourceError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "GlobalAccelerator_V20180706.TagResource");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, TagResourceError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<TagResourceResponse, _>()
    }

    /// <p>Remove tags from a Global Accelerator resource. When you specify a tag key, the action removes both that key and its associated value. To see an AWS CLI example of removing tags from an accelerator, scroll down to <b>Example</b>. The operation succeeds even if you attempt to remove tags from an accelerator that was already removed.</p> <p>For more information, see <a href="https://docs.aws.amazon.com/global-accelerator/latest/dg/tagging-in-global-accelerator.html">Tagging in AWS Global Accelerator</a> in the <i>AWS Global Accelerator Developer Guide</i>.</p>
    async fn untag_resource(
        &self,
        input: UntagResourceRequest,
    ) -> Result<UntagResourceResponse, RusotoError<UntagResourceError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "GlobalAccelerator_V20180706.UntagResource");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, UntagResourceError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<UntagResourceResponse, _>()
    }

    /// <p><p>Update an accelerator. To see an AWS CLI example of updating an accelerator, scroll down to <b>Example</b>.</p> <important> <p>Global Accelerator is a global service that supports endpoints in multiple AWS Regions but you must specify the US West (Oregon) Region to create or update accelerators.</p> </important></p>
    async fn update_accelerator(
        &self,
        input: UpdateAcceleratorRequest,
    ) -> Result<UpdateAcceleratorResponse, RusotoError<UpdateAcceleratorError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "GlobalAccelerator_V20180706.UpdateAccelerator",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, UpdateAcceleratorError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<UpdateAcceleratorResponse, _>()
    }

    /// <p>Update the attributes for an accelerator. To see an AWS CLI example of updating an accelerator to enable flow logs, scroll down to <b>Example</b>.</p>
    async fn update_accelerator_attributes(
        &self,
        input: UpdateAcceleratorAttributesRequest,
    ) -> Result<UpdateAcceleratorAttributesResponse, RusotoError<UpdateAcceleratorAttributesError>>
    {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "GlobalAccelerator_V20180706.UpdateAcceleratorAttributes",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, UpdateAcceleratorAttributesError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response)
            .deserialize::<UpdateAcceleratorAttributesResponse, _>()
    }

    /// <p>Update an endpoint group. A resource must be valid and active when you add it as an endpoint.</p> <p>To see an AWS CLI example of updating an endpoint group, scroll down to <b>Example</b>. </p>
    async fn update_endpoint_group(
        &self,
        input: UpdateEndpointGroupRequest,
    ) -> Result<UpdateEndpointGroupResponse, RusotoError<UpdateEndpointGroupError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "GlobalAccelerator_V20180706.UpdateEndpointGroup",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, UpdateEndpointGroupError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<UpdateEndpointGroupResponse, _>()
    }

    /// <p>Update a listener. To see an AWS CLI example of updating listener, scroll down to <b>Example</b>.</p>
    async fn update_listener(
        &self,
        input: UpdateListenerRequest,
    ) -> Result<UpdateListenerResponse, RusotoError<UpdateListenerError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "GlobalAccelerator_V20180706.UpdateListener");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, UpdateListenerError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<UpdateListenerResponse, _>()
    }

    /// <p>Stops advertising an address range that is provisioned as an address pool. You can perform this operation at most once every 10 seconds, even if you specify different address ranges each time. To see an AWS CLI example of withdrawing an address range for BYOIP so it will no longer be advertised by AWS, scroll down to <b>Example</b>.</p> <p>It can take a few minutes before traffic to the specified addresses stops routing to AWS because of propagation delays.</p> <p>For more information, see <a href="https://docs.aws.amazon.com/global-accelerator/latest/dg/using-byoip.html">Bring Your Own IP Addresses (BYOIP)</a> in the <i>AWS Global Accelerator Developer Guide</i>.</p>
    async fn withdraw_byoip_cidr(
        &self,
        input: WithdrawByoipCidrRequest,
    ) -> Result<WithdrawByoipCidrResponse, RusotoError<WithdrawByoipCidrError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "GlobalAccelerator_V20180706.WithdrawByoipCidr",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, WithdrawByoipCidrError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<WithdrawByoipCidrResponse, _>()
    }
}
